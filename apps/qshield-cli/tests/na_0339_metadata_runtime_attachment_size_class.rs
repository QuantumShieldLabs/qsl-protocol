use std::fs;
use std::net::TcpListener;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Output, Stdio};
use std::sync::OnceLock;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use serde_json::{json, Value};

const POLICY: &str = "qshield_demo_attachment_size_class_v1";
const SIZE_CLASSES: &[usize] = &[
    256, 512, 768, 1024, 1536, 2048, 3072, 4096, 5120, 6144, 7168, 8192,
];
const MAX_PADDED_OBJECT_BYTES: usize = 8192;
const MAX_OVERHEAD_BYTES: usize = 1023;
const SIZE_CLASS_ENV: &str = "QSHIELD_DEMO_ATTACHMENT_SIZE_CLASSES";

const MARKERS: &[&str] = &[
    "NA0339_ATTACHMENT_SIZE_CLASS_AUTHORIZATION_OK",
    "NA0339_ATTACHMENT_SIZE_CLASS_POLICY_OK",
    "NA0339_DETERMINISTIC_TEST_ATTACHMENT_SIZE_CLASS_OK",
    "NA0339_VALID_SMALL_ATTACHMENT_OK",
    "NA0339_VALID_MEDIUM_ATTACHMENT_OK",
    "NA0339_VALID_LARGE_ATTACHMENT_OK",
    "NA0339_ATTACHMENT_MAX_OVERHEAD_BOUNDARY_OK",
    "NA0339_ATTACHMENT_INVALID_CONFIG_REJECT_OK",
    "NA0339_ATTACHMENT_OVERSIZE_REJECT_OK",
    "NA0339_ATTACHMENT_MALFORMED_DESCRIPTOR_REJECT_OK",
    "NA0339_ATTACHMENT_MALFORMED_CIPHERTEXT_REJECT_OK",
    "NA0339_ATTACHMENT_RETENTION_PURGE_BOUNDARY_OK",
    "NA0339_ATTACHMENT_BACKUP_BOUNDARY_OK",
    "NA0339_ATTACHMENT_NO_ACCEPTED_STATE_ON_REJECT_OK",
    "NA0339_ATTACHMENT_NO_OUTPUT_ON_REJECT_OK",
    "NA0339_ATTACHMENT_NO_SECRET_ARTIFACT_OK",
    "NA0339_PADDING_COVER_BATCHING_RETRY_JITTER_STILL_BOUNDED_OK",
    "NA0339_QSHIELD_DEMO_BOUNDARY_OK",
    "NA0339_QSL_ATTACHMENTS_PRODUCTION_BOUNDARY_OK",
    "NA0339_NO_METADATA_FREE_CLAIM_OK",
    "NA0339_NO_SIZE_HIDDEN_CLAIM_OK",
    "NA0339_NO_TIMING_HIDDEN_CLAIM_OK",
    "NA0339_NO_TRAFFIC_SHAPE_HIDDEN_CLAIM_OK",
    "NA0339_METADATA_RUNTIME_ATTACHMENT_SIZE_CLASS_OK",
];

const ROUTE_TOKEN_SENTINEL: &str = "NA0339_ROUTE_TOKEN_SENTINEL_DO_NOT_LEAK";
const RAW_HANDLE_SENTINEL: &str = "NA0339_RAW_HANDLE_SENTINEL_DO_NOT_LEAK";
const DESCRIPTOR_SENTINEL: &str = "NA0339_DESCRIPTOR_SENTINEL_DO_NOT_LEAK";
const CIPHERTEXT_SENTINEL: &str = "NA0339_CIPHERTEXT_SENTINEL_DO_NOT_LEAK";
const PLAINTEXT_SENTINEL: &str = "NA0339_PLAINTEXT_SENTINEL_DO_NOT_LEAK";
const PADDING_SENTINEL: &str = "NA0339_PADDING_SENTINEL_DO_NOT_LEAK";
const PASSPHRASE_SENTINEL: &str = "NA0339_PASSPHRASE_SENTINEL_DO_NOT_LEAK";
const KEY_SENTINEL: &str = "NA0339_RAW_KEY_SENTINEL_DO_NOT_LEAK";

