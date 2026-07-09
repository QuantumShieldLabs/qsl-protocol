//! Suite-2 ratchet surface (minimal helpers).

use crate::crypto::traits::{Aead, CryptoError, Hash, Kmac, X25519Dh, X25519Priv, X25519Pub};
#[cfg(test)]
use std::cell::Cell;
use std::collections::{BTreeSet, HashSet};

use crate::suite2::{binding, parse, scka, types};

const MAX_HEADER_ATTEMPTS: usize = 100;
const MAX_SKIP: u32 = 1000;
const MAX_MKSKIPPED: usize = 1000;
const HDR_CT_LEN: usize = 24;
const BODY_CT_MIN: usize = 16;
const REJECT_S2_CHAINKEY_UNSET: &str =
    "REJECT_S2_CHAINKEY_UNSET; reason_code=REJECT_S2_CHAINKEY_UNSET";
// NA-0618 (ENG-0013): terminal fail-closed reject when a symmetric message counter would
// pass u32::MAX. Sessions never re-key (no DH ratchet / PQ reseed fires), so a saturated
// counter with static header keys would repeat header ciphertext (nonce-reuse class);
// hard-stop instead. Local reason code only (not wire-transmitted). Mirrors the sibling
// qsp module's `ns == u32::MAX` guard.
const REJECT_S2_COUNTER_OVERFLOW: &str =
    "REJECT_S2_COUNTER_OVERFLOW; reason_code=REJECT_S2_COUNTER_OVERFLOW";

/// NA-0618 (ENG-0013): fail-closed increment of a u32 symmetric message counter. Returns the
/// next counter value, or `REJECT_S2_COUNTER_OVERFLOW` if it would pass `u32::MAX`. Used at
/// every `ns`/`nr` advance in place of `saturating_add`, so a saturated counter can never
/// freeze (which, with static header keys, would reuse a header nonce/ciphertext).
#[inline]
fn checked_counter_inc(counter: u32) -> Result<u32, &'static str> {
    counter.checked_add(1).ok_or(REJECT_S2_COUNTER_OVERFLOW)
}

#[cfg(test)]
thread_local! {
    static S2_HDR_TRY_COUNT_NONBOUNDARY: Cell<usize> = const { Cell::new(0) };
    static S2_HDR_TRY_COUNT_BOUNDARY: Cell<usize> = const { Cell::new(0) };
}

fn kmac32(kmac: &dyn Kmac, key: &[u8], label: &str, data: &[u8]) -> Result<[u8; 32], CryptoError> {
    let out = kmac.kmac256(key, label, data, 32);
    if out.len() != 32 {
        return Err(CryptoError::InvalidKey);
    }
    let arr: [u8; 32] = out
        .as_slice()
        .try_into()
        .map_err(|_| CryptoError::InvalidKey)?;
    Ok(arr)
}

fn is_zero32(v: &[u8; 32]) -> bool {
    v.iter().all(|b| *b == 0)
}

// NA-0621 (ENG-0012 Stage 1b-i): classical DH-ratchet key derivations (DOC-CAN-003 §3.3.2,
// §3.4, §8.1). These are used ONLY by the DH-boundary send/receive paths; the non-boundary
// message path (send_wire / recv_nonboundary_ooo) is unchanged.

/// §3.3.2 Root update from DH ratchet: `KMAC256(RK, "QSP5.0/RKDH", dh_out, 64)` -> (RK', CK_ec0).
fn kdf_rk_dh(
    kmac: &dyn Kmac,
    rk: &[u8; 32],
    dh_out: &[u8; 32],
) -> Result<([u8; 32], [u8; 32]), CryptoError> {
    let out = kmac.kmac256(rk, "QSP5.0/RKDH", dh_out, 64);
    if out.len() != 64 {
        return Err(CryptoError::InvalidKey);
    }
    let mut rk1 = [0u8; 32];
    let mut ck = [0u8; 32];
    rk1.copy_from_slice(&out[0..32]);
    ck.copy_from_slice(&out[32..64]);
    Ok((rk1, ck))
}

/// §3.4/§8.1 directional header keys from RK. `next` selects HK (false) vs NHK (true).
/// NA-0625 (ENG-0023): `pub` so the conformance harness (refimpl actor) can construct
/// NHK-sealed boundary frames exactly as the senders do; pure derivation, no state.
pub fn header_key(
    kmac: &dyn Kmac,
    rk: &[u8; 32],
    a_to_b: bool,
    next: bool,
) -> Result<[u8; 32], CryptoError> {
    let label = match (next, a_to_b) {
        (false, true) => "QSP5.0/HK/A->B",
        (false, false) => "QSP5.0/HK/B->A",
        (true, true) => "QSP5.0/NHK/A->B",
        (true, false) => "QSP5.0/NHK/B->A",
    };
    kmac32(kmac, rk, label, &[0x01])
}

/// NA-0625 (ENG-0023, Operator Decision 1): SPQR-style SCKA control-plane authenticator —
/// `adv_mac = KMAC32(RK, "QSP5.0/ADVAUTH", u32be(pq_adv_id) || pq_adv_pub || [0x01])`, carried
/// as the FIRST 32 BYTES of the sealed ADV body plaintext (`body_pt = adv_mac || app_payload`).
/// The pq_prefix layout is normatively fixed by DOC-CAN-004 §1.1/§1.3 and cannot carry it. `RK`
/// is the canonical session root at send time; the receiver verifies under its canonical root,
/// so the MAC inherits exactly the header-key synchronization envelope. KMAC only; no new
/// primitive; no new reason code (a mismatch reuses `REJECT_S2_BODY_AUTH_FAIL`).
const ADV_MAC_LEN: usize = 32;
fn adv_auth_mac(
    kmac: &dyn Kmac,
    rk: &[u8; 32],
    pq_adv_id: u32,
    pq_adv_pub: &[u8],
) -> Result<[u8; 32], CryptoError> {
    let mut data = Vec::with_capacity(4 + pq_adv_pub.len() + 1);
    data.extend_from_slice(&pq_adv_id.to_be_bytes());
    data.extend_from_slice(pq_adv_pub);
    data.push(0x01);
    kmac32(kmac, rk, "QSP5.0/ADVAUTH", &data)
}

/// Constant-time 32-byte tag comparison (no early exit on mismatch position).
fn ct_eq32(a: &[u8; 32], b: &[u8]) -> bool {
    if b.len() != 32 {
        return false;
    }
    let mut diff = 0u8;
    for i in 0..32 {
        diff |= a[i] ^ b[i];
    }
    diff == 0
}

/// §3.3.3 Root update from a PQ shared secret: `RK' = KMAC32(RK, "QSP5.0/RKPQ", pq_ss || [0x01])`.
///
/// NA-0623 (ENG-0012 Stage 2a): used by the PQ-reseed boundary send/receive so the PQ epoch secret
/// lands in the root `RK` (DOC-CAN-003 §8.5.3 step 5). This is the same derivation the base
/// handshake uses to mix in the initial PQ seed (suite2::establish `QSP5.0/RKPQ`); advancing the
/// root here is what lets the classical DH ratchet carry the PQ hardening forward permanently.
fn kdf_rk_pq(kmac: &dyn Kmac, rk: &[u8; 32], pq_ss: &[u8]) -> Result<[u8; 32], CryptoError> {
    let mut data = Vec::with_capacity(pq_ss.len() + 1);
    data.extend_from_slice(pq_ss);
    data.push(0x01);
    kmac32(kmac, rk, "QSP5.0/RKPQ", &data)
}

/// The A->B direction is the sending direction for role A and the receiving direction for role B.
fn send_is_a_to_b(role_is_a: bool) -> bool {
    role_is_a
}

/// §8.5.2 PQ send-chain reinit label after a DH boundary (matches establish.rs).
fn pq0_send_label(role_is_a: bool) -> &'static str {
    if role_is_a {
        "QSP5.0/PQ0/A->B"
    } else {
        "QSP5.0/PQ0/B->A"
    }
}

/// §8.5.2 PQ recv-chain reinit label after a DH boundary.
fn pq0_recv_label(role_is_a: bool) -> &'static str {
    if role_is_a {
        "QSP5.0/PQ0/B->A"
    } else {
        "QSP5.0/PQ0/A->B"
    }
}

fn evict_mkskipped(mut entries: Vec<MkSkippedEntry>) -> Vec<MkSkippedEntry> {
    if entries.len() <= MAX_MKSKIPPED {
        return entries;
    }
    entries.sort_by(|a, b| {
        let n_cmp = a.n.cmp(&b.n);
        if n_cmp != std::cmp::Ordering::Equal {
            return n_cmp;
        }
        a.dh_pub.cmp(&b.dh_pub)
    });
    let excess = entries.len().saturating_sub(MAX_MKSKIPPED);
    if excess > 0 {
        entries.drain(0..excess);
    }
    entries
}

#[derive(Clone)]
pub struct MkSkippedEntry {
    pub dh_pub: [u8; 32],
    pub n: u32,
    pub mk: [u8; 32],
}

#[derive(Clone)]
pub struct Suite2RecvState {
    pub session_id: [u8; 16],
    pub protocol_version: u16,
    pub suite_id: u16,
    pub dh_pub: [u8; 32],
    pub hk_r: [u8; 32],
    pub ck_ec: [u8; 32],
    pub ck_pq: [u8; 32],
    pub nr: u32,
    pub mkskipped: Vec<MkSkippedEntry>,
}

pub struct RecvOutcome {
    pub state: Suite2RecvState,
    pub ok: bool,
    pub reason: Option<&'static str>,
    pub plaintext: Option<Vec<u8>>,
    pub pn: Option<u32>,
    pub n: Option<u32>,
}

#[derive(Clone)]
pub struct Suite2BoundaryState {
    pub session_id: [u8; 16],
    pub protocol_version: u16,
    pub suite_id: u16,
    pub dh_pub: [u8; 32],
    pub hk_r: [u8; 32],
    pub rk: [u8; 32],
    pub ck_ec: [u8; 32],
    pub ck_pq_send: [u8; 32],
    pub ck_pq_recv: [u8; 32],
    pub nr: u32,
    pub role_is_a: bool,
    pub peer_max_adv_id_seen: u32,
    pub known_targets: BTreeSet<u32>,
    pub consumed_targets: BTreeSet<u32>,
    pub tombstoned_targets: BTreeSet<u32>,
}

pub struct BoundaryOutcome {
    pub state: Suite2BoundaryState,
    pub ok: bool,
    pub reason: Option<&'static str>,
    pub plaintext: Option<Vec<u8>>,
    pub pn: Option<u32>,
    pub n: Option<u32>,
}

struct ParsedPqPrefix {
    pq_target_id: u32,
    pq_ct: Vec<u8>,
}

fn parse_pq_prefix(flags: u16, pq_prefix: &[u8]) -> Result<ParsedPqPrefix, &'static str> {
    let known_flags = types::FLAG_PQ_ADV | types::FLAG_PQ_CTXT | types::FLAG_BOUNDARY;
    if (flags & !known_flags) != 0 {
        return Err("REJECT_S2_LOCAL_UNSUPPORTED");
    }
    if (flags & types::FLAG_BOUNDARY) == 0 {
        return Err("REJECT_S2_LOCAL_UNSUPPORTED");
    }
    if (flags & types::FLAG_PQ_ADV) != 0 {
        return Err("REJECT_S2_LOCAL_UNSUPPORTED");
    }
    if (flags & types::FLAG_PQ_CTXT) == 0 {
        return Err("REJECT_S2_LOCAL_UNSUPPORTED");
    }
    if pq_prefix.len() < 4 {
        return Err("REJECT_S2_PQPREFIX_PARSE");
    }
    let pq_target_id = u32::from_be_bytes([pq_prefix[0], pq_prefix[1], pq_prefix[2], pq_prefix[3]]);
    let pq_ct = pq_prefix[4..].to_vec();
    Ok(ParsedPqPrefix {
        pq_target_id,
        pq_ct,
    })
}

pub fn nonce_hdr(hash: &dyn Hash, session_id: &[u8; 16], dh_pub: &[u8; 32], n: u32) -> [u8; 12] {
    let mut m = Vec::with_capacity(15 + 16 + 32 + 4);
    m.extend_from_slice(b"QSP5.0/HDR-NONCE");
    m.extend_from_slice(session_id);
    m.extend_from_slice(dh_pub);
    m.extend_from_slice(&n.to_be_bytes());
    let h = hash.sha512(&m);
    let mut out = [0u8; 12];
    out.copy_from_slice(&h[0..12]);
    out
}

pub fn nonce_body(hash: &dyn Hash, session_id: &[u8; 16], dh_pub: &[u8; 32], n: u32) -> [u8; 12] {
    let mut m = Vec::with_capacity(16 + 16 + 32 + 4);
    m.extend_from_slice(b"QSP5.0/BODY-NONCE");
    m.extend_from_slice(session_id);
    m.extend_from_slice(dh_pub);
    m.extend_from_slice(&n.to_be_bytes());
    let h = hash.sha512(&m);
    let mut out = [0u8; 12];
    out.copy_from_slice(&h[0..12]);
    out
}

/// Derive per-message chain updates and hybrid mk.
type MkStep = ([u8; 32], [u8; 32], [u8; 32]);
pub fn derive_mk_step(
    kmac: &dyn Kmac,
    ck_ec: &[u8; 32],
    ck_pq: &[u8; 32],
) -> Result<MkStep, CryptoError> {
    if is_zero32(ck_ec) || is_zero32(ck_pq) {
        return Err(CryptoError::InvalidKey);
    }
    let ck_ec_p = kmac32(kmac, ck_ec, "QSP5.0/CK", &[0x01])?;
    let ec_mk = kmac32(kmac, ck_ec, "QSP5.0/MK", &[0x02])?;

    let ck_pq_p = kmac32(kmac, ck_pq, "QSP5.0/PQCK", &[0x01])?;
    let pq_mk = kmac32(kmac, ck_pq, "QSP5.0/PQMK", &[0x02])?;

    let mut data = Vec::with_capacity(pq_mk.len() + 1);
    data.extend_from_slice(&pq_mk);
    data.push(0x01);
    let mk = kmac32(kmac, &ec_mk, "QSP5.0/HYBRID", &data)?;

    Ok((ck_ec_p, ck_pq_p, mk))
}

