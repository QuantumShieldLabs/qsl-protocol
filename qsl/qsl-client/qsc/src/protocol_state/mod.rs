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
use serde::{Deserialize, Serialize};
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
// qsc session blob's encrypted plaintext (v2 = b"QTRG" + trigger(13) + QS2S snapshot; NA-0624
// v3 additionally carries a length-delimited SCKA section between the trigger and the snapshot).
// A legacy raw-QS2S plaintext (starting with b"QS2S") migrates transparently with a default trigger. This
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

// NA-0624 (ENG-0012 Stage 2b): qsc-side SCKA state (DOC-CAN-004 §2/§3), persisted INSIDE the
// session blob's encrypted plaintext as the v3 SCKA section (v3 = b"QTRG" + trigger(13) +
// scka_len(u32 LE) + scka(scka_len) + QS2S snapshot). The refimpl Suite2SessionState / QS2S
// snapshot stays FROZEN: the advertised-key store (ML-KEM-768 receive secret keys), the peer
// advertisement, and the reseed/advertise cadence counters are client policy, kept out of the
// crypto core exactly like the v2 `QTRG` trigger. Legacy v2/v1 plaintexts migrate transparently
// with an empty SCKA section. The ML-KEM secret keys live ONLY inside the AEAD-encrypted blob.
/// Bound on live advertised receive keys (deterministic lowest-id eviction; each sk is ~2.4 KB).
pub(crate) const QSP_SCKA_ADVKEY_CAP: usize = 4;
/// Reseed cadence: originate a PQ reseed after this many sent DH boundaries (Decision 3).
pub(crate) const QSP_PQ_RESEED_N: u32 = 8;
/// Reseed cadence: or after this many seconds since the last reseed; also the advertised-key
/// rotation period (a stale unconsumed advertisement is re-advertised after this long).
pub(crate) const QSP_PQ_RESEED_T_SECS: u64 = 3600;
const QSP_SCKA_SECTION_MAX: usize = 64 * 1024;
const QSP_SCKA_SK_MAX: usize = 4096;
const QSP_SCKA_PUB_MAX: usize = 2048;
const QSP_SCKA_TOMB_MAX: usize = 4096;

/// One advertised ML-KEM-768 receive keypair (DOC-CAN-004 §3.1 step 2). `secret` is emptied
/// (zero-overwritten) when the peer consumes the target; the entry stays as a consumed marker
/// until evicted.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct SckaAdvKey {
    pub adv_id: u32,
    pub consumed: bool,
    pub secret: Vec<u8>,
}

/// The peer's most recent unconsumed advertisement (DOC-CAN-004 §3.2); removed on consumption.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct SckaPeerAdv {
    pub adv_id: u32,
    pub pubkey: Vec<u8>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct SckaLocalState {
    /// Next local advertisement id to allocate (strictly increasing; first allocation is 1).
    pub local_next_adv_id: u32,
    /// Advertised receive keys, ascending `adv_id`, at most `QSP_SCKA_ADVKEY_CAP` entries.
    pub advkeys: Vec<SckaAdvKey>,
    /// Local ids retired without a live secret (evicted or consumed-and-pruned).
    pub tombstones: BTreeSet<u32>,
    /// The peer's live (unconsumed) advertisement, if any.
    pub peer_adv: Option<SckaPeerAdv>,
    /// Highest peer advertisement id ever tracked (track_peer_adv monotonicity floor).
    pub peer_adv_max_seen: u32,
    /// Highest peer advertisement id ever CONSUMED by an originated reseed (or dropped as
    /// unusable). One-time targets: a rolled-back store must never re-consume one (G2).
    pub peer_adv_consumed_max: u32,
    /// Sent DH boundaries since the last originated reseed (cadence N).
    pub boundaries_since_reseed: u32,
    /// Unix seconds of the last originated reseed (cadence T; 0 = never).
    pub last_reseed_unix_secs: u64,
    /// Unix seconds of the last originated advertisement (rotation; 0 = never).
    pub last_adv_unix_secs: u64,
}

impl SckaLocalState {
    /// True when the SCKA path has never been used on this session (nothing to persist or track).
    pub(crate) fn is_default(&self) -> bool {
        *self == SckaLocalState::default()
    }

    /// The live (unconsumed, secret-bearing) advertised key with the highest id, if any.
    pub(crate) fn live_advkey(&self) -> Option<&SckaAdvKey> {
        self.advkeys
            .iter()
            .rev()
            .find(|k| !k.consumed && !k.secret.is_empty())
    }

