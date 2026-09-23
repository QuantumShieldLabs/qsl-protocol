#![allow(dead_code)]

use assert_cmd::Command;
use axum::serve;
use qsl_attachments::{
    build_router, AppState as AttachmentAppState, Config as AttachmentConfig,
    TestClock as AttachmentTestClock,
};
use qsl_server::{
    app as qsl_relay_app, AppState as QslRelayAppState, Limits as QslRelayLimits,
    ResourceControls as QslRelayResourceControls, StoreConfig as QslRelayStoreConfig,
};
use serde::Serialize;
use std::collections::{HashMap, VecDeque};
use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::process::Command as StdCommand;
use std::sync::{
    atomic::{AtomicBool, AtomicUsize, Ordering},
    Arc, Mutex,
};
use std::thread;
use std::time::{Duration, Instant};
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::oneshot;

pub const TEST_MOCK_VAULT_PASSPHRASE_ENV: &str = "QSC_DESKTOP_SESSION_PASSPHRASE";
pub const TEST_MOCK_VAULT_PASSPHRASE: &str = "qsc-test-mock-vault-passphrase";
pub const UNSAFE_TEST_SEED_FALLBACK_ENV: &str = "QSC_UNSAFE_TEST_SEED_FALLBACK";

/// The string the marker layer substitutes for a value it redacts
/// (`src/output/mod.rs`, `redact_value_for_output`).
pub const REDACTION_SENTINEL: &str = "<redacted>";

/// NA-0686 / D-1325 (ENG-0087) — THE SENTINEL FAIL-FAST RULE.
///
/// A test that learns a value by scraping a DIAGNOSTIC MARKER is coupled to
/// REDACTION POLICY. The marker layer redacts by VALUE SHAPE, not by key
/// (`should_redact_value` -> `looks_high_cardinality`: `len() >= 24` and
/// contains a digit), so **any change to a value's WIDTH is a behavioural
/// change to every scrape that reads it** — and nothing in the code declares
/// that coupling. That is OBS-FA, and it is the root condition; the scrapes are
/// only where it surfaces.
///
/// ⚠ The failure mode this exists to stop is NOT a broken scrape. It is a
/// scrape that **succeeds and returns the literal string `<redacted>`**. The
/// sentinel PARSES AS A VALID IDENTIFIER, so the test proceeds and fails much
/// later, in a different subsystem, with a misleading code — NA-0682 spent a
/// full diagnostic cycle tracing exactly that, from `state_unknown` back to a
/// scrape three steps upstream. **A test that fails at the point of the defect
/// is cheap; this one was not.**
///
/// FIRST-PARTY acquisition is the remedy and is always preferred: read the
/// value from the store, from the return value, or from the fixture that minted
/// it (`first_party_sent_msg_id` is the reference shape). Where a legacy scrape
/// must remain, it routes through here so it **fails AT the defect** instead of
/// handing policy output onward as data.
///
/// ⚠ Deliberately **FIELD-AGNOSTIC**: `field` names the failure and nothing
/// else. The id class is the only caller today, but the same scrape pattern
/// appears across the suite for `identity_fp=`, `identity_kem_pk=`,
/// `identity_sig_pk=`, `device=`, `state=`, `invite=`, `send_seq=` and `max=`.
/// None of those crosses the redactor today; all of them would adopt this
/// without redesign if one ever did. That population is enumerated in the
/// ENG-0087 annex rather than fixed here.
pub fn scraped_marker_value(field: &str, value: &str) -> String {
    assert_ne!(
        value, REDACTION_SENTINEL,
        "scraped `{field}=` and got the redaction sentinel. The marker layer \
         redacted this value, so the scrape returned REDACTION POLICY OUTPUT, \
         not data — and the sentinel would have parsed as a valid `{field}`. \
         Acquire `{field}` FIRST-PARTY (store / return value / minting fixture) \
         instead of scraping a marker. See ENG-0087."
    );
    value.to_string()
}

#[allow(dead_code)]
pub fn add_unsafe_seed_fallback_env(cmd: &mut StdCommand) {
    cmd.env("QSC_ALLOW_SEED_FALLBACK", "1")
        .env("QSC_UNSAFE_TEST_SEED_FALLBACK", "1")
        .env(UNSAFE_TEST_SEED_FALLBACK_ENV, "1");
}

#[allow(dead_code)]
pub fn init_mock_vault(cfg: &Path) {
    init_passphrase_vault(cfg, TEST_MOCK_VAULT_PASSPHRASE);
}

#[allow(dead_code)]
pub fn write_passphrase_file(dir: &Path, stem: &str, passphrase: &str) -> PathBuf {
    ensure_dir_700(dir);
    let path = dir.join(format!("{stem}.passphrase"));
    std::fs::write(&path, passphrase.as_bytes()).expect("write passphrase file");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o600)).expect("chmod 600");
    }
    path
}

#[allow(dead_code)]
pub fn add_global_unlock_passphrase_file_arg(
    cmd: &mut StdCommand,
    cfg: &Path,
    stem: &str,
    passphrase: &str,
) {
    let passphrase_file = write_passphrase_file(cfg, stem, passphrase);
    cmd.arg("--unlock-passphrase-file")
        .arg(passphrase_file.to_str().expect("passphrase file path"));
}

#[allow(dead_code)]
pub fn add_mock_vault_unlock_env_args(cmd: &mut StdCommand) {
    cmd.env(TEST_MOCK_VAULT_PASSPHRASE_ENV, TEST_MOCK_VAULT_PASSPHRASE)
        .arg("--unlock-passphrase-env")
        .arg(TEST_MOCK_VAULT_PASSPHRASE_ENV);
}

#[allow(dead_code)]
pub fn add_mock_vault_unlock_env_args_assert(cmd: &mut Command) {
    cmd.env(TEST_MOCK_VAULT_PASSPHRASE_ENV, TEST_MOCK_VAULT_PASSPHRASE)
        .arg("--unlock-passphrase-env")
        .arg(TEST_MOCK_VAULT_PASSPHRASE_ENV);
}

#[allow(dead_code)]
pub fn qsc_std_command() -> StdCommand {
    let mut cmd = StdCommand::new(assert_cmd::cargo::cargo_bin!("qsc"));
    add_mock_vault_unlock_env_args(&mut cmd);
    cmd
}

#[allow(dead_code)]
pub fn qsc_assert_command() -> Command {
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("qsc"));
    add_mock_vault_unlock_env_args_assert(&mut cmd);
    cmd
}

#[allow(dead_code)]
pub fn add_vault_passphrase_file_arg(
    cmd: &mut StdCommand,
    cfg: &Path,
    stem: &str,
    passphrase: &str,
) {
    let passphrase_file = write_passphrase_file(cfg, stem, passphrase);
    cmd.arg("--passphrase-file")
        .arg(passphrase_file.to_str().expect("passphrase file path"));
}

