//! NA-0757 / ENG-0239 — THE SESSION DIRECTORY'S PERMISSIONS ARE THE STORE'S PRECONDITION.
//!
//! `qsp_sessions` used to be created by a bare `fs::create_dir_all`, so its mode was whatever
//! the CALLING process's umask happened to be. The `qsc` BINARY sets one (`main.rs:59`); every
//! in-process library consumer — the desktop included — does not. A directory born with any
//! write bit beyond the owner is then refused by `enforce_safe_parents`
//! (`perms_group_or_world_writable` = `mode & 0o022 != 0`), and the refusal was flattened by
//! `map_err(|_| "handshake_session_store_failed")` into one opaque string that named none of
//! the seven `ErrorCode`s able to reach it.
//!
//! ⚠ THE REFUSAL IS NOT THE BUG AND IS NOT REPAIRED HERE. `R388` A1(d): a directory found in
//! that state stays REFUSED. What changes is (a) a directory qsc CREATES is born `0700`
//! regardless of the umask, and (b) the refusal now NAMES its cause on the error marker.
//!
//! ⚠ EACH ARM IS DETERMINISTIC REGARDLESS OF THE RUNNER'S UMASK — arm A sets the process umask
//! explicitly and restores it, arm B sets the directory mode explicitly. `tests/vault.rs:423`
//! established that idiom: never let the runner's ambient umask decide a fixture.

mod common;

use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::{Mutex, MutexGuard, OnceLock};
use std::{env, fs};

use qsc::facade;

const ALICE_INBOX: &str = "na0757_alice_inbox_token_abcdefgh";
const BOB_INBOX: &str = "na0757_bob_inbox_token_ijklmnopq";

/// ⚠ SERIALIZES THE WHOLE FILE. Arm A mutates the PROCESS umask, which is shared by every
/// thread in this binary; nothing else here may create a directory while it is changed.
fn guard() -> MutexGuard<'static, ()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
        .lock()
        .unwrap_or_else(|e| e.into_inner())
}

// The test binary links libc; the crate's own `umask` binding is private, so this test
// declares the one symbol it needs, exactly as several tests in this suite re-declare the
// secret-name constants they assert against.
#[cfg(unix)]
extern "C" {
    fn umask(mask: u32) -> u32;
}

/// Set the process umask, returning the previous value so the caller can restore it.
#[cfg(unix)]
fn set_umask(mask: u32) -> u32 {
    unsafe { umask(mask) }
}

fn ensure_dir_700(path: &Path) {
    fs::create_dir_all(path).expect("mk dir");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(path, fs::Permissions::from_mode(0o700)).expect("chmod 700");
    }
}

fn mode_of(path: &Path) -> u32 {
    use std::os::unix::fs::PermissionsExt;
    fs::metadata(path).expect("metadata").permissions().mode() & 0o777
}

fn test_root(tag: &str) -> PathBuf {
    let root = env::var("CARGO_TARGET_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("target"));
    let root = root.join("qsc-test-tmp").join(tag);
    let _ = fs::remove_dir_all(&root);
    ensure_dir_700(&root);
    root
}

fn output_text(out: &std::process::Output) -> String {
    format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    )
}

fn qsc(cfg: &Path) -> Command {
    let mut c = common::qsc_std_command();
    c.env("QSC_CONFIG_DIR", cfg)
        .env("QSC_QSP_SEED", "1")
        .env("QSC_ALLOW_SEED_FALLBACK", "1")
        .env("QSC_UNSAFE_TEST_SEED_FALLBACK", "1")
        .env("QSC_MARK_FORMAT", "plain");
    c
}

fn run_ok(cfg: &Path, args: &[&str]) -> String {
    let out = qsc(cfg).args(args).output().expect("run qsc");
    let text = output_text(&out);
    assert!(out.status.success(), "qsc {args:?} failed: {text}");
    text
}

/// Extract one marker field's value. The tree's own idiom (`relay_pull_diagnostics.rs:190`),
/// so an assertion can compare the VALUE by equality instead of substring-matching the line.
fn marker_field(line: &str, key: &str) -> Option<String> {
    let key_eq = format!("{}=", key);
    line.split(' ')
        .find(|tok| tok.starts_with(key_eq.as_str()))
        .map(|tok| tok[key_eq.len()..].to_string())
}

fn set_env_once(cfg: &Path) {
    env::set_var("QSC_CONFIG_DIR", cfg);
    env::set_var("QSC_QSP_SEED", "1");
    env::set_var("QSC_ALLOW_SEED_FALLBACK", "1");
    env::set_var("QSC_UNSAFE_TEST_SEED_FALLBACK", "1");
    env::set_var("QSC_MARK_FORMAT", "plain");
}

