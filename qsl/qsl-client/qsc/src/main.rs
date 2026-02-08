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
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Terminal,
};
use reqwest::blocking::Client as HttpClient;
use reqwest::StatusCode as HttpStatus;
use serde::{Deserialize, Serialize};
use serde_json::Map;
use std::collections::{BTreeSet, VecDeque};
use std::env;
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::process;
use std::sync::atomic::{AtomicU64, AtomicU8, Ordering};
use std::sync::{Mutex, OnceLock};
use std::time::{Duration, Instant};

#[derive(Parser, Debug)]
#[command(name = "qsc", version, about = "QSC client (Phase 2 scaffold)")]
struct Cli {
    /// Reveal sensitive output (non-default; demos should keep redaction).
    #[arg(long, global = true)]
    reveal: bool,
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
        /// Fixed polling interval (ms). Requires --poll-ticks and --poll-max-per-tick.
        #[arg(long, value_name = "MS")]
        poll_interval_ms: Option<u64>,
        /// Number of polling ticks (bounded).
        #[arg(long)]
        poll_ticks: Option<u32>,
        /// Max items per poll tick (bounded).
        #[arg(long)]
        poll_max_per_tick: Option<u32>,
        /// Deterministic metadata seed (explicit-only).
        #[arg(long)]
        meta_seed: Option<u64>,
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
}

#[derive(Subcommand, Debug)]
enum SendCmd {
    /// Abort a pending send by clearing the outbox (idempotent).
    Abort,
}

