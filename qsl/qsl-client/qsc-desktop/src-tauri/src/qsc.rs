use crate::model::{
    AppSnapshot, ContactSummary, DeviceSummary, DoctorSummary, PeerDetails, ProtocolSummary,
    ReceiveResult, ReceivedFile, SendResult, TimelineItemSummary, UiError, VaultSummary,
};
use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};
use tauri::{AppHandle, Manager};
use tempfile::{tempdir, NamedTempFile};
use zeroize::{Zeroize, Zeroizing};

const PASS_ENV_KEY: &str = "QSC_DESKTOP_SESSION_PASSPHRASE";
const QSC_BIN_ENV: &str = "QSC_DESKTOP_QSC_BIN";

#[derive(Default, Clone)]
pub struct DesktopRuntime {
    inner: Arc<RuntimeInner>,
}

#[derive(Default)]
struct RuntimeInner {
    gate: Mutex<()>,
    session: Mutex<SessionSecrets>,
}

#[derive(Default)]
struct SessionSecrets {
    passphrase: Option<Zeroizing<String>>,
}

struct ResolvedSidecar {
    path: PathBuf,
    source: String,
}

struct Capture {
    stdout: String,
    stderr: String,
    success: bool,
}

enum PassphraseUse<'a> {
    None,
    SessionGlobalUnlock,
    ExplicitGlobalUnlock(&'a str),
    ExplicitChildEnv(&'a str),
}

struct CommandSpec<'a> {
    args: Vec<String>,
    stdin: Option<Vec<u8>>,
    passphrase: PassphraseUse<'a>,
}

type FieldMap = BTreeMap<String, String>;

impl DesktopRuntime {
    pub fn refresh_snapshot(
        &self,
        app: &AppHandle,
        selected_peer: Option<String>,
    ) -> Result<AppSnapshot, UiError> {
        self.refresh_snapshot_inner(app, selected_peer, true)
    }

    pub fn init_passphrase_profile(
        &self,
        app: &AppHandle,
        passphrase: String,
        selected_peer: Option<String>,
    ) -> Result<AppSnapshot, UiError> {
        if passphrase.trim().is_empty() {
            return Err(UiError::new(
                "passphrase_required",
                "A passphrase is required to initialize a profile.",
            ));
        }
        let resolved = resolve_sidecar(app)?;
        self.run_checked(
            &resolved,
            CommandSpec {
                args: vec![
                    "vault".into(),
                    "init".into(),
                    "--non-interactive".into(),
                    "--passphrase-stdin".into(),
                ],
                stdin: Some(passphrase.clone().into_bytes()),
                passphrase: PassphraseUse::None,
            },
        )?;
        self.run_checked(
            &resolved,
            CommandSpec {
                args: vec!["identity".into(), "rotate".into(), "--confirm".into()],
                stdin: None,
                passphrase: PassphraseUse::ExplicitGlobalUnlock(passphrase.as_str()),
            },
        )?;
        self.store_passphrase(passphrase);
        self.refresh_snapshot(app, selected_peer)
    }

    pub fn unlock_profile(
        &self,
        app: &AppHandle,
        passphrase: String,
        selected_peer: Option<String>,
    ) -> Result<AppSnapshot, UiError> {
        if passphrase.trim().is_empty() {
            return Err(UiError::new(
                "passphrase_required",
                "Enter the profile passphrase before unlocking.",
            ));
        }
        let resolved = resolve_sidecar(app)?;
        self.run_checked(
            &resolved,
            CommandSpec {
                args: vec![
                    "vault".into(),
                    "unlock".into(),
                    "--non-interactive".into(),
                    "--passphrase-env".into(),
                    PASS_ENV_KEY.into(),
                ],
                stdin: None,
                passphrase: PassphraseUse::ExplicitChildEnv(passphrase.as_str()),
            },
        )?;
        self.store_passphrase(passphrase);
        self.refresh_snapshot(app, selected_peer)
    }

    pub fn clear_session_unlock(
        &self,
        app: &AppHandle,
        selected_peer: Option<String>,
    ) -> Result<AppSnapshot, UiError> {
        self.clear_passphrase();
        self.refresh_snapshot(app, selected_peer)
    }

    pub fn set_inbox_token(
        &self,
        app: &AppHandle,
        token: String,
        selected_peer: Option<String>,
    ) -> Result<AppSnapshot, UiError> {
        if token.trim().is_empty() {
            return Err(UiError::new(
                "route_token_required",
                "Enter the self inbox route token before saving it.",
            ));
        }
        let resolved = resolve_sidecar(app)?;
        self.run_checked(
            &resolved,
            CommandSpec {
                args: vec!["relay".into(), "inbox-set".into(), "--token".into(), token],
                stdin: None,
                passphrase: PassphraseUse::SessionGlobalUnlock,
            },
        )?;
        self.refresh_snapshot(app, selected_peer)
    }

