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

fn send_once(cfg_dir: &Path, seed: &str) -> String {
    let payload_path = cfg_dir.join("msg.bin");
    fs::write(&payload_path, b"drop_reorder").expect("write payload");

    let output = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", cfg_dir)
        .env("QSC_QSP_SEED", "1")
        .env("QSC_ALLOW_SEED_FALLBACK", "1")
        .env("QSC_MARK_FORMAT", "plain")
        .env("QSC_SCENARIO", "drop-reorder")
        .env("QSC_SEED", seed)
        .args([
            "relay",
            "send",
            "--to",
            "peer",
            "--file",
            payload_path.to_str().unwrap(),
            "--relay",
            "http://127.0.0.1:9",
        ])
        .output()
        .expect("relay send");

    let mut combined = String::from_utf8_lossy(&output.stdout).to_string();
    combined.push_str(&String::from_utf8_lossy(&output.stderr));
    assert!(!output.status.success());
    combined
}

fn assert_no_implicit_recovery(markers: &str) {
    assert!(!markers.contains("recover"));
    assert!(!markers.contains("resync"));
    assert!(!markers.contains("retry"));
}

#[test]
fn relay_reorder_no_implicit_recovery() {
    let base = safe_test_root().join(format!("na0075_relay_reorder_{}", std::process::id()));
    create_dir_700(&base);
    let cfg = base.join("cfg");
    create_dir_700(&cfg);

    let out = send_once(&cfg, "3");
    assert!(out.contains("event=relay_event action=reorder"));
    assert!(out.contains("event=send_prepare"));
    assert!(out.contains("event=send_attempt ok=false"));
    assert!(out.contains("code=relay_inbox_push_failed"));
    assert_no_implicit_recovery(&out);
    assert!(cfg.join("outbox.json").exists());
    assert!(!cfg.join("send.state").exists());
}

#[test]
fn relay_drop_plus_reorder_no_mutation() {
    let base = safe_test_root().join(format!("na0075_relay_drop_reorder_{}", std::process::id()));
    create_dir_700(&base);

    let drop_cfg = base.join("cfg_drop");
    create_dir_700(&drop_cfg);
    let drop_out = send_once(&drop_cfg, "0");
    assert!(drop_out.contains("event=relay_event action=drop"));
    assert!(drop_out.contains("code=relay_drop_injected"));
    assert!(drop_cfg.join("outbox.json").exists());
    assert!(!drop_cfg.join("send.state").exists());

    let reorder_cfg = base.join("cfg_reorder");
    create_dir_700(&reorder_cfg);
    let reorder_out = send_once(&reorder_cfg, "3");
    assert!(reorder_out.contains("event=relay_event action=reorder"));
    assert!(reorder_out.contains("code=relay_inbox_push_failed"));
    assert!(reorder_cfg.join("outbox.json").exists());
    assert!(!reorder_cfg.join("send.state").exists());
}