    /// Insert a freshly advertised keypair, evicting deterministically (lowest id first,
    /// consumed entries before live ones) to stay within `QSP_SCKA_ADVKEY_CAP`. Evicted and
    /// pruned ids are tombstoned so they are never re-accepted.
    pub(crate) fn insert_advkey(&mut self, adv_id: u32, secret: Vec<u8>) {
        while self.advkeys.len() >= QSP_SCKA_ADVKEY_CAP {
            let evict_idx = self
                .advkeys
                .iter()
                .position(|k| k.consumed)
                .unwrap_or(0usize);
            let mut evicted = self.advkeys.remove(evict_idx);
            for b in evicted.secret.iter_mut() {
                *b = 0;
            }
            self.tombstones.insert(evicted.adv_id);
        }
        self.advkeys.push(SckaAdvKey {
            adv_id,
            consumed: false,
            secret,
        });
        self.advkeys.sort_by_key(|k| k.adv_id);
    }

    /// Mark a local advertised key consumed (one-time use): zero-overwrite and drop the secret.
    pub(crate) fn consume_advkey(&mut self, adv_id: u32) {
        if let Some(k) = self.advkeys.iter_mut().find(|k| k.adv_id == adv_id) {
            for b in k.secret.iter_mut() {
                *b = 0;
            }
            k.secret = Vec::new();
            k.consumed = true;
        }
        self.tombstones.insert(adv_id);
    }

    /// Encode the SCKA section (empty for a default state, so a non-advertising session keeps
    /// `scka_len == 0` and the v3 plaintext stays byte-stable modulo the 4-byte length field).
    fn encode(&self) -> Vec<u8> {
        if self.is_default() {
            return Vec::new();
        }
        let mut out = Vec::new();
        out.extend_from_slice(&self.local_next_adv_id.to_le_bytes());
        out.extend_from_slice(&(self.advkeys.len() as u32).to_le_bytes());
        for k in &self.advkeys {
            out.extend_from_slice(&k.adv_id.to_le_bytes());
            out.push(k.consumed as u8);
            out.extend_from_slice(&(k.secret.len() as u16).to_le_bytes());
            out.extend_from_slice(&k.secret);
        }
        out.extend_from_slice(&(self.tombstones.len() as u32).to_le_bytes());
        for id in &self.tombstones {
            out.extend_from_slice(&id.to_le_bytes());
        }
        match &self.peer_adv {
            Some(p) => {
                out.push(1u8);
                out.extend_from_slice(&p.adv_id.to_le_bytes());
                out.extend_from_slice(&(p.pubkey.len() as u16).to_le_bytes());
                out.extend_from_slice(&p.pubkey);
            }
            None => out.push(0u8),
        }
        out.extend_from_slice(&self.peer_adv_max_seen.to_le_bytes());
        out.extend_from_slice(&self.peer_adv_consumed_max.to_le_bytes());
        out.extend_from_slice(&self.boundaries_since_reseed.to_le_bytes());
        out.extend_from_slice(&self.last_reseed_unix_secs.to_le_bytes());
        out.extend_from_slice(&self.last_adv_unix_secs.to_le_bytes());
        out
    }

