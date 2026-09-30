// QSC vault: encrypted-at-rest secrets store (NA-0061 Phase 2).
//
// Invariants:
// - encrypted-at-rest default (no plaintext mode)
// - keychain preferred when available; deterministic passphrase fallback
// - noninteractive never prompts; fails closed with stable marker
// - no-mutation-on-reject for all storage boundaries touched
//
// This module intentionally prints only deterministic markers (no secrets).

#![allow(unexpected_cfgs)]

// NA-0658 (D594, D-1281): the ENG-0044 vault-protection surface restored as a library
// submodule — guarded unlock with escalating delay (default-on), wipe-after-N as an
// explicit opt-in, the one-call lock(), and token-confirmed destroy.
pub mod protection;

use crate::adversarial::vault_format::{classify_vault_magic, VaultMagicClass, VAULT_MAGIC};
// NA-0788 F04/S6: the successor classifier and the provider's seam, for the S6 block only.
use crate::adversarial::vault_format::classify_vault_magic_v4;
use crate::freshness::{LineageFields, OpenError};
use crate::fs_store::{lock_store_exclusive, write_atomic};
use crate::model::{ConfigSource, ErrorCode};
use crate::output::{CliError, CliResult};
use std::collections::BTreeMap;
use std::fs;
use std::io::{IsTerminal, Read, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Mutex, OnceLock};

use argon2::{Algorithm, Argon2, Params, Version};
use chacha20poly1305::aead::{Aead, AeadCore, KeyInit, Payload};
use chacha20poly1305::{ChaCha20Poly1305, Key, Nonce};
use clap::{Args, Subcommand};
#[cfg(feature = "keychain")]
use keyring::Entry;
use rand_core::{OsRng, RngCore};
use serde::{Deserialize, Serialize};
use zeroize::{Zeroize, Zeroizing};

// NA-0694 (D628, D-1334): the magic const's single owner is
// `crate::adversarial::vault_format` (imported above); the envelope header layout's single
// owner is `envelope_header_bytes` below. HEADER_LEN covers every pre-ciphertext byte:
// magic(6) + key_source(1) + salt_len(1) + nonce_len(1) + 3×KDF(4 LE) + ct_len(4 LE) +
// salt(16) + nonce(12).
const HEADER_LEN: usize = 53;
const KDF_M_KIB: u32 = 19456;
const KDF_T: u32 = 2;
const KDF_P: u32 = 1;
const RELAY_INBOX_TOKEN_SECRET_KEY: &str = "tui.relay.inbox_token";
const OWNER_FREE_PROFILE: &str = "NA0780-OWNER-FREE-01";
const PAYLOAD_VERSION: u8 = 4;

const DESKTOP_PASS_ENV_KEY: &str = "QSC_DESKTOP_SESSION_PASSPHRASE";

#[cfg(qsc_rng_failure_test_seam)]
fn vault_rng_failure_forced(label: &str) -> bool {
    std::env::var("QSC_RNG_FAILURE_TEST_SEAM")
        .ok()
        .map(|v| v == label || v == "all")
        .unwrap_or(false)
}

#[cfg(qsc_rng_failure_test_seam)]
fn vault_rng_fill(label: &str, out: &mut [u8]) -> Result<(), &'static str> {
    if vault_rng_failure_forced(label) {
        return Err("rng_failure_forced");
    }
    OsRng.fill_bytes(out);
    Ok(())
}

#[cfg(qsc_rng_failure_test_seam)]
fn vault_rng_nonce(label: &str) -> Result<Nonce, &'static str> {
    if vault_rng_failure_forced(label) {
        return Err("rng_failure_forced");
    }
    Ok(ChaCha20Poly1305::generate_nonce(&mut OsRng))
}

#[cfg(feature = "keychain")]
const VAULT_KEYCHAIN_SERVICE: &str = "qsc";
// NA-0695 (D629 R5, D-1335): probe-only, DELIBERATELY fixed — `keychain_supported`'s
// availability probe is constructor-only and never addresses the store, and pinning its
// account fixed keeps ENG-0116's availability-semantics surface untouched. Store entries
// are addressed per-vault via `vault_keychain_account` (R1); no fixed account reaches the
// store anywhere.
#[cfg(feature = "keychain")]
const VAULT_KEYCHAIN_PROBE_ACCOUNT: &str = "qsc-availability-probe";

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct VaultPayload {
    version: u8,
    protocol: String,
    #[serde(deserialize_with = "unique_secret_map")]
    secrets: BTreeMap<String, String>,
}

// A JSON map with repeated keys is contradictory authenticated state, not a
// last-value-wins update. This only parses; it never repairs or rewrites input.
fn unique_secret_map<'de, D>(deserializer: D) -> Result<BTreeMap<String, String>, D::Error>
where D: serde::Deserializer<'de> {
    struct Unique;
    impl<'de> serde::de::Visitor<'de> for Unique {
        type Value = BTreeMap<String, String>;
        fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.write_str("a secret map with unique keys")
        }
        fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
        where A: serde::de::MapAccess<'de> {
            let mut entries = BTreeMap::new();
            while let Some((key,value)) = map.next_entry::<String,String>()? {
                if entries.insert(key,value).is_some() {
                    return Err(serde::de::Error::custom("duplicate secret key"));
                }
            }
            Ok(entries)
        }
    }
    deserializer.deserialize_map(Unique)
}

impl VaultPayload {
    fn empty(directional: bool) -> Result<Self, &'static str> {
        let mut payload = Self {
            version: PAYLOAD_VERSION,
            protocol: if directional {
                String::from_utf8(crate::directional_delivery::INTEGRATION_PROFILE.to_vec())
                    .map_err(|_| "vault_version_unsupported")?
            } else { OWNER_FREE_PROFILE.to_owned() },
            secrets: BTreeMap::new(),
        };
        if directional {
            let layout = crate::protocol_state::approved_directional_layout()?;
            let owner = crate::protocol_state::CapacityOwner {
                generation: 0, peers: BTreeMap::new(), entries: BTreeMap::new(),
            };
            payload.secrets.insert(layout.owner_key.to_owned(),
                serde_json::to_string(&owner).map_err(|_| "directional_owner_encode")?);
        }
        Ok(payload)
    }
}

#[derive(Debug, Subcommand)]
pub enum VaultCmd {
    /// Initialize vault (creates encrypted envelope)
    Init(VaultInitArgs),
    /// Report vault status (no secrets; deterministic markers)
    Status,
    /// Validate local unlock credentials (no mutation).
    Unlock(VaultUnlockArgs),
}

#[derive(Debug, Args)]
pub struct VaultInitArgs {
    /// Fresh development selection: directional-v1 or storage-only owner-free-v1.
    #[arg(long, value_name = "PROTOCOL")]
    protocol: Option<String>,
    /// Noninteractive mode never prompts; fails closed if passphrase not provided.
    #[arg(long)]
    non_interactive: bool,

    /// Retired secret ingress; use --passphrase-file or --passphrase-stdin.
    #[arg(long, value_name = "ENV", hide = true)]
    passphrase_env: Option<String>,

    /// Read passphrase from a file path (contents are passphrase; trailing newline trimmed).
    #[arg(long, value_name = "PATH")]
    passphrase_file: Option<std::path::PathBuf>,

    /// Retired secret ingress; use --passphrase-file or --passphrase-stdin.
    #[arg(long, value_name = "PASS", hide = true)]
    passphrase: Option<String>,

    /// Read passphrase from stdin (explicit; never prompts).
    #[arg(long)]
    passphrase_stdin: bool,

    /// Explicit key source selection: passphrase | keychain | yubikey.
    #[arg(long, value_name = "SRC")]
    key_source: Option<String>,
}

#[derive(Debug, Args)]
pub struct VaultUnlockArgs {
    /// Noninteractive mode never prompts; fails closed if passphrase not provided.
    #[arg(long)]
    non_interactive: bool,

    /// Read passphrase from a file path (contents are passphrase; trailing newline trimmed).
    #[arg(long, value_name = "PATH")]
    passphrase_file: Option<std::path::PathBuf>,

    /// Read passphrase from stdin (explicit; never prompts).
    #[arg(long)]
    passphrase_stdin: bool,

    /// Desktop bridge compatibility only; operators should use --passphrase-file.
    #[arg(long, value_name = "ENV", hide = true)]
    passphrase_env: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum KeySource {
    Keychain,
    Passphrase,
    YubiKeyStub,
}

#[derive(Debug)]
#[allow(dead_code)]
enum ProviderError {
    YubiKeyNotImplemented,
    TokenMissing,
    TokenUnavailable,
    ProviderFailed,
    // NA-0695 (D629 R4, D-1335): init REFUSES an existing keychain entry rather than
    // overwriting it; a new cause gets its own name (D-1333 mapping discipline).
    EntryExists,
}

pub fn cmd_vault(cmd: VaultCmd) -> CliResult {
    match cmd {
        VaultCmd::Init(args) => vault_init(args),
        VaultCmd::Status => vault_status(),
        VaultCmd::Unlock(args) => vault_unlock(args),
    }
}

pub fn unlock_with_passphrase_env(passphrase_env: Option<&str>) -> Result<(), &'static str> {
    if let Some(env_name) = passphrase_env {
        let mut pass = passphrase_from_allowed_env(env_name)?;
        let out = unlock_with_passphrase(pass.as_str());
        pass.zeroize();
        return out;
    }

    finish_ownership_unlock(open_session(None)?)
}

pub fn unlock_with_passphrase_file(path: &Path) -> Result<(), &'static str> {
    let mut pass = read_passphrase_file(path)?;
    let out = unlock_with_passphrase(pass.as_str());
    pass.zeroize();
    out
}

pub fn unlock_with_passphrase(passphrase: &str) -> Result<(), &'static str> {
    finish_ownership_unlock(authenticate_with_passphrase(passphrase)?)
}

// Keep post-authentication storage failures out of the failed-password counter.
fn finish_ownership_unlock(mut session: VaultSession) -> Result<(), &'static str> {
    if retain_ownership_in_session(&mut session, None).is_err() {
        protection::lock(None);
        return Err(crate::invite::INVITE_OWNERSHIP_UNAVAILABLE);
    }
    Ok(())
}

// NA-0785 F03 S8 (C01 O7): an unlock-path refusal, typed where its cause is known. `code`
// is the unchanged marker every caller sees; the string alone cannot carry the class
// ("vault_locked" is overloaded). `aead_key_source` is set at ONE site, the AEAD tag
// check in `decrypt_payload_typed`, to the key source whose key failed it; every other
// refusal converts through `From` with None.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct UnlockRefusal {
    code: &'static str,
    aead_key_source: Option<u8>,
}

impl UnlockRefusal {
    /// The one class the attempt guard counts: the AEAD tag failed under a key derived
    /// from the passphrase (key_source 1). Guard callers always supply the passphrase.
    fn is_passphrase_authentication_failure(&self) -> bool {
        self.aead_key_source == Some(1)
    }

    fn code(&self) -> &'static str {
        self.code
    }
}

impl From<&'static str> for UnlockRefusal {
    fn from(code: &'static str) -> Self {
        UnlockRefusal {
            code,
            aead_key_source: None,
        }
    }
}

impl From<UnlockRefusal> for &'static str {
    fn from(refusal: UnlockRefusal) -> Self {
        refusal.code
    }
}

fn authenticate_with_passphrase(passphrase: &str) -> Result<VaultSession, UnlockRefusal> {
    if passphrase.is_empty() {
        return Err("vault_locked".into());
    }
    let session = open_session_typed(Some(passphrase))?;
    set_process_passphrase(Some(passphrase));
    Ok(session)
}

pub(crate) fn retain_invitation_ownership(
    mint: Option<([u8; 16], [u8; 32])>,
) -> Result<(), &'static str> {
    retain_ownership_in_session(&mut open_session(None)?, mint)
}

fn retain_ownership_in_session(
    session: &mut VaultSession,
    mint: Option<([u8; 16], [u8; 32])>,
) -> Result<(), &'static str> {
    let (dir, source) = crate::fs_store::config_dir().map_err(store_err_marker)?;
    let _lock = lock_store_exclusive(&dir, source).map_err(store_err_marker)?;
    // Re-read under the lock using the already authenticated key, not another KDF.
    let bytes = read_vault_file(&session.vault_path).map_err(|e| e.code("vault_missing"))?;
    session.payload = decrypt_payload(&VaultRuntime {
        envelope: parse_envelope(&bytes)?,
        key: session.key,
    })?;
    let update = crate::invite::updated_ownership(
        session
            .payload
            .secrets
            .get(crate::invite::OWNERSHIP_SECRET_KEY)
            .map(String::as_str),
        session
            .payload
            .secrets
            .get(crate::store::INVITES_SECRET_KEY)
            .map(String::as_str),
        mint,
    )?;
    if let Some(history) = update {
        persist_session_with_ownership(session, Some(history))?;
    }
    Ok(())
}

/// NA-0649 (D585 B1): in-process vault creation for the GUI — the passphrase arrives
/// in memory (no argv/env/file/stdin/terminal ingress on this path; the NA-0216B
/// retired-ingress decisions are untouched) and behavior matches a successful
/// `vault init --passphrase-file`: same envelope, same default inbox route-token
/// seeding, same `vault_init` success marker, same error codes returned as values
/// (`vault_exists`, …). No process unlock-state side effect — init and unlock stay
/// orthogonal; the caller decides whether to unlock after init.
pub fn vault_init_with_passphrase(_passphrase: &str) -> Result<(), &'static str> {
    Err("directional_profile_required")
}

/// Explicit opt-in for a fresh first-release development vault.
pub fn vault_init_directional_with_passphrase(passphrase: &str) -> Result<(), &'static str> {
    if passphrase.is_empty() {
        return Err("vault_passphrase_required");
    }
    vault_init_core(KeySource::Passphrase, Some(passphrase.to_string()), true)
}

pub fn secret_get(name: &str) -> Result<Option<String>, &'static str> {
    if name.is_empty() {
        return Err("vault_secret_name_invalid");
    }
    let (_vault_path, env) = load_vault_runtime()?;
    let payload = decrypt_payload(&env)?;
    let out = payload.secrets.get(name).cloned();
    Ok(out)
}

pub fn secret_set(name: &str, value: &str) -> Result<(), &'static str> {
    if name.is_empty() {
        return Err("vault_secret_name_invalid");
    }
    // NA-0693 (D627, D-1333): the exclusive store lock spans the WHOLE read-modify-write
    // (load → decrypt → mutate → encrypt → write), never the write alone. NA-0696 (D630
    // D1, D-1336): a caller already inside a locked transaction (the transport send paths)
    // nests legally through the reentrant registry — the per-site inner variant is retired.
    let (cfg_dir, _, source) = vault_path_resolved()?;
    let _lock = lock_store_exclusive(&cfg_dir, source).map_err(store_err_marker)?;
    let (vault_path, mut env) = load_vault_runtime()?;
    let mut payload = decrypt_payload(&env)?;
    guard_directional_generic_write(&payload, name)?;
    payload.secrets.insert(name.to_string(), value.to_string());
    check_directional_aggregate(&payload)?;
    let plaintext = serde_json::to_vec(&payload).map_err(|_| "vault_payload_serialize_failed")?;
    let cipher = ChaCha20Poly1305::new(Key::from_slice(&env.key));
    #[cfg(qsc_rng_failure_test_seam)]
    let nonce = vault_rng_nonce("QSC.VAULT.SECRET_SET.NONCE")?;
    #[cfg(not(qsc_rng_failure_test_seam))]
    let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);
    // NA-0694 (D628 §5.2, ENG-0107): AAD = the exact 53-byte header the serializer writes
    // below — ct_len is plaintext + the 16-byte Poly1305 tag, known before the cipher call.
    let aad = envelope_header_bytes(
        env.envelope.key_source,
        env.envelope.kdf_m_kib,
        env.envelope.kdf_t,
        env.envelope.kdf_p,
        envelope_ct_len(plaintext.len())?,
        &env.envelope.salt,
        nonce.as_slice().try_into().map_err(|_| "encrypt_failed")?,
    );
    let ciphertext = cipher
        .encrypt(
            &nonce,
            Payload {
                msg: plaintext.as_ref(),
                aad: &aad,
            },
        )
        .map_err(|_| "encrypt_failed")?;
    debug_assert_eq!(ciphertext.len(), plaintext.len() + 16);
    let bytes = encode_envelope(&env, nonce.as_slice(), &ciphertext);
    PERF_VAULT_ENCRYPT_WRITES.fetch_add(1, Ordering::Relaxed);
    write_atomic(&vault_path, &bytes, source).map_err(store_err_marker)?;
    VAULT_WRITE_EPOCH.fetch_add(1, Ordering::Relaxed);
    env.key.zeroize();
    Ok(())
}

// D581 KEEP -> NA-0646 (D582): part of the library's pub GUI surface, seeded for the GUI
// phase; dormant until the GUI consumes it (dead_code allowance retained meanwhile).
#[allow(dead_code)]
pub fn secret_set_with_passphrase(
    name: &str,
    value: &str,
    passphrase: &str,
) -> Result<(), &'static str> {
    if name.is_empty() {
        return Err("vault_secret_name_invalid");
    }
    if passphrase.is_empty() {
        return Err("vault_locked");
    }
    // NA-0693 (D627, D-1333): same locked read-modify-write transaction as `secret_set`.
    let (cfg_dir, _, source) = vault_path_resolved()?;
    let _lock = lock_store_exclusive(&cfg_dir, source).map_err(store_err_marker)?;
    let (vault_path, mut env) = load_vault_runtime_with_passphrase(Some(passphrase))?;
    let mut payload = decrypt_payload(&env)?;
    guard_directional_generic_write(&payload, name)?;
    payload.secrets.insert(name.to_string(), value.to_string());
    check_directional_aggregate(&payload)?;
    let plaintext = serde_json::to_vec(&payload).map_err(|_| "vault_payload_serialize_failed")?;
    let cipher = ChaCha20Poly1305::new(Key::from_slice(&env.key));
    #[cfg(qsc_rng_failure_test_seam)]
    let nonce = vault_rng_nonce("QSC.VAULT.SECRET_SET_WITH_PASSPHRASE.NONCE")?;
    #[cfg(not(qsc_rng_failure_test_seam))]
    let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);
    let aad = envelope_header_bytes(
        env.envelope.key_source,
        env.envelope.kdf_m_kib,
        env.envelope.kdf_t,
        env.envelope.kdf_p,
        envelope_ct_len(plaintext.len())?,
        &env.envelope.salt,
        nonce.as_slice().try_into().map_err(|_| "encrypt_failed")?,
    );
    let ciphertext = cipher
        .encrypt(
            &nonce,
            Payload {
                msg: plaintext.as_ref(),
                aad: &aad,
            },
        )
        .map_err(|_| "encrypt_failed")?;
    debug_assert_eq!(ciphertext.len(), plaintext.len() + 16);
    let bytes = encode_envelope(&env, nonce.as_slice(), &ciphertext);
    PERF_VAULT_ENCRYPT_WRITES.fetch_add(1, Ordering::Relaxed);
    write_atomic(&vault_path, &bytes, source).map_err(store_err_marker)?;
    VAULT_WRITE_EPOCH.fetch_add(1, Ordering::Relaxed);
    env.key.zeroize();
    Ok(())
}

// D581 KEEP -> NA-0646 (D582): part of the library's pub GUI surface, seeded for the GUI
// phase; dormant until the GUI consumes it (dead_code allowance retained meanwhile).
#[allow(dead_code)]
pub fn open_session(passphrase_override: Option<&str>) -> Result<VaultSession, &'static str> {
    open_session_typed(passphrase_override).map_err(|refusal| refusal.code())
}

fn open_session_typed(passphrase_override: Option<&str>) -> Result<VaultSession, UnlockRefusal> {
    let (vault_path, runtime) = load_vault_runtime_with_passphrase(passphrase_override)?;
    let payload = decrypt_payload_typed(&runtime)?;
    Ok(VaultSession {
        vault_path,
        envelope: runtime.envelope,
        key: runtime.key,
        payload,
        write_epoch_seen: VAULT_WRITE_EPOCH.load(Ordering::Relaxed),
    })
}

// D581 KEEP -> NA-0646 (D582): part of the library's pub GUI surface, seeded for the GUI
// phase; dormant until the GUI consumes it (dead_code allowance retained meanwhile).
#[allow(dead_code)]
pub fn open_session_with_passphrase(passphrase: &str) -> Result<VaultSession, &'static str> {
    if passphrase.is_empty() {
        return Err("vault_locked");
    }
    open_session(Some(passphrase))
}

// D581 KEEP -> NA-0646 (D582): part of the library's pub GUI surface, seeded for the GUI
// phase; dormant until the GUI consumes it (dead_code allowance retained meanwhile).
#[allow(dead_code)]
pub fn session_get(session: &VaultSession, name: &str) -> Result<Option<String>, &'static str> {
    if name.is_empty() {
        return Err("vault_secret_name_invalid");
    }
    Ok(session.payload.secrets.get(name).cloned())
}

