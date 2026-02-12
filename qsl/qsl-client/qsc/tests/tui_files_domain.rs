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
fn files_view_renders_unified_nav_and_full_cmdbar() {
    let out = run_headless("/inspector files;/exit");
    assert!(
        out.contains("event=tui_render mode=home layout=h3 inspector=files")
            && out.contains("expanded=files")
            && out.contains("cmdbar=full"),
        "missing files home render markers: {}",
        out
    );
    assert!(
        out.contains("event=tui_files_view"),
        "missing files view marker: {}",
        out
    );
}

#[test]
fn files_multi_select_shows_selected_count() {
    let out = run_headless(
        "/inspector files;/files inject f1 RECEIVING 10 a.bin;/files inject f2 RECEIVING 11 b.bin;/files select f1;/files toggle;/files select f2;/files toggle;/exit",
    );
    assert!(
        out.contains("event=tui_files_view") && out.contains("selected_count=2"),
        "missing selected_count=2 marker: {}",
        out
    );
}

#[test]
fn files_truth_display_only_shows_verified_when_verified() {
    let pre = run_headless("/inspector files;/files inject f1 RECEIVING 10 a.bin;/exit");
    assert!(
        pre.contains("event=tui_files_view") && pre.contains("state=RECEIVING"),
        "expected receiving state marker: {}",
        pre
    );
    assert!(
        !pre.contains("state=VERIFIED"),
        "must not over-claim verified for pre-verified item: {}",
        pre
    );

    let verified = run_headless("/inspector files;/files inject f1 VERIFIED 10 a.bin;/exit");
    assert!(
        verified.contains("event=tui_files_view") && verified.contains("state=VERIFIED"),
        "expected verified marker for verified item: {}",
        verified
    );
}

#[test]
fn files_multiselect_is_blocked_outside_files_domain() {
    let out = run_headless(
        "/inspector events;/files inject f1 RECEIVING 10 a.bin;/files select f1;/files toggle;/exit",
    );
    assert!(
        out.contains("event=tui_files_multiselect_blocked")
            && out.contains("reason=domain_not_files"),
        "expected guardrail marker for non-files multi-select: {}",
        out
    );
}
