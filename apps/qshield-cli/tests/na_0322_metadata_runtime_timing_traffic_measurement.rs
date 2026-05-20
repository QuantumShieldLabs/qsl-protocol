use std::fs::{self, File};
use std::io::Write;
use std::net::TcpListener;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Output, Stdio};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use serde_json::{json, Map, Value};

const SCHEMA_VERSION: &str = "qsl.na0322.metadata_runtime_timing_traffic_measurement.v1";
const RUN_ID: &str = "na0322-qshield-demo-bounded";
const BUCKETS: &[usize] = &[512, 1024, 2048, 4096, 8192];

const MARKERS: &[&str] = &[
    "NA0322_TIMING_SURFACE_INVENTORY_OK",
    "NA0322_TRAFFIC_SHAPE_THREAT_MODEL_OK",
    "NA0322_QSHIELD_DEMO_TIMING_MEASUREMENT_OK",
    "NA0322_SENDER_CADENCE_MEASURED_OK",
    "NA0322_RECEIVER_CADENCE_MEASURED_OK",
    "NA0322_QUEUE_CADENCE_MEASURED_OK",
    "NA0322_ACK_COMMIT_TIMING_MEASURED_OK",
    "NA0322_INVALID_RETRY_CADENCE_BOUNDED_OK",
    "NA0322_PADDING_SIZE_DISTRIBUTION_OK",
    "NA0322_ORDERING_CORRELATION_CLASSIFIED_OK",
    "NA0322_NO_SECRET_TIMING_ARTIFACT_OK",
    "NA0322_MEASUREMENT_NOT_MITIGATION_OK",
    "NA0322_METADATA_TIMING_MEASUREMENT_HARNESS_OK",
    "NA0322_QSHIELD_DEMO_BOUNDARY_OK",
    "NA0322_QSL_SERVER_TIMING_NOT_PROVEN_OK",
    "NA0322_QSL_ATTACHMENTS_TIMING_NOT_PROVEN_OK",
    "NA0322_NO_METADATA_FREE_CLAIM_OK",
];

const ROUTE_TOKEN_SENTINEL: &str = "NA0322_ROUTE_TOKEN_SENTINEL_DO_NOT_LEAK";
const RAW_HANDLE_SENTINEL: &str = "NA0322_RAW_HANDLE_SENTINEL_DO_NOT_LEAK";
const CANDIDATE_ACK_SENTINEL: &str = "NA0322_CANDIDATE_ACK_SENTINEL_DO_NOT_LEAK";
const PLAINTEXT_SENTINEL: &str = "NA0322_PLAINTEXT_SENTINEL_DO_NOT_LEAK";
const PADDING_SENTINEL: &str = "NA0322_PADDING_SENTINEL_DO_NOT_LEAK";
const PASSPHRASE_SENTINEL: &str = "NA0322_PASSPHRASE_SENTINEL_DO_NOT_LEAK";
const KEY_SENTINEL: &str = "NA0322_RAW_KEY_SENTINEL_DO_NOT_LEAK";

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

struct MeasurementArtifact {
    started: Instant,
    seq: u64,
    path: PathBuf,
    file: File,
}

impl MeasurementArtifact {
    fn create() -> Self {
        let root = artifact_root();
        fs::create_dir_all(&root).expect("create artifact root");
        let path = root.join("na0322_metadata_runtime_timing_traffic_measurement.jsonl");
        let file = File::create(&path).expect("create measurement artifact");
        Self {
            started: Instant::now(),
            seq: 0,
            path,
            file,
        }
    }

    fn record(&mut self, event: &str, duration: Duration, fields: Value) {
        self.seq += 1;
        let mut object = Map::new();
        object.insert("schema_version".to_string(), json!(SCHEMA_VERSION));
        object.insert("run_id".to_string(), json!(RUN_ID));
        object.insert("sequence".to_string(), json!(self.seq));
        object.insert(
            "relative_ms".to_string(),
            json!(self.started.elapsed().as_millis()),
        );
        object.insert("event".to_string(), json!(event));
        object.insert("duration_us".to_string(), json!(duration.as_micros()));
        object.insert(
            "artifact_class".to_string(),
            json!("measurement_not_mitigation"),
        );
        object.insert(
            "boundary".to_string(),
            json!("qshield_embedded_relay_demo_only"),
        );
        if let Some(fields) = fields.as_object() {
            for (key, value) in fields {
                object.insert(key.clone(), value.clone());
            }
        }
        writeln!(self.file, "{}", Value::Object(object)).expect("write measurement event");
    }

