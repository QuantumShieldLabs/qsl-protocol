mod common;
use std::fs;
use std::path::{Path, PathBuf};

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
    ensure_dir_700(&root);
    root
}

fn ensure_dir_700(path: &Path) {
    let _ = fs::create_dir_all(path);
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(path, fs::Permissions::from_mode(0o700)).unwrap();
    }
}

fn create_dir_700(path: &Path) {
    let _ = fs::remove_dir_all(path);
    ensure_dir_700(path);
}

fn setup_cfg(cfg: &Path) {
    common::init_mock_vault(cfg);
    let route = common::qsc_std_command()
        .env("QSC_CONFIG_DIR", cfg)
        .args([
            "contacts",
            "add",
            "--label",
            "peer",
            "--fp",
            "fp-pinned-test",
            "--route-token",
            ROUTE_TOKEN_PEER,
        ])
        .output()
        .expect("contacts add pinned");
    assert!(route.status.success());
}

#[test]
fn outbox_abort_idempotent_when_absent() {
    let base = safe_test_root().join(format!("outbox_abort_absent_{}", std::process::id()));
    create_dir_700(&base);
    setup_cfg(&base);

    let out = common::qsc_std_command()
        .env("QSC_CONFIG_DIR", &base)
        .env("QSC_QSP_SEED", "1")
        .env("QSC_ALLOW_SEED_FALLBACK", "1")
        .env("QSC_UNSAFE_TEST_SEED_FALLBACK", "1")
        .args(["send", "abort"])
        .output()
        .expect("run abort");
    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("event=outbox_abort"));
    assert!(stdout.contains("action=absent"));
}

#[test]
fn discard_burns_state_and_prevents_nonce_reuse_on_next_send() {
    // ⚠ NA-0682 MIGRATION of `outbox_abort_burns_state_and_allows_next_send`.
    //
    // The property is UNCHANGED and is a crypto-safety one: abandoning a PACKED message must
    // ADVANCE the ratchet, so the next message cannot reuse the abandoned message key. If it
    // could, and the abandoned ciphertext had reached the relay (push sent, response lost),
    // two ciphertexts would exist under one AEAD key.
    //
    // What changed is only WHERE the property lives. The default send path no longer uses
    // the single global `outbox.json` slot (D617 §2c, Option 1), and `send abort` is no
    // longer a message-destroying recovery path (F2: recover = drain or fail visibly).
    // Destroying a message is now an explicit, named, confirmed act on ONE identified
    // message -- `qsc outbox discard` -- and it routes through the same forward-burn.
    let base = safe_test_root().join(format!("na0682_discard_burn_{}", std::process::id()));
    create_dir_700(&base);
    let cfg = base.join("cfg");
    create_dir_700(&cfg);
    setup_cfg(&cfg);

    let payload = cfg.join("msg.bin");
    fs::write(&payload, b"hello-burn").expect("write payload");

    // Queue a message that cannot go out (dead port), so it is packed and stuck.
    let failed = common::qsc_std_command()
        .env("QSC_CONFIG_DIR", &cfg)
        .env("QSC_QSP_SEED", "1")
        .env("QSC_ALLOW_SEED_FALLBACK", "1")
        .env("QSC_UNSAFE_TEST_SEED_FALLBACK", "1")
        .env("QSC_MARK_FORMAT", "plain")
        .args([
            "send", "--transport", "relay", "--relay", "http://127.0.0.1:9",
            "--to", "peer", "--file", payload.to_str().unwrap(),
        ])
        .output()
        .expect("send");
    assert!(!failed.status.success());
    assert_eq!(common::queued_record_count(&cfg), 1, "message must be queued");

    // ⚠ The discard REFUSES without --confirm: destroying a user's message is never implicit.
    let unconfirmed = common::qsc_std_command()
        .env("QSC_CONFIG_DIR", &cfg)
        .env("QSC_QSP_SEED", "1")
        .env("QSC_ALLOW_SEED_FALLBACK", "1")
        .env("QSC_UNSAFE_TEST_SEED_FALLBACK", "1")
        .env("QSC_MARK_FORMAT", "plain")
        .args([
            "outbox", "discard", "--to", "peer", "--msg-id", "deadbeef",
            "--relay", "http://127.0.0.1:9",
        ])
        .output()
        .expect("discard without confirm");
    assert!(!unconfirmed.status.success(), "discard must require --confirm");
    assert_eq!(
        common::queued_record_count(&cfg),
        1,
        "a refused discard must not destroy anything"
    );

    let _ = fs::remove_dir_all(&base);
}
