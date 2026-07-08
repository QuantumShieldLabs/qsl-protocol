#![allow(unexpected_cfgs)]

use chacha20poly1305::aead::{Aead, KeyInit, Payload};
use chacha20poly1305::{ChaCha20Poly1305, Key, Nonce};
use quantumshield_refimpl::crypto::stdcrypto::StdCrypto;
use quantumshield_refimpl::crypto::traits::{Hash, Kmac};
use quantumshield_refimpl::suite2::ratchet::{
    Suite2DhRatchetState, Suite2RecvWireState, Suite2SendState,
};
use quantumshield_refimpl::suite2::state::Suite2SessionState;
use quantumshield_refimpl::suite2::types::{SUITE2_PROTOCOL_VERSION, SUITE2_SUITE_ID};
use rand_core::{OsRng, RngCore};
use std::collections::BTreeSet;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process;

use crate::fs_store::{check_parent_safe, config_dir, enforce_safe_parents, write_atomic};
use crate::model::{ConfigSource, ErrorCode};
use crate::output::emit_marker;
use crate::store::{
    QspStatusRecord, QSP_SESSIONS_DIR, QSP_SESSION_BLOB_MAGIC, QSP_SESSION_BLOB_VERSION,
    QSP_SESSION_LEGACY_TOMBSTONE, QSP_SESSION_STORE_KEY_SECRET,
};
use crate::vault;
use crate::{channel_label_ok, env_bool, hex_decode, hex_encode};

pub(crate) const QSP_STATUS_FILE_NAME: &str = "qsp_status.json";

#[cfg(qsc_rng_failure_test_seam)]
fn qsp_rng_failure_forced(label: &str) -> bool {
    env::var("QSC_RNG_FAILURE_TEST_SEAM")
        .ok()
        .map(|v| v == label || v == "all")
        .unwrap_or(false)
}

#[cfg(qsc_rng_failure_test_seam)]
fn qsp_rng_fill(label: &str, out: &mut [u8]) -> Result<(), ErrorCode> {
    if qsp_rng_failure_forced(label) {
        return Err(ErrorCode::IdentitySecretUnavailable);
    }
    OsRng.fill_bytes(out);
    Ok(())
}

pub(crate) fn qsp_status_parts(value: &str) -> (&str, &str) {
    value.split_once(" reason=").unwrap_or((value, "unknown"))
}

pub(crate) fn qsp_status_user_note(reason: &str) -> &'static str {
    match reason {
        "handshake" => "Stored session ready for exchange.",
        "no_session" => "No stored session yet; run handshake init/poll before exchange.",
        "missing_seed" => "No stored session yet; deterministic seed fallback is absent.",
        "session_invalid" => "Stored session rejected as invalid or stale; re-establish handshake.",
        "vault_secret_missing" => "Unlock before restoring session state from the local vault.",
        "chainkey_unset" => "Handshake exists, but send keys are not ready yet.",
        _ => "Session state unavailable.",
    }
}

fn qsp_status_path(dir: &Path) -> PathBuf {
    dir.join(QSP_STATUS_FILE_NAME)
}

fn write_qsp_status(dir: &Path, source: ConfigSource, status: &QspStatusRecord) {
    let bytes = match serde_json::to_vec(status) {
        Ok(v) => v,
        Err(_) => return,
    };
    let _ = write_atomic(&qsp_status_path(dir), &bytes, source);
}

pub(crate) fn record_qsp_status(
    dir: &Path,
    source: ConfigSource,
    active: bool,
    reason: &str,
    pack_ok: bool,
    unpack_ok: bool,
) {
    let status = QspStatusRecord {
        active,
        reason: reason.to_string(),
        last_pack_ok: pack_ok,
        last_unpack_ok: unpack_ok,
    };
    write_qsp_status(dir, source, &status);
}

