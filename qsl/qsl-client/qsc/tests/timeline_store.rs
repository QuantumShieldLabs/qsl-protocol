mod common;

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

fn ensure_dir_700(path: &Path) {
    let _ = fs::create_dir_all(path);
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let _ = fs::set_permissions(path, fs::Permissions::from_mode(0o700));
    }
}

fn create_dir_700(path: &Path) {
    let _ = fs::remove_dir_all(path);
    ensure_dir_700(path);
}

fn output_text(out: &std::process::Output) -> String {
    let mut s = String::from_utf8_lossy(&out.stdout).to_string();
    s.push_str(&String::from_utf8_lossy(&out.stderr));
    s
}

fn qsc_base(cfg: &Path) -> Command {
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("qsc"));
    cmd.env("QSC_CONFIG_DIR", cfg)
        .env("QSC_MARK_FORMAT", "plain")
        .env("QSC_QSP_SEED", "1")
        .env("QSC_ALLOW_SEED_FALLBACK", "1");
    cmd
}

#[test]
fn timeline_written_on_receive_success() {
    let server = common::start_inbox_server(1024 * 1024, 16);
    let base = safe_test_root().join(format!("na0117_recv_ok_{}", std::process::id()));
    create_dir_700(&base);
    let alice_cfg = base.join("alice_cfg");
    let bob_cfg = base.join("bob_cfg");
    let bob_out = base.join("bob_out");
    create_dir_700(&alice_cfg);
    create_dir_700(&bob_cfg);
    create_dir_700(&bob_out);
    common::init_mock_vault(&alice_cfg);
    common::init_mock_vault(&bob_cfg);

    let payload = base.join("msg.bin");
    fs::write(&payload, b"timeline-recv-ok").unwrap();

    let send = qsc_base(&alice_cfg)
        .args([
            "send",
            "--transport",
            "relay",
            "--relay",
            server.base_url(),
            "--to",
            "bob",
            "--file",
            payload.to_str().unwrap(),
        ])
        .output()
        .expect("send");
    assert!(send.status.success(), "{}", output_text(&send));

    let recv = qsc_base(&bob_cfg)
        .args([
            "receive",
            "--transport",
            "relay",
            "--relay",
            server.base_url(),
            "--mailbox",
            "bob",
            "--from",
            "bob",
            "--max",
            "1",
            "--out",
            bob_out.to_str().unwrap(),
        ])
        .output()
        .expect("receive");
    assert!(recv.status.success(), "{}", output_text(&recv));
    assert!(bob_out.join("recv_1.bin").exists());

    let list = qsc_base(&bob_cfg)
        .args(["timeline", "list", "--peer", "bob", "--limit", "10"])
        .output()
        .expect("timeline list");
    assert!(list.status.success(), "{}", output_text(&list));
    let text = output_text(&list);
    assert!(
        text.contains("event=timeline_list count=1 peer=bob"),
        "{}",
        text
    );
    assert!(text.contains("event=timeline_item"), "{}", text);
    assert!(text.contains("dir=in"), "{}", text);
    assert!(text.contains("kind=msg"), "{}", text);
    assert!(text.contains("ts="), "{}", text);
}

#[test]
fn timeline_not_written_on_receive_reject_no_mutation() {
    let server = common::start_inbox_server(1024 * 1024, 16);
    let base = safe_test_root().join(format!("na0117_recv_reject_{}", std::process::id()));
    create_dir_700(&base);
    let cfg = base.join("cfg");
    let out = base.join("out");
    create_dir_700(&cfg);
    create_dir_700(&out);
    common::init_mock_vault(&cfg);

    server.enqueue_raw("bob", vec![1, 2, 3, 4, 5, 6, 7]);

    let before = qsc_base(&cfg)
        .args(["timeline", "list", "--peer", "bob", "--limit", "10"])
        .output()
        .expect("timeline list before");
    assert!(before.status.success(), "{}", output_text(&before));
    assert!(output_text(&before).contains("event=timeline_list count=0 peer=bob"));

    let recv = qsc_base(&cfg)
        .args([
            "receive",
            "--transport",
            "relay",
            "--relay",
            server.base_url(),
            "--mailbox",
            "bob",
            "--from",
            "bob",
            "--max",
            "1",
            "--out",
            out.to_str().unwrap(),
        ])
        .output()
        .expect("receive reject");
    assert!(!recv.status.success(), "receive reject must fail");

    let after = qsc_base(&cfg)
        .args(["timeline", "list", "--peer", "bob", "--limit", "10"])
        .output()
        .expect("timeline list after");
    assert!(after.status.success(), "{}", output_text(&after));
    assert!(output_text(&after).contains("event=timeline_list count=0 peer=bob"));
}

