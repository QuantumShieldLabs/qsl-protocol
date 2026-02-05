use pqcrypto_kyber::kyber768;
use pqcrypto_traits::kem::{PublicKey as _, SecretKey as _};
use quantumshield_refimpl::crypto::stdcrypto::StdCrypto;
use quantumshield_refimpl::crypto::traits::PqKem768;

#[test]
fn pqkem768_roundtrip_matches() {
    let c = StdCrypto;
    let (pk, sk) = kyber768::keypair();
    let (ct, ss1) = c.encap(pk.as_bytes()).expect("encap");
    let ss2 = c.decap(sk.as_bytes(), &ct).expect("decap");
    assert_eq!(ss1, ss2);
}

#[test]
fn pqkem768_tamper_changes_secret() {
    let c = StdCrypto;
    let (pk, sk) = kyber768::keypair();
    let (mut ct, ss1) = c.encap(pk.as_bytes()).expect("encap");
    if let Some(b) = ct.get_mut(0) {
        *b ^= 0x01;
    }
    let ss2 = c.decap(sk.as_bytes(), &ct).expect("decap");
    assert_ne!(ss1, ss2);
}
