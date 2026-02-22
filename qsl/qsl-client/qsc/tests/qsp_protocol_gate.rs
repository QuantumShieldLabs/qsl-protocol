mod common;

use assert_cmd::Command as AssertCommand;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const ROUTE_TOKEN_BOB: &str = "route_token_bob_abcdefghijklmnopqr";

fn safe_test_root() -> PathBuf {
    let root = if let Ok(v) = env::var("QSC_TEST_ROOT") {
        PathBuf::from(v)
    } else if let Ok(v) = env::var("CARGO_TARGET_DIR") {
        PathBuf::from(v)
    } else {
        PathBuf::from("target")
    };
    let root = root.join("qsc-test-tmp");
    ensure_dir_700(&root);
    root
}

fn create_dir_700(path: &Path) {
    let _ = fs::remove_dir_all(path);
    if let Ok(meta) = fs::symlink_metadata(path) {
        if meta.is_file() {
            let _ = fs::remove_file(path);
        }
    }
    fs::create_dir_all(path).unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(path, fs::Permissions::from_mode(0o700)).unwrap();
    }
}

fn ensure_dir_700(path: &Path) {
    if let Ok(meta) = fs::symlink_metadata(path) {
        if meta.is_file() {
            let _ = fs::remove_file(path);
        }
    }
    fs::create_dir_all(path).unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(path, fs::Permissions::from_mode(0o700)).unwrap();
    }
}

fn combined_output(output: &std::process::Output) -> String {
    let mut combined = String::from_utf8_lossy(&output.stdout).to_string();
    combined.push_str(&String::from_utf8_lossy(&output.stderr));
    combined
}

fn contacts_route_set(cfg: &Path, label: &str, token: &str) {
    let out = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", cfg)
        .args([
            "contacts",
            "route-set",
            "--label",
            label,
            "--route-token",
            token,
        ])
        .output()
        .expect("contacts route set");
    assert!(out.status.success(), "{}", combined_output(&out));
}

#[test]
fn send_refuses_when_protocol_inactive() {
    let base = safe_test_root().join(format!("na0094_send_inactive_{}", std::process::id()));
    create_dir_700(&base);
    let cfg = base.join("cfg");
    create_dir_700(&cfg);
    let msg = base.join("msg.bin");
    fs::write(&msg, b"hello").expect("write msg");

    let output = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", &cfg)
        .env("QSC_ALLOW_SEED_FALLBACK", "1")
        .env("QSC_MARK_FORMAT", "plain")
        .args([
            "send",
            "--transport",
            "relay",
            "--relay",
            "http://127.0.0.1:1",
            "--to",
            "bob",
            "--file",
            msg.to_str().unwrap(),
        ])
        .output()
        .expect("send inactive");
    let out = combined_output(&output);
    assert!(!output.status.success(), "send should fail");
    assert!(
        out.contains("event=error code=protocol_inactive reason=missing_seed"),
        "missing protocol_inactive marker: {}",
        out
    );
    let outbox = cfg.join("outbox.json");
    assert!(!outbox.exists(), "outbox should not be created");
}

#[test]
fn receive_refuses_when_protocol_inactive() {
    let base = safe_test_root().join(format!("na0094_recv_inactive_{}", std::process::id()));
    create_dir_700(&base);
    let cfg = base.join("cfg");
    create_dir_700(&cfg);
    common::init_mock_vault(&cfg);
    let out_dir = base.join("out");
    create_dir_700(&out_dir);

    let output = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", &cfg)
        .env("QSC_ALLOW_SEED_FALLBACK", "1")
        .env("QSC_MARK_FORMAT", "plain")
        .args([
            "receive",
            "--transport",
            "relay",
            "--relay",
            "http://127.0.0.1:1",
            "--from",
            "bob",
            "--max",
            "1",
            "--out",
            out_dir.to_str().unwrap(),
        ])
        .output()
        .expect("receive inactive");
    let out = combined_output(&output);
    assert!(!output.status.success(), "receive should fail");
    assert!(
        out.contains("event=error code=protocol_inactive reason=missing_seed"),
        "missing protocol_inactive marker: {}",
        out
    );
    let entries = fs::read_dir(&out_dir).unwrap().count();
    assert_eq!(entries, 0, "out dir should remain empty");
}

