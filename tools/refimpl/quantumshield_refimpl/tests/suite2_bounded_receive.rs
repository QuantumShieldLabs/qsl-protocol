use quantumshield_refimpl::crypto::stdcrypto::StdCrypto;
use quantumshield_refimpl::crypto::traits::{Aead, CryptoError};
use quantumshield_refimpl::suite2::ratchet::{
    recv_nonboundary_ooo, send_wire, Suite2RecvState, Suite2SendState,
};
use std::cell::Cell;

const HDR_CT_LEN: usize = 24;
const BODY_CT_MIN: usize = 16;
const MAX_HEADER_ATTEMPTS: usize = 100;

#[derive(Clone)]
struct CountingRejectAead {
    opens: Cell<usize>,
}

impl CountingRejectAead {
    fn new() -> Self {
        Self {
            opens: Cell::new(0),
        }
    }

    fn opens(&self) -> usize {
        self.opens.get()
    }
}

impl Aead for CountingRejectAead {
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
        self.opens.set(self.opens.get().saturating_add(1));
        Err(CryptoError::AuthFail)
    }
}

struct CountingStdAead {
    inner: StdCrypto,
    opens: Cell<usize>,
}

impl CountingStdAead {
    fn new() -> Self {
        Self {
            inner: StdCrypto,
            opens: Cell::new(0),
        }
    }

    fn opens(&self) -> usize {
        self.opens.get()
    }
}

impl Aead for CountingStdAead {
    fn seal(&self, key32: &[u8; 32], nonce12: &[u8; 12], ad: &[u8], pt: &[u8]) -> Vec<u8> {
        self.inner.seal(key32, nonce12, ad, pt)
    }

    fn open(
        &self,
        key32: &[u8; 32],
        nonce12: &[u8; 12],
        ad: &[u8],
        ct: &[u8],
    ) -> Result<Vec<u8>, CryptoError> {
        self.opens.set(self.opens.get().saturating_add(1));
        self.inner.open(key32, nonce12, ad, ct)
    }
}

fn recv_state() -> Suite2RecvState {
    fn derive_arr16(seed: u8) -> [u8; 16] {
        std::array::from_fn(|i| seed.wrapping_add(i as u8).rotate_left(1))
    }
    fn derive_arr32(seed: u8) -> [u8; 32] {
        std::array::from_fn(|i| seed.wrapping_add(i as u8).rotate_left(1))
    }

    Suite2RecvState {
        session_id: derive_arr16(0x11),
        protocol_version: 5,
        suite_id: 2,
        dh_pub: derive_arr32(0x22),
        hk_r: derive_arr32(0x33),
        ck_ec: derive_arr32(0x44),
        ck_pq: derive_arr32(0x55),
        nr: 0,
        mkskipped: Vec::new(),
    }
}

fn snapshot_recv(st: &Suite2RecvState) -> Vec<u8> {
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
    for e in st.mkskipped.iter() {
        out.extend_from_slice(&e.dh_pub);
        out.extend_from_slice(&e.n.to_be_bytes());
        out.extend_from_slice(&e.mk);
    }
    out
}

fn split_wire_for_nonboundary(wire: &[u8]) -> (&[u8], &[u8]) {
    let header_offset = 10usize;
    let hdr_ct_start = header_offset + 32 + 2;
    let hdr_ct_end = hdr_ct_start + HDR_CT_LEN;
    (&wire[hdr_ct_start..hdr_ct_end], &wire[hdr_ct_end..])
}

#[test]
fn recv_invalid_message_caps_header_attempts() {
    let c = StdCrypto;
    let aead = CountingRejectAead::new();
    let st = recv_state();
    let pre = snapshot_recv(&st);
    let hdr_ct = vec![0u8; HDR_CT_LEN];
    let body_ct = vec![0u8; BODY_CT_MIN];

    let out = recv_nonboundary_ooo(&c, &c, &aead, st, 0, &hdr_ct, &body_ct);

    assert!(!out.ok);
    assert_eq!(out.reason, Some("REJECT_S2_HDR_AUTH_FAIL"));
    assert!(aead.opens() <= MAX_HEADER_ATTEMPTS);
    assert_eq!(pre, snapshot_recv(&out.state));
}

#[test]
fn recv_far_future_message_fails_fast_without_mutation() {
    let c = StdCrypto;
    let recv = recv_state();
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

    let mut far_wire = Vec::new();
    for i in 0..(MAX_HEADER_ATTEMPTS + 2) {
        let out = send_wire(&c, &c, &c, send.clone(), 0, b"x").expect("send wire");
        send = out.state;
        if i == MAX_HEADER_ATTEMPTS + 1 {
            far_wire = out.wire;
        }
    }

    let (hdr_ct, body_ct) = split_wire_for_nonboundary(&far_wire);
    let aead = CountingStdAead::new();
    let pre = snapshot_recv(&recv);
    let out = recv_nonboundary_ooo(&c, &c, &aead, recv, 0, hdr_ct, body_ct);

    assert!(!out.ok);
    assert_eq!(out.reason, Some("REJECT_S2_HDR_AUTH_FAIL"));
    assert!(aead.opens() <= MAX_HEADER_ATTEMPTS);
    assert_eq!(pre, snapshot_recv(&out.state));
}

#[test]
fn recv_in_order_message_uses_small_attempt_count() {
    let c = StdCrypto;
    let recv = recv_state();
    let send = Suite2SendState {
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

    let wire = send_wire(&c, &c, &c, send, 0, b"hello")
        .expect("send wire")
        .wire;
    let (hdr_ct, body_ct) = split_wire_for_nonboundary(&wire);
    let aead = CountingStdAead::new();

    let out = recv_nonboundary_ooo(&c, &c, &aead, recv, 0, hdr_ct, body_ct);
    assert!(out.ok);
    assert_eq!(out.plaintext.as_deref(), Some(b"hello".as_slice()));
    assert!(aead.opens() <= 3);
}
