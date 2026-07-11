#![allow(dead_code)]

use quantumshield_refimpl::crypto::stdcrypto::{
    runtime_pq_kem_ciphertext_bytes, runtime_pq_kem_public_key_bytes,
    runtime_pq_sig_public_key_bytes, runtime_pq_sig_signature_bytes,
};

const HS_MAGIC: &[u8; 4] = b"QHSM";
const HS_VERSION_LEGACY: u16 = 1;
const HS_VERSION_V2: u16 = 2;
const HS_TYPE_INIT: u8 = 1;
const HS_TYPE_RESP: u8 = 2;
const HS_TYPE_CONFIRM: u8 = 3;
const HS_PARAM_BLOCK_MAX: usize = 64;
const HS_PARAM_SUITE_CONTEXT: u16 = 0x0001;
const HS_PARAM_FLAG_CRITICAL: u8 = 0x01;
const HS_SUITE2_PROTOCOL_VERSION_WIRE: u16 = 0x0500;
const HS_SUITE2_SUITE_ID_WIRE: u16 = 0x0002;
const HS_LEGACY_PROTOCOL_VERSION_WIRE: u16 = 0x0403;
const HS_LEGACY_SUITE_ID_WIRE: u16 = 0x0001;
pub const NA0487_BINDING_FUZZ_HELPER_SCOPE_CONSUMED_OK: &str =
    "NA0487_BINDING_FUZZ_HELPER_SCOPE_CONSUMED_OK";
pub const NA0487_HELPER_API_TEST_FUZZ_ONLY_OK: &str = "NA0487_HELPER_API_TEST_FUZZ_ONLY_OK";
pub const NA0487_HELPER_API_NO_PRODUCTION_BEHAVIOR_CHANGE_OK: &str =
    "NA0487_HELPER_API_NO_PRODUCTION_BEHAVIOR_CHANGE_OK";
