use assert_cmd::Command as AssertCommand;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

const CONFIG_FILE: &str = "vault_security.txt";
const COUNTER_FILE: &str = "vault_unlock_failures.txt";

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

fn ensure_dir_700(path: &Path) {
    std::fs::create_dir_all(path).expect("create dir");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o700)).expect("chmod");
    }
}

fn run_headless(cfg: &Path, script: &str) -> String {
    let out = AssertCommand::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", cfg)
        .env("QSC_TUI_HEADLESS", "1")
        .env("QSC_DISABLE_KEYCHAIN", "1")
        .env("NO_COLOR", "1")
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
fn attempt_limit_disabled_does_not_wipe() {
    let cfg = unique_cfg_dir("na0157_attempt_limit_off");
    ensure_dir_700(&cfg);
    let out = run_headless(
        &cfg,
        "/init DemoUser StrongPassphrase1234 StrongPassphrase1234 I UNDERSTAND;/unlock StrongPassphrase1234;/lock;/unlock wrong-pass-1;/unlock wrong-pass-2;/exit",
    );
    assert!(
        out.contains("event=tui_unlock code=vault_locked ok=false")
            && !out.contains("QSC_ERR_VAULT_WIPED_AFTER_FAILED_UNLOCKS"),
        "disabled attempt-limit should reject without wipe: {}",
        out
    );
    assert!(
        cfg.join("vault.qsv").exists(),
        "vault must remain present when attempt-limit is disabled"
    );
}

#[test]
fn attempt_limit_enabled_wipes_exactly_at_threshold() {
    let cfg = unique_cfg_dir("na0157_attempt_limit_wipe");
    ensure_dir_700(&cfg);
    let out = run_headless(
        &cfg,
        "/init DemoUser StrongPassphrase1234 StrongPassphrase1234 I UNDERSTAND;/unlock StrongPassphrase1234;/vault attempt_limit set 2;/lock;/unlock wrong-pass-1;/unlock wrong-pass-2;/exit",
    );
    assert!(
        out.contains("QSC_ERR_VAULT_WIPED_AFTER_FAILED_UNLOCKS")
            && out.contains("event=tui_lock_state locked=LOCKED reason=unlock_attempt_limit_wipe")
            && out.contains("event=tui_locked_shell")
            && out.contains("main=init_required"),
        "wipe should trigger deterministically on second failed unlock and return to first-run shell: {}",
        out
    );
    assert!(
        !cfg.join("vault.qsv").exists(),
        "vault should be removed/tombstoned once threshold is reached"
    );
}

#[test]
fn successful_unlock_resets_failed_counter() {
    let cfg = unique_cfg_dir("na0157_attempt_limit_reset");
    ensure_dir_700(&cfg);
    let out = run_headless(
        &cfg,
        "/init DemoUser StrongPassphrase1234 StrongPassphrase1234 I UNDERSTAND;/unlock StrongPassphrase1234;/vault attempt_limit set 3;/lock;/unlock wrong-pass-1;/unlock StrongPassphrase1234;/lock;/unlock wrong-pass-2;/unlock wrong-pass-3;/exit",
    );
    assert!(
        out.contains("event=tui_unlock ok=true")
            && !out.contains("QSC_ERR_VAULT_WIPED_AFTER_FAILED_UNLOCKS"),
        "successful unlock must reset failure counter so later failures do not wipe early: {}",
        out
    );
    assert!(
        cfg.join("vault.qsv").exists(),
        "vault should still exist after reset flow"
    );
}

#[test]
fn attempt_limit_files_written_with_strict_perms() {
    let cfg = unique_cfg_dir("na0157_attempt_limit_perms");
    ensure_dir_700(&cfg);
    let out = run_headless(
        &cfg,
        "/init DemoUser StrongPassphrase1234 StrongPassphrase1234 I UNDERSTAND;/unlock StrongPassphrase1234;/vault attempt_limit set 2;/exit",
    );
    assert!(
        out.contains("event=tui_cmd_result kind=ok command=vault_attempt_limit_set"),
        "attempt-limit set command must succeed: {}",
        out
    );

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        for name in [CONFIG_FILE, COUNTER_FILE] {
            let path = cfg.join(name);
            let md = std::fs::metadata(&path).expect("metadata");
            let mode = md.permissions().mode() & 0o777;
            assert_eq!(mode, 0o600, "{} should be 0600", path.display());
        }
    }
}
