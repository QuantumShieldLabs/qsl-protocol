use quantumshield_refimpl::crypto::stdcrypto::{runtime_pq_kem_keypair, StdCrypto};
use quantumshield_refimpl::crypto::traits::{CryptoError, PqKem768};

#[test]
fn pqkem768_roundtrip_matches() {
    let c = StdCrypto;
    let (pk, sk) = runtime_pq_kem_keypair();
    let (ct, ss1) = c.encap(&pk).expect("encap");
    let ss2 = c.decap(&sk, &ct).expect("decap");
    assert_eq!(ss1, ss2);
}

#[test]
fn pqkem768_tamper_changes_secret() {
    let c = StdCrypto;
    let (pk, sk) = runtime_pq_kem_keypair();
    let (mut ct, ss1) = c.encap(&pk).expect("encap");
    if let Some(b) = ct.get_mut(0) {
        *b ^= 0x01;
    }
    let ss2 = c.decap(&sk, &ct).expect("decap");
    assert_ne!(ss1, ss2);
}

#[test]
fn pqkem768_wrong_length_inputs_fail_closed() {
    let c = StdCrypto;
    let (pk, sk) = runtime_pq_kem_keypair();
    let (ct, _ss) = c.encap(&pk).expect("encap");

    assert!(matches!(
        c.encap(&pk[..pk.len() - 1]),
        Err(CryptoError::InvalidKey)
    ));
    assert!(matches!(
        c.decap(&sk[..sk.len() - 1], &ct),
        Err(CryptoError::InvalidKey)
    ));
    assert!(matches!(
        c.decap(&sk, &ct[..ct.len() - 1]),
        Err(CryptoError::InvalidKey)
    ));
}
