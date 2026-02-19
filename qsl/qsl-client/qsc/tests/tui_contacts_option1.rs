use assert_cmd::Command as AssertCommand;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

const CODE_ALICE: &str = "ABCD-EFGH-JKMN-PQRS-T";
const CODE_BOB: &str = "QRST-VWXY-2345-6789-A";
const CODE_WRONG: &str = "VVVV-WWXX-YYYY-ZZZZ-0";

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

fn init_unlock_script() -> String {
    "/init DemoUser StrongPassphrase1234 StrongPassphrase1234 I UNDERSTAND;/unlock StrongPassphrase1234;"
        .to_string()
}

#[test]
fn contacts_overview_renders_table() {
    let cfg = unique_cfg_dir("na0145_contacts_overview_table");
    ensure_dir_700(&cfg);
    let script = format!(
        "{} /contacts add Alice {};/contacts add Bob {};/inspector contacts;/exit",
        init_unlock_script(),
        CODE_ALICE,
        CODE_BOB
    );
    let out = run_headless(&cfg, script.as_str());
    assert!(
        out.contains("event=tui_contacts_add label=Alice ok=true status=UNVERIFIED")
            && out.contains("event=tui_contacts_add label=Bob ok=true status=UNVERIFIED")
            && out.contains("event=tui_contacts_view")
            && out.contains("view=overview")
            && out.contains("sections=overview_table,contact_card,commands"),
        "contacts overview should render Option-1 overview/table marker shape: {}",
        out
    );
}

#[test]
fn contact_detail_renders_card_sections() {
    let cfg = unique_cfg_dir("na0145_contact_detail_card");
    ensure_dir_700(&cfg);
    let script = format!(
        "{} /contacts add Alice {};/inspector contacts;/key down;/exit",
        init_unlock_script(),
        CODE_ALICE
    );
    let out = run_headless(&cfg, script.as_str());
    assert!(
        out.contains("event=tui_nav_select domain=contacts label=Alice")
            && out.contains("event=tui_contacts_view")
            && out.contains("selected=Alice")
            && out.contains("view=detail")
            && out.contains("trust=UNVERIFIED")
            && out.contains("blocked=false"),
        "contact detail should render card-semantic marker fields: {}",
        out
    );
}

#[test]
fn contacts_add_creates_unverified_and_persists() {
    let cfg = unique_cfg_dir("na0145_contacts_add_persists");
    ensure_dir_700(&cfg);
    let first = format!(
        "{} /contacts add Alice {};/inspector contacts;/key down;/exit",
        init_unlock_script(),
        CODE_ALICE
    );
    let first_out = run_headless(&cfg, first.as_str());
    assert!(
        first_out.contains("event=tui_contacts_add label=Alice ok=true status=UNVERIFIED")
            && first_out.contains("event=tui_contacts_view")
            && first_out.contains("selected=Alice")
            && first_out.contains("trust=UNVERIFIED"),
        "contacts add should create unverified contact in current session: {}",
        first_out
    );

    let second = "/unlock StrongPassphrase1234;/inspector contacts;/key down;/exit";
    let second_out = run_headless(&cfg, second);
    assert!(
        second_out.contains("event=tui_contacts_view")
            && second_out.contains("selected=Alice")
            && second_out.contains("trust=UNVERIFIED"),
        "contact should persist across reload using vault-backed cache: {}",
        second_out
    );
}

#[test]
fn verify_sets_verified_or_mismatch_and_routes_on_error() {
    let cfg_ok = unique_cfg_dir("na0145_verify_match");
    ensure_dir_700(&cfg_ok);
    let ok_script = format!(
        "{} /inspector contacts;/contacts add Alice {};/verify Alice {};/exit",
        init_unlock_script(),
        CODE_ALICE,
        CODE_ALICE
    );
    let ok_out = run_headless(&cfg_ok, ok_script.as_str());
    assert!(
        ok_out.contains("event=tui_contacts_verify label=Alice ok=true status=VERIFIED")
            && ok_out.contains("event=tui_cmd_result kind=ok command=verify_Alice")
            && !ok_out.contains("event=tui_inspector pane=cmd_results"),
        "successful verify should set VERIFIED and avoid error-route navigation: {}",
        ok_out
    );

    let cfg_err = unique_cfg_dir("na0145_verify_mismatch");
    ensure_dir_700(&cfg_err);
    let err_script = format!(
        "{} /inspector contacts;/contacts add Alice {};/verify Alice {};/exit",
        init_unlock_script(),
        CODE_ALICE,
        CODE_WRONG
    );
    let err_out = run_headless(&cfg_err, err_script.as_str());
    assert!(
        err_out.contains("event=tui_contacts_verify code=verification_mismatch")
            && err_out.contains("event=tui_inspector pane=cmd_results")
            && err_out.contains("event=tui_focus_home pane=nav")
            && err_out.contains("event=tui_cmd_result kind=err command="),
        "verify mismatch should be err and route to results with nav focus: {}",
        err_out
    );
}

#[test]
fn block_unblock_persists() {
    let cfg = unique_cfg_dir("na0145_block_unblock_persists");
    ensure_dir_700(&cfg);
    let first = format!(
        "{} /contacts add Alice {};/contacts block Alice;/contacts unblock Alice;/inspector contacts;/key down;/exit",
        init_unlock_script(),
        CODE_ALICE
    );
    let first_out = run_headless(&cfg, first.as_str());
    assert!(
        first_out.contains("event=tui_contacts_block label=Alice ok=true")
            && first_out.contains("event=tui_contacts_unblock label=Alice ok=true")
            && first_out.contains("event=tui_contacts_view")
            && first_out.contains("selected=Alice")
            && first_out.contains("blocked=false"),
        "block/unblock should execute and end unblocked in-session: {}",
        first_out
    );

    let second = "/unlock StrongPassphrase1234;/inspector contacts;/key down;/exit";
    let second_out = run_headless(&cfg, second);
    assert!(
        second_out.contains("event=tui_contacts_view")
            && second_out.contains("selected=Alice")
            && second_out.contains("blocked=false"),
        "blocked state should persist after reload using vault-backed contacts store: {}",
        second_out
    );
}