    pub fn add_contact(
        &self,
        app: &AppHandle,
        label: String,
        fingerprint: String,
        route_token: Option<String>,
        selected_peer: Option<String>,
    ) -> Result<AppSnapshot, UiError> {
        if label.trim().is_empty() {
            return Err(UiError::new(
                "contact_label_required",
                "Enter a contact label before adding or refreshing a contact.",
            ));
        }
        if fingerprint.trim().is_empty() {
            return Err(UiError::new(
                "contact_fingerprint_required",
                "Enter a fingerprint or verification code before adding a contact.",
            ));
        }
        let resolved = resolve_sidecar(app)?;
        let mut args = vec![
            "contacts".into(),
            "add".into(),
            "--label".into(),
            label.clone(),
            "--fp".into(),
            fingerprint,
        ];
        if let Some(route_token) = route_token {
            if !route_token.trim().is_empty() {
                args.push("--route-token".into());
                args.push(route_token);
            }
        }
        self.run_checked(
            &resolved,
            CommandSpec {
                args,
                stdin: None,
                passphrase: PassphraseUse::SessionGlobalUnlock,
            },
        )?;
        self.refresh_snapshot(app, selected_peer.or(Some(label)))
    }

    pub fn trust_device(
        &self,
        app: &AppHandle,
        label: String,
        device_id: String,
        selected_peer: Option<String>,
    ) -> Result<AppSnapshot, UiError> {
        if label.trim().is_empty() || device_id.trim().is_empty() {
            return Err(UiError::new(
                "device_selection_required",
                "Choose a contact and a device before trusting it.",
            ));
        }
        let resolved = resolve_sidecar(app)?;
        self.run_checked(
            &resolved,
            CommandSpec {
                args: vec![
                    "contacts".into(),
                    "device".into(),
                    "trust".into(),
                    "--label".into(),
                    label.clone(),
                    "--device".into(),
                    device_id,
                    "--confirm".into(),
                ],
                stdin: None,
                passphrase: PassphraseUse::SessionGlobalUnlock,
            },
        )?;
        self.refresh_snapshot(app, selected_peer.or(Some(label)))
    }

    pub fn send_message(
        &self,
        app: &AppHandle,
        relay_url: String,
        label: String,
        message: String,
        selected_peer: Option<String>,
    ) -> Result<SendResult, UiError> {
        if relay_url.trim().is_empty() {
            return Err(UiError::new(
                "relay_url_required",
                "Enter a relay URL before sending a message.",
            ));
        }
        if label.trim().is_empty() {
            return Err(UiError::new(
                "contact_label_required",
                "Choose a peer label before sending a message.",
            ));
        }
        if message.trim().is_empty() {
            return Err(UiError::new(
                "message_required",
                "Compose a message before sending.",
            ));
        }
        let resolved = resolve_sidecar(app)?;
        let mut file = NamedTempFile::new().map_err(|_| {
            UiError::new(
                "message_tempfile_failed",
                "The desktop bridge could not create a temporary compose file.",
            )
        })?;
        file.write_all(message.as_bytes()).map_err(|_| {
            UiError::new(
                "message_tempfile_failed",
                "The desktop bridge could not write the compose file.",
            )
        })?;
        file.flush().map_err(|_| {
            UiError::new(
                "message_tempfile_failed",
                "The desktop bridge could not flush the compose file.",
            )
        })?;
        let capture = self.run_checked(
            &resolved,
            CommandSpec {
                args: vec![
                    "send".into(),
                    "--transport".into(),
                    "relay".into(),
                    "--relay".into(),
                    relay_url,
                    "--to".into(),
                    label.clone(),
                    "--file".into(),
                    file.path().display().to_string(),
                    "--receipt".into(),
                    "delivered".into(),
                ],
                stdin: None,
                passphrase: PassphraseUse::SessionGlobalUnlock,
            },
        )?;
        let delivery = parse_delivery_lines(capture.stdout.as_str());
        let snapshot = self.refresh_snapshot(app, selected_peer.or(Some(label)))?;
        Ok(SendResult { snapshot, delivery })
    }

