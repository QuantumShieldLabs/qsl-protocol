#![allow(unexpected_cfgs)]

use super::{
    cmd::HandshakeSuiteMode, config_dir, emit_marker, enforce_peer_not_blocked,
    enforce_safe_parents, fs, hex_encode, identity_fingerprint_from_pk, identity_marker_display,
    identity_peer_status, identity_pin_matches_seen, identity_read_pin, identity_read_sig_pin,
    identity_self_kem_keypair, init_from_base_handshake, kmac_out, print_error_marker,
    qsp_send_ready_tuple, qsp_session_load, qsp_session_store, relay_peer_route_token,
    relay_self_inbox_route_token, require_unlocked, resolve_peer_device_target,
    runtime_pq_kem_ciphertext_bytes, runtime_pq_kem_keypair, runtime_pq_kem_public_key_bytes,
    runtime_pq_sig_keypair, runtime_pq_sig_public_key_bytes, runtime_pq_sig_signature_bytes,
    transport, vault, vault_unlocked, Deserialize, ErrorCode, Hash, IdentityKeypair, OsRng, Path,
    PathBuf, PqKem768, PqSigMldsa65, RngCore, Serialize, StdCrypto, Suite2SessionState, X25519Dh,
    X25519Priv, X25519Pub, IDENTITY_FP_PREFIX, SUITE2_PROTOCOL_VERSION, SUITE2_SUITE_ID,
};

const HS_MAGIC: &[u8; 4] = b"QHSM";
const HS_VERSION_LEGACY: u16 = 1;
const HS_VERSION_V2: u16 = 2;
const HS_TYPE_INIT: u8 = 1;
const HS_TYPE_RESP: u8 = 2;
const HS_TYPE_CONFIRM: u8 = 3;
const HS_PARAM_BLOCK_MAX: usize = 64;
#[cfg(not(qsc_binding_fuzz_helper))]
const HS_PARAM_SUITE_CONTEXT: u16 = 0x0001;
#[cfg(not(qsc_binding_fuzz_helper))]
const HS_PARAM_FLAG_CRITICAL: u8 = 0x01;
const HS_SUITE_CONTEXT_BLOCK: [u8; 9] = [0x00, 0x01, 0x01, 0x00, 0x04, 0x05, 0x00, 0x00, 0x02];
const HS_SUITE2_PROTOCOL_VERSION_WIRE: u16 = 0x0500;
const HS_SUITE2_SUITE_ID_WIRE: u16 = 0x0002;
#[cfg(not(qsc_binding_fuzz_helper))]
const HS_LEGACY_PROTOCOL_VERSION_WIRE: u16 = 0x0403;
#[cfg(not(qsc_binding_fuzz_helper))]
const HS_LEGACY_SUITE_ID_WIRE: u16 = 0x0001;

fn hs_kem_pk_len() -> usize {
    runtime_pq_kem_public_key_bytes()
}

fn hs_kem_ct_len() -> usize {
    runtime_pq_kem_ciphertext_bytes()
}

pub(crate) fn hs_kem_keypair() -> (Vec<u8>, Vec<u8>) {
    runtime_pq_kem_keypair()
}

#[cfg(qsc_rng_failure_test_seam)]
pub(crate) fn hs_kem_keypair_with_failure_label(
    label: &str,
) -> Result<(Vec<u8>, Vec<u8>), &'static str> {
    if hs_rng_failure_forced(label) {
        return Err("rng_failure_forced");
    }
    Ok(runtime_pq_kem_keypair())
}

fn hs_sig_pk_len() -> usize {
    runtime_pq_sig_public_key_bytes()
}

fn hs_sig_sig_len() -> usize {
    runtime_pq_sig_signature_bytes()
}

pub(crate) fn hs_sig_keypair() -> (Vec<u8>, Vec<u8>) {
    runtime_pq_sig_keypair()
}

fn hs_default_role() -> String {
    "initiator".to_string()
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum HsSuiteContext {
    LegacyV1,
    ExplicitV2 {
        block: Vec<u8>,
        protocol_version: u16,
        suite_id: u16,
    },
}

impl HsSuiteContext {
    fn suite2() -> Self {
        Self::ExplicitV2 {
            block: HS_SUITE_CONTEXT_BLOCK.to_vec(),
            protocol_version: HS_SUITE2_PROTOCOL_VERSION_WIRE,
            suite_id: HS_SUITE2_SUITE_ID_WIRE,
        }
    }

    fn explicit_block(&self) -> Option<&[u8]> {
        match self {
            Self::LegacyV1 => None,
            Self::ExplicitV2 { block, .. } => Some(block.as_slice()),
        }
    }

    fn is_explicit(&self) -> bool {
        self.explicit_block().is_some()
    }

    fn as_pending_block(&self) -> Option<Vec<u8>> {
        self.explicit_block().map(|v| v.to_vec())
    }

    fn wire_version(&self) -> u16 {
        match self {
            Self::LegacyV1 => HS_VERSION_LEGACY,
            Self::ExplicitV2 { .. } => HS_VERSION_V2,
        }
    }

    fn mode_label(&self) -> &'static str {
        match self {
            Self::LegacyV1 => "legacy_v1",
            Self::ExplicitV2 { .. } => "v2_suite_context",
        }
    }
}

#[derive(Clone, Debug)]
struct HsInit {
    suite_context: HsSuiteContext,
    session_id: [u8; 16],
    kem_pk: Vec<u8>,
    sig_pk: Vec<u8>,
    dh_pub: [u8; 32],
}

#[derive(Clone, Debug)]
struct HsResp {
    suite_context: HsSuiteContext,
    session_id: [u8; 16],
    kem_ct: Vec<u8>,
    mac: [u8; 32],
    sig_pk: Vec<u8>,
    sig: Vec<u8>,
    dh_pub: [u8; 32],
}

#[derive(Clone, Debug)]
struct HsConfirm {
    suite_context: HsSuiteContext,
    session_id: [u8; 16],
    mac: [u8; 32],
    sig: Vec<u8>,
}

#[derive(Serialize, Deserialize, Clone)]
struct HandshakePending {
    self_label: String,
    peer: String,
    session_id: [u8; 16],
    kem_sk: Vec<u8>,
    kem_pk: Vec<u8>,
    #[serde(default)]
    dh_sk: Vec<u8>,
    #[serde(default)]
    dh_pub: Vec<u8>,
    #[serde(default)]
    sig_pk: Vec<u8>,
    #[serde(default)]
    peer_fp: Option<String>,
    #[serde(default)]
    peer_sig_fp: Option<String>,
    #[serde(default)]
    peer_sig_pk: Option<Vec<u8>>,
    #[serde(default = "hs_default_role")]
    role: String,
    #[serde(default)]
    confirm_key: Option<[u8; 32]>,
    #[serde(default)]
    transcript_hash: Option<[u8; 32]>,
    #[serde(default)]
    pending_session: Option<Vec<u8>>,
    #[serde(default)]
    suite_context: Option<Vec<u8>>,
}

fn hs_suite_context_for_mode(mode: HandshakeSuiteMode) -> HsSuiteContext {
    match mode {
        HandshakeSuiteMode::LegacyCompat => HsSuiteContext::LegacyV1,
        HandshakeSuiteMode::SuiteRequired => HsSuiteContext::suite2(),
    }
}

fn hs_decode_reason_label(reason: &'static str) -> &'static str {
    if reason.starts_with("REJECT_QSC_HS_") {
        reason
    } else {
        "decode_failed"
    }
}

fn hs_emit_suite_reject(reason: &'static str) {
    emit_marker(
        "handshake_suite_admission",
        Some(reason),
        &[("result", "reject"), ("reason", reason)],
    );
    emit_marker("handshake_reject", None, &[("reason", reason)]);
}

fn hs_emit_decode_reject(reason: &'static str) {
    if reason.starts_with("REJECT_QSC_HS_") {
        hs_emit_suite_reject(reason);
    } else {
        emit_marker(
            "handshake_reject",
            None,
            &[("reason", hs_decode_reason_label(reason))],
        );
    }
}

fn hs_emit_suite_accept(ctx: &HsSuiteContext, compatibility: bool) {
    if compatibility {
        emit_marker(
            "handshake_suite_admission",
            None,
            &[
                ("result", "compatibility_accept"),
                ("mode", "legacy_compat"),
                ("reason", "ACCEPT_QSC_HS_LEGACY_COMPATIBILITY"),
            ],
        );
        return;
    }
    if let HsSuiteContext::ExplicitV2 {
        protocol_version,
        suite_id,
        ..
    } = ctx
    {
        let protocol_s = format!("0x{protocol_version:04x}");
        let suite_s = format!("0x{suite_id:04x}");
        emit_marker(
            "handshake_suite_admission",
            None,
            &[
                ("result", "accept"),
                ("version", "v2"),
                ("protocol_version", protocol_s.as_str()),
                ("suite_id", suite_s.as_str()),
                ("reason", "ACCEPT_QSC_HS_SUITE2"),
            ],
        );
    }
}

fn hs_parse_parameter_block(block: &[u8]) -> Result<HsSuiteContext, &'static str> {
    #[cfg(qsc_binding_fuzz_helper)]
    {
        return hs_suite_context_from_fuzz(crate::adversarial::binding_fuzz::parse_suite_context(
            block,
        )?);
    }
    #[cfg(not(qsc_binding_fuzz_helper))]
    {
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
        if protocol_version == HS_SUITE2_PROTOCOL_VERSION_WIRE
            && suite_id == HS_SUITE2_SUITE_ID_WIRE
        {
            return Ok(HsSuiteContext::ExplicitV2 {
                block: block.to_vec(),
                protocol_version,
                suite_id,
            });
        }
        if protocol_version == HS_SUITE2_PROTOCOL_VERSION_WIRE {
            return Err("REJECT_QSC_HS_SUITE_UNSUPPORTED");
        }
        if protocol_version == HS_LEGACY_PROTOCOL_VERSION_WIRE
            && suite_id == HS_LEGACY_SUITE_ID_WIRE
        {
            return Err("REJECT_QSC_HS_DOWNGRADE");
        }
        Err("REJECT_QSC_HS_INCONSISTENT_TUPLE")
    }
}

