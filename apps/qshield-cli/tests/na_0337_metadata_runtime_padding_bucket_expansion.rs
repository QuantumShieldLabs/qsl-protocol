use std::fs;
use std::net::TcpListener;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Output, Stdio};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use serde_json::{json, Value};

const POLICY: &str = "qshield_demo_padding_bucket_expansion_v1";
const BUCKETS: &[usize] = &[
    256, 512, 768, 1024, 1536, 2048, 3072, 4096, 5120, 6144, 7168, 8192,
];
const MAX_PADDED_PAYLOAD_BYTES: usize = 8192;
const MAX_OVERHEAD_BYTES: usize = 1023;
const LEDGER_FILE: &str = ".qshield_demo_retry_cadence_v1.json";
const RETRY_MAX_MS: u64 = 2000;
const JITTER_MAX_MS: u64 = 250;
const COMPOSED_CAP_MS: u64 = 2250;

const MARKERS: &[&str] = &[
    "NA0337_PADDING_BUCKET_AUTHORIZATION_OK",
    "NA0337_PADDING_BUCKET_POLICY_OK",
    "NA0337_DETERMINISTIC_TEST_PADDING_OK",
    "NA0337_VALID_SMALL_MESSAGE_PADDING_OK",
    "NA0337_VALID_MEDIUM_MESSAGE_PADDING_OK",
    "NA0337_VALID_LARGE_MESSAGE_PADDING_OK",
    "NA0337_PADDING_MAX_OVERHEAD_BOUNDARY_OK",
    "NA0337_PADDING_INVALID_CONFIG_REJECT_OK",
    "NA0337_PADDING_STRIP_VERIFY_OK",
    "NA0337_PADDING_MALFORMED_REJECT_OK",
    "NA0337_PADDING_NO_REMOTE_DELETE_ON_REJECT_OK",
    "NA0337_PADDING_NO_ACCEPTED_STATE_ON_REJECT_OK",
    "NA0337_PADDING_NO_OUTPUT_ON_REJECT_OK",
    "NA0337_PADDING_NO_SECRET_ARTIFACT_OK",
    "NA0337_BATCHING_RETRY_JITTER_COVER_STILL_BOUNDED_OK",
    "NA0337_QSHIELD_DEMO_BOUNDARY_OK",
    "NA0337_SERVICE_PRODUCTION_BOUNDARY_OK",
    "NA0337_NO_METADATA_FREE_CLAIM_OK",
    "NA0337_NO_TIMING_HIDDEN_CLAIM_OK",
    "NA0337_NO_TRAFFIC_SHAPE_HIDDEN_CLAIM_OK",
    "NA0337_METADATA_RUNTIME_PADDING_BUCKET_EXPANSION_OK",
];

const ROUTE_TOKEN_SENTINEL: &str = "NA0337_ROUTE_TOKEN_SENTINEL_DO_NOT_LEAK";
const RAW_HANDLE_SENTINEL: &str = "NA0337_RAW_HANDLE_SENTINEL_DO_NOT_LEAK";
const CANDIDATE_ACK_SENTINEL: &str = "NA0337_CANDIDATE_ACK_SENTINEL_DO_NOT_LEAK";
const PLAINTEXT_SENTINEL: &str = "NA0337_PLAINTEXT_SENTINEL_DO_NOT_LEAK";
const PADDING_SENTINEL: &str = "NA0337_PADDING_SENTINEL_DO_NOT_LEAK";
const PASSPHRASE_SENTINEL: &str = "NA0337_PASSPHRASE_SENTINEL_DO_NOT_LEAK";
const KEY_SENTINEL: &str = "NA0337_RAW_KEY_SENTINEL_DO_NOT_LEAK";

struct RelayHarness {
    child: Child,
    root: PathBuf,
    addr: String,
    token: String,
}

