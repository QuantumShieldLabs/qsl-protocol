#![allow(unexpected_cfgs)]

use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;
use chacha20poly1305::aead::{Aead, KeyInit, Payload};
use chacha20poly1305::{ChaCha20Poly1305, Key, Nonce};
use quantumshield_refimpl::crypto::stdcrypto::{
    runtime_pq_kem_ciphertext_bytes, runtime_pq_kem_keypair, runtime_pq_kem_public_key_bytes,
    runtime_pq_sig_keypair, runtime_pq_sig_public_key_bytes, runtime_pq_sig_signature_bytes,
    StdCrypto,
};
use quantumshield_refimpl::crypto::traits::{
    Hash, PqKem768, PqSigMldsa65, X25519Dh, X25519Priv, X25519Pub,
};
use quantumshield_refimpl::suite2::establish::init_from_base_handshake;
use quantumshield_refimpl::suite2::state::Suite2SessionState;
use quantumshield_refimpl::suite2::types::{
    SUITE2_PROTOCOL_VERSION, SUITE2_SUITE_ID,
};
use rand_core::{OsRng, RngCore};
use reqwest::blocking::Client as HttpClient;
use reqwest::StatusCode as HttpStatus;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha512};
use std::collections::{BTreeMap, VecDeque};
use std::env;
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::process;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::time::Duration;
use zeroize::Zeroize;

const CONFIG_FILE_NAME: &str = "config.txt";
const STORE_META_NAME: &str = "store.meta";
const LOCK_FILE_NAME: &str = ".qsc.lock";
const OUTBOX_FILE_NAME: &str = "outbox.json";
/// NA-0682 (D617 F1): control payloads are emitted at v2 (CSPRNG `msg_id` + `ns` marker).
/// ⚠ v1 is still ACCEPTED on receive -- see `classify_control`, which matches the legacy
/// shapes exactly and unchanged before it consults the marker.
const CTRL_VERSION: u8 = 2;
const SEND_STATE_NAME: &str = "send.state";
const QSE_ENV_VERSION_V1: u16 = 0x0100;
const POLICY_KEY: &str = "policy_profile";
// NA-0688 C4 (D622 R7): the per-install acknowledged-pull preference, in the CONFIG FILE rather
// than the vault. It is not a secret, and a config-file preference cannot silently fail to apply
// when the vault happens to be locked -- which is the whole reason R7 chose this store.
// ⚠ Deliberately NOT named `tui.*`: that namespace belongs to a subsystem that was retired and
// stripped (NA-0645), and four of its keys are dead reads with no writer at all.
pub(crate) const ACK_MODE_KEY: &str = "ack_mode";
const STORE_META_TEMPLATE: &str = "store_version=1\nvmk_status=unset\nkeyslots=0\n";
pub const MAX_QUEUE_LEN: usize = 64;
pub const MAX_HISTORY_LEN: usize = 128;
pub const MAX_RETRY_ATTEMPTS: u32 = 5;
const RETRY_BASE_MS: u64 = 20;
const RETRY_MAX_MS: u64 = 200;
const RETRY_JITTER_MS: u64 = 10;
pub const MAX_TIMEOUT_MS: u64 = 2000;
// ⚠ NA-0688 / D622 (R2a THIRD AMENDMENT) — WHAT THESE TWO ACTUALLY DO, MEASURED.
//
// Neither of them defers anything in time, and the prose that said otherwise has been
// corrected rather than left standing:
//
//   RECEIPT_BATCH_WINDOW_MS_DEFAULT is INERT at runtime. It is read at exactly two sites,
//   and BOTH only echo it into a diagnostic marker. No code waits on it, sleeps on it, or
//   schedules against it. It survives as a configurable value, not as a delay.
//
//   RECEIPT_JITTER_MS_DEFAULT is an ORDERING knob, not a delay. `flush_batched_receipts`
//   uses it solely as a stable-sort key bias, so it permutes the order receipts are flushed
//   in and changes nothing about WHEN.
//
// The real cadence is therefore: receipts are QUEUED IN MEMORY during a receive-pull and
// COALESCED INTO THE END-OF-PULL FLUSH — one batch per pull, ordered by the jitter bias.
// There is no wall-clock deferral in v1. That property is pinned by
// `na0688_eng0095_ack_nonce_barrier::receipt_sends_are_coalesced_into_the_end_of_pull_flush`.
//
// ⚠ ANY honest-limit wording (R2d) must be written against THIS mechanism and must never
// claim a timing window that does not exist. Removing the inert constant is a later
// cleanup, deliberately out of scope for the lane that measured it.
const RECEIPT_BATCH_WINDOW_MS_DEFAULT: u64 = 250;
const RECEIPT_JITTER_MS_DEFAULT: u64 = 0;
const RECEIPT_BATCH_WINDOW_MS_MAX: u64 = 60_000;
const RECEIPT_JITTER_MS_MAX: u64 = 5_000;
const ATTACHMENT_DESCRIPTOR_VERSION: u8 = 1;
const ATTACHMENT_DESCRIPTOR_TYPE: &str = "attachment_descriptor";
const ATTACHMENT_CONFIRM_KIND: &str = "attachment_confirmed";
#[cfg(test)]
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

// NA0487_HELPER_API_NO_PRODUCTION_BEHAVIOR_CHANGE_OK:
// binding fuzz helper exports live behind qsc_binding_fuzz_helper only.
pub mod adversarial;
mod owed_receipts;
pub mod attachments;
pub mod clock;
pub mod cmd;
pub mod contacts;
pub mod dedup;
pub mod envelope;
// NA-0751 (D-1393): the GUI-facing typed facade. Calls; never edits.
pub mod facade;
pub mod fs_store;
// NA-0741 (D-1376) lane 1: the receive-side frame classifier. Crate-private — it is an
// internal dispatch aid, not a surface, and LANE 2 will call it from `invite/` and
// `handshake/` as well, which is why it is a top-level module rather than a child of
// `transport/`.
mod frameclass;
pub mod handshake;
pub mod identity;
// NA-0681 (D616) messaging-epic Slice 2: the invite system's client half --
// canonical encodings, commitment, signature, state machine, handshake envelope.
// Sockets stay in `transport` (D616 F4).
pub mod invite;
pub mod model;
pub mod msgqueue;
pub mod output;
pub mod protocol_state;
pub mod quarantine;
pub mod relay;
pub mod store;
pub mod timeline;
mod directional_core;
mod directional_delivery;
pub mod transport;
pub mod vault;

pub(crate) use timeline::{timeline_ts_default, TimelineEntry};

use attachments::*;
use cmd::*;
use contacts::*;
use fs_store::{
    check_parent_safe, check_symlink_safe, config_dir, enforce_file_perms, enforce_safe_parents,
    ensure_dir_secure, ensure_store_layout, fsync_dir_best_effort, lock_store_exclusive,
    lock_store_shared, normalize_profile, probe_dir_writable, read_ack_mode_state,
    read_policy_profile, write_atomic, write_config_key,
};
use handshake::{
    hs_kem_keypair, hs_sig_keypair,
};
use identity::{
    identities_dir, identity_fingerprint_from_identity, identity_fingerprint_single,
    identity_pin_matches_seen, identity_pin_matches_seen_identity, identity_read_peer_kem_pk,
    identity_read_pin, identity_read_self_public, identity_read_sig_pin,
    identity_rotate_kem_keypair, identity_rotate_sig_keypair, identity_secret_store,
    identity_self_kem_keypair, identity_sig_secret_store, identity_write_public_record,
    FpRole, IdentityKeypair,
};
use model::*;
use output::{
    CliError, CliResult,
    emit_cli_named_marker, emit_marker, emit_tui_named_marker, print_marker, };
use protocol_state::{
    kmac_out, protocol_active_or_reason_for_peer,
    protocol_inactive_error, qsp_send_ready_tuple,
    qsp_session_load, qsp_session_store,
    qsp_session_store_with_trigger, record_qsp_status,
    QspTriggerState, SendOrigination,
};
use relay::*;
use store::*;
use timeline::{
    apply_file_peer_confirmation,
    apply_message_peer_confirmation, emit_cli_confirm_policy, emit_cli_delivery_state_with_device,
    emit_cli_file_delivery_with_device, emit_cli_receipt_ignored_wrong_device,
    emit_message_state_reject,

    file_delivery_short_id, file_transfer_confirm_id,
    file_transfer_upsert_outbound_record, latest_outbound_file_id,
    timeline_append_entry, timeline_append_entry_for_target, ConfirmApplyOutcome, MessageState,
};

static VAULT_UNLOCKED_THIS_RUN: AtomicBool = AtomicBool::new(false);

pub fn set_vault_unlocked(unlocked: bool) {
    VAULT_UNLOCKED_THIS_RUN.store(unlocked, Ordering::SeqCst);
}

pub fn vault_unlocked() -> bool {
    VAULT_UNLOCKED_THIS_RUN.load(Ordering::SeqCst)
}

pub(crate) fn cli_err(code: ErrorCode) -> CliError {
    CliError::code(code.as_str())
}

pub fn require_unlocked(op_name: &'static str) -> CliResult {
    if vault_unlocked() {
        return Ok(());
    }
    emit_marker(
        "error",
        Some("vault_locked"),
        &[("op", op_name), ("reason", "explicit_unlock_required")],
    );
    Err(CliError::Emitted)
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

fn normalize_relay_endpoint(value: &str) -> Result<String, &'static str> {
    adversarial::route::normalize_relay_endpoint(value)
}

