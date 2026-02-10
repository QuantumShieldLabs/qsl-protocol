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

fn qsc_cmd() -> assert_cmd::Command {
    assert_cmd::cargo::cargo_bin_cmd!("qsc")
}

#[test]
fn receive_reject_no_mutation() {
    let base = safe_test_root().join(format!("na0074_recv_reject_{}", std::process::id()));
    let cfg = base.join("cfg");
    fs::create_dir_all(&cfg).unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&cfg, fs::Permissions::from_mode(0o700)).unwrap();
    }

    let state = cfg.join("state.bin");
    fs::write(&state, b"state=1").unwrap();
    let before = fs::read(&state).unwrap();

    let bad = base.join("bad.bin");
    fs::write(&bad, b"").unwrap();

    let mut cmd = qsc_cmd();
    cmd.env("QSC_CONFIG_DIR", &cfg)
        .env("QSC_ALLOW_SEED_FALLBACK", "1")
        .env("QSC_MARK_FORMAT", "plain")
        .args(["receive", "--file", bad.to_str().unwrap()]);
    cmd.assert()
        .failure()
        .stdout(predicate::str::contains("event=recv_reject"))
        .stdout(predicate::str::contains("code=recv_reject_parse"));

    let after = fs::read(&state).unwrap();
    assert_eq!(before, after);

    let entries: Vec<_> = fs::read_dir(&cfg).unwrap().collect();
    assert_eq!(entries.len(), 1);
}