#[cfg(qsc_binding_fuzz_helper)]
fn hs_suite_context_from_fuzz(
    ctx: crate::adversarial::binding_fuzz::FuzzSuiteContext,
) -> Result<HsSuiteContext, &'static str> {
    match ctx {
        crate::adversarial::binding_fuzz::FuzzSuiteContext::LegacyV1 => {
            Ok(HsSuiteContext::LegacyV1)
        }
        crate::adversarial::binding_fuzz::FuzzSuiteContext::ExplicitV2 {
            block,
            protocol_version,
            suite_id,
        } => Ok(HsSuiteContext::ExplicitV2 {
            block,
            protocol_version,
            suite_id,
        }),
    }
}

#[cfg(qsc_binding_fuzz_helper)]
fn hs_fuzz_suite_mode(mode: HandshakeSuiteMode) -> crate::adversarial::binding_fuzz::FuzzSuiteMode {
    match mode {
        HandshakeSuiteMode::LegacyCompat => {
            crate::adversarial::binding_fuzz::FuzzSuiteMode::LegacyCompat
        }
        HandshakeSuiteMode::SuiteRequired => {
            crate::adversarial::binding_fuzz::FuzzSuiteMode::SuiteRequired
        }
    }
}

#[cfg(qsc_binding_fuzz_helper)]
fn hs_fuzz_frame_kind(
    frame_type: u8,
) -> Option<crate::adversarial::binding_fuzz::BindingFuzzFrameKind> {
    match frame_type {
        HS_TYPE_INIT => Some(crate::adversarial::binding_fuzz::BindingFuzzFrameKind::A1),
        HS_TYPE_RESP => Some(crate::adversarial::binding_fuzz::BindingFuzzFrameKind::B1),
        HS_TYPE_CONFIRM => Some(crate::adversarial::binding_fuzz::BindingFuzzFrameKind::A2),
        _ => None,
    }
}

fn hs_encode_header(out: &mut Vec<u8>, frame_type: u8, suite_context: &HsSuiteContext) -> bool {
    out.extend_from_slice(HS_MAGIC);
    out.extend_from_slice(&suite_context.wire_version().to_be_bytes());
    out.push(frame_type);
    if let Some(block) = suite_context.explicit_block() {
        if block.len() > HS_PARAM_BLOCK_MAX {
            return false;
        }
        out.extend_from_slice(&(block.len() as u16).to_be_bytes());
        out.extend_from_slice(block);
    }
    true
}

fn hs_decode_header(
    bytes: &[u8],
    frame_type: u8,
    payload_len: usize,
    mode: HandshakeSuiteMode,
    admit_context: bool,
) -> Result<(HsSuiteContext, usize), &'static str> {
    #[cfg(qsc_binding_fuzz_helper)]
    {
        let Some(frame_kind) = hs_fuzz_frame_kind(frame_type) else {
            return Err("handshake_type");
        };
        let header = crate::adversarial::binding_fuzz::decode_header(
            bytes,
            frame_kind,
            payload_len,
            hs_fuzz_suite_mode(mode),
            admit_context,
        )?;
        return Ok((
            hs_suite_context_from_fuzz(header.suite_context().clone())?,
            header.payload_offset(),
        ));
    }
    #[cfg(not(qsc_binding_fuzz_helper))]
    {
        if bytes.len() < 7 {
            return Err("handshake_len");
        }
        if &bytes[0..4] != HS_MAGIC {
            return Err("handshake_magic");
        }
        let ver = u16::from_be_bytes([bytes[4], bytes[5]]);
        if bytes[6] != frame_type {
            return Err("handshake_type");
        }
        match ver {
            HS_VERSION_LEGACY => {
                if mode == HandshakeSuiteMode::SuiteRequired {
                    return Err("REJECT_QSC_HS_LEGACY_REQUIRED");
                }
                if bytes.len() != 7 + payload_len {
                    return Err("handshake_len");
                }
                Ok((HsSuiteContext::LegacyV1, 7))
            }
            HS_VERSION_V2 => {
                if bytes.len() < 9 {
                    return Err("REJECT_QSC_HS_MALFORMED_LENGTH");
                }
                let block_len = u16::from_be_bytes([bytes[7], bytes[8]]) as usize;
                if block_len > HS_PARAM_BLOCK_MAX {
                    return Err("REJECT_QSC_HS_MALFORMED_LENGTH");
                }
                let payload_off = 9 + block_len;
                if bytes.len() != payload_off + payload_len {
                    return Err("REJECT_QSC_HS_MALFORMED_LENGTH");
                }
                let block = &bytes[9..payload_off];
                let suite_context = if admit_context {
                    hs_parse_parameter_block(block)?
                } else {
                    HsSuiteContext::ExplicitV2 {
                        block: block.to_vec(),
                        protocol_version: 0,
                        suite_id: 0,
                    }
                };
                Ok((suite_context, payload_off))
            }
            _ => Err("handshake_version"),
        }
    }
}

fn hs_encode_init(msg: &HsInit) -> Vec<u8> {
    let pk_len = hs_kem_pk_len();
    let sig_pk_len = hs_sig_pk_len();
    if msg.kem_pk.len() != pk_len || msg.sig_pk.len() != sig_pk_len {
        return Vec::new();
    }
    let header_len = 4
        + 2
        + 1
        + msg
            .suite_context
            .explicit_block()
            .map_or(0, |b| 2 + b.len());
    let mut out = Vec::with_capacity(header_len + 16 + pk_len + sig_pk_len + 32);
    if !hs_encode_header(&mut out, HS_TYPE_INIT, &msg.suite_context) {
        return Vec::new();
    }
    out.extend_from_slice(&msg.session_id);
    out.extend_from_slice(&msg.kem_pk);
    out.extend_from_slice(&msg.sig_pk);
    out.extend_from_slice(&msg.dh_pub);
    out
}

fn hs_decode_init(bytes: &[u8], mode: HandshakeSuiteMode) -> Result<HsInit, &'static str> {
    let pk_len = hs_kem_pk_len();
    let sig_pk_len = hs_sig_pk_len();
    let payload_len = 16 + pk_len + sig_pk_len + 32;
    let (suite_context, off) = hs_decode_header(bytes, HS_TYPE_INIT, payload_len, mode, true)?;
    let mut sid = [0u8; 16];
    sid.copy_from_slice(&bytes[off..off + 16]);
    let pk_off = off + 16;
    let kem_pk = bytes[pk_off..(pk_off + pk_len)].to_vec();
    let sig_pk = bytes[(pk_off + pk_len)..(pk_off + pk_len + sig_pk_len)].to_vec();
    let mut dh_pub = [0u8; 32];
    let dh_off = pk_off + pk_len + sig_pk_len;
    dh_pub.copy_from_slice(&bytes[dh_off..dh_off + 32]);
    Ok(HsInit {
        suite_context,
        session_id: sid,
        kem_pk,
        sig_pk,
        dh_pub,
    })
}

fn hs_encode_resp_no_auth(
    session_id: &[u8; 16],
    kem_ct: &[u8],
    sig_pk: &[u8],
    dh_pub: &[u8; 32],
    suite_context: &HsSuiteContext,
) -> Vec<u8> {
    let mut out = Vec::with_capacity(
        4 + 2
            + 1
            + suite_context.explicit_block().map_or(0, |b| 2 + b.len())
            + 16
            + kem_ct.len()
            + sig_pk.len()
            + 32,
    );
    if !hs_encode_header(&mut out, HS_TYPE_RESP, suite_context) {
        return Vec::new();
    }
    out.extend_from_slice(session_id);
    out.extend_from_slice(kem_ct);
    out.extend_from_slice(sig_pk);
    out.extend_from_slice(dh_pub);
    out
}

