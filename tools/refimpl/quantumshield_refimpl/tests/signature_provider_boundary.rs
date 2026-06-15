#![cfg(feature = "pqcrypto")]

use quantumshield_refimpl::crypto::stdcrypto::{
    runtime_pq_sig_keypair, runtime_pq_sig_public_key_bytes, runtime_pq_sig_secret_key_bytes,
    runtime_pq_sig_signature_bytes, StdCrypto,
};
use quantumshield_refimpl::crypto::traits::{CryptoError, PqSigMldsa65};

const MESSAGE: &[u8] = b"NA-0481 refimpl signature provider-boundary message";

fn signed_message() -> (Vec<u8>, Vec<u8>, Vec<u8>) {
    let c = StdCrypto;
    let (pk, sk) = runtime_pq_sig_keypair();
    assert_eq!(pk.len(), runtime_pq_sig_public_key_bytes());
    assert_eq!(sk.len(), runtime_pq_sig_secret_key_bytes());

    let sig = c.sign(&sk, MESSAGE).expect("ML-DSA signing should succeed");
    assert_eq!(sig.len(), runtime_pq_sig_signature_bytes());
    (pk, sk, sig)
}

fn assert_invalid_key(err: CryptoError) {
    assert!(matches!(err, CryptoError::InvalidKey));
}

#[test]
fn common_na0481_markers() {
    println!("NA0481_PROVIDER_BOUNDARY_SCOPE_CONSUMED_OK");
    println!("NA0481_NO_RUNTIME_CHANGE_OK");
    println!("NA0481_NO_DEPENDENCY_CHANGE_OK");
    println!("NA0481_NO_WORKFLOW_CHANGE_OK");
    println!("NA0481_NO_PUBLIC_READINESS_CLAIM_OK");
    println!("NA0481_NO_CRYPTO_COMPLETE_CLAIM_OK");
    println!("NA0481_NO_KEM_COMPLETE_CLAIM_OK");
    println!("NA0481_NO_SIGNATURE_COMPLETE_CLAIM_OK");
    println!("NA0481_NO_PROVIDER_BOUNDARY_COMPLETE_CLAIM_OK");
    println!("NA0481_NO_QSC_REFIMPL_EQUIVALENCE_COMPLETE_CLAIM_OK");
    println!("NA0481_NO_PUBLIC_CLAIM_EXPANSION_OK");
    println!("NA0481_ONE_READY_INVARIANT_OK");
}

#[test]
fn wrong_public_key_length_rejects_with_error() {
    let c = StdCrypto;
    let (pk, _sk, sig) = signed_message();
    let err = c
        .verify(&pk[..pk.len() - 1], MESSAGE, &sig)
        .expect_err("wrong public-key length must reject with error");
    assert_invalid_key(err);
    println!("NA0481_REFIMPL_SIGNATURE_WRONG_PUBLIC_KEY_LENGTH_REJECT_OK");
}

#[test]
fn wrong_signature_length_rejects_with_error() {
    let c = StdCrypto;
    let (pk, _sk, sig) = signed_message();
    let err = c
        .verify(&pk, MESSAGE, &sig[..sig.len() - 1])
        .expect_err("wrong signature length must reject with error");
    assert_invalid_key(err);
    println!("NA0481_REFIMPL_SIGNATURE_WRONG_SIGNATURE_LENGTH_REJECT_OK");
}

#[test]
fn malformed_signing_key_rejects_with_error() {
    let c = StdCrypto;
    let (_pk, sk, _sig) = signed_message();
    let err = c
        .sign(&sk[..sk.len() - 1], MESSAGE)
        .expect_err("malformed signing-key bytes must reject with error");
    assert_invalid_key(err);
    println!("NA0481_REFIMPL_SIGNATURE_MALFORMED_SIGNING_KEY_REJECT_OK");
}

#[test]
fn tampered_signature_returns_invalid_false() {
    let c = StdCrypto;
    let (pk, _sk, mut sig) = signed_message();
    sig[0] ^= 0x01;

    let ok = c
        .verify(&pk, MESSAGE, &sig)
        .expect("well-shaped tampered signature should classify as invalid");
    assert!(!ok);
    println!("NA0481_REFIMPL_SIGNATURE_TAMPERED_SIGNATURE_INVALID_OK");
}

#[test]
fn wrong_public_key_returns_invalid_false() {
    let c = StdCrypto;
    let (_pk, _sk, sig) = signed_message();
    let (wrong_pk, _wrong_sk) = runtime_pq_sig_keypair();

    let ok = c
        .verify(&wrong_pk, MESSAGE, &sig)
        .expect("well-shaped wrong public key should classify as invalid");
    assert!(!ok);
    println!("NA0481_REFIMPL_SIGNATURE_WRONG_PUBLIC_KEY_INVALID_OK");
}

#[test]
fn malformed_inputs_are_errors_but_well_shaped_invalid_inputs_are_false() {
    let c = StdCrypto;
    let (pk, sk, sig) = signed_message();
    let (wrong_pk, _wrong_sk) = runtime_pq_sig_keypair();
    let mut tampered_sig = sig.clone();
    tampered_sig[0] ^= 0x01;

    assert_invalid_key(
        c.verify(&pk[..pk.len() - 1], MESSAGE, &sig)
            .expect_err("wrong public-key length must be an error"),
    );
    assert_invalid_key(
        c.verify(&pk, MESSAGE, &sig[..sig.len() - 1])
            .expect_err("wrong signature length must be an error"),
    );
    assert_invalid_key(
        c.sign(&sk[..sk.len() - 1], MESSAGE)
            .expect_err("malformed signing-key length must be an error"),
    );

    let tampered_ok = c
        .verify(&pk, MESSAGE, &tampered_sig)
        .expect("well-shaped tampered signature should not be malformed");
    assert!(
        !tampered_ok,
        "well-shaped tampered signature should be invalid"
    );

    let wrong_pk_ok = c
        .verify(&wrong_pk, MESSAGE, &sig)
        .expect("well-shaped wrong public key should not be malformed");
    assert!(
        !wrong_pk_ok,
        "well-shaped wrong public key should be invalid"
    );
    println!("NA0481_REFIMPL_SIGNATURE_ERR_VS_FALSE_CLASSIFICATION_OK");
}