    /// Fail-closed decode: exact-length consumption with hard caps (mirror the QS2S
    /// `restore_bytes` restore caps). A zero-length section is the default state.
    fn decode(b: &[u8]) -> Result<SckaLocalState, ()> {
        if b.is_empty() {
            return Ok(SckaLocalState::default());
        }
        let mut pos = 0usize;
        let take = |pos: &mut usize, n: usize| -> Result<&[u8], ()> {
            let end = pos.checked_add(n).ok_or(())?;
            if end > b.len() {
                return Err(());
            }
            let s = &b[*pos..end];
            *pos = end;
            Ok(s)
        };
        let read_u32 = |pos: &mut usize| -> Result<u32, ()> {
            let s = take(pos, 4)?;
            Ok(u32::from_le_bytes([s[0], s[1], s[2], s[3]]))
        };
        let read_u16 = |pos: &mut usize| -> Result<u16, ()> {
            let s = take(pos, 2)?;
            Ok(u16::from_le_bytes([s[0], s[1]]))
        };
        let read_u64 = |pos: &mut usize| -> Result<u64, ()> {
            let s = take(pos, 8)?;
            Ok(u64::from_le_bytes([
                s[0], s[1], s[2], s[3], s[4], s[5], s[6], s[7],
            ]))
        };
        let local_next_adv_id = read_u32(&mut pos)?;
        let advkey_count = read_u32(&mut pos)? as usize;
        if advkey_count > QSP_SCKA_ADVKEY_CAP {
            return Err(());
        }
        let mut advkeys = Vec::with_capacity(advkey_count);
        let mut prev_id: Option<u32> = None;
        for _ in 0..advkey_count {
            let adv_id = read_u32(&mut pos)?;
            if let Some(p) = prev_id {
                if adv_id <= p {
                    return Err(());
                }
            }
            prev_id = Some(adv_id);
            let consumed = match take(&mut pos, 1)?[0] {
                0 => false,
                1 => true,
                _ => return Err(()),
            };
            let sk_len = read_u16(&mut pos)? as usize;
            if sk_len > QSP_SCKA_SK_MAX {
                return Err(());
            }
            let secret = take(&mut pos, sk_len)?.to_vec();
            advkeys.push(SckaAdvKey {
                adv_id,
                consumed,
                secret,
            });
        }
        let tomb_count = read_u32(&mut pos)? as usize;
        if tomb_count > QSP_SCKA_TOMB_MAX {
            return Err(());
        }
        let mut tombstones = BTreeSet::new();
        for _ in 0..tomb_count {
            tombstones.insert(read_u32(&mut pos)?);
        }
        if tombstones.len() != tomb_count {
            return Err(());
        }
        let peer_adv = match take(&mut pos, 1)?[0] {
            0 => None,
            1 => {
                let adv_id = read_u32(&mut pos)?;
                let pub_len = read_u16(&mut pos)? as usize;
                if pub_len > QSP_SCKA_PUB_MAX {
                    return Err(());
                }
                let pubkey = take(&mut pos, pub_len)?.to_vec();
                Some(SckaPeerAdv { adv_id, pubkey })
            }
            _ => return Err(()),
        };
        let peer_adv_max_seen = read_u32(&mut pos)?;
        let peer_adv_consumed_max = read_u32(&mut pos)?;
        let boundaries_since_reseed = read_u32(&mut pos)?;
        let last_reseed_unix_secs = read_u64(&mut pos)?;
        let last_adv_unix_secs = read_u64(&mut pos)?;
        if pos != b.len() {
            return Err(());
        }
        Ok(SckaLocalState {
            local_next_adv_id,
            advkeys,
            tombstones,
            peer_adv,
            peer_adv_max_seen,
            peer_adv_consumed_max,
            boundaries_since_reseed,
            last_reseed_unix_secs,
            last_adv_unix_secs,
        })
    }
}

/// G2 rollback guard: a small monotonic side-record (ids only, NO key material), stored next to
/// the session blob and merge-updated on every store — the qsc mirror of the interop actor's
/// `dur_scka` record (`check_dur_scka_rollback`). A session blob whose SCKA counters regress
/// below this record fails closed on load (`session_rollback_detected`).
#[derive(Serialize, Deserialize)]
struct SckaMonoRecord {
    version: u8,
    peer_max_adv_id_seen: u32,
    local_next_adv_id: u32,
    peer_adv_max_seen: u32,
    #[serde(default)]
    peer_adv_consumed_max: u32,
    tombstones: Vec<u32>,
}

fn qsp_scka_mono_path(dir: &Path, peer: &str) -> PathBuf {
    qsp_sessions_dir(dir).join(format!("{}.scka.json", peer))
}

fn qsp_scka_mono_load(dir: &Path, peer: &str) -> Result<Option<SckaMonoRecord>, ()> {
    let path = qsp_scka_mono_path(dir, peer);
    if !path.exists() {
        return Ok(None);
    }
    let bytes = fs::read(&path).map_err(|_| ())?;
    let rec: SckaMonoRecord = serde_json::from_slice(&bytes).map_err(|_| ())?;
    if rec.version != 1 || rec.tombstones.len() > QSP_SCKA_TOMB_MAX {
        return Err(());
    }
    Ok(Some(rec))
}

fn qsp_scka_rollback_check(
    rec: &SckaMonoRecord,
    recv: &Suite2RecvWireState,
    scka: &SckaLocalState,
) -> Result<(), ()> {
    if recv.peer_max_adv_id_seen < rec.peer_max_adv_id_seen {
        return Err(());
    }
    if scka.local_next_adv_id < rec.local_next_adv_id {
        return Err(());
    }
    if scka.peer_adv_max_seen < rec.peer_adv_max_seen {
        return Err(());
    }
    if scka.peer_adv_consumed_max < rec.peer_adv_consumed_max {
        return Err(());
    }
    for t in rec.tombstones.iter() {
        if !recv.tombstoned_targets.contains(t) && !scka.tombstones.contains(t) {
            return Err(());
        }
    }
    Ok(())
}