#[allow(dead_code)]
pub fn init_passphrase_vault(cfg: &Path, passphrase: &str) {
    ensure_dir_700(cfg);
    let input_dir = tempfile::tempdir().expect("private vault initialization input");
    let passphrase_file = write_passphrase_file(input_dir.path(), "vault-init", passphrase);
    let out = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", cfg)
        .env("QSC_DISABLE_KEYCHAIN", "1")
        .args([
            "vault",
            "init",
            "--protocol",
            "directional-v1",
            "--non-interactive",
            "--key-source",
            "passphrase",
            "--passphrase-file",
            passphrase_file.to_str().expect("passphrase file path"),
        ])
        .output()
        .expect("vault init passphrase");
    assert!(
        out.status.success(),
        "vault init failed: {}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
}

fn ensure_dir_700(path: &Path) {
    std::fs::create_dir_all(path).expect("create dir");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o700)).expect("chmod 700");
    }
}

static TEST_NONCE_COUNTER: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

fn qsc_test_root_base() -> PathBuf {
    if let Ok(v) = std::env::var("QSC_TEST_ROOT") {
        return PathBuf::from(v);
    }
    if let Ok(v) = std::env::var("CARGO_TARGET_DIR") {
        return PathBuf::from(v);
    }
    PathBuf::from("target")
}

pub fn unique_test_root(tag: &str) -> PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock")
        .as_nanos();
    let seq = TEST_NONCE_COUNTER.fetch_add(1, Ordering::Relaxed);
    qsc_test_root_base().join("qsc-test-tmp").join(format!(
        "{tag}_{}_{}_{}",
        std::process::id(),
        nonce,
        seq
    ))
}

#[derive(Clone, Debug)]
pub struct TestIsolation {
    pub root: PathBuf,
    home: PathBuf,
    xdg_config_home: PathBuf,
    tmpdir: PathBuf,
}

impl TestIsolation {
    pub fn new(tag: &str) -> Self {
        let root = unique_test_root(tag);
        let home = root.join("home");
        let xdg_config_home = home.join(".config");
        let tmpdir = root.join("tmp");
        ensure_dir_700(&root);
        ensure_dir_700(&home);
        ensure_dir_700(&xdg_config_home);
        ensure_dir_700(&tmpdir);
        Self {
            root,
            home,
            xdg_config_home,
            tmpdir,
        }
    }

    pub fn apply_to(&self, cmd: &mut StdCommand) {
        cmd.env("HOME", &self.home)
            .env("XDG_CONFIG_HOME", &self.xdg_config_home)
            .env("TMPDIR", &self.tmpdir);
    }
}

#[derive(Serialize)]
struct InboxPullItem {
    id: String,
    data: Vec<u8>,
}

#[derive(Serialize)]
struct InboxPullResp {
    items: Vec<InboxPullItem>,
}

const ROUTE_TOKEN_HEADER: &str = "x-qsl-route-token";

struct InboxStore {
    queues: HashMap<String, VecDeque<(String, Vec<u8>)>>,
    next_id: u64,
    max_body: usize,
    max_queue: usize,
    push_journal: Option<Vec<DirectionalPushAttempt>>,
    review_lease: Option<ReviewLease>,
    r02_push_plan: VecDeque<u16>,
}

impl InboxStore {
    fn new(max_body: usize, max_queue: usize) -> Self {
        Self {
            queues: HashMap::new(),
            next_id: 1,
            max_body,
            max_queue,
            push_journal: None,
            review_lease: None,
            r02_push_plan: VecDeque::new(),
        }
    }
}

// Exact R01/R02 allowlisted fixtures only. Pull leases retain bytes until a real ACK; observation
// never drains, reorders or expires a lease. Existing mock defaults stay unchanged.
#[derive(Default)]
struct ReviewLease {
    until: HashMap<String, Instant>,
    pulls: Vec<(String, Vec<String>)>,
    acks: Vec<(String, Vec<String>)>,
}

#[derive(Clone)]
pub struct ReviewLeaseSnapshot {
    pub retained: Vec<(String, Vec<u8>)>,
    pub pulls: Vec<Vec<String>>,
    pub acks: Vec<Vec<String>>,
}

// Opt-in, synthetic-fixture-only observation; no change to readiness or fault selection.
#[derive(Clone)]
pub struct DirectionalPushAttempt {
    pub body: Vec<u8>,
    pub status: u16,
    pub response_written: bool,
}

#[allow(dead_code)]
pub struct InboxTestServer {
    base_url: String,
    store: Arc<Mutex<InboxStore>>,
    fail_push_remaining: Arc<AtomicUsize>,
    shutdown: Arc<AtomicBool>,
    handle: Option<thread::JoinHandle<()>>,
}

#[allow(dead_code)]
impl InboxTestServer {
    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    pub fn drain_channel(&self, channel: &str) -> Vec<Vec<u8>> {
        let mut store = self.store.lock().unwrap();
        let mut out = Vec::new();
        if let Some(queue) = store.queues.get_mut(channel) {
            while let Some((_, data)) = queue.pop_front() {
                out.push(data);
            }
        }
        out
    }

    pub fn replace_channel(&self, channel: &str, items: Vec<Vec<u8>>) {
        let mut store = self.store.lock().unwrap();
        let next_id = store.next_id;
        store.next_id = store.next_id.saturating_add(items.len() as u64);
        let queue = store.queues.entry(channel.to_string()).or_default();
        queue.clear();
        for (idx, data) in items.into_iter().enumerate() {
            let id = next_id.saturating_add(idx as u64).to_string();
            queue.push_back((id, data));
        }
    }

    pub fn enqueue_raw(&self, channel: &str, data: Vec<u8>) {
        let mut store = self.store.lock().unwrap();
        let id = store.next_id.to_string();
        store.next_id += 1;
        let queue = store.queues.entry(channel.to_string()).or_default();
        queue.push_back((id, data));
    }

    pub fn enable_review_leases(&self) {
        assert!(matches!(std::env::var("QSC_NA0780_ISOLATED_CASE").as_deref(),
            Ok("directional_review_r01_receive_batch_baseline" | "directional_review_r01_receive_batch_fixed" | "directional_r02_queuefull_matrix")));
        let mut store = self.store.lock().unwrap();
        assert!(store.review_lease.is_none());
        assert!(store.queues.values().all(|queue| queue.is_empty()));
        store.review_lease = Some(ReviewLease::default());
        store.push_journal = Some(Vec::new());
    }

    pub fn review_lease_snapshot(&self, channel: &str) -> ReviewLeaseSnapshot {
        let store = self.store.lock().unwrap();
        let lease = store.review_lease.as_ref().expect("review leases enabled");
        ReviewLeaseSnapshot {
            retained: store.queues.get(channel).map(|q| q.iter().cloned().collect()).unwrap_or_default(),
            pulls: lease.pulls.iter().filter(|(c, _)| c == channel).map(|(_, ids)| ids.clone()).collect(),
            acks: lease.acks.iter().filter(|(c, _)| c == channel).map(|(_, ids)| ids.clone()).collect(),
        }
    }