#[derive(ValueEnum, Debug, Clone, Copy)]
enum SendTransport {
    Relay,
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
fn main() {
    set_umask_077();
    let cli = Cli::parse();
    init_output_policy(cli.reveal);
    match cli.cmd {
        None => {
            // Shell-first UX expects help by default.
            println!("QSC_MARK/1 event=help_stub");
        }
        Some(Cmd::Status) => {
            print_marker("status", &[("ok", "true"), ("locked", "unknown")]);
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
            meta_seed,
        }) => match cmd {
            Some(SendCmd::Abort) => send_abort(),
            None => send_execute(transport, relay, to, file, pad_to, pad_bucket, meta_seed),
        },
        Some(Cmd::Receive {
            transport,
            relay,
            from,
            mailbox,
            max,
            out,
            file,
            poll_interval_ms,
            poll_ticks,
            poll_max_per_tick,
            meta_seed,
        }) => {
            if let Some(path) = file {
                if transport.is_some()
                    || relay.is_some()
                    || from.is_some()
                    || mailbox.is_some()
                    || max.is_some()
                    || out.is_some()
                    || poll_interval_ms.is_some()
                    || poll_ticks.is_some()
                    || poll_max_per_tick.is_some()
                    || meta_seed.is_some()
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
                    poll_interval_ms,
                    poll_ticks,
                    poll_max_per_tick,
                    meta_seed,
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
        RelayCmd::Send { to, file, relay } => relay_send(&to, &file, &relay, None, None),
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

fn tui_interactive(cfg: TuiConfig) -> std::io::Result<()> {
    set_marker_routing(MarkerRouting::InApp);
    let mut state = TuiState::new(cfg);
    emit_marker("tui_open", None, &[]);
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut input = String::new();

    let mut exit = false;
    let result = loop {
        state.drain_marker_queue();
        terminal.draw(|f| {
            draw_tui(f, &state, &input);
        })?;

        if event::poll(Duration::from_millis(200))? {
            if let Event::Key(key) = event::read()? {
                exit = handle_tui_key(&mut state, &mut input, key);
            }
        }
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

fn handle_tui_key(state: &mut TuiState, input: &mut String, key: KeyEvent) -> bool {
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
                } else {
                    let max_len = state.focus_max_len();
                    state.focus_scroll_move(-1, max_len);
                }
            }
            KeyCode::Down => {
                if state.mode == TuiMode::FocusContacts {
                    state.contacts_move(1);
                } else {
                    let max_len = state.focus_max_len();
                    state.focus_scroll_move(1, max_len);
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
        KeyCode::Esc => return true,
        KeyCode::F(1) | KeyCode::Char('?') => state.toggle_help_mode(),
        KeyCode::Enter => {
            if input.trim().is_empty() {
                state.enter_focus_mode(state.focus_mode_for_inspector());
            } else if let Some(cmd) = parse_tui_command(input) {
                let exit = handle_tui_command(&cmd, state);
                input.clear();
                return exit;
            } else if !input.is_empty() {
                emit_marker("tui_input_text", None, &[("kind", "plain")]);
            }
            input.clear();
        }
        KeyCode::Backspace => {
            input.pop();
        }
        KeyCode::Char(ch) => {
            input.push(ch);
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

fn parse_tui_script_key(spec: &str) -> Option<KeyEvent> {
    let normalized = spec.trim().to_ascii_lowercase();
    match normalized.as_str() {
        "esc" => Some(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE)),
        "enter" => Some(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE)),
        "up" => Some(KeyEvent::new(KeyCode::Up, KeyModifiers::NONE)),
        "down" => Some(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE)),
        "f2" => Some(KeyEvent::new(KeyCode::F(2), KeyModifiers::NONE)),
        "f3" => Some(KeyEvent::new(KeyCode::F(3), KeyModifiers::NONE)),
        "f4" => Some(KeyEvent::new(KeyCode::F(4), KeyModifiers::NONE)),
        "f5" => Some(KeyEvent::new(KeyCode::F(5), KeyModifiers::NONE)),
        "ctrl-f2" | "c-f2" => Some(KeyEvent::new(KeyCode::F(2), KeyModifiers::CONTROL)),
        "ctrl-f3" | "c-f3" => Some(KeyEvent::new(KeyCode::F(3), KeyModifiers::CONTROL)),
        "ctrl-f4" | "c-f4" => Some(KeyEvent::new(KeyCode::F(4), KeyModifiers::CONTROL)),
        "ctrl-f5" | "c-f5" => Some(KeyEvent::new(KeyCode::F(5), KeyModifiers::CONTROL)),
        _ => None,
    }
}

fn handle_tui_command(cmd: &TuiParsedCmd, state: &mut TuiState) -> bool {
    match cmd.cmd.as_str() {
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
                "status" => state.enter_focus_mode(TuiMode::FocusStatus),
                "session" => state.enter_focus_mode(TuiMode::FocusSession),
                "contacts" => state.enter_focus_mode(TuiMode::FocusContacts),
                _ => {
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
                "status" => state.set_inspector(TuiInspectorPane::Status),
                "session" => state.set_inspector(TuiInspectorPane::Session),
                "contacts" => state.set_inspector(TuiInspectorPane::Contacts),
                _ => emit_marker("tui_inspector_invalid", None, &[("reason", "unknown_pane")]),
            }
            false
        }
        "key" => {
            emit_marker("tui_cmd", None, &[("cmd", "key")]);
            let spec = cmd.args.first().map(|s| s.as_str()).unwrap_or("");
            if let Some(key) = parse_tui_script_key(spec) {
                let mut input = String::new();
                handle_tui_key(state, &mut input, key)
            } else {
                emit_marker("tui_key_invalid", None, &[("reason", "unknown_key")]);
                false
            }
        }
        "back" | "unfocus" => {
            emit_marker("tui_cmd", None, &[("cmd", "back")]);
            state.exit_focus_mode();
            false
        }
        "exit" | "quit" => {
            emit_marker("tui_cmd", None, &[("cmd", "exit")]);
            true
        }
        "send" => {
            emit_marker("tui_cmd", None, &[("cmd", "send")]);
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
            emit_marker("tui_cmd", None, &[("cmd", other)]);
            false
        }
    }
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
    if let Err(reason) = protocol_active_or_reason_for_peer(to) {
        emit_protocol_inactive(reason.as_str());
        state.update_send_lifecycle("blocked");
        return;
    }
    let payload = tui_payload_bytes(state.send_seq);
    state.send_seq = state.send_seq.wrapping_add(1);
    let outcome = relay_send_with_payload(
        to,
        payload,
        relay.relay.as_str(),
        fault_injector_from_tui(relay),
        None,
        None,
    );
    state.push_event("relay_event", outcome.action.as_str());
    if outcome.delivered {
        state.update_send_lifecycle("committed");
        state.session.sent_count = state.session.sent_count.saturating_add(1);
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

fn draw_tui(f: &mut ratatui::Frame, state: &TuiState, input: &str) {
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
        TuiMode::Normal => {}
    }
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(3), Constraint::Length(3)].as_ref())
        .split(area);
    let layout = state.home_layout_snapshot(area.width, area.height);
    if layout.contacts_shown {
        let cols = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Percentage(20),
                    Constraint::Percentage(53),
                    Constraint::Percentage(27),
                ]
                .as_ref(),
            )
            .split(rows[0]);
        render_contacts(f, cols[0], &state.contacts, &state.session);
        render_messages(f, cols[1], &state.messages);
        render_inspector(f, cols[2], state);
    } else {
        let cols = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
            .split(rows[0]);
        render_messages(f, cols[0], &state.messages);
        render_inspector(f, cols[1], state);
    }

    let cmd_title = if layout.header_compact {
        "Cmd"
    } else {
        "Command (/help, F2-5 inspector, Enter focus, Esc exit)"
    };
    let cmd = Paragraph::new(format!("> {}", input))
        .block(Block::default().borders(Borders::ALL).title(cmd_title));
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
    let items = state.events.iter().cloned().collect::<Vec<String>>();
    let start = state.focus_scroll.min(items.len());
    let body = items[start..].join("\n");
    let panel = Paragraph::new(body).block(
        Block::default()
            .borders(Borders::ALL)
            .title("FOCUS: EVENTS"),
    );
    f.render_widget(panel, area);
}

fn draw_focus_status(f: &mut ratatui::Frame, area: Rect, state: &TuiState) {
    let lines = [
        format!("fingerprint: {}", state.status.fingerprint),
        format!("peer_fp: {}", state.status.peer_fp),
        format!("qsp: {}", state.status.qsp),
        format!("envelope: {}", state.status.envelope),
        format!("send: {}", state.status.send_lifecycle),
    ];
    let start = state.focus_scroll.min(lines.len());
    let body = lines[start..].join("\n");
    let panel = Paragraph::new(body).block(
        Block::default()
            .borders(Borders::ALL)
            .title("FOCUS: STATUS"),
    );
    f.render_widget(panel, area);
}

fn draw_focus_session(f: &mut ratatui::Frame, area: Rect, state: &TuiState) {
    let lines = [
        format!("peer: {}", state.session.peer_label),
        format!("verified: {}", state.session.verified),
        format!("sent_count: {}", state.session.sent_count),
        format!("recv_count: {}", state.session.recv_count),
    ];
    let start = state.focus_scroll.min(lines.len());
    let body = lines[start..].join("\n");
    let panel = Paragraph::new(body).block(
        Block::default()
            .borders(Borders::ALL)
            .title("FOCUS: SESSION"),
    );
    f.render_widget(panel, area);
}

fn draw_focus_contacts(f: &mut ratatui::Frame, area: Rect, state: &TuiState) {
    let items: Vec<ListItem> = state
        .contacts
        .iter()
        .map(|c| ListItem::new(c.clone()))
        .collect();
    let mut list_state = ratatui::widgets::ListState::default();
    if !items.is_empty() {
        list_state.select(Some(state.contacts_selected.min(items.len() - 1)));
    }
    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("FOCUS: CONTACTS"),
        )
        .highlight_style(Style::default().add_modifier(Modifier::BOLD));
    f.render_stateful_widget(list, area, &mut list_state);
}