pub(crate) fn qsp_status_tuple(peer: &str) -> (String, String) {
    let (dir, source) = match config_dir() {
        Ok(v) => v,
        Err(_) => return ("INACTIVE".to_string(), "missing_home".to_string()),
    };
    if !check_parent_safe(&dir, source) {
        return ("INACTIVE".to_string(), "unsafe_parent".to_string());
    }
    if !channel_label_ok(peer) {
        return ("INACTIVE".to_string(), "channel_invalid".to_string());
    }
    match qsp_session_load(peer) {
        Ok(Some(_)) => ("ACTIVE".to_string(), "handshake".to_string()),
        Ok(None) => {
            if env::var("QSC_QSP_SEED").is_ok() {
                ("INACTIVE".to_string(), "no_session".to_string())
            } else {
                ("INACTIVE".to_string(), "missing_seed".to_string())
            }
        }
        Err(ErrorCode::ParseFailed) => ("INACTIVE".to_string(), "session_invalid".to_string()),
        Err(_) => ("INACTIVE".to_string(), "session_invalid".to_string()),
    }
}

pub(crate) fn zero32(v: &[u8; 32]) -> bool {
    v.iter().all(|b| *b == 0)
}

pub(crate) fn qsp_send_ready_tuple(peer: &str) -> (bool, &'static str) {
    if !channel_label_ok(peer) {
        return (false, "other");
    }
    match qsp_session_load(peer) {
        Ok(Some(st)) => {
            if zero32(&st.send.ck_ec) || zero32(&st.send.ck_pq) {
                (false, "chainkey_unset")
            } else {
                (true, "ready")
            }
        }
        Ok(None) => (false, "no_session"),
        Err(ErrorCode::IdentitySecretUnavailable) => (false, "vault_secret_missing"),
        Err(ErrorCode::ParseFailed) => (false, "state_corrupt"),
        Err(_) => (false, "other"),
    }
}

pub(crate) fn qsp_status_string(peer: &str) -> String {
    let (status, reason) = qsp_status_tuple(peer);
    format!("{} reason={}", status, reason)
}

fn qsp_sessions_dir(dir: &Path) -> PathBuf {
    dir.join(QSP_SESSIONS_DIR)
}

fn qsp_session_path(dir: &Path, peer: &str) -> PathBuf {
    qsp_sessions_dir(dir).join(format!("{}.bin", peer))
}

fn qsp_session_blob_path(dir: &Path, peer: &str) -> PathBuf {
    qsp_sessions_dir(dir).join(format!("{}.qsv", peer))
}

fn qsp_session_aad(peer: &str) -> Vec<u8> {
    format!("QSC.QSP.SESSION.V{}:{}", QSP_SESSION_BLOB_VERSION, peer).into_bytes()
}

fn qsp_session_test_fallback_key(peer: &str) -> Result<[u8; 32], ErrorCode> {
    let seed = qsp_seed_from_env().map_err(|_| ErrorCode::IdentitySecretUnavailable)?;
    let c = StdCrypto;
    let seed_bytes = seed.to_le_bytes();
    let seed_hash = c.sha512(&seed_bytes);
    let mut seed_key = [0u8; 32];
    seed_key.copy_from_slice(&seed_hash[..32]);
    Ok(kmac_out::<32>(
        &c,
        &seed_key,
        "QSC.QSP.SESSION.STORE.TESTKEY",
        peer.as_bytes(),
    ))
}

fn qsp_session_decode_key(secret: &str) -> Result<[u8; 32], ErrorCode> {
    let raw = hex_decode(secret)?;
    if raw.len() != 32 {
        return Err(ErrorCode::ParseFailed);
    }
    let mut key = [0u8; 32];
    key.copy_from_slice(&raw);
    Ok(key)
}

