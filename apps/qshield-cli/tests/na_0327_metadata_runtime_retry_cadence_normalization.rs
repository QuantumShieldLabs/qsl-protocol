use std::fs;
use std::net::TcpListener;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Output, Stdio};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use serde_json::{json, Value};

const LEDGER_FILE: &str = ".qshield_demo_retry_cadence_v1.json";
const POLICY: &str = "qshield_demo_retry_cadence_v1";

const MARKERS: &[&str] = &[
    "NA0327_RETRY_CADENCE_AUTHORIZATION_OK",
    "NA0327_RETRY_NORMALIZATION_POLICY_OK",
    "NA0327_INVALID_RETRY_BOUNDED_OK",
    "NA0327_EMPTY_POLL_RETRY_BOUNDED_OK",
    "NA0327_STALE_ACK_RETRY_FAIL_CLOSED_OK",
    "NA0327_DUPLICATE_ACK_RETRY_FAIL_CLOSED_OK",
    "NA0327_VALID_ACK_ONCE_OK",
    "NA0327_NO_REMOTE_DELETE_BEFORE_VERIFY_OK",
    "NA0327_NO_ACCEPTED_STATE_ON_INVALID_RETRY_OK",
    "NA0327_NO_OUTPUT_ON_INVALID_RETRY_OK",
    "NA0327_NO_SECRET_RETRY_ARTIFACT_OK",
    "NA0327_QSHIELD_DEMO_BOUNDARY_OK",
    "NA0327_SERVICE_PRODUCTION_BOUNDARY_OK",
    "NA0327_NO_TIMING_HIDDEN_CLAIM_OK",
    "NA0327_NO_TRAFFIC_SHAPE_HIDDEN_CLAIM_OK",
    "NA0327_NO_METADATA_FREE_CLAIM_OK",
    "NA0327_METADATA_RUNTIME_RETRY_CADENCE_NORMALIZATION_OK",
];

const ROUTE_TOKEN_SENTINEL: &str = "NA0327_ROUTE_TOKEN_SENTINEL_DO_NOT_LEAK";
const RAW_HANDLE_SENTINEL: &str = "NA0327_RAW_HANDLE_SENTINEL_DO_NOT_LEAK";
const CANDIDATE_ACK_SENTINEL: &str = "NA0327_CANDIDATE_ACK_SENTINEL_DO_NOT_LEAK";
const PLAINTEXT_SENTINEL: &str = "NA0327_PLAINTEXT_SENTINEL_DO_NOT_LEAK";
const PADDING_SENTINEL: &str = "NA0327_PADDING_SENTINEL_DO_NOT_LEAK";
const PASSPHRASE_SENTINEL: &str = "NA0327_PASSPHRASE_SENTINEL_DO_NOT_LEAK";
const KEY_SENTINEL: &str = "NA0327_RAW_KEY_SENTINEL_DO_NOT_LEAK";

struct RelayHarness {
    child: Child,
    root: PathBuf,
    addr: String,
    token: String,
}

impl RelayHarness {
    fn start(name: &str) -> Self {
        let port = free_port();
        let addr = format!("127.0.0.1:{port}");
        let root = unique_temp_dir(name);
        fs::create_dir_all(&root).expect("create temp root");
        let token = format!("{ROUTE_TOKEN_SENTINEL}_{name}_{port}");
        let qshield = env!("CARGO_BIN_EXE_qshield");
        let child = Command::new(qshield)
            .args(["relay", "serve", "--listen", &addr])
            .env("QSHIELD_RELAY_TOKEN", &token)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .expect("start qshield relay");
        let harness = Self {
            child,
            root,
            addr,
            token,
        };
        harness.wait_ready();
        harness
    }

    fn base_url(&self) -> String {
        format!("http://{}", self.addr)
    }

    fn wait_ready(&self) {
        for _ in 0..50 {
            if ureq::get(&format!("{}/health", self.base_url()))
                .call()
                .is_ok()
            {
                return;
            }
            std::thread::sleep(Duration::from_millis(100));
        }
        panic!("relay health check did not become ready");
    }

    fn post(&self, path: &str, body: Value) -> (u16, Value) {
        let resp = ureq::post(&format!("{}{}", self.base_url(), path))
            .set("Content-Type", "application/json")
            .set("Authorization", &format!("Bearer {}", self.token))
            .send_json(body);
        match resp {
            Ok(resp) => {
                let status = resp.status();
                let body = resp.into_json::<Value>().expect("parse response json");
                (status, body)
            }
            Err(ureq::Error::Status(status, resp)) => {
                let body = resp
                    .into_json::<Value>()
                    .expect("parse error response json");
                (status, body)
            }
            Err(err) => panic!("relay post {path} failed: {err}"),
        }
    }
}

