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
        std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o700)).expect("chmod 700");
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

fn run_headless(cfg: &Path, script: &str, relay: Option<&str>) -> String {
    let mut cmd = AssertCommand::new(assert_cmd::cargo::cargo_bin!("qsc"));
    cmd.env("QSC_CONFIG_DIR", cfg)
        .env("QSC_TUI_HEADLESS", "1")
        .env("QSC_DISABLE_KEYCHAIN", "1")
        .env("QSC_TUI_SCRIPT", script)
        .env("QSC_TUI_COLS", "140")
        .env("QSC_TUI_ROWS", "40")
        .env("QSC_MARK_FORMAT", "plain");
    let args = if let Some(relay_url) = relay {
        vec!["tui", "--transport", "relay", "--relay", relay_url]
    } else {
        vec!["tui"]
    };
    let out = cmd.args(args).output().expect("run tui headless");
    let mut combined = String::from_utf8_lossy(&out.stdout).to_string();
    combined.push_str(&String::from_utf8_lossy(&out.stderr));
    combined
}

fn marker_lines<'a>(out: &'a str, event: &str) -> Vec<&'a str> {
    out.lines()
        .filter(|line| line.contains(&format!("event={event}")))
        .collect()
}

#[test]
fn fixed_polling_has_deterministic_cadence_without_in_between_ticks() {
    let cfg = unique_cfg_dir("na0138_fixed_poll_cadence");
    init_vault(&cfg, "StrongPassphrase1234");
    let out = run_headless(
        &cfg,
        "/unlock StrongPassphrase1234;/poll set fixed 10;wait 9999;wait 1;wait 10000;wait 10000;/exit",
        Some("http://127.0.0.1:9"),
    );
    let ticks = marker_lines(&out, "tui_poll_tick");
    assert_eq!(ticks.len(), 3, "expected 3 fixed polling ticks: {out}");
    assert!(
        ticks[0].contains("due_ms=10000") && ticks[0].contains("now_ms=10000"),
        "first tick should land at 10s: {}",
        ticks[0]
    );
    assert!(
        ticks[1].contains("due_ms=20000") && ticks[1].contains("now_ms=20000"),
        "second tick should land at 20s: {}",
        ticks[1]
    );
    assert!(
        ticks[2].contains("due_ms=30000") && ticks[2].contains("now_ms=30000"),
        "third tick should land at 30s: {}",
        ticks[2]
    );
}

#[test]
fn poll_validation_rejects_invalid_seconds_without_mutation() {
    let cfg = unique_cfg_dir("na0138_poll_validation");
    init_vault(&cfg, "StrongPassphrase1234");
    let out = run_headless(
        &cfg,
        "/unlock StrongPassphrase1234;/poll set fixed 1;/poll show;/exit",
        None,
    );
    assert!(
        out.contains("event=tui_poll_set code=poll_invalid_seconds ok=false"),
        "invalid poll interval should reject: {out}"
    );
    assert!(
        out.contains("event=tui_poll_show ok=true mode=adaptive interval_seconds=10"),
        "reject path should keep adaptive default without mutation: {out}"
    );
}

#[test]
fn poll_mode_switch_persists_and_status_exposes_configuration() {
    let cfg = unique_cfg_dir("na0138_poll_persist");
    init_vault(&cfg, "StrongPassphrase1234");

    let first = run_headless(
        &cfg,
        "/unlock StrongPassphrase1234;/poll set fixed 12;/exit",
        None,
    );
    assert!(
        first.contains("event=tui_poll_set ok=true mode=fixed interval_seconds=12"),
        "fixed polling set marker missing: {first}"
    );

    let second = run_headless(
        &cfg,
        "/unlock StrongPassphrase1234;/poll show;/inspector status;/exit",
        None,
    );
    assert!(
        second.contains("event=tui_poll_show ok=true mode=fixed interval_seconds=12"),
        "fixed polling config should persist across sessions: {second}"
    );
    assert!(
        second.contains("event=tui_status_view")
            && second.contains("poll_mode=fixed")
            && second.contains("poll_interval_seconds=12"),
        "status view should expose poll mode + interval: {second}"
    );

    let third = run_headless(
        &cfg,
        "/unlock StrongPassphrase1234;/poll set adaptive;/poll show;/exit",
        None,
    );
    assert!(
        third.contains("event=tui_poll_set ok=true mode=adaptive interval_seconds=12")
            && third.contains("event=tui_poll_show ok=true mode=adaptive interval_seconds=12"),
        "adaptive switch should succeed and keep persisted interval value: {third}"
    );
}

#[test]
fn fixed_polling_does_not_add_extra_tick_after_receive() {
    let cfg = unique_cfg_dir("na0138_poll_no_extra");
    init_vault(&cfg, "StrongPassphrase1234");
    let out = run_headless(
        &cfg,
        "/unlock StrongPassphrase1234;/poll set fixed 10;wait 10000;/receive;wait 9999;wait 1;/exit",
        Some("http://127.0.0.1:9"),
    );
    let ticks = marker_lines(&out, "tui_poll_tick");
    assert_eq!(
        ticks.len(),
        2,
        "expected only cadence ticks (10s and 20s) without extra post-receive tick: {out}"
    );
    assert!(ticks[0].contains("due_ms=10000"));
    assert!(ticks[1].contains("due_ms=20000"));
}

#[test]
fn adaptive_polling_does_not_schedule_fixed_ticks() {
    let cfg = unique_cfg_dir("na0144_poll_adaptive_no_fixed_ticks");
    init_vault(&cfg, "StrongPassphrase1234");
    let out = run_headless(
        &cfg,
        "/unlock StrongPassphrase1234;/poll set adaptive;wait 60000;/exit",
        Some("http://127.0.0.1:9"),
    );
    let ticks = marker_lines(&out, "tui_poll_tick");
    assert!(
        ticks.is_empty(),
        "adaptive mode should not emit fixed-cadence poll ticks: {out}"
    );
}