// D581 KEEP -> NA-0646 (D582): part of the library's pub GUI surface, seeded for the GUI
// phase; dormant until the GUI consumes it (dead_code allowance retained meanwhile).
#[allow(dead_code)]
pub fn session_set(
    session: &mut VaultSession,
    name: &str,
    value: &str,
) -> Result<(), &'static str> {
    if name.is_empty() {
        return Err("vault_secret_name_invalid");
    }
    // Named save never overlays a stale session map. Refresh only after success.
    let (dir, source)=crate::fs_store::config_dir().map_err(store_err_marker)?;
    let _lock=lock_store_exclusive(&dir,source).map_err(store_err_marker)?;
    let env=read_session_runtime(session)?;
    let mut latest=decrypt_payload(&env)?;
    guard_directional_generic_write(&latest,name)?;
    latest.secrets.insert(name.to_owned(),value.to_owned());
    check_directional_aggregate(&latest)?;
    write_directional_payload(&session.vault_path,source,&env,&latest)?;
    session.payload=latest;
    session.envelope=env.envelope.clone();
    session.write_epoch_seen=VAULT_WRITE_EPOCH.load(Ordering::Relaxed);
    Ok(())
}

// D581 KEEP -> NA-0646 (D582): part of the library's pub GUI surface, seeded for the GUI
// phase; dormant until the GUI consumes it (dead_code allowance retained meanwhile).
#[allow(dead_code)]
pub fn perf_snapshot() -> (u64, u64, u64, u64) {
    (
        PERF_KDF_CALLS.load(Ordering::Relaxed),
        PERF_VAULT_FILE_READS.load(Ordering::Relaxed),
        PERF_VAULT_DECRYPTS.load(Ordering::Relaxed),
        PERF_VAULT_ENCRYPT_WRITES.load(Ordering::Relaxed),
    )
}

// D581 KEEP -> NA-0646 (D582): part of the library's pub GUI surface, seeded for the GUI
// phase; dormant until the GUI consumes it (dead_code allowance retained meanwhile).
#[allow(dead_code)]
pub fn persist_session(session: &mut VaultSession) -> Result<(), &'static str> {
    persist_session_with_ownership(session, None)
}

// Only the locked ownership transaction may supply an updated ownership record.
fn persist_session_with_ownership(
    session: &mut VaultSession,
    ownership_update: Option<String>,
) -> Result<(), &'static str> {
    let (dir, source) = crate::fs_store::config_dir().map_err(store_err_marker)?;
    let _lock = lock_store_exclusive(&dir, source).map_err(store_err_marker)?;
    // Even another process may have appended history since this session opened.
    // Preserve the authoritative encrypted record, never the session's stale copy.
    let bytes = read_vault_file(&session.vault_path).map_err(|e| e.code("vault_missing"))?;
    let latest = decrypt_payload(&VaultRuntime {
        envelope: parse_envelope(&bytes)?,
        key: session.key,
    })?;
    let layout=crate::protocol_state::approved_directional_layout()?;
    if latest.secrets.contains_key(layout.owner_key) {
        // Only the already-authorized ownership append is a typed exception.
        // Discard no live record and never overlay a whole caller snapshot.
        let Some(history)=ownership_update else {
            return Err("directional_untyped_snapshot_refused");
        };
        let mut named=latest;
        named.secrets.insert(crate::invite::OWNERSHIP_SECRET_KEY.to_owned(),history);
        check_directional_aggregate(&named)?;
        let env=read_session_runtime(session)?;
        write_directional_payload(&session.vault_path,source,&env,&named)?;
        session.payload=named;
        session.envelope=env.envelope.clone();
        session.write_epoch_seen=VAULT_WRITE_EPOCH.load(Ordering::Relaxed);
        return Ok(());
    }
    if session.payload.version != latest.version || session.payload.protocol != latest.protocol {
        return Err("vault_version_unsupported");
    }
    check_directional_aggregate(&session.payload)?;
    let ownership = latest
        .secrets
        .get(crate::invite::OWNERSHIP_SECRET_KEY)
        .cloned();
    let write_epoch = VAULT_WRITE_EPOCH.load(Ordering::Relaxed);
    if write_epoch != session.write_epoch_seen {
        let mut latest = latest;
        for (key, value) in session.payload.secrets.iter() {
            latest.secrets.insert(key.clone(), value.clone());
        }
        session.payload = latest;
    }
    session
        .payload
        .secrets
        .remove(crate::invite::OWNERSHIP_SECRET_KEY);
    if let Some(history) = ownership_update.or(ownership) {
        session
            .payload
            .secrets
            .insert(crate::invite::OWNERSHIP_SECRET_KEY.to_string(), history);
    }
    check_directional_aggregate(&session.payload)?;
    let plaintext =
        serde_json::to_vec(&session.payload).map_err(|_| "vault_payload_serialize_failed")?;
    let cipher = ChaCha20Poly1305::new(Key::from_slice(&session.key));
    #[cfg(qsc_rng_failure_test_seam)]
    let nonce = vault_rng_nonce("QSC.VAULT.SESSION_PERSIST.NONCE")?;
    #[cfg(not(qsc_rng_failure_test_seam))]
    let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);
    let aad = envelope_header_bytes(
        session.envelope.key_source,
        session.envelope.kdf_m_kib,
        session.envelope.kdf_t,
        session.envelope.kdf_p,
        envelope_ct_len(plaintext.len())?,
        &session.envelope.salt,
        nonce.as_slice().try_into().map_err(|_| "encrypt_failed")?,
    );
    let ciphertext = cipher
        .encrypt(
            &nonce,
            Payload {
                msg: plaintext.as_ref(),
                aad: &aad,
            },
        )
        .map_err(|_| "encrypt_failed")?;
    debug_assert_eq!(ciphertext.len(), plaintext.len() + 16);
    let bytes = encode_envelope(
        &VaultRuntime {
            envelope: session.envelope.clone(),
            key: session.key,
        },
        nonce.as_slice(),
        &ciphertext,
    );
    // NA-0693 (D627 §3.2): MECHANICAL redirect only, forced by the duplicate-writer
    // deletion. NA-0780 now locks and preserves authoritative ownership history;
    // unrelated secrets retain the existing merge. The refuse-not-merge
    // semantic for the epoch mismatch above is DECIDED and its code rides the Slice-4
    // GUI-wiring lane, which consumes `VAULT_WRITE_EPOCH` (the reason the epoch is kept).
    let (_, _, source) = vault_path_resolved()?;
    PERF_VAULT_ENCRYPT_WRITES.fetch_add(1, Ordering::Relaxed);
    write_atomic(&session.vault_path, &bytes, source).map_err(store_err_marker)?;
    session.write_epoch_seen = VAULT_WRITE_EPOCH.fetch_add(1, Ordering::Relaxed) + 1;
    Ok(())
}

fn vault_init(args: VaultInitArgs) -> CliResult {
    let directional = match args.protocol.as_deref() {
        Some("directional-v1") => true,
        Some("owner-free-v1") => false,
        _ => return Err(CliError::code("directional_profile_required")),
    };
    let noninteractive = args.non_interactive
        || std::env::var("QSC_NONINTERACTIVE").ok().as_deref() == Some("1")
        || !std::io::stdin().is_terminal();

    let mut args = args;
    let mut pass = match resolve_passphrase(&mut args) {
        Ok(pass) => pass,
        Err(code) => return Err(CliError::code(code)),
    };
    let pass_present = pass.as_ref().map(|p| !p.is_empty()).unwrap_or(false);

    let explicit_key_source = key_source_explicit(&args);
    let mut key_source = match resolve_key_source(&args) {
        Ok(src) => src,
        Err(code) => return Err(fail_with_marker_pass(code, &mut pass)),
    };

    if key_source == KeySource::Keychain && !keychain_supported() {
        if explicit_key_source {
            return Err(handle_provider_error_with_pass(ProviderError::TokenUnavailable, &mut pass));
        } else if pass_present {
            // Deterministic passphrase fallback when keychain is unavailable.
            key_source = KeySource::Passphrase;
        } else if noninteractive {
            return Err(fail_with_marker_pass("vault_passphrase_required_noninteractive", &mut pass));
        } else {
            return Err(fail_with_marker_pass("vault_passphrase_required", &mut pass));
        }
    }

    if key_source == KeySource::Passphrase && !pass_present {
        if noninteractive {
            return Err(fail_with_marker_pass("vault_passphrase_required_noninteractive", &mut pass));
        } else {
            return Err(fail_with_marker_pass("vault_passphrase_required", &mut pass));
        }
    }

    vault_init_core(key_source, pass, directional).map_err(CliError::code)
}

// NA-0649 (D585 B1): the ingress-independent tail of `vault init`, shared verbatim by
// the CLI path (`vault_init`) and the in-process library entry
// (`vault_init_with_passphrase`). No argv/env/file/stdin/terminal access here; errors
// are returned as marker-code values; the only output is the existing `vault_init`
// success marker.
fn vault_init_core(key_source: KeySource, mut pass: Option<String>, directional: bool) -> Result<(), &'static str> {
    let params = match Params::new(KDF_M_KIB, KDF_T, KDF_P, Some(32)) {
        Ok(p) => p,
        Err(_) => {
            zeroize_passphrase(&mut pass);
            return Err("vault_kdf_params_invalid");
        }
    };
    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);

    let mut pass_bytes = match pass.take() {
        Some(p) => p.into_bytes(),
        None => Vec::new(),
    };

    let mut key_bytes = [0u8; 32];
    let mut salt = [0u8; 16];
    #[cfg(qsc_rng_failure_test_seam)]
    if let Err(code) = vault_rng_fill("QSC.VAULT.INIT.SALT", &mut salt) {
        return Err(fail_core_buffers(code, &mut pass_bytes, &mut key_bytes));
    }
    #[cfg(not(qsc_rng_failure_test_seam))]
    rand_core::OsRng.fill_bytes(&mut salt);

    if let Err(err) = derive_key(
        key_source,
        &argon2,
        &mut pass_bytes,
        &mut salt,
        &mut key_bytes,
    ) {
        pass_bytes.zeroize();
        key_bytes.zeroize();
        return Err(provider_error_code(err));
    }

    let cipher = ChaCha20Poly1305::new(Key::from_slice(&key_bytes));

    let mut nonce_bytes = [0u8; 12];
    #[cfg(qsc_rng_failure_test_seam)]
    if let Err(code) = vault_rng_fill("QSC.VAULT.INIT.NONCE", &mut nonce_bytes) {
        return Err(fail_core_buffers(code, &mut pass_bytes, &mut key_bytes));
    }
    #[cfg(not(qsc_rng_failure_test_seam))]
    rand_core::OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    #[cfg(qsc_rng_failure_test_seam)]
    let default_route_token = match generate_default_route_token() {
        Ok(token) => token,
        Err(code) => return Err(fail_core_buffers(code, &mut pass_bytes, &mut key_bytes)),
    };
    #[cfg(not(qsc_rng_failure_test_seam))]
    let default_route_token = generate_default_route_token();

    let mut payload = match VaultPayload::empty(directional) {
        Ok(payload) => payload,
        Err(code) => return Err(fail_core_buffers(code, &mut pass_bytes, &mut key_bytes)),
    };
    payload.secrets.insert(
        RELAY_INBOX_TOKEN_SECRET_KEY.to_string(),
        default_route_token,
    );
    let plaintext = match serde_json::to_vec(&payload) {
        Ok(v) => v,
        Err(_) => {
            return Err(fail_core_buffers(
                "vault_payload_serialize_failed",
                &mut pass_bytes,
                &mut key_bytes,
            ));
        }
    };

    // NA-0694 (D628 §5.2, ENG-0107): here the encrypt runs BEFORE the header bytes are
    // written, so the AAD is built first from the same inputs the serializer below uses —
    // ct_len is plaintext + the 16-byte Poly1305 tag.
    // F04/S3b D29: the length is checked, never truncated by a cast.
    let ct_len = match envelope_ct_len(plaintext.len()) {
        Ok(n) => n,
        Err(code) => return Err(fail_core_buffers(code, &mut pass_bytes, &mut key_bytes)),
    };
    let aad = envelope_header_bytes(
        key_source_tag(key_source),
        KDF_M_KIB,
        KDF_T,
        KDF_P,
        ct_len,
        &salt,
        &nonce_bytes,
    );
    let ciphertext = match cipher.encrypt(
        nonce,
        Payload {
            msg: plaintext.as_ref(),
            aad: &aad,
        },
    ) {
        Ok(ct) => ct,
        Err(_) => {
            return Err(fail_core_buffers("encrypt_failed", &mut pass_bytes, &mut key_bytes));
        }
    };
    debug_assert_eq!(ciphertext.len(), plaintext.len() + 16);

    // NA-0693 (D627, D-1333): Slice A produced the ConfigSource; consumed here. The lock is
    // taken AFTER the pure-crypto work (every earlier reject still touches nothing on disk)
    // and BEFORE exists(), so it covers the exists()→rename window (N-03) and the write.
    // Acquisition itself may create the config dir and a byte-empty `.qsc.lock` — the one
    // recorded mutation a post-lock reject (e.g. vault_exists) can now leave behind.
    let (cfg_dir, vault_path, source) = match vault_path_resolved() {
        Ok(v) => v,
        Err(code) => return Err(fail_core_buffers(code, &mut pass_bytes, &mut key_bytes)),
    };

    let _lock = match lock_store_exclusive(&cfg_dir, source) {
        Ok(guard) => guard,
        Err(code) => {
            return Err(fail_core_buffers(
                store_err_marker(code),
                &mut pass_bytes,
                &mut key_bytes,
            ));
        }
    };

    if vault_path.exists() {
        return Err(fail_core_buffers("vault_exists", &mut pass_bytes, &mut key_bytes));
    }

    // The acquired store lock is the sole permitted entry in a fresh config.
    let entries = fs::read_dir(&cfg_dir).map_err(|_| "vault_read_failed")?;
    for entry in entries {
        let entry = entry.map_err(|_| "vault_read_failed")?;
        if entry.file_name() != ".qsc.lock" {
            return Err(fail_core_buffers("directional_fresh_vault_required", &mut pass_bytes, &mut key_bytes));
        }
    }

    let parent = match vault_path.parent() {
        Some(p) => p,
        None => return Err(fail_core_buffers("vault_path_invalid", &mut pass_bytes, &mut key_bytes)),
    };

    // Only create directory after all crypto work succeeded to minimize mutation on reject.
    if fs::create_dir_all(parent).is_err() {
        return Err(fail_core_buffers(
            "vault_parent_create_failed",
            &mut pass_bytes,
            &mut key_bytes,
        ));
    }

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        if fs::set_permissions(parent, fs::Permissions::from_mode(0o700)).is_err() {
            return Err(fail_core_buffers("vault_parent_perms_failed", &mut pass_bytes, &mut key_bytes));
        }
    }

    let mut buf = Vec::with_capacity(HEADER_LEN + ciphertext.len());
    buf.extend_from_slice(&envelope_header_bytes(
        key_source_tag(key_source),
        KDF_M_KIB,
        KDF_T,
        KDF_P,
        ct_len,
        &salt,
        &nonce_bytes,
    ));
    buf.extend_from_slice(&ciphertext);

    let tmp = vault_path.with_extension("qsv.tmp");
    if tmp.exists() {
        let _ = fs::remove_file(&tmp);
    }

    // For keychain provider, store the key *before* file write to avoid mutation on reject.
    if key_source == KeySource::Keychain {
        if let Err(err) = keychain_store_key(&salt, &key_bytes) {
            pass_bytes.zeroize();
            key_bytes.zeroize();
            return Err(provider_error_code(err));
        }
    }

    let res = (|| -> Result<(), ()> {
        let mut f = fs::OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(&tmp)
            .map_err(|_| ())?;
        f.write_all(&buf).map_err(|_| ())?;
        f.sync_all().map_err(|_| ())?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(&tmp, fs::Permissions::from_mode(0o600)).map_err(|_| ())?;
        }
        fs::rename(&tmp, &vault_path).map_err(|_| ())?;
        crate::fsync_dir_best_effort(parent);
        Ok(())
    })();

    if res.is_err() {
        let _ = fs::remove_file(&tmp);
        let _ = fs::remove_file(&vault_path);
        if key_source == KeySource::Keychain {
            let _ = keychain_remove_key(&salt);
        }
        return Err(fail_core_buffers("vault_write_failed", &mut pass_bytes, &mut key_bytes));
    }

    // Zeroize secrets after successful commit.
    key_bytes.zeroize();
    pass_bytes.zeroize();

    crate::print_marker("vault_init", &[("path", "redacted")]);
    Ok(())
}

#[cfg(qsc_rng_failure_test_seam)]
fn generate_default_route_token() -> Result<String, &'static str> {
    let mut bytes = [0u8; 16];
    vault_rng_fill("QSC.VAULT.INIT.DEFAULT_ROUTE_TOKEN", &mut bytes)?;
    let mut out = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        out.push_str(format!("{:02x}", b).as_str());
    }
    Ok(out)
}

#[cfg(not(qsc_rng_failure_test_seam))]
fn generate_default_route_token() -> String {
    let mut bytes = [0u8; 16];
    OsRng.fill_bytes(&mut bytes);
    let mut out = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        out.push_str(format!("{:02x}", b).as_str());
    }
    out
}

fn vault_status() -> CliResult {
    let (_cfg_dir, vault_path, _source) = match vault_path_resolved() {
        Ok(v) => v,
        Err(code) => return Err(CliError::code(code)),
    };
    if !vault_path.exists() {
        return Err(CliError::code("vault_missing"));
    }

    let bytes = match read_vault_file(&vault_path) {
        Ok(b) => b,
        Err(e) => return Err(CliError::code(e.code("vault_read_failed"))),
    };

    if bytes.len() < 6 + 1 {
        return Err(CliError::code("vault_parse_failed"));
    }
    // NA-0694 (D628 §5.4, Ruling A): the same three-way version arm as the unlock parser,
    // AFTER this site's own min-length gate — an old dev vault names itself here too.
    match classify_vault_magic(&bytes[..6]) {
        VaultMagicClass::Current => {}
        VaultMagicClass::KnownOld => return Err(CliError::code("vault_version_unsupported")),
        VaultMagicClass::Unknown => return Err(CliError::code("vault_parse_failed")),
    }
    let key_source = key_source_name(bytes[6]);

    crate::print_marker(
        "vault_status",
        &[("present", "true"), ("key_source", key_source)],
    );
    Ok(())
}

fn vault_unlock(args: VaultUnlockArgs) -> CliResult {
    let noninteractive = args.non_interactive
        || std::env::var("QSC_NONINTERACTIVE").ok().as_deref() == Some("1")
        || !std::io::stdin().is_terminal();

    let mut passphrase_buf = String::new();
    let passphrase_env = args
        .passphrase_env
        .as_deref()
        .map(|env_name| env_name.to_string());

    let unlock_result = if let Some(path) = args.passphrase_file.as_deref() {
        unlock_with_passphrase_file(path)
    } else if args.passphrase_stdin {
        match read_passphrase_from_stdin() {
            Ok(passphrase) => {
                passphrase_buf = passphrase;
                unlock_with_passphrase(passphrase_buf.as_str())
            }
            Err(code) => Err(code),
        }
    } else if let Some(env_name) = passphrase_env.as_deref() {
        unlock_with_passphrase_env(Some(env_name))
    } else if noninteractive {
        Err("vault_passphrase_required_noninteractive")
    } else {
        eprint!("vault unlock passphrase: ");
        let _ = std::io::stderr().flush();
        if std::io::stdin().read_line(&mut passphrase_buf).is_err() {
            return Err(CliError::code("vault_locked"));
        }
        while passphrase_buf.ends_with('\n') || passphrase_buf.ends_with('\r') {
            passphrase_buf.pop();
        }
        if passphrase_buf.is_empty() {
            return Err(CliError::code("vault_locked"));
        }
        unlock_with_passphrase(passphrase_buf.as_str())
    };

    match unlock_result {
        Ok(()) => crate::print_marker("vault_unlock", &[("ok", "true"), ("state", "unlocked")]),
        Err(code) => return Err(CliError::code(code)),
    }
    passphrase_buf.zeroize();
    Ok(())
}

#[derive(Clone)]
struct VaultRuntimeEnvelope {
    key_source: u8,
    salt: [u8; 16],
    kdf_m_kib: u32,
    kdf_t: u32,
    kdf_p: u32,
    ciphertext: Vec<u8>,
}

struct VaultRuntime {
    envelope: VaultRuntimeEnvelope,
    key: [u8; 32],
}

// D581 KEEP -> NA-0646 (D582): part of the library's pub GUI surface, seeded for the GUI
// phase; dormant until the GUI consumes it (dead_code allowance retained meanwhile).
#[allow(dead_code)]
pub struct VaultSession {
    vault_path: PathBuf,
    envelope: VaultRuntimeEnvelope,
    key: [u8; 32],
    payload: VaultPayload,
    write_epoch_seen: u64,
}

impl Drop for VaultSession {
    fn drop(&mut self) {
        self.key.zeroize();
        for value in self.payload.secrets.values_mut() {
            value.zeroize();
        }
        self.payload.secrets.clear();
    }
}

static PERF_KDF_CALLS: AtomicU64 = AtomicU64::new(0);
static PERF_VAULT_FILE_READS: AtomicU64 = AtomicU64::new(0);
static PERF_VAULT_DECRYPTS: AtomicU64 = AtomicU64::new(0);
static PERF_VAULT_ENCRYPT_WRITES: AtomicU64 = AtomicU64::new(0);
static VAULT_WRITE_EPOCH: AtomicU64 = AtomicU64::new(0);
static PROCESS_PASSPHRASE: OnceLock<Mutex<Option<String>>> = OnceLock::new();

// NA-0788 F04/S3b N1 (I06, C01 D34): THE vault-file cap -- R-07's bound, now one constant for
// every vault read: the aggregate plus the 53-byte header and the 16-byte tag (16,777,285).
const VAULT_FILE_CAP: usize = crate::protocol_state::REVIEW_AGGREGATE_CANDIDATE + HEADER_LEN + 16;

