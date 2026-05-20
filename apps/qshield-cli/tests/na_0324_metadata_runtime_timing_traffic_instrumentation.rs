use std::collections::BTreeSet;
use std::fs::{self, File};
use std::io::Write;
use std::net::TcpListener;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Output, Stdio};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use serde_json::{json, Map, Value};

const SCHEMA_VERSION: &str = "qsl.na0324.metadata_runtime_timing_traffic_instrumentation.v1";
const RUN_ID: &str = "na0324-qshield-demo-instrumentation-bounded";
const BUCKETS: &[usize] = &[512, 1024, 2048, 4096, 8192];

const MARKERS: &[&str] = &[
    "NA0324_TIMING_INSTRUMENTATION_PLAN_OK",
    "NA0324_QSHIELD_DEMO_TRACE_ARTIFACT_SCHEMA_OK",
    "NA0324_QSHIELD_EVENT_TIMING_CAPTURE_OK",
    "NA0324_QUEUE_CADENCE_INSTRUMENTATION_OK",
    "NA0324_PADDING_SIZE_CLASS_INSTRUMENTATION_OK",
    "NA0324_INVALID_RETRY_INSTRUMENTATION_OK",
    "NA0324_NO_SECRET_TRACE_ARTIFACT_OK",
    "NA0324_INSTRUMENTATION_NOT_MITIGATION_OK",
    "NA0324_MEASUREMENT_BEFORE_MITIGATION_OK",
    "NA0324_QSHIELD_DEMO_BOUNDARY_OK",
    "NA0324_QSL_SERVER_TIMING_NOT_PROVEN_OK",
    "NA0324_QSL_ATTACHMENTS_TIMING_NOT_PROVEN_OK",
    "NA0324_NO_TIMING_HIDDEN_CLAIM_OK",
    "NA0324_NO_TRAFFIC_SHAPE_HIDDEN_CLAIM_OK",
    "NA0324_NO_METADATA_FREE_CLAIM_OK",
    "NA0324_METADATA_TIMING_INSTRUMENTATION_HARNESS_OK",
];

const ROUTE_TOKEN_SENTINEL: &str = "NA0324_ROUTE_TOKEN_SENTINEL_DO_NOT_LEAK";
const RAW_HANDLE_SENTINEL: &str = "NA0324_RAW_HANDLE_SENTINEL_DO_NOT_LEAK";
const CANDIDATE_ACK_SENTINEL: &str = "NA0324_CANDIDATE_ACK_SENTINEL_DO_NOT_LEAK";
const PLAINTEXT_SENTINEL: &str = "NA0324_PLAINTEXT_SENTINEL_DO_NOT_LEAK";
const PADDING_SENTINEL: &str = "NA0324_PADDING_SENTINEL_DO_NOT_LEAK";
const PASSPHRASE_SENTINEL: &str = "NA0324_PASSPHRASE_SENTINEL_DO_NOT_LEAK";
const KEY_SENTINEL: &str = "NA0324_RAW_KEY_SENTINEL_DO_NOT_LEAK";

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

struct TraceArtifact {
    started: Instant,
    seq: u64,
    path: PathBuf,
    file: File,
}

impl TraceArtifact {
    fn create() -> Self {
        let root = artifact_root();
        fs::create_dir_all(&root).expect("create artifact root");
        let path = root.join("na0324_metadata_runtime_timing_traffic_instrumentation.jsonl");
        let file = File::create(&path).expect("create trace artifact");
        Self {
            started: Instant::now(),
            seq: 0,
            path,
            file,
        }
    }

    fn record(&mut self, event: &str, duration: Option<Duration>, fields: Value) {
        self.seq += 1;
        let mut object = Map::new();
        object.insert("schema_version".to_string(), json!(SCHEMA_VERSION));
        object.insert("run_id".to_string(), json!(RUN_ID));
        object.insert(
            "event_id".to_string(),
            json!(format!("evt-{:03}", self.seq)),
        );
        object.insert("sequence".to_string(), json!(self.seq));
        object.insert(
            "relative_ms".to_string(),
            json!(self.started.elapsed().as_millis()),
        );
        object.insert("event".to_string(), json!(event));
        object.insert(
            "artifact_class".to_string(),
            json!("instrumentation_measurement_not_mitigation"),
        );
        object.insert(
            "boundary".to_string(),
            json!("qshield_embedded_relay_demo_only"),
        );
        object.insert(
            "duration_class".to_string(),
            json!(duration.map(duration_class).unwrap_or("not_measured")),
        );
        object.insert("phase_class".to_string(), json!("not_applicable"));
        object.insert("queue_depth_class".to_string(), json!("not_observed"));
        object.insert("padding_size_class".to_string(), json!("not_applicable"));
        object.insert("retry_count_class".to_string(), json!("not_applicable"));
        object.insert("correlation_class".to_string(), json!("not_observed"));
        if let Some(fields) = fields.as_object() {
            for (key, value) in fields {
                object.insert(key.clone(), value.clone());
            }
        }
        writeln!(self.file, "{}", Value::Object(object)).expect("write trace event");
    }

