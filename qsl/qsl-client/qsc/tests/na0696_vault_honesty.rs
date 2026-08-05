// NA-0696 (D630 as amended, D-1336): ENG-0110 + ENG-0111 (ENG-0118 folded) — the
// VAULT-HONESTY gate instrument. The store lock is REENTRANT (a per-process path-keyed
// depth registry at the LockGuard layer) and the per-site inner-variant regime is
// RETIRED; destroy earns the erase it documents (zero-in-place + sync + inode pin) and
// its BOUNDARY (vault-keyed satellites die; vault-independent config survives, BY NAME);
// destroy gains a keychain-vault deliberateness ceremony; the keychain load error splits
// three ways so a missing key stops reading as a wrong passphrase.
//
// The instruments and their honest red-stories (D630 §4c as amended by A1.3):
// (i)   nested_commit_emits_debug_instrument_and_transaction_lands — a REAL plain send
//       through the spawned DEBUG binary: the D1(c) commit transaction lands (timeline
//       row + send_commit) AND the child's stderr carries the F5 nested-acquire
//       instrument. Red against Control R (reentrancy reverted) and Control V (emit
//       no-op'd — the stderr half alone).
// (ii)  destroy_residue_set_enumerated_by_name — the A1.4/R2 boundary, asserted as a
//       DIRECTORY-LISTING EQUALITY, never a spot check: survivors exactly
//       {.qsc.lock, config.txt, store.meta}; the gone set (vault.qsv, both
//       protection-state files, send.state, msgqueue_v1/, quarantine_v1/, attachments/)
//       asserted absent BY NAME. Red against Control R (the nested protection-clear
//       refuses and the state files survive).
// (iii) destroy_passphrase_vault_flow_unchanged — the ks1 flow is BYTE-UNCHANGED
//       (wrong commitment → vault_locked; correct → destroyed). Behavior pin, honestly
//       NOT red at base (A1.3 replaced the draft's universal-intent test).
// (iv)  keychain_unsupported_load_split_headless — the cfg(not(feature)) arm still
//       refuses explicit keychain with vault_token_unavailable. Behavior pin, honest
//       story: pre-existing behavior, NOT red-capable against D5 alone.
// Seam-armed arm (the NAMED LANE GATE outside CI, the D-1335 R3 shape; compiled only
// under `--cfg qsc_keychain_test_seam` + `--features keychain`):
// (v)   keychain_missing_entry_reads_token_missing — RED AT BASE (the base collapses
//       every load failure to vault_locked); a deleted seam entry reads EXACTLY
//       vault_token_missing, an unreadable seam dir (daemon-down class) reads
//       vault_token_unavailable. Control L's red set.
// (vi)  keychain_destroy_ceremony_requires_destroy_word — the A1.3 ceremony pin:
//       token "WRONG" → vault_destroy_confirm_mismatch with the vault INTACT; token
//       "DESTROY" → destroyed. Control C's red set.
//
// Recorded untestable-headless (D627-A4, applied not deferred): the real
// Secret-Service / macOS Keychain / Credential Manager keyring plumbing (unchanged
// posture, D-1335); the non-Unix compile refusal's LIVE demonstration (D630 §0.9 — the
// cross-check dies in ring's mingw cc before qsc compiles, so the gate is
// STRUCTURAL-ONLY this lane, upgrade path named: install gcc-mingw-w64-x86-64 and a
// future micro-lane runs the real gate); the DefaultHome-write-success property
// (D627-A4's own record, unchanged).
//
// Harness: the NA-0693/0694/0695 pattern — ENV_LOCK serialization, fresh per-test dirs
// chmod 0700 after create_dir_all, CLI/pub surface only. The in-src registry and
// inode-pin units live in `model/mod.rs` / `vault/protection.rs` (pub(crate) surfaces
// unreachable from here) and do NOT satisfy goal-lint; THIS file is the path-based gate
// instrument.

// A test binary is its own crate root, so the lib-side allow does not reach it (the
// rng-seam and na0695 test files carry the same attribute for the same custom cfg
// reason).
#![allow(unexpected_cfgs)]

