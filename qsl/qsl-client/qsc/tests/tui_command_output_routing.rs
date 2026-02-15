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
        .env("QSC_TUI_TEST_UNLOCK", "1")
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

fn first_line_index(out: &str, needle: &str) -> usize {
    out.lines()
        .position(|line| line.contains(needle))
        .unwrap_or(usize::MAX)
}

fn assert_show_routes_to_status(script_cmd: &str, cmd_marker: &str) {
    let cfg = unique_cfg_dir("na0140_show_routes");
    ensure_dir_700(&cfg);
    let script = format!("/inspector settings;{script_cmd};/exit");
    let out = run_headless(&cfg, &script);
    let cmd_idx = first_line_index(&out, cmd_marker);
    let status_idx = first_line_index(&out, "event=tui_inspector pane=status");
    assert!(
        out.contains("event=tui_inspector pane=settings"),
        "settings precondition missing before show command: {}",
        out
    );
    assert!(
        cmd_idx != usize::MAX && status_idx != usize::MAX && status_idx > cmd_idx,
        "show command should route to status after command marker: {}",
        out
    );
}

#[test]
fn show_commands_route_to_status() {
    assert_show_routes_to_status("/status", "event=tui_cmd cmd=status");
    assert_show_routes_to_status("/poll show", "event=tui_cmd cmd=poll");
    assert_show_routes_to_status("/autolock show", "event=tui_cmd cmd=autolock");
}

#[test]
fn set_commands_do_not_change_view() {
    let cfg = unique_cfg_dir("na0140_set_stays_settings");
    ensure_dir_700(&cfg);
    let out = run_headless(
        &cfg,
        "/inspector settings;/poll set fixed 10;/autolock set 15;/status;/exit",
    );
    let status_cmd_idx = first_line_index(&out, "event=tui_cmd cmd=status");
    let inspector_status_idx = first_line_index(&out, "event=tui_inspector pane=status");
    assert!(
        out.contains("event=tui_poll_set ok=true mode=fixed interval_seconds=10")
            && out.contains("event=tui_autolock_set ok=true minutes=15"),
        "set command success markers missing: {}",
        out
    );
    assert!(
        status_cmd_idx != usize::MAX
            && inspector_status_idx != usize::MAX
            && inspector_status_idx > status_cmd_idx,
        "set commands should not change view before explicit /status: {}",
        out
    );
    assert!(
        out.contains("event=tui_status_view") && out.contains("last_result=autolock set 15 min"),
        "status view should expose deterministic last command result: {}",
        out
    );
}

#[test]
fn settings_layout_is_clean() {
    let cfg = unique_cfg_dir("na0140_settings_clean");
    ensure_dir_700(&cfg);
    let out = run_headless(&cfg, "/inspector settings;/exit");
    assert!(
        out.contains("event=tui_settings_view")
            && out.contains("sections=lock,autolock,polling,commands"),
        "settings marker should report cleaned grouped sections: {}",
        out
    );
    assert!(
        !out.contains("status_containment")
            && !out.contains("policy: read_only")
            && !out.contains("dangerous_actions: command_bar_only"),
        "removed internal-ish settings fields still present: {}",
        out
    );
}

#[test]
fn show_set_commands_do_not_wedge_or_relock() {
    let cfg = unique_cfg_dir("na0140_routing_no_wedge");
    ensure_dir_700(&cfg);
    let out = run_headless(
        &cfg,
        "/inspector settings;/status;/poll show;/autolock show;/poll set fixed 10;/autolock set 15;/key down;/status;/exit",
    );
    assert!(
        out.contains("event=tui_cmd cmd=key") && out.contains("event=tui_cmd cmd=status"),
        "UI should remain responsive after show/set commands: {}",
        out
    );
    assert!(
        !out.contains("event=tui_lock_state locked=LOCKED")
            && !out.contains("locked_unlock_required"),
        "show/set commands must not relock or force unlock-required recovery: {}",
        out
    );
}