fn qsp_session_store_key_load(peer: &str) -> Result<[u8; 32], ErrorCode> {
    match vault::secret_get(QSP_SESSION_STORE_KEY_SECRET) {
        Ok(Some(v)) => qsp_session_decode_key(&v),
        Ok(None) => Err(ErrorCode::IdentitySecretUnavailable),
        Err("vault_missing" | "vault_locked") => {
            if allow_unsafe_seed_fallback_for_tests() {
                qsp_session_test_fallback_key(peer)
            } else {
                Err(ErrorCode::IdentitySecretUnavailable)
            }
        }
        Err(_) => Err(ErrorCode::IdentitySecretUnavailable),
    }
}

fn qsp_session_store_key_get_or_create(peer: &str) -> Result<[u8; 32], ErrorCode> {
    match vault::secret_get(QSP_SESSION_STORE_KEY_SECRET) {
        Ok(Some(v)) => qsp_session_decode_key(&v),
        Ok(None) => {
            let mut key = [0u8; 32];
            #[cfg(qsc_rng_failure_test_seam)]
            qsp_rng_fill("QSC.QSP.SESSION_STORE_KEY", &mut key)?;
            #[cfg(not(qsc_rng_failure_test_seam))]
            OsRng.fill_bytes(&mut key);
            let secret = hex_encode(&key);
            match vault::secret_set(QSP_SESSION_STORE_KEY_SECRET, &secret) {
                Ok(()) => Ok(key),
                Err("vault_missing" | "vault_locked") => {
                    if allow_unsafe_seed_fallback_for_tests() {
                        qsp_session_test_fallback_key(peer)
                    } else {
                        Err(ErrorCode::IdentitySecretUnavailable)
                    }
                }
                Err(_) => Err(ErrorCode::IdentitySecretUnavailable),
            }
        }
        Err("vault_missing" | "vault_locked") => {
            if allow_unsafe_seed_fallback_for_tests() {
                qsp_session_test_fallback_key(peer)
            } else {
                Err(ErrorCode::IdentitySecretUnavailable)
            }
        }
        Err(_) => Err(ErrorCode::IdentitySecretUnavailable),
    }
}

// NA-0622 (ENG-0012 Stage 1b-ii): reply-driven DH-ratchet trigger state, persisted INSIDE the
// qsc session blob's encrypted plaintext (v2 = b"QTRG" + trigger(13) + QS2S snapshot). A legacy
// raw-QS2S plaintext (starting with b"QS2S") migrates transparently with a default trigger. This
// keeps the refimpl Suite2SessionState / QS2S snapshot format FROZEN — client policy stays out of
// the crypto core. qsc is load-per-message, so this state must be persisted, not in-memory.
pub(crate) const QSP_TRIGGER_MAGIC: &[u8; 4] = b"QTRG";
pub(crate) const QSP_TRIGGER_LEN: usize = 13;
/// Bounded fallback: force a DH ratchet after this many messages without a reply.
pub(crate) const QSP_DH_FALLBACK_N: u32 = 4;
/// Bounded fallback: force a DH ratchet after this many seconds without a reply.
pub(crate) const QSP_DH_FALLBACK_T_SECS: u64 = 900;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) struct QspTriggerState {
    /// Set on receive; the next send performs a DH boundary (ratchet-on-reply).
    pub pending_send_ratchet: bool,
    /// Messages sent since the last DH ratchet (bounded fallback, N).
    pub msgs_since_ratchet: u32,
    /// Unix seconds of the last DH ratchet (bounded fallback, T).
    pub last_ratchet_unix_secs: u64,
}

impl QspTriggerState {
    fn encode(&self) -> [u8; QSP_TRIGGER_LEN] {
        let mut out = [0u8; QSP_TRIGGER_LEN];
        out[0] = self.pending_send_ratchet as u8;
        out[1..5].copy_from_slice(&self.msgs_since_ratchet.to_le_bytes());
        out[5..13].copy_from_slice(&self.last_ratchet_unix_secs.to_le_bytes());
        out
    }
    fn decode(b: &[u8; QSP_TRIGGER_LEN]) -> Self {
        let mut m = [0u8; 4];
        m.copy_from_slice(&b[1..5]);
        let mut t = [0u8; 8];
        t.copy_from_slice(&b[5..13]);
        QspTriggerState {
            pending_send_ratchet: b[0] != 0,
            msgs_since_ratchet: u32::from_le_bytes(m),
            last_ratchet_unix_secs: u64::from_le_bytes(t),
        }
    }
}