pub fn identity_peer_status(peer: &str) -> (String, bool) {
    match identity_read_pin(peer) {
        Ok(Some(fp)) => (fp, true),
        Ok(None) => ("untrusted".to_string(), false),
        Err(_) => ("untrusted".to_string(), false),
    }
}

pub fn identity_show(self_label: &str) -> CliResult {
    let Some(rec) =
        identity_read_self_public(self_label).map_err(|e| CliError::code(e.as_str()))?
    else {
        emit_marker(
            "identity_show",
            None,
            &[("ok", "false"), ("reason", "missing_identity")],
        );
        return Err(CliError::code("identity_missing"));
    };
    // NA-0634 (D571 Decision 2a): the verification code binds BOTH identity keys (KEM + signing).
    let fp = identity_fingerprint_from_identity(&rec.kem_pk, &rec.sig_pk);
    emit_marker(
        "identity_show",
        None,
        &[("ok", "true"), ("fp", fp.as_str())],
    );
    output::emit_raw_payload_line(&format!("identity_fp={}", fp));
    // NA-0633 (ENG-0038): also emit the full identity KEM public key so a peer can provision it
    // (`contacts add --fp <this fp> --kem-pk <this>`) and thereby authenticate this side as the
    // handshake responder. The fingerprint stays the human-comparable element.
    output::emit_raw_payload_line(&format!("identity_kem_pk={}", hex_encode(&rec.kem_pk)));
    // NA-0634 (D571 Decision 2a): also emit the signing key so a peer provisions BOTH keys against the
    // single verification code (`contacts add --fp <fp> --kem-pk <kem> --sig-pk <sig>`).
    output::emit_raw_payload_line(&format!("identity_sig_pk={}", hex_encode(&rec.sig_pk)));
    Ok(())
}

pub fn identity_rotate(self_label: &str, confirm: bool, reset_peers: bool) -> CliResult {
    require_unlocked("identity_rotate")?;
    if !confirm {
        emit_marker(
            "identity_rotate",
            None,
            &[("ok", "false"), ("reason", "confirm_required")],
        );
        return Err(CliError::code("identity_rotate_confirm_required"));
    }
    let (dir, source) =
        fs_store::config_dir().map_err(|_| CliError::code(invite::INVITE_OWNERSHIP_UNAVAILABLE))?;
    let _ownership_lock = fs_store::lock_store_exclusive(&dir, source)
        .map_err(|_| CliError::code(invite::INVITE_OWNERSHIP_UNAVAILABLE))?;
    invite::retain_ownership().map_err(CliError::code)?;
    let (kem_pk, kem_sk) = match identity_rotate_kem_keypair() {
        Ok(v) => v,
        Err(e) => {
            emit_marker(
                "identity_secret_unavailable",
                Some(e),
                &[("reason", "rng_failure_forced")],
            );
            return Err(CliError::code("identity_secret_unavailable"));
        }
    };
    let (sig_pk, sig_sk) = match identity_rotate_sig_keypair() {
        Ok(v) => v,
        Err(e) => {
            emit_marker(
                "identity_secret_unavailable",
                Some(e),
                &[("reason", "rng_failure_forced")],
            );
            return Err(CliError::code("identity_secret_unavailable"));
        }
    };
    if identity_secret_store(self_label, &kem_sk).is_err() {
        emit_marker(
            "identity_secret_unavailable",
            None,
            &[("reason", "vault_missing_or_locked")],
        );
        return Err(CliError::code("identity_secret_unavailable"));
    }
    if identity_sig_secret_store(self_label, &sig_sk).is_err() {
        emit_marker(
            "identity_secret_unavailable",
            None,
            &[("reason", "vault_missing_or_locked")],
        );
        return Err(CliError::code("identity_secret_unavailable"));
    }
    if identity_write_public_record(self_label, &kem_pk, &sig_pk).is_err() {
        emit_marker(
            "identity_rotate",
            None,
            &[("ok", "false"), ("reason", "write_failed")],
        );
        return Err(CliError::code("identity_rotate_write_failed"));
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
    // NA-0634 (D571 Decision 2a): the verification code binds BOTH identity keys (KEM + signing).
    let fp = identity_fingerprint_from_identity(&kem_pk, &sig_pk);
    emit_marker(
        "identity_rotate",
        None,
        &[("ok", "true"), ("fp", fp.as_str())],
    );
    output::emit_raw_payload_line(&format!("identity_fp={}", fp));
    // NA-0633 (ENG-0038): emit the full identity KEM public key for peer provisioning (see identity_show).
    output::emit_raw_payload_line(&format!("identity_kem_pk={}", hex_encode(&kem_pk)));
    // NA-0634 (D571 Decision 2a): emit the signing key for full-identity peer provisioning.
    output::emit_raw_payload_line(&format!("identity_sig_pk={}", hex_encode(&sig_pk)));
    Ok(())
}

pub fn peers_list() -> CliResult {
    let mut peers = contacts_list_entries()
        .map_err(|_| CliError::code("contacts_store_unavailable"))?
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
        output::emit_raw_payload_line(&format!("peer={} fp={} status=pinned", peer, fp));
    }
    Ok(())
}

fn env_bool(key: &str) -> bool {
    matches!(
        env::var(key).ok().as_deref(),
        Some("1") | Some("true") | Some("TRUE") | Some("yes") | Some("YES")
    )
}

/// NA-0688 C4 (D622 R7): `config set` now accepts `ack-mode` alongside `policy-profile`.
///
/// ⚠ The CLI spelling is hyphenated (`ack-mode`) and the on-disk spelling is underscored
/// (`ack_mode`), which is not an inconsistency but the existing convention: `policy-profile` is
/// stored as `policy_profile` and reported in markers as `policy_profile`. The new key follows it
/// rather than inventing a second style.
pub fn config_set(key: &str, value: &str) -> CliResult {
    let (store_key, normalized) = match key {
        "policy-profile" => match normalize_profile(value) {
            Ok(v) => (POLICY_KEY, v),
            Err(e) => return Err(cli_err(e)),
        },
        // NA-0770 (D-1411) S-2: THE TOMBSTONE WRITER. The key is RETAINED here and REFUSES.
        //
        // ⚠ **IT REFUSES EVERY VALUE, `lease` INCLUDED.** With one behaviour there is nothing to
        // select, and accepting `lease` would leave a knob whose only position is the default —
        // an invitation to a future contributor to add a second.
        //
        // ⚠ **THE ERROR IS DISTINGUISHABLE FROM A TYPO, DELIBERATELY.** Falling through to the
        // generic `ParseFailed` arm below would emit the identical error `config set ack-mdoe`
        // produces, and "you mistyped a key" and "this key is retired" are different facts that
        // need different remedies.
        //
        // ⚠ **AND IT TELLS THE USER HOW TO CLEAR THE KEY, because measured, they cannot ask us
        // to.** `ConfigCmd` is exactly `Set` and `Get` — there is NO `config unset`, and
        // `write_config_key` only sets or replaces. Deleting `config.txt` wholesale would also
        // destroy `policy_profile`, which the multi-key read-modify-write exists to protect. So
        // the refusal names the file and the key and leaves the user able to act. A refusal that
        // strands the user is its own defect, and this lane will not create one while removing
        // others.
        "ack-mode" => {
            let (dir, _source) = match config_dir() {
                Ok(v) => v,
                Err(e) => return Err(cli_err(e)),
            };
            let path = dir.join(CONFIG_FILE_NAME);
            print_marker(
                "config_set_refused",
                &[
                    ("key", ACK_MODE_KEY),
                    ("reason", "retired_at_NA-0770"),
                    ("file", path.to_str().unwrap_or(CONFIG_FILE_NAME)),
                    ("remedy", "remove the ack_mode line from that file"),
                ],
            );
            return Err(CliError::code(fs_store::ACK_MODE_RETIRED_KEY_STATE));
        }
        _ => return Err(cli_err(ErrorCode::ParseFailed)),
    };

    let (dir, source) = match config_dir() {
        Ok(v) => v,
        Err(e) => return Err(cli_err(e)),
    };
    let file = dir.join(CONFIG_FILE_NAME);

    let _lock = match lock_store_exclusive(&dir, source) {
        Ok(v) => v,
        Err(e) => return Err(cli_err(e)),
    };
    if let Err(e) = ensure_store_layout(&dir, source) {
        return Err(cli_err(e));
    }
    // ⚠ Read-modify-write: setting one key must not delete the other.
    if let Err(e) = write_config_key(&file, store_key, &normalized, source) {
        return Err(cli_err(e));
    }

    print_marker(
        "config_set",
        &[("key", store_key), ("value", &normalized), ("ok", "true")],
    );
    Ok(())
}