/// Why a bounded vault read failed. Each call site keeps its own existing code for an open or
/// read failure; an oversized file is always `vault_file_oversized` (QRC-0020).
#[derive(Debug, PartialEq, Eq)]
enum VaultFileError {
    Open,
    Read,
    Oversized,
}

impl VaultFileError {
    fn code(self, io_code: &'static str) -> &'static str {
        match self {
            VaultFileError::Oversized => crate::freshness::codes::VAULT_FILE_OVERSIZED,
            VaultFileError::Open | VaultFileError::Read => io_code,
        }
    }
}

/// THE ONE BOUNDED VAULT READER (F04/S3b N1): every vault-file read goes through here. At most
/// cap + 1 bytes are ever read from `source` -- the byte limit is on the read itself, not a stat
/// taken before it, so a file that grows in between is still bounded -- and more than the cap
/// refuses before anything is parsed.
fn read_vault_limited(source: impl Read) -> Result<Vec<u8>, VaultFileError> {
    let mut bytes = Vec::new();
    source
        .take(VAULT_FILE_CAP as u64 + 1)
        .read_to_end(&mut bytes)
        .map_err(|_| VaultFileError::Read)?;
    if bytes.len() > VAULT_FILE_CAP {
        return Err(VaultFileError::Oversized);
    }
    Ok(bytes)
}

/// The vault file at `path`, through the one bounded reader.
fn read_vault_file(path: &Path) -> Result<Vec<u8>, VaultFileError> {
    read_vault_limited(fs::File::open(path).map_err(|_| VaultFileError::Open)?)
}

fn load_vault_runtime() -> Result<(PathBuf, VaultRuntime), &'static str> {
    load_vault_runtime_with_passphrase(None)
}

fn load_vault_runtime_with_passphrase(
    passphrase_override: Option<&str>,
) -> Result<(PathBuf, VaultRuntime), &'static str> {
    let (_cfg_dir, vault_path, _source) = vault_path_resolved()?;
    PERF_VAULT_FILE_READS.fetch_add(1, Ordering::Relaxed);
    let bytes = read_vault_file(&vault_path).map_err(|e| e.code("vault_missing"))?;
    let envelope = parse_envelope(&bytes)?;
    let mut key = [0u8; 32];
    derive_runtime_key(&envelope, &mut key, passphrase_override)?;
    Ok((vault_path, VaultRuntime { envelope, key }))
}

fn parse_envelope(bytes: &[u8]) -> Result<VaultRuntimeEnvelope, &'static str> {
    let parsed = crate::adversarial::vault_format::parse_vault_envelope(bytes)?;
    // The vault has one truthful on-disk KDF profile, for BOTH key sources — init writes
    // canonical params unconditionally (NA-0694 / N-06: the former passphrase-only gate
    // accepted keychain envelopes' params unread). Reject any other stored profile rather
    // than deriving under attacker-supplied params.
    if parsed.kdf_m_kib != KDF_M_KIB || parsed.kdf_t != KDF_T || parsed.kdf_p != KDF_P {
        return Err("vault_parse_failed");
    }
    // NA-0785 F03 S8b (C01 O7, T2 V8): the ciphertext after the 12-byte nonce must hold at
    // least the 16-byte tag. A shorter one can never authenticate: a format defect, refused
    // here before any key is derived, so it is never counted as a wrong passphrase.
    if parsed.ciphertext.len() < 12 + 16 {
        return Err("vault_parse_failed");
    }
    Ok(VaultRuntimeEnvelope {
        key_source: parsed.key_source,
        salt: parsed.salt,
        kdf_m_kib: parsed.kdf_m_kib,
        kdf_t: parsed.kdf_t,
        kdf_p: parsed.kdf_p,
        ciphertext: parsed.ciphertext,
    })
}

fn derive_runtime_key(
    env: &VaultRuntimeEnvelope,
    out: &mut [u8; 32],
    passphrase_override: Option<&str>,
) -> Result<(), &'static str> {
    PERF_KDF_CALLS.fetch_add(1, Ordering::Relaxed);
    match env.key_source {
        1 => {
            let pass = match passphrase_override {
                Some(v) => v.to_string(),
                None => clone_process_passphrase().ok_or("vault_locked")?,
            };
            if pass.is_empty() {
                return Err("vault_locked");
            }
            let mut pass_bytes = pass.into_bytes();
            let params =
                Params::new(KDF_M_KIB, KDF_T, KDF_P, Some(32)).map_err(|_| "vault_parse_failed")?;
            let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);
            let res = argon2.hash_password_into(&pass_bytes, &env.salt, out);
            pass_bytes.zeroize();
            res.map_err(|_| "vault_locked")
        }
        2 => keychain_load_key(&env.salt, out).map_err(|err| match err {
            // NA-0696 (D630 §5f/R3, D-1336): the three-way split — a missing keychain
            // entry no longer reads as a wrong passphrase; ONLY decrypt failures
            // (downstream of key load) keep `vault_locked`. Defensive arms fail closed
            // under the provider's own name. Zero new strings — every name pre-existed.
            ProviderError::TokenMissing => "vault_token_missing",
            ProviderError::TokenUnavailable => "vault_token_unavailable",
            ProviderError::ProviderFailed
            | ProviderError::EntryExists
            | ProviderError::YubiKeyNotImplemented => "vault_provider_failed",
        }),
        4 => Err("vault_mock_provider_retired"),
        _ => Err("vault_locked"),
    }
}

fn decrypt_payload(env: &VaultRuntime) -> Result<VaultPayload, &'static str> {
    decrypt_payload_typed(env).map_err(|refusal| refusal.code())
}

fn decrypt_payload_typed(env: &VaultRuntime) -> Result<VaultPayload, UnlockRefusal> {
    PERF_VAULT_DECRYPTS.fetch_add(1, Ordering::Relaxed);
    if env.envelope.ciphertext.len() < 12 {
        return Err("vault_parse_failed".into());
    }
    let (nonce_bytes, ciphertext) = env.envelope.ciphertext.split_at(12);
    let cipher = ChaCha20Poly1305::new(Key::from_slice(&env.key));
    let nonce = Nonce::from_slice(nonce_bytes);
    // NA-0694 (D628 §2b, ENG-0107): the AAD is rebuilt byte-exactly from parsed state —
    // the parser fixed the field widths and `ct_len == ciphertext.len() - nonce(12)` by
    // construction, so any altered header byte fails authentication here.
    let aad = envelope_header_bytes(
        env.envelope.key_source,
        env.envelope.kdf_m_kib,
        env.envelope.kdf_t,
        env.envelope.kdf_p,
        ciphertext.len() as u32,
        &env.envelope.salt,
        nonce_bytes.try_into().map_err(|_| "vault_parse_failed")?,
    );
    let plaintext = cipher
        .decrypt(
            nonce,
            Payload {
                msg: ciphertext,
                aad: &aad,
            },
        )
        // C01 O7: the only site that records an AEAD failure, with the key's source.
        .map_err(|_| UnlockRefusal {
            code: "vault_locked",
            aead_key_source: Some(env.envelope.key_source),
        })?;
    let payload: VaultPayload = serde_json::from_slice(&plaintext).map_err(|_| "vault_parse_failed")?;
    check_directional_aggregate(&payload)?;
    Ok(payload)
}

// NA-0694 (D628 §5.2, D-1334): the ONE header serializer — every envelope byte layout in
// src routes through this builder, and the same 53 bytes are the AEAD associated data at
// every encrypt and the decrypt (ENG-0107; the Slice-A one-owner-for-one-layout property).
// PURE byte assembly: no locks, no I/O, no call edges — the D-1333 locked-region boundary
// depends on this staying true.
fn envelope_header_bytes(
    key_source: u8,
    kdf_m_kib: u32,
    kdf_t: u32,
    kdf_p: u32,
    ct_len: u32,
    salt: &[u8; 16],
    nonce: &[u8; 12],
) -> Vec<u8> {
    let mut buf = Vec::with_capacity(HEADER_LEN);
    buf.extend_from_slice(VAULT_MAGIC);
    buf.push(key_source);
    buf.push(16);
    buf.push(12);
    buf.extend_from_slice(&kdf_m_kib.to_le_bytes());
    buf.extend_from_slice(&kdf_t.to_le_bytes());
    buf.extend_from_slice(&kdf_p.to_le_bytes());
    buf.extend_from_slice(&ct_len.to_le_bytes());
    buf.extend_from_slice(salt);
    buf.extend_from_slice(nonce);
    debug_assert_eq!(buf.len(), HEADER_LEN);
    buf
}

/// The envelope's ct_len for `plaintext_len` bytes sealed with the 16-byte tag (F04/S3b, D29): a
/// length that does not fit the header's u32 refuses, never truncated by an `as` cast. The code is
/// the one the directional writer already returned for exactly this condition.
fn envelope_ct_len(plaintext_len: usize) -> Result<u32, &'static str> {
    plaintext_len
        .checked_add(16)
        .and_then(|n| u32::try_from(n).ok())
        .ok_or("directional_capacity_overflow")
}

fn encode_envelope(env: &VaultRuntime, nonce: &[u8], ciphertext: &[u8]) -> Vec<u8> {
    debug_assert_eq!(nonce.len(), 12);
    let mut nonce_arr = [0u8; 12];
    nonce_arr.copy_from_slice(nonce);
    let mut buf = Vec::with_capacity(HEADER_LEN + ciphertext.len());
    buf.extend_from_slice(&envelope_header_bytes(
        env.envelope.key_source,
        env.envelope.kdf_m_kib,
        env.envelope.kdf_t,
        env.envelope.kdf_p,
        ciphertext.len() as u32,
        &env.envelope.salt,
        &nonce_arr,
    ));
    buf.extend_from_slice(ciphertext);
    buf
}

// =============================================================================================
// NA-0788 F04/S6 (SPLIT S11a; C07-02, C07-03; C01 rows 13-16): PAYLOAD VERSION 5 AND ITS OPENER.
// DEAD CODE UNTIL S7: nothing the product runs calls an item of this block. The live VaultPayload
// (v4), PAYLOAD_VERSION, VAULT_MAGIC, parse_vault_envelope, the header serializer and the unlock
// path above are unchanged (P1, P2). S7's profile cut switches the product onto these items and
// removes every dead-code allowance in the block (F-23). The block ends at the banner naming its end.
// =============================================================================================

/// C01 row 15: the payload version under QSCV04. Exact: 4 (the live version) and 6 are
/// `PayloadV5Error::VersionUnsupported`, never read as 5.
#[allow(dead_code)] // removed at S7 (F-23)
const PAYLOAD_VERSION_V5: u64 = 5;

/// C01 row 16 / A14: the vault's storage mode, exactly one of two spellings; the serde renames
/// below are the ONE spelling table (exact match, no case folding, no trimming).
#[allow(dead_code)] // removed at S7 (F-23)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum VaultMode {
    #[serde(rename = "messaging")]
    Messaging,
    #[serde(rename = "storage-only")]
    StorageOnly,
}

impl VaultMode {
    /// The spelling table above applied to a text: `None` for a third value.
    #[allow(dead_code)] // removed at S7 (F-23)
    fn parse(value: &str) -> Option<Self> {
        use serde::de::IntoDeserializer;
        let text: serde::de::value::StrDeserializer<serde::de::value::Error> =
            value.into_deserializer();
        Self::deserialize(text).ok()
    }
}

/// C07 T6.1 F2 / C07-03 through the ONE parser, `freshness::ProtectionMode::parse` (exact match).
#[allow(dead_code)] // removed at S7 (F-23)
mod protection_mode_text {
    use crate::freshness::ProtectionMode;
    use serde::{Deserialize, Deserializer, Serializer};

    pub(super) fn serialize<S: Serializer>(
        mode: &ProtectionMode,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(mode.as_str())
    }
    pub(super) fn deserialize<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<ProtectionMode, D::Error> {
        let text = String::deserialize(deserializer)?;
        ProtectionMode::parse(&text)
            .map_err(|_| serde::de::Error::custom("protection_mode is not a known profile"))
    }
}

/// C07 T6.1 F6 / QQ6: the one accepted primary template spelling.
#[allow(dead_code)] // removed at S7 (F-23)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum PrimaryTemplate {
    #[serde(rename = "qsl-srk-ecc-p256-v1")]
    QslSrkEccP256V1,
}

/// C07 T6.1 F6: the TPM enrollment record -- an object in the "tpm" profile, null in
/// "local-checkpoint" (the combination is checked by `decode_payload_v5`). Every member required,
/// no unknown member; nv_auth is key material (the NV index's auth value) and rests only in
/// zeroizing storage.
#[allow(dead_code)] // removed at S7 (F-23)
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct TpmEnrollmentV5 {
    #[serde(with = "crate::strict_json::b64")]
    nv_public: [u8; 16],
    #[serde(with = "crate::strict_json::b64")]
    nv_auth: Zeroizing<[u8; 32]>,
    primary_template: PrimaryTemplate,
    #[serde(with = "crate::strict_json::b64")]
    primary_name: [u8; 34],
}

/// C07-02: payload version 5 -- the ten FROZEN members, in C07-02's order. Every member is
/// REQUIRED (S6a Q-13: no serde default; an absent member is refused -- the `Option` member goes
/// through `deserialize_with`, which serde treats as required); no unknown member; a repeated
/// member is refused by serde's derive; a repeated `secrets` key by the live `unique_secret_map`;
/// the byte members through the S5 codec (canonical padded base64, 44 characters for 32 bytes);
/// `generation` a u64 JSON number (a string, a sign, a fraction or an exponent is refused).
/// `protocol`'s value is not checked here: its checks are the existing ones, applied by S7. Decoded
/// only through `decode_payload_v5`, which fixes the detection order; written by serde_json in
/// member order (C04 L2), as the live payload is.
#[allow(dead_code)] // removed at S7 (F-23)
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct VaultPayloadV5 {
    version: u8,
    protocol: String,
    mode: VaultMode,
    #[serde(with = "protection_mode_text")]
    protection_mode: crate::freshness::ProtectionMode,
    #[serde(with = "crate::strict_json::b64")]
    vault_id: [u8; 32],
    generation: u64,
    #[serde(with = "crate::strict_json::b64")]
    predecessor_anchor: [u8; 32],
    #[serde(with = "crate::strict_json::b64")]
    checkpoint_mac_key: Zeroizing<[u8; 32]>,
    #[serde(deserialize_with = "crate::strict_json::required")]
    tpm_enrollment: Option<TpmEnrollmentV5>,
    #[serde(deserialize_with = "unique_secret_map")]
    secrets: BTreeMap<String, String>,
}

/// Why a v5 text was not read. Typed, so that the opener maps every kind to `OpenError::Malformed`
/// and S7 maps each to a client code where one exists (`code`).
#[allow(dead_code)] // removed at S7 (F-23)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PayloadV5Error {
    /// `version` is present and not 5 (C01 row 15): the existing vault_version_unsupported.
    VersionUnsupported,
    /// `mode` is absent, or a text that is not one of C01 row 16's two spellings. NO code string
    /// in S6 (P4): C01 row 16's vault_mode_unsupported is PROPOSED, not registered; its spelling is
    /// an operator decision at S7.
    ModeUnsupported,
    /// `protection_mode` is a text that is not one of C07-03's two spellings: QRC-0009.
    ProtectionModeUnsupported,
    /// Anything else: an absent, repeated or unknown member, a member of the wrong JSON type, a
    /// non-canonical or wrong-length byte member, a repeated secrets key, the F6 combination, a
    /// text that is not one JSON object. The existing vault_parse_failed.
    Malformed,
}

impl PayloadV5Error {
    /// The registered client code, where one exists (P4): none for `ModeUnsupported`.
    #[allow(dead_code)] // removed at S7 (F-23)
    pub(crate) fn code(self) -> Option<&'static str> {
        match self {
            Self::VersionUnsupported => Some("vault_version_unsupported"),
            Self::ModeUnsupported => None,
            Self::ProtectionModeUnsupported => {
                Some(crate::freshness::codes::VAULT_PROTECTION_MODE_UNSUPPORTED)
            }
            Self::Malformed => Some("vault_parse_failed"),
        }
    }
}

/// The three discriminators of a v5 text, read from the WHOLE object before any member is decoded
/// (S5's detection order, E2): a version, mode or protection_mode that is not this client's is
/// named as such whatever else the text holds and wherever the member sits (order-independent;
/// SR-15 F5). Every other member is skipped unread. A repeated discriminator, or one of the wrong
/// JSON type, is `Malformed` at once; an absent `version` is `Malformed` (Q-13); an absent `mode`
/// is `ModeUnsupported` (C01 row 16: "unknown/absent"); an absent `protection_mode` is left to the
/// strict decode, which refuses it as absent.
#[allow(dead_code)] // removed at S7 (F-23)
fn peek_v5_discriminators(text: &[u8]) -> Result<(), PayloadV5Error> {
    use serde::de::{IgnoredAny, MapAccess, Visitor};
    #[derive(Default)]
    struct Seen {
        version: Option<u64>,
        mode: Option<String>,
        protection_mode: Option<String>,
    }
    impl<'de> Deserialize<'de> for Seen {
        fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            struct Peek;
            impl<'de> Visitor<'de> for Peek {
                type Value = Seen;
                fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                    f.write_str("a vault payload object")
                }
                fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Seen, A::Error> {
                    let mut seen = Seen::default();
                    while let Some(key) = map.next_key::<String>()? {
                        let repeated = match key.as_str() {
                            "version" => seen.version.replace(map.next_value()?).is_some(),
                            "mode" => seen.mode.replace(map.next_value()?).is_some(),
                            "protection_mode" => {
                                seen.protection_mode.replace(map.next_value()?).is_some()
                            }
                            _ => {
                                map.next_value::<IgnoredAny>()?;
                                false
                            }
                        };
                        if repeated {
                            return Err(serde::de::Error::custom("repeated discriminator"));
                        }
                    }
                    Ok(seen)
                }
            }
            deserializer.deserialize_map(Peek)
        }
    }
    let seen: Seen = serde_json::from_slice(text).map_err(|_| PayloadV5Error::Malformed)?;
    let version = seen.version.ok_or(PayloadV5Error::Malformed)?;
    if version != PAYLOAD_VERSION_V5 {
        return Err(PayloadV5Error::VersionUnsupported);
    }
    if seen.mode.as_deref().and_then(VaultMode::parse).is_none() {
        return Err(PayloadV5Error::ModeUnsupported);
    }
    if let Some(profile) = seen.protection_mode {
        crate::freshness::ProtectionMode::parse(&profile)
            .map_err(|_| PayloadV5Error::ProtectionModeUnsupported)?;
    }
    Ok(())
}

/// The ONE decoder of a v5 payload text: the discriminators first (`peek_v5_discriminators`),
/// then the strict decode of every member, then the F6 combination. Called only on bytes that
/// have authenticated: the opener decrypts first.
#[allow(dead_code)] // removed at S7 (F-23)
pub(crate) fn decode_payload_v5(text: &[u8]) -> Result<VaultPayloadV5, PayloadV5Error> {
    peek_v5_discriminators(text)?;
    let payload: VaultPayloadV5 =
        serde_json::from_slice(text).map_err(|_| PayloadV5Error::Malformed)?;
    // C07 T6.1 F6: null iff local-checkpoint, an object iff tpm; any other combination refuses.
    let consistent = match payload.protection_mode {
        crate::freshness::ProtectionMode::LocalCheckpoint => payload.tpm_enrollment.is_none(),
        crate::freshness::ProtectionMode::Tpm => payload.tpm_enrollment.is_some(),
    };
    if !consistent {
        return Err(PayloadV5Error::Malformed);
    }
    Ok(payload)
}

/// The successor vault's opener: `freshness::LineageOpen` over a QSCV04 envelope holding payload
/// v5, authenticating FIRST. It holds the PASSPHRASE, in zeroizing storage, rather than a key:
/// the key is per blob -- Argon2id over the blob's own salt under the current constants (P6),
/// derived by the unlock path's `derive_runtime_key` -- and the trait hands it arbitrary blobs
/// (the current vault, a prepared slot). Passphrase key source only: it holds no key for any
/// other source, so any other `key_source` byte is `Unauthenticated`.
///
/// `open(blob)`, in order: (1) LOCATE -- the existing header layout (HEADER_LEN bytes, then the
/// ciphertext and its tag): the QSCV04 magic by `classify_vault_magic_v4`, key_source 1, the
/// fixed salt and nonce widths, the three KDF words equal to KDF_M_KIB/KDF_T/KDF_P (I06: never an
/// Argon2 run under a foreign profile), ct_len exactly the bytes that follow, at least the tag;
/// (2) DERIVE the key; (3) AUTHENTICATE with the existing AEAD, the AAD being the blob's own 53
/// header bytes -- the 53 bytes `envelope_header_bytes` writes, magic included, which is what the
/// serializer writes once VAULT_MAGIC is QSCV04 at S7. EVERY failure through (3) is
/// `Unauthenticated` (DF-12): no plaintext header field decides anything but where the ciphertext
/// is and which key opens it. Only a text that authenticated can be `Malformed` (4:
/// `decode_payload_v5`); success is the `LineageFields` constructor over the five C07 values, the
/// MAC key MOVED out of the payload's zeroizing storage. Everything else in the payload
/// (`secrets` included) is dropped here, unread.
#[allow(dead_code)] // removed at S7 (F-23)
pub(crate) struct PassphraseOpener {
    passphrase: Zeroizing<String>,
}