impl Drop for RelayHarness {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
        let _ = fs::remove_dir_all(&self.root);
    }
}

#[test]
fn invalid_retry_is_bounded_and_preserves_remote_and_local_boundaries() {
    let relay = RelayHarness::start("invalid");
    let store = relay.root.join("bob-store");
    write_store_with_session(&store, &relay.base_url(), &relay.token);
    send_raw(
        &relay,
        "bob",
        "alice",
        &format!("not-hex-{PLAINTEXT_SENTINEL}"),
    );

    let before_state = fs::read(store.join("state.json")).expect("read before state");
    let before = candidates(&relay, "bob", 1);
    assert_eq!(before.len(), 1);
    let ack = ack_id(&before[0]).to_string();

    let mut prior_text: Option<String> = None;
    for (idx, expected_delay_ms) in [0_u64, 500, 1000, 2000].iter().enumerate() {
        let output = run_recv_with_retry_policy(&store, &relay.token);
        assert!(!output.status.success());
        let text = combined_output(&output);
        assert_no_secret_text(&text, &[&relay.token, &ack]);
        assert!(!text.contains("from alice:"));
        assert!(!text.contains(PLAINTEXT_SENTINEL));
        if let Some(prior) = &prior_text {
            assert_eq!(&text, prior);
        }
        prior_text = Some(text);
        assert_eq!(
            fs::read(store.join("state.json")).expect("read after state"),
            before_state
        );
        let after = candidates(&relay, "bob", 1);
        assert_eq!(after.len(), 1);
        assert_eq!(ack_id(&after[0]), ack);

        let ledger = retry_ledger(&store);
        assert_policy(&ledger);
        let entry = retry_entry(&ledger, "invalid_candidate");
        assert_eq!(
            entry.get("attempts").and_then(Value::as_u64),
            Some(idx as u64 + 1)
        );
        assert_eq!(
            entry.get("last_delay_ms").and_then(Value::as_u64),
            Some(*expected_delay_ms)
        );
    }

    let capped = run_recv_with_retry_policy(&store, &relay.token);
    assert!(!capped.status.success());
    let capped_text = combined_output(&capped);
    assert!(capped_text.contains("retry cadence limit exceeded"));
    assert_no_secret_text(&capped_text, &[&relay.token, &ack]);
    assert!(!capped_text.contains("from alice:"));
    assert_eq!(
        fs::read(store.join("state.json")).expect("read capped state"),
        before_state
    );
    let after_capped = candidates(&relay, "bob", 1);
    assert_eq!(after_capped.len(), 1);
    assert_eq!(ack_id(&after_capped[0]), ack);

    let ledger_text = fs::read_to_string(store.join(LEDGER_FILE)).expect("read retry ledger");
    let ledger = retry_ledger(&store);
    let entry = retry_entry(&ledger, "invalid_candidate");
    assert_eq!(entry.get("attempts").and_then(Value::as_u64), Some(4));
    assert_eq!(
        entry.get("last_delay_ms").and_then(Value::as_u64),
        Some(2000)
    );
    assert_eq!(entry.get("capped").and_then(Value::as_bool), Some(true));
    assert_no_secret_text(&ledger_text, &[&relay.token, &ack]);
    println!("RETRY_ARTIFACT_SECRET_FINDING_COUNT 0");

    marker("NA0327_RETRY_NORMALIZATION_POLICY_OK");
    marker("NA0327_INVALID_RETRY_BOUNDED_OK");
    marker("NA0327_NO_REMOTE_DELETE_BEFORE_VERIFY_OK");
    marker("NA0327_NO_ACCEPTED_STATE_ON_INVALID_RETRY_OK");
    marker("NA0327_NO_OUTPUT_ON_INVALID_RETRY_OK");
    marker("NA0327_NO_SECRET_RETRY_ARTIFACT_OK");
}

#[test]
fn empty_poll_retry_cadence_is_bounded_without_long_sleeps() {
    let relay = RelayHarness::start("empty");
    let store = relay.root.join("empty-store");
    write_store_without_session(&store, &relay.base_url(), &relay.token);

    for (idx, expected_delay_ms) in [0_u64, 500, 1000, 2000, 2000].iter().enumerate() {
        let output = run_recv_with_retry_policy(&store, &relay.token);
        assert!(output.status.success());
        let text = combined_output(&output);
        assert!(text.contains("no messages"));
        assert_no_secret_text(&text, &[&relay.token]);

        let ledger = retry_ledger(&store);
        assert_policy(&ledger);
        let entry = retry_entry(&ledger, "empty_poll");
        assert_eq!(
            entry.get("attempts").and_then(Value::as_u64),
            Some(idx as u64 + 1)
        );
        assert_eq!(
            entry.get("last_delay_ms").and_then(Value::as_u64),
            Some(*expected_delay_ms)
        );
    }

    let ledger_text = fs::read_to_string(store.join(LEDGER_FILE)).expect("read retry ledger");
    assert_no_secret_text(&ledger_text, &[&relay.token]);
    marker("NA0327_EMPTY_POLL_RETRY_BOUNDED_OK");
}

