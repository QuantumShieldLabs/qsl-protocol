use assert_cmd::Command as AssertCommand;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

fn unique_cfg_dir(tag: &str) -> PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock")
        .as_nanos();
    std::env::current_dir()
        .expect("cwd")
        .join("target")
        .join("qsc-tests")
        .join(format!("{tag}-{}-{nonce}", std::process::id()))
}

fn run_headless(cfg: &Path, script: &str, test_unlock: bool) -> String {
    let mut cmd = AssertCommand::new(assert_cmd::cargo::cargo_bin!("qsc"));
    cmd.env("QSC_CONFIG_DIR", cfg)
        .env("QSC_TUI_HEADLESS", "1")
        .env("QSC_TUI_SCRIPT", script)
        .env("QSC_TUI_COLS", "140")
        .env("QSC_TUI_ROWS", "40")
        .env("QSC_MARK_FORMAT", "plain");
    if test_unlock {
        cmd.env("QSC_TUI_TEST_UNLOCK", "1");
    }
    let out = cmd.args(["tui"]).output().expect("run tui headless");
    let mut combined = String::from_utf8_lossy(&out.stdout).to_string();
    combined.push_str(&String::from_utf8_lossy(&out.stderr));
    combined
}

#[test]
fn autolock_triggers_after_inactivity() {
    let cfg = unique_cfg_dir("na0130_autolock_trigger");
    std::fs::create_dir_all(&cfg).expect("create cfg");
    let out = run_headless(&cfg, "wait 600001;/status;/exit", true);
    assert!(
        out.contains("event=tui_autolock ok=true minutes=10")
            && out.contains("event=tui_lock_state locked=LOCKED reason=inactivity_timeout"),
        "autolock did not transition to locked state: {}",
        out
    );
}

#[test]
fn autolock_resets_on_keypress() {
    let cfg = unique_cfg_dir("na0130_autolock_reset");
    std::fs::create_dir_all(&cfg).expect("create cfg");
    let out = run_headless(
        &cfg,
        "wait 300000;/key down;wait 350000;/status;/exit",
        true,
    );
    assert!(
        !out.contains("event=tui_autolock ok=true"),
        "autolock should not trigger after keypress reset: {}",
        out
    );
    assert!(
        out.contains("event=tui_status_view locked=UNLOCKED"),
        "status view should remain unlocked: {}",
        out
    );
}

#[test]
fn lock_clears_rendered_buffers() {
    let cfg = unique_cfg_dir("na0130_lock_clear");
    std::fs::create_dir_all(&cfg).expect("create cfg");
    let out = run_headless(
        &cfg,
        "/inspector events;/messages select peer-0;/injectmsg peer-0 RECEIVED;/lock;/exit",
        true,
    );
    assert!(
        out.contains("event=tui_buffer_clear ok=true reason=explicit_command")
            && out.contains("event=tui_lock_state locked=LOCKED reason=explicit_command"),
        "explicit lock did not clear buffers on lock path: {}",
        out
    );
    assert!(
        !out.contains("No messages yet for peer-0."),
        "plaintext fixture leaked after lock transition: {}",
        out
    );
}

#[test]
fn locked_shell_still_allows_exit() {
    let cfg = unique_cfg_dir("na0130_locked_exit");
    std::fs::create_dir_all(&cfg).expect("create cfg");
    let out = run_headless(&cfg, "/exit", false);
    assert!(
        out.contains("event=tui_locked_shell") && out.contains("event=tui_cmd cmd=exit"),
        "locked shell did not accept /exit: {}",
        out
    );
    assert!(
        out.contains("event=tui_exit"),
        "missing tui_exit marker: {}",
        out
    );
}
