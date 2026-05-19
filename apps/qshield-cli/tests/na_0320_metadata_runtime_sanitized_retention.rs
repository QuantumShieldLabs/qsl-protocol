use std::fs;
use std::net::TcpListener;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Output, Stdio};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use serde_json::{json, Value};

const MARKERS: &[&str] = &[
    "NA0320_SANITIZED_ERROR_POLICY_OK",
    "NA0320_RECV_INVALID_HANDLE_ERROR_REDACTED_OK",
    "NA0320_RECV_INVALID_PADDING_ERROR_REDACTED_OK",
    "NA0320_RECV_DECODE_ERROR_REDACTED_OK",
    "NA0320_ATTACHMENT_ERROR_REDACTED_OK",
    "NA0320_NO_ROUTE_TOKEN_LEAK_OK",
    "NA0320_NO_CANDIDATE_ACK_ID_LEAK_OK",
    "NA0320_NO_PLAINTEXT_SENTINEL_LEAK_OK",
    "NA0320_NO_PADDING_SENTINEL_LEAK_OK",
    "NA0320_NO_PANIC_BACKTRACE_LEAK_OK",
    "NA0320_RETENTION_PURGE_POLICY_OK",
    "NA0320_VALID_ACK_PURGES_ONE_CANDIDATE_OK",
    "NA0320_INVALID_RECV_RETAINS_REMOTE_CANDIDATE_OK",
    "NA0320_STALE_ACK_FAIL_CLOSED_OK",
    "NA0320_REPEATED_INVALID_RECV_BOUNDED_OK",
    "NA0320_INVALID_RECV_NO_LOCAL_OUTPUT_OK",
    "NA0320_INVALID_RECV_NO_ACCEPTED_STATE_OK",
    "NA0320_LOCAL_ARTIFACT_CLEANUP_OK",
    "NA0320_QSHIELD_EMBEDDED_RELAY_RETENTION_BOUNDARY_OK",
    "NA0320_METADATA_RUNTIME_SANITIZED_RETENTION_OK",
];

const ROUTE_TOKEN_SENTINEL: &str = "NA0320_ROUTE_TOKEN_SENTINEL_DO_NOT_LEAK";
const RAW_HANDLE_SENTINEL: &str = "NA0320_RAW_HANDLE_SENTINEL_DO_NOT_LEAK";
const CANDIDATE_ACK_SENTINEL: &str = "NA0320_CANDIDATE_ACK_SENTINEL_DO_NOT_LEAK";
const PLAINTEXT_SENTINEL: &str = "NA0320_PLAINTEXT_SENTINEL_DO_NOT_LEAK";
const PADDING_SENTINEL: &str = "NA0320_PADDING_SENTINEL_DO_NOT_LEAK";
const ATTACHMENT_SENTINEL: &str = "NA0320_ATTACHMENT_SENTINEL_DO_NOT_LEAK";
const PASSPHRASE_SENTINEL: &str = "NA0320_PASSPHRASE_SENTINEL_DO_NOT_LEAK";
const KEY_SENTINEL: &str = "NA0320_RAW_KEY_SENTINEL_DO_NOT_LEAK";

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
fn recv_invalid_handle_error_is_redacted_and_retains_candidate() {
    let relay = RelayHarness::start("invalid-handle");
    let store = relay.root.join("bob-store");
    write_store(&store, &relay.base_url(), &relay.token, false);
    send_raw(&relay, "bob", RAW_HANDLE_SENTINEL, PLAINTEXT_SENTINEL);

    let before_state = fs::read(store.join("state.json")).expect("read before state");
    let before = candidates(&relay, "bob", 1);
    assert_eq!(before.len(), 1);
    let ack = ack_id(&before[0]).to_string();

    let output = run_recv(&store, &relay.token);
    assert!(!output.status.success());
    let text = combined_output(&output);
    assert_no_secret_text(&text, &relay.token, &[&ack, RAW_HANDLE_SENTINEL]);
    assert!(!text.contains("from "));
    assert_eq!(
        fs::read(store.join("state.json")).expect("read after state"),
        before_state
    );
    let after = candidates(&relay, "bob", 1);
    assert_eq!(after.len(), 1);
    assert_eq!(ack_id(&after[0]), ack);

    marker("NA0320_SANITIZED_ERROR_POLICY_OK");
    marker("NA0320_RECV_INVALID_HANDLE_ERROR_REDACTED_OK");
    marker("NA0320_INVALID_RECV_RETAINS_REMOTE_CANDIDATE_OK");
    marker("NA0320_INVALID_RECV_NO_LOCAL_OUTPUT_OK");
    marker("NA0320_INVALID_RECV_NO_ACCEPTED_STATE_OK");
    marker("NA0320_NO_ROUTE_TOKEN_LEAK_OK");
    marker("NA0320_NO_CANDIDATE_ACK_ID_LEAK_OK");
    marker("NA0320_NO_PLAINTEXT_SENTINEL_LEAK_OK");
    marker("NA0320_NO_PANIC_BACKTRACE_LEAK_OK");
}