    pub fn record_directional_pushes(&self) {
        self.store.lock().unwrap().push_journal = Some(Vec::new());
    }

    pub fn directional_pushes(&self) -> Vec<DirectionalPushAttempt> {
        self.store.lock().unwrap().push_journal.as_ref().expect("journal enabled").clone()
    }

    // Exact R02 children only; installed after readiness. Each actual push consumes
    // one status. 200 delegates to the ordinary mock admission path unchanged.
    pub fn r02_push_plan(&self, statuses: &[u16]) {
        assert!(matches!(std::env::var("QSC_NA0780_ISOLATED_CASE").as_deref(),
            Ok("directional_r02_queuefull_matrix" | "directional_r02_completion_cuts")));
        assert!(statuses.iter().all(|s| matches!(s, 200 | 429 | 500)));
        let mut store=self.store.lock().unwrap();
        assert!(store.r02_push_plan.is_empty(), "previous fault plan not consumed");
        store.r02_push_plan=statuses.iter().copied().collect();
    }

    pub fn set_fail_pushes(&self, count: usize) {
        self.fail_push_remaining.store(count, Ordering::SeqCst);
    }
}

impl Drop for InboxTestServer {
    fn drop(&mut self) {
        self.shutdown.store(true, Ordering::SeqCst);
        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }
    }
}

// Mock relay contract (tests-only):
// - Connection model: single-request-per-connection with "Connection: close" responses.
// - Timeout policy: bounded read timeout + request deadline to prevent CI hangs on partial/malformed input.
// - Readiness semantics: server is considered ready only after a bounded streak of successful push+pull probes.
pub fn start_inbox_server(max_body: usize, max_queue: usize) -> InboxTestServer {
    start_inbox_server_with_fail_pushes(max_body, max_queue, 0)
}

#[allow(dead_code)]
pub struct AttachmentTestServer {
    base_url: String,
    clock: AttachmentTestClock,
    shutdown: Option<oneshot::Sender<()>>,
    handle: Option<thread::JoinHandle<()>>,
}

#[allow(dead_code)]
impl AttachmentTestServer {
    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    pub fn advance(&self, delta_secs: u64) {
        self.clock.advance(delta_secs);
    }
}

impl Drop for AttachmentTestServer {
    fn drop(&mut self) {
        if let Some(tx) = self.shutdown.take() {
            let _ = tx.send(());
        }
        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }
    }
}

#[allow(dead_code)]
pub fn start_attachment_server(max_ciphertext_bytes: u64) -> AttachmentTestServer {
    let root = unique_test_root("qatt-runtime");
    let storage_root = root.join("storage");
    ensure_dir_700(&root);
    ensure_dir_700(&storage_root);
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("wall clock")
        .as_secs();
    let clock = AttachmentTestClock::new(now);
    let clock_clone = clock.clone();
    let (addr_tx, addr_rx) = std::sync::mpsc::channel();
    let (shutdown_tx, shutdown_rx) = oneshot::channel();
    let handle = thread::spawn(move || {
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("qatt runtime");
        runtime.block_on(async move {
            let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
                .await
                .expect("qatt bind");
            let addr = listener.local_addr().expect("qatt local addr");
            addr_tx.send(addr).expect("qatt ready send");
            let cfg = AttachmentConfig {
                storage_root,
                bind_addr: addr,
                max_ciphertext_bytes,
                ..AttachmentConfig::default()
            };
            let state =
                AttachmentAppState::new(cfg, Arc::new(clock_clone)).expect("qatt state init");
            let router = build_router(state);
            serve(listener, router)
                .with_graceful_shutdown(async {
                    let _ = shutdown_rx.await;
                })
                .await
                .expect("qatt serve");
        });
    });
    let addr = addr_rx.recv().expect("qatt ready addr");
    let server = AttachmentTestServer {
        base_url: format!("http://{}", addr),
        clock,
        shutdown: Some(shutdown_tx),
        handle: Some(handle),
    };
    wait_until_attachment_ready(server.base_url());
    server
}

// NA-0640 (D576): run the REAL qsl-server relay in-process for the full-stack e2e
// round-trip, mirroring start_attachment_server (bind 127.0.0.1:0, take local_addr,
// serve, graceful shutdown on drop). Auth is EXPLICIT: qsl-server's env-reading
// constructors (AppState::new / new_with_controls) consult the ambient RELAY_TOKEN,
// which a test must never depend on — new_with_auth_and_controls pins it per call.
#[allow(dead_code)]
pub struct QslRelayTestServer {
    base_url: String,
    shutdown: Option<oneshot::Sender<()>>,
    handle: Option<thread::JoinHandle<()>>,
}

#[allow(dead_code)]
impl QslRelayTestServer {
    pub fn base_url(&self) -> &str {
        &self.base_url
    }
}

impl Drop for QslRelayTestServer {
    fn drop(&mut self) {
        if let Some(tx) = self.shutdown.take() {
            let _ = tx.send(());
        }
        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }
    }
}

#[allow(dead_code)]
pub fn start_qsl_server(
    max_body: usize,
    max_queue: usize,
    relay_token: Option<&str>,
) -> QslRelayTestServer {
    let relay_token = relay_token.map(|t| t.to_string());
    let (addr_tx, addr_rx) = std::sync::mpsc::channel();
    let (shutdown_tx, shutdown_rx) = oneshot::channel();
    let handle = thread::spawn(move || {
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("qsl-server runtime");
        runtime.block_on(async move {
            let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
                .await
                .expect("qsl-server bind");
            let addr = listener.local_addr().expect("qsl-server local addr");
            addr_tx.send(addr).expect("qsl-server ready send");
            let limits = QslRelayLimits::new(max_body, max_queue).expect("qsl-server limits");
            let state = QslRelayAppState::new_with_auth_and_controls(
                limits,
                QslRelayResourceControls::default(),
                relay_token,
            );
            let router = qsl_relay_app(state);
            serve(listener, router)
                .with_graceful_shutdown(async {
                    let _ = shutdown_rx.await;
                })
                .await
                .expect("qsl-server serve");
        });
    });
    let addr = addr_rx.recv().expect("qsl-server ready addr");
    let server = QslRelayTestServer {
        base_url: format!("http://{}", addr),
        shutdown: Some(shutdown_tx),
        handle: Some(handle),
    };
    wait_until_qsl_server_ready(server.base_url());
    server
}

