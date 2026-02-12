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
fn keys_domain_renders_and_blocks_multiselect() {
    let out = run_headless(
        "/inspector keys;/files inject f1 RECEIVING 10 a.bin;/files select f1;/files toggle;/exit",
    );
    assert!(
        out.contains("event=tui_keys_view")
            && out.contains("sections=metadata,verification,commands")
            && out.contains("multi_select=false"),
        "missing keys view invariant marker: {}",
        out
    );
    assert!(
        out.contains("event=tui_files_multiselect_blocked")
            && out.contains("reason=domain_not_files")
            && out.contains("domain=session"),
        "keys domain must block multiselect: {}",
        out
    );
}

#[test]
fn activity_unfocused_updates_increment_counter_only() {
    let out = run_headless("/inspector activity;/injectevent relay pulled;/exit");
    assert!(
        out.contains("event=tui_activity_view")
            && out.contains("sections=ledger,commands")
            && out.contains("total=1")
            && out.contains("visible=0")
            && out.contains("unread=1"),
        "missing activity buffered counter invariant: {}",
        out
    );
}

#[test]
fn status_locked_state_is_redacted() {
    let out = run_headless("/lock;/inspector status;/exit");
    assert!(
        out.contains("event=tui_lock_state")
            && out.contains("locked=LOCKED")
            && out.contains("event=tui_locked_shell")
            && out.contains("main=locked")
            && out.contains("cmd=/unlock"),
        "missing locked-shell redaction marker: {}",
        out
    );
}
