mod common;
use predicates::prelude::*;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

const ROUTE_TOKEN_PEER: &str = "route_token_peer_abcdefghijklmnopq";

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

fn create_dir_700(path: &Path) {
    let _ = fs::remove_dir_all(path);
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

fn combined_output(output: &std::process::Output) -> String {
    let mut combined = String::from_utf8_lossy(&output.stdout).to_string();
    combined.push_str(&String::from_utf8_lossy(&output.stderr));
    combined
}

fn init_cfg_with_peer_route_token(cfg: &Path) {
    common::init_mock_vault(cfg);
    let out = common::qsc_std_command()
        .env("QSC_CONFIG_DIR", cfg)
        .env("QSC_QSP_SEED", "1")
        .env("QSC_ALLOW_SEED_FALLBACK", "1")
        .env("QSC_UNSAFE_TEST_SEED_FALLBACK", "1")
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
        .expect("contacts add");
    assert!(out.status.success(), "{}", combined_output(&out));
}

#[test]
fn send_refuses_without_transport() {
    let base = safe_test_root().join(format!("na0084_send_no_transport_{}", std::process::id()));
    create_dir_700(&base);
    let cfg = base.join("cfg");
    create_dir_700(&cfg);
    init_cfg_with_peer_route_token(&cfg);

    let payload = cfg.join("msg.bin");
    fs::write(&payload, b"hello").expect("write payload");

    let mut cmd = common::qsc_assert_command();
    cmd.env("QSC_CONFIG_DIR", &cfg)
        .env("QSC_QSP_SEED", "1")
        .env("QSC_ALLOW_SEED_FALLBACK", "1")
        .env("QSC_UNSAFE_TEST_SEED_FALLBACK", "1")
        .env("QSC_MARK_FORMAT", "plain")
        .args(["send", "--to", "bob", "--file", payload.to_str().unwrap()]);
    cmd.assert().failure().stdout(predicate::eq(
        "QSC_MARK/1 event=error code=send_transport_required\n",
    ));
}

#[test]
fn send_happy_path_local_relay() {
    let base = safe_test_root().join(format!("na0084_send_happy_{}", std::process::id()));
    create_dir_700(&base);
    let cfg = base.join("cfg");
    create_dir_700(&cfg);
    init_cfg_with_peer_route_token(&cfg);

    let payload = cfg.join("msg.bin");
    fs::write(&payload, b"hello").expect("write payload");

    let relay = common::start_inbox_server(1024 * 1024, 8);
    let relay_addr = relay.base_url().to_string();

    let output = common::qsc_std_command()
        .env("QSC_CONFIG_DIR", &cfg)
        .env("QSC_QSP_SEED", "1")
        .env("QSC_ALLOW_SEED_FALLBACK", "1")
        .env("QSC_UNSAFE_TEST_SEED_FALLBACK", "1")
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
        ])
        .output()
        .expect("run send");

    if !output.status.success() {
        panic!("send failed: {}", combined_output(&output));
    }
    let combined = combined_output(&output);
    assert!(combined.contains("event=send_prepare"));
    assert!(combined.contains("event=send_attempt ok=true"));
    assert!(combined.contains("event=send_commit"));
}

#[test]
fn send_failure_no_commit() {
    let base = safe_test_root().join(format!("na0084_send_fail_{}", std::process::id()));
    create_dir_700(&base);
    let cfg = base.join("cfg");
    create_dir_700(&cfg);
    init_cfg_with_peer_route_token(&cfg);

    let payload = cfg.join("msg.bin");
    fs::write(&payload, b"hello").expect("write payload");

    let output = common::qsc_std_command()
        .env("QSC_CONFIG_DIR", &cfg)
        .env("QSC_QSP_SEED", "1")
        .env("QSC_ALLOW_SEED_FALLBACK", "1")
        .env("QSC_UNSAFE_TEST_SEED_FALLBACK", "1")
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
        ])
        .output()
        .expect("run send fail");

    assert!(!output.status.success(), "send should fail");
    let combined = combined_output(&output);
    assert!(combined.contains("event=relay_event action=push_fail"));
    assert!(combined.contains("event=send_attempt ok=false"));
    assert!(!combined.contains("event=send_commit"));
}

