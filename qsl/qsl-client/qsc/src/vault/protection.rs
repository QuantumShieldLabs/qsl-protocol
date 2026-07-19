// NA-0658 (D594, D-1281): the ENG-0044 vault/account-protection surface restored as
// LIBRARY surface per the operator-approved 2026-07-17 refinement (DOC-PROG-004 step 4).
// The TUI-era machinery (deleted with the TUI at NA-0645/86c0858d; last present at
// 2efc9dab) returns re-homed from the CLI binary into this submodule so the GUI can
// call it; the CLI does not re-expose it and its unlock ingresses stay un-guarded.
//
// Invariants:
// - the guarded path is ALWAYS ON (fails safe): every wrong attempt through it counts
//   into a persisted counter; from the 3rd consecutive failure an escalating delay
//   (5 s doubling, capped at 300 s) refuses attempts without decrypting
// - a delay-window refusal never increments the counter; clock rollback never
//   shortens the wait
// - wipe-after-N is a SEPARATE explicit opt-in (absent config file = no wipe, ever)
// - lock() is one idempotent operation: process passphrase + unlocked flag + session
// - destroy requires a passphrase-committed confirmation token; no single plain call
//   can destroy by accident
// - typed pub results carry retry-after/attempt data as VALUES; the only marker
//   emitted is the restored QSC_ERR_VAULT_WIPED_AFTER_FAILED_UNLOCKS on the
//   pre-existing "vault_unlock" event when the opt-in wipe triggers
//
// Honest scope (the roadmap note stands): these protections defend the
// device-in-hand path THROUGH the app; an offline copy of the vault file is defended
// only by passphrase strength + Argon2id.

use super::{set_process_passphrase, unlock_with_passphrase, VaultSession};
use crate::fs_store::{
    config_dir, enforce_safe_parents, ensure_store_layout, fsync_dir_best_effort,
    lock_store_exclusive, lock_store_shared, write_atomic,
};
use crate::model::ErrorCode;
use crate::output::emit_marker;
use crate::store::{
    QSC_ERR_VAULT_WIPED_AFTER_FAILED_UNLOCKS, VAULT_ATTEMPT_LIMIT_MAX, VAULT_ATTEMPT_LIMIT_MIN,
    VAULT_SECURITY_CONFIG_NAME, VAULT_UNLOCK_COUNTER_NAME,
};
use std::fs;
use std::fs::File;
use std::io::Read;
use std::time::{SystemTime, UNIX_EPOCH};
use zeroize::Zeroize;

/// Outcome of one attempt through the guarded unlock path. Retry-after and attempt
/// counts travel as values; no new marker vocabulary exists for the delay state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GuardedUnlockOutcome {
    /// Correct passphrase: process passphrase set, unlocked flag set, counter reset.
    Unlocked,
    /// The attempt ran and was refused (wrong credentials); it was counted.
    /// `retry_after_s` is the delay now in force before the next attempt (0 while
    /// the failure count is still inside the free tier).
    Rejected {
        failed_unlocks: u32,
        retry_after_s: u64,
    },
    /// Refused INSIDE the delay window: nothing was decrypted and nothing was
    /// counted; `retry_after_s` is the remaining wait.
    Delayed {
        failed_unlocks: u32,
        retry_after_s: u64,
    },
    /// The explicit opt-in limit was reached: the vault file is gone, both
    /// protection-state files are cleared, and the process is left locked.
    Wiped { marker: &'static str },
}

/// Snapshot of the persisted protection state, for the GUI's Vault/Security pane.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VaultProtectionStatus {
    pub failed_unlocks: u32,
    pub wipe_after: Option<u32>,
    pub retry_after_s: u64,
}

/// The second, distinct, visible arming step destroy requires: the token commits to
/// the passphrase it was built for and is consumed by the destroy call. Tokenless
/// destroy calls do not compile; a token built for a different passphrase is refused
/// with no destruction.
pub struct DestroyConfirmToken {
    commitment: String,
}

impl DestroyConfirmToken {
    pub fn confirm_with_passphrase(passphrase: &str) -> DestroyConfirmToken {
        DestroyConfirmToken {
            commitment: passphrase.to_string(),
        }
    }
}

impl Drop for DestroyConfirmToken {
    fn drop(&mut self) {
        self.commitment.zeroize();
    }
}

struct VaultProtectionState {
    attempt_limit: Option<u32>,
    failed_unlocks: u32,
    last_failure_unix_s: Option<u64>,
}