// NA-0644 (D580): additive-only variant of start_qsl_server for the ack-path tests —
// same in-process real relay, but with an explicit StoreConfig so a test can pin a short
// pull lease (real lease expiry + real redelivery, no 60s waits). The existing
// start_qsl_server and every test using it are untouched.
#[allow(dead_code)]
pub fn start_qsl_server_with_store(
    max_body: usize,
    max_queue: usize,
    relay_token: Option<&str>,
    pull_lease_secs: usize,
) -> QslRelayTestServer {
    let relay_token = relay_token.map(|t| t.to_string());
    let (addr_tx, addr_rx) = std::sync::mpsc::channel();
    let (shutdown_tx, shutdown_rx) = oneshot::channel();
    let handle = thread::spawn(move || {
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("qsl-server runtime");
        runtime.block_on(async move {
            let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
                .await
                .expect("qsl-server bind");
            let addr = listener.local_addr().expect("qsl-server local addr");
            addr_tx.send(addr).expect("qsl-server ready send");
            let limits = QslRelayLimits::new(max_body, max_queue).expect("qsl-server limits");
            let store_cfg = QslRelayStoreConfig {
                pull_lease_secs,
                ..QslRelayStoreConfig::default()
            };
            let state = QslRelayAppState::new_with_auth_controls_and_store(
                limits,
                QslRelayResourceControls::default(),
                relay_token,
                store_cfg,
            )
            .expect("qsl-server store open");
            let router = qsl_relay_app(state);
            serve(listener, router)
                .with_graceful_shutdown(async {
                    let _ = shutdown_rx.await;
                })
                .await
                .expect("qsl-server serve");
        });
    });
    let addr = addr_rx.recv().expect("qsl-server ready addr");
    let server = QslRelayTestServer {
        base_url: format!("http://{}", addr),
        shutdown: Some(shutdown_tx),
        handle: Some(handle),
    };
    wait_until_qsl_server_ready(server.base_url());
    server
}

fn wait_until_qsl_server_ready(base_url: &str) {
    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_millis(250))
        .build()
        .expect("build qsl-server readiness client");
    let deadline = Instant::now() + Duration::from_secs(5);
    while Instant::now() < deadline {
        let url = format!("{}/v1/pull?max=1", base_url);
        // Any parsed HTTP response (200 empty pull, 401 in token mode) proves the
        // relay is accepting and routing requests.
        if client
            .get(&url)
            .header("x-qsl-route-token", "readiness_probe_route_token_0000")
            .send()
            .is_ok()
        {
            return;
        }
        thread::sleep(Duration::from_millis(50));
    }
    panic!("qsl-server failed readiness: {base_url}");
}

#[allow(dead_code)]
pub fn start_inbox_server_with_fail_pushes(
    max_body: usize,
    max_queue: usize,
    fail_pushes: usize,
) -> InboxTestServer {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind inbox server");
    let addr = listener.local_addr().expect("inbox addr");
    listener
        .set_nonblocking(true)
        .expect("nonblocking inbox listener");
    let store = Arc::new(Mutex::new(InboxStore::new(max_body, max_queue)));
    let fail_push_remaining = Arc::new(AtomicUsize::new(0));
    let shutdown = Arc::new(AtomicBool::new(false));
    let shutdown_thread = Arc::clone(&shutdown);
    let store_thread = Arc::clone(&store);
    let fail_push_thread = Arc::clone(&fail_push_remaining);
    let handle = thread::spawn(move || {
        while !shutdown_thread.load(Ordering::SeqCst) {
            match listener.accept() {
                Ok((stream, _)) => {
                    let store_conn = Arc::clone(&store_thread);
                    let fail_push_conn = Arc::clone(&fail_push_thread);
                    thread::spawn(move || handle_conn(stream, store_conn, fail_push_conn));
                }
                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    thread::sleep(Duration::from_millis(10));
                }
                Err(_) => break,
            }
        }
    });
    let server = InboxTestServer {
        base_url: format!("http://{}", addr),
        store,
        fail_push_remaining,
        shutdown,
        handle: Some(handle),
    };
    wait_until_ready(server.base_url());
    server.set_fail_pushes(fail_pushes);
    server
}

fn wait_until_ready(base_url: &str) {
    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_millis(150))
        .build()
        .expect("build readiness client");
    let deadline = Instant::now() + Duration::from_secs(5);
    let mut attempt = 0u64;
    let mut consecutive_ok = 0u8;
    const READY_STREAK: u8 = 3;
    while Instant::now() < deadline {
        let probe_channel = format!("qsc_ready_probe_{}_{}", std::process::id(), attempt);
        let push_url = format!("{}/v1/push", base_url);
        let mut ok = false;
        if let Ok(resp) = client
            .post(&push_url)
            .header("X-QSL-Route-Token", probe_channel.as_str())
            .body(vec![0x51])
            .send()
        {
            if resp.status().as_u16() == 200 {
                let pull_url = format!("{}/v1/pull?max=1", base_url);
                if let Ok(resp) = client
                    .get(&pull_url)
                    .header("X-QSL-Route-Token", probe_channel.as_str())
                    .send()
                {
                    if resp.status().as_u16() == 200 {
                        ok = true;
                    }
                }
            }
        }
        if ok {
            consecutive_ok = consecutive_ok.saturating_add(1);
            if consecutive_ok >= READY_STREAK {
                return;
            }
        } else {
            consecutive_ok = 0;
        }
        attempt = attempt.saturating_add(1);
        thread::sleep(Duration::from_millis(20));
    }
    panic!("inbox test server readiness probe timed out");
}

fn wait_until_attachment_ready(base_url: &str) {
    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_millis(250))
        .build()
        .expect("build qatt readiness client");
    let deadline = Instant::now() + Duration::from_secs(5);
    while Instant::now() < deadline {
        let url = format!("{}/v1/attachments/sessions", base_url);
        let body = serde_json::json!({
            "attachment_id": "0".repeat(64),
            "ciphertext_len": 32_u64,
            "part_size_class": "p64k",
            "part_count": 1_u32,
            "integrity_alg": "sha512_merkle_v1",
            "integrity_root": "0".repeat(128),
            "retention_class": "standard"
        });
        if let Ok(resp) = client.post(&url).json(&body).send() {
            if resp.status().is_success() || resp.status().as_u16() == 201 {
                return;
            }
            if resp.status().as_u16() == 422 || resp.status().as_u16() == 400 {
                return;
            }
        }
        thread::sleep(Duration::from_millis(50));
    }
    panic!("attachment server failed readiness: {base_url}");
}