    pub fn receive_messages(
        &self,
        app: &AppHandle,
        relay_url: String,
        label: String,
        max_items: usize,
        selected_peer: Option<String>,
    ) -> Result<ReceiveResult, UiError> {
        if relay_url.trim().is_empty() {
            return Err(UiError::new(
                "relay_url_required",
                "Enter a relay URL before polling for messages.",
            ));
        }
        if label.trim().is_empty() {
            return Err(UiError::new(
                "contact_label_required",
                "Choose a peer label before polling for messages.",
            ));
        }
        if max_items == 0 || max_items > 16 {
            return Err(UiError::new(
                "receive_max_invalid",
                "Receive max must stay within the bounded 1..16 range.",
            ));
        }
        let resolved = resolve_sidecar(app)?;
        let out_dir = tempdir().map_err(|_| {
            UiError::new(
                "receive_outdir_failed",
                "The desktop bridge could not create a temporary receive directory.",
            )
        })?;
        self.run_checked(
            &resolved,
            CommandSpec {
                args: vec![
                    "receive".into(),
                    "--transport".into(),
                    "relay".into(),
                    "--relay".into(),
                    relay_url,
                    "--from".into(),
                    label.clone(),
                    "--max".into(),
                    max_items.to_string(),
                    "--out".into(),
                    out_dir.path().display().to_string(),
                    "--emit-receipts".into(),
                    "delivered".into(),
                    "--receipt-mode".into(),
                    "immediate".into(),
                ],
                stdin: None,
                passphrase: PassphraseUse::SessionGlobalUnlock,
            },
        )?;
        let received_files = collect_received_files(out_dir.path())?;
        let snapshot = self.refresh_snapshot(app, selected_peer.or(Some(label)))?;
        Ok(ReceiveResult {
            snapshot,
            received_files,
        })
    }

    fn refresh_snapshot_inner(
        &self,
        app: &AppHandle,
        selected_peer: Option<String>,
        retry_on_lock_loss: bool,
    ) -> Result<AppSnapshot, UiError> {
        let selected_peer = selected_peer.and_then(|peer| {
            let trimmed = peer.trim();
            if trimmed.is_empty() {
                None
            } else {
                Some(trimmed.to_string())
            }
        });
        let resolved = resolve_sidecar(app)?;
        self.run_checked(
            &resolved,
            CommandSpec {
                args: vec!["status".into()],
                stdin: None,
                passphrase: PassphraseUse::None,
            },
        )?;
        let doctor = self.run_checked(
            &resolved,
            CommandSpec {
                args: vec!["doctor".into(), "--check-only".into()],
                stdin: None,
                passphrase: PassphraseUse::None,
            },
        )?;
        let doctor = parse_doctor_summary(doctor.stdout.as_str())?;
        let vault = self.capture(
            &resolved,
            CommandSpec {
                args: vec!["vault".into(), "status".into()],
                stdin: None,
                passphrase: PassphraseUse::None,
            },
        )?;
        let vault = parse_vault_summary(vault.stdout.as_str(), vault.success)?;
        let session_unlocked = self.has_passphrase();
        let mut identity_fp = None;
        let mut contacts = Vec::new();
        let mut peer_details = None;
        let mut protocol = default_protocol_summary(
            selected_peer.as_deref(),
            vault.present,
            vault.key_source.as_str(),
            session_unlocked,
        );

        if session_unlocked && vault.present {
            let identity = self.run_checked(
                &resolved,
                CommandSpec {
                    args: vec!["identity".into(), "show".into()],
                    stdin: None,
                    passphrase: PassphraseUse::SessionGlobalUnlock,
                },
            );
            match identity {
                Ok(identity) => {
                    identity_fp = parse_identity_fp(identity.stdout.as_str());
                    let contacts_capture = self.run_checked(
                        &resolved,
                        CommandSpec {
                            args: vec!["contacts".into(), "list".into()],
                            stdin: None,
                            passphrase: PassphraseUse::SessionGlobalUnlock,
                        },
                    )?;
                    contacts = parse_contacts_list(contacts_capture.stdout.as_str())?;
                    if let Some(peer) = selected_peer.filter(|peer| !peer.trim().is_empty()) {
                        let handshake_capture = self.run_checked(
                            &resolved,
                            CommandSpec {
                                args: vec![
                                    "handshake".into(),
                                    "status".into(),
                                    "--peer".into(),
                                    peer.clone(),
                                ],
                                stdin: None,
                                passphrase: PassphraseUse::SessionGlobalUnlock,
                            },
                        )?;
                        protocol = parse_protocol_summary(
                            peer.clone(),
                            handshake_capture.stdout.as_str(),
                        )?;
                        let devices_capture = self.run_checked(
                            &resolved,
                            CommandSpec {
                                args: vec![
                                    "contacts".into(),
                                    "device".into(),
                                    "list".into(),
                                    "--label".into(),
                                    peer.clone(),
                                ],
                                stdin: None,
                                passphrase: PassphraseUse::SessionGlobalUnlock,
                            },
                        )?;
                        let timeline_capture = self.run_checked(
                            &resolved,
                            CommandSpec {
                                args: vec![
                                    "timeline".into(),
                                    "list".into(),
                                    "--peer".into(),
                                    peer.clone(),
                                    "--limit".into(),
                                    "8".into(),
                                ],
                                stdin: None,
                                passphrase: PassphraseUse::SessionGlobalUnlock,
                            },
                        )?;
                        peer_details = Some(PeerDetails {
                            label: peer,
                            devices: parse_device_list(devices_capture.stdout.as_str())?,
                            timeline: parse_timeline_items(timeline_capture.stdout.as_str()),
                        });
                    }
                }
                Err(err) if retry_on_lock_loss && err.code == "vault_locked" => {
                    self.clear_passphrase();
                    return self.refresh_snapshot_inner(app, selected_peer, false);
                }
                Err(err) => return Err(err),
            }
        }

        let session_note = session_note(vault.present, vault.key_source.as_str(), session_unlocked);

        Ok(AppSnapshot {
            sidecar_ready: true,
            sidecar_source: resolved.source,
            session_unlocked,
            session_note,
            protocol,
            doctor,
            vault,
            identity_fp,
            contacts,
            peer_details,
        })
    }

