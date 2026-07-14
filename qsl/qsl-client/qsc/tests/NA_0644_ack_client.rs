// NA-0644 (D580, ENG-0040): the qsc acknowledged-pull (lease) client against the REAL
// in-process qsl-server at the pinned rev.
//
// The ordering under test (the whole lane): pull-lease -> persist durably -> ONLY THEN
// ack -> the server deletes. Never ack before the durable persist. Lease delivery is
// at-least-once, so a lost ack redelivers ids in normal operation — the client dedups
// by relay msg_id BEFORE unpack (ack-and-skip, never reprocess, never process-exit).
//
// Scenarios:
//   (a) legacy default: no --ack-mode -> the exact pre-lane pull URL, zero ack POSTs;
//   (b) lease happy path: ack fires and the server DELETES (proven via lease expiry —
//       an unacked message would reappear; an acked one stays gone);
//   (c) THE LANE-PROVER: a LOST ack -> real lease expiry -> real redelivery -> every
//       item deduped (recv_dup_skipped), no reprocessing, no process-exit, then acked;
//   (d) crash between persist and ack (SIGKILL mid-ack-stall): the payload is already
//       on disk while the ack is stalled (persist-BEFORE-ack made visible), and the
//       redelivery after the kill is deduped cleanly;
//   (e) old-server tolerance: ack=lease ignored upstream + 404 on the ack route ->
//       ack_legacy_complete, no error, nothing lost, nothing redelivered;
//   (f) the commit-before-write seam (pre-existing, filed): a crash between the ratchet
//       commit and the payload write makes that one message unrecoverable — the lease
//       backstop acks it LOUDLY (ack_replay_unrecoverable) and the redelivery loop ends.
//
// The relay leg is qsl-server's actual router; the only test double is a thin recording
// proxy in front of it that shapes the ACK ROUTE (drop / stall / 404) — exactly the
// behaviors of a lossy network and of the pre-durability relay (which has no ack route
// and ignores the ack param).

mod common;

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, MutexGuard, OnceLock};
use std::thread;
use std::time::{Duration, Instant};

const ALICE_MAILBOX: &str = "na0644_alice_mailbox_token_abcd12";
const BOB_MAILBOX: &str = "na0644_bob_mailbox_token_wxyz567";

// Short real lease so expiry/redelivery are real server behavior, not simulation.
const TEST_PULL_LEASE_SECS: usize = 1;
const LEASE_EXPIRY_WAIT: Duration = Duration::from_millis(2500);

fn test_guard() -> MutexGuard<'static, ()> {
    static TEST_GUARD: OnceLock<Mutex<()>> = OnceLock::new();
    TEST_GUARD
        .get_or_init(|| Mutex::new(()))
        .lock()
        .unwrap_or_else(|e| e.into_inner())
}

fn output_text(out: &std::process::Output) -> String {
    let mut s = String::from_utf8_lossy(&out.stdout).to_string();
    s.push_str(&String::from_utf8_lossy(&out.stderr));
    s
}

fn ensure_dir_700(path: &Path) {
    fs::create_dir_all(path).expect("create dir");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(path, fs::Permissions::from_mode(0o700)).expect("chmod 700");
    }
}

fn create_dir_700(path: &Path) {
    let _ = fs::remove_dir_all(path);
    ensure_dir_700(path);
}

fn safe_test_root(tag: &str) -> PathBuf {
    let root = if let Ok(v) = std::env::var("QSC_TEST_ROOT") {
        PathBuf::from(v)
    } else if let Ok(v) = std::env::var("CARGO_TARGET_DIR") {
        PathBuf::from(v)
    } else {
        PathBuf::from("target")
    };
    let root = root.join("qsc-test-tmp").join(tag);
    create_dir_700(&root);
    root
}

fn qsc_cmd(cfg: &Path) -> std::process::Command {
    let mut cmd = common::qsc_std_command();
    cmd.env("QSC_CONFIG_DIR", cfg)
        .env("QSC_QSP_SEED", "1")
        .env("QSC_ALLOW_SEED_FALLBACK", "1")
        .env("QSC_UNSAFE_TEST_SEED_FALLBACK", "1")
        .env("QSC_MARK_FORMAT", "plain");
    cmd
}

