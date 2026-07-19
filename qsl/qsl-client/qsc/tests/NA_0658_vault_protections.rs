// NA-0658 (D594, D-1281): library-level coverage for the restored ENG-0044
// vault-protection surface — test families 1-4 of the directive: (1) escalating
// delay incl. persistence-across-simulated-restart and clock rollback, (2) the
// explicit opt-in wipe with the unarmed default proven safe, (3) lock() as one
// idempotent operation, (4) destroy irreversibility + the confirmation-token
// requirement. External-crate-shaped: only the pub library surface is touched
// (the NA-0649 pattern); the CLI byte-identity spot-check is lane-run evidence.
//
// Delay tests never sleep: `unlock_guarded_at`/`protection_status_at` are the
// design-locked clock seam and take fabricated unix-second readings as values.
//
// Process-global state (env vars, the process passphrase, the unlocked flag,
// marker routing/queue) is shared across tests in this binary, so every test
// serializes on ENV_LOCK and resets that state up front.

use qsc::output::{marker_queue, set_marker_routing, MarkerRouting};
use qsc::store::{
    QSC_ERR_VAULT_WIPED_AFTER_FAILED_UNLOCKS, VAULT_ATTEMPT_LIMIT_MAX, VAULT_ATTEMPT_LIMIT_MIN,
};
use qsc::vault::protection::{
    destroy_with_passphrase, lock, protection_status, protection_status_at,
    unlock_delay_schedule_s, unlock_guarded, unlock_guarded_at, wipe_after_failed_unlocks_arm,
    wipe_after_failed_unlocks_disarm, wipe_after_failed_unlocks_limit, DestroyConfirmToken,
    GuardedUnlockOutcome,
};
use qsc::vault::{
    has_process_passphrase, open_session_with_passphrase, secret_get, secret_set,
    set_process_passphrase, unlock_with_passphrase, vault_init_with_passphrase,
};
use qsc::{set_vault_unlocked, vault_unlocked};
use std::fs;
use std::path::PathBuf;
use std::sync::{Mutex, MutexGuard, OnceLock};

const PASS: &str = "na0658-lane-pass";
const WRONG: &str = "na0658-wrong-pass";
const CONFIG_FILE: &str = "vault_security.txt";
const COUNTER_FILE: &str = "vault_unlock_failures.txt";
// A fabricated epoch far from the real clock so rollback semantics are explicit.
const T0: u64 = 1_000_000;

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
    let root = root.join("qsc-test-tmp").join("na0658-vault-protections");
    ensure_dir_700(&root);
    root
}

/// Point QSC_CONFIG_DIR at a FRESH per-test config dir and reset every piece of
/// process-global state the lane surface can touch.
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
    drain_markers();
    cfg
}

fn drain_markers() -> Vec<String> {
    marker_queue()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .drain(..)
        .collect()
}

fn read_file(path: &PathBuf) -> String {
    fs::read_to_string(path).unwrap_or_default()
}

// ---------------------------------------------------------------------------
// Family 1 — escalating delay (default-on through the guarded path)
// ---------------------------------------------------------------------------

#[test]
fn delay_schedule_matches_the_accepted_rails() {
    // failures 1-2 free; from the 3rd: 5 s doubling capped at 300 s.
    assert_eq!(unlock_delay_schedule_s(0), 0);
    assert_eq!(unlock_delay_schedule_s(1), 0);
    assert_eq!(unlock_delay_schedule_s(2), 0);
    assert_eq!(unlock_delay_schedule_s(3), 5);
    assert_eq!(unlock_delay_schedule_s(4), 10);
    assert_eq!(unlock_delay_schedule_s(5), 20);
    assert_eq!(unlock_delay_schedule_s(6), 40);
    assert_eq!(unlock_delay_schedule_s(7), 80);
    assert_eq!(unlock_delay_schedule_s(8), 160);
    assert_eq!(unlock_delay_schedule_s(9), 300);
    assert_eq!(unlock_delay_schedule_s(10), 300);
    assert_eq!(unlock_delay_schedule_s(u32::MAX), 300);
    // Monotonic non-decreasing across the whole rail.
    let mut prev = 0;
    for n in 0..=64u32 {
        let d = unlock_delay_schedule_s(n);
        assert!(d >= prev, "schedule must never decrease (n={})", n);
        prev = d;
    }
}

