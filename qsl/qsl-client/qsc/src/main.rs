use chacha20poly1305::aead::{Aead, KeyInit, Payload};
use chacha20poly1305::{ChaCha20Poly1305, Key, Nonce};
use clap::{Parser, Subcommand, ValueEnum};
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use quantumshield_refimpl::crypto::stdcrypto::StdCrypto;
use quantumshield_refimpl::crypto::traits::{Hash, Kmac, PqKem768, PqSigMldsa65};
use quantumshield_refimpl::qse::{Envelope, EnvelopeProfile};
use quantumshield_refimpl::suite2::establish::init_from_base_handshake;
use quantumshield_refimpl::suite2::ratchet::{Suite2RecvWireState, Suite2SendState};
use quantumshield_refimpl::suite2::state::Suite2SessionState;
use quantumshield_refimpl::suite2::types::{SUITE2_PROTOCOL_VERSION, SUITE2_SUITE_ID};
use quantumshield_refimpl::suite2::{recv_wire_canon, send_wire_canon};
use quantumshield_refimpl::RefimplError;
use rand_core::{OsRng, RngCore};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear as TuiClear, List, ListItem, Paragraph},
    Terminal,
};
use reqwest::blocking::Client as HttpClient;
use reqwest::StatusCode as HttpStatus;
use serde::{Deserialize, Serialize};
use serde_json::Map;
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::env;
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::process;
use std::process::Command;
use std::sync::atomic::{AtomicBool, AtomicU64, AtomicU8, Ordering};
use std::sync::{Mutex, OnceLock};
use std::time::{Duration, Instant};

#[derive(Parser, Debug)]
#[command(name = "qsc", version, about = "QSC client (Phase 2 scaffold)")]
struct Cli {
    /// Reveal sensitive output (non-default; demos should keep redaction).
    #[arg(long, global = true)]
    reveal: bool,
    /// Explicit unlock source for this invocation (default is locked).
    #[arg(long, global = true, value_name = "ENV")]
    unlock_passphrase_env: Option<String>,
    #[command(subcommand)]
    cmd: Option<Cmd>,
}

#[derive(Subcommand, Debug)]
enum Cmd {
    /// Print a deterministic status summary (no secrets, no timestamps).
    Status,
    /// Read/write config values.
    Config {
        #[command(subcommand)]
        cmd: ConfigCmd,
    },
    /// Diagnostic checks (read-only).
    Doctor {
        /// Run check-only diagnostics (no repairs).
        #[arg(long)]
        check_only: bool,
        /// Max time to probe any single filesystem check (ms).
        #[arg(long, default_value_t = 2000)]
        timeout_ms: u64,
        /// Export a redacted doctor report (check-only safe).
        #[arg(long, value_name = "PATH")]
        export: Option<PathBuf>,
    },
    /// Utility helpers.
    Util {
        #[command(subcommand)]
        cmd: UtilCmd,
    },
    /// Privacy envelope helpers (deterministic).
    Envelope {
        #[command(subcommand)]
        cmd: EnvelopeCmd,
    },
    /// Encrypted-at-rest vault operations.
    Vault {
        #[command(subcommand)]
        cmd: vault::VaultCmd,
    },
    /// Send commit semantics (prepare→send→commit).
    Send {
        /// Subcommand for send (e.g., abort a pending outbox).
        #[command(subcommand)]
        cmd: Option<SendCmd>,
        /// Transport selection (explicit-only).
        #[arg(long, value_enum)]
        transport: Option<SendTransport>,
        /// Relay address (host:port) for transport=relay.
        #[arg(long)]
        relay: Option<String>,
        /// Destination peer label.
        #[arg(long)]
        to: Option<String>,
        /// Path to payload file.
        #[arg(long, value_name = "PATH")]
        file: Option<PathBuf>,
        /// Pad to a specific envelope size (bounded; explicit-only).
        #[arg(long, value_name = "BYTES")]
        pad_to: Option<usize>,
        /// Pad to a standard size class (bounded; explicit-only).
        #[arg(long, value_enum)]
        pad_bucket: Option<MetaPadBucket>,
        /// Deterministic metadata seed (explicit-only).
        #[arg(long)]
        meta_seed: Option<u64>,
        /// Metadata bucket ceiling in bytes (marker-only).
        #[arg(long)]
        bucket_max: Option<usize>,
        /// Request delivered receipt (explicit-only; default off).
        #[arg(long, value_enum)]
        receipt: Option<ReceiptKind>,
    },
    /// Receive an inbound envelope (explicit-only).
    Receive {
        /// Transport selection (explicit-only).
        #[arg(long, value_enum)]
        transport: Option<SendTransport>,
        /// Relay base URL (http/https) for inbox transport.
        #[arg(long)]
        relay: Option<String>,
        /// Protocol peer label/session key used for decrypt context.
        #[arg(long)]
        from: Option<String>,
        /// Relay mailbox/channel label to pull from (default: self label when known; otherwise --from).
        #[arg(long)]
        mailbox: Option<String>,
        /// Max items to pull (bounded).
        #[arg(long)]
        max: Option<usize>,
        /// Output directory for received items.
        #[arg(long, value_name = "DIR")]
        out: Option<PathBuf>,
        /// Path to an inbound envelope file (legacy file mode).
        #[arg(long, value_name = "PATH")]
        file: Option<PathBuf>,
        /// Deterministic metadata mode (emit tick markers without sleeping).
        #[arg(long)]
        deterministic_meta: bool,
        /// Fixed polling interval in ms for metadata schedule.
        #[arg(long)]
        interval_ms: Option<u64>,
        /// Fixed polling interval (ms). Requires --poll-ticks and --poll-max-per-tick.
        #[arg(long, value_name = "MS", hide = true)]
        poll_interval_ms: Option<u64>,
        /// Number of polling ticks (bounded).
        #[arg(long)]
        poll_ticks: Option<u32>,
        /// Max items per poll tick/batch (bounded).
        #[arg(long)]
        batch_max_count: Option<u32>,
        /// Max items per poll tick (bounded).
        #[arg(long, hide = true)]
        poll_max_per_tick: Option<u32>,
        /// Metadata bucket ceiling in bytes.
        #[arg(long)]
        bucket_max: Option<usize>,
        /// Deterministic metadata seed (explicit-only).
        #[arg(long)]
        meta_seed: Option<u64>,
        /// Emit delivered receipts after successful unpack (explicit-only; default off).
        #[arg(long, value_enum)]
        emit_receipts: Option<ReceiptKind>,
    },
    /// Interactive handshake (explicit-only; inbox transport).
    Handshake {
        #[command(subcommand)]
        cmd: HandshakeCmd,
    },
    /// Identity utilities (show/rotate).
    Identity {
        #[command(subcommand)]
        cmd: IdentityCmd,
    },
    /// Peer identity list.
    Peers {
        #[command(subcommand)]
        cmd: PeersCmd,
    },
    /// Contacts + verify/block management.
    Contacts {
        #[command(subcommand)]
        cmd: ContactsCmd,
    },
    /// Encrypted timeline store/list/show/clear.
    Timeline {
        #[command(subcommand)]
        cmd: TimelineCmd,
    },
    /// File transfer MVP (bounded + integrity checked).
    File {
        #[command(subcommand)]
        cmd: FileCmd,
    },
    /// Security Lens TUI (read-mostly; no implicit actions).
    Tui {
        /// Run in headless scripted mode (tests only).
        #[arg(long, hide = true)]
        headless: bool,
        /// Transport selection (explicit-only).
        #[arg(long, value_enum)]
        transport: Option<TuiTransport>,
        /// Relay address (host:port) for transport=relay.
        #[arg(long)]
        relay: Option<String>,
        /// Seed for deterministic relay scenarios.
        #[arg(long, default_value_t = 0)]
        seed: u64,
        /// Scenario label (used for deterministic headless tests).
        #[arg(long, default_value = "default")]
        scenario: String,
    },
    /// Relay demo transport (explicit-only; deterministic fault injection).
    Relay {
        #[command(subcommand)]
        cmd: RelayCmd,
    },
    /// Metadata minimization planning (dry-run only).
    Meta {
        #[command(subcommand)]
        cmd: MetaCmd,
    },
}

#[derive(Subcommand, Debug)]
enum SendCmd {
    /// Abort a pending send by clearing the outbox (idempotent).
    Abort,
}

#[derive(Subcommand, Debug)]
enum FileCmd {
    /// Send a file transfer bundle using bounded chunks and manifest integrity.
    Send {
        /// Transport selection (explicit-only).
        #[arg(long, value_enum)]
        transport: Option<SendTransport>,
        /// Relay base URL (http/https) for inbox transport.
        #[arg(long)]
        relay: Option<String>,
        /// Destination peer label.
        #[arg(long)]
        to: String,
        /// Path to source file.
        #[arg(long, value_name = "PATH")]
        path: PathBuf,
        /// Chunk size in bytes (bounded).
        #[arg(long, default_value_t = FILE_XFER_DEFAULT_CHUNK_SIZE)]
        chunk_size: usize,
        /// Maximum file size in bytes (bounded).
        #[arg(long, default_value_t = FILE_XFER_DEFAULT_MAX_FILE_SIZE)]
        max_file_size: usize,
        /// Maximum chunks per transfer (bounded).
        #[arg(long, default_value_t = FILE_XFER_DEFAULT_MAX_CHUNKS)]
        max_chunks: usize,
    },
}

#[derive(ValueEnum, Debug, Clone, Copy)]
enum SendTransport {
    Relay,
}

#[derive(ValueEnum, Debug, Clone, Copy, PartialEq, Eq)]
enum ReceiptKind {
    Delivered,
}

#[derive(ValueEnum, Debug, Clone, Copy)]
enum TuiTransport {
    Relay,
}

#[derive(ValueEnum, Debug, Clone, Copy)]
enum MetaPadBucket {
    Standard,
    Enhanced,
    Private,
    Auto,
}

#[derive(Subcommand, Debug)]
enum HandshakeCmd {
    /// Initiate a handshake (A1) to a peer inbox.
    Init {
        /// Local label (used for inbox channel naming).
        #[arg(long = "as", value_name = "LABEL")]
        as_label: String,
        /// Peer label.
        #[arg(long, value_name = "LABEL")]
        peer: String,
        /// Relay base URL for inbox transport.
        #[arg(long)]
        relay: String,
    },
    /// Poll inbox and process handshake messages.
    Poll {
        /// Local label (used for inbox channel naming).
        #[arg(long = "as", value_name = "LABEL")]
        as_label: String,
        /// Peer label.
        #[arg(long, value_name = "LABEL")]
        peer: String,
        /// Relay base URL for inbox transport.
        #[arg(long)]
        relay: String,
        /// Max items to pull (bounded).
        #[arg(long, default_value_t = 4)]
        max: usize,
    },
    /// Show handshake status.
    Status {
        /// Peer label (optional; default peer-0).
        #[arg(long, value_name = "LABEL")]
        peer: Option<String>,
    },
}

#[derive(Subcommand, Debug)]
enum IdentityCmd {
    /// Show local identity fingerprint.
    Show {
        /// Local label (defaults to "self").
        #[arg(long = "as", value_name = "LABEL", default_value = "self")]
        as_label: String,
    },
    /// Rotate local identity keypair (explicit confirm required).
    Rotate {
        /// Local label (defaults to "self").
        #[arg(long = "as", value_name = "LABEL", default_value = "self")]
        as_label: String,
        /// Explicit confirmation to rotate identity.
        #[arg(long)]
        confirm: bool,
        /// Explicitly reset peer pins (opt-in).
        #[arg(long)]
        reset_peers: bool,
    },
}

#[derive(Subcommand, Debug)]
enum PeersCmd {
    /// List pinned peers and fingerprints.
    List,
}

#[derive(Subcommand, Debug)]
enum ContactsCmd {
    /// Add or update a contact pin.
    Add {
        #[arg(long, value_name = "LABEL")]
        label: String,
        #[arg(long, value_name = "FINGERPRINT")]
        fp: String,
        #[arg(long)]
        verify: bool,
    },
    /// Show one contact.
    Show {
        #[arg(long, value_name = "LABEL")]
        label: String,
    },
    /// List contacts.
    List,
    /// Verify/update a contact pin (requires explicit confirm for changes).
    Verify {
        #[arg(long, value_name = "LABEL")]
        label: String,
        #[arg(long, value_name = "FINGERPRINT")]
        fp: String,
        #[arg(long)]
        confirm: bool,
    },
    /// Block a contact.
    Block {
        #[arg(long, value_name = "LABEL")]
        label: String,
    },
    /// Unblock a contact.
    Unblock {
        #[arg(long, value_name = "LABEL")]
        label: String,
    },
}

#[derive(Subcommand, Debug)]
enum TimelineCmd {
    /// List timeline entries for a peer.
    List {
        #[arg(long, value_name = "LABEL")]
        peer: String,
        #[arg(long, value_name = "N")]
        limit: Option<usize>,
    },
    /// Show a single timeline entry by id.
    Show {
        #[arg(long, value_name = "LABEL")]
        peer: String,
        #[arg(long, value_name = "ID")]
        id: String,
    },
    /// Clear timeline entries for a peer (explicit confirm required).
    Clear {
        #[arg(long, value_name = "LABEL")]
        peer: String,
        #[arg(long)]
        confirm: bool,
    },
}

#[derive(Subcommand, Debug)]
enum RelayCmd {
    /// Run a local relay server with deterministic fault injection.
    Serve {
        /// Port to bind (0 = auto-assign).
        #[arg(long, default_value_t = 0)]
        port: u16,
        /// Seed for deterministic fault injection.
        #[arg(long, default_value_t = 0)]
        seed: u64,
        /// Drop percentage (0..100).
        #[arg(long, default_value_t = 0)]
        drop_pct: u8,
        /// Duplicate percentage (0..100).
        #[arg(long, default_value_t = 0)]
        dup_pct: u8,
        /// Reorder window size (0 disables).
        #[arg(long, default_value_t = 0)]
        reorder_window: usize,
        /// Fixed latency in milliseconds.
        #[arg(long, default_value_t = 0)]
        fixed_latency_ms: u64,
        /// Jitter window in milliseconds (0 disables).
        #[arg(long, default_value_t = 0)]
        jitter_ms: u64,
        /// Stop after processing N messages (tests only).
        #[arg(long, default_value_t = 0, hide = true)]
        max_messages: u64,
    },
    /// Send a message via a relay (explicit-only; no retries).
    Send {
        /// Destination peer label.
        #[arg(long)]
        to: String,
        /// Path to payload file.
        #[arg(long, value_name = "PATH")]
        file: PathBuf,
        /// Relay address (host:port).
        #[arg(long)]
        relay: String,
        /// Metadata bucket ceiling in bytes (marker-only).
        #[arg(long)]
        bucket_max: Option<usize>,
    },
}

#[derive(Subcommand, Debug)]
enum MetaCmd {
    /// Plan deterministic metadata schedule (dry-run only; no network, no writes).
    Plan {
        /// Deterministic planning mode.
        #[arg(long)]
        deterministic: bool,
        /// Number of plan ticks.
        #[arg(long, default_value_t = META_TICK_COUNT_DEFAULT)]
        tick_count: u32,
        /// Interval between ticks in ms.
        #[arg(long, default_value_t = META_INTERVAL_MS_DEFAULT)]
        interval_ms: u64,
        /// Metadata bucket ceiling in bytes.
        #[arg(long, default_value_t = META_BUCKET_MAX_DEFAULT)]
        bucket_max: usize,
        /// Max batch count per tick.
        #[arg(long, default_value_t = META_BATCH_MAX_COUNT_DEFAULT)]
        batch_max_count: u32,
        /// Plan explicit cover traffic markers.
        #[arg(long)]
        cover_enabled: bool,
    },
}

#[derive(Subcommand, Debug)]
enum ConfigCmd {
    /// Set a config key to a value.
    Set { key: String, value: String },
    /// Get a config key.
    Get { key: String },
}

#[derive(Subcommand, Debug)]
enum UtilCmd {
    /// Sanitize untrusted text for terminal output.
    Sanitize {
        /// Text to sanitize and print (joined by spaces).
        #[arg(long)]
        print: Option<Vec<String>>,
    },
    /// Enforce bounded queue limits (deterministic).
    Queue {
        /// Number of items to enqueue.
        #[arg(long)]
        len: usize,
    },
    /// Enforce bounded history limits (deterministic).
    History {
        /// Number of items to record.
        #[arg(long)]
        len: usize,
    },
    /// Bounded retry demo with deterministic jitter.
    Retry {
        /// Number of forced failures before success.
        #[arg(long)]
        fail: u32,
    },
    /// Bounded timeout demo (deterministic; no infinite waits).
    Timeout {
        /// Simulated wait time (ms).
        #[arg(long)]
        wait_ms: u64,
        /// Timeout limit (ms).
        #[arg(long)]
        timeout_ms: u64,
    },
    /// Privacy envelope planner (deterministic; no secrets).
    Envelope {
        /// Number of ticks to generate.
        #[arg(long, default_value_t = 4)]
        tick_count: usize,
        /// Tick interval (ms).
        #[arg(long, default_value_t = 100)]
        interval_ms: u64,
        /// Maximum ticks allowed (bounded).
        #[arg(long, default_value_t = envelope::MAX_TICKS_DEFAULT)]
        max_ticks: usize,
        /// Maximum bundle size in bytes.
        #[arg(long, default_value_t = envelope::MAX_BUNDLE_SIZE_DEFAULT)]
        max_bundle: usize,
        /// Maximum payload count per bundle.
        #[arg(long, default_value_t = envelope::MAX_PAYLOAD_COUNT_DEFAULT)]
        max_count: usize,
        /// Payload lengths to pack (comma-separated).
        #[arg(long, value_delimiter = ',')]
        payload_lens: Vec<usize>,
    },
    /// Panic demo for lifecycle redaction verification.
    PanicDemo,
}

#[derive(Subcommand, Debug)]
enum EnvelopeCmd {
    /// Plan an ACK/receipt envelope (deterministic; no send).
    PlanAck {
        /// Require deterministic planning (no wall clock).
        #[arg(long)]
        deterministic: bool,
        /// Number of ticks to generate.
        #[arg(long, default_value_t = 1)]
        tick_count: usize,
        /// Tick interval (ms).
        #[arg(long, default_value_t = 100)]
        interval_ms: u64,
        /// Maximum ticks allowed (bounded).
        #[arg(long, default_value_t = envelope::MAX_TICKS_DEFAULT)]
        max_ticks: usize,
        /// Maximum bundle size in bytes.
        #[arg(long, default_value_t = envelope::MAX_BUNDLE_SIZE_DEFAULT)]
        max_bundle: usize,
        /// Maximum payload count per bundle.
        #[arg(long, default_value_t = envelope::MAX_PAYLOAD_COUNT_DEFAULT)]
        max_count: usize,
        /// Payload length that defines the small-message class.
        #[arg(long, default_value_t = 1)]
        small_len: usize,
    },
}

#[derive(Debug, Clone, Copy)]
enum ErrorCode {
    MissingHome,
    InvalidPolicyProfile,
    UnsafePathSymlink,
    UnsafeParentPerms,
    LockOpenFailed,
    LockContended,
    LockFailed,
    IoWriteFailed,
    IoReadFailed,
    ParseFailed,
    IdentitySecretUnavailable,
}

impl ErrorCode {
    fn as_str(self) -> &'static str {
        match self {
            ErrorCode::MissingHome => "missing_home",
            ErrorCode::InvalidPolicyProfile => "invalid_policy_profile",
            ErrorCode::UnsafePathSymlink => "unsafe_path_symlink",
            ErrorCode::UnsafeParentPerms => "unsafe_parent_perms",
            ErrorCode::LockOpenFailed => "lock_open_failed",
            ErrorCode::LockContended => "lock_contended",
            ErrorCode::LockFailed => "lock_failed",
            ErrorCode::IoWriteFailed => "io_write_failed",
            ErrorCode::IoReadFailed => "io_read_failed",
            ErrorCode::ParseFailed => "parse_failed",
            ErrorCode::IdentitySecretUnavailable => "identity_secret_unavailable",
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum ConfigSource {
    EnvOverride,
    XdgConfigHome,
    DefaultHome,
}

const CONFIG_FILE_NAME: &str = "config.txt";
const TUI_AUTOLOCK_FILE_NAME: &str = "tui_autolock.txt";
const TUI_POLL_FILE_NAME: &str = "tui_polling.txt";
const STORE_META_NAME: &str = "store.meta";
const LOCK_FILE_NAME: &str = ".qsc.lock";
const OUTBOX_FILE_NAME: &str = "outbox.json";
const SEND_STATE_NAME: &str = "send.state";
const QSP_STATUS_FILE_NAME: &str = "qsp_status.json";
const QSE_ENV_VERSION_V1: u16 = 0x0100;
const POLICY_KEY: &str = "policy_profile";
const STORE_META_TEMPLATE: &str = "store_version=1\nvmk_status=unset\nkeyslots=0\n";
const MARKER_SCHEMA_V1: u8 = 1;
const MAX_QUEUE_LEN: usize = 64;
const MAX_HISTORY_LEN: usize = 128;
const MAX_RETRY_ATTEMPTS: u32 = 5;
const RETRY_BASE_MS: u64 = 20;
const RETRY_MAX_MS: u64 = 200;
const RETRY_JITTER_MS: u64 = 10;
const MAX_TIMEOUT_MS: u64 = 2000;
const TUI_AUTOLOCK_DEFAULT_MINUTES: u64 = 10;
const TUI_AUTOLOCK_MIN_MINUTES: u64 = 1;
const TUI_AUTOLOCK_MAX_MINUTES: u64 = 120;
const TUI_POLL_DEFAULT_INTERVAL_SECONDS: u64 = 10;
const TUI_POLL_MIN_INTERVAL_SECONDS: u64 = 2;
const TUI_POLL_MAX_INTERVAL_SECONDS: u64 = 300;

#[derive(Debug, Clone, Copy)]
enum LockMode {
    Shared,
    Exclusive,
}

struct LockGuard {
    file: File,
}

impl LockGuard {
    #[cfg(unix)]
    fn lock(file: &File, mode: LockMode) -> Result<(), ErrorCode> {
        use std::io::ErrorKind;
        use std::os::unix::io::AsRawFd;
        const LOCK_SH: i32 = 1;
        const LOCK_EX: i32 = 2;
        const LOCK_NB: i32 = 4;
        let op = match mode {
            LockMode::Shared => LOCK_SH,
            LockMode::Exclusive => LOCK_EX,
        };
        let rc = unsafe { flock(file.as_raw_fd(), op | LOCK_NB) };
        if rc != 0 {
            let err = std::io::Error::last_os_error();
            if err.kind() == ErrorKind::WouldBlock {
                return Err(ErrorCode::LockContended);
            }
            return Err(ErrorCode::LockFailed);
        }
        Ok(())
    }
}

impl Drop for LockGuard {
    fn drop(&mut self) {
        #[cfg(unix)]
        {
            use std::os::unix::io::AsRawFd;
            const LOCK_UN: i32 = 8;
            let _ = unsafe { flock(self.file.as_raw_fd(), LOCK_UN) };
        }
    }
}

mod envelope;
mod vault;

const PANIC_REDACTED_MARKER: &str = "QSC_MARK/1 event=panic code=panic_redacted";
const PANIC_DEMO_SENTINEL: &str = "QSC_SECRET_PANIC_SENTINEL=SHOULD_NOT_LEAK";
static VAULT_UNLOCKED_THIS_RUN: AtomicBool = AtomicBool::new(false);

fn install_panic_redaction_hook() {
    std::panic::set_hook(Box::new(|_| {
        let _ = std::io::stderr().write_all(PANIC_REDACTED_MARKER.as_bytes());
        let _ = std::io::stderr().write_all(b"\n");
    }));
}

fn set_vault_unlocked(unlocked: bool) {
    VAULT_UNLOCKED_THIS_RUN.store(unlocked, Ordering::SeqCst);
}

fn vault_unlocked() -> bool {
    VAULT_UNLOCKED_THIS_RUN.load(Ordering::SeqCst)
}

fn bootstrap_unlock(passphrase_env: Option<&str>) {
    if vault::unlock_if_mock_provider() {
        set_vault_unlocked(true);
        return;
    }
    if allow_seed_fallback_for_tests() {
        // Deterministic test mode keeps existing seeded test workflows intact.
        set_vault_unlocked(true);
        return;
    }
    if let Some(env_name) = passphrase_env {
        if env_name.trim().is_empty() {
            print_error_marker("vault_locked");
        }
        match vault::unlock_with_passphrase_env(Some(env_name)) {
            Ok(()) => set_vault_unlocked(true),
            Err(code) => print_error_marker(code),
        }
    }
}

fn require_unlocked(op_name: &'static str) -> bool {
    if vault_unlocked() {
        return true;
    }
    emit_marker(
        "error",
        Some("vault_locked"),
        &[("op", op_name), ("reason", "explicit_unlock_required")],
    );
    process::exit(1);
}

fn main() {
    set_umask_077();
    install_panic_redaction_hook();
    let cli = Cli::parse();
    init_output_policy(cli.reveal);
    set_vault_unlocked(false);
    bootstrap_unlock(cli.unlock_passphrase_env.as_deref());
    match cli.cmd {
        None => {
            // Shell-first UX expects help by default.
            println!("QSC_MARK/1 event=help_stub");
        }
        Some(Cmd::Status) => {
            let locked = if vault_unlocked() { "false" } else { "true" };
            print_marker("status", &[("ok", "true"), ("locked", locked)]);
            let status_peer = "peer-0";
            let (status, reason) = qsp_status_tuple(status_peer);
            emit_marker(
                "qsp_status",
                None,
                &[("status", status.as_str()), ("reason", reason.as_str())],
            );
            let (peer_fp, pinned) = identity_peer_status(status_peer);
            let pinned_s = if pinned { "true" } else { "false" };
            emit_marker(
                "identity_status",
                None,
                &[
                    ("peer", status_peer),
                    ("peer_fp", peer_fp.as_str()),
                    ("pinned", pinned_s),
                ],
            );
        }
        Some(Cmd::Config { cmd }) => match cmd {
            ConfigCmd::Set { key, value } => config_set(&key, &value),
            ConfigCmd::Get { key } => config_get(&key),
        },
        Some(Cmd::Doctor {
            check_only,
            timeout_ms,
            export,
        }) => doctor_check_only(check_only, timeout_ms, export),
        Some(Cmd::Util { cmd }) => match cmd {
            UtilCmd::Sanitize { print } => util_sanitize(print),
            UtilCmd::Queue { len } => util_queue(len),
            UtilCmd::History { len } => util_history(len),
            UtilCmd::Retry { fail } => util_retry(fail),
            UtilCmd::Timeout {
                wait_ms,
                timeout_ms,
            } => util_timeout(wait_ms, timeout_ms),
            UtilCmd::Envelope {
                tick_count,
                interval_ms,
                max_ticks,
                max_bundle,
                max_count,
                payload_lens,
            } => util_envelope(
                tick_count,
                interval_ms,
                max_ticks,
                max_bundle,
                max_count,
                payload_lens,
            ),
            UtilCmd::PanicDemo => util_panic_demo(),
        },
        Some(Cmd::Envelope { cmd }) => match cmd {
            EnvelopeCmd::PlanAck {
                deterministic,
                tick_count,
                interval_ms,
                max_ticks,
                max_bundle,
                max_count,
                small_len,
            } => envelope_plan_ack(
                deterministic,
                tick_count,
                interval_ms,
                max_ticks,
                max_bundle,
                max_count,
                small_len,
            ),
        },
        Some(Cmd::Vault { cmd }) => vault::cmd_vault(cmd),
        Some(Cmd::Send {
            cmd,
            transport,
            relay,
            to,
            file,
            pad_to,
            pad_bucket,
            bucket_max,
            meta_seed,
            receipt,
        }) => match cmd {
            Some(SendCmd::Abort) => send_abort(),
            None => send_execute(SendExecuteArgs {
                transport,
                relay,
                to,
                file,
                pad_to,
                pad_bucket,
                bucket_max,
                meta_seed,
                receipt,
            }),
        },
        Some(Cmd::Receive {
            transport,
            relay,
            from,
            mailbox,
            max,
            out,
            file,
            deterministic_meta,
            interval_ms,
            poll_interval_ms,
            poll_ticks,
            batch_max_count,
            poll_max_per_tick,
            bucket_max,
            meta_seed,
            emit_receipts,
        }) => {
            if let Some(path) = file {
                if transport.is_some()
                    || relay.is_some()
                    || from.is_some()
                    || mailbox.is_some()
                    || max.is_some()
                    || out.is_some()
                    || deterministic_meta
                    || interval_ms.is_some()
                    || poll_interval_ms.is_some()
                    || poll_ticks.is_some()
                    || batch_max_count.is_some()
                    || poll_max_per_tick.is_some()
                    || bucket_max.is_some()
                    || meta_seed.is_some()
                    || emit_receipts.is_some()
                {
                    print_error_marker("recv_file_conflict");
                }
                receive_file(&path);
            } else {
                let args = ReceiveArgs {
                    transport,
                    relay,
                    from,
                    mailbox,
                    max,
                    out,
                    deterministic_meta,
                    interval_ms,
                    poll_interval_ms,
                    poll_ticks,
                    batch_max_count,
                    poll_max_per_tick,
                    bucket_max,
                    meta_seed,
                    emit_receipts,
                };
                receive_execute(args);
            }
        }
        Some(Cmd::Handshake { cmd }) => match cmd {
            HandshakeCmd::Init {
                as_label,
                peer,
                relay,
            } => handshake_init(&as_label, &peer, &relay),
            HandshakeCmd::Poll {
                as_label,
                peer,
                relay,
                max,
            } => handshake_poll(&as_label, &peer, &relay, max),
            HandshakeCmd::Status { peer } => handshake_status(peer.as_deref()),
        },
        Some(Cmd::Identity { cmd }) => match cmd {
            IdentityCmd::Show { as_label } => identity_show(&as_label),
            IdentityCmd::Rotate {
                as_label,
                confirm,
                reset_peers,
            } => identity_rotate(&as_label, confirm, reset_peers),
        },
        Some(Cmd::Peers { cmd }) => match cmd {
            PeersCmd::List => peers_list(),
        },
        Some(Cmd::Contacts { cmd }) => match cmd {
            ContactsCmd::Add { label, fp, verify } => contacts_add(&label, &fp, verify),
            ContactsCmd::Show { label } => contacts_show(&label),
            ContactsCmd::List => contacts_list(),
            ContactsCmd::Verify { label, fp, confirm } => contacts_verify(&label, &fp, confirm),
            ContactsCmd::Block { label } => contacts_block(&label),
            ContactsCmd::Unblock { label } => contacts_unblock(&label),
        },
        Some(Cmd::Timeline { cmd }) => match cmd {
            TimelineCmd::List { peer, limit } => timeline_list(&peer, limit),
            TimelineCmd::Show { peer, id } => timeline_show(&peer, &id),
            TimelineCmd::Clear { peer, confirm } => timeline_clear(&peer, confirm),
        },
        Some(Cmd::File { cmd }) => match cmd {
            FileCmd::Send {
                transport,
                relay,
                to,
                path,
                chunk_size,
                max_file_size,
                max_chunks,
            } => file_send_execute(
                transport,
                relay.as_deref(),
                to.as_str(),
                path.as_path(),
                chunk_size,
                max_file_size,
                max_chunks,
            ),
        },
        Some(Cmd::Tui {
            headless,
            transport,
            relay,
            seed,
            scenario,
        }) => tui_entry(
            headless,
            TuiConfig {
                transport,
                relay,
                seed,
                scenario,
            },
        ),
        Some(Cmd::Relay { cmd }) => relay_cmd(cmd),
        Some(Cmd::Meta { cmd }) => meta_cmd(cmd),
    }
}

fn relay_cmd(cmd: RelayCmd) {
    match cmd {
        RelayCmd::Serve {
            port,
            seed,
            drop_pct,
            dup_pct,
            reorder_window,
            fixed_latency_ms,
            jitter_ms,
            max_messages,
        } => {
            if drop_pct > 100 || dup_pct > 100 {
                print_error_marker("relay_pct_invalid");
            }
            let cfg = RelayConfig {
                seed,
                drop_pct,
                dup_pct,
                reorder_window,
                fixed_latency_ms,
                jitter_ms,
            };
            relay_serve(port, cfg, max_messages);
        }
        RelayCmd::Send {
            to,
            file,
            relay,
            bucket_max,
        } => {
            if !require_unlocked("relay_send") {
                return;
            }
            relay_send(&to, &file, &relay, None, bucket_max, None, None)
        }
    }
}

fn meta_cmd(cmd: MetaCmd) {
    match cmd {
        MetaCmd::Plan {
            deterministic,
            tick_count,
            interval_ms,
            bucket_max,
            batch_max_count,
            cover_enabled,
        } => {
            let cfg = match meta_poll_config_from_args(MetaPollArgs {
                deterministic_meta: deterministic,
                interval_ms: Some(interval_ms),
                poll_interval_ms: None,
                ticks: Some(tick_count),
                batch_max_count: Some(batch_max_count),
                poll_max_per_tick: None,
                bucket_max: Some(bucket_max),
                meta_seed: None,
            }) {
                Ok(Some(v)) => v,
                Ok(None) => print_error_marker("meta_poll_invalid"),
                Err(code) => print_error_marker(code),
            };
            let deterministic_s = if cfg.deterministic { "true" } else { "false" };
            let ticks_s = cfg.ticks.to_string();
            let interval_s = cfg.interval_ms.to_string();
            let bucket_s = cfg.bucket_max.to_string();
            let batch_s = cfg.batch_max_count.to_string();
            emit_marker(
                "meta_plan",
                None,
                &[
                    ("deterministic", deterministic_s),
                    ("ticks", ticks_s.as_str()),
                    ("interval_ms", interval_s.as_str()),
                    ("bucket_max", bucket_s.as_str()),
                    ("batch_max_count", batch_s.as_str()),
                ],
            );
            for tick in 0..cfg.ticks {
                let tick_s = tick.to_string();
                let bucket = meta_bucket_for_len(1, cfg.bucket_max);
                let bucket_out_s = bucket.to_string();
                let planned_count_s = cfg.batch_max_count.to_string();
                emit_marker(
                    "meta_tick",
                    None,
                    &[
                        ("tick", tick_s.as_str()),
                        ("interval_ms", interval_s.as_str()),
                        ("deterministic", deterministic_s),
                    ],
                );
                emit_marker(
                    "meta_bucket",
                    None,
                    &[
                        ("bucket", bucket_out_s.as_str()),
                        ("orig", "1"),
                        ("capped", "1"),
                        ("metric", "planned_envelope_len"),
                    ],
                );
                emit_marker(
                    "meta_batch",
                    None,
                    &[
                        ("count", planned_count_s.as_str()),
                        ("bytes", "0"),
                        ("planned", "true"),
                    ],
                );
                if cover_enabled {
                    emit_marker(
                        "meta_cover",
                        None,
                        &[("enabled", "true"), ("tick", tick_s.as_str())],
                    );
                }
            }
        }
    }
}

struct TuiConfig {
    transport: Option<TuiTransport>,
    relay: Option<String>,
    seed: u64,
    scenario: String,
}

#[derive(Clone)]
struct TuiRelayConfig {
    relay: String,
    seed: u64,
    scenario: String,
}

fn tui_entry(headless: bool, cfg: TuiConfig) {
    let headless = headless || env_bool("QSC_TUI_HEADLESS");
    if headless {
        tui_headless(cfg);
        return;
    }
    if env_bool("QSC_TUI_TEST_MODE") {
        tui_interactive_test(cfg);
        return;
    }
    if let Err(e) = tui_interactive(cfg) {
        emit_marker("tui_error", Some("io"), &[("stage", "interactive")]);
        eprintln!("tui_error: {}", e);
        process::exit(1);
    }
}

fn tui_headless(cfg: TuiConfig) {
    set_marker_routing(MarkerRouting::Stdout);
    let mut state = TuiState::new(cfg);
    emit_marker("tui_open", None, &[]);
    state.emit_home_render_marker(terminal_cols_for_headless(), terminal_rows_for_headless());
    for line in load_tui_script() {
        if let Some(wait_ms) = parse_tui_wait_ms(&line) {
            state.headless_advance_clock(wait_ms);
            state.emit_home_render_marker(
                terminal_cols_for_headless(),
                terminal_rows_for_headless(),
            );
            continue;
        }
        if let Some(cmd) = parse_tui_command(&line) {
            if handle_tui_command(&cmd, &mut state) {
                emit_marker("tui_exit", None, &[]);
                return;
            }
            state.emit_home_render_marker(
                terminal_cols_for_headless(),
                terminal_rows_for_headless(),
            );
        } else {
            state.mark_input_activity(state.headless_now_ms());
            emit_marker("tui_input_text", None, &[("kind", "plain")]);
        }
    }
    emit_marker("tui_exit", None, &[]);
}

fn terminal_cols_for_headless() -> u16 {
    env::var("QSC_TUI_COLS")
        .ok()
        .and_then(|v| v.parse::<u16>().ok())
        .filter(|v| *v > 0)
        .unwrap_or(140)
}

fn terminal_rows_for_headless() -> u16 {
    env::var("QSC_TUI_ROWS")
        .ok()
        .and_then(|v| v.parse::<u16>().ok())
        .filter(|v| *v > 0)
        .unwrap_or(40)
}

fn tui_deterministic_timestamps() -> bool {
    env_bool("QSC_TUI_DETERMINISTIC") || env_bool("QSC_TUI_HEADLESS")
}

fn tui_timestamp_token(idx: usize) -> String {
    format!("t={:04}", idx.saturating_add(1))
}

fn tui_interactive(cfg: TuiConfig) -> std::io::Result<()> {
    set_marker_routing(MarkerRouting::InApp);
    let mut state = TuiState::new(cfg);
    emit_marker("tui_open", None, &[]);
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let started = Instant::now();

    let mut exit = false;
    let result = loop {
        let now_ms = started.elapsed().as_millis() as u64;
        state.drain_marker_queue();
        let force_full_redraw = state.take_force_full_redraw() || state.take_clear_screen_pending();
        terminal.draw(|f| {
            if force_full_redraw {
                let area = f.size();
                f.render_widget(TuiClear, area);
            }
            draw_tui(f, &state);
        })?;

        if event::poll(Duration::from_millis(200))? {
            if let Event::Key(key) = event::read()? {
                state.mark_input_activity(now_ms);
                exit = handle_tui_key(&mut state, key);
            }
        } else {
            state.maybe_autolock(now_ms);
        }
        state.maybe_run_fixed_poll(now_ms);
        if exit {
            break Ok(());
        }
    };

    let _ = disable_raw_mode();
    let _ = execute!(terminal.backend_mut(), LeaveAlternateScreen);
    let _ = terminal.show_cursor();
    if result.is_ok() {
        emit_marker("tui_exit", None, &[]);
    } else {
        emit_marker("tui_exit", Some("io"), &[]);
    }
    result
}

fn focus_mode_for_fkey(code: KeyCode) -> Option<TuiMode> {
    match code {
        KeyCode::F(2) => Some(TuiMode::FocusEvents),
        KeyCode::F(3) => Some(TuiMode::FocusStatus),
        KeyCode::F(4) => Some(TuiMode::FocusSession),
        KeyCode::F(5) => Some(TuiMode::FocusContacts),
        _ => None,
    }
}

fn inspector_for_fkey(code: KeyCode) -> Option<TuiInspectorPane> {
    match code {
        KeyCode::F(2) => Some(TuiInspectorPane::Events),
        KeyCode::F(3) => Some(TuiInspectorPane::Status),
        KeyCode::F(4) => Some(TuiInspectorPane::Session),
        KeyCode::F(5) => Some(TuiInspectorPane::Contacts),
        _ => None,
    }
}

fn locked_cmd_input_value(input: &str, command: &str) -> String {
    let trimmed = input.trim();
    let prefix = format!("/{}", command);
    if trimmed == prefix {
        return String::new();
    }
    if let Some(rest) = trimmed.strip_prefix(&(prefix.clone() + " ")) {
        return rest.trim().to_string();
    }
    trimmed.to_string()
}

fn handle_locked_prompt_submit(state: &mut TuiState) -> bool {
    match state.locked_flow.clone() {
        LockedFlow::None => {
            if let Some(cmd) = parse_tui_command(state.cmd_input.as_str()) {
                let exit = handle_tui_command(&cmd, state);
                state.cmd_input_clear();
                return exit;
            }
            state.cmd_input_clear();
            state.locked_clear_error();
            false
        }
        LockedFlow::UnlockPassphrase => {
            let passphrase = locked_cmd_input_value(state.cmd_input.as_str(), "unlock");
            let unlocked = if passphrase.is_empty() {
                false
            } else {
                vault::unlock_with_passphrase(passphrase.as_str()).is_ok()
            };
            state.cmd_input_clear();
            if unlocked {
                state.set_locked_state(false, "explicit_command");
                state.locked_clear_error();
                emit_marker("tui_unlock", None, &[("ok", "true")]);
            } else {
                state.locked_set_error("passphrase required");
                emit_marker(
                    "tui_unlock",
                    Some("vault_locked"),
                    &[("ok", "false"), ("reason", "passphrase_required")],
                );
            }
            false
        }
        LockedFlow::InitAlias => {
            let alias = locked_cmd_input_value(state.cmd_input.as_str(), "init");
            if !tui_alias_is_valid(alias.as_str()) {
                state.locked_set_error("alias must be 2-32 chars [A-Za-z0-9._-]");
                emit_marker(
                    "tui_init_reject",
                    Some("alias_invalid"),
                    &[("ok", "false"), ("reason", "alias_invalid")],
                );
                state.cmd_input_clear();
                return false;
            }
            state.locked_flow = LockedFlow::InitPassphrase { alias };
            state.cmd_input_clear();
            state.locked_clear_error();
            emit_marker("tui_init_wizard", None, &[("step", "passphrase")]);
            false
        }
        LockedFlow::InitPassphrase { alias } => {
            let passphrase = state.cmd_input.clone();
            if !tui_passphrase_is_strong(passphrase.as_str()) {
                state.locked_set_error("passphrase must be 16+ chars and not weak/common");
                emit_marker(
                    "tui_init_reject",
                    Some("passphrase_weak"),
                    &[("ok", "false"), ("reason", "passphrase_weak")],
                );
                state.cmd_input_clear();
                return false;
            }
            state.locked_flow = LockedFlow::InitConfirm { alias, passphrase };
            state.cmd_input_clear();
            state.locked_clear_error();
            emit_marker("tui_init_wizard", None, &[("step", "confirm")]);
            false
        }
        LockedFlow::InitConfirm { alias, passphrase } => {
            let confirm = state.cmd_input.clone();
            state.cmd_input_clear();
            if confirm != passphrase {
                state.locked_set_error("passphrase confirmation does not match");
                emit_marker(
                    "tui_init_reject",
                    Some("passphrase_mismatch"),
                    &[("ok", "false"), ("reason", "passphrase_mismatch")],
                );
                state.locked_flow = LockedFlow::InitPassphrase { alias };
                emit_marker("tui_init_wizard", None, &[("step", "passphrase")]);
                return false;
            }
            state.locked_flow = LockedFlow::InitAck { alias, passphrase };
            state.locked_clear_error();
            emit_marker("tui_init_wizard", None, &[("step", "ack")]);
            false
        }
        LockedFlow::InitAck { alias, passphrase } => {
            let ack = state.cmd_input.trim().to_string();
            if ack != "I UNDERSTAND" {
                state.locked_set_error("type exact acknowledgement: I UNDERSTAND");
                emit_marker(
                    "tui_init_reject",
                    Some("ack_required"),
                    &[("ok", "false"), ("reason", "ack_required")],
                );
                state.cmd_input_clear();
                return false;
            }
            match tui_try_vault_init(passphrase.as_str()) {
                Ok(()) => {}
                Err(code) => {
                    state.locked_set_error("vault init failed");
                    emit_marker(
                        "tui_init_reject",
                        Some(code.as_str()),
                        &[("ok", "false"), ("reason", "vault_init_failed")],
                    );
                    state.locked_flow = LockedFlow::InitAlias;
                    emit_marker("tui_init_wizard", None, &[("step", "alias")]);
                    return false;
                }
            }
            if vault::secret_set_with_passphrase(
                "profile_alias",
                alias.as_str(),
                passphrase.as_str(),
            )
            .is_err()
            {
                emit_marker(
                    "tui_init_reject",
                    Some("alias_store_failed"),
                    &[("ok", "false"), ("reason", "alias_store_failed")],
                );
                state.locked_flow = LockedFlow::InitAlias;
                state.locked_set_error("failed to store alias");
                emit_marker("tui_init_wizard", None, &[("step", "alias")]);
                return false;
            }
            state.mark_vault_present();
            state.set_locked_state(true, "post_init_locked");
            state.locked_flow = LockedFlow::None;
            state.locked_clear_error();
            emit_marker(
                "tui_init",
                None,
                &[("ok", "true"), ("alias", "stored_local_only")],
            );
            false
        }
    }
}

fn handle_tui_locked_key(state: &mut TuiState, key: KeyEvent) -> bool {
    let no_ctrl_alt = !key
        .modifiers
        .intersects(KeyModifiers::CONTROL | KeyModifiers::ALT | KeyModifiers::SUPER);
    match key.code {
        KeyCode::Up => {
            state.nav_move(-1);
            false
        }
        KeyCode::Down => {
            state.nav_move(1);
            false
        }
        KeyCode::Enter => {
            if state.home_focus == TuiHomeFocus::Nav {
                state.locked_nav_activate()
            } else if state.home_focus == TuiHomeFocus::Command {
                handle_locked_prompt_submit(state)
            } else {
                false
            }
        }
        KeyCode::Tab => {
            state.locked_focus_toggle();
            false
        }
        KeyCode::Esc => {
            state.home_focus = TuiHomeFocus::Nav;
            state.locked_flow = LockedFlow::None;
            state.locked_clear_error();
            state.cmd_input_clear();
            emit_marker("tui_focus_home", None, &[("pane", state.home_focus_name())]);
            false
        }
        KeyCode::Char('/') => {
            state.home_focus = TuiHomeFocus::Command;
            state.cmd_input_push('/');
            state.locked_clear_error();
            emit_marker("tui_focus_home", None, &[("pane", state.home_focus_name())]);
            false
        }
        KeyCode::Backspace | KeyCode::Delete => {
            if state.home_focus == TuiHomeFocus::Command {
                if !state.cmd_input.is_empty() {
                    state.cmd_input_pop();
                    state.locked_clear_error();
                } else {
                    match state.locked_flow.clone() {
                        LockedFlow::InitPassphrase { alias: _ } => {
                            state.locked_flow = LockedFlow::InitAlias;
                            state.locked_clear_error();
                            emit_marker("tui_init_wizard", None, &[("step", "alias")]);
                        }
                        LockedFlow::InitConfirm { alias, .. } => {
                            state.locked_flow = LockedFlow::InitPassphrase { alias };
                            state.locked_clear_error();
                            emit_marker("tui_init_wizard", None, &[("step", "passphrase")]);
                        }
                        LockedFlow::InitAck { alias, passphrase } => {
                            state.locked_flow = LockedFlow::InitConfirm { alias, passphrase };
                            state.locked_clear_error();
                            emit_marker("tui_init_wizard", None, &[("step", "confirm")]);
                        }
                        _ => {}
                    }
                }
            }
            false
        }
        KeyCode::Char(ch) => {
            if no_ctrl_alt && state.home_focus == TuiHomeFocus::Command && !ch.is_control() {
                state.cmd_input_push(ch);
                state.locked_clear_error();
            }
            false
        }
        _ => false,
    }
}

fn handle_tui_key(state: &mut TuiState, key: KeyEvent) -> bool {
    if state.is_locked() {
        return handle_tui_locked_key(state, key);
    }
    state.clear_command_feedback();
    if state.is_help_mode() {
        match key.code {
            KeyCode::Esc => state.exit_help_mode(),
            KeyCode::F(1) => state.toggle_help_mode(),
            KeyCode::Char('?') => state.toggle_help_mode(),
            KeyCode::Up => state.help_move(-1),
            KeyCode::Down => state.help_move(1),
            _ => {}
        }
        return false;
    }
    if state.is_focus_mode() {
        match key.code {
            KeyCode::Esc => state.exit_focus_mode(),
            KeyCode::Up => {
                if state.mode == TuiMode::FocusContacts {
                    state.contacts_move(-1);
                } else if state.mode == TuiMode::FocusFiles {
                    state.files_move(-1);
                } else {
                    let max_len = state.focus_max_len();
                    state.focus_scroll_move(-1, max_len);
                }
            }
            KeyCode::Down => {
                if state.mode == TuiMode::FocusContacts {
                    state.contacts_move(1);
                } else if state.mode == TuiMode::FocusFiles {
                    state.files_move(1);
                } else {
                    let max_len = state.focus_max_len();
                    state.focus_scroll_move(1, max_len);
                }
            }
            KeyCode::PageUp => {
                if state.mode == TuiMode::FocusContacts {
                    state.contacts_move(-(state.focus_view_rows() as i32));
                } else if state.mode == TuiMode::FocusFiles {
                    state.files_move(-(state.focus_view_rows() as i32));
                } else {
                    let max_len = state.focus_max_len();
                    state.focus_scroll_move(-(state.focus_view_rows() as i32), max_len);
                }
            }
            KeyCode::PageDown => {
                if state.mode == TuiMode::FocusContacts {
                    state.contacts_move(state.focus_view_rows() as i32);
                } else if state.mode == TuiMode::FocusFiles {
                    state.files_move(state.focus_view_rows() as i32);
                } else {
                    let max_len = state.focus_max_len();
                    state.focus_scroll_move(state.focus_view_rows() as i32, max_len);
                }
            }
            _ => {
                if let Some(mode) = focus_mode_for_fkey(key.code) {
                    state.enter_focus_mode(mode);
                }
            }
        }
        return false;
    }
    match key.code {
        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => return true,
        KeyCode::Char('l') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            emit_marker("tui_cmd", None, &[("cmd", "lock_shortcut")]);
            state.set_locked_state(true, "ctrl_l_shortcut");
        }
        KeyCode::Esc => return true,
        KeyCode::Tab => {
            state.home_focus_cycle(1);
        }
        KeyCode::BackTab => {
            state.home_focus_cycle(-1);
        }
        KeyCode::F(1) | KeyCode::Char('?') => {
            if state.is_locked() {
                handle_locked_reject(state, "help", "locked_unlock_required");
            } else {
                state.toggle_help_mode();
            }
        }
        KeyCode::Enter => {
            if state.home_focus == TuiHomeFocus::Nav {
                state.nav_activate();
            } else if state.home_focus != TuiHomeFocus::Command {
                state.enter_focus_mode(state.focus_mode_for_inspector());
            } else if let Some(cmd) = parse_tui_command(state.cmd_input.as_str()) {
                let exit = handle_tui_command(&cmd, state);
                state.cmd_input_clear();
                return exit;
            } else if !state.cmd_input.is_empty() {
                emit_marker("tui_input_text", None, &[("kind", "plain")]);
            }
            state.cmd_input_clear();
        }
        KeyCode::Backspace => {
            if state.home_focus == TuiHomeFocus::Command {
                state.cmd_input_pop();
            }
        }
        KeyCode::Up => {
            state.nav_move(-1);
        }
        KeyCode::Down => {
            state.nav_move(1);
        }
        KeyCode::Char(ch) => {
            if state.home_focus == TuiHomeFocus::Command {
                state.cmd_input_push(ch);
            } else if ch == '/' {
                state.home_focus = TuiHomeFocus::Command;
                emit_marker("tui_focus_home", None, &[("pane", state.home_focus_name())]);
                state.cmd_input_push(ch);
            }
        }
        _ => {
            if key.modifiers.contains(KeyModifiers::CONTROL) {
                if let Some(mode) = focus_mode_for_fkey(key.code) {
                    state.enter_focus_mode(mode);
                }
            } else if let Some(pane) = inspector_for_fkey(key.code) {
                state.set_inspector(pane);
            }
        }
    }
    false
}

fn tui_interactive_test(cfg: TuiConfig) {
    set_marker_routing(MarkerRouting::InApp);
    let mut state = TuiState::new(cfg);
    emit_marker("tui_open", None, &[]);
    state.drain_marker_queue();
    println!("tui_test_done");
}

fn load_tui_script() -> Vec<String> {
    if let Ok(path) = env::var("QSC_TUI_SCRIPT_FILE") {
        if let Ok(text) = fs::read_to_string(path) {
            return parse_script_lines(&text);
        }
    }
    if let Ok(text) = env::var("QSC_TUI_SCRIPT") {
        return parse_script_lines(&text);
    }
    vec!["/exit".to_string()]
}

fn parse_script_lines(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    for line in text.lines() {
        for part in line.split(';') {
            let trimmed = part.trim();
            if !trimmed.is_empty() {
                out.push(trimmed.to_string());
            }
        }
    }
    out
}

struct TuiParsedCmd {
    cmd: String,
    args: Vec<String>,
}

fn parse_tui_command(line: &str) -> Option<TuiParsedCmd> {
    let trimmed = line.trim();
    if !trimmed.starts_with('/') {
        return None;
    }
    let mut parts = trimmed.trim_start_matches('/').split_whitespace();
    let cmd = parts.next()?;
    if cmd.is_empty() {
        return None;
    }
    let args = parts.map(|s| s.to_string()).collect::<Vec<_>>();
    Some(TuiParsedCmd {
        cmd: cmd.to_string(),
        args,
    })
}

fn parse_tui_wait_ms(line: &str) -> Option<u64> {
    let mut parts = line.split_whitespace();
    let head = parts.next()?;
    if !head.eq_ignore_ascii_case("wait") {
        return None;
    }
    let ms = parts.next()?.parse::<u64>().ok()?;
    if parts.next().is_some() {
        return None;
    }
    Some(ms)
}

fn parse_tui_script_key(spec: &str) -> Option<KeyEvent> {
    let raw = spec.trim();
    let normalized = raw.to_ascii_lowercase();
    match normalized.as_str() {
        "esc" => Some(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE)),
        "enter" => Some(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE)),
        "tab" => Some(KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE)),
        "shift-tab" | "s-tab" | "backtab" => {
            Some(KeyEvent::new(KeyCode::BackTab, KeyModifiers::SHIFT))
        }
        "up" => Some(KeyEvent::new(KeyCode::Up, KeyModifiers::NONE)),
        "down" => Some(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE)),
        "pgup" | "pageup" => Some(KeyEvent::new(KeyCode::PageUp, KeyModifiers::NONE)),
        "pgdn" | "pagedown" => Some(KeyEvent::new(KeyCode::PageDown, KeyModifiers::NONE)),
        "f2" => Some(KeyEvent::new(KeyCode::F(2), KeyModifiers::NONE)),
        "f3" => Some(KeyEvent::new(KeyCode::F(3), KeyModifiers::NONE)),
        "f4" => Some(KeyEvent::new(KeyCode::F(4), KeyModifiers::NONE)),
        "f5" => Some(KeyEvent::new(KeyCode::F(5), KeyModifiers::NONE)),
        "ctrl-f2" | "c-f2" => Some(KeyEvent::new(KeyCode::F(2), KeyModifiers::CONTROL)),
        "ctrl-f3" | "c-f3" => Some(KeyEvent::new(KeyCode::F(3), KeyModifiers::CONTROL)),
        "ctrl-f4" | "c-f4" => Some(KeyEvent::new(KeyCode::F(4), KeyModifiers::CONTROL)),
        "ctrl-f5" | "c-f5" => Some(KeyEvent::new(KeyCode::F(5), KeyModifiers::CONTROL)),
        "ctrl-l" | "c-l" => Some(KeyEvent::new(KeyCode::Char('l'), KeyModifiers::CONTROL)),
        "slash" => Some(KeyEvent::new(KeyCode::Char('/'), KeyModifiers::NONE)),
        "space" => Some(KeyEvent::new(KeyCode::Char(' '), KeyModifiers::NONE)),
        "backspace" => Some(KeyEvent::new(KeyCode::Backspace, KeyModifiers::NONE)),
        "delete" | "del" => Some(KeyEvent::new(KeyCode::Delete, KeyModifiers::NONE)),
        _ => {
            let mut chars = raw.chars();
            let ch = chars.next()?;
            if chars.next().is_none() && !ch.is_control() {
                Some(KeyEvent::new(KeyCode::Char(ch), KeyModifiers::NONE))
            } else {
                None
            }
        }
    }
}

fn tui_alias_is_valid(alias: &str) -> bool {
    let len = alias.chars().count();
    if !(2..=32).contains(&len) {
        return false;
    }
    alias
        .chars()
        .all(|ch| ch.is_ascii_alphanumeric() || ch == '-' || ch == '_' || ch == '.')
}

fn tui_passphrase_is_strong(passphrase: &str) -> bool {
    if passphrase.chars().count() < 16 {
        return false;
    }
    let lowered = passphrase.to_ascii_lowercase();
    if lowered.contains("password")
        || lowered.contains("qwerty")
        || lowered.contains("letmein")
        || lowered.contains("123456")
    {
        return false;
    }
    let all_same = passphrase
        .chars()
        .next()
        .map(|first| passphrase.chars().all(|ch| ch == first))
        .unwrap_or(true);
    !all_same
}

fn tui_try_vault_init(passphrase: &str) -> Result<(), String> {
    let exe = env::current_exe().map_err(|_| "spawn_failed".to_string())?;
    let out = Command::new(exe)
        .env("QSC_TUI_INIT_PASSPHRASE", passphrase)
        .args([
            "vault",
            "init",
            "--non-interactive",
            "--passphrase-env",
            "QSC_TUI_INIT_PASSPHRASE",
            "--key-source",
            "passphrase",
        ])
        .output()
        .map_err(|_| "spawn_failed".to_string())?;
    if out.status.success() {
        return Ok(());
    }
    let mut text = String::from_utf8_lossy(&out.stdout).to_string();
    text.push_str(&String::from_utf8_lossy(&out.stderr));
    for line in text.lines() {
        if let Some(code) = line.split("code=").nth(1) {
            return Err(code
                .split_whitespace()
                .next()
                .unwrap_or("vault_init_failed")
                .to_string());
        }
    }
    Err("vault_init_failed".to_string())
}

fn handle_locked_reject(state: &mut TuiState, cmd: &str, reason: &'static str) {
    state.set_command_error("locked: unlock required");
    emit_marker(
        "tui_locked_cmd_reject",
        Some("locked_unlock_required"),
        &[
            ("cmd", cmd),
            ("reason", reason),
            ("error", "locked_unlock_required"),
        ],
    );
    state
        .events
        .push_back("error: locked: unlock required".to_string());
}

fn handle_tui_locked_command(cmd: &TuiParsedCmd, state: &mut TuiState) -> Option<bool> {
    match cmd.cmd.as_str() {
        "exit" | "quit" => {
            emit_marker("tui_cmd", None, &[("cmd", "exit")]);
            Some(true)
        }
        "init" if !state.has_vault() => {
            emit_marker("tui_cmd", None, &[("cmd", "init")]);
            if cmd.args.is_empty() {
                state.start_init_prompt();
                return Some(false);
            }
            emit_marker(
                "tui_init_warning",
                None,
                &[("no_recovery", "true"), ("ack_required", "I UNDERSTAND")],
            );
            if cmd.args.len() < 4 {
                emit_marker(
                    "tui_init_reject",
                    Some("init_args_missing"),
                    &[("ok", "false"), ("required", "alias_pass_confirm_ack")],
                );
                state.start_init_prompt();
                return Some(false);
            }
            let alias = cmd.args[0].as_str();
            let passphrase = cmd.args[1].as_str();
            let confirm = cmd.args[2].as_str();
            let ack = cmd.args[3..].join(" ");
            if !tui_alias_is_valid(alias) {
                emit_marker(
                    "tui_init_reject",
                    Some("alias_invalid"),
                    &[("ok", "false"), ("reason", "alias_invalid")],
                );
                state.start_init_prompt();
                return Some(false);
            }
            if !tui_passphrase_is_strong(passphrase) {
                emit_marker(
                    "tui_init_reject",
                    Some("passphrase_weak"),
                    &[("ok", "false"), ("reason", "passphrase_weak")],
                );
                state.start_init_prompt();
                return Some(false);
            }
            if passphrase != confirm {
                emit_marker(
                    "tui_init_reject",
                    Some("passphrase_mismatch"),
                    &[("ok", "false"), ("reason", "passphrase_mismatch")],
                );
                state.start_init_prompt();
                return Some(false);
            }
            if ack != "I UNDERSTAND" {
                emit_marker(
                    "tui_init_reject",
                    Some("ack_required"),
                    &[("ok", "false"), ("reason", "ack_required")],
                );
                state.start_init_prompt();
                return Some(false);
            }
            match tui_try_vault_init(passphrase) {
                Ok(()) => {}
                Err(code) => {
                    emit_marker(
                        "tui_init_reject",
                        Some(code.as_str()),
                        &[("ok", "false"), ("reason", "vault_init_failed")],
                    );
                    state.start_init_prompt();
                    return Some(false);
                }
            }
            if vault::secret_set_with_passphrase("profile_alias", alias, passphrase).is_err() {
                emit_marker(
                    "tui_init_reject",
                    Some("alias_store_failed"),
                    &[("ok", "false"), ("reason", "alias_store_failed")],
                );
                state.start_init_prompt();
                return Some(false);
            }
            state.mark_vault_present();
            state.set_locked_state(true, "post_init_locked");
            state.locked_flow = LockedFlow::None;
            emit_marker(
                "tui_init",
                None,
                &[("ok", "true"), ("alias", "stored_local_only")],
            );
            Some(false)
        }
        "unlock" if state.has_vault() => {
            emit_marker("tui_cmd", None, &[("cmd", "unlock")]);
            if !state.is_locked() {
                emit_marker(
                    "tui_unlock",
                    None,
                    &[("ok", "true"), ("reason", "already_unlocked")],
                );
                return Some(false);
            }
            if cmd.args.is_empty() {
                state.start_unlock_prompt();
                return Some(false);
            }
            let unlocked = cmd
                .args
                .first()
                .map(|v| vault::unlock_with_passphrase(v.as_str()).is_ok())
                .unwrap_or(false);
            if unlocked {
                state.set_locked_state(false, "explicit_command");
                emit_marker("tui_unlock", None, &[("ok", "true")]);
            } else {
                emit_marker(
                    "tui_unlock",
                    Some("vault_locked"),
                    &[("ok", "false"), ("reason", "passphrase_required")],
                );
            }
            Some(false)
        }
        _ => {
            handle_locked_reject(state, cmd.cmd.as_str(), "locked_unlock_required");
            Some(false)
        }
    }
}

fn handle_tui_command(cmd: &TuiParsedCmd, state: &mut TuiState) -> bool {
    let command_label = if cmd.args.is_empty() {
        cmd.cmd.clone()
    } else {
        format!("{} {}", cmd.cmd, cmd.args.join(" "))
    };
    let before_results_len = state.cmd_results.len();
    state.begin_command_tracking(command_label.clone());
    state.mark_input_activity(state.current_now_ms());
    state.clear_command_error();
    state.clear_command_feedback();
    if cmd.cmd == "key" {
        emit_marker("tui_cmd", None, &[("cmd", "key")]);
        let spec = cmd.args.first().map(|s| s.as_str()).unwrap_or("");
        if let Some(key) = parse_tui_script_key(spec) {
            let exit = handle_tui_key(state, key);
            if state.cmd_results.len() == before_results_len {
                state.push_cmd_result(command_label.as_str(), true, "ok");
            }
            if !exit && state.command_error.is_none() {
                state.set_command_feedback("ok: key event");
            }
            state.end_command_tracking();
            return exit;
        }
        state.set_command_error("key: unknown key");
        emit_marker("tui_key_invalid", None, &[("reason", "unknown_key")]);
        state.end_command_tracking();
        return false;
    }
    if state.is_locked() {
        if let Some(exit) = handle_tui_locked_command(cmd, state) {
            if state.cmd_results.len() == before_results_len {
                if let Some(err) = state.command_error.clone() {
                    state.push_cmd_result(command_label.as_str(), false, err);
                } else {
                    state.push_cmd_result(command_label.as_str(), true, "ok");
                }
            }
            state.end_command_tracking();
            return exit;
        }
    }
    let exit = match cmd.cmd.as_str() {
        "help" => {
            emit_marker("tui_cmd", None, &[("cmd", "help")]);
            state.enter_help_mode();
            false
        }
        "exithelp" => {
            emit_marker("tui_cmd", None, &[("cmd", "exithelp")]);
            state.exit_help_mode();
            false
        }
        "focus" => {
            emit_marker("tui_cmd", None, &[("cmd", "focus")]);
            let target = cmd.args.first().map(|s| s.as_str()).unwrap_or("");
            match target {
                "events" => state.enter_focus_mode(TuiMode::FocusEvents),
                "files" => state.enter_focus_mode(TuiMode::FocusFiles),
                "activity" => state.enter_focus_mode(TuiMode::FocusActivity),
                "status" => state.enter_focus_mode(TuiMode::FocusStatus),
                "session" | "keys" => state.enter_focus_mode(TuiMode::FocusSession),
                "contacts" => state.enter_focus_mode(TuiMode::FocusContacts),
                "settings" => state.enter_focus_mode(TuiMode::FocusSettings),
                "lock" => state.enter_focus_mode(TuiMode::FocusLock),
                _ => {
                    state.set_command_error("focus: unknown pane");
                    emit_marker("tui_focus_invalid", None, &[("reason", "unknown_pane")]);
                }
            }
            false
        }
        "inspector" | "ins" => {
            emit_marker("tui_cmd", None, &[("cmd", "inspector")]);
            let target = cmd.args.first().map(|s| s.as_str()).unwrap_or("");
            match target {
                "events" => state.set_inspector(TuiInspectorPane::Events),
                "files" => state.set_inspector(TuiInspectorPane::Files),
                "activity" => state.set_inspector(TuiInspectorPane::Activity),
                "status" => state.set_inspector(TuiInspectorPane::Status),
                "overview" => state.set_inspector(TuiInspectorPane::Status),
                "cmdresults" | "results" => state.set_inspector(TuiInspectorPane::CmdResults),
                "session" | "keys" => state.set_inspector(TuiInspectorPane::Session),
                "contacts" => state.set_inspector(TuiInspectorPane::Contacts),
                "settings" => state.set_inspector(TuiInspectorPane::Settings),
                "lock" => state.set_inspector(TuiInspectorPane::Lock),
                "help" => state.set_inspector(TuiInspectorPane::Help),
                "about" => state.set_inspector(TuiInspectorPane::About),
                "legal" => state.set_inspector(TuiInspectorPane::Legal),
                _ => {
                    state.set_command_error("inspector: unknown pane");
                    emit_marker("tui_inspector_invalid", None, &[("reason", "unknown_pane")]);
                }
            }
            false
        }
        "back" | "unfocus" => {
            emit_marker("tui_cmd", None, &[("cmd", "back")]);
            state.exit_focus_mode();
            false
        }
        "down" => {
            emit_marker("tui_cmd", None, &[("cmd", "down")]);
            if state.is_focus_mode() {
                if state.mode == TuiMode::FocusContacts {
                    state.contacts_move(1);
                } else if state.mode == TuiMode::FocusFiles {
                    state.files_move(1);
                } else {
                    let max_len = state.focus_max_len();
                    state.focus_scroll_move(1, max_len);
                }
            }
            false
        }
        "up" => {
            emit_marker("tui_cmd", None, &[("cmd", "up")]);
            if state.is_focus_mode() {
                if state.mode == TuiMode::FocusContacts {
                    state.contacts_move(-1);
                } else if state.mode == TuiMode::FocusFiles {
                    state.files_move(-1);
                } else {
                    let max_len = state.focus_max_len();
                    state.focus_scroll_move(-1, max_len);
                }
            }
            false
        }
        "pgdn" | "pagedown" => {
            emit_marker("tui_cmd", None, &[("cmd", "pgdn")]);
            if state.is_focus_mode() {
                if state.mode == TuiMode::FocusContacts {
                    state.contacts_move(state.focus_view_rows() as i32);
                } else if state.mode == TuiMode::FocusFiles {
                    state.files_move(state.focus_view_rows() as i32);
                } else {
                    let max_len = state.focus_max_len();
                    state.focus_scroll_move(state.focus_view_rows() as i32, max_len);
                }
            }
            false
        }
        "pgup" | "pageup" => {
            emit_marker("tui_cmd", None, &[("cmd", "pgup")]);
            if state.is_focus_mode() {
                if state.mode == TuiMode::FocusContacts {
                    state.contacts_move(-(state.focus_view_rows() as i32));
                } else if state.mode == TuiMode::FocusFiles {
                    state.files_move(-(state.focus_view_rows() as i32));
                } else {
                    let max_len = state.focus_max_len();
                    state.focus_scroll_move(-(state.focus_view_rows() as i32), max_len);
                }
            }
            false
        }
        "exit" | "quit" => {
            emit_marker("tui_cmd", None, &[("cmd", "exit")]);
            true
        }
        "send" => {
            emit_marker("tui_cmd", None, &[("cmd", "send")]);
            if state.is_locked() {
                emit_marker(
                    "tui_send_blocked",
                    None,
                    &[("reason", "vault_locked"), ("hint", "run_qsc_vault_unlock")],
                );
                state.update_send_lifecycle("blocked");
                return false;
            }
            if state.relay.is_none() {
                emit_marker(
                    "tui_send_blocked",
                    None,
                    &[("reason", "explicit_only_no_transport")],
                );
                state.update_send_lifecycle("blocked");
            } else {
                tui_send_via_relay(state);
            }
            false
        }
        "receive" => {
            emit_marker("tui_cmd", None, &[("cmd", "receive")]);
            if state.is_locked() {
                emit_marker(
                    "tui_receive_blocked",
                    None,
                    &[("reason", "vault_locked"), ("hint", "run_qsc_vault_unlock")],
                );
                return false;
            }
            if state.relay.is_none() {
                emit_marker(
                    "tui_receive_blocked",
                    None,
                    &[("reason", "explicit_only_no_transport")],
                );
            } else {
                let peer = cmd
                    .args
                    .first()
                    .map(|s| s.as_str())
                    .unwrap_or(state.session.peer_label);
                tui_receive_via_relay(state, peer);
            }
            false
        }
        "handshake" => {
            emit_marker("tui_cmd", None, &[("cmd", "handshake")]);
            if state.is_locked() {
                emit_marker(
                    "tui_handshake_blocked",
                    None,
                    &[("reason", "vault_locked"), ("hint", "run_qsc_vault_unlock")],
                );
                return false;
            }
            let sub = cmd.args.first().map(|s| s.as_str()).unwrap_or("status");
            let peer = cmd
                .args
                .get(1)
                .map(|s| s.as_str())
                .unwrap_or(state.session.peer_label);
            let self_label = env::var("QSC_SELF_LABEL").unwrap_or_else(|_| "peer-0".to_string());
            match sub {
                "status" => {
                    handshake_status(Some(peer));
                    state
                        .events
                        .push_back(format!("handshake status peer={}", peer));
                }
                "init" => {
                    if let Some(r) = state.relay.as_ref() {
                        handshake_init(&self_label, peer, &r.relay);
                        state
                            .events
                            .push_back(format!("handshake init peer={}", peer));
                    } else {
                        emit_marker(
                            "tui_handshake_blocked",
                            None,
                            &[("reason", "explicit_only_no_transport")],
                        );
                    }
                }
                "poll" => {
                    if let Some(r) = state.relay.as_ref() {
                        handshake_poll(&self_label, peer, &r.relay, 4);
                        state
                            .events
                            .push_back(format!("handshake poll peer={}", peer));
                    } else {
                        emit_marker(
                            "tui_handshake_blocked",
                            None,
                            &[("reason", "explicit_only_no_transport")],
                        );
                    }
                }
                _ => {
                    emit_marker(
                        "tui_handshake_invalid",
                        None,
                        &[("reason", "unknown_subcmd")],
                    );
                }
            }
            false
        }
        "status" => {
            emit_marker("tui_cmd", None, &[("cmd", "status")]);
            state.refresh_envelope(state.last_payload_len());
            state.refresh_qsp_status();
            state.push_cmd_result("status", true, "system overview refreshed");
            false
        }
        "autolock" => {
            emit_marker("tui_cmd", None, &[("cmd", "autolock")]);
            let sub = cmd.args.first().map(|s| s.as_str()).unwrap_or("show");
            match sub {
                "set" => {
                    let Some(minutes_s) = cmd.args.get(1).map(|s| s.as_str()) else {
                        state.set_command_error("autolock: missing minutes");
                        emit_marker(
                            "tui_autolock_set",
                            Some("autolock_invalid_minutes"),
                            &[("ok", "false"), ("reason", "missing_minutes")],
                        );
                        return false;
                    };
                    let Ok(minutes) = minutes_s.parse::<u64>() else {
                        state.set_command_error("autolock: invalid minutes");
                        emit_marker(
                            "tui_autolock_set",
                            Some("autolock_invalid_minutes"),
                            &[("ok", "false"), ("reason", "invalid_minutes")],
                        );
                        return false;
                    };
                    if let Err(code) = state.set_autolock_minutes(minutes) {
                        state.set_command_error(format!("autolock: {}", code));
                        emit_marker("tui_autolock_set", Some(code), &[("ok", "false")]);
                    } else {
                        state.set_status_last_command_result(format!(
                            "autolock set {} min",
                            minutes
                        ));
                        state.push_cmd_result(
                            "autolock set",
                            true,
                            format!("timeout={} min", minutes),
                        );
                    }
                }
                "show" => {
                    let minutes_s = state.autolock_minutes().to_string();
                    emit_marker(
                        "tui_autolock_show",
                        None,
                        &[("ok", "true"), ("minutes", minutes_s.as_str())],
                    );
                    state.set_status_last_command_result(format!(
                        "autolock {} min",
                        state.autolock_minutes()
                    ));
                    state.push_cmd_result(
                        "autolock show",
                        true,
                        format!("timeout={} min", state.autolock_minutes()),
                    );
                }
                _ => {
                    state.set_command_error("autolock: unknown subcommand");
                    emit_marker(
                        "tui_autolock_set",
                        Some("autolock_invalid_subcmd"),
                        &[("ok", "false"), ("reason", "unknown_subcmd")],
                    );
                }
            }
            false
        }
        "poll" | "polling" => {
            emit_marker("tui_cmd", None, &[("cmd", "poll")]);
            let sub = cmd.args.first().map(|s| s.as_str()).unwrap_or("show");
            match sub {
                "show" => {
                    state.emit_poll_show_marker();
                    state.set_status_last_command_result(format!(
                        "poll {} {}s",
                        state.poll_mode().as_str(),
                        state.poll_interval_seconds()
                    ));
                    state.push_cmd_result(
                        "poll show",
                        true,
                        format!(
                            "mode={} interval={}s",
                            state.poll_mode().as_str(),
                            state.poll_interval_seconds()
                        ),
                    );
                }
                "set" => {
                    let Some(mode) = cmd.args.get(1).map(|s| s.as_str()) else {
                        state.set_command_error("poll: missing mode");
                        emit_marker(
                            "tui_poll_set",
                            Some("poll_invalid_subcmd"),
                            &[("ok", "false"), ("reason", "missing_mode")],
                        );
                        return false;
                    };
                    match mode {
                        "adaptive" => {
                            if let Err(code) = state.set_poll_mode_adaptive() {
                                state.set_command_error(format!("poll: {}", code));
                                emit_marker("tui_poll_set", Some(code), &[("ok", "false")]);
                            } else {
                                state.set_status_last_command_result("poll set adaptive");
                                state.push_cmd_result("poll set adaptive", true, "mode=adaptive");
                            }
                        }
                        "fixed" => {
                            let Some(seconds_s) = cmd.args.get(2).map(|s| s.as_str()) else {
                                state.set_command_error("poll: missing seconds");
                                emit_marker(
                                    "tui_poll_set",
                                    Some("poll_invalid_seconds"),
                                    &[("ok", "false"), ("reason", "missing_seconds")],
                                );
                                return false;
                            };
                            let Ok(seconds) = seconds_s.parse::<u64>() else {
                                state.set_command_error("poll: invalid seconds");
                                emit_marker(
                                    "tui_poll_set",
                                    Some("poll_invalid_seconds"),
                                    &[("ok", "false"), ("reason", "invalid_seconds")],
                                );
                                return false;
                            };
                            let now_ms = state.current_now_ms();
                            if let Err(code) = state.set_poll_mode_fixed(seconds, now_ms) {
                                state.set_command_error(format!("poll: {}", code));
                                emit_marker("tui_poll_set", Some(code), &[("ok", "false")]);
                            } else {
                                state.set_status_last_command_result(format!(
                                    "poll set fixed {}s",
                                    seconds
                                ));
                                state.push_cmd_result(
                                    "poll set fixed",
                                    true,
                                    format!("interval={}s", seconds),
                                );
                            }
                        }
                        _ => {
                            state.set_command_error("poll: unknown mode");
                            emit_marker(
                                "tui_poll_set",
                                Some("poll_invalid_subcmd"),
                                &[("ok", "false"), ("reason", "unknown_mode")],
                            );
                        }
                    }
                }
                _ => {
                    state.set_command_error("poll: unknown subcommand");
                    emit_marker(
                        "tui_poll_set",
                        Some("poll_invalid_subcmd"),
                        &[("ok", "false"), ("reason", "unknown_subcmd")],
                    );
                }
            }
            false
        }
        "lock" => {
            emit_marker("tui_cmd", None, &[("cmd", "lock")]);
            state.set_locked_state(true, "explicit_command");
            false
        }
        "unlock" => {
            emit_marker("tui_cmd", None, &[("cmd", "unlock")]);
            if !state.is_locked() {
                emit_marker(
                    "tui_unlock",
                    None,
                    &[("ok", "true"), ("reason", "already_unlocked")],
                );
                return false;
            }
            if vault::unlock_if_mock_provider()
                || vault::unlock_with_passphrase_env(Some("QSC_PASSPHRASE")).is_ok()
            {
                state.set_locked_state(false, "explicit_command");
                emit_marker("tui_unlock", None, &[("ok", "true")]);
            } else {
                emit_marker(
                    "tui_unlock",
                    Some("vault_locked"),
                    &[("ok", "false"), ("reason", "passphrase_required")],
                );
            }
            false
        }
        "contacts" => {
            emit_marker("tui_cmd", None, &[("cmd", "contacts")]);
            let sub = cmd.args.first().map(|s| s.as_str()).unwrap_or("list");
            match sub {
                "list" => {
                    state.refresh_contacts();
                    let count_s = state.contacts.len().to_string();
                    emit_marker("tui_contacts_list", None, &[("count", count_s.as_str())]);
                }
                "block" => {
                    let Some(label) = cmd.args.get(1).map(|s| s.as_str()) else {
                        state.set_command_error("contacts: missing label");
                        emit_marker("tui_contacts_invalid", None, &[("reason", "missing_label")]);
                        return false;
                    };
                    match contacts_set_blocked(label, true) {
                        Ok(true) => emit_marker(
                            "tui_contacts_block",
                            None,
                            &[("label", label), ("ok", "true")],
                        ),
                        Ok(false) => emit_marker(
                            "tui_contacts_block",
                            Some("peer_unknown"),
                            &[("label", label), ("ok", "false")],
                        ),
                        Err(_) => emit_marker(
                            "tui_contacts_block",
                            Some("contacts_store_unavailable"),
                            &[("label", label), ("ok", "false")],
                        ),
                    }
                    state.refresh_contacts();
                }
                "unblock" => {
                    let Some(label) = cmd.args.get(1).map(|s| s.as_str()) else {
                        state.set_command_error("contacts: missing label");
                        emit_marker("tui_contacts_invalid", None, &[("reason", "missing_label")]);
                        return false;
                    };
                    match contacts_set_blocked(label, false) {
                        Ok(true) => emit_marker(
                            "tui_contacts_unblock",
                            None,
                            &[("label", label), ("ok", "true")],
                        ),
                        Ok(false) => emit_marker(
                            "tui_contacts_unblock",
                            Some("peer_unknown"),
                            &[("label", label), ("ok", "false")],
                        ),
                        Err(_) => emit_marker(
                            "tui_contacts_unblock",
                            Some("contacts_store_unavailable"),
                            &[("label", label), ("ok", "false")],
                        ),
                    }
                    state.refresh_contacts();
                }
                "add" => {
                    let Some(label) = cmd.args.get(1).map(|s| s.as_str()) else {
                        state.set_command_error("contacts: missing label");
                        emit_marker("tui_contacts_invalid", None, &[("reason", "missing_label")]);
                        return false;
                    };
                    let Some(fp) = cmd.args.get(2).map(|s| s.as_str()) else {
                        state.set_command_error("contacts: missing fingerprint");
                        emit_marker("tui_contacts_invalid", None, &[("reason", "missing_fp")]);
                        return false;
                    };
                    let rec = ContactRecord {
                        fp: fp.to_string(),
                        status: "pinned".to_string(),
                        blocked: false,
                        seen_at: None,
                        sig_fp: None,
                    };
                    match contacts_entry_upsert(label, rec) {
                        Ok(()) => emit_marker(
                            "tui_contacts_add",
                            None,
                            &[("label", label), ("ok", "true"), ("status", "pinned")],
                        ),
                        Err(_) => emit_marker(
                            "tui_contacts_add",
                            Some("contacts_store_unavailable"),
                            &[("label", label), ("ok", "false")],
                        ),
                    }
                    state.refresh_contacts();
                }
                _ => {
                    state.set_command_error("contacts: unknown subcommand");
                    emit_marker(
                        "tui_contacts_invalid",
                        None,
                        &[("reason", "unknown_subcmd")],
                    );
                }
            }
            false
        }
        "messages" => {
            emit_marker("tui_cmd", None, &[("cmd", "messages")]);
            let sub = cmd.args.first().map(|s| s.as_str()).unwrap_or("list");
            match sub {
                "list" => {
                    let labels = state.conversation_labels();
                    let count_s = labels.len().to_string();
                    emit_marker("tui_messages_list", None, &[("count", count_s.as_str())]);
                }
                "select" => {
                    let Some(peer) = cmd.args.get(1).map(|s| s.as_str()) else {
                        state.set_command_error("messages: missing peer");
                        emit_marker("tui_messages_invalid", None, &[("reason", "missing_peer")]);
                        return false;
                    };
                    state.ensure_conversation(peer);
                    let labels = state.conversation_labels();
                    if let Some(idx) = labels.iter().position(|p| p == peer) {
                        state.conversation_selected = idx;
                        state.set_active_peer(peer);
                        state.sync_messages_if_main_focused();
                        emit_marker(
                            "tui_messages_select",
                            None,
                            &[("peer", peer), ("ok", "true")],
                        );
                    } else {
                        emit_marker(
                            "tui_messages_select",
                            None,
                            &[("peer", peer), ("ok", "false")],
                        );
                    }
                }
                _ => {
                    state.set_command_error("messages: unknown subcommand");
                    emit_marker(
                        "tui_messages_invalid",
                        None,
                        &[("reason", "unknown_subcmd")],
                    );
                }
            }
            false
        }
        "files" => {
            emit_marker("tui_cmd", None, &[("cmd", "files")]);
            state.refresh_files_from_timeline();
            let sub = cmd.args.first().map(|s| s.as_str()).unwrap_or("list");
            match sub {
                "list" => {
                    emit_marker(
                        "tui_files_list",
                        None,
                        &[("count", state.files.len().to_string().as_str())],
                    );
                }
                "select" => {
                    let Some(id) = cmd.args.get(1).map(|s| s.as_str()) else {
                        state.set_command_error("files: missing id");
                        emit_marker("tui_files_invalid", None, &[("reason", "missing_id")]);
                        return false;
                    };
                    let ok = state.files_select_by_id(id);
                    emit_marker(
                        "tui_files_select",
                        None,
                        &[("id", id), ("ok", if ok { "true" } else { "false" })],
                    );
                }
                "toggle" => {
                    if state.inspector != TuiInspectorPane::Files {
                        state.set_command_error("files: multiselect only allowed in files");
                        emit_marker(
                            "tui_files_multiselect_blocked",
                            None,
                            &[
                                ("reason", "domain_not_files"),
                                ("domain", state.inspector_name()),
                            ],
                        );
                        return false;
                    }
                    if let Some(id) = cmd.args.get(1).map(|s| s.as_str()) {
                        if !state.files_select_by_id(id) {
                            state.set_command_error("files: unknown id");
                            emit_marker(
                                "tui_files_toggle",
                                None,
                                &[("id", id), ("ok", "false"), ("reason", "unknown_id")],
                            );
                            return false;
                        }
                    }
                    let ok = state.files_toggle_selected();
                    emit_marker(
                        "tui_files_toggle",
                        None,
                        &[
                            ("id", state.selected_file_id().unwrap_or("none")),
                            ("ok", if ok { "true" } else { "false" }),
                            (
                                "selected",
                                state.file_multi_selected.len().to_string().as_str(),
                            ),
                        ],
                    );
                }
                "clear-selection" => {
                    state.file_multi_selected.clear();
                    emit_marker(
                        "tui_files_clear_selection",
                        None,
                        &[("ok", "true"), ("selected", "0")],
                    );
                }
                "inject" => {
                    let Some(id) = cmd.args.get(1).map(|s| s.as_str()) else {
                        state.set_command_error("files: missing id");
                        emit_marker("tui_files_invalid", None, &[("reason", "missing_id")]);
                        return false;
                    };
                    let state_name = cmd.args.get(2).map(|s| s.as_str()).unwrap_or("RECEIVING");
                    let byte_len = cmd
                        .args
                        .get(3)
                        .and_then(|v| v.parse::<usize>().ok())
                        .unwrap_or(0usize);
                    let filename = cmd
                        .args
                        .get(4)
                        .map(|s| s.to_string())
                        .unwrap_or_else(|| format!("{}.bin", id));
                    let item = TuiFileItem {
                        id: id.to_string(),
                        peer: state.selected_conversation_label(),
                        filename,
                        byte_len,
                        state: state_name.to_string(),
                        display_state: tui_file_display_state(state_name),
                    };
                    state.upsert_file_item(item, true);
                    emit_marker(
                        "tui_files_inject",
                        None,
                        &[
                            ("id", id),
                            ("state", state_name),
                            ("display_state", tui_file_display_state(state_name).as_str()),
                        ],
                    );
                }
                _ => {
                    state.set_command_error("files: unknown subcommand");
                    emit_marker("tui_files_invalid", None, &[("reason", "unknown_subcmd")]);
                }
            }
            false
        }
        "injectmsg" => {
            emit_marker("tui_cmd", None, &[("cmd", "injectmsg")]);
            let peer = cmd.args.first().map(|s| s.as_str()).unwrap_or("peer-0");
            let state_name = cmd.args.get(1).map(|s| s.as_str()).unwrap_or("RECEIVED");
            state.record_message_line(peer, state_name, "in", "source=test_harness");
            false
        }
        "injectevent" => {
            emit_marker("tui_cmd", None, &[("cmd", "injectevent")]);
            let kind = cmd.args.first().map(|s| s.as_str()).unwrap_or("activity");
            let action = cmd.args.get(1).map(|s| s.as_str()).unwrap_or("test");
            state.push_event(kind, action);
            false
        }
        "envelope" => {
            emit_marker("tui_cmd", None, &[("cmd", "envelope")]);
            state.refresh_envelope(state.last_payload_len());
            false
        }
        "export" => {
            emit_marker("tui_cmd", None, &[("cmd", "export")]);
            false
        }
        other => {
            state.set_command_error(format!("unknown command: {}", other));
            emit_marker("tui_cmd", None, &[("cmd", other)]);
            false
        }
    };
    if state.cmd_results.len() == before_results_len {
        if let Some(err) = state.command_error.clone() {
            state.push_cmd_result(command_label.as_str(), false, err);
        } else {
            state.push_cmd_result(command_label.as_str(), true, "ok");
        }
    }
    if !exit && state.command_error.is_none() {
        if let Some(entry) = state.cmd_results.back() {
            if let Some(msg) = entry
                .strip_prefix("[ok] /")
                .and_then(|v| v.split_once(' '))
                .map(|(_, msg)| msg.to_string())
            {
                state.set_command_feedback(format!("ok: {}", msg));
            } else {
                state.set_command_feedback(format!("ok: /{}", command_label));
            }
        } else {
            state.set_command_feedback(format!("ok: /{}", command_label));
        }
    }
    state.end_command_tracking();
    exit
}

fn tui_send_via_relay(state: &mut TuiState) {
    let relay = match state.relay.as_ref() {
        Some(v) => v,
        None => {
            emit_marker(
                "tui_send_blocked",
                None,
                &[("reason", "explicit_only_no_transport")],
            );
            state.update_send_lifecycle("blocked");
            return;
        }
    };
    let to = state.session.peer_label;
    match contact_blocked(to) {
        Ok(true) => {
            emit_marker("tui_send_blocked", None, &[("reason", "peer_blocked")]);
            state.update_send_lifecycle("blocked");
            return;
        }
        Ok(false) => {}
        Err(_) => {
            emit_marker(
                "tui_send_blocked",
                None,
                &[("reason", "contacts_store_unavailable")],
            );
            state.update_send_lifecycle("blocked");
            return;
        }
    }
    if let Err(reason) = protocol_active_or_reason_for_peer(to) {
        emit_protocol_inactive(reason.as_str());
        state.update_send_lifecycle("blocked");
        return;
    }
    let payload = tui_payload_bytes(state.send_seq);
    state.send_seq = state.send_seq.wrapping_add(1);
    let outcome = relay_send_with_payload(RelaySendPayloadArgs {
        to,
        payload,
        relay: relay.relay.as_str(),
        injector: fault_injector_from_tui(relay),
        pad_cfg: None,
        bucket_max: None,
        meta_seed: None,
        receipt: None,
    });
    state.push_event("relay_event", outcome.action.as_str());
    if outcome.delivered {
        state.update_send_lifecycle("committed");
        state.session.sent_count = state.session.sent_count.saturating_add(1);
        state.record_message_line(to, "SENT", "out", "transport=relay");
    } else {
        state.update_send_lifecycle("failed");
    }
    state.refresh_qsp_status();
}

fn resolve_receive_out_dir() -> (PathBuf, ConfigSource) {
    if let Ok(dir) = env::var("QSC_RECEIVE_OUT") {
        return (PathBuf::from(dir), ConfigSource::EnvOverride);
    }
    let (dir, source) = match config_dir() {
        Ok(v) => v,
        Err(e) => print_error(e),
    };
    (dir.join("inbox"), source)
}

fn tui_receive_via_relay(state: &mut TuiState, from: &str) {
    let relay = match state.relay.as_ref() {
        Some(v) => v,
        None => {
            emit_marker(
                "tui_receive_blocked",
                None,
                &[("reason", "explicit_only_no_transport")],
            );
            return;
        }
    };
    if !relay_is_http(&relay.relay) {
        emit_marker(
            "tui_receive_blocked",
            None,
            &[("reason", "relay_http_required")],
        );
        return;
    }
    let (out_dir, source) = resolve_receive_out_dir();
    if let Err(e) = ensure_dir_secure(&out_dir, source) {
        print_error(e);
    }
    let (cfg_dir, cfg_source) = match config_dir() {
        Ok(v) => v,
        Err(e) => print_error(e),
    };
    if !check_symlink_safe(&cfg_dir) {
        print_error(ErrorCode::UnsafePathSymlink);
    }
    if !check_parent_safe(&cfg_dir, cfg_source) {
        print_error(ErrorCode::UnsafeParentPerms);
    }
    if let Err(reason) = protocol_active_or_reason_for_peer(from) {
        emit_protocol_inactive(reason.as_str());
        emit_marker("tui_receive", None, &[("from", from), ("count", "0")]);
        state.push_event("recv_blocked", reason.as_str());
        return;
    }
    emit_marker(
        "recv_start",
        None,
        &[("transport", "relay"), ("from", from), ("max", "1")],
    );
    let max = 1usize;
    let items = match relay_inbox_pull(&relay.relay, from, max) {
        Ok(v) => v,
        Err(code) => print_error_marker(code),
    };
    if items.is_empty() {
        emit_marker("recv_none", None, &[]);
        emit_marker("tui_receive", None, &[("from", from), ("count", "0")]);
        state.push_event("recv_none", from);
        return;
    }
    let mut count = 0usize;
    for item in items {
        match qsp_unpack(from, &item.data) {
            Ok(outcome) => {
                record_qsp_status(&cfg_dir, cfg_source, true, "unpack_ok", false, true);
                emit_marker("qsp_unpack", None, &[("ok", "true"), ("version", "5.0")]);
                let msg_idx_s = outcome.msg_idx.to_string();
                emit_marker(
                    "ratchet_recv_advance",
                    None,
                    &[("msg_idx", msg_idx_s.as_str())],
                );
                if outcome.skip_delta > 0 {
                    let sd = outcome.skip_delta.to_string();
                    emit_marker("ratchet_skip_store", None, &[("count", sd.as_str())]);
                }
                if outcome.evicted > 0 {
                    let ev = outcome.evicted.to_string();
                    emit_marker("ratchet_skip_evict", None, &[("count", ev.as_str())]);
                }
                if qsp_session_store(from, &outcome.next_state).is_err() {
                    emit_marker("error", Some("qsp_session_store_failed"), &[]);
                    print_error_marker("qsp_session_store_failed");
                }
                count = count.saturating_add(1);
                let seq = state.session.recv_count.saturating_add(count as u64);
                let name = format!("recv_{}.bin", seq);
                let path = out_dir.join(&name);
                if write_atomic(&path, &outcome.plaintext, source).is_err() {
                    print_error_marker("recv_write_failed");
                }
                let size_s = outcome.plaintext.len().to_string();
                emit_marker(
                    "recv_item",
                    None,
                    &[
                        ("idx", count.to_string().as_str()),
                        ("size", size_s.as_str()),
                        ("id", item.id.as_str()),
                    ],
                );
                state.events.push_back(format!(
                    "recv: from={} size={} saved={}",
                    from, size_s, name
                ));
                let detail = format!("bytes={}", size_s);
                state.record_message_line(from, "RECEIVED", "in", detail.as_str());
            }
            Err(code) => {
                record_qsp_status(&cfg_dir, cfg_source, false, code, false, false);
                emit_marker("qsp_unpack", Some(code), &[("ok", "false")]);
                if code == "qsp_replay_reject" {
                    let msg_idx = qsp_session_for_channel(from)
                        .map(|st| st.recv.nr.to_string())
                        .unwrap_or_else(|_| "0".to_string());
                    emit_marker("ratchet_replay_reject", None, &[("msg_idx", &msg_idx)]);
                }
                print_error_marker(code);
            }
        }
    }
    state.session.recv_count = state.session.recv_count.saturating_add(count as u64);
    let count_s = count.to_string();
    emit_marker(
        "tui_receive",
        None,
        &[("from", from), ("count", count_s.as_str())],
    );
    emit_marker("recv_commit", None, &[("count", count_s.as_str())]);
    state.refresh_qsp_status();
}

fn tui_payload_bytes(seq: u64) -> Vec<u8> {
    // Deterministic, non-secret payload bytes.
    format!("tui_msg_seq={}", seq).into_bytes()
}

fn draw_tui(f: &mut ratatui::Frame, state: &TuiState) {
    let area = f.size();
    match state.mode {
        TuiMode::Help => {
            draw_help_mode(f, area, state);
            return;
        }
        TuiMode::FocusEvents => {
            draw_focus_events(f, area, state);
            return;
        }
        TuiMode::FocusFiles => {
            draw_focus_files(f, area, state);
            return;
        }
        TuiMode::FocusActivity => {
            draw_focus_activity(f, area, state);
            return;
        }
        TuiMode::FocusStatus => {
            draw_focus_status(f, area, state);
            return;
        }
        TuiMode::FocusSession => {
            draw_focus_session(f, area, state);
            return;
        }
        TuiMode::FocusContacts => {
            draw_focus_contacts(f, area, state);
            return;
        }
        TuiMode::FocusSettings => {
            draw_focus_settings(f, area, state);
            return;
        }
        TuiMode::FocusLock => {
            draw_focus_lock(f, area, state);
            return;
        }
        TuiMode::Normal => {}
    }
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(3), Constraint::Length(3)].as_ref())
        .split(area);
    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(26), Constraint::Percentage(74)].as_ref())
        .split(rows[0]);
    render_unified_nav(f, cols[0], state);
    render_main_panel(f, cols[1], state);

    let cmd_text = state.cmd_bar_text();
    let cmd = Paragraph::new(Line::from(vec![Span::styled(
        cmd_text.as_str(),
        state.cmd_bar_style(cmd_text.as_str()),
    )]))
    .block(Block::default().borders(Borders::ALL));
    f.render_widget(cmd, rows[1]);
}

fn draw_help_mode(f: &mut ratatui::Frame, area: Rect, state: &TuiState) {
    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)].as_ref())
        .split(area);

    let items = tui_help_items();
    let list_items: Vec<ListItem> = items
        .iter()
        .map(|item| ListItem::new(format!("/{} — {}", item.cmd, item.desc)))
        .collect();
    let mut list_state = ratatui::widgets::ListState::default();
    list_state.select(Some(state.help_selected.min(items.len().saturating_sub(1))));

    let list = List::new(list_items)
        .block(Block::default().borders(Borders::ALL).title("Help"))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD));
    f.render_stateful_widget(list, cols[0], &mut list_state);

    let detail = state.help_selected_item();
    let detail_body = match detail {
        Some(item) => format!("command: /{}\n\n{}", item.cmd, item.desc),
        None => "no help items".to_string(),
    };
    let details =
        Paragraph::new(detail_body).block(Block::default().borders(Borders::ALL).title("Details"));
    f.render_widget(details, cols[1]);
}

fn draw_focus_events(f: &mut ratatui::Frame, area: Rect, state: &TuiState) {
    let body = state.focus_events_lines().join("\n");
    let panel = Paragraph::new(body)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("FOCUS: EVENTS (Up/Down PgUp/PgDn Esc back)"),
        )
        .scroll((state.focus_scroll_index() as u16, 0));
    f.render_widget(panel, area);
}

fn draw_focus_files(f: &mut ratatui::Frame, area: Rect, state: &TuiState) {
    let body = state.focus_files_lines().join("\n");
    let panel = Paragraph::new(body)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("FOCUS: FILES (Up/Down PgUp/PgDn Esc back)"),
        )
        .scroll((state.focus_scroll_index() as u16, 0));
    f.render_widget(panel, area);
}

fn draw_focus_activity(f: &mut ratatui::Frame, area: Rect, state: &TuiState) {
    let body = state.focus_activity_lines().join("\n");
    let panel = Paragraph::new(body)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("FOCUS: ACTIVITY (Up/Down PgUp/PgDn Esc back)"),
        )
        .scroll((state.focus_scroll_index() as u16, 0));
    f.render_widget(panel, area);
}

fn draw_focus_status(f: &mut ratatui::Frame, area: Rect, state: &TuiState) {
    let body = state.focus_status_lines().join("\n");
    let panel = Paragraph::new(body)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("FOCUS: STATUS (Up/Down PgUp/PgDn Esc back)"),
        )
        .scroll((state.focus_scroll_index() as u16, 0));
    f.render_widget(panel, area);
}

fn draw_focus_session(f: &mut ratatui::Frame, area: Rect, state: &TuiState) {
    let body = state.focus_session_lines().join("\n");
    let panel = Paragraph::new(body)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("FOCUS: SESSION (Up/Down PgUp/PgDn Esc back)"),
        )
        .scroll((state.focus_scroll_index() as u16, 0));
    f.render_widget(panel, area);
}

fn draw_focus_contacts(f: &mut ratatui::Frame, area: Rect, state: &TuiState) {
    let body = state.focus_contacts_lines().join("\n");
    let panel = Paragraph::new(body)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("FOCUS: CONTACTS (Up/Down PgUp/PgDn Esc back)"),
        )
        .scroll((state.focus_scroll_index() as u16, 0));
    f.render_widget(panel, area);
}

fn draw_focus_settings(f: &mut ratatui::Frame, area: Rect, state: &TuiState) {
    let body = state.focus_settings_lines().join("\n");
    let panel = Paragraph::new(body)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("FOCUS: SETTINGS (Up/Down PgUp/PgDn Esc back)"),
        )
        .scroll((state.focus_scroll_index() as u16, 0));
    f.render_widget(panel, area);
}

fn draw_focus_lock(f: &mut ratatui::Frame, area: Rect, state: &TuiState) {
    let body = state.focus_lock_lines().join("\n");
    let panel = Paragraph::new(body)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("FOCUS: LOCK (Up/Down PgUp/PgDn Esc back)"),
        )
        .scroll((state.focus_scroll_index() as u16, 0));
    f.render_widget(panel, area);
}

fn render_unified_nav(f: &mut ratatui::Frame, area: Rect, state: &TuiState) {
    let rows = state.nav_rows();
    let selected_idx = state.nav_selected.min(rows.len().saturating_sub(1));
    let show_nav_marker = state.home_focus == TuiHomeFocus::Nav;
    let mut lines = Vec::new();
    for (idx, row) in rows.iter().enumerate() {
        let prefix = if show_nav_marker && idx == selected_idx {
            ">"
        } else {
            " "
        };
        match row.kind {
            NavRowKind::Domain(domain) => {
                let title = match domain {
                    TuiNavDomain::System => "System",
                    TuiNavDomain::Contacts => "Contacts",
                    TuiNavDomain::Messages => "Messages",
                };
                lines.push(format!("{} {}", prefix, title));
            }
            NavRowKind::SystemSettings => lines.push(format!("{}   Settings", prefix)),
            NavRowKind::SystemCmdResults => lines.push(format!("{}   Cmd Results", prefix)),
            NavRowKind::Header(pane) => {
                let header = match pane {
                    TuiInspectorPane::Events => format!("{} Messages", prefix),
                    TuiInspectorPane::Files => format!("{} Files", prefix),
                    TuiInspectorPane::Activity => format!("{} Activity", prefix),
                    TuiInspectorPane::Status => format!("{} Status", prefix),
                    TuiInspectorPane::CmdResults => format!("{} Cmd Results", prefix),
                    TuiInspectorPane::Session => format!("{} Keys", prefix),
                    TuiInspectorPane::Contacts => format!("{} Contacts", prefix),
                    TuiInspectorPane::Settings => format!("{} Settings", prefix),
                    TuiInspectorPane::Lock => format!("{} Lock", prefix),
                    TuiInspectorPane::Help => format!("{} Help", prefix),
                    TuiInspectorPane::About => format!("{} About", prefix),
                    TuiInspectorPane::Legal => format!("{} Legal", prefix),
                };
                lines.push(header);
            }
            NavRowKind::Conversation(item_idx) => {
                let labels = state.conversation_labels();
                if let Some(peer) = labels.get(item_idx) {
                    lines.push(format!("{}   {}", prefix, peer));
                }
            }
            NavRowKind::Contact(item_idx) => {
                if let Some(peer) = state.contacts.get(item_idx) {
                    lines.push(format!("{}   {}", prefix, peer));
                }
            }
            NavRowKind::Unlock => lines.push(format!("{} Unlock", prefix)),
            NavRowKind::Exit => lines.push(format!("{} Exit", prefix)),
        }
    }
    let selected_markers = if rows.is_empty() || !show_nav_marker {
        0
    } else {
        1
    };
    let selected_idx_s = selected_idx.to_string();
    let selected_label = rows
        .get(selected_idx)
        .map(|row| state.nav_row_label(row))
        .unwrap_or_else(|| "none".to_string());
    let header_text = "[ QSC ]";
    let inner_width = usize::from(area.width.saturating_sub(2));
    let header_left_padding = inner_width.saturating_sub(header_text.len()) / 2;
    let header_left_padding_s = header_left_padding.to_string();
    emit_marker(
        "tui_nav_render",
        None,
        &[
            (
                "selected_markers",
                if selected_markers == 1 { "1" } else { "0" },
            ),
            ("selected_index", selected_idx_s.as_str()),
            ("selected_label", selected_label.as_str()),
            ("header", header_text),
            ("header_left_padding", header_left_padding_s.as_str()),
            ("counters", "none"),
        ],
    );
    let panel = Paragraph::new(lines.join("\n")).block(
        Block::default()
            .borders(Borders::ALL)
            .title(Line::from(vec![Span::raw(header_text)]))
            .title_alignment(Alignment::Center),
    );
    f.render_widget(panel, area);
}

struct TuiStatus<'a> {
    fingerprint: &'a str,
    peer_fp: &'a str,
    qsp: &'a str,
    envelope: &'a str,
    send_lifecycle: &'a str,
    locked: &'a str,
}

struct TuiSession<'a> {
    peer_label: &'a str,
    verified: bool,
    sent_count: u64,
    recv_count: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct TuiFileItem {
    id: String,
    peer: String,
    filename: String,
    byte_len: usize,
    state: String,
    display_state: String,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum TuiMode {
    Normal,
    Help,
    FocusEvents,
    FocusFiles,
    FocusActivity,
    FocusStatus,
    FocusSession,
    FocusContacts,
    FocusSettings,
    FocusLock,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum TuiInspectorPane {
    Events,
    Files,
    Activity,
    Status,
    CmdResults,
    Session,
    Contacts,
    Settings,
    Lock,
    Help,
    About,
    Legal,
}

impl TuiInspectorPane {
    fn as_name(self) -> &'static str {
        match self {
            TuiInspectorPane::Events => "events",
            TuiInspectorPane::Files => "files",
            TuiInspectorPane::Activity => "activity",
            TuiInspectorPane::Status => "status",
            TuiInspectorPane::CmdResults => "cmd_results",
            TuiInspectorPane::Session => "session",
            TuiInspectorPane::Contacts => "contacts",
            TuiInspectorPane::Settings => "settings",
            TuiInspectorPane::Lock => "lock",
            TuiInspectorPane::Help => "help",
            TuiInspectorPane::About => "about",
            TuiInspectorPane::Legal => "legal",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum TuiHomeFocus {
    Nav,
    Main,
    Command,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum TuiPollMode {
    Adaptive,
    Fixed,
}

impl TuiPollMode {
    fn as_str(self) -> &'static str {
        match self {
            TuiPollMode::Adaptive => "adaptive",
            TuiPollMode::Fixed => "fixed",
        }
    }
}

#[derive(Clone)]
enum LockedFlow {
    None,
    UnlockPassphrase,
    InitAlias,
    InitPassphrase { alias: String },
    InitConfirm { alias: String, passphrase: String },
    InitAck { alias: String, passphrase: String },
}

#[derive(Clone, Copy)]
enum NavRowKind {
    Domain(TuiNavDomain),
    SystemSettings,
    SystemCmdResults,
    Header(TuiInspectorPane),
    Conversation(usize),
    Contact(usize),
    Unlock,
    Exit,
}

#[derive(Clone, Copy)]
struct NavRow {
    kind: NavRowKind,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum TuiNavDomain {
    System,
    Contacts,
    Messages,
}

const TUI_H3_WIDE_MIN: u16 = 120;
const TUI_H3_TALL_MIN: u16 = 28;
const TUI_INSPECTOR_CONTACTS_MAX: usize = 8;

fn tui_vault_present() -> bool {
    config_dir()
        .ok()
        .map(|(dir, _)| dir.join("vault.qsv").exists())
        .unwrap_or(false)
}

struct HomeLayoutSnapshot {
    contacts_shown: bool,
    header_compact: bool,
}

fn tui_relay_config(cfg: &TuiConfig) -> Option<TuiRelayConfig> {
    if cfg.transport.is_none() && cfg.relay.is_some() {
        print_error_marker("tui_transport_missing");
    }
    match cfg.transport {
        None => None,
        Some(TuiTransport::Relay) => {
            let relay = cfg
                .relay
                .clone()
                .unwrap_or_else(|| print_error_marker("tui_relay_missing"));
            Some(TuiRelayConfig {
                relay,
                seed: cfg.seed,
                scenario: cfg.scenario.clone(),
            })
        }
    }
}

struct TuiState {
    contacts: Vec<String>,
    conversations: BTreeMap<String, VecDeque<String>>,
    unread_counts: BTreeMap<String, usize>,
    visible_counts: BTreeMap<String, usize>,
    files: Vec<TuiFileItem>,
    file_selected: usize,
    file_multi_selected: BTreeSet<String>,
    file_unseen_updates: usize,
    activity_visible_count: usize,
    activity_unseen_updates: usize,
    events: VecDeque<String>,
    status: TuiStatus<'static>,
    session: TuiSession<'static>,
    send_lifecycle: String,
    qsp_status: String,
    envelope: String,
    last_payload_len: usize,
    event_seq: u64,
    relay: Option<TuiRelayConfig>,
    send_seq: u64,
    mode: TuiMode,
    help_selected: usize,
    focus_scroll: usize,
    contacts_selected: usize,
    conversation_selected: usize,
    inspector: TuiInspectorPane,
    home_focus: TuiHomeFocus,
    nav_selected: usize,
    vault_locked: bool,
    vault_present: bool,
    autolock_timeout_ms: u64,
    autolock_last_activity_ms: u64,
    poll_mode: TuiPollMode,
    poll_interval_seconds: u64,
    poll_next_due_ms: Option<u64>,
    headless_clock_ms: u64,
    clear_screen_pending: bool,
    force_full_redraw: bool,
    cmd_input: String,
    locked_flow: LockedFlow,
    locked_error: Option<String>,
    command_error: Option<String>,
    command_feedback: Option<String>,
    status_last_command_result: Option<String>,
    cmd_results: VecDeque<String>,
    active_command_label: Option<String>,
    active_command_result_recorded: bool,
}

impl TuiState {
    fn new(cfg: TuiConfig) -> Self {
        let vault_present = tui_vault_present();
        let mut contacts = contacts_list_labels();
        if contacts.is_empty() {
            contacts.push("peer-0".to_string());
        }
        let mut conversations = BTreeMap::new();
        conversations.insert("peer-0".to_string(), VecDeque::new());
        let unread_counts = BTreeMap::new();
        let visible_counts = BTreeMap::new();
        let mut events = VecDeque::new();
        let fingerprint = compute_local_fingerprint();
        let peer_fp = compute_peer_fingerprint("peer-0");
        let qsp_status = qsp_status_string("peer-0");
        let envelope = compute_envelope_status(0);
        let send_lifecycle = "idle".to_string();
        let locked_s = if vault_unlocked() {
            "UNLOCKED"
        } else {
            "LOCKED"
        };
        let status = TuiStatus {
            fingerprint: Box::leak(fingerprint.clone().into_boxed_str()),
            peer_fp: Box::leak(peer_fp.clone().into_boxed_str()),
            qsp: Box::leak(qsp_status.clone().into_boxed_str()),
            envelope: Box::leak(envelope.clone().into_boxed_str()),
            send_lifecycle: Box::leak(send_lifecycle.clone().into_boxed_str()),
            locked: Box::leak(locked_s.to_string().into_boxed_str()),
        };
        let session = TuiSession {
            peer_label: "peer-0",
            verified: false,
            sent_count: 0,
            recv_count: 0,
        };
        let relay = tui_relay_config(&cfg);
        if let Some(r) = relay.as_ref() {
            let seed_s = r.seed.to_string();
            emit_marker(
                "tui_transport",
                None,
                &[
                    ("transport", "relay"),
                    ("relay", r.relay.as_str()),
                    ("seed", seed_s.as_str()),
                    ("scenario", r.scenario.as_str()),
                ],
            );
            events.push_back(format!(
                "transport relay {} seed={} scenario={}",
                r.relay, r.seed, r.scenario
            ));
        }
        let (poll_mode, poll_interval_seconds) = load_tui_polling_config();
        let mut state = Self {
            contacts,
            conversations,
            unread_counts,
            visible_counts,
            files: load_tui_files_snapshot(),
            file_selected: 0,
            file_multi_selected: BTreeSet::new(),
            file_unseen_updates: 0,
            activity_visible_count: 0,
            activity_unseen_updates: 0,
            events,
            status,
            session,
            send_lifecycle,
            qsp_status,
            envelope,
            last_payload_len: 0,
            event_seq: 0,
            relay,
            send_seq: 0,
            mode: TuiMode::Normal,
            help_selected: 0,
            focus_scroll: 0,
            contacts_selected: 0,
            conversation_selected: 0,
            inspector: TuiInspectorPane::Status,
            home_focus: TuiHomeFocus::Nav,
            nav_selected: 0,
            vault_locked: !vault_unlocked(),
            vault_present,
            autolock_timeout_ms: load_tui_autolock_minutes().saturating_mul(60_000),
            autolock_last_activity_ms: 0,
            poll_mode,
            poll_interval_seconds,
            poll_next_due_ms: None,
            headless_clock_ms: 0,
            clear_screen_pending: false,
            force_full_redraw: false,
            cmd_input: String::new(),
            locked_flow: LockedFlow::None,
            locked_error: None,
            command_error: None,
            command_feedback: None,
            status_last_command_result: None,
            cmd_results: VecDeque::new(),
            active_command_label: None,
            active_command_result_recorded: false,
        };
        if env_bool("QSC_TUI_TEST_UNLOCK") {
            state.vault_locked = false;
            state.vault_present = true;
            state.status.locked = "UNLOCKED";
            set_vault_unlocked(true);
        }
        if state.vault_locked {
            state.inspector = TuiInspectorPane::Lock;
            state.nav_selected = 0;
        } else {
            state.sync_nav_to_inspector_header();
        }
        state
    }

    fn is_locked(&self) -> bool {
        self.vault_locked
    }

    fn has_vault(&self) -> bool {
        self.vault_present
    }

    fn mark_vault_present(&mut self) {
        self.vault_present = true;
    }

    fn cmd_input_clear(&mut self) {
        self.cmd_input.clear();
    }

    fn cmd_input_push(&mut self, ch: char) {
        self.cmd_input.push(ch);
    }

    fn cmd_input_pop(&mut self) {
        self.cmd_input.pop();
    }

    fn locked_flow_name(&self) -> &'static str {
        match self.locked_flow {
            LockedFlow::None => "none",
            LockedFlow::UnlockPassphrase => "unlock_passphrase",
            LockedFlow::InitAlias => "init_alias",
            LockedFlow::InitPassphrase { .. } => "init_passphrase",
            LockedFlow::InitConfirm { .. } => "init_confirm",
            LockedFlow::InitAck { .. } => "init_ack",
        }
    }

    fn locked_wizard_step_label(&self) -> Option<&'static str> {
        match self.locked_flow {
            LockedFlow::None => None,
            LockedFlow::UnlockPassphrase => Some("Passphrase"),
            LockedFlow::InitAlias => Some("Alias"),
            LockedFlow::InitPassphrase { .. } => Some("Passphrase"),
            LockedFlow::InitConfirm { .. } => Some("Confirm"),
            LockedFlow::InitAck { .. } => Some("Ack"),
        }
    }

    fn locked_set_error(&mut self, message: impl Into<String>) {
        self.locked_error = Some(message.into());
    }

    fn locked_clear_error(&mut self) {
        self.locked_error = None;
    }

    fn set_command_error(&mut self, message: impl Into<String>) {
        let message = message.into();
        self.command_error = Some(message.clone());
        self.command_feedback = None;
        if !self.active_command_result_recorded {
            if let Some(command) = self.active_command_label.clone() {
                self.push_cmd_result(command.as_str(), false, message);
            }
        }
        if !self.is_locked() {
            self.route_show_to_system_nav(TuiInspectorPane::CmdResults);
        }
    }

    fn clear_command_error(&mut self) {
        self.command_error = None;
    }

    fn set_command_feedback(&mut self, message: impl Into<String>) {
        let message = message.into();
        let marker_msg = message.replace(' ', "_");
        emit_marker(
            "tui_cmd_feedback",
            None,
            &[("kind", "ok"), ("message", marker_msg.as_str())],
        );
        self.command_feedback = Some(message);
    }

    fn clear_command_feedback(&mut self) {
        self.command_feedback = None;
    }

    fn set_status_last_command_result(&mut self, message: impl Into<String>) {
        self.status_last_command_result = Some(message.into());
    }

    fn status_last_command_result_text(&self) -> &str {
        self.status_last_command_result.as_deref().unwrap_or("none")
    }

    fn push_cmd_result(&mut self, command: &str, ok: bool, message: impl Into<String>) {
        let status = if ok { "ok" } else { "err" };
        let message = message.into();
        let line = format!("[{}] /{} {}", status, command, message);
        let cmd_marker = command.replace(' ', "_");
        emit_marker(
            "tui_cmd_result",
            None,
            &[("kind", status), ("command", cmd_marker.as_str())],
        );
        self.cmd_results.push_back(line);
        self.active_command_result_recorded = true;
        while self.cmd_results.len() > 50 {
            self.cmd_results.pop_front();
        }
    }

    fn begin_command_tracking(&mut self, command: impl Into<String>) {
        self.active_command_label = Some(command.into());
        self.active_command_result_recorded = false;
    }

    fn end_command_tracking(&mut self) {
        self.active_command_label = None;
        self.active_command_result_recorded = false;
    }

    fn locked_cmd_masked(&self) -> bool {
        matches!(
            self.locked_flow,
            LockedFlow::UnlockPassphrase
                | LockedFlow::InitPassphrase { .. }
                | LockedFlow::InitConfirm { .. }
        )
    }

    fn cmd_display_value(&self) -> String {
        if self.locked_cmd_masked() {
            "•".repeat(self.cmd_input.chars().count())
        } else {
            self.cmd_input.clone()
        }
    }

    fn cmd_bar_text(&self) -> String {
        if self.is_locked() {
            if let Some(label) = self.locked_wizard_step_label() {
                if self.home_focus == TuiHomeFocus::Command {
                    format!("{}: {}{}", label, self.cmd_display_value(), '█')
                } else {
                    format!("{}:", label)
                }
            } else if self.home_focus == TuiHomeFocus::Command {
                format!("Cmd: {}{}", self.cmd_display_value(), '█')
            } else {
                "Cmd:".to_string()
            }
        } else if self.cmd_input.is_empty() {
            if let Some(msg) = self.command_feedback.as_ref() {
                msg.clone()
            } else if self.home_focus == TuiHomeFocus::Command {
                format!("Cmd: {}{}", self.cmd_display_value(), '█')
            } else {
                "Cmd: /help".to_string()
            }
        } else if self.home_focus == TuiHomeFocus::Command {
            format!("Cmd: {}{}", self.cmd_display_value(), '█')
        } else {
            "Cmd: /help".to_string()
        }
    }

    fn accent_color_enabled(&self) -> bool {
        tui_color_enabled()
    }

    fn cmd_bar_style(&self, text: &str) -> Style {
        if !self.accent_color_enabled() {
            return Style::default();
        }
        if text.starts_with("ok:") {
            Style::default().fg(Color::Green)
        } else if text.starts_with("error:") {
            Style::default().fg(Color::Red)
        } else {
            Style::default()
        }
    }

    fn locked_main_lines(&self) -> Vec<String> {
        match &self.locked_flow {
            LockedFlow::None => {
                if self.has_vault() {
                    vec!["Locked: unlock required".to_string()]
                } else {
                    vec!["No vault found - run /init".to_string()]
                }
            }
            LockedFlow::UnlockPassphrase => {
                let mut lines = vec![
                    "Unlock".to_string(),
                    String::new(),
                    format!("Passphrase: {}", self.cmd_display_value()),
                ];
                if let Some(err) = self.locked_error.as_ref() {
                    lines.push(format!("error: {}", err));
                }
                lines
            }
            LockedFlow::InitAlias
            | LockedFlow::InitPassphrase { .. }
            | LockedFlow::InitConfirm { .. }
            | LockedFlow::InitAck { .. } => {
                let mut lines = vec![
                    "Initialize Vault".to_string(),
                    String::new(),
                    "This will create an encrypted vault to store your identity, contacts, messages, and files.".to_string(),
                    "Choose a strong passphrase — there is no recovery if it’s lost.".to_string(),
                    String::new(),
                ];
                let (step_header, input_label) = match self.locked_flow {
                    LockedFlow::InitAlias => ("Step 1/4 - Alias (required)", "Alias"),
                    LockedFlow::InitPassphrase { .. } => {
                        ("Step 2/4 - Create passphrase (required)", "Passphrase")
                    }
                    LockedFlow::InitConfirm { .. } => ("Step 3/4 - Confirm passphrase", "Confirm"),
                    LockedFlow::InitAck { .. } => ("Step 4/4 - Final acknowledgement", "Ack"),
                    _ => ("Step", "Input"),
                };
                lines.push(step_header.to_string());
                lines.push(format!("{}: {}", input_label, self.cmd_display_value()));
                if let Some(err) = self.locked_error.as_ref() {
                    lines.push(format!("error: {}", err));
                }
                lines.push(match self.locked_flow {
                    LockedFlow::InitAck { .. } => {
                        "Keys: Enter=submit (type exact: I UNDERSTAND)  Esc=cancel".to_string()
                    }
                    _ => "Keys: Enter=continue  Backspace=delete/back  Esc=cancel".to_string(),
                });
                lines
            }
        }
    }

    fn locked_main_body(&self) -> String {
        self.locked_main_lines().join("\n")
    }

    fn start_unlock_prompt(&mut self) {
        self.home_focus = TuiHomeFocus::Command;
        self.locked_flow = LockedFlow::UnlockPassphrase;
        self.cmd_input_clear();
        self.locked_clear_error();
        emit_marker("tui_unlock_prompt", None, &[("step", "passphrase")]);
        emit_marker("tui_focus_home", None, &[("pane", self.home_focus_name())]);
    }

    fn start_init_prompt(&mut self) {
        self.home_focus = TuiHomeFocus::Command;
        self.locked_flow = LockedFlow::InitAlias;
        self.cmd_input_clear();
        self.locked_clear_error();
        emit_marker(
            "tui_init_warning",
            None,
            &[("no_recovery", "true"), ("ack_required", "I UNDERSTAND")],
        );
        emit_marker("tui_init_wizard", None, &[("step", "alias")]);
        emit_marker("tui_focus_home", None, &[("pane", self.home_focus_name())]);
    }

    fn locked_focus_toggle(&mut self) {
        self.home_focus = if self.home_focus == TuiHomeFocus::Command {
            TuiHomeFocus::Nav
        } else {
            TuiHomeFocus::Command
        };
        emit_marker("tui_focus_home", None, &[("pane", self.home_focus_name())]);
    }

    fn autolock_minutes(&self) -> u64 {
        let minutes = self.autolock_timeout_ms / 60_000;
        minutes.clamp(TUI_AUTOLOCK_MIN_MINUTES, TUI_AUTOLOCK_MAX_MINUTES)
    }

    fn set_autolock_minutes(&mut self, minutes: u64) -> Result<(), &'static str> {
        if !(TUI_AUTOLOCK_MIN_MINUTES..=TUI_AUTOLOCK_MAX_MINUTES).contains(&minutes) {
            return Err("autolock_invalid_minutes");
        }
        persist_tui_autolock_minutes(minutes)?;
        self.autolock_timeout_ms = minutes.saturating_mul(60_000);
        let minutes_s = minutes.to_string();
        emit_marker(
            "tui_autolock_set",
            None,
            &[("ok", "true"), ("minutes", minutes_s.as_str())],
        );
        Ok(())
    }

    fn poll_mode(&self) -> TuiPollMode {
        self.poll_mode
    }

    fn poll_interval_seconds(&self) -> u64 {
        self.poll_interval_seconds
            .clamp(TUI_POLL_MIN_INTERVAL_SECONDS, TUI_POLL_MAX_INTERVAL_SECONDS)
    }

    fn poll_interval_ms(&self) -> u64 {
        self.poll_interval_seconds().saturating_mul(1_000)
    }

    fn set_poll_mode_adaptive(&mut self) -> Result<(), &'static str> {
        persist_tui_polling_config(TuiPollMode::Adaptive, self.poll_interval_seconds())?;
        self.poll_mode = TuiPollMode::Adaptive;
        self.poll_next_due_ms = None;
        let interval_s = self.poll_interval_seconds().to_string();
        emit_marker(
            "tui_poll_set",
            None,
            &[
                ("ok", "true"),
                ("mode", self.poll_mode.as_str()),
                ("interval_seconds", interval_s.as_str()),
            ],
        );
        Ok(())
    }

    fn set_poll_mode_fixed(&mut self, seconds: u64, now_ms: u64) -> Result<(), &'static str> {
        if !(TUI_POLL_MIN_INTERVAL_SECONDS..=TUI_POLL_MAX_INTERVAL_SECONDS).contains(&seconds) {
            return Err("poll_invalid_seconds");
        }
        persist_tui_polling_config(TuiPollMode::Fixed, seconds)?;
        self.poll_mode = TuiPollMode::Fixed;
        self.poll_interval_seconds = seconds;
        self.poll_next_due_ms = Some(now_ms.saturating_add(self.poll_interval_ms()));
        let interval_s = self.poll_interval_seconds().to_string();
        emit_marker(
            "tui_poll_set",
            None,
            &[
                ("ok", "true"),
                ("mode", self.poll_mode.as_str()),
                ("interval_seconds", interval_s.as_str()),
            ],
        );
        Ok(())
    }

    fn emit_poll_show_marker(&self) {
        let interval_s = self.poll_interval_seconds().to_string();
        emit_marker(
            "tui_poll_show",
            None,
            &[
                ("ok", "true"),
                ("mode", self.poll_mode.as_str()),
                ("interval_seconds", interval_s.as_str()),
            ],
        );
    }

    fn maybe_run_fixed_poll(&mut self, now_ms: u64) {
        if self.is_locked() || self.poll_mode != TuiPollMode::Fixed {
            return;
        }
        if self.relay.is_none() {
            return;
        }
        let interval_ms = self.poll_interval_ms();
        if interval_ms == 0 {
            return;
        }
        let mut due = self
            .poll_next_due_ms
            .unwrap_or_else(|| now_ms.saturating_add(interval_ms));
        if due > now_ms {
            self.poll_next_due_ms = Some(due);
            return;
        }
        while due <= now_ms {
            let due_s = due.to_string();
            let now_s = now_ms.to_string();
            let interval_s = self.poll_interval_seconds().to_string();
            let peer = self.selected_conversation_label();
            emit_marker(
                "tui_poll_tick",
                None,
                &[
                    ("mode", self.poll_mode.as_str()),
                    ("peer", peer.as_str()),
                    ("interval_seconds", interval_s.as_str()),
                    ("due_ms", due_s.as_str()),
                    ("now_ms", now_s.as_str()),
                ],
            );
            tui_receive_via_relay(self, peer.as_str());
            due = due.saturating_add(interval_ms);
        }
        self.poll_next_due_ms = Some(due);
    }

    fn mark_input_activity(&mut self, now_ms: u64) {
        self.autolock_last_activity_ms = now_ms;
    }

    fn headless_now_ms(&self) -> u64 {
        self.headless_clock_ms
    }

    fn current_now_ms(&self) -> u64 {
        self.headless_clock_ms.max(self.autolock_last_activity_ms)
    }

    fn headless_advance_clock(&mut self, delta_ms: u64) {
        self.headless_clock_ms = self.headless_clock_ms.saturating_add(delta_ms);
        self.maybe_autolock(self.headless_clock_ms);
        self.maybe_run_fixed_poll(self.headless_clock_ms);
    }

    fn maybe_autolock(&mut self, now_ms: u64) {
        if self.is_locked() || self.autolock_timeout_ms == 0 {
            return;
        }
        if now_ms.saturating_sub(self.autolock_last_activity_ms) < self.autolock_timeout_ms {
            return;
        }
        self.set_locked_state(true, "inactivity_timeout");
        let minutes_s = self.autolock_minutes().to_string();
        emit_marker(
            "tui_autolock",
            None,
            &[("ok", "true"), ("minutes", minutes_s.as_str())],
        );
    }

    fn take_clear_screen_pending(&mut self) -> bool {
        let pending = self.clear_screen_pending;
        self.clear_screen_pending = false;
        pending
    }

    fn take_force_full_redraw(&mut self) -> bool {
        let pending = self.force_full_redraw;
        self.force_full_redraw = false;
        pending
    }

    fn clear_ui_buffers_on_lock(&mut self, reason: &'static str) {
        self.mode = TuiMode::Normal;
        self.help_selected = 0;
        self.focus_scroll = 0;
        self.home_focus = TuiHomeFocus::Nav;
        self.locked_flow = LockedFlow::None;
        self.locked_clear_error();
        self.clear_command_error();
        self.cmd_input_clear();
        self.inspector = TuiInspectorPane::Lock;
        self.nav_selected = 0;
        self.sync_messages_if_main_focused();
        self.sync_files_if_main_focused();
        self.sync_activity_if_main_focused();
        self.clear_screen_pending = true;
        self.force_full_redraw = true;
        emit_marker(
            "tui_buffer_clear",
            None,
            &[("ok", "true"), ("reason", reason)],
        );
    }

    fn set_locked_state(&mut self, locked: bool, reason: &'static str) {
        let was_locked = self.vault_locked;
        self.vault_locked = locked;
        self.status.locked = if locked { "LOCKED" } else { "UNLOCKED" };
        set_vault_unlocked(!locked);
        if locked && !was_locked {
            self.clear_ui_buffers_on_lock(reason);
        } else if locked {
            self.home_focus = TuiHomeFocus::Nav;
            self.inspector = TuiInspectorPane::Lock;
            self.nav_selected = 0;
        } else {
            self.locked_flow = LockedFlow::None;
            self.clear_command_error();
            self.cmd_input_clear();
            self.home_focus = TuiHomeFocus::Nav;
            self.inspector = TuiInspectorPane::Status;
            self.sync_nav_to_inspector_header();
        }
        emit_marker(
            "tui_lock_state",
            None,
            &[
                ("locked", self.status.locked),
                ("reason", reason),
                (
                    "vault",
                    if self.vault_present {
                        "present"
                    } else {
                        "missing"
                    },
                ),
                ("ok", "true"),
            ],
        );
    }

    fn last_payload_len(&self) -> usize {
        self.last_payload_len
    }

    fn ensure_conversation(&mut self, peer: &str) {
        self.conversations.entry(peer.to_string()).or_default();
        self.visible_counts.entry(peer.to_string()).or_insert(0);
        self.unread_counts.entry(peer.to_string()).or_insert(0);
    }

    fn conversation_labels(&self) -> Vec<String> {
        let mut labels = self
            .contacts
            .iter()
            .cloned()
            .chain(self.conversations.keys().cloned())
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();
        if labels.is_empty() {
            labels.push("peer-0".to_string());
        }
        labels
    }

    fn selected_conversation_label(&self) -> String {
        let labels = self.conversation_labels();
        labels
            .get(
                self.conversation_selected
                    .min(labels.len().saturating_sub(1)),
            )
            .cloned()
            .unwrap_or_else(|| "peer-0".to_string())
    }

    fn selected_contact_label(&self) -> String {
        if self.contacts.is_empty() {
            "peer-0".to_string()
        } else {
            self.contacts[self
                .contacts_selected
                .min(self.contacts.len().saturating_sub(1))]
            .clone()
        }
    }

    fn selected_file_id(&self) -> Option<&str> {
        self.files
            .get(self.file_selected.min(self.files.len().saturating_sub(1)))
            .map(|v| v.id.as_str())
    }

    fn refresh_file_selection_bounds(&mut self) {
        if self.file_selected >= self.files.len() {
            self.file_selected = self.files.len().saturating_sub(1);
        }
        if self.files.is_empty() {
            self.file_selected = 0;
            self.file_multi_selected.clear();
            return;
        }
        self.file_multi_selected
            .retain(|id| self.files.iter().any(|f| &f.id == id));
    }

    fn upsert_file_item(&mut self, item: TuiFileItem, from_update: bool) {
        let mut changed = false;
        if let Some(existing) = self.files.iter_mut().find(|v| v.id == item.id) {
            if existing != &item {
                *existing = item;
                changed = true;
            }
        } else {
            self.files.push(item);
            changed = true;
        }
        self.files.sort_by(|a, b| a.id.cmp(&b.id));
        self.refresh_file_selection_bounds();
        if from_update
            && changed
            && !(self.mode == TuiMode::Normal
                && self.inspector == TuiInspectorPane::Files
                && self.home_focus == TuiHomeFocus::Main)
        {
            self.file_unseen_updates = self.file_unseen_updates.saturating_add(1);
        }
    }

    fn refresh_files_from_timeline(&mut self) {
        for item in load_tui_files_snapshot() {
            self.upsert_file_item(item, true);
        }
    }

    fn files_select_by_id(&mut self, id: &str) -> bool {
        if let Some(idx) = self.files.iter().position(|v| v.id == id) {
            self.file_selected = idx;
            true
        } else {
            false
        }
    }

    fn files_toggle_selected(&mut self) -> bool {
        let Some(id) = self.selected_file_id().map(str::to_string) else {
            return false;
        };
        if self.file_multi_selected.contains(id.as_str()) {
            self.file_multi_selected.remove(id.as_str());
        } else {
            self.file_multi_selected.insert(id);
        }
        true
    }

    fn files_move(&mut self, delta: i32) {
        if self.files.is_empty() {
            self.file_selected = 0;
            return;
        }
        let max = (self.files.len() - 1) as i32;
        let mut idx = self.file_selected as i32 + delta;
        if idx < 0 {
            idx = 0;
        }
        if idx > max {
            idx = max;
        }
        self.file_selected = idx as usize;
    }

    fn set_active_peer(&mut self, peer: &str) {
        self.session.peer_label = Box::leak(peer.to_string().into_boxed_str());
        self.refresh_qsp_status();
    }

    fn sync_messages_if_main_focused(&mut self) {
        if self.mode != TuiMode::Normal
            || self.inspector != TuiInspectorPane::Events
            || self.home_focus != TuiHomeFocus::Main
        {
            return;
        }
        let peer = self.selected_conversation_label();
        let total = self
            .conversations
            .get(peer.as_str())
            .map(|v| v.len())
            .unwrap_or(0usize);
        self.visible_counts.insert(peer.clone(), total);
        self.unread_counts.insert(peer, 0);
    }

    fn sync_files_if_main_focused(&mut self) {
        if self.mode != TuiMode::Normal
            || self.inspector != TuiInspectorPane::Files
            || self.home_focus != TuiHomeFocus::Main
        {
            return;
        }
        self.file_unseen_updates = 0;
    }

    fn sync_activity_if_main_focused(&mut self) {
        if self.mode != TuiMode::Normal
            || self.inspector != TuiInspectorPane::Activity
            || self.home_focus != TuiHomeFocus::Main
        {
            return;
        }
        self.activity_visible_count = self.events.len();
        self.activity_unseen_updates = 0;
    }

    fn record_message_line(&mut self, peer: &str, state: &str, direction: &str, detail: &str) {
        self.ensure_conversation(peer);
        let line = format!("state={} dir={} {}", state, direction, detail);
        {
            let stream = self.conversations.entry(peer.to_string()).or_default();
            stream.push_back(line);
            if stream.len() > 128 {
                stream.pop_front();
            }
        }
        let selected = self.selected_conversation_label();
        let auto_append = self.mode == TuiMode::Normal
            && self.inspector == TuiInspectorPane::Events
            && self.home_focus == TuiHomeFocus::Main
            && selected == peer;
        let total = self
            .conversations
            .get(peer)
            .map(|v| v.len())
            .unwrap_or(0usize);
        if auto_append {
            self.visible_counts.insert(peer.to_string(), total);
            self.unread_counts.insert(peer.to_string(), 0);
        } else {
            let unread = self
                .unread_counts
                .get(peer)
                .copied()
                .unwrap_or(0usize)
                .saturating_add(1);
            self.unread_counts.insert(peer.to_string(), unread);
            self.visible_counts
                .entry(peer.to_string())
                .or_insert(total.saturating_sub(1));
        }
        let total_s = total.to_string();
        let unread_s = self
            .unread_counts
            .get(peer)
            .copied()
            .unwrap_or(0)
            .to_string();
        emit_marker(
            "tui_message_event",
            None,
            &[
                ("peer", peer),
                ("state", state),
                ("mode", if auto_append { "append" } else { "buffer" }),
                ("total", total_s.as_str()),
                ("unread", unread_s.as_str()),
            ],
        );
    }

    fn update_send_lifecycle(&mut self, value: &str) {
        self.send_lifecycle = value.to_string();
        self.status.send_lifecycle = Box::leak(self.send_lifecycle.clone().into_boxed_str());
        emit_marker(
            "tui_status_update",
            None,
            &[("field", "send_lifecycle"), ("value", value)],
        );
    }

    fn refresh_envelope(&mut self, payload_len: usize) {
        self.last_payload_len = payload_len;
        self.envelope = compute_envelope_status(payload_len);
        self.status.envelope = Box::leak(self.envelope.clone().into_boxed_str());
        emit_marker(
            "tui_status_update",
            None,
            &[("field", "envelope"), ("value", &self.envelope)],
        );
    }

    fn refresh_qsp_status(&mut self) {
        let peer = self.session.peer_label;
        self.qsp_status = qsp_status_string(peer);
        self.status.qsp = Box::leak(self.qsp_status.clone().into_boxed_str());
        let peer_fp = compute_peer_fingerprint(peer);
        self.status.peer_fp = Box::leak(peer_fp.into_boxed_str());
        self.refresh_contacts();
        emit_marker(
            "tui_status_update",
            None,
            &[("field", "qsp"), ("value", &self.qsp_status)],
        );
    }

    fn refresh_contacts(&mut self) {
        let mut labels = contacts_list_labels();
        if labels.is_empty() {
            labels.push("peer-0".to_string());
        }
        self.contacts = labels;
        if self.contacts_selected >= self.contacts.len() {
            self.contacts_selected = self.contacts.len().saturating_sub(1);
        }
        for peer in self.conversation_labels() {
            self.ensure_conversation(peer.as_str());
        }
        let labels = self.conversation_labels();
        if self.conversation_selected >= labels.len() {
            self.conversation_selected = labels.len().saturating_sub(1);
        }
    }

    fn push_event(&mut self, kind: &str, action: &str) {
        self.event_seq = self.event_seq.wrapping_add(1);
        let seq_s = self.event_seq.to_string();
        emit_marker(
            "tui_event",
            None,
            &[("kind", kind), ("action", action), ("seq", seq_s.as_str())],
        );
        let line = format!("{}:{} #{}", kind, action, self.event_seq);
        self.events.push_back(line);
        if self.events.len() > 64 {
            self.events.pop_front();
        }
        self.record_activity_update();
    }

    fn push_event_line(&mut self, line: String) {
        self.events.push_back(line);
        if self.events.len() > 64 {
            self.events.pop_front();
        }
        self.record_activity_update();
    }

    fn record_activity_update(&mut self) {
        let total = self.events.len();
        let auto_append = self.mode == TuiMode::Normal
            && self.inspector == TuiInspectorPane::Activity
            && self.home_focus == TuiHomeFocus::Main;
        if auto_append {
            self.activity_visible_count = total;
            self.activity_unseen_updates = 0;
            return;
        }
        self.activity_unseen_updates = self.activity_unseen_updates.saturating_add(1);
        if self.activity_visible_count == 0 && total > 0 {
            self.activity_visible_count = total.saturating_sub(1);
        } else if self.activity_visible_count > total {
            self.activity_visible_count = total;
        }
    }

    fn enter_help_mode(&mut self) {
        if matches!(
            self.mode,
            TuiMode::FocusEvents
                | TuiMode::FocusFiles
                | TuiMode::FocusActivity
                | TuiMode::FocusStatus
                | TuiMode::FocusSession
                | TuiMode::FocusContacts
                | TuiMode::FocusSettings
                | TuiMode::FocusLock
        ) {
            let pane = TuiState::focus_pane_name(self.mode);
            emit_marker("tui_focus", None, &[("pane", pane), ("on", "false")]);
        }
        self.mode = TuiMode::Help;
        self.help_selected = 0;
        emit_marker("tui_help_mode", None, &[("on", "true")]);
        let items = tui_help_items();
        for item in items {
            emit_marker("tui_help_item", None, &[("cmd", item.cmd)]);
        }
        let count_s = items.len().to_string();
        emit_marker("tui_help_rendered", None, &[("count", count_s.as_str())]);
    }

    fn exit_help_mode(&mut self) {
        if self.mode == TuiMode::Help {
            self.mode = TuiMode::Normal;
            emit_marker("tui_help_mode", None, &[("on", "false")]);
        }
    }

    fn toggle_help_mode(&mut self) {
        if self.mode == TuiMode::Help {
            self.exit_help_mode();
        } else {
            self.enter_help_mode();
        }
    }

    fn focus_pane_name(mode: TuiMode) -> &'static str {
        match mode {
            TuiMode::FocusEvents => "events",
            TuiMode::FocusFiles => "files",
            TuiMode::FocusActivity => "activity",
            TuiMode::FocusStatus => "status",
            TuiMode::FocusSession => "session",
            TuiMode::FocusContacts => "contacts",
            TuiMode::FocusSettings => "settings",
            TuiMode::FocusLock => "lock",
            _ => "dashboard",
        }
    }

    fn inspector_name(&self) -> &'static str {
        match self.inspector {
            TuiInspectorPane::Events => "events",
            TuiInspectorPane::Files => "files",
            TuiInspectorPane::Activity => "activity",
            TuiInspectorPane::Status => "status",
            TuiInspectorPane::CmdResults => "cmd_results",
            TuiInspectorPane::Session => "session",
            TuiInspectorPane::Contacts => "contacts",
            TuiInspectorPane::Settings => "settings",
            TuiInspectorPane::Lock => "lock",
            TuiInspectorPane::Help => "help",
            TuiInspectorPane::About => "about",
            TuiInspectorPane::Legal => "legal",
        }
    }

    fn set_inspector(&mut self, pane: TuiInspectorPane) {
        self.inspector = pane;
        self.sync_nav_to_inspector_header();
        self.sync_messages_if_main_focused();
        self.sync_files_if_main_focused();
        self.sync_activity_if_main_focused();
        emit_marker("tui_inspector", None, &[("pane", self.inspector_name())]);
    }

    fn route_show_to_system_nav(&mut self, pane: TuiInspectorPane) {
        self.set_inspector(pane);
        self.home_focus = TuiHomeFocus::Nav;
        self.cmd_input_clear();
        emit_marker("tui_focus_home", None, &[("pane", self.home_focus_name())]);
    }

    fn focus_mode_for_inspector(&self) -> TuiMode {
        match self.inspector {
            TuiInspectorPane::Events => TuiMode::FocusEvents,
            TuiInspectorPane::Files => TuiMode::FocusFiles,
            TuiInspectorPane::Activity => TuiMode::FocusActivity,
            TuiInspectorPane::Status => TuiMode::FocusStatus,
            TuiInspectorPane::CmdResults => TuiMode::FocusStatus,
            TuiInspectorPane::Session => TuiMode::FocusSession,
            TuiInspectorPane::Contacts => TuiMode::FocusContacts,
            TuiInspectorPane::Settings => TuiMode::FocusSettings,
            TuiInspectorPane::Lock => TuiMode::FocusLock,
            TuiInspectorPane::Help => TuiMode::FocusSettings,
            TuiInspectorPane::About => TuiMode::FocusSettings,
            TuiInspectorPane::Legal => TuiMode::FocusSettings,
        }
    }

    fn home_layout_snapshot(&self, cols: u16, rows: u16) -> HomeLayoutSnapshot {
        HomeLayoutSnapshot {
            contacts_shown: cols >= TUI_H3_WIDE_MIN,
            header_compact: rows < TUI_H3_TALL_MIN,
        }
    }

    fn home_focus_name(&self) -> &'static str {
        match self.home_focus {
            TuiHomeFocus::Nav => "nav",
            TuiHomeFocus::Main => "main",
            TuiHomeFocus::Command => "command",
        }
    }

    fn home_focus_cycle(&mut self, delta: i32) {
        let idx = match self.home_focus {
            TuiHomeFocus::Nav => 0i32,
            TuiHomeFocus::Main => 1i32,
            TuiHomeFocus::Command => 2i32,
        };
        let next = (idx + delta).rem_euclid(3);
        self.home_focus = match next {
            0 => TuiHomeFocus::Nav,
            1 => TuiHomeFocus::Main,
            _ => TuiHomeFocus::Command,
        };
        self.sync_messages_if_main_focused();
        self.sync_files_if_main_focused();
        self.sync_activity_if_main_focused();
        emit_marker("tui_focus_home", None, &[("pane", self.home_focus_name())]);
    }

    fn emit_home_render_marker(&self, cols: u16, rows: u16) {
        if self.mode != TuiMode::Normal {
            return;
        }
        if self.is_locked() {
            let cmdbar_text = self.cmd_bar_text();
            let main_lines = self.locked_main_lines();
            let main_step = self.locked_flow_name();
            let main_intro_a = main_lines
                .iter()
                .find(|line| line.contains("This will create an encrypted vault"))
                .map(|v| v.as_str())
                .unwrap_or("none");
            let main_intro_b = main_lines
                .iter()
                .find(|line| line.contains("Choose a strong passphrase"))
                .map(|v| v.as_str())
                .unwrap_or("none");
            let main_input_line = main_lines
                .iter()
                .find(|line| {
                    line.starts_with("Alias:")
                        || line.starts_with("Passphrase:")
                        || line.starts_with("Confirm:")
                        || line.starts_with("Ack:")
                })
                .map(|v| v.as_str())
                .unwrap_or("none");
            let main_error_line = main_lines
                .iter()
                .find(|line| line.starts_with("error:"))
                .map(|v| v.as_str())
                .unwrap_or("none");
            let main_hints_line = main_lines
                .iter()
                .find(|line| line.starts_with("Keys:") || line.starts_with("Submit:"))
                .map(|v| v.as_str())
                .unwrap_or("none");
            emit_marker(
                "tui_locked_shell",
                None,
                &[
                    (
                        "vault",
                        if self.has_vault() {
                            "present"
                        } else {
                            "missing"
                        },
                    ),
                    ("nav", "unlock,exit"),
                    (
                        "main",
                        if self.has_vault() {
                            "locked"
                        } else {
                            "init_required"
                        },
                    ),
                    ("cmd", if self.has_vault() { "/unlock" } else { "/init" }),
                    ("cmdbar_text", cmdbar_text.as_str()),
                    ("wizard", self.locked_flow_name()),
                    ("nav_title", "qsc"),
                    ("main_title", "none"),
                    ("cmd_panel_title", "none"),
                    ("focus", self.home_focus_name()),
                    ("main_step", main_step),
                    ("main_intro_a", main_intro_a),
                    ("main_intro_b", main_intro_b),
                    ("main_input", main_input_line),
                    ("main_error", main_error_line),
                    ("main_hints", main_hints_line),
                ],
            );
            let nav_rows = self.nav_rows();
            let nav_selected = self.nav_selected.min(nav_rows.len().saturating_sub(1));
            let nav_selected_s = nav_selected.to_string();
            let selected_label = nav_rows
                .get(nav_selected)
                .map(|row| self.nav_row_label(row))
                .unwrap_or_else(|| "none".to_string());
            let selected_markers = if self.home_focus == TuiHomeFocus::Nav && !nav_rows.is_empty() {
                "1"
            } else {
                "0"
            };
            emit_marker(
                "tui_nav_render",
                None,
                &[
                    ("selected_markers", selected_markers),
                    ("selected_index", nav_selected_s.as_str()),
                    ("selected_label", selected_label.as_str()),
                    ("header", "[ QSC ]"),
                    ("header_left_padding", "1"),
                    ("counters", "none"),
                ],
            );
            return;
        }
        let layout = self.home_layout_snapshot(cols, rows);
        let cmdbar_text = self.cmd_bar_text();
        let cmdbar_marker = cmdbar_text.replace(' ', "_");
        emit_marker(
            "tui_render",
            None,
            &[
                ("mode", "home"),
                ("layout", "h3"),
                ("inspector", self.inspector_name()),
                (
                    "contacts",
                    if layout.contacts_shown {
                        "shown"
                    } else {
                        "hidden"
                    },
                ),
                (
                    "header",
                    if layout.header_compact {
                        "compact"
                    } else {
                        "full"
                    },
                ),
                ("focus", self.home_focus_name()),
                ("nav", "shown"),
                ("expanded", self.inspector_name()),
                ("cmdbar", "full"),
                (
                    "cmd_hint",
                    if self.home_focus == TuiHomeFocus::Command {
                        "input"
                    } else {
                        "help"
                    },
                ),
                ("nav_title", "qsc"),
                ("main_title", "none"),
                ("cmd_panel_title", "none"),
                ("cmdbar_text", cmdbar_marker.as_str()),
            ],
        );
        let nav_rows = self.nav_rows();
        let nav_selected = self.nav_selected.min(nav_rows.len().saturating_sub(1));
        let nav_selected_s = nav_selected.to_string();
        let selected_label = nav_rows
            .get(nav_selected)
            .map(|row| self.nav_row_label(row))
            .unwrap_or_else(|| "none".to_string());
        emit_marker(
            "tui_nav_render",
            None,
            &[
                (
                    "selected_markers",
                    if nav_rows.is_empty() { "0" } else { "1" },
                ),
                ("selected_index", nav_selected_s.as_str()),
                ("selected_label", selected_label.as_str()),
                ("header", "[ QSC ]"),
                ("header_left_padding", "1"),
                ("counters", "none"),
            ],
        );
        if self.inspector == TuiInspectorPane::Events {
            let peer = self.selected_conversation_label();
            let total = self
                .conversations
                .get(peer.as_str())
                .map(|v| v.len())
                .unwrap_or(0usize);
            let visible = self
                .visible_counts
                .get(peer.as_str())
                .copied()
                .unwrap_or(total);
            let unread = self
                .unread_counts
                .get(peer.as_str())
                .copied()
                .unwrap_or(0usize);
            let total_s = total.to_string();
            let visible_s = visible.to_string();
            let unread_s = unread.to_string();
            emit_marker(
                "tui_messages_view",
                None,
                &[
                    ("peer", peer.as_str()),
                    ("total", total_s.as_str()),
                    ("visible", visible_s.as_str()),
                    ("unread", unread_s.as_str()),
                    ("preview", "none"),
                    ("redacted", if self.is_locked() { "true" } else { "false" }),
                ],
            );
        }
        if self.inspector == TuiInspectorPane::Contacts {
            let selected = self.selected_contact_label();
            let selected_line = contact_display_line(selected.as_str());
            emit_marker(
                "tui_contacts_view",
                None,
                &[
                    ("selected", selected.as_str()),
                    ("summary", selected_line.as_str()),
                    ("sections", "verification,pinning,commands"),
                    ("preview", "none"),
                    ("redacted", if self.is_locked() { "true" } else { "false" }),
                ],
            );
        }
        if self.inspector == TuiInspectorPane::Activity {
            let total_s = self.events.len().to_string();
            let visible_s = self
                .activity_visible_count
                .min(self.events.len())
                .to_string();
            let unread_s = self.activity_unseen_updates.to_string();
            emit_marker(
                "tui_activity_view",
                None,
                &[
                    ("total", total_s.as_str()),
                    ("visible", visible_s.as_str()),
                    ("unread", unread_s.as_str()),
                    ("sections", "ledger,commands"),
                    ("redacted", if self.is_locked() { "true" } else { "false" }),
                ],
            );
        }
        if self.inspector == TuiInspectorPane::Files {
            let selected = self.selected_file_id().unwrap_or("none");
            let selected_state = self
                .files
                .get(self.file_selected.min(self.files.len().saturating_sub(1)))
                .map(|v| v.display_state.as_str())
                .unwrap_or("none");
            emit_marker(
                "tui_files_view",
                None,
                &[
                    ("total", self.files.len().to_string().as_str()),
                    ("selected", selected),
                    (
                        "selected_count",
                        self.file_multi_selected.len().to_string().as_str(),
                    ),
                    ("updates", self.file_unseen_updates.to_string().as_str()),
                    ("state", selected_state),
                    ("preview", "none"),
                    ("redacted", if self.is_locked() { "true" } else { "false" }),
                ],
            );
        }
        if self.inspector == TuiInspectorPane::Session {
            let selected = self.session.peer_label;
            emit_marker(
                "tui_keys_view",
                None,
                &[
                    ("selected", selected),
                    ("sections", "metadata,verification,commands"),
                    ("multi_select", "false"),
                    ("preview", "none"),
                    ("redacted", if self.is_locked() { "true" } else { "false" }),
                ],
            );
        }
        if self.inspector == TuiInspectorPane::Settings {
            let minutes_s = self.autolock_minutes().to_string();
            let poll_interval_s = self.poll_interval_seconds().to_string();
            emit_marker(
                "tui_settings_view",
                None,
                &[
                    ("read_only", "true"),
                    ("inline_actions", "false"),
                    ("lock_state", self.status.locked),
                    ("autolock_minutes", minutes_s.as_str()),
                    ("poll_mode", self.poll_mode().as_str()),
                    ("poll_interval_seconds", poll_interval_s.as_str()),
                    ("sections", "system_settings,lock,autolock,polling,commands"),
                ],
            );
        }
        if self.inspector == TuiInspectorPane::CmdResults {
            let count_s = self.cmd_results.len().to_string();
            emit_marker(
                "tui_cmd_results_view",
                None,
                &[("count", count_s.as_str()), ("sections", "history")],
            );
        }
        if self.inspector == TuiInspectorPane::Lock {
            let effect = if self.status.locked == "UNLOCKED" {
                "sensitive_content_displayed"
            } else {
                "sensitive_content_redacted"
            };
            let minutes_s = self.autolock_minutes().to_string();
            emit_marker(
                "tui_lock_view",
                None,
                &[
                    ("locked", self.status.locked),
                    ("redacted", if self.is_locked() { "true" } else { "false" }),
                    ("sections", "state,effect,autolock,commands"),
                    ("title", "Lock Status"),
                    ("state", self.status.locked),
                    ("effect", effect),
                    ("autolock_minutes", minutes_s.as_str()),
                    ("commands", "lock,autolock_show,autolock_set"),
                ],
            );
        }
        if self.inspector == TuiInspectorPane::Status {
            let redacted = if self.status.locked == "LOCKED" {
                "true"
            } else {
                "false"
            };
            let minutes_s = self.autolock_minutes().to_string();
            let poll_interval_s = self.poll_interval_seconds().to_string();
            emit_marker(
                "tui_status_view",
                None,
                &[
                    ("locked", self.status.locked),
                    ("redacted", redacted),
                    ("autolock_minutes", minutes_s.as_str()),
                    ("poll_mode", self.poll_mode().as_str()),
                    ("poll_interval_seconds", poll_interval_s.as_str()),
                    ("last_result", self.status_last_command_result_text()),
                    ("sections", "system_overview,snapshot,transport,queue"),
                ],
            );
        }
    }

    fn focus_render_count(&self, mode: TuiMode) -> usize {
        match mode {
            TuiMode::FocusEvents => self.focus_events_lines().len(),
            TuiMode::FocusFiles => self.focus_files_lines().len(),
            TuiMode::FocusActivity => self.focus_activity_lines().len(),
            TuiMode::FocusContacts => self.contacts.len(),
            TuiMode::FocusStatus => self.focus_status_lines().len(),
            TuiMode::FocusSession => self.focus_session_lines().len(),
            TuiMode::FocusSettings => self.focus_settings_lines().len(),
            TuiMode::FocusLock => self.focus_lock_lines().len(),
            _ => 0,
        }
    }

    fn enter_focus_mode(&mut self, mode: TuiMode) {
        if self.mode == mode {
            return;
        }
        if matches!(
            self.mode,
            TuiMode::FocusEvents
                | TuiMode::FocusFiles
                | TuiMode::FocusActivity
                | TuiMode::FocusStatus
                | TuiMode::FocusSession
                | TuiMode::FocusContacts
                | TuiMode::FocusSettings
                | TuiMode::FocusLock
        ) {
            let pane = TuiState::focus_pane_name(self.mode);
            emit_marker("tui_focus", None, &[("pane", pane), ("on", "false")]);
        }
        self.mode = mode;
        self.focus_scroll = 0;
        self.contacts_selected = 0;
        let pane = TuiState::focus_pane_name(self.mode);
        emit_marker("tui_focus", None, &[("pane", pane), ("on", "true")]);
        let count = self.focus_render_count(self.mode);
        emit_marker(
            "tui_focus_rendered",
            None,
            &[("pane", pane), ("count", count.to_string().as_str())],
        );
        self.emit_focus_render_marker();
    }

    fn exit_focus_mode(&mut self) {
        if matches!(
            self.mode,
            TuiMode::FocusEvents
                | TuiMode::FocusFiles
                | TuiMode::FocusActivity
                | TuiMode::FocusStatus
                | TuiMode::FocusSession
                | TuiMode::FocusContacts
                | TuiMode::FocusSettings
                | TuiMode::FocusLock
        ) {
            let pane = TuiState::focus_pane_name(self.mode);
            emit_marker("tui_focus", None, &[("pane", pane), ("on", "false")]);
            self.mode = TuiMode::Normal;
        }
    }

    fn is_help_mode(&self) -> bool {
        self.mode == TuiMode::Help
    }

    fn is_focus_mode(&self) -> bool {
        matches!(
            self.mode,
            TuiMode::FocusEvents
                | TuiMode::FocusFiles
                | TuiMode::FocusActivity
                | TuiMode::FocusStatus
                | TuiMode::FocusSession
                | TuiMode::FocusContacts
                | TuiMode::FocusSettings
                | TuiMode::FocusLock
        )
    }

    fn focus_max_len(&self) -> usize {
        match self.mode {
            TuiMode::FocusEvents => self.focus_events_lines().len(),
            TuiMode::FocusFiles => self.focus_files_lines().len(),
            TuiMode::FocusActivity => self.focus_activity_lines().len(),
            TuiMode::FocusContacts => self.contacts.len(),
            TuiMode::FocusStatus => self.focus_status_lines().len(),
            TuiMode::FocusSession => self.focus_session_lines().len(),
            TuiMode::FocusSettings => self.focus_settings_lines().len(),
            TuiMode::FocusLock => self.focus_lock_lines().len(),
            _ => 0,
        }
    }

    fn focus_view_rows(&self) -> usize {
        usize::from(terminal_rows_for_headless().saturating_sub(2)).max(1)
    }

    fn focus_scroll_index(&self) -> usize {
        match self.mode {
            TuiMode::FocusContacts => self.contacts_selected,
            TuiMode::FocusFiles => self.file_selected,
            _ => self.focus_scroll,
        }
    }

    fn focus_events_lines(&self) -> Vec<String> {
        self.events
            .iter()
            .enumerate()
            .map(|(i, line)| format!("{} {}", tui_timestamp_token(i), line))
            .collect()
    }

    fn focus_activity_lines(&self) -> Vec<String> {
        self.focus_events_lines()
    }

    fn focus_status_lines(&self) -> Vec<String> {
        let locked = self.status.locked == "LOCKED";
        let poll_interval_s = self.poll_interval_seconds().to_string();
        [
            format!("vault_locked: {}", self.status.locked),
            format!(
                "fingerprint: {}",
                if locked {
                    "hidden (unlock required)"
                } else {
                    self.status.fingerprint
                }
            ),
            format!(
                "peer_fp: {}",
                if locked {
                    "hidden (unlock required)"
                } else {
                    self.status.peer_fp
                }
            ),
            format!("qsp: {}", self.status.qsp),
            format!("envelope: {}", self.status.envelope),
            format!("send: {}", self.status.send_lifecycle),
            format!("poll_mode: {}", self.poll_mode().as_str()),
            format!("poll_interval_seconds: {}", poll_interval_s),
        ]
        .into_iter()
        .enumerate()
        .map(|(i, line)| format!("{} {}", tui_timestamp_token(i), line))
        .collect()
    }

    fn focus_session_lines(&self) -> Vec<String> {
        let locked = self.is_locked();
        [
            "domain: keys".to_string(),
            format!("selected_peer: {}", self.session.peer_label),
            format!(
                "verified: {}",
                if locked {
                    "hidden (unlock required)".to_string()
                } else {
                    self.session.verified.to_string()
                }
            ),
            if locked {
                "identity_metadata: hidden (unlock required)".to_string()
            } else {
                "identity_metadata: visible".to_string()
            },
            "dangerous_ops: command_bar_only".to_string(),
        ]
        .into_iter()
        .enumerate()
        .map(|(i, line)| format!("{} {}", tui_timestamp_token(i), line))
        .collect()
    }

    fn focus_contacts_lines(&self) -> Vec<String> {
        self.contacts
            .iter()
            .enumerate()
            .map(|(i, peer)| format!("{} {}", tui_timestamp_token(i), contact_display_line(peer)))
            .collect()
    }

    fn focus_files_lines(&self) -> Vec<String> {
        let locked = self.is_locked();
        self.files
            .iter()
            .enumerate()
            .map(|(i, item)| {
                let selected = if self.file_multi_selected.contains(item.id.as_str()) {
                    "[x]"
                } else {
                    "[ ]"
                };
                format!(
                    "{} {} id={} peer={} size={} state={} name={}",
                    tui_timestamp_token(i),
                    selected,
                    item.id,
                    item.peer,
                    item.byte_len,
                    item.display_state,
                    if locked {
                        "hidden (unlock required)"
                    } else {
                        item.filename.as_str()
                    }
                )
            })
            .collect()
    }

    fn focus_settings_lines(&self) -> Vec<String> {
        let poll_interval = if self.poll_mode() == TuiPollMode::Fixed {
            self.poll_interval_seconds().to_string()
        } else {
            "n/a".to_string()
        };
        [
            "settings".to_string(),
            String::new(),
            format!("lock_state: {}", self.status.locked),
            format!("autolock_timeout_minutes: {}", self.autolock_minutes()),
            format!("poll_mode: {}", self.poll_mode().as_str()),
            format!("poll_interval_seconds: {}", poll_interval),
            "commands: /status /autolock show /autolock set <minutes> /poll show /poll set adaptive /poll set fixed <seconds>".to_string(),
        ]
        .into_iter()
        .enumerate()
        .map(|(i, line)| format!("{} {}", tui_timestamp_token(i), line))
        .collect()
    }

    fn focus_lock_lines(&self) -> Vec<String> {
        [
            "domain: lock".to_string(),
            format!("state: {}", self.status.locked),
            "redaction: sensitive_content_hidden_when_locked".to_string(),
            "commands: /lock /unlock".to_string(),
        ]
        .into_iter()
        .enumerate()
        .map(|(i, line)| format!("{} {}", tui_timestamp_token(i), line))
        .collect()
    }

    fn emit_focus_render_marker(&self) {
        if !self.is_focus_mode() {
            return;
        }
        let pane = TuiState::focus_pane_name(self.mode);
        let count = self.focus_max_len();
        let scroll = self.focus_scroll_index();
        let view_rows = self.focus_view_rows();
        let ts_start_idx = if count == 0 { 0 } else { scroll.min(count - 1) };
        let ts_end_idx = if count == 0 {
            0
        } else {
            count
                .min(scroll.saturating_add(view_rows))
                .saturating_sub(1)
        };
        let scroll_s = scroll.to_string();
        let count_s = count.to_string();
        let view_rows_s = view_rows.to_string();
        let ts_start = tui_timestamp_token(ts_start_idx);
        let ts_end = tui_timestamp_token(ts_end_idx);
        emit_marker(
            "tui_render",
            None,
            &[
                ("mode", "focus"),
                ("layout", "h3"),
                ("focus", pane),
                ("viewport", "full"),
                ("scroll", scroll_s.as_str()),
                ("count", count_s.as_str()),
                ("view_rows", view_rows_s.as_str()),
                ("ts_start", ts_start.as_str()),
                ("ts_end", ts_end.as_str()),
                (
                    "deterministic",
                    if tui_deterministic_timestamps() {
                        "true"
                    } else {
                        "false"
                    },
                ),
            ],
        );
    }

    fn help_selected_item(&self) -> Option<&'static TuiHelpItem> {
        let items = tui_help_items();
        if items.is_empty() {
            None
        } else {
            Some(&items[self.help_selected.min(items.len() - 1)])
        }
    }

    fn help_move(&mut self, delta: i32) {
        let items = tui_help_items();
        if items.is_empty() {
            self.help_selected = 0;
            return;
        }
        let len = items.len() as i32;
        let mut idx = self.help_selected as i32 + delta;
        if idx < 0 {
            idx = 0;
        }
        if idx >= len {
            idx = len - 1;
        }
        self.help_selected = idx as usize;
    }

    fn focus_scroll_move(&mut self, delta: i32, max_len: usize) {
        if max_len == 0 {
            self.focus_scroll = 0;
            self.emit_focus_render_marker();
            return;
        }
        let max = (max_len.saturating_sub(1)) as i32;
        let mut idx = self.focus_scroll as i32 + delta;
        if idx < 0 {
            idx = 0;
        }
        if idx > max {
            idx = max;
        }
        self.focus_scroll = idx as usize;
        self.emit_focus_render_marker();
    }

    fn contacts_move(&mut self, delta: i32) {
        if self.contacts.is_empty() {
            self.contacts_selected = 0;
            self.emit_focus_render_marker();
            return;
        }
        let max = (self.contacts.len() - 1) as i32;
        let mut idx = self.contacts_selected as i32 + delta;
        if idx < 0 {
            idx = 0;
        }
        if idx > max {
            idx = max;
        }
        self.contacts_selected = idx as usize;
        self.emit_focus_render_marker();
    }

    fn nav_move(&mut self, delta: i32) {
        if self.home_focus != TuiHomeFocus::Nav {
            return;
        }
        let rows = self.nav_rows();
        if rows.is_empty() {
            self.nav_selected = 0;
            return;
        }
        let max = (rows.len() - 1) as i32;
        let mut idx = self.nav_selected as i32 + delta;
        if idx < 0 {
            idx = 0;
        }
        if idx > max {
            idx = max;
        }
        self.nav_selected = idx as usize;
        self.nav_preview_select(rows[self.nav_selected].kind);
    }

    fn nav_activate(&mut self) {
        if self.home_focus != TuiHomeFocus::Nav {
            return;
        }
        let rows = self.nav_rows();
        if rows.is_empty() {
            return;
        }
        let row = rows[self.nav_selected.min(rows.len().saturating_sub(1))];
        match row.kind {
            NavRowKind::Domain(_) => self.nav_preview_select(row.kind),
            NavRowKind::SystemSettings => self.set_inspector(TuiInspectorPane::Settings),
            NavRowKind::SystemCmdResults => self.set_inspector(TuiInspectorPane::CmdResults),
            NavRowKind::Header(pane) => self.set_inspector(pane),
            NavRowKind::Conversation(idx) => {
                self.set_inspector(TuiInspectorPane::Events);
                self.nav_preview_select(NavRowKind::Conversation(idx));
            }
            NavRowKind::Contact(idx) => {
                self.set_inspector(TuiInspectorPane::Contacts);
                self.nav_preview_select(NavRowKind::Contact(idx));
            }
            NavRowKind::Unlock => {
                emit_marker(
                    "tui_nav_select",
                    None,
                    &[("domain", "locked"), ("label", "unlock")],
                );
            }
            NavRowKind::Exit => {
                emit_marker(
                    "tui_nav_select",
                    None,
                    &[("domain", "locked"), ("label", "exit")],
                );
            }
        }
        emit_marker("tui_nav_activate", None, &[("pane", self.inspector_name())]);
    }

    fn locked_nav_activate(&mut self) -> bool {
        if !self.is_locked() || self.home_focus != TuiHomeFocus::Nav {
            return false;
        }
        self.nav_activate();
        let rows = self.nav_rows();
        if rows.is_empty() {
            return false;
        }
        let row = rows[self.nav_selected.min(rows.len().saturating_sub(1))];
        match row.kind {
            NavRowKind::Exit => {
                emit_marker("tui_cmd", None, &[("cmd", "exit")]);
                true
            }
            NavRowKind::Unlock => {
                if self.has_vault() {
                    self.start_unlock_prompt();
                } else {
                    self.start_init_prompt();
                }
                false
            }
            _ => false,
        }
    }

    fn nav_preview_select(&mut self, kind: NavRowKind) {
        match kind {
            NavRowKind::Domain(domain) => {
                match domain {
                    TuiNavDomain::System => self.set_inspector(TuiInspectorPane::Status),
                    TuiNavDomain::Contacts => self.set_inspector(TuiInspectorPane::Contacts),
                    TuiNavDomain::Messages => self.set_inspector(TuiInspectorPane::Events),
                }
                emit_marker(
                    "tui_nav_select",
                    None,
                    &[(
                        "domain",
                        match domain {
                            TuiNavDomain::System => "system",
                            TuiNavDomain::Contacts => "contacts",
                            TuiNavDomain::Messages => "messages",
                        },
                    )],
                );
            }
            NavRowKind::SystemSettings => {
                self.set_inspector(TuiInspectorPane::Settings);
                emit_marker(
                    "tui_nav_select",
                    None,
                    &[("domain", "system"), ("label", "settings")],
                );
            }
            NavRowKind::SystemCmdResults => {
                self.set_inspector(TuiInspectorPane::CmdResults);
                emit_marker(
                    "tui_nav_select",
                    None,
                    &[("domain", "system"), ("label", "cmd_results")],
                );
            }
            NavRowKind::Header(pane) => {
                self.inspector = pane;
                self.sync_nav_to_inspector_header();
                self.sync_messages_if_main_focused();
                self.sync_files_if_main_focused();
                self.sync_activity_if_main_focused();
                emit_marker("tui_inspector", None, &[("pane", self.inspector_name())]);
                emit_marker(
                    "tui_nav_select",
                    None,
                    &[
                        ("domain", Self::pane_domain_name(pane)),
                        ("label", pane.as_name()),
                    ],
                );
            }
            NavRowKind::Conversation(idx) => {
                let labels = self.conversation_labels();
                if labels.is_empty() {
                    self.conversation_selected = 0;
                    return;
                }
                self.conversation_selected = idx.min(labels.len().saturating_sub(1));
                let selected = self.selected_conversation_label();
                self.set_active_peer(selected.as_str());
                self.sync_messages_if_main_focused();
                emit_marker(
                    "tui_nav_select",
                    None,
                    &[("domain", "messages"), ("label", selected.as_str())],
                );
            }
            NavRowKind::Contact(idx) => {
                if self.contacts.is_empty() {
                    self.contacts_selected = 0;
                    return;
                }
                self.contacts_selected = idx.min(self.contacts.len().saturating_sub(1));
                let selected = self.selected_contact_label();
                emit_marker(
                    "tui_nav_select",
                    None,
                    &[("domain", "contacts"), ("label", selected.as_str())],
                );
            }
            NavRowKind::Unlock => {
                emit_marker(
                    "tui_nav_select",
                    None,
                    &[("domain", "locked"), ("label", "unlock")],
                );
            }
            NavRowKind::Exit => {
                emit_marker(
                    "tui_nav_select",
                    None,
                    &[("domain", "locked"), ("label", "exit")],
                );
            }
        }
    }

    fn pane_domain_name(pane: TuiInspectorPane) -> &'static str {
        match pane {
            TuiInspectorPane::Events => "messages",
            TuiInspectorPane::Files => "files",
            TuiInspectorPane::Activity => "activity",
            TuiInspectorPane::Status => "system",
            TuiInspectorPane::CmdResults => "system",
            TuiInspectorPane::Session => "keys",
            TuiInspectorPane::Contacts => "contacts",
            TuiInspectorPane::Settings => "system",
            TuiInspectorPane::Lock => "lock",
            TuiInspectorPane::Help => "help",
            TuiInspectorPane::About => "about",
            TuiInspectorPane::Legal => "legal",
        }
    }

    fn nav_row_label(&self, row: &NavRow) -> String {
        match row.kind {
            NavRowKind::Domain(TuiNavDomain::System) => "system".to_string(),
            NavRowKind::Domain(TuiNavDomain::Contacts) => "contacts".to_string(),
            NavRowKind::Domain(TuiNavDomain::Messages) => "messages".to_string(),
            NavRowKind::SystemSettings => "settings".to_string(),
            NavRowKind::SystemCmdResults => "cmd_results".to_string(),
            NavRowKind::Header(_) => "header".to_string(),
            NavRowKind::Conversation(item_idx) => self
                .conversation_labels()
                .get(item_idx)
                .cloned()
                .unwrap_or_else(|| "none".to_string()),
            NavRowKind::Contact(item_idx) => self
                .contacts
                .get(item_idx)
                .cloned()
                .unwrap_or_else(|| "none".to_string()),
            NavRowKind::Unlock => "unlock".to_string(),
            NavRowKind::Exit => "exit".to_string(),
        }
    }

    fn expanded_nav_domain(&self) -> Option<TuiNavDomain> {
        match self.inspector {
            TuiInspectorPane::Status
            | TuiInspectorPane::Settings
            | TuiInspectorPane::CmdResults => Some(TuiNavDomain::System),
            TuiInspectorPane::Contacts => Some(TuiNavDomain::Contacts),
            TuiInspectorPane::Events => Some(TuiNavDomain::Messages),
            _ => None,
        }
    }

    fn nav_rows(&self) -> Vec<NavRow> {
        if self.is_locked() {
            return vec![
                NavRow {
                    kind: NavRowKind::Unlock,
                },
                NavRow {
                    kind: NavRowKind::Exit,
                },
            ];
        }
        let expanded = self.expanded_nav_domain();
        let mut rows = Vec::new();
        rows.push(NavRow {
            kind: NavRowKind::Domain(TuiNavDomain::System),
        });
        if expanded == Some(TuiNavDomain::System) {
            rows.push(NavRow {
                kind: NavRowKind::SystemSettings,
            });
            rows.push(NavRow {
                kind: NavRowKind::SystemCmdResults,
            });
        }
        rows.push(NavRow {
            kind: NavRowKind::Domain(TuiNavDomain::Contacts),
        });
        if expanded == Some(TuiNavDomain::Contacts) {
            for idx in 0..self.contacts.len().min(4) {
                rows.push(NavRow {
                    kind: NavRowKind::Contact(idx),
                });
            }
        }
        rows.push(NavRow {
            kind: NavRowKind::Domain(TuiNavDomain::Messages),
        });
        if expanded == Some(TuiNavDomain::Messages) {
            for idx in 0..self.conversation_labels().len().min(6) {
                rows.push(NavRow {
                    kind: NavRowKind::Conversation(idx),
                });
            }
        }
        for pane in [
            TuiInspectorPane::Activity,
            TuiInspectorPane::Session,
            TuiInspectorPane::Help,
            TuiInspectorPane::About,
            TuiInspectorPane::Legal,
        ] {
            rows.push(NavRow {
                kind: NavRowKind::Header(pane),
            });
        }
        rows
    }

    fn sync_nav_to_inspector_header(&mut self) {
        if self.is_locked() {
            self.nav_selected = 0;
            return;
        }
        let rows = self.nav_rows();
        self.nav_selected = match self.inspector {
            TuiInspectorPane::Status => rows
                .iter()
                .position(|row| matches!(row.kind, NavRowKind::Domain(TuiNavDomain::System)))
                .unwrap_or(0),
            TuiInspectorPane::Settings => rows
                .iter()
                .position(|row| matches!(row.kind, NavRowKind::SystemSettings))
                .unwrap_or(0),
            TuiInspectorPane::CmdResults => rows
                .iter()
                .position(|row| matches!(row.kind, NavRowKind::SystemCmdResults))
                .unwrap_or(0),
            TuiInspectorPane::Contacts => rows
                .iter()
                .position(|row| matches!(row.kind, NavRowKind::Domain(TuiNavDomain::Contacts)))
                .unwrap_or(0),
            TuiInspectorPane::Events => rows
                .iter()
                .position(|row| matches!(row.kind, NavRowKind::Domain(TuiNavDomain::Messages)))
                .unwrap_or(0),
            pane => rows
                .iter()
                .position(|row| matches!(row.kind, NavRowKind::Header(p) if p == pane))
                .unwrap_or(0),
        };
    }

    fn drain_marker_queue(&mut self) {
        self.refresh_files_from_timeline();
        let mut queue = marker_queue().lock().expect("marker queue lock");
        while let Some(line) = queue.pop_front() {
            if line.contains("QSC_MARK/1") || line.contains("event=tui_nav_render") {
                continue;
            }
            self.push_event_line(line);
        }
    }
}

struct TuiHelpItem {
    cmd: &'static str,
    desc: &'static str,
}

fn tui_help_items() -> &'static [TuiHelpItem] {
    &[
        TuiHelpItem {
            cmd: "help",
            desc: "show commands",
        },
        TuiHelpItem {
            cmd:
                "inspector status|settings|cmdresults|events|session|contacts|lock|help|about|legal",
            desc: "set home inspector pane",
        },
        TuiHelpItem {
            cmd: "focus events",
            desc: "focus Events pane",
        },
        TuiHelpItem {
            cmd: "focus files",
            desc: "focus Files pane",
        },
        TuiHelpItem {
            cmd: "focus activity",
            desc: "focus Activity pane",
        },
        TuiHelpItem {
            cmd: "focus status",
            desc: "focus Status pane",
        },
        TuiHelpItem {
            cmd: "focus session",
            desc: "focus Session pane",
        },
        TuiHelpItem {
            cmd: "focus contacts",
            desc: "focus Contacts pane",
        },
        TuiHelpItem {
            cmd: "focus settings",
            desc: "focus Settings pane",
        },
        TuiHelpItem {
            cmd: "focus lock",
            desc: "focus Lock pane",
        },
        TuiHelpItem {
            cmd: "contacts list|block <label>|unblock <label>|add <label> <fp>",
            desc: "manage contact states",
        },
        TuiHelpItem {
            cmd: "messages list|select <peer>",
            desc: "manage conversation selection",
        },
        TuiHelpItem {
            cmd: "files list|select <id>|toggle <id?>|clear-selection|inject <id> <state>",
            desc: "manage files view and multi-select in Files domain only",
        },
        TuiHelpItem {
            cmd: "injectmsg <peer> [STATE]",
            desc: "headless-only deterministic message injection",
        },
        TuiHelpItem {
            cmd: "injectevent <kind> <action>",
            desc: "headless-only deterministic activity event injection",
        },
        TuiHelpItem {
            cmd: "back",
            desc: "exit focus mode",
        },
        TuiHelpItem {
            cmd: "exit",
            desc: "exit TUI",
        },
        TuiHelpItem {
            cmd: "exithelp",
            desc: "exit help mode",
        },
        TuiHelpItem {
            cmd: "send",
            desc: "send via explicit transport",
        },
        TuiHelpItem {
            cmd: "handshake status",
            desc: "show handshake status",
        },
        TuiHelpItem {
            cmd: "handshake init",
            desc: "initiate handshake to peer",
        },
        TuiHelpItem {
            cmd: "handshake poll",
            desc: "poll inbox for handshake",
        },
        TuiHelpItem {
            cmd: "status",
            desc: "refresh status",
        },
        TuiHelpItem {
            cmd: "autolock show|set <minutes>",
            desc: "view or set inactivity lock timeout (minutes)",
        },
        TuiHelpItem {
            cmd: "poll show|set adaptive|set fixed <seconds>",
            desc: "view or set optional fixed poll cadence",
        },
        TuiHelpItem {
            cmd: "lock",
            desc: "explicitly lock and redact sensitive content",
        },
        TuiHelpItem {
            cmd: "unlock",
            desc: "explicitly unlock using configured vault auth",
        },
        TuiHelpItem {
            cmd: "envelope",
            desc: "refresh envelope",
        },
        TuiHelpItem {
            cmd: "export",
            desc: "export redacted diagnostics",
        },
    ]
}

fn render_main_panel(f: &mut ratatui::Frame, area: Rect, state: &TuiState) {
    if state.is_locked() {
        let body = state.locked_main_body();
        let panel = Paragraph::new(body).block(Block::default().borders(Borders::ALL));
        f.render_widget(panel, area);
        return;
    }
    let body = match state.inspector {
        TuiInspectorPane::Events => {
            let peer = state.selected_conversation_label();
            let stream = state.conversations.get(peer.as_str());
            let total = stream.map(|v| v.len()).unwrap_or(0usize);
            let visible = state
                .visible_counts
                .get(peer.as_str())
                .copied()
                .unwrap_or(total)
                .min(total);
            if total == 0 {
                format!(
                    "Messages Overview\n\nNo messages yet for {peer}.\nUse command bar: /send (explicit intent)."
                )
            } else {
                let mut lines = Vec::new();
                lines.push("Messages Overview".to_string());
                lines.push(String::new());
                lines.push(format!("conversation: {}", peer));
                if state.is_locked() {
                    lines.push("redaction: active (unlock required)".to_string());
                }
                lines.push(String::new());
                if let Some(entries) = stream {
                    for line in entries.iter().take(visible) {
                        if state.is_locked() {
                            let mut tokens = line.split_whitespace();
                            let state_tok = tokens.next().unwrap_or("state=unknown");
                            let dir_tok = tokens.next().unwrap_or("dir=unknown");
                            lines.push(format!(
                                "{} {} hidden (unlock required)",
                                state_tok, dir_tok
                            ));
                        } else {
                            lines.push(line.clone());
                        }
                    }
                }
                if visible < total {
                    lines.push(String::new());
                    lines.push(format!(
                        "(buffered: {} unread; focus Main on Messages to append)",
                        total - visible
                    ));
                }
                lines.join("\n")
            }
        }
        TuiInspectorPane::Files => {
            if state.files.is_empty() {
                "Files\n\nNo file transfers yet.\nUse command bar only for actions.".to_string()
            } else {
                let selected = state
                    .files
                    .get(state.file_selected.min(state.files.len().saturating_sub(1)));
                let mut lines = Vec::new();
                lines.push(format!(
                    "files: {} ({} selected)",
                    state.files.len(),
                    state.file_multi_selected.len()
                ));
                lines.push(String::new());
                if let Some(item) = selected {
                    lines.push(format!("id: {}", item.id));
                    lines.push(format!(
                        "peer: {}",
                        if state.is_locked() {
                            "hidden (unlock required)"
                        } else {
                            item.peer.as_str()
                        }
                    ));
                    lines.push(format!(
                        "name: {}",
                        if state.is_locked() {
                            "hidden (unlock required)"
                        } else {
                            item.filename.as_str()
                        }
                    ));
                    lines.push(format!("size: {} bytes", item.byte_len));
                    lines.push(format!("state: {}", item.display_state));
                    lines.push("at_rest: encrypted(vault timeline)".to_string());
                } else {
                    lines.push("selected: none".to_string());
                }
                if state.file_unseen_updates > 0 && state.home_focus != TuiHomeFocus::Main {
                    lines.push(String::new());
                    lines.push(format!(
                        "(buffered updates: {}; focus Main on Files to clear)",
                        state.file_unseen_updates
                    ));
                }
                lines.push(String::new());
                lines.push("Commands (command bar only)".to_string());
                lines.push("- /files list".to_string());
                lines.push("- /files select <id>".to_string());
                lines.push("- /files toggle <id?>".to_string());
                lines.push("- /files clear-selection".to_string());
                lines
                    .push("- /files inject <id> <state> [size] [name] (headless test)".to_string());
                lines.join("\n")
            }
        }
        TuiInspectorPane::Activity => {
            let total = state.events.len();
            let visible = state.activity_visible_count.min(total);
            let mut lines = Vec::new();
            lines.push("Activity".to_string());
            lines.push(String::new());
            lines.push(format!(
                "ledger: {} (visible={} unread={})",
                total, visible, state.activity_unseen_updates
            ));
            lines.push(String::new());
            for line in state.events.iter().take(visible) {
                lines.push(line.clone());
            }
            if visible < total {
                lines.push(String::new());
                lines.push(format!(
                    "(buffered: {} events; focus Main on Activity to append)",
                    total - visible
                ));
            }
            lines.push(String::new());
            lines.push("Commands (command bar only)".to_string());
            lines.push("- /focus activity".to_string());
            lines.join("\n")
        }
        TuiInspectorPane::Status => {
            let locked = state.status.locked == "LOCKED";
            let own_fp = if locked {
                "hidden (unlock required)"
            } else {
                state.status.fingerprint
            };
            let peer_fp = if locked {
                "hidden (unlock required)"
            } else {
                state.status.peer_fp
            };
            let poll_interval_s = state.poll_interval_seconds().to_string();
            let last_result = state.status_last_command_result_text();
            format!(
                "System Overview\n\nlocked: {}\nautolock_minutes: {}\npoll_mode: {}\npoll_interval_seconds: {}\nlast_command_result: {}\nqsp: {}\nown_fp: {}\npeer_fp: {}\nsend: {}\ncounts: sent={} recv={}",
                state.status.locked,
                state.autolock_minutes(),
                state.poll_mode().as_str(),
                poll_interval_s,
                last_result,
                state.status.qsp,
                own_fp,
                peer_fp,
                state.status.send_lifecycle,
                state.session.sent_count,
                state.session.recv_count
            )
        }
        TuiInspectorPane::CmdResults => {
            let mut lines = Vec::new();
            lines.push("Command Results".to_string());
            lines.push(String::new());
            if state.cmd_results.is_empty() {
                lines.push("No command results yet.".to_string());
            } else {
                for entry in state.cmd_results.iter().rev().take(50) {
                    lines.push(entry.clone());
                }
            }
            lines.join("\n")
        }
        TuiInspectorPane::Session => {
            let replay_rejects = state
                .events
                .iter()
                .filter(|line| line.contains("ratchet_replay_reject"))
                .count();
            let mut lines = Vec::new();
            lines.push("Keys".to_string());
            lines.push(String::new());
            lines.push(format!("selected_peer: {}", state.session.peer_label));
            lines.push(format!("qsp: {}", state.status.qsp));
            lines.push(format!(
                "verification: {}",
                if state.is_locked() {
                    "hidden (unlock required)"
                } else if state.session.verified {
                    "verified"
                } else {
                    "not_verified"
                }
            ));
            lines.push(format!("replay_rejects: {}", replay_rejects));
            lines.push(String::new());
            lines.push("Metadata".to_string());
            if state.is_locked() {
                lines.push("- identity: hidden (unlock required)".to_string());
                lines.push("- peer key: hidden (unlock required)".to_string());
                lines.push("- transport key: hidden (unlock required)".to_string());
            } else {
                lines.push("- identity: inspection only".to_string());
                lines.push("- peer key: inspection only".to_string());
                lines.push("- transport key: inspection only".to_string());
            }
            lines.push(String::new());
            lines.push("Commands (command bar only)".to_string());
            lines.push("- /verify <peer> <fp>".to_string());
            lines.push("- /contacts add <peer> <fp>".to_string());
            lines.push("- /contacts block <peer>".to_string());
            lines.join("\n")
        }
        TuiInspectorPane::Contacts => {
            let mut lines = Vec::new();
            lines.push("Contacts Overview".to_string());
            lines.push(String::new());
            lines.push(format!("contacts: {}", state.contacts.len()));
            lines.push(String::new());
            let selected = state.selected_contact_label();
            let rec = contacts_entry_read(selected.as_str()).ok().flatten();
            lines.push(format!("selected: {}", selected));
            lines.push(format!("state: {}", contact_state(rec.as_ref())));
            lines.push(format!(
                "blocked: {}",
                bool_str(rec.as_ref().map(|v| v.blocked).unwrap_or(false))
            ));
            lines.push(format!(
                "fingerprint: {}",
                if state.is_locked() {
                    "hidden (unlock required)".to_string()
                } else {
                    rec.as_ref()
                        .map(|v| v.fp.clone())
                        .unwrap_or_else(|| "unknown".to_string())
                }
            ));
            lines.push(String::new());
            lines.push("Verification / Pinning".to_string());
            lines.push("- fingerprints are explicit; no implicit trust changes".to_string());
            lines.push("- mismatch is resolved by explicit verify command".to_string());
            lines.push(String::new());
            lines.push("Commands (command bar only)".to_string());
            lines.push("- /contacts add <label> <fp>".to_string());
            lines.push("- /contacts block <label>".to_string());
            lines.push("- /contacts unblock <label>".to_string());
            lines.push("- /contacts list".to_string());
            lines.push(String::new());
            lines.push("Known contacts".to_string());
            for c in state.contacts.iter().take(TUI_INSPECTOR_CONTACTS_MAX) {
                lines.push(format!("- {}", contact_display_line(c)));
            }
            lines.join("\n")
        }
        TuiInspectorPane::Settings => {
            let poll_interval = if state.poll_mode() == TuiPollMode::Fixed {
                state.poll_interval_seconds().to_string()
            } else {
                "n/a".to_string()
            };
            [
                "System Settings".to_string(),
                String::new(),
                "Lock:".to_string(),
                format!("  state: {}", state.status.locked),
                String::new(),
                "Auto-lock:".to_string(),
                "  enabled_by_default: true".to_string(),
                format!("  timeout_minutes: {}", state.autolock_minutes()),
                String::new(),
                "Polling:".to_string(),
                format!("  mode: {}", state.poll_mode().as_str()),
                format!("  interval_seconds: {}", poll_interval),
                String::new(),
                "Commands:".to_string(),
                "  /status".to_string(),
                "  /autolock show".to_string(),
                "  /autolock set <minutes>".to_string(),
                "  /poll show".to_string(),
                "  /poll set adaptive".to_string(),
                "  /poll set fixed <seconds>".to_string(),
            ]
            .join("\n")
        }
        TuiInspectorPane::Lock => {
            let mut lines = Vec::new();
            lines.push("Lock Status".to_string());
            lines.push(String::new());
            lines.push(format!("State: {}", state.status.locked));
            if state.status.locked == "UNLOCKED" {
                lines.push("Effect: sensitive content is displayed while UNLOCKED.".to_string());
            } else {
                lines.push("Effect: sensitive content is redacted while LOCKED.".to_string());
            }
            lines.push(String::new());
            lines.push(format!(
                "Auto-lock: enabled, timeout={} min",
                state.autolock_minutes()
            ));
            lines.push(String::new());
            lines.push("Commands:".to_string());
            lines.push("  /lock".to_string());
            lines.push("  /autolock show".to_string());
            lines.push("  /autolock set <min>".to_string());
            lines.join("\n")
        }
        TuiInspectorPane::Help => [
            "Help".to_string(),
            String::new(),
            "Global".to_string(),
            "- /help (opens fullscreen help)".to_string(),
            "- /inspector <domain>".to_string(),
            "- /exit".to_string(),
            String::new(),
            "Keybindings".to_string(),
            "- Tab / Shift+Tab: cycle focus".to_string(),
            "- Up / Down: move nav selection".to_string(),
            "- Enter: activate selected nav item".to_string(),
            "- Esc: clear/cancel or exit".to_string(),
            String::new(),
            "Safety".to_string(),
            "- command bar explicit intent only".to_string(),
        ]
        .join("\n"),
        TuiInspectorPane::About => [
            "About".to_string(),
            String::new(),
            format!("version: {}", env!("CARGO_PKG_VERSION")),
            format!(
                "commit: {}",
                option_env!("QSC_GIT_SHA")
                    .or(option_env!("VERGEN_GIT_SHA"))
                    .unwrap_or("unknown")
            ),
            "posture: truthful state reflection; explicit intent only".to_string(),
        ]
        .join("\n"),
        TuiInspectorPane::Legal => [
            "Legal".to_string(),
            String::new(),
            "Use at your own risk.".to_string(),
            "No warranty is provided by this interface.".to_string(),
            "Follow local policy and applicable law.".to_string(),
        ]
        .join("\n"),
    };
    let panel = Paragraph::new(body).block(Block::default().borders(Borders::ALL));
    f.render_widget(panel, area);
}

fn compute_envelope_status(payload_len: usize) -> String {
    let plan = envelope::plan_for_payload_len(
        payload_len,
        3,
        100,
        envelope::MAX_TICKS_DEFAULT,
        envelope::MAX_BUNDLE_SIZE_DEFAULT,
        envelope::MAX_PAYLOAD_COUNT_DEFAULT,
    );
    match plan {
        Ok(p) => {
            let tick = p.ticks.first().copied().unwrap_or(0);
            format!("bucket={} tick={}", p.bundle.bucket_len, tick)
        }
        Err(e) => format!("invalid({})", e.code()),
    }
}

fn compute_local_fingerprint() -> String {
    match identity_self_fingerprint("self") {
        Ok(fp) => fp,
        Err(_) => "untrusted".to_string(),
    }
}

fn compute_peer_fingerprint(peer: &str) -> String {
    let (fp, pinned) = identity_peer_status(peer);
    if pinned {
        format!("{} (pinned)", fp)
    } else {
        "untrusted".to_string()
    }
}

fn identity_peer_status(peer: &str) -> (String, bool) {
    match identity_read_pin(peer) {
        Ok(Some(fp)) => (fp, true),
        Ok(None) => ("untrusted".to_string(), false),
        Err(_) => ("untrusted".to_string(), false),
    }
}

fn identity_show(self_label: &str) {
    let Some(rec) =
        identity_read_self_public(self_label).unwrap_or_else(|e| print_error_marker(e.as_str()))
    else {
        emit_marker(
            "identity_show",
            None,
            &[("ok", "false"), ("reason", "missing_identity")],
        );
        print_error_marker("identity_missing");
    };
    let fp = identity_fingerprint_from_pk(&rec.kem_pk);
    emit_marker(
        "identity_show",
        None,
        &[("ok", "true"), ("fp", fp.as_str())],
    );
    println!("identity_fp={}", fp);
}

fn identity_rotate(self_label: &str, confirm: bool, reset_peers: bool) {
    if !require_unlocked("identity_rotate") {
        return;
    }
    if !confirm {
        emit_marker(
            "identity_rotate",
            None,
            &[("ok", "false"), ("reason", "confirm_required")],
        );
        print_error_marker("identity_rotate_confirm_required");
    }
    let (kem_pk, kem_sk) = hs_kem_keypair();
    let (sig_pk, sig_sk) = hs_sig_keypair();
    if identity_secret_store(self_label, &kem_sk).is_err() {
        emit_marker(
            "identity_secret_unavailable",
            None,
            &[("reason", "vault_missing_or_locked")],
        );
        print_error_marker("identity_secret_unavailable");
    }
    if identity_sig_secret_store(self_label, &sig_sk).is_err() {
        emit_marker(
            "identity_secret_unavailable",
            None,
            &[("reason", "vault_missing_or_locked")],
        );
        print_error_marker("identity_secret_unavailable");
    }
    if identity_write_public_record(self_label, &kem_pk, &sig_pk).is_err() {
        emit_marker(
            "identity_rotate",
            None,
            &[("ok", "false"), ("reason", "write_failed")],
        );
        print_error_marker("identity_rotate_write_failed");
    }
    if reset_peers {
        let empty = ContactsStore::default();
        let _ = contacts_store_save(&empty);
        if let Ok((dir, source)) = config_dir() {
            let identities = identities_dir(&dir);
            if ensure_dir_secure(&identities, source).is_ok() {
                if let Ok(entries) = fs::read_dir(&identities) {
                    for entry in entries.flatten() {
                        if let Some(name) = entry.file_name().to_str() {
                            if name.starts_with("peer_") && name.ends_with(".fp") {
                                let _ = fs::remove_file(entry.path());
                            }
                        }
                    }
                }
            }
        }
    }
    let fp = identity_fingerprint_from_pk(&kem_pk);
    emit_marker(
        "identity_rotate",
        None,
        &[("ok", "true"), ("fp", fp.as_str())],
    );
    println!("identity_fp={}", fp);
}

fn peers_list() {
    let mut peers = contacts_list_entries()
        .unwrap_or_else(|_| print_error_marker("contacts_store_unavailable"))
        .into_iter()
        .map(|(label, rec)| (label, rec.fp))
        .collect::<Vec<_>>();
    peers.sort_by(|a, b| a.0.cmp(&b.0));
    let count_s = peers.len().to_string();
    emit_marker("peers_list", None, &[("count", count_s.as_str())]);
    for (peer, fp) in peers.iter() {
        emit_marker(
            "peer_item",
            None,
            &[
                ("peer", peer.as_str()),
                ("fp", fp.as_str()),
                ("status", "pinned"),
            ],
        );
        println!("peer={} fp={} status=pinned", peer, fp);
    }
}

fn env_bool(key: &str) -> bool {
    matches!(
        env::var(key).ok().as_deref(),
        Some("1") | Some("true") | Some("TRUE") | Some("yes") | Some("YES")
    )
}

fn tui_color_enabled() -> bool {
    if env::var_os("NO_COLOR").is_some() {
        return false;
    }
    if env::var("TERM")
        .ok()
        .map(|v| v.eq_ignore_ascii_case("dumb"))
        .unwrap_or(false)
    {
        return false;
    }
    true
}

fn config_set(key: &str, value: &str) {
    if key != "policy-profile" {
        print_error(ErrorCode::ParseFailed);
    }
    let profile = match normalize_profile(value) {
        Ok(v) => v,
        Err(e) => print_error(e),
    };

    let (dir, source) = match config_dir() {
        Ok(v) => v,
        Err(e) => print_error(e),
    };
    let file = dir.join(CONFIG_FILE_NAME);

    let _lock = match lock_store_exclusive(&dir, source) {
        Ok(v) => v,
        Err(e) => print_error(e),
    };
    if let Err(e) = ensure_store_layout(&dir, source) {
        print_error(e);
    }
    if let Err(e) = write_config_atomic(&file, &profile, source) {
        print_error(e);
    }

    print_marker(
        "config_set",
        &[
            ("key", "policy_profile"),
            ("value", &profile),
            ("ok", "true"),
        ],
    );
}

fn config_get(key: &str) {
    if key != "policy-profile" {
        print_error(ErrorCode::ParseFailed);
    }
    let (dir, source) = match config_dir() {
        Ok(v) => v,
        Err(e) => print_error(e),
    };
    let file = dir.join(CONFIG_FILE_NAME);

    if let Err(e) = enforce_safe_parents(&file, source) {
        print_error(e);
    }
    let _lock = match lock_store_shared(&dir, source) {
        Ok(v) => v,
        Err(e) => print_error(e),
    };
    #[cfg(unix)]
    if file.exists() {
        if let Err(e) = enforce_file_perms(&file) {
            print_error(e);
        }
    }

    let value = match read_policy_profile(&file) {
        Ok(Some(v)) => v,
        Ok(None) => "unset".to_string(),
        Err(e) => print_error(e),
    };

    print_marker(
        "config_get",
        &[("key", "policy_profile"), ("value", &value), ("ok", "true")],
    );
}

#[derive(Serialize)]
struct DoctorReport {
    check_only: bool,
    ok: bool,
    dir_exists: bool,
    dir_writable: bool,
    file_parseable: bool,
    symlink_safe: bool,
    parent_safe: bool,
    config_dir: &'static str,
    redacted: bool,
}

fn doctor_check_only(check_only: bool, timeout_ms: u64, export: Option<PathBuf>) {
    if !check_only {
        print_error(ErrorCode::ParseFailed);
    }
    let (dir, source) = match config_dir() {
        Ok(v) => v,
        Err(e) => print_error(e),
    };
    let file = dir.join(CONFIG_FILE_NAME);

    let symlink_safe = check_symlink_safe(&dir);
    let parent_safe = check_parent_safe(&dir, source);
    let dir_exists = dir.is_dir();
    let checked_dir = dir.display().to_string();
    let dir_writable_required = false;
    let dir_writable = if dir_exists && symlink_safe && parent_safe {
        probe_dir_writable(&dir, timeout_ms)
    } else {
        false
    };

    let file_parseable = file.exists()
        && matches!(read_policy_profile(&file), Ok(Some(_)) | Ok(None))
        || !file.exists();

    let report = DoctorReport {
        check_only: true,
        ok: true,
        dir_exists,
        dir_writable,
        file_parseable,
        symlink_safe,
        parent_safe,
        config_dir: "<redacted>",
        redacted: true,
    };

    if let Some(path) = export {
        if let Err(e) = write_doctor_export(&path, &report) {
            print_error(e);
        }
    }

    print_marker(
        "doctor",
        &[
            ("check_only", "true"),
            ("ok", "true"),
            ("checked_dir", &checked_dir),
            (
                "dir_writable_required",
                if dir_writable_required {
                    "true"
                } else {
                    "false"
                },
            ),
            ("dir_exists", bool_str(dir_exists)),
            ("dir_writable", bool_str(dir_writable)),
            ("file_parseable", bool_str(file_parseable)),
            ("symlink_safe", bool_str(symlink_safe)),
            ("parent_safe", bool_str(parent_safe)),
        ],
    );
}

fn config_dir() -> Result<(PathBuf, ConfigSource), ErrorCode> {
    if let Ok(v) = env::var("QSC_CONFIG_DIR") {
        if !v.trim().is_empty() {
            return Ok((PathBuf::from(v), ConfigSource::EnvOverride));
        }
    }
    if let Ok(v) = env::var("XDG_CONFIG_HOME") {
        if !v.trim().is_empty() {
            return Ok((PathBuf::from(v).join("qsc"), ConfigSource::XdgConfigHome));
        }
    }
    if let Ok(home) = env::var("HOME") {
        if !home.trim().is_empty() {
            return Ok((
                PathBuf::from(home).join(".config").join("qsc"),
                ConfigSource::DefaultHome,
            ));
        }
    }
    Err(ErrorCode::MissingHome)
}

#[derive(Serialize, Deserialize)]
struct OutboxRecord {
    version: u8,
    payload_len: usize,
}

#[derive(Serialize, Deserialize)]
struct QspStatusRecord {
    active: bool,
    reason: String,
    last_pack_ok: bool,
    last_unpack_ok: bool,
}

const QSP_SESSIONS_DIR: &str = "qsp_sessions";
const QSP_SESSION_LEGACY_TOMBSTONE: &[u8] = b"QSC_SESSION_MIGRATED_V1\n";
const QSP_SESSION_BLOB_MAGIC: &[u8; 6] = b"QSSV01";
const QSP_SESSION_BLOB_VERSION: u8 = 1;
const QSP_SESSION_STORE_KEY_SECRET: &str = "qsp_session_store_key_v1";
const CONTACTS_SECRET_KEY: &str = "contacts.json";
const TIMELINE_SECRET_KEY: &str = "timeline.json";
const FILE_XFER_VERSION: u8 = 1;
const FILE_XFER_DEFAULT_MAX_FILE_SIZE: usize = 256 * 1024;
const FILE_XFER_MAX_FILE_SIZE_CEILING: usize = 4 * 1024 * 1024;
const FILE_XFER_DEFAULT_CHUNK_SIZE: usize = 16 * 1024;
const FILE_XFER_MAX_CHUNK_SIZE_CEILING: usize = 64 * 1024;
const FILE_XFER_DEFAULT_MAX_CHUNKS: usize = 64;
const FILE_XFER_MAX_CHUNKS_CEILING: usize = 256;

#[derive(Serialize, Deserialize, Clone, Debug)]
struct FileTransferRecord {
    id: String,
    peer: String,
    filename: String,
    total_size: usize,
    chunk_count: usize,
    manifest_hash: String,
    #[serde(default)]
    chunk_hashes: Vec<String>,
    #[serde(default)]
    chunks_hex: Vec<String>,
    state: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
struct ContactsStore {
    peers: BTreeMap<String, ContactRecord>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct ContactRecord {
    fp: String,
    status: String,
    blocked: bool,
    #[serde(default)]
    seen_at: Option<u64>,
    #[serde(default)]
    sig_fp: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
struct TimelineStore {
    #[serde(default = "timeline_ts_default")]
    next_ts: u64,
    #[serde(default)]
    peers: BTreeMap<String, Vec<TimelineEntry>>,
    #[serde(default)]
    file_transfers: BTreeMap<String, FileTransferRecord>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct TimelineEntry {
    id: String,
    peer: String,
    direction: String,
    byte_len: usize,
    kind: String,
    ts: u64,
    #[serde(default)]
    state: String,
    #[serde(default)]
    status: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct FileTransferChunkPayload {
    v: u8,
    t: String,
    file_id: String,
    filename: String,
    total_size: usize,
    chunk_index: usize,
    chunk_count: usize,
    chunk_hash: String,
    manifest_hash: String,
    chunk: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug)]
struct FileTransferManifestPayload {
    v: u8,
    t: String,
    file_id: String,
    filename: String,
    total_size: usize,
    chunk_count: usize,
    chunk_hashes: Vec<String>,
    manifest_hash: String,
}

enum FileTransferPayload {
    Chunk(FileTransferChunkPayload),
    Manifest(FileTransferManifestPayload),
}

fn timeline_entry_default_state(direction: &str, status: &str) -> MessageState {
    if let Some(parsed) = MessageState::parse(status) {
        return parsed;
    }
    if direction == "out" {
        MessageState::Sent
    } else {
        MessageState::Received
    }
}

fn timeline_ts_default() -> u64 {
    1
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum MessageState {
    Created,
    Sent,
    Received,
    Delivered,
    Failed,
}

impl MessageState {
    fn as_str(self) -> &'static str {
        match self {
            MessageState::Created => "CREATED",
            MessageState::Sent => "SENT",
            MessageState::Received => "RECEIVED",
            MessageState::Delivered => "DELIVERED",
            MessageState::Failed => "FAILED",
        }
    }

    fn as_status(self) -> &'static str {
        match self {
            MessageState::Created => "created",
            MessageState::Sent => "sent",
            MessageState::Received => "received",
            MessageState::Delivered => "delivered",
            MessageState::Failed => "failed",
        }
    }

    fn parse(s: &str) -> Option<Self> {
        match s {
            "CREATED" | "created" => Some(MessageState::Created),
            "SENT" | "sent" => Some(MessageState::Sent),
            "RECEIVED" | "received" => Some(MessageState::Received),
            "DELIVERED" | "delivered" => Some(MessageState::Delivered),
            "FAILED" | "failed" => Some(MessageState::Failed),
            _ => None,
        }
    }
}

fn message_state_transition_allowed(
    from: MessageState,
    to: MessageState,
    direction: &str,
) -> Result<(), &'static str> {
    if from == MessageState::Failed {
        return Err("failed_terminal");
    }
    if from == to {
        return Err("state_duplicate");
    }
    if direction == "out" {
        return match (from, to) {
            (MessageState::Created, MessageState::Sent)
            | (MessageState::Created, MessageState::Failed)
            | (MessageState::Sent, MessageState::Delivered)
            | (MessageState::Sent, MessageState::Failed) => Ok(()),
            _ => Err("state_invalid_transition"),
        };
    }
    match (from, to) {
        (MessageState::Created, MessageState::Received)
        | (MessageState::Created, MessageState::Failed)
        | (MessageState::Received, MessageState::Failed) => Ok(()),
        _ => Err("state_invalid_transition"),
    }
}

fn emit_message_state_transition(id: &str, from: MessageState, to: MessageState) {
    emit_marker(
        "message_state_transition",
        None,
        &[
            ("from", from.as_str()),
            ("to", to.as_str()),
            ("id", id),
            ("ok", "true"),
        ],
    );
}

fn emit_message_state_reject(id: &str, reason: &'static str) {
    emit_marker(
        "message_state_reject",
        Some(reason),
        &[("reason", reason), ("id", id)],
    );
}

fn timeline_entry_state(entry: &TimelineEntry) -> MessageState {
    MessageState::parse(entry.state.as_str())
        .or_else(|| MessageState::parse(entry.status.as_str()))
        .unwrap_or_else(|| {
            timeline_entry_default_state(entry.direction.as_str(), entry.status.as_str())
        })
}

fn tui_file_display_state(raw: &str) -> String {
    let upper = raw.trim().to_ascii_uppercase();
    match upper.as_str() {
        "VERIFIED" | "COMPLETE" => "VERIFIED".to_string(),
        "FAILED" | "REJECTED" => "FAILED".to_string(),
        "RECEIVING" | "CREATED" | "SENT" | "ANNOUNCED" | "PENDING" => "RECEIVING".to_string(),
        _ => upper,
    }
}

fn load_tui_files_snapshot() -> Vec<TuiFileItem> {
    let store = match timeline_store_load() {
        Ok(v) => v,
        Err(_) => return Vec::new(),
    };
    let mut out: Vec<TuiFileItem> = store
        .file_transfers
        .values()
        .map(|rec| TuiFileItem {
            id: rec.id.clone(),
            peer: rec.peer.clone(),
            filename: rec.filename.clone(),
            byte_len: rec.total_size,
            state: rec.state.clone(),
            display_state: tui_file_display_state(rec.state.as_str()),
        })
        .collect();
    out.sort_by(|a, b| a.id.cmp(&b.id));
    out
}

fn qsp_status_path(dir: &Path) -> PathBuf {
    dir.join(QSP_STATUS_FILE_NAME)
}

fn write_qsp_status(dir: &Path, source: ConfigSource, status: &QspStatusRecord) {
    let bytes = match serde_json::to_vec(status) {
        Ok(v) => v,
        Err(_) => return,
    };
    let _ = write_atomic(&qsp_status_path(dir), &bytes, source);
}

fn record_qsp_status(
    dir: &Path,
    source: ConfigSource,
    active: bool,
    reason: &str,
    pack_ok: bool,
    unpack_ok: bool,
) {
    let status = QspStatusRecord {
        active,
        reason: reason.to_string(),
        last_pack_ok: pack_ok,
        last_unpack_ok: unpack_ok,
    };
    write_qsp_status(dir, source, &status);
}

fn qsp_status_tuple(peer: &str) -> (String, String) {
    let (dir, source) = match config_dir() {
        Ok(v) => v,
        Err(_) => return ("INACTIVE".to_string(), "missing_home".to_string()),
    };
    if !check_parent_safe(&dir, source) {
        return ("INACTIVE".to_string(), "unsafe_parent".to_string());
    }
    if !channel_label_ok(peer) {
        return ("INACTIVE".to_string(), "channel_invalid".to_string());
    }
    match qsp_session_load(peer) {
        Ok(Some(_)) => ("ACTIVE".to_string(), "handshake".to_string()),
        Ok(None) => {
            if env::var("QSC_QSP_SEED").is_ok() {
                ("INACTIVE".to_string(), "no_session".to_string())
            } else {
                ("INACTIVE".to_string(), "missing_seed".to_string())
            }
        }
        Err(ErrorCode::ParseFailed) => ("INACTIVE".to_string(), "session_invalid".to_string()),
        Err(_) => ("INACTIVE".to_string(), "session_invalid".to_string()),
    }
}

fn qsp_status_string(peer: &str) -> String {
    let (status, reason) = qsp_status_tuple(peer);
    format!("{} reason={}", status, reason)
}

fn qsp_sessions_dir(dir: &Path) -> PathBuf {
    dir.join(QSP_SESSIONS_DIR)
}

fn qsp_session_path(dir: &Path, peer: &str) -> PathBuf {
    qsp_sessions_dir(dir).join(format!("{}.bin", peer))
}

fn qsp_session_blob_path(dir: &Path, peer: &str) -> PathBuf {
    qsp_sessions_dir(dir).join(format!("{}.qsv", peer))
}

fn qsp_session_aad(peer: &str) -> Vec<u8> {
    format!("QSC.QSP.SESSION.V{}:{}", QSP_SESSION_BLOB_VERSION, peer).into_bytes()
}

fn qsp_session_test_fallback_key(peer: &str) -> Result<[u8; 32], ErrorCode> {
    let seed = qsp_seed_from_env().map_err(|_| ErrorCode::IdentitySecretUnavailable)?;
    let c = StdCrypto;
    let seed_bytes = seed.to_le_bytes();
    let seed_hash = c.sha512(&seed_bytes);
    let mut seed_key = [0u8; 32];
    seed_key.copy_from_slice(&seed_hash[..32]);
    Ok(kmac_out::<32>(
        &c,
        &seed_key,
        "QSC.QSP.SESSION.STORE.TESTKEY",
        peer.as_bytes(),
    ))
}

fn qsp_session_decode_key(secret: &str) -> Result<[u8; 32], ErrorCode> {
    let raw = hex_decode(secret)?;
    if raw.len() != 32 {
        return Err(ErrorCode::ParseFailed);
    }
    let mut key = [0u8; 32];
    key.copy_from_slice(&raw);
    Ok(key)
}

fn qsp_session_store_key_load(peer: &str) -> Result<[u8; 32], ErrorCode> {
    match vault::secret_get(QSP_SESSION_STORE_KEY_SECRET) {
        Ok(Some(v)) => qsp_session_decode_key(&v),
        Ok(None) => Err(ErrorCode::IdentitySecretUnavailable),
        Err("vault_missing" | "vault_locked") => {
            if allow_seed_fallback_for_tests() {
                qsp_session_test_fallback_key(peer)
            } else {
                Err(ErrorCode::IdentitySecretUnavailable)
            }
        }
        Err(_) => Err(ErrorCode::IdentitySecretUnavailable),
    }
}

fn qsp_session_store_key_get_or_create(peer: &str) -> Result<[u8; 32], ErrorCode> {
    match vault::secret_get(QSP_SESSION_STORE_KEY_SECRET) {
        Ok(Some(v)) => qsp_session_decode_key(&v),
        Ok(None) => {
            let mut key = [0u8; 32];
            OsRng.fill_bytes(&mut key);
            let secret = hex_encode(&key);
            match vault::secret_set(QSP_SESSION_STORE_KEY_SECRET, &secret) {
                Ok(()) => Ok(key),
                Err("vault_missing" | "vault_locked") => {
                    if allow_seed_fallback_for_tests() {
                        qsp_session_test_fallback_key(peer)
                    } else {
                        Err(ErrorCode::IdentitySecretUnavailable)
                    }
                }
                Err(_) => Err(ErrorCode::IdentitySecretUnavailable),
            }
        }
        Err("vault_missing" | "vault_locked") => {
            if allow_seed_fallback_for_tests() {
                qsp_session_test_fallback_key(peer)
            } else {
                Err(ErrorCode::IdentitySecretUnavailable)
            }
        }
        Err(_) => Err(ErrorCode::IdentitySecretUnavailable),
    }
}

fn qsp_session_encrypt_blob(peer: &str, plaintext: &[u8]) -> Result<Vec<u8>, ErrorCode> {
    let key = qsp_session_store_key_get_or_create(peer)?;
    let cipher = ChaCha20Poly1305::new(Key::from_slice(&key));
    let mut nonce_bytes = [0u8; 12];
    OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);
    let aad = qsp_session_aad(peer);
    let payload = Payload {
        msg: plaintext,
        aad: aad.as_slice(),
    };
    let ciphertext = cipher
        .encrypt(nonce, payload)
        .map_err(|_| ErrorCode::ParseFailed)?;
    let mut out = Vec::with_capacity(6 + 1 + 1 + 4 + 12 + ciphertext.len());
    out.extend_from_slice(QSP_SESSION_BLOB_MAGIC);
    out.push(QSP_SESSION_BLOB_VERSION);
    out.push(12);
    out.extend_from_slice(&(ciphertext.len() as u32).to_le_bytes());
    out.extend_from_slice(&nonce_bytes);
    out.extend_from_slice(&ciphertext);
    Ok(out)
}

fn qsp_session_decrypt_blob(peer: &str, blob: &[u8]) -> Result<Vec<u8>, &'static str> {
    let min = 6 + 1 + 1 + 4 + 12;
    if blob.len() < min || &blob[..6] != QSP_SESSION_BLOB_MAGIC {
        return Err("session_decrypt_failed");
    }
    if blob[6] != QSP_SESSION_BLOB_VERSION {
        return Err("session_decrypt_failed");
    }
    let nonce_len = blob[7] as usize;
    if nonce_len != 12 {
        return Err("session_decrypt_failed");
    }
    let ct_len = u32::from_le_bytes([blob[8], blob[9], blob[10], blob[11]]) as usize;
    let need = 12 + nonce_len + ct_len;
    if blob.len() < need {
        return Err("session_decrypt_failed");
    }
    let nonce_bytes = &blob[12..12 + nonce_len];
    let ciphertext = &blob[12 + nonce_len..need];
    let key = match qsp_session_store_key_load(peer) {
        Ok(v) => v,
        Err(ErrorCode::IdentitySecretUnavailable) => return Err("session_decrypt_failed"),
        Err(_) => return Err("session_decrypt_failed"),
    };
    let cipher = ChaCha20Poly1305::new(Key::from_slice(&key));
    let nonce = Nonce::from_slice(nonce_bytes);
    let aad = qsp_session_aad(peer);
    let payload = Payload {
        msg: ciphertext,
        aad: aad.as_slice(),
    };
    cipher
        .decrypt(nonce, payload)
        .map_err(|_| "session_integrity_failed")
}

fn qsp_session_load_encrypted(
    peer: &str,
    source: ConfigSource,
    blob_path: &Path,
) -> Result<Suite2SessionState, ErrorCode> {
    enforce_safe_parents(blob_path, source)?;
    let blob = fs::read(blob_path).map_err(|_| ErrorCode::IoReadFailed)?;
    let plaintext = match qsp_session_decrypt_blob(peer, &blob) {
        Ok(v) => v,
        Err(code) => {
            emit_marker("error", Some(code), &[]);
            return Err(ErrorCode::ParseFailed);
        }
    };
    let st = Suite2SessionState::restore_bytes(&plaintext).map_err(|_| {
        emit_marker("error", Some("session_decrypt_failed"), &[]);
        ErrorCode::ParseFailed
    })?;
    emit_marker("session_load", None, &[("ok", "true"), ("format", "v1")]);
    Ok(st)
}

fn qsp_session_migrate_legacy(
    peer: &str,
    source: ConfigSource,
    legacy_path: &Path,
    blob_path: &Path,
) -> Result<Option<Suite2SessionState>, ErrorCode> {
    enforce_safe_parents(legacy_path, source)?;
    let legacy = fs::read(legacy_path).map_err(|_| ErrorCode::IoReadFailed)?;
    if legacy == QSP_SESSION_LEGACY_TOMBSTONE {
        emit_marker(
            "session_migrate",
            None,
            &[
                ("ok", "true"),
                ("action", "skipped"),
                ("reason", "already_migrated"),
            ],
        );
        return Ok(None);
    }
    let st = Suite2SessionState::restore_bytes(&legacy).map_err(|_| ErrorCode::ParseFailed)?;
    let blob = match qsp_session_encrypt_blob(peer, &legacy) {
        Ok(v) => v,
        Err(ErrorCode::IdentitySecretUnavailable) => {
            emit_marker(
                "session_migrate",
                Some("migration_blocked"),
                &[
                    ("ok", "false"),
                    ("action", "skipped"),
                    ("reason", "vault_unavailable"),
                ],
            );
            return Err(ErrorCode::IdentitySecretUnavailable);
        }
        Err(e) => return Err(e),
    };
    write_atomic(blob_path, &blob, source)?;
    if let Err(e) = write_atomic(legacy_path, QSP_SESSION_LEGACY_TOMBSTONE, source) {
        let _ = fs::remove_file(blob_path);
        return Err(e);
    }
    emit_marker(
        "session_migrate",
        None,
        &[
            ("ok", "true"),
            ("action", "imported"),
            ("reason", "legacy_plaintext"),
        ],
    );
    Ok(Some(st))
}

fn qsp_session_load(peer: &str) -> Result<Option<Suite2SessionState>, ErrorCode> {
    if !channel_label_ok(peer) {
        return Err(ErrorCode::ParseFailed);
    }
    let (dir, source) = config_dir()?;
    let blob_path = qsp_session_blob_path(&dir, peer);
    if blob_path.exists() {
        return qsp_session_load_encrypted(peer, source, &blob_path).map(Some);
    }
    let legacy_path = qsp_session_path(&dir, peer);
    if legacy_path.exists() {
        return qsp_session_migrate_legacy(peer, source, &legacy_path, &blob_path);
    }
    Ok(None)
}

fn qsp_session_store(peer: &str, st: &Suite2SessionState) -> Result<(), ErrorCode> {
    if !channel_label_ok(peer) {
        return Err(ErrorCode::ParseFailed);
    }
    let (dir, source) = config_dir()?;
    let sessions = qsp_sessions_dir(&dir);
    enforce_safe_parents(&sessions, source)?;
    fs::create_dir_all(&sessions).map_err(|_| ErrorCode::IoWriteFailed)?;
    let bytes = st.snapshot_bytes();
    let blob = qsp_session_encrypt_blob(peer, &bytes)?;
    let blob_path = qsp_session_blob_path(&dir, peer);
    write_atomic(&blob_path, &blob, source)?;
    let legacy_path = qsp_session_path(&dir, peer);
    if legacy_path.exists() {
        write_atomic(&legacy_path, QSP_SESSION_LEGACY_TOMBSTONE, source)?;
    }
    emit_marker(
        "session_store",
        None,
        &[("ok", "true"), ("format", "v1"), ("enc", "aead")],
    );
    Ok(())
}

fn protocol_active_or_reason_for_peer(peer: &str) -> Result<(), String> {
    let (status, reason) = qsp_status_tuple(peer);
    if status == "ACTIVE" || (allow_seed_fallback_for_tests() && env::var("QSC_QSP_SEED").is_ok()) {
        Ok(())
    } else {
        Err(reason)
    }
}

fn emit_protocol_inactive(reason: &str) {
    emit_marker("error", Some("protocol_inactive"), &[("reason", reason)]);
}

fn protocol_inactive_exit(reason: &str) -> ! {
    emit_protocol_inactive(reason);
    process::exit(1);
}

fn allow_seed_fallback_for_tests() -> bool {
    env_bool("QSC_ALLOW_SEED_FALLBACK")
}

fn qsp_seed_from_env() -> Result<u64, &'static str> {
    let seed_str = env::var("QSC_QSP_SEED").map_err(|_| "qsp_seed_required")?;
    let seed = seed_str
        .trim()
        .parse::<u64>()
        .map_err(|_| "qsp_seed_invalid")?;
    Ok(seed)
}

fn kmac_out<const N: usize>(kmac: &StdCrypto, key: &[u8], label: &str, data: &[u8]) -> [u8; N] {
    let out = kmac.kmac256(key, label, data, N);
    out[..N].try_into().expect("kmac output")
}

fn qsp_session_for_channel(channel: &str) -> Result<Suite2SessionState, &'static str> {
    if !channel_label_ok(channel) {
        return Err("qsp_channel_invalid");
    }
    if let Ok(Some(st)) = qsp_session_load(channel) {
        return Ok(st);
    }
    if !allow_seed_fallback_for_tests() {
        return Err("qsp_no_session");
    }
    let seed = qsp_seed_from_env()?;
    let c = StdCrypto;
    let seed_bytes = seed.to_le_bytes();
    let seed_hash = c.sha512(&seed_bytes);
    let mut seed_key = [0u8; 32];
    seed_key.copy_from_slice(&seed_hash[..32]);

    let base = kmac_out::<32>(&c, &seed_key, "QSC.QSP.BASE", channel.as_bytes());
    let session_id = kmac_out::<16>(&c, &base, "QSC.QSP.SID", channel.as_bytes());
    let hk = kmac_out::<32>(&c, &base, "QSC.QSP.HK", b"");
    let ck_ec = kmac_out::<32>(&c, &base, "QSC.QSP.CK.EC", b"");
    let ck_pq = kmac_out::<32>(&c, &base, "QSC.QSP.CK.PQ", b"");
    let rk = kmac_out::<32>(&c, &base, "QSC.QSP.RK", b"");
    let dh_pub = kmac_out::<32>(&c, &base, "QSC.QSP.DH", b"");

    let send = Suite2SendState {
        session_id,
        protocol_version: SUITE2_PROTOCOL_VERSION,
        suite_id: SUITE2_SUITE_ID,
        dh_pub,
        hk_s: hk,
        ck_ec,
        ck_pq,
        ns: 0,
        pn: 0,
    };
    let recv = Suite2RecvWireState {
        session_id,
        protocol_version: SUITE2_PROTOCOL_VERSION,
        suite_id: SUITE2_SUITE_ID,
        dh_pub,
        hk_r: hk,
        rk,
        ck_ec,
        ck_pq_send: ck_pq,
        ck_pq_recv: ck_pq,
        nr: 0,
        role_is_a: true,
        peer_max_adv_id_seen: 0,
        known_targets: BTreeSet::new(),
        consumed_targets: BTreeSet::new(),
        tombstoned_targets: BTreeSet::new(),
        mkskipped: Vec::new(),
    };
    Ok(Suite2SessionState { send, recv })
}

struct QspPackOutcome {
    envelope: Vec<u8>,
    next_state: Suite2SessionState,
    msg_idx: u32,
    ck_idx: u32,
    padded_len: usize,
    pad_label: Option<&'static str>,
}

struct QspUnpackOutcome {
    plaintext: Vec<u8>,
    next_state: Suite2SessionState,
    msg_idx: u32,
    skip_delta: usize,
    evicted: usize,
}

const MKSKIPPED_CAP_DEFAULT: usize = 32;
const POLL_INTERVAL_MS_MAX: u64 = 60_000;
const POLL_TICKS_MAX: u32 = 64;
const POLL_MAX_PER_TICK_MAX: u32 = 32;
const PAD_TO_MAX: usize = 65_536;
const META_TICK_COUNT_DEFAULT: u32 = 1;
const META_INTERVAL_MS_DEFAULT: u64 = 1_000;
const META_BATCH_MAX_COUNT_DEFAULT: u32 = 1;
const META_BUCKET_MAX_DEFAULT: usize = 4_096;
const META_BUCKET_MAX_CEILING: usize = 65_536;

struct MetaPollConfig {
    interval_ms: u64,
    ticks: u32,
    batch_max_count: usize,
    bucket_max: usize,
    deterministic: bool,
}

#[derive(Clone, Copy)]
struct MetaPadConfig {
    target_len: Option<usize>,
    profile: Option<EnvelopeProfile>,
    label: Option<&'static str>,
}

fn mkskipped_cap() -> usize {
    let cap = env::var("QSC_MKSKIPPED_CAP")
        .ok()
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(MKSKIPPED_CAP_DEFAULT);
    cap.clamp(1, 1000)
}

fn bound_mkskipped(st: &mut Suite2RecvWireState) -> usize {
    let cap = mkskipped_cap();
    if st.mkskipped.len() <= cap {
        return 0;
    }
    st.mkskipped.sort_by_key(|e| e.n);
    let excess = st.mkskipped.len().saturating_sub(cap);
    if excess > 0 {
        st.mkskipped.drain(0..excess);
    }
    excess
}

fn meta_poll_config_from_args(args: MetaPollArgs) -> Result<Option<MetaPollConfig>, &'static str> {
    let MetaPollArgs {
        deterministic_meta,
        interval_ms,
        poll_interval_ms,
        ticks,
        batch_max_count,
        poll_max_per_tick,
        bucket_max,
        meta_seed,
    } = args;
    if interval_ms.is_some() && poll_interval_ms.is_some() {
        return Err("meta_poll_conflict");
    }
    if batch_max_count.is_some() && poll_max_per_tick.is_some() {
        return Err("meta_poll_conflict");
    }
    let any = deterministic_meta
        || interval_ms.is_some()
        || poll_interval_ms.is_some()
        || ticks.is_some()
        || batch_max_count.is_some()
        || poll_max_per_tick.is_some()
        || bucket_max.is_some()
        || meta_seed.is_some();
    if !any {
        return Ok(None);
    }
    let interval_ms = interval_ms
        .or(poll_interval_ms)
        .unwrap_or(META_INTERVAL_MS_DEFAULT);
    let ticks = ticks.unwrap_or(META_TICK_COUNT_DEFAULT);
    let batch_max_count = batch_max_count
        .or(poll_max_per_tick)
        .unwrap_or(META_BATCH_MAX_COUNT_DEFAULT);
    let bucket_max = bucket_max.unwrap_or(META_BUCKET_MAX_DEFAULT);
    if interval_ms == 0 || interval_ms > POLL_INTERVAL_MS_MAX {
        return Err("meta_poll_invalid");
    }
    if ticks == 0 || ticks > POLL_TICKS_MAX {
        return Err("meta_poll_invalid");
    }
    if batch_max_count == 0 || batch_max_count > POLL_MAX_PER_TICK_MAX {
        return Err("meta_poll_invalid");
    }
    if bucket_max == 0 || bucket_max > META_BUCKET_MAX_CEILING {
        return Err("meta_poll_invalid");
    }
    Ok(Some(MetaPollConfig {
        interval_ms,
        ticks,
        batch_max_count: batch_max_count as usize,
        bucket_max,
        deterministic: deterministic_meta || meta_seed.is_some(),
    }))
}

struct MetaPollArgs {
    deterministic_meta: bool,
    interval_ms: Option<u64>,
    poll_interval_ms: Option<u64>,
    ticks: Option<u32>,
    batch_max_count: Option<u32>,
    poll_max_per_tick: Option<u32>,
    bucket_max: Option<usize>,
    meta_seed: Option<u64>,
}

fn meta_bucket_for_len(orig_len: usize, bucket_max: usize) -> usize {
    let capped = orig_len.min(bucket_max).max(1);
    let mut bucket = 1usize;
    while bucket < capped {
        bucket = bucket.saturating_mul(2);
    }
    bucket.min(bucket_max)
}

#[derive(Serialize, Deserialize, Debug)]
struct ReceiptControlPayload {
    v: u8,
    t: String,
    kind: String,
    msg_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    body: Option<Vec<u8>>,
}

fn receipt_kind_str(kind: ReceiptKind) -> &'static str {
    match kind {
        ReceiptKind::Delivered => "delivered",
    }
}

fn receipt_msg_id(payload: &[u8]) -> String {
    let c = StdCrypto;
    let h = c.sha512(payload);
    hex_encode(&h[..8])
}

fn encode_receipt_data_payload(
    payload: Vec<u8>,
    receipt: Option<ReceiptKind>,
) -> (Vec<u8>, Option<String>) {
    let Some(kind) = receipt else {
        return (payload, None);
    };
    let msg_id = receipt_msg_id(&payload);
    let ctrl = ReceiptControlPayload {
        v: 1,
        t: "data".to_string(),
        kind: receipt_kind_str(kind).to_string(),
        msg_id: msg_id.clone(),
        body: Some(payload),
    };
    let encoded =
        serde_json::to_vec(&ctrl).unwrap_or_else(|_| print_error_marker("receipt_encode_failed"));
    (encoded, Some(msg_id))
}

fn parse_receipt_payload(plaintext: &[u8]) -> Option<ReceiptControlPayload> {
    serde_json::from_slice::<ReceiptControlPayload>(plaintext).ok()
}

fn parse_file_transfer_payload(plaintext: &[u8]) -> Option<FileTransferPayload> {
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

fn file_xfer_chunk_hash(chunk: &[u8]) -> String {
    let c = StdCrypto;
    let hash = c.sha512(chunk);
    hex_encode(&hash[..16])
}

fn file_xfer_id(peer: &str, filename: &str, payload: &[u8]) -> String {
    let c = StdCrypto;
    let mut data = Vec::new();
    data.extend_from_slice(peer.as_bytes());
    data.push(0);
    data.extend_from_slice(filename.as_bytes());
    data.push(0);
    data.extend_from_slice(payload);
    let hash = c.sha512(&data);
    hex_encode(&hash[..12])
}

fn file_xfer_manifest_hash(
    peer: &str,
    file_id: &str,
    total_size: usize,
    chunk_count: usize,
    chunk_hashes: &[String],
) -> String {
    let c = StdCrypto;
    let joined = chunk_hashes.join(",");
    let data = format!(
        "{}|{}|{}|{}|{}",
        peer, file_id, total_size, chunk_count, joined
    );
    let hash = c.sha512(data.as_bytes());
    hex_encode(&hash[..16])
}

fn file_xfer_reject(id: &str, reason: &'static str) -> ! {
    emit_marker(
        "file_xfer_reject",
        Some(reason),
        &[("id", id), ("reason", reason)],
    );
    print_error_marker(reason);
}

fn file_xfer_store_key(peer: &str, file_id: &str) -> String {
    format!("{}:{}", peer, file_id)
}

fn file_send_execute(
    transport: Option<SendTransport>,
    relay: Option<&str>,
    to: &str,
    path: &Path,
    chunk_size: usize,
    max_file_size: usize,
    max_chunks: usize,
) {
    if !require_unlocked("file_send") {
        return;
    }
    let transport = match transport {
        Some(v) => v,
        None => file_xfer_reject("unknown", "file_xfer_transport_required"),
    };
    match transport {
        SendTransport::Relay => {}
    }
    let relay = match relay {
        Some(v) => v,
        None => file_xfer_reject("unknown", "file_xfer_relay_required"),
    };
    if !channel_label_ok(to) {
        file_xfer_reject("unknown", "file_xfer_peer_invalid");
    }
    if max_file_size == 0 || max_file_size > FILE_XFER_MAX_FILE_SIZE_CEILING {
        file_xfer_reject("unknown", "file_xfer_size_bound_invalid");
    }
    if chunk_size == 0 || chunk_size > FILE_XFER_MAX_CHUNK_SIZE_CEILING {
        file_xfer_reject("unknown", "file_xfer_chunk_bound_invalid");
    }
    if max_chunks == 0 || max_chunks > FILE_XFER_MAX_CHUNKS_CEILING {
        file_xfer_reject("unknown", "file_xfer_chunks_bound_invalid");
    }
    if let Err(code) = enforce_peer_not_blocked(to) {
        file_xfer_reject("unknown", code);
    }
    if let Err(reason) = protocol_active_or_reason_for_peer(to) {
        emit_marker(
            "file_xfer_reject",
            Some("protocol_inactive"),
            &[
                ("id", "unknown"),
                ("reason", "protocol_inactive"),
                ("detail", reason.as_str()),
            ],
        );
        protocol_inactive_exit(reason.as_str());
    }
    let payload =
        fs::read(path).unwrap_or_else(|_| file_xfer_reject("unknown", "file_xfer_read_failed"));
    if payload.is_empty() {
        file_xfer_reject("unknown", "file_xfer_empty");
    }
    if payload.len() > max_file_size {
        file_xfer_reject("unknown", "size_exceeds_max");
    }
    let chunk_count = payload.len().div_ceil(chunk_size);
    if chunk_count > max_chunks {
        file_xfer_reject("unknown", "chunk_count_exceeds_max");
    }
    let filename = path
        .file_name()
        .and_then(|v| v.to_str())
        .unwrap_or("file.bin")
        .to_string();
    let file_id = file_xfer_id(to, filename.as_str(), &payload);
    let size_s = payload.len().to_string();
    emit_marker(
        "file_xfer_prepare",
        None,
        &[
            ("id", file_id.as_str()),
            ("size", size_s.as_str()),
            ("ok", "true"),
        ],
    );
    let mut chunk_hashes = Vec::with_capacity(chunk_count);
    for idx in 0..chunk_count {
        let start = idx * chunk_size;
        let end = (start + chunk_size).min(payload.len());
        let chunk = &payload[start..end];
        chunk_hashes.push(file_xfer_chunk_hash(chunk));
    }
    let manifest_hash = file_xfer_manifest_hash(
        to,
        file_id.as_str(),
        payload.len(),
        chunk_count,
        chunk_hashes.as_slice(),
    );

    for (idx, chunk_hash) in chunk_hashes.iter().enumerate() {
        let start = idx * chunk_size;
        let end = (start + chunk_size).min(payload.len());
        let chunk = payload[start..end].to_vec();
        let chunk_payload = FileTransferChunkPayload {
            v: FILE_XFER_VERSION,
            t: "file_chunk".to_string(),
            file_id: file_id.clone(),
            filename: filename.clone(),
            total_size: payload.len(),
            chunk_index: idx,
            chunk_count,
            chunk_hash: chunk_hash.clone(),
            manifest_hash: manifest_hash.clone(),
            chunk,
        };
        let body = serde_json::to_vec(&chunk_payload)
            .unwrap_or_else(|_| file_xfer_reject(file_id.as_str(), "file_xfer_encode_failed"));
        let outcome = relay_send_with_payload(RelaySendPayloadArgs {
            to,
            payload: body,
            relay,
            injector: fault_injector_from_env(),
            pad_cfg: None,
            bucket_max: None,
            meta_seed: None,
            receipt: None,
        });
        if let Some(code) = outcome.error_code {
            file_xfer_reject(file_id.as_str(), code);
        }
        let idx_s = idx.to_string();
        emit_marker(
            "file_xfer_chunk",
            None,
            &[
                ("id", file_id.as_str()),
                ("idx", idx_s.as_str()),
                ("ok", "true"),
            ],
        );
    }

    let manifest = FileTransferManifestPayload {
        v: FILE_XFER_VERSION,
        t: "file_manifest".to_string(),
        file_id: file_id.clone(),
        filename,
        total_size: payload.len(),
        chunk_count,
        chunk_hashes,
        manifest_hash,
    };
    let manifest_body = serde_json::to_vec(&manifest)
        .unwrap_or_else(|_| file_xfer_reject(file_id.as_str(), "file_xfer_encode_failed"));
    let outcome = relay_send_with_payload(RelaySendPayloadArgs {
        to,
        payload: manifest_body,
        relay,
        injector: fault_injector_from_env(),
        pad_cfg: None,
        bucket_max: None,
        meta_seed: None,
        receipt: None,
    });
    if let Some(code) = outcome.error_code {
        file_xfer_reject(file_id.as_str(), code);
    }
    emit_marker(
        "file_xfer_manifest",
        None,
        &[("id", file_id.as_str()), ("ok", "true")],
    );
    if let Err(code) = timeline_append_entry(
        to,
        "out",
        payload.len(),
        "file",
        MessageState::Sent,
        Some(file_id.as_str()),
    ) {
        emit_message_state_reject(file_id.as_str(), code);
        file_xfer_reject(file_id.as_str(), code);
    }
    emit_marker(
        "file_xfer_complete",
        None,
        &[("id", file_id.as_str()), ("ok", "true")],
    );
}

fn file_transfer_handle_chunk(
    ctx: &ReceivePullCtx<'_>,
    chunk: FileTransferChunkPayload,
) -> Result<(), &'static str> {
    if chunk.total_size == 0 || chunk.total_size > FILE_XFER_DEFAULT_MAX_FILE_SIZE {
        return Err("size_exceeds_max");
    }
    if chunk.chunk_count == 0 || chunk.chunk_count > FILE_XFER_DEFAULT_MAX_CHUNKS {
        return Err("chunk_count_exceeds_max");
    }
    if chunk.chunk.len() > FILE_XFER_DEFAULT_CHUNK_SIZE {
        return Err("chunk_size_exceeds_max");
    }
    if chunk.chunk_index >= chunk.chunk_count {
        return Err("chunk_index_invalid");
    }
    if chunk.chunk_hash != file_xfer_chunk_hash(&chunk.chunk) {
        return Err("chunk_hash_invalid");
    }
    let key = file_xfer_store_key(ctx.from, chunk.file_id.as_str());
    let mut store = timeline_store_load().map_err(|_| "timeline_unavailable")?;
    let rec = store
        .file_transfers
        .entry(key)
        .or_insert_with(|| FileTransferRecord {
            id: chunk.file_id.clone(),
            peer: ctx.from.to_string(),
            filename: chunk.filename.clone(),
            total_size: chunk.total_size,
            chunk_count: chunk.chunk_count,
            manifest_hash: chunk.manifest_hash.clone(),
            chunk_hashes: Vec::new(),
            chunks_hex: Vec::new(),
            state: "RECEIVING".to_string(),
        });
    if rec.state == "FAILED" || rec.state == "VERIFIED" {
        return Err("state_invalid_transition");
    }
    if rec.total_size != chunk.total_size
        || rec.chunk_count != chunk.chunk_count
        || rec.manifest_hash != chunk.manifest_hash
    {
        return Err("chunk_meta_mismatch");
    }
    let expected = rec.chunks_hex.len();
    if chunk.chunk_index != expected {
        return Err("chunk_order_invalid");
    }
    rec.chunk_hashes.push(chunk.chunk_hash.clone());
    rec.chunks_hex.push(hex_encode(&chunk.chunk));
    rec.state = "RECEIVING".to_string();
    timeline_store_save(&store).map_err(|_| "timeline_unavailable")?;
    let idx_s = chunk.chunk_index.to_string();
    emit_marker(
        "file_xfer_chunk",
        None,
        &[
            ("id", chunk.file_id.as_str()),
            ("idx", idx_s.as_str()),
            ("ok", "true"),
        ],
    );
    Ok(())
}

fn file_transfer_handle_manifest(
    ctx: &ReceivePullCtx<'_>,
    manifest: FileTransferManifestPayload,
) -> Result<(), &'static str> {
    let key = file_xfer_store_key(ctx.from, manifest.file_id.as_str());
    let mut store = timeline_store_load().map_err(|_| "timeline_unavailable")?;
    let rec = store
        .file_transfers
        .get_mut(&key)
        .ok_or("manifest_missing_chunks")?;
    if rec.state == "FAILED" || rec.state == "VERIFIED" {
        return Err("state_invalid_transition");
    }
    if rec.total_size != manifest.total_size
        || rec.chunk_count != manifest.chunk_count
        || rec.filename != manifest.filename
    {
        return Err("manifest_meta_mismatch");
    }
    if rec.chunks_hex.len() != rec.chunk_count {
        return Err("manifest_missing_chunks");
    }
    if manifest.chunk_hashes.len() != rec.chunk_count {
        return Err("manifest_chunk_count_mismatch");
    }
    let expected_manifest = file_xfer_manifest_hash(
        ctx.from,
        manifest.file_id.as_str(),
        manifest.total_size,
        manifest.chunk_count,
        manifest.chunk_hashes.as_slice(),
    );
    if expected_manifest != manifest.manifest_hash || rec.manifest_hash != manifest.manifest_hash {
        return Err("manifest_mismatch");
    }
    if rec.chunk_hashes != manifest.chunk_hashes {
        return Err("manifest_mismatch");
    }
    let mut reconstructed = Vec::new();
    for (idx, chunk_hex) in rec.chunks_hex.iter().enumerate() {
        let chunk = hex_decode(chunk_hex).map_err(|_| "chunk_decode_failed")?;
        if file_xfer_chunk_hash(&chunk) != manifest.chunk_hashes[idx] {
            return Err("chunk_hash_invalid");
        }
        reconstructed.extend_from_slice(&chunk);
    }
    if reconstructed.len() != manifest.total_size {
        return Err("manifest_size_mismatch");
    }
    rec.state = "VERIFIED".to_string();
    timeline_store_save(&store).map_err(|_| "timeline_unavailable")?;
    timeline_append_entry(
        ctx.from,
        "in",
        reconstructed.len(),
        "file",
        MessageState::Received,
        Some(manifest.file_id.as_str()),
    )?;
    emit_marker(
        "file_xfer_manifest",
        None,
        &[("id", manifest.file_id.as_str()), ("ok", "true")],
    );
    emit_marker(
        "file_xfer_complete",
        None,
        &[("id", manifest.file_id.as_str()), ("ok", "true")],
    );
    Ok(())
}

fn build_delivered_ack(msg_id: &str) -> Vec<u8> {
    let ack = ReceiptControlPayload {
        v: 1,
        t: "ack".to_string(),
        kind: "delivered".to_string(),
        msg_id: msg_id.to_string(),
        body: None,
    };
    serde_json::to_vec(&ack).unwrap_or_else(|_| print_error_marker("receipt_encode_failed"))
}

fn send_delivered_receipt_ack(relay: &str, to: &str, msg_id: &str) -> Result<(), &'static str> {
    let payload = build_delivered_ack(msg_id);
    let pad_cfg = Some(MetaPadConfig {
        target_len: None,
        profile: Some(EnvelopeProfile::Standard),
        label: Some("small"),
    });
    let pack = qsp_pack(to, &payload, pad_cfg, None)?;
    relay_inbox_push(relay, to, &pack.envelope)?;
    qsp_session_store(to, &pack.next_state).map_err(|_| "qsp_session_store_failed")?;
    Ok(())
}

fn meta_pad_config_from_args(
    pad_to: Option<usize>,
    pad_bucket: Option<MetaPadBucket>,
    meta_seed: Option<u64>,
) -> Result<Option<MetaPadConfig>, &'static str> {
    if pad_to.is_none() && pad_bucket.is_none() {
        return Ok(None);
    }
    if pad_to.is_some() && pad_bucket.is_some() {
        return Err("meta_pad_conflict");
    }
    if let Some(len) = pad_to {
        if len == 0 || len > PAD_TO_MAX {
            return Err("meta_pad_invalid");
        }
        return Ok(Some(MetaPadConfig {
            target_len: Some(len),
            profile: None,
            label: Some("pad_to"),
        }));
    }
    let bucket = pad_bucket.unwrap_or(MetaPadBucket::Standard);
    let profile = match bucket {
        MetaPadBucket::Standard => EnvelopeProfile::Standard,
        MetaPadBucket::Enhanced => EnvelopeProfile::Enhanced,
        MetaPadBucket::Private => EnvelopeProfile::Private,
        MetaPadBucket::Auto => {
            let seed = meta_seed.ok_or("meta_seed_required")?;
            let mut rng = RelayRng::new(seed ^ 0x51d2a9f1);
            match rng.next_u32() % 3 {
                0 => EnvelopeProfile::Standard,
                1 => EnvelopeProfile::Enhanced,
                _ => EnvelopeProfile::Private,
            }
        }
    };
    let label = match bucket {
        MetaPadBucket::Standard => "standard",
        MetaPadBucket::Enhanced => "enhanced",
        MetaPadBucket::Private => "private",
        MetaPadBucket::Auto => "auto",
    };
    Ok(Some(MetaPadConfig {
        target_len: None,
        profile: Some(profile),
        label: Some(label),
    }))
}

fn map_qsp_recv_err(err: &RefimplError) -> &'static str {
    let s = err.to_string();
    if s.contains("REJECT_S2_REPLAY") {
        "qsp_replay_reject"
    } else if s.contains("REJECT_S2_OOO_BOUNDS") {
        "qsp_ooo_reject"
    } else if s.contains("REJECT_S2_BODY_AUTH_FAIL") {
        "qsp_auth_failed"
    } else if s.contains("REJECT_S2_HDR_AUTH_FAIL") {
        "qsp_hdr_auth_failed"
    } else {
        "qsp_verify_failed"
    }
}

fn qsp_pack(
    channel: &str,
    plaintext: &[u8],
    pad_cfg: Option<MetaPadConfig>,
    meta_seed: Option<u64>,
) -> Result<QspPackOutcome, &'static str> {
    let st = qsp_session_for_channel(channel)?;
    let c = StdCrypto;
    let outcome = send_wire_canon(&c, &c, &c, st.send.clone(), 0, plaintext)
        .map_err(|_| "qsp_pack_failed")?;
    let mut env = Envelope {
        env_version: QSE_ENV_VERSION_V1,
        flags: 0,
        route_token: Vec::new(),
        timestamp_bucket: 0,
        payload: outcome.wire,
        padding: Vec::new(),
    };
    let mut pad_label = None;
    let mut encoded_len = env.encode().len();
    let min_len = EnvelopeProfile::Standard.min_size_bytes();
    if encoded_len < min_len {
        let need = min_len - encoded_len;
        let mut seed_bytes = Vec::new();
        if let Some(seed) = meta_seed {
            seed_bytes.extend_from_slice(&seed.to_le_bytes());
        }
        let pad = c.kmac256(&env.payload, "QSC.QSP.PAD", &seed_bytes, need);
        env = env
            .pad_to_profile(EnvelopeProfile::Standard, &pad)
            .map_err(|_| "qsp_pack_failed")?;
        encoded_len = env.encode().len();
    }
    if let Some(cfg) = pad_cfg {
        if let Some(target) = cfg.target_len {
            if target < encoded_len {
                return Err("meta_pad_too_small");
            }
            let need = target - encoded_len;
            if need > 0 {
                let mut seed_bytes = Vec::new();
                if let Some(seed) = meta_seed {
                    seed_bytes.extend_from_slice(&seed.to_le_bytes());
                }
                let pad = c.kmac256(&env.payload, "QSC.META.PAD", &seed_bytes, need);
                env.padding.extend_from_slice(&pad);
                encoded_len = env.encode().len();
            }
            pad_label = cfg.label;
        } else if let Some(profile) = cfg.profile {
            let min_len = profile.min_size_bytes();
            if encoded_len < min_len {
                let need = min_len - encoded_len;
                let mut seed_bytes = Vec::new();
                if let Some(seed) = meta_seed {
                    seed_bytes.extend_from_slice(&seed.to_le_bytes());
                }
                let pad = c.kmac256(&env.payload, "QSC.META.PAD", &seed_bytes, need);
                env = env
                    .pad_to_profile(profile, &pad)
                    .map_err(|_| "qsp_pack_failed")?;
                encoded_len = env.encode().len();
            }
            pad_label = cfg.label;
        }
    }
    let mut next_state = st.clone();
    next_state.send = outcome.state;
    Ok(QspPackOutcome {
        envelope: env.encode(),
        next_state,
        msg_idx: outcome.n,
        ck_idx: outcome.n,
        padded_len: encoded_len,
        pad_label,
    })
}

fn qsp_unpack(channel: &str, envelope_bytes: &[u8]) -> Result<QspUnpackOutcome, &'static str> {
    let env = Envelope::decode(envelope_bytes).map_err(|_| "qsp_env_decode_failed")?;
    let st = qsp_session_for_channel(channel)?;
    let c = StdCrypto;
    let outcome = recv_wire_canon(&c, &c, &c, st.recv.clone(), &env.payload, None, None)
        .map_err(|e| map_qsp_recv_err(&e))?;
    let mut next_state = st.clone();
    let prev_len = next_state.recv.mkskipped.len();
    next_state.recv = outcome.state;
    let skip_delta = next_state.recv.mkskipped.len().saturating_sub(prev_len);
    let evicted = bound_mkskipped(&mut next_state.recv);
    Ok(QspUnpackOutcome {
        plaintext: outcome.plaintext,
        next_state,
        msg_idx: outcome.n,
        skip_delta,
        evicted,
    })
}

const HS_MAGIC: &[u8; 4] = b"QHSM";
const HS_VERSION: u16 = 1;
const HS_TYPE_INIT: u8 = 1;
const HS_TYPE_RESP: u8 = 2;
const HS_TYPE_CONFIRM: u8 = 3;

fn hs_kem_pk_len() -> usize {
    pqcrypto_kyber::kyber768::public_key_bytes()
}

fn hs_kem_ct_len() -> usize {
    pqcrypto_kyber::kyber768::ciphertext_bytes()
}

fn hs_kem_keypair() -> (Vec<u8>, Vec<u8>) {
    use pqcrypto_kyber::kyber768;
    use pqcrypto_traits::kem::{PublicKey as _, SecretKey as _};
    let (pk, sk) = kyber768::keypair();
    (pk.as_bytes().to_vec(), sk.as_bytes().to_vec())
}

fn hs_sig_pk_len() -> usize {
    pqcrypto_dilithium::dilithium3::public_key_bytes()
}

fn hs_sig_sig_len() -> usize {
    pqcrypto_dilithium::dilithium3::signature_bytes()
}

fn hs_sig_keypair() -> (Vec<u8>, Vec<u8>) {
    use pqcrypto_dilithium::dilithium3;
    use pqcrypto_traits::sign::{PublicKey as _, SecretKey as _};
    let (pk, sk) = dilithium3::keypair();
    (pk.as_bytes().to_vec(), sk.as_bytes().to_vec())
}

fn hs_default_role() -> String {
    "initiator".to_string()
}

#[derive(Clone, Debug)]
struct HsInit {
    session_id: [u8; 16],
    kem_pk: Vec<u8>,
    sig_pk: Vec<u8>,
}

#[derive(Clone, Debug)]
struct HsResp {
    session_id: [u8; 16],
    kem_ct: Vec<u8>,
    mac: [u8; 32],
    sig_pk: Vec<u8>,
    sig: Vec<u8>,
}

#[derive(Clone, Debug)]
struct HsConfirm {
    session_id: [u8; 16],
    mac: [u8; 32],
    sig: Vec<u8>,
}

#[derive(Serialize, Deserialize)]
struct IdentityKeypair {
    kem_pk: Vec<u8>,
    kem_sk: Vec<u8>,
    sig_pk: Vec<u8>,
    sig_sk: Vec<u8>,
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct IdentityPublicRecord {
    kem_pk: Vec<u8>,
    #[serde(default)]
    sig_pk: Vec<u8>,
}

#[derive(Serialize, Deserialize)]
struct IdentityLegacyRecord {
    kem_pk: Vec<u8>,
    kem_sk: Vec<u8>,
}

#[derive(Serialize, Deserialize, Clone)]
struct HandshakePending {
    self_label: String,
    peer: String,
    session_id: [u8; 16],
    kem_sk: Vec<u8>,
    kem_pk: Vec<u8>,
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
}

fn hs_encode_init(msg: &HsInit) -> Vec<u8> {
    let pk_len = hs_kem_pk_len();
    let sig_pk_len = hs_sig_pk_len();
    if msg.kem_pk.len() != pk_len || msg.sig_pk.len() != sig_pk_len {
        return Vec::new();
    }
    let mut out = Vec::with_capacity(4 + 2 + 1 + 16 + pk_len + sig_pk_len);
    out.extend_from_slice(HS_MAGIC);
    out.extend_from_slice(&HS_VERSION.to_be_bytes());
    out.push(HS_TYPE_INIT);
    out.extend_from_slice(&msg.session_id);
    out.extend_from_slice(&msg.kem_pk);
    out.extend_from_slice(&msg.sig_pk);
    out
}

fn hs_decode_init(bytes: &[u8]) -> Result<HsInit, &'static str> {
    let pk_len = hs_kem_pk_len();
    let sig_pk_len = hs_sig_pk_len();
    if bytes.len() != 4 + 2 + 1 + 16 + pk_len + sig_pk_len {
        return Err("handshake_init_len");
    }
    if &bytes[0..4] != HS_MAGIC {
        return Err("handshake_magic");
    }
    let ver = u16::from_be_bytes([bytes[4], bytes[5]]);
    if ver != HS_VERSION {
        return Err("handshake_version");
    }
    if bytes[6] != HS_TYPE_INIT {
        return Err("handshake_type");
    }
    let mut sid = [0u8; 16];
    sid.copy_from_slice(&bytes[7..23]);
    let kem_pk = bytes[23..(23 + pk_len)].to_vec();
    let sig_pk = bytes[(23 + pk_len)..(23 + pk_len + sig_pk_len)].to_vec();
    Ok(HsInit {
        session_id: sid,
        kem_pk,
        sig_pk,
    })
}

fn hs_encode_resp(msg: &HsResp) -> Vec<u8> {
    let ct_len = hs_kem_ct_len();
    let sig_pk_len = hs_sig_pk_len();
    let sig_len = hs_sig_sig_len();
    if msg.kem_ct.len() != ct_len || msg.sig_pk.len() != sig_pk_len || msg.sig.len() != sig_len {
        return Vec::new();
    }
    let mut out = Vec::with_capacity(4 + 2 + 1 + 16 + ct_len + 32 + sig_pk_len + sig_len);
    out.extend_from_slice(HS_MAGIC);
    out.extend_from_slice(&HS_VERSION.to_be_bytes());
    out.push(HS_TYPE_RESP);
    out.extend_from_slice(&msg.session_id);
    out.extend_from_slice(&msg.kem_ct);
    out.extend_from_slice(&msg.mac);
    out.extend_from_slice(&msg.sig_pk);
    out.extend_from_slice(&msg.sig);
    out
}

fn hs_decode_resp(bytes: &[u8]) -> Result<HsResp, &'static str> {
    let ct_len = hs_kem_ct_len();
    let sig_pk_len = hs_sig_pk_len();
    let sig_len = hs_sig_sig_len();
    if bytes.len() != 4 + 2 + 1 + 16 + ct_len + 32 + sig_pk_len + sig_len {
        return Err("handshake_resp_len");
    }
    if &bytes[0..4] != HS_MAGIC {
        return Err("handshake_magic");
    }
    let ver = u16::from_be_bytes([bytes[4], bytes[5]]);
    if ver != HS_VERSION {
        return Err("handshake_version");
    }
    if bytes[6] != HS_TYPE_RESP {
        return Err("handshake_type");
    }
    let mut sid = [0u8; 16];
    sid.copy_from_slice(&bytes[7..23]);
    let kem_ct = bytes[23..(23 + ct_len)].to_vec();
    let mut mac = [0u8; 32];
    let mac_off = 23 + ct_len;
    mac.copy_from_slice(&bytes[mac_off..(mac_off + 32)]);
    let sig_pk_off = mac_off + 32;
    let sig_off = sig_pk_off + sig_pk_len;
    let sig_pk = bytes[sig_pk_off..sig_off].to_vec();
    let sig = bytes[sig_off..(sig_off + sig_len)].to_vec();
    Ok(HsResp {
        session_id: sid,
        kem_ct,
        mac,
        sig_pk,
        sig,
    })
}

fn hs_encode_confirm(msg: &HsConfirm) -> Vec<u8> {
    let sig_len = hs_sig_sig_len();
    if msg.sig.len() != sig_len {
        return Vec::new();
    }
    let mut out = Vec::with_capacity(4 + 2 + 1 + 16 + 32 + sig_len);
    out.extend_from_slice(HS_MAGIC);
    out.extend_from_slice(&HS_VERSION.to_be_bytes());
    out.push(HS_TYPE_CONFIRM);
    out.extend_from_slice(&msg.session_id);
    out.extend_from_slice(&msg.mac);
    out.extend_from_slice(&msg.sig);
    out
}

fn hs_decode_confirm(bytes: &[u8]) -> Result<HsConfirm, &'static str> {
    let sig_len = hs_sig_sig_len();
    if bytes.len() != 4 + 2 + 1 + 16 + 32 + sig_len {
        return Err("handshake_confirm_len");
    }
    if &bytes[0..4] != HS_MAGIC {
        return Err("handshake_magic");
    }
    let ver = u16::from_be_bytes([bytes[4], bytes[5]]);
    if ver != HS_VERSION {
        return Err("handshake_version");
    }
    if bytes[6] != HS_TYPE_CONFIRM {
        return Err("handshake_type");
    }
    let mut sid = [0u8; 16];
    sid.copy_from_slice(&bytes[7..23]);
    let mut mac = [0u8; 32];
    mac.copy_from_slice(&bytes[23..55]);
    let sig = bytes[55..(55 + sig_len)].to_vec();
    Ok(HsConfirm {
        session_id: sid,
        mac,
        sig,
    })
}

const IDENTITY_DIR: &str = "identities";
const IDENTITY_FP_PREFIX: &str = "QSCFP-";

fn identities_dir(dir: &Path) -> PathBuf {
    dir.join(IDENTITY_DIR)
}

fn identity_self_path(dir: &Path, self_label: &str) -> PathBuf {
    identities_dir(dir).join(format!("self_{}.json", self_label))
}

fn hex_encode(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut out = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        out.push(HEX[(b >> 4) as usize] as char);
        out.push(HEX[(b & 0x0f) as usize] as char);
    }
    out
}

fn identity_fingerprint_from_pk(pk: &[u8]) -> String {
    let c = StdCrypto;
    let hash = c.sha512(pk);
    let fp = &hash[..16];
    format!("{}{}", IDENTITY_FP_PREFIX, hex_encode(fp))
}

fn hex_decode(s: &str) -> Result<Vec<u8>, ErrorCode> {
    if !s.len().is_multiple_of(2) {
        return Err(ErrorCode::ParseFailed);
    }
    let mut out = Vec::with_capacity(s.len() / 2);
    let bytes = s.as_bytes();
    let mut i = 0usize;
    while i < bytes.len() {
        let hi = hex_nibble(bytes[i]).ok_or(ErrorCode::ParseFailed)?;
        let lo = hex_nibble(bytes[i + 1]).ok_or(ErrorCode::ParseFailed)?;
        out.push((hi << 4) | lo);
        i += 2;
    }
    Ok(out)
}

fn hex_nibble(c: u8) -> Option<u8> {
    match c {
        b'0'..=b'9' => Some(c - b'0'),
        b'a'..=b'f' => Some(c - b'a' + 10),
        b'A'..=b'F' => Some(c - b'A' + 10),
        _ => None,
    }
}

fn identity_secret_name(self_label: &str) -> String {
    format!("identity.kem_sk.{}", self_label)
}

fn identity_sig_secret_name(self_label: &str) -> String {
    format!("identity.sig_sk.{}", self_label)
}

fn identity_secret_store(self_label: &str, kem_sk: &[u8]) -> Result<(), ErrorCode> {
    let key = identity_secret_name(self_label);
    let secret = hex_encode(kem_sk);
    if let Err(e) = vault::secret_set(&key, &secret) {
        let reason = match e {
            "vault_missing" => "vault_missing",
            "vault_locked" => "vault_locked",
            _ => "vault_write_failed",
        };
        emit_marker(
            "identity_secret_unavailable",
            Some(e),
            &[("reason", reason)],
        );
        return Err(match e {
            "vault_missing" => ErrorCode::IdentitySecretUnavailable,
            "vault_locked" => ErrorCode::IdentitySecretUnavailable,
            _ => ErrorCode::IoWriteFailed,
        });
    }
    emit_marker(
        "identity_secret_store",
        None,
        &[("ok", "true"), ("method", "vault")],
    );
    Ok(())
}

fn identity_secret_load(self_label: &str) -> Result<Vec<u8>, ErrorCode> {
    let key = identity_secret_name(self_label);
    let Some(secret) = vault::secret_get(&key).map_err(|e| {
        let reason = match e {
            "vault_missing" => "vault_missing",
            "vault_locked" => "vault_locked",
            _ => "vault_read_failed",
        };
        emit_marker(
            "identity_secret_unavailable",
            Some(e),
            &[("reason", reason)],
        );
        match e {
            "vault_missing" => ErrorCode::IdentitySecretUnavailable,
            "vault_locked" => ErrorCode::IdentitySecretUnavailable,
            _ => ErrorCode::IoReadFailed,
        }
    })?
    else {
        emit_marker(
            "identity_secret_unavailable",
            Some("identity_secret_unavailable"),
            &[("reason", "missing_secret")],
        );
        return Err(ErrorCode::IdentitySecretUnavailable);
    };
    hex_decode(&secret)
}

fn identity_sig_secret_store(self_label: &str, sig_sk: &[u8]) -> Result<(), ErrorCode> {
    let key = identity_sig_secret_name(self_label);
    let secret = hex_encode(sig_sk);
    if let Err(e) = vault::secret_set(&key, &secret) {
        let reason = match e {
            "vault_missing" => "vault_missing",
            "vault_locked" => "vault_locked",
            _ => "vault_write_failed",
        };
        emit_marker(
            "identity_secret_unavailable",
            Some(e),
            &[("reason", reason)],
        );
        return Err(match e {
            "vault_missing" | "vault_locked" => ErrorCode::IdentitySecretUnavailable,
            _ => ErrorCode::IoWriteFailed,
        });
    }
    emit_marker(
        "identity_secret_store",
        None,
        &[("ok", "true"), ("method", "vault")],
    );
    Ok(())
}

fn identity_sig_secret_load(self_label: &str) -> Result<Vec<u8>, ErrorCode> {
    let key = identity_sig_secret_name(self_label);
    let Some(secret) = vault::secret_get(&key).map_err(|e| {
        let reason = match e {
            "vault_missing" => "vault_missing",
            "vault_locked" => "vault_locked",
            _ => "vault_read_failed",
        };
        emit_marker(
            "identity_secret_unavailable",
            Some(e),
            &[("reason", reason)],
        );
        match e {
            "vault_missing" | "vault_locked" => ErrorCode::IdentitySecretUnavailable,
            _ => ErrorCode::IoReadFailed,
        }
    })?
    else {
        emit_marker(
            "identity_secret_unavailable",
            Some("identity_secret_unavailable"),
            &[("reason", "missing_secret")],
        );
        return Err(ErrorCode::IdentitySecretUnavailable);
    };
    hex_decode(&secret)
}

fn identity_write_public_record(
    self_label: &str,
    kem_pk: &[u8],
    sig_pk: &[u8],
) -> Result<(), ErrorCode> {
    if !channel_label_ok(self_label) {
        return Err(ErrorCode::ParseFailed);
    }
    let (dir, source) = config_dir()?;
    let identities = identities_dir(&dir);
    ensure_dir_secure(&identities, source)?;
    let path = identity_self_path(&dir, self_label);
    let rec = IdentityPublicRecord {
        kem_pk: kem_pk.to_vec(),
        sig_pk: sig_pk.to_vec(),
    };
    let bytes = serde_json::to_vec(&rec).map_err(|_| ErrorCode::ParseFailed)?;
    write_atomic(&path, &bytes, source)?;
    Ok(())
}

fn identity_migrate_legacy(
    self_label: &str,
    source: ConfigSource,
    path: &Path,
    legacy: IdentityLegacyRecord,
) -> Result<IdentityKeypair, ErrorCode> {
    let (sig_pk, sig_sk) = hs_sig_keypair();
    if let Err(e) = identity_secret_store(self_label, &legacy.kem_sk) {
        emit_marker(
            "identity_secret_migrate",
            Some(e.as_str()),
            &[
                ("ok", "false"),
                ("action", "skipped"),
                ("reason", "vault_unavailable"),
            ],
        );
        return Err(e);
    }
    if let Err(e) = identity_sig_secret_store(self_label, &sig_sk) {
        emit_marker(
            "identity_secret_migrate",
            Some(e.as_str()),
            &[
                ("ok", "false"),
                ("action", "skipped"),
                ("reason", "vault_unavailable"),
            ],
        );
        return Err(e);
    }
    let rec = IdentityPublicRecord {
        kem_pk: legacy.kem_pk.clone(),
        sig_pk: sig_pk.clone(),
    };
    let bytes = serde_json::to_vec(&rec).map_err(|_| ErrorCode::ParseFailed)?;
    write_atomic(path, &bytes, source)?;
    emit_marker(
        "identity_secret_migrate",
        None,
        &[
            ("ok", "true"),
            ("action", "imported"),
            ("reason", "legacy_plaintext"),
        ],
    );
    Ok(IdentityKeypair {
        kem_pk: legacy.kem_pk,
        kem_sk: legacy.kem_sk,
        sig_pk,
        sig_sk,
    })
}

fn identity_read_self_kem_keypair(self_label: &str) -> Result<Option<IdentityKeypair>, ErrorCode> {
    if !channel_label_ok(self_label) {
        return Err(ErrorCode::ParseFailed);
    }
    let (dir, source) = config_dir()?;
    let identities = identities_dir(&dir);
    ensure_dir_secure(&identities, source)?;
    let path = identity_self_path(&dir, self_label);
    if !path.exists() {
        return Ok(None);
    }
    enforce_safe_parents(&path, source)?;
    let bytes = fs::read(&path).map_err(|_| ErrorCode::IoReadFailed)?;
    if let Ok(rec) = serde_json::from_slice::<IdentityPublicRecord>(&bytes) {
        let kem_sk = identity_secret_load(self_label)?;
        let (sig_pk, sig_sk) = if rec.sig_pk.is_empty() {
            let (sig_pk, sig_sk) = hs_sig_keypair();
            identity_sig_secret_store(self_label, &sig_sk)?;
            identity_write_public_record(self_label, &rec.kem_pk, &sig_pk)?;
            (sig_pk, sig_sk)
        } else {
            (rec.sig_pk.clone(), identity_sig_secret_load(self_label)?)
        };
        return Ok(Some(IdentityKeypair {
            kem_pk: rec.kem_pk,
            kem_sk,
            sig_pk,
            sig_sk,
        }));
    }
    if let Ok(legacy) = serde_json::from_slice::<IdentityLegacyRecord>(&bytes) {
        let migrated = identity_migrate_legacy(self_label, source, &path, legacy)?;
        return Ok(Some(migrated));
    }
    Err(ErrorCode::ParseFailed)
}

fn identity_read_self_public(self_label: &str) -> Result<Option<IdentityPublicRecord>, ErrorCode> {
    if !channel_label_ok(self_label) {
        return Err(ErrorCode::ParseFailed);
    }
    let (dir, source) = config_dir()?;
    let identities = identities_dir(&dir);
    ensure_dir_secure(&identities, source)?;
    let path = identity_self_path(&dir, self_label);
    if !path.exists() {
        return Ok(None);
    }
    enforce_safe_parents(&path, source)?;
    let bytes = fs::read(&path).map_err(|_| ErrorCode::IoReadFailed)?;
    if let Ok(rec) = serde_json::from_slice::<IdentityPublicRecord>(&bytes) {
        return Ok(Some(rec));
    }
    if let Ok(legacy) = serde_json::from_slice::<IdentityLegacyRecord>(&bytes) {
        return Ok(Some(IdentityPublicRecord {
            kem_pk: legacy.kem_pk,
            sig_pk: Vec::new(),
        }));
    }
    Err(ErrorCode::ParseFailed)
}

fn identity_self_kem_keypair(self_label: &str) -> Result<IdentityKeypair, ErrorCode> {
    if !channel_label_ok(self_label) {
        return Err(ErrorCode::ParseFailed);
    }
    let (dir, source) = config_dir()?;
    let identities = identities_dir(&dir);
    ensure_dir_secure(&identities, source)?;
    let path = identity_self_path(&dir, self_label);
    if path.exists() {
        enforce_safe_parents(&path, source)?;
        if let Some(kp) = identity_read_self_kem_keypair(self_label)? {
            return Ok(kp);
        }
        return Err(ErrorCode::ParseFailed);
    }
    let (kem_pk, kem_sk) = hs_kem_keypair();
    let (sig_pk, sig_sk) = hs_sig_keypair();
    identity_secret_store(self_label, &kem_sk)?;
    identity_sig_secret_store(self_label, &sig_sk)?;
    identity_write_public_record(self_label, &kem_pk, &sig_pk)?;
    Ok(IdentityKeypair {
        kem_pk,
        kem_sk,
        sig_pk,
        sig_sk,
    })
}

fn identity_self_fingerprint(self_label: &str) -> Result<String, ErrorCode> {
    match identity_read_self_public(self_label)? {
        Some(rec) => Ok(identity_fingerprint_from_pk(&rec.kem_pk)),
        None => Ok("untrusted".to_string()),
    }
}

fn contacts_store_load() -> Result<ContactsStore, ErrorCode> {
    match vault::secret_get(CONTACTS_SECRET_KEY) {
        Ok(None) => Ok(ContactsStore::default()),
        Ok(Some(v)) => {
            serde_json::from_str::<ContactsStore>(&v).map_err(|_| ErrorCode::ParseFailed)
        }
        Err("vault_missing" | "vault_locked") => Err(ErrorCode::IdentitySecretUnavailable),
        Err(_) => Err(ErrorCode::IoReadFailed),
    }
}

fn contacts_store_save(store: &ContactsStore) -> Result<(), ErrorCode> {
    let json = serde_json::to_string(store).map_err(|_| ErrorCode::ParseFailed)?;
    match vault::secret_set(CONTACTS_SECRET_KEY, &json) {
        Ok(()) => Ok(()),
        Err("vault_missing" | "vault_locked") => Err(ErrorCode::IdentitySecretUnavailable),
        Err(_) => Err(ErrorCode::IoWriteFailed),
    }
}

fn contacts_entry_read(label: &str) -> Result<Option<ContactRecord>, ErrorCode> {
    if !channel_label_ok(label) {
        return Err(ErrorCode::ParseFailed);
    }
    let store = contacts_store_load()?;
    Ok(store.peers.get(label).cloned())
}

fn contacts_entry_upsert(label: &str, rec: ContactRecord) -> Result<(), ErrorCode> {
    if !channel_label_ok(label) {
        return Err(ErrorCode::ParseFailed);
    }
    let mut store = contacts_store_load()?;
    store.peers.insert(label.to_string(), rec);
    contacts_store_save(&store)
}

fn contacts_set_blocked(label: &str, blocked: bool) -> Result<bool, ErrorCode> {
    if !channel_label_ok(label) {
        return Err(ErrorCode::ParseFailed);
    }
    let mut store = contacts_store_load()?;
    let Some(rec) = store.peers.get_mut(label) else {
        return Ok(false);
    };
    rec.blocked = blocked;
    contacts_store_save(&store)?;
    Ok(true)
}

fn contacts_list_entries() -> Result<Vec<(String, ContactRecord)>, ErrorCode> {
    let store = contacts_store_load()?;
    Ok(store.peers.into_iter().collect())
}

fn contacts_list_labels() -> Vec<String> {
    let mut out = match contacts_list_entries() {
        Ok(v) => v.into_iter().map(|(k, _)| k).collect::<Vec<_>>(),
        Err(_) => Vec::new(),
    };
    out.sort();
    out
}

fn contact_display_line(label: &str) -> String {
    match contacts_entry_read(label) {
        Ok(Some(rec)) => format!(
            "{} state={} blocked={} mismatch=false",
            label,
            contact_state(Some(&rec)),
            bool_str(rec.blocked)
        ),
        Ok(None) => format!("{} state=unknown blocked=false mismatch=false", label),
        Err(_) => format!("{} state=unknown blocked=false mismatch=false", label),
    }
}

fn contact_state(rec: Option<&ContactRecord>) -> &'static str {
    match rec {
        Some(v) if v.status == "verified" => "verified",
        Some(_) => "pinned",
        None => "unknown",
    }
}

fn contact_blocked(label: &str) -> Result<bool, ErrorCode> {
    Ok(contacts_entry_read(label)?
        .map(|v| v.blocked)
        .unwrap_or(false))
}

fn enforce_peer_not_blocked(label: &str) -> Result<(), &'static str> {
    match contact_blocked(label) {
        Ok(true) => {
            emit_marker(
                "contacts_refuse",
                None,
                &[("label", label), ("reason", "peer_blocked")],
            );
            Err("peer_blocked")
        }
        Ok(false) => Ok(()),
        // Missing/locked contacts store means no explicit block policy is available.
        Err(ErrorCode::IdentitySecretUnavailable) => Ok(()),
        Err(_) => Err("contacts_store_invalid"),
    }
}

fn emit_peer_mismatch(peer: &str, pinned_fp: &str, seen_fp: &str) {
    emit_marker(
        "identity_mismatch",
        None,
        &[
            ("peer", peer),
            ("pinned_fp", pinned_fp),
            ("seen_fp", seen_fp),
        ],
    );
    emit_marker("error", Some("peer_mismatch"), &[("peer", peer)]);
}

fn identity_read_pin(peer: &str) -> Result<Option<String>, ErrorCode> {
    Ok(contacts_entry_read(peer)?.map(|v| v.fp))
}

fn identity_read_sig_pin(peer: &str) -> Result<Option<String>, ErrorCode> {
    Ok(contacts_entry_read(peer)?.and_then(|v| v.sig_fp))
}

fn contacts_add(label: &str, fp: &str, verify: bool) {
    if !require_unlocked("contacts_add") {
        return;
    }
    let status = if verify { "verified" } else { "pinned" };
    let rec = ContactRecord {
        fp: fp.to_string(),
        status: status.to_string(),
        blocked: false,
        seen_at: None,
        sig_fp: None,
    };
    if contacts_entry_upsert(label, rec).is_err() {
        print_error_marker("contacts_store_unavailable");
    }
    emit_marker(
        "contacts_add",
        None,
        &[("ok", "true"), ("label", label), ("status", status)],
    );
    println!("contact={} status={}", label, status);
}

fn contacts_show(label: &str) {
    if !require_unlocked("contacts_show") {
        return;
    }
    let rec = contacts_entry_read(label)
        .unwrap_or_else(|_| print_error_marker("contacts_store_unavailable"));
    let state = contact_state(rec.as_ref());
    let blocked = bool_str(rec.as_ref().map(|v| v.blocked).unwrap_or(false));
    emit_marker(
        "contacts_show",
        None,
        &[("label", label), ("state", state), ("blocked", blocked)],
    );
    if let Some(v) = rec {
        println!(
            "label={} fp={} state={} blocked={}",
            label, v.fp, state, blocked
        );
    } else {
        println!("label={} state=unknown blocked=false", label);
    }
}

fn contacts_list() {
    if !require_unlocked("contacts_list") {
        return;
    }
    let mut entries = contacts_list_entries()
        .unwrap_or_else(|_| print_error_marker("contacts_store_unavailable"));
    entries.sort_by(|a, b| a.0.cmp(&b.0));
    let count_s = entries.len().to_string();
    emit_marker("contacts_list", None, &[("count", count_s.as_str())]);
    for (label, rec) in entries {
        let state = contact_state(Some(&rec));
        let blocked = bool_str(rec.blocked);
        println!(
            "label={} fp={} state={} blocked={}",
            label, rec.fp, state, blocked
        );
    }
}

fn contacts_verify(label: &str, fp: &str, confirm: bool) {
    if !require_unlocked("contacts_verify") {
        return;
    }
    let Some(mut rec) = contacts_entry_read(label)
        .unwrap_or_else(|_| print_error_marker("contacts_store_unavailable"))
    else {
        emit_marker(
            "contacts_verify",
            None,
            &[
                ("ok", "false"),
                ("label", label),
                ("result", "refused"),
                ("reason", "peer_unknown"),
            ],
        );
        print_error_marker("peer_unknown");
    };
    if rec.fp == fp {
        rec.status = "verified".to_string();
        if contacts_entry_upsert(label, rec).is_err() {
            print_error_marker("contacts_store_unavailable");
        }
        emit_marker(
            "contacts_verify",
            None,
            &[
                ("ok", "true"),
                ("label", label),
                ("result", "unchanged"),
                ("reason", "already_pinned"),
            ],
        );
        return;
    }
    if !confirm {
        emit_marker(
            "contacts_verify",
            None,
            &[
                ("ok", "false"),
                ("label", label),
                ("result", "refused"),
                ("reason", "confirm_required"),
            ],
        );
        print_error_marker("verify_requires_confirm");
    }
    rec.fp = fp.to_string();
    rec.status = "verified".to_string();
    if contacts_entry_upsert(label, rec).is_err() {
        print_error_marker("contacts_store_unavailable");
    }
    emit_marker(
        "contacts_verify",
        None,
        &[
            ("ok", "true"),
            ("label", label),
            ("result", "updated"),
            ("reason", "explicit_confirm"),
        ],
    );
}

fn contacts_block(label: &str) {
    if !require_unlocked("contacts_block") {
        return;
    }
    match contacts_set_blocked(label, true) {
        Ok(true) => emit_marker("contacts_block", None, &[("label", label), ("ok", "true")]),
        Ok(false) => print_error_marker("peer_unknown"),
        Err(_) => print_error_marker("contacts_store_unavailable"),
    }
}

fn contacts_unblock(label: &str) {
    if !require_unlocked("contacts_unblock") {
        return;
    }
    match contacts_set_blocked(label, false) {
        Ok(true) => emit_marker(
            "contacts_unblock",
            None,
            &[("label", label), ("ok", "true")],
        ),
        Ok(false) => print_error_marker("peer_unknown"),
        Err(_) => print_error_marker("contacts_store_unavailable"),
    }
}

fn timeline_store_load() -> Result<TimelineStore, &'static str> {
    let mut store = match vault::secret_get(TIMELINE_SECRET_KEY) {
        Ok(None) => Ok(TimelineStore::default()),
        Ok(Some(v)) => serde_json::from_str::<TimelineStore>(&v).map_err(|_| "timeline_tampered"),
        Err("vault_missing" | "vault_locked") => Err("timeline_unavailable"),
        Err(_) => Err("timeline_unavailable"),
    }?;
    if store.next_ts == 0 {
        store.next_ts = 1;
    }
    Ok(store)
}

fn timeline_store_save(store: &TimelineStore) -> Result<(), &'static str> {
    let json = serde_json::to_string(store).map_err(|_| "timeline_unavailable")?;
    match vault::secret_set(TIMELINE_SECRET_KEY, &json) {
        Ok(()) => Ok(()),
        Err("vault_missing" | "vault_locked") => Err("timeline_unavailable"),
        Err(_) => Err("timeline_unavailable"),
    }
}

fn timeline_append_entry(
    peer: &str,
    direction: &str,
    byte_len: usize,
    kind: &str,
    final_state: MessageState,
    forced_id: Option<&str>,
) -> Result<TimelineEntry, &'static str> {
    if !channel_label_ok(peer) {
        return Err("timeline_peer_invalid");
    }
    if let Some(v) = forced_id {
        if v.trim().is_empty() {
            return Err("state_id_invalid");
        }
    }
    message_state_transition_allowed(MessageState::Created, final_state, direction)?;
    let mut store = timeline_store_load()?;
    let ts = store.next_ts;
    store.next_ts = store.next_ts.saturating_add(1);
    let id = forced_id
        .map(|v| v.to_string())
        .unwrap_or_else(|| format!("{}-{}", direction, ts));
    let entry = TimelineEntry {
        id: id.clone(),
        peer: peer.to_string(),
        direction: direction.to_string(),
        byte_len,
        kind: kind.to_string(),
        ts,
        state: final_state.as_str().to_string(),
        status: final_state.as_status().to_string(),
    };
    store
        .peers
        .entry(peer.to_string())
        .or_default()
        .push(entry.clone());
    timeline_store_save(&store)?;
    emit_message_state_transition(id.as_str(), MessageState::Created, final_state);
    Ok(entry)
}

fn timeline_transition_entry_state(
    peer: &str,
    id: &str,
    to: MessageState,
) -> Result<TimelineEntry, &'static str> {
    if !channel_label_ok(peer) {
        return Err("timeline_peer_invalid");
    }
    if id.trim().is_empty() {
        return Err("state_id_invalid");
    }
    let mut store = timeline_store_load()?;
    let Some(entries) = store.peers.get_mut(peer) else {
        return Err("state_unknown");
    };
    let Some(entry) = entries.iter_mut().find(|v| v.id == id) else {
        return Err("state_unknown");
    };
    let from = timeline_entry_state(entry);
    message_state_transition_allowed(from, to, entry.direction.as_str())?;
    entry.state = to.as_str().to_string();
    entry.status = to.as_status().to_string();
    let out = entry.clone();
    timeline_store_save(&store)?;
    emit_message_state_transition(id, from, to);
    Ok(out)
}

fn timeline_entries_for_peer(peer: &str) -> Result<Vec<TimelineEntry>, &'static str> {
    if !channel_label_ok(peer) {
        return Err("timeline_peer_invalid");
    }
    let store = timeline_store_load()?;
    Ok(store.peers.get(peer).cloned().unwrap_or_default())
}

fn timeline_emit_item(entry: &TimelineEntry) {
    let len_s = entry.byte_len.to_string();
    let ts_s = entry.ts.to_string();
    let state = timeline_entry_state(entry);
    emit_marker(
        "timeline_item",
        None,
        &[
            ("id", entry.id.as_str()),
            ("dir", entry.direction.as_str()),
            ("len", len_s.as_str()),
            ("kind", entry.kind.as_str()),
            ("ts", ts_s.as_str()),
            ("state", state.as_str()),
        ],
    );
}

fn timeline_list(peer: &str, limit: Option<usize>) {
    if !require_unlocked("timeline_list") {
        return;
    }
    let mut entries =
        timeline_entries_for_peer(peer).unwrap_or_else(|code| print_error_marker(code));
    entries.sort_by(|a, b| b.ts.cmp(&a.ts).then_with(|| a.id.cmp(&b.id)));
    let take_n = limit.unwrap_or(entries.len()).min(entries.len());
    let count_s = take_n.to_string();
    emit_marker(
        "timeline_list",
        None,
        &[("count", count_s.as_str()), ("peer", peer)],
    );
    for entry in entries.into_iter().take(take_n) {
        timeline_emit_item(&entry);
    }
}

fn timeline_show(peer: &str, id: &str) {
    if !require_unlocked("timeline_show") {
        return;
    }
    let entries = timeline_entries_for_peer(peer).unwrap_or_else(|code| print_error_marker(code));
    let Some(entry) = entries.into_iter().find(|v| v.id == id) else {
        print_error_marker("timeline_item_missing");
    };
    timeline_emit_item(&entry);
}

fn timeline_clear(peer: &str, confirm: bool) {
    if !require_unlocked("timeline_clear") {
        return;
    }
    if !confirm {
        emit_marker(
            "error",
            Some("timeline_clear_confirm_required"),
            &[("peer", peer), ("reason", "explicit_confirm_required")],
        );
        print_error_marker("timeline_clear_confirm_required");
    }
    if !channel_label_ok(peer) {
        print_error_marker("timeline_peer_invalid");
    }
    let mut store = timeline_store_load().unwrap_or_else(|code| print_error_marker(code));
    let removed = store.peers.remove(peer).map(|v| v.len()).unwrap_or(0usize);
    timeline_store_save(&store).unwrap_or_else(|code| print_error_marker(code));
    let removed_s = removed.to_string();
    emit_marker(
        "timeline_clear",
        None,
        &[
            ("ok", "true"),
            ("peer", peer),
            ("removed", removed_s.as_str()),
        ],
    );
}

fn hs_seed_from_env() -> Option<u64> {
    env::var("QSC_HANDSHAKE_SEED")
        .ok()?
        .trim()
        .parse::<u64>()
        .ok()
}

fn hs_rand_bytes(label: &str, len: usize) -> Vec<u8> {
    if let Some(seed) = hs_seed_from_env() {
        let c = StdCrypto;
        let seed_bytes = seed.to_le_bytes();
        let seed_hash = c.sha512(&seed_bytes);
        let mut seed_key = [0u8; 32];
        seed_key.copy_from_slice(&seed_hash[..32]);
        return c.kmac256(&seed_key, label, b"", len);
    }
    let mut out = vec![0u8; len];
    let mut rng = OsRng;
    rng.fill_bytes(&mut out);
    out
}

fn hs_session_id(label: &str) -> [u8; 16] {
    let bytes = hs_rand_bytes(label, 16);
    let mut sid = [0u8; 16];
    sid.copy_from_slice(&bytes[..16]);
    sid
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

fn hs_pq_init_ss(ss_pq: &[u8], session_id: &[u8; 16]) -> [u8; 32] {
    let c = StdCrypto;
    let mut data = Vec::with_capacity(16 + 1);
    data.extend_from_slice(session_id);
    data.push(0x01);
    kmac_out::<32>(&c, ss_pq, "QSC.HS.PQ", &data)
}

fn hs_dh_init_from_pq(pq_init_ss: &[u8; 32], session_id: &[u8; 16]) -> [u8; 32] {
    let c = StdCrypto;
    let mut data = Vec::with_capacity(16 + 1);
    data.extend_from_slice(session_id);
    data.push(0x02);
    kmac_out::<32>(&c, pq_init_ss, "QSC.HS.DHINIT", &data)
}

fn hs_dh_pubs_from_pq(
    pq_init_ss: &[u8; 32],
    session_id: &[u8; 16],
    role_is_a: bool,
) -> ([u8; 32], [u8; 32]) {
    let c = StdCrypto;
    let (self_tag, peer_tag) = if role_is_a {
        ("QSC.HS.DHSELF.A", "QSC.HS.DHSELF.B")
    } else {
        ("QSC.HS.DHSELF.B", "QSC.HS.DHSELF.A")
    };
    let self_pub = kmac_out::<32>(&c, pq_init_ss, self_tag, session_id);
    let peer_pub = kmac_out::<32>(&c, pq_init_ss, peer_tag, session_id);
    (self_pub, peer_pub)
}

fn hs_confirm_key(pq_init_ss: &[u8; 32], session_id: &[u8; 16], th: &[u8; 32]) -> [u8; 32] {
    let c = StdCrypto;
    let mut data = Vec::with_capacity(16 + 32);
    data.extend_from_slice(session_id);
    data.extend_from_slice(th);
    kmac_out::<32>(&c, pq_init_ss, "QSC.HS.CONFIRM", &data)
}

fn hs_confirm_mac(k_confirm: &[u8; 32], session_id: &[u8; 16], th: &[u8; 32]) -> [u8; 32] {
    let c = StdCrypto;
    let mut data = Vec::with_capacity(16 + 32 + 2);
    data.extend_from_slice(session_id);
    data.extend_from_slice(th);
    data.extend_from_slice(b"A2");
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

fn hs_build_session(
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
        true,
    )
}

fn handshake_channel(label: &str) -> Result<String, &'static str> {
    if !channel_label_ok(label) {
        return Err("handshake_channel_invalid");
    }
    Ok(format!("hs-{}", label))
}

fn hs_pending_path(dir: &Path, self_label: &str, peer: &str) -> PathBuf {
    dir.join(format!("handshake_pending_{}_{}.json", self_label, peer))
}

fn hs_pending_load(self_label: &str, peer: &str) -> Result<Option<HandshakePending>, ErrorCode> {
    let (dir, source) = config_dir()?;
    let path = hs_pending_path(&dir, self_label, peer);
    if !path.exists() {
        return Ok(None);
    }
    enforce_safe_parents(&path, source)?;
    let bytes = fs::read(&path).map_err(|_| ErrorCode::IoReadFailed)?;
    let pending: HandshakePending =
        serde_json::from_slice(&bytes).map_err(|_| ErrorCode::ParseFailed)?;
    Ok(Some(pending))
}

fn hs_pending_store(pending: &HandshakePending) -> Result<(), ErrorCode> {
    let (dir, source) = config_dir()?;
    let path = hs_pending_path(&dir, &pending.self_label, &pending.peer);
    enforce_safe_parents(&path, source)?;
    let bytes = serde_json::to_vec(pending).map_err(|_| ErrorCode::IoWriteFailed)?;
    write_atomic(&path, &bytes, source)?;
    Ok(())
}

fn hs_pending_clear(self_label: &str, peer: &str) -> Result<(), ErrorCode> {
    let (dir, source) = config_dir()?;
    let path = hs_pending_path(&dir, self_label, peer);
    enforce_safe_parents(&path, source)?;
    let _ = fs::remove_file(path);
    Ok(())
}

fn handshake_status(peer: Option<&str>) {
    if !require_unlocked("handshake_status") {
        return;
    }
    let peer_label = peer.unwrap_or("peer-0");
    if let Err(code) = enforce_peer_not_blocked(peer_label) {
        print_error_marker(code);
    }
    let (peer_fp, pinned) = identity_peer_status(peer_label);
    let pinned_s = if pinned { "true" } else { "false" };
    match qsp_session_load(peer_label) {
        Ok(Some(_)) => {
            emit_marker(
                "handshake_status",
                None,
                &[
                    ("status", "established"),
                    ("peer", peer_label),
                    ("peer_fp", peer_fp.as_str()),
                    ("pinned", pinned_s),
                ],
            );
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
                ],
            );
        }
    }
}

fn handshake_init(self_label: &str, peer: &str, relay: &str) {
    if !require_unlocked("handshake_init") {
        return;
    }
    if let Err(code) = enforce_peer_not_blocked(peer) {
        print_error_marker(code);
    }
    let channel = match handshake_channel(peer) {
        Ok(v) => v,
        Err(code) => print_error_marker(code),
    };
    let IdentityKeypair {
        kem_pk,
        kem_sk,
        sig_pk,
        sig_sk: _,
    } = identity_self_kem_keypair(self_label).unwrap_or_else(|e| print_error_marker(e.as_str()));
    let sid = hs_session_id("QSC.HS.SID");
    let msg = HsInit {
        session_id: sid,
        kem_pk: kem_pk.clone(),
        sig_pk: sig_pk.clone(),
    };
    let bytes = hs_encode_init(&msg);
    if bytes.is_empty() {
        print_error_marker("handshake_init_encode_failed");
    }
    let pending = HandshakePending {
        self_label: self_label.to_string(),
        peer: peer.to_string(),
        session_id: sid,
        kem_sk,
        kem_pk,
        sig_pk,
        peer_sig_fp: None,
        peer_sig_pk: None,
        peer_fp: None,
        role: "initiator".to_string(),
        confirm_key: None,
        transcript_hash: None,
        pending_session: None,
    };
    hs_pending_store(&pending)
        .unwrap_or_else(|_| print_error_marker("handshake_pending_store_failed"));
    emit_marker(
        "handshake_start",
        None,
        &[("role", "initiator"), ("peer", peer)],
    );
    let size_s = bytes.len().to_string();
    let pk_len_s = hs_kem_pk_len().to_string();
    let sig_pk_len_s = hs_sig_pk_len().to_string();
    emit_marker(
        "handshake_send",
        None,
        &[
            ("msg", "A1"),
            ("size", size_s.as_str()),
            ("kem_pk_len", pk_len_s.as_str()),
            ("sig_pk_len", sig_pk_len_s.as_str()),
        ],
    );
    relay_inbox_push(relay, &channel, &bytes).unwrap_or_else(|code| print_error_marker(code));
}

fn handshake_poll(self_label: &str, peer: &str, relay: &str, max: usize) {
    if !require_unlocked("handshake_poll") {
        return;
    }
    if let Err(code) = enforce_peer_not_blocked(peer) {
        print_error_marker(code);
    }
    let channel = match handshake_channel(self_label) {
        Ok(v) => v,
        Err(code) => print_error_marker(code),
    };
    let items = match relay_inbox_pull(relay, &channel, max) {
        Ok(v) => v,
        Err(code) => {
            emit_marker("handshake_recv", Some(code), &[("ok", "false")]);
            return;
        }
    };
    if items.is_empty() {
        emit_marker("handshake_recv", None, &[("msg", "none"), ("ok", "true")]);
        return;
    }

    if let Ok(Some(pending)) = hs_pending_load(self_label, peer) {
        if pending.role == "initiator" {
            // Initiator finalize: expect HS2
            for item in items {
                match hs_decode_resp(&item.data) {
                    Ok(resp) => {
                        if resp.session_id != pending.session_id {
                            emit_marker(
                                "handshake_reject",
                                None,
                                &[("reason", "session_id_mismatch")],
                            );
                            continue;
                        }
                        let c = StdCrypto;
                        let ss_pq = match c.decap(&pending.kem_sk, &resp.kem_ct) {
                            Ok(v) => v,
                            Err(_) => {
                                emit_marker(
                                    "handshake_reject",
                                    None,
                                    &[("reason", "pq_decap_failed")],
                                );
                                return;
                            }
                        };
                        let pq_init_ss = hs_pq_init_ss(&ss_pq, &resp.session_id);
                        let dh_init_arr = hs_dh_init_from_pq(&pq_init_ss, &resp.session_id);
                        let (dh_self_pub, dh_peer_pub) =
                            hs_dh_pubs_from_pq(&pq_init_ss, &resp.session_id, true);
                        let a1 = hs_encode_init(&HsInit {
                            session_id: pending.session_id,
                            kem_pk: pending.kem_pk.clone(),
                            sig_pk: pending.sig_pk.clone(),
                        });
                        let b1_no_auth = {
                            let mut tmp = Vec::with_capacity(
                                4 + 2 + 1 + 16 + hs_kem_ct_len() + hs_sig_pk_len(),
                            );
                            tmp.extend_from_slice(HS_MAGIC);
                            tmp.extend_from_slice(&HS_VERSION.to_be_bytes());
                            tmp.push(HS_TYPE_RESP);
                            tmp.extend_from_slice(&resp.session_id);
                            tmp.extend_from_slice(&resp.kem_ct);
                            tmp.extend_from_slice(&resp.sig_pk);
                            tmp
                        };
                        let mac = hs_transcript_mac(&pq_init_ss, &a1, &b1_no_auth);
                        if mac != resp.mac {
                            emit_marker("handshake_reject", None, &[("reason", "bad_transcript")]);
                            return;
                        }
                        let th = hs_transcript_hash(&pq_init_ss, &a1, &b1_no_auth);
                        let sig_msg = hs_sig_msg_b1(&resp.session_id, &th);
                        if hs_sig_verify(&resp.sig_pk, &sig_msg, &resp.sig, "b1_verify").is_err() {
                            emit_marker("handshake_reject", None, &[("reason", "sig_invalid")]);
                            return;
                        }
                        let sig_fp = hs_sig_fingerprint(&resp.sig_pk);
                        match identity_read_sig_pin(peer) {
                            Ok(Some(pinned)) => {
                                if pinned != sig_fp {
                                    emit_peer_mismatch(peer, pinned.as_str(), sig_fp.as_str());
                                    emit_marker(
                                        "handshake_reject",
                                        None,
                                        &[("reason", "peer_mismatch")],
                                    );
                                    return;
                                }
                                emit_marker(
                                    "identity_ok",
                                    None,
                                    &[("peer", peer), ("fp", sig_fp.as_str())],
                                );
                            }
                            Ok(None) => emit_marker(
                                "identity_unknown",
                                None,
                                &[("peer", peer), ("seen_fp", sig_fp.as_str())],
                            ),
                            Err(_) => {
                                emit_marker(
                                    "handshake_reject",
                                    None,
                                    &[("reason", "identity_pin_failed")],
                                );
                                return;
                            }
                        }
                        let st = match hs_build_session(
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
                                return;
                            }
                        };
                        qsp_session_store(peer, &st).unwrap_or_else(|_| {
                            print_error_marker("handshake_session_store_failed")
                        });
                        let _ = hs_pending_clear(self_label, peer);
                        let k_confirm = hs_confirm_key(&pq_init_ss, &resp.session_id, &th);
                        let cmac = hs_confirm_mac(&k_confirm, &resp.session_id, &th);
                        let sig_sk = identity_self_kem_keypair(self_label)
                            .unwrap_or_else(|e| print_error_marker(e.as_str()))
                            .sig_sk;
                        let a2_sig_msg = hs_sig_msg_a2(&resp.session_id, &th, &cmac);
                        let a2_sig = match c.sign(&sig_sk, &a2_sig_msg) {
                            Ok(v) => v,
                            Err(_) => {
                                emit_marker(
                                    "handshake_reject",
                                    None,
                                    &[("reason", "sig_sign_failed")],
                                );
                                return;
                            }
                        };
                        emit_marker(
                            "sig_status",
                            None,
                            &[("ok", "true"), ("alg", "ML-DSA-65"), ("reason", "a2_sign")],
                        );
                        let confirm = HsConfirm {
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
                        let resp_channel = match handshake_channel(peer) {
                            Ok(v) => v,
                            Err(code) => print_error_marker(code),
                        };
                        relay_inbox_push(relay, &resp_channel, &cbytes)
                            .unwrap_or_else(|code| print_error_marker(code));
                        emit_marker(
                            "handshake_complete",
                            None,
                            &[("peer", peer), ("role", "initiator")],
                        );
                        return;
                    }
                    Err(_) => {
                        emit_marker("handshake_reject", None, &[("reason", "decode_failed")]);
                        continue;
                    }
                }
            }
            return;
        }
        if pending.role == "responder" {
            // Responder confirm: expect A2
            for item in items {
                match hs_decode_confirm(&item.data) {
                    Ok(confirm) => {
                        if confirm.session_id != pending.session_id {
                            emit_marker(
                                "handshake_reject",
                                None,
                                &[("reason", "session_id_mismatch")],
                            );
                            continue;
                        }
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
                        let expect = hs_confirm_mac(&k_confirm, &confirm.session_id, &th);
                        if expect != confirm.mac {
                            emit_marker("handshake_recv", None, &[("msg", "A2"), ("ok", "false")]);
                            emit_marker("handshake_reject", None, &[("reason", "bad_confirm")]);
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
                        match identity_read_pin(peer) {
                            Ok(None) => emit_marker(
                                "identity_unknown",
                                None,
                                &[("peer", peer), ("seen_fp", peer_fp.as_str())],
                            ),
                            Ok(Some(pinned)) => {
                                if pinned != *peer_fp {
                                    emit_peer_mismatch(peer, pinned.as_str(), peer_fp.as_str());
                                    emit_marker(
                                        "handshake_reject",
                                        None,
                                        &[("reason", "peer_mismatch")],
                                    );
                                    continue;
                                }
                                emit_marker(
                                    "identity_ok",
                                    None,
                                    &[("peer", peer), ("fp", peer_fp.as_str())],
                                );
                            }
                            Err(_) => {
                                emit_marker(
                                    "handshake_reject",
                                    None,
                                    &[("reason", "identity_pin_failed")],
                                );
                                continue;
                            }
                        }
                        match identity_read_sig_pin(peer) {
                            Ok(None) => emit_marker(
                                "identity_unknown",
                                None,
                                &[("peer", peer), ("seen_fp", peer_sig_fp.as_str())],
                            ),
                            Ok(Some(pinned)) => {
                                if pinned != *peer_sig_fp {
                                    emit_peer_mismatch(peer, pinned.as_str(), peer_sig_fp.as_str());
                                    emit_marker(
                                        "handshake_reject",
                                        None,
                                        &[("reason", "peer_mismatch")],
                                    );
                                    continue;
                                }
                                emit_marker(
                                    "identity_ok",
                                    None,
                                    &[("peer", peer), ("fp", peer_sig_fp.as_str())],
                                );
                            }
                            Err(_) => {
                                emit_marker(
                                    "handshake_reject",
                                    None,
                                    &[("reason", "identity_pin_failed")],
                                );
                                continue;
                            }
                        }
                        qsp_session_store(peer, &st).unwrap_or_else(|_| {
                            print_error_marker("handshake_session_store_failed")
                        });
                        let _ = hs_pending_clear(self_label, peer);
                        emit_marker(
                            "handshake_complete",
                            None,
                            &[("peer", peer), ("role", "responder")],
                        );
                        return;
                    }
                    Err(_) => {
                        emit_marker("handshake_reject", None, &[("reason", "decode_failed")]);
                        continue;
                    }
                }
            }
            return;
        }
    }

    // Responder: process HS1 and send HS2
    for item in items {
        match hs_decode_init(&item.data) {
            Ok(init) => {
                let peer_fp = identity_fingerprint_from_pk(&init.kem_pk);
                let peer_sig_fp = hs_sig_fingerprint(&init.sig_pk);
                match identity_read_pin(peer) {
                    Ok(Some(pinned)) => {
                        if pinned != peer_fp {
                            emit_peer_mismatch(peer, pinned.as_str(), peer_fp.as_str());
                            emit_marker("handshake_reject", None, &[("reason", "peer_mismatch")]);
                            continue;
                        }
                    }
                    Ok(None) => {}
                    Err(_) => {
                        emit_marker(
                            "handshake_reject",
                            None,
                            &[("reason", "identity_pin_failed")],
                        );
                        continue;
                    }
                }
                match identity_read_sig_pin(peer) {
                    Ok(Some(pinned)) => {
                        if pinned != peer_sig_fp {
                            emit_peer_mismatch(peer, pinned.as_str(), peer_sig_fp.as_str());
                            emit_marker("handshake_reject", None, &[("reason", "peer_mismatch")]);
                            continue;
                        }
                    }
                    Ok(None) => {}
                    Err(_) => {
                        emit_marker(
                            "handshake_reject",
                            None,
                            &[("reason", "identity_pin_failed")],
                        );
                        continue;
                    }
                }
                let c = StdCrypto;
                let (kem_ct, ss_pq) = match c.encap(&init.kem_pk) {
                    Ok(v) => v,
                    Err(_) => {
                        emit_marker("handshake_reject", None, &[("reason", "pq_encap_failed")]);
                        continue;
                    }
                };
                let pq_init_ss = hs_pq_init_ss(&ss_pq, &init.session_id);
                let dh_init_arr = hs_dh_init_from_pq(&pq_init_ss, &init.session_id);
                let (dh_self_pub, dh_peer_pub) =
                    hs_dh_pubs_from_pq(&pq_init_ss, &init.session_id, false);
                let st = match hs_build_session(
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
                let b1_no_auth = {
                    let mut tmp =
                        Vec::with_capacity(4 + 2 + 1 + 16 + hs_kem_ct_len() + hs_sig_pk_len());
                    tmp.extend_from_slice(HS_MAGIC);
                    tmp.extend_from_slice(&HS_VERSION.to_be_bytes());
                    tmp.push(HS_TYPE_RESP);
                    tmp.extend_from_slice(&init.session_id);
                    tmp.extend_from_slice(&kem_ct);
                    tmp.extend_from_slice(&self_sig_pk);
                    tmp
                };
                let mac = hs_transcript_mac(&pq_init_ss, &a1, &b1_no_auth);
                let th = hs_transcript_hash(&pq_init_ss, &a1, &b1_no_auth);
                let sig_msg = hs_sig_msg_b1(&init.session_id, &th);
                let sig = match c.sign(&self_sig_sk, &sig_msg) {
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
                let k_confirm = hs_confirm_key(&pq_init_ss, &init.session_id, &th);
                let pending = HandshakePending {
                    self_label: self_label.to_string(),
                    peer: peer.to_string(),
                    session_id: init.session_id,
                    kem_sk: Vec::new(),
                    kem_pk: Vec::new(),
                    sig_pk: Vec::new(),
                    peer_fp: Some(peer_fp),
                    peer_sig_fp: Some(peer_sig_fp),
                    peer_sig_pk: Some(init.sig_pk.clone()),
                    role: "responder".to_string(),
                    confirm_key: Some(k_confirm),
                    transcript_hash: Some(th),
                    pending_session: Some(st.snapshot_bytes()),
                };
                hs_pending_store(&pending)
                    .unwrap_or_else(|_| print_error_marker("handshake_pending_store_failed"));
                let resp = HsResp {
                    session_id: init.session_id,
                    kem_ct,
                    mac,
                    sig_pk: self_sig_pk,
                    sig,
                };
                let bytes = hs_encode_resp(&resp);
                let size_s = bytes.len().to_string();
                let ct_len_s = hs_kem_ct_len().to_string();
                let sig_pk_len_s = hs_sig_pk_len().to_string();
                emit_marker(
                    "handshake_send",
                    None,
                    &[
                        ("msg", "B1"),
                        ("size", size_s.as_str()),
                        ("kem_ct_len", ct_len_s.as_str()),
                        ("sig_pk_len", sig_pk_len_s.as_str()),
                    ],
                );
                let resp_channel = match handshake_channel(peer) {
                    Ok(v) => v,
                    Err(code) => print_error_marker(code),
                };
                relay_inbox_push(relay, &resp_channel, &bytes)
                    .unwrap_or_else(|code| print_error_marker(code));
                return;
            }
            Err(_) => {
                emit_marker("handshake_reject", None, &[("reason", "decode_failed")]);
                continue;
            }
        }
    }
}

#[derive(Clone, Debug)]
struct RelayConfig {
    seed: u64,
    drop_pct: u8,
    dup_pct: u8,
    reorder_window: usize,
    fixed_latency_ms: u64,
    jitter_ms: u64,
}

#[derive(Serialize, Deserialize)]
struct RelayFrame {
    to: String,
    data: Vec<u8>,
}

#[derive(Serialize, Deserialize)]
struct RelayResponse {
    action: String,
    delivered: bool,
}

struct RelayRng {
    state: u64,
}

impl RelayRng {
    fn new(seed: u64) -> Self {
        Self {
            state: seed ^ 0x9e3779b97f4a7c15,
        }
    }

    fn next_u64(&mut self) -> u64 {
        // xorshift64*
        let mut x = self.state;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.state = x;
        x.wrapping_mul(0x2545f4914f6cdd1d)
    }

    fn next_u32(&mut self) -> u32 {
        (self.next_u64() >> 32) as u32
    }
}

struct RelayDecision {
    action: &'static str,
    delivered: bool,
    delay_ms: u64,
}

fn relay_decide(cfg: &RelayConfig, seq: u64) -> RelayDecision {
    let mut rng = RelayRng::new(cfg.seed ^ seq);
    let roll = (rng.next_u32() % 100) as u8;
    if cfg.drop_pct > 0 && roll < cfg.drop_pct {
        return RelayDecision {
            action: "drop",
            delivered: false,
            delay_ms: 0,
        };
    }
    let roll_dup = (rng.next_u32() % 100) as u8;
    if cfg.dup_pct > 0 && roll_dup < cfg.dup_pct {
        return RelayDecision {
            action: "dup",
            delivered: false,
            delay_ms: 0,
        };
    }

    let mut delay_ms = 0;
    if cfg.fixed_latency_ms > 0 || cfg.jitter_ms > 0 {
        delay_ms = cfg.fixed_latency_ms;
        if cfg.jitter_ms > 0 {
            delay_ms = delay_ms.saturating_add(rng.next_u64() % (cfg.jitter_ms + 1));
        }
    }

    if cfg.reorder_window > 1 && (seq % (cfg.reorder_window as u64)) == 1 {
        return RelayDecision {
            action: "reorder",
            delivered: true,
            delay_ms,
        };
    }
    if delay_ms > 0 {
        return RelayDecision {
            action: "delay",
            delivered: true,
            delay_ms,
        };
    }
    RelayDecision {
        action: "deliver",
        delivered: true,
        delay_ms: 0,
    }
}

fn send_execute(args: SendExecuteArgs) {
    if !require_unlocked("send") {
        return;
    }
    let SendExecuteArgs {
        transport,
        relay,
        to,
        file,
        pad_to,
        pad_bucket,
        bucket_max,
        meta_seed,
        receipt,
    } = args;
    let transport = match transport {
        Some(v) => v,
        None => print_error_marker("send_transport_required"),
    };

    match transport {
        SendTransport::Relay => {
            let relay = match relay {
                Some(v) => v,
                None => print_error_marker("send_relay_required"),
            };
            let to = match to {
                Some(v) => v,
                None => print_error_marker("send_to_required"),
            };
            let file = match file {
                Some(v) => v,
                None => print_error_marker("send_file_required"),
            };
            if let Err(code) = enforce_peer_not_blocked(to.as_str()) {
                print_error_marker(code);
            }
            let pad_cfg = match meta_pad_config_from_args(pad_to, pad_bucket, meta_seed) {
                Ok(v) => v,
                Err(code) => print_error_marker(code),
            };
            if let Err(reason) = protocol_active_or_reason_for_peer(to.as_str()) {
                protocol_inactive_exit(reason.as_str());
            }
            if let Some(seed) = meta_seed {
                let seed_s = seed.to_string();
                emit_marker(
                    "meta_mode",
                    None,
                    &[("deterministic", "true"), ("seed", seed_s.as_str())],
                );
            }
            if receipt.is_none() {
                emit_marker("receipt_disabled", None, &[]);
            }
            relay_send(&to, &file, &relay, pad_cfg, bucket_max, meta_seed, receipt);
        }
    }
}

struct SendExecuteArgs {
    transport: Option<SendTransport>,
    relay: Option<String>,
    to: Option<String>,
    file: Option<PathBuf>,
    pad_to: Option<usize>,
    pad_bucket: Option<MetaPadBucket>,
    bucket_max: Option<usize>,
    meta_seed: Option<u64>,
    receipt: Option<ReceiptKind>,
}

fn send_abort() {
    if !require_unlocked("send_abort") {
        return;
    }
    let (dir, source) = match config_dir() {
        Ok(v) => v,
        Err(e) => print_error(e),
    };
    let _lock = match lock_store_exclusive(&dir, source) {
        Ok(v) => v,
        Err(e) => print_error(e),
    };
    if let Err(e) = ensure_store_layout(&dir, source) {
        print_error(e);
    }

    let outbox_path = dir.join(OUTBOX_FILE_NAME);
    if let Err(e) = enforce_safe_parents(&outbox_path, source) {
        print_error(e);
    }

    if outbox_path.exists() {
        if fs::remove_file(&outbox_path).is_err() {
            print_error_marker("outbox_abort_failed");
        }
        emit_marker(
            "outbox_abort",
            None,
            &[("ok", "true"), ("action", "removed")],
        );
    } else {
        emit_marker(
            "outbox_abort",
            None,
            &[("ok", "true"), ("action", "absent")],
        );
    }
}

struct ReceiveArgs {
    transport: Option<SendTransport>,
    relay: Option<String>,
    from: Option<String>,
    mailbox: Option<String>,
    max: Option<usize>,
    out: Option<PathBuf>,
    deterministic_meta: bool,
    interval_ms: Option<u64>,
    poll_interval_ms: Option<u64>,
    poll_ticks: Option<u32>,
    batch_max_count: Option<u32>,
    poll_max_per_tick: Option<u32>,
    bucket_max: Option<usize>,
    meta_seed: Option<u64>,
    emit_receipts: Option<ReceiptKind>,
}

fn receive_execute(args: ReceiveArgs) {
    if !require_unlocked("receive") {
        return;
    }
    let ReceiveArgs {
        transport,
        relay,
        from,
        mailbox,
        max,
        out,
        deterministic_meta,
        interval_ms,
        poll_interval_ms,
        poll_ticks,
        batch_max_count,
        poll_max_per_tick,
        bucket_max,
        meta_seed,
        emit_receipts,
    } = args;
    let transport = match transport {
        Some(v) => v,
        None => print_error_marker("recv_transport_required"),
    };
    match transport {
        SendTransport::Relay => {
            let relay = match relay {
                Some(v) => v,
                None => print_error_marker("recv_relay_required"),
            };
            if !relay_is_http(&relay) {
                print_error_marker("recv_relay_http_required");
            }
            let from = match from {
                Some(v) => v,
                None => print_error_marker("recv_from_required"),
            };
            let mailbox = mailbox.unwrap_or_else(|| default_receive_mailbox(from.as_str()));
            if !channel_label_ok(mailbox.as_str()) {
                print_error_marker("recv_mailbox_invalid");
            }
            let max = match max {
                Some(v) if v > 0 => v,
                _ => print_error_marker("recv_max_required"),
            };
            let out = match out {
                Some(v) => v,
                None => print_error_marker("recv_out_required"),
            };
            let poll_cfg = match meta_poll_config_from_args(MetaPollArgs {
                deterministic_meta,
                interval_ms,
                poll_interval_ms,
                ticks: poll_ticks,
                batch_max_count,
                poll_max_per_tick,
                bucket_max,
                meta_seed,
            }) {
                Ok(v) => v,
                Err(code) => print_error_marker(code),
            };
            let source = ConfigSource::EnvOverride;
            if let Err(e) = ensure_dir_secure(&out, source) {
                print_error(e);
            }
            let (cfg_dir, cfg_source) = match config_dir() {
                Ok(v) => v,
                Err(e) => print_error(e),
            };
            if !check_symlink_safe(&cfg_dir) {
                print_error(ErrorCode::UnsafePathSymlink);
            }
            if !check_parent_safe(&cfg_dir, cfg_source) {
                print_error(ErrorCode::UnsafeParentPerms);
            }
            if let Err(reason) = protocol_active_or_reason_for_peer(from.as_str()) {
                protocol_inactive_exit(reason.as_str());
            }

            if let Some(seed) = meta_seed {
                let seed_s = seed.to_string();
                emit_marker(
                    "meta_mode",
                    None,
                    &[("deterministic", "true"), ("seed", seed_s.as_str())],
                );
            }
            let recv_max = poll_cfg.as_ref().map(|c| c.batch_max_count).unwrap_or(max);
            let max_s = recv_max.to_string();
            emit_marker(
                "recv_start",
                None,
                &[
                    ("transport", "relay"),
                    ("mailbox", mailbox.as_str()),
                    ("from", from.as_str()),
                    ("max", max_s.as_str()),
                ],
            );
            let mut total = 0usize;
            if let Some(cfg) = poll_cfg {
                let interval_s = cfg.interval_ms.to_string();
                let ticks_s = cfg.ticks.to_string();
                let max_tick_s = cfg.batch_max_count.to_string();
                let bucket_max_s = cfg.bucket_max.to_string();
                emit_marker(
                    "meta_poll_config",
                    None,
                    &[
                        ("interval_ms", interval_s.as_str()),
                        ("ticks", ticks_s.as_str()),
                        ("batch_max_count", max_tick_s.as_str()),
                        ("bucket_max", bucket_max_s.as_str()),
                    ],
                );
                for tick in 0..cfg.ticks {
                    let tick_s = tick.to_string();
                    let deterministic_s = if cfg.deterministic { "true" } else { "false" };
                    emit_marker(
                        "meta_tick",
                        None,
                        &[
                            ("tick", tick_s.as_str()),
                            ("interval_ms", interval_s.as_str()),
                            ("deterministic", deterministic_s),
                        ],
                    );
                    let pull = ReceivePullCtx {
                        relay: &relay,
                        mailbox: mailbox.as_str(),
                        from: &from,
                        out: &out,
                        source,
                        cfg_dir: &cfg_dir,
                        cfg_source,
                        bucket_max: cfg.bucket_max,
                        emit_receipts,
                    };
                    let stats = receive_pull_and_write(&pull, cfg.batch_max_count);
                    total = total.saturating_add(stats.count);
                    let count_s = stats.count.to_string();
                    let bytes_s = stats.bytes.to_string();
                    emit_marker(
                        "meta_batch",
                        None,
                        &[("count", count_s.as_str()), ("bytes", bytes_s.as_str())],
                    );
                    if !cfg.deterministic && cfg.interval_ms > 0 {
                        std::thread::sleep(Duration::from_millis(cfg.interval_ms));
                    }
                }
            } else {
                let pull = ReceivePullCtx {
                    relay: &relay,
                    mailbox: mailbox.as_str(),
                    from: &from,
                    out: &out,
                    source,
                    cfg_dir: &cfg_dir,
                    cfg_source,
                    bucket_max: META_BUCKET_MAX_DEFAULT,
                    emit_receipts,
                };
                total = receive_pull_and_write(&pull, max).count;
            }
            if total == 0 {
                emit_marker("recv_none", None, &[]);
                return;
            }
            let count_s = total.to_string();
            emit_marker("recv_commit", None, &[("count", count_s.as_str())]);
        }
    }
}

fn default_receive_mailbox(from: &str) -> String {
    let self_label = env::var("QSC_SELF_LABEL")
        .ok()
        .map(|v| v.trim().to_string())
        .filter(|v| !v.is_empty());
    match self_label {
        Some(v) if channel_label_ok(v.as_str()) => v,
        _ => from.to_string(),
    }
}

struct ReceivePullCtx<'a> {
    relay: &'a str,
    mailbox: &'a str,
    from: &'a str,
    out: &'a Path,
    source: ConfigSource,
    cfg_dir: &'a Path,
    cfg_source: ConfigSource,
    bucket_max: usize,
    emit_receipts: Option<ReceiptKind>,
}

struct ReceivePullStats {
    count: usize,
    bytes: usize,
}

fn receive_pull_and_write(ctx: &ReceivePullCtx<'_>, max: usize) -> ReceivePullStats {
    let items = match relay_inbox_pull(ctx.relay, ctx.mailbox, max) {
        Ok(v) => v,
        Err(code) => print_error_marker(code),
    };
    if items.is_empty() {
        return ReceivePullStats { count: 0, bytes: 0 };
    }
    let mut stats = ReceivePullStats { count: 0, bytes: 0 };
    for item in items {
        let envelope_len = item.data.len();
        match qsp_unpack(ctx.from, &item.data) {
            Ok(outcome) => {
                let commit_unpack_state = || {
                    record_qsp_status(ctx.cfg_dir, ctx.cfg_source, true, "unpack_ok", false, true);
                    emit_marker("qsp_unpack", None, &[("ok", "true"), ("version", "5.0")]);
                    let msg_idx_s = outcome.msg_idx.to_string();
                    emit_marker(
                        "ratchet_recv_advance",
                        None,
                        &[("msg_idx", msg_idx_s.as_str())],
                    );
                    if outcome.skip_delta > 0 {
                        let sd = outcome.skip_delta.to_string();
                        emit_marker("ratchet_skip_store", None, &[("count", sd.as_str())]);
                    }
                    if outcome.evicted > 0 {
                        let ev = outcome.evicted.to_string();
                        emit_marker("ratchet_skip_evict", None, &[("count", ev.as_str())]);
                    }
                    if qsp_session_store(ctx.from, &outcome.next_state).is_err() {
                        emit_marker("error", Some("qsp_session_store_failed"), &[]);
                        print_error_marker("qsp_session_store_failed");
                    }
                };
                let mut payload = outcome.plaintext.clone();
                let mut request_receipt = false;
                let mut request_msg_id = String::new();
                if let Some(file_payload) = parse_file_transfer_payload(&outcome.plaintext) {
                    let file_id = match &file_payload {
                        FileTransferPayload::Chunk(v) => v.file_id.clone(),
                        FileTransferPayload::Manifest(v) => v.file_id.clone(),
                    };
                    let file_res = match file_payload {
                        FileTransferPayload::Chunk(v) => file_transfer_handle_chunk(ctx, v),
                        FileTransferPayload::Manifest(v) => file_transfer_handle_manifest(ctx, v),
                    };
                    if let Err(reason) = file_res {
                        emit_marker(
                            "file_xfer_reject",
                            Some(reason),
                            &[("id", file_id.as_str()), ("reason", reason)],
                        );
                        print_error_marker(reason);
                    }
                    commit_unpack_state();
                    continue;
                }
                if let Some(ctrl) = parse_receipt_payload(&outcome.plaintext) {
                    if ctrl.v == 1 && ctrl.kind == "delivered" && ctrl.t == "ack" {
                        commit_unpack_state();
                        match timeline_transition_entry_state(
                            ctx.from,
                            ctrl.msg_id.as_str(),
                            MessageState::Delivered,
                        ) {
                            Ok(_) => {
                                emit_marker(
                                    "receipt_recv",
                                    None,
                                    &[("kind", "delivered"), ("msg_id", "<redacted>")],
                                );
                                emit_marker(
                                    "delivered_to_peer",
                                    None,
                                    &[("kind", "delivered"), ("msg_id", "<redacted>")],
                                );
                            }
                            Err(reason) => emit_message_state_reject(ctrl.msg_id.as_str(), reason),
                        }
                        continue;
                    }
                    if ctrl.v == 1 && ctrl.kind == "delivered" && ctrl.t == "data" {
                        if let Some(body) = ctrl.body {
                            payload = body;
                            request_receipt = true;
                            request_msg_id = ctrl.msg_id;
                        }
                    }
                }
                commit_unpack_state();
                stats.count = stats.count.saturating_add(1);
                stats.bytes = stats.bytes.saturating_add(envelope_len);
                let bucket = meta_bucket_for_len(envelope_len, ctx.bucket_max);
                let bucket_s = bucket.to_string();
                let orig_s = envelope_len.to_string();
                let capped_s = if envelope_len > ctx.bucket_max {
                    ctx.bucket_max.to_string()
                } else {
                    envelope_len.to_string()
                };
                emit_marker(
                    "meta_bucket",
                    None,
                    &[
                        ("bucket", bucket_s.as_str()),
                        ("orig", orig_s.as_str()),
                        ("capped", capped_s.as_str()),
                        ("metric", "envelope_len"),
                    ],
                );
                let name = format!("recv_{}.bin", stats.count);
                let path = ctx.out.join(name);
                if write_atomic(&path, &payload, ctx.source).is_err() {
                    print_error_marker("recv_write_failed");
                }
                let idx_s = stats.count.to_string();
                let size_s = payload.len().to_string();
                emit_marker(
                    "recv_item",
                    None,
                    &[
                        ("idx", idx_s.as_str()),
                        ("size", size_s.as_str()),
                        ("id", item.id.as_str()),
                    ],
                );
                if let Err(code) = timeline_append_entry(
                    ctx.from,
                    "in",
                    payload.len(),
                    "msg",
                    MessageState::Received,
                    if request_msg_id.is_empty() {
                        None
                    } else {
                        Some(request_msg_id.as_str())
                    },
                ) {
                    emit_message_state_reject("<redacted>", code);
                    emit_marker("error", Some(code), &[("op", "timeline_receive_ingest")]);
                }
                if request_receipt {
                    match ctx.emit_receipts {
                        Some(ReceiptKind::Delivered) => {
                            match send_delivered_receipt_ack(ctx.relay, ctx.from, &request_msg_id) {
                                Ok(()) => emit_marker(
                                    "receipt_send",
                                    None,
                                    &[
                                        ("kind", "delivered"),
                                        ("bucket", "small"),
                                        ("msg_id", "<redacted>"),
                                    ],
                                ),
                                Err(code) => emit_marker(
                                    "receipt_send_failed",
                                    Some(code),
                                    &[("code", code)],
                                ),
                            }
                        }
                        None => emit_marker("receipt_disabled", None, &[]),
                    }
                }
            }
            Err(code) => {
                record_qsp_status(ctx.cfg_dir, ctx.cfg_source, false, code, false, false);
                emit_marker("qsp_unpack", Some(code), &[("ok", "false")]);
                if code == "qsp_replay_reject" {
                    let msg_idx = qsp_session_for_channel(ctx.from)
                        .map(|st| st.recv.nr.to_string())
                        .unwrap_or_else(|_| "0".to_string());
                    emit_marker("ratchet_replay_reject", None, &[("msg_idx", &msg_idx)]);
                }
                print_error_marker(code);
            }
        }
    }
    stats
}

fn receive_file(path: &Path) {
    if !require_unlocked("receive_file") {
        return;
    }
    let (dir, source) = match config_dir() {
        Ok(v) => v,
        Err(e) => print_error(e),
    };
    // Fail-closed: reject if config dir parents or symlinks are unsafe.
    if !check_symlink_safe(&dir) {
        print_error(ErrorCode::UnsafePathSymlink);
    }
    if !check_parent_safe(&dir, source) {
        print_error(ErrorCode::UnsafeParentPerms);
    }

    let bytes = match fs::read(path) {
        Ok(v) => v,
        Err(_) => print_error(ErrorCode::IoReadFailed),
    };
    if bytes.is_empty() {
        emit_marker("recv_reject", None, &[("reason", "empty")]);
        print_error_marker("recv_reject_parse");
    }
    if bytes.len() > envelope::MAX_BUNDLE_SIZE_DEFAULT {
        emit_marker("recv_reject", None, &[("reason", "oversize")]);
        print_error_marker("recv_reject_size");
    }

    emit_marker("recv_reject", None, &[("reason", "malformed")]);
    print_error_marker("recv_reject_parse");
}

fn relay_serve(port: u16, cfg: RelayConfig, max_messages: u64) {
    let addr = format!("127.0.0.1:{}", port);
    let listener =
        TcpListener::bind(&addr).unwrap_or_else(|_| print_error_marker("relay_bind_failed"));
    let bound = listener
        .local_addr()
        .unwrap_or_else(|_| print_error_marker("relay_bind_failed"));
    let port_s = bound.port().to_string();
    let seed_s = cfg.seed.to_string();
    emit_marker(
        "relay_listen",
        None,
        &[("port", port_s.as_str()), ("seed", seed_s.as_str())],
    );

    let mut seq: u64 = 0;
    for stream in listener.incoming() {
        let mut stream = match stream {
            Ok(s) => s,
            Err(_) => continue,
        };
        seq = seq.wrapping_add(1);
        let seq_s = seq.to_string();
        let decision = relay_decide(&cfg, seq);
        if decision.delay_ms > 0 {
            let delay_s = decision.delay_ms.to_string();
            emit_marker(
                "relay_event",
                None,
                &[
                    ("action", "delay"),
                    ("ms", delay_s.as_str()),
                    ("seq", seq_s.as_str()),
                ],
            );
            std::thread::sleep(Duration::from_millis(decision.delay_ms));
        }

        let frame: RelayFrame = match read_frame(&mut stream) {
            Ok(v) => v,
            Err(_) => {
                let resp = RelayResponse {
                    action: "reject".to_string(),
                    delivered: false,
                };
                let _ = write_frame(&mut stream, &resp);
                emit_marker(
                    "relay_event",
                    None,
                    &[("action", "reject"), ("seq", seq_s.as_str())],
                );
                if max_messages > 0 && seq >= max_messages {
                    break;
                }
                continue;
            }
        };

        let _ = frame; // relay is a dumb pipe; no persistence or content logging.
        emit_marker(
            "relay_event",
            None,
            &[("action", decision.action), ("seq", seq_s.as_str())],
        );
        let resp = RelayResponse {
            action: decision.action.to_string(),
            delivered: decision.delivered,
        };
        let _ = write_frame(&mut stream, &resp);

        if max_messages > 0 && seq >= max_messages {
            break;
        }
    }
}

fn relay_send(
    to: &str,
    file: &Path,
    relay: &str,
    pad_cfg: Option<MetaPadConfig>,
    bucket_max: Option<usize>,
    meta_seed: Option<u64>,
    receipt: Option<ReceiptKind>,
) {
    if let Err(code) = enforce_peer_not_blocked(to) {
        print_error_marker(code);
    }
    if let Err(reason) = protocol_active_or_reason_for_peer(to) {
        protocol_inactive_exit(reason.as_str());
    }
    let payload = match fs::read(file) {
        Ok(v) => v,
        Err(_) => print_error_marker("relay_payload_read_failed"),
    };
    let outcome = relay_send_with_payload(RelaySendPayloadArgs {
        to,
        payload,
        relay,
        injector: fault_injector_from_env(),
        pad_cfg,
        bucket_max,
        meta_seed,
        receipt,
    });
    if let Some(code) = outcome.error_code {
        print_error_marker(code);
    }
}

struct RelaySendOutcome {
    action: String,
    delivered: bool,
    error_code: Option<&'static str>,
}

#[derive(Clone, Copy)]
struct TimelineSendIngest<'a> {
    peer: &'a str,
    byte_len: usize,
    kind: &'static str,
    message_id: Option<&'a str>,
}

#[derive(Deserialize)]
struct InboxPullItem {
    id: String,
    data: Vec<u8>,
}

#[derive(Deserialize)]
struct InboxPullResp {
    items: Vec<InboxPullItem>,
}

#[derive(Clone)]
struct FaultInjector {
    seed: u64,
    scenario: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum FaultAction {
    Drop,
    Reorder,
}

fn fault_injector_from_env() -> Option<FaultInjector> {
    let scenario = env::var("QSC_SCENARIO").ok()?;
    if scenario == "happy-path" || scenario == "default" {
        return None;
    }
    let seed_str = match env::var("QSC_SEED") {
        Ok(v) => v,
        Err(_) => print_error_marker("fault_injection_seed_required"),
    };
    let seed = seed_str
        .trim()
        .parse::<u64>()
        .unwrap_or_else(|_| print_error_marker("fault_injection_seed_invalid"));
    Some(FaultInjector { seed, scenario })
}

fn fault_injector_from_tui(cfg: &TuiRelayConfig) -> Option<FaultInjector> {
    if cfg.scenario == "happy-path" || cfg.scenario == "default" {
        return None;
    }
    Some(FaultInjector {
        seed: cfg.seed,
        scenario: cfg.scenario.clone(),
    })
}

fn relay_is_http(relay: &str) -> bool {
    relay.starts_with("http://") || relay.starts_with("https://")
}

fn channel_label_ok(label: &str) -> bool {
    !label.is_empty()
        && label
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
}

fn relay_auth_token() -> Option<String> {
    let primary = env::var("QSC_RELAY_TOKEN").ok();
    let fallback = env::var("RELAY_TOKEN").ok();
    primary
        .or(fallback)
        .map(|v| v.trim().to_string())
        .filter(|v| !v.is_empty())
}

fn relay_inbox_push(relay_base: &str, channel: &str, payload: &[u8]) -> Result<(), &'static str> {
    if !channel_label_ok(channel) {
        return Err("relay_inbox_channel_invalid");
    }
    let base = relay_base.trim_end_matches('/');
    let url = format!("{}/v1/push/{}", base, channel);
    let client = HttpClient::new();
    let mut req = client.post(url).body(payload.to_vec());
    if let Some(token) = relay_auth_token() {
        req = req.header("Authorization", format!("Bearer {}", token));
    }
    let resp = match req.send() {
        Ok(v) => v,
        Err(_) => return Err("relay_inbox_push_failed"),
    };
    match resp.status() {
        HttpStatus::OK => Ok(()),
        HttpStatus::UNAUTHORIZED | HttpStatus::FORBIDDEN => Err("relay_unauthorized"),
        HttpStatus::PAYLOAD_TOO_LARGE => Err("relay_inbox_too_large"),
        HttpStatus::TOO_MANY_REQUESTS => Err("relay_inbox_queue_full"),
        _ => Err("relay_inbox_push_failed"),
    }
}

fn relay_inbox_pull(
    relay_base: &str,
    channel: &str,
    max: usize,
) -> Result<Vec<InboxPullItem>, &'static str> {
    if !channel_label_ok(channel) {
        return Err("relay_inbox_channel_invalid");
    }
    let base = relay_base.trim_end_matches('/');
    let url = format!("{}/v1/pull/{}?max={}", base, channel, max);
    let client = HttpClient::new();
    let mut req = client.get(url);
    if let Some(token) = relay_auth_token() {
        req = req.header("Authorization", format!("Bearer {}", token));
    }
    let resp = match req.send() {
        Ok(v) => v,
        Err(_) => return Err("relay_inbox_pull_failed"),
    };
    match resp.status() {
        HttpStatus::OK => {
            let body: InboxPullResp = match resp.json() {
                Ok(v) => v,
                Err(_) => return Err("relay_inbox_parse_failed"),
            };
            Ok(body.items)
        }
        HttpStatus::NO_CONTENT => Ok(Vec::new()),
        HttpStatus::UNAUTHORIZED | HttpStatus::FORBIDDEN => Err("relay_unauthorized"),
        HttpStatus::BAD_REQUEST => Err("relay_inbox_bad_request"),
        HttpStatus::PAYLOAD_TOO_LARGE => Err("relay_inbox_too_large"),
        HttpStatus::TOO_MANY_REQUESTS => Err("relay_inbox_queue_full"),
        _ => Err("relay_inbox_pull_failed"),
    }
}

fn fault_action_for(fi: &FaultInjector, idx: u64) -> Option<FaultAction> {
    if fi.scenario != "drop-reorder" {
        return None;
    }
    let k = fi.seed.wrapping_add(idx);
    match k % 4 {
        0 => Some(FaultAction::Reorder),
        1 => Some(FaultAction::Drop),
        _ => None,
    }
}

static FAULT_IDX: AtomicU64 = AtomicU64::new(0);

fn next_fault_index() -> u64 {
    FAULT_IDX.fetch_add(1, Ordering::SeqCst).wrapping_add(1)
}

struct RelaySendPayloadArgs<'a> {
    to: &'a str,
    payload: Vec<u8>,
    relay: &'a str,
    injector: Option<FaultInjector>,
    pad_cfg: Option<MetaPadConfig>,
    bucket_max: Option<usize>,
    meta_seed: Option<u64>,
    receipt: Option<ReceiptKind>,
}

fn relay_send_with_payload(args: RelaySendPayloadArgs<'_>) -> RelaySendOutcome {
    let RelaySendPayloadArgs {
        to,
        payload,
        relay,
        injector,
        pad_cfg,
        bucket_max,
        meta_seed,
        receipt,
    } = args;
    let (dir, source) = match config_dir() {
        Ok(v) => v,
        Err(e) => print_error(e),
    };
    let _lock = match lock_store_exclusive(&dir, source) {
        Ok(v) => v,
        Err(e) => print_error(e),
    };
    if let Err(e) = ensure_store_layout(&dir, source) {
        print_error(e);
    }

    let outbox_path = dir.join(OUTBOX_FILE_NAME);
    if outbox_path.exists() {
        emit_marker("error", Some("outbox_exists"), &[]);
        return RelaySendOutcome {
            action: "outbox_exists".to_string(),
            delivered: false,
            error_code: Some("outbox_exists"),
        };
    }

    let (payload, receipt_msg_id) = encode_receipt_data_payload(payload, receipt);
    let pack = match qsp_pack(to, &payload, pad_cfg, meta_seed) {
        Ok(v) => {
            record_qsp_status(&dir, source, true, "pack_ok", true, false);
            emit_marker("qsp_pack", None, &[("ok", "true"), ("version", "5.0")]);
            if let Some(label) = v.pad_label {
                let len_s = v.padded_len.to_string();
                emit_marker(
                    "meta_pad",
                    None,
                    &[("bucket", label), ("padded_len", len_s.as_str())],
                );
            }
            let msg_idx_s = v.msg_idx.to_string();
            let ck_idx_s = v.ck_idx.to_string();
            emit_marker(
                "ratchet_send_advance",
                None,
                &[
                    ("msg_idx", msg_idx_s.as_str()),
                    ("ck_idx", ck_idx_s.as_str()),
                ],
            );
            v
        }
        Err(code) => {
            record_qsp_status(&dir, source, false, code, false, false);
            emit_marker("qsp_pack", Some(code), &[("ok", "false")]);
            return RelaySendOutcome {
                action: code.to_string(),
                delivered: false,
                error_code: Some(code),
            };
        }
    };
    let ciphertext = pack.envelope.clone();
    if receipt_msg_id.is_some() {
        emit_marker(
            "receipt_request",
            None,
            &[("kind", "delivered"), ("msg_id", "<redacted>")],
        );
    }
    if let Some(max_bucket) = bucket_max {
        if max_bucket == 0 || max_bucket > META_BUCKET_MAX_CEILING {
            return RelaySendOutcome {
                action: "meta_bucket_invalid".to_string(),
                delivered: false,
                error_code: Some("meta_bucket_invalid"),
            };
        }
        let bucket = meta_bucket_for_len(ciphertext.len(), max_bucket);
        let bucket_s = bucket.to_string();
        let orig_s = ciphertext.len().to_string();
        let capped_s = ciphertext.len().min(max_bucket).to_string();
        emit_marker(
            "meta_bucket",
            None,
            &[
                ("bucket", bucket_s.as_str()),
                ("orig", orig_s.as_str()),
                ("capped", capped_s.as_str()),
                ("metric", "envelope_len"),
            ],
        );
    }
    let outbox = OutboxRecord {
        version: 1,
        payload_len: payload.len(),
    };
    let outbox_bytes = match serde_json::to_vec(&outbox) {
        Ok(v) => v,
        Err(_) => {
            emit_marker("error", Some("outbox_serialize_failed"), &[]);
            return RelaySendOutcome {
                action: "outbox_serialize_failed".to_string(),
                delivered: false,
                error_code: Some("outbox_serialize_failed"),
            };
        }
    };
    if write_atomic(&outbox_path, &outbox_bytes, source).is_err() {
        emit_marker("error", Some("outbox_write_failed"), &[]);
        return RelaySendOutcome {
            action: "outbox_write_failed".to_string(),
            delivered: false,
            error_code: Some("outbox_write_failed"),
        };
    }

    if let Some(fi) = injector.as_ref() {
        let idx = next_fault_index();
        let idx_s = idx.to_string();
        let seed_s = fi.seed.to_string();
        if let Some(action) = fault_action_for(fi, idx) {
            match action {
                FaultAction::Drop => {
                    emit_marker(
                        "relay_event",
                        None,
                        &[
                            ("action", "drop"),
                            ("idx", idx_s.as_str()),
                            ("seed", seed_s.as_str()),
                            ("scenario", fi.scenario.as_str()),
                        ],
                    );
                    print_marker("send_attempt", &[("ok", "false")]);
                    return RelaySendOutcome {
                        action: "drop".to_string(),
                        delivered: false,
                        error_code: Some("relay_drop_injected"),
                    };
                }
                FaultAction::Reorder => {
                    emit_marker(
                        "relay_event",
                        None,
                        &[
                            ("action", "reorder"),
                            ("idx", idx_s.as_str()),
                            ("seed", seed_s.as_str()),
                            ("scenario", fi.scenario.as_str()),
                        ],
                    );
                }
            }
        }
    }

    let len_s = payload.len().to_string();
    print_marker("send_prepare", &[("payload_len", len_s.as_str())]);

    if relay_is_http(relay) {
        match relay_inbox_push(relay, to, &ciphertext) {
            Ok(()) => {
                emit_marker("relay_event", None, &[("action", "deliver")]);
                return finalize_send_commit(
                    &dir,
                    source,
                    &outbox_path,
                    "deliver".to_string(),
                    Some((to, pack.next_state.clone())),
                    Some(TimelineSendIngest {
                        peer: to,
                        byte_len: payload.len(),
                        kind: "file",
                        message_id: receipt_msg_id.as_deref(),
                    }),
                );
            }
            Err(code) => {
                emit_marker("relay_event", None, &[("action", "push_fail")]);
                print_marker("send_attempt", &[("ok", "false")]);
                return RelaySendOutcome {
                    action: "push_fail".to_string(),
                    delivered: false,
                    error_code: Some(code),
                };
            }
        }
    }

    let mut stream = match TcpStream::connect(relay) {
        Ok(s) => s,
        Err(_) => {
            emit_marker("relay_event", None, &[("action", "connect_fail")]);
            print_marker("send_attempt", &[("ok", "false")]);
            return RelaySendOutcome {
                action: "connect_fail".to_string(),
                delivered: false,
                error_code: Some("relay_connect_failed"),
            };
        }
    };
    let frame = RelayFrame {
        to: to.to_string(),
        data: ciphertext,
    };
    if write_frame(&mut stream, &frame).is_err() {
        emit_marker("relay_event", None, &[("action", "write_fail")]);
        print_marker("send_attempt", &[("ok", "false")]);
        return RelaySendOutcome {
            action: "write_fail".to_string(),
            delivered: false,
            error_code: Some("relay_send_failed"),
        };
    }
    let resp = match read_frame::<RelayResponse>(&mut stream) {
        Ok(v) => v,
        Err(_) => {
            emit_marker("relay_event", None, &[("action", "read_fail")]);
            print_marker("send_attempt", &[("ok", "false")]);
            return RelaySendOutcome {
                action: "read_fail".to_string(),
                delivered: false,
                error_code: Some("relay_send_failed"),
            };
        }
    };
    emit_marker("relay_event", None, &[("action", resp.action.as_str())]);
    if !resp.delivered {
        print_marker("send_attempt", &[("ok", "false")]);
        return RelaySendOutcome {
            action: resp.action,
            delivered: false,
            error_code: Some("relay_delivery_failed"),
        };
    }

    finalize_send_commit(
        &dir,
        source,
        &outbox_path,
        resp.action,
        Some((to, pack.next_state)),
        Some(TimelineSendIngest {
            peer: to,
            byte_len: payload.len(),
            kind: "file",
            message_id: receipt_msg_id.as_deref(),
        }),
    )
}

fn finalize_send_commit(
    dir: &Path,
    source: ConfigSource,
    outbox_path: &Path,
    action: String,
    session_update: Option<(&str, Suite2SessionState)>,
    timeline_ingest: Option<TimelineSendIngest<'_>>,
) -> RelaySendOutcome {
    let next_seq = match read_send_state(dir, source) {
        Ok(v) => v + 1,
        Err(()) => {
            emit_marker("error", Some("send_state_parse_failed"), &[]);
            return RelaySendOutcome {
                action,
                delivered: true,
                error_code: Some("send_state_parse_failed"),
            };
        }
    };
    if let Some((peer, st)) = session_update {
        if qsp_session_store(peer, &st).is_err() {
            emit_marker("error", Some("qsp_session_store_failed"), &[]);
            return RelaySendOutcome {
                action,
                delivered: true,
                error_code: Some("qsp_session_store_failed"),
            };
        }
    }
    if let Some(ingest) = timeline_ingest {
        if let Err(code) = timeline_append_entry(
            ingest.peer,
            "out",
            ingest.byte_len,
            ingest.kind,
            MessageState::Sent,
            ingest.message_id,
        ) {
            emit_message_state_reject("<redacted>", code);
            emit_marker("error", Some(code), &[("op", "timeline_send_ingest")]);
        }
    }
    let state_bytes = format!("send_seq={}\n", next_seq).into_bytes();
    if write_atomic(&dir.join(SEND_STATE_NAME), &state_bytes, source).is_err() {
        emit_marker("error", Some("send_commit_write_failed"), &[]);
        return RelaySendOutcome {
            action,
            delivered: true,
            error_code: Some("send_commit_write_failed"),
        };
    }
    if fs::remove_file(outbox_path).is_err() {
        emit_marker("error", Some("outbox_remove_failed"), &[]);
        return RelaySendOutcome {
            action,
            delivered: true,
            error_code: Some("outbox_remove_failed"),
        };
    }
    print_marker("send_attempt", &[("ok", "true")]);
    let seq_s = next_seq.to_string();
    print_marker("send_commit", &[("send_seq", seq_s.as_str())]);
    RelaySendOutcome {
        action,
        delivered: true,
        error_code: None,
    }
}

fn read_frame<T: for<'de> Deserialize<'de>>(stream: &mut TcpStream) -> Result<T, ()> {
    let mut len_buf = [0u8; 4];
    stream.read_exact(&mut len_buf).map_err(|_| ())?;
    let len = u32::from_be_bytes(len_buf) as usize;
    if len == 0 || len > 1_048_576 {
        return Err(());
    }
    let mut buf = vec![0u8; len];
    stream.read_exact(&mut buf).map_err(|_| ())?;
    serde_json::from_slice(&buf).map_err(|_| ())
}

fn write_frame<T: Serialize>(stream: &mut TcpStream, value: &T) -> Result<(), ()> {
    let bytes = serde_json::to_vec(value).map_err(|_| ())?;
    let len = bytes.len();
    if len > u32::MAX as usize {
        return Err(());
    }
    let len_buf = (len as u32).to_be_bytes();
    stream.write_all(&len_buf).map_err(|_| ())?;
    stream.write_all(&bytes).map_err(|_| ())?;
    Ok(())
}

fn read_send_state(dir: &Path, source: ConfigSource) -> Result<u64, ()> {
    let path = dir.join(SEND_STATE_NAME);
    if let Err(e) = enforce_safe_parents(&path, source) {
        print_error(e);
    }
    if !path.exists() {
        return Ok(0);
    }
    let mut f = File::open(&path).map_err(|_| ())?;
    let mut buf = String::new();
    f.read_to_string(&mut buf).map_err(|_| ())?;
    for line in buf.lines() {
        if let Some(rest) = line.trim().strip_prefix("send_seq=") {
            let v = rest.trim().parse::<u64>().map_err(|_| ())?;
            return Ok(v);
        }
    }
    Err(())
}

fn qsc_mark(event: &str, code: &str) {
    emit_marker(event, Some(code), &[]);
}

fn qsc_sanitize_terminal_text(input: &str) -> String {
    // Terminal-safe deterministic sanitizer:
    // - drop ESC (0x1b) and ASCII control chars (except \n and \t)
    // - drop DEL (0x7f)
    let mut out = String::with_capacity(input.len());
    let mut it = input.chars().peekable();
    let mut in_csi = false;
    while let Some(ch) = it.next() {
        let c = ch as u32;
        if in_csi {
            // ANSI CSI sequences end at a final byte in the range 0x40-0x7E.
            if (0x40..=0x7e).contains(&c) {
                in_csi = false;
            }
            continue;
        }
        if c == 0x1b || c == 0x7f {
            // If this is a CSI introducer, skip until its final byte.
            if let Some('[') = it.peek().copied() {
                let _ = it.next();
                in_csi = true;
            }
            continue;
        }
        if ch == '\n' || ch == '\t' {
            out.push(ch);
            continue;
        }
        if c < 0x20 {
            continue;
        }
        if ch.is_control() {
            continue;
        }
        out.push(ch);
    }
    out
}

fn util_sanitize(print: Option<Vec<String>>) {
    let Some(parts) = print else {
        qsc_mark("util_sanitize", "usage");
        eprintln!("usage: qsc util sanitize --print <text>");
        process::exit(2);
    };
    let raw = parts.join(" ");
    let sanitized = qsc_sanitize_terminal_text(&raw);
    println!("{}", redact_text_for_output(&sanitized));
    qsc_mark("util_sanitize", "ok");
}

fn util_panic_demo() {
    panic!("panic_demo {}", PANIC_DEMO_SENTINEL);
}

struct BoundedQueue<T> {
    max: usize,
    items: VecDeque<T>,
}

impl<T> BoundedQueue<T> {
    fn new(max: usize) -> Self {
        Self {
            max,
            items: VecDeque::new(),
        }
    }

    fn push(&mut self, item: T) -> Result<(), ()> {
        if self.items.len() >= self.max {
            return Err(());
        }
        self.items.push_back(item);
        Ok(())
    }
}

fn util_queue(len: usize) {
    let mut q = BoundedQueue::new(MAX_QUEUE_LEN);
    for i in 0..len {
        if q.push(i).is_err() {
            print_error_marker("queue_limit_exceeded");
        }
    }
    print_marker("queue_limit", &[("ok", "true")]);
}

fn util_history(len: usize) {
    let mut h = BoundedQueue::new(MAX_HISTORY_LEN);
    for i in 0..len {
        if h.push(i).is_err() {
            print_error_marker("history_limit_exceeded");
        }
    }
    print_marker("history_limit", &[("ok", "true")]);
}

fn bounded_retry<F>(mut attempts: u32, mut op: F) -> Result<u32, ()>
where
    F: FnMut() -> Result<(), ()>,
{
    let mut tried = 0;
    let mut backoff = RETRY_BASE_MS;
    while attempts > 0 {
        tried += 1;
        match op() {
            Ok(()) => return Ok(tried),
            Err(()) => {
                attempts -= 1;
                if attempts == 0 {
                    return Err(());
                }
                let jitter = (tried as u64 % (RETRY_JITTER_MS + 1)).min(RETRY_JITTER_MS);
                let sleep_ms = (backoff + jitter).min(RETRY_MAX_MS);
                std::thread::sleep(Duration::from_millis(sleep_ms));
                backoff = (backoff * 2).min(RETRY_MAX_MS);
            }
        }
    }
    Err(())
}

fn util_retry(fail: u32) {
    let mut remaining = fail;
    let res = bounded_retry(MAX_RETRY_ATTEMPTS, || {
        if remaining > 0 {
            remaining -= 1;
            Err(())
        } else {
            Ok(())
        }
    });
    match res {
        Ok(attempts) => {
            let attempts_s = attempts.to_string();
            print_marker("retry_bound", &[("attempts", attempts_s.as_str())]);
        }
        Err(()) => print_error_marker("retry_limit_exceeded"),
    }
}

fn util_timeout(wait_ms: u64, timeout_ms: u64) {
    let limit = timeout_ms.clamp(1, MAX_TIMEOUT_MS);
    if wait_ms > limit {
        print_error_marker("timeout_exceeded");
    }
    let elapsed_s = wait_ms.to_string();
    print_marker("timeout_ok", &[("elapsed_ms", elapsed_s.as_str())]);
}

fn util_envelope(
    tick_count: usize,
    interval_ms: u64,
    max_ticks: usize,
    max_bundle: usize,
    max_count: usize,
    payload_lens: Vec<usize>,
) {
    let ticks = match envelope::tick_schedule(tick_count, interval_ms, max_ticks) {
        Ok(v) => v,
        Err(e) => print_error_marker(e.code()),
    };
    let bundle = match envelope::pack_bundle(&payload_lens, max_bundle, max_count) {
        Ok(v) => v,
        Err(e) => print_error_marker(e.code()),
    };
    let ticks_s = ticks.len().to_string();
    let interval_s = interval_ms.to_string();
    let bucket_s = bundle.bucket_len.to_string();
    let total_s = bundle.total_len.to_string();
    let count_s = bundle.payload_lens.len().to_string();
    print_marker(
        "envelope_plan",
        &[
            ("ticks", ticks_s.as_str()),
            ("interval_ms", interval_s.as_str()),
            ("bucket_size", bucket_s.as_str()),
            ("bundle_len", total_s.as_str()),
            ("payload_count", count_s.as_str()),
        ],
    );
}

fn envelope_plan_ack(
    deterministic: bool,
    tick_count: usize,
    interval_ms: u64,
    max_ticks: usize,
    max_bundle: usize,
    max_count: usize,
    small_len: usize,
) {
    if !deterministic {
        print_error_marker("ack_plan_requires_deterministic");
    }
    let plan = match envelope::plan_ack(
        small_len,
        tick_count,
        interval_ms,
        max_ticks,
        max_bundle,
        max_count,
    ) {
        Ok(v) => v,
        Err(e) => print_error_marker(e.code()),
    };
    let tick = plan.ticks.first().copied().unwrap_or(0);
    let tick_s = tick.to_string();
    let bucket_s = plan.bundle.bucket_len.to_string();
    print_marker(
        "ack_plan",
        &[("size_class", bucket_s.as_str()), ("tick", tick_s.as_str())],
    );
}

fn normalize_profile(value: &str) -> Result<String, ErrorCode> {
    match value {
        "baseline" => Ok("baseline".to_string()),
        "strict" => Ok("strict".to_string()),
        _ => Err(ErrorCode::InvalidPolicyProfile),
    }
}

fn read_policy_profile(path: &Path) -> Result<Option<String>, ErrorCode> {
    if !path.exists() {
        return Ok(None);
    }
    let mut f = File::open(path).map_err(|_| ErrorCode::IoReadFailed)?;
    let mut buf = String::new();
    f.read_to_string(&mut buf)
        .map_err(|_| ErrorCode::IoReadFailed)?;
    for line in buf.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if let Some(rest) = line.strip_prefix("policy_profile=") {
            return match normalize_profile(rest.trim()) {
                Ok(v) => Ok(Some(v)),
                Err(_) => Err(ErrorCode::ParseFailed),
            };
        }
    }
    Err(ErrorCode::ParseFailed)
}

fn ensure_dir_secure(dir: &Path, source: ConfigSource) -> Result<(), ErrorCode> {
    enforce_safe_parents(dir, source)?;
    if !dir.exists() {
        fs::create_dir_all(dir).map_err(|_| ErrorCode::IoWriteFailed)?;
    }
    #[cfg(unix)]
    {
        enforce_dir_perms(dir)?;
    }
    Ok(())
}

fn write_config_atomic(path: &Path, value: &str, source: ConfigSource) -> Result<(), ErrorCode> {
    let content = format!("{}={}\n", POLICY_KEY, value);
    write_atomic(path, content.as_bytes(), source)
}

fn normalize_tui_autolock_minutes(value: &str) -> Result<u64, ErrorCode> {
    let minutes = value
        .trim()
        .parse::<u64>()
        .map_err(|_| ErrorCode::ParseFailed)?;
    if !(TUI_AUTOLOCK_MIN_MINUTES..=TUI_AUTOLOCK_MAX_MINUTES).contains(&minutes) {
        return Err(ErrorCode::ParseFailed);
    }
    Ok(minutes)
}

fn read_tui_autolock_minutes(path: &Path) -> Result<Option<u64>, ErrorCode> {
    if !path.exists() {
        return Ok(None);
    }
    let mut f = File::open(path).map_err(|_| ErrorCode::IoReadFailed)?;
    let mut buf = String::new();
    f.read_to_string(&mut buf)
        .map_err(|_| ErrorCode::IoReadFailed)?;
    for line in buf.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if let Some(rest) = line.strip_prefix("autolock_minutes=") {
            return normalize_tui_autolock_minutes(rest.trim()).map(Some);
        }
    }
    Err(ErrorCode::ParseFailed)
}

fn write_tui_autolock_minutes_atomic(
    path: &Path,
    minutes: u64,
    source: ConfigSource,
) -> Result<(), ErrorCode> {
    let content = format!("autolock_minutes={minutes}\n");
    write_atomic(path, content.as_bytes(), source)
}

fn load_tui_autolock_minutes() -> u64 {
    let Ok((dir, source)) = config_dir() else {
        return TUI_AUTOLOCK_DEFAULT_MINUTES;
    };
    let path = dir.join(TUI_AUTOLOCK_FILE_NAME);
    if enforce_safe_parents(&path, source).is_err() {
        return TUI_AUTOLOCK_DEFAULT_MINUTES;
    }
    let Ok(_lock) = lock_store_shared(&dir, source) else {
        return TUI_AUTOLOCK_DEFAULT_MINUTES;
    };
    match read_tui_autolock_minutes(&path) {
        Ok(Some(v)) => v,
        Ok(None) => TUI_AUTOLOCK_DEFAULT_MINUTES,
        Err(_) => TUI_AUTOLOCK_DEFAULT_MINUTES,
    }
}

fn persist_tui_autolock_minutes(minutes: u64) -> Result<(), &'static str> {
    if !(TUI_AUTOLOCK_MIN_MINUTES..=TUI_AUTOLOCK_MAX_MINUTES).contains(&minutes) {
        return Err("autolock_invalid_minutes");
    }
    let (dir, source) = config_dir().map_err(|_| "autolock_config_unavailable")?;
    let _lock = lock_store_exclusive(&dir, source).map_err(|_| "autolock_lock_failed")?;
    ensure_store_layout(&dir, source).map_err(|_| "autolock_config_unavailable")?;
    let path = dir.join(TUI_AUTOLOCK_FILE_NAME);
    write_tui_autolock_minutes_atomic(&path, minutes, source)
        .map_err(|_| "autolock_config_unavailable")
}

fn normalize_tui_poll_interval_seconds(value: &str) -> Result<u64, ErrorCode> {
    let seconds = value
        .trim()
        .parse::<u64>()
        .map_err(|_| ErrorCode::ParseFailed)?;
    if !(TUI_POLL_MIN_INTERVAL_SECONDS..=TUI_POLL_MAX_INTERVAL_SECONDS).contains(&seconds) {
        return Err(ErrorCode::ParseFailed);
    }
    Ok(seconds)
}

fn normalize_tui_poll_mode(value: &str) -> Result<TuiPollMode, ErrorCode> {
    match value.trim() {
        "adaptive" => Ok(TuiPollMode::Adaptive),
        "fixed" => Ok(TuiPollMode::Fixed),
        _ => Err(ErrorCode::ParseFailed),
    }
}

fn read_tui_polling_config(path: &Path) -> Result<Option<(TuiPollMode, u64)>, ErrorCode> {
    if !path.exists() {
        return Ok(None);
    }
    let mut f = File::open(path).map_err(|_| ErrorCode::IoReadFailed)?;
    let mut buf = String::new();
    f.read_to_string(&mut buf)
        .map_err(|_| ErrorCode::IoReadFailed)?;
    let mut mode = None;
    let mut interval = None;
    for line in buf.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if let Some(rest) = line.strip_prefix("poll_mode=") {
            mode = Some(normalize_tui_poll_mode(rest.trim())?);
            continue;
        }
        if let Some(rest) = line.strip_prefix("poll_interval_seconds=") {
            interval = Some(normalize_tui_poll_interval_seconds(rest.trim())?);
        }
    }
    let mode = mode.unwrap_or(TuiPollMode::Adaptive);
    let interval = interval.unwrap_or(TUI_POLL_DEFAULT_INTERVAL_SECONDS);
    Ok(Some((mode, interval)))
}

fn write_tui_polling_config_atomic(
    path: &Path,
    mode: TuiPollMode,
    interval_seconds: u64,
    source: ConfigSource,
) -> Result<(), ErrorCode> {
    let content = format!(
        "poll_mode={}\npoll_interval_seconds={}\n",
        mode.as_str(),
        interval_seconds
    );
    write_atomic(path, content.as_bytes(), source)
}

fn load_tui_polling_config() -> (TuiPollMode, u64) {
    let Ok((dir, source)) = config_dir() else {
        return (TuiPollMode::Adaptive, TUI_POLL_DEFAULT_INTERVAL_SECONDS);
    };
    let path = dir.join(TUI_POLL_FILE_NAME);
    if enforce_safe_parents(&path, source).is_err() {
        return (TuiPollMode::Adaptive, TUI_POLL_DEFAULT_INTERVAL_SECONDS);
    }
    let Ok(_lock) = lock_store_shared(&dir, source) else {
        return (TuiPollMode::Adaptive, TUI_POLL_DEFAULT_INTERVAL_SECONDS);
    };
    match read_tui_polling_config(&path) {
        Ok(Some((mode, interval))) => (mode, interval),
        Ok(None) => (TuiPollMode::Adaptive, TUI_POLL_DEFAULT_INTERVAL_SECONDS),
        Err(_) => (TuiPollMode::Adaptive, TUI_POLL_DEFAULT_INTERVAL_SECONDS),
    }
}

fn persist_tui_polling_config(
    mode: TuiPollMode,
    interval_seconds: u64,
) -> Result<(), &'static str> {
    if !(TUI_POLL_MIN_INTERVAL_SECONDS..=TUI_POLL_MAX_INTERVAL_SECONDS).contains(&interval_seconds)
    {
        return Err("poll_invalid_seconds");
    }
    let (dir, source) = config_dir().map_err(|_| "poll_config_unavailable")?;
    let _lock = lock_store_exclusive(&dir, source).map_err(|_| "poll_lock_failed")?;
    ensure_store_layout(&dir, source).map_err(|_| "poll_config_unavailable")?;
    let path = dir.join(TUI_POLL_FILE_NAME);
    write_tui_polling_config_atomic(&path, mode, interval_seconds, source)
        .map_err(|_| "poll_config_unavailable")
}

fn ensure_store_layout(dir: &Path, source: ConfigSource) -> Result<(), ErrorCode> {
    ensure_dir_secure(dir, source)?;
    let meta = dir.join(STORE_META_NAME);
    if meta.exists() {
        return Ok(());
    }
    write_atomic(&meta, STORE_META_TEMPLATE.as_bytes(), source)?;
    Ok(())
}

fn write_atomic(path: &Path, content: &[u8], source: ConfigSource) -> Result<(), ErrorCode> {
    let dir = path.parent().ok_or(ErrorCode::IoWriteFailed)?;
    enforce_safe_parents(path, source)?;
    #[cfg(unix)]
    if dir.exists() {
        enforce_dir_perms(dir)?;
    }
    let tmp_name = format!(
        "{}.tmp.{}",
        path.file_name().and_then(|v| v.to_str()).unwrap_or("tmp"),
        process::id()
    );
    let tmp_path = dir.join(tmp_name);
    let _ = fs::remove_file(&tmp_path);

    let mut f = OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(&tmp_path)
        .map_err(|_| ErrorCode::IoWriteFailed)?;
    #[cfg(unix)]
    enforce_file_perms(&tmp_path)?;
    f.write_all(content).map_err(|_| ErrorCode::IoWriteFailed)?;
    f.sync_all().map_err(|_| ErrorCode::IoWriteFailed)?;
    fs::rename(&tmp_path, path).map_err(|_| ErrorCode::IoWriteFailed)?;
    fsync_dir_best_effort(dir);
    Ok(())
}

fn enforce_safe_parents(path: &Path, source: ConfigSource) -> Result<(), ErrorCode> {
    // First pass: detect symlinks before any permission checks.
    let mut cur = PathBuf::new();
    for comp in path.components() {
        cur.push(comp);
        if cur.exists() {
            let md = fs::symlink_metadata(&cur).map_err(|_| ErrorCode::IoReadFailed)?;
            if md.file_type().is_symlink() {
                return Err(ErrorCode::UnsafePathSymlink);
            }
        } else {
            break;
        }
    }
    // Second pass: enforce parent permission safety (symlinks already ruled out).
    match source {
        ConfigSource::DefaultHome => {
            let mut cur = PathBuf::new();
            for comp in path.components() {
                cur.push(comp);
                if cur.exists() {
                    let md = fs::symlink_metadata(&cur).map_err(|_| ErrorCode::IoReadFailed)?;
                    #[cfg(unix)]
                    {
                        if md.is_dir() && perms_group_or_world_writable(&md) {
                            return Err(ErrorCode::UnsafeParentPerms);
                        }
                    }
                } else {
                    break;
                }
            }
        }
        ConfigSource::EnvOverride | ConfigSource::XdgConfigHome => {
            let root = if path.is_dir() {
                path
            } else {
                path.parent().unwrap_or(path)
            };
            if root.exists() {
                let md = fs::symlink_metadata(root).map_err(|_| ErrorCode::IoReadFailed)?;
                #[cfg(unix)]
                {
                    if md.is_dir() && perms_group_or_world_writable(&md) {
                        return Err(ErrorCode::UnsafeParentPerms);
                    }
                }
            }
        }
    }
    Ok(())
}

fn check_symlink_safe(path: &Path) -> bool {
    let mut cur = PathBuf::new();
    for comp in path.components() {
        cur.push(comp);
        if cur.exists() {
            match fs::symlink_metadata(&cur) {
                Ok(md) => {
                    if md.file_type().is_symlink() {
                        return false;
                    }
                }
                Err(_) => return false,
            }
        } else {
            break;
        }
    }
    true
}

fn check_parent_safe(path: &Path, source: ConfigSource) -> bool {
    let mut cur = PathBuf::new();
    match source {
        ConfigSource::DefaultHome => {
            for comp in path.components() {
                cur.push(comp);
                if cur.exists() {
                    match fs::symlink_metadata(&cur) {
                        Ok(md) => {
                            #[cfg(unix)]
                            {
                                if md.is_dir() && perms_group_or_world_writable(&md) {
                                    return false;
                                }
                            }
                        }
                        Err(_) => return false,
                    }
                } else {
                    break;
                }
            }
        }
        ConfigSource::EnvOverride | ConfigSource::XdgConfigHome => {
            let root = if path.is_dir() {
                path
            } else {
                path.parent().unwrap_or(path)
            };
            if root.exists() {
                match fs::symlink_metadata(root) {
                    Ok(md) => {
                        #[cfg(unix)]
                        {
                            if md.is_dir() && perms_group_or_world_writable(&md) {
                                return false;
                            }
                        }
                    }
                    Err(_) => return false,
                }
            }
        }
    }
    true
}

fn lock_store_exclusive(dir: &Path, source: ConfigSource) -> Result<LockGuard, ErrorCode> {
    enforce_safe_parents(dir, source)?;
    if !dir.exists() {
        fs::create_dir_all(dir).map_err(|_| ErrorCode::IoWriteFailed)?;
    }
    #[cfg(unix)]
    {
        enforce_dir_perms(dir)?;
    }
    let lock_path = dir.join(LOCK_FILE_NAME);
    enforce_safe_parents(&lock_path, source)?;
    let file = OpenOptions::new()
        .create(true)
        .truncate(false)
        .read(true)
        .write(true)
        .open(&lock_path)
        .map_err(|_| ErrorCode::LockOpenFailed)?;
    #[cfg(unix)]
    enforce_file_perms(&lock_path)?;
    #[cfg(unix)]
    LockGuard::lock(&file, LockMode::Exclusive)?;
    Ok(LockGuard { file })
}

fn lock_store_shared(dir: &Path, source: ConfigSource) -> Result<Option<LockGuard>, ErrorCode> {
    enforce_safe_parents(dir, source)?;
    if !dir.exists() {
        return Ok(None);
    }
    #[cfg(unix)]
    {
        enforce_dir_perms(dir)?;
    }
    let lock_path = dir.join(LOCK_FILE_NAME);
    enforce_safe_parents(&lock_path, source)?;
    let file = OpenOptions::new()
        .create(true)
        .truncate(false)
        .read(true)
        .write(true)
        .open(&lock_path)
        .map_err(|_| ErrorCode::LockOpenFailed)?;
    #[cfg(unix)]
    enforce_file_perms(&lock_path)?;
    #[cfg(unix)]
    LockGuard::lock(&file, LockMode::Shared)?;
    Ok(Some(LockGuard { file }))
}

fn probe_dir_writable(dir: &Path, timeout_ms: u64) -> bool {
    let tmp = dir.join(format!("probe.tmp.{}", process::id()));
    let start = Instant::now();
    let timeout = Duration::from_millis(timeout_ms.max(1));
    loop {
        let res = OpenOptions::new().create_new(true).write(true).open(&tmp);
        if let Ok(mut f) = res {
            let _ = f.write_all(b"");
            let _ = f.sync_all();
            let _ = fs::remove_file(&tmp);
            return true;
        }
        if start.elapsed() >= timeout {
            return false;
        }
        std::thread::sleep(Duration::from_millis(10));
    }
}

fn print_error(code: ErrorCode) -> ! {
    emit_marker("error", Some(code.as_str()), &[]);
    process::exit(1);
}

fn bool_str(v: bool) -> &'static str {
    if v {
        "true"
    } else {
        "false"
    }
}

#[cfg(unix)]
fn perms_group_or_world_writable(md: &fs::Metadata) -> bool {
    use std::os::unix::fs::PermissionsExt;
    let mode = md.permissions().mode();
    (mode & 0o022) != 0
}

#[cfg(unix)]
fn enforce_dir_perms(dir: &Path) -> Result<(), ErrorCode> {
    use std::os::unix::fs::PermissionsExt;
    let md = fs::symlink_metadata(dir).map_err(|_| ErrorCode::IoReadFailed)?;
    if md.file_type().is_symlink() {
        return Err(ErrorCode::UnsafePathSymlink);
    }
    let perms = md.permissions().mode() & 0o777;
    if perms != 0o700 {
        fs::set_permissions(dir, fs::Permissions::from_mode(0o700))
            .map_err(|_| ErrorCode::IoWriteFailed)?;
    }
    Ok(())
}

#[cfg(unix)]
fn enforce_file_perms(path: &Path) -> Result<(), ErrorCode> {
    use std::os::unix::fs::PermissionsExt;
    let md = fs::symlink_metadata(path).map_err(|_| ErrorCode::IoReadFailed)?;
    if md.file_type().is_symlink() {
        return Err(ErrorCode::UnsafePathSymlink);
    }
    let perms = md.permissions().mode() & 0o777;
    if perms != 0o600 {
        fs::set_permissions(path, fs::Permissions::from_mode(0o600))
            .map_err(|_| ErrorCode::IoWriteFailed)?;
    }
    Ok(())
}

#[cfg(not(unix))]
fn fsync_dir_best_effort(_dir: &Path) {}

#[cfg(unix)]
fn fsync_dir_best_effort(dir: &Path) {
    let _ = File::open(dir).and_then(|d| d.sync_all());
}

#[cfg(not(unix))]
fn set_umask_077() {}

#[cfg(unix)]
fn set_umask_077() {
    unsafe {
        umask(0o077);
    }
}

#[cfg(unix)]
extern "C" {
    fn umask(mask: u32) -> u32;
    fn flock(fd: i32, operation: i32) -> i32;
}

// Marker helpers (deterministic; no secrets)
fn print_marker(event: &str, kv: &[(&str, &str)]) {
    emit_marker(event, None, kv);
}
fn print_error_marker(code: &str) -> ! {
    emit_marker("error", Some(code), &[]);
    process::exit(1);
}

#[derive(Debug, Clone, Copy)]
enum MarkerFormat {
    Plain,
    Jsonl,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum MarkerRouting {
    Stdout,
    InApp,
}

static MARKER_ROUTING: AtomicU8 = AtomicU8::new(0);
static MARKER_QUEUE: OnceLock<Mutex<VecDeque<String>>> = OnceLock::new();

fn set_marker_routing(routing: MarkerRouting) {
    let value = match routing {
        MarkerRouting::Stdout => 0,
        MarkerRouting::InApp => 1,
    };
    MARKER_ROUTING.store(value, Ordering::SeqCst);
}

fn marker_routing() -> MarkerRouting {
    match MARKER_ROUTING.load(Ordering::SeqCst) {
        1 => MarkerRouting::InApp,
        _ => MarkerRouting::Stdout,
    }
}

fn marker_queue() -> &'static Mutex<VecDeque<String>> {
    MARKER_QUEUE.get_or_init(|| Mutex::new(VecDeque::new()))
}

#[derive(Clone, Copy)]
struct OutputPolicy {
    reveal: bool,
}

static OUTPUT_POLICY: OnceLock<OutputPolicy> = OnceLock::new();

fn init_output_policy(reveal: bool) {
    let _ = OUTPUT_POLICY.set(OutputPolicy { reveal });
}

fn output_policy() -> OutputPolicy {
    *OUTPUT_POLICY
        .get()
        .unwrap_or(&OutputPolicy { reveal: false })
}

fn marker_format() -> MarkerFormat {
    match env::var("QSC_MARK_FORMAT").ok().as_deref() {
        Some("jsonl") | Some("JSONL") => MarkerFormat::Jsonl,
        _ => MarkerFormat::Plain,
    }
}

fn emit_marker(event: &str, code: Option<&str>, kv: &[(&str, &str)]) {
    let line = format_marker_line(event, code, kv);
    match marker_routing() {
        MarkerRouting::Stdout => println!("{}", line),
        MarkerRouting::InApp => {
            let mut queue = marker_queue().lock().expect("marker queue lock");
            queue.push_back(line);
        }
    }
    log_marker(event, code, kv);
}

fn format_marker_line(event: &str, code: Option<&str>, kv: &[(&str, &str)]) -> String {
    match marker_format() {
        MarkerFormat::Plain => {
            let mut line = format!("QSC_MARK/1 event={}", event);
            if let Some(c) = code {
                line.push_str(&format!(" code={}", c));
            }
            for (k, v) in kv {
                let rv = redact_value_for_output(k, v);
                line.push_str(&format!(" {}={}", k, rv));
            }
            line
        }
        MarkerFormat::Jsonl => {
            let mut obj = Map::new();
            obj.insert("v".to_string(), serde_json::Value::from(MARKER_SCHEMA_V1));
            obj.insert("event".to_string(), serde_json::Value::from(event));
            if let Some(c) = code {
                obj.insert("code".to_string(), serde_json::Value::from(c));
            }
            if !kv.is_empty() {
                let mut kv_map = Map::new();
                for (k, v) in kv {
                    kv_map.insert(
                        (*k).to_string(),
                        serde_json::Value::from(redact_value_for_output(k, v)),
                    );
                }
                obj.insert("kv".to_string(), serde_json::Value::Object(kv_map));
            }
            serde_json::Value::Object(obj).to_string()
        }
    }
}

fn redact_value_for_output(key: &str, value: &str) -> String {
    if output_policy().reveal {
        return value.to_string();
    }
    if should_redact_value(key, value) {
        return "<redacted>".to_string();
    }
    value.to_string()
}

fn redact_text_for_output(value: &str) -> String {
    if output_policy().reveal {
        return value.to_string();
    }
    if should_redact_value("", value) {
        return "<redacted>".to_string();
    }
    value.to_string()
}

fn redact_value_for_log(key: &str, value: &str) -> String {
    if should_redact_value(key, value) {
        return "<redacted>".to_string();
    }
    value.to_string()
}

fn should_redact_value(key: &str, value: &str) -> bool {
    let k = key.to_ascii_lowercase();
    if k == "checked_dir" || k == "peer_fp" || k == "fp" || k == "pinned_fp" || k == "seen_fp" {
        return false;
    }
    if k == "value"
        || k == "config_dir"
        || k.contains("passphrase")
        || k.contains("secret")
        || k.contains("token")
        || k == "path"
        || k == "url"
        || k == "endpoint"
        || k == "timestamp"
    {
        return true;
    }
    looks_like_url(value) || looks_like_timestamp(value) || looks_high_cardinality(value)
}

fn looks_like_url(value: &str) -> bool {
    let v = value.to_ascii_lowercase();
    v.contains("http://") || v.contains("https://")
}

fn looks_like_timestamp(value: &str) -> bool {
    let v = value.as_bytes();
    if v.len() < 19 {
        return false;
    }
    value.contains('T') && value.contains(':') && value.contains('-')
}

fn looks_high_cardinality(value: &str) -> bool {
    value.len() >= 24 && value.chars().any(|c| c.is_ascii_digit())
}

fn log_marker(event: &str, code: Option<&str>, kv: &[(&str, &str)]) {
    if env::var("QSC_LOG").ok().as_deref() != Some("1") {
        return;
    }
    let path = match env::var("QSC_LOG_PATH").ok() {
        Some(p) if !p.is_empty() => PathBuf::from(p),
        _ => return,
    };

    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }

    let mut obj = Map::new();
    obj.insert("v".to_string(), serde_json::Value::from(MARKER_SCHEMA_V1));
    obj.insert("event".to_string(), serde_json::Value::from(event));
    if let Some(c) = code {
        obj.insert("code".to_string(), serde_json::Value::from(c));
    }
    if !kv.is_empty() {
        let mut kv_map = Map::new();
        for (k, v) in kv {
            kv_map.insert(
                (*k).to_string(),
                serde_json::Value::from(redact_value_for_log(k, v)),
            );
        }
        obj.insert("kv".to_string(), serde_json::Value::Object(kv_map));
    }
    obj.insert("redacted".to_string(), serde_json::Value::from(true));

    let line = serde_json::Value::Object(obj).to_string() + "\n";
    let _ = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .and_then(|mut f| f.write_all(line.as_bytes()));
}

fn write_doctor_export(path: &Path, report: &DoctorReport) -> Result<(), ErrorCode> {
    let dir = path.parent().ok_or(ErrorCode::IoWriteFailed)?;
    let payload = serde_json::to_vec(report).map_err(|_| ErrorCode::IoWriteFailed)?;
    let tmp = dir.join(format!(
        "{}.tmp.{}",
        path.file_name()
            .and_then(|v| v.to_str())
            .unwrap_or("doctor"),
        process::id()
    ));
    let _ = fs::remove_file(&tmp);
    fs::create_dir_all(dir).map_err(|_| ErrorCode::IoWriteFailed)?;

    let mut f = OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(&tmp)
        .map_err(|_| ErrorCode::IoWriteFailed)?;
    f.write_all(&payload)
        .map_err(|_| ErrorCode::IoWriteFailed)?;
    f.sync_all().map_err(|_| ErrorCode::IoWriteFailed)?;
    fs::rename(&tmp, path).map_err(|_| ErrorCode::IoWriteFailed)?;
    fsync_dir_best_effort(dir);
    Ok(())
}

#[cfg(test)]
mod message_state_tests {
    use super::{message_state_transition_allowed, MessageState};

    #[test]
    fn failed_state_is_terminal() {
        let err =
            message_state_transition_allowed(MessageState::Failed, MessageState::Delivered, "out")
                .expect_err("FAILED must be terminal");
        assert_eq!(err, "failed_terminal");
    }

    #[test]
    fn out_state_cannot_skip_to_delivered() {
        let err =
            message_state_transition_allowed(MessageState::Created, MessageState::Delivered, "out")
                .expect_err("CREATED -> DELIVERED must reject");
        assert_eq!(err, "state_invalid_transition");
    }

    #[test]
    fn in_state_cannot_transition_to_delivered() {
        let err =
            message_state_transition_allowed(MessageState::Received, MessageState::Delivered, "in")
                .expect_err("RECEIVED -> DELIVERED must reject for inbound timeline");
        assert_eq!(err, "state_invalid_transition");
    }
}
