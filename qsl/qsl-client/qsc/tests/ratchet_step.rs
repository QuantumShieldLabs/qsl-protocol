mod common;
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

fn send_cmd(cfg: &Path, relay: &str, to: &str, msg: &Path) -> std::process::Output {
    Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", cfg)
        .env("QSC_QSP_SEED", "1")
        .env("QSC_ALLOW_SEED_FALLBACK", "1")
        .env("QSC_MARK_FORMAT", "plain")
        .args([
            "send",
            "--transport",
            "relay",
            "--relay",
            relay,
            "--to",
            to,
            "--file",
            msg.to_str().unwrap(),
        ])
        .output()
        .expect("send")
}

fn receive_cmd(cfg: &Path, relay: &str, from: &str, out: &Path, max: &str) -> std::process::Output {
    Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", cfg)
        .env("QSC_QSP_SEED", "1")
        .env("QSC_ALLOW_SEED_FALLBACK", "1")
        .env("QSC_MARK_FORMAT", "plain")
        .args([
            "receive",
            "--transport",
            "relay",
            "--relay",
            relay,
            "--from",
            from,
            "--max",
            max,
            "--out",
            out.to_str().unwrap(),
        ])
        .output()
        .expect("receive")
}

#[test]
fn ratchet_in_order_advances_and_ciphertext_differs() {
    let server = common::start_inbox_server(1024 * 1024, 64);
    let base = safe_test_root().join(format!("na0096_ratchet_inorder_{}", std::process::id()));
    create_dir_700(&base);
    let cfg = base.join("cfg");
    create_dir_700(&cfg);
    let out_dir = base.join("out");
    create_dir_700(&out_dir);

    let msg = base.join("msg.bin");
    fs::write(&msg, b"same-plaintext").expect("write msg");

    let out1 = send_cmd(&cfg, server.base_url(), "bob", &msg);
    assert!(out1.status.success(), "send 1 failed");
    let out2 = send_cmd(&cfg, server.base_url(), "bob", &msg);
    assert!(out2.status.success(), "send 2 failed");

    let s1 = combined_output(&out1);
    let s2 = combined_output(&out2);
    assert!(s1.contains("event=ratchet_send_advance"));
    assert!(s2.contains("event=ratchet_send_advance"));
    assert!(s1.contains("msg_idx=0"));
    assert!(s2.contains("msg_idx=1"));

    let items = server.drain_channel("bob");
    assert_eq!(items.len(), 2);
    assert_ne!(
        items[0], items[1],
        "ciphertext should differ for same plaintext"
    );
    server.replace_channel("bob", items);

    let recv = receive_cmd(&cfg, server.base_url(), "bob", &out_dir, "2");
    assert!(recv.status.success(), "receive failed");
    let recv_out = combined_output(&recv);
    assert!(recv_out.contains("event=ratchet_recv_advance"));
}

#[test]
fn ratchet_out_of_order_store_and_consume() {
    let server = common::start_inbox_server(1024 * 1024, 64);
    let base = safe_test_root().join(format!("na0096_ratchet_ooo_{}", std::process::id()));
    create_dir_700(&base);
    let cfg = base.join("cfg");
    create_dir_700(&cfg);
    let out_dir = base.join("out");
    create_dir_700(&out_dir);

    let msg1 = base.join("msg1.bin");
    let msg2 = base.join("msg2.bin");
    fs::write(&msg1, b"m1").expect("write msg1");
    fs::write(&msg2, b"m2").expect("write msg2");

    let out1 = send_cmd(&cfg, server.base_url(), "bob", &msg1);
    assert!(out1.status.success(), "send 1 failed");
    let out2 = send_cmd(&cfg, server.base_url(), "bob", &msg2);
    assert!(out2.status.success(), "send 2 failed");

    let mut items = server.drain_channel("bob");
    assert_eq!(items.len(), 2);
    items.reverse();
    server.replace_channel("bob", items);

    let recv1 = receive_cmd(&cfg, server.base_url(), "bob", &out_dir, "1");
    assert!(recv1.status.success(), "receive 1 failed");
    let out_recv1 = combined_output(&recv1);
    assert!(out_recv1.contains("event=ratchet_skip_store"));
    assert!(out_recv1.contains("event=ratchet_recv_advance"));

    let recv2 = receive_cmd(&cfg, server.base_url(), "bob", &out_dir, "1");
    assert!(recv2.status.success(), "receive 2 failed");
    let out_recv2 = combined_output(&recv2);
    assert!(out_recv2.contains("event=ratchet_recv_advance"));
}

