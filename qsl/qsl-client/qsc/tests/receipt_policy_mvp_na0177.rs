mod common;

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const ROUTE_TOKEN_BOB: &str = "route_token_bob_abcdefghijklmnopqr";

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
    fs::create_dir_all(path).expect("create dir");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(path, fs::Permissions::from_mode(0o700)).expect("chmod 700");
    }
}

fn create_dir_700(path: &Path) {
    let _ = fs::remove_dir_all(path);
    ensure_dir_700(path);
}

fn combined_output(out: &std::process::Output) -> String {
    let mut s = String::from_utf8_lossy(&out.stdout).to_string();
    s.push_str(&String::from_utf8_lossy(&out.stderr));
    s
}

fn leak_counts(s: &str) -> (usize, usize) {
    let v1 = s.match_indices("/v1/").count();
    let mut hex32 = 0usize;
    let bytes = s.as_bytes();
    let mut i = 0usize;
    while i < bytes.len() {
        if bytes[i].is_ascii_hexdigit() {
            let start = i;
            i += 1;
            while i < bytes.len() && bytes[i].is_ascii_hexdigit() {
                i += 1;
            }
            if i - start >= 32 {
                hex32 = hex32.saturating_add(1);
            }
        } else {
            i += 1;
        }
    }
    (v1, hex32)
}

fn assert_no_leaks(s: &str) {
    let (v1, hex32) = leak_counts(s);
    assert_eq!(v1, 0, "found /v1/ in output: {s}");
    assert_eq!(hex32, 0, "found long hex in output: {s}");
}

fn contacts_add_with_route(cfg: &Path, label: &str, token: &str) {
    let out = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", cfg)
        .args([
            "contacts",
            "add",
            "--label",
            label,
            "--fp",
            "fp-pinned-test",
            "--route-token",
            token,
        ])
        .output()
        .expect("contacts add");
    assert!(out.status.success(), "{}", combined_output(&out));
}

fn send_msg(cfg: &Path, relay: &str, file: &Path) -> String {
    let out = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
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
            "bob",
            "--file",
            file.to_str().expect("path"),
            "--receipt",
            "delivered",
        ])
        .output()
        .expect("send msg");
    assert!(out.status.success(), "{}", combined_output(&out));
    combined_output(&out)
}

struct ReceiptPolicyArgs<'a> {
    mode: &'a str,
    batch_window_ms: Option<u64>,
    jitter_ms: Option<u64>,
}

fn recv_with_policy(
    cfg: &Path,
    relay: &str,
    mailbox: &str,
    from: &str,
    out_dir: &Path,
    policy: ReceiptPolicyArgs<'_>,
) -> String {
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("qsc"));
    cmd.env("QSC_CONFIG_DIR", cfg)
        .env("QSC_QSP_SEED", "1")
        .env("QSC_ALLOW_SEED_FALLBACK", "1")
        .env("QSC_MARK_FORMAT", "plain")
        .env("QSC_TUI_TEST_MODE", "1")
        .args([
            "receive",
            "--transport",
            "relay",
            "--relay",
            relay,
            "--mailbox",
            mailbox,
            "--from",
            from,
            "--max",
            "2",
            "--out",
            out_dir.to_str().expect("path"),
            "--receipt-mode",
            policy.mode,
            "--file-confirm-mode",
            "complete-only",
        ]);
    if let Some(ms) = policy.batch_window_ms {
        cmd.args(["--receipt-batch-window-ms", ms.to_string().as_str()]);
    }
    if let Some(ms) = policy.jitter_ms {
        cmd.args(["--receipt-jitter-ms", ms.to_string().as_str()]);
    }
    let out = cmd.output().expect("receive");
    assert!(out.status.success(), "{}", combined_output(&out));
    combined_output(&out)
}

fn recv_default(cfg: &Path, relay: &str, mailbox: &str, from: &str, out_dir: &Path) -> String {
    let out = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
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
            "--mailbox",
            mailbox,
            "--from",
            from,
            "--max",
            "2",
            "--out",
            out_dir.to_str().expect("path"),
        ])
        .output()
        .expect("receive default");
    assert!(out.status.success(), "{}", combined_output(&out));
    combined_output(&out)
}

