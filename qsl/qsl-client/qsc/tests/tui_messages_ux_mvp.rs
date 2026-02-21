use assert_cmd::Command as AssertCommand;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

const CODE_ALPHA: &str = "ABCD-EFGH-JKMN-PQRS-T";

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

fn run_headless(cfg: &Path, script: &str, cols: u16, rows: u16) -> String {
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

#[test]
fn note_to_self_always_present_even_empty() {
    let cfg = unique_cfg_dir("na0149_note_to_self_empty");
    ensure_dir_700(&cfg);
    let script = format!("{} /inspector events;/key down;/exit", init_unlock_script());
    let out = run_headless(&cfg, script.as_str(), 140, 40);
    assert!(
        out.contains("event=tui_nav_select domain=messages label=Note_to_Self")
            && out.contains("event=tui_messages_view peer=Note_to_Self"),
        "Note to Self must always exist even with empty history: {}",
        out
    );
}

#[test]
fn thread_only_appears_after_history_exists_except_note_to_self() {
    let cfg = unique_cfg_dir("na0149_thread_visibility");
    ensure_dir_700(&cfg);
    let first = format!(
        "{} /contacts add Alice {};/inspector events;/exit",
        init_unlock_script(),
        CODE_ALPHA
    );
    let out_first = run_headless(&cfg, first.as_str(), 140, 40);
    assert!(
        !out_first.contains("event=tui_nav_select domain=messages label=Alice"),
        "Alice must not appear as message thread before history exists: {}",
        out_first
    );

    let second = "/unlock StrongPassphrase1234;/inspector events;/injectmsg Alice RECEIVED hello;/key down;/key down;/exit";
    let out_second = run_headless(&cfg, second, 140, 40);
    assert!(
        out_second.contains("event=tui_nav_select domain=messages label=Alice")
            && out_second.contains("event=tui_messages_view peer=Alice"),
        "Alice thread should appear only after message history exists: {}",
        out_second
    );
}

#[test]
fn msg_command_appends_outgoing_to_timeline_and_renders() {
    let cfg = unique_cfg_dir("na0149_msg_append");
    ensure_dir_700(&cfg);
    let script = format!(
        "{} /inspector events;/messages select \"Note to Self\";/msg \"hello\";/inspector results;/exit",
        init_unlock_script()
    );
    let out = run_headless(&cfg, script.as_str(), 140, 40);
    assert!(
        out.contains("event=tui_message_event peer=Note to Self state=SENT")
            && out.contains("event=tui_cmd_result kind=ok command=msg_hello"),
        "/msg should append outgoing timeline/message state and log ok result: {}",
        out
    );
}

#[test]
fn scroll_changes_visible_transcript_in_small_viewport() {
    let cfg = unique_cfg_dir("na0149_scroll_transcript");
    ensure_dir_700(&cfg);
    let mut script = format!(
        "{} /inspector events;/messages select \"Note to Self\";",
        init_unlock_script()
    );
    for idx in 0..18 {
        script.push_str(format!("/msg \"line {:02}\";", idx).as_str());
    }
    script.push_str("/key tab;/key end;/exit");
    let out = run_headless(&cfg, script.as_str(), 120, 12);
    assert!(
        out.contains("event=tui_main_scroll inspector=events offset=")
            && out.contains("event=tui_messages_view peer=Note_to_Self total=18"),
        "main-focused scroll should change visible transcript window in small viewport: {}",
        out
    );
}

#[test]
fn msg_rejects_empty_and_does_not_mutate() {
    let cfg = unique_cfg_dir("na0149_msg_empty_reject");
    ensure_dir_700(&cfg);
    let script = format!(
        "{} /inspector events;/messages select \"Note to Self\";/msg \"\";/inspector events;/status;/exit",
        init_unlock_script()
    );
    let out = run_headless(&cfg, script.as_str(), 140, 40);
    assert!(
        out.contains("event=tui_cmd_result kind=err command=msg")
            && out.contains("event=tui_msg_reject reason=empty")
            && out.contains("event=tui_status_view locked=UNLOCKED"),
        "empty /msg must reject deterministically without lock mutation: {}",
        out
    );
    assert!(
        out.contains("event=tui_messages_view peer=Note_to_Self total=0")
            && !out.contains("event=tui_message_event peer=Note to Self state=SENT"),
        "empty /msg must not append transcript entries or timeline state: {}",
        out
    );
}