    fn has_passphrase(&self) -> bool {
        self.inner
            .session
            .lock()
            .map(|session| session.passphrase.is_some())
            .unwrap_or(false)
    }

    fn store_passphrase(&self, passphrase: String) {
        if let Ok(mut session) = self.inner.session.lock() {
            session.passphrase = Some(Zeroizing::new(passphrase));
        }
    }

    fn clear_passphrase(&self) {
        if let Ok(mut session) = self.inner.session.lock() {
            session.passphrase = None;
        }
    }

    fn run_checked(
        &self,
        resolved: &ResolvedSidecar,
        spec: CommandSpec<'_>,
    ) -> Result<Capture, UiError> {
        let capture = self.capture(resolved, spec)?;
        if let Some(fields) = first_error_fields(capture.stdout.as_str()) {
            return Err(ui_error_from_fields(&fields));
        }
        if !capture.success {
            return Err(UiError::with_detail(
                "sidecar_failed",
                "The qsc sidecar exited without a stable error marker.",
                capture.stderr,
            ));
        }
        Ok(capture)
    }

    fn capture(
        &self,
        resolved: &ResolvedSidecar,
        spec: CommandSpec<'_>,
    ) -> Result<Capture, UiError> {
        let _gate = self.inner.gate.lock().map_err(|_| {
            UiError::new(
                "desktop_gate_poisoned",
                "The desktop command gate is unavailable.",
            )
        })?;

        let mut command = Command::new(&resolved.path);
        command
            .env("QSC_NONINTERACTIVE", "1")
            .env("QSC_MARK_FORMAT", "plain")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        let mut borrowed_passphrase = None;
        let mut owned_passphrase = None;
        match spec.passphrase {
            PassphraseUse::None => {}
            PassphraseUse::SessionGlobalUnlock => {
                let session = self.inner.session.lock().map_err(|_| {
                    UiError::new(
                        "desktop_session_poisoned",
                        "The desktop session lock is unavailable.",
                    )
                })?;
                let Some(passphrase) = session.passphrase.as_ref() else {
                    return Err(ui_error_from_code("vault_locked", None));
                };
                borrowed_passphrase = Some(Zeroizing::new(passphrase.to_string()));
                command.arg("--unlock-passphrase-env").arg(PASS_ENV_KEY);
            }
            PassphraseUse::ExplicitGlobalUnlock(passphrase) => {
                owned_passphrase = Some(Zeroizing::new(passphrase.to_string()));
                command.arg("--unlock-passphrase-env").arg(PASS_ENV_KEY);
            }
            PassphraseUse::ExplicitChildEnv(passphrase) => {
                owned_passphrase = Some(Zeroizing::new(passphrase.to_string()));
            }
        }

        if let Some(passphrase) = borrowed_passphrase.as_ref().or(owned_passphrase.as_ref()) {
            command.env(PASS_ENV_KEY, passphrase.as_str());
        }

        command.args(spec.args);

        if spec.stdin.is_some() {
            command.stdin(Stdio::piped());
        } else {
            command.stdin(Stdio::null());
        }

        let mut child = command.spawn().map_err(|_| {
            UiError::with_detail(
                "sidecar_spawn_failed",
                "The desktop bridge could not start the qsc sidecar.",
                resolved.path.display().to_string(),
            )
        })?;

        if let Some(stdin) = spec.stdin {
            let mut handle = child.stdin.take().ok_or_else(|| {
                UiError::new(
                    "sidecar_stdin_unavailable",
                    "The qsc sidecar did not expose stdin for the requested operation.",
                )
            })?;
            handle.write_all(stdin.as_slice()).map_err(|_| {
                UiError::new(
                    "sidecar_stdin_write_failed",
                    "The desktop bridge could not write to the qsc sidecar stdin.",
                )
            })?;
        }

        let output = child.wait_with_output().map_err(|_| {
            UiError::new(
                "sidecar_wait_failed",
                "The desktop bridge could not collect qsc sidecar output.",
            )
        })?;

        if let Some(mut passphrase) = borrowed_passphrase {
            passphrase.zeroize();
        }
        if let Some(mut passphrase) = owned_passphrase {
            passphrase.zeroize();
        }

        Ok(Capture {
            stdout: String::from_utf8_lossy(output.stdout.as_slice()).to_string(),
            stderr: String::from_utf8_lossy(output.stderr.as_slice()).to_string(),
            success: output.status.success(),
        })
    }
}

