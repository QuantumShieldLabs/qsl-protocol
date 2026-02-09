use assert_cmd::Command as AssertCommand;

fn run_headless(script: &str, cols: &str, rows: &str) -> String {
    let mut cmd = AssertCommand::new(assert_cmd::cargo::cargo_bin!("qsc"));
    let out = cmd
        .env("QSC_TUI_HEADLESS", "1")
        .env("QSC_TUI_DETERMINISTIC", "1")
        .env("QSC_TUI_SCRIPT", script)
        .env("QSC_TUI_COLS", cols)
        .env("QSC_TUI_ROWS", rows)
        .env("QSC_MARK_FORMAT", "plain")
        .args(["tui"])
        .output()
        .expect("run tui readability headless");
    let mut combined = String::from_utf8_lossy(&out.stdout).to_string();
    combined.push_str(&String::from_utf8_lossy(&out.stderr));
    combined
}

fn has_any_secret_markers(text: &str) -> bool {
    [
        "TOKEN",
        "SECRET",
        "KEY",
        "PASS",
        "PRIVATE",
        "BEARER",
        "CREDENTIAL",
        "RELAY_TOKEN",
    ]
    .iter()
    .any(|pat| text.contains(pat))
}

#[test]
fn focus_events_includes_deterministic_timestamps() {
    let out = run_headless("/focus events;/down;/exit", "140", "40");
    assert!(
        out.contains("event=tui_render mode=focus") && out.contains("focus=events"),
        "missing focus events render marker: {}",
        out
    );
    assert!(
        out.contains("ts_start=t="),
        "missing ts_start token: {}",
        out
    );
    assert!(out.contains("ts_end=t="), "missing ts_end token: {}", out);
    assert!(
        out.contains("deterministic=true"),
        "focus render must be deterministic in headless mode: {}",
        out
    );
}

#[test]
fn focus_status_includes_deterministic_timestamps() {
    let out = run_headless("/focus status;/exit", "140", "40");
    assert!(
        out.contains("event=tui_render mode=focus") && out.contains("focus=status"),
        "missing focus status render marker: {}",
        out
    );
    assert!(
        out.contains("ts_start=t="),
        "missing status ts_start token: {}",
        out
    );
    assert!(
        out.contains("ts_end=t="),
        "missing status ts_end token: {}",
        out
    );
}

#[test]
fn focus_scroll_affects_render() {
    let out = run_headless("/focus status;/down;/exit", "140", "40");
    assert!(
        out.contains("event=tui_render mode=focus") && out.contains("focus=status"),
        "missing focus render marker: {}",
        out
    );
    assert!(
        out.contains("scroll=0"),
        "missing initial scroll=0 marker: {}",
        out
    );
    assert!(
        out.contains("scroll=1"),
        "missing scrolled marker scroll=1: {}",
        out
    );
    assert!(
        out.contains("viewport=full"),
        "missing viewport=full marker: {}",
        out
    );
    assert!(
        out.contains("view_rows="),
        "missing view_rows marker: {}",
        out
    );
}

#[test]
fn determinism_replay() {
    let script = "/focus events;/down;/focus status;/exit";
    let a = run_headless(script, "140", "40");
    let b = run_headless(script, "140", "40");

    let normalize = |s: &str| {
        s.lines()
            .filter(|line| {
                line.contains("event=tui_focus")
                    || (line.contains("event=tui_render mode=focus")
                        && (line.contains("focus=events") || line.contains("focus=status")))
            })
            .collect::<Vec<_>>()
            .join("\n")
    };

    assert_eq!(
        normalize(&a),
        normalize(&b),
        "focus determinism replay mismatch\nA:\n{}\nB:\n{}",
        normalize(&a),
        normalize(&b)
    );
}

#[test]
fn no_secrets_in_output_guard() {
    let out = run_headless("/focus events;/focus status;/exit", "140", "40");
    assert!(
        !has_any_secret_markers(&out),
        "secret-like token leaked in tui readability output: {}",
        out
    );
}

#[test]
fn interactive_no_qsc_mark_stdout_still_true() {
    let mut cmd = AssertCommand::new(assert_cmd::cargo::cargo_bin!("qsc"));
    let out = cmd
        .env("QSC_TUI_TEST_MODE", "1")
        .env("QSC_TUI_HEADLESS", "0")
        .env("QSC_MARK_FORMAT", "plain")
        .args(["tui"])
        .output()
        .expect("run tui interactive test mode");
    let mut combined = String::from_utf8_lossy(&out.stdout).to_string();
    combined.push_str(&String::from_utf8_lossy(&out.stderr));
    assert!(
        !combined.contains("QSC_MARK/1"),
        "interactive mode printed markers: {}",
        combined
    );
}