fn hs_encode_resp(msg: &HsResp) -> Vec<u8> {
    let ct_len = hs_kem_ct_len();
    let sig_pk_len = hs_sig_pk_len();
    let sig_len = hs_sig_sig_len();
    if msg.kem_ct.len() != ct_len || msg.sig_pk.len() != sig_pk_len || msg.sig.len() != sig_len {
        return Vec::new();
    }
    let header_len = 4
        + 2
        + 1
        + msg
            .suite_context
            .explicit_block()
            .map_or(0, |b| 2 + b.len());
    let mut out = Vec::with_capacity(header_len + 16 + ct_len + 32 + sig_pk_len + sig_len + 32);
    if !hs_encode_header(&mut out, HS_TYPE_RESP, &msg.suite_context) {
        return Vec::new();
    }
    out.extend_from_slice(&msg.session_id);
    out.extend_from_slice(&msg.kem_ct);
    out.extend_from_slice(&msg.mac);
    out.extend_from_slice(&msg.sig_pk);
    out.extend_from_slice(&msg.sig);
    out.extend_from_slice(&msg.dh_pub);
    out
}

fn hs_decode_resp_with_admission(
    bytes: &[u8],
    mode: HandshakeSuiteMode,
    admit_context: bool,
) -> Result<HsResp, &'static str> {
    let ct_len = hs_kem_ct_len();
    let sig_pk_len = hs_sig_pk_len();
    let sig_len = hs_sig_sig_len();
    let payload_len = 16 + ct_len + 32 + sig_pk_len + sig_len + 32;
    let (suite_context, off) =
        hs_decode_header(bytes, HS_TYPE_RESP, payload_len, mode, admit_context)?;
    let mut sid = [0u8; 16];
    sid.copy_from_slice(&bytes[off..off + 16]);
    let kem_off = off + 16;
    let kem_ct = bytes[kem_off..(kem_off + ct_len)].to_vec();
    let mut mac = [0u8; 32];
    let mac_off = kem_off + ct_len;
    mac.copy_from_slice(&bytes[mac_off..(mac_off + 32)]);
    let sig_pk_off = mac_off + 32;
    let sig_off = sig_pk_off + sig_pk_len;
    let sig_pk = bytes[sig_pk_off..sig_off].to_vec();
    let sig = bytes[sig_off..(sig_off + sig_len)].to_vec();
    let mut dh_pub = [0u8; 32];
    dh_pub.copy_from_slice(&bytes[(sig_off + sig_len)..(sig_off + sig_len + 32)]);
    Ok(HsResp {
        suite_context,
        session_id: sid,
        kem_ct,
        mac,
        sig_pk,
        sig,
        dh_pub,
    })
}

fn hs_decode_resp_pending(bytes: &[u8], mode: HandshakeSuiteMode) -> Result<HsResp, &'static str> {
    hs_decode_resp_with_admission(bytes, mode, false)
}

fn hs_encode_confirm(msg: &HsConfirm) -> Vec<u8> {
    let sig_len = hs_sig_sig_len();
    if msg.sig.len() != sig_len {
        return Vec::new();
    }
    let header_len = 4
        + 2
        + 1
        + msg
            .suite_context
            .explicit_block()
            .map_or(0, |b| 2 + b.len());
    let mut out = Vec::with_capacity(header_len + 16 + 32 + sig_len);
    if !hs_encode_header(&mut out, HS_TYPE_CONFIRM, &msg.suite_context) {
        return Vec::new();
    }
    out.extend_from_slice(&msg.session_id);
    out.extend_from_slice(&msg.mac);
    out.extend_from_slice(&msg.sig);
    out
}

fn hs_decode_confirm_with_admission(
    bytes: &[u8],
    mode: HandshakeSuiteMode,
    admit_context: bool,
) -> Result<HsConfirm, &'static str> {
    let sig_len = hs_sig_sig_len();
    let payload_len = 16 + 32 + sig_len;
    let (suite_context, off) =
        hs_decode_header(bytes, HS_TYPE_CONFIRM, payload_len, mode, admit_context)?;
    let mut sid = [0u8; 16];
    sid.copy_from_slice(&bytes[off..off + 16]);
    let mut mac = [0u8; 32];
    mac.copy_from_slice(&bytes[off + 16..off + 48]);
    let sig = bytes[off + 48..(off + 48 + sig_len)].to_vec();
    Ok(HsConfirm {
        suite_context,
        session_id: sid,
        mac,
        sig,
    })
}

fn hs_decode_confirm(bytes: &[u8], mode: HandshakeSuiteMode) -> Result<HsConfirm, &'static str> {
    hs_decode_confirm_with_admission(bytes, mode, true)
}

fn hs_decode_confirm_pending(
    bytes: &[u8],
    mode: HandshakeSuiteMode,
) -> Result<HsConfirm, &'static str> {
    hs_decode_confirm_with_admission(bytes, mode, false)
}

fn emit_peer_mismatch(peer: &str, pinned_fp: &str, seen_fp: &str) {
    let pinned_display = identity_marker_display(pinned_fp);
    let seen_display = identity_marker_display(seen_fp);
    emit_marker(
        "identity_mismatch",
        None,
        &[
            ("peer", peer),
            ("pinned_fp", pinned_display.as_str()),
            ("seen_fp", seen_display.as_str()),
        ],
    );
    emit_marker("error", Some("peer_mismatch"), &[("peer", peer)]);
}

#[cfg(qsc_rng_failure_test_seam)]
fn hs_rng_failure_forced(label: &str) -> bool {
    std::env::var("QSC_RNG_FAILURE_TEST_SEAM")
        .ok()
        .map(|v| v == label || v == "all")
        .unwrap_or(false)
}

#[cfg(qsc_rng_failure_test_seam)]
fn hs_rand_bytes(label: &str, len: usize) -> Result<Vec<u8>, &'static str> {
    if hs_rng_failure_forced(label) {
        return Err("rng_failure_forced");
    }
    let mut out = vec![0u8; len];
    let mut rng = OsRng;
    rng.fill_bytes(&mut out);
    Ok(out)
}

#[cfg(not(qsc_rng_failure_test_seam))]
fn hs_rand_bytes(_label: &str, len: usize) -> Vec<u8> {
    let mut out = vec![0u8; len];
    let mut rng = OsRng;
    rng.fill_bytes(&mut out);
    out
}

#[cfg(qsc_rng_failure_test_seam)]
fn hs_session_id(label: &str) -> Result<[u8; 16], &'static str> {
    let bytes = hs_rand_bytes(label, 16)?;
    let mut sid = [0u8; 16];
    sid.copy_from_slice(&bytes[..16]);
    Ok(sid)
}

#[cfg(not(qsc_rng_failure_test_seam))]
fn hs_session_id(label: &str) -> [u8; 16] {
    let bytes = hs_rand_bytes(label, 16);
    let mut sid = [0u8; 16];
    sid.copy_from_slice(&bytes[..16]);
    sid
}

// Constant-time equality for fixed-length 32-byte MAC/tag values. Bit-for-bit
// equal to `a == b` for all inputs, but does not short-circuit on the first
// differing byte, closing the handshake MAC-comparison timing side-channel
// (ENG-0003). Accept/reject semantics and wire format are unchanged.
fn hs_ct_eq_32(a: &[u8; 32], b: &[u8; 32]) -> bool {
    let mut diff = 0u8;
    for i in 0..32 {
        diff |= a[i] ^ b[i];
    }
    diff == 0
}

fn hs_transcript_mac(pq_init_ss: &[u8; 32], a1: &[u8], b1_no_mac: &[u8]) -> [u8; 32] {
    let c = StdCrypto;
    let mut data = Vec::with_capacity(a1.len() + b1_no_mac.len());
    data.extend_from_slice(a1);
    data.extend_from_slice(b1_no_mac);
    kmac_out::<32>(&c, pq_init_ss, "QSC.HS.TRANSCRIPT", &data)
}

fn hs_transcript_hash(pq_init_ss: &[u8; 32], a1: &[u8], b1_no_mac: &[u8]) -> [u8; 32] {
    let c = StdCrypto;
    let mut data = Vec::with_capacity(a1.len() + b1_no_mac.len());
    data.extend_from_slice(a1);
    data.extend_from_slice(b1_no_mac);
    kmac_out::<32>(&c, pq_init_ss, "QSC.HS.TRANSCRIPT.H", &data)
}

fn hs_context_len(ctx: &HsSuiteContext) -> usize {
    ctx.explicit_block().map_or(0, |b| 2 + b.len())
}

fn hs_append_key_context(data: &mut Vec<u8>, ctx: &HsSuiteContext) {
    if let Some(block) = ctx.explicit_block() {
        data.extend_from_slice(&(block.len() as u16).to_be_bytes());
        data.extend_from_slice(block);
    }
}

fn hs_pq_init_ss(ss_pq: &[u8], session_id: &[u8; 16], ctx: &HsSuiteContext) -> [u8; 32] {
    let c = StdCrypto;
    let mut data = Vec::with_capacity(16 + 1 + hs_context_len(ctx));
    data.extend_from_slice(session_id);
    data.push(0x01);
    hs_append_key_context(&mut data, ctx);
    kmac_out::<32>(&c, ss_pq, "QSC.HS.PQ", &data)
}

