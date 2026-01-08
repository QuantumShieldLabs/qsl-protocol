//! Key Transparency (KT) interface.
//!
//! QSP requires identity key distribution and pinning via KT in Authenticated mode.
//! Wire formats for STH/inclusion/consistency proofs may vary by KT system; therefore
//! this reference skeleton defines an interface and defers the concrete encoding until
//! the project finalizes KT serialization.

use thiserror::Error;

#[derive(Debug, Error)]
pub enum KtError {
    #[error("proof verification failed: {0}")]
    VerifyFailed(&'static str),
    #[error("not implemented")]
    NotImplemented,
}

pub trait KtVerifier {
    /// Verify KT materials carried in a PrekeyBundle (log id, STH, inclusion proof, consistency proof).
    ///
    /// Implementations MUST enforce:
    /// - log id pinning policy,
    /// - STH signature verification,
    /// - inclusion proof for the bundle leaf,
    /// - consistency proof when provided.
    fn verify_bundle(&self,
        kt_log_id: &[u8; 32],
        kt_sth: &[u8],
        kt_inclusion_proof: &[u8],
        kt_consistency_proof: &[u8],
    ) -> Result<(), KtError>;
}

/// Stub verifier that always errors (useful to ensure callers do not silently skip KT).
pub struct StubKt;
impl KtVerifier for StubKt {
    fn verify_bundle(&self, _kt_log_id: &[u8; 32], _kt_sth: &[u8], _kt_inclusion_proof: &[u8], _kt_consistency_proof: &[u8]) -> Result<(), KtError> {
        Err(KtError::NotImplemented)
    }
}
