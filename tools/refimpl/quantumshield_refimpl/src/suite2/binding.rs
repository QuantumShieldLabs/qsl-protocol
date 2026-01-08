//! Suite-2 transcript / AD binding helpers.

use crate::crypto::traits::Hash;

const PQ_BIND_LABEL: &[u8] = b"QSP5.0/PQ-BIND";

pub fn pq_bind_sha512_32(hash: &dyn Hash, flags: u16, pq_prefix: &[u8]) -> [u8; 32] {
    let mut m = Vec::with_capacity(PQ_BIND_LABEL.len() + 2 + pq_prefix.len());
    m.extend_from_slice(PQ_BIND_LABEL);
    m.extend_from_slice(&flags.to_be_bytes());
    m.extend_from_slice(pq_prefix);
    let full = hash.sha512(&m);
    let mut out = [0u8; 32];
    out.copy_from_slice(&full[..32]);
    out
}

pub fn ad_hdr(
    session_id: &[u8],
    protocol_version: u16,
    suite_id: u16,
    dh_pub: &[u8],
    flags: u16,
    pq_bind: &[u8],
) -> Vec<u8> {
    let mut ad = Vec::with_capacity(session_id.len() + 2 + 2 + dh_pub.len() + 2 + pq_bind.len());
    ad.extend_from_slice(session_id);
    ad.extend_from_slice(&protocol_version.to_be_bytes());
    ad.extend_from_slice(&suite_id.to_be_bytes());
    ad.extend_from_slice(dh_pub);
    ad.extend_from_slice(&flags.to_be_bytes());
    ad.extend_from_slice(pq_bind);
    ad
}

pub fn ad_body(
    session_id: &[u8],
    protocol_version: u16,
    suite_id: u16,
    pq_bind: &[u8],
) -> Vec<u8> {
    let mut ad = Vec::with_capacity(session_id.len() + 2 + 2 + pq_bind.len());
    ad.extend_from_slice(session_id);
    ad.extend_from_slice(&protocol_version.to_be_bytes());
    ad.extend_from_slice(&suite_id.to_be_bytes());
    ad.extend_from_slice(pq_bind);
    ad
}
