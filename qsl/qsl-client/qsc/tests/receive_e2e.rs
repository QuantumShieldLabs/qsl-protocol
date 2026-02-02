mod common;

use assert_cmd::Command as AssertCommand;
use predicates::str::contains;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

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

#[test]
fn receive_two_way_e2e_local_inbox() {
    let server = common::start_inbox_server(1024 * 1024, 32);
    let base = safe_test_root().join(format!("na0091_recv_e2e_{}", std::process::id()));
    create_dir_700(&base);

    let alice_cfg = base.join("alice_cfg");
    let bob_cfg = base.join("bob_cfg");
    create_dir_700(&alice_cfg);
    create_dir_700(&bob_cfg);

    let alice_out = base.join("alice_out");
    let bob_out = base.join("bob_out");
    create_dir_700(&alice_out);
    create_dir_700(&bob_out);

    let msg_a = base.join("msg_a.bin");
    let msg_b = base.join("msg_b.bin");
    fs::write(&msg_a, b"hello-bob").expect("write msg_a");
    fs::write(&msg_b, b"hello-alice").expect("write msg_b");

    let output_a = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", &alice_cfg)
        .env("QSC_MARK_FORMAT", "plain")
        .args([
            "send",
            "--transport",
            "relay",
            "--relay",
            server.base_url(),
            "--to",
            "bob",
            "--file",
            msg_a.to_str().unwrap(),
        ])
        .output()
        .expect("send a");
    assert!(output_a.status.success(), "send a failed");

    let output_b = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", &bob_cfg)
        .env("QSC_MARK_FORMAT", "plain")
        .args([
            "receive",
            "--transport",
            "relay",
            "--relay",
            server.base_url(),
            "--from",
            "bob",
            "--max",
            "1",
            "--out",
            bob_out.to_str().unwrap(),
        ])
        .output()
        .expect("receive b");
    let out_b = combined_output(&output_b);
    assert!(output_b.status.success(), "receive b failed");
    assert!(out_b.contains("event=recv_item"));
    assert!(out_b.contains("event=recv_commit"));

    let bob_file = bob_out.join("recv_1.bin");
    let bob_bytes = fs::read(&bob_file).expect("bob recv file");
    assert_eq!(bob_bytes, b"hello-bob");

    let output_b_send = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", &bob_cfg)
        .env("QSC_MARK_FORMAT", "plain")
        .args([
            "send",
            "--transport",
            "relay",
            "--relay",
            server.base_url(),
            "--to",
            "alice",
            "--file",
            msg_b.to_str().unwrap(),
        ])
        .output()
        .expect("send b");
    assert!(output_b_send.status.success(), "send b failed");

    let output_a_recv = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", &alice_cfg)
        .env("QSC_MARK_FORMAT", "plain")
        .args([
            "receive",
            "--transport",
            "relay",
            "--relay",
            server.base_url(),
            "--from",
            "alice",
            "--max",
            "1",
            "--out",
            alice_out.to_str().unwrap(),
        ])
        .output()
        .expect("receive a");
    let out_a = combined_output(&output_a_recv);
    assert!(output_a_recv.status.success(), "receive a failed");
    assert!(out_a.contains("event=recv_item"));
    assert!(out_a.contains("event=recv_commit"));

    let alice_file = alice_out.join("recv_1.bin");
    let alice_bytes = fs::read(&alice_file).expect("alice recv file");
    assert_eq!(alice_bytes, b"hello-alice");

    let secret_patterns = [
        "TOKEN",
        "SECRET",
        "KEY",
        "PASS",
        "PRIVATE",
        "BEARER",
        "CREDENTIAL",
    ];
    for pat in secret_patterns {
        assert!(!out_b.contains(pat), "secret pattern in receive output");
        assert!(!out_a.contains(pat), "secret pattern in receive output");
    }
}

#[test]
fn tui_receive_headless_marks() {
    let server = common::start_inbox_server(1024 * 1024, 32);
    let base = safe_test_root().join(format!("na0091_tui_recv_{}", std::process::id()));
    create_dir_700(&base);
    let cfg = base.join("cfg");
    create_dir_700(&cfg);
    let msg = base.join("msg.bin");
    fs::write(&msg, b"hello").expect("write msg");

    let output_send = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", &cfg)
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
        .expect("send to peer-0");
    assert!(output_send.status.success(), "send failed");

    let mut cmd = AssertCommand::new(assert_cmd::cargo::cargo_bin!("qsc"));
    cmd.env("QSC_CONFIG_DIR", &cfg)
        .env("QSC_MARK_FORMAT", "plain")
        .env("QSC_TUI_HEADLESS", "1")
        .env("QSC_TUI_SCRIPT", "/receive;/exit")
        .args([
            "tui",
            "--transport",
            "relay",
            "--relay",
            server.base_url(),
            "--seed",
            "7",
            "--scenario",
            "happy-path",
        ]);
    cmd.assert()
        .success()
        .stdout(contains("event=tui_receive"))
        .stdout(contains("event=recv_item"));
}