impl PassphraseOpener {
    #[allow(dead_code)] // removed at S7 (F-23)
    pub(crate) fn new(passphrase: Zeroizing<String>) -> Self {
        Self { passphrase }
    }
}

impl crate::freshness::LineageOpen for PassphraseOpener {
    fn open(&self, blob: &[u8]) -> Result<LineageFields, OpenError> {
        // (1) locate.
        if blob.len() < HEADER_LEN + 16 {
            return Err(OpenError::Unauthenticated);
        }
        let (header, ciphertext) = blob.split_at(HEADER_LEN);
        if classify_vault_magic_v4(&header[..6]) != VaultMagicClass::Current
            || header[6] != key_source_tag(KeySource::Passphrase)
            || header[7] != 16
            || header[8] != 12
        {
            return Err(OpenError::Unauthenticated);
        }
        let word = |at: usize| u32::from_le_bytes(header[at..at + 4].try_into().expect("4 bytes"));
        if word(9) != KDF_M_KIB || word(13) != KDF_T || word(17) != KDF_P {
            return Err(OpenError::Unauthenticated);
        }
        if usize::try_from(word(21)).ok() != Some(ciphertext.len()) {
            return Err(OpenError::Unauthenticated);
        }
        let salt: [u8; 16] = header[25..41].try_into().expect("16 bytes");
        let nonce = Nonce::from_slice(&header[41..HEADER_LEN]);
        // (2) derive, by the unlock path's own derivation, into zeroizing storage.
        let envelope = VaultRuntimeEnvelope {
            key_source: key_source_tag(KeySource::Passphrase),
            salt,
            kdf_m_kib: KDF_M_KIB,
            kdf_t: KDF_T,
            kdf_p: KDF_P,
            ciphertext: Vec::new(),
        };
        let mut key = Zeroizing::new([0u8; 32]);
        derive_runtime_key(&envelope, &mut key, Some(&self.passphrase))
            .map_err(|_| OpenError::Unauthenticated)?;
        // (3) authenticate: the existing AEAD, the AAD the header as written.
        let plaintext = Zeroizing::new(
            ChaCha20Poly1305::new(Key::from_slice(key.as_slice()))
                .decrypt(
                    nonce,
                    Payload {
                        msg: ciphertext,
                        aad: header,
                    },
                )
                .map_err(|_| OpenError::Unauthenticated)?,
        );
        // (4) only now may the payload be unreadable.
        let VaultPayloadV5 {
            vault_id,
            generation,
            predecessor_anchor,
            checkpoint_mac_key,
            protection_mode,
            ..
        } = decode_payload_v5(&plaintext).map_err(|_| OpenError::Malformed)?;
        Ok(LineageFields::new(
            vault_id,
            generation,
            predecessor_anchor,
            checkpoint_mac_key,
            protection_mode,
        ))
    }
}
// ============================== END OF S6 (payload v5 and the opener) ==============================

// NA-0693 (D627, D-1333): the vault-local duplicate writer is DELETED — `fs_store::write_atomic`
// is the one hardened write primitive (unique tmp name closes N-02; `enforce_safe_parents` is
// N-04 arriving by design; dir creation and 0700 enforcement moved to exclusive-lock acquisition
// at transaction start; the NA-0669 dir-fsync survives inside `write_atomic`). This mapper is
// the ruled `ErrorCode` → vault-marker translation: `IoWriteFailed` keeps the vault's pinned
// write-path marker; every other cause keeps its own tree-wide `as_str` name — no two causes
// share a marker, and contention surfaces fail-closed as `lock_contended` (no retry loop;
// retry policy is ENG-0111's design space).
// NA-0696 (D630 §5c, D-1336): pub(crate) so the D1(c) commit transaction maps its lock
// acquisition through the ONE owner of the cause-name mapping — no cause loses its name.
pub(crate) fn store_err_marker(code: ErrorCode) -> &'static str {
    match code {
        ErrorCode::IoWriteFailed => "vault_write_failed",
        other => other.as_str(),
    }
}

fn resolve_key_source(args: &VaultInitArgs) -> Result<KeySource, &'static str> {
    let env_src = std::env::var("QSC_KEY_SOURCE").ok();
    let src = args
        .key_source
        .as_ref()
        .or(env_src.as_ref())
        .map(|s| s.as_str());

    match src {
        Some("yubikey") => Ok(KeySource::YubiKeyStub),
        Some("keychain") => Ok(KeySource::Keychain),
        Some("passphrase") => Ok(KeySource::Passphrase),
        Some("mock") => Err("vault_mock_provider_retired"),
        Some(_) => Err("key_source_invalid"),
        None => {
            if std::env::var("QSC_DISABLE_KEYCHAIN").ok().as_deref() == Some("1") {
                Ok(KeySource::Passphrase)
            } else if keychain_supported() {
                Ok(KeySource::Keychain)
            } else {
                Ok(KeySource::Passphrase)
            }
        }
    }
}

fn key_source_explicit(args: &VaultInitArgs) -> bool {
    args.key_source.is_some() || std::env::var("QSC_KEY_SOURCE").ok().is_some()
}

fn key_source_tag(src: KeySource) -> u8 {
    match src {
        KeySource::Passphrase => 1,
        KeySource::Keychain => 2,
        KeySource::YubiKeyStub => 3,
    }
}

fn key_source_name(tag: u8) -> &'static str {
    match tag {
        1 => "passphrase",
        2 => "keychain",
        3 => "yubikey",
        4 => "mock_retired",
        _ => "unknown",
    }
}

fn keychain_supported() -> bool {
    if std::env::var("QSC_DISABLE_KEYCHAIN").ok().as_deref() == Some("1") {
        return false;
    }
    #[cfg(all(feature = "keychain", qsc_keychain_test_seam))]
    if keychain_seam_dir().is_some() {
        return true;
    }
    #[cfg(feature = "keychain")]
    {
        Entry::new(VAULT_KEYCHAIN_SERVICE, VAULT_KEYCHAIN_PROBE_ACCOUNT).is_ok()
    }
    #[cfg(not(feature = "keychain"))]
    {
        false
    }
}

// NA-0695 (D629 R1, E-A, D-1335): the ONE account-derivation site — the keychain account is
// per-vault BY CONSTRUCTION ("vault-" + raw hex of the envelope salt, 38 chars). Every salt
// this reads was either drawn by init before the store call or parsed through
// `parse_vault_envelope` (D-1334's one parser), and no code path ever re-salts an existing
// vault (E-A: salt-fill sites = 1), so the address is vault-lifetime-stable. No caller
// assembles an address (the D-1332 one-owner property). The account string is an ADDRESS,
// not key material — deliberately not zeroized (§5a).
#[cfg(feature = "keychain")]
fn vault_keychain_account(salt: &[u8; 16]) -> String {
    format!("vault-{}", hex_encode(salt))
}

fn keychain_store_key(salt: &[u8; 16], key: &[u8]) -> Result<(), ProviderError> {
    #[cfg(feature = "keychain")]
    {
        let account = vault_keychain_account(salt);

        // Raw existence read — the seam swaps exactly this read (E-B); the refuse DECISION
        // below is the single shared copy both backends feed. NA-0696 (D630 §5f): the raw
        // primitive now reports absent-vs-unreadable; an unreadable seam store fails
        // CLOSED here exactly as the real backend's arm below does (R4).
        #[cfg(qsc_keychain_test_seam)]
        let seam_existing: Option<bool> = match keychain_seam_dir() {
            Some(dir) => match keychain_seam_get(&dir, &account) {
                Ok(found) => Some(found.is_some()),
                Err(()) => return Err(ProviderError::ProviderFailed),
            },
            None => None,
        };
        #[cfg(not(qsc_keychain_test_seam))]
        let seam_existing: Option<bool> = None;
        let mut entry_slot: Option<Entry> = None;
        let existing = match seam_existing {
            Some(found) => found,
            None => {
                let entry = Entry::new(VAULT_KEYCHAIN_SERVICE, &account)
                    .map_err(|_| ProviderError::ProviderFailed)?;
                let found = match entry.get_password() {
                    Ok(mut prior) => {
                        prior.zeroize();
                        true
                    }
                    Err(keyring::Error::NoEntry) => false,
                    // Fail CLOSED (R4): an unreadable store must never fail open into an
                    // overwrite.
                    Err(_) => return Err(ProviderError::ProviderFailed),
                };
                entry_slot = Some(entry);
                found
            }
        };

        // THE refuse (R4): one decision, exercised identically by the real and seam
        // backends (E-B — the seam must never carry its own copy of this).
        if existing {
            return Err(ProviderError::EntryExists);
        }

        let mut enc = hex_encode(key);
        // Raw write — the seam swaps exactly this write (E-B).
        #[cfg(qsc_keychain_test_seam)]
        if let Some(dir) = keychain_seam_dir() {
            let res = keychain_seam_set(&dir, &account, &enc);
            enc.zeroize();
            return res;
        }
        let res = match entry_slot {
            Some(entry) => entry
                .set_password(&enc)
                .map_err(|_| ProviderError::ProviderFailed),
            None => Err(ProviderError::ProviderFailed),
        };
        enc.zeroize();
        res?;
        Ok(())
    }
    #[cfg(not(feature = "keychain"))]
    {
        let _ = (salt, key);
        Err(ProviderError::TokenUnavailable)
    }
}

fn keychain_load_key(salt: &[u8; 16], out: &mut [u8; 32]) -> Result<(), ProviderError> {
    #[cfg(feature = "keychain")]
    {
        let account = vault_keychain_account(salt);
        // Raw read — the seam swaps exactly this read (E-B); the decode below is shared.
        // NA-0696 (D630 §5f, D-1336; E-B extension, BINDING): the CLASSIFICATION lives
        // once, expressed identically for both backends — an ABSENT entry is
        // `TokenMissing` (this arm and the real backend's NoEntry arm below are the same
        // decision); an unreadable seam dir stays `TokenUnavailable` (the daemon-down
        // class).
        #[cfg(qsc_keychain_test_seam)]
        let seam_secret: Option<String> = match keychain_seam_dir() {
            Some(dir) => match keychain_seam_get(&dir, &account) {
                Ok(Some(value)) => Some(value),
                Ok(None) => return Err(ProviderError::TokenMissing),
                Err(()) => return Err(ProviderError::TokenUnavailable),
            },
            None => None,
        };
        #[cfg(not(qsc_keychain_test_seam))]
        let seam_secret: Option<String> = None;
        let secret = match seam_secret {
            Some(value) => value,
            None => {
                let entry = Entry::new(VAULT_KEYCHAIN_SERVICE, &account)
                    .map_err(|_| ProviderError::ProviderFailed)?;
                entry.get_password().map_err(|err| match err {
                    keyring::Error::NoEntry => ProviderError::TokenMissing,
                    _ => ProviderError::TokenUnavailable,
                })?
            }
        };
        if !key_from_hex(&secret, out) {
            return Err(ProviderError::ProviderFailed);
        }
        Ok(())
    }
    #[cfg(not(feature = "keychain"))]
    {
        let _ = (salt, out);
        Err(ProviderError::TokenUnavailable)
    }
}

fn keychain_remove_key(salt: &[u8; 16]) -> Result<(), ProviderError> {
    #[cfg(feature = "keychain")]
    {
        let account = vault_keychain_account(salt);
        // Raw delete — the seam swaps exactly this delete (E-B).
        #[cfg(qsc_keychain_test_seam)]
        if let Some(dir) = keychain_seam_dir() {
            return keychain_seam_delete(&dir, &account);
        }
        let entry = Entry::new(VAULT_KEYCHAIN_SERVICE, &account)
            .map_err(|_| ProviderError::ProviderFailed)?;
        entry
            .delete_credential()
            .map_err(|_| ProviderError::ProviderFailed)?;
        Ok(())
    }
    #[cfg(not(feature = "keychain"))]
    {
        let _ = salt;
        Err(ProviderError::TokenUnavailable)
    }
}

// NA-0695 (D629 §5c, R3, D-1335): the cfg-fenced FILE-BACKED keychain test seam — the
// instrument that makes the banked two-profiles acceptance red-capable headless (keyring's
// built-in mock is EntryOnly and cannot model cross-call collision, §0.5). One file per
// (service, account) under the env-named directory; the store survives process boundaries
// because the corpus drives spawned binaries and a real keychain IS cross-process state.
// ⚠ E-B (BINDING): these functions are RAW STORAGE PRIMITIVES ONLY — get/set/delete on an
// already-derived (service, account) key. The account derivation and the exists→refuse
// decision live exactly once, in the shared helper bodies above; this seam must never
// carry its own copy of either. ⚠ Plaintext store, test-only: compiled solely under
// `--cfg qsc_keychain_test_seam` (never a default or release build), and the env var alone
// can never conjure a store where the cfg is absent — the env read itself is cfg-fenced
// (the rng-seam twin-arm property; test (ii) pins it).
#[cfg(all(feature = "keychain", qsc_keychain_test_seam))]
fn keychain_seam_dir() -> Option<PathBuf> {
    match std::env::var("QSC_KEYCHAIN_TEST_SEAM") {
        Ok(v) if !v.trim().is_empty() => Some(PathBuf::from(v)),
        _ => None,
    }
}

#[cfg(all(feature = "keychain", qsc_keychain_test_seam))]
fn keychain_seam_entry_path(dir: &Path, account: &str) -> PathBuf {
    dir.join(format!("{}__{}", VAULT_KEYCHAIN_SERVICE, account))
}

#[cfg(all(feature = "keychain", qsc_keychain_test_seam))]
fn keychain_seam_get(dir: &Path, account: &str) -> Result<Option<String>, ()> {
    // Raw storage outcome ONLY (E-B): present → the value; absent (NotFound) → Ok(None);
    // any other read failure (an unreadable seam dir modeling daemon-down) → Err. What
    // these outcomes MEAN is decided once, in `keychain_load_key`, for both backends.
    match fs::read_to_string(keychain_seam_entry_path(dir, account)) {
        Ok(value) => Ok(Some(value)),
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(_) => Err(()),
    }
}

#[cfg(all(feature = "keychain", qsc_keychain_test_seam))]
fn keychain_seam_set(dir: &Path, account: &str, value: &str) -> Result<(), ProviderError> {
    if fs::create_dir_all(dir).is_err() {
        return Err(ProviderError::ProviderFailed);
    }
    fs::write(keychain_seam_entry_path(dir, account), value)
        .map_err(|_| ProviderError::ProviderFailed)
}

#[cfg(all(feature = "keychain", qsc_keychain_test_seam))]
fn keychain_seam_delete(dir: &Path, account: &str) -> Result<(), ProviderError> {
    // Mirrors the real backend's remove mapping: any failure (including a missing entry)
    // surfaces as ProviderFailed.
    fs::remove_file(keychain_seam_entry_path(dir, account))
        .map_err(|_| ProviderError::ProviderFailed)
}

#[cfg(feature = "keychain")]
fn hex_encode(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut out = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        out.push(HEX[(b >> 4) as usize] as char);
        out.push(HEX[(b & 0x0f) as usize] as char);
    }
    out
}

/// The keychain entry's 32-byte key from its hex (F04/S3b N5, I-B5): any length other than 64
/// hex characters refuses FIRST, before anything is decoded; the decode fills a fixed stack
/// buffer (no allocation) and `out` is written only when every digit decoded.
#[cfg(any(feature = "keychain", test))]
fn key_from_hex(secret: &str, out: &mut [u8; 32]) -> bool {
    let digits = secret.as_bytes();
    if digits.len() != 2 * out.len() {
        return false;
    }
    let mut key = [0u8; 32];
    for (i, byte) in key.iter_mut().enumerate() {
        match (hex_nibble(digits[2 * i]), hex_nibble(digits[2 * i + 1])) {
            (Some(hi), Some(lo)) => *byte = (hi << 4) | lo,
            _ => {
                key.zeroize();
                return false;
            }
        }
    }
    out.copy_from_slice(&key);
    key.zeroize();
    true
}

#[cfg(any(feature = "keychain", test))]
fn hex_nibble(c: u8) -> Option<u8> {
    match c {
        b'0'..=b'9' => Some(c - b'0'),
        b'a'..=b'f' => Some(c - b'a' + 10),
        b'A'..=b'F' => Some(c - b'A' + 10),
        _ => None,
    }
}

fn resolve_passphrase(args: &mut VaultInitArgs) -> Result<Option<String>, &'static str> {
    if let Some(mut passphrase) = args.passphrase.take() {
        let retired = !passphrase.is_empty();
        passphrase.zeroize();
        if retired {
            return Err("vault_passphrase_argv_retired");
        }
    }

    if args.passphrase_env.take().is_some() {
        return Err("vault_passphrase_env_retired");
    }

    if let Some(path) = args.passphrase_file.as_deref() {
        return read_passphrase_file(path).map(Some);
    }

    if args.passphrase_stdin {
        return read_passphrase_from_stdin().map(Some);
    }

    Ok(None)
}

fn process_passphrase_slot() -> &'static Mutex<Option<String>> {
    PROCESS_PASSPHRASE.get_or_init(|| Mutex::new(None))
}

fn clone_process_passphrase() -> Option<String> {
    process_passphrase_slot()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .clone()
}

pub fn set_process_passphrase(passphrase: Option<&str>) {
    let mut slot = process_passphrase_slot()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    if let Some(existing) = slot.as_mut() {
        existing.zeroize();
    }
    *slot = passphrase.map(|value| value.to_string());
}

// D581 KEEP -> NA-0646 (D582): part of the library's pub GUI surface, seeded for the GUI
// phase; dormant until the GUI consumes it (dead_code allowance retained meanwhile).
#[allow(dead_code)]
pub fn has_process_passphrase() -> bool {
    process_passphrase_slot()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .as_ref()
        .map(|value| !value.is_empty())
        .unwrap_or(false)
}

pub fn passphrase_env_allowed(env_name: &str) -> bool {
    env_name == DESKTOP_PASS_ENV_KEY
}

pub fn passphrase_from_allowed_env(env_name: &str) -> Result<String, &'static str> {
    if env_name.trim().is_empty() {
        return Err("vault_locked");
    }
    if !passphrase_env_allowed(env_name) {
        return Err("vault_passphrase_env_retired");
    }
    let passphrase = std::env::var(env_name).map_err(|_| "vault_locked")?;
    if passphrase.is_empty() {
        return Err("vault_locked");
    }
    Ok(passphrase)
}

pub fn read_passphrase_file(path: &Path) -> Result<String, &'static str> {
    let bytes = fs::read(path).map_err(|_| "vault_passphrase_file_read_failed")?;
    // NA-0669 (C-4): REJECT non-UTF-8 rather than transforming it. `from_utf8_lossy` collapsed
    // every invalid byte to U+FFFD, so `head -c 32 /dev/urandom > pass.txt` produced a vault the
    // operator believed held 256 bits and which held ~144 (measured: one random byte retains
    // Shannon H = 4.500 bits of 8). That silent degradation is the defect; failing loudly at the
    // moment of use is the fix. This also removes the ingress asymmetry that was the tell —
    // `read_passphrase_from_stdin` already errors on invalid UTF-8 via `read_to_string`.
    let mut passphrase =
        String::from_utf8(bytes).map_err(|_| "vault_passphrase_file_read_failed")?;
    while passphrase.ends_with('\n') || passphrase.ends_with('\r') {
        passphrase.pop();
    }
    if passphrase.is_empty() {
        return Err("vault_passphrase_file_read_failed");
    }
    Ok(passphrase)
}

fn read_passphrase_from_stdin() -> Result<String, &'static str> {
    let mut buf = String::new();
    std::io::stdin()
        .read_to_string(&mut buf)
        .map_err(|_| "vault_locked")?;
    while buf.ends_with('\n') || buf.ends_with('\r') {
        buf.pop();
    }
    if buf.is_empty() {
        return Err("vault_locked");
    }
    Ok(buf)
}

fn derive_key(
    key_source: KeySource,
    argon2: &Argon2,
    pass_bytes: &mut [u8],
    salt: &mut [u8; 16],
    key_bytes: &mut [u8; 32],
) -> Result<(), ProviderError> {
    match key_source {
        KeySource::Passphrase => {
            if argon2
                .hash_password_into(pass_bytes, salt, key_bytes)
                .is_err()
            {
                return Err(ProviderError::ProviderFailed);
            }
        }
        KeySource::Keychain => {
            rand_core::OsRng.fill_bytes(key_bytes);
        }
        KeySource::YubiKeyStub => {
            return Err(ProviderError::YubiKeyNotImplemented);
        }
    }
    Ok(())
}

fn provider_error_code(err: ProviderError) -> &'static str {
    match err {
        ProviderError::YubiKeyNotImplemented => "vault_yubikey_not_implemented",
        ProviderError::TokenMissing => "vault_token_missing",
        ProviderError::TokenUnavailable => "vault_token_unavailable",
        ProviderError::ProviderFailed => "vault_provider_failed",
        ProviderError::EntryExists => "vault_keychain_entry_exists",
    }
}

fn handle_provider_error(err: ProviderError) -> CliError {
    CliError::code(provider_error_code(err))
}

fn zeroize_passphrase(pass: &mut Option<String>) {
    if let Some(p) = pass.as_mut() {
        p.zeroize();
    }
}

fn fail_with_marker_pass(code: &str, pass: &mut Option<String>) -> CliError {
    zeroize_passphrase(pass);
    CliError::code(code)
}

fn fail_core_buffers(
    code: &'static str,
    pass_bytes: &mut Vec<u8>,
    key_bytes: &mut [u8; 32],
) -> &'static str {
    pass_bytes.zeroize();
    key_bytes.zeroize();
    code
}

