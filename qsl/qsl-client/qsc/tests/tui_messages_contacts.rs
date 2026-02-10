use assert_cmd::Command as AssertCommand;

fn run_headless(script: &str) -> String {
    let mut cmd = AssertCommand::new(assert_cmd::cargo::cargo_bin!("qsc"));
    let out = cmd
        .env("QSC_TUI_HEADLESS", "1")
        .env("QSC_TUI_SCRIPT", script)
        .env("QSC_TUI_COLS", "140")
        .env("QSC_TUI_ROWS", "40")
        .env("QSC_MARK_FORMAT", "plain")
        .args(["tui"])
        .output()
        .expect("run tui headless");
    let mut combined = String::from_utf8_lossy(&out.stdout).to_string();
    combined.push_str(&String::from_utf8_lossy(&out.stderr));
    combined
}

#[test]
fn messages_domain_renders_unified_nav_and_full_cmdbar() {
    let out = run_headless("/inspector events;/exit");
    assert!(
        out.contains("event=tui_render mode=home layout=h3 inspector=events")
            && out.contains("expanded=events")
            && out.contains("cmdbar=full"),
        "missing home render markers: {}",
        out
    );
    assert!(
        out.contains("event=tui_messages_view"),
        "missing messages view marker: {}",
        out
    );
}

#[test]
fn unfocused_message_update_buffers_and_increments_unread() {
    let out = run_headless(
        "/inspector events;/messages select alice;/injectmsg alice SENT;/key tab;/injectmsg alice RECEIVED;/exit",
    );
    assert!(
        out.contains("event=tui_messages_view peer=alice total=2 visible=1 unread=1"),
        "expected buffered unread behavior marker: {}",
        out
    );
}

#[test]
fn contacts_domain_renders_verification_and_pinning_sections() {
    let out = run_headless("/inspector contacts;/exit");
    assert!(
        out.contains("event=tui_render mode=home layout=h3 inspector=contacts"),
        "missing contacts render marker: {}",
        out
    );
    assert!(
        out.contains("event=tui_contacts_view")
            && out.contains("sections=verification,pinning,commands"),
        "missing contacts section marker: {}",
        out
    );
}
