use assert_cmd::Command as AssertCommand;
use std::path::Path;
use std::path::PathBuf;

fn safe_test_root() -> PathBuf {
    std::env::temp_dir().join("qsc-tests")
}

fn run_headless(script: &str) -> String {
    let mut cmd = AssertCommand::new(assert_cmd::cargo::cargo_bin!("qsc"));
    let out = cmd
        .env("QSC_TUI_HEADLESS", "1")
        .env("QSC_TUI_TEST_UNLOCK", "1")
        .env("QSC_TUI_SCRIPT", script)
        .env("QSC_TUI_COLS", "140")
        .env("QSC_TUI_ROWS", "40")
        .env("QSC_MARK_FORMAT", "plain")
        .args(["tui"])
        .output()
        .expect("run tui headless");
    let mut combined = String::from_utf8_lossy(&out.stdout).to_string();
    combined.push_str(&String::from_utf8_lossy(&out.stderr));
    combined
}

fn run_locked_headless(cfg: &Path, script: &str) -> String {
    let out = AssertCommand::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", cfg)
        .env("QSC_TUI_HEADLESS", "1")
        .env("QSC_TUI_SCRIPT", script)
        .env("QSC_TUI_COLS", "140")
        .env("QSC_TUI_ROWS", "40")
        .env("QSC_MARK_FORMAT", "plain")
        .args(["tui"])
        .output()
        .expect("run locked tui headless");
    let mut combined = String::from_utf8_lossy(&out.stdout).to_string();
    combined.push_str(&String::from_utf8_lossy(&out.stderr));
    combined
}

#[test]
fn unlocked_chrome_is_minimal_and_branded() {
    let out = run_headless("/exit");
    assert!(
        out.contains("event=tui_render mode=home layout=h3")
            && out.contains("nav_title=qsc")
            && out.contains("main_title=none")
            && out.contains("cmd_hint=help")
            && out.contains("chrome=single")
            && out.contains("outer_border=1")
            && out.contains("header_divider=0")
            && out.contains("v_divider=1")
            && out.contains("h_divider=1")
            && out.contains("divider_h_char=─")
            && out.contains("divider_v_char=│"),
        "missing minimal chrome marker fields: {}",
        out
    );
    assert!(
        !out.contains("Cmd (Tab focus /help F2-5 ins Ctrl+F2-5 focus Enter Esc")
            && !out.contains("Nav [focus]")
            && !out.contains("Main:"),
        "legacy chrome labels leaked: {}",
        out
    );
}

#[test]
fn locked_status_renders_in_main_not_header() {
    let out = run_headless("/lock;/exit");
    assert!(
        out.contains("event=tui_locked_shell")
            && out.contains("main_locked_line=Locked: unlock required")
            && out.contains("header=[ QSC ]")
            && !out.contains("header=Locked")
            && !out.contains("header=System")
            && !out.contains("header=Account")
            && out.contains("header_divider=0"),
        "locked status must remain in main content area while header stays chrome-only: {}",
        out
    );
}

#[test]
fn help_about_legal_are_available_only_when_unlocked() {
    let unlocked = run_headless("/inspector help;/inspector about;/inspector legal;/exit");
    assert!(
        unlocked.contains("event=tui_render mode=home layout=h3 inspector=help")
            && unlocked.contains("event=tui_render mode=home layout=h3 inspector=about")
            && unlocked.contains("event=tui_render mode=home layout=h3 inspector=legal"),
        "help/about/legal not rendered post-unlock: {}",
        unlocked
    );

    let cfg = safe_test_root().join(format!("na0129_locked_inspector_{}", std::process::id()));
    std::fs::create_dir_all(&cfg).expect("create cfg");
    let locked = run_locked_headless(&cfg, "/inspector help;/exit");
    assert!(
        locked.contains("event=tui_locked_shell")
            && locked.contains("nav=unlock,exit")
            && locked
                .contains("event=tui_locked_cmd_reject code=locked_unlock_required cmd=inspector")
            && !locked.contains("inspector=help"),
        "locked mode must keep help/about/legal unavailable: {}",
        locked
    );
}

#[test]
fn activity_view_does_not_ingest_internal_marker_lines() {
    let out = run_headless("/inspector activity;/injectevent relay pulled;/exit");
    assert!(
        out.contains("event=tui_activity_view")
            && out.contains("total=1")
            && out.contains("visible=0")
            && out.contains("unread=1"),
        "activity counters indicate unexpected marker-line ingestion: {}",
        out
    );
}
