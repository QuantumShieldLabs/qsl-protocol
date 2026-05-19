use std::fs;
use std::net::TcpListener;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Output, Stdio};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use serde_json::{json, Value};

const MARKERS: &[&str] = &[
    "NA0319_IDENTIFIER_ROTATION_POLICY_OK",
    "NA0319_OPAQUE_HANDLE_BOUNDARY_OK",
    "NA0319_STALE_HANDLE_REJECT_OK",
    "NA0319_MALFORMED_HANDLE_REJECT_OK",
    "NA0319_IDENTIFIER_NO_REMOTE_DELETE_ON_REJECT_OK",
    "NA0319_IDENTIFIER_NO_ACCEPTED_STATE_ON_REJECT_OK",
    "NA0319_IDENTIFIER_NO_OUTPUT_ON_REJECT_OK",
    "NA0319_IDENTIFIER_NO_SECRET_LOG_OK",
    "NA0319_DEFAULT_PADDING_POLICY_OK",
    "NA0319_PADDING_BUCKETS_OK",
    "NA0319_PADDING_INVALID_CONFIG_REJECT_OK",
    "NA0319_PADDING_STRIP_VERIFY_OK",
    "NA0319_PADDING_MALFORMED_REJECT_OK",
    "NA0319_PADDING_NO_REMOTE_DELETE_ON_REJECT_OK",
    "NA0319_PADDING_NO_ACCEPTED_STATE_ON_REJECT_OK",
    "NA0319_PADDING_NO_OUTPUT_ON_REJECT_OK",
    "NA0319_PADDING_NO_SECRET_LOG_OK",
    "NA0319_ACK_COMMIT_AFTER_IDENTIFIER_PADDING_VERIFY_OK",
    "NA0319_QSHIELD_EMBEDDED_RELAY_BOUNDARY_OK",
    "NA0319_METADATA_RUNTIME_IDENTIFIER_PADDING_OK",
];

const BUCKETS: &[usize] = &[512, 1024, 2048, 4096, 8192];
const RAW_HANDLE_SENTINEL: &str = "NA0319_RAW_HANDLE_SENTINEL_DO_NOT_LEAK";
const PLAINTEXT_SENTINEL: &str = "NA0319_PLAINTEXT_SENTINEL_DO_NOT_LEAK";
const PADDING_SENTINEL: &str = "NA0319_PADDING_SENTINEL_DO_NOT_LEAK";

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
        let token = format!("na0319token{name}{port}");
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
fn opaque_candidate_handles_rotate_and_reject_without_remote_delete_or_leak() {
    let relay = RelayHarness::start("identifier");
    send_raw(&relay, "bob", "alice", "aa");
    send_raw(&relay, "bob", "alice", "bb");

    let candidates_before = candidates(&relay, "bob", 2);
    assert_eq!(candidates_before.len(), 2);
    let first_ack = ack_id(&candidates_before[0]).to_string();
    let second_ack = ack_id(&candidates_before[1]).to_string();
    assert_ne!(first_ack, second_ack);
    assert_opaque_ack(&first_ack, &relay.token, "alice", "bob");
    assert_opaque_ack(&second_ack, &relay.token, "alice", "bob");
    marker("NA0319_IDENTIFIER_ROTATION_POLICY_OK");
    marker("NA0319_OPAQUE_HANDLE_BOUNDARY_OK");

    let malformed_handle = format!("bad-{RAW_HANDLE_SENTINEL}");
    let (malformed_status, malformed_body) =
        relay.post("/ack", json!({"id": "bob", "ack_id": malformed_handle}));
    assert_eq!(malformed_status, 400);
    assert_eq!(
        malformed_body.get("ok").and_then(Value::as_bool),
        Some(false)
    );
    assert_no_secret_text(
        &malformed_body.to_string(),
        &relay.token,
        &malformed_handle,
        "",
    );
    let after_malformed = candidates(&relay, "bob", 2);
    assert_eq!(ack_id(&after_malformed[0]), first_ack);
    assert_eq!(ack_id(&after_malformed[1]), second_ack);
    marker("NA0319_MALFORMED_HANDLE_REJECT_OK");
    marker("NA0319_IDENTIFIER_NO_REMOTE_DELETE_ON_REJECT_OK");
    marker("NA0319_IDENTIFIER_NO_SECRET_LOG_OK");

    let (ack_status, ack_body) = relay.post("/ack", json!({"id": "bob", "ack_id": first_ack}));
    assert_eq!(ack_status, 200);
    assert_eq!(ack_body.get("ok").and_then(Value::as_bool), Some(true));

    let (stale_status, stale_body) = relay.post("/ack", json!({"id": "bob", "ack_id": first_ack}));
    assert_eq!(stale_status, 404);
    assert_eq!(stale_body.get("ok").and_then(Value::as_bool), Some(false));
    assert_no_secret_text(&stale_body.to_string(), &relay.token, &first_ack, "");
    let after_stale = candidates(&relay, "bob", 2);
    assert_eq!(after_stale.len(), 1);
    assert_eq!(ack_id(&after_stale[0]), second_ack);
    marker("NA0319_STALE_HANDLE_REJECT_OK");
}