fn render_contacts(
    f: &mut ratatui::Frame,
    area: Rect,
    contacts: &[String],
    session: &TuiSession<'_>,
) {
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)].as_ref())
        .split(area);
    let items: Vec<ListItem> = contacts.iter().map(|c| ListItem::new(c.clone())).collect();
    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Contacts"))
        .style(Style::default());
    f.render_widget(list, rows[0]);
    render_session(f, rows[1], session);
}

fn render_messages(f: &mut ratatui::Frame, area: Rect, messages: &VecDeque<String>) {
    let body = messages.iter().cloned().collect::<Vec<String>>().join("\n");
    let panel =
        Paragraph::new(body).block(Block::default().borders(Borders::ALL).title("Timeline"));
    f.render_widget(panel, area);
}

struct TuiStatus<'a> {
    fingerprint: &'a str,
    peer_fp: &'a str,
    qsp: &'a str,
    envelope: &'a str,
    send_lifecycle: &'a str,
}

struct TuiSession<'a> {
    peer_label: &'a str,
    verified: bool,
    sent_count: u64,
    recv_count: u64,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum TuiMode {
    Normal,
    Help,
    FocusEvents,
    FocusStatus,
    FocusSession,
    FocusContacts,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum TuiInspectorPane {
    Events,
    Status,
    Session,
    Contacts,
}

const TUI_H3_WIDE_MIN: u16 = 120;
const TUI_H3_TALL_MIN: u16 = 28;
const TUI_INSPECTOR_EVENTS_MAX: usize = 6;
const TUI_INSPECTOR_CONTACTS_MAX: usize = 8;

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
    messages: VecDeque<String>,
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
    inspector: TuiInspectorPane,
}