#[test]
fn send_allows_when_protocol_active() {
    let server = common::start_inbox_server(1024 * 1024, 32);
    let base = safe_test_root().join(format!("na0094_send_active_{}", std::process::id()));
    create_dir_700(&base);
    let cfg = base.join("cfg");
    create_dir_700(&cfg);
    common::init_mock_vault(&cfg);
    contacts_route_set(&cfg, "bob", ROUTE_TOKEN_BOB);
    let msg = base.join("msg.bin");
    fs::write(&msg, b"hello").expect("write msg");

    let output = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", &cfg)
        .env("QSC_QSP_SEED", "1")
        .env("QSC_ALLOW_SEED_FALLBACK", "1")
        .env("QSC_MARK_FORMAT", "plain")
        .args([
            "send",
            "--transport",
            "relay",
            "--relay",
            server.base_url(),
            "--to",
            "bob",
            "--file",
            msg.to_str().unwrap(),
        ])
        .output()
        .expect("send active");
    let out = combined_output(&output);
    assert!(output.status.success(), "send should succeed");
    assert!(out.contains("event=qsp_pack ok=true"), "missing qsp_pack");
    assert!(out.contains("event=send_commit"), "missing send_commit");
}

#[test]
fn receive_allows_when_protocol_active() {
    let server = common::start_inbox_server(1024 * 1024, 32);
    let base = safe_test_root().join(format!("na0094_recv_active_{}", std::process::id()));
    create_dir_700(&base);
    let cfg = base.join("cfg");
    create_dir_700(&cfg);
    common::init_mock_vault(&cfg);
    contacts_route_set(&cfg, "bob", ROUTE_TOKEN_BOB);
    let out_dir = base.join("out");
    create_dir_700(&out_dir);
    let msg = base.join("msg.bin");
    fs::write(&msg, b"hello").expect("write msg");

    let output_send = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", &cfg)
        .env("QSC_QSP_SEED", "1")
        .env("QSC_ALLOW_SEED_FALLBACK", "1")
        .env("QSC_MARK_FORMAT", "plain")
        .args([
            "send",
            "--transport",
            "relay",
            "--relay",
            server.base_url(),
            "--to",
            "bob",
            "--file",
            msg.to_str().unwrap(),
        ])
        .output()
        .expect("send for receive");
    assert!(output_send.status.success(), "send for receive failed");

    let output_recv = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", &cfg)
        .env("QSC_QSP_SEED", "1")
        .env("QSC_ALLOW_SEED_FALLBACK", "1")
        .env("QSC_MARK_FORMAT", "plain")
        .args([
            "receive",
            "--transport",
            "relay",
            "--relay",
            server.base_url(),
            "--mailbox",
            ROUTE_TOKEN_BOB,
            "--from",
            "bob",
            "--max",
            "1",
            "--out",
            out_dir.to_str().unwrap(),
        ])
        .output()
        .expect("receive active");
    let out = combined_output(&output_recv);
    assert!(output_recv.status.success(), "receive should succeed");
    assert!(
        out.contains("event=qsp_unpack ok=true"),
        "missing qsp_unpack"
    );
    assert!(out.contains("event=recv_commit"), "missing recv_commit");

    let secret_patterns = [
        "TOKEN",
        "SECRET",
        "KEY",
        "PASS",
        "PRIVATE",
        "BEARER",
        "CREDENTIAL",
    ];
    for pat in secret_patterns {
        assert!(!out.contains(pat), "secret pattern in receive output");
    }
}

#[test]
fn status_output_no_secrets() {
    let base = safe_test_root().join(format!("na0094_status_secrets_{}", std::process::id()));
    create_dir_700(&base);
    let cfg = base.join("cfg");
    create_dir_700(&cfg);

    let mut cmd = AssertCommand::new(assert_cmd::cargo::cargo_bin!("qsc"));
    cmd.env("QSC_CONFIG_DIR", &cfg)
        .env("QSC_MARK_FORMAT", "plain")
        .args(["status"]);
    let output = cmd.output().expect("status output");
    let out = combined_output(&output);
    let secret_patterns = [
        "TOKEN",
        "SECRET",
        "KEY",
        "PASS",
        "PRIVATE",
        "BEARER",
        "CREDENTIAL",
    ];
    for pat in secret_patterns {
        assert!(!out.contains(pat), "secret pattern in status output");
    }
}
