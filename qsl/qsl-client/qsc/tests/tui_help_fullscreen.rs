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

fn run_help_headless() -> String {
    let mut cmd = AssertCommand::new(assert_cmd::cargo::cargo_bin!("qsc"));
    let assert = cmd
        .env("QSC_TUI_HEADLESS", "1")
        .env("QSC_TUI_TEST_UNLOCK", "1")
        .env("QSC_TUI_SCRIPT", "/help;/exithelp;/exit")
        .env("QSC_MARK_FORMAT", "plain")
        .args(["tui"])
        .assert();

    let output = assert.get_output();
    let mut combined = String::from_utf8_lossy(&output.stdout).to_string();
    combined.push_str(&String::from_utf8_lossy(&output.stderr));
    combined
}

#[test]
fn help_mode_renders_fullscreen_headless() {
    let combined = run_help_headless();

    assert!(
        combined.contains("QSC_MARK/1 event=tui_help_mode on=true"),
        "help mode marker missing: {}",
        combined
    );
    assert!(
        combined.contains("QSC_MARK/1 event=tui_help_rendered"),
        "help render marker missing: {}",
        combined
    );
    assert!(
        combined.contains("QSC_MARK/1 event=tui_help_item"),
        "help items missing: {}",
        combined
    );
    assert!(
        !has_any_secret_markers(&combined),
        "secret-like tokens found in output: {}",
        combined
    );
}

#[test]
fn help_mode_deterministic() {
    let a = run_help_headless();
    let b = run_help_headless();

    let filter = |s: &str| {
        s.lines()
            .filter(|line| line.contains("event=tui_help_") || line.contains("event=tui_help_mode"))
            .collect::<Vec<_>>()
            .join("\n")
    };
    let fa = filter(&a);
    let fb = filter(&b);
    assert_eq!(
        fa, fb,
        "help markers not deterministic\nA:\n{}\nB:\n{}",
        fa, fb
    );
}
