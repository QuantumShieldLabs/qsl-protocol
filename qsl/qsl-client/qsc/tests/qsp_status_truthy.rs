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
fn status_seeded_is_active() {
    let base = safe_test_root().join(format!("na0093_status_active_{}", std::process::id()));
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
    assert!(combined.contains("event=qsp_status status=ACTIVE reason=active"));
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
