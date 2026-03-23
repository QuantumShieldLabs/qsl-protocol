use assert_cmd::cargo::cargo_bin_cmd;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};

fn safe_test_dir(label: &str) -> PathBuf {
    let base = env::temp_dir().join("qsc-test-tmp");
    fs::create_dir_all(&base).expect("create tui test root");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&base, fs::Permissions::from_mode(0o700)).expect("chmod test root");
    }
    static NEXT_ID: AtomicUsize = AtomicUsize::new(0);
    let id = NEXT_ID.fetch_add(1, Ordering::SeqCst);
    let dir = base.join(format!("{}-{}-{}", label, std::process::id(), id));
    fs::create_dir(&dir).expect("create tui test dir");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&dir, fs::Permissions::from_mode(0o700)).expect("chmod test dir");
    }
    dir
}

fn run_tui_script(script: &str) -> String {
    let cfg_dir = safe_test_dir("tui-charter");
    let mut cmd = cargo_bin_cmd!("qsc");
    let output = cmd
        .arg("tui")
        .env("QSC_CONFIG_DIR", &cfg_dir)
        .env("QSC_TUI_HEADLESS", "1")
        .env("QSC_TUI_SCRIPT", script)
        .env("QSC_MARK_FORMAT", "plain")
        .output()
        .expect("run tui");
    assert!(output.status.success());
    let mut out = String::new();
    out.push_str(&String::from_utf8_lossy(&output.stdout));
    out.push_str(&String::from_utf8_lossy(&output.stderr));
    out
}

#[test]
fn tui_does_not_send_without_explicit_command() {
    let out = run_tui_script("hello world\n/exit\n");
    assert!(out.contains("event=tui_open"));
    assert!(out.contains("event=tui_input_text"));
    assert!(out.contains("event=tui_exit"));
    assert!(!out.contains("tui_send"));
    assert!(!out.contains("retry"));
}

#[test]
fn tui_markers_are_deterministic() {
    let script = "/status\n/exit\n";
    let first = run_tui_script(script);
    let second = run_tui_script(script);
    assert_eq!(first, second);
}

#[test]
fn tui_no_secrets_in_output() {
    let secret = "secret-passphrase";
    let out = run_tui_script(&format!("{secret}\n/exit\n"));
    let lowered = out.to_lowercase();
    assert!(!lowered.contains("secret"));
    assert!(!lowered.contains(secret));
}