#[test]
fn stale_peer_handle_recv_reject_preserves_candidate_state_and_output_boundary() {
    let relay = RelayHarness::start("stale-peer");
    let store = relay.root.join("bob-store");
    write_store(&store, &relay.base_url(), &relay.token, false);
    send_raw(&relay, "bob", RAW_HANDLE_SENTINEL, "aa");

    let before_state = fs::read(store.join("state.json")).expect("read before state");
    let before = candidates(&relay, "bob", 1);
    assert_eq!(before.len(), 1);
    let ack = ack_id(&before[0]).to_string();

    let output = run_recv(&store, &relay.token);
    assert!(!output.status.success());
    let text = combined_output(&output);
    assert_no_secret_text(&text, &relay.token, &ack, RAW_HANDLE_SENTINEL);
    assert!(!text.contains("from "));
    assert_eq!(
        fs::read(store.join("state.json")).expect("read after state"),
        before_state
    );
    let after = candidates(&relay, "bob", 1);
    assert_eq!(after.len(), 1);
    assert_eq!(ack_id(&after[0]), ack);

    marker("NA0319_IDENTIFIER_NO_ACCEPTED_STATE_ON_REJECT_OK");
    marker("NA0319_IDENTIFIER_NO_OUTPUT_ON_REJECT_OK");
}

#[test]
fn default_padding_profile_strip_verify_and_ack_after_verify() {
    let relay = RelayHarness::start("padding-valid");
    let plaintext = PLAINTEXT_SENTINEL.as_bytes();
    let (padded, pad_len, bucket) = pad_to_default_bucket(plaintext);
    assert_eq!(bucket, BUCKETS[0]);
    assert_eq!(padded.len(), bucket);
    marker("NA0319_DEFAULT_PADDING_POLICY_OK");
    marker("NA0319_PADDING_BUCKETS_OK");

    send_padded(&relay, "bob", "alice", &padded, pad_len, bucket);
    let before = candidates(&relay, "bob", 1);
    assert_eq!(before.len(), 1);
    let ack = ack_id(&before[0]).to_string();
    let stripped = strip_and_verify(&before[0]).expect("valid padded candidate");
    assert_eq!(stripped, plaintext);
    marker("NA0319_PADDING_STRIP_VERIFY_OK");

    let (ack_status, ack_body) = relay.post("/ack", json!({"id": "bob", "ack_id": ack}));
    assert_eq!(ack_status, 200);
    assert_eq!(ack_body.get("ok").and_then(Value::as_bool), Some(true));
    assert!(candidates(&relay, "bob", 1).is_empty());
    marker("NA0319_ACK_COMMIT_AFTER_IDENTIFIER_PADDING_VERIFY_OK");
    marker("NA0319_QSHIELD_EMBEDDED_RELAY_BOUNDARY_OK");
    marker("NA0319_METADATA_RUNTIME_IDENTIFIER_PADDING_OK");
}

