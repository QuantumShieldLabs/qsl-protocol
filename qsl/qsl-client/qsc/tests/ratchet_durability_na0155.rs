mod common;
use serde::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const ROUTE_TOKEN_PEER: &str = "route_token_peer_abcdefghijklmnopq";

#[derive(Deserialize)]
struct OutboxRecord {
    ciphertext: Vec<u8>,
}

fn safe_test_root() -> PathBuf {
    let root = if let Ok(v) = std::env::var("QSC_TEST_ROOT") {
        PathBuf::from(v)
    } else if let Ok(v) = std::env::var("CARGO_TARGET_DIR") {
        PathBuf::from(v)
    } else {
        PathBuf::from("target")
    };
    let root = root.join("qsc-test-tmp");
    create_dir_700(&root);
    root
}

fn create_dir_700(path: &Path) {
    let _ = fs::remove_dir_all(path);
    fs::create_dir_all(path).unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(path, fs::Permissions::from_mode(0o700)).unwrap();
    }
}

fn setup_cfg(cfg: &Path) {
    common::init_mock_vault(cfg);
    let route = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", cfg)
        .args([
            "contacts",
            "route-set",
            "--label",
            "peer",
            "--route-token",
            ROUTE_TOKEN_PEER,
        ])
        .output()
        .expect("contacts route set");
    assert!(route.status.success());
}

fn run_send(cfg: &Path, relay: &str, file: &Path) -> std::process::Output {
    Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", cfg)
        .env("QSC_QSP_SEED", "1")
        .env("QSC_ALLOW_SEED_FALLBACK", "1")
        .env("QSC_MARK_FORMAT", "plain")
        .args([
            "send",
            "--transport",
            "relay",
            "--relay",
            relay,
            "--to",
            "peer",
            "--file",
            file.to_str().unwrap(),
        ])
        .output()
        .expect("run send")
}

fn combined(out: &std::process::Output) -> String {
    let mut s = String::from_utf8_lossy(&out.stdout).to_string();
    s.push_str(&String::from_utf8_lossy(&out.stderr));
    s
}

fn read_outbox_ciphertext(cfg: &Path) -> Vec<u8> {
    let bytes = fs::read(cfg.join("outbox.json")).expect("read outbox");
    let outbox: OutboxRecord = serde_json::from_slice(&bytes).expect("parse outbox");
    outbox.ciphertext
}

#[test]
fn retry_resends_identical_ciphertext_no_reencrypt() {
    let base = safe_test_root().join(format!("na0155_retry_identical_{}", std::process::id()));
    create_dir_700(&base);
    setup_cfg(&base);
    let payload = base.join("msg.bin");
    fs::write(&payload, b"hello").expect("write payload");

    let first = run_send(&base, "http://127.0.0.1:9", &payload);
    assert!(!first.status.success());
    let expected_ct = read_outbox_ciphertext(&base);

    let relay = common::start_inbox_server(1024 * 1024, 8);
    let second = run_send(&base, relay.base_url(), &payload);
    assert!(second.status.success(), "{}", combined(&second));
    let out = combined(&second);
    assert!(out.contains("event=send_retry mode=outbox_replay"));
    assert!(!out.contains("event=qsp_pack"));
    assert!(out.contains("event=send_commit"));

    let delivered = relay.drain_channel(ROUTE_TOKEN_PEER);
    assert_eq!(delivered.len(), 1);
    assert_eq!(delivered[0], expected_ct);
    assert!(!base.join("outbox.json").exists());
}

#[test]
fn crash_recovery_sends_from_outbox_not_recomputed_payload() {
    let base = safe_test_root().join(format!("na0155_crash_recovery_{}", std::process::id()));
    create_dir_700(&base);
    setup_cfg(&base);
    let payload_a = base.join("msg_a.bin");
    fs::write(&payload_a, b"hello-a").expect("write payload a");
    let payload_b = base.join("msg_b.bin");
    fs::write(&payload_b, b"hello-b-new").expect("write payload b");

    let first = run_send(&base, "http://127.0.0.1:9", &payload_a);
    assert!(!first.status.success());
    let expected_ct = read_outbox_ciphertext(&base);

    // New process invocation simulates restart; replay must ignore new plaintext input.
    let relay = common::start_inbox_server(1024 * 1024, 8);
    let second = run_send(&base, relay.base_url(), &payload_b);
    assert!(second.status.success(), "{}", combined(&second));
    let out = combined(&second);
    assert!(out.contains("event=send_retry mode=outbox_replay"));
    assert!(out.contains("event=send_commit"));
    let delivered = relay.drain_channel(ROUTE_TOKEN_PEER);
    assert_eq!(delivered.len(), 1);
    assert_eq!(delivered[0], expected_ct);
    assert!(!base.join("outbox.json").exists());
}

#[test]
fn abort_burns_state_and_prevents_nonce_reuse_on_next_send() {
    let base = safe_test_root().join(format!("na0155_abort_burns_{}", std::process::id()));
    create_dir_700(&base);
    setup_cfg(&base);
    let payload = base.join("msg.bin");
    fs::write(&payload, b"hello-burn").expect("write payload");

    let first = run_send(&base, "http://127.0.0.1:9", &payload);
    assert!(!first.status.success());
    let first_ct = read_outbox_ciphertext(&base);

    let abort = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", &base)
        .env("QSC_QSP_SEED", "1")
        .env("QSC_ALLOW_SEED_FALLBACK", "1")
        .args(["send", "abort"])
        .output()
        .expect("send abort");
    assert!(abort.status.success());
    let abort_out = combined(&abort);
    assert!(abort_out.contains("event=outbox_abort"));
    assert!(abort_out.contains("action=burned"));
    assert!(!base.join("outbox.json").exists());

    let relay = common::start_inbox_server(1024 * 1024, 8);
    let second = run_send(&base, relay.base_url(), &payload);
    assert!(second.status.success(), "{}", combined(&second));
    let delivered = relay.drain_channel(ROUTE_TOKEN_PEER);
    assert_eq!(delivered.len(), 1);
    assert_ne!(delivered[0], first_ct);
}