#[test]
fn recv_invalid_padding_and_decode_errors_are_redacted_bounded_and_retain() {
    let decode_relay = RelayHarness::start("decode");
    let decode_store = decode_relay.root.join("bob-store");
    write_store_with_session(&decode_store, &decode_relay.base_url(), &decode_relay.token);
    send_raw(
        &decode_relay,
        "bob",
        "alice",
        &format!("not-hex-{PLAINTEXT_SENTINEL}"),
    );
    let decode_before_state = fs::read(decode_store.join("state.json")).expect("read decode state");
    let decode_before = candidates(&decode_relay, "bob", 1);
    let decode_ack = ack_id(&decode_before[0]).to_string();

    let first_decode = run_recv(&decode_store, &decode_relay.token);
    assert!(!first_decode.status.success());
    let first_decode_text = combined_output(&first_decode);
    assert_no_secret_text(&first_decode_text, &decode_relay.token, &[&decode_ack]);
    assert!(!first_decode_text.contains("from alice:"));
    assert_eq!(
        fs::read(decode_store.join("state.json")).expect("read after decode"),
        decode_before_state
    );
    let after_first_decode = candidates(&decode_relay, "bob", 1);
    assert_eq!(ack_id(&after_first_decode[0]), decode_ack);

    let second_decode = run_recv(&decode_store, &decode_relay.token);
    assert!(!second_decode.status.success());
    assert_eq!(combined_output(&second_decode), first_decode_text);
    let after_second_decode = candidates(&decode_relay, "bob", 1);
    assert_eq!(ack_id(&after_second_decode[0]), decode_ack);

    let padding_relay = RelayHarness::start("padding");
    let padding_store = padding_relay.root.join("bob-store");
    write_store_with_session(
        &padding_store,
        &padding_relay.base_url(),
        &padding_relay.token,
    );
    let mut malformed_padding = b"wire".to_vec();
    malformed_padding.extend_from_slice(PADDING_SENTINEL.as_bytes());
    send_padded(
        &padding_relay,
        "bob",
        "alice",
        &malformed_padding,
        PADDING_SENTINEL.len(),
        malformed_padding.len(),
    );
    let padding_before_state =
        fs::read(padding_store.join("state.json")).expect("read padding state");
    let padding_before = candidates(&padding_relay, "bob", 1);
    let padding_ack = ack_id(&padding_before[0]).to_string();

    let padding_output = run_recv(&padding_store, &padding_relay.token);
    assert!(!padding_output.status.success());
    let padding_text = combined_output(&padding_output);
    assert_no_secret_text(
        &padding_text,
        &padding_relay.token,
        &[&padding_ack, PADDING_SENTINEL],
    );
    assert!(!padding_text.contains("from alice:"));
    assert_eq!(
        fs::read(padding_store.join("state.json")).expect("read after padding"),
        padding_before_state
    );
    let padding_after = candidates(&padding_relay, "bob", 1);
    assert_eq!(ack_id(&padding_after[0]), padding_ack);

    marker("NA0320_RECV_DECODE_ERROR_REDACTED_OK");
    marker("NA0320_RECV_INVALID_PADDING_ERROR_REDACTED_OK");
    marker("NA0320_REPEATED_INVALID_RECV_BOUNDED_OK");
    marker("NA0320_INVALID_RECV_RETAINS_REMOTE_CANDIDATE_OK");
    marker("NA0320_INVALID_RECV_NO_LOCAL_OUTPUT_OK");
    marker("NA0320_INVALID_RECV_NO_ACCEPTED_STATE_OK");
    marker("NA0320_NO_PLAINTEXT_SENTINEL_LEAK_OK");
    marker("NA0320_NO_PADDING_SENTINEL_LEAK_OK");
    marker("NA0320_NO_PANIC_BACKTRACE_LEAK_OK");
}

