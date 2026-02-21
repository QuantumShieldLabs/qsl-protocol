mod common;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

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

#[test]
fn outbox_abort_idempotent() {
    let base = safe_test_root().join(format!("outbox_abort_{}", std::process::id()));
    create_dir_700(&base);

    let outbox = base.join("outbox.json");
    fs::write(&outbox, br#"{"version":1,"payload_len":5}"#).expect("write outbox");

    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("qsc"));
    cmd.env("QSC_CONFIG_DIR", &base)
        .env("QSC_QSP_SEED", "1")
        .env("QSC_ALLOW_SEED_FALLBACK", "1")
        .args(["send", "abort"]);
    let out = cmd.output().expect("run abort");
    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("event=outbox_abort"));
    assert!(stdout.contains("action=removed"));
    assert!(!outbox.exists());

    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("qsc"));
    cmd.env("QSC_CONFIG_DIR", &base)
        .env("QSC_QSP_SEED", "1")
        .env("QSC_ALLOW_SEED_FALLBACK", "1")
        .args(["send", "abort"]);
    let out = cmd.output().expect("run abort twice");
    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("event=outbox_abort"));
    assert!(stdout.contains("action=absent"));

    let combined =
        String::from_utf8_lossy(&out.stdout).to_string() + &String::from_utf8_lossy(&out.stderr);
    assert!(!combined.contains("TOKEN"));
    assert!(!combined.contains("SECRET"));
    assert!(!combined.contains("PASSWORD"));
}

#[test]
fn outbox_abort_allows_relay_send() {
    let base = safe_test_root().join(format!("outbox_abort_relay_{}", std::process::id()));
    create_dir_700(&base);

    let payload = base.join("msg.bin");
    fs::write(&payload, b"hello").expect("write payload");

    let outbox = base.join("outbox.json");
    fs::write(&outbox, br#"{"version":1,"payload_len":5}"#).expect("write outbox");
    let send_state = base.join("send.state");

    let relay = common::start_inbox_server(1024 * 1024, 8);
    let relay_addr = relay.base_url().to_string();

    let output = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", &base)
        .env("QSC_QSP_SEED", "1")
        .env("QSC_ALLOW_SEED_FALLBACK", "1")
        .env("QSC_MARK_FORMAT", "plain")
        .args([
            "relay",
            "send",
            "--to",
            "peer",
            "--file",
            payload.to_str().unwrap(),
            "--relay",
            relay_addr.as_str(),
        ])
        .output()
        .expect("run relay send (stale outbox)");

    assert!(!output.status.success());
    let mut combined = String::from_utf8_lossy(&output.stdout).to_string();
    combined.push_str(&String::from_utf8_lossy(&output.stderr));
    assert!(combined.contains("code=outbox_exists"));
    assert!(outbox.exists());
    assert!(!send_state.exists());

    let out = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", &base)
        .env("QSC_QSP_SEED", "1")
        .env("QSC_ALLOW_SEED_FALLBACK", "1")
        .args(["send", "abort"])
        .output()
        .expect("run abort");
    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("event=outbox_abort"));
    assert!(stdout.contains("action=removed"));
    assert!(!outbox.exists());

    let output = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", &base)
        .env("QSC_QSP_SEED", "1")
        .env("QSC_ALLOW_SEED_FALLBACK", "1")
        .env("QSC_MARK_FORMAT", "plain")
        .args([
            "relay",
            "send",
            "--to",
            "peer",
            "--file",
            payload.to_str().unwrap(),
            "--relay",
            relay_addr.as_str(),
        ])
        .output()
        .expect("run relay send (after abort)");

    assert!(output.status.success());
    let mut combined = String::from_utf8_lossy(&output.stdout).to_string();
    combined.push_str(&String::from_utf8_lossy(&output.stderr));
    assert!(combined.contains("event=send_prepare"));
    assert!(combined.contains("event=send_commit"));
    assert!(!combined.contains("TOKEN"));
    assert!(!combined.contains("SECRET"));
}
