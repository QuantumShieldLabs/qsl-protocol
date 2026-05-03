use quantumshield_refimpl::crypto::stdcrypto::StdCrypto;
use quantumshield_refimpl::crypto::traits::{Aead, CryptoError};
use quantumshield_refimpl::suite2::ratchet::{
    MkSkippedEntry, Suite2RecvWireState, Suite2SendState,
};
use quantumshield_refimpl::suite2::state::Suite2SessionState;
use quantumshield_refimpl::suite2::{recv_wire_canon, send_wire_canon, types};
use quantumshield_refimpl::RefimplError;
use std::collections::BTreeSet;

const HDR_CT_LEN: usize = 24;
const BODY_CT_MIN: usize = 16;

struct SkippedHeaderBodyRejectAead {
    pn: u32,
    n: u32,
}

impl Aead for SkippedHeaderBodyRejectAead {
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

fn arr16(seed: u8) -> [u8; 16] {
    std::array::from_fn(|i| seed.wrapping_add((i as u8).wrapping_mul(3)).rotate_left(1))
}

fn arr32(seed: u8) -> [u8; 32] {
    std::array::from_fn(|i| seed.wrapping_add((i as u8).wrapping_mul(5)).rotate_left(1))
}

fn base_session() -> Suite2SessionState {
    let session_id = arr16(0x11);
    let dh_pub = arr32(0x21);
    let hk = arr32(0x31);
    let ck_ec = arr32(0x41);
    let ck_pq = arr32(0x51);

    Suite2SessionState {
        send: Suite2SendState {
            session_id,
            protocol_version: types::SUITE2_PROTOCOL_VERSION,
            suite_id: types::SUITE2_SUITE_ID,
            dh_pub,
            hk_s: hk,
            ck_ec,
            ck_pq,
            ns: 0,
            pn: 0,
        },
        recv: Suite2RecvWireState {
            session_id,
            protocol_version: types::SUITE2_PROTOCOL_VERSION,
            suite_id: types::SUITE2_SUITE_ID,
            dh_pub,
            hk_r: hk,
            rk: arr32(0x61),
            ck_ec,
            ck_pq_send: arr32(0x71),
            ck_pq_recv: ck_pq,
            nr: 0,
            role_is_a: true,
            peer_max_adv_id_seen: 0,
            known_targets: BTreeSet::new(),
            consumed_targets: BTreeSet::new(),
            tombstoned_targets: BTreeSet::new(),
            mkskipped: Vec::new(),
        },
    }
}

fn nonboundary_wire(dh_pub: &[u8; 32], hdr_ct: &[u8], body_ct: &[u8]) -> Vec<u8> {
    let mut header = Vec::with_capacity(32 + 2 + hdr_ct.len());
    header.extend_from_slice(dh_pub);
    header.extend_from_slice(&0u16.to_be_bytes());
    header.extend_from_slice(hdr_ct);

    let mut wire = Vec::with_capacity(10 + header.len() + body_ct.len());
    wire.extend_from_slice(&types::SUITE2_PROTOCOL_VERSION.to_be_bytes());
    wire.extend_from_slice(&types::SUITE2_SUITE_ID.to_be_bytes());
    wire.push(0x02);
    wire.push(0x00);
    wire.extend_from_slice(&(header.len() as u16).to_be_bytes());
    wire.extend_from_slice(&(body_ct.len() as u16).to_be_bytes());
    wire.extend_from_slice(&header);
    wire.extend_from_slice(body_ct);
    wire
}

fn try_recv_into_session(
    crypto: &StdCrypto,
    aead: &dyn Aead,
    session: &mut Suite2SessionState,
    wire: &[u8],
) -> Result<Vec<u8>, RefimplError> {
    let out = recv_wire_canon(crypto, crypto, aead, session.recv.clone(), wire, None, None)?;
    session.recv = out.state;
    Ok(out.plaintext)
}

#[test]
fn skipped_key_body_auth_reject_does_not_consume_skipped_key_or_mutate_snapshot() {
    let crypto = StdCrypto;
    let mut session = base_session();
    session.recv.mkskipped.push(MkSkippedEntry {
        dh_pub: session.recv.dh_pub,
        n: 5,
        mk: arr32(0x81),
    });

    let wire = nonboundary_wire(
        &session.recv.dh_pub,
        &[0xA5; HDR_CT_LEN],
        &[0x5A; BODY_CT_MIN],
    );
    let aead = SkippedHeaderBodyRejectAead { pn: 0, n: 5 };
    let before = session.snapshot_bytes();

    let err1 = try_recv_into_session(&crypto, &aead, &mut session, &wire)
        .expect_err("skipped-key body auth failure must reject");
    let after_first = session.snapshot_bytes();
    let err2 = try_recv_into_session(&crypto, &aead, &mut session, &wire)
        .expect_err("skipped-key body auth failure must reject deterministically");

    assert_eq!(err1.code(), "REJECT_S2_BODY_AUTH_FAIL");
    assert_eq!(err1, err2);
    assert_eq!(before, after_first);
    assert_eq!(before, session.snapshot_bytes());
    assert_eq!(session.recv.mkskipped.len(), 1);
    assert_eq!(session.recv.mkskipped[0].n, 5);
}

#[test]
fn receive_body_auth_reject_does_not_advance_or_mutate_session_snapshot() {
    let crypto = StdCrypto;
    let mut session = base_session();
    let send = send_wire_canon(&crypto, &crypto, &crypto, session.send.clone(), 0, b"hello")
        .expect("send wire");
    let mut tampered = send.wire;
    let last = tampered
        .last_mut()
        .expect("wire from send_wire has a body ciphertext");
    *last ^= 0x80;

    let before = session.snapshot_bytes();
    let err1 = try_recv_into_session(&crypto, &crypto, &mut session, &tampered)
        .expect_err("tampered receive body must reject");
    let after_first = session.snapshot_bytes();
    let err2 = try_recv_into_session(&crypto, &crypto, &mut session, &tampered)
        .expect_err("tampered receive body must reject deterministically");

    assert_eq!(err1.code(), "REJECT_S2_BODY_AUTH_FAIL");
    assert_eq!(err1, err2);
    assert_eq!(before, after_first);
    assert_eq!(before, session.snapshot_bytes());
    assert_eq!(session.recv.nr, 0);
    assert!(session.recv.mkskipped.is_empty());
}