#[test]
fn timeline_written_on_send_commit_only() {
    let server = common::start_inbox_server(1024 * 1024, 16);
    let base = safe_test_root().join(format!("na0117_send_commit_{}", std::process::id()));
    create_dir_700(&base);
    let cfg = base.join("cfg");
    create_dir_700(&cfg);
    common::init_mock_vault(&cfg);

    let payload = base.join("msg.bin");
    fs::write(&payload, b"timeline-send-commit").unwrap();

    let ok_send = qsc_base(&cfg)
        .args([
            "send",
            "--transport",
            "relay",
            "--relay",
            server.base_url(),
            "--to",
            "bob",
            "--file",
            payload.to_str().unwrap(),
        ])
        .output()
        .expect("send ok");
    assert!(ok_send.status.success(), "{}", output_text(&ok_send));

    let list1 = qsc_base(&cfg)
        .args(["timeline", "list", "--peer", "bob", "--limit", "10"])
        .output()
        .expect("timeline after ok send");
    let t1 = output_text(&list1);
    assert!(list1.status.success(), "{}", t1);
    assert!(
        t1.contains("event=timeline_list count=1 peer=bob"),
        "{}",
        t1
    );
    assert!(t1.contains("dir=out"), "{}", t1);

    let fail_send = qsc_base(&cfg)
        .args([
            "send",
            "--transport",
            "relay",
            "--relay",
            "127.0.0.1:9",
            "--to",
            "bob",
            "--file",
            payload.to_str().unwrap(),
        ])
        .output()
        .expect("send fail");
    assert!(!fail_send.status.success(), "send fail must fail");

    let list2 = qsc_base(&cfg)
        .args(["timeline", "list", "--peer", "bob", "--limit", "10"])
        .output()
        .expect("timeline after fail send");
    let t2 = output_text(&list2);
    assert!(list2.status.success(), "{}", t2);
    assert!(
        t2.contains("event=timeline_list count=1 peer=bob"),
        "{}",
        t2
    );
}

#[test]
fn timeline_is_encrypted_at_rest() {
    let server = common::start_inbox_server(1024 * 1024, 16);
    let base = safe_test_root().join(format!("na0117_timeline_at_rest_{}", std::process::id()));
    create_dir_700(&base);
    let cfg = base.join("cfg");
    create_dir_700(&cfg);
    common::init_mock_vault(&cfg);

    let payload = base.join("msg.bin");
    fs::write(&payload, b"NA0117_TIMELINE_PLAINTEXT_SENTINEL").unwrap();

    let send = qsc_base(&cfg)
        .args([
            "send",
            "--transport",
            "relay",
            "--relay",
            server.base_url(),
            "--to",
            "bob",
            "--file",
            payload.to_str().unwrap(),
        ])
        .output()
        .expect("send");
    assert!(send.status.success(), "{}", output_text(&send));

    let timeline_plain = cfg.join("timeline.json");
    assert!(
        !timeline_plain.exists(),
        "timeline plaintext file must not exist"
    );

    let vault_blob = cfg.join("vault.qsv");
    assert!(vault_blob.exists(), "vault blob missing");
    let bytes = fs::read(&vault_blob).unwrap();
    assert!(
        !bytes
            .windows(b"NA0117_TIMELINE_PLAINTEXT_SENTINEL".len())
            .any(|w| w == b"NA0117_TIMELINE_PLAINTEXT_SENTINEL"),
        "plaintext leaked into vault blob"
    );

    let list = qsc_base(&cfg)
        .args(["timeline", "list", "--peer", "bob", "--limit", "10"])
        .output()
        .expect("timeline list");
    assert!(list.status.success(), "{}", output_text(&list));
    assert!(output_text(&list).contains("event=timeline_list count=1 peer=bob"));
}