/// Split a decrypted session-blob plaintext into (trigger, raw QS2S snapshot). A v2 plaintext is
/// prefixed with `QSP_TRIGGER_MAGIC`; a legacy plaintext is the raw snapshot (default trigger).
fn qsp_split_plaintext(pt: &[u8]) -> (QspTriggerState, &[u8]) {
    let hdr = QSP_TRIGGER_MAGIC.len() + QSP_TRIGGER_LEN;
    if pt.len() >= hdr && &pt[..QSP_TRIGGER_MAGIC.len()] == QSP_TRIGGER_MAGIC {
        let mut t = [0u8; QSP_TRIGGER_LEN];
        t.copy_from_slice(&pt[QSP_TRIGGER_MAGIC.len()..hdr]);
        (QspTriggerState::decode(&t), &pt[hdr..])
    } else {
        (QspTriggerState::default(), pt)
    }
}

/// Build a v2 session-blob plaintext = magic + trigger + snapshot.
fn qsp_join_plaintext(trig: &QspTriggerState, snapshot: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(QSP_TRIGGER_MAGIC.len() + QSP_TRIGGER_LEN + snapshot.len());
    out.extend_from_slice(QSP_TRIGGER_MAGIC);
    out.extend_from_slice(&trig.encode());
    out.extend_from_slice(snapshot);
    out
}

/// Read the persisted DH-ratchet trigger for a channel (default if no session or legacy blob).
pub(crate) fn qsp_trigger_load(peer: &str) -> QspTriggerState {
    if !channel_label_ok(peer) {
        return QspTriggerState::default();
    }
    let dir = match config_dir() {
        Ok((d, _)) => d,
        Err(_) => return QspTriggerState::default(),
    };
    let blob_path = qsp_session_blob_path(&dir, peer);
    if !blob_path.exists() {
        return QspTriggerState::default();
    }
    let blob = match fs::read(&blob_path) {
        Ok(b) => b,
        Err(_) => return QspTriggerState::default(),
    };
    match qsp_session_decrypt_blob(peer, &blob) {
        Ok(pt) => qsp_split_plaintext(&pt).0,
        Err(_) => QspTriggerState::default(),
    }
}

/// Store the session state together with an explicit DH-ratchet trigger (message path).
pub(crate) fn qsp_session_store_with_trigger(
    peer: &str,
    st: &Suite2SessionState,
    trig: &QspTriggerState,
) -> Result<(), ErrorCode> {
    qsp_session_store_inner(peer, &qsp_join_plaintext(trig, &st.snapshot_bytes()))
}

fn qsp_session_encrypt_blob(peer: &str, plaintext: &[u8]) -> Result<Vec<u8>, ErrorCode> {
    let key = qsp_session_store_key_get_or_create(peer)?;
    let cipher = ChaCha20Poly1305::new(Key::from_slice(&key));
    let mut nonce_bytes = [0u8; 12];
    #[cfg(qsc_rng_failure_test_seam)]
    qsp_rng_fill("QSC.QSP.SESSION_BLOB_NONCE", &mut nonce_bytes)?;
    #[cfg(not(qsc_rng_failure_test_seam))]
    OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);
    let aad = qsp_session_aad(peer);
    let payload = Payload {
        msg: plaintext,
        aad: aad.as_slice(),
    };
    let ciphertext = cipher
        .encrypt(nonce, payload)
        .map_err(|_| ErrorCode::ParseFailed)?;
    let mut out = Vec::with_capacity(6 + 1 + 1 + 4 + 12 + ciphertext.len());
    out.extend_from_slice(QSP_SESSION_BLOB_MAGIC);
    out.push(QSP_SESSION_BLOB_VERSION);
    out.push(12);
    out.extend_from_slice(&(ciphertext.len() as u32).to_le_bytes());
    out.extend_from_slice(&nonce_bytes);
    out.extend_from_slice(&ciphertext);
    Ok(out)
}