fn handle_provider_error_with_pass(err: ProviderError, pass: &mut Option<String>) -> CliError {
    zeroize_passphrase(pass);
    handle_provider_error(err)
}

// NA-0692 (ENG-0109): ONE config-directory resolver in the crate.
//
// This used to re-implement `fs_store::config_dir` without its `!v.trim().is_empty()`
// guard, so a blank or whitespace config-dir variable put the vault at a RELATIVE path
// while the lock, the protection state and the store metadata — which all resolve
// through `config_dir` — fell through to the XDG or home location. The vault and the
// unlock counter that limits attempts against it ended up in different directories.
// Delegating (rather than copying the guard in) is the fix, because the defect is the
// DUPLICATION: a copied guard would close the symptom and leave two resolvers behind.
//
// ⚠ `config_dir` has exactly ONE `Err` return, measured at NA-0692:
// `ErrorCode::MissingHome` (`fs_store/mod.rs:29`). There are no `?` operators and no
// other early returns in its body, so the blanket `map_err` below is total and lossless
// AS MEASURED. If a second `Err` variant is ever added to `config_dir`, THIS SITE MUST
// MAP IT EXPLICITLY — a wildcard arm would launder a new variant exactly as silently as
// `|_|` does, and `ErrorCode` is too large for an exhaustive match to be practical.
fn vault_path_resolved() -> Result<(PathBuf, PathBuf, ConfigSource), &'static str> {
    let (cfg, source) = crate::fs_store::config_dir().map_err(|_| "vault_config_missing")?;
    Ok((cfg.clone(), cfg.join("vault.qsv"), source))
}

// NA-0692 (D626, D-1332): ENG-0109 — the `ConfigSource` pin.
//
// `vault_path_resolved` is PRIVATE and stays private, and `ConfigSource` is
// `pub(crate)`, so a same-file `#[cfg(test)] mod` is the ONLY place in the tree that
// can observe what this resolver returns. That is the property that made the
// `confirm_capture_reason_tests` precedent (`transport/mod.rs:4394`) the right shape:
// the function under test is private, so only a module inside the same file can call
// it directly. Exporting the resolver to make an external test compile would trade
// the encapsulation for the instrument.
//
// ⚠ `ConfigSource` derives only `Debug, Clone, Copy` (`model/mod.rs:44`) — there is
// NO `PartialEq`. Assert with `matches!`; adding a derive to a shared crate-wide type
// to make one assertion compile would widen a type this lane does not own.
//
// ⚠ These are the FIRST env-mutating tests in the lib unit-test binary (measured at
// NA-0692: 111 `#[test]` functions across 14 files in `qsc/src`, zero `set_var` /
// `remove_var`, and none of them resolves the config directory from the environment).
// They carry their OWN `ENV_LOCK`, every test takes it, and every variable touched is
// snapshotted and restored including the unset case. `set_var` is safe without an
// `unsafe` block on this crate — `edition = "2021"` (`qsc/Cargo.toml:4`).
#[cfg(test)]
mod na0692_config_resolver_tests {
    use super::vault_path_resolved;
    use crate::model::ConfigSource;
    use std::sync::{Mutex, MutexGuard, OnceLock};

    /// Every variable these tests write. `HOME` is deliberately NOT in the set: both
    /// cases set a non-blank earlier-precedence variable, so no branch reaches it.
    const TOUCHED_VARS: [&str; 2] = ["QSC_CONFIG_DIR", "XDG_CONFIG_HOME"];

    static ENV_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

    fn env_lock() -> MutexGuard<'static, ()> {
        ENV_LOCK
            .get_or_init(|| Mutex::new(()))
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
    }

    /// Restores on `Drop`, including the unset case, so a panicking assertion cannot
    /// leak a mutated environment into the rest of this binary.
    struct EnvSnapshot {
        vars: Vec<(&'static str, Option<String>)>,
    }

    impl EnvSnapshot {
        fn take() -> Self {
            Self {
                vars: TOUCHED_VARS
                    .iter()
                    .map(|k| (*k, std::env::var(k).ok()))
                    .collect(),
            }
        }
    }

    impl Drop for EnvSnapshot {
        fn drop(&mut self) {
            for (key, value) in &self.vars {
                match value {
                    Some(v) => std::env::set_var(key, v),
                    None => std::env::remove_var(key),
                }
            }
        }
    }

    /// THE NEGATIVE CONTROL — the two resolvers must not diverge.
    ///
    /// A blank `QSC_CONFIG_DIR` is exactly the input that used to split them: the
    /// vault took `PathBuf::from("")` and landed at a relative path while the store,
    /// the lock and the protection state fell through to XDG. Nothing here touches
    /// the filesystem; the paths need not exist to be resolved.
    #[test]
    fn a_blank_config_dir_override_resolves_the_vault_and_the_store_to_the_same_directory() {
        let _guard = env_lock();
        let _snapshot = EnvSnapshot::take();

        let xdg = std::env::temp_dir().join(format!("na0692-xdg-{}", std::process::id()));
        std::env::set_var("QSC_CONFIG_DIR", "");
        std::env::set_var("XDG_CONFIG_HOME", &xdg);

        let (cfg, vault_path, source) = vault_path_resolved().expect("the resolver must succeed");
        let (store_cfg, store_source) =
            crate::fs_store::config_dir().expect("config_dir must succeed");

        assert_eq!(
            cfg, store_cfg,
            "the vault and the store must resolve to the SAME directory"
        );
        assert_eq!(
            cfg,
            xdg.join("qsc"),
            "a blank QSC_CONFIG_DIR must fall through to XDG_CONFIG_HOME"
        );
        assert_eq!(vault_path, cfg.join("vault.qsv"));
        assert!(
            matches!(source, ConfigSource::XdgConfigHome),
            "the vault's source must be XdgConfigHome, got {:?}",
            source
        );
        assert!(
            matches!(store_source, ConfigSource::XdgConfigHome),
            "the store's source must be XdgConfigHome, got {:?}",
            store_source
        );
    }

    /// THE POSITIVE CONTROL — the instrument sees AGREEMENT, not merely the absence
    /// of divergence.
    ///
    /// Without this, a resolver that returned the same wrong answer twice, or a test
    /// that never exercised the override branch at all, would look identical to a
    /// correct one. `XDG_CONFIG_HOME` is set to a value that must NOT be chosen, so
    /// precedence is observed rather than assumed.
    #[test]
    fn an_absolute_config_dir_override_resolves_both_resolvers_to_that_path() {
        let _guard = env_lock();
        let _snapshot = EnvSnapshot::take();

        let override_dir =
            std::env::temp_dir().join(format!("na0692-override-{}", std::process::id()));
        assert!(
            override_dir.is_absolute(),
            "the override under test must be an absolute path"
        );
        std::env::set_var("QSC_CONFIG_DIR", &override_dir);
        std::env::set_var("XDG_CONFIG_HOME", "/na0692/must/not/be/chosen");

        let (cfg, vault_path, source) = vault_path_resolved().expect("the resolver must succeed");
        let (store_cfg, store_source) =
            crate::fs_store::config_dir().expect("config_dir must succeed");

        assert_eq!(cfg, override_dir, "the vault must honour the override");
        assert_eq!(
            store_cfg, override_dir,
            "the store must honour the override"
        );
        assert_eq!(vault_path, override_dir.join("vault.qsv"));
        assert!(
            matches!(source, ConfigSource::EnvOverride),
            "the vault's source must be EnvOverride, got {:?}",
            source
        );
        assert!(
            matches!(store_source, ConfigSource::EnvOverride),
            "the store's source must be EnvOverride, got {:?}",
            store_source
        );
    }
}

// NA-0695 (D629 §4c, D-1335): the one collision `tests/` cannot construct — init always
// draws a fresh salt, so the same-salt second store (the §5b refuse, directly) is reachable
// only here. Runs ONLY under the seam-armed lane build (R3); goal-lint is path-based and an
// in-src test never satisfies it — the gate-satisfying instruments live in
// `tests/na0695_vault_keychain_addressing.rs`. Deliberately never calls
// `vault_keychain_account` (the §7.1 one-owner call-site count stays at the three helpers).
#[cfg(all(test, feature = "keychain", qsc_keychain_test_seam))]
mod na0695_keychain_refuse_unit {
    use super::*;

    #[test]
    fn same_salt_second_store_refuses_with_entry_exists() {
        let dir = std::env::temp_dir().join(format!("na0695-seam-unit-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).expect("seam dir");
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(&dir, fs::Permissions::from_mode(0o700)).expect("seam perms");
        }
        std::env::set_var("QSC_KEYCHAIN_TEST_SEAM", &dir);

        let salt = [0x5a_u8; 16];
        let first_key = [0x11_u8; 32];
        let second_key = [0x22_u8; 32];
        keychain_store_key(&salt, &first_key).expect("first store");
        let second = keychain_store_key(&salt, &second_key);
        assert!(
            matches!(second, Err(ProviderError::EntryExists)),
            "second same-salt store must refuse, got {:?}",
            second
        );
        assert_eq!(
            provider_error_code(ProviderError::EntryExists),
            "vault_keychain_entry_exists"
        );
        // Refuse means ZERO mutation: the first key is still the stored one.
        let mut out = [0u8; 32];
        keychain_load_key(&salt, &mut out).expect("load after refuse");
        assert_eq!(out, first_key, "refuse must not overwrite the stored key");

        std::env::remove_var("QSC_KEYCHAIN_TEST_SEAM");
        let _ = fs::remove_dir_all(&dir);
    }
}

// R02 integrated UNAPPLIED review: strict fresh schema; no migration/backfill.
// Application and execution remain gated independently of operator allocation.
fn read_directional_runtime(path:&Path, authenticated_key:Option<[u8;32]>)
    ->Result<VaultRuntime,&'static str> {
    // Bound reads on the opened handle, including nonce/tag/header allowance.
    // This conditional proposal is NOT a claim existing vaults fit this budget.
    // F04/S3b N1 (D34): the one bounded reader, on this same handle; an oversized file is
    // vault_file_oversized, not the aggregate's backpressure code.
    let file=fs::File::open(path).map_err(|_|"vault_missing")?;
    let metadata=file.metadata().map_err(|_|"vault_parse_failed")?;
    if !metadata.is_file() {
        return Err("directional_aggregate_waiting");
    }
    let bytes = read_vault_limited(file).map_err(|e| e.code("vault_parse_failed"))?;
    let envelope=parse_envelope(&bytes)?;
    let mut key=authenticated_key.unwrap_or([0;32]);
    if authenticated_key.is_none() {derive_runtime_key(&envelope,&mut key,None)?;}
    Ok(VaultRuntime{envelope,key})
}
// Ordinary sessions retain existing storage read behavior; the conditional R02
// evaluation envelope applies only to authenticated directional state. A schema
// transition is never legal, so the fresh payload must match the session tuple.
fn read_session_runtime(session:&VaultSession)->Result<VaultRuntime,&'static str> {
    let runtime = if session.payload.version == PAYLOAD_VERSION
        && session.payload.protocol == OWNER_FREE_PROFILE {
        let bytes = read_vault_file(&session.vault_path).map_err(|e| e.code("vault_missing"))?;
        VaultRuntime { envelope: parse_envelope(&bytes)?, key: session.key }
    } else { read_directional_runtime(&session.vault_path,Some(session.key))? };
    let current = decrypt_payload(&runtime)?;
    if current.version != session.payload.version || current.protocol != session.payload.protocol {
        return Err("vault_version_unsupported");
    }
    Ok(runtime)
}
fn directional_owner(payload:&VaultPayload)
    ->Result<crate::protocol_state::CapacityOwner,&'static str> {
    let layout=crate::protocol_state::approved_directional_layout()?;
    if payload.version != PAYLOAD_VERSION || payload.protocol.as_bytes() != crate::directional_delivery::INTEGRATION_PROFILE {
        return Err("directional_profile_required");
    }
    let raw=payload.secrets.get(layout.owner_key).ok_or("directional_reserve_missing")?;
    crate::protocol_state::CapacityOwner::decode(raw)
}
fn check_directional_aggregate(payload:&VaultPayload)->Result<(),&'static str> {
    let layout=crate::protocol_state::approved_directional_layout()?;
    if payload.version != PAYLOAD_VERSION {
        return Err("vault_version_unsupported");
    }
    // Reject historical and unknown directional schema keys even in ordinary mode.
    // The exact owner key and exact peer prefix are the only admitted namespaces.
    for key in payload.secrets.keys() {
        if key.starts_with("na0780_directional_")
            && key != layout.owner_key && !key.starts_with(layout.peer_prefix) {
            return Err("directional_schema_incompatible");
        }
    }
    if payload.protocol == OWNER_FREE_PROFILE {
        if payload.secrets.keys().any(|k| k == layout.owner_key || k.starts_with(layout.peer_prefix)) {
            return Err("directional_owner_binding");
        }
        return Ok(());
    }
    if payload.protocol.as_bytes() != crate::directional_delivery::INTEGRATION_PROFILE {
        return Err("vault_version_unsupported");
    }
    // Owned absence is corruption, including before the first peer. Initialization
    // creates the discriminator and empty owner in the same encrypted payload.
    let owner=directional_owner(payload)?;
    // Authenticated ownership is necessary but not sufficient: recompute stored
    // liabilities using current bytes, including on generic unrelated writes.
    for (peer,p) in &owner.peers {
        crate::directional_single_channel(peer,peer)?;
        if !crate::channel_label_ok(peer) {return Err("directional_peer_invalid");}
        let key=format!("{}{}",layout.peer_prefix,peer);
        let raw=payload.secrets.get(&key).ok_or("directional_reserve_missing")?;
        let mut state=crate::directional_delivery::Transaction::decode(raw)?;
        owner.peer(peer,&state.core.sid)?;
        if p.generation!=state.generation || p.control.generation!=p.generation || p.generation>owner.generation {
            return Err("directional_stale_generation");
        }
        state.hydrate_reserve(p.control.clone())?;
        state.verify_reservation(p)?;
    }
    for key in payload.secrets.keys().filter(|k|k.starts_with(layout.peer_prefix)) {
        let peer=key.strip_prefix(layout.peer_prefix).ok_or("directional_owner_binding")?;
        if !owner.peers.contains_key(peer) {return Err("directional_reserve_missing");}
    }
    for (ticket, entry) in &owner.entries {
        let p = owner.peers.get(&entry.peer).ok_or("directional_reserve_missing")?;
        let canonical = serde_json::to_string(&(&entry.peer,entry.sid,entry.direction,&entry.operation))
            .map_err(|_| "directional_owner_encode")?;
        if ticket != &entry.ticket || ticket != &canonical || entry.sid != p.sid
            || entry.direction > 1 || entry.state & !3 != 0 || entry.reference_state > 1
            || entry.generation > owner.generation {
            return Err("directional_owner_binding");
        }
    }
    let actual=serde_json::to_vec(payload).map_err(|_|"vault_payload_serialize_failed")?.len();
    owner.check_aggregate(actual)
}
fn guard_directional_generic_write(payload:&VaultPayload,name:&str)->Result<(),&'static str> {
    let layout=crate::protocol_state::approved_directional_layout()?;
    check_directional_aggregate(payload)?;
    if name.starts_with("na0780_directional_") {
        return Err("directional_owned_key_requires_pair");
    }
    // Unowned growth is charged by check_directional_aggregate AFTER insertion.
    // It cannot debit any owner's remaining reservation to make the write fit.
    if payload.secrets.contains_key(layout.owner_key) {
        directional_owner(payload)?.remaining_vault_bytes()?;
    }
    Ok(())
}
fn write_directional_payload(path:&Path,source:ConfigSource,env:&VaultRuntime,
    payload:&VaultPayload)->Result<(),&'static str> {
    check_directional_aggregate(payload)?;
    let plaintext=serde_json::to_vec(payload).map_err(|_|"vault_payload_serialize_failed")?;
    let ct_len = envelope_ct_len(plaintext.len())?;
    let cipher=ChaCha20Poly1305::new(Key::from_slice(&env.key));
    #[cfg(qsc_rng_failure_test_seam)]
    let nonce=vault_rng_nonce("QSC.VAULT.SESSION_PERSIST.NONCE")?;
    #[cfg(not(qsc_rng_failure_test_seam))]
    let nonce=ChaCha20Poly1305::generate_nonce(&mut OsRng);
    let aad=envelope_header_bytes(env.envelope.key_source,env.envelope.kdf_m_kib,
        env.envelope.kdf_t,env.envelope.kdf_p,ct_len,&env.envelope.salt,
        nonce.as_slice().try_into().map_err(|_|"encrypt_failed")?);
    let ciphertext=cipher.encrypt(&nonce,Payload{msg:&plaintext,aad:&aad})
        .map_err(|_|"encrypt_failed")?;
    let bytes=encode_envelope(env,nonce.as_slice(),&ciphertext);
    PERF_VAULT_ENCRYPT_WRITES.fetch_add(1,Ordering::Relaxed);
    write_atomic(path,&bytes,source).map_err(store_err_marker)?;
    VAULT_WRITE_EPOCH.fetch_add(1,Ordering::Relaxed);
    Ok(())
}

// Exactly ONE fresh payload replacement for owner+peer, no session_set pair.
// Caller supplies only the typed correction transition; projection I/O is a
// separate already-owned phase and must finish before entering this closure.
pub(crate) fn commit_directional_pair<T>(peer_key:&str,peer:&str,
    expected_owner_generation:u64,expected_peer_generation:u64,
    admission:&std::cell::Cell<bool>,
    change:impl FnOnce(&mut crate::protocol_state::CapacityOwner,
        &mut crate::directional_delivery::Transaction)->Result<T,&'static str>)
    ->Result<T,&'static str> {
    let layout=crate::protocol_state::approved_directional_layout()?;
    if peer_key!=format!("{}{}",layout.peer_prefix,peer) || peer_key==layout.owner_key {
        return Err("directional_owner_binding");
    }
    let (dir,path,source)=vault_path_resolved()?;
    let _lock=lock_store_exclusive(&dir,source).map_err(store_err_marker)?;
    let mut env=read_directional_runtime(&path,None)?;
    let mut latest=decrypt_payload(&env)?;
    let mut owner=directional_owner(&latest)?;
    let raw=latest.secrets.get(peer_key).ok_or("directional_reserve_missing")?;
    let mut state=crate::directional_delivery::Transaction::decode(raw)?;
    let p=owner.peer(peer,&state.core.sid)?;
    if owner.generation!=expected_owner_generation || state.generation!=expected_peer_generation
        || p.generation!=state.generation {return Err("directional_stale_generation");}
    state.hydrate_reserve(p.control.clone())?;
    state.verify_reservation(p)?;
    let sid=state.core.sid;
    let result=change(&mut owner,&mut state).map_err(|e| {
        // Only the typed staging/accounting decision sets this provenance flag.
        // Decode, lock, encode and physical-write errors never set it.
        if e=="TRANSACTION_CAPACITY" {admission.set(true);}
        e
    })?;
    if state.core.sid!=sid {return Err("directional_session_replacement_refused");}
    // Generation advances for projection/cause changes too, not only core traffic.
    state.generation=expected_peer_generation.checked_add(1).ok_or("GENERATION_OVERFLOW")?;
    owner.generation=expected_owner_generation.checked_add(1).ok_or("GENERATION_OVERFLOW")?;
    let p=owner.peers.get_mut(peer).ok_or("directional_reserve_missing")?;
    if p.sid!=sid || p.peer!=peer {return Err("directional_owner_binding");}
    p.generation=state.generation;
    state.refresh_reservation(p).map_err(|e| {
        if e=="TRANSACTION_CAPACITY" {admission.set(true);} e
    })?;
    p.control.validate()?;
    latest.secrets.insert(peer_key.to_owned(),state.encode()?);
    latest.secrets.insert(layout.owner_key.to_owned(),
        serde_json::to_string(&owner).map_err(|_|"directional_owner_encode")?);
    // Encoded owner, peer and unrelated content all appear in actual bytes here.
    // Only unmaterialized liabilities are added; no retained-byte double charge.
    check_directional_aggregate(&latest).map_err(|e| {
        if e=="directional_aggregate_waiting" {admission.set(true);} e
    })?;
    write_directional_payload(&path,source,&env,&latest)?;
    env.key.zeroize();
    // Nothing external (wire/receipt/ACK) may use result before this returns.
    Ok(result)
}