pub fn recv_nonboundary_ooo(
    hash: &dyn Hash,
    kmac: &dyn Kmac,
    aead: &dyn Aead,
    st: Suite2RecvState,
    flags: u16,
    hdr_ct: &[u8],
    body_ct: &[u8],
) -> RecvOutcome {
    if flags != 0 {
        return RecvOutcome {
            state: st,
            ok: false,
            reason: Some("REJECT_S2_LOCAL_UNSUPPORTED"),
            plaintext: None,
            pn: None,
            n: None,
        };
    }
    if hdr_ct.len() != HDR_CT_LEN {
        return RecvOutcome {
            state: st,
            ok: false,
            reason: Some("REJECT_S2_HDR_AUTH_FAIL"),
            plaintext: None,
            pn: None,
            n: None,
        };
    }
    if body_ct.len() < BODY_CT_MIN {
        return RecvOutcome {
            state: st,
            ok: false,
            reason: Some("REJECT_S2_BODY_AUTH_FAIL"),
            plaintext: None,
            pn: None,
            n: None,
        };
    }
    if is_zero32(&st.ck_ec) || is_zero32(&st.ck_pq) {
        return RecvOutcome {
            state: st,
            ok: false,
            reason: Some(REJECT_S2_CHAINKEY_UNSET),
            plaintext: None,
            pn: None,
            n: None,
        };
    }

    let pq_bind = binding::pq_bind_sha512_32(hash, flags, &[]);
    let ad_hdr = binding::ad_hdr(
        &st.session_id,
        st.protocol_version,
        st.suite_id,
        &st.dh_pub,
        flags,
        &pq_bind,
    );
    let ad_body = binding::ad_body(&st.session_id, st.protocol_version, st.suite_id, &pq_bind);

    let mut header_pt: Option<[u8; 8]> = None;
    let mut header_n: u32 = 0;
    let mut header_pn: u32 = 0;
    let mut attempts: usize = 0;
    macro_rules! try_candidate {
        ($cand:expr) => {{
            if attempts >= MAX_HEADER_ATTEMPTS || header_pt.is_some() {
                false
            } else {
                let cand = $cand;
                attempts = attempts.saturating_add(1);
                let nonce = nonce_hdr(hash, &st.session_id, &st.dh_pub, cand);
                #[cfg(test)]
                S2_HDR_TRY_COUNT_NONBOUNDARY.with(|c| c.set(c.get().saturating_add(1)));
                if let Ok(pt) = aead.open(&st.hk_r, &nonce, &ad_hdr, hdr_ct) {
                    if pt.len() == 8 {
                        let pn = u32::from_be_bytes([pt[0], pt[1], pt[2], pt[3]]);
                        let n_val = u32::from_be_bytes([pt[4], pt[5], pt[6], pt[7]]);
                        if n_val == cand {
                            header_pt =
                                Some([pt[0], pt[1], pt[2], pt[3], pt[4], pt[5], pt[6], pt[7]]);
                            header_n = n_val;
                            header_pn = pn;
                            true
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
        }};
    }

    let mut seen: HashSet<u32> = HashSet::with_capacity(MAX_HEADER_ATTEMPTS);
    macro_rules! try_unique {
        ($cand:expr) => {{
            if attempts >= MAX_HEADER_ATTEMPTS || header_pt.is_some() {
                true
            } else {
                let cand = $cand;
                if seen.insert(cand) {
                    let _ = try_candidate!(cand);
                }
                attempts >= MAX_HEADER_ATTEMPTS || header_pt.is_some()
            }
        }};
    }

    // Fixed-priority probes: in-order, replay-nearby, and OOO bounds edges.
    let mut seed_candidates = vec![
        st.nr,
        st.nr.saturating_add(1),
        st.nr.saturating_add(MAX_SKIP),
        st.nr.saturating_add(MAX_SKIP + 1),
    ];
    if MAX_SKIP > 0 {
        seed_candidates.push(st.nr.saturating_add(MAX_SKIP - 1));
    }
    if st.nr > 0 {
        seed_candidates.push(st.nr.saturating_sub(1));
    }
    for cand in seed_candidates {
        if try_unique!(cand) {
            break;
        }
    }

    // Prefer most-recent skipped keys first for OOO recovery under capped work.
    if header_pt.is_none() && attempts < MAX_HEADER_ATTEMPTS {
        for entry in st.mkskipped.iter().rev() {
            if try_unique!(entry.n) {
                break;
            }
        }
    }

    // Backward window next so replay cases inside window normalize correctly.
    if header_pt.is_none() && attempts < MAX_HEADER_ATTEMPTS {
        let mut back = st.nr;
        let back_start = st.nr.saturating_sub(MAX_SKIP);
        while back > back_start {
            back = back.saturating_sub(1);
            if try_unique!(back) {
                break;
            }
        }
    }

    // Finally probe forward window.
    if header_pt.is_none() && attempts < MAX_HEADER_ATTEMPTS {
        let max_forward = st.nr.saturating_add(MAX_SKIP);
        let mut fwd = st.nr.saturating_add(2);
        while fwd < max_forward {
            if try_unique!(fwd) {
                break;
            }
            match fwd.checked_add(1) {
                Some(next) => fwd = next,
                None => break,
            }
        }
    }

    if header_pt.is_none() {
        return RecvOutcome {
            state: st,
            ok: false,
            reason: Some("REJECT_S2_HDR_AUTH_FAIL"),
            plaintext: None,
            pn: None,
            n: None,
        };
    }

    // Check for MKSKIPPED hit
    if let Some(pos) = st
        .mkskipped
        .iter()
        .position(|e| e.dh_pub == st.dh_pub && e.n == header_n)
    {
        let mk = st.mkskipped[pos].mk;
        let nonce = nonce_body(hash, &st.session_id, &st.dh_pub, header_n);
        match aead.open(&mk, &nonce, &ad_body, body_ct) {
            Ok(pt) => {
                let mut new_state = st.clone();
                new_state.mkskipped.remove(pos);
                return RecvOutcome {
                    state: new_state,
                    ok: true,
                    reason: None,
                    plaintext: Some(pt),
                    pn: Some(header_pn),
                    n: Some(header_n),
                };
            }
            Err(_) => {
                return RecvOutcome {
                    state: st,
                    ok: false,
                    reason: Some("REJECT_S2_BODY_AUTH_FAIL"),
                    plaintext: None,
                    pn: Some(header_pn),
                    n: Some(header_n),
                };
            }
        }
    }

    if header_n < st.nr {
        return RecvOutcome {
            state: st,
            ok: false,
            reason: Some("REJECT_S2_REPLAY"),
            plaintext: None,
            pn: Some(header_pn),
            n: Some(header_n),
        };
    }
    if header_n - st.nr > MAX_SKIP {
        return RecvOutcome {
            state: st,
            ok: false,
            reason: Some("REJECT_S2_OOO_BOUNDS"),
            plaintext: None,
            pn: Some(header_pn),
            n: Some(header_n),
        };
    }

    // Stage derivations from Nr..=N
    let mut ck_ec = st.ck_ec;
    let mut ck_pq = st.ck_pq;
    let mut staged: Vec<MkSkippedEntry> = Vec::new();
    let mut mk_n: Option<[u8; 32]> = None;

    if is_zero32(&ck_ec) || is_zero32(&ck_pq) {
        return RecvOutcome {
            state: st,
            ok: false,
            reason: Some(REJECT_S2_CHAINKEY_UNSET),
            plaintext: None,
            pn: Some(header_pn),
            n: Some(header_n),
        };
    }

    for i in st.nr..=header_n {
        let (ck_ec_p, ck_pq_p, mk) = match derive_mk_step(kmac, &ck_ec, &ck_pq) {
            Ok(v) => v,
            Err(_) => {
                return RecvOutcome {
                    state: st,
                    ok: false,
                    reason: Some("REJECT_S2_BODY_AUTH_FAIL"),
                    plaintext: None,
                    pn: Some(header_pn),
                    n: Some(header_n),
                };
            }
        };
        if i < header_n {
            staged.push(MkSkippedEntry {
                dh_pub: st.dh_pub,
                n: i,
                mk,
            });
        } else {
            mk_n = Some(mk);
        }
        ck_ec = ck_ec_p;
        ck_pq = ck_pq_p;
    }

    let mk = match mk_n {
        Some(v) => v,
        None => {
            return RecvOutcome {
                state: st,
                ok: false,
                reason: Some("REJECT_S2_BODY_AUTH_FAIL"),
                plaintext: None,
                pn: Some(header_pn),
                n: Some(header_n),
            }
        }
    };
    let nonce = nonce_body(hash, &st.session_id, &st.dh_pub, header_n);
    match aead.open(&mk, &nonce, &ad_body, body_ct) {
        Ok(pt) => {
            // NA-0618 (ENG-0013): hard-stop before nr would pass u32::MAX. Fail closed with
            // no state mutation (state: st is returned unchanged). A well-behaved sender
            // never originates a message at this counter (send_wire guards symmetrically).
            let nr_next = match checked_counter_inc(header_n) {
                Ok(v) => v,
                Err(reason) => {
                    return RecvOutcome {
                        state: st,
                        ok: false,
                        reason: Some(reason),
                        plaintext: None,
                        pn: None,
                        n: None,
                    };
                }
            };
            let mut new_state = st.clone();
            new_state.ck_ec = ck_ec;
            new_state.ck_pq = ck_pq;
            new_state.nr = nr_next;
            new_state.mkskipped.extend(staged);
            new_state.mkskipped = evict_mkskipped(new_state.mkskipped);
            RecvOutcome {
                state: new_state,
                ok: true,
                reason: None,
                plaintext: Some(pt),
                pn: Some(header_pn),
                n: Some(header_n),
            }
        }
        Err(_) => RecvOutcome {
            state: st,
            ok: false,
            reason: Some("REJECT_S2_BODY_AUTH_FAIL"),
            plaintext: None,
            pn: Some(header_pn),
            n: Some(header_n),
        },
    }
}

#[allow(clippy::too_many_arguments)]
pub fn recv_boundary_in_order(
    hash: &dyn Hash,
    kmac: &dyn Kmac,
    aead: &dyn Aead,
    st: Suite2BoundaryState,
    flags: u16,
    pq_prefix: &[u8],
    hdr_ct: &[u8],
    body_ct: &[u8],
    pq_epoch_ss: &[u8],
    peer_adv_id: u32,
) -> BoundaryOutcome {
    if flags == 0 {
        return BoundaryOutcome {
            state: st,
            ok: false,
            reason: Some("REJECT_S2_LOCAL_UNSUPPORTED"),
            plaintext: None,
            pn: None,
            n: None,
        };
    }

    let parsed = match parse_pq_prefix(flags, pq_prefix) {
        Ok(v) => v,
        Err(code) => {
            return BoundaryOutcome {
                state: st,
                ok: false,
                reason: Some(code),
                plaintext: None,
                pn: None,
                n: None,
            };
        }
    };

    if hdr_ct.len() != HDR_CT_LEN {
        return BoundaryOutcome {
            state: st,
            ok: false,
            reason: Some("REJECT_S2_HDR_AUTH_FAIL"),
            plaintext: None,
            pn: None,
            n: None,
        };
    }
    if body_ct.len() < BODY_CT_MIN {
        return BoundaryOutcome {
            state: st,
            ok: false,
            reason: Some("REJECT_S2_BODY_AUTH_FAIL"),
            plaintext: None,
            pn: None,
            n: None,
        };
    }

    let pq_bind = binding::pq_bind_sha512_32(hash, flags, pq_prefix);
    let ad_hdr = binding::ad_hdr(
        &st.session_id,
        st.protocol_version,
        st.suite_id,
        &st.dh_pub,
        flags,
        &pq_bind,
    );
    let ad_body = binding::ad_body(&st.session_id, st.protocol_version, st.suite_id, &pq_bind);

    // NA-0625 (ENG-0023): §8.5.1 + §8.5.3 step 1 — the PQ-CTXT boundary header MUST decrypt
    // under the receiver's CURRENT `NHK_r`, derived on demand from the PRE-reseed root (the
    // `recv_dh_boundary` pattern; no stored NHK field). NHK-ONLY open: a pre-NA-0625 (HK-sealed)
    // boundary frame fails generically with REJECT_S2_HDR_AUTH_FAIL — §8.5.1's "decrypts under
    // any other candidate key MUST reject" is satisfied by never trying other keys.
    let a2b_recv = !send_is_a_to_b(st.role_is_a);
    let nhk_r = match header_key(kmac, &st.rk, a2b_recv, true) {
        Ok(v) => v,
        Err(_) => {
            return BoundaryOutcome {
                state: st,
                ok: false,
                reason: Some("REJECT_S2_LOCAL_UNSUPPORTED"),
                plaintext: None,
                pn: None,
                n: None,
            };
        }
    };

    let mut header_pt: Option<[u8; 8]> = None;
    let mut n: u32 = 0;
    let candidates = [st.nr, st.nr.saturating_add(1)];
    for cand in candidates {
        let nonce_hdr = nonce_hdr(hash, &st.session_id, &st.dh_pub, cand);
        #[cfg(test)]
        S2_HDR_TRY_COUNT_BOUNDARY.with(|c| c.set(c.get().saturating_add(1)));
        if let Ok(pt) = aead.open(&nhk_r, &nonce_hdr, &ad_hdr, hdr_ct) {
            if pt.len() == 8 {
                let pn = u32::from_be_bytes([pt[0], pt[1], pt[2], pt[3]]);
                let n_val = u32::from_be_bytes([pt[4], pt[5], pt[6], pt[7]]);
                if n_val == cand {
                    header_pt = Some([pt[0], pt[1], pt[2], pt[3], pt[4], pt[5], pt[6], pt[7]]);
                    n = n_val;
                    let _ = pn;
                    break;
                }
            }
        }
    }
    let header_pt = match header_pt {
        Some(v) => v,
        None => {
            return BoundaryOutcome {
                state: st,
                ok: false,
                reason: Some("REJECT_S2_HDR_AUTH_FAIL"),
                plaintext: None,
                pn: None,
                n: None,
            };
        }
    };
    let header_pn = u32::from_be_bytes([header_pt[0], header_pt[1], header_pt[2], header_pt[3]]);
    if n != st.nr {
        return BoundaryOutcome {
            state: st,
            ok: false,
            reason: Some(
                "REJECT_S2_BOUNDARY_NOT_IN_ORDER; reason_code=REJECT_S2_BOUNDARY_NOT_IN_ORDER",
            ),
            plaintext: None,
            pn: Some(header_pn),
            n: Some(n),
        };
    }

    if is_zero32(&st.ck_ec) || is_zero32(&st.ck_pq_recv) {
        return BoundaryOutcome {
            state: st,
            ok: false,
            reason: Some(REJECT_S2_CHAINKEY_UNSET),
            plaintext: None,
            pn: Some(header_pn),
            n: Some(n),
        };
    }

    let (ck_ec_p, _ck_pq_p, mk) = match derive_mk_step(kmac, &st.ck_ec, &st.ck_pq_recv) {
        Ok(v) => v,
        Err(_) => {
            return BoundaryOutcome {
                state: st,
                ok: false,
                reason: Some("REJECT_S2_BODY_AUTH_FAIL"),
                plaintext: None,
                pn: Some(header_pn),
                n: Some(n),
            };
        }
    };

    let nonce_body = nonce_body(hash, &st.session_id, &st.dh_pub, n);
    let body_pt = match aead.open(&mk, &nonce_body, &ad_body, body_ct) {
        Ok(pt) => pt,
        Err(_) => {
            return BoundaryOutcome {
                state: st,
                ok: false,
                reason: Some("REJECT_S2_BODY_AUTH_FAIL"),
                plaintext: None,
                pn: Some(header_pn),
                n: Some(n),
            };
        }
    };

    let apply = match scka::apply_pq_reseed(
        hash,
        kmac,
        st.role_is_a,
        &st.rk,
        &parsed.pq_ct,
        pq_epoch_ss,
        peer_adv_id,
        st.peer_max_adv_id_seen,
        &st.known_targets,
        &st.consumed_targets,
        &st.tombstoned_targets,
        parsed.pq_target_id,
        true,
        &st.ck_pq_send,
        &st.ck_pq_recv,
    ) {
        Ok(v) => v,
        Err(scka::Suite2Reject::Code(code)) => {
            return BoundaryOutcome {
                state: st,
                ok: false,
                reason: Some(code),
                plaintext: None,
                pn: Some(header_pn),
                n: Some(n),
            };
        }
    };

    // NA-0618 (ENG-0013): hard-stop before nr would pass u32::MAX. Fail closed with no state
    // mutation (state: st is returned unchanged).
    let nr_next = match checked_counter_inc(n) {
        Ok(v) => v,
        Err(reason) => {
            return BoundaryOutcome {
                state: st,
                ok: false,
                reason: Some(reason),
                plaintext: None,
                pn: Some(header_pn),
                n: Some(n),
            };
        }
    };
    // NA-0623 (ENG-0012 Stage 2a, D560 AMENDMENT): advance the root with the PQ epoch secret and
    // recompute the receive header key (DOC-CAN-003 §8.5.3 steps 5+7, §3.3.6 normative ordering).
    // apply_pq_reseed above absorbs `pq_epoch_ss` into the directional PQ chains from `RK_old`
    // ONLY; it never advances the root. Without this step, a subsequent DH ratchet reinitialises
    // `CK_pq` from the un-hardened `RK` (§8.5.2 step 6) and wipes the post-quantum protection.
    // Advancing `RK` here (mirrored in the SCKA sender) lands the PQ secret in the root so the DH
    // ratchet carries it forward permanently. Fail-closed with no state mutation on KDF error.
    let new_rk = match kdf_rk_pq(kmac, &st.rk, pq_epoch_ss) {
        Ok(v) => v,
        Err(_) => {
            return BoundaryOutcome {
                state: st,
                ok: false,
                reason: Some("REJECT_S2_LOCAL_UNSUPPORTED"),
                plaintext: None,
                pn: Some(header_pn),
                n: Some(n),
            };
        }
    };
    let new_hk_r = match header_key(kmac, &new_rk, a2b_recv, false) {
        Ok(v) => v,
        Err(_) => {
            return BoundaryOutcome {
                state: st,
                ok: false,
                reason: Some("REJECT_S2_LOCAL_UNSUPPORTED"),
                plaintext: None,
                pn: Some(header_pn),
                n: Some(n),
            };
        }
    };

    let mut new_state = st.clone();
    new_state.ck_ec = ck_ec_p;
    new_state.ck_pq_send = apply.ck_pq_send_after;
    new_state.ck_pq_recv = apply.ck_pq_recv_after;
    new_state.peer_max_adv_id_seen = apply.peer_max_adv_id_seen_after;
    new_state.consumed_targets = apply.consumed_targets_after;
    new_state.tombstoned_targets = apply.tombstoned_targets_after;
    new_state.nr = nr_next;
    new_state.rk = new_rk;
    new_state.hk_r = new_hk_r;

    BoundaryOutcome {
        state: new_state,
        ok: true,
        reason: None,
        plaintext: Some(body_pt),
        pn: Some(header_pn),
        n: Some(n),
    }
}

#[derive(Clone)]
pub struct Suite2RecvWireState {
    pub session_id: [u8; 16],
    pub protocol_version: u16,
    pub suite_id: u16,
    pub dh_pub: [u8; 32],
    pub hk_r: [u8; 32],
    pub rk: [u8; 32],
    pub ck_ec: [u8; 32],
    pub ck_pq_send: [u8; 32],
    pub ck_pq_recv: [u8; 32],
    pub nr: u32,
    pub role_is_a: bool,
    pub peer_max_adv_id_seen: u32,
    pub known_targets: BTreeSet<u32>,
    pub consumed_targets: BTreeSet<u32>,
    pub tombstoned_targets: BTreeSet<u32>,
    pub mkskipped: Vec<MkSkippedEntry>,
}

pub struct RecvWireOutcome {
    pub state: Suite2RecvWireState,
    pub plaintext: Vec<u8>,
    pub flags: u16,
    pub pn: u32,
    pub n: u32,
}

#[derive(Clone)]
pub struct Suite2SendState {
    pub session_id: [u8; 16],
    pub protocol_version: u16,
    pub suite_id: u16,
    pub dh_pub: [u8; 32],
    pub hk_s: [u8; 32],
    pub ck_ec: [u8; 32],
    pub ck_pq: [u8; 32],
    pub ns: u32,
    pub pn: u32,
}

/// NA-0620 (ENG-0012 Stage 1a): session-level DH-ratchet state, carried and persisted so the
/// send-side DH ratchet (Stage 1b, DOC-G5-008 §5) has the material it needs — the local X25519
/// keypair (`dhs_priv`/`dhs_pub`), the current peer DH public (`dhr`), and the live root key
/// (`rk`). This is PLUMBING ONLY: no message-path code reads it in Stage 1a. `dhs_priv` is
/// populated by the client after establishment (`set_dh_self_priv`); establishment itself
/// leaves it zero for callers that do not ratchet.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Suite2DhRatchetState {
    pub dhs_priv: [u8; 32],
    pub dhs_pub: [u8; 32],
    pub dhr: [u8; 32],
    pub rk: [u8; 32],
}

pub struct SendWireOutcome {
    pub state: Suite2SendState,
    pub wire: Vec<u8>,
    pub flags: u16,
    pub pn: u32,
    pub n: u32,
}

pub fn send_wire(
    hash: &dyn Hash,
    kmac: &dyn Kmac,
    aead: &dyn Aead,
    st: Suite2SendState,
    flags: u16,
    plaintext: &[u8],
) -> Result<SendWireOutcome, &'static str> {
    if flags != 0 {
        return Err("REJECT_S2_LOCAL_UNSUPPORTED");
    }
    if is_zero32(&st.ck_ec) || is_zero32(&st.ck_pq) {
        return Err(REJECT_S2_CHAINKEY_UNSET);
    }
    // NA-0618 (ENG-0013): hard-stop before the send counter would pass u32::MAX (fail closed
    // before deriving any key material). Advancing past this would freeze `ns` and (with
    // static header keys) repeat header ciphertext.
    let ns_next = checked_counter_inc(st.ns)?;
    let (ck_ec_p, ck_pq_p, mk) =
        derive_mk_step(kmac, &st.ck_ec, &st.ck_pq).map_err(|_| "REJECT_S2_LOCAL_UNSUPPORTED")?;

    let pq_bind = binding::pq_bind_sha512_32(hash, flags, &[]);
    let ad_hdr = binding::ad_hdr(
        &st.session_id,
        st.protocol_version,
        st.suite_id,
        &st.dh_pub,
        flags,
        &pq_bind,
    );
    let ad_body = binding::ad_body(&st.session_id, st.protocol_version, st.suite_id, &pq_bind);

    let hdr_pt = {
        let mut v = Vec::with_capacity(8);
        v.extend_from_slice(&st.pn.to_be_bytes());
        v.extend_from_slice(&st.ns.to_be_bytes());
        v
    };
    let hdr_ct = aead.seal(
        &st.hk_s,
        &nonce_hdr(hash, &st.session_id, &st.dh_pub, st.ns),
        &ad_hdr,
        &hdr_pt,
    );
    let body_ct = aead.seal(
        &mk,
        &nonce_body(hash, &st.session_id, &st.dh_pub, st.ns),
        &ad_body,
        plaintext,
    );
    if hdr_ct.is_empty() || body_ct.is_empty() {
        return Err("REJECT_S2_LOCAL_AEAD_FAIL");
    }

    let mut header = Vec::with_capacity(32 + 2 + hdr_ct.len());
    header.extend_from_slice(&st.dh_pub);
    header.extend_from_slice(&flags.to_be_bytes());
    header.extend_from_slice(&hdr_ct);

    let mut wire = Vec::with_capacity(10 + header.len() + body_ct.len());
    wire.extend_from_slice(&st.protocol_version.to_be_bytes());
    wire.extend_from_slice(&st.suite_id.to_be_bytes());
    wire.push(0x02);
    wire.push(0x00);
    wire.extend_from_slice(&(header.len() as u16).to_be_bytes());
    wire.extend_from_slice(&(body_ct.len() as u16).to_be_bytes());
    wire.extend_from_slice(&header);
    wire.extend_from_slice(&body_ct);

    let mut new_state = st.clone();
    new_state.ck_ec = ck_ec_p;
    new_state.ck_pq = ck_pq_p;
    new_state.ns = ns_next;

    Ok(SendWireOutcome {
        state: new_state,
        wire,
        flags,
        pn: st.pn,
        n: st.ns,
    })
}
pub fn recv_wire(
    hash: &dyn Hash,
    kmac: &dyn Kmac,
    aead: &dyn Aead,
    st: Suite2RecvWireState,
    wire: &[u8],
    pq_epoch_ss: Option<&[u8]>,
    peer_adv_id: Option<u32>,
) -> Result<RecvWireOutcome, &'static str> {
    let (_pv, _sid, _msg_type, parsed) = parse::decode_suite2_wire(wire)?;
    let flags = parsed.flags;

    if flags == 0 {
        let recv_state = Suite2RecvState {
            session_id: st.session_id,
            protocol_version: st.protocol_version,
            suite_id: st.suite_id,
            dh_pub: parsed.dh_pub,
            hk_r: st.hk_r,
            ck_ec: st.ck_ec,
            ck_pq: st.ck_pq_recv,
            nr: st.nr,
            mkskipped: st.mkskipped.clone(),
        };
        let out = recv_nonboundary_ooo(
            hash,
            kmac,
            aead,
            recv_state,
            flags,
            &parsed.hdr_ct,
            &parsed.body_ct,
        );
        if !out.ok {
            return Err(out.reason.unwrap_or("REJECT_S2_HDR_AUTH_FAIL"));
        }
        let mut new_state = st.clone();
        new_state.ck_ec = out.state.ck_ec;
        new_state.ck_pq_recv = out.state.ck_pq;
        new_state.nr = out.state.nr;
        new_state.mkskipped = out.state.mkskipped;
        return Ok(RecvWireOutcome {
            state: new_state,
            plaintext: out.plaintext.unwrap_or_default(),
            flags,
            pn: out.pn.unwrap_or(0),
            n: out.n.unwrap_or(0),
        });
    }

    if (flags & types::FLAG_PQ_ADV) != 0 {
        // NA-0625 (ENG-0023): authenticated ADV receive (was REJECT_S2_LOCAL_UNSUPPORTED — the
        // named gap (2)). recv_pq_adv enforces flags == BOUNDARY|PQ_ADV exactly, so a combined
        // ADV+CTXT frame still rejects as unsupported (ENG-0026 territory). On this path the
        // `peer_adv_id` parameter carries the caller-owned peer-ADV WATERMARK (qsc: the SCKA
        // store's `peer_adv_max_seen`; None => 0, a fresh session with no tracked peer ADV).
        let pq_adv_id = parsed.pq_adv_id.ok_or("REJECT_S2_PQPREFIX_PARSE")?;
        let pq_adv_pub = parsed
            .pq_adv_pub
            .as_deref()
            .ok_or("REJECT_S2_PQPREFIX_PARSE")?;
        let out = recv_pq_adv(
            hash,
            kmac,
            aead,
            st,
            flags,
            &parsed.pq_prefix,
            pq_adv_id,
            pq_adv_pub,
            peer_adv_id.unwrap_or(0),
            &parsed.dh_pub,
            &parsed.hdr_ct,
            &parsed.body_ct,
        );
        if !out.ok {
            return Err(out.reason.unwrap_or("REJECT_S2_HDR_AUTH_FAIL"));
        }
        return Ok(RecvWireOutcome {
            state: out.state,
            plaintext: out.plaintext.unwrap_or_default(),
            flags,
            pn: out.pn.unwrap_or(0),
            n: out.n.unwrap_or(0),
        });
    }

    let pq_epoch_ss = pq_epoch_ss.ok_or("REJECT_S2_LOCAL_UNSUPPORTED")?;
    let peer_adv_id = peer_adv_id.unwrap_or(st.peer_max_adv_id_seen.saturating_add(1));

    let boundary_state = Suite2BoundaryState {
        session_id: st.session_id,
        protocol_version: st.protocol_version,
        suite_id: st.suite_id,
        dh_pub: parsed.dh_pub,
        hk_r: st.hk_r,
        rk: st.rk,
        ck_ec: st.ck_ec,
        ck_pq_send: st.ck_pq_send,
        ck_pq_recv: st.ck_pq_recv,
        nr: st.nr,
        role_is_a: st.role_is_a,
        peer_max_adv_id_seen: st.peer_max_adv_id_seen,
        known_targets: st.known_targets.clone(),
        consumed_targets: st.consumed_targets.clone(),
        tombstoned_targets: st.tombstoned_targets.clone(),
    };
    let out = recv_boundary_in_order(
        hash,
        kmac,
        aead,
        boundary_state,
        flags,
        &parsed.pq_prefix,
        &parsed.hdr_ct,
        &parsed.body_ct,
        pq_epoch_ss,
        peer_adv_id,
    );
    if !out.ok {
        return Err(out.reason.unwrap_or("REJECT_S2_HDR_AUTH_FAIL"));
    }
    let mut new_state = st.clone();
    new_state.ck_ec = out.state.ck_ec;
    new_state.ck_pq_send = out.state.ck_pq_send;
    new_state.ck_pq_recv = out.state.ck_pq_recv;
    new_state.peer_max_adv_id_seen = out.state.peer_max_adv_id_seen;
    new_state.consumed_targets = out.state.consumed_targets;
    new_state.tombstoned_targets = out.state.tombstoned_targets;
    new_state.nr = out.state.nr;
    // NA-0623 (ENG-0012 Stage 2a): the PQ reseed advanced the root and recomputed the receive
    // header key (recv_boundary_in_order); carry both forward so the next (post-reseed) message
    // decrypts under the new key schedule (DOC-CAN-003 §3.4/§8.5.3 step 7). Dropping them would
    // leave the receiver on the stale pre-reseed header key.
    new_state.rk = out.state.rk;
    new_state.hk_r = out.state.hk_r;
    Ok(RecvWireOutcome {
        state: new_state,
        plaintext: out.plaintext.unwrap_or_default(),
        flags,
        pn: out.pn.unwrap_or(0),
        n: out.n.unwrap_or(0),
    })
}

// ======================= NA-0621 (ENG-0012 Stage 1b-i): DH ratchet =======================
// Classical X25519 DH ratchet (send + receive) for Suite-2, operating at the session level
// (Suite2SessionState: the Stage-1a `dh` field holds DHs/DHr/RK; per-direction chains stay in
// send/recv). NHK header keys are derived on demand from RK (no new stored field, no snapshot
// change). This uses the DH_pub already carried on every ratchet message (DOC-CAN-003 §4.3), so
// there is NO wire-format change; the non-boundary message path is untouched, and the PQ-reseed
// path (apply_pq_reseed) is untouched. The qsc trigger + static-rk removal are Stage 1b-ii.

pub struct SendBoundaryOutcome {
    pub state: crate::suite2::state::Suite2SessionState,
    pub wire: Vec<u8>,
}

pub struct RecvDhBoundaryOutcome {
    pub state: crate::suite2::state::Suite2SessionState,
    pub plaintext: Vec<u8>,
    pub ok: bool,
    pub reason: Option<&'static str>,
}

/// Frame a header + body into the Suite-2 wire envelope (mirrors send_wire's framing exactly).
fn frame_suite2_wire(
    protocol_version: u16,
    suite_id: u16,
    dh_pub: &[u8; 32],
    flags: u16,
    hdr_ct: &[u8],
    body_ct: &[u8],
) -> Vec<u8> {
    let mut header = Vec::with_capacity(32 + 2 + hdr_ct.len());
    header.extend_from_slice(dh_pub);
    header.extend_from_slice(&flags.to_be_bytes());
    header.extend_from_slice(hdr_ct);
    let mut wire = Vec::with_capacity(10 + header.len() + body_ct.len());
    wire.extend_from_slice(&protocol_version.to_be_bytes());
    wire.extend_from_slice(&suite_id.to_be_bytes());
    wire.push(0x02);
    wire.push(0x00);
    wire.extend_from_slice(&(header.len() as u16).to_be_bytes());
    wire.extend_from_slice(&(body_ct.len() as u16).to_be_bytes());
    wire.extend_from_slice(&header);
    wire.extend_from_slice(body_ct);
    wire
}

/// §8.5.2 DH-ratchet SEND (boundary without PQ). Generates a fresh X25519 keypair, advances the
/// root via KDF_RK_DH, reinitialises the send chains, recomputes HK_s, and emits a FLAG_BOUNDARY
/// message whose header is encrypted under the pre-boundary NHK_s (§8.5.1). The receive chain is
/// untouched (it advances only on the peer's boundary). Fail-closed on unset chain keys or a zero
/// DH key.
pub fn send_boundary(
    hash: &dyn Hash,
    kmac: &dyn Kmac,
    aead: &dyn Aead,
    dh: &dyn X25519Dh,
    mut st: crate::suite2::state::Suite2SessionState,
    plaintext: &[u8],
) -> Result<SendBoundaryOutcome, &'static str> {
    // A DH boundary CREATES a fresh send chain from KDF_RK_DH; it does not consume the prior
    // send chain (so the responder, whose send chain is zero until its first ratchet, can send).
    // It does require a live root key and a known peer DH public key.
    if is_zero32(&st.dh.dhr) || is_zero32(&st.dh.rk) {
        return Err("REJECT_S2_LOCAL_UNSUPPORTED");
    }
    let role_is_a = st.recv.role_is_a;
    let a2b = send_is_a_to_b(role_is_a);

    // Boundary header key = pre-boundary NHK_s (§8.5.1 anti-spoof).
    let boundary_hk =
        header_key(kmac, &st.dh.rk, a2b, true).map_err(|_| "REJECT_S2_LOCAL_UNSUPPORTED")?;

    // Fresh DH keypair; advance the root; reinit send chains; recompute HK_s.
    let (new_priv, new_pub) = dh.keypair();
    let dh_out = dh.dh(&new_priv, &X25519Pub(st.dh.dhr));
    let (rk1, ck_ec0) =
        kdf_rk_dh(kmac, &st.dh.rk, &dh_out).map_err(|_| "REJECT_S2_LOCAL_UNSUPPORTED")?;
    let ck_pq0 = kmac32(kmac, &rk1, pq0_send_label(role_is_a), &[0x01])
        .map_err(|_| "REJECT_S2_LOCAL_UNSUPPORTED")?;
    let hk_s_new = header_key(kmac, &rk1, a2b, false).map_err(|_| "REJECT_S2_LOCAL_UNSUPPORTED")?;

    // The boundary message is message n=0 of the new send epoch.
    let (ck_ec_p, ck_pq_p, mk) =
        derive_mk_step(kmac, &ck_ec0, &ck_pq0).map_err(|_| "REJECT_S2_LOCAL_UNSUPPORTED")?;

    let flags = types::FLAG_BOUNDARY;
    let pq_bind = binding::pq_bind_sha512_32(hash, flags, &[]);
    let ad_hdr = binding::ad_hdr(
        &st.send.session_id,
        st.send.protocol_version,
        st.send.suite_id,
        &new_pub.0,
        flags,
        &pq_bind,
    );
    let ad_body = binding::ad_body(
        &st.send.session_id,
        st.send.protocol_version,
        st.send.suite_id,
        &pq_bind,
    );
    let pn_new = st.send.ns;
    let n0: u32 = 0;
    let hdr_pt = {
        let mut v = Vec::with_capacity(8);
        v.extend_from_slice(&pn_new.to_be_bytes());
        v.extend_from_slice(&n0.to_be_bytes());
        v
    };
    let hdr_ct = aead.seal(
        &boundary_hk,
        &nonce_hdr(hash, &st.send.session_id, &new_pub.0, n0),
        &ad_hdr,
        &hdr_pt,
    );
    let body_ct = aead.seal(
        &mk,
        &nonce_body(hash, &st.send.session_id, &new_pub.0, n0),
        &ad_body,
        plaintext,
    );
    if hdr_ct.is_empty() || body_ct.is_empty() {
        return Err("REJECT_S2_LOCAL_AEAD_FAIL");
    }
    let wire = frame_suite2_wire(
        st.send.protocol_version,
        st.send.suite_id,
        &new_pub.0,
        flags,
        &hdr_ct,
        &body_ct,
    );

    // Commit send + DH state (receive chain untouched).
    st.dh.rk = rk1;
    st.dh.dhs_priv = new_priv.0;
    st.dh.dhs_pub = new_pub.0;
    st.send.dh_pub = new_pub.0;
    st.send.hk_s = hk_s_new;
    st.send.ck_ec = ck_ec_p;
    st.send.ck_pq = ck_pq_p;
    st.send.pn = pn_new;
    st.send.ns = 1;
    Ok(SendBoundaryOutcome { state: st, wire })
}