fn run_ok(cfg: &Path, args: &[&str]) -> String {
    let out = qsc_cmd(cfg).args(args).output().expect("run qsc");
    let text = output_text(&out);
    assert!(out.status.success(), "command failed: {:?}\n{}", args, text);
    text
}

fn run_fail(cfg: &Path, args: &[&str]) -> String {
    let out = qsc_cmd(cfg).args(args).output().expect("run qsc");
    let text = output_text(&out);
    assert!(
        !out.status.success(),
        "command unexpectedly succeeded: {:?}\n{}",
        args,
        text
    );
    text
}

fn init_identity(cfg: &Path) -> String {
    common::init_mock_vault(cfg);
    run_ok(cfg, &["identity", "rotate", "--confirm"]);
    let show = run_ok(cfg, &["identity", "show"]);
    show.lines()
        .find_map(|line| line.strip_prefix("identity_fp="))
        .unwrap_or_else(|| panic!("missing identity_fp output: {show}"))
        .to_string()
}

fn add_contact(cfg: &Path, label: &str, fp: &str, route_token: &str) {
    run_ok(
        cfg,
        &[
            "contacts",
            "add",
            "--label",
            label,
            "--fp",
            fp,
            "--route-token",
            route_token,
        ],
    );
    let list = run_ok(cfg, &["contacts", "device", "list", "--label", label]);
    let device = list
        .lines()
        .find_map(|line| {
            line.split_whitespace()
                .find_map(|tok| tok.strip_prefix("device="))
        })
        .unwrap_or_else(|| panic!("missing device output: {list}"));
    run_ok(
        cfg,
        &[
            "contacts",
            "device",
            "trust",
            "--label",
            label,
            "--device",
            device,
            "--confirm",
        ],
    );
}

// Contact labels aligned ("bob" on both sides) so receive --from matches the local peer
// label on both clients — the na0182/na0640 convention.
fn setup_pair(base: &Path) -> (PathBuf, PathBuf, PathBuf) {
    let alice_cfg = base.join("alice_cfg");
    let bob_cfg = base.join("bob_cfg");
    let bob_out = base.join("bob_out");
    create_dir_700(&alice_cfg);
    create_dir_700(&bob_cfg);
    create_dir_700(&bob_out);

    let alice_fp = init_identity(&alice_cfg);
    let bob_fp = init_identity(&bob_cfg);

    run_ok(&alice_cfg, &["relay", "inbox-set", "--token", ALICE_MAILBOX]);
    run_ok(&bob_cfg, &["relay", "inbox-set", "--token", BOB_MAILBOX]);

    add_contact(&alice_cfg, "bob", bob_fp.as_str(), BOB_MAILBOX);
    add_contact(&bob_cfg, "bob", alice_fp.as_str(), ALICE_MAILBOX);

    (alice_cfg, bob_cfg, bob_out)
}

fn send_message(alice_cfg: &Path, relay_url: &str, base: &Path, name: &str, bytes: &[u8]) {
    let msg = base.join(name);
    fs::write(&msg, bytes).expect("write msg");
    let text = run_ok(
        alice_cfg,
        &[
            "send",
            "--transport",
            "relay",
            "--relay",
            relay_url,
            "--to",
            "bob",
            "--file",
            msg.to_str().expect("msg path"),
        ],
    );
    assert!(
        text.contains("QSC_DELIVERY state=accepted_by_relay"),
        "{}",
        text
    );
}

fn receive_args<'a>(relay_url: &'a str, out: &'a str, lease: bool) -> Vec<&'a str> {
    let mut args = vec![
        "receive",
        "--transport",
        "relay",
        "--relay",
        relay_url,
        "--mailbox",
        BOB_MAILBOX,
        "--from",
        "bob",
        "--max",
        "8",
        "--out",
        out,
    ];
    if lease {
        args.push("--ack-mode");
        args.push("lease");
    }
    args
}