fn resolve_sidecar(app: &AppHandle) -> Result<ResolvedSidecar, UiError> {
    if let Ok(path) = env::var(QSC_BIN_ENV) {
        let path = PathBuf::from(path);
        if path.exists() {
            return Ok(ResolvedSidecar {
                path,
                source: "env override".to_string(),
            });
        }
    }

    let resource_dir = app.path().resource_dir().map_err(|_| {
        UiError::new(
            "sidecar_resource_dir_unavailable",
            "The desktop bridge could not resolve the bundled resource directory.",
        )
    })?;
    for relative in ["bin/qsc", "resources/bin/qsc", "qsc"] {
        let candidate = resource_dir.join(relative);
        if candidate.exists() {
            return Ok(ResolvedSidecar {
                path: candidate,
                source: "bundled resource".to_string(),
            });
        }
    }

    Err(UiError::with_detail(
        "sidecar_missing",
        "The bundled qsc sidecar is missing. Run the sidecar preparation step or set QSC_DESKTOP_QSC_BIN.",
        resource_dir.display().to_string(),
    ))
}

fn parse_doctor_summary(stdout: &str) -> Result<DoctorSummary, UiError> {
    let Some(fields) = marker_fields(stdout, "doctor") else {
        return Err(UiError::new(
            "doctor_parse_failed",
            "The desktop bridge could not parse qsc doctor output.",
        ));
    };
    Ok(DoctorSummary {
        ok: bool_field(&fields, "ok"),
        config_dir: fields.get("checked_dir").cloned().unwrap_or_default(),
        dir_exists: bool_field(&fields, "dir_exists"),
        dir_writable: bool_field(&fields, "dir_writable"),
        file_parseable: bool_field(&fields, "file_parseable"),
        symlink_safe: bool_field(&fields, "symlink_safe"),
        parent_safe: bool_field(&fields, "parent_safe"),
    })
}

fn parse_vault_summary(stdout: &str, success: bool) -> Result<VaultSummary, UiError> {
    if let Some(code) = first_error_code(stdout) {
        if code == "vault_missing" {
            return Ok(VaultSummary::missing());
        }
        return Err(ui_error_from_code(code.as_str(), None));
    }
    if !success {
        return Err(UiError::new(
            "vault_status_failed",
            "The desktop bridge could not collect vault status.",
        ));
    }
    let Some(fields) = marker_fields(stdout, "vault_status") else {
        return Err(UiError::new(
            "vault_status_parse_failed",
            "The desktop bridge could not parse qsc vault status output.",
        ));
    };
    Ok(VaultSummary {
        present: bool_field(&fields, "present"),
        key_source: fields
            .get("key_source")
            .cloned()
            .unwrap_or_else(|| "unknown".to_string()),
    })
}

fn parse_identity_fp(stdout: &str) -> Option<String> {
    stdout
        .lines()
        .find_map(|line| line.strip_prefix("identity_fp=").map(ToOwned::to_owned))
}

fn parse_contacts_list(stdout: &str) -> Result<Vec<ContactSummary>, UiError> {
    let mut contacts = Vec::new();
    for line in stdout.lines() {
        if !line.starts_with("label=") {
            continue;
        }
        let fields = token_map(line);
        let label = fields.get("label").cloned().unwrap_or_default();
        if label.is_empty() {
            continue;
        }
        contacts.push(ContactSummary {
            label,
            state: fields.get("state").cloned().unwrap_or_default(),
            blocked: fields.get("blocked").map(|v| v == "true").unwrap_or(false),
            device_count: fields
                .get("device_count")
                .and_then(|value| value.parse::<usize>().ok())
                .unwrap_or(0),
            primary_device: fields.get("primary_device").cloned(),
        });
    }
    if marker_fields(stdout, "contacts_list").is_none() {
        return Err(UiError::new(
            "contacts_parse_failed",
            "The desktop bridge could not parse qsc contacts output.",
        ));
    }
    Ok(contacts)
}

fn parse_device_list(stdout: &str) -> Result<Vec<DeviceSummary>, UiError> {
    let mut devices = Vec::new();
    for line in stdout.lines() {
        if !line.starts_with("device=") {
            continue;
        }
        let fields = token_map(line);
        let device = fields.get("device").cloned().unwrap_or_default();
        if device.is_empty() {
            continue;
        }
        devices.push(DeviceSummary {
            device,
            state: fields.get("state").cloned().unwrap_or_default(),
        });
    }
    if marker_fields(stdout, "contacts_device_list").is_none() {
        return Err(UiError::new(
            "devices_parse_failed",
            "The desktop bridge could not parse qsc device-list output.",
        ));
    }
    Ok(devices)
}

