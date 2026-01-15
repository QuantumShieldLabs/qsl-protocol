use core::fmt;

use crate::codec::CodecError;
use crate::qsp::RatchetError;

/// Canonical refimpl error surface for callers.
///
/// Audit Issue #25: Suite-2 returns string codes while QSP uses typed errors.
/// This wrapper makes boundary-layer errors composable and deterministically testable.
///
/// Formatting invariant:
/// - All user-visible rejects MUST include a stable `reason_code=<CODE>` token.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RefimplError {
    Reject(&'static str),
    QspCodec,
    QspRatchet,
    Internal,
}

impl RefimplError {
    pub fn code(&self) -> &'static str {
        match self {
            RefimplError::Reject(c) => c,
            RefimplError::QspCodec => "REJECT_QSP_CODEC_ERROR",
            RefimplError::QspRatchet => "REJECT_QSP_RATCHET_ERROR",
            RefimplError::Internal => "REJECT_INTERNAL",
        }
    }
}

impl From<&'static str> for RefimplError {
    fn from(code: &'static str) -> Self {
        RefimplError::Reject(code)
    }
}

impl From<CodecError> for RefimplError {
    fn from(_: CodecError) -> Self {
        RefimplError::QspCodec
    }
}

impl From<RatchetError> for RefimplError {
    fn from(_: RatchetError) -> Self {
        RefimplError::QspRatchet
    }
}

impl fmt::Display for RefimplError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = self.code();
        write!(f, "invalid request: reject: {c}; reason_code={c}")
    }
}

impl std::error::Error for RefimplError {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codec::CodecError;
    use crate::qsp::RatchetError;

    #[test]
    fn formats_suite2_reject_with_reason_code_token() {
        let e: RefimplError = "REJECT_S2_PARSE_PREFIX".into();
        assert_eq!(
            e.to_string(),
            "invalid request: reject: REJECT_S2_PARSE_PREFIX; reason_code=REJECT_S2_PARSE_PREFIX"
        );
    }

    #[test]
    fn formats_qsp_codec_with_reason_code_token() {
        let e: RefimplError = CodecError::Invalid("flags").into();
        assert_eq!(
            e.to_string(),
            "invalid request: reject: REJECT_QSP_CODEC_ERROR; reason_code=REJECT_QSP_CODEC_ERROR"
        );
    }

    #[test]
    fn formats_qsp_ratchet_with_reason_code_token() {
        let e: RefimplError = RatchetError::Invalid("ns overflow").into();
        assert_eq!(
            e.to_string(),
            "invalid request: reject: REJECT_QSP_RATCHET_ERROR; reason_code=REJECT_QSP_RATCHET_ERROR"
        );
    }
}