/// §8.5.2 DH-ratchet RECEIVE (boundary without PQ) + §8.5.1 anti-spoof. The boundary header MUST
/// decrypt under the receiver's CURRENT NHK_r; then the root advances with
/// `dh_out = X25519(DHs_priv, msg.DH_pub)`, the receive chains reinitialise, HK_r is recomputed,
/// DHr updates, Nr := 0, and the body decrypts under the new epoch's first message key. State is
/// committed only on full success (no mutation on reject). The send chain is untouched.
pub fn recv_dh_boundary(
    hash: &dyn Hash,
    kmac: &dyn Kmac,
    aead: &dyn Aead,
    dh: &dyn X25519Dh,
    st: crate::suite2::state::Suite2SessionState,
    wire: &[u8],
) -> RecvDhBoundaryOutcome {
    macro_rules! reject {
        ($st:expr, $reason:expr) => {
            return RecvDhBoundaryOutcome {
                state: $st,
                plaintext: Vec::new(),
                ok: false,
                reason: Some($reason),
            }
        };
    }

    let (_pv, _sid, _mt, parsed) = match parse::decode_suite2_wire(wire) {
        Ok(v) => v,
        Err(code) => reject!(st, code),
    };
    let flags = parsed.flags;
    if (flags & types::FLAG_BOUNDARY) == 0
        || (flags & types::FLAG_PQ_CTXT) != 0
        || (flags & types::FLAG_PQ_ADV) != 0
    {
        reject!(st, "REJECT_S2_LOCAL_UNSUPPORTED");
    }
    if parsed.hdr_ct.len() != HDR_CT_LEN || parsed.body_ct.len() < BODY_CT_MIN {
        reject!(st, "REJECT_S2_HDR_AUTH_FAIL");
    }
    if is_zero32(&st.dh.rk) || is_zero32(&st.dh.dhs_priv) {
        reject!(st, "REJECT_S2_LOCAL_UNSUPPORTED");
    }
    // A boundary MUST advance the peer DH key, and it must be non-zero.
    if is_zero32(&parsed.dh_pub) {
        reject!(st, "REJECT_S2_HDR_AUTH_FAIL");
    }
    if parsed.dh_pub == st.dh.dhr {
        reject!(
            st,
            "REJECT_S2_BOUNDARY_NOT_IN_ORDER; reason_code=REJECT_S2_BOUNDARY_NOT_IN_ORDER"
        );
    }

    let role_is_a = st.recv.role_is_a;
    let a2b_recv = !send_is_a_to_b(role_is_a); // the receive direction

    // §8.5.1: the boundary header MUST decrypt under the CURRENT NHK_r (pre-boundary RK).
    let current_nhk_r = match header_key(kmac, &st.dh.rk, a2b_recv, true) {
        Ok(v) => v,
        Err(_) => reject!(st, "REJECT_S2_LOCAL_UNSUPPORTED"),
    };
    let pq_bind = binding::pq_bind_sha512_32(hash, flags, &[]);
    let ad_hdr = binding::ad_hdr(
        &st.recv.session_id,
        st.recv.protocol_version,
        st.recv.suite_id,
        &parsed.dh_pub,
        flags,
        &pq_bind,
    );
    let ad_body = binding::ad_body(
        &st.recv.session_id,
        st.recv.protocol_version,
        st.recv.suite_id,
        &pq_bind,
    );
    let n0: u32 = 0;
    let hdr_pt = match aead.open(
        &current_nhk_r,
        &nonce_hdr(hash, &st.recv.session_id, &parsed.dh_pub, n0),
        &ad_hdr,
        &parsed.hdr_ct,
    ) {
        Ok(pt) => pt,
        Err(_) => reject!(st, "REJECT_S2_HDR_AUTH_FAIL"),
    };
    if hdr_pt.len() != 8 {
        reject!(st, "REJECT_S2_HDR_AUTH_FAIL");
    }
    let n_val = u32::from_be_bytes([hdr_pt[4], hdr_pt[5], hdr_pt[6], hdr_pt[7]]);
    if n_val != 0 {
        reject!(
            st,
            "REJECT_S2_BOUNDARY_NOT_IN_ORDER; reason_code=REJECT_S2_BOUNDARY_NOT_IN_ORDER"
        );
    }

    // DH ratchet: advance the root, reinit the receive chains, recompute HK_r.
    let dh_out = dh.dh(&X25519Priv(st.dh.dhs_priv), &X25519Pub(parsed.dh_pub));
    let (rk1, ck_ec0) = match kdf_rk_dh(kmac, &st.dh.rk, &dh_out) {
        Ok(v) => v,
        Err(_) => reject!(st, "REJECT_S2_LOCAL_UNSUPPORTED"),
    };
    let ck_pq0 = match kmac32(kmac, &rk1, pq0_recv_label(role_is_a), &[0x01]) {
        Ok(v) => v,
        Err(_) => reject!(st, "REJECT_S2_LOCAL_UNSUPPORTED"),
    };
    let hk_r_new = match header_key(kmac, &rk1, a2b_recv, false) {
        Ok(v) => v,
        Err(_) => reject!(st, "REJECT_S2_LOCAL_UNSUPPORTED"),
    };
    let (ck_ec_p, ck_pq_p, mk) = match derive_mk_step(kmac, &ck_ec0, &ck_pq0) {
        Ok(v) => v,
        Err(_) => reject!(st, "REJECT_S2_LOCAL_UNSUPPORTED"),
    };
    let pt = match aead.open(
        &mk,
        &nonce_body(hash, &st.recv.session_id, &parsed.dh_pub, n0),
        &ad_body,
        &parsed.body_ct,
    ) {
        Ok(pt) => pt,
        Err(_) => reject!(st, "REJECT_S2_BODY_AUTH_FAIL"),
    };

    // Commit receive + DH state (send chain untouched).
    let mut new = st;
    new.dh.rk = rk1;
    new.dh.dhr = parsed.dh_pub;
    new.recv.dh_pub = parsed.dh_pub;
    new.recv.hk_r = hk_r_new;
    new.recv.ck_ec = ck_ec_p;
    new.recv.ck_pq_recv = ck_pq_p;
    new.recv.nr = 1;
    RecvDhBoundaryOutcome {
        state: new,
        plaintext: pt,
        ok: true,
        reason: None,
    }
}

