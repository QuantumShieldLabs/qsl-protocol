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
    let out = AssertCommand::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", cfg)
        .env("QSC_TUI_HEADLESS", "1")
        .env("QSC_DISABLE_KEYCHAIN", "1")
        .env("QSC_TUI_DETERMINISTIC", "1")
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
fn you_copy_only_once() {
    let cfg = unique_cfg_dir("na0147_you_copy_once");
    ensure_dir_700(&cfg);
    let script = format!(
        "{} /contacts add Alice {};/inspector contacts;/key down;/exit",
        init_unlock_script(),
        CODE_ALPHA
    );
    let out = run_headless(&cfg, script.as_str());
    assert!(
        line_with_all(
            &out,
            &["event=tui_contacts_view", "view=overview", "you_copy=shown"]
        )
        .is_some()
            && line_with_all(
                &out,
                &["event=tui_contacts_view", "view=detail", "you_copy=hidden"]
            )
            .is_some(),
        "contacts overview/detail should enforce one You-copy location: {}",
        out
    );
    assert!(
        line_with_all(
            &out,
            &["event=tui_contacts_view", "view=detail", "you_copy=shown"]
        )
        .is_none(),
        "contact detail must not render You-copy: {}",
        out,
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
