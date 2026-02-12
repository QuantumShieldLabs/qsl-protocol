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

fn run_h3_headless(script: &str, cols: &str, rows: &str) -> String {
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
        .expect("run tui h3 headless");
    let mut combined = String::from_utf8_lossy(&out.stdout).to_string();
    combined.push_str(&String::from_utf8_lossy(&out.stderr));
    combined
}

#[test]
fn h3_renders_status_inspector_headless() {
    let combined = run_h3_headless("/inspector status;/exit", "140", "40");
    assert!(
        combined.contains("event=tui_render mode=home layout=h3 inspector=status"),
        "missing h3 status render marker: {}",
        combined
    );
}

#[test]
fn h3_renders_events_inspector_headless() {
    let combined = run_h3_headless("/inspector events;/exit", "140", "40");
    assert!(
        combined.contains("event=tui_render mode=home layout=h3 inspector=events"),
        "missing h3 events render marker: {}",
        combined
    );
}

#[test]
fn h3_renders_session_inspector_headless() {
    let combined = run_h3_headless("/inspector session;/exit", "140", "40");
    assert!(
        combined.contains("event=tui_render mode=home layout=h3 inspector=session"),
        "missing h3 session render marker: {}",
        combined
    );
}

#[test]
fn h3_responsive_hides_contacts_on_narrow() {
    let combined = run_h3_headless("/inspector contacts;/exit", "90", "40");
    assert!(
        combined
            .contains("event=tui_render mode=home layout=h3 inspector=contacts contacts=hidden"),
        "missing narrow hidden-contacts marker: {}",
        combined
    );
}

#[test]
fn h3_deterministic_markers_across_runs() {
    let a = run_h3_headless("/inspector events;/inspector status;/exit", "140", "40");
    let b = run_h3_headless("/inspector events;/inspector status;/exit", "140", "40");
    let filter = |s: &str| {
        s.lines()
            .filter(|line| {
                line.contains("event=tui_inspector") || line.contains("event=tui_render")
            })
            .collect::<Vec<_>>()
            .join("\n")
    };
    assert_eq!(filter(&a), filter(&b), "h3 markers not deterministic");
    assert!(
        !has_any_secret_markers(&a),
        "secret-like token leaked: {}",
        a
    );
    assert!(
        !has_any_secret_markers(&b),
        "secret-like token leaked: {}",
        b
    );
}
