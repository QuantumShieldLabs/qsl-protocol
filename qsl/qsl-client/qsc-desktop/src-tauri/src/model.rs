use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct UiError {
    pub code: String,
    pub message: String,
    pub detail: String,
}

impl UiError {
    pub fn new(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            message: message.into(),
            detail: String::new(),
        }
    }

    pub fn with_detail(
        code: impl Into<String>,
        message: impl Into<String>,
        detail: impl Into<String>,
    ) -> Self {
        Self {
            code: code.into(),
            message: message.into(),
            detail: detail.into(),
        }
    }
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct DoctorSummary {
    pub ok: bool,
    pub config_dir: String,
    pub dir_exists: bool,
    pub dir_writable: bool,
    pub file_parseable: bool,
    pub symlink_safe: bool,
    pub parent_safe: bool,
}

#[derive(Clone, Debug, Serialize)]
pub struct VaultSummary {
    pub present: bool,
    pub key_source: String,
}

impl VaultSummary {
    pub fn missing() -> Self {
        Self {
            present: false,
            key_source: "missing".to_string(),
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct ContactSummary {
    pub label: String,
    pub state: String,
    pub blocked: bool,
    pub device_count: usize,
    pub primary_device: Option<String>,
}

#[derive(Clone, Debug, Serialize)]
pub struct DeviceSummary {
    pub device: String,
    pub state: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct TimelineItemSummary {
    pub id: String,
    pub direction: String,
    pub kind: String,
    pub ts: u64,
    pub state: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct PeerDetails {
    pub label: String,
    pub devices: Vec<DeviceSummary>,
    pub timeline: Vec<TimelineItemSummary>,
}

#[derive(Clone, Debug, Serialize)]
pub struct AppSnapshot {
    pub sidecar_ready: bool,
    pub sidecar_source: String,
    pub session_unlocked: bool,
    pub session_note: Option<String>,
    pub doctor: DoctorSummary,
    pub vault: VaultSummary,
    pub identity_fp: Option<String>,
    pub contacts: Vec<ContactSummary>,
    pub peer_details: Option<PeerDetails>,
}

#[derive(Clone, Debug, Serialize)]
pub struct ReceivedFile {
    pub file_name: String,
    pub kind: String,
    pub byte_len: usize,
    pub preview: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct SendResult {
    pub snapshot: AppSnapshot,
    pub delivery: Vec<String>,
}

#[derive(Clone, Debug, Serialize)]
pub struct ReceiveResult {
    pub snapshot: AppSnapshot,
    pub received_files: Vec<ReceivedFile>,
}