fn handle_conn(
    mut stream: TcpStream,
    store: Arc<Mutex<InboxStore>>,
    fail_push_remaining: Arc<AtomicUsize>,
) {
    let _ = stream.set_read_timeout(Some(Duration::from_millis(200)));
    let deadline = Instant::now() + Duration::from_secs(2);
    let mut buf = Vec::with_capacity(1024);

    let header_end = match read_until_header_end(&mut stream, &mut buf, deadline) {
        Some(pos) => pos,
        None => {
            let _ = write_response(&mut stream, 400, "bad request");
            return;
        }
    };
    let header_bytes = &buf[..header_end];
    let header_str = String::from_utf8_lossy(header_bytes);
    let mut lines = header_str.split("\r\n");
    let request_line = match lines.next() {
        Some(line) => line,
        None => {
            let _ = write_response(&mut stream, 400, "bad request");
            return;
        }
    };
    let mut req_parts = request_line.split_whitespace();
    let method = req_parts.next().unwrap_or("");
    let target = req_parts.next().unwrap_or("");
    let mut content_len = 0usize;
    let mut seen_content_len = false;
    let mut has_chunked_transfer_encoding = false;
    let mut route_token_header = None::<String>;
    for line in lines {
        let Some((name, value)) = line.split_once(':') else {
            continue;
        };
        if name.trim().eq_ignore_ascii_case(ROUTE_TOKEN_HEADER) {
            route_token_header = Some(value.trim().to_string());
            continue;
        }
        if name.trim().eq_ignore_ascii_case("transfer-encoding") {
            has_chunked_transfer_encoding |= value
                .split(',')
                .any(|v| v.trim().eq_ignore_ascii_case("chunked"));
            continue;
        }
        if !name.trim().eq_ignore_ascii_case("content-length") {
            continue;
        }
        let Ok(n) = value.trim().parse::<usize>() else {
            let _ = write_response(&mut stream, 400, "bad request");
            return;
        };
        if seen_content_len && n != content_len {
            let _ = write_response(&mut stream, 400, "bad request");
            return;
        }
        seen_content_len = true;
        content_len = n;
    }
    let initial_body = &buf[(header_end + 4)..];
    let body = match read_body_exact(&mut stream, initial_body, content_len, deadline) {
        Some(b) => b,
        None => {
            let _ = write_response(&mut stream, 400, "bad request");
            return;
        }
    };

    if method == "POST" && target == "/v1/push" {
        let remaining = fail_push_remaining.load(Ordering::SeqCst);
        if remaining > 0
            && fail_push_remaining
                .compare_exchange(
                    remaining,
                    remaining.saturating_sub(1),
                    Ordering::SeqCst,
                    Ordering::SeqCst,
                )
                .is_ok()
        {
            let response_written = write_response(&mut stream, 500, "ERR_PUSH_FAIL_INJECTED").is_ok();
            if let Some(journal) = &mut store.lock().unwrap().push_journal {
                journal.push(DirectionalPushAttempt { body, status: 500, response_written });
            }
            return;
        }
        if has_chunked_transfer_encoding {
            let _ = write_response(&mut stream, 400, "ERR_UNSUPPORTED_TRANSFER_ENCODING");
            return;
        }
        let channel = match resolve_route_token(route_token_header.clone()) {
            Ok(v) => v,
            Err(code) => {
                let _ = write_response(&mut stream, 400, code);
                return;
            }
        };
        if !channel_label_ok(channel.as_str()) {
            let _ = write_response(&mut stream, 400, "ERR_BAD_CHANNEL");
            return;
        }
        let mut store = store.lock().unwrap();
        if let Some(status) = store.r02_push_plan.pop_front() {
            if status != 200 {
                let code=if status==429 {"ERR_QUEUE_FULL"} else {"ERR_PUSH_FAIL_INJECTED"};
                let response_written=write_response(&mut stream,status,code).is_ok();
                if let Some(journal)=&mut store.push_journal {
                    journal.push(DirectionalPushAttempt {body,status,response_written});
                }
                return;
            }
        }
        if body.len() > store.max_body {
            let _ = write_response(&mut stream, 413, "ERR_TOO_LARGE");
            return;
        }
        let queue_len = store
            .queues
            .get(channel.as_str())
            .map(|q| q.len())
            .unwrap_or(0);
        if queue_len >= store.max_queue {
            let _ = write_response(&mut stream, 429, "ERR_QUEUE_FULL");
            return;
        }
        let id = store.next_id.to_string();
        store.next_id += 1;
        let queue = store.queues.entry(channel).or_default();
        queue.push_back((id.clone(), body.clone()));
        let response = format!("{{\"id\":\"{}\"}}", id);
        let response_written = write_response_json(&mut stream, 200, &response).is_ok();
        if let Some(journal) = &mut store.push_journal {
            journal.push(DirectionalPushAttempt { body, status: 200, response_written });
        }
        return;
    }

    if method == "POST" && target == "/v1/pull/ack" && store.lock().unwrap().review_lease.is_some() {
        let channel = match resolve_route_token(route_token_header.clone()) {
            Ok(channel) => channel,
            Err(code) => { let _ = write_response(&mut stream, 400, code); return; }
        };
        #[derive(serde::Deserialize)]
        #[serde(deny_unknown_fields)]
        struct Ack { ids: Vec<String> }
        let ack: Ack = match serde_json::from_slice(&body) {
            Ok(ack) => ack,
            Err(_) => { let _ = write_response(&mut stream, 400, "bad ack"); return; }
        };
        let mut store = store.lock().unwrap();
        store.review_lease.as_mut().unwrap().acks.push((channel.clone(), ack.ids.clone()));
        let queue = store.queues.entry(channel).or_default();
        let before = queue.len();
        queue.retain(|(id, _)| !ack.ids.contains(id));
        let count = before - queue.len();
        let _ = write_response_json(&mut stream, 200, &format!("{{\"acked\":{count}}}"));
        return;
    }

    if method == "GET" && target.starts_with("/v1/pull") {
        let (path, query) = match target.split_once('?') {
            Some((p, q)) => (p, Some(q)),
            None => (target, None),
        };
        if path != "/v1/pull" {
            let _ = write_response(&mut stream, 404, "not found");
            return;
        }
        let mut max_n = 1usize;
        if let Some(query) = query {
            for part in query.split('&') {
                if let Some(v) = part.strip_prefix("max=") {
                    if let Ok(n) = v.parse::<usize>() {
                        max_n = n;
                    }
                }
            }
        }
        let channel = match resolve_route_token(route_token_header) {
            Ok(v) => v,
            Err(code) => {
                let _ = write_response(&mut stream, 400, code);
                return;
            }
        };
        if !channel_label_ok(channel.as_str()) {
            let _ = write_response(&mut stream, 400, "ERR_BAD_CHANNEL");
            return;
        }
        let mut store = store.lock().unwrap();
        if store.review_lease.is_some() {
            assert!(target.split('?').nth(1).unwrap_or("").split('&').any(|p| p == "ack=lease"));
            let now = Instant::now();
            let lease = store.review_lease.as_ref().unwrap();
            let items: Vec<InboxPullItem> = store.queues.get(&channel).into_iter().flatten()
                .filter(|(id, _)| lease.until.get(id).is_none_or(|until| *until <= now))
                .take(max_n).map(|(id, data)| InboxPullItem { id: id.clone(), data: data.clone() }).collect();
            let lease = store.review_lease.as_mut().unwrap();
            for item in &items { lease.until.insert(item.id.clone(), now + Duration::from_secs(600)); }
            lease.pulls.push((channel, items.iter().map(|item| item.id.clone()).collect()));
            if items.is_empty() { let _ = write_response_empty(&mut stream, 204); }
            else { let _ = write_response_json(&mut stream, 200, &serde_json::to_string(&InboxPullResp { items }).unwrap()); }
            return;
        }
        let queue = store.queues.entry(channel).or_default();
        if queue.is_empty() {
            let _ = write_response_empty(&mut stream, 204);
            return;
        }
        let mut items = Vec::new();
        let mut count = 0usize;
        while count < max_n {
            if let Some((id, data)) = queue.pop_front() {
                items.push(InboxPullItem { id, data });
                count += 1;
            } else {
                break;
            }
        }
        let resp = InboxPullResp { items };
        let body = serde_json::to_string(&resp).unwrap();
        let _ = write_response_json(&mut stream, 200, &body);
        return;
    }

    let _ = write_response(&mut stream, 404, "not found");
}

