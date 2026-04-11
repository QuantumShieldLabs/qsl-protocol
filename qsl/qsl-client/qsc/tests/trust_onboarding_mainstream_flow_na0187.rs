mod common;

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

const CODE_BOB: &str = "ABCD-EFGH-JKMP-QRST-V";
const ROUTE_BOB: &str = "route_token_bob_abcdefghijklmnopqr";
const MAILBOX_ALICE: &str = "mailbox_alice_abcdefghijklmnopqrst";

fn ensure_dir_700(path: &Path) {
    fs::create_dir_all(path).expect("create dir");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(path, fs::Permissions::from_mode(0o700)).expect("chmod 700");
    }
}

fn unique_test_dir(tag: &str) -> PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock")
        .as_nanos();
    std::env::current_dir()
        .expect("cwd")
        .join("target")
        .join("qsc-tests")
        .join(format!("{tag}-{}-{nonce}", std::process::id()))
}

fn output_text(out: &std::process::Output) -> String {
    let mut s = String::from_utf8_lossy(&out.stdout).to_string();
    s.push_str(&String::from_utf8_lossy(&out.stderr));
    s
}

fn assert_no_leaks(s: &str) {
    assert!(!s.contains("/v1/"), "unexpected /v1/ output: {s}");
    let mut run = 0usize;
    for ch in s.chars() {
        if ch.is_ascii_hexdigit() {
            run = run.saturating_add(1);
            assert!(run < 32, "unexpected long-hex output: {s}");
        } else {
            run = 0;
        }
    }
}

fn qsc(cfg: &Path) -> Command {
    let mut cmd = common::qsc_std_command();
    cmd.env("QSC_CONFIG_DIR", cfg)
        .env("QSC_MARK_FORMAT", "plain")
        .env("QSC_ALLOW_SEED_FALLBACK", "1")
        .env("QSC_QSP_SEED", "7");
    cmd
}

fn run_headless(cfg: &Path, script: &str) -> String {
    let mut cmd = common::qsc_assert_command();
    let out = cmd
        .env("QSC_CONFIG_DIR", cfg)
        .env("QSC_TUI_HEADLESS", "1")
        .env("QSC_TUI_SCRIPT", script)
        .env("QSC_TUI_COLS", "140")
        .env("QSC_TUI_ROWS", "40")
        .env("QSC_MARK_FORMAT", "plain")
        .env("QSC_TUI_TEST_UNLOCK", "1")
        .args(["tui"])
        .output()
        .expect("run tui");
    let combined = output_text(&out);
    assert!(out.status.success(), "tui failed: {combined}");
    combined
}

fn first_device_id(cfg: &Path, label: &str) -> String {
    let out = qsc(cfg)
        .args(["contacts", "device", "list", "--label", label])
        .output()
        .expect("device list");
    assert!(out.status.success(), "{}", output_text(&out));
    output_text(&out)
        .lines()
        .find_map(|line| line.strip_prefix("device="))
        .and_then(|line| line.split_whitespace().next())
        .expect("device line")
        .to_string()
}

#[test]
fn balanced_mode_verify_promotes_to_trusted() {
    let cfg = unique_test_dir("na0187_balanced_promote");
    ensure_dir_700(&cfg);
    common::init_mock_vault(&cfg);

    let set_mode = qsc(&cfg)
        .args(["contacts", "trust-mode", "set", "--mode", "balanced"])
        .output()
        .expect("set mode");
    assert!(set_mode.status.success(), "{}", output_text(&set_mode));

    let add = qsc(&cfg)
        .args([
            "contacts",
            "add",
            "--label",
            "bob",
            "--fp",
            CODE_BOB,
            "--route-token",
            ROUTE_BOB,
            "--verify",
        ])
        .output()
        .expect("contacts add");
    assert!(add.status.success(), "{}", output_text(&add));

    let verify = qsc(&cfg)
        .args([
            "contacts",
            "verify",
            "--label",
            "bob",
            "--fp",
            CODE_BOB,
            "--confirm",
        ])
        .output()
        .expect("verify");
    assert!(verify.status.success(), "{}", output_text(&verify));
    let text = output_text(&verify);
    assert!(
        text.contains("QSC_TRUST_PROMOTION result=trusted reason=verified_match peer=bob")
            && text.contains("mode=balanced"),
        "{}",
        text
    );

    let show = qsc(&cfg)
        .args(["contacts", "show", "--label", "bob"])
        .output()
        .expect("show");
    assert!(show.status.success(), "{}", output_text(&show));
    let show_text = output_text(&show);
    assert!(show_text.contains("state=PINNED"), "{}", show_text);
    assert_no_leaks(&text);
    assert_no_leaks(&show_text);
}