#[test]
fn a_second_message_while_one_is_stuck_is_queued_not_dropped() {
    // ⚠ NA-0682 (D617 census C4) — THIS TEST REPLACES `outbox_recovery_via_send_abort`,
    // WHICH ASSERTED THE DEFECT AS CORRECT BEHAVIOUR.
    //
    // What the old test pinned: with a message stuck in the single global in-flight slot,
    // a second `qsc send` REPLAYED the first and asserted `!contains("event=qsp_pack")` --
    // i.e. it asserted that the caller's new message was NEVER EVEN PACKED. It was silently
    // dropped, and the only sanctioned recovery (`send abort`) DESTROYED the stuck one.
    // That is a silent loss, and the test made it look intentional.
    //
    // What this test pins instead (F2 + §2b/§2c, operator-ruled): both messages are durably
    // QUEUED before anything is packed or pushed, so neither can be lost; and recovery means
    // DRAIN, not destroy. The old behaviour is now impossible: there is no path that pushes
    // a message the store has not already committed.
    let base = safe_test_root().join(format!("na0682_second_msg_{}", std::process::id()));
    create_dir_700(&base);
    let cfg = base.join("cfg");
    create_dir_700(&cfg);
    init_cfg_with_peer_route_token(&cfg);

    let first = cfg.join("first.bin");
    fs::write(&first, b"first").expect("write first");
    let second = cfg.join("second.bin");
    fs::write(&second, b"second-must-survive").expect("write second");

    // Both sends fail at the network (dead port) -- and both must be queued, not lost.
    for f in [&first, &second] {
        let out = common::qsc_std_command()
            .env("QSC_CONFIG_DIR", &cfg)
            .env("QSC_QSP_SEED", "1")
            .env("QSC_ALLOW_SEED_FALLBACK", "1")
            .env("QSC_UNSAFE_TEST_SEED_FALLBACK", "1")
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
                f.to_str().unwrap(),
            ])
            .output()
            .expect("run send");
        // Honest reporting: SAFE is not SENT, so this still exits non-zero.
        assert!(
            !out.status.success(),
            "a queued-not-sent message must not report success"
        );
    }

    // ⚠ THE POINT: TWO records on disk. Under the old behaviour the second was never packed
    // and never stored -- this count would have been 1.
    assert_eq!(
        common::queued_record_count(&cfg),
        2,
        "the second message was dropped -- the C4 silent loss has returned"
    );

    // Recovery is DRAIN, not destroy: bring the relay up and both go out.
    let relay = common::start_inbox_server(1024 * 1024, 8);
    let out = common::qsc_std_command()
        .env("QSC_CONFIG_DIR", &cfg)
        .env("QSC_QSP_SEED", "1")
        .env("QSC_ALLOW_SEED_FALLBACK", "1")
        .env("QSC_UNSAFE_TEST_SEED_FALLBACK", "1")
        .env("QSC_MARK_FORMAT", "plain")
        .args(["outbox", "retry", "--relay", relay.base_url()])
        .output()
        .expect("outbox retry");
    assert!(out.status.success(), "{}", combined_output(&out));
    let text = combined_output(&out);
    assert!(
        text.contains("event=outbox_drain") && text.contains("sent=2"),
        "both queued messages must drain: {text}"
    );
    assert_eq!(
        relay.drain_channel(ROUTE_TOKEN_PEER).len(),
        2,
        "both messages must reach the relay"
    );

    let _ = fs::remove_dir_all(&base);
}

#[test]
fn send_outputs_have_no_secrets() {
    let dir = safe_test_root().join(format!("na0084_send_no_secrets_{}", std::process::id()));
    create_dir_700(&dir);
    let cfg = dir.join("cfg");
    create_dir_700(&cfg);
    init_cfg_with_peer_route_token(&cfg);

    let payload = cfg.join("msg.bin");
    fs::write(&payload, b"hello").expect("write payload");

    let output = common::qsc_std_command()
        .env("QSC_CONFIG_DIR", &cfg)
        .env("QSC_QSP_SEED", "1")
        .env("QSC_ALLOW_SEED_FALLBACK", "1")
        .env("QSC_UNSAFE_TEST_SEED_FALLBACK", "1")
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
        ])
        .output()
        .expect("run send no secrets");

    let combined = combined_output(&output);
    for needle in [
        "TOKEN",
        "SECRET",
        "KEY",
        "PASS",
        "PRIVATE",
        "BEARER",
        "CREDENTIAL",
    ] {
        assert!(
            !combined.contains(needle),
            "unexpected secret token in output"
        );
    }
}
