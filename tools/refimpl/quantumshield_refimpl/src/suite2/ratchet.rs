//! Suite-2 ratchet surface (minimal helpers).

use crate::crypto::traits::{Aead, CryptoError, Hash, Kmac};
use std::collections::BTreeSet;
#[cfg(test)]
use std::cell::Cell;

use crate::suite2::{binding, parse, scka, types};

const MAX_SKIP: u32 = 1000;
const MAX_MKSKIPPED: usize = 1000;
const MAX_HEADER_ATTEMPTS: usize = 100;
const HDR_CT_LEN: usize = 24;
const BODY_CT_MIN: usize = 16;

#[cfg(test)]
thread_local! {
    static S2_HDR_TRY_COUNT_NONBOUNDARY: Cell<usize> = Cell::new(0);
    static S2_HDR_TRY_COUNT_BOUNDARY: Cell<usize> = Cell::new(0);
}

fn kmac32(kmac: &dyn Kmac, key: &[u8], label: &str, data: &[u8]) -> Result<[u8; 32], CryptoError> {
    let out = kmac.kmac256(key, label, data, 32);
    if out.len() != 32 {
        return Err(CryptoError::InvalidKey);
    }
    let mut arr = [0u8; 32];
    arr.copy_from_slice(&out);
    Ok(arr)
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
    let pq_target_id = u32::from_be_bytes([
        pq_prefix[0],
        pq_prefix[1],
        pq_prefix[2],
        pq_prefix[3],
    ]);
    let pq_ct = pq_prefix[4..].to_vec();
    Ok(ParsedPqPrefix { pq_target_id, pq_ct })
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
pub fn derive_mk_step(
    kmac: &dyn Kmac,
    ck_ec: &[u8; 32],
    ck_pq: &[u8; 32],
) -> Result<([u8; 32], [u8; 32], [u8; 32]), CryptoError> {
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
        return RecvOutcome { state: st, ok: false, reason: Some("REJECT_S2_LOCAL_UNSUPPORTED"), plaintext: None, pn: None, n: None };
    }
    if hdr_ct.len() != HDR_CT_LEN {
        return RecvOutcome { state: st, ok: false, reason: Some("REJECT_S2_HDR_AUTH_FAIL"), plaintext: None, pn: None, n: None };
    }
    if body_ct.len() < BODY_CT_MIN {
        return RecvOutcome { state: st, ok: false, reason: Some("REJECT_S2_BODY_AUTH_FAIL"), plaintext: None, pn: None, n: None };
    }

    let pq_bind = binding::pq_bind_sha512_32(hash, flags, &[]);
    let ad_hdr = binding::ad_hdr(&st.session_id, st.protocol_version, st.suite_id, &st.dh_pub, flags, &pq_bind);
    let ad_body = binding::ad_body(&st.session_id, st.protocol_version, st.suite_id, &pq_bind);

    // Build candidate N list: MKSKIPPED entries, then a bounded window around Nr.
    let mut candidates: Vec<u32> = st.mkskipped.iter().map(|e| e.n).collect();
    let back_start = st.nr.saturating_sub(MAX_SKIP);
    for n in back_start..st.nr {
        candidates.push(n);
    }
    let mut n = st.nr;
    while n <= st.nr.saturating_add(MAX_SKIP + 1) {
        candidates.push(n);
        n = n.saturating_add(1);
    }

    let mut header_pt: Option<[u8; 8]> = None;
    let mut header_n: u32 = 0;
    let mut header_pn: u32 = 0;
    for cand in candidates.into_iter() {
        let nonce = nonce_hdr(hash, &st.session_id, &st.dh_pub, cand);
        #[cfg(test)]
        S2_HDR_TRY_COUNT_NONBOUNDARY.with(|c| c.set(c.get().saturating_add(1)));
        if let Ok(pt) = aead.open(&st.hk_r, &nonce, &ad_hdr, hdr_ct) {
            if pt.len() == 8 {
                let pn = u32::from_be_bytes([pt[0], pt[1], pt[2], pt[3]]);
                let n_val = u32::from_be_bytes([pt[4], pt[5], pt[6], pt[7]]);
                if n_val == cand {
                    if header_pt.is_none() {
                        header_pt = Some([pt[0], pt[1], pt[2], pt[3], pt[4], pt[5], pt[6], pt[7]]);
                        header_n = n_val;
                        header_pn = pn;
                    }
                }
            }
        }
    }

    if header_pt.is_none() {
        return RecvOutcome { state: st, ok: false, reason: Some("REJECT_S2_HDR_AUTH_FAIL"), plaintext: None, pn: None, n: None };
    }

    // Check for MKSKIPPED hit
    if let Some(pos) = st
        .mkskipped
        .iter()
        .position(|e| e.dh_pub == st.dh_pub && e.n == header_n)
    {
        let mut new_state = st.clone();
        let mk = new_state.mkskipped.remove(pos).mk;
        let nonce = nonce_body(hash, &new_state.session_id, &new_state.dh_pub, header_n);
        match aead.open(&mk, &nonce, &ad_body, body_ct) {
            Ok(pt) => {
                return RecvOutcome { state: new_state, ok: true, reason: None, plaintext: Some(pt), pn: Some(header_pn), n: Some(header_n) };
            }
            Err(_) => {
                return RecvOutcome { state: new_state, ok: false, reason: Some("REJECT_S2_BODY_AUTH_FAIL"), plaintext: None, pn: Some(header_pn), n: Some(header_n) };
            }
        }
    }

    if header_n < st.nr {
        return RecvOutcome { state: st, ok: false, reason: Some("REJECT_S2_REPLAY"), plaintext: None, pn: Some(header_pn), n: Some(header_n) };
    }
    if header_n - st.nr > MAX_SKIP {
        return RecvOutcome { state: st, ok: false, reason: Some("REJECT_S2_OOO_BOUNDS"), plaintext: None, pn: Some(header_pn), n: Some(header_n) };
    }

    // Stage derivations from Nr..=N
    let mut ck_ec = st.ck_ec;
    let mut ck_pq = st.ck_pq;
    let mut staged: Vec<MkSkippedEntry> = Vec::new();
    let mut mk_n: Option<[u8; 32]> = None;

    for i in st.nr..=header_n {
        let (ck_ec_p, ck_pq_p, mk) = match derive_mk_step(kmac, &ck_ec, &ck_pq) {
            Ok(v) => v,
            Err(_) => {
                return RecvOutcome { state: st, ok: false, reason: Some("REJECT_S2_BODY_AUTH_FAIL"), plaintext: None, pn: Some(header_pn), n: Some(header_n) };
            }
        };
        if i < header_n {
            staged.push(MkSkippedEntry { dh_pub: st.dh_pub, n: i, mk });
        } else {
            mk_n = Some(mk);
        }
        ck_ec = ck_ec_p;
        ck_pq = ck_pq_p;
    }

    let mk = match mk_n {
        Some(v) => v,
        None => return RecvOutcome { state: st, ok: false, reason: Some("REJECT_S2_BODY_AUTH_FAIL"), plaintext: None, pn: Some(header_pn), n: Some(header_n) },
    };
    let nonce = nonce_body(hash, &st.session_id, &st.dh_pub, header_n);
    match aead.open(&mk, &nonce, &ad_body, body_ct) {
        Ok(pt) => {
            let mut new_state = st.clone();
            new_state.ck_ec = ck_ec;
            new_state.ck_pq = ck_pq;
            new_state.nr = header_n.saturating_add(1);
            new_state.mkskipped.extend(staged);
            new_state.mkskipped = evict_mkskipped(new_state.mkskipped);
            RecvOutcome { state: new_state, ok: true, reason: None, plaintext: Some(pt), pn: Some(header_pn), n: Some(header_n) }
        }
        Err(_) => RecvOutcome { state: st, ok: false, reason: Some("REJECT_S2_BODY_AUTH_FAIL"), plaintext: None, pn: Some(header_pn), n: Some(header_n) },
    }
}

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
        return BoundaryOutcome { state: st, ok: false, reason: Some("REJECT_S2_LOCAL_UNSUPPORTED"), plaintext: None, pn: None, n: None };
    }

    let parsed = match parse_pq_prefix(flags, pq_prefix) {
        Ok(v) => v,
        Err(code) => {
            return BoundaryOutcome { state: st, ok: false, reason: Some(code), plaintext: None, pn: None, n: None };
        }
    };

    if hdr_ct.len() != HDR_CT_LEN {
        return BoundaryOutcome { state: st, ok: false, reason: Some("REJECT_S2_HDR_AUTH_FAIL"), plaintext: None, pn: None, n: None };
    }
    if body_ct.len() < BODY_CT_MIN {
        return BoundaryOutcome { state: st, ok: false, reason: Some("REJECT_S2_BODY_AUTH_FAIL"), plaintext: None, pn: None, n: None };
    }

    let pq_bind = binding::pq_bind_sha512_32(hash, flags, pq_prefix);
    let ad_hdr = binding::ad_hdr(&st.session_id, st.protocol_version, st.suite_id, &st.dh_pub, flags, &pq_bind);
    let ad_body = binding::ad_body(&st.session_id, st.protocol_version, st.suite_id, &pq_bind);

    let mut header_pt: Option<[u8; 8]> = None;
    let mut n: u32 = 0;
    for i in 0..MAX_HEADER_ATTEMPTS {
        let cand = st.nr.saturating_add(i as u32);
        let nonce_hdr = nonce_hdr(hash, &st.session_id, &st.dh_pub, cand);
        #[cfg(test)]
        S2_HDR_TRY_COUNT_BOUNDARY.with(|c| c.set(c.get().saturating_add(1)));
        if let Ok(pt) = aead.open(&st.hk_r, &nonce_hdr, &ad_hdr, hdr_ct) {
            if pt.len() == 8 {
                let pn = u32::from_be_bytes([pt[0], pt[1], pt[2], pt[3]]);
                let n_val = u32::from_be_bytes([pt[4], pt[5], pt[6], pt[7]]);
                if n_val == cand {
                    if header_pt.is_none() {
                        header_pt = Some([pt[0], pt[1], pt[2], pt[3], pt[4], pt[5], pt[6], pt[7]]);
                        n = n_val;
                        let _ = pn;
                    }
                }
            }
        }
    }
    let header_pt = match header_pt {
        Some(v) => v,
        None => {
            return BoundaryOutcome { state: st, ok: false, reason: Some("REJECT_S2_HDR_AUTH_FAIL"), plaintext: None, pn: None, n: None };
        }
    };
    let header_pn = u32::from_be_bytes([header_pt[0], header_pt[1], header_pt[2], header_pt[3]]);
    if n != st.nr {
        return BoundaryOutcome { state: st, ok: false, reason: Some("REJECT_S2_BOUNDARY_NOT_IN_ORDER"), plaintext: None, pn: Some(header_pn), n: Some(n) };
    }

    let (ck_ec_p, _ck_pq_p, mk) = match derive_mk_step(kmac, &st.ck_ec, &st.ck_pq_recv) {
        Ok(v) => v,
        Err(_) => {
            return BoundaryOutcome { state: st, ok: false, reason: Some("REJECT_S2_BODY_AUTH_FAIL"), plaintext: None, pn: Some(header_pn), n: Some(n) };
        }
    };

    let nonce_body = nonce_body(hash, &st.session_id, &st.dh_pub, n);
    let body_pt = match aead.open(&mk, &nonce_body, &ad_body, body_ct) {
        Ok(pt) => pt,
        Err(_) => {
            return BoundaryOutcome { state: st, ok: false, reason: Some("REJECT_S2_BODY_AUTH_FAIL"), plaintext: None, pn: Some(header_pn), n: Some(n) };
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
            return BoundaryOutcome { state: st, ok: false, reason: Some(code), plaintext: None, pn: Some(header_pn), n: Some(n) };
        }
    };

    let mut new_state = st.clone();
    new_state.ck_ec = ck_ec_p;
    new_state.ck_pq_send = apply.ck_pq_send_after;
    new_state.ck_pq_recv = apply.ck_pq_recv_after;
    new_state.peer_max_adv_id_seen = apply.peer_max_adv_id_seen_after;
    new_state.consumed_targets = apply.consumed_targets_after;
    new_state.tombstoned_targets = apply.tombstoned_targets_after;
    new_state.nr = n.saturating_add(1);

    BoundaryOutcome { state: new_state, ok: true, reason: None, plaintext: Some(body_pt), pn: Some(header_pn), n: Some(n) }
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
    let (ck_ec_p, ck_pq_p, mk) =
        derive_mk_step(kmac, &st.ck_ec, &st.ck_pq).map_err(|_| "REJECT_S2_LOCAL_UNSUPPORTED")?;

    let pq_bind = binding::pq_bind_sha512_32(hash, flags, &[]);
    let ad_hdr = binding::ad_hdr(&st.session_id, st.protocol_version, st.suite_id, &st.dh_pub, flags, &pq_bind);
    let ad_body = binding::ad_body(&st.session_id, st.protocol_version, st.suite_id, &pq_bind);

    let hdr_pt = {
        let mut v = Vec::with_capacity(8);
        v.extend_from_slice(&st.pn.to_be_bytes());
        v.extend_from_slice(&st.ns.to_be_bytes());
        v
    };
    let hdr_ct = aead.seal(&st.hk_s, &nonce_hdr(hash, &st.session_id, &st.dh_pub, st.ns), &ad_hdr, &hdr_pt);
    let body_ct = aead.seal(&mk, &nonce_body(hash, &st.session_id, &st.dh_pub, st.ns), &ad_body, plaintext);
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
    new_state.ns = st.ns.saturating_add(1);

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
        let out = recv_nonboundary_ooo(hash, kmac, aead, recv_state, flags, &parsed.hdr_ct, &parsed.body_ct);
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
        return Err("REJECT_S2_LOCAL_UNSUPPORTED");
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
    Ok(RecvWireOutcome {
        state: new_state,
        plaintext: out.plaintext.unwrap_or_default(),
        flags,
        pn: out.pn.unwrap_or(0),
        n: out.n.unwrap_or(0),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::stdcrypto::StdCrypto;
    use crate::crypto::traits::CryptoError;
    use crate::suite2::types;

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
            session_id: [0x11; 16],
            protocol_version: 5,
            suite_id: 2,
            dh_pub: [0x22; 32],
            hk_r: [0x33; 32],
            rk: [0x44; 32],
            ck_ec: [0x55; 32],
            ck_pq_send: [0x66; 32],
            ck_pq_recv: [0x77; 32],
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

    struct RejectAead;
    impl Aead for RejectAead {
        fn seal(&self, _key32: &[u8; 32], _nonce12: &[u8; 12], _ad: &[u8], _pt: &[u8]) -> Vec<u8> {
            Vec::new()
        }
        fn open(&self, _key32: &[u8; 32], _nonce12: &[u8; 12], _ad: &[u8], _ct: &[u8]) -> Result<Vec<u8>, CryptoError> {
            Err(CryptoError::AuthFail)
        }
    }

    struct AcceptAead;
    impl Aead for AcceptAead {
        fn seal(&self, _key32: &[u8; 32], _nonce12: &[u8; 12], _ad: &[u8], _pt: &[u8]) -> Vec<u8> {
            Vec::new()
        }
        fn open(&self, _key32: &[u8; 32], _nonce12: &[u8; 12], _ad: &[u8], _ct: &[u8]) -> Result<Vec<u8>, CryptoError> {
            Ok(vec![0, 0, 0, 0, 0, 0, 0, 0])
        }
    }

    struct HeaderPtInvalidAead;
    impl Aead for HeaderPtInvalidAead {
        fn seal(&self, _key32: &[u8; 32], _nonce12: &[u8; 12], _ad: &[u8], _pt: &[u8]) -> Vec<u8> {
            Vec::new()
        }
        fn open(&self, _key32: &[u8; 32], _nonce12: &[u8; 12], _ad: &[u8], _ct: &[u8]) -> Result<Vec<u8>, CryptoError> {
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
        let ad_hdr = binding::ad_hdr(&st.session_id, st.protocol_version, st.suite_id, &st.dh_pub, flags, &pq_bind);
        let hdr_pt = {
            let mut v = Vec::with_capacity(8);
            v.extend_from_slice(&0u32.to_be_bytes());
            v.extend_from_slice(&st.nr.to_be_bytes());
            v
        };
        let hdr_ct = c.seal(&st.hk_r, &nonce_hdr(&c, &st.session_id, &st.dh_pub, st.nr), &ad_hdr, &hdr_pt);
        let body_ct = vec![0u8; BODY_CT_MIN];

        let snap_before = snapshot_boundary_state(&st);
        let out1 = recv_boundary_in_order(&c, &c, &c, st.clone(), flags, &pq_prefix, &hdr_ct, &body_ct, &pq_epoch_ss, 1);
        let out2 = recv_boundary_in_order(&c, &c, &c, st.clone(), flags, &pq_prefix, &hdr_ct, &body_ct, &pq_epoch_ss, 1);

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
        let out1 = recv_boundary_in_order(&c, &c, &HeaderPtInvalidAead, st.clone(), flags, &pq_prefix, &hdr_ct, &body_ct, &pq_epoch_ss, 1);
        let out2 = recv_boundary_in_order(&c, &c, &HeaderPtInvalidAead, st.clone(), flags, &pq_prefix, &hdr_ct, &body_ct, &pq_epoch_ss, 1);

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

        let out = recv_boundary_in_order(&c, &c, &HeaderPtInvalidAead, st, flags, &pq_prefix, &hdr_ct, &body_ct, &pq_epoch_ss, 1);
        assert!(!out.ok);
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
        ).expect("apply_pq_reseed");

        let (_ck_ec_p, _ck_pq_p, mk) = derive_mk_step(&c, &st.ck_ec, &st.ck_pq_recv).expect("derive_mk_step");

        let pq_bind = binding::pq_bind_sha512_32(&c, flags, &pq_prefix);
        let ad_hdr = binding::ad_hdr(&st.session_id, st.protocol_version, st.suite_id, &st.dh_pub, flags, &pq_bind);
        let ad_body = binding::ad_body(&st.session_id, st.protocol_version, st.suite_id, &pq_bind);

        let hdr_pt = {
            let mut v = Vec::with_capacity(8);
            v.extend_from_slice(&0u32.to_be_bytes());
            v.extend_from_slice(&st.nr.to_be_bytes());
            v
        };
        let hdr_ct = c.seal(&st.hk_r, &nonce_hdr(&c, &st.session_id, &st.dh_pub, st.nr), &ad_hdr, &hdr_pt);
        let body_ct = c.seal(&mk, &nonce_body(&c, &st.session_id, &st.dh_pub, st.nr), &ad_body, b"ok");

        let out = recv_boundary_in_order(&c, &c, &c, st.clone(), flags, &pq_prefix, &hdr_ct, &body_ct, &pq_epoch_ss, 1);
        assert!(out.ok);
        assert_ne!(st.ck_pq_recv, out.state.ck_pq_recv);
        assert_eq!(apply.ck_pq_recv_after, out.state.ck_pq_recv);
    }

    #[test]
    fn nonboundary_rejects_deterministically_and_no_state_mutation() {
        let c = StdCrypto;
        let st = Suite2RecvState {
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
    fn nonboundary_header_attempts_all_candidates_even_on_first_success() {
        S2_HDR_TRY_COUNT_NONBOUNDARY.with(|c| c.set(0));
        let c = StdCrypto;
        let st = Suite2RecvState {
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
        let flags = 0;
        let hdr_ct = vec![0u8; HDR_CT_LEN];
        let body_ct = vec![0u8; BODY_CT_MIN];
        let out = recv_nonboundary_ooo(&c, &c, &AcceptAead, st.clone(), flags, &hdr_ct, &body_ct);
        assert!(out.ok);
        let expected = (MAX_SKIP + 2) as usize;
        let count = S2_HDR_TRY_COUNT_NONBOUNDARY.with(|c| c.get());
        assert_eq!(count, expected);
    }
}