    fn flush(&mut self) {
        self.file.flush().expect("flush measurement artifact");
    }
}

#[test]
fn qshield_demo_timing_traffic_shape_measurement_artifact_is_secret_safe() {
    let relay = RelayHarness::start("cadence");
    let invalid_relay = RelayHarness::start("invalid-retry");
    let mut artifact = MeasurementArtifact::create();
    let mut sensitive_values = vec![relay.token.clone(), invalid_relay.token.clone()];

    artifact.record(
        "timing_surface_inventory",
        Duration::ZERO,
        json!({
            "surface": "qshield_demo_measurement_inventory",
            "sender_cadence": "explicit_send_command_timing_observable",
            "receiver_cadence": "candidate_fetch_ack_and_invalid_recv_timing_observable",
            "queue_cadence": "candidate_count_and_ack_delete_observable",
            "size_padding": "bucket_class_observable",
            "ordering_correlation": "front_of_queue_and_ack_order_observable",
            "qsl_server_boundary": "not_measured",
            "qsl_attachments_boundary": "not_measured"
        }),
    );

    let initial_fetch = timed(|| candidates(&relay, "recipient-class-a", 8));
    assert!(initial_fetch.value.is_empty());
    artifact.record(
        "queue_depth_before_send",
        initial_fetch.duration,
        json!({
            "surface": "relay_poll_candidate",
            "queue_depth_observed": 0,
            "result_class": "empty_queue",
            "raw_candidate_values": "redacted"
        }),
    );

    for burst_index in 0..3 {
        let msg = format!("{:02x}", burst_index + 1);
        let sent = timed(|| send_raw(&relay, "recipient-class-a", "sender-class-a", &msg));
        artifact.record(
            if burst_index == 0 {
                "sender_send_single"
            } else {
                "sender_send_burst"
            },
            sent.duration,
            json!({
                "surface": "relay_send",
                "sender_cadence_class": "explicit_command_burst",
                "burst_index": burst_index,
                "result_class": "queued",
                "payload_class": "redacted_demo_hex"
            }),
        );
    }
    marker("NA0322_SENDER_CADENCE_MEASURED_OK");

    let fetched = timed(|| candidates(&relay, "recipient-class-a", 8));
    assert_eq!(fetched.value.len(), 3);
    let first_ack = ack_id(&fetched.value[0]).to_string();
    let second_ack = ack_id(&fetched.value[1]).to_string();
    let third_ack = ack_id(&fetched.value[2]).to_string();
    sensitive_values.extend([first_ack.clone(), second_ack.clone(), third_ack.clone()]);
    artifact.record(
        "receiver_candidate_fetch",
        fetched.duration,
        json!({
            "surface": "relay_poll_candidate",
            "receiver_cadence_class": "explicit_candidate_fetch",
            "queue_depth_observed": 3,
            "candidate_count": 3,
            "raw_candidate_values": "redacted"
        }),
    );

    let repeated_fetch = timed(|| candidates(&relay, "recipient-class-a", 8));
    assert_eq!(ack_id(&repeated_fetch.value[0]), first_ack);
    artifact.record(
        "queue_candidate_fetch_no_delete",
        repeated_fetch.duration,
        json!({
            "surface": "relay_poll_candidate",
            "queue_depth_observed": 3,
            "candidate_count": 3,
            "queue_cadence_class": "fetch_without_delete",
            "raw_candidate_values": "redacted"
        }),
    );
    marker("NA0322_RECEIVER_CADENCE_MEASURED_OK");
    marker("NA0322_QUEUE_CADENCE_MEASURED_OK");

    let acked = timed(|| ack(&relay, "recipient-class-a", &first_ack));
    artifact.record(
        "ack_commit_delete_one",
        acked.duration,
        json!({
            "surface": "relay_ack",
            "ack_commit_class": "valid_ack_deletes_one_candidate",
            "raw_ack_id": "redacted",
            "result_class": "acked"
        }),
    );
    let after_ack = candidates(&relay, "recipient-class-a", 8);
    assert_eq!(after_ack.len(), 2);
    assert_eq!(ack_id(&after_ack[0]), second_ack);
    artifact.record(
        "queue_depth_after_ack",
        Duration::ZERO,
        json!({
            "surface": "relay_poll_candidate",
            "queue_depth_observed": 2,
            "candidate_count": 2,
            "queue_cadence_class": "valid_ack_removed_one"
        }),
    );
    marker("NA0322_ACK_COMMIT_TIMING_MEASURED_OK");

    artifact.record(
        "ordering_correlation_classified",
        Duration::ZERO,
        json!({
            "surface": "relay_queue_ordering",
            "ordering_class": "front_of_queue_candidate_order_visible",
            "correlation_class": "recipient_queue_and_ack_order_visible_in_demo_relay",
            "raw_route_or_candidate_values": "redacted"
        }),
    );
    marker("NA0322_ORDERING_CORRELATION_CLASSIFIED_OK");

    let padding_samples = [(31usize, BUCKETS[0]), (700usize, BUCKETS[1])];
    for (idx, (payload_len, bucket)) in padding_samples.iter().copied().enumerate() {
        let payload = vec![b'a' + idx as u8; payload_len];
        let padded = pad_to_bucket(&payload, bucket);
        let sent = timed(|| {
            send_padded(
                &relay,
                "padding-recipient-class",
                "padding-sender-class",
                &padded,
                bucket - payload_len,
                bucket,
            )
        });
        artifact.record(
            "padding_size_distribution_sample",
            sent.duration,
            json!({
                "surface": "relay_send_padding_metadata",
                "bucket_class": format!("bucket_{bucket}"),
                "observable_size_class": format!("padded_bucket_{bucket}"),
                "plaintext_size_class": format!("sample_class_{}", idx + 1),
                "raw_plaintext_size": "not_recorded",
                "raw_plaintext": "not_recorded"
            }),
        );
    }
    let padding_candidates = candidates(&relay, "padding-recipient-class", 8);
    assert_eq!(padding_candidates.len(), padding_samples.len());
    assert_eq!(
        padding_candidates[0].get("bucket").and_then(Value::as_u64),
        Some(BUCKETS[0] as u64)
    );
    assert_eq!(
        padding_candidates[1].get("bucket").and_then(Value::as_u64),
        Some(BUCKETS[1] as u64)
    );
    marker("NA0322_PADDING_SIZE_DISTRIBUTION_OK");

    let invalid_store = invalid_relay.root.join("bob-store");
    write_store_with_session(
        &invalid_store,
        &invalid_relay.base_url(),
        &invalid_relay.token,
    );
    send_raw(
        &invalid_relay,
        "bob",
        "alice",
        &format!("not-hex-{PLAINTEXT_SENTINEL}"),
    );
    let invalid_before = candidates(&invalid_relay, "bob", 1);
    assert_eq!(invalid_before.len(), 1);
    let invalid_ack = ack_id(&invalid_before[0]).to_string();
    sensitive_values.push(invalid_ack.clone());

    let invalid_first = timed(|| run_recv(&invalid_store, &invalid_relay.token));
    assert!(!invalid_first.value.status.success());
    let invalid_first_text = combined_output(&invalid_first.value);
    assert_no_secret_text(&invalid_first_text, &[&invalid_relay.token, &invalid_ack]);
    artifact.record(
        "invalid_recv_first_reject",
        invalid_first.duration,
        json!({
            "surface": "qshield_recv_invalid_decode",
            "receiver_cadence_class": "explicit_recv_command_reject",
            "invalid_retry_class": "first_invalid_attempt_retained_candidate",
            "queue_depth_observed": 1,
            "raw_ack_id": "redacted",
            "raw_output": "not_recorded"
        }),
    );
    let invalid_after_first = candidates(&invalid_relay, "bob", 1);
    assert_eq!(ack_id(&invalid_after_first[0]), invalid_ack);

    let invalid_second = timed(|| run_recv(&invalid_store, &invalid_relay.token));
    assert!(!invalid_second.value.status.success());
    let invalid_second_text = combined_output(&invalid_second.value);
    assert_eq!(invalid_second_text, invalid_first_text);
    assert_no_secret_text(&invalid_second_text, &[&invalid_relay.token, &invalid_ack]);
    let invalid_after_second = candidates(&invalid_relay, "bob", 1);
    assert_eq!(ack_id(&invalid_after_second[0]), invalid_ack);
    artifact.record(
        "invalid_recv_retry_reject",
        invalid_second.duration,
        json!({
            "surface": "qshield_recv_invalid_decode",
            "receiver_cadence_class": "explicit_recv_command_reject",
            "invalid_retry_class": "repeat_invalid_attempt_bounded_same_output",
            "queue_depth_observed": 1,
            "raw_ack_id": "redacted",
            "raw_output": "not_recorded"
        }),
    );
    marker("NA0322_INVALID_RETRY_CADENCE_BOUNDED_OK");

    artifact.record(
        "measurement_boundary",
        Duration::ZERO,
        json!({
            "measurement_status": "records_observable_timing_and_shape_only",
            "mitigation_status": "not_implemented",
            "runtime_timing_jitter_batching_cover_traffic": "not_changed",
            "metadata_free_claim": "not_claimed",
            "timing_hidden_claim": "not_claimed",
            "traffic_shape_hidden_claim": "not_claimed",
            "qsl_server_timing": "not_proven",
            "qsl_attachments_timing": "not_proven"
        }),
    );

    artifact.flush();
    let artifact_text = fs::read_to_string(&artifact.path).expect("read measurement artifact");
    assert_measurement_artifact_safe(&artifact_text, &sensitive_values);
    println!("NA0322_ARTIFACT_PATH {}", artifact.path.display());
    println!("ARTIFACT_SECRET_FINDING_COUNT 0");

    marker("NA0322_TIMING_SURFACE_INVENTORY_OK");
    marker("NA0322_TRAFFIC_SHAPE_THREAT_MODEL_OK");
    marker("NA0322_QSHIELD_DEMO_TIMING_MEASUREMENT_OK");
    marker("NA0322_NO_SECRET_TIMING_ARTIFACT_OK");
    marker("NA0322_MEASUREMENT_NOT_MITIGATION_OK");
    marker("NA0322_METADATA_TIMING_MEASUREMENT_HARNESS_OK");
    marker("NA0322_QSHIELD_DEMO_BOUNDARY_OK");
    marker("NA0322_QSL_SERVER_TIMING_NOT_PROVEN_OK");
    marker("NA0322_QSL_ATTACHMENTS_TIMING_NOT_PROVEN_OK");
    marker("NA0322_NO_METADATA_FREE_CLAIM_OK");
}