fn parse_timeline_items(stdout: &str) -> Vec<TimelineItemSummary> {
    let mut items = Vec::new();
    for line in stdout.lines() {
        if !line.contains("event=timeline_item") {
            continue;
        }
        let fields = token_map(line);
        let id = fields.get("id").cloned().unwrap_or_default();
        if id.is_empty() {
            continue;
        }
        items.push(TimelineItemSummary {
            id,
            direction: fields.get("dir").cloned().unwrap_or_default(),
            kind: fields.get("kind").cloned().unwrap_or_default(),
            ts: fields
                .get("ts")
                .and_then(|value| value.parse::<u64>().ok())
                .unwrap_or(0),
            state: fields.get("state").cloned().unwrap_or_default(),
        });
    }
    items
}

fn parse_delivery_lines(stdout: &str) -> Vec<String> {
    stdout
        .lines()
        .filter(|line| line.starts_with("QSC_DELIVERY "))
        .map(ToOwned::to_owned)
        .collect()
}

fn collect_received_files(out_dir: &Path) -> Result<Vec<ReceivedFile>, UiError> {
    let mut files = Vec::new();
    let entries = fs::read_dir(out_dir).map_err(|_| {
        UiError::new(
            "receive_read_failed",
            "The desktop bridge could not inspect the receive output directory.",
        )
    })?;
    for entry in entries {
        let entry = entry.map_err(|_| {
            UiError::new(
                "receive_read_failed",
                "The desktop bridge could not enumerate received files.",
            )
        })?;
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        let bytes = fs::read(path.as_path()).map_err(|_| {
            UiError::new(
                "receive_read_failed",
                "The desktop bridge could not read a received file.",
            )
        })?;
        let mut preview = String::from_utf8_lossy(bytes.as_slice()).to_string();
        if preview.chars().count() > 240 {
            preview = preview.chars().take(240).collect::<String>();
            preview.push('…');
        }
        files.push(ReceivedFile {
            file_name: path
                .file_name()
                .map(|name| name.to_string_lossy().to_string())
                .unwrap_or_else(|| "received.bin".to_string()),
            kind: "message_payload".to_string(),
            byte_len: bytes.len(),
            preview,
        });
    }
    files.sort_by(|left, right| left.file_name.cmp(&right.file_name));
    Ok(files)
}

fn first_error_code(stdout: &str) -> Option<String> {
    first_error_fields(stdout).and_then(|fields| fields.get("code").cloned())
}

fn first_error_fields(stdout: &str) -> Option<FieldMap> {
    stdout.lines().find_map(|line| {
        if !line.starts_with("QSC_MARK/1 ") {
            return None;
        }
        let fields = token_map(line);
        if fields.get("event").map(String::as_str) == Some("error") {
            return Some(fields);
        }
        None
    })
}

fn marker_fields(stdout: &str, event: &str) -> Option<FieldMap> {
    stdout.lines().find_map(|line| {
        if !line.starts_with("QSC_MARK/1 ") {
            return None;
        }
        let fields = token_map(line);
        if fields.get("event").map(String::as_str) == Some(event) {
            return Some(fields);
        }
        None
    })
}

fn token_map(line: &str) -> FieldMap {
    line.split_whitespace()
        .filter_map(|part| {
            part.split_once('=')
                .map(|(key, value)| (key.to_string(), value.to_string()))
        })
        .collect()
}

fn bool_field(fields: &FieldMap, key: &str) -> bool {
    fields
        .get(key)
        .map(|value| value == "true")
        .unwrap_or(false)
}

fn default_protocol_summary(
    selected_peer: Option<&str>,
    vault_present: bool,
    key_source: &str,
    session_unlocked: bool,
) -> ProtocolSummary {
    let peer = selected_peer.map(ToOwned::to_owned);
    if !vault_present {
        return ProtocolSummary {
            peer,
            status: "profile_missing".to_string(),
            send_ready: false,
            receive_ready: false,
            note: "Initialize a local qsc profile first.".to_string(),
        };
    }
    if key_source == "keychain" {
        return ProtocolSummary {
            peer,
            status: "keychain_deferred".to_string(),
            send_ready: false,
            receive_ready: false,
            note: "Keychain-backed active operations remain deferred in this prototype."
                .to_string(),
        };
    }
    if !session_unlocked {
        return ProtocolSummary {
            peer,
            status: "vault_locked".to_string(),
            send_ready: false,
            receive_ready: false,
            note: "Unlock the passphrase-backed profile before checking protocol readiness."
                .to_string(),
        };
    }
    if peer.is_none() {
        return ProtocolSummary {
            peer,
            status: "peer_unselected".to_string(),
            send_ready: false,
            receive_ready: false,
            note: "Select a contact to inspect protocol readiness.".to_string(),
        };
    }
    ProtocolSummary {
        peer,
        status: "unknown".to_string(),
        send_ready: false,
        receive_ready: false,
        note: "Protocol readiness could not be confirmed for the selected peer.".to_string(),
    }
}

