mod common;

use assert_cmd::Command as AssertCommand;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

fn unique_cfg_dir(tag: &str) -> PathBuf {
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

fn ensure_dir_700(path: &Path) {
    fs::create_dir_all(path).expect("create dir");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(path, fs::Permissions::from_mode(0o700)).expect("chmod");
    }
}

fn run_headless(
    cfg: &Path,
    script: &str,
    extra_env: &[(&str, &str)],
    extra_args: &[&str],
) -> String {
    let mut cmd = AssertCommand::new(assert_cmd::cargo::cargo_bin!("qsc"));
    cmd.env("QSC_CONFIG_DIR", cfg)
        .env("QSC_TUI_HEADLESS", "1")
        .env("QSC_TUI_SCRIPT", script)
        .env("QSC_TUI_COLS", "140")
        .env("QSC_TUI_ROWS", "40")
        .env("QSC_MARK_FORMAT", "plain");
    for (k, v) in extra_env {
        cmd.env(k, v);
    }
    let out = cmd
        .args(["tui"])
        .args(extra_args)
        .output()
        .expect("run tui");
    let mut combined = String::from_utf8_lossy(&out.stdout).to_string();
    combined.push_str(&String::from_utf8_lossy(&out.stderr));
    assert!(out.status.success(), "tui failed: {combined}");
    combined
}

fn assert_no_leaks(out: &str) {
    assert!(
        !out.contains("/v1/"),
        "unexpected /v1/ content in output: {out}"
    );
    let mut run = 0usize;
    for ch in out.chars() {
        if ch.is_ascii_hexdigit() {
            run = run.saturating_add(1);
            assert!(run < 32, "unexpected long hex token in output: {out}");
        } else {
            run = 0;
        }
    }
}

#[test]
fn zero_arg_tui_stays_open_with_setup_required_marker() {
    let cfg = unique_cfg_dir("na0177_setup_required");
    ensure_dir_700(&cfg);
    let out = run_headless(&cfg, "/exit", &[], &[]);
    assert!(
        out.contains("QSC_TUI_SETUP_REQUIRED relay=missing auth=missing"),
        "missing setup-required marker: {out}"
    );
    assert_no_leaks(&out);
}

#[test]
fn msg_unknown_peer_is_fail_closed_and_no_mutation() {
    let cfg = unique_cfg_dir("na0177_unknown_blocked");
    ensure_dir_700(&cfg);
    common::init_mock_vault(&cfg);
    let out = run_headless(
        &cfg,
        "/inspector events;/msg unknown hello;/messages list;/exit",
        &[("QSC_TUI_TEST_UNLOCK", "1")],
        &[],
    );
    assert!(
        out.contains("QSC_TUI_SEND_BLOCKED reason=unknown_contact peer=unknown"),
        "unknown peer block marker missing: {out}"
    );
    assert!(
        !out.contains("event=tui_message_event peer=unknown state=SENT"),
        "unknown peer send should not mutate message timeline: {out}"
    );
    assert!(
        !out.contains("QSC_TUI_DELIVERY state=accepted_by_relay thread=unknown"),
        "unknown peer path must not emit delivery success marker: {out}"
    );
    assert!(
        !out.contains("QSC_TUI_ORCH stage=send status=ok"),
        "unknown peer path must not attempt send: {out}"
    );
    assert_no_leaks(&out);
}

#[test]
fn msg_peer_auto_orchestrates_and_focuses_messages_thread() {
    let server = common::start_inbox_server(1024 * 1024, 16);
    let relay = server.base_url().to_string();

    let cfg_a = unique_cfg_dir("na0177_orch_a");
    let cfg_b = unique_cfg_dir("na0177_orch_b");
    ensure_dir_700(&cfg_a);
    ensure_dir_700(&cfg_b);
    common::init_mock_vault(&cfg_a);
    common::init_mock_vault(&cfg_b);

    let code = "ABCD-EFGH-JKMP-QRST-V";
    let inbox_a = "alice_inbox_AA11BB22CC33";
    let inbox_b = "bob_inbox_DD44EE55FF66";
    let route_a = inbox_a;
    let route_b = inbox_b;

    let script_a = format!(
        "/unlock;/relay set endpoint {relay};/relay inbox set {inbox_a};/contacts add bob {code} {route_b};/trust pin bob confirm;/msg bob hello;/exit"
    );
    let out_a = run_headless(
        &cfg_a,
        script_a.as_str(),
        &[
            ("QSC_TUI_TEST_UNLOCK", "1"),
            ("QSC_ALLOW_SEED_FALLBACK", "1"),
            ("QSC_QSP_SEED", "7"),
            ("QSC_SELF_LABEL", "alice"),
        ],
        &[],
    );
    assert!(
        out_a.contains("QSC_TUI_ORCH stage=ensure_handshake status=skip")
            || out_a.contains("QSC_TUI_ORCH stage=ensure_handshake status=ok"),
        "handshake orchestration marker missing: {out_a}"
    );
    assert!(
        out_a.contains("QSC_TUI_ORCH stage=send status=ok"),
        "send orchestration marker missing: {out_a}"
    );
    assert!(
        out_a.contains("QSC_TUI_NAV focus=messages thread=bob"),
        "thread focus marker missing for bob: {out_a}"
    );
    assert!(
        out_a.contains("QSC_TUI_DELIVERY state=accepted_by_relay")
            && out_a.contains(" thread=bob "),
        "accepted_by_relay delivery marker missing for bob: {out_a}"
    );

    let script_b = format!(
        "/unlock;/relay set endpoint {relay};/relay inbox set {inbox_b};/contacts add bob {code} {route_a};/trust pin bob confirm;/msg bob ack;/exit"
    );
    let out_b = run_headless(
        &cfg_b,
        script_b.as_str(),
        &[
            ("QSC_TUI_TEST_UNLOCK", "1"),
            ("QSC_ALLOW_SEED_FALLBACK", "1"),
            ("QSC_QSP_SEED", "7"),
            ("QSC_SELF_LABEL", "bob"),
        ],
        &[],
    );
    assert!(
        out_b.contains("event=tui_receive from=bob count=1")
            || out_b.contains("event=tui_message_event peer=bob state=RECEIVED"),
        "receiver should observe inbound message without manual /receive: {out_b}"
    );
    assert_no_leaks(&out_a);
    assert_no_leaks(&out_b);
}

