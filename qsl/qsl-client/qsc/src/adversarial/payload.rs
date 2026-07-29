use serde::{Deserialize, Serialize};

pub const FILE_XFER_VERSION: u8 = 1;
pub const ATTACHMENT_DESCRIPTOR_VERSION: u8 = 1;
pub const ATTACHMENT_DESCRIPTOR_TYPE: &str = "attachment_descriptor";
pub const ATTACHMENT_CONFIRM_KIND: &str = "attachment_confirmed";
pub const FILE_CONFIRM_KIND: &str = "file_confirmed";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct ReceiptControlPayload {
    pub v: u8,
    pub t: String,
    pub kind: String,
    pub msg_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body: Option<Vec<u8>>,
    /// NA-0682 (D617 C6/F1): the namespace marker, present from `v:2` onwards.
    ///
    /// ⚠ WHY THIS FIELD EXISTS, and it is a SILENT-LOSS guard, not decoration.
    ///
    /// C6 requires unknown control types to be IGNORED rather than rendered to the user as
    /// messages -- otherwise DESIGN F2's "a new ack type is a new type, no format break" is
    /// false, because an older client shows the new type's raw JSON as a message.
    ///
    /// But "ignore anything that parses as this struct" would be far worse than the bug it
    /// fixes: `parse_receipt_payload` accepts ANY JSON carrying these field names, so a USER
    /// MESSAGE whose plaintext happens to be such JSON would be silently swallowed. That is
    /// a silent loss on the receive path -- exactly what this slice exists to prevent.
    ///
    /// So the ignore rule keys on an UNAMBIGUOUS marker instead. A payload is ours only if
    /// it carries `ns == "qsc.ctrl"`. Unknown `t`/`v` WITH the marker is ignored; anything
    /// without it falls through and is delivered as a message, exactly as before.
    ///
    /// `v:1` payloads predate the marker and are matched by their exact legacy shape, so
    /// their behaviour is byte-identical to before this lane.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ns: Option<String>,
}

/// The one namespace marker. Anything carrying it is ours; anything else is a user message.
pub const CTRL_NS: &str = "qsc.ctrl";

/// The highest control-payload version this build understands. Anything ours but newer is
/// IGNORED rather than decoded on today's rules.
pub const CTRL_VERSION_MAX: u8 = 2;

/// What a decoded control payload is, from the receiver's point of view.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ControlClass {
    /// A delivery acknowledgement (`t=ack`, `kind=delivered`), v1 or v2.
    DeliveredAck,
    /// A data envelope carrying a body plus a delivery-receipt request.
    DataEnvelope,
    /// Recognisably OURS (carries `ns`) but of a type this build does not know.
    /// ⚠ IGNORE IT -- never render it to the user. This is the read-receipt seam.
    UnknownControl,
    /// Not ours. Deliver it as an ordinary message, exactly as before.
    NotControl,
}

