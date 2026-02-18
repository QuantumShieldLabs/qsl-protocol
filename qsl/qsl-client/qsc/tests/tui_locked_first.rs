use assert_cmd::Command as AssertCommand;
use std::path::Path;
use std::path::PathBuf;

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
fn locked_view_leaks_nothing() {
    let cfg = safe_test_root().join(format!("na0128_locked_view_{}", std::process::id()));
    std::fs::create_dir_all(&cfg).expect("create cfg");
    init_passphrase_vault(&cfg);

    let out = run_headless(&cfg, "/exit");
    assert!(
        out.contains("event=tui_locked_shell")
            && out.contains("vault=present")
            && out.contains("nav=unlock,exit")
            && out.contains("cmd=/unlock"),
        "missing locked shell marker: {}",
        out
    );
    for forbidden in [
        "missing_seed",
        "Messages",
        "Contacts",
        "Files",
        "Keys",
        "Activity",
        "Status",
        "Settings",
        "(1)",
    ] {
        assert!(
            !out.contains(forbidden),
            "forbidden text leaked while locked: {} in {}",
            forbidden,
            out
        );
    }
}

#[test]
fn first_run_no_vault_shows_init_prompt_only() {
    let cfg = safe_test_root().join(format!("na0128_no_vault_{}", std::process::id()));
    std::fs::create_dir_all(&cfg).expect("create cfg");

    let out = run_headless(&cfg, "/exit");
    assert!(
        out.contains("event=tui_locked_shell")
            && out.contains("vault=missing")
            && out.contains("main=init_required")
            && out.contains("cmd=/init"),
        "missing no-vault locked marker: {}",
        out
    );
}

#[test]
fn locked_rejects_help_and_other_cmds() {
    let cfg = safe_test_root().join(format!("na0128_locked_reject_{}", std::process::id()));
    std::fs::create_dir_all(&cfg).expect("create cfg");

    let out = run_headless(&cfg, "/help;/status;/exit");
    assert!(
        out.contains("event=tui_locked_cmd_reject code=locked_unlock_required cmd=help"),
        "missing help reject marker: {}",
        out
    );
    assert!(
        out.contains("event=tui_locked_cmd_reject code=locked_unlock_required cmd=status"),
        "missing status reject marker: {}",
        out
    );
}

#[test]
fn init_requires_acknowledgement() {
    let cfg = safe_test_root().join(format!("na0128_init_ack_{}", std::process::id()));
    std::fs::create_dir_all(&cfg).expect("create cfg");
    let vault_path = cfg.join("vault.qsv");
    assert!(
        !vault_path.exists(),
        "vault unexpectedly exists before test"
    );

    let out = run_headless(
        &cfg,
        "/init alias-1 StrongPassphrase123! StrongPassphrase123! I DISAGREE;/exit",
    );
    assert!(
        out.contains("event=tui_init_warning")
            && out.contains("event=tui_init_reject code=confirm_required"),
        "missing confirm reject marker: {}",
        out
    );
    assert!(
        !vault_path.exists(),
        "vault mutated on ack reject: {}",
        vault_path.display()
    );
}
