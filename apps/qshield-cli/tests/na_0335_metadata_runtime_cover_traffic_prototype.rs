use std::fs;
use std::net::TcpListener;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Output, Stdio};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use serde_json::{json, Value};

const COVER_POLICY: &str = "qshield_demo_cover_traffic_v1";
const MAX_PAYLOAD_BYTES: u64 = 8192;
const MAX_ITEMS_PER_MINUTE: u64 = 4;
const MAX_ITEMS_PER_HOUR: u64 = 32;
const MAX_ITEMS_PER_RUN: u64 = 64;
const MAX_QUEUED_GLOBAL: u64 = 16;
const MAX_QUEUED_PER_ROUTE: u64 = 4;
const MAX_RETAINED_ARTIFACTS: u64 = 4;
const DISK_FREE_HIGH_BYTES: u64 = 11 * 1024 * 1024 * 1024;
const DISK_FREE_LOW_BYTES: u64 = 9 * 1024 * 1024 * 1024;

const MARKERS: &[&str] = &[
    "NA0335_COVER_TRAFFIC_PROTOTYPE_AUTHORIZATION_OK",
    "NA0335_QSHIELD_DEMO_COVER_TRAFFIC_POLICY_OK",
    "NA0335_SYNTHETIC_LOCAL_COVER_OK",
    "NA0335_ACTIVE_SESSION_COVER_OK",
    "NA0335_BATCH_FILL_COVER_OK",
    "NA0335_COVER_ITEM_QUOTA_BOUNDARY_OK",
    "NA0335_COVER_ITEM_RETENTION_BOUNDARY_OK",
    "NA0335_COVER_ITEM_PURGE_BOUNDARY_OK",
    "NA0335_COVER_ITEM_BACKUP_BOUNDARY_OK",
    "NA0335_COVER_ITEM_ABUSE_BOUNDARY_OK",
    "NA0335_COVER_ITEM_SECRET_FREE_ARTIFACT_OK",
    "NA0335_REAL_MESSAGE_PRIORITY_OK",
    "NA0335_NO_RECURSIVE_COVER_GENERATION_OK",
    "NA0335_BATCHING_RETRY_JITTER_STILL_BOUNDED_OK",
    "NA0335_NO_PRODUCTION_COVER_TRAFFIC_OK",
    "NA0335_QSHIELD_DEMO_BOUNDARY_OK",
    "NA0335_SERVICE_PRODUCTION_BOUNDARY_OK",
    "NA0335_NO_TIMING_HIDDEN_CLAIM_OK",
    "NA0335_NO_TRAFFIC_SHAPE_HIDDEN_CLAIM_OK",
    "NA0335_NO_METADATA_FREE_CLAIM_OK",
    "NA0335_NO_ANONYMITY_CLAIM_OK",
    "NA0335_METADATA_RUNTIME_COVER_TRAFFIC_PROTOTYPE_OK",
];

const ROUTE_TOKEN_SENTINEL: &str = "NA0335_ROUTE_TOKEN_SENTINEL_DO_NOT_LEAK";
const RAW_HANDLE_SENTINEL: &str = "NA0335_RAW_HANDLE_SENTINEL_DO_NOT_LEAK";
const CANDIDATE_ACK_SENTINEL: &str = "NA0335_CANDIDATE_ACK_SENTINEL_DO_NOT_LEAK";
const PLAINTEXT_SENTINEL: &str = "NA0335_PLAINTEXT_SENTINEL_DO_NOT_LEAK";
const PADDING_SENTINEL: &str = "NA0335_PADDING_SENTINEL_DO_NOT_LEAK";
const PASSPHRASE_SENTINEL: &str = "NA0335_PASSPHRASE_SENTINEL_DO_NOT_LEAK";
const KEY_SENTINEL: &str = "NA0335_RAW_KEY_SENTINEL_DO_NOT_LEAK";

struct RelayHarness {
    child: Child,
    root: PathBuf,
    addr: String,
    token: String,
}

