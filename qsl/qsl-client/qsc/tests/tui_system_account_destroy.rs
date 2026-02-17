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
    ensure_dir_700(&cfg);
    let out = run_headless(
        &cfg,
        "/init DemoUser StrongPassphrase1234 StrongPassphrase1234 I UNDERSTAND;/unlock StrongPassphrase1234;/key down;/exit",
    );
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
    ensure_dir_700(&cfg);
    let out = run_headless(
        &cfg,
        "/init DemoUser StrongPassphrase1234 StrongPassphrase1234 I UNDERSTAND;/unlock StrongPassphrase1234;/inspector results;/exit",
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
    ensure_dir_700(&cfg_unlock);
    let unlock_out = run_headless(
        &cfg_unlock,
        "/init DemoUser StrongPassphrase1234 StrongPassphrase1234 I UNDERSTAND;/key enter;/exit",
    );

    let cfg_init = unique_cfg_dir("na0142_no_footer_init");
    ensure_dir_700(&cfg_init);
    let init_out = run_headless(&cfg_init, "/init;/exit");

    let cfg_destroy = unique_cfg_dir("na0142_no_footer_destroy");
    ensure_dir_700(&cfg_destroy);
    let destroy_out = run_headless(
        &cfg_destroy,
        "/init DemoUser StrongPassphrase1234 StrongPassphrase1234 I UNDERSTAND;/unlock StrongPassphrase1234;/account destroy;/exit",
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
    ensure_dir_700(&cfg);
    let out = run_headless(
        &cfg,
        "/init DemoUser StrongPassphrase1234 StrongPassphrase1234 I UNDERSTAND;/unlock StrongPassphrase1234;/inspector account;/inspector settings;/inspector status;/exit",
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
fn init_success_populates_account_fields() {
    let cfg = unique_cfg_dir("na0142_init_populates_fields");
    ensure_dir_700(&cfg);
    let out = run_headless(
        &cfg,
        "/init Matthew_01 StrongPassphrase1234 StrongPassphrase1234 I UNDERSTAND;/unlock StrongPassphrase1234;/inspector account;/exit",
    );
    assert!(
        out.contains("event=tui_init ok=true")
            && out.contains("event=tui_account_view")
            && out.contains("alias_set=true")
            && out.contains("verification_code=")
            && !out.contains("verification_code=none"),
        "init should populate alias and a real verification code: {}",
        out
    );
    assert!(
        cfg.join("identities")
            .read_dir()
            .map(|entries| {
                entries.flatten().any(|entry| {
                    entry.file_name().to_string_lossy().starts_with("self_")
                        && entry.path().extension().and_then(|ext| ext.to_str()) == Some("json")
                })
            })
            .unwrap_or(false),
        "identity keys/public record should exist after init"
    );
    let vault_bytes = std::fs::read(cfg.join("vault.qsv")).expect("read vault");
    let vault_text = String::from_utf8_lossy(&vault_bytes);
    assert!(
        !vault_text.contains("Matthew_01"),
        "vault should not store alias in plaintext"
    );
}

#[test]
fn init_persists_settings_in_vault() {
    let cfg = unique_cfg_dir("na0142_init_persists_settings");
    ensure_dir_700(&cfg);

    let first = run_headless(
        &cfg,
        "/init DemoUser StrongPassphrase1234 StrongPassphrase1234 I UNDERSTAND;/unlock StrongPassphrase1234;/autolock set 15;/poll set fixed 12;/exit",
    );
    assert!(
        first.contains("event=tui_autolock_set ok=true minutes=15")
            && first.contains("event=tui_poll_set ok=true mode=fixed interval_seconds=12"),
        "settings update should succeed: {}",
        first
    );

    let second = run_headless(
        &cfg,
        "/unlock StrongPassphrase1234;/autolock show;/poll show;/exit",
    );
    assert!(
        second.contains("event=tui_autolock_show ok=true minutes=15")
            && second.contains("event=tui_poll_show ok=true mode=fixed interval_seconds=12"),
        "settings should reload from vault after restart: {}",
        second
    );
}

#[test]
fn destroy_wipes_everything_and_resets_defaults() {
    let cfg = unique_cfg_dir("na0142_destroy_wipes");
    ensure_dir_700(&cfg);

    let mut script = String::from(
        "/init DemoUser StrongPassphrase1234 StrongPassphrase1234 I UNDERSTAND;/unlock StrongPassphrase1234;/autolock set 15;/poll set fixed 12;/account destroy;",
    );
    script.push_str(&key_script_for("StrongPassphrase1234"));
    script.push_str("/key enter;/key Y;/key enter;/exit");
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

    let post_destroy = run_headless(&cfg, "/init NewUser StrongPassphrase1234 StrongPassphrase1234 I UNDERSTAND;/unlock StrongPassphrase1234;/autolock show;/poll show;/inspector results;/exit");
    assert!(
        post_destroy.contains("event=tui_autolock_show ok=true minutes=10")
            && post_destroy
                .contains("event=tui_poll_show ok=true mode=adaptive interval_seconds=10")
            && post_destroy.contains("last_command=inspector")
            && post_destroy.contains("last_status=ok"),
        "post-destroy re-init should restore default settings and last-result baseline: {}",
        post_destroy
    );
}

#[test]
fn destroy_wrong_passphrase_no_mutation() {
    let cfg = unique_cfg_dir("na0142_destroy_wrong_pass");
    ensure_dir_700(&cfg);
    let mut script = String::from(
        "/init DemoUser StrongPassphrase1234 StrongPassphrase1234 I UNDERSTAND;/unlock StrongPassphrase1234;/account destroy;",
    );
    script.push_str(&key_script_for("WrongPassphrase1234"));
    script.push_str("/key enter;/inspector results;/exit");
    let out = run_headless(&cfg, script.as_str());

    assert!(
        out.contains("event=tui_cmd_result kind=err command=account_destroy")
            && out.contains("event=tui_cmd_results_view")
            && !out.contains("event=tui_lock_state locked=LOCKED reason=account_destroy"),
        "wrong passphrase must not destroy vault or transition to init-required shell: {}",
        out
    );
    assert!(
        cfg.join("vault.qsv").exists(),
        "vault file should remain present after failed destroy attempt"
    );
}

#[test]
fn init_ui_no_step_text_and_shows_summary() {
    let cfg = unique_cfg_dir("na0142_init_summary_no_step");
    ensure_dir_700(&cfg);
    let out = run_headless(
        &cfg,
        "/init;/key M;/key a;/key t;/key t;/key h;/key e;/key w;/key _;/key 0;/key 1;/key enter;/key S;/key t;/key r;/key o;/key n;/key g;/key P;/key a;/key s;/key s;/key p;/key h;/key r;/key a;/key s;/key e;/key 1;/key 2;/key 3;/key 4;/key enter;/exit",
    );
    assert!(
        !out.contains("Step 1/4")
            && !out.contains("Step 2/4")
            && out.contains("main_summary_alias=Alias: Matthew_01")
            && out.contains("main_summary_passphrase=<redacted>"),
        "init wizard should have persistent summary without step text: {}",
        out
    );
}

#[test]
fn results_is_last_command_only() {
    let cfg = unique_cfg_dir("na0142_results_last_only");
    ensure_dir_700(&cfg);
    let out = run_headless(
        &cfg,
        "/init DemoUser StrongPassphrase1234 StrongPassphrase1234 I UNDERSTAND;/unlock StrongPassphrase1234;/status;/poll show;/inspector results;/exit",
    );
    assert!(
        out.contains("event=tui_cmd_results_view")
            && out.contains("last_command=inspector")
            && !out.contains("last_command=status"),
        "results view should expose only the latest command result: {}",
        out
    );
}
