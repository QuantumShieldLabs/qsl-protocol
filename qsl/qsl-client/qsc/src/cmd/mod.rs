use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "qsc", version, about = "QSC client (Phase 2 scaffold)")]
pub struct Cli {
    /// Reveal sensitive output (non-default; demos should keep redaction).
    #[arg(long, global = true)]
    pub reveal: bool,
    /// Explicit unlock source for this invocation (default is locked).
    #[arg(long, global = true, value_name = "PATH")]
    pub unlock_passphrase_file: Option<PathBuf>,
    /// Desktop bridge compatibility only; operators should use --unlock-passphrase-file.
    #[arg(long, global = true, value_name = "ENV", hide = true)]
    pub unlock_passphrase_env: Option<String>,
    #[command(subcommand)]
    pub cmd: Option<Cmd>,
}

#[derive(Subcommand, Debug)]
pub enum Cmd {
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
        /// Exact total padded body bytes, including reserved closure overhead (maximum 60000).
        #[arg(long, value_name = "BYTES")]
        pad_to: Option<usize>,
        /// Padding profile; Auto resolves saved baseline/strict to Standard/Private at enqueue.
        #[arg(long, value_enum)]
        pad_bucket: Option<MetaPadBucket>,
        /// Unsupported directional seed option; explicit requests refuse.
        #[arg(long)]
        meta_seed: Option<u64>,
        /// Saved padding ceiling (default 4096, maximum 65536); impossible requests refuse.
        #[arg(long)]
        bucket_max: Option<usize>,
        /// Directional delivery receipts are mandatory: omit or select delivered.
        /// Explicit off refuses before queue or network effects.
        #[arg(long, value_enum)]
        receipt: Option<ReceiptRequest>,
    },
    /// Receive an inbound envelope (explicit-only).
    Receive {
        /// Transport selection (explicit-only).
        #[arg(long, value_enum)]
        transport: Option<SendTransport>,
        /// Relay base URL (http/https) for inbox transport.
        #[arg(long)]
        relay: Option<String>,
        /// Only retired is supported; explicit coexistence refuses in the directional profile.
        #[arg(long, value_enum)]
        legacy_receive_mode: Option<LegacyReceiveMode>,
        /// Unsupported in this directional slice; explicit attachment-service requests refuse.
        #[arg(long)]
        attachment_service: Option<String>,
        /// Protocol peer label/session key used for decrypt context.
        #[arg(long)]
        from: Option<String>,
        /// Relay inbox route token override (default: account inbox route token).
        #[arg(long)]
        mailbox: Option<String>,
        /// Max items to pull (bounded).
        #[arg(long)]
        max: Option<usize>,
        /// Unsupported file-transfer option in the directional profile; explicit requests refuse.
        #[arg(long)]
        max_file_size: Option<usize>,
        /// Unsupported file-transfer option in the directional profile; explicit requests refuse.
        #[arg(long)]
        max_file_chunks: Option<usize>,
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
        /// Unsupported directional bucketing option; explicit requests refuse.
        #[arg(long)]
        bucket_max: Option<usize>,
        /// Unsupported directional seed option; explicit requests refuse.
        #[arg(long)]
        meta_seed: Option<u64>,
        /// Select the directional profile's mandatory immediate delivery receipt.
        #[arg(long, value_enum)]
        emit_receipts: Option<ReceiptKind>,
        /// Directional receipts are mandatory and immediate; off/batched explicitly refuse.
        #[arg(long, value_enum)]
        receipt_mode: Option<ReceiptMode>,
        /// Unsupported directional receipt batching option; explicit requests refuse.
        #[arg(long, value_name = "MS")]
        receipt_batch_window_ms: Option<u64>,
        /// Unsupported directional receipt ordering option; explicit requests refuse.
        #[arg(long, value_name = "MS")]
        receipt_jitter_ms: Option<u64>,
        /// Unsupported directional file-confirmation option; explicit requests refuse.
        #[arg(long, value_enum)]
        file_confirm_mode: Option<FileConfirmMode>,
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
    /// NA-0681 (D616): invite create / redeem / accept — the contact-add crypto.
    Invite {
        #[command(subcommand)]
        cmd: InviteCmd,
    },
    Contacts {
        #[command(subcommand)]
        cmd: ContactsCmd,
    },
    /// Encrypted timeline store/list/show/clear.
    Timeline {
        #[command(subcommand)]
        cmd: TimelineCmd,
    },
    /// NA-0682: the durable message queue — status, retry, and the named discard.
    Outbox {
        #[command(subcommand)]
        cmd: OutboxCmd,
    },
    /// NA-0689: items the client kept instead of destroying — list and delete.
    Quarantine {
        #[command(subcommand)]
        cmd: QuarantineCmd,
    },
    /// File transfer MVP (bounded + integrity checked).
    File {
        #[command(subcommand)]
        cmd: FileCmd,
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
pub enum SendCmd {
    /// Abort a pending send by clearing the outbox (idempotent).
    Abort,
}

#[derive(Subcommand, Debug)]
pub enum FileCmd {
    /// Send a file transfer bundle using bounded chunks and manifest integrity.
    Send {
        /// Transport selection (explicit-only).
        #[arg(long, value_enum)]
        transport: Option<SendTransport>,
        /// Relay base URL (http/https) for inbox transport.
        #[arg(long)]
        relay: Option<String>,
        /// Attachment service base URL override/diagnostic for the streaming attachment path (by itself it does not activate the validated post-`w0` default; `QSC_ATTACHMENT_SERVICE` does).
        #[arg(long)]
        attachment_service: Option<String>,
        /// Legacy in-message stage for <=4 MiB sends (`w2` is the validated post-`w0` default once `QSC_ATTACHMENT_SERVICE` is set; `w0`/`w1` no longer restore coexistence there).
        #[arg(long, value_enum)]
        legacy_in_message_stage: Option<LegacyInMessageStage>,
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
        #[arg(long)]
        max_file_size: Option<usize>,
        /// Maximum chunks per transfer (bounded).
        #[arg(long)]
        max_chunks: Option<usize>,
        /// Request peer completion confirmation (coarse file receipt; explicit-only).
        #[arg(long, value_enum)]
        receipt: Option<ReceiptKind>,
    },
}

#[derive(ValueEnum, Debug, Clone, Copy)]
pub enum SendTransport {
    Relay,
}

#[derive(ValueEnum, Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReceiptKind {
    Delivered,
}

/// Directional delivery receipts are mandatory. Omission and Delivered select
/// the fixed profile; Off remains parseable so the caller receives an explicit
/// unsupported-policy refusal before any queue/network effects.
#[derive(ValueEnum, Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReceiptRequest {
    Off,
    Delivered,
}

