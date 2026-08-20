//! NA-0751 (D-1393) — THE LOCKED CONTROL. ITS OWN BINARY, AND THAT IS THE POINT.
//!
//! ⚠ The subject of this control is a PROCESS ATOMIC — `VAULT_UNLOCKED_THIS_RUN`
//! (`lib.rs:190`), read by `vault_unlocked()` (`:196`). It is not per-identity and not
//! per-config-dir, and `vault_init_with_passphrase` is documented at `vault/mod.rs:220-221` as
//! having "No process unlock-state side effect" — so a fresh config directory does NOT reset
//! it. Isolating this control by DIRECTORY would prove nothing: if any earlier test in the
//! same binary unlocked, the pre-check would not fire and the assertion would either fail
//! confusingly or pass against a FORCED flag. It is isolated BY PROCESS instead, and the first
//! thing it does is prove the flag is genuinely default here.
//!
//! ⚠ NO SEED-FALLBACK PAIR IN THIS FILE. `QSC_ALLOW_SEED_FALLBACK` /
//! `QSC_UNSAFE_TEST_SEED_FALLBACK` turn `qsp_session_store_key_load`'s locked branch into a
//! test fallback key, which is exactly the path whose real behaviour these seals concern.
//! (They ARE the suite's standard idiom — 62 of 133 qsc test files set them — and are
//! permitted in `na0751_facade_invite_surface.rs`, where the demonstrated route needs them.
//! The prohibition is per-file, the way the tree's own lanes scope it.)

use std::path::{Path, PathBuf};
use std::{env, fs};

use qsc::facade::{connect_status, contact_list, contact_requests, ConnectReason, FacadeError};

fn dir700(p: &Path) {
    fs::create_dir_all(p).unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(p, fs::Permissions::from_mode(0o700)).unwrap();
    }
}

fn fresh_cfg(tag: &str) -> PathBuf {
    let root = env::var("CARGO_TARGET_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("target"));
    let dir = root
        .join("qsc-test-tmp")
        .join(format!("na0751_locked_{tag}_{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    dir700(&dir);
    dir
}

#[test]
fn na0751_locked_control_fires_on_a_genuinely_default_flag() {
    // (0) THE CONTROL'S OWN PRECONDITION, asserted FIRST and before anything could have set
    // it: the process unlock flag is default-false HERE. Without this line the whole file
    // could be passing against a forced flag.
    assert!(
        !qsc::vault_unlocked(),
        "the process unlock flag must be default-false in this binary"
    );

    let cfg = fresh_cfg("action_verbs");
    env::set_var("QSC_CONFIG_DIR", &cfg);
    env::set_var("QSC_QSP_SEED", "1");

    // (1) A vault that exists but was NEVER unlocked. `vault_init_with_passphrase` has no
    // process unlock-state side effect, so the flag stays false.
    qsc::vault::vault_init_with_passphrase("correct horse battery staple")
        .expect("vault init");
    assert!(!qsc::vault_unlocked(), "init must not unlock");

    // (2) EVERY verb reports `Locked`, from the facade's OWN measurement.
    assert!(matches!(contact_list(), Err(FacadeError::Locked)));
    assert!(matches!(contact_requests(), Err(FacadeError::Locked)));
    assert!(matches!(
        qsc::facade::contact_request_accept("someone"),
        Err(FacadeError::Locked)
    ));
    assert!(matches!(
        qsc::facade::invite_list(),
        Err(FacadeError::Locked)
    ));

    // (3) THE ARMS DIFFER — unlock and the same call succeeds, so the refusal above was a
    // measurement and not a vacuum.
    qsc::vault::unlock_with_passphrase("correct horse battery staple").expect("unlock");
    qsc::set_vault_unlocked(true);
    assert!(qsc::vault_unlocked());
    assert!(
        contact_list().is_ok(),
        "with the vault unlocked the same call must succeed"
    );
    qsc::set_vault_unlocked(false);
}

#[test]
fn na0751_locked_pass_through_truth_the_override_is_scoped() {
    // ⚠ THE SECOND WINDOW TRUTH-CHECK. A fresh never-unlocked vault has NO session blob, so
    // `qsp_session_load` returns `Ok(None)` (`protocol_state:964`) and the tuple yields
    // `no_session` / `missing_seed` — NEVER `session_invalid`. The facade must therefore pass
    // those through UNCHANGED even though the vault is locked: the override is scoped to one
    // arm, and this proves the scope rather than assuming it.
    let cfg = fresh_cfg("pass_through");
    env::set_var("QSC_CONFIG_DIR", &cfg);
    env::set_var("QSC_QSP_SEED", "1");
    qsc::set_vault_unlocked(false);

    let s = connect_status("peer-0");
    assert_eq!(
        s.reason,
        ConnectReason::NoSession,
        "a locked vault with no blob is no_session, NOT VaultLocked"
    );

    env::remove_var("QSC_QSP_SEED");
    let s = connect_status("peer-0");
    assert_eq!(
        s.reason,
        ConnectReason::MissingSeed,
        "a locked vault with no blob and no seed is missing_seed, NOT VaultLocked"
    );
}

#[test]
fn na0751_locked_override_arm_on_a_fabricated_blob() {
    // ⚠ FABRICATED-STATE CONTROL, and it is stated as one. The `locked AND session_invalid`
    // arm cannot be reached from a fresh never-unlocked vault (see the test above), because
    // writing a real session blob needs the store key, which needs the vault unlocked or the
    // seed fallback on — both excluded here. Garbage bytes at the measured path reach it,
    // because `qsp_session_decrypt_blob` short-circuits at `protocol_state:870-874` on
    // key-unavailable BEFORE any structural or integrity check.
    let cfg = fresh_cfg("fabricated");
    let sessions = cfg.join("qsp_sessions");
    dir700(&sessions);
    fs::write(sessions.join("peer-0.bin"), b"not-a-session").unwrap();
    env::set_var("QSC_CONFIG_DIR", &cfg);
    env::set_var("QSC_QSP_SEED", "1");
    qsc::set_vault_unlocked(false);

    let s = connect_status("peer-0");
    assert_eq!(
        s.reason,
        ConnectReason::VaultLocked,
        "locked AND session_invalid is the ONE overridden arm"
    );

    // THE ARMS DIFFER: the same fabricated state with the flag set reports the underlying
    // string instead, so the override is doing the work and not the fixture.
    qsc::set_vault_unlocked(true);
    let s = connect_status("peer-0");
    assert_eq!(
        s.reason,
        ConnectReason::SessionInvalid,
        "unlocked, the same state reports session_invalid"
    );
    qsc::set_vault_unlocked(false);
}
