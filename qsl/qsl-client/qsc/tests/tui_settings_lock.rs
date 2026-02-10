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
fn settings_view_is_read_only_and_cmdbar_present() {
    let out = run_headless("/inspector settings;/exit");
    assert!(
        out.contains("event=tui_render mode=home layout=h3 inspector=settings")
            && out.contains("expanded=settings")
            && out.contains("cmdbar=full"),
        "missing settings home layout markers: {}",
        out
    );
    assert!(
        out.contains("event=tui_settings_view")
            && out.contains("read_only=true")
            && out.contains("inline_actions=false")
            && out.contains("sections=policy,maintenance,commands"),
        "missing settings invariants marker: {}",
        out
    );
}

#[test]
fn lock_view_redacts_sensitive_content_across_domains() {
    let out = run_headless(
        "/inspector lock;/inspector events;/injectmsg alice RECEIVED;/inspector keys;/exit",
    );
    assert!(
        out.contains("event=tui_lock_view")
            && out.contains("locked=LOCKED")
            && out.contains("redacted=true"),
        "missing lock-view redaction marker: {}",
        out
    );
    assert!(
        out.contains("event=tui_messages_view") && out.contains("redacted=true"),
        "messages view must be redacted while locked: {}",
        out
    );
    assert!(
        out.contains("event=tui_keys_view") && out.contains("redacted=true"),
        "keys view must be redacted while locked: {}",
        out
    );
}

#[test]
fn left_nav_never_renders_content_previews() {
    let out = run_headless("/inspector events;/injectmsg alice RECEIVED;/exit");
    assert!(
        out.contains("event=tui_messages_view") && out.contains("preview=none"),
        "messages nav must not expose content previews: {}",
        out
    );
    assert!(
        !out.contains("plaintext_preview"),
        "unexpected preview leakage marker found: {}",
        out
    );
}

#[test]
fn locked_state_disables_auto_append() {
    let out = run_headless(
        "/inspector events;/messages select alice;/key tab;/injectmsg alice SENT;/exit",
    );
    assert!(
        out.contains("event=tui_messages_view")
            && out.contains("peer=alice")
            && out.contains("total=1")
            && out.contains("visible=0")
            && out.contains("unread=1")
            && out.contains("redacted=true"),
        "locked mode must buffer messages instead of auto-append: {}",
        out
    );
}
