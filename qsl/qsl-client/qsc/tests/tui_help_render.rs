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
fn tui_help_renders_command_list_headless() {
    let mut cmd = AssertCommand::new(assert_cmd::cargo::cargo_bin!("qsc"));
    let assert = cmd
        .env("QSC_TUI_HEADLESS", "1")
        .env("QSC_TUI_SCRIPT", "/help;/exit")
        .env("QSC_MARK_FORMAT", "plain")
        .args(["tui"])
        .assert();

    let output = assert.get_output();
    let mut combined = String::from_utf8_lossy(&output.stdout).to_string();
    combined.push_str(&String::from_utf8_lossy(&output.stderr));

    assert!(
        combined.contains("tui_cmd") && combined.contains("cmd=help"),
        "missing tui_cmd help marker: {}",
        combined
    );
    assert!(
        combined.contains("tui_help_rendered"),
        "missing tui_help_rendered marker: {}",
        combined
    );

    for cmd_name in ["help", "exit", "send", "status", "envelope", "export"] {
        assert!(
            combined.contains("tui_help_item") && combined.contains(&format!("cmd={}", cmd_name)),
            "missing help item for {}: {}",
            cmd_name,
            combined
        );
    }

    assert!(
        !has_any_secret_markers(&combined),
        "secret-like tokens found in output: {}",
        combined
    );
}

#[test]
fn tui_help_rendered_is_deterministic() {
    let mut cmd_a = AssertCommand::new(assert_cmd::cargo::cargo_bin!("qsc"));
    let out_a = cmd_a
        .env("QSC_TUI_HEADLESS", "1")
        .env("QSC_TUI_SCRIPT", "/help;/exit")
        .env("QSC_MARK_FORMAT", "plain")
        .args(["tui"])
        .output()
        .expect("run tui help a");

    let mut cmd_b = AssertCommand::new(assert_cmd::cargo::cargo_bin!("qsc"));
    let out_b = cmd_b
        .env("QSC_TUI_HEADLESS", "1")
        .env("QSC_TUI_SCRIPT", "/help;/exit")
        .env("QSC_MARK_FORMAT", "plain")
        .args(["tui"])
        .output()
        .expect("run tui help b");

    let mut a = String::from_utf8_lossy(&out_a.stdout).to_string();
    a.push_str(&String::from_utf8_lossy(&out_a.stderr));
    let mut b = String::from_utf8_lossy(&out_b.stdout).to_string();
    b.push_str(&String::from_utf8_lossy(&out_b.stderr));

    assert!(a.contains("tui_help_rendered"));
    assert!(b.contains("tui_help_rendered"));

    let a_lines: Vec<&str> = a
        .lines()
        .filter(|l| {
            l.contains("tui_help_item") || l.contains("tui_help_rendered") || l.contains("tui_cmd")
        })
        .collect();
    let b_lines: Vec<&str> = b
        .lines()
        .filter(|l| {
            l.contains("tui_help_item") || l.contains("tui_help_rendered") || l.contains("tui_cmd")
        })
        .collect();

    assert_eq!(a_lines, b_lines, "help markers not deterministic");
}