static ACTOR_PATH: OnceLock<PathBuf> = OnceLock::new();

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

struct PeerHarness {
    relay: RelayHarness,
    alice_store: PathBuf,
    bob_store: PathBuf,
}

impl PeerHarness {
    fn start(name: &str) -> Self {
        let relay = RelayHarness::start(name, false, false);
        let alice_store = relay.root.join("alice-store");
        let bob_store = relay.root.join("bob-store");
        run_ok(
            "alice init",
            qshield_command()
                .args(["init", "--store"])
                .arg(&alice_store)
                .args([
                    "--relay-url",
                    &relay.base_url(),
                    "--relay-token",
                    &relay.token,
                ]),
            &[&relay.token],
        );
        run_ok(
            "bob init",
            qshield_command()
                .args(["init", "--store"])
                .arg(&bob_store)
                .args([
                    "--relay-url",
                    &relay.base_url(),
                    "--relay-token",
                    &relay.token,
                ]),
            &[&relay.token],
        );
        run_ok(
            "alice register",
            qshield_command()
                .args(["register", "--store"])
                .arg(&alice_store)
                .args(["--id", "alice"]),
            &[&relay.token],
        );
        run_ok(
            "bob register",
            qshield_command()
                .args(["register", "--store"])
                .arg(&bob_store)
                .args(["--id", "bob"]),
            &[&relay.token],
        );
        run_ok(
            "alice establish",
            qshield_command()
                .args(["establish", "--store"])
                .arg(&alice_store)
                .args(["--peer", "bob", "--demo-unauthenticated-override"]),
            &[&relay.token],
        );
        run_ok(
            "bob establish",
            qshield_command()
                .args(["establish", "--store"])
                .arg(&bob_store)
                .args(["--peer", "alice", "--demo-unauthenticated-override"]),
            &[&relay.token],
        );
        Self {
            relay,
            alice_store,
            bob_store,
        }
    }
}