#[derive(ValueEnum, Debug, Clone, Copy, PartialEq, Eq)]
pub enum LegacyInMessageStage {
    W0,
    W1,
    W2,
}

#[derive(ValueEnum, Debug, Clone, Copy, PartialEq, Eq)]
pub enum LegacyReceiveMode {
    Coexistence,
    Retired,
}

// NA-0770 (D-1411): `AckMode` IS RETIRED. It was introduced at NA-0644 (D580, ENG-0040) as a
// two-variant selector whose `Legacy` arm was the delete-on-pull contract; NA-0688 C4 (D622)
// flipped the default to `Lease`, and this lane removes the mode outright.
//
// ⚠ THE RECORD'S OWN REASON, because the enum's birth comment claimed the opposite: Legacy was
// never designed. It was the NAME GIVEN TO PRE-EXISTING BEHAVIOUR at the moment acknowledged-pull
// was added opt-in — "byte-identical to the pre-lane behavior", in that comment's own words —
// kept as the default so the lane could ship without a behaviour change.
//
// ⚠ E2's HONEST WORDING: delete-on-pull is gone FROM THE CLIENT'S INTENT, NOT FROM THE SYSTEM.
// The relay's default arm is still `None => PullMode::Legacy` for any request omitting
// `ack=lease`, and a pre-durability relay ignores the parameter entirely. This client can no
// longer REQUEST it; that is a different and smaller claim than the hazard being gone.
//
// The `--ack-mode` flag is removed with it, so the parser itself rejects it. The config KEY is
// TOMBSTONED rather than deleted — see `fs_store::AckModeConfigState`.
#[derive(ValueEnum, Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReceiptMode {
    Off,
    Batched,
    Immediate,
}

