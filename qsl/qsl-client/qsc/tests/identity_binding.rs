use assert_cmd::Command;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

mod common;

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

fn session_path(cfg: &Path, peer: &str) -> PathBuf {
    cfg.join("qsp_sessions").join(format!("{}.qsv", peer))
}

fn identity_pin_path(cfg: &Path, peer: &str) -> PathBuf {
    cfg.join("identities").join(format!("peer_{}.fp", peer))
}

fn read_trimmed(path: &Path) -> String {
    let bytes = fs::read(path).unwrap();
    let mut s = String::from_utf8_lossy(&bytes).to_string();
    while s.ends_with('\n') || s.ends_with('\r') {
        s.pop();
    }
    s
}

#[test]
fn tofu_pins_on_first_handshake() {
    let base = safe_test_root().join(format!("na0100_identity_pin_{}", std::process::id()));
    let _ = fs::remove_dir_all(&base);
    ensure_dir_700(&base);
    let alice_cfg = base.join("alice");
    let bob_cfg = base.join("bob");
    ensure_dir_700(&alice_cfg);
    ensure_dir_700(&bob_cfg);
    common::init_mock_vault(&alice_cfg);
    common::init_mock_vault(&bob_cfg);

    let server = common::start_inbox_server(1024 * 1024, 16);
    let relay = server.base_url().to_string();

    let out_init = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", &alice_cfg)
        .args([
            "handshake",
            "init",
            "--as",
            "alice",
            "--peer",
            "bob",
            "--relay",
            &relay,
        ])
        .output()
        .expect("handshake init");
    assert!(out_init.status.success());

    let out_bob = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", &bob_cfg)
        .args([
            "handshake",
            "poll",
            "--as",
            "bob",
            "--peer",
            "alice",
            "--relay",
            &relay,
            "--max",
            "4",
        ])
        .output()
        .expect("handshake poll bob");
    assert!(out_bob.status.success());

    let out_alice = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", &alice_cfg)
        .args([
            "handshake",
            "poll",
            "--as",
            "alice",
            "--peer",
            "bob",
            "--relay",
            &relay,
            "--max",
            "4",
        ])
        .output()
        .expect("handshake poll alice");
    assert!(out_alice.status.success());

    let out_bob_confirm = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", &bob_cfg)
        .args([
            "handshake",
            "poll",
            "--as",
            "bob",
            "--peer",
            "alice",
            "--relay",
            &relay,
            "--max",
            "4",
        ])
        .output()
        .expect("handshake poll bob confirm");
    assert!(out_bob_confirm.status.success());

    let pin_path = identity_pin_path(&bob_cfg, "alice");
    assert!(pin_path.exists());
    let fp = read_trimmed(&pin_path);
    assert!(fp.starts_with("QSCFP-"));

    let mut combined = String::from_utf8_lossy(&out_init.stdout).to_string()
        + &String::from_utf8_lossy(&out_init.stderr);
    combined.push_str(&String::from_utf8_lossy(&out_bob.stdout));
    combined.push_str(&String::from_utf8_lossy(&out_bob.stderr));
    combined.push_str(&String::from_utf8_lossy(&out_alice.stdout));
    combined.push_str(&String::from_utf8_lossy(&out_alice.stderr));
    combined.push_str(&String::from_utf8_lossy(&out_bob_confirm.stdout));
    combined.push_str(&String::from_utf8_lossy(&out_bob_confirm.stderr));
    assert!(combined.contains("identity_pin"));
    assert!(combined.contains("fp=QSCFP-"));

    for pat in [
        "TOKEN",
        "SECRET",
        "KEY",
        "PASS",
        "PRIVATE",
        "BEARER",
        "CREDENTIAL",
    ] {
        assert!(!combined.contains(pat));
    }
}

#[test]
fn tofu_mismatch_rejected_no_mutation() {
    let base = safe_test_root().join(format!("na0100_identity_mismatch_{}", std::process::id()));
    let _ = fs::remove_dir_all(&base);
    ensure_dir_700(&base);
    let alice_cfg = base.join("alice");
    let alice2_cfg = base.join("alice2");
    let bob_cfg = base.join("bob");
    ensure_dir_700(&alice_cfg);
    ensure_dir_700(&alice2_cfg);
    ensure_dir_700(&bob_cfg);
    common::init_mock_vault(&alice_cfg);
    common::init_mock_vault(&alice2_cfg);
    common::init_mock_vault(&bob_cfg);

    let server = common::start_inbox_server(1024 * 1024, 16);
    let relay = server.base_url().to_string();

    let out_init = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", &alice_cfg)
        .args([
            "handshake",
            "init",
            "--as",
            "alice",
            "--peer",
            "bob",
            "--relay",
            &relay,
        ])
        .output()
        .expect("handshake init");
    assert!(out_init.status.success());

    let out_bob = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", &bob_cfg)
        .args([
            "handshake",
            "poll",
            "--as",
            "bob",
            "--peer",
            "alice",
            "--relay",
            &relay,
            "--max",
            "4",
        ])
        .output()
        .expect("handshake poll bob");
    assert!(out_bob.status.success());

    let out_alice = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", &alice_cfg)
        .args([
            "handshake",
            "poll",
            "--as",
            "alice",
            "--peer",
            "bob",
            "--relay",
            &relay,
            "--max",
            "4",
        ])
        .output()
        .expect("handshake poll alice");
    assert!(out_alice.status.success());

    let out_bob_confirm = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", &bob_cfg)
        .args([
            "handshake",
            "poll",
            "--as",
            "bob",
            "--peer",
            "alice",
            "--relay",
            &relay,
            "--max",
            "4",
        ])
        .output()
        .expect("handshake poll bob confirm");
    assert!(out_bob_confirm.status.success());

    let pin_path = identity_pin_path(&bob_cfg, "alice");
    assert!(pin_path.exists());
    let pinned = read_trimmed(&pin_path);

    let session_path = session_path(&bob_cfg, "alice");
    assert!(session_path.exists());
    let session_before = fs::read(&session_path).unwrap();

    let out_init2 = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", &alice2_cfg)
        .args([
            "handshake",
            "init",
            "--as",
            "alice",
            "--peer",
            "bob",
            "--relay",
            &relay,
        ])
        .output()
        .expect("handshake init 2");
    assert!(out_init2.status.success());

    let out_bob2 = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", &bob_cfg)
        .args([
            "handshake",
            "poll",
            "--as",
            "bob",
            "--peer",
            "alice",
            "--relay",
            &relay,
            "--max",
            "4",
        ])
        .output()
        .expect("handshake poll bob mismatch");
    assert!(out_bob2.status.success());

    let pinned_after = read_trimmed(&pin_path);
    assert_eq!(pinned, pinned_after);

    let session_after = fs::read(&session_path).unwrap();
    assert_eq!(session_before, session_after);

    let mut combined = String::from_utf8_lossy(&out_init2.stdout).to_string()
        + &String::from_utf8_lossy(&out_init2.stderr);
    combined.push_str(&String::from_utf8_lossy(&out_bob2.stdout));
    combined.push_str(&String::from_utf8_lossy(&out_bob2.stderr));
    assert!(combined.contains("identity_mismatch"));
    assert!(combined.contains("reason=identity_mismatch"));

    for pat in [
        "TOKEN",
        "SECRET",
        "KEY",
        "PASS",
        "PRIVATE",
        "BEARER",
        "CREDENTIAL",
    ] {
        assert!(!combined.contains(pat));
    }
}
