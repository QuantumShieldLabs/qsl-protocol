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
    Hash, Kmac, PqKem768, PqSigMldsa65, X25519Dh, X25519Priv, X25519Pub,
};
use quantumshield_refimpl::qse::{Envelope, EnvelopeProfile};
use quantumshield_refimpl::suite2::establish::init_from_base_handshake;
use quantumshield_refimpl::suite2::ratchet::Suite2RecvWireState;
use quantumshield_refimpl::suite2::ratchet::{
    recv_dh_boundary, recv_pq_adv_session, recv_pq_reseed, send_boundary, send_pq_advertise,
    send_pq_reseed,
};
use quantumshield_refimpl::suite2::state::Suite2SessionState;
use quantumshield_refimpl::suite2::types::{
    FLAG_BOUNDARY, FLAG_PQ_ADV, FLAG_PQ_CTXT, SUITE2_PROTOCOL_VERSION, SUITE2_SUITE_ID,
};
use quantumshield_refimpl::suite2::{decode_suite2_wire_canon, recv_wire_canon, send_wire_canon};
use quantumshield_refimpl::RefimplError;
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
pub mod fs_store;
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
pub mod transport;
pub mod vault;

pub(crate) use timeline::{timeline_ts_default, TimelineEntry};

use attachments::*;
use cmd::*;
use contacts::*;
use fs_store::{
    check_parent_safe, check_symlink_safe, config_dir, enforce_file_perms, enforce_safe_parents,
    ensure_dir_secure, ensure_store_layout, fsync_dir_best_effort, lock_store_exclusive,
    lock_store_shared, normalize_ack_mode, normalize_profile, probe_dir_writable, read_ack_mode,
    read_policy_profile, write_atomic, write_config_key,
};
use handshake::{
    hs_kem_keypair, hs_sig_keypair,
};
use identity::{
    identities_dir, identity_fingerprint_from_identity, identity_fingerprint_from_pk,
    identity_marker_display, identity_pin_matches_seen, identity_read_peer_kem_pk,
    identity_read_pin, identity_read_self_public, identity_read_sig_pin,
    identity_rotate_kem_keypair, identity_rotate_sig_keypair, identity_secret_store,
    identity_self_kem_keypair, identity_sig_secret_store, identity_write_public_record,
    IdentityKeypair, IDENTITY_FP_PREFIX,
};
use model::*;
use output::{
    CliError, CliResult,
    emit_cli_named_marker, emit_marker, emit_tui_named_marker, print_marker, };