#[test]
fn timeline_clear_requires_confirm_no_mutation() {
    let server = common::start_inbox_server(1024 * 1024, 16);
    let base = safe_test_root().join(format!("na0117_timeline_clear_{}", std::process::id()));
    create_dir_700(&base);
    let cfg = base.join("cfg");
    create_dir_700(&cfg);
    common::init_mock_vault(&cfg);

    let payload = base.join("msg.bin");
    fs::write(&payload, b"timeline-clear").unwrap();

    let send = qsc_base(&cfg)
        .args([
            "send",
            "--transport",
            "relay",
            "--relay",
            server.base_url(),
            "--to",
            "bob",
            "--file",
            payload.to_str().unwrap(),
        ])
        .output()
        .expect("send");
    assert!(send.status.success(), "{}", output_text(&send));

    let reject = qsc_base(&cfg)
        .args(["timeline", "clear", "--peer", "bob"])
        .output()
        .expect("timeline clear reject");
    assert!(!reject.status.success(), "clear without confirm must fail");
    assert!(
        output_text(&reject).contains("event=error code=timeline_clear_confirm_required"),
        "{}",
        output_text(&reject)
    );

    let still = qsc_base(&cfg)
        .args(["timeline", "list", "--peer", "bob", "--limit", "10"])
        .output()
        .expect("timeline list still");
    assert!(output_text(&still).contains("event=timeline_list count=1 peer=bob"));

    let clear = qsc_base(&cfg)
        .args(["timeline", "clear", "--peer", "bob", "--confirm"])
        .output()
        .expect("timeline clear");
    assert!(clear.status.success(), "{}", output_text(&clear));
    assert!(
        output_text(&clear).contains("event=timeline_clear ok=true peer=bob removed=1"),
        "{}",
        output_text(&clear)
    );

    let empty = qsc_base(&cfg)
        .args(["timeline", "list", "--peer", "bob", "--limit", "10"])
        .output()
        .expect("timeline list empty");
    assert!(output_text(&empty).contains("event=timeline_list count=0 peer=bob"));
}

#[test]
fn no_secrets_in_timeline_output() {
    let server = common::start_inbox_server(1024 * 1024, 16);
    let base = safe_test_root().join(format!("na0117_timeline_nosecret_{}", std::process::id()));
    create_dir_700(&base);
    let cfg = base.join("cfg");
    create_dir_700(&cfg);
    common::init_mock_vault(&cfg);

    let payload = base.join("msg.bin");
    fs::write(&payload, b"timeline-no-secret").unwrap();
    let send = qsc_base(&cfg)
        .args([
            "send",
            "--transport",
            "relay",
            "--relay",
            server.base_url(),
            "--to",
            "bob",
            "--file",
            payload.to_str().unwrap(),
        ])
        .output()
        .expect("send");
    assert!(send.status.success(), "{}", output_text(&send));

    let out = qsc_base(&cfg)
        .args(["timeline", "list", "--peer", "bob", "--limit", "10"])
        .output()
        .expect("timeline list");
    assert!(out.status.success(), "{}", output_text(&out));
    let text = output_text(&out);
    for pat in [
        "TOKEN",
        "SECRET",
        "KEY",
        "PASS",
        "PRIVATE",
        "BEARER",
        "CREDENTIAL",
        "RELAY_TOKEN",
    ] {
        assert!(!text.contains(pat), "unexpected secret pattern: {pat}");
    }
}