impl TuiState {
    fn new(cfg: TuiConfig) -> Self {
        let contacts = vec!["peer-0".to_string()];
        let mut messages = VecDeque::new();
        messages.push_back("(no messages)".to_string());
        let mut events = VecDeque::new();
        let fingerprint = compute_local_fingerprint();
        let peer_fp = compute_peer_fingerprint("peer-0");
        let qsp_status = qsp_status_string("peer-0");
        let envelope = compute_envelope_status(0);
        let send_lifecycle = "idle".to_string();
        let status = TuiStatus {
            fingerprint: Box::leak(fingerprint.clone().into_boxed_str()),
            peer_fp: Box::leak(peer_fp.clone().into_boxed_str()),
            qsp: Box::leak(qsp_status.clone().into_boxed_str()),
            envelope: Box::leak(envelope.clone().into_boxed_str()),
            send_lifecycle: Box::leak(send_lifecycle.clone().into_boxed_str()),
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
        Self {
            contacts,
            messages,
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
            inspector: TuiInspectorPane::Status,
        }
    }

    fn last_payload_len(&self) -> usize {
        self.last_payload_len
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
        emit_marker(
            "tui_status_update",
            None,
            &[("field", "qsp"), ("value", &self.qsp_status)],
        );
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
    }

    fn push_event_line(&mut self, line: String) {
        self.events.push_back(line);
        if self.events.len() > 64 {
            self.events.pop_front();
        }
    }

    fn enter_help_mode(&mut self) {
        if matches!(
            self.mode,
            TuiMode::FocusEvents
                | TuiMode::FocusStatus
                | TuiMode::FocusSession
                | TuiMode::FocusContacts
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
            TuiMode::FocusStatus => "status",
            TuiMode::FocusSession => "session",
            TuiMode::FocusContacts => "contacts",
            _ => "dashboard",
        }
    }

    fn inspector_name(&self) -> &'static str {
        match self.inspector {
            TuiInspectorPane::Events => "events",
            TuiInspectorPane::Status => "status",
            TuiInspectorPane::Session => "session",
            TuiInspectorPane::Contacts => "contacts",
        }
    }

    fn set_inspector(&mut self, pane: TuiInspectorPane) {
        self.inspector = pane;
        emit_marker("tui_inspector", None, &[("pane", self.inspector_name())]);
    }

    fn focus_mode_for_inspector(&self) -> TuiMode {
        match self.inspector {
            TuiInspectorPane::Events => TuiMode::FocusEvents,
            TuiInspectorPane::Status => TuiMode::FocusStatus,
            TuiInspectorPane::Session => TuiMode::FocusSession,
            TuiInspectorPane::Contacts => TuiMode::FocusContacts,
        }
    }

    fn home_layout_snapshot(&self, cols: u16, rows: u16) -> HomeLayoutSnapshot {
        HomeLayoutSnapshot {
            contacts_shown: cols >= TUI_H3_WIDE_MIN,
            header_compact: rows < TUI_H3_TALL_MIN,
        }
    }

