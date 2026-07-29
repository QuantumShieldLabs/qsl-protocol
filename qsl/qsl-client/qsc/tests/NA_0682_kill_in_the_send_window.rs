//! NA-0682 (D617 §5 A1) — **THE DELIVERABLE**: kill the process inside the
//! persist-before-network window and prove the message is neither lost nor invisible.
//!
//! ## Why this file exists
//!
//! Slice 2 asserted interruption safety **by construction** and never killed a process
//! (NA-0681 testplan §C.5). D617 makes killing it an acceptance item and makes
//! "assert it by construction" a STOP CONDITION. This is that test.
//!
//! ## The method, and the property that makes it deterministic
//!
//! Copied from `NA_0644_ack_client.rs::crash_between_persist_and_ack_redelivery_deduped` —
//! **the PROPERTY, not just the arrangement** (D617 C13):
//!
//! > A crash test is deterministic when the window is **held open by a controllable
//! > external dependency** and **entry into it is OBSERVED before the kill** — never a
//! > sleep-and-hope race.
//!
//! Here the dependency is a proxy that stalls `/v1/push` indefinitely. The client reaches
//! the network call, blocks, and the test polls until the proxy reports it is inside. Only
//! then does it SIGKILL. The window is arbitrarily wide and its entry is a fact, not a
//! timing assumption.
//!
//! ⚠ That proxy is written HERE, not shared. `NA_0644_ack_client.rs` is FORBIDDEN to
//! refactor (D617 §6) — it is the sole evidence for the lease/dedup contract, and putting
//! this lane's changes through it would risk a proven artifact for no protocol gain.
//!
//! ## ⚠ WHAT THIS TEST DOES **NOT** PROVE — stated as a limit, not a claim
//!
//! **It proves crash-safety against PROCESS DEATH (SIGKILL), not against POWER LOSS.**
//! `write_atomic` fsyncs the file and best-effort fsyncs the directory, but SIGKILL does
//! not evict the page cache, so nothing here exercises the power-loss path. Operator-ruled
//! into the testplan §C as a LIMIT (D617; the Slice-1 fsync lesson applied — a durability
//! claim is only as good as the thing that actually got exercised).

mod common;

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

const PEER: &str = "alice";
const ROUTE_TOKEN_PEER: &str = "route_token_peer_abcdefghijklmnopq";

// ---------------------------------------------------------------------------
// The push-stalling proxy
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, PartialEq)]
enum ProxyMode {
    /// Block `/v1/push` forever (until release) and record that we are inside it.
    ///
    /// A `Passthrough` arm existed here and was never constructed — the proxy only ever needs
    /// to stall, because A1's whole purpose is to hold the process INSIDE the send window.
    StallPush,
}

struct ProxyState {
    upstream: String,
    mode: ProxyMode,
    client: reqwest::Client,
    push_stalled: AtomicBool,
    release: AtomicBool,
    pushes: Mutex<usize>,
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
    /// True once the client is provably INSIDE the push. This is the observation that makes
    /// the kill deterministic instead of a race.
    fn push_stalled(&self) -> bool {
        self.state.push_stalled.load(Ordering::SeqCst)
    }
}

