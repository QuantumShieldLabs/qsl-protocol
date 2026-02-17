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

fn ensure_dir_700(path: &Path) {
    std::fs::create_dir_all(path).expect("create dir");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o700)).expect("chmod");
    }
}

fn init_vault(cfg: &Path, passphrase: &str) {
    ensure_dir_700(cfg);
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

fn key_script_for(text: &str) -> String {
    let mut out = String::new();
    for ch in text.chars() {
        if ch == ' ' {
            out.push_str("/key space;");
        } else {
            out.push_str(&format!("/key {};", ch));
        }
    }
    out
}

#[test]
fn system_account_page_exists_and_is_first() {
    let cfg = unique_cfg_dir("na0142_system_account_first");
    init_vault(&cfg, "StrongPassphrase1234");
    let out = run_headless(&cfg, "/unlock StrongPassphrase1234;/key down;/exit");
    assert!(
        out.contains("event=tui_nav_render")
            && out.contains("selected_label=account")
            && out.contains("event=tui_render mode=home layout=h3 inspector=account"),
        "system first child should be account and render account pane: {}",
        out
    );
}

#[test]
fn results_label_is_results() {
    let cfg = unique_cfg_dir("na0142_results_label");
    init_vault(&cfg, "StrongPassphrase1234");
    let out = run_headless(
        &cfg,
        "/unlock StrongPassphrase1234;/inspector cmdresults;/exit",
    );
    assert!(
        out.contains("selected_label=results")
            && out.contains("event=tui_render mode=home layout=h3 inspector=cmd_results"),
        "system results nav label should be 'results': {}",
        out
    );
    assert!(
        !out.contains("Cmd Results"),
        "legacy 'Cmd Results' label must not appear in rendered output: {}",
        out
    );
}

#[test]
fn no_submit_cancel_footer_strings() {
    let cfg_unlock = unique_cfg_dir("na0142_no_footer_unlock");
    init_vault(&cfg_unlock, "StrongPassphrase1234");
    let unlock_out = run_headless(&cfg_unlock, "/key enter;/exit");

    let cfg_init = unique_cfg_dir("na0142_no_footer_init");
    ensure_dir_700(&cfg_init);
    let init_out = run_headless(&cfg_init, "/init;/exit");

    let cfg_destroy = unique_cfg_dir("na0142_no_footer_destroy");
    init_vault(&cfg_destroy, "StrongPassphrase1234");
    let destroy_out = run_headless(
        &cfg_destroy,
        "/unlock StrongPassphrase1234;/account destroy;/exit",
    );

    let combined = format!("{}\n{}\n{}", unlock_out, init_out, destroy_out);
    assert!(
        !combined.contains("Submit: Enter | Cancel: Esc") && !combined.contains("Keys:"),
        "submit/cancel footer hints should not appear in unlock/init/destroy views: {}",
        combined
    );
}

#[test]
fn labels_have_no_underscores() {
    let cfg = unique_cfg_dir("na0142_labels_no_underscores");
    init_vault(&cfg, "StrongPassphrase1234");
    let out = run_headless(
        &cfg,
        "/unlock StrongPassphrase1234;/inspector account;/inspector settings;/inspector status;/exit",
    );
    assert!(
        out.contains("event=tui_account_view")
            && out.contains("event=tui_settings_view")
            && out.contains("event=tui_status_view"),
        "account/settings/status views should render during label-cleanup regression test: {}",
        out
    );
    assert!(
        !out.contains("verification_code:")
            && !out.contains("poll_mode:")
            && !out.contains("poll_interval_seconds:")
            && !out.contains("enabled_by_default:")
            && !out.contains("timeout_minutes:"),
        "underscore labels should not appear in user-facing panes: {}",
        out
    );
}

#[test]
fn account_destroy_happy_path_resets_to_init_ready() {
    let cfg = unique_cfg_dir("na0142_destroy_happy");
    init_vault(&cfg, "StrongPassphrase1234");

    let mut script = String::from("/unlock StrongPassphrase1234;/account destroy;");
    script.push_str(&key_script_for("StrongPassphrase1234"));
    script.push_str("/key enter;");
    script.push_str(&key_script_for("DESTROY MY VAULT"));
    script.push_str("/key enter;/exit");
    let out = run_headless(&cfg, script.as_str());

    assert!(
        out.contains("event=tui_lock_state locked=LOCKED reason=account_destroy")
            && out.contains("event=tui_locked_shell")
            && out.contains("vault=missing")
            && out.contains("main=init_required"),
        "destroy success should return to locked init-ready shell: {}",
        out
    );
    assert!(
        !cfg.join("vault.qsv").exists(),
        "vault file should be removed after destroy"
    );

    let init_again = run_headless(
        &cfg,
        "/init DemoUser StrongPassphrase1234 StrongPassphrase1234 I UNDERSTAND;/exit",
    );
    assert!(
        init_again.contains("event=tui_init ok=true"),
        "/init should work after account destroy reset: {}",
        init_again
    );
}

#[test]
fn account_destroy_wrong_passphrase_or_phrase_has_no_mutation() {
    let cfg = unique_cfg_dir("na0142_destroy_wrong");
    init_vault(&cfg, "StrongPassphrase1234");

    let mut script = String::from("/unlock StrongPassphrase1234;/account destroy;");
    script.push_str(&key_script_for("WrongPassphrase1234"));
    script.push_str("/key enter;");
    script.push_str("/key esc;/account destroy;");
    script.push_str(&key_script_for("StrongPassphrase1234"));
    script.push_str("/key enter;");
    script.push_str(&key_script_for("DESTROY MY vault"));
    script.push_str("/key enter;/exit");
    let out = run_headless(&cfg, script.as_str());

    assert!(
        out.contains("event=tui_cmd_result kind=err command=account_destroy")
            && !out.contains("event=tui_lock_state locked=LOCKED reason=account_destroy"),
        "wrong passphrase/phrase must not destroy vault or lock-transition to missing-vault shell: {}",
        out
    );
    assert!(
        cfg.join("vault.qsv").exists(),
        "vault file should remain present after failed destroy attempts"
    );
}