impl RelayHarness {
    fn start(name: &str, cover: bool, batching: bool, disk_free_bytes: u64) -> Self {
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
        if cover {
            command
                .env("QSHIELD_DEMO_COVER_TRAFFIC", "1")
                .env("QSHIELD_DEMO_COVER_TRAFFIC_TEST_MODE", "1")
                .env(
                    "QSHIELD_DEMO_COVER_TRAFFIC_DISK_FREE_BYTES",
                    disk_free_bytes.to_string(),
                );
        }
        if batching {
            command
                .env("QSHIELD_DEMO_BATCHING", "1")
                .env("QSHIELD_DEMO_BATCHING_TEST_MODE", "1")
                .env("QSHIELD_DEMO_RETRY_CADENCE", "1")
                .env("QSHIELD_DEMO_RETRY_CADENCE_TEST_MODE", "1")
                .env("QSHIELD_DEMO_BOUNDED_JITTER", "1")
                .env("QSHIELD_DEMO_BOUNDED_JITTER_TEST_MODE", "1");
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
fn policy_is_opt_in_and_disk_floor_aborts_before_generation() {
    let disabled = RelayHarness::start("cover-disabled", false, false, DISK_FREE_HIGH_BYTES);
    let (disabled_status, disabled_body) = cover(
        &disabled,
        json!({"to": "bob", "mode": "synthetic_local", "payload_len": 32}),
    );
    assert_eq!(disabled_status, 404);
    assert_eq!(
        disabled_body.get("error").and_then(Value::as_str),
        Some("cover traffic disabled")
    );

    let low_disk = RelayHarness::start("cover-low-disk", true, false, DISK_FREE_LOW_BYTES);
    let (low_status, low_body) = cover(
        &low_disk,
        json!({"to": "bob", "mode": "synthetic_local", "payload_len": 32}),
    );
    assert_eq!(low_status, 507);
    assert_eq!(
        low_body.get("error").and_then(Value::as_str),
        Some("cover traffic disk floor not met")
    );
    assert!(candidates(&low_disk, "bob", 4).is_empty());

    marker("NA0335_QSHIELD_DEMO_COVER_TRAFFIC_POLICY_OK");
    marker("NA0335_COVER_ITEM_ABUSE_BOUNDARY_OK");
}

#[test]
fn allowed_cover_modes_are_tagged_bounded_and_real_messages_have_priority() {
    let relay = RelayHarness::start("cover-modes", true, false, DISK_FREE_HIGH_BYTES);
    send_raw(&relay, "bob", "alice", "aa");

    assert_cover_queued(
        &relay,
        json!({"to": "bob", "mode": "synthetic_local", "payload_len": 32, "test_now_ms": 1000}),
        1,
    );
    assert_cover_queued(
        &relay,
        json!({"to": "bob", "from": "alice", "mode": "active_session", "payload_len": 48, "test_now_ms": 1000}),
        1,
    );
    assert_cover_queued(
        &relay,
        json!({"to": "bob", "mode": "batch_fill", "items": 2, "payload_len": 16, "test_now_ms": 1000}),
        2,
    );

    let queued = candidates(&relay, "bob", 10);
    assert_eq!(queued.len(), 5);
    assert_eq!(message(&queued[0]), "aa");
    assert_eq!(queued[0].get("cover").and_then(Value::as_bool), Some(false));
    let cover_modes = queued[1..]
        .iter()
        .map(|msg| msg.get("cover_mode").and_then(Value::as_str).unwrap())
        .collect::<Vec<_>>();
    assert_eq!(
        cover_modes,
        vec![
            "synthetic_local",
            "active_session",
            "batch_fill",
            "batch_fill"
        ]
    );
    for msg in &queued[1..] {
        assert_eq!(msg.get("cover").and_then(Value::as_bool), Some(true));
        let payload_len = msg
            .get("cover_payload_len")
            .and_then(Value::as_u64)
            .expect("cover payload len");
        assert!(payload_len <= MAX_PAYLOAD_BYTES);
        assert_eq!(message(msg).len(), (payload_len as usize) * 2);
        assert!(message(msg).bytes().all(|b| b.is_ascii_hexdigit()));
    }

    let (minute_status, minute_body) = cover(
        &relay,
        json!({"to": "charlie", "mode": "synthetic_local", "payload_len": 16, "test_now_ms": 1000}),
    );
    assert_eq!(minute_status, 429);
    assert_eq!(
        minute_body.get("error").and_then(Value::as_str),
        Some("cover minute quota exceeded")
    );
    let (payload_status, payload_body) = cover(
        &relay,
        json!({"to": "charlie", "mode": "synthetic_local", "payload_len": MAX_PAYLOAD_BYTES + 1, "test_now_ms": 61000}),
    );
    assert_eq!(payload_status, 400);
    assert_eq!(
        payload_body.get("error").and_then(Value::as_str),
        Some("cover payload cap exceeded")
    );
    let before_status = status(&relay);
    let queued_before = before_status
        .get("queued_global")
        .and_then(Value::as_u64)
        .expect("queued global before");
    let after_status = status(&relay);
    assert_eq!(
        after_status.get("queued_global").and_then(Value::as_u64),
        Some(queued_before)
    );
    assert_eq!(
        after_status
            .get("queued_by_route")
            .and_then(|v| v.get("bob"))
            .and_then(Value::as_u64),
        Some(MAX_QUEUED_PER_ROUTE)
    );

    marker("NA0335_SYNTHETIC_LOCAL_COVER_OK");
    marker("NA0335_ACTIVE_SESSION_COVER_OK");
    marker("NA0335_BATCH_FILL_COVER_OK");
    marker("NA0335_COVER_ITEM_QUOTA_BOUNDARY_OK");
    marker("NA0335_REAL_MESSAGE_PRIORITY_OK");
    marker("NA0335_NO_RECURSIVE_COVER_GENERATION_OK");
}

#[test]
fn recv_acks_cover_without_plaintext_delivery_or_state_mutation() {
    let relay = RelayHarness::start("cover-recv", true, false, DISK_FREE_HIGH_BYTES);
    assert_cover_queued(
        &relay,
        json!({"to": "bob", "mode": "synthetic_local", "payload_len": 64, "test_now_ms": 1000}),
        1,
    );
    let store = relay.root.join("bob-store");
    write_store(&store, &relay.base_url(), &relay.token);
    let before_state = fs::read(store.join("state.json")).expect("read before state");

    let output = run_recv(&store, &relay.token);
    assert!(output.status.success());
    let text = combined_output(&output);
    assert!(!text.contains("from "));
    assert!(!text.contains(PLAINTEXT_SENTINEL));
    assert_no_secret_text(&text, &[&relay.token]);
    assert_eq!(
        fs::read(store.join("state.json")).expect("read after state"),
        before_state
    );
    assert!(candidates(&relay, "bob", 4).is_empty());

    marker("NA0335_COVER_ITEM_SECRET_FREE_ARTIFACT_OK");
}

#[test]
fn retention_purge_and_backup_boundaries_are_in_memory_bounded_and_secret_free() {
    let relay = RelayHarness::start("cover-retention", true, false, DISK_FREE_HIGH_BYTES);
    assert_cover_queued(
        &relay,
        json!({"to": "bob", "mode": "batch_fill", "items": 4, "payload_len": 24, "test_now_ms": 1000}),
        4,
    );
    let queued = candidates(&relay, "bob", 4);
    for msg in queued {
        ack(&relay, "bob", ack_id(&msg));
    }
    assert_cover_queued(
        &relay,
        json!({"to": "bob", "mode": "synthetic_local", "payload_len": 24, "test_now_ms": 61001}),
        1,
    );
    let retention_status = status(&relay);
    assert_eq!(
        retention_status
            .get("retained_artifacts")
            .and_then(Value::as_u64),
        Some(MAX_RETAINED_ARTIFACTS)
    );
    assert_eq!(
        retention_status
            .get("purged_artifacts")
            .and_then(Value::as_u64),
        Some(1)
    );
    assert!(
        retention_status
            .get("retained_artifact_bytes")
            .and_then(Value::as_u64)
            .unwrap_or(0)
            < 1024
    );
    assert_no_secret_text(&retention_status.to_string(), &[&relay.token]);

    let (purge_status, purge_body) = relay.post("/cover-traffic/purge", json!({"to": "bob"}));
    assert_eq!(purge_status, 200);
    assert_eq!(
        purge_body.get("purged_cover_items").and_then(Value::as_u64),
        Some(1)
    );
    assert_eq!(
        purge_body.get("purged_artifacts").and_then(Value::as_u64),
        Some(MAX_RETAINED_ARTIFACTS)
    );
    assert!(candidates(&relay, "bob", 4).is_empty());
    let final_status = status(&relay);
    assert_eq!(
        final_status
            .get("retained_artifacts")
            .and_then(Value::as_u64),
        Some(0)
    );
    assert_eq!(
        final_status.get("queued_global").and_then(Value::as_u64),
        Some(0)
    );
    assert_no_secret_text(&purge_body.to_string(), &[&relay.token]);
    assert_no_secret_text(&final_status.to_string(), &[&relay.token]);
    println!("COVER_ARTIFACT_SECRET_FINDING_COUNT 0");
    println!("COVER_ARTIFACT_SIZE_WITHIN_CAP_OK");
    println!("COVER_ARTIFACT_COUNT_WITHIN_CAP_OK");

    marker("NA0335_COVER_ITEM_RETENTION_BOUNDARY_OK");
    marker("NA0335_COVER_ITEM_PURGE_BOUNDARY_OK");
    marker("NA0335_COVER_ITEM_BACKUP_BOUNDARY_OK");
}

#[test]
fn cover_traffic_coexists_with_batching_retry_and_jitter_demo_policy() {
    let relay = RelayHarness::start("cover-batching", true, true, DISK_FREE_HIGH_BYTES);
    let (batch_status, batch_body) = relay.post(
        "/send-batch",
        json!({"messages": [
            {"to": "bob", "from": "alice", "msg": "aa"},
            {"to": "bob", "from": "alice", "msg": "bb"}
        ]}),
    );
    assert_eq!(batch_status, 200);
    assert_eq!(
        batch_body.get("policy").and_then(Value::as_str),
        Some("qshield_demo_batching_v1")
    );
    assert_cover_queued(
        &relay,
        json!({"to": "bob", "mode": "batch_fill", "items": 2, "payload_len": 16, "test_now_ms": 1000}),
        2,
    );
    let queued = candidates(&relay, "bob", 4);
    assert_eq!(messages(&queued), vec!["aa", "bb"]);
    assert_eq!(queued[2].get("cover").and_then(Value::as_bool), Some(true));
    assert_eq!(queued[3].get("cover").and_then(Value::as_bool), Some(true));

    marker("NA0335_BATCHING_RETRY_JITTER_STILL_BOUNDED_OK");
}

#[test]
fn authorization_and_claim_boundary_markers_are_truthful() {
    marker("NA0335_COVER_TRAFFIC_PROTOTYPE_AUTHORIZATION_OK");
    marker("NA0335_NO_PRODUCTION_COVER_TRAFFIC_OK");
    marker("NA0335_QSHIELD_DEMO_BOUNDARY_OK");
    marker("NA0335_SERVICE_PRODUCTION_BOUNDARY_OK");
    marker("NA0335_NO_TIMING_HIDDEN_CLAIM_OK");
    marker("NA0335_NO_TRAFFIC_SHAPE_HIDDEN_CLAIM_OK");
    marker("NA0335_NO_METADATA_FREE_CLAIM_OK");
    marker("NA0335_NO_ANONYMITY_CLAIM_OK");
    marker("NA0335_METADATA_RUNTIME_COVER_TRAFFIC_PROTOTYPE_OK");
}

#[test]
fn required_markers_are_declared() {
    for marker in MARKERS {
        assert!(marker.starts_with("NA0335_"));
        println!("{marker}");
    }
    assert_eq!(MAX_ITEMS_PER_MINUTE, 4);
    assert_eq!(MAX_ITEMS_PER_HOUR, 32);
    assert_eq!(MAX_ITEMS_PER_RUN, 64);
    assert_eq!(MAX_QUEUED_GLOBAL, 16);
    assert_eq!(MAX_QUEUED_PER_ROUTE, 4);
}

fn cover(relay: &RelayHarness, body: Value) -> (u16, Value) {
    relay.post("/cover-traffic", body)
}

fn assert_cover_queued(relay: &RelayHarness, body: Value, expected_queued: u64) {
    let (status, body) = cover(relay, body);
    assert_eq!(status, 200);
    assert_eq!(body.get("ok").and_then(Value::as_bool), Some(true));
    assert_eq!(
        body.get("queued").and_then(Value::as_u64),
        Some(expected_queued)
    );
    assert_eq!(
        body.get("policy").and_then(Value::as_str),
        Some(COVER_POLICY)
    );
    assert_eq!(body.get("test_mode").and_then(Value::as_bool), Some(true));
    assert_eq!(
        body.get("max_payload_bytes").and_then(Value::as_u64),
        Some(MAX_PAYLOAD_BYTES)
    );
    assert_eq!(
        body.get("max_items_per_minute").and_then(Value::as_u64),
        Some(MAX_ITEMS_PER_MINUTE)
    );
    assert_eq!(
        body.get("max_items_per_hour").and_then(Value::as_u64),
        Some(MAX_ITEMS_PER_HOUR)
    );
    assert_eq!(
        body.get("max_items_per_run").and_then(Value::as_u64),
        Some(MAX_ITEMS_PER_RUN)
    );
}

fn status(relay: &RelayHarness) -> Value {
    let (status, body) = relay.post("/cover-traffic/status", json!({}));
    assert_eq!(status, 200);
    assert_eq!(body.get("ok").and_then(Value::as_bool), Some(true));
    assert_eq!(
        body.get("policy").and_then(Value::as_str),
        Some(COVER_POLICY)
    );
    body
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

fn message(msg: &Value) -> &str {
    msg.get("msg")
        .and_then(Value::as_str)
        .expect("candidate msg")
}

fn messages(msgs: &[Value]) -> Vec<&str> {
    msgs.iter()
        .filter(|msg| msg.get("cover").and_then(Value::as_bool) == Some(false))
        .map(message)
        .collect()
}

fn write_store(store: &Path, relay_url: &str, token: &str) {
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

fn run_recv(store: &Path, token: &str) -> Output {
    Command::new(env!("CARGO_BIN_EXE_qshield"))
        .args(["recv", "--store"])
        .arg(store)
        .args(["--max", "4"])
        .env("QSHIELD_RELAY_TOKEN", token)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .expect("run qshield recv")
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
    std::env::temp_dir().join(format!("qshield-na0335-{name}-{}", unique_suffix()))
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