    fn emit_home_render_marker(&self, cols: u16, rows: u16) {
        if self.mode != TuiMode::Normal {
            return;
        }
        let layout = self.home_layout_snapshot(cols, rows);
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
            ],
        );
    }

    fn focus_render_count(&self, mode: TuiMode) -> usize {
        match mode {
            TuiMode::FocusEvents => self.events.len(),
            TuiMode::FocusContacts => self.contacts.len(),
            TuiMode::FocusStatus => 4,
            TuiMode::FocusSession => 4,
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
                | TuiMode::FocusStatus
                | TuiMode::FocusSession
                | TuiMode::FocusContacts
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
    }

    fn exit_focus_mode(&mut self) {
        if matches!(
            self.mode,
            TuiMode::FocusEvents
                | TuiMode::FocusStatus
                | TuiMode::FocusSession
                | TuiMode::FocusContacts
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
                | TuiMode::FocusStatus
                | TuiMode::FocusSession
                | TuiMode::FocusContacts
        )
    }

    fn focus_max_len(&self) -> usize {
        match self.mode {
            TuiMode::FocusEvents => self.events.len(),
            TuiMode::FocusContacts => self.contacts.len(),
            TuiMode::FocusStatus => 4,
            TuiMode::FocusSession => 4,
            _ => 0,
        }
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
    }

    fn contacts_move(&mut self, delta: i32) {
        if self.contacts.is_empty() {
            self.contacts_selected = 0;
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
    }

    fn drain_marker_queue(&mut self) {
        let mut queue = marker_queue().lock().expect("marker queue lock");
        while let Some(line) = queue.pop_front() {
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
            cmd: "inspector status|events|session|contacts",
            desc: "set home inspector pane",
        },
        TuiHelpItem {
            cmd: "focus events",
            desc: "focus Events pane",
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
            cmd: "envelope",
            desc: "refresh envelope",
        },
        TuiHelpItem {
            cmd: "export",
            desc: "export redacted diagnostics",
        },
    ]
}

fn render_session(f: &mut ratatui::Frame, area: Rect, session: &TuiSession<'_>) {
    let verify = if session.verified {
        "verified"
    } else {
        "unverified"
    };
    let body = format!(
        "peer: {}\nstatus: {}\nclient_sent: {}\nclient_recv: {}",
        session.peer_label, verify, session.sent_count, session.recv_count
    );
    let panel = Paragraph::new(body).block(Block::default().borders(Borders::ALL).title("Session"));
    f.render_widget(panel, area);
}

fn render_inspector(f: &mut ratatui::Frame, area: Rect, state: &TuiState) {
    let title = format!("Inspector: {}", state.inspector_name());
    let body = match state.inspector {
        TuiInspectorPane::Status => format!(
            "qsp: {}\nown_fp: {}\npeer_fp: {}\nsend: {}\ncounts: sent={} recv={}",
            state.status.qsp,
            state.status.fingerprint,
            state.status.peer_fp,
            state.status.send_lifecycle,
            state.session.sent_count,
            state.session.recv_count
        ),
        TuiInspectorPane::Events => state
            .events
            .iter()
            .rev()
            .take(TUI_INSPECTOR_EVENTS_MAX)
            .cloned()
            .collect::<Vec<String>>()
            .into_iter()
            .rev()
            .collect::<Vec<String>>()
            .join("\n"),
        TuiInspectorPane::Session => {
            let replay_rejects = state
                .events
                .iter()
                .filter(|line| line.contains("ratchet_replay_reject"))
                .count();
            format!(
                "peer: {}\nverified: {}\nqsp: {}\nclient_sent: {}\nclient_recv: {}\nreplay_rejects: {}",
                state.session.peer_label,
                state.session.verified,
                state.status.qsp,
                state.session.sent_count,
                state.session.recv_count,
                replay_rejects
            )
        }
        TuiInspectorPane::Contacts => {
            let mut lines = Vec::new();
            lines.push(format!("contacts: {}", state.contacts.len()));
            for c in state.contacts.iter().take(TUI_INSPECTOR_CONTACTS_MAX) {
                lines.push(c.clone());
            }
            lines.join("\n")
        }
    };
    let panel = Paragraph::new(body).block(Block::default().borders(Borders::ALL).title(title));
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
    let (dir, source) = match config_dir() {
        Ok(v) => v,
        Err(e) => print_error_marker(e.as_str()),
    };
    let identities = identities_dir(&dir);
    if ensure_dir_secure(&identities, source).is_err() {
        print_error_marker("identity_dir_insecure");
    }
    let mut peers = Vec::new();
    if let Ok(entries) = fs::read_dir(&identities) {
        for entry in entries.flatten() {
            let path = entry.path();
            let Some(name) = path.file_name().and_then(|v| v.to_str()) else {
                continue;
            };
            if !name.starts_with("peer_") || !name.ends_with(".fp") {
                continue;
            }
            let peer = name.trim_start_matches("peer_").trim_end_matches(".fp");
            if !channel_label_ok(peer) {
                continue;
            }
            if let Ok(Some(fp)) = identity_read_pin(peer) {
                peers.push((peer.to_string(), fp));
            }
        }
    }
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
const POLL_TICKS_MAX: u32 = 1000;
const POLL_MAX_PER_TICK_MAX: u32 = 100;
const PAD_TO_MAX: usize = 65_536;

struct MetaPollConfig {
    interval_ms: u64,
    ticks: u32,
    max_per_tick: usize,
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

fn meta_poll_config_from_args(
    interval_ms: Option<u64>,
    ticks: Option<u32>,
    max_per_tick: Option<u32>,
    meta_seed: Option<u64>,
) -> Result<Option<MetaPollConfig>, &'static str> {
    let any = interval_ms.is_some() || ticks.is_some() || max_per_tick.is_some();
    if !any {
        return Ok(None);
    }
    let interval_ms = interval_ms.ok_or("meta_poll_required")?;
    let ticks = ticks.ok_or("meta_poll_required")?;
    let max_per_tick = max_per_tick.ok_or("meta_poll_required")?;
    if interval_ms == 0 || interval_ms > POLL_INTERVAL_MS_MAX {
        return Err("meta_poll_invalid");
    }
    if ticks == 0 || ticks > POLL_TICKS_MAX {
        return Err("meta_poll_invalid");
    }
    if max_per_tick == 0 || max_per_tick > POLL_MAX_PER_TICK_MAX {
        return Err("meta_poll_invalid");
    }
    Ok(Some(MetaPollConfig {
        interval_ms,
        ticks,
        max_per_tick: max_per_tick as usize,
        deterministic: meta_seed.is_some(),
    }))
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

fn identity_peer_path(dir: &Path, peer: &str) -> PathBuf {
    identities_dir(dir).join(format!("peer_{}.fp", peer))
}

fn identity_peer_sig_path(dir: &Path, peer: &str) -> PathBuf {
    identities_dir(dir).join(format!("peer_{}.sigfp", peer))
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

fn identity_read_pin(peer: &str) -> Result<Option<String>, ErrorCode> {
    if !channel_label_ok(peer) {
        return Err(ErrorCode::ParseFailed);
    }
    let (dir, source) = config_dir()?;
    let path = identity_peer_path(&dir, peer);
    if !path.exists() {
        return Ok(None);
    }
    enforce_safe_parents(&path, source)?;
    let bytes = fs::read(&path).map_err(|_| ErrorCode::IoReadFailed)?;
    let mut v = String::from_utf8_lossy(&bytes).to_string();
    while v.ends_with('\n') || v.ends_with('\r') {
        v.pop();
    }
    Ok(Some(v))
}

fn identity_write_pin(peer: &str, fp: &str) -> Result<(), ErrorCode> {
    if !channel_label_ok(peer) {
        return Err(ErrorCode::ParseFailed);
    }
    let (dir, source) = config_dir()?;
    let identities = identities_dir(&dir);
    ensure_dir_secure(&identities, source)?;
    let path = identity_peer_path(&dir, peer);
    let content = format!("{}\n", fp);
    write_atomic(&path, content.as_bytes(), source)?;
    Ok(())
}

fn identity_read_sig_pin(peer: &str) -> Result<Option<String>, ErrorCode> {
    if !channel_label_ok(peer) {
        return Err(ErrorCode::ParseFailed);
    }
    let (dir, source) = config_dir()?;
    let path = identity_peer_sig_path(&dir, peer);
    if !path.exists() {
        return Ok(None);
    }
    enforce_safe_parents(&path, source)?;
    let bytes = fs::read(&path).map_err(|_| ErrorCode::IoReadFailed)?;
    let mut v = String::from_utf8_lossy(&bytes).to_string();
    while v.ends_with('\n') || v.ends_with('\r') {
        v.pop();
    }
    Ok(Some(v))
}

fn identity_write_sig_pin(peer: &str, fp: &str) -> Result<(), ErrorCode> {
    if !channel_label_ok(peer) {
        return Err(ErrorCode::ParseFailed);
    }
    let (dir, source) = config_dir()?;
    let identities = identities_dir(&dir);
    ensure_dir_secure(&identities, source)?;
    let path = identity_peer_sig_path(&dir, peer);
    let content = format!("{}\n", fp);
    write_atomic(&path, content.as_bytes(), source)?;
    Ok(())
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
    let peer_label = peer.unwrap_or("peer-0");
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
                                    emit_marker(
                                        "identity_mismatch",
                                        None,
                                        &[
                                            ("peer", peer),
                                            ("pinned_fp", pinned.as_str()),
                                            ("seen_fp", sig_fp.as_str()),
                                        ],
                                    );
                                    emit_marker(
                                        "handshake_reject",
                                        None,
                                        &[("reason", "identity_mismatch")],
                                    );
                                    return;
                                }
                                emit_marker(
                                    "identity_ok",
                                    None,
                                    &[("peer", peer), ("fp", sig_fp.as_str())],
                                );
                            }
                            Ok(None) => {
                                if identity_write_sig_pin(peer, &sig_fp).is_err() {
                                    emit_marker(
                                        "handshake_reject",
                                        None,
                                        &[("reason", "identity_pin_failed")],
                                    );
                                    return;
                                }
                                emit_marker(
                                    "identity_pin",
                                    None,
                                    &[("peer", peer), ("fp", sig_fp.as_str())],
                                );
                            }
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
                            Ok(None) => {
                                if identity_write_pin(peer, peer_fp).is_err() {
                                    emit_marker(
                                        "handshake_reject",
                                        None,
                                        &[("reason", "identity_pin_failed")],
                                    );
                                    continue;
                                }
                                emit_marker(
                                    "identity_pin",
                                    None,
                                    &[("peer", peer), ("fp", peer_fp.as_str())],
                                );
                            }
                            Ok(Some(pinned)) => {
                                if pinned != *peer_fp {
                                    emit_marker(
                                        "identity_mismatch",
                                        None,
                                        &[
                                            ("peer", peer),
                                            ("pinned_fp", pinned.as_str()),
                                            ("seen_fp", peer_fp.as_str()),
                                        ],
                                    );
                                    emit_marker(
                                        "handshake_reject",
                                        None,
                                        &[("reason", "identity_mismatch")],
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
                            Ok(None) => {
                                if identity_write_sig_pin(peer, peer_sig_fp).is_err() {
                                    emit_marker(
                                        "handshake_reject",
                                        None,
                                        &[("reason", "identity_pin_failed")],
                                    );
                                    continue;
                                }
                                emit_marker(
                                    "identity_pin",
                                    None,
                                    &[("peer", peer), ("fp", peer_sig_fp.as_str())],
                                );
                            }
                            Ok(Some(pinned)) => {
                                if pinned != *peer_sig_fp {
                                    emit_marker(
                                        "identity_mismatch",
                                        None,
                                        &[
                                            ("peer", peer),
                                            ("pinned_fp", pinned.as_str()),
                                            ("seen_fp", peer_sig_fp.as_str()),
                                        ],
                                    );
                                    emit_marker(
                                        "handshake_reject",
                                        None,
                                        &[("reason", "identity_mismatch")],
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
                            emit_marker(
                                "identity_mismatch",
                                None,
                                &[
                                    ("peer", peer),
                                    ("pinned_fp", pinned.as_str()),
                                    ("seen_fp", peer_fp.as_str()),
                                ],
                            );
                            emit_marker(
                                "handshake_reject",
                                None,
                                &[("reason", "identity_mismatch")],
                            );
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
                            emit_marker(
                                "identity_mismatch",
                                None,
                                &[
                                    ("peer", peer),
                                    ("pinned_fp", pinned.as_str()),
                                    ("seen_fp", peer_sig_fp.as_str()),
                                ],
                            );
                            emit_marker(
                                "handshake_reject",
                                None,
                                &[("reason", "identity_mismatch")],
                            );
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

fn send_execute(
    transport: Option<SendTransport>,
    relay: Option<String>,
    to: Option<String>,
    file: Option<PathBuf>,
    pad_to: Option<usize>,
    pad_bucket: Option<MetaPadBucket>,
    meta_seed: Option<u64>,
) {
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
            relay_send(&to, &file, &relay, pad_cfg, meta_seed);
        }
    }
}

fn send_abort() {
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
    poll_interval_ms: Option<u64>,
    poll_ticks: Option<u32>,
    poll_max_per_tick: Option<u32>,
    meta_seed: Option<u64>,
}

fn receive_execute(args: ReceiveArgs) {
    let ReceiveArgs {
        transport,
        relay,
        from,
        mailbox,
        max,
        out,
        poll_interval_ms,
        poll_ticks,
        poll_max_per_tick,
        meta_seed,
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
            let poll_cfg = match meta_poll_config_from_args(
                poll_interval_ms,
                poll_ticks,
                poll_max_per_tick,
                meta_seed,
            ) {
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
            let recv_max = poll_cfg.as_ref().map(|c| c.max_per_tick).unwrap_or(max);
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
                let max_tick_s = cfg.max_per_tick.to_string();
                emit_marker(
                    "meta_poll_config",
                    None,
                    &[
                        ("interval_ms", interval_s.as_str()),
                        ("ticks", ticks_s.as_str()),
                        ("max_per_tick", max_tick_s.as_str()),
                    ],
                );
                for tick in 0..cfg.ticks {
                    let pull = ReceivePullCtx {
                        relay: &relay,
                        mailbox: mailbox.as_str(),
                        from: &from,
                        out: &out,
                        source,
                        cfg_dir: &cfg_dir,
                        cfg_source,
                    };
                    let pulled = receive_pull_and_write(&pull, cfg.max_per_tick);
                    total = total.saturating_add(pulled);
                    let tick_s = tick.to_string();
                    let pulled_s = pulled.to_string();
                    emit_marker(
                        "meta_poll_tick",
                        None,
                        &[("idx", tick_s.as_str()), ("pulled", pulled_s.as_str())],
                    );
                    emit_marker(
                        "meta_batch",
                        None,
                        &[("pulled", pulled_s.as_str()), ("max", max_tick_s.as_str())],
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
                };
                total = receive_pull_and_write(&pull, max);
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
}

fn receive_pull_and_write(ctx: &ReceivePullCtx<'_>, max: usize) -> usize {
    let items = match relay_inbox_pull(ctx.relay, ctx.mailbox, max) {
        Ok(v) => v,
        Err(code) => print_error_marker(code),
    };
    if items.is_empty() {
        return 0;
    }
    let mut idx = 0usize;
    for item in items {
        match qsp_unpack(ctx.from, &item.data) {
            Ok(outcome) => {
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
                idx = idx.saturating_add(1);
                let name = format!("recv_{}.bin", idx);
                let path = ctx.out.join(name);
                if write_atomic(&path, &outcome.plaintext, ctx.source).is_err() {
                    print_error_marker("recv_write_failed");
                }
                let idx_s = idx.to_string();
                let size_s = outcome.plaintext.len().to_string();
                emit_marker(
                    "recv_item",
                    None,
                    &[
                        ("idx", idx_s.as_str()),
                        ("size", size_s.as_str()),
                        ("id", item.id.as_str()),
                    ],
                );
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
    idx
}

fn receive_file(path: &Path) {
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
    meta_seed: Option<u64>,
) {
    if let Err(reason) = protocol_active_or_reason_for_peer(to) {
        protocol_inactive_exit(reason.as_str());
    }
    let payload = match fs::read(file) {
        Ok(v) => v,
        Err(_) => print_error_marker("relay_payload_read_failed"),
    };
    let outcome = relay_send_with_payload(
        to,
        payload,
        relay,
        fault_injector_from_env(),
        pad_cfg,
        meta_seed,
    );
    if let Some(code) = outcome.error_code {
        print_error_marker(code);
    }
}

struct RelaySendOutcome {
    action: String,
    delivered: bool,
    error_code: Option<&'static str>,
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

fn relay_send_with_payload(
    to: &str,
    payload: Vec<u8>,
    relay: &str,
    injector: Option<FaultInjector>,
    pad_cfg: Option<MetaPadConfig>,
    meta_seed: Option<u64>,
) -> RelaySendOutcome {
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
    )
}

fn finalize_send_commit(
    dir: &Path,
    source: ConfigSource,
    outbox_path: &Path,
    action: String,
    session_update: Option<(&str, Suite2SessionState)>,
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
