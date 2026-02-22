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

#[test]
fn relay_push_fail_no_mutation() {
    let base = safe_test_root().join(format!("na0075_relay_push_fail_{}", std::process::id()));
    create_dir_700(&base);
    let cfg = base.join("cfg");
    create_dir_700(&cfg);

    let payload = cfg.join("msg.bin");
    fs::write(&payload, b"dup").expect("write payload");

    let init = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", &cfg)
        .env("QSC_QSP_SEED", "1")
        .env("QSC_ALLOW_SEED_FALLBACK", "1")
        .args(["vault", "init", "--non-interactive", "--key-source", "mock"])
        .output()
        .expect("init vault");
    assert!(
        init.status.success(),
        "{}",
        String::from_utf8_lossy(&init.stdout)
    );
    let add_contact = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", &cfg)
        .env("QSC_QSP_SEED", "1")
        .env("QSC_ALLOW_SEED_FALLBACK", "1")
        .args([
            "contacts",
            "add",
            "--label",
            "peer",
            "--fp",
            "fp-test",
            "--route-token",
            ROUTE_TOKEN_PEER,
        ])
        .output()
        .expect("add contact");
    assert!(
        add_contact.status.success(),
        "{}",
        String::from_utf8_lossy(&add_contact.stdout)
    );

    let outbox = cfg.join("outbox.json");
    let send_state = cfg.join("send.state");

    let output = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", &cfg)
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
        .expect("run relay send");

    assert!(!output.status.success());
    let mut combined = String::from_utf8_lossy(&output.stdout).to_string();
    combined.push_str(&String::from_utf8_lossy(&output.stderr));
    assert!(
        !combined.contains("QSC_ERR_CONTACT_ROUTE_TOKEN_REQUIRED"),
        "route token missing unexpectedly: {combined}"
    );
    assert!(combined.contains("event=send_prepare"));
    assert!(combined.contains("event=relay_event action=push_fail"));
    assert!(combined.contains("event=send_attempt ok=false"));
    assert!(combined.contains("code=relay_inbox_push_failed"));

    assert!(!send_state.exists());
    assert!(outbox.exists());
}