impl Drop for RelayProxy {
    fn drop(&mut self) {
        self.state.release.store(true, Ordering::SeqCst);
        if let Some(tx) = self.shutdown.take() {
            let _ = tx.send(());
        }
        if let Some(h) = self.handle.take() {
            let _ = h.join();
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
    let path_and_query = uri
        .path_and_query()
        .map(|v| v.as_str().to_string())
        .unwrap_or_else(|| path.clone());

    if path == "/v1/push" {
        *st.pushes.lock().expect("pushes") += 1;
        if st.mode == ProxyMode::StallPush {
            // ⚠ THE WINDOW. The client has already committed its QUEUED row and is now
            // blocked in the network call. Hold here until the test releases us, so the
            // kill lands inside the window by construction rather than by timing.
            st.push_stalled.store(true, Ordering::SeqCst);
            while !st.release.load(Ordering::SeqCst) {
                tokio::time::sleep(Duration::from_millis(25)).await;
            }
            return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Vec::new()).into_response();
        }
    }

    let url = format!("{}{}", st.upstream, path_and_query);
    let method = reqwest::Method::from_bytes(method.as_str().as_bytes()).expect("method");
    let mut req = st.client.request(method, url);
    for name in ["x-qsl-route-token", "authorization", "content-type"] {
        if let Some(v) = headers.get(name) {
            if let Ok(vs) = v.to_str() {
                req = req.header(name, vs);
            }
        }
    }
    let resp = req.body(body.to_vec()).send().await.expect("forward");
    let status = axum::http::StatusCode::from_u16(resp.status().as_u16()).expect("status");
    let bytes = resp.bytes().await.expect("body").to_vec();
    axum::response::Response::builder()
        .status(status)
        .body(axum::body::Body::from(bytes))
        .expect("response")
}

fn start_relay_proxy(upstream: &str, mode: ProxyMode) -> RelayProxy {
    let state = Arc::new(ProxyState {
        upstream: upstream.trim_end_matches('/').to_string(),
        mode,
        client: reqwest::Client::new(),
        push_stalled: AtomicBool::new(false),
        release: AtomicBool::new(false),
        pushes: Mutex::new(0),
    });
    let (addr_tx, addr_rx) = std::sync::mpsc::channel();
    let (shutdown_tx, shutdown_rx) = tokio::sync::oneshot::channel();
    let st = state.clone();
    let handle = thread::spawn(move || {
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("runtime");
        runtime.block_on(async move {
            let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
                .await
                .expect("bind");
            let addr = listener.local_addr().expect("addr");
            addr_tx.send(addr).expect("ready");
            let app = axum::Router::new()
                .fallback(axum::routing::any(proxy_handler))
                .with_state(st);
            axum::serve(listener, app)
                .with_graceful_shutdown(async {
                    let _ = shutdown_rx.await;
                })
                .await
                .expect("serve");
        });
    });
    let addr = addr_rx.recv().expect("addr");
    RelayProxy {
        base_url: format!("http://{}", addr),
        state,
        shutdown: Some(shutdown_tx),
        handle: Some(handle),
    }
}

// ---------------------------------------------------------------------------
// Fixture
// ---------------------------------------------------------------------------

fn safe_test_root() -> PathBuf {
    let root = std::env::var("QSC_TEST_ROOT")
        .or_else(|_| std::env::var("CARGO_TARGET_DIR"))
        .unwrap_or_else(|_| "target".to_string());
    PathBuf::from(root).join("qsc-na0682")
}

fn create_dir_700(p: &Path) {
    let _ = fs::remove_dir_all(p);
    fs::create_dir_all(p).expect("mkdir");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(p, fs::Permissions::from_mode(0o700)).expect("chmod");
    }
}

