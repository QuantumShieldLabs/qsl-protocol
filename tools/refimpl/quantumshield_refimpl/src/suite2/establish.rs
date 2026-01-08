//! Suite-2 establishment helper (base handshake -> initial state).

use std::collections::BTreeSet;

use crate::crypto::traits::{CryptoError, Kmac};
use crate::suite2::ratchet::{Suite2RecvWireState, Suite2SendState};
use crate::suite2::state::Suite2SessionState;

const ZERO32: [u8; 32] = [0u8; 32];

fn kmac32(kmac: &dyn Kmac, key: &[u8], label: &str, data: &[u8]) -> Result<[u8; 32], CryptoError> {
    let out = kmac.kmac256(key, label, data, 32);
    if out.len() != 32 {
        return Err(CryptoError::InvalidKey);
    }
    let mut arr = [0u8; 32];
    arr.copy_from_slice(&out);
    Ok(arr)
}

/// Initialize Suite-2 session state from base-handshake outputs (DOC-CAN-003 §8.2).
pub fn init_from_base_handshake(
    kmac: &dyn Kmac,
    role_is_a: bool,
    protocol_version: u16,
    suite_id: u16,
    session_id: &[u8],
    dh_init: &[u8],
    pq_init_ss: &[u8],
    dh_self_pub: &[u8],
    dh_peer_pub: &[u8],
    authenticated: bool,
) -> Result<Suite2SessionState, &'static str> {
    if session_id.len() != 16
        || dh_init.len() != 32
        || pq_init_ss.len() != 32
        || dh_self_pub.len() != 32
        || dh_peer_pub.len() != 32
    {
        return Err("REJECT_S2_ESTABLISH_BAD_INPUT_LEN");
    }
    if !authenticated {
        return Err("REJECT_S2_ESTABLISH_UNAUTHENTICATED");
    }

    let mut sid = [0u8; 16];
    sid.copy_from_slice(session_id);
    let mut dh_self = [0u8; 32];
    dh_self.copy_from_slice(dh_self_pub);
    let mut dh_peer = [0u8; 32];
    dh_peer.copy_from_slice(dh_peer_pub);

    let mut rk0_data = Vec::with_capacity(16 + 1);
    rk0_data.extend_from_slice(session_id);
    rk0_data.push(0x01);
    let rk0 = kmac32(kmac, dh_init, "QSP5.0/RK0", &rk0_data)
        .map_err(|_| "REJECT_S2_ESTABLISH_BAD_INPUT_LEN")?;

    let mut rk_data = Vec::with_capacity(32 + 1);
    rk_data.extend_from_slice(pq_init_ss);
    rk_data.push(0x01);
    let rk = kmac32(kmac, &rk0, "QSP5.0/RKPQ", &rk_data)
        .map_err(|_| "REJECT_S2_ESTABLISH_BAD_INPUT_LEN")?;

    let hk_a2b = kmac32(kmac, &rk, "QSP5.0/HK/A->B", &[0x01])
        .map_err(|_| "REJECT_S2_ESTABLISH_BAD_INPUT_LEN")?;
    let hk_b2a = kmac32(kmac, &rk, "QSP5.0/HK/B->A", &[0x01])
        .map_err(|_| "REJECT_S2_ESTABLISH_BAD_INPUT_LEN")?;
    let ck0_a2b = kmac32(kmac, &rk, "QSP5.0/CK0/A->B", &[0x01])
        .map_err(|_| "REJECT_S2_ESTABLISH_BAD_INPUT_LEN")?;
    let pq0_a2b = kmac32(kmac, &rk, "QSP5.0/PQ0/A->B", &[0x01])
        .map_err(|_| "REJECT_S2_ESTABLISH_BAD_INPUT_LEN")?;

    let send = if role_is_a {
        Suite2SendState {
            session_id: sid,
            protocol_version,
            suite_id,
            dh_pub: dh_self,
            hk_s: hk_a2b,
            ck_ec: ck0_a2b,
            ck_pq: pq0_a2b,
            ns: 0,
            pn: 0,
        }
    } else {
        Suite2SendState {
            session_id: sid,
            protocol_version,
            suite_id,
            dh_pub: dh_self,
            hk_s: hk_b2a,
            ck_ec: ZERO32,
            ck_pq: ZERO32,
            ns: 0,
            pn: 0,
        }
    };

    let recv = if role_is_a {
        Suite2RecvWireState {
            session_id: sid,
            protocol_version,
            suite_id,
            dh_pub: dh_peer,
            hk_r: hk_b2a,
            rk,
            ck_ec: ZERO32,
            ck_pq_send: pq0_a2b,
            ck_pq_recv: ZERO32,
            nr: 0,
            role_is_a,
            peer_max_adv_id_seen: 0,
            known_targets: BTreeSet::new(),
            consumed_targets: BTreeSet::new(),
            tombstoned_targets: BTreeSet::new(),
            mkskipped: Vec::new(),
        }
    } else {
        Suite2RecvWireState {
            session_id: sid,
            protocol_version,
            suite_id,
            dh_pub: dh_peer,
            hk_r: hk_a2b,
            rk,
            ck_ec: ck0_a2b,
            ck_pq_send: ZERO32,
            ck_pq_recv: pq0_a2b,
            nr: 0,
            role_is_a,
            peer_max_adv_id_seen: 0,
            known_targets: BTreeSet::new(),
            consumed_targets: BTreeSet::new(),
            tombstoned_targets: BTreeSet::new(),
            mkskipped: Vec::new(),
        }
    };

    Ok(Suite2SessionState { send, recv })
}
