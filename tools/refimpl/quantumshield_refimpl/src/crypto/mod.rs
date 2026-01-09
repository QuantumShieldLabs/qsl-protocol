//! Cryptographic interfaces and (optional) standard-crypto implementations.
//!
//! QSP v4.3.1 requires:
//! - SHA-512, KMAC-256
//! - AES-256-GCM
//! - X25519
//! - Ed25519 signatures
//! - ML-KEM-768, ML-DSA-65 (interfaces here; algorithm binding must be validated for your chosen PQ library)

pub mod traits;
#[cfg(feature = "stdcrypto")]
pub mod stdcrypto;