#[test]
fn ratchet_replay_reject_no_mutation() {
    let server = common::start_inbox_server(1024 * 1024, 64);
    let base = safe_test_root().join(format!("na0096_ratchet_replay_{}", std::process::id()));
    create_dir_700(&base);
    let cfg = base.join("cfg");
    create_dir_700(&cfg);
    let out_dir = base.join("out");
    create_dir_700(&out_dir);

    let msg = base.join("msg.bin");
    fs::write(&msg, b"replay").expect("write msg");

    let send = send_cmd(&cfg, server.base_url(), "bob", &msg);
    assert!(send.status.success(), "send failed");

    let mut items = server.drain_channel("bob");
    assert_eq!(items.len(), 1);
    let ct = items.pop().unwrap();
    server.enqueue_raw("bob", ct.clone());

    let recv1 = receive_cmd(&cfg, server.base_url(), "bob", &out_dir, "1");
    assert!(recv1.status.success(), "receive 1 failed");

    server.enqueue_raw("bob", ct);
    let recv2 = receive_cmd(&cfg, server.base_url(), "bob", &out_dir, "1");
    assert!(!recv2.status.success(), "replay should fail");
    let out2 = combined_output(&recv2);
    assert!(out2.contains("event=ratchet_replay_reject"));
    assert!(
        out2.contains("msg_idx=1"),
        "replay marker missing msg_idx: {out2}"
    );
    assert!(!out2.contains("event=recv_item"));
}

#[test]
fn ratchet_tamper_reject_no_mutation() {
    let server = common::start_inbox_server(1024 * 1024, 64);
    let base = safe_test_root().join(format!("na0096_ratchet_tamper_{}", std::process::id()));
    create_dir_700(&base);
    let cfg = base.join("cfg");
    create_dir_700(&cfg);
    let out_dir = base.join("out");
    create_dir_700(&out_dir);

    let msg = base.join("msg.bin");
    fs::write(&msg, b"tamper").expect("write msg");

    let send = send_cmd(&cfg, server.base_url(), "bob", &msg);
    assert!(send.status.success(), "send failed");

    let mut items = server.drain_channel("bob");
    assert_eq!(items.len(), 1);
    let mut ct = items.pop().unwrap();
    if let Some(first) = ct.get_mut(0) {
        *first ^= 0x01;
    }
    server.enqueue_raw("bob", ct);

    let recv = receive_cmd(&cfg, server.base_url(), "bob", &out_dir, "1");
    assert!(!recv.status.success(), "tamper should fail");
    let out = combined_output(&recv);
    assert!(!out.contains("event=recv_item"));
}

#[test]
fn ratchet_skip_cap_eviction_deterministic() {
    let server = common::start_inbox_server(1024 * 1024, 256);
    let base = safe_test_root().join(format!("na0096_ratchet_cap_{}", std::process::id()));
    create_dir_700(&base);
    let cfg = base.join("cfg");
    create_dir_700(&cfg);
    let out_dir = base.join("out");
    create_dir_700(&out_dir);

    let msg = base.join("msg.bin");
    fs::write(&msg, b"cap").expect("write msg");

    for _ in 0..6 {
        let send = send_cmd(&cfg, server.base_url(), "bob", &msg);
        assert!(send.status.success(), "send failed");
    }

    let mut items = server.drain_channel("bob");
    items.reverse();
    server.replace_channel("bob", items);

    let recv = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", &cfg)
        .env("QSC_QSP_SEED", "1")
        .env("QSC_ALLOW_SEED_FALLBACK", "1")
        .env("QSC_MARK_FORMAT", "plain")
        .env("QSC_MKSKIPPED_CAP", "3")
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
            out_dir.to_str().unwrap(),
        ])
        .output()
        .expect("receive");
    assert!(recv.status.success(), "receive failed");
    let out = combined_output(&recv);
    assert!(out.contains("event=ratchet_skip_evict"));

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
        assert!(!out.contains(pat), "secret pattern in output");
    }
}