mod common;

use predicates::prelude::*;
use qsc::output::{marker_queue, set_marker_routing, MarkerRouting};
use qsc::set_vault_unlocked;
use qsc::vault::protection::{
    destroy_with_passphrase, unlock_guarded_at, wipe_after_failed_unlocks_arm, DestroyConfirmToken,
};
use qsc::vault::{
    secret_get, secret_set, set_process_passphrase, unlock_with_passphrase,
    vault_init_with_passphrase,
};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, MutexGuard, OnceLock};

const PASS: &str = "na0696-lane-pass";
const WRONG: &str = "na0696-wrong-pass";
const PEER: &str = "alice";
const ROUTE_TOKEN_PEER: &str = "route_token_peer_abcdefghijklmnopq";

static ENV_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

fn env_lock() -> MutexGuard<'static, ()> {
    ENV_LOCK
        .get_or_init(|| Mutex::new(()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

fn ensure_dir_700(path: &Path) {
    fs::create_dir_all(path).unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(path, fs::Permissions::from_mode(0o700)).unwrap();
    }
}

fn test_root() -> PathBuf {
    let root = if let Ok(v) = std::env::var("QSC_TEST_ROOT") {
        PathBuf::from(v)
    } else if let Ok(v) = std::env::var("CARGO_TARGET_DIR") {
        PathBuf::from(v)
    } else {
        PathBuf::from("target")
    };
    let root = root.join("qsc-test-tmp").join("na0696-vault-honesty");
    ensure_dir_700(&root);
    root
}

fn fresh_base(tag: &str) -> PathBuf {
    let base = test_root().join(format!("{}_{}", tag, std::process::id()));
    if base.exists() {
        fs::remove_dir_all(&base).unwrap();
    }
    ensure_dir_700(&base);
    base
}

fn qsc_cmd() -> assert_cmd::Command {
    assert_cmd::cargo::cargo_bin_cmd!("qsc")
}

/// Point QSC_CONFIG_DIR at a FRESH per-test dir and reset every piece of
/// process-global state the lib surface can touch (the NA-0658/na0693 pattern).
fn fresh_lib_env(tag: &str) -> PathBuf {
    let base = fresh_base(tag);
    let cfg = base.join("cfg");
    std::env::set_var("QSC_CONFIG_DIR", &cfg);
    std::env::set_var("QSC_DISABLE_KEYCHAIN", "1");
    std::env::remove_var("QSC_KEYCHAIN_TEST_SEAM");
    std::env::remove_var("QSC_MARK_FORMAT");
    set_process_passphrase(None);
    set_vault_unlocked(false);
    set_marker_routing(MarkerRouting::InApp);
    let _ = marker_queue()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .drain(..)
        .collect::<Vec<_>>();
    cfg
}

/// (i) The D1(c) transaction observed END TO END on a real plain send: the spawned
/// DEBUG binary holds the commit lock while the session-blob store and the timeline
/// ingest nest through the reentrant registry — the child's stderr carries the Q4/F5
/// instrument (`qsc_lock_nested_acquire `), and the transaction LANDS (send_commit
/// marker + the timeline row). Log-not-assert on the src side; asserted here.
#[test]
fn nested_commit_emits_debug_instrument_and_transaction_lands() {
    let _g = env_lock();
    let base = fresh_base("nested_commit");
    let cfg = base.join("cfg");
    ensure_dir_700(&cfg);
    common::init_mock_vault(&cfg);

    let contacts = common::qsc_std_command()
        .env("QSC_CONFIG_DIR", &cfg)
        .env("QSC_QSP_SEED", "1")
        .env("QSC_ALLOW_SEED_FALLBACK", "1")
        .env("QSC_UNSAFE_TEST_SEED_FALLBACK", "1")
        .args([
            "contacts",
            "add",
            "--label",
            PEER,
            "--fp",
            "fp-na0696",
            "--route-token",
            ROUTE_TOKEN_PEER,
        ])
        .output()
        .expect("contacts add");
    assert!(
        contacts.status.success(),
        "contacts add failed: {}{}",
        String::from_utf8_lossy(&contacts.stdout),
        String::from_utf8_lossy(&contacts.stderr)
    );

    let relay = common::start_inbox_server(1024 * 1024, 16);
    let payload = base.join("msg.bin");
    fs::write(&payload, b"na0696 nested commit payload").expect("write payload");

    let send = common::qsc_std_command()
        .env("QSC_CONFIG_DIR", &cfg)
        .env("QSC_QSP_SEED", "1")
        .env("QSC_ALLOW_SEED_FALLBACK", "1")
        .env("QSC_UNSAFE_TEST_SEED_FALLBACK", "1")
        .env("QSC_MARK_FORMAT", "plain")
        .args([
            "send",
            "--transport",
            "relay",
            "--relay",
            relay.base_url(),
            "--to",
            PEER,
            "--file",
            payload.to_str().expect("payload path"),
        ])
        .output()
        .expect("send");
    let send_stdout = String::from_utf8_lossy(&send.stdout);
    let send_stderr = String::from_utf8_lossy(&send.stderr);
    assert!(
        send.status.success(),
        "send failed: {send_stdout}{send_stderr}"
    );
    assert!(
        send_stdout.contains("send_commit"),
        "the commit transaction must land: {send_stdout}"
    );
    assert!(
        send_stderr.contains("qsc_lock_nested_acquire "),
        "the debug instrument must emit on the REAL nested commit path: {send_stderr}"
    );

    let timeline = common::qsc_std_command()
        .env("QSC_CONFIG_DIR", &cfg)
        .env("QSC_MARK_FORMAT", "plain")
        .args(["timeline", "list", "--peer", PEER, "--limit", "5"])
        .output()
        .expect("timeline list");
    let timeline_stdout = String::from_utf8_lossy(&timeline.stdout);
    assert!(
        timeline.status.success(),
        "timeline list failed: {timeline_stdout}"
    );
    assert!(
        timeline_stdout.contains("count=1"),
        "exactly the committed row: {timeline_stdout}"
    );
}

/// (ii) The destroy boundary (A1.4/R2), enumerated BY NAME as a directory-listing
/// EQUALITY: after a populated destroy the config dir contains EXACTLY the survivors
/// {.qsc.lock, config.txt, store.meta}; every gone-set artifact — the vault file, both
/// protection-state files, and the four vault-keyed satellites — is asserted absent by
/// its own name. Red-capable: it fails the moment a future store forgets itself.
#[test]
fn destroy_residue_set_enumerated_by_name() {
    let _g = env_lock();
    let cfg = fresh_lib_env("residue_set");
    vault_init_with_passphrase(PASS).expect("init");
    unlock_with_passphrase(PASS).expect("unlock");
    secret_set("na0696.residue", "resident-value").expect("seed secret");
    wipe_after_failed_unlocks_arm(3).expect("arm the limit (vault_security.txt)");
    let _ = unlock_guarded_at(WRONG, 1_700_000_000)
        .expect("guarded wrong attempt seeds vault_unlock_failures.txt");
    qsc::config_set("policy-profile", "strict").expect("config.txt exists as a survivor");
    fs::write(cfg.join("send.state"), b"send_seq=3\n").expect("send.state satellite");
    for satellite_dir in ["msgqueue_v1", "quarantine_v1", "attachments"] {
        let dir = cfg.join(satellite_dir);
        ensure_dir_700(&dir);
        fs::write(dir.join("resident.bin"), b"na0696").expect("satellite content");
    }
    // Every named artifact PRESENT before destroy, so no absence below is vacuous.
    for present in [
        "vault.qsv",
        "vault_security.txt",
        "vault_unlock_failures.txt",
        "send.state",
        "msgqueue_v1",
        "quarantine_v1",
        "attachments",
        ".qsc.lock",
        "config.txt",
        "store.meta",
    ] {
        assert!(
            cfg.join(present).exists(),
            "populated before destroy: {present}"
        );
    }

    destroy_with_passphrase(PASS, DestroyConfirmToken::confirm(PASS)).expect("destroy");

    let mut listing: Vec<String> = fs::read_dir(&cfg)
        .expect("config dir readable")
        .map(|e| {
            e.expect("dir entry")
                .file_name()
                .to_string_lossy()
                .into_owned()
        })
        .collect();
    listing.sort();
    assert_eq!(
        listing,
        vec![
            ".qsc.lock".to_string(),
            "config.txt".to_string(),
            "store.meta".to_string(),
        ],
        "survivors BY DESIGN, exactly and only"
    );
    for gone in [
        "vault.qsv",
        "vault_security.txt",
        "vault_unlock_failures.txt",
        "send.state",
        "msgqueue_v1",
        "quarantine_v1",
        "attachments",
    ] {
        assert!(!cfg.join(gone).exists(), "gone with the vault: {gone}");
    }
}

/// (iii) The ks1 flow, byte-unchanged (A1.3: passphrase vaults keep real
/// authentication): a token committed to the WRONG value refuses `vault_locked` with
/// the vault intact and readable; the matching token destroys. Pre-existing-behavior
/// pin — honestly NOT red at base.
#[test]
fn destroy_passphrase_vault_flow_unchanged() {
    let _g = env_lock();
    let cfg = fresh_lib_env("pass_flow");
    vault_init_with_passphrase(PASS).expect("init");
    unlock_with_passphrase(PASS).expect("unlock");
    secret_set("na0696.k", "na0696-v").expect("seed secret");

    assert_eq!(
        destroy_with_passphrase(PASS, DestroyConfirmToken::confirm(WRONG)),
        Err("vault_locked"),
        "wrong commitment keeps the ks1 refusal marker"
    );
    assert!(
        cfg.join("vault.qsv").is_file(),
        "vault intact after refusal"
    );
    unlock_with_passphrase(PASS).expect("still unlockable");
    assert_eq!(
        secret_get("na0696.k").expect("secret readable"),
        Some("na0696-v".to_string()),
        "protected data unchanged after the refused destroy"
    );

    destroy_with_passphrase(PASS, DestroyConfirmToken::confirm(PASS)).expect("destroy");
    assert!(
        !cfg.join("vault.qsv").exists(),
        "destroyed with the matching token"
    );
}

/// (iv) The headless keychain-unsupported pin: the `cfg(not(feature))` arm still
/// refuses explicit keychain with `vault_token_unavailable` — the split did not
/// disturb the refusal. Behavior pin, honest story: NOT red-capable against D5 alone.
#[test]
fn keychain_unsupported_load_split_headless() {
    let _g = env_lock();
    let base = fresh_base("unsupported_split");
    let cfg = base.join("cfg");
    ensure_dir_700(&cfg);

    let mut cmd = qsc_cmd();
    cmd.env("QSC_TEST_ROOT", &base)
        .env("QSC_CONFIG_DIR", &cfg)
        .env("QSC_DISABLE_KEYCHAIN", "1")
        .env_remove("QSC_KEYCHAIN_TEST_SEAM")
        .args([
            "vault",
            "init",
            "--non-interactive",
            "--key-source",
            "keychain",
        ]);
    cmd.assert()
        .failure()
        .stdout(predicate::str::contains("code=vault_token_unavailable"));
    assert!(!cfg.join("vault.qsv").exists(), "no vault file on reject");
}

// ---------------------------------------------------------------------------------------
// Seam-armed arm — the NAMED LANE GATE (D-1335 R3 shape). Compiled only under
// `--cfg qsc_keychain_test_seam` + `--features keychain`; never in the default suite;
// its green lives in the lane-gate evidence (ENG-0112 recorded).
// ---------------------------------------------------------------------------------------
#[cfg(all(feature = "keychain", qsc_keychain_test_seam))]
mod seam_armed {
    use super::*;

    fn init_keychain_cmd(base: &Path, cfg: &Path, seam: &Path) -> assert_cmd::Command {
        let mut cmd = qsc_cmd();
        cmd.env("QSC_TEST_ROOT", base)
            .env("QSC_CONFIG_DIR", cfg)
            .env("QSC_KEYCHAIN_TEST_SEAM", seam)
            .env_remove("QSC_DISABLE_KEYCHAIN")
            .args([
                "vault",
                "init",
                "--non-interactive",
                "--key-source",
                "keychain",
            ]);
        cmd
    }

    fn arm_lib_env(cfg: &Path, seam: &Path) {
        std::env::set_var("QSC_CONFIG_DIR", cfg);
        std::env::set_var("QSC_KEYCHAIN_TEST_SEAM", seam);
        std::env::remove_var("QSC_DISABLE_KEYCHAIN");
        set_process_passphrase(None);
        set_vault_unlocked(false);
    }

    /// (v) ⚠ The load-split acceptance — RED AT BASE (the base's `|_|` collapse reads
    /// every load failure as `vault_locked`): a DELETED seam entry reads EXACTLY
    /// `vault_token_missing`; an UNREADABLE seam dir (the daemon-down class) reads
    /// `vault_token_unavailable`. Decrypt failures are elsewhere and keep
    /// `vault_locked` (untouched by this lane).
    #[test]
    fn keychain_missing_entry_reads_token_missing() {
        let _g = env_lock();
        let base = fresh_base("missing_entry");
        let seam = base.join("seam");
        ensure_dir_700(&seam);
        let cfg = base.join("cfg");
        ensure_dir_700(&cfg);

        init_keychain_cmd(&base, &cfg, &seam).assert().success();
        arm_lib_env(&cfg, &seam);
        secret_set("na0696.k", "na0696-v").expect("openable before the delete");

        let entries: Vec<PathBuf> = fs::read_dir(&seam)
            .expect("seam dir readable")
            .map(|e| e.expect("seam entry").path())
            .collect();
        assert_eq!(entries.len(), 1, "exactly the vault's entry: {entries:?}");
        fs::remove_file(&entries[0]).expect("delete the keychain entry");

        assert_eq!(
            secret_get("na0696.k"),
            Err("vault_token_missing"),
            "a missing keychain entry must name itself — NOT vault_locked"
        );

        // Daemon-down modeled by an unreadable seam dir.
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(&seam, fs::Permissions::from_mode(0o000))
                .expect("make seam dir unreadable");
            assert_eq!(
                secret_get("na0696.k"),
                Err("vault_token_unavailable"),
                "the daemon-down class keeps its own name"
            );
            fs::set_permissions(&seam, fs::Permissions::from_mode(0o700))
                .expect("restore seam dir perms");
        }
    }

    /// (vi) ⚠ The ceremony pin (A1.3/R1) — red-capable: a keychain vault refuses a
    /// token committed to anything but the ceremony word, with the vault INTACT and
    /// still openable; the word destroys.
    #[test]
    fn keychain_destroy_ceremony_requires_destroy_word() {
        let _g = env_lock();
        let base = fresh_base("ceremony");
        let seam = base.join("seam");
        ensure_dir_700(&seam);
        let cfg = base.join("cfg");
        ensure_dir_700(&cfg);

        init_keychain_cmd(&base, &cfg, &seam).assert().success();
        arm_lib_env(&cfg, &seam);
        secret_set("na0696.c", "ceremony-v").expect("seed secret");

        assert_eq!(
            destroy_with_passphrase("anything", DestroyConfirmToken::confirm("WRONG")),
            Err("vault_destroy_confirm_mismatch"),
            "the wrong word refuses with the mismatch marker and no destruction"
        );
        assert!(
            cfg.join("vault.qsv").is_file(),
            "vault intact after refusal"
        );
        assert_eq!(
            secret_get("na0696.c").expect("still openable"),
            Some("ceremony-v".to_string())
        );

        destroy_with_passphrase("anything", DestroyConfirmToken::confirm("DESTROY"))
            .expect("the ceremony word destroys");
        assert!(!cfg.join("vault.qsv").exists(), "destroyed");
    }
}