fn resolve_route_token(header_token: Option<String>) -> Result<String, &'static str> {
    let header_token = match header_token {
        None => None,
        Some(raw) => {
            let token = raw.trim();
            if token.is_empty() {
                return Err("ERR_MISSING_ROUTE_TOKEN");
            }
            Some(token.to_string())
        }
    };
    header_token.ok_or("ERR_MISSING_ROUTE_TOKEN")
}

fn read_until_header_end(
    stream: &mut TcpStream,
    buf: &mut Vec<u8>,
    deadline: Instant,
) -> Option<usize> {
    let mut tmp = [0u8; 1024];
    while Instant::now() < deadline {
        if let Some(pos) = find_header_end(buf) {
            return Some(pos);
        }
        match stream.read(&mut tmp) {
            Ok(0) => return find_header_end(buf),
            Ok(n) => buf.extend_from_slice(&tmp[..n]),
            Err(e)
                if matches!(
                    e.kind(),
                    std::io::ErrorKind::Interrupted
                        | std::io::ErrorKind::WouldBlock
                        | std::io::ErrorKind::TimedOut
                ) =>
            {
                continue
            }
            Err(_) => return None,
        }
    }
    find_header_end(buf)
}

fn read_body_exact(
    stream: &mut TcpStream,
    initial: &[u8],
    content_len: usize,
    deadline: Instant,
) -> Option<Vec<u8>> {
    if content_len == 0 {
        return Some(Vec::new());
    }
    let mut body = Vec::with_capacity(content_len);
    body.extend_from_slice(&initial[..initial.len().min(content_len)]);
    while body.len() < content_len && Instant::now() < deadline {
        let mut tmp = [0u8; 1024];
        match stream.read(&mut tmp) {
            Ok(0) => break,
            Ok(n) => {
                let remaining = content_len - body.len();
                body.extend_from_slice(&tmp[..n.min(remaining)]);
            }
            Err(e)
                if matches!(
                    e.kind(),
                    std::io::ErrorKind::Interrupted
                        | std::io::ErrorKind::WouldBlock
                        | std::io::ErrorKind::TimedOut
                ) =>
            {
                continue
            }
            Err(_) => return None,
        }
    }
    if body.len() == content_len {
        Some(body)
    } else {
        None
    }
}

fn channel_label_ok(label: &str) -> bool {
    !label.is_empty()
        && label
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
}

fn find_header_end(buf: &[u8]) -> Option<usize> {
    buf.windows(4).position(|w| w == b"\r\n\r\n")
}

fn write_response(stream: &mut TcpStream, code: u16, body: &str) -> std::io::Result<()> {
    let body_bytes = body.as_bytes();
    let status = status_line(code);
    let header = format!(
        "HTTP/1.1 {}\r\nContent-Length: {}\r\nContent-Type: text/plain\r\nConnection: close\r\n\r\n",
        status,
        body_bytes.len()
    );
    stream.write_all(header.as_bytes())?;
    stream.write_all(body_bytes)?;
    stream.flush()?;
    Ok(())
}

fn write_response_json(stream: &mut TcpStream, code: u16, body: &str) -> std::io::Result<()> {
    let body_bytes = body.as_bytes();
    let status = status_line(code);
    let header = format!(
        "HTTP/1.1 {}\r\nContent-Length: {}\r\nContent-Type: application/json\r\nConnection: close\r\n\r\n",
        status,
        body_bytes.len()
    );
    stream.write_all(header.as_bytes())?;
    stream.write_all(body_bytes)?;
    stream.flush()?;
    Ok(())
}

fn write_response_empty(stream: &mut TcpStream, code: u16) -> std::io::Result<()> {
    let status = status_line(code);
    let header = format!(
        "HTTP/1.1 {}\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
        status
    );
    stream.write_all(header.as_bytes())?;
    stream.flush()?;
    Ok(())
}

fn status_line(code: u16) -> &'static str {
    match code {
        200 => "200 OK",
        204 => "204 No Content",
        400 => "400 Bad Request",
        401 => "401 Unauthorized",
        403 => "403 Forbidden",
        404 => "404 Not Found",
        413 => "413 Payload Too Large",
        429 => "429 Too Many Requests",
        _ => "500 Internal Server Error",
    }
}

/// NA-0682: count QUEUED message records in the message store.
///
/// ⚠ MIGRATION HELPER. Before NA-0682 an unsent message left `outbox.json` on disk, and
/// several tests asserted `outbox.exists()` to mean "the message survived a failed send".
/// The default send path now commits to the per-contact message queue instead (D617 §2b/§2c,
/// operator-ruled Option A), so the same property is observed here.
///
/// Deliberately structural, not cryptographic: it counts `.rec` files without decrypting
/// them, because the property under test is "the message is still on disk, recoverable",
/// not "the bytes are X". The byte-level properties (replay-identical ciphertext, pack
/// exactly once, ratchet-advance-before-drop) are guarded by unit tests inside the module,
/// which can see the plaintext and count the operations.
pub fn queued_record_count(cfg: &Path) -> usize {
    let root = cfg.join("msgqueue_v1");
    let Ok(contacts) = fs::read_dir(&root) else {
        return 0;
    };
    let mut n = 0;
    for c in contacts.flatten() {
        if !c.path().is_dir() {
            continue;
        }
        if let Ok(files) = fs::read_dir(c.path()) {
            for f in files.flatten() {
                if f.path().extension().and_then(|v| v.to_str()) == Some("rec") {
                    n += 1;
                }
            }
        }
    }
    n
}

