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

    #[test]
    fn attachment_descriptor_rejects_unknown_fields() {
        let raw = br#"{
            "v":1,
            "t":"attachment_descriptor",
            "attachment_id":"a1",
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
}
