use chacha20poly1305::aead::{Aead, KeyInit, Payload};
use chacha20poly1305::{ChaCha20Poly1305, Key, Nonce};
use quantumshield_refimpl::crypto::stdcrypto::StdCrypto;
use quantumshield_refimpl::crypto::traits::{Hash, Kmac};
use quantumshield_refimpl::suite2::ratchet::{Suite2RecvWireState, Suite2SendState};
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
            if allow_seed_fallback_for_tests() {
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
            OsRng.fill_bytes(&mut key);
            let secret = hex_encode(&key);
            match vault::secret_set(QSP_SESSION_STORE_KEY_SECRET, &secret) {
                Ok(()) => Ok(key),
                Err("vault_missing" | "vault_locked") => {
                    if allow_seed_fallback_for_tests() {
                        qsp_session_test_fallback_key(peer)
                    } else {
                        Err(ErrorCode::IdentitySecretUnavailable)
                    }
                }
                Err(_) => Err(ErrorCode::IdentitySecretUnavailable),
            }
        }
        Err("vault_missing" | "vault_locked") => {
            if allow_seed_fallback_for_tests() {
                qsp_session_test_fallback_key(peer)
            } else {
                Err(ErrorCode::IdentitySecretUnavailable)
            }
        }
        Err(_) => Err(ErrorCode::IdentitySecretUnavailable),
    }
}

fn qsp_session_encrypt_blob(peer: &str, plaintext: &[u8]) -> Result<Vec<u8>, ErrorCode> {
    let key = qsp_session_store_key_get_or_create(peer)?;
    let cipher = ChaCha20Poly1305::new(Key::from_slice(&key));
    let mut nonce_bytes = [0u8; 12];
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
    let st = Suite2SessionState::restore_bytes(&plaintext).map_err(|_| {
        emit_marker("error", Some("session_decrypt_failed"), &[]);
        ErrorCode::ParseFailed
    })?;
    emit_marker("session_load", None, &[("ok", "true"), ("format", "v1")]);
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
    if !channel_label_ok(peer) {
        return Err(ErrorCode::ParseFailed);
    }
    let (dir, source) = config_dir()?;
    let sessions = qsp_sessions_dir(&dir);
    enforce_safe_parents(&sessions, source)?;
    fs::create_dir_all(&sessions).map_err(|_| ErrorCode::IoWriteFailed)?;
    let bytes = st.snapshot_bytes();
    let blob = qsp_session_encrypt_blob(peer, &bytes)?;
    let blob_path = qsp_session_blob_path(&dir, peer);
    write_atomic(&blob_path, &blob, source)?;
    let legacy_path = qsp_session_path(&dir, peer);
    if legacy_path.exists() {
        write_atomic(&legacy_path, QSP_SESSION_LEGACY_TOMBSTONE, source)?;
    }
    emit_marker(
        "session_store",
        None,
        &[("ok", "true"), ("format", "v1"), ("enc", "aead")],
    );
    Ok(())
}

pub(crate) fn protocol_active_or_reason_for_peer(peer: &str) -> Result<(), String> {
    let (status, reason) = qsp_status_tuple(peer);
    if status == "ACTIVE" || (allow_seed_fallback_for_tests() && env::var("QSC_QSP_SEED").is_ok()) {
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

pub(crate) fn allow_seed_fallback_for_tests() -> bool {
    env_bool("QSC_ALLOW_SEED_FALLBACK")
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
    if !allow_seed_fallback_for_tests() {
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
    Ok(Suite2SessionState { send, recv })
}