// ============ NA-0623 (ENG-0012 Stage 2a): SCKA sender (advertisement + PQ reseed) ============
// The send side of the SCKA control plane (DOC-CAN-004 §3.1/§3.3) and the PQ-reseed boundary
// (DOC-CAN-003 §8.5.3/§8.5.4), operating at the session level (Suite2SessionState) so the PQ root
// advance composes with the classical DH ratchet: on a reseed the new root lands in BOTH the
// PQ-path root (`recv.rk`) and the DH-ratchet root (`dh.rk`), so a subsequent DH boundary
// (send_boundary/recv_dh_boundary) reinitialises `CK_pq` from a PQ-hardened root and the
// post-quantum protection is carried forward permanently (the D560 AMENDMENT).
//
// These functions are PURE and generate/store NO key material: the caller (the interop actor / a
// test now; qsc in Stage 2b) owns the advertised-key store (`advkeys`: adv_id -> ML-KEM secret
// key + consumed), `local_next_adv_id`, and the peer's advertised public key (DOC-CAN-004 §2), and
// performs the ML-KEM KeyGen / Encap / Decap. The public/ciphertext bytes are passed in. The
// reseed is the exact structural mirror of `recv_boundary_in_order`'s PQ path (which the receiver
// uses to decrypt), including its header key: NA-0625 (ENG-0023) aligned BOTH sides to the
// §8.5.1 `NHK` boundary-header rule (sealed/opened under the NHK derived from the PRE-reseed
// root), closing the NA-0623 deviation. The ADV header stays `HK` (§8.5.4 states no CURRENT_NHK
// step and an ADV advances no root; the §8.5.1/§8.5.4 textual tension is recorded in DOC-G5-008).

const PQ_ADV_PUB_LEN: usize = 1184; // ML-KEM-768 public key (DOC-CAN-004 §1.3)
const MLKEM768_CT_LEN: usize = 1088; // ML-KEM-768 ciphertext (DOC-CAN-004 §1.3)
const MLKEM768_SS_LEN: usize = 32; // ML-KEM-768 shared secret (DOC-CAN-004 §1.3)

pub struct SendPqAdvertiseOutcome {
    pub state: crate::suite2::state::Suite2SessionState,
    pub wire: Vec<u8>,
}

pub struct SendPqReseedOutcome {
    pub state: crate::suite2::state::Suite2SessionState,
    pub wire: Vec<u8>,
}

/// Frame a Suite-2 ratchet wire message that carries a PQ prefix (mirrors `send_wire` /
/// `frame_suite2_wire`, inserting `pq_prefix` between the flags and the header ciphertext exactly
/// as `parse.rs` decodes it).
fn frame_pq_wire(
    protocol_version: u16,
    suite_id: u16,
    dh_pub: &[u8; 32],
    flags: u16,
    pq_prefix: &[u8],
    hdr_ct: &[u8],
    body_ct: &[u8],
) -> Vec<u8> {
    let mut header = Vec::with_capacity(32 + 2 + pq_prefix.len() + hdr_ct.len());
    header.extend_from_slice(dh_pub);
    header.extend_from_slice(&flags.to_be_bytes());
    header.extend_from_slice(pq_prefix);
    header.extend_from_slice(hdr_ct);
    let mut wire = Vec::with_capacity(10 + header.len() + body_ct.len());
    wire.extend_from_slice(&protocol_version.to_be_bytes());
    wire.extend_from_slice(&suite_id.to_be_bytes());
    wire.push(0x02);
    wire.push(0x00);
    wire.extend_from_slice(&(header.len() as u16).to_be_bytes());
    wire.extend_from_slice(&(body_ct.len() as u16).to_be_bytes());
    wire.extend_from_slice(&header);
    wire.extend_from_slice(body_ct);
    wire
}

/// The canonical session root (DOC-CAN-003 §8.1: one `RK`). The refimpl stores it redundantly in
/// `recv.rk` (read by the PQ path) and `dh.rk` (read/advanced by the DH ratchet); prefer the DH
/// root when populated (a live ratcheting session), else the recv root (the interop actor's
/// non-DH plumbing path leaves `dh` zero). The reseed writes BOTH on commit so they stay equal.
fn session_root(st: &crate::suite2::state::Suite2SessionState) -> [u8; 32] {
    if !is_zero32(&st.dh.rk) {
        st.dh.rk
    } else {
        st.recv.rk
    }
}

/// DOC-CAN-004 §3.1 / DOC-CAN-003 §8.5.4: SCKA advertisement SEND (boundary with `FLAG_PQ_ADV`).
///
/// The caller has already generated the ML-KEM-768 receive keypair, allocated the strictly
/// increasing `pq_adv_id` (`local_next_adv_id + 1`), and persisted the secret key in its
/// advertised-key store (DOC-CAN-004 §3.1 steps 1-3). This function records `pq_adv_id` in the
/// local `known_targets` set — so a later peer CTXT targeting it passes `apply_pq_reseed`'s
/// known-target check — and frames a `FLAG_PQ_ADV | FLAG_BOUNDARY` message carrying
/// `(pq_adv_id, pq_adv_pub)`. Fail-closed with no state mutation on any reject.
pub fn send_pq_advertise(
    hash: &dyn Hash,
    kmac: &dyn Kmac,
    aead: &dyn Aead,
    mut st: crate::suite2::state::Suite2SessionState,
    pq_adv_id: u32,
    pq_adv_pub: &[u8],
    plaintext: &[u8],
) -> Result<SendPqAdvertiseOutcome, &'static str> {
    if pq_adv_pub.len() != PQ_ADV_PUB_LEN {
        return Err("REJECT_SCKA_ADV_BAD_PUB_LEN");
    }
    // Strictly-increasing allocation (DOC-CAN-004 §3.1 step 1): the id must exceed every id we
    // have already advertised (and thus not already be present).
    if let Some(max_known) = st.recv.known_targets.iter().next_back() {
        if pq_adv_id <= *max_known {
            return Err("REJECT_SCKA_ADV_NONMONOTONIC");
        }
    }
    if is_zero32(&st.send.ck_ec) || is_zero32(&st.send.ck_pq) {
        return Err(REJECT_S2_CHAINKEY_UNSET);
    }
    // NA-0625 (ENG-0023): the ADVAUTH MAC keys off the canonical session root; a session with no
    // live root cannot authenticate its control plane (fail-closed, mirrors send_pq_reseed).
    let rk = session_root(&st);
    if is_zero32(&rk) {
        return Err(REJECT_S2_CHAINKEY_UNSET);
    }
    let ns_next = checked_counter_inc(st.send.ns)?;
    let (ck_ec_p, ck_pq_p, mk) = derive_mk_step(kmac, &st.send.ck_ec, &st.send.ck_pq)
        .map_err(|_| "REJECT_S2_LOCAL_UNSUPPORTED")?;

    let flags = types::FLAG_PQ_ADV | types::FLAG_BOUNDARY;
    let mut pq_prefix = Vec::with_capacity(4 + PQ_ADV_PUB_LEN);
    pq_prefix.extend_from_slice(&pq_adv_id.to_be_bytes());
    pq_prefix.extend_from_slice(pq_adv_pub);

    let pq_bind = binding::pq_bind_sha512_32(hash, flags, &pq_prefix);
    let ad_hdr = binding::ad_hdr(
        &st.send.session_id,
        st.send.protocol_version,
        st.send.suite_id,
        &st.send.dh_pub,
        flags,
        &pq_bind,
    );
    let ad_body = binding::ad_body(
        &st.send.session_id,
        st.send.protocol_version,
        st.send.suite_id,
        &pq_bind,
    );
    let hdr_pt = {
        let mut v = Vec::with_capacity(8);
        v.extend_from_slice(&st.send.pn.to_be_bytes());
        v.extend_from_slice(&st.send.ns.to_be_bytes());
        v
    };
    // NA-0625 (ENG-0023): authenticated ADV — the SPQR-style ADVAUTH MAC rides as the first 32
    // bytes of the sealed body plaintext (the receiver strips + verifies it before tracking).
    // The ADV HEADER stays under `HK_s` (§8.5.4 states no CURRENT_NHK step; an ADV advances no
    // root — the §8.5.1/§8.5.4 textual tension is recorded in DOC-G5-008, ENG-0023 note).
    let adv_mac = adv_auth_mac(kmac, &rk, pq_adv_id, pq_adv_pub)
        .map_err(|_| "REJECT_S2_LOCAL_UNSUPPORTED")?;
    let mut body_pt = Vec::with_capacity(ADV_MAC_LEN + plaintext.len());
    body_pt.extend_from_slice(&adv_mac);
    body_pt.extend_from_slice(plaintext);
    let hdr_ct = aead.seal(
        &st.send.hk_s,
        &nonce_hdr(hash, &st.send.session_id, &st.send.dh_pub, st.send.ns),
        &ad_hdr,
        &hdr_pt,
    );
    let body_ct = aead.seal(
        &mk,
        &nonce_body(hash, &st.send.session_id, &st.send.dh_pub, st.send.ns),
        &ad_body,
        &body_pt,
    );
    if hdr_ct.is_empty() || body_ct.is_empty() {
        return Err("REJECT_S2_LOCAL_AEAD_FAIL");
    }
    let wire = frame_pq_wire(
        st.send.protocol_version,
        st.send.suite_id,
        &st.send.dh_pub,
        flags,
        &pq_prefix,
        &hdr_ct,
        &body_ct,
    );

    // Commit only on full success.
    st.recv.known_targets.insert(pq_adv_id);
    st.send.ck_ec = ck_ec_p;
    st.send.ck_pq = ck_pq_p;
    st.send.ns = ns_next;
    Ok(SendPqAdvertiseOutcome { state: st, wire })
}

