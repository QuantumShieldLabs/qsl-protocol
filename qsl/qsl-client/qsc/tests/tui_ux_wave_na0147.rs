use assert_cmd::Command as AssertCommand;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

const CODE_ALPHA: &str = "ABCD-EFGH-JKMN-PQRS-T";
const CODE_LONG: &str = "QRST-VWXY-2345-6789-A";

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
    run_headless_sized(cfg, script, 140, 40)
}

fn run_headless_sized(cfg: &Path, script: &str, cols: u16, rows: u16) -> String {
    let out = AssertCommand::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", cfg)
        .env("QSC_TUI_HEADLESS", "1")
        .env("QSC_DISABLE_KEYCHAIN", "1")
        .env("QSC_TUI_DETERMINISTIC", "1")
        .env("NO_COLOR", "1")
        .env("QSC_TUI_SCRIPT", script)
        .env("QSC_TUI_COLS", cols.to_string())
        .env("QSC_TUI_ROWS", rows.to_string())
        .env("QSC_MARK_FORMAT", "plain")
        .args(["tui"])
        .output()
        .expect("run tui headless");
    let mut combined = String::from_utf8_lossy(&out.stdout).to_string();
    combined.push_str(&String::from_utf8_lossy(&out.stderr));
    combined
}

fn init_unlock_script() -> String {
    "/init DemoUser StrongPassphrase1234 StrongPassphrase1234 I UNDERSTAND;/unlock StrongPassphrase1234;"
        .to_string()
}

fn marker_line<'a>(out: &'a str, marker: &str) -> &'a str {
    out.lines()
        .find(|line| line.contains(marker))
        .unwrap_or_else(|| panic!("missing marker {marker} in output:\n{out}"))
}

fn marker_field(line: &str, key: &str) -> String {
    line.split_whitespace()
        .find_map(|token| token.strip_prefix(&format!("{key}=")))
        .unwrap_or_else(|| panic!("missing key {key} in line: {line}"))
        .to_string()
}

fn token_offset(line: &str, token: &str) -> usize {
    line.find(token)
        .unwrap_or_else(|| panic!("missing token {token} in line: {line}"))
}

fn line_with_all<'a>(out: &'a str, needles: &[&str]) -> Option<&'a str> {
    out.lines()
        .find(|line| needles.iter().all(|needle| line.contains(needle)))
}

#[test]
fn contacts_table_alignment_fixed_width() {
    let cfg = unique_cfg_dir("na0147_contacts_table_alignment");
    ensure_dir_700(&cfg);
    let script = format!(
        "{} /contacts add Alpha {};/contacts add VeryLongAliasName {};/inspector contacts;/exit",
        init_unlock_script(),
        CODE_ALPHA,
        CODE_LONG
    );
    let out = run_headless(&cfg, script.as_str());
    let line = marker_line(&out, "event=tui_contacts_table");
    let header = marker_field(line, "header");
    let row0 = marker_field(line, "row0");
    assert!(
        !header.contains('|') && !row0.contains('|'),
        "contacts table must not use pipes: {}",
        line
    );
    assert_eq!(
        token_offset(&header, "Alias"),
        token_offset(&row0, "Alpha"),
        "alias column not aligned: {}",
        line
    );
    assert_eq!(
        token_offset(&header, "Trust"),
        token_offset(&row0, "UNVERIFIED"),
        "trust column not aligned: {}",
        line
    );
    assert_eq!(
        token_offset(&header, "Blocked"),
        token_offset(&row0, "no"),
        "blocked column not aligned: {}",
        line
    );
    assert_eq!(
        token_offset(&header, "Last_seen"),
        token_offset(&row0, "-"),
        "last seen column not aligned: {}",
        line
    );
}

#[test]
fn note_to_self_always_present() {
    let cfg = unique_cfg_dir("na0147_note_to_self_present");
    ensure_dir_700(&cfg);
    let script = format!("{} /inspector events;/key down;/exit", init_unlock_script());
    let out = run_headless(&cfg, script.as_str());
    assert!(
        out.contains("event=tui_nav_select domain=messages label=Note_to_Self")
            && out.contains("event=tui_messages_view peer=Note_to_Self"),
        "Note to Self must always appear and be selectable in messages nav: {}",
        out
    );
}

#[test]
fn messages_subnav_filters_empty_threads() {
    let cfg = unique_cfg_dir("na0147_messages_thread_filter");
    ensure_dir_700(&cfg);
    let script = format!(
        "{} /contacts add Alice {};/inspector events;/key down;/injectmsg Alice RECEIVED;/key down;/exit",
        init_unlock_script(),
        CODE_ALPHA
    );
    let out = run_headless(&cfg, script.as_str());
    assert!(
        out.contains("event=tui_nav_select domain=messages label=Note_to_Self")
            && out.contains("event=tui_nav_select domain=messages label=Alice")
            && out.contains("event=tui_messages_view peer=Alice"),
        "Alice should only become a messages child after message history exists: {}",
        out
    );
}

