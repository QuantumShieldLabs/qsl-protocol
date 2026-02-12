use assert_cmd::Command as AssertCommand;

fn run_headless(script: &str, cols: &str, rows: &str) -> String {
    let mut cmd = AssertCommand::new(assert_cmd::cargo::cargo_bin!("qsc"));
    let out = cmd
        .env("QSC_TUI_HEADLESS", "1")
        .env("QSC_TUI_TEST_UNLOCK", "1")
        .env("QSC_TUI_SCRIPT", script)
        .env("QSC_TUI_COLS", cols)
        .env("QSC_TUI_ROWS", rows)
        .env("QSC_MARK_FORMAT", "plain")
        .args(["tui"])
        .output()
        .expect("run tui headless");
    let mut combined = String::from_utf8_lossy(&out.stdout).to_string();
    combined.push_str(&String::from_utf8_lossy(&out.stderr));
    combined
}

#[test]
fn home_render_reports_unified_nav_and_full_cmdbar() {
    let combined = run_headless("/inspector status;/exit", "120", "32");
    assert!(
        combined.contains("event=tui_render mode=home layout=h3")
            && combined.contains(" nav=shown "),
        "missing nav marker: {}",
        combined
    );
    assert!(
        combined.contains("event=tui_render mode=home layout=h3")
            && combined.contains("cmdbar=full"),
        "missing cmdbar marker: {}",
        combined
    );
}

#[test]
fn home_render_keeps_single_expanded_domain() {
    let combined = run_headless("/inspector events;/inspector contacts;/exit", "120", "32");
    let expanded_lines: Vec<&str> = combined
        .lines()
        .filter(|line| line.contains("event=tui_render mode=home layout=h3"))
        .filter(|line| line.contains(" expanded="))
        .collect();
    assert!(
        !expanded_lines.is_empty(),
        "expected home render lines: {}",
        combined
    );
    for line in expanded_lines {
        let expanded_count = [
            "expanded=events",
            "expanded=status",
            "expanded=session",
            "expanded=contacts",
        ]
        .iter()
        .filter(|tok| line.contains(**tok))
        .count();
        assert_eq!(
            expanded_count, 1,
            "line should have one expanded domain: {}",
            line
        );
    }
}

#[test]
fn tab_cycles_home_focus_markers() {
    let combined = run_headless("/key tab;/key tab;/key tab;/exit", "120", "32");
    let focus_lines: Vec<&str> = combined
        .lines()
        .filter(|line| line.contains("event=tui_focus_home"))
        .collect();
    assert!(
        focus_lines.len() >= 3,
        "expected focus cycle markers, got: {}",
        combined
    );
    assert!(
        focus_lines.iter().any(|l| l.contains("pane=command")),
        "missing command focus marker: {}",
        combined
    );
}