fn party(root: &Path, name: &str, inbox: &str) -> PathBuf {
    let cfg = root.join(name);
    ensure_dir_700(&cfg);
    common::init_mock_vault(&cfg);
    run_ok(&cfg, &["identity", "rotate", "--confirm"]);
    run_ok(&cfg, &["relay", "inbox-set", "--token", inbox]);
    cfg
}

/// Bob, the in-process party — env BEFORE the unlock, and both BEFORE any server thread
/// exists (`na0756_two_party_invite_roundtrip.rs:128-136`).
fn bob_in_process(root: &Path) -> PathBuf {
    let cfg = root.join("bob");
    ensure_dir_700(&cfg);
    common::init_mock_vault(&cfg);
    run_ok(&cfg, &["identity", "rotate", "--confirm"]);
    run_ok(&cfg, &["relay", "inbox-set", "--token", BOB_INBOX]);
    set_env_once(&cfg);
    qsc::vault::unlock_with_passphrase(common::TEST_MOCK_VAULT_PASSPHRASE)
        .expect("in-process unlock of the same mock vault the subprocess used");
    qsc::set_vault_unlocked(true);
    cfg
}

fn mint(alice: &Path, base: &str) -> (String, String) {
    let out = run_ok(
        alice,
        &["invite", "create", "--relay", base, "--ttl-secs", "3600"],
    );
    let code = out
        .lines()
        .find(|l| l.starts_with("QSLI-1-"))
        .expect("invite code on stdout")
        .trim()
        .to_string();
    let listed = run_ok(alice, &["invite", "list"]);
    let id = listed
        .lines()
        .find_map(|l| l.strip_prefix("invite="))
        .and_then(|l| l.split_whitespace().next())
        .expect("invite id")
        .to_string();
    (code, id)
}

/// Drive create -> redeem -> accept, leaving the reply waiting for Bob's finish. Returns
/// (bob_cfg, relay_base) with the relay kept alive by the caller.
fn drive_to_pending_finish(root: &Path) -> (PathBuf, String, common::QslRelayTestServer) {
    let bob = bob_in_process(root);
    let relay = common::start_qsl_server(2 * 1024 * 1024, 512, None);
    let base = relay.base_url().to_string();
    let alice = party(root, "alice", ALICE_INBOX);

    let (code, invite_id) = mint(&alice, &base);
    facade::invite_redeem(&code, "alice", None).expect("facade redeem succeeds");
    let accept = run_ok(
        &alice,
        &[
            "invite",
            "accept",
            "--invite-id",
            &invite_id,
            "--alias",
            "bob",
        ],
    );
    assert!(accept.contains("status=pinned"), "alice must pin: {accept}");
    (bob, base, relay)
}

/// ARM A (`R388` A4(i)) — A FRESH SESSION DIRECTORY IS BORN `0700` DESPITE A PERMISSIVE UMASK.
///
/// ⚠ MUST GO RED IF the creation stops being mode-explicit. At the pre-repair base this arm is
/// RED: under `umask 002` the bare `create_dir_all` yielded `0775` and the finish died at
/// `handshake_session_store_failed` (banked, `evidence/repro_inprocess_umask002_RED.log`).
/// The assertion is BY EQUALITY on the observed mode — not "not group-writable", which `0755`
/// would also satisfy while still being wrong.
#[test]
#[cfg(unix)]
fn na0757_a_fresh_session_dir_is_born_0700_despite_a_permissive_umask() {
    let _g = guard();
    let root = test_root("na0757_fresh_dir_umask");

    // The whole point of the arm: a umask that WOULD have produced a group-writable directory.
    let prev = set_umask(0o002);
    let outcome = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let (bob, base, _relay) = drive_to_pending_finish(&root);
        let sessions = bob.join("qsp_sessions");
        assert!(
            !sessions.exists(),
            "precondition: the session dir must not exist before the finish creates it"
        );

        let done = facade::invite_finish(None, "alice", &base, 1)
            .expect("the in-process finish must COMPLETE, not die in the session store");
        assert!(done, "the reply was waiting, so finish reports FOUND");

        assert_eq!(
            mode_of(&sessions),
            0o700,
            "the session dir must be BORN 0700 under umask 002 — a mode-explicit creation, \
             not the ambient umask"
        );

        let st = facade::connect_status("alice");
        assert_eq!(st.state, facade::ConnectState::Active);
        assert_eq!(st.reason, facade::ConnectReason::Handshake);
    }));
    set_umask(prev);
    if let Err(p) = outcome {
        std::panic::resume_unwind(p);
    }
}