#[test]
fn strict_mode_verify_requires_explicit_trust() {
    let cfg = unique_test_dir("na0187_strict_requires_trust");
    ensure_dir_700(&cfg);
    common::init_mock_vault(&cfg);

    let set_mode = qsc(&cfg)
        .args(["contacts", "trust-mode", "set", "--mode", "strict"])
        .output()
        .expect("set mode");
    assert!(set_mode.status.success(), "{}", output_text(&set_mode));

    let add = qsc(&cfg)
        .args([
            "contacts",
            "add",
            "--label",
            "bob",
            "--fp",
            CODE_BOB,
            "--route-token",
            ROUTE_BOB,
            "--verify",
        ])
        .output()
        .expect("contacts add");
    assert!(add.status.success(), "{}", output_text(&add));

    let verify = qsc(&cfg)
        .args([
            "contacts",
            "verify",
            "--label",
            "bob",
            "--fp",
            CODE_BOB,
            "--confirm",
        ])
        .output()
        .expect("verify");
    assert!(verify.status.success(), "{}", output_text(&verify));
    let verify_text = output_text(&verify);
    assert!(
        verify_text
            .contains("QSC_TRUST_PROMOTION result=verified_only reason=strict_mode peer=bob")
            && verify_text.contains("mode=strict"),
        "{}",
        verify_text
    );

    let payload = cfg.join("msg.bin");
    fs::write(&payload, b"hello").expect("payload");
    let send_blocked = qsc(&cfg)
        .args([
            "send",
            "--transport",
            "relay",
            "--relay",
            "http://127.0.0.1:9",
            "--to",
            "bob",
            "--file",
            payload.to_str().unwrap(),
        ])
        .output()
        .expect("send blocked");
    assert!(
        !send_blocked.status.success(),
        "{}",
        output_text(&send_blocked)
    );
    let blocked_text = output_text(&send_blocked);
    assert!(
        blocked_text.contains("QSC_SEND_BLOCKED reason=no_trusted_device peer=bob"),
        "{}",
        blocked_text
    );

    let dev = first_device_id(&cfg, "bob");
    let trust = qsc(&cfg)
        .args([
            "contacts",
            "device",
            "trust",
            "--label",
            "bob",
            "--device",
            dev.as_str(),
            "--confirm",
        ])
        .output()
        .expect("trust");
    assert!(trust.status.success(), "{}", output_text(&trust));
    assert_no_leaks(&verify_text);
    assert_no_leaks(&blocked_text);
}