#[test]
fn escalating_delay_engages_counts_and_grows() {
    let _g = env_lock();
    fresh_test_env("delay_grows");
    vault_init_with_passphrase(PASS).expect("init");

    // Failures 1-2 are free: counted, no delay engaged.
    assert_eq!(
        unlock_guarded_at(WRONG, T0),
        Ok(GuardedUnlockOutcome::Rejected {
            failed_unlocks: 1,
            retry_after_s: 0
        })
    );
    assert_eq!(
        unlock_guarded_at(WRONG, T0),
        Ok(GuardedUnlockOutcome::Rejected {
            failed_unlocks: 2,
            retry_after_s: 0
        })
    );
    // 3rd consecutive failure: the delay engages at 5 s.
    assert_eq!(
        unlock_guarded_at(WRONG, T0),
        Ok(GuardedUnlockOutcome::Rejected {
            failed_unlocks: 3,
            retry_after_s: 5
        })
    );
    // Inside the window even the CORRECT passphrase is refused without decrypting
    // and WITHOUT incrementing (no inflation from impatient retries).
    assert_eq!(
        unlock_guarded_at(PASS, T0 + 1),
        Ok(GuardedUnlockOutcome::Delayed {
            failed_unlocks: 3,
            retry_after_s: 4
        })
    );
    assert!(!has_process_passphrase(), "delay refusal must not decrypt");
    let status = protection_status_at(T0 + 1).expect("status");
    assert_eq!(status.failed_unlocks, 3, "window refusal must not count");
    assert_eq!(status.retry_after_s, 4);

    // After the window elapses the next wrong attempt counts and the delay doubles.
    assert_eq!(
        unlock_guarded_at(WRONG, T0 + 5),
        Ok(GuardedUnlockOutcome::Rejected {
            failed_unlocks: 4,
            retry_after_s: 10
        })
    );
    assert_eq!(
        unlock_guarded_at(WRONG, T0 + 15),
        Ok(GuardedUnlockOutcome::Rejected {
            failed_unlocks: 5,
            retry_after_s: 20
        })
    );
}

#[test]
fn correct_passphrase_after_window_unlocks_and_resets() {
    let _g = env_lock();
    let cfg = fresh_test_env("delay_reset");
    vault_init_with_passphrase(PASS).expect("init");

    for _ in 0..3 {
        unlock_guarded_at(WRONG, T0).expect("guarded attempt");
    }
    // Correct passphrase AFTER the window: unlocks and resets counter + delay.
    assert_eq!(
        unlock_guarded_at(PASS, T0 + 5),
        Ok(GuardedUnlockOutcome::Unlocked)
    );
    assert!(
        has_process_passphrase(),
        "unlock sets the process passphrase"
    );
    assert!(
        vault_unlocked(),
        "guarded unlock sets the unlocked flag (R3)"
    );
    let status = protection_status_at(T0 + 5).expect("status");
    assert_eq!(status.failed_unlocks, 0, "success resets the counter");
    assert_eq!(status.retry_after_s, 0, "success clears the delay");
    assert!(
        read_file(&cfg.join(COUNTER_FILE)).contains("failed_unlocks=0"),
        "reset persisted"
    );
    // The real-clock wrappers ride the same path (fresh state: no wait, no count).
    let real = protection_status().expect("real-clock status");
    assert_eq!(real.failed_unlocks, 0);
    assert_eq!(real.retry_after_s, 0);
}

