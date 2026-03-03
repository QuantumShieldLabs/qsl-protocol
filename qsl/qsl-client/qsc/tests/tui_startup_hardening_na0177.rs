use assert_cmd::Command as AssertCommand;

fn has_long_hex(text: &str, min_len: usize) -> bool {
    let mut run = 0usize;
    for ch in text.chars() {
        if ch.is_ascii_hexdigit() {
            run += 1;
            if run >= min_len {
                return true;
            }
        } else {
            run = 0;
        }
    }
    false
}

#[test]
fn tui_non_tty_fails_with_deterministic_marker() {
    let out = AssertCommand::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env_remove("QSC_TUI_HEADLESS")
        .env_remove("QSC_TUI_TEST_MODE")
        .env("QSC_MARK_FORMAT", "plain")
        .args(["tui"])
        .output()
        .expect("run tui non-tty");

    assert_eq!(out.status.code(), Some(2), "unexpected exit code");
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);
    let mut combined = stdout.to_string();
    combined.push_str(&stderr);

    assert!(
        stderr.contains("QSC_TUI_STARTUP FAIL code=stdin_not_tty"),
        "missing deterministic startup failure marker: {combined}"
    );
    assert!(
        stderr.contains("HINT: run in an interactive terminal"),
        "missing deterministic startup hint: {combined}"
    );
    assert!(!combined.contains("/v1/"), "unexpected /v1/ in output");
    assert!(
        !has_long_hex(&combined, 32),
        "unexpected long hex in output: {combined}"
    );
}

#[test]
fn tui_headless_env_bypasses_tty_preflight_and_is_deterministic() {
    let out = AssertCommand::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_TUI_HEADLESS", "1")
        .env("QSC_TUI_SCRIPT", "/exit")
        .env("QSC_MARK_FORMAT", "plain")
        .args(["tui"])
        .output()
        .expect("run tui headless");

    assert!(out.status.success(), "headless mode should succeed");
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);
    let mut combined = stdout.to_string();
    combined.push_str(&stderr);

    assert!(
        combined.contains("QSC_TUI_STARTUP OK mode=headless"),
        "missing deterministic headless startup marker: {combined}"
    );
    assert!(!combined.contains("/v1/"), "unexpected /v1/ in output");
    assert!(
        !has_long_hex(&combined, 32),
        "unexpected long hex in output: {combined}"
    );
}
