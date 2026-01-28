use assert_cmd::cargo::cargo_bin_cmd;

fn run_tui_script(script: &str) -> String {
    let mut cmd = cargo_bin_cmd!("qsc");
    let output = cmd
        .arg("tui")
        .env("QSC_TUI_HEADLESS", "1")
        .env("QSC_TUI_SCRIPT", script)
        .env("QSC_MARK_FORMAT", "plain")
        .output()
        .expect("run tui");
    assert!(output.status.success());
    let mut out = String::new();
    out.push_str(&String::from_utf8_lossy(&output.stdout));
    out.push_str(&String::from_utf8_lossy(&output.stderr));
    out
}

#[test]
fn tui_does_not_send_without_explicit_command() {
    let out = run_tui_script("hello world\n/exit\n");
    assert!(out.contains("event=tui_open"));
    assert!(out.contains("event=tui_input_text"));
    assert!(out.contains("event=tui_exit"));
    assert!(!out.contains("tui_send"));
    assert!(!out.contains("retry"));
}

#[test]
fn tui_markers_are_deterministic() {
    let script = "/status\n/exit\n";
    let first = run_tui_script(script);
    let second = run_tui_script(script);
    assert_eq!(first, second);
}

#[test]
fn tui_no_secrets_in_output() {
    let out = run_tui_script("secret-passphrase\n/exit\n");
    assert!(!out.to_lowercase().contains("secret"));
    assert!(!out.to_lowercase().contains("passphrase"));
}