#[test]
fn msg_peer_auto_trusts_first_use_after_handshake_and_sends() {
    let server = common::start_inbox_server(1024 * 1024, 16);
    let relay = server.base_url().to_string();

    let cfg_a = unique_cfg_dir("na0177_auto_trust_a");
    let cfg_b = unique_cfg_dir("na0177_auto_trust_b");
    ensure_dir_700(&cfg_a);
    ensure_dir_700(&cfg_b);
    common::init_mock_vault(&cfg_a);
    common::init_mock_vault(&cfg_b);

    let code = "ABCD-EFGH-JKMP-QRST-V";
    let inbox_a = "alice_inbox_112233445566";
    let inbox_b = "bob_inbox_665544332211";
    let route_a = inbox_a;
    let route_b = inbox_b;

    let script_a = format!(
        "/unlock;/relay set endpoint {relay};/relay inbox set {inbox_a};/contacts add bob {code} {route_b};/msg bob hello;/exit"
    );
    let out_a = run_headless(
        &cfg_a,
        script_a.as_str(),
        &[
            ("QSC_TUI_TEST_UNLOCK", "1"),
            ("QSC_ALLOW_SEED_FALLBACK", "1"),
            ("QSC_QSP_SEED", "11"),
            ("QSC_SELF_LABEL", "alice"),
        ],
        &[],
    );
    assert!(
        out_a.contains("QSC_TUI_ORCH stage=ensure_handshake status=ok")
            || out_a.contains("QSC_TUI_ORCH stage=ensure_handshake status=skip"),
        "handshake marker missing: {out_a}"
    );
    assert!(
        out_a.contains("QSC_TUI_ORCH stage=auto_trust status=ok"),
        "auto-trust marker missing: {out_a}"
    );
    assert!(
        out_a.contains("QSC_TUI_ORCH stage=send status=ok"),
        "send marker missing: {out_a}"
    );
    assert!(
        out_a.contains("QSC_TUI_NAV focus=messages thread=bob"),
        "navigation marker missing: {out_a}"
    );

    let script_b = format!(
        "/unlock;/relay set endpoint {relay};/relay inbox set {inbox_b};/contacts add bob {code} {route_a};/trust pin bob confirm;/msg bob ack;/exit"
    );
    let out_b = run_headless(
        &cfg_b,
        script_b.as_str(),
        &[
            ("QSC_TUI_TEST_UNLOCK", "1"),
            ("QSC_ALLOW_SEED_FALLBACK", "1"),
            ("QSC_QSP_SEED", "11"),
            ("QSC_SELF_LABEL", "bob"),
        ],
        &[],
    );
    assert!(
        out_b.contains("event=tui_receive from=bob count=1")
            || out_b.contains("event=tui_message_event peer=bob state=RECEIVED"),
        "receiver should observe inbound after sender auto-trust flow: {out_b}"
    );
    assert_no_leaks(&out_a);
    assert_no_leaks(&out_b);
}

#[test]
fn files_view_emits_file_confirmation_semantics_marker() {
    let cfg = unique_cfg_dir("na0177_file_confirm_marker");
    ensure_dir_700(&cfg);
    common::init_mock_vault(&cfg);
    let out = run_headless(
        &cfg,
        "/unlock;/inspector files;/files inject fileabc123 AWAITING_CONFIRMATION 42 demo.bin;/files select fileabc123;/inspector files;/exit",
        &[("QSC_TUI_TEST_UNLOCK", "1")],
        &[],
    );
    assert!(
        out.contains("QSC_TUI_FILE_CONFIRM state=awaiting_confirmation")
            && out.contains(" thread="),
        "missing file confirmation semantics marker: {out}"
    );
    assert!(
        out.contains("file=fileabc123"),
        "missing short file marker id in TUI marker: {out}"
    );
    assert_no_leaks(&out);
}