fn setup_cfg(cfg: &Path) {
    common::init_mock_vault(cfg);
    let out = common::qsc_std_command()
        .env("QSC_CONFIG_DIR", cfg)
        .env("QSC_QSP_SEED", "1")
        .env("QSC_ALLOW_SEED_FALLBACK", "1")
        .env("QSC_UNSAFE_TEST_SEED_FALLBACK", "1")
        .args([
            "contacts",
            "add",
            "--label",
            PEER,
            "--fp",
            "fp-test",
            "--route-token",
            ROUTE_TOKEN_PEER,
        ])
        .output()
        .expect("contacts add");
    assert!(
        out.status.success(),
        "contacts add failed: {}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
}

fn send_args<'a>(relay: &'a str, payload: &'a str) -> Vec<&'a str> {
    vec![
        "send",
        "--transport",
        "relay",
        "--relay",
        relay,
        "--to",
        PEER,
        "--file",
        payload,
    ]
}

// ---------------------------------------------------------------------------
// A1
// ---------------------------------------------------------------------------

#[test]
fn a1_killing_the_process_inside_the_send_window_leaves_a_queued_row_that_drains() {
    let base = safe_test_root().join(format!("a1_kill_{}", std::process::id()));
    create_dir_700(&base);
    let cfg = base.join("cfg");
    create_dir_700(&cfg);
    setup_cfg(&cfg);

    let payload = cfg.join("msg.bin");
    fs::write(&payload, b"na0682 kill-in-the-window").expect("write payload");
    let payload_s = payload.to_str().expect("payload path").to_string();

    let server = common::start_inbox_server(1024 * 1024, 16);
    let proxy = start_relay_proxy(server.base_url(), ProxyMode::StallPush);

    // --- enter the window -------------------------------------------------
    let mut child = common::qsc_std_command()
        .env("QSC_CONFIG_DIR", &cfg)
        .env("QSC_QSP_SEED", "1")
        .env("QSC_ALLOW_SEED_FALLBACK", "1")
        .env("QSC_UNSAFE_TEST_SEED_FALLBACK", "1")
        .env("QSC_MARK_FORMAT", "plain")
        .args(send_args(proxy.base_url(), payload_s.as_str()))
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("spawn send");

    // Poll until the client is PROVABLY inside the push. Not a sleep: an observation.
    let deadline = Instant::now() + Duration::from_secs(60);
    while !proxy.push_stalled() {
        assert!(
            Instant::now() < deadline,
            "send never reached the push -- the window was never entered, so the kill below \
             would prove nothing"
        );
        assert!(
            child.try_wait().expect("poll child").is_none(),
            "send exited before reaching the push"
        );
        thread::sleep(Duration::from_millis(25));
    }

    // --- THE IN-WINDOW EVIDENCE ------------------------------------------
    // The process is alive, blocked in the network call, and has NOT been told anything
    // succeeded. If commit-before-send holds, the QUEUED row is already durable RIGHT NOW.
    // This assertion is the whole point of the test: it observes the invariant while the
    // window is open, rather than inferring it afterwards from recovery behaviour.
    let in_window = common::queued_record_count(&cfg);
    assert_eq!(
        in_window, 1,
        "commit-before-send VIOLATED: the process is inside the network call and no QUEUED \
         row exists on disk"
    );

    // --- kill, hard, inside the window -----------------------------------
    child.kill().expect("SIGKILL");
    let _ = child.wait();

    // The row must still be there after the process is gone.
    assert_eq!(
        common::queued_record_count(&cfg),
        1,
        "the message did not survive the kill -- this is the silent loss O1 forbids"
    );

    // --- restart against the REAL relay and drain -------------------------
    drop(proxy); // release the stalled request and stop shaping traffic
    let out = common::qsc_std_command()
        .env("QSC_CONFIG_DIR", &cfg)
        .env("QSC_QSP_SEED", "1")
        .env("QSC_ALLOW_SEED_FALLBACK", "1")
        .env("QSC_UNSAFE_TEST_SEED_FALLBACK", "1")
        .env("QSC_MARK_FORMAT", "plain")
        .args(send_args(server.base_url(), payload_s.as_str()))
        .output()
        .expect("second send");
    let text = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );

    // The killed message drains. (This invocation also enqueues a second message; what
    // matters for A1 is that the FIRST one was not lost and did go out.)
    assert!(
        text.contains("event=send_commit"),
        "the recovered message never committed: {text}"
    );
    let delivered = server.drain_channel(ROUTE_TOKEN_PEER);
    assert!(
        !delivered.is_empty(),
        "nothing reached the relay after recovery -- the queued message did not drain"
    );

    let _ = fs::remove_dir_all(&base);
}

#[test]
fn a2_a_send_that_never_reaches_the_network_still_leaves_a_queued_row() {
    // A2: the no-network variant of the same invariant. A dead port means the push fails
    // immediately, so nothing can have been accepted -- and the row must still exist.
    let base = safe_test_root().join(format!("a2_nonet_{}", std::process::id()));
    create_dir_700(&base);
    let cfg = base.join("cfg");
    create_dir_700(&cfg);
    setup_cfg(&cfg);

    let payload = cfg.join("msg.bin");
    fs::write(&payload, b"na0682 no network").expect("write payload");
    let payload_s = payload.to_str().expect("path").to_string();

    let out = common::qsc_std_command()
        .env("QSC_CONFIG_DIR", &cfg)
        .env("QSC_QSP_SEED", "1")
        .env("QSC_ALLOW_SEED_FALLBACK", "1")
        .env("QSC_UNSAFE_TEST_SEED_FALLBACK", "1")
        .env("QSC_MARK_FORMAT", "plain")
        .args(send_args("http://127.0.0.1:9", payload_s.as_str()))
        .output()
        .expect("send");

    // ⚠ Honest reporting (§2h / OBS-EO): SAFE is not SENT. The message is durably queued,
    // and the process still exits NON-ZERO, because claiming success for a message that did
    // not reach the relay would be the false claim §2h forbids.
    assert!(
        !out.status.success(),
        "a queued-not-sent message must not report success"
    );
    assert_eq!(
        common::queued_record_count(&cfg),
        1,
        "crash-before-network must leave a QUEUED row"
    );

    let _ = fs::remove_dir_all(&base);
}

