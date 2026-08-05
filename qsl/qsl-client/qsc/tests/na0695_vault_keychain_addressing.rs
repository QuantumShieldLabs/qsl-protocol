// NA-0695 (D629 as amended, D-1335): ENG-0108 — the keychain ACCOUNT is per-vault
// ("vault-" + raw hex of the envelope salt, ONE derivation fn; R1/E-A) and init REFUSES an
// existing entry instead of overwriting it (`ProviderError::EntryExists` →
// `vault_keychain_entry_exists`, R4). One OS-keychain slot can no longer serve every vault
// on the machine, so a second profile's `vault init --key-source keychain` can no longer
// silently and permanently destroy the first profile's key (the ledger's P2 sentence).
//
// The instruments and their honest red-stories (D629 §4c):
// (i)   explicit-keychain-unsupported refuses without mutation — pins the previously
//       UNPINNED `vault_token_unavailable` refusal (the first headless pin; §0.6).
//       Behavior pin: PRE-EXISTING behavior, honestly NOT red at base.
// (ii)  seam-inert-without-cfg — the `QSC_KEYCHAIN_TEST_SEAM` env var on a default-built
//       binary changes NOTHING and conjures NO store (the rng-seam twin-arm property; the
//       cfg-fenced seam is compiled out entirely without `--cfg qsc_keychain_test_seam`).
// (iii) ⚠ THE ACCEPTANCE — `two_profiles_both_stay_openable`: RED AT BASE in the overwrite
//       mode (Control AB's shape: profile 2's init succeeds silently and profile 1 reads
//       `vault_locked`). GREEN after: distinct salts → distinct accounts → both openable.
// (iv)  reinit-after-manual-delete → a DISJOINT entry; the old entry is orphaned residue,
//       recorded and NEVER cleaned or reused (the R2 hard-break posture, observed).
// (v)   destroy removes exactly its own entry — the other profile's key survives.
// The same-salt store collision (the §5b refuse, directly) is unreachable from this file —
// init always draws a fresh salt — and lives as the in-src seam unit in `vault/mod.rs`.
//
// Seam-armed arm (iii)-(v) is a NAMED LANE GATE outside CI (R3, recorded against
// ENG-0112's open half): it compiles only under
// `RUSTFLAGS="--cfg qsc_keychain_test_seam"` + `--features keychain`, never in the
// default 129-binary suite. Recorded untestable-headless (D627-A4, applied not deferred):
// the REAL keyring plumbing — `Entry::new`/`set_password`/`get_password`/
// `delete_credential` against a live Secret Service / macOS Keychain / Credential
// Manager — is NOT exercised by any of these tests and is NOT claimable from this lane;
// it is covered by the structural gates (the derivation and refuse sit in the SAME
// product functions the real backend uses; the seam swaps only the raw storage
// primitives, E-B), the `--features keychain` compile gate, and real-OS manual
// verification per the existing runbook posture.
//
// Harness: the NA-0693/0694 pattern — ENV_LOCK serialization, fresh per-test dirs
// chmod 0700 after create_dir_all (the Slice-B W1/X1 lesson), CLI/pub surface only.

// A test binary is its own crate root, so the lib-side allow does not reach it (the rng-seam
// test files carry the same attribute for the same custom cfg reason).
#![allow(unexpected_cfgs)]

use predicates::prelude::*;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, MutexGuard, OnceLock};

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
    let root = root
        .join("qsc-test-tmp")
        .join("na0695-vault-keychain-addressing");
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

/// (i) The first headless pin of the explicit-keychain-unsupported refusal (§0.6: ZERO
/// tests asserted `vault_token_unavailable` before this lane). Behavior pin — honest
/// story: pre-existing behavior, NOT red at base. The refusal fires in `vault_init`
/// BEFORE `vault_init_core`, hence before the config-dir resolve and the store lock, so
/// the config dir must stay COMPLETELY untouched (not even the lock artifact).
#[test]
fn keychain_explicit_unsupported_refuses_without_mutation() {
    let _g = env_lock();
    let base = fresh_base("explicit_unsupported");
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
        .stdout(predicate::str::contains("QSC_MARK/1 event=error"))
        .stdout(predicate::str::contains("code=vault_token_unavailable"));

    assert!(!cfg.join("vault.qsv").exists(), "no vault file on reject");
    let leftover: Vec<_> = fs::read_dir(&cfg)
        .expect("config dir readable")
        .map(|e| e.expect("dir entry").file_name())
        .collect();
    assert!(
        leftover.is_empty(),
        "explicit-unsupported reject must not mutate the config dir: {:?}",
        leftover
    );
}

