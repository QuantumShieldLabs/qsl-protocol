use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize)]
pub(crate) struct OutboxRecord {
    pub(crate) version: u8,
    pub(crate) payload_len: usize,
    #[serde(default)]
    pub(crate) to: String,
    #[serde(default)]
    pub(crate) channel: Option<String>,
    #[serde(default)]
    pub(crate) ciphertext: Vec<u8>,
    #[serde(default)]
    pub(crate) kind: String,
    #[serde(default)]
    pub(crate) message_id: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct QspStatusRecord {
    pub(crate) active: bool,
    pub(crate) reason: String,
    pub(crate) last_pack_ok: bool,
    pub(crate) last_unpack_ok: bool,
}

pub(crate) const QSP_SESSIONS_DIR: &str = "qsp_sessions";
pub(crate) const QSP_SESSION_LEGACY_TOMBSTONE: &[u8] = b"QSC_SESSION_MIGRATED_V1\n";
pub(crate) const QSP_SESSION_BLOB_MAGIC: &[u8; 6] = b"QSSV01";
pub(crate) const QSP_SESSION_BLOB_VERSION: u8 = 1;
pub(crate) const QSP_SESSION_STORE_KEY_SECRET: &str = "qsp_session_store_key_v1";
pub(crate) const CONTACTS_SECRET_KEY: &str = "contacts.json";
pub(crate) const TIMELINE_SECRET_KEY: &str = "timeline.json";
pub(crate) const TUI_AUTOLOCK_SECRET_KEY: &str = "tui.autolock.minutes";
pub(crate) const TUI_POLL_MODE_SECRET_KEY: &str = "tui.poll.mode";
pub(crate) const TUI_POLL_INTERVAL_SECRET_KEY: &str = "tui.poll.interval_seconds";
pub(crate) const TUI_RECEIPT_MODE_SECRET_KEY: &str = "tui.receipt.mode";
pub(crate) const TUI_RECEIPT_BATCH_WINDOW_MS_SECRET_KEY: &str = "tui.receipt.batch_window_ms";
pub(crate) const TUI_RECEIPT_JITTER_MS_SECRET_KEY: &str = "tui.receipt.jitter_ms";
pub(crate) const TUI_FILE_CONFIRM_MODE_SECRET_KEY: &str = "tui.file_confirm.mode";
pub(crate) const TUI_TRUST_MODE_SECRET_KEY: &str = "tui.trust.mode";
pub(crate) const TUI_LAST_COMMAND_RESULT_SECRET_KEY: &str = "tui.last_command_result";
pub(crate) const TUI_RELAY_ENDPOINT_SECRET_KEY: &str = "tui.relay.endpoint";
pub(crate) const TUI_RELAY_TOKEN_SECRET_KEY: &str = "tui.relay.token";
pub(crate) const TUI_RELAY_TOKEN_FILE_SECRET_KEY: &str = "tui.relay.token_file";
pub(crate) const TUI_RELAY_INBOX_TOKEN_SECRET_KEY: &str = "tui.relay.inbox_token";
pub(crate) const OUTBOX_NEXT_STATE_SECRET_KEY: &str = "outbox.next_state.v1";
pub(crate) const ACCOUNT_VERIFICATION_SEED_SECRET_KEY: &str = "account.verification_seed_v1";
pub(crate) const CONTACT_REQUESTS_SECRET_KEY: &str = "contact_requests.json";
pub(crate) const ATTACHMENT_JOURNAL_SECRET_KEY: &str = "attachments.json";
pub(crate) const QSC_ERR_RELAY_TLS_REQUIRED: &str = "QSC_ERR_RELAY_TLS_REQUIRED";
pub(crate) const QSC_ERR_RELAY_INBOX_TOKEN_REQUIRED: &str = "QSC_ERR_RELAY_INBOX_TOKEN_REQUIRED";
pub(crate) const QSC_ERR_CONTACT_ROUTE_TOKEN_REQUIRED: &str =
    "QSC_ERR_CONTACT_ROUTE_TOKEN_REQUIRED";
pub(crate) const QSC_ERR_ROUTE_TOKEN_INVALID: &str = "QSC_ERR_ROUTE_TOKEN_INVALID";
pub(crate) const QSC_ERR_VAULT_WIPED_AFTER_FAILED_UNLOCKS: &str =
    "QSC_ERR_VAULT_WIPED_AFTER_FAILED_UNLOCKS";
pub(crate) const VAULT_SECURITY_CONFIG_NAME: &str = "vault_security.txt";
pub(crate) const VAULT_UNLOCK_COUNTER_NAME: &str = "vault_unlock_failures.txt";
pub(crate) const VAULT_ATTEMPT_LIMIT_MIN: u32 = 1;
pub(crate) const VAULT_ATTEMPT_LIMIT_MAX: u32 = 100;
pub(crate) const FILE_XFER_VERSION: u8 = 1;
pub(crate) const FILE_XFER_DEFAULT_MAX_FILE_SIZE: usize = 256 * 1024;
pub(crate) const FILE_XFER_MAX_FILE_SIZE_CEILING: usize = 4 * 1024 * 1024;
pub(crate) const FILE_XFER_DEFAULT_CHUNK_SIZE: usize = 16 * 1024;
// Sender and receiver must share the same supported chunk ceiling. Larger
// chunks can overflow the current Suite-2 wire body-length field once file
// metadata is serialized, so fail closed before any relay send occurs.
pub(crate) const FILE_XFER_MAX_CHUNK_SIZE_CEILING: usize = FILE_XFER_DEFAULT_CHUNK_SIZE;
pub(crate) const FILE_XFER_DEFAULT_MAX_CHUNKS: usize = 64;
pub(crate) const FILE_XFER_MAX_CHUNKS_CEILING: usize = 256;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub(crate) struct FileTransferRecord {
    pub(crate) id: String,
    pub(crate) peer: String,
    pub(crate) filename: String,
    pub(crate) total_size: usize,
    pub(crate) chunk_count: usize,
    pub(crate) manifest_hash: String,
    #[serde(default)]
    pub(crate) chunk_hashes: Vec<String>,
    #[serde(default)]
    pub(crate) chunks_hex: Vec<String>,
    #[serde(default)]
    pub(crate) confirm_requested: bool,
    #[serde(default)]
    pub(crate) confirm_id: Option<String>,
    #[serde(default)]
    pub(crate) target_device_id: Option<String>,
    pub(crate) state: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub(crate) struct AttachmentJournal {
    #[serde(default)]
    pub(crate) records: BTreeMap<String, AttachmentTransferRecord>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub(crate) struct AttachmentTransferRecord {
    pub(crate) attachment_id: String,
    pub(crate) peer: String,
    pub(crate) direction: String,
    pub(crate) service_url: Option<String>,
    pub(crate) state: String,
    pub(crate) plaintext_len: u64,
    pub(crate) ciphertext_len: u64,
    pub(crate) part_size_class: String,
    pub(crate) part_count: u32,
    pub(crate) integrity_alg: String,
    pub(crate) integrity_root: String,
    pub(crate) retention_class: String,
    pub(crate) enc_ctx_alg: String,
    pub(crate) enc_ctx_b64u: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) locator_kind: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) locator_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) fetch_capability: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) expires_at_unix_s: Option<u64>,
    #[serde(default)]
    pub(crate) confirm_requested: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) confirm_handle: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) filename_hint: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) media_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) source_path: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) staged_ciphertext_rel: Option<String>,
    #[serde(
        default,
        rename = "session_id",
        alias = "session_id",
        skip_serializing_if = "Option::is_none"
    )]
    pub(crate) session_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) resume_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) timeline_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) target_device_id: Option<String>,
    #[serde(default)]
    pub(crate) uploaded_parts: Vec<u32>,
    #[serde(default)]
    pub(crate) downloaded_ciphertext_bytes: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) download_ciphertext_rel: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) download_output_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) last_error: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub(crate) struct ContactsStore {
    pub(crate) peers: BTreeMap<String, ContactRecord>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub(crate) struct ContactRequestsStore {
    pub(crate) requests: BTreeMap<String, ContactRequestRecord>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub(crate) struct ContactRequestRecord {
    pub(crate) alias: String,
    #[serde(default)]
    pub(crate) device_id: Option<String>,
    #[serde(default)]
    pub(crate) state: String,
    #[serde(default)]
    pub(crate) reason: Option<String>,
    #[serde(default)]
    pub(crate) seen_at: Option<u64>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub(crate) struct ContactRecord {
    pub(crate) fp: String,
    pub(crate) status: String,
    pub(crate) blocked: bool,
    #[serde(default)]
    pub(crate) seen_at: Option<u64>,
    #[serde(default)]
    pub(crate) sig_fp: Option<String>,
    #[serde(default)]
    pub(crate) route_token: Option<String>,
    #[serde(default)]
    pub(crate) primary_device_id: Option<String>,
    #[serde(default)]
    pub(crate) devices: Vec<ContactDeviceRecord>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub(crate) struct ContactDeviceRecord {
    pub(crate) device_id: String,
    pub(crate) fp: String,
    #[serde(default)]
    pub(crate) sig_fp: Option<String>,
    pub(crate) state: String,
    #[serde(default)]
    pub(crate) route_token: Option<String>,
    #[serde(default)]
    pub(crate) seen_at: Option<u64>,
    #[serde(default)]
    pub(crate) label: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub(crate) struct TimelineStore {
    #[serde(default = "crate::timeline_ts_default")]
    pub(crate) next_ts: u64,
    #[serde(default)]
    pub(crate) peers: BTreeMap<String, Vec<crate::TimelineEntry>>,
    #[serde(default)]
    pub(crate) file_transfers: BTreeMap<String, FileTransferRecord>,
}

#[derive(Clone, Debug, Default)]
pub(crate) struct VaultSecurityState {
    pub(crate) attempt_limit: Option<u32>,
    pub(crate) failed_unlocks: u32,
}

pub(crate) enum UnlockAttemptOutcome {
    Unlocked,
    Rejected,
    Wiped,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct FileTransferChunkPayload {
    pub(crate) v: u8,
    pub(crate) t: String,
    pub(crate) file_id: String,
    pub(crate) filename: String,
    pub(crate) total_size: usize,
    pub(crate) chunk_index: usize,
    pub(crate) chunk_count: usize,
    pub(crate) chunk_hash: String,
    pub(crate) manifest_hash: String,
    pub(crate) chunk: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct FileTransferManifestPayload {
    pub(crate) v: u8,
    pub(crate) t: String,
    pub(crate) file_id: String,
    pub(crate) filename: String,
    pub(crate) total_size: usize,
    pub(crate) chunk_count: usize,
    pub(crate) chunk_hashes: Vec<String>,
    pub(crate) manifest_hash: String,
    #[serde(default)]
    pub(crate) confirm_requested: bool,
    #[serde(default)]
    pub(crate) confirm_id: String,
}

pub(crate) enum FileTransferPayload {
    Chunk(FileTransferChunkPayload),
    Manifest(FileTransferManifestPayload),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub(crate) struct AttachmentDescriptorPayload {
    pub(crate) v: u8,
    pub(crate) t: String,
    pub(crate) attachment_id: String,
    pub(crate) plaintext_len: u64,
    pub(crate) ciphertext_len: u64,
    pub(crate) part_size_class: String,
    pub(crate) part_count: u32,
    pub(crate) integrity_alg: String,
    pub(crate) integrity_root: String,
    pub(crate) locator_kind: String,
    pub(crate) locator_ref: String,
    pub(crate) fetch_capability: String,
    pub(crate) enc_ctx_alg: String,
    pub(crate) enc_ctx_b64u: String,
    pub(crate) retention_class: String,
    pub(crate) expires_at_unix_s: u64,
    pub(crate) confirm_requested: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) confirm_handle: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) filename_hint: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) media_type: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub(crate) struct AttachmentConfirmPayload {
    pub(crate) v: u8,
    pub(crate) t: String,
    pub(crate) kind: String,
    pub(crate) attachment_id: String,
    pub(crate) confirm_handle: String,
}