#[test]
fn persistence_across_simulated_restart_continues_count_and_delay() {
    let _g = env_lock();
    let cfg = fresh_test_env("delay_restart");
    vault_init_with_passphrase(PASS).expect("init");

    for _ in 0..3 {
        unlock_guarded_at(WRONG, T0).expect("guarded attempt");
    }
    // Simulated restart: clear ALL in-process state; disk is the only carrier.
    lock(None);
    let counter_raw = read_file(&cfg.join(COUNTER_FILE));
    assert!(
        counter_raw.contains("failed_unlocks=3"),
        "counter persisted: {counter_raw}"
    );
    assert!(
        counter_raw.contains(&format!("last_failure_unix_s={}", T0)),
        "timestamp persisted: {counter_raw}"
    );

    // A fresh load from disk continues the count and the delay — no reset.
    let status = protection_status_at(T0 + 1).expect("status after restart");
    assert_eq!(status.failed_unlocks, 3);
    assert_eq!(status.retry_after_s, 4);
    assert_eq!(
        unlock_guarded_at(PASS, T0 + 1),
        Ok(GuardedUnlockOutcome::Delayed {
            failed_unlocks: 3,
            retry_after_s: 4
        })
    );
    // And the escalation continues from the persisted state after the window.
    assert_eq!(
        unlock_guarded_at(WRONG, T0 + 5),
        Ok(GuardedUnlockOutcome::Rejected {
            failed_unlocks: 4,
            retry_after_s: 10
        })
    );
}

#[test]
fn clock_rollback_fails_safe() {
    let _g = env_lock();
    fresh_test_env("delay_rollback");
    vault_init_with_passphrase(PASS).expect("init");

    for _ in 0..3 {
        unlock_guarded_at(WRONG, T0).expect("guarded attempt");
    }
    // A clock reading EARLIER than the last failure treats the FULL current delay
    // as unelapsed — it never shortens the wait.
    let status = protection_status_at(T0 - 100).expect("status under rollback");
    assert_eq!(status.retry_after_s, 5, "rollback yields the full delay");
    assert_eq!(
        unlock_guarded_at(PASS, T0 - 100),
        Ok(GuardedUnlockOutcome::Delayed {
            failed_unlocks: 3,
            retry_after_s: 5
        })
    );
    // Forward time still elapses normally.
    let status = protection_status_at(T0 + 4).expect("status forward");
    assert_eq!(status.retry_after_s, 1);
}

// ---------------------------------------------------------------------------
// Family 2 — wipe-after-N: the SEPARATE explicit opt-in
// ---------------------------------------------------------------------------

#[test]
fn unarmed_default_never_wipes() {
    let _g = env_lock();
    let cfg = fresh_test_env("wipe_unarmed");
    vault_init_with_passphrase(PASS).expect("init");
    assert_eq!(
        wipe_after_failed_unlocks_limit(),
        Ok(None),
        "unarmed default"
    );

    // Many wrong attempts with no limit armed: never a wipe, only reject/delay.
    let mut t = T0;
    for round in 1..=10u32 {
        let outcome = unlock_guarded_at(WRONG, t).expect("guarded attempt");
        assert_eq!(
            outcome,
            GuardedUnlockOutcome::Rejected {
                failed_unlocks: round,
                retry_after_s: unlock_delay_schedule_s(round)
            },
            "unarmed round {round} must reject, not wipe"
        );
        t += unlock_delay_schedule_s(round);
    }
    assert!(
        cfg.join("vault.qsv").is_file(),
        "vault must remain present when no limit is armed"
    );
    assert_eq!(wipe_after_failed_unlocks_limit(), Ok(None));
    // A correct passphrase after the window still unlocks.
    assert_eq!(
        unlock_guarded_at(PASS, t),
        Ok(GuardedUnlockOutcome::Unlocked)
    );
}