fn parse_protocol_summary(peer: String, stdout: &str) -> Result<ProtocolSummary, UiError> {
    let Some(fields) = marker_fields(stdout, "handshake_status") else {
        return Err(UiError::new(
            "protocol_status_parse_failed",
            "The desktop bridge could not parse qsc handshake status output.",
        ));
    };
    if fields.contains_key("code") {
        return Err(UiError::new(
            "protocol_status_failed",
            "The desktop bridge could not confirm protocol readiness for the selected peer.",
        ));
    }
    let status = fields
        .get("status")
        .cloned()
        .unwrap_or_else(|| "unknown".to_string());
    let send_ready = fields
        .get("send_ready")
        .map(|value| value == "yes")
        .unwrap_or(false);
    let receive_ready = matches!(status.as_str(), "established" | "established_recv_only");
    let send_ready_reason = fields.get("send_ready_reason").map(String::as_str);
    Ok(ProtocolSummary {
        peer: Some(peer),
        status: status.clone(),
        send_ready,
        receive_ready,
        note: protocol_note(status.as_str(), send_ready_reason),
    })
}

fn protocol_note(status: &str, send_ready_reason: Option<&str>) -> String {
    match status {
        "established" => "Protocol ready for send and receive for this peer.".to_string(),
        "established_recv_only" => {
            "Receive is ready for this peer, but send stays blocked until qsc completes activation outside this prototype."
                .to_string()
        }
        "no_session" => {
            "Protocol inactive for this peer. Run qsc handshake init/poll outside this prototype before sending or receiving."
                .to_string()
        }
        _ => match send_ready_reason {
            Some("chainkey_unset") => {
                "Receive is ready for this peer, but send keys are not ready yet.".to_string()
            }
            Some("vault_secret_missing") => {
                "Unlock the local profile before qsc can restore protocol state.".to_string()
            }
            Some("state_corrupt") => {
                "Stored protocol state is invalid or stale. Re-establish it outside this prototype."
                    .to_string()
            }
            _ => "Protocol readiness could not be confirmed for the selected peer.".to_string(),
        },
    }
}

fn session_note(vault_present: bool, key_source: &str, session_unlocked: bool) -> Option<String> {
    if !vault_present {
        return Some("Initialize a local qsc profile first.".to_string());
    }
    if key_source == "keychain" {
        return Some(
            "Keychain-backed active operations remain deferred in this prototype.".to_string(),
        );
    }
    if !session_unlocked && key_source == "passphrase" {
        return Some("Passphrase unlock required for contacts and message actions.".to_string());
    }
    None
}

fn protocol_inactive_detail(reason: Option<&str>) -> String {
    match reason {
        Some("no_session" | "missing_seed") => {
            "Use qsc handshake init/poll for this peer outside the GUI before retrying."
                .to_string()
        }
        Some("chainkey_unset") => {
            "Receive state exists, but send remains blocked until qsc completes activation for this peer outside the GUI."
                .to_string()
        }
        Some("session_invalid") => {
            "Stored protocol state is invalid or stale. Re-establish it outside the GUI."
                .to_string()
        }
        Some("vault_secret_missing") => {
            "Unlock the local profile before qsc can restore protocol state for this peer."
                .to_string()
        }
        Some(other) => format!("Stable qsc reason: {other}."),
        None => String::new(),
    }
}

fn ui_error_from_fields(fields: &FieldMap) -> UiError {
    let code = fields
        .get("code")
        .map(String::as_str)
        .unwrap_or("sidecar_failed");
    let reason = fields.get("reason").map(String::as_str);
    ui_error_from_code(code, reason)
}

