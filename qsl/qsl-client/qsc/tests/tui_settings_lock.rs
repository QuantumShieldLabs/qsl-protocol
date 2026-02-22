use assert_cmd::Command as AssertCommand;

fn run_headless(script: &str) -> String {
    let mut cmd = AssertCommand::new(assert_cmd::cargo::cargo_bin!("qsc"));
    let out = cmd
        .env("QSC_TUI_HEADLESS", "1")
        .env("QSC_TUI_TEST_UNLOCK", "1")
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
            && out
                .contains("sections=system_settings,lock,autolock,polling,vault_security,commands")
            && out.contains("vault_attempt_limit=off")
            && out.contains("vault_failed_unlocks=0"),
        "missing settings invariants marker: {}",
        out
    );
}

#[test]
fn lock_view_redacts_sensitive_content_across_domains() {
    let out = run_headless("/lock;/exit");
    assert!(
        out.contains("event=tui_lock_state")
            && out.contains("locked=LOCKED")
            && out.contains("event=tui_locked_shell")
            && out.contains("main=locked")
            && out.contains("cmd=/unlock"),
        "missing lock-view redaction marker: {}",
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
    let out = run_headless("/lock;/injectmsg alice SENT;/exit");
    assert!(
        out.contains("event=tui_locked_cmd_reject")
            && out.contains("cmd=injectmsg")
            && out.contains("code=locked_unlock_required")
            && !out.contains("event=tui_message_event"),
        "locked mode must reject injectmsg and avoid appends: {}",
        out
    );
}
