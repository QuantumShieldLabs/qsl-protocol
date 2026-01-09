use super::{ProtocolMessage, SessionState, HeaderSource, SessionRole};
use super::constants::*;
use super::state::derive_header_keys_kmac;
use crate::crypto::traits::*;
use crate::codec::CodecError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RatchetError {
    #[error("codec: {0}")]
    Codec(#[from] CodecError),
    #[error("crypto: {0}")]
    Crypto(#[from] CryptoError),
    #[error("invalid: {0}")]
    Invalid(&'static str),
}

fn kmac32(kmac: &dyn Kmac, key: &[u8;32], label: &str, data: &[u8]) -> [u8;32] {
    let v = kmac.kmac256(key, label, data, 32);
    let mut out = [0u8;32];
    out.copy_from_slice(&v);
    out
}

fn kdf_ck(kmac: &dyn Kmac, ck: &[u8;32]) -> ([u8;32],[u8;32]) {
    // QSP §3.3
    let ck1 = kmac32(kmac, ck, "QSP4.3/CK", &[0x01]);
    let mk  = kmac32(kmac, ck, "QSP4.3/MK", &[0x02]);
    (ck1, mk)
}

fn kdf_rk_dh(kmac: &dyn Kmac, rk: &[u8;32], dh_out: &[u8;32]) -> ([u8;32],[u8;32]) {
    // QSP §3.3
    let tmp = kmac.kmac256(rk, "QSP4.3/RKDH", dh_out, 64);
    let mut rk1 = [0u8;32];
    let mut ck = [0u8;32];
    rk1.copy_from_slice(&tmp[0..32]);
    ck.copy_from_slice(&tmp[32..64]);
    (rk1, ck)
}

fn kdf_rk_pq(kmac: &dyn Kmac, rk: &[u8;32], pq_ss: &[u8]) -> [u8;32] {
    let v = kmac.kmac256(rk, "QSP4.3/RKPQ", pq_ss, 32);
    let mut out = [0u8;32];
    out.copy_from_slice(&v);
    out
}

fn ad_hdr(session_id: &[u8;16], protocol_version: u16, suite_id: u16, dh_pub: &[u8;32], flags: u16) -> Vec<u8> {
    let mut ad = Vec::with_capacity(16+2+2+32+2);
    ad.extend_from_slice(session_id);
    ad.extend_from_slice(&protocol_version.to_be_bytes());
    ad.extend_from_slice(&suite_id.to_be_bytes());
    ad.extend_from_slice(dh_pub);
    ad.extend_from_slice(&flags.to_be_bytes());
    ad
}

fn ad_body(session_id: &[u8;16], protocol_version: u16, suite_id: u16) -> Vec<u8> {
    let mut ad = Vec::with_capacity(16+2+2);
    ad.extend_from_slice(session_id);
    ad.extend_from_slice(&protocol_version.to_be_bytes());
    ad.extend_from_slice(&suite_id.to_be_bytes());
    ad
}

fn nonce_body(hash: &dyn Hash, session_id: &[u8;16], dh_pub: &[u8;32], n: u32) -> [u8;12] {
    // QSP §6.2
    let mut m = b"QSP4.3/BODY-NONCE".to_vec();
    m.extend_from_slice(session_id);
    m.extend_from_slice(dh_pub);
    m.extend_from_slice(&n.to_be_bytes());
    let h = hash.sha512(&m);
    let mut out = [0u8;12];
    out.copy_from_slice(&h[0..12]);
    out
}

/// QSP §9.1
pub fn dh_ratchet_send(st: &mut SessionState, kmac: &dyn Kmac, dh: &dyn X25519Dh) {
    let boundary_hk = st.nhk_s; // pre-ratchet
    st.pn = st.ns;
    st.ns = 0;

    st.dh_self = dh.keypair();
    let dh_out = dh.dh(&st.dh_self.0, &X25519Pub(st.dh_peer));
    let (rk1, ck_s) = kdf_rk_dh(kmac, &st.rk, &dh_out);
    st.rk = rk1;
    st.ck_s = Some(ck_s);

    // recompute header keys from new RK (QSP §3.4)
    let (hk_s,hk_r,nhk_s,nhk_r) = derive_header_keys_kmac(st.role, &st.rk, kmac);
    st.hk_s = hk_s; st.hk_r = hk_r; st.nhk_s = nhk_s; st.nhk_r = nhk_r;

    st.boundary_pending = true;
    st.boundary_hk = boundary_hk;
}

/// QSP §9.2
pub fn dh_ratchet_receive(
    st: &mut SessionState,
    kmac: &dyn Kmac,
    dh: &dyn X25519Dh,
    dh_new: [u8; 32],
    pn: u32,
) -> Result<(), RatchetError> {
    // 1) if ck_r exists: derive skipped keys up to PN (bounded by MAX_SKIP via caller)
    if let Some(mut ck_r) = st.ck_r {
        while st.nr < pn {
            let (ck1, mk) = kdf_ck(kmac, &ck_r);
            ck_r = ck1;
            st.store_mk_skipped(st.dh_peer, st.nr, mk);
            st.nr = checked_inc_nr(st.nr, "nr overflow in skip loop")?;
        }
        st.ck_r = Some(ck_r);
    }

    // 2) store HKSKIPPED for old dh_peer
    st.store_hk_skipped(st.dh_peer, st.hk_r, st.nhk_r);

    // 3) update PN and reset Ns
    st.pn = st.ns;
    st.ns = 0;

    // 4) set DH_peer
    st.dh_peer = dh_new;

    // 5) dh_in
    let dh_in = dh.dh(&st.dh_self.0, &X25519Pub(st.dh_peer));
    let (rk1, ck_r) = kdf_rk_dh(kmac, &st.rk, &dh_in);
    st.rk = rk1;
    st.ck_r = Some(ck_r);

    // 7) recompute header keys
    let (hk_s,hk_r,nhk_s,nhk_r) = derive_header_keys_kmac(st.role, &st.rk, kmac);
    st.hk_s = hk_s; st.hk_r = hk_r; st.nhk_s = nhk_s; st.nhk_r = nhk_r;

    // 8) force send ratchet next outbound
    st.ck_s = None;

    Ok(())
}

/// QSP §9.4
pub fn header_decrypt(
    st: &SessionState,
    aead: &dyn Aead,
    msg: &ProtocolMessage,
) -> Result<(u32,u32,HeaderSource), CryptoError> {
    // HeaderPlain = PN(u32) || N(u32) (QSP §7.2)
    // Try HK_r, then NHK_r, then HKSKIPPED keyed by msg.dh_pub
    let ad = ad_hdr(&msg.session_id, msg.protocol_version, msg.suite_id, &msg.dh_pub, msg.flags);
    let nonce = &msg.nonce_hdr;

    let mut attempts = 0usize;
    let mut try_key = |k: &[u8;32], src: HeaderSource| -> Option<(u32,u32,HeaderSource)> {
        attempts += 1;
        if attempts > MAX_HEADER_ATTEMPTS { return None; }
        let pt = aead.open(k, nonce, &ad, &msg.hdr_ct).ok()?;
        if pt.len() != 8 { return None; }
        let pn = u32::from_be_bytes([pt[0],pt[1],pt[2],pt[3]]);
        let n  = u32::from_be_bytes([pt[4],pt[5],pt[6],pt[7]]);
        Some((pn,n,src))
    };

    if let Some(v) = try_key(&st.hk_r, HeaderSource::CurrentHk) { return Ok(v); }
    if let Some(v) = try_key(&st.nhk_r, HeaderSource::CurrentNhk) { return Ok(v); }

    if let Some((hk_old, nhk_old)) = st.hk_pair_for(&msg.dh_pub) {
        if let Some(v) = try_key(&hk_old, HeaderSource::SkippedHk) { return Ok(v); }
        if let Some(v) = try_key(&nhk_old, HeaderSource::SkippedNhk) { return Ok(v); }
    }

    Err(CryptoError::AuthFail)
}

/// QSP §9.3 (RatchetEncrypt) + PQ options.
pub fn ratchet_encrypt(
    st: &mut SessionState,
    hash: &dyn Hash,
    kmac: &dyn Kmac,
    aead: &dyn Aead,
    dh: &dyn X25519Dh,
    pq_kem: &dyn PqKem768,
    mut rng: &mut dyn Rng12,
    plaintext: &[u8],
    request_pq_mix: bool,
    request_pq_adv: bool,
) -> Result<ProtocolMessage, RatchetError> {
    if st.ck_s.is_none() {
        dh_ratchet_send(st, kmac, dh);
    }

    let n = st.ns;
    let ck_s = st.ck_s.ok_or(RatchetError::Invalid("ck_s missing"))?;
    let (ck1, mk) = kdf_ck(kmac, &ck_s);
    st.ck_s = Some(ck1);
    st.ns = st.ns.checked_add(1).ok_or(RatchetError::Invalid("ns overflow"))?;

    let hp = [st.pn.to_be_bytes(), n.to_be_bytes()].concat();

    // choose header key
    let mut flags = 0u16;
    let hk_hdr = if st.boundary_pending {
        flags |= FLAG_BOUNDARY;
        st.boundary_pending = false;
        st.boundary_hk
    } else {
        st.hk_s
    };

    // PQ fields
    let mut pq_target_id = None;
    let mut pq_ct = None;
    let mut pq_adv_id = None;
    let mut pq_adv_pub = None;

    // PQ_CTXT: only meaningful on boundary sends (policy decision). This skeleton gates on request_pq_mix.
    if request_pq_mix {
        if let (Some(peer_id), Some(peer_pub)) = (st.pq_peer_id, st.pq_peer_pub.as_ref()) {
            let (ct, ss) = pq_kem.encap(peer_pub)?;
            flags |= FLAG_PQ_CTXT;
            pq_target_id = Some(peer_id);
            pq_ct = Some(ct);

            // Update RK immediately after constructing the message (sender-side). Receiver updates after decrypt (QSP §9.5 step 10).
            st.rk = kdf_rk_pq(kmac, &st.rk, &ss);
            let (hk_s,hk_r,nhk_s,nhk_r) = derive_header_keys_kmac(st.role, &st.rk, kmac);
            st.hk_s = hk_s; st.hk_r = hk_r; st.nhk_s = nhk_s; st.nhk_r = nhk_r;
        }
    }

    // PQ_ADV: advertise a (new) PQ receive key for the peer to use on its next boundary.
    if request_pq_adv {
        // Choose the lowest pq_self id for determinism; production should use rotation.
        if let Some((&id, (pubk,_))) = st.pq_self.iter().min_by_key(|(k,_)| *k) {
            flags |= FLAG_PQ_ADV;
            pq_adv_id = Some(id);
            pq_adv_pub = Some(pubk.clone());
        }
    }

    // nonces + AD
    let nonce_hdr = rng.random_nonce12();
    let ad_h = ad_hdr(&st.session_id, QSP_PROTOCOL_VERSION, QSP_SUITE_ID, &st.dh_self.1 .0, flags);
    let hdr_ct = aead.seal(&hk_hdr, &nonce_hdr, &ad_h, &hp);

    let nb = nonce_body(hash, &st.session_id, &st.dh_self.1 .0, n);
    let ad_b = ad_body(&st.session_id, QSP_PROTOCOL_VERSION, QSP_SUITE_ID);
    let body_ct = aead.seal(&mk, &nb, &ad_b, plaintext);

    Ok(ProtocolMessage {
        protocol_version: QSP_PROTOCOL_VERSION,
        suite_id: QSP_SUITE_ID,
        session_id: st.session_id,
        dh_pub: st.dh_self.1 .0,
        flags,
        nonce_hdr,
        pq_adv_id,
        pq_adv_pub,
        pq_target_id,
        pq_ct,
        hdr_ct,
        body_ct,
    })
}

/// QSP §9.5 (RatchetDecrypt complete)
pub fn ratchet_decrypt(
    st: &mut SessionState,
    hash: &dyn Hash,
    kmac: &dyn Kmac,
    aead: &dyn Aead,
    dh: &dyn X25519Dh,
    pq_kem: &dyn PqKem768,
    msg: &ProtocolMessage,
) -> Result<Vec<u8>, RatchetError> {
    // 1) prefix validation already done in ProtocolMessage::decode; enforce here as defense-in-depth
    if msg.protocol_version != QSP_PROTOCOL_VERSION { return Err(RatchetError::Invalid("protocol_version")); }
    if msg.suite_id != QSP_SUITE_ID { return Err(RatchetError::Invalid("suite_id")); }

    // 2) work on a copy
    let mut tmp = st.clone();

    // 3) decrypt header
    let (pn, n, hdr_src) = header_decrypt(&tmp, aead, msg).map_err(RatchetError::Crypto)?;

    // Determine epoch handling (QSP §9.5 step 4)
    let mut old_epoch_delayed = false;
    if msg.dh_pub != tmp.dh_peer {
        if tmp.hk_pair_for(&msg.dh_pub).is_some() || matches!(hdr_src, HeaderSource::SkippedHk | HeaderSource::SkippedNhk) {
            old_epoch_delayed = true;
        } else {
            // new epoch: must be boundary header under CURRENT_NHK
            if hdr_src != HeaderSource::CurrentNhk { return Err(RatchetError::Invalid("boundary header not under CURRENT_NHK")); }
            // ratchet receive
            // bound skipped derivation to PN
            if pn.saturating_sub(tmp.nr) > MAX_SKIP { return Err(RatchetError::Invalid("MAX_SKIP exceeded on PN")); }
            dh_ratchet_receive(&mut tmp, kmac, dh, msg.dh_pub, pn)?;
        }
    }

    // 5/6) MKSKIPPED lookup
    if let Some(mk) = tmp.take_mk_skipped(msg.dh_pub, n) {
        let nb = nonce_body(hash, &tmp.session_id, &msg.dh_pub, n);
        let ad_b = ad_body(&tmp.session_id, msg.protocol_version, msg.suite_id);
        let pt = aead.open(&mk, &nb, &ad_b, &msg.body_ct)?;
        *st = tmp; // commit
        return Ok(pt);
    }

    if old_epoch_delayed {
        // QSP §9.5: if delayed old-epoch message and no MKSKIPPED entry exists, drop.
        return Err(RatchetError::Invalid("old-epoch message without MKSKIPPED"));
    }

    // 7) enforce MAX_SKIP for current epoch
    if n.saturating_sub(tmp.nr) > MAX_SKIP { return Err(RatchetError::Invalid("MAX_SKIP exceeded")); }

    // 7) derive and store skipped message keys up to N
    if let Some(mut ck_r) = tmp.ck_r {
        while tmp.nr < n {
            let (ck1, mki) = kdf_ck(kmac, &ck_r);
            ck_r = ck1;
            tmp.store_mk_skipped(tmp.dh_peer, tmp.nr, mki);
            tmp.nr = checked_inc_nr(tmp.nr, "nr overflow in skip loop")?;
        }
        tmp.ck_r = Some(ck_r);
    } else {
        // No receiving chain yet – protocol requires DH receive ratchet to set CK_r.
        return Err(RatchetError::Invalid("ck_r missing"));
    }

    // 8) derive MK for this message
    let ck_r = tmp.ck_r.ok_or(RatchetError::Invalid("ck_r missing"))?;
    let (ck1, mk) = kdf_ck(kmac, &ck_r);
    tmp.ck_r = Some(ck1);
    tmp.nr = tmp.nr.checked_add(1).ok_or(RatchetError::Invalid("nr overflow"))?;

    // 9) decrypt body
    let nb = nonce_body(hash, &tmp.session_id, &msg.dh_pub, n);
    let ad_b = ad_body(&tmp.session_id, msg.protocol_version, msg.suite_id);
    let pt = aead.open(&mk, &nb, &ad_b, &msg.body_ct)?;

    // 10) PQ_CTXT mixing
    if (msg.flags & FLAG_PQ_CTXT) != 0 {
        let target = msg.pq_target_id.ok_or(RatchetError::Invalid("missing pq_target_id"))?;
        let ct = msg.pq_ct.as_ref().ok_or(RatchetError::Invalid("missing pq_ct"))?;
        let (_pubk, privk) = tmp.pq_self.get(&target).ok_or(RatchetError::Invalid("unknown pq_target_id"))?.clone();
        let pq_ss = pq_kem.decap(&privk, ct)?;
        tmp.rk = kdf_rk_pq(kmac, &tmp.rk, &pq_ss);
        let (hk_s,hk_r,nhk_s,nhk_r) = derive_header_keys_kmac(tmp.role, &tmp.rk, kmac);
        tmp.hk_s = hk_s; tmp.hk_r = hk_r; tmp.nhk_s = nhk_s; tmp.nhk_r = nhk_r;
    }

    // 11) PQ_ADV update
    if (msg.flags & FLAG_PQ_ADV) != 0 {
        let id = msg.pq_adv_id.ok_or(RatchetError::Invalid("missing pq_adv_id"))?;
        let pubk = msg.pq_adv_pub.as_ref().ok_or(RatchetError::Invalid("missing pq_adv_pub"))?.clone();
        tmp.pq_peer_id = Some(id);
        tmp.pq_peer_pub = Some(pubk);
    }

    // 12) commit
    *st = tmp;
    Ok(pt)
}

fn checked_inc_nr(nr: u32, err: &'static str) -> Result<u32, RatchetError> {
    nr.checked_add(1).ok_or(RatchetError::Invalid(err))
}

#[cfg(test)]
mod tests {
    use super::{checked_inc_nr, RatchetError};

    #[test]
    fn checked_inc_nr_overflow_rejects() {
        let err = checked_inc_nr(u32::MAX, "nr overflow in skip loop");
        assert!(matches!(err, Err(RatchetError::Invalid(_))));
    }
}