fn recv_file_count(out: &Path) -> usize {
    fs::read_dir(out)
        .expect("read out dir")
        .filter_map(|e| e.ok())
        .filter(|e| {
            let name = e.file_name().to_string_lossy().to_string();
            name.starts_with("recv_") && name.ends_with(".bin")
        })
        .count()
}

// ---------------------------------------------------------------------------
// The recording/shaping proxy: forwards /v1/push and /v1/pull to the REAL pinned
// qsl-server and shapes only the ACK ROUTE per mode — a lossy network (drop/stall)
// or the pre-durability relay (no ack route -> 404; ack param ignored upstream).
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, PartialEq)]
enum ProxyMode {
    Record,
    DropAck,
    StallAck,
    OldServer,
}

struct ProxyState {
    upstream: String,
    mode: ProxyMode,
    client: reqwest::Client,
    pull_uris: Mutex<Vec<String>>,
    ack_posts: AtomicUsize,
    ack_stalled: AtomicBool,
    release: AtomicBool,
}

struct RelayProxy {
    base_url: String,
    state: Arc<ProxyState>,
    shutdown: Option<tokio::sync::oneshot::Sender<()>>,
    handle: Option<thread::JoinHandle<()>>,
}

impl RelayProxy {
    fn base_url(&self) -> &str {
        &self.base_url
    }
    fn pull_uris(&self) -> Vec<String> {
        self.state.pull_uris.lock().expect("pull uris").clone()
    }
    fn ack_posts(&self) -> usize {
        self.state.ack_posts.load(Ordering::SeqCst)
    }
    fn ack_stalled(&self) -> bool {
        self.state.ack_stalled.load(Ordering::SeqCst)
    }
}

impl Drop for RelayProxy {
    fn drop(&mut self) {
        self.state.release.store(true, Ordering::SeqCst);
        if let Some(tx) = self.shutdown.take() {
            let _ = tx.send(());
        }
        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }
    }
}

async fn proxy_handler(
    axum::extract::State(st): axum::extract::State<Arc<ProxyState>>,
    method: axum::http::Method,
    uri: axum::http::Uri,
    headers: axum::http::HeaderMap,
    body: axum::body::Bytes,
) -> axum::response::Response {
    use axum::response::IntoResponse;
    let path = uri.path().to_string();
    let mut path_and_query = uri
        .path_and_query()
        .map(|v| v.as_str().to_string())
        .unwrap_or_else(|| path.clone());
    if path == "/v1/pull/ack" {
        match st.mode {
            ProxyMode::DropAck => {
                return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Vec::new())
                    .into_response();
            }
            ProxyMode::StallAck => {
                st.ack_stalled.store(true, Ordering::SeqCst);
                while !st.release.load(Ordering::SeqCst) {
                    tokio::time::sleep(Duration::from_millis(50)).await;
                }
                return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Vec::new())
                    .into_response();
            }
            ProxyMode::OldServer => {
                // The pre-durability relay has no /v1/pull/ack route at all.
                return (axum::http::StatusCode::NOT_FOUND, Vec::new()).into_response();
            }
            ProxyMode::Record => {
                st.ack_posts.fetch_add(1, Ordering::SeqCst);
            }
        }
    }
    if path == "/v1/pull" {
        st.pull_uris
            .lock()
            .expect("pull uris")
            .push(path_and_query.clone());
        if st.mode == ProxyMode::OldServer {
            // The pre-durability relay deserializes PullQuery{max} only and silently
            // ignores the ack param; strip it so the pinned server behaves exactly
            // like the old one (legacy delete-on-deliver).
            path_and_query = path_and_query
                .replace("&ack=lease", "")
                .replace("?ack=lease&", "?")
                .replace("?ack=lease", "");
        }
    }
    let url = format!("{}{}", st.upstream, path_and_query);
    let method = reqwest::Method::from_bytes(method.as_str().as_bytes()).expect("proxy method");
    let mut req = st.client.request(method, url);
    for name in ["x-qsl-route-token", "authorization", "content-type"] {
        if let Some(v) = headers.get(name) {
            if let Ok(vs) = v.to_str() {
                req = req.header(name, vs);
            }
        }
    }
    let resp = req.body(body.to_vec()).send().await.expect("proxy forward");
    let status = axum::http::StatusCode::from_u16(resp.status().as_u16()).expect("proxy status");
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());
    let bytes = resp.bytes().await.expect("proxy body").to_vec();
    let mut builder = axum::response::Response::builder().status(status);
    if let Some(ct) = content_type {
        builder = builder.header("content-type", ct);
    }
    builder
        .body(axum::body::Body::from(bytes))
        .expect("proxy response")
}