// A funded ordinary timeline projection changes only the named existing timeline
// key and its own credit, with one atomic owner+projection replacement. Peer state
// remains authoritative until a later fresh pair commit retires its event.
pub(crate) fn project_owned_secret(ticket:&str,expected_owner_generation:u64,
    expected_entry_generation:u64,expected_timeline:Option<&str>,new_timeline:&str)
    ->Result<(),&'static str> {
    let layout=crate::protocol_state::approved_directional_layout()?;
    let (dir,path,source)=vault_path_resolved()?;
    let _lock=lock_store_exclusive(&dir,source).map_err(store_err_marker)?;
    let mut env=read_directional_runtime(&path,None)?;
    let mut latest=decrypt_payload(&env)?;
    let mut owner=directional_owner(&latest)?;
    if owner.generation!=expected_owner_generation
        || latest.secrets.get(crate::store::TIMELINE_SECRET_KEY).map(String::as_str)!=expected_timeline {
        return Err("directional_stale_generation");
    }
    let before=serde_json::to_vec(&latest).map_err(|_|"vault_payload_serialize_failed")?.len();
    latest.secrets.insert(crate::store::TIMELINE_SECRET_KEY.to_owned(),new_timeline.to_owned());
    let after=serde_json::to_vec(&latest).map_err(|_|"vault_payload_serialize_failed")?.len();
    let growth=after.saturating_sub(before) as u64;
    let entry=owner.entries.get_mut(ticket).ok_or("directional_reserve_missing")?;
    if entry.generation!=expected_entry_generation || growth>entry.projection {
        return Err("directional_projection_credit");
    }
    entry.projection-=growth;
    entry.charge.vault_bytes=entry.charge.vault_bytes.checked_sub(growth)
        .ok_or("directional_owner_invariant")?;
    entry.generation=entry.generation.checked_add(1).ok_or("GENERATION_OVERFLOW")?;
    owner.generation=owner.generation.checked_add(1).ok_or("GENERATION_OVERFLOW")?;
    latest.secrets.insert(layout.owner_key.to_owned(),
        serde_json::to_string(&owner).map_err(|_|"directional_owner_encode")?);
    // Counter-width/metadata growth is included, never assumed free.
    check_directional_aggregate(&latest)?;
    write_directional_payload(&path,source,&env,&latest)?;
    env.key.zeroize();
    Ok(())
}

// Create-only counterpart. Neither old peer state nor missing reservation is
// backfilled. The approved future-layout gate precedes every read or mutation.
pub(crate) fn create_directional_pair(peer_key:&str,peer:&str,
    mut state:crate::directional_delivery::Transaction)->Result<(),&'static str> {
    use crate::protocol_state::{PeerReserve,SessionControlReserve};
    let layout=crate::protocol_state::approved_directional_layout()?;
    if peer_key!=format!("{}{}",layout.peer_prefix,peer) {return Err("directional_owner_binding");}
    let (dir,path,source)=vault_path_resolved()?;
    let _lock=lock_store_exclusive(&dir,source).map_err(store_err_marker)?;
    let mut env=read_directional_runtime(&path,None)?;
    let mut latest=decrypt_payload(&env)?;
    if latest.secrets.contains_key(peer_key) {return Err("directional_session_replacement_refused");}
    let mut owner=directional_owner(&latest)?;
    if owner.peers.contains_key(peer) {return Err("directional_reserve_missing");}
    let control=SessionControlReserve::fresh(state.core.sid);
    state.hydrate_reserve(control.clone())?;
    // Hypothesis under review, not permission to increase if the derivation fails.
    let control_bound=36_775u64;
    let core_future=state.core_context_future()?;
    let future=control_bound.checked_sub(state.control_retained_encoded()? as u64)
        .and_then(|n|n.checked_add(core_future)).ok_or("TRANSACTION_CAPACITY")?;
    owner.peers.insert(peer.to_owned(),PeerReserve{peer:peer.to_owned(),sid:state.core.sid,
        generation:state.generation,control_bound,peer_future:future,
        // Transient initializer only; refreshed before any encode/save.
        vault_future:0,control});
    state.refresh_reservation(owner.peers.get_mut(peer).ok_or("directional_reserve_missing")?)?;
    owner.generation=owner.generation.checked_add(1).ok_or("GENERATION_OVERFLOW")?;
    state.check_reserved_cost(future)?;
    latest.secrets.insert(peer_key.to_owned(),state.encode()?);
    latest.secrets.insert(layout.owner_key.to_owned(),serde_json::to_string(&owner)
        .map_err(|_|"directional_owner_encode")?);
    check_directional_aggregate(&latest)?;
    write_directional_payload(&path,source,&env,&latest)?;
    env.key.zeroize();Ok(())
}


#[cfg(test)]
mod r02_layout_tests {
    use super::*;

    // Parser/empty-layout negative controls only: no protocol keys, peer roots,
    // receipts, clock forcing, or writes are synthesized by these tests.
    #[test]
    fn r02_fresh_discriminator_and_owner_are_one_payload() {
        let layout=crate::protocol_state::approved_directional_layout().unwrap();
        let owned=VaultPayload::empty(true).unwrap();
        let encoded=serde_json::to_vec(&owned).unwrap();
        let loaded:VaultPayload=serde_json::from_slice(&encoded).unwrap();
        assert_eq!(loaded.version,4);
        assert_eq!(loaded.protocol.as_bytes(),crate::directional_delivery::INTEGRATION_PROFILE);
        assert_eq!(check_directional_aggregate(&loaded),Ok(()));
        let owner=directional_owner(&loaded).unwrap();
        assert_eq!(owner.generation,0);assert!(owner.peers.is_empty());assert!(owner.entries.is_empty());
        let mut missing=loaded.clone();missing.secrets.remove(layout.owner_key);
        assert_eq!(check_directional_aggregate(&missing),Err("directional_reserve_missing"));
        let ordinary=VaultPayload::empty(false).unwrap();
        assert_eq!(ordinary.protocol,OWNER_FREE_PROFILE);
        assert!(ordinary.secrets.is_empty());assert_eq!(check_directional_aggregate(&ordinary),Ok(()));
        let mut mixed=ordinary.clone();mixed.secrets=owned.secrets.clone();
        assert_eq!(check_directional_aggregate(&mixed),Err("directional_owner_binding"));
        let mut corrupt=owned.clone();corrupt.secrets.insert(layout.owner_key.to_owned(),"{".to_owned());
        assert_eq!(check_directional_aggregate(&corrupt),Err("directional_owner_tampered"));
        for version in [0,1,2,3,5,u8::MAX] {
            let mut old=owned.clone();old.version=version;
            assert_eq!(check_directional_aggregate(&old),Err("vault_version_unsupported"));
        }
        for profile in ["","NA0780-DIR-INTEGRATION-01","NA0780-DIR-INTEGRATION-02","unknown"] {
            let mut old=owned.clone();old.protocol=profile.to_owned();
            assert_eq!(check_directional_aggregate(&old),Err("vault_version_unsupported"));
        }
        for key in ["na0780_directional_transaction/bob","na0780_directional_owner_unknown"] {
            let mut unknown=ordinary.clone();unknown.secrets.insert(key.to_owned(),"{}".to_owned());
            assert_eq!(check_directional_aggregate(&unknown),Err("directional_schema_incompatible"));
        }
        let mut orphan=owned;orphan.secrets.insert(format!("{}bob",layout.peer_prefix),"{}".to_owned());
        assert_eq!(check_directional_aggregate(&orphan),Err("directional_reserve_missing"));
    }

    #[test]
    fn r02_payload_parser_rejects_ambiguous_or_missing_fields() {
        for raw in [
            r#"{"version":4,"secrets":{}}"#,
            r#"{"version":4,"protocol":"NA0780-OWNER-FREE-01","secrets":{},"extra":0}"#,
            r#"{"version":4,"protocol":"NA0780-OWNER-FREE-01","secrets":{"name":"first","name":"second"}}"#,
            r#"{"version":4,"version":3,"protocol":"NA0780-OWNER-FREE-01","secrets":{}}"#,
        ] { assert!(serde_json::from_str::<VaultPayload>(raw).is_err()); }
    }
}

// NA-0788 F04/S3b: the one bounded vault reader on every read path (N1), the envelope's exact
// length through a real vault (N2), the owner-free window (R4), the pins of the old code's
// callers (A2), the keychain hex length (N5) and the checked ct_len (D29). A path test runs in an
// isolated child process (the S1 pattern) so it owns QSC_CONFIG_DIR and the process passphrase.
#[cfg(test)]
mod f04_s3b_bounds_tests {
    use super::*;
    use sha2::{Digest, Sha256};
    use std::io;

    const CHILD: &str = "QSC_F04_S3B_CHILD";
    const OVERSIZED: &str = crate::freshness::codes::VAULT_FILE_OVERSIZED;
    /// What the bytes-read instrument may add on top of cap + 1 (its own /proc reads).
    #[cfg(target_os = "linux")]
    const SLACK: u64 = 65_536;

    /// Runs the named test again in a child process; true only inside that child.
    fn in_child(name: &str) -> bool {
        if std::env::var_os(CHILD).is_some() {
            return true;
        }
        let status = std::process::Command::new(std::env::current_exe().unwrap())
            .args(["--exact", &format!("vault::f04_s3b_bounds_tests::{name}")])
            .args(["--nocapture", "--test-threads=1"])
            .env(CHILD, "1")
            .status()
            .unwrap();
        assert!(status.success(), "isolated fixture {name} failed");
        false
    }

    struct Fixture {
        _dir: tempfile::TempDir,
        path: PathBuf,
        pass: String,
    }

    /// A fresh 0700 config directory for this child and a random passphrase, set for the process.
    fn fixture() -> Fixture {
        let dir = tempfile::tempdir().unwrap();
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(dir.path(), fs::Permissions::from_mode(0o700)).unwrap();
        }
        std::env::set_var("QSC_CONFIG_DIR", dir.path());
        let mut raw = [0u8; 16];
        OsRng.fill_bytes(&mut raw);
        let pass: String = raw.iter().map(|b| format!("{b:02x}")).collect();
        set_process_passphrase(Some(&pass));
        let path = dir.path().join("vault.qsv");
        Fixture {
            _dir: dir,
            path,
            pass,
        }
    }

    fn write_file(path: &Path, bytes: &[u8]) {
        fs::write(path, bytes).unwrap();
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(path, fs::Permissions::from_mode(0o600)).unwrap();
        }
    }

    /// A REAL owner-free vault, sealed through the product's own header and envelope builders
    /// under an Argon2 key from the fixture's passphrase, whose file is exactly `file_len` bytes.
    fn owner_free_vault(fx: &Fixture, file_len: usize) -> Vec<u8> {
        let mut payload = VaultPayload::empty(false).unwrap();
        payload
            .secrets
            .insert("f04_s3b_filler".into(), String::new());
        let base = serde_json::to_vec(&payload).unwrap().len();
        let plain_len = file_len - HEADER_LEN - 16;
        payload
            .secrets
            .insert("f04_s3b_filler".into(), "f".repeat(plain_len - base));
        let plaintext = serde_json::to_vec(&payload).unwrap();
        assert_eq!(plaintext.len(), plain_len);
        let mut salt = [0u8; 16];
        OsRng.fill_bytes(&mut salt);
        let mut nonce = [0u8; 12];
        OsRng.fill_bytes(&mut nonce);
        let mut runtime = VaultRuntime {
            envelope: VaultRuntimeEnvelope {
                key_source: 1,
                salt,
                kdf_m_kib: KDF_M_KIB,
                kdf_t: KDF_T,
                kdf_p: KDF_P,
                ciphertext: Vec::new(),
            },
            key: [0u8; 32],
        };
        derive_runtime_key(&runtime.envelope, &mut runtime.key, Some(&fx.pass)).unwrap();
        let ct_len = envelope_ct_len(plain_len).unwrap();
        let aad = envelope_header_bytes(1, KDF_M_KIB, KDF_T, KDF_P, ct_len, &salt, &nonce);
        let sealed = Payload {
            msg: &plaintext,
            aad: &aad,
        };
        let ciphertext = ChaCha20Poly1305::new(Key::from_slice(&runtime.key))
            .encrypt(Nonce::from_slice(&nonce), sealed)
            .unwrap();
        let bytes = encode_envelope(&runtime, &nonce, &ciphertext);
        assert_eq!(bytes.len(), file_len);
        write_file(&fx.path, &bytes);
        bytes
    }

    /// Bytes this thread has read so far: `rchar` of /proc/thread-self/io.
    #[cfg(target_os = "linux")]
    fn rchar() -> u64 {
        let io = fs::read_to_string("/proc/thread-self/io").unwrap();
        let line = io.lines().find_map(|l| l.strip_prefix("rchar:")).unwrap();
        line.trim().parse().unwrap()
    }

    /// `read` on a SPARSE file four times the cap refuses as oversized and, on Linux, this thread
    /// read at most cap + 1 bytes (plus the instrument's slack); an unbounded read reads 4 x cap.
    fn assert_bounded(fx: &Fixture, read: impl FnOnce() -> Option<String>) {
        let file = fs::File::create(&fx.path).unwrap();
        file.set_len(4 * VAULT_FILE_CAP as u64).unwrap();
        drop(file);
        #[cfg(target_os = "linux")]
        let before = rchar();
        let refusal = read();
        #[cfg(target_os = "linux")]
        {
            let read = rchar() - before;
            assert!(
                read <= VAULT_FILE_CAP as u64 + 1 + SLACK,
                "read {read} bytes"
            );
        }
        assert_eq!(refusal.as_deref(), Some(OVERSIZED));
    }

    fn err<T>(result: Result<T, &'static str>) -> Option<String> {
        result.err().map(String::from)
    }

    fn status() -> Option<String> {
        match vault_status() {
            Ok(()) => None,
            Err(CliError::Code(code)) => Some(code),
            Err(CliError::Emitted) => Some("emitted".into()),
        }
    }

    struct Counting<R>(R, u64);
    impl<R: Read> Read for Counting<R> {
        fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
            let n = self.0.read(buf)?;
            self.1 += n as u64;
            Ok(n)
        }
    }

    #[test]
    fn t_n1_one_reader_reads_at_most_cap_plus_one() {
        assert_eq!(VAULT_FILE_CAP, 16_777_285);
        let cap = VAULT_FILE_CAP as u64;
        let mut endless = Counting(io::repeat(0x5a), 0);
        assert_eq!(
            read_vault_limited(&mut endless),
            Err(VaultFileError::Oversized)
        );
        assert_eq!(endless.1, cap + 1);
        let mut exact = Counting(io::repeat(0x5a).take(cap), 0);
        let bytes = read_vault_limited(&mut exact).unwrap();
        assert_eq!((bytes.len(), exact.1), (VAULT_FILE_CAP, cap));
        let mut over = Counting(io::repeat(0x5a).take(cap + 1), 0);
        assert_eq!(
            read_vault_limited(&mut over),
            Err(VaultFileError::Oversized)
        );
        assert_eq!(over.1, cap + 1);
    }

    #[test]
    fn t_n1_r01_unlock_read_bounded() {
        if !in_child("t_n1_r01_unlock_read_bounded") {
            return;
        }
        let fx = fixture();
        owner_free_vault(&fx, VAULT_FILE_CAP);
        let (_, runtime) = load_vault_runtime_with_passphrase(Some(&fx.pass)).unwrap();
        assert!(
            decrypt_payload(&runtime).is_ok(),
            "at the cap: read and opened"
        );
        let over = owner_free_vault(&fx, VAULT_FILE_CAP + 1);
        let load = || err(load_vault_runtime_with_passphrase(Some(&fx.pass)));
        assert_eq!(load().as_deref(), Some(OVERSIZED));
        assert_eq!(fs::read(&fx.path).unwrap(), over);
        assert_bounded(&fx, load);
    }

    #[test]
    fn t_n1_r04_retain_ownership_read_bounded() {
        if !in_child("t_n1_r04_retain_ownership_read_bounded") {
            return;
        }
        let fx = fixture();
        owner_free_vault(&fx, VAULT_FILE_CAP);
        let mut session = open_session_with_passphrase(&fx.pass).unwrap();
        assert_eq!(retain_ownership_in_session(&mut session, None), Ok(()));
        let over = owner_free_vault(&fx, VAULT_FILE_CAP + 1);
        assert_eq!(
            retain_ownership_in_session(&mut session, None),
            Err(OVERSIZED)
        );
        assert_eq!(fs::read(&fx.path).unwrap(), over);
        assert_bounded(&fx, || err(retain_ownership_in_session(&mut session, None)));
    }

    #[test]
    fn t_n1_r05_persist_read_bounded() {
        if !in_child("t_n1_r05_persist_read_bounded") {
            return;
        }
        let fx = fixture();
        owner_free_vault(&fx, VAULT_FILE_CAP);
        let mut session = open_session_with_passphrase(&fx.pass).unwrap();
        assert_eq!(persist_session_with_ownership(&mut session, None), Ok(()));
        let over = owner_free_vault(&fx, VAULT_FILE_CAP + 1);
        assert_eq!(
            persist_session_with_ownership(&mut session, None),
            Err(OVERSIZED)
        );
        assert_eq!(fs::read(&fx.path).unwrap(), over);
        assert_bounded(&fx, || {
            err(persist_session_with_ownership(&mut session, None))
        });
    }

    #[test]
    fn t_n1_r06_owner_free_session_read_bounded() {
        if !in_child("t_n1_r06_owner_free_session_read_bounded") {
            return;
        }
        let fx = fixture();
        owner_free_vault(&fx, VAULT_FILE_CAP);
        let session = open_session_with_passphrase(&fx.pass).unwrap();
        assert_eq!(session.payload.protocol, OWNER_FREE_PROFILE);
        assert!(read_session_runtime(&session).is_ok(), "at the cap");
        let over = owner_free_vault(&fx, VAULT_FILE_CAP + 1);
        let read = || err(read_session_runtime(&session));
        assert_eq!(read().as_deref(), Some(OVERSIZED));
        assert_eq!(fs::read(&fx.path).unwrap(), over);
        assert_bounded(&fx, read);
    }

    #[test]
    fn t_n1_r07_directional_read_bounded() {
        if !in_child("t_n1_r07_directional_read_bounded") {
            return;
        }
        let fx = fixture();
        owner_free_vault(&fx, VAULT_FILE_CAP);
        let read = || err(read_directional_runtime(&fx.path, Some([0u8; 32])));
        assert_eq!(read(), None, "at the cap");
        let over = owner_free_vault(&fx, VAULT_FILE_CAP + 1);
        assert_eq!(read().as_deref(), Some(OVERSIZED));
        assert_eq!(fs::read(&fx.path).unwrap(), over);
        assert_bounded(&fx, read);
        let absent = fx.path.with_extension("absent");
        assert_eq!(
            err(read_directional_runtime(&absent, Some([0u8; 32]))).as_deref(),
            Some("vault_missing")
        );
    }

    #[test]
    fn t_n1_r10_status_read_bounded() {
        if !in_child("t_n1_r10_status_read_bounded") {
            return;
        }
        let fx = fixture();
        owner_free_vault(&fx, VAULT_FILE_CAP);
        assert_eq!(status(), None, "at the cap");
        let over = owner_free_vault(&fx, VAULT_FILE_CAP + 1);
        assert_eq!(status().as_deref(), Some(OVERSIZED));
        assert_eq!(fs::read(&fx.path).unwrap(), over);
        assert_bounded(&fx, status);
    }

    #[test]
    fn t_n1_r11_destroy_peek_read_bounded() {
        if !in_child("t_n1_r11_destroy_peek_read_bounded") {
            return;
        }
        let fx = fixture();
        let destroy = || {
            let token = protection::DestroyConfirmToken::confirm(&fx.pass);
            err(protection::destroy_with_passphrase(&fx.pass, token))
        };
        let over = owner_free_vault(&fx, VAULT_FILE_CAP + 1);
        assert_eq!(destroy().as_deref(), Some(OVERSIZED));
        assert_eq!(fs::read(&fx.path).unwrap(), over, "nothing destroyed");
        assert_bounded(&fx, destroy);
        assert!(fx.path.exists(), "nothing destroyed");
        owner_free_vault(&fx, VAULT_FILE_CAP);
        assert_eq!(destroy(), None, "at the cap: read, then destroyed");
        assert!(!fx.path.exists());
    }

    #[test]
    fn t_n2_real_vault_trailing_byte_refused_on_unlock() {
        if !in_child("t_n2_real_vault_trailing_byte_refused_on_unlock") {
            return;
        }
        let fx = fixture();
        let exact = owner_free_vault(&fx, 1024);
        let (_, runtime) = load_vault_runtime_with_passphrase(Some(&fx.pass)).unwrap();
        assert!(decrypt_payload(&runtime).is_ok(), "exact length: opened");
        let mut tail = exact;
        tail.push(0);
        write_file(&fx.path, &tail);
        assert_eq!(
            err(load_vault_runtime_with_passphrase(Some(&fx.pass))).as_deref(),
            Some("vault_parse_failed")
        );
        assert_eq!(fs::read(&fx.path).unwrap(), tail);
    }

    // RULING_NA0788_S3_stop R4: an owner-free vault has no write-side cap, so one past the read
    // cap can exist; it is refused loudly on read and left exactly as it was.
    #[test]
    fn t_a4_owner_free_oversized_vault_refused_and_unchanged() {
        if !in_child("t_a4_owner_free_oversized_vault_refused_and_unchanged") {
            return;
        }
        let fx = fixture();
        let digest = Sha256::digest(owner_free_vault(&fx, VAULT_FILE_CAP + 1));
        assert_eq!(secret_get("f04_s3b_probe"), Err(OVERSIZED));
        assert_eq!(secret_set("f04_s3b_probe", "v"), Err(OVERSIZED));
        assert_eq!(Sha256::digest(fs::read(&fx.path).unwrap()), digest);
    }

    // A2: the old code's matchers (vault/mod.rs commit_directional_pair's admission wrapper and
    // the two saturation searches in tests/na0780_directional_integration.rs) match the
    // aggregate check's code, which is unchanged; R-07 keeps it for a path that is not a file.
    #[test]
    fn t_a2_pin_old_code_callers() {
        let mut owned = VaultPayload::empty(true).unwrap();
        assert_eq!(check_directional_aggregate(&owned), Ok(()));
        let filler = "f".repeat(crate::protocol_state::REVIEW_AGGREGATE_CANDIDATE);
        owned.secrets.insert("f04_s3b_filler".into(), filler);
        assert_eq!(
            check_directional_aggregate(&owned),
            Err("directional_aggregate_waiting")
        );
        let dir = tempfile::tempdir().unwrap();
        assert_eq!(
            err(read_directional_runtime(dir.path(), Some([0u8; 32]))).as_deref(),
            Some("directional_aggregate_waiting")
        );
    }

    #[test]
    fn t_n5_keychain_hex_length_checked_before_decode() {
        let key: [u8; 32] = std::array::from_fn(|i| (i * 7 + 1) as u8);
        let hex: String = key.iter().map(|b| format!("{b:02x}")).collect();
        let mut out = [0u8; 32];
        assert!(key_from_hex(&hex, &mut out));
        assert_eq!(out, key);
        let untouched = [0xa5u8; 32];
        let mut non_hex = hex.clone();
        non_hex.replace_range(10..11, "g");
        let (extra, long) = (format!("{hex}00"), "0".repeat(2_097_152));
        let wrong = [
            &hex[..62],
            &hex[..63],
            extra.as_str(),
            long.as_str(),
            non_hex.as_str(),
        ];
        for secret in wrong {
            let mut out = untouched;
            assert!(!key_from_hex(secret, &mut out), "{} chars", secret.len());
            assert_eq!(out, untouched);
        }
    }

    #[test]
    fn t_d29_ct_len_checked_not_truncated() {
        assert_eq!(envelope_ct_len(0), Ok(16));
        assert_eq!(envelope_ct_len(u32::MAX as usize - 16), Ok(u32::MAX));
        for len in [u32::MAX as usize - 15, 1usize << 32, usize::MAX] {
            assert_eq!(envelope_ct_len(len), Err("directional_capacity_overflow"));
        }
    }
}