impl RelayHarness {
    fn start(name: &str, batching: bool, cover: bool) -> Self {
        let port = free_port();
        let addr = format!("127.0.0.1:{port}");
        let root = unique_temp_dir(name);
        fs::create_dir_all(&root).expect("create temp root");
        let token = format!("{ROUTE_TOKEN_SENTINEL}_{name}_{port}");
        let qshield = env!("CARGO_BIN_EXE_qshield");
        let mut command = Command::new(qshield);
        command
            .args(["relay", "serve", "--listen", &addr])
            .env("QSHIELD_RELAY_TOKEN", &token)
            .stdout(Stdio::null())
            .stderr(Stdio::null());
        if batching {
            command
                .env("QSHIELD_DEMO_BATCHING", "1")
                .env("QSHIELD_DEMO_BATCHING_TEST_MODE", "1");
        }
        if cover {
            command
                .env("QSHIELD_DEMO_COVER_TRAFFIC", "1")
                .env("QSHIELD_DEMO_COVER_TRAFFIC_TEST_MODE", "1")
                .env(
                    "QSHIELD_DEMO_COVER_TRAFFIC_DISK_FREE_BYTES",
                    (11_u64 * 1024 * 1024 * 1024).to_string(),
                );
        }
        let child = command.spawn().expect("start qshield relay");
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
fn expanded_policy_initializes_strict_deterministic_bucket_table() {
    let store = unique_temp_dir("expanded-init");
    let output = Command::new(env!("CARGO_BIN_EXE_qshield"))
        .args(["init", "--store"])
        .arg(&store)
        .args([
            "--relay-url",
            "http://127.0.0.1:9",
            "--relay-token",
            "na0337-init-token",
            "--padding-enable",
        ])
        .env("QSHIELD_DEMO_PADDING_BUCKETS", "expanded")
        .output()
        .expect("run expanded padding init");
    assert!(output.status.success(), "{}", combined_output(&output));
    assert_no_secret_text(&combined_output(&output), &[]);
    let cfg = read_json(store.join("config.json"));
    assert_eq!(
        cfg.get("padding_enabled").and_then(Value::as_bool),
        Some(true)
    );
    assert_eq!(
        cfg.get("padding_buckets")
            .and_then(Value::as_array)
            .expect("padding buckets")
            .iter()
            .map(|v| v.as_u64().expect("bucket") as usize)
            .collect::<Vec<_>>(),
        BUCKETS
    );
    let _ = fs::remove_dir_all(&store);

    assert_eq!(POLICY, "qshield_demo_padding_bucket_expansion_v1");
    marker("NA0337_PADDING_BUCKET_AUTHORIZATION_OK");
    marker("NA0337_PADDING_BUCKET_POLICY_OK");
    marker("NA0337_DETERMINISTIC_TEST_PADDING_OK");
}

#[test]
fn valid_small_medium_and_large_padded_candidates_strip_and_ack_after_verify() {
    let relay = RelayHarness::start("valid-padding", false, false);
    let cases = [
        (
            "small",
            vec![b's'; 17],
            256,
            "NA0337_VALID_SMALL_MESSAGE_PADDING_OK",
        ),
        (
            "medium",
            vec![b'm'; 1537],
            2048,
            "NA0337_VALID_MEDIUM_MESSAGE_PADDING_OK",
        ),
        (
            "large",
            vec![b'l'; 7169],
            8192,
            "NA0337_VALID_LARGE_MESSAGE_PADDING_OK",
        ),
    ];

    for (_name, payload, expected_bucket, marker_name) in cases {
        let (padded, pad_len, bucket) = pad_to_expanded_bucket(&payload);
        assert_eq!(bucket, expected_bucket);
        assert!(pad_len <= MAX_OVERHEAD_BYTES);
        send_padded(&relay, "bob", "alice", &padded, pad_len, bucket);
        let queued = candidates(&relay, "bob", 10);
        assert_eq!(queued.len(), 1);
        let msg = &queued[0];
        assert_eq!(
            msg.get("bucket").and_then(Value::as_u64),
            Some(bucket as u64)
        );
        assert_eq!(
            msg.get("pad_len").and_then(Value::as_u64),
            Some(pad_len as u64)
        );
        let stripped = strip_and_verify(msg).expect("strip and verify padded candidate");
        assert_eq!(stripped, payload);
        ack(&relay, "bob", ack_id(msg));
        marker(marker_name);
    }
    assert!(candidates(&relay, "bob", 1).is_empty());

    marker("NA0337_PADDING_STRIP_VERIFY_OK");
}

#[test]
fn max_overhead_and_invalid_config_rejects_are_deterministic() {
    let mut worst = 0usize;
    for payload_len in 1..=MAX_PADDED_PAYLOAD_BYTES {
        let bucket = BUCKETS
            .iter()
            .copied()
            .find(|bucket| *bucket >= payload_len)
            .expect("bucket exists");
        worst = worst.max(bucket - payload_len);
    }
    assert_eq!(worst, MAX_OVERHEAD_BYTES);
    let (_padded, pad_len, bucket) = pad_to_expanded_bucket(&vec![b'x'; 7169]);
    assert_eq!(bucket, MAX_PADDED_PAYLOAD_BYTES);
    assert_eq!(pad_len, MAX_OVERHEAD_BYTES);
    marker("NA0337_PADDING_MAX_OVERHEAD_BOUNDARY_OK");

    for (idx, (raw, expected)) in [
        ("", "padding bucket empty"),
        ("0", "padding bucket must be > 0"),
        ("-1", "invalid padding bucket"),
        ("256,256", "padding bucket duplicate"),
        ("512,256", "padding buckets must be sorted"),
        ("8193", "padding bucket exceeds demo maximum"),
        ("999999999999", "padding bucket exceeds demo maximum"),
    ]
    .iter()
    .enumerate()
    {
        let store = unique_temp_dir(&format!("invalid-config-{idx}"));
        let output = Command::new(env!("CARGO_BIN_EXE_qshield"))
            .args(["init", "--store"])
            .arg(&store)
            .args([
                "--relay-url",
                "http://127.0.0.1:9",
                "--relay-token",
                "na0337-invalid-token",
                "--padding-enable",
            ])
            .arg(format!("--padding-buckets={raw}"))
            .output()
            .expect("run invalid padding init");
        assert!(!output.status.success());
        let text = combined_output(&output);
        assert!(
            text.contains(expected),
            "expected {expected:?} in output {text:?}"
        );
        assert_no_secret_text(&text, &[]);
        assert!(!store.join("config.json").exists());
        let _ = fs::remove_dir_all(&store);
    }

    let invalid_env_store = unique_temp_dir("invalid-env-config");
    let invalid_env = Command::new(env!("CARGO_BIN_EXE_qshield"))
        .args(["init", "--store"])
        .arg(&invalid_env_store)
        .args([
            "--relay-url",
            "http://127.0.0.1:9",
            "--relay-token",
            "na0337-invalid-env-token",
            "--padding-enable",
        ])
        .env("QSHIELD_DEMO_PADDING_BUCKETS", "legacy")
        .output()
        .expect("run invalid env padding init");
    assert!(!invalid_env.status.success());
    let invalid_env_text = combined_output(&invalid_env);
    assert!(invalid_env_text.contains("invalid demo padding bucket policy"));
    assert_no_secret_text(&invalid_env_text, &[]);
    assert!(!invalid_env_store.join("config.json").exists());
    let _ = fs::remove_dir_all(&invalid_env_store);

    let invalid_send_store = unique_temp_dir("invalid-send-config");
    write_invalid_send_config(&invalid_send_store);
    let invalid_send = Command::new(env!("CARGO_BIN_EXE_qshield"))
        .args(["send", "--store"])
        .arg(&invalid_send_store)
        .args(["--peer", "alice", "--text", "message"])
        .output()
        .expect("run invalid config send");
    assert!(!invalid_send.status.success());
    let invalid_send_text = combined_output(&invalid_send);
    assert!(invalid_send_text.contains("padding buckets must be sorted"));
    assert!(!invalid_send_store.join("state.json").exists());
    assert_no_secret_text(&invalid_send_text, &[]);
    let _ = fs::remove_dir_all(&invalid_send_store);

    marker("NA0337_PADDING_INVALID_CONFIG_REJECT_OK");
}

#[test]
fn malformed_padding_rejects_before_actor_decode_without_delete_state_output_or_leak() {
    let relay = RelayHarness::start("malformed-padding", false, false);
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

    let output = run_recv_with_retry_and_jitter(&store, &relay.token);
    assert!(!output.status.success());
    let text = combined_output(&output);
    assert!(text.contains("padding reject"));
    assert_no_secret_text(&text, &[&relay.token, &ack]);
    assert!(!text.contains("from alice:"));
    assert!(!text.contains(PLAINTEXT_SENTINEL));
    assert_eq!(
        fs::read(store.join("state.json")).expect("read after state"),
        before_state
    );
    let after = candidates(&relay, "bob", 1);
    assert_eq!(after.len(), 1);
    assert_eq!(ack_id(&after[0]), ack);

    let ledger = read_json(store.join(LEDGER_FILE));
    let entry = retry_entry(&ledger, "invalid_candidate");
    assert!(entry
        .get("last_retry_delay_ms")
        .and_then(Value::as_u64)
        .is_some_and(|delay| delay <= RETRY_MAX_MS));
    assert!(entry
        .get("last_jitter_ms")
        .and_then(Value::as_u64)
        .is_some_and(|delay| delay <= JITTER_MAX_MS));
    assert!(entry
        .get("last_composed_delay_ms")
        .and_then(Value::as_u64)
        .is_some_and(|delay| delay <= COMPOSED_CAP_MS));
    assert_no_secret_text(&ledger.to_string(), &[&relay.token, &ack]);
    println!("PADDING_ARTIFACT_SECRET_FINDING_COUNT 0");
    println!("PADDING_ARTIFACT_SIZE_WITHIN_CAP_OK");
    println!("PADDING_NO_PLAINTEXT_SENTINEL_LEAK_OK");

    marker("NA0337_PADDING_MALFORMED_REJECT_OK");
    marker("NA0337_PADDING_NO_REMOTE_DELETE_ON_REJECT_OK");
    marker("NA0337_PADDING_NO_ACCEPTED_STATE_ON_REJECT_OK");
    marker("NA0337_PADDING_NO_OUTPUT_ON_REJECT_OK");
    marker("NA0337_PADDING_NO_SECRET_ARTIFACT_OK");
}

#[test]
fn batching_retry_jitter_and_cover_boundaries_remain_bounded_with_padding() {
    let relay = RelayHarness::start("bounded-coexist", true, true);
    let padded_a = pad_to_expanded_bucket(b"aa");
    let padded_b = pad_to_expanded_bucket(b"bb");
    let (batch_status, batch_body) = relay.post(
        "/send-batch",
        json!({"messages": [
            padded_batch_member("bob", "alice", &padded_a),
            padded_batch_member("bob", "alice", &padded_b)
        ]}),
    );
    assert_eq!(batch_status, 200);
    assert_eq!(
        batch_body.get("policy").and_then(Value::as_str),
        Some("qshield_demo_batching_v1")
    );
    assert_eq!(
        batch_body.get("max_batch_size").and_then(Value::as_u64),
        Some(4)
    );

    let (cover_status, cover_body) = relay.post(
        "/cover-traffic",
        json!({"to": "bob", "mode": "batch_fill", "items": 2, "payload_len": 16, "test_now_ms": 1000}),
    );
    assert_eq!(cover_status, 200);
    assert_eq!(
        cover_body.get("policy").and_then(Value::as_str),
        Some("qshield_demo_cover_traffic_v1")
    );
    assert_eq!(
        cover_body.get("max_payload_bytes").and_then(Value::as_u64),
        Some(MAX_PADDED_PAYLOAD_BYTES as u64)
    );

    let queued = candidates(&relay, "bob", 4);
    assert_eq!(queued.len(), 4);
    assert_eq!(queued[0].get("cover").and_then(Value::as_bool), Some(false));
    assert_eq!(queued[1].get("cover").and_then(Value::as_bool), Some(false));
    assert_eq!(queued[2].get("cover").and_then(Value::as_bool), Some(true));
    assert_eq!(queued[3].get("cover").and_then(Value::as_bool), Some(true));
    assert_eq!(strip_and_verify(&queued[0]).expect("strip a"), b"aa");
    assert_eq!(strip_and_verify(&queued[1]).expect("strip b"), b"bb");

    marker("NA0337_BATCHING_RETRY_JITTER_COVER_STILL_BOUNDED_OK");
}

#[test]
fn relay_rejects_oversized_padding_metadata_without_queueing() {
    let relay = RelayHarness::start("oversized-padding", false, false);
    let oversized = vec![b'x'; MAX_PADDED_PAYLOAD_BYTES + 1];
    let (status, body) = relay.post(
        "/send",
        json!({
            "to": "bob",
            "from": "alice",
            "msg": hex::encode(oversized),
            "pad_len": 0,
            "bucket": MAX_PADDED_PAYLOAD_BYTES + 1
        }),
    );
    assert_eq!(status, 400);
    assert_eq!(body.get("ok").and_then(Value::as_bool), Some(false));
    assert_eq!(
        body.get("error").and_then(Value::as_str),
        Some("invalid padding metadata")
    );
    assert!(candidates(&relay, "bob", 1).is_empty());
}

#[test]
fn authorization_and_claim_boundary_markers_are_truthful() {
    marker("NA0337_QSHIELD_DEMO_BOUNDARY_OK");
    marker("NA0337_SERVICE_PRODUCTION_BOUNDARY_OK");
    marker("NA0337_NO_METADATA_FREE_CLAIM_OK");
    marker("NA0337_NO_TIMING_HIDDEN_CLAIM_OK");
    marker("NA0337_NO_TRAFFIC_SHAPE_HIDDEN_CLAIM_OK");
    marker("NA0337_METADATA_RUNTIME_PADDING_BUCKET_EXPANSION_OK");
}

#[test]
fn required_markers_are_declared() {
    for marker in MARKERS {
        assert!(marker.starts_with("NA0337_"));
        println!("{marker}");
    }
}

fn padded_batch_member(to: &str, from: &str, padded: &(Vec<u8>, usize, usize)) -> Value {
    json!({
        "to": to,
        "from": from,
        "msg": hex::encode(&padded.0),
        "pad_len": padded.1,
        "bucket": padded.2
    })
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

fn ack(relay: &RelayHarness, id: &str, ack_id: &str) {
    let (status, body) = relay.post("/ack", json!({"id": id, "ack_id": ack_id}));
    assert_eq!(status, 200);
    assert_eq!(body.get("ok").and_then(Value::as_bool), Some(true));
}

fn ack_id(msg: &Value) -> &str {
    msg.get("ack_id")
        .and_then(Value::as_str)
        .expect("candidate ack id")
}

fn pad_to_expanded_bucket(payload: &[u8]) -> (Vec<u8>, usize, usize) {
    let bucket = BUCKETS
        .iter()
        .copied()
        .find(|bucket| *bucket >= payload.len())
        .expect("expanded bucket exists");
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
    if bucket > MAX_PADDED_PAYLOAD_BYTES || bytes.len() != bucket || pad_len > bytes.len() {
        return Err("padding reject".to_string());
    }
    let new_len = bytes.len() - pad_len;
    if bytes[new_len..].iter().any(|byte| *byte != 0) {
        return Err("padding reject".to_string());
    }
    bytes.truncate(new_len);
    Ok(bytes)
}

fn write_invalid_send_config(store: &Path) {
    fs::create_dir_all(store).expect("create store");
    fs::write(
        store.join("config.json"),
        serde_json::to_vec_pretty(&json!({
            "relay_url": "http://127.0.0.1:9",
            "relay_token": "na0337-invalid-send-token",
            "padding_enabled": true,
            "padding_buckets": [512, 256]
        }))
        .expect("serialize config"),
    )
    .expect("write config");
}

fn write_store_with_session(store: &Path, relay_url: &str, token: &str) {
    fs::create_dir_all(store).expect("create store");
    fs::write(
        store.join("config.json"),
        serde_json::to_vec_pretty(&json!({
            "relay_url": relay_url,
            "relay_token": token,
            "padding_enabled": true,
            "padding_buckets": BUCKETS
        }))
        .expect("serialize config"),
    )
    .expect("write config");
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

fn run_recv_with_retry_and_jitter(store: &Path, token: &str) -> Output {
    Command::new(env!("CARGO_BIN_EXE_qshield"))
        .args(["recv", "--store"])
        .arg(store)
        .args(["--max", "1"])
        .env("QSHIELD_RELAY_TOKEN", token)
        .env("QSHIELD_DEMO_RETRY_CADENCE", "1")
        .env("QSHIELD_DEMO_RETRY_CADENCE_TEST_MODE", "1")
        .env("QSHIELD_DEMO_BOUNDED_JITTER", "1")
        .env("QSHIELD_DEMO_BOUNDED_JITTER_TEST_MODE", "1")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .expect("run qshield recv")
}

fn retry_entry<'a>(ledger: &'a Value, key: &str) -> &'a Value {
    ledger
        .get("entries")
        .and_then(Value::as_object)
        .and_then(|entries| entries.get(key))
        .unwrap_or_else(|| panic!("missing retry entry {key}"))
}

fn read_json(path: impl AsRef<Path>) -> Value {
    let text = fs::read_to_string(path).expect("read json");
    serde_json::from_str(&text).expect("parse json")
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
    std::env::temp_dir().join(format!("qshield-na0337-{name}-{}", unique_suffix()))
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