/// Classify a decoded payload.
///
/// ⚠ Ordering matters: the two known v1 shapes are matched FIRST and exactly, so legacy
/// traffic behaves byte-identically. The namespace marker is consulted only afterwards.
pub fn classify_control(ctrl: &ReceiptControlPayload) -> ControlClass {
    let ours = ctrl.ns.as_deref() == Some(CTRL_NS);
    // Legacy (v1) shapes: matched exactly as they were before NA-0682.
    if ctrl.v == 1 && ctrl.kind == "delivered" && ctrl.t == "ack" {
        return ControlClass::DeliveredAck;
    }
    if ctrl.v == 1 && ctrl.kind == "delivered" && ctrl.t == "data" {
        return ControlClass::DataEnvelope;
    }
    // v2+ must carry the marker to be treated as ours at all, AND be a version this build
    // actually understands.
    //
    // ⚠ The version bound is load-bearing, not defensive dressing. A FUTURE version may
    // give the same `t`/`kind` pair different semantics, so parsing an unknown version with
    // today's meaning would be a silent misinterpretation -- strictly worse than ignoring
    // it. This mirrors the Slice-2 precedent where `QSLI-2-` reads as NEWER rather than
    // being decoded on v1 rules.
    let known_version = ctrl.v <= CTRL_VERSION_MAX;
    if ours && known_version && ctrl.kind == "delivered" && ctrl.t == "ack" {
        return ControlClass::DeliveredAck;
    }
    if ours && known_version && ctrl.kind == "delivered" && ctrl.t == "data" {
        return ControlClass::DataEnvelope;
    }
    if ours {
        // Ours, but a type this build does not know -- the seam a future read-receipt
        // rides on. Ignoring it is what makes "no format break" true.
        return ControlClass::UnknownControl;
    }
    ControlClass::NotControl
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct FileConfirmPayload {
    pub v: u8,
    pub t: String,
    pub kind: String,
    pub file_id: String,
    pub confirm_id: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct FileTransferChunkPayload {
    pub v: u8,
    pub t: String,
    pub file_id: String,
    pub filename: String,
    pub total_size: usize,
    pub chunk_index: usize,
    pub chunk_count: usize,
    pub chunk_hash: String,
    pub manifest_hash: String,
    pub chunk: Vec<u8>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct FileTransferManifestPayload {
    pub v: u8,
    pub t: String,
    pub file_id: String,
    pub filename: String,
    pub total_size: usize,
    pub chunk_count: usize,
    pub chunk_hashes: Vec<String>,
    pub manifest_hash: String,
    #[serde(default)]
    pub confirm_requested: bool,
    #[serde(default)]
    pub confirm_id: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FileTransferPayload {
    Chunk(FileTransferChunkPayload),
    Manifest(FileTransferManifestPayload),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct AttachmentDescriptorPayload {
    pub v: u8,
    pub t: String,
    pub attachment_id: String,
    // NA-0614: true delivered length (required). `plaintext_len` is the padded/encrypted
    // length (a size-ladder bucket); the receiver truncates output to `content_len`.
    // Required in the v1 wire format from first release; not defaulted (strict).
    pub content_len: u64,
    pub plaintext_len: u64,
    pub ciphertext_len: u64,
    pub part_size_class: String,
    pub part_count: u32,
    pub integrity_alg: String,
    pub integrity_root: String,
    pub locator_kind: String,
    pub locator_ref: String,
    pub fetch_capability: String,
    pub enc_ctx_alg: String,
    pub enc_ctx_b64u: String,
    pub retention_class: String,
    pub expires_at_unix_s: u64,
    pub confirm_requested: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub confirm_handle: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filename_hint: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub media_type: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct AttachmentConfirmPayload {
    pub v: u8,
    pub t: String,
    pub kind: String,
    pub attachment_id: String,
    pub confirm_handle: String,
}

pub fn parse_receipt_payload(plaintext: &[u8]) -> Option<ReceiptControlPayload> {
    serde_json::from_slice::<ReceiptControlPayload>(plaintext).ok()
}

pub fn parse_file_confirm_payload(plaintext: &[u8]) -> Option<FileConfirmPayload> {
    serde_json::from_slice::<FileConfirmPayload>(plaintext)
        .ok()
        .filter(|v| v.v == 1 && v.t == "ack" && v.kind == FILE_CONFIRM_KIND)
}

pub fn parse_file_transfer_payload(plaintext: &[u8]) -> Option<FileTransferPayload> {
    if let Ok(chunk) = serde_json::from_slice::<FileTransferChunkPayload>(plaintext) {
        if chunk.v == FILE_XFER_VERSION && chunk.t == "file_chunk" {
            return Some(FileTransferPayload::Chunk(chunk));
        }
    }
    if let Ok(manifest) = serde_json::from_slice::<FileTransferManifestPayload>(plaintext) {
        if manifest.v == FILE_XFER_VERSION && manifest.t == "file_manifest" {
            return Some(FileTransferPayload::Manifest(manifest));
        }
    }
    None
}

pub fn parse_attachment_descriptor_payload(
    plaintext: &[u8],
) -> Option<AttachmentDescriptorPayload> {
    serde_json::from_slice::<AttachmentDescriptorPayload>(plaintext)
        .ok()
        .filter(|v| v.v == ATTACHMENT_DESCRIPTOR_VERSION && v.t == ATTACHMENT_DESCRIPTOR_TYPE)
}

pub fn parse_attachment_confirm_payload(plaintext: &[u8]) -> Option<AttachmentConfirmPayload> {
    serde_json::from_slice::<AttachmentConfirmPayload>(plaintext)
        .ok()
        .filter(|v| v.v == 1 && v.t == "ack" && v.kind == ATTACHMENT_CONFIRM_KIND)
}

#[cfg(test)]
mod tests {
    use super::*;

    // NA-0682 (D617 C6/F1): control classification.
    //
    // §3b: the happy paths are asserted FIRST, then the negatives, and the most important
    // negative is the SILENT-LOSS one -- a user message that merely looks like a control
    // payload must still be delivered.

    fn ctrl(v: u8, t: &str, kind: &str, ns: Option<&str>) -> ReceiptControlPayload {
        ReceiptControlPayload {
            v,
            t: t.to_string(),
            kind: kind.to_string(),
            msg_id: "0123456789abcdef0123456789abcdef".to_string(),
            body: None,
            ns: ns.map(|x| x.to_string()),
        }
    }

    #[test]
    fn legacy_v1_control_shapes_are_matched_exactly_as_before() {
        // Byte-identical behaviour for traffic that predates the marker.
        assert_eq!(
            classify_control(&ctrl(1, "ack", "delivered", None)),
            ControlClass::DeliveredAck
        );
        assert_eq!(
            classify_control(&ctrl(1, "data", "delivered", None)),
            ControlClass::DataEnvelope
        );
    }

    #[test]
    fn v2_control_shapes_are_matched_when_they_carry_the_marker() {
        assert_eq!(
            classify_control(&ctrl(2, "ack", "delivered", Some(CTRL_NS))),
            ControlClass::DeliveredAck
        );
        assert_eq!(
            classify_control(&ctrl(2, "data", "delivered", Some(CTRL_NS))),
            ControlClass::DataEnvelope
        );
    }

    #[test]
    fn an_unknown_control_type_carrying_the_marker_is_ignored_not_rendered() {
        // The read-receipt seam: a future type must be IGNORED by this build, which is what
        // makes DESIGN F2's "no format break" true rather than aspirational.
        assert_eq!(
            classify_control(&ctrl(2, "read_receipt", "read", Some(CTRL_NS))),
            ControlClass::UnknownControl
        );
        assert_eq!(
            classify_control(&ctrl(9, "ack", "delivered", Some(CTRL_NS))),
            ControlClass::UnknownControl,
            "a newer version we cannot parse is still ours, so ignore it"
        );
    }

    #[test]
    fn a_user_message_that_merely_looks_like_a_control_is_still_delivered() {
        // ⚠ THE SILENT-LOSS GUARD, and the reason the `ns` marker exists at all.
        //
        // `parse_receipt_payload` accepts ANY JSON carrying these field names. If "ignore
        // unknown control" keyed on that alone, a user whose message text happens to be
        // such JSON would have it SILENTLY SWALLOWED on receive -- a silent loss, which is
        // precisely what this slice exists to prevent. Without the marker, it is not ours.
        assert_eq!(
            classify_control(&ctrl(2, "read_receipt", "read", None)),
            ControlClass::NotControl
        );
        assert_eq!(
            classify_control(&ctrl(7, "anything", "whatever", None)),
            ControlClass::NotControl
        );
        // Even a perfectly-shaped v1 ack is NOT ours if the kind is foreign.
        assert_eq!(
            classify_control(&ctrl(1, "ack", "something_else", None)),
            ControlClass::NotControl
        );
    }

    #[test]
    fn a_wrong_namespace_is_not_ours() {
        assert_eq!(
            classify_control(&ctrl(2, "read_receipt", "read", Some("evil.ctrl"))),
            ControlClass::NotControl
        );
    }

    #[test]
    fn attachment_descriptor_rejects_unknown_fields() {
        let raw = br#"{
            "v":1,
            "t":"attachment_descriptor",
            "attachment_id":"a1",
            "content_len":1,
            "plaintext_len":1,
            "ciphertext_len":2,
            "part_size_class":"small",
            "part_count":1,
            "integrity_alg":"sha512_merkle_v1",
            "integrity_root":"root",
            "locator_kind":"service_ref_v1",
            "locator_ref":"loc",
            "fetch_capability":"cap",
            "enc_ctx_alg":"ctx",
            "enc_ctx_b64u":"ctxb64",
            "retention_class":"default",
            "expires_at_unix_s":1,
            "confirm_requested":false,
            "extra":"nope"
        }"#;
        assert!(parse_attachment_descriptor_payload(raw).is_none());
    }

    #[test]
    fn file_manifest_requires_expected_tag() {
        let raw = br#"{
            "v":1,
            "t":"wrong",
            "file_id":"f1",
            "filename":"x",
            "total_size":1,
            "chunk_count":1,
            "chunk_hashes":["h"],
            "manifest_hash":"m"
        }"#;
        assert!(parse_file_transfer_payload(raw).is_none());
    }

    // NA-0610: explicit malformed attachment-descriptor and confirm negatives.
    // These convert the NA-0608 "corrupted descriptor / not separately exercised"
    // hedge into deterministic fail-closed reject assertions at the descriptor and
    // confirm parse boundary. Each malformed input must yield None (reject); the
    // sanity case confirms the well-formed template parses so the negatives are
    // meaningful. This changes no production parse behavior; it observes it.

    // A well-formed attachment descriptor (the unknown-fields test above minus the
    // extra field). Mutated below for each malformed case.
    const VALID_DESCRIPTOR: &[u8] = br#"{
        "v":1,
        "t":"attachment_descriptor",
        "attachment_id":"a1",
        "content_len":1,
        "plaintext_len":1,
        "ciphertext_len":2,
        "part_size_class":"small",
        "part_count":1,
        "integrity_alg":"sha512_merkle_v1",
        "integrity_root":"root",
        "locator_kind":"service_ref_v1",
        "locator_ref":"loc",
        "fetch_capability":"cap",
        "enc_ctx_alg":"ctx",
        "enc_ctx_b64u":"ctxb64",
        "retention_class":"default",
        "expires_at_unix_s":1,
        "confirm_requested":false
    }"#;

    #[test]
    fn attachment_descriptor_wellformed_template_parses() {
        // Sanity: the template the negatives mutate is itself accepted.
        assert!(parse_attachment_descriptor_payload(VALID_DESCRIPTOR).is_some());
    }

    #[test]
    fn attachment_descriptor_rejects_empty_input() {
        assert!(parse_attachment_descriptor_payload(b"").is_none());
    }

    #[test]
    fn attachment_descriptor_rejects_non_json_garbage() {
        assert!(parse_attachment_descriptor_payload(b"\x00\x01\x02not-json").is_none());
    }

    #[test]
    fn attachment_descriptor_rejects_truncated_json() {
        // The valid template cut off mid-object is not valid JSON.
        let truncated = &VALID_DESCRIPTOR[..VALID_DESCRIPTOR.len() / 2];
        assert!(parse_attachment_descriptor_payload(truncated).is_none());
    }

    #[test]
    fn attachment_descriptor_rejects_wrong_version() {
        let raw = std::str::from_utf8(VALID_DESCRIPTOR)
            .unwrap()
            .replace("\"v\":1", "\"v\":2");
        assert!(parse_attachment_descriptor_payload(raw.as_bytes()).is_none());
    }

    #[test]
    fn attachment_descriptor_rejects_wrong_type() {
        let raw = std::str::from_utf8(VALID_DESCRIPTOR).unwrap().replace(
            "\"attachment_descriptor\"",
            "\"attachment_descriptor_wrong\"",
        );
        assert!(parse_attachment_descriptor_payload(raw.as_bytes()).is_none());
    }

    #[test]
    fn attachment_descriptor_rejects_missing_required_field() {
        // Drop a required field (attachment_id); serde must fail closed.
        let raw = std::str::from_utf8(VALID_DESCRIPTOR)
            .unwrap()
            .replace("\"attachment_id\":\"a1\",", "");
        assert!(parse_attachment_descriptor_payload(raw.as_bytes()).is_none());
    }

    #[test]
    fn attachment_confirm_rejects_malformed_inputs() {
        // Empty, non-JSON, and a JSON object missing required confirm fields all
        // reject fail-closed regardless of the confirm struct's exact shape.
        assert!(parse_attachment_confirm_payload(b"").is_none());
        assert!(parse_attachment_confirm_payload(b"not-json").is_none());
        assert!(parse_attachment_confirm_payload(br#"{"v":1,"t":"ack"}"#).is_none());
        // Valid JSON with the wrong discriminant also rejects.
        assert!(parse_attachment_confirm_payload(br#"{"v":2,"t":"nack","kind":"x"}"#).is_none());
    }
}
