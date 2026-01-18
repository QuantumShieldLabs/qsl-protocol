//! Standard-crypto implementations (non-PQ).
//!
//! These are convenience wrappers. For the PQ primitives, use the `PqKem768` and `PqSigMldsa65` traits
//! and provide an implementation suitable for your environment.

use super::traits::*;
use aes_gcm::{
    aead::{Aead as _, Payload},
    Aes256Gcm, KeyInit, Nonce,
};
use rand::{rngs::OsRng, RngCore};
use sha2::{Digest, Sha512};
use tiny_keccak::{Hasher, Kmac as KeccakKmac};

pub struct StdCrypto;

impl StdCrypto {
    fn seal_inner(
        &self,
        key: &[u8],
        nonce: &[u8],
        ad: &[u8],
        pt: &[u8],
    ) -> Result<Vec<u8>, CryptoError> {
        if key.len() != 32 || nonce.len() != 12 {
            return Err(CryptoError::InvalidKey);
        }
        let cipher = Aes256Gcm::new_from_slice(key).map_err(|_| CryptoError::InvalidKey)?;
        let nonce = Nonce::from_slice(nonce);
        cipher
            .encrypt(nonce, Payload { msg: pt, aad: ad })
            .map_err(|_| CryptoError::AuthFail)
    }
}

impl Hash for StdCrypto {
    fn sha512(&self, data: &[u8]) -> [u8; 64] {
        let mut h = Sha512::new();
        h.update(data);
        let out = h.finalize();
        let mut r = [0u8; 64];
        r.copy_from_slice(&out);
        r
    }
}

impl Kmac for StdCrypto {
    fn kmac256(&self, key: &[u8], label: &str, data: &[u8], outlen: usize) -> Vec<u8> {
        let mut kmac = KeccakKmac::v256(key, label.as_bytes());
        kmac.update(data);
        let mut out = vec![0u8; outlen];
        kmac.finalize(&mut out);
        out
    }
}

impl Aead for StdCrypto {
    fn seal(&self, key32: &[u8; 32], nonce12: &[u8; 12], ad: &[u8], pt: &[u8]) -> Vec<u8> {
        match self.seal_inner(key32, nonce12, ad, pt) {
            Ok(ct) => ct,
            Err(_) => Vec::new(),
        }
    }

    fn open(
        &self,
        key32: &[u8; 32],
        nonce12: &[u8; 12],
        ad: &[u8],
        ct: &[u8],
    ) -> Result<Vec<u8>, CryptoError> {
        let cipher = Aes256Gcm::new_from_slice(key32).map_err(|_| CryptoError::InvalidKey)?;
        cipher
            .decrypt(nonce12.into(), Payload { msg: ct, aad: ad })
            .map_err(|_| CryptoError::AuthFail)
    }
}

impl X25519Dh for StdCrypto {
    fn keypair(&self) -> (X25519Priv, X25519Pub) {
        use x25519_dalek::{PublicKey, StaticSecret};
        let mut sk_bytes = [0u8; 32];
        OsRng.fill_bytes(&mut sk_bytes);
        let sk = StaticSecret::from(sk_bytes);
        let pk = PublicKey::from(&sk);
        (X25519Priv(sk.to_bytes()), X25519Pub(pk.to_bytes()))
    }

    fn dh(&self, privk: &X25519Priv, pubk: &X25519Pub) -> [u8; 32] {
        use x25519_dalek::{PublicKey, StaticSecret};
        let sk = StaticSecret::from(privk.0);
        let pk = PublicKey::from(pubk.0);
        (sk.diffie_hellman(&pk)).to_bytes()
    }
}

pub struct StdEd25519;

impl SigEd25519 for StdEd25519 {
    fn sign(&self, privk: &[u8], msg: &[u8]) -> Vec<u8> {
        use ed25519_dalek::{Signature, Signer, SigningKey};
        let Ok(bytes) = <[u8; 32]>::try_from(privk) else {
            return Vec::new();
        };
        let sk = SigningKey::from_bytes(&bytes);
        let sig: Signature = sk.sign(msg);
        sig.to_bytes().to_vec()
    }

    fn verify(&self, pubk: &[u8], msg: &[u8], sig: &[u8]) -> bool {
        use ed25519_dalek::{Signature, Verifier, VerifyingKey};
        let Ok(bytes) = <[u8; 32]>::try_from(pubk) else {
            return false;
        };
        let pk = match VerifyingKey::from_bytes(&bytes) {
            Ok(v) => v,
            Err(_) => return false,
        };
        let sig = match Signature::from_slice(sig) {
            Ok(s) => s,
            Err(_) => return false,
        };
        pk.verify(msg, &sig).is_ok()
    }
}

pub struct StdRng;
impl Rng12 for StdRng {
    fn random_nonce12(&mut self) -> [u8; 12] {
        let mut n = [0u8; 12];
        OsRng.fill_bytes(&mut n);
        n
    }
}

#[cfg(test)]
mod tests {
    use super::{CryptoError, Rng12, SigEd25519, StdCrypto, StdEd25519, StdRng, X25519Dh};
    use rand::{rngs::OsRng, RngCore};

    fn rand_vec(len: usize) -> Vec<u8> {
        let mut v = Vec::with_capacity(len);
        // Safety: OsRng fills all bytes before use.
        unsafe { v.set_len(len) };
        OsRng.fill_bytes(&mut v);
        v
    }

    #[test]
    fn ed25519_verify_rejects_invalid_pubk_len() {
        let ed = StdEd25519;
        let ok = ed.verify(&[0u8; 31], b"msg", &[0u8; 64]);
        assert!(!ok);
    }

    #[test]
    fn ed25519_sign_invalid_priv_len_is_fail_closed() {
        let ed = StdEd25519;
        let sig = ed.sign(&[0u8; 31], b"msg");
        assert!(sig.is_empty());
    }

    #[test]
    fn aead_seal_invalid_key_len_is_fail_closed() {
        let c = StdCrypto;
        let key = rand_vec(31);
        let nonce = rand_vec(12);
        let err = c.seal_inner(&key, &nonce, b"ad", b"pt").unwrap_err();
        assert!(matches!(err, CryptoError::InvalidKey));
    }

    #[test]
    fn aead_seal_invalid_nonce_len_is_fail_closed() {
        let c = StdCrypto;
        let key = rand_vec(32);
        let nonce = rand_vec(11);
        let err = c.seal_inner(&key, &nonce, b"ad", b"pt").unwrap_err();
        assert!(matches!(err, CryptoError::InvalidKey));
    }

    #[test]
    fn x25519_keypair_uses_os_rng() {
        let c = StdCrypto;
        let (sk, _pk) = c.keypair();
        assert!(sk.0.iter().any(|b| *b != 0));
    }

    #[test]
    fn random_nonce12_not_all_zero() {
        let mut r = StdRng;
        let n = r.random_nonce12();
        assert!(n.iter().any(|b| *b != 0));
    }
}