/// DOC-CAN-003 §8.5.3 (sender/encapsulator side) / DOC-CAN-004 §3.3: SCKA PQ-reseed SEND
/// (boundary with `FLAG_PQ_CTXT`).
///
/// The caller has already run `MLKEM768.Encap(peer_adv_pub) -> (pq_ct, pq_epoch_ss)` on the peer's
/// advertised public key and passes the ciphertext + shared secret in; `pq_target_id` is the peer
/// advertisement id being targeted. The exact structural mirror of `recv_boundary_in_order`'s PQ
/// path: derive the directional PQ seeds from `RK_old` (reused `kdf_pq_reseed_seeds`, so both
/// parties converge), advance the root with `KDF_RK_PQ` (§3.3.6 ordering: seeds first), recompute
/// the directional header keys, replace the directional PQ chains, and frame the boundary message
/// under the PRE-reseed key schedule. The new root is written to both root slots so the DH ratchet
/// carries the PQ hardening forward. Fail-closed with no state mutation on any reject.
#[allow(clippy::too_many_arguments)]
pub fn send_pq_reseed(
    hash: &dyn Hash,
    kmac: &dyn Kmac,
    aead: &dyn Aead,
    mut st: crate::suite2::state::Suite2SessionState,
    pq_target_id: u32,
    pq_ct: &[u8],
    pq_epoch_ss: &[u8],
    plaintext: &[u8],
) -> Result<SendPqReseedOutcome, &'static str> {
    if pq_ct.len() != MLKEM768_CT_LEN {
        return Err("REJECT_SCKA_CTXT_BAD_CT_LEN");
    }
    if pq_epoch_ss.len() != MLKEM768_SS_LEN {
        return Err("REJECT_SCKA_CTXT_BAD_SS_LEN");
    }
    if is_zero32(&st.send.ck_ec) || is_zero32(&st.send.ck_pq) {
        return Err(REJECT_S2_CHAINKEY_UNSET);
    }
    let rk_old = session_root(&st);
    if is_zero32(&rk_old) {
        return Err(REJECT_S2_CHAINKEY_UNSET);
    }
    let role_is_a = st.recv.role_is_a;

    // §8.5.3 step 4: directional PQ seeds from RK_old (identical to the receiver's derivation).
    let (seed_a2b, seed_b2a) =
        scka::kdf_pq_reseed_seeds(hash, kmac, &rk_old, pq_ct, pq_epoch_ss, pq_target_id);
    let (ck_pq_send_after, ck_pq_recv_after) = if role_is_a {
        (seed_a2b, seed_b2a)
    } else {
        (seed_b2a, seed_a2b)
    };

    // §8.5.3 step 5: advance the root (AFTER the seeds — §3.3.6 normative ordering).
    let new_rk =
        kdf_rk_pq(kmac, &rk_old, pq_epoch_ss).map_err(|_| "REJECT_S2_LOCAL_UNSUPPORTED")?;

    // §8.5.3 step 7: recompute the directional header keys from the new root.
    let a2b_send = send_is_a_to_b(role_is_a);
    let hk_s_new =
        header_key(kmac, &new_rk, a2b_send, false).map_err(|_| "REJECT_S2_LOCAL_UNSUPPORTED")?;
    let hk_r_new =
        header_key(kmac, &new_rk, !a2b_send, false).map_err(|_| "REJECT_S2_LOCAL_UNSUPPORTED")?;

    // NA-0625 (ENG-0023): §8.5.1 — the boundary header is sealed under the sender's CURRENT
    // `NHK_s`, derived from the PRE-reseed root (the receiver mirrors it from its canonical
    // root). Derived on demand; no stored NHK field. The post-commit directional header keys
    // above remain HK-from-new-root (§8.5.3 step 7 unchanged).
    let nhk_s =
        header_key(kmac, &rk_old, a2b_send, true).map_err(|_| "REJECT_S2_LOCAL_UNSUPPORTED")?;

    // The boundary message is an in-order message under the PRE-reseed key schedule (mirror
    // recv_boundary_in_order, which opens the header under the pre-boundary hk_r at n == nr and
    // derives the body mk from the pre-boundary ck_ec/ck_pq): advance the EC send chain one step;
    // the reseed replaces the PQ chains for FUTURE messages.
    let ns_next = checked_counter_inc(st.send.ns)?;
    let (ck_ec_p, _ck_pq_p, mk) = derive_mk_step(kmac, &st.send.ck_ec, &st.send.ck_pq)
        .map_err(|_| "REJECT_S2_LOCAL_UNSUPPORTED")?;

    let flags = types::FLAG_PQ_CTXT | types::FLAG_BOUNDARY;
    let mut pq_prefix = Vec::with_capacity(4 + MLKEM768_CT_LEN);
    pq_prefix.extend_from_slice(&pq_target_id.to_be_bytes());
    pq_prefix.extend_from_slice(pq_ct);

    let pq_bind = binding::pq_bind_sha512_32(hash, flags, &pq_prefix);
    let ad_hdr = binding::ad_hdr(
        &st.send.session_id,
        st.send.protocol_version,
        st.send.suite_id,
        &st.send.dh_pub,
        flags,
        &pq_bind,
    );
    let ad_body = binding::ad_body(
        &st.send.session_id,
        st.send.protocol_version,
        st.send.suite_id,
        &pq_bind,
    );
    let hdr_pt = {
        let mut v = Vec::with_capacity(8);
        v.extend_from_slice(&st.send.pn.to_be_bytes());
        v.extend_from_slice(&st.send.ns.to_be_bytes());
        v
    };
    let hdr_ct = aead.seal(
        &nhk_s,
        &nonce_hdr(hash, &st.send.session_id, &st.send.dh_pub, st.send.ns),
        &ad_hdr,
        &hdr_pt,
    );
    let body_ct = aead.seal(
        &mk,
        &nonce_body(hash, &st.send.session_id, &st.send.dh_pub, st.send.ns),
        &ad_body,
        plaintext,
    );
    if hdr_ct.is_empty() || body_ct.is_empty() {
        return Err("REJECT_S2_LOCAL_AEAD_FAIL");
    }
    let wire = frame_pq_wire(
        st.send.protocol_version,
        st.send.suite_id,
        &st.send.dh_pub,
        flags,
        &pq_prefix,
        &hdr_ct,
        &body_ct,
    );

    // Commit only on full success (§8.5.3 step 8 semantics). Advance the EC send chain; replace the
    // directional PQ chains; advance the root in BOTH slots; recompute the directional header keys.
    st.send.ck_ec = ck_ec_p;
    st.send.ck_pq = ck_pq_send_after;
    st.send.hk_s = hk_s_new;
    st.send.ns = ns_next;
    st.recv.ck_pq_send = ck_pq_send_after;
    st.recv.ck_pq_recv = ck_pq_recv_after;
    st.recv.hk_r = hk_r_new;
    st.recv.rk = new_rk;
    st.dh.rk = new_rk;
    Ok(SendPqReseedOutcome { state: st, wire })
}

pub struct RecvPqAdvOutcome {
    pub state: Suite2RecvWireState,
    pub ok: bool,
    pub reason: Option<&'static str>,
    pub plaintext: Option<Vec<u8>>,
    pub pn: Option<u32>,
    pub n: Option<u32>,
}

/// NA-0625 (ENG-0023): AUTHENTICATED SCKA advertisement RECEIVE (DOC-CAN-004 §3.2 / DOC-CAN-003
/// §8.5.4) — the receive-side mirror of `send_pq_advertise`. A tracked advertisement is
/// cryptographically bound to the session BEFORE it can be persisted: the header must open under
/// the receive header key with the ADV `pq_bind` in the AD, the body must open under the in-order
/// hybrid message key, and the ADVAUTH MAC (first 32 bytes of the body plaintext) must verify
/// under the canonical session root. An unauthenticated or failed ADV is REJECTED, never tracked.
///
/// IN-ORDER-ONLY control plane (`n == nr`, mirroring the CTXT boundary receiver), and the ADV
/// CONSUMES its chain slot (Operator Decision 2): BOTH `ck_ec` and `ck_pq_recv` advance one step
/// (mirroring the sender, which advances both send chains) plus `nr` — so an in-order ADV leaves
/// no receive-chain gap (no mkskipped growth) and an [ADV, reseed] pack round-trips. Replay of an
/// old authentic ADV fails the header open (counter/nonce mismatch at candidates [nr, nr+1]) and
/// would also fail the `track_peer_adv` monotonicity check. Fail-closed: NO state mutation on any
/// reject.
///
/// `peer_adv_watermark` is the CALLER-OWNED peer-advertisement watermark (qsc: the SCKA store's
/// `peer_adv_max_seen`; both parties allocate adv ids from independent counters). It is
/// deliberately NOT `st.peer_max_adv_id_seen`: that session field is the FROZEN CTXT receiver's
/// reseed-target watermark (`apply_pq_reseed` compares the ids of OUR OWN consumed
/// advertisements against it), and the two id spaces collide — tracking the peer's ADV id 1
/// there would make the peer's next reseed of our key id 1 reject NONMONOTONIC. The caller
/// persists the advanced watermark (`pq_adv_id`) alongside the staged peer advertisement.
#[allow(clippy::too_many_arguments)]
pub fn recv_pq_adv(
    hash: &dyn Hash,
    kmac: &dyn Kmac,
    aead: &dyn Aead,
    st: Suite2RecvWireState,
    flags: u16,
    pq_prefix: &[u8],
    pq_adv_id: u32,
    pq_adv_pub: &[u8],
    peer_adv_watermark: u32,
    dh_pub: &[u8; 32],
    hdr_ct: &[u8],
    body_ct: &[u8],
) -> RecvPqAdvOutcome {
    macro_rules! reject {
        ($st:expr, $reason:expr, $pn:expr, $n:expr) => {
            return RecvPqAdvOutcome {
                state: $st,
                ok: false,
                reason: Some($reason),
                plaintext: None,
                pn: $pn,
                n: $n,
            }
        };
    }

    // Flags must be exactly BOUNDARY|PQ_ADV: a combined ADV+CTXT frame stays unsupported
    // (ENG-0026 territory).
    if flags != (types::FLAG_BOUNDARY | types::FLAG_PQ_ADV) {
        reject!(st, "REJECT_S2_LOCAL_UNSUPPORTED", None, None);
    }
    if hdr_ct.len() != HDR_CT_LEN {
        reject!(st, "REJECT_S2_HDR_AUTH_FAIL", None, None);
    }
    if body_ct.len() < BODY_CT_MIN {
        reject!(st, "REJECT_S2_BODY_AUTH_FAIL", None, None);
    }

    let pq_bind = binding::pq_bind_sha512_32(hash, flags, pq_prefix);
    let ad_hdr = binding::ad_hdr(
        &st.session_id,
        st.protocol_version,
        st.suite_id,
        dh_pub,
        flags,
        &pq_bind,
    );
    let ad_body = binding::ad_body(&st.session_id, st.protocol_version, st.suite_id, &pq_bind);

    // The ADV header opens under the receive header key `HK_r` (§8.5.4 — an ADV advances no
    // root, so §8.5.1's NHK epoch-transition rule does not bind it; see DOC-G5-008).
    let mut header_pt: Option<[u8; 8]> = None;
    let mut n: u32 = 0;
    let candidates = [st.nr, st.nr.saturating_add(1)];
    for cand in candidates {
        let nonce_hdr = nonce_hdr(hash, &st.session_id, dh_pub, cand);
        #[cfg(test)]
        S2_HDR_TRY_COUNT_BOUNDARY.with(|c| c.set(c.get().saturating_add(1)));
        if let Ok(pt) = aead.open(&st.hk_r, &nonce_hdr, &ad_hdr, hdr_ct) {
            if pt.len() == 8 {
                let n_val = u32::from_be_bytes([pt[4], pt[5], pt[6], pt[7]]);
                if n_val == cand {
                    header_pt = Some([pt[0], pt[1], pt[2], pt[3], pt[4], pt[5], pt[6], pt[7]]);
                    n = n_val;
                    break;
                }
            }
        }
    }
    let header_pt = match header_pt {
        Some(v) => v,
        None => reject!(st, "REJECT_S2_HDR_AUTH_FAIL", None, None),
    };
    let header_pn = u32::from_be_bytes([header_pt[0], header_pt[1], header_pt[2], header_pt[3]]);
    if n != st.nr {
        reject!(
            st,
            "REJECT_S2_BOUNDARY_NOT_IN_ORDER; reason_code=REJECT_S2_BOUNDARY_NOT_IN_ORDER",
            Some(header_pn),
            Some(n)
        );
    }

    if is_zero32(&st.ck_ec) || is_zero32(&st.ck_pq_recv) {
        reject!(st, REJECT_S2_CHAINKEY_UNSET, Some(header_pn), Some(n));
    }

    // Chain-consume in-order (Decision 2): derive the slot's hybrid mk and advance BOTH chains.
    let (ck_ec_p, ck_pq_p, mk) = match derive_mk_step(kmac, &st.ck_ec, &st.ck_pq_recv) {
        Ok(v) => v,
        Err(_) => reject!(st, "REJECT_S2_BODY_AUTH_FAIL", Some(header_pn), Some(n)),
    };

    let nonce_body = nonce_body(hash, &st.session_id, dh_pub, n);
    let body_pt = match aead.open(&mk, &nonce_body, &ad_body, body_ct) {
        Ok(pt) => pt,
        Err(_) => reject!(st, "REJECT_S2_BODY_AUTH_FAIL", Some(header_pn), Some(n)),
    };

    // ADVAUTH (Decision 1): body_pt = adv_mac(32) || app_payload. A pre-NA-0625-format body
    // (shorter than the MAC) and a MAC mismatch both reuse REJECT_S2_BODY_AUTH_FAIL.
    if body_pt.len() < ADV_MAC_LEN {
        reject!(st, "REJECT_S2_BODY_AUTH_FAIL", Some(header_pn), Some(n));
    }
    let expected_mac = match adv_auth_mac(kmac, &st.rk, pq_adv_id, pq_adv_pub) {
        Ok(v) => v,
        Err(_) => reject!(st, "REJECT_S2_LOCAL_UNSUPPORTED", Some(header_pn), Some(n)),
    };
    if !ct_eq32(&expected_mac, &body_pt[..ADV_MAC_LEN]) {
        reject!(st, "REJECT_S2_BODY_AUTH_FAIL", Some(header_pn), Some(n));
    }
    let app_payload = body_pt[ADV_MAC_LEN..].to_vec();

    // DOC-CAN-004 §3.2 track rules (length + monotonicity against the caller-owned peer-ADV
    // watermark), existing reason codes. The advanced watermark is caller-persisted.
    if let Err(code) = track_peer_adv(peer_adv_watermark, pq_adv_id, pq_adv_pub) {
        reject!(st, code, Some(header_pn), Some(n));
    }

    let nr_next = match checked_counter_inc(n) {
        Ok(v) => v,
        Err(reason) => reject!(st, reason, Some(header_pn), Some(n)),
    };

    // Commit atomically: both receive chains step and nr advances. `peer_max_adv_id_seen` is
    // NOT touched (see the watermark note above — it belongs to the frozen CTXT path).
    let mut new_state = st.clone();
    new_state.ck_ec = ck_ec_p;
    new_state.ck_pq_recv = ck_pq_p;
    new_state.nr = nr_next;

    RecvPqAdvOutcome {
        state: new_state,
        ok: true,
        reason: None,
        plaintext: Some(app_payload),
        pn: Some(header_pn),
        n: Some(n),
    }
}

