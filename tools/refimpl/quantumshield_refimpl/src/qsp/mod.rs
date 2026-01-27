//! QuantumShield Protocol (QSP) v4.3.1

mod constants;
mod handshake;
mod ratchet;
mod state;
mod types;

pub use constants::*;
pub use handshake::*;
pub use ratchet::*;
pub use state::*;
pub use types::*;

use crate::crypto::traits::{Kmac, X25519Dh};
use crate::RefimplError;

pub fn decode_prekey_bundle_canon(buf: &[u8]) -> Result<PrekeyBundle, RefimplError> {
    PrekeyBundle::decode(buf).map_err(RefimplError::from)
}

pub fn decode_handshake_init_canon(buf: &[u8]) -> Result<HandshakeInit, RefimplError> {
    HandshakeInit::decode(buf).map_err(RefimplError::from)
}

pub fn decode_handshake_resp_canon(buf: &[u8]) -> Result<HandshakeResp, RefimplError> {
    HandshakeResp::decode(buf).map_err(RefimplError::from)
}

pub fn decode_protocol_message_canon(buf: &[u8]) -> Result<ProtocolMessage, RefimplError> {
    ProtocolMessage::decode(buf).map_err(RefimplError::from)
}

pub fn dh_ratchet_send_canon(
    st: &mut SessionState,
    kmac: &dyn Kmac,
    dh: &dyn X25519Dh,
) -> Result<(), RefimplError> {
    ratchet::dh_ratchet_send(st, kmac, dh).map_err(RefimplError::from)
}

#[cfg(test)]
mod issue25_qsp_tests {
    use super::*;
    use crate::crypto::traits::{Kmac, X25519Dh};
    use crate::crypto::traits::{X25519Priv, X25519Pub};
    use rand::rngs::OsRng;
    use rand::RngCore;

    struct DummyKmac;
    impl Kmac for DummyKmac {
        fn kmac256(&self, _key: &[u8], _label: &str, _data: &[u8], outlen: usize) -> Vec<u8> {
            let mut v = vec![0u8; outlen];
            OsRng.fill_bytes(&mut v);
            v
        }
    }

    struct DummyDh;
    impl X25519Dh for DummyDh {
        fn keypair(&self) -> (X25519Priv, X25519Pub) {
            let privk = X25519Priv(rand32());
            let pubk = X25519Pub(rand32());
            (privk, pubk)
        }
        fn dh(&self, _privk: &X25519Priv, _pubk: &X25519Pub) -> [u8; 32] {
            rand32()
        }
    }

    fn rand32() -> [u8; 32] {
        use core::mem::MaybeUninit;
        let mut out = MaybeUninit::<[u8; 32]>::uninit();
        let buf = unsafe { &mut *out.as_mut_ptr() };
        OsRng.fill_bytes(&mut buf[..]);
        unsafe { out.assume_init() }
    }

    fn rand16() -> [u8; 16] {
        use core::mem::MaybeUninit;
        let mut out = MaybeUninit::<[u8; 16]>::uninit();
        let buf = unsafe { &mut *out.as_mut_ptr() };
        OsRng.fill_bytes(&mut buf[..]);
        unsafe { out.assume_init() }
    }

    fn rand_vec32() -> Vec<u8> {
        rand32().to_vec()
    }

    fn base_state() -> SessionState {
        let kmac = DummyKmac;
        SessionState::new(
            SessionRole::Initiator,
            rand16(),
            rand32(),
            &kmac,
            (X25519Priv(rand32()), X25519Pub(rand32())),
            rand32(),
            (1u32, rand_vec32(), rand_vec32()),
        )
    }

    #[test]
    fn dh_ratchet_send_canon_rejects_deterministically_and_no_mutation() {
        let kmac = DummyKmac;
        let dh = DummyDh;

        let mut st = base_state();
        st.ns = u32::MAX;
        let before = st.snapshot_bytes();

        let err1 = dh_ratchet_send_canon(&mut st, &kmac, &dh).unwrap_err();
        let msg1 = err1.to_string();
        assert!(
            msg1.contains("reason_code=REJECT_QSP_RATCHET_ERROR"),
            "missing reason_code token: {msg1}"
        );
        assert_eq!(st.snapshot_bytes(), before, "state mutated on reject");

        let mut st2 = base_state();
        st2.ns = u32::MAX;
        let err2 = dh_ratchet_send_canon(&mut st2, &kmac, &dh).unwrap_err();
        assert_eq!(msg1, err2.to_string(), "reject not deterministic");
    }
}