fn qsp_session_decrypt_blob(peer: &str, blob: &[u8]) -> Result<Vec<u8>, &'static str> {
    let min = 6 + 1 + 1 + 4 + 12;
    if blob.len() < min || &blob[..6] != QSP_SESSION_BLOB_MAGIC {
        return Err("session_decrypt_failed");
    }
    if blob[6] != QSP_SESSION_BLOB_VERSION {
        return Err("session_decrypt_failed");
    }
    let nonce_len = blob[7] as usize;
    if nonce_len != 12 {
        return Err("session_decrypt_failed");
    }
    let ct_len = u32::from_le_bytes([blob[8], blob[9], blob[10], blob[11]]) as usize;
    let need = 12 + nonce_len + ct_len;
    if blob.len() < need {
        return Err("session_decrypt_failed");
    }
    let nonce_bytes = &blob[12..12 + nonce_len];
    let ciphertext = &blob[12 + nonce_len..need];
    let key = match qsp_session_store_key_load(peer) {
        Ok(v) => v,
        Err(ErrorCode::IdentitySecretUnavailable) => return Err("session_decrypt_failed"),
        Err(_) => return Err("session_decrypt_failed"),
    };
    let cipher = ChaCha20Poly1305::new(Key::from_slice(&key));
    let nonce = Nonce::from_slice(nonce_bytes);
    let aad = qsp_session_aad(peer);
    let payload = Payload {
        msg: ciphertext,
        aad: aad.as_slice(),
    };
    cipher
        .decrypt(nonce, payload)
        .map_err(|_| "session_integrity_failed")
}

fn qsp_session_load_encrypted(
    peer: &str,
    source: ConfigSource,
    blob_path: &Path,
) -> Result<Suite2SessionState, ErrorCode> {
    enforce_safe_parents(blob_path, source)?;
    let blob = fs::read(blob_path).map_err(|_| ErrorCode::IoReadFailed)?;
    let plaintext = match qsp_session_decrypt_blob(peer, &blob) {
        Ok(v) => v,
        Err(code) => {
            emit_marker("error", Some(code), &[]);
            return Err(ErrorCode::ParseFailed);
        }
    };
    // Strip the optional v2 DH-ratchet trigger prefix; a legacy raw-snapshot plaintext is
    // returned unchanged (default trigger). The trigger itself is read via qsp_trigger_load.
    let is_v2 = plaintext.starts_with(QSP_TRIGGER_MAGIC);
    let (_trig, snapshot) = qsp_split_plaintext(&plaintext);
    let st = Suite2SessionState::restore_bytes(snapshot).map_err(|_| {
        emit_marker("error", Some("session_decrypt_failed"), &[]);
        ErrorCode::ParseFailed
    })?;
    let format = if is_v2 { "v2" } else { "v1" };
    emit_marker("session_load", None, &[("ok", "true"), ("format", format)]);
    Ok(st)
}