/// DOC-CAN-004 §3.2: process a peer advertisement (the SCKA track side). Enforces the peer-ADV
/// monotonicity and public-key length rules and returns the updated `peer_max_adv_id_seen`. The
/// caller records the peer's advertised public key in its peer state on success. Pure; fail-closed.
pub fn track_peer_adv(
    peer_max_adv_id_seen: u32,
    peer_adv_id: u32,
    peer_adv_pub: &[u8],
) -> Result<u32, &'static str> {
    if peer_adv_pub.len() != PQ_ADV_PUB_LEN {
        return Err("REJECT_SCKA_ADV_BAD_PUB_LEN");
    }
    if peer_adv_id <= peer_max_adv_id_seen {
        return Err("REJECT_SCKA_ADV_NONMONOTONIC");
    }
    Ok(peer_adv_id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::stdcrypto::StdCrypto;
    use crate::crypto::traits::CryptoError;
    use crate::suite2::types;
    use rand_core::{OsRng, RngCore};

    fn snapshot_boundary_state(st: &Suite2BoundaryState) -> Vec<u8> {
        let mut out = Vec::new();
        out.extend_from_slice(&st.session_id);
        out.extend_from_slice(&st.protocol_version.to_be_bytes());
        out.extend_from_slice(&st.suite_id.to_be_bytes());
        out.extend_from_slice(&st.dh_pub);
        out.extend_from_slice(&st.hk_r);
        out.extend_from_slice(&st.rk);
        out.extend_from_slice(&st.ck_ec);
        out.extend_from_slice(&st.ck_pq_send);
        out.extend_from_slice(&st.ck_pq_recv);
        out.extend_from_slice(&st.nr.to_be_bytes());
        out.push(if st.role_is_a { 1 } else { 0 });
        out.extend_from_slice(&st.peer_max_adv_id_seen.to_be_bytes());
        out.extend_from_slice(&(st.known_targets.len() as u32).to_be_bytes());
        for v in &st.known_targets {
            out.extend_from_slice(&v.to_be_bytes());
        }
        out.extend_from_slice(&(st.consumed_targets.len() as u32).to_be_bytes());
        for v in &st.consumed_targets {
            out.extend_from_slice(&v.to_be_bytes());
        }
        out.extend_from_slice(&(st.tombstoned_targets.len() as u32).to_be_bytes());
        for v in &st.tombstoned_targets {
            out.extend_from_slice(&v.to_be_bytes());
        }
        out
    }

    fn boundary_state_with_target(target_id: u32) -> Suite2BoundaryState {
        let mut known = BTreeSet::new();
        known.insert(target_id);
        Suite2BoundaryState {
            session_id: rng16(),
            protocol_version: 5,
            suite_id: 2,
            dh_pub: rng32(),
            hk_r: rng32(),
            rk: rng32(),
            ck_ec: rng32(),
            ck_pq_send: rng32(),
            ck_pq_recv: rng32(),
            nr: 0,
            role_is_a: true,
            peer_max_adv_id_seen: 0,
            known_targets: known,
            consumed_targets: BTreeSet::new(),
            tombstoned_targets: BTreeSet::new(),
        }
    }

    fn make_pq_prefix(target_id: u32, pq_ct: &[u8]) -> Vec<u8> {
        let mut out = Vec::with_capacity(4 + pq_ct.len());
        out.extend_from_slice(&target_id.to_be_bytes());
        out.extend_from_slice(pq_ct);
        out
    }

    fn snapshot_recv_state(st: &Suite2RecvState) -> Vec<u8> {
        let mut out = Vec::new();
        out.extend_from_slice(&st.session_id);
        out.extend_from_slice(&st.protocol_version.to_be_bytes());
        out.extend_from_slice(&st.suite_id.to_be_bytes());
        out.extend_from_slice(&st.dh_pub);
        out.extend_from_slice(&st.hk_r);
        out.extend_from_slice(&st.ck_ec);
        out.extend_from_slice(&st.ck_pq);
        out.extend_from_slice(&st.nr.to_be_bytes());
        out.extend_from_slice(&(st.mkskipped.len() as u32).to_be_bytes());
        for entry in &st.mkskipped {
            out.extend_from_slice(&entry.dh_pub);
            out.extend_from_slice(&entry.n.to_be_bytes());
            out.extend_from_slice(&entry.mk);
        }
        out
    }

    fn snapshot_send_state(st: &Suite2SendState) -> Vec<u8> {
        let mut out = Vec::new();
        out.extend_from_slice(&st.session_id);
        out.extend_from_slice(&st.protocol_version.to_be_bytes());
        out.extend_from_slice(&st.suite_id.to_be_bytes());
        out.extend_from_slice(&st.dh_pub);
        out.extend_from_slice(&st.hk_s);
        out.extend_from_slice(&st.ck_ec);
        out.extend_from_slice(&st.ck_pq);
        out.extend_from_slice(&st.ns.to_be_bytes());
        out.extend_from_slice(&st.pn.to_be_bytes());
        out
    }

    struct RejectAead;
    impl Aead for RejectAead {
        fn seal(&self, _key32: &[u8; 32], _nonce12: &[u8; 12], _ad: &[u8], _pt: &[u8]) -> Vec<u8> {
            Vec::new()
        }
        fn open(
            &self,
            _key32: &[u8; 32],
            _nonce12: &[u8; 12],
            _ad: &[u8],
            _ct: &[u8],
        ) -> Result<Vec<u8>, CryptoError> {
            Err(CryptoError::AuthFail)
        }
    }

    struct PanicAead;
    impl Aead for PanicAead {
        fn seal(&self, _key32: &[u8; 32], _nonce12: &[u8; 12], _ad: &[u8], _pt: &[u8]) -> Vec<u8> {
            panic!("unexpected AEAD use");
        }
        fn open(
            &self,
            _key32: &[u8; 32],
            _nonce12: &[u8; 12],
            _ad: &[u8],
            _ct: &[u8],
        ) -> Result<Vec<u8>, CryptoError> {
            panic!("unexpected AEAD use");
        }
    }

    struct AcceptAead;
    impl Aead for AcceptAead {
        fn seal(&self, _key32: &[u8; 32], _nonce12: &[u8; 12], _ad: &[u8], _pt: &[u8]) -> Vec<u8> {
            Vec::new()
        }
        fn open(
            &self,
            _key32: &[u8; 32],
            _nonce12: &[u8; 12],
            _ad: &[u8],
            _ct: &[u8],
        ) -> Result<Vec<u8>, CryptoError> {
            Ok(vec![0, 0, 0, 0, 0, 0, 0, 0])
        }
    }

    struct MkSkippedHeaderAead {
        pn: u32,
        n: u32,
    }
    impl Aead for MkSkippedHeaderAead {
        fn seal(&self, _key32: &[u8; 32], _nonce12: &[u8; 12], _ad: &[u8], _pt: &[u8]) -> Vec<u8> {
            Vec::new()
        }
        fn open(
            &self,
            _key32: &[u8; 32],
            _nonce12: &[u8; 12],
            _ad: &[u8],
            ct: &[u8],
        ) -> Result<Vec<u8>, CryptoError> {
            if ct.len() == HDR_CT_LEN {
                let mut out = Vec::with_capacity(8);
                out.extend_from_slice(&self.pn.to_be_bytes());
                out.extend_from_slice(&self.n.to_be_bytes());
                return Ok(out);
            }
            Err(CryptoError::AuthFail)
        }
    }

    struct HeaderPtInvalidAead;
    impl Aead for HeaderPtInvalidAead {
        fn seal(&self, _key32: &[u8; 32], _nonce12: &[u8; 12], _ad: &[u8], _pt: &[u8]) -> Vec<u8> {
            Vec::new()
        }
        fn open(
            &self,
            _key32: &[u8; 32],
            _nonce12: &[u8; 12],
            _ad: &[u8],
            _ct: &[u8],
        ) -> Result<Vec<u8>, CryptoError> {
            Ok(vec![0u8; 7])
        }
    }

    #[test]
    fn boundary_reject_is_deterministic_and_no_state_mutation_on_bad_ct_len() {
        let c = StdCrypto;
        let st = boundary_state_with_target(1);
        let flags = types::FLAG_BOUNDARY | types::FLAG_PQ_CTXT;
        let pq_prefix = make_pq_prefix(1, &[0xAA]);
        let pq_epoch_ss = [0xBB; 32];

        let pq_bind = binding::pq_bind_sha512_32(&c, flags, &pq_prefix);
        let ad_hdr = binding::ad_hdr(
            &st.session_id,
            st.protocol_version,
            st.suite_id,
            &st.dh_pub,
            flags,
            &pq_bind,
        );
        let hdr_pt = {
            let mut v = Vec::with_capacity(8);
            v.extend_from_slice(&0u32.to_be_bytes());
            v.extend_from_slice(&st.nr.to_be_bytes());
            v
        };
        // NA-0625 (ENG-0023): boundary headers seal under the NHK from the pre-reseed root
        // (§8.5.1), so the ct-len reject (not HDR_AUTH_FAIL) is what this test exercises.
        let nhk = header_key(&c, &st.rk, !send_is_a_to_b(st.role_is_a), true).expect("nhk");
        let hdr_ct = c.seal(
            &nhk,
            &nonce_hdr(&c, &st.session_id, &st.dh_pub, st.nr),
            &ad_hdr,
            &hdr_pt,
        );
        let body_ct = vec![0u8; BODY_CT_MIN];

        let snap_before = snapshot_boundary_state(&st);
        let out1 = recv_boundary_in_order(
            &c,
            &c,
            &c,
            st.clone(),
            flags,
            &pq_prefix,
            &hdr_ct,
            &body_ct,
            &pq_epoch_ss,
            1,
        );
        let out2 = recv_boundary_in_order(
            &c,
            &c,
            &c,
            st.clone(),
            flags,
            &pq_prefix,
            &hdr_ct,
            &body_ct,
            &pq_epoch_ss,
            1,
        );

        assert!(!out1.ok);
        assert_eq!(out1.reason, out2.reason);
        assert_eq!(snap_before, snapshot_boundary_state(&out1.state));
    }

    #[test]
    fn header_pt_invalid_rejects_deterministically_and_no_state_mutation() {
        let c = StdCrypto;
        let st = boundary_state_with_target(1);
        let flags = types::FLAG_BOUNDARY | types::FLAG_PQ_CTXT;
        let pq_prefix = make_pq_prefix(1, &[]);
        let pq_epoch_ss = [0xDD; 32];
        let hdr_ct = vec![0u8; HDR_CT_LEN];
        let body_ct = vec![0u8; BODY_CT_MIN];

        let snap_before = snapshot_boundary_state(&st);
        let out1 = recv_boundary_in_order(
            &c,
            &c,
            &HeaderPtInvalidAead,
            st.clone(),
            flags,
            &pq_prefix,
            &hdr_ct,
            &body_ct,
            &pq_epoch_ss,
            1,
        );
        let out2 = recv_boundary_in_order(
            &c,
            &c,
            &HeaderPtInvalidAead,
            st.clone(),
            flags,
            &pq_prefix,
            &hdr_ct,
            &body_ct,
            &pq_epoch_ss,
            1,
        );

        assert!(!out1.ok);
        assert_eq!(out1.reason, out2.reason);
        assert_eq!(snap_before, snapshot_boundary_state(&out1.state));
    }

    #[test]
    fn header_pt_invalid_does_not_panic() {
        let c = StdCrypto;
        let st = boundary_state_with_target(1);
        let flags = types::FLAG_BOUNDARY | types::FLAG_PQ_CTXT;
        let pq_prefix = make_pq_prefix(1, &[]);
        let pq_epoch_ss = [0xEE; 32];
        let hdr_ct = vec![0u8; HDR_CT_LEN];
        let body_ct = vec![0u8; BODY_CT_MIN];

        let out = recv_boundary_in_order(
            &c,
            &c,
            &HeaderPtInvalidAead,
            st,
            flags,
            &pq_prefix,
            &hdr_ct,
            &body_ct,
            &pq_epoch_ss,
            1,
        );
        assert!(!out.ok);
    }

    #[test]
    fn issue22_boundary_single_attempt_no_mutation_on_reject() {
        let c = StdCrypto;
        let st = boundary_state_with_target(1);
        let flags = types::FLAG_BOUNDARY | types::FLAG_PQ_CTXT;
        let pq_prefix = make_pq_prefix(1, &[]);
        let pq_epoch_ss = [0xAB; 32];
        let hdr_ct = vec![0u8; HDR_CT_LEN];
        let body_ct = vec![0u8; BODY_CT_MIN];

        let snap_before = snapshot_boundary_state(&st);
        S2_HDR_TRY_COUNT_BOUNDARY.with(|c| c.set(0));
        let out1 = recv_boundary_in_order(
            &c,
            &c,
            &RejectAead,
            st.clone(),
            flags,
            &pq_prefix,
            &hdr_ct,
            &body_ct,
            &pq_epoch_ss,
            1,
        );
        let tries1 = S2_HDR_TRY_COUNT_BOUNDARY.with(|c| c.get());

        S2_HDR_TRY_COUNT_BOUNDARY.with(|c| c.set(0));
        let out2 = recv_boundary_in_order(
            &c,
            &c,
            &RejectAead,
            st.clone(),
            flags,
            &pq_prefix,
            &hdr_ct,
            &body_ct,
            &pq_epoch_ss,
            1,
        );
        let tries2 = S2_HDR_TRY_COUNT_BOUNDARY.with(|c| c.get());

        assert!(!out1.ok);
        assert_eq!(out1.reason, Some("REJECT_S2_HDR_AUTH_FAIL"));
        assert_eq!(out1.reason, out2.reason);
        assert_eq!(snap_before, snapshot_boundary_state(&out1.state));
        assert_eq!(snap_before, snapshot_boundary_state(&out2.state));
        assert_eq!(tries1, 2);
        assert_eq!(tries2, 2);
    }

    #[test]
    fn boundary_success_advances_ck_pq_recv_from_reseed() {
        let c = StdCrypto;
        let st = boundary_state_with_target(7);
        let flags = types::FLAG_BOUNDARY | types::FLAG_PQ_CTXT;
        let pq_ct = vec![0u8; 1088];
        let pq_prefix = make_pq_prefix(7, &pq_ct);
        let pq_epoch_ss = [0xCC; 32];

        let apply = scka::apply_pq_reseed(
            &c,
            &c,
            st.role_is_a,
            &st.rk,
            &pq_ct,
            &pq_epoch_ss,
            1,
            st.peer_max_adv_id_seen,
            &st.known_targets,
            &st.consumed_targets,
            &st.tombstoned_targets,
            7,
            true,
            &st.ck_pq_send,
            &st.ck_pq_recv,
        )
        .expect("apply_pq_reseed");

        let (_ck_ec_p, _ck_pq_p, mk) =
            derive_mk_step(&c, &st.ck_ec, &st.ck_pq_recv).expect("derive_mk_step");

        let pq_bind = binding::pq_bind_sha512_32(&c, flags, &pq_prefix);
        let ad_hdr = binding::ad_hdr(
            &st.session_id,
            st.protocol_version,
            st.suite_id,
            &st.dh_pub,
            flags,
            &pq_bind,
        );
        let ad_body = binding::ad_body(&st.session_id, st.protocol_version, st.suite_id, &pq_bind);

        let hdr_pt = {
            let mut v = Vec::with_capacity(8);
            v.extend_from_slice(&0u32.to_be_bytes());
            v.extend_from_slice(&st.nr.to_be_bytes());
            v
        };
        // NA-0625 (ENG-0023): the boundary header seals under the sender's NHK from the
        // pre-reseed root (§8.5.1); the receiver derives the same key from its `rk`.
        let nhk = header_key(&c, &st.rk, !send_is_a_to_b(st.role_is_a), true).expect("nhk");
        let hdr_ct = c.seal(
            &nhk,
            &nonce_hdr(&c, &st.session_id, &st.dh_pub, st.nr),
            &ad_hdr,
            &hdr_pt,
        );
        let body_ct = c.seal(
            &mk,
            &nonce_body(&c, &st.session_id, &st.dh_pub, st.nr),
            &ad_body,
            b"ok",
        );

        let out = recv_boundary_in_order(
            &c,
            &c,
            &c,
            st.clone(),
            flags,
            &pq_prefix,
            &hdr_ct,
            &body_ct,
            &pq_epoch_ss,
            1,
        );
        assert!(out.ok);
        assert_ne!(st.ck_pq_recv, out.state.ck_pq_recv);
        assert_eq!(apply.ck_pq_recv_after, out.state.ck_pq_recv);
    }

    fn rng32() -> [u8; 32] {
        use core::mem::MaybeUninit;
        #[cfg(test)]
        use rand_core::RngCore;

        let mut out = MaybeUninit::<[u8; 32]>::uninit();
        let buf = unsafe { &mut *out.as_mut_ptr() };
        rand_core::OsRng.fill_bytes(&mut buf[..]);
        unsafe { out.assume_init() }
    }

    fn rng16() -> [u8; 16] {
        use core::mem::MaybeUninit;
        #[cfg(test)]
        use rand_core::RngCore;

        let mut out = MaybeUninit::<[u8; 16]>::uninit();
        let buf = unsafe { &mut *out.as_mut_ptr() };
        rand_core::OsRng.fill_bytes(&mut buf[..]);
        unsafe { out.assume_init() }
    }

    fn zero32() -> [u8; 32] {
        let mut out = rng32();
        out.fill(0);
        out
    }

    fn zero_send_state() -> Suite2SendState {
        Suite2SendState {
            session_id: [0x11; 16],
            protocol_version: 5,
            suite_id: 2,
            dh_pub: rng32(),
            hk_s: rng32(),
            ck_ec: zero32(),
            ck_pq: zero32(),
            ns: 0,
            pn: 0,
        }
    }

    #[test]
    fn send_wire_rejects_unset_chainkey_deterministically_and_no_mutation() {
        let c = StdCrypto;
        let aead = PanicAead;
        let st = zero_send_state();
        let before = snapshot_send_state(&st);
        let err1 = match send_wire(&c, &c, &aead, zero_send_state(), 0, b"hi") {
            Ok(_) => panic!("expected send_wire to reject unset chain key"),
            Err(e) => e,
        };
        let err2 = match send_wire(&c, &c, &aead, zero_send_state(), 0, b"hi") {
            Ok(_) => panic!("expected send_wire to reject unset chain key"),
            Err(e) => e,
        };
        assert_eq!(err1, REJECT_S2_CHAINKEY_UNSET);
        assert_eq!(err1, err2);
        assert_eq!(before, snapshot_send_state(&st));
    }

    #[test]
    fn asymmetric_send_unset_chainkey_rejects_deterministically_and_no_mutation() {
        let c = StdCrypto;
        let aead = PanicAead;
        let st = Suite2SendState {
            session_id: rng16(),
            protocol_version: 5,
            suite_id: 2,
            dh_pub: rng32(),
            hk_s: rng32(),
            ck_ec: zero32(),
            ck_pq: rng32(),
            ns: 0,
            pn: 0,
        };
        let before = snapshot_send_state(&st);
        let err1 = match send_wire(&c, &c, &aead, st.clone(), 0, b"hi") {
            Ok(_) => panic!("expected send_wire to reject unset chain key"),
            Err(e) => e,
        };
        let err2 = match send_wire(&c, &c, &aead, st.clone(), 0, b"hi") {
            Ok(_) => panic!("expected send_wire to reject unset chain key"),
            Err(e) => e,
        };
        assert!(err1.contains("reason_code=REJECT_S2_CHAINKEY_UNSET"));
        assert_eq!(err1, err2);
        assert_eq!(before, snapshot_send_state(&st));
    }

    // NA-0618 (ENG-0013): the counter-overflow hard-stop. `checked_counter_inc` is the single
    // point used at every ns/nr advance (send_wire, recv_nonboundary_ooo,
    // recv_boundary_in_order), so this directly covers the fail-closed logic at all three
    // sites — including the receive-side guards, which are otherwise unreachable through the
    // public API because a compliant sender (guarded below) never originates a message at the
    // saturating counter.
    #[test]
    fn checked_counter_inc_boundary_and_normal() {
        assert_eq!(checked_counter_inc(0), Ok(1));
        assert_eq!(checked_counter_inc(u32::MAX - 1), Ok(u32::MAX));
        match checked_counter_inc(u32::MAX) {
            Ok(_) => panic!("expected overflow reject at u32::MAX"),
            Err(e) => assert!(e.contains("REJECT_S2_COUNTER_OVERFLOW")),
        }
    }

    #[test]
    fn send_wire_rejects_counter_overflow_at_ns_max_and_no_mutation() {
        let c = StdCrypto;
        let aead = PanicAead; // the guard returns before any AEAD use
        let st = Suite2SendState {
            session_id: rng16(),
            protocol_version: 5,
            suite_id: 2,
            dh_pub: rng32(),
            hk_s: rng32(),
            ck_ec: rng32(),
            ck_pq: rng32(),
            ns: u32::MAX,
            pn: 0,
        };
        let before = snapshot_send_state(&st);
        let err1 = match send_wire(&c, &c, &aead, st.clone(), 0, b"hi") {
            Ok(_) => panic!("expected send_wire to reject at ns == u32::MAX"),
            Err(e) => e,
        };
        let err2 = match send_wire(&c, &c, &aead, st.clone(), 0, b"hi") {
            Ok(_) => panic!("expected send_wire to reject at ns == u32::MAX"),
            Err(e) => e,
        };
        assert!(err1.contains("REJECT_S2_COUNTER_OVERFLOW"));
        assert_eq!(err1, err2);
        assert_eq!(before, snapshot_send_state(&st));
    }

    #[test]
    fn asymmetric_recv_unset_chainkey_rejects_deterministically_and_no_mutation() {
        let c = StdCrypto;
        let st = Suite2RecvState {
            session_id: rng16(),
            protocol_version: 5,
            suite_id: 2,
            dh_pub: rng32(),
            hk_r: rng32(),
            ck_ec: rng32(),
            ck_pq: zero32(),
            nr: 0,
            mkskipped: Vec::new(),
        };
        let flags = 0;
        let hdr_ct = vec![0u8; HDR_CT_LEN];
        let body_ct = vec![0u8; BODY_CT_MIN];
        let pre = snapshot_recv_state(&st);
        let out1 = recv_nonboundary_ooo(&c, &c, &RejectAead, st.clone(), flags, &hdr_ct, &body_ct);
        let out2 = recv_nonboundary_ooo(&c, &c, &RejectAead, st.clone(), flags, &hdr_ct, &body_ct);
        assert!(!out1.ok);
        let err1 = out1.reason.unwrap_or("");
        let err2 = out2.reason.unwrap_or("");
        assert!(err1.contains("reason_code=REJECT_S2_CHAINKEY_UNSET"));
        assert_eq!(err1, err2);
        assert_eq!(pre, snapshot_recv_state(&out1.state));
    }

    #[test]
    fn nonboundary_rejects_deterministically_and_no_state_mutation() {
        let c = StdCrypto;
        let st = Suite2RecvState {
            session_id: [0x11; 16],
            protocol_version: 5,
            suite_id: 2,
            dh_pub: rng32(),
            hk_r: rng32(),
            ck_ec: rng32(),
            ck_pq: rng32(),
            nr: 0,
            mkskipped: Vec::new(),
        };
        let flags = 0;
        let hdr_ct = vec![0u8; HDR_CT_LEN];
        let body_ct = vec![0u8; BODY_CT_MIN];
        let pre = snapshot_recv_state(&st);
        let out1 = recv_nonboundary_ooo(&c, &c, &RejectAead, st.clone(), flags, &hdr_ct, &body_ct);
        let out2 = recv_nonboundary_ooo(&c, &c, &RejectAead, st.clone(), flags, &hdr_ct, &body_ct);
        assert!(!out1.ok);
        assert_eq!(out1.reason, out2.reason);
        assert_eq!(pre, snapshot_recv_state(&out1.state));
    }

    #[test]
    fn issue21_mkskipped_not_removed_on_auth_fail() {
        let c = StdCrypto;
        let hk_r: [u8; 32] = {
            let mut v = vec![1u8; 32];
            OsRng.fill_bytes(&mut v);
            v.try_into().expect("hk_r length")
        };
        let ck_ec: [u8; 32] = {
            let mut v = vec![1u8; 32];
            OsRng.fill_bytes(&mut v);
            v.try_into().expect("ck_ec length")
        };
        let ck_pq: [u8; 32] = {
            let mut v = vec![1u8; 32];
            OsRng.fill_bytes(&mut v);
            v.try_into().expect("ck_pq length")
        };
        let mk: [u8; 32] = {
            let mut v = vec![1u8; 32];
            OsRng.fill_bytes(&mut v);
            v.try_into().expect("mk length")
        };
        let st = Suite2RecvState {
            session_id: [0x11; 16],
            protocol_version: 5,
            suite_id: 2,
            dh_pub: [0x22; 32],
            hk_r,
            ck_ec,
            ck_pq,
            nr: 0,
            mkskipped: vec![MkSkippedEntry {
                dh_pub: [0x22; 32],
                n: 5,
                mk,
            }],
        };
        let flags = 0;
        let hdr_ct = vec![0u8; HDR_CT_LEN];
        let body_ct = vec![0u8; BODY_CT_MIN];
        let aead = MkSkippedHeaderAead { pn: 0, n: 5 };

        let pre = snapshot_recv_state(&st);
        let out1 = recv_nonboundary_ooo(&c, &c, &aead, st.clone(), flags, &hdr_ct, &body_ct);
        let out2 = recv_nonboundary_ooo(&c, &c, &aead, st.clone(), flags, &hdr_ct, &body_ct);
        assert!(!out1.ok);
        assert_eq!(out1.reason, Some("REJECT_S2_BODY_AUTH_FAIL"));
        assert_eq!(out1.reason, out2.reason);
        assert_eq!(pre, snapshot_recv_state(&out1.state));
        assert_eq!(pre, snapshot_recv_state(&out2.state));
    }

    #[test]
    fn nonboundary_header_breaks_on_first_success() {
        S2_HDR_TRY_COUNT_NONBOUNDARY.with(|c| c.set(0));
        let c = StdCrypto;
        let st = Suite2RecvState {
            session_id: rng16(),
            protocol_version: 5,
            suite_id: 2,
            dh_pub: rng32(),
            hk_r: rng32(),
            ck_ec: rng32(),
            ck_pq: rng32(),
            nr: 0,
            mkskipped: Vec::new(),
        };
        let flags = 0;
        let hdr_ct = vec![0u8; HDR_CT_LEN];
        let body_ct = vec![0u8; BODY_CT_MIN];
        let out = recv_nonboundary_ooo(&c, &c, &AcceptAead, st.clone(), flags, &hdr_ct, &body_ct);
        assert!(out.ok);
        let count = S2_HDR_TRY_COUNT_NONBOUNDARY.with(|c| c.get());
        assert_eq!(count, 1);
    }

    #[test]
    fn recv_invalid_message_caps_header_attempts() {
        let c = StdCrypto;
        let st = Suite2RecvState {
            session_id: rng16(),
            protocol_version: 5,
            suite_id: 2,
            dh_pub: rng32(),
            hk_r: rng32(),
            ck_ec: rng32(),
            ck_pq: rng32(),
            nr: 7,
            mkskipped: Vec::new(),
        };
        let flags = 0;
        let hdr_ct = vec![0u8; HDR_CT_LEN];
        let body_ct = vec![0u8; BODY_CT_MIN];
        let pre = snapshot_recv_state(&st);
        S2_HDR_TRY_COUNT_NONBOUNDARY.with(|c| c.set(0));
        let out = recv_nonboundary_ooo(&c, &c, &RejectAead, st.clone(), flags, &hdr_ct, &body_ct);
        let count = S2_HDR_TRY_COUNT_NONBOUNDARY.with(|c| c.get());
        assert!(!out.ok);
        assert_eq!(out.reason, Some("REJECT_S2_HDR_AUTH_FAIL"));
        assert!(count <= MAX_HEADER_ATTEMPTS);
        assert_eq!(pre, snapshot_recv_state(&out.state));
    }

    #[test]
    fn recv_far_future_message_fails_fast_without_mutation() {
        let c = StdCrypto;
        let recv = Suite2RecvState {
            session_id: [0x11; 16],
            protocol_version: 5,
            suite_id: 2,
            dh_pub: [0x22; 32],
            hk_r: [0x33; 32],
            ck_ec: [0x44; 32],
            ck_pq: [0x55; 32],
            nr: 0,
            mkskipped: Vec::new(),
        };
        let mut send = Suite2SendState {
            session_id: recv.session_id,
            protocol_version: recv.protocol_version,
            suite_id: recv.suite_id,
            dh_pub: recv.dh_pub,
            hk_s: recv.hk_r,
            ck_ec: recv.ck_ec,
            ck_pq: recv.ck_pq,
            ns: 0,
            pn: 0,
        };
        let mut far_wire: Option<Vec<u8>> = None;
        for i in 0..(MAX_SKIP as usize + 2) {
            let out = send_wire(&c, &c, &c, send.clone(), 0, b"x").expect("send_wire");
            send = out.state;
            if i == (MAX_SKIP as usize + 1) {
                far_wire = Some(out.wire);
            }
        }
        let wire = far_wire.expect("far wire");
        let header_offset = 10usize;
        let hdr_ct_start = header_offset + 32 + 2;
        let hdr_ct_end = hdr_ct_start + HDR_CT_LEN;
        let body_ct_start = hdr_ct_end;
        let hdr_ct = &wire[hdr_ct_start..hdr_ct_end];
        let body_ct = &wire[body_ct_start..];
        let pre = snapshot_recv_state(&recv);
        S2_HDR_TRY_COUNT_NONBOUNDARY.with(|c| c.set(0));
        let out = recv_nonboundary_ooo(&c, &c, &c, recv.clone(), 0, hdr_ct, body_ct);
        let count = S2_HDR_TRY_COUNT_NONBOUNDARY.with(|c| c.get());
        assert!(!out.ok);
        assert_eq!(out.reason, Some("REJECT_S2_OOO_BOUNDS"));
        assert!(count <= MAX_HEADER_ATTEMPTS);
        assert_eq!(pre, snapshot_recv_state(&out.state));
    }

    // NA-0621 (ENG-0012 Stage 1b-i): DH-ratchet round-trip, no-mutation-on-reject, and the
    // post-compromise self-healing (PCS) property.
    fn matched_dh_pair(
        c: &StdCrypto,
    ) -> (
        crate::suite2::state::Suite2SessionState,
        crate::suite2::state::Suite2SessionState,
    ) {
        let (a_priv, a_pub) = c.keypair();
        let (b_priv, b_pub) = c.keypair();
        let session_id = [0x5a; 16];
        let pq_init_ss = [0x77; 32];
        let dh_init = c.dh(&a_priv, &b_pub);
        assert_eq!(dh_init, c.dh(&b_priv, &a_pub), "X25519 must be symmetric");
        let mut a = crate::suite2::establish::init_from_base_handshake(
            c,
            true,
            types::SUITE2_PROTOCOL_VERSION,
            types::SUITE2_SUITE_ID,
            &session_id,
            &dh_init,
            &pq_init_ss,
            &a_pub.0,
            &b_pub.0,
            true,
        )
        .expect("establish A");
        a.set_dh_self_priv(a_priv.0);
        let mut b = crate::suite2::establish::init_from_base_handshake(
            c,
            false,
            types::SUITE2_PROTOCOL_VERSION,
            types::SUITE2_SUITE_ID,
            &session_id,
            &dh_init,
            &pq_init_ss,
            &b_pub.0,
            &a_pub.0,
            true,
        )
        .expect("establish B");
        b.set_dh_self_priv(b_priv.0);
        (a, b)
    }

    #[test]
    fn dh_ratchet_two_party_roundtrip_both_directions() {
        let c = StdCrypto;
        let (a, b) = matched_dh_pair(&c);
        // A performs a DH boundary and sends; B receives and decrypts.
        let sa = send_boundary(&c, &c, &c, &c, a, b"hello-from-a").expect("A send_boundary");
        let rb = recv_dh_boundary(&c, &c, &c, &c, b, &sa.wire);
        assert!(rb.ok, "B recv failed: {:?}", rb.reason);
        assert_eq!(rb.plaintext, b"hello-from-a");
        // Reverse direction: B performs a DH boundary and sends; A receives and decrypts.
        let sb = send_boundary(&c, &c, &c, &c, rb.state, b"hello-from-b").expect("B send_boundary");
        let ra = recv_dh_boundary(&c, &c, &c, &c, sa.state, &sb.wire);
        assert!(ra.ok, "A recv failed: {:?}", ra.reason);
        assert_eq!(ra.plaintext, b"hello-from-b");
    }

    #[test]
    fn dh_ratchet_no_mutation_on_reject() {
        let c = StdCrypto;
        let (a, b) = matched_dh_pair(&c);
        let sa = send_boundary(&c, &c, &c, &c, a, b"m").expect("A send_boundary");
        // Flip a body-ciphertext bit -> body AEAD fails; state must be returned unchanged.
        let mut bad = sa.wire.clone();
        let last = bad.len() - 1;
        bad[last] ^= 0x01;
        let b_pre = b.snapshot_bytes();
        let rb = recv_dh_boundary(&c, &c, &c, &c, b, &bad);
        assert!(!rb.ok);
        assert_eq!(
            rb.state.snapshot_bytes(),
            b_pre,
            "state must not mutate on reject"
        );
    }

    #[test]
    fn dh_ratchet_pcs_healing() {
        let c = StdCrypto;
        let (a0, b0) = matched_dh_pair(&c);
        // Adversary snapshots B's full state at epoch 0 (all keys, incl. DHs_priv and RK).
        let b_captured = b0.clone();
        // Round 1: A ratchets, B receives.
        let sa1 = send_boundary(&c, &c, &c, &c, a0, b"r1").expect("A1");
        let rb1 = recv_dh_boundary(&c, &c, &c, &c, b0, &sa1.wire);
        assert!(rb1.ok);
        // Round 2: B ratchets with a FRESH keypair the adversary never captured; A receives.
        let sb1 = send_boundary(&c, &c, &c, &c, rb1.state, b"r2").expect("B1");
        let ra1 = recv_dh_boundary(&c, &c, &c, &c, sa1.state, &sb1.wire);
        assert!(ra1.ok);
        // Round 3: A ratchets again and sends a secret; the REAL B decrypts it.
        let sa2 = send_boundary(&c, &c, &c, &c, ra1.state, b"top-secret").expect("A2");
        let rb2 = recv_dh_boundary(&c, &c, &c, &c, sb1.state, &sa2.wire);
        assert!(rb2.ok, "real B must decrypt post-ratchet: {:?}", rb2.reason);
        assert_eq!(rb2.plaintext, b"top-secret");
        // PCS: the epoch-0 snapshot lacks B's fresh key and the advanced root, so it CANNOT
        // decrypt the round-3 message (the boundary header fails to authenticate under its stale
        // NHK). Post-compromise security has self-healed.
        let heal = recv_dh_boundary(&c, &c, &c, &c, b_captured, &sa2.wire);
        assert!(
            !heal.ok,
            "pre-ratchet snapshot must NOT decrypt a post-ratchet message (PCS self-healing)"
        );
    }

    // ============ NA-0625 (ENG-0023): NHK boundary header + authenticated ADV receive ============

    /// A pre-NA-0625-style PQ-CTXT boundary frame (header sealed under HK, not NHK) MUST fail
    /// generically with REJECT_S2_HDR_AUTH_FAIL and no state mutation (the header-downgrade
    /// rejection of the DoD).
    #[test]
    fn ctxt_boundary_hk_downgrade_rejects_no_mutation() {
        let c = StdCrypto;
        let st = boundary_state_with_target(7);
        let flags = types::FLAG_BOUNDARY | types::FLAG_PQ_CTXT;
        let pq_ct = vec![0u8; 1088];
        let pq_prefix = make_pq_prefix(7, &pq_ct);
        let pq_epoch_ss = [0xCC; 32];

        let (_ck_ec_p, _ck_pq_p, mk) =
            derive_mk_step(&c, &st.ck_ec, &st.ck_pq_recv).expect("derive_mk_step");
        let pq_bind = binding::pq_bind_sha512_32(&c, flags, &pq_prefix);
        let ad_hdr = binding::ad_hdr(
            &st.session_id,
            st.protocol_version,
            st.suite_id,
            &st.dh_pub,
            flags,
            &pq_bind,
        );
        let ad_body = binding::ad_body(&st.session_id, st.protocol_version, st.suite_id, &pq_bind);
        let hdr_pt = {
            let mut v = Vec::with_capacity(8);
            v.extend_from_slice(&0u32.to_be_bytes());
            v.extend_from_slice(&st.nr.to_be_bytes());
            v
        };
        // DOWNGRADE: seal under the ordinary HK_r (the pre-NA-0625 sender behaviour).
        let hdr_ct = c.seal(
            &st.hk_r,
            &nonce_hdr(&c, &st.session_id, &st.dh_pub, st.nr),
            &ad_hdr,
            &hdr_pt,
        );
        let body_ct = c.seal(
            &mk,
            &nonce_body(&c, &st.session_id, &st.dh_pub, st.nr),
            &ad_body,
            b"ok",
        );

        let snap_before = snapshot_boundary_state(&st);
        let out = recv_boundary_in_order(
            &c,
            &c,
            &c,
            st.clone(),
            flags,
            &pq_prefix,
            &hdr_ct,
            &body_ct,
            &pq_epoch_ss,
            1,
        );
        assert!(
            !out.ok,
            "an HK-sealed boundary frame must not open under NHK"
        );
        assert_eq!(out.reason, Some("REJECT_S2_HDR_AUTH_FAIL"));
        assert_eq!(snap_before, snapshot_boundary_state(&out.state));
    }

    fn adv_recv_state(nr: u32) -> Suite2RecvWireState {
        Suite2RecvWireState {
            session_id: rng16(),
            protocol_version: 5,
            suite_id: 2,
            dh_pub: rng32(),
            hk_r: rng32(),
            rk: rng32(),
            ck_ec: rng32(),
            ck_pq_send: rng32(),
            ck_pq_recv: rng32(),
            nr,
            role_is_a: true,
            peer_max_adv_id_seen: 0,
            known_targets: BTreeSet::new(),
            consumed_targets: BTreeSet::new(),
            tombstoned_targets: BTreeSet::new(),
            mkskipped: Vec::new(),
        }
    }

    fn snapshot_recv_wire_state(st: &Suite2RecvWireState) -> Vec<u8> {
        let mut out = Vec::new();
        out.extend_from_slice(&st.hk_r);
        out.extend_from_slice(&st.rk);
        out.extend_from_slice(&st.ck_ec);
        out.extend_from_slice(&st.ck_pq_send);
        out.extend_from_slice(&st.ck_pq_recv);
        out.extend_from_slice(&st.nr.to_be_bytes());
        out.extend_from_slice(&st.peer_max_adv_id_seen.to_be_bytes());
        out.extend_from_slice(&(st.mkskipped.len() as u32).to_be_bytes());
        out
    }

    /// Seal an ADV frame exactly as `send_pq_advertise` frames it, against the given receiver
    /// state's keys (the mirror sender), with knobs for the negative cases.
    #[allow(clippy::too_many_arguments)]
    fn seal_adv_frame(
        c: &StdCrypto,
        st: &Suite2RecvWireState,
        n: u32,
        adv_id: u32,
        adv_pub: &[u8],
        payload: &[u8],
        mac_rk: Option<&[u8; 32]>, // None => omit the MAC (pre-NA-0625-format body)
        hdr_key: &[u8; 32],
    ) -> (u16, Vec<u8>, Vec<u8>, Vec<u8>) {
        let flags = types::FLAG_PQ_ADV | types::FLAG_BOUNDARY;
        let mut pq_prefix = Vec::with_capacity(4 + adv_pub.len());
        pq_prefix.extend_from_slice(&adv_id.to_be_bytes());
        pq_prefix.extend_from_slice(adv_pub);

        let mut ck_ec = st.ck_ec;
        let mut ck_pq = st.ck_pq_recv;
        let mut mk = [0u8; 32];
        for _ in st.nr..=n {
            let (e, p, m) = derive_mk_step(c, &ck_ec, &ck_pq).expect("derive");
            mk = m;
            ck_ec = e;
            ck_pq = p;
        }

        let pq_bind = binding::pq_bind_sha512_32(c, flags, &pq_prefix);
        let ad_hdr = binding::ad_hdr(
            &st.session_id,
            st.protocol_version,
            st.suite_id,
            &st.dh_pub,
            flags,
            &pq_bind,
        );
        let ad_body = binding::ad_body(&st.session_id, st.protocol_version, st.suite_id, &pq_bind);
        let hdr_pt = {
            let mut v = Vec::with_capacity(8);
            v.extend_from_slice(&0u32.to_be_bytes());
            v.extend_from_slice(&n.to_be_bytes());
            v
        };
        let hdr_ct = c.seal(
            hdr_key,
            &nonce_hdr(c, &st.session_id, &st.dh_pub, n),
            &ad_hdr,
            &hdr_pt,
        );
        let mut body_pt = Vec::new();
        if let Some(rk) = mac_rk {
            let mac = adv_auth_mac(c, rk, adv_id, adv_pub).expect("adv mac");
            body_pt.extend_from_slice(&mac);
        }
        body_pt.extend_from_slice(payload);
        let body_ct = c.seal(
            &mk,
            &nonce_body(c, &st.session_id, &st.dh_pub, n),
            &ad_body,
            &body_pt,
        );
        (flags, pq_prefix, hdr_ct, body_ct)
    }

    /// Authenticated in-order ADV: accepted, BOTH receive chains consume the slot, nr advances,
    /// the peer-ADV watermark tracks, and the app payload comes back.
    #[test]
    fn adv_recv_accept_consumes_chain_and_tracks() {
        let c = StdCrypto;
        let st = adv_recv_state(3);
        let adv_pub = vec![0xBB; 1184];
        let adv_id = 9u32;
        let (flags, pq_prefix, hdr_ct, body_ct) = seal_adv_frame(
            &c,
            &st,
            st.nr,
            adv_id,
            &adv_pub,
            b"adv-payload",
            Some(&st.rk),
            &st.hk_r,
        );
        let (exp_ck_ec, exp_ck_pq, _mk) =
            derive_mk_step(&c, &st.ck_ec, &st.ck_pq_recv).expect("derive");

        let out = recv_pq_adv(
            &c,
            &c,
            &c,
            st.clone(),
            flags,
            &pq_prefix,
            adv_id,
            &adv_pub,
            0,
            &st.dh_pub,
            &hdr_ct,
            &body_ct,
        );
        assert!(
            out.ok,
            "authenticated ADV must be accepted: {:?}",
            out.reason
        );
        assert_eq!(out.plaintext.as_deref(), Some(&b"adv-payload"[..]));
        assert_eq!(out.n, Some(3));
        assert_eq!(out.state.nr, 4, "nr advances exactly once (chain-consume)");
        assert_eq!(out.state.ck_ec, exp_ck_ec, "EC chain consumed the slot");
        assert_eq!(
            out.state.ck_pq_recv, exp_ck_pq,
            "PQ chain consumed the slot"
        );
        assert_eq!(
            out.state.peer_max_adv_id_seen, st.peer_max_adv_id_seen,
            "the frozen CTXT-path watermark field is untouched (caller owns the ADV watermark)"
        );
        assert!(out.state.mkskipped.is_empty(), "no receive-chain gap");
    }

    /// A valid AEAD body whose ADVAUTH MAC is corrupted rejects with the reused
    /// REJECT_S2_BODY_AUTH_FAIL and no state mutation.
    #[test]
    fn adv_recv_bad_mac_rejects_no_mutation() {
        let c = StdCrypto;
        let st = adv_recv_state(0);
        let adv_pub = vec![0xBB; 1184];
        let wrong_rk = rng32();
        let (flags, pq_prefix, hdr_ct, body_ct) = seal_adv_frame(
            &c,
            &st,
            st.nr,
            1,
            &adv_pub,
            b"",
            Some(&wrong_rk), // MAC under a foreign root
            &st.hk_r,
        );
        let snap = snapshot_recv_wire_state(&st);
        let out = recv_pq_adv(
            &c,
            &c,
            &c,
            st.clone(),
            flags,
            &pq_prefix,
            1,
            &adv_pub,
            0,
            &st.dh_pub,
            &hdr_ct,
            &body_ct,
        );
        assert!(!out.ok);
        assert_eq!(out.reason, Some("REJECT_S2_BODY_AUTH_FAIL"));
        assert_eq!(snap, snapshot_recv_wire_state(&out.state));
    }

    /// A pre-NA-0625-format ADV body (no leading MAC) rejects — the ADV downgrade case.
    #[test]
    fn adv_recv_missing_mac_rejects_no_mutation() {
        let c = StdCrypto;
        let st = adv_recv_state(0);
        let adv_pub = vec![0xBB; 1184];
        let (flags, pq_prefix, hdr_ct, body_ct) =
            seal_adv_frame(&c, &st, st.nr, 1, &adv_pub, b"legacy", None, &st.hk_r);
        let snap = snapshot_recv_wire_state(&st);
        let out = recv_pq_adv(
            &c,
            &c,
            &c,
            st.clone(),
            flags,
            &pq_prefix,
            1,
            &adv_pub,
            0,
            &st.dh_pub,
            &hdr_ct,
            &body_ct,
        );
        assert!(!out.ok);
        assert_eq!(out.reason, Some("REJECT_S2_BODY_AUTH_FAIL"));
        assert_eq!(snap, snapshot_recv_wire_state(&out.state));
    }

    /// A planted/foreign-key ADV (header sealed under an attacker key) fails FIRST at the header
    /// AEAD with REJECT_S2_HDR_AUTH_FAIL and no mutation — the primary spoofed-ADV rejection.
    #[test]
    fn adv_recv_spoofed_header_rejects_no_mutation() {
        let c = StdCrypto;
        let st = adv_recv_state(0);
        let adv_pub = vec![0xBB; 1184];
        let foreign_key = rng32();
        let (flags, pq_prefix, hdr_ct, body_ct) =
            seal_adv_frame(&c, &st, st.nr, 1, &adv_pub, b"", Some(&st.rk), &foreign_key);
        let snap = snapshot_recv_wire_state(&st);
        let out = recv_pq_adv(
            &c,
            &c,
            &c,
            st.clone(),
            flags,
            &pq_prefix,
            1,
            &adv_pub,
            0,
            &st.dh_pub,
            &hdr_ct,
            &body_ct,
        );
        assert!(!out.ok);
        assert_eq!(out.reason, Some("REJECT_S2_HDR_AUTH_FAIL"));
        assert_eq!(snap, snapshot_recv_wire_state(&out.state));
    }

    /// The ADV control plane is in-order-only: a frame at n = nr+1 opens (candidate window) but
    /// rejects REJECT_S2_BOUNDARY_NOT_IN_ORDER with no mutation.
    #[test]
    fn adv_recv_out_of_order_rejects_no_mutation() {
        let c = StdCrypto;
        let st = adv_recv_state(2);
        let adv_pub = vec![0xBB; 1184];
        let (flags, pq_prefix, hdr_ct, body_ct) =
            seal_adv_frame(&c, &st, st.nr + 1, 1, &adv_pub, b"", Some(&st.rk), &st.hk_r);
        let snap = snapshot_recv_wire_state(&st);
        let out = recv_pq_adv(
            &c,
            &c,
            &c,
            st.clone(),
            flags,
            &pq_prefix,
            1,
            &adv_pub,
            0,
            &st.dh_pub,
            &hdr_ct,
            &body_ct,
        );
        assert!(!out.ok);
        assert!(out
            .reason
            .unwrap_or("")
            .starts_with("REJECT_S2_BOUNDARY_NOT_IN_ORDER"));
        assert_eq!(snap, snapshot_recv_wire_state(&out.state));
    }

    /// An authenticated but NON-MONOTONIC advertisement (id <= the caller-owned watermark)
    /// rejects with the existing REJECT_SCKA_ADV_NONMONOTONIC and no mutation (replays are
    /// doubly rejected: the header counter has moved on AND monotonicity fails).
    #[test]
    fn adv_recv_nonmonotonic_rejects_no_mutation() {
        let c = StdCrypto;
        let st = adv_recv_state(0);
        let adv_pub = vec![0xBB; 1184];
        let (flags, pq_prefix, hdr_ct, body_ct) =
            seal_adv_frame(&c, &st, st.nr, 5, &adv_pub, b"", Some(&st.rk), &st.hk_r);
        let snap = snapshot_recv_wire_state(&st);
        let out = recv_pq_adv(
            &c,
            &c,
            &c,
            st.clone(),
            flags,
            &pq_prefix,
            5,
            &adv_pub,
            5,
            &st.dh_pub,
            &hdr_ct,
            &body_ct,
        );
        assert!(!out.ok);
        assert_eq!(out.reason, Some("REJECT_SCKA_ADV_NONMONOTONIC"));
        assert_eq!(snap, snapshot_recv_wire_state(&out.state));
    }

    /// A combined ADV+CTXT frame stays REJECT_S2_LOCAL_UNSUPPORTED (ENG-0026 territory).
    #[test]
    fn adv_recv_combined_flags_rejects() {
        let c = StdCrypto;
        let st = adv_recv_state(0);
        let adv_pub = vec![0xBB; 1184];
        let flags = types::FLAG_PQ_ADV | types::FLAG_PQ_CTXT | types::FLAG_BOUNDARY;
        let out = recv_pq_adv(
            &c,
            &c,
            &c,
            st.clone(),
            flags,
            &[],
            1,
            &adv_pub,
            0,
            &st.dh_pub,
            &[0u8; HDR_CT_LEN],
            &[0u8; BODY_CT_MIN],
        );
        assert!(!out.ok);
        assert_eq!(out.reason, Some("REJECT_S2_LOCAL_UNSUPPORTED"));
    }
}
