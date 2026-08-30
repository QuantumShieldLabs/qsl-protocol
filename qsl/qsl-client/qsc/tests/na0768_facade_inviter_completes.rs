#![allow(dead_code)]
//! NA-0768 (D-1409) -- `L1`: THE INVITER COMPLETES **THROUGH THE FACADE**, NOT THROUGH THE CLI.
//!
//! `P-FACADE` (`D-1413`) requires that a capability the desktop depends on is proven at the
//! FACADE, because the facade is what the desktop calls. Every other arm in this lane drives the
//! `qsc` binary as a subprocess; a subprocess proves the CLI works and says nothing about the
//! path the GUI takes. **This file is the one that closes that gap.**
//!
//! ## THE PROPERTY
//! With a real A2 from an inviter-role contact sitting in the user's shared inbox --
//! the arrangement that left the inviter reading "Connecting..." forever (`ENG-0250`,
//! `ENG-0251`) -- calling `qsc::facade::invite_finish` **IN-PROCESS** must leave
//! `qsc::facade::connect_status(peer).state == ConnectState::Active`.
//!
//! That is the first link of the F10 chain: any stored session reads `active` to the GUI
//! (`protocol_state` -> `facade::connect_status` -> the Tauri command -> `ui/main.js`). The rest
//! of the chain is desktop-side and is not asserted here; what IS asserted is that the protocol
//! side of it delivers.
//!
//! ## ⚠ RED-ARMED, AND THAT IS THE WHOLE POINT
//! On the UNREPAIRED tree `invite_finish` pulls the inbox, LEASES the A2, declines it and hands
//! it to nothing that can consume it -- so no session is stored and `connect_status` stays
//! `Inactive`. **This arm fails there.** An arm that cannot go red is not evidence.
//!
//! ## ⚠ ENVIRONMENT IS SET ONCE, BEFORE ANY SERVER STARTS
//! `std::env::set_var` is unsound once other threads are running, and the relay fixture spawns a
//! runtime. The file guard serializes; the env is set before the fixture exists. This is the
//! tree's own idiom (`na0751_facade_invite_surface.rs:59`).
//!
//! ## ⚠ THE RELAY IS THE REAL ONE, AT THE DEPLOYED LEASE
//! `common::start_qsl_server_with_store` at `PRODUCTION_PULL_LEASE_SECS`. The test-local mock
//! pops on pull and cannot express a lease, which would make the whole arrangement vacuous --
//! the defect this lane repairs is ABOUT leasing.

mod common;

use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::{Mutex, MutexGuard, OnceLock};
use std::{env, fs};

use qsc::facade::{connect_status, ConnectState};

const PRODUCTION_PULL_LEASE_SECS: usize = 60;
const ALPHA_INBOX: &str = "na0768-l1-alpha-inbox-token-aaaa";
const CHARLIE_INBOX: &str = "na0768-l1-charlie-inbox-tok-cccc";

fn guard() -> MutexGuard<'static, ()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
        .lock()
        .unwrap_or_else(|e| e.into_inner())
}

fn ensure_dir_700(path: &Path) {
    fs::create_dir_all(path).expect("create dir");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut p = fs::metadata(path).expect("meta").permissions();
        p.set_mode(0o700);
        fs::set_permissions(path, p).expect("chmod");
    }
}

fn test_root(tag: &str) -> PathBuf {
    let root = common::unique_test_root(tag);
    ensure_dir_700(&root);
    root
}

/// ⚠ ONE call, BEFORE any server. Everything downstream inherits it.
fn set_env_once(cfg: &Path) {
    env::set_var("QSC_CONFIG_DIR", cfg);
    env::set_var("QSC_QSP_SEED", "1");
    env::set_var("QSC_ALLOW_SEED_FALLBACK", "1");
    env::set_var("QSC_UNSAFE_TEST_SEED_FALLBACK", "1");
    env::set_var("QSC_MARK_FORMAT", "plain");
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
    assert!(out.status.success(), "expected success: {args:?}\n{text}");
    text
}

fn run_any(cfg: &Path, args: &[&str]) -> (bool, String) {
    let out = qsc(cfg).args(args).output().expect("run qsc");
    (out.status.success(), output_text(&out))
}

fn party(root: &Path, name: &str, inbox: &str) -> PathBuf {
    let cfg = root.join(name);
    ensure_dir_700(&cfg);
    common::init_mock_vault(&cfg);
    run_ok(&cfg, &["identity", "rotate", "--confirm"]);
    run_ok(&cfg, &["relay", "inbox-set", "--token", inbox]);
    cfg
}