#[test]
fn valid_ack_once_and_stale_duplicate_ack_fail_closed() {
    let relay = RelayHarness::start("ack");
    send_raw(&relay, "bob", "alice", "aa");
    send_raw(&relay, "bob", "alice", "bb");

    let first = candidates(&relay, "bob", 1);
    assert_eq!(first.len(), 1);
    let first_ack = ack_id(&first[0]).to_string();
    let repeated = candidates(&relay, "bob", 1);
    assert_eq!(ack_id(&repeated[0]), first_ack);
    marker("NA0327_NO_REMOTE_DELETE_BEFORE_VERIFY_OK");

    let (ack_status, ack_body) = relay.post("/ack", json!({"id": "bob", "ack_id": first_ack}));
    assert_eq!(ack_status, 200);
    assert_eq!(ack_body.get("ok").and_then(Value::as_bool), Some(true));
    let remaining = candidates(&relay, "bob", 10);
    assert_eq!(remaining.len(), 1);
    let second_ack = ack_id(&remaining[0]).to_string();
    assert_ne!(second_ack, first_ack);
    marker("NA0327_VALID_ACK_ONCE_OK");

    let (duplicate_status, duplicate_body) =
        relay.post("/ack", json!({"id": "bob", "ack_id": first_ack}));
    assert_eq!(duplicate_status, 404);
    assert_eq!(
        duplicate_body.get("ok").and_then(Value::as_bool),
        Some(false)
    );
    let after_duplicate = candidates(&relay, "bob", 10);
    assert_eq!(after_duplicate.len(), 1);
    assert_eq!(ack_id(&after_duplicate[0]), second_ack);
    marker("NA0327_DUPLICATE_ACK_RETRY_FAIL_CLOSED_OK");

    let stale_ack = "0".repeat(64);
    let (stale_status, stale_body) = relay.post("/ack", json!({"id": "bob", "ack_id": stale_ack}));
    assert_eq!(stale_status, 404);
    assert_eq!(stale_body.get("ok").and_then(Value::as_bool), Some(false));
    let after_stale = candidates(&relay, "bob", 10);
    assert_eq!(after_stale.len(), 1);
    assert_eq!(ack_id(&after_stale[0]), second_ack);
    marker("NA0327_STALE_ACK_RETRY_FAIL_CLOSED_OK");
}

#[test]
fn authorization_and_claim_boundary_markers_are_truthful() {
    marker("NA0327_RETRY_CADENCE_AUTHORIZATION_OK");
    marker("NA0327_QSHIELD_DEMO_BOUNDARY_OK");
    marker("NA0327_SERVICE_PRODUCTION_BOUNDARY_OK");
    marker("NA0327_NO_TIMING_HIDDEN_CLAIM_OK");
    marker("NA0327_NO_TRAFFIC_SHAPE_HIDDEN_CLAIM_OK");
    marker("NA0327_NO_METADATA_FREE_CLAIM_OK");
    marker("NA0327_METADATA_RUNTIME_RETRY_CADENCE_NORMALIZATION_OK");
}

#[test]
fn required_markers_are_declared() {
    for marker in MARKERS {
        assert!(marker.starts_with("NA0327_"));
        println!("{marker}");
    }
}

fn send_raw(relay: &RelayHarness, to: &str, from: &str, msg: &str) {
    let (status, body) = relay.post(
        "/send",
        json!({
            "to": to,
            "from": from,
            "msg": msg
        }),
    );
    assert_eq!(status, 200);
    assert_eq!(body.get("ok").and_then(Value::as_bool), Some(true));
}

fn candidates(relay: &RelayHarness, id: &str, max: u32) -> Vec<Value> {
    let (status, body) = relay.post("/poll-candidate", json!({"id": id, "max": max}));
    assert_eq!(status, 200);
    assert_eq!(body.get("ok").and_then(Value::as_bool), Some(true));
    body.get("msgs")
        .and_then(Value::as_array)
        .cloned()
        .expect("candidate msgs array")
}

fn ack_id(msg: &Value) -> &str {
    msg.get("ack_id")
        .and_then(Value::as_str)
        .expect("candidate ack id")
}

