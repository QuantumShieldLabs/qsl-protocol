//! NA-0708 (D643 A3, decision D-1345) — **THE STRANDED-ACK DEFECT.**
//!
//! Acks queued inside `receive_pull_and_write`'s `'pull:` loop are lost whenever the pull leaves
//! that function through one of its early exits, because `flush_pending_acks` sits AFTER the loop
//! (`transport/mod.rs:1222`) and every early exit jumps over it.
//!
//! ⚠ **WHAT THIS FILE ASSERTS, AND WHAT IT DELIBERATELY DOES NOT.** It asserts **ACK STATE** — that
//! the acks earned before the failure actually reach the relay. It does **NOT** assert that the
//! receive succeeds: NA-0708 does **not** fix the whole-pull abort, and both arms still exit
//! non-zero after the fix. A row that asserted success would be asserting the successor lane's
//! work, and would go green for the wrong reason.
//!
//! ⚠ **THE SEVERITY, STATED HONESTLY (SR-15 F-15).** A stranded ack is **self-healing under
//! Lease**: every queued id is already in the durable seen store, so the relay's redelivery is
//! dup-skipped and acked on the next receive. The cost is a lease-expiry round trip and a
//! redundant redelivery — **not loss, and not reprocessing.** So the RED these arms record is
//! "the item comes back once", never "the item is gone".
//!
//! ⚠ **LEASE-ONLY BY CONSTRUCTION.** Both `pending_acks` push sites sit inside `if let Some(seen)`
//! (`transport/mod.rs:472`, `:1365`) and `seen_ids` is `None` under Legacy (`:448-456`), so under
//! Legacy the accumulator is **always empty** and `flush_pending_acks` is a no-op. **Legacy cannot
//! strand at all**, which is why every arm here pins `--ack-mode lease` explicitly rather than
//! inheriting the C4 default.
//!
//! ⚠ **THE RELAY MUST BE THE REAL ONE.** The test-local mock in `common` parses only `max=` and
//! always pops on pull, so it cannot express lease semantics and would make both arms vacuous —
//! the same reason `na0689_p3_a2_stranding.rs:39-41` gives. Here the real in-process `qsl-server`
//! runs behind a **recording proxy** adopted from `NA_0644_ack_client.rs`, which counts the ack
//! POSTs directly instead of inferring them.
//!
//! ## THE TWO ARMS, AND WHY BOTH ARE NEEDED
//!
//! | arm | the exit it covers | how it is reached |
//! |---|---|---|
//! | **R3a** | `:1210` — the reject arm, **inside** `for item` | a garbage frame behind a real message |
//! | **R3b** | `:462` — the pull failure, **outside** `for item` | a control envelope in round 1, then a failed round-2 pull |
//!
//! ⚠ **R3b is the arm that decides the CONSTRUCTION.** `:462` sits inside `'pull:` but outside
//! `for item`, so a wrapper enclosing only the `for` body — the natural reading of "the loop's
//! early exits" — would leave it stranding. It is reachable with a plain network blip: alice's
//! first send emits an SCKA advertisement as its own item (`qsp_scka_advertise_due` is `true` while
//! `live_advkey()` is `None`), that control envelope queues an ack and increments `controls`
//! without touching `stats.count` (`transport/mod.rs:520-524`), so with `--max 8` the loop takes a
//! second pull (`:1215`) — and that pull is the one the proxy fails.

mod common;

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, MutexGuard, OnceLock};
use std::thread;
use std::time::Duration;

const ALICE_INBOX: &str = "na0708-alice-inbox-token-aaaaaaaa";
const BOB_INBOX: &str = "na0708-bob-inbox-token-bbbbbbbb";