fn qsp_scka_mono_update(
    dir: &Path,
    source: ConfigSource,
    peer: &str,
    recv: &Suite2RecvWireState,
    scka: &SckaLocalState,
) -> Result<(), ErrorCode> {
    // Nothing to guard until SCKA state exists; avoid churning a side-record for every
    // non-SCKA session (keeps pre-Stage-2b store behavior unchanged on disk).
    let prev = qsp_scka_mono_load(dir, peer).map_err(|_| ErrorCode::ParseFailed)?;
    if prev.is_none() && scka.is_default() && recv.peer_max_adv_id_seen == 0 {
        return Ok(());
    }
    let mut tombs: BTreeSet<u32> = recv.tombstoned_targets.iter().copied().collect();
    tombs.extend(scka.tombstones.iter().copied());
    let mut rec = SckaMonoRecord {
        version: 1,
        peer_max_adv_id_seen: recv.peer_max_adv_id_seen,
        local_next_adv_id: scka.local_next_adv_id,
        peer_adv_max_seen: scka.peer_adv_max_seen,
        peer_adv_consumed_max: scka.peer_adv_consumed_max,
        tombstones: Vec::new(),
    };
    if let Some(p) = prev {
        rec.peer_max_adv_id_seen = rec.peer_max_adv_id_seen.max(p.peer_max_adv_id_seen);
        rec.local_next_adv_id = rec.local_next_adv_id.max(p.local_next_adv_id);
        rec.peer_adv_max_seen = rec.peer_adv_max_seen.max(p.peer_adv_max_seen);
        rec.peer_adv_consumed_max = rec.peer_adv_consumed_max.max(p.peer_adv_consumed_max);
        tombs.extend(p.tombstones.iter().copied());
    }
    while tombs.len() > QSP_SCKA_TOMB_MAX {
        let lowest = match tombs.iter().next().copied() {
            Some(v) => v,
            None => break,
        };
        tombs.remove(&lowest);
    }
    rec.tombstones = tombs.into_iter().collect();
    let bytes = serde_json::to_vec(&rec).map_err(|_| ErrorCode::ParseFailed)?;
    write_atomic(&qsp_scka_mono_path(dir, peer), &bytes, source)
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

/// The QS2S snapshot magic (the refimpl `Suite2SessionState` snapshot prefix), used to tell a
/// legacy v2 plaintext (trigger + raw snapshot) from a v3 plaintext (trigger + SCKA section +
/// snapshot) without ambiguity.
const QS2S_SNAPSHOT_MAGIC: &[u8; 4] = b"QS2S";

/// NA-0626 (Operator Decision 1): how a session-blob plaintext fails to split. A pre-v3 layout
/// (legacy raw-QS2S v1 plaintext, or the pre-SCKA v2 trigger+snapshot layout) necessarily
/// carries a pre-v3 QS2S section, which `restore_bytes` no longer accepts — those stored
/// sessions are UNRECOVERABLE by design (no migration; the session must be re-established),
/// distinct from a merely malformed blob.
#[derive(Debug, PartialEq, Eq)]
enum QspPlaintextError {
    UnrecoverableLegacy,
    Malformed,
}

/// Split a decrypted session-blob plaintext into (trigger, SCKA state, raw QS2S snapshot).
/// v3 = magic + trigger + scka_len(u32 LE) + scka + snapshot is the ONLY accepted layout
/// (NA-0626: the v2 trigger+raw-snapshot and legacy v1 raw-snapshot migration branches are
/// REMOVED — they necessarily carry a pre-v3 QS2S section and are unrecoverable). Fail-closed
/// on a malformed v3 SCKA section.
fn qsp_split_plaintext(
    pt: &[u8],
) -> Result<(QspTriggerState, SckaLocalState, &[u8]), QspPlaintextError> {
    let hdr = QSP_TRIGGER_MAGIC.len() + QSP_TRIGGER_LEN;
    if pt.len() >= hdr && &pt[..QSP_TRIGGER_MAGIC.len()] == QSP_TRIGGER_MAGIC {
        let mut t = [0u8; QSP_TRIGGER_LEN];
        t.copy_from_slice(&pt[QSP_TRIGGER_MAGIC.len()..hdr]);
        let trig = QspTriggerState::decode(&t);
        let rest = &pt[hdr..];
        if rest.starts_with(QS2S_SNAPSHOT_MAGIC) {
            // Pre-SCKA v2 layout (a QS2S snapshot follows the trigger directly): unrecoverable.
            return Err(QspPlaintextError::UnrecoverableLegacy);
        }
        if rest.len() < 4 {
            return Err(QspPlaintextError::Malformed);
        }
        let scka_len = u32::from_le_bytes([rest[0], rest[1], rest[2], rest[3]]) as usize;
        if scka_len > QSP_SCKA_SECTION_MAX || 4 + scka_len > rest.len() {
            return Err(QspPlaintextError::Malformed);
        }
        let scka = SckaLocalState::decode(&rest[4..4 + scka_len])
            .map_err(|()| QspPlaintextError::Malformed)?;
        Ok((trig, scka, &rest[4 + scka_len..]))
    } else {
        // Legacy v1 layout (a raw QS2S snapshot with no trigger/SCKA prefix): unrecoverable.
        Err(QspPlaintextError::UnrecoverableLegacy)
    }
}

/// Build a v3 session-blob plaintext = magic + trigger + scka_len + scka + snapshot.
fn qsp_join_plaintext(trig: &QspTriggerState, scka: &SckaLocalState, snapshot: &[u8]) -> Vec<u8> {
    let scka_bytes = scka.encode();
    let mut out = Vec::with_capacity(
        QSP_TRIGGER_MAGIC.len() + QSP_TRIGGER_LEN + 4 + scka_bytes.len() + snapshot.len(),
    );
    out.extend_from_slice(QSP_TRIGGER_MAGIC);
    out.extend_from_slice(&trig.encode());
    out.extend_from_slice(&(scka_bytes.len() as u32).to_le_bytes());
    out.extend_from_slice(&scka_bytes);
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
        Ok(pt) => match qsp_split_plaintext(&pt) {
            Ok((trig, _, _)) => trig,
            Err(_) => QspTriggerState::default(),
        },
        Err(_) => QspTriggerState::default(),
    }
}

