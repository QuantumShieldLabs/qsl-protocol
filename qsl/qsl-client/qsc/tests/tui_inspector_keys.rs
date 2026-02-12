use assert_cmd::Command as AssertCommand;

fn has_any_secret_markers(text: &str) -> bool {
    [
        "TOKEN",
        "SECRET",
        "KEY",
        "PASS",
        "PRIVATE",
        "BEARER",
        "CREDENTIAL",
    ]
    .iter()
    .any(|pat| text.contains(pat))
}

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
fn ins_alias_switches_inspector_headless() {
    let combined = run_headless("/ins events;/exit");
    assert!(
        combined.contains("event=tui_cmd cmd=inspector"),
        "missing inspector command marker: {}",
        combined
    );
    assert!(
        combined.contains("event=tui_inspector pane=events"),
        "missing inspector pane switch marker: {}",
        combined
    );
    assert!(
        combined.contains("event=tui_render mode=home layout=h3 inspector=events"),
        "missing inspector render marker: {}",
        combined
    );
}

#[test]
fn plain_f2_switches_inspector_not_focus() {
    let combined = run_headless("/key f2;/exit");
    assert!(
        combined.contains("event=tui_inspector pane=events"),
        "plain f2 should switch inspector: {}",
        combined
    );
    assert!(
        !combined.contains("event=tui_focus pane=events on=true"),
        "plain f2 must not enter focus: {}",
        combined
    );
}

#[test]
fn ctrl_fkeys_jump_to_focus_headless() {
    let combined = run_headless("/key ctrl-f2;/key ctrl-f3;/key ctrl-f4;/key ctrl-f5;/exit");
    for pane in ["events", "status", "session", "contacts"] {
        let marker = format!("event=tui_focus pane={pane} on=true");
        assert!(
            combined.contains(&marker),
            "missing focus marker: {marker}\n{combined}"
        );
    }
}

#[test]
fn no_secrets_in_tui_key_outputs() {
    let combined = run_headless("/ins status;/key ctrl-f2;/exit");
    assert!(
        !has_any_secret_markers(&combined),
        "secret-like token leaked: {}",
        combined
    );
}