/// A 1-second server-side pull lease, so an unacked item becomes visible again quickly.
/// The same value `NA_0644_ack_client.rs`, `na0688_c4_collateral_arms.rs` and
/// `na0689_p3_a2_stranding.rs` use to prove lease redelivery.
// ⚠⚠ NA-0770 (D-1411) WIDENED THESE 1s/2500ms -> 8s/20000ms. THE RECORDED REASON IS MARGIN
// AGAINST CONTENTION, and it is stated so a later reader does not "tidy" them back.
//
// Before this lane, arms that wanted a NEGATIVE result reached for delete-on-pull, which is
// instantaneous and needs no waiting. With the mode retired, every such arm must instead wait out a
// lease — so the suite's dependence on these two numbers went UP at the moment the mode went away.
// They are now load-bearing in shards that run twelve-wide on six cores, where a 2500ms wait
// against a 1s lease left almost no margin: an overrun does not merely slow the arm, it reports a
// perfectly intact message as lost. The pair is kept in step across every file that defines it
// (`NA_0644`, `na0688`, `na0689_capture`, `na0689_p3_a2`, `na0690`, `na0708`, `na0741`, and
// `na0742` under its own names) — if you change one, change all of them.
const TEST_PULL_LEASE_SECS: usize = 8;
const LEASE_EXPIRY_WAIT: Duration = Duration::from_millis(20_000);

fn guard() -> MutexGuard<'static, ()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
        .lock()
        .unwrap_or_else(|e| e.into_inner())
}

fn output_text(out: &std::process::Output) -> String {
    format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    )
}

fn ensure_dir_700(path: &Path) {
    fs::create_dir_all(path).expect("create dir");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut p = fs::metadata(path).expect("meta").permissions();
        p.set_mode(0o700);
        fs::set_permissions(path, p).expect("chmod");
    }
}

fn test_root(tag: &str) -> PathBuf {
    let root = common::unique_test_root(tag);
    ensure_dir_700(&root);
    root
}

// ---------------------------------------------------------------------------
// Party / contact / send scaffolding — adopted wholesale from
// `na0690_ack_gated_on_store.rs` rather than re-derived beside it, so a setup difference
// cannot masquerade as this lane's result.
// ---------------------------------------------------------------------------
fn qsc(cfg: &Path) -> Command {
    // ⚠ `qsc_std_command()` ALREADY applies the mock-vault unlock args; re-adding them makes clap
    // reject `--unlock-passphrase-env` as repeated and fails setup before any measurement runs.
    let mut c = common::qsc_std_command();
    c.env("QSC_CONFIG_DIR", cfg)
        .env("QSC_QSP_SEED", "1")
        .env("QSC_ALLOW_SEED_FALLBACK", "1")
        .env("QSC_UNSAFE_TEST_SEED_FALLBACK", "1")
        .env("QSC_MARK_FORMAT", "plain");
    c
}

fn run_ok(cfg: &Path, args: &[&str]) -> String {
    let out = qsc(cfg).args(args).output().expect("run qsc");
    let text = output_text(&out);
    assert!(out.status.success(), "expected success: {args:?}\n{text}");
    text
}

fn run_any(cfg: &Path, args: &[&str]) -> (bool, String) {
    let out = qsc(cfg).args(args).output().expect("run qsc");
    (out.status.success(), output_text(&out))
}

fn party(root: &Path, name: &str, inbox: &str) -> PathBuf {
    let cfg = root.join(name);
    ensure_dir_700(&cfg);
    common::init_mock_vault(&cfg);
    run_ok(&cfg, &["identity", "rotate", "--confirm"]);
    run_ok(&cfg, &["relay", "inbox-set", "--token", inbox]);
    cfg
}

fn fingerprint(cfg: &Path) -> String {
    run_ok(cfg, &["identity", "show"])
        .lines()
        .find_map(|l| l.strip_prefix("identity_fp="))
        .expect("identity_fp")
        .trim()
        .to_string()
}

/// ⚠ Adding the contact is NOT enough to send: its device must also be TRUSTED.
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