/// Read the persisted SCKA state for a channel (default if no session or a pre-v3 blob).
pub(crate) fn qsp_scka_load(peer: &str) -> SckaLocalState {
    if !channel_label_ok(peer) {
        return SckaLocalState::default();
    }
    let dir = match config_dir() {
        Ok((d, _)) => d,
        Err(_) => return SckaLocalState::default(),
    };
    let blob_path = qsp_session_blob_path(&dir, peer);
    if !blob_path.exists() {
        return SckaLocalState::default();
    }
    let blob = match fs::read(&blob_path) {
        Ok(b) => b,
        Err(_) => return SckaLocalState::default(),
    };
    match qsp_session_decrypt_blob(peer, &blob) {
        Ok(pt) => match qsp_split_plaintext(&pt) {
            Ok((_, scka, _)) => scka,
            Err(_) => SckaLocalState::default(),
        },
        Err(_) => SckaLocalState::default(),
    }
}

/// Persist an updated SCKA state against the CURRENTLY STORED session snapshot and trigger
/// (read-modify-write). Used by the message path at SCKA mutation points (an advertised secret
/// key MUST be durable before its advertisement can leave the client; a consumed peer
/// advertisement MUST be durable before the reseed wire exists — fail closed).
pub(crate) fn qsp_scka_store(peer: &str, scka: &SckaLocalState) -> Result<(), ErrorCode> {
    if !channel_label_ok(peer) {
        return Err(ErrorCode::ParseFailed);
    }
    let (dir, source) = config_dir()?;
    let blob_path = qsp_session_blob_path(&dir, peer);
    let blob = fs::read(&blob_path).map_err(|_| ErrorCode::IoReadFailed)?;
    let pt = qsp_session_decrypt_blob(peer, &blob).map_err(|_| ErrorCode::ParseFailed)?;
    let (trig, _old_scka, snapshot) =
        qsp_split_plaintext(&pt).map_err(|_| ErrorCode::ParseFailed)?;
    let recv = Suite2SessionState::restore_bytes(snapshot)
        .map_err(|_| ErrorCode::ParseFailed)?
        .recv;
    qsp_session_store_inner(peer, &qsp_join_plaintext(&trig, scka, snapshot))?;
    qsp_scka_mono_update(&dir, source, peer, &recv, scka)
}

/// Store the session state together with an explicit DH-ratchet trigger (message path). The
/// persisted SCKA state is preserved.
pub(crate) fn qsp_session_store_with_trigger(
    peer: &str,
    st: &Suite2SessionState,
    trig: &QspTriggerState,
) -> Result<(), ErrorCode> {
    let scka = qsp_scka_load(peer);
    qsp_session_store_with_trigger_scka(peer, st, trig, &scka)
}

