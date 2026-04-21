//! Key Transparency (KT) verification interfaces and canonical verifier wiring.

mod canonical;

use crate::crypto::traits::{PqSigMldsa65, SigEd25519};
use crate::qsp::{HandshakeInit, PrekeyBundle};
use thiserror::Error;

pub use canonical::{AcceptedSth, CanonicalKtVerifier, KtPinnedLog, KtTimeSource};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KtVerification {
    Verified,
    DisabledNonProduction,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum KtError {
    #[error("reason_code=bundle_sig_fail")]
    BundleSigFail { detail: &'static str },
    #[error("reason_code=kt_fail")]
    VerifyFailed { detail: &'static str },
}

impl KtError {
    pub(crate) fn bundle_sig(detail: &'static str) -> Self {
        Self::BundleSigFail { detail }
    }

    pub(crate) fn kt_fail(detail: &'static str) -> Self {
        Self::VerifyFailed { detail }
    }

    pub fn detail(&self) -> &'static str {
        match self {
            Self::BundleSigFail { detail } | Self::VerifyFailed { detail } => detail,
        }
    }
}

pub trait KtVerifier {
    /// Verify a peer bundle before authenticated use.
    fn verify_bundle(
        &self,
        bundle: &PrekeyBundle,
        ed25519: &dyn SigEd25519,
        pq_sig: &dyn PqSigMldsa65,
    ) -> Result<KtVerification, KtError>;

    /// Verify the responder-side initiator bundle evidence required by DOC-CAN-008.
    fn verify_responder_binding(
        &self,
        hs1: &HandshakeInit,
        initiator_bundle: Option<&PrekeyBundle>,
        ed25519: &dyn SigEd25519,
        pq_sig: &dyn PqSigMldsa65,
    ) -> Result<KtVerification, KtError>;
}