fn hs_ephemeral_keypair() -> ([u8; 32], [u8; 32]) {
    let c = StdCrypto;
    let (sk, pk) = c.keypair();
    (sk.0, pk.0)
}

fn hs_dh_init_from_shared(
    dh_shared: &[u8; 32],
    session_id: &[u8; 16],
    ctx: &HsSuiteContext,
) -> [u8; 32] {
    let c = StdCrypto;
    let mut data = Vec::with_capacity(16 + 1 + hs_context_len(ctx));
    data.extend_from_slice(session_id);
    data.push(0x02);
    hs_append_key_context(&mut data, ctx);
    kmac_out::<32>(&c, dh_shared, "QSC.HS.DHINIT", &data)
}

fn hs_dh_shared(self_sk: &[u8], peer_pub: &[u8]) -> Result<[u8; 32], &'static str> {
    if self_sk.len() != 32 || peer_pub.len() != 32 {
        return Err("handshake_dh_len");
    }
    let mut sk = [0u8; 32];
    sk.copy_from_slice(self_sk);
    let mut pk = [0u8; 32];
    pk.copy_from_slice(peer_pub);
    let c = StdCrypto;
    Ok(c.dh(&X25519Priv(sk), &X25519Pub(pk)))
}

fn hs_dh_pub_from_bytes(bytes: &[u8]) -> Result<[u8; 32], &'static str> {
    if bytes.len() != 32 {
        return Err("handshake_dh_len");
    }
    let mut out = [0u8; 32];
    out.copy_from_slice(bytes);
    Ok(out)
}

fn hs_dh_pub_is_all_zero(dh_pub: &[u8; 32]) -> bool {
    dh_pub.iter().all(|b| *b == 0)
}

fn hs_confirm_key(
    pq_init_ss: &[u8; 32],
    session_id: &[u8; 16],
    th: &[u8; 32],
    ctx: &HsSuiteContext,
) -> [u8; 32] {
    let c = StdCrypto;
    let mut data = Vec::with_capacity(16 + 32 + hs_context_len(ctx));
    data.extend_from_slice(session_id);
    data.extend_from_slice(th);
    hs_append_key_context(&mut data, ctx);
    kmac_out::<32>(&c, pq_init_ss, "QSC.HS.CONFIRM", &data)
}

fn hs_confirm_mac(
    k_confirm: &[u8; 32],
    session_id: &[u8; 16],
    th: &[u8; 32],
    ctx: &HsSuiteContext,
) -> [u8; 32] {
    let c = StdCrypto;
    let mut data = Vec::with_capacity(16 + 32 + 2 + hs_context_len(ctx));
    data.extend_from_slice(session_id);
    data.extend_from_slice(th);
    data.extend_from_slice(b"A2");
    hs_append_key_context(&mut data, ctx);
    kmac_out::<32>(&c, k_confirm, "QSC.HS.A2", &data)
}

fn hs_sig_fingerprint(sig_pk: &[u8]) -> String {
    let c = StdCrypto;
    let hash = c.sha512(sig_pk);
    format!("{}{}", IDENTITY_FP_PREFIX, hex_encode(&hash[..16]))
}

fn hs_sig_msg_b1(session_id: &[u8; 16], th: &[u8; 32]) -> Vec<u8> {
    let mut data = Vec::with_capacity(4 + 2 + 1 + 16 + 32);
    data.extend_from_slice(b"QSC.HS.SIG.B1");
    data.extend_from_slice(session_id);
    data.extend_from_slice(th);
    data
}

fn hs_sig_msg_a2(session_id: &[u8; 16], th: &[u8; 32], cmac: &[u8; 32]) -> Vec<u8> {
    let mut data = Vec::with_capacity(4 + 2 + 1 + 16 + 32 + 32);
    data.extend_from_slice(b"QSC.HS.SIG.A2");
    data.extend_from_slice(session_id);
    data.extend_from_slice(th);
    data.extend_from_slice(cmac);
    data
}

fn hs_sig_verify(sig_pk: &[u8], msg: &[u8], sig: &[u8], reason: &str) -> Result<(), &'static str> {
    let c = StdCrypto;
    match c.verify(sig_pk, msg, sig) {
        Ok(true) => {
            emit_marker(
                "sig_status",
                None,
                &[("ok", "true"), ("alg", "ML-DSA-65"), ("reason", reason)],
            );
            Ok(())
        }
        Ok(false) => {
            emit_marker(
                "sig_status",
                Some("sig_invalid"),
                &[("ok", "false"), ("alg", "ML-DSA-65"), ("reason", reason)],
            );
            Err("sig_invalid")
        }
        Err(_) => {
            emit_marker(
                "sig_status",
                Some("sig_invalid"),
                &[("ok", "false"), ("alg", "ML-DSA-65"), ("reason", reason)],
            );
            Err("sig_invalid")
        }
    }
}

fn hs_require_primary_identity_pin<F>(
    peer: &str,
    seen_fp: &str,
    read_pin: F,
) -> Result<(), &'static str>
where
    F: Fn(&str) -> Result<Option<String>, ErrorCode>,
{
    match read_pin(peer) {
        Ok(Some(pinned)) => {
            #[cfg(qsc_binding_fuzz_helper)]
            let pin_matches = {
                let _canonical_pin_matches = identity_pin_matches_seen(pinned.as_str(), seen_fp);
                crate::adversarial::binding_fuzz::trusted_pin_matches_seen(pinned.as_str(), seen_fp)
            };
            #[cfg(not(qsc_binding_fuzz_helper))]
            let pin_matches = identity_pin_matches_seen(pinned.as_str(), seen_fp);
            if !pin_matches {
                emit_peer_mismatch(peer, pinned.as_str(), seen_fp);
                emit_marker("handshake_reject", None, &[("reason", "peer_mismatch")]);
                return Err("peer_mismatch");
            }
            let fp_display = identity_marker_display(seen_fp);
            emit_marker(
                "identity_ok",
                None,
                &[("peer", peer), ("fp", fp_display.as_str())],
            );
            Ok(())
        }
        Ok(None) => {
            let fp_display = identity_marker_display(seen_fp);
            emit_marker(
                "identity_unknown",
                None,
                &[("peer", peer), ("seen_fp", fp_display.as_str())],
            );
            emit_marker("handshake_reject", None, &[("reason", "identity_unknown")]);
            Err("identity_unknown")
        }
        Err(_) => {
            emit_marker(
                "handshake_reject",
                None,
                &[("reason", "identity_pin_failed")],
            );
            Err("identity_pin_failed")
        }
    }
}

fn hs_check_optional_identity_pin<F>(
    peer: &str,
    seen_fp: &str,
    read_pin: F,
) -> Result<(), &'static str>
where
    F: Fn(&str) -> Result<Option<String>, ErrorCode>,
{
    match read_pin(peer) {
        Ok(Some(pinned)) => {
            #[cfg(qsc_binding_fuzz_helper)]
            let pin_matches = {
                let _canonical_pin_matches = identity_pin_matches_seen(pinned.as_str(), seen_fp);
                crate::adversarial::binding_fuzz::trusted_pin_matches_seen(pinned.as_str(), seen_fp)
            };
            #[cfg(not(qsc_binding_fuzz_helper))]
            let pin_matches = identity_pin_matches_seen(pinned.as_str(), seen_fp);
            if !pin_matches {
                emit_peer_mismatch(peer, pinned.as_str(), seen_fp);
                emit_marker("handshake_reject", None, &[("reason", "peer_mismatch")]);
                return Err("peer_mismatch");
            }
            let fp_display = identity_marker_display(seen_fp);
            emit_marker(
                "identity_ok",
                None,
                &[("peer", peer), ("fp", fp_display.as_str())],
            );
            Ok(())
        }
        Ok(None) => Ok(()),
        Err(_) => {
            emit_marker(
                "handshake_reject",
                None,
                &[("reason", "identity_pin_failed")],
            );
            Err("identity_pin_failed")
        }
    }
}

fn hs_build_session(
    authenticated: bool,
    role_is_a: bool,
    session_id: [u8; 16],
    dh_init: [u8; 32],
    pq_init_ss: [u8; 32],
    dh_self_pub: [u8; 32],
    dh_peer_pub: [u8; 32],
) -> Result<Suite2SessionState, &'static str> {
    let c = StdCrypto;
    init_from_base_handshake(
        &c,
        role_is_a,
        SUITE2_PROTOCOL_VERSION,
        SUITE2_SUITE_ID,
        &session_id,
        &dh_init,
        &pq_init_ss,
        &dh_self_pub,
        &dh_peer_pub,
        authenticated,
    )
}

fn hs_pending_legacy_path(dir: &Path, self_label: &str, peer: &str) -> PathBuf {
    dir.join(format!("handshake_pending_{}_{}.json", self_label, peer))
}

fn hs_pending_secret_key(self_label: &str, peer: &str) -> String {
    format!("handshake.pending.{}.{}", self_label, peer)
}

