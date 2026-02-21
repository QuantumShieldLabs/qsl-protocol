mod common;
use predicates::prelude::*;
use std::fs;
use std::path::{Path, PathBuf};

/// Create a writable, safe test root without relying on $HOME.
fn safe_test_root() -> PathBuf {
    let root = if let Ok(v) = std::env::var("QSC_TEST_ROOT") {
        PathBuf::from(v)
    } else if let Ok(v) = std::env::var("CARGO_TARGET_DIR") {
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
    fs::create_dir_all(path).unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(path, fs::Permissions::from_mode(0o700)).unwrap();
    }
}

fn ensure_dir_700(path: &Path) {
    fs::create_dir_all(path).unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(path, fs::Permissions::from_mode(0o700)).unwrap();
    }
}

fn qsc_cmd() -> assert_cmd::Command {
    assert_cmd::cargo::cargo_bin_cmd!("qsc")
}

fn read_send_seq(path: &PathBuf) -> u64 {
    let content = fs::read_to_string(path).expect("read send.state");
    let line = content
        .lines()
        .find(|l| l.trim().starts_with("send_seq="))
        .expect("send_seq present");
    line.trim()
        .strip_prefix("send_seq=")
        .unwrap()
        .parse::<u64>()
        .expect("send_seq parse")
}

#[test]
fn send_failure_no_commit() {
    let base = safe_test_root().join(format!("na0070_send_fail_{}", std::process::id()));
    create_dir_700(&base);

    let cfg = base.join("cfg");
    create_dir_700(&cfg);

    let payload = cfg.join("msg.bin");
    fs::write(&payload, b"hello").expect("write payload");

    let outbox = cfg.join("outbox.json");
    let send_state = cfg.join("send.state");
    assert!(!outbox.exists());
    assert!(!send_state.exists());

    let mut cmd = qsc_cmd();
    cmd.env("QSC_CONFIG_DIR", &cfg)
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
        ]);

    cmd.assert()
        .failure()
        .stdout(predicate::str::contains("event=send_prepare"))
        .stdout(predicate::str::contains("event=send_attempt ok=false"))
        .stdout(predicate::str::contains("code=relay_inbox_push_failed"));

    // No mutation on reject: send state must not advance.
    assert!(!send_state.exists());
    assert!(outbox.exists());
}

#[test]
fn outbox_commit_advances_once() {
    let base = safe_test_root().join(format!("na0070_send_commit_{}", std::process::id()));
    create_dir_700(&base);

    let cfg = base.join("cfg");
    create_dir_700(&cfg);

    let payload = cfg.join("msg.bin");
    fs::write(&payload, b"hello").expect("write payload");

    let outbox = cfg.join("outbox.json");
    let send_state = cfg.join("send.state");

    let relay = common::start_inbox_server(1024 * 1024, 8);
    let relay_addr = relay.base_url().to_string();

    let mut cmd = qsc_cmd();
    cmd.env("QSC_CONFIG_DIR", &cfg)
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
        ]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("event=send_commit"));

    assert!(!outbox.exists());
    assert!(send_state.exists());
    assert_eq!(read_send_seq(&send_state), 1);

    let mut cmd = qsc_cmd();
    cmd.env("QSC_CONFIG_DIR", &cfg)
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
        ]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("event=send_commit"));

    assert!(!outbox.exists());
    assert_eq!(read_send_seq(&send_state), 2);
}
