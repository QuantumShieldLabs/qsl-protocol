mod common;

use assert_cmd::Command;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

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

fn run_status(cfg: &Path) -> String {
    let output = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", cfg)
        .env("QSC_MARK_FORMAT", "plain")
        .args(["status"])
        .output()
        .expect("status");
    String::from_utf8_lossy(&output.stdout).to_string() + &String::from_utf8_lossy(&output.stderr)
}

#[test]
fn status_seed_alone_is_inactive() {
    let base = safe_test_root().join(format!("na0105_status_seed_alone_{}", std::process::id()));
    let _ = fs::remove_dir_all(&base);
    ensure_dir_700(&base);
    let cfg = base.join("cfg");
    ensure_dir_700(&cfg);

    let output = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", &cfg)
        .env("QSC_QSP_SEED", "1")
        .env("QSC_MARK_FORMAT", "plain")
        .args(["status"])
        .output()
        .expect("status");
    let combined = String::from_utf8_lossy(&output.stdout).to_string()
        + &String::from_utf8_lossy(&output.stderr);
    assert!(combined.contains("event=qsp_status status=INACTIVE reason=no_session"));
    assert!(!combined.contains("TOKEN"));
    assert!(!combined.contains("SECRET"));
}

#[test]
fn status_missing_seed_reason_missing_seed() {
    let base = safe_test_root().join(format!("na0093_status_missing_seed_{}", std::process::id()));
    let _ = fs::remove_dir_all(&base);
    ensure_dir_700(&base);
    let cfg = base.join("cfg");
    ensure_dir_700(&cfg);

    let combined = run_status(&cfg);
    assert!(combined.contains("event=qsp_status status=INACTIVE reason=missing_seed"));
}

#[test]
fn status_invalid_session_reason_session_invalid() {
    let base = safe_test_root().join(format!(
        "na0105_status_invalid_session_{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&base);
    ensure_dir_700(&base);
    let cfg = base.join("cfg");
    ensure_dir_700(&cfg);
    let sessions = cfg.join("qsp_sessions");
    ensure_dir_700(&sessions);
    fs::write(sessions.join("peer-0.bin"), b"not-a-session").unwrap();

    let combined = run_status(&cfg);
    assert!(combined.contains("event=qsp_status status=INACTIVE reason=session_invalid"));
}

#[test]
fn status_valid_session_reason_handshake() {
    let server = common::start_inbox_server(1024 * 1024, 32);
    let base = safe_test_root().join(format!(
        "na0105_status_valid_session_{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&base);
    ensure_dir_700(&base);
    let cfg = base.join("cfg");
    ensure_dir_700(&cfg);
    common::init_mock_vault(&cfg);
    let msg = base.join("msg.bin");
    fs::write(&msg, b"hello").unwrap();

    let send = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
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
            "peer-0",
            "--file",
            msg.to_str().unwrap(),
        ])
        .output()
        .expect("seed fallback send");
    assert!(send.status.success());

    let combined = run_status(&cfg);
    assert!(combined.contains("event=qsp_status status=ACTIVE reason=handshake"));
}

#[test]
fn status_unsafe_parent_reason_unsafe_parent() {
    let base = safe_test_root().join(format!(
        "na0093_status_unsafe_parent_{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&base);
    ensure_dir_700(&base);
    let cfg = base.join("cfg");
    ensure_dir_700(&cfg);
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&cfg, fs::Permissions::from_mode(0o777)).unwrap();
    }

    let output = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", &cfg)
        .env("QSC_QSP_SEED", "1")
        .env("QSC_ALLOW_SEED_FALLBACK", "1")
        .env("QSC_MARK_FORMAT", "plain")
        .args(["status"])
        .output()
        .expect("status");
    let combined = String::from_utf8_lossy(&output.stdout).to_string()
        + &String::from_utf8_lossy(&output.stderr);
    assert!(combined.contains("event=qsp_status status=INACTIVE reason=unsafe_parent"));
}

#[test]
fn status_no_secrets() {
    let base = safe_test_root().join(format!("na0093_status_no_secrets_{}", std::process::id()));
    let _ = fs::remove_dir_all(&base);
    ensure_dir_700(&base);
    let cfg = base.join("cfg");
    ensure_dir_700(&cfg);

    let output = run_status(&cfg);
    for needle in [
        "TOKEN",
        "SECRET",
        "KEY",
        "PASS",
        "PRIVATE",
        "BEARER",
        "CREDENTIAL",
    ] {
        assert!(!output.contains(needle));
    }
}
