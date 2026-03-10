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
pub(crate) const FILE_XFER_MAX_CHUNK_SIZE_CEILING: usize = 64 * 1024;
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
