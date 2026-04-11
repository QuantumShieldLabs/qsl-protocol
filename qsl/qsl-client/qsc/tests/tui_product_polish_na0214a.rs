mod common;

use std::fs;
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
    fs::create_dir_all(path).expect("create dir");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(path, fs::Permissions::from_mode(0o700)).expect("chmod");
    }
}

fn init_passphrase_vault(cfg: &Path, passphrase: &str) {
    common::init_passphrase_vault(cfg, passphrase);
}

fn run_headless(cfg: &Path, script: &str, unlocked: bool) -> String {
    let out = common::qsc_assert_command()
        .env("QSC_CONFIG_DIR", cfg)
        .env("QSC_TUI_HEADLESS", "1")
        .env("QSC_TUI_TEST_UNLOCK", if unlocked { "1" } else { "0" })
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

fn last_marker_line<'a>(out: &'a str, event: &str) -> &'a str {
    out.lines()
        .rev()
        .find(|line| line.contains(&format!("event={event}")))
        .unwrap_or_else(|| panic!("missing marker {event} in {out}"))
}

#[test]
fn qsc_front_door_docs_are_truthful() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let readme = fs::read_to_string(root.join("README.md")).expect("readme");
    let local = fs::read_to_string(root.join("LOCAL_TWO_CLIENT_RUNBOOK.md")).expect("local");
    let aws = fs::read_to_string(root.join("REMOTE_TWO_CLIENT_AWS_RUNBOOK.md")).expect("aws");
    let soak = fs::read_to_string(root.join("REMOTE_SOAK_PLAYBOOK.md")).expect("soak");

    assert!(
        readme.contains("qbuild-first, AWS-free operator front door")
            && readme.contains("LOCAL_TWO_CLIENT_RUNBOOK.md")
            && readme.contains("compatibility-only")
            && readme.contains("QSC_ATTACHMENT_SERVICE"),
        "README should define the front door and migration posture truthfully: {readme}"
    );
    assert!(
        local.contains("validated qbuild/local operator baseline")
            && local.contains("not the validated front door"),
        "local runbook should state its front-door role explicitly: {local}"
    );
    assert!(
        aws.contains("not the validated qbuild/local front door")
            && aws.contains("secondary compatibility evidence"),
        "AWS runbook should be demoted to compatibility-only evidence: {aws}"
    );
    assert!(
        soak.contains("compatibility/operational evidence only")
            && soak.contains("not the validated qbuild/local front door"),
        "remote soak playbook should be clearly non-baseline: {soak}"
    );
}

#[test]
fn relay_surface_avoids_tbd_security_copy() {
    let cfg = unique_cfg_dir("na0214a_relay_copy");
    ensure_dir_700(&cfg);
    common::init_mock_vault(&cfg);

    let out = run_headless(
        &cfg,
        "/relay set endpoint https://relay.example.test:8443;/relay show;/exit",
        true,
    );
    let relay = last_marker_line(&out, "tui_relay_view");
    assert!(
        relay.contains("pinning=not configured")
            && relay.contains("baseline=local_qbuild_front_door")
            && relay.contains("compatibility=remote_aws_compat_only")
            && !relay.contains("pinning=TBD"),
        "relay pane should replace placeholder security copy with truthful markers: {out}"
    );
}

#[test]
fn status_surface_explains_invalid_session_and_migration_posture() {
    let cfg = unique_cfg_dir("na0214a_status_copy");
    ensure_dir_700(&cfg);
    common::init_mock_vault(&cfg);
    let sessions = cfg.join("qsp_sessions");
    ensure_dir_700(&sessions);
    fs::write(sessions.join("peer-0.qsv"), b"bad-session").expect("write bad session");

    let out = run_headless(&cfg, "/inspector status;/exit", true);
    let status = last_marker_line(&out, "tui_status_view");
    assert!(
        status.contains("qsp_note=Stored session rejected as invalid or stale; re-establish handshake.")
            && status.contains("baseline=local_qbuild_front_door")
            && status.contains("compatibility=remote_aws_compat_only")
            && status.contains("migration=attachment_service_required"),
        "status pane should explain invalid-session recovery and migration posture with stable markers: {out}"
    );
}

#[test]
fn lock_surface_and_unlock_errors_are_actionable() {
    let cfg = unique_cfg_dir("na0214a_lock_copy");
    ensure_dir_700(&cfg);
    init_passphrase_vault(&cfg, "StrongPassphrase1234");

    let unlock_reject = run_headless(&cfg, "/unlock wrong-pass;/exit", false);
    let locked_shell = last_marker_line(&unlock_reject, "tui_locked_shell");
    assert!(
        locked_shell
            .contains("main_error=error: unlock failed: passphrase did not open the local vault")
            && unlock_reject
                .contains("event=tui_unlock code=vault_locked ok=false reason=passphrase_invalid"),
        "unlock reject should use actionable wording and a truthful reason code: {unlock_reject}"
    );

    let lock_view = run_headless(
        &cfg,
        "/unlock StrongPassphrase1234;/vault attempt_limit set 2;/inspector lock;/exit",
        false,
    );
    let lock = last_marker_line(&lock_view, "tui_lock_view");
    assert!(
        lock.contains("vault_attempt_limit_mode=threshold_wipe")
            && lock.contains("vault_attempt_limit_threshold=2")
            && lock.contains("vault_attempt_limit_scope=vault_and_state")
            && lock.contains("recovery=rerun_init_if_wiped"),
        "lock pane should explain wipe-threshold behavior truthfully with stable markers: {lock_view}"
    );
}
