// NA-0693 (D627, D-1333): ENG-0106 — the vault write lock, observed through the pub
// surface only. Harness extends NA_0658_vault_protections.rs's proven pattern:
// every test serializes on ENV_LOCK, points QSC_CONFIG_DIR at a fresh per-test dir,
// disables the keychain, and resets process-global state up front.
//
// The three instruments, each red-capable (D627 §7.6):
// - destroy_under_lock_succeeds_with_armed_limit: destroy performs its nested
//   protection-state clear THROUGH the unlocked inner variant while holding the
//   store lock — the op succeeds instead of self-denying.
// - destroy_clears_protection_state_observed_via_pub_surface: ⚠ THE LOAD-BEARING
//   REENTRANCY REGRESSION PROOF (Director ruling at STOP 0): under the ruled
//   loud-not-fail disposition for the destroy clear site, the naive self-colliding
//   lock plus a restored `let _ =` keeps destroy returning Ok while the armed limit
//   silently survives — THIS test is the one that goes red against that variant.
// - secret_set_fails_closed_while_store_lock_held: deterministic serialization —
//   the test itself holds `flock(LOCK_EX)` on the store's `.qsc.lock` (no thread
//   race), and a write that cannot take the lock is refused, not interleaved.
//   Red at base by construction: an unlocked secret_set succeeds under a held lock.

use qsc::output::{marker_queue, set_marker_routing, MarkerRouting};
use qsc::set_vault_unlocked;
use qsc::vault::protection::{
    destroy_with_passphrase, wipe_after_failed_unlocks_arm, wipe_after_failed_unlocks_limit,
    DestroyConfirmToken,
};
use qsc::vault::{
    secret_set, set_process_passphrase, unlock_with_passphrase, vault_init_with_passphrase,
};
use std::fs;
use std::path::PathBuf;
use std::sync::{Mutex, MutexGuard, OnceLock};

const PASS: &str = "na0693-lane-pass";

static ENV_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

fn env_lock() -> MutexGuard<'static, ()> {
    ENV_LOCK
        .get_or_init(|| Mutex::new(()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

fn ensure_dir_700(path: &PathBuf) {
    fs::create_dir_all(path).unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(path, fs::Permissions::from_mode(0o700)).unwrap();
    }
}

fn safe_test_root() -> PathBuf {
    let root = if let Ok(v) = std::env::var("QSC_TEST_ROOT") {
        PathBuf::from(v)
    } else if let Ok(v) = std::env::var("CARGO_TARGET_DIR") {
        PathBuf::from(v)
    } else {
        PathBuf::from("target")
    };
    let root = root.join("qsc-test-tmp").join("na0693-vault-write-lock");
    ensure_dir_700(&root);
    root
}

/// Point QSC_CONFIG_DIR at a FRESH per-test config dir and reset every piece of
/// process-global state the lane surface can touch (the NA-0658 pattern).
fn fresh_test_env(tag: &str) -> PathBuf {
    let case_root = safe_test_root().join(format!("{}_{}", tag, std::process::id()));
    if case_root.exists() {
        fs::remove_dir_all(&case_root).unwrap();
    }
    ensure_dir_700(&case_root);
    let cfg = case_root.join("cfg");
    std::env::set_var("QSC_CONFIG_DIR", &cfg);
    std::env::set_var("QSC_DISABLE_KEYCHAIN", "1");
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

/// (i) A vault op that touches protection state UNDER the store lock succeeds:
/// destroy holds the exclusive lock for its whole transaction and still clears the
/// armed protection state through the inner variant instead of self-denying.
#[test]
fn destroy_under_lock_succeeds_with_armed_limit() {
    let _g = env_lock();
    let cfg = fresh_test_env("destroy_under_lock");
    vault_init_with_passphrase(PASS).expect("init");
    wipe_after_failed_unlocks_arm(3).expect("arm the wipe-after-N limit");

    destroy_with_passphrase(PASS, DestroyConfirmToken::confirm_with_passphrase(PASS))
        .expect("destroy must succeed while holding the store lock");

    assert!(!cfg.join("vault.qsv").exists(), "vault file removed");
}

/// (ii) ⚠ The load-bearing reentrancy regression proof (see the file header):
/// destroy CLEARS the protection state, observed purely through the pub surface —
/// the state filenames are pub(crate) and invisible here by design.
#[test]
fn destroy_clears_protection_state_observed_via_pub_surface() {
    let _g = env_lock();
    let _cfg = fresh_test_env("destroy_clears");
    wipe_after_failed_unlocks_arm(3).expect("arm the wipe-after-N limit");
    assert_eq!(
        wipe_after_failed_unlocks_limit(),
        Ok(Some(3)),
        "arming must be observable before destroy"
    );
    vault_init_with_passphrase(PASS).expect("init");

    destroy_with_passphrase(PASS, DestroyConfirmToken::confirm_with_passphrase(PASS))
        .expect("destroy");

    assert_eq!(
        wipe_after_failed_unlocks_limit(),
        Ok(None),
        "destroy must clear the protection state for the next vault"
    );
}

/// (iii) Deterministic serialization: the test holds `flock(LOCK_EX)` on the
/// store's `.qsc.lock` directly (same open flags as the lock helpers; the extern
/// mirrors lib.rs's own declaration — no new dependency), and a write that cannot
/// take the lock FAILS CLOSED with the ruled marker instead of interleaving.
/// Release, and the same write succeeds — the in-test green control.
#[cfg(unix)]
#[test]
fn secret_set_fails_closed_while_store_lock_held() {
    use std::os::unix::io::AsRawFd;
    extern "C" {
        fn flock(fd: i32, operation: i32) -> i32;
    }
    const LOCK_EX: i32 = 2;
    const LOCK_NB: i32 = 4;

    let _g = env_lock();
    let cfg = fresh_test_env("serialization");
    vault_init_with_passphrase(PASS).expect("init");
    unlock_with_passphrase(PASS).expect("unlock");
    secret_set("na0693-pre", "v").expect("an unlocked-store write succeeds");

    let lock_file = fs::OpenOptions::new()
        .create(true)
        .truncate(false)
        .read(true)
        .write(true)
        .open(cfg.join(".qsc.lock"))
        .expect("open the store lock file");
    let rc = unsafe { flock(lock_file.as_raw_fd(), LOCK_EX | LOCK_NB) };
    assert_eq!(rc, 0, "the test must be able to take the store lock");

    assert_eq!(
        secret_set("na0693-held", "v"),
        Err("lock_contended"),
        "a write that cannot take the store lock must fail closed"
    );

    drop(lock_file);
    secret_set("na0693-held", "v").expect("the same write succeeds after release");
}
