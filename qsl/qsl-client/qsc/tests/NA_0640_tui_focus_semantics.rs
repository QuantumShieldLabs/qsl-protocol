// NA-0640 (D576), part 2: preserve the TUI focus-routing semantics IN-SUITE before the
// drifted relay-ui-integration workflow is retired.
//
// These two assertions previously lived ONLY in the #[ignore]d relay_ui_integration.rs
// tests (na-0127), which ran solely from the schedule-only relay-ui-integration.yml
// workflow — dead since 2026-02-11, so the semantics have been unguarded for months:
//   - inbound while the thread is UNFOCUSED buffers and increments the unread counter
//     (event=tui_message_event ... mode=buffer ... unread=1);
//   - inbound while the thread is FOCUSED appends to the visible stream and does not
//     touch unread (mode=append ... unread=0).
// The semantics are TUI-local: they do not need a real relay (D576), so this port uses
// the in-suite mock inbox (common::start_inbox_server) with the current na0177
// headless-TUI idiom (script-driven contacts/trust; the stale na-0127 scaffolding
// predates the route-token migration and is not reused).
//
// Focus scripting note: "focused" here is the append condition in
// controller/state/account.rs (mode==Normal && inspector==Events && home_focus==Main
// && selected==peer). Under the CURRENT key model, `/messages select <peer>` leaves
// home_focus on the nav column (thread NOT focused) and `/key tab` toggles home_focus
// into Main (thread focused) — the inverse of the na-0127 scripts' era. The scripts
// below drive the current controls; the ASSERTED semantics are unchanged.

mod common;

use std::fs;
use std::path::{Path, PathBuf};

fn unique_cfg_dir(tag: &str) -> PathBuf {
    common::unique_test_root(tag)
}

fn ensure_dir_700(path: &Path) {
    let _ = fs::create_dir_all(path);
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let _ = fs::set_permissions(path, fs::Permissions::from_mode(0o700));
    }
}

fn run_headless(cfg: &Path, script: &str, self_label: &str) -> String {
    let mut cmd = common::qsc_assert_command();
    cmd.env("QSC_CONFIG_DIR", cfg)
        .env("QSC_TUI_HEADLESS", "1")
        .env("QSC_TUI_SCRIPT", script)
        .env("QSC_TUI_COLS", "140")
        .env("QSC_TUI_ROWS", "40")
        .env("QSC_MARK_FORMAT", "plain")
        .env("QSC_TUI_TEST_UNLOCK", "1")
        .env("QSC_ALLOW_SEED_FALLBACK", "1")
        .env("QSC_UNSAFE_TEST_SEED_FALLBACK", "1")
        .env("QSC_QSP_SEED", "7")
        .env("QSC_SELF_LABEL", self_label)
        .arg("tui");
    let out = cmd.output().expect("headless tui run");
    let text = {
        let mut s = String::from_utf8_lossy(&out.stdout).to_string();
        s.push_str(&String::from_utf8_lossy(&out.stderr));
        s
    };
    assert!(out.status.success(), "tui failed: {text}");
    text
}

fn latest_message_event_line<'a>(text: &'a str, peer: &str) -> Option<&'a str> {
    text.lines().rfind(|line| {
        line.contains("event=tui_message_event") && line.contains(&format!("peer={peer}"))
    })
}

fn send_one_from(cfg: &Path, relay: &str, inbox_self: &str, route_peer: &str, body: &str) {
    let code = "ABCD-EFGH-JKMP-QRST-V";
    let script = format!(
        "/unlock;/relay set endpoint {relay};/relay inbox set {inbox_self};/contacts add bob {code} {route_peer};/trust pin bob confirm;/msg bob {body};/exit"
    );
    let out = run_headless(cfg, script.as_str(), "alice");
    assert!(
        out.contains("QSC_TUI_ORCH stage=send status=ok"),
        "sender orchestration failed: {out}"
    );
    assert!(
        out.contains("QSC_TUI_DELIVERY state=accepted_by_relay"),
        "sender delivery marker missing: {out}"
    );
}

