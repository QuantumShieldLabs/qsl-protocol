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

fn init_vault(cfg: &Path, passphrase: &str) {
    std::fs::create_dir_all(cfg).expect("create cfg");
    let out = AssertCommand::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", cfg)
        .env("QSC_DISABLE_KEYCHAIN", "1")
        .env("QSC_PASSPHRASE", passphrase)
        .args([
            "vault",
            "init",
            "--non-interactive",
            "--passphrase-env",
            "QSC_PASSPHRASE",
        ])
        .output()
        .expect("vault init");
    assert!(out.status.success(), "vault init failed");
}

fn run_headless(cfg: &Path, script: &str) -> String {
    let out = AssertCommand::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", cfg)
        .env("QSC_TUI_HEADLESS", "1")
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

#[test]
fn unlock_focus_defaults_to_nav_and_lock_selected() {
    let cfg = unique_cfg_dir("na0131_unlock_focus_lock_default");
    init_vault(&cfg, "StrongPassphrase1234");
    let out = run_headless(&cfg, "/unlock StrongPassphrase1234;/exit");
    assert!(
        out.contains("event=tui_lock_state locked=UNLOCKED reason=explicit_command"),
        "unlock transition marker missing: {}",
        out
    );
    assert!(
        out.contains("event=tui_render mode=home layout=h3 inspector=lock")
            && out.contains("focus=nav"),
        "post-unlock should land on nav with lock inspector: {}",
        out
    );
    assert!(
        out.contains("event=tui_nav_render selected_markers=1 selected_index=7"),
        "lock header should be selected in nav after unlock: {}",
        out
    );
}

#[test]
fn lock_view_is_clean_and_includes_autolock() {
    let cfg = unique_cfg_dir("na0131_lock_view_clean");
    init_vault(&cfg, "StrongPassphrase1234");
    let out = run_headless(&cfg, "/unlock StrongPassphrase1234;/inspector lock;/exit");
    assert!(
        out.contains("event=tui_lock_view")
            && out.contains("title=Lock Status")
            && out.contains("state=UNLOCKED")
            && out.contains("autolock_minutes=10"),
        "lock view marker missing clean status fields: {}",
        out
    );
    assert!(
        !out.contains("focus_steal: disabled") && !out.contains("Commands (command bar only)"),
        "legacy noisy lock-view text still present: {}",
        out
    );
}

#[test]
fn lock_transition_full_redraw_removes_stale_text() {
    let cfg = unique_cfg_dir("na0131_lock_full_redraw");
    init_vault(&cfg, "StrongPassphrase1234");
    let out = run_headless(
        &cfg,
        "/unlock StrongPassphrase1234;/inspector status;/lock;/exit",
    );
    assert!(
        out.contains("event=tui_buffer_clear ok=true reason=explicit_command")
            && out.contains("event=tui_lock_state locked=LOCKED reason=explicit_command"),
        "lock transition did not use shared clear/full-redraw path: {}",
        out
    );
    assert!(
        out.contains("event=tui_locked_shell")
            && out.contains("main=locked")
            && !out.contains("ed - unlock required"),
        "locked shell should render cleanly without stale partial remnants: {}",
        out
    );
}