/// Establish fresh, explicitly opted-in peers through the real CLI and handshake.
/// This supplies a sending precondition for transport tests; it never fabricates
/// session keys or enables the retired seed fallback.
pub fn init_directional_pair(
    left: &Path,
    left_label: &str,
    left_route: &str,
    right: &Path,
    right_label: &str,
    right_route: &str,
) {
    fn run(cfg: &Path, args: &[&str]) -> String {
        let out = qsc_std_command()
            .env("QSC_CONFIG_DIR", cfg)
            .env("QSC_DISABLE_KEYCHAIN", "1")
            .args(args)
            .output()
            .expect("directional fixture command");
        assert!(out.status.success(), "directional fixture command failed");
        String::from_utf8(out.stdout).expect("fixture output UTF-8")
    }
    fn public_field<'a>(text: &'a str, field: &str) -> &'a str {
        let value = text.lines().find_map(|line| line.strip_prefix(field))
            .expect("identity public field");
        assert_ne!(value, REDACTION_SENTINEL, "public identity field redacted");
        value
    }
    for (cfg, label, route) in [(left, left_label, left_route), (right, right_label, right_route)] {
        init_mock_vault(cfg);
        run(cfg, &["identity", "rotate", "--as", label, "--confirm"]);
        run(cfg, &["relay", "inbox-set", "--token", route]);
    }
    let left_public = run(left, &["identity", "show", "--as", left_label]);
    let right_public = run(right, &["identity", "show", "--as", right_label]);
    for (cfg, label, route, public) in [
        (left, right_label, right_route, right_public.as_str()),
        (right, left_label, left_route, left_public.as_str()),
    ] {
        run(cfg, &["contacts", "add", "--label", label,
            "--fp", public_field(public, "identity_fp="),
            "--kem-pk", public_field(public, "identity_kem_pk="),
            "--sig-pk", public_field(public, "identity_sig_pk="),
            "--route-token", route]);
        let devices = run(cfg, &["contacts", "device", "list", "--label", label]);
        let device = devices.lines().find_map(|line| line.strip_prefix("device="))
            .and_then(|line| line.split_whitespace().next()).expect("fixture device");
        assert_ne!(device, REDACTION_SENTINEL);
        run(cfg, &["contacts", "device", "trust", "--label", label,
            "--device", device, "--confirm"]);
    }
    let relay = start_inbox_server(1024 * 1024, 16);
    run(left, &["handshake", "init", "--as", left_label, "--peer", right_label,
        "--relay", relay.base_url(), "--suite-mode", "suite-required"]);
    run(right, &["handshake", "poll", "--as", right_label, "--peer", left_label,
        "--relay", relay.base_url(), "--max", "4", "--suite-mode", "suite-required"]);
    let left_done = run(left, &["handshake", "poll", "--as", left_label, "--peer", right_label,
        "--relay", relay.base_url(), "--max", "4", "--suite-mode", "suite-required"]);
    let right_done = run(right, &["handshake", "poll", "--as", right_label, "--peer", left_label,
        "--relay", relay.base_url(), "--max", "4", "--suite-mode", "suite-required"]);
    assert!(left_done.contains("event=handshake_complete"), "initiator did not complete");
    assert!(right_done.contains("event=handshake_complete"), "responder did not complete");
}

/// Run the exact allowlisted fixtures in their own process. The parent
/// never selects a vault or unlocks one; unrelated libtest workers remain isolated.
pub fn directional_case_child(case: &str) -> bool {
    directional_case_child_bounded(case, Duration::from_secs(590), None)
}

pub fn directional_case_child_bounded(case: &str, limit: Duration, probe: Option<&str>) -> bool {
    const GUARD: &str = "QSC_NA0780_ISOLATED_CASE";
    assert!(matches!(case, "dh_ratchet_e2e_roundtrip_over_real_handshake" | "send_failure_no_commit" | "directional_isolation_child_probe" | "directional_successor_authenticated_malformed_no_mutation" | "directional_review_r01_receive_batch_baseline" | "directional_review_r01_receive_batch_fixed" | "directional_r02_fresh_and_ordinary_writers" | "directional_r02_genuine_receipt_restart" | "directional_r02_funded_release_at_saturation" | "directional_r02_serializer_maintenance" | "directional_r02_repeated_controls" | "directional_r02_queuefull_matrix" | "directional_r02_completion_cuts"));
    assert!(limit <= Duration::from_secs(590));
    if let Some(selected) = std::env::var_os(GUARD) {
        assert_eq!(selected, std::ffi::OsStr::new(case), "exact child case guard");
        return true;
    }
    let parent_env: std::collections::BTreeMap<_, _> = std::env::vars_os().collect();
    let parent_passphrase_present = qsc::vault::has_process_passphrase();
    let mut command = StdCommand::new(std::env::current_exe().unwrap());
    command.args([case, "--exact", "--nocapture", "--test-threads=1"])
        .env(GUARD, case).env("QSC_DISABLE_KEYCHAIN", "1")
        .env_remove("QSC_CONFIG_DIR")
        .env_remove("QSC_PASSPHRASE")
        .env_remove(TEST_MOCK_VAULT_PASSPHRASE_ENV);
    if let Some(probe) = probe { command.env("QSC_NA0780_OBSERVER_PROBE", probe); }
    #[cfg(unix)]
    {
        use std::os::unix::process::CommandExt;
        command.process_group(0);
    }
    struct ChildGuard(std::process::Child);
    impl Drop for ChildGuard {
        fn drop(&mut self) {
            // Only this self-child's private process group is owned by this guard.
            #[cfg(unix)]
            {
                unsafe extern "C" { fn kill(pid: i32, sig: i32) -> i32; }
                unsafe { kill(-(self.0.id() as i32), 9); }
            }
            let _ = self.0.kill();
            let _ = self.0.wait();
        }
    }
    let mut child = ChildGuard(command.spawn().expect("spawn isolated exact case"));
    let deadline = Instant::now() + limit;
    let status = loop {
        if let Some(status) = child.0.try_wait().expect("observe isolated case") { break status; }
        assert!(Instant::now() < deadline, "isolated case deadline");
        thread::sleep(Duration::from_millis(25));
    };
    drop(child);
    assert!(parent_env == std::env::vars_os().collect(), "parent environment unchanged");
    assert_eq!(qsc::vault::has_process_passphrase(), parent_passphrase_present, "parent unlock state unchanged");
    assert!(status.success(), "isolated exact case failed");
    false
}

/// Authenticated read-only observations, reachable only inside an exact self-child.
fn directional_observer_session(cfg: &Path) -> qsc::vault::VaultSession {
    assert!(matches!(std::env::var("QSC_NA0780_ISOLATED_CASE").as_deref(),
        Ok("dh_ratchet_e2e_roundtrip_over_real_handshake" | "send_failure_no_commit" | "directional_successor_authenticated_malformed_no_mutation" | "directional_review_r01_receive_batch_baseline" | "directional_review_r01_receive_batch_fixed" | "directional_r02_fresh_and_ordinary_writers" | "directional_r02_genuine_receipt_restart" | "directional_r02_funded_release_at_saturation" | "directional_r02_serializer_maintenance" | "directional_r02_repeated_controls" | "directional_r02_queuefull_matrix" | "directional_r02_completion_cuts")),
        "observer requires isolated exact case");
    std::env::set_var("QSC_CONFIG_DIR", cfg);
    qsc::vault::open_session_with_passphrase(TEST_MOCK_VAULT_PASSPHRASE)
        .expect("authenticated read-only session")
}

