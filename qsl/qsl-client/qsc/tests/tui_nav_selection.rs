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
fn nav_renders_exactly_one_selected_marker() {
    let out = run_headless("/inspector events;/exit");
    assert!(
        out.contains("event=tui_nav_render selected_markers=1"),
        "missing single selected marker invariant: {}",
        out
    );
}

#[test]
fn nav_arrow_keys_move_selection_deterministically() {
    let out = run_headless("/inspector events;/key down;/key up;/exit");
    assert!(
        out.contains("event=tui_nav_render selected_markers=1 selected_index=1"),
        "missing moved selection index marker: {}",
        out
    );
    assert!(
        out.contains("event=tui_nav_render selected_markers=1 selected_index=0"),
        "missing moved-back selection index marker: {}",
        out
    );
}

#[test]
fn enter_activates_selected_nav_item() {
    let out = run_headless("/inspector events;/key down;/key down;/key enter;/exit");
    assert!(
        out.contains("event=tui_nav_activate pane=files"),
        "expected Enter activation marker for files pane: {}",
        out
    );
    assert!(
        out.contains("event=tui_render mode=home layout=h3 inspector=files"),
        "expected files inspector render after Enter activation: {}",
        out
    );
}

#[test]
fn nav_enter_does_not_execute_command_actions() {
    let out = run_headless("/inspector events;/key enter;/exit");
    assert!(
        !out.contains("event=tui_cmd cmd=send")
            && !out.contains("event=tui_cmd cmd=receive")
            && !out.contains("event=tui_cmd cmd=handshake"),
        "nav Enter must not execute command actions: {}",
        out
    );
}