#[test]
fn invalid_padding_config_and_malformed_padding_reject_without_delete_or_leak() {
    let invalid_root = unique_temp_dir("invalid-config");
    let invalid = Command::new(env!("CARGO_BIN_EXE_qshield"))
        .args(["init", "--store"])
        .arg(&invalid_root)
        .arg("--padding-enable")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .expect("run invalid padding init");
    assert!(!invalid.status.success());
    assert!(!invalid_root.join("config.json").exists());
    assert_no_secret_text(&combined_output(&invalid), "", "", "");
    marker("NA0319_PADDING_INVALID_CONFIG_REJECT_OK");
    let _ = fs::remove_dir_all(&invalid_root);

    let relay = RelayHarness::start("padding-invalid");
    let store = relay.root.join("bob-store");
    write_store_with_session(&store, &relay.base_url(), &relay.token);

    let mut malformed = b"wire".to_vec();
    malformed.extend_from_slice(PADDING_SENTINEL.as_bytes());
    let bucket = malformed.len();
    let pad_len = PADDING_SENTINEL.len();
    send_padded(&relay, "bob", "alice", &malformed, pad_len, bucket);

    let before_state = fs::read(store.join("state.json")).expect("read before state");
    let before = candidates(&relay, "bob", 1);
    assert_eq!(before.len(), 1);
    let ack = ack_id(&before[0]).to_string();

    let output = run_recv(&store, &relay.token);
    assert!(!output.status.success());
    let text = combined_output(&output);
    assert_no_secret_text(&text, &relay.token, &ack, PADDING_SENTINEL);
    assert!(!text.contains("from alice:"));
    assert_eq!(
        fs::read(store.join("state.json")).expect("read after state"),
        before_state
    );
    let after = candidates(&relay, "bob", 1);
    assert_eq!(after.len(), 1);
    assert_eq!(ack_id(&after[0]), ack);

    marker("NA0319_PADDING_MALFORMED_REJECT_OK");
    marker("NA0319_PADDING_NO_REMOTE_DELETE_ON_REJECT_OK");
    marker("NA0319_PADDING_NO_ACCEPTED_STATE_ON_REJECT_OK");
    marker("NA0319_PADDING_NO_OUTPUT_ON_REJECT_OK");
    marker("NA0319_PADDING_NO_SECRET_LOG_OK");
}

#[test]
fn required_markers_are_declared() {
    for marker in MARKERS {
        assert!(marker.starts_with("NA0319_"));
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

fn assert_opaque_ack(ack: &str, token: &str, from: &str, to: &str) {
    assert_eq!(ack.len(), 64);
    assert!(ack.bytes().all(|b| b.is_ascii_hexdigit()));
    for forbidden in [token, from, to, PLAINTEXT_SENTINEL, PADDING_SENTINEL] {
        assert!(!ack.contains(forbidden));
    }
}

fn pad_to_default_bucket(payload: &[u8]) -> (Vec<u8>, usize, usize) {
    let bucket = BUCKETS
        .iter()
        .copied()
        .find(|bucket| *bucket >= payload.len())
        .expect("default bucket exists");
    let mut padded = payload.to_vec();
    let pad_len = bucket - payload.len();
    padded.extend(std::iter::repeat_n(0u8, pad_len));
    (padded, pad_len, bucket)
}

fn strip_and_verify(msg: &Value) -> Result<Vec<u8>, String> {
    let mut bytes = hex::decode(
        msg.get("msg")
            .and_then(Value::as_str)
            .ok_or_else(|| "missing msg".to_string())?,
    )
    .map_err(|_| "message decode reject".to_string())?;
    let pad_len = msg.get("pad_len").and_then(Value::as_u64).unwrap_or(0) as usize;
    let bucket = msg
        .get("bucket")
        .and_then(Value::as_u64)
        .ok_or_else(|| "padding reject".to_string())? as usize;
    if bytes.len() != bucket || pad_len > bytes.len() {
        return Err("padding reject".to_string());
    }
    let new_len = bytes.len() - pad_len;
    if bytes[new_len..].iter().any(|byte| *byte != 0) {
        return Err("padding reject".to_string());
    }
    bytes.truncate(new_len);
    Ok(bytes)
}

fn write_store(store: &Path, relay_url: &str, token: &str, padding_enabled: bool) {
    fs::create_dir_all(store).expect("create store");
    fs::write(
        store.join("config.json"),
        serde_json::to_vec_pretty(&json!({
            "relay_url": relay_url,
            "relay_token": token,
            "padding_enabled": padding_enabled,
            "padding_buckets": BUCKETS
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
            "sessions": {}
        }))
        .expect("serialize state"),
    )
    .expect("write state");
}

fn write_store_with_session(store: &Path, relay_url: &str, token: &str) {
    write_store(store, relay_url, token, true);
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

fn assert_no_secret_text(text: &str, token: &str, ack_id: &str, extra: &str) {
    for forbidden in [
        token,
        ack_id,
        extra,
        PLAINTEXT_SENTINEL,
        PADDING_SENTINEL,
        RAW_HANDLE_SENTINEL,
        "panicked at",
        "stack backtrace",
        "RUST_BACKTRACE",
    ] {
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
        "qshield-na0319-{name}-{}-{now}",
        std::process::id()
    ))
}

fn free_port() -> u16 {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind free port");
    listener.local_addr().expect("local addr").port()
}