pub fn config_get(key: &str) -> CliResult {
    let store_key = match key {
        "policy-profile" => POLICY_KEY,
        "ack-mode" => ACK_MODE_KEY,
        _ => return Err(cli_err(ErrorCode::ParseFailed)),
    };
    let (dir, source) = match config_dir() {
        Ok(v) => v,
        Err(e) => return Err(cli_err(e)),
    };
    let file = dir.join(CONFIG_FILE_NAME);

    if let Err(e) = enforce_safe_parents(&file, source) {
        return Err(cli_err(e));
    }
    let _lock = match lock_store_shared(&dir, source) {
        Ok(v) => v,
        Err(e) => return Err(cli_err(e)),
    };
    #[cfg(unix)]
    if file.exists() {
        if let Err(e) = enforce_file_perms(&file) {
            return Err(cli_err(e));
        }
    }

    // NA-0770 (D-1411) S-3: THE TOMBSTONE READER reports a THIRD STATE.
    //
    // ⚠ **"unset" AND "retired-key-present" ARE DIFFERENT ANSWERS AND MUST NOT SHARE A WORD.**
    // Reporting a file that carries `ack_mode=legacy` as `unset` would be false, and reporting
    // it as `legacy` would imply a mode that no longer exists. Both readings mislead; the third
    // state is the only honest one, and it is why the key was tombstoned rather than deleted.
    if store_key == ACK_MODE_KEY {
        let (value, state) = match read_ack_mode_state(&file) {
            Ok(fs_store::AckModeConfigState::RetiredKeyPresent(raw)) => (raw, "retired_present"),
            Ok(fs_store::AckModeConfigState::Nothing) => ("unset".to_string(), "absent"),
            Err(e) => return Err(cli_err(e)),
        };
        print_marker(
            "config_get",
            &[
                ("key", store_key),
                ("value", &value),
                ("state", state),
                ("retired", "true"),
                ("effect", "ignored"),
                ("ok", "true"),
            ],
        );
        return Ok(());
    }

    let value = match read_policy_profile(&file) {
        Ok(Some(v)) => v,
        Ok(None) => "unset".to_string(),
        Err(e) => return Err(cli_err(e)),
    };

    print_marker(
        "config_get",
        &[("key", store_key), ("value", &value), ("ok", "true")],
    );
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

pub fn doctor_check_only(check_only: bool, timeout_ms: u64, export: Option<PathBuf>) -> CliResult {
    if !check_only {
        return Err(cli_err(ErrorCode::ParseFailed));
    }
    let (dir, source) = match config_dir() {
        Ok(v) => v,
        Err(e) => return Err(cli_err(e)),
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
            return Err(cli_err(e));
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
    Ok(())
}

fn protocol_active_or_reason_for_send_peer(peer: &str) -> Result<(), String> {
    let routing = resolve_send_routing_target(peer).map_err(|code| code.to_string())?;
    protocol_active_or_reason_for_peer(routing.channel.as_str())
}

struct QspPackOutcome {
    envelope: Vec<u8>,
    /// NA-0624: SCKA control envelopes (advertisements) to push BEFORE `envelope`, in order.
    pre_envelopes: Vec<Vec<u8>>,
    next_state: Suite2SessionState,
    trigger: QspTriggerState,
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


const POLL_INTERVAL_MS_MAX: u64 = 60_000;
const POLL_TICKS_MAX: u32 = 64;
const POLL_MAX_PER_TICK_MAX: u32 = 32;
const META_TICK_COUNT_DEFAULT: u32 = 1;
const META_INTERVAL_MS_DEFAULT: u64 = 1_000;
const META_BATCH_MAX_COUNT_DEFAULT: u32 = 1;
const META_BUCKET_MAX_DEFAULT: usize = 4_096;
const META_BUCKET_MAX_CEILING: usize = 65_536;

pub struct MetaPollConfig {
    pub interval_ms: u64,
    pub ticks: u32,
    pub batch_max_count: usize,
    pub bucket_max: usize,
    pub deterministic: bool,
}



pub fn meta_poll_config_from_args(args: MetaPollArgs) -> Result<Option<MetaPollConfig>, &'static str> {
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

pub struct MetaPollArgs {
    pub deterministic_meta: bool,
    pub interval_ms: Option<u64>,
    pub poll_interval_ms: Option<u64>,
    pub ticks: Option<u32>,
    pub batch_max_count: Option<u32>,
    pub poll_max_per_tick: Option<u32>,
    pub bucket_max: Option<usize>,
    pub meta_seed: Option<u64>,
}

pub fn meta_bucket_for_len(orig_len: usize, bucket_max: usize) -> usize {
    let capped = orig_len.min(bucket_max).max(1);
    let mut bucket = 1usize;
    while bucket < capped {
        bucket = bucket.saturating_mul(2);
    }
    bucket.min(bucket_max)
}

type ReceiptControlPayload = adversarial::payload::ReceiptControlPayload;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReceiptEmitMode {
    Off,
    Batched,
    Immediate,
}

impl ReceiptEmitMode {
    pub fn as_str(self) -> &'static str {
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


}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FileConfirmEmitMode {
    Off,
    CompleteOnly,
}

impl FileConfirmEmitMode {
    pub fn as_str(self) -> &'static str {
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


}

#[derive(Clone, Copy, Debug)]
pub struct ReceiptPolicy {
    pub mode: ReceiptEmitMode,
    pub batch_window_ms: u64,
    pub jitter_ms: u64,
    pub file_confirm_mode: FileConfirmEmitMode,
}

impl Default for ReceiptPolicy {
    /// ⚠ NA-0688 C3 (R1b): delivery acks default **ON**, mode **Batched**, BOTH HALVES.
    ///
    /// ⚠ THIS COMMENT IS A BEHAVIOUR-ENCODER, NOT DECORATION. It previously carried NA-0682's
    /// deferral and the four findings that justified it, and every one of those findings has
    /// now been ANSWERED BY MEASUREMENT rather than by picking a value. Rewriting it to the
    /// new truth is part of the flip; leaving it would have left the file arguing against its
    /// own code.
    ///
    /// What each of NA-0682's four deferral findings turned into:
    ///   1. "the ack CONSUMES the DH ratchet-on-reply boundary" — TRUE, and measured: before
    ///      passivation an ack originated `qsp_dh_ratchet dir=send reason=reply`. C2 closed
    ///      it: a control send originates no ROTATION, and does not count toward the N/T
    ///      cadence either (the counter was a second, quieter channel). It still ESTABLISHES
    ///      its own chain if it has none — a necessity, reported `reason=first_send`.
    ///   2. "a PQ RESEED per received message" — closed by the same suppression;
    ///      `boundaries_since_reseed` only advances on a rotation an ack no longer takes.
    ///   3. "every receive produces a send" — TRUE and UNCHANGED. Receipts are coalesced into
    ///      the end-of-pull flush, so it is one send per PULL rather than per message, and
    ///      there is no wall-clock deferral in v1. Stated honestly rather than mitigated.
    ///   4. "envelope shape differs" — MEASURED from the relay's stored bytes, three arms,
    ///      each drained separately so every number is labelled rather than positional:
    ///      **ack 1024 · SHORT user reply (20-byte body) 1024 · LONG user reply (4096-byte
    ///      body) 17682.** An ack is ALWAYS padded up to the Standard 1024 floor; a user message
    ///      is UNBUCKETED, so it coincides with the floor only while its body fits under it and
    ///      takes its natural size otherwise. **So the two are distinguishable by size for any
    ///      message that does not fit under the floor**, and the prescribed remedy cannot close
    ///      it: the ack is already the padded one, and no amount of padding a receipt makes it
    ///      resemble an unbucketed message of arbitrary size. Only bucketing the USER path
    ///      would — see ENG-0098. ⚠ A user send that also mints an SCKA advertisement emits
    ///      TWO envelopes (1320 + 1024) where an ack emits one, so envelope COUNT is a second
    ///      distinguishing signal alongside size. Recorded, not papered over.
    ///
    /// ⚠ **BOTH HALVES ARE ON — AND THIS FIELD IS ONLY THE RECIPIENT-HONOURS HALF.**
    /// The sender-requests half lives in `resolve_sender_receipt_request`, which consults THIS
    /// policy, so turning receipts off here turns off both asking and answering.
    ///
    /// ⚠ HOW THE SENDER HALF WAS NEARLY SHIPPED BROKEN, kept because the failure shape is the
    /// reusable part. It was first flipped by giving `RelayMessageSender::new` a new default —
    /// which is where D622 §1b.4 located it — and MEASUREMENT showed that value never reached
    /// the wire: `qsc send` builds its sender with `.with_meta(…, receipt)`, and `with_meta`
    /// assigns the caller's choice UNCONDITIONALLY, so an absent `--receipt` overwrote the new
    /// default microseconds after it was set. `qsc outbox retry` and `qsc outbox discard`, which
    /// do not call `with_meta`, DID inherit it. Both halves were pinned in isolation — a unit
    /// test on the constructor's field, integration tests on the recipient's behaviour — and
    /// nothing asserted that the constructor's value survives to the WIRE.
    ///
    /// The evidence was a NEGATIVE result from an instrument that could have gone positive:
    /// ENG-0087 instance #4 carries a sentinel NA-0686 recorded as firing red BY DESIGN if this
    /// default flipped, and under the full flip it **did not fire**.
    ///
    /// Ruled (STOP #016 option (a)) and closed: an absent `--receipt` means the policy default,
    /// resolved at every construction site by one function. `na0688_c3_sender_default` pins it
    /// end-to-end via the peer's ack rather than any sender-side field.
    fn default() -> Self {
        Self {
            mode: ReceiptEmitMode::Batched,
            batch_window_ms: RECEIPT_BATCH_WINDOW_MS_DEFAULT,
            jitter_ms: RECEIPT_JITTER_MS_DEFAULT,
            file_confirm_mode: FileConfirmEmitMode::CompleteOnly,
        }
    }
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

/// NA-0770 (D-1411): THE ACK-MODE TOMBSTONE, IN ONE PLACE.
///
/// There is no longer a mode to resolve. `AckMode::Legacy` — delete-on-pull — is retired, so
/// every pull carries the lease shape by construction and no caller chooses anything. What
/// remains here is the one job the retirement created: **detecting a config file that still
/// carries the retired `ack_mode` key, and saying so.**
///
/// ⚠⚠ **WHY THIS IS A FUNCTION AND NOT A DELETION — the defect a deletion would have created.**
/// The pre-lane resolution read `stored_ack_mode().unwrap_or(AckMode::Lease)`, and
/// `stored_ack_mode` turned BOTH an `Err` (through `.ok()?`) AND an unrecognised value
/// (through `_ => None`) into `None`. So there were three distinct ways for a machine
/// configured `ack_mode=legacy` to end up silently on Lease, and **deleting the key's handling
/// would simply have been a fourth.** A two-state return cannot express "the key is present
/// but retired"; only a third state can. That is [`fs_store::AckModeConfigState`].
///
/// ⚠ **THE CLAIM THIS PROTECTS IS PRIVACY-RELEVANT, NOT MERELY TIDY.** Whoever wrote
/// `ack_mode=legacy` asked for "the relay keeps nothing, even briefly". Lease holds an unacked
/// frame for the lease window — measured at `PULL_LEASE_SECS=60` on the deployed relay. Moving
/// a user from the first to the second without telling them is a silent change to a setting
/// they chose. The lane removes the mode; it does not get to remove the notice.
///
/// ⚠ **ABLE-TO-CONFIGURE AND CURRENTLY-CONFIGURED ARE DIFFERENT CLAIMS.** Before this lane
/// **every** user could write that key — `qsc config set ack-mode legacy` succeeded. What was
/// measured is narrower: **no field machine currently carries such a config**, on two machines,
/// on the operator's word (2026-08-28). The population able to configure it was everyone; the
/// population configured for it was zero.
///
/// Returns the notice-worthy state, or `Nothing` when the config is absent or unreadable. ⚠ An
/// unreadable config yields `Nothing` rather than an error, unchanged from before: resolving a
/// transport preference must never break an unrelated `receive`. That is a deliberate
/// asymmetry — a missing notice is a lesser harm than a broken command — and it is stated
/// rather than left to be discovered.
fn ack_mode_config_state() -> fs_store::AckModeConfigState {
    let Ok((dir, _source)) = config_dir() else {
        return fs_store::AckModeConfigState::Nothing;
    };
    read_ack_mode_state(&dir.join(CONFIG_FILE_NAME))
        .unwrap_or(fs_store::AckModeConfigState::Nothing)
}

/// S-8: the retired-key state ANNOUNCES ITSELF on `qsc`'s own output.
///
/// ⚠ **THIS IS THE WHOLE POINT OF THE TOMBSTONE AND IT MUST NOT BECOME SILENT.** A tombstone
/// that behaves like a deletion is not a tombstone. Called at the head of every command that
/// previously resolved an ack mode.
///
/// ⚠ SCOPE, stated so it is not overread: the desktop's notice line is a FUTURE consumer and is
/// out of this lane (zero desktop bytes). This surfaces on `qsc`'s own output only.
pub(crate) fn announce_retired_ack_mode_key() {
    if let fs_store::AckModeConfigState::RetiredKeyPresent(raw) = ack_mode_config_state() {
        emit_marker(
            fs_store::ACK_MODE_RETIRED_KEY_STATE,
            None,
            &[
                ("key", ACK_MODE_KEY),
                ("value", raw.as_str()),
                ("file", CONFIG_FILE_NAME),
                ("effect", "ignored"),
            ],
        );
    }
}

pub fn load_receipt_policy_from_account() -> ReceiptPolicy {
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

fn receipt_kind_str(kind: ReceiptKind) -> &'static str {
    match kind {
        ReceiptKind::Delivered => "delivered",
    }
}


fn encode_receipt_data_payload(
    payload: Vec<u8>,
    receipt: Option<ReceiptKind>,
) -> CliResult<(Vec<u8>, Option<String>)> {
    let Some(kind) = receipt else {
        return Ok((payload, None));
    };
    // NA-0682 (D617 F1): a 128-bit CSPRNG id, NOT `sha512(plaintext)[..8]`.
    //
    // ⚠ The derived id was a correctness AND a privacy defect: two identical messages to
    // the same peer shared an id, so an ack flipped the wrong row and DESIGN §4's own dedup
    // rule would have discarded the second copy as a duplicate; and because the id is a
    // fingerprint of the body, the one unredacted emission site turned it into a
    // plaintext-confirmation oracle. A random id closes both.
    let msg_id = crate::msgqueue::mint_msg_id();
    let ctrl = ReceiptControlPayload {
        v: CTRL_VERSION,
        t: "data".to_string(),
        kind: receipt_kind_str(kind).to_string(),
        msg_id: msg_id.clone(),
        body: Some(payload),
        ns: Some(adversarial::payload::CTRL_NS.to_string()),
    };
    let encoded =
        serde_json::to_vec(&ctrl).map_err(|_| CliError::code("receipt_encode_failed"))?;
    Ok((encoded, Some(msg_id)))
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


fn build_delivered_ack(msg_id: &str) -> CliResult<Vec<u8>> {
    let ack = ReceiptControlPayload {
        v: CTRL_VERSION,
        t: "ack".to_string(),
        kind: "delivered".to_string(),
        msg_id: msg_id.to_string(),
        body: None,
        ns: Some(adversarial::payload::CTRL_NS.to_string()),
    };
    serde_json::to_vec(&ack).map_err(|_| CliError::code("receipt_encode_failed"))
}

#[derive(Clone, Debug)]
enum PendingReceipt {
    AttachmentComplete {
        attachment_id: String,
        confirm_handle: String,
    },
}


pub(crate) fn flush_owed_receipts(peer: &str, relay: &str) {
    if !owed_receipts::any_owed(peer) {
        return;
    }
    let owed = match owed_receipts::take_for_peer(peer) {
        Ok(v) => v,
        Err(code) => {
            // Vault locked between receive and send: degrade like msgqueue ("unlock to send"),
            // never fail the send itself over a receipt.
            emit_marker("receipt_flush_deferred", Some(code), &[("code", code)]);
            return;
        }
    };
    let mut sent = 0usize;
    for msg_id in owed {
        match send_delivered_receipt_ack(relay, peer, &msg_id) {
            Ok(()) => {
                sent += 1;
                emit_marker(
                    "receipt_send",
                    None,
                    &[
                        ("kind", "delivered"),
                        ("bucket", "small"),
                        ("msg_id", "<redacted>"),
                        ("held", "true"),
                    ],
                );
            }
            Err(_) => {
                // Put it back. Losing it here would reintroduce exactly the drop this store
                // exists to prevent.
                let _ = owed_receipts::record(peer, &msg_id);
            }
        }
    }
    if sent > 0 {
        let n = sent.to_string();
        emit_marker("receipt_flush", None, &[("count", n.as_str())]);
    }
}

fn send_pending_receipt(ctx: &ReceivePullCtx<'_>, item: PendingReceipt) -> CliResult {
    match item {
        PendingReceipt::AttachmentComplete {
            attachment_id,
            confirm_handle,
        } => {
            let payload = build_attachment_completion_ack(&attachment_id, &confirm_handle)?;
            let outcome = transport::relay_send_with_payload(RelaySendPayloadArgs {
                to: ctx.from,
                payload,
                relay: ctx.relay,
                injector: transport::fault_injector_from_env()?,
                bucket_max: None,
                meta_seed: None,
                receipt: None,
                routing_override: None,
                // ⚠ A RECEIPT. Machine traffic: originates nothing, counts toward nothing.
                origination: SendOrigination::Control,
            })?;
            if let Some(code) = outcome.error_code {
                emit_marker(
                    "attachment_confirm_send_failed",
                    Some(code),
                    &[("code", code)],
                );
                return Ok(());
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
    Ok(())
}

fn flush_batched_receipts(ctx: &ReceivePullCtx<'_>, queue: &mut Vec<PendingReceipt>) -> CliResult {
    if ctx.receipt_policy.mode != ReceiptEmitMode::Batched || queue.is_empty() {
        return Ok(());
    }
    // Deterministic ordering; jitter only affects stable sort priority.
    queue.sort_by_key(|item| match item {
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
        send_pending_receipt(ctx, item)?;
    }
    Ok(())
}

// NA-0646 (D582) PR-B: receipt sends fail SOFT (the caller emits *_send_failed and
// continues) except the encode step, which was a fatal funnel exit. From impls route
// both through the existing `?` sites unchanged.
fn send_delivered_receipt_ack(relay: &str, to: &str, msg_id: &str) -> CliResult {
    let payload = build_delivered_ack(msg_id)?;
    // ⚠ ENG-0095 — THE ORDER OF THE NEXT FOUR STATEMENTS IS A CRYPTO INVARIANT.
    //
    // This used to pack, push, and only THEN commit. A push failure therefore abandoned a
    // PACKED receipt whose ratchet advance was never durable, and the next send on the chain
    // was handed the same message key back -- two plaintexts under one AEAD key if the
    // abandoned ciphertext reached the relay (push sent, response lost: the common path).
    // The failure is SOFT, so nothing reported it and `flush_batched_receipts` carried on.
    //
    // MEASURED, not argued: `na0688_eng0095_ack_nonce_barrier` was RED on the old order,
    // both arms of a single-variable experiment landing on `msg_idx=0`.
    //
    // The rule `msgqueue::retire_packed` enforces on the queue path, now enforced here:
    // nothing abandons a packed message without first committing its ratchet advance.
    //   1. route token FIRST -- fallible, and must not sit between pack and commit;
    //   2. pack;
    //   3. COMMIT, fail-closed -- a failed commit attempts NO push;
    //   4. only then push.
    // A push failure now BURNS the index: same semantics as the user send path, absorbed by
    // the recipient's skipped-key machinery, and self-healing once lease is the default (C4).
    let route_token = relay_peer_route_token(to).map_err(CliError::code)?;
    let pack = qsp_pack(to, &payload, None, SendOrigination::Control)
        .map_err(|e| CliError::code(e.code))?;
    qsp_session_store_with_trigger(to, &pack.next_state, &pack.trigger)
        .map_err(|_| CliError::code("qsp_session_store_failed"))?;
    for pre in pack.pre_envelopes.iter() {
        transport::relay_inbox_push(relay, route_token.as_str(), pre).map_err(CliError::code)?;
    }
    transport::relay_inbox_push(relay, route_token.as_str(), &pack.envelope).map_err(CliError::code)?;
    Ok(())
}






/// NA-0622 (ENG-0012 Stage 1b-ii): wall-clock seconds for the bounded DH-ratchet time fallback.
fn qsp_now_unix_secs() -> u64 {
    #[cfg(not(feature = "na0780-test-hooks"))]
    return std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs();
    #[cfg(feature = "na0780-test-hooks")]
    {
    // NA-0688 C1 (R4a): delegates to the ONE clock. See `crate::clock`.
    //
    // ⚠ This is the sixth and last of the private clocks C1 consolidated, and the only one
    // that feeds a CRYPTO cadence rather than a policy deadline: it drives
    // `QSP_DH_FALLBACK_T_SECS` and `QSP_PQ_RESEED_T_SECS`. Pinning the clock therefore makes
    // the ratchet's time-fallback deterministic in a test, which is what C2's
    // deferred-rotation guards will need.
    crate::clock::now_unix_s()
    }
}


// Obsolete stateless entry points cannot operate the directional vault transaction.
fn qsp_pack(
    _channel: &str,
    _plaintext: &[u8],
    _meta_seed: Option<u64>,
    _origination: SendOrigination,
) -> Result<QspPackOutcome, QspPackError> {
    Err(QspPackError { code: "directional_entry_required", reason: None })
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

fn short_peer_marker(peer: &str) -> String {
    let all_hex = peer.chars().all(|ch| ch.is_ascii_hexdigit());
    if all_hex && peer.len() >= 32 {
        peer.chars().take(12).collect()
    } else {
        peer.to_string()
    }
}

pub struct ReceiveArgs {
    pub transport: Option<SendTransport>,
    pub relay: Option<String>,
    pub legacy_receive_mode: Option<LegacyReceiveMode>,
    pub attachment_service: Option<String>,
    pub from: Option<String>,
    pub mailbox: Option<String>,
    pub max: Option<usize>,
    pub max_file_size: Option<usize>,
    pub max_file_chunks: Option<usize>,
    pub out: Option<PathBuf>,
    pub deterministic_meta: bool,
    pub interval_ms: Option<u64>,
    pub poll_interval_ms: Option<u64>,
    pub poll_ticks: Option<u32>,
    pub batch_max_count: Option<u32>,
    pub poll_max_per_tick: Option<u32>,
    pub bucket_max: Option<usize>,
    pub meta_seed: Option<u64>,
    pub emit_receipts: Option<ReceiptKind>,
    pub receipt_mode: Option<ReceiptMode>,
    pub receipt_batch_window_ms: Option<u64>,
    pub receipt_jitter_ms: Option<u64>,
    pub file_confirm_mode: Option<FileConfirmMode>,
}

struct ReceivePullCtx<'a> {
    relay: &'a str,
    attachment_service: Option<&'a str>,
    mailbox: &'a str,
    from: &'a str,
    out: &'a Path,
    source: ConfigSource,
    cfg_dir: &'a Path,
    cfg_source: ConfigSource,
    receipt_policy: ReceiptPolicy,
}

struct ReceivePullStats {
    count: usize,
    bytes: usize,
}

pub fn receive_file(path: &Path) -> CliResult {
    require_unlocked("receive_file")?;
    let (dir, source) = match config_dir() {
        Ok(v) => v,
        Err(e) => return Err(cli_err(e)),
    };
    // Fail-closed: reject if config dir parents or symlinks are unsafe.
    if !check_symlink_safe(&dir) {
        return Err(cli_err(ErrorCode::UnsafePathSymlink));
    }
    if !check_parent_safe(&dir, source) {
        return Err(cli_err(ErrorCode::UnsafeParentPerms));
    }

    let bytes = match fs::read(path) {
        Ok(v) => v,
        Err(_) => return Err(cli_err(ErrorCode::IoReadFailed)),
    };
    if bytes.is_empty() {
        emit_marker("recv_reject", None, &[("reason", "empty")]);
        return Err(CliError::code("recv_reject_parse"));
    }
    if bytes.len() > envelope::MAX_BUNDLE_SIZE_DEFAULT {
        emit_marker("recv_reject", None, &[("reason", "oversize")]);
        return Err(CliError::code("recv_reject_size"));
    }

    emit_marker("recv_reject", None, &[("reason", "malformed")]);
    Err(CliError::code("recv_reject_parse"))
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

type HttpRelayTarget = adversarial::route::HttpRelayTarget;
type HttpRequestParsed = adversarial::route::HttpRequestParsed;

pub struct RelaySendOutcome {
    // D581 KEEP -> NA-0646 (D582): pub GUI-surface fields (send outcome for the GUI);
    // dormant until the GUI consumes them.
    #[allow(dead_code)]
    pub action: String,
    #[allow(dead_code)]
    pub delivered: bool,
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

#[derive(Clone, Deserialize, Serialize)]
struct InboxPullItem {
    id: String,
    data: Vec<u8>,
}

#[derive(Deserialize, Serialize)]
struct InboxPullResp {
    items: Vec<InboxPullItem>,
}

// NA-0644 (D580): the acknowledged-pull wire pair (POST /v1/pull/ack). Route-scoped,
// idempotent, deletes only leased copies server-side; <= RELAY_ACK_MAX_IDS ids per POST.
#[derive(Deserialize, Serialize)]
struct AckReq {
    ids: Vec<String>,
}

#[derive(Deserialize, Serialize)]
struct AckResp {
    acked: usize,
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

fn channel_label_ok(label: &str) -> bool {
    !label.is_empty()
        && label
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_' || c == '#')
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

static FAULT_IDX: AtomicU64 = AtomicU64::new(0);

struct RelaySendPayloadArgs<'a> {
    to: &'a str,
    payload: Vec<u8>,
    relay: &'a str,
    injector: Option<FaultInjector>,
    bucket_max: Option<usize>,
    meta_seed: Option<u64>,
    receipt: Option<ReceiptKind>,
    routing_override: Option<SendRoutingTarget>,
    /// ⚠ NA-0688 C2: this path carries BOTH user messages and the attachment-completion
    /// receipt, so the caller must say which it is. There is deliberately no default.
    origination: SendOrigination,
}

pub fn util_receipt_apply(
    peer: &str,
    channel: &str,
    msg_id: Option<String>,
    file_id: Option<String>,
    confirm_id: Option<String>,
) -> CliResult {
    if !env_bool("QSC_TEST_MODE") {
        return Err(CliError::code("test_mode_required"));
    }
    if !channel_label_ok(peer) || !channel_label_ok(channel) {
        return Err(CliError::code("qsp_channel_invalid"));
    }
    emit_cli_confirm_policy();
    match (msg_id.as_deref(), file_id.as_deref(), confirm_id.as_deref()) {
        (Some(msg), None, None) => match apply_message_peer_confirmation(peer, msg, channel) {
            Ok((ConfirmApplyOutcome::IgnoredWrongDevice, _)) => {
                let dev = channel_device_marker(channel);
                emit_cli_receipt_ignored_wrong_device(peer, dev.as_str());
                Ok(())
            }
            Ok((ConfirmApplyOutcome::Confirmed, target)) => {
                let device = target.as_deref().or_else(|| channel_device_id(channel));
                emit_cli_delivery_state_with_device(peer, "peer_confirmed", device);
                Ok(())
            }
            Err(code) => Err(CliError::code(code)),
        },
        (None, Some(file), Some(confirm)) => {
            let file_id = if file == "latest" {
                latest_outbound_file_id(peer).map_err(CliError::code)?
            } else {
                file.to_string()
            };
            let confirm_id = if confirm == "auto" {
                file_transfer_confirm_id(peer, file_id.as_str())
                    .map_err(CliError::code)?
            } else {
                confirm.to_string()
            };
            match apply_file_peer_confirmation(peer, file_id.as_str(), confirm_id.as_str(), channel)
            {
                Ok((ConfirmApplyOutcome::IgnoredWrongDevice, _)) => {
                    let dev = channel_device_marker(channel);
                    emit_cli_receipt_ignored_wrong_device(peer, dev.as_str());
                    Ok(())
                }
                Ok((ConfirmApplyOutcome::Confirmed, target)) => {
                    let device = target.as_deref().or_else(|| channel_device_id(channel));
                    emit_cli_file_delivery_with_device(
                        peer,
                        "peer_confirmed",
                        file_id.as_str(),
                        device,
                    );
                    Ok(())
                }
                Err(code) => Err(CliError::code(code)),
            }
        }
        _ => Err(CliError::code("receipt_apply_invalid_args")),
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct QueueFull;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RetryExhausted;

pub struct BoundedQueue<T> {
    max: usize,
    items: VecDeque<T>,
}

impl<T> BoundedQueue<T> {
    pub fn new(max: usize) -> Self {
        Self {
            max,
            items: VecDeque::new(),
        }
    }

    pub fn push(&mut self, item: T) -> Result<(), QueueFull> {
        if self.items.len() >= self.max {
            return Err(QueueFull);
        }
        self.items.push_back(item);
        Ok(())
    }
}

pub fn bounded_retry<F>(mut attempts: u32, mut op: F) -> Result<u32, RetryExhausted>
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
                    return Err(RetryExhausted);
                }
                let jitter = (tried as u64 % (RETRY_JITTER_MS + 1)).min(RETRY_JITTER_MS);
                let sleep_ms = (backoff + jitter).min(RETRY_MAX_MS);
                std::thread::sleep(Duration::from_millis(sleep_ms));
                backoff = (backoff * 2).min(RETRY_MAX_MS);
            }
        }
    }
    Err(RetryExhausted)
}

pub fn util_envelope(
    tick_count: usize,
    interval_ms: u64,
    max_ticks: usize,
    max_bundle: usize,
    max_count: usize,
    payload_lens: Vec<usize>,
) -> CliResult {
    let ticks = match envelope::tick_schedule(tick_count, interval_ms, max_ticks) {
        Ok(v) => v,
        Err(e) => return Err(CliError::code(e.code())),
    };
    let bundle = match envelope::pack_bundle(&payload_lens, max_bundle, max_count) {
        Ok(v) => v,
        Err(e) => return Err(CliError::code(e.code())),
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
    Ok(())
}

pub fn envelope_plan_ack(
    deterministic: bool,
    tick_count: usize,
    interval_ms: u64,
    max_ticks: usize,
    max_bundle: usize,
    max_count: usize,
    small_len: usize,
) -> CliResult {
    if !deterministic {
        return Err(CliError::code("ack_plan_requires_deterministic"));
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
        Err(e) => return Err(CliError::code(e.code())),
    };
    let tick = plan.ticks.first().copied().unwrap_or(0);
    let tick_s = tick.to_string();
    let bucket_s = plan.bundle.bucket_len.to_string();
    print_marker(
        "ack_plan",
        &[("size_class", bucket_s.as_str()), ("tick", tick_s.as_str())],
    );
    Ok(())
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

// ---------------------------------------------------------------------------
// NA-0682 (D617 §2h / §2i) — the message-queue surface.
//
// ⚠ §2i: these return / emit STRUCTURED state, not parsed marker text. Today's
// `timeline_list` prints markers and `TimelineEntry`'s fields are `pub(super)`, so a GUI
// can call it but cannot read it -- Slice 4 would have had to parse stdout. This is the
// shape that does not repeat that.
// ---------------------------------------------------------------------------

/// Per-contact queue state, as data. The GUI consumes this; the CLI renders it.
pub fn outbox_summary() -> CliResult<Vec<msgqueue::ContactQueueSummary>> {
    require_unlocked("outbox_status")?;
    let (dir, _source) = config_dir().map_err(cli_err)?;
    msgqueue::summarize_at(&dir, msgqueue::now_unix_s()).map_err(CliError::code)
}

/// `qsc outbox status` — the honest one-line status per contact.
///
/// ⚠ §2h is CLAIMS-HONESTY, not UX. v1 has NO BACKGROUND DAEMON: messages move only while
/// the app is open and the vault unlocked, and a locked vault means the queue is PAUSED
/// because the store key lives in the vault. So a paused queue says what to do about it
/// ("unlock to send") and an unreachable relay says it will send later -- neither may read
/// as work in progress. A paused outbox that looks like a sending one is a FALSE CLAIM.
/// NA-0689 P4: list quarantined items as **redacted metadata only**.
///
/// ⚠ **FIELD NAMES, NEVER VALUES.** `reason` and `site` are our own diagnostic constants, not
/// user or peer data; `bytes` is a length. **No captured content is printed, and none is
/// printable** — `QuarantineSummary` carries no accessor for the stored bytes at all, so this is
/// a property of the type rather than a discipline of this function.
pub fn quarantine_list() -> CliResult {
    require_unlocked("quarantine_list")?;
    let (dir, _source) = match config_dir() {
        Ok(v) => v,
        Err(e) => return Err(cli_err(e)),
    };
    let items = quarantine::list(&dir).map_err(CliError::code)?;
    let n_s = items.len().to_string();
    emit_marker("quarantine_list", None, &[("count", n_s.as_str())]);
    for it in &items {
        let ts_s = it.captured_at_unix.to_string();
        let bytes_s = it.bytes.to_string();
        emit_marker(
            "quarantine_item",
            None,
            &[
                ("id", it.entry_id.as_str()),
                // ⚠ Both discriminators are shown because neither implies the other
                // (D-1328 Rulings 2 and 7): subclass says WHY it was kept, content says WHAT
                // the bytes are. A reader given one of them alone cannot infer the other.
                ("subclass", it.subclass),
                ("content", it.content),
                ("reason", it.reason.as_str()),
                ("site", it.site.as_str()),
                ("captured_at", ts_s.as_str()),
                ("bytes", bytes_s.as_str()),
            ],
        );
    }
    // Stated unconditionally, so the boundary is never inferable only from silence.
    emit_marker(
        "quarantine_limitation",
        None,
        &[
            ("content_readable", "false"),
            ("reingestion", "not_supported"),
        ],
    );
    Ok(())
}

/// NA-0689 P4: delete one quarantined item by id.
///
/// **A stored item must always be deletable** — otherwise this lane would have traded
/// "destroyed without consent" for "kept without consent".
pub fn quarantine_drop(id: &str) -> CliResult {
    require_unlocked("quarantine_drop")?;
    let (dir, _source) = match config_dir() {
        Ok(v) => v,
        Err(e) => return Err(cli_err(e)),
    };
    quarantine::drop_entry(&dir, id).map_err(CliError::code)?;
    Ok(())
}

pub fn outbox_status() -> CliResult {
    let summaries = outbox_summary()?;
    let total: usize = summaries.iter().map(|s| s.queued).sum();
    let total_s = total.to_string();
    let n_s = summaries.len().to_string();
    emit_marker(
        "outbox_status",
        None,
        &[
            ("contacts", n_s.as_str()),
            ("queued_total", total_s.as_str()),
        ],
    );
    // Stated once, unconditionally, so the limitation is never inferable only from silence.
    emit_marker(
        "outbox_limitation",
        None,
        &[
            ("background_daemon", "false"),
            ("sends_while", "app_open_and_vault_unlocked"),
        ],
    );
    for s in summaries.iter() {
        let q = s.queued.to_string();
        let sent = s.sent.to_string();
        let del = s.delivered.to_string();
        let failed = s.failed.to_string();
        let line = s.honest_line().unwrap_or_else(|| "idle".to_string());
        emit_marker(
            "outbox_contact",
            None,
            &[
                ("peer_key", s.peer_key.as_str()),
                ("queued", q.as_str()),
                ("sent", sent.as_str()),
                ("delivered", del.as_str()),
                ("failed", failed.as_str()),
                ("paused", s.paused.map(|c| c.as_str()).unwrap_or("none")),
                ("status", line.as_str()),
            ],
        );
    }
    Ok(())
}

/// `qsc outbox retry` — the manual "Retry now" trigger (DESIGN §2).
///
/// ⚠ F3: this is the DRAIN ENTRY POINT, not a loop. Slice 3 ships the callable and the
/// trigger vocabulary; Slice 4 owns the timer that calls it on unlock, settings-save,
/// manual retry, and after any successful send.
pub fn outbox_retry(relay: &str) -> CliResult {
    require_unlocked("outbox_retry")?;
    let (dir, source) = config_dir().map_err(cli_err)?;
    let mut sender = transport::RelayMessageSender::new(relay);
    let out = msgqueue::drain_at(
        &dir,
        source,
        msgqueue::DrainTrigger::ManualRetry,
        msgqueue::now_unix_s(),
        &mut sender,
    )
    .map_err(CliError::code)?;
    let (a, s, p, f, q) = (
        out.attempted.to_string(),
        out.sent.to_string(),
        out.paused.to_string(),
        out.failed.to_string(),
        out.still_queued.to_string(),
    );
    emit_marker(
        "outbox_drain",
        None,
        &[
            ("trigger", msgqueue::DrainTrigger::ManualRetry.as_str()),
            ("attempted", a.as_str()),
            ("sent", s.as_str()),
            ("paused", p.as_str()),
            ("failed", f.as_str()),
            ("still_queued", q.as_str()),
        ],
    );
    Ok(())
}

/// `qsc outbox discard` — ⚠ DESTROY one specifically-identified queued message.
///
/// ⚠ F2: recovery means DRAIN OR FAIL VISIBLY, NEVER DESTROY. This is deliberately off the
/// generic recovery path, requires naming the exact message, and requires `--confirm`.
///
/// ⚠ It routes through `msgqueue::discard_at`, which commits the ratchet advance BEFORE
/// dropping the bytes. A plain delete here would be NONCE REUSE: the next pack would reuse
/// the abandoned message key, and if that ciphertext reached the relay (push sent, response
/// lost) two ciphertexts would exist under one key.
pub fn outbox_discard(to: &str, msg_id: &str, relay: &str, confirm: bool) -> CliResult {
    require_unlocked("outbox_discard")?;
    if !confirm {
        emit_marker(
            "error",
            Some("outbox_discard_confirm_required"),
            &[("reason", "explicit_confirm_required")],
        );
        return Err(CliError::code("outbox_discard_confirm_required"));
    }
    let (dir, _source) = config_dir().map_err(cli_err)?;
    let mut sender = transport::RelayMessageSender::new(relay);
    msgqueue::discard_at(&dir, to, msg_id, &mut sender).map_err(CliError::code)?;
    emit_marker(
        "outbox_discard",
        None,
        &[
            ("ok", "true"),
            ("action", "burned"),
            ("msg_id", "<redacted>"),
        ],
    );
    Ok(())
}

#[cfg(test)]
mod message_state_tests {
    use super::timeline::{message_state_transition_allowed, MessageState};

    /// NA-0682 (D617 F6, DEFERRED 2026-07-28 — operator Condition 4). ⚠ PIN THE DEFAULT.
    ///
    /// The ack MECHANISM ships in this lane; the ON-BY-DEFAULT FLIP does not. That decision is
    /// a **wire-behaviour** decision, not a style one — turning acks on consumes the DH
    /// ratchet-on-reply boundary and triggers a PQ reseed per received message — so the default
    /// is pinned by a test rather than left to whoever next edits the struct literal.
    ///
    /// ⚠ This asserts the RECIPIENT-HONOURS half. The SENDER-REQUESTS half is pinned in
    /// `transport::receipt_sender_default_tests`; a half-flip would leave the wire noisy while
    /// the feature looked disabled, so both are pinned separately and deliberately.
    ///
    /// If a future lane flips this ON, this test SHOULD go red — that is the point. Flip it in
    /// the same commit, with the measurement the flip ENG requires.
    #[test]
    fn receipt_default_is_batched_recipient_half() {
        // NA-0688 C3 (R1b) — MIGRATED, not rewritten down. This pin was
        // `receipt_default_is_off_recipient_half` and asserted `Off`; it was DESIGNED to go red
        // when the flip landed, and it did. The property it defends is unchanged — the
        // recipient-honours default must be pinned so it cannot drift silently — only the
        // pinned value moved, and the name now states what is true.
        //
        // ⚠ `Batched`, not `Immediate`: receipts are coalesced into the end-of-pull flush
        // (R2a as amended). `Immediate` would put a send on the wire per received message.
        assert_eq!(
            super::ReceiptPolicy::default().mode,
            super::ReceiptEmitMode::Batched,
            "delivery acks are ON (Batched) by default as of NA-0688 C3"
        );
    }

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

// Isolated experiment process-cut seams: debug test builds only, no auth/crypto
// override. A cut exits before the next external side effect.
pub(crate) fn directional_cut(_point:&str) {
    #[cfg(feature = "na0780-test-hooks")]
    if std::env::var("QSC_NA0780_CUT").ok().as_deref()==Some(_point) { std::process::exit(86); }
}
#[cfg(feature = "na0780-test-hooks")]
#[doc(hidden)]
pub fn na0780_test_stale_generation(peer:&str,generation:u64)->Result<(),&'static str>{
    protocol_state::directional_update(peer,Some(generation),|_|Ok(()))
}

// Deterministic test rendezvous after the actual queue push, before commit's lock.
fn directional_commit_pause()->Result<(),&'static str>{
    #[cfg(feature = "na0780-test-hooks")]
    if let Ok(base)=std::env::var("QSC_NA0780_COMMIT_GATE") {
        let base=std::path::PathBuf::from(base);
        std::fs::write(base.with_extension("ready"),b"ready").map_err(|_|"test_gate_io")?;
        let deadline=std::time::Instant::now()+std::time::Duration::from_secs(240);
        while !base.with_extension("release").exists() {
            if std::time::Instant::now()>=deadline{return Err("test_gate_timeout");}
            std::thread::sleep(std::time::Duration::from_millis(20));
        }
    }
    Ok(())
}

#[cfg(feature = "na0780-test-hooks")]
#[doc(hidden)]
pub fn na0780_test_commit_probe(peer:&str,body:&[u8],raw:Vec<u8>)->Result<(),&'static str>{
    use msgqueue::MessageSender;
    let(dir,_)=config_dir().map_err(|_|"directional_store")?;
    let mut rec=msgqueue::load_contact(&dir,peer)?.into_iter().find(|r|r.body==body).ok_or("directional_queue_missing")?;
    rec.ciphertext=Some(raw);rec.channel=Some(resolve_send_routing_target(peer)?.channel);
    transport::RelayMessageSender::new("http://127.0.0.1").commit(&rec)
}
#[cfg(feature = "na0780-test-hooks")]
#[doc(hidden)]
pub fn na0780_test_receive_probe(peer:&str,raw:&[u8])->Result<(),&'static str>{
    protocol_state::directional_update(peer,None,|state|state.receive(peer,raw).map(|_|()))
}
#[cfg(feature = "na0780-test-hooks")]
#[doc(hidden)]
pub fn na0780_test_receive_response(peer:&str,raw:&[u8])->Result<Option<Vec<u8>>,&'static str>{
    protocol_state::directional_receive_update(peer,peer,raw).map_err(protocol_state::DirectionalUpdateError::code)
}
#[cfg(feature = "na0780-test-hooks")]
#[doc(hidden)]
pub fn na0780_test_queue_contains(peer:&str,body:&[u8])->Result<bool,&'static str>{
    let(dir,source)=config_dir().map_err(|_|"directional_store")?;
    let _lock=lock_store_shared(&dir,source).map_err(vault::store_err_marker)?;
    let count=msgqueue::load_contact(&dir,peer)?.into_iter().filter(|r|r.body==body).count();
    if count>1{return Err("test_duplicate_queue_payload");}
    Ok(count==1)
}

// Isolated acceptance seams: normal locked transaction preparation, and hostile
// authenticated frames from a disposable sender snapshot. The latter never saves
// a state or bypasses the receiver's authentication/selection checks.
#[cfg(feature = "na0780-test-hooks")]
#[doc(hidden)]
pub fn na0780_test_prepare(peer:&str,id:&str,body:&[u8],now:u64,advertise:bool,maintenance:bool)->Result<Vec<u8>,&'static str>{
    protocol_state::directional_update(peer,None,|state|state.prepare(id,body,now,advertise,maintenance))
}
#[cfg(feature = "na0780-test-hooks")]
#[doc(hidden)]
pub fn na0780_test_hostile_wire(snapshot:&str,mode:&str,id:u32)->Result<Vec<u8>,&'static str>{
    use directional_core::{Core,ectx,k,lp,MAX_SKIP};
    let mut core:Core=serde_json::from_str(snapshot).map_err(|_|"test_snapshot")?;
    match mode {
        "spent" => {
            let(pk,_)=quantumshield_refimpl::crypto::stdcrypto::runtime_pq_kem_keypair();
            core.craft_boundary(Some((id,pk)),b"negative spent target").map(|(_,raw)|raw)
        }
        "adv" => {
            let(pk,_)=quantumshield_refimpl::crypto::stdcrypto::runtime_pq_kem_keypair();
            let epoch=core.send.as_ref().ok_or("SEND_UNSET")?;
            let mut data=ectx(epoch.id,epoch.dir,&epoch.dh);data.extend(id.to_be_bytes());data.extend(lp(&pk));
            let mut body=id.to_be_bytes().to_vec();body.extend(lp(&pk));body.extend(k(&core.sid,&epoch.adv,"ADV_AUTH",&data));
            core.ordinary(2,&body)
        }
        "gap" | "terminal" => {
            if mode=="gap" {
                for _ in 0..=MAX_SKIP {core.send.as_mut().ok_or("SEND_UNSET")?.step(&core.sid)?;}
            }
            // Existing NDI1 closure-only body, with no promises.
            let mut body=b"NDI1".to_vec();body.push(1);body.extend(lp(b""));body.push(0);body.extend(lp(&[2]));
            core.ordinary(0,&body)
        }
        _=>Err("test_mode"),
    }
}

#[cfg(feature = "na0780-test-hooks")]
#[doc(hidden)]
pub fn na0780_test_seal_observer(reset:bool)->[usize;5] {
    if reset {directional_core::seal_observer::reset();}
    directional_core::seal_observer::stats()
}
#[cfg(feature = "na0780-test-hooks")]
#[doc(hidden)]
pub fn na0780_test_seal_observer_controls() {
    use directional_core::{seal,seal_observer as o};
    // Synthetic key only; this never enters a client store or transport.
    o::reset();
    {let _attempt=o::begin();seal(&[37;32],&[0;12],b"ad",b"one").unwrap();}
    {let attempt=o::begin();seal(&[37;32],&[0;12],b"ad",b"two").unwrap();attempt.committed();}
    assert_eq!(o::stats(),[2,1,1,0,0],"aborted computation is not exposure");
    {let attempt=o::begin();seal(&[37;32],&[0;12],b"ad",b"one").unwrap();attempt.committed();}
    assert_eq!(o::stats(),[3,2,2,1,0],"conflicting committed use must be detected");
    o::disable();
}


pub(crate) fn directional_single_channel(peer: &str, channel: &str) -> Result<(), &'static str> {
    if peer.is_empty() || peer.contains('#') || channel != peer {
        return Err("directional_single_channel_required");
    }
    Ok(())
}
fn directional_routing_target(peer: &str) -> Result<contacts::SendRoutingTarget, &'static str> {
    directional_single_channel(peer, peer)?;
    let route = contacts::resolve_send_routing_target(peer)?;
    directional_single_channel(peer, &route.channel)?;
    Ok(route)
}

#[cfg(test)]
mod directional_channel_tests {
    use super::*;

    #[test]
    fn directional_rejects_alias_channel_divergence_before_transaction_access() {
        assert_eq!(directional_single_channel("alice", "alice"), Ok(()));
        for (alias, channel) in [("alice", "bob"), ("alice", "alice#device"), ("alice#device", "alice#device"), ("", "")] {
            assert_eq!(directional_single_channel(alias, channel), Err("directional_single_channel_required"));
            let error = protocol_state::directional_receive_update(channel, alias, b"untrusted frame").unwrap_err();
            assert!(!error.expected_non_admission());
            assert_eq!(error.code(), "directional_single_channel_required");
        }
    }
}

// The first-release directional profile has fixed framing and mandatory receipts.
// Validate explicit requests before reading payloads, creating directories or
// mutating queues. Absence selects the profile, never the old padding defaults.
fn directional_send_options(
    pad_to: Option<usize>,
    pad_bucket: Option<MetaPadBucket>,
    bucket_max: Option<usize>,
    meta_seed: Option<u64>,
    receipt: Option<ReceiptRequest>,
) -> Result<(), &'static str> {
    if pad_to.is_some() || pad_bucket.is_some() {
        return Err("directional_padding_unsupported");
    }
    if bucket_max.is_some() || meta_seed.is_some() {
        return Err("directional_send_metadata_unsupported");
    }
    if matches!(receipt, Some(ReceiptRequest::Off)) {
        return Err("directional_receipts_required");
    }
    Ok(())
}

fn directional_receive_options(args: &ReceiveArgs) -> Result<(), &'static str> {
    if matches!(args.legacy_receive_mode, Some(LegacyReceiveMode::Coexistence)) {
        return Err("directional_legacy_receive_unsupported");
    }
    if args.attachment_service.is_some() || args.max_file_size.is_some()
        || args.max_file_chunks.is_some() || args.file_confirm_mode.is_some()
    {
        return Err("directional_attachments_unsupported");
    }
    if args.bucket_max.is_some() {
        return Err("directional_receive_bucketing_unsupported");
    }
    if args.meta_seed.is_some() {
        return Err("directional_receive_seed_unsupported");
    }
    if matches!(args.receipt_mode, Some(ReceiptMode::Off | ReceiptMode::Batched))
        || args.receipt_batch_window_ms.is_some() || args.receipt_jitter_ms.is_some()
    {
        return Err("directional_receipt_policy_unsupported");
    }
    // Explicit Delivered/Immediate select the already mandatory exact receipt path.
    // Poll intervals/ticks/counts and deterministic pacing are consumed by the
    // existing bounded scheduler. No file/resource policy is silently discarded.
    Ok(())
}