fn ui_error_from_code(code: &str, reason: Option<&str>) -> UiError {
    match code {
        "vault_locked" => UiError::new(
            "vault_locked",
            "The profile is locked. Unlock it with the passphrase-backed path before continuing.",
        ),
        "vault_missing" => UiError::new(
            "vault_missing",
            "No local qsc profile was found. Initialize a profile first.",
        ),
        "protocol_inactive" => UiError::with_detail(
            "protocol_inactive",
            "The sidecar reported that this peer is not protocol-ready for the requested action.",
            protocol_inactive_detail(reason),
        ),
        "sidecar_missing" => UiError::new("sidecar_missing", "The bundled qsc sidecar is missing."),
        other => UiError::new(
            other.to_string(),
            format!("The qsc sidecar returned the stable error code `{other}`."),
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn doctor_summary_parses_checked_dir_and_bools() {
        let stdout = "QSC_MARK/1 event=doctor check_only=true ok=true checked_dir=/tmp/qsc dir_writable_required=false dir_exists=true dir_writable=true file_parseable=true symlink_safe=true parent_safe=true receipt_mode=off file_confirm_mode=complete_only receipt_batch_window_ms=250 receipt_jitter_ms=0\n";
        let doctor = parse_doctor_summary(stdout).expect("doctor parse");
        assert!(doctor.ok);
        assert_eq!(doctor.config_dir, "/tmp/qsc");
        assert!(doctor.dir_exists);
        assert!(doctor.dir_writable);
        assert!(doctor.file_parseable);
        assert!(doctor.symlink_safe);
        assert!(doctor.parent_safe);
    }

    #[test]
    fn contacts_list_parses_frozen_fields_only() {
        let stdout = "QSC_MARK/1 event=contacts_list count=1\nlabel=bob state=TRUSTED blocked=false device_count=2 primary_device=abc123\n";
        let contacts = parse_contacts_list(stdout).expect("contacts parse");
        assert_eq!(contacts.len(), 1);
        assert_eq!(contacts[0].label, "bob");
        assert_eq!(contacts[0].state, "TRUSTED");
        assert_eq!(contacts[0].device_count, 2);
        assert_eq!(contacts[0].primary_device.as_deref(), Some("abc123"));
    }

    #[test]
    fn timeline_items_parse_marker_rows() {
        let stdout = "QSC_MARK/1 event=timeline_list count=1 peer=bob\nQSC_MARK/1 event=timeline_item id=msg-1 dir=out kind=msg ts=42 state=peer_confirmed\n";
        let items = parse_timeline_items(stdout);
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].id, "msg-1");
        assert_eq!(items[0].direction, "out");
        assert_eq!(items[0].kind, "msg");
        assert_eq!(items[0].ts, 42);
        assert_eq!(items[0].state, "peer_confirmed");
    }

    #[test]
    fn delivery_lines_only_keep_message_delivery_markers() {
        let stdout = "QSC_DELIVERY state=accepted_by_relay id=msg-1\nQSC_MARK/1 event=status ok=true locked=false\nQSC_DELIVERY state=peer_confirmed id=msg-1\n";
        let lines = parse_delivery_lines(stdout);
        assert_eq!(lines.len(), 2);
        assert!(lines[0].contains("accepted_by_relay"));
        assert!(lines[1].contains("peer_confirmed"));
    }

    #[test]
    fn received_files_collect_preview_without_persisting_extra_state() {
        let dir = tempdir().expect("tempdir");
        fs::write(dir.path().join("recv_1.bin"), "hello from qsc").expect("write preview");
        let files = collect_received_files(dir.path()).expect("collect files");
        assert_eq!(files.len(), 1);
        assert_eq!(files[0].file_name, "recv_1.bin");
        assert_eq!(files[0].byte_len, 14);
        assert_eq!(files[0].preview, "hello from qsc");
    }

    #[test]
    fn protocol_summary_parses_no_session_fail_closed_note() {
        let stdout = "QSC_MARK/1 event=handshake_status status=no_session peer=bob peer_fp=abc pinned=false send_ready=no send_ready_reason=no_session\n";
        let summary = parse_protocol_summary("bob".to_string(), stdout).expect("protocol parse");
        assert_eq!(summary.peer.as_deref(), Some("bob"));
        assert_eq!(summary.status, "no_session");
        assert!(!summary.send_ready);
        assert!(!summary.receive_ready);
        assert!(summary.note.contains("handshake init/poll"));
    }

    #[test]
    fn protocol_summary_parses_receive_only_state() {
        let stdout = "QSC_MARK/1 event=handshake_status status=established_recv_only peer=alice peer_fp=def pinned=true send_ready=no send_ready_reason=chainkey_unset\n";
        let summary =
            parse_protocol_summary("alice".to_string(), stdout).expect("protocol parse recv only");
        assert_eq!(summary.status, "established_recv_only");
        assert!(!summary.send_ready);
        assert!(summary.receive_ready);
        assert!(summary.note.contains("send stays blocked"));
    }

    #[test]
    fn protocol_summary_parses_established_state() {
        let stdout =
            "QSC_MARK/1 event=handshake_status status=established peer=alice peer_fp=def pinned=true send_ready=yes\n";
        let summary = parse_protocol_summary("alice".to_string(), stdout)
            .expect("protocol parse established");
        assert_eq!(summary.status, "established");
        assert!(summary.send_ready);
        assert!(summary.receive_ready);
    }

    #[test]
    fn protocol_inactive_error_surfaces_stable_reason_detail() {
        let mut fields = FieldMap::new();
        fields.insert("code".to_string(), "protocol_inactive".to_string());
        fields.insert("reason".to_string(), "no_session".to_string());
        let err = ui_error_from_fields(&fields);
        assert_eq!(err.code, "protocol_inactive");
        assert!(err.detail.contains("handshake init/poll"));
    }
}