fn start_relay_proxy(upstream: &str, mode: ProxyMode) -> RelayProxy {
    let state = Arc::new(ProxyState {
        upstream: upstream.trim_end_matches('/').to_string(),
        mode,
        client: reqwest::Client::new(),
        pull_uris: Mutex::new(Vec::new()),
        ack_posts: AtomicUsize::new(0),
        ack_stalled: AtomicBool::new(false),
        release: AtomicBool::new(false),
    });
    let (addr_tx, addr_rx) = std::sync::mpsc::channel();
    let (shutdown_tx, shutdown_rx) = tokio::sync::oneshot::channel();
    let st = state.clone();
    let handle = thread::spawn(move || {
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("proxy runtime");
        runtime.block_on(async move {
            let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
                .await
                .expect("proxy bind");
            let addr = listener.local_addr().expect("proxy addr");
            addr_tx.send(addr).expect("proxy ready");
            let app = axum::Router::new()
                .fallback(axum::routing::any(proxy_handler))
                .with_state(st);
            axum::serve(listener, app)
                .with_graceful_shutdown(async {
                    let _ = shutdown_rx.await;
                })
                .await
                .expect("proxy serve");
        });
    });
    let addr = addr_rx.recv().expect("proxy ready addr");
    RelayProxy {
        base_url: format!("http://{}", addr),
        state,
        shutdown: Some(shutdown_tx),
        handle: Some(handle),
    }
}

// ---------------------------------------------------------------------------
// (a) Legacy default: without --ack-mode the pull URL is the exact pre-lane one
//     (no ack param) and the ack route is never touched.
// ---------------------------------------------------------------------------
#[test]
fn legacy_default_sends_no_ack_param_and_never_acks() {
    let _guard = test_guard();
    let server = common::start_qsl_server(2 * 1024 * 1024, 512, None);
    let proxy = start_relay_proxy(server.base_url(), ProxyMode::Record);
    let base = safe_test_root("na0644_legacy_default");
    let (alice_cfg, bob_cfg, bob_out) = setup_pair(&base);

    let msg: &[u8] = b"na0644 legacy default message";
    send_message(&alice_cfg, proxy.base_url(), &base, "msg.txt", msg);

    let out_s = bob_out.to_str().expect("out").to_string();
    let text = run_ok(&bob_cfg, &receive_args(proxy.base_url(), out_s.as_str(), false));

    let received = fs::read(bob_out.join("recv_1.bin")).expect("read received");
    assert_eq!(received, msg, "legacy plaintext differs");
    let pulls = proxy.pull_uris();
    assert!(!pulls.is_empty(), "no pull observed");
    for uri in &pulls {
        assert!(
            uri.starts_with("/v1/pull?max=") && !uri.contains("ack="),
            "legacy pull URL changed: {uri}"
        );
    }
    assert_eq!(proxy.ack_posts(), 0, "legacy receive must never ack");
    assert!(
        !text.contains("event=recv_ack_mode")
            && !text.contains("event=relay_ack")
            && !text.contains("event=recv_dup_skipped"),
        "lease-mode markers leaked into legacy output: {text}"
    );
}