    fn flush(&mut self) {
        self.file.flush().expect("flush trace artifact");
    }
}

#[test]
fn qshield_demo_timing_traffic_shape_instrumentation_trace_is_secret_safe() {
    let relay = RelayHarness::start("trace");
    let invalid_relay = RelayHarness::start("invalid-retry");
    let mut trace = TraceArtifact::create();
    let mut sensitive_values = vec![relay.token.clone(), invalid_relay.token.clone()];

    trace.record(
        "instrumentation_plan",
        None,
        json!({
            "phase_class": "plan_boundary",
            "queue_depth_class": "not_observed",
            "padding_size_class": "not_applicable",
            "retry_count_class": "not_applicable",
            "correlation_class": "not_observed",
            "measurement_scope": "qshield_embedded_relay_demo_only",
            "mitigation_status": "not_implemented"
        }),
    );

    trace.record(
        "candidate_fetch_start",
        None,
        json!({
            "phase_class": "queue_probe",
            "queue_depth_class": "unknown_before_fetch",
            "correlation_class": "recipient_queue_probe",
            "raw_candidate_values": "not_recorded"
        }),
    );
    let initial_fetch = timed(|| candidates(&relay, "recipient-class-a", 8));
    assert!(initial_fetch.value.is_empty());
    trace.record(
        "candidate_fetch_complete",
        Some(initial_fetch.duration),
        json!({
            "phase_class": "queue_probe",
            "queue_depth_class": queue_depth_class(initial_fetch.value.len()),
            "correlation_class": "empty_recipient_queue_observable",
            "result_class": "empty_queue",
            "raw_candidate_values": "not_recorded"
        }),
    );

    trace.record(
        "send_start",
        None,
        json!({
            "phase_class": "sender_cadence",
            "queue_depth_class": "empty_before_send",
            "padding_size_class": "unpadded_demo_class",
            "correlation_class": "explicit_send_command_observable",
            "raw_plaintext": "not_recorded"
        }),
    );
    let sent = timed(|| send_raw(&relay, "recipient-class-a", "sender-class-a", "aa"));
    trace.record(
        "send_complete",
        Some(sent.duration),
        json!({
            "phase_class": "sender_cadence",
            "queue_depth_class": "one_after_send",
            "padding_size_class": "unpadded_demo_class",
            "correlation_class": "explicit_send_command_observable",
            "result_class": "queued",
            "raw_plaintext": "not_recorded"
        }),
    );

    trace.record(
        "candidate_fetch_start",
        None,
        json!({
            "phase_class": "receiver_cadence",
            "queue_depth_class": "unknown_before_fetch",
            "correlation_class": "front_of_queue_order_observable",
            "raw_candidate_values": "not_recorded"
        }),
    );
    let fetched = timed(|| candidates(&relay, "recipient-class-a", 8));
    assert_eq!(fetched.value.len(), 1);
    let first_ack = ack_id(&fetched.value[0]).to_string();
    sensitive_values.push(first_ack.clone());
    trace.record(
        "candidate_fetch_complete",
        Some(fetched.duration),
        json!({
            "phase_class": "receiver_cadence",
            "queue_depth_class": queue_depth_class(fetched.value.len()),
            "correlation_class": "front_of_queue_and_ack_order_visible_in_demo_relay",
            "result_class": "candidate_available",
            "raw_ack_id": "redacted",
            "raw_candidate_values": "not_recorded"
        }),
    );

    trace.record(
        "local_verify_start",
        None,
        json!({
            "phase_class": "local_verify_before_commit",
            "queue_depth_class": "one_candidate_seen",
            "correlation_class": "candidate_class_only",
            "raw_candidate_values": "not_recorded"
        }),
    );
    let verified = timed(|| verify_candidate_class(&fetched.value[0], "sender-class-a"));
    trace.record(
        "local_verify_complete",
        Some(verified.duration),
        json!({
            "phase_class": "local_verify_before_commit",
            "queue_depth_class": "one_candidate_seen",
            "correlation_class": "candidate_class_only",
            "result_class": "verified_class_only",
            "raw_candidate_values": "not_recorded"
        }),
    );

    trace.record(
        "ack_start",
        None,
        json!({
            "phase_class": "ack_commit_timing",
            "queue_depth_class": "one_before_ack",
            "correlation_class": "ack_after_local_verify_observable",
            "raw_ack_id": "redacted"
        }),
    );
    let acked = timed(|| ack(&relay, "recipient-class-a", &first_ack));
    trace.record(
        "ack_complete",
        Some(acked.duration),
        json!({
            "phase_class": "ack_commit_timing",
            "queue_depth_class": "empty_after_ack",
            "correlation_class": "valid_ack_deletes_one_candidate",
            "result_class": "acked_after_verify",
            "raw_ack_id": "redacted"
        }),
    );
    let after_ack = candidates(&relay, "recipient-class-a", 8);
    assert!(after_ack.is_empty());

    for (idx, (payload_len, bucket)) in [(31usize, BUCKETS[0]), (700usize, BUCKETS[1])]
        .iter()
        .copied()
        .enumerate()
    {
        let payload = vec![b'a' + idx as u8; payload_len];
        let padded = pad_to_bucket(&payload, bucket);
        trace.record(
            "send_start",
            None,
            json!({
                "phase_class": "padding_size_class",
                "queue_depth_class": "not_observed",
                "padding_size_class": bucket_class(bucket),
                "correlation_class": "bucket_class_observable",
                "raw_plaintext": "not_recorded",
                "raw_plaintext_size": "not_recorded"
            }),
        );
        let padded_send = timed(|| {
            send_padded(
                &relay,
                "padding-recipient-class",
                "padding-sender-class",
                &padded,
                bucket - payload_len,
                bucket,
            )
        });
        trace.record(
            "send_complete",
            Some(padded_send.duration),
            json!({
                "phase_class": "padding_size_class",
                "queue_depth_class": "not_observed",
                "padding_size_class": bucket_class(bucket),
                "correlation_class": "bucket_class_observable",
                "result_class": "queued_padded_bucket_class",
                "raw_plaintext": "not_recorded",
                "raw_plaintext_size": "not_recorded"
            }),
        );
    }
    let padding_candidates = candidates(&relay, "padding-recipient-class", 8);
    assert_eq!(padding_candidates.len(), 2);
    assert_eq!(
        padding_candidates[0].get("bucket").and_then(Value::as_u64),
        Some(BUCKETS[0] as u64)
    );
    assert_eq!(
        padding_candidates[1].get("bucket").and_then(Value::as_u64),
        Some(BUCKETS[1] as u64)
    );

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

    trace.record(
        "invalid_retry_start",
        None,
        json!({
            "phase_class": "invalid_retry_cadence",
            "queue_depth_class": "one_before_invalid_retry",
            "retry_count_class": "first_invalid_attempt",
            "correlation_class": "invalid_candidate_retention_observable",
            "raw_ack_id": "redacted"
        }),
    );
    let invalid_first = timed(|| run_recv(&invalid_store, &invalid_relay.token));
    assert!(!invalid_first.value.status.success());
    let invalid_first_text = combined_output(&invalid_first.value);
    assert_no_secret_text(&invalid_first_text, &[&invalid_relay.token, &invalid_ack]);
    trace.record(
        "invalid_retry_complete",
        Some(invalid_first.duration),
        json!({
            "phase_class": "invalid_retry_cadence",
            "queue_depth_class": "one_retained_after_invalid_retry",
            "retry_count_class": "first_invalid_attempt",
            "correlation_class": "invalid_candidate_retention_observable",
            "result_class": "fail_closed_retained_candidate",
            "raw_ack_id": "redacted",
            "raw_output": "not_recorded"
        }),
    );
    let invalid_after_first = candidates(&invalid_relay, "bob", 1);
    assert_eq!(ack_id(&invalid_after_first[0]), invalid_ack);

    trace.record(
        "output_write_start",
        None,
        json!({
            "phase_class": "output_classification",
            "queue_depth_class": "one_retained_after_invalid_retry",
            "retry_count_class": "first_invalid_attempt",
            "correlation_class": "sanitized_reject_output_class_only",
            "output_class": "classify_reject_output",
            "raw_output": "not_recorded"
        }),
    );
    trace.record(
        "output_write_complete",
        None,
        json!({
            "phase_class": "output_classification",
            "queue_depth_class": "one_retained_after_invalid_retry",
            "retry_count_class": "first_invalid_attempt",
            "correlation_class": "sanitized_reject_output_class_only",
            "output_class": "sanitized_reject_no_local_message_output",
            "raw_output": "not_recorded"
        }),
    );

    trace.record(
        "invalid_retry_start",
        None,
        json!({
            "phase_class": "invalid_retry_cadence",
            "queue_depth_class": "one_before_invalid_retry",
            "retry_count_class": "repeat_invalid_attempt",
            "correlation_class": "invalid_candidate_retention_observable",
            "raw_ack_id": "redacted"
        }),
    );
    let invalid_second = timed(|| run_recv(&invalid_store, &invalid_relay.token));
    assert!(!invalid_second.value.status.success());
    let invalid_second_text = combined_output(&invalid_second.value);
    assert_eq!(invalid_second_text, invalid_first_text);
    assert_no_secret_text(&invalid_second_text, &[&invalid_relay.token, &invalid_ack]);
    let invalid_after_second = candidates(&invalid_relay, "bob", 1);
    assert_eq!(ack_id(&invalid_after_second[0]), invalid_ack);
    trace.record(
        "invalid_retry_complete",
        Some(invalid_second.duration),
        json!({
            "phase_class": "invalid_retry_cadence",
            "queue_depth_class": "one_retained_after_invalid_retry",
            "retry_count_class": "repeat_invalid_attempt",
            "correlation_class": "same_invalid_candidate_same_output_bounded",
            "result_class": "fail_closed_retained_candidate",
            "raw_ack_id": "redacted",
            "raw_output": "not_recorded"
        }),
    );

    trace.record(
        "instrumentation_boundary",
        None,
        json!({
            "phase_class": "claim_boundary",
            "queue_depth_class": "not_observed",
            "padding_size_class": "not_applicable",
            "retry_count_class": "not_applicable",
            "correlation_class": "runtime_gaps_remain_visible",
            "instrumentation_status": "measurement_evidence_only",
            "mitigation_status": "not_implemented",
            "jitter_batching_cover_traffic_status": "not_implemented",
            "timing_hidden_claim": "not_claimed",
            "traffic_shape_hidden_claim": "not_claimed",
            "metadata_free_claim": "not_claimed",
            "qsl_server_timing": "not_proven",
            "qsl_attachments_timing": "not_proven"
        }),
    );

    trace.flush();
    let artifact_text = fs::read_to_string(&trace.path).expect("read trace artifact");
    assert_trace_schema(&artifact_text);
    assert_trace_artifact_safe(&artifact_text, &sensitive_values);
    assert!(artifact_text.len() < 64 * 1024, "trace artifact is bounded");
    println!("NA0324_ARTIFACT_PATH {}", trace.path.display());
    println!("TRACE_ARTIFACT_SECRET_FINDING_COUNT 0");

    marker("NA0324_TIMING_INSTRUMENTATION_PLAN_OK");
    marker("NA0324_QSHIELD_DEMO_TRACE_ARTIFACT_SCHEMA_OK");
    marker("NA0324_QSHIELD_EVENT_TIMING_CAPTURE_OK");
    marker("NA0324_QUEUE_CADENCE_INSTRUMENTATION_OK");
    marker("NA0324_PADDING_SIZE_CLASS_INSTRUMENTATION_OK");
    marker("NA0324_INVALID_RETRY_INSTRUMENTATION_OK");
    marker("NA0324_NO_SECRET_TRACE_ARTIFACT_OK");
    marker("NA0324_INSTRUMENTATION_NOT_MITIGATION_OK");
    marker("NA0324_MEASUREMENT_BEFORE_MITIGATION_OK");
    marker("NA0324_QSHIELD_DEMO_BOUNDARY_OK");
    marker("NA0324_QSL_SERVER_TIMING_NOT_PROVEN_OK");
    marker("NA0324_QSL_ATTACHMENTS_TIMING_NOT_PROVEN_OK");
    marker("NA0324_NO_TIMING_HIDDEN_CLAIM_OK");
    marker("NA0324_NO_TRAFFIC_SHAPE_HIDDEN_CLAIM_OK");
    marker("NA0324_NO_METADATA_FREE_CLAIM_OK");
    marker("NA0324_METADATA_TIMING_INSTRUMENTATION_HARNESS_OK");
}