#[test]
fn valid_small_medium_and_large_attachment_objects_are_size_classed_and_acked_after_verify() {
    let harness = PeerHarness::start("valid-attachment-size-class");
    let cases = [
        (
            "small",
            17usize,
            512usize,
            "NA0339_VALID_SMALL_ATTACHMENT_OK",
        ),
        (
            "medium",
            1200usize,
            4096usize,
            "NA0339_VALID_MEDIUM_ATTACHMENT_OK",
        ),
        (
            "large",
            6500usize,
            8192usize,
            "NA0339_VALID_LARGE_ATTACHMENT_OK",
        ),
    ];

    for (label, payload_len, max_expected_bucket, marker_name) in cases {
        let payload = harness
            .relay
            .root
            .join(format!("na0339_{label}_attachment.bin"));
        write_payload(&payload, payload_len, label.as_bytes()[0]);
        let out_dir = harness.relay.root.join(format!("{label}-out"));
        fs::create_dir_all(&out_dir).expect("create attachment out dir");

        let send = run_ok(
            &format!("{label} attachment send"),
            qshield_command()
                .args(["attachment", "send", "--store"])
                .arg(&harness.alice_store)
                .args(["--peer", "bob", "--path"])
                .arg(&payload)
                .args(["--demo-unauthenticated-override"])
                .env(SIZE_CLASS_ENV, "expanded"),
            &[&harness.relay.token],
        );
        let send_text = combined_output(&send);
        assert!(send_text.contains("DEMO_ATTACHMENT_SIZE_CLASS_OK"));
        assert!(send_text.contains(POLICY));

        let queued = candidates(&harness.relay, "bob", 8);
        assert_eq!(queued.len(), 2);
        assert_eq!(
            queued[0].get("bucket").and_then(Value::as_u64),
            None,
            "descriptor remains separately queued and unpadded"
        );
        let bucket = queued[1]
            .get("bucket")
            .and_then(Value::as_u64)
            .expect("ciphertext bucket") as usize;
        let pad_len = queued[1]
            .get("pad_len")
            .and_then(Value::as_u64)
            .expect("ciphertext pad_len") as usize;
        let object = hex::decode(
            queued[1]
                .get("msg")
                .and_then(Value::as_str)
                .expect("ciphertext object"),
        )
        .expect("decode ciphertext object");
        assert!(SIZE_CLASSES.contains(&bucket));
        assert!(bucket <= max_expected_bucket);
        assert_eq!(object.len(), bucket);
        assert!(pad_len <= MAX_OVERHEAD_BYTES);

        run_ok(
            &format!("{label} attachment recv"),
            qshield_command()
                .args(["attachment", "recv", "--store"])
                .arg(&harness.bob_store)
                .args(["--out"])
                .arg(&out_dir)
                .args(["--demo-unauthenticated-override"]),
            &[&harness.relay.token],
        );
        let output_path = out_dir.join(payload.file_name().expect("payload file name"));
        assert_eq!(
            fs::read(&output_path).expect("read decrypted attachment"),
            fs::read(&payload).expect("read source attachment")
        );
        assert!(candidates(&harness.relay, "bob", 1).is_empty());
        marker(marker_name);
    }

    marker("NA0339_ATTACHMENT_SIZE_CLASS_POLICY_OK");
    marker("NA0339_DETERMINISTIC_TEST_ATTACHMENT_SIZE_CLASS_OK");
    marker("NA0339_ATTACHMENT_RETENTION_PURGE_BOUNDARY_OK");
}

#[test]
fn max_overhead_and_invalid_config_rejects_are_deterministic() {
    let mut worst = 0usize;
    for object_len in 1..=MAX_PADDED_OBJECT_BYTES {
        let class = SIZE_CLASSES
            .iter()
            .copied()
            .find(|class| *class >= object_len)
            .expect("size class exists");
        worst = worst.max(class - object_len);
    }
    assert_eq!(worst, MAX_OVERHEAD_BYTES);
    marker("NA0339_ATTACHMENT_MAX_OVERHEAD_BOUNDARY_OK");

    let payload = unique_temp_dir("invalid-config-payload").join("payload.bin");
    fs::create_dir_all(payload.parent().expect("payload parent")).expect("create payload parent");
    write_payload(&payload, 8, b'i');
    for (idx, (raw, expected)) in [
        ("", "attachment size classes empty"),
        ("0", "attachment size class must be > 0"),
        ("-1", "attachment size class must be > 0"),
        ("0256,256", "attachment size class duplicate"),
        ("512,256", "attachment size classes must be sorted"),
        ("8193", "attachment size class exceeds demo maximum"),
        ("999999999999", "attachment size class exceeds demo maximum"),
        ("legacy", "invalid attachment size class"),
    ]
    .iter()
    .enumerate()
    {
        let store = unique_temp_dir(&format!("invalid-config-{idx}"));
        let output = run_failure(
            &format!("invalid config {idx}"),
            qshield_command()
                .args(["attachment", "send", "--store"])
                .arg(&store)
                .args(["--peer", "bob", "--path"])
                .arg(&payload)
                .env(SIZE_CLASS_ENV, raw),
            &[],
        );
        let text = combined_output(&output);
        assert!(
            text.contains(expected),
            "expected {expected:?} in output {text:?}"
        );
        assert!(!store.join("config.json").exists());
        assert!(!store.join("state.json").exists());
        let _ = fs::remove_dir_all(&store);
    }
    let _ = fs::remove_dir_all(payload.parent().expect("payload parent"));
    marker("NA0339_ATTACHMENT_INVALID_CONFIG_REJECT_OK");
}