// ⚠ ADOPTED VERBATIM from `na0742_invite_finish_scan_producer_acks.rs` rather than re-derived.
// A first cut of this file invented both parsers -- `code=` and `invite_id=` -- and neither
// field exists: `invite create` prints the bare `QSLI-1-...` code on stdout and `invite list`
// prints `invite=<id> ...`. **Bespoke scaffolding measures your own setup by mistake.**
fn invite_code(text: &str) -> String {
    text.lines()
        .find(|l| l.starts_with("QSLI-1-"))
        .expect("invite code on stdout")
        .trim()
        .to_string()
}

fn newest_invite_id(cfg: &Path) -> String {
    let listing = run_ok(cfg, &["invite", "list"]);
    listing
        .lines()
        .filter_map(|l| l.strip_prefix("invite="))
        .filter_map(|l| l.split_whitespace().next())
        .last()
        .expect("invite id")
        .to_string()
}

/// Builds the arrangement that defines the defect: a REAL A2 from `charlie`, who redeemed
/// alpha's invite, sitting in ALPHA's shared inbox. Driven by subprocesses because every
/// session-establishing function in `protocol_state` is `pub(crate)` or private -- an
/// integration test has no `pub` path to one. **The FINISH is then done in-process, which is
/// what makes this a FACADE assertion rather than a CLI one.**
fn plant_a2_from_an_inviter_role_contact(root: &Path, base: &str) -> PathBuf {
    let alpha = party(root, "alpha", ALPHA_INBOX);
    let charlie = party(root, "charlie", CHARLIE_INBOX);
    let code = invite_code(&run_ok(
        &alpha,
        &["invite", "create", "--relay", base, "--ttl-secs", "3600"],
    ));
    let aid = newest_invite_id(&alpha);
    run_ok(&charlie, &["invite", "redeem", "--code", &code, "--alias", "alpha"]);
    run_ok(&alpha, &["invite", "accept", "--invite-id", &aid, "--alias", "charlie"]);
    // charlie consumes the B1 and pushes her A2 into ALPHA's inbox
    let (ok, t) = run_any(&charlie, &["invite", "finish", "--alias", "alpha", "--relay", base]);
    assert!(ok, "charlie's finish must succeed or there is no A2 to measure:\n{t}");
    alpha
}

#[test]
fn l1_the_inviter_completes_through_the_facade() {
    let _g = guard();
    let root = test_root("na0768_l1");
    let alpha_dir = root.join("alpha");
    ensure_dir_700(&alpha_dir);
    // ⚠ BEFORE the fixture spawns any thread.
    set_env_once(&alpha_dir);

    let relay = common::start_qsl_server_with_store(2 * 1024 * 1024, 512, None, PRODUCTION_PULL_LEASE_SECS);
    let base = relay.base_url().to_string();
    let alpha = plant_a2_from_an_inviter_role_contact(&root, &base);
    assert_eq!(alpha, alpha_dir, "the env and the subprocesses must share one config dir");

    // The subprocesses carried the mock-vault unlock args; this process has not unlocked yet,
    // and the facade fails closed on a locked vault. Unlock the SAME mock vault in-process.
    qsc::vault::unlock_with_passphrase(common::TEST_MOCK_VAULT_PASSPHRASE)
        .expect("in-process unlock of the same mock vault the subprocesses used");
    qsc::set_vault_unlocked(true);

    let before = connect_status("charlie");
    println!("L1 BEFORE: state={:?} reason={:?}", before.state, before.reason);
    assert_eq!(
        before.state,
        ConnectState::Inactive,
        "L1 PRECONDITION: the inviter must start DISCONNECTED, or this arm proves nothing"
    );

    // ── THE FACADE CALL. This is the property. ────────────────────────────────────────────
    let done = qsc::facade::invite_finish(None, "charlie", &base, 1)
        .expect("facade::invite_finish must not error on a present, valid A2");
    let after = connect_status("charlie");
    println!("L1 facade::invite_finish -> {done}");
    println!("L1 AFTER : state={:?} reason={:?}", after.state, after.reason);

    // ⚠ RED-ARMED. On the unrepaired tree the A2 is leased, declined, and handed to nothing:
    // no session is stored and this stays `Inactive`.
    assert_eq!(
        after.state,
        ConnectState::Active,
        "L1: after facade::invite_finish the INVITER must read ACTIVE to the GUI -- \
         this is the first link of the F10 chain, and the whole promise of the lane. \
         before={before:?} after={after:?}"
    );
}