#[test]
fn unknown_inbound_creates_request_and_accept_keeps_untrusted() {
    let cfg = unique_test_dir("na0187_request_flow");
    ensure_dir_700(&cfg);
    common::init_mock_vault(&cfg);
    let server = common::start_inbox_server(1024 * 1024, 64);
    server.enqueue_raw(MAILBOX_ALICE, b"not-an-envelope".to_vec());
    let out_dir = cfg.join("recv");
    ensure_dir_700(&out_dir);

    let recv = qsc(&cfg)
        .args([
            "receive",
            "--transport",
            "relay",
            "--relay",
            server.base_url(),
            "--mailbox",
            MAILBOX_ALICE,
            "--from",
            "bob",
            "--max",
            "1",
            "--out",
            out_dir.to_str().unwrap(),
        ])
        .output()
        .expect("receive");
    let recv_text = output_text(&recv);
    assert!(
        recv_text.contains("QSC_CONTACT_REQUEST action=created peer=bob"),
        "{}",
        recv_text
    );

    let accept = qsc(&cfg)
        .args(["contacts", "request", "accept", "--label", "bob"])
        .output()
        .expect("request accept");
    assert!(accept.status.success(), "{}", output_text(&accept));
    let accept_text = output_text(&accept);
    assert!(
        accept_text.contains("QSC_CONTACT_REQUEST action=accept peer=bob"),
        "{}",
        accept_text
    );

    let payload = cfg.join("msg.bin");
    fs::write(&payload, b"hello").expect("payload");
    let send_blocked = qsc(&cfg)
        .args([
            "send",
            "--transport",
            "relay",
            "--relay",
            "http://127.0.0.1:9",
            "--to",
            "bob",
            "--file",
            payload.to_str().unwrap(),
        ])
        .output()
        .expect("send blocked");
    assert!(
        !send_blocked.status.success(),
        "{}",
        output_text(&send_blocked)
    );
    let blocked_text = output_text(&send_blocked);
    assert!(
        blocked_text.contains("QSC_SEND_BLOCKED reason=no_trusted_device peer=bob"),
        "{}",
        blocked_text
    );

    assert_no_leaks(&recv_text);
    assert_no_leaks(&accept_text);
    assert_no_leaks(&blocked_text);
}

#[test]
fn tui_strict_mode_blocks_first_use_auto_promotion() {
    let cfg = unique_test_dir("na0187_tui_strict");
    ensure_dir_700(&cfg);
    common::init_mock_vault(&cfg);

    let script = format!(
        "/unlock;/trust mode strict;/contacts add bob {} {};/msg bob hello;/exit",
        CODE_BOB, ROUTE_BOB
    );
    let out = run_headless(&cfg, script.as_str());
    assert!(out.contains("QSC_TUI_TRUST_MODE mode=strict"), "{}", out);
    assert!(
        out.contains(
            "QSC_TUI_TRUST_PROMOTION result=verified_only reason=strict_mode peer=bob mode=strict"
        ),
        "{}",
        out
    );
    assert!(
        out.contains("QSC_TUI_SEND_BLOCKED reason=no_trusted_device peer=bob"),
        "{}",
        out
    );
    assert_no_leaks(&out);
}

#[test]
fn offline_add_stays_discovered_until_retry_verify() {
    let cfg = unique_test_dir("na0187_offline_retry");
    ensure_dir_700(&cfg);
    common::init_mock_vault(&cfg);

    let set_mode = qsc(&cfg)
        .args(["contacts", "trust-mode", "set", "--mode", "balanced"])
        .output()
        .expect("set mode");
    assert!(set_mode.status.success(), "{}", output_text(&set_mode));

    let add = qsc(&cfg)
        .args([
            "contacts",
            "add",
            "--label",
            "bob",
            "--fp",
            CODE_BOB,
            "--route-token",
            ROUTE_BOB,
            "--verify",
        ])
        .output()
        .expect("add");
    assert!(add.status.success(), "{}", output_text(&add));
    let add_text = output_text(&add);
    assert!(
        add_text.contains("QSC_CONTACT_FLOW action=add state=VERIFIED peer=bob mode=balanced"),
        "{}",
        add_text
    );

    let payload = cfg.join("msg.bin");
    fs::write(&payload, b"hello").expect("payload");
    let blocked = qsc(&cfg)
        .args([
            "send",
            "--transport",
            "relay",
            "--relay",
            "http://127.0.0.1:9",
            "--to",
            "bob",
            "--file",
            payload.to_str().unwrap(),
        ])
        .output()
        .expect("send blocked");
    assert!(!blocked.status.success(), "{}", output_text(&blocked));
    let blocked_text = output_text(&blocked);
    assert!(
        blocked_text.contains("QSC_SEND_BLOCKED reason=no_trusted_device peer=bob"),
        "{}",
        blocked_text
    );

    let verify = qsc(&cfg)
        .args([
            "contacts",
            "verify",
            "--label",
            "bob",
            "--fp",
            CODE_BOB,
            "--confirm",
        ])
        .output()
        .expect("verify retry");
    assert!(verify.status.success(), "{}", output_text(&verify));
    let verify_text = output_text(&verify);
    assert!(
        verify_text.contains("QSC_CONTACT_FLOW action=verify state=TRUSTED peer=bob")
            && verify_text.contains("mode=balanced")
            && verify_text
                .contains("QSC_TRUST_PROMOTION result=trusted reason=verified_match peer=bob"),
        "{}",
        verify_text
    );
    assert_no_leaks(&add_text);
    assert_no_leaks(&blocked_text);
    assert_no_leaks(&verify_text);
}