/// (ii) The twin-arm inertness pin (the b1 rng-seam property): on a binary built WITHOUT
/// `--cfg qsc_keychain_test_seam`, setting `QSC_KEYCHAIN_TEST_SEAM` changes NOTHING — the
/// refusal is byte-identical to (i) and the named seam directory stays EMPTY. The env
/// read itself is cfg-fenced in src, so a production build cannot reach a file-backed
/// store under any environment.
#[cfg(not(qsc_keychain_test_seam))]
#[test]
fn keychain_seam_inert_without_cfg() {
    let _g = env_lock();
    let base = fresh_base("seam_inert");
    let cfg = base.join("cfg");
    ensure_dir_700(&cfg);
    let seam = base.join("seam");
    ensure_dir_700(&seam);

    let mut cmd = qsc_cmd();
    cmd.env("QSC_TEST_ROOT", &base)
        .env("QSC_CONFIG_DIR", &cfg)
        .env("QSC_DISABLE_KEYCHAIN", "1")
        .env("QSC_KEYCHAIN_TEST_SEAM", &seam)
        .args([
            "vault",
            "init",
            "--non-interactive",
            "--key-source",
            "keychain",
        ]);
    cmd.assert()
        .failure()
        .stdout(predicate::str::contains("QSC_MARK/1 event=error"))
        .stdout(predicate::str::contains("code=vault_token_unavailable"));

    assert!(!cfg.join("vault.qsv").exists(), "no vault file on reject");
    let seam_leftover: Vec<_> = fs::read_dir(&seam)
        .expect("seam dir readable")
        .map(|e| e.expect("dir entry").file_name())
        .collect();
    assert!(
        seam_leftover.is_empty(),
        "the env var alone must never conjure a store in a production build: {:?}",
        seam_leftover
    );
}