use protocol_state::{
    kmac_out, protocol_active_or_reason_for_peer,
    protocol_inactive_error, qsp_scka_load, qsp_scka_store, qsp_send_ready_tuple,
    qsp_session_for_channel, qsp_session_load, qsp_session_store,
    qsp_session_store_with_trigger, qsp_trigger_load, record_qsp_status,
    zero32, QspTriggerState, SckaLocalState, SckaPeerAdv, SendOrigination, QSP_DH_FALLBACK_N,
    QSP_DH_FALLBACK_T_SECS, QSP_PQ_RESEED_N, QSP_PQ_RESEED_T_SECS,
};
use relay::*;
use store::*;
use timeline::{
    apply_attachment_peer_confirmation, apply_file_peer_confirmation,
    apply_message_peer_confirmation, emit_cli_confirm_policy, emit_cli_delivery_state_with_device,
    emit_cli_file_delivery_with_device, emit_cli_receipt_ignored_wrong_device,
    emit_message_state_reject, emit_tui_delivery_state_with_device,
    emit_tui_file_delivery_with_device, emit_tui_receipt_ignored_wrong_device,
    file_delivery_short_id, file_transfer_confirm_id,
    file_transfer_upsert_outbound_record, latest_outbound_file_id,
    timeline_append_entry, timeline_append_entry_for_target, timeline_store_load, timeline_store_save, ConfirmApplyOutcome, MessageState,
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
    println!("identity_fp={}", fp);
    // NA-0633 (ENG-0038): also emit the full identity KEM public key so a peer can provision it
    // (`contacts add --fp <this fp> --kem-pk <this>`) and thereby authenticate this side as the
    // handshake responder. The fingerprint stays the human-comparable element.
    println!("identity_kem_pk={}", hex_encode(&rec.kem_pk));
    // NA-0634 (D571 Decision 2a): also emit the signing key so a peer provisions BOTH keys against the
    // single verification code (`contacts add --fp <fp> --kem-pk <kem> --sig-pk <sig>`).
    println!("identity_sig_pk={}", hex_encode(&rec.sig_pk));
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
    println!("identity_fp={}", fp);
    // NA-0633 (ENG-0038): emit the full identity KEM public key for peer provisioning (see identity_show).
    println!("identity_kem_pk={}", hex_encode(&kem_pk));
    // NA-0634 (D571 Decision 2a): emit the signing key for full-identity peer provisioning.
    println!("identity_sig_pk={}", hex_encode(&sig_pk));
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
        println!("peer={} fp={} status=pinned", peer, fp);
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
        "ack-mode" => match normalize_ack_mode(value) {
            Ok(v) => (ACK_MODE_KEY, v),
            Err(e) => return Err(cli_err(e)),
        },
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

    let read = if store_key == ACK_MODE_KEY {
        read_ack_mode(&file)
    } else {
        read_policy_profile(&file)
    };
    let value = match read {
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

struct QspUnpackOutcome {
    plaintext: Vec<u8>,
    next_state: Suite2SessionState,
    trigger: QspTriggerState,
    msg_idx: u32,
    skip_delta: usize,
    evicted: usize,
    /// NA-0624: an SCKA control message (peer advertisement) — commit state, but there is no
    /// application payload (the frozen receiver has no ADV body decrypt path).
    is_control: bool,
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

pub struct MetaPollConfig {
    pub interval_ms: u64,
    pub ticks: u32,
    pub batch_max_count: usize,
    pub bucket_max: usize,
    pub deterministic: bool,
}

#[derive(Clone, Copy)]
pub struct MetaPadConfig {
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

    fn from_arg(value: ReceiptMode) -> Self {
        match value {
            ReceiptMode::Off => Self::Off,
            ReceiptMode::Batched => Self::Batched,
            ReceiptMode::Immediate => Self::Immediate,
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

    fn from_arg(value: FileConfirmMode) -> Self {
        match value {
            FileConfirmMode::Off => Self::Off,
            FileConfirmMode::CompleteOnly => Self::CompleteOnly,
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

/// THE ACK-MODE RULE, IN ONE PLACE — NA-0688 C4 (D622). **Every production pull resolves its
/// `AckMode` here**, whether it came from `--ack-mode` or from no flag at all.
///
/// ⚠ **THE DEFAULT IS NOW `Lease`, AND THAT IS THE POINT OF C4.** Under the previous `Legacy`
/// default the relay DELETES ON PULL, so anything a pull collected but could not process was
/// destroyed with no witness and no way back — while the command reported success. Under
/// `Lease` the relay holds the item until it is acked after a durable persist, so an item that
/// was pulled collaterally is redelivered rather than lost.
///
/// ⚠ **This mitigates the TRIGGER, it does not fix the underlying defect.** The defect is that
/// a pull path can collect an item it will not process; the general remedy is
/// quarantine-instead-of-drop, which is a separate owed lane. C4 only removes the destruction
/// that made the defect unrecoverable.
///
/// | input | result | why |
/// |---|---|---|
/// | `Some(mode)` | that mode, verbatim | an explicit `--ack-mode legacy` is the escape hatch and beats everything |
/// | `None` + `ack_mode` set in `config.txt` | the stored mode | the per-install choice, for paths that take no flag |
/// | `None` + nothing stored | `Lease` | the new default |
///
/// ⚠ **THE PREFERENCE LIVES IN THE CONFIG FILE, NOT THE VAULT (D622 R7).** An ack mode is not a
/// secret, so the vault buys nothing — and it would cost something real: a vault-backed preference
/// is unreadable while the vault is locked, so the user's persistent choice would silently fail to
/// apply on some invocations and not others, with no witness. That is the exact silent-divergence
/// class this lane exists to remove, so the store that cannot exhibit it was chosen. Vault keys
/// remain the pattern for actual secrets (relay tokens, CA paths).
///
/// An unreadable or malformed config falls back to the default rather than failing the command:
/// resolving a transport preference must not be able to break an unrelated `receive`.
fn resolve_ack_mode(explicit: Option<AckMode>) -> AckMode {
    if let Some(mode) = explicit {
        return mode;
    }
    stored_ack_mode().unwrap_or(AckMode::Lease)
}

fn stored_ack_mode() -> Option<AckMode> {
    let (dir, _source) = config_dir().ok()?;
    let raw = read_ack_mode(&dir.join(CONFIG_FILE_NAME)).ok()??;
    match raw.as_str() {
        "legacy" => Some(AckMode::Legacy),
        "lease" => Some(AckMode::Lease),
        _ => None,
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

/// THE SENDER-SIDE RULE, IN ONE PLACE — NA-0688 C3 (D622 R1b; operator ruling on STOP #016,
/// option (a)). **Every production `RelayMessageSender` gets its receipt request from here.**
///
/// ⚠ WHY IT HAS TO BE ONE FUNCTION, AND WHY IT LIVES HERE RATHER THAN IN `with_meta`.
/// The C3 flip was first written as a new default on `RelayMessageSender::new`, and MEASUREMENT
/// showed it never reached the wire: `qsc send` builds its sender with `.with_meta(…, receipt)`,
/// and `with_meta` assigns the caller's choice UNCONDITIONALLY, so an absent `--receipt`
/// overwrote the new default microseconds after it was set. Meanwhile `qsc outbox retry` and
/// `qsc outbox discard`, which do NOT call `with_meta`, DID inherit it — so the very same queued
/// row could go out with or without a receipt request depending on which command drained it.
///
/// The fix is NOT to make `with_meta` conditional. `with_meta` takes the caller's choice
/// **verbatim in both directions**, that contract is pinned, and it is what lets a caller
/// deliberately disable a receipt. The fix is to resolve the caller's choice BEFORE handing it
/// over, at every construction site, through this one function — so "absent" cannot mean
/// different things on different paths.
///
/// | input | result | why |
/// |---|---|---|
/// | `None` (no flag) | the POLICY default | R1b: the sender half is only ON if a default send actually asks |
/// | `Some(Off)` | `None` | explicit off is verbatim and beats the policy |
/// | `Some(Delivered)` | `Some(Delivered)` | explicit on is verbatim and beats the policy |
///
/// ⚠ **The policy consulted is the SAME `ReceiptPolicy` the recipient half honours**, so a user
/// who turns receipts off persistently turns off both asking and answering with one setting —
/// rather than discovering that a switch labelled "delivery receipts" only moved one half.
/// `load_receipt_policy_from_account` returns the compiled-in default when the vault is locked,
/// so this is safe to call before unlock and in unit tests.
///
/// ⚠ **RESIDUAL, recorded not solved — and NARROWER than it first looked.** This resolves per
/// INVOCATION and a queued row carries no field for the caller's choice, so the obvious worry is
/// that an explicit `--receipt off` is forgotten by a later retry. **Measurement says otherwise
/// for the normal case:** `msgqueue::attempt_one` packs a record **at most once in its life** and
/// every later attempt REPLAYS the same bytes verbatim (a crypto-safety invariant — re-packing
/// would burn a second message key), and `receipt_kind` is consumed at PACK time. So the caller's
/// choice is already persisted, as packed ciphertext rather than as a field, and a retry cannot
/// change it.
///
/// The gap is only this: a record whose FIRST PACK FAILED is still unpacked when a retry runs, so
/// that retry resolves against the policy and an explicit `off` would be lost. Filed under
/// ENG-0096, where the row schema gains fields anyway.
pub(crate) fn resolve_sender_receipt_request(explicit: Option<ReceiptRequest>) -> Option<ReceiptKind> {
    match explicit {
        Some(ReceiptRequest::Off) => None,
        Some(ReceiptRequest::Delivered) => Some(ReceiptKind::Delivered),
        None => match load_receipt_policy_from_account().mode {
            ReceiptEmitMode::Off => None,
            ReceiptEmitMode::Batched | ReceiptEmitMode::Immediate => Some(ReceiptKind::Delivered),
        },
    }
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


/// NA-0682: wrap a body in the data control envelope using a CALLER-SUPPLIED `msg_id`.
///
/// ⚠ The id must be the MESSAGE QUEUE RECORD's `msg_id`, not a fresh mint. That is what
/// makes the delivery-ack correlate to the queued row: the peer echoes this id back, and
/// the sender flips exactly that record SENT -> DELIVERED. Minting a second id here would
/// leave the ack pointing at nothing.
pub(crate) fn encode_data_payload_with_id(
    payload: Vec<u8>,
    kind: ReceiptKind,
    msg_id: &str,
) -> CliResult<Vec<u8>> {
    let ctrl = ReceiptControlPayload {
        v: CTRL_VERSION,
        t: "data".to_string(),
        kind: receipt_kind_str(kind).to_string(),
        msg_id: msg_id.to_string(),
        body: Some(payload),
        ns: Some(adversarial::payload::CTRL_NS.to_string()),
    };
    serde_json::to_vec(&ctrl).map_err(|_| CliError::code("receipt_encode_failed"))
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

fn parse_receipt_payload(plaintext: &[u8]) -> Option<ReceiptControlPayload> {
    adversarial::payload::parse_receipt_payload(plaintext)
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

/// The DATA-ENVELOPE receipt obligation, honoured independently of what the inner body turned
/// out to be — NA-0688 C3 (D622; operator ruling on STOP #018, Gate 2 item 1).
///
/// ⚠ WHY THIS EXISTS AS ITS OWN CALL. Under transparent framing the envelope is unwrapped before
/// the typed-payload dispatch runs, and the typed branches (`attachment_descriptor`,
/// `file_chunk`/`file_manifest`, the two confirms) all `continue` **before** the generic
/// user-message path — which is where the delivery receipt used to be queued. Re-dispatching
/// without this call would have silently dropped the receipt for every wrapped typed payload:
/// the sender would sit on SENT forever for exactly the messages that carry a file.
///
/// **The rule, ruled rather than improvised: acking is INDEPENDENT of inner dispatch.** If the
/// envelope asked for a receipt, the receipt is owed, whatever the body turned out to be — a
/// manifest, a confirm, an ack, or an ordinary message. `request_msg_id` is the ENVELOPE's id,
/// never anything read out of the body.
fn queue_envelope_receipt(
    ctx: &ReceivePullCtx<'_>,
    queue: &mut Vec<PendingReceipt>,
    request_receipt: bool,
    request_msg_id: &str,
) -> CliResult {
    if request_receipt && !request_msg_id.is_empty() {
        queue_or_send_receipt(
            ctx,
            queue,
            PendingReceipt::Message {
                msg_id: request_msg_id.to_string(),
            },
        )?;
    }
    Ok(())
}

fn queue_or_send_receipt(
    ctx: &ReceivePullCtx<'_>,
    queue: &mut Vec<PendingReceipt>,
    item: PendingReceipt,
) -> CliResult {
    let kind = match item {
        PendingReceipt::Message { .. } => "message",
        PendingReceipt::FileComplete { .. } => "file_complete",
        PendingReceipt::AttachmentComplete { .. } => "attachment_complete",
    };
    // NA-0682 (D617 F6): with acks ON by default, we now attempt them for every received
    // message -- including from peers we have no route BACK to.
    //
    // ⚠ Do not attempt an ack that cannot be sent. A one-way contact (added without a route
    // token, or still pending) has no reverse route, so the attempt can only ever fail. It
    // failed SOFTLY and non-fatally, but it emitted `receipt_send_failed
    // code=QSC_ERR_CONTACT_ROUTE_TOKEN_REQUIRED` into the receive stream on every message --
    // noise that says nothing a caller can act on, and that a substring-based secret scan
    // reasonably flags because the CODE NAME contains "TOKEN".
    //
    // Skipping quietly is both quieter and more correct: an ack we structurally cannot
    // deliver is not a failure to report, it is a thing not to attempt. The sender simply
    // stays at SENT, which is the honest state -- we have no way to tell them otherwise.
    if matches!(item, PendingReceipt::Message { .. }) && relay_peer_route_token(ctx.from).is_err() {
        emit_marker(
            "receipt_skipped",
            None,
            &[("reason", "no_reverse_route"), ("kind", kind)],
        );
        return Ok(());
    }
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
            send_pending_receipt(ctx, item)?;
        }
        ReceiptEmitMode::Batched => {
            queue.push(item);
            emit_cli_receipt_policy_event(ctx.receipt_policy.mode, "queued", kind, ctx.from);
            emit_tui_receipt_policy_event(ctx.receipt_policy.mode, "queued", kind, ctx.from);
        }
    }
    Ok(())
}

/// Is our SENDING chain to this peer still unseeded (i.e. we have never sent to them)?
///
/// ⚠ Read-only, and it must stay that way — this is consulted on the receive path purely to
/// decide whether an ack can ride at all. A missing session reads as "unseeded", which is the
/// conservative answer: it defers the receipt rather than attempting a send that cannot work.
fn qsp_send_chain_unseeded(peer: &str) -> bool {
    match qsp_session_for_channel(peer) {
        Ok(st) => zero32(&st.send.ck_ec) || zero32(&st.send.ck_pq),
        Err(_) => true,
    }
}

/// Flush every receipt owed to a peer, now that our sending chain exists.
///
/// ⚠ CALLED FROM THE SEND PATH, AND THAT COUPLING IS INTRINSIC RATHER THAN INCIDENTAL: the first
/// real send is the exact moment an owed receipt becomes sendable, because it is the thing that
/// establishes the chain. The consult is read-only from the send path's point of view apart from
/// the removal, and the common case (nothing owed) costs one read and no write.
///
/// A receipt that still cannot be sent is RE-RECORDED rather than dropped — a failed flush must
/// not be a silent loss, which is the whole reason the hold exists.
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
        PendingReceipt::Message { msg_id } => {
            // ⚠ NA-0688 — THE DEFERRAL, AND IT IS THE REASON THE A6 REVERSAL IS SAFE.
            //
            // With A6 reversed an ack can no longer establish a chain, so a message from a peer
            // we have never sent to has nowhere to ride. Dropping it here was MEASURED to lose
            // the first receipt of every conversation — the sender sits on SENT forever. Instead
            // the obligation is written down durably and flushed on our first real send.
            //
            // The check is cheap and read-only in the common case: an established chain skips it
            // entirely.
            if qsp_send_chain_unseeded(ctx.from) {
                match owed_receipts::record(ctx.from, &msg_id) {
                    Ok(()) => {
                        emit_marker(
                            "receipt_owed",
                            None,
                            &[("reason", "chain_unseeded"), ("msg_id", "<redacted>")],
                        );
                        return Ok(());
                    }
                    // ⚠ A locked vault is a PAUSE, not a failure — the same degrade msgqueue
                    // uses. The receipt stays owed in spirit but unrecorded; say so rather than
                    // failing the receive, which would strand the MESSAGE as well as the ack.
                    Err(code) => {
                        emit_marker("receipt_owed_failed", Some(code), &[("code", code)]);
                        return Ok(());
                    }
                }
            }
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
                Err(ReceiptSendError::Soft(code)) => {
                    emit_marker("receipt_send_failed", Some(code), &[("code", code)])
                }
                Err(ReceiptSendError::Fatal(e)) => return Err(e),
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
                Err(ReceiptSendError::Soft(code)) => {
                    emit_marker("file_confirm_send_failed", Some(code), &[("code", code)])
                }
                Err(ReceiptSendError::Fatal(e)) => return Err(e),
            }
        }
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
                pad_cfg: None,
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
        send_pending_receipt(ctx, item)?;
    }
    Ok(())
}

// NA-0646 (D582) PR-B: receipt sends fail SOFT (the caller emits *_send_failed and
// continues) except the encode step, which was a fatal funnel exit. From impls route
// both through the existing `?` sites unchanged.
enum ReceiptSendError {
    Soft(&'static str),
    Fatal(CliError),
}

impl From<&'static str> for ReceiptSendError {
    fn from(code: &'static str) -> Self {
        ReceiptSendError::Soft(code)
    }
}

impl From<CliError> for ReceiptSendError {
    fn from(err: CliError) -> Self {
        ReceiptSendError::Fatal(err)
    }
}

fn send_delivered_receipt_ack(relay: &str, to: &str, msg_id: &str) -> Result<(), ReceiptSendError> {
    let payload = build_delivered_ack(msg_id)?;
    let pad_cfg = Some(MetaPadConfig {
        target_len: None,
        profile: Some(EnvelopeProfile::Standard),
        label: Some("small"),
    });
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
    let route_token = relay_peer_route_token(to)?;
    let pack = qsp_pack(to, &payload, pad_cfg, None, SendOrigination::Control)
        .map_err(|e| e.code)?;
    qsp_session_store_with_trigger(to, &pack.next_state, &pack.trigger)
        .map_err(|_| "qsp_session_store_failed")?;
    for pre in pack.pre_envelopes.iter() {
        transport::relay_inbox_push(relay, route_token.as_str(), pre)?;
    }
    transport::relay_inbox_push(relay, route_token.as_str(), &pack.envelope)?;
    Ok(())
}

fn send_file_completion_ack(
    relay: &str,
    to: &str,
    file_id: &str,
    confirm_id: &str,
) -> Result<(), ReceiptSendError> {
    let payload = build_file_completion_ack(file_id, confirm_id)?;
    let pad_cfg = Some(MetaPadConfig {
        target_len: None,
        profile: Some(EnvelopeProfile::Standard),
        label: Some("small"),
    });
    // ⚠ ENG-0095: the same barrier and the same reasoning as `send_delivered_receipt_ack`.
    // Route token first, pack, COMMIT fail-closed, only then push. Both receipt kinds move
    // together, because a barrier covering one of two sibling paths is not a barrier.
    let route_token = relay_peer_route_token(to)?;
    let pack = qsp_pack(to, &payload, pad_cfg, None, SendOrigination::Control)
        .map_err(|e| e.code)?;
    qsp_session_store_with_trigger(to, &pack.next_state, &pack.trigger)
        .map_err(|_| "qsp_session_store_failed")?;
    for pre in pack.pre_envelopes.iter() {
        transport::relay_inbox_push(relay, route_token.as_str(), pre)?;
    }
    transport::relay_inbox_push(relay, route_token.as_str(), &pack.envelope)?;
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

fn map_qsp_recv_reason(s: &str) -> &'static str {
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

fn map_qsp_recv_err(err: &RefimplError) -> &'static str {
    map_qsp_recv_reason(&err.to_string())
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

/// NA-0622 (ENG-0012 Stage 1b-ii): wall-clock seconds for the bounded DH-ratchet time fallback.
fn qsp_now_unix_secs() -> u64 {
    // NA-0688 C1 (R4a): delegates to the ONE clock. See `crate::clock`.
    //
    // ⚠ This is the sixth and last of the private clocks C1 consolidated, and the only one
    // that feeds a CRYPTO cadence rather than a policy deadline: it drives
    // `QSP_DH_FALLBACK_T_SECS` and `QSP_PQ_RESEED_T_SECS`. Pinning the clock therefore makes
    // the ratchet's time-fallback deterministic in a test, which is what C2's
    // deferred-rotation guards will need.
    crate::clock::now_unix_s()
}

/// NA-0622 (ENG-0012 Stage 1b-ii): decide whether this send performs a classical DH ratchet.
/// Ratchet-on-reply (a reply is pending) OR the bounded fallback fired (N messages / T seconds)
/// OR the send chain is unset — the responder's first send, which the ratchet CREATES now that
/// the static-`rk` bootstrap is gone.
fn qsp_should_ratchet(st: &Suite2SessionState, trig: &QspTriggerState, now: u64) -> bool {
    // A degenerate self-DH session (peer DH key == our own) is the UNSAFE seed-fallback test model
    // (symmetric, both role-A); it cannot round-trip the DIRECTION-sensitive DH ratchet (a sender
    // signs a boundary header under NHK_A->B while a role-A receiver would try NHK_B->A) and its
    // send chain is already seeded, so it retains the pre-ratchet behavior. We key off the SESSION
    // STATE (not the seed-permitted flag, which real-handshake tests also set): real handshake
    // sessions have dhr != dhs. The ratchet is proven end-to-end over a real A/B handshake in
    // tests/handshake_mvp.rs::dh_ratchet_e2e_*.
    if st.dh.dhr == st.dh.dhs_pub {
        return false;
    }
    if zero32(&st.send.ck_ec) || zero32(&st.send.ck_pq) {
        return true;
    }
    trig.pending_send_ratchet
        || trig.msgs_since_ratchet >= QSP_DH_FALLBACK_N
        || (trig.last_ratchet_unix_secs != 0
            && now.saturating_sub(trig.last_ratchet_unix_secs) >= QSP_DH_FALLBACK_T_SECS)
}

// NA-0624 (ENG-0012 Stage 2b): SCKA cadence policy (Operator Decision 3). The SCKA path is
// gated OFF for the degenerate self-DH seed session (`dhr == dhs`, the UNSAFE seed-fallback
// test model), exactly like the DH ratchet — real handshake sessions have dhr != dhs. With no
// advertisements the SCKA path is inert and the persisted SCKA section stays empty, keeping the
// seed-model runtime-equivalence byte-for-byte.
fn qsp_scka_enabled(st: &Suite2SessionState) -> bool {
    st.dh.dhr != st.dh.dhs_pub
}

/// Advertise on establishment (no live advertised key yet — also re-arms after the peer consumes
/// our key) and on rotation (a live key advertised more than the rotation period ago is refreshed
/// so a lost advertisement or lost reseed self-heals).
fn qsp_scka_advertise_due(scka: &SckaLocalState, now: u64) -> bool {
    match scka.live_advkey() {
        None => true,
        Some(_) => {
            scka.last_adv_unix_secs != 0
                && now.saturating_sub(scka.last_adv_unix_secs) >= QSP_PQ_RESEED_T_SECS
        }
    }
}

/// Reseed when a fresh (unconsumed) peer advertisement is available AND sparsely: immediately
/// for the first reseed, then every `QSP_PQ_RESEED_N` sent DH boundaries or
/// `QSP_PQ_RESEED_T_SECS` seconds. Evaluated only on a non-DH-boundary send, so a reseed is
/// co-scheduled after DH boundaries rather than replacing one.
fn qsp_scka_reseed_due(scka: &SckaLocalState, now: u64) -> bool {
    if scka.peer_adv.is_none() {
        return false;
    }
    scka.last_reseed_unix_secs == 0
        || scka.boundaries_since_reseed >= QSP_PQ_RESEED_N
        || now.saturating_sub(scka.last_reseed_unix_secs) >= QSP_PQ_RESEED_T_SECS
}

/// Wrap a Suite-2 wire message in a standard-profile QSE envelope (the SCKA control-envelope
/// path; mirrors the main-message envelope padding in `qsp_pack`).
fn qsp_wrap_standard_envelope(
    c: &StdCrypto,
    wire: Vec<u8>,
    meta_seed: Option<u64>,
) -> Result<Vec<u8>, QspPackError> {
    let mut env = Envelope {
        env_version: QSE_ENV_VERSION_V1,
        flags: 0,
        route_token: Vec::new(),
        timestamp_bucket: 0,
        payload: wire,
        padding: Vec::new(),
    };
    let encoded_len = env.encode().len();
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
    }
    Ok(env.encode())
}

fn qsp_pack(
    channel: &str,
    plaintext: &[u8],
    pad_cfg: Option<MetaPadConfig>,
    meta_seed: Option<u64>,
    origination: SendOrigination,
) -> Result<QspPackOutcome, QspPackError> {
    let st =
        qsp_session_for_channel(channel).map_err(|code| QspPackError { code, reason: None })?;
    let mut trig = qsp_trigger_load(channel);
    let c = StdCrypto;
    // NA-0622 (ENG-0012 Stage 1b-ii): originate a classical DH boundary when the trigger fires
    // (ratchet-on-reply + N=4/T=15min fallback + the responder's first send), else a normal
    // message on the current sending chain. The DH ratchet reuses the refimpl `send_boundary`.
    let now = qsp_now_unix_secs();
    // NA-0624 (ENG-0012 Stage 2b): SCKA advertisement — a CONTROL message pushed BEFORE the main
    // message. It rides the current send chain (one chain step; the peer's OOO machinery skips
    // it), so it needs live send chain keys; the advertised ML-KEM secret key is persisted
    // fail-closed BEFORE the advertisement envelope can exist.
    let scka_on = qsp_scka_enabled(&st);
    let mut scka = if scka_on {
        qsp_scka_load(channel)
    } else {
        SckaLocalState::default()
    };
    let mut pre_envelopes: Vec<Vec<u8>> = Vec::new();
    let mut scka_dirty = false;
    let mut st_cur = st.clone();
    // NA-0625 (ENG-0023, Operator Decision 2): the authenticated ADV receiver consumes its
    // chain slot in-order, so an advertisement may share a pack with a reseed — the NA-0624
    // ADV/reseed pack-exclusion rule (and the mkskipped control-slot growth it worked around)
    // is RETIRED. The receiver processes the pack in order: ADV first (nr passes through the
    // control slot), then the reseed's strict n == nr check holds.
    if scka_on
        && !zero32(&st_cur.send.ck_ec)
        && !zero32(&st_cur.send.ck_pq)
        && origination.may_originate()
        && qsp_scka_advertise_due(&scka, now)
    {
        let max_known = st_cur
            .recv
            .known_targets
            .iter()
            .next_back()
            .copied()
            .unwrap_or(0);
        let adv_id = scka
            .local_next_adv_id
            .max(max_known.saturating_add(1))
            .max(1);
        let (pk, sk) = runtime_pq_kem_keypair();
        match send_pq_advertise(&c, &c, &c, st_cur.clone(), adv_id, &pk, &[]) {
            Ok(out) => {
                scka.insert_advkey(adv_id, sk);
                scka.local_next_adv_id = adv_id.saturating_add(1);
                scka.last_adv_unix_secs = now;
                scka_dirty = true;
                pre_envelopes.push(qsp_wrap_standard_envelope(&c, out.wire, meta_seed)?);
                st_cur = out.state;
                if origination.counts_toward_rotation() {
                    trig.msgs_since_ratchet = trig.msgs_since_ratchet.saturating_add(1);
                }
                let id_s = adv_id.to_string();
                emit_marker(
                    "qsp_scka_adv",
                    None,
                    &[("dir", "send"), ("adv_id", id_s.as_str())],
                );
            }
            Err(e) => {
                // Fail-safe skip: no advertisement means no reseed (the classical status quo);
                // the user message itself is unaffected.
                emit_marker("qsp_scka_adv", Some(e), &[("dir", "send"), ("ok", "false")]);
            }
        }
    }
    // ⚠ NA-0688 C2 (R1a as amended, ruling A6) — THE ESTABLISHMENT EXCEPTION.
    //
    // A control send never originates a ROTATION. But `qsp_should_ratchet` returns true
    // unconditionally when the sending chain is unseeded, and that is not the cadence — it is
    // chain ESTABLISHMENT, the thing `send_boundary` exists to do. A send that skips it has no
    // chain to send on at all, and ENG-0086 finding 1 makes this the COMMON case for a
    // receipt: "the recipient's automatic ack becomes their first send."
    //
    // Establishment is a NECESSITY, permitted to every send; rotation is an OPPORTUNITY,
    // reserved to user sends. The two are distinguishable in the emitted marker —
    // establishment reports `reason=first_send`, rotation reports `reply` or `fallback` — and
    // the guards assert on exactly that distinction.
    //
    // ⚠ When rotation is DUE but forbidden, falling through leaves `trig` UNTOUCHED. That is
    // the deferred-rotation semantics: the due-state survives byte-identical and the next
    // USER send honours it. An ack that cleared the due-state without rotating would be worse
    // than an ack that rotated, and it is guarded in both directions.
    let chain_unseeded = zero32(&st_cur.send.ck_ec) || zero32(&st_cur.send.ck_pq);
    // ⚠ NA-0688 — A6 IS REVERSED. A CONTROL SEND ORIGINATES NOTHING, INCLUDING ESTABLISHMENT.
    //
    // A6 originally carved out chain ESTABLISHMENT as a necessity every send could perform,
    // including an ack. That exception was measured to break sessions: `send_boundary` MINTS A
    // FRESH DH KEYPAIR AND ADVANCES THE SHARED ROOT (it is the only way the refimpl can seed a
    // send chain), so an ack moved the recipient's key — and a sender who had not pulled that
    // ack then computed a boundary against a stale key. Measured result: a PERMANENT,
    // BIDIRECTIONAL wedge — the sender could not decrypt, the recipient could not decrypt the
    // sender's acks either, and subsequent messages failed too.
    //
    // So R1a is restored to its literal meaning: an ack requires an ALREADY-ESTABLISHED sending
    // chain. When there is none the receipt is not dropped — it is written to the durable
    // owed-receipt hold and flushed on the peer's first real send, which establishes normally.
    // That distinction between REFUSING and DEFERRING is the whole design; refusing alone was
    // measured to lose the first receipt of every conversation.
    let boundary_permitted = origination.may_originate();
    if chain_unseeded && !boundary_permitted {
        return Err(QspPackError {
            code: "qsp_chain_unseeded",
            reason: Some("CONTROL_SEND_CANNOT_ESTABLISH"),
        });
    }
    let (wire, next_state, msg_n) = if qsp_should_ratchet(&st_cur, &trig, now) && boundary_permitted
    {
        let out =
            send_boundary(&c, &c, &c, &c, st_cur.clone(), plaintext).map_err(|e| QspPackError {
                code: "qsp_pack_failed",
                reason: Some(e),
            })?;
        // ⚠ NA-0688 C2: ESTABLISHMENT IS REPORTED FIRST, and the order matters.
        //
        // `pending_send_ratchet` used to win this label, so a boundary that ESTABLISHED an
        // unseeded chain was reported as `reason=reply` whenever a reply also happened to be
        // pending — which is the normal state of a recipient about to ack. The marker named
        // the opportunity rather than the cause, and under passivation that is exactly the
        // distinction the guards turn on: a control send may ESTABLISH but may never ROTATE.
        // Measured: before this fix a first ack reported `reason=reply` while doing
        // establishment, which would have made the ruled first-send guard unsatisfiable and
        // left the marker saying something untrue.
        let reason = if chain_unseeded {
            "first_send"
        } else if trig.pending_send_ratchet {
            "reply"
        } else {
            "fallback"
        };
        emit_marker(
            "qsp_dh_ratchet",
            None,
            &[("dir", "send"), ("reason", reason)],
        );
        // ⚠ NA-0688 C3 — THE ESTABLISHMENT MUST NOT EAT THE HUMAN'S OWED REPLY.
        //
        // This reset used to be unconditional, and C2's establishment exception therefore had
        // a hole: a CONTROL send that established an unseeded chain cleared
        // `pending_send_ratchet`, silently consuming the rotation the human's reply was owed —
        // the exact outcome RULING A's deferred-rotation semantics exist to prevent, arriving
        // through the one branch a control send is still allowed to take.
        //
        // Found by C3's flip: with receipts on by default, `a_user_reply_still_rotates_the_
        // ratchet` went red, because bob's ack established his chain and his real reply then
        // had nothing left to rotate on. C2's own guard caught C2's own defect, one commit later.
        //
        // ⚠ THE PREDICATE IS `may_originate()`, NOT THE BOUNDARY PERMISSION. The first attempt
        // at this fix keyed on `boundary_permitted` (then misnamed `rotation_permitted`), which
        // is TRUE for an establishing control send — so it read as correct and changed nothing
        // on the only path it was written for. Instrumenting the trigger state at each pack
        // measured `orig=control ... rot_perm=1` and `pending 1 -> 0` across the establishing
        // ack; the fix is keyed on the question actually being asked. Only a send permitted to
        // ROTATE consumes the due-state.
        //
        // The WHOLE reset is skipped, not just the flag: `msgs_since_ratchet` and
        // `last_ratchet_unix_secs` are the N and T fallbacks' due-state by the same argument,
        // and an establishing boundary must not consume any of it. A USER send is unaffected —
        // `may_originate()` is true for `User`, so it resets exactly as it always has.
        if origination.may_originate() {
            trig = QspTriggerState {
                pending_send_ratchet: false,
                msgs_since_ratchet: 0,
                last_ratchet_unix_secs: now,
            };
        }
        if scka_on && !scka.is_default() {
            scka.boundaries_since_reseed = scka.boundaries_since_reseed.saturating_add(1);
            scka_dirty = true;
        }
        (out.wire, out.state, 0u32)
    } else if scka_on && origination.may_originate() && qsp_scka_reseed_due(&scka, now) {
        // NA-0624: PQ reseed (DOC-CAN-003 §8.5.3) — encapsulate to the peer's advertised key and
        // originate the FLAG_PQ_CTXT boundary via the FROZEN Stage-2a sender. The consumed peer
        // advertisement is persisted fail-closed BEFORE the reseed wire exists (re-targeting a
        // consumed advertisement after a crash would desynchronise the root).
        let peer_adv = scka.peer_adv.clone().expect("reseed_due implies peer_adv");
        match c.encap(&peer_adv.pubkey) {
            Ok((ct, ss)) => {
                let target_s = peer_adv.adv_id.to_string();
                let out = send_pq_reseed(
                    &c,
                    &c,
                    &c,
                    st_cur.clone(),
                    peer_adv.adv_id,
                    &ct,
                    &ss,
                    plaintext,
                )
                .map_err(|e| QspPackError {
                    code: "qsp_pack_failed",
                    reason: Some(e),
                })?;
                scka.peer_adv = None;
                scka.peer_adv_consumed_max = scka.peer_adv_consumed_max.max(peer_adv.adv_id);
                scka.boundaries_since_reseed = 0;
                scka.last_reseed_unix_secs = now;
                scka_dirty = true;
                emit_marker(
                    "qsp_pq_reseed",
                    None,
                    &[("dir", "send"), ("target_id", target_s.as_str())],
                );
                let n = st_cur.send.ns;
                if origination.counts_toward_rotation() {
                    trig.msgs_since_ratchet = trig.msgs_since_ratchet.saturating_add(1);
                }
                (out.wire, out.state, n)
            }
            Err(_) => {
                // Fail-safe skip: an un-encapsulatable advertisement is dropped (never
                // re-targeted) and the message goes out on the current chain.
                scka.peer_adv = None;
                scka.peer_adv_consumed_max = scka.peer_adv_consumed_max.max(peer_adv.adv_id);
                scka_dirty = true;
                emit_marker(
                    "qsp_pq_reseed",
                    Some("scka_encap_failed"),
                    &[("dir", "send"), ("ok", "false")],
                );
                let out = send_wire_canon(&c, &c, &c, st_cur.send.clone(), 0, plaintext).map_err(
                    |e| QspPackError {
                        code: "qsp_pack_failed",
                        reason: Some(map_qsp_pack_reason(&e)),
                    },
                )?;
                let mut ns = st_cur.clone();
                ns.send = out.state;
                if origination.counts_toward_rotation() {
                    trig.msgs_since_ratchet = trig.msgs_since_ratchet.saturating_add(1);
                }
                (out.wire, ns, out.n)
            }
        }
    } else {
        let out = send_wire_canon(&c, &c, &c, st_cur.send.clone(), 0, plaintext).map_err(|e| {
            QspPackError {
                code: "qsp_pack_failed",
                reason: Some(map_qsp_pack_reason(&e)),
            }
        })?;
        let mut ns = st_cur.clone();
        ns.send = out.state;
        // ⚠ RULING A. Without this gate four received messages produce four acks,
        // `msgs_since_ratchet` reaches QSP_DH_FALLBACK_N, and the ratchet rotates on machine
        // traffic in a conversation where the human replied to nothing. Suppressing
        // origination alone does NOT close that: the counter is a second, quieter channel.
        if origination.counts_toward_rotation() {
            trig.msgs_since_ratchet = trig.msgs_since_ratchet.saturating_add(1);
        }
        (out.wire, ns, out.n)
    };
    let mut env = Envelope {
        env_version: QSE_ENV_VERSION_V1,
        flags: 0,
        route_token: Vec::new(),
        timestamp_bucket: 0,
        payload: wire,
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
    // NA-0624: persist the SCKA store exactly once, at the success boundary — an advertised
    // ML-KEM secret key and a consumed peer advertisement MUST be durable before any wire that
    // depends on them leaves this function, and nothing may persist if the pack fails (no
    // orphaned live advertised key can suppress future advertisements).
    if scka_dirty {
        qsp_scka_store(channel, &scka).map_err(|_| QspPackError {
            code: "qsp_pack_failed",
            reason: Some("scka_store_failed"),
        })?;
    }
    Ok(QspPackOutcome {
        envelope: env.encode(),
        pre_envelopes,
        next_state,
        trigger: trig,
        msg_idx: msg_n,
        ck_idx: msg_n,
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
    let mut trig = qsp_trigger_load(channel);
    let c = StdCrypto;
    // NA-0622 (ENG-0012 Stage 1b-ii) + NA-0624 (Stage 2b) + NA-0625 (ENG-0023) routing: an SCKA
    // advertisement (FLAG_PQ_ADV) drives the AUTHENTICATED `recv_pq_adv` receiver via
    // `recv_wire` (a control message, cryptographically bound to the session before tracking);
    // a PQ-reseed boundary (FLAG_PQ_CTXT) decapsulates against the local advertised key and
    // drives `apply_pq_reseed` via `recv_wire`; a classical DH boundary goes to
    // `recv_dh_boundary`; else a normal message.
    let flags = match decode_suite2_wire_canon(&env.payload) {
        Ok((_, _, _, parsed)) => parsed.flags,
        Err(_) => 0,
    };
    let is_pq_adv = (flags & FLAG_PQ_ADV) != 0;
    let is_pq_ctxt = !is_pq_adv && (flags & FLAG_BOUNDARY) != 0 && (flags & FLAG_PQ_CTXT) != 0;
    let is_dh_boundary = !is_pq_adv && (flags & FLAG_BOUNDARY) != 0 && (flags & FLAG_PQ_CTXT) == 0;
    if is_pq_adv {
        // NA-0625 (ENG-0023): AUTHENTICATED SCKA TRACK (DOC-CAN-004 §3.2) — the ADV drives the
        // refimpl SESSION-LEVEL `recv_pq_adv_session` (NA-0626 ENG-0024: the root injection is
        // internal; the INJECT/ADOPT dances are gone with the duplicated root slots). A
        // planted/unauthenticated advertisement is REJECTED, never tracked: it fails the header
        // AEAD under session keys and/or the ADVAUTH MAC under the canonical root. The ADV
        // consumes its chain slot in-order (Operator Decision 2), so it leaves no receive-chain
        // gap (mkskipped stays empty) and may share a pack with a reseed (the NA-0624 exclusion
        // rule is retired).
        if !qsp_scka_enabled(&st) {
            return Err("qsp_recv_failed");
        }
        let parsed = match decode_suite2_wire_canon(&env.payload) {
            Ok((_, _, _, p)) => p,
            Err(_) => return Err("qsp_recv_failed"),
        };
        let adv_id = parsed.pq_adv_id.ok_or("qsp_recv_failed")?;
        let adv_pub = parsed.pq_adv_pub.ok_or("qsp_recv_failed")?;
        let mut scka = qsp_scka_load(channel);
        // The `peer_adv_watermark` is CALLER-OWNED (the SCKA store's peer_adv_max_seen); the
        // session-state watermark field belongs to the frozen CTXT receiver's consumed-target
        // monotonicity and is not touched.
        let outcome =
            recv_pq_adv_session(&c, &c, &c, st.clone(), &env.payload, scka.peer_adv_max_seen);
        if !outcome.ok {
            let code = map_qsp_recv_reason(outcome.reason.unwrap_or("qsp_recv_failed"));
            emit_marker(
                "qsp_scka_adv",
                Some(code),
                &[("dir", "recv"), ("ok", "false")],
            );
            return Err("qsp_scka_adv_reject");
        }
        let msg_n = outcome.n.unwrap_or(0);
        let next_state = outcome.state;
        // Any received message arms the reply-driven trigger.
        trig.pending_send_ratchet = true;
        // G2 ordering pin: persist the SESSION FIRST (the consumed chain slot must be durable
        // before anything depends on it — a replayed ADV can never re-derive the slot key),
        // the SCKA store SECOND. A crash between the two loses only an UNTRACKED peer
        // advertisement — bounded by the peer's T_pq re-advertise — and can never break the
        // chain, accept a replay, or roll back consumed-monotonicity. (Contrast the CTXT arm
        // below, which keeps its erase-consumed-key-BEFORE-plaintext order for the
        // one-time-key hazard.)
        qsp_session_store_with_trigger(channel, &next_state, &trig)
            .map_err(|_| "qsp_session_store_failed")?;
        scka.peer_adv = Some(SckaPeerAdv {
            adv_id,
            pubkey: adv_pub,
        });
        scka.peer_adv_max_seen = adv_id;
        qsp_scka_store(channel, &scka).map_err(|_| "qsp_session_store_failed")?;
        let id_s = adv_id.to_string();
        emit_marker(
            "qsp_scka_adv",
            None,
            &[("dir", "recv"), ("adv_id", id_s.as_str()), ("auth", "ok")],
        );
        return Ok(QspUnpackOutcome {
            plaintext: Vec::new(),
            next_state,
            trigger: trig,
            msg_idx: msg_n,
            skip_delta: 0,
            evicted: 0,
            is_control: true,
        });
    }
    let (plaintext, next_state, msg_n, skip_delta, evicted) = if is_pq_ctxt {
        // SCKA RESEED RECEIVE (DOC-CAN-003 §8.5.3 receiver side): look up the targeted local
        // advertised key, decapsulate, and drive the SESSION-LEVEL `recv_pq_reseed` (NA-0626
        // ENG-0030 structural): the entry point returns a FULLY updated session state — root,
        // receive schedule, AND the receiver's send half — so the NA-0624 INJECT/ADOPT root
        // dances and the NA-0625 caller-side send-half refresh no longer exist (the duplicated
        // fields are gone; the compiler enforces it). The same entry point accepts the combined
        // DH+PQ boundary (ENG-0026: a fresh DH_pub on the 0x0006 frame). The consumed local key
        // is erased fail-closed before the plaintext is released.
        if !qsp_scka_enabled(&st) {
            return Err("qsp_recv_failed");
        }
        let parsed = match decode_suite2_wire_canon(&env.payload) {
            Ok((_, _, _, p)) => p,
            Err(_) => return Err("qsp_recv_failed"),
        };
        let target_id = parsed.pq_target_id.ok_or("qsp_recv_failed")?;
        let ct = parsed.pq_ct.ok_or("qsp_recv_failed")?;
        let mut scka = qsp_scka_load(channel);
        let secret = match scka
            .advkeys
            .iter()
            .find(|k| k.adv_id == target_id && !k.consumed && !k.secret.is_empty())
        {
            Some(k) => k.secret.clone(),
            None => return Err("qsp_scka_target_unknown"),
        };
        let ss = c.decap(&secret, &ct).map_err(|_| "qsp_scka_decap_failed")?;
        let outcome = recv_pq_reseed(&c, &c, &c, &c, st.clone(), &env.payload, &ss, target_id);
        if !outcome.ok {
            return Err(map_qsp_recv_reason(
                outcome.reason.unwrap_or("qsp_recv_failed"),
            ));
        }
        let msg_n = outcome.n.unwrap_or(0);
        let combined = parsed.dh_pub != st.dh.dhr;
        let prev_len = st.recv.mkskipped.len();
        let mut next_state = outcome.state;
        let skip_delta = next_state.recv.mkskipped.len().saturating_sub(prev_len);
        let evicted = bound_mkskipped(&mut next_state.recv);
        scka.consume_advkey(target_id);
        qsp_scka_store(channel, &scka).map_err(|_| "qsp_session_store_failed")?;
        let target_s = target_id.to_string();
        emit_marker(
            "qsp_pq_reseed",
            None,
            &[
                ("dir", "recv"),
                ("target_id", target_s.as_str()),
                ("combined", if combined { "true" } else { "false" }),
            ],
        );
        (outcome.plaintext, next_state, msg_n, skip_delta, evicted)
    } else if is_dh_boundary {
        let out = recv_dh_boundary(&c, &c, &c, &c, st.clone(), &env.payload);
        if !out.ok {
            return Err(out.reason.unwrap_or("qsp_recv_failed"));
        }
        emit_marker("qsp_dh_ratchet", None, &[("dir", "recv")]);
        (out.plaintext, out.state, 0u32, 0usize, 0usize)
    } else {
        let outcome = recv_wire_canon(
            &c,
            &c,
            &c,
            st.recv.clone(),
            &st.rk,
            &env.payload,
            None,
            None,
        )
        .map_err(|e| map_qsp_recv_err(&e))?;
        let mut next_state = st.clone();
        let prev_len = next_state.recv.mkskipped.len();
        next_state.recv = outcome.state;
        next_state.rk = outcome.rk;
        let skip_delta = next_state.recv.mkskipped.len().saturating_sub(prev_len);
        let evicted = bound_mkskipped(&mut next_state.recv);
        (
            outcome.plaintext,
            next_state,
            outcome.n,
            skip_delta,
            evicted,
        )
    };
    // Any received message arms the reply-driven trigger: the next send performs a DH ratchet.
    trig.pending_send_ratchet = true;
    Ok(QspUnpackOutcome {
        plaintext,
        next_state,
        trigger: trig,
        msg_idx: msg_n,
        skip_delta,
        evicted,
        is_control: false,
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
    pub ack_mode: Option<AckMode>,
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
    legacy_receive_mode: LegacyReceiveMode,
    ack_mode: AckMode,
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
    return Err(CliError::code("recv_reject_parse"));
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
    pad_cfg: Option<MetaPadConfig>,
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
            Err(code) => return Err(CliError::code(code)),
        },
        (None, Some(file), Some(confirm)) => {
            let file_id = if file == "latest" {
                latest_outbound_file_id(peer).map_err(|code| CliError::code(code))?
            } else {
                file.to_string()
            };
            let confirm_id = if confirm == "auto" {
                file_transfer_confirm_id(peer, file_id.as_str())
                    .map_err(|code| CliError::code(code))?
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
                Err(code) => return Err(CliError::code(code)),
            }
        }
        _ => return Err(CliError::code("receipt_apply_invalid_args")),
    }
}

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

    pub fn push(&mut self, item: T) -> Result<(), ()> {
        if self.items.len() >= self.max {
            return Err(());
        }
        self.items.push_back(item);
        Ok(())
    }
}

pub fn bounded_retry<F>(mut attempts: u32, mut op: F) -> Result<u32, ()>
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