fn send_file(cfg: &Path, relay: &str, path: &Path) -> String {
    let out = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", cfg)
        .env("QSC_QSP_SEED", "1")
        .env("QSC_ALLOW_SEED_FALLBACK", "1")
        .env("QSC_MARK_FORMAT", "plain")
        .args([
            "file",
            "send",
            "--transport",
            "relay",
            "--relay",
            relay,
            "--to",
            "bob",
            "--path",
            path.to_str().expect("path"),
            "--receipt",
            "delivered",
        ])
        .output()
        .expect("send file");
    assert!(out.status.success(), "{}", combined_output(&out));
    combined_output(&out)
}

#[test]
fn receipt_mode_off_skips_message_and_file_peer_confirmation() {
    let server = common::start_inbox_server(1024 * 1024, 64);
    let base = safe_test_root().join(format!("na0177_receipt_off_{}", std::process::id()));
    create_dir_700(&base);
    let alice_cfg = base.join("alice_cfg");
    let bob_cfg = base.join("bob_cfg");
    let alice_out = base.join("alice_out");
    let bob_out = base.join("bob_out");
    create_dir_700(&alice_cfg);
    create_dir_700(&bob_cfg);
    create_dir_700(&alice_out);
    create_dir_700(&bob_out);
    common::init_mock_vault(&alice_cfg);
    common::init_mock_vault(&bob_cfg);
    contacts_add_with_route(&alice_cfg, "bob", ROUTE_TOKEN_BOB);
    contacts_add_with_route(&bob_cfg, "bob", ROUTE_TOKEN_BOB);

    let msg = base.join("msg.bin");
    fs::write(&msg, b"policy-off-message").expect("write msg");
    let send_msg_out = send_msg(&alice_cfg, server.base_url(), &msg);
    assert!(send_msg_out.contains("QSC_DELIVERY state=accepted_by_relay"));
    assert!(send_msg_out.contains(" peer=bob "));

    let bob_msg = recv_with_policy(
        &bob_cfg,
        server.base_url(),
        ROUTE_TOKEN_BOB,
        "bob",
        &bob_out,
        ReceiptPolicyArgs {
            mode: "off",
            batch_window_ms: None,
            jitter_ms: None,
        },
    );
    assert!(bob_msg.contains("QSC_RECEIPT mode=off status=skipped kind=message peer=bob"));
    assert!(bob_msg.contains("QSC_TUI_RECEIPT mode=off status=skipped kind=message thread=bob"));
    let alice_msg = recv_default(
        &alice_cfg,
        server.base_url(),
        ROUTE_TOKEN_BOB,
        "bob",
        &alice_out,
    );
    assert!(!alice_msg.contains("QSC_DELIVERY state=peer_confirmed"));

    let payload = base.join("file.bin");
    fs::write(&payload, vec![0x3au8; 4096]).expect("write file");
    let send_file_out = send_file(&alice_cfg, server.base_url(), &payload);
    assert!(send_file_out.contains("QSC_FILE_DELIVERY state=awaiting_confirmation"));
    assert!(send_file_out.contains(" peer=bob "));
    let bob_file = recv_with_policy(
        &bob_cfg,
        server.base_url(),
        ROUTE_TOKEN_BOB,
        "bob",
        &bob_out,
        ReceiptPolicyArgs {
            mode: "off",
            batch_window_ms: None,
            jitter_ms: None,
        },
    );
    assert!(bob_file.contains("QSC_RECEIPT mode=off status=skipped kind=file_complete peer=bob"));
    let alice_file = recv_default(
        &alice_cfg,
        server.base_url(),
        ROUTE_TOKEN_BOB,
        "bob",
        &alice_out,
    );
    assert!(!alice_file.contains("QSC_FILE_DELIVERY state=peer_confirmed"));

    let mut all = String::new();
    all.push_str(&send_msg_out);
    all.push_str(&bob_msg);
    all.push_str(&alice_msg);
    all.push_str(&send_file_out);
    all.push_str(&bob_file);
    all.push_str(&alice_file);
    assert_no_leaks(&all);
}

