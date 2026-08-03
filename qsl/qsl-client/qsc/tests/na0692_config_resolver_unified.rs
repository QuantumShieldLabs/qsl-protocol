// NA-0692 (D626, D-1332): ENG-0109 — one config-directory resolver.
//
// THE PROPERTY PINNED HERE: with a blank or whitespace `QSC_CONFIG_DIR`, the vault
// file is created under the RESOLVED config directory — never as a relative
// `vault.qsv` in the process working directory.
//
// Before this lane, `vault_path_resolved` re-implemented `fs_store::config_dir`
// WITHOUT its `!v.trim().is_empty()` guard, so a blank override put the vault at a
// relative path in the CWD while the lock, the protection state and the store
// metadata fell through to the XDG location: the vault and the unlock counter that
// is supposed to limit attempts against it ended up in different directories, and
// the attempt limit stopped travelling with the vault it governs.
//
// External-crate-shaped: only the pub library surface is touched (the NA-0658
// pattern), reaching the private resolver through
// `vault_init_with_passphrase` -> `vault_init_core` -> `vault_path_resolved`.
//
// ⚠ BOTH TESTS MUTATE PROCESS-GLOBAL STATE — the environment AND the working
// directory — so every test serializes on this file's own ENV_LOCK, and every
// variable touched is snapshotted and restored INCLUDING THE UNSET CASE. Restore
// runs from `Drop`, so it happens even when an assertion panics.
//
// ⚠ THE CWD MOVE IS A SAFETY REQUIREMENT, NOT A CONVENIENCE. With the delegation
// reverted, the red state writes into the process working directory — which for a
// cargo integration test is the crate root: the whitespace case creates a directory
// literally named "   " containing a real `vault.qsv`. Moving the CWD into a temp
// dir makes the relative-path case contained and directly observable instead of
// destructive. This file must not later acquire a test that does not take the lock.

use qsc::vault::vault_init_with_passphrase;
use std::fs;
use std::path::PathBuf;
use std::sync::{Mutex, MutexGuard, OnceLock};

const PASS: &str = "na0692-lane-pass";

/// Every process-global variable these tests write. Snapshotted as a set so a later
/// edit cannot add a write without adding a restore.
const TOUCHED_VARS: [&str; 4] = [
    "QSC_CONFIG_DIR",
    "XDG_CONFIG_HOME",
    "HOME",
    "QSC_DISABLE_KEYCHAIN",
];

static ENV_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

fn env_lock() -> MutexGuard<'static, ()> {
    ENV_LOCK
        .get_or_init(|| Mutex::new(()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

/// Snapshot of the process-global state these tests mutate. Restored on `Drop` —
/// including the unset case, and including the working directory — so a panicking
/// assertion cannot leave the process pointing at a temp dir that is about to be
/// deleted. Restore never panics: a panic in `Drop` during unwind aborts the process.
struct ProcessStateSnapshot {
    vars: Vec<(&'static str, Option<String>)>,
    cwd: PathBuf,
}

impl ProcessStateSnapshot {
    fn take() -> Self {
        Self {
            vars: TOUCHED_VARS
                .iter()
                .map(|k| (*k, std::env::var(k).ok()))
                .collect(),
            cwd: std::env::current_dir().expect("current_dir must be readable"),
        }
    }
}

impl Drop for ProcessStateSnapshot {
    fn drop(&mut self) {
        let _ = std::env::set_current_dir(&self.cwd);
        for (key, value) in &self.vars {
            match value {
                Some(v) => std::env::set_var(key, v),
                None => std::env::remove_var(key),
            }
        }
    }
}

/// A fresh, ABSOLUTE case root. Absolute matters: these tests move the CWD, and a
/// relative root would silently re-anchor when they do.
fn case_root(tag: &str) -> PathBuf {
    let base = if let Ok(v) = std::env::var("QSC_TEST_ROOT") {
        PathBuf::from(v)
    } else if let Ok(v) = std::env::var("CARGO_TARGET_DIR") {
        PathBuf::from(v)
    } else {
        PathBuf::from("target")
    };
    let root = base
        .join("qsc-test-tmp")
        .join("na0692-config-resolver")
        .join(format!("{}_{}", tag, std::process::id()));
    if root.exists() {
        fs::remove_dir_all(&root).expect("clear case root");
    }
    fs::create_dir_all(&root).expect("create case root");
    fs::canonicalize(&root).expect("case root must resolve to an absolute path")
}

/// The shared body of both tests: the only difference is the blank-like value.
///
/// The whole environment is hermetic — `HOME` is redirected too, so that no
/// fall-through can reach the real home directory in any state of the resolver.
fn a_blank_like_override_puts_the_vault_under_the_config_dir(tag: &str, blank_value: &str) {
    let _guard = env_lock();
    let _snapshot = ProcessStateSnapshot::take();

    let root = case_root(tag);
    let xdg = root.join("xdg");
    let home = root.join("home");
    let cwd = root.join("cwd");
    for dir in [&xdg, &home, &cwd] {
        fs::create_dir_all(dir).expect("create case dir");
    }

    std::env::set_var("QSC_CONFIG_DIR", blank_value);
    std::env::set_var("XDG_CONFIG_HOME", &xdg);
    std::env::set_var("HOME", &home);
    std::env::set_var("QSC_DISABLE_KEYCHAIN", "1");
    std::env::set_current_dir(&cwd).expect("move the process CWD into the case dir");

    let result = vault_init_with_passphrase(PASS);

    let expected_dir = xdg.join("qsc");
    let expected_vault = expected_dir.join("vault.qsv");
    let stray_vault = cwd.join("vault.qsv");
    let mut cwd_entries: Vec<String> = fs::read_dir(&cwd)
        .expect("read the case CWD")
        .map(|e| {
            e.expect("dir entry")
                .file_name()
                .to_string_lossy()
                .into_owned()
        })
        .collect();
    cwd_entries.sort();

    assert!(
        result.is_ok(),
        "a blank-like QSC_CONFIG_DIR ({:?}) must fall through to XDG, not fail: {:?}",
        blank_value,
        result
    );
    assert!(
        expected_vault.exists(),
        "the vault must be created at {} (blank-like QSC_CONFIG_DIR = {:?})",
        expected_vault.display(),
        blank_value
    );
    assert!(
        !stray_vault.exists(),
        "the vault must never be created as a relative path in the process CWD ({})",
        stray_vault.display()
    );
    // The strongest form of the property: the resolver wrote NOTHING into the
    // process working directory. The `!stray_vault.exists()` assertion above alone
    // would pass in the whitespace red state, which creates `<cwd>/   /vault.qsv`.
    assert!(
        cwd_entries.is_empty(),
        "nothing may be written into the process CWD; found {:?} (blank-like QSC_CONFIG_DIR = {:?})",
        cwd_entries,
        blank_value
    );
    assert_eq!(
        expected_vault.parent(),
        Some(expected_dir.as_path()),
        "the vault's parent must be the resolved config directory"
    );
}

#[test]
fn a_blank_config_dir_override_creates_the_vault_under_the_config_dir_not_the_cwd() {
    a_blank_like_override_puts_the_vault_under_the_config_dir("blank", "");
}

#[test]
fn a_whitespace_config_dir_override_behaves_exactly_as_a_blank_one() {
    a_blank_like_override_puts_the_vault_under_the_config_dir("whitespace", "   ");
}