/// Store the session state together with an explicit trigger AND SCKA state (v3 blob), then
/// merge-update the G2 monotonic side-record.
pub(crate) fn qsp_session_store_with_trigger_scka(
    peer: &str,
    st: &Suite2SessionState,
    trig: &QspTriggerState,
    scka: &SckaLocalState,
) -> Result<(), ErrorCode> {
    qsp_session_store_inner(peer, &qsp_join_plaintext(trig, scka, &st.snapshot_bytes()))?;
    let (dir, source) = config_dir()?;
    qsp_scka_mono_update(&dir, source, peer, &st.recv, scka)
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
    // Strip the v3 trigger + SCKA prefix. NA-0626 (Operator Decision 1): a pre-v3 blob layout
    // or a pre-v3 QS2S section is UNRECOVERABLE — a DISTINCT deterministic marker fires, the
    // stored blob is never mutated, and the session must be re-established (no migration; a v2
    // snapshot whose two root copies diverged cannot be soundly collapsed to the single root).
    let (_trig, scka, snapshot) = qsp_split_plaintext(&plaintext).map_err(|e| {
        let code = match e {
            QspPlaintextError::UnrecoverableLegacy => "session_unsupported_version",
            QspPlaintextError::Malformed => "session_decrypt_failed",
        };
        emit_marker("error", Some(code), &[]);
        ErrorCode::ParseFailed
    })?;
    let st = Suite2SessionState::restore_bytes(snapshot).map_err(|_| {
        // Valid magic + non-v3 version = the unrecoverable class; anything else is generic.
        let code =
            if snapshot.len() >= 5 && snapshot.starts_with(QS2S_SNAPSHOT_MAGIC) && snapshot[4] != 3
            {
                "session_unsupported_version"
            } else {
                "session_decrypt_failed"
            };
        emit_marker("error", Some(code), &[]);
        ErrorCode::ParseFailed
    })?;
    // NA-0624 G2: SCKA monotonicity rollback guard — a session blob whose SCKA counters or
    // tombstones regress below the persisted monotonic side-record fails closed.
    if let Ok((dir, _)) = config_dir() {
        match qsp_scka_mono_load(&dir, peer) {
            Ok(Some(rec)) => {
                if qsp_scka_rollback_check(&rec, &st.recv, &scka).is_err() {
                    emit_marker("error", Some("session_rollback_detected"), &[]);
                    return Err(ErrorCode::ParseFailed);
                }
            }
            Ok(None) => {}
            Err(()) => {
                emit_marker("error", Some("session_rollback_detected"), &[]);
                return Err(ErrorCode::ParseFailed);
            }
        }
    }
    emit_marker("session_load", None, &[("ok", "true"), ("format", "v3")]);
    Ok(st)
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
    // NA-0626 (Operator Decision 1): the legacy plaintext-session migration branch is REMOVED —
    // a legacy plaintext file necessarily holds a pre-v3 QS2S snapshot, which is unrecoverable
    // by design. Distinct deterministic marker; the file is left untouched; the session must be
    // re-established.
    let legacy_path = qsp_session_path(&dir, peer);
    if legacy_path.exists() {
        emit_marker("error", Some("session_unsupported_version"), &[]);
        return Err(ErrorCode::ParseFailed);
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
        &[("ok", "true"), ("format", "v3"), ("enc", "aead")],
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
    };
    Ok(Suite2SessionState { rk, send, recv, dh })
}

// NA-0624 (ENG-0012 Stage 2b): co-located tests for the v3 SCKA persistence layer — the
// section codec (fail-closed), the v3/v2/v1 plaintext split, and the G2 rollback guard.
#[cfg(test)]
mod scka_tests {
    use super::*;

    fn sample_scka() -> SckaLocalState {
        let mut tombstones = BTreeSet::new();
        tombstones.insert(1);
        tombstones.insert(3);
        SckaLocalState {
            local_next_adv_id: 5,
            advkeys: vec![
                SckaAdvKey {
                    adv_id: 2,
                    consumed: true,
                    secret: Vec::new(),
                },
                SckaAdvKey {
                    adv_id: 4,
                    consumed: false,
                    secret: vec![0xA5; 2400],
                },
            ],
            tombstones,
            peer_adv: Some(SckaPeerAdv {
                adv_id: 7,
                pubkey: vec![0x5A; 1184],
            }),
            peer_adv_max_seen: 7,
            peer_adv_consumed_max: 6,
            boundaries_since_reseed: 3,
            last_reseed_unix_secs: 1_700_000_000,
            last_adv_unix_secs: 1_700_000_100,
        }
    }