/// ARM B (`R388` A4(ii), and seal Z2's control) — AN ALREADY-POISONED DIRECTORY IS STILL
/// REFUSED, AND THE REFUSAL NOW NAMES ITS CODE.
///
/// This is the arm that proves the un-flattening. Before the repair the run emitted exactly
/// `event=error code=handshake_session_store_failed` and nothing else (banked,
/// `evidence/repro_field_shape_subprocess_poisoned_RED.log` — the field's own marker
/// sequence). After it, the same refusal carries `store_code=unsafe_parent_perms`.
///
/// ⚠ MUST GO RED IF the repair ever starts silently repairing an INHERITED directory: the
/// refusal is the ruled behaviour (`R388` A1(d)) and the heal-policy axis is deferred whole.
#[test]
#[cfg(unix)]
fn na0757_b_a_poisoned_session_dir_is_refused_and_the_marker_names_the_code() {
    let _g = guard();
    let root = test_root("na0757_poisoned_dir");
    let (bob, base, _relay) = drive_to_pending_finish(&root);

    // Exactly what a process that never set a umask leaves behind.
    let sessions = bob.join("qsp_sessions");
    fs::create_dir_all(&sessions).expect("pre-create the session dir");
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&sessions, fs::Permissions::from_mode(0o775)).expect("chmod 775");
    }
    assert_eq!(
        mode_of(&sessions),
        0o775,
        "the fixture must actually be group-writable, or this arm proves nothing"
    );

    let out = qsc(&bob)
        .args(["invite", "finish", "--alias", "alice", "--relay", &base])
        .output()
        .expect("run qsc");
    let text = output_text(&out);
    assert!(
        !out.status.success(),
        "an inherited group-writable session dir must still be REFUSED: {text}"
    );

    let line = text
        .lines()
        .find(|l| {
            l.contains("event=error")
                && marker_field(l, "code").as_deref() == Some("handshake_session_store_failed")
        })
        .unwrap_or_else(|| panic!("the flattened error marker must still be emitted: {text}"));

    assert_eq!(
        marker_field(line, "store_code").as_deref(),
        Some("unsafe_parent_perms"),
        "the refusal must NAME the underlying store code on the marker: {line}"
    );
    assert_eq!(
        marker_field(line, "code").as_deref(),
        Some("handshake_session_store_failed"),
        "and the OUTER code string is deliberately unchanged — no new string enters the \
         facade's wire vocabulary (`R388` A1(b))"
    );
}

/// ARM C (`R388` A4(iii)) — THE ENV-GATED TEST SEED FALLBACK IS UNCHANGED.
///
/// One guard assertion, because the repair touched the directory's creation and NOTHING in
/// the key path: the fallback must still require BOTH gates and must still refuse on either
/// alone. ⚠ MUST GO RED IF a future edit widens the fallback to a single env var.
#[test]
fn na0757_c_the_seed_fallback_still_requires_both_env_gates() {
    let _g = guard();
    let restore = (
        env::var("QSC_ALLOW_SEED_FALLBACK").ok(),
        env::var("QSC_UNSAFE_TEST_SEED_FALLBACK").ok(),
    );

    env::set_var("QSC_ALLOW_SEED_FALLBACK", "1");
    env::set_var("QSC_UNSAFE_TEST_SEED_FALLBACK", "1");
    assert!(
        qsc::protocol_state::allow_unsafe_seed_fallback_for_tests(),
        "both gates set must enable the fixture fallback"
    );

    env::remove_var("QSC_UNSAFE_TEST_SEED_FALLBACK");
    assert!(
        !qsc::protocol_state::allow_unsafe_seed_fallback_for_tests(),
        "the UNSAFE gate alone being absent must disable it"
    );

    env::set_var("QSC_UNSAFE_TEST_SEED_FALLBACK", "1");
    env::remove_var("QSC_ALLOW_SEED_FALLBACK");
    assert!(
        !qsc::protocol_state::allow_unsafe_seed_fallback_for_tests(),
        "the ALLOW gate alone being absent must disable it"
    );

    match restore.0 {
        Some(v) => env::set_var("QSC_ALLOW_SEED_FALLBACK", v),
        None => env::remove_var("QSC_ALLOW_SEED_FALLBACK"),
    }
    match restore.1 {
        Some(v) => env::set_var("QSC_UNSAFE_TEST_SEED_FALLBACK", v),
        None => env::remove_var("QSC_UNSAFE_TEST_SEED_FALLBACK"),
    }
}