/// The accepted escalation schedule: failures 1-2 free, then 5 s doubling per
/// consecutive failure, capped at 300 s. Monotonic non-decreasing.
pub fn unlock_delay_schedule_s(failed_unlocks: u32) -> u64 {
    if failed_unlocks < 3 {
        return 0;
    }
    let exp = (failed_unlocks - 3).min(6);
    (5u64 << exp).min(300)
}

fn now_unix_s() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

/// Remaining wait before the next attempt is allowed. A clock reading EARLIER than
/// the recorded last failure (rollback) treats the full current delay as unelapsed —
/// it never shortens the wait.
fn retry_after_at(state: &VaultProtectionState, now_unix_s: u64) -> u64 {
    let delay = unlock_delay_schedule_s(state.failed_unlocks);
    if delay == 0 {
        return 0;
    }
    let Some(last) = state.last_failure_unix_s else {
        return 0;
    };
    if now_unix_s < last {
        return delay;
    }
    last.saturating_add(delay).saturating_sub(now_unix_s)
}

pub fn unlock_guarded(passphrase: &str) -> Result<GuardedUnlockOutcome, &'static str> {
    unlock_guarded_at(passphrase, now_unix_s())
}

/// The guarded unlock with an explicit clock reading (unix seconds) — the
/// test-visible clock seam. `unlock_guarded` delegates here with the real clock;
/// behavior is identical.
pub fn unlock_guarded_at(
    passphrase: &str,
    now_unix_s: u64,
) -> Result<GuardedUnlockOutcome, &'static str> {
    // Fail closed: if the protection state cannot be read, refuse to attempt.
    let mut state = protection_state_load().map_err(|_| "vault_attempt_limit_io")?;
    let wait = retry_after_at(&state, now_unix_s);
    if wait > 0 {
        return Ok(GuardedUnlockOutcome::Delayed {
            failed_unlocks: state.failed_unlocks,
            retry_after_s: wait,
        });
    }
    if unlock_with_passphrase(passphrase).is_ok() {
        // Best-effort reset, the historical semantics: written only when there is
        // something to reset, and a persist failure must not undo the unlock.
        if state.failed_unlocks != 0 || state.last_failure_unix_s.is_some() {
            state.failed_unlocks = 0;
            state.last_failure_unix_s = None;
            let _ = protection_state_store(&state);
        }
        crate::set_vault_unlocked(true);
        return Ok(GuardedUnlockOutcome::Unlocked);
    }
    state.failed_unlocks = state.failed_unlocks.saturating_add(1);
    state.last_failure_unix_s = Some(now_unix_s);
    protection_state_store(&state).map_err(|_| "vault_attempt_limit_io")?;
    if let Some(limit) = state.attempt_limit {
        if state.failed_unlocks >= limit {
            wipe_vault_file_best_effort().map_err(|_| "vault_wipe_failed")?;
            let _ = protection_state_clear_files();
            lock(None);
            emit_marker(
                "vault_unlock",
                Some(QSC_ERR_VAULT_WIPED_AFTER_FAILED_UNLOCKS),
                &[("ok", "false"), ("reason", "failed_unlock_limit_reached")],
            );
            return Ok(GuardedUnlockOutcome::Wiped {
                marker: QSC_ERR_VAULT_WIPED_AFTER_FAILED_UNLOCKS,
            });
        }
    }
    Ok(GuardedUnlockOutcome::Rejected {
        failed_unlocks: state.failed_unlocks,
        retry_after_s: unlock_delay_schedule_s(state.failed_unlocks),
    })
}

pub fn protection_status() -> Result<VaultProtectionStatus, &'static str> {
    protection_status_at(now_unix_s())
}

pub fn protection_status_at(now_unix_s: u64) -> Result<VaultProtectionStatus, &'static str> {
    let state = protection_state_load().map_err(|_| "vault_attempt_limit_io")?;
    Ok(VaultProtectionStatus {
        failed_unlocks: state.failed_unlocks,
        wipe_after: state.attempt_limit,
        retry_after_s: retry_after_at(&state, now_unix_s),
    })
}

/// Arm the EXPLICIT OPT-IN wipe-after-N consequence (bounds 1..=100, the historical
/// bounds). Arming or disarming resets the failure counter (the historical
/// set_unlock_attempt_limit semantics).
pub fn wipe_after_failed_unlocks_arm(limit: u32) -> Result<(), &'static str> {
    if !(VAULT_ATTEMPT_LIMIT_MIN..=VAULT_ATTEMPT_LIMIT_MAX).contains(&limit) {
        return Err("vault_attempt_limit_invalid");
    }
    set_attempt_limit(Some(limit))
}