fn hs_pending_load(self_label: &str, peer: &str) -> Result<Option<HandshakePending>, ErrorCode> {
    let secret_key = hs_pending_secret_key(self_label, peer);
    match vault::secret_get(&secret_key) {
        Ok(Some(v)) if !v.is_empty() => {
            let pending: HandshakePending =
                serde_json::from_str(&v).map_err(|_| ErrorCode::ParseFailed)?;
            return Ok(Some(pending));
        }
        Ok(_) => {}
        Err("vault_missing" | "vault_locked") => return Err(ErrorCode::IdentitySecretUnavailable),
        Err(_) => return Err(ErrorCode::IoReadFailed),
    }

    let (dir, source) = config_dir()?;
    let path = hs_pending_legacy_path(&dir, self_label, peer);
    if !path.exists() {
        return Ok(None);
    }
    enforce_safe_parents(&path, source)?;
    let bytes = fs::read(&path).map_err(|_| ErrorCode::IoReadFailed)?;
    let pending: HandshakePending =
        serde_json::from_slice(&bytes).map_err(|_| ErrorCode::ParseFailed)?;
    let v = serde_json::to_string(&pending).map_err(|_| ErrorCode::IoWriteFailed)?;
    match vault::secret_set(&secret_key, &v) {
        Ok(()) => {
            let _ = fs::remove_file(&path);
        }
        Err("vault_missing" | "vault_locked") => return Err(ErrorCode::IdentitySecretUnavailable),
        Err(_) => return Err(ErrorCode::IoWriteFailed),
    }
    Ok(Some(pending))
}

fn hs_pending_store(pending: &HandshakePending) -> Result<(), ErrorCode> {
    let key = hs_pending_secret_key(&pending.self_label, &pending.peer);
    let value = serde_json::to_string(pending).map_err(|_| ErrorCode::IoWriteFailed)?;
    match vault::secret_set(&key, &value) {
        Ok(()) => Ok(()),
        Err("vault_missing" | "vault_locked") => Err(ErrorCode::IdentitySecretUnavailable),
        Err(_) => Err(ErrorCode::IoWriteFailed),
    }
}

fn hs_pending_clear(self_label: &str, peer: &str) -> Result<(), ErrorCode> {
    let key = hs_pending_secret_key(self_label, peer);
    match vault::secret_set(&key, "") {
        Ok(()) => {}
        Err("vault_missing" | "vault_locked") => return Err(ErrorCode::IdentitySecretUnavailable),
        Err(_) => return Err(ErrorCode::IoWriteFailed),
    }
    let (dir, source) = config_dir()?;
    let path = hs_pending_legacy_path(&dir, self_label, peer);
    enforce_safe_parents(&path, source)?;
    let _ = fs::remove_file(path);
    Ok(())
}

fn hs_pending_suite_context(pending: &HandshakePending) -> Result<HsSuiteContext, &'static str> {
    match pending.suite_context.as_deref() {
        Some(block) => hs_parse_parameter_block(block),
        None => Ok(HsSuiteContext::LegacyV1),
    }
}

fn hs_contexts_match(a: &HsSuiteContext, b: &HsSuiteContext) -> bool {
    match (a, b) {
        (HsSuiteContext::LegacyV1, HsSuiteContext::LegacyV1) => true,
        (
            HsSuiteContext::ExplicitV2 { block: a_block, .. },
            HsSuiteContext::ExplicitV2 { block: b_block, .. },
        ) => a_block == b_block,
        _ => false,
    }
}

fn hs_reject_context_mismatch() {
    hs_emit_suite_reject("REJECT_QSC_HS_CONTEXT_MISMATCH");
}

fn hs_reject_key_context() {
    hs_emit_suite_reject("REJECT_QSC_HS_KEY_CONTEXT");
}

fn hs_reject_replay() {
    hs_emit_suite_reject("REJECT_QSC_HS_REPLAY");
}

fn hs_zero32(v: &[u8; 32]) -> bool {
    v.iter().all(|b| *b == 0)
}

fn hs_send_ready_from_session(st: &Suite2SessionState) -> bool {
    !(hs_zero32(&st.send.ck_ec) || hs_zero32(&st.send.ck_pq))
}

fn hs_status_truth(st: &Suite2SessionState) -> (&'static str, &'static str, Option<&'static str>) {
    if !hs_send_ready_from_session(st) {
        return ("established_recv_only", "yes", Some("chainkey_unset"));
    }
    if st.recv.nr == 0 {
        return ("awaiting_peer_confirm", "no", None);
    }
    ("established", "yes", None)
}

pub(crate) fn handshake_status(peer: Option<&str>) {
    if !require_unlocked("handshake_status") {
        return;
    }
    let peer_label = peer.unwrap_or("peer-0");
    if let Err(code) = enforce_peer_not_blocked(peer_label) {
        print_error_marker(code);
    }
    let (peer_fp, pinned) = identity_peer_status(peer_label);
    let pinned_s = if pinned { "true" } else { "false" };
    let (send_ready, send_ready_reason) = qsp_send_ready_tuple(peer_label);
    let send_ready_s = if send_ready { "yes" } else { "no" };
    match qsp_session_load(peer_label) {
        Ok(Some(st)) => {
            let (status, peer_confirmed, local_reason) = hs_status_truth(&st);
            if let Some(reason) = local_reason {
                emit_marker(
                    "handshake_status",
                    None,
                    &[
                        ("status", status),
                        ("peer", peer_label),
                        ("peer_fp", peer_fp.as_str()),
                        ("pinned", pinned_s),
                        ("peer_confirmed", peer_confirmed),
                        ("send_ready", send_ready_s),
                        ("send_ready_reason", reason),
                    ],
                );
            } else {
                emit_marker(
                    "handshake_status",
                    None,
                    &[
                        ("status", status),
                        ("peer", peer_label),
                        ("peer_fp", peer_fp.as_str()),
                        ("pinned", pinned_s),
                        ("peer_confirmed", peer_confirmed),
                        ("send_ready", send_ready_s),
                    ],
                );
            }
        }
        Ok(None) => {
            emit_marker(
                "handshake_status",
                None,
                &[
                    ("status", "no_session"),
                    ("peer", peer_label),
                    ("peer_fp", peer_fp.as_str()),
                    ("pinned", pinned_s),
                    ("peer_confirmed", "no"),
                    ("send_ready", send_ready_s),
                    ("send_ready_reason", send_ready_reason),
                ],
            );
        }
        Err(_) => {
            emit_marker(
                "handshake_status",
                Some("handshake_status_failed"),
                &[
                    ("peer", peer_label),
                    ("peer_fp", peer_fp.as_str()),
                    ("pinned", pinned_s),
                    ("peer_confirmed", "unknown"),
                    ("send_ready", send_ready_s),
                    ("send_ready_reason", send_ready_reason),
                ],
            );
        }
    }
}

fn perform_handshake_init_with_route(
    self_label: &str,
    peer: &str,
    relay: &str,
    route_token: &str,
    suite_mode: HandshakeSuiteMode,
) -> Result<(), &'static str> {
    enforce_peer_not_blocked(peer)?;
    let peer_fp = match identity_read_pin(peer) {
        Ok(Some(peer_fp)) => peer_fp,
        Ok(None) => {
            emit_marker(
                "identity_unknown",
                None,
                &[("peer", peer), ("seen_fp", "unknown")],
            );
            emit_marker("handshake_reject", None, &[("reason", "identity_unknown")]);
            return Err("identity_unknown");
        }
        Err(_) => {
            emit_marker(
                "handshake_reject",
                None,
                &[("reason", "identity_pin_failed")],
            );
            return Err("identity_pin_failed");
        }
    };
    let IdentityKeypair {
        kem_pk,
        kem_sk,
        sig_pk,
        sig_sk: _,
    } = identity_self_kem_keypair(self_label).map_err(|e| e.as_str())?;
    #[cfg(qsc_rng_failure_test_seam)]
    let sid = hs_session_id("QSC.HS.SID")?;
    #[cfg(not(qsc_rng_failure_test_seam))]
    let sid = hs_session_id("QSC.HS.SID");
    let (dh_sk, dh_pub) = hs_ephemeral_keypair();
    let suite_context = hs_suite_context_for_mode(suite_mode);
    let msg = HsInit {
        suite_context: suite_context.clone(),
        session_id: sid,
        kem_pk: kem_pk.clone(),
        sig_pk: sig_pk.clone(),
        dh_pub,
    };
    let bytes = hs_encode_init(&msg);
    if bytes.is_empty() {
        return Err("handshake_init_encode_failed");
    }
    let pending = HandshakePending {
        self_label: self_label.to_string(),
        peer: peer.to_string(),
        session_id: sid,
        kem_sk,
        kem_pk,
        dh_sk: dh_sk.to_vec(),
        dh_pub: dh_pub.to_vec(),
        sig_pk,
        peer_sig_fp: None,
        peer_sig_pk: None,
        peer_fp: Some(peer_fp),
        role: "initiator".to_string(),
        confirm_key: None,
        transcript_hash: None,
        pending_session: None,
        suite_context: suite_context.as_pending_block(),
    };
    hs_pending_store(&pending).map_err(|_| "handshake_pending_store_failed")?;
    emit_marker(
        "handshake_start",
        None,
        &[("role", "initiator"), ("peer", peer)],
    );
    let size_s = bytes.len().to_string();
    let pk_len_s = hs_kem_pk_len().to_string();
    let sig_pk_len_s = hs_sig_pk_len().to_string();
    let hs_version_s = suite_context.wire_version().to_string();
    emit_marker(
        "handshake_send",
        None,
        &[
            ("msg", "A1"),
            ("size", size_s.as_str()),
            ("kem_pk_len", pk_len_s.as_str()),
            ("sig_pk_len", sig_pk_len_s.as_str()),
            ("hs_version", hs_version_s.as_str()),
            ("suite_context", suite_context.mode_label()),
        ],
    );
    transport::relay_inbox_push(relay, route_token, &bytes)?;
    Ok(())
}

