use assert_cmd::Command as AssertCommand;
use std::path::{Path, PathBuf};

fn safe_test_root() -> PathBuf {
    std::env::temp_dir().join("qsc-tests")
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

fn init_passphrase_vault(cfg: &Path) {
    let out = AssertCommand::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", cfg)
        .env("QSC_PASSPHRASE", "StrongPassphrase123!")
        .args([
            "vault",
            "init",
            "--non-interactive",
            "--key-source",
            "passphrase",
            "--passphrase-env",
            "QSC_PASSPHRASE",
        ])
        .output()
        .expect("vault init passphrase");
    assert!(
        out.status.success(),
        "vault init failed: {}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
}

#[test]
fn locked_cmd_focus_echo_and_cursor() {
    let cfg = safe_test_root().join(format!("na0131_cmd_echo_{}", std::process::id()));
    std::fs::create_dir_all(&cfg).expect("create cfg");

    let out = run_headless(
        &cfg,
        "/key slash;/key e;/key x;/key i;/key t;/key esc;/exit",
    );
    assert!(
        out.contains("cmdbar_text=Cmd: /exit█"),
        "missing command echo/cursor in locked shell markers: {}",
        out
    );
    assert!(
        out.contains("cmdbar_text=Cmd:")
            && out.contains("event=tui_focus_home pane=nav")
            && !out.contains("cmdbar_text=Cmd: /exit█ wizard=none focus=nav"),
        "esc must clear command input and return focus to nav: {}",
        out
    );
}

#[test]
fn locked_enter_on_exit_exits() {
    let cfg = safe_test_root().join(format!("na0131_enter_exit_{}", std::process::id()));
    std::fs::create_dir_all(&cfg).expect("create cfg");

    let out = run_headless(&cfg, "/key down;/key enter");
    assert!(
        out.contains("event=tui_nav_select domain=locked label=exit")
            && out.contains("event=tui_exit"),
        "enter on Exit must close TUI in locked shell: {}",
        out
    );
}

#[test]
fn locked_disables_ctrl_f_focus() {
    let cfg = safe_test_root().join(format!("na0131_ctrlf_blocked_{}", std::process::id()));
    std::fs::create_dir_all(&cfg).expect("create cfg");

    let out = run_headless(
        &cfg,
        "/key ctrl-f2;/key ctrl-f3;/key ctrl-f4;/key ctrl-f5;/exit",
    );
    assert!(
        !out.contains("event=tui_focus pane=events on=true")
            && !out.contains("event=tui_focus pane=status on=true")
            && !out.contains("event=tui_focus pane=session on=true")
            && !out.contains("event=tui_focus pane=contacts on=true"),
        "ctrl+F* focus shortcuts must be disabled while locked: {}",
        out
    );
}

#[test]
fn init_wizard_is_visible() {
    let cfg = safe_test_root().join(format!("na0131_init_wizard_{}", std::process::id()));
    std::fs::create_dir_all(&cfg).expect("create cfg");

    let out = run_headless(&cfg, "/init;/key t;/key e;/key s;/key t;/key enter;/exit");
    assert!(
        out.contains("event=tui_init_wizard step=alias")
            && out.contains("event=tui_init_wizard step=ack"),
        "init wizard must advance visibly from alias to ack: {}",
        out
    );
}

#[test]
fn no_leak_pre_unlock_preserved() {
    let cfg = safe_test_root().join(format!("na0131_zero_leak_{}", std::process::id()));
    std::fs::create_dir_all(&cfg).expect("create cfg");
    init_passphrase_vault(&cfg);

    let out = run_headless(&cfg, "/exit");
    assert!(
        out.contains("event=tui_locked_shell")
            && out.contains("vault=present")
            && out.contains("nav=unlock,exit")
            && out.contains("main=locked")
            && out.contains("cmdbar_text=Cmd:"),
        "locked shell markers missing required zero-leak structure: {}",
        out
    );
    for forbidden in [
        "Messages", "Contacts", "Files", "Keys", "Activity", "Status", "Settings",
    ] {
        assert!(
            !out.contains(forbidden),
            "forbidden text leaked while locked: {} in {}",
            forbidden,
            out
        );
    }
}