#[test]
fn armed_wipe_triggers_exactly_at_threshold_with_restored_marker() {
    let _g = env_lock();
    let cfg = fresh_test_env("wipe_armed");
    vault_init_with_passphrase(PASS).expect("init");
    wipe_after_failed_unlocks_arm(3).expect("arm");
    assert_eq!(wipe_after_failed_unlocks_limit(), Ok(Some(3)));
    drain_markers();

    assert_eq!(
        unlock_guarded_at(WRONG, T0),
        Ok(GuardedUnlockOutcome::Rejected {
            failed_unlocks: 1,
            retry_after_s: 0
        })
    );
    assert_eq!(
        unlock_guarded_at(WRONG, T0),
        Ok(GuardedUnlockOutcome::Rejected {
            failed_unlocks: 2,
            retry_after_s: 0
        })
    );
    assert!(
        cfg.join("vault.qsv").is_file(),
        "still present below threshold"
    );

    // Exactly at N the historical wipe path fires.
    assert_eq!(
        unlock_guarded_at(WRONG, T0),
        Ok(GuardedUnlockOutcome::Wiped {
            marker: QSC_ERR_VAULT_WIPED_AFTER_FAILED_UNLOCKS
        })
    );
    assert!(!cfg.join("vault.qsv").exists(), "vault gone at threshold");
    assert!(
        !cfg.join(CONFIG_FILE).exists() && !cfg.join(COUNTER_FILE).exists(),
        "both protection-state files cleared"
    );
    // The lock() postconditions hold after the wipe.
    assert!(!has_process_passphrase());
    assert!(!vault_unlocked());
    // The restored marker is observable via the in-app queue on the pre-existing
    // vault_unlock event (tui_unlock stays deleted).
    let lines = drain_markers();
    assert!(
        lines.iter().any(|l| l.contains("event=vault_unlock")
            && l.contains(QSC_ERR_VAULT_WIPED_AFTER_FAILED_UNLOCKS)
            && l.contains("reason=failed_unlock_limit_reached")),
        "restored wipe marker expected in the queue, got: {:?}",
        lines
    );
}

#[test]
fn arm_bounds_enforced_and_disarm_persists_off() {
    let _g = env_lock();
    let cfg = fresh_test_env("wipe_bounds");
    vault_init_with_passphrase(PASS).expect("init");

    // Historical bounds 1..=100 restored and enforced.
    assert_eq!(
        wipe_after_failed_unlocks_arm(VAULT_ATTEMPT_LIMIT_MIN - 1),
        Err("vault_attempt_limit_invalid")
    );
    assert_eq!(
        wipe_after_failed_unlocks_arm(VAULT_ATTEMPT_LIMIT_MAX + 1),
        Err("vault_attempt_limit_invalid")
    );
    wipe_after_failed_unlocks_arm(VAULT_ATTEMPT_LIMIT_MIN).expect("arm at min");
    wipe_after_failed_unlocks_arm(VAULT_ATTEMPT_LIMIT_MAX).expect("arm at max");
    assert!(
        read_file(&cfg.join(CONFIG_FILE)).contains("attempt_limit=100"),
        "armed limit persisted"
    );

    wipe_after_failed_unlocks_disarm().expect("disarm");
    assert_eq!(wipe_after_failed_unlocks_limit(), Ok(None));
    assert!(
        read_file(&cfg.join(CONFIG_FILE)).contains("attempt_limit=off"),
        "disarm persists off"
    );
    // Disarmed-safe: failures past the previously-armed limit no longer wipe.
    for _ in 0..2 {
        unlock_guarded_at(WRONG, T0).expect("guarded attempt");
    }
    assert!(cfg.join("vault.qsv").is_file(), "no wipe after disarm");
}

// ---------------------------------------------------------------------------
// Family 3 — lock() as ONE operation (the library half of idle autolock, R3)
// ---------------------------------------------------------------------------

#[test]
fn lock_is_one_operation_idempotent_and_relockable() {
    let _g = env_lock();
    fresh_test_env("lock_one_op");
    vault_init_with_passphrase(PASS).expect("init");

    // Unlocked state with a live session: passphrase set, flag true, session open.
    assert_eq!(unlock_guarded(PASS), Ok(GuardedUnlockOutcome::Unlocked));
    let session = open_session_with_passphrase(PASS).expect("session");
    assert!(has_process_passphrase());
    assert!(vault_unlocked());

    // ONE call: passphrase cleared + flag cleared + session disposed (Drop zeroize).
    lock(Some(session));
    assert!(
        !has_process_passphrase(),
        "lock clears the process passphrase"
    );
    assert!(!vault_unlocked(), "lock clears the unlocked flag");

    // Idempotent: a second call with nothing live is a no-op.
    lock(None);
    assert!(!has_process_passphrase());
    assert!(!vault_unlocked());

    // Safe mid-process re-lock: a subsequent unlock works through EITHER path.
    assert_eq!(unlock_guarded(PASS), Ok(GuardedUnlockOutcome::Unlocked));
    assert!(has_process_passphrase() && vault_unlocked());
    lock(None);
    unlock_with_passphrase(PASS).expect("existing unlock path still works");
    assert!(has_process_passphrase());
}