fn qsp_session_migrate_legacy(
    peer: &str,
    source: ConfigSource,
    legacy_path: &Path,
    blob_path: &Path,
) -> Result<Option<Suite2SessionState>, ErrorCode> {
    enforce_safe_parents(legacy_path, source)?;
    let legacy = fs::read(legacy_path).map_err(|_| ErrorCode::IoReadFailed)?;
    if legacy == QSP_SESSION_LEGACY_TOMBSTONE {
        emit_marker(
            "session_migrate",
            None,
            &[
                ("ok", "true"),
                ("action", "skipped"),
                ("reason", "already_migrated"),
            ],
        );
        return Ok(None);
    }
    let st = Suite2SessionState::restore_bytes(&legacy).map_err(|_| ErrorCode::ParseFailed)?;
    let blob = match qsp_session_encrypt_blob(peer, &legacy) {
        Ok(v) => v,
        Err(ErrorCode::IdentitySecretUnavailable) => {
            emit_marker(
                "session_migrate",
                Some("migration_blocked"),
                &[
                    ("ok", "false"),
                    ("action", "skipped"),
                    ("reason", "vault_unavailable"),
                ],
            );
            return Err(ErrorCode::IdentitySecretUnavailable);
        }
        Err(e) => return Err(e),
    };
    write_atomic(blob_path, &blob, source)?;
    if let Err(e) = write_atomic(legacy_path, QSP_SESSION_LEGACY_TOMBSTONE, source) {
        let _ = fs::remove_file(blob_path);
        return Err(e);
    }
    emit_marker(
        "session_migrate",
        None,
        &[
            ("ok", "true"),
            ("action", "imported"),
            ("reason", "legacy_plaintext"),
        ],
    );
    Ok(Some(st))
}

pub(crate) fn qsp_session_load(peer: &str) -> Result<Option<Suite2SessionState>, ErrorCode> {
    if !channel_label_ok(peer) {
        return Err(ErrorCode::ParseFailed);
    }
    let (dir, source) = config_dir()?;
    let blob_path = qsp_session_blob_path(&dir, peer);
    if blob_path.exists() {
        return qsp_session_load_encrypted(peer, source, &blob_path).map(Some);
    }
    let legacy_path = qsp_session_path(&dir, peer);
    if legacy_path.exists() {
        return qsp_session_migrate_legacy(peer, source, &legacy_path, &blob_path);
    }
    Ok(None)
}

pub(crate) fn qsp_session_store(peer: &str, st: &Suite2SessionState) -> Result<(), ErrorCode> {
    // Preserve the persisted DH-ratchet trigger across a snapshot-only store (non-message-path
    // callers: handshake, transport setup, status). The message path (qsp_pack/qsp_unpack) uses
    // qsp_session_store_with_trigger to update it explicitly.
    let trig = qsp_trigger_load(peer);
    qsp_session_store_with_trigger(peer, st, &trig)
}

fn qsp_session_store_inner(peer: &str, plaintext: &[u8]) -> Result<(), ErrorCode> {
    if !channel_label_ok(peer) {
        return Err(ErrorCode::ParseFailed);
    }
    let (dir, source) = config_dir()?;
    let sessions = qsp_sessions_dir(&dir);
    enforce_safe_parents(&sessions, source)?;
    fs::create_dir_all(&sessions).map_err(|_| ErrorCode::IoWriteFailed)?;
    let blob = qsp_session_encrypt_blob(peer, plaintext)?;
    let blob_path = qsp_session_blob_path(&dir, peer);
    write_atomic(&blob_path, &blob, source)?;
    let legacy_path = qsp_session_path(&dir, peer);
    if legacy_path.exists() {
        write_atomic(&legacy_path, QSP_SESSION_LEGACY_TOMBSTONE, source)?;
    }
    emit_marker(
        "session_store",
        None,
        &[("ok", "true"), ("format", "v2"), ("enc", "aead")],
    );
    Ok(())
}

pub(crate) fn protocol_active_or_reason_for_peer(peer: &str) -> Result<(), String> {
    let (status, reason) = qsp_status_tuple(peer);
    if status == "ACTIVE"
        || (allow_unsafe_seed_fallback_for_tests() && env::var("QSC_QSP_SEED").is_ok())
    {
        Ok(())
    } else {
        Err(reason)
    }
}

pub(crate) fn emit_protocol_inactive(reason: &str) {
    emit_marker("error", Some("protocol_inactive"), &[("reason", reason)]);
}