// ---------------------------------------------------------------------------------------
// Seam-armed arm — the NAMED LANE GATE (R3). Compiled only under
// `--cfg qsc_keychain_test_seam` + `--features keychain`; drives spawned `qsc` binaries
// (the seam store is file-backed precisely because a real keychain is cross-process
// state) plus the pub library surface.
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

    fn status_cmd(base: &Path, cfg: &Path, seam: &Path) -> assert_cmd::Command {
        let mut cmd = qsc_cmd();
        cmd.env("QSC_TEST_ROOT", base)
            .env("QSC_CONFIG_DIR", cfg)
            .env("QSC_KEYCHAIN_TEST_SEAM", seam)
            .env_remove("QSC_DISABLE_KEYCHAIN")
            .args(["vault", "status"]);
        cmd
    }

    /// (file name, contents) for every seam entry, sorted by name.
    fn seam_entries(seam: &Path) -> Vec<(String, String)> {
        let mut out: Vec<(String, String)> = fs::read_dir(seam)
            .expect("seam dir readable")
            .map(|e| {
                let e = e.expect("seam entry");
                let name = e.file_name().to_string_lossy().into_owned();
                let body = fs::read_to_string(e.path()).expect("seam entry readable");
                (name, body)
            })
            .collect();
        out.sort();
        out
    }

    fn assert_account_shape(name: &str) {
        // service "qsc" ‖ "__" ‖ "vault-" ‖ 32 raw-hex chars (R1: raw hex, 38-char account).
        assert!(
            name.starts_with("qsc__vault-"),
            "seam entry must be per-vault addressed: {name}"
        );
        let hex = &name["qsc__vault-".len()..];
        assert_eq!(hex.len(), 32, "16-byte salt as raw hex: {name}");
        assert!(
            hex.chars().all(|c| c.is_ascii_hexdigit()),
            "raw-hex account: {name}"
        );
    }

    /// In-process roundtrip through the pub surface — a SECOND process relative to the
    /// spawned init, so the seam store's cross-process property is exercised, not assumed.
    fn roundtrip(cfg: &Path, seam: &Path, key: &str, value: &str) {
        std::env::set_var("QSC_CONFIG_DIR", cfg);
        std::env::set_var("QSC_KEYCHAIN_TEST_SEAM", seam);
        std::env::remove_var("QSC_DISABLE_KEYCHAIN");
        qsc::vault::secret_set(key, value).expect("secret set");
        let got = qsc::vault::secret_get(key).expect("secret get");
        assert_eq!(got.as_deref(), Some(value), "roundtrip value");
    }

    /// (iii) ⚠ THE ACCEPTANCE (banked; the ledger's test shape verbatim): two profiles
    /// under different config directories, ONE keychain (one seam store, outside both),
    /// BOTH stay openable. RED AT BASE in the overwrite mode — Control AB observes the
    /// P2 sentence: profile 2's init succeeds silently and profile 1 reads `vault_locked`.
    #[test]
    fn two_profiles_both_stay_openable() {
        let _g = env_lock();
        let base = fresh_base("two_profiles");
        let seam = base.join("seam");
        ensure_dir_700(&seam);
        let cfg_a = base.join("cfg_a");
        ensure_dir_700(&cfg_a);
        let cfg_b = base.join("cfg_b");
        ensure_dir_700(&cfg_b);

        init_keychain_cmd(&base, &cfg_a, &seam)
            .assert()
            .success()
            .stdout(predicate::str::contains("event=vault_init"));
        init_keychain_cmd(&base, &cfg_b, &seam)
            .assert()
            .success()
            .stdout(predicate::str::contains("event=vault_init"));

        // Profile /a FIRST — under the base (overwrite) shape this is the vault whose key
        // is destroyed, and this roundtrip is where Control AB goes red with the ledger's
        // P2 sentence observed literally: `secret set` on profile /a reads `vault_locked`.
        roundtrip(&cfg_a, &seam, "na0695.probe.a", "profile-a-value");
        status_cmd(&base, &cfg_a, &seam)
            .assert()
            .success()
            .stdout(predicate::str::contains("key_source=keychain"));
        roundtrip(&cfg_b, &seam, "na0695.probe.b", "profile-b-value");
        status_cmd(&base, &cfg_b, &seam)
            .assert()
            .success()
            .stdout(predicate::str::contains("key_source=keychain"));

        let entries = seam_entries(&seam);
        assert_eq!(entries.len(), 2, "one slot per vault: {:?}", entries);
        assert_ne!(entries[0].0, entries[1].0, "distinct per-vault accounts");
        assert_account_shape(&entries[0].0);
        assert_account_shape(&entries[1].0);
    }

    /// (iv) Re-init after a MANUAL vault-file delete (not destroy: the keychain entry
    /// survives) draws a fresh salt → a DISJOINT entry. The old entry is orphaned residue:
    /// recorded, byte-untouched, never reused (R2 — cleanup would resurrect the fixed-slot
    /// read path this lane retires; a pre-D legacy entry is the same class of residue).
    #[test]
    fn reinit_after_manual_vault_delete_creates_disjoint_entry() {
        let _g = env_lock();
        let base = fresh_base("reinit_disjoint");
        let seam = base.join("seam");
        ensure_dir_700(&seam);
        let cfg = base.join("cfg");
        ensure_dir_700(&cfg);

        init_keychain_cmd(&base, &cfg, &seam).assert().success();
        let before = seam_entries(&seam);
        assert_eq!(before.len(), 1, "first init stores one entry");
        let (orphan_name, orphan_body) = before[0].clone();

        fs::remove_file(cfg.join("vault.qsv")).expect("manual vault delete");

        init_keychain_cmd(&base, &cfg, &seam).assert().success();
        let after = seam_entries(&seam);
        assert_eq!(
            after.len(),
            2,
            "re-init must create a DISJOINT entry and orphan the old one: {:?}",
            after
        );
        let orphan_still = after
            .iter()
            .find(|(name, _)| *name == orphan_name)
            .expect("orphaned entry recorded, not cleaned (R2)");
        assert_eq!(
            orphan_still.1, orphan_body,
            "orphaned entry byte-untouched, its key never reused"
        );
        let new_entry = after
            .iter()
            .find(|(name, _)| *name != orphan_name)
            .expect("new per-vault entry");
        assert_account_shape(&new_entry.0);
        assert_ne!(
            new_entry.1, orphan_body,
            "fresh key material in the new slot"
        );

        roundtrip(&cfg, &seam, "na0695.probe.reinit", "reinit-value");
    }

    /// (v) Destroy removes EXACTLY its own entry: the other profile's key survives and its
    /// vault stays openable. (Destroy's passphrase is any non-empty string for a keychain
    /// vault — the derivation ignores it and only the decrypt validates; E-D observation
    /// (1), recorded in D-1335 for Slice E, deliberately not asserted as a specification.)
    #[test]
    fn destroy_removes_exactly_its_own_entry() {
        let _g = env_lock();
        let base = fresh_base("destroy_own_entry");
        let seam = base.join("seam");
        ensure_dir_700(&seam);
        let cfg_a = base.join("cfg_a");
        ensure_dir_700(&cfg_a);
        let cfg_b = base.join("cfg_b");
        ensure_dir_700(&cfg_b);

        init_keychain_cmd(&base, &cfg_a, &seam).assert().success();
        let after_a = seam_entries(&seam);
        assert_eq!(after_a.len(), 1);
        init_keychain_cmd(&base, &cfg_b, &seam).assert().success();
        let after_b = seam_entries(&seam);
        assert_eq!(after_b.len(), 2);
        let b_entry = after_b
            .iter()
            .find(|(name, _)| *name != after_a[0].0)
            .expect("profile B's entry")
            .clone();

        std::env::set_var("QSC_CONFIG_DIR", &cfg_a);
        std::env::set_var("QSC_KEYCHAIN_TEST_SEAM", &seam);
        std::env::remove_var("QSC_DISABLE_KEYCHAIN");
        qsc::vault::protection::destroy_with_passphrase(
            "na0695-destroy",
            qsc::vault::protection::DestroyConfirmToken::confirm_with_passphrase("na0695-destroy"),
        )
        .expect("destroy profile A");

        let remaining = seam_entries(&seam);
        assert_eq!(
            remaining.len(),
            1,
            "destroy removes exactly its own entry: {:?}",
            remaining
        );
        assert_eq!(remaining[0], b_entry, "profile B's entry untouched");
        assert!(
            !cfg_a.join("vault.qsv").exists(),
            "profile A's vault erased"
        );

        roundtrip(&cfg_b, &seam, "na0695.probe.survivor", "b-still-opens");
    }
}