#[test]
fn receipt_mode_immediate_confirms_message_and_file() {
    let server = common::start_inbox_server(1024 * 1024, 64);
    let base = safe_test_root().join(format!("na0177_receipt_immediate_{}", std::process::id()));
    create_dir_700(&base);
    let alice_cfg = base.join("alice_cfg");
    let bob_cfg = base.join("bob_cfg");
    let alice_out = base.join("alice_out");
    let bob_out = base.join("bob_out");
    create_dir_700(&alice_cfg);
    create_dir_700(&bob_cfg);
    create_dir_700(&alice_out);
    create_dir_700(&bob_out);
    common::init_mock_vault(&alice_cfg);
    common::init_mock_vault(&bob_cfg);
    contacts_add_with_route(&alice_cfg, "bob", ROUTE_TOKEN_BOB);
    contacts_add_with_route(&bob_cfg, "bob", ROUTE_TOKEN_BOB);

    let msg = base.join("msg.bin");
    fs::write(&msg, b"policy-immediate-message").expect("write msg");
    let _ = send_msg(&alice_cfg, server.base_url(), &msg);
    let bob_msg = recv_with_policy(
        &bob_cfg,
        server.base_url(),
        ROUTE_TOKEN_BOB,
        "bob",
        &bob_out,
        ReceiptPolicyArgs {
            mode: "immediate",
            batch_window_ms: None,
            jitter_ms: None,
        },
    );
    assert!(bob_msg.contains("QSC_RECEIPT mode=immediate status=sent kind=message peer=bob"));
    assert!(bob_msg.contains("QSC_TUI_RECEIPT mode=immediate status=sent kind=message thread=bob"));
    let alice_msg = recv_default(
        &alice_cfg,
        server.base_url(),
        ROUTE_TOKEN_BOB,
        "bob",
        &alice_out,
    );
    assert!(alice_msg.contains("QSC_DELIVERY state=peer_confirmed"));
    assert!(alice_msg.contains(" peer=bob "));

    let payload = base.join("file.bin");
    fs::write(&payload, vec![0x55u8; 4096]).expect("write file");
    let _ = send_file(&alice_cfg, server.base_url(), &payload);
    let bob_file = recv_with_policy(
        &bob_cfg,
        server.base_url(),
        ROUTE_TOKEN_BOB,
        "bob",
        &bob_out,
        ReceiptPolicyArgs {
            mode: "immediate",
            batch_window_ms: None,
            jitter_ms: None,
        },
    );
    assert!(bob_file.contains("QSC_RECEIPT mode=immediate status=sent kind=file_complete peer=bob"));
    let alice_file = recv_default(
        &alice_cfg,
        server.base_url(),
        ROUTE_TOKEN_BOB,
        "bob",
        &alice_out,
    );
    assert!(alice_file.contains("QSC_FILE_DELIVERY state=peer_confirmed"));
    assert!(alice_file.contains(" peer=bob "));

    let mut all = String::new();
    all.push_str(&bob_msg);
    all.push_str(&alice_msg);
    all.push_str(&bob_file);
    all.push_str(&alice_file);
    assert_no_leaks(&all);
}

#[test]
fn receipt_mode_batched_is_deterministic_and_bounded() {
    let server = common::start_inbox_server(1024 * 1024, 64);
    let base = safe_test_root().join(format!("na0177_receipt_batched_{}", std::process::id()));
    create_dir_700(&base);
    let alice_cfg = base.join("alice_cfg");
    let bob_cfg = base.join("bob_cfg");
    let alice_out = base.join("alice_out");
    let bob_out = base.join("bob_out");
    create_dir_700(&alice_cfg);
    create_dir_700(&bob_cfg);
    create_dir_700(&alice_out);
    create_dir_700(&bob_out);
    common::init_mock_vault(&alice_cfg);
    common::init_mock_vault(&bob_cfg);
    contacts_add_with_route(&alice_cfg, "bob", ROUTE_TOKEN_BOB);
    contacts_add_with_route(&bob_cfg, "bob", ROUTE_TOKEN_BOB);

    let msg1 = base.join("msg1.bin");
    let msg2 = base.join("msg2.bin");
    fs::write(&msg1, b"policy-batched-1").expect("write msg1");
    fs::write(&msg2, b"policy-batched-2").expect("write msg2");
    let _ = send_msg(&alice_cfg, server.base_url(), &msg1);
    let _ = send_msg(&alice_cfg, server.base_url(), &msg2);

    let bob_recv = recv_with_policy(
        &bob_cfg,
        server.base_url(),
        ROUTE_TOKEN_BOB,
        "bob",
        &bob_out,
        ReceiptPolicyArgs {
            mode: "batched",
            batch_window_ms: Some(25),
            jitter_ms: Some(0),
        },
    );
    assert!(bob_recv.contains("QSC_RECEIPT mode=batched status=queued kind=message peer=bob"));
    assert!(bob_recv.contains("QSC_RECEIPT mode=batched status=sent kind=message peer=bob"));
    assert!(bob_recv.contains("QSC_TUI_RECEIPT mode=batched status=queued kind=message thread=bob"));

    let mut all = String::new();
    all.push_str(&bob_recv);
    assert_no_leaks(&all);
}