// NA-0788 F04/S4b N5 MV-1 (RULING_NA0788_S4_stop R4): the largest directional vault the writer admits
// -- actual + promises + H_W == B_V exactly -- is written by the real writer and reopens byte-equal
// through the one bounded reader; one byte more is refused BY THE WRITER with its measured code
// (directional_aggregate_waiting, S3b-pinned) and never reaches disk. Isolated in a child process: it
// owns QSC_CONFIG_DIR and the process passphrase. The build time is printed for the record.
#[cfg(test)]
mod f04_s4b_mv1_tests {
    use super::*;
    use sha2::{Digest, Sha256};
    use std::time::Instant;

    const CHILD: &str = "QSC_F04_S4B_CHILD";
    const FILLER: &str = "f04_s4b_filler";

    #[test]
    fn t_n5_mv1_directional_vault_at_the_bound_reopens() {
        if std::env::var_os(CHILD).is_none() {
            let name = "vault::f04_s4b_mv1_tests::t_n5_mv1_directional_vault_at_the_bound_reopens";
            let status = std::process::Command::new(std::env::current_exe().unwrap())
                .args(["--exact", name, "--nocapture", "--test-threads=1"])
                .env(CHILD, "1")
                .status()
                .unwrap();
            assert!(status.success(), "isolated MV-1 fixture failed");
            return;
        }
        let started = Instant::now();
        let dir = tempfile::tempdir().unwrap();
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(dir.path(), fs::Permissions::from_mode(0o700)).unwrap();
        }
        std::env::set_var("QSC_CONFIG_DIR", dir.path());
        let mut raw = [0u8; 16];
        OsRng.fill_bytes(&mut raw);
        let pass: String = raw.iter().map(|b| format!("{b:02x}")).collect();
        set_process_passphrase(Some(&pass));
        let path = dir.path().join("vault.qsv");
        let digest = || Sha256::digest(fs::read(&path).unwrap());
        vault_init_directional_with_passphrase(&pass).unwrap();
        let mut session = open_session_with_passphrase(&pass).unwrap();
        let owner = directional_owner(&session.payload).unwrap();
        let promised = owner.remaining_vault_bytes().unwrap();
        let mut probe = session.payload.clone();
        probe.secrets.insert(FILLER.into(), String::new());
        let base = serde_json::to_vec(&probe).unwrap().len();
        let b_v = crate::protocol_state::REVIEW_AGGREGATE_CANDIDATE;
        let h_w = crate::protocol_state::REVIEW_WRITE_HEADROOM;
        let fill = b_v - h_w - promised - base;
        assert_eq!(session_set(&mut session, FILLER, &"f".repeat(fill)), Ok(()));
        let written = serde_json::to_vec(&session.payload).unwrap();
        assert_eq!(written.len() + promised + h_w, b_v, "exactly at the bound");
        let at_bound = digest();
        let reopened = open_session_with_passphrase(&pass).unwrap();
        assert_eq!(
            serde_json::to_vec(&reopened.payload).unwrap(),
            written,
            "reopened"
        );
        assert_eq!(digest(), at_bound);
        assert_eq!(
            session_set(&mut session, FILLER, &"f".repeat(fill + 1)),
            Err("directional_aggregate_waiting")
        );
        assert_eq!(digest(), at_bound, "the refused write never reached disk");
        let again = open_session_with_passphrase(&pass).unwrap();
        assert_eq!(
            serde_json::to_vec(&again.payload).unwrap(),
            written,
            "after"
        );
        println!(
            "F04S4B_FIXTURE t_n5_mv1_directional_vault_at_the_bound_reopens ms={}",
            started.elapsed().as_millis()
        );
    }
}

// NA-0788 F04/S6: payload v5 and the opener (dead code until S7), tested through the opener over
// real QSCV04 blobs the new code builds, and through the decoder directly for the typed kinds.
#[cfg(test)]
mod f04_s6_v5_tests {
    use super::*;
    use crate::adversarial::vault_format::VAULT_MAGIC_V4;
    use crate::freshness::test_support::{syn_mac_key, syn_prev, syn_vault_id, SYN_GENERATION};
    use crate::freshness::{LineageOpen, ProtectionMode};
    use serde_json::Value;
    use std::time::Instant;

    const U: OpenError = OpenError::Unauthenticated;
    const M: OpenError = OpenError::Malformed;
    /// A neutral protocol value: S6 does not check it (its checks are S7's).
    const PROTOCOL: &str = "protocol-unchecked-at-s6";

    fn pass() -> Zeroizing<String> {
        Zeroizing::new("f04-s6-opener-fixture".to_owned())
    }
    fn opener() -> PassphraseOpener {
        PassphraseOpener::new(pass())
    }
    fn open(blob: &[u8]) -> Result<LineageFields, OpenError> {
        opener().open(blob)
    }
    fn nv_auth() -> [u8; 32] {
        std::array::from_fn(|i| 0x80 ^ i as u8)
    }
    fn local_payload() -> VaultPayloadV5 {
        VaultPayloadV5 {
            version: 5,
            protocol: PROTOCOL.to_owned(),
            mode: VaultMode::Messaging,
            protection_mode: ProtectionMode::LocalCheckpoint,
            vault_id: syn_vault_id(),
            generation: SYN_GENERATION,
            predecessor_anchor: syn_prev(),
            checkpoint_mac_key: Zeroizing::new(syn_mac_key()),
            tpm_enrollment: None,
            secrets: BTreeMap::from([("k".to_owned(), "v".to_owned())]),
        }
    }
    fn tpm_payload() -> VaultPayloadV5 {
        VaultPayloadV5 {
            mode: VaultMode::StorageOnly,
            protection_mode: ProtectionMode::Tpm,
            tpm_enrollment: Some(TpmEnrollmentV5 {
                nv_public: [0x11; 16],
                nv_auth: Zeroizing::new(nv_auth()),
                primary_template: PrimaryTemplate::QslSrkEccP256V1,
                primary_name: [0x22; 34],
            }),
            ..local_payload()
        }
    }
    fn text(payload: &VaultPayloadV5) -> Vec<u8> {
        serde_json::to_vec(payload).unwrap()
    }
    /// The text with one member replaced (or added) at a JSON pointer; members re-emitted in
    /// serde_json's (sorted) order, so every case below also exercises order independence.
    fn set(text: &[u8], pointer: &str, to: Value) -> Vec<u8> {
        let mut v: Value = serde_json::from_slice(text).unwrap();
        *v.pointer_mut(pointer).unwrap() = to;
        serde_json::to_vec(&v).unwrap()
    }
    fn add(text: &[u8], pointer: &str, name: &str, to: Value) -> Vec<u8> {
        let mut v: Value = serde_json::from_slice(text).unwrap();
        v.pointer_mut(pointer).unwrap()[name] = to;
        serde_json::to_vec(&v).unwrap()
    }
    fn without(text: &[u8], name: &str) -> Vec<u8> {
        let mut v: Value = serde_json::from_slice(text).unwrap();
        v.as_object_mut().unwrap().remove(name).expect(name);
        serde_json::to_vec(&v).unwrap()
    }
    /// The member `name` repeated: its `"name":value` text spliced in once more, after itself.
    fn repeated(text: &[u8], name: &str, value: &str) -> Vec<u8> {
        let s = std::str::from_utf8(text).unwrap();
        let member = format!("\"{name}\":");
        let at = s.find(&member).expect(name);
        let mut out = s[..at].to_owned();
        out.push_str(&format!("{member}{value},"));
        out.push_str(&s[at..]);
        out.into_bytes()
    }
    fn b64(bytes: &[u8]) -> String {
        use base64::Engine as _;
        base64::engine::general_purpose::STANDARD.encode(bytes)
    }
    fn typed(text: &[u8]) -> Result<(), PayloadV5Error> {
        decode_payload_v5(text).map(|_| ())
    }

    struct Fixture {
        salt: [u8; 16],
        nonce: [u8; 12],
        key: Zeroizing<[u8; 32]>,
    }
    /// ONE Argon2id derivation per fixture, by the unlock path's own derivation.
    fn fixture() -> Fixture {
        let salt: [u8; 16] = std::array::from_fn(|i| 0x30 + i as u8);
        let nonce: [u8; 12] = std::array::from_fn(|i| 0x50 + i as u8);
        let envelope = VaultRuntimeEnvelope {
            key_source: 1,
            salt,
            kdf_m_kib: KDF_M_KIB,
            kdf_t: KDF_T,
            kdf_p: KDF_P,
            ciphertext: Vec::new(),
        };
        let mut key = Zeroizing::new([0u8; 32]);
        derive_runtime_key(&envelope, &mut key, Some(&pass())).unwrap();
        Fixture { salt, nonce, key }
    }
    impl Fixture {
        /// The existing serializer's 53 header bytes with the magic swapped to QSCV04: exactly what
        /// `envelope_header_bytes` writes once VAULT_MAGIC is QSCV04 (S7).
        fn header(&self, magic: &[u8; 6], key_source: u8, ct_len: usize) -> Vec<u8> {
            let mut h = envelope_header_bytes(
                key_source,
                KDF_M_KIB,
                KDF_T,
                KDF_P,
                u32::try_from(ct_len).unwrap(),
                &self.salt,
                &self.nonce,
            );
            h[..6].copy_from_slice(magic);
            h
        }
        /// A blob: `header` then the plaintext sealed by the existing AEAD under `aad`.
        fn blob_with(&self, header: Vec<u8>, aad: &[u8], plaintext: &[u8]) -> Vec<u8> {
            let ct = ChaCha20Poly1305::new(Key::from_slice(self.key.as_slice()))
                .encrypt(
                    Nonce::from_slice(&self.nonce),
                    Payload {
                        msg: plaintext,
                        aad,
                    },
                )
                .unwrap();
            let mut blob = header;
            blob.extend_from_slice(&ct);
            blob
        }
        /// A well-formed QSCV04 blob over `plaintext`: the AAD is the header, as the opener expects.
        fn blob(&self, plaintext: &[u8]) -> Vec<u8> {
            let header = self.header(VAULT_MAGIC_V4, 1, plaintext.len() + 16);
            self.blob_with(header.clone(), &header, plaintext)
        }
    }

    fn assert_fields(fields: &LineageFields, mode: ProtectionMode) {
        assert_eq!(fields.vault_id, syn_vault_id());
        assert_eq!(fields.generation, SYN_GENERATION);
        assert_eq!(fields.predecessor_anchor, syn_prev());
        assert_eq!(fields.checkpoint_mac_key(), &syn_mac_key());
        assert_eq!(fields.protection_mode, mode);
    }

    /// A valid local-checkpoint blob round-trips through the opener to the exact LineageFields, and
    /// the writer's text decodes back and re-serializes byte for byte.
    #[test]
    fn t_s6_round_trip_local_checkpoint_returns_the_exact_lineage_fields() {
        let fx = fixture();
        let written = text(&local_payload());
        let fields = open(&fx.blob(&written)).unwrap();
        assert_fields(&fields, ProtectionMode::LocalCheckpoint);
        let again = decode_payload_v5(&written).unwrap();
        assert_eq!(text(&again), written, "byte-exact round trip");
        assert_eq!(again.mode, VaultMode::Messaging);
        assert_eq!(again.protocol, PROTOCOL);
        assert_eq!(
            again.secrets,
            BTreeMap::from([("k".to_owned(), "v".to_owned())])
        );
    }

    /// A valid tpm blob round-trips (F6's object arm), and every defect of the enrollment object
    /// is Malformed: an unknown member, an absent member, wrong lengths, another template.
    #[test]
    fn t_s6_round_trip_tpm_payload_and_its_enrollment_object() {
        let fx = fixture();
        let written = text(&tpm_payload());
        let fields = open(&fx.blob(&written)).unwrap();
        assert_fields(&fields, ProtectionMode::Tpm);
        let tpm = &decode_payload_v5(&written).unwrap().tpm_enrollment;
        let tpm = tpm.as_ref().unwrap();
        assert_eq!(tpm.nv_public, [0x11; 16]);
        assert_eq!(*tpm.nv_auth, nv_auth());
        assert_eq!(tpm.primary_name, [0x22; 34]);
        let obj = "/tpm_enrollment";
        let cases: Vec<(&str, Vec<u8>)> = vec![
            (
                "unknown member",
                add(&written, obj, "extra", Value::from(1)),
            ),
            ("nv_public absent", {
                let mut v: Value = serde_json::from_slice(&written).unwrap();
                v[obj.trim_start_matches('/')]
                    .as_object_mut()
                    .unwrap()
                    .remove("nv_public");
                serde_json::to_vec(&v).unwrap()
            }),
            (
                "nv_public 15 bytes",
                set(
                    &written,
                    "/tpm_enrollment/nv_public",
                    Value::from(b64(&[1; 15])),
                ),
            ),
            (
                "nv_public 17 bytes",
                set(
                    &written,
                    "/tpm_enrollment/nv_public",
                    Value::from(b64(&[1; 17])),
                ),
            ),
            (
                "nv_auth 31 bytes",
                set(
                    &written,
                    "/tpm_enrollment/nv_auth",
                    Value::from(b64(&[1; 31])),
                ),
            ),
            (
                "nv_auth 33 bytes",
                set(
                    &written,
                    "/tpm_enrollment/nv_auth",
                    Value::from(b64(&[1; 33])),
                ),
            ),
            (
                "primary_name 33 bytes",
                set(
                    &written,
                    "/tpm_enrollment/primary_name",
                    Value::from(b64(&[1; 33])),
                ),
            ),
            (
                "primary_name 35 bytes",
                set(
                    &written,
                    "/tpm_enrollment/primary_name",
                    Value::from(b64(&[1; 35])),
                ),
            ),
            (
                "other template",
                set(
                    &written,
                    "/tpm_enrollment/primary_template",
                    Value::from("qsl-srk-ecc-p256-v2"),
                ),
            ),
            (
                "template case",
                set(
                    &written,
                    "/tpm_enrollment/primary_template",
                    Value::from("QSL-SRK-ECC-P256-V1"),
                ),
            ),
            (
                "nv_auth number array",
                set(
                    &written,
                    "/tpm_enrollment/nv_auth",
                    Value::from(vec![1u8; 32]),
                ),
            ),
            (
                "enrollment a string",
                set(&written, obj, Value::from("tpm")),
            ),
            (
                "enrollment an array",
                set(&written, obj, Value::from(vec![1])),
            ),
        ];
        for (name, bad) in cases {
            assert_eq!(typed(&bad), Err(PayloadV5Error::Malformed), "{name}");
            assert_eq!(open(&fx.blob(&bad)).err(), Some(M), "{name}");
        }
    }

    /// DF-12: a single flipped byte at EVERY offset of a valid blob is Unauthenticated, and so is
    /// every other value of every header byte outside the salt and nonce (those never reach
    /// Argon2: the header refuses first).
    #[test]
    fn t_s6_df12_every_single_byte_flip_is_unauthenticated() {
        let fx = fixture();
        let blob = fx.blob(&text(&local_payload()));
        assert!(open(&blob).is_ok(), "the control opens");
        // Every flip past the header costs one Argon2id run (the header refuses the rest before
        // the key is derived), so the offsets are shared out over the box's cores.
        let started = Instant::now();
        let workers = std::thread::available_parallelism().map_or(1, |n| n.get());
        let opener = opener();
        std::thread::scope(|scope| {
            for worker in 0..workers {
                let (blob, opener) = (&blob, &opener);
                scope.spawn(move || {
                    for at in (worker..blob.len()).step_by(workers) {
                        let mut flipped = blob.clone();
                        flipped[at] ^= 0x01;
                        assert_eq!(
                            opener.open(&flipped).err(),
                            Some(U),
                            "bit flip at offset {at}"
                        );
                    }
                });
            }
        });
        let flips = started.elapsed();
        let started = Instant::now();
        for at in 0..25 {
            for value in 0..=255u8 {
                if value == blob[at] {
                    continue;
                }
                let mut other = blob.clone();
                other[at] = value;
                assert_eq!(open(&other).err(), Some(U), "header byte {at} = {value}");
            }
        }
        let header_values = started.elapsed();
        println!(
            "S6PROBE df12 blob_len={} workers={workers} flips_ms={} header_values_ms={}",
            blob.len(),
            flips.as_millis(),
            header_values.as_millis()
        );
    }

    /// DF-12: every truncation of a valid blob, the empty blob, and appended bytes are Unauthenticated.
    #[test]
    fn t_s6_df12_every_truncation_the_empty_blob_and_trailing_bytes_are_unauthenticated() {
        let fx = fixture();
        let blob = fx.blob(&text(&local_payload()));
        for n in 0..blob.len() {
            assert_eq!(open(&blob[..n]).err(), Some(U), "truncated to {n}");
        }
        assert_eq!(open(&[]).err(), Some(U));
        for extra in [1usize, 4096] {
            let mut longer = blob.clone();
            longer.extend(std::iter::repeat_n(0u8, extra));
            assert_eq!(open(&longer).err(), Some(U), "{extra} trailing bytes");
        }
    }

    /// DF-12: a QSCV03, QSCV02, QSCV01 or unknown magic is Unauthenticated -- including a genuine
    /// -03 envelope sealed by the live serializer's header (the opener never reads a -03 vault).
    #[test]
    fn t_s6_df12_old_and_unknown_magics_are_unauthenticated() {
        let fx = fixture();
        let written = text(&local_payload());
        for magic in [
            b"QSCV03", b"QSCV02", b"QSCV01", b"QSCV05", b"XXXXXX", b"qscv04",
        ] {
            let header = fx.header(magic, 1, written.len() + 16);
            let blob = fx.blob_with(header.clone(), &header, &written);
            assert_eq!(
                open(&blob).err(),
                Some(U),
                "{}",
                String::from_utf8_lossy(magic)
            );
        }
        let live = envelope_header_bytes(
            1,
            KDF_M_KIB,
            KDF_T,
            KDF_P,
            (written.len() + 16) as u32,
            &fx.salt,
            &fx.nonce,
        );
        assert_eq!(&live[..6], VAULT_MAGIC);
        let genuine_v3 = fx.blob_with(live.clone(), &live, &written);
        assert_eq!(open(&genuine_v3).err(), Some(U), "a live -03 envelope");
    }

    /// DF-12: a wrong passphrase, an empty one, and a consistent envelope of another key source
    /// (keychain) are Unauthenticated: the opener holds no key for them.
    #[test]
    fn t_s6_df12_wrong_passphrase_and_foreign_key_source_are_unauthenticated() {
        let fx = fixture();
        let written = text(&local_payload());
        let blob = fx.blob(&written);
        let wrong = PassphraseOpener::new(Zeroizing::new("f04-s6-other".to_owned()));
        assert_eq!(wrong.open(&blob).err(), Some(U));
        let empty = PassphraseOpener::new(Zeroizing::new(String::new()));
        assert_eq!(empty.open(&blob).err(), Some(U));
        for source in [2u8, 3, 4, 0] {
            let header = fx.header(VAULT_MAGIC_V4, source, written.len() + 16);
            let foreign = fx.blob_with(header.clone(), &header, &written);
            assert_eq!(open(&foreign).err(), Some(U), "key_source {source}");
        }
    }

    /// The tag covers the header: a ciphertext sealed under no AAD, or under the live serializer's
    /// -03 header for the same fields, does not open behind a QSCV04 header; the same ciphertext
    /// sealed under the QSCV04 header does. The magic is AEAD-bound (C01 row 13).
    #[test]
    fn t_s6_the_tag_covers_the_header() {
        let fx = fixture();
        let written = text(&local_payload());
        let header = fx.header(VAULT_MAGIC_V4, 1, written.len() + 16);
        assert_eq!(
            open(&fx.blob_with(header.clone(), &[], &written)).err(),
            Some(U),
            "no AAD"
        );
        let live = envelope_header_bytes(
            1,
            KDF_M_KIB,
            KDF_T,
            KDF_P,
            (written.len() + 16) as u32,
            &fx.salt,
            &fx.nonce,
        );
        assert_eq!(
            open(&fx.blob_with(header.clone(), &live, &written)).err(),
            Some(U),
            "-03 AAD"
        );
        let mut other = header.clone();
        other[24] ^= 0x80;
        assert_eq!(
            open(&fx.blob_with(header.clone(), &other, &written)).err(),
            Some(U),
            "another header as AAD"
        );
        assert!(open(&fx.blob_with(header.clone(), &header, &written)).is_ok());
    }

