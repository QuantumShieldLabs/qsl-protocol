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

fn first_line_index(out: &str, needle: &str) -> usize {
    out.lines()
        .position(|line| line.contains(needle))
        .unwrap_or(usize::MAX)
}

fn first_line_index_after(out: &str, start: usize, needles: &[&str]) -> usize {
    out.lines()
        .enumerate()
        .skip(start.saturating_add(1))
        .find(|(_, line)| needles.iter().all(|needle| line.contains(needle)))
        .map(|(idx, _)| idx)
        .unwrap_or(usize::MAX)
}

#[test]
fn nav_children_are_alias_only() {
    let cfg = unique_cfg_dir("na0141_alias_only_nav");
    ensure_dir_700(&cfg);
    let out = run_headless(
        &cfg,
        "/inspector contacts;/key down;/inspector events;/key down;/exit",
    );
    assert!(
        out.contains("event=tui_nav_render")
            && out.contains("selected_label=peer-0")
            && !out.contains("selected_label=peer-0_state="),
        "nav marker should expose alias-only child label (no key=value state blobs): {}",
        out
    );
    assert!(
        !out.contains("selected_label=peer-0_blocked=")
            && !out.contains("selected_label=peer-0_mismatch="),
        "nav child labels must not include blocked/mismatch fields: {}",
        out
    );
}

#[test]
fn error_routes_to_cmd_results_and_focuses_nav() {
    let cfg = unique_cfg_dir("na0141_error_routes_cmd_results");
    ensure_dir_700(&cfg);
    let out = run_headless(&cfg, "/inspector contacts;/contacts add;/exit");
    let cmd_idx = first_line_index(&out, "event=tui_cmd cmd=contacts");
    let routed_idx =
        first_line_index_after(&out, cmd_idx, &["event=tui_inspector pane=cmd_results"]);
    let nav_focus_idx =
        first_line_index_after(&out, cmd_idx, &["event=tui_focus_home", "pane=nav"]);
    let nav_select_idx = first_line_index_after(
        &out,
        cmd_idx,
        &[
            "event=tui_nav_render",
            "selected_markers=1",
            "selected_index=2",
        ],
    );
    assert!(
        routed_idx != usize::MAX && nav_focus_idx != usize::MAX && nav_select_idx != usize::MAX,
        "error should route to cmd results and focus nav on cmd results: {}",
        out
    );
    assert!(
        out.contains("event=tui_cmd_result kind=err command=contacts_add"),
        "command results should append deterministic err entry marker: {}",
        out
    );
}

#[test]
fn success_logs_but_does_not_navigate() {
    let cfg = unique_cfg_dir("na0141_success_no_navigate");
    ensure_dir_700(&cfg);
    let out = run_headless(
        &cfg,
        "/inspector contacts;/poll show;/inspector cmdresults;/exit",
    );
    let poll_cmd_idx = first_line_index(&out, "event=tui_cmd cmd=poll");
    let still_contacts_idx = first_line_index_after(
        &out,
        poll_cmd_idx,
        &["event=tui_render", "inspector=contacts", "focus=nav"],
    );
    assert!(
        still_contacts_idx != usize::MAX,
        "successful command must not navigate away from current view: {}",
        out
    );
    assert!(
        out.contains("event=tui_cmd_result kind=ok command=poll_show")
            && out.contains("event=tui_cmd_feedback kind=ok"),
        "success should append ok result and show one-line command-bar feedback: {}",
        out
    );
}

#[test]
fn no_global_error_banner_in_other_views() {
    let cfg = unique_cfg_dir("na0141_no_error_banner_bleed");
    ensure_dir_700(&cfg);
    let out = run_headless(&cfg, "/contacts add;/inspector contacts;/exit");
    assert!(
        out.contains("event=tui_inspector pane=contacts"),
        "contacts view should render after navigation: {}",
        out
    );
    assert!(
        !out.contains("error: contacts: missing label\n\nContacts Overview"),
        "error banner should not bleed into contacts main panel: {}",
        out
    );
}

#[test]
fn lock_nav_removed_and_ctrl_l_locks() {
    let cfg = unique_cfg_dir("na0141_lock_nav_removed_ctrl_l");
    ensure_dir_700(&cfg);
    let out = run_headless(&cfg, "/inspector status;/key ctrl-l;/key ctrl-l;/exit");
    assert!(
        out.contains("event=tui_nav_render")
            && !out.contains("selected_label=lock")
            && !out.contains(" label=lock"),
        "lock must not appear as a nav row label: {}",
        out
    );
    assert!(
        out.contains("event=tui_lock_state locked=LOCKED reason=ctrl_l_shortcut")
            && out.contains("event=tui_locked_shell")
            && out.contains("nav=unlock,exit"),
        "ctrl-l from unlocked state must lock and render locked shell: {}",
        out
    );
}

#[test]
fn header_style_present() {
    let cfg = unique_cfg_dir("na0141_header_style_present");
    ensure_dir_700(&cfg);
    let out = run_headless(&cfg, "/exit");
    assert!(
        out.contains("event=tui_nav_render")
            && out.contains("header=[ QSC ]")
            && out.contains("header_left_padding=")
            && !out.contains("header_left_padding=0")
            && !out.contains("----[ QSC ]----"),
        "nav header marker must report polished centered QSC header token: {}",
        out
    );
}
