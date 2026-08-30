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
use zeroize::Zeroize;

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
struct VaultPayload {
    version: u8,
    secrets: BTreeMap<String, String>,
}

impl VaultPayload {
    fn empty() -> Self {
        Self {
            version: 1,
            secrets: BTreeMap::new(),
        }
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

    let (_vault_path, runtime) = load_vault_runtime_with_passphrase(None)?;
    decrypt_payload(&runtime).map(|_| ())
}

pub fn unlock_with_passphrase_file(path: &Path) -> Result<(), &'static str> {
    let mut pass = read_passphrase_file(path)?;
    let out = unlock_with_passphrase(pass.as_str());
    pass.zeroize();
    out
}

pub fn unlock_with_passphrase(passphrase: &str) -> Result<(), &'static str> {
    if passphrase.is_empty() {
        return Err("vault_locked");
    }
    let (_vault_path, runtime) = load_vault_runtime_with_passphrase(Some(passphrase))?;
    let out = decrypt_payload(&runtime).map(|_| ());
    if out.is_ok() {
        set_process_passphrase(Some(passphrase));
    }
    out
}

/// NA-0649 (D585 B1): in-process vault creation for the GUI — the passphrase arrives
/// in memory (no argv/env/file/stdin/terminal ingress on this path; the NA-0216B
/// retired-ingress decisions are untouched) and behavior matches a successful
/// `vault init --passphrase-file`: same envelope, same default inbox route-token
/// seeding, same `vault_init` success marker, same error codes returned as values
/// (`vault_exists`, …). No process unlock-state side effect — init and unlock stay
/// orthogonal; the caller decides whether to unlock after init.
pub fn vault_init_with_passphrase(passphrase: &str) -> Result<(), &'static str> {
    if passphrase.is_empty() {
        return Err("vault_passphrase_required");
    }
    vault_init_core(KeySource::Passphrase, Some(passphrase.to_string()))
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
    payload.secrets.insert(name.to_string(), value.to_string());
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
        plaintext.len() as u32 + 16,
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
    payload.secrets.insert(name.to_string(), value.to_string());
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
        plaintext.len() as u32 + 16,
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
    let (vault_path, runtime) = load_vault_runtime_with_passphrase(passphrase_override)?;
    let payload = decrypt_payload(&runtime)?;
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
    session
        .payload
        .secrets
        .insert(name.to_string(), value.to_string());
    persist_session(session)
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
    let write_epoch = VAULT_WRITE_EPOCH.load(Ordering::Relaxed);
    if write_epoch != session.write_epoch_seen {
        let latest_payload = fs::read(&session.vault_path)
            .ok()
            .and_then(|bytes| parse_envelope(&bytes).ok())
            .and_then(|envelope| {
                decrypt_payload(&VaultRuntime {
                    envelope,
                    key: session.key,
                })
                .ok()
            });
        if let Some(mut latest) = latest_payload {
            for (key, value) in session.payload.secrets.iter() {
                latest.secrets.insert(key.clone(), value.clone());
            }
            session.payload = latest;
        }
    }
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
        plaintext.len() as u32 + 16,
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
    // deletion — no lock and no semantic change on this dead path. The refuse-not-merge
    // semantic for the epoch mismatch above is DECIDED and its code rides the Slice-4
    // GUI-wiring lane, which consumes `VAULT_WRITE_EPOCH` (the reason the epoch is kept).
    let (_, _, source) = vault_path_resolved()?;
    PERF_VAULT_ENCRYPT_WRITES.fetch_add(1, Ordering::Relaxed);
    write_atomic(&session.vault_path, &bytes, source).map_err(store_err_marker)?;
    session.write_epoch_seen = VAULT_WRITE_EPOCH.fetch_add(1, Ordering::Relaxed) + 1;
    Ok(())
}

fn vault_init(args: VaultInitArgs) -> CliResult {
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

    vault_init_core(key_source, pass).map_err(CliError::code)
}

// NA-0649 (D585 B1): the ingress-independent tail of `vault init`, shared verbatim by
// the CLI path (`vault_init`) and the in-process library entry
// (`vault_init_with_passphrase`). No argv/env/file/stdin/terminal access here; errors
// are returned as marker-code values; the only output is the existing `vault_init`
// success marker.
fn vault_init_core(key_source: KeySource, mut pass: Option<String>) -> Result<(), &'static str> {
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

    let mut payload = VaultPayload::empty();
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
    let aad = envelope_header_bytes(
        key_source_tag(key_source),
        KDF_M_KIB,
        KDF_T,
        KDF_P,
        plaintext.len() as u32 + 16,
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
        ciphertext.len() as u32,
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

    let bytes = match fs::read(&vault_path) {
        Ok(b) => b,
        Err(_) => return Err(CliError::code("vault_read_failed")),
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

fn load_vault_runtime() -> Result<(PathBuf, VaultRuntime), &'static str> {
    load_vault_runtime_with_passphrase(None)
}

fn load_vault_runtime_with_passphrase(
    passphrase_override: Option<&str>,
) -> Result<(PathBuf, VaultRuntime), &'static str> {
    let (_cfg_dir, vault_path, _source) = vault_path_resolved()?;
    PERF_VAULT_FILE_READS.fetch_add(1, Ordering::Relaxed);
    let bytes = fs::read(&vault_path).map_err(|_| "vault_missing")?;
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
    PERF_VAULT_DECRYPTS.fetch_add(1, Ordering::Relaxed);
    if env.envelope.ciphertext.len() < 12 {
        return Err("vault_parse_failed");
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
        .map_err(|_| "vault_locked")?;
    serde_json::from_slice(&plaintext).map_err(|_| "vault_parse_failed")
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
        let bytes = hex_decode(&secret).ok_or(ProviderError::ProviderFailed)?;
        if bytes.len() != 32 {
            return Err(ProviderError::ProviderFailed);
        }
        out.copy_from_slice(&bytes);
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

#[cfg(feature = "keychain")]
fn hex_decode(s: &str) -> Option<Vec<u8>> {
    if !s.len().is_multiple_of(2) {
        return None;
    }
    let mut out = Vec::with_capacity(s.len() / 2);
    let bytes = s.as_bytes();
    let mut i = 0usize;
    while i < bytes.len() {
        let hi = hex_nibble(bytes[i])?;
        let lo = hex_nibble(bytes[i + 1])?;
        out.push((hi << 4) | lo);
        i += 2;
    }
    Some(out)
}

#[cfg(feature = "keychain")]
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