// ---------------------------------------------------------------------------
// Family 4 — destroy: deliberate, instant, token-confirmed
// ---------------------------------------------------------------------------

#[test]
fn destroy_refuses_wrong_passphrase_empty_passphrase_and_wrong_token() {
    let _g = env_lock();
    let cfg = fresh_test_env("destroy_refusals");
    vault_init_with_passphrase(PASS).expect("init");
    unlock_with_passphrase(PASS).expect("unlock");
    secret_set("na0658-k", "na0658-v").expect("seed secret");

    // Wrong passphrase (even with a token committed to it): refused, state unchanged.
    assert_eq!(
        destroy_with_passphrase(WRONG, DestroyConfirmToken::confirm_with_passphrase(WRONG)),
        Err("vault_locked")
    );
    // Empty passphrase: refused (the historical guard).
    assert_eq!(
        destroy_with_passphrase("", DestroyConfirmToken::confirm_with_passphrase("")),
        Err("vault_locked")
    );
    // Correct passphrase but a token committed to a DIFFERENT value: the wrong-VALUE
    // path — refused with NO destruction (the absent-token call does not compile).
    assert_eq!(
        destroy_with_passphrase(PASS, DestroyConfirmToken::confirm_with_passphrase(WRONG)),
        Err("vault_locked")
    );

    assert!(
        cfg.join("vault.qsv").is_file(),
        "vault intact after refusals"
    );
    unlock_with_passphrase(PASS).expect("vault still unlockable");
    assert_eq!(
        secret_get("na0658-k").expect("secret readable"),
        Some("na0658-v".to_string()),
        "protected data unchanged after refused destroys"
    );
}

#[test]
fn destroy_with_token_is_irreversible_and_leaves_locked() {
    let _g = env_lock();
    let cfg = fresh_test_env("destroy_final");
    vault_init_with_passphrase(PASS).expect("init");
    unlock_with_passphrase(PASS).expect("unlock");
    secret_set("na0658-k", "na0658-v").expect("seed secret");
    set_vault_unlocked(true);
    // Ensure protection-state files exist so their post-destroy clearing is proven.
    unlock_guarded_at(WRONG, T0).expect("guarded attempt seeds state files");
    assert!(cfg.join(COUNTER_FILE).is_file());

    destroy_with_passphrase(PASS, DestroyConfirmToken::confirm_with_passphrase(PASS))
        .expect("deliberate destroy with matching token");

    // The vault file is gone (erase-then-remove) and everything it protected is
    // permanently unreadable through the library.
    assert!(!cfg.join("vault.qsv").exists(), "vault file removed");
    assert_eq!(unlock_with_passphrase(PASS), Err("vault_missing"));
    assert!(secret_get("na0658-k").is_err(), "protected data unreadable");
    // Protection-state files cleared; the process is left locked.
    assert!(
        !cfg.join(CONFIG_FILE).exists() && !cfg.join(COUNTER_FILE).exists(),
        "protection-state files cleared by destroy"
    );
    assert!(
        !has_process_passphrase(),
        "destroy leaves no process passphrase"
    );
    assert!(
        !vault_unlocked(),
        "destroy leaves the unlocked flag cleared"
    );
    // Keychain removal (key_source == 2) is not exercisable headless
    // (QSC_DISABLE_KEYCHAIN=1 forces the passphrase source); classified honestly
    // in the lane testplan rather than faked here.
}