// ---------------------------------------------------------------------------
// (b) Lease happy path: persist -> ack -> the server DELETES. Deletion (not just
//     lease-invisibility) is proven by waiting past the lease: an unacked message
//     would reappear; the acked one stays gone.
// ---------------------------------------------------------------------------
#[test]
fn lease_happy_path_acks_and_deletes_server_side() {
    let _guard = test_guard();
    let server =
        common::start_qsl_server_with_store(2 * 1024 * 1024, 512, None, TEST_PULL_LEASE_SECS);
    let base = safe_test_root("na0644_lease_happy");
    let (alice_cfg, bob_cfg, bob_out) = setup_pair(&base);

    let msg: &[u8] = b"na0644 lease happy path message";
    send_message(&alice_cfg, server.base_url(), &base, "msg.txt", msg);

    let out_s = bob_out.to_str().expect("out").to_string();
    let text = run_ok(&bob_cfg, &receive_args(server.base_url(), out_s.as_str(), true));
    assert!(text.contains("event=recv_ack_mode"), "{text}");
    assert!(text.contains("event=relay_ack"), "missing relay_ack: {text}");
    let received = fs::read(bob_out.join("recv_1.bin")).expect("read received");
    assert_eq!(received, msg, "lease plaintext differs");

    // Past the lease: an unacked copy would be redelivered here. It must not be.
    thread::sleep(LEASE_EXPIRY_WAIT);
    let text2 = run_ok(&bob_cfg, &receive_args(server.base_url(), out_s.as_str(), true));
    assert!(
        text2.contains("event=recv_none"),
        "acked message reappeared (server did not delete): {text2}"
    );
    assert_eq!(recv_file_count(&bob_out), 1, "unexpected extra recv files");
}

// ---------------------------------------------------------------------------
// (c) THE LANE-PROVER — lost ack -> real lease expiry -> real redelivery -> dedup.
//     Run 1 persists everything but its ack is dropped by the network. The relay
//     redelivers. Run 2 must ack-and-skip every id: no reprocessing, no new files,
//     no process-exit. Run 3 proves the re-ack landed (queue drained).
// ---------------------------------------------------------------------------
#[test]
fn lost_ack_redelivery_is_deduped_not_reprocessed() {
    let _guard = test_guard();
    let server =
        common::start_qsl_server_with_store(2 * 1024 * 1024, 512, None, TEST_PULL_LEASE_SECS);
    let proxy = start_relay_proxy(server.base_url(), ProxyMode::DropAck);
    let base = safe_test_root("na0644_lost_ack");
    let (alice_cfg, bob_cfg, bob_out) = setup_pair(&base);

    let msg1: &[u8] = b"na0644 lost-ack message one";
    let msg2: &[u8] = b"na0644 lost-ack message two";
    send_message(&alice_cfg, server.base_url(), &base, "msg1.txt", msg1);
    send_message(&alice_cfg, server.base_url(), &base, "msg2.txt", msg2);

    // Run 1: pull-lease through the ack-dropping network. Persist succeeds; ack lost.
    let out_s = bob_out.to_str().expect("out").to_string();
    let text1 = run_ok(&bob_cfg, &receive_args(proxy.base_url(), out_s.as_str(), true));
    assert!(text1.contains("event=ack_failed"), "missing ack_failed: {text1}");
    assert_eq!(
        fs::read(bob_out.join("recv_1.bin")).expect("recv_1"),
        msg1,
        "run 1 did not persist message one"
    );
    assert_eq!(
        fs::read(bob_out.join("recv_2.bin")).expect("recv_2"),
        msg2,
        "run 1 did not persist message two"
    );
    let files_after_run1 = recv_file_count(&bob_out);

    // The lease expires; the relay redelivers the un-acked ids.
    thread::sleep(LEASE_EXPIRY_WAIT);

    // Run 2: direct to the relay (network healed). Everything must dedup.
    let text2 = run_ok(&bob_cfg, &receive_args(server.base_url(), out_s.as_str(), true));
    let dups = text2.matches("event=recv_dup_skipped").count();
    assert!(
        dups >= 2,
        "expected the redelivered messages to be deduped (got {dups}): {text2}"
    );
    assert!(
        !text2.contains("event=ack_replay_unrecoverable"),
        "persisted items misclassified as unrecoverable: {text2}"
    );
    assert!(text2.contains("event=relay_ack"), "missing re-ack: {text2}");
    assert_eq!(
        recv_file_count(&bob_out),
        files_after_run1,
        "redelivery was reprocessed into new files"
    );

    // Run 3: past another lease window — the re-ack deleted the copies for good.
    thread::sleep(LEASE_EXPIRY_WAIT);
    let text3 = run_ok(&bob_cfg, &receive_args(server.base_url(), out_s.as_str(), true));
    assert!(
        text3.contains("event=recv_none") && !text3.contains("event=recv_dup_skipped"),
        "queue not drained after re-ack: {text3}"
    );
}

