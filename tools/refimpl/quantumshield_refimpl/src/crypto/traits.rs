use thiserror::Error;

#[derive(Debug, Error)]
pub enum CryptoError {
    #[error("authentication failed")]
    AuthFail,
    #[error("invalid key material")]
    InvalidKey,
    #[error("not implemented")]
    NotImplemented,
}

pub trait Hash {
    fn sha512(&self, data: &[u8]) -> [u8; 64];
}

pub trait Kmac {
    fn kmac256(&self, key: &[u8], label: &str, data: &[u8], outlen: usize) -> Vec<u8>;
}

pub trait Aead {
    fn seal(&self, key32: &[u8; 32], nonce12: &[u8; 12], ad: &[u8], pt: &[u8]) -> Vec<u8>;
    fn open(&self, key32: &[u8; 32], nonce12: &[u8; 12], ad: &[u8], ct: &[u8]) -> Result<Vec<u8>, CryptoError>;
}

pub trait X25519Dh {
    fn keypair(&self) -> (X25519Priv, X25519Pub);
    fn dh(&self, privk: &X25519Priv, pubk: &X25519Pub) -> [u8; 32];
}

#[derive(Clone)]
pub struct X25519Priv(pub [u8; 32]);
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct X25519Pub(pub [u8; 32]);

pub trait SigEd25519 {
    fn sign(&self, privk: &[u8], msg: &[u8]) -> Vec<u8>; // 64 bytes
    fn verify(&self, pubk: &[u8], msg: &[u8], sig: &[u8]) -> bool;
}

pub trait PqKem768 {
    fn encap(&self, pubk: &[u8]) -> Result<(Vec<u8>, Vec<u8>), CryptoError>; // (ct, ss)
    fn decap(&self, privk: &[u8], ct: &[u8]) -> Result<Vec<u8>, CryptoError>;  // ss
}

pub trait PqSigMldsa65 {
    fn sign(&self, privk: &[u8], msg: &[u8]) -> Result<Vec<u8>, CryptoError>;
    fn verify(&self, pubk: &[u8], msg: &[u8], sig: &[u8]) -> Result<bool, CryptoError>;
}

pub trait Rng12 {
    fn random_nonce12(&mut self) -> [u8; 12];
}