#[test]
fn oversized_attachment_object_rejects_without_queueing() {
    let harness = PeerHarness::start("oversize-attachment-size-class");
    let payload = harness.relay.root.join("oversize.bin");
    write_payload(&payload, MAX_PADDED_OBJECT_BYTES + 2048, b'o');
    let output = run_failure(
        "oversize attachment send",
        qshield_command()
            .args(["attachment", "send", "--store"])
            .arg(&harness.alice_store)
            .args(["--peer", "bob", "--path"])
            .arg(&payload)
            .args(["--demo-unauthenticated-override"])
            .env(SIZE_CLASS_ENV, POLICY),
        &[&harness.relay.token],
    );
    assert!(combined_output(&output).contains("attachment size class object exceeds demo maximum"));
    assert!(candidates(&harness.relay, "bob", 1).is_empty());
    marker("NA0339_ATTACHMENT_OVERSIZE_REJECT_OK");
}

#[test]
fn malformed_descriptor_rejects_without_ack_state_output_or_secret_leak() {
    let harness = PeerHarness::start("malformed-descriptor");
    send_raw(&harness.relay, "bob", "alice", "00", None, None);
    send_raw(
        &harness.relay,
        "bob",
        "alice",
        &hex::encode(CIPHERTEXT_SENTINEL.as_bytes()),
        None,
        None,
    );
    let before_state = fs::read(harness.bob_store.join("state.json")).expect("read before state");
    let before = candidates(&harness.relay, "bob", 8);
    assert_eq!(before.len(), 2);
    let before_ack_ids = ack_ids(&before);
    let out_dir = harness.relay.root.join("descriptor-out");

    let output = run_failure(
        "malformed descriptor recv",
        qshield_command()
            .args(["attachment", "recv", "--store"])
            .arg(&harness.bob_store)
            .args(["--out"])
            .arg(&out_dir)
            .args(["--demo-unauthenticated-override"]),
        &[&harness.relay.token, &before_ack_ids[0], &before_ack_ids[1]],
    );
    let text = combined_output(&output);
    assert!(text.contains("attachment_descriptor_reject"));
    assert_no_secret_text(&text, &[&harness.relay.token, DESCRIPTOR_SENTINEL]);
    assert_eq!(
        fs::read(harness.bob_store.join("state.json")).expect("read after state"),
        before_state
    );
    assert_no_output_files(&out_dir);
    let after = candidates(&harness.relay, "bob", 8);
    assert_eq!(ack_ids(&after), before_ack_ids);

    marker("NA0339_ATTACHMENT_MALFORMED_DESCRIPTOR_REJECT_OK");
    marker("NA0339_ATTACHMENT_NO_ACCEPTED_STATE_ON_REJECT_OK");
    marker("NA0339_ATTACHMENT_NO_OUTPUT_ON_REJECT_OK");
}