// ---------------------------------------------------------------------------
// (d) Crash between persist and ack: SIGKILL while the ack POST is stalled. The
//     payload must already be on disk at that moment (persist-BEFORE-ack made
//     visible), and the post-crash redelivery must dedup cleanly.
// ---------------------------------------------------------------------------
#[test]
fn crash_between_persist_and_ack_redelivery_deduped() {
    let _guard = test_guard();
    let server =
        common::start_qsl_server_with_store(2 * 1024 * 1024, 512, None, TEST_PULL_LEASE_SECS);
    let proxy = start_relay_proxy(server.base_url(), ProxyMode::StallAck);
    let base = safe_test_root("na0644_crash_before_ack");
    let (alice_cfg, bob_cfg, bob_out) = setup_pair(&base);

    let msg: &[u8] = b"na0644 crash-between-persist-and-ack message";
    send_message(&alice_cfg, server.base_url(), &base, "msg.txt", msg);

    let out_s = bob_out.to_str().expect("out").to_string();
    let mut child = qsc_cmd(&bob_cfg)
        .args(&receive_args(proxy.base_url(), out_s.as_str(), true))
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("spawn receive");

    // Wait until the client is INSIDE the ack POST (persist already done, ack pending).
    let deadline = Instant::now() + Duration::from_secs(30);
    while !proxy.ack_stalled() {
        assert!(
            Instant::now() < deadline,
            "receive never reached the ack POST"
        );
        assert!(
            child.try_wait().expect("child poll").is_none(),
            "receive exited before the ack stall"
        );
        thread::sleep(Duration::from_millis(50));
    }
    // Persist-before-ack, observed: the payload is durable while the ack is in flight.
    let received = fs::read(bob_out.join("recv_1.bin")).expect("payload persisted before ack");
    assert_eq!(received, msg, "persisted payload differs");
    child.kill().expect("SIGKILL receive");
    let _ = child.wait();

    // The lease expires; the relay redelivers; the healed client dedups and re-acks.
    thread::sleep(LEASE_EXPIRY_WAIT);
    let text2 = run_ok(&bob_cfg, &receive_args(server.base_url(), out_s.as_str(), true));
    assert!(
        text2.matches("event=recv_dup_skipped").count() >= 1,
        "redelivery not deduped: {text2}"
    );
    assert!(text2.contains("event=relay_ack"), "missing re-ack: {text2}");
    assert_eq!(recv_file_count(&bob_out), 1, "redelivery reprocessed");

    thread::sleep(LEASE_EXPIRY_WAIT);
    let text3 = run_ok(&bob_cfg, &receive_args(server.base_url(), out_s.as_str(), true));
    assert!(
        text3.contains("event=recv_none"),
        "queue not drained after crash recovery: {text3}"
    );
}