fn handshake_init_with_route(
    self_label: &str,
    peer: &str,
    relay: &str,
    route_token: &str,
    suite_mode: HandshakeSuiteMode,
) {
    if !require_unlocked("handshake_init") {
        return;
    }
    if let Err(code) =
        perform_handshake_init_with_route(self_label, peer, relay, route_token, suite_mode)
    {
        print_error_marker(code);
    }
}

pub(crate) fn handshake_init_with_suite_mode(
    self_label: &str,
    peer: &str,
    relay: &str,
    suite_mode: HandshakeSuiteMode,
) {
    if !vault_unlocked() {
        require_unlocked("handshake_init");
    }
    let peer_channel = resolve_peer_device_target(peer, false)
        .map(|v| v.channel)
        .unwrap_or_else(|_| peer.to_string());
    let route_token = relay_peer_route_token(peer).unwrap_or_else(|code| print_error_marker(code));
    handshake_init_with_route(
        self_label,
        peer_channel.as_str(),
        relay,
        route_token.as_str(),
        suite_mode,
    );
}

pub(crate) fn handshake_init(self_label: &str, peer: &str, relay: &str) {
    handshake_init_with_suite_mode(self_label, peer, relay, HandshakeSuiteMode::LegacyCompat);
}

fn perform_handshake_poll_with_tokens(
    self_label: &str,
    peer: &str,
    relay: &str,
    inbox_route_token: &str,
    peer_route_token: &str,
    max: usize,
    suite_mode: HandshakeSuiteMode,
) -> Result<(), &'static str> {
    enforce_peer_not_blocked(peer)?;
    let items = match transport::relay_inbox_pull(relay, inbox_route_token, max) {
        Ok(v) => v,
        Err(code) => {
            emit_marker("handshake_recv", Some(code), &[("ok", "false")]);
            return Err(code);
        }
    };
    if items.is_empty() {
        emit_marker("handshake_recv", None, &[("msg", "none"), ("ok", "true")]);
        return Ok(());
    }

    if let Some(pending) = hs_pending_load(self_label, peer).map_err(|e| e.as_str())? {
        emit_marker(
            "handshake_pending",
            None,
            &[
                ("peer", peer),
                ("present", "true"),
                ("role", pending.role.as_str()),
            ],
        );
        if pending.role == "initiator" {
            let pending_suite_context = match hs_pending_suite_context(&pending) {
                Ok(v) => v,
                Err(_) => {
                    let _ = hs_pending_clear(self_label, peer);
                    hs_reject_key_context();
                    return Ok(());
                }
            };
            for item in items {
                match hs_decode_resp_pending(&item.data, suite_mode) {
                    Ok(resp) => {
                        if resp.session_id != pending.session_id {
                            emit_marker(
                                "handshake_reject",
                                None,
                                &[("reason", "session_id_mismatch")],
                            );
                            continue;
                        }
                        if suite_mode == HandshakeSuiteMode::SuiteRequired
                            && !pending_suite_context.is_explicit()
                        {
                            let _ = hs_pending_clear(self_label, peer);
                            hs_reject_key_context();
                            return Ok(());
                        }
                        if !hs_contexts_match(&pending_suite_context, &resp.suite_context) {
                            let _ = hs_pending_clear(self_label, peer);
                            hs_reject_context_mismatch();
                            return Ok(());
                        }
                        let active_suite_context = pending_suite_context.clone();
                        let c = StdCrypto;
                        let ss_pq = match c.decap(&pending.kem_sk, &resp.kem_ct) {
                            Ok(v) => v,
                            Err(_) => {
                                emit_marker(
                                    "handshake_reject",
                                    None,
                                    &[("reason", "pq_decap_failed")],
                                );
                                return Ok(());
                            }
                        };
                        let pq_init_ss =
                            hs_pq_init_ss(&ss_pq, &resp.session_id, &active_suite_context);
                        if hs_dh_pub_is_all_zero(&resp.dh_pub) {
                            emit_marker("handshake_reject", None, &[("reason", "dh_pub_invalid")]);
                            return Ok(());
                        }
                        let dh_self_pub = match hs_dh_pub_from_bytes(&pending.dh_pub) {
                            Ok(v) => v,
                            Err(_) => {
                                emit_marker("handshake_reject", None, &[("reason", "dh_missing")]);
                                return Ok(());
                            }
                        };
                        let dh_shared = match hs_dh_shared(&pending.dh_sk, &resp.dh_pub) {
                            Ok(v) => v,
                            Err(_) => {
                                emit_marker("handshake_reject", None, &[("reason", "dh_failed")]);
                                return Ok(());
                            }
                        };
                        let dh_init_arr = hs_dh_init_from_shared(
                            &dh_shared,
                            &resp.session_id,
                            &active_suite_context,
                        );
                        let dh_peer_pub = resp.dh_pub;
                        let a1 = hs_encode_init(&HsInit {
                            suite_context: pending_suite_context.clone(),
                            session_id: pending.session_id,
                            kem_pk: pending.kem_pk.clone(),
                            sig_pk: pending.sig_pk.clone(),
                            dh_pub: dh_self_pub,
                        });
                        let b1_no_auth = hs_encode_resp_no_auth(
                            &resp.session_id,
                            &resp.kem_ct,
                            &resp.sig_pk,
                            &resp.dh_pub,
                            &active_suite_context,
                        );
                        let mac = hs_transcript_mac(&pq_init_ss, &a1, &b1_no_auth);
                        if !hs_ct_eq_32(&mac, &resp.mac) {
                            if resp.suite_context.is_explicit() {
                                let _ = hs_pending_clear(self_label, peer);
                                hs_emit_suite_reject("REJECT_QSC_HS_TRANSCRIPT_CONTEXT");
                            } else {
                                emit_marker(
                                    "handshake_reject",
                                    None,
                                    &[("reason", "bad_transcript")],
                                );
                            }
                            return Ok(());
                        }
                        let th = hs_transcript_hash(&pq_init_ss, &a1, &b1_no_auth);
                        let sig_msg = hs_sig_msg_b1(&resp.session_id, &th);
                        if hs_sig_verify(&resp.sig_pk, &sig_msg, &resp.sig, "b1_verify").is_err() {
                            emit_marker("handshake_reject", None, &[("reason", "sig_invalid")]);
                            return Ok(());
                        }
                        let sig_fp = hs_sig_fingerprint(&resp.sig_pk);
                        let Some(peer_fp) = pending.peer_fp.as_deref() else {
                            emit_marker(
                                "identity_unknown",
                                None,
                                &[("peer", peer), ("seen_fp", "unknown")],
                            );
                            emit_marker(
                                "handshake_reject",
                                None,
                                &[("reason", "identity_unknown")],
                            );
                            return Ok(());
                        };
                        if hs_require_primary_identity_pin(peer, peer_fp, identity_read_pin)
                            .is_err()
                        {
                            return Ok(());
                        }
                        if hs_check_optional_identity_pin(
                            peer,
                            sig_fp.as_str(),
                            identity_read_sig_pin,
                        )
                        .is_err()
                        {
                            return Ok(());
                        }
                        let st = match hs_build_session(
                            true,
                            true,
                            pending.session_id,
                            dh_init_arr,
                            pq_init_ss,
                            dh_self_pub,
                            dh_peer_pub,
                        ) {
                            Ok(v) => v,
                            Err(_) => {
                                emit_marker(
                                    "handshake_reject",
                                    None,
                                    &[("reason", "session_init_failed")],
                                );
                                return Ok(());
                            }
                        };
                        qsp_session_store(peer, &st)
                            .map_err(|_| "handshake_session_store_failed")?;
                        let _ = hs_pending_clear(self_label, peer);
                        if active_suite_context.is_explicit() {
                            hs_emit_suite_accept(&active_suite_context, false);
                        } else {
                            hs_emit_suite_accept(&active_suite_context, true);
                        }
                        let k_confirm = hs_confirm_key(
                            &pq_init_ss,
                            &resp.session_id,
                            &th,
                            &active_suite_context,
                        );
                        let cmac = hs_confirm_mac(
                            &k_confirm,
                            &resp.session_id,
                            &th,
                            &active_suite_context,
                        );
                        let sig_sk = identity_self_kem_keypair(self_label)
                            .map_err(|e| e.as_str())?
                            .sig_sk;
                        let a2_sig_msg = hs_sig_msg_a2(&resp.session_id, &th, &cmac);
                        #[cfg(qsc_rng_failure_test_seam)]
                        let a2_sig_result = if hs_rng_failure_forced("QSC.SIG.A2") {
                            Err(())
                        } else {
                            c.sign(&sig_sk, &a2_sig_msg).map_err(|_| ())
                        };
                        #[cfg(not(qsc_rng_failure_test_seam))]
                        let a2_sig_result = c.sign(&sig_sk, &a2_sig_msg).map_err(|_| ());
                        let a2_sig = match a2_sig_result {
                            Ok(v) => v,
                            Err(_) => {
                                emit_marker(
                                    "handshake_reject",
                                    None,
                                    &[("reason", "sig_sign_failed")],
                                );
                                return Ok(());
                            }
                        };
                        emit_marker(
                            "sig_status",
                            None,
                            &[("ok", "true"), ("alg", "ML-DSA-65"), ("reason", "a2_sign")],
                        );
                        let confirm = HsConfirm {
                            suite_context: active_suite_context,
                            session_id: resp.session_id,
                            mac: cmac,
                            sig: a2_sig,
                        };
                        let cbytes = hs_encode_confirm(&confirm);
                        let size_s = cbytes.len().to_string();
                        emit_marker(
                            "handshake_send",
                            None,
                            &[("msg", "A2"), ("size", size_s.as_str())],
                        );
                        transport::relay_inbox_push(relay, peer_route_token, &cbytes)?;
                        emit_marker(
                            "handshake_complete",
                            None,
                            &[
                                ("peer", peer),
                                ("role", "initiator"),
                                ("peer_confirmed", "no"),
                            ],
                        );
                        return Ok(());
                    }
                    Err(reason) => {
                        if reason.starts_with("REJECT_QSC_HS_")
                            && pending_suite_context.is_explicit()
                        {
                            let _ = hs_pending_clear(self_label, peer);
                        }
                        hs_emit_decode_reject(reason);
                        continue;
                    }
                }
            }
            return Ok(());
        }
        if pending.role == "responder" {
            let pending_suite_context = match hs_pending_suite_context(&pending) {
                Ok(v) => v,
                Err(_) => {
                    let _ = hs_pending_clear(self_label, peer);
                    hs_reject_key_context();
                    return Ok(());
                }
            };
            for item in items {
                match hs_decode_confirm_pending(&item.data, suite_mode) {
                    Ok(confirm) => {
                        if confirm.session_id != pending.session_id {
                            emit_marker(
                                "handshake_reject",
                                None,
                                &[("reason", "session_id_mismatch")],
                            );
                            continue;
                        }
                        if suite_mode == HandshakeSuiteMode::SuiteRequired
                            && !pending_suite_context.is_explicit()
                        {
                            let _ = hs_pending_clear(self_label, peer);
                            hs_reject_key_context();
                            continue;
                        }
                        if !hs_contexts_match(&pending_suite_context, &confirm.suite_context) {
                            let _ = hs_pending_clear(self_label, peer);
                            hs_reject_context_mismatch();
                            continue;
                        }
                        let active_suite_context = pending_suite_context.clone();
                        let Some(k_confirm) = pending.confirm_key else {
                            emit_marker(
                                "handshake_reject",
                                None,
                                &[("reason", "missing_confirm_key")],
                            );
                            continue;
                        };
                        let Some(th) = pending.transcript_hash else {
                            emit_marker(
                                "handshake_reject",
                                None,
                                &[("reason", "missing_transcript")],
                            );
                            continue;
                        };
                        let expect = hs_confirm_mac(
                            &k_confirm,
                            &confirm.session_id,
                            &th,
                            &active_suite_context,
                        );
                        if !hs_ct_eq_32(&expect, &confirm.mac) {
                            emit_marker("handshake_recv", None, &[("msg", "A2"), ("ok", "false")]);
                            if active_suite_context.is_explicit() {
                                let _ = hs_pending_clear(self_label, peer);
                                hs_emit_suite_reject("REJECT_QSC_HS_TRANSCRIPT_CONTEXT");
                            } else {
                                emit_marker("handshake_reject", None, &[("reason", "bad_confirm")]);
                            }
                            continue;
                        }
                        let Some(peer_sig_pk) = pending.peer_sig_pk.as_ref() else {
                            emit_marker(
                                "handshake_reject",
                                None,
                                &[("reason", "identity_missing")],
                            );
                            continue;
                        };
                        let sig_msg = hs_sig_msg_a2(&confirm.session_id, &th, &confirm.mac);
                        if hs_sig_verify(peer_sig_pk, &sig_msg, &confirm.sig, "a2_verify").is_err()
                        {
                            emit_marker("handshake_reject", None, &[("reason", "sig_invalid")]);
                            continue;
                        }
                        emit_marker("handshake_recv", None, &[("msg", "A2"), ("ok", "true")]);
                        let Some(ref pending_bytes) = pending.pending_session else {
                            emit_marker("handshake_reject", None, &[("reason", "missing_session")]);
                            continue;
                        };
                        let st = match Suite2SessionState::restore_bytes(pending_bytes) {
                            Ok(v) => v,
                            Err(_) => {
                                emit_marker(
                                    "handshake_reject",
                                    None,
                                    &[("reason", "session_restore_failed")],
                                );
                                continue;
                            }
                        };
                        let Some(peer_fp) = pending.peer_fp.as_ref() else {
                            emit_marker(
                                "handshake_reject",
                                None,
                                &[("reason", "identity_missing")],
                            );
                            continue;
                        };
                        let Some(peer_sig_fp) = pending.peer_sig_fp.as_ref() else {
                            emit_marker(
                                "handshake_reject",
                                None,
                                &[("reason", "identity_missing")],
                            );
                            continue;
                        };
                        if hs_require_primary_identity_pin(
                            peer,
                            peer_fp.as_str(),
                            identity_read_pin,
                        )
                        .is_err()
                        {
                            continue;
                        }
                        if hs_check_optional_identity_pin(
                            peer,
                            peer_sig_fp.as_str(),
                            identity_read_sig_pin,
                        )
                        .is_err()
                        {
                            continue;
                        }
                        qsp_session_store(peer, &st)
                            .map_err(|_| "handshake_session_store_failed")?;
                        let _ = hs_pending_clear(self_label, peer);
                        if active_suite_context.is_explicit() {
                            hs_emit_suite_accept(&active_suite_context, false);
                        } else {
                            hs_emit_suite_accept(&active_suite_context, true);
                        }
                        emit_marker(
                            "handshake_complete",
                            None,
                            &[
                                ("peer", peer),
                                ("role", "responder"),
                                ("peer_confirmed", "yes"),
                            ],
                        );
                        return Ok(());
                    }
                    Err(reason) => {
                        if pending_suite_context.is_explicit() {
                            #[cfg(qsc_binding_fuzz_helper)]
                            {
                                if crate::adversarial::binding_fuzz::replay_candidate_matches_pending_init(
                                    &item.data,
                                    &pending.session_id,
                                    pending_suite_context.explicit_block(),
                                    hs_fuzz_suite_mode(suite_mode),
                                ) {
                                    let _ = hs_pending_clear(self_label, peer);
                                    hs_reject_replay();
                                    continue;
                                }
                            }
                            #[cfg(not(qsc_binding_fuzz_helper))]
                            if let Ok(init) = hs_decode_init(&item.data, suite_mode) {
                                if init.session_id == pending.session_id
                                    && hs_contexts_match(
                                        &pending_suite_context,
                                        &init.suite_context,
                                    )
                                {
                                    let _ = hs_pending_clear(self_label, peer);
                                    hs_reject_replay();
                                    continue;
                                }
                            }
                        }
                        if reason.starts_with("REJECT_QSC_HS_")
                            && pending_suite_context.is_explicit()
                        {
                            let _ = hs_pending_clear(self_label, peer);
                        }
                        hs_emit_decode_reject(reason);
                        continue;
                    }
                }
            }
            return Ok(());
        }
    }

    emit_marker(
        "handshake_pending",
        None,
        &[("peer", peer), ("present", "false"), ("role", "none")],
    );

    for item in items {
        if let Ok(confirm) = hs_decode_confirm(&item.data, suite_mode) {
            if confirm.suite_context.is_explicit() && matches!(qsp_session_load(peer), Ok(Some(_)))
            {
                hs_reject_replay();
                continue;
            }
        }
        match hs_decode_init(&item.data, suite_mode) {
            Ok(init) => {
                if hs_dh_pub_is_all_zero(&init.dh_pub) {
                    emit_marker("handshake_reject", None, &[("reason", "dh_pub_invalid")]);
                    continue;
                }
                let peer_fp = identity_fingerprint_from_pk(&init.kem_pk);
                let peer_sig_fp = hs_sig_fingerprint(&init.sig_pk);
                if hs_require_primary_identity_pin(peer, peer_fp.as_str(), identity_read_pin)
                    .is_err()
                {
                    continue;
                }
                if hs_check_optional_identity_pin(peer, peer_sig_fp.as_str(), identity_read_sig_pin)
                    .is_err()
                {
                    continue;
                }
                let c = StdCrypto;
                #[cfg(qsc_rng_failure_test_seam)]
                if hs_rng_failure_forced("QSC.KEM.ENCAP") {
                    emit_marker("handshake_reject", None, &[("reason", "pq_encap_failed")]);
                    continue;
                }
                let (kem_ct, ss_pq) = match c.encap(&init.kem_pk) {
                    Ok(v) => v,
                    Err(_) => {
                        emit_marker("handshake_reject", None, &[("reason", "pq_encap_failed")]);
                        continue;
                    }
                };
                let pq_init_ss = hs_pq_init_ss(&ss_pq, &init.session_id, &init.suite_context);
                let (dh_sk, dh_self_pub) = hs_ephemeral_keypair();
                let dh_shared = match hs_dh_shared(&dh_sk, &init.dh_pub) {
                    Ok(v) => v,
                    Err(_) => {
                        emit_marker("handshake_reject", None, &[("reason", "dh_failed")]);
                        continue;
                    }
                };
                let dh_init_arr =
                    hs_dh_init_from_shared(&dh_shared, &init.session_id, &init.suite_context);
                let dh_peer_pub = init.dh_pub;
                let st = match hs_build_session(
                    true,
                    false,
                    init.session_id,
                    dh_init_arr,
                    pq_init_ss,
                    dh_self_pub,
                    dh_peer_pub,
                ) {
                    Ok(v) => v,
                    Err(_) => {
                        emit_marker(
                            "handshake_reject",
                            None,
                            &[("reason", "session_init_failed")],
                        );
                        continue;
                    }
                };
                let a1 = hs_encode_init(&init);
                let self_sig = match identity_self_kem_keypair(self_label) {
                    Ok(k) => (k.sig_pk, k.sig_sk),
                    Err(_) => {
                        emit_marker("handshake_reject", None, &[("reason", "identity_missing")]);
                        continue;
                    }
                };
                let (self_sig_pk, self_sig_sk) = self_sig;
                let b1_no_auth = hs_encode_resp_no_auth(
                    &init.session_id,
                    &kem_ct,
                    &self_sig_pk,
                    &dh_self_pub,
                    &init.suite_context,
                );
                let mac = hs_transcript_mac(&pq_init_ss, &a1, &b1_no_auth);
                let th = hs_transcript_hash(&pq_init_ss, &a1, &b1_no_auth);
                let sig_msg = hs_sig_msg_b1(&init.session_id, &th);
                #[cfg(qsc_rng_failure_test_seam)]
                let sig_result = if hs_rng_failure_forced("QSC.SIG.B1") {
                    Err(())
                } else {
                    c.sign(&self_sig_sk, &sig_msg).map_err(|_| ())
                };
                #[cfg(not(qsc_rng_failure_test_seam))]
                let sig_result = c.sign(&self_sig_sk, &sig_msg).map_err(|_| ());
                let sig = match sig_result {
                    Ok(v) => v,
                    Err(_) => {
                        emit_marker("handshake_reject", None, &[("reason", "sig_sign_failed")]);
                        continue;
                    }
                };
                emit_marker(
                    "sig_status",
                    None,
                    &[("ok", "true"), ("alg", "ML-DSA-65"), ("reason", "b1_sign")],
                );
                let k_confirm =
                    hs_confirm_key(&pq_init_ss, &init.session_id, &th, &init.suite_context);
                let pending = HandshakePending {
                    self_label: self_label.to_string(),
                    peer: peer.to_string(),
                    session_id: init.session_id,
                    kem_sk: Vec::new(),
                    kem_pk: Vec::new(),
                    dh_sk: dh_sk.to_vec(),
                    dh_pub: dh_self_pub.to_vec(),
                    sig_pk: Vec::new(),
                    peer_fp: Some(peer_fp),
                    peer_sig_fp: Some(peer_sig_fp),
                    peer_sig_pk: Some(init.sig_pk.clone()),
                    role: "responder".to_string(),
                    confirm_key: Some(k_confirm),
                    transcript_hash: Some(th),
                    pending_session: Some(st.snapshot_bytes()),
                    suite_context: init.suite_context.as_pending_block(),
                };
                hs_pending_store(&pending).map_err(|_| "handshake_pending_store_failed")?;
                let resp = HsResp {
                    suite_context: init.suite_context.clone(),
                    session_id: init.session_id,
                    kem_ct,
                    mac,
                    sig_pk: self_sig_pk,
                    sig,
                    dh_pub: dh_self_pub,
                };
                let bytes = hs_encode_resp(&resp);
                let size_s = bytes.len().to_string();
                let ct_len_s = hs_kem_ct_len().to_string();
                let sig_pk_len_s = hs_sig_pk_len().to_string();
                let hs_version_s = init.suite_context.wire_version().to_string();
                emit_marker(
                    "handshake_send",
                    None,
                    &[
                        ("msg", "B1"),
                        ("size", size_s.as_str()),
                        ("kem_ct_len", ct_len_s.as_str()),
                        ("sig_pk_len", sig_pk_len_s.as_str()),
                        ("hs_version", hs_version_s.as_str()),
                        ("suite_context", init.suite_context.mode_label()),
                    ],
                );
                transport::relay_inbox_push(relay, peer_route_token, &bytes)?;
                return Ok(());
            }
            Err(reason) => {
                hs_emit_decode_reject(reason);
                continue;
            }
        }
    }
    Ok(())
}

