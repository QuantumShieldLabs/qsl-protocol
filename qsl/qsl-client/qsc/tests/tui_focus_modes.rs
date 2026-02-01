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

fn run_focus_headless() -> String {
    let mut cmd = AssertCommand::new(assert_cmd::cargo::cargo_bin!("qsc"));
    let assert = cmd
        .env("QSC_TUI_HEADLESS", "1")
        .env(
            "QSC_TUI_SCRIPT",
            "/focus events;/back;/focus status;/back;/focus session;/back;/focus contacts;/back;/exit",
        )
        .env("QSC_MARK_FORMAT", "plain")
        .args(["tui"])
        .assert();

    let output = assert.get_output();
    let mut combined = String::from_utf8_lossy(&output.stdout).to_string();
    combined.push_str(&String::from_utf8_lossy(&output.stderr));
    combined
}

#[test]
fn focus_mode_switches_headless() {
    let combined = run_focus_headless();

    for pane in ["events", "status", "session", "contacts"] {
        let on = format!("QSC_MARK/1 event=tui_focus pane={pane} on=true");
        let off = format!("QSC_MARK/1 event=tui_focus pane={pane} on=false");
        assert!(combined.contains(&on), "missing focus on marker: {}", on);
        assert!(combined.contains(&off), "missing focus off marker: {}", off);
    }
    assert!(
        !has_any_secret_markers(&combined),
        "secret-like tokens found in output: {}",
        combined
    );
}

#[test]
fn focus_mode_deterministic() {
    let a = run_focus_headless();
    let b = run_focus_headless();

    let filter = |s: &str| {
        s.lines()
            .filter(|line| line.contains("event=tui_focus"))
            .collect::<Vec<_>>()
            .join("\n")
    };
    let fa = filter(&a);
    let fb = filter(&b);
    assert_eq!(
        fa, fb,
        "focus markers not deterministic\nA:\n{}\nB:\n{}",
        fa, fb
    );
}