// ---------------------------------------------------------------------------
// (e) Old-server tolerance: the pre-durability relay ignores ?ack=lease (delivers
//     legacy delete-on-deliver) and 404s the ack route. The client must treat the
//     404 as "legacy-complete": no error, no retry — and nothing is lost.
// ---------------------------------------------------------------------------
#[test]
fn old_server_ack_404_is_legacy_complete() {
    let _guard = test_guard();
    let server =
        common::start_qsl_server_with_store(2 * 1024 * 1024, 512, None, TEST_PULL_LEASE_SECS);
    let proxy = start_relay_proxy(server.base_url(), ProxyMode::OldServer);
    let base = safe_test_root("na0644_old_server");
    let (alice_cfg, bob_cfg, bob_out) = setup_pair(&base);

    let msg: &[u8] = b"na0644 old server message";
    send_message(&alice_cfg, proxy.base_url(), &base, "msg.txt", msg);

    let out_s = bob_out.to_str().expect("out").to_string();
    let text = run_ok(&bob_cfg, &receive_args(proxy.base_url(), out_s.as_str(), true));
    assert!(
        text.contains("event=ack_legacy_complete"),
        "missing legacy-complete: {text}"
    );
    assert!(
        !text.contains("event=ack_failed") && !text.contains("event=error"),
        "old-server 404 treated as an error: {text}"
    );
    let received = fs::read(bob_out.join("recv_1.bin")).expect("read received");
    assert_eq!(received, msg, "old-server plaintext differs");

    // The old server already deleted on deliver: nothing redelivers, nothing is lost.
    thread::sleep(LEASE_EXPIRY_WAIT);
    let text2 = run_ok(&bob_cfg, &receive_args(proxy.base_url(), out_s.as_str(), true));
    assert!(
        text2.contains("event=recv_none"),
        "old-server delivery redelivered or lost: {text2}"
    );
}

// ---------------------------------------------------------------------------
// (f) The commit-before-write seam (pre-existing, FILED — not fixed here): the
//     ratchet key is consumed durably BEFORE the payload write, so a crash in that
//     gap makes the one message unrecoverable. The lease backstop must ack it
//     LOUDLY (ack_replay_unrecoverable) and END the redelivery loop — bounded,
//     visible behavior instead of an eternal poison redelivery.
// ---------------------------------------------------------------------------
#[test]
fn commit_before_write_seam_acked_loudly_no_poison_loop() {
    let _guard = test_guard();
    let server =
        common::start_qsl_server_with_store(2 * 1024 * 1024, 512, None, TEST_PULL_LEASE_SECS);
    let base = safe_test_root("na0644_seam");
    let (alice_cfg, bob_cfg, bob_out) = setup_pair(&base);

    let msg: &[u8] = b"na0644 commit-before-write seam message";
    send_message(&alice_cfg, server.base_url(), &base, "msg.txt", msg);

    // Force the crash INSIDE the gap: the write_atomic rename target is occupied by a
    // directory, so the ratchet commit succeeds and the payload write then fails —
    // process-exit after commit, before ack (exactly the seam).
    fs::create_dir(bob_out.join("recv_1.bin")).expect("occupy rename target");
    let out_s = bob_out.to_str().expect("out").to_string();
    let text1 = run_fail(&bob_cfg, &receive_args(server.base_url(), out_s.as_str(), true));
    assert!(
        text1.contains("recv_write_failed"),
        "expected the write to fail inside the gap: {text1}"
    );
    fs::remove_dir(bob_out.join("recv_1.bin")).expect("clear rename target");

    // The lease expires and the relay redelivers an envelope whose key is consumed.
    thread::sleep(LEASE_EXPIRY_WAIT);
    let text2 = run_ok(&bob_cfg, &receive_args(server.base_url(), out_s.as_str(), true));
    assert!(
        text2.contains("event=ack_replay_unrecoverable"),
        "seam redelivery not acked loudly: {text2}"
    );
    assert!(
        text2.contains("event=relay_ack"),
        "unrecoverable id not acked: {text2}"
    );

    // No poison loop: the loud ack deleted the copy; the queue drains for good.
    thread::sleep(LEASE_EXPIRY_WAIT);
    let text3 = run_ok(&bob_cfg, &receive_args(server.base_url(), out_s.as_str(), true));
    assert!(
        text3.contains("event=recv_none")
            && !text3.contains("event=ack_replay_unrecoverable"),
        "poison redelivery loop not ended: {text3}"
    );
}