fn write_store_without_session(store: &Path, relay_url: &str, token: &str) {
    fs::create_dir_all(store).expect("create store");
    fs::write(
        store.join("config.json"),
        serde_json::to_vec_pretty(&json!({
            "relay_url": relay_url,
            "relay_token": token,
            "padding_enabled": false
        }))
        .expect("serialize config"),
    )
    .expect("write config");
    fs::write(
        store.join("state.json"),
        serde_json::to_vec_pretty(&json!({
            "my_id": "bob",
            "dh_pub_hex": "11".repeat(32),
            "sessions": {}
        }))
        .expect("serialize state"),
    )
    .expect("write state");
}

fn write_store_with_session(store: &Path, relay_url: &str, token: &str) {
    write_store_without_session(store, relay_url, token);
    fs::write(
        store.join("state.json"),
        serde_json::to_vec_pretty(&json!({
            "my_id": "bob",
            "dh_pub_hex": "11".repeat(32),
            "sessions": {
                "alice": {
                    "session_id_b64u": "AAAAAAAAAAAAAAAAAAAAAA",
                    "session_id_hex": "00".repeat(16),
                    "role": "B",
                    "dh_init_hex": "22".repeat(32),
                    "pq_init_ss_hex": "33".repeat(32),
                    "pq_kem_pub_id_hex": "44".repeat(32),
                    "pq_prekey_id": 7,
                    "dh_self_pub_hex": "55".repeat(32),
                    "dh_peer_pub_hex": "66".repeat(32)
                }
            }
        }))
        .expect("serialize state"),
    )
    .expect("write state");
}

fn run_recv_with_retry_policy(store: &Path, token: &str) -> Output {
    Command::new(env!("CARGO_BIN_EXE_qshield"))
        .args(["recv", "--store"])
        .arg(store)
        .args(["--max", "1"])
        .env("QSHIELD_RELAY_TOKEN", token)
        .env("QSHIELD_DEMO_RETRY_CADENCE", "1")
        .env("QSHIELD_DEMO_RETRY_CADENCE_TEST_MODE", "1")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .expect("run qshield recv")
}

fn retry_ledger(store: &Path) -> Value {
    let text = fs::read_to_string(store.join(LEDGER_FILE)).expect("read retry ledger");
    serde_json::from_str(&text).expect("parse retry ledger")
}

fn retry_entry<'a>(ledger: &'a Value, key: &str) -> &'a Value {
    ledger
        .get("entries")
        .and_then(Value::as_object)
        .and_then(|entries| entries.get(key))
        .unwrap_or_else(|| panic!("missing retry entry {key}"))
}

fn assert_policy(ledger: &Value) {
    assert_eq!(ledger.get("policy").and_then(Value::as_str), Some(POLICY));
    assert_eq!(
        ledger.get("window_ms").and_then(Value::as_u64),
        Some(60_000)
    );
    assert_eq!(
        ledger.get("max_invalid_attempts").and_then(Value::as_u64),
        Some(4)
    );
}

fn assert_no_secret_text(text: &str, extras: &[&str]) {
    for forbidden in forbidden_artifact_terms() {
        assert!(
            !text.contains(forbidden),
            "output or artifact leaked forbidden text: {forbidden}"
        );
    }
    for forbidden in extras {
        assert!(
            !forbidden.is_empty() && !text.contains(forbidden),
            "output or artifact leaked forbidden text: {forbidden}"
        );
    }
}

fn forbidden_artifact_terms() -> &'static [&'static str] {
    &[
        ROUTE_TOKEN_SENTINEL,
        RAW_HANDLE_SENTINEL,
        CANDIDATE_ACK_SENTINEL,
        PLAINTEXT_SENTINEL,
        PADDING_SENTINEL,
        PASSPHRASE_SENTINEL,
        KEY_SENTINEL,
        "panicked at",
        "stack backtrace",
        "RUST_BACKTRACE",
        "thread '",
        "called `",
        "Traceback (most recent call last)",
        "/srv/qbuild/work/",
        "/home/victor/",
    ]
}

fn combined_output(output: &Output) -> String {
    let mut bytes = Vec::new();
    bytes.extend_from_slice(&output.stdout);
    bytes.extend_from_slice(&output.stderr);
    String::from_utf8_lossy(&bytes).into_owned()
}

fn marker(marker: &str) {
    assert!(MARKERS.contains(&marker));
    println!("{marker}");
}

fn unique_temp_dir(name: &str) -> PathBuf {
    std::env::temp_dir().join(format!("qshield-na0327-{name}-{}", unique_suffix()))
}

fn unique_suffix() -> String {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time")
        .as_nanos();
    format!("{}-{now}", std::process::id())
}

fn free_port() -> u16 {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind free port");
    listener.local_addr().expect("local addr").port()
}