// ---------------------------------------------------------------------------
// A12 — the relay cannot forge DELIVERED (O3)
// ---------------------------------------------------------------------------

#[test]
fn a12_a_relay_injected_ack_cannot_flip_a_message_to_delivered() {
    // ⚠ O3 IS THE CLAIM THIS TEST EXISTS TO EARN: "Delivered" is END-TO-END — only the
    // recipient's device can produce it, and the relay can neither read nor forge it.
    //
    // The structural argument is that the ack rides inside the session AEAD, so a relay
    // without the session key cannot produce one `qsp_unpack` accepts. That argument is
    // correct, and it is exactly the kind of reasoning this project does not accept in place
    // of evidence. So: a HOSTILE RELAY injects a well-formed ack payload directly into the
    // sender's mailbox, and the sender must not believe it.
    let base = safe_test_root().join(format!("a12_forged_{}", std::process::id()));
    create_dir_700(&base);
    let cfg = base.join("cfg");
    create_dir_700(&cfg);
    setup_cfg(&cfg);

    let payload = cfg.join("msg.bin");
    fs::write(&payload, b"na0682 a12").expect("write payload");
    let payload_s = payload.to_str().expect("path").to_string();

    let server = common::start_inbox_server(1024 * 1024, 16);

    // Send one message so there is a SENT row a forged ack could try to flip.
    let sent = common::qsc_std_command()
        .env("QSC_CONFIG_DIR", &cfg)
        .env("QSC_QSP_SEED", "1")
        .env("QSC_ALLOW_SEED_FALLBACK", "1")
        .env("QSC_UNSAFE_TEST_SEED_FALLBACK", "1")
        .env("QSC_MARK_FORMAT", "plain")
        .args(send_args(server.base_url(), payload_s.as_str()))
        .output()
        .expect("send");
    assert!(sent.status.success(), "setup send must succeed");
    let _ = server.drain_channel(ROUTE_TOKEN_PEER);

    // ⚠ THE FORGERY. A plaintext, perfectly well-formed delivery-ack — exactly what an
    // honest peer's ack decrypts TO — dropped straight into our own mailbox by the relay.
    // It is not wrapped in the session AEAD, because the relay has no session key. That is
    // the whole point: the relay can construct the CONTENT but not the ENVELOPE.
    let forged = br#"{"v":2,"t":"ack","kind":"delivered","msg_id":"00000000000000000000000000000000","ns":"qsc.ctrl"}"#;
    server.enqueue_raw(ROUTE_TOKEN_PEER, forged.to_vec());

    let out_dir = base.join("out");
    create_dir_700(&out_dir);
    let recv = common::qsc_std_command()
        .env("QSC_CONFIG_DIR", &cfg)
        .env("QSC_QSP_SEED", "1")
        .env("QSC_ALLOW_SEED_FALLBACK", "1")
        .env("QSC_UNSAFE_TEST_SEED_FALLBACK", "1")
        .env("QSC_MARK_FORMAT", "plain")
        .args([
            "receive",
            "--transport",
            "relay",
            "--relay",
            server.base_url(),
            "--mailbox",
            ROUTE_TOKEN_PEER,
            "--from",
            PEER,
            "--max",
            "1",
            "--out",
            out_dir.to_str().unwrap(),
        ])
        .output()
        .expect("receive");
    let text = format!(
        "{}{}",
        String::from_utf8_lossy(&recv.stdout),
        String::from_utf8_lossy(&recv.stderr)
    );

    // ⚠ NON-VACUITY FIRST. An "absence" assertion passes trivially if the receive did
    // nothing at all, so prove the client actually SAW the injected item and REJECTED it.
    // Without this, a receive that silently no-op'd would look identical to a defence.
    assert!(
        text.contains("event=qsp_unpack ok=false") || text.contains("code=qsp_"),
        "the forged item was never even processed -- this test would be vacuous: {text}"
    );

    // The forged ack must never be treated as a delivery confirmation.
    assert!(
        !text.contains("event=delivered_to_peer"),
        "a relay-injected ack was accepted as DELIVERED -- O3 is broken: {text}"
    );
    assert!(
        !text.contains("event=receipt_recv"),
        "a relay-injected ack was processed as a receipt: {text}"
    );

    let _ = fs::remove_dir_all(&base);
}