    /// Authenticated but unreadable: each kind through the decoder (typed) and through the opener
    /// (Malformed, whatever the kind), with the discriminators order-independent.
    #[test]
    fn t_s6_authenticated_but_unreadable_is_malformed_and_typed() {
        use PayloadV5Error::{
            Malformed, ModeUnsupported, ProtectionModeUnsupported, VersionUnsupported,
        };
        let fx = fixture();
        let local = text(&local_payload());
        let tpm = text(&tpm_payload());
        let id = b64(&syn_vault_id());
        assert_eq!(id.len(), 44);
        let mut cases: Vec<(String, Vec<u8>, PayloadV5Error)> = Vec::new();
        for name in [
            "version",
            "protocol",
            "mode",
            "protection_mode",
            "vault_id",
            "generation",
            "predecessor_anchor",
            "checkpoint_mac_key",
            "tpm_enrollment",
            "secrets",
        ] {
            let kind = if name == "mode" {
                ModeUnsupported
            } else {
                Malformed
            };
            cases.push((format!("{name} absent"), without(&local, name), kind));
        }
        cases.push((
            "unknown member".into(),
            add(&local, "", "extra", Value::from(1)),
            Malformed,
        ));
        cases.push((
            "version 4".into(),
            set(&local, "/version", Value::from(4)),
            VersionUnsupported,
        ));
        cases.push((
            "version 6".into(),
            set(&local, "/version", Value::from(6)),
            VersionUnsupported,
        ));
        cases.push((
            "version 0".into(),
            set(&local, "/version", Value::from(0)),
            VersionUnsupported,
        ));
        cases.push((
            "version 300".into(),
            set(&local, "/version", Value::from(300)),
            VersionUnsupported,
        ));
        cases.push((
            "version \"5\"".into(),
            set(&local, "/version", Value::from("5")),
            Malformed,
        ));
        cases.push((
            "version 5.0".into(),
            set(&local, "/version", Value::from(5.0)),
            Malformed,
        ));
        cases.push((
            "version -5".into(),
            set(&local, "/version", Value::from(-5)),
            Malformed,
        ));
        cases.push((
            "version null".into(),
            set(&local, "/version", Value::Null),
            Malformed,
        ));
        for bad in [
            "storage",
            "Messaging",
            "storage_only",
            "",
            "messaging ",
            "MESSAGING",
            "archive",
        ] {
            cases.push((
                format!("mode {bad:?}"),
                set(&local, "/mode", Value::from(bad)),
                ModeUnsupported,
            ));
        }
        cases.push((
            "mode a number".into(),
            set(&local, "/mode", Value::from(5)),
            Malformed,
        ));
        cases.push((
            "mode null".into(),
            set(&local, "/mode", Value::Null),
            Malformed,
        ));
        for bad in ["Tpm", "local_checkpoint", "", "tpm ", "none"] {
            cases.push((
                format!("protection_mode {bad:?}"),
                set(&local, "/protection_mode", Value::from(bad)),
                ProtectionModeUnsupported,
            ));
        }
        cases.push((
            "protection_mode a number".into(),
            set(&local, "/protection_mode", Value::from(1)),
            Malformed,
        ));
        cases.push((
            "tpm object with local-checkpoint".into(),
            set(
                &local,
                "/tpm_enrollment",
                serde_json::from_slice::<Value>(&tpm).unwrap()["tpm_enrollment"].clone(),
            ),
            Malformed,
        ));
        cases.push((
            "null with tpm".into(),
            set(&tpm, "/tpm_enrollment", Value::Null),
            Malformed,
        ));
        for member in ["vault_id", "predecessor_anchor", "checkpoint_mac_key"] {
            let p = format!("/{member}");
            let forms = [
                ("no pad, 43 chars", id[..43].to_owned()),
                ("no pad, 44 chars (33 bytes)", format!("{}A", &id[..43])),
                ("two pads (31 bytes)", format!("{}==", &id[..42])),
                ("extra pad", format!("{id}=")),
                ("pad inside", format!("{}={}", &id[..10], &id[11..])),
                ("non-alphabet", format!("{}-{}", &id[..10], &id[11..])),
                ("space inside", format!("{} {}", &id[..10], &id[11..])),
                ("16-byte text", b64(&[7; 16])),
                ("48-char text (36 bytes)", b64(&[7; 36])),
                ("non-zero trailing bits", format!("{}B=", &id[..42])),
                ("empty", String::new()),
            ];
            for (form, bad) in forms {
                cases.push((
                    format!("{member} {form}"),
                    set(&local, &p, Value::from(bad)),
                    Malformed,
                ));
            }
            cases.push((
                format!("{member} number array"),
                set(&local, &p, Value::from(vec![1u8; 32])),
                Malformed,
            ));
            cases.push((
                format!("{member} null"),
                set(&local, &p, Value::Null),
                Malformed,
            ));
        }
        for (form, bad) in [
            ("\"1\"", Value::from("1")),
            ("-1", Value::from(-1)),
            ("1.0", Value::from(1.0)),
            ("null", Value::Null),
            (
                "2^64",
                serde_json::from_str("18446744073709551616").unwrap(),
            ),
        ] {
            cases.push((
                format!("generation {form}"),
                set(&local, "/generation", bad),
                Malformed,
            ));
        }
        cases.push((
            "duplicate secrets key".into(),
            set(&local, "/secrets", Value::Null).splice_secrets(),
            Malformed,
        ));
        for (name, value) in [
            ("version", "5"),
            ("mode", "\"messaging\""),
            ("protection_mode", "\"local-checkpoint\""),
            ("vault_id", &format!("\"{id}\"")),
            ("secrets", "{}"),
        ] {
            cases.push((
                format!("{name} repeated"),
                repeated(&local, name, value),
                Malformed,
            ));
        }
        // Order independence: the discriminators are judged before any other member, wherever they sit.
        cases.push((
            "version 6 and an unknown member".into(),
            add(
                &set(&local, "/version", Value::from(6)),
                "",
                "aaa_first",
                Value::from(1),
            ),
            VersionUnsupported,
        ));
        cases.push((
            "version 6 and mode absent".into(),
            without(&set(&local, "/version", Value::from(6)), "mode"),
            VersionUnsupported,
        ));
        cases.push((
            "version 6 and vault_id 43 chars".into(),
            set(
                &set(&local, "/version", Value::from(6)),
                "/vault_id",
                Value::from(id[..43].to_owned()),
            ),
            VersionUnsupported,
        ));
        cases.push((
            "mode and protection_mode unknown".into(),
            set(
                &set(&local, "/mode", Value::from("x")),
                "/protection_mode",
                Value::from("y"),
            ),
            ModeUnsupported,
        ));
        cases.push((
            "mode absent and an unknown member".into(),
            add(&without(&local, "mode"), "", "aaa_first", Value::from(1)),
            ModeUnsupported,
        ));
        cases.push((
            "protection_mode unknown and an unknown member".into(),
            add(
                &set(&local, "/protection_mode", Value::from("y")),
                "",
                "aaa_first",
                Value::from(1),
            ),
            ProtectionModeUnsupported,
        ));
        cases.push((
            "protection_mode unknown and vault_id absent".into(),
            without(
                &set(&local, "/protection_mode", Value::from("y")),
                "vault_id",
            ),
            ProtectionModeUnsupported,
        ));
        for (name, bad) in [
            ("empty text", b"".to_vec()),
            ("empty object", b"{}".to_vec()),
            ("an array", b"[]".to_vec()),
            ("null", b"null".to_vec()),
            ("a string", b"\"\"".to_vec()),
            ("trailing garbage", [local.clone(), b"x".to_vec()].concat()),
            ("two objects", [local.clone(), local.clone()].concat()),
        ] {
            cases.push((name.into(), bad, Malformed));
        }
        cases.push((
            "the live v4 shape".into(),
            serde_json::to_vec(&VaultPayload::empty(false).unwrap()).unwrap(),
            VersionUnsupported,
        ));
        let live_v4_with_version_5 = set(
            &serde_json::to_vec(&VaultPayload::empty(false).unwrap()).unwrap(),
            "/version",
            Value::from(5),
        );
        cases.push((
            "the live v4 shape with version 5".into(),
            live_v4_with_version_5,
            ModeUnsupported,
        ));
        let mut seen = std::collections::BTreeSet::new();
        for (name, bad, kind) in &cases {
            assert!(seen.insert(name.clone()), "case named twice: {name}");
            assert_eq!(typed(bad), Err(*kind), "{name}");
            assert_eq!(
                open(&fx.blob(bad)).err(),
                Some(M),
                "{name} through the opener"
            );
        }
        println!("S6PROBE malformed_cases={}", cases.len());
        assert!(cases.len() >= 90, "{}", cases.len());
        assert!(
            open(&fx.blob(&local)).is_ok() && open(&fx.blob(&tpm)).is_ok(),
            "controls"
        );
    }
    trait SpliceSecrets {
        fn splice_secrets(self) -> Vec<u8>;
    }
    impl SpliceSecrets for Vec<u8> {
        /// `"secrets":null` -> `"secrets":{"a":"1","a":"2"}` by text, since a Value cannot hold a
        /// repeated key.
        fn splice_secrets(self) -> Vec<u8> {
            let s = String::from_utf8(self).unwrap();
            assert_eq!(s.matches("\"secrets\":null").count(), 1);
            s.replace("\"secrets\":null", "\"secrets\":{\"a\":\"1\",\"a\":\"2\"}")
                .into_bytes()
        }
    }

    /// The typed kinds map to the registered codes where one exists (P4): none for the mode.
    #[test]
    fn t_s6_typed_kinds_map_to_registered_codes_only() {
        assert_eq!(
            PayloadV5Error::VersionUnsupported.code(),
            Some("vault_version_unsupported")
        );
        assert_eq!(PayloadV5Error::ModeUnsupported.code(), None);
        assert_eq!(
            PayloadV5Error::ProtectionModeUnsupported.code(),
            Some("vault_protection_mode_unsupported")
        );
        assert_eq!(
            PayloadV5Error::ProtectionModeUnsupported.code(),
            Some(crate::freshness::codes::VAULT_PROTECTION_MODE_UNSUPPORTED)
        );
        assert_eq!(PayloadV5Error::Malformed.code(), Some("vault_parse_failed"));
    }

    /// The writer emits the ten members in C07-02's order, version first.
    #[test]
    fn t_s6_the_writer_emits_c07_02_member_order() {
        let s = String::from_utf8(text(&tpm_payload())).unwrap();
        assert!(s.starts_with("{\"version\":5,\"protocol\":\"protocol-unchecked-at-s6\",\"mode\":\"storage-only\",\"protection_mode\":\"tpm\",\"vault_id\":\""), "{s}");
        let order = [
            "version",
            "protocol",
            "mode",
            "protection_mode",
            "vault_id",
            "generation",
            "predecessor_anchor",
            "checkpoint_mac_key",
            "tpm_enrollment",
            "secrets",
        ];
        let at: Vec<usize> = order
            .iter()
            .map(|m| s.find(&format!("\"{m}\":")).unwrap())
            .collect();
        assert!(at.windows(2).all(|w| w[0] < w[1]), "{at:?}");
        let v: Value = serde_json::from_str(&s).unwrap();
        assert_eq!(
            v.as_object().unwrap().len(),
            10,
            "ten top-level members {s}"
        );
        assert_eq!(v["tpm_enrollment"].as_object().unwrap().len(), 4);
        assert!(s.contains("\"tpm_enrollment\":{\"nv_public\":\""));
        assert!(s.contains("\",\"nv_auth\":\""));
        assert!(s.contains("\",\"primary_template\":\"qsl-srk-ecc-p256-v1\",\"primary_name\":\""));
    }

    /// B: the L2 bytes of the six C07 members as this writer serializes them equal T6.1 F1-F6 and
    /// F-sum (LOCAL 268 at a 1-digit generation, 287 at u64::MAX; TPM 455 and 474).
    #[test]
    fn t_s6_c07_member_byte_count_matches_t6_1() {
        fn member(payload: &VaultPayloadV5, name: &str) -> usize {
            let v: Value = serde_json::from_slice(&text(payload)).unwrap();
            format!("\"{name}\":").len() + serde_json::to_string(&v[name]).unwrap().len()
        }
        let six = [
            "vault_id",
            "protection_mode",
            "generation",
            "predecessor_anchor",
            "checkpoint_mac_key",
            "tpm_enrollment",
        ];
        let sum = |p: &VaultPayloadV5| six.iter().map(|m| member(p, m)).sum::<usize>() + 6;
        let local1 = VaultPayloadV5 {
            generation: 1,
            ..local_payload()
        };
        let local20 = VaultPayloadV5 {
            generation: u64::MAX,
            ..local_payload()
        };
        let tpm1 = VaultPayloadV5 {
            generation: 1,
            ..tpm_payload()
        };
        let tpm20 = VaultPayloadV5 {
            generation: u64::MAX,
            ..tpm_payload()
        };
        assert_eq!(member(&local1, "vault_id"), 57, "F1");
        assert_eq!(member(&local1, "protection_mode"), 36, "F2 local");
        assert_eq!(member(&tpm1, "protection_mode"), 23, "F2 tpm");
        assert_eq!(member(&local1, "generation"), 14, "F3 1 digit");
        assert_eq!(member(&local20, "generation"), 33, "F3 20 digits");
        assert_eq!(u64::MAX.to_string().len(), 20);
        assert_eq!(member(&local1, "predecessor_anchor"), 67, "F4");
        assert_eq!(member(&local1, "checkpoint_mac_key"), 67, "F5");
        assert_eq!(member(&local1, "tpm_enrollment"), 21, "F6 null");
        assert_eq!(member(&tpm1, "tpm_enrollment"), 221, "F6 object");
        assert_eq!(
            serde_json::to_string(
                &serde_json::from_slice::<Value>(&text(&tpm1)).unwrap()["tpm_enrollment"]
            )
            .unwrap()
            .len(),
            204,
            "F6 object of 204"
        );
        assert_eq!(sum(&local1), 268, "F-sum LOCAL 1 digit");
        assert_eq!(sum(&local20), 287, "F-sum LOCAL 20 digits");
        assert_eq!(sum(&tpm1), 455, "F-sum TPM 1 digit");
        assert_eq!(sum(&tpm20), 474, "F-sum TPM 20 digits");
        println!(
            "S6PROBE bc local={}..{} tpm={}..{}",
            sum(&local1),
            sum(&local20),
            sum(&tpm1),
            sum(&tpm20)
        );
    }

    /// KEY HYGIENE, the source census: in the S6 block, the MAC key and nv_auth are declared in
    /// zeroizing storage, every line that names the MAC key outside a comment is that declaration
    /// or a bare move, the derived key and the plaintext are bound into `Zeroizing::new(` on the
    /// line that creates them, and the passphrase rests in `Zeroizing<String>`.
    #[test]
    fn t_s6_key_material_rests_only_in_zeroizing_storage() {
        let src = include_str!("mod.rs");
        let start = src
            .find(
                "// NA-0788 F04/S6 (SPLIT S11a; C07-02, C07-03; C01 rows 13-16): PAYLOAD VERSION 5",
            )
            .unwrap();
        let end = start
            + src[start..]
                .find("// ============================== END OF S6")
                .unwrap();
        let block = &src[start..end];
        assert!(
            block.contains("\n    checkpoint_mac_key: Zeroizing<[u8; 32]>,\n"),
            "the MAC key member"
        );
        assert!(
            block.contains("\n    nv_auth: Zeroizing<[u8; 32]>,\n"),
            "the nv_auth member"
        );
        assert!(
            block.contains("\n    passphrase: Zeroizing<String>,\n"),
            "the passphrase"
        );
        let code = |l: &str| !l.trim_start().starts_with("//");
        for line in block
            .lines()
            .filter(|l| code(l) && l.contains("checkpoint_mac_key"))
        {
            let t = line.trim();
            assert!(
                t == "checkpoint_mac_key: Zeroizing<[u8; 32]>," || t == "checkpoint_mac_key,",
                "a MAC-key line that is neither the zeroizing declaration nor a move: {line}"
            );
        }
        for line in block.lines().filter(|l| code(l) && l.contains("nv_auth")) {
            assert_eq!(line.trim(), "nv_auth: Zeroizing<[u8; 32]>,", "{line}");
        }
        let statement = |lead: &str| {
            let at = block.find(lead).unwrap_or_else(|| panic!("{lead}"));
            &block[at + lead.len()..at + lead.len() + 16]
        };
        assert!(
            statement("let mut key = ").starts_with("Zeroizing::new("),
            "the derived key"
        );
        assert!(
            statement("let plaintext = ").starts_with("Zeroizing::new("),
            "the plaintext"
        );
        assert_eq!(block.matches("let mut key = ").count(), 1);
        assert_eq!(block.matches("let plaintext = ").count(), 1);
        assert!(!block.contains(".clone()"), "no clone in the block");
        assert!(!block.contains("to_vec()"), "no copy in the block");
    }

    /// KEY HYGIENE, the Drop witness: dropping a decoded payload zeroizes the MAC key and nv_auth in
    /// place (the storage is kept alive by ManuallyDrop and read back after the drop), while a
    /// plain-array twin keeps its bytes -- the control that proves the read observes the drop.
    #[test]
    fn t_s6_dropping_the_payload_zeroizes_its_key_material() {
        use std::mem::ManuallyDrop;
        let payload = decode_payload_v5(&text(&tpm_payload())).unwrap();
        let mut slot = ManuallyDrop::new(payload);
        let mac: *const [u8; 32] = &*slot.checkpoint_mac_key;
        let auth: *const [u8; 32] = &*slot.tpm_enrollment.as_ref().unwrap().nv_auth;
        // SAFETY: both pointers address storage inside `slot`, which ManuallyDrop keeps alive for
        // the whole function; a u8 array is valid for any byte pattern, so reading it after the
        // drop observes what the drop left there.
        unsafe {
            assert_eq!(*mac, syn_mac_key());
            assert_eq!(*auth, nv_auth());
            ManuallyDrop::drop(&mut slot);
            assert_eq!(*mac, [0u8; 32], "checkpoint_mac_key survived its drop");
            assert_eq!(*auth, [0u8; 32], "nv_auth survived its drop");
        }
        struct PlainTwin {
            key: [u8; 32],
        }
        let mut twin = ManuallyDrop::new(PlainTwin { key: syn_mac_key() });
        let plain: *const [u8; 32] = &twin.key;
        // SAFETY: as above; the control shows a plain array is NOT cleared by its drop.
        unsafe {
            ManuallyDrop::drop(&mut twin);
            assert_eq!(
                *plain,
                syn_mac_key(),
                "the control: a plain array keeps its bytes"
            );
        }
    }

    /// DEAD CODE UNTIL S7: outside the S6 block no product line of vault/mod.rs names an S6 item
    /// (the two imports excepted), vault_format.rs only defines the classifier, and no product line
    /// of the freshness tree calls the constructor.
    #[test]
    fn t_s6_no_product_caller() {
        let vault = include_str!("mod.rs");
        let product = &vault[..vault.find("\n#[cfg(test)]\n").unwrap()];
        let start = product
            .find(
                "// NA-0788 F04/S6 (SPLIT S11a; C07-02, C07-03; C01 rows 13-16): PAYLOAD VERSION 5",
            )
            .unwrap();
        let end = start
            + product[start..]
                .find("// ============================== END OF S6")
                .unwrap();
        let outside = format!("{}{}", &product[..start], &product[end..]);
        for name in [
            "PassphraseOpener",
            "decode_payload_v5",
            "peek_v5_discriminators",
            "VaultPayloadV5",
            "TpmEnrollmentV5",
            "PayloadV5Error",
            "VaultMode",
            "PAYLOAD_VERSION_V5",
            "VAULT_MAGIC_V4",
            "LineageFields::new(",
        ] {
            assert_eq!(outside.matches(name).count(), 0, "{name} outside the block");
        }
        assert_eq!(
            outside.matches("classify_vault_magic_v4").count(),
            1,
            "the import only"
        );
        assert_eq!(
            outside.matches("LineageFields").count(),
            1,
            "the import only"
        );
        let format = include_str!("../adversarial/vault_format.rs");
        let format = &format[..format.find("\n#[cfg(test)]\n").unwrap()];
        assert_eq!(
            format.matches("classify_vault_magic_v4(").count(),
            1,
            "the definition only"
        );
        assert_eq!(
            format.matches("VAULT_MAGIC_V4").count(),
            2,
            "the definition and the classifier"
        );
        let freshness = include_str!("../freshness/mod.rs");
        assert_eq!(
            freshness.matches("LineageFields::new(").count(),
            1,
            "its own test only"
        );
        assert!(
            freshness.find("LineageFields::new(").unwrap()
                > freshness.find("\nmod tests {\n").unwrap()
        );
        for (name, src) in [
            (
                "freshness/recover.rs",
                include_str!("../freshness/recover.rs"),
            ),
            ("freshness/txn.rs", include_str!("../freshness/txn.rs")),
        ] {
            assert_eq!(src.matches("LineageFields::new(").count(), 0, "{name}");
        }
    }
}