#[derive(ValueEnum, Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileConfirmMode {
    Off,
    CompleteOnly,
}

#[derive(ValueEnum, Debug, Clone, Copy)]
pub enum MetaPadBucket {
    Standard,
    Enhanced,
    Private,
    Auto,
}

#[derive(ValueEnum, Debug, Clone, Copy, PartialEq, Eq)]
pub enum HandshakeSuiteMode {
    LegacyCompat,
    SuiteRequired,
}

#[derive(Subcommand, Debug)]
pub enum HandshakeCmd {
    /// Initiate a handshake (A1) to a peer inbox.
    Init {
        /// Local label (defaults to "self"; the canonical single self-identity).
        #[arg(long = "as", value_name = "LABEL", default_value = "self")]
        as_label: String,
        /// Peer label.
        #[arg(long, value_name = "LABEL")]
        peer: String,
        /// Relay base URL for inbox transport.
        #[arg(long)]
        relay: String,
        /// QHSM suite-id admission mode.
        #[arg(long, value_enum, default_value_t = HandshakeSuiteMode::LegacyCompat)]
        suite_mode: HandshakeSuiteMode,
    },
    /// Poll inbox and process handshake messages.
    Poll {
        /// Local label. ⚠ NA-0711 (D647 A4): OMIT IT and the client derives the config dir's
        /// single self-identity; give it and it must agree with that identity or the poll fails
        /// closed BEFORE the relay pull.
        #[arg(long = "as", value_name = "LABEL")]
        as_label: Option<String>,
        /// Peer label.
        #[arg(long, value_name = "LABEL")]
        peer: String,
        /// Relay base URL for inbox transport.
        #[arg(long)]
        relay: String,
        /// Max items to pull (bounded).
        #[arg(long, default_value_t = 4)]
        max: usize,
        /// QHSM suite-id admission mode.
        #[arg(long, value_enum, default_value_t = HandshakeSuiteMode::LegacyCompat)]
        suite_mode: HandshakeSuiteMode,
    },
    /// Show handshake status.
    Status {
        /// Peer label (optional; default peer-0).
        #[arg(long, value_name = "LABEL")]
        peer: Option<String>,
    },
}