fn handshake_poll_with_tokens(
    self_label: &str,
    peer: &str,
    relay: &str,
    inbox_route_token: &str,
    peer_route_token: &str,
    max: usize,
    suite_mode: HandshakeSuiteMode,
) {
    if !require_unlocked("handshake_poll") {
        return;
    }
    if let Err(code) = perform_handshake_poll_with_tokens(
        self_label,
        peer,
        relay,
        inbox_route_token,
        peer_route_token,
        max,
        suite_mode,
    ) {
        print_error_marker(code);
    }
}

pub(crate) fn handshake_poll_with_suite_mode(
    self_label: &str,
    peer: &str,
    relay: &str,
    max: usize,
    suite_mode: HandshakeSuiteMode,
) {
    let peer_channel = resolve_peer_device_target(peer, false)
        .map(|v| v.channel)
        .unwrap_or_else(|_| peer.to_string());
    let inbox_route_token =
        relay_self_inbox_route_token().unwrap_or_else(|code| print_error_marker(code));
    let peer_route_token =
        relay_peer_route_token(peer).unwrap_or_else(|code| print_error_marker(code));
    handshake_poll_with_tokens(
        self_label,
        peer_channel.as_str(),
        relay,
        inbox_route_token.as_str(),
        peer_route_token.as_str(),
        max,
        suite_mode,
    );
}