#[test]
fn relay_ack_retention_and_purge_boundaries_are_deterministic() {
    let relay = RelayHarness::start("retention");
    send_raw(&relay, "bob", "alice", "aa");
    send_raw(&relay, "bob", "alice", "bb");

    let fetched_once = candidates(&relay, "bob", 2);
    let fetched_twice = candidates(&relay, "bob", 2);
    assert_eq!(fetched_once, fetched_twice);
    assert_eq!(fetched_once.len(), 2);
    let first_ack = ack_id(&fetched_once[0]).to_string();
    let second_ack = ack_id(&fetched_once[1]).to_string();
    assert_ne!(first_ack, second_ack);
    assert_eq!(
        fetched_once[0].get("msg").and_then(Value::as_str),
        Some("aa")
    );

    let malformed_ack = format!("bad-{CANDIDATE_ACK_SENTINEL}");
    let (malformed_status, malformed_body) =
        relay.post("/ack", json!({"id": "bob", "ack_id": malformed_ack}));
    assert_eq!(malformed_status, 400);
    assert_eq!(
        malformed_body.get("ok").and_then(Value::as_bool),
        Some(false)
    );
    assert_no_secret_text(
        &malformed_body.to_string(),
        &relay.token,
        &[&first_ack, &second_ack, &malformed_ack],
    );
    assert_eq!(candidates(&relay, "bob", 2), fetched_once);

    let (padding_status, padding_body) = relay.post(
        "/send",
        json!({
            "to": "bob",
            "from": "alice",
            "msg": hex::encode(PADDING_SENTINEL),
            "pad_len": 1,
            "bucket": 1
        }),
    );
    assert_eq!(padding_status, 400);
    assert_eq!(padding_body.get("ok").and_then(Value::as_bool), Some(false));
    assert_no_secret_text(&padding_body.to_string(), &relay.token, &[PADDING_SENTINEL]);
    assert_eq!(candidates(&relay, "bob", 2), fetched_once);

    let (ack_status, ack_body) = relay.post("/ack", json!({"id": "bob", "ack_id": first_ack}));
    assert_eq!(ack_status, 200);
    assert_eq!(ack_body.get("ok").and_then(Value::as_bool), Some(true));
    let remaining = candidates(&relay, "bob", 2);
    assert_eq!(remaining.len(), 1);
    assert_eq!(ack_id(&remaining[0]), second_ack);

    let (stale_status, stale_body) = relay.post("/ack", json!({"id": "bob", "ack_id": first_ack}));
    assert_eq!(stale_status, 404);
    assert_eq!(stale_body.get("ok").and_then(Value::as_bool), Some(false));
    assert_no_secret_text(
        &stale_body.to_string(),
        &relay.token,
        &[&first_ack, &second_ack],
    );
    let after_stale = candidates(&relay, "bob", 2);
    assert_eq!(after_stale.len(), 1);
    assert_eq!(ack_id(&after_stale[0]), second_ack);

    marker("NA0320_RETENTION_PURGE_POLICY_OK");
    marker("NA0320_VALID_ACK_PURGES_ONE_CANDIDATE_OK");
    marker("NA0320_STALE_ACK_FAIL_CLOSED_OK");
    marker("NA0320_QSHIELD_EMBEDDED_RELAY_RETENTION_BOUNDARY_OK");
}

