use assert_cmd::Command as AssertCommand;

fn run_headless(script: &str, unlocked: bool) -> String {
    let mut cmd = AssertCommand::new(assert_cmd::cargo::cargo_bin!("qsc"));
    let out = cmd
        .env("QSC_TUI_HEADLESS", "1")
        .env("QSC_TUI_TEST_UNLOCK", if unlocked { "1" } else { "0" })
        .env("QSC_TUI_DETERMINISTIC", "1")
        .env("QSC_TUI_SCRIPT", script)
        .env("QSC_TUI_COLS", "120")
        .env("QSC_TUI_ROWS", "30")
        .env("QSC_MARK_FORMAT", "plain")
        .args(["tui"])
        .output()
        .expect("run tui headless");
    let mut combined = String::from_utf8_lossy(&out.stdout).to_string();
    combined.push_str(&String::from_utf8_lossy(&out.stderr));
    combined
}

#[test]
fn focus_glyph_moves_with_focus() {
    let out = run_headless("/key tab;/key tab;/exit", true);
    assert!(
        out.contains("focus_glyph=nav"),
        "missing nav focus glyph marker: {}",
        out
    );
    assert!(
        out.contains("focus_glyph=main"),
        "missing main focus glyph marker: {}",
        out
    );
    assert!(
        out.contains("focus_glyph=command"),
        "missing command focus glyph marker: {}",
        out
    );
}

#[test]
fn main_and_cmd_have_inner_padding() {
    let out = run_headless("/inspector help;/key tab;/key tab;/exit", true);
    assert!(
        out.contains("main_first_line_padded=__Help"),
        "main panel did not render with expected left padding: {}",
        out
    );
    assert!(
        out.contains("cmdbar_padded=__"),
        "cmd panel did not render with expected left padding: {}",
        out
    );
}

#[test]
fn focus_glyph_is_static_without_timer_markers() {
    let out = run_headless("wait 25;wait 25;/exit", true);
    assert!(
        out.contains("focus_glyph=nav"),
        "missing static glyph marker: {}",
        out
    );
    assert!(
        !out.contains("blink") && !out.contains("pulse") && !out.contains("ticker"),
        "unexpected timer-style glyph behavior markers present: {}",
        out
    );
}