fn setup(root: &Path) -> (PathBuf, PathBuf) {
    let alice = party(root, "alice", ALICE_INBOX);
    let bob = party(root, "bob", BOB_INBOX);
    let alice_fp = fingerprint(&alice);
    let bob_fp = fingerprint(&bob);
    add_contact(&alice, "bob", &bob_fp, BOB_INBOX);
    add_contact(&bob, "bob", &alice_fp, ALICE_INBOX);
    (alice, bob)
}

fn send_message(alice: &Path, relay: &str, base: &Path, name: &str, bytes: &[u8]) {
    let msg = base.join(name);
    fs::write(&msg, bytes).expect("write msg");
    let text = run_ok(
        alice,
        &[
            "send",
            "--transport",
            "relay",
            "--relay",
            relay,
            "--to",
            "bob",
            "--file",
            msg.to_str().expect("msg path"),
        ],
    );
    assert!(
        text.contains("QSC_DELIVERY state=accepted_by_relay"),
        "{text}"
    );
}

/// ⚠ **LEASE IS PASSED EXPLICITLY**, never inherited: the defect is unobservable under Legacy
/// (the ack accumulator is never populated there), so each arm states the mode it needs.
/// `--max 8` is load-bearing for R3b — it keeps `stats.count < max` so the control round
/// re-pulls.
fn receive_args<'a>(relay: &'a str, out: &'a str) -> Vec<&'a str> {
    vec![
        "receive",
        "--transport",
        "relay",
        "--relay",
        relay,
        "--mailbox",
        BOB_INBOX,
        "--from",
        "bob",
        "--max",
        "8",
        "--out",
        out,
    ]
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

/// Put a frame into a mailbox that `qsp_unpack` can never process, through the relay's own
/// public push route — the same route the client uses (`transport/mod.rs:2840`).
///
/// ⚠ Garbage is the honest choice here: it dies at **envelope decode**, upstream of every
/// quarantine site, which is exactly the shape SR-15 F-1 measured for a bare handshake frame.
fn push_unprocessable(base: &str, route_token: &str) {
    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .expect("push client");
    let resp = client
        .post(format!("{}/v1/push", base.trim_end_matches('/')))
        .header("X-QSL-Route-Token", route_token)
        .body(b"na0708-not-an-envelope".to_vec())
        .send()
        .expect("push unprocessable frame");
    assert!(
        resp.status().is_success(),
        "the relay refused the frame, so the arm would measure nothing: {}",
        resp.status()
    );
}

// ---------------------------------------------------------------------------
// THE RECORDING PROXY — adopted from `NA_0644_ack_client.rs` with one mode added.
// The relay behind it is the REAL in-process `qsl-server`; the proxy only counts, and (in
// `FailPullAfter`) breaks one hop, which is precisely the "plain network blip" that makes
// `:462` reachable.
// ---------------------------------------------------------------------------
struct ProxyState {
    upstream: String,
    client: reqwest::Client,
    pulls: AtomicUsize,
    ack_posts: AtomicUsize,
    /// Pull ordinal at and after which the proxy answers 500. `usize::MAX` = never.
    /// ⚠ ARMED LATE, ON PURPOSE: the handshake in R3b pulls several times during setup, and a
    /// threshold fixed at construction would break those instead of the measured re-pull.
    fail_from: AtomicUsize,
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
    /// The number of POSTs to `/v1/pull/ack` — **the direct ack-state observable**, counted at
    /// the wire rather than inferred from a marker.
    fn ack_posts(&self) -> usize {
        self.state.ack_posts.load(Ordering::SeqCst)
    }
    fn pulls(&self) -> usize {
        self.state.pulls.load(Ordering::SeqCst)
    }
    // NA-0770 (D-1411): `arm_fail_after` removed — its only caller was `r3b`, retired (L3).
}

impl Drop for RelayProxy {
    fn drop(&mut self) {
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
    let path_and_query = uri
        .path_and_query()
        .map(|v| v.as_str().to_string())
        .unwrap_or_else(|| path.clone());

    if path == "/v1/pull/ack" {
        st.ack_posts.fetch_add(1, Ordering::SeqCst);
    }
    if path == "/v1/pull" {
        let n = st.pulls.fetch_add(1, Ordering::SeqCst);
        if n >= st.fail_from.load(Ordering::SeqCst) {
            // The blip: this pull never reaches the relay.
            return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Vec::new()).into_response();
        }
    }

    let url = format!("{}{}", st.upstream, path_and_query);
    let method = reqwest::Method::from_bytes(method.as_str().as_bytes()).expect("proxy method");
    let mut req = st.client.request(method, url);
    for name in [
        "x-qsl-route-token",
        "x-qsl-invite-ticket",
        "authorization",
        "content-type",
    ] {
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

fn start_relay_proxy(upstream: &str) -> RelayProxy {
    let state = Arc::new(ProxyState {
        upstream: upstream.trim_end_matches('/').to_string(),
        client: reqwest::Client::new(),
        pulls: AtomicUsize::new(0),
        ack_posts: AtomicUsize::new(0),
        fail_from: AtomicUsize::new(usize::MAX),
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
            addr_tx.send(addr).expect("proxy addr send");
            let router = axum::Router::new()
                .fallback(axum::routing::any(proxy_handler))
                .with_state(st);
            axum::serve(listener, router)
                .with_graceful_shutdown(async {
                    let _ = shutdown_rx.await;
                })
                .await
                .expect("proxy serve");
        });
    });
    let addr = addr_rx.recv().expect("proxy addr recv");
    RelayProxy {
        base_url: format!("http://{}", addr),
        state,
        shutdown: Some(shutdown_tx),
        handle: Some(handle),
    }
}

// ---------------------------------------------------------------------------
// R3a — THE `:1210` EXIT. A garbage frame behind a real message.
// ---------------------------------------------------------------------------
#[test]
fn r3a_acks_earned_before_a_reject_reach_the_relay() {
    let _g = guard();
    let relay =
        common::start_qsl_server_with_store(2 * 1024 * 1024, 512, None, TEST_PULL_LEASE_SECS);
    let proxy = start_relay_proxy(relay.base_url());
    let base = proxy.base_url().to_string();
    let root = test_root("na0708_r3a");
    let (alice, bob) = setup(&root);

    // A real message, then a frame nothing can process, in that order.
    send_message(&alice, &base, &root, "m1.bin", b"na0708 r3a payload");
    push_unprocessable(&base, BOB_INBOX);

    let out1 = root.join("out1");
    ensure_dir_700(&out1);
    let (ok1, text1) = run_any(&bob, &receive_args(&base, out1.to_str().expect("out1")));

    // ⚠ The receive STILL FAILS after the fix — NA-0708 does not fix the abort. Asserting
    // success here would be asserting the successor lane's work.
    assert!(
        !ok1,
        "the unprocessable frame must still abort the pull; this lane does not fix that:\n{text1}"
    );
    assert_eq!(
        recv_file_count(&out1),
        1,
        "the real message must have been processed BEFORE the reject, or the arm measures \
         nothing:\n{text1}"
    );

    // THE ASSERTION: the ack earned by the message that DID process must have reached the relay.
    assert!(
        proxy.ack_posts() >= 1,
        "STRANDED: the message was processed and its ack queued, but no ack POST reached the \
         relay — `flush_pending_acks` was jumped over by the early exit at \
         `transport/mod.rs:1210`. ack_posts={} pulls={}\n{text1}",
        proxy.ack_posts(),
        proxy.pulls()
    );
    assert!(
        text1.contains("relay_ack"),
        "the relay_ack marker must witness the flush:\n{text1}"
    );

    // The consequence, stated as SR-15 F-15 corrects it: a strand makes the item COME BACK ONCE,
    // it does not lose it. After the fix it must not come back at all.
    thread::sleep(LEASE_EXPIRY_WAIT);
    let out2 = root.join("out2");
    ensure_dir_700(&out2);
    let (_ok2, text2) = run_any(&bob, &receive_args(&base, out2.to_str().expect("out2")));
    assert!(
        !text2.contains("recv_dup_skipped"),
        "the acked message came back on redelivery, so its ack never landed:\n{text2}"
    );
}

// ---------------------------------------------------------------------------
// R3b — THE `:462` EXIT. The arm a `for`-body wrapper would miss.
//
// ⚠ **WHY THIS ARM NEEDS A REAL HANDSHAKE, MEASURED NOT ASSUMED.** Reaching `:462` with a
// non-empty ack accumulator requires `controls > 0`, and only a CONTROL envelope bumps it
// (`transport/mod.rs:520-524`). A control envelope requires SCKA, and
// `qsp_scka_enabled` is `st.dh.dhr != st.dh.dhs_pub` (`lib.rs:1725-1727`) — but the seed
// fallback sets BOTH to the same value (`protocol_state/mod.rs:1064-1101`: `dhs_pub: dh_pub,
// dhr: dh_pub`), so **SCKA is off in every seed-derived session and no control envelope can
// ever exist there.** The first draft of this arm assumed otherwise and measured `pulls=1`;
// the assertion below is written so that mistake fails loudly instead of passing quietly.
//
// With a real handshake, `qsp_scka_advertise_due` is true while `live_advkey()` is `None`
// (`lib.rs:1732-1740`), and the advertisement is pushed as its **own relay item** before the
// message (`lib.rs:1570-1573` pushes each `pre_envelope` separately) — the two-item shape.
// ---------------------------------------------------------------------------
// NA-0770 (D-1411): `drive_handshake` removed — its only caller was `r3b`, retired above (L3).
// NA-0770 (D-1411): `receive_args_from` removed — its only caller was `r3b`, retired above (L3).
// ---------------------------------------------------------------------------
// NA-0770 (D-1411): `r3b_acks_earned_before_a_failed_repull_reach_the_relay` RETIRED.
//
// ⚠⚠ IT IS RETIRED FOR A CAPABILITY, NOT FOR A CONTROL — and that distinction is the whole
// entry. Its SUBJECT (acks earned before a failed re-pull still reach the relay) survives the
// retirement untouched. What does not survive is its SETUP.
//
// The arm had to clear a poison frame from the relay mailbox first: bob's receive died at
// `qsp_env_decode_failed` on that frame before any control envelope could be reached — this
// program's own UNFIXED wedge standing between the instrument and the thing it measures. It
// cleared it with `--ack-mode legacy`, "precisely BECAUSE legacy delete-on-delivers: the relay
// drops what it hands over, so one pass empties the mailbox even though the client still errors
// on the frame."
//
// ⚠⚠ LOSS L3, AND IT OUTLIVES THIS TEST. Measured across the whole CLI: `QuarantineCmd` is a
// LOCAL store, `Outbox Discard` is the OUTBOUND queue, and `relay_inbox_ack` is unreachable for
// a frame the client cannot persist. ⇒ AFTER THIS LANE NO `qsc` COMMAND CAN CLEAR A RELAY
// MAILBOX HEAD CARRYING AN UNDECODABLE FRAME, and such a frame redelivers forever. Under lease
// the drain simply leases the frame and it comes back, so the setup cannot be reconstructed —
// which is why the test retires rather than being re-aimed.
//
// ⚠ THIS IS A CONSEQUENCE OF THE RETIREMENT, NOT A DEFECT INTRODUCED BY IT: the eviction verb
// never existed; delete-on-pull was standing in for one. It STRENGTHENS the case for scheduling
// the `ENG-0142`/`ENG-0198` wedge repair, which is where the replacement belongs. An eviction
// verb, if built, must also add itself to G-A's and G-B's asserted sets or turn them red.
// ---------------------------------------------------------------------------