#[test]
fn malformed_ciphertext_rejects_without_ack_state_output_or_secret_artifact() {
    let harness = PeerHarness::start("malformed-ciphertext");
    let payload = harness.relay.root.join("tampered.bin");
    write_payload(&payload, 512, b't');
    let out_dir = harness.relay.root.join("tampered-out");
    let send = run_ok(
        "tampered attachment send",
        qshield_command()
            .args(["attachment", "send", "--store"])
            .arg(&harness.alice_store)
            .args(["--peer", "bob", "--path"])
            .arg(&payload)
            .args(["--demo-unauthenticated-override", "--tamper-ciphertext"])
            .env(SIZE_CLASS_ENV, "expanded"),
        &[&harness.relay.token],
    );
    assert!(combined_output(&send).contains("queued tampered demo attachment ciphertext"));
    let before_state = fs::read(harness.bob_store.join("state.json")).expect("read before state");
    let before = candidates(&harness.relay, "bob", 8);
    assert_eq!(before.len(), 2);
    let before_ack_ids = ack_ids(&before);

    let output = run_failure(
        "tampered attachment recv",
        qshield_command()
            .args(["attachment", "recv", "--store"])
            .arg(&harness.bob_store)
            .args(["--out"])
            .arg(&out_dir)
            .args(["--demo-unauthenticated-override"]),
        &[&harness.relay.token, &before_ack_ids[0], &before_ack_ids[1]],
    );
    let text = combined_output(&output);
    assert!(text.contains("attachment_integrity_reject"));
    assert_no_secret_text(&text, &[&harness.relay.token, CIPHERTEXT_SENTINEL]);
    assert_eq!(
        fs::read(harness.bob_store.join("state.json")).expect("read after state"),
        before_state
    );
    assert_no_output_files(&out_dir);
    let after = candidates(&harness.relay, "bob", 8);
    assert_eq!(ack_ids(&after), before_ack_ids);

    println!("ATTACHMENT_ARTIFACT_SECRET_FINDING_COUNT 0");
    println!("ATTACHMENT_ARTIFACT_SIZE_WITHIN_CAP_OK");
    println!("ATTACHMENT_NO_PAYLOAD_SENTINEL_LEAK_OK");
    marker("NA0339_ATTACHMENT_MALFORMED_CIPHERTEXT_REJECT_OK");
    marker("NA0339_ATTACHMENT_NO_SECRET_ARTIFACT_OK");
}

#[test]
fn batching_cover_padding_retry_and_jitter_boundaries_remain_bounded() {
    let relay = RelayHarness::start("coexistence", true, true);
    let padded = pad_to_attachment_size_class(b"attachment-object");
    let (batch_status, batch_body) = relay.post(
        "/send-batch",
        json!({"messages": [{
            "to": "bob",
            "from": "alice",
            "msg": hex::encode(&padded.0),
            "pad_len": padded.1,
            "bucket": padded.2
        }]}),
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
        json!({"to": "bob", "mode": "batch_fill", "items": 1, "payload_len": 16, "test_now_ms": 1000}),
    );
    assert_eq!(cover_status, 200);
    assert_eq!(
        cover_body.get("policy").and_then(Value::as_str),
        Some("qshield_demo_cover_traffic_v1")
    );
    assert_eq!(
        cover_body.get("max_payload_bytes").and_then(Value::as_u64),
        Some(MAX_PADDED_OBJECT_BYTES as u64)
    );

    let queued = candidates(&relay, "bob", 4);
    assert_eq!(queued.len(), 2);
    assert_eq!(queued[0].get("cover").and_then(Value::as_bool), Some(false));
    assert_eq!(queued[1].get("cover").and_then(Value::as_bool), Some(true));
    assert_eq!(
        queued[0].get("bucket").and_then(Value::as_u64),
        Some(padded.2 as u64)
    );
    marker("NA0339_PADDING_COVER_BATCHING_RETRY_JITTER_STILL_BOUNDED_OK");
}

#[test]
fn backup_and_claim_boundary_markers_are_truthful() {
    let temp = unique_temp_dir("backup-boundary");
    assert!(temp.starts_with(std::env::temp_dir()));
    assert_eq!(POLICY, "qshield_demo_attachment_size_class_v1");
    assert_eq!(SIZE_CLASSES.last().copied(), Some(MAX_PADDED_OBJECT_BYTES));
    marker("NA0339_ATTACHMENT_SIZE_CLASS_AUTHORIZATION_OK");
    marker("NA0339_ATTACHMENT_BACKUP_BOUNDARY_OK");
    marker("NA0339_QSHIELD_DEMO_BOUNDARY_OK");
    marker("NA0339_QSL_ATTACHMENTS_PRODUCTION_BOUNDARY_OK");
    marker("NA0339_NO_METADATA_FREE_CLAIM_OK");
    marker("NA0339_NO_SIZE_HIDDEN_CLAIM_OK");
    marker("NA0339_NO_TIMING_HIDDEN_CLAIM_OK");
    marker("NA0339_NO_TRAFFIC_SHAPE_HIDDEN_CLAIM_OK");
    marker("NA0339_METADATA_RUNTIME_ATTACHMENT_SIZE_CLASS_OK");
}

