use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;
use chacha20poly1305::aead::{Aead, KeyInit, Payload};
use chacha20poly1305::{ChaCha20Poly1305, Key, Nonce};
use clap::Parser;
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use quantumshield_refimpl::crypto::stdcrypto::{
    runtime_pq_kem_ciphertext_bytes, runtime_pq_kem_keypair, runtime_pq_kem_public_key_bytes,
    runtime_pq_sig_keypair, runtime_pq_sig_public_key_bytes, runtime_pq_sig_signature_bytes,
    StdCrypto,
};
use quantumshield_refimpl::crypto::traits::{
    Hash, Kmac, PqKem768, PqSigMldsa65, X25519Dh, X25519Priv, X25519Pub,
};
use quantumshield_refimpl::qse::{Envelope, EnvelopeProfile};
use quantumshield_refimpl::suite2::establish::init_from_base_handshake;
use quantumshield_refimpl::suite2::ratchet::Suite2RecvWireState;
use quantumshield_refimpl::suite2::state::Suite2SessionState;
use quantumshield_refimpl::suite2::types::{SUITE2_PROTOCOL_VERSION, SUITE2_SUITE_ID};
use quantumshield_refimpl::suite2::{recv_wire_canon, send_wire_canon};
use quantumshield_refimpl::RefimplError;
use rand_core::{OsRng, RngCore};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear as TuiClear, List, ListItem, Paragraph},
    Terminal,
};
use reqwest::blocking::Client as HttpClient;
use reqwest::StatusCode as HttpStatus;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha512};
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::env;
use std::fs::{self, File, OpenOptions};
use std::io::{IsTerminal, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::process::{self, Command, Stdio};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::mpsc;
use std::time::{Duration, Instant};
use zeroize::Zeroize;

const CONFIG_FILE_NAME: &str = "config.txt";
const STORE_META_NAME: &str = "store.meta";
const LOCK_FILE_NAME: &str = ".qsc.lock";
const OUTBOX_FILE_NAME: &str = "outbox.json";
const SEND_STATE_NAME: &str = "send.state";
const QSE_ENV_VERSION_V1: u16 = 0x0100;
const POLICY_KEY: &str = "policy_profile";
const STORE_META_TEMPLATE: &str = "store_version=1\nvmk_status=unset\nkeyslots=0\n";
const MAX_QUEUE_LEN: usize = 64;
const MAX_HISTORY_LEN: usize = 128;
const MAX_RETRY_ATTEMPTS: u32 = 5;
const RETRY_BASE_MS: u64 = 20;
const RETRY_MAX_MS: u64 = 200;
const RETRY_JITTER_MS: u64 = 10;
const MAX_TIMEOUT_MS: u64 = 2000;
const TUI_AUTOLOCK_DEFAULT_MINUTES: u64 = 10;
const TUI_AUTOLOCK_MIN_MINUTES: u64 = 1;
const TUI_AUTOLOCK_MAX_MINUTES: u64 = 120;
const TUI_POLL_DEFAULT_INTERVAL_SECONDS: u64 = 10;
const TUI_POLL_MIN_INTERVAL_SECONDS: u64 = 2;
const TUI_POLL_MAX_INTERVAL_SECONDS: u64 = 300;
const RECEIPT_BATCH_WINDOW_MS_DEFAULT: u64 = 250;
const RECEIPT_JITTER_MS_DEFAULT: u64 = 0;
const RECEIPT_BATCH_WINDOW_MS_MAX: u64 = 60_000;
const RECEIPT_JITTER_MS_MAX: u64 = 5_000;
const ATTACHMENT_DESCRIPTOR_VERSION: u8 = 1;
const ATTACHMENT_DESCRIPTOR_TYPE: &str = "attachment_descriptor";
const ATTACHMENT_CONFIRM_KIND: &str = "attachment_confirmed";
const ATTACHMENT_LOCATOR_KIND_V1: &str = "service_ref_v1";
const ATTACHMENT_INTEGRITY_ALG_V1: &str = "sha512_merkle_v1";
const ATTACHMENT_ENC_CTX_ALG_V1: &str = "chacha20poly1305_part_v1";
const ATTACHMENT_CONTEXT_PACKAGE_LEN: usize = 41;
const ATTACHMENT_CONTEXT_PACKAGE_B64U_LEN: usize = 55;
const ATTACHMENT_CIPHER_TAG_LEN: usize = 16;
const ATTACHMENT_LEGACY_THRESHOLD_BYTES: usize = FILE_XFER_MAX_FILE_SIZE_CEILING;
const ATTACHMENT_DEFAULT_MAX_FILE_SIZE: usize = 100 * 1024 * 1024;
const ATTACHMENT_DEFAULT_MAX_PARTS: usize = 4096;
const ATTACHMENT_STAGING_DIR: &str = "attachments";
const QSC_ATTACHMENT_SERVICE_ENV: &str = "QSC_ATTACHMENT_SERVICE";
const QSC_LEGACY_IN_MESSAGE_STAGE_ENV: &str = "QSC_LEGACY_IN_MESSAGE_STAGE";

mod adversarial;
mod cmd;
mod envelope;
mod fs_store;
mod identity;
mod model;
mod output;
mod protocol_state;
mod relay;
mod store;
mod tui;
mod vault;

use cmd::*;
use fs_store::{
    check_parent_safe, check_symlink_safe, config_dir, enforce_file_perms, enforce_safe_parents,
    ensure_dir_secure, ensure_store_layout, fsync_dir_best_effort, lock_store_exclusive,
    lock_store_shared, normalize_profile, probe_dir_writable, read_policy_profile, set_umask_077,
    write_atomic, write_config_atomic,
};
use identity::{
    format_verification_code_from_fingerprint, identities_dir, identity_fingerprint_from_pk,
    identity_marker_display, identity_pin_matches_seen, identity_read_pin,
    identity_read_self_public, identity_read_sig_pin, identity_secret_name, identity_secret_store,
    identity_self_fingerprint, identity_self_kem_keypair, identity_self_path,
    identity_sig_secret_name, identity_sig_secret_store, identity_write_public_record,
    IdentityKeypair, IDENTITY_FP_PREFIX,
};
use model::*;
use output::{
    emit_cli_named_marker, emit_marker, emit_tui_named_marker, init_output_policy,
    install_panic_redaction_hook, marker_queue, print_error_marker, print_marker, qsc_mark,
    qsc_sanitize_terminal_text, redact_text_for_output, set_marker_routing, MarkerRouting,
    PANIC_DEMO_SENTINEL,
};
use protocol_state::{
    allow_seed_fallback_for_tests, emit_protocol_inactive, kmac_out,
    protocol_active_or_reason_for_peer, protocol_inactive_exit, qsp_send_ready_tuple,
    qsp_session_for_channel, qsp_session_load, qsp_session_store, qsp_status_parts,
    qsp_status_string, qsp_status_tuple, qsp_status_user_note, record_qsp_status, zero32,
    QSP_STATUS_FILE_NAME,
};
use relay::*;
use store::*;
use tui::*;

static VAULT_UNLOCKED_THIS_RUN: AtomicBool = AtomicBool::new(false);

fn set_vault_unlocked(unlocked: bool) {
    VAULT_UNLOCKED_THIS_RUN.store(unlocked, Ordering::SeqCst);
}

fn vault_unlocked() -> bool {
    VAULT_UNLOCKED_THIS_RUN.load(Ordering::SeqCst)
}

fn bootstrap_unlock(passphrase_file: Option<&Path>, passphrase_env: Option<&str>) {
    if vault::unlock_if_mock_provider() {
        set_vault_unlocked(true);
        return;
    }
    if let Some(path) = passphrase_file {
        match vault::unlock_with_passphrase_file(path) {
            Ok(()) => set_vault_unlocked(true),
            Err(code) => print_error_marker(code),
        }
    } else if let Some(env_name) = passphrase_env {
        if env_name.trim().is_empty() {
            print_error_marker("vault_locked");
        }
        match vault::unlock_with_passphrase_env(Some(env_name)) {
            Ok(()) => set_vault_unlocked(true),
            Err(code) => print_error_marker(code),
        }
    } else if allow_seed_fallback_for_tests() {
        // Deterministic test mode keeps existing seeded test workflows intact.
        set_vault_unlocked(true);
    }
}

fn require_unlocked(op_name: &'static str) -> bool {
    if vault_unlocked() {
        return true;
    }
    emit_marker(
        "error",
        Some("vault_locked"),
        &[("op", op_name), ("reason", "explicit_unlock_required")],
    );
    process::exit(1);
}

fn main() {
    set_umask_077();
    install_panic_redaction_hook();
    let cli = Cli::parse();
    init_output_policy(cli.reveal);
    set_vault_unlocked(false);
    bootstrap_unlock(
        cli.unlock_passphrase_file.as_deref(),
        cli.unlock_passphrase_env.as_deref(),
    );
    match cli.cmd {
        None => {
            // Shell-first UX expects help by default.
            println!("QSC_MARK/1 event=help_stub");
        }
        Some(Cmd::Status) => {
            let locked = if vault_unlocked() { "false" } else { "true" };
            print_marker("status", &[("ok", "true"), ("locked", locked)]);
            let status_peer = "peer-0";
            let (status, reason) = qsp_status_tuple(status_peer);
            emit_marker(
                "qsp_status",
                None,
                &[("status", status.as_str()), ("reason", reason.as_str())],
            );
            let (peer_fp, pinned) = identity_peer_status(status_peer);
            let pinned_s = if pinned { "true" } else { "false" };
            emit_marker(
                "identity_status",
                None,
                &[
                    ("peer", status_peer),
                    ("peer_fp", peer_fp.as_str()),
                    ("pinned", pinned_s),
                ],
            );
            let policy = load_receipt_policy_from_account();
            let batch_window_s = policy.batch_window_ms.to_string();
            let jitter_s = policy.jitter_ms.to_string();
            emit_marker(
                "receipt_policy",
                None,
                &[
                    ("mode", policy.mode.as_str()),
                    ("batch_window_ms", batch_window_s.as_str()),
                    ("jitter_ms", jitter_s.as_str()),
                    ("file_confirm_mode", policy.file_confirm_mode.as_str()),
                ],
            );
        }
        Some(Cmd::Config { cmd }) => match cmd {
            ConfigCmd::Set { key, value } => config_set(&key, &value),
            ConfigCmd::Get { key } => config_get(&key),
        },
        Some(Cmd::Doctor {
            check_only,
            timeout_ms,
            export,
        }) => doctor_check_only(check_only, timeout_ms, export),
        Some(Cmd::Util { cmd }) => match cmd {
            UtilCmd::Sanitize { print } => util_sanitize(print),
            UtilCmd::Queue { len } => util_queue(len),
            UtilCmd::History { len } => util_history(len),
            UtilCmd::Retry { fail } => util_retry(fail),
            UtilCmd::Timeout {
                wait_ms,
                timeout_ms,
            } => util_timeout(wait_ms, timeout_ms),
            UtilCmd::Envelope {
                tick_count,
                interval_ms,
                max_ticks,
                max_bundle,
                max_count,
                payload_lens,
            } => util_envelope(
                tick_count,
                interval_ms,
                max_ticks,
                max_bundle,
                max_count,
                payload_lens,
            ),
            UtilCmd::PanicDemo => util_panic_demo(),
            UtilCmd::ReceiptApply {
                peer,
                channel,
                msg_id,
                file_id,
                confirm_id,
            } => util_receipt_apply(peer.as_str(), channel.as_str(), msg_id, file_id, confirm_id),
        },
        Some(Cmd::Envelope { cmd }) => match cmd {
            EnvelopeCmd::PlanAck {
                deterministic,
                tick_count,
                interval_ms,
                max_ticks,
                max_bundle,
                max_count,
                small_len,
            } => envelope_plan_ack(
                deterministic,
                tick_count,
                interval_ms,
                max_ticks,
                max_bundle,
                max_count,
                small_len,
            ),
        },
        Some(Cmd::Vault { cmd }) => vault::cmd_vault(cmd),
        Some(Cmd::Send {
            cmd,
            transport,
            relay,
            to,
            file,
            pad_to,
            pad_bucket,
            bucket_max,
            meta_seed,
            receipt,
        }) => match cmd {
            Some(SendCmd::Abort) => send_abort(),
            None => send_execute(SendExecuteArgs {
                transport,
                relay,
                to,
                file,
                pad_to,
                pad_bucket,
                bucket_max,
                meta_seed,
                receipt,
            }),
        },
        Some(Cmd::Receive {
            transport,
            relay,
            legacy_receive_mode,
            attachment_service,
            from,
            mailbox,
            max,
            max_file_size,
            max_file_chunks,
            out,
            file,
            deterministic_meta,
            interval_ms,
            poll_interval_ms,
            poll_ticks,
            batch_max_count,
            poll_max_per_tick,
            bucket_max,
            meta_seed,
            emit_receipts,
            receipt_mode,
            receipt_batch_window_ms,
            receipt_jitter_ms,
            file_confirm_mode,
        }) => {
            if let Some(path) = file {
                if transport.is_some()
                    || relay.is_some()
                    || legacy_receive_mode.is_some()
                    || attachment_service.is_some()
                    || from.is_some()
                    || mailbox.is_some()
                    || max.is_some()
                    || max_file_size.is_some()
                    || max_file_chunks.is_some()
                    || out.is_some()
                    || deterministic_meta
                    || interval_ms.is_some()
                    || poll_interval_ms.is_some()
                    || poll_ticks.is_some()
                    || batch_max_count.is_some()
                    || poll_max_per_tick.is_some()
                    || bucket_max.is_some()
                    || meta_seed.is_some()
                    || emit_receipts.is_some()
                    || receipt_mode.is_some()
                    || receipt_batch_window_ms.is_some()
                    || receipt_jitter_ms.is_some()
                    || file_confirm_mode.is_some()
                {
                    print_error_marker("recv_file_conflict");
                }
                receive_file(&path);
            } else {
                let args = ReceiveArgs {
                    transport,
                    relay,
                    legacy_receive_mode,
                    attachment_service,
                    from,
                    mailbox,
                    max,
                    max_file_size,
                    max_file_chunks,
                    out,
                    deterministic_meta,
                    interval_ms,
                    poll_interval_ms,
                    poll_ticks,
                    batch_max_count,
                    poll_max_per_tick,
                    bucket_max,
                    meta_seed,
                    emit_receipts,
                    receipt_mode,
                    receipt_batch_window_ms,
                    receipt_jitter_ms,
                    file_confirm_mode,
                };
                receive_execute(args);
            }
        }
        Some(Cmd::Handshake { cmd }) => match cmd {
            HandshakeCmd::Init {
                as_label,
                peer,
                relay,
            } => handshake_init(&as_label, &peer, &relay),
            HandshakeCmd::Poll {
                as_label,
                peer,
                relay,
                max,
            } => handshake_poll(&as_label, &peer, &relay, max),
            HandshakeCmd::Status { peer } => handshake_status(peer.as_deref()),
        },
        Some(Cmd::Identity { cmd }) => match cmd {
            IdentityCmd::Show { as_label } => identity_show(&as_label),
            IdentityCmd::Rotate {
                as_label,
                confirm,
                reset_peers,
            } => identity_rotate(&as_label, confirm, reset_peers),
        },
        Some(Cmd::Peers { cmd }) => match cmd {
            PeersCmd::List => peers_list(),
        },
        Some(Cmd::Contacts { cmd }) => match cmd {
            ContactsCmd::Add {
                label,
                fp,
                route_token,
                verify,
            } => contacts_add(&label, &fp, route_token.as_deref(), verify),
            ContactsCmd::Show { label } => contacts_show(&label),
            ContactsCmd::List => contacts_list(),
            ContactsCmd::Verify { label, fp, confirm } => contacts_verify(&label, &fp, confirm),
            ContactsCmd::Block { label } => contacts_block(&label),
            ContactsCmd::Unblock { label } => contacts_unblock(&label),
            ContactsCmd::RouteSet { label, route_token } => {
                contacts_route_set(&label, &route_token)
            }
            ContactsCmd::Device { cmd } => match cmd {
                ContactsDeviceCmd::Add {
                    label,
                    fp,
                    route_token,
                } => contacts_device_add(&label, &fp, route_token.as_deref()),
                ContactsDeviceCmd::List { label } => contacts_device_list(&label),
                ContactsDeviceCmd::Status { label, device } => {
                    contacts_device_status(&label, device.as_deref())
                }
                ContactsDeviceCmd::Verify { label, device, fp } => {
                    contacts_device_verify(&label, &device, &fp)
                }
                ContactsDeviceCmd::Trust {
                    label,
                    device,
                    confirm,
                } => contacts_device_trust(&label, &device, confirm),
                ContactsDeviceCmd::Revoke {
                    label,
                    device,
                    confirm,
                } => contacts_device_revoke(&label, &device, confirm),
                ContactsDeviceCmd::Primary { cmd } => match cmd {
                    ContactsDevicePrimaryCmd::Set {
                        label,
                        device,
                        confirm,
                    } => contacts_device_primary_set(&label, &device, confirm),
                    ContactsDevicePrimaryCmd::Show { label } => {
                        contacts_device_primary_show(&label)
                    }
                },
            },
            ContactsCmd::TrustMode { cmd } => match cmd {
                ContactsTrustModeCmd::Show => contacts_trust_mode_show(),
                ContactsTrustModeCmd::Set { mode } => contacts_trust_mode_set(mode),
            },
            ContactsCmd::Request { cmd } => match cmd {
                ContactsRequestCmd::List => contacts_request_list(),
                ContactsRequestCmd::Accept { label } => contacts_request_accept(&label),
                ContactsRequestCmd::Ignore { label } => contacts_request_ignore(&label),
                ContactsRequestCmd::Block { label } => contacts_request_block(&label),
            },
        },
        Some(Cmd::Timeline { cmd }) => match cmd {
            TimelineCmd::List { peer, limit } => timeline_list(&peer, limit),
            TimelineCmd::Show { peer, id } => timeline_show(&peer, &id),
            TimelineCmd::Clear { peer, confirm } => timeline_clear(&peer, confirm),
        },
        Some(Cmd::File { cmd }) => match cmd {
            FileCmd::Send {
                transport,
                relay,
                attachment_service,
                legacy_in_message_stage,
                to,
                path,
                chunk_size,
                max_file_size,
                max_chunks,
                receipt,
            } => file_send_execute(FileSendExec {
                transport,
                relay: relay.as_deref(),
                attachment_service: attachment_service.as_deref(),
                legacy_in_message_stage,
                to: to.as_str(),
                path: path.as_path(),
                chunk_size,
                max_file_size,
                max_chunks,
                receipt,
            }),
        },
        Some(Cmd::Tui {
            headless,
            transport: _transport,
            relay,
            token_file,
            seed,
            scenario,
        }) => tui_entry(
            headless,
            TuiConfig {
                relay,
                token_file,
                seed,
                scenario,
            },
        ),
        Some(Cmd::Relay { cmd }) => relay_cmd(cmd),
        Some(Cmd::Meta { cmd }) => meta_cmd(cmd),
    }
}

fn relay_cmd(cmd: RelayCmd) {
    match cmd {
        RelayCmd::Serve {
            port,
            seed,
            drop_pct,
            dup_pct,
            reorder_window,
            fixed_latency_ms,
            jitter_ms,
            max_messages,
        } => {
            if drop_pct > 100 || dup_pct > 100 {
                print_error_marker("relay_pct_invalid");
            }
            let cfg = RelayConfig {
                seed,
                drop_pct,
                dup_pct,
                reorder_window,
                fixed_latency_ms,
                jitter_ms,
            };
            relay_serve(port, cfg, max_messages);
        }
        RelayCmd::Send {
            to,
            file,
            relay,
            bucket_max,
        } => {
            if !require_unlocked("relay_send") {
                return;
            }
            relay_send(&to, &file, &relay, None, bucket_max, None, None)
        }
        RelayCmd::InboxSet { token } => {
            if !require_unlocked("relay_inbox_set") {
                return;
            }
            let token = normalize_route_token(token.as_str())
                .unwrap_or_else(|code| print_error_marker(code));
            if vault::secret_set(TUI_RELAY_INBOX_TOKEN_SECRET_KEY, token.as_str()).is_err() {
                print_error_marker("relay_inbox_token_store_failed");
            }
            let hash = route_token_hash8(token.as_str());
            emit_marker(
                "relay_inbox_set",
                None,
                &[
                    ("ok", "true"),
                    ("token", "redacted"),
                    ("token_hash", hash.as_str()),
                ],
            );
            println!("relay_inbox_token=set hash={}", hash);
        }
        RelayCmd::InboxClear => {
            if !require_unlocked("relay_inbox_clear") {
                return;
            }
            if vault::secret_set(TUI_RELAY_INBOX_TOKEN_SECRET_KEY, "").is_err() {
                print_error_marker("relay_inbox_token_store_failed");
            }
            emit_marker(
                "relay_inbox_clear",
                None,
                &[("ok", "true"), ("token", "cleared")],
            );
            println!("relay_inbox_token=cleared");
        }
    }
}

fn meta_cmd(cmd: MetaCmd) {
    match cmd {
        MetaCmd::Plan {
            deterministic,
            tick_count,
            interval_ms,
            bucket_max,
            batch_max_count,
            cover_enabled,
        } => {
            let cfg = match meta_poll_config_from_args(MetaPollArgs {
                deterministic_meta: deterministic,
                interval_ms: Some(interval_ms),
                poll_interval_ms: None,
                ticks: Some(tick_count),
                batch_max_count: Some(batch_max_count),
                poll_max_per_tick: None,
                bucket_max: Some(bucket_max),
                meta_seed: None,
            }) {
                Ok(Some(v)) => v,
                Ok(None) => print_error_marker("meta_poll_invalid"),
                Err(code) => print_error_marker(code),
            };
            let deterministic_s = if cfg.deterministic { "true" } else { "false" };
            let ticks_s = cfg.ticks.to_string();
            let interval_s = cfg.interval_ms.to_string();
            let bucket_s = cfg.bucket_max.to_string();
            let batch_s = cfg.batch_max_count.to_string();
            emit_marker(
                "meta_plan",
                None,
                &[
                    ("deterministic", deterministic_s),
                    ("ticks", ticks_s.as_str()),
                    ("interval_ms", interval_s.as_str()),
                    ("bucket_max", bucket_s.as_str()),
                    ("batch_max_count", batch_s.as_str()),
                ],
            );
            for tick in 0..cfg.ticks {
                let tick_s = tick.to_string();
                let bucket = meta_bucket_for_len(1, cfg.bucket_max);
                let bucket_out_s = bucket.to_string();
                let planned_count_s = cfg.batch_max_count.to_string();
                emit_marker(
                    "meta_tick",
                    None,
                    &[
                        ("tick", tick_s.as_str()),
                        ("interval_ms", interval_s.as_str()),
                        ("deterministic", deterministic_s),
                    ],
                );
                emit_marker(
                    "meta_bucket",
                    None,
                    &[
                        ("bucket", bucket_out_s.as_str()),
                        ("orig", "1"),
                        ("capped", "1"),
                        ("metric", "planned_envelope_len"),
                    ],
                );
                emit_marker(
                    "meta_batch",
                    None,
                    &[
                        ("count", planned_count_s.as_str()),
                        ("bytes", "0"),
                        ("planned", "true"),
                    ],
                );
                if cover_enabled {
                    emit_marker(
                        "meta_cover",
                        None,
                        &[("enabled", "true"), ("tick", tick_s.as_str())],
                    );
                }
            }
        }
    }
}

struct TuiConfig {
    relay: Option<String>,
    token_file: Option<PathBuf>,
    seed: u64,
    scenario: String,
}

#[derive(Clone)]
struct TuiRelayConfig {
    relay: String,
    seed: u64,
    scenario: String,
}

#[derive(Debug)]
struct RelayTestOutcome {
    ok: bool,
    code: &'static str,
    message: String,
}

fn emit_tui_relay_test_event(result: &'static str, code: &'static str) {
    emit_tui_named_marker("QSC_TUI_RELAY_TEST", &[("result", result), ("code", code)]);
}

fn run_relay_test_probe(
    endpoint: &str,
    token: Option<String>,
    token_file: Option<&str>,
) -> RelayTestOutcome {
    let url = match relay_probe_url(endpoint) {
        Ok(v) => v,
        Err(code) => {
            return RelayTestOutcome {
                ok: false,
                code,
                message: code.to_string(),
            };
        }
    };
    let client = match HttpClient::builder()
        .timeout(Duration::from_secs(2))
        .build()
    {
        Ok(v) => v,
        Err(_) => {
            return RelayTestOutcome {
                ok: false,
                code: "relay_client_init_failed",
                message: "client init failed".to_string(),
            };
        }
    };
    let mut auth_token = token
        .as_ref()
        .map(|v| v.trim().to_string())
        .filter(|v| !v.is_empty());
    if auth_token.is_none() {
        if let Some(path) = token_file {
            match read_relay_token_file(path) {
                Ok(v) => auth_token = Some(v),
                Err(code) => {
                    return RelayTestOutcome {
                        ok: false,
                        code,
                        message: relay_user_reason_from_code(code).to_string(),
                    };
                }
            }
        }
    }
    let mut req = client
        .get(url.as_str())
        .header("X-QSL-Route-Token", "qsc-relay-probe");
    if let Some(token) = auth_token.as_ref() {
        req = req.bearer_auth(token);
    }
    match req.send() {
        Ok(resp) => {
            let status = resp.status().as_u16();
            if status == 200 || status == 204 {
                RelayTestOutcome {
                    ok: true,
                    code: "relay_authenticated",
                    message: "authenticated".to_string(),
                }
            } else if status == 401 {
                RelayTestOutcome {
                    ok: false,
                    code: "relay_unauthorized",
                    message: "unauthorized (401)".to_string(),
                }
            } else if status == 429 {
                RelayTestOutcome {
                    ok: false,
                    code: "relay_overloaded",
                    message: "overloaded (429)".to_string(),
                }
            } else {
                RelayTestOutcome {
                    ok: false,
                    code: "relay_http_failure",
                    message: format!("http {}", status),
                }
            }
        }
        Err(err) => {
            let txt = err.to_string().to_ascii_lowercase();
            if txt.contains("dns")
                || txt.contains("name or service")
                || txt.contains("failed to lookup")
            {
                RelayTestOutcome {
                    ok: false,
                    code: "relay_dns_failure",
                    message: "dns failure".to_string(),
                }
            } else if txt.contains("timed out") {
                RelayTestOutcome {
                    ok: false,
                    code: "relay_network_timeout",
                    message: "network timeout".to_string(),
                }
            } else {
                RelayTestOutcome {
                    ok: false,
                    code: "relay_network_unreachable",
                    message: "network unreachable".to_string(),
                }
            }
        }
    }
}

fn tui_entry(headless: bool, cfg: TuiConfig) {
    let env_headless = env_bool("QSC_TUI_HEADLESS");
    let env_test_mode = env_bool("QSC_TUI_TEST_MODE");
    let headless = headless || env_headless;
    if headless {
        eprintln!("QSC_TUI_STARTUP OK mode=headless");
        tui_headless(cfg);
        return;
    }
    if env_test_mode {
        eprintln!("QSC_TUI_STARTUP OK mode=headless");
        tui_interactive_test(cfg);
        return;
    }
    if let Err(code) = tui_startup_preflight() {
        emit_tui_startup_fail(code);
        process::exit(2);
    }
    if let Err(code) = tui_interactive(cfg) {
        emit_tui_startup_fail(code);
        process::exit(2);
    }
}

fn tui_headless(cfg: TuiConfig) {
    set_marker_routing(MarkerRouting::Stdout);
    let mut state = TuiState::new(cfg);
    emit_marker("tui_open", None, &[]);
    state.emit_home_render_marker(terminal_cols_for_headless(), terminal_rows_for_headless());
    for line in load_tui_script() {
        state.poll_relay_test_task();
        if let Some(wait_ms) = parse_tui_wait_ms(&line) {
            state.headless_advance_clock(wait_ms);
            state.poll_relay_test_task();
            state.emit_home_render_marker(
                terminal_cols_for_headless(),
                terminal_rows_for_headless(),
            );
            continue;
        }
        if let Some(tag) = parse_tui_perf_snapshot(&line) {
            let (kdf, reads, decrypts, writes) = vault::perf_snapshot();
            let kdf_s = kdf.to_string();
            let reads_s = reads.to_string();
            let decrypts_s = decrypts.to_string();
            let writes_s = writes.to_string();
            emit_marker(
                "tui_perf",
                None,
                &[
                    ("tag", tag.as_str()),
                    ("kdf", kdf_s.as_str()),
                    ("reads", reads_s.as_str()),
                    ("decrypts", decrypts_s.as_str()),
                    ("writes", writes_s.as_str()),
                ],
            );
            continue;
        }
        if let Some(cmd) = parse_tui_command(&line) {
            if handle_tui_command(&cmd, &mut state) {
                state.wait_for_relay_test_task_headless();
                emit_marker("tui_exit", None, &[]);
                return;
            }
            state.wait_for_relay_test_task_headless();
            state.poll_relay_test_task();
            state.emit_home_render_marker(
                terminal_cols_for_headless(),
                terminal_rows_for_headless(),
            );
        } else {
            state.mark_input_activity(state.headless_now_ms());
            emit_marker("tui_input_text", None, &[("kind", "plain")]);
        }
    }
    emit_marker("tui_exit", None, &[]);
}

fn terminal_cols_for_headless() -> u16 {
    env::var("QSC_TUI_COLS")
        .ok()
        .and_then(|v| v.parse::<u16>().ok())
        .filter(|v| *v > 0)
        .unwrap_or(140)
}

fn terminal_rows_for_headless() -> u16 {
    env::var("QSC_TUI_ROWS")
        .ok()
        .and_then(|v| v.parse::<u16>().ok())
        .filter(|v| *v > 0)
        .unwrap_or(40)
}

fn tui_next_poll_timeout_ms() -> u64 {
    const TUI_POLL_MS_DEFAULT: u64 = 200;
    const TUI_POLL_MS_MIN: u64 = 50;
    TUI_POLL_MS_DEFAULT.max(TUI_POLL_MS_MIN)
}

fn tui_deterministic_timestamps() -> bool {
    env_bool("QSC_TUI_DETERMINISTIC") || env_bool("QSC_TUI_HEADLESS")
}

fn tui_timestamp_token(idx: usize) -> String {
    format!("t={:04}", idx.saturating_add(1))
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum TuiStartupCode {
    StdinNotTty,
    StdoutNotTty,
    TermInvalid,
    StdinClosed,
    RawModeFailed,
    AltScreenFailed,
    EventStreamFailed,
    Unknown,
}

impl TuiStartupCode {
    fn as_str(self) -> &'static str {
        match self {
            Self::StdinNotTty => "stdin_not_tty",
            Self::StdoutNotTty => "stdout_not_tty",
            Self::TermInvalid => "term_invalid",
            Self::StdinClosed => "stdin_closed",
            Self::RawModeFailed => "raw_mode_failed",
            Self::AltScreenFailed => "alt_screen_failed",
            Self::EventStreamFailed => "event_stream_failed",
            Self::Unknown => "unknown",
        }
    }
}

fn emit_tui_startup_fail(code: TuiStartupCode) {
    eprintln!("QSC_TUI_STARTUP FAIL code={}", code.as_str());
    eprintln!(
        "HINT: run in an interactive terminal (stdin+stdout must be a TTY). If running non-interactively, set QSC_TUI_HEADLESS=1."
    );
}

fn tui_startup_preflight() -> Result<(), TuiStartupCode> {
    if !std::io::stdin().is_terminal() {
        return Err(TuiStartupCode::StdinNotTty);
    }
    if !std::io::stdout().is_terminal() {
        return Err(TuiStartupCode::StdoutNotTty);
    }
    let term = env::var("TERM").unwrap_or_default();
    if term.trim().is_empty() || term.eq_ignore_ascii_case("dumb") {
        return Err(TuiStartupCode::TermInvalid);
    }
    match event::poll(Duration::from_millis(0)) {
        Ok(_) => Ok(()),
        Err(_) => Err(TuiStartupCode::StdinClosed),
    }
}

fn event_error_code(err: &std::io::Error) -> TuiStartupCode {
    match err.kind() {
        std::io::ErrorKind::UnexpectedEof
        | std::io::ErrorKind::BrokenPipe
        | std::io::ErrorKind::ConnectionAborted
        | std::io::ErrorKind::NotConnected => TuiStartupCode::StdinClosed,
        _ => TuiStartupCode::EventStreamFailed,
    }
}

fn tui_interactive(cfg: TuiConfig) -> Result<(), TuiStartupCode> {
    set_marker_routing(MarkerRouting::InApp);
    let mut state = TuiState::new(cfg);
    emit_marker("tui_open", None, &[]);
    enable_raw_mode().map_err(|_| TuiStartupCode::RawModeFailed)?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen).map_err(|_| TuiStartupCode::AltScreenFailed)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).map_err(|_| TuiStartupCode::Unknown)?;
    let started = Instant::now();

    let mut exit = false;
    let mut last_draw_ms = 0u64;
    let result = loop {
        let now_ms = started.elapsed().as_millis() as u64;
        state.drain_marker_queue();
        state.poll_relay_test_task();
        let force_full_redraw = state.take_force_full_redraw() || state.take_clear_screen_pending();
        if force_full_redraw
            || state.needs_redraw
            || now_ms == 0
            || now_ms.saturating_sub(last_draw_ms) >= 1_000
        {
            if terminal
                .draw(|f| {
                    if force_full_redraw {
                        let area = f.area();
                        f.render_widget(TuiClear, area);
                    }
                    draw_tui(f, &mut state);
                })
                .is_err()
            {
                break Err(TuiStartupCode::EventStreamFailed);
            }
            state.needs_redraw = false;
            last_draw_ms = now_ms;
        }

        let polled = match event::poll(Duration::from_millis(tui_next_poll_timeout_ms())) {
            Ok(polled) => polled,
            Err(err) => break Err(event_error_code(&err)),
        };
        if polled {
            let event = match event::read() {
                Ok(event) => event,
                Err(err) => break Err(event_error_code(&err)),
            };
            if let Event::Key(key) = event {
                state.mark_input_activity(now_ms);
                exit = handle_tui_key(&mut state, key);
                state.request_redraw();
            }
        } else if state.maybe_autolock(now_ms) {
            state.request_redraw();
        }
        if state.maybe_run_fixed_poll(now_ms) {
            state.request_redraw();
        }
        if exit {
            break Ok(());
        }
    };

    let _ = disable_raw_mode();
    let _ = execute!(terminal.backend_mut(), LeaveAlternateScreen);
    let _ = terminal.show_cursor();
    if result.is_ok() {
        emit_marker("tui_exit", None, &[]);
    } else {
        emit_marker("tui_exit", Some("io"), &[]);
    }
    result
}

fn focus_mode_for_fkey(code: KeyCode) -> Option<TuiMode> {
    match code {
        KeyCode::F(2) => Some(TuiMode::FocusEvents),
        KeyCode::F(3) => Some(TuiMode::FocusStatus),
        KeyCode::F(4) => Some(TuiMode::FocusSession),
        KeyCode::F(5) => Some(TuiMode::FocusContacts),
        _ => None,
    }
}

fn inspector_for_fkey(code: KeyCode) -> Option<TuiInspectorPane> {
    match code {
        KeyCode::F(2) => Some(TuiInspectorPane::Events),
        KeyCode::F(3) => Some(TuiInspectorPane::Status),
        KeyCode::F(4) => Some(TuiInspectorPane::Session),
        KeyCode::F(5) => Some(TuiInspectorPane::Contacts),
        _ => None,
    }
}

fn locked_cmd_input_value(input: &str, command: &str) -> String {
    let trimmed = input.trim();
    let prefix = format!("/{}", command);
    if trimmed == prefix {
        return String::new();
    }
    if let Some(rest) = trimmed.strip_prefix(&(prefix.clone() + " ")) {
        return rest.trim().to_string();
    }
    trimmed.to_string()
}

fn handle_locked_prompt_submit(state: &mut TuiState) -> bool {
    match state.locked_flow.clone() {
        LockedFlow::None => {
            if let Some(cmd) = parse_tui_command(state.cmd_input.as_str()) {
                let exit = handle_tui_command(&cmd, state);
                state.cmd_input_clear();
                return exit;
            }
            state.cmd_input_clear();
            state.locked_clear_error();
            false
        }
        LockedFlow::UnlockPassphrase => {
            let mut passphrase = locked_cmd_input_value(state.cmd_input.as_str(), "unlock");
            state.cmd_input_clear();
            if passphrase.is_empty() {
                state.locked_set_error("passphrase required");
                emit_marker(
                    "tui_unlock",
                    Some("vault_locked"),
                    &[("ok", "false"), ("reason", "passphrase_required")],
                );
                return false;
            }
            match state.unlock_with_policy(passphrase.as_str()) {
                UnlockAttemptOutcome::Unlocked => {
                    state.set_locked_state(false, "explicit_command");
                    state.locked_clear_error();
                    emit_marker("tui_unlock", None, &[("ok", "true")]);
                }
                UnlockAttemptOutcome::Wiped => {
                    state.locked_set_error(
                        "vault wiped after failed unlock attempts; run /init to rebuild local state",
                    );
                    state.command_error = Some(format!(
                        "vault: {} (run /init to rebuild local state)",
                        QSC_ERR_VAULT_WIPED_AFTER_FAILED_UNLOCKS
                    ));
                }
                UnlockAttemptOutcome::Rejected => {
                    state
                        .locked_set_error("unlock failed: passphrase did not open the local vault");
                    emit_marker(
                        "tui_unlock",
                        Some("vault_locked"),
                        &[("ok", "false"), ("reason", "passphrase_invalid")],
                    );
                }
            }
            passphrase.zeroize();
            false
        }
        LockedFlow::InitAlias => {
            let alias = locked_cmd_input_value(state.cmd_input.as_str(), "init");
            if !tui_alias_is_valid(alias.as_str()) {
                state.locked_set_error("alias must be 2-32 chars [A-Za-z0-9._-]");
                emit_marker(
                    "tui_init_reject",
                    Some("alias_invalid"),
                    &[("ok", "false"), ("reason", "alias_invalid")],
                );
                state.cmd_input_clear();
                return false;
            }
            state.locked_flow = LockedFlow::InitPassphrase { alias };
            state.cmd_input_clear();
            state.locked_clear_error();
            emit_marker("tui_init_wizard", None, &[("step", "passphrase")]);
            false
        }
        LockedFlow::InitPassphrase { alias } => {
            let passphrase = state.cmd_input.clone();
            if !tui_passphrase_is_strong(passphrase.as_str()) {
                state.locked_set_error("passphrase must be 16+ chars and not weak/common");
                emit_marker(
                    "tui_init_reject",
                    Some("passphrase_weak"),
                    &[("ok", "false"), ("reason", "passphrase_weak")],
                );
                state.cmd_input_clear();
                return false;
            }
            state.locked_flow = LockedFlow::InitConfirm { alias, passphrase };
            state.cmd_input_clear();
            state.locked_clear_error();
            emit_marker("tui_init_wizard", None, &[("step", "confirm")]);
            false
        }
        LockedFlow::InitConfirm { alias, passphrase } => {
            let confirm = state.cmd_input.clone();
            state.cmd_input_clear();
            if confirm != passphrase {
                state.locked_set_error("passphrase confirmation does not match");
                emit_marker(
                    "tui_init_reject",
                    Some("passphrase_mismatch"),
                    &[("ok", "false"), ("reason", "passphrase_mismatch")],
                );
                state.locked_flow = LockedFlow::InitPassphrase { alias };
                emit_marker("tui_init_wizard", None, &[("step", "passphrase")]);
                return false;
            }
            state.locked_flow = LockedFlow::InitDecision { alias, passphrase };
            state.locked_clear_error();
            emit_marker("tui_init_wizard", None, &[("step", "confirm_decision")]);
            false
        }
        LockedFlow::InitDecision { alias, passphrase } => {
            let decision = state.cmd_input.trim().to_ascii_uppercase();
            if decision == "N" || decision == "NO" {
                state.locked_flow = LockedFlow::None;
                state.cmd_input_clear();
                state.locked_clear_error();
                emit_marker(
                    "tui_init_reject",
                    Some("confirm_cancelled"),
                    &[("ok", "false"), ("reason", "confirm_cancelled")],
                );
                return false;
            }
            if decision != "Y"
                && decision != "YES"
                && decision != "I AGREE"
                && decision != "I UNDERSTAND"
            {
                state.locked_set_error("confirm with I AGREE or N");
                emit_marker(
                    "tui_init_reject",
                    Some("confirm_required"),
                    &[("ok", "false"), ("reason", "confirm_required")],
                );
                state.cmd_input_clear();
                return false;
            }
            match tui_try_vault_init(passphrase.as_str()) {
                Ok(()) => {}
                Err(code) => {
                    state.locked_set_error("vault init failed");
                    emit_marker(
                        "tui_init_reject",
                        Some(code.as_str()),
                        &[("ok", "false"), ("reason", "vault_init_failed")],
                    );
                    state.locked_flow = LockedFlow::InitAlias;
                    emit_marker("tui_init_wizard", None, &[("step", "alias")]);
                    return false;
                }
            }
            if let Err(code) = initialize_account_after_init(alias.as_str(), passphrase.as_str()) {
                emit_marker(
                    "tui_init_reject",
                    Some(code.as_str()),
                    &[("ok", "false"), ("reason", "account_init_failed")],
                );
                state.locked_flow = LockedFlow::InitAlias;
                state.locked_set_error("failed to initialize account");
                emit_marker("tui_init_wizard", None, &[("step", "alias")]);
                return false;
            }
            let _ = vault_security_state_clear_files();
            state.unlock_attempt_limit = None;
            state.failed_unlock_attempts = 0;
            state.mark_vault_present();
            state.set_locked_state(true, "post_init_locked");
            state.locked_flow = LockedFlow::None;
            state.locked_clear_error();
            emit_marker(
                "tui_init",
                None,
                &[("ok", "true"), ("alias", "stored_local_only")],
            );
            false
        }
    }
}

fn handle_tui_locked_key(state: &mut TuiState, key: KeyEvent) -> bool {
    let no_ctrl_alt = !key
        .modifiers
        .intersects(KeyModifiers::CONTROL | KeyModifiers::ALT | KeyModifiers::SUPER);
    match key.code {
        KeyCode::Up => {
            state.nav_move(-1);
            false
        }
        KeyCode::Down => {
            state.nav_move(1);
            false
        }
        KeyCode::Enter => {
            if state.home_focus == TuiHomeFocus::Nav {
                state.locked_nav_activate()
            } else if state.home_focus == TuiHomeFocus::Command {
                handle_locked_prompt_submit(state)
            } else {
                false
            }
        }
        KeyCode::Tab => {
            state.locked_focus_toggle();
            false
        }
        KeyCode::Esc => {
            state.home_focus = TuiHomeFocus::Nav;
            state.locked_flow = LockedFlow::None;
            state.locked_clear_error();
            state.cmd_input_clear();
            emit_marker("tui_focus_home", None, &[("pane", state.home_focus_name())]);
            false
        }
        KeyCode::Char('/') => {
            state.home_focus = TuiHomeFocus::Command;
            state.cmd_input_push('/');
            state.locked_clear_error();
            emit_marker("tui_focus_home", None, &[("pane", state.home_focus_name())]);
            false
        }
        KeyCode::Backspace | KeyCode::Delete => {
            if state.home_focus == TuiHomeFocus::Command {
                if !state.cmd_input.is_empty() {
                    state.cmd_input_pop();
                    state.locked_clear_error();
                } else {
                    match state.locked_flow.clone() {
                        LockedFlow::InitPassphrase { alias: _ } => {
                            state.locked_flow = LockedFlow::InitAlias;
                            state.locked_clear_error();
                            emit_marker("tui_init_wizard", None, &[("step", "alias")]);
                        }
                        LockedFlow::InitConfirm { alias, .. } => {
                            state.locked_flow = LockedFlow::InitPassphrase { alias };
                            state.locked_clear_error();
                            emit_marker("tui_init_wizard", None, &[("step", "passphrase")]);
                        }
                        LockedFlow::InitDecision { alias, passphrase } => {
                            state.locked_flow = LockedFlow::InitConfirm { alias, passphrase };
                            state.locked_clear_error();
                            emit_marker("tui_init_wizard", None, &[("step", "confirm")]);
                        }
                        _ => {}
                    }
                }
            }
            false
        }
        KeyCode::Char(ch) => {
            if no_ctrl_alt && state.home_focus == TuiHomeFocus::Command && !ch.is_control() {
                state.cmd_input_push(ch);
                state.locked_clear_error();
            }
            false
        }
        _ => false,
    }
}

fn handle_tui_key(state: &mut TuiState, key: KeyEvent) -> bool {
    if state.is_locked() {
        return handle_tui_locked_key(state, key);
    }
    if state.account_destroy_active() {
        return handle_tui_account_destroy_key(state, key);
    }
    state.clear_command_feedback();
    if state.is_help_mode() {
        match key.code {
            KeyCode::Esc => state.exit_help_mode(),
            KeyCode::F(1) => state.toggle_help_mode(),
            KeyCode::Char('?') => state.toggle_help_mode(),
            KeyCode::Up => state.help_move(-1),
            KeyCode::Down => state.help_move(1),
            _ => {}
        }
        return false;
    }
    if state.is_focus_mode() {
        match key.code {
            KeyCode::Esc => state.exit_focus_mode(),
            KeyCode::Up => {
                if state.mode == TuiMode::FocusContacts {
                    state.contacts_move(-1);
                } else if state.mode == TuiMode::FocusFiles {
                    state.files_move(-1);
                } else {
                    let max_len = state.focus_max_len();
                    state.focus_scroll_move(-1, max_len);
                }
            }
            KeyCode::Down => {
                if state.mode == TuiMode::FocusContacts {
                    state.contacts_move(1);
                } else if state.mode == TuiMode::FocusFiles {
                    state.files_move(1);
                } else {
                    let max_len = state.focus_max_len();
                    state.focus_scroll_move(1, max_len);
                }
            }
            KeyCode::PageUp => {
                if state.mode == TuiMode::FocusContacts {
                    state.contacts_move(-(state.focus_view_rows() as i32));
                } else if state.mode == TuiMode::FocusFiles {
                    state.files_move(-(state.focus_view_rows() as i32));
                } else {
                    let max_len = state.focus_max_len();
                    state.focus_scroll_move(-(state.focus_view_rows() as i32), max_len);
                }
            }
            KeyCode::PageDown => {
                if state.mode == TuiMode::FocusContacts {
                    state.contacts_move(state.focus_view_rows() as i32);
                } else if state.mode == TuiMode::FocusFiles {
                    state.files_move(state.focus_view_rows() as i32);
                } else {
                    let max_len = state.focus_max_len();
                    state.focus_scroll_move(state.focus_view_rows() as i32, max_len);
                }
            }
            _ => {
                if let Some(mode) = focus_mode_for_fkey(key.code) {
                    state.enter_focus_mode(mode);
                }
            }
        }
        return false;
    }
    match key.code {
        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => return true,
        KeyCode::Char('l') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            emit_marker("tui_cmd", None, &[("cmd", "lock_shortcut")]);
            state.set_locked_state(true, "ctrl_l_shortcut");
        }
        KeyCode::Esc => {
            state.home_focus = TuiHomeFocus::Nav;
            state.cmd_input_clear();
            emit_marker("tui_focus_home", None, &[("pane", state.home_focus_name())]);
        }
        KeyCode::Tab => {
            state.home_focus_cycle(1);
        }
        KeyCode::BackTab => {
            state.home_focus_cycle(-1);
        }
        KeyCode::F(1) | KeyCode::Char('?') => {
            if state.is_locked() {
                handle_locked_reject(state, "help", "locked_unlock_required");
            } else {
                state.toggle_help_mode();
            }
        }
        KeyCode::Enter => {
            if state.home_focus == TuiHomeFocus::Nav {
                state.nav_activate();
            } else if state.home_focus != TuiHomeFocus::Command {
                state.enter_focus_mode(state.focus_mode_for_inspector());
            } else if let Some(cmd) = parse_tui_command(state.cmd_input.as_str()) {
                let exit = handle_tui_command(&cmd, state);
                state.cmd_input_clear();
                return exit;
            } else if !state.cmd_input.is_empty() {
                emit_marker("tui_input_text", None, &[("kind", "plain")]);
            }
            state.cmd_input_clear();
        }
        KeyCode::Backspace => {
            if state.home_focus == TuiHomeFocus::Command {
                state.cmd_input_pop();
            }
        }
        KeyCode::Up => {
            if state.home_focus == TuiHomeFocus::Main {
                state.main_scroll_move(-1);
            } else {
                state.nav_move(-1);
            }
        }
        KeyCode::Down => {
            if state.home_focus == TuiHomeFocus::Main {
                state.main_scroll_move(1);
            } else {
                state.nav_move(1);
            }
        }
        KeyCode::PageUp => {
            if state.home_focus == TuiHomeFocus::Main {
                state.main_scroll_page(-1);
            }
        }
        KeyCode::PageDown => {
            if state.home_focus == TuiHomeFocus::Main {
                state.main_scroll_page(1);
            }
        }
        KeyCode::Home => {
            if state.home_focus == TuiHomeFocus::Main {
                state.main_scroll_home();
            }
        }
        KeyCode::End => {
            if state.home_focus == TuiHomeFocus::Main {
                state.main_scroll_end();
            }
        }
        KeyCode::Char(ch) => {
            if state.home_focus == TuiHomeFocus::Command {
                state.cmd_input_push(ch);
            } else if ch == '/' {
                state.home_focus = TuiHomeFocus::Command;
                emit_marker("tui_focus_home", None, &[("pane", state.home_focus_name())]);
                state.cmd_input_push(ch);
            }
        }
        _ => {
            if key.modifiers.contains(KeyModifiers::CONTROL) {
                if let Some(mode) = focus_mode_for_fkey(key.code) {
                    state.enter_focus_mode(mode);
                }
            } else if let Some(pane) = inspector_for_fkey(key.code) {
                state.set_inspector(pane);
            }
        }
    }
    false
}

fn handle_tui_account_destroy_key(state: &mut TuiState, key: KeyEvent) -> bool {
    let no_ctrl_alt = !key
        .modifiers
        .intersects(KeyModifiers::CONTROL | KeyModifiers::ALT);
    match key.code {
        KeyCode::Esc => {
            state.cancel_account_destroy_prompt();
            false
        }
        KeyCode::Enter => {
            if state.home_focus != TuiHomeFocus::Command {
                state.home_focus = TuiHomeFocus::Command;
                emit_marker("tui_focus_home", None, &[("pane", state.home_focus_name())]);
                return false;
            }
            match state.account_destroy_flow.clone() {
                AccountDestroyFlow::None => false,
                AccountDestroyFlow::Passphrase => {
                    if state.cmd_input.is_empty() {
                        state.account_destroy_set_error("passphrase required");
                        state.push_cmd_result("account destroy", false, "passphrase required");
                        return false;
                    }
                    if vault::unlock_with_passphrase(state.cmd_input.as_str()).is_err() {
                        state.account_destroy_set_error("current passphrase invalid");
                        state.push_cmd_result("account destroy", false, "passphrase invalid");
                        state.cmd_input_clear();
                        return false;
                    }
                    let passphrase = state.cmd_input.clone();
                    state.account_destroy_flow = AccountDestroyFlow::ConfirmDecision { passphrase };
                    state.account_destroy_clear_error();
                    state.cmd_input_clear();
                    emit_marker("tui_account_destroy", None, &[("step", "confirm")]);
                    false
                }
                AccountDestroyFlow::ConfirmDecision { passphrase } => {
                    let mut passphrase = passphrase;
                    let decision = state.cmd_input.trim().to_ascii_uppercase();
                    if decision == "N" || decision == "NO" {
                        state.account_destroy_set_error("destroy cancelled");
                        state.push_cmd_result("account destroy", false, "cancelled");
                        state.cmd_input_clear();
                        state.cancel_account_destroy_prompt();
                        return false;
                    }
                    if decision != "Y" && decision != "YES" {
                        state.account_destroy_set_error("confirm with Y or N");
                        state.push_cmd_result("account destroy", false, "confirmation required");
                        state.cmd_input_clear();
                        return false;
                    }
                    match vault::destroy_with_passphrase(passphrase.as_str()) {
                        Ok(()) => {
                            wipe_account_local_state_best_effort();
                            let _ = vault_security_state_clear_files();
                            state.close_vault_session();
                            state.mark_vault_absent();
                            state.unlock_attempt_limit = None;
                            state.failed_unlock_attempts = 0;
                            state.account_destroy_flow = AccountDestroyFlow::None;
                            state.account_destroy_clear_error();
                            state.apply_default_account_settings();
                            state.cmd_results.clear();
                            state.status_last_command_result = None;
                            state.command_feedback = None;
                            state.push_cmd_result("account destroy", true, "vault destroyed");
                            state.set_locked_state(true, "account_destroy");
                            passphrase.zeroize();
                            false
                        }
                        Err(code) => {
                            state.account_destroy_set_error(format!("destroy failed: {}", code));
                            state.push_cmd_result(
                                "account destroy",
                                false,
                                format!("destroy failed ({})", code),
                            );
                            state.cmd_input_clear();
                            passphrase.zeroize();
                            false
                        }
                    }
                }
            }
        }
        KeyCode::Backspace => {
            if state.home_focus == TuiHomeFocus::Command {
                state.cmd_input_pop();
                state.account_destroy_clear_error();
            }
            false
        }
        KeyCode::Tab => {
            state.home_focus_cycle(1);
            false
        }
        KeyCode::BackTab => {
            state.home_focus_cycle(-1);
            false
        }
        KeyCode::Up => {
            if state.home_focus == TuiHomeFocus::Main {
                state.main_scroll_move(-1);
            } else {
                state.nav_move(-1);
            }
            false
        }
        KeyCode::Down => {
            if state.home_focus == TuiHomeFocus::Main {
                state.main_scroll_move(1);
            } else {
                state.nav_move(1);
            }
            false
        }
        KeyCode::PageUp => {
            if state.home_focus == TuiHomeFocus::Main {
                state.main_scroll_page(-1);
            }
            false
        }
        KeyCode::PageDown => {
            if state.home_focus == TuiHomeFocus::Main {
                state.main_scroll_page(1);
            }
            false
        }
        KeyCode::Home => {
            if state.home_focus == TuiHomeFocus::Main {
                state.main_scroll_home();
            }
            false
        }
        KeyCode::End => {
            if state.home_focus == TuiHomeFocus::Main {
                state.main_scroll_end();
            }
            false
        }
        KeyCode::Char(ch) => {
            if state.home_focus == TuiHomeFocus::Command && no_ctrl_alt && !ch.is_control() {
                state.cmd_input_push(ch);
                state.account_destroy_clear_error();
            } else if ch == '/' {
                state.home_focus = TuiHomeFocus::Command;
                emit_marker("tui_focus_home", None, &[("pane", state.home_focus_name())]);
                state.cmd_input_push(ch);
                state.account_destroy_clear_error();
            }
            false
        }
        _ => false,
    }
}

fn tui_interactive_test(cfg: TuiConfig) {
    set_marker_routing(MarkerRouting::InApp);
    let mut state = TuiState::new(cfg);
    emit_marker("tui_open", None, &[]);
    state.drain_marker_queue();
    println!("tui_test_done");
}

fn load_tui_script() -> Vec<String> {
    if let Ok(path) = env::var("QSC_TUI_SCRIPT_FILE") {
        if let Ok(text) = fs::read_to_string(path) {
            return parse_script_lines(&text);
        }
    }
    if let Ok(text) = env::var("QSC_TUI_SCRIPT") {
        return parse_script_lines(&text);
    }
    vec!["/exit".to_string()]
}

fn parse_script_lines(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    for line in text.lines() {
        for part in line.split(';') {
            let trimmed = part.trim();
            if !trimmed.is_empty() {
                out.push(trimmed.to_string());
            }
        }
    }
    out
}

struct TuiParsedCmd {
    cmd: String,
    args: Vec<String>,
}

fn parse_tui_command(line: &str) -> Option<TuiParsedCmd> {
    let trimmed = line.trim();
    if !trimmed.starts_with('/') {
        return None;
    }
    let parts = parse_tui_command_tokens(trimmed.trim_start_matches('/'));
    let cmd = parts.first()?;
    if cmd.is_empty() {
        return None;
    }
    let args = parts.iter().skip(1).cloned().collect::<Vec<_>>();
    Some(TuiParsedCmd {
        cmd: cmd.clone(),
        args,
    })
}

fn parse_tui_command_tokens(input: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut buf = String::new();
    let mut chars = input.chars().peekable();
    let mut in_quotes = false;
    while let Some(ch) = chars.next() {
        match ch {
            '"' => {
                in_quotes = !in_quotes;
            }
            '\\' if in_quotes => {
                if let Some(next) = chars.next() {
                    buf.push(next);
                }
            }
            c if c.is_whitespace() && !in_quotes => {
                if !buf.is_empty() {
                    out.push(std::mem::take(&mut buf));
                }
            }
            _ => buf.push(ch),
        }
    }
    if !buf.is_empty() {
        out.push(buf);
    }
    out
}

fn parse_tui_wait_ms(line: &str) -> Option<u64> {
    let mut parts = line.split_whitespace();
    let head = parts.next()?;
    if !head.eq_ignore_ascii_case("wait") {
        return None;
    }
    let ms = parts.next()?.parse::<u64>().ok()?;
    if parts.next().is_some() {
        return None;
    }
    Some(ms)
}

fn parse_tui_perf_snapshot(line: &str) -> Option<String> {
    let mut parts = line.split_whitespace();
    let head = parts.next()?;
    if !head.eq_ignore_ascii_case("perf") {
        return None;
    }
    let action = parts.next()?;
    if !action.eq_ignore_ascii_case("snapshot") {
        return None;
    }
    let tag = parts.next().unwrap_or("default");
    if parts.next().is_some() {
        return None;
    }
    Some(tag.to_string())
}

fn parse_tui_script_key(spec: &str) -> Option<KeyEvent> {
    let raw = spec.trim();
    let normalized = raw.to_ascii_lowercase();
    match normalized.as_str() {
        "esc" => Some(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE)),
        "enter" => Some(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE)),
        "tab" => Some(KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE)),
        "shift-tab" | "s-tab" | "backtab" => {
            Some(KeyEvent::new(KeyCode::BackTab, KeyModifiers::SHIFT))
        }
        "up" => Some(KeyEvent::new(KeyCode::Up, KeyModifiers::NONE)),
        "down" => Some(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE)),
        "pgup" | "pageup" => Some(KeyEvent::new(KeyCode::PageUp, KeyModifiers::NONE)),
        "pgdn" | "pagedown" => Some(KeyEvent::new(KeyCode::PageDown, KeyModifiers::NONE)),
        "home" => Some(KeyEvent::new(KeyCode::Home, KeyModifiers::NONE)),
        "end" => Some(KeyEvent::new(KeyCode::End, KeyModifiers::NONE)),
        "f2" => Some(KeyEvent::new(KeyCode::F(2), KeyModifiers::NONE)),
        "f3" => Some(KeyEvent::new(KeyCode::F(3), KeyModifiers::NONE)),
        "f4" => Some(KeyEvent::new(KeyCode::F(4), KeyModifiers::NONE)),
        "f5" => Some(KeyEvent::new(KeyCode::F(5), KeyModifiers::NONE)),
        "ctrl-f2" | "c-f2" => Some(KeyEvent::new(KeyCode::F(2), KeyModifiers::CONTROL)),
        "ctrl-f3" | "c-f3" => Some(KeyEvent::new(KeyCode::F(3), KeyModifiers::CONTROL)),
        "ctrl-f4" | "c-f4" => Some(KeyEvent::new(KeyCode::F(4), KeyModifiers::CONTROL)),
        "ctrl-f5" | "c-f5" => Some(KeyEvent::new(KeyCode::F(5), KeyModifiers::CONTROL)),
        "ctrl-l" | "c-l" => Some(KeyEvent::new(KeyCode::Char('l'), KeyModifiers::CONTROL)),
        "slash" => Some(KeyEvent::new(KeyCode::Char('/'), KeyModifiers::NONE)),
        "space" => Some(KeyEvent::new(KeyCode::Char(' '), KeyModifiers::NONE)),
        "backspace" => Some(KeyEvent::new(KeyCode::Backspace, KeyModifiers::NONE)),
        "delete" | "del" => Some(KeyEvent::new(KeyCode::Delete, KeyModifiers::NONE)),
        _ => {
            let mut chars = raw.chars();
            let ch = chars.next()?;
            if chars.next().is_none() && !ch.is_control() {
                Some(KeyEvent::new(KeyCode::Char(ch), KeyModifiers::NONE))
            } else {
                None
            }
        }
    }
}

fn tui_alias_is_valid(alias: &str) -> bool {
    let len = alias.chars().count();
    if !(2..=32).contains(&len) {
        return false;
    }
    alias
        .chars()
        .all(|ch| ch.is_ascii_alphanumeric() || ch == '-' || ch == '_' || ch == '.')
}

fn tui_verification_code_is_valid(code: &str) -> bool {
    const CROCKFORD: &str = "0123456789ABCDEFGHJKMNPQRSTVWXYZ";
    let upper = code.trim().to_ascii_uppercase();
    if upper.len() != 21 {
        return false;
    }
    for idx in [4usize, 9, 14, 19] {
        if upper.as_bytes().get(idx).copied() != Some(b'-') {
            return false;
        }
    }
    for (idx, ch) in upper.chars().enumerate() {
        if [4usize, 9, 14, 19].contains(&idx) {
            continue;
        }
        if !CROCKFORD.contains(ch) {
            return false;
        }
    }
    true
}

fn tui_passphrase_is_strong(passphrase: &str) -> bool {
    if passphrase.chars().count() < 16 {
        return false;
    }
    let lowered = passphrase.to_ascii_lowercase();
    if lowered.contains("password")
        || lowered.contains("qwerty")
        || lowered.contains("letmein")
        || lowered.contains("123456")
    {
        return false;
    }
    let all_same = passphrase
        .chars()
        .next()
        .map(|first| passphrase.chars().all(|ch| ch == first))
        .unwrap_or(true);
    !all_same
}

fn format_message_transcript_line(
    peer: &str,
    state: &str,
    direction: &str,
    detail: &str,
) -> String {
    let prefix = if direction.eq_ignore_ascii_case("out") {
        "You".to_string()
    } else {
        peer.to_string()
    };
    let message = detail.trim();
    let semantic = message_delivery_semantic_from_state_str(direction, state).unwrap_or(state);
    if message.is_empty() {
        format!("{}:", prefix)
    } else if message.eq_ignore_ascii_case("source=test_harness") {
        format!("{}: (test message) [{}]", prefix, semantic)
    } else {
        format!("{}: {} [{}]", prefix, message, semantic)
    }
}

fn tui_try_vault_init(passphrase: &str) -> Result<(), String> {
    let exe = env::current_exe().map_err(|_| "spawn_failed".to_string())?;
    let mut child = Command::new(exe)
        .args([
            "vault",
            "init",
            "--non-interactive",
            "--passphrase-stdin",
            "--key-source",
            "passphrase",
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|_| "spawn_failed".to_string())?;
    {
        let Some(mut stdin) = child.stdin.take() else {
            return Err("spawn_failed".to_string());
        };
        stdin
            .write_all(passphrase.as_bytes())
            .map_err(|_| "spawn_failed".to_string())?;
    }
    let out = child
        .wait_with_output()
        .map_err(|_| "spawn_failed".to_string())?;
    if out.status.success() {
        return Ok(());
    }
    let mut text = String::from_utf8_lossy(&out.stdout).to_string();
    text.push_str(&String::from_utf8_lossy(&out.stderr));
    for line in text.lines() {
        if let Some(code) = line.split("code=").nth(1) {
            return Err(code
                .split_whitespace()
                .next()
                .unwrap_or("vault_init_failed")
                .to_string());
        }
    }
    Err("vault_init_failed".to_string())
}

fn init_account_defaults_with_passphrase(passphrase: &str) -> Result<(), &'static str> {
    let autolock = TUI_AUTOLOCK_DEFAULT_MINUTES.to_string();
    let poll_interval = TUI_POLL_DEFAULT_INTERVAL_SECONDS.to_string();
    vault::secret_set_with_passphrase(TUI_AUTOLOCK_SECRET_KEY, autolock.as_str(), passphrase)?;
    vault::secret_set_with_passphrase(
        TUI_POLL_MODE_SECRET_KEY,
        TuiPollMode::Adaptive.as_str(),
        passphrase,
    )?;
    vault::secret_set_with_passphrase(
        TUI_POLL_INTERVAL_SECRET_KEY,
        poll_interval.as_str(),
        passphrase,
    )?;
    vault::secret_set_with_passphrase(
        TUI_RECEIPT_MODE_SECRET_KEY,
        ReceiptEmitMode::Off.as_str(),
        passphrase,
    )?;
    vault::secret_set_with_passphrase(
        TUI_RECEIPT_BATCH_WINDOW_MS_SECRET_KEY,
        RECEIPT_BATCH_WINDOW_MS_DEFAULT.to_string().as_str(),
        passphrase,
    )?;
    vault::secret_set_with_passphrase(
        TUI_RECEIPT_JITTER_MS_SECRET_KEY,
        RECEIPT_JITTER_MS_DEFAULT.to_string().as_str(),
        passphrase,
    )?;
    vault::secret_set_with_passphrase(
        TUI_FILE_CONFIRM_MODE_SECRET_KEY,
        FileConfirmEmitMode::CompleteOnly.as_str(),
        passphrase,
    )?;
    let mut seed = [0u8; 16];
    OsRng.fill_bytes(&mut seed);
    vault::secret_set_with_passphrase(
        ACCOUNT_VERIFICATION_SEED_SECRET_KEY,
        hex_encode(&seed).as_str(),
        passphrase,
    )?;
    vault::secret_set_with_passphrase(TUI_RELAY_ENDPOINT_SECRET_KEY, "", passphrase)?;
    vault::secret_set_with_passphrase(TUI_RELAY_TOKEN_SECRET_KEY, "", passphrase)?;
    let inbox_token = generate_route_token();
    vault::secret_set_with_passphrase(
        TUI_RELAY_INBOX_TOKEN_SECRET_KEY,
        inbox_token.as_str(),
        passphrase,
    )?;
    Ok(())
}

fn init_identity_with_passphrase(passphrase: &str) -> Result<(), ErrorCode> {
    let self_label = "self";
    if !channel_label_ok(self_label) {
        return Err(ErrorCode::ParseFailed);
    }
    let (dir, source) = config_dir()?;
    let identities = identities_dir(&dir);
    ensure_dir_secure(&identities, source)?;
    let path = identity_self_path(&dir, self_label);
    if path.exists() {
        enforce_safe_parents(&path, source)?;
        if identity_read_self_public(self_label)?.is_some() {
            return Ok(());
        }
    }
    let (kem_pk, kem_sk) = hs_kem_keypair();
    let (sig_pk, sig_sk) = hs_sig_keypair();
    vault::secret_set_with_passphrase(
        identity_secret_name(self_label).as_str(),
        hex_encode(&kem_sk).as_str(),
        passphrase,
    )
    .map_err(|_| ErrorCode::IoWriteFailed)?;
    vault::secret_set_with_passphrase(
        identity_sig_secret_name(self_label).as_str(),
        hex_encode(&sig_sk).as_str(),
        passphrase,
    )
    .map_err(|_| ErrorCode::IoWriteFailed)?;
    identity_write_public_record(self_label, &kem_pk, &sig_pk)?;
    Ok(())
}

fn initialize_account_after_init(alias: &str, passphrase: &str) -> Result<(), String> {
    if vault::secret_set_with_passphrase("profile_alias", alias, passphrase).is_err() {
        return Err("alias_store_failed".to_string());
    }
    init_account_defaults_with_passphrase(passphrase)
        .map_err(|_| "settings_init_failed".to_string())?;
    init_identity_with_passphrase(passphrase).map_err(|_| "identity_init_failed".to_string())?;
    Ok(())
}

fn wipe_account_local_state_best_effort() {
    let Ok((dir, _)) = config_dir() else {
        return;
    };
    let identities = identities_dir(&dir);
    let sessions = dir.join(QSP_SESSIONS_DIR);
    let qsp_status = dir.join(QSP_STATUS_FILE_NAME);
    let send_state = dir.join(SEND_STATE_NAME);
    let outbox = dir.join(OUTBOX_FILE_NAME);
    let poll_legacy = dir.join("tui_polling.txt");
    let autolock_legacy = dir.join("tui_autolock.txt");
    let _ = fs::remove_dir_all(identities);
    let _ = fs::remove_dir_all(sessions);
    let _ = fs::remove_file(qsp_status);
    let _ = fs::remove_file(send_state);
    let _ = fs::remove_file(outbox);
    let _ = fs::remove_file(poll_legacy);
    let _ = fs::remove_file(autolock_legacy);
    fsync_dir_best_effort(&dir);
}

fn handle_locked_reject(state: &mut TuiState, cmd: &str, reason: &'static str) {
    state.set_command_error("locked: unlock required");
    emit_marker(
        "tui_locked_cmd_reject",
        Some("locked_unlock_required"),
        &[
            ("cmd", cmd),
            ("reason", reason),
            ("error", "locked_unlock_required"),
        ],
    );
    state
        .events
        .push_back("error: locked: unlock required".to_string());
}

fn handle_tui_locked_command(cmd: &TuiParsedCmd, state: &mut TuiState) -> Option<bool> {
    match cmd.cmd.as_str() {
        "exit" | "quit" => {
            emit_marker("tui_cmd", None, &[("cmd", "exit")]);
            Some(true)
        }
        "init" if !state.has_vault() => {
            emit_marker("tui_cmd", None, &[("cmd", "init")]);
            if cmd.args.is_empty() {
                state.start_init_prompt();
                return Some(false);
            }
            emit_marker(
                "tui_init_warning",
                None,
                &[("no_recovery", "true"), ("confirm_prompt", "Y_or_N")],
            );
            if cmd.args.len() < 4 {
                emit_marker(
                    "tui_init_reject",
                    Some("init_args_missing"),
                    &[("ok", "false"), ("required", "alias_pass_confirm_decision")],
                );
                state.start_init_prompt();
                return Some(false);
            }
            let alias = cmd.args[0].as_str();
            let passphrase = cmd.args[1].as_str();
            let confirm = cmd.args[2].as_str();
            let decision = cmd.args[3..].join(" ").to_ascii_uppercase();
            if !tui_alias_is_valid(alias) {
                emit_marker(
                    "tui_init_reject",
                    Some("alias_invalid"),
                    &[("ok", "false"), ("reason", "alias_invalid")],
                );
                state.start_init_prompt();
                return Some(false);
            }
            if !tui_passphrase_is_strong(passphrase) {
                emit_marker(
                    "tui_init_reject",
                    Some("passphrase_weak"),
                    &[("ok", "false"), ("reason", "passphrase_weak")],
                );
                state.start_init_prompt();
                return Some(false);
            }
            if passphrase != confirm {
                emit_marker(
                    "tui_init_reject",
                    Some("passphrase_mismatch"),
                    &[("ok", "false"), ("reason", "passphrase_mismatch")],
                );
                state.start_init_prompt();
                return Some(false);
            }
            if decision != "Y"
                && decision != "YES"
                && decision != "I AGREE"
                && decision != "I UNDERSTAND"
            {
                emit_marker(
                    "tui_init_reject",
                    Some("confirm_required"),
                    &[("ok", "false"), ("reason", "confirm_required")],
                );
                state.start_init_prompt();
                return Some(false);
            }
            match tui_try_vault_init(passphrase) {
                Ok(()) => {}
                Err(code) => {
                    emit_marker(
                        "tui_init_reject",
                        Some(code.as_str()),
                        &[("ok", "false"), ("reason", "vault_init_failed")],
                    );
                    state.start_init_prompt();
                    return Some(false);
                }
            }
            if let Err(code) = initialize_account_after_init(alias, passphrase) {
                emit_marker(
                    "tui_init_reject",
                    Some(code.as_str()),
                    &[("ok", "false"), ("reason", "account_init_failed")],
                );
                state.start_init_prompt();
                return Some(false);
            }
            let _ = vault_security_state_clear_files();
            state.unlock_attempt_limit = None;
            state.failed_unlock_attempts = 0;
            state.mark_vault_present();
            state.set_locked_state(true, "post_init_locked");
            state.locked_flow = LockedFlow::None;
            emit_marker(
                "tui_init",
                None,
                &[("ok", "true"), ("alias", "stored_local_only")],
            );
            Some(false)
        }
        "unlock" if state.has_vault() => {
            emit_marker("tui_cmd", None, &[("cmd", "unlock")]);
            if !state.is_locked() {
                emit_marker(
                    "tui_unlock",
                    None,
                    &[("ok", "true"), ("reason", "already_unlocked")],
                );
                return Some(false);
            }
            if cmd.args.is_empty() {
                state.start_unlock_prompt();
                return Some(false);
            }
            let mut passphrase = cmd.args.first().cloned().unwrap_or_default();
            match state.unlock_with_policy(passphrase.as_str()) {
                UnlockAttemptOutcome::Unlocked => {
                    state.set_locked_state(false, "explicit_command");
                    state.locked_clear_error();
                    emit_marker("tui_unlock", None, &[("ok", "true")]);
                }
                UnlockAttemptOutcome::Wiped => {
                    state.locked_set_error(
                        "vault wiped after failed unlock attempts; run /init to rebuild local state",
                    );
                    state.command_error = Some(format!(
                        "vault: {} (run /init to rebuild local state)",
                        QSC_ERR_VAULT_WIPED_AFTER_FAILED_UNLOCKS
                    ));
                }
                UnlockAttemptOutcome::Rejected => {
                    state
                        .locked_set_error("unlock failed: passphrase did not open the local vault");
                    state.command_error =
                        Some("unlock: passphrase did not open the local vault".to_string());
                    emit_marker(
                        "tui_unlock",
                        Some("vault_locked"),
                        &[("ok", "false"), ("reason", "passphrase_invalid")],
                    );
                }
            }
            passphrase.zeroize();
            Some(false)
        }
        _ => {
            handle_locked_reject(state, cmd.cmd.as_str(), "locked_unlock_required");
            Some(false)
        }
    }
}

fn handle_tui_command(cmd: &TuiParsedCmd, state: &mut TuiState) -> bool {
    let mut command_label = if cmd.args.is_empty() {
        cmd.cmd.clone()
    } else {
        format!("{} {}", cmd.cmd, cmd.args.join(" "))
    };
    if cmd.cmd == "relay" || cmd.cmd == "server" {
        if matches!(
            (
                cmd.args.first().map(|s| s.as_str()),
                cmd.args.get(1).map(|s| s.as_str())
            ),
            (Some("set"), Some("endpoint"))
        ) {
            command_label = "relay set endpoint <redacted>".to_string();
        } else if matches!(
            (
                cmd.args.first().map(|s| s.as_str()),
                cmd.args.get(1).map(|s| s.as_str())
            ),
            (Some("set"), Some("token"))
        ) {
            command_label = "relay set token <redacted>".to_string();
        } else if matches!(
            (
                cmd.args.first().map(|s| s.as_str()),
                cmd.args.get(1).map(|s| s.as_str())
            ),
            (Some("set"), Some("token-file"))
        ) {
            command_label = "relay set token-file <redacted>".to_string();
        } else if matches!(
            (
                cmd.args.first().map(|s| s.as_str()),
                cmd.args.get(1).map(|s| s.as_str())
            ),
            (Some("inbox"), Some("set"))
        ) {
            command_label = "relay inbox set <redacted>".to_string();
        }
    } else if cmd.cmd == "contacts"
        && matches!(
            (
                cmd.args.first().map(|s| s.as_str()),
                cmd.args.get(1).map(|s| s.as_str())
            ),
            (Some("route"), Some("set"))
        )
    {
        command_label = "contacts route set <redacted>".to_string();
    }
    let before_results_len = state.cmd_results.len();
    state.begin_command_tracking(command_label.clone());
    state.mark_input_activity(state.current_now_ms());
    state.clear_command_error();
    state.clear_command_feedback();
    if cmd.cmd == "key" {
        emit_marker("tui_cmd", None, &[("cmd", "key")]);
        let spec = cmd.args.first().map(|s| s.as_str()).unwrap_or("");
        if let Some(key) = parse_tui_script_key(spec) {
            let exit = handle_tui_key(state, key);
            if state.cmd_results.len() == before_results_len {
                state.push_cmd_result(command_label.as_str(), true, "ok");
            }
            if !exit && state.command_error.is_none() {
                state.set_command_feedback("ok: key event");
            }
            state.end_command_tracking();
            return exit;
        }
        state.set_command_error("key: unknown key");
        emit_marker("tui_key_invalid", None, &[("reason", "unknown_key")]);
        state.end_command_tracking();
        return false;
    }
    if state.is_locked() {
        if let Some(exit) = handle_tui_locked_command(cmd, state) {
            if state.cmd_results.len() == before_results_len {
                if let Some(err) = state.command_error.clone() {
                    state.push_cmd_result(command_label.as_str(), false, err);
                } else {
                    state.push_cmd_result(command_label.as_str(), true, "ok");
                }
            }
            state.end_command_tracking();
            return exit;
        }
    }
    let exit = match cmd.cmd.as_str() {
        "help" => {
            emit_marker("tui_cmd", None, &[("cmd", "help")]);
            state.enter_help_mode();
            false
        }
        "exithelp" => {
            emit_marker("tui_cmd", None, &[("cmd", "exithelp")]);
            state.exit_help_mode();
            false
        }
        "focus" => {
            emit_marker("tui_cmd", None, &[("cmd", "focus")]);
            let target = cmd.args.first().map(|s| s.as_str()).unwrap_or("");
            match target {
                "events" => state.enter_focus_mode(TuiMode::FocusEvents),
                "files" => state.enter_focus_mode(TuiMode::FocusFiles),
                "activity" => state.enter_focus_mode(TuiMode::FocusActivity),
                "status" => state.enter_focus_mode(TuiMode::FocusStatus),
                "session" | "keys" => state.enter_focus_mode(TuiMode::FocusSession),
                "contacts" => state.enter_focus_mode(TuiMode::FocusContacts),
                "settings" => state.enter_focus_mode(TuiMode::FocusSettings),
                "lock" => state.enter_focus_mode(TuiMode::FocusLock),
                _ => {
                    state.set_command_error("focus: unknown pane");
                    emit_marker("tui_focus_invalid", None, &[("reason", "unknown_pane")]);
                }
            }
            false
        }
        "inspector" | "ins" => {
            emit_marker("tui_cmd", None, &[("cmd", "inspector")]);
            let target = cmd.args.first().map(|s| s.as_str()).unwrap_or("");
            match target {
                "events" => state.set_inspector(TuiInspectorPane::Events),
                "files" => state.set_inspector(TuiInspectorPane::Files),
                "activity" => state.set_inspector(TuiInspectorPane::Activity),
                "status" => state.set_inspector(TuiInspectorPane::Status),
                "overview" => state.set_inspector(TuiInspectorPane::Status),
                "account" => state.set_inspector(TuiInspectorPane::Account),
                "relay" | "server" => state.set_inspector(TuiInspectorPane::Relay),
                "cmdresults" | "results" => state.set_inspector(TuiInspectorPane::CmdResults),
                "session" | "keys" => state.set_inspector(TuiInspectorPane::Session),
                "contacts" => state.set_inspector(TuiInspectorPane::Contacts),
                "settings" => state.set_inspector(TuiInspectorPane::Settings),
                "lock" => state.set_inspector(TuiInspectorPane::Lock),
                "help" => state.set_inspector(TuiInspectorPane::Help),
                "about" => state.set_inspector(TuiInspectorPane::About),
                "legal" => state.set_inspector(TuiInspectorPane::Legal),
                _ => {
                    state.set_command_error("inspector: unknown pane");
                    emit_marker("tui_inspector_invalid", None, &[("reason", "unknown_pane")]);
                }
            }
            false
        }
        "back" | "unfocus" => {
            emit_marker("tui_cmd", None, &[("cmd", "back")]);
            state.exit_focus_mode();
            false
        }
        "down" => {
            emit_marker("tui_cmd", None, &[("cmd", "down")]);
            if state.is_focus_mode() {
                if state.mode == TuiMode::FocusContacts {
                    state.contacts_move(1);
                } else if state.mode == TuiMode::FocusFiles {
                    state.files_move(1);
                } else {
                    let max_len = state.focus_max_len();
                    state.focus_scroll_move(1, max_len);
                }
            }
            false
        }
        "up" => {
            emit_marker("tui_cmd", None, &[("cmd", "up")]);
            if state.is_focus_mode() {
                if state.mode == TuiMode::FocusContacts {
                    state.contacts_move(-1);
                } else if state.mode == TuiMode::FocusFiles {
                    state.files_move(-1);
                } else {
                    let max_len = state.focus_max_len();
                    state.focus_scroll_move(-1, max_len);
                }
            }
            false
        }
        "pgdn" | "pagedown" => {
            emit_marker("tui_cmd", None, &[("cmd", "pgdn")]);
            if state.is_focus_mode() {
                if state.mode == TuiMode::FocusContacts {
                    state.contacts_move(state.focus_view_rows() as i32);
                } else if state.mode == TuiMode::FocusFiles {
                    state.files_move(state.focus_view_rows() as i32);
                } else {
                    let max_len = state.focus_max_len();
                    state.focus_scroll_move(state.focus_view_rows() as i32, max_len);
                }
            }
            false
        }
        "pgup" | "pageup" => {
            emit_marker("tui_cmd", None, &[("cmd", "pgup")]);
            if state.is_focus_mode() {
                if state.mode == TuiMode::FocusContacts {
                    state.contacts_move(-(state.focus_view_rows() as i32));
                } else if state.mode == TuiMode::FocusFiles {
                    state.files_move(-(state.focus_view_rows() as i32));
                } else {
                    let max_len = state.focus_max_len();
                    state.focus_scroll_move(-(state.focus_view_rows() as i32), max_len);
                }
            }
            false
        }
        "exit" | "quit" => {
            emit_marker("tui_cmd", None, &[("cmd", "exit")]);
            true
        }
        "send" => {
            emit_marker("tui_cmd", None, &[("cmd", "send")]);
            if state.is_locked() {
                emit_marker(
                    "tui_send_blocked",
                    None,
                    &[("reason", "vault_locked"), ("hint", "run_qsc_vault_unlock")],
                );
                state.update_send_lifecycle("blocked");
                return false;
            }
            let selected_peer = state
                .selected_messages_thread()
                .unwrap_or_else(|| state.session.peer_label.to_string());
            if let Err(reason) = state.trust_allows_peer_send_strict(selected_peer.as_str()) {
                emit_marker("tui_send_blocked", Some(reason), &[("reason", reason)]);
                state.update_send_lifecycle("blocked");
                return false;
            }
            tui_send_via_relay(state, selected_peer.as_str());
            false
        }
        "receive" => {
            emit_marker("tui_cmd", None, &[("cmd", "receive")]);
            if state.is_locked() {
                emit_marker(
                    "tui_receive_blocked",
                    None,
                    &[("reason", "vault_locked"), ("hint", "run_qsc_vault_unlock")],
                );
                return false;
            }
            let selected_peer = state.selected_messages_thread();
            let peer = cmd
                .args
                .first()
                .map(|s| s.as_str())
                .or(selected_peer.as_deref())
                .unwrap_or(state.session.peer_label);
            tui_receive_via_relay(state, peer);
            false
        }
        "handshake" => {
            emit_marker("tui_cmd", None, &[("cmd", "handshake")]);
            if state.is_locked() {
                emit_marker(
                    "tui_handshake_blocked",
                    None,
                    &[("reason", "vault_locked"), ("hint", "run_qsc_vault_unlock")],
                );
                return false;
            }
            let sub = cmd.args.first().map(|s| s.as_str()).unwrap_or("status");
            let selected_peer = state.selected_messages_thread();
            let peer = cmd
                .args
                .get(1)
                .map(|s| s.as_str())
                .or(selected_peer.as_deref())
                .unwrap_or(state.session.peer_label);
            let self_label = env::var("QSC_SELF_LABEL").unwrap_or_else(|_| "self".to_string());
            match sub {
                "status" => {
                    handshake_status(Some(peer));
                    state
                        .events
                        .push_back(format!("handshake status peer={}", peer));
                }
                "init" => {
                    if let Some(r) = state.effective_relay_config() {
                        if let Err(code) = resolve_peer_device_target(peer, false) {
                            emit_marker("tui_handshake_blocked", Some(code), &[("reason", code)]);
                            return false;
                        }
                        if let Err(code) = tui_enforce_peer_not_blocked(state, peer) {
                            emit_marker("tui_handshake_blocked", Some(code), &[("reason", code)]);
                            return false;
                        }
                        handshake_init(&self_label, peer, &r.relay);
                        state
                            .events
                            .push_back(format!("handshake init peer={}", peer));
                    } else {
                        emit_marker(
                            "tui_handshake_blocked",
                            None,
                            &[("reason", "explicit_only_no_transport")],
                        );
                    }
                }
                "poll" => {
                    if let Some(r) = state.effective_relay_config() {
                        if let Err(code) = resolve_peer_device_target(peer, false) {
                            emit_marker("tui_handshake_blocked", Some(code), &[("reason", code)]);
                            return false;
                        }
                        if let Err(code) = relay_self_inbox_route_token() {
                            emit_marker("tui_handshake_blocked", Some(code), &[("reason", code)]);
                            return false;
                        }
                        if let Err(code) = tui_enforce_peer_not_blocked(state, peer) {
                            emit_marker("tui_handshake_blocked", Some(code), &[("reason", code)]);
                            return false;
                        }
                        handshake_poll(&self_label, peer, &r.relay, 4);
                        state
                            .events
                            .push_back(format!("handshake poll peer={}", peer));
                    } else {
                        emit_marker(
                            "tui_handshake_blocked",
                            None,
                            &[("reason", "explicit_only_no_transport")],
                        );
                    }
                }
                _ => {
                    emit_marker(
                        "tui_handshake_invalid",
                        None,
                        &[("reason", "unknown_subcmd")],
                    );
                }
            }
            false
        }
        "status" => {
            emit_marker("tui_cmd", None, &[("cmd", "status")]);
            state.refresh_envelope(state.last_payload_len());
            state.refresh_qsp_status();
            state.push_cmd_result("status", true, "system overview refreshed");
            false
        }
        "autolock" => {
            emit_marker("tui_cmd", None, &[("cmd", "autolock")]);
            let sub = cmd.args.first().map(|s| s.as_str()).unwrap_or("show");
            match sub {
                "set" => {
                    let Some(minutes_s) = cmd.args.get(1).map(|s| s.as_str()) else {
                        state.set_command_error("autolock: missing minutes");
                        emit_marker(
                            "tui_autolock_set",
                            Some("autolock_invalid_minutes"),
                            &[("ok", "false"), ("reason", "missing_minutes")],
                        );
                        return false;
                    };
                    let Ok(minutes) = minutes_s.parse::<u64>() else {
                        state.set_command_error("autolock: invalid minutes");
                        emit_marker(
                            "tui_autolock_set",
                            Some("autolock_invalid_minutes"),
                            &[("ok", "false"), ("reason", "invalid_minutes")],
                        );
                        return false;
                    };
                    if let Err(code) = state.set_autolock_minutes(minutes) {
                        state.set_command_error(format!("autolock: {}", code));
                        emit_marker("tui_autolock_set", Some(code), &[("ok", "false")]);
                    } else {
                        state.set_status_last_command_result(format!(
                            "autolock set {} min",
                            minutes
                        ));
                        state.push_cmd_result(
                            "autolock set",
                            true,
                            format!("timeout={} min", minutes),
                        );
                    }
                }
                "show" => {
                    let minutes_s = state.autolock_minutes().to_string();
                    emit_marker(
                        "tui_autolock_show",
                        None,
                        &[("ok", "true"), ("minutes", minutes_s.as_str())],
                    );
                    state.set_status_last_command_result(format!(
                        "autolock {} min",
                        state.autolock_minutes()
                    ));
                    state.push_cmd_result(
                        "autolock show",
                        true,
                        format!("timeout={} min", state.autolock_minutes()),
                    );
                }
                _ => {
                    state.set_command_error("autolock: unknown subcommand");
                    emit_marker(
                        "tui_autolock_set",
                        Some("autolock_invalid_subcmd"),
                        &[("ok", "false"), ("reason", "unknown_subcmd")],
                    );
                }
            }
            false
        }
        "poll" | "polling" => {
            emit_marker("tui_cmd", None, &[("cmd", "poll")]);
            let sub = cmd.args.first().map(|s| s.as_str()).unwrap_or("show");
            match sub {
                "show" => {
                    state.emit_poll_show_marker();
                    state.set_status_last_command_result(format!(
                        "poll {} {}s",
                        state.poll_mode().as_str(),
                        state.poll_interval_seconds()
                    ));
                    state.push_cmd_result(
                        "poll show",
                        true,
                        format!(
                            "mode={} interval={}s",
                            state.poll_mode().as_str(),
                            state.poll_interval_seconds()
                        ),
                    );
                }
                "set" => {
                    let Some(mode) = cmd.args.get(1).map(|s| s.as_str()) else {
                        state.set_command_error("poll: missing mode");
                        emit_marker(
                            "tui_poll_set",
                            Some("poll_invalid_subcmd"),
                            &[("ok", "false"), ("reason", "missing_mode")],
                        );
                        return false;
                    };
                    match mode {
                        "adaptive" => {
                            if let Err(code) = state.set_poll_mode_adaptive() {
                                state.set_command_error(format!("poll: {}", code));
                                emit_marker("tui_poll_set", Some(code), &[("ok", "false")]);
                            } else {
                                state.set_status_last_command_result("poll set adaptive");
                                state.push_cmd_result("poll set adaptive", true, "mode=adaptive");
                            }
                        }
                        "fixed" => {
                            let Some(seconds_s) = cmd.args.get(2).map(|s| s.as_str()) else {
                                state.set_command_error("poll: missing seconds");
                                emit_marker(
                                    "tui_poll_set",
                                    Some("poll_invalid_seconds"),
                                    &[("ok", "false"), ("reason", "missing_seconds")],
                                );
                                return false;
                            };
                            let Ok(seconds) = seconds_s.parse::<u64>() else {
                                state.set_command_error("poll: invalid seconds");
                                emit_marker(
                                    "tui_poll_set",
                                    Some("poll_invalid_seconds"),
                                    &[("ok", "false"), ("reason", "invalid_seconds")],
                                );
                                return false;
                            };
                            let now_ms = state.current_now_ms();
                            if let Err(code) = state.set_poll_mode_fixed(seconds, now_ms) {
                                state.set_command_error(format!("poll: {}", code));
                                emit_marker("tui_poll_set", Some(code), &[("ok", "false")]);
                            } else {
                                state.set_status_last_command_result(format!(
                                    "poll set fixed {}s",
                                    seconds
                                ));
                                state.push_cmd_result(
                                    "poll set fixed",
                                    true,
                                    format!("interval={}s", seconds),
                                );
                            }
                        }
                        _ => {
                            state.set_command_error("poll: unknown mode");
                            emit_marker(
                                "tui_poll_set",
                                Some("poll_invalid_subcmd"),
                                &[("ok", "false"), ("reason", "unknown_mode")],
                            );
                        }
                    }
                }
                _ => {
                    state.set_command_error("poll: unknown subcommand");
                    emit_marker(
                        "tui_poll_set",
                        Some("poll_invalid_subcmd"),
                        &[("ok", "false"), ("reason", "unknown_subcmd")],
                    );
                }
            }
            false
        }
        "relay" | "server" => {
            emit_marker("tui_cmd", None, &[("cmd", "relay")]);
            let sub = cmd.args.first().map(|s| s.as_str()).unwrap_or("show");
            match sub {
                "show" => {
                    state.push_cmd_result("relay show", true, "relay page");
                    state.set_status_last_command_result("relay show");
                    state.route_show_to_system_nav(TuiInspectorPane::Relay);
                }
                "set" => {
                    let Some(key) = cmd.args.get(1).map(|s| s.as_str()) else {
                        state.set_command_error("relay: missing field");
                        return false;
                    };
                    match key {
                        "endpoint" => {
                            let Some(value) = cmd.args.get(2).map(|s| s.as_str()) else {
                                state.set_command_error("relay: missing endpoint");
                                return false;
                            };
                            match state.set_relay_endpoint(value) {
                                Ok(()) => {
                                    let hash = state
                                        .relay_endpoint_hash_cache
                                        .as_deref()
                                        .unwrap_or("none")
                                        .to_string();
                                    state.push_cmd_result(
                                        "relay set endpoint",
                                        true,
                                        "relay endpoint set (redacted)",
                                    );
                                    state.set_status_last_command_result(format!(
                                        "relay endpoint set (hash {})",
                                        hash
                                    ));
                                    state.set_command_feedback("ok: relay endpoint set");
                                }
                                Err(code) => {
                                    emit_marker("tui_relay_policy_reject", Some(code), &[]);
                                    state.set_command_error(format!(
                                        "relay: {}",
                                        relay_user_reason_from_code(code)
                                    ));
                                }
                            }
                        }
                        "token" => {
                            let Some(value) = cmd.args.get(2).map(|s| s.as_str()) else {
                                state.set_command_error("relay: missing token");
                                return false;
                            };
                            match state.set_relay_token(value) {
                                Ok(()) => {
                                    state.push_cmd_result(
                                        "relay set token",
                                        true,
                                        "relay token stored (redacted)",
                                    );
                                    state.set_status_last_command_result("relay token set");
                                    state.set_command_feedback("ok: relay token set");
                                }
                                Err(code) => state.set_command_error(format!(
                                    "relay: {}",
                                    relay_user_reason_from_code(code)
                                )),
                            }
                        }
                        "token-file" => {
                            let Some(value) = cmd.args.get(2).map(|s| s.as_str()) else {
                                state.set_command_error("relay: missing token file path");
                                return false;
                            };
                            match state.set_relay_token_file(value) {
                                Ok(()) => {
                                    state.push_cmd_result(
                                        "relay set token-file",
                                        true,
                                        "relay token file stored (redacted)",
                                    );
                                    state.set_status_last_command_result("relay token file set");
                                    state.set_command_feedback("ok: relay token file set");
                                }
                                Err(code) => state.set_command_error(format!(
                                    "relay: {}",
                                    relay_user_reason_from_code(code)
                                )),
                            }
                        }
                        _ => state.set_command_error("relay: unknown field"),
                    }
                }
                "inbox" => {
                    let Some(action) = cmd.args.get(1).map(|s| s.as_str()) else {
                        state.set_command_error("relay: missing inbox subcommand");
                        return false;
                    };
                    match action {
                        "set" => {
                            let Some(value) = cmd.args.get(2).map(|s| s.as_str()) else {
                                state.set_command_error("relay: missing inbox token");
                                return false;
                            };
                            match state.set_relay_inbox_token(value) {
                                Ok(()) => {
                                    state.push_cmd_result(
                                        "relay inbox set",
                                        true,
                                        "relay inbox token stored (redacted)",
                                    );
                                    state.set_status_last_command_result("relay inbox token set");
                                    state.set_command_feedback("ok: relay inbox token set");
                                }
                                Err(code) => state.set_command_error(format!(
                                    "relay: {}",
                                    relay_user_reason_from_code(code)
                                )),
                            }
                        }
                        "clear" => {
                            if let Err(code) = state.clear_relay_inbox_token() {
                                state.set_command_error(format!(
                                    "relay: {}",
                                    relay_user_reason_from_code(code)
                                ));
                                return false;
                            }
                            state.push_cmd_result(
                                "relay inbox clear",
                                true,
                                "relay inbox token cleared",
                            );
                            state.set_status_last_command_result("relay inbox token cleared");
                            state.set_command_feedback("ok: relay inbox token cleared");
                        }
                        _ => state.set_command_error("relay: unknown field"),
                    }
                }
                "clear" => {
                    let clear_token_only =
                        matches!(cmd.args.get(1).map(|s| s.as_str()), Some("token"));
                    let clear_token_file_only =
                        matches!(cmd.args.get(1).map(|s| s.as_str()), Some("token-file"));
                    let clear_inbox_only =
                        matches!(cmd.args.get(1).map(|s| s.as_str()), Some("inbox"));
                    if clear_token_only {
                        if let Err(code) =
                            state.persist_account_secret(TUI_RELAY_TOKEN_SECRET_KEY, "")
                        {
                            state.set_command_error(format!(
                                "relay: {}",
                                relay_user_reason_from_code(code)
                            ));
                            return false;
                        }
                        state.relay_token_set_cache = false;
                        state.push_cmd_result("relay clear token", true, "relay token cleared");
                        state.set_status_last_command_result("relay token cleared");
                        state.set_command_feedback("ok: relay token cleared");
                    } else if clear_token_file_only {
                        if let Err(code) =
                            state.persist_account_secret(TUI_RELAY_TOKEN_FILE_SECRET_KEY, "")
                        {
                            state.set_command_error(format!(
                                "relay: {}",
                                relay_user_reason_from_code(code)
                            ));
                            return false;
                        }
                        state.relay_token_file_cache = None;
                        state.relay_token_file_hash_cache = None;
                        state.push_cmd_result(
                            "relay clear token-file",
                            true,
                            "relay token file cleared",
                        );
                        state.set_status_last_command_result("relay token file cleared");
                        state.set_command_feedback("ok: relay token file cleared");
                    } else if clear_inbox_only {
                        if let Err(code) = state.clear_relay_inbox_token() {
                            state.set_command_error(format!(
                                "relay: {}",
                                relay_user_reason_from_code(code)
                            ));
                            return false;
                        }
                        state.push_cmd_result("relay clear inbox", true, "relay inbox cleared");
                        state.set_status_last_command_result("relay inbox cleared");
                        state.set_command_feedback("ok: relay inbox cleared");
                    } else if let Err(code) = state.clear_relay_config() {
                        state.set_command_error(format!(
                            "relay: {}",
                            relay_user_reason_from_code(code)
                        ));
                    } else {
                        state.push_cmd_result("relay clear", true, "relay config cleared");
                        state.set_status_last_command_result("relay config cleared");
                        state.set_command_feedback("ok: relay config cleared");
                    }
                }
                "test" => {
                    if state.relay_endpoint_cache.is_none() {
                        emit_tui_relay_test_event("err", "relay_endpoint_missing");
                        state.push_cmd_result("relay test", false, "endpoint not configured");
                        state.set_status_last_command_result("relay test err (endpoint missing)");
                        state.set_command_error("relay: endpoint not configured");
                        return false;
                    }
                    if state.relay_test_task.is_some() {
                        emit_tui_relay_test_event("err", "relay_test_already_running");
                        state.push_cmd_result("relay test", false, "test already running");
                        state.set_status_last_command_result("relay test err (already running)");
                        state.set_command_error("relay: test already running");
                        return false;
                    }
                    let endpoint = state.relay_endpoint_cache.clone().unwrap_or_default();
                    if let Err(code) = normalize_relay_endpoint(endpoint.as_str()) {
                        emit_marker("tui_relay_policy_reject", Some(code), &[]);
                        let reason = relay_user_reason_from_code(code);
                        emit_tui_relay_test_event("err", code);
                        state.push_cmd_result("relay test", false, reason);
                        state
                            .set_status_last_command_result(format!("relay test err ({})", reason));
                        state.set_command_error(format!("relay: {}", reason));
                        return false;
                    }
                    let token = if state.relay_token_set_cache {
                        state.read_account_secret(TUI_RELAY_TOKEN_SECRET_KEY)
                    } else {
                        None
                    };
                    let token_file = state.relay_token_file_cache.clone();
                    let (tx, rx) = mpsc::channel::<RelayTestOutcome>();
                    std::thread::spawn(move || {
                        let outcome =
                            run_relay_test_probe(endpoint.as_str(), token, token_file.as_deref());
                        let _ = tx.send(outcome);
                    });
                    state.relay_last_test_result = "pending".to_string();
                    state.relay_test_task = Some(rx);
                    emit_tui_relay_test_event("started", "pending");
                    state.push_cmd_result("relay test", true, "started");
                    state.set_status_last_command_result("relay test started");
                    state.set_command_feedback("ok: relay test started");
                }
                _ => state.set_command_error("relay: unknown subcommand"),
            }
            false
        }
        "vault" => {
            emit_marker("tui_cmd", None, &[("cmd", "vault")]);
            let sub = cmd.args.first().map(|s| s.as_str()).unwrap_or("");
            match sub {
                "where" => match config_dir() {
                    Ok((cfg, _)) => {
                        let path = cfg.join("vault.qsv");
                        let path_s = path.display().to_string();
                        emit_marker(
                            "tui_vault_where",
                            None,
                            &[("ok", "true"), ("path", path_s.as_str())],
                        );
                        state.set_status_last_command_result(format!("vault path {}", path_s));
                        state.push_cmd_result("vault where", true, format!("path={}", path_s));
                    }
                    Err(code) => {
                        state.set_command_error(format!("vault: {}", code.as_str()));
                        emit_marker("tui_vault_where", Some(code.as_str()), &[("ok", "false")]);
                    }
                },
                "attempt_limit" => {
                    let action = cmd.args.get(1).map(|s| s.as_str()).unwrap_or("show");
                    match action {
                        "set" => {
                            let Some(limit_s) = cmd.args.get(2).map(|s| s.as_str()) else {
                                state.set_command_error("vault: attempt_limit missing value");
                                return false;
                            };
                            let Ok(limit) = limit_s.parse::<u32>() else {
                                state.set_command_error("vault: attempt_limit invalid value");
                                return false;
                            };
                            match state.set_unlock_attempt_limit(Some(limit)) {
                                Ok(()) => {
                                    state.set_status_last_command_result(format!(
                                        "vault attempt limit set {}",
                                        limit
                                    ));
                                    state.push_cmd_result(
                                        "vault attempt_limit set",
                                        true,
                                        format!(
                                            "limit={} (warning: enabling this can permanently destroy your vault after {} failed unlocks)",
                                            limit, limit
                                        ),
                                    );
                                    state.set_command_feedback(format!(
                                        "ok: vault attempt limit set {}",
                                        limit
                                    ));
                                }
                                Err(code) => {
                                    state.set_command_error(format!("vault: {}", code));
                                }
                            }
                        }
                        "clear" => match state.set_unlock_attempt_limit(None) {
                            Ok(()) => {
                                state.set_status_last_command_result(
                                    "vault attempt limit cleared".to_string(),
                                );
                                state.push_cmd_result(
                                    "vault attempt_limit clear",
                                    true,
                                    "attempt limit disabled",
                                );
                                state.set_command_feedback("ok: vault attempt limit cleared");
                            }
                            Err(code) => state.set_command_error(format!("vault: {}", code)),
                        },
                        "show" => {
                            let limit = state
                                .unlock_attempt_limit
                                .map(|v| v.to_string())
                                .unwrap_or_else(|| "off".to_string());
                            let failures = state.failed_unlock_attempts.to_string();
                            emit_marker(
                                "tui_vault_attempt_limit_show",
                                None,
                                &[
                                    ("ok", "true"),
                                    ("limit", limit.as_str()),
                                    ("failed_unlocks", failures.as_str()),
                                ],
                            );
                            state.set_status_last_command_result(format!(
                                "vault attempt limit {} failures {}",
                                limit, failures
                            ));
                            state.push_cmd_result(
                                "vault attempt_limit show",
                                true,
                                format!("limit={} failed_unlocks={}", limit, failures),
                            );
                        }
                        _ => state.set_command_error("vault: attempt_limit unknown subcommand"),
                    }
                }
                _ => {
                    state.set_command_error("vault: unknown subcommand");
                    emit_marker(
                        "tui_vault_where",
                        Some("vault_invalid_subcmd"),
                        &[("ok", "false"), ("reason", "unknown_subcmd")],
                    );
                }
            }
            false
        }
        "device" => {
            emit_marker("tui_cmd", None, &[("cmd", "device")]);
            let sub = cmd.args.first().map(|s| s.as_str()).unwrap_or("");
            match sub {
                "show" => {
                    let mode = "single-device";
                    let device_id = "local-vault";
                    emit_marker(
                        "tui_device_show",
                        None,
                        &[("ok", "true"), ("mode", mode), ("id", device_id)],
                    );
                    state.set_status_last_command_result(format!("device {} {}", mode, device_id));
                    state.push_cmd_result(
                        "device show",
                        true,
                        format!("mode={} id={}", mode, device_id),
                    );
                }
                _ => {
                    state.set_command_error("device: unknown subcommand");
                    emit_marker(
                        "tui_device_show",
                        Some("device_invalid_subcmd"),
                        &[("ok", "false"), ("reason", "unknown_subcmd")],
                    );
                }
            }
            false
        }
        "account" => {
            emit_marker("tui_cmd", None, &[("cmd", "account")]);
            let sub = cmd.args.first().map(|s| s.as_str()).unwrap_or("show");
            match sub {
                "show" => {
                    state.set_inspector(TuiInspectorPane::Account);
                    state.push_cmd_result("account show", true, "account page");
                    state.set_status_last_command_result("account page");
                }
                "destroy" => {
                    if state.is_locked() {
                        state.set_command_error("locked: unlock required");
                        return false;
                    }
                    if !state.has_vault() {
                        state.set_command_error("account: no vault found");
                        return false;
                    }
                    state.start_account_destroy_prompt();
                    state.push_cmd_result("account destroy", true, "confirmation required");
                    state.set_status_last_command_result("account destroy started");
                }
                _ => {
                    state.set_command_error("account: unknown subcommand");
                    emit_marker("tui_account_invalid", None, &[("reason", "unknown_subcmd")]);
                }
            }
            false
        }
        "lock" => {
            emit_marker("tui_cmd", None, &[("cmd", "lock")]);
            state.set_locked_state(true, "explicit_command");
            false
        }
        "unlock" => {
            emit_marker("tui_cmd", None, &[("cmd", "unlock")]);
            if !state.is_locked() {
                emit_marker(
                    "tui_unlock",
                    None,
                    &[("ok", "true"), ("reason", "already_unlocked")],
                );
                return false;
            }
            let unlocked = if vault::unlock_if_mock_provider() || vault::has_process_passphrase() {
                state.open_vault_session(None).is_ok()
            } else {
                false
            };
            if unlocked {
                state.set_locked_state(false, "explicit_command");
                emit_marker("tui_unlock", None, &[("ok", "true")]);
            } else {
                emit_marker(
                    "tui_unlock",
                    Some("vault_locked"),
                    &[("ok", "false"), ("reason", "passphrase_required")],
                );
            }
            false
        }
        "contacts" => {
            emit_marker("tui_cmd", None, &[("cmd", "contacts")]);
            let sub = cmd.args.first().map(|s| s.as_str()).unwrap_or("list");
            match sub {
                "list" => {
                    state.refresh_contacts();
                    let count_s = state.contacts.len().to_string();
                    emit_marker("tui_contacts_list", None, &[("count", count_s.as_str())]);
                }
                "device" => {
                    let Some(action) = cmd.args.get(1).map(|s| s.as_str()) else {
                        state.set_command_error("contacts device: missing action");
                        emit_marker(
                            "tui_contacts_invalid",
                            None,
                            &[("reason", "missing_device_action")],
                        );
                        return false;
                    };
                    match action {
                        "list" => {
                            let Some(label) = cmd.args.get(2).map(|s| s.as_str()) else {
                                state.set_command_error("contacts device list: missing alias");
                                emit_marker(
                                    "tui_contacts_invalid",
                                    None,
                                    &[("reason", "missing_label")],
                                );
                                return false;
                            };
                            let Some(rec) = state.contacts_records.get(label) else {
                                state.set_command_error("contacts device list: unknown alias");
                                emit_marker(
                                    "tui_contacts_device_list",
                                    Some("peer_unknown"),
                                    &[("label", label), ("ok", "false")],
                                );
                                return false;
                            };
                            let mut rec = rec.clone();
                            normalize_contact_record(label, &mut rec);
                            let count_s = rec.devices.len().to_string();
                            emit_marker(
                                "tui_contacts_device_list",
                                None,
                                &[("label", label), ("count", count_s.as_str())],
                            );
                            for dev in rec.devices.iter() {
                                emit_marker(
                                    "tui_contacts_device",
                                    None,
                                    &[
                                        ("label", label),
                                        ("device", dev.device_id.as_str()),
                                        ("state", canonical_device_state(dev.state.as_str())),
                                    ],
                                );
                            }
                        }
                        "status" => {
                            let Some(label) = cmd.args.get(2).map(|s| s.as_str()) else {
                                state.set_command_error("contacts device status: missing alias");
                                emit_marker(
                                    "tui_contacts_invalid",
                                    None,
                                    &[("reason", "missing_label")],
                                );
                                return false;
                            };
                            let Some(rec) = state.contacts_records.get(label) else {
                                state.set_command_error("contacts device status: unknown alias");
                                emit_marker(
                                    "tui_contacts_device_status",
                                    Some("peer_unknown"),
                                    &[("label", label), ("ok", "false")],
                                );
                                return false;
                            };
                            let mut rec = rec.clone();
                            normalize_contact_record(label, &mut rec);
                            if let Some(device_id) = cmd.args.get(3).map(|s| s.as_str()) {
                                let Some(idx) = contact_device_find_index(&rec, device_id) else {
                                    state.set_command_error(
                                        "contacts device status: unknown device id",
                                    );
                                    emit_marker(
                                        "tui_contacts_device_status",
                                        Some("device_unknown"),
                                        &[("label", label), ("device", device_id), ("ok", "false")],
                                    );
                                    return false;
                                };
                                let dev = &rec.devices[idx];
                                emit_marker(
                                    "tui_contacts_device_status",
                                    None,
                                    &[
                                        ("label", label),
                                        ("device", device_id),
                                        ("state", canonical_device_state(dev.state.as_str())),
                                    ],
                                );
                            } else {
                                let count_s = rec.devices.len().to_string();
                                emit_marker(
                                    "tui_contacts_device_status",
                                    None,
                                    &[("label", label), ("count", count_s.as_str())],
                                );
                            }
                        }
                        "add" => {
                            let Some(label) = cmd.args.get(2).map(|s| s.as_str()) else {
                                state.set_command_error("contacts device add: missing alias");
                                emit_marker(
                                    "tui_contacts_invalid",
                                    None,
                                    &[("reason", "missing_label")],
                                );
                                return false;
                            };
                            let Some(code) = cmd.args.get(3).map(|s| s.as_str()) else {
                                state.set_command_error(
                                    "contacts device add: missing verification code",
                                );
                                emit_marker(
                                    "tui_contacts_invalid",
                                    None,
                                    &[("reason", "missing_fp")],
                                );
                                return false;
                            };
                            if !tui_verification_code_is_valid(code) {
                                state.set_command_error(
                                    "contacts device add: invalid verification code format",
                                );
                                emit_marker(
                                    "tui_contacts_invalid",
                                    None,
                                    &[("reason", "invalid_code")],
                                );
                                return false;
                            }
                            let route_token = match cmd.args.get(4).map(|s| s.as_str()) {
                                Some(raw) => match normalize_route_token(raw) {
                                    Ok(token) => Some(token),
                                    Err(code) => {
                                        state.set_command_error(format!(
                                            "contacts device add: {}",
                                            code
                                        ));
                                        emit_marker("tui_contacts_invalid", Some(code), &[]);
                                        return false;
                                    }
                                },
                                None => None,
                            };
                            let Some(rec) = state.contacts_records.get_mut(label) else {
                                state.set_command_error("contacts device add: unknown alias");
                                emit_marker(
                                    "tui_contacts_device_add",
                                    Some("peer_unknown"),
                                    &[("label", label), ("ok", "false")],
                                );
                                return false;
                            };
                            normalize_contact_record(label, rec);
                            let device_id = device_id_short(label, None, code);
                            if contact_device_find_index(rec, device_id.as_str()).is_some() {
                                state.set_command_error(
                                    "contacts device add: device already exists",
                                );
                                emit_marker(
                                    "tui_contacts_device_add",
                                    Some("device_exists"),
                                    &[
                                        ("label", label),
                                        ("device", device_id.as_str()),
                                        ("ok", "false"),
                                    ],
                                );
                                return false;
                            }
                            rec.devices.push(ContactDeviceRecord {
                                device_id: device_id.clone(),
                                fp: code.to_ascii_uppercase(),
                                sig_fp: None,
                                state: "UNVERIFIED".to_string(),
                                route_token,
                                seen_at: None,
                                label: None,
                            });
                            normalize_contact_record(label, rec);
                            if state.persist_contacts_cache().is_err() {
                                state.set_command_error("contacts device add: store unavailable");
                                emit_marker(
                                    "tui_contacts_device_add",
                                    Some("contacts_store_unavailable"),
                                    &[("label", label), ("ok", "false")],
                                );
                                return false;
                            }
                            emit_marker(
                                "tui_contacts_device_add",
                                None,
                                &[
                                    ("label", label),
                                    ("device", device_id.as_str()),
                                    ("state", "UNVERIFIED"),
                                    ("ok", "true"),
                                ],
                            );
                            state.refresh_contacts();
                        }
                        "verify" => {
                            let Some(label) = cmd.args.get(2).map(|s| s.as_str()) else {
                                state.set_command_error("contacts device verify: missing alias");
                                emit_marker(
                                    "tui_contacts_invalid",
                                    None,
                                    &[("reason", "missing_label")],
                                );
                                return false;
                            };
                            let Some(device_id) = cmd.args.get(3).map(|s| s.as_str()) else {
                                state
                                    .set_command_error("contacts device verify: missing device id");
                                emit_marker(
                                    "tui_contacts_invalid",
                                    None,
                                    &[("reason", "missing_device")],
                                );
                                return false;
                            };
                            let Some(code) = cmd.args.get(4).map(|s| s.as_str()) else {
                                state.set_command_error(
                                    "contacts device verify: missing verification code",
                                );
                                emit_marker(
                                    "tui_contacts_invalid",
                                    None,
                                    &[("reason", "missing_fp")],
                                );
                                return false;
                            };
                            if !tui_verification_code_is_valid(code) {
                                state.set_command_error(
                                    "contacts device verify: invalid verification code format",
                                );
                                emit_marker(
                                    "tui_contacts_invalid",
                                    None,
                                    &[("reason", "invalid_code")],
                                );
                                return false;
                            }
                            let Some(rec) = state.contacts_records.get_mut(label) else {
                                state.set_command_error("contacts device verify: unknown alias");
                                emit_marker(
                                    "tui_contacts_device_verify",
                                    Some("peer_unknown"),
                                    &[("label", label), ("ok", "false")],
                                );
                                return false;
                            };
                            normalize_contact_record(label, rec);
                            let Some(idx) = contact_device_find_index(rec, device_id) else {
                                state
                                    .set_command_error("contacts device verify: unknown device id");
                                emit_marker(
                                    "tui_contacts_device_verify",
                                    Some("device_unknown"),
                                    &[("label", label), ("device", device_id), ("ok", "false")],
                                );
                                return false;
                            };
                            if rec.devices[idx].fp.eq_ignore_ascii_case(code) {
                                let mode = state.trust_onboarding_mode;
                                rec.devices[idx].state = "VERIFIED".to_string();
                                rec.status = "VERIFIED".to_string();
                                if mode == TrustOnboardingMode::Balanced {
                                    rec.devices[idx].state = "TRUSTED".to_string();
                                    rec.status = "PINNED".to_string();
                                }
                                if state.persist_contacts_cache().is_err() {
                                    state.set_command_error(
                                        "contacts device verify: store unavailable",
                                    );
                                    emit_marker(
                                        "tui_contacts_device_verify",
                                        Some("contacts_store_unavailable"),
                                        &[("label", label), ("ok", "false")],
                                    );
                                    return false;
                                }
                                emit_marker(
                                    "tui_contacts_device_verify",
                                    None,
                                    &[
                                        ("label", label),
                                        ("device", device_id),
                                        (
                                            "state",
                                            if mode == TrustOnboardingMode::Balanced {
                                                "TRUSTED"
                                            } else {
                                                "VERIFIED"
                                            },
                                        ),
                                        ("ok", "true"),
                                    ],
                                );
                                emit_tui_contact_flow(
                                    "verify",
                                    if mode == TrustOnboardingMode::Balanced {
                                        "TRUSTED"
                                    } else {
                                        "VERIFIED"
                                    },
                                    label,
                                    Some(device_id),
                                    mode,
                                );
                                if mode == TrustOnboardingMode::Balanced {
                                    emit_tui_trust_promotion(
                                        "trusted",
                                        "verified_match",
                                        label,
                                        Some(device_id),
                                        mode,
                                    );
                                    state.set_command_feedback(
                                        "ok: verification matched and device auto-trusted (balanced mode)",
                                    );
                                } else {
                                    emit_tui_trust_promotion(
                                        "verified_only",
                                        "strict_mode",
                                        label,
                                        Some(device_id),
                                        mode,
                                    );
                                    state.set_command_feedback(
                                        "ok: verification code matched identity (strict mode requires trust)",
                                    );
                                }
                            } else {
                                rec.devices[idx].state = "CHANGED".to_string();
                                rec.status = "CHANGED".to_string();
                                let _ = state.persist_contacts_cache();
                                state.set_command_error(
                                    "contacts device verify: verification code mismatch",
                                );
                                emit_marker(
                                    "tui_contacts_device_verify",
                                    Some("verification_mismatch"),
                                    &[
                                        ("label", label),
                                        ("device", device_id),
                                        ("state", "CHANGED"),
                                        ("ok", "false"),
                                    ],
                                );
                                emit_tui_contact_flow(
                                    "verify",
                                    "CHANGED",
                                    label,
                                    Some(device_id),
                                    state.trust_onboarding_mode,
                                );
                                return false;
                            }
                            state.refresh_contacts();
                        }
                        "trust" => {
                            let Some(label) = cmd.args.get(2).map(|s| s.as_str()) else {
                                state.set_command_error("contacts device trust: missing alias");
                                emit_marker(
                                    "tui_contacts_invalid",
                                    None,
                                    &[("reason", "missing_label")],
                                );
                                return false;
                            };
                            let Some(device_id) = cmd.args.get(3).map(|s| s.as_str()) else {
                                state.set_command_error("contacts device trust: missing device id");
                                emit_marker(
                                    "tui_contacts_invalid",
                                    None,
                                    &[("reason", "missing_device")],
                                );
                                return false;
                            };
                            let confirmed = cmd
                                .args
                                .get(4)
                                .map(|s| s.eq_ignore_ascii_case("confirm"))
                                .unwrap_or(false);
                            if !confirmed {
                                state.set_command_error(
                                    "contacts device trust: confirmation required",
                                );
                                emit_marker(
                                    "tui_contacts_device_trust",
                                    Some("confirm_required"),
                                    &[("label", label), ("device", device_id), ("ok", "false")],
                                );
                                return false;
                            }
                            let Some(rec) = state.contacts_records.get_mut(label) else {
                                state.set_command_error("contacts device trust: unknown alias");
                                emit_marker(
                                    "tui_contacts_device_trust",
                                    Some("peer_unknown"),
                                    &[("label", label), ("ok", "false")],
                                );
                                return false;
                            };
                            normalize_contact_record(label, rec);
                            let Some(idx) = contact_device_find_index(rec, device_id) else {
                                state.set_command_error("contacts device trust: unknown device id");
                                emit_marker(
                                    "tui_contacts_device_trust",
                                    Some("device_unknown"),
                                    &[("label", label), ("device", device_id), ("ok", "false")],
                                );
                                return false;
                            };
                            rec.devices[idx].state = "TRUSTED".to_string();
                            rec.status = "PINNED".to_string();
                            if state.persist_contacts_cache().is_err() {
                                state.set_command_error("contacts device trust: store unavailable");
                                emit_marker(
                                    "tui_contacts_device_trust",
                                    Some("contacts_store_unavailable"),
                                    &[("label", label), ("ok", "false")],
                                );
                                return false;
                            }
                            emit_marker(
                                "tui_contacts_device_trust",
                                None,
                                &[
                                    ("label", label),
                                    ("device", device_id),
                                    ("state", "TRUSTED"),
                                    ("ok", "true"),
                                ],
                            );
                            emit_tui_contact_flow(
                                "trust",
                                "TRUSTED",
                                label,
                                Some(device_id),
                                state.trust_onboarding_mode,
                            );
                            emit_tui_trust_promotion(
                                "trusted",
                                "explicit_operator_action",
                                label,
                                Some(device_id),
                                state.trust_onboarding_mode,
                            );
                            state.set_command_feedback("ok: device trusted (allowed to send)");
                            state.refresh_contacts();
                        }
                        "revoke" => {
                            let Some(label) = cmd.args.get(2).map(|s| s.as_str()) else {
                                state.set_command_error("contacts device revoke: missing alias");
                                emit_marker(
                                    "tui_contacts_invalid",
                                    None,
                                    &[("reason", "missing_label")],
                                );
                                return false;
                            };
                            let Some(device_id) = cmd.args.get(3).map(|s| s.as_str()) else {
                                state
                                    .set_command_error("contacts device revoke: missing device id");
                                emit_marker(
                                    "tui_contacts_invalid",
                                    None,
                                    &[("reason", "missing_device")],
                                );
                                return false;
                            };
                            let confirmed = cmd
                                .args
                                .get(4)
                                .map(|s| s.eq_ignore_ascii_case("confirm"))
                                .unwrap_or(false);
                            if !confirmed {
                                state.set_command_error(
                                    "contacts device revoke: confirmation required",
                                );
                                emit_marker(
                                    "tui_contacts_device_revoke",
                                    Some("confirm_required"),
                                    &[("label", label), ("device", device_id), ("ok", "false")],
                                );
                                return false;
                            }
                            let Some(rec) = state.contacts_records.get_mut(label) else {
                                state.set_command_error("contacts device revoke: unknown alias");
                                emit_marker(
                                    "tui_contacts_device_revoke",
                                    Some("peer_unknown"),
                                    &[("label", label), ("ok", "false")],
                                );
                                return false;
                            };
                            normalize_contact_record(label, rec);
                            let Some(idx) = contact_device_find_index(rec, device_id) else {
                                state
                                    .set_command_error("contacts device revoke: unknown device id");
                                emit_marker(
                                    "tui_contacts_device_revoke",
                                    Some("device_unknown"),
                                    &[("label", label), ("device", device_id), ("ok", "false")],
                                );
                                return false;
                            };
                            rec.devices[idx].state = "REVOKED".to_string();
                            if state.persist_contacts_cache().is_err() {
                                state
                                    .set_command_error("contacts device revoke: store unavailable");
                                emit_marker(
                                    "tui_contacts_device_revoke",
                                    Some("contacts_store_unavailable"),
                                    &[("label", label), ("ok", "false")],
                                );
                                return false;
                            }
                            emit_marker(
                                "tui_contacts_device_revoke",
                                None,
                                &[
                                    ("label", label),
                                    ("device", device_id),
                                    ("state", "REVOKED"),
                                    ("ok", "true"),
                                ],
                            );
                            state.refresh_contacts();
                        }
                        "primary" => {
                            let Some(primary_action) = cmd.args.get(2).map(|s| s.as_str()) else {
                                state.set_command_error("contacts device primary: missing action");
                                emit_marker(
                                    "tui_contacts_invalid",
                                    None,
                                    &[("reason", "missing_primary_action")],
                                );
                                return false;
                            };
                            match primary_action {
                                "set" => {
                                    let Some(label) = cmd.args.get(3).map(|s| s.as_str()) else {
                                        state.set_command_error(
                                            "contacts device primary set: missing alias",
                                        );
                                        emit_marker(
                                            "tui_contacts_invalid",
                                            None,
                                            &[("reason", "missing_label")],
                                        );
                                        return false;
                                    };
                                    let Some(device_id) = cmd.args.get(4).map(|s| s.as_str())
                                    else {
                                        state.set_command_error(
                                            "contacts device primary set: missing device id",
                                        );
                                        emit_marker(
                                            "tui_contacts_invalid",
                                            None,
                                            &[("reason", "missing_device")],
                                        );
                                        return false;
                                    };
                                    let confirmed = cmd
                                        .args
                                        .get(5)
                                        .map(|s| s.eq_ignore_ascii_case("confirm"))
                                        .unwrap_or(false);
                                    if !confirmed {
                                        state.set_command_error(
                                            "contacts device primary set: confirmation required",
                                        );
                                        emit_marker(
                                            "tui_contacts_device_primary_set",
                                            Some("confirm_required"),
                                            &[
                                                ("label", label),
                                                ("device", device_id),
                                                ("ok", "false"),
                                            ],
                                        );
                                        return false;
                                    }
                                    let Some(rec) = state.contacts_records.get_mut(label) else {
                                        state.set_command_error(
                                            "contacts device primary set: unknown alias",
                                        );
                                        emit_marker(
                                            "tui_contacts_device_primary_set",
                                            Some("peer_unknown"),
                                            &[("label", label), ("ok", "false")],
                                        );
                                        return false;
                                    };
                                    normalize_contact_record(label, rec);
                                    let Some(_) = contact_device_find_index(rec, device_id) else {
                                        state.set_command_error(
                                            "contacts device primary set: unknown device id",
                                        );
                                        emit_marker(
                                            "tui_contacts_device_primary_set",
                                            Some("device_unknown"),
                                            &[
                                                ("label", label),
                                                ("device", device_id),
                                                ("ok", "false"),
                                            ],
                                        );
                                        return false;
                                    };
                                    rec.primary_device_id = Some(device_id.to_string());
                                    normalize_contact_record(label, rec);
                                    if state.persist_contacts_cache().is_err() {
                                        state.set_command_error(
                                            "contacts device primary set: store unavailable",
                                        );
                                        emit_marker(
                                            "tui_contacts_device_primary_set",
                                            Some("contacts_store_unavailable"),
                                            &[("label", label), ("ok", "false")],
                                        );
                                        return false;
                                    }
                                    emit_marker(
                                        "tui_contacts_device_primary_set",
                                        None,
                                        &[
                                            ("label", label),
                                            ("device", device_id),
                                            ("selected", "explicit"),
                                            ("policy", "primary_only"),
                                            ("ok", "true"),
                                        ],
                                    );
                                    state.refresh_contacts();
                                }
                                "show" => {
                                    let Some(label) = cmd.args.get(3).map(|s| s.as_str()) else {
                                        state.set_command_error(
                                            "contacts device primary show: missing alias",
                                        );
                                        emit_marker(
                                            "tui_contacts_invalid",
                                            None,
                                            &[("reason", "missing_label")],
                                        );
                                        return false;
                                    };
                                    let Some(rec) = state.contacts_records.get(label) else {
                                        state.set_command_error(
                                            "contacts device primary show: unknown alias",
                                        );
                                        emit_marker(
                                            "tui_contacts_device_primary_show",
                                            Some("peer_unknown"),
                                            &[("label", label), ("ok", "false")],
                                        );
                                        return false;
                                    };
                                    let mut rec = rec.clone();
                                    let implicit = rec.primary_device_id.is_none();
                                    normalize_contact_record(label, &mut rec);
                                    let primary = primary_device(&rec)
                                        .map(|d| d.device_id.as_str())
                                        .unwrap_or("none")
                                        .to_string();
                                    emit_marker(
                                        "tui_contacts_device_primary_show",
                                        None,
                                        &[
                                            ("label", label),
                                            ("device", primary.as_str()),
                                            (
                                                "selected",
                                                if implicit { "implicit" } else { "explicit" },
                                            ),
                                            ("policy", "primary_only"),
                                            ("ok", "true"),
                                        ],
                                    );
                                }
                                _ => {
                                    state.set_command_error(
                                        "contacts device primary: unknown action",
                                    );
                                    emit_marker(
                                        "tui_contacts_invalid",
                                        None,
                                        &[("reason", "unknown_primary_action")],
                                    );
                                    return false;
                                }
                            }
                        }
                        _ => {
                            state.set_command_error("contacts device: unknown action");
                            emit_marker(
                                "tui_contacts_invalid",
                                None,
                                &[("reason", "unknown_device_action")],
                            );
                            return false;
                        }
                    }
                }
                "block" => {
                    let Some(label) = cmd.args.get(1).map(|s| s.as_str()) else {
                        state.set_command_error("contacts: missing label");
                        emit_marker("tui_contacts_invalid", None, &[("reason", "missing_label")]);
                        return false;
                    };
                    if let Some(rec) = state.contacts_records.get_mut(label) {
                        rec.blocked = true;
                        match state.persist_contacts_cache() {
                            Ok(()) => emit_marker(
                                "tui_contacts_block",
                                None,
                                &[("label", label), ("ok", "true")],
                            ),
                            Err(_) => {
                                state.set_command_error("contacts: store unavailable");
                                emit_marker(
                                    "tui_contacts_block",
                                    Some("contacts_store_unavailable"),
                                    &[("label", label), ("ok", "false")],
                                );
                                return false;
                            }
                        }
                    } else {
                        state.set_command_error("contacts: unknown alias");
                        emit_marker(
                            "tui_contacts_block",
                            Some("peer_unknown"),
                            &[("label", label), ("ok", "false")],
                        );
                        return false;
                    }
                    state.refresh_contacts();
                }
                "unblock" => {
                    let Some(label) = cmd.args.get(1).map(|s| s.as_str()) else {
                        state.set_command_error("contacts: missing label");
                        emit_marker("tui_contacts_invalid", None, &[("reason", "missing_label")]);
                        return false;
                    };
                    if let Some(rec) = state.contacts_records.get_mut(label) {
                        rec.blocked = false;
                        match state.persist_contacts_cache() {
                            Ok(()) => emit_marker(
                                "tui_contacts_unblock",
                                None,
                                &[("label", label), ("ok", "true")],
                            ),
                            Err(_) => {
                                state.set_command_error("contacts: store unavailable");
                                emit_marker(
                                    "tui_contacts_unblock",
                                    Some("contacts_store_unavailable"),
                                    &[("label", label), ("ok", "false")],
                                );
                                return false;
                            }
                        }
                    } else {
                        state.set_command_error("contacts: unknown alias");
                        emit_marker(
                            "tui_contacts_unblock",
                            Some("peer_unknown"),
                            &[("label", label), ("ok", "false")],
                        );
                        return false;
                    }
                    state.refresh_contacts();
                }
                "add" => {
                    let Some(label) = cmd.args.get(1).map(|s| s.as_str()) else {
                        state.set_command_error("contacts: missing label");
                        emit_marker("tui_contacts_invalid", None, &[("reason", "missing_label")]);
                        return false;
                    };
                    if !tui_alias_is_valid(label) {
                        state
                            .set_command_error("contacts: alias must be 2-32 chars [A-Za-z0-9._-]");
                        emit_marker("tui_contacts_invalid", None, &[("reason", "alias_invalid")]);
                        return false;
                    }
                    let Some(code) = cmd.args.get(2).map(|s| s.as_str()) else {
                        state.set_command_error("contacts: missing verification code");
                        emit_marker("tui_contacts_invalid", None, &[("reason", "missing_fp")]);
                        return false;
                    };
                    if !tui_verification_code_is_valid(code) {
                        state.set_command_error("contacts: invalid verification code format");
                        emit_marker("tui_contacts_invalid", None, &[("reason", "invalid_code")]);
                        return false;
                    }
                    let route_token = match cmd.args.get(3).map(|s| s.as_str()) {
                        Some(raw) => match normalize_route_token(raw) {
                            Ok(token) => Some(token),
                            Err(code) => {
                                state.set_command_error(format!("contacts: {}", code));
                                emit_marker(
                                    "tui_contacts_invalid",
                                    Some(code),
                                    &[("reason", "invalid_route_token")],
                                );
                                return false;
                            }
                        },
                        None => Some(generate_route_token()),
                    };
                    let rec = ContactRecord {
                        fp: code.to_ascii_uppercase(),
                        status: "UNVERIFIED".to_string(),
                        blocked: false,
                        seen_at: None,
                        sig_fp: None,
                        route_token: route_token.clone(),
                        primary_device_id: None,
                        devices: vec![ContactDeviceRecord {
                            device_id: device_id_short(label, None, code),
                            fp: code.to_ascii_uppercase(),
                            sig_fp: None,
                            state: "UNVERIFIED".to_string(),
                            route_token: route_token.clone(),
                            seen_at: None,
                            label: None,
                        }],
                    };
                    state.contacts_records.insert(label.to_string(), rec);
                    match state.persist_contacts_cache() {
                        Ok(()) => emit_marker(
                            "tui_contacts_add",
                            None,
                            &[("label", label), ("ok", "true"), ("status", "UNVERIFIED")],
                        ),
                        Err(_) => {
                            state.set_command_error("contacts: store unavailable");
                            emit_marker(
                                "tui_contacts_add",
                                Some("contacts_store_unavailable"),
                                &[("label", label), ("ok", "false")],
                            );
                            return false;
                        }
                    }
                    emit_tui_contact_flow(
                        "add",
                        "DISCOVERED",
                        label,
                        None,
                        state.trust_onboarding_mode,
                    );
                    state.refresh_contacts();
                }
                "route" => {
                    let Some(action) = cmd.args.get(1).map(|s| s.as_str()) else {
                        state.set_command_error("contacts: missing route subcommand");
                        emit_marker(
                            "tui_contacts_invalid",
                            None,
                            &[("reason", "missing_route_subcmd")],
                        );
                        return false;
                    };
                    if action != "set" {
                        state.set_command_error("contacts: unknown route subcommand");
                        emit_marker(
                            "tui_contacts_invalid",
                            None,
                            &[("reason", "unknown_route_subcmd")],
                        );
                        return false;
                    }
                    let Some(label) = cmd.args.get(2).map(|s| s.as_str()) else {
                        state.set_command_error("contacts: missing alias");
                        emit_marker("tui_contacts_invalid", None, &[("reason", "missing_label")]);
                        return false;
                    };
                    let Some(raw_token) = cmd.args.get(3).map(|s| s.as_str()) else {
                        state.set_command_error("contacts: missing route token");
                        emit_marker("tui_contacts_invalid", None, &[("reason", "missing_token")]);
                        return false;
                    };
                    let token = match normalize_route_token(raw_token) {
                        Ok(v) => v,
                        Err(code) => {
                            state.set_command_error(format!("contacts: {}", code));
                            emit_marker("tui_contacts_invalid", Some(code), &[]);
                            return false;
                        }
                    };
                    let rec =
                        state
                            .contacts_records
                            .entry(label.to_string())
                            .or_insert(ContactRecord {
                                fp: "UNSET".to_string(),
                                status: "UNVERIFIED".to_string(),
                                blocked: false,
                                seen_at: None,
                                sig_fp: None,
                                route_token: None,
                                primary_device_id: None,
                                devices: vec![ContactDeviceRecord {
                                    device_id: device_id_short(label, None, "UNSET"),
                                    fp: "UNSET".to_string(),
                                    sig_fp: None,
                                    state: "UNVERIFIED".to_string(),
                                    route_token: None,
                                    seen_at: None,
                                    label: None,
                                }],
                            });
                    rec.route_token = Some(token);
                    let primary_route_token = rec.route_token.clone();
                    if let Some(primary) = primary_device_mut(rec) {
                        primary.route_token = primary_route_token;
                    }
                    if state.persist_contacts_cache().is_err() {
                        state.set_command_error("contacts: store unavailable");
                        emit_marker(
                            "tui_contacts_route",
                            Some("contacts_store_unavailable"),
                            &[("label", label), ("ok", "false")],
                        );
                        return false;
                    }
                    emit_marker(
                        "tui_contacts_route",
                        None,
                        &[("label", label), ("ok", "true"), ("action", "set")],
                    );
                    state.push_cmd_result(
                        "contacts route set",
                        true,
                        "contact route token stored (redacted)",
                    );
                    state.set_status_last_command_result(format!("contact route set {}", label));
                    state.set_command_feedback("ok: contact route token set");
                    state.refresh_contacts();
                }
                _ => {
                    state.set_command_error("contacts: unknown subcommand");
                    emit_marker(
                        "tui_contacts_invalid",
                        None,
                        &[("reason", "unknown_subcmd")],
                    );
                }
            }
            false
        }
        "trust" => {
            emit_marker("tui_cmd", None, &[("cmd", "trust")]);
            let Some(action) = cmd.args.first().map(|s| s.as_str()) else {
                state.set_command_error("trust: missing action (use pin)");
                emit_marker("tui_trust_invalid", None, &[("reason", "missing_action")]);
                return false;
            };
            match action {
                "pin" => {
                    let Some(label) = cmd.args.get(1).map(|s| s.as_str()) else {
                        state.set_command_error("trust: missing alias");
                        emit_marker("tui_trust_invalid", None, &[("reason", "missing_label")]);
                        return false;
                    };
                    let confirmed = cmd
                        .args
                        .get(2)
                        .map(|s| s.eq_ignore_ascii_case("confirm"))
                        .unwrap_or(false);
                    if !confirmed {
                        state.set_command_error(
                            "trust: confirmation required (use '/trust pin <alias> confirm')",
                        );
                        emit_marker(
                            "tui_trust_pin",
                            Some("confirm_required"),
                            &[("label", label), ("ok", "false")],
                        );
                        return false;
                    }
                    let Some(rec) = state.contacts_records.get_mut(label) else {
                        state.set_command_error("trust: unknown alias");
                        emit_marker(
                            "tui_trust_pin",
                            Some("peer_unknown"),
                            &[("label", label), ("ok", "false")],
                        );
                        return false;
                    };
                    normalize_contact_record(label, rec);
                    rec.status = "PINNED".to_string();
                    if let Some(primary) = primary_device_mut(rec) {
                        primary.state = "TRUSTED".to_string();
                    }
                    if state.persist_contacts_cache().is_err() {
                        state.set_command_error("trust: store unavailable");
                        emit_marker(
                            "tui_trust_pin",
                            Some("contacts_store_unavailable"),
                            &[("label", label), ("ok", "false")],
                        );
                        return false;
                    }
                    state.push_cmd_result("trust pin", true, "contact pinned");
                    state.set_status_last_command_result(format!("trust pinned {}", label));
                    state.set_command_feedback("ok: contact trusted (allowed to send)");
                    emit_marker(
                        "tui_trust_pin",
                        None,
                        &[("label", label), ("ok", "true"), ("status", "PINNED")],
                    );
                    state.refresh_contacts();
                }
                "mode" => {
                    let mode_arg = cmd.args.get(1).map(|s| s.as_str());
                    match mode_arg {
                        None | Some("show") => {
                            let mode = state.trust_onboarding_mode.as_str();
                            emit_tui_named_marker("QSC_TUI_TRUST_MODE", &[("mode", mode)]);
                            state.push_cmd_result("trust mode", true, format!("mode={mode}"));
                        }
                        Some(raw) => {
                            let Some(mode) = TrustOnboardingMode::from_raw(raw) else {
                                state.set_command_error("trust mode: expected strict|balanced");
                                emit_tui_named_marker(
                                    "QSC_TUI_TRUST_MODE",
                                    &[("ok", "false"), ("reason", "invalid_mode")],
                                );
                                return false;
                            };
                            if state
                                .persist_account_secret(TUI_TRUST_MODE_SECRET_KEY, mode.as_str())
                                .is_err()
                            {
                                state.set_command_error("trust mode: store unavailable");
                                emit_tui_named_marker(
                                    "QSC_TUI_TRUST_MODE",
                                    &[("ok", "false"), ("reason", "contacts_store_unavailable")],
                                );
                                return false;
                            }
                            state.trust_onboarding_mode = mode;
                            emit_tui_named_marker(
                                "QSC_TUI_TRUST_MODE",
                                &[("mode", mode.as_str()), ("ok", "true")],
                            );
                            state.push_cmd_result(
                                "trust mode",
                                true,
                                format!("mode={}", mode.as_str()),
                            );
                        }
                    }
                }
                _ => {
                    state.set_command_error("trust: unknown action");
                    emit_marker("tui_trust_invalid", None, &[("reason", "unknown_action")]);
                }
            }
            false
        }
        "requests" => {
            emit_marker("tui_cmd", None, &[("cmd", "requests")]);
            let action = cmd.args.first().map(|s| s.as_str()).unwrap_or("list");
            match action {
                "list" => {
                    let items = contact_request_list().unwrap_or_default();
                    let count_s = items.len().to_string();
                    emit_tui_contact_request("list", "all", None);
                    state.push_cmd_result("requests list", true, format!("count={count_s}"));
                }
                "accept" => {
                    let Some(label) = cmd.args.get(1).map(|s| s.as_str()) else {
                        state.set_command_error("requests accept: missing alias");
                        return false;
                    };
                    let removed = contact_request_remove(label).unwrap_or(false);
                    if !removed {
                        state.set_command_error("requests accept: unknown request");
                        return false;
                    }
                    let mut rec =
                        state
                            .contacts_records
                            .get(label)
                            .cloned()
                            .unwrap_or(ContactRecord {
                                fp: "UNSET".to_string(),
                                status: "UNVERIFIED".to_string(),
                                blocked: false,
                                seen_at: None,
                                sig_fp: None,
                                route_token: None,
                                primary_device_id: None,
                                devices: vec![ContactDeviceRecord {
                                    device_id: device_id_short(label, None, "UNSET"),
                                    fp: "UNSET".to_string(),
                                    sig_fp: None,
                                    state: "UNVERIFIED".to_string(),
                                    route_token: None,
                                    seen_at: None,
                                    label: Some("request".to_string()),
                                }],
                            });
                    normalize_contact_record(label, &mut rec);
                    rec.status = "UNVERIFIED".to_string();
                    if state.persist_contacts_cache_with(label, rec).is_err() {
                        state.set_command_error("requests accept: store unavailable");
                        return false;
                    }
                    state.refresh_contacts();
                    emit_tui_contact_request("accept", label, None);
                    emit_tui_contact_flow(
                        "add",
                        "DISCOVERED",
                        label,
                        None,
                        state.trust_onboarding_mode,
                    );
                    state.push_cmd_result("requests accept", true, label.to_string());
                }
                "ignore" => {
                    let Some(label) = cmd.args.get(1).map(|s| s.as_str()) else {
                        state.set_command_error("requests ignore: missing alias");
                        return false;
                    };
                    let removed = contact_request_remove(label).unwrap_or(false);
                    if !removed {
                        state.set_command_error("requests ignore: unknown request");
                        return false;
                    }
                    emit_tui_contact_request("ignore", label, None);
                    state.push_cmd_result("requests ignore", true, label.to_string());
                }
                "block" => {
                    let Some(label) = cmd.args.get(1).map(|s| s.as_str()) else {
                        state.set_command_error("requests block: missing alias");
                        return false;
                    };
                    let _ = contact_request_remove(label);
                    let mut rec =
                        state
                            .contacts_records
                            .get(label)
                            .cloned()
                            .unwrap_or(ContactRecord {
                                fp: "UNSET".to_string(),
                                status: "REVOKED".to_string(),
                                blocked: true,
                                seen_at: None,
                                sig_fp: None,
                                route_token: None,
                                primary_device_id: None,
                                devices: vec![ContactDeviceRecord {
                                    device_id: device_id_short(label, None, "UNSET"),
                                    fp: "UNSET".to_string(),
                                    sig_fp: None,
                                    state: "REVOKED".to_string(),
                                    route_token: None,
                                    seen_at: None,
                                    label: Some("blocked_request".to_string()),
                                }],
                            });
                    normalize_contact_record(label, &mut rec);
                    rec.blocked = true;
                    if let Some(primary) = primary_device_mut(&mut rec) {
                        primary.state = "REVOKED".to_string();
                    }
                    if state.persist_contacts_cache_with(label, rec).is_err() {
                        state.set_command_error("requests block: store unavailable");
                        return false;
                    }
                    state.refresh_contacts();
                    emit_tui_contact_request("block", label, None);
                    state.push_cmd_result("requests block", true, label.to_string());
                }
                _ => {
                    state.set_command_error("requests: unknown action");
                    return false;
                }
            }
            false
        }
        "verify" => {
            emit_marker("tui_cmd", None, &[("cmd", "verify")]);
            let Some(label) = cmd.args.first().map(|s| s.as_str()) else {
                state.set_command_error("verify: missing alias");
                emit_marker("tui_contacts_invalid", None, &[("reason", "missing_label")]);
                return false;
            };
            let Some(code) = cmd.args.get(1).map(|s| s.as_str()) else {
                state.set_command_error("verify: missing verification code");
                emit_marker("tui_contacts_invalid", None, &[("reason", "missing_fp")]);
                return false;
            };
            if !tui_verification_code_is_valid(code) {
                state.set_command_error("verify: invalid verification code format");
                emit_marker("tui_contacts_invalid", None, &[("reason", "invalid_code")]);
                return false;
            }
            let Some(rec) = state.contacts_records.get_mut(label) else {
                state.set_command_error("verify: unknown alias");
                emit_marker(
                    "tui_contacts_verify",
                    Some("peer_unknown"),
                    &[("label", label), ("ok", "false")],
                );
                return false;
            };
            normalize_contact_record(label, rec);
            let expected = primary_device(rec)
                .map(|d| d.fp.to_ascii_uppercase())
                .unwrap_or_else(|| rec.fp.to_ascii_uppercase());
            let provided = code.to_ascii_uppercase();
            if expected == provided {
                let mode = state.trust_onboarding_mode;
                rec.status = "VERIFIED".to_string();
                if let Some(primary) = primary_device_mut(rec) {
                    primary.state = "VERIFIED".to_string();
                    if mode == TrustOnboardingMode::Balanced {
                        primary.state = "TRUSTED".to_string();
                        rec.status = "PINNED".to_string();
                    }
                }
                if state.persist_contacts_cache().is_err() {
                    state.set_command_error("verify: store unavailable");
                    emit_marker(
                        "tui_contacts_verify",
                        Some("contacts_store_unavailable"),
                        &[("label", label), ("ok", "false")],
                    );
                    return false;
                }
                emit_marker(
                    "tui_contacts_verify",
                    None,
                    &[
                        ("label", label),
                        ("ok", "true"),
                        (
                            "status",
                            if mode == TrustOnboardingMode::Balanced {
                                "TRUSTED"
                            } else {
                                "VERIFIED"
                            },
                        ),
                    ],
                );
                emit_tui_contact_flow(
                    "verify",
                    if mode == TrustOnboardingMode::Balanced {
                        "TRUSTED"
                    } else {
                        "VERIFIED"
                    },
                    label,
                    None,
                    mode,
                );
                if mode == TrustOnboardingMode::Balanced {
                    emit_tui_trust_promotion("trusted", "verified_match", label, None, mode);
                    state.set_command_feedback("ok: verification matched and contact auto-trusted");
                } else {
                    emit_tui_trust_promotion("verified_only", "strict_mode", label, None, mode);
                    state.set_command_feedback(
                        "ok: verification matched; strict mode requires explicit trust",
                    );
                }
            } else {
                rec.status = "CHANGED".to_string();
                if let Some(primary) = primary_device_mut(rec) {
                    primary.state = "CHANGED".to_string();
                }
                let _ = state.persist_contacts_cache();
                emit_marker(
                    "tui_contacts_verify",
                    Some("verification_mismatch"),
                    &[("label", label), ("ok", "false"), ("status", "CHANGED")],
                );
                emit_tui_contact_flow(
                    "verify",
                    "CHANGED",
                    label,
                    None,
                    state.trust_onboarding_mode,
                );
                state.set_command_error("verify: verification code mismatch");
                return false;
            }
            state.refresh_contacts();
            false
        }
        "messages" => {
            emit_marker("tui_cmd", None, &[("cmd", "messages")]);
            let sub = cmd.args.first().map(|s| s.as_str()).unwrap_or("list");
            match sub {
                "list" => {
                    let labels = state.conversation_labels();
                    let count_s = labels.len().to_string();
                    emit_marker("tui_messages_list", None, &[("count", count_s.as_str())]);
                }
                "select" => {
                    let Some(peer) = cmd.args.get(1).map(|s| s.as_str()) else {
                        state.set_command_error("messages: missing peer");
                        emit_marker("tui_messages_invalid", None, &[("reason", "missing_peer")]);
                        return false;
                    };
                    let labels = state.conversation_labels();
                    let known_contact = state.contact_record_cached(peer).is_some();
                    if labels.iter().any(|p| p == peer) || known_contact {
                        state.focus_messages_thread(peer);
                        emit_marker(
                            "tui_messages_select",
                            None,
                            &[("peer", peer), ("ok", "true")],
                        );
                    } else {
                        emit_marker(
                            "tui_messages_select",
                            None,
                            &[("peer", peer), ("ok", "false")],
                        );
                    }
                }
                _ => {
                    state.set_command_error("messages: unknown subcommand");
                    emit_marker(
                        "tui_messages_invalid",
                        None,
                        &[("reason", "unknown_subcmd")],
                    );
                }
            }
            false
        }
        "msg" => {
            emit_marker("tui_cmd", None, &[("cmd", "msg")]);
            let (thread, text) = if cmd.args.len() >= 2 {
                let peer = cmd.args[0].clone();
                let body = cmd.args[1..].join(" ");
                (peer, body)
            } else {
                let Some(selected) = state.selected_messages_thread() else {
                    state.set_command_error("msg: select a messages thread first");
                    emit_marker("tui_msg_reject", None, &[("reason", "thread_not_selected")]);
                    return false;
                };
                (selected, cmd.args.join(" "))
            };
            let trimmed = text.trim();
            if trimmed.is_empty() {
                state.set_command_error("msg: empty message");
                emit_marker("tui_msg_reject", None, &[("reason", "empty")]);
                return false;
            }
            if trimmed.chars().count() > TUI_MESSAGE_MAX_CHARS {
                state.set_command_error("msg: message too long");
                emit_marker("tui_msg_reject", None, &[("reason", "too_long")]);
                return false;
            }
            if thread.eq_ignore_ascii_case("Note to Self")
                || thread.eq_ignore_ascii_case("Note_to_Self")
            {
                let timeline_peer = TuiState::map_thread_to_timeline_peer(thread.as_str());
                if state
                    .append_tui_timeline_entry(
                        timeline_peer,
                        "out",
                        trimmed.len(),
                        "msg",
                        MessageState::Sent,
                    )
                    .is_err()
                {
                    state.set_command_error("msg: timeline unavailable");
                    emit_marker(
                        "tui_msg_reject",
                        None,
                        &[("reason", "timeline_unavailable")],
                    );
                    return false;
                }
                state.record_message_line(thread.as_str(), "SENT", "out", trimmed);
                state.focus_messages_thread(thread.as_str());
                emit_tui_delivery_state(thread.as_str(), "accepted_by_relay");
                let len_s = trimmed.len().to_string();
                emit_marker(
                    "tui_msg_send",
                    None,
                    &[
                        ("peer", thread.as_str()),
                        ("len", len_s.as_str()),
                        ("ok", "true"),
                    ],
                );
            } else {
                if let Err(reason) = state.trust_allows_peer_send_strict(thread.as_str()) {
                    if reason == "no_trusted_device"
                        && tui_msg_autotrust_first_use(state, thread.as_str()).is_ok()
                    {
                        if state
                            .trust_allows_peer_send_strict(thread.as_str())
                            .is_err()
                        {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
                let effective_relay = state.effective_relay_config();
                if effective_relay.is_none() {
                    state.emit_setup_required_marker_if_needed();
                    state.set_command_error(
                        "msg: relay not configured; use '/relay set endpoint <https://...>'",
                    );
                    emit_marker(
                        "tui_msg_reject",
                        None,
                        &[("reason", "explicit_only_no_transport")],
                    );
                    return false;
                }
                if let Err(code) = tui_msg_ensure_handshake(state, thread.as_str()) {
                    emit_marker(
                        "tui_msg_reject",
                        Some(code),
                        &[("reason", code), ("peer", thread.as_str())],
                    );
                    state.set_command_error("msg: handshake failed");
                    return false;
                }
                let relay = match effective_relay.as_ref() {
                    Some(v) => v,
                    None => return false,
                };
                let routing = match tui_resolve_peer_device_target(state, thread.as_str(), true) {
                    Ok(v) => v,
                    Err(code) => {
                        emit_marker(
                            "tui_msg_reject",
                            Some(code),
                            &[("reason", code), ("peer", thread.as_str())],
                        );
                        state.set_command_error("msg: send blocked");
                        return false;
                    }
                };
                if let Err(code) = tui_enforce_peer_not_blocked(state, routing.channel.as_str()) {
                    emit_marker(
                        "tui_msg_reject",
                        Some(code),
                        &[("reason", code), ("peer", thread.as_str())],
                    );
                    state.set_command_error("msg: send blocked");
                    return false;
                }
                let outcome = relay_send_with_payload(RelaySendPayloadArgs {
                    to: thread.as_str(),
                    payload: trimmed.as_bytes().to_vec(),
                    relay: relay.relay.as_str(),
                    injector: fault_injector_from_tui(relay),
                    pad_cfg: None,
                    bucket_max: None,
                    meta_seed: None,
                    receipt: None,
                    routing_override: Some(routing),
                    tui_thread: Some(thread.as_str()),
                });
                if !outcome.delivered {
                    state.set_command_error("msg: send failed");
                    emit_tui_named_marker("QSC_TUI_ORCH", &[("stage", "send"), ("status", "fail")]);
                    return false;
                }
                emit_tui_named_marker("QSC_TUI_ORCH", &[("stage", "send"), ("status", "ok")]);
                let timeline_peer = TuiState::map_thread_to_timeline_peer(thread.as_str());
                if state
                    .append_tui_timeline_entry(
                        timeline_peer,
                        "out",
                        trimmed.len(),
                        "msg",
                        MessageState::Sent,
                    )
                    .is_err()
                {
                    state.set_command_error("msg: timeline unavailable");
                    emit_marker(
                        "tui_msg_reject",
                        None,
                        &[("reason", "timeline_unavailable")],
                    );
                    return false;
                }
                state.record_message_line(thread.as_str(), "SENT", "out", trimmed);
                state.focus_messages_thread(thread.as_str());
                tui_msg_recv_poll_bounded(state, thread.as_str());
                let len_s = trimmed.len().to_string();
                emit_marker(
                    "tui_msg_send",
                    None,
                    &[
                        ("peer", thread.as_str()),
                        ("len", len_s.as_str()),
                        ("ok", "true"),
                    ],
                );
            }
            false
        }
        "files" => {
            emit_marker("tui_cmd", None, &[("cmd", "files")]);
            state.refresh_files_from_timeline();
            let sub = cmd.args.first().map(|s| s.as_str()).unwrap_or("list");
            match sub {
                "list" => {
                    emit_marker(
                        "tui_files_list",
                        None,
                        &[("count", state.files.len().to_string().as_str())],
                    );
                }
                "select" => {
                    let Some(id) = cmd.args.get(1).map(|s| s.as_str()) else {
                        state.set_command_error("files: missing id");
                        emit_marker("tui_files_invalid", None, &[("reason", "missing_id")]);
                        return false;
                    };
                    let ok = state.files_select_by_id(id);
                    emit_marker(
                        "tui_files_select",
                        None,
                        &[("id", id), ("ok", if ok { "true" } else { "false" })],
                    );
                }
                "toggle" => {
                    if state.inspector != TuiInspectorPane::Files {
                        state.set_command_error("files: multiselect only allowed in files");
                        emit_marker(
                            "tui_files_multiselect_blocked",
                            None,
                            &[
                                ("reason", "domain_not_files"),
                                ("domain", state.inspector_name()),
                            ],
                        );
                        return false;
                    }
                    if let Some(id) = cmd.args.get(1).map(|s| s.as_str()) {
                        if !state.files_select_by_id(id) {
                            state.set_command_error("files: unknown id");
                            emit_marker(
                                "tui_files_toggle",
                                None,
                                &[("id", id), ("ok", "false"), ("reason", "unknown_id")],
                            );
                            return false;
                        }
                    }
                    let ok = state.files_toggle_selected();
                    emit_marker(
                        "tui_files_toggle",
                        None,
                        &[
                            ("id", state.selected_file_id().unwrap_or("none")),
                            ("ok", if ok { "true" } else { "false" }),
                            (
                                "selected",
                                state.file_multi_selected.len().to_string().as_str(),
                            ),
                        ],
                    );
                }
                "clear-selection" => {
                    state.file_multi_selected.clear();
                    emit_marker(
                        "tui_files_clear_selection",
                        None,
                        &[("ok", "true"), ("selected", "0")],
                    );
                }
                "inject" => {
                    let Some(id) = cmd.args.get(1).map(|s| s.as_str()) else {
                        state.set_command_error("files: missing id");
                        emit_marker("tui_files_invalid", None, &[("reason", "missing_id")]);
                        return false;
                    };
                    let state_name = cmd.args.get(2).map(|s| s.as_str()).unwrap_or("RECEIVING");
                    let byte_len = cmd
                        .args
                        .get(3)
                        .and_then(|v| v.parse::<usize>().ok())
                        .unwrap_or(0usize);
                    let filename = cmd
                        .args
                        .get(4)
                        .map(|s| s.to_string())
                        .unwrap_or_else(|| format!("{}.bin", id));
                    let item = TuiFileItem {
                        id: id.to_string(),
                        peer: state.selected_conversation_label(),
                        filename,
                        byte_len,
                        state: state_name.to_string(),
                        display_state: tui_file_display_state(state_name),
                    };
                    state.upsert_file_item(item, true);
                    emit_marker(
                        "tui_files_inject",
                        None,
                        &[
                            ("id", id),
                            ("state", state_name),
                            ("display_state", tui_file_display_state(state_name).as_str()),
                        ],
                    );
                }
                _ => {
                    state.set_command_error("files: unknown subcommand");
                    emit_marker("tui_files_invalid", None, &[("reason", "unknown_subcmd")]);
                }
            }
            false
        }
        "injectmsg" => {
            emit_marker("tui_cmd", None, &[("cmd", "injectmsg")]);
            let peer = cmd.args.first().map(|s| s.as_str()).unwrap_or("peer-0");
            let state_name = cmd.args.get(1).map(|s| s.as_str()).unwrap_or("RECEIVED");
            let detail = if cmd.args.len() > 2 {
                cmd.args[2..].join(" ")
            } else {
                "source=test_harness".to_string()
            };
            let _ = state.append_tui_timeline_entry(
                peer,
                "in",
                detail.len(),
                "msg",
                MessageState::Received,
            );
            state.record_message_line(peer, state_name, "in", detail.as_str());
            false
        }
        "injectevent" => {
            emit_marker("tui_cmd", None, &[("cmd", "injectevent")]);
            let kind = cmd.args.first().map(|s| s.as_str()).unwrap_or("activity");
            let action = cmd.args.get(1).map(|s| s.as_str()).unwrap_or("test");
            state.push_event(kind, action);
            false
        }
        "envelope" => {
            emit_marker("tui_cmd", None, &[("cmd", "envelope")]);
            state.refresh_envelope(state.last_payload_len());
            false
        }
        "export" => {
            emit_marker("tui_cmd", None, &[("cmd", "export")]);
            false
        }
        other => {
            state.set_command_error(format!("unknown command: {}", other));
            emit_marker("tui_cmd", None, &[("cmd", other)]);
            false
        }
    };
    if state.cmd_results.len() == before_results_len {
        if let Some(err) = state.command_error.clone() {
            state.push_cmd_result(command_label.as_str(), false, err);
        } else {
            state.push_cmd_result(command_label.as_str(), true, "ok");
        }
    }
    if !exit && state.command_error.is_none() {
        if let Some(entry) = state.cmd_results.back() {
            if let Some(msg) = entry
                .strip_prefix("[ok] /")
                .and_then(|v| v.split_once(' '))
                .map(|(_, msg)| msg.to_string())
            {
                state.set_command_feedback(format!("ok: {}", msg));
            } else {
                state.set_command_feedback(format!("ok: /{}", command_label));
            }
        } else {
            state.set_command_feedback(format!("ok: /{}", command_label));
        }
    }
    state.end_command_tracking();
    exit
}

fn tui_msg_ensure_handshake(state: &mut TuiState, peer: &str) -> Result<(), &'static str> {
    let routing = resolve_peer_device_target(peer, false)?;
    let inbox_route_token = state.tui_relay_inbox_route_token()?;
    if protocol_active_or_reason_for_peer(routing.channel.as_str()).is_ok() {
        emit_tui_named_marker(
            "QSC_TUI_ORCH",
            &[("stage", "ensure_handshake"), ("status", "skip")],
        );
        return Ok(());
    }
    let relay = state
        .effective_relay_config()
        .ok_or("explicit_only_no_transport")?
        .relay;
    let self_label = env::var("QSC_SELF_LABEL").unwrap_or_else(|_| "self".to_string());
    let backoff_ms = [50u64, 100, 200];
    let _ = inbox_route_token;
    handshake_init(&self_label, peer, relay.as_str());
    for delay in backoff_ms {
        handshake_poll(&self_label, peer, relay.as_str(), 4);
        let routing = resolve_peer_device_target(peer, false)?;
        if protocol_active_or_reason_for_peer(routing.channel.as_str()).is_ok() {
            emit_tui_named_marker(
                "QSC_TUI_ORCH",
                &[("stage", "ensure_handshake"), ("status", "ok")],
            );
            return Ok(());
        }
        std::thread::sleep(Duration::from_millis(delay));
    }
    emit_tui_named_marker(
        "QSC_TUI_ORCH_FAIL",
        &[("stage", "handshake"), ("code", "handshake_incomplete")],
    );
    Err("handshake_incomplete")
}

fn tui_msg_autotrust_first_use(state: &mut TuiState, peer: &str) -> Result<(), &'static str> {
    let mode = state.trust_onboarding_mode;
    let Some(rec) = state.contacts_records.get(peer) else {
        return Err("unknown_contact");
    };
    let Some(primary) = primary_device(rec) else {
        return Err("no_trusted_device");
    };
    let primary_state = canonical_device_state(primary.state.as_str());
    if primary_state == "CHANGED" {
        return Err("device_changed_reapproval_required");
    }
    if primary_state == "REVOKED" {
        return Err("device_revoked");
    }
    if primary_state != "UNVERIFIED" && primary_state != "VERIFIED" {
        return Err("no_trusted_device");
    }
    if mode == TrustOnboardingMode::Strict {
        emit_cli_trust_promotion("verified_only", "strict_mode", peer, None, mode);
        emit_tui_trust_promotion("verified_only", "strict_mode", peer, None, mode);
        return Err("no_trusted_device");
    }
    if tui_msg_ensure_handshake(state, peer).is_err() {
        return Err("handshake_incomplete");
    }
    let mut rec = match state.contacts_records.get(peer) {
        Some(v) => v.clone(),
        None => return Err("unknown_contact"),
    };
    normalize_contact_record(peer, &mut rec);
    let Some(primary) = primary_device_mut(&mut rec) else {
        return Err("no_trusted_device");
    };
    let state_now = canonical_device_state(primary.state.as_str());
    if state_now == "CHANGED" || state_now == "REVOKED" {
        return Err("device_changed_reapproval_required");
    }
    primary.state = "TRUSTED".to_string();
    if state.persist_contacts_cache_with(peer, rec).is_err() {
        emit_tui_named_marker(
            "QSC_TUI_ORCH_FAIL",
            &[
                ("stage", "auto_trust"),
                ("code", "contacts_store_unavailable"),
            ],
        );
        return Err("contacts_store_unavailable");
    }
    emit_cli_trust_promotion("trusted", "verified_match", peer, None, mode);
    emit_tui_trust_promotion("trusted", "verified_match", peer, None, mode);
    emit_tui_named_marker("QSC_TUI_ORCH", &[("stage", "auto_trust"), ("status", "ok")]);
    Ok(())
}

fn tui_msg_recv_poll_bounded(state: &mut TuiState, peer: &str) {
    let mut got_total = 0usize;
    let mut polls = 0usize;
    for delay in [0u64, 50, 100] {
        if delay > 0 {
            std::thread::sleep(Duration::from_millis(delay));
        }
        let before = state.session.recv_count;
        tui_receive_via_relay(state, peer);
        polls = polls.saturating_add(1);
        let after = state.session.recv_count;
        if after > before {
            got_total = got_total.saturating_add((after - before) as usize);
        }
        if got_total > 0 {
            break;
        }
    }
    let polls_s = polls.to_string();
    let got_s = got_total.to_string();
    emit_tui_named_marker(
        "QSC_TUI_ORCH",
        &[
            ("stage", "recv_poll"),
            ("status", "ok"),
            ("polls", polls_s.as_str()),
            ("got", got_s.as_str()),
        ],
    );
}

fn tui_send_via_relay(state: &mut TuiState, to: &str) {
    let relay = match state.effective_relay_config() {
        Some(v) => v,
        None => {
            emit_marker(
                "tui_send_blocked",
                None,
                &[("reason", "explicit_only_no_transport")],
            );
            state.update_send_lifecycle("blocked");
            return;
        }
    };
    let routing = match tui_resolve_peer_device_target(state, to, true) {
        Ok(v) => v,
        Err(code) => {
            emit_marker("tui_send_blocked", Some(code), &[("reason", code)]);
            state.update_send_lifecycle("blocked");
            return;
        }
    };
    if let Err(code) = tui_enforce_peer_not_blocked(state, routing.channel.as_str()) {
        emit_marker("tui_send_blocked", Some(code), &[("reason", code)]);
        state.update_send_lifecycle("blocked");
        return;
    }
    if let Err(reason) = protocol_active_or_reason_for_peer(routing.channel.as_str()) {
        emit_protocol_inactive(reason.as_str());
        state.update_send_lifecycle("blocked");
        return;
    }
    let payload = tui_payload_bytes(state.send_seq);
    state.send_seq = state.send_seq.wrapping_add(1);
    let outcome = relay_send_with_payload(RelaySendPayloadArgs {
        to,
        payload,
        relay: relay.relay.as_str(),
        injector: fault_injector_from_tui(&relay),
        pad_cfg: None,
        bucket_max: None,
        meta_seed: None,
        receipt: None,
        routing_override: Some(routing),
        tui_thread: Some(to),
    });
    state.push_event("relay_event", outcome.action.as_str());
    if outcome.delivered {
        state.update_send_lifecycle("committed");
        state.session.sent_count = state.session.sent_count.saturating_add(1);
        state.record_message_line(to, "SENT", "out", "transport=relay");
    } else {
        state.update_send_lifecycle("failed");
    }
    state.refresh_qsp_status();
}

fn resolve_receive_out_dir() -> (PathBuf, ConfigSource) {
    if let Ok(dir) = env::var("QSC_RECEIVE_OUT") {
        return (PathBuf::from(dir), ConfigSource::EnvOverride);
    }
    let (dir, source) = match config_dir() {
        Ok(v) => v,
        Err(e) => print_error(e),
    };
    (dir.join("inbox"), source)
}

fn tui_receive_via_relay(state: &mut TuiState, from: &str) {
    let relay = match state.effective_relay_config() {
        Some(v) => v,
        None => {
            emit_marker(
                "tui_receive_blocked",
                None,
                &[("reason", "explicit_only_no_transport")],
            );
            return;
        }
    };
    if let Err(code) = normalize_relay_endpoint(relay.relay.as_str()) {
        emit_marker("tui_receive_blocked", None, &[("reason", code)]);
        return;
    }
    let (out_dir, source) = resolve_receive_out_dir();
    if let Err(e) = ensure_dir_secure(&out_dir, source) {
        print_error(e);
    }
    let (cfg_dir, cfg_source) = match config_dir() {
        Ok(v) => v,
        Err(e) => print_error(e),
    };
    if !check_symlink_safe(&cfg_dir) {
        print_error(ErrorCode::UnsafePathSymlink);
    }
    if !check_parent_safe(&cfg_dir, cfg_source) {
        print_error(ErrorCode::UnsafeParentPerms);
    }
    if let Err(reason) = protocol_active_or_reason_for_peer(from) {
        emit_protocol_inactive(reason.as_str());
        emit_marker("tui_receive", None, &[("from", from), ("count", "0")]);
        state.push_event("recv_blocked", reason.as_str());
        return;
    }
    let inbox_route_token = match state.tui_relay_inbox_route_token() {
        Ok(v) => v,
        Err(code) => {
            emit_marker("tui_receive_blocked", None, &[("reason", code)]);
            state.push_event("recv_blocked", code);
            return;
        }
    };
    let mailbox_hash = route_token_hash8(inbox_route_token.as_str());
    emit_marker(
        "recv_start",
        None,
        &[
            ("transport", "relay"),
            ("mailbox", "redacted"),
            ("mailbox_hash", mailbox_hash.as_str()),
            ("from", from),
            ("max", "1"),
        ],
    );
    let max = 1usize;
    let items = match relay_inbox_pull(&relay.relay, inbox_route_token.as_str(), max) {
        Ok(v) => v,
        Err(code) => print_error_marker(code),
    };
    if items.is_empty() {
        emit_marker("recv_none", None, &[]);
        emit_marker("tui_receive", None, &[("from", from), ("count", "0")]);
        state.push_event("recv_none", from);
        return;
    }
    let mut count = 0usize;
    for item in items {
        match qsp_unpack_for_peer(from, &item.data) {
            Ok((outcome, channel)) => {
                record_qsp_status(&cfg_dir, cfg_source, true, "unpack_ok", false, true);
                emit_marker("qsp_unpack", None, &[("ok", "true"), ("version", "5.0")]);
                let msg_idx_s = outcome.msg_idx.to_string();
                emit_marker(
                    "ratchet_recv_advance",
                    None,
                    &[("msg_idx", msg_idx_s.as_str())],
                );
                if outcome.skip_delta > 0 {
                    let sd = outcome.skip_delta.to_string();
                    emit_marker("ratchet_skip_store", None, &[("count", sd.as_str())]);
                }
                if outcome.evicted > 0 {
                    let ev = outcome.evicted.to_string();
                    emit_marker("ratchet_skip_evict", None, &[("count", ev.as_str())]);
                }
                if qsp_session_store(channel.as_str(), &outcome.next_state).is_err() {
                    emit_marker("error", Some("qsp_session_store_failed"), &[]);
                    print_error_marker("qsp_session_store_failed");
                }
                count = count.saturating_add(1);
                let seq = state.session.recv_count.saturating_add(count as u64);
                let name = format!("recv_{}.bin", seq);
                let path = out_dir.join(&name);
                if write_atomic(&path, &outcome.plaintext, source).is_err() {
                    print_error_marker("recv_write_failed");
                }
                let size_s = outcome.plaintext.len().to_string();
                emit_marker(
                    "recv_item",
                    None,
                    &[
                        ("idx", count.to_string().as_str()),
                        ("size", size_s.as_str()),
                        ("id", item.id.as_str()),
                    ],
                );
                state.events.push_back(format!(
                    "recv: from={} size={} saved={}",
                    from, size_s, name
                ));
                let detail = format!("bytes={}", size_s);
                state.record_message_line(from, "RECEIVED", "in", detail.as_str());
            }
            Err(code) => {
                record_qsp_status(&cfg_dir, cfg_source, false, code, false, false);
                emit_marker("qsp_unpack", Some(code), &[("ok", "false")]);
                if code == "qsp_replay_reject" {
                    let msg_idx = qsp_session_for_channel(from)
                        .map(|st| st.recv.nr.to_string())
                        .unwrap_or_else(|_| "0".to_string());
                    emit_marker("ratchet_replay_reject", None, &[("msg_idx", &msg_idx)]);
                }
                print_error_marker(code);
            }
        }
    }
    state.session.recv_count = state.session.recv_count.saturating_add(count as u64);
    let count_s = count.to_string();
    emit_marker(
        "tui_receive",
        None,
        &[("from", from), ("count", count_s.as_str())],
    );
    emit_marker("recv_commit", None, &[("count", count_s.as_str())]);
    state.refresh_qsp_status();
}

fn tui_payload_bytes(seq: u64) -> Vec<u8> {
    // Deterministic, non-secret payload bytes.
    format!("tui_msg_seq={}", seq).into_bytes()
}

fn draw_tui(f: &mut ratatui::Frame, state: &mut TuiState) {
    let area = f.area();
    match state.mode {
        TuiMode::Help => {
            draw_help_mode(f, area, state);
            return;
        }
        TuiMode::FocusEvents => {
            draw_focus_events(f, area, state);
            return;
        }
        TuiMode::FocusFiles => {
            draw_focus_files(f, area, state);
            return;
        }
        TuiMode::FocusActivity => {
            draw_focus_activity(f, area, state);
            return;
        }
        TuiMode::FocusStatus => {
            draw_focus_status(f, area, state);
            return;
        }
        TuiMode::FocusSession => {
            draw_focus_session(f, area, state);
            return;
        }
        TuiMode::FocusContacts => {
            draw_focus_contacts(f, area, state);
            return;
        }
        TuiMode::FocusSettings => {
            draw_focus_settings(f, area, state);
            return;
        }
        TuiMode::FocusLock => {
            draw_focus_lock(f, area, state);
            return;
        }
        TuiMode::Normal => {}
    }
    let outer = Block::default().borders(Borders::ALL);
    f.render_widget(outer, area);
    let inner = area.inner(ratatui::layout::Margin {
        vertical: 1,
        horizontal: 1,
    });
    if inner.width == 0 || inner.height == 0 {
        return;
    }

    // Fallback for tiny terminals: render command line only.
    if inner.width < 3 || inner.height < 3 {
        let cmd_text = pad_panel_text(state.cmd_bar_text().as_str());
        let cmd = Paragraph::new(Line::from(vec![Span::styled(
            cmd_text.as_str(),
            state.cmd_bar_style(cmd_text.as_str()),
        )]));
        f.render_widget(cmd, inner);
        return;
    }

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Min(1),
                Constraint::Length(1),
                Constraint::Length(1),
            ]
            .as_ref(),
        )
        .split(inner);
    let content_area = rows[0];
    let h_divider_area = rows[1];
    let cmd_area = rows[2];

    let nav_width = ((u32::from(content_area.width) * 26) / 100) as u16;
    let nav_width = nav_width.clamp(1, content_area.width.saturating_sub(2));
    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Length(nav_width),
                Constraint::Length(1),
                Constraint::Min(1),
            ]
            .as_ref(),
        )
        .split(content_area);
    render_unified_nav(f, cols[0], state);
    if content_area.height >= 2 {
        let header_divider_area = Rect {
            x: content_area.x,
            y: content_area.y + 1,
            width: content_area.width,
            height: 1,
        };
        render_header_divider(f, header_divider_area);
    }
    let body_main_area = if cols[2].height > 2 {
        Rect {
            x: cols[2].x,
            y: cols[2].y + 2,
            width: cols[2].width,
            height: cols[2].height - 2,
        }
    } else {
        Rect {
            x: cols[2].x,
            y: cols[2].y + cols[2].height,
            width: cols[2].width,
            height: 0,
        }
    };
    let body_v_divider_area = if cols[1].height > 2 {
        Rect {
            x: cols[1].x,
            y: cols[1].y + 2,
            width: cols[1].width,
            height: cols[1].height - 2,
        }
    } else {
        Rect {
            x: cols[1].x,
            y: cols[1].y + cols[1].height,
            width: cols[1].width,
            height: 0,
        }
    };
    render_vertical_divider(f, body_v_divider_area);
    render_main_panel(f, body_main_area, state);
    render_horizontal_divider(f, h_divider_area);

    let cmd_text = pad_panel_text(state.cmd_bar_text().as_str());
    let cmd_text_marker = cmd_text.replace(' ', "_");
    let cmd = Paragraph::new(Line::from(vec![Span::styled(
        cmd_text.as_str(),
        state.cmd_bar_style(cmd_text.as_str()),
    )]));
    f.render_widget(cmd, cmd_area);
    emit_marker(
        "tui_cmd_render",
        None,
        &[
            ("pad", "2"),
            ("text", cmd_text_marker.as_str()),
            ("focus", state.home_focus_name()),
        ],
    );
}

fn internal_divider_style() -> Style {
    Style::default()
        .fg(Color::DarkGray)
        .add_modifier(Modifier::DIM)
}

fn render_header_divider(f: &mut ratatui::Frame, area: Rect) {
    if area.width == 0 || area.height == 0 {
        return;
    }
    let body = "─".repeat(area.width as usize);
    let line = Line::from(vec![Span::styled(body, internal_divider_style())]);
    f.render_widget(Paragraph::new(line), area);
}

fn render_vertical_divider(f: &mut ratatui::Frame, area: Rect) {
    if area.width == 0 || area.height == 0 {
        return;
    }
    let body = std::iter::repeat_n("│", area.height as usize)
        .collect::<Vec<_>>()
        .join("\n");
    f.render_widget(Paragraph::new(body).style(internal_divider_style()), area);
}

fn render_horizontal_divider(f: &mut ratatui::Frame, area: Rect) {
    if area.width == 0 || area.height == 0 {
        return;
    }
    let body = "─".repeat(area.width as usize);
    f.render_widget(Paragraph::new(body).style(internal_divider_style()), area);
}

fn pad_panel_text(text: &str) -> String {
    let pad = " ".repeat(PANEL_INNER_PAD);
    text.lines()
        .map(|line| {
            if line.is_empty() {
                String::new()
            } else {
                format!("{}{}", pad, line)
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn truncate_with_ellipsis(value: &str, width: usize) -> String {
    if width == 0 {
        return String::new();
    }
    let char_count = value.chars().count();
    if char_count <= width {
        return value.to_string();
    }
    if width == 1 {
        return "…".to_string();
    }
    let prefix = value.chars().take(width - 1).collect::<String>();
    format!("{prefix}…")
}

fn format_contacts_table_row(alias: &str, trust: &str, blocked: &str, last_seen: &str) -> String {
    format!(
        "{:<alias_w$} {:<trust_w$} {:<blocked_w$} {}",
        truncate_with_ellipsis(alias, CONTACTS_COL_ALIAS_WIDTH),
        truncate_with_ellipsis(trust, CONTACTS_COL_TRUST_WIDTH),
        truncate_with_ellipsis(blocked, CONTACTS_COL_BLOCKED_WIDTH),
        last_seen,
        alias_w = CONTACTS_COL_ALIAS_WIDTH,
        trust_w = CONTACTS_COL_TRUST_WIDTH,
        blocked_w = CONTACTS_COL_BLOCKED_WIDTH
    )
}

fn draw_help_mode(f: &mut ratatui::Frame, area: Rect, state: &TuiState) {
    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)].as_ref())
        .split(area);

    let items = tui_help_items();
    let list_items: Vec<ListItem> = items
        .iter()
        .map(|item| ListItem::new(format!("/{} — {}", item.cmd, item.desc)))
        .collect();
    let mut list_state = ratatui::widgets::ListState::default();
    list_state.select(Some(state.help_selected.min(items.len().saturating_sub(1))));

    let list = List::new(list_items)
        .block(Block::default().borders(Borders::ALL).title("Help"))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD));
    f.render_stateful_widget(list, cols[0], &mut list_state);

    let detail = state.help_selected_item();
    let detail_body = match detail {
        Some(item) => format!("command: /{}\n\n{}", item.cmd, item.desc),
        None => "no help items".to_string(),
    };
    let details =
        Paragraph::new(detail_body).block(Block::default().borders(Borders::ALL).title("Details"));
    f.render_widget(details, cols[1]);
}

fn draw_focus_events(f: &mut ratatui::Frame, area: Rect, state: &TuiState) {
    let body = state.focus_events_lines().join("\n");
    let panel = Paragraph::new(body)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("FOCUS: EVENTS (Up/Down PgUp/PgDn Esc back)"),
        )
        .scroll((state.focus_scroll_index() as u16, 0));
    f.render_widget(panel, area);
}

fn draw_focus_files(f: &mut ratatui::Frame, area: Rect, state: &TuiState) {
    let body = state.focus_files_lines().join("\n");
    let panel = Paragraph::new(body)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("FOCUS: FILES (Up/Down PgUp/PgDn Esc back)"),
        )
        .scroll((state.focus_scroll_index() as u16, 0));
    f.render_widget(panel, area);
}

fn draw_focus_activity(f: &mut ratatui::Frame, area: Rect, state: &TuiState) {
    let body = state.focus_activity_lines().join("\n");
    let panel = Paragraph::new(body)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("FOCUS: ACTIVITY (Up/Down PgUp/PgDn Esc back)"),
        )
        .scroll((state.focus_scroll_index() as u16, 0));
    f.render_widget(panel, area);
}

fn draw_focus_status(f: &mut ratatui::Frame, area: Rect, state: &TuiState) {
    let body = state.focus_status_lines().join("\n");
    let panel = Paragraph::new(body)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("FOCUS: STATUS (Up/Down PgUp/PgDn Esc back)"),
        )
        .scroll((state.focus_scroll_index() as u16, 0));
    f.render_widget(panel, area);
}

fn draw_focus_session(f: &mut ratatui::Frame, area: Rect, state: &TuiState) {
    let body = state.focus_session_lines().join("\n");
    let panel = Paragraph::new(body)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("FOCUS: SESSION (Up/Down PgUp/PgDn Esc back)"),
        )
        .scroll((state.focus_scroll_index() as u16, 0));
    f.render_widget(panel, area);
}

fn draw_focus_contacts(f: &mut ratatui::Frame, area: Rect, state: &TuiState) {
    let body = state.focus_contacts_lines().join("\n");
    let panel = Paragraph::new(body)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("FOCUS: CONTACTS (Up/Down PgUp/PgDn Esc back)"),
        )
        .scroll((state.focus_scroll_index() as u16, 0));
    f.render_widget(panel, area);
}

fn draw_focus_settings(f: &mut ratatui::Frame, area: Rect, state: &TuiState) {
    let body = state.focus_settings_lines().join("\n");
    let panel = Paragraph::new(body)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("FOCUS: SETTINGS (Up/Down PgUp/PgDn Esc back)"),
        )
        .scroll((state.focus_scroll_index() as u16, 0));
    f.render_widget(panel, area);
}

fn draw_focus_lock(f: &mut ratatui::Frame, area: Rect, state: &TuiState) {
    let body = state.focus_lock_lines().join("\n");
    let panel = Paragraph::new(body)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("FOCUS: LOCK (Up/Down PgUp/PgDn Esc back)"),
        )
        .scroll((state.focus_scroll_index() as u16, 0));
    f.render_widget(panel, area);
}

fn render_unified_nav(f: &mut ratatui::Frame, area: Rect, state: &TuiState) {
    let rows = state.nav_rows();
    let selected_idx = state.nav_selected.min(rows.len().saturating_sub(1));
    let show_nav_marker = state.home_focus == TuiHomeFocus::Nav;
    let base_pad = " ".repeat(PANEL_INNER_PAD);
    let child_pad = " ".repeat(PANEL_INNER_PAD + NAV_CHILD_INDENT);
    let mut lines = Vec::new();
    for (idx, row) in rows.iter().enumerate() {
        let prefix = if show_nav_marker && idx == selected_idx {
            ">"
        } else {
            " "
        };
        match row.kind {
            NavRowKind::Domain(domain) => {
                let title = match domain {
                    TuiNavDomain::System => "System",
                    TuiNavDomain::Contacts => "Contacts",
                    TuiNavDomain::Messages => "Messages",
                };
                lines.push(format!("{}{}{}", prefix, base_pad, title));
            }
            NavRowKind::SystemAccount => lines.push(format!("{}{}Account", prefix, child_pad)),
            NavRowKind::SystemRelay => lines.push(format!("{}{}Relay", prefix, child_pad)),
            NavRowKind::SystemSettings => lines.push(format!("{}{}Settings", prefix, child_pad)),
            NavRowKind::SystemCmdResults => lines.push(format!("{}{}Results", prefix, child_pad)),
            NavRowKind::Header(pane) => {
                let header = match pane {
                    TuiInspectorPane::Events => format!("{}{}Messages", prefix, base_pad),
                    TuiInspectorPane::Files => format!("{}{}Files", prefix, base_pad),
                    TuiInspectorPane::Activity => format!("{}{}Activity", prefix, base_pad),
                    TuiInspectorPane::Status => format!("{}{}Status", prefix, base_pad),
                    TuiInspectorPane::Account => format!("{}{}Account", prefix, base_pad),
                    TuiInspectorPane::Relay => format!("{}{}Relay", prefix, base_pad),
                    TuiInspectorPane::CmdResults => format!("{}{}Results", prefix, base_pad),
                    TuiInspectorPane::Session => format!("{}{}Keys", prefix, base_pad),
                    TuiInspectorPane::Contacts => format!("{}{}Contacts", prefix, base_pad),
                    TuiInspectorPane::Settings => format!("{}{}Settings", prefix, base_pad),
                    TuiInspectorPane::Lock => format!("{}{}Lock", prefix, base_pad),
                    TuiInspectorPane::Help => format!("{}{}Help", prefix, base_pad),
                    TuiInspectorPane::About => format!("{}{}About", prefix, base_pad),
                    TuiInspectorPane::Legal => format!("{}{}Legal", prefix, base_pad),
                };
                lines.push(header);
            }
            NavRowKind::Conversation(item_idx) => {
                let labels = state.conversation_labels();
                if let Some(peer) = labels.get(item_idx) {
                    lines.push(format!("{}{}{}", prefix, child_pad, peer));
                }
            }
            NavRowKind::Contact(item_idx) => {
                if let Some(peer) = state.contacts.get(item_idx) {
                    lines.push(format!("{}{}{}", prefix, child_pad, peer));
                }
            }
            NavRowKind::Unlock => lines.push(format!("{}{}Unlock", prefix, base_pad)),
            NavRowKind::Exit => lines.push(format!("{}{}Exit", prefix, base_pad)),
        }
    }
    let selected_markers = if rows.is_empty() || !show_nav_marker {
        0
    } else {
        1
    };
    let selected_idx_s = selected_idx.to_string();
    let selected_label = rows
        .get(selected_idx)
        .map(|row| state.nav_row_label(row))
        .unwrap_or_else(|| "none".to_string());
    let header_text = "[ QSC ]";
    let header_left_padding = 1usize;
    let header_left_padding_s = header_left_padding.to_string();
    emit_marker(
        "tui_nav_render",
        None,
        &[
            (
                "selected_markers",
                if selected_markers == 1 { "1" } else { "0" },
            ),
            ("selected_index", selected_idx_s.as_str()),
            ("selected_label", selected_label.as_str()),
            ("header", header_text),
            ("header_left_padding", header_left_padding_s.as_str()),
            ("counters", "none"),
        ],
    );
    lines.insert(
        0,
        format!("{}{}", " ".repeat(header_left_padding), header_text),
    );
    lines.insert(1, String::new());
    let panel = Paragraph::new(lines.join("\n"));
    f.render_widget(panel, area);
}

fn tui_vault_present() -> bool {
    config_dir()
        .ok()
        .map(|(dir, _)| dir.join("vault.qsv").exists())
        .unwrap_or(false)
}

fn tui_relay_config(cfg: &TuiConfig) -> Option<TuiRelayConfig> {
    let relay = cfg.relay.clone()?;
    if normalize_relay_endpoint(relay.as_str()).is_err() {
        return None;
    }
    Some(TuiRelayConfig {
        relay,
        seed: cfg.seed,
        scenario: cfg.scenario.clone(),
    })
}

struct TuiState {
    contacts: Vec<String>,
    conversations: BTreeMap<String, VecDeque<String>>,
    unread_counts: BTreeMap<String, usize>,
    visible_counts: BTreeMap<String, usize>,
    files: Vec<TuiFileItem>,
    file_selected: usize,
    file_multi_selected: BTreeSet<String>,
    file_unseen_updates: usize,
    activity_visible_count: usize,
    activity_unseen_updates: usize,
    events: VecDeque<String>,
    status: TuiStatus<'static>,
    session: TuiSession<'static>,
    send_lifecycle: String,
    qsp_status: String,
    envelope: String,
    last_payload_len: usize,
    event_seq: u64,
    relay: Option<TuiRelayConfig>,
    send_seq: u64,
    mode: TuiMode,
    help_selected: usize,
    focus_scroll: usize,
    contacts_selected: usize,
    conversation_selected: usize,
    inspector: TuiInspectorPane,
    home_focus: TuiHomeFocus,
    nav_selected: usize,
    vault_locked: bool,
    vault_present: bool,
    vault_session: Option<vault::VaultSession>,
    autolock_timeout_ms: u64,
    autolock_last_activity_ms: u64,
    poll_mode: TuiPollMode,
    poll_interval_seconds: u64,
    receipt_policy: ReceiptPolicy,
    trust_onboarding_mode: TrustOnboardingMode,
    poll_next_due_ms: Option<u64>,
    headless_clock_ms: u64,
    clear_screen_pending: bool,
    force_full_redraw: bool,
    cmd_input: String,
    locked_flow: LockedFlow,
    locked_error: Option<String>,
    account_destroy_flow: AccountDestroyFlow,
    account_destroy_error: Option<String>,
    command_error: Option<String>,
    command_feedback: Option<String>,
    status_last_command_result: Option<String>,
    cmd_results: VecDeque<String>,
    active_command_label: Option<String>,
    active_command_result_recorded: bool,
    contacts_records: BTreeMap<String, ContactRecord>,
    account_alias_cache: String,
    account_verification_code_cache: String,
    account_storage_safety_cache: String,
    account_cache_last_refresh_ms: u64,
    relay_endpoint_cache: Option<String>,
    relay_endpoint_hash_cache: Option<String>,
    relay_token_set_cache: bool,
    relay_token_file_cache: Option<String>,
    relay_token_file_hash_cache: Option<String>,
    relay_inbox_token_hash_cache: Option<String>,
    relay_inbox_token_set_cache: bool,
    relay_last_test_result: String,
    relay_test_task: Option<mpsc::Receiver<RelayTestOutcome>>,
    unlock_attempt_limit: Option<u32>,
    failed_unlock_attempts: u32,
    main_scroll_offsets: BTreeMap<&'static str, usize>,
    main_scroll_max_current: usize,
    main_view_rows_current: usize,
    needs_redraw: bool,
}

impl TuiState {
    fn new(cfg: TuiConfig) -> Self {
        let vault_present = tui_vault_present();
        let contacts = vec!["peer-0".to_string()];
        let conversations = BTreeMap::new();
        let unread_counts = BTreeMap::new();
        let visible_counts = BTreeMap::new();
        let mut events = VecDeque::new();
        let fingerprint = compute_local_fingerprint();
        let peer_fp = compute_peer_fingerprint("peer-0");
        let qsp_status = qsp_status_string("peer-0");
        let envelope = compute_envelope_status(0);
        let send_lifecycle = "idle".to_string();
        let locked_s = if vault_unlocked() {
            "UNLOCKED"
        } else {
            "LOCKED"
        };
        let status = TuiStatus {
            fingerprint: Box::leak(fingerprint.clone().into_boxed_str()),
            peer_fp: Box::leak(peer_fp.clone().into_boxed_str()),
            qsp: Box::leak(qsp_status.clone().into_boxed_str()),
            envelope: Box::leak(envelope.clone().into_boxed_str()),
            send_lifecycle: Box::leak(send_lifecycle.clone().into_boxed_str()),
            locked: Box::leak(locked_s.to_string().into_boxed_str()),
        };
        let session = TuiSession {
            peer_label: "peer-0",
            verified: false,
            sent_count: 0,
            recv_count: 0,
        };
        let relay = tui_relay_config(&cfg);
        if let Some(r) = relay.as_ref() {
            let seed_s = r.seed.to_string();
            emit_marker(
                "tui_transport",
                None,
                &[
                    ("transport", "relay"),
                    ("relay", r.relay.as_str()),
                    ("seed", seed_s.as_str()),
                    ("scenario", r.scenario.as_str()),
                ],
            );
            events.push_back(format!(
                "transport relay {} seed={} scenario={}",
                r.relay, r.seed, r.scenario
            ));
        }
        let mut state = Self {
            contacts,
            conversations,
            unread_counts,
            visible_counts,
            files: load_tui_files_snapshot(),
            file_selected: 0,
            file_multi_selected: BTreeSet::new(),
            file_unseen_updates: 0,
            activity_visible_count: 0,
            activity_unseen_updates: 0,
            events,
            status,
            session,
            send_lifecycle,
            qsp_status,
            envelope,
            last_payload_len: 0,
            event_seq: 0,
            relay,
            send_seq: 0,
            mode: TuiMode::Normal,
            help_selected: 0,
            focus_scroll: 0,
            contacts_selected: 0,
            conversation_selected: 0,
            inspector: TuiInspectorPane::Status,
            home_focus: TuiHomeFocus::Nav,
            nav_selected: 0,
            vault_locked: !vault_unlocked(),
            vault_present,
            vault_session: None,
            autolock_timeout_ms: TUI_AUTOLOCK_DEFAULT_MINUTES.saturating_mul(60_000),
            autolock_last_activity_ms: 0,
            poll_mode: TuiPollMode::Adaptive,
            poll_interval_seconds: TUI_POLL_DEFAULT_INTERVAL_SECONDS,
            receipt_policy: ReceiptPolicy::default(),
            trust_onboarding_mode: TrustOnboardingMode::Balanced,
            poll_next_due_ms: None,
            headless_clock_ms: 0,
            clear_screen_pending: false,
            force_full_redraw: false,
            cmd_input: String::new(),
            locked_flow: LockedFlow::None,
            locked_error: None,
            account_destroy_flow: AccountDestroyFlow::None,
            account_destroy_error: None,
            command_error: None,
            command_feedback: None,
            status_last_command_result: None,
            cmd_results: VecDeque::new(),
            active_command_label: None,
            active_command_result_recorded: false,
            contacts_records: BTreeMap::new(),
            account_alias_cache: "unset".to_string(),
            account_verification_code_cache: "none".to_string(),
            account_storage_safety_cache: "unknown".to_string(),
            account_cache_last_refresh_ms: 0,
            relay_endpoint_cache: None,
            relay_endpoint_hash_cache: None,
            relay_token_set_cache: false,
            relay_token_file_cache: cfg.token_file.as_ref().map(|p| p.display().to_string()),
            relay_token_file_hash_cache: cfg
                .token_file
                .as_ref()
                .map(|p| relay_token_file_hash8(p.display().to_string().as_str())),
            relay_inbox_token_hash_cache: None,
            relay_inbox_token_set_cache: false,
            relay_last_test_result: "none".to_string(),
            relay_test_task: None,
            unlock_attempt_limit: None,
            failed_unlock_attempts: 0,
            main_scroll_offsets: BTreeMap::new(),
            main_scroll_max_current: 0,
            main_view_rows_current: 1,
            needs_redraw: true,
        };
        if env_bool("QSC_TUI_TEST_UNLOCK") {
            state.vault_locked = false;
            state.vault_present = true;
            state.status.locked = "UNLOCKED";
            set_vault_unlocked(true);
        }
        state.reload_unlock_security_state();
        if state.vault_locked {
            state.inspector = TuiInspectorPane::Lock;
            state.nav_selected = 0;
        } else {
            let _ = state.open_vault_session(None);
            state.reload_account_settings_from_vault();
            state.refresh_identity_status();
            state.sync_nav_to_inspector_header();
        }
        if let Some(path) = cfg.token_file.as_ref() {
            let canonical = path
                .canonicalize()
                .unwrap_or(path.to_path_buf())
                .to_string_lossy()
                .to_string();
            state.relay_token_file_hash_cache = Some(relay_token_file_hash8(canonical.as_str()));
            state.relay_token_file_cache = Some(canonical.clone());
            if !state.is_locked() {
                let _ = state
                    .persist_account_secret(TUI_RELAY_TOKEN_FILE_SECRET_KEY, canonical.as_str());
            }
        }
        state.emit_setup_required_marker_if_needed();
        state
    }

    fn is_locked(&self) -> bool {
        self.vault_locked
    }

    fn has_vault(&self) -> bool {
        self.vault_present
    }

    fn mark_vault_present(&mut self) {
        self.vault_present = true;
    }

    fn mark_vault_absent(&mut self) {
        self.vault_present = false;
        vault::set_process_passphrase(None);
    }

    fn reload_unlock_security_state(&mut self) {
        match vault_security_state_load() {
            Ok(state) => {
                self.unlock_attempt_limit = state.attempt_limit;
                self.failed_unlock_attempts = state.failed_unlocks;
            }
            Err(_) => {
                self.unlock_attempt_limit = None;
                self.failed_unlock_attempts = 0;
            }
        }
    }

    fn persist_unlock_security_state(&self) -> Result<(), &'static str> {
        let state = VaultSecurityState {
            attempt_limit: self.unlock_attempt_limit,
            failed_unlocks: self.failed_unlock_attempts,
        };
        vault_security_state_store(&state).map_err(|_| "vault_attempt_limit_io")
    }

    fn set_unlock_attempt_limit(&mut self, limit: Option<u32>) -> Result<(), &'static str> {
        if let Some(value) = limit {
            if !(VAULT_ATTEMPT_LIMIT_MIN..=VAULT_ATTEMPT_LIMIT_MAX).contains(&value) {
                return Err("vault_attempt_limit_invalid");
            }
        }
        self.unlock_attempt_limit = limit;
        self.failed_unlock_attempts = 0;
        self.persist_unlock_security_state()?;
        self.request_redraw();
        Ok(())
    }

    fn reset_unlock_failure_counter(&mut self) {
        if self.failed_unlock_attempts == 0 {
            return;
        }
        self.failed_unlock_attempts = 0;
        let _ = self.persist_unlock_security_state();
    }

    fn wipe_after_failed_unlock_limit(&mut self) -> Result<(), &'static str> {
        wipe_vault_file_best_effort().map_err(|_| "vault_wipe_failed")?;
        let _ = vault_security_state_clear_files();
        wipe_account_local_state_best_effort();
        self.close_vault_session();
        self.mark_vault_absent();
        self.unlock_attempt_limit = None;
        self.failed_unlock_attempts = 0;
        self.apply_default_account_settings();
        self.cmd_results.clear();
        self.status_last_command_result = None;
        self.command_feedback = None;
        self.locked_flow = LockedFlow::None;
        self.locked_clear_error();
        self.set_locked_state(true, "unlock_attempt_limit_wipe");
        self.push_cmd_result(
            "unlock",
            false,
            format!(
                "vault wiped after failed unlock attempts; run /init to rebuild local state ({})",
                QSC_ERR_VAULT_WIPED_AFTER_FAILED_UNLOCKS
            ),
        );
        emit_marker(
            "tui_unlock",
            Some(QSC_ERR_VAULT_WIPED_AFTER_FAILED_UNLOCKS),
            &[("ok", "false"), ("reason", "failed_unlock_limit_reached")],
        );
        Ok(())
    }

    fn record_unlock_failure_and_maybe_wipe(&mut self) -> UnlockAttemptOutcome {
        let Some(limit) = self.unlock_attempt_limit else {
            return UnlockAttemptOutcome::Rejected;
        };
        self.failed_unlock_attempts = self.failed_unlock_attempts.saturating_add(1);
        if self.persist_unlock_security_state().is_err() {
            return UnlockAttemptOutcome::Rejected;
        }
        if self.failed_unlock_attempts >= limit && self.wipe_after_failed_unlock_limit().is_ok() {
            return UnlockAttemptOutcome::Wiped;
        }
        UnlockAttemptOutcome::Rejected
    }

    fn unlock_with_policy(&mut self, passphrase: &str) -> UnlockAttemptOutcome {
        let unlocked = vault::unlock_with_passphrase(passphrase).is_ok()
            && self.open_vault_session(Some(passphrase)).is_ok();
        if unlocked {
            vault::set_process_passphrase(Some(passphrase));
            self.reset_unlock_failure_counter();
            return UnlockAttemptOutcome::Unlocked;
        }
        self.record_unlock_failure_and_maybe_wipe()
    }

    fn cmd_input_clear(&mut self) {
        self.cmd_input.clear();
    }

    fn cmd_input_push(&mut self, ch: char) {
        self.cmd_input.push(ch);
    }

    fn cmd_input_pop(&mut self) {
        self.cmd_input.pop();
    }

    fn locked_flow_name(&self) -> &'static str {
        match self.locked_flow {
            LockedFlow::None => "none",
            LockedFlow::UnlockPassphrase => "unlock_passphrase",
            LockedFlow::InitAlias => "init_alias",
            LockedFlow::InitPassphrase { .. } => "init_passphrase",
            LockedFlow::InitConfirm { .. } => "init_confirm",
            LockedFlow::InitDecision { .. } => "init_decision",
        }
    }

    fn locked_wizard_step_label(&self) -> Option<&'static str> {
        match self.locked_flow {
            LockedFlow::None => None,
            LockedFlow::UnlockPassphrase => Some("Passphrase"),
            LockedFlow::InitAlias => Some("Alias"),
            LockedFlow::InitPassphrase { .. } => Some("Passphrase"),
            LockedFlow::InitConfirm { .. } => Some("Confirm"),
            LockedFlow::InitDecision { .. } => Some("Confirm (I AGREE/N)"),
        }
    }

    fn account_destroy_step_label(&self) -> Option<&'static str> {
        match self.account_destroy_flow {
            AccountDestroyFlow::None => None,
            AccountDestroyFlow::Passphrase => Some("Passphrase"),
            AccountDestroyFlow::ConfirmDecision { .. } => Some("Confirm (Y/N)"),
        }
    }

    fn account_destroy_set_error(&mut self, message: impl Into<String>) {
        self.account_destroy_error = Some(message.into());
    }

    fn account_destroy_clear_error(&mut self) {
        self.account_destroy_error = None;
    }

    fn account_destroy_active(&self) -> bool {
        !matches!(self.account_destroy_flow, AccountDestroyFlow::None)
    }

    fn locked_set_error(&mut self, message: impl Into<String>) {
        self.locked_error = Some(message.into());
    }

    fn locked_clear_error(&mut self) {
        self.locked_error = None;
    }

    fn set_command_error(&mut self, message: impl Into<String>) {
        let message = message.into();
        self.command_error = Some(message.clone());
        self.command_feedback = None;
        if !self.active_command_result_recorded {
            if let Some(command) = self.active_command_label.clone() {
                self.push_cmd_result(command.as_str(), false, message);
            }
        }
        self.route_show_to_system_nav(TuiInspectorPane::CmdResults);
        self.request_redraw();
    }

    fn clear_command_error(&mut self) {
        self.command_error = None;
        self.request_redraw();
    }

    fn set_command_feedback(&mut self, message: impl Into<String>) {
        let message = message.into();
        let marker_msg = message.replace(' ', "_");
        emit_marker(
            "tui_cmd_feedback",
            None,
            &[("kind", "ok"), ("message", marker_msg.as_str())],
        );
        self.command_feedback = Some(message);
        self.request_redraw();
    }

    fn clear_command_feedback(&mut self) {
        self.command_feedback = None;
        self.request_redraw();
    }

    fn request_redraw(&mut self) {
        self.needs_redraw = true;
    }

    fn refresh_account_cache(&mut self, now_ms: u64, force: bool) -> bool {
        let alias = self
            .read_account_secret("profile_alias")
            .filter(|value| !value.is_empty())
            .unwrap_or_else(|| "unset".to_string());
        let verification_code = self
            .read_account_secret(ACCOUNT_VERIFICATION_SEED_SECRET_KEY)
            .map(|seed| format_verification_code_from_fingerprint(seed.as_str()))
            .unwrap_or_else(|| "none".to_string());
        let storage_safety = if force || self.account_storage_safety_cache == "unknown" {
            account_storage_safety_status()
        } else {
            self.account_storage_safety_cache.clone()
        };
        let changed = self.account_alias_cache != alias
            || self.account_verification_code_cache != verification_code
            || self.account_storage_safety_cache != storage_safety;
        self.account_alias_cache = alias;
        self.account_verification_code_cache = verification_code;
        self.account_storage_safety_cache = storage_safety;
        self.account_cache_last_refresh_ms = now_ms;
        emit_marker(
            "tui_account_cache_refresh",
            None,
            &[
                ("changed", if changed { "true" } else { "false" }),
                ("force", if force { "true" } else { "false" }),
            ],
        );
        changed
    }

    fn set_status_last_command_result(&mut self, message: impl Into<String>) {
        let message = message.into();
        self.status_last_command_result = Some(message.clone());
        let _ = self.persist_account_secret(TUI_LAST_COMMAND_RESULT_SECRET_KEY, message.as_str());
        self.request_redraw();
    }

    fn status_last_command_result_text(&self) -> &str {
        self.status_last_command_result.as_deref().unwrap_or("none")
    }

    fn push_cmd_result(&mut self, command: &str, ok: bool, message: impl Into<String>) {
        let status = if ok { "ok" } else { "err" };
        let message = message.into();
        let line = format!("[{}] /{} {}", status, command, message);
        let cmd_marker = command.replace(' ', "_");
        emit_marker(
            "tui_cmd_result",
            None,
            &[("kind", status), ("command", cmd_marker.as_str())],
        );
        self.cmd_results.push_back(line);
        self.status_last_command_result = self.cmd_results.back().cloned();
        if let Some(last) = self.status_last_command_result.clone() {
            let _ = self.persist_account_secret(TUI_LAST_COMMAND_RESULT_SECRET_KEY, last.as_str());
        }
        self.active_command_result_recorded = true;
        while self.cmd_results.len() > 50 {
            self.cmd_results.pop_front();
        }
        self.request_redraw();
    }

    fn begin_command_tracking(&mut self, command: impl Into<String>) {
        self.active_command_label = Some(command.into());
        self.active_command_result_recorded = false;
    }

    fn end_command_tracking(&mut self) {
        self.active_command_label = None;
        self.active_command_result_recorded = false;
    }

    fn locked_cmd_masked(&self) -> bool {
        matches!(
            self.locked_flow,
            LockedFlow::UnlockPassphrase
                | LockedFlow::InitPassphrase { .. }
                | LockedFlow::InitConfirm { .. }
        )
    }

    fn account_destroy_cmd_masked(&self) -> bool {
        matches!(self.account_destroy_flow, AccountDestroyFlow::Passphrase)
    }

    fn cmd_display_value(&self) -> String {
        if self.locked_cmd_masked() || self.account_destroy_cmd_masked() {
            "•".repeat(self.cmd_input.chars().count())
        } else {
            self.cmd_input.clone()
        }
    }

    fn cmd_bar_text(&self) -> String {
        let content = if self.is_locked() {
            if let Some(label) = self.locked_wizard_step_label() {
                if self.home_focus == TuiHomeFocus::Command {
                    format!("{}: {}{}", label, self.cmd_display_value(), '█')
                } else {
                    format!("{}:", label)
                }
            } else if self.home_focus == TuiHomeFocus::Command {
                format!("Cmd: {}{}", self.cmd_display_value(), '█')
            } else {
                "Cmd:".to_string()
            }
        } else if let Some(label) = self.account_destroy_step_label() {
            if self.home_focus == TuiHomeFocus::Command {
                format!("{}: {}{}", label, self.cmd_display_value(), '█')
            } else {
                format!("{}:", label)
            }
        } else if self.cmd_input.is_empty() {
            if let Some(msg) = self.command_feedback.as_ref() {
                msg.clone()
            } else if self.home_focus == TuiHomeFocus::Command {
                format!("Cmd: {}{}", self.cmd_display_value(), '█')
            } else {
                "Cmd: /help".to_string()
            }
        } else if self.home_focus == TuiHomeFocus::Command {
            format!("Cmd: {}{}", self.cmd_display_value(), '█')
        } else {
            "Cmd: /help".to_string()
        };
        format!("Focus: {} | {}", self.home_focus_label_token(), content)
    }

    fn accent_color_enabled(&self) -> bool {
        tui_color_enabled()
    }

    fn cmd_bar_style(&self, text: &str) -> Style {
        if !self.accent_color_enabled() {
            return Style::default();
        }
        if text.starts_with("ok:") {
            Style::default().fg(Color::Green)
        } else if text.starts_with("error:") {
            Style::default().fg(Color::Red)
        } else {
            Style::default()
        }
    }

    fn locked_main_lines(&self) -> Vec<String> {
        match &self.locked_flow {
            LockedFlow::None => {
                let mut lines = if self.has_vault() {
                    vec![
                        "Locked: unlock required".to_string(),
                        "Use /unlock to open the local vault.".to_string(),
                    ]
                } else {
                    vec![
                        "No vault found - run /init".to_string(),
                        "This creates the local encrypted state qsc needs before use.".to_string(),
                    ]
                };
                if let Some(err) = self.locked_error.as_ref() {
                    lines.push(format!("error: {}", err));
                }
                lines
            }
            LockedFlow::UnlockPassphrase => {
                let mut lines = vec![
                    "Unlock".to_string(),
                    String::new(),
                    format!("Passphrase: {}", self.cmd_display_value()),
                ];
                if let Some(err) = self.locked_error.as_ref() {
                    lines.push(format!("error: {}", err));
                }
                lines
            }
            LockedFlow::InitAlias
            | LockedFlow::InitPassphrase { .. }
            | LockedFlow::InitConfirm { .. }
            | LockedFlow::InitDecision { .. } => {
                let mut lines = vec![
                    "Initialize Vault".to_string(),
                    String::new(),
                    "This will create an encrypted vault to store your identity, contacts, messages, and files.".to_string(),
                    "Choose a strong passphrase — there is no recovery if it’s lost.".to_string(),
                    String::new(),
                ];
                let alias_summary = match &self.locked_flow {
                    LockedFlow::InitPassphrase { alias }
                    | LockedFlow::InitConfirm { alias, .. }
                    | LockedFlow::InitDecision { alias, .. } => alias.clone(),
                    LockedFlow::InitAlias => String::new(),
                    _ => String::new(),
                };
                if !alias_summary.is_empty() {
                    lines.push(format!("Alias: {}", alias_summary));
                }
                let passphrase_ready = matches!(
                    self.locked_flow,
                    LockedFlow::InitConfirm { .. } | LockedFlow::InitDecision { .. }
                );
                if passphrase_ready {
                    lines.push("Passphrase: set (hidden)".to_string());
                }
                if !alias_summary.is_empty() || passphrase_ready {
                    lines.push(String::new());
                }
                let input_label = match self.locked_flow {
                    LockedFlow::InitAlias => "Alias",
                    LockedFlow::InitPassphrase { .. } => "Passphrase",
                    LockedFlow::InitConfirm { .. } => "Confirm",
                    LockedFlow::InitDecision { .. } => "Confirm (I AGREE/N)",
                    _ => "Input",
                };
                if matches!(self.locked_flow, LockedFlow::InitDecision { .. }) {
                    lines.push("Legal acceptance required: type I AGREE to continue.".to_string());
                    lines.push(
                        "Type N to cancel. See full terms in System -> Legal after unlock."
                            .to_string(),
                    );
                    lines.push(String::new());
                }
                lines.push(format!("{}: {}", input_label, self.cmd_display_value()));
                if let Some(err) = self.locked_error.as_ref() {
                    lines.push(format!("error: {}", err));
                }
                lines
            }
        }
    }

    fn locked_main_body(&self) -> String {
        self.locked_main_lines().join("\n")
    }

    fn start_unlock_prompt(&mut self) {
        self.home_focus = TuiHomeFocus::Command;
        self.locked_flow = LockedFlow::UnlockPassphrase;
        self.cmd_input_clear();
        self.locked_clear_error();
        emit_marker("tui_unlock_prompt", None, &[("step", "passphrase")]);
        emit_marker("tui_focus_home", None, &[("pane", self.home_focus_name())]);
    }

    fn start_init_prompt(&mut self) {
        self.home_focus = TuiHomeFocus::Command;
        self.locked_flow = LockedFlow::InitAlias;
        self.cmd_input_clear();
        self.locked_clear_error();
        emit_marker(
            "tui_init_warning",
            None,
            &[("no_recovery", "true"), ("confirm_prompt", "Y_or_N")],
        );
        emit_marker("tui_init_wizard", None, &[("step", "alias")]);
        emit_marker("tui_focus_home", None, &[("pane", self.home_focus_name())]);
    }

    fn start_account_destroy_prompt(&mut self) {
        self.inspector = TuiInspectorPane::Account;
        self.sync_nav_to_inspector_header();
        self.home_focus = TuiHomeFocus::Command;
        self.account_destroy_flow = AccountDestroyFlow::Passphrase;
        self.cmd_input_clear();
        self.account_destroy_clear_error();
        emit_marker("tui_account_destroy", None, &[("step", "passphrase")]);
        emit_marker("tui_focus_home", None, &[("pane", self.home_focus_name())]);
    }

    fn cancel_account_destroy_prompt(&mut self) {
        self.account_destroy_flow = AccountDestroyFlow::None;
        self.account_destroy_clear_error();
        self.cmd_input_clear();
        self.home_focus = TuiHomeFocus::Nav;
        emit_marker("tui_account_destroy", None, &[("step", "cancel")]);
        emit_marker("tui_focus_home", None, &[("pane", self.home_focus_name())]);
    }

    fn locked_focus_toggle(&mut self) {
        self.home_focus = if self.home_focus == TuiHomeFocus::Command {
            TuiHomeFocus::Nav
        } else {
            TuiHomeFocus::Command
        };
        emit_marker("tui_focus_home", None, &[("pane", self.home_focus_name())]);
    }

    fn autolock_minutes(&self) -> u64 {
        let minutes = self.autolock_timeout_ms / 60_000;
        minutes.clamp(TUI_AUTOLOCK_MIN_MINUTES, TUI_AUTOLOCK_MAX_MINUTES)
    }

    fn open_vault_session(&mut self, passphrase: Option<&str>) -> Result<(), &'static str> {
        let session = match passphrase {
            Some(value) => vault::open_session_with_passphrase(value),
            None => vault::open_session(None),
        }?;
        self.vault_session = Some(session);
        Ok(())
    }

    fn close_vault_session(&mut self) {
        self.vault_session = None;
    }

    fn persist_account_secret(&mut self, key: &str, value: &str) -> Result<(), &'static str> {
        if let Some(session) = self.vault_session.as_mut() {
            return vault::session_set(session, key, value);
        }
        Err("vault_locked")
    }

    fn read_account_secret(&self, key: &str) -> Option<String> {
        self.vault_session
            .as_ref()
            .and_then(|session| vault::session_get(session, key).ok().flatten())
    }

    fn relay_endpoint_redacted(&self) -> String {
        match self.relay_endpoint_hash_cache.as_ref() {
            Some(hash) => format!("set (hash: {})", hash),
            None => "unset".to_string(),
        }
    }

    fn relay_auth_label(&self) -> &'static str {
        if self.relay_token_set_cache {
            "bearer token (set)"
        } else if self.relay_token_file_cache.is_some() {
            "bearer token file (set)"
        } else {
            "none (optional bearer token)"
        }
    }

    fn relay_token_file_redacted(&self) -> String {
        match self.relay_token_file_hash_cache.as_ref() {
            Some(hash) => format!("set (hash: {})", hash),
            None => "unset".to_string(),
        }
    }

    fn relay_token_file_status(&self) -> (&'static str, &'static str) {
        let Some(path) = self.relay_token_file_cache.as_ref() else {
            return ("unset", "n/a");
        };
        let p = Path::new(path.as_str());
        let md = match fs::metadata(p) {
            Ok(v) => v,
            Err(_) => return ("missing", "n/a"),
        };
        if !md.is_file() {
            return ("invalid", "n/a");
        }
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mode = md.permissions().mode() & 0o777;
            if mode == 0o600 {
                ("exists", "0600_ok")
            } else {
                ("exists", "too_open")
            }
        }
        #[cfg(not(unix))]
        {
            ("exists", "platform_default")
        }
    }

    fn relay_inbox_token_redacted(&self) -> String {
        match self.relay_inbox_token_hash_cache.as_ref() {
            Some(hash) => format!("set (hash: {})", hash),
            None => "unset".to_string(),
        }
    }

    fn relay_setup_status(&self) -> (&'static str, &'static str) {
        let relay_state = if self.relay_endpoint_cache.is_some() {
            "set"
        } else {
            "missing"
        };
        let auth_state = if self.relay_token_set_cache {
            "set"
        } else {
            let (file_state, file_perms) = self.relay_token_file_status();
            if file_state == "unset" {
                "missing"
            } else if file_state == "missing" {
                "token_file_missing"
            } else if file_perms == "too_open" {
                "bad_perms"
            } else if file_state == "exists" {
                "set"
            } else {
                "missing"
            }
        };
        (relay_state, auth_state)
    }

    fn emit_setup_required_marker_if_needed(&self) {
        let (relay_state, auth_state) = self.relay_setup_status();
        if relay_state == "set" && auth_state == "set" {
            return;
        }
        emit_tui_named_marker(
            "QSC_TUI_SETUP_REQUIRED",
            &[("relay", relay_state), ("auth", auth_state)],
        );
        emit_marker(
            "tui_setup_required",
            None,
            &[("relay", relay_state), ("auth", auth_state)],
        );
    }

    fn set_relay_endpoint(&mut self, value: &str) -> Result<(), &'static str> {
        let endpoint = normalize_relay_endpoint(value)?;
        self.persist_account_secret(TUI_RELAY_ENDPOINT_SECRET_KEY, endpoint.as_str())
            .map_err(|_| "relay_config_unavailable")?;
        self.relay_endpoint_hash_cache = Some(relay_endpoint_hash8(endpoint.as_str()));
        self.relay_endpoint_cache = Some(endpoint.clone());
        self.relay = Some(TuiRelayConfig {
            relay: endpoint,
            seed: 0,
            scenario: "default".to_string(),
        });
        self.request_redraw();
        Ok(())
    }

    fn set_relay_token(&mut self, value: &str) -> Result<(), &'static str> {
        let token = value.trim();
        if token.is_empty() {
            return Err("relay_token_missing");
        }
        self.persist_account_secret(TUI_RELAY_TOKEN_SECRET_KEY, token)
            .map_err(|_| "relay_config_unavailable")?;
        self.relay_token_set_cache = true;
        self.request_redraw();
        Ok(())
    }

    fn set_relay_token_file(&mut self, value: &str) -> Result<(), &'static str> {
        let raw = value.trim();
        if raw.is_empty() {
            return Err("relay_token_file_missing");
        }
        let path = PathBuf::from(raw);
        let canonical = path
            .canonicalize()
            .unwrap_or(path)
            .to_string_lossy()
            .to_string();
        self.persist_account_secret(TUI_RELAY_TOKEN_FILE_SECRET_KEY, canonical.as_str())
            .map_err(|_| "relay_config_unavailable")?;
        self.relay_token_file_hash_cache = Some(relay_token_file_hash8(canonical.as_str()));
        self.relay_token_file_cache = Some(canonical);
        self.request_redraw();
        Ok(())
    }

    fn set_relay_inbox_token(&mut self, value: &str) -> Result<(), &'static str> {
        let token = normalize_route_token(value)?;
        self.persist_account_secret(TUI_RELAY_INBOX_TOKEN_SECRET_KEY, token.as_str())
            .map_err(|_| "relay_config_unavailable")?;
        self.relay_inbox_token_set_cache = true;
        self.relay_inbox_token_hash_cache = Some(route_token_hash8(token.as_str()));
        self.request_redraw();
        Ok(())
    }

    fn clear_relay_inbox_token(&mut self) -> Result<(), &'static str> {
        self.persist_account_secret(TUI_RELAY_INBOX_TOKEN_SECRET_KEY, "")
            .map_err(|_| "relay_config_unavailable")?;
        self.relay_inbox_token_set_cache = false;
        self.relay_inbox_token_hash_cache = None;
        self.request_redraw();
        Ok(())
    }

    fn clear_relay_config(&mut self) -> Result<(), &'static str> {
        self.persist_account_secret(TUI_RELAY_ENDPOINT_SECRET_KEY, "")
            .map_err(|_| "relay_config_unavailable")?;
        self.persist_account_secret(TUI_RELAY_TOKEN_SECRET_KEY, "")
            .map_err(|_| "relay_config_unavailable")?;
        self.persist_account_secret(TUI_RELAY_TOKEN_FILE_SECRET_KEY, "")
            .map_err(|_| "relay_config_unavailable")?;
        self.persist_account_secret(TUI_RELAY_INBOX_TOKEN_SECRET_KEY, "")
            .map_err(|_| "relay_config_unavailable")?;
        self.relay_endpoint_cache = None;
        self.relay_endpoint_hash_cache = None;
        self.relay_token_set_cache = false;
        self.relay_token_file_cache = None;
        self.relay_token_file_hash_cache = None;
        self.relay_inbox_token_hash_cache = None;
        self.relay_inbox_token_set_cache = false;
        self.relay_last_test_result = "none".to_string();
        self.relay_test_task = None;
        self.relay = None;
        self.request_redraw();
        Ok(())
    }

    fn effective_relay_config(&self) -> Option<TuiRelayConfig> {
        if let Some(relay) = self.relay.as_ref() {
            return Some(relay.clone());
        }
        self.relay_endpoint_cache
            .as_ref()
            .map(|endpoint| TuiRelayConfig {
                relay: endpoint.clone(),
                seed: 0,
                scenario: "default".to_string(),
            })
    }

    fn finish_relay_test_task(&mut self, outcome: RelayTestOutcome) {
        self.relay_test_task = None;
        if outcome.ok {
            self.relay_last_test_result = format!("ok: {}", outcome.message);
            self.set_status_last_command_result(format!("relay test ok ({})", outcome.message));
            self.push_cmd_result("relay test", true, outcome.message);
            self.set_command_feedback("ok: relay test succeeded");
        } else {
            let reason = relay_user_reason_from_code(outcome.code);
            self.relay_last_test_result = format!("err: {}", reason);
            self.set_status_last_command_result(format!("relay test err ({})", reason));
            self.push_cmd_result("relay test", false, reason);
            self.command_error = Some(format!("relay: {}", reason));
            self.command_feedback = None;
            self.route_show_to_system_nav(TuiInspectorPane::CmdResults);
        }
        emit_tui_relay_test_event(if outcome.ok { "ok" } else { "err" }, outcome.code);
        emit_marker(
            "tui_relay_test_done",
            None,
            &[
                ("ok", if outcome.ok { "true" } else { "false" }),
                ("reason", outcome.code),
            ],
        );
        self.request_redraw();
    }

    fn poll_relay_test_task(&mut self) {
        let outcome = self
            .relay_test_task
            .as_ref()
            .and_then(|rx| rx.try_recv().ok());
        let Some(outcome) = outcome else {
            return;
        };
        self.finish_relay_test_task(outcome);
    }

    fn wait_for_relay_test_task_headless(&mut self) {
        if !env_bool("QSC_TUI_HEADLESS") || self.relay_test_task.is_none() {
            return;
        }
        let outcome = self
            .relay_test_task
            .as_ref()
            .and_then(|rx| rx.recv_timeout(Duration::from_secs(3)).ok())
            .unwrap_or(RelayTestOutcome {
                ok: false,
                code: "relay_test_pending_timeout",
                message: "relay test timed out".to_string(),
            });
        self.finish_relay_test_task(outcome);
    }

    fn set_autolock_minutes(&mut self, minutes: u64) -> Result<(), &'static str> {
        if !(TUI_AUTOLOCK_MIN_MINUTES..=TUI_AUTOLOCK_MAX_MINUTES).contains(&minutes) {
            return Err("autolock_invalid_minutes");
        }
        let minutes_value = minutes.to_string();
        self.persist_account_secret(TUI_AUTOLOCK_SECRET_KEY, minutes_value.as_str())
            .map_err(|_| "autolock_config_unavailable")?;
        self.autolock_timeout_ms = minutes.saturating_mul(60_000);
        let minutes_s = minutes.to_string();
        emit_marker(
            "tui_autolock_set",
            None,
            &[("ok", "true"), ("minutes", minutes_s.as_str())],
        );
        self.request_redraw();
        Ok(())
    }

    fn poll_mode(&self) -> TuiPollMode {
        self.poll_mode
    }

    fn poll_interval_seconds(&self) -> u64 {
        self.poll_interval_seconds
            .clamp(TUI_POLL_MIN_INTERVAL_SECONDS, TUI_POLL_MAX_INTERVAL_SECONDS)
    }

    fn poll_interval_ms(&self) -> u64 {
        self.poll_interval_seconds().saturating_mul(1_000)
    }

    fn set_poll_mode_adaptive(&mut self) -> Result<(), &'static str> {
        self.persist_account_secret(TUI_POLL_MODE_SECRET_KEY, TuiPollMode::Adaptive.as_str())
            .map_err(|_| "poll_config_unavailable")?;
        let interval_value = self.poll_interval_seconds().to_string();
        self.persist_account_secret(TUI_POLL_INTERVAL_SECRET_KEY, interval_value.as_str())
            .map_err(|_| "poll_config_unavailable")?;
        self.poll_mode = TuiPollMode::Adaptive;
        self.poll_next_due_ms = None;
        let interval_s = self.poll_interval_seconds().to_string();
        emit_marker(
            "tui_poll_set",
            None,
            &[
                ("ok", "true"),
                ("mode", self.poll_mode.as_str()),
                ("interval_seconds", interval_s.as_str()),
            ],
        );
        self.request_redraw();
        Ok(())
    }

    fn set_poll_mode_fixed(&mut self, seconds: u64, now_ms: u64) -> Result<(), &'static str> {
        if !(TUI_POLL_MIN_INTERVAL_SECONDS..=TUI_POLL_MAX_INTERVAL_SECONDS).contains(&seconds) {
            return Err("poll_invalid_seconds");
        }
        self.persist_account_secret(TUI_POLL_MODE_SECRET_KEY, TuiPollMode::Fixed.as_str())
            .map_err(|_| "poll_config_unavailable")?;
        let interval_value = seconds.to_string();
        self.persist_account_secret(TUI_POLL_INTERVAL_SECRET_KEY, interval_value.as_str())
            .map_err(|_| "poll_config_unavailable")?;
        self.poll_mode = TuiPollMode::Fixed;
        self.poll_interval_seconds = seconds;
        self.poll_next_due_ms = Some(now_ms.saturating_add(self.poll_interval_ms()));
        let interval_s = self.poll_interval_seconds().to_string();
        emit_marker(
            "tui_poll_set",
            None,
            &[
                ("ok", "true"),
                ("mode", self.poll_mode.as_str()),
                ("interval_seconds", interval_s.as_str()),
            ],
        );
        self.request_redraw();
        Ok(())
    }

    fn emit_poll_show_marker(&self) {
        let interval_s = self.poll_interval_seconds().to_string();
        emit_marker(
            "tui_poll_show",
            None,
            &[
                ("ok", "true"),
                ("mode", self.poll_mode.as_str()),
                ("interval_seconds", interval_s.as_str()),
            ],
        );
    }

    fn maybe_run_fixed_poll(&mut self, now_ms: u64) -> bool {
        if self.is_locked() || self.poll_mode != TuiPollMode::Fixed {
            return false;
        }
        if self.relay.is_none() {
            return false;
        }
        let interval_ms = self.poll_interval_ms();
        if interval_ms == 0 {
            return false;
        }
        let mut due = self
            .poll_next_due_ms
            .unwrap_or_else(|| now_ms.saturating_add(interval_ms));
        if due > now_ms {
            self.poll_next_due_ms = Some(due);
            return false;
        }
        let mut ticked = false;
        while due <= now_ms {
            let due_s = due.to_string();
            let now_s = now_ms.to_string();
            let interval_s = self.poll_interval_seconds().to_string();
            let peer = self.selected_conversation_label();
            emit_marker(
                "tui_poll_tick",
                None,
                &[
                    ("mode", self.poll_mode.as_str()),
                    ("peer", peer.as_str()),
                    ("interval_seconds", interval_s.as_str()),
                    ("due_ms", due_s.as_str()),
                    ("now_ms", now_s.as_str()),
                ],
            );
            tui_receive_via_relay(self, peer.as_str());
            ticked = true;
            due = due.saturating_add(interval_ms);
        }
        self.poll_next_due_ms = Some(due);
        ticked
    }

    fn mark_input_activity(&mut self, now_ms: u64) {
        self.autolock_last_activity_ms = now_ms;
    }

    fn headless_now_ms(&self) -> u64 {
        self.headless_clock_ms
    }

    fn current_now_ms(&self) -> u64 {
        self.headless_clock_ms.max(self.autolock_last_activity_ms)
    }

    fn headless_advance_clock(&mut self, delta_ms: u64) {
        self.headless_clock_ms = self.headless_clock_ms.saturating_add(delta_ms);
        self.maybe_autolock(self.headless_clock_ms);
        let _ = self.maybe_run_fixed_poll(self.headless_clock_ms);
    }

    fn maybe_autolock(&mut self, now_ms: u64) -> bool {
        if self.is_locked() || self.autolock_timeout_ms == 0 {
            return false;
        }
        if now_ms.saturating_sub(self.autolock_last_activity_ms) < self.autolock_timeout_ms {
            return false;
        }
        self.set_locked_state(true, "inactivity_timeout");
        let minutes_s = self.autolock_minutes().to_string();
        emit_marker(
            "tui_autolock",
            None,
            &[("ok", "true"), ("minutes", minutes_s.as_str())],
        );
        true
    }

    fn take_clear_screen_pending(&mut self) -> bool {
        let pending = self.clear_screen_pending;
        self.clear_screen_pending = false;
        pending
    }

    fn take_force_full_redraw(&mut self) -> bool {
        let pending = self.force_full_redraw;
        self.force_full_redraw = false;
        pending
    }

    fn clear_ui_buffers_on_lock(&mut self, reason: &'static str) {
        self.mode = TuiMode::Normal;
        self.help_selected = 0;
        self.focus_scroll = 0;
        self.home_focus = TuiHomeFocus::Nav;
        self.locked_flow = LockedFlow::None;
        self.locked_clear_error();
        self.account_destroy_flow = AccountDestroyFlow::None;
        self.account_destroy_clear_error();
        self.clear_command_error();
        self.cmd_input_clear();
        self.inspector = TuiInspectorPane::Lock;
        self.nav_selected = 0;
        self.sync_messages_if_main_focused();
        self.sync_files_if_main_focused();
        self.sync_activity_if_main_focused();
        self.clear_screen_pending = true;
        self.force_full_redraw = true;
        self.request_redraw();
        emit_marker(
            "tui_buffer_clear",
            None,
            &[("ok", "true"), ("reason", reason)],
        );
    }

    fn set_locked_state(&mut self, locked: bool, reason: &'static str) {
        let was_locked = self.vault_locked;
        self.vault_locked = locked;
        self.status.locked = if locked { "LOCKED" } else { "UNLOCKED" };
        set_vault_unlocked(!locked);
        if locked && !was_locked {
            self.close_vault_session();
            self.reload_unlock_security_state();
            self.clear_ui_buffers_on_lock(reason);
        } else if locked {
            self.close_vault_session();
            self.reload_unlock_security_state();
            self.home_focus = TuiHomeFocus::Nav;
            self.inspector = TuiInspectorPane::Lock;
            self.nav_selected = 0;
        } else {
            self.locked_flow = LockedFlow::None;
            self.account_destroy_flow = AccountDestroyFlow::None;
            self.account_destroy_clear_error();
            self.clear_command_error();
            self.cmd_input_clear();
            if self.vault_session.is_none() {
                let _ = self.open_vault_session(None);
            }
            self.reload_unlock_security_state();
            self.reload_account_settings_from_vault();
            self.home_focus = TuiHomeFocus::Nav;
            self.inspector = TuiInspectorPane::Status;
            self.sync_nav_to_inspector_header();
            self.refresh_identity_status();
            let _ = self.refresh_account_cache(self.current_now_ms(), true);
        }
        self.request_redraw();
        emit_marker(
            "tui_lock_state",
            None,
            &[
                ("locked", self.status.locked),
                ("reason", reason),
                (
                    "vault",
                    if self.vault_present {
                        "present"
                    } else {
                        "missing"
                    },
                ),
                ("ok", "true"),
            ],
        );
    }

    fn last_payload_len(&self) -> usize {
        self.last_payload_len
    }

    fn ensure_conversation(&mut self, peer: &str) {
        self.conversations.entry(peer.to_string()).or_default();
        self.visible_counts.entry(peer.to_string()).or_insert(0);
        self.unread_counts.entry(peer.to_string()).or_insert(0);
    }

    fn conversation_labels(&self) -> Vec<String> {
        let mut labels = BTreeSet::new();
        labels.insert(TUI_NOTE_TO_SELF_LABEL.to_string());
        for (peer, stream) in &self.conversations {
            if !stream.is_empty() {
                labels.insert(peer.clone());
            }
        }
        for item in &self.files {
            if !item.peer.trim().is_empty() {
                labels.insert(item.peer.clone());
            }
        }
        let mut labels = labels.into_iter().collect::<Vec<_>>();
        if let Some(note_idx) = labels
            .iter()
            .position(|label| label == TUI_NOTE_TO_SELF_LABEL)
        {
            let note = labels.remove(note_idx);
            labels.insert(0, note);
        }
        labels
    }

    fn selected_conversation_label(&self) -> String {
        let active = self.session.peer_label.to_string();
        if active == TUI_NOTE_TO_SELF_TIMELINE_PEER {
            return TUI_NOTE_TO_SELF_LABEL.to_string();
        }
        if self.contact_record_cached(active.as_str()).is_some()
            || self.conversations.contains_key(active.as_str())
            || self.files.iter().any(|item| item.peer == active)
        {
            return active;
        }
        let labels = self.conversation_labels();
        labels
            .get(
                self.conversation_selected
                    .min(labels.len().saturating_sub(1)),
            )
            .cloned()
            .unwrap_or_else(|| TUI_NOTE_TO_SELF_LABEL.to_string())
    }

    fn apply_default_account_settings(&mut self) {
        self.autolock_timeout_ms = TUI_AUTOLOCK_DEFAULT_MINUTES.saturating_mul(60_000);
        self.poll_mode = TuiPollMode::Adaptive;
        self.poll_interval_seconds = TUI_POLL_DEFAULT_INTERVAL_SECONDS;
        self.receipt_policy = ReceiptPolicy::default();
        self.trust_onboarding_mode = TrustOnboardingMode::Balanced;
        self.poll_next_due_ms = None;
        self.status_last_command_result = None;
        self.relay_endpoint_cache = None;
        self.relay_endpoint_hash_cache = None;
        self.relay_token_set_cache = false;
        self.relay_inbox_token_hash_cache = None;
        self.relay_inbox_token_set_cache = false;
        self.relay_last_test_result = "none".to_string();
        self.relay_test_task = None;
        self.contacts_records.clear();
        self.refresh_contacts();
        self.request_redraw();
    }

    fn reload_account_settings_from_vault(&mut self) {
        let autolock_minutes = self
            .read_account_secret(TUI_AUTOLOCK_SECRET_KEY)
            .as_deref()
            .and_then(|v| normalize_tui_autolock_minutes(v).ok())
            .unwrap_or(TUI_AUTOLOCK_DEFAULT_MINUTES);
        self.autolock_timeout_ms = autolock_minutes.saturating_mul(60_000);
        let mode = self
            .read_account_secret(TUI_POLL_MODE_SECRET_KEY)
            .as_deref()
            .and_then(|v| normalize_tui_poll_mode(v).ok())
            .unwrap_or(TuiPollMode::Adaptive);
        let interval = self
            .read_account_secret(TUI_POLL_INTERVAL_SECRET_KEY)
            .as_deref()
            .and_then(|v| normalize_tui_poll_interval_seconds(v).ok())
            .unwrap_or(TUI_POLL_DEFAULT_INTERVAL_SECONDS);
        let receipt_mode = self
            .read_account_secret(TUI_RECEIPT_MODE_SECRET_KEY)
            .as_deref()
            .and_then(ReceiptEmitMode::from_raw)
            .unwrap_or(ReceiptEmitMode::Off);
        let receipt_batch_window_ms = self
            .read_account_secret(TUI_RECEIPT_BATCH_WINDOW_MS_SECRET_KEY)
            .as_deref()
            .and_then(parse_receipt_batch_window_ms)
            .unwrap_or(RECEIPT_BATCH_WINDOW_MS_DEFAULT);
        let receipt_jitter_ms = self
            .read_account_secret(TUI_RECEIPT_JITTER_MS_SECRET_KEY)
            .as_deref()
            .and_then(parse_receipt_jitter_ms)
            .unwrap_or(RECEIPT_JITTER_MS_DEFAULT);
        let file_confirm_mode = self
            .read_account_secret(TUI_FILE_CONFIRM_MODE_SECRET_KEY)
            .as_deref()
            .and_then(FileConfirmEmitMode::from_raw)
            .unwrap_or(FileConfirmEmitMode::CompleteOnly);
        let trust_onboarding_mode = self
            .read_account_secret(TUI_TRUST_MODE_SECRET_KEY)
            .as_deref()
            .and_then(TrustOnboardingMode::from_raw)
            .unwrap_or(TrustOnboardingMode::Balanced);
        self.poll_mode = mode;
        self.poll_interval_seconds = interval;
        self.receipt_policy = ReceiptPolicy {
            mode: receipt_mode,
            batch_window_ms: receipt_batch_window_ms,
            jitter_ms: receipt_jitter_ms,
            file_confirm_mode,
        };
        self.trust_onboarding_mode = trust_onboarding_mode;
        self.poll_next_due_ms = if self.poll_mode == TuiPollMode::Fixed {
            Some(
                self.current_now_ms()
                    .saturating_add(self.poll_interval_ms()),
            )
        } else {
            None
        };
        self.status_last_command_result =
            self.read_account_secret(TUI_LAST_COMMAND_RESULT_SECRET_KEY);
        self.relay_endpoint_cache = self
            .read_account_secret(TUI_RELAY_ENDPOINT_SECRET_KEY)
            .and_then(|v| {
                let trimmed = v.trim().to_string();
                if trimmed.is_empty() {
                    None
                } else {
                    Some(trimmed)
                }
            });
        self.relay_endpoint_hash_cache = self
            .relay_endpoint_cache
            .as_ref()
            .map(|endpoint| relay_endpoint_hash8(endpoint.as_str()));
        self.relay_token_set_cache = self
            .read_account_secret(TUI_RELAY_TOKEN_SECRET_KEY)
            .map(|v| !v.trim().is_empty())
            .unwrap_or(false);
        self.relay_token_file_cache = self
            .read_account_secret(TUI_RELAY_TOKEN_FILE_SECRET_KEY)
            .and_then(|v| {
                let trimmed = v.trim().to_string();
                if trimmed.is_empty() {
                    None
                } else {
                    Some(trimmed)
                }
            });
        self.relay_token_file_hash_cache = self
            .relay_token_file_cache
            .as_ref()
            .map(|path| relay_token_file_hash8(path.as_str()));
        let relay_inbox_token = self
            .read_account_secret(TUI_RELAY_INBOX_TOKEN_SECRET_KEY)
            .and_then(|v| {
                let trimmed = v.trim().to_string();
                if trimmed.is_empty() {
                    None
                } else {
                    Some(trimmed)
                }
            });
        self.relay_inbox_token_set_cache = relay_inbox_token.is_some();
        self.relay_inbox_token_hash_cache = relay_inbox_token
            .as_ref()
            .map(|token| route_token_hash8(token.as_str()));
        self.relay_last_test_result = "none".to_string();
        self.relay_test_task = None;
        let mut contacts = self
            .read_account_secret(CONTACTS_SECRET_KEY)
            .and_then(|raw| serde_json::from_str::<ContactsStore>(&raw).ok())
            .map(|store| store.peers)
            .unwrap_or_default();
        for (alias, rec) in contacts.iter_mut() {
            normalize_contact_record(alias.as_str(), rec);
        }
        self.contacts_records = contacts;
        self.refresh_contacts();
        let _ = self.refresh_account_cache(self.current_now_ms(), true);
        self.request_redraw();
    }

    fn refresh_identity_status(&mut self) {
        let fingerprint = compute_local_fingerprint();
        self.status.fingerprint = Box::leak(fingerprint.into_boxed_str());
        let peer_fp = compute_peer_fingerprint(self.session.peer_label);
        self.status.peer_fp = Box::leak(peer_fp.into_boxed_str());
        let _ = self.refresh_account_cache(self.current_now_ms(), true);
        self.request_redraw();
    }

    fn selected_contact_label(&self) -> String {
        if self.contacts.is_empty() {
            "peer-0".to_string()
        } else {
            self.contacts[self
                .contacts_selected
                .min(self.contacts.len().saturating_sub(1))]
            .clone()
        }
    }

    fn selected_peer_trust_state(&self) -> &'static str {
        contact_state(self.contact_record_cached(self.session.peer_label))
    }

    fn trust_allows_peer_send_strict(&mut self, peer: &str) -> Result<(), &'static str> {
        let Some(rec) = self.contact_record_cached(peer) else {
            self.set_command_error("msg: unknown contact; add contact first");
            self.push_cmd_result("msg blocked", false, "unknown contact (add contact first)");
            emit_tui_named_marker(
                "QSC_TUI_SEND_BLOCKED",
                &[("reason", "unknown_contact"), ("peer", peer)],
            );
            emit_tui_trust_remediation("unknown_contact", peer, None);
            self.push_cmd_result(
                "trust remediation",
                false,
                trust_remediation_hint("unknown_contact"),
            );
            self.push_cmd_result(
                "trust remediation",
                false,
                trust_remediation_verify_vs_trusted_hint(),
            );
            emit_marker(
                "tui_msg_reject",
                Some("unknown_contact"),
                &[("reason", "unknown_contact"), ("peer", peer)],
            );
            return Err("unknown_contact");
        };
        let Some(primary) = primary_device(rec).cloned() else {
            self.set_command_error("msg: no trusted device; verify and trust a device first");
            self.push_cmd_result("msg blocked", false, "no trusted device");
            emit_tui_named_marker(
                "QSC_TUI_SEND_BLOCKED",
                &[("reason", "no_trusted_device"), ("peer", peer)],
            );
            emit_tui_trust_remediation("no_trusted_device", peer, None);
            self.push_cmd_result(
                "trust remediation",
                false,
                trust_remediation_hint("no_trusted_device"),
            );
            self.push_cmd_result(
                "trust remediation",
                false,
                trust_remediation_verify_vs_trusted_hint(),
            );
            emit_marker(
                "tui_msg_reject",
                Some("no_trusted_device"),
                &[("reason", "no_trusted_device"), ("peer", peer)],
            );
            return Err("no_trusted_device");
        };
        let primary_device_id = primary.device_id;
        let primary_state = canonical_device_state(primary.state.as_str());
        let has_trusted = contact_has_trusted_device(rec);
        match primary_state {
            "CHANGED" => {
                self.set_command_error(
                    "msg: primary device changed; explicit re-approval required",
                );
                self.push_cmd_result("msg blocked", false, "primary device changed");
                emit_tui_named_marker(
                    "QSC_TUI_SEND_BLOCKED",
                    &[
                        ("reason", "device_changed_reapproval_required"),
                        ("peer", peer),
                    ],
                );
                emit_tui_trust_remediation(
                    "device_changed_reapproval_required",
                    peer,
                    Some(primary_device_id.as_str()),
                );
                self.push_cmd_result(
                    "trust remediation",
                    false,
                    trust_remediation_hint("device_changed_reapproval_required"),
                );
                self.push_cmd_result(
                    "trust remediation",
                    false,
                    trust_remediation_verify_vs_trusted_hint(),
                );
                emit_marker(
                    "tui_msg_reject",
                    Some("device_changed_reapproval_required"),
                    &[
                        ("reason", "device_changed_reapproval_required"),
                        ("peer", peer),
                    ],
                );
                return Err("device_changed_reapproval_required");
            }
            "REVOKED" => {
                self.set_command_error(
                    "msg: primary device revoked; select/re-approve a trusted device",
                );
                self.push_cmd_result("msg blocked", false, "primary device revoked");
                emit_tui_named_marker(
                    "QSC_TUI_SEND_BLOCKED",
                    &[("reason", "device_revoked"), ("peer", peer)],
                );
                emit_tui_trust_remediation(
                    "device_revoked",
                    peer,
                    Some(primary_device_id.as_str()),
                );
                self.push_cmd_result(
                    "trust remediation",
                    false,
                    trust_remediation_hint("device_revoked"),
                );
                self.push_cmd_result(
                    "trust remediation",
                    false,
                    trust_remediation_verify_vs_trusted_hint(),
                );
                emit_marker(
                    "tui_msg_reject",
                    Some("device_revoked"),
                    &[("reason", "device_revoked"), ("peer", peer)],
                );
                return Err("device_revoked");
            }
            _ => {}
        }
        if !has_trusted {
            self.set_command_error("msg: no trusted device; verify and trust a device first");
            self.push_cmd_result("msg blocked", false, "no trusted device");
            emit_tui_named_marker(
                "QSC_TUI_SEND_BLOCKED",
                &[("reason", "no_trusted_device"), ("peer", peer)],
            );
            emit_tui_trust_remediation("no_trusted_device", peer, Some(primary_device_id.as_str()));
            self.push_cmd_result(
                "trust remediation",
                false,
                trust_remediation_hint("no_trusted_device"),
            );
            self.push_cmd_result(
                "trust remediation",
                false,
                trust_remediation_verify_vs_trusted_hint(),
            );
            emit_marker(
                "tui_msg_reject",
                Some("no_trusted_device"),
                &[("reason", "no_trusted_device"), ("peer", peer)],
            );
            return Err("no_trusted_device");
        }
        Ok(())
    }

    fn focus_messages_thread(&mut self, peer: &str) {
        self.ensure_conversation(peer);
        let labels = self.conversation_labels();
        if let Some(idx) = labels.iter().position(|p| p == peer) {
            self.conversation_selected = idx;
        }
        self.inspector = TuiInspectorPane::Events;
        self.mode = TuiMode::Normal;
        self.home_focus = TuiHomeFocus::Nav;
        self.set_active_peer(peer);
        self.sync_messages_if_main_focused();
        emit_tui_named_marker("QSC_TUI_NAV", &[("focus", "messages"), ("thread", peer)]);
    }

    fn selected_peer_identity_short(&self) -> String {
        self.contact_record_cached(self.session.peer_label)
            .map(|rec| short_identity_display(rec.fp.as_str()))
            .unwrap_or_else(|| "untrusted".to_string())
    }

    fn contact_record_cached(&self, label: &str) -> Option<&ContactRecord> {
        self.contacts_records.get(label)
    }

    fn contact_display_line_cached(&self, label: &str) -> String {
        label.to_string()
    }

    fn persist_contacts_cache(&mut self) -> Result<(), ErrorCode> {
        let mut store = ContactsStore {
            peers: self.contacts_records.clone(),
        };
        for (alias, rec) in store.peers.iter_mut() {
            normalize_contact_record(alias.as_str(), rec);
        }
        let json = serde_json::to_string(&store).map_err(|_| ErrorCode::ParseFailed)?;
        self.persist_account_secret(CONTACTS_SECRET_KEY, json.as_str())
            .map_err(|_| ErrorCode::IoWriteFailed)?;
        self.contacts_records = store.peers;
        Ok(())
    }

    fn persist_contacts_cache_with(
        &mut self,
        label: &str,
        mut rec: ContactRecord,
    ) -> Result<(), ErrorCode> {
        normalize_contact_record(label, &mut rec);
        self.contacts_records.insert(label.to_string(), rec);
        self.persist_contacts_cache()
    }

    fn tui_relay_inbox_route_token(&self) -> Result<String, &'static str> {
        // Reuse the shared vault helper so TUI and CLI resolve the persisted inbox
        // token through the same path.
        relay_self_inbox_route_token()
    }

    fn tui_timeline_store_load(&self) -> Result<TimelineStore, &'static str> {
        let raw = if let Some(session) = self.vault_session.as_ref() {
            vault::session_get(session, TIMELINE_SECRET_KEY).map_err(|_| "timeline_tampered")?
        } else {
            None
        };
        let mut store = raw
            .map(|encoded| {
                serde_json::from_str::<TimelineStore>(encoded.as_str())
                    .map_err(|_| "timeline_tampered")
            })
            .transpose()?
            .unwrap_or_default();
        if store.next_ts == 0 {
            store.next_ts = 1;
        }
        Ok(store)
    }

    fn tui_timeline_store_save(&mut self, store: &TimelineStore) -> Result<(), &'static str> {
        let json = serde_json::to_string(store).map_err(|_| "timeline_unavailable")?;
        self.persist_account_secret(TIMELINE_SECRET_KEY, json.as_str())
            .map_err(|_| "timeline_unavailable")
    }

    fn append_tui_timeline_entry(
        &mut self,
        peer: &str,
        direction: &str,
        byte_len: usize,
        kind: &str,
        final_state: MessageState,
    ) -> Result<(), &'static str> {
        if !channel_label_ok(peer) {
            return Err("timeline_peer_invalid");
        }
        message_state_transition_allowed(MessageState::Created, final_state, direction)?;
        let mut store = self.tui_timeline_store_load()?;
        let ts = store.next_ts;
        store.next_ts = store.next_ts.saturating_add(1);
        let id = format!("{}-{}", direction, ts);
        let entry = TimelineEntry {
            id: id.clone(),
            peer: peer.to_string(),
            direction: direction.to_string(),
            byte_len,
            kind: kind.to_string(),
            ts,
            target_device_id: None,
            state: final_state.as_str().to_string(),
            status: final_state.as_status().to_string(),
        };
        store.peers.entry(peer.to_string()).or_default().push(entry);
        self.tui_timeline_store_save(&store)?;
        emit_message_state_transition(id.as_str(), MessageState::Created, final_state);
        Ok(())
    }

    fn selected_file_id(&self) -> Option<&str> {
        self.files
            .get(self.file_selected.min(self.files.len().saturating_sub(1)))
            .map(|v| v.id.as_str())
    }

    fn refresh_file_selection_bounds(&mut self) {
        if self.file_selected >= self.files.len() {
            self.file_selected = self.files.len().saturating_sub(1);
        }
        if self.files.is_empty() {
            self.file_selected = 0;
            self.file_multi_selected.clear();
            return;
        }
        self.file_multi_selected
            .retain(|id| self.files.iter().any(|f| &f.id == id));
    }

    fn upsert_file_item(&mut self, item: TuiFileItem, from_update: bool) {
        let mut changed = false;
        if let Some(existing) = self.files.iter_mut().find(|v| v.id == item.id) {
            if existing != &item {
                *existing = item;
                changed = true;
            }
        } else {
            self.files.push(item);
            changed = true;
        }
        self.files.sort_by(|a, b| a.id.cmp(&b.id));
        self.refresh_file_selection_bounds();
        if from_update
            && changed
            && !(self.mode == TuiMode::Normal
                && self.inspector == TuiInspectorPane::Files
                && self.home_focus == TuiHomeFocus::Main)
        {
            self.file_unseen_updates = self.file_unseen_updates.saturating_add(1);
        }
    }

    fn refresh_files_from_timeline(&mut self) {
        for item in load_tui_files_snapshot() {
            self.upsert_file_item(item, true);
        }
    }

    fn files_select_by_id(&mut self, id: &str) -> bool {
        if let Some(idx) = self.files.iter().position(|v| v.id == id) {
            self.file_selected = idx;
            true
        } else {
            false
        }
    }

    fn files_toggle_selected(&mut self) -> bool {
        let Some(id) = self.selected_file_id().map(str::to_string) else {
            return false;
        };
        if self.file_multi_selected.contains(id.as_str()) {
            self.file_multi_selected.remove(id.as_str());
        } else {
            self.file_multi_selected.insert(id);
        }
        true
    }

    fn files_move(&mut self, delta: i32) {
        if self.files.is_empty() {
            self.file_selected = 0;
            return;
        }
        let max = (self.files.len() - 1) as i32;
        let mut idx = self.file_selected as i32 + delta;
        if idx < 0 {
            idx = 0;
        }
        if idx > max {
            idx = max;
        }
        self.file_selected = idx as usize;
    }

    fn set_active_peer(&mut self, peer: &str) {
        self.session.peer_label = Box::leak(peer.to_string().into_boxed_str());
        self.refresh_qsp_status();
    }

    fn sync_messages_if_main_focused(&mut self) {
        if self.mode != TuiMode::Normal
            || self.inspector != TuiInspectorPane::Events
            || self.home_focus != TuiHomeFocus::Main
        {
            return;
        }
        let peer = self.selected_conversation_label();
        let total = self
            .conversations
            .get(peer.as_str())
            .map(|v| v.len())
            .unwrap_or(0usize);
        self.visible_counts.insert(peer.clone(), total);
        self.unread_counts.insert(peer, 0);
    }

    fn sync_files_if_main_focused(&mut self) {
        if self.mode != TuiMode::Normal
            || self.inspector != TuiInspectorPane::Files
            || self.home_focus != TuiHomeFocus::Main
        {
            return;
        }
        self.file_unseen_updates = 0;
    }

    fn sync_activity_if_main_focused(&mut self) {
        if self.mode != TuiMode::Normal
            || self.inspector != TuiInspectorPane::Activity
            || self.home_focus != TuiHomeFocus::Main
        {
            return;
        }
        self.activity_visible_count = self.events.len();
        self.activity_unseen_updates = 0;
    }

    fn record_message_line(&mut self, peer: &str, state: &str, direction: &str, detail: &str) {
        self.ensure_conversation(peer);
        let line = format_message_transcript_line(peer, state, direction, detail);
        {
            let stream = self.conversations.entry(peer.to_string()).or_default();
            stream.push_back(line);
            if stream.len() > 128 {
                stream.pop_front();
            }
        }
        let selected = self.selected_conversation_label();
        let auto_append = self.mode == TuiMode::Normal
            && self.inspector == TuiInspectorPane::Events
            && self.home_focus == TuiHomeFocus::Main
            && selected == peer;
        let total = self
            .conversations
            .get(peer)
            .map(|v| v.len())
            .unwrap_or(0usize);
        if auto_append {
            self.visible_counts.insert(peer.to_string(), total);
            self.unread_counts.insert(peer.to_string(), 0);
        } else {
            let unread = self
                .unread_counts
                .get(peer)
                .copied()
                .unwrap_or(0usize)
                .saturating_add(1);
            self.unread_counts.insert(peer.to_string(), unread);
            self.visible_counts
                .entry(peer.to_string())
                .or_insert(total.saturating_sub(1));
        }
        let total_s = total.to_string();
        let unread_s = self
            .unread_counts
            .get(peer)
            .copied()
            .unwrap_or(0)
            .to_string();
        emit_marker(
            "tui_message_event",
            None,
            &[
                ("peer", peer),
                ("state", state),
                ("mode", if auto_append { "append" } else { "buffer" }),
                ("total", total_s.as_str()),
                ("unread", unread_s.as_str()),
            ],
        );
        if let Some(delivery) = message_delivery_semantic_from_state_str(direction, state) {
            emit_tui_delivery_state(peer, delivery);
        }
    }

    fn selected_messages_thread(&self) -> Option<String> {
        if self.inspector != TuiInspectorPane::Events {
            return None;
        }
        Some(self.selected_conversation_label())
    }

    fn map_thread_to_timeline_peer(thread: &str) -> &str {
        if thread == TUI_NOTE_TO_SELF_LABEL {
            TUI_NOTE_TO_SELF_TIMELINE_PEER
        } else {
            thread
        }
    }

    fn update_send_lifecycle(&mut self, value: &str) {
        self.send_lifecycle = value.to_string();
        self.status.send_lifecycle = Box::leak(self.send_lifecycle.clone().into_boxed_str());
        emit_marker(
            "tui_status_update",
            None,
            &[("field", "send_lifecycle"), ("value", value)],
        );
    }

    fn refresh_envelope(&mut self, payload_len: usize) {
        self.last_payload_len = payload_len;
        self.envelope = compute_envelope_status(payload_len);
        self.status.envelope = Box::leak(self.envelope.clone().into_boxed_str());
        emit_marker(
            "tui_status_update",
            None,
            &[("field", "envelope"), ("value", &self.envelope)],
        );
    }

    fn refresh_qsp_status(&mut self) {
        let peer = self.session.peer_label;
        self.qsp_status = qsp_status_string(peer);
        self.status.qsp = Box::leak(self.qsp_status.clone().into_boxed_str());
        let peer_fp = compute_peer_fingerprint(peer);
        self.status.peer_fp = Box::leak(peer_fp.into_boxed_str());
        self.refresh_contacts();
        emit_marker(
            "tui_status_update",
            None,
            &[("field", "qsp"), ("value", &self.qsp_status)],
        );
    }

    fn refresh_contacts(&mut self) {
        self.contacts = self.contacts_records.keys().cloned().collect::<Vec<_>>();
        self.contacts.sort();
        if self.contacts.is_empty() {
            self.contacts.push("peer-0".to_string());
        }
        if self.contacts_selected >= self.contacts.len() {
            self.contacts_selected = self.contacts.len().saturating_sub(1);
        }
        let labels = self.conversation_labels();
        if self.conversation_selected >= labels.len() {
            self.conversation_selected = labels.len().saturating_sub(1);
        }
    }

    fn push_event(&mut self, kind: &str, action: &str) {
        self.event_seq = self.event_seq.wrapping_add(1);
        let seq_s = self.event_seq.to_string();
        emit_marker(
            "tui_event",
            None,
            &[("kind", kind), ("action", action), ("seq", seq_s.as_str())],
        );
        let line = format!("{}:{} #{}", kind, action, self.event_seq);
        self.events.push_back(line);
        if self.events.len() > 64 {
            self.events.pop_front();
        }
        self.record_activity_update();
    }

    fn push_event_line(&mut self, line: String) {
        self.events.push_back(line);
        if self.events.len() > 64 {
            self.events.pop_front();
        }
        self.record_activity_update();
    }

    fn record_activity_update(&mut self) {
        let total = self.events.len();
        let auto_append = self.mode == TuiMode::Normal
            && self.inspector == TuiInspectorPane::Activity
            && self.home_focus == TuiHomeFocus::Main;
        if auto_append {
            self.activity_visible_count = total;
            self.activity_unseen_updates = 0;
            return;
        }
        self.activity_unseen_updates = self.activity_unseen_updates.saturating_add(1);
        if self.activity_visible_count == 0 && total > 0 {
            self.activity_visible_count = total.saturating_sub(1);
        } else if self.activity_visible_count > total {
            self.activity_visible_count = total;
        }
    }

    fn enter_help_mode(&mut self) {
        if matches!(
            self.mode,
            TuiMode::FocusEvents
                | TuiMode::FocusFiles
                | TuiMode::FocusActivity
                | TuiMode::FocusStatus
                | TuiMode::FocusSession
                | TuiMode::FocusContacts
                | TuiMode::FocusSettings
                | TuiMode::FocusLock
        ) {
            let pane = TuiState::focus_pane_name(self.mode);
            emit_marker("tui_focus", None, &[("pane", pane), ("on", "false")]);
        }
        self.mode = TuiMode::Help;
        self.help_selected = 0;
        emit_marker("tui_help_mode", None, &[("on", "true")]);
        let items = tui_help_items();
        for item in items {
            emit_marker("tui_help_item", None, &[("cmd", item.cmd)]);
        }
        let count_s = items.len().to_string();
        emit_marker("tui_help_rendered", None, &[("count", count_s.as_str())]);
    }

    fn exit_help_mode(&mut self) {
        if self.mode == TuiMode::Help {
            self.mode = TuiMode::Normal;
            emit_marker("tui_help_mode", None, &[("on", "false")]);
        }
    }

    fn toggle_help_mode(&mut self) {
        if self.mode == TuiMode::Help {
            self.exit_help_mode();
        } else {
            self.enter_help_mode();
        }
    }

    fn focus_pane_name(mode: TuiMode) -> &'static str {
        match mode {
            TuiMode::FocusEvents => "events",
            TuiMode::FocusFiles => "files",
            TuiMode::FocusActivity => "activity",
            TuiMode::FocusStatus => "status",
            TuiMode::FocusSession => "session",
            TuiMode::FocusContacts => "contacts",
            TuiMode::FocusSettings => "settings",
            TuiMode::FocusLock => "lock",
            _ => "dashboard",
        }
    }

    fn inspector_name(&self) -> &'static str {
        match self.inspector {
            TuiInspectorPane::Events => "events",
            TuiInspectorPane::Files => "files",
            TuiInspectorPane::Activity => "activity",
            TuiInspectorPane::Status => "status",
            TuiInspectorPane::Account => "account",
            TuiInspectorPane::Relay => "relay",
            TuiInspectorPane::CmdResults => "cmd_results",
            TuiInspectorPane::Session => "session",
            TuiInspectorPane::Contacts => "contacts",
            TuiInspectorPane::Settings => "settings",
            TuiInspectorPane::Lock => "lock",
            TuiInspectorPane::Help => "help",
            TuiInspectorPane::About => "about",
            TuiInspectorPane::Legal => "legal",
        }
    }

    fn set_inspector(&mut self, pane: TuiInspectorPane) {
        self.inspector = pane;
        self.main_scroll_max_current = 0;
        self.main_view_rows_current = self.main_view_rows();
        self.sync_nav_to_inspector_header();
        self.sync_messages_if_main_focused();
        self.sync_files_if_main_focused();
        self.sync_activity_if_main_focused();
        self.request_redraw();
        emit_marker("tui_inspector", None, &[("pane", self.inspector_name())]);
    }

    fn route_show_to_system_nav(&mut self, pane: TuiInspectorPane) {
        self.set_inspector(pane);
        self.home_focus = TuiHomeFocus::Nav;
        self.cmd_input_clear();
        self.request_redraw();
        emit_marker("tui_focus_home", None, &[("pane", self.home_focus_name())]);
    }

    fn focus_mode_for_inspector(&self) -> TuiMode {
        match self.inspector {
            TuiInspectorPane::Events => TuiMode::FocusEvents,
            TuiInspectorPane::Files => TuiMode::FocusFiles,
            TuiInspectorPane::Activity => TuiMode::FocusActivity,
            TuiInspectorPane::Status => TuiMode::FocusStatus,
            TuiInspectorPane::Account => TuiMode::FocusSettings,
            TuiInspectorPane::Relay => TuiMode::FocusSettings,
            TuiInspectorPane::CmdResults => TuiMode::FocusStatus,
            TuiInspectorPane::Session => TuiMode::FocusSession,
            TuiInspectorPane::Contacts => TuiMode::FocusContacts,
            TuiInspectorPane::Settings => TuiMode::FocusSettings,
            TuiInspectorPane::Lock => TuiMode::FocusLock,
            TuiInspectorPane::Help => TuiMode::FocusSettings,
            TuiInspectorPane::About => TuiMode::FocusSettings,
            TuiInspectorPane::Legal => TuiMode::FocusSettings,
        }
    }

    fn home_layout_snapshot(&self, cols: u16, rows: u16) -> HomeLayoutSnapshot {
        HomeLayoutSnapshot {
            contacts_shown: cols >= TUI_H3_WIDE_MIN,
            header_compact: rows < TUI_H3_TALL_MIN,
        }
    }

    fn home_focus_name(&self) -> &'static str {
        match self.home_focus {
            TuiHomeFocus::Nav => "nav",
            TuiHomeFocus::Main => "main",
            TuiHomeFocus::Command => "command",
        }
    }

    fn home_focus_label_token(&self) -> &'static str {
        match self.home_focus {
            TuiHomeFocus::Nav => "NAV",
            TuiHomeFocus::Main => "MAIN",
            TuiHomeFocus::Command => "CMD",
        }
    }

    fn main_marker_title(&self) -> &'static str {
        match self.inspector {
            TuiInspectorPane::Events => "Messages Overview",
            TuiInspectorPane::Files => "Files",
            TuiInspectorPane::Activity => "Activity",
            TuiInspectorPane::Status => "System Overview",
            TuiInspectorPane::Account => "Account",
            TuiInspectorPane::Relay => "Relay",
            TuiInspectorPane::CmdResults => "Results",
            TuiInspectorPane::Session => "Keys",
            TuiInspectorPane::Contacts => "Contacts Overview",
            TuiInspectorPane::Settings => "System Settings",
            TuiInspectorPane::Lock => "Lock Status",
            TuiInspectorPane::Help => "Help",
            TuiInspectorPane::About => "About",
            TuiInspectorPane::Legal => "Legal",
        }
    }

    fn home_focus_cycle(&mut self, delta: i32) {
        let idx = match self.home_focus {
            TuiHomeFocus::Nav => 0i32,
            TuiHomeFocus::Main => 1i32,
            TuiHomeFocus::Command => 2i32,
        };
        let next = (idx + delta).rem_euclid(3);
        self.home_focus = match next {
            0 => TuiHomeFocus::Nav,
            1 => TuiHomeFocus::Main,
            _ => TuiHomeFocus::Command,
        };
        self.sync_messages_if_main_focused();
        self.sync_files_if_main_focused();
        self.sync_activity_if_main_focused();
        emit_marker("tui_focus_home", None, &[("pane", self.home_focus_name())]);
    }

    fn main_scroll_key(&self) -> &'static str {
        self.inspector.as_name()
    }

    fn main_scroll_offset(&self) -> usize {
        self.main_scroll_offsets
            .get(self.main_scroll_key())
            .copied()
            .unwrap_or(0)
    }

    fn set_main_scroll_offset(&mut self, value: usize) {
        let key = self.main_scroll_key();
        if value == 0 {
            self.main_scroll_offsets.remove(key);
        } else {
            self.main_scroll_offsets.insert(key, value);
        }
    }

    fn update_main_scroll_metrics(&mut self, content_lines: usize, view_rows: usize) {
        self.main_view_rows_current = view_rows.max(1);
        self.main_scroll_max_current = content_lines.saturating_sub(self.main_view_rows_current);
        let clamped = self.main_scroll_offset().min(self.main_scroll_max_current);
        self.set_main_scroll_offset(clamped);
    }

    fn main_view_rows(&self) -> usize {
        usize::from(terminal_rows_for_headless().saturating_sub(5)).max(1)
    }

    fn estimated_main_line_count(&self) -> usize {
        match self.inspector {
            TuiInspectorPane::Events => self
                .conversations
                .get(self.selected_conversation_label().as_str())
                .map(|v| v.len())
                .unwrap_or(0)
                .saturating_add(8),
            TuiInspectorPane::Files => self.files.len().saturating_add(14),
            TuiInspectorPane::Activity => self.events.len().saturating_add(8),
            TuiInspectorPane::Status => 22,
            TuiInspectorPane::Account => 20,
            TuiInspectorPane::Relay => 20,
            TuiInspectorPane::CmdResults => 8,
            TuiInspectorPane::Session => 16,
            TuiInspectorPane::Contacts => self.contacts.len().saturating_add(14),
            TuiInspectorPane::Settings => 20,
            TuiInspectorPane::Lock => 16,
            TuiInspectorPane::Help => 24,
            TuiInspectorPane::About => 8,
            TuiInspectorPane::Legal => 8,
        }
    }

    fn ensure_main_scroll_metrics(&mut self) {
        if self.main_view_rows_current <= 1 {
            self.main_view_rows_current = self.main_view_rows();
        }
        if self.main_scroll_max_current == 0 {
            let line_count = self.estimated_main_line_count();
            self.main_scroll_max_current = line_count.saturating_sub(self.main_view_rows_current);
            let clamped = self.main_scroll_offset().min(self.main_scroll_max_current);
            self.set_main_scroll_offset(clamped);
        }
    }

    fn emit_main_scroll_marker(&self) {
        let offset_s = self.main_scroll_offset().to_string();
        let max_s = self.main_scroll_max_current.to_string();
        let rows_s = self.main_view_rows_current.to_string();
        emit_marker(
            "tui_main_scroll",
            None,
            &[
                ("inspector", self.inspector_name()),
                ("offset", offset_s.as_str()),
                ("max", max_s.as_str()),
                ("view_rows", rows_s.as_str()),
            ],
        );
    }

    fn main_scroll_move(&mut self, delta: i32) {
        self.ensure_main_scroll_metrics();
        let mut idx = self.main_scroll_offset() as i32 + delta;
        if idx < 0 {
            idx = 0;
        }
        let max = self.main_scroll_max_current as i32;
        if idx > max {
            idx = max;
        }
        self.set_main_scroll_offset(idx as usize);
        self.request_redraw();
        self.emit_main_scroll_marker();
    }

    fn main_scroll_page(&mut self, direction: i32) {
        self.ensure_main_scroll_metrics();
        let page = self.main_view_rows_current.max(1) as i32;
        self.main_scroll_move(direction.saturating_mul(page));
    }

    fn main_scroll_home(&mut self) {
        self.ensure_main_scroll_metrics();
        self.set_main_scroll_offset(0);
        self.request_redraw();
        self.emit_main_scroll_marker();
    }

    fn main_scroll_end(&mut self) {
        self.ensure_main_scroll_metrics();
        self.set_main_scroll_offset(self.main_scroll_max_current);
        self.request_redraw();
        self.emit_main_scroll_marker();
    }

    fn emit_home_render_marker(&self, cols: u16, rows: u16) {
        if self.mode != TuiMode::Normal {
            return;
        }
        if self.is_locked() {
            let cmdbar_text = self.cmd_bar_text();
            let main_lines = self.locked_main_lines();
            let main_step = self.locked_flow_name();
            let main_intro_a = main_lines
                .iter()
                .find(|line| line.contains("This will create an encrypted vault"))
                .map(|v| v.as_str())
                .unwrap_or("none");
            let main_intro_b = main_lines
                .iter()
                .find(|line| line.contains("Choose a strong passphrase"))
                .map(|v| v.as_str())
                .unwrap_or("none");
            let main_input_line = main_lines
                .iter()
                .rev()
                .find(|line| {
                    line.starts_with("Alias:")
                        || line.starts_with("Passphrase:")
                        || line.starts_with("Confirm:")
                        || line.starts_with("Confirm (Y/N):")
                        || line.starts_with("Confirm (I AGREE/N):")
                })
                .map(|v| v.as_str())
                .unwrap_or("none");
            let main_error_line = main_lines
                .iter()
                .find(|line| line.starts_with("error:"))
                .map(|v| v.as_str())
                .unwrap_or("none");
            let main_summary_alias = main_lines
                .iter()
                .find(|line| line.starts_with("Alias:"))
                .map(|v| v.as_str())
                .unwrap_or("none");
            let main_summary_passphrase = if main_lines
                .iter()
                .any(|line| line.as_str() == "Passphrase: set (hidden)")
            {
                "set_hidden"
            } else {
                "none"
            };
            let main_hints_line = main_lines
                .iter()
                .find(|line| line.starts_with("Keys:") || line.starts_with("Submit:"))
                .map(|v| v.as_str())
                .unwrap_or("none");
            emit_marker(
                "tui_locked_shell",
                None,
                &[
                    (
                        "vault",
                        if self.has_vault() {
                            "present"
                        } else {
                            "missing"
                        },
                    ),
                    ("nav", "unlock,exit"),
                    (
                        "main",
                        if self.has_vault() {
                            "locked"
                        } else {
                            "init_required"
                        },
                    ),
                    ("cmd", if self.has_vault() { "/unlock" } else { "/init" }),
                    ("cmdbar_text", cmdbar_text.as_str()),
                    ("wizard", self.locked_flow_name()),
                    ("nav_title", "qsc"),
                    ("main_title", "none"),
                    ("cmd_panel_title", "none"),
                    ("focus", self.home_focus_name()),
                    ("main_step", main_step),
                    ("main_intro_a", main_intro_a),
                    ("main_intro_b", main_intro_b),
                    ("main_input", main_input_line),
                    ("main_error", main_error_line),
                    ("main_summary_alias", main_summary_alias),
                    ("main_summary_passphrase", main_summary_passphrase),
                    ("main_hints", main_hints_line),
                    (
                        "main_locked_line",
                        main_lines
                            .iter()
                            .find(|line| line.starts_with("Locked:"))
                            .map(|v| v.as_str())
                            .unwrap_or("none"),
                    ),
                    ("panel_pad", "2"),
                    ("nav_child_indent", "2"),
                    ("chrome", "single"),
                    ("outer_border", "1"),
                    ("header_divider", "1"),
                    ("header_row_vdiv", "0"),
                    ("v_divider", "1"),
                    ("h_divider", "1"),
                    ("divider_h_char", "─"),
                    ("divider_v_char", "│"),
                    ("divider_style", "dim"),
                ],
            );
            let nav_rows = self.nav_rows();
            let nav_selected = self.nav_selected.min(nav_rows.len().saturating_sub(1));
            let nav_selected_s = nav_selected.to_string();
            let selected_label = nav_rows
                .get(nav_selected)
                .map(|row| self.nav_row_label(row))
                .unwrap_or_else(|| "none".to_string());
            let selected_label_marker = selected_label.replace(' ', "_");
            let selected_markers = if self.home_focus == TuiHomeFocus::Nav && !nav_rows.is_empty() {
                "1"
            } else {
                "0"
            };
            emit_marker(
                "tui_nav_render",
                None,
                &[
                    ("selected_markers", selected_markers),
                    ("selected_index", nav_selected_s.as_str()),
                    ("selected_label", selected_label_marker.as_str()),
                    ("header", "[ QSC ]"),
                    ("header_left_padding", "1"),
                    ("counters", "none"),
                ],
            );
            return;
        }
        let layout = self.home_layout_snapshot(cols, rows);
        let cmdbar_text = self.cmd_bar_text();
        let cmdbar_marker = cmdbar_text.replace(' ', "_");
        let cmdbar_padded_marker = pad_panel_text(cmdbar_text.as_str()).replace(' ', "_");
        let main_scroll_s = self.main_scroll_offset().to_string();
        let main_scroll_max_s = self.main_scroll_max_current.to_string();
        let main_first_line_marker = format!(
            "{}{}",
            " ".repeat(PANEL_INNER_PAD),
            self.main_marker_title()
        )
        .replace(' ', "_");
        emit_marker(
            "tui_render",
            None,
            &[
                ("mode", "home"),
                ("layout", "h3"),
                ("inspector", self.inspector_name()),
                (
                    "contacts",
                    if layout.contacts_shown {
                        "shown"
                    } else {
                        "hidden"
                    },
                ),
                (
                    "header",
                    if layout.header_compact {
                        "compact"
                    } else {
                        "full"
                    },
                ),
                ("focus", self.home_focus_name()),
                ("nav", "shown"),
                ("expanded", self.inspector_name()),
                ("cmdbar", "full"),
                (
                    "cmd_hint",
                    if self.home_focus == TuiHomeFocus::Command {
                        "input"
                    } else {
                        "help"
                    },
                ),
                ("nav_title", "qsc"),
                ("main_title", "none"),
                ("cmd_panel_title", "none"),
                ("cmdbar_text", cmdbar_marker.as_str()),
                ("cmdbar_padded", cmdbar_padded_marker.as_str()),
                ("main_scroll", main_scroll_s.as_str()),
                ("main_scroll_max", main_scroll_max_s.as_str()),
                ("panel_pad", "2"),
                ("nav_child_indent", "2"),
                ("chrome", "single"),
                ("outer_border", "1"),
                ("header_divider", "1"),
                ("header_row_vdiv", "0"),
                ("v_divider", "1"),
                ("h_divider", "1"),
                ("divider_h_char", "─"),
                ("divider_v_char", "│"),
                ("divider_style", "dim"),
                ("main_first_line_padded", main_first_line_marker.as_str()),
            ],
        );
        match self.inspector {
            TuiInspectorPane::About => emit_marker(
                "tui_about_links",
                None,
                &[
                    ("governance", "1"),
                    ("traceability", "1"),
                    ("decisions", "1"),
                    ("docs", "1"),
                    ("tests", "1"),
                ],
            ),
            TuiInspectorPane::Legal => emit_marker(
                "tui_legal_fulltext",
                None,
                &[
                    ("sections", "summary,warranty,operator,privacy,init"),
                    ("overclaim", "none"),
                ],
            ),
            _ => {}
        }
        let nav_rows = self.nav_rows();
        let nav_selected = self.nav_selected.min(nav_rows.len().saturating_sub(1));
        let nav_selected_s = nav_selected.to_string();
        let selected_label = nav_rows
            .get(nav_selected)
            .map(|row| self.nav_row_label(row))
            .unwrap_or_else(|| "none".to_string());
        let selected_label_marker = selected_label.replace(' ', "_");
        emit_marker(
            "tui_nav_render",
            None,
            &[
                (
                    "selected_markers",
                    if nav_rows.is_empty() { "0" } else { "1" },
                ),
                ("selected_index", nav_selected_s.as_str()),
                ("selected_label", selected_label_marker.as_str()),
                ("header", "[ QSC ]"),
                ("header_left_padding", "1"),
                ("counters", "none"),
            ],
        );
        if self.inspector == TuiInspectorPane::Events {
            let peer = self.selected_conversation_label();
            let peer_marker = peer.replace(' ', "_");
            let total = self
                .conversations
                .get(peer.as_str())
                .map(|v| v.len())
                .unwrap_or(0usize);
            let visible = self
                .visible_counts
                .get(peer.as_str())
                .copied()
                .unwrap_or(total);
            let unread = self
                .unread_counts
                .get(peer.as_str())
                .copied()
                .unwrap_or(0usize);
            let total_s = total.to_string();
            let visible_s = visible.to_string();
            let unread_s = unread.to_string();
            emit_marker(
                "tui_messages_view",
                None,
                &[
                    ("peer", peer_marker.as_str()),
                    ("total", total_s.as_str()),
                    ("visible", visible_s.as_str()),
                    ("unread", unread_s.as_str()),
                    ("preview", "none"),
                    ("redacted", if self.is_locked() { "true" } else { "false" }),
                ],
            );
        }
        if self.inspector == TuiInspectorPane::Contacts {
            let selected = self.selected_contact_label();
            let selected_line = self.contact_display_line_cached(selected.as_str());
            let rec = self.contact_record_cached(selected.as_str());
            let nav_rows = self.nav_rows();
            let nav_kind = nav_rows
                .get(self.nav_selected.min(nav_rows.len().saturating_sub(1)))
                .map(|row| row.kind);
            let view = if matches!(nav_kind, Some(NavRowKind::Domain(TuiNavDomain::Contacts))) {
                "overview"
            } else {
                "detail"
            };
            emit_marker(
                "tui_contacts_view",
                None,
                &[
                    ("selected", selected.as_str()),
                    ("summary", selected_line.as_str()),
                    ("view", view),
                    ("trust", contact_state(rec)),
                    ("blocked", bool_str(rec.map(|v| v.blocked).unwrap_or(false))),
                    ("sections", "overview_table,contact_card,commands"),
                    ("you_copy", "hidden"),
                    ("commands_gap", "2"),
                    ("preview", "none"),
                    ("redacted", if self.is_locked() { "true" } else { "false" }),
                ],
            );
            if view == "overview" {
                let header = format_contacts_table_row("Alias", "Trust", "Blocked", "Last seen");
                let first_row = self
                    .contacts
                    .iter()
                    .take(TUI_INSPECTOR_CONTACTS_MAX)
                    .next()
                    .map(|alias| {
                        if let Some(rec) = self.contact_record_cached(alias) {
                            let trust = contact_state(Some(rec));
                            let blocked = if rec.blocked { "yes" } else { "no" };
                            let last_seen = rec
                                .seen_at
                                .map(|v| v.to_string())
                                .unwrap_or_else(|| "-".to_string());
                            format_contacts_table_row(alias, trust, blocked, last_seen.as_str())
                        } else {
                            format_contacts_table_row(alias, "UNVERIFIED", "no", "-")
                        }
                    })
                    .unwrap_or_else(|| format_contacts_table_row("-", "-", "-", "-"));
                let header_marker = header.replace(' ', "_");
                let first_row_marker = first_row.replace(' ', "_");
                emit_marker(
                    "tui_contacts_table",
                    None,
                    &[
                        ("header", header_marker.as_str()),
                        ("row0", first_row_marker.as_str()),
                    ],
                );
            }
        }
        if self.inspector == TuiInspectorPane::Activity {
            let total_s = self.events.len().to_string();
            let visible_s = self
                .activity_visible_count
                .min(self.events.len())
                .to_string();
            let unread_s = self.activity_unseen_updates.to_string();
            emit_marker(
                "tui_activity_view",
                None,
                &[
                    ("total", total_s.as_str()),
                    ("visible", visible_s.as_str()),
                    ("unread", unread_s.as_str()),
                    ("sections", "ledger,commands"),
                    ("redacted", if self.is_locked() { "true" } else { "false" }),
                ],
            );
        }
        if self.inspector == TuiInspectorPane::Files {
            let selected = self.selected_file_id().unwrap_or("none");
            let selected_state = self
                .files
                .get(self.file_selected.min(self.files.len().saturating_sub(1)))
                .map(|v| v.display_state.as_str())
                .unwrap_or("none");
            if let Some(item) = self
                .files
                .get(self.file_selected.min(self.files.len().saturating_sub(1)))
            {
                if let Some(delivery) = file_delivery_semantic_from_state(item.state.as_str()) {
                    emit_tui_file_delivery(item.peer.as_str(), delivery, item.id.as_str());
                }
            }
            emit_marker(
                "tui_files_view",
                None,
                &[
                    ("total", self.files.len().to_string().as_str()),
                    ("selected", selected),
                    (
                        "selected_count",
                        self.file_multi_selected.len().to_string().as_str(),
                    ),
                    ("updates", self.file_unseen_updates.to_string().as_str()),
                    ("state", selected_state),
                    ("preview", "none"),
                    ("redacted", if self.is_locked() { "true" } else { "false" }),
                ],
            );
        }
        if self.inspector == TuiInspectorPane::Session {
            let selected = self.session.peer_label;
            emit_marker(
                "tui_keys_view",
                None,
                &[
                    ("selected", selected),
                    ("sections", "metadata,verification,commands"),
                    ("multi_select", "false"),
                    ("preview", "none"),
                    ("redacted", if self.is_locked() { "true" } else { "false" }),
                ],
            );
        }
        if self.inspector == TuiInspectorPane::Settings {
            let minutes_s = self.autolock_minutes().to_string();
            let poll_interval_s = self.poll_interval_seconds().to_string();
            let receipt_batch_window_s = self.receipt_policy.batch_window_ms.to_string();
            let receipt_jitter_s = self.receipt_policy.jitter_ms.to_string();
            let attempt_limit = vault_attempt_limit_note(self.unlock_attempt_limit);
            let failed_unlocks_s = self.failed_unlock_attempts.to_string();
            emit_marker(
                "tui_settings_view",
                None,
                &[
                    ("read_only", "true"),
                    ("inline_actions", "false"),
                    ("lock_state", self.status.locked),
                    ("autolock_minutes", minutes_s.as_str()),
                    ("poll_mode", self.poll_mode().as_str()),
                    ("poll_interval_seconds", poll_interval_s.as_str()),
                    ("receipt_mode", self.receipt_policy.mode.as_str()),
                    ("receipt_batch_window_ms", receipt_batch_window_s.as_str()),
                    ("receipt_jitter_ms", receipt_jitter_s.as_str()),
                    (
                        "file_confirm_mode",
                        self.receipt_policy.file_confirm_mode.as_str(),
                    ),
                    ("vault_attempt_limit", attempt_limit.as_str()),
                    ("vault_failed_unlocks", failed_unlocks_s.as_str()),
                    ("commands_gap", "2"),
                    (
                        "sections",
                        "system_settings,lock,autolock,polling,vault_security,commands",
                    ),
                ],
            );
        }
        if self.inspector == TuiInspectorPane::Account {
            let alias_set =
                if self.account_alias_cache.is_empty() || self.account_alias_cache == "unset" {
                    "false"
                } else {
                    "true"
                };
            let alias_value = if self.is_locked() {
                "hidden"
            } else {
                self.account_alias_cache.as_str()
            };
            emit_marker(
                "tui_account_view",
                None,
                &[
                    ("sections", "identity,vault,device,commands"),
                    ("alias_set", alias_set),
                    ("alias_value", alias_value),
                    (
                        "verification_code",
                        self.account_verification_code_cache.as_str(),
                    ),
                    ("commands_gap", "2"),
                ],
            );
        }
        if self.inspector == TuiInspectorPane::Relay {
            let endpoint = self.relay_endpoint_redacted();
            let inbox_token = self.relay_inbox_token_redacted();
            let token_file = self.relay_token_file_redacted();
            let (token_file_state, token_file_perms) = self.relay_token_file_status();
            let pinning = relay_pinning_label(self.relay_endpoint_cache.as_deref());
            emit_marker(
                "tui_relay_view",
                None,
                &[
                    (
                        "configured",
                        if self.relay_endpoint_cache.is_some() {
                            "true"
                        } else {
                            "false"
                        },
                    ),
                    ("endpoint", endpoint.as_str()),
                    (
                        "transport",
                        relay_transport_label(self.relay_endpoint_cache.as_deref()),
                    ),
                    ("tls", relay_tls_label(self.relay_endpoint_cache.as_deref())),
                    ("pinning", pinning),
                    ("auth", self.relay_auth_label()),
                    ("token_file", token_file.as_str()),
                    ("token_file_state", token_file_state),
                    ("token_file_perms", token_file_perms),
                    ("inbox_token", inbox_token.as_str()),
                    ("last_test", self.relay_last_test_result.as_str()),
                    ("baseline", validated_front_door_marker()),
                    ("compatibility", compatibility_surface_marker()),
                    ("commands_gap", "2"),
                    (
                        "sections",
                        "relay_status,transport,auth,token_file,inbox_token,test,baseline,commands",
                    ),
                ],
            );
        }
        if self.inspector == TuiInspectorPane::Contacts {
            let self_alias = if self.is_locked() {
                "hidden"
            } else {
                self.account_alias_cache.as_str()
            };
            emit_marker(
                "tui_contacts_view",
                None,
                &[
                    ("selected", self.selected_contact_label().as_str()),
                    ("self_alias", self_alias),
                ],
            );
        }
        if self.inspector == TuiInspectorPane::CmdResults {
            let count_s = self.cmd_results.len().to_string();
            let (last_status, last_command, _last_detail) = self
                .cmd_results
                .back()
                .map(|entry| split_cmd_result_entry(entry.as_str()))
                .unwrap_or(("none", "none", "none"));
            let last_command_marker = last_command.replace(' ', "_");
            emit_marker(
                "tui_cmd_results_view",
                None,
                &[
                    ("count", count_s.as_str()),
                    ("sections", "last"),
                    ("last_command", last_command_marker.as_str()),
                    ("last_status", last_status),
                ],
            );
        }
        if self.inspector == TuiInspectorPane::Lock {
            let effect = if self.status.locked == "UNLOCKED" {
                "sensitive_content_displayed"
            } else {
                "sensitive_content_redacted"
            };
            let minutes_s = self.autolock_minutes().to_string();
            let attempt_limit = vault_attempt_limit_note(self.unlock_attempt_limit);
            let attempt_limit_mode = if self.unlock_attempt_limit.is_some() {
                "threshold_wipe"
            } else {
                "off"
            };
            let attempt_limit_threshold = self
                .unlock_attempt_limit
                .map(|value| value.to_string())
                .unwrap_or_else(|| "0".to_string());
            let failed_unlocks = self.failed_unlock_attempts.to_string();
            emit_marker(
                "tui_lock_view",
                None,
                &[
                    ("locked", self.status.locked),
                    ("redacted", if self.is_locked() { "true" } else { "false" }),
                    ("sections", "state,effect,autolock,vault_security,commands"),
                    ("title", "Lock Status"),
                    ("state", self.status.locked),
                    ("effect", effect),
                    ("autolock_minutes", minutes_s.as_str()),
                    ("vault_attempt_limit", attempt_limit.as_str()),
                    ("vault_attempt_limit_mode", attempt_limit_mode),
                    (
                        "vault_attempt_limit_threshold",
                        attempt_limit_threshold.as_str(),
                    ),
                    ("vault_attempt_limit_scope", "vault_and_state"),
                    ("failed_unlock_attempts", failed_unlocks.as_str()),
                    ("recovery", "rerun_init_if_wiped"),
                    ("commands", "lock,autolock_show,autolock_set"),
                ],
            );
        }
        if self.inspector == TuiInspectorPane::Status {
            let redacted = if self.status.locked == "LOCKED" {
                "true"
            } else {
                "false"
            };
            let attachment_service_active = validated_attachment_service_from_env().is_some();
            let minutes_s = self.autolock_minutes().to_string();
            let poll_interval_s = self.poll_interval_seconds().to_string();
            let receipt_batch_window_s = self.receipt_policy.batch_window_ms.to_string();
            let receipt_jitter_s = self.receipt_policy.jitter_ms.to_string();
            let peer_identity = self.selected_peer_identity_short();
            let setup_token_file = self.relay_token_file_redacted();
            emit_marker(
                "tui_status_view",
                None,
                &[
                    ("locked", self.status.locked),
                    ("redacted", redacted),
                    ("autolock_minutes", minutes_s.as_str()),
                    ("poll_mode", self.poll_mode().as_str()),
                    ("poll_interval_seconds", poll_interval_s.as_str()),
                    ("receipt_mode", self.receipt_policy.mode.as_str()),
                    (
                        "file_confirm_mode",
                        self.receipt_policy.file_confirm_mode.as_str(),
                    ),
                    ("receipt_batch_window_ms", receipt_batch_window_s.as_str()),
                    ("receipt_jitter_ms", receipt_jitter_s.as_str()),
                    ("last_result", self.status_last_command_result_text()),
                    (
                        "qsp_note",
                        qsp_status_user_note(qsp_status_parts(self.status.qsp).1),
                    ),
                    ("baseline", validated_front_door_marker()),
                    ("compatibility", compatibility_surface_marker()),
                    (
                        "migration",
                        migration_posture_marker(attachment_service_active),
                    ),
                    ("peer_trust", self.selected_peer_trust_state()),
                    ("peer_identity", peer_identity.as_str()),
                    ("setup_token_file", setup_token_file.as_str()),
                    ("sections", "system_overview,snapshot,transport,setup,queue"),
                ],
            );
        }
    }

    fn focus_render_count(&self, mode: TuiMode) -> usize {
        match mode {
            TuiMode::FocusEvents => self.focus_events_lines().len(),
            TuiMode::FocusFiles => self.focus_files_lines().len(),
            TuiMode::FocusActivity => self.focus_activity_lines().len(),
            TuiMode::FocusContacts => self.contacts.len(),
            TuiMode::FocusStatus => self.focus_status_lines().len(),
            TuiMode::FocusSession => self.focus_session_lines().len(),
            TuiMode::FocusSettings => self.focus_settings_lines().len(),
            TuiMode::FocusLock => self.focus_lock_lines().len(),
            _ => 0,
        }
    }

    fn enter_focus_mode(&mut self, mode: TuiMode) {
        if self.mode == mode {
            return;
        }
        if matches!(
            self.mode,
            TuiMode::FocusEvents
                | TuiMode::FocusFiles
                | TuiMode::FocusActivity
                | TuiMode::FocusStatus
                | TuiMode::FocusSession
                | TuiMode::FocusContacts
                | TuiMode::FocusSettings
                | TuiMode::FocusLock
        ) {
            let pane = TuiState::focus_pane_name(self.mode);
            emit_marker("tui_focus", None, &[("pane", pane), ("on", "false")]);
        }
        self.mode = mode;
        self.focus_scroll = 0;
        self.contacts_selected = 0;
        let pane = TuiState::focus_pane_name(self.mode);
        emit_marker("tui_focus", None, &[("pane", pane), ("on", "true")]);
        let count = self.focus_render_count(self.mode);
        emit_marker(
            "tui_focus_rendered",
            None,
            &[("pane", pane), ("count", count.to_string().as_str())],
        );
        self.emit_focus_render_marker();
    }

    fn exit_focus_mode(&mut self) {
        if matches!(
            self.mode,
            TuiMode::FocusEvents
                | TuiMode::FocusFiles
                | TuiMode::FocusActivity
                | TuiMode::FocusStatus
                | TuiMode::FocusSession
                | TuiMode::FocusContacts
                | TuiMode::FocusSettings
                | TuiMode::FocusLock
        ) {
            let pane = TuiState::focus_pane_name(self.mode);
            emit_marker("tui_focus", None, &[("pane", pane), ("on", "false")]);
            self.mode = TuiMode::Normal;
        }
    }

    fn is_help_mode(&self) -> bool {
        self.mode == TuiMode::Help
    }

    fn is_focus_mode(&self) -> bool {
        matches!(
            self.mode,
            TuiMode::FocusEvents
                | TuiMode::FocusFiles
                | TuiMode::FocusActivity
                | TuiMode::FocusStatus
                | TuiMode::FocusSession
                | TuiMode::FocusContacts
                | TuiMode::FocusSettings
                | TuiMode::FocusLock
        )
    }

    fn focus_max_len(&self) -> usize {
        match self.mode {
            TuiMode::FocusEvents => self.focus_events_lines().len(),
            TuiMode::FocusFiles => self.focus_files_lines().len(),
            TuiMode::FocusActivity => self.focus_activity_lines().len(),
            TuiMode::FocusContacts => self.contacts.len(),
            TuiMode::FocusStatus => self.focus_status_lines().len(),
            TuiMode::FocusSession => self.focus_session_lines().len(),
            TuiMode::FocusSettings => self.focus_settings_lines().len(),
            TuiMode::FocusLock => self.focus_lock_lines().len(),
            _ => 0,
        }
    }

    fn focus_view_rows(&self) -> usize {
        usize::from(terminal_rows_for_headless().saturating_sub(2)).max(1)
    }

    fn focus_scroll_index(&self) -> usize {
        match self.mode {
            TuiMode::FocusContacts => self.contacts_selected,
            TuiMode::FocusFiles => self.file_selected,
            _ => self.focus_scroll,
        }
    }

    fn focus_events_lines(&self) -> Vec<String> {
        self.events
            .iter()
            .enumerate()
            .map(|(i, line)| format!("{} {}", tui_timestamp_token(i), line))
            .collect()
    }

    fn focus_activity_lines(&self) -> Vec<String> {
        self.focus_events_lines()
    }

    fn focus_status_lines(&self) -> Vec<String> {
        let locked = self.status.locked == "LOCKED";
        let (qsp_state, qsp_reason) = qsp_status_parts(self.status.qsp);
        let attachment_service_active = validated_attachment_service_from_env().is_some();
        let poll_interval_s = self.poll_interval_seconds().to_string();
        let own_fp = if locked {
            "hidden (unlock required)".to_string()
        } else {
            short_identity_display(self.status.fingerprint)
        };
        let peer_fp = if locked {
            "hidden (unlock required)".to_string()
        } else {
            self.selected_peer_identity_short()
        };
        let peer_trust = self.selected_peer_trust_state();
        [
            format!("vault locked: {}", self.status.locked),
            format!("vault access: {}", vault_access_note(locked)),
            format!("fingerprint: {}", own_fp),
            format!("peer fp: {}", peer_fp),
            format!("peer trust: {}", peer_trust),
            format!("session state: {}", qsp_state),
            format!("session reason: {}", qsp_reason),
            format!("session note: {}", qsp_status_user_note(qsp_reason)),
            format!("envelope: {}", self.status.envelope),
            format!("send: {}", self.status.send_lifecycle),
            format!("poll mode: {}", self.poll_mode().as_str()),
            format!("poll interval seconds: {}", poll_interval_s),
            format!("trust mode: {}", self.trust_onboarding_mode.as_str()),
            format!("receipt mode: {}", self.receipt_policy.mode.as_str()),
            format!(
                "file confirm mode: {}",
                self.receipt_policy.file_confirm_mode.as_str()
            ),
            format!("baseline: {}", validated_front_door_note()),
            format!(
                "migration note: {}",
                migration_posture_note(attachment_service_active)
            ),
        ]
        .into_iter()
        .enumerate()
        .map(|(i, line)| format!("{} {}", tui_timestamp_token(i), line))
        .collect()
    }

    fn focus_session_lines(&self) -> Vec<String> {
        let locked = self.is_locked();
        [
            "domain: keys".to_string(),
            format!("selected_peer: {}", self.session.peer_label),
            format!(
                "verified: {}",
                if locked {
                    "hidden (unlock required)".to_string()
                } else {
                    self.session.verified.to_string()
                }
            ),
            if locked {
                "identity_metadata: hidden (unlock required)".to_string()
            } else {
                "identity_metadata: visible".to_string()
            },
            "dangerous_ops: command_bar_only".to_string(),
        ]
        .into_iter()
        .enumerate()
        .map(|(i, line)| format!("{} {}", tui_timestamp_token(i), line))
        .collect()
    }

    fn focus_contacts_lines(&self) -> Vec<String> {
        self.contacts
            .iter()
            .enumerate()
            .map(|(i, peer)| {
                format!(
                    "{} {}",
                    tui_timestamp_token(i),
                    self.contact_display_line_cached(peer)
                )
            })
            .collect()
    }

    fn focus_files_lines(&self) -> Vec<String> {
        let locked = self.is_locked();
        self.files
            .iter()
            .enumerate()
            .map(|(i, item)| {
                let selected = if self.file_multi_selected.contains(item.id.as_str()) {
                    "[x]"
                } else {
                    "[ ]"
                };
                format!(
                    "{} {} id={} peer={} size={} state={} name={}",
                    tui_timestamp_token(i),
                    selected,
                    item.id,
                    item.peer,
                    item.byte_len,
                    item.display_state,
                    if locked {
                        "hidden (unlock required)"
                    } else {
                        item.filename.as_str()
                    }
                )
            })
            .collect()
    }

    fn focus_settings_lines(&self) -> Vec<String> {
        let poll_interval = if self.poll_mode() == TuiPollMode::Fixed {
            self.poll_interval_seconds().to_string()
        } else {
            "n/a".to_string()
        };
        let attempt_limit = vault_attempt_limit_note(self.unlock_attempt_limit);
        [
            "settings".to_string(),
            String::new(),
            format!("lock state: {}", self.status.locked),
            format!("autolock timeout minutes: {}", self.autolock_minutes()),
            format!("poll mode: {}", self.poll_mode().as_str()),
            format!("poll interval seconds: {}", poll_interval),
            format!("trust mode: {}", self.trust_onboarding_mode.as_str()),
            format!("receipt mode: {}", self.receipt_policy.mode.as_str()),
            format!(
                "receipt batch window ms: {}",
                self.receipt_policy.batch_window_ms
            ),
            format!("receipt jitter ms: {}", self.receipt_policy.jitter_ms),
            format!(
                "file confirm mode: {}",
                self.receipt_policy.file_confirm_mode.as_str()
            ),
            format!("vault attempt limit: {}", attempt_limit),
            format!(
                "failed unlock attempts since last success: {}",
                self.failed_unlock_attempts
            ),
            "commands: /status /autolock show /autolock set <minutes> /poll show /poll set adaptive /poll set fixed <seconds>".to_string(),
        ]
        .into_iter()
        .enumerate()
        .map(|(i, line)| format!("{} {}", tui_timestamp_token(i), line))
        .collect()
    }

    fn focus_lock_lines(&self) -> Vec<String> {
        let attempt_limit = vault_attempt_limit_note(self.unlock_attempt_limit);
        [
            "domain: lock".to_string(),
            format!("state: {}", self.status.locked),
            format!(
                "vault: {}",
                if self.has_vault() {
                    "present"
                } else {
                    "missing"
                }
            ),
            "redaction: sensitive_content_hidden_when_locked".to_string(),
            format!("attempt limit: {}", attempt_limit),
            format!("failed unlock attempts: {}", self.failed_unlock_attempts),
            "recovery: rerun /init if the wipe threshold is reached".to_string(),
            "commands: /lock /unlock".to_string(),
        ]
        .into_iter()
        .enumerate()
        .map(|(i, line)| format!("{} {}", tui_timestamp_token(i), line))
        .collect()
    }

    fn emit_focus_render_marker(&self) {
        if !self.is_focus_mode() {
            return;
        }
        let pane = TuiState::focus_pane_name(self.mode);
        let count = self.focus_max_len();
        let scroll = self.focus_scroll_index();
        let view_rows = self.focus_view_rows();
        let ts_start_idx = if count == 0 { 0 } else { scroll.min(count - 1) };
        let ts_end_idx = if count == 0 {
            0
        } else {
            count
                .min(scroll.saturating_add(view_rows))
                .saturating_sub(1)
        };
        let scroll_s = scroll.to_string();
        let count_s = count.to_string();
        let view_rows_s = view_rows.to_string();
        let ts_start = tui_timestamp_token(ts_start_idx);
        let ts_end = tui_timestamp_token(ts_end_idx);
        emit_marker(
            "tui_render",
            None,
            &[
                ("mode", "focus"),
                ("layout", "h3"),
                ("focus", pane),
                ("viewport", "full"),
                ("scroll", scroll_s.as_str()),
                ("count", count_s.as_str()),
                ("view_rows", view_rows_s.as_str()),
                ("ts_start", ts_start.as_str()),
                ("ts_end", ts_end.as_str()),
                (
                    "deterministic",
                    if tui_deterministic_timestamps() {
                        "true"
                    } else {
                        "false"
                    },
                ),
            ],
        );
    }

    fn help_selected_item(&self) -> Option<&'static TuiHelpItem> {
        let items = tui_help_items();
        if items.is_empty() {
            None
        } else {
            Some(&items[self.help_selected.min(items.len() - 1)])
        }
    }

    fn help_move(&mut self, delta: i32) {
        let items = tui_help_items();
        if items.is_empty() {
            self.help_selected = 0;
            return;
        }
        let len = items.len() as i32;
        let mut idx = self.help_selected as i32 + delta;
        if idx < 0 {
            idx = 0;
        }
        if idx >= len {
            idx = len - 1;
        }
        self.help_selected = idx as usize;
    }

    fn focus_scroll_move(&mut self, delta: i32, max_len: usize) {
        if max_len == 0 {
            self.focus_scroll = 0;
            self.emit_focus_render_marker();
            return;
        }
        let max = (max_len.saturating_sub(1)) as i32;
        let mut idx = self.focus_scroll as i32 + delta;
        if idx < 0 {
            idx = 0;
        }
        if idx > max {
            idx = max;
        }
        self.focus_scroll = idx as usize;
        self.emit_focus_render_marker();
    }

    fn contacts_move(&mut self, delta: i32) {
        if self.contacts.is_empty() {
            self.contacts_selected = 0;
            self.emit_focus_render_marker();
            return;
        }
        let max = (self.contacts.len() - 1) as i32;
        let mut idx = self.contacts_selected as i32 + delta;
        if idx < 0 {
            idx = 0;
        }
        if idx > max {
            idx = max;
        }
        self.contacts_selected = idx as usize;
        self.emit_focus_render_marker();
    }

    fn nav_move(&mut self, delta: i32) {
        if self.home_focus != TuiHomeFocus::Nav {
            return;
        }
        let rows = self.nav_rows();
        if rows.is_empty() {
            self.nav_selected = 0;
            return;
        }
        let len = rows.len() as i32;
        let base = self.nav_selected as i32;
        let idx = (base + delta).rem_euclid(len);
        self.nav_selected = idx as usize;
        self.nav_preview_select(rows[self.nav_selected].kind);
    }

    fn nav_activate(&mut self) {
        if self.home_focus != TuiHomeFocus::Nav {
            return;
        }
        let rows = self.nav_rows();
        if rows.is_empty() {
            return;
        }
        let row = rows[self.nav_selected.min(rows.len().saturating_sub(1))];
        match row.kind {
            NavRowKind::Domain(_) => self.nav_preview_select(row.kind),
            NavRowKind::SystemAccount => self.set_inspector(TuiInspectorPane::Account),
            NavRowKind::SystemRelay => self.set_inspector(TuiInspectorPane::Relay),
            NavRowKind::SystemSettings => self.set_inspector(TuiInspectorPane::Settings),
            NavRowKind::SystemCmdResults => self.set_inspector(TuiInspectorPane::CmdResults),
            NavRowKind::Header(pane) => self.set_inspector(pane),
            NavRowKind::Conversation(idx) => {
                self.set_inspector(TuiInspectorPane::Events);
                self.nav_preview_select(NavRowKind::Conversation(idx));
            }
            NavRowKind::Contact(idx) => {
                self.set_inspector(TuiInspectorPane::Contacts);
                self.nav_preview_select(NavRowKind::Contact(idx));
            }
            NavRowKind::Unlock => {
                emit_marker(
                    "tui_nav_select",
                    None,
                    &[("domain", "locked"), ("label", "unlock")],
                );
            }
            NavRowKind::Exit => {
                emit_marker(
                    "tui_nav_select",
                    None,
                    &[("domain", "locked"), ("label", "exit")],
                );
            }
        }
        emit_marker("tui_nav_activate", None, &[("pane", self.inspector_name())]);
    }

    fn locked_nav_activate(&mut self) -> bool {
        if !self.is_locked() || self.home_focus != TuiHomeFocus::Nav {
            return false;
        }
        self.nav_activate();
        let rows = self.nav_rows();
        if rows.is_empty() {
            return false;
        }
        let row = rows[self.nav_selected.min(rows.len().saturating_sub(1))];
        match row.kind {
            NavRowKind::Exit => {
                emit_marker("tui_cmd", None, &[("cmd", "exit")]);
                true
            }
            NavRowKind::Unlock => {
                if self.has_vault() {
                    self.start_unlock_prompt();
                } else {
                    self.start_init_prompt();
                }
                false
            }
            _ => false,
        }
    }

    fn nav_preview_select(&mut self, kind: NavRowKind) {
        match kind {
            NavRowKind::Domain(domain) => {
                match domain {
                    TuiNavDomain::System => self.set_inspector(TuiInspectorPane::Status),
                    TuiNavDomain::Contacts => self.set_inspector(TuiInspectorPane::Contacts),
                    TuiNavDomain::Messages => self.set_inspector(TuiInspectorPane::Events),
                }
                emit_marker(
                    "tui_nav_select",
                    None,
                    &[(
                        "domain",
                        match domain {
                            TuiNavDomain::System => "system",
                            TuiNavDomain::Contacts => "contacts",
                            TuiNavDomain::Messages => "messages",
                        },
                    )],
                );
            }
            NavRowKind::SystemAccount => {
                self.set_inspector(TuiInspectorPane::Account);
                emit_marker(
                    "tui_nav_select",
                    None,
                    &[("domain", "system"), ("label", "account")],
                );
            }
            NavRowKind::SystemRelay => {
                self.set_inspector(TuiInspectorPane::Relay);
                emit_marker(
                    "tui_nav_select",
                    None,
                    &[("domain", "system"), ("label", "relay")],
                );
            }
            NavRowKind::SystemSettings => {
                self.set_inspector(TuiInspectorPane::Settings);
                emit_marker(
                    "tui_nav_select",
                    None,
                    &[("domain", "system"), ("label", "settings")],
                );
            }
            NavRowKind::SystemCmdResults => {
                self.set_inspector(TuiInspectorPane::CmdResults);
                emit_marker(
                    "tui_nav_select",
                    None,
                    &[("domain", "system"), ("label", "cmd_results")],
                );
            }
            NavRowKind::Header(pane) => {
                self.inspector = pane;
                self.sync_nav_to_inspector_header();
                self.sync_messages_if_main_focused();
                self.sync_files_if_main_focused();
                self.sync_activity_if_main_focused();
                emit_marker("tui_inspector", None, &[("pane", self.inspector_name())]);
                emit_marker(
                    "tui_nav_select",
                    None,
                    &[
                        ("domain", Self::pane_domain_name(pane)),
                        ("label", pane.as_name()),
                    ],
                );
            }
            NavRowKind::Conversation(idx) => {
                let labels = self.conversation_labels();
                if labels.is_empty() {
                    self.conversation_selected = 0;
                    return;
                }
                self.conversation_selected = idx.min(labels.len().saturating_sub(1));
                let selected = self.selected_conversation_label();
                self.set_active_peer(selected.as_str());
                self.sync_messages_if_main_focused();
                let selected_marker = selected.replace(' ', "_");
                emit_marker(
                    "tui_nav_select",
                    None,
                    &[("domain", "messages"), ("label", selected_marker.as_str())],
                );
            }
            NavRowKind::Contact(idx) => {
                if self.contacts.is_empty() {
                    self.contacts_selected = 0;
                    return;
                }
                self.contacts_selected = idx.min(self.contacts.len().saturating_sub(1));
                let selected = self.selected_contact_label();
                emit_marker(
                    "tui_nav_select",
                    None,
                    &[("domain", "contacts"), ("label", selected.as_str())],
                );
            }
            NavRowKind::Unlock => {
                emit_marker(
                    "tui_nav_select",
                    None,
                    &[("domain", "locked"), ("label", "unlock")],
                );
            }
            NavRowKind::Exit => {
                emit_marker(
                    "tui_nav_select",
                    None,
                    &[("domain", "locked"), ("label", "exit")],
                );
            }
        }
    }

    fn pane_domain_name(pane: TuiInspectorPane) -> &'static str {
        match pane {
            TuiInspectorPane::Events => "messages",
            TuiInspectorPane::Files => "files",
            TuiInspectorPane::Activity => "activity",
            TuiInspectorPane::Status => "system",
            TuiInspectorPane::Account => "system",
            TuiInspectorPane::Relay => "system",
            TuiInspectorPane::CmdResults => "system",
            TuiInspectorPane::Session => "keys",
            TuiInspectorPane::Contacts => "contacts",
            TuiInspectorPane::Settings => "system",
            TuiInspectorPane::Lock => "lock",
            TuiInspectorPane::Help => "help",
            TuiInspectorPane::About => "about",
            TuiInspectorPane::Legal => "legal",
        }
    }

    fn nav_row_label(&self, row: &NavRow) -> String {
        match row.kind {
            NavRowKind::Domain(TuiNavDomain::System) => "system".to_string(),
            NavRowKind::Domain(TuiNavDomain::Contacts) => "contacts".to_string(),
            NavRowKind::Domain(TuiNavDomain::Messages) => "messages".to_string(),
            NavRowKind::SystemAccount => "account".to_string(),
            NavRowKind::SystemRelay => "relay".to_string(),
            NavRowKind::SystemSettings => "settings".to_string(),
            NavRowKind::SystemCmdResults => "results".to_string(),
            NavRowKind::Header(_) => "header".to_string(),
            NavRowKind::Conversation(item_idx) => self
                .conversation_labels()
                .get(item_idx)
                .cloned()
                .unwrap_or_else(|| "none".to_string()),
            NavRowKind::Contact(item_idx) => self
                .contacts
                .get(item_idx)
                .cloned()
                .unwrap_or_else(|| "none".to_string()),
            NavRowKind::Unlock => "unlock".to_string(),
            NavRowKind::Exit => "exit".to_string(),
        }
    }

    fn expanded_nav_domain(&self) -> Option<TuiNavDomain> {
        match self.inspector {
            TuiInspectorPane::Status
            | TuiInspectorPane::Account
            | TuiInspectorPane::Relay
            | TuiInspectorPane::Settings
            | TuiInspectorPane::CmdResults => Some(TuiNavDomain::System),
            TuiInspectorPane::Contacts => Some(TuiNavDomain::Contacts),
            TuiInspectorPane::Events => Some(TuiNavDomain::Messages),
            _ => None,
        }
    }

    fn nav_rows(&self) -> Vec<NavRow> {
        if self.is_locked() {
            return vec![
                NavRow {
                    kind: NavRowKind::Unlock,
                },
                NavRow {
                    kind: NavRowKind::Exit,
                },
            ];
        }
        let expanded = self.expanded_nav_domain();
        let mut rows = Vec::new();
        rows.push(NavRow {
            kind: NavRowKind::Domain(TuiNavDomain::System),
        });
        if expanded == Some(TuiNavDomain::System) {
            rows.push(NavRow {
                kind: NavRowKind::SystemAccount,
            });
            rows.push(NavRow {
                kind: NavRowKind::SystemRelay,
            });
            rows.push(NavRow {
                kind: NavRowKind::SystemSettings,
            });
            rows.push(NavRow {
                kind: NavRowKind::SystemCmdResults,
            });
        }
        rows.push(NavRow {
            kind: NavRowKind::Domain(TuiNavDomain::Contacts),
        });
        if expanded == Some(TuiNavDomain::Contacts) {
            for idx in 0..self.contacts.len().min(4) {
                rows.push(NavRow {
                    kind: NavRowKind::Contact(idx),
                });
            }
        }
        rows.push(NavRow {
            kind: NavRowKind::Domain(TuiNavDomain::Messages),
        });
        if expanded == Some(TuiNavDomain::Messages) {
            for idx in 0..self.conversation_labels().len().min(6) {
                rows.push(NavRow {
                    kind: NavRowKind::Conversation(idx),
                });
            }
        }
        for pane in [
            TuiInspectorPane::Activity,
            TuiInspectorPane::Session,
            TuiInspectorPane::Help,
            TuiInspectorPane::About,
            TuiInspectorPane::Legal,
        ] {
            rows.push(NavRow {
                kind: NavRowKind::Header(pane),
            });
        }
        rows
    }

    fn sync_nav_to_inspector_header(&mut self) {
        if self.is_locked() {
            self.nav_selected = 0;
            return;
        }
        let rows = self.nav_rows();
        self.nav_selected = match self.inspector {
            TuiInspectorPane::Status => rows
                .iter()
                .position(|row| matches!(row.kind, NavRowKind::Domain(TuiNavDomain::System)))
                .unwrap_or(0),
            TuiInspectorPane::Account => rows
                .iter()
                .position(|row| matches!(row.kind, NavRowKind::SystemAccount))
                .unwrap_or(0),
            TuiInspectorPane::Relay => rows
                .iter()
                .position(|row| matches!(row.kind, NavRowKind::SystemRelay))
                .unwrap_or(0),
            TuiInspectorPane::Settings => rows
                .iter()
                .position(|row| matches!(row.kind, NavRowKind::SystemSettings))
                .unwrap_or(0),
            TuiInspectorPane::CmdResults => rows
                .iter()
                .position(|row| matches!(row.kind, NavRowKind::SystemCmdResults))
                .unwrap_or(0),
            TuiInspectorPane::Contacts => rows
                .iter()
                .position(|row| matches!(row.kind, NavRowKind::Domain(TuiNavDomain::Contacts)))
                .unwrap_or(0),
            TuiInspectorPane::Events => rows
                .iter()
                .position(|row| matches!(row.kind, NavRowKind::Domain(TuiNavDomain::Messages)))
                .unwrap_or(0),
            pane => rows
                .iter()
                .position(|row| matches!(row.kind, NavRowKind::Header(p) if p == pane))
                .unwrap_or(0),
        };
    }

    fn drain_marker_queue(&mut self) {
        self.refresh_files_from_timeline();
        let mut queue = marker_queue().lock().expect("marker queue lock");
        while let Some(line) = queue.pop_front() {
            if line.contains("QSC_MARK/1") || line.contains("event=tui_nav_render") {
                continue;
            }
            self.push_event_line(line);
        }
    }
}

struct TuiHelpItem {
    cmd: &'static str,
    desc: &'static str,
}

fn tui_help_items() -> &'static [TuiHelpItem] {
    &[
        TuiHelpItem {
            cmd: "help",
            desc: "show commands",
        },
        TuiHelpItem {
            cmd:
                "inspector status|account|relay|settings|cmdresults|events|session|contacts|lock|help|about|legal",
            desc: "set home inspector pane",
        },
        TuiHelpItem {
            cmd: "focus events",
            desc: "focus Events pane",
        },
        TuiHelpItem {
            cmd: "focus files",
            desc: "focus Files pane",
        },
        TuiHelpItem {
            cmd: "focus activity",
            desc: "focus Activity pane",
        },
        TuiHelpItem {
            cmd: "focus status",
            desc: "focus Status pane",
        },
        TuiHelpItem {
            cmd: "focus session",
            desc: "focus Session pane",
        },
        TuiHelpItem {
            cmd: "focus contacts",
            desc: "focus Contacts pane",
        },
        TuiHelpItem {
            cmd: "focus settings",
            desc: "focus Settings pane",
        },
        TuiHelpItem {
            cmd: "focus lock",
            desc: "focus Lock pane",
        },
        TuiHelpItem {
            cmd: "contacts list|block <alias>|unblock <alias>|add <alias> <verification code> [route token]|route set <alias> <route token>",
            desc: "manage contact states",
        },
        TuiHelpItem {
            cmd: "verify <alias> <verification code>",
            desc: "verify stored contact code (mismatch routes to Results)",
        },
        TuiHelpItem {
            cmd: "trust pin <alias> confirm",
            desc: "pin trusted peer after out-of-band verification",
        },
        TuiHelpItem {
            cmd: "messages list|select <peer>",
            desc: "manage conversation selection",
        },
        TuiHelpItem {
            cmd: "files list|select <id>|toggle <id?>|clear-selection|inject <id> <state>",
            desc: "manage files view and multi-select in Files domain only",
        },
        TuiHelpItem {
            cmd: "injectmsg <peer> [STATE]",
            desc: "headless-only deterministic message injection",
        },
        TuiHelpItem {
            cmd: "injectevent <kind> <action>",
            desc: "headless-only deterministic activity event injection",
        },
        TuiHelpItem {
            cmd: "back",
            desc: "exit focus mode",
        },
        TuiHelpItem {
            cmd: "exit",
            desc: "exit TUI",
        },
        TuiHelpItem {
            cmd: "exithelp",
            desc: "exit help mode",
        },
        TuiHelpItem {
            cmd: "send",
            desc: "send via explicit transport",
        },
        TuiHelpItem {
            cmd: "handshake status",
            desc: "show handshake status",
        },
        TuiHelpItem {
            cmd: "handshake init",
            desc: "initiate handshake to peer",
        },
        TuiHelpItem {
            cmd: "handshake poll",
            desc: "poll inbox for handshake",
        },
        TuiHelpItem {
            cmd: "status",
            desc: "refresh status",
        },
        TuiHelpItem {
            cmd: "autolock show|set <minutes>",
            desc: "view or set inactivity lock timeout (minutes)",
        },
        TuiHelpItem {
            cmd: "poll show|set adaptive|set fixed <seconds>",
            desc: "view or set optional fixed poll cadence",
        },
        TuiHelpItem {
            cmd: "msg \"<text>\"|msg <peer> \"<text>\"",
            desc: "send message to selected thread or explicit peer",
        },
        TuiHelpItem {
            cmd: "relay show|set endpoint <url>|set token <token>|set token-file <path>|inbox set <token>|clear|clear token|clear inbox|test",
            desc: "configure/test relay endpoint with redacted output",
        },
        TuiHelpItem {
            cmd: "vault where|attempt_limit show|attempt_limit set <N>|attempt_limit clear",
            desc: "show vault path or configure failed-unlock wipe option",
        },
        TuiHelpItem {
            cmd: "device show",
            desc: "show local device mode/id summary",
        },
        TuiHelpItem {
            cmd: "lock",
            desc: "explicitly lock and redact sensitive content",
        },
        TuiHelpItem {
            cmd: "unlock",
            desc: "explicitly unlock using configured vault auth",
        },
        TuiHelpItem {
            cmd: "envelope",
            desc: "refresh envelope",
        },
        TuiHelpItem {
            cmd: "export",
            desc: "export redacted diagnostics",
        },
    ]
}

fn render_main_panel(f: &mut ratatui::Frame, area: Rect, state: &mut TuiState) {
    if state.is_locked() {
        let body = pad_panel_text(state.locked_main_body().as_str());
        let main_first_line = body
            .lines()
            .find(|line| !line.trim().is_empty())
            .unwrap_or("none")
            .replace(' ', "_");
        let panel = Paragraph::new(body);
        f.render_widget(panel, area);
        emit_marker(
            "tui_main_render",
            None,
            &[("pad", "2"), ("first_line", main_first_line.as_str())],
        );
        return;
    }
    let body = match state.inspector {
        TuiInspectorPane::Events => {
            let peer = state.selected_conversation_label();
            let stream = state.conversations.get(peer.as_str());
            let total = stream.map(|v| v.len()).unwrap_or(0usize);
            let visible = state
                .visible_counts
                .get(peer.as_str())
                .copied()
                .unwrap_or(total)
                .min(total);
            if total == 0 {
                if peer == TUI_NOTE_TO_SELF_LABEL {
                    "Messages Overview\n\nThread: Note to Self\n\nNo messages yet.\nUse command bar: /msg \"<text>\"."
                        .to_string()
                } else {
                    format!(
                        "Messages Overview\n\nThread: {peer}\n\nNo messages yet.\nUse command bar: /msg \"<text>\"."
                    )
                }
            } else {
                let mut lines = Vec::new();
                lines.push("Messages Overview".to_string());
                lines.push(String::new());
                lines.push(format!("Thread: {}", peer));
                lines.push(String::new());
                if let Some(entries) = stream {
                    for line in entries.iter().take(visible) {
                        lines.push(line.clone());
                    }
                }
                if visible < total {
                    lines.push(String::new());
                    lines.push(format!(
                        "(buffered: {} unread; focus Main on Messages to append)",
                        total - visible
                    ));
                }
                lines.join("\n")
            }
        }
        TuiInspectorPane::Files => {
            if state.files.is_empty() {
                "Files\n\nNo file transfers yet.\nUse command bar only for actions.".to_string()
            } else {
                let selected = state
                    .files
                    .get(state.file_selected.min(state.files.len().saturating_sub(1)));
                let mut lines = Vec::new();
                lines.push(format!(
                    "files: {} ({} selected)",
                    state.files.len(),
                    state.file_multi_selected.len()
                ));
                lines.push(String::new());
                if let Some(item) = selected {
                    lines.push(format!("id: {}", item.id));
                    lines.push(format!(
                        "peer: {}",
                        if state.is_locked() {
                            "hidden (unlock required)"
                        } else {
                            item.peer.as_str()
                        }
                    ));
                    lines.push(format!(
                        "name: {}",
                        if state.is_locked() {
                            "hidden (unlock required)"
                        } else {
                            item.filename.as_str()
                        }
                    ));
                    lines.push(format!("size: {} bytes", item.byte_len));
                    lines.push(format!("state: {}", item.display_state));
                    lines.push("at_rest: encrypted(vault timeline)".to_string());
                } else {
                    lines.push("selected: none".to_string());
                }
                if state.file_unseen_updates > 0 && state.home_focus != TuiHomeFocus::Main {
                    lines.push(String::new());
                    lines.push(format!(
                        "(buffered updates: {}; focus Main on Files to clear)",
                        state.file_unseen_updates
                    ));
                }
                lines.push(String::new());
                lines.push("Commands (command bar only)".to_string());
                lines.push("- /files list".to_string());
                lines.push("- /files select <id>".to_string());
                lines.push("- /files toggle <id?>".to_string());
                lines.push("- /files clear-selection".to_string());
                lines
                    .push("- /files inject <id> <state> [size] [name] (headless test)".to_string());
                lines.join("\n")
            }
        }
        TuiInspectorPane::Activity => {
            let total = state.events.len();
            let visible = state.activity_visible_count.min(total);
            let mut lines = Vec::new();
            lines.push("Activity".to_string());
            lines.push(String::new());
            lines.push(format!(
                "ledger: {} (visible={} unread={})",
                total, visible, state.activity_unseen_updates
            ));
            lines.push(String::new());
            for line in state.events.iter().take(visible) {
                lines.push(line.clone());
            }
            if visible < total {
                lines.push(String::new());
                lines.push(format!(
                    "(buffered: {} events; focus Main on Activity to append)",
                    total - visible
                ));
            }
            lines.push(String::new());
            lines.push("Commands (command bar only)".to_string());
            lines.push("- /focus activity".to_string());
            lines.join("\n")
        }
        TuiInspectorPane::Status => {
            let locked = state.status.locked == "LOCKED";
            let (qsp_state, qsp_reason) = qsp_status_parts(state.status.qsp);
            let attachment_service_active = validated_attachment_service_from_env().is_some();
            let own_fp = if locked {
                "hidden (unlock required)".to_string()
            } else {
                short_identity_display(state.status.fingerprint)
            };
            let peer_fp = if locked {
                "hidden (unlock required)".to_string()
            } else {
                state.selected_peer_identity_short()
            };
            let poll_interval_s = state.poll_interval_seconds().to_string();
            let receipt_batch_window_s = state.receipt_policy.batch_window_ms.to_string();
            let receipt_jitter_s = state.receipt_policy.jitter_ms.to_string();
            let last_result = state.status_last_command_result_text();
            let peer_trust = state.selected_peer_trust_state();
            let (token_file_state, token_file_perms) = state.relay_token_file_status();
            format!(
                "System Overview\n\nlocked: {}\nvault access: {}\nautolock minutes: {}\npoll mode: {}\npoll interval seconds: {}\nreceipt mode: {}\nreceipt batch window ms: {}\nreceipt jitter ms: {}\nfile confirm mode: {}\nlast command result: {}\n\nSession Snapshot\n\nsession state: {}\nsession reason: {}\nsession note: {}\nown fp12: {}\npeer fp12: {}\npeer trust: {}\nsend: {}\ncounts: sent={} recv={}\n\nConnection Setup\n\nrelay endpoint: {}\nauth source: {}\ntoken file: {} (state={} perms={})\nauth check: {}\n\nValidated Lane\n\nbaseline: {}\ncompatibility: {}\nmigration posture: {}",
                state.status.locked,
                vault_access_note(locked),
                state.autolock_minutes(),
                state.poll_mode().as_str(),
                poll_interval_s,
                state.receipt_policy.mode.as_str(),
                receipt_batch_window_s,
                receipt_jitter_s,
                state.receipt_policy.file_confirm_mode.as_str(),
                last_result,
                qsp_state,
                qsp_reason,
                qsp_status_user_note(qsp_reason),
                own_fp,
                peer_fp,
                peer_trust,
                state.status.send_lifecycle,
                state.session.sent_count,
                state.session.recv_count,
                state.relay_endpoint_redacted(),
                state.relay_auth_label(),
                state.relay_token_file_redacted(),
                token_file_state,
                token_file_perms,
                state.relay_last_test_result,
                validated_front_door_note(),
                compatibility_surface_note(),
                migration_posture_note(attachment_service_active)
            )
        }
        TuiInspectorPane::Account => {
            let alias = if state.is_locked() {
                "hidden (unlock required)".to_string()
            } else {
                state.account_alias_cache.clone()
            };
            let verification_code = state.account_verification_code_cache.clone();
            let storage_safety = if state.account_storage_safety_cache == "OK" {
                "OK (path perms)".to_string()
            } else {
                state.account_storage_safety_cache.clone()
            };
            let mut lines = vec![
                "Account".to_string(),
                String::new(),
                "Identity:".to_string(),
                format!("  alias: {}", alias),
                format!("  verification code: {}", verification_code),
                String::new(),
                "Vault:".to_string(),
                format!(
                    "  state: {}",
                    if state.is_locked() {
                        "LOCKED"
                    } else {
                        "UNLOCKED"
                    }
                ),
                "  location: hidden (use /vault where)".to_string(),
                format!("  storage safety: {}", storage_safety),
                "  vault: encrypted at rest".to_string(),
                String::new(),
                "Device:".to_string(),
                "  mode: single device".to_string(),
                "  device id: hidden (use /device show)".to_string(),
                String::new(),
                String::new(),
                "Commands:".to_string(),
                "  /account destroy".to_string(),
                "  /vault where".to_string(),
                "  /device show".to_string(),
            ];
            if state.account_destroy_active() {
                lines.push(String::new());
                lines.push("Destroy Vault".to_string());
                match state.account_destroy_flow {
                    AccountDestroyFlow::None => {}
                    AccountDestroyFlow::Passphrase => {
                        lines.push(format!("Passphrase: {}", state.cmd_display_value()));
                    }
                    AccountDestroyFlow::ConfirmDecision { .. } => {
                        lines.push(format!(
                            "Confirm destroy (Y/N): {}",
                            state.cmd_display_value()
                        ));
                    }
                }
                if let Some(err) = state.account_destroy_error.as_ref() {
                    lines.push(format!("error: {}", err));
                }
            }
            lines.join("\n")
        }
        TuiInspectorPane::Relay => {
            let endpoint_redacted = state.relay_endpoint_redacted();
            let endpoint = state.relay_endpoint_cache.as_deref();
            let transport = relay_transport_label(endpoint);
            let tls = relay_tls_label(endpoint);
            let pinning = relay_pinning_label(endpoint);
            let token_file_redacted = state.relay_token_file_redacted();
            let (token_file_state, token_file_perms) = state.relay_token_file_status();
            let inbox_token_redacted = state.relay_inbox_token_redacted();
            let mut lines = vec![
                "Relay".to_string(),
                String::new(),
                format!(
                    "relay status: {}",
                    if endpoint.is_some() {
                        "configured"
                    } else {
                        "not configured"
                    }
                ),
                format!("endpoint: {}", endpoint_redacted),
                format!("transport: {}", transport),
                format!("tls: {}", tls),
                format!("pinning: {}", pinning),
                format!("auth: {}", state.relay_auth_label()),
                format!("token file: {}", token_file_redacted),
                format!("token file state: {}", token_file_state),
                format!("token file perms: {}", token_file_perms),
                format!("inbox token: {}", inbox_token_redacted),
                format!("test status: {}", state.relay_last_test_result),
                format!("validated baseline: {}", validated_front_door_note()),
                format!("compatibility note: {}", compatibility_surface_note()),
                String::new(),
                String::new(),
                "Commands:".to_string(),
                "  /relay show".to_string(),
                "  /relay set endpoint <https://...>".to_string(),
                "  /relay set token <token>".to_string(),
                "  /relay set token-file <path>".to_string(),
                "  /relay inbox set <token>".to_string(),
                "  /relay clear".to_string(),
                "  /relay clear token".to_string(),
                "  /relay clear inbox".to_string(),
                "  /relay test".to_string(),
            ];
            if state.is_locked() {
                lines.push(String::new());
                lines.push("locked: unlock required".to_string());
            }
            lines.join("\n")
        }
        TuiInspectorPane::CmdResults => {
            let mut lines = Vec::new();
            lines.push("Results".to_string());
            lines.push(String::new());
            if let Some(entry) = state.cmd_results.back() {
                let (status, command, detail) = split_cmd_result_entry(entry.as_str());
                lines.push(format!("last command: /{}", command));
                lines.push(format!("status: {}", status));
                lines.push(format!("detail: {}", detail));
            } else if let Some(last) = state.status_last_command_result.as_ref() {
                let (status, command, detail) = split_cmd_result_entry(last.as_str());
                lines.push(format!("last command: /{}", command));
                lines.push(format!("status: {}", status));
                lines.push(format!("detail: {}", detail));
            } else {
                lines.push("No command results yet.".to_string());
            }
            lines.join("\n")
        }
        TuiInspectorPane::Session => {
            let replay_rejects = state
                .events
                .iter()
                .filter(|line| line.contains("ratchet_replay_reject"))
                .count();
            let mut lines = Vec::new();
            lines.push("Keys".to_string());
            lines.push(String::new());
            lines.push(format!("selected_peer: {}", state.session.peer_label));
            lines.push(format!("qsp: {}", state.status.qsp));
            lines.push(format!(
                "verification: {}",
                if state.is_locked() {
                    "hidden (unlock required)"
                } else if state.session.verified {
                    "verified"
                } else {
                    "not_verified"
                }
            ));
            lines.push(format!("replay_rejects: {}", replay_rejects));
            lines.push(String::new());
            lines.push("Metadata".to_string());
            if state.is_locked() {
                lines.push("- identity: hidden (unlock required)".to_string());
                lines.push("- peer key: hidden (unlock required)".to_string());
                lines.push("- transport key: hidden (unlock required)".to_string());
            } else {
                lines.push("- identity: inspection only".to_string());
                lines.push("- peer key: inspection only".to_string());
                lines.push("- transport key: inspection only".to_string());
            }
            lines.push(String::new());
            lines.push("Commands (command bar only)".to_string());
            lines.push("- /verify <alias> <verification code>".to_string());
            lines.push("- /trust pin <alias> confirm".to_string());
            lines.push("- /contacts add <alias> <verification code> [route token]".to_string());
            lines.push("- /contacts route set <alias> <route token>".to_string());
            lines.push("- /contacts block <peer>".to_string());
            lines.join("\n")
        }
        TuiInspectorPane::Contacts => {
            let mut lines = Vec::new();
            lines.push("Contacts".to_string());
            lines.push(String::new());
            let nav_rows = state.nav_rows();
            let nav_kind = nav_rows
                .get(state.nav_selected.min(nav_rows.len().saturating_sub(1)))
                .map(|row| row.kind);
            if matches!(nav_kind, Some(NavRowKind::Domain(TuiNavDomain::Contacts))) {
                lines.push(format_contacts_table_row(
                    "Alias",
                    "Trust",
                    "Blocked",
                    "Last seen",
                ));
                for alias in state.contacts.iter().take(TUI_INSPECTOR_CONTACTS_MAX) {
                    if let Some(rec) = state.contact_record_cached(alias) {
                        let trust = contact_state(Some(rec));
                        let blocked = if rec.blocked { "yes" } else { "no" };
                        let last_seen = rec
                            .seen_at
                            .map(|v| v.to_string())
                            .unwrap_or_else(|| "-".to_string());
                        lines.push(format_contacts_table_row(
                            alias,
                            trust,
                            blocked,
                            last_seen.as_str(),
                        ));
                    } else {
                        lines.push(format_contacts_table_row(alias, "UNVERIFIED", "no", "-"));
                    }
                }
                lines.push(String::new());
                lines.push(String::new());
                lines.push("Commands:".to_string());
                lines.push("  /contacts add <alias> <verification code> [route token]".to_string());
                lines.push("  /contacts route set <alias> <route token>".to_string());
                lines.push("  /verify <alias> <verification code>".to_string());
                lines.push("  /trust pin <alias> confirm".to_string());
                lines.push("  /contacts block <alias>".to_string());
                lines.push("  /contacts unblock <alias>".to_string());
            } else {
                let selected = state.selected_contact_label();
                let rec = state.contact_record_cached(selected.as_str()).cloned();
                let trust = contact_state(rec.as_ref());
                let blocked = rec.as_ref().map(|v| v.blocked).unwrap_or(false);
                let verification_code = if state.is_locked() {
                    "hidden (unlock required)".to_string()
                } else {
                    rec.as_ref()
                        .map(|v| v.fp.clone())
                        .unwrap_or_else(|| "unknown".to_string())
                };
                lines.push(format!("Contact: {}", selected));
                lines.push(String::new());
                lines.push("Trust".to_string());
                lines.push(format!("  state: {}", trust));
                lines.push(format!(
                    "  last verified: {}",
                    rec.as_ref()
                        .and_then(|v| v.seen_at)
                        .map(|v| v.to_string())
                        .unwrap_or_else(|| "-".to_string())
                ));
                lines.push(String::new());
                lines.push("Identity".to_string());
                lines.push(format!("  verification code: {}", verification_code));
                lines.push("  fingerprint: hidden".to_string());
                lines.push(String::new());
                lines.push("Policy".to_string());
                lines.push(format!("  blocked: {}", if blocked { "yes" } else { "no" }));
                lines.push(String::new());
                lines.push("Notes".to_string());
                lines.push("  local only: -".to_string());
                lines.push(String::new());
                lines.push(String::new());
                lines.push("Commands:".to_string());
                lines.push("  /verify <alias> <verification code>".to_string());
                lines.push("  /trust pin <alias> confirm".to_string());
                lines.push("  /contacts route set <alias> <route token>".to_string());
                lines.push("  /contacts block <alias>".to_string());
                lines.push("  /contacts unblock <alias>".to_string());
            }
            lines.join("\n")
        }
        TuiInspectorPane::Settings => {
            let poll_interval = if state.poll_mode() == TuiPollMode::Fixed {
                state.poll_interval_seconds().to_string()
            } else {
                "n/a".to_string()
            };
            let attempt_limit = vault_attempt_limit_note(state.unlock_attempt_limit);
            [
                "System Settings".to_string(),
                String::new(),
                "Lock:".to_string(),
                format!("  state: {}", state.status.locked),
                String::new(),
                "Auto-lock:".to_string(),
                "  enabled by default: true".to_string(),
                format!("  timeout minutes: {}", state.autolock_minutes()),
                String::new(),
                "Polling:".to_string(),
                format!("  mode: {}", state.poll_mode().as_str()),
                format!("  interval seconds: {}", poll_interval),
                String::new(),
                "Vault Security:".to_string(),
                format!("  attempt limit: {}", attempt_limit),
                format!(
                    "  failures since last success: {}",
                    state.failed_unlock_attempts
                ),
                "  recovery: rerun /init if the wipe threshold is reached".to_string(),
                String::new(),
                String::new(),
                "Commands:".to_string(),
                "  /status".to_string(),
                "  /autolock show".to_string(),
                "  /autolock set <minutes>".to_string(),
                "  /poll show".to_string(),
                "  /poll set adaptive".to_string(),
                "  /poll set fixed <seconds>".to_string(),
                "  /vault attempt_limit show".to_string(),
                "  /vault attempt_limit set <N>".to_string(),
                "  /vault attempt_limit clear".to_string(),
                "  /vault where".to_string(),
                "  /device show".to_string(),
            ]
            .join("\n")
        }
        TuiInspectorPane::Lock => {
            let attempt_limit = vault_attempt_limit_note(state.unlock_attempt_limit);
            let mut lines = Vec::new();
            lines.push("Lock Status".to_string());
            lines.push(String::new());
            lines.push(format!("State: {}", state.status.locked));
            lines.push(format!(
                "Vault: {}",
                if state.has_vault() {
                    "present"
                } else {
                    "missing"
                }
            ));
            if state.status.locked == "UNLOCKED" {
                lines.push("Effect: sensitive content is displayed while UNLOCKED.".to_string());
            } else {
                lines.push("Effect: sensitive content is redacted while LOCKED.".to_string());
            }
            lines.push(String::new());
            lines.push(format!(
                "Auto-lock: enabled, timeout={} min",
                state.autolock_minutes()
            ));
            lines.push(format!("Attempt limit: {}", attempt_limit));
            lines.push(format!(
                "Failed unlock attempts since last success: {}",
                state.failed_unlock_attempts
            ));
            lines.push("Recovery: rerun /init if the wipe threshold is reached.".to_string());
            lines.push(String::new());
            lines.push(String::new());
            lines.push("Commands:".to_string());
            lines.push("  /lock".to_string());
            lines.push("  /autolock show".to_string());
            lines.push("  /autolock set <min>".to_string());
            lines.join("\n")
        }
        TuiInspectorPane::Help => [
            "Help".to_string(),
            String::new(),
            "Global".to_string(),
            "- /help (opens fullscreen help)".to_string(),
            "- /inspector <domain>".to_string(),
            "- /exit".to_string(),
            String::new(),
            "Keybindings".to_string(),
            "- Tab / Shift+Tab: cycle Nav/Main/Cmd focus".to_string(),
            "- Up / Down: move nav selection".to_string(),
            "- Enter: activate selected nav item only".to_string(),
            "- Esc: return focus to Nav / clear-cancel prompts".to_string(),
            String::new(),
            "Safety".to_string(),
            "- command bar explicit intent only".to_string(),
            String::new(),
            "Validated Baseline".to_string(),
            "- qbuild/local: LOCAL_TWO_CLIENT_RUNBOOK.md is the current front door.".to_string(),
            "- remote/AWS: compatibility evidence only, not the validated baseline.".to_string(),
            String::new(),
            "Attachment Migration".to_string(),
            "- Set QSC_ATTACHMENT_SERVICE to activate the validated post-w0 lane.".to_string(),
            "- On that lane, <= 4 MiB sends use w2 and legacy receive defaults to retired."
                .to_string(),
        ]
        .join("\n"),
        TuiInspectorPane::About => {
            emit_marker(
                "tui_about_links",
                None,
                &[
                    ("governance", "1"),
                    ("traceability", "1"),
                    ("decisions", "1"),
                    ("docs", "1"),
                    ("tests", "1"),
                ],
            );
            [
                "About".to_string(),
                String::new(),
                format!("version: {}", env!("CARGO_PKG_VERSION")),
                format!(
                    "commit: {}",
                    option_env!("QSC_GIT_SHA")
                        .or(option_env!("VERGEN_GIT_SHA"))
                        .unwrap_or("unknown")
                ),
                "posture: truthful state reflection; explicit intent only".to_string(),
                String::new(),
                "Proof links".to_string(),
                "  governance: NEXT_ACTIONS.md".to_string(),
                "  traceability: TRACEABILITY.md".to_string(),
                "  decisions: DECISIONS.md".to_string(),
                "  docs: docs/canonical/".to_string(),
                "  tests: qsl/qsl-client/qsc/tests/".to_string(),
            ]
            .join("\n")
        }
        TuiInspectorPane::Legal => {
            emit_marker(
                "tui_legal_fulltext",
                None,
                &[
                    ("sections", "summary,warranty,operator,privacy,init"),
                    ("overclaim", "none"),
                ],
            );
            [
                "Legal".to_string(),
                String::new(),
                "Summary".to_string(),
                "  This software is for testing and research workflows.".to_string(),
                "  It may fail, lose data, or become unavailable without notice.".to_string(),
                String::new(),
                "Warranty and liability".to_string(),
                "  Provided \"as is\" and \"as available\" without warranties.".to_string(),
                "  Operators and contributors are not liable for indirect or consequential losses."
                    .to_string(),
                String::new(),
                "Operator responsibility".to_string(),
                "  You are responsible for lawful use, local policy compliance, and key handling."
                    .to_string(),
                "  Verify identities out-of-band before relying on trust state.".to_string(),
                String::new(),
                "Privacy and security notes".to_string(),
                "  This interface does not claim metadata elimination.".to_string(),
                "  Treat endpoint, traffic timing, and deployment logs as potentially observable."
                    .to_string(),
                String::new(),
                "Init acceptance".to_string(),
                "  /init requires explicit legal acceptance (I AGREE) before vault creation."
                    .to_string(),
            ]
            .join("\n")
        }
    };
    let commands_gap = if body.contains("\n\n\nCommands:") {
        "2_plus"
    } else if body.contains("\n\nCommands:") {
        "1"
    } else if body.contains("\nCommands:") {
        "0"
    } else {
        "na"
    };
    emit_marker(
        "tui_commands_spacing",
        None,
        &[("inspector", state.inspector_name()), ("gap", commands_gap)],
    );
    let body = pad_panel_text(body.as_str());
    let main_first_line = body
        .lines()
        .find(|line| !line.trim().is_empty())
        .unwrap_or("none")
        .replace(' ', "_");
    let view_rows = usize::from(area.height).max(1);
    let content_lines = body.lines().count().max(1);
    state.update_main_scroll_metrics(content_lines, view_rows);
    let scroll = state.main_scroll_offset();
    let panel = Paragraph::new(body).scroll((scroll as u16, 0));
    f.render_widget(panel, area);
    emit_marker(
        "tui_main_render",
        None,
        &[("pad", "2"), ("first_line", main_first_line.as_str())],
    );
}

fn compute_envelope_status(payload_len: usize) -> String {
    let plan = envelope::plan_for_payload_len(
        payload_len,
        3,
        100,
        envelope::MAX_TICKS_DEFAULT,
        envelope::MAX_BUNDLE_SIZE_DEFAULT,
        envelope::MAX_PAYLOAD_COUNT_DEFAULT,
    );
    match plan {
        Ok(p) => {
            let tick = p.ticks.first().copied().unwrap_or(0);
            format!("bucket={} tick={}", p.bundle.bucket_len, tick)
        }
        Err(e) => format!("invalid({})", e.code()),
    }
}

fn compute_local_fingerprint() -> String {
    match identity_self_fingerprint("self") {
        Ok(fp) => fp,
        Err(_) => "untrusted".to_string(),
    }
}

fn compute_peer_fingerprint(peer: &str) -> String {
    let (fp, pinned) = identity_peer_status(peer);
    if pinned {
        format!("{} (pinned)", fp)
    } else {
        "untrusted".to_string()
    }
}

fn split_cmd_result_entry(entry: &str) -> (&str, &str, &str) {
    let Some(rest) = entry.strip_prefix('[') else {
        return ("unknown", "unknown", entry);
    };
    let Some((status, after_status)) = rest.split_once("] /") else {
        return ("unknown", "unknown", entry);
    };
    let Some((command, detail)) = after_status.split_once(' ') else {
        return (status, after_status, "ok");
    };
    (status, command, detail)
}

fn relay_endpoint_hash8(endpoint: &str) -> String {
    let c = StdCrypto;
    let hash = c.sha512(endpoint.as_bytes());
    hex_encode(&hash[..4])
}

fn relay_token_file_hash8(path: &str) -> String {
    let c = StdCrypto;
    let hash = c.sha512(path.as_bytes());
    hex_encode(&hash[..4])
}

fn route_token_hash8(token: &str) -> String {
    let c = StdCrypto;
    let hash = c.sha512(token.as_bytes());
    hex_encode(&hash[..4])
}

fn short_hash12(value: &str) -> String {
    let c = StdCrypto;
    let hash = c.sha512(value.as_bytes());
    hex_encode(&hash[..6])
}

fn short_identity_display(value: &str) -> String {
    if value.eq_ignore_ascii_case("untrusted") || value.trim().is_empty() {
        "untrusted".to_string()
    } else {
        short_hash12(value)
    }
}

fn relay_user_reason_from_code(code: &str) -> &'static str {
    match code {
        "relay_endpoint_missing" => "Relay endpoint missing: configure an endpoint first.",
        "relay_test_already_running" => "Relay test already running: wait for completion.",
        "relay_test_pending_timeout" => "Relay test did not complete in time.",
        "relay_unauthorized" => "Unauthorized (401): check token or token file.",
        "relay_overloaded" | "relay_inbox_queue_full" => "Relay overloaded (429): retry shortly.",
        "relay_network_unreachable" => "Network unreachable: check host, network, and firewall.",
        "relay_dns_failure" => "DNS failure: verify relay hostname.",
        "relay_network_timeout" => "Network timeout: relay did not respond in time.",
        "relay_inbox_push_failed" | "relay_inbox_pull_failed" | "relay_http_failure" => {
            "Relay request failed: verify endpoint and retry."
        }
        "relay_token_file_missing" => "Token file missing: set a valid token file path.",
        "relay_token_file_unreadable" => "Token file unreadable: check file ownership and perms.",
        "relay_token_file_empty" => "Token file empty: provide a valid bearer token.",
        "relay_token_file_perms_too_open" => "Token file perms too open: require 0600.",
        "relay_client_init_failed" => "Client init failed: local HTTP client unavailable.",
        "QSC_ERR_RELAY_TLS_REQUIRED" => "TLS required: use HTTPS (or loopback HTTP).",
        _ => "Relay operation failed.",
    }
}

fn read_relay_token_file(path: &str) -> Result<String, &'static str> {
    let p = Path::new(path);
    let md = fs::metadata(p).map_err(|_| "relay_token_file_missing")?;
    if !md.is_file() {
        return Err("relay_token_file_unreadable");
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mode = md.permissions().mode() & 0o777;
        if mode != 0o600 {
            return Err("relay_token_file_perms_too_open");
        }
    }
    let raw = fs::read_to_string(p).map_err(|_| "relay_token_file_unreadable")?;
    let token = raw.trim().to_string();
    if token.is_empty() {
        return Err("relay_token_file_empty");
    }
    Ok(token)
}

fn normalize_route_token(raw: &str) -> Result<String, &'static str> {
    adversarial::route::normalize_route_token(raw)
}

fn generate_route_token() -> String {
    let mut bytes = [0u8; 32];
    OsRng.fill_bytes(&mut bytes);
    hex_encode(&bytes)
}

fn relay_self_inbox_route_token() -> Result<String, &'static str> {
    let raw = vault::secret_get(TUI_RELAY_INBOX_TOKEN_SECRET_KEY)
        .map_err(|_| QSC_ERR_RELAY_INBOX_TOKEN_REQUIRED)?
        .ok_or(QSC_ERR_RELAY_INBOX_TOKEN_REQUIRED)?;
    if raw.trim().is_empty() {
        return Err(QSC_ERR_RELAY_INBOX_TOKEN_REQUIRED);
    }
    normalize_route_token(raw.as_str())
}

fn relay_peer_route_token(peer: &str) -> Result<String, &'static str> {
    let peer_alias = peer_alias_from_channel(peer);
    let rec = contacts_entry_read(peer_alias).map_err(|_| QSC_ERR_CONTACT_ROUTE_TOKEN_REQUIRED)?;
    let token = rec
        .and_then(|v| {
            primary_device(&v)
                .and_then(|d| d.route_token.clone())
                .or(v.route_token)
        })
        .ok_or(QSC_ERR_CONTACT_ROUTE_TOKEN_REQUIRED)?;
    normalize_route_token(token.as_str()).map_err(|_| QSC_ERR_CONTACT_ROUTE_TOKEN_REQUIRED)
}

fn normalize_relay_endpoint(value: &str) -> Result<String, &'static str> {
    adversarial::route::normalize_relay_endpoint(value)
}

fn relay_transport_label(endpoint: Option<&str>) -> &'static str {
    let Some(url) = endpoint else {
        return "unset";
    };
    if url.starts_with("https://") {
        "https"
    } else if url.starts_with("http://") {
        "http"
    } else {
        "unknown"
    }
}

fn relay_tls_label(endpoint: Option<&str>) -> &'static str {
    let Some(url) = endpoint else {
        return "unset";
    };
    if url.starts_with("https://") {
        "enabled"
    } else if url.starts_with("http://") {
        "disabled"
    } else {
        "unknown"
    }
}

fn relay_pinning_label(endpoint: Option<&str>) -> &'static str {
    match relay_tls_label(endpoint) {
        "enabled" => "not configured",
        _ => "n/a",
    }
}

fn validated_front_door_note() -> &'static str {
    "qbuild/local runbook is the validated front door."
}

fn compatibility_surface_note() -> &'static str {
    "remote/AWS artifacts remain compatibility evidence only."
}

fn validated_front_door_marker() -> &'static str {
    "local_qbuild_front_door"
}

fn compatibility_surface_marker() -> &'static str {
    "remote_aws_compat_only"
}

fn migration_posture_note(attachment_service_active: bool) -> &'static str {
    if attachment_service_active {
        "Validated post-w0 lane active: <= 4 MiB sends use w2 and legacy receive defaults to retired."
    } else {
        "Set QSC_ATTACHMENT_SERVICE to activate the validated post-w0 lane (w2 sends + retired legacy receive)."
    }
}

fn migration_posture_marker(attachment_service_active: bool) -> &'static str {
    if attachment_service_active {
        "attachment_service_active"
    } else {
        "attachment_service_required"
    }
}

fn vault_access_note(locked: bool) -> &'static str {
    if locked {
        "unlock required for local state"
    } else {
        "unlocked"
    }
}

fn vault_attempt_limit_note(limit: Option<u32>) -> String {
    match limit {
        Some(value) => format!("{value} failures wipe local vault + state"),
        None => "off (no automatic wipe threshold)".to_string(),
    }
}

fn relay_probe_url(endpoint: &str) -> Result<String, &'static str> {
    adversarial::route::relay_probe_url(endpoint)
}

fn account_storage_safety_status() -> String {
    let (cfg_dir, source) = match config_dir() {
        Ok(v) => v,
        Err(code) => return format!("reject ({})", code.as_str()),
    };
    if !check_symlink_safe(&cfg_dir) {
        return "reject (unsafe path symlink)".to_string();
    }
    if !check_parent_safe(&cfg_dir, source) {
        return "reject (unsafe parent perms)".to_string();
    }
    "OK".to_string()
}

fn identity_peer_status(peer: &str) -> (String, bool) {
    match identity_read_pin(peer) {
        Ok(Some(fp)) => (fp, true),
        Ok(None) => ("untrusted".to_string(), false),
        Err(_) => ("untrusted".to_string(), false),
    }
}

fn identity_show(self_label: &str) {
    let Some(rec) =
        identity_read_self_public(self_label).unwrap_or_else(|e| print_error_marker(e.as_str()))
    else {
        emit_marker(
            "identity_show",
            None,
            &[("ok", "false"), ("reason", "missing_identity")],
        );
        print_error_marker("identity_missing");
    };
    let fp = identity_fingerprint_from_pk(&rec.kem_pk);
    emit_marker(
        "identity_show",
        None,
        &[("ok", "true"), ("fp", fp.as_str())],
    );
    println!("identity_fp={}", fp);
}

fn identity_rotate(self_label: &str, confirm: bool, reset_peers: bool) {
    if !require_unlocked("identity_rotate") {
        return;
    }
    if !confirm {
        emit_marker(
            "identity_rotate",
            None,
            &[("ok", "false"), ("reason", "confirm_required")],
        );
        print_error_marker("identity_rotate_confirm_required");
    }
    let (kem_pk, kem_sk) = hs_kem_keypair();
    let (sig_pk, sig_sk) = hs_sig_keypair();
    if identity_secret_store(self_label, &kem_sk).is_err() {
        emit_marker(
            "identity_secret_unavailable",
            None,
            &[("reason", "vault_missing_or_locked")],
        );
        print_error_marker("identity_secret_unavailable");
    }
    if identity_sig_secret_store(self_label, &sig_sk).is_err() {
        emit_marker(
            "identity_secret_unavailable",
            None,
            &[("reason", "vault_missing_or_locked")],
        );
        print_error_marker("identity_secret_unavailable");
    }
    if identity_write_public_record(self_label, &kem_pk, &sig_pk).is_err() {
        emit_marker(
            "identity_rotate",
            None,
            &[("ok", "false"), ("reason", "write_failed")],
        );
        print_error_marker("identity_rotate_write_failed");
    }
    if reset_peers {
        let empty = ContactsStore::default();
        let _ = contacts_store_save(&empty);
        if let Ok((dir, source)) = config_dir() {
            let identities = identities_dir(&dir);
            if ensure_dir_secure(&identities, source).is_ok() {
                if let Ok(entries) = fs::read_dir(&identities) {
                    for entry in entries.flatten() {
                        if let Some(name) = entry.file_name().to_str() {
                            if name.starts_with("peer_") && name.ends_with(".fp") {
                                let _ = fs::remove_file(entry.path());
                            }
                        }
                    }
                }
            }
        }
    }
    let fp = identity_fingerprint_from_pk(&kem_pk);
    emit_marker(
        "identity_rotate",
        None,
        &[("ok", "true"), ("fp", fp.as_str())],
    );
    println!("identity_fp={}", fp);
}

fn peers_list() {
    let mut peers = contacts_list_entries()
        .unwrap_or_else(|_| print_error_marker("contacts_store_unavailable"))
        .into_iter()
        .map(|(label, rec)| (label, rec.fp))
        .collect::<Vec<_>>();
    peers.sort_by(|a, b| a.0.cmp(&b.0));
    let count_s = peers.len().to_string();
    emit_marker("peers_list", None, &[("count", count_s.as_str())]);
    for (peer, fp) in peers.iter() {
        emit_marker(
            "peer_item",
            None,
            &[
                ("peer", peer.as_str()),
                ("fp", fp.as_str()),
                ("status", "pinned"),
            ],
        );
        println!("peer={} fp={} status=pinned", peer, fp);
    }
}

fn env_bool(key: &str) -> bool {
    matches!(
        env::var(key).ok().as_deref(),
        Some("1") | Some("true") | Some("TRUE") | Some("yes") | Some("YES")
    )
}

fn tui_color_enabled() -> bool {
    if env::var_os("NO_COLOR").is_some() {
        return false;
    }
    if env::var("TERM")
        .ok()
        .map(|v| v.eq_ignore_ascii_case("dumb"))
        .unwrap_or(false)
    {
        return false;
    }
    true
}

fn config_set(key: &str, value: &str) {
    if key != "policy-profile" {
        print_error(ErrorCode::ParseFailed);
    }
    let profile = match normalize_profile(value) {
        Ok(v) => v,
        Err(e) => print_error(e),
    };

    let (dir, source) = match config_dir() {
        Ok(v) => v,
        Err(e) => print_error(e),
    };
    let file = dir.join(CONFIG_FILE_NAME);

    let _lock = match lock_store_exclusive(&dir, source) {
        Ok(v) => v,
        Err(e) => print_error(e),
    };
    if let Err(e) = ensure_store_layout(&dir, source) {
        print_error(e);
    }
    if let Err(e) = write_config_atomic(&file, &profile, source) {
        print_error(e);
    }

    print_marker(
        "config_set",
        &[
            ("key", "policy_profile"),
            ("value", &profile),
            ("ok", "true"),
        ],
    );
}

fn config_get(key: &str) {
    if key != "policy-profile" {
        print_error(ErrorCode::ParseFailed);
    }
    let (dir, source) = match config_dir() {
        Ok(v) => v,
        Err(e) => print_error(e),
    };
    let file = dir.join(CONFIG_FILE_NAME);

    if let Err(e) = enforce_safe_parents(&file, source) {
        print_error(e);
    }
    let _lock = match lock_store_shared(&dir, source) {
        Ok(v) => v,
        Err(e) => print_error(e),
    };
    #[cfg(unix)]
    if file.exists() {
        if let Err(e) = enforce_file_perms(&file) {
            print_error(e);
        }
    }

    let value = match read_policy_profile(&file) {
        Ok(Some(v)) => v,
        Ok(None) => "unset".to_string(),
        Err(e) => print_error(e),
    };

    print_marker(
        "config_get",
        &[("key", "policy_profile"), ("value", &value), ("ok", "true")],
    );
}

fn parse_vault_attempt_limit_config(raw: &str) -> Result<Option<u32>, ErrorCode> {
    for line in raw.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        let Some(value) = trimmed.strip_prefix("attempt_limit=") else {
            continue;
        };
        let value = value.trim();
        if value.eq_ignore_ascii_case("off") {
            return Ok(None);
        }
        let parsed = value.parse::<u32>().map_err(|_| ErrorCode::ParseFailed)?;
        if !(VAULT_ATTEMPT_LIMIT_MIN..=VAULT_ATTEMPT_LIMIT_MAX).contains(&parsed) {
            return Err(ErrorCode::ParseFailed);
        }
        return Ok(Some(parsed));
    }
    Ok(None)
}

fn parse_vault_failed_unlocks(raw: &str) -> Result<u32, ErrorCode> {
    for line in raw.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        let Some(value) = trimmed.strip_prefix("failed_unlocks=") else {
            continue;
        };
        return value
            .trim()
            .parse::<u32>()
            .map_err(|_| ErrorCode::ParseFailed);
    }
    Ok(0)
}

fn vault_security_state_load() -> Result<VaultSecurityState, ErrorCode> {
    let (dir, source) = config_dir()?;
    ensure_store_layout(&dir, source)?;
    let config_path = dir.join(VAULT_SECURITY_CONFIG_NAME);
    let counter_path = dir.join(VAULT_UNLOCK_COUNTER_NAME);
    enforce_safe_parents(&config_path, source)?;
    enforce_safe_parents(&counter_path, source)?;
    let _lock = lock_store_shared(&dir, source)?;

    let attempt_limit = if config_path.exists() {
        #[cfg(unix)]
        enforce_file_perms(&config_path)?;
        let mut raw = String::new();
        File::open(&config_path)
            .map_err(|_| ErrorCode::IoReadFailed)?
            .read_to_string(&mut raw)
            .map_err(|_| ErrorCode::IoReadFailed)?;
        parse_vault_attempt_limit_config(raw.as_str())?
    } else {
        None
    };

    let failed_unlocks = if counter_path.exists() {
        #[cfg(unix)]
        enforce_file_perms(&counter_path)?;
        let mut raw = String::new();
        File::open(&counter_path)
            .map_err(|_| ErrorCode::IoReadFailed)?
            .read_to_string(&mut raw)
            .map_err(|_| ErrorCode::IoReadFailed)?;
        parse_vault_failed_unlocks(raw.as_str())?
    } else {
        0
    };

    Ok(VaultSecurityState {
        attempt_limit,
        failed_unlocks,
    })
}

fn vault_security_state_store(state: &VaultSecurityState) -> Result<(), ErrorCode> {
    let (dir, source) = config_dir()?;
    ensure_store_layout(&dir, source)?;
    let config_path = dir.join(VAULT_SECURITY_CONFIG_NAME);
    let counter_path = dir.join(VAULT_UNLOCK_COUNTER_NAME);
    enforce_safe_parents(&config_path, source)?;
    enforce_safe_parents(&counter_path, source)?;
    let _lock = lock_store_exclusive(&dir, source)?;

    let config_content = match state.attempt_limit {
        Some(limit) => format!("attempt_limit={limit}\n"),
        None => "attempt_limit=off\n".to_string(),
    };
    let counter_content = format!("failed_unlocks={}\n", state.failed_unlocks);
    write_atomic(&config_path, config_content.as_bytes(), source)?;
    write_atomic(&counter_path, counter_content.as_bytes(), source)?;
    Ok(())
}

fn vault_security_state_clear_files() -> Result<(), ErrorCode> {
    let (dir, source) = config_dir()?;
    ensure_store_layout(&dir, source)?;
    let config_path = dir.join(VAULT_SECURITY_CONFIG_NAME);
    let counter_path = dir.join(VAULT_UNLOCK_COUNTER_NAME);
    enforce_safe_parents(&config_path, source)?;
    enforce_safe_parents(&counter_path, source)?;
    let _lock = lock_store_exclusive(&dir, source)?;
    let _ = fs::remove_file(config_path);
    let _ = fs::remove_file(counter_path);
    fsync_dir_best_effort(&dir);
    Ok(())
}

fn wipe_vault_file_best_effort() -> Result<(), ErrorCode> {
    let (dir, source) = config_dir()?;
    ensure_store_layout(&dir, source)?;
    let vault_path = dir.join("vault.qsv");
    enforce_safe_parents(&vault_path, source)?;
    let _lock = lock_store_exclusive(&dir, source)?;
    if !vault_path.exists() {
        return Ok(());
    }
    let tombstone = dir.join(format!("vault.qsv.tombstone.{}", process::id()));
    if fs::rename(&vault_path, &tombstone).is_ok() {
        let _ = fs::remove_file(&tombstone);
    } else {
        let _ = fs::remove_file(&vault_path);
    }
    fsync_dir_best_effort(&dir);
    Ok(())
}

#[derive(Serialize)]
struct DoctorReport {
    check_only: bool,
    ok: bool,
    dir_exists: bool,
    dir_writable: bool,
    file_parseable: bool,
    symlink_safe: bool,
    parent_safe: bool,
    config_dir: &'static str,
    redacted: bool,
}

fn doctor_check_only(check_only: bool, timeout_ms: u64, export: Option<PathBuf>) {
    if !check_only {
        print_error(ErrorCode::ParseFailed);
    }
    let (dir, source) = match config_dir() {
        Ok(v) => v,
        Err(e) => print_error(e),
    };
    let file = dir.join(CONFIG_FILE_NAME);

    let symlink_safe = check_symlink_safe(&dir);
    let parent_safe = check_parent_safe(&dir, source);
    let dir_exists = dir.is_dir();
    let checked_dir = dir.display().to_string();
    let dir_writable_required = false;
    let dir_writable = if dir_exists && symlink_safe && parent_safe {
        probe_dir_writable(&dir, timeout_ms)
    } else {
        false
    };

    let file_parseable = file.exists()
        && matches!(read_policy_profile(&file), Ok(Some(_)) | Ok(None))
        || !file.exists();
    let receipt_policy = load_receipt_policy_from_account();
    let receipt_batch_window_s = receipt_policy.batch_window_ms.to_string();
    let receipt_jitter_s = receipt_policy.jitter_ms.to_string();

    let report = DoctorReport {
        check_only: true,
        ok: true,
        dir_exists,
        dir_writable,
        file_parseable,
        symlink_safe,
        parent_safe,
        config_dir: "<redacted>",
        redacted: true,
    };

    if let Some(path) = export {
        if let Err(e) = write_doctor_export(&path, &report) {
            print_error(e);
        }
    }

    print_marker(
        "doctor",
        &[
            ("check_only", "true"),
            ("ok", "true"),
            ("checked_dir", &checked_dir),
            (
                "dir_writable_required",
                if dir_writable_required {
                    "true"
                } else {
                    "false"
                },
            ),
            ("dir_exists", bool_str(dir_exists)),
            ("dir_writable", bool_str(dir_writable)),
            ("file_parseable", bool_str(file_parseable)),
            ("symlink_safe", bool_str(symlink_safe)),
            ("parent_safe", bool_str(parent_safe)),
            ("receipt_mode", receipt_policy.mode.as_str()),
            (
                "file_confirm_mode",
                receipt_policy.file_confirm_mode.as_str(),
            ),
            ("receipt_batch_window_ms", receipt_batch_window_s.as_str()),
            ("receipt_jitter_ms", receipt_jitter_s.as_str()),
        ],
    );
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct TimelineEntry {
    id: String,
    peer: String,
    direction: String,
    byte_len: usize,
    kind: String,
    ts: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    target_device_id: Option<String>,
    #[serde(default)]
    state: String,
    #[serde(default)]
    status: String,
}

fn timeline_entry_default_state(direction: &str, status: &str) -> MessageState {
    if let Some(parsed) = MessageState::parse(status) {
        return parsed;
    }
    if direction == "out" {
        MessageState::Sent
    } else {
        MessageState::Received
    }
}

fn timeline_ts_default() -> u64 {
    1
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum MessageState {
    Created,
    Sent,
    Received,
    Delivered,
    Failed,
}

impl MessageState {
    fn as_str(self) -> &'static str {
        match self {
            MessageState::Created => "CREATED",
            MessageState::Sent => "SENT",
            MessageState::Received => "RECEIVED",
            MessageState::Delivered => "DELIVERED",
            MessageState::Failed => "FAILED",
        }
    }

    fn as_status(self) -> &'static str {
        match self {
            MessageState::Created => "created",
            MessageState::Sent => "sent",
            MessageState::Received => "received",
            MessageState::Delivered => "delivered",
            MessageState::Failed => "failed",
        }
    }

    fn parse(s: &str) -> Option<Self> {
        match s {
            "CREATED" | "created" => Some(MessageState::Created),
            "SENT" | "sent" => Some(MessageState::Sent),
            "RECEIVED" | "received" => Some(MessageState::Received),
            "DELIVERED" | "delivered" => Some(MessageState::Delivered),
            "FAILED" | "failed" => Some(MessageState::Failed),
            _ => None,
        }
    }
}

fn message_delivery_semantic(direction: &str, state: MessageState) -> Option<&'static str> {
    if direction != "out" {
        return None;
    }
    match state {
        MessageState::Sent => Some("accepted_by_relay"),
        MessageState::Delivered => Some("peer_confirmed"),
        _ => None,
    }
}

fn message_delivery_semantic_from_state_str(direction: &str, state: &str) -> Option<&'static str> {
    MessageState::parse(state).and_then(|parsed| message_delivery_semantic(direction, parsed))
}

fn file_delivery_semantic_from_state(state: &str) -> Option<&'static str> {
    match state.trim().to_ascii_uppercase().as_str() {
        "SENT" | "ACCEPTED_BY_RELAY" => Some("accepted_by_relay"),
        "AWAITING_CONFIRMATION" => Some("awaiting_confirmation"),
        "DELIVERED" | "PEER_CONFIRMED" => Some("peer_confirmed"),
        _ => None,
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ConfirmPolicy {
    PrimaryOnly,
}

impl ConfirmPolicy {
    fn as_str(self) -> &'static str {
        match self {
            Self::PrimaryOnly => "primary_only",
        }
    }
}

const CONFIRM_POLICY: ConfirmPolicy = ConfirmPolicy::PrimaryOnly;

fn emit_cli_confirm_policy() {
    emit_cli_named_marker("QSC_CONFIRM_POLICY", &[("policy", CONFIRM_POLICY.as_str())]);
}

fn emit_cli_delivery_state_with_device(peer: &str, state: &'static str, device: Option<&str>) {
    let safe_peer = short_peer_marker(peer);
    let safe_device = short_device_marker(device.unwrap_or("unknown"));
    emit_cli_named_marker(
        "QSC_DELIVERY",
        &[
            ("state", state),
            ("policy", CONFIRM_POLICY.as_str()),
            ("peer", safe_peer.as_str()),
            ("device", safe_device.as_str()),
        ],
    );
}

fn emit_tui_delivery_state_with_device(thread: &str, state: &'static str, device: Option<&str>) {
    let safe_thread = short_peer_marker(thread);
    let safe_device = short_device_marker(device.unwrap_or("unknown"));
    emit_tui_named_marker(
        "QSC_TUI_DELIVERY",
        &[
            ("state", state),
            ("policy", CONFIRM_POLICY.as_str()),
            ("thread", safe_thread.as_str()),
            ("device", safe_device.as_str()),
        ],
    );
}

fn emit_tui_delivery_state(thread: &str, state: &'static str) {
    emit_tui_delivery_state_with_device(thread, state, None);
}

fn emit_cli_receipt_ignored_wrong_device(peer: &str, device: &str) {
    let safe_peer = short_peer_marker(peer);
    let safe_device = short_device_marker(device);
    emit_cli_named_marker(
        "QSC_RECEIPT_IGNORED",
        &[
            ("reason", "wrong_device"),
            ("peer", safe_peer.as_str()),
            ("device", safe_device.as_str()),
        ],
    );
}

fn emit_tui_receipt_ignored_wrong_device(thread: &str, device: &str) {
    let safe_thread = short_peer_marker(thread);
    let safe_device = short_device_marker(device);
    emit_tui_named_marker(
        "QSC_TUI_RECEIPT_IGNORED",
        &[
            ("reason", "wrong_device"),
            ("thread", safe_thread.as_str()),
            ("device", safe_device.as_str()),
        ],
    );
}

fn message_state_transition_allowed(
    from: MessageState,
    to: MessageState,
    direction: &str,
) -> Result<(), &'static str> {
    if from == MessageState::Failed {
        return Err("failed_terminal");
    }
    if from == to {
        return Err("state_duplicate");
    }
    if direction == "out" {
        return match (from, to) {
            (MessageState::Created, MessageState::Sent)
            | (MessageState::Created, MessageState::Failed)
            | (MessageState::Sent, MessageState::Delivered)
            | (MessageState::Sent, MessageState::Failed) => Ok(()),
            _ => Err("state_invalid_transition"),
        };
    }
    match (from, to) {
        (MessageState::Created, MessageState::Received)
        | (MessageState::Created, MessageState::Failed)
        | (MessageState::Received, MessageState::Failed) => Ok(()),
        _ => Err("state_invalid_transition"),
    }
}

fn emit_message_state_transition(id: &str, from: MessageState, to: MessageState) {
    emit_marker(
        "message_state_transition",
        None,
        &[
            ("from", from.as_str()),
            ("to", to.as_str()),
            ("id", id),
            ("ok", "true"),
        ],
    );
}

fn emit_message_state_reject(id: &str, reason: &'static str) {
    emit_marker(
        "message_state_reject",
        Some(reason),
        &[("reason", reason), ("id", id)],
    );
}

fn timeline_entry_state(entry: &TimelineEntry) -> MessageState {
    MessageState::parse(entry.state.as_str())
        .or_else(|| MessageState::parse(entry.status.as_str()))
        .unwrap_or_else(|| {
            timeline_entry_default_state(entry.direction.as_str(), entry.status.as_str())
        })
}

fn tui_file_display_state(raw: &str) -> String {
    let upper = raw.trim().to_ascii_uppercase();
    match upper.as_str() {
        "VERIFIED" | "COMPLETE" => "VERIFIED".to_string(),
        "FAILED" | "REJECTED" => "FAILED".to_string(),
        "RECEIVING" | "CREATED" | "SENT" | "ANNOUNCED" | "PENDING" => "RECEIVING".to_string(),
        _ => upper,
    }
}

fn load_tui_files_snapshot() -> Vec<TuiFileItem> {
    let store = match timeline_store_load() {
        Ok(v) => v,
        Err(_) => return Vec::new(),
    };
    let mut out: Vec<TuiFileItem> = store
        .file_transfers
        .values()
        .map(|rec| TuiFileItem {
            id: rec.id.clone(),
            peer: rec.peer.clone(),
            filename: rec.filename.clone(),
            byte_len: rec.total_size,
            state: rec.state.clone(),
            display_state: tui_file_display_state(rec.state.as_str()),
        })
        .collect();
    out.sort_by(|a, b| a.id.cmp(&b.id));
    out
}

fn protocol_active_or_reason_for_send_peer(peer: &str) -> Result<(), String> {
    let routing = resolve_send_routing_target(peer).map_err(|code| code.to_string())?;
    protocol_active_or_reason_for_peer(routing.channel.as_str())
}

struct QspPackOutcome {
    envelope: Vec<u8>,
    next_state: Suite2SessionState,
    msg_idx: u32,
    ck_idx: u32,
    padded_len: usize,
    pad_label: Option<&'static str>,
}

#[derive(Clone, Copy)]
struct QspPackError {
    code: &'static str,
    reason: Option<&'static str>,
}

struct QspUnpackOutcome {
    plaintext: Vec<u8>,
    next_state: Suite2SessionState,
    msg_idx: u32,
    skip_delta: usize,
    evicted: usize,
}

const MKSKIPPED_CAP_DEFAULT: usize = 32;
const POLL_INTERVAL_MS_MAX: u64 = 60_000;
const POLL_TICKS_MAX: u32 = 64;
const POLL_MAX_PER_TICK_MAX: u32 = 32;
const PAD_TO_MAX: usize = 65_536;
const META_TICK_COUNT_DEFAULT: u32 = 1;
const META_INTERVAL_MS_DEFAULT: u64 = 1_000;
const META_BATCH_MAX_COUNT_DEFAULT: u32 = 1;
const META_BUCKET_MAX_DEFAULT: usize = 4_096;
const META_BUCKET_MAX_CEILING: usize = 65_536;

struct MetaPollConfig {
    interval_ms: u64,
    ticks: u32,
    batch_max_count: usize,
    bucket_max: usize,
    deterministic: bool,
}

#[derive(Clone, Copy)]
struct MetaPadConfig {
    target_len: Option<usize>,
    profile: Option<EnvelopeProfile>,
    label: Option<&'static str>,
}

fn mkskipped_cap() -> usize {
    let cap = env::var("QSC_MKSKIPPED_CAP")
        .ok()
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(MKSKIPPED_CAP_DEFAULT);
    cap.clamp(1, 1000)
}

fn bound_mkskipped(st: &mut Suite2RecvWireState) -> usize {
    let cap = mkskipped_cap();
    if st.mkskipped.len() <= cap {
        return 0;
    }
    st.mkskipped.sort_by_key(|e| e.n);
    let excess = st.mkskipped.len().saturating_sub(cap);
    if excess > 0 {
        st.mkskipped.drain(0..excess);
    }
    excess
}

fn meta_poll_config_from_args(args: MetaPollArgs) -> Result<Option<MetaPollConfig>, &'static str> {
    let MetaPollArgs {
        deterministic_meta,
        interval_ms,
        poll_interval_ms,
        ticks,
        batch_max_count,
        poll_max_per_tick,
        bucket_max,
        meta_seed,
    } = args;
    if interval_ms.is_some() && poll_interval_ms.is_some() {
        return Err("meta_poll_conflict");
    }
    if batch_max_count.is_some() && poll_max_per_tick.is_some() {
        return Err("meta_poll_conflict");
    }
    let any = deterministic_meta
        || interval_ms.is_some()
        || poll_interval_ms.is_some()
        || ticks.is_some()
        || batch_max_count.is_some()
        || poll_max_per_tick.is_some()
        || bucket_max.is_some()
        || meta_seed.is_some();
    if !any {
        return Ok(None);
    }
    let interval_ms = interval_ms
        .or(poll_interval_ms)
        .unwrap_or(META_INTERVAL_MS_DEFAULT);
    let ticks = ticks.unwrap_or(META_TICK_COUNT_DEFAULT);
    let batch_max_count = batch_max_count
        .or(poll_max_per_tick)
        .unwrap_or(META_BATCH_MAX_COUNT_DEFAULT);
    let bucket_max = bucket_max.unwrap_or(META_BUCKET_MAX_DEFAULT);
    if interval_ms == 0 || interval_ms > POLL_INTERVAL_MS_MAX {
        return Err("meta_poll_invalid");
    }
    if ticks == 0 || ticks > POLL_TICKS_MAX {
        return Err("meta_poll_invalid");
    }
    if batch_max_count == 0 || batch_max_count > POLL_MAX_PER_TICK_MAX {
        return Err("meta_poll_invalid");
    }
    if bucket_max == 0 || bucket_max > META_BUCKET_MAX_CEILING {
        return Err("meta_poll_invalid");
    }
    Ok(Some(MetaPollConfig {
        interval_ms,
        ticks,
        batch_max_count: batch_max_count as usize,
        bucket_max,
        deterministic: deterministic_meta || meta_seed.is_some(),
    }))
}

struct MetaPollArgs {
    deterministic_meta: bool,
    interval_ms: Option<u64>,
    poll_interval_ms: Option<u64>,
    ticks: Option<u32>,
    batch_max_count: Option<u32>,
    poll_max_per_tick: Option<u32>,
    bucket_max: Option<usize>,
    meta_seed: Option<u64>,
}

fn meta_bucket_for_len(orig_len: usize, bucket_max: usize) -> usize {
    let capped = orig_len.min(bucket_max).max(1);
    let mut bucket = 1usize;
    while bucket < capped {
        bucket = bucket.saturating_mul(2);
    }
    bucket.min(bucket_max)
}

type ReceiptControlPayload = adversarial::payload::ReceiptControlPayload;
type FileConfirmPayload = adversarial::payload::FileConfirmPayload;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ReceiptEmitMode {
    Off,
    Batched,
    Immediate,
}

impl ReceiptEmitMode {
    fn as_str(self) -> &'static str {
        match self {
            Self::Off => "off",
            Self::Batched => "batched",
            Self::Immediate => "immediate",
        }
    }

    fn from_raw(value: &str) -> Option<Self> {
        match value.trim().to_ascii_lowercase().as_str() {
            "off" => Some(Self::Off),
            "batched" => Some(Self::Batched),
            "immediate" => Some(Self::Immediate),
            _ => None,
        }
    }

    fn from_arg(value: ReceiptMode) -> Self {
        match value {
            ReceiptMode::Off => Self::Off,
            ReceiptMode::Batched => Self::Batched,
            ReceiptMode::Immediate => Self::Immediate,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum FileConfirmEmitMode {
    Off,
    CompleteOnly,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum TrustOnboardingMode {
    Strict,
    Balanced,
}

impl TrustOnboardingMode {
    fn as_str(self) -> &'static str {
        match self {
            Self::Strict => "strict",
            Self::Balanced => "balanced",
        }
    }

    fn from_raw(value: &str) -> Option<Self> {
        match value.trim().to_ascii_lowercase().as_str() {
            "strict" => Some(Self::Strict),
            "balanced" => Some(Self::Balanced),
            _ => None,
        }
    }

    fn from_arg(value: TrustMode) -> Self {
        match value {
            TrustMode::Strict => Self::Strict,
            TrustMode::Balanced => Self::Balanced,
        }
    }
}

impl FileConfirmEmitMode {
    fn as_str(self) -> &'static str {
        match self {
            Self::Off => "off",
            Self::CompleteOnly => "complete_only",
        }
    }

    fn from_raw(value: &str) -> Option<Self> {
        match value.trim().to_ascii_lowercase().as_str() {
            "off" => Some(Self::Off),
            "complete_only" | "complete-only" | "completeonly" => Some(Self::CompleteOnly),
            _ => None,
        }
    }

    fn from_arg(value: FileConfirmMode) -> Self {
        match value {
            FileConfirmMode::Off => Self::Off,
            FileConfirmMode::CompleteOnly => Self::CompleteOnly,
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct ReceiptPolicy {
    mode: ReceiptEmitMode,
    batch_window_ms: u64,
    jitter_ms: u64,
    file_confirm_mode: FileConfirmEmitMode,
}

impl Default for ReceiptPolicy {
    fn default() -> Self {
        Self {
            mode: ReceiptEmitMode::Off,
            batch_window_ms: RECEIPT_BATCH_WINDOW_MS_DEFAULT,
            jitter_ms: RECEIPT_JITTER_MS_DEFAULT,
            file_confirm_mode: FileConfirmEmitMode::CompleteOnly,
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
struct ReceiptPolicyOverrides {
    emit_receipts: Option<ReceiptKind>,
    receipt_mode: Option<ReceiptMode>,
    receipt_batch_window_ms: Option<u64>,
    receipt_jitter_ms: Option<u64>,
    file_confirm_mode: Option<FileConfirmMode>,
}

fn parse_receipt_batch_window_ms(value: &str) -> Option<u64> {
    let parsed = value.trim().parse::<u64>().ok()?;
    if (1..=RECEIPT_BATCH_WINDOW_MS_MAX).contains(&parsed) {
        Some(parsed)
    } else {
        None
    }
}

fn parse_receipt_jitter_ms(value: &str) -> Option<u64> {
    let parsed = value.trim().parse::<u64>().ok()?;
    if parsed <= RECEIPT_JITTER_MS_MAX {
        Some(parsed)
    } else {
        None
    }
}

fn account_secret_trimmed(key: &str) -> Option<String> {
    vault::secret_get(key)
        .ok()
        .flatten()
        .map(|v| v.trim().to_string())
        .filter(|v| !v.is_empty())
}

fn load_receipt_policy_from_account() -> ReceiptPolicy {
    if !vault_unlocked() {
        return ReceiptPolicy::default();
    }
    let mut policy = ReceiptPolicy::default();
    if let Some(raw) = account_secret_trimmed(TUI_RECEIPT_MODE_SECRET_KEY) {
        if let Some(mode) = ReceiptEmitMode::from_raw(raw.as_str()) {
            policy.mode = mode;
        }
    }
    if let Some(raw) = account_secret_trimmed(TUI_RECEIPT_BATCH_WINDOW_MS_SECRET_KEY) {
        if let Some(ms) = parse_receipt_batch_window_ms(raw.as_str()) {
            policy.batch_window_ms = ms;
        }
    }
    if let Some(raw) = account_secret_trimmed(TUI_RECEIPT_JITTER_MS_SECRET_KEY) {
        if let Some(ms) = parse_receipt_jitter_ms(raw.as_str()) {
            policy.jitter_ms = ms;
        }
    }
    if let Some(raw) = account_secret_trimmed(TUI_FILE_CONFIRM_MODE_SECRET_KEY) {
        if let Some(mode) = FileConfirmEmitMode::from_raw(raw.as_str()) {
            policy.file_confirm_mode = mode;
        }
    }
    policy
}

fn load_trust_onboarding_mode_from_account() -> TrustOnboardingMode {
    if !vault_unlocked() {
        return TrustOnboardingMode::Balanced;
    }
    account_secret_trimmed(TUI_TRUST_MODE_SECRET_KEY)
        .as_deref()
        .and_then(TrustOnboardingMode::from_raw)
        .unwrap_or(TrustOnboardingMode::Balanced)
}

fn resolve_receipt_policy(overrides: ReceiptPolicyOverrides) -> ReceiptPolicy {
    let mut policy = load_receipt_policy_from_account();
    if overrides.emit_receipts.is_some() {
        policy.mode = ReceiptEmitMode::Immediate;
        policy.file_confirm_mode = FileConfirmEmitMode::CompleteOnly;
    }
    if let Some(mode) = overrides.receipt_mode {
        policy.mode = ReceiptEmitMode::from_arg(mode);
    }
    if let Some(ms) = overrides.receipt_batch_window_ms {
        policy.batch_window_ms = ms.clamp(1, RECEIPT_BATCH_WINDOW_MS_MAX);
    }
    if let Some(ms) = overrides.receipt_jitter_ms {
        policy.jitter_ms = ms.min(RECEIPT_JITTER_MS_MAX);
    }
    if let Some(mode) = overrides.file_confirm_mode {
        policy.file_confirm_mode = FileConfirmEmitMode::from_arg(mode);
    }
    if policy.mode != ReceiptEmitMode::Batched {
        policy.batch_window_ms = RECEIPT_BATCH_WINDOW_MS_DEFAULT;
        policy.jitter_ms = RECEIPT_JITTER_MS_DEFAULT;
    }
    policy
}

fn receipt_kind_str(kind: ReceiptKind) -> &'static str {
    match kind {
        ReceiptKind::Delivered => "delivered",
    }
}

fn receipt_msg_id(payload: &[u8]) -> String {
    let c = StdCrypto;
    let h = c.sha512(payload);
    hex_encode(&h[..8])
}

fn encode_receipt_data_payload(
    payload: Vec<u8>,
    receipt: Option<ReceiptKind>,
) -> (Vec<u8>, Option<String>) {
    let Some(kind) = receipt else {
        return (payload, None);
    };
    let msg_id = receipt_msg_id(&payload);
    let ctrl = ReceiptControlPayload {
        v: 1,
        t: "data".to_string(),
        kind: receipt_kind_str(kind).to_string(),
        msg_id: msg_id.clone(),
        body: Some(payload),
    };
    let encoded =
        serde_json::to_vec(&ctrl).unwrap_or_else(|_| print_error_marker("receipt_encode_failed"));
    (encoded, Some(msg_id))
}

fn file_delivery_short_id(raw: &str) -> String {
    let mut out = String::new();
    for ch in raw.chars() {
        if ch.is_ascii_alphanumeric() || ch == '-' || ch == '_' {
            out.push(ch.to_ascii_lowercase());
        }
        if out.len() >= 12 {
            break;
        }
    }
    if out.is_empty() {
        "unknown".to_string()
    } else {
        out
    }
}

fn emit_cli_file_delivery_with_device(
    peer: &str,
    state: &'static str,
    file_id: &str,
    device: Option<&str>,
) {
    let safe_peer = short_peer_marker(peer);
    let safe_file = file_delivery_short_id(file_id);
    let safe_device = short_device_marker(device.unwrap_or("unknown"));
    emit_cli_named_marker(
        "QSC_FILE_DELIVERY",
        &[
            ("state", state),
            ("policy", CONFIRM_POLICY.as_str()),
            ("peer", safe_peer.as_str()),
            ("device", safe_device.as_str()),
            ("file", safe_file.as_str()),
        ],
    );
}

fn emit_tui_file_delivery_with_device(
    thread: &str,
    state: &'static str,
    file_id: &str,
    device: Option<&str>,
) {
    let safe_thread = short_peer_marker(thread);
    let safe_file = file_delivery_short_id(file_id);
    let safe_device = short_device_marker(device.unwrap_or("unknown"));
    emit_tui_named_marker(
        "QSC_TUI_FILE_CONFIRM",
        &[
            ("state", state),
            ("policy", CONFIRM_POLICY.as_str()),
            ("thread", safe_thread.as_str()),
            ("device", safe_device.as_str()),
            ("file", safe_file.as_str()),
        ],
    );
}

fn emit_tui_file_delivery(thread: &str, state: &'static str, file_id: &str) {
    emit_tui_file_delivery_with_device(thread, state, file_id, None);
}

fn emit_cli_receipt_policy_event(
    mode: ReceiptEmitMode,
    status: &'static str,
    kind: &'static str,
    peer: &str,
) {
    let safe_peer = short_peer_marker(peer);
    emit_cli_named_marker(
        "QSC_RECEIPT",
        &[
            ("mode", mode.as_str()),
            ("status", status),
            ("kind", kind),
            ("peer", safe_peer.as_str()),
        ],
    );
}

fn emit_tui_receipt_policy_event(
    mode: ReceiptEmitMode,
    status: &'static str,
    kind: &'static str,
    thread: &str,
) {
    let safe_thread = short_peer_marker(thread);
    emit_tui_named_marker(
        "QSC_TUI_RECEIPT",
        &[
            ("mode", mode.as_str()),
            ("status", status),
            ("kind", kind),
            ("thread", safe_thread.as_str()),
        ],
    );
}

fn parse_receipt_payload(plaintext: &[u8]) -> Option<ReceiptControlPayload> {
    adversarial::payload::parse_receipt_payload(plaintext)
}

fn parse_file_confirm_payload(plaintext: &[u8]) -> Option<FileConfirmPayload> {
    adversarial::payload::parse_file_confirm_payload(plaintext)
}

fn parse_file_transfer_payload(plaintext: &[u8]) -> Option<FileTransferPayload> {
    adversarial::payload::parse_file_transfer_payload(plaintext)
}

fn parse_attachment_descriptor_payload(plaintext: &[u8]) -> Option<AttachmentDescriptorPayload> {
    adversarial::payload::parse_attachment_descriptor_payload(plaintext)
}

fn parse_attachment_confirm_payload(plaintext: &[u8]) -> Option<AttachmentConfirmPayload> {
    adversarial::payload::parse_attachment_confirm_payload(plaintext)
}

fn attachment_journal_load() -> Result<AttachmentJournal, &'static str> {
    match vault::secret_get(ATTACHMENT_JOURNAL_SECRET_KEY) {
        Ok(None) => Ok(AttachmentJournal::default()),
        Ok(Some(v)) => {
            serde_json::from_str::<AttachmentJournal>(&v).map_err(|_| "attachment_journal_tampered")
        }
        Err("vault_missing" | "vault_locked") => Err("attachment_journal_unavailable"),
        Err(_) => Err("attachment_journal_unavailable"),
    }
}

fn attachment_journal_save(store: &AttachmentJournal) -> Result<(), &'static str> {
    let json = serde_json::to_string(store).map_err(|_| "attachment_journal_unavailable")?;
    match vault::secret_set(ATTACHMENT_JOURNAL_SECRET_KEY, &json) {
        Ok(()) => Ok(()),
        Err("vault_missing" | "vault_locked") => Err("attachment_journal_unavailable"),
        Err(_) => Err("attachment_journal_unavailable"),
    }
}

fn attachment_record_key(direction: &str, peer: &str, attachment_id: &str) -> String {
    format!("{direction}:{peer}:{attachment_id}")
}

fn attachment_find_outbound_by_source(
    store: &AttachmentJournal,
    peer: &str,
    source_path: &Path,
) -> Option<(String, AttachmentTransferRecord)> {
    let needle = source_path.to_string_lossy();
    store.records.iter().find_map(|(key, rec)| {
        if rec.direction == "out"
            && rec.peer == peer
            && rec.source_path.as_deref() == Some(needle.as_ref())
            && rec.state != "PEER_CONFIRMED"
        {
            Some((key.clone(), rec.clone()))
        } else {
            None
        }
    })
}

fn attachment_stage_root(cfg_dir: &Path) -> PathBuf {
    cfg_dir.join(ATTACHMENT_STAGING_DIR)
}

fn attachment_staging_dir(cfg_dir: &Path, direction: &str) -> Result<PathBuf, &'static str> {
    let (root, source) = config_dir().map_err(|_| "attachment_stage_unavailable")?;
    if root != cfg_dir {
        return Err("attachment_stage_unavailable");
    }
    let dir = attachment_stage_root(cfg_dir).join(direction);
    ensure_dir_secure(&dir, source).map_err(|_| "attachment_stage_unavailable")?;
    Ok(dir)
}

fn attachment_outbound_rel(attachment_id: &str) -> String {
    format!("outbound/{attachment_id}.cipher")
}

fn attachment_inbound_rel(attachment_id: &str) -> String {
    format!("inbound/{attachment_id}.cipher")
}

fn attachment_path_from_rel(cfg_dir: &Path, rel: &str) -> Result<PathBuf, &'static str> {
    if rel.contains("..") {
        return Err("attachment_stage_unavailable");
    }
    let path = attachment_stage_root(cfg_dir).join(rel);
    let source = ConfigSource::EnvOverride;
    enforce_safe_parents(&path, source).map_err(|_| "attachment_stage_unavailable")?;
    Ok(path)
}

fn attachment_part_size_bytes(class: &str) -> Option<usize> {
    match class {
        "p64k" => Some(65_536),
        "p256k" => Some(262_144),
        "p1024k" => Some(1_048_576),
        _ => None,
    }
}

fn choose_attachment_part_size_class(plaintext_len: u64) -> &'static str {
    if plaintext_len <= 16 * 1024 * 1024 {
        "p64k"
    } else if plaintext_len <= 64 * 1024 * 1024 {
        "p256k"
    } else {
        "p1024k"
    }
}

fn attachment_plaintext_capacity(class: &str) -> Option<usize> {
    attachment_part_size_bytes(class)?.checked_sub(ATTACHMENT_CIPHER_TAG_LEN)
}

fn attachment_part_count_for_plaintext(plaintext_len: u64, class: &str) -> Option<u32> {
    let capacity = attachment_plaintext_capacity(class)? as u64;
    if plaintext_len == 0 || capacity == 0 {
        return None;
    }
    let count = plaintext_len.div_ceil(capacity);
    u32::try_from(count).ok()
}

fn attachment_ciphertext_len_for_plaintext(plaintext_len: u64, part_count: u32) -> Option<u64> {
    plaintext_len.checked_add(u64::from(part_count) * ATTACHMENT_CIPHER_TAG_LEN as u64)
}

fn attachment_ciphertext_part_len(
    part_index: u32,
    _plaintext_len: u64,
    part_size_class: &str,
    part_count: u32,
    ciphertext_len: u64,
) -> Option<usize> {
    let part_size = attachment_part_size_bytes(part_size_class)? as u64;
    if part_index >= part_count || part_count == 0 {
        return None;
    }
    if part_index + 1 < part_count {
        return usize::try_from(part_size).ok();
    }
    let offset = u64::from(part_index) * part_size;
    usize::try_from(ciphertext_len.checked_sub(offset)?).ok()
}

fn attachment_nonce(prefix: &[u8; 8], part_index: u32) -> [u8; 12] {
    let mut out = [0u8; 12];
    out[..8].copy_from_slice(prefix);
    out[8..].copy_from_slice(&part_index.to_be_bytes());
    out
}

fn attachment_part_aad(
    attachment_id: &str,
    enc_ctx_alg: &str,
    plaintext_len: u64,
    ciphertext_len: u64,
    part_size_class: &str,
    part_count: u32,
    part_index: u32,
) -> Vec<u8> {
    format!(
        "QATT-PART-V1|{attachment_id}|{enc_ctx_alg}|{plaintext_len}|{ciphertext_len}|{part_size_class}|{part_count}|{part_index}"
    )
    .into_bytes()
}

fn attachment_merkle_leaf(part_index: u32, bytes: &[u8]) -> [u8; 64] {
    let mut hasher = Sha512::new();
    hasher.update([0x00]);
    hasher.update(part_index.to_be_bytes());
    hasher.update((bytes.len() as u64).to_be_bytes());
    hasher.update(bytes);
    let digest = hasher.finalize();
    let mut out = [0u8; 64];
    out.copy_from_slice(&digest);
    out
}

fn attachment_merkle_root(mut level: Vec<[u8; 64]>) -> Option<String> {
    if level.is_empty() {
        return None;
    }
    while level.len() > 1 {
        let mut next = Vec::with_capacity(level.len().div_ceil(2));
        let mut idx = 0usize;
        while idx < level.len() {
            let left = level[idx];
            let right = if idx + 1 < level.len() {
                level[idx + 1]
            } else {
                level[idx]
            };
            let mut hasher = Sha512::new();
            hasher.update([0x01]);
            hasher.update(left);
            hasher.update(right);
            let digest = hasher.finalize();
            let mut out = [0u8; 64];
            out.copy_from_slice(&digest);
            next.push(out);
            idx += 2;
        }
        level = next;
    }
    Some(hex_encode(&level[0]))
}

fn attachment_generate_id() -> String {
    let mut bytes = [0u8; 32];
    OsRng.fill_bytes(&mut bytes);
    hex_encode(&bytes)
}

fn attachment_build_enc_ctx() -> (String, [u8; 32], [u8; 8]) {
    let mut cek = [0u8; 32];
    let mut prefix = [0u8; 8];
    OsRng.fill_bytes(&mut cek);
    OsRng.fill_bytes(&mut prefix);
    let mut raw = [0u8; ATTACHMENT_CONTEXT_PACKAGE_LEN];
    raw[0] = 0x01;
    raw[1..33].copy_from_slice(&cek);
    raw[33..].copy_from_slice(&prefix);
    (URL_SAFE_NO_PAD.encode(raw), cek, prefix)
}

fn attachment_decode_enc_ctx(token: &str) -> Result<([u8; 32], [u8; 8]), &'static str> {
    if token.len() != ATTACHMENT_CONTEXT_PACKAGE_B64U_LEN {
        return Err("REJECT_ATT_DESC_ENC_CTX");
    }
    let raw = URL_SAFE_NO_PAD
        .decode(token.as_bytes())
        .map_err(|_| "REJECT_ATT_DESC_ENC_CTX")?;
    if raw.len() != ATTACHMENT_CONTEXT_PACKAGE_LEN || raw[0] != 0x01 {
        return Err("REJECT_ATT_DESC_ENC_CTX");
    }
    let mut cek = [0u8; 32];
    let mut prefix = [0u8; 8];
    cek.copy_from_slice(&raw[1..33]);
    prefix.copy_from_slice(&raw[33..41]);
    Ok((cek, prefix))
}

struct AttachmentConfirmHandleInput<'a> {
    attachment_id: &'a str,
    plaintext_len: u64,
    ciphertext_len: u64,
    part_size_class: &'a str,
    part_count: u32,
    integrity_alg: &'a str,
    integrity_root: &'a str,
    retention_class: &'a str,
    expires_at_unix_s: u64,
}

fn attachment_confirm_handle(input: AttachmentConfirmHandleInput<'_>) -> String {
    let AttachmentConfirmHandleInput {
        attachment_id,
        plaintext_len,
        ciphertext_len,
        part_size_class,
        part_count,
        integrity_alg,
        integrity_root,
        retention_class,
        expires_at_unix_s,
    } = input;
    let material = format!(
        "QATT-CONFIRM-V1|{attachment_id}|{plaintext_len}|{ciphertext_len}|{part_size_class}|{part_count}|{integrity_alg}|{integrity_root}|{retention_class}|{expires_at_unix_s}"
    );
    let digest = Sha512::digest(material.as_bytes());
    hex_encode(&digest[..12])
}

fn attachment_is_lower_hex_len(value: &str, len: usize) -> bool {
    value.len() == len
        && value
            .chars()
            .all(|ch| ch.is_ascii_hexdigit() && !ch.is_ascii_uppercase())
}

fn file_xfer_chunk_hash(chunk: &[u8]) -> String {
    let c = StdCrypto;
    let hash = c.sha512(chunk);
    hex_encode(&hash[..16])
}

fn file_xfer_id(peer: &str, filename: &str, payload: &[u8]) -> String {
    let c = StdCrypto;
    let mut data = Vec::new();
    data.extend_from_slice(peer.as_bytes());
    data.push(0);
    data.extend_from_slice(filename.as_bytes());
    data.push(0);
    data.extend_from_slice(payload);
    let hash = c.sha512(&data);
    hex_encode(&hash[..12])
}

fn file_xfer_manifest_hash(
    file_id: &str,
    total_size: usize,
    chunk_count: usize,
    chunk_hashes: &[String],
) -> String {
    let c = StdCrypto;
    let joined = chunk_hashes.join(",");
    let data = format!("{}|{}|{}|{}", file_id, total_size, chunk_count, joined);
    let hash = c.sha512(data.as_bytes());
    hex_encode(&hash[..16])
}

fn file_xfer_confirm_id(file_id: &str, manifest_hash: &str) -> String {
    let c = StdCrypto;
    let data = format!("{}|{}", file_id, manifest_hash);
    let hash = c.sha512(data.as_bytes());
    hex_encode(&hash[..12])
}

#[derive(Serialize)]
struct AttachmentServiceCreateSessionRequest {
    attachment_id: String,
    ciphertext_len: u64,
    part_size_class: String,
    part_count: u32,
    integrity_alg: String,
    integrity_root: String,
    retention_class: String,
}

#[derive(Deserialize)]
struct AttachmentServiceCreateSessionResponse {
    #[serde(rename = "session_id")]
    session_ref: String,
    resume_token: String,
}

#[derive(Deserialize)]
struct AttachmentServiceMissingRange {
    start: u32,
    end: u32,
}

#[derive(Deserialize)]
struct AttachmentServiceSessionStatusResponse {
    missing_part_ranges: Vec<AttachmentServiceMissingRange>,
}

#[derive(Serialize)]
struct AttachmentServiceCommitRequest {
    attachment_id: String,
    ciphertext_len: u64,
    part_count: u32,
    integrity_alg: String,
    integrity_root: String,
    retention_class: String,
}

#[derive(Deserialize)]
struct AttachmentServiceCommitResponse {
    locator_kind: String,
    locator_ref: String,
    fetch_capability: String,
    expires_at_unix_s: u64,
}

#[derive(Deserialize)]
struct AttachmentServiceErrorBody {
    reason_code: String,
}

fn attachment_now_unix_s() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

fn attachment_service_reason(
    response: reqwest::blocking::Response,
    fallback: &'static str,
) -> String {
    let status = response.status().as_u16();
    match response.json::<AttachmentServiceErrorBody>() {
        Ok(body) if !body.reason_code.trim().is_empty() => body.reason_code,
        _ => format!("{fallback}_{status}"),
    }
}

fn attachment_service_create_session(
    service_url: &str,
    record: &AttachmentTransferRecord,
) -> Result<AttachmentServiceCreateSessionResponse, String> {
    let url = format!("{service_url}/v1/attachments/sessions");
    let client = HttpClient::new();
    let request = AttachmentServiceCreateSessionRequest {
        attachment_id: record.attachment_id.clone(),
        ciphertext_len: record.ciphertext_len,
        part_size_class: record.part_size_class.clone(),
        part_count: record.part_count,
        integrity_alg: record.integrity_alg.clone(),
        integrity_root: record.integrity_root.clone(),
        retention_class: record.retention_class.clone(),
    };
    let response = client
        .post(url)
        .json(&request)
        .send()
        .map_err(|_| "attachment_service_create_failed".to_string())?;
    if !response.status().is_success() {
        return Err(attachment_service_reason(
            response,
            "attachment_service_create_failed",
        ));
    }
    response
        .json::<AttachmentServiceCreateSessionResponse>()
        .map_err(|_| "attachment_service_create_parse_failed".to_string())
}

fn attachment_service_status(
    service_url: &str,
    session_ref: &str,
    resume_token: &str,
) -> Result<AttachmentServiceSessionStatusResponse, String> {
    let url = format!("{service_url}/v1/attachments/sessions/{session_ref}");
    let client = HttpClient::new();
    let token = env::var("QSC_ATTACHMENT_RESUME_TOKEN_OVERRIDE")
        .ok()
        .filter(|v| !v.trim().is_empty())
        .unwrap_or_else(|| resume_token.to_string());
    let response = client
        .get(url)
        .header("X-QATT-Resume-Token", token)
        .send()
        .map_err(|_| "attachment_service_status_failed".to_string())?;
    if !response.status().is_success() {
        return Err(attachment_service_reason(
            response,
            "attachment_service_status_failed",
        ));
    }
    response
        .json::<AttachmentServiceSessionStatusResponse>()
        .map_err(|_| "attachment_service_status_parse_failed".to_string())
}

fn attachment_service_upload_part(
    service_url: &str,
    session_ref: &str,
    part_index: u32,
    resume_token: &str,
    bytes: Vec<u8>,
) -> Result<(), String> {
    let url = format!("{service_url}/v1/attachments/sessions/{session_ref}/parts/{part_index}");
    let client = HttpClient::new();
    let token = env::var("QSC_ATTACHMENT_RESUME_TOKEN_OVERRIDE")
        .ok()
        .filter(|v| !v.trim().is_empty())
        .unwrap_or_else(|| resume_token.to_string());
    let response = client
        .put(url)
        .header("X-QATT-Resume-Token", token)
        .body(bytes)
        .send()
        .map_err(|_| "attachment_service_upload_failed".to_string())?;
    if !response.status().is_success() {
        return Err(attachment_service_reason(
            response,
            "attachment_service_upload_failed",
        ));
    }
    Ok(())
}

fn attachment_service_commit(
    service_url: &str,
    session_ref: &str,
    resume_token: &str,
    record: &AttachmentTransferRecord,
) -> Result<AttachmentServiceCommitResponse, String> {
    let url = format!("{service_url}/v1/attachments/sessions/{session_ref}/commit");
    let client = HttpClient::new();
    let token = env::var("QSC_ATTACHMENT_RESUME_TOKEN_OVERRIDE")
        .ok()
        .filter(|v| !v.trim().is_empty())
        .unwrap_or_else(|| resume_token.to_string());
    let request = AttachmentServiceCommitRequest {
        attachment_id: record.attachment_id.clone(),
        ciphertext_len: record.ciphertext_len,
        part_count: record.part_count,
        integrity_alg: record.integrity_alg.clone(),
        integrity_root: record.integrity_root.clone(),
        retention_class: record.retention_class.clone(),
    };
    let response = client
        .post(url)
        .header("X-QATT-Resume-Token", token)
        .json(&request)
        .send()
        .map_err(|_| "attachment_service_commit_failed".to_string())?;
    if !response.status().is_success() {
        return Err(attachment_service_reason(
            response,
            "attachment_service_commit_failed",
        ));
    }
    response
        .json::<AttachmentServiceCommitResponse>()
        .map_err(|_| "attachment_service_commit_parse_failed".to_string())
}

fn attachment_validate_filename_hint(raw: &str) -> Result<String, &'static str> {
    let trimmed = raw.trim();
    if trimmed.is_empty()
        || trimmed.len() > 255
        || trimmed.contains('/')
        || trimmed.contains('\\')
        || trimmed == "."
        || trimmed == ".."
    {
        return Err("REJECT_ATT_DESC_FIELD_DOMAIN");
    }
    Ok(trimmed.to_string())
}

fn attachment_output_name(record: &AttachmentTransferRecord) -> String {
    record
        .filename_hint
        .as_deref()
        .and_then(|v| attachment_validate_filename_hint(v).ok())
        .unwrap_or_else(|| {
            format!(
                "attachment-{}.bin",
                file_delivery_short_id(&record.attachment_id)
            )
        })
}

fn attachment_build_outbound_record(
    peer: &str,
    service_url: &str,
    path: &Path,
    receipt: Option<ReceiptKind>,
) -> Result<AttachmentTransferRecord, &'static str> {
    let metadata = fs::metadata(path).map_err(|_| "file_xfer_read_failed")?;
    let plaintext_len = metadata.len();
    if plaintext_len == 0 {
        return Err("file_xfer_empty");
    }
    if plaintext_len > ATTACHMENT_DEFAULT_MAX_FILE_SIZE as u64 {
        return Err("size_exceeds_max");
    }
    let filename_hint = path
        .file_name()
        .and_then(|v| v.to_str())
        .map(attachment_validate_filename_hint)
        .transpose()?;
    let attachment_id = attachment_generate_id();
    let part_size_class = choose_attachment_part_size_class(plaintext_len).to_string();
    let part_count = attachment_part_count_for_plaintext(plaintext_len, &part_size_class)
        .ok_or("attachment_shape_invalid")?;
    let ciphertext_len = attachment_ciphertext_len_for_plaintext(plaintext_len, part_count)
        .ok_or("attachment_shape_invalid")?;
    let (enc_ctx_b64u, cek, nonce_prefix) = attachment_build_enc_ctx();
    let (cfg_dir, _) = config_dir().map_err(|_| "attachment_stage_unavailable")?;
    let stage_dir = attachment_staging_dir(&cfg_dir, "outbound")?;
    let staged_rel = attachment_outbound_rel(&attachment_id);
    let staged_path = stage_dir.join(format!("{attachment_id}.cipher"));
    let mut src = File::open(path).map_err(|_| "file_xfer_read_failed")?;
    let mut dst = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(&staged_path)
        .map_err(|_| "attachment_stage_unavailable")?;
    #[cfg(unix)]
    enforce_file_perms(&staged_path).map_err(|_| "attachment_stage_unavailable")?;
    let cipher = ChaCha20Poly1305::new(Key::from_slice(&cek));
    let capacity =
        attachment_plaintext_capacity(&part_size_class).ok_or("attachment_shape_invalid")?;
    let mut leaves = Vec::with_capacity(part_count as usize);
    let mut buf = vec![0u8; capacity];
    let mut produced = 0u32;
    loop {
        let mut read_len = 0usize;
        while read_len < capacity {
            let n = src
                .read(&mut buf[read_len..])
                .map_err(|_| "file_xfer_read_failed")?;
            if n == 0 {
                break;
            }
            read_len += n;
        }
        if read_len == 0 {
            break;
        }
        let nonce = attachment_nonce(&nonce_prefix, produced);
        let aad = attachment_part_aad(
            &attachment_id,
            ATTACHMENT_ENC_CTX_ALG_V1,
            plaintext_len,
            ciphertext_len,
            &part_size_class,
            part_count,
            produced,
        );
        let ciphertext = cipher
            .encrypt(
                Nonce::from_slice(&nonce),
                Payload {
                    msg: &buf[..read_len],
                    aad: &aad,
                },
            )
            .map_err(|_| "attachment_encrypt_failed")?;
        dst.write_all(&ciphertext)
            .map_err(|_| "attachment_stage_unavailable")?;
        leaves.push(attachment_merkle_leaf(produced, &ciphertext));
        produced = produced.saturating_add(1);
        if read_len < capacity {
            break;
        }
    }
    dst.sync_all().map_err(|_| "attachment_stage_unavailable")?;
    if produced != part_count {
        let _ = fs::remove_file(&staged_path);
        return Err("attachment_shape_invalid");
    }
    let integrity_root = attachment_merkle_root(leaves).ok_or("attachment_shape_invalid")?;
    Ok(AttachmentTransferRecord {
        attachment_id,
        peer: peer.to_string(),
        direction: "out".to_string(),
        service_url: Some(service_url.to_string()),
        state: "STAGED".to_string(),
        plaintext_len,
        ciphertext_len,
        part_size_class,
        part_count,
        integrity_alg: ATTACHMENT_INTEGRITY_ALG_V1.to_string(),
        integrity_root,
        retention_class: "standard".to_string(),
        enc_ctx_alg: ATTACHMENT_ENC_CTX_ALG_V1.to_string(),
        enc_ctx_b64u,
        locator_kind: None,
        locator_ref: None,
        fetch_capability: None,
        expires_at_unix_s: None,
        confirm_requested: receipt.is_some(),
        confirm_handle: None,
        filename_hint,
        media_type: None,
        source_path: Some(path.to_string_lossy().to_string()),
        staged_ciphertext_rel: Some(staged_rel),
        session_ref: None,
        resume_token: None,
        timeline_id: None,
        target_device_id: None,
        uploaded_parts: Vec::new(),
        downloaded_ciphertext_bytes: 0,
        download_ciphertext_rel: None,
        download_output_name: None,
        last_error: None,
    })
}

fn attachment_read_staged_part(
    cfg_dir: &Path,
    record: &AttachmentTransferRecord,
    part_index: u32,
) -> Result<Vec<u8>, &'static str> {
    let rel = record
        .staged_ciphertext_rel
        .as_deref()
        .ok_or("attachment_stage_missing")?;
    let path = attachment_path_from_rel(cfg_dir, rel)?;
    let mut file = File::open(path).map_err(|_| "attachment_stage_missing")?;
    let part_size = attachment_part_size_bytes(&record.part_size_class)
        .ok_or("attachment_shape_invalid")? as u64;
    let offset = u64::from(part_index) * part_size;
    let len = attachment_ciphertext_part_len(
        part_index,
        record.plaintext_len,
        &record.part_size_class,
        record.part_count,
        record.ciphertext_len,
    )
    .ok_or("attachment_shape_invalid")?;
    use std::io::Seek;
    use std::io::SeekFrom;
    file.seek(SeekFrom::Start(offset))
        .map_err(|_| "attachment_stage_missing")?;
    let mut buf = vec![0u8; len];
    file.read_exact(&mut buf)
        .map_err(|_| "attachment_stage_missing")?;
    Ok(buf)
}

fn attachment_upload_missing_parts(
    service_url: &str,
    record: &AttachmentTransferRecord,
) -> Result<(), String> {
    let session_ref = record
        .session_ref
        .as_deref()
        .ok_or_else(|| "attachment_session_missing".to_string())?;
    let resume_token = record
        .resume_token
        .as_deref()
        .ok_or_else(|| "attachment_resume_missing".to_string())?;
    let status = attachment_service_status(service_url, session_ref, resume_token)?;
    let (cfg_dir, _) = config_dir().map_err(|_| "attachment_stage_unavailable".to_string())?;
    let abort_after = env::var("QSC_ATTACHMENT_TEST_ABORT_AFTER_UPLOAD_PARTS")
        .ok()
        .and_then(|v| v.parse::<u32>().ok());
    let mut uploaded = 0u32;
    for range in status.missing_part_ranges {
        for part_index in range.start..=range.end {
            let bytes = attachment_read_staged_part(&cfg_dir, record, part_index)
                .map_err(|e| e.to_string())?;
            attachment_service_upload_part(
                service_url,
                session_ref,
                part_index,
                resume_token,
                bytes,
            )?;
            uploaded = uploaded.saturating_add(1);
            let short_id = file_delivery_short_id(&record.attachment_id);
            let part_s = part_index.to_string();
            emit_marker(
                "attachment_upload_part",
                None,
                &[
                    ("id", short_id.as_str()),
                    ("part", part_s.as_str()),
                    ("ok", "true"),
                ],
            );
            if abort_after.is_some_and(|limit| uploaded >= limit) {
                return Err("attachment_test_interrupt_upload".to_string());
            }
        }
    }
    Ok(())
}

fn attachment_build_descriptor(record: &AttachmentTransferRecord) -> Result<Vec<u8>, &'static str> {
    let expires_at = record
        .expires_at_unix_s
        .ok_or("attachment_descriptor_missing")?;
    let confirm_handle = if record.confirm_requested {
        Some(attachment_confirm_handle(AttachmentConfirmHandleInput {
            attachment_id: &record.attachment_id,
            plaintext_len: record.plaintext_len,
            ciphertext_len: record.ciphertext_len,
            part_size_class: &record.part_size_class,
            part_count: record.part_count,
            integrity_alg: &record.integrity_alg,
            integrity_root: &record.integrity_root,
            retention_class: &record.retention_class,
            expires_at_unix_s: expires_at,
        }))
    } else {
        None
    };
    let payload = AttachmentDescriptorPayload {
        v: ATTACHMENT_DESCRIPTOR_VERSION,
        t: ATTACHMENT_DESCRIPTOR_TYPE.to_string(),
        attachment_id: record.attachment_id.clone(),
        plaintext_len: record.plaintext_len,
        ciphertext_len: record.ciphertext_len,
        part_size_class: record.part_size_class.clone(),
        part_count: record.part_count,
        integrity_alg: record.integrity_alg.clone(),
        integrity_root: record.integrity_root.clone(),
        locator_kind: record
            .locator_kind
            .clone()
            .ok_or("attachment_descriptor_missing")?,
        locator_ref: record
            .locator_ref
            .clone()
            .ok_or("attachment_descriptor_missing")?,
        fetch_capability: record
            .fetch_capability
            .clone()
            .ok_or("attachment_descriptor_missing")?,
        enc_ctx_alg: record.enc_ctx_alg.clone(),
        enc_ctx_b64u: record.enc_ctx_b64u.clone(),
        retention_class: record.retention_class.clone(),
        expires_at_unix_s: expires_at,
        confirm_requested: record.confirm_requested,
        confirm_handle,
        filename_hint: record.filename_hint.clone(),
        media_type: record.media_type.clone(),
    };
    serde_json::to_vec(&payload).map_err(|_| "attachment_descriptor_encode_failed")
}

struct AttachmentSendExec<'a> {
    to: &'a str,
    path: &'a Path,
    relay: &'a str,
    service_url: &'a str,
    allow_legacy_sized: bool,
    max_file_size: Option<usize>,
    max_parts: Option<usize>,
    receipt: Option<ReceiptKind>,
}

fn attachment_send_execute(args: AttachmentSendExec<'_>) -> Result<(), String> {
    let AttachmentSendExec {
        to,
        path,
        relay,
        service_url,
        allow_legacy_sized,
        max_file_size,
        max_parts,
        receipt,
    } = args;
    if let Err(code) = enforce_peer_not_blocked(to) {
        return Err(code.to_string());
    }
    if let Err(code) = enforce_cli_send_contact_trust(to) {
        return Err(code.to_string());
    }
    if let Err(reason) = protocol_active_or_reason_for_send_peer(to) {
        protocol_inactive_exit(reason.as_str());
    }
    let routing = resolve_send_routing_target(to).map_err(|e| e.to_string())?;
    let effective_limit = max_file_size.unwrap_or(ATTACHMENT_DEFAULT_MAX_FILE_SIZE);
    let effective_max_parts = max_parts.unwrap_or(ATTACHMENT_DEFAULT_MAX_PARTS);
    let payload_len = fs::metadata(path)
        .map_err(|_| "file_xfer_read_failed".to_string())?
        .len() as usize;
    if payload_len <= ATTACHMENT_LEGACY_THRESHOLD_BYTES && !allow_legacy_sized {
        return Err("attachment_path_requires_large_file".to_string());
    }
    if payload_len > effective_limit {
        return Err("size_exceeds_max".to_string());
    }
    let mut journal = attachment_journal_load().map_err(|e| e.to_string())?;
    let (record_key, mut record) = match attachment_find_outbound_by_source(&journal, to, path) {
        Some((key, existing)) => (key, existing),
        None => {
            let fresh = attachment_build_outbound_record(to, service_url, path, receipt)
                .map_err(|e| e.to_string())?;
            let key = attachment_record_key("out", to, &fresh.attachment_id);
            (key, fresh)
        }
    };
    if record.part_count as usize > effective_max_parts {
        return Err("chunk_count_exceeds_max".to_string());
    }
    if record.state == "PEER_CONFIRMED" || record.state == "AWAITING_CONFIRMATION" {
        return Err("attachment_send_inflight".to_string());
    }
    record.service_url = Some(service_url.to_string());
    record.target_device_id = Some(short_device_marker(&routing.device_id));
    journal.records.insert(record_key.clone(), record.clone());
    attachment_journal_save(&journal).map_err(|e| e.to_string())?;

    if record.session_ref.is_none() {
        let created = attachment_service_create_session(service_url, &record)?;
        record.session_ref = Some(created.session_ref);
        record.resume_token = Some(created.resume_token);
        record.state = "SESSION_CREATED".to_string();
        journal.records.insert(record_key.clone(), record.clone());
        attachment_journal_save(&journal).map_err(|e| e.to_string())?;
    }

    record.state = "UPLOADING".to_string();
    journal.records.insert(record_key.clone(), record.clone());
    attachment_journal_save(&journal).map_err(|e| e.to_string())?;
    attachment_upload_missing_parts(service_url, &record)?;

    let session_ref = record
        .session_ref
        .clone()
        .ok_or_else(|| "attachment_session_missing".to_string())?;
    let resume_token = record
        .resume_token
        .clone()
        .ok_or_else(|| "attachment_resume_missing".to_string())?;
    let committed = attachment_service_commit(service_url, &session_ref, &resume_token, &record)?;
    record.locator_kind = Some(committed.locator_kind);
    record.locator_ref = Some(committed.locator_ref);
    record.fetch_capability = Some(committed.fetch_capability);
    record.expires_at_unix_s = Some(committed.expires_at_unix_s);
    record.confirm_handle = if record.confirm_requested {
        Some(attachment_confirm_handle(AttachmentConfirmHandleInput {
            attachment_id: &record.attachment_id,
            plaintext_len: record.plaintext_len,
            ciphertext_len: record.ciphertext_len,
            part_size_class: &record.part_size_class,
            part_count: record.part_count,
            integrity_alg: &record.integrity_alg,
            integrity_root: &record.integrity_root,
            retention_class: &record.retention_class,
            expires_at_unix_s: committed.expires_at_unix_s,
        }))
    } else {
        None
    };
    record.state = "COMMITTED".to_string();
    journal.records.insert(record_key.clone(), record.clone());
    attachment_journal_save(&journal).map_err(|e| e.to_string())?;
    let short_id = file_delivery_short_id(&record.attachment_id);
    emit_marker(
        "attachment_service_commit",
        None,
        &[("id", short_id.as_str()), ("ok", "true")],
    );

    let descriptor = attachment_build_descriptor(&record).map_err(|e| e.to_string())?;
    let outcome = relay_send_with_payload(RelaySendPayloadArgs {
        to,
        payload: descriptor,
        relay,
        injector: fault_injector_from_env(),
        pad_cfg: None,
        bucket_max: None,
        meta_seed: None,
        receipt: None,
        routing_override: None,
        tui_thread: None,
    });
    if let Some(code) = outcome.error_code {
        return Err(code.to_string());
    }
    record.timeline_id = latest_outbound_file_id(to).ok();
    record.state = if record.confirm_requested {
        "AWAITING_CONFIRMATION".to_string()
    } else {
        "ACCEPTED_BY_RELAY".to_string()
    };
    record.last_error = None;
    journal.records.insert(record_key, record);
    attachment_journal_save(&journal).map_err(|e| e.to_string())?;
    emit_cli_file_delivery_with_device(
        to,
        "accepted_by_relay",
        path.file_name()
            .and_then(|v| v.to_str())
            .unwrap_or("attachment.bin"),
        Some(routing.device_id.as_str()),
    );
    if receipt.is_some() {
        emit_cli_file_delivery_with_device(
            to,
            "awaiting_confirmation",
            path.file_name()
                .and_then(|v| v.to_str())
                .unwrap_or("attachment.bin"),
            Some(routing.device_id.as_str()),
        );
    }
    Ok(())
}

fn attachment_record_matches_descriptor(
    record: &AttachmentTransferRecord,
    desc: &AttachmentDescriptorPayload,
) -> bool {
    record.attachment_id == desc.attachment_id
        && record.plaintext_len == desc.plaintext_len
        && record.ciphertext_len == desc.ciphertext_len
        && record.part_size_class == desc.part_size_class
        && record.part_count == desc.part_count
        && record.integrity_alg == desc.integrity_alg
        && record.integrity_root == desc.integrity_root
        && record.enc_ctx_alg == desc.enc_ctx_alg
        && record.enc_ctx_b64u == desc.enc_ctx_b64u
        && record.retention_class == desc.retention_class
        && record.locator_kind.as_deref() == Some(desc.locator_kind.as_str())
        && record.locator_ref.as_deref() == Some(desc.locator_ref.as_str())
        && record.fetch_capability.as_deref() == Some(desc.fetch_capability.as_str())
        && record.expires_at_unix_s == Some(desc.expires_at_unix_s)
        && record.confirm_requested == desc.confirm_requested
        && record.confirm_handle == desc.confirm_handle
}

fn attachment_validate_descriptor(
    desc: &AttachmentDescriptorPayload,
    max_file_size: usize,
    max_parts: usize,
) -> Result<(), &'static str> {
    if !attachment_is_lower_hex_len(&desc.attachment_id, 64) {
        return Err("REJECT_ATT_DESC_FIELD_DOMAIN");
    }
    if desc.plaintext_len == 0 || desc.ciphertext_len == 0 || desc.part_count == 0 {
        return Err("REJECT_ATT_DESC_MISSING_REQUIRED_FIELD");
    }
    if desc.plaintext_len as usize > max_file_size || desc.part_count as usize > max_parts {
        return Err("REJECT_ATT_DESC_POLICY");
    }
    if attachment_part_size_bytes(&desc.part_size_class).is_none() {
        return Err("REJECT_ATT_DESC_FIELD_DOMAIN");
    }
    if desc.integrity_alg != ATTACHMENT_INTEGRITY_ALG_V1
        || desc.locator_kind != ATTACHMENT_LOCATOR_KIND_V1
    {
        return Err("REJECT_ATT_DESC_FIELD_DOMAIN");
    }
    if desc.locator_ref.trim().is_empty() || desc.locator_ref.len() > 128 {
        return Err("REJECT_ATT_DESC_LOCATOR_PLACEMENT");
    }
    if desc.fetch_capability.len() < 32 || desc.fetch_capability.len() > 255 {
        return Err("REJECT_ATT_DESC_LOCATOR_PLACEMENT");
    }
    if desc.enc_ctx_alg != ATTACHMENT_ENC_CTX_ALG_V1 {
        return Err("REJECT_ATT_DESC_ENC_CTX");
    }
    let _ = attachment_decode_enc_ctx(&desc.enc_ctx_b64u)?;
    if desc.retention_class != "short"
        && desc.retention_class != "standard"
        && desc.retention_class != "extended"
    {
        return Err("REJECT_ATT_DESC_FIELD_DOMAIN");
    }
    if desc.expires_at_unix_s <= attachment_now_unix_s() {
        return Err("REJECT_ATT_DESC_EXPIRED");
    }
    if desc.confirm_requested != desc.confirm_handle.is_some() {
        return Err("REJECT_ATT_DESC_MISSING_REQUIRED_FIELD");
    }
    if let Some(handle) = desc.confirm_handle.as_deref() {
        if !attachment_is_lower_hex_len(handle, 24) {
            return Err("REJECT_ATT_DESC_FIELD_DOMAIN");
        }
    }
    if let Some(name) = desc.filename_hint.as_deref() {
        let _ = attachment_validate_filename_hint(name)?;
    }
    let expected_part_count =
        attachment_part_count_for_plaintext(desc.plaintext_len, &desc.part_size_class)
            .ok_or("REJECT_ATT_DESC_INCONSISTENT_SHAPE")?;
    let expected_ciphertext_len =
        attachment_ciphertext_len_for_plaintext(desc.plaintext_len, expected_part_count)
            .ok_or("REJECT_ATT_DESC_INCONSISTENT_SHAPE")?;
    if expected_part_count != desc.part_count || expected_ciphertext_len != desc.ciphertext_len {
        return Err("REJECT_ATT_DESC_INCONSISTENT_SHAPE");
    }
    Ok(())
}

fn attachment_inbound_record_from_descriptor(
    peer: &str,
    service_url: Option<&str>,
    desc: &AttachmentDescriptorPayload,
) -> AttachmentTransferRecord {
    AttachmentTransferRecord {
        attachment_id: desc.attachment_id.clone(),
        peer: peer.to_string(),
        direction: "in".to_string(),
        service_url: service_url.map(|v| v.to_string()),
        state: "PENDING_FETCH".to_string(),
        plaintext_len: desc.plaintext_len,
        ciphertext_len: desc.ciphertext_len,
        part_size_class: desc.part_size_class.clone(),
        part_count: desc.part_count,
        integrity_alg: desc.integrity_alg.clone(),
        integrity_root: desc.integrity_root.clone(),
        retention_class: desc.retention_class.clone(),
        enc_ctx_alg: desc.enc_ctx_alg.clone(),
        enc_ctx_b64u: desc.enc_ctx_b64u.clone(),
        locator_kind: Some(desc.locator_kind.clone()),
        locator_ref: Some(desc.locator_ref.clone()),
        fetch_capability: Some(desc.fetch_capability.clone()),
        expires_at_unix_s: Some(desc.expires_at_unix_s),
        confirm_requested: desc.confirm_requested,
        confirm_handle: desc.confirm_handle.clone(),
        filename_hint: desc.filename_hint.clone(),
        media_type: desc.media_type.clone(),
        source_path: None,
        staged_ciphertext_rel: None,
        session_ref: None,
        resume_token: None,
        timeline_id: None,
        target_device_id: None,
        uploaded_parts: Vec::new(),
        downloaded_ciphertext_bytes: 0,
        download_ciphertext_rel: Some(attachment_inbound_rel(&desc.attachment_id)),
        download_output_name: Some(
            desc.filename_hint
                .as_deref()
                .and_then(|v| attachment_validate_filename_hint(v).ok())
                .unwrap_or_else(|| {
                    format!(
                        "attachment-{}.bin",
                        file_delivery_short_id(&desc.attachment_id)
                    )
                }),
        ),
        last_error: None,
    }
}

enum AttachmentFetchOutcome {
    Complete(u64),
    Interrupted(u64),
}

fn attachment_fetch_ciphertext(
    service_url: &str,
    record: &AttachmentTransferRecord,
    cfg_dir: &Path,
) -> Result<AttachmentFetchOutcome, String> {
    let rel = record
        .download_ciphertext_rel
        .as_deref()
        .ok_or_else(|| "attachment_stage_missing".to_string())?;
    let path = attachment_path_from_rel(cfg_dir, rel).map_err(|e| e.to_string())?;
    if let Some(parent) = path.parent() {
        ensure_dir_secure(parent, ConfigSource::EnvOverride)
            .map_err(|_| "attachment_stage_unavailable".to_string())?;
    }
    let mut existing_len = fs::metadata(&path).map(|v| v.len()).unwrap_or(0);
    let locator_ref = record
        .locator_ref
        .as_deref()
        .ok_or_else(|| "REJECT_ATT_DESC_LOCATOR_PLACEMENT".to_string())?;
    let fetch_capability = env::var("QSC_ATTACHMENT_FETCH_CAPABILITY_OVERRIDE")
        .ok()
        .filter(|v| !v.trim().is_empty())
        .or_else(|| record.fetch_capability.clone())
        .ok_or_else(|| "REJECT_ATT_DESC_LOCATOR_PLACEMENT".to_string())?;
    let url = format!("{service_url}/v1/attachments/objects/{locator_ref}");
    let client = HttpClient::new();
    let mut req = client
        .get(url)
        .header("X-QATT-Fetch-Capability", fetch_capability);
    if existing_len > 0 && existing_len < record.ciphertext_len {
        req = req.header(
            "Range",
            format!("bytes={existing_len}-{}", record.ciphertext_len - 1),
        );
    }
    let mut response = req
        .send()
        .map_err(|_| "attachment_fetch_failed".to_string())?;
    if !(response.status().is_success() || response.status() == HttpStatus::PARTIAL_CONTENT) {
        return Err(attachment_service_reason(
            response,
            "attachment_fetch_failed",
        ));
    }
    let restart = existing_len > 0 && response.status() == HttpStatus::OK;
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(!restart && existing_len > 0)
        .truncate(restart || existing_len == 0)
        .open(&path)
        .map_err(|_| "attachment_stage_unavailable".to_string())?;
    #[cfg(unix)]
    enforce_file_perms(&path).map_err(|_| "attachment_stage_unavailable".to_string())?;
    if restart {
        existing_len = 0;
    }
    let abort_after = env::var("QSC_ATTACHMENT_TEST_ABORT_AFTER_FETCH_BYTES")
        .ok()
        .and_then(|v| v.parse::<u64>().ok());
    let mut downloaded = existing_len;
    let mut buf = [0u8; 8192];
    loop {
        let n = response
            .read(&mut buf)
            .map_err(|_| "attachment_fetch_failed".to_string())?;
        if n == 0 {
            break;
        }
        file.write_all(&buf[..n])
            .map_err(|_| "attachment_stage_unavailable".to_string())?;
        downloaded = downloaded.saturating_add(n as u64);
        if abort_after.is_some_and(|limit| downloaded >= limit) {
            file.sync_all()
                .map_err(|_| "attachment_stage_unavailable".to_string())?;
            return Ok(AttachmentFetchOutcome::Interrupted(downloaded));
        }
    }
    file.sync_all()
        .map_err(|_| "attachment_stage_unavailable".to_string())?;
    Ok(AttachmentFetchOutcome::Complete(downloaded))
}

fn attachment_verify_ciphertext_root(
    cfg_dir: &Path,
    record: &AttachmentTransferRecord,
) -> Result<(), &'static str> {
    let rel = record
        .download_ciphertext_rel
        .as_deref()
        .ok_or("REJECT_ATT_CIPHERTEXT_PRECHECK")?;
    let path = attachment_path_from_rel(cfg_dir, rel)?;
    let actual_len = fs::metadata(&path)
        .map_err(|_| "REJECT_ATT_CIPHERTEXT_PRECHECK")?
        .len();
    if actual_len != record.ciphertext_len {
        return Err("REJECT_ATT_CIPHERTEXT_PRECHECK");
    }
    let mut file = File::open(path).map_err(|_| "REJECT_ATT_CIPHERTEXT_PRECHECK")?;
    let mut leaves = Vec::with_capacity(record.part_count as usize);
    for part_index in 0..record.part_count {
        let expected_len = attachment_ciphertext_part_len(
            part_index,
            record.plaintext_len,
            &record.part_size_class,
            record.part_count,
            record.ciphertext_len,
        )
        .ok_or("REJECT_ATT_CIPHERTEXT_PRECHECK")?;
        let mut buf = vec![0u8; expected_len];
        file.read_exact(&mut buf)
            .map_err(|_| "REJECT_ATT_CIPHERTEXT_PRECHECK")?;
        leaves.push(attachment_merkle_leaf(part_index, &buf));
    }
    let root = attachment_merkle_root(leaves).ok_or("REJECT_ATT_CIPHERTEXT_PRECHECK")?;
    if root != record.integrity_root {
        return Err("REJECT_ATT_CIPHERTEXT_PRECHECK");
    }
    Ok(())
}

fn attachment_decrypt_to_output(
    cfg_dir: &Path,
    out_dir: &Path,
    source: ConfigSource,
    record: &AttachmentTransferRecord,
) -> Result<(), &'static str> {
    let rel = record
        .download_ciphertext_rel
        .as_deref()
        .ok_or("REJECT_ATT_DECRYPT_CTX_MISMATCH")?;
    let ciphertext_path = attachment_path_from_rel(cfg_dir, rel)?;
    let mut src = File::open(ciphertext_path).map_err(|_| "REJECT_ATT_DECRYPT_CTX_MISMATCH")?;
    let (mut cek, nonce_prefix) = attachment_decode_enc_ctx(&record.enc_ctx_b64u)?;
    let cipher = ChaCha20Poly1305::new(Key::from_slice(&cek));
    let output_name = record
        .download_output_name
        .clone()
        .unwrap_or_else(|| attachment_output_name(record));
    let final_path = out_dir.join(output_name);
    enforce_safe_parents(&final_path, source).map_err(|_| "REJECT_ATT_PLAINTEXT_SHAPE")?;
    let tmp_path = out_dir.join(format!(
        ".{}.tmp.{}",
        file_delivery_short_id(&record.attachment_id),
        process::id()
    ));
    let _ = fs::remove_file(&tmp_path);
    let mut dst = OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(&tmp_path)
        .map_err(|_| "REJECT_ATT_PLAINTEXT_SHAPE")?;
    #[cfg(unix)]
    enforce_file_perms(&tmp_path).map_err(|_| "REJECT_ATT_PLAINTEXT_SHAPE")?;
    let mut plaintext_len = 0u64;
    for part_index in 0..record.part_count {
        let ct_len = attachment_ciphertext_part_len(
            part_index,
            record.plaintext_len,
            &record.part_size_class,
            record.part_count,
            record.ciphertext_len,
        )
        .ok_or("REJECT_ATT_DECRYPT_CTX_MISMATCH")?;
        let mut ciphertext = vec![0u8; ct_len];
        src.read_exact(&mut ciphertext)
            .map_err(|_| "REJECT_ATT_DECRYPT_AUTH")?;
        let nonce = attachment_nonce(&nonce_prefix, part_index);
        let aad = attachment_part_aad(
            &record.attachment_id,
            &record.enc_ctx_alg,
            record.plaintext_len,
            record.ciphertext_len,
            &record.part_size_class,
            record.part_count,
            part_index,
        );
        let plaintext = cipher
            .decrypt(
                Nonce::from_slice(&nonce),
                Payload {
                    msg: &ciphertext,
                    aad: &aad,
                },
            )
            .map_err(|_| "REJECT_ATT_DECRYPT_AUTH")?;
        plaintext_len = plaintext_len.saturating_add(plaintext.len() as u64);
        dst.write_all(&plaintext)
            .map_err(|_| "REJECT_ATT_PLAINTEXT_SHAPE")?;
    }
    dst.sync_all().map_err(|_| "REJECT_ATT_PLAINTEXT_SHAPE")?;
    if plaintext_len != record.plaintext_len {
        let _ = fs::remove_file(&tmp_path);
        return Err("REJECT_ATT_PLAINTEXT_SHAPE");
    }
    fs::rename(&tmp_path, &final_path).map_err(|_| "REJECT_ATT_PLAINTEXT_SHAPE")?;
    cek.zeroize();
    Ok(())
}

fn attachment_process_inbound_record(
    ctx: &ReceivePullCtx<'_>,
    record_key: &str,
) -> Result<Option<(String, String)>, &'static str> {
    let Some(service_url) = ctx.attachment_service else {
        return Ok(None);
    };
    let mut journal = attachment_journal_load()?;
    let mut record = journal
        .records
        .get(record_key)
        .cloned()
        .ok_or("REJECT_ATT_DECRYPT_CTX_MISMATCH")?;
    record.service_url = Some(service_url.to_string());
    let (cfg_dir, _) = config_dir().map_err(|_| "attachment_stage_unavailable")?;
    match attachment_fetch_ciphertext(service_url, &record, &cfg_dir) {
        Ok(AttachmentFetchOutcome::Interrupted(downloaded)) => {
            record.downloaded_ciphertext_bytes = downloaded;
            record.state = "DOWNLOADING".to_string();
            journal.records.insert(record_key.to_string(), record);
            attachment_journal_save(&journal)?;
            return Err("attachment_test_interrupt_download");
        }
        Ok(AttachmentFetchOutcome::Complete(downloaded)) => {
            record.downloaded_ciphertext_bytes = downloaded;
            record.state = "FETCHED".to_string();
        }
        Err(code) => {
            record.last_error = Some(code.clone());
            journal.records.insert(record_key.to_string(), record);
            attachment_journal_save(&journal)?;
            return Err(Box::leak(code.into_boxed_str()));
        }
    }
    journal
        .records
        .insert(record_key.to_string(), record.clone());
    attachment_journal_save(&journal)?;
    attachment_verify_ciphertext_root(&cfg_dir, &record)?;
    attachment_decrypt_to_output(&cfg_dir, ctx.out, ctx.source, &record)?;
    record.state = "VERIFIED".to_string();
    record.last_error = None;
    journal
        .records
        .insert(record_key.to_string(), record.clone());
    attachment_journal_save(&journal)?;
    timeline_append_entry(
        ctx.from,
        "in",
        record.plaintext_len as usize,
        "file",
        MessageState::Received,
        None,
    )?;
    if record.confirm_requested
        && ctx.receipt_policy.file_confirm_mode == FileConfirmEmitMode::CompleteOnly
    {
        return Ok(Some((
            record.attachment_id.clone(),
            record
                .confirm_handle
                .clone()
                .ok_or("REJECT_ATT_CONFIRM_EARLY")?,
        )));
    }
    Ok(None)
}

fn attachment_handle_descriptor(
    ctx: &ReceivePullCtx<'_>,
    desc: AttachmentDescriptorPayload,
) -> Result<Option<(String, String)>, &'static str> {
    attachment_validate_descriptor(&desc, ctx.file_max_size, ctx.file_max_chunks)?;
    let mut journal = attachment_journal_load()?;
    let key = attachment_record_key("in", ctx.from, &desc.attachment_id);
    let mut record = match journal.records.get(&key).cloned() {
        Some(existing) => {
            if !attachment_record_matches_descriptor(&existing, &desc) {
                return Err("REJECT_ATT_DECRYPT_CTX_MISMATCH");
            }
            existing
        }
        None => attachment_inbound_record_from_descriptor(ctx.from, ctx.attachment_service, &desc),
    };
    record.service_url = ctx.attachment_service.map(|v| v.to_string());
    record.state = "PENDING_FETCH".to_string();
    journal.records.insert(key.clone(), record);
    attachment_journal_save(&journal)?;
    if ctx.attachment_service.is_none() {
        emit_marker(
            "attachment_pending_service",
            None,
            &[
                ("id", file_delivery_short_id(&desc.attachment_id).as_str()),
                ("ok", "true"),
            ],
        );
        return Ok(None);
    }
    attachment_process_inbound_record(ctx, &key)
}

fn attachment_resume_pending_for_peer(
    ctx: &ReceivePullCtx<'_>,
    service_url: &str,
) -> Result<usize, &'static str> {
    let journal = attachment_journal_load()?;
    let pending: Vec<String> = journal
        .records
        .iter()
        .filter(|(_, rec)| {
            rec.direction == "in"
                && rec.peer == ctx.from
                && rec.service_url.as_deref().unwrap_or(service_url) == service_url
                && matches!(
                    rec.state.as_str(),
                    "PENDING_FETCH" | "DOWNLOADING" | "FETCHED"
                )
        })
        .map(|(key, _)| key.clone())
        .collect();
    drop(journal);
    let mut resumed = 0usize;
    for key in pending {
        if let Some((attachment_id, confirm_handle)) = attachment_process_inbound_record(ctx, &key)?
        {
            resumed = resumed.saturating_add(1);
            let item = PendingReceipt::AttachmentComplete {
                attachment_id,
                confirm_handle,
            };
            match ctx.receipt_policy.mode {
                ReceiptEmitMode::Off => {
                    emit_cli_receipt_policy_event(
                        ctx.receipt_policy.mode,
                        "skipped",
                        "attachment_complete",
                        ctx.from,
                    );
                    emit_tui_receipt_policy_event(
                        ctx.receipt_policy.mode,
                        "skipped",
                        "attachment_complete",
                        ctx.from,
                    );
                }
                ReceiptEmitMode::Immediate | ReceiptEmitMode::Batched => {
                    send_pending_receipt(ctx, item);
                }
            }
        }
    }
    Ok(resumed)
}

fn file_xfer_reject(id: &str, reason: &str) -> ! {
    emit_marker(
        "file_xfer_reject",
        Some(reason),
        &[("id", id), ("reason", reason)],
    );
    print_error_marker(reason);
}

fn file_xfer_store_key(peer: &str, file_id: &str) -> String {
    format!("{}:{}", peer, file_id)
}

const FILE_PUSH_MAX_ATTEMPTS: usize = 3;
const FILE_PUSH_RETRY_BASE_BACKOFF_MS: u64 = 50;

fn file_push_retryable(code: &str) -> bool {
    matches!(
        code,
        "relay_inbox_push_failed"
            | "relay_inbox_queue_full"
            | "relay_network_timeout"
            | "relay_network_unreachable"
            | "relay_http_failure"
    )
}

fn emit_file_push_retry(attempt: usize, backoff_ms: u64, reason: &str) {
    let attempt_s = attempt.to_string();
    let backoff_s = backoff_ms.to_string();
    emit_marker(
        "file_push_retry",
        None,
        &[
            ("attempt", attempt_s.as_str()),
            ("backoff_ms", backoff_s.as_str()),
            ("reason", reason),
        ],
    );
    emit_cli_named_marker(
        "QSC_FILE_PUSH_RETRY",
        &[
            ("attempt", attempt_s.as_str()),
            ("backoff_ms", backoff_s.as_str()),
            ("reason", reason),
        ],
    );
}

fn emit_file_integrity_fail(reason: &str, action: &str) {
    emit_cli_named_marker(
        "QSC_FILE_INTEGRITY_FAIL",
        &[("reason", reason), ("action", action)],
    );
    emit_tui_named_marker(
        "QSC_TUI_FILE_INTEGRITY_FAIL",
        &[("reason", reason), ("action", action)],
    );
}

fn file_transfer_fail_clean(peer: &str, file_id: &str, reason: &str) -> Result<(), &'static str> {
    let key = file_xfer_store_key(peer, file_id);
    let mut store = timeline_store_load().map_err(|_| "timeline_unavailable")?;
    if let Some(rec) = store.file_transfers.get_mut(&key) {
        rec.state = "FAILED".to_string();
        rec.chunk_hashes.clear();
        rec.chunks_hex.clear();
        rec.confirm_requested = false;
        rec.confirm_id = None;
        timeline_store_save(&store).map_err(|_| "timeline_unavailable")?;
        emit_file_integrity_fail(reason, "purge_partials");
        emit_marker(
            "file_xfer_fail_clean",
            None,
            &[
                ("id", file_id),
                ("reason", reason),
                ("action", "purge_partials"),
            ],
        );
        return Ok(());
    }
    emit_file_integrity_fail(reason, "rotate_mailbox_hint");
    Ok(())
}

fn relay_send_file_payload_with_retry(to: &str, payload: Vec<u8>, relay: &str) -> RelaySendOutcome {
    let mut attempt = 1usize;
    loop {
        let outcome = relay_send_with_payload(RelaySendPayloadArgs {
            to,
            payload: payload.clone(),
            relay,
            injector: fault_injector_from_env(),
            pad_cfg: None,
            bucket_max: None,
            meta_seed: None,
            receipt: None,
            routing_override: None,
            tui_thread: None,
        });
        let Some(code) = outcome.error_code else {
            return outcome;
        };
        if !file_push_retryable(code) || attempt >= FILE_PUSH_MAX_ATTEMPTS {
            return outcome;
        }
        let backoff_ms = FILE_PUSH_RETRY_BASE_BACKOFF_MS * (1u64 << (attempt - 1));
        emit_file_push_retry(attempt, backoff_ms, code);
        std::thread::sleep(Duration::from_millis(backoff_ms));
        attempt += 1;
    }
}

fn file_transfer_upsert_outbound_record(
    peer: &str,
    file_id: &str,
    rec: FileTransferRecord,
) -> Result<(), &'static str> {
    let key = file_xfer_store_key(peer, file_id);
    let mut store = timeline_store_load().map_err(|_| "timeline_unavailable")?;
    store.file_transfers.insert(key, rec);
    timeline_store_save(&store).map_err(|_| "timeline_unavailable")
}

fn file_transfer_apply_confirmation(
    peer: &str,
    file_id: &str,
    confirm_id: &str,
    recv_channel: &str,
) -> Result<(), &'static str> {
    let key = file_xfer_store_key(peer, file_id);
    let mut store = timeline_store_load().map_err(|_| "timeline_unavailable")?;
    let rec = store.file_transfers.get_mut(&key).ok_or("state_unknown")?;
    if !rec.confirm_requested {
        return Err("confirm_not_requested");
    }
    if rec.state == "PEER_CONFIRMED" {
        return Err("state_duplicate");
    }
    if rec.confirm_id.as_deref().unwrap_or("") != confirm_id {
        return Err("confirm_id_mismatch");
    }
    if !confirm_target_matches_channel(rec.target_device_id.as_deref(), recv_channel) {
        return Err("confirm_wrong_device");
    }
    rec.state = "PEER_CONFIRMED".to_string();
    timeline_store_save(&store).map_err(|_| "timeline_unavailable")
}

fn attachment_transfer_apply_confirmation(
    peer: &str,
    attachment_id: &str,
    confirm_handle: &str,
    recv_channel: &str,
) -> Result<(), &'static str> {
    let key = attachment_record_key("out", peer, attachment_id);
    let mut journal = attachment_journal_load()?;
    let rec = journal
        .records
        .get_mut(&key)
        .ok_or("REJECT_ATT_CONFIRM_LINKAGE")?;
    if !rec.confirm_requested {
        return Err("REJECT_ATT_CONFIRM_LINKAGE");
    }
    if rec.state == "PEER_CONFIRMED" {
        return Err("REJECT_ATT_CONFIRM_LINKAGE");
    }
    if rec.confirm_handle.as_deref() != Some(confirm_handle) {
        return Err("REJECT_ATT_CONFIRM_LINKAGE");
    }
    if !confirm_target_matches_channel(rec.target_device_id.as_deref(), recv_channel) {
        return Err("confirm_wrong_device");
    }
    rec.state = "PEER_CONFIRMED".to_string();
    attachment_journal_save(&journal)?;
    Ok(())
}

fn file_transfer_target_device(peer: &str, file_id: &str) -> Result<Option<String>, &'static str> {
    let key = file_xfer_store_key(peer, file_id);
    let store = timeline_store_load().map_err(|_| "timeline_unavailable")?;
    let rec = store.file_transfers.get(&key).ok_or("state_unknown")?;
    Ok(rec.target_device_id.clone())
}

fn file_transfer_confirm_id(peer: &str, file_id: &str) -> Result<String, &'static str> {
    let key = file_xfer_store_key(peer, file_id);
    let store = timeline_store_load().map_err(|_| "timeline_unavailable")?;
    let rec = store.file_transfers.get(&key).ok_or("state_unknown")?;
    rec.confirm_id.clone().ok_or("confirm_id_missing")
}

fn attachment_transfer_timeline_id(
    peer: &str,
    attachment_id: &str,
) -> Result<String, &'static str> {
    let key = attachment_record_key("out", peer, attachment_id);
    let store = attachment_journal_load()?;
    let rec = store
        .records
        .get(&key)
        .ok_or("REJECT_ATT_CONFIRM_LINKAGE")?;
    rec.timeline_id.clone().ok_or("REJECT_ATT_CONFIRM_LINKAGE")
}

struct FileSendExec<'a> {
    transport: Option<SendTransport>,
    relay: Option<&'a str>,
    attachment_service: Option<&'a str>,
    legacy_in_message_stage: Option<LegacyInMessageStage>,
    to: &'a str,
    path: &'a Path,
    chunk_size: usize,
    max_file_size: Option<usize>,
    max_chunks: Option<usize>,
    receipt: Option<ReceiptKind>,
}

fn file_send_execute(args: FileSendExec<'_>) {
    let FileSendExec {
        transport,
        relay,
        attachment_service,
        legacy_in_message_stage,
        to,
        path,
        chunk_size,
        max_file_size,
        max_chunks,
        receipt,
    } = args;
    if !require_unlocked("file_send") {
        return;
    }
    let legacy_in_message_stage = resolve_legacy_in_message_stage(legacy_in_message_stage)
        .unwrap_or_else(|code| file_xfer_reject("unknown", code));
    let path_len_hint = fs::metadata(path)
        .map(|v| v.len() as usize)
        .unwrap_or_else(|_| file_xfer_reject("unknown", "file_xfer_read_failed"));
    let size_class = if path_len_hint > ATTACHMENT_LEGACY_THRESHOLD_BYTES {
        "above_threshold"
    } else {
        "legacy_sized"
    };
    let use_attachment_path = path_len_hint > ATTACHMENT_LEGACY_THRESHOLD_BYTES
        || matches!(
            legacy_in_message_stage,
            LegacyInMessageStage::W1 | LegacyInMessageStage::W2
        );
    let path_kind = if use_attachment_path {
        "attachment"
    } else {
        "legacy_in_message"
    };
    emit_marker(
        "file_send_policy",
        None,
        &[
            (
                "stage",
                legacy_in_message_stage_name(legacy_in_message_stage),
            ),
            ("size_class", size_class),
            ("path", path_kind),
        ],
    );
    if use_attachment_path {
        let service_url = resolve_large_file_attachment_service(attachment_service)
            .unwrap_or_else(|code| file_xfer_reject("unknown", code));
        if chunk_size != FILE_XFER_DEFAULT_CHUNK_SIZE {
            file_xfer_reject("unknown", "attachment_chunk_flag_invalid");
        }
        let transport = match transport {
            Some(v) => v,
            None => file_xfer_reject("unknown", "file_xfer_transport_required"),
        };
        match transport {
            SendTransport::Relay => {}
        }
        let relay = match relay {
            Some(v) => v,
            None => file_xfer_reject("unknown", "file_xfer_relay_required"),
        };
        if let Err(code) = attachment_send_execute(AttachmentSendExec {
            to,
            path,
            relay,
            service_url: service_url.as_str(),
            allow_legacy_sized: path_len_hint <= ATTACHMENT_LEGACY_THRESHOLD_BYTES,
            max_file_size,
            max_parts: max_chunks,
            receipt,
        }) {
            file_xfer_reject("unknown", code.as_str());
        }
        return;
    }
    let transport = match transport {
        Some(v) => v,
        None => file_xfer_reject("unknown", "file_xfer_transport_required"),
    };
    match transport {
        SendTransport::Relay => {}
    }
    let relay = match relay {
        Some(v) => v,
        None => file_xfer_reject("unknown", "file_xfer_relay_required"),
    };
    if !channel_label_ok(to) {
        file_xfer_reject("unknown", "file_xfer_peer_invalid");
    }
    let max_file_size = max_file_size.unwrap_or(FILE_XFER_DEFAULT_MAX_FILE_SIZE);
    let max_chunks = max_chunks.unwrap_or(FILE_XFER_DEFAULT_MAX_CHUNKS);
    if max_file_size == 0 || max_file_size > FILE_XFER_MAX_FILE_SIZE_CEILING {
        file_xfer_reject("unknown", "file_xfer_size_bound_invalid");
    }
    if chunk_size == 0 || chunk_size > FILE_XFER_MAX_CHUNK_SIZE_CEILING {
        file_xfer_reject("unknown", "file_xfer_chunk_bound_invalid");
    }
    if max_chunks == 0 || max_chunks > FILE_XFER_MAX_CHUNKS_CEILING {
        file_xfer_reject("unknown", "file_xfer_chunks_bound_invalid");
    }
    if let Err(code) = enforce_peer_not_blocked(to) {
        file_xfer_reject("unknown", code);
    }
    let payload =
        fs::read(path).unwrap_or_else(|_| file_xfer_reject("unknown", "file_xfer_read_failed"));
    if payload.is_empty() {
        file_xfer_reject("unknown", "file_xfer_empty");
    }
    if payload.len() > max_file_size {
        file_xfer_reject("unknown", "size_exceeds_max");
    }
    let chunk_count = payload.len().div_ceil(chunk_size);
    if chunk_count > max_chunks {
        file_xfer_reject("unknown", "chunk_count_exceeds_max");
    }
    if let Err(code) = enforce_cli_send_contact_trust(to) {
        file_xfer_reject("unknown", code);
    }
    let routing = match resolve_send_routing_target(to) {
        Ok(v) => v,
        Err(code) => file_xfer_reject("unknown", code),
    };
    if let Err(reason) = protocol_active_or_reason_for_peer(routing.channel.as_str()) {
        emit_marker(
            "file_xfer_reject",
            Some("protocol_inactive"),
            &[
                ("id", "unknown"),
                ("reason", "protocol_inactive"),
                ("detail", reason.as_str()),
            ],
        );
        protocol_inactive_exit(reason.as_str());
    }
    let filename = path
        .file_name()
        .and_then(|v| v.to_str())
        .unwrap_or("file.bin")
        .to_string();
    let file_id = file_xfer_id(to, filename.as_str(), &payload);
    let size_s = payload.len().to_string();
    emit_marker(
        "file_xfer_prepare",
        None,
        &[
            ("id", file_id.as_str()),
            ("size", size_s.as_str()),
            ("ok", "true"),
        ],
    );
    let mut chunk_hashes = Vec::with_capacity(chunk_count);
    for idx in 0..chunk_count {
        let start = idx * chunk_size;
        let end = (start + chunk_size).min(payload.len());
        let chunk = &payload[start..end];
        chunk_hashes.push(file_xfer_chunk_hash(chunk));
    }
    let manifest_hash = file_xfer_manifest_hash(
        file_id.as_str(),
        payload.len(),
        chunk_count,
        chunk_hashes.as_slice(),
    );
    let confirm_requested = receipt.is_some();
    let confirm_id = file_xfer_confirm_id(file_id.as_str(), manifest_hash.as_str());

    for (idx, chunk_hash) in chunk_hashes.iter().enumerate() {
        let start = idx * chunk_size;
        let end = (start + chunk_size).min(payload.len());
        let chunk = payload[start..end].to_vec();
        let chunk_payload = FileTransferChunkPayload {
            v: FILE_XFER_VERSION,
            t: "file_chunk".to_string(),
            file_id: file_id.clone(),
            filename: filename.clone(),
            total_size: payload.len(),
            chunk_index: idx,
            chunk_count,
            chunk_hash: chunk_hash.clone(),
            manifest_hash: manifest_hash.clone(),
            chunk,
        };
        let body = serde_json::to_vec(&chunk_payload)
            .unwrap_or_else(|_| file_xfer_reject(file_id.as_str(), "file_xfer_encode_failed"));
        let outcome = relay_send_file_payload_with_retry(to, body, relay);
        if let Some(code) = outcome.error_code {
            file_xfer_reject(file_id.as_str(), code);
        }
        let idx_s = idx.to_string();
        emit_marker(
            "file_xfer_chunk",
            None,
            &[
                ("id", file_id.as_str()),
                ("idx", idx_s.as_str()),
                ("ok", "true"),
            ],
        );
    }

    let manifest = FileTransferManifestPayload {
        v: FILE_XFER_VERSION,
        t: "file_manifest".to_string(),
        file_id: file_id.clone(),
        filename,
        total_size: payload.len(),
        chunk_count,
        chunk_hashes,
        manifest_hash,
        confirm_requested,
        confirm_id: confirm_id.clone(),
    };
    let manifest_body = serde_json::to_vec(&manifest)
        .unwrap_or_else(|_| file_xfer_reject(file_id.as_str(), "file_xfer_encode_failed"));
    let outcome = relay_send_file_payload_with_retry(to, manifest_body, relay);
    if let Some(code) = outcome.error_code {
        file_xfer_reject(file_id.as_str(), code);
    }
    emit_marker(
        "file_xfer_manifest",
        None,
        &[("id", file_id.as_str()), ("ok", "true")],
    );
    if let Err(code) = timeline_append_entry_for_target(
        to,
        "out",
        payload.len(),
        "file",
        MessageState::Sent,
        Some(file_id.as_str()),
        Some(routing.device_id.as_str()),
    ) {
        emit_message_state_reject(file_id.as_str(), code);
        file_xfer_reject(file_id.as_str(), code);
    }
    let outbound = FileTransferRecord {
        id: file_id.clone(),
        peer: to.to_string(),
        filename: manifest.filename.clone(),
        total_size: payload.len(),
        chunk_count,
        manifest_hash: manifest.manifest_hash.clone(),
        chunk_hashes: Vec::new(),
        chunks_hex: Vec::new(),
        confirm_requested,
        confirm_id: if confirm_requested {
            Some(confirm_id.clone())
        } else {
            None
        },
        target_device_id: Some(short_device_marker(routing.device_id.as_str())),
        state: if confirm_requested {
            "AWAITING_CONFIRMATION".to_string()
        } else {
            "ACCEPTED_BY_RELAY".to_string()
        },
    };
    if let Err(code) = file_transfer_upsert_outbound_record(to, file_id.as_str(), outbound) {
        file_xfer_reject(file_id.as_str(), code);
    }
    emit_marker(
        "file_xfer_complete",
        None,
        &[("id", file_id.as_str()), ("ok", "true")],
    );
    emit_cli_confirm_policy();
    emit_cli_file_delivery_with_device(
        to,
        "accepted_by_relay",
        file_id.as_str(),
        Some(routing.device_id.as_str()),
    );
    if confirm_requested {
        emit_cli_file_delivery_with_device(
            to,
            "awaiting_confirmation",
            file_id.as_str(),
            Some(routing.device_id.as_str()),
        );
    }
}

fn file_transfer_handle_chunk(
    ctx: &ReceivePullCtx<'_>,
    chunk: FileTransferChunkPayload,
) -> Result<(), &'static str> {
    if chunk.total_size == 0 || chunk.total_size > ctx.file_max_size {
        return Err("size_exceeds_max");
    }
    if chunk.chunk_count == 0 || chunk.chunk_count > ctx.file_max_chunks {
        return Err("chunk_count_exceeds_max");
    }
    if chunk.chunk.len() > FILE_XFER_DEFAULT_CHUNK_SIZE {
        return Err("chunk_size_exceeds_max");
    }
    if chunk.chunk_index >= chunk.chunk_count {
        return Err("chunk_index_invalid");
    }
    if chunk.chunk_hash != file_xfer_chunk_hash(&chunk.chunk) {
        return Err("chunk_hash_invalid");
    }
    let key = file_xfer_store_key(ctx.from, chunk.file_id.as_str());
    let mut store = timeline_store_load().map_err(|_| "timeline_unavailable")?;
    let rec = store
        .file_transfers
        .entry(key)
        .or_insert_with(|| FileTransferRecord {
            id: chunk.file_id.clone(),
            peer: ctx.from.to_string(),
            filename: chunk.filename.clone(),
            total_size: chunk.total_size,
            chunk_count: chunk.chunk_count,
            manifest_hash: chunk.manifest_hash.clone(),
            chunk_hashes: Vec::new(),
            chunks_hex: Vec::new(),
            confirm_requested: false,
            confirm_id: None,
            target_device_id: None,
            state: "RECEIVING".to_string(),
        });
    if rec.state == "VERIFIED" {
        return Err("state_invalid_transition");
    }
    if chunk.chunk_index == 0 {
        if rec.state == "FAILED" {
            rec.filename = chunk.filename.clone();
            rec.total_size = chunk.total_size;
            rec.chunk_count = chunk.chunk_count;
            rec.manifest_hash = chunk.manifest_hash.clone();
            rec.chunk_hashes.clear();
            rec.chunks_hex.clear();
            rec.confirm_requested = false;
            rec.confirm_id = None;
            rec.state = "RECEIVING".to_string();
            emit_marker(
                "file_xfer_reset",
                None,
                &[("id", chunk.file_id.as_str()), ("reason", "rerun_detected")],
            );
        }
    } else if rec.state == "FAILED" {
        return Err("state_invalid_transition");
    }
    if rec.total_size != chunk.total_size
        || rec.chunk_count != chunk.chunk_count
        || rec.manifest_hash != chunk.manifest_hash
    {
        return Err("chunk_meta_mismatch");
    }
    let expected = rec.chunks_hex.len();
    if chunk.chunk_index != expected {
        return Err("chunk_order_invalid");
    }
    rec.chunk_hashes.push(chunk.chunk_hash.clone());
    rec.chunks_hex.push(hex_encode(&chunk.chunk));
    rec.state = "RECEIVING".to_string();
    timeline_store_save(&store).map_err(|_| "timeline_unavailable")?;
    let idx_s = chunk.chunk_index.to_string();
    emit_marker(
        "file_xfer_chunk",
        None,
        &[
            ("id", chunk.file_id.as_str()),
            ("idx", idx_s.as_str()),
            ("ok", "true"),
        ],
    );
    Ok(())
}

fn file_transfer_handle_manifest(
    ctx: &ReceivePullCtx<'_>,
    manifest: FileTransferManifestPayload,
) -> Result<Option<(String, String)>, &'static str> {
    if manifest.total_size == 0 || manifest.total_size > ctx.file_max_size {
        return Err("size_exceeds_max");
    }
    if manifest.chunk_count == 0 || manifest.chunk_count > ctx.file_max_chunks {
        return Err("chunk_count_exceeds_max");
    }
    let key = file_xfer_store_key(ctx.from, manifest.file_id.as_str());
    let mut store = timeline_store_load().map_err(|_| "timeline_unavailable")?;
    let rec = store
        .file_transfers
        .get_mut(&key)
        .ok_or("manifest_missing_chunks")?;
    if rec.state == "FAILED" || rec.state == "VERIFIED" {
        return Err("state_invalid_transition");
    }
    if rec.total_size != manifest.total_size
        || rec.chunk_count != manifest.chunk_count
        || rec.filename != manifest.filename
    {
        return Err("manifest_meta_mismatch");
    }
    if rec.chunks_hex.len() != rec.chunk_count {
        return Err("manifest_missing_chunks");
    }
    if manifest.chunk_hashes.len() != rec.chunk_count {
        return Err("manifest_chunk_count_mismatch");
    }
    let expected_manifest = file_xfer_manifest_hash(
        manifest.file_id.as_str(),
        manifest.total_size,
        manifest.chunk_count,
        manifest.chunk_hashes.as_slice(),
    );
    if expected_manifest != manifest.manifest_hash || rec.manifest_hash != manifest.manifest_hash {
        return Err("manifest_mismatch");
    }
    if rec.chunk_hashes != manifest.chunk_hashes {
        return Err("manifest_mismatch");
    }
    let mut reconstructed = Vec::new();
    for (idx, chunk_hex) in rec.chunks_hex.iter().enumerate() {
        let chunk = hex_decode(chunk_hex).map_err(|_| "chunk_decode_failed")?;
        if file_xfer_chunk_hash(&chunk) != manifest.chunk_hashes[idx] {
            return Err("chunk_hash_invalid");
        }
        reconstructed.extend_from_slice(&chunk);
    }
    if reconstructed.len() != manifest.total_size {
        return Err("manifest_size_mismatch");
    }
    rec.state = "VERIFIED".to_string();
    rec.confirm_requested = manifest.confirm_requested;
    rec.confirm_id = if manifest.confirm_requested {
        Some(manifest.confirm_id.clone())
    } else {
        None
    };
    timeline_store_save(&store).map_err(|_| "timeline_unavailable")?;
    timeline_append_entry(
        ctx.from,
        "in",
        reconstructed.len(),
        "file",
        MessageState::Received,
        Some(manifest.file_id.as_str()),
    )?;
    emit_marker(
        "file_xfer_manifest",
        None,
        &[("id", manifest.file_id.as_str()), ("ok", "true")],
    );
    emit_marker(
        "file_xfer_complete",
        None,
        &[("id", manifest.file_id.as_str()), ("ok", "true")],
    );
    if manifest.confirm_requested {
        if ctx.receipt_policy.file_confirm_mode == FileConfirmEmitMode::CompleteOnly {
            return Ok(Some((manifest.file_id, manifest.confirm_id)));
        }
        emit_cli_receipt_policy_event(
            ctx.receipt_policy.mode,
            "skipped",
            "file_complete",
            ctx.from,
        );
        emit_tui_receipt_policy_event(
            ctx.receipt_policy.mode,
            "skipped",
            "file_complete",
            ctx.from,
        );
    }
    Ok(None)
}

fn build_delivered_ack(msg_id: &str) -> Vec<u8> {
    let ack = ReceiptControlPayload {
        v: 1,
        t: "ack".to_string(),
        kind: "delivered".to_string(),
        msg_id: msg_id.to_string(),
        body: None,
    };
    serde_json::to_vec(&ack).unwrap_or_else(|_| print_error_marker("receipt_encode_failed"))
}

fn build_file_completion_ack(file_id: &str, confirm_id: &str) -> Vec<u8> {
    let ack = FileConfirmPayload {
        v: 1,
        t: "ack".to_string(),
        kind: "file_confirmed".to_string(),
        file_id: file_id.to_string(),
        confirm_id: confirm_id.to_string(),
    };
    serde_json::to_vec(&ack).unwrap_or_else(|_| print_error_marker("receipt_encode_failed"))
}

fn build_attachment_completion_ack(attachment_id: &str, confirm_handle: &str) -> Vec<u8> {
    let ack = AttachmentConfirmPayload {
        v: 1,
        t: "ack".to_string(),
        kind: ATTACHMENT_CONFIRM_KIND.to_string(),
        attachment_id: attachment_id.to_string(),
        confirm_handle: confirm_handle.to_string(),
    };
    serde_json::to_vec(&ack).unwrap_or_else(|_| print_error_marker("receipt_encode_failed"))
}

#[derive(Clone, Debug)]
enum PendingReceipt {
    Message {
        msg_id: String,
    },
    FileComplete {
        file_id: String,
        confirm_id: String,
    },
    AttachmentComplete {
        attachment_id: String,
        confirm_handle: String,
    },
}

fn queue_or_send_receipt(
    ctx: &ReceivePullCtx<'_>,
    queue: &mut Vec<PendingReceipt>,
    item: PendingReceipt,
) {
    let kind = match item {
        PendingReceipt::Message { .. } => "message",
        PendingReceipt::FileComplete { .. } => "file_complete",
        PendingReceipt::AttachmentComplete { .. } => "attachment_complete",
    };
    match ctx.receipt_policy.mode {
        ReceiptEmitMode::Off => {
            emit_cli_receipt_policy_event(ctx.receipt_policy.mode, "skipped", kind, ctx.from);
            emit_tui_receipt_policy_event(ctx.receipt_policy.mode, "skipped", kind, ctx.from);
            emit_marker(
                "receipt_disabled",
                None,
                &[("mode", ctx.receipt_policy.mode.as_str()), ("kind", kind)],
            );
        }
        ReceiptEmitMode::Immediate => {
            send_pending_receipt(ctx, item);
        }
        ReceiptEmitMode::Batched => {
            queue.push(item);
            emit_cli_receipt_policy_event(ctx.receipt_policy.mode, "queued", kind, ctx.from);
            emit_tui_receipt_policy_event(ctx.receipt_policy.mode, "queued", kind, ctx.from);
        }
    }
}

fn send_pending_receipt(ctx: &ReceivePullCtx<'_>, item: PendingReceipt) {
    match item {
        PendingReceipt::Message { msg_id } => {
            match send_delivered_receipt_ack(ctx.relay, ctx.from, &msg_id) {
                Ok(()) => {
                    emit_marker(
                        "receipt_send",
                        None,
                        &[
                            ("kind", "delivered"),
                            ("bucket", "small"),
                            ("msg_id", "<redacted>"),
                        ],
                    );
                    emit_cli_receipt_policy_event(
                        ctx.receipt_policy.mode,
                        "sent",
                        "message",
                        ctx.from,
                    );
                    emit_tui_receipt_policy_event(
                        ctx.receipt_policy.mode,
                        "sent",
                        "message",
                        ctx.from,
                    );
                }
                Err(code) => emit_marker("receipt_send_failed", Some(code), &[("code", code)]),
            }
        }
        PendingReceipt::FileComplete {
            file_id,
            confirm_id,
        } => {
            match send_file_completion_ack(
                ctx.relay,
                ctx.from,
                file_id.as_str(),
                confirm_id.as_str(),
            ) {
                Ok(()) => {
                    let safe_file_id = file_delivery_short_id(file_id.as_str());
                    emit_marker(
                        "file_confirm_send",
                        None,
                        &[
                            ("kind", "coarse_complete"),
                            ("file_id", safe_file_id.as_str()),
                            ("ok", "true"),
                        ],
                    );
                    emit_cli_receipt_policy_event(
                        ctx.receipt_policy.mode,
                        "sent",
                        "file_complete",
                        ctx.from,
                    );
                    emit_tui_receipt_policy_event(
                        ctx.receipt_policy.mode,
                        "sent",
                        "file_complete",
                        ctx.from,
                    );
                }
                Err(code) => emit_marker("file_confirm_send_failed", Some(code), &[("code", code)]),
            }
        }
        PendingReceipt::AttachmentComplete {
            attachment_id,
            confirm_handle,
        } => {
            let payload = build_attachment_completion_ack(&attachment_id, &confirm_handle);
            let outcome = relay_send_with_payload(RelaySendPayloadArgs {
                to: ctx.from,
                payload,
                relay: ctx.relay,
                injector: fault_injector_from_env(),
                pad_cfg: None,
                bucket_max: None,
                meta_seed: None,
                receipt: None,
                routing_override: None,
                tui_thread: None,
            });
            if let Some(code) = outcome.error_code {
                emit_marker(
                    "attachment_confirm_send_failed",
                    Some(code),
                    &[("code", code)],
                );
                return;
            }
            let safe_attachment = file_delivery_short_id(&attachment_id);
            emit_marker(
                "attachment_confirm_send",
                None,
                &[
                    ("kind", "complete"),
                    ("attachment_id", safe_attachment.as_str()),
                    ("ok", "true"),
                ],
            );
        }
    }
}

fn flush_batched_receipts(ctx: &ReceivePullCtx<'_>, queue: &mut Vec<PendingReceipt>) {
    if ctx.receipt_policy.mode != ReceiptEmitMode::Batched || queue.is_empty() {
        return;
    }
    // Deterministic ordering; jitter only affects stable sort priority.
    queue.sort_by_key(|item| match item {
        PendingReceipt::Message { msg_id } => {
            let bias = if ctx.receipt_policy.jitter_ms == 0 {
                0
            } else {
                let mut acc: u64 = 0;
                for b in msg_id.as_bytes() {
                    acc = acc.wrapping_add(*b as u64);
                }
                acc % (ctx.receipt_policy.jitter_ms + 1)
            };
            (0u8, bias, msg_id.clone())
        }
        PendingReceipt::FileComplete { file_id, .. } => {
            let bias = if ctx.receipt_policy.jitter_ms == 0 {
                0
            } else {
                let mut acc: u64 = 0;
                for b in file_id.as_bytes() {
                    acc = acc.wrapping_add(*b as u64);
                }
                acc % (ctx.receipt_policy.jitter_ms + 1)
            };
            (1u8, bias, file_id.clone())
        }
        PendingReceipt::AttachmentComplete { attachment_id, .. } => {
            let bias = if ctx.receipt_policy.jitter_ms == 0 {
                0
            } else {
                let mut acc: u64 = 0;
                for b in attachment_id.as_bytes() {
                    acc = acc.wrapping_add(*b as u64);
                }
                acc % (ctx.receipt_policy.jitter_ms + 1)
            };
            (2u8, bias, attachment_id.clone())
        }
    });
    let pending = std::mem::take(queue);
    for item in pending {
        send_pending_receipt(ctx, item);
    }
}

fn send_delivered_receipt_ack(relay: &str, to: &str, msg_id: &str) -> Result<(), &'static str> {
    let payload = build_delivered_ack(msg_id);
    let pad_cfg = Some(MetaPadConfig {
        target_len: None,
        profile: Some(EnvelopeProfile::Standard),
        label: Some("small"),
    });
    let pack = qsp_pack(to, &payload, pad_cfg, None).map_err(|e| e.code)?;
    let route_token = relay_peer_route_token(to)?;
    relay_inbox_push(relay, route_token.as_str(), &pack.envelope)?;
    qsp_session_store(to, &pack.next_state).map_err(|_| "qsp_session_store_failed")?;
    Ok(())
}

fn send_file_completion_ack(
    relay: &str,
    to: &str,
    file_id: &str,
    confirm_id: &str,
) -> Result<(), &'static str> {
    let payload = build_file_completion_ack(file_id, confirm_id);
    let pad_cfg = Some(MetaPadConfig {
        target_len: None,
        profile: Some(EnvelopeProfile::Standard),
        label: Some("small"),
    });
    let pack = qsp_pack(to, &payload, pad_cfg, None).map_err(|e| e.code)?;
    let route_token = relay_peer_route_token(to)?;
    relay_inbox_push(relay, route_token.as_str(), &pack.envelope)?;
    qsp_session_store(to, &pack.next_state).map_err(|_| "qsp_session_store_failed")?;
    Ok(())
}

fn meta_pad_config_from_args(
    pad_to: Option<usize>,
    pad_bucket: Option<MetaPadBucket>,
    meta_seed: Option<u64>,
) -> Result<Option<MetaPadConfig>, &'static str> {
    if pad_to.is_none() && pad_bucket.is_none() {
        return Ok(None);
    }
    if pad_to.is_some() && pad_bucket.is_some() {
        return Err("meta_pad_conflict");
    }
    if let Some(len) = pad_to {
        if len == 0 || len > PAD_TO_MAX {
            return Err("meta_pad_invalid");
        }
        return Ok(Some(MetaPadConfig {
            target_len: Some(len),
            profile: None,
            label: Some("pad_to"),
        }));
    }
    let bucket = pad_bucket.unwrap_or(MetaPadBucket::Standard);
    let profile = match bucket {
        MetaPadBucket::Standard => EnvelopeProfile::Standard,
        MetaPadBucket::Enhanced => EnvelopeProfile::Enhanced,
        MetaPadBucket::Private => EnvelopeProfile::Private,
        MetaPadBucket::Auto => {
            let seed = meta_seed.ok_or("meta_seed_required")?;
            let mut rng = RelayRng::new(seed ^ 0x51d2a9f1);
            match rng.next_u32() % 3 {
                0 => EnvelopeProfile::Standard,
                1 => EnvelopeProfile::Enhanced,
                _ => EnvelopeProfile::Private,
            }
        }
    };
    let label = match bucket {
        MetaPadBucket::Standard => "standard",
        MetaPadBucket::Enhanced => "enhanced",
        MetaPadBucket::Private => "private",
        MetaPadBucket::Auto => "auto",
    };
    Ok(Some(MetaPadConfig {
        target_len: None,
        profile: Some(profile),
        label: Some(label),
    }))
}

fn map_qsp_recv_err(err: &RefimplError) -> &'static str {
    let s = err.to_string();
    if s.contains("REJECT_S2_REPLAY") {
        "qsp_replay_reject"
    } else if s.contains("REJECT_S2_OOO_BOUNDS") {
        "qsp_ooo_reject"
    } else if s.contains("REJECT_S2_BODY_AUTH_FAIL") {
        "qsp_auth_failed"
    } else if s.contains("REJECT_S2_HDR_AUTH_FAIL") {
        "qsp_hdr_auth_failed"
    } else {
        "qsp_verify_failed"
    }
}

fn map_qsp_pack_reason(err: &RefimplError) -> &'static str {
    let s = err.to_string();
    if s.contains("REJECT_S2_CHAINKEY_UNSET") {
        "chainkey_unset"
    } else if s.contains("REJECT_S2_LOCAL_UNSUPPORTED") {
        "local_unsupported"
    } else if s.contains("REJECT_S2_LOCAL_AEAD_FAIL") {
        "local_aead_fail"
    } else {
        "pack_internal"
    }
}

fn qsp_activate_responder_send_chain_if_needed(st: &mut Suite2SessionState) {
    if st.recv.role_is_a {
        return;
    }
    if !(zero32(&st.send.ck_ec) || zero32(&st.send.ck_pq)) {
        return;
    }
    if zero32(&st.recv.rk) || zero32(&st.send.hk_s) {
        return;
    }
    let c = StdCrypto;
    st.send.ck_ec = kmac_out::<32>(&c, &st.recv.rk, "QSP5.0/CK0/B->A", &[0x01]);
    st.send.ck_pq = kmac_out::<32>(&c, &st.recv.rk, "QSP5.0/PQ0/B->A", &[0x01]);
    emit_marker(
        "qsp_send_chain",
        None,
        &[
            ("activated", "true"),
            ("reason", "responder_recv_bootstrap"),
        ],
    );
}

fn qsp_activate_initiator_recv_chain_if_needed(st: &mut Suite2SessionState) {
    if !st.recv.role_is_a {
        return;
    }
    if !(zero32(&st.recv.ck_ec) || zero32(&st.recv.ck_pq_recv)) {
        return;
    }
    if zero32(&st.recv.rk) || zero32(&st.recv.hk_r) {
        return;
    }
    let c = StdCrypto;
    st.recv.ck_ec = kmac_out::<32>(&c, &st.recv.rk, "QSP5.0/CK0/B->A", &[0x01]);
    st.recv.ck_pq_recv = kmac_out::<32>(&c, &st.recv.rk, "QSP5.0/PQ0/B->A", &[0x01]);
    emit_marker(
        "qsp_recv_chain",
        None,
        &[
            ("activated", "true"),
            ("reason", "initiator_send_bootstrap"),
        ],
    );
}

fn qsp_pack(
    channel: &str,
    plaintext: &[u8],
    pad_cfg: Option<MetaPadConfig>,
    meta_seed: Option<u64>,
) -> Result<QspPackOutcome, QspPackError> {
    let st =
        qsp_session_for_channel(channel).map_err(|code| QspPackError { code, reason: None })?;
    let c = StdCrypto;
    let outcome =
        send_wire_canon(&c, &c, &c, st.send.clone(), 0, plaintext).map_err(|e| QspPackError {
            code: "qsp_pack_failed",
            reason: Some(map_qsp_pack_reason(&e)),
        })?;
    let mut env = Envelope {
        env_version: QSE_ENV_VERSION_V1,
        flags: 0,
        route_token: Vec::new(),
        timestamp_bucket: 0,
        payload: outcome.wire,
        padding: Vec::new(),
    };
    let mut pad_label = None;
    let mut encoded_len = env.encode().len();
    let min_len = EnvelopeProfile::Standard.min_size_bytes();
    if encoded_len < min_len {
        let need = min_len - encoded_len;
        let mut seed_bytes = Vec::new();
        if let Some(seed) = meta_seed {
            seed_bytes.extend_from_slice(&seed.to_le_bytes());
        }
        let pad = c.kmac256(&env.payload, "QSC.QSP.PAD", &seed_bytes, need);
        env = env
            .pad_to_profile(EnvelopeProfile::Standard, &pad)
            .map_err(|_| QspPackError {
                code: "qsp_pack_failed",
                reason: Some("QSP_PACK_INTERNAL"),
            })?;
        encoded_len = env.encode().len();
    }
    if let Some(cfg) = pad_cfg {
        if let Some(target) = cfg.target_len {
            if target < encoded_len {
                return Err(QspPackError {
                    code: "meta_pad_too_small",
                    reason: None,
                });
            }
            let need = target - encoded_len;
            if need > 0 {
                let mut seed_bytes = Vec::new();
                if let Some(seed) = meta_seed {
                    seed_bytes.extend_from_slice(&seed.to_le_bytes());
                }
                let pad = c.kmac256(&env.payload, "QSC.META.PAD", &seed_bytes, need);
                env.padding.extend_from_slice(&pad);
                encoded_len = env.encode().len();
            }
            pad_label = cfg.label;
        } else if let Some(profile) = cfg.profile {
            let min_len = profile.min_size_bytes();
            if encoded_len < min_len {
                let need = min_len - encoded_len;
                let mut seed_bytes = Vec::new();
                if let Some(seed) = meta_seed {
                    seed_bytes.extend_from_slice(&seed.to_le_bytes());
                }
                let pad = c.kmac256(&env.payload, "QSC.META.PAD", &seed_bytes, need);
                env = env
                    .pad_to_profile(profile, &pad)
                    .map_err(|_| QspPackError {
                        code: "qsp_pack_failed",
                        reason: Some("QSP_PACK_INTERNAL"),
                    })?;
                encoded_len = env.encode().len();
            }
            pad_label = cfg.label;
        }
    }
    let mut next_state = st.clone();
    next_state.send = outcome.state;
    qsp_activate_initiator_recv_chain_if_needed(&mut next_state);
    Ok(QspPackOutcome {
        envelope: env.encode(),
        next_state,
        msg_idx: outcome.n,
        ck_idx: outcome.n,
        padded_len: encoded_len,
        pad_label,
    })
}

fn qsp_unpack_channels_for_peer(peer: &str) -> Vec<String> {
    let mut channels = Vec::new();
    channels.push(peer.to_string());
    let peer_alias = peer_alias_from_channel(peer);
    if peer_alias != peer {
        channels.push(peer_alias.to_string());
    }
    if let Ok(Some(mut rec)) = contacts_entry_read(peer_alias) {
        normalize_contact_record(peer_alias, &mut rec);
        for dev in rec.devices.iter() {
            if let Some(channel) = channel_label_for_device(peer_alias, dev.device_id.as_str()) {
                if !channels.iter().any(|v| v == &channel) {
                    channels.push(channel);
                }
            }
        }
    }
    channels
}

fn qsp_unpack_for_peer(
    peer: &str,
    envelope_bytes: &[u8],
) -> Result<(QspUnpackOutcome, String), &'static str> {
    let mut first_err: Option<&'static str> = None;
    for channel in qsp_unpack_channels_for_peer(peer).into_iter() {
        match qsp_unpack(channel.as_str(), envelope_bytes) {
            Ok(outcome) => return Ok((outcome, channel)),
            Err(code) => {
                if first_err.is_none() {
                    first_err = Some(code);
                }
            }
        }
    }
    Err(first_err.unwrap_or("qsp_channel_invalid"))
}

fn qsp_unpack(channel: &str, envelope_bytes: &[u8]) -> Result<QspUnpackOutcome, &'static str> {
    let env = Envelope::decode(envelope_bytes).map_err(|_| "qsp_env_decode_failed")?;
    let st = qsp_session_for_channel(channel)?;
    let c = StdCrypto;
    let outcome = recv_wire_canon(&c, &c, &c, st.recv.clone(), &env.payload, None, None)
        .map_err(|e| map_qsp_recv_err(&e))?;
    let mut next_state = st.clone();
    let prev_len = next_state.recv.mkskipped.len();
    next_state.recv = outcome.state;
    qsp_activate_responder_send_chain_if_needed(&mut next_state);
    let skip_delta = next_state.recv.mkskipped.len().saturating_sub(prev_len);
    let evicted = bound_mkskipped(&mut next_state.recv);
    Ok(QspUnpackOutcome {
        plaintext: outcome.plaintext,
        next_state,
        msg_idx: outcome.n,
        skip_delta,
        evicted,
    })
}

const HS_MAGIC: &[u8; 4] = b"QHSM";
const HS_VERSION: u16 = 1;
const HS_TYPE_INIT: u8 = 1;
const HS_TYPE_RESP: u8 = 2;
const HS_TYPE_CONFIRM: u8 = 3;

fn hs_kem_pk_len() -> usize {
    runtime_pq_kem_public_key_bytes()
}

fn hs_kem_ct_len() -> usize {
    runtime_pq_kem_ciphertext_bytes()
}

fn hs_kem_keypair() -> (Vec<u8>, Vec<u8>) {
    runtime_pq_kem_keypair()
}

fn hs_sig_pk_len() -> usize {
    runtime_pq_sig_public_key_bytes()
}

fn hs_sig_sig_len() -> usize {
    runtime_pq_sig_signature_bytes()
}

fn hs_sig_keypair() -> (Vec<u8>, Vec<u8>) {
    runtime_pq_sig_keypair()
}

fn hs_default_role() -> String {
    "initiator".to_string()
}

#[derive(Clone, Debug)]
struct HsInit {
    session_id: [u8; 16],
    kem_pk: Vec<u8>,
    sig_pk: Vec<u8>,
    dh_pub: [u8; 32],
}

#[derive(Clone, Debug)]
struct HsResp {
    session_id: [u8; 16],
    kem_ct: Vec<u8>,
    mac: [u8; 32],
    sig_pk: Vec<u8>,
    sig: Vec<u8>,
    dh_pub: [u8; 32],
}

#[derive(Clone, Debug)]
struct HsConfirm {
    session_id: [u8; 16],
    mac: [u8; 32],
    sig: Vec<u8>,
}

#[derive(Serialize, Deserialize, Clone)]
struct HandshakePending {
    self_label: String,
    peer: String,
    session_id: [u8; 16],
    kem_sk: Vec<u8>,
    kem_pk: Vec<u8>,
    #[serde(default)]
    dh_sk: Vec<u8>,
    #[serde(default)]
    dh_pub: Vec<u8>,
    #[serde(default)]
    sig_pk: Vec<u8>,
    #[serde(default)]
    peer_fp: Option<String>,
    #[serde(default)]
    peer_sig_fp: Option<String>,
    #[serde(default)]
    peer_sig_pk: Option<Vec<u8>>,
    #[serde(default = "hs_default_role")]
    role: String,
    #[serde(default)]
    confirm_key: Option<[u8; 32]>,
    #[serde(default)]
    transcript_hash: Option<[u8; 32]>,
    #[serde(default)]
    pending_session: Option<Vec<u8>>,
}

fn hs_encode_init(msg: &HsInit) -> Vec<u8> {
    let pk_len = hs_kem_pk_len();
    let sig_pk_len = hs_sig_pk_len();
    if msg.kem_pk.len() != pk_len || msg.sig_pk.len() != sig_pk_len {
        return Vec::new();
    }
    let mut out = Vec::with_capacity(4 + 2 + 1 + 16 + pk_len + sig_pk_len + 32);
    out.extend_from_slice(HS_MAGIC);
    out.extend_from_slice(&HS_VERSION.to_be_bytes());
    out.push(HS_TYPE_INIT);
    out.extend_from_slice(&msg.session_id);
    out.extend_from_slice(&msg.kem_pk);
    out.extend_from_slice(&msg.sig_pk);
    out.extend_from_slice(&msg.dh_pub);
    out
}

fn hs_decode_init(bytes: &[u8]) -> Result<HsInit, &'static str> {
    let pk_len = hs_kem_pk_len();
    let sig_pk_len = hs_sig_pk_len();
    if bytes.len() != 4 + 2 + 1 + 16 + pk_len + sig_pk_len + 32 {
        return Err("handshake_init_len");
    }
    if &bytes[0..4] != HS_MAGIC {
        return Err("handshake_magic");
    }
    let ver = u16::from_be_bytes([bytes[4], bytes[5]]);
    if ver != HS_VERSION {
        return Err("handshake_version");
    }
    if bytes[6] != HS_TYPE_INIT {
        return Err("handshake_type");
    }
    let mut sid = [0u8; 16];
    sid.copy_from_slice(&bytes[7..23]);
    let kem_pk = bytes[23..(23 + pk_len)].to_vec();
    let sig_pk = bytes[(23 + pk_len)..(23 + pk_len + sig_pk_len)].to_vec();
    let mut dh_pub = [0u8; 32];
    dh_pub.copy_from_slice(&bytes[(23 + pk_len + sig_pk_len)..(23 + pk_len + sig_pk_len + 32)]);
    Ok(HsInit {
        session_id: sid,
        kem_pk,
        sig_pk,
        dh_pub,
    })
}

fn hs_encode_resp(msg: &HsResp) -> Vec<u8> {
    let ct_len = hs_kem_ct_len();
    let sig_pk_len = hs_sig_pk_len();
    let sig_len = hs_sig_sig_len();
    if msg.kem_ct.len() != ct_len || msg.sig_pk.len() != sig_pk_len || msg.sig.len() != sig_len {
        return Vec::new();
    }
    let mut out = Vec::with_capacity(4 + 2 + 1 + 16 + ct_len + 32 + sig_pk_len + sig_len + 32);
    out.extend_from_slice(HS_MAGIC);
    out.extend_from_slice(&HS_VERSION.to_be_bytes());
    out.push(HS_TYPE_RESP);
    out.extend_from_slice(&msg.session_id);
    out.extend_from_slice(&msg.kem_ct);
    out.extend_from_slice(&msg.mac);
    out.extend_from_slice(&msg.sig_pk);
    out.extend_from_slice(&msg.sig);
    out.extend_from_slice(&msg.dh_pub);
    out
}

fn hs_decode_resp(bytes: &[u8]) -> Result<HsResp, &'static str> {
    let ct_len = hs_kem_ct_len();
    let sig_pk_len = hs_sig_pk_len();
    let sig_len = hs_sig_sig_len();
    if bytes.len() != 4 + 2 + 1 + 16 + ct_len + 32 + sig_pk_len + sig_len + 32 {
        return Err("handshake_resp_len");
    }
    if &bytes[0..4] != HS_MAGIC {
        return Err("handshake_magic");
    }
    let ver = u16::from_be_bytes([bytes[4], bytes[5]]);
    if ver != HS_VERSION {
        return Err("handshake_version");
    }
    if bytes[6] != HS_TYPE_RESP {
        return Err("handshake_type");
    }
    let mut sid = [0u8; 16];
    sid.copy_from_slice(&bytes[7..23]);
    let kem_ct = bytes[23..(23 + ct_len)].to_vec();
    let mut mac = [0u8; 32];
    let mac_off = 23 + ct_len;
    mac.copy_from_slice(&bytes[mac_off..(mac_off + 32)]);
    let sig_pk_off = mac_off + 32;
    let sig_off = sig_pk_off + sig_pk_len;
    let sig_pk = bytes[sig_pk_off..sig_off].to_vec();
    let sig = bytes[sig_off..(sig_off + sig_len)].to_vec();
    let mut dh_pub = [0u8; 32];
    dh_pub.copy_from_slice(&bytes[(sig_off + sig_len)..(sig_off + sig_len + 32)]);
    Ok(HsResp {
        session_id: sid,
        kem_ct,
        mac,
        sig_pk,
        sig,
        dh_pub,
    })
}

fn hs_encode_confirm(msg: &HsConfirm) -> Vec<u8> {
    let sig_len = hs_sig_sig_len();
    if msg.sig.len() != sig_len {
        return Vec::new();
    }
    let mut out = Vec::with_capacity(4 + 2 + 1 + 16 + 32 + sig_len);
    out.extend_from_slice(HS_MAGIC);
    out.extend_from_slice(&HS_VERSION.to_be_bytes());
    out.push(HS_TYPE_CONFIRM);
    out.extend_from_slice(&msg.session_id);
    out.extend_from_slice(&msg.mac);
    out.extend_from_slice(&msg.sig);
    out
}

fn hs_decode_confirm(bytes: &[u8]) -> Result<HsConfirm, &'static str> {
    let sig_len = hs_sig_sig_len();
    if bytes.len() != 4 + 2 + 1 + 16 + 32 + sig_len {
        return Err("handshake_confirm_len");
    }
    if &bytes[0..4] != HS_MAGIC {
        return Err("handshake_magic");
    }
    let ver = u16::from_be_bytes([bytes[4], bytes[5]]);
    if ver != HS_VERSION {
        return Err("handshake_version");
    }
    if bytes[6] != HS_TYPE_CONFIRM {
        return Err("handshake_type");
    }
    let mut sid = [0u8; 16];
    sid.copy_from_slice(&bytes[7..23]);
    let mut mac = [0u8; 32];
    mac.copy_from_slice(&bytes[23..55]);
    let sig = bytes[55..(55 + sig_len)].to_vec();
    Ok(HsConfirm {
        session_id: sid,
        mac,
        sig,
    })
}

fn hex_encode(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut out = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        out.push(HEX[(b >> 4) as usize] as char);
        out.push(HEX[(b & 0x0f) as usize] as char);
    }
    out
}

fn hex_decode(s: &str) -> Result<Vec<u8>, ErrorCode> {
    if !s.len().is_multiple_of(2) {
        return Err(ErrorCode::ParseFailed);
    }
    let mut out = Vec::with_capacity(s.len() / 2);
    let bytes = s.as_bytes();
    let mut i = 0usize;
    while i < bytes.len() {
        let hi = hex_nibble(bytes[i]).ok_or(ErrorCode::ParseFailed)?;
        let lo = hex_nibble(bytes[i + 1]).ok_or(ErrorCode::ParseFailed)?;
        out.push((hi << 4) | lo);
        i += 2;
    }
    Ok(out)
}

fn hex_nibble(c: u8) -> Option<u8> {
    match c {
        b'0'..=b'9' => Some(c - b'0'),
        b'a'..=b'f' => Some(c - b'a' + 10),
        b'A'..=b'F' => Some(c - b'A' + 10),
        _ => None,
    }
}

fn legacy_contact_status_to_device_state(status: &str) -> &'static str {
    let upper = status.trim().to_ascii_uppercase();
    match upper.as_str() {
        "PINNED" => "TRUSTED",
        "VERIFIED" | "COMPLETE" => "VERIFIED",
        "MISMATCH" | "CHANGED" => "CHANGED",
        "REVOKED" => "REVOKED",
        _ => "UNVERIFIED",
    }
}

fn canonical_device_state(state: &str) -> &'static str {
    let upper = state.trim().to_ascii_uppercase();
    match upper.as_str() {
        "TRUSTED" => "TRUSTED",
        "VERIFIED" => "VERIFIED",
        "CHANGED" | "MISMATCH" => "CHANGED",
        "REVOKED" => "REVOKED",
        _ => "UNVERIFIED",
    }
}

fn device_state_to_legacy_status(state: &str) -> &'static str {
    match canonical_device_state(state) {
        "TRUSTED" => "PINNED",
        "VERIFIED" => "VERIFIED",
        "CHANGED" => "CHANGED",
        "REVOKED" => "CHANGED",
        _ => "UNVERIFIED",
    }
}

fn device_id_short(alias: &str, sig_fp: Option<&str>, fp: &str) -> String {
    let basis = sig_fp
        .filter(|v| !v.trim().is_empty())
        .unwrap_or(fp)
        .trim()
        .to_string();
    let basis = if basis.is_empty() {
        alias.trim().to_string()
    } else {
        basis
    };
    let c = StdCrypto;
    let hash = c.sha512(basis.as_bytes());
    hex_encode(&hash[..6])
}

fn normalize_contact_record(alias: &str, rec: &mut ContactRecord) -> bool {
    let mut mutated = false;
    if rec.devices.is_empty() {
        rec.devices.push(ContactDeviceRecord {
            device_id: device_id_short(alias, rec.sig_fp.as_deref(), rec.fp.as_str()),
            fp: rec.fp.clone(),
            sig_fp: rec.sig_fp.clone(),
            state: legacy_contact_status_to_device_state(rec.status.as_str()).to_string(),
            route_token: rec.route_token.clone(),
            seen_at: rec.seen_at,
            label: None,
        });
        mutated = true;
    }
    for dev in rec.devices.iter_mut() {
        let canonical = canonical_device_state(dev.state.as_str());
        if dev.state != canonical {
            dev.state = canonical.to_string();
            mutated = true;
        }
        if dev.device_id.trim().is_empty() || dev.device_id.len() > 12 {
            dev.device_id = device_id_short(alias, dev.sig_fp.as_deref(), dev.fp.as_str());
            mutated = true;
        }
        if dev.fp.trim().is_empty() {
            dev.fp = "UNSET".to_string();
            mutated = true;
        }
    }
    let mut normalized = rec.devices.clone();
    normalized.sort_by(|a, b| a.device_id.cmp(&b.device_id));
    if normalized.len() != rec.devices.len()
        || normalized
            .iter()
            .zip(rec.devices.iter())
            .any(|(a, b)| a.device_id != b.device_id)
    {
        rec.devices = normalized;
        mutated = true;
    }
    if let Some(primary) = rec.devices.first_mut() {
        if primary.route_token.is_none() && rec.route_token.is_some() {
            primary.route_token = rec.route_token.clone();
            mutated = true;
        }
    }
    let canonical_primary = rec
        .primary_device_id
        .as_ref()
        .and_then(|id| {
            rec.devices
                .iter()
                .find(|d| d.device_id == *id)
                .map(|d| d.device_id.clone())
        })
        .or_else(|| {
            rec.devices
                .iter()
                .find(|d| canonical_device_state(d.state.as_str()) == "TRUSTED")
                .map(|d| d.device_id.clone())
        })
        .or_else(|| rec.devices.first().map(|d| d.device_id.clone()));
    if rec.primary_device_id != canonical_primary {
        rec.primary_device_id = canonical_primary;
        mutated = true;
    }
    if let Some(primary) = rec
        .primary_device_id
        .as_ref()
        .and_then(|id| rec.devices.iter().find(|d| d.device_id == *id))
    {
        let legacy_status = device_state_to_legacy_status(primary.state.as_str()).to_string();
        if rec.status.to_ascii_uppercase() != legacy_status {
            rec.status = legacy_status;
            mutated = true;
        }
        if rec.fp != primary.fp {
            rec.fp = primary.fp.clone();
            mutated = true;
        }
        if rec.sig_fp != primary.sig_fp {
            rec.sig_fp = primary.sig_fp.clone();
            mutated = true;
        }
        if rec.route_token != primary.route_token {
            rec.route_token = primary.route_token.clone();
            mutated = true;
        }
        if rec.seen_at != primary.seen_at {
            rec.seen_at = primary.seen_at;
            mutated = true;
        }
    }
    mutated
}

fn primary_device(rec: &ContactRecord) -> Option<&ContactDeviceRecord> {
    if let Some(primary_id) = rec.primary_device_id.as_ref() {
        if let Some(dev) = rec.devices.iter().find(|d| d.device_id == *primary_id) {
            return Some(dev);
        }
    }
    rec.devices.first()
}

fn primary_device_mut(rec: &mut ContactRecord) -> Option<&mut ContactDeviceRecord> {
    if let Some(primary_id) = rec.primary_device_id.as_ref() {
        if let Some(idx) = rec.devices.iter().position(|d| d.device_id == *primary_id) {
            return rec.devices.get_mut(idx);
        }
    }
    rec.devices.first_mut()
}

fn peer_alias_from_channel(peer: &str) -> &str {
    peer.split_once('#').map(|(alias, _)| alias).unwrap_or(peer)
}

fn channel_device_id(channel: &str) -> Option<&str> {
    channel
        .split_once('#')
        .map(|(_, device)| device)
        .filter(|v| !v.is_empty())
}

fn channel_device_marker(channel: &str) -> String {
    channel_device_id(channel)
        .map(short_device_marker)
        .unwrap_or_else(|| "unknown".to_string())
}

fn confirm_target_matches_channel(target_device_id: Option<&str>, channel: &str) -> bool {
    match target_device_id {
        None => true,
        Some(expected) => match channel_device_id(channel) {
            Some(actual) => short_device_marker(actual) == short_device_marker(expected),
            // Legacy receive flows may not carry a device-qualified channel label.
            // We keep these confirmations compatible while enforcing strict matching
            // when a device-qualified channel is present.
            None => true,
        },
    }
}

fn channel_label_for_device(peer_alias: &str, device_id: &str) -> Option<String> {
    if !channel_label_ok(peer_alias) || !channel_label_ok(device_id) {
        return None;
    }
    let label = format!("{peer_alias}#{device_id}");
    if channel_label_ok(label.as_str()) {
        Some(label)
    } else {
        None
    }
}

#[derive(Clone, Debug)]
struct SendRoutingTarget {
    peer_alias: String,
    channel: String,
    device_id: String,
    route_token: String,
    implicit_primary: bool,
}

fn resolve_peer_device_target(
    peer: &str,
    require_trusted: bool,
) -> Result<SendRoutingTarget, &'static str> {
    let peer_alias = peer_alias_from_channel(peer);
    if !channel_label_ok(peer_alias) {
        return Err("unknown_contact");
    }
    let mut rec = contacts_entry_read(peer_alias).map_err(|_| "contacts_store_invalid")?;
    let Some(mut rec) = rec.take() else {
        return Err("unknown_contact");
    };
    let implicit_primary = rec.primary_device_id.is_none();
    let mut mutated = normalize_contact_record(peer_alias, &mut rec);
    let Some(primary) = primary_device(&rec).cloned() else {
        return Err("no_trusted_device");
    };
    let canonical_state = canonical_device_state(primary.state.as_str());
    match canonical_state {
        "CHANGED" => return Err("device_changed_reapproval_required"),
        "REVOKED" => return Err("device_revoked"),
        "TRUSTED" => {}
        _ if require_trusted => return Err("no_trusted_device"),
        _ => {}
    }
    let route_token = primary
        .route_token
        .clone()
        .or_else(|| rec.route_token.clone())
        .ok_or("contact_route_token_missing")?;
    let route_token =
        normalize_route_token(route_token.as_str()).map_err(|_| "contact_route_token_missing")?;
    if rec.route_token != Some(route_token.clone()) {
        rec.route_token = Some(route_token.clone());
        mutated = true;
    }
    if rec.primary_device_id.as_deref() != Some(primary.device_id.as_str()) {
        rec.primary_device_id = Some(primary.device_id.clone());
        mutated = true;
    }
    let multi_device = rec.devices.len() > 1;
    if mutated {
        contacts_entry_upsert(peer_alias, rec).map_err(|_| "contacts_store_invalid")?;
    }
    let channel = if multi_device {
        channel_label_for_device(peer_alias, primary.device_id.as_str())
            .ok_or("qsp_channel_invalid")?
    } else {
        peer_alias.to_string()
    };
    Ok(SendRoutingTarget {
        peer_alias: peer_alias.to_string(),
        channel,
        device_id: primary.device_id,
        route_token,
        implicit_primary,
    })
}

fn resolve_send_routing_target(peer: &str) -> Result<SendRoutingTarget, &'static str> {
    resolve_peer_device_target(peer, true)
}

fn tui_resolve_peer_device_target(
    state: &TuiState,
    peer: &str,
    require_trusted: bool,
) -> Result<SendRoutingTarget, &'static str> {
    let peer_alias = peer_alias_from_channel(peer);
    if !channel_label_ok(peer_alias) {
        return Err("unknown_contact");
    }
    let mut rec = state
        .contact_record_cached(peer_alias)
        .cloned()
        .ok_or("unknown_contact")?;
    let implicit_primary = rec.primary_device_id.is_none();
    normalize_contact_record(peer_alias, &mut rec);
    let Some(primary) = primary_device(&rec).cloned() else {
        return Err("no_trusted_device");
    };
    let canonical_state = canonical_device_state(primary.state.as_str());
    match canonical_state {
        "CHANGED" => return Err("device_changed_reapproval_required"),
        "REVOKED" => return Err("device_revoked"),
        "TRUSTED" => {}
        _ if require_trusted => return Err("no_trusted_device"),
        _ => {}
    }
    let route_token = primary
        .route_token
        .clone()
        .or_else(|| rec.route_token.clone())
        .ok_or("contact_route_token_missing")?;
    let route_token =
        normalize_route_token(route_token.as_str()).map_err(|_| "contact_route_token_missing")?;
    let multi_device = rec.devices.len() > 1;
    let channel = if multi_device {
        channel_label_for_device(peer_alias, primary.device_id.as_str())
            .ok_or("qsp_channel_invalid")?
    } else {
        peer_alias.to_string()
    };
    Ok(SendRoutingTarget {
        peer_alias: peer_alias.to_string(),
        channel,
        device_id: primary.device_id,
        route_token,
        implicit_primary,
    })
}

fn contacts_store_load() -> Result<ContactsStore, ErrorCode> {
    match vault::secret_get(CONTACTS_SECRET_KEY) {
        Ok(None) => Ok(ContactsStore::default()),
        Ok(Some(v)) => {
            let mut store =
                serde_json::from_str::<ContactsStore>(&v).map_err(|_| ErrorCode::ParseFailed)?;
            let mut migrated = false;
            for (alias, rec) in store.peers.iter_mut() {
                if normalize_contact_record(alias.as_str(), rec) {
                    migrated = true;
                }
            }
            if migrated {
                contacts_store_save(&store)?;
            }
            Ok(store)
        }
        Err("vault_missing" | "vault_locked") => Err(ErrorCode::IdentitySecretUnavailable),
        Err(_) => Err(ErrorCode::IoReadFailed),
    }
}

fn contacts_store_save(store: &ContactsStore) -> Result<(), ErrorCode> {
    let mut normalized = store.clone();
    for (alias, rec) in normalized.peers.iter_mut() {
        normalize_contact_record(alias.as_str(), rec);
    }
    let json = serde_json::to_string(&normalized).map_err(|_| ErrorCode::ParseFailed)?;
    match vault::secret_set(CONTACTS_SECRET_KEY, &json) {
        Ok(()) => Ok(()),
        Err("vault_missing" | "vault_locked") => Err(ErrorCode::IdentitySecretUnavailable),
        Err(_) => Err(ErrorCode::IoWriteFailed),
    }
}

fn contacts_entry_read(label: &str) -> Result<Option<ContactRecord>, ErrorCode> {
    if !channel_label_ok(label) {
        return Err(ErrorCode::ParseFailed);
    }
    let store = contacts_store_load()?;
    Ok(store.peers.get(label).cloned())
}

fn contact_requests_store_load() -> Result<ContactRequestsStore, ErrorCode> {
    match vault::secret_get(CONTACT_REQUESTS_SECRET_KEY) {
        Ok(None) => Ok(ContactRequestsStore::default()),
        Ok(Some(v)) => {
            serde_json::from_str::<ContactRequestsStore>(&v).map_err(|_| ErrorCode::ParseFailed)
        }
        Err("vault_missing" | "vault_locked") => Err(ErrorCode::IdentitySecretUnavailable),
        Err(_) => Err(ErrorCode::IoReadFailed),
    }
}

fn contact_requests_store_save(store: &ContactRequestsStore) -> Result<(), ErrorCode> {
    let json = serde_json::to_string(store).map_err(|_| ErrorCode::ParseFailed)?;
    match vault::secret_set(CONTACT_REQUESTS_SECRET_KEY, &json) {
        Ok(()) => Ok(()),
        Err("vault_missing" | "vault_locked") => Err(ErrorCode::IdentitySecretUnavailable),
        Err(_) => Err(ErrorCode::IoWriteFailed),
    }
}

fn contact_request_upsert(
    alias: &str,
    device_id: Option<&str>,
    reason: Option<&str>,
) -> Result<(), ErrorCode> {
    if !channel_label_ok(alias) {
        return Err(ErrorCode::ParseFailed);
    }
    let mut store = contact_requests_store_load()?;
    let rec = ContactRequestRecord {
        alias: alias.to_string(),
        device_id: device_id.map(short_device_marker),
        state: "PENDING".to_string(),
        reason: reason.map(|v| v.to_string()),
        seen_at: None,
    };
    store.requests.insert(alias.to_string(), rec);
    contact_requests_store_save(&store)
}

fn contact_request_remove(alias: &str) -> Result<bool, ErrorCode> {
    if !channel_label_ok(alias) {
        return Err(ErrorCode::ParseFailed);
    }
    let mut store = contact_requests_store_load()?;
    let removed = store.requests.remove(alias).is_some();
    if removed {
        contact_requests_store_save(&store)?;
    }
    Ok(removed)
}

fn contact_request_list() -> Result<Vec<ContactRequestRecord>, ErrorCode> {
    let mut items = contact_requests_store_load()?
        .requests
        .into_values()
        .collect::<Vec<_>>();
    items.sort_by(|a, b| a.alias.cmp(&b.alias));
    Ok(items)
}

fn contacts_entry_upsert(label: &str, rec: ContactRecord) -> Result<(), ErrorCode> {
    if !channel_label_ok(label) {
        return Err(ErrorCode::ParseFailed);
    }
    let mut store = contacts_store_load()?;
    store.peers.insert(label.to_string(), rec);
    contacts_store_save(&store)
}

fn contacts_set_blocked(label: &str, blocked: bool) -> Result<bool, ErrorCode> {
    if !channel_label_ok(label) {
        return Err(ErrorCode::ParseFailed);
    }
    let mut store = contacts_store_load()?;
    let Some(rec) = store.peers.get_mut(label) else {
        return Ok(false);
    };
    rec.blocked = blocked;
    contacts_store_save(&store)?;
    Ok(true)
}

fn contacts_list_entries() -> Result<Vec<(String, ContactRecord)>, ErrorCode> {
    let store = contacts_store_load()?;
    Ok(store.peers.into_iter().collect())
}

fn contact_state(rec: Option<&ContactRecord>) -> &'static str {
    match rec {
        Some(v) => match primary_device(v).map(|d| canonical_device_state(d.state.as_str())) {
            Some("TRUSTED") => "PINNED",
            Some("VERIFIED") => "VERIFIED",
            Some("CHANGED") => "CHANGED",
            Some("REVOKED") => "CHANGED",
            _ => "UNVERIFIED",
        },
        None => "UNVERIFIED",
    }
}

fn short_peer_marker(peer: &str) -> String {
    let all_hex = peer.chars().all(|ch| ch.is_ascii_hexdigit());
    if all_hex && peer.len() >= 32 {
        peer.chars().take(12).collect()
    } else {
        peer.to_string()
    }
}

fn short_device_marker(device: &str) -> String {
    let mut out = String::new();
    for ch in device.chars() {
        if ch.is_ascii_alphanumeric() || ch == '-' || ch == '_' {
            out.push(ch.to_ascii_lowercase());
        }
        if out.len() >= 12 {
            break;
        }
    }
    if out.is_empty() {
        "unknown".to_string()
    } else {
        out
    }
}

fn emit_cli_contact_flow(
    action: &str,
    state: &str,
    peer: &str,
    device: Option<&str>,
    mode: TrustOnboardingMode,
) {
    let safe_peer = short_peer_marker(peer);
    let safe_device = device.map(short_device_marker);
    if let Some(dev) = safe_device.as_ref() {
        emit_cli_named_marker(
            "QSC_CONTACT_FLOW",
            &[
                ("action", action),
                ("state", state),
                ("peer", safe_peer.as_str()),
                ("device", dev.as_str()),
                ("mode", mode.as_str()),
            ],
        );
    } else {
        emit_cli_named_marker(
            "QSC_CONTACT_FLOW",
            &[
                ("action", action),
                ("state", state),
                ("peer", safe_peer.as_str()),
                ("mode", mode.as_str()),
            ],
        );
    }
}

fn emit_tui_contact_flow(
    action: &str,
    state: &str,
    peer: &str,
    device: Option<&str>,
    mode: TrustOnboardingMode,
) {
    let safe_peer = short_peer_marker(peer);
    let safe_device = device.map(short_device_marker);
    if let Some(dev) = safe_device.as_ref() {
        emit_tui_named_marker(
            "QSC_TUI_CONTACT_FLOW",
            &[
                ("action", action),
                ("state", state),
                ("peer", safe_peer.as_str()),
                ("device", dev.as_str()),
                ("mode", mode.as_str()),
            ],
        );
    } else {
        emit_tui_named_marker(
            "QSC_TUI_CONTACT_FLOW",
            &[
                ("action", action),
                ("state", state),
                ("peer", safe_peer.as_str()),
                ("mode", mode.as_str()),
            ],
        );
    }
}

fn emit_cli_contact_request(action: &str, peer: &str, device: Option<&str>) {
    let safe_peer = short_peer_marker(peer);
    let safe_device = device.map(short_device_marker);
    if let Some(dev) = safe_device.as_ref() {
        emit_cli_named_marker(
            "QSC_CONTACT_REQUEST",
            &[
                ("action", action),
                ("peer", safe_peer.as_str()),
                ("device", dev),
            ],
        );
    } else {
        emit_cli_named_marker(
            "QSC_CONTACT_REQUEST",
            &[("action", action), ("peer", safe_peer.as_str())],
        );
    }
}

fn emit_tui_contact_request(action: &str, peer: &str, device: Option<&str>) {
    let safe_peer = short_peer_marker(peer);
    let safe_device = device.map(short_device_marker);
    if let Some(dev) = safe_device.as_ref() {
        emit_tui_named_marker(
            "QSC_TUI_CONTACT_REQUEST",
            &[
                ("action", action),
                ("peer", safe_peer.as_str()),
                ("device", dev),
            ],
        );
    } else {
        emit_tui_named_marker(
            "QSC_TUI_CONTACT_REQUEST",
            &[("action", action), ("peer", safe_peer.as_str())],
        );
    }
}

fn emit_cli_trust_promotion(
    result: &str,
    reason: &str,
    peer: &str,
    device: Option<&str>,
    mode: TrustOnboardingMode,
) {
    let safe_peer = short_peer_marker(peer);
    let safe_device = device.map(short_device_marker);
    if let Some(dev) = safe_device.as_ref() {
        emit_cli_named_marker(
            "QSC_TRUST_PROMOTION",
            &[
                ("result", result),
                ("reason", reason),
                ("peer", safe_peer.as_str()),
                ("device", dev.as_str()),
                ("mode", mode.as_str()),
            ],
        );
    } else {
        emit_cli_named_marker(
            "QSC_TRUST_PROMOTION",
            &[
                ("result", result),
                ("reason", reason),
                ("peer", safe_peer.as_str()),
                ("mode", mode.as_str()),
            ],
        );
    }
}

fn emit_tui_trust_promotion(
    result: &str,
    reason: &str,
    peer: &str,
    device: Option<&str>,
    mode: TrustOnboardingMode,
) {
    let safe_peer = short_peer_marker(peer);
    let safe_device = device.map(short_device_marker);
    if let Some(dev) = safe_device.as_ref() {
        emit_tui_named_marker(
            "QSC_TUI_TRUST_PROMOTION",
            &[
                ("result", result),
                ("reason", reason),
                ("peer", safe_peer.as_str()),
                ("device", dev.as_str()),
                ("mode", mode.as_str()),
            ],
        );
    } else {
        emit_tui_named_marker(
            "QSC_TUI_TRUST_PROMOTION",
            &[
                ("result", result),
                ("reason", reason),
                ("peer", safe_peer.as_str()),
                ("mode", mode.as_str()),
            ],
        );
    }
}

fn trust_remediation_steps(reason: &str) -> &'static [&'static str] {
    match reason {
        "unknown_contact" => &["add_contact", "learn_more"],
        "no_trusted_device" => &[
            "list_devices",
            "verify_device",
            "trust_device",
            "learn_more",
        ],
        "device_changed_reapproval_required" => &[
            "reapprove_changed_device",
            "verify_device",
            "trust_device",
            "learn_more",
        ],
        "device_revoked" => &[
            "readd_revoked_device",
            "verify_device",
            "trust_device",
            "learn_more",
        ],
        _ => &["learn_more"],
    }
}

fn trust_remediation_hint(reason: &str) -> &'static str {
    match reason {
        "unknown_contact" => {
            "Add contact first: /contacts add <alias> <verification_code> [route_token]"
        }
        "no_trusted_device" => {
            "No trusted device for this contact. List devices, verify one, then trust it."
        }
        "device_changed_reapproval_required" => {
            "Device changed. Re-verify and explicitly trust that device before sending."
        }
        "device_revoked" => {
            "Device revoked. Re-add or verify a replacement device before trusting it."
        }
        _ => "Send blocked by trust policy. Review contact and device trust state.",
    }
}

fn trust_remediation_verify_vs_trusted_hint() -> &'static str {
    "VERIFIED means identity/code matched; TRUSTED means send-authorized."
}

fn emit_cli_trust_remediation(reason: &str, peer: &str, device: Option<&str>) {
    let safe_peer = short_peer_marker(peer);
    let safe_device = device.map(short_device_marker);
    for step in trust_remediation_steps(reason) {
        if let Some(dev) = safe_device.as_ref() {
            emit_cli_named_marker(
                "QSC_TRUST_REMEDIATION",
                &[
                    ("reason", reason),
                    ("step", step),
                    ("peer", safe_peer.as_str()),
                    ("device", dev.as_str()),
                ],
            );
        } else {
            emit_cli_named_marker(
                "QSC_TRUST_REMEDIATION",
                &[
                    ("reason", reason),
                    ("step", step),
                    ("peer", safe_peer.as_str()),
                ],
            );
        }
    }
}

fn emit_tui_trust_remediation(reason: &str, peer: &str, device: Option<&str>) {
    let safe_peer = short_peer_marker(peer);
    let safe_device = device.map(short_device_marker);
    for step in trust_remediation_steps(reason) {
        if let Some(dev) = safe_device.as_ref() {
            emit_tui_named_marker(
                "QSC_TUI_TRUST_REMEDIATION",
                &[
                    ("reason", reason),
                    ("step", step),
                    ("peer", safe_peer.as_str()),
                    ("device", dev.as_str()),
                ],
            );
        } else {
            emit_tui_named_marker(
                "QSC_TUI_TRUST_REMEDIATION",
                &[
                    ("reason", reason),
                    ("step", step),
                    ("peer", safe_peer.as_str()),
                ],
            );
        }
    }
}

fn trust_gate_device_hint(peer: &str, reason: &str) -> Option<String> {
    match reason {
        "no_trusted_device" | "device_changed_reapproval_required" | "device_revoked" => {}
        _ => return None,
    }
    let alias = peer_alias_from_channel(peer);
    if !channel_label_ok(alias) {
        return None;
    }
    let rec = contacts_entry_read(alias).ok().flatten()?;
    let primary = primary_device(&rec)?;
    Some(short_device_marker(primary.device_id.as_str()))
}

fn send_contact_trust_gate(peer: &str) -> Result<(), &'static str> {
    let peer_alias = peer_alias_from_channel(peer);
    if !channel_label_ok(peer_alias) {
        return Err("unknown_contact");
    }
    let rec = contacts_entry_read(peer_alias).map_err(|_| "contacts_store_invalid")?;
    let Some(rec) = rec else {
        return Err("unknown_contact");
    };
    let Some(primary) = primary_device(&rec) else {
        return Err("no_trusted_device");
    };
    match canonical_device_state(primary.state.as_str()) {
        "CHANGED" => return Err("device_changed_reapproval_required"),
        "REVOKED" => return Err("device_revoked"),
        _ => {}
    }
    if !contact_has_trusted_device(&rec) {
        return Err("no_trusted_device");
    }
    Ok(())
}

fn emit_cli_send_blocked(reason: &'static str, peer: &str, device: Option<&str>) {
    let safe_peer = short_peer_marker(peer);
    emit_cli_named_marker(
        "QSC_SEND_BLOCKED",
        &[("reason", reason), ("peer", safe_peer.as_str())],
    );
    emit_marker(
        "send_blocked",
        Some(reason),
        &[("reason", reason), ("peer", safe_peer.as_str())],
    );
    emit_cli_trust_remediation(reason, peer, device);
    eprintln!("HINT: {}", trust_remediation_hint(reason));
    eprintln!("HINT: {}", trust_remediation_verify_vs_trusted_hint());
}

fn emit_cli_routing_marker(peer: &str, device_id: &str, implicit: bool) {
    let safe_peer = short_peer_marker(peer);
    let mut fields = vec![
        ("policy", "primary_only"),
        ("peer", safe_peer.as_str()),
        ("device", device_id),
    ];
    if implicit {
        fields.push(("selected", "implicit"));
    }
    emit_cli_named_marker("QSC_ROUTING", fields.as_slice());
}

fn emit_tui_routing_marker(thread: &str, device_id: &str, implicit: bool) {
    let safe_thread = short_peer_marker(thread);
    let mut fields = vec![
        ("policy", "primary_only"),
        ("thread", safe_thread.as_str()),
        ("device", device_id),
    ];
    if implicit {
        fields.push(("selected", "implicit"));
    }
    emit_tui_named_marker("QSC_TUI_ROUTING", fields.as_slice());
}

fn enforce_cli_send_contact_trust(peer: &str) -> Result<(), &'static str> {
    match send_contact_trust_gate(peer) {
        Ok(()) => Ok(()),
        Err("unknown_contact") => {
            emit_cli_send_blocked("unknown_contact", peer, None);
            Err("unknown_contact")
        }
        Err("no_trusted_device") => {
            let device = trust_gate_device_hint(peer, "no_trusted_device");
            emit_cli_send_blocked("no_trusted_device", peer, device.as_deref());
            Err("no_trusted_device")
        }
        Err("device_changed_reapproval_required") => {
            let device = trust_gate_device_hint(peer, "device_changed_reapproval_required");
            emit_cli_send_blocked(
                "device_changed_reapproval_required",
                peer,
                device.as_deref(),
            );
            Err("device_changed_reapproval_required")
        }
        Err("device_revoked") => {
            let device = trust_gate_device_hint(peer, "device_revoked");
            emit_cli_send_blocked("device_revoked", peer, device.as_deref());
            Err("device_revoked")
        }
        Err(code) => Err(code),
    }
}

fn contact_blocked(label: &str) -> Result<bool, ErrorCode> {
    let alias = peer_alias_from_channel(label);
    Ok(contacts_entry_read(alias)?
        .map(|v| v.blocked)
        .unwrap_or(false))
}

fn tui_contact_blocked(state: &TuiState, label: &str) -> Result<bool, &'static str> {
    let alias = peer_alias_from_channel(label);
    let rec = state
        .contact_record_cached(alias)
        .ok_or("unknown_contact")?;
    Ok(rec.blocked)
}

fn enforce_peer_not_blocked(label: &str) -> Result<(), &'static str> {
    let alias = peer_alias_from_channel(label);
    match contact_blocked(label) {
        Ok(true) => {
            emit_marker(
                "contacts_refuse",
                None,
                &[("label", alias), ("reason", "peer_blocked")],
            );
            Err("peer_blocked")
        }
        Ok(false) => Ok(()),
        // Missing/locked contacts store means no explicit block policy is available.
        Err(ErrorCode::IdentitySecretUnavailable) => Ok(()),
        Err(_) => Err("contacts_store_invalid"),
    }
}

fn tui_enforce_peer_not_blocked(state: &TuiState, label: &str) -> Result<(), &'static str> {
    let alias = peer_alias_from_channel(label);
    match tui_contact_blocked(state, label) {
        Ok(true) => {
            emit_marker(
                "contacts_refuse",
                None,
                &[("label", alias), ("reason", "peer_blocked")],
            );
            Err("peer_blocked")
        }
        Ok(false) => Ok(()),
        Err(code) => Err(code),
    }
}

fn emit_peer_mismatch(peer: &str, pinned_fp: &str, seen_fp: &str) {
    let pinned_display = identity_marker_display(pinned_fp);
    let seen_display = identity_marker_display(seen_fp);
    emit_marker(
        "identity_mismatch",
        None,
        &[
            ("peer", peer),
            ("pinned_fp", pinned_display.as_str()),
            ("seen_fp", seen_display.as_str()),
        ],
    );
    emit_marker("error", Some("peer_mismatch"), &[("peer", peer)]);
}

fn contacts_add(label: &str, fp: &str, route_token: Option<&str>, verify: bool) {
    if !require_unlocked("contacts_add") {
        return;
    }
    let status = if verify { "verified" } else { "pinned" };
    let route_token = match route_token {
        Some(raw) => {
            Some(normalize_route_token(raw).unwrap_or_else(|code| print_error_marker(code)))
        }
        None => Some(generate_route_token()),
    };
    let rec = ContactRecord {
        fp: fp.to_string(),
        status: status.to_string(),
        blocked: false,
        seen_at: None,
        sig_fp: None,
        route_token: route_token.clone(),
        primary_device_id: None,
        devices: vec![ContactDeviceRecord {
            device_id: device_id_short(label, None, fp),
            fp: fp.to_string(),
            sig_fp: None,
            state: legacy_contact_status_to_device_state(status).to_string(),
            route_token: route_token.clone(),
            seen_at: None,
            label: None,
        }],
    };
    if contacts_entry_upsert(label, rec).is_err() {
        print_error_marker("contacts_store_unavailable");
    }
    emit_marker(
        "contacts_add",
        None,
        &[("ok", "true"), ("label", label), ("status", status)],
    );
    let mode = load_trust_onboarding_mode_from_account();
    let state = if verify { "VERIFIED" } else { "DISCOVERED" };
    emit_cli_contact_flow("add", state, label, None, mode);
    println!("contact={} status={}", label, status);
}

fn contact_device_find_index(rec: &ContactRecord, device_id: &str) -> Option<usize> {
    rec.devices.iter().position(|d| d.device_id == device_id)
}

fn contact_has_trusted_device(rec: &ContactRecord) -> bool {
    rec.devices
        .iter()
        .any(|d| canonical_device_state(d.state.as_str()) == "TRUSTED")
}

fn contacts_device_add(label: &str, fp: &str, route_token: Option<&str>) {
    if !require_unlocked("contacts_device_add") {
        return;
    }
    let mut rec = contacts_entry_read(label)
        .unwrap_or_else(|_| print_error_marker("contacts_store_unavailable"))
        .unwrap_or_else(|| print_error_marker("peer_unknown"));
    normalize_contact_record(label, &mut rec);
    let route_token = route_token
        .map(|raw| normalize_route_token(raw).unwrap_or_else(|code| print_error_marker(code)));
    let device_id = device_id_short(label, None, fp);
    if contact_device_find_index(&rec, device_id.as_str()).is_some() {
        emit_marker(
            "contacts_device_add",
            Some("device_exists"),
            &[
                ("ok", "false"),
                ("label", label),
                ("device", device_id.as_str()),
            ],
        );
        print_error_marker("device_exists");
    }
    rec.devices.push(ContactDeviceRecord {
        device_id: device_id.clone(),
        fp: fp.to_ascii_uppercase(),
        sig_fp: None,
        state: "UNVERIFIED".to_string(),
        route_token,
        seen_at: None,
        label: None,
    });
    normalize_contact_record(label, &mut rec);
    if contacts_entry_upsert(label, rec).is_err() {
        print_error_marker("contacts_store_unavailable");
    }
    emit_marker(
        "contacts_device_add",
        None,
        &[
            ("ok", "true"),
            ("label", label),
            ("device", device_id.as_str()),
            ("state", "UNVERIFIED"),
        ],
    );
}

fn contacts_device_list(label: &str) {
    if !require_unlocked("contacts_device_list") {
        return;
    }
    let rec = contacts_entry_read(label)
        .unwrap_or_else(|_| print_error_marker("contacts_store_unavailable"))
        .unwrap_or_else(|| print_error_marker("peer_unknown"));
    let mut rec = rec;
    normalize_contact_record(label, &mut rec);
    let count_s = rec.devices.len().to_string();
    emit_marker(
        "contacts_device_list",
        None,
        &[("label", label), ("count", count_s.as_str())],
    );
    let primary = primary_device(&rec)
        .map(|d| d.device_id.as_str())
        .unwrap_or("none");
    println!(
        "label={} device_count={} primary_device={}",
        label, count_s, primary
    );
    for dev in rec.devices {
        println!(
            "device={} state={}",
            dev.device_id,
            canonical_device_state(dev.state.as_str())
        );
    }
}

fn contacts_device_status(label: &str, device: Option<&str>) {
    if !require_unlocked("contacts_device_status") {
        return;
    }
    let rec = contacts_entry_read(label)
        .unwrap_or_else(|_| print_error_marker("contacts_store_unavailable"))
        .unwrap_or_else(|| print_error_marker("peer_unknown"));
    let mut rec = rec;
    normalize_contact_record(label, &mut rec);
    let primary = primary_device(&rec)
        .map(|d| d.device_id.as_str())
        .unwrap_or("none")
        .to_string();
    match device {
        Some(device_id) => {
            let Some(idx) = contact_device_find_index(&rec, device_id) else {
                print_error_marker("device_unknown");
            };
            let dev = &rec.devices[idx];
            let state = canonical_device_state(dev.state.as_str());
            emit_marker(
                "contacts_device_status",
                None,
                &[
                    ("label", label),
                    ("device", device_id),
                    ("state", state),
                    ("primary", bool_str(primary == device_id)),
                ],
            );
            println!(
                "label={} device={} state={} primary={}",
                label,
                device_id,
                state,
                bool_str(primary == device_id)
            );
        }
        None => {
            let count_s = rec.devices.len().to_string();
            emit_marker(
                "contacts_device_status",
                None,
                &[("label", label), ("count", count_s.as_str())],
            );
            println!(
                "label={} device_count={} primary_device={}",
                label, count_s, primary
            );
            for dev in rec.devices.iter() {
                println!(
                    "device={} state={} primary={}",
                    dev.device_id,
                    canonical_device_state(dev.state.as_str()),
                    bool_str(dev.device_id == primary)
                );
            }
        }
    }
}

fn contacts_device_verify(label: &str, device: &str, fp: &str) {
    if !require_unlocked("contacts_device_verify") {
        return;
    }
    let mut rec = contacts_entry_read(label)
        .unwrap_or_else(|_| print_error_marker("contacts_store_unavailable"))
        .unwrap_or_else(|| print_error_marker("peer_unknown"));
    normalize_contact_record(label, &mut rec);
    let Some(idx) = contact_device_find_index(&rec, device) else {
        print_error_marker("device_unknown");
    };
    let expected = rec.devices[idx].fp.to_ascii_uppercase();
    let provided = fp.to_ascii_uppercase();
    if expected == provided {
        let mode = load_trust_onboarding_mode_from_account();
        rec.devices[idx].state = "VERIFIED".to_string();
        rec.status = "VERIFIED".to_string();
        if mode == TrustOnboardingMode::Balanced {
            rec.devices[idx].state = "TRUSTED".to_string();
            rec.status = "PINNED".to_string();
        }
        if contacts_entry_upsert(label, rec).is_err() {
            print_error_marker("contacts_store_unavailable");
        }
        emit_marker(
            "contacts_device_verify",
            None,
            &[
                ("ok", "true"),
                ("label", label),
                ("device", device),
                (
                    "state",
                    if mode == TrustOnboardingMode::Balanced {
                        "TRUSTED"
                    } else {
                        "VERIFIED"
                    },
                ),
            ],
        );
        emit_cli_contact_flow(
            "verify",
            if mode == TrustOnboardingMode::Balanced {
                "TRUSTED"
            } else {
                "VERIFIED"
            },
            label,
            Some(device),
            mode,
        );
        if mode == TrustOnboardingMode::Balanced {
            emit_cli_trust_promotion("trusted", "verified_match", label, Some(device), mode);
        } else {
            emit_cli_trust_promotion("verified_only", "strict_mode", label, Some(device), mode);
        }
        return;
    }
    rec.devices[idx].state = "CHANGED".to_string();
    rec.status = "CHANGED".to_string();
    let _ = contacts_entry_upsert(label, rec);
    emit_marker(
        "contacts_device_verify",
        Some("verification_mismatch"),
        &[
            ("ok", "false"),
            ("label", label),
            ("device", device),
            ("state", "CHANGED"),
        ],
    );
    emit_cli_contact_flow(
        "verify",
        "CHANGED",
        label,
        Some(device),
        load_trust_onboarding_mode_from_account(),
    );
    print_error_marker("verification_mismatch");
}

fn contacts_device_trust(label: &str, device: &str, confirm: bool) {
    if !require_unlocked("contacts_device_trust") {
        return;
    }
    if !confirm {
        print_error_marker("trust_requires_confirm");
    }
    let mut rec = contacts_entry_read(label)
        .unwrap_or_else(|_| print_error_marker("contacts_store_unavailable"))
        .unwrap_or_else(|| print_error_marker("peer_unknown"));
    normalize_contact_record(label, &mut rec);
    let Some(idx) = contact_device_find_index(&rec, device) else {
        print_error_marker("device_unknown");
    };
    rec.devices[idx].state = "TRUSTED".to_string();
    rec.status = "PINNED".to_string();
    if contacts_entry_upsert(label, rec).is_err() {
        print_error_marker("contacts_store_unavailable");
    }
    emit_marker(
        "contacts_device_trust",
        None,
        &[
            ("ok", "true"),
            ("label", label),
            ("device", device),
            ("state", "TRUSTED"),
        ],
    );
    let mode = load_trust_onboarding_mode_from_account();
    emit_cli_contact_flow("trust", "TRUSTED", label, Some(device), mode);
    emit_cli_trust_promotion(
        "trusted",
        "explicit_operator_action",
        label,
        Some(device),
        mode,
    );
}

fn contacts_device_revoke(label: &str, device: &str, confirm: bool) {
    if !require_unlocked("contacts_device_revoke") {
        return;
    }
    if !confirm {
        print_error_marker("revoke_requires_confirm");
    }
    let mut rec = contacts_entry_read(label)
        .unwrap_or_else(|_| print_error_marker("contacts_store_unavailable"))
        .unwrap_or_else(|| print_error_marker("peer_unknown"));
    normalize_contact_record(label, &mut rec);
    let Some(idx) = contact_device_find_index(&rec, device) else {
        print_error_marker("device_unknown");
    };
    rec.devices[idx].state = "REVOKED".to_string();
    if contacts_entry_upsert(label, rec).is_err() {
        print_error_marker("contacts_store_unavailable");
    }
    emit_marker(
        "contacts_device_revoke",
        None,
        &[
            ("ok", "true"),
            ("label", label),
            ("device", device),
            ("state", "REVOKED"),
        ],
    );
}

fn contacts_device_primary_set(label: &str, device: &str, confirm: bool) {
    if !require_unlocked("contacts_device_primary_set") {
        return;
    }
    if !confirm {
        print_error_marker("primary_set_requires_confirm");
    }
    let mut rec = contacts_entry_read(label)
        .unwrap_or_else(|_| print_error_marker("contacts_store_unavailable"))
        .unwrap_or_else(|| print_error_marker("peer_unknown"));
    normalize_contact_record(label, &mut rec);
    let Some(_) = contact_device_find_index(&rec, device) else {
        print_error_marker("device_unknown");
    };
    rec.primary_device_id = Some(device.to_string());
    normalize_contact_record(label, &mut rec);
    if contacts_entry_upsert(label, rec).is_err() {
        print_error_marker("contacts_store_unavailable");
    }
    emit_marker(
        "contacts_device_primary_set",
        None,
        &[
            ("ok", "true"),
            ("label", label),
            ("device", device),
            ("selected", "explicit"),
            ("policy", "primary_only"),
        ],
    );
}

fn contacts_device_primary_show(label: &str) {
    if !require_unlocked("contacts_device_primary_show") {
        return;
    }
    let mut rec = contacts_entry_read(label)
        .unwrap_or_else(|_| print_error_marker("contacts_store_unavailable"))
        .unwrap_or_else(|| print_error_marker("peer_unknown"));
    let implicit = rec.primary_device_id.is_none();
    normalize_contact_record(label, &mut rec);
    let primary = primary_device(&rec)
        .map(|d| d.device_id.as_str())
        .unwrap_or("none");
    let selected = if implicit { "implicit" } else { "explicit" };
    emit_marker(
        "contacts_device_primary_show",
        None,
        &[
            ("label", label),
            ("device", primary),
            ("selected", selected),
            ("policy", "primary_only"),
        ],
    );
    println!(
        "label={} primary_device={} selected={} policy=primary_only",
        label, primary, selected
    );
}

fn contacts_route_set(label: &str, route_token: &str) {
    if !require_unlocked("contacts_route_set") {
        return;
    }
    let token = normalize_route_token(route_token).unwrap_or_else(|code| print_error_marker(code));
    let mut rec = contacts_entry_read(label)
        .unwrap_or_else(|_| print_error_marker("contacts_store_unavailable"))
        .unwrap_or(ContactRecord {
            fp: "UNSET".to_string(),
            status: "UNVERIFIED".to_string(),
            blocked: false,
            seen_at: None,
            sig_fp: None,
            route_token: None,
            primary_device_id: None,
            devices: vec![ContactDeviceRecord {
                device_id: device_id_short(label, None, "UNSET"),
                fp: "UNSET".to_string(),
                sig_fp: None,
                state: "UNVERIFIED".to_string(),
                route_token: None,
                seen_at: None,
                label: None,
            }],
        });
    rec.route_token = Some(token);
    normalize_contact_record(label, &mut rec);
    let primary_route_token = rec.route_token.clone();
    if let Some(primary) = primary_device_mut(&mut rec) {
        primary.route_token = primary_route_token;
    }
    if contacts_entry_upsert(label, rec).is_err() {
        print_error_marker("contacts_store_unavailable");
    }
    emit_marker(
        "contacts_route_set",
        None,
        &[("ok", "true"), ("label", label)],
    );
}

fn contacts_show(label: &str) {
    if !require_unlocked("contacts_show") {
        return;
    }
    let rec = contacts_entry_read(label)
        .unwrap_or_else(|_| print_error_marker("contacts_store_unavailable"));
    let state = contact_state(rec.as_ref());
    let blocked = bool_str(rec.as_ref().map(|v| v.blocked).unwrap_or(false));
    let device_count = rec.as_ref().map(|v| v.devices.len()).unwrap_or(0);
    let device_count_s = device_count.to_string();
    emit_marker(
        "contacts_show",
        None,
        &[
            ("label", label),
            ("state", state),
            ("blocked", blocked),
            ("device_count", device_count_s.as_str()),
        ],
    );
    if let Some(v) = rec {
        let primary_id = primary_device(&v)
            .map(|d| d.device_id.as_str())
            .unwrap_or("none");
        println!(
            "label={} state={} blocked={} device_count={} primary_device={}",
            label, state, blocked, device_count, primary_id
        );
        for dev in v.devices.iter() {
            let state = canonical_device_state(dev.state.as_str());
            println!("device={} state={}", dev.device_id, state);
        }
    } else {
        println!("label={} state=unknown blocked=false", label);
    }
}

fn contacts_list() {
    if !require_unlocked("contacts_list") {
        return;
    }
    let mut entries = contacts_list_entries()
        .unwrap_or_else(|_| print_error_marker("contacts_store_unavailable"));
    entries.sort_by(|a, b| a.0.cmp(&b.0));
    let count_s = entries.len().to_string();
    emit_marker("contacts_list", None, &[("count", count_s.as_str())]);
    for (label, rec) in entries {
        let state = contact_state(Some(&rec));
        let blocked = bool_str(rec.blocked);
        let device_count = rec.devices.len();
        let primary_id = primary_device(&rec)
            .map(|d| d.device_id.as_str())
            .unwrap_or("none");
        println!(
            "label={} state={} blocked={} device_count={} primary_device={}",
            label, state, blocked, device_count, primary_id
        );
    }
}

fn contacts_verify(label: &str, fp: &str, confirm: bool) {
    if !require_unlocked("contacts_verify") {
        return;
    }
    let Some(mut rec) = contacts_entry_read(label)
        .unwrap_or_else(|_| print_error_marker("contacts_store_unavailable"))
    else {
        emit_marker(
            "contacts_verify",
            None,
            &[
                ("ok", "false"),
                ("label", label),
                ("result", "refused"),
                ("reason", "peer_unknown"),
            ],
        );
        print_error_marker("peer_unknown");
    };
    normalize_contact_record(label, &mut rec);
    let primary = primary_device(&rec).map(|d| d.device_id.clone());
    let Some(primary) = primary else {
        print_error_marker("device_unknown");
    };
    if !confirm {
        emit_marker(
            "contacts_verify",
            None,
            &[
                ("ok", "false"),
                ("label", label),
                ("result", "refused"),
                ("reason", "confirm_required"),
            ],
        );
        print_error_marker("verify_requires_confirm");
    }
    contacts_device_verify(label, primary.as_str(), fp);
}

fn contacts_block(label: &str) {
    if !require_unlocked("contacts_block") {
        return;
    }
    match contacts_set_blocked(label, true) {
        Ok(true) => emit_marker("contacts_block", None, &[("label", label), ("ok", "true")]),
        Ok(false) => print_error_marker("peer_unknown"),
        Err(_) => print_error_marker("contacts_store_unavailable"),
    }
}

fn contacts_unblock(label: &str) {
    if !require_unlocked("contacts_unblock") {
        return;
    }
    match contacts_set_blocked(label, false) {
        Ok(true) => emit_marker(
            "contacts_unblock",
            None,
            &[("label", label), ("ok", "true")],
        ),
        Ok(false) => print_error_marker("peer_unknown"),
        Err(_) => print_error_marker("contacts_store_unavailable"),
    }
}

fn contacts_trust_mode_show() {
    if !require_unlocked("contacts_trust_mode_show") {
        return;
    }
    let mode = load_trust_onboarding_mode_from_account();
    emit_cli_named_marker("QSC_TRUST_MODE", &[("mode", mode.as_str())]);
    println!("trust_mode={}", mode.as_str());
}

fn contacts_trust_mode_set(mode: TrustMode) {
    if !require_unlocked("contacts_trust_mode_set") {
        return;
    }
    let mode = TrustOnboardingMode::from_arg(mode);
    match vault::secret_set(TUI_TRUST_MODE_SECRET_KEY, mode.as_str()) {
        Ok(()) => {
            emit_cli_named_marker("QSC_TRUST_MODE", &[("mode", mode.as_str()), ("ok", "true")]);
            println!("trust_mode={}", mode.as_str());
        }
        Err(_) => print_error_marker("contacts_store_unavailable"),
    }
}

fn contacts_request_list() {
    if !require_unlocked("contacts_request_list") {
        return;
    }
    let items =
        contact_request_list().unwrap_or_else(|_| print_error_marker("contacts_store_unavailable"));
    let count_s = items.len().to_string();
    emit_cli_named_marker(
        "QSC_CONTACT_REQUEST",
        &[("action", "list"), ("count", count_s.as_str())],
    );
    for item in items {
        println!(
            "request alias={} state={} device={}",
            item.alias,
            item.state,
            item.device_id.unwrap_or_else(|| "unknown".to_string())
        );
    }
}

fn contacts_request_accept(label: &str) {
    if !require_unlocked("contacts_request_accept") {
        return;
    }
    let removed = contact_request_remove(label)
        .unwrap_or_else(|_| print_error_marker("contacts_store_unavailable"));
    if !removed {
        print_error_marker("request_unknown");
    }
    let fp = "UNSET".to_string();
    let mut rec = contacts_entry_read(label)
        .unwrap_or_else(|_| print_error_marker("contacts_store_unavailable"))
        .unwrap_or(ContactRecord {
            fp: fp.clone(),
            status: "UNVERIFIED".to_string(),
            blocked: false,
            seen_at: None,
            sig_fp: None,
            route_token: None,
            primary_device_id: None,
            devices: vec![ContactDeviceRecord {
                device_id: device_id_short(label, None, fp.as_str()),
                fp: fp.clone(),
                sig_fp: None,
                state: "UNVERIFIED".to_string(),
                route_token: None,
                seen_at: None,
                label: Some("request".to_string()),
            }],
        });
    normalize_contact_record(label, &mut rec);
    rec.status = "UNVERIFIED".to_string();
    if contacts_entry_upsert(label, rec).is_err() {
        print_error_marker("contacts_store_unavailable");
    }
    emit_cli_contact_request("accept", label, None);
    emit_cli_contact_flow(
        "add",
        "DISCOVERED",
        label,
        None,
        load_trust_onboarding_mode_from_account(),
    );
}

fn contacts_request_ignore(label: &str) {
    if !require_unlocked("contacts_request_ignore") {
        return;
    }
    let removed = contact_request_remove(label)
        .unwrap_or_else(|_| print_error_marker("contacts_store_unavailable"));
    if !removed {
        print_error_marker("request_unknown");
    }
    emit_cli_contact_request("ignore", label, None);
}

fn contacts_request_block(label: &str) {
    if !require_unlocked("contacts_request_block") {
        return;
    }
    let _ = contact_request_remove(label);
    let mut rec = contacts_entry_read(label)
        .unwrap_or_else(|_| print_error_marker("contacts_store_unavailable"))
        .unwrap_or(ContactRecord {
            fp: "UNSET".to_string(),
            status: "REVOKED".to_string(),
            blocked: true,
            seen_at: None,
            sig_fp: None,
            route_token: None,
            primary_device_id: None,
            devices: vec![ContactDeviceRecord {
                device_id: device_id_short(label, None, "UNSET"),
                fp: "UNSET".to_string(),
                sig_fp: None,
                state: "REVOKED".to_string(),
                route_token: None,
                seen_at: None,
                label: Some("blocked_request".to_string()),
            }],
        });
    normalize_contact_record(label, &mut rec);
    rec.blocked = true;
    if let Some(primary) = primary_device_mut(&mut rec) {
        primary.state = "REVOKED".to_string();
    }
    if contacts_entry_upsert(label, rec).is_err() {
        print_error_marker("contacts_store_unavailable");
    }
    emit_cli_contact_request("block", label, None);
}

fn timeline_store_load() -> Result<TimelineStore, &'static str> {
    let mut store = match vault::secret_get(TIMELINE_SECRET_KEY) {
        Ok(None) => Ok(TimelineStore::default()),
        Ok(Some(v)) => serde_json::from_str::<TimelineStore>(&v).map_err(|_| "timeline_tampered"),
        Err("vault_missing" | "vault_locked") => Err("timeline_unavailable"),
        Err(_) => Err("timeline_unavailable"),
    }?;
    if store.next_ts == 0 {
        store.next_ts = 1;
    }
    Ok(store)
}

fn timeline_store_save(store: &TimelineStore) -> Result<(), &'static str> {
    let json = serde_json::to_string(store).map_err(|_| "timeline_unavailable")?;
    match vault::secret_set(TIMELINE_SECRET_KEY, &json) {
        Ok(()) => Ok(()),
        Err("vault_missing" | "vault_locked") => Err("timeline_unavailable"),
        Err(_) => Err("timeline_unavailable"),
    }
}

fn timeline_append_entry(
    peer: &str,
    direction: &str,
    byte_len: usize,
    kind: &str,
    final_state: MessageState,
    forced_id: Option<&str>,
) -> Result<TimelineEntry, &'static str> {
    timeline_append_entry_for_target(
        peer,
        direction,
        byte_len,
        kind,
        final_state,
        forced_id,
        None,
    )
}

fn timeline_append_entry_for_target(
    peer: &str,
    direction: &str,
    byte_len: usize,
    kind: &str,
    final_state: MessageState,
    forced_id: Option<&str>,
    target_device_id: Option<&str>,
) -> Result<TimelineEntry, &'static str> {
    if !channel_label_ok(peer) {
        return Err("timeline_peer_invalid");
    }
    if let Some(v) = forced_id {
        if v.trim().is_empty() {
            return Err("state_id_invalid");
        }
    }
    message_state_transition_allowed(MessageState::Created, final_state, direction)?;
    let mut store = timeline_store_load()?;
    let ts = store.next_ts;
    store.next_ts = store.next_ts.saturating_add(1);
    let id = forced_id
        .map(|v| v.to_string())
        .unwrap_or_else(|| format!("{}-{}", direction, ts));
    let entry = TimelineEntry {
        id: id.clone(),
        peer: peer.to_string(),
        direction: direction.to_string(),
        byte_len,
        kind: kind.to_string(),
        ts,
        target_device_id: target_device_id.map(short_device_marker),
        state: final_state.as_str().to_string(),
        status: final_state.as_status().to_string(),
    };
    store
        .peers
        .entry(peer.to_string())
        .or_default()
        .push(entry.clone());
    timeline_store_save(&store)?;
    emit_message_state_transition(id.as_str(), MessageState::Created, final_state);
    Ok(entry)
}

fn timeline_transition_entry_state(
    peer: &str,
    id: &str,
    to: MessageState,
) -> Result<TimelineEntry, &'static str> {
    if !channel_label_ok(peer) {
        return Err("timeline_peer_invalid");
    }
    if id.trim().is_empty() {
        return Err("state_id_invalid");
    }
    let mut store = timeline_store_load()?;
    let Some(entries) = store.peers.get_mut(peer) else {
        return Err("state_unknown");
    };
    let Some(entry) = entries.iter_mut().find(|v| v.id == id) else {
        return Err("state_unknown");
    };
    let from = timeline_entry_state(entry);
    message_state_transition_allowed(from, to, entry.direction.as_str())?;
    entry.state = to.as_str().to_string();
    entry.status = to.as_status().to_string();
    let out = entry.clone();
    timeline_store_save(&store)?;
    emit_message_state_transition(id, from, to);
    Ok(out)
}

fn timeline_entries_for_peer(peer: &str) -> Result<Vec<TimelineEntry>, &'static str> {
    if !channel_label_ok(peer) {
        return Err("timeline_peer_invalid");
    }
    let store = timeline_store_load()?;
    Ok(store.peers.get(peer).cloned().unwrap_or_default())
}

fn timeline_outbound_target_device(peer: &str, id: &str) -> Result<Option<String>, &'static str> {
    if !channel_label_ok(peer) {
        return Err("timeline_peer_invalid");
    }
    if id.trim().is_empty() {
        return Err("state_id_invalid");
    }
    let store = timeline_store_load()?;
    let Some(entries) = store.peers.get(peer) else {
        return Err("state_unknown");
    };
    let Some(entry) = entries.iter().find(|v| v.id == id) else {
        return Err("state_unknown");
    };
    Ok(entry.target_device_id.clone())
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ConfirmApplyOutcome {
    Confirmed,
    IgnoredWrongDevice,
}

fn apply_message_peer_confirmation(
    peer: &str,
    msg_id: &str,
    recv_channel: &str,
) -> Result<(ConfirmApplyOutcome, Option<String>), &'static str> {
    let target = timeline_outbound_target_device(peer, msg_id)?;
    if !confirm_target_matches_channel(target.as_deref(), recv_channel) {
        return Ok((ConfirmApplyOutcome::IgnoredWrongDevice, target));
    }
    timeline_transition_entry_state(peer, msg_id, MessageState::Delivered)?;
    Ok((ConfirmApplyOutcome::Confirmed, target))
}

fn apply_file_peer_confirmation(
    peer: &str,
    file_id: &str,
    confirm_id: &str,
    recv_channel: &str,
) -> Result<(ConfirmApplyOutcome, Option<String>), &'static str> {
    let target = file_transfer_target_device(peer, file_id)?;
    if !confirm_target_matches_channel(target.as_deref(), recv_channel) {
        return Ok((ConfirmApplyOutcome::IgnoredWrongDevice, target));
    }
    file_transfer_apply_confirmation(peer, file_id, confirm_id, recv_channel)?;
    timeline_transition_entry_state(peer, file_id, MessageState::Delivered)?;
    Ok((ConfirmApplyOutcome::Confirmed, target))
}

fn apply_attachment_peer_confirmation(
    peer: &str,
    attachment_id: &str,
    confirm_handle: &str,
    recv_channel: &str,
) -> Result<(ConfirmApplyOutcome, Option<String>), &'static str> {
    let key = attachment_record_key("out", peer, attachment_id);
    let store = attachment_journal_load()?;
    let Some(rec) = store.records.get(&key) else {
        return Err("REJECT_ATT_CONFIRM_LINKAGE");
    };
    let target = rec.target_device_id.clone();
    if !confirm_target_matches_channel(target.as_deref(), recv_channel) {
        return Ok((ConfirmApplyOutcome::IgnoredWrongDevice, target));
    }
    attachment_transfer_apply_confirmation(peer, attachment_id, confirm_handle, recv_channel)?;
    let timeline_id = attachment_transfer_timeline_id(peer, attachment_id)?;
    timeline_transition_entry_state(peer, timeline_id.as_str(), MessageState::Delivered)?;
    Ok((ConfirmApplyOutcome::Confirmed, target))
}

fn timeline_emit_item(entry: &TimelineEntry) {
    let len_s = entry.byte_len.to_string();
    let ts_s = entry.ts.to_string();
    let state = timeline_entry_state(entry);
    emit_marker(
        "timeline_item",
        None,
        &[
            ("id", entry.id.as_str()),
            ("dir", entry.direction.as_str()),
            ("len", len_s.as_str()),
            ("kind", entry.kind.as_str()),
            ("ts", ts_s.as_str()),
            ("state", state.as_str()),
        ],
    );
    if let Some(delivery) = message_delivery_semantic(entry.direction.as_str(), state) {
        if entry.kind == "file" {
            emit_cli_file_delivery_with_device(
                entry.peer.as_str(),
                delivery,
                entry.id.as_str(),
                entry.target_device_id.as_deref(),
            );
        } else {
            let safe_peer = short_peer_marker(entry.peer.as_str());
            emit_cli_named_marker(
                "QSC_DELIVERY",
                &[("state", delivery), ("peer", safe_peer.as_str())],
            );
        }
    }
}

fn latest_outbound_file_id(peer: &str) -> Result<String, &'static str> {
    let entries = timeline_entries_for_peer(peer)?;
    let Some(entry) = entries
        .into_iter()
        .filter(|v| v.direction == "out" && v.kind == "file")
        .max_by(|a, b| a.ts.cmp(&b.ts).then_with(|| a.id.cmp(&b.id)))
    else {
        return Err("state_unknown");
    };
    Ok(entry.id)
}

fn timeline_list(peer: &str, limit: Option<usize>) {
    if !require_unlocked("timeline_list") {
        return;
    }
    let mut entries =
        timeline_entries_for_peer(peer).unwrap_or_else(|code| print_error_marker(code));
    entries.sort_by(|a, b| b.ts.cmp(&a.ts).then_with(|| a.id.cmp(&b.id)));
    let take_n = limit.unwrap_or(entries.len()).min(entries.len());
    let count_s = take_n.to_string();
    emit_marker(
        "timeline_list",
        None,
        &[("count", count_s.as_str()), ("peer", peer)],
    );
    for entry in entries.into_iter().take(take_n) {
        timeline_emit_item(&entry);
    }
}

fn timeline_show(peer: &str, id: &str) {
    if !require_unlocked("timeline_show") {
        return;
    }
    let entries = timeline_entries_for_peer(peer).unwrap_or_else(|code| print_error_marker(code));
    let Some(entry) = entries.into_iter().find(|v| v.id == id) else {
        print_error_marker("timeline_item_missing");
    };
    timeline_emit_item(&entry);
}

fn timeline_clear(peer: &str, confirm: bool) {
    if !require_unlocked("timeline_clear") {
        return;
    }
    if !confirm {
        emit_marker(
            "error",
            Some("timeline_clear_confirm_required"),
            &[("peer", peer), ("reason", "explicit_confirm_required")],
        );
        print_error_marker("timeline_clear_confirm_required");
    }
    if !channel_label_ok(peer) {
        print_error_marker("timeline_peer_invalid");
    }
    let mut store = timeline_store_load().unwrap_or_else(|code| print_error_marker(code));
    let removed = store.peers.remove(peer).map(|v| v.len()).unwrap_or(0usize);
    timeline_store_save(&store).unwrap_or_else(|code| print_error_marker(code));
    let removed_s = removed.to_string();
    emit_marker(
        "timeline_clear",
        None,
        &[
            ("ok", "true"),
            ("peer", peer),
            ("removed", removed_s.as_str()),
        ],
    );
}

fn hs_seed_from_env() -> Option<u64> {
    env::var("QSC_HANDSHAKE_SEED")
        .ok()?
        .trim()
        .parse::<u64>()
        .ok()
}

fn hs_rand_bytes(label: &str, len: usize) -> Vec<u8> {
    if let Some(seed) = hs_seed_from_env() {
        let c = StdCrypto;
        let seed_bytes = seed.to_le_bytes();
        let seed_hash = c.sha512(&seed_bytes);
        let mut seed_key = [0u8; 32];
        seed_key.copy_from_slice(&seed_hash[..32]);
        return c.kmac256(&seed_key, label, b"", len);
    }
    let mut out = vec![0u8; len];
    let mut rng = OsRng;
    rng.fill_bytes(&mut out);
    out
}

fn hs_session_id(label: &str) -> [u8; 16] {
    let bytes = hs_rand_bytes(label, 16);
    let mut sid = [0u8; 16];
    sid.copy_from_slice(&bytes[..16]);
    sid
}

fn hs_transcript_mac(pq_init_ss: &[u8; 32], a1: &[u8], b1_no_mac: &[u8]) -> [u8; 32] {
    let c = StdCrypto;
    let mut data = Vec::with_capacity(a1.len() + b1_no_mac.len());
    data.extend_from_slice(a1);
    data.extend_from_slice(b1_no_mac);
    kmac_out::<32>(&c, pq_init_ss, "QSC.HS.TRANSCRIPT", &data)
}

fn hs_transcript_hash(pq_init_ss: &[u8; 32], a1: &[u8], b1_no_mac: &[u8]) -> [u8; 32] {
    let c = StdCrypto;
    let mut data = Vec::with_capacity(a1.len() + b1_no_mac.len());
    data.extend_from_slice(a1);
    data.extend_from_slice(b1_no_mac);
    kmac_out::<32>(&c, pq_init_ss, "QSC.HS.TRANSCRIPT.H", &data)
}

fn hs_pq_init_ss(ss_pq: &[u8], session_id: &[u8; 16]) -> [u8; 32] {
    let c = StdCrypto;
    let mut data = Vec::with_capacity(16 + 1);
    data.extend_from_slice(session_id);
    data.push(0x01);
    kmac_out::<32>(&c, ss_pq, "QSC.HS.PQ", &data)
}

fn hs_ephemeral_keypair() -> ([u8; 32], [u8; 32]) {
    let c = StdCrypto;
    let (sk, pk) = c.keypair();
    (sk.0, pk.0)
}

fn hs_dh_init_from_shared(dh_shared: &[u8; 32], session_id: &[u8; 16]) -> [u8; 32] {
    let c = StdCrypto;
    let mut data = Vec::with_capacity(16 + 1);
    data.extend_from_slice(session_id);
    data.push(0x02);
    kmac_out::<32>(&c, dh_shared, "QSC.HS.DHINIT", &data)
}

fn hs_dh_shared(self_sk: &[u8], peer_pub: &[u8]) -> Result<[u8; 32], &'static str> {
    if self_sk.len() != 32 || peer_pub.len() != 32 {
        return Err("handshake_dh_len");
    }
    let mut sk = [0u8; 32];
    sk.copy_from_slice(self_sk);
    let mut pk = [0u8; 32];
    pk.copy_from_slice(peer_pub);
    let c = StdCrypto;
    Ok(c.dh(&X25519Priv(sk), &X25519Pub(pk)))
}

fn hs_dh_pub_from_bytes(bytes: &[u8]) -> Result<[u8; 32], &'static str> {
    if bytes.len() != 32 {
        return Err("handshake_dh_len");
    }
    let mut out = [0u8; 32];
    out.copy_from_slice(bytes);
    Ok(out)
}

fn hs_dh_pub_is_all_zero(dh_pub: &[u8; 32]) -> bool {
    dh_pub.iter().all(|b| *b == 0)
}

fn hs_confirm_key(pq_init_ss: &[u8; 32], session_id: &[u8; 16], th: &[u8; 32]) -> [u8; 32] {
    let c = StdCrypto;
    let mut data = Vec::with_capacity(16 + 32);
    data.extend_from_slice(session_id);
    data.extend_from_slice(th);
    kmac_out::<32>(&c, pq_init_ss, "QSC.HS.CONFIRM", &data)
}

fn hs_confirm_mac(k_confirm: &[u8; 32], session_id: &[u8; 16], th: &[u8; 32]) -> [u8; 32] {
    let c = StdCrypto;
    let mut data = Vec::with_capacity(16 + 32 + 2);
    data.extend_from_slice(session_id);
    data.extend_from_slice(th);
    data.extend_from_slice(b"A2");
    kmac_out::<32>(&c, k_confirm, "QSC.HS.A2", &data)
}

fn hs_sig_fingerprint(sig_pk: &[u8]) -> String {
    let c = StdCrypto;
    let hash = c.sha512(sig_pk);
    format!("{}{}", IDENTITY_FP_PREFIX, hex_encode(&hash[..16]))
}

fn hs_sig_msg_b1(session_id: &[u8; 16], th: &[u8; 32]) -> Vec<u8> {
    let mut data = Vec::with_capacity(4 + 2 + 1 + 16 + 32);
    data.extend_from_slice(b"QSC.HS.SIG.B1");
    data.extend_from_slice(session_id);
    data.extend_from_slice(th);
    data
}

fn hs_sig_msg_a2(session_id: &[u8; 16], th: &[u8; 32], cmac: &[u8; 32]) -> Vec<u8> {
    let mut data = Vec::with_capacity(4 + 2 + 1 + 16 + 32 + 32);
    data.extend_from_slice(b"QSC.HS.SIG.A2");
    data.extend_from_slice(session_id);
    data.extend_from_slice(th);
    data.extend_from_slice(cmac);
    data
}

fn hs_sig_verify(sig_pk: &[u8], msg: &[u8], sig: &[u8], reason: &str) -> Result<(), &'static str> {
    let c = StdCrypto;
    match c.verify(sig_pk, msg, sig) {
        Ok(true) => {
            emit_marker(
                "sig_status",
                None,
                &[("ok", "true"), ("alg", "ML-DSA-65"), ("reason", reason)],
            );
            Ok(())
        }
        Ok(false) => {
            emit_marker(
                "sig_status",
                Some("sig_invalid"),
                &[("ok", "false"), ("alg", "ML-DSA-65"), ("reason", reason)],
            );
            Err("sig_invalid")
        }
        Err(_) => {
            emit_marker(
                "sig_status",
                Some("sig_invalid"),
                &[("ok", "false"), ("alg", "ML-DSA-65"), ("reason", reason)],
            );
            Err("sig_invalid")
        }
    }
}

fn hs_build_session(
    role_is_a: bool,
    session_id: [u8; 16],
    dh_init: [u8; 32],
    pq_init_ss: [u8; 32],
    dh_self_pub: [u8; 32],
    dh_peer_pub: [u8; 32],
) -> Result<Suite2SessionState, &'static str> {
    let c = StdCrypto;
    init_from_base_handshake(
        &c,
        role_is_a,
        SUITE2_PROTOCOL_VERSION,
        SUITE2_SUITE_ID,
        &session_id,
        &dh_init,
        &pq_init_ss,
        &dh_self_pub,
        &dh_peer_pub,
        true,
    )
}

fn hs_pending_legacy_path(dir: &Path, self_label: &str, peer: &str) -> PathBuf {
    dir.join(format!("handshake_pending_{}_{}.json", self_label, peer))
}

fn hs_pending_secret_key(self_label: &str, peer: &str) -> String {
    format!("handshake.pending.{}.{}", self_label, peer)
}

fn hs_pending_load(self_label: &str, peer: &str) -> Result<Option<HandshakePending>, ErrorCode> {
    let secret_key = hs_pending_secret_key(self_label, peer);
    match vault::secret_get(&secret_key) {
        Ok(Some(v)) if !v.is_empty() => {
            let pending: HandshakePending =
                serde_json::from_str(&v).map_err(|_| ErrorCode::ParseFailed)?;
            return Ok(Some(pending));
        }
        Ok(_) => {}
        Err("vault_missing" | "vault_locked") => return Err(ErrorCode::IdentitySecretUnavailable),
        Err(_) => return Err(ErrorCode::IoReadFailed),
    }

    // Legacy plaintext pending-file migration path: read once, re-store encrypted, then delete file.
    let (dir, source) = config_dir()?;
    let path = hs_pending_legacy_path(&dir, self_label, peer);
    if !path.exists() {
        return Ok(None);
    }
    enforce_safe_parents(&path, source)?;
    let bytes = fs::read(&path).map_err(|_| ErrorCode::IoReadFailed)?;
    let pending: HandshakePending =
        serde_json::from_slice(&bytes).map_err(|_| ErrorCode::ParseFailed)?;
    let v = serde_json::to_string(&pending).map_err(|_| ErrorCode::IoWriteFailed)?;
    match vault::secret_set(&secret_key, &v) {
        Ok(()) => {
            let _ = fs::remove_file(&path);
        }
        Err("vault_missing" | "vault_locked") => return Err(ErrorCode::IdentitySecretUnavailable),
        Err(_) => return Err(ErrorCode::IoWriteFailed),
    }
    Ok(Some(pending))
}

fn hs_pending_store(pending: &HandshakePending) -> Result<(), ErrorCode> {
    let key = hs_pending_secret_key(&pending.self_label, &pending.peer);
    let value = serde_json::to_string(pending).map_err(|_| ErrorCode::IoWriteFailed)?;
    match vault::secret_set(&key, &value) {
        Ok(()) => Ok(()),
        Err("vault_missing" | "vault_locked") => Err(ErrorCode::IdentitySecretUnavailable),
        Err(_) => Err(ErrorCode::IoWriteFailed),
    }
}

fn hs_pending_clear(self_label: &str, peer: &str) -> Result<(), ErrorCode> {
    let key = hs_pending_secret_key(self_label, peer);
    match vault::secret_set(&key, "") {
        Ok(()) => {}
        Err("vault_missing" | "vault_locked") => return Err(ErrorCode::IdentitySecretUnavailable),
        Err(_) => return Err(ErrorCode::IoWriteFailed),
    }
    // Best-effort legacy plaintext cleanup.
    let (dir, source) = config_dir()?;
    let path = hs_pending_legacy_path(&dir, self_label, peer);
    enforce_safe_parents(&path, source)?;
    let _ = fs::remove_file(path);
    Ok(())
}

fn handshake_status(peer: Option<&str>) {
    if !require_unlocked("handshake_status") {
        return;
    }
    let peer_label = peer.unwrap_or("peer-0");
    if let Err(code) = enforce_peer_not_blocked(peer_label) {
        print_error_marker(code);
    }
    let (peer_fp, pinned) = identity_peer_status(peer_label);
    let pinned_s = if pinned { "true" } else { "false" };
    let (send_ready, send_ready_reason) = qsp_send_ready_tuple(peer_label);
    let send_ready_s = if send_ready { "yes" } else { "no" };
    match qsp_session_load(peer_label) {
        Ok(Some(_)) => {
            if send_ready {
                emit_marker(
                    "handshake_status",
                    None,
                    &[
                        ("status", "established"),
                        ("peer", peer_label),
                        ("peer_fp", peer_fp.as_str()),
                        ("pinned", pinned_s),
                        ("send_ready", send_ready_s),
                    ],
                );
            } else {
                emit_marker(
                    "handshake_status",
                    None,
                    &[
                        ("status", "established_recv_only"),
                        ("peer", peer_label),
                        ("peer_fp", peer_fp.as_str()),
                        ("pinned", pinned_s),
                        ("send_ready", send_ready_s),
                        ("send_ready_reason", send_ready_reason),
                    ],
                );
            }
        }
        Ok(None) => {
            emit_marker(
                "handshake_status",
                None,
                &[
                    ("status", "no_session"),
                    ("peer", peer_label),
                    ("peer_fp", peer_fp.as_str()),
                    ("pinned", pinned_s),
                    ("send_ready", send_ready_s),
                    ("send_ready_reason", send_ready_reason),
                ],
            );
        }
        Err(_) => {
            emit_marker(
                "handshake_status",
                Some("handshake_status_failed"),
                &[
                    ("peer", peer_label),
                    ("peer_fp", peer_fp.as_str()),
                    ("pinned", pinned_s),
                    ("send_ready", send_ready_s),
                    ("send_ready_reason", send_ready_reason),
                ],
            );
        }
    }
}

fn perform_handshake_init_with_route(
    self_label: &str,
    peer: &str,
    relay: &str,
    route_token: &str,
) -> Result<(), &'static str> {
    enforce_peer_not_blocked(peer)?;
    let IdentityKeypair {
        kem_pk,
        kem_sk,
        sig_pk,
        sig_sk: _,
    } = identity_self_kem_keypair(self_label).map_err(|e| e.as_str())?;
    let sid = hs_session_id("QSC.HS.SID");
    let (dh_sk, dh_pub) = hs_ephemeral_keypair();
    let msg = HsInit {
        session_id: sid,
        kem_pk: kem_pk.clone(),
        sig_pk: sig_pk.clone(),
        dh_pub,
    };
    let bytes = hs_encode_init(&msg);
    if bytes.is_empty() {
        return Err("handshake_init_encode_failed");
    }
    let pending = HandshakePending {
        self_label: self_label.to_string(),
        peer: peer.to_string(),
        session_id: sid,
        kem_sk,
        kem_pk,
        dh_sk: dh_sk.to_vec(),
        dh_pub: dh_pub.to_vec(),
        sig_pk,
        peer_sig_fp: None,
        peer_sig_pk: None,
        peer_fp: None,
        role: "initiator".to_string(),
        confirm_key: None,
        transcript_hash: None,
        pending_session: None,
    };
    hs_pending_store(&pending).map_err(|_| "handshake_pending_store_failed")?;
    emit_marker(
        "handshake_start",
        None,
        &[("role", "initiator"), ("peer", peer)],
    );
    let size_s = bytes.len().to_string();
    let pk_len_s = hs_kem_pk_len().to_string();
    let sig_pk_len_s = hs_sig_pk_len().to_string();
    emit_marker(
        "handshake_send",
        None,
        &[
            ("msg", "A1"),
            ("size", size_s.as_str()),
            ("kem_pk_len", pk_len_s.as_str()),
            ("sig_pk_len", sig_pk_len_s.as_str()),
        ],
    );
    relay_inbox_push(relay, route_token, &bytes)?;
    Ok(())
}

fn handshake_init_with_route(self_label: &str, peer: &str, relay: &str, route_token: &str) {
    if !require_unlocked("handshake_init") {
        return;
    }
    if let Err(code) = perform_handshake_init_with_route(self_label, peer, relay, route_token) {
        print_error_marker(code);
    }
}

fn handshake_init(self_label: &str, peer: &str, relay: &str) {
    if !vault_unlocked() {
        require_unlocked("handshake_init");
    }
    let peer_channel = resolve_peer_device_target(peer, false)
        .map(|v| v.channel)
        .unwrap_or_else(|_| peer.to_string());
    let route_token = relay_peer_route_token(peer).unwrap_or_else(|code| print_error_marker(code));
    handshake_init_with_route(
        self_label,
        peer_channel.as_str(),
        relay,
        route_token.as_str(),
    );
}

fn perform_handshake_poll_with_tokens(
    self_label: &str,
    peer: &str,
    relay: &str,
    inbox_route_token: &str,
    peer_route_token: &str,
    max: usize,
) -> Result<(), &'static str> {
    enforce_peer_not_blocked(peer)?;
    let items = match relay_inbox_pull(relay, inbox_route_token, max) {
        Ok(v) => v,
        Err(code) => {
            emit_marker("handshake_recv", Some(code), &[("ok", "false")]);
            return Err(code);
        }
    };
    if items.is_empty() {
        emit_marker("handshake_recv", None, &[("msg", "none"), ("ok", "true")]);
        return Ok(());
    }

    if let Some(pending) = hs_pending_load(self_label, peer).map_err(|e| e.as_str())? {
        emit_marker(
            "handshake_pending",
            None,
            &[
                ("peer", peer),
                ("present", "true"),
                ("role", pending.role.as_str()),
            ],
        );
        if pending.role == "initiator" {
            // Initiator finalize: expect HS2
            for item in items {
                match hs_decode_resp(&item.data) {
                    Ok(resp) => {
                        if resp.session_id != pending.session_id {
                            emit_marker(
                                "handshake_reject",
                                None,
                                &[("reason", "session_id_mismatch")],
                            );
                            continue;
                        }
                        let c = StdCrypto;
                        let ss_pq = match c.decap(&pending.kem_sk, &resp.kem_ct) {
                            Ok(v) => v,
                            Err(_) => {
                                emit_marker(
                                    "handshake_reject",
                                    None,
                                    &[("reason", "pq_decap_failed")],
                                );
                                return Ok(());
                            }
                        };
                        let pq_init_ss = hs_pq_init_ss(&ss_pq, &resp.session_id);
                        if hs_dh_pub_is_all_zero(&resp.dh_pub) {
                            emit_marker("handshake_reject", None, &[("reason", "dh_pub_invalid")]);
                            return Ok(());
                        }
                        let dh_self_pub = match hs_dh_pub_from_bytes(&pending.dh_pub) {
                            Ok(v) => v,
                            Err(_) => {
                                emit_marker("handshake_reject", None, &[("reason", "dh_missing")]);
                                return Ok(());
                            }
                        };
                        let dh_shared = match hs_dh_shared(&pending.dh_sk, &resp.dh_pub) {
                            Ok(v) => v,
                            Err(_) => {
                                emit_marker("handshake_reject", None, &[("reason", "dh_failed")]);
                                return Ok(());
                            }
                        };
                        let dh_init_arr = hs_dh_init_from_shared(&dh_shared, &resp.session_id);
                        let dh_peer_pub = resp.dh_pub;
                        let a1 = hs_encode_init(&HsInit {
                            session_id: pending.session_id,
                            kem_pk: pending.kem_pk.clone(),
                            sig_pk: pending.sig_pk.clone(),
                            dh_pub: dh_self_pub,
                        });
                        let b1_no_auth = {
                            let mut tmp = Vec::with_capacity(
                                4 + 2 + 1 + 16 + hs_kem_ct_len() + hs_sig_pk_len(),
                            );
                            tmp.extend_from_slice(HS_MAGIC);
                            tmp.extend_from_slice(&HS_VERSION.to_be_bytes());
                            tmp.push(HS_TYPE_RESP);
                            tmp.extend_from_slice(&resp.session_id);
                            tmp.extend_from_slice(&resp.kem_ct);
                            tmp.extend_from_slice(&resp.sig_pk);
                            tmp.extend_from_slice(&resp.dh_pub);
                            tmp
                        };
                        let mac = hs_transcript_mac(&pq_init_ss, &a1, &b1_no_auth);
                        if mac != resp.mac {
                            emit_marker("handshake_reject", None, &[("reason", "bad_transcript")]);
                            return Ok(());
                        }
                        let th = hs_transcript_hash(&pq_init_ss, &a1, &b1_no_auth);
                        let sig_msg = hs_sig_msg_b1(&resp.session_id, &th);
                        if hs_sig_verify(&resp.sig_pk, &sig_msg, &resp.sig, "b1_verify").is_err() {
                            emit_marker("handshake_reject", None, &[("reason", "sig_invalid")]);
                            return Ok(());
                        }
                        let sig_fp = hs_sig_fingerprint(&resp.sig_pk);
                        match identity_read_sig_pin(peer) {
                            Ok(Some(pinned)) => {
                                if !identity_pin_matches_seen(pinned.as_str(), sig_fp.as_str()) {
                                    emit_peer_mismatch(peer, pinned.as_str(), sig_fp.as_str());
                                    emit_marker(
                                        "handshake_reject",
                                        None,
                                        &[("reason", "peer_mismatch")],
                                    );
                                    return Ok(());
                                }
                                emit_marker(
                                    "identity_ok",
                                    None,
                                    &[
                                        ("peer", peer),
                                        ("fp", identity_marker_display(sig_fp.as_str()).as_str()),
                                    ],
                                );
                            }
                            Ok(None) => emit_marker(
                                "identity_unknown",
                                None,
                                &[
                                    ("peer", peer),
                                    ("seen_fp", identity_marker_display(sig_fp.as_str()).as_str()),
                                ],
                            ),
                            Err(_) => {
                                emit_marker(
                                    "handshake_reject",
                                    None,
                                    &[("reason", "identity_pin_failed")],
                                );
                                return Ok(());
                            }
                        }
                        let st = match hs_build_session(
                            true,
                            pending.session_id,
                            dh_init_arr,
                            pq_init_ss,
                            dh_self_pub,
                            dh_peer_pub,
                        ) {
                            Ok(v) => v,
                            Err(_) => {
                                emit_marker(
                                    "handshake_reject",
                                    None,
                                    &[("reason", "session_init_failed")],
                                );
                                return Ok(());
                            }
                        };
                        qsp_session_store(peer, &st)
                            .map_err(|_| "handshake_session_store_failed")?;
                        let _ = hs_pending_clear(self_label, peer);
                        let k_confirm = hs_confirm_key(&pq_init_ss, &resp.session_id, &th);
                        let cmac = hs_confirm_mac(&k_confirm, &resp.session_id, &th);
                        let sig_sk = identity_self_kem_keypair(self_label)
                            .map_err(|e| e.as_str())?
                            .sig_sk;
                        let a2_sig_msg = hs_sig_msg_a2(&resp.session_id, &th, &cmac);
                        let a2_sig = match c.sign(&sig_sk, &a2_sig_msg) {
                            Ok(v) => v,
                            Err(_) => {
                                emit_marker(
                                    "handshake_reject",
                                    None,
                                    &[("reason", "sig_sign_failed")],
                                );
                                return Ok(());
                            }
                        };
                        emit_marker(
                            "sig_status",
                            None,
                            &[("ok", "true"), ("alg", "ML-DSA-65"), ("reason", "a2_sign")],
                        );
                        let confirm = HsConfirm {
                            session_id: resp.session_id,
                            mac: cmac,
                            sig: a2_sig,
                        };
                        let cbytes = hs_encode_confirm(&confirm);
                        let size_s = cbytes.len().to_string();
                        emit_marker(
                            "handshake_send",
                            None,
                            &[("msg", "A2"), ("size", size_s.as_str())],
                        );
                        relay_inbox_push(relay, peer_route_token, &cbytes)?;
                        emit_marker(
                            "handshake_complete",
                            None,
                            &[("peer", peer), ("role", "initiator")],
                        );
                        return Ok(());
                    }
                    Err(_) => {
                        emit_marker("handshake_reject", None, &[("reason", "decode_failed")]);
                        continue;
                    }
                }
            }
            return Ok(());
        }
        if pending.role == "responder" {
            // Responder confirm: expect A2
            for item in items {
                match hs_decode_confirm(&item.data) {
                    Ok(confirm) => {
                        if confirm.session_id != pending.session_id {
                            emit_marker(
                                "handshake_reject",
                                None,
                                &[("reason", "session_id_mismatch")],
                            );
                            continue;
                        }
                        let Some(k_confirm) = pending.confirm_key else {
                            emit_marker(
                                "handshake_reject",
                                None,
                                &[("reason", "missing_confirm_key")],
                            );
                            continue;
                        };
                        let Some(th) = pending.transcript_hash else {
                            emit_marker(
                                "handshake_reject",
                                None,
                                &[("reason", "missing_transcript")],
                            );
                            continue;
                        };
                        let expect = hs_confirm_mac(&k_confirm, &confirm.session_id, &th);
                        if expect != confirm.mac {
                            emit_marker("handshake_recv", None, &[("msg", "A2"), ("ok", "false")]);
                            emit_marker("handshake_reject", None, &[("reason", "bad_confirm")]);
                            continue;
                        }
                        let Some(peer_sig_pk) = pending.peer_sig_pk.as_ref() else {
                            emit_marker(
                                "handshake_reject",
                                None,
                                &[("reason", "identity_missing")],
                            );
                            continue;
                        };
                        let sig_msg = hs_sig_msg_a2(&confirm.session_id, &th, &confirm.mac);
                        if hs_sig_verify(peer_sig_pk, &sig_msg, &confirm.sig, "a2_verify").is_err()
                        {
                            emit_marker("handshake_reject", None, &[("reason", "sig_invalid")]);
                            continue;
                        }
                        emit_marker("handshake_recv", None, &[("msg", "A2"), ("ok", "true")]);
                        let Some(ref pending_bytes) = pending.pending_session else {
                            emit_marker("handshake_reject", None, &[("reason", "missing_session")]);
                            continue;
                        };
                        let st = match Suite2SessionState::restore_bytes(pending_bytes) {
                            Ok(v) => v,
                            Err(_) => {
                                emit_marker(
                                    "handshake_reject",
                                    None,
                                    &[("reason", "session_restore_failed")],
                                );
                                continue;
                            }
                        };
                        let Some(peer_fp) = pending.peer_fp.as_ref() else {
                            emit_marker(
                                "handshake_reject",
                                None,
                                &[("reason", "identity_missing")],
                            );
                            continue;
                        };
                        let Some(peer_sig_fp) = pending.peer_sig_fp.as_ref() else {
                            emit_marker(
                                "handshake_reject",
                                None,
                                &[("reason", "identity_missing")],
                            );
                            continue;
                        };
                        match identity_read_pin(peer) {
                            Ok(None) => emit_marker(
                                "identity_unknown",
                                None,
                                &[
                                    ("peer", peer),
                                    (
                                        "seen_fp",
                                        identity_marker_display(peer_fp.as_str()).as_str(),
                                    ),
                                ],
                            ),
                            Ok(Some(pinned)) => {
                                if !identity_pin_matches_seen(pinned.as_str(), peer_fp.as_str()) {
                                    emit_peer_mismatch(peer, pinned.as_str(), peer_fp.as_str());
                                    emit_marker(
                                        "handshake_reject",
                                        None,
                                        &[("reason", "peer_mismatch")],
                                    );
                                    continue;
                                }
                                emit_marker(
                                    "identity_ok",
                                    None,
                                    &[
                                        ("peer", peer),
                                        ("fp", identity_marker_display(peer_fp.as_str()).as_str()),
                                    ],
                                );
                            }
                            Err(_) => {
                                emit_marker(
                                    "handshake_reject",
                                    None,
                                    &[("reason", "identity_pin_failed")],
                                );
                                continue;
                            }
                        }
                        match identity_read_sig_pin(peer) {
                            Ok(None) => emit_marker(
                                "identity_unknown",
                                None,
                                &[
                                    ("peer", peer),
                                    (
                                        "seen_fp",
                                        identity_marker_display(peer_sig_fp.as_str()).as_str(),
                                    ),
                                ],
                            ),
                            Ok(Some(pinned)) => {
                                if !identity_pin_matches_seen(pinned.as_str(), peer_sig_fp.as_str())
                                {
                                    emit_peer_mismatch(peer, pinned.as_str(), peer_sig_fp.as_str());
                                    emit_marker(
                                        "handshake_reject",
                                        None,
                                        &[("reason", "peer_mismatch")],
                                    );
                                    continue;
                                }
                                emit_marker(
                                    "identity_ok",
                                    None,
                                    &[
                                        ("peer", peer),
                                        (
                                            "fp",
                                            identity_marker_display(peer_sig_fp.as_str()).as_str(),
                                        ),
                                    ],
                                );
                            }
                            Err(_) => {
                                emit_marker(
                                    "handshake_reject",
                                    None,
                                    &[("reason", "identity_pin_failed")],
                                );
                                continue;
                            }
                        }
                        qsp_session_store(peer, &st)
                            .map_err(|_| "handshake_session_store_failed")?;
                        let _ = hs_pending_clear(self_label, peer);
                        emit_marker(
                            "handshake_complete",
                            None,
                            &[("peer", peer), ("role", "responder")],
                        );
                        return Ok(());
                    }
                    Err(_) => {
                        emit_marker("handshake_reject", None, &[("reason", "decode_failed")]);
                        continue;
                    }
                }
            }
            return Ok(());
        }
    }

    emit_marker(
        "handshake_pending",
        None,
        &[("peer", peer), ("present", "false"), ("role", "none")],
    );

    // Responder: process HS1 and send HS2
    for item in items {
        match hs_decode_init(&item.data) {
            Ok(init) => {
                if hs_dh_pub_is_all_zero(&init.dh_pub) {
                    emit_marker("handshake_reject", None, &[("reason", "dh_pub_invalid")]);
                    continue;
                }
                let peer_fp = identity_fingerprint_from_pk(&init.kem_pk);
                let peer_sig_fp = hs_sig_fingerprint(&init.sig_pk);
                match identity_read_pin(peer) {
                    Ok(Some(pinned)) => {
                        if !identity_pin_matches_seen(pinned.as_str(), peer_fp.as_str()) {
                            emit_peer_mismatch(peer, pinned.as_str(), peer_fp.as_str());
                            emit_marker("handshake_reject", None, &[("reason", "peer_mismatch")]);
                            continue;
                        }
                    }
                    Ok(None) => {}
                    Err(_) => {
                        emit_marker(
                            "handshake_reject",
                            None,
                            &[("reason", "identity_pin_failed")],
                        );
                        continue;
                    }
                }
                match identity_read_sig_pin(peer) {
                    Ok(Some(pinned)) => {
                        if !identity_pin_matches_seen(pinned.as_str(), peer_sig_fp.as_str()) {
                            emit_peer_mismatch(peer, pinned.as_str(), peer_sig_fp.as_str());
                            emit_marker("handshake_reject", None, &[("reason", "peer_mismatch")]);
                            continue;
                        }
                    }
                    Ok(None) => {}
                    Err(_) => {
                        emit_marker(
                            "handshake_reject",
                            None,
                            &[("reason", "identity_pin_failed")],
                        );
                        continue;
                    }
                }
                let c = StdCrypto;
                let (kem_ct, ss_pq) = match c.encap(&init.kem_pk) {
                    Ok(v) => v,
                    Err(_) => {
                        emit_marker("handshake_reject", None, &[("reason", "pq_encap_failed")]);
                        continue;
                    }
                };
                let pq_init_ss = hs_pq_init_ss(&ss_pq, &init.session_id);
                let (dh_sk, dh_self_pub) = hs_ephemeral_keypair();
                let dh_shared = match hs_dh_shared(&dh_sk, &init.dh_pub) {
                    Ok(v) => v,
                    Err(_) => {
                        emit_marker("handshake_reject", None, &[("reason", "dh_failed")]);
                        continue;
                    }
                };
                let dh_init_arr = hs_dh_init_from_shared(&dh_shared, &init.session_id);
                let dh_peer_pub = init.dh_pub;
                let st = match hs_build_session(
                    false,
                    init.session_id,
                    dh_init_arr,
                    pq_init_ss,
                    dh_self_pub,
                    dh_peer_pub,
                ) {
                    Ok(v) => v,
                    Err(_) => {
                        emit_marker(
                            "handshake_reject",
                            None,
                            &[("reason", "session_init_failed")],
                        );
                        continue;
                    }
                };
                let a1 = hs_encode_init(&init);
                let self_sig = match identity_self_kem_keypair(self_label) {
                    Ok(k) => (k.sig_pk, k.sig_sk),
                    Err(_) => {
                        emit_marker("handshake_reject", None, &[("reason", "identity_missing")]);
                        continue;
                    }
                };
                let (self_sig_pk, self_sig_sk) = self_sig;
                let b1_no_auth = {
                    let mut tmp =
                        Vec::with_capacity(4 + 2 + 1 + 16 + hs_kem_ct_len() + hs_sig_pk_len());
                    tmp.extend_from_slice(HS_MAGIC);
                    tmp.extend_from_slice(&HS_VERSION.to_be_bytes());
                    tmp.push(HS_TYPE_RESP);
                    tmp.extend_from_slice(&init.session_id);
                    tmp.extend_from_slice(&kem_ct);
                    tmp.extend_from_slice(&self_sig_pk);
                    tmp.extend_from_slice(&dh_self_pub);
                    tmp
                };
                let mac = hs_transcript_mac(&pq_init_ss, &a1, &b1_no_auth);
                let th = hs_transcript_hash(&pq_init_ss, &a1, &b1_no_auth);
                let sig_msg = hs_sig_msg_b1(&init.session_id, &th);
                let sig = match c.sign(&self_sig_sk, &sig_msg) {
                    Ok(v) => v,
                    Err(_) => {
                        emit_marker("handshake_reject", None, &[("reason", "sig_sign_failed")]);
                        continue;
                    }
                };
                emit_marker(
                    "sig_status",
                    None,
                    &[("ok", "true"), ("alg", "ML-DSA-65"), ("reason", "b1_sign")],
                );
                let k_confirm = hs_confirm_key(&pq_init_ss, &init.session_id, &th);
                let pending = HandshakePending {
                    self_label: self_label.to_string(),
                    peer: peer.to_string(),
                    session_id: init.session_id,
                    kem_sk: Vec::new(),
                    kem_pk: Vec::new(),
                    dh_sk: dh_sk.to_vec(),
                    dh_pub: dh_self_pub.to_vec(),
                    sig_pk: Vec::new(),
                    peer_fp: Some(peer_fp),
                    peer_sig_fp: Some(peer_sig_fp),
                    peer_sig_pk: Some(init.sig_pk.clone()),
                    role: "responder".to_string(),
                    confirm_key: Some(k_confirm),
                    transcript_hash: Some(th),
                    pending_session: Some(st.snapshot_bytes()),
                };
                hs_pending_store(&pending).map_err(|_| "handshake_pending_store_failed")?;
                let resp = HsResp {
                    session_id: init.session_id,
                    kem_ct,
                    mac,
                    sig_pk: self_sig_pk,
                    sig,
                    dh_pub: dh_self_pub,
                };
                let bytes = hs_encode_resp(&resp);
                let size_s = bytes.len().to_string();
                let ct_len_s = hs_kem_ct_len().to_string();
                let sig_pk_len_s = hs_sig_pk_len().to_string();
                emit_marker(
                    "handshake_send",
                    None,
                    &[
                        ("msg", "B1"),
                        ("size", size_s.as_str()),
                        ("kem_ct_len", ct_len_s.as_str()),
                        ("sig_pk_len", sig_pk_len_s.as_str()),
                    ],
                );
                relay_inbox_push(relay, peer_route_token, &bytes)?;
                return Ok(());
            }
            Err(_) => {
                emit_marker("handshake_reject", None, &[("reason", "decode_failed")]);
                continue;
            }
        }
    }
    Ok(())
}

fn handshake_poll_with_tokens(
    self_label: &str,
    peer: &str,
    relay: &str,
    inbox_route_token: &str,
    peer_route_token: &str,
    max: usize,
) {
    if !require_unlocked("handshake_poll") {
        return;
    }
    if let Err(code) = perform_handshake_poll_with_tokens(
        self_label,
        peer,
        relay,
        inbox_route_token,
        peer_route_token,
        max,
    ) {
        print_error_marker(code);
    }
}

fn handshake_poll(self_label: &str, peer: &str, relay: &str, max: usize) {
    let peer_channel = resolve_peer_device_target(peer, false)
        .map(|v| v.channel)
        .unwrap_or_else(|_| peer.to_string());
    let inbox_route_token =
        relay_self_inbox_route_token().unwrap_or_else(|code| print_error_marker(code));
    let peer_route_token =
        relay_peer_route_token(peer).unwrap_or_else(|code| print_error_marker(code));
    handshake_poll_with_tokens(
        self_label,
        peer_channel.as_str(),
        relay,
        inbox_route_token.as_str(),
        peer_route_token.as_str(),
        max,
    );
}

fn send_execute(args: SendExecuteArgs) {
    if !require_unlocked("send") {
        return;
    }
    let SendExecuteArgs {
        transport,
        relay,
        to,
        file,
        pad_to,
        pad_bucket,
        bucket_max,
        meta_seed,
        receipt,
    } = args;
    let transport = match transport {
        Some(v) => v,
        None => print_error_marker("send_transport_required"),
    };

    match transport {
        SendTransport::Relay => {
            let relay = match relay {
                Some(v) => v,
                None => print_error_marker("send_relay_required"),
            };
            let to = match to {
                Some(v) => v,
                None => print_error_marker("send_to_required"),
            };
            let file = match file {
                Some(v) => v,
                None => print_error_marker("send_file_required"),
            };
            let pad_cfg = match meta_pad_config_from_args(pad_to, pad_bucket, meta_seed) {
                Ok(v) => v,
                Err(code) => print_error_marker(code),
            };
            if let Err(code) = enforce_cli_send_contact_trust(to.as_str()) {
                print_error_marker(code);
            }
            if let Err(code) = enforce_peer_not_blocked(to.as_str()) {
                print_error_marker(code);
            }
            if let Err(reason) = protocol_active_or_reason_for_send_peer(to.as_str()) {
                protocol_inactive_exit(reason.as_str());
            }
            if let Some(seed) = meta_seed {
                let seed_s = seed.to_string();
                emit_marker(
                    "meta_mode",
                    None,
                    &[("deterministic", "true"), ("seed", seed_s.as_str())],
                );
            }
            if receipt.is_none() {
                emit_marker("receipt_disabled", None, &[]);
            }
            relay_send(&to, &file, &relay, pad_cfg, bucket_max, meta_seed, receipt);
        }
    }
}

fn send_abort() {
    let (dir, source) = match config_dir() {
        Ok(v) => v,
        Err(e) => print_error(e),
    };
    let _lock = match lock_store_exclusive(&dir, source) {
        Ok(v) => v,
        Err(e) => print_error(e),
    };
    if let Err(e) = ensure_store_layout(&dir, source) {
        print_error(e);
    }

    let outbox_path = dir.join(OUTBOX_FILE_NAME);
    if let Err(e) = enforce_safe_parents(&outbox_path, source) {
        print_error(e);
    }

    if outbox_path.exists() {
        let outbox = outbox_record_load(&outbox_path).unwrap_or_else(|e| print_error_marker(e));
        if outbox.to.is_empty() {
            print_error_marker("outbox_recovery_required");
        }
        let next_state = outbox_next_state_load().unwrap_or_else(|e| print_error_marker(e));
        if qsp_session_store(
            outbox.channel.as_deref().unwrap_or(outbox.to.as_str()),
            &next_state,
        )
        .is_err()
        {
            print_error_marker("qsp_session_store_failed");
        }
        let next_seq = match read_send_state(&dir, source) {
            Ok(v) => v + 1,
            Err(()) => print_error_marker("send_state_parse_failed"),
        };
        let state_bytes = format!("send_seq={}\n", next_seq).into_bytes();
        if write_atomic(&dir.join(SEND_STATE_NAME), &state_bytes, source).is_err() {
            print_error_marker("send_commit_write_failed");
        }
        if fs::remove_file(&outbox_path).is_err() {
            print_error_marker("outbox_abort_failed");
        }
        if let Err(code) = outbox_next_state_clear() {
            print_error_marker(code);
        }
        let seq_s = next_seq.to_string();
        emit_marker(
            "outbox_abort",
            None,
            &[
                ("ok", "true"),
                ("action", "burned"),
                ("send_seq", seq_s.as_str()),
            ],
        );
    } else {
        let _ = outbox_next_state_clear();
        emit_marker(
            "outbox_abort",
            None,
            &[("ok", "true"), ("action", "absent")],
        );
    }
}

fn outbox_record_load(path: &Path) -> Result<OutboxRecord, &'static str> {
    let bytes = fs::read(path).map_err(|_| "outbox_read_failed")?;
    serde_json::from_slice(&bytes).map_err(|_| "outbox_parse_failed")
}

fn outbox_next_state_store(st: &Suite2SessionState) -> Result<(), &'static str> {
    let bytes = st.snapshot_bytes();
    let secret = hex_encode(&bytes);
    match vault::secret_set(OUTBOX_NEXT_STATE_SECRET_KEY, &secret) {
        Ok(()) => Ok(()),
        Err("vault_missing" | "vault_locked") => Err("outbox_state_vault_unavailable"),
        Err(_) => Err("outbox_state_store_failed"),
    }
}

fn outbox_next_state_load() -> Result<Suite2SessionState, &'static str> {
    let Some(secret) =
        vault::secret_get(OUTBOX_NEXT_STATE_SECRET_KEY).map_err(|_| "outbox_state_read_failed")?
    else {
        return Err("outbox_state_missing");
    };
    if secret.is_empty() {
        return Err("outbox_state_missing");
    }
    let bytes = hex_decode(secret.as_str()).map_err(|_| "outbox_state_parse_failed")?;
    Suite2SessionState::restore_bytes(&bytes).map_err(|_| "outbox_state_parse_failed")
}

fn outbox_next_state_clear() -> Result<(), &'static str> {
    match vault::secret_set(OUTBOX_NEXT_STATE_SECRET_KEY, "") {
        Ok(()) => Ok(()),
        Err("vault_missing" | "vault_locked") => Err("outbox_state_vault_unavailable"),
        Err(_) => Err("outbox_state_clear_failed"),
    }
}

struct ReceiveArgs {
    transport: Option<SendTransport>,
    relay: Option<String>,
    legacy_receive_mode: Option<LegacyReceiveMode>,
    attachment_service: Option<String>,
    from: Option<String>,
    mailbox: Option<String>,
    max: Option<usize>,
    max_file_size: Option<usize>,
    max_file_chunks: Option<usize>,
    out: Option<PathBuf>,
    deterministic_meta: bool,
    interval_ms: Option<u64>,
    poll_interval_ms: Option<u64>,
    poll_ticks: Option<u32>,
    batch_max_count: Option<u32>,
    poll_max_per_tick: Option<u32>,
    bucket_max: Option<usize>,
    meta_seed: Option<u64>,
    emit_receipts: Option<ReceiptKind>,
    receipt_mode: Option<ReceiptMode>,
    receipt_batch_window_ms: Option<u64>,
    receipt_jitter_ms: Option<u64>,
    file_confirm_mode: Option<FileConfirmMode>,
}

fn receive_execute(args: ReceiveArgs) {
    if !require_unlocked("receive") {
        return;
    }
    let ReceiveArgs {
        transport,
        relay,
        legacy_receive_mode,
        attachment_service,
        from,
        mailbox,
        max,
        max_file_size,
        max_file_chunks,
        out,
        deterministic_meta,
        interval_ms,
        poll_interval_ms,
        poll_ticks,
        batch_max_count,
        poll_max_per_tick,
        bucket_max,
        meta_seed,
        emit_receipts,
        receipt_mode,
        receipt_batch_window_ms,
        receipt_jitter_ms,
        file_confirm_mode,
    } = args;
    let receipt_policy = resolve_receipt_policy(ReceiptPolicyOverrides {
        emit_receipts,
        receipt_mode,
        receipt_batch_window_ms,
        receipt_jitter_ms,
        file_confirm_mode,
    });
    let batch_window_s = receipt_policy.batch_window_ms.to_string();
    let jitter_s = receipt_policy.jitter_ms.to_string();
    emit_marker(
        "receipt_policy",
        None,
        &[
            ("mode", receipt_policy.mode.as_str()),
            ("batch_window_ms", batch_window_s.as_str()),
            ("jitter_ms", jitter_s.as_str()),
            (
                "file_confirm_mode",
                receipt_policy.file_confirm_mode.as_str(),
            ),
        ],
    );
    let transport = match transport {
        Some(v) => v,
        None => print_error_marker("recv_transport_required"),
    };
    match transport {
        SendTransport::Relay => {
            let relay = match relay {
                Some(v) => v,
                None => print_error_marker("recv_relay_required"),
            };
            let attachment_service = attachment_service.map(|v| {
                normalize_relay_endpoint(v.as_str()).unwrap_or_else(|code| print_error_marker(code))
            });
            let legacy_receive_mode =
                resolve_legacy_receive_mode(legacy_receive_mode, attachment_service.as_deref())
                    .unwrap_or_else(|code| print_error_marker(code));
            if let Err(code) = normalize_relay_endpoint(relay.as_str()) {
                print_error_marker(code);
            }
            let from = match from {
                Some(v) => v,
                None => print_error_marker("recv_from_required"),
            };
            let mailbox = match mailbox {
                Some(raw) => normalize_route_token(raw.as_str())
                    .unwrap_or_else(|code| print_error_marker(code)),
                None => {
                    relay_self_inbox_route_token().unwrap_or_else(|code| print_error_marker(code))
                }
            };
            let max = match max {
                Some(v) if v > 0 => v,
                _ => print_error_marker("recv_max_required"),
            };
            let max_file_size = match max_file_size {
                Some(v) if v > 0 && v <= ATTACHMENT_DEFAULT_MAX_FILE_SIZE => v,
                Some(_) => print_error_marker("recv_file_size_bound_invalid"),
                None => {
                    if attachment_service.is_some() {
                        ATTACHMENT_DEFAULT_MAX_FILE_SIZE
                    } else {
                        FILE_XFER_DEFAULT_MAX_FILE_SIZE
                    }
                }
            };
            let max_file_chunks = match max_file_chunks {
                Some(v) if v > 0 && v <= ATTACHMENT_DEFAULT_MAX_PARTS => v,
                Some(_) => print_error_marker("recv_file_chunks_bound_invalid"),
                None => {
                    if attachment_service.is_some() {
                        ATTACHMENT_DEFAULT_MAX_PARTS
                    } else {
                        FILE_XFER_DEFAULT_MAX_CHUNKS
                    }
                }
            };
            let out = match out {
                Some(v) => v,
                None => print_error_marker("recv_out_required"),
            };
            let poll_cfg = match meta_poll_config_from_args(MetaPollArgs {
                deterministic_meta,
                interval_ms,
                poll_interval_ms,
                ticks: poll_ticks,
                batch_max_count,
                poll_max_per_tick,
                bucket_max,
                meta_seed,
            }) {
                Ok(v) => v,
                Err(code) => print_error_marker(code),
            };
            let source = ConfigSource::EnvOverride;
            if let Err(e) = ensure_dir_secure(&out, source) {
                print_error(e);
            }
            let (cfg_dir, cfg_source) = match config_dir() {
                Ok(v) => v,
                Err(e) => print_error(e),
            };
            if !check_symlink_safe(&cfg_dir) {
                print_error(ErrorCode::UnsafePathSymlink);
            }
            if !check_parent_safe(&cfg_dir, cfg_source) {
                print_error(ErrorCode::UnsafeParentPerms);
            }
            if let Err(reason) = protocol_active_or_reason_for_peer(from.as_str()) {
                protocol_inactive_exit(reason.as_str());
            }

            if let Some(seed) = meta_seed {
                let seed_s = seed.to_string();
                emit_marker(
                    "meta_mode",
                    None,
                    &[("deterministic", "true"), ("seed", seed_s.as_str())],
                );
            }
            let recv_max = poll_cfg.as_ref().map(|c| c.batch_max_count).unwrap_or(max);
            let max_s = recv_max.to_string();
            let mailbox_hash = route_token_hash8(mailbox.as_str());
            emit_marker(
                "recv_start",
                None,
                &[
                    ("transport", "relay"),
                    ("mailbox", "redacted"),
                    ("mailbox_hash", mailbox_hash.as_str()),
                    ("from", from.as_str()),
                    ("max", max_s.as_str()),
                ],
            );
            let mut total = 0usize;
            if let Some(cfg) = poll_cfg {
                let interval_s = cfg.interval_ms.to_string();
                let ticks_s = cfg.ticks.to_string();
                let max_tick_s = cfg.batch_max_count.to_string();
                let bucket_max_s = cfg.bucket_max.to_string();
                emit_marker(
                    "meta_poll_config",
                    None,
                    &[
                        ("interval_ms", interval_s.as_str()),
                        ("ticks", ticks_s.as_str()),
                        ("batch_max_count", max_tick_s.as_str()),
                        ("bucket_max", bucket_max_s.as_str()),
                    ],
                );
                for tick in 0..cfg.ticks {
                    let tick_s = tick.to_string();
                    let deterministic_s = if cfg.deterministic { "true" } else { "false" };
                    emit_marker(
                        "meta_tick",
                        None,
                        &[
                            ("tick", tick_s.as_str()),
                            ("interval_ms", interval_s.as_str()),
                            ("deterministic", deterministic_s),
                        ],
                    );
                    let pull = ReceivePullCtx {
                        relay: &relay,
                        legacy_receive_mode,
                        attachment_service: attachment_service.as_deref(),
                        mailbox: mailbox.as_str(),
                        from: &from,
                        out: &out,
                        source,
                        cfg_dir: &cfg_dir,
                        cfg_source,
                        bucket_max: cfg.bucket_max,
                        file_max_size: max_file_size,
                        file_max_chunks: max_file_chunks,
                        receipt_policy,
                    };
                    let stats = receive_pull_and_write(&pull, cfg.batch_max_count);
                    total = total.saturating_add(stats.count);
                    let count_s = stats.count.to_string();
                    let bytes_s = stats.bytes.to_string();
                    emit_marker(
                        "meta_batch",
                        None,
                        &[("count", count_s.as_str()), ("bytes", bytes_s.as_str())],
                    );
                    if !cfg.deterministic && cfg.interval_ms > 0 {
                        std::thread::sleep(Duration::from_millis(cfg.interval_ms));
                    }
                }
            } else {
                let pull = ReceivePullCtx {
                    relay: &relay,
                    legacy_receive_mode,
                    attachment_service: attachment_service.as_deref(),
                    mailbox: mailbox.as_str(),
                    from: &from,
                    out: &out,
                    source,
                    cfg_dir: &cfg_dir,
                    cfg_source,
                    bucket_max: META_BUCKET_MAX_DEFAULT,
                    file_max_size: max_file_size,
                    file_max_chunks: max_file_chunks,
                    receipt_policy,
                };
                total = receive_pull_and_write(&pull, max).count;
            }
            if total == 0 {
                emit_marker("recv_none", None, &[]);
                return;
            }
            let count_s = total.to_string();
            emit_marker("recv_commit", None, &[("count", count_s.as_str())]);
        }
    }
}

struct ReceivePullCtx<'a> {
    relay: &'a str,
    legacy_receive_mode: LegacyReceiveMode,
    attachment_service: Option<&'a str>,
    mailbox: &'a str,
    from: &'a str,
    out: &'a Path,
    source: ConfigSource,
    cfg_dir: &'a Path,
    cfg_source: ConfigSource,
    bucket_max: usize,
    file_max_size: usize,
    file_max_chunks: usize,
    receipt_policy: ReceiptPolicy,
}

struct ReceivePullStats {
    count: usize,
    bytes: usize,
}

fn receive_pull_and_write(ctx: &ReceivePullCtx<'_>, max: usize) -> ReceivePullStats {
    let items = match relay_inbox_pull(ctx.relay, ctx.mailbox, max) {
        Ok(v) => v,
        Err(code) => print_error_marker(code),
    };
    let mut stats = ReceivePullStats { count: 0, bytes: 0 };
    let mut pending_receipts: Vec<PendingReceipt> = Vec::new();
    for item in items {
        let envelope_len = item.data.len();
        match qsp_unpack_for_peer(ctx.from, &item.data) {
            Ok((outcome, channel)) => {
                let commit_unpack_state = || {
                    record_qsp_status(ctx.cfg_dir, ctx.cfg_source, true, "unpack_ok", false, true);
                    emit_marker("qsp_unpack", None, &[("ok", "true"), ("version", "5.0")]);
                    let msg_idx_s = outcome.msg_idx.to_string();
                    emit_marker(
                        "ratchet_recv_advance",
                        None,
                        &[("msg_idx", msg_idx_s.as_str())],
                    );
                    if outcome.skip_delta > 0 {
                        let sd = outcome.skip_delta.to_string();
                        emit_marker("ratchet_skip_store", None, &[("count", sd.as_str())]);
                    }
                    if outcome.evicted > 0 {
                        let ev = outcome.evicted.to_string();
                        emit_marker("ratchet_skip_evict", None, &[("count", ev.as_str())]);
                    }
                    if qsp_session_store(channel.as_str(), &outcome.next_state).is_err() {
                        emit_marker("error", Some("qsp_session_store_failed"), &[]);
                        print_error_marker("qsp_session_store_failed");
                    }
                };
                let mut payload = outcome.plaintext.clone();
                let mut request_receipt = false;
                let mut request_msg_id = String::new();
                if let Some(desc) = parse_attachment_descriptor_payload(&outcome.plaintext) {
                    let attachment_id = desc.attachment_id.clone();
                    match attachment_handle_descriptor(ctx, desc) {
                        Ok(Some((confirm_attachment_id, confirm_handle))) => {
                            commit_unpack_state();
                            queue_or_send_receipt(
                                ctx,
                                &mut pending_receipts,
                                PendingReceipt::AttachmentComplete {
                                    attachment_id: confirm_attachment_id,
                                    confirm_handle,
                                },
                            );
                        }
                        Ok(None) => {
                            commit_unpack_state();
                        }
                        Err(reason) => {
                            emit_marker(
                                "attachment_desc_reject",
                                Some(reason),
                                &[
                                    (
                                        "attachment_id",
                                        file_delivery_short_id(&attachment_id).as_str(),
                                    ),
                                    ("reason", reason),
                                ],
                            );
                            print_error_marker(reason);
                        }
                    }
                    continue;
                }
                if let Some(file_payload) = parse_file_transfer_payload(&outcome.plaintext) {
                    let file_id = match &file_payload {
                        FileTransferPayload::Chunk(v) => v.file_id.clone(),
                        FileTransferPayload::Manifest(v) => v.file_id.clone(),
                    };
                    if ctx.legacy_receive_mode == LegacyReceiveMode::Retired {
                        let payload_type = match &file_payload {
                            FileTransferPayload::Chunk(_) => "file_chunk",
                            FileTransferPayload::Manifest(_) => "file_manifest",
                        };
                        emit_marker(
                            "legacy_receive_reject",
                            Some("legacy_receive_retired_post_w0"),
                            &[
                                ("id", file_id.as_str()),
                                ("mode", legacy_receive_mode_name(ctx.legacy_receive_mode)),
                                ("payload_type", payload_type),
                                ("reason", "legacy_receive_retired_post_w0"),
                            ],
                        );
                        emit_marker(
                            "file_xfer_reject",
                            Some("legacy_receive_retired_post_w0"),
                            &[
                                ("id", file_id.as_str()),
                                ("reason", "legacy_receive_retired_post_w0"),
                            ],
                        );
                        print_error_marker("legacy_receive_retired_post_w0");
                    }
                    let file_res = match file_payload {
                        FileTransferPayload::Chunk(v) => {
                            file_transfer_handle_chunk(ctx, v).map(|_| None)
                        }
                        FileTransferPayload::Manifest(v) => file_transfer_handle_manifest(ctx, v),
                    };
                    match file_res {
                        Ok(Some((confirm_file_id, confirm_id))) => {
                            // Persist the receive-side ratchet before emitting a file-complete
                            // receipt so the send-side advance cannot be clobbered by the older
                            // unpack snapshot from this inbound file item.
                            commit_unpack_state();
                            queue_or_send_receipt(
                                ctx,
                                &mut pending_receipts,
                                PendingReceipt::FileComplete {
                                    file_id: confirm_file_id,
                                    confirm_id,
                                },
                            );
                        }
                        Ok(None) => {
                            commit_unpack_state();
                        }
                        Err(reason) => {
                            if reason == "manifest_mismatch" {
                                let _ =
                                    file_transfer_fail_clean(ctx.from, file_id.as_str(), reason);
                            }
                            emit_marker(
                                "file_xfer_reject",
                                Some(reason),
                                &[("id", file_id.as_str()), ("reason", reason)],
                            );
                            print_error_marker(reason);
                        }
                    }
                    continue;
                }
                if let Some(confirm) = parse_attachment_confirm_payload(&outcome.plaintext) {
                    commit_unpack_state();
                    match apply_attachment_peer_confirmation(
                        ctx.from,
                        confirm.attachment_id.as_str(),
                        confirm.confirm_handle.as_str(),
                        channel.as_str(),
                    ) {
                        Ok((ConfirmApplyOutcome::Confirmed, target)) => {
                            let device = target
                                .as_deref()
                                .or_else(|| channel_device_id(channel.as_str()));
                            emit_marker(
                                "attachment_confirm_recv",
                                None,
                                &[("attachment_id", "redacted"), ("ok", "true")],
                            );
                            emit_cli_file_delivery_with_device(
                                ctx.from,
                                "peer_confirmed",
                                confirm.attachment_id.as_str(),
                                device,
                            );
                            emit_tui_file_delivery_with_device(
                                ctx.from,
                                "peer_confirmed",
                                confirm.attachment_id.as_str(),
                                device,
                            );
                        }
                        Ok((ConfirmApplyOutcome::IgnoredWrongDevice, _)) => {
                            let dev = channel_device_marker(channel.as_str());
                            emit_cli_receipt_ignored_wrong_device(ctx.from, dev.as_str());
                            emit_tui_receipt_ignored_wrong_device(ctx.from, dev.as_str());
                        }
                        Err(reason) => emit_marker(
                            "attachment_confirm_reject",
                            Some(reason),
                            &[("reason", reason), ("ok", "false")],
                        ),
                    }
                    continue;
                }
                if let Some(file_confirm) = parse_file_confirm_payload(&outcome.plaintext) {
                    commit_unpack_state();
                    match apply_file_peer_confirmation(
                        ctx.from,
                        file_confirm.file_id.as_str(),
                        file_confirm.confirm_id.as_str(),
                        channel.as_str(),
                    ) {
                        Ok((ConfirmApplyOutcome::Confirmed, target)) => {
                            let device = target
                                .as_deref()
                                .or_else(|| channel_device_id(channel.as_str()));
                            emit_marker(
                                "file_confirm_recv",
                                None,
                                &[
                                    ("kind", "coarse_complete"),
                                    ("file_id", "redacted"),
                                    ("ok", "true"),
                                ],
                            );
                            emit_cli_file_delivery_with_device(
                                ctx.from,
                                "peer_confirmed",
                                file_confirm.file_id.as_str(),
                                device,
                            );
                            emit_tui_file_delivery_with_device(
                                ctx.from,
                                "peer_confirmed",
                                file_confirm.file_id.as_str(),
                                device,
                            );
                        }
                        Ok((ConfirmApplyOutcome::IgnoredWrongDevice, _)) => {
                            let dev = channel_device_marker(channel.as_str());
                            emit_cli_receipt_ignored_wrong_device(ctx.from, dev.as_str());
                            emit_tui_receipt_ignored_wrong_device(ctx.from, dev.as_str());
                        }
                        Err(reason) => emit_marker(
                            "file_confirm_reject",
                            Some(reason),
                            &[("reason", reason), ("ok", "false")],
                        ),
                    }
                    continue;
                }
                if let Some(ctrl) = parse_receipt_payload(&outcome.plaintext) {
                    if ctrl.v == 1 && ctrl.kind == "delivered" && ctrl.t == "ack" {
                        commit_unpack_state();
                        match apply_message_peer_confirmation(
                            ctx.from,
                            ctrl.msg_id.as_str(),
                            channel.as_str(),
                        ) {
                            Ok((ConfirmApplyOutcome::IgnoredWrongDevice, _)) => {
                                let dev = channel_device_marker(channel.as_str());
                                emit_cli_receipt_ignored_wrong_device(ctx.from, dev.as_str());
                                emit_tui_receipt_ignored_wrong_device(ctx.from, dev.as_str());
                            }
                            Ok((ConfirmApplyOutcome::Confirmed, target)) => {
                                let device = target
                                    .as_deref()
                                    .or_else(|| channel_device_id(channel.as_str()));
                                emit_marker(
                                    "receipt_recv",
                                    None,
                                    &[("kind", "delivered"), ("msg_id", "<redacted>")],
                                );
                                emit_marker(
                                    "delivered_to_peer",
                                    None,
                                    &[("kind", "delivered"), ("msg_id", "<redacted>")],
                                );
                                emit_cli_delivery_state_with_device(
                                    ctx.from,
                                    "peer_confirmed",
                                    device,
                                );
                                emit_tui_delivery_state_with_device(
                                    ctx.from,
                                    "peer_confirmed",
                                    device,
                                );
                            }
                            Err(reason) => emit_message_state_reject(ctrl.msg_id.as_str(), reason),
                        }
                        continue;
                    }
                    if ctrl.v == 1 && ctrl.kind == "delivered" && ctrl.t == "data" {
                        if let Some(body) = ctrl.body {
                            payload = body;
                            request_receipt = true;
                            request_msg_id = ctrl.msg_id;
                        }
                    }
                }
                commit_unpack_state();
                stats.count = stats.count.saturating_add(1);
                stats.bytes = stats.bytes.saturating_add(envelope_len);
                let bucket = meta_bucket_for_len(envelope_len, ctx.bucket_max);
                let bucket_s = bucket.to_string();
                let orig_s = envelope_len.to_string();
                let capped_s = if envelope_len > ctx.bucket_max {
                    ctx.bucket_max.to_string()
                } else {
                    envelope_len.to_string()
                };
                emit_marker(
                    "meta_bucket",
                    None,
                    &[
                        ("bucket", bucket_s.as_str()),
                        ("orig", orig_s.as_str()),
                        ("capped", capped_s.as_str()),
                        ("metric", "envelope_len"),
                    ],
                );
                let name = format!("recv_{}.bin", stats.count);
                let path = ctx.out.join(name);
                if write_atomic(&path, &payload, ctx.source).is_err() {
                    print_error_marker("recv_write_failed");
                }
                let idx_s = stats.count.to_string();
                let size_s = payload.len().to_string();
                emit_marker(
                    "recv_item",
                    None,
                    &[
                        ("idx", idx_s.as_str()),
                        ("size", size_s.as_str()),
                        ("id", item.id.as_str()),
                    ],
                );
                if let Err(code) = timeline_append_entry(
                    ctx.from,
                    "in",
                    payload.len(),
                    "msg",
                    MessageState::Received,
                    if request_msg_id.is_empty() {
                        None
                    } else {
                        Some(request_msg_id.as_str())
                    },
                ) {
                    emit_message_state_reject("<redacted>", code);
                    emit_marker("error", Some(code), &[("op", "timeline_receive_ingest")]);
                }
                if request_receipt {
                    queue_or_send_receipt(
                        ctx,
                        &mut pending_receipts,
                        PendingReceipt::Message {
                            msg_id: request_msg_id,
                        },
                    );
                }
            }
            Err(code) => {
                let from_alias = peer_alias_from_channel(ctx.from);
                if contacts_entry_read(from_alias).ok().flatten().is_none()
                    && channel_label_ok(from_alias)
                {
                    let _ = contact_request_upsert(from_alias, None, Some(code));
                    emit_cli_contact_request("created", from_alias, None);
                    emit_tui_contact_request("created", from_alias, None);
                }
                if code == "qsp_verify_failed" {
                    emit_file_integrity_fail(code, "rotate_mailbox_hint");
                }
                record_qsp_status(ctx.cfg_dir, ctx.cfg_source, false, code, false, false);
                emit_marker("qsp_unpack", Some(code), &[("ok", "false")]);
                if code == "qsp_replay_reject" {
                    let msg_idx = qsp_session_for_channel(ctx.from)
                        .map(|st| st.recv.nr.to_string())
                        .unwrap_or_else(|_| "0".to_string());
                    emit_marker("ratchet_replay_reject", None, &[("msg_idx", &msg_idx)]);
                }
                print_error_marker(code);
            }
        }
    }
    if let Some(service_url) = ctx.attachment_service {
        match attachment_resume_pending_for_peer(ctx, service_url) {
            Ok(resumed) => {
                stats.count = stats.count.saturating_add(resumed);
            }
            Err(reason) => {
                print_error_marker(reason);
            }
        }
    }
    flush_batched_receipts(ctx, &mut pending_receipts);
    stats
}

fn receive_file(path: &Path) {
    if !require_unlocked("receive_file") {
        return;
    }
    let (dir, source) = match config_dir() {
        Ok(v) => v,
        Err(e) => print_error(e),
    };
    // Fail-closed: reject if config dir parents or symlinks are unsafe.
    if !check_symlink_safe(&dir) {
        print_error(ErrorCode::UnsafePathSymlink);
    }
    if !check_parent_safe(&dir, source) {
        print_error(ErrorCode::UnsafeParentPerms);
    }

    let bytes = match fs::read(path) {
        Ok(v) => v,
        Err(_) => print_error(ErrorCode::IoReadFailed),
    };
    if bytes.is_empty() {
        emit_marker("recv_reject", None, &[("reason", "empty")]);
        print_error_marker("recv_reject_parse");
    }
    if bytes.len() > envelope::MAX_BUNDLE_SIZE_DEFAULT {
        emit_marker("recv_reject", None, &[("reason", "oversize")]);
        print_error_marker("recv_reject_size");
    }

    emit_marker("recv_reject", None, &[("reason", "malformed")]);
    print_error_marker("recv_reject_parse");
}

fn relay_serve(port: u16, cfg: RelayConfig, max_messages: u64) {
    let addr = format!("127.0.0.1:{}", port);
    let listener =
        TcpListener::bind(&addr).unwrap_or_else(|_| print_error_marker("relay_bind_failed"));
    let bound = listener
        .local_addr()
        .unwrap_or_else(|_| print_error_marker("relay_bind_failed"));
    let port_s = bound.port().to_string();
    let seed_s = cfg.seed.to_string();
    emit_marker(
        "relay_listen",
        None,
        &[("port", port_s.as_str()), ("seed", seed_s.as_str())],
    );

    let mut seq: u64 = 0;
    let mut inbox = RelayInboxStore::new(1024 * 1024, 1024);
    for stream in listener.incoming() {
        let mut stream = match stream {
            Ok(s) => s,
            Err(_) => continue,
        };
        seq = seq.wrapping_add(1);
        let seq_s = seq.to_string();
        let decision = relay_decide(&cfg, seq);
        if decision.delay_ms > 0 {
            let delay_s = decision.delay_ms.to_string();
            emit_marker(
                "relay_event",
                None,
                &[
                    ("action", "delay"),
                    ("ms", delay_s.as_str()),
                    ("seq", seq_s.as_str()),
                ],
            );
            std::thread::sleep(Duration::from_millis(decision.delay_ms));
        }

        if relay_try_handle_http_inbox(&mut stream, &mut inbox, &decision, seq_s.as_str()) {
            if max_messages > 0 && seq >= max_messages {
                break;
            }
            continue;
        }

        let frame: RelayFrame = match read_frame(&mut stream) {
            Ok(v) => v,
            Err(_) => {
                let resp = RelayResponse {
                    action: "reject".to_string(),
                    delivered: false,
                };
                let _ = write_frame(&mut stream, &resp);
                emit_marker(
                    "relay_event",
                    None,
                    &[("action", "reject"), ("seq", seq_s.as_str())],
                );
                if max_messages > 0 && seq >= max_messages {
                    break;
                }
                continue;
            }
        };

        let _ = frame; // relay is a dumb pipe; no persistence or content logging.
        emit_marker(
            "relay_event",
            None,
            &[("action", decision.action), ("seq", seq_s.as_str())],
        );
        let resp = RelayResponse {
            action: decision.action.to_string(),
            delivered: decision.delivered,
        };
        let _ = write_frame(&mut stream, &resp);

        if max_messages > 0 && seq >= max_messages {
            break;
        }
    }
}

struct RelayInboxStore {
    queues: BTreeMap<String, VecDeque<(u64, Vec<u8>)>>,
    next_id: u64,
    max_body: usize,
    max_queue: usize,
}

impl RelayInboxStore {
    fn new(max_body: usize, max_queue: usize) -> Self {
        Self {
            queues: BTreeMap::new(),
            next_id: 1,
            max_body,
            max_queue,
        }
    }
}

fn relay_try_handle_http_inbox(
    stream: &mut TcpStream,
    store: &mut RelayInboxStore,
    decision: &RelayDecision,
    seq: &str,
) -> bool {
    let mut prefix = [0u8; 5];
    let Ok(n) = stream.peek(&mut prefix) else {
        return false;
    };
    let is_http = (n >= 4 && &prefix[..4] == b"GET ") || (n >= 5 && &prefix[..5] == b"POST ");
    if !is_http {
        return false;
    }
    let _ = stream.set_read_timeout(Some(Duration::from_millis(1500)));
    let req = match read_http_request(stream) {
        Ok(v) => v,
        Err(_) => {
            write_http_response(stream, 400, "text/plain", b"bad_request");
            emit_marker(
                "relay_event",
                None,
                &[("action", "reject"), ("seq", seq), ("proto", "http")],
            );
            return true;
        }
    };
    if decision.delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(decision.delay_ms));
    }
    match (req.method.as_str(), parse_http_target(req.target.as_str())) {
        ("POST", Some(HttpRelayTarget::Push)) => {
            let token = match parse_http_route_token(&req) {
                Ok(v) => v,
                Err(code) => {
                    write_http_response(stream, 400, "text/plain", code.as_bytes());
                    emit_marker(
                        "relay_event",
                        None,
                        &[("action", "reject"), ("seq", seq), ("proto", "http")],
                    );
                    return true;
                }
            };
            let content_len = req
                .headers
                .get("content-length")
                .and_then(|v| v.parse::<usize>().ok())
                .unwrap_or(req.body.len());
            if content_len != req.body.len() {
                write_http_response(stream, 400, "text/plain", b"content_length_mismatch");
                emit_marker(
                    "relay_event",
                    None,
                    &[("action", "reject"), ("seq", seq), ("proto", "http")],
                );
                return true;
            }
            if req.body.len() > store.max_body {
                write_http_response(stream, 413, "text/plain", b"too_large");
                emit_marker(
                    "relay_event",
                    None,
                    &[("action", "reject"), ("seq", seq), ("proto", "http")],
                );
                return true;
            }
            if decision.action == "drop" {
                write_http_response(stream, 503, "text/plain", b"dropped");
                emit_marker(
                    "relay_event",
                    None,
                    &[("action", "drop"), ("seq", seq), ("proto", "http")],
                );
                return true;
            }
            let queue = store.queues.entry(token).or_default();
            if queue.len() >= store.max_queue {
                write_http_response(stream, 429, "text/plain", b"queue_full");
                emit_marker(
                    "relay_event",
                    None,
                    &[("action", "reject"), ("seq", seq), ("proto", "http")],
                );
                return true;
            }
            queue.push_back((store.next_id, req.body.clone()));
            store.next_id = store.next_id.saturating_add(1);
            if decision.action == "dup" && queue.len() < store.max_queue {
                queue.push_back((store.next_id, req.body));
                store.next_id = store.next_id.saturating_add(1);
            }
            write_http_response(stream, 200, "text/plain", b"ok");
            emit_marker(
                "relay_event",
                None,
                &[("action", decision.action), ("seq", seq), ("proto", "http")],
            );
            true
        }
        ("GET", Some(HttpRelayTarget::Pull(max))) => {
            let token = match parse_http_route_token(&req) {
                Ok(v) => v,
                Err(code) => {
                    write_http_response(stream, 400, "text/plain", code.as_bytes());
                    emit_marker(
                        "relay_event",
                        None,
                        &[("action", "reject"), ("seq", seq), ("proto", "http")],
                    );
                    return true;
                }
            };
            if decision.action == "drop" {
                write_http_response(stream, 503, "text/plain", b"dropped");
                emit_marker(
                    "relay_event",
                    None,
                    &[("action", "drop"), ("seq", seq), ("proto", "http")],
                );
                return true;
            }
            let pull_max = max.clamp(1, 64);
            let queue = store.queues.entry(token).or_default();
            let mut items = Vec::new();
            for _ in 0..pull_max {
                let Some((id, data)) = queue.pop_front() else {
                    break;
                };
                items.push(InboxPullItem {
                    id: id.to_string(),
                    data,
                });
            }
            if items.is_empty() {
                write_http_response(stream, 204, "text/plain", b"");
            } else {
                let payload = serde_json::to_vec(&InboxPullResp { items })
                    .unwrap_or_else(|_| b"{\"items\":[]}".to_vec());
                write_http_response(stream, 200, "application/json", payload.as_slice());
            }
            emit_marker(
                "relay_event",
                None,
                &[("action", decision.action), ("seq", seq), ("proto", "http")],
            );
            true
        }
        _ => {
            write_http_response(stream, 404, "text/plain", b"not_found");
            emit_marker(
                "relay_event",
                None,
                &[("action", "reject"), ("seq", seq), ("proto", "http")],
            );
            true
        }
    }
}

type HttpRelayTarget = adversarial::route::HttpRelayTarget;
type HttpRequestParsed = adversarial::route::HttpRequestParsed;

fn parse_http_target(target: &str) -> Option<HttpRelayTarget> {
    adversarial::route::parse_http_target(target)
}

fn parse_http_route_token(req: &HttpRequestParsed) -> Result<String, &'static str> {
    adversarial::route::parse_http_route_token_from_request(req)
}

fn read_http_request(stream: &mut TcpStream) -> Result<HttpRequestParsed, ()> {
    let mut buf = Vec::with_capacity(2048);
    let mut temp = [0u8; 1024];
    let (header_end, content_len) = loop {
        if buf.len() > 64 * 1024 {
            return Err(());
        }
        let n = stream.read(&mut temp).map_err(|_| ())?;
        if n == 0 {
            return Err(());
        }
        buf.extend_from_slice(&temp[..n]);
        if let Some(pos) = find_http_header_end(buf.as_slice()) {
            let header_bytes = &buf[..pos];
            let header_text = std::str::from_utf8(header_bytes).map_err(|_| ())?;
            let mut lines = header_text.split("\r\n");
            let _request_line = lines.next().ok_or(())?;
            let mut content_len = 0usize;
            for line in lines {
                if line.is_empty() {
                    continue;
                }
                let (k, v) = line.split_once(':').ok_or(())?;
                if k.trim().eq_ignore_ascii_case("content-length") {
                    content_len = v.trim().parse::<usize>().map_err(|_| ())?;
                }
            }
            break (pos, content_len);
        }
    };
    let body_start = header_end + 4;
    while buf.len() < body_start.saturating_add(content_len) {
        let n = stream.read(&mut temp).map_err(|_| ())?;
        if n == 0 {
            return Err(());
        }
        buf.extend_from_slice(&temp[..n]);
    }
    adversarial::route::parse_http_request_bytes(buf.as_slice()).map_err(|_| ())
}

fn find_http_header_end(buf: &[u8]) -> Option<usize> {
    adversarial::route::find_http_header_end(buf)
}

fn write_http_response(stream: &mut TcpStream, status: u16, content_type: &str, body: &[u8]) {
    let reason = match status {
        200 => "OK",
        204 => "No Content",
        400 => "Bad Request",
        404 => "Not Found",
        413 => "Payload Too Large",
        429 => "Too Many Requests",
        503 => "Service Unavailable",
        _ => "Error",
    };
    let header = format!(
        "HTTP/1.1 {} {}\r\nContent-Type: {}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
        status,
        reason,
        content_type,
        body.len()
    );
    let _ = stream.write_all(header.as_bytes());
    if !body.is_empty() {
        let _ = stream.write_all(body);
    }
}

fn relay_send(
    to: &str,
    file: &Path,
    relay: &str,
    pad_cfg: Option<MetaPadConfig>,
    bucket_max: Option<usize>,
    meta_seed: Option<u64>,
    receipt: Option<ReceiptKind>,
) {
    if let Err(code) = enforce_cli_send_contact_trust(to) {
        print_error_marker(code);
    }
    if let Err(code) = enforce_peer_not_blocked(to) {
        print_error_marker(code);
    }
    if let Err(reason) = protocol_active_or_reason_for_send_peer(to) {
        protocol_inactive_exit(reason.as_str());
    }
    let payload = match fs::read(file) {
        Ok(v) => v,
        Err(_) => print_error_marker("relay_payload_read_failed"),
    };
    let outcome = relay_send_with_payload(RelaySendPayloadArgs {
        to,
        payload,
        relay,
        injector: fault_injector_from_env(),
        pad_cfg,
        bucket_max,
        meta_seed,
        receipt,
        routing_override: None,
        tui_thread: None,
    });
    if let Some(code) = outcome.error_code {
        print_error_marker(code);
    }
}

struct RelaySendOutcome {
    action: String,
    delivered: bool,
    error_code: Option<&'static str>,
}

#[derive(Clone, Copy)]
struct TimelineSendIngest<'a> {
    peer: &'a str,
    byte_len: usize,
    kind: &'a str,
    message_id: Option<&'a str>,
    target_device_id: Option<&'a str>,
}

#[derive(Deserialize, Serialize)]
struct InboxPullItem {
    id: String,
    data: Vec<u8>,
}

#[derive(Deserialize, Serialize)]
struct InboxPullResp {
    items: Vec<InboxPullItem>,
}

#[derive(Clone)]
struct FaultInjector {
    seed: u64,
    scenario: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum FaultAction {
    Drop,
    Reorder,
}

fn fault_injector_from_env() -> Option<FaultInjector> {
    let scenario = env::var("QSC_SCENARIO").ok()?;
    if scenario == "happy-path" || scenario == "default" {
        return None;
    }
    let seed_str = match env::var("QSC_SEED") {
        Ok(v) => v,
        Err(_) => print_error_marker("fault_injection_seed_required"),
    };
    let seed = seed_str
        .trim()
        .parse::<u64>()
        .unwrap_or_else(|_| print_error_marker("fault_injection_seed_invalid"));
    Some(FaultInjector { seed, scenario })
}

fn fault_injector_from_tui(cfg: &TuiRelayConfig) -> Option<FaultInjector> {
    if cfg.scenario == "happy-path" || cfg.scenario == "default" {
        return None;
    }
    Some(FaultInjector {
        seed: cfg.seed,
        scenario: cfg.scenario.clone(),
    })
}

fn channel_label_ok(label: &str) -> bool {
    !label.is_empty()
        && label
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_' || c == '#')
}

fn relay_auth_token() -> Option<String> {
    if let Some(token) = relay_auth_token_from_env() {
        return Some(token);
    }
    if let Some(token) = relay_auth_token_from_account_secret() {
        return Some(token);
    }
    relay_auth_token_from_token_file()
}

fn relay_auth_token_from_env() -> Option<String> {
    relay_trimmed_nonempty(env::var("QSC_RELAY_TOKEN").ok())
        .or_else(|| relay_trimmed_nonempty(env::var("RELAY_TOKEN").ok()))
}

fn relay_auth_token_from_account_secret() -> Option<String> {
    let value = match vault::secret_get(TUI_RELAY_TOKEN_SECRET_KEY) {
        Ok(Some(v)) => Some(v),
        _ => None,
    };
    relay_trimmed_nonempty(value)
}

fn relay_auth_token_from_token_file() -> Option<String> {
    let token_file = match vault::secret_get(TUI_RELAY_TOKEN_FILE_SECRET_KEY) {
        Ok(Some(v)) => relay_trimmed_nonempty(Some(v)),
        _ => None,
    }?;
    read_relay_token_file(token_file.as_str()).ok()
}

fn relay_trimmed_nonempty(value: Option<String>) -> Option<String> {
    let value = value?;
    let trimmed = value.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    }
}

fn validated_attachment_service_from_env() -> Option<String> {
    relay_trimmed_nonempty(env::var(QSC_ATTACHMENT_SERVICE_ENV).ok())
}

fn legacy_in_message_stage_name(stage: LegacyInMessageStage) -> &'static str {
    match stage {
        LegacyInMessageStage::W0 => "w0",
        LegacyInMessageStage::W1 | LegacyInMessageStage::W2 => "w2",
    }
}

fn legacy_receive_mode_name(mode: LegacyReceiveMode) -> &'static str {
    match mode {
        LegacyReceiveMode::Coexistence => "coexistence",
        LegacyReceiveMode::Retired => "retired",
    }
}

fn validated_legacy_in_message_stage_from_env() -> Result<Option<LegacyInMessageStage>, &'static str>
{
    let Some(raw) = relay_trimmed_nonempty(env::var(QSC_LEGACY_IN_MESSAGE_STAGE_ENV).ok()) else {
        return Ok(None);
    };
    match raw.to_ascii_lowercase().as_str() {
        "w0" => Ok(Some(LegacyInMessageStage::W0)),
        "w1" => Ok(Some(LegacyInMessageStage::W1)),
        "w2" => Ok(Some(LegacyInMessageStage::W2)),
        _ => Err("legacy_in_message_stage_invalid"),
    }
}

fn resolve_legacy_in_message_stage(
    explicit_stage: Option<LegacyInMessageStage>,
) -> Result<LegacyInMessageStage, &'static str> {
    let env_stage = validated_legacy_in_message_stage_from_env()?;
    if validated_attachment_service_from_env().is_some() {
        let selected = explicit_stage.or(env_stage);
        return match selected {
            Some(LegacyInMessageStage::W0 | LegacyInMessageStage::W1) => {
                Err("legacy_in_message_stage_retired_post_w0")
            }
            Some(LegacyInMessageStage::W2) | None => Ok(LegacyInMessageStage::W2),
        };
    }
    if let Some(stage) = explicit_stage {
        return Ok(stage);
    }
    if let Some(stage) = env_stage {
        return Ok(stage);
    }
    Ok(LegacyInMessageStage::W0)
}

fn resolve_legacy_receive_mode(
    explicit_mode: Option<LegacyReceiveMode>,
    attachment_service: Option<&str>,
) -> Result<LegacyReceiveMode, &'static str> {
    if attachment_service.is_some() {
        return match explicit_mode {
            Some(LegacyReceiveMode::Coexistence) => Err("legacy_receive_mode_retired_post_w0"),
            Some(LegacyReceiveMode::Retired) | None => Ok(LegacyReceiveMode::Retired),
        };
    }
    Ok(explicit_mode.unwrap_or(LegacyReceiveMode::Coexistence))
}

fn resolve_large_file_attachment_service(
    explicit_attachment_service: Option<&str>,
) -> Result<String, &'static str> {
    let raw = explicit_attachment_service
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
        .or_else(validated_attachment_service_from_env)
        .ok_or("attachment_service_required")?;
    normalize_relay_endpoint(raw.as_str())
}

fn relay_inbox_push(
    relay_base: &str,
    route_token: &str,
    payload: &[u8],
) -> Result<(), &'static str> {
    let route_token = normalize_route_token(route_token)?;
    let base = normalize_relay_endpoint(relay_base)?;
    let base = base.trim_end_matches('/');
    let url = format!("{}/v1/push", base);
    let client = HttpClient::new();
    let mut req = client
        .post(url)
        .header("X-QSL-Route-Token", route_token.as_str())
        .body(payload.to_vec());
    if let Some(token) = relay_auth_token() {
        req = req.header("Authorization", format!("Bearer {}", token));
    }
    let resp = match req.send() {
        Ok(v) => v,
        Err(_) => return Err("relay_inbox_push_failed"),
    };
    match resp.status() {
        HttpStatus::OK => Ok(()),
        HttpStatus::UNAUTHORIZED | HttpStatus::FORBIDDEN => Err("relay_unauthorized"),
        HttpStatus::PAYLOAD_TOO_LARGE => Err("relay_inbox_too_large"),
        HttpStatus::TOO_MANY_REQUESTS => Err("relay_inbox_queue_full"),
        _ => Err("relay_inbox_push_failed"),
    }
}

fn relay_inbox_pull(
    relay_base: &str,
    route_token: &str,
    max: usize,
) -> Result<Vec<InboxPullItem>, &'static str> {
    let route_token = normalize_route_token(route_token)?;
    let base = normalize_relay_endpoint(relay_base)?;
    let base = base.trim_end_matches('/');
    let url = format!("{}/v1/pull?max={}", base, max);
    let client = HttpClient::new();
    let mut req = client
        .get(url)
        .header("X-QSL-Route-Token", route_token.as_str());
    if let Some(token) = relay_auth_token() {
        req = req.header("Authorization", format!("Bearer {}", token));
    }
    let resp = match req.send() {
        Ok(v) => v,
        Err(_) => return Err("relay_inbox_pull_failed"),
    };
    match resp.status() {
        HttpStatus::OK => {
            let body: InboxPullResp = match resp.json() {
                Ok(v) => v,
                Err(_) => return Err("relay_inbox_parse_failed"),
            };
            Ok(body.items)
        }
        HttpStatus::NO_CONTENT => Ok(Vec::new()),
        HttpStatus::UNAUTHORIZED | HttpStatus::FORBIDDEN => Err("relay_unauthorized"),
        HttpStatus::BAD_REQUEST => Err("relay_inbox_bad_request"),
        HttpStatus::PAYLOAD_TOO_LARGE => Err("relay_inbox_too_large"),
        HttpStatus::TOO_MANY_REQUESTS => Err("relay_inbox_queue_full"),
        _ => Err("relay_inbox_pull_failed"),
    }
}

fn fault_action_for(fi: &FaultInjector, idx: u64) -> Option<FaultAction> {
    if fi.scenario != "drop-reorder" {
        return None;
    }
    let k = fi.seed.wrapping_add(idx);
    match k % 4 {
        0 => Some(FaultAction::Reorder),
        1 => Some(FaultAction::Drop),
        _ => None,
    }
}

static FAULT_IDX: AtomicU64 = AtomicU64::new(0);

fn next_fault_index() -> u64 {
    FAULT_IDX.fetch_add(1, Ordering::SeqCst).wrapping_add(1)
}

struct RelaySendPayloadArgs<'a> {
    to: &'a str,
    payload: Vec<u8>,
    relay: &'a str,
    injector: Option<FaultInjector>,
    pad_cfg: Option<MetaPadConfig>,
    bucket_max: Option<usize>,
    meta_seed: Option<u64>,
    receipt: Option<ReceiptKind>,
    routing_override: Option<SendRoutingTarget>,
    tui_thread: Option<&'a str>,
}

fn relay_send_with_payload(args: RelaySendPayloadArgs<'_>) -> RelaySendOutcome {
    let RelaySendPayloadArgs {
        to,
        payload,
        relay,
        injector,
        pad_cfg,
        bucket_max,
        meta_seed,
        receipt,
        routing_override,
        tui_thread,
    } = args;
    if let Err(code) = normalize_relay_endpoint(relay) {
        return RelaySendOutcome {
            action: "endpoint_reject".to_string(),
            delivered: false,
            error_code: Some(code),
        };
    }
    let routing = match routing_override {
        Some(v) => v,
        None => match resolve_send_routing_target(to) {
            Ok(v) => v,
            Err(code) => {
                return RelaySendOutcome {
                    action: "route_token_reject".to_string(),
                    delivered: false,
                    error_code: Some(code),
                };
            }
        },
    };
    emit_cli_routing_marker(
        routing.peer_alias.as_str(),
        routing.device_id.as_str(),
        routing.implicit_primary,
    );
    emit_cli_confirm_policy();
    if let Some(thread) = tui_thread {
        emit_tui_routing_marker(thread, routing.device_id.as_str(), routing.implicit_primary);
        emit_tui_named_marker("QSC_CONFIRM_POLICY", &[("policy", CONFIRM_POLICY.as_str())]);
    }
    let push_route_token = routing.route_token.clone();
    let (dir, source) = match config_dir() {
        Ok(v) => v,
        Err(e) => print_error(e),
    };
    let _lock = match lock_store_exclusive(&dir, source) {
        Ok(v) => v,
        Err(e) => print_error(e),
    };
    if let Err(e) = ensure_store_layout(&dir, source) {
        print_error(e);
    }

    let outbox_path = dir.join(OUTBOX_FILE_NAME);
    if outbox_path.exists() {
        let outbox = match outbox_record_load(&outbox_path) {
            Ok(v) => v,
            Err(code) => {
                emit_marker("error", Some(code), &[]);
                return RelaySendOutcome {
                    action: "outbox_load_failed".to_string(),
                    delivered: false,
                    error_code: Some(code),
                };
            }
        };
        if outbox.to.is_empty() || outbox.ciphertext.is_empty() {
            emit_marker("error", Some("outbox_recovery_required"), &[]);
            return RelaySendOutcome {
                action: "outbox_recovery_required".to_string(),
                delivered: false,
                error_code: Some("outbox_recovery_required"),
            };
        }
        let next_state = match outbox_next_state_load() {
            Ok(v) => v,
            Err(code) => {
                emit_marker("error", Some(code), &[]);
                return RelaySendOutcome {
                    action: "outbox_state_missing".to_string(),
                    delivered: false,
                    error_code: Some(code),
                };
            }
        };
        let replay_route_token = if outbox.to == routing.peer_alias {
            routing.route_token.clone()
        } else {
            match relay_peer_route_token(outbox.to.as_str()) {
                Ok(v) => v,
                Err(code) => {
                    return RelaySendOutcome {
                        action: "route_token_reject".to_string(),
                        delivered: false,
                        error_code: Some(code),
                    };
                }
            }
        };
        print_marker("send_retry", &[("mode", "outbox_replay")]);
        return match relay_inbox_push(relay, replay_route_token.as_str(), &outbox.ciphertext) {
            Ok(()) => finalize_send_commit(
                &dir,
                source,
                &outbox_path,
                "replay_deliver".to_string(),
                Some((
                    outbox.channel.as_deref().unwrap_or(outbox.to.as_str()),
                    next_state,
                )),
                Some(TimelineSendIngest {
                    peer: outbox.to.as_str(),
                    byte_len: outbox.payload_len,
                    kind: outbox.kind.as_str(),
                    message_id: outbox.message_id.as_deref(),
                    target_device_id: outbox.channel.as_deref().and_then(channel_device_id),
                }),
            ),
            Err(code) => {
                print_marker("send_attempt", &[("ok", "false")]);
                RelaySendOutcome {
                    action: "push_fail".to_string(),
                    delivered: false,
                    error_code: Some(code),
                }
            }
        };
    }

    let (payload, receipt_msg_id) = encode_receipt_data_payload(payload, receipt);
    let pack = match qsp_pack(routing.channel.as_str(), &payload, pad_cfg, meta_seed) {
        Ok(v) => {
            record_qsp_status(&dir, source, true, "pack_ok", true, false);
            emit_marker("qsp_pack", None, &[("ok", "true"), ("version", "5.0")]);
            if let Some(label) = v.pad_label {
                let len_s = v.padded_len.to_string();
                emit_marker(
                    "meta_pad",
                    None,
                    &[("bucket", label), ("padded_len", len_s.as_str())],
                );
            }
            let msg_idx_s = v.msg_idx.to_string();
            let ck_idx_s = v.ck_idx.to_string();
            emit_marker(
                "ratchet_send_advance",
                None,
                &[
                    ("msg_idx", msg_idx_s.as_str()),
                    ("ck_idx", ck_idx_s.as_str()),
                ],
            );
            v
        }
        Err(err) => {
            record_qsp_status(&dir, source, false, err.code, false, false);
            if let Some(reason) = err.reason {
                emit_marker(
                    "qsp_pack",
                    Some(err.code),
                    &[("ok", "false"), ("reason", reason)],
                );
            } else {
                emit_marker("qsp_pack", Some(err.code), &[("ok", "false")]);
            }
            return RelaySendOutcome {
                action: err.code.to_string(),
                delivered: false,
                error_code: Some(err.code),
            };
        }
    };
    let ciphertext = pack.envelope.clone();
    if receipt_msg_id.is_some() {
        emit_marker(
            "receipt_request",
            None,
            &[("kind", "delivered"), ("msg_id", "<redacted>")],
        );
    }
    if let Some(max_bucket) = bucket_max {
        if max_bucket == 0 || max_bucket > META_BUCKET_MAX_CEILING {
            return RelaySendOutcome {
                action: "meta_bucket_invalid".to_string(),
                delivered: false,
                error_code: Some("meta_bucket_invalid"),
            };
        }
        let bucket = meta_bucket_for_len(ciphertext.len(), max_bucket);
        let bucket_s = bucket.to_string();
        let orig_s = ciphertext.len().to_string();
        let capped_s = ciphertext.len().min(max_bucket).to_string();
        emit_marker(
            "meta_bucket",
            None,
            &[
                ("bucket", bucket_s.as_str()),
                ("orig", orig_s.as_str()),
                ("capped", capped_s.as_str()),
                ("metric", "envelope_len"),
            ],
        );
    }
    let outbox = OutboxRecord {
        version: 1,
        payload_len: payload.len(),
        to: to.to_string(),
        channel: Some(routing.channel.clone()),
        ciphertext: ciphertext.clone(),
        kind: "file".to_string(),
        message_id: receipt_msg_id.clone(),
    };
    let outbox_bytes = match serde_json::to_vec(&outbox) {
        Ok(v) => v,
        Err(_) => {
            emit_marker("error", Some("outbox_serialize_failed"), &[]);
            return RelaySendOutcome {
                action: "outbox_serialize_failed".to_string(),
                delivered: false,
                error_code: Some("outbox_serialize_failed"),
            };
        }
    };
    if write_atomic(&outbox_path, &outbox_bytes, source).is_err() {
        emit_marker("error", Some("outbox_write_failed"), &[]);
        return RelaySendOutcome {
            action: "outbox_write_failed".to_string(),
            delivered: false,
            error_code: Some("outbox_write_failed"),
        };
    }
    if let Err(code) = outbox_next_state_store(&pack.next_state) {
        let _ = fs::remove_file(&outbox_path);
        emit_marker("error", Some(code), &[]);
        return RelaySendOutcome {
            action: "outbox_state_store_failed".to_string(),
            delivered: false,
            error_code: Some(code),
        };
    }

    if let Some(fi) = injector.as_ref() {
        let idx = next_fault_index();
        let idx_s = idx.to_string();
        let seed_s = fi.seed.to_string();
        if let Some(action) = fault_action_for(fi, idx) {
            match action {
                FaultAction::Drop => {
                    emit_marker(
                        "relay_event",
                        None,
                        &[
                            ("action", "drop"),
                            ("idx", idx_s.as_str()),
                            ("seed", seed_s.as_str()),
                            ("scenario", fi.scenario.as_str()),
                        ],
                    );
                    print_marker("send_attempt", &[("ok", "false")]);
                    return RelaySendOutcome {
                        action: "drop".to_string(),
                        delivered: false,
                        error_code: Some("relay_drop_injected"),
                    };
                }
                FaultAction::Reorder => {
                    emit_marker(
                        "relay_event",
                        None,
                        &[
                            ("action", "reorder"),
                            ("idx", idx_s.as_str()),
                            ("seed", seed_s.as_str()),
                            ("scenario", fi.scenario.as_str()),
                        ],
                    );
                }
            }
        }
    }

    let len_s = payload.len().to_string();
    print_marker("send_prepare", &[("payload_len", len_s.as_str())]);

    match relay_inbox_push(relay, push_route_token.as_str(), &ciphertext) {
        Ok(()) => {
            emit_marker("relay_event", None, &[("action", "deliver")]);
            emit_cli_delivery_state_with_device(
                to,
                "accepted_by_relay",
                Some(routing.device_id.as_str()),
            );
            if let Some(thread) = tui_thread {
                emit_tui_delivery_state_with_device(
                    thread,
                    "accepted_by_relay",
                    Some(routing.device_id.as_str()),
                );
            }
            finalize_send_commit(
                &dir,
                source,
                &outbox_path,
                "deliver".to_string(),
                Some((routing.channel.as_str(), pack.next_state.clone())),
                Some(TimelineSendIngest {
                    peer: to,
                    byte_len: payload.len(),
                    kind: "file",
                    message_id: receipt_msg_id.as_deref(),
                    target_device_id: Some(routing.device_id.as_str()),
                }),
            )
        }
        Err(code) => {
            emit_marker("relay_event", None, &[("action", "push_fail")]);
            print_marker("send_attempt", &[("ok", "false")]);
            RelaySendOutcome {
                action: "push_fail".to_string(),
                delivered: false,
                error_code: Some(code),
            }
        }
    }
}

fn finalize_send_commit(
    dir: &Path,
    source: ConfigSource,
    outbox_path: &Path,
    action: String,
    session_update: Option<(&str, Suite2SessionState)>,
    timeline_ingest: Option<TimelineSendIngest<'_>>,
) -> RelaySendOutcome {
    let next_seq = match read_send_state(dir, source) {
        Ok(v) => v + 1,
        Err(()) => {
            emit_marker("error", Some("send_state_parse_failed"), &[]);
            return RelaySendOutcome {
                action,
                delivered: true,
                error_code: Some("send_state_parse_failed"),
            };
        }
    };
    if let Some((peer, st)) = session_update {
        if qsp_session_store(peer, &st).is_err() {
            emit_marker("error", Some("qsp_session_store_failed"), &[]);
            return RelaySendOutcome {
                action,
                delivered: true,
                error_code: Some("qsp_session_store_failed"),
            };
        }
    }
    if let Some(ingest) = timeline_ingest {
        if let Err(code) = timeline_append_entry_for_target(
            ingest.peer,
            "out",
            ingest.byte_len,
            ingest.kind,
            MessageState::Sent,
            ingest.message_id,
            ingest.target_device_id,
        ) {
            emit_message_state_reject("<redacted>", code);
            emit_marker("error", Some(code), &[("op", "timeline_send_ingest")]);
        }
    }
    let state_bytes = format!("send_seq={}\n", next_seq).into_bytes();
    if write_atomic(&dir.join(SEND_STATE_NAME), &state_bytes, source).is_err() {
        emit_marker("error", Some("send_commit_write_failed"), &[]);
        return RelaySendOutcome {
            action,
            delivered: true,
            error_code: Some("send_commit_write_failed"),
        };
    }
    if fs::remove_file(outbox_path).is_err() {
        emit_marker("error", Some("outbox_remove_failed"), &[]);
        return RelaySendOutcome {
            action,
            delivered: true,
            error_code: Some("outbox_remove_failed"),
        };
    }
    if let Err(code) = outbox_next_state_clear() {
        emit_marker("error", Some(code), &[]);
        return RelaySendOutcome {
            action,
            delivered: true,
            error_code: Some(code),
        };
    }
    print_marker("send_attempt", &[("ok", "true")]);
    let seq_s = next_seq.to_string();
    print_marker("send_commit", &[("send_seq", seq_s.as_str())]);
    RelaySendOutcome {
        action,
        delivered: true,
        error_code: None,
    }
}

fn read_frame<T: for<'de> Deserialize<'de>>(stream: &mut TcpStream) -> Result<T, ()> {
    let mut len_buf = [0u8; 4];
    stream.read_exact(&mut len_buf).map_err(|_| ())?;
    let len = u32::from_be_bytes(len_buf) as usize;
    if len == 0 || len > 1_048_576 {
        return Err(());
    }
    let mut buf = vec![0u8; len];
    stream.read_exact(&mut buf).map_err(|_| ())?;
    serde_json::from_slice(&buf).map_err(|_| ())
}

fn write_frame<T: Serialize>(stream: &mut TcpStream, value: &T) -> Result<(), ()> {
    let bytes = serde_json::to_vec(value).map_err(|_| ())?;
    let len = bytes.len();
    if len > u32::MAX as usize {
        return Err(());
    }
    let len_buf = (len as u32).to_be_bytes();
    stream.write_all(&len_buf).map_err(|_| ())?;
    stream.write_all(&bytes).map_err(|_| ())?;
    Ok(())
}

fn read_send_state(dir: &Path, source: ConfigSource) -> Result<u64, ()> {
    let path = dir.join(SEND_STATE_NAME);
    if let Err(e) = enforce_safe_parents(&path, source) {
        print_error(e);
    }
    if !path.exists() {
        return Ok(0);
    }
    let mut f = File::open(&path).map_err(|_| ())?;
    let mut buf = String::new();
    f.read_to_string(&mut buf).map_err(|_| ())?;
    for line in buf.lines() {
        if let Some(rest) = line.trim().strip_prefix("send_seq=") {
            let v = rest.trim().parse::<u64>().map_err(|_| ())?;
            return Ok(v);
        }
    }
    Err(())
}

fn util_sanitize(print: Option<Vec<String>>) {
    let Some(parts) = print else {
        qsc_mark("util_sanitize", "usage");
        eprintln!("usage: qsc util sanitize --print <text>");
        process::exit(2);
    };
    let raw = parts.join(" ");
    let sanitized = qsc_sanitize_terminal_text(&raw);
    println!("{}", redact_text_for_output(&sanitized));
    qsc_mark("util_sanitize", "ok");
}

fn util_panic_demo() {
    panic!("panic_demo {}", PANIC_DEMO_SENTINEL);
}

fn util_receipt_apply(
    peer: &str,
    channel: &str,
    msg_id: Option<String>,
    file_id: Option<String>,
    confirm_id: Option<String>,
) {
    if !env_bool("QSC_TEST_MODE") {
        print_error_marker("test_mode_required");
    }
    if !channel_label_ok(peer) || !channel_label_ok(channel) {
        print_error_marker("qsp_channel_invalid");
    }
    emit_cli_confirm_policy();
    match (msg_id.as_deref(), file_id.as_deref(), confirm_id.as_deref()) {
        (Some(msg), None, None) => match apply_message_peer_confirmation(peer, msg, channel) {
            Ok((ConfirmApplyOutcome::IgnoredWrongDevice, _)) => {
                let dev = channel_device_marker(channel);
                emit_cli_receipt_ignored_wrong_device(peer, dev.as_str());
            }
            Ok((ConfirmApplyOutcome::Confirmed, target)) => {
                let device = target.as_deref().or_else(|| channel_device_id(channel));
                emit_cli_delivery_state_with_device(peer, "peer_confirmed", device);
            }
            Err(code) => print_error_marker(code),
        },
        (None, Some(file), Some(confirm)) => {
            let file_id = if file == "latest" {
                latest_outbound_file_id(peer).unwrap_or_else(|code| print_error_marker(code))
            } else {
                file.to_string()
            };
            let confirm_id = if confirm == "auto" {
                file_transfer_confirm_id(peer, file_id.as_str())
                    .unwrap_or_else(|code| print_error_marker(code))
            } else {
                confirm.to_string()
            };
            match apply_file_peer_confirmation(peer, file_id.as_str(), confirm_id.as_str(), channel)
            {
                Ok((ConfirmApplyOutcome::IgnoredWrongDevice, _)) => {
                    let dev = channel_device_marker(channel);
                    emit_cli_receipt_ignored_wrong_device(peer, dev.as_str());
                }
                Ok((ConfirmApplyOutcome::Confirmed, target)) => {
                    let device = target.as_deref().or_else(|| channel_device_id(channel));
                    emit_cli_file_delivery_with_device(
                        peer,
                        "peer_confirmed",
                        file_id.as_str(),
                        device,
                    );
                }
                Err(code) => print_error_marker(code),
            }
        }
        _ => print_error_marker("receipt_apply_invalid_args"),
    }
}

struct BoundedQueue<T> {
    max: usize,
    items: VecDeque<T>,
}

impl<T> BoundedQueue<T> {
    fn new(max: usize) -> Self {
        Self {
            max,
            items: VecDeque::new(),
        }
    }

    fn push(&mut self, item: T) -> Result<(), ()> {
        if self.items.len() >= self.max {
            return Err(());
        }
        self.items.push_back(item);
        Ok(())
    }
}

fn util_queue(len: usize) {
    let mut q = BoundedQueue::new(MAX_QUEUE_LEN);
    for i in 0..len {
        if q.push(i).is_err() {
            print_error_marker("queue_limit_exceeded");
        }
    }
    print_marker("queue_limit", &[("ok", "true")]);
}

fn util_history(len: usize) {
    let mut h = BoundedQueue::new(MAX_HISTORY_LEN);
    for i in 0..len {
        if h.push(i).is_err() {
            print_error_marker("history_limit_exceeded");
        }
    }
    print_marker("history_limit", &[("ok", "true")]);
}

fn bounded_retry<F>(mut attempts: u32, mut op: F) -> Result<u32, ()>
where
    F: FnMut() -> Result<(), ()>,
{
    let mut tried = 0;
    let mut backoff = RETRY_BASE_MS;
    while attempts > 0 {
        tried += 1;
        match op() {
            Ok(()) => return Ok(tried),
            Err(()) => {
                attempts -= 1;
                if attempts == 0 {
                    return Err(());
                }
                let jitter = (tried as u64 % (RETRY_JITTER_MS + 1)).min(RETRY_JITTER_MS);
                let sleep_ms = (backoff + jitter).min(RETRY_MAX_MS);
                std::thread::sleep(Duration::from_millis(sleep_ms));
                backoff = (backoff * 2).min(RETRY_MAX_MS);
            }
        }
    }
    Err(())
}

fn util_retry(fail: u32) {
    let mut remaining = fail;
    let res = bounded_retry(MAX_RETRY_ATTEMPTS, || {
        if remaining > 0 {
            remaining -= 1;
            Err(())
        } else {
            Ok(())
        }
    });
    match res {
        Ok(attempts) => {
            let attempts_s = attempts.to_string();
            print_marker("retry_bound", &[("attempts", attempts_s.as_str())]);
        }
        Err(()) => print_error_marker("retry_limit_exceeded"),
    }
}

fn util_timeout(wait_ms: u64, timeout_ms: u64) {
    let limit = timeout_ms.clamp(1, MAX_TIMEOUT_MS);
    if wait_ms > limit {
        print_error_marker("timeout_exceeded");
    }
    let elapsed_s = wait_ms.to_string();
    print_marker("timeout_ok", &[("elapsed_ms", elapsed_s.as_str())]);
}

fn util_envelope(
    tick_count: usize,
    interval_ms: u64,
    max_ticks: usize,
    max_bundle: usize,
    max_count: usize,
    payload_lens: Vec<usize>,
) {
    let ticks = match envelope::tick_schedule(tick_count, interval_ms, max_ticks) {
        Ok(v) => v,
        Err(e) => print_error_marker(e.code()),
    };
    let bundle = match envelope::pack_bundle(&payload_lens, max_bundle, max_count) {
        Ok(v) => v,
        Err(e) => print_error_marker(e.code()),
    };
    let ticks_s = ticks.len().to_string();
    let interval_s = interval_ms.to_string();
    let bucket_s = bundle.bucket_len.to_string();
    let total_s = bundle.total_len.to_string();
    let count_s = bundle.payload_lens.len().to_string();
    print_marker(
        "envelope_plan",
        &[
            ("ticks", ticks_s.as_str()),
            ("interval_ms", interval_s.as_str()),
            ("bucket_size", bucket_s.as_str()),
            ("bundle_len", total_s.as_str()),
            ("payload_count", count_s.as_str()),
        ],
    );
}

fn envelope_plan_ack(
    deterministic: bool,
    tick_count: usize,
    interval_ms: u64,
    max_ticks: usize,
    max_bundle: usize,
    max_count: usize,
    small_len: usize,
) {
    if !deterministic {
        print_error_marker("ack_plan_requires_deterministic");
    }
    let plan = match envelope::plan_ack(
        small_len,
        tick_count,
        interval_ms,
        max_ticks,
        max_bundle,
        max_count,
    ) {
        Ok(v) => v,
        Err(e) => print_error_marker(e.code()),
    };
    let tick = plan.ticks.first().copied().unwrap_or(0);
    let tick_s = tick.to_string();
    let bucket_s = plan.bundle.bucket_len.to_string();
    print_marker(
        "ack_plan",
        &[("size_class", bucket_s.as_str()), ("tick", tick_s.as_str())],
    );
}

fn normalize_tui_autolock_minutes(value: &str) -> Result<u64, ErrorCode> {
    let minutes = value
        .trim()
        .parse::<u64>()
        .map_err(|_| ErrorCode::ParseFailed)?;
    if !(TUI_AUTOLOCK_MIN_MINUTES..=TUI_AUTOLOCK_MAX_MINUTES).contains(&minutes) {
        return Err(ErrorCode::ParseFailed);
    }
    Ok(minutes)
}

fn normalize_tui_poll_interval_seconds(value: &str) -> Result<u64, ErrorCode> {
    let seconds = value
        .trim()
        .parse::<u64>()
        .map_err(|_| ErrorCode::ParseFailed)?;
    if !(TUI_POLL_MIN_INTERVAL_SECONDS..=TUI_POLL_MAX_INTERVAL_SECONDS).contains(&seconds) {
        return Err(ErrorCode::ParseFailed);
    }
    Ok(seconds)
}

fn normalize_tui_poll_mode(value: &str) -> Result<TuiPollMode, ErrorCode> {
    match value.trim() {
        "adaptive" => Ok(TuiPollMode::Adaptive),
        "fixed" => Ok(TuiPollMode::Fixed),
        _ => Err(ErrorCode::ParseFailed),
    }
}

fn print_error(code: ErrorCode) -> ! {
    emit_marker("error", Some(code.as_str()), &[]);
    process::exit(1);
}

fn bool_str(v: bool) -> &'static str {
    if v {
        "true"
    } else {
        "false"
    }
}

#[cfg(unix)]
extern "C" {
    fn umask(mask: u32) -> u32;
    fn flock(fd: i32, operation: i32) -> i32;
}

fn write_doctor_export(path: &Path, report: &DoctorReport) -> Result<(), ErrorCode> {
    let dir = path.parent().ok_or(ErrorCode::IoWriteFailed)?;
    let payload = serde_json::to_vec(report).map_err(|_| ErrorCode::IoWriteFailed)?;
    let tmp = dir.join(format!(
        "{}.tmp.{}",
        path.file_name()
            .and_then(|v| v.to_str())
            .unwrap_or("doctor"),
        process::id()
    ));
    let _ = fs::remove_file(&tmp);
    fs::create_dir_all(dir).map_err(|_| ErrorCode::IoWriteFailed)?;

    let mut f = OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(&tmp)
        .map_err(|_| ErrorCode::IoWriteFailed)?;
    f.write_all(&payload)
        .map_err(|_| ErrorCode::IoWriteFailed)?;
    f.sync_all().map_err(|_| ErrorCode::IoWriteFailed)?;
    fs::rename(&tmp, path).map_err(|_| ErrorCode::IoWriteFailed)?;
    fsync_dir_best_effort(dir);
    Ok(())
}

#[cfg(test)]
mod message_state_tests {
    use super::{message_state_transition_allowed, MessageState};

    #[test]
    fn failed_state_is_terminal() {
        let err =
            message_state_transition_allowed(MessageState::Failed, MessageState::Delivered, "out")
                .expect_err("FAILED must be terminal");
        assert_eq!(err, "failed_terminal");
    }

    #[test]
    fn out_state_cannot_skip_to_delivered() {
        let err =
            message_state_transition_allowed(MessageState::Created, MessageState::Delivered, "out")
                .expect_err("CREATED -> DELIVERED must reject");
        assert_eq!(err, "state_invalid_transition");
    }

    #[test]
    fn in_state_cannot_transition_to_delivered() {
        let err =
            message_state_transition_allowed(MessageState::Received, MessageState::Delivered, "in")
                .expect_err("RECEIVED -> DELIVERED must reject for inbound timeline");
        assert_eq!(err, "state_invalid_transition");
    }
}

#[cfg(test)]
mod tui_perf_tests {
    use super::{
        tui_next_poll_timeout_ms, TuiConfig, TuiPollMode, TuiState, TUI_POLL_MIN_INTERVAL_SECONDS,
    };

    fn test_tui_config(relay: bool) -> TuiConfig {
        TuiConfig {
            relay: if relay {
                Some("http://127.0.0.1:9".to_string())
            } else {
                None
            },
            token_file: None,
            seed: 0,
            scenario: "direct".to_string(),
        }
    }

    #[test]
    fn interactive_poll_timeout_is_never_zero() {
        assert!(
            tui_next_poll_timeout_ms() >= 50,
            "interactive poll timeout must be clamped to prevent busy loops"
        );
    }

    #[test]
    fn idle_clock_advance_without_state_change_does_not_request_redraw() {
        let mut state = TuiState::new(test_tui_config(false));
        state.vault_locked = false;
        state.status.locked = "UNLOCKED";
        state.needs_redraw = false;
        for _ in 0..32 {
            state.headless_advance_clock(100);
            assert!(
                !state.needs_redraw,
                "idle clock advancement must not schedule redraws without state changes"
            );
        }
    }

    #[test]
    fn fixed_poll_due_seed_respects_minimum_interval_clamp() {
        let mut state = TuiState::new(test_tui_config(true));
        state.vault_locked = false;
        state.status.locked = "UNLOCKED";
        state.poll_mode = TuiPollMode::Fixed;
        state.poll_interval_seconds = TUI_POLL_MIN_INTERVAL_SECONDS.saturating_sub(1);
        state.poll_next_due_ms = None;
        assert!(
            !state.maybe_run_fixed_poll(0),
            "initial scheduling should seed next due timestamp without immediate tick"
        );
        assert_eq!(
            state.poll_next_due_ms,
            Some(TUI_POLL_MIN_INTERVAL_SECONDS.saturating_mul(1_000)),
            "fixed polling due timestamp must respect min interval clamp"
        );
    }
}

#[cfg(test)]
mod relay_url_policy_tests {
    use super::normalize_relay_endpoint;
    use crate::adversarial::route::QSC_ERR_RELAY_TLS_REQUIRED;

    #[test]
    fn relay_url_policy_allow_deny_matrix() {
        assert!(normalize_relay_endpoint("http://localhost:8080").is_ok());
        assert!(normalize_relay_endpoint("http://127.0.0.1:8080").is_ok());
        assert!(normalize_relay_endpoint("http://[::1]:8080").is_ok());
        assert!(normalize_relay_endpoint("https://example.com").is_ok());

        assert_eq!(
            normalize_relay_endpoint("http://example.com")
                .expect_err("non-loopback http must reject"),
            QSC_ERR_RELAY_TLS_REQUIRED
        );
        assert_eq!(
            normalize_relay_endpoint("http://192.168.1.10")
                .expect_err("non-loopback LAN http must reject"),
            QSC_ERR_RELAY_TLS_REQUIRED
        );
        assert_eq!(
            normalize_relay_endpoint("tcp://example.com")
                .expect_err("non-http scheme must reject deterministically"),
            "relay_endpoint_invalid_scheme"
        );
    }
}
