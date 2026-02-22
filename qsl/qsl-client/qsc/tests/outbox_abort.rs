#[allow(dead_code)]
mod common;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const ROUTE_TOKEN_PEER: &str = "route_token_peer_abcdefghijklmnopq";

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

#[test]
fn outbox_abort_idempotent_when_absent() {
    let base = safe_test_root().join(format!("outbox_abort_absent_{}", std::process::id()));
    create_dir_700(&base);
    setup_cfg(&base);

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
    assert!(stdout.contains("action=absent"));
}

#[test]
fn outbox_abort_burns_state_and_allows_next_send() {
    let base = safe_test_root().join(format!("outbox_abort_burn_{}", std::process::id()));
    create_dir_700(&base);
    setup_cfg(&base);

    let payload = base.join("msg.bin");
    fs::write(&payload, b"hello").expect("write payload");

    let outbox = base.join("outbox.json");
    let send_state = base.join("send.state");

    let failed = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
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
            "http://127.0.0.1:9",
        ])
        .output()
        .expect("run relay send fail");
    assert!(!failed.status.success());
    assert!(outbox.exists());

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
    assert!(stdout.contains("action=burned"));
    assert!(!outbox.exists());
    assert!(send_state.exists());
}