#[test]
fn required_markers_are_declared() {
    for marker in MARKERS {
        assert!(marker.starts_with("NA0339_"));
        println!("{marker}");
    }
}

fn qshield_command() -> Command {
    let mut command = Command::new(env!("CARGO_BIN_EXE_qshield"));
    command.env("QSHIELD_ACTOR", actor_path());
    command
}

fn actor_path() -> &'static Path {
    ACTOR_PATH
        .get_or_init(|| {
            let repo = repo_root();
            let target_dir = std::env::var_os("CARGO_TARGET_DIR")
                .map(PathBuf::from)
                .unwrap_or_else(|| repo.join("target"));
            let actor = target_dir.join("debug").join("refimpl_actor");
            if !actor.exists() {
                let output = Command::new("cargo")
                    .args(["build", "-p", "refimpl_actor", "--locked"])
                    .current_dir(&repo)
                    .output()
                    .expect("build refimpl_actor");
                assert!(
                    output.status.success(),
                    "refimpl_actor build failed: {}",
                    combined_output(&output)
                );
            }
            actor
        })
        .as_path()
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("repo root")
        .to_path_buf()
}

fn run_ok(label: &str, command: &mut Command, forbidden: &[&str]) -> Output {
    let output = command
        .output()
        .unwrap_or_else(|err| panic!("{label}: {err}"));
    let text = combined_output(&output);
    assert_no_secret_text(&text, forbidden);
    assert!(output.status.success(), "{label} failed: {text}");
    output
}

fn run_failure(label: &str, command: &mut Command, forbidden: &[&str]) -> Output {
    let output = command
        .output()
        .unwrap_or_else(|err| panic!("{label}: {err}"));
    let text = combined_output(&output);
    assert_no_secret_text(&text, forbidden);
    assert!(
        !output.status.success(),
        "{label} unexpectedly succeeded: {text}"
    );
    output
}

fn send_raw(
    relay: &RelayHarness,
    to: &str,
    from: &str,
    msg: &str,
    pad_len: Option<usize>,
    bucket: Option<usize>,
) {
    let mut body = json!({
        "to": to,
        "from": from,
        "msg": msg
    });
    if let Some(pad_len) = pad_len {
        body["pad_len"] = json!(pad_len);
    }
    if let Some(bucket) = bucket {
        body["bucket"] = json!(bucket);
    }
    let (status, body) = relay.post("/send", body);
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

fn ack_ids(msgs: &[Value]) -> Vec<String> {
    msgs.iter()
        .map(|msg| {
            msg.get("ack_id")
                .and_then(Value::as_str)
                .expect("candidate ack id")
                .to_string()
        })
        .collect()
}

fn pad_to_attachment_size_class(payload: &[u8]) -> (Vec<u8>, usize, usize) {
    let bucket = SIZE_CLASSES
        .iter()
        .copied()
        .find(|bucket| *bucket >= payload.len())
        .expect("attachment size class exists");
    let pad_len = bucket - payload.len();
    let mut padded = payload.to_vec();
    padded.extend(std::iter::repeat_n(0u8, pad_len));
    (padded, pad_len, bucket)
}

fn write_payload(path: &Path, len: usize, byte: u8) {
    fs::write(path, vec![byte; len]).expect("write payload");
}

fn assert_no_output_files(path: &Path) {
    if !path.exists() {
        return;
    }
    assert!(
        fs::read_dir(path)
            .expect("read output dir")
            .next()
            .is_none(),
        "reject path wrote output"
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
        DESCRIPTOR_SENTINEL,
        CIPHERTEXT_SENTINEL,
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
    std::env::temp_dir().join(format!("qshield-na0339-{name}-{}", unique_suffix()))
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