pub(crate) fn handshake_poll(self_label: &str, peer: &str, relay: &str, max: usize) {
    handshake_poll_with_suite_mode(
        self_label,
        peer,
        relay,
        max,
        HandshakeSuiteMode::LegacyCompat,
    );
}

#[cfg(test)]
mod ct_eq_tests {
    use super::hs_ct_eq_32;

    // hs_ct_eq_32 must be bit-for-bit equivalent to `==` for all 32-byte inputs;
    // only the timing (no early-out) differs. These vectors assert that equivalence
    // so the ENG-0003 hardening cannot change accept/reject semantics.

    #[test]
    fn equal_arrays_are_equal() {
        let a = [0x5au8; 32];
        let b = [0x5au8; 32];
        assert!(hs_ct_eq_32(&a, &b));
        assert_eq!(hs_ct_eq_32(&a, &b), a == b);
    }

    #[test]
    fn single_byte_flip_is_unequal_at_every_position() {
        let base = {
            let mut v = [0u8; 32];
            for (i, b) in v.iter_mut().enumerate() {
                *b = i as u8;
            }
            v
        };
        for pos in 0..32 {
            let mut other = base;
            other[pos] ^= 0x01;
            assert!(!hs_ct_eq_32(&base, &other), "flip at {pos} must be unequal");
            assert_eq!(hs_ct_eq_32(&base, &other), base == other);
        }
    }

    #[test]
    fn high_bit_flip_and_all_different_are_unequal() {
        let base = [0x00u8; 32];
        let mut hi = base;
        hi[31] ^= 0x80;
        assert!(!hs_ct_eq_32(&base, &hi));
        let allff = [0xffu8; 32];
        assert!(!hs_ct_eq_32(&base, &allff));
        assert_eq!(hs_ct_eq_32(&base, &allff), base == allff);
    }

    #[test]
    fn matches_operator_over_mixed_vectors() {
        let vectors: [([u8; 32], [u8; 32]); 4] = [
            ([0u8; 32], [0u8; 32]),
            ([1u8; 32], [1u8; 32]),
            ([0u8; 32], [1u8; 32]),
            (
                {
                    let mut v = [7u8; 32];
                    v[0] = 9;
                    v
                },
                [7u8; 32],
            ),
        ];
        for (a, b) in vectors.iter() {
            assert_eq!(hs_ct_eq_32(a, b), a == b);
        }
    }
}