pub fn wipe_after_failed_unlocks_disarm() -> Result<(), &'static str> {
    set_attempt_limit(None)
}

pub fn wipe_after_failed_unlocks_limit() -> Result<Option<u32>, &'static str> {
    protection_state_load()
        .map(|state| state.attempt_limit)
        .map_err(|_| "vault_attempt_limit_io")
}

fn set_attempt_limit(limit: Option<u32>) -> Result<(), &'static str> {
    let mut state = protection_state_load().map_err(|_| "vault_attempt_limit_io")?;
    state.attempt_limit = limit;
    state.failed_unlocks = 0;
    state.last_failure_unix_s = None;
    protection_state_store(&state).map_err(|_| "vault_attempt_limit_io")
}

/// The library half of idle autolock (investigation residue R3): ONE call clears the
/// process passphrase, clears the unlocked flag, and disposes any live session the
/// caller hands over (VaultSession's Drop zeroizes key material). Idempotent; the
/// library is left in the pre-unlock state and a subsequent unlock through either
/// path works. The idle TIMER and its minutes setting are GUI-side (step 5).
pub fn lock(session: Option<VaultSession>) {
    set_process_passphrase(None);
    crate::set_vault_unlocked(false);
    drop(session);
}

/// Deliberate, instant account destroy: the historical machinery (validate by full
/// decrypt; wrong passphrase refused with state unchanged; runtime-key zeroize;
/// keychain removal when key_source == 2; zero-overwrite at recorded length THEN
/// remove THEN fsync — the erase-then-remove ordering) with the REFINED call shape:
/// the confirmation token is required and must match the passphrase. Post-destroy the
/// protection-state files are cleared and the process is left locked. Independent of,
/// and not armed by, the wipe-after-N opt-in.
pub fn destroy_with_passphrase(
    passphrase: &str,
    token: DestroyConfirmToken,
) -> Result<(), &'static str> {
    if passphrase.is_empty() {
        return Err("vault_locked");
    }
    if token.commitment != passphrase {
        return Err("vault_locked");
    }
    let (vault_path, mut runtime) = super::load_vault_runtime_with_passphrase(Some(passphrase))?;
    let _ = super::decrypt_payload(&runtime)?;
    let key_source = runtime.envelope.key_source;
    runtime.key.zeroize();

    if key_source == 2 {
        super::keychain_remove_key().map_err(|_| "vault_erase_failed")?;
    }

    // Best-effort cryptographic erase path: remove wrapped material and then delete file.
    if vault_path.exists() {
        let len = fs::metadata(&vault_path)
            .ok()
            .map(|md| md.len() as usize)
            .unwrap_or(0usize);
        if len > 0 {
            let zeros = vec![0u8; len];
            fs::write(&vault_path, zeros).map_err(|_| "vault_erase_failed")?;
        }
        fs::remove_file(&vault_path).map_err(|_| "vault_erase_failed")?;
        if let Some(parent) = vault_path.parent() {
            fsync_dir_best_effort(parent);
        }
    }
    let _ = protection_state_clear_files();
    lock(None);
    Ok(())
}

fn parse_vault_attempt_limit_config(raw: &str) -> Result<Option<u32>, ErrorCode> {
    for line in raw.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        let Some(value) = trimmed.strip_prefix("attempt_limit=") else {
            continue;
        };
        let value = value.trim();
        if value.eq_ignore_ascii_case("off") {
            return Ok(None);
        }
        let parsed = value.parse::<u32>().map_err(|_| ErrorCode::ParseFailed)?;
        if !(VAULT_ATTEMPT_LIMIT_MIN..=VAULT_ATTEMPT_LIMIT_MAX).contains(&parsed) {
            return Err(ErrorCode::ParseFailed);
        }
        return Ok(Some(parsed));
    }
    Ok(None)
}

fn parse_vault_failed_unlocks(raw: &str) -> Result<u32, ErrorCode> {
    for line in raw.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        let Some(value) = trimmed.strip_prefix("failed_unlocks=") else {
            continue;
        };
        return value
            .trim()
            .parse::<u32>()
            .map_err(|_| ErrorCode::ParseFailed);
    }
    Ok(0)
}

// The Δ4 additive field: the counter file's last-failure timestamp line the delay
// computation needs. The historical line-scan parser skips lines it does not
// recognize, so this field is invisible to the historical format and its own parse
// tolerates the absent field (absent = no delay window active).
fn parse_vault_last_failure_unix_s(raw: &str) -> Result<Option<u64>, ErrorCode> {
    for line in raw.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        let Some(value) = trimmed.strip_prefix("last_failure_unix_s=") else {
            continue;
        };
        return value
            .trim()
            .parse::<u64>()
            .map(Some)
            .map_err(|_| ErrorCode::ParseFailed);
    }
    Ok(None)
}