#[test]
fn wrong_code_marks_changed_and_blocks_send() {
    let cfg = unique_test_dir("na0187_wrong_code_changed");
    ensure_dir_700(&cfg);
    common::init_mock_vault(&cfg);

    let add = qsc(&cfg)
        .args([
            "contacts",
            "add",
            "--label",
            "bob",
            "--fp",
            CODE_BOB,
            "--route-token",
            ROUTE_BOB,
            "--verify",
        ])
        .output()
        .expect("add");
    assert!(add.status.success(), "{}", output_text(&add));

    let verify = qsc(&cfg)
        .args([
            "contacts",
            "verify",
            "--label",
            "bob",
            "--fp",
            "WXYZ-1234-ABCD-9876-V",
            "--confirm",
        ])
        .output()
        .expect("verify mismatch");
    assert!(!verify.status.success(), "{}", output_text(&verify));
    let verify_text = output_text(&verify);
    assert!(
        verify_text.contains("QSC_CONTACT_FLOW action=verify state=CHANGED peer=bob")
            && verify_text.contains("QSC_MARK/1 event=error code=verification_mismatch"),
        "{}",
        verify_text
    );

    let payload = cfg.join("msg.bin");
    fs::write(&payload, b"hello").expect("payload");
    let blocked = qsc(&cfg)
        .args([
            "send",
            "--transport",
            "relay",
            "--relay",
            "http://127.0.0.1:9",
            "--to",
            "bob",
            "--file",
            payload.to_str().unwrap(),
        ])
        .output()
        .expect("send blocked");
    assert!(!blocked.status.success(), "{}", output_text(&blocked));
    let blocked_text = output_text(&blocked);
    assert!(
        blocked_text
            .contains("QSC_SEND_BLOCKED reason=device_changed_reapproval_required peer=bob"),
        "{}",
        blocked_text
    );
    assert_no_leaks(&verify_text);
    assert_no_leaks(&blocked_text);
}

#[test]
fn request_ignore_and_block_are_deterministic() {
    let cfg = unique_test_dir("na0187_request_ignore_block");
    ensure_dir_700(&cfg);
    common::init_mock_vault(&cfg);
    let server = common::start_inbox_server(1024 * 1024, 64);
    server.enqueue_raw(MAILBOX_ALICE, b"not-an-envelope".to_vec());
    let out_dir = cfg.join("recv");
    ensure_dir_700(&out_dir);

    let recv = qsc(&cfg)
        .args([
            "receive",
            "--transport",
            "relay",
            "--relay",
            server.base_url(),
            "--mailbox",
            MAILBOX_ALICE,
            "--from",
            "eve",
            "--max",
            "1",
            "--out",
            out_dir.to_str().unwrap(),
        ])
        .output()
        .expect("receive");
    let recv_text = output_text(&recv);
    assert!(
        recv_text.contains("QSC_CONTACT_REQUEST action=created peer=eve"),
        "{}",
        recv_text
    );

    let ignore = qsc(&cfg)
        .args(["contacts", "request", "ignore", "--label", "eve"])
        .output()
        .expect("ignore");
    assert!(ignore.status.success(), "{}", output_text(&ignore));
    let ignore_text = output_text(&ignore);
    assert!(
        ignore_text.contains("QSC_CONTACT_REQUEST action=ignore peer=eve"),
        "{}",
        ignore_text
    );

    server.enqueue_raw(MAILBOX_ALICE, b"not-an-envelope-2".to_vec());
    let recv_again = qsc(&cfg)
        .args([
            "receive",
            "--transport",
            "relay",
            "--relay",
            server.base_url(),
            "--mailbox",
            MAILBOX_ALICE,
            "--from",
            "eve",
            "--max",
            "1",
            "--out",
            out_dir.to_str().unwrap(),
        ])
        .output()
        .expect("receive again");
    let recv_again_text = output_text(&recv_again);
    assert!(
        recv_again_text.contains("QSC_CONTACT_REQUEST action=created peer=eve"),
        "{}",
        recv_again_text
    );

    let block = qsc(&cfg)
        .args(["contacts", "request", "block", "--label", "eve"])
        .output()
        .expect("block");
    assert!(block.status.success(), "{}", output_text(&block));
    let block_text = output_text(&block);
    assert!(
        block_text.contains("QSC_CONTACT_REQUEST action=block peer=eve"),
        "{}",
        block_text
    );

    let show = qsc(&cfg)
        .args(["contacts", "show", "--label", "eve"])
        .output()
        .expect("show");
    assert!(show.status.success(), "{}", output_text(&show));
    let show_text = output_text(&show);
    assert!(
        show_text.contains("state=CHANGED")
            && show_text.contains("blocked=true")
            && show_text.contains("state=REVOKED"),
        "{}",
        show_text
    );
    assert_no_leaks(&recv_text);
    assert_no_leaks(&ignore_text);
    assert_no_leaks(&recv_again_text);
    assert_no_leaks(&block_text);
    assert_no_leaks(&show_text);
}

