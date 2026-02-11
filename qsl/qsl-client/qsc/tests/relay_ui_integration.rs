use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn safe_test_root() -> PathBuf {
    let root = if let Ok(v) = env::var("QSC_TEST_ROOT") {
        PathBuf::from(v)
    } else if let Ok(v) = env::var("CARGO_TARGET_DIR") {
        PathBuf::from(v)
    } else {
        PathBuf::from("target")
    };
    let root = root.join("qsc-test-tmp");
    ensure_dir_700(&root);
    root
}

fn ensure_dir_700(path: &Path) {
    let _ = fs::create_dir_all(path);
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let _ = fs::set_permissions(path, fs::Permissions::from_mode(0o700));
    }
}

fn create_dir_700(path: &Path) {
    let _ = fs::remove_dir_all(path);
    ensure_dir_700(path);
}

fn output_text(out: &std::process::Output) -> String {
    let mut s = String::from_utf8_lossy(&out.stdout).to_string();
    s.push_str(&String::from_utf8_lossy(&out.stderr));
    s
}

fn required_env(name: &str) -> String {
    env::var(name).unwrap_or_else(|_| panic!("missing required env var: {name}"))
}

fn init_and_unlock(cfg: &Path, passphrase: &str) {
    let init = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", cfg)
        .env("QSC_DISABLE_KEYCHAIN", "1")
        .env("QSC_MARK_FORMAT", "plain")
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
    assert!(
        init.status.success(),
        "vault init failed: {}",
        output_text(&init)
    );

    let unlock = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", cfg)
        .env("QSC_DISABLE_KEYCHAIN", "1")
        .env("QSC_MARK_FORMAT", "plain")
        .env("QSC_PASSPHRASE", passphrase)
        .args(["vault", "unlock", "--passphrase-env", "QSC_PASSPHRASE"])
        .output()
        .expect("vault unlock");
    assert!(
        unlock.status.success(),
        "vault unlock failed: {}",
        output_text(&unlock)
    );
}

fn send_one(cfg: &Path, relay: &str, token: &str, payload: &Path) {
    let out = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", cfg)
        .env("QSC_DISABLE_KEYCHAIN", "1")
        .env("QSC_RELAY_TOKEN", token)
        .env("QSC_MARK_FORMAT", "plain")
        .args([
            "send",
            "--transport",
            "relay",
            "--relay",
            relay,
            "--to",
            "peer-0",
            "--file",
            payload.to_str().expect("payload path"),
        ])
        .output()
        .expect("send one");
    assert!(out.status.success(), "send failed: {}", output_text(&out));
}

fn run_tui_receive(cfg: &Path, relay: &str, token: &str, script: &str) -> String {
    let out = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", cfg)
        .env("QSC_DISABLE_KEYCHAIN", "1")
        .env("QSC_RELAY_TOKEN", token)
        .env("QSC_MARK_FORMAT", "plain")
        .env("QSC_TUI_HEADLESS", "1")
        .env("QSC_TUI_SCRIPT", script)
        .args([
            "tui",
            "--transport",
            "relay",
            "--relay",
            relay,
            "--seed",
            "7",
            "--scenario",
            "happy-path",
        ])
        .output()
        .expect("tui headless receive");
    let text = output_text(&out);
    assert!(out.status.success(), "tui failed: {text}");
    text
}

fn latest_messages_view_line<'a>(text: &'a str, peer: &str) -> Option<&'a str> {
    text.lines().rfind(|line| {
        line.contains("event=tui_messages_view") && line.contains(&format!("peer={peer}"))
    })
}

#[test]
#[ignore = "relay-backed integration lane; run by workflow_dispatch/schedule"]
fn relay_unfocused_inbound_increments_counter_only() {
    let relay = required_env("QSC_RELAY_UI_URL");
    let token = required_env("QSC_RELAY_UI_TOKEN");

    let base = safe_test_root().join(format!("na0127_unfocused_{}", std::process::id()));
    create_dir_700(&base);
    let sender_cfg = base.join("sender_cfg");
    let recv_cfg = base.join("recv_cfg");
    create_dir_700(&sender_cfg);
    create_dir_700(&recv_cfg);

    init_and_unlock(&sender_cfg, "na0127-pass-send");
    init_and_unlock(&recv_cfg, "na0127-pass-recv");

    let payload = base.join("msg.bin");
    fs::write(&payload, b"na0127-unfocused-inbound").expect("write payload");
    send_one(&sender_cfg, &relay, &token, &payload);

    let out = run_tui_receive(
        &recv_cfg,
        &relay,
        &token,
        "/messages select peer-0;/key tab;/receive;/exit",
    );
    assert!(
        out.contains("event=tui_receive"),
        "missing tui_receive: {out}"
    );
    let line = latest_messages_view_line(&out, "peer-0").expect("messages view marker for peer-0");
    assert!(line.contains("total=1"), "missing total=1: {line}");
    assert!(
        line.contains("visible=0"),
        "main view should not auto-append while unfocused: {line}"
    );
    assert!(
        line.contains("unread=1"),
        "unfocused inbound must increment unread: {line}"
    );
}

#[test]
#[ignore = "relay-backed integration lane; run by workflow_dispatch/schedule"]
fn relay_focused_inbound_appends_to_stream() {
    let relay = required_env("QSC_RELAY_UI_URL");
    let token = required_env("QSC_RELAY_UI_TOKEN");

    let base = safe_test_root().join(format!("na0127_focused_{}", std::process::id()));
    create_dir_700(&base);
    let sender_cfg = base.join("sender_cfg");
    let recv_cfg = base.join("recv_cfg");
    create_dir_700(&sender_cfg);
    create_dir_700(&recv_cfg);

    init_and_unlock(&sender_cfg, "na0127-pass-send");
    init_and_unlock(&recv_cfg, "na0127-pass-recv");

    let payload = base.join("msg.bin");
    fs::write(&payload, b"na0127-focused-inbound").expect("write payload");
    send_one(&sender_cfg, &relay, &token, &payload);

    let out = run_tui_receive(
        &recv_cfg,
        &relay,
        &token,
        "/messages select peer-0;/receive;/exit",
    );
    assert!(
        out.contains("event=tui_receive"),
        "missing tui_receive: {out}"
    );
    let line = latest_messages_view_line(&out, "peer-0").expect("messages view marker for peer-0");
    assert!(line.contains("total=1"), "missing total=1: {line}");
    assert!(
        line.contains("visible=1"),
        "focused inbound should append to main stream: {line}"
    );
    assert!(
        line.contains("unread=0"),
        "focused inbound should not increment unread: {line}"
    );
}
