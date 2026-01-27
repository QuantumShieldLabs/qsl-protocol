use quantumshield_refimpl::crypto::traits::{
    Aead, CryptoError, Hash, Kmac, PqKem768, Rng12, X25519Dh, X25519Priv, X25519Pub,
};
use quantumshield_refimpl::qsp::{ratchet_decrypt, ratchet_encrypt};
use quantumshield_refimpl::{SessionRole, SessionState};

struct TestKmac;
impl Kmac for TestKmac {
    fn kmac256(&self, key: &[u8], label: &str, data: &[u8], outlen: usize) -> Vec<u8> {
        let mut src = Vec::with_capacity(key.len() + label.len() + data.len());
        src.extend_from_slice(key);
        src.extend_from_slice(label.as_bytes());
        src.extend_from_slice(data);
        let mut out = vec![0u8; outlen];
        let shift = label.len() % src.len().max(1);
        for (i, b) in out.iter_mut().enumerate() {
            *b = src[(i + shift) % src.len()] ^ (i as u8);
        }
        out
    }
}

struct DummyHash;
impl Hash for DummyHash {
    fn sha512(&self, _data: &[u8]) -> [u8; 64] {
        [0u8; 64]
    }
}

struct DummyDh;
impl X25519Dh for DummyDh {
    fn keypair(&self) -> (X25519Priv, X25519Pub) {
        (X25519Priv([0xA1u8; 32]), X25519Pub([0xA2u8; 32]))
    }
    fn dh(&self, _privk: &X25519Priv, _pubk: &X25519Pub) -> [u8; 32] {
        [0xD3u8; 32]
    }
}

struct DummyPqKem;
impl PqKem768 for DummyPqKem {
    fn encap(&self, _pubk: &[u8]) -> Result<(Vec<u8>, Vec<u8>), CryptoError> {
        Err(CryptoError::NotImplemented)
    }
    fn decap(&self, _privk: &[u8], _ct: &[u8]) -> Result<Vec<u8>, CryptoError> {
        Err(CryptoError::NotImplemented)
    }
}

struct FixedRng;
impl Rng12 for FixedRng {
    fn random_nonce12(&mut self) -> [u8; 12] {
        [0x11u8; 12]
    }
}

struct KeyedAead;
impl Aead for KeyedAead {
    fn seal(&self, key32: &[u8; 32], _nonce12: &[u8; 12], _ad: &[u8], pt: &[u8]) -> Vec<u8> {
        let mut out = Vec::with_capacity(32 + pt.len());
        out.extend_from_slice(key32);
        out.extend_from_slice(pt);
        out
    }

    fn open(
        &self,
        key32: &[u8; 32],
        _nonce12: &[u8; 12],
        _ad: &[u8],
        ct: &[u8],
    ) -> Result<Vec<u8>, CryptoError> {
        if ct.len() < 32 {
            return Err(CryptoError::AuthFail);
        }
        if &ct[..32] != key32 {
            return Err(CryptoError::AuthFail);
        }
        Ok(ct[32..].to_vec())
    }
}

fn make_state(
    role: SessionRole,
    session_id: [u8; 16],
    rk0: [u8; 32],
    kmac: &dyn Kmac,
    dh_self: (X25519Priv, X25519Pub),
    dh_peer: [u8; 32],
) -> SessionState {
    let pq_self = (1u32, vec![0x01u8; 4], vec![0x02u8; 4]);
    SessionState::new(role, session_id, rk0, kmac, dh_self, dh_peer, pq_self)
}

#[test]
fn header_keys_depend_on_rk() {
    let kmac = TestKmac;
    let session_id = [0x10u8; 16];
    let dh_self = (X25519Priv([0xA1u8; 32]), X25519Pub([0xA2u8; 32]));
    let dh_peer = [0xB2u8; 32];

    let st1 = make_state(
        SessionRole::Initiator,
        session_id,
        [0x01u8; 32],
        &kmac,
        dh_self.clone(),
        dh_peer,
    );
    let st2 = make_state(
        SessionRole::Initiator,
        session_id,
        [0x02u8; 32],
        &kmac,
        dh_self,
        dh_peer,
    );

    assert_ne!(st1.nhk_s, st2.nhk_s, "nhk_s must depend on rk");
    assert_ne!(st1.hk_s, st2.hk_s, "hk_s must depend on rk");
}

#[test]
fn boundary_header_wrong_rk_rejects_and_no_mutation() {
    let kmac = TestKmac;
    let hash = DummyHash;
    let aead = KeyedAead;
    let dh = DummyDh;
    let pq = DummyPqKem;
    let mut rng = FixedRng;

    let session_id = [0x33u8; 16];
    let rk0 = [0x44u8; 32];
    let rk0_bad = [0x99u8; 32];

    // Use initial DH keys that differ from DummyDh::keypair() output so boundary ratchet occurs.
    let dh_a = (X25519Priv([0xA0u8; 32]), X25519Pub([0xA0u8; 32]));
    let dh_b = (X25519Priv([0xB0u8; 32]), X25519Pub([0xB0u8; 32]));

    let mut st_send = make_state(
        SessionRole::Initiator,
        session_id,
        rk0,
        &kmac,
        dh_a.clone(),
        dh_b.1 .0,
    );

    let mut st_recv_ok = make_state(
        SessionRole::Responder,
        session_id,
        rk0,
        &kmac,
        dh_b.clone(),
        dh_a.1 .0,
    );

    let mut st_recv_bad = make_state(
        SessionRole::Responder,
        session_id,
        rk0_bad,
        &kmac,
        dh_b,
        dh_a.1 .0,
    );

    let msg = ratchet_encrypt(
        &mut st_send,
        &hash,
        &kmac,
        &aead,
        &dh,
        &pq,
        &mut rng,
        b"hello",
        false,
        false,
    )
    .expect("encrypt");

    let pt_ok =
        ratchet_decrypt(&mut st_recv_ok, &hash, &kmac, &aead, &dh, &pq, &msg).expect("decrypt ok");
    assert_eq!(pt_ok, b"hello");

    let before = st_recv_bad.snapshot_bytes();
    let err = ratchet_decrypt(&mut st_recv_bad, &hash, &kmac, &aead, &dh, &pq, &msg)
        .expect_err("wrong rk should fail");
    let after = st_recv_bad.snapshot_bytes();
    assert_eq!(before, after, "state mutated on reject: {err:?}");
}