    #[test]
    fn scka_section_roundtrips() {
        let s = sample_scka();
        let bytes = s.encode();
        assert!(!bytes.is_empty());
        let d = SckaLocalState::decode(&bytes).expect("decode");
        assert_eq!(d, s);
    }

    #[test]
    fn scka_default_is_empty_section() {
        assert!(SckaLocalState::default().encode().is_empty());
        assert_eq!(
            SckaLocalState::decode(&[]).expect("decode empty"),
            SckaLocalState::default()
        );
    }

    #[test]
    fn scka_decode_fails_closed() {
        let s = sample_scka();
        let bytes = s.encode();
        // Trailing garbage rejects (exact-length consumption).
        let mut trailing = bytes.clone();
        trailing.push(0);
        assert!(SckaLocalState::decode(&trailing).is_err());
        // Truncation rejects at every prefix length.
        for cut in 1..bytes.len() {
            assert!(
                SckaLocalState::decode(&bytes[..cut]).is_err(),
                "prefix of len {cut} must reject"
            );
        }
        // An oversize advkey count rejects.
        let mut oversize = bytes.clone();
        oversize[4..8].copy_from_slice(&(QSP_SCKA_ADVKEY_CAP as u32 + 1).to_le_bytes());
        assert!(SckaLocalState::decode(&oversize).is_err());
        // A non-boolean consumed byte rejects.
        let mut badflag = bytes.clone();
        badflag[12] = 2;
        assert!(SckaLocalState::decode(&badflag).is_err());
    }

    #[test]
    fn v3_plaintext_join_split_roundtrips() {
        let trig = QspTriggerState {
            pending_send_ratchet: true,
            msgs_since_ratchet: 9,
            last_ratchet_unix_secs: 42,
        };
        let scka = sample_scka();
        let snapshot = b"QS2Sfake-snapshot-bytes".to_vec();
        // v3 round trip.
        let pt = qsp_join_plaintext(&trig, &scka, &snapshot);
        let (t, s, snap) = qsp_split_plaintext(&pt).expect("split v3");
        assert_eq!(t, trig);
        assert_eq!(s, scka);
        assert_eq!(snap, snapshot.as_slice());
        // v3 with an empty SCKA section keeps the snapshot at offset 17 + 4.
        let pt0 = qsp_join_plaintext(&trig, &SckaLocalState::default(), &snapshot);
        assert_eq!(&pt0[17..21], &0u32.to_le_bytes());
        let (_, s0, snap0) = qsp_split_plaintext(&pt0).expect("split v3 empty");
        assert!(s0.is_default());
        assert_eq!(snap0, snapshot.as_slice());
        // A malformed v3 SCKA section fails closed (Malformed, not the legacy class).
        let mut bad = qsp_join_plaintext(&trig, &scka, &snapshot);
        bad.truncate(30);
        assert_eq!(
            qsp_split_plaintext(&bad).err(),
            Some(QspPlaintextError::Malformed)
        );
    }

    // NA-0626 (Operator Decision 1): one test per REMOVED legacy-migration branch. A pre-v3
    // blob layout necessarily carries a pre-v3 QS2S section (which restore_bytes no longer
    // accepts) and is UNRECOVERABLE by design — distinct from a merely malformed blob, and
    // deterministic.
    #[test]
    fn v2_plaintext_layout_is_unrecoverable() {
        let trig = QspTriggerState {
            pending_send_ratchet: true,
            msgs_since_ratchet: 9,
            last_ratchet_unix_secs: 42,
        };
        // The removed v2 branch: trigger + raw QS2S snapshot, no SCKA section.
        let mut v2 = Vec::new();
        v2.extend_from_slice(QSP_TRIGGER_MAGIC);
        v2.extend_from_slice(&trig.encode());
        v2.extend_from_slice(b"QS2Sfake-snapshot-bytes");
        let before = v2.clone();
        assert_eq!(
            qsp_split_plaintext(&v2).err(),
            Some(QspPlaintextError::UnrecoverableLegacy)
        );
        assert_eq!(
            qsp_split_plaintext(&v2).err(),
            Some(QspPlaintextError::UnrecoverableLegacy),
            "deterministic"
        );
        assert_eq!(before, v2, "the input is never mutated");
    }

    #[test]
    fn v1_raw_snapshot_plaintext_is_unrecoverable() {
        // The removed v1 branch: a raw QS2S snapshot with no trigger/SCKA prefix.
        let v1 = b"QS2Sfake-snapshot-bytes".to_vec();
        let before = v1.clone();
        assert_eq!(
            qsp_split_plaintext(&v1).err(),
            Some(QspPlaintextError::UnrecoverableLegacy)
        );
        assert_eq!(before, v1, "the input is never mutated");
    }

