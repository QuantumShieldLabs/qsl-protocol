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
// - C01 O7: a version or format refusal returns Err(code), uncounted; Rejected/Wiped only
//   for a passphrase-authentication failure
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

use super::{
    authenticate_with_passphrase, finish_ownership_unlock, set_process_passphrase, VaultSession,
};
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
use std::path::Path;
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

/// The second, distinct, visible arming step destroy requires: the token carries what the
/// caller's human actually TYPED, and is consumed by the destroy call. Tokenless destroy
/// calls do not compile. NA-0696 (D630 A1.3/R1, D-1336): the constructor takes the typed
/// word alone — what the commitment must equal branches on the key source at the destroy
/// site (passphrase vaults: the passphrase, real authentication, unchanged; keychain
/// vaults: the ceremony word, a deliberateness guard).
pub struct DestroyConfirmToken {
    commitment: String,
}

impl DestroyConfirmToken {
    pub fn confirm(typed: &str) -> DestroyConfirmToken {
        DestroyConfirmToken {
            commitment: typed.to_string(),
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
    // NA-0688 C1 (R4a): delegates to the ONE clock. See `crate::clock`.
    crate::clock::now_unix_s()
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
/// behavior is identical. Err(code) for a version or format refusal (uncounted);
/// Rejected/Wiped only for a passphrase-authentication failure (C01 O7).
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
    // NA-0785 F03 S8 (C01 O7): only a passphrase-authentication failure counts. Every
    // other refusal of the unlock path (version, format, KDF header, provider, the
    // post-AEAD payload checks, the empty passphrase) returns its own unchanged code
    // BEFORE the counter below: no counter write, no delay, no wipe.
    let attempt = authenticate_with_passphrase(passphrase);
    if let Err(refusal) = &attempt {
        if !refusal.is_passphrase_authentication_failure() {
            return Err(refusal.code());
        }
    }
    if let Ok(session) = attempt {
        finish_ownership_unlock(session)?;
        // Best-effort reset, the historical semantics: written only when there is
        // something to reset, and a persist failure must not undo the unlock.
        if state.failed_unlocks != 0 || state.last_failure_unix_s.is_some() {
            state.failed_unlocks = 0;
            state.last_failure_unix_s = None;
            // Un-swallowed (D-1333): loud, and the unlock stands — the documented
            // semantic above ("a persist failure must not undo the unlock") holds.
            if let Err(code) = protection_state_store(&state) {
                emit_marker(
                    "vault_unlock",
                    None,
                    &[("ok", "true"), ("counter_reset", code.as_str())],
                );
            }
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
            // Un-swallowed (D-1333): loud, and the Wiped outcome stands unchanged.
            if let Err(code) = protection_state_clear_files() {
                emit_marker(
                    "vault_unlock",
                    None,
                    &[("ok", "false"), ("protection_clear", code.as_str())],
                );
            }
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

/// NA-0696 (D630 A1.3/R1): the keychain-destroy ceremony word. A keychain-vault destroy
/// (key_source 2) requires the confirmation token's commitment to equal this literal —
/// a deliberateness guard, not authentication; see `destroy_with_passphrase`.
pub const VAULT_DESTROY_INTENT_PHRASE: &str = "DESTROY";

/// Deliberate, instant account destroy — the honest contract (NA-0696, D630 D4/D5a,
/// D-1336):
///
/// WHAT DESTROY GUARANTEES: the cryptographic erase. The runtime key is zeroized; for a
/// keychain vault (key_source 2) the OS-keychain entry is removed — everything keyed off
/// the vault becomes permanently undecryptable when the key dies. The zero pass below is
/// defense-in-depth on top of that, and it is now actually ordered: the vault file is
/// zeroed IN PLACE on its recorded inode (no truncate — `fs::write`'s O_TRUNC freed the
/// original blocks first, so the historical overwrite touched the old data on NO
/// filesystem), synced to disk, THEN unlinked, then the directory fsynced. That is
/// filesystem-level zeroization on non-CoW filesystems; it is explicitly NOT
/// physical-flash (FTL) or CoW-snapshot erasure. A passphrase vault's ultimate backstop
/// is passphrase strength plus full-disk encryption.
///
/// THE CEREMONY branches on the key source, peeked FIRST through the one parser: a
/// passphrase vault keeps the historical checks verbatim — commitment == passphrase is
/// real authentication. A keychain vault requires the commitment to equal the ceremony
/// word (the const above), refusing `vault_destroy_confirm_mismatch` with no
/// destruction — deliberateness, not authentication: the derivation ignores the
/// passphrase entirely, and same-machine security rests on the OS keychain and
/// idle-autolock, never on the destroy word.
///
/// THE BOUNDARY: vault-derived and vault-keyed artifacts die with the vault
/// (`vault.qsv`, the protection-state files, the keychain entry, process key material,
/// `send.state`, `msgqueue_v1/`, `quarantine_v1/`, `attachments/`); vault-independent
/// app configuration survives by design (`.qsc.lock` — held by destroy itself,
/// `config.txt`, `store.meta`). Post-destroy the process is left locked. Independent of,
/// and not armed by, the wipe-after-N opt-in.
pub fn destroy_with_passphrase(
    passphrase: &str,
    token: DestroyConfirmToken,
) -> Result<(), &'static str> {
    // The key-source peek: a light read through THE parser (D-1334's one owner);
    // read/parse failures map to the existing markers.
    let (cfg_dir, vault_path, source) = super::vault_path_resolved()?;
    // F04/S3b N1: through the one bounded vault reader (an oversized file refuses here, before
    // anything is destroyed).
    let peek_bytes = super::read_vault_file(&vault_path).map_err(|e| e.code("vault_missing"))?;
    let peeked_key_source =
        crate::adversarial::vault_format::parse_vault_envelope(&peek_bytes)?.key_source;
    if peeked_key_source == 2 {
        // Keychain vault: the ceremony word, checked before any other work.
        if token.commitment != VAULT_DESTROY_INTENT_PHRASE {
            return Err("vault_destroy_confirm_mismatch");
        }
    } else {
        // Passphrase vault: the historical checks, byte-for-byte (real authentication).
        if passphrase.is_empty() {
            return Err("vault_locked");
        }
        if token.commitment != passphrase {
            return Err("vault_locked");
        }
    }
    // NA-0693 (D627, D-1333): destroy is one locked transaction — resolved through the SAME
    // resolver as every vault write op, the exclusive store lock held across
    // validate → keychain-remove → erase → remove → satellite-clear → protection-clear.
    // NA-0696 (D630 D1(a), D-1336): the nested protection-clear below reaches the plain
    // entry and nests legally through the reentrant registry.
    let _lock = lock_store_exclusive(&cfg_dir, source).map_err(super::store_err_marker)?;
    let (_, mut runtime) = super::load_vault_runtime_with_passphrase(Some(passphrase))?;
    let _ = super::decrypt_payload(&runtime)?;
    let key_source = runtime.envelope.key_source;
    runtime.key.zeroize();

    if key_source == 2 {
        super::keychain_remove_key(&runtime.envelope.salt).map_err(|_| "vault_erase_failed")?;
    }

    // The erase, ordered (D630 D4): zero in place on the recorded inode, sync, unlink,
    // then the directory fsync ordering the unlink.
    if vault_path.exists() {
        zero_fill_in_place(&vault_path)?;
        fs::remove_file(&vault_path).map_err(|_| "vault_erase_failed")?;
        if let Some(parent) = vault_path.parent() {
            fsync_dir_best_effort(parent);
        }
    }
    // The destroy boundary (D630 A1.4/R2, D-1336): the vault-keyed satellite stores die
    // with the vault — their content is undecryptable once the key is gone, and
    // post-destroy ciphertext queues and send counters are seizure-relevant residue with
    // metadata value. Best-effort under the held lock, after the vault erase; absent
    // entries are fine. The consts are the one existing owner of each name — no
    // duplicated literals.
    let _ = fs::remove_file(cfg_dir.join(crate::SEND_STATE_NAME));
    let _ = fs::remove_dir_all(cfg_dir.join(crate::msgqueue::MSGQUEUE_DIR));
    let _ = fs::remove_dir_all(cfg_dir.join(crate::quarantine::QUARANTINE_DIR));
    let _ = fs::remove_dir_all(cfg_dir.join(crate::ATTACHMENT_STAGING_DIR));
    fsync_dir_best_effort(&cfg_dir);
    // Un-swallowed (D-1333): loud, NOT fatal — the vault file and any keychain entry are
    // already gone, so an Err here would misreport a completed destroy. The residue stays
    // observable through `wipe_after_failed_unlocks_limit()`.
    if let Err(code) = protection_state_clear_files() {
        emit_marker(
            "vault_destroy",
            None,
            &[("ok", "true"), ("protection_clear", code.as_str())],
        );
    }
    lock(None);
    Ok(())
}

/// NA-0696 (D630 D4, D-1336; ENG-0110): filesystem-level zeroization IN PLACE. Stat first
/// (inode and length recorded), open `write(true)` with NO truncate and NO create, then
/// the INODE-EQUALITY PIN: the opened fd must carry the inode the stat saw — on mismatch
/// the file was swapped underneath us, and zeroing the impostor (then unlinking the path)
/// would erase nothing, so refuse. Zeros land over `[0, len)` on the SAME inode and are
/// synced to disk before the caller unlinks.
fn zero_fill_in_place(path: &Path) -> Result<(), &'static str> {
    use std::os::unix::fs::MetadataExt;
    let md = fs::metadata(path).map_err(|_| "vault_erase_failed")?;
    let mut file = fs::OpenOptions::new()
        .write(true)
        .open(path)
        .map_err(|_| "vault_erase_failed")?;
    zero_fill_opened(md.ino(), md.len() as usize, &mut file)
}

/// The pin + the zero pass, on an already-opened fd (factored so the swapped-inode
/// refusal is deterministically constructible by the in-src unit — a true TOCTOU
/// interleave is not reachable from a single-threaded test).
fn zero_fill_opened(
    expected_ino: u64,
    len: usize,
    file: &mut fs::File,
) -> Result<(), &'static str> {
    use std::io::{Seek, SeekFrom, Write};
    use std::os::unix::fs::MetadataExt;
    let opened = file.metadata().map_err(|_| "vault_erase_failed")?;
    if opened.ino() != expected_ino {
        return Err("vault_erase_failed");
    }
    if len > 0 {
        let zeros = vec![0u8; len];
        file.seek(SeekFrom::Start(0))
            .map_err(|_| "vault_erase_failed")?;
        file.write_all(&zeros).map_err(|_| "vault_erase_failed")?;
    }
    file.sync_all().map_err(|_| "vault_erase_failed")
}

// The key set of each protection file (C01 A23: today's exact keys).
const CONFIG_KEYS: [&str; 1] = ["attempt_limit"];
const COUNTER_KEYS: [&str; 2] = ["failed_unlocks", "last_failure_unix_s"];

// NA-0788 F04/S2 (C01 O7): ONE strict scan of a protection file. A blank line or a `#` comment is
// skipped, as before; every other line must be `key=value` for a key of THIS file, and a key may
// appear once. An unknown line or a repeated key refuses the whole file -- the callers fail closed
// and count nothing -- where the historical scan took the first match and skipped the rest.
fn protection_lines<'a>(
    raw: &'a str,
    keys: &[&'static str],
) -> Result<std::collections::BTreeMap<&'static str, &'a str>, ErrorCode> {
    let mut lines = std::collections::BTreeMap::new();
    for line in raw.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        let (key, value) = keys
            .iter()
            .find_map(|key| {
                let value = trimmed.strip_prefix(key)?.strip_prefix('=')?;
                Some((*key, value))
            })
            .ok_or(ErrorCode::ParseFailed)?;
        if lines.insert(key, value).is_some() {
            return Err(ErrorCode::ParseFailed);
        }
    }
    Ok(lines)
}

// A present config file names its limit: an absent attempt_limit line is refused, not read as
// "off" (which would disarm the wipe). An absent FILE is still the unarmed default.
fn parse_vault_attempt_limit_config(raw: &str) -> Result<Option<u32>, ErrorCode> {
    let lines = protection_lines(raw, &CONFIG_KEYS)?;
    let value = lines
        .get("attempt_limit")
        .ok_or(ErrorCode::ParseFailed)?
        .trim();
    if value.eq_ignore_ascii_case("off") {
        return Ok(None);
    }
    let parsed = value.parse::<u32>().map_err(|_| ErrorCode::ParseFailed)?;
    if !(VAULT_ATTEMPT_LIMIT_MIN..=VAULT_ATTEMPT_LIMIT_MAX).contains(&parsed) {
        return Err(ErrorCode::ParseFailed);
    }
    Ok(Some(parsed))
}

// A present counter file carries its counter: an absent failed_unlocks line is refused, not read
// as 0. An absent FILE is still the never-failed default.
fn parse_vault_failed_unlocks(raw: &str) -> Result<u32, ErrorCode> {
    let lines = protection_lines(raw, &COUNTER_KEYS)?;
    let value = lines.get("failed_unlocks").ok_or(ErrorCode::ParseFailed)?;
    value
        .trim()
        .parse::<u32>()
        .map_err(|_| ErrorCode::ParseFailed)
}

// The Δ4 additive field: the counter file's last-failure timestamp line the delay
// computation needs. The historical line-scan parser skips lines it does not
// recognize, so this field is invisible to the historical format. It is the one OPTIONAL
// line: its writer omits it when there is no failure time, so absent = no delay window active.
fn parse_vault_last_failure_unix_s(raw: &str) -> Result<Option<u64>, ErrorCode> {
    let lines = protection_lines(raw, &COUNTER_KEYS)?;
    lines
        .get("last_failure_unix_s")
        .map(|value| {
            value
                .trim()
                .parse::<u64>()
                .map_err(|_| ErrorCode::ParseFailed)
        })
        .transpose()
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

// NA-0696 (D630 D1(a), D-1336): one entry, one lock — a caller already holding the store
// lock (destroy's transaction) nests legally through the reentrant registry, so the
// per-site inner variant is retired.
fn protection_state_clear_files() -> Result<(), ErrorCode> {
    let (dir, source) = config_dir()?;
    let _lock = lock_store_exclusive(&dir, source)?;
    ensure_store_layout(&dir, source)?;
    let config_path = dir.join(VAULT_SECURITY_CONFIG_NAME);
    let counter_path = dir.join(VAULT_UNLOCK_COUNTER_NAME);
    enforce_safe_parents(&config_path, source)?;
    enforce_safe_parents(&counter_path, source)?;
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

// NA-0696 (D630 §4c, D-1336): the inode-equality pin's unit — `zero_fill_*` is private
// to this module, so a same-file `#[cfg(test)] mod` is the only place that can drive it
// (the na0692/na0696 in-src precedent). No env, no config resolution: direct paths only.
// Does NOT satisfy goal-lint (path-based); the gate instrument is the `tests/` binary.
#[cfg(test)]
mod na0696_zero_fill_tests {
    use super::*;
    use std::os::unix::fs::MetadataExt;
    use std::path::PathBuf;

    fn unit_dir(tag: &str) -> PathBuf {
        let root = if let Ok(v) = std::env::var("QSC_TEST_ROOT") {
            PathBuf::from(v)
        } else if let Ok(v) = std::env::var("CARGO_TARGET_DIR") {
            PathBuf::from(v)
        } else {
            PathBuf::from("target")
        };
        let dir = root
            .join("qsc-test-tmp")
            .join("na0696-zero-fill")
            .join(format!("{}_{}", tag, std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    /// Green half: the zero pass lands on the SAME inode (no truncate, no recreate),
    /// same length, all zeros. Refusal half: a swapped inode — the stat saw file A, the
    /// opened fd carries file B — refuses `vault_erase_failed` and writes NOTHING (the
    /// deterministic construction of the swap the pin exists for; a true stat-to-open
    /// interleave is not reachable single-threaded, which is why the pin is factored
    /// onto the opened fd).
    #[test]
    fn zero_fill_refuses_swapped_inode() {
        let dir = unit_dir("swapped_inode");
        // Green half.
        let pattern = dir.join("pattern.bin");
        fs::write(&pattern, [0xA5u8; 4096]).unwrap();
        let ino_before = fs::metadata(&pattern).unwrap().ino();
        zero_fill_in_place(&pattern).expect("zero pass on an unswapped file succeeds");
        let md_after = fs::metadata(&pattern).unwrap();
        assert_eq!(md_after.ino(), ino_before, "zeroed IN PLACE — same inode");
        assert_eq!(md_after.len(), 4096, "same length");
        let bytes = fs::read(&pattern).unwrap();
        assert!(bytes.iter().all(|b| *b == 0), "every byte zeroed");
        // Refusal half: stat records file A's inode; the opened fd is file B.
        let file_a = dir.join("original.bin");
        let file_b = dir.join("impostor.bin");
        fs::write(&file_a, [0x5Au8; 128]).unwrap();
        fs::write(&file_b, [0xC3u8; 128]).unwrap();
        let recorded = fs::metadata(&file_a).unwrap();
        let mut opened_impostor = fs::OpenOptions::new().write(true).open(&file_b).unwrap();
        let refused = zero_fill_opened(
            recorded.ino(),
            recorded.len() as usize,
            &mut opened_impostor,
        );
        assert_eq!(
            refused,
            Err("vault_erase_failed"),
            "a swapped inode must refuse — zeroing the impostor would erase nothing"
        );
        let impostor_bytes = fs::read(&file_b).unwrap();
        assert!(
            impostor_bytes.iter().all(|b| *b == 0xC3),
            "the refusal must land BEFORE any write"
        );
    }
}

// NA-0788 F04/S2 (C01 O7): the protection files' strict readers. The parsers are the reader's
// parts; t_p_o7 drives the whole reader (protection_state_load) through the guarded unlock.
#[cfg(test)]
mod f04_s2_strict_tests {
    use super::*;

    fn refused<T: std::fmt::Debug>(r: Result<T, ErrorCode>) -> bool {
        matches!(r, Err(ErrorCode::ParseFailed))
    }

    #[test]
    fn t_p_writer_shapes_decode() {
        // Every shape protection_state_store writes.
        assert_eq!(
            parse_vault_attempt_limit_config("attempt_limit=5\n").ok(),
            Some(Some(5))
        );
        assert_eq!(
            parse_vault_attempt_limit_config("attempt_limit=off\n").ok(),
            Some(None)
        );
        let counter = "failed_unlocks=3\nlast_failure_unix_s=77\n";
        assert_eq!(parse_vault_failed_unlocks(counter).ok(), Some(3));
        assert_eq!(
            parse_vault_last_failure_unix_s(counter).ok(),
            Some(Some(77))
        );
        assert_eq!(
            parse_vault_failed_unlocks("failed_unlocks=0\n").ok(),
            Some(0)
        );
        assert_eq!(
            parse_vault_last_failure_unix_s("failed_unlocks=0\n").ok(),
            Some(None)
        );
        // The historical grammar is kept: blank lines, comments, surrounding whitespace.
        let spaced = "# note\n\n  failed_unlocks= 4 \n";
        assert_eq!(parse_vault_failed_unlocks(spaced).ok(), Some(4));
    }

    #[test]
    fn t_p_duplicate_key_refused() {
        assert!(
            parse_vault_attempt_limit_config("attempt_limit=5\n").is_ok(),
            "control"
        );
        for config in [
            "attempt_limit=5\nattempt_limit=7\n",
            "attempt_limit=5\nattempt_limit=5\n",
            "attempt_limit=off\nattempt_limit=5\n",
        ] {
            assert!(
                refused(parse_vault_attempt_limit_config(config)),
                "{config:?}"
            );
        }
        for counter in [
            "failed_unlocks=1\nfailed_unlocks=5\n",
            "failed_unlocks=1\nfailed_unlocks=1\n",
            "failed_unlocks=1\nlast_failure_unix_s=10\nlast_failure_unix_s=20\n",
        ] {
            assert!(refused(parse_vault_failed_unlocks(counter)), "{counter:?}");
            assert!(
                refused(parse_vault_last_failure_unix_s(counter)),
                "{counter:?}"
            );
        }
    }

    #[test]
    fn t_p_unknown_line_refused() {
        assert!(
            parse_vault_failed_unlocks("failed_unlocks=2\n").is_ok(),
            "control"
        );
        for config in [
            "attempt_limit=5\nbogus=1\n",
            "attempt_limit=5\nfailed_unlocks=3\n",
            "attempt_limit=5\ngarbage\n",
            "attempt_limit =5\n",
        ] {
            assert!(
                refused(parse_vault_attempt_limit_config(config)),
                "{config:?}"
            );
        }
        for counter in [
            "failed_unlocks=2\nbogus=1\n",
            "failed_unlocks=2\nattempt_limit=5\n",
            "failed_unlocks =9\nfailed_unlocks=2\n",
        ] {
            assert!(refused(parse_vault_failed_unlocks(counter)), "{counter:?}");
            assert!(
                refused(parse_vault_last_failure_unix_s(counter)),
                "{counter:?}"
            );
        }
    }

    #[test]
    fn t_p_missing_counter_refused() {
        assert_eq!(
            parse_vault_failed_unlocks("failed_unlocks=0\n").ok(),
            Some(0),
            "control"
        );
        for counter in ["last_failure_unix_s=10\n", "", "# only a comment\n"] {
            assert!(refused(parse_vault_failed_unlocks(counter)), "{counter:?}");
        }
    }

    #[test]
    fn t_p_missing_attempt_limit_refused() {
        assert!(
            parse_vault_attempt_limit_config("attempt_limit=off\n").is_ok(),
            "control"
        );
        for config in ["", "\n", "# only a comment\n"] {
            assert!(
                refused(parse_vault_attempt_limit_config(config)),
                "{config:?}"
            );
        }
    }

    // The whole reader through the guarded unlock, in an isolated child (it sets QSC_CONFIG_DIR).
    const CHILD: &str = "F04S2_PROTECTION_CHILD";

    #[test]
    fn t_p_o7_refusal_fails_closed_and_counts_nothing() {
        if std::env::var_os(CHILD).is_none() {
            let name = "vault::protection::f04_s2_strict_tests::\
                        t_p_o7_refusal_fails_closed_and_counts_nothing";
            let status = std::process::Command::new(std::env::current_exe().unwrap())
                .args(["--exact", name, "--nocapture"])
                .env(CHILD, "1")
                .status()
                .unwrap();
            assert!(status.success(), "isolated protection fixture failed");
            return;
        }
        use rand_core::{OsRng, RngCore};
        let dir = tempfile::tempdir().unwrap();
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(dir.path(), fs::Permissions::from_mode(0o700)).unwrap();
        }
        std::env::set_var("QSC_CONFIG_DIR", dir.path());
        let mut seed = [0u8; 16];
        OsRng.fill_bytes(&mut seed);
        let pass: String = seed.iter().map(|b| format!("{b:02x}")).collect();
        crate::vault::vault_init_directional_with_passphrase(&pass).unwrap();
        wipe_after_failed_unlocks_arm(50).unwrap();
        let config = dir.path().join(VAULT_SECURITY_CONFIG_NAME);
        let counter = dir.path().join(VAULT_UNLOCK_COUNTER_NAME);
        let vault = dir.path().join("vault.qsv");
        assert_eq!(fs::read_to_string(&config).unwrap(), "attempt_limit=50\n");
        let valid_counter = "failed_unlocks=2\n";
        fs::write(&counter, valid_counter).unwrap();
        let valid_config = fs::read(&config).unwrap();
        let now = 1_000_000;
        let cases: [(&str, &Path, &str); 6] = [
            (
                "duplicate counter",
                &counter,
                "failed_unlocks=2\nfailed_unlocks=0\n",
            ),
            (
                "unknown counter line",
                &counter,
                "failed_unlocks=2\nbogus=1\n",
            ),
            ("missing counter", &counter, "last_failure_unix_s=1\n"),
            (
                "duplicate limit",
                &config,
                "attempt_limit=50\nattempt_limit=1\n",
            ),
            (
                "unknown config line",
                &config,
                "attempt_limit=50\nbogus=1\n",
            ),
            ("missing limit", &config, "# armed?\n"),
        ];
        for (label, path, text) in cases {
            fs::write(path, text).unwrap();
            let files = || [&config, &counter, &vault].map(|p| fs::read(p).unwrap());
            let before = files();
            let status = protection_status_at(now).map(|s| s.failed_unlocks);
            assert_eq!(status, Err("vault_attempt_limit_io"), "{label}: status");
            for attempt in ["wrong-passphrase", pass.as_str()] {
                let outcome = unlock_guarded_at(attempt, now);
                assert_eq!(
                    outcome,
                    Err("vault_attempt_limit_io"),
                    "{label}: fail closed"
                );
            }
            assert_eq!(files(), before, "{label}: nothing written, nothing counted");
            fs::write(&config, &valid_config).unwrap();
            fs::write(&counter, valid_counter).unwrap();
            let status = protection_status_at(now).map(|s| (s.failed_unlocks, s.wipe_after));
            assert_eq!(
                status,
                Ok((2, Some(50))),
                "{label}: the count is the seeded one"
            );
        }
        // Control arm: the valid files count a wrong passphrase.
        let outcome = unlock_guarded_at("wrong-passphrase", now);
        let rejected = GuardedUnlockOutcome::Rejected {
            failed_unlocks: 3,
            retry_after_s: unlock_delay_schedule_s(3),
        };
        assert_eq!(outcome, Ok(rejected));
    }
}