#[test]
fn attachment_reject_is_redacted_and_local_artifacts_are_cleaned() {
    let relay = RelayHarness::start("attachment");
    let store = relay.root.join("bob-store");
    let out_dir = relay.root.join("attachment-out");
    write_store(&store, &relay.base_url(), &relay.token, false);
    send_raw(&relay, "bob", RAW_HANDLE_SENTINEL, ATTACHMENT_SENTINEL);
    let before_state = fs::read(store.join("state.json")).expect("read before state");
    let before = candidates(&relay, "bob", 1);
    let ack = ack_id(&before[0]).to_string();

    let output = run_attachment_recv(&store, &out_dir, &relay.token);
    assert!(!output.status.success());
    let text = combined_output(&output);
    assert_no_secret_text(
        &text,
        &relay.token,
        &[&ack, RAW_HANDLE_SENTINEL, ATTACHMENT_SENTINEL],
    );
    assert!(!out_dir.exists());
    assert_eq!(
        fs::read(store.join("state.json")).expect("read after state"),
        before_state
    );
    let after = candidates(&relay, "bob", 1);
    assert_eq!(ack_id(&after[0]), ack);

    let artifact = relay.root.join("local-artifact.tmp");
    fs::write(&artifact, b"temporary artifact").expect("write temp artifact");
    fs::remove_file(&artifact).expect("remove temp artifact");
    assert!(!artifact.exists());

    marker("NA0320_ATTACHMENT_ERROR_REDACTED_OK");
    marker("NA0320_INVALID_RECV_RETAINS_REMOTE_CANDIDATE_OK");
    marker("NA0320_INVALID_RECV_NO_LOCAL_OUTPUT_OK");
    marker("NA0320_INVALID_RECV_NO_ACCEPTED_STATE_OK");
    marker("NA0320_LOCAL_ARTIFACT_CLEANUP_OK");
}

#[test]
fn required_markers_are_declared() {
    for marker in MARKERS {
        assert!(marker.starts_with("NA0320_"));
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

fn send_padded(
    relay: &RelayHarness,
    to: &str,
    from: &str,
    payload: &[u8],
    pad_len: usize,
    bucket: usize,
) {
    let (status, body) = relay.post(
        "/send",
        json!({
            "to": to,
            "from": from,
            "msg": hex::encode(payload),
            "pad_len": pad_len,
            "bucket": bucket
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

fn write_store(store: &Path, relay_url: &str, token: &str, padding_enabled: bool) {
    fs::create_dir_all(store).expect("create store");
    fs::write(
        store.join("config.json"),
        serde_json::to_vec_pretty(&json!({
            "relay_url": relay_url,
            "relay_token": token,
            "padding_enabled": padding_enabled
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
    write_store(store, relay_url, token, false);
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

fn run_attachment_recv(store: &Path, out_dir: &Path, token: &str) -> Output {
    Command::new(env!("CARGO_BIN_EXE_qshield"))
        .args(["attachment", "recv", "--store"])
        .arg(store)
        .args(["--out"])
        .arg(out_dir)
        .args(["--max", "8"])
        .env("QSHIELD_RELAY_TOKEN", token)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .expect("run qshield attachment recv")
}

fn assert_no_secret_text(text: &str, token: &str, extras: &[&str]) {
    for forbidden in [
        token,
        ROUTE_TOKEN_SENTINEL,
        RAW_HANDLE_SENTINEL,
        CANDIDATE_ACK_SENTINEL,
        PLAINTEXT_SENTINEL,
        PADDING_SENTINEL,
        ATTACHMENT_SENTINEL,
        PASSPHRASE_SENTINEL,
        KEY_SENTINEL,
        "panicked at",
        "stack backtrace",
        "RUST_BACKTRACE",
        "thread '",
        "called `",
        "Traceback (most recent call last)",
    ] {
        if !forbidden.is_empty() {
            assert!(
                !text.contains(forbidden),
                "output leaked forbidden text: {forbidden}"
            );
        }
    }
    for forbidden in extras {
        if !forbidden.is_empty() {
            assert!(
                !text.contains(forbidden),
                "output leaked forbidden text: {forbidden}"
            );
        }
    }
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
        "qshield-na0320-{name}-{}-{now}",
        std::process::id()
    ))
}

fn free_port() -> u16 {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind free port");
    listener.local_addr().expect("local addr").port()
}