#[test]
fn contacts_overview_has_no_you_line() {
    let cfg = unique_cfg_dir("na0147_contacts_no_you_line");
    ensure_dir_700(&cfg);
    let script = format!(
        "{} /contacts add Alice {};/inspector contacts;/exit",
        init_unlock_script(),
        CODE_ALPHA
    );
    let out = run_headless(&cfg, script.as_str());
    assert!(
        line_with_all(
            &out,
            &[
                "event=tui_contacts_view",
                "view=overview",
                "you_copy=hidden"
            ]
        )
        .is_some(),
        "contacts overview must hide You-copy marker: {}",
        out
    );
    assert!(
        !out.contains("You:"),
        "contacts overview must not render a You line: {}",
        out
    );
}

#[test]
fn cmd_bar_focus_label_updates() {
    let cfg = unique_cfg_dir("na0147_focus_label_updates");
    ensure_dir_700(&cfg);
    let unlocked = format!("{} /key tab;/key tab;/key esc;/exit", init_unlock_script());
    let unlocked_out = run_headless(&cfg, unlocked.as_str());
    assert!(
        unlocked_out.contains("cmdbar_text=Focus:_NAV")
            && unlocked_out.contains("cmdbar_text=Focus:_MAIN")
            && unlocked_out.contains("cmdbar_text=Focus:_CMD"),
        "unlocked focus label must track nav/main/cmd transitions: {}",
        unlocked_out
    );

    let locked_cfg = unique_cfg_dir("na0147_focus_label_locked");
    ensure_dir_700(&locked_cfg);
    let locked_out = run_headless(&locked_cfg, "/key tab;/key tab;/exit");
    assert!(
        locked_out.contains("cmdbar_text=Focus: NAV")
            && locked_out.contains("cmdbar_text=Focus: CMD")
            && !locked_out.contains("cmdbar_text=Focus: MAIN"),
        "locked mode focus label must only toggle nav/cmd: {}",
        locked_out
    );
}

#[test]
fn main_scroll_changes_visible_content() {
    let cfg = unique_cfg_dir("na0147_main_scroll_changes");
    ensure_dir_700(&cfg);
    let script = format!(
        "{} /inspector settings;/key tab;/key down;/key pgdn;/exit",
        init_unlock_script()
    );
    let out = run_headless_sized(&cfg, script.as_str(), 120, 14);
    assert!(
        out.contains("event=tui_main_scroll inspector=settings offset=1")
            || out.contains("event=tui_main_scroll inspector=settings offset=2"),
        "main-focused scrolling must move settings viewport offset: {}",
        out
    );
    assert!(
        out.contains("selected_label=settings"),
        "nav selection must remain on settings while main scrolls: {}",
        out
    );
}

#[test]
fn commands_section_has_two_blank_lines() {
    let cfg = unique_cfg_dir("na0147_commands_spacing");
    ensure_dir_700(&cfg);
    let script = format!(
        "{} /inspector account;/inspector settings;/inspector contacts;/exit",
        init_unlock_script()
    );
    let out = run_headless(&cfg, script.as_str());
    assert!(
        out.contains("event=tui_account_view")
            && out.contains("event=tui_settings_view")
            && out.contains("event=tui_contacts_view")
            && out.contains("commands_gap=2"),
        "commands sections must keep two blank lines above Commands: {}",
        out
    );
}

#[test]
fn vault_where_device_show_work_unlocked() {
    let cfg = unique_cfg_dir("na0147_vault_device_unlocked");
    ensure_dir_700(&cfg);
    let script = format!(
        "{} /vault where;/device show;/inspector results;/exit",
        init_unlock_script()
    );
    let out = run_headless(&cfg, script.as_str());
    assert!(
        out.contains("event=tui_vault_where ok=true")
            && out.contains("event=tui_device_show ok=true")
            && out.contains("event=tui_cmd_result kind=ok command=vault_where")
            && out.contains("event=tui_cmd_result kind=ok command=device_show"),
        "vault/device show should succeed unlocked and update results last command: {}",
        out
    );
}

#[test]
fn vault_where_device_show_reject_locked() {
    let cfg = unique_cfg_dir("na0147_vault_device_locked_reject");
    ensure_dir_700(&cfg);
    let out = run_headless(&cfg, "/vault where;/device show;/exit");
    assert!(
        out.contains("event=tui_cmd_result kind=err command=vault_where")
            && out.contains("event=tui_cmd_result kind=err command=device_show")
            && out.contains("event=tui_inspector pane=cmd_results")
            && out.contains("event=tui_focus_home pane=nav"),
        "locked vault/device commands must reject deterministically and route error focus: {}",
        out
    );
}

#[test]
fn nav_wraps_top_bottom() {
    let cfg = unique_cfg_dir("na0147_nav_wraps");
    ensure_dir_700(&cfg);
    let script = format!("{} /key up;/key down;/exit", init_unlock_script());
    let out = run_headless(&cfg, script.as_str());
    assert!(
        out.contains("event=tui_nav_select domain=legal label=legal")
            && out.contains("event=tui_nav_select domain=system"),
        "nav up/down should wrap between system and legal headers: {}",
        out
    );
}