#[derive(Subcommand, Debug)]
pub enum IdentityCmd {
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
pub enum PeersCmd {
    /// List pinned peers and fingerprints.
    List,
}

#[derive(Subcommand, Debug)]
pub enum ContactsCmd {
    /// Add or update a contact pin.
    Add {
        #[arg(long, value_name = "LABEL")]
        label: String,
        #[arg(long, value_name = "FINGERPRINT")]
        fp: String,
        /// NA-0633 (ENG-0038): the peer's full identity KEM public key (hex), verified against
        /// FINGERPRINT. Required for the initiator to authenticate this peer as the responder.
        #[arg(long, value_name = "KEM_PK_HEX")]
        kem_pk: Option<String>,
        /// NA-0634 (D571): the peer's full identity SIGNING public key (hex). With --kem-pk, the pair is
        /// verified against FINGERPRINT = fingerprint(kem_pk, sig_pk) at add-time and populates the
        /// responder sig-pin (sig_fp), so the initiator authenticates the responder's signing identity.
        #[arg(long, value_name = "SIG_PK_HEX")]
        sig_pk: Option<String>,
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
    /// Per-device contact operations.
    Device {
        #[command(subcommand)]
        cmd: ContactsDeviceCmd,
    },
    /// Trust onboarding policy mode.
    TrustMode {
        #[command(subcommand)]
        cmd: ContactsTrustModeCmd,
    },
    /// Inbound unknown-sender requests.
    Request {
        #[command(subcommand)]
        cmd: ContactsRequestCmd,
    },
}

#[derive(Subcommand, Debug)]
pub enum ContactsTrustModeCmd {
    /// Show current trust onboarding mode.
    Show,
    /// Set trust onboarding mode.
    Set {
        #[arg(long, value_enum)]
        mode: TrustMode,
    },
}

#[derive(ValueEnum, Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrustMode {
    Strict,
    Balanced,
}

#[derive(Subcommand, Debug)]
pub enum ContactsRequestCmd {
    /// List pending inbound requests.
    List,
    /// Accept an inbound request into contacts (still not trusted).
    Accept {
        #[arg(long, value_name = "LABEL")]
        label: String,
    },
    /// Ignore (drop) an inbound request.
    Ignore {
        #[arg(long, value_name = "LABEL")]
        label: String,
    },
    /// Block an inbound request alias.
    Block {
        #[arg(long, value_name = "LABEL")]
        label: String,
    },
}

#[derive(Subcommand, Debug)]
pub enum ContactsDeviceCmd {
    /// Add a device under an existing contact.
    Add {
        #[arg(long, value_name = "LABEL")]
        label: String,
        #[arg(long, value_name = "FINGERPRINT")]
        fp: String,
        #[arg(long, value_name = "ROUTE_TOKEN")]
        route_token: Option<String>,
    },
    /// List devices for a contact.
    List {
        #[arg(long, value_name = "LABEL")]
        label: String,
    },
    /// Show status for one device or all devices under a contact.
    Status {
        #[arg(long, value_name = "LABEL")]
        label: String,
        #[arg(long, value_name = "DEVICE_ID")]
        device: Option<String>,
    },
    /// Verify a specific device fingerprint code.
    Verify {
        #[arg(long, value_name = "LABEL")]
        label: String,
        #[arg(long, value_name = "DEVICE_ID")]
        device: String,
        #[arg(long, value_name = "FINGERPRINT")]
        fp: String,
    },
    /// Trust (pin) a specific device.
    Trust {
        #[arg(long, value_name = "LABEL")]
        label: String,
        #[arg(long, value_name = "DEVICE_ID")]
        device: String,
        #[arg(long)]
        confirm: bool,
    },
    /// Revoke a specific device.
    Revoke {
        #[arg(long, value_name = "LABEL")]
        label: String,
        #[arg(long, value_name = "DEVICE_ID")]
        device: String,
        #[arg(long)]
        confirm: bool,
    },
    /// Primary-device selection operations.
    Primary {
        #[command(subcommand)]
        cmd: ContactsDevicePrimaryCmd,
    },
}

#[derive(Subcommand, Debug)]
pub enum ContactsDevicePrimaryCmd {
    /// Set the primary device (primary_only routing target).
    Set {
        #[arg(long, value_name = "LABEL")]
        label: String,
        #[arg(long, value_name = "DEVICE_ID")]
        device: String,
        #[arg(long)]
        confirm: bool,
    },
    /// Show the current primary device.
    Show {
        #[arg(long, value_name = "LABEL")]
        label: String,
    },
}

/// NA-0689 P4: the MINIMAL inspection surface over the quarantine store.
///
/// ⚠ `show` and `export` are deliberately ABSENT and are out of scope. A quarantined item is
/// undelivered content the user never chose to receive; a verb that printed it would make this
/// store a way to read things the normal path refused, which is not what it is for.
#[derive(Subcommand, Debug)]
pub enum QuarantineCmd {
    /// List captured items as REDACTED METADATA — the names of fields, never their values.
    ///
    /// ⚠ No content is printed and none is printable: the summary type carries no accessor for
    /// the stored bytes.
    List,
    /// ⚠ DESTROY one specifically-identified quarantined item.
    ///
    /// A stored item must always be deletable — otherwise this lane would have replaced
    /// "destroyed without consent" with "kept without consent". Naming the exact id is required,
    /// following the `outbox discard` shape.
    ///
    /// ⚠ Unlike `outbox discard` this does NOT route through a ratchet barrier, and that is
    /// correct rather than an omission: an incoming item's message key was already consumed at
    /// `commit_unpack_state` before capture, so no advance is owed.
    Drop {
        /// The quarantine entry id, as shown by `quarantine list`.
        #[arg(long)]
        id: String,
    },
}

#[derive(Subcommand, Debug)]
pub enum OutboxCmd {
    /// Show each contact's queue as data, plus the honest one-line status.
    ///
    /// ⚠ §2h is claims-honesty, not UX: a paused queue must never read as a sending one,
    /// so each line names its cause ("unlock to send") rather than implying work in flight.
    Status,
    /// Drain now — the manual "Retry now" trigger (DESIGN §2).
    Retry {
        /// Relay base URL.
        #[arg(long)]
        relay: String,
    },
    /// ⚠ DESTROY one specifically-identified queued message.
    ///
    /// F2: recovery means drain or fail visibly, NEVER destroy — so this is deliberately
    /// off the generic recovery path and requires naming the exact message. It routes
    /// through the ratchet barrier (`retire_packed`), because discarding a packed message
    /// without committing its advance is nonce reuse.
    Discard {
        /// Contact label.
        #[arg(long)]
        to: String,
        /// The message id to discard.
        #[arg(long)]
        msg_id: String,
        /// Relay base URL (needed to commit the ratchet advance).
        #[arg(long)]
        relay: String,
        /// Required: destroying a user's message is never implicit.
        #[arg(long, default_value_t = false)]
        confirm: bool,
    },
}

#[derive(Subcommand, Debug)]
pub enum TimelineCmd {
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
pub enum RelayCmd {
    /// Run a local relay with deterministic fault injection.
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
        /// Saved padding ceiling (default 4096, maximum 65536); impossible requests refuse.
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
    /// Set relay auth bearer token (account secret; env token takes precedence).
    TokenSet {
        /// Bearer token value.
        #[arg(long, value_name = "TOKEN")]
        token: String,
    },
    /// Set path to a relay auth bearer token file (lowest-precedence source).
    TokenFileSet {
        /// Path to the token file.
        #[arg(long, value_name = "PATH")]
        path: PathBuf,
    },
    /// Set path to an explicit CA certificate (PEM) trusted IN ADDITION to the
    /// system and built-in roots (env CA file takes precedence).
    CaSet {
        /// Path to the PEM CA certificate or bundle.
        #[arg(long, value_name = "PATH")]
        path: PathBuf,
    },
    /// Clear the explicit relay CA certificate path.
    CaClear,
    /// Show whether an explicit relay CA certificate is configured (redacted).
    CaShow,
    /// Clear the relay auth bearer token (account secret).
    TokenClear,
    /// Show whether a relay auth bearer token is configured. Presence ONLY: the
    /// token is a secret, so -- unlike `ca-show` -- NO hash is emitted.
    TokenShow,
    /// Probe a relay's GET /v1/server-info: reachability, auth mode, and the
    /// advertised capabilities. A self-hoster diagnostic -- and the cleanest way
    /// to reproduce a GUI connection-panel issue without driving the GUI.
    ServerInfo {
        /// Relay address (https://host[:port]).
        #[arg(long)]
        relay: String,
    },
}

#[derive(Subcommand, Debug)]
pub enum MetaCmd {
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
        /// Unsupported directional bucketing option; explicit requests refuse.
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
pub enum ConfigCmd {
    /// Set a config key to a value.
    Set { key: String, value: String },
    /// Get a config key.
    Get { key: String },
}

#[derive(Subcommand, Debug)]
pub enum UtilCmd {
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
    /// Test-only: apply a synthetic receipt/confirm against local state (no network send).
    ReceiptApply {
        /// Peer alias to apply receipt against.
        #[arg(long)]
        peer: String,
        /// Receive channel/session label that carried the receipt.
        #[arg(long)]
        channel: String,
        /// Message id for delivered receipt acks.
        #[arg(long)]
        msg_id: Option<String>,
        /// File id for file completion confirms.
        #[arg(long)]
        file_id: Option<String>,
        /// Confirmation id for file completion confirms.
        #[arg(long)]
        confirm_id: Option<String>,
    },
}

#[derive(Subcommand, Debug)]
pub enum EnvelopeCmd {
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

/// NA-0681 (D616 §2i). `create`/`revoke`/`list` are Alice's side; `redeem` is Bob's;
/// `accept` and `finish` collect the two handshake legs. Every one of them takes the relay
/// explicitly, matching every other qsc command — the relay URL is per-invocation
/// configuration, never ambient state.
#[derive(Subcommand, Debug)]
pub enum InviteCmd {
    /// Mint an invite and print the QSLI-1- code.
    Create {
        /// Local label. ⚠ NA-0711 (D647 A4): OMIT IT and the client derives the config dir's
        /// single self-identity; give it and it must agree with that identity or the command
        /// fails closed. Was `--self-label`, which had no help text and no pin anywhere.
        #[arg(long = "as", value_name = "LABEL")]
        self_label: Option<String>,
        #[arg(long, value_name = "URL")]
        relay: String,
        /// Requested lifetime in seconds. The relay's advertised ceiling clamps this, and a
        /// clamp is a NORMAL outcome, never an error.
        #[arg(long, default_value_t = 259_200)]
        ttl_secs: u64,
    },
    /// List this account's invites and their states.
    List,
    /// Kill a slot. Recorded locally too, so a late redemption is refused even if the relay lies.
    Revoke {
        #[arg(long, value_name = "INVITE_ID")]
        invite_id: String,
    },
    /// Redeem a pasted code: verify, provision a PENDING contact, hand shake into the slot.
    Redeem {
        #[arg(long, value_name = "QSLI_CODE")]
        code: String,
        /// Local-only alias. Required, user-typed, never pre-populated.
        #[arg(long, value_name = "ALIAS")]
        alias: String,
        /// Local label. ⚠ NA-0711 (D647 A4): OMIT IT and the client derives the config dir's
        /// single self-identity; give it and it must agree with that identity or the command
        /// fails closed. Was `--self-label`, which had no help text and no pin anywhere.
        #[arg(long = "as", value_name = "LABEL")]
        self_label: Option<String>,
    },
    /// Collect the handshake left in one of our own invite slots and answer it.
    Accept {
        #[arg(long, value_name = "INVITE_ID")]
        invite_id: String,
        #[arg(long, value_name = "ALIAS")]
        alias: String,
        /// Local label. ⚠ NA-0711 (D647 A4): OMIT IT and the client derives the config dir's
        /// single self-identity; give it and it must agree with that identity or the command
        /// fails closed. Was `--self-label`, which had no help text and no pin anywhere.
        #[arg(long = "as", value_name = "LABEL")]
        self_label: Option<String>,
        #[arg(long, default_value_t = 1)]
        max: usize,
    },
    /// Collect the wrapped reply, learn the peer's real route token, finish the handshake.
    Finish {
        #[arg(long, value_name = "ALIAS")]
        alias: String,
        #[arg(long, value_name = "URL")]
        relay: String,
        /// Local label. ⚠ NA-0711 (D647 A4): OMIT IT and the client derives the config dir's
        /// single self-identity; give it and it must agree with that identity or the command
        /// fails closed. Was `--self-label`, which had no help text and no pin anywhere.
        #[arg(long = "as", value_name = "LABEL")]
        self_label: Option<String>,
        /// Scan bound for locating the invite reply; clamped to [16,128]
        #[arg(long, default_value_t = 1)]
        max: usize,
    },
}