// The current directional payload path projects opaque messages and has no
// attachment descriptor/file assembly consumer. Refuse the attachment operation
// at entry, before reading files, staging or creating an upload session.
fn directional_attachment_preflight() -> CliResult {
    Err(CliError::code("directional_attachments_unsupported"))
}

fn directional_receipt_account_preflight() -> CliResult {
    for (key, supported) in [
        (TUI_RECEIPT_MODE_SECRET_KEY, Some("immediate")),
        (TUI_RECEIPT_BATCH_WINDOW_MS_SECRET_KEY, None),
        (TUI_RECEIPT_JITTER_MS_SECRET_KEY, None),
        (TUI_FILE_CONFIRM_MODE_SECRET_KEY, None),
    ] {
        let raw = vault::secret_get(key).map_err(CliError::code)?;
        if let Some(raw) = raw {
            if supported != Some(raw.trim()) {
                return Err(CliError::code("directional_receipt_policy_unsupported"));
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod na0780_semantic_option_tests {
    use super::*;

    fn receive_args() -> ReceiveArgs {
        ReceiveArgs {
            transport: None, relay: None, legacy_receive_mode: None,
            attachment_service: None, from: None, mailbox: None, max: None,
            max_file_size: None, max_file_chunks: None, out: None,
            deterministic_meta: false, interval_ms: None, poll_interval_ms: None,
            poll_ticks: None, batch_max_count: None, poll_max_per_tick: None,
            bucket_max: None, meta_seed: None, emit_receipts: None,
            receipt_mode: None, receipt_batch_window_ms: None,
            receipt_jitter_ms: None, file_confirm_mode: None,
        }
    }

    #[test]
    fn default_send_and_delivered_request_use_fixed_directional_profile() {
        assert_eq!(directional_send_options(None, None, None, None, None), Ok(()));
        assert_eq!(directional_send_options(None, None, None, None, Some(ReceiptRequest::Delivered)), Ok(()));
        assert_eq!(directional_send_options(None, None, None, None, Some(ReceiptRequest::Off)), Err("directional_receipts_required"));
    }

    #[test]
    fn every_explicit_padding_request_refuses_including_old_standard_default() {
        for len in [0, 1, 65536, usize::MAX] {
            assert_eq!(directional_send_options(Some(len), None, None, None, None), Err("directional_padding_unsupported"));
        }
        for profile in [MetaPadBucket::Standard, MetaPadBucket::Enhanced, MetaPadBucket::Private, MetaPadBucket::Auto] {
            assert_eq!(directional_send_options(None, Some(profile), None, None, None), Err("directional_padding_unsupported"));
        }
        assert_eq!(directional_send_options(None, None, Some(4096), None, None), Err("directional_send_metadata_unsupported"));
        assert_eq!(directional_send_options(None, None, None, Some(0), None), Err("directional_send_metadata_unsupported"));
    }

    #[test]
    fn receive_default_and_supported_poll_options_remain_accepted() {
        let mut args = receive_args();
        assert_eq!(directional_receive_options(&args), Ok(()));
        args.legacy_receive_mode = Some(LegacyReceiveMode::Retired);
        args.receipt_mode = Some(ReceiptMode::Immediate);
        args.emit_receipts = Some(ReceiptKind::Delivered);
        args.interval_ms = Some(10); args.poll_ticks = Some(2);
        args.batch_max_count = Some(1); args.deterministic_meta = true;
        assert_eq!(directional_receive_options(&args), Ok(()));
    }

    #[test]
    fn receive_unsupported_options_refuse_at_entry_before_vault_and_output_effects() {
        let cases: Vec<Box<dyn Fn(&mut ReceiveArgs)>> = vec![
            Box::new(|a| a.legacy_receive_mode = Some(LegacyReceiveMode::Coexistence)),
            Box::new(|a| a.attachment_service = Some("invalid".into())),
            Box::new(|a| a.max_file_size = Some(1)),
            Box::new(|a| a.max_file_chunks = Some(1)),
            Box::new(|a| a.bucket_max = Some(1)),
            Box::new(|a| a.meta_seed = Some(0)),
            Box::new(|a| a.receipt_mode = Some(ReceiptMode::Off)),
            Box::new(|a| a.receipt_mode = Some(ReceiptMode::Batched)),
            Box::new(|a| a.receipt_batch_window_ms = Some(0)),
            Box::new(|a| a.receipt_jitter_ms = Some(0)),
            Box::new(|a| a.file_confirm_mode = Some(FileConfirmMode::Off)),
        ];
        for set in cases {
            let mut args = receive_args(); set(&mut args);
            let expected = directional_receive_options(&args).unwrap_err();
            let err = transport::receive_execute(args).err().expect("must refuse before missing transport/vault");
            assert!(matches!(err, CliError::Code(ref code) if code == expected));
        }
    }

    #[test]
    fn queue_capacity_error_preserves_exact_items_including_zero_capacity() {
        let mut empty = BoundedQueue::new(0);
        assert_eq!(empty.push(9), Err(QueueFull)); assert!(empty.items.is_empty());
        let mut queue = BoundedQueue::new(2);
        assert_eq!(queue.push(1), Ok(())); assert_eq!(queue.push(2), Ok(()));
        assert_eq!(queue.push(3), Err(QueueFull));
        assert_eq!(queue.items.into_iter().collect::<Vec<_>>(), vec![1,2]);
    }

    #[test]
    fn retry_preserves_zero_limit_counts_and_last_attempt_success() {
        let mut calls = 0;
        assert_eq!(bounded_retry(0, || { calls += 1; Ok(()) }), Err(RetryExhausted));
        assert_eq!(calls, 0);
        assert_eq!(bounded_retry(1, || { calls += 1; Err(()) }), Err(RetryExhausted));
        assert_eq!(calls, 1);
        calls = 0;
        assert_eq!(bounded_retry(3, || { calls += 1; if calls == 3 { Ok(()) } else { Err(()) } }), Ok(3));
        assert_eq!(calls, 3);
        calls = 0;
        assert_eq!(bounded_retry(3, || { calls += 1; Err(()) }), Err(RetryExhausted));
        assert_eq!(calls, 3);
    }
}
