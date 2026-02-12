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

#[test]
fn headless_still_emits_markers() {
    let mut cmd = AssertCommand::new(assert_cmd::cargo::cargo_bin!("qsc"));
    let assert = cmd
        .env("QSC_TUI_HEADLESS", "1")
        .env("QSC_TUI_TEST_UNLOCK", "1")
        .env("QSC_TUI_SCRIPT", "/help;/exit")
        .env("QSC_MARK_FORMAT", "plain")
        .args(["tui"])
        .assert();

    let output = assert.get_output();
    let mut combined = String::from_utf8_lossy(&output.stdout).to_string();
    combined.push_str(&String::from_utf8_lossy(&output.stderr));

    assert!(
        combined.contains("QSC_MARK/1 event=tui_cmd") && combined.contains("tui_help_item"),
        "headless markers missing: {}",
        combined
    );
    assert!(
        !has_any_secret_markers(&combined),
        "secret-like tokens found in output: {}",
        combined
    );
}

#[test]
fn interactive_does_not_emit_markers_to_stdout() {
    let mut cmd = AssertCommand::new(assert_cmd::cargo::cargo_bin!("qsc"));
    let assert = cmd
        .env("QSC_TUI_TEST_MODE", "1")
        .env("QSC_TUI_HEADLESS", "0")
        .env("QSC_MARK_FORMAT", "plain")
        .args(["tui"])
        .assert();

    let output = assert.get_output();
    let mut combined = String::from_utf8_lossy(&output.stdout).to_string();
    combined.push_str(&String::from_utf8_lossy(&output.stderr));

    assert!(
        !combined.contains("QSC_MARK/1"),
        "interactive mode printed markers: {}",
        combined
    );
    assert!(combined.contains("tui_test_done"));
    assert!(
        !has_any_secret_markers(&combined),
        "secret-like tokens found in output: {}",
        combined
    );
}