fn protection_state_load() -> Result<VaultProtectionState, ErrorCode> {
    let (dir, source) = config_dir()?;
    ensure_store_layout(&dir, source)?;
    let config_path = dir.join(VAULT_SECURITY_CONFIG_NAME);
    let counter_path = dir.join(VAULT_UNLOCK_COUNTER_NAME);
    enforce_safe_parents(&config_path, source)?;
    enforce_safe_parents(&counter_path, source)?;
    let _lock = lock_store_shared(&dir, source)?;

    let attempt_limit = if config_path.exists() {
        #[cfg(unix)]
        crate::fs_store::enforce_file_perms(&config_path)?;
        let mut raw = String::new();
        File::open(&config_path)
            .map_err(|_| ErrorCode::IoReadFailed)?
            .read_to_string(&mut raw)
            .map_err(|_| ErrorCode::IoReadFailed)?;
        parse_vault_attempt_limit_config(raw.as_str())?
    } else {
        None
    };

    let (failed_unlocks, last_failure_unix_s) = if counter_path.exists() {
        #[cfg(unix)]
        crate::fs_store::enforce_file_perms(&counter_path)?;
        let mut raw = String::new();
        File::open(&counter_path)
            .map_err(|_| ErrorCode::IoReadFailed)?
            .read_to_string(&mut raw)
            .map_err(|_| ErrorCode::IoReadFailed)?;
        (
            parse_vault_failed_unlocks(raw.as_str())?,
            parse_vault_last_failure_unix_s(raw.as_str())?,
        )
    } else {
        (0, None)
    };

    Ok(VaultProtectionState {
        attempt_limit,
        failed_unlocks,
        last_failure_unix_s,
    })
}

fn protection_state_store(state: &VaultProtectionState) -> Result<(), ErrorCode> {
    let (dir, source) = config_dir()?;
    ensure_store_layout(&dir, source)?;
    let config_path = dir.join(VAULT_SECURITY_CONFIG_NAME);
    let counter_path = dir.join(VAULT_UNLOCK_COUNTER_NAME);
    enforce_safe_parents(&config_path, source)?;
    enforce_safe_parents(&counter_path, source)?;
    let _lock = lock_store_exclusive(&dir, source)?;

    let config_content = match state.attempt_limit {
        Some(limit) => format!("attempt_limit={limit}\n"),
        None => "attempt_limit=off\n".to_string(),
    };
    let counter_content = match state.last_failure_unix_s {
        Some(ts) => format!(
            "failed_unlocks={}\nlast_failure_unix_s={}\n",
            state.failed_unlocks, ts
        ),
        None => format!("failed_unlocks={}\n", state.failed_unlocks),
    };
    write_atomic(&config_path, config_content.as_bytes(), source)?;
    write_atomic(&counter_path, counter_content.as_bytes(), source)?;
    Ok(())
}

fn protection_state_clear_files() -> Result<(), ErrorCode> {
    let (dir, source) = config_dir()?;
    ensure_store_layout(&dir, source)?;
    let config_path = dir.join(VAULT_SECURITY_CONFIG_NAME);
    let counter_path = dir.join(VAULT_UNLOCK_COUNTER_NAME);
    enforce_safe_parents(&config_path, source)?;
    enforce_safe_parents(&counter_path, source)?;
    let _lock = lock_store_exclusive(&dir, source)?;
    let _ = fs::remove_file(config_path);
    let _ = fs::remove_file(counter_path);
    fsync_dir_best_effort(&dir);
    Ok(())
}

fn wipe_vault_file_best_effort() -> Result<(), ErrorCode> {
    let (dir, source) = config_dir()?;
    ensure_store_layout(&dir, source)?;
    let vault_path = dir.join("vault.qsv");
    enforce_safe_parents(&vault_path, source)?;
    let _lock = lock_store_exclusive(&dir, source)?;
    if !vault_path.exists() {
        return Ok(());
    }
    let tombstone = dir.join(format!("vault.qsv.tombstone.{}", std::process::id()));
    if fs::rename(&vault_path, &tombstone).is_ok() {
        let _ = fs::remove_file(&tombstone);
    } else {
        let _ = fs::remove_file(&vault_path);
    }
    fsync_dir_best_effort(&dir);
    Ok(())
}