fn receive_with_script(
    cfg: &Path,
    relay: &str,
    inbox_self: &str,
    route_peer: &str,
    focus_script: &str,
) -> String {
    let code = "ABCD-EFGH-JKMP-QRST-V";
    let script = format!(
        "/unlock;/relay set endpoint {relay};/relay inbox set {inbox_self};/contacts add bob {code} {route_peer};/trust pin bob confirm;{focus_script};/exit"
    );
    run_headless(cfg, script.as_str(), "bob")
}

// Ported assertion 1 of 2 (was relay_unfocused_inbound_increments_counter_only):
// unfocused inbound => the main view buffers and the unread counter increments.
#[test]
fn unfocused_inbound_increments_counter_only() {
    let server = common::start_inbox_server(1024 * 1024, 16);
    let relay = server.base_url().to_string();

    let cfg_a = unique_cfg_dir("na0640_tui_unfocused_a");
    let cfg_b = unique_cfg_dir("na0640_tui_unfocused_b");
    ensure_dir_700(&cfg_a);
    ensure_dir_700(&cfg_b);
    common::init_mock_vault(&cfg_a);
    common::init_mock_vault(&cfg_b);

    let inbox_a = "na0640_unf_alice_inbox_AA11BB22";
    let inbox_b = "na0640_unf_bob_inbox_DD44EE55";

    send_one_from(&cfg_a, &relay, inbox_a, inbox_b, "na0640-unfocused-inbound");

    let out = receive_with_script(
        &cfg_b,
        &relay,
        inbox_b,
        inbox_a,
        "/inspector events;/messages select bob;/receive",
    );
    assert!(
        out.contains("event=tui_receive"),
        "missing tui_receive: {out}"
    );
    let line = latest_message_event_line(&out, "bob")
        .unwrap_or_else(|| panic!("message event marker for bob missing in output:\n{out}"));
    assert!(line.contains("total=1"), "missing total=1: {line}");
    assert!(
        line.contains("mode=buffer"),
        "main view should buffer while unfocused: {line}"
    );
    assert!(
        line.contains("unread=1"),
        "unfocused inbound must increment unread: {line}"
    );
}

// Ported assertion 2 of 2 (was relay_focused_inbound_appends_to_stream):
// focused inbound => appends to the visible stream, unread untouched.
#[test]
fn focused_inbound_appends_to_stream() {
    let server = common::start_inbox_server(1024 * 1024, 16);
    let relay = server.base_url().to_string();

    let cfg_a = unique_cfg_dir("na0640_tui_focused_a");
    let cfg_b = unique_cfg_dir("na0640_tui_focused_b");
    ensure_dir_700(&cfg_a);
    ensure_dir_700(&cfg_b);
    common::init_mock_vault(&cfg_a);
    common::init_mock_vault(&cfg_b);

    let inbox_a = "na0640_foc_alice_inbox_AA11BB22";
    let inbox_b = "na0640_foc_bob_inbox_DD44EE55";

    send_one_from(&cfg_a, &relay, inbox_a, inbox_b, "na0640-focused-inbound");

    let out = receive_with_script(
        &cfg_b,
        &relay,
        inbox_b,
        inbox_a,
        "/inspector events;/messages select bob;/key tab;/receive",
    );
    assert!(
        out.contains("event=tui_receive"),
        "missing tui_receive: {out}"
    );
    let line = latest_message_event_line(&out, "bob")
        .unwrap_or_else(|| panic!("message event marker for bob missing in output:\n{out}"));
    assert!(line.contains("total=1"), "missing total=1: {line}");
    assert!(
        line.contains("mode=append"),
        "focused inbound should append to main stream: {line}"
    );
    assert!(
        line.contains("unread=0"),
        "focused inbound should not increment unread: {line}"
    );
}