#[test]
fn required_markers_are_declared() {
    for marker in MARKERS {
        assert!(marker.starts_with("NA0322_"));
        println!("{marker}");
    }
}

fn timed<T>(operation: impl FnOnce() -> T) -> Timed<T> {
    let start = Instant::now();
    let value = operation();
    Timed {
        value,
        duration: start.elapsed(),
    }
}

struct Timed<T> {
    value: T,
    duration: Duration,
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

fn pad_to_bucket(payload: &[u8], bucket: usize) -> Vec<u8> {
    assert!(bucket >= payload.len());
    let mut padded = payload.to_vec();
    padded.extend(std::iter::repeat_n(0u8, bucket - payload.len()));
    padded
}

fn write_store_with_session(store: &Path, relay_url: &str, token: &str) {
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

fn combined_output(output: &Output) -> String {
    let mut bytes = Vec::new();
    bytes.extend_from_slice(&output.stdout);
    bytes.extend_from_slice(&output.stderr);
    String::from_utf8_lossy(&bytes).into_owned()
}

fn assert_no_secret_text(text: &str, extras: &[&str]) {
    for forbidden in [
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
    ] {
        assert!(
            !text.contains(forbidden),
            "output leaked forbidden text: {forbidden}"
        );
    }
    for forbidden in extras {
        assert!(
            !forbidden.is_empty() && !text.contains(forbidden),
            "output leaked forbidden text: {forbidden}"
        );
    }
}

fn assert_measurement_artifact_safe(text: &str, sensitive_values: &[String]) {
    for forbidden in [
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
    ] {
        assert!(
            !text.contains(forbidden),
            "artifact leaked forbidden text: {forbidden}"
        );
    }
    for forbidden in sensitive_values {
        assert!(
            !forbidden.is_empty() && !text.contains(forbidden),
            "artifact leaked sensitive value: {forbidden}"
        );
    }
}

fn marker(marker: &str) {
    assert!(MARKERS.contains(&marker));
    println!("{marker}");
}

fn artifact_root() -> PathBuf {
    if let Ok(path) = std::env::var("NA0322_ARTIFACT_DIR") {
        return PathBuf::from(path);
    }

    let preferred = PathBuf::from("/srv/qbuild/tmp");
    let base = if fs::create_dir_all(&preferred).is_ok() {
        preferred
    } else {
        std::env::temp_dir()
    };
    base.join(format!(
        "NA-0322_metadata_runtime_timing_traffic_measurement_{}",
        unique_suffix()
    ))
}

fn unique_temp_dir(name: &str) -> PathBuf {
    std::env::temp_dir().join(format!("qshield-na0322-{name}-{}", unique_suffix()))
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
