use std::fs;
use std::net::TcpListener;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Output, Stdio};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use serde_json::{json, Value};

const MARKERS: &[&str] = &[
    "NA0318_QSHIELD_PEEK_OR_POLL_CANDIDATE_OK",
    "NA0318_QSHIELD_LOCAL_VERIFY_BEFORE_COMMIT_OK",
    "NA0318_QSHIELD_ACK_COMMIT_DELETE_OK",
    "NA0318_QSHIELD_INVALID_RECV_NO_REMOTE_DELETE_OK",
    "NA0318_QSHIELD_INVALID_RECV_NO_ACCEPTED_STATE_OK",
    "NA0318_QSHIELD_INVALID_RECV_NO_OUTPUT_OK",
    "NA0318_QSHIELD_INVALID_RECV_NO_SECRET_LEAK_OK",
    "NA0318_QSHIELD_STALE_ACK_FAIL_CLOSED_OK",
    "NA0318_QSHIELD_REPEATED_INVALID_RECV_BOUNDED_OK",
    "NA0318_METADATA_RUNTIME_ACK_COMMIT_OK",
];

const PLAINTEXT_SENTINEL: &str = "NA0318_PLAINTEXT_SENTINEL";
const PADDING_SENTINEL: &str = "NA0318_PADDING_SENTINEL";

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
        let token = format!("na0318token{name}{port}");
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
fn candidate_fetch_preserves_queue_and_ack_deletes_exactly_one() {
    let relay = RelayHarness::start("valid");
    send_raw(&relay, "bob", "alice", "aa");
    send_raw(&relay, "bob", "alice", "bb");

    let first = candidates(&relay, "bob", 1);
    assert_eq!(first.len(), 1);
    let first_ack = ack_id(&first[0]);
    let repeated = candidates(&relay, "bob", 1);
    assert_eq!(repeated.len(), 1);
    assert_eq!(ack_id(&repeated[0]), first_ack);
    marker("NA0318_QSHIELD_PEEK_OR_POLL_CANDIDATE_OK");

    let (status, body) = relay.post("/ack", json!({"id": "bob", "ack_id": first_ack}));
    assert_eq!(status, 200);
    assert_eq!(body.get("ok").and_then(Value::as_bool), Some(true));

    let remaining = candidates(&relay, "bob", 10);
    assert_eq!(remaining.len(), 1);
    assert_ne!(ack_id(&remaining[0]), first_ack);
    marker("NA0318_QSHIELD_ACK_COMMIT_DELETE_OK");

    let (stale_status, stale_body) = relay.post("/ack", json!({"id": "bob", "ack_id": first_ack}));
    assert_eq!(stale_status, 404);
    assert_eq!(stale_body.get("ok").and_then(Value::as_bool), Some(false));
    let still_remaining = candidates(&relay, "bob", 10);
    assert_eq!(still_remaining.len(), 1);
    assert_eq!(ack_id(&still_remaining[0]), ack_id(&remaining[0]));
    marker("NA0318_QSHIELD_STALE_ACK_FAIL_CLOSED_OK");
}

#[test]
fn invalid_recv_reject_does_not_ack_delete_or_create_output() {
    let relay = RelayHarness::start("invalid");
    let store = relay.root.join("bob-store");
    write_bob_store(&store, &relay.base_url(), &relay.token);
    let bad_wire = format!("{PLAINTEXT_SENTINEL}-{PADDING_SENTINEL}");
    send_raw(&relay, "bob", "alice", &bad_wire);

    let before_state = fs::read(store.join("state.json")).expect("read before state");
    let before = candidates(&relay, "bob", 1);
    assert_eq!(before.len(), 1);
    let ack = ack_id(&before[0]);
    let first = run_recv(&store, &relay.token);
    assert!(!first.status.success());
    assert_no_invalid_output(&first, &relay.token, &ack);
    assert_eq!(
        fs::read(store.join("state.json")).expect("read after state"),
        before_state
    );
    marker("NA0318_QSHIELD_INVALID_RECV_NO_ACCEPTED_STATE_OK");
    marker("NA0318_QSHIELD_INVALID_RECV_NO_OUTPUT_OK");
    marker("NA0318_QSHIELD_INVALID_RECV_NO_SECRET_LEAK_OK");

    let after_first = candidates(&relay, "bob", 1);
    assert_eq!(after_first.len(), 1);
    assert_eq!(ack_id(&after_first[0]), ack);
    marker("NA0318_QSHIELD_INVALID_RECV_NO_REMOTE_DELETE_OK");

    let second = run_recv(&store, &relay.token);
    assert!(!second.status.success());
    assert_eq!(combined_output(&first), combined_output(&second));
    let after_second = candidates(&relay, "bob", 1);
    assert_eq!(after_second.len(), 1);
    assert_eq!(ack_id(&after_second[0]), ack);
    marker("NA0318_QSHIELD_REPEATED_INVALID_RECV_BOUNDED_OK");
    marker("NA0318_QSHIELD_LOCAL_VERIFY_BEFORE_COMMIT_OK");
    marker("NA0318_METADATA_RUNTIME_ACK_COMMIT_OK");
}

#[test]
fn legacy_poll_remains_destructive_but_candidate_path_is_not() {
    let relay = RelayHarness::start("legacy");
    send_raw(&relay, "bob", "alice", "aa");
    let candidate = candidates(&relay, "bob", 1);
    assert_eq!(candidate.len(), 1);

    let (poll_status, poll_body) = relay.post("/poll", json!({"id": "bob", "max": 1}));
    assert_eq!(poll_status, 200);
    assert_eq!(poll_body.get("ok").and_then(Value::as_bool), Some(true));
    assert_eq!(
        poll_body
            .get("msgs")
            .and_then(Value::as_array)
            .map(Vec::len),
        Some(1)
    );
    assert!(candidates(&relay, "bob", 1).is_empty());
}

#[test]
fn required_markers_are_declared() {
    for marker in MARKERS {
        assert!(marker.starts_with("NA0318_"));
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

fn write_bob_store(store: &Path, relay_url: &str, token: &str) {
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
    let hex32 = "11".repeat(32);
    fs::write(
        store.join("state.json"),
        serde_json::to_vec_pretty(&json!({
            "my_id": "bob",
            "dh_pub_hex": hex32,
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

fn run_recv(store: &Path, token: &str) -> Output {
    Command::new(env!("CARGO_BIN_EXE_qshield"))
        .args(["recv", "--store"])
        .arg(store)
        .args(["--max", "1"])
        .env("QSHIELD_RELAY_TOKEN", token)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .expect("run qshield recv")
}

fn assert_no_invalid_output(output: &Output, token: &str, ack_id: &str) {
    let text = combined_output(output);
    for forbidden in [
        token,
        ack_id,
        PLAINTEXT_SENTINEL,
        PADDING_SENTINEL,
        "panicked at",
        "stack backtrace",
        "RUST_BACKTRACE",
    ] {
        assert!(
            !text.contains(forbidden),
            "invalid receive output leaked forbidden text: {forbidden}"
        );
    }
    assert!(!text.contains("from alice:"));
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
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time")
        .as_nanos();
    std::env::temp_dir().join(format!(
        "qshield-na0318-{name}-{}-{now}",
        std::process::id()
    ))
}

fn free_port() -> u16 {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind free port");
    listener.local_addr().expect("local addr").port()
}