#[test]
fn new_device_does_not_change_primary_or_send_routing() {
    let cfg = unique_test_dir("na0187_multidevice_primary");
    ensure_dir_700(&cfg);
    common::init_mock_vault(&cfg);

    let add = qsc(&cfg)
        .args([
            "contacts",
            "add",
            "--label",
            "bob",
            "--fp",
            CODE_BOB,
            "--route-token",
            ROUTE_BOB,
            "--verify",
        ])
        .output()
        .expect("add");
    assert!(add.status.success(), "{}", output_text(&add));

    let verify = qsc(&cfg)
        .args([
            "contacts",
            "verify",
            "--label",
            "bob",
            "--fp",
            CODE_BOB,
            "--confirm",
        ])
        .output()
        .expect("verify");
    assert!(verify.status.success(), "{}", output_text(&verify));

    let primary_before = qsc(&cfg)
        .args(["contacts", "device", "primary", "show", "--label", "bob"])
        .output()
        .expect("primary show");
    assert!(
        primary_before.status.success(),
        "{}",
        output_text(&primary_before)
    );
    let primary_before_text = output_text(&primary_before);

    let add_device = qsc(&cfg)
        .args([
            "contacts",
            "device",
            "add",
            "--label",
            "bob",
            "--fp",
            "WXYZ-ABCD-EFGH-JKLM-V",
            "--route-token",
            ROUTE_BOB,
        ])
        .output()
        .expect("device add");
    assert!(add_device.status.success(), "{}", output_text(&add_device));
    let add_device_text = output_text(&add_device);
    assert!(
        add_device_text.contains("state=UNVERIFIED"),
        "{}",
        add_device_text
    );

    let primary_after = qsc(&cfg)
        .args(["contacts", "device", "primary", "show", "--label", "bob"])
        .output()
        .expect("primary show after");
    assert!(
        primary_after.status.success(),
        "{}",
        output_text(&primary_after)
    );
    let primary_after_text = output_text(&primary_after);
    assert_eq!(primary_before_text, primary_after_text);

    let payload = cfg.join("msg.bin");
    fs::write(&payload, b"hello").expect("payload");
    let send = qsc(&cfg)
        .args([
            "send",
            "--transport",
            "relay",
            "--relay",
            "http://127.0.0.1:9",
            "--to",
            "bob",
            "--file",
            payload.to_str().unwrap(),
        ])
        .output()
        .expect("send");
    assert!(!send.status.success(), "{}", output_text(&send));
    let send_text = output_text(&send);
    assert!(
        send_text.contains("QSC_ROUTING policy=primary_only peer=bob"),
        "{}",
        send_text
    );
    assert_no_leaks(&primary_before_text);
    assert_no_leaks(&add_device_text);
    assert_no_leaks(&primary_after_text);
    assert_no_leaks(&send_text);
}