pub const NA0487_HELPER_API_NO_SECRET_OUTPUT_OK: &str = "NA0487_HELPER_API_NO_SECRET_OUTPUT_OK";
pub const NA0487_HELPER_API_REAL_REJECT_PATHS_OK: &str = "NA0487_HELPER_API_REAL_REJECT_PATHS_OK";
pub const NA0487_HELPER_API_VECTOR_TRACEABILITY_OK: &str =
    "NA0487_HELPER_API_VECTOR_TRACEABILITY_OK";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BindingFuzzCategory {
    A1Mutation,
    B1Mutation,
    A2Mutation,
    SuiteConfusion,
    Replay,
    StalePublicRecord,
    VectorTraceability,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BindingFuzzFrameKind {
    A1,
    B1,
    A2,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FuzzSuiteMode {
    LegacyCompat,
    SuiteRequired,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FuzzSuiteContext {
    LegacyV1,
    ExplicitV2 {
        block: Vec<u8>,
        protocol_version: u16,
        suite_id: u16,
    },
}

impl FuzzSuiteContext {
    pub fn explicit_block(&self) -> Option<&[u8]> {
        match self {
            Self::LegacyV1 => None,
            Self::ExplicitV2 { block, .. } => Some(block.as_slice()),
        }
    }

    pub fn is_explicit(&self) -> bool {
        self.explicit_block().is_some()
    }

    pub fn wire_version(&self) -> u16 {
        match self {
            Self::LegacyV1 => HS_VERSION_LEGACY,
            Self::ExplicitV2 { .. } => HS_VERSION_V2,
        }
    }

    pub fn protocol_version(&self) -> u16 {
        match self {
            Self::LegacyV1 => HS_LEGACY_PROTOCOL_VERSION_WIRE,
            Self::ExplicitV2 {
                protocol_version, ..
            } => *protocol_version,
        }
    }

    pub fn suite_id(&self) -> u16 {
        match self {
            Self::LegacyV1 => HS_LEGACY_SUITE_ID_WIRE,
            Self::ExplicitV2 { suite_id, .. } => *suite_id,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FuzzHeader {
    suite_context: FuzzSuiteContext,
    payload_offset: usize,
}

impl FuzzHeader {
    pub fn suite_context(&self) -> &FuzzSuiteContext {
        &self.suite_context
    }

    pub fn payload_offset(&self) -> usize {
        self.payload_offset
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FuzzFrameShape {
    kind: BindingFuzzFrameKind,
    suite_context: FuzzSuiteContext,
    session_id: [u8; 16],
}

impl FuzzFrameShape {
    pub fn kind(&self) -> BindingFuzzFrameKind {
        self.kind
    }

    pub fn suite_context(&self) -> &FuzzSuiteContext {
        &self.suite_context
    }

    pub fn session_id(&self) -> &[u8; 16] {
        &self.session_id
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BindingFuzzOutcomeKind {
    AcceptedFrame,
    DecodeRejected,
    ReplayRejected,
    IdentityRejected,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BindingFuzzOutcome {
    category: BindingFuzzCategory,
    kind: BindingFuzzOutcomeKind,
    frame_kind: Option<BindingFuzzFrameKind>,
    reason: &'static str,
}

impl BindingFuzzOutcome {
    pub fn category(&self) -> BindingFuzzCategory {
        self.category
    }

    pub fn kind(&self) -> BindingFuzzOutcomeKind {
        self.kind
    }

    pub fn frame_kind(&self) -> Option<BindingFuzzFrameKind> {
        self.frame_kind
    }

    pub fn reason(&self) -> &'static str {
        self.reason
    }
}

pub fn parse_suite_context(block: &[u8]) -> Result<FuzzSuiteContext, &'static str> {
    if block.len() > HS_PARAM_BLOCK_MAX {
        return Err("REJECT_QSC_HS_MALFORMED_LENGTH");
    }
    let mut off = 0usize;
    let mut prior_id: Option<u16> = None;
    let mut suite_value: Option<[u8; 4]> = None;
    let mut unknown_critical = false;
    let mut unknown_parameter = false;

    while off < block.len() {
        if block.len().saturating_sub(off) < 5 {
            return Err("REJECT_QSC_HS_MALFORMED_LENGTH");
        }
        let param_id = u16::from_be_bytes([block[off], block[off + 1]]);
        let flags = block[off + 2];
        let value_len = u16::from_be_bytes([block[off + 3], block[off + 4]]) as usize;
        off += 5;
        if flags & !HS_PARAM_FLAG_CRITICAL != 0 {
            return Err("REJECT_QSC_HS_MALFORMED_LENGTH");
        }
        if let Some(prev) = prior_id {
            if param_id == prev {
                return Err("REJECT_QSC_HS_DUPLICATE_PARAMETER");
            }
            if param_id < prev {
                return Err("REJECT_QSC_HS_NONCANONICAL_ORDER");
            }
        }
        prior_id = Some(param_id);
        if block.len().saturating_sub(off) < value_len {
            return Err("REJECT_QSC_HS_MALFORMED_LENGTH");
        }
        let value = &block[off..off + value_len];
        off += value_len;

        if param_id == HS_PARAM_SUITE_CONTEXT {
            if suite_value.is_some() {
                return Err("REJECT_QSC_HS_DUPLICATE_PARAMETER");
            }
            if flags != HS_PARAM_FLAG_CRITICAL || value_len != 4 {
                return Err("REJECT_QSC_HS_MALFORMED_LENGTH");
            }
            let mut tuple = [0u8; 4];
            tuple.copy_from_slice(value);
            suite_value = Some(tuple);
            continue;
        }

        if flags & HS_PARAM_FLAG_CRITICAL != 0 {
            unknown_critical = true;
        } else {
            unknown_parameter = true;
        }
    }

    let Some(tuple) = suite_value else {
        return Err("REJECT_QSC_HS_SUITE_MISSING");
    };
    if unknown_critical {
        return Err("REJECT_QSC_HS_UNKNOWN_CRITICAL");
    }
    if unknown_parameter {
        return Err("REJECT_QSC_HS_UNKNOWN_PARAMETER");
    }

    let protocol_version = u16::from_be_bytes([tuple[0], tuple[1]]);
    let suite_id = u16::from_be_bytes([tuple[2], tuple[3]]);
    if protocol_version == HS_SUITE2_PROTOCOL_VERSION_WIRE && suite_id == HS_SUITE2_SUITE_ID_WIRE {
        return Ok(FuzzSuiteContext::ExplicitV2 {
            block: block.to_vec(),
            protocol_version,
            suite_id,
        });
    }
    if protocol_version == HS_SUITE2_PROTOCOL_VERSION_WIRE {
        return Err("REJECT_QSC_HS_SUITE_UNSUPPORTED");
    }
    if protocol_version == HS_LEGACY_PROTOCOL_VERSION_WIRE && suite_id == HS_LEGACY_SUITE_ID_WIRE {
        return Err("REJECT_QSC_HS_DOWNGRADE");
    }
    Err("REJECT_QSC_HS_INCONSISTENT_TUPLE")
}

pub fn decode_header(
    bytes: &[u8],
    frame_kind: BindingFuzzFrameKind,
    payload_len: usize,
    mode: FuzzSuiteMode,
    admit_context: bool,
) -> Result<FuzzHeader, &'static str> {
    if bytes.len() < 7 {
        return Err("handshake_len");
    }
    if &bytes[0..4] != HS_MAGIC {
        return Err("handshake_magic");
    }
    let ver = u16::from_be_bytes([bytes[4], bytes[5]]);
    if bytes[6] != frame_type(frame_kind) {
        return Err("handshake_type");
    }
    match ver {
        HS_VERSION_LEGACY => {
            if mode == FuzzSuiteMode::SuiteRequired {
                return Err("REJECT_QSC_HS_LEGACY_REQUIRED");
            }
            if bytes.len() != 7 + payload_len {
                return Err("handshake_len");
            }
            Ok(FuzzHeader {
                suite_context: FuzzSuiteContext::LegacyV1,
                payload_offset: 7,
            })
        }
        HS_VERSION_V2 => {
            if bytes.len() < 9 {
                return Err("REJECT_QSC_HS_MALFORMED_LENGTH");
            }
            let block_len = u16::from_be_bytes([bytes[7], bytes[8]]) as usize;
            if block_len > HS_PARAM_BLOCK_MAX {
                return Err("REJECT_QSC_HS_MALFORMED_LENGTH");
            }
            let payload_offset = 9 + block_len;
            if bytes.len() != payload_offset + payload_len {
                return Err("REJECT_QSC_HS_MALFORMED_LENGTH");
            }
            let block = &bytes[9..payload_offset];
            let suite_context = if admit_context {
                parse_suite_context(block)?
            } else {
                FuzzSuiteContext::ExplicitV2 {
                    block: block.to_vec(),
                    protocol_version: 0,
                    suite_id: 0,
                }
            };
            Ok(FuzzHeader {
                suite_context,
                payload_offset,
            })
        }
        _ => Err("handshake_version"),
    }
}

pub fn decode_init_shape(
    bytes: &[u8],
    mode: FuzzSuiteMode,
) -> Result<FuzzFrameShape, &'static str> {
    // NA-0633 (ENG-0038, C1): A1 now carries the initiator's encapsulation to the responder's identity
    // KEM key (ct), so the payload grows by one ML-KEM ciphertext.
    let payload_len =
        16 + kem_public_key_len() + sig_public_key_len() + 32 + kem_ciphertext_len();
    let header = decode_header(bytes, BindingFuzzFrameKind::A1, payload_len, mode, true)?;
    let mut session_id = [0u8; 16];
    let off = header.payload_offset();
    session_id.copy_from_slice(&bytes[off..off + 16]);
    Ok(FuzzFrameShape {
        kind: BindingFuzzFrameKind::A1,
        suite_context: header.suite_context().clone(),
        session_id,
    })
}

pub fn decode_resp_shape(
    bytes: &[u8],
    mode: FuzzSuiteMode,
    admit_context: bool,
) -> Result<FuzzFrameShape, &'static str> {
    let payload_len =
        16 + kem_ciphertext_len() + 32 + sig_public_key_len() + sig_signature_len() + 32;
    let header = decode_header(
        bytes,
        BindingFuzzFrameKind::B1,
        payload_len,
        mode,
        admit_context,
    )?;
    let mut session_id = [0u8; 16];
    let off = header.payload_offset();
    session_id.copy_from_slice(&bytes[off..off + 16]);
    Ok(FuzzFrameShape {
        kind: BindingFuzzFrameKind::B1,
        suite_context: header.suite_context().clone(),
        session_id,
    })
}

pub fn decode_confirm_shape(
    bytes: &[u8],
    mode: FuzzSuiteMode,
    admit_context: bool,
) -> Result<FuzzFrameShape, &'static str> {
    let payload_len = 16 + 32 + sig_signature_len();
    let header = decode_header(
        bytes,
        BindingFuzzFrameKind::A2,
        payload_len,
        mode,
        admit_context,
    )?;
    let mut session_id = [0u8; 16];
    let off = header.payload_offset();
    session_id.copy_from_slice(&bytes[off..off + 16]);
    Ok(FuzzFrameShape {
        kind: BindingFuzzFrameKind::A2,
        suite_context: header.suite_context().clone(),
        session_id,
    })
}

pub fn replay_candidate_matches_pending_init(
    bytes: &[u8],
    pending_session_id: &[u8; 16],
    pending_block: Option<&[u8]>,
    mode: FuzzSuiteMode,
) -> bool {
    let Ok(init) = decode_init_shape(bytes, mode) else {
        return false;
    };
    init.session_id() == pending_session_id
        && explicit_blocks_match(init.suite_context().explicit_block(), pending_block)
}

pub fn trusted_pin_matches_seen(pinned: &str, seen_fp: &str) -> bool {
    let pinned = pinned.trim();
    if pinned.is_empty() {
        return false;
    }
    pinned.eq_ignore_ascii_case(seen_fp)
}

pub fn trusted_pin_reject_reason(pinned: &str, seen_fp: &str) -> Option<&'static str> {
    if trusted_pin_matches_seen(pinned, seen_fp) {
        None
    } else {
        Some("peer_mismatch")
    }
}

pub fn exercise_binding_fuzz_case(
    category: BindingFuzzCategory,
    data: &[u8],
) -> BindingFuzzOutcome {
    match category {
        BindingFuzzCategory::A1Mutation => classify_frame(
            category,
            BindingFuzzFrameKind::A1,
            decode_init_shape(data, FuzzSuiteMode::SuiteRequired),
        ),
        BindingFuzzCategory::B1Mutation => classify_frame(
            category,
            BindingFuzzFrameKind::B1,
            decode_resp_shape(data, FuzzSuiteMode::SuiteRequired, false),
        ),
        BindingFuzzCategory::A2Mutation => classify_frame(
            category,
            BindingFuzzFrameKind::A2,
            decode_confirm_shape(data, FuzzSuiteMode::SuiteRequired, false),
        ),
        BindingFuzzCategory::SuiteConfusion => classify_frame(
            category,
            BindingFuzzFrameKind::A1,
            decode_init_shape(data, FuzzSuiteMode::SuiteRequired),
        ),
        BindingFuzzCategory::Replay => classify_replay(category, data),
        BindingFuzzCategory::StalePublicRecord => classify_stale_public_record(category, data),
        BindingFuzzCategory::VectorTraceability => {
            let delegated = category_from_input(data.get(1..).unwrap_or_default());
            exercise_binding_fuzz_case(delegated, data.get(1..).unwrap_or_default())
        }
    }
}

pub fn category_from_input(data: &[u8]) -> BindingFuzzCategory {
    match data.first().copied().unwrap_or(0) % 6 {
        0 => BindingFuzzCategory::A1Mutation,
        1 => BindingFuzzCategory::B1Mutation,
        2 => BindingFuzzCategory::A2Mutation,
        3 => BindingFuzzCategory::SuiteConfusion,
        4 => BindingFuzzCategory::Replay,
        _ => BindingFuzzCategory::StalePublicRecord,
    }
}

fn classify_frame(
    category: BindingFuzzCategory,
    frame_kind: BindingFuzzFrameKind,
    decoded: Result<FuzzFrameShape, &'static str>,
) -> BindingFuzzOutcome {
    match decoded {
        Ok(shape) => BindingFuzzOutcome {
            category,
            kind: BindingFuzzOutcomeKind::AcceptedFrame,
            frame_kind: Some(shape.kind()),
            reason: "frame_shape_accepted",
        },
        Err(reason) => BindingFuzzOutcome {
            category,
            kind: BindingFuzzOutcomeKind::DecodeRejected,
            frame_kind: Some(frame_kind),
            reason,
        },
    }
}

fn classify_replay(category: BindingFuzzCategory, data: &[u8]) -> BindingFuzzOutcome {
    match decode_init_shape(data, FuzzSuiteMode::SuiteRequired) {
        Ok(shape) if shape.suite_context().is_explicit() => {
            let pending_block = shape.suite_context().explicit_block();
            if replay_candidate_matches_pending_init(
                data,
                shape.session_id(),
                pending_block,
                FuzzSuiteMode::SuiteRequired,
            ) {
                BindingFuzzOutcome {
                    category,
                    kind: BindingFuzzOutcomeKind::ReplayRejected,
                    frame_kind: Some(BindingFuzzFrameKind::A1),
                    reason: "REJECT_QSC_HS_REPLAY",
                }
            } else {
                BindingFuzzOutcome {
                    category,
                    kind: BindingFuzzOutcomeKind::AcceptedFrame,
                    frame_kind: Some(BindingFuzzFrameKind::A1),
                    reason: "replay_candidate_not_matched",
                }
            }
        }
        Ok(shape) => BindingFuzzOutcome {
            category,
            kind: BindingFuzzOutcomeKind::AcceptedFrame,
            frame_kind: Some(shape.kind()),
            reason: "legacy_replay_not_applicable",
        },
        Err(reason) => BindingFuzzOutcome {
            category,
            kind: BindingFuzzOutcomeKind::DecodeRejected,
            frame_kind: Some(BindingFuzzFrameKind::A1),
            reason,
        },
    }
}

fn classify_stale_public_record(category: BindingFuzzCategory, data: &[u8]) -> BindingFuzzOutcome {
    let seen = "QSCFP-00000000000000000000000000000000";
    let pinned = if data.first().copied().unwrap_or(0) & 1 == 0 {
        "QSCFP-ffffffffffffffffffffffffffffffff"
    } else {
        seen
    };
    match trusted_pin_reject_reason(pinned, seen) {
        Some(reason) => BindingFuzzOutcome {
            category,
            kind: BindingFuzzOutcomeKind::IdentityRejected,
            frame_kind: None,
            reason,
        },
        None => BindingFuzzOutcome {
            category,
            kind: BindingFuzzOutcomeKind::AcceptedFrame,
            frame_kind: None,
            reason: "trusted_pin_matched",
        },
    }
}

fn frame_type(frame_kind: BindingFuzzFrameKind) -> u8 {
    match frame_kind {
        BindingFuzzFrameKind::A1 => HS_TYPE_INIT,
        BindingFuzzFrameKind::B1 => HS_TYPE_RESP,
        BindingFuzzFrameKind::A2 => HS_TYPE_CONFIRM,
    }
}

fn explicit_blocks_match(a: Option<&[u8]>, b: Option<&[u8]>) -> bool {
    match (a, b) {
        (None, None) => true,
        (Some(a), Some(b)) => a == b,
        _ => false,
    }
}

fn kem_public_key_len() -> usize {
    runtime_pq_kem_public_key_bytes()
}

fn kem_ciphertext_len() -> usize {
    runtime_pq_kem_ciphertext_bytes()
}

fn sig_public_key_len() -> usize {
    runtime_pq_sig_public_key_bytes()
}

fn sig_signature_len() -> usize {
    runtime_pq_sig_signature_bytes()
}