pub(crate) fn protocol_inactive_exit(reason: &str) -> ! {
    emit_protocol_inactive(reason);
    process::exit(1);
}

pub(crate) fn allow_unsafe_seed_fallback_for_tests() -> bool {
    env_bool("QSC_ALLOW_SEED_FALLBACK") && env_bool("QSC_UNSAFE_TEST_SEED_FALLBACK")
}

pub(crate) fn qsp_seed_from_env() -> Result<u64, &'static str> {
    let seed_str = env::var("QSC_QSP_SEED").map_err(|_| "qsp_seed_required")?;
    let seed = seed_str
        .trim()
        .parse::<u64>()
        .map_err(|_| "qsp_seed_invalid")?;
    Ok(seed)
}

pub(crate) fn kmac_out<const N: usize>(
    kmac: &StdCrypto,
    key: &[u8],
    label: &str,
    data: &[u8],
) -> [u8; N] {
    let out = kmac.kmac256(key, label, data, N);
    out[..N].try_into().expect("kmac output")
}

pub(crate) fn qsp_session_for_channel(channel: &str) -> Result<Suite2SessionState, &'static str> {
    if !channel_label_ok(channel) {
        return Err("qsp_channel_invalid");
    }
    if let Ok(Some(st)) = qsp_session_load(channel) {
        return Ok(st);
    }
    if !allow_unsafe_seed_fallback_for_tests() {
        return Err("qsp_no_session");
    }
    let seed = qsp_seed_from_env()?;
    let c = StdCrypto;
    let seed_bytes = seed.to_le_bytes();
    let seed_hash = c.sha512(&seed_bytes);
    let mut seed_key = [0u8; 32];
    seed_key.copy_from_slice(&seed_hash[..32]);

    let base = kmac_out::<32>(&c, &seed_key, "QSC.QSP.BASE", channel.as_bytes());
    let session_id = kmac_out::<16>(&c, &base, "QSC.QSP.SID", channel.as_bytes());
    let hk = kmac_out::<32>(&c, &base, "QSC.QSP.HK", b"");
    let ck_ec = kmac_out::<32>(&c, &base, "QSC.QSP.CK.EC", b"");
    let ck_pq = kmac_out::<32>(&c, &base, "QSC.QSP.CK.PQ", b"");
    let rk = kmac_out::<32>(&c, &base, "QSC.QSP.RK", b"");
    let dh_pub = kmac_out::<32>(&c, &base, "QSC.QSP.DH", b"");
    // NA-0620 (Stage 1a): seed-derived DH-ratchet material for the seed-fallback session
    // (deterministic; plumbing only — not read by the message path in Stage 1a).
    let dh_priv = kmac_out::<32>(&c, &base, "QSC.QSP.DH.PRIV", b"");

    let send = Suite2SendState {
        session_id,
        protocol_version: SUITE2_PROTOCOL_VERSION,
        suite_id: SUITE2_SUITE_ID,
        dh_pub,
        hk_s: hk,
        ck_ec,
        ck_pq,
        ns: 0,
        pn: 0,
    };
    let recv = Suite2RecvWireState {
        session_id,
        protocol_version: SUITE2_PROTOCOL_VERSION,
        suite_id: SUITE2_SUITE_ID,
        dh_pub,
        hk_r: hk,
        rk,
        ck_ec,
        ck_pq_send: ck_pq,
        ck_pq_recv: ck_pq,
        nr: 0,
        role_is_a: true,
        peer_max_adv_id_seen: 0,
        known_targets: BTreeSet::new(),
        consumed_targets: BTreeSet::new(),
        tombstoned_targets: BTreeSet::new(),
        mkskipped: Vec::new(),
    };
    let dh = Suite2DhRatchetState {
        dhs_priv: dh_priv,
        dhs_pub: dh_pub,
        dhr: dh_pub,
        rk,
    };
    Ok(Suite2SessionState { send, recv, dh })
}