/// Read-only authenticated observations for the exact allowlisted directional cases.
/// No guarded unlock, failure-counter reset or parent process mutation.
pub fn directional_state(cfg: &Path, peer: &str) -> serde_json::Value {
    let session = directional_observer_session(cfg);
    let raw = qsc::vault::session_get(&session, &format!("na0780_directional_transaction_v2/{peer}"))
        .unwrap().expect("authenticated directional transaction");
    let state: serde_json::Value = serde_json::from_str(&raw).unwrap();
    assert!(state["version"] == "NA0780-DIR-INTEGRATION-03", "exact profile required");
    state
}

pub fn directional_pair_assert(a: &serde_json::Value, b: &serde_json::Value) {
    assert!(a["version"] == b["version"] && a["core"]["sid"] == b["core"]["sid"], "same authenticated profile/session");
    assert!(a["core"]["root"] == b["core"]["root"], "authenticated roots agree");
    assert!(a["core"]["seq"] == b["core"]["seq"] && a["core"]["owner"] == b["core"]["owner"], "sequence/owner agree");
    assert!(a["core"]["role"] == 0 && b["core"]["role"] == 1, "complementary roles");
}

pub fn directional_queue_records(cfg: &Path, peer: &str) -> Vec<qsc::msgqueue::QueuedMessage> {
    use chacha20poly1305::{aead::{Aead, Payload}, ChaCha20Poly1305, Key, KeyInit, Nonce};
    use sha2::{Digest, Sha512};
    let digest = Sha512::digest(peer.as_bytes());
    let contact: String = digest[..8].iter().map(|b| format!("{b:02x}")).collect();
    let entries = match fs::read_dir(cfg.join("msgqueue_v1").join(&contact)) {
        Ok(entries) => entries,
        Err(error) if error.kind()==std::io::ErrorKind::NotFound => return Vec::new(),
        Err(error) => panic!("queue inspection failed: {error}"),
    };
    let session = directional_observer_session(cfg);
    let encoded = qsc::vault::session_get(&session, "msgqueue_store_key_v1").unwrap().unwrap();
    assert_eq!(encoded.len(), 64);
    let key: Vec<u8> = encoded.as_bytes().chunks_exact(2).map(|pair| {
        u8::from_str_radix(std::str::from_utf8(pair).unwrap(), 16).unwrap()
    }).collect();
    assert_eq!(key.len(), 32);
    let mut records = Vec::new();
    for entry in entries {
        let path = entry.unwrap().path();
        if path.extension().and_then(|s| s.to_str()) != Some("rec") { continue; }
        let filename = path.file_name().unwrap().to_str().unwrap();
        let (sequence, id) = filename.strip_suffix(".rec").unwrap().split_once('_').unwrap();
        let sequence: u64 = sequence.parse().unwrap();
        let aad = format!("qsc.msgqueue.v1|{contact}|{id}|{sequence}");
        let raw = fs::read(&path).unwrap();
        let clear = ChaCha20Poly1305::new(Key::from_slice(&key)).decrypt(
            Nonce::from_slice(&raw[..12]), Payload { msg: &raw[12..], aad: aad.as_bytes() }
        ).expect("synthetic queue authentication failed");
        let rec: qsc::msgqueue::QueuedMessage = serde_json::from_slice(&clear).unwrap();
        assert!(rec.msg_id == id && rec.seq == sequence && rec.peer == peer);
        records.push(rec);
    }
    records.sort_by_key(|rec| rec.seq);
    records
}

/// Observe every normal CLI operation, attributing emitted versus authenticated
/// received boundaries to its actor. No wire-only claim of authentication.
#[derive(Default)]
pub struct DirectionalTrace {
    emitted: std::collections::BTreeMap<u64, serde_json::Value>,
    authenticated: std::collections::BTreeSet<u64>,
}
impl DirectionalTrace {
    pub fn operation<T>(&mut self, cfg: &Path, peer: &str, phase: &str, run: impl FnOnce() -> T) -> T {
        eprintln!("NA0780_DIAG phase={phase} begin");
        let before = directional_state(cfg, peer);
        let result = run();
        let after = directional_state(cfg, peer);
        let b = &before["core"];
        let a = &after["core"];
        assert!(b["sid"] == a["sid"] && b["role"] == a["role"], "session/role preserved");
        let old = b["seq"].as_u64().unwrap();
        let new = a["seq"].as_u64().unwrap();
        if new == old {
            assert!(a["root"] == b["root"] && a["owner"] == b["owner"], "no hidden root/owner transition");
        } else {
            assert_eq!(new, old + 1, "observe each individual boundary");
            assert!(a["root"] != b["root"], "fresh root required");
            let role = a["role"].as_u64().unwrap();
            assert_eq!(a["owner"].as_u64().unwrap(), 1 - b["owner"].as_u64().unwrap());
            if b["owner"] == b["role"] {
                assert!(before["demand"] == true || before["since_boundary"].as_u64().unwrap() >= 4 || b["send"].is_null(), "normal owner/demand/no-send prerequisite");
                assert!(a["own_pub"] != b["own_pub"], "fresh sender DH required");
                assert_eq!(a["send"]["id"].as_u64(), Some(new));
                let flights: Vec<_> = after["flights"].as_object().unwrap().values()
                    .filter(|f| f["epoch"].as_u64() == Some(new) && f["slot"] == 0).collect();
                assert_eq!(flights.len(), 1, "boundary durably retained");
                let wire: Vec<u8> = serde_json::from_value(flights[0]["wire"].clone()).unwrap();
                assert!(wire.len() >= 74 && &wire[..4] == b"NDE1" && wire[4] == 1, "actual boundary wire");
                assert_eq!(wire[21] as u64, role);
                assert_eq!(u64::from_be_bytes(wire[22..30].try_into().unwrap()), new);
                assert_eq!(u64::from_be_bytes(wire[66..74].try_into().unwrap()), old);
                assert!(serde_json::to_value(&wire[5..21]).unwrap() == a["sid"], "wire session binding");
                assert!(serde_json::to_value(&wire[34..66]).unwrap() == a["own_pub"], "wire sender DH binding");
                assert!(self.emitted.insert(new, after.clone()).is_none(), "one emission per sequence");
            } else {
                let sender = self.emitted.get(&new).expect("observed originating boundary before intake");
                assert!(a["root"] == sender["core"]["root"] && a["sid"] == sender["core"]["sid"], "authenticated root convergence");
                assert!(a["peer_pub"] == sender["core"]["own_pub"] && a["peer_pub"] != b["peer_pub"], "authenticated peer DH installed");
                assert_eq!(a["active_recv"].as_u64(), Some(new));
                assert!(after["recv"].get(new.to_string()).is_some(), "authenticated receipt context installed");
                self.authenticated.insert(new);
            }
        }
        eprintln!("NA0780_DIAG phase={phase} end seq={new}");
        result
    }

    pub fn both_directions(&self) -> bool {
        [0, 1].iter().all(|role| self.emitted.iter().any(|(seq, s)|
            s["core"]["role"] == *role && self.authenticated.contains(seq)))
    }
}