    fn recv_state_with(peer_max: u32, tombs: &[u32]) -> Suite2RecvWireState {
        Suite2RecvWireState {
            session_id: [0u8; 16],
            protocol_version: SUITE2_PROTOCOL_VERSION,
            suite_id: SUITE2_SUITE_ID,
            dh_pub: [0u8; 32],
            hk_r: [0u8; 32],
            ck_ec: [0u8; 32],
            ck_pq_send: [0u8; 32],
            ck_pq_recv: [0u8; 32],
            nr: 0,
            role_is_a: true,
            peer_max_adv_id_seen: peer_max,
            known_targets: BTreeSet::new(),
            consumed_targets: BTreeSet::new(),
            tombstoned_targets: tombs.iter().copied().collect(),
            mkskipped: Vec::new(),
        }
    }

    #[test]
    fn scka_rollback_guard_rejects_regressions_and_accepts_progress() {
        let rec = SckaMonoRecord {
            version: 1,
            peer_max_adv_id_seen: 3,
            local_next_adv_id: 5,
            peer_adv_max_seen: 2,
            peer_adv_consumed_max: 2,
            tombstones: vec![1, 3],
        };
        let mut scka = SckaLocalState {
            local_next_adv_id: 5,
            peer_adv_max_seen: 2,
            peer_adv_consumed_max: 2,
            ..SckaLocalState::default()
        };
        // Equal state passes.
        assert!(qsp_scka_rollback_check(&rec, &recv_state_with(3, &[1, 3]), &scka).is_ok());
        // Progressed state passes.
        scka.local_next_adv_id = 9;
        assert!(qsp_scka_rollback_check(&rec, &recv_state_with(4, &[1, 3, 4]), &scka).is_ok());
        // A regressed peer_max fails closed.
        assert!(qsp_scka_rollback_check(&rec, &recv_state_with(2, &[1, 3]), &scka).is_err());
        // A regressed local_next fails closed.
        scka.local_next_adv_id = 4;
        assert!(qsp_scka_rollback_check(&rec, &recv_state_with(3, &[1, 3]), &scka).is_err());
        scka.local_next_adv_id = 5;
        // A regressed peer_adv_max fails closed.
        scka.peer_adv_max_seen = 1;
        assert!(qsp_scka_rollback_check(&rec, &recv_state_with(3, &[1, 3]), &scka).is_err());
        scka.peer_adv_max_seen = 2;
        // A regressed peer_adv_consumed_max fails closed (a rolled-back store must never
        // re-consume a one-time peer target).
        scka.peer_adv_consumed_max = 1;
        assert!(qsp_scka_rollback_check(&rec, &recv_state_with(3, &[1, 3]), &scka).is_err());
        scka.peer_adv_consumed_max = 2;
        // A missing tombstone fails closed (SCKA-section tombstones also satisfy it).
        assert!(qsp_scka_rollback_check(&rec, &recv_state_with(3, &[1]), &scka).is_err());
        scka.tombstones.insert(3);
        assert!(qsp_scka_rollback_check(&rec, &recv_state_with(3, &[1]), &scka).is_ok());
    }

    #[test]
    fn scka_advkey_cap_evicts_deterministically_and_tombstones() {
        let mut s = SckaLocalState::default();
        for id in 1..=(QSP_SCKA_ADVKEY_CAP as u32) {
            s.insert_advkey(id, vec![id as u8; 8]);
        }
        assert_eq!(s.advkeys.len(), QSP_SCKA_ADVKEY_CAP);
        // Consumed entries evict before live ones; else the lowest id evicts.
        s.consume_advkey(2);
        s.insert_advkey(99, vec![9; 8]);
        assert_eq!(s.advkeys.len(), QSP_SCKA_ADVKEY_CAP);
        assert!(s.advkeys.iter().all(|k| k.adv_id != 2));
        assert!(s.tombstones.contains(&2));
        s.insert_advkey(100, vec![10; 8]);
        assert!(s.advkeys.iter().all(|k| k.adv_id != 1));
        assert!(s.tombstones.contains(&1));
        // The live key is the highest unconsumed one; consumption erases the secret.
        assert_eq!(s.live_advkey().expect("live").adv_id, 100);
        s.consume_advkey(100);
        assert!(s
            .advkeys
            .iter()
            .any(|k| k.adv_id == 100 && k.consumed && k.secret.is_empty()));
        assert_eq!(s.live_advkey().map(|k| k.adv_id), Some(99));
    }
}