#[test]
fn required_markers_are_declared() {
    for marker in MARKERS {
        assert!(marker.starts_with("NA0324_"));
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

fn verify_candidate_class(msg: &Value, expected_from: &str) {
    assert_eq!(msg.get("from").and_then(Value::as_str), Some(expected_from));
    assert!(msg.get("ack_id").and_then(Value::as_str).is_some());
    assert!(msg.get("msg").and_then(Value::as_str).is_some());
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

fn assert_trace_schema(text: &str) {
    let mut labels = BTreeSet::new();
    for (idx, line) in text.lines().enumerate() {
        let value: Value = serde_json::from_str(line).expect("trace line is json");
        assert_eq!(
            value.get("schema_version").and_then(Value::as_str),
            Some(SCHEMA_VERSION)
        );
        assert_eq!(value.get("run_id").and_then(Value::as_str), Some(RUN_ID));
        assert!(
            value
                .get("event_id")
                .and_then(Value::as_str)
                .is_some_and(|event_id| event_id.starts_with("evt-")),
            "line {idx} missing event_id"
        );
        assert!(
            value.get("relative_ms").and_then(Value::as_u64).is_some(),
            "line {idx} missing relative_ms"
        );
        for key in [
            "event",
            "phase_class",
            "queue_depth_class",
            "padding_size_class",
            "retry_count_class",
            "correlation_class",
            "boundary",
            "artifact_class",
            "duration_class",
        ] {
            assert!(
                value.get(key).and_then(Value::as_str).is_some(),
                "line {idx} missing {key}"
            );
        }
        assert_eq!(
            value.get("boundary").and_then(Value::as_str),
            Some("qshield_embedded_relay_demo_only")
        );
        labels.insert(
            value
                .get("event")
                .and_then(Value::as_str)
                .expect("event label")
                .to_string(),
        );
    }

    for required in [
        "send_start",
        "send_complete",
        "candidate_fetch_start",
        "candidate_fetch_complete",
        "local_verify_start",
        "local_verify_complete",
        "ack_start",
        "ack_complete",
        "invalid_retry_start",
        "invalid_retry_complete",
        "output_write_start",
        "output_write_complete",
    ] {
        assert!(labels.contains(required), "missing trace event {required}");
    }
}

fn assert_no_secret_text(text: &str, extras: &[&str]) {
    for forbidden in forbidden_artifact_terms() {
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

fn assert_trace_artifact_safe(text: &str, sensitive_values: &[String]) {
    for forbidden in forbidden_artifact_terms() {
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

fn marker(marker: &str) {
    assert!(MARKERS.contains(&marker));
    println!("{marker}");
}

fn queue_depth_class(depth: usize) -> &'static str {
    match depth {
        0 => "empty",
        1 => "single",
        2..=8 => "small_bounded",
        _ => "large_not_used_by_harness",
    }
}

fn bucket_class(bucket: usize) -> String {
    format!("bucket_{bucket}")
}

fn duration_class(duration: Duration) -> &'static str {
    let micros = duration.as_micros();
    match micros {
        0 => "zero",
        1..=1_000 => "le_1ms",
        1_001..=10_000 => "le_10ms",
        10_001..=100_000 => "le_100ms",
        _ => "gt_100ms",
    }
}

fn artifact_root() -> PathBuf {
    if let Ok(path) = std::env::var("NA0324_ARTIFACT_DIR") {
        return PathBuf::from(path);
    }

    let preferred = PathBuf::from("/srv/qbuild/tmp");
    let base = if fs::create_dir_all(&preferred).is_ok() {
        preferred
    } else {
        std::env::temp_dir()
    };
    base.join(format!(
        "NA-0324_metadata_runtime_timing_traffic_instrumentation_{}",
        unique_suffix()
    ))
}

fn unique_temp_dir(name: &str) -> PathBuf {
    std::env::temp_dir().join(format!("qshield-na0324-{name}-{}", unique_suffix()))
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
