use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "qsc", version, about = "QSC client (Phase 2 scaffold)")]
pub(crate) struct Cli {
    /// Reveal sensitive output (non-default; demos should keep redaction).
    #[arg(long, global = true)]
    pub(crate) reveal: bool,
    /// Explicit unlock source for this invocation (default is locked).
    #[arg(long, global = true, value_name = "ENV")]
    pub(crate) unlock_passphrase_env: Option<String>,
    #[command(subcommand)]
    pub(crate) cmd: Option<Cmd>,
}

#[derive(Subcommand, Debug)]
pub(crate) enum Cmd {
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
        cmd: crate::vault::VaultCmd,
    },
    /// Send commit semantics (prepare→send→commit).
    Send {
        /// Subcommand for send (e.g., abort a pending outbox).
        #[command(subcommand)]
        cmd: Option<SendCmd>,
        /// Transport selection (explicit-only).
        #[arg(long, value_enum)]
        transport: Option<SendTransport>,
        /// Relay base URL (http/https) for transport=relay.
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
        /// Relay inbox route token override (default: account inbox route token).
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
pub(crate) enum SendCmd {
    /// Abort a pending send by clearing the outbox (idempotent).
    Abort,
}

#[derive(Subcommand, Debug)]
pub(crate) enum FileCmd {
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
        #[arg(long, default_value_t = crate::FILE_XFER_DEFAULT_CHUNK_SIZE)]
        chunk_size: usize,
        /// Maximum file size in bytes (bounded).
        #[arg(long, default_value_t = crate::FILE_XFER_DEFAULT_MAX_FILE_SIZE)]
        max_file_size: usize,
        /// Maximum chunks per transfer (bounded).
        #[arg(long, default_value_t = crate::FILE_XFER_DEFAULT_MAX_CHUNKS)]
        max_chunks: usize,
    },
}

#[derive(ValueEnum, Debug, Clone, Copy)]
pub(crate) enum SendTransport {
    Relay,
}

#[derive(ValueEnum, Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ReceiptKind {
    Delivered,
}

#[derive(ValueEnum, Debug, Clone, Copy)]
pub(crate) enum TuiTransport {
    Relay,
}

#[derive(ValueEnum, Debug, Clone, Copy)]
pub(crate) enum MetaPadBucket {
    Standard,
    Enhanced,
    Private,
    Auto,
}

#[derive(Subcommand, Debug)]
pub(crate) enum HandshakeCmd {
    /// Initiate a handshake (A1) to a peer inbox.
    Init {
        /// Local label (used for identity context).
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
        /// Local label (used for identity context).
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
pub(crate) enum IdentityCmd {
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
pub(crate) enum PeersCmd {
    /// List pinned peers and fingerprints.
    List,
}

#[derive(Subcommand, Debug)]
pub(crate) enum ContactsCmd {
    /// Add or update a contact pin.
    Add {
        #[arg(long, value_name = "LABEL")]
        label: String,
        #[arg(long, value_name = "FINGERPRINT")]
        fp: String,
        #[arg(long, value_name = "ROUTE_TOKEN")]
        route_token: Option<String>,
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
    /// Set/update a peer route token used for relay transport addressing.
    RouteSet {
        #[arg(long, value_name = "LABEL")]
        label: String,
        #[arg(long, value_name = "ROUTE_TOKEN")]
        route_token: String,
    },
}

#[derive(Subcommand, Debug)]
pub(crate) enum TimelineCmd {
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
pub(crate) enum RelayCmd {
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
    /// Set self inbox route token used for relay pull addressing.
    InboxSet {
        /// Route token value (URL-safe, opaque).
        #[arg(long, value_name = "ROUTE_TOKEN")]
        token: String,
    },
    /// Clear self inbox route token.
    InboxClear,
}

#[derive(Subcommand, Debug)]
pub(crate) enum MetaCmd {
    /// Plan deterministic metadata schedule (dry-run only; no network, no writes).
    Plan {
        /// Deterministic planning mode.
        #[arg(long)]
        deterministic: bool,
        /// Number of plan ticks.
        #[arg(long, default_value_t = crate::META_TICK_COUNT_DEFAULT)]
        tick_count: u32,
        /// Interval between ticks in ms.
        #[arg(long, default_value_t = crate::META_INTERVAL_MS_DEFAULT)]
        interval_ms: u64,
        /// Metadata bucket ceiling in bytes.
        #[arg(long, default_value_t = crate::META_BUCKET_MAX_DEFAULT)]
        bucket_max: usize,
        /// Max batch count per tick.
        #[arg(long, default_value_t = crate::META_BATCH_MAX_COUNT_DEFAULT)]
        batch_max_count: u32,
        /// Plan explicit cover traffic markers.
        #[arg(long)]
        cover_enabled: bool,
    },
}

#[derive(Subcommand, Debug)]
pub(crate) enum ConfigCmd {
    /// Set a config key to a value.
    Set { key: String, value: String },
    /// Get a config key.
    Get { key: String },
}

#[derive(Subcommand, Debug)]
pub(crate) enum UtilCmd {
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
        #[arg(long, default_value_t = crate::envelope::MAX_TICKS_DEFAULT)]
        max_ticks: usize,
        /// Maximum bundle size in bytes.
        #[arg(long, default_value_t = crate::envelope::MAX_BUNDLE_SIZE_DEFAULT)]
        max_bundle: usize,
        /// Maximum payload count per bundle.
        #[arg(long, default_value_t = crate::envelope::MAX_PAYLOAD_COUNT_DEFAULT)]
        max_count: usize,
        /// Payload lengths to pack (comma-separated).
        #[arg(long, value_delimiter = ',')]
        payload_lens: Vec<usize>,
    },
    /// Panic demo for lifecycle redaction verification.
    PanicDemo,
}

#[derive(Subcommand, Debug)]
pub(crate) enum EnvelopeCmd {
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
        #[arg(long, default_value_t = crate::envelope::MAX_TICKS_DEFAULT)]
        max_ticks: usize,
        /// Maximum bundle size in bytes.
        #[arg(long, default_value_t = crate::envelope::MAX_BUNDLE_SIZE_DEFAULT)]
        max_bundle: usize,
        /// Maximum payload count per bundle.
        #[arg(long, default_value_t = crate::envelope::MAX_PAYLOAD_COUNT_DEFAULT)]
        max_count: usize,
        /// Payload length that defines the small-message class.
        #[arg(long, default_value_t = 1)]
        small_len: usize,
    },
}
