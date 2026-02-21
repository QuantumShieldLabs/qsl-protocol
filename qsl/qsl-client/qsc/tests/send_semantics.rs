mod common;
use assert_cmd::Command as AssertCommand;
use predicates::prelude::*;
use predicates::str::contains;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

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

#[test]
fn send_refuses_without_transport() {
    let base = safe_test_root().join(format!("na0084_send_no_transport_{}", std::process::id()));
    create_dir_700(&base);
    let cfg = base.join("cfg");
    create_dir_700(&cfg);

    let payload = cfg.join("msg.bin");
    fs::write(&payload, b"hello").expect("write payload");

    let mut cmd = AssertCommand::new(assert_cmd::cargo::cargo_bin!("qsc"));
    cmd.env("QSC_CONFIG_DIR", &cfg)
        .env("QSC_QSP_SEED", "1")
        .env("QSC_ALLOW_SEED_FALLBACK", "1")
        .env("QSC_MARK_FORMAT", "plain")
        .args(["send", "--to", "bob", "--file", payload.to_str().unwrap()]);
    cmd.assert().failure().stdout(predicate::eq(
        "QSC_MARK/1 event=error code=send_transport_required\n",
    ));
}

#[test]
fn send_happy_path_local_relay() {
    let base = safe_test_root().join(format!("na0084_send_happy_{}", std::process::id()));
    create_dir_700(&base);
    let cfg = base.join("cfg");
    create_dir_700(&cfg);

    let payload = cfg.join("msg.bin");
    fs::write(&payload, b"hello").expect("write payload");

    let relay = common::start_inbox_server(1024 * 1024, 8);
    let relay_addr = relay.base_url().to_string();

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
            relay_addr.as_str(),
            "--to",
            "peer",
            "--file",
            payload.to_str().unwrap(),
        ])
        .output()
        .expect("run send");

    if !output.status.success() {
        panic!("send failed: {}", combined_output(&output));
    }
    let combined = combined_output(&output);
    assert!(combined.contains("event=send_prepare"));
    assert!(combined.contains("event=send_attempt ok=true"));
    assert!(combined.contains("event=send_commit"));
}

#[test]
fn send_failure_no_commit() {
    let base = safe_test_root().join(format!("na0084_send_fail_{}", std::process::id()));
    create_dir_700(&base);
    let cfg = base.join("cfg");
    create_dir_700(&cfg);

    let payload = cfg.join("msg.bin");
    fs::write(&payload, b"hello").expect("write payload");

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
            "http://127.0.0.1:9",
            "--to",
            "peer",
            "--file",
            payload.to_str().unwrap(),
        ])
        .output()
        .expect("run send fail");

    assert!(!output.status.success(), "send should fail");
    let combined = combined_output(&output);
    assert!(combined.contains("event=relay_event action=push_fail"));
    assert!(combined.contains("event=send_attempt ok=false"));
    assert!(!combined.contains("event=send_commit"));
}

#[test]
fn outbox_recovery_via_send_abort() {
    let base = safe_test_root().join(format!("na0084_outbox_recover_{}", std::process::id()));
    create_dir_700(&base);
    let cfg = base.join("cfg");
    create_dir_700(&cfg);

    let payload = cfg.join("msg.bin");
    fs::write(&payload, b"hello").expect("write payload");

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
            "http://127.0.0.1:9",
            "--to",
            "peer",
            "--file",
            payload.to_str().unwrap(),
        ])
        .output()
        .expect("run send fail");
    assert!(!output.status.success());

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
            "http://127.0.0.1:9",
            "--to",
            "peer",
            "--file",
            payload.to_str().unwrap(),
        ])
        .output()
        .expect("run send outbox exists");
    assert!(!output.status.success());
    let mut combined = String::from_utf8_lossy(&output.stdout).to_string();
    combined.push_str(&String::from_utf8_lossy(&output.stderr));
    assert!(combined.contains("event=error code=outbox_exists"));

    let mut cmd = AssertCommand::new(assert_cmd::cargo::cargo_bin!("qsc"));
    cmd.env("QSC_CONFIG_DIR", &cfg)
        .env("QSC_QSP_SEED", "1")
        .env("QSC_ALLOW_SEED_FALLBACK", "1")
        .env("QSC_MARK_FORMAT", "plain")
        .args(["send", "abort"]);
    cmd.assert()
        .success()
        .stdout(contains("event=outbox_abort"));

    let relay = common::start_inbox_server(1024 * 1024, 8);
    let relay_addr = relay.base_url().to_string();

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
            relay_addr.as_str(),
            "--to",
            "peer",
            "--file",
            payload.to_str().unwrap(),
        ])
        .output()
        .expect("run send after abort");

    if !output.status.success() {
        panic!("send after abort failed: {}", combined_output(&output));
    }
    let combined = combined_output(&output);
    assert!(combined.contains("event=send_prepare"));
    assert!(combined.contains("event=send_attempt ok=true"));
    assert!(combined.contains("event=send_commit"));
}

#[test]
fn send_outputs_have_no_secrets() {
    let dir = safe_test_root().join(format!("na0084_send_no_secrets_{}", std::process::id()));
    create_dir_700(&dir);
    let cfg = dir.join("cfg");
    create_dir_700(&cfg);

    let payload = cfg.join("msg.bin");
    fs::write(&payload, b"hello").expect("write payload");

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
            "http://127.0.0.1:9",
            "--to",
            "peer",
            "--file",
            payload.to_str().unwrap(),
        ])
        .output()
        .expect("run send no secrets");

    let combined = combined_output(&output);
    for needle in [
        "TOKEN",
        "SECRET",
        "KEY",
        "PASS",
        "PRIVATE",
        "BEARER",
        "CREDENTIAL",
    ] {
        assert!(
            !combined.contains(needle),
            "unexpected secret token in output"
        );
    }
}
