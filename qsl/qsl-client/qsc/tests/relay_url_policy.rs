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

fn relay_view_lines(out: &str) -> Vec<&str> {
    out.lines()
        .filter(|line| line.contains("event=tui_relay_view"))
        .collect()
}

#[test]
fn relay_url_policy_matrix_accepts_loopback_and_https() {
    let cfg = unique_cfg_dir("na0151_policy_allow");
    ensure_dir_700(&cfg);
    let out = run_headless(
        &cfg,
        "/init DemoUser StrongPassphrase1234 StrongPassphrase1234 I UNDERSTAND;/unlock StrongPassphrase1234;/relay set endpoint http://localhost:8080;/relay set endpoint http://127.0.0.1:8080;/relay set endpoint http://[::1]:8080;/relay set endpoint https://example.com;/relay show;/exit",
    );
    assert!(
        !out.contains("QSC_ERR_RELAY_TLS_REQUIRED"),
        "allowed relay endpoints must not trigger TLS-required marker: {}",
        out
    );
    assert!(
        out.contains("event=tui_cmd_result kind=ok command=relay_set_endpoint")
            && out.contains("event=tui_relay_view configured=true"),
        "allowed endpoints should persist and render as configured: {}",
        out
    );
}

#[test]
fn relay_url_policy_reject_does_not_mutate_persisted_endpoint() {
    let cfg = unique_cfg_dir("na0151_policy_no_mutation");
    ensure_dir_700(&cfg);
    let out = run_headless(
        &cfg,
        "/init DemoUser StrongPassphrase1234 StrongPassphrase1234 I UNDERSTAND;/unlock StrongPassphrase1234;/relay set endpoint https://relay.example.test:8443;/relay show;/relay set endpoint http://example.com;/relay show;/exit",
    );
    assert!(
        out.contains("QSC_ERR_RELAY_TLS_REQUIRED"),
        "non-loopback http reject must include deterministic TLS-required marker: {}",
        out
    );
    let views = relay_view_lines(&out);
    assert!(
        views.len() >= 2,
        "expected relay view before and after reject path: {}",
        out
    );
    let first_endpoint = views
        .first()
        .copied()
        .unwrap_or_default()
        .split_whitespace()
        .find(|part| part.starts_with("endpoint="))
        .unwrap_or("endpoint=<missing>");
    let second_endpoint = views
        .last()
        .copied()
        .unwrap_or_default()
        .split_whitespace()
        .find(|part| part.starts_with("endpoint="))
        .unwrap_or("endpoint=<missing>");
    assert_eq!(
        first_endpoint, second_endpoint,
        "reject path must not mutate persisted relay endpoint: {}",
        out
    );
}
