// QSC vault: encrypted-at-rest secrets store (NA-0061 Phase 2).
//
// Invariants:
// - encrypted-at-rest default (no plaintext mode)
// - keychain preferred when available; deterministic passphrase fallback
// - noninteractive never prompts; fails closed with stable marker
// - no-mutation-on-reject for all storage boundaries touched
//
// This module intentionally prints only deterministic markers (no secrets).

use std::collections::BTreeMap;
use std::fs;
use std::io::{IsTerminal, Read, Write};
use std::path::PathBuf;

use argon2::{Algorithm, Argon2, Params, Version};
use chacha20poly1305::aead::{Aead, KeyInit};
use chacha20poly1305::{ChaCha20Poly1305, Key, Nonce};
use clap::{Args, Subcommand};
#[cfg(feature = "keychain")]
use keyring::Entry;
use rand_core::RngCore;
use serde::{Deserialize, Serialize};
use zeroize::Zeroize;

const VAULT_MAGIC: &[u8; 6] = b"QSCV01";
const KDF_M_KIB: u32 = 19456;
const KDF_T: u32 = 2;
const KDF_P: u32 = 1;

#[cfg(feature = "keychain")]
const VAULT_KEYCHAIN_SERVICE: &str = "qsc";
#[cfg(feature = "keychain")]
const VAULT_KEYCHAIN_ACCOUNT: &str = "vault";

#[derive(Debug, Serialize, Deserialize)]
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
}

#[derive(Debug, Args)]
pub struct VaultInitArgs {
    /// Noninteractive mode never prompts; fails closed if passphrase not provided.
    #[arg(long)]
    non_interactive: bool,

    /// Read passphrase from the given environment variable name.
    #[arg(long, value_name = "ENV")]
    passphrase_env: Option<String>,

    /// Read passphrase from a file path (contents are passphrase; trailing newline trimmed).
    #[arg(long, value_name = "PATH")]
    passphrase_file: Option<std::path::PathBuf>,

    /// Provide passphrase directly (discouraged; intended for tests only).
    #[arg(long, value_name = "PASS")]
    passphrase: Option<String>,

    /// Read passphrase from stdin (explicit; never prompts).
    #[arg(long)]
    passphrase_stdin: bool,

    /// Explicit key source selection: passphrase | keychain | yubikey | mock.
    #[arg(long, value_name = "SRC")]
    key_source: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum KeySource {
    Keychain,
    Passphrase,
    YubiKeyStub,
    MockProvider,
}

#[derive(Debug)]
#[allow(dead_code)]
enum ProviderError {
    YubiKeyNotImplemented,
    TokenMissing,
    TokenUnavailable,
    ProviderFailed,
}

pub fn cmd_vault(cmd: VaultCmd) {
    match cmd {
        VaultCmd::Init(args) => vault_init(args),
        VaultCmd::Status => vault_status(),
    }
}

fn vault_init(args: VaultInitArgs) {
    let noninteractive = args.non_interactive
        || std::env::var("QSC_NONINTERACTIVE").ok().as_deref() == Some("1")
        || !std::io::stdin().is_terminal();

    let mut pass = resolve_passphrase(&args);
    let pass_present = pass.as_ref().map(|p| !p.is_empty()).unwrap_or(false);

    let explicit_key_source = key_source_explicit(&args);
    let mut key_source = match resolve_key_source(&args) {
        Ok(src) => src,
        Err(()) => fail_with_marker_pass("key_source_invalid", &mut pass),
    };

    if key_source == KeySource::Keychain && !keychain_supported() {
        if explicit_key_source {
            handle_provider_error_with_pass(ProviderError::TokenUnavailable, &mut pass);
        } else if pass_present {
            // Deterministic passphrase fallback when keychain is unavailable.
            key_source = KeySource::Passphrase;
        } else if noninteractive {
            fail_with_marker_pass("vault_passphrase_required_noninteractive", &mut pass);
        } else {
            fail_with_marker_pass("vault_passphrase_required", &mut pass);
        }
    }

    if key_source == KeySource::Passphrase && !pass_present {
        if noninteractive {
            fail_with_marker_pass("vault_passphrase_required_noninteractive", &mut pass);
        } else {
            fail_with_marker_pass("vault_passphrase_required", &mut pass);
        }
    }

    let params = match Params::new(KDF_M_KIB, KDF_T, KDF_P, Some(32)) {
        Ok(p) => p,
        Err(_) => fail_with_marker_pass("vault_kdf_params_invalid", &mut pass),
    };
    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);

    let mut pass_bytes = match pass.take() {
        Some(p) => p.into_bytes(),
        None => Vec::new(),
    };

    let mut salt = [0u8; 16];
    rand_core::OsRng.fill_bytes(&mut salt);

    let mut key_bytes = [0u8; 32];
    if let Err(err) = derive_key(
        key_source,
        &argon2,
        &mut pass_bytes,
        &mut salt,
        &mut key_bytes,
    ) {
        pass_bytes.zeroize();
        key_bytes.zeroize();
        handle_provider_error(err);
    }

    let cipher = ChaCha20Poly1305::new(Key::from_slice(&key_bytes));

    let mut nonce_bytes = [0u8; 12];
    rand_core::OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    let payload = VaultPayload::empty();
    let plaintext = match serde_json::to_vec(&payload) {
        Ok(v) => v,
        Err(_) => {
            fail_with_marker_buffers(
                "vault_payload_serialize_failed",
                &mut pass_bytes,
                &mut key_bytes,
            );
        }
    };

    let ciphertext = match cipher.encrypt(nonce, plaintext.as_ref()) {
        Ok(ct) => ct,
        Err(_) => {
            fail_with_marker_buffers("encrypt_failed", &mut pass_bytes, &mut key_bytes);
        }
    };

    let (_cfg_dir, vault_path) = vault_path_resolved();

    if vault_path.exists() {
        fail_with_marker_buffers("vault_exists", &mut pass_bytes, &mut key_bytes);
    }

    let parent = match vault_path.parent() {
        Some(p) => p,
        None => fail_with_marker_buffers("vault_path_invalid", &mut pass_bytes, &mut key_bytes),
    };

    // Only create directory after all crypto work succeeded to minimize mutation on reject.
    if let Err(_) = fs::create_dir_all(parent) {
        fail_with_marker_buffers(
            "vault_parent_create_failed",
            &mut pass_bytes,
            &mut key_bytes,
        );
    }

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        if let Err(_) = fs::set_permissions(parent, fs::Permissions::from_mode(0o700)) {
            fail_with_marker_buffers("vault_parent_perms_failed", &mut pass_bytes, &mut key_bytes);
        }
    }

    let mut buf = Vec::with_capacity(6 + 1 + 1 + 1 + 4 * 4 + 16 + 12 + ciphertext.len());
    buf.extend_from_slice(VAULT_MAGIC);
    buf.push(key_source_tag(key_source));
    buf.push(16);
    buf.push(12);
    buf.extend_from_slice(&KDF_M_KIB.to_le_bytes());
    buf.extend_from_slice(&KDF_T.to_le_bytes());
    buf.extend_from_slice(&KDF_P.to_le_bytes());
    buf.extend_from_slice(&(ciphertext.len() as u32).to_le_bytes());
    buf.extend_from_slice(&salt);
    buf.extend_from_slice(&nonce_bytes);
    buf.extend_from_slice(&ciphertext);

    let tmp = vault_path.with_extension("qsv.tmp");
    if tmp.exists() {
        let _ = fs::remove_file(&tmp);
    }

    // For keychain provider, store the key *before* file write to avoid mutation on reject.
    if key_source == KeySource::Keychain {
        if let Err(err) = keychain_store_key(&key_bytes) {
            pass_bytes.zeroize();
            key_bytes.zeroize();
            handle_provider_error(err);
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
            let _ = keychain_remove_key();
        }
        fail_with_marker_buffers("vault_write_failed", &mut pass_bytes, &mut key_bytes);
    }

    // Zeroize secrets after successful commit.
    key_bytes.zeroize();
    pass_bytes.zeroize();

    crate::print_marker("vault_init", &[("path", "redacted")]);
}

fn vault_status() {
    let (_cfg_dir, vault_path) = vault_path_resolved();
    if !vault_path.exists() {
        crate::print_error_marker("vault_missing");
    }

    let bytes = match fs::read(&vault_path) {
        Ok(b) => b,
        Err(_) => crate::print_error_marker("vault_read_failed"),
    };

    if bytes.len() < 6 + 1 {
        crate::print_error_marker("vault_parse_failed");
    }
    if &bytes[..6] != VAULT_MAGIC {
        crate::print_error_marker("vault_parse_failed");
    }
    let key_source = key_source_name(bytes[6]);

    crate::print_marker(
        "vault_status",
        &[("present", "true"), ("key_source", key_source)],
    );
}

fn resolve_key_source(args: &VaultInitArgs) -> Result<KeySource, ()> {
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
        Some("mock") => Ok(KeySource::MockProvider),
        Some(_) => Err(()),
        None => {
            if std::env::var("QSC_DISABLE_KEYCHAIN").ok().as_deref() == Some("1") {
                Ok(KeySource::Passphrase)
            } else {
                if keychain_supported() {
                    Ok(KeySource::Keychain)
                } else {
                    Ok(KeySource::Passphrase)
                }
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
        KeySource::MockProvider => 4,
    }
}

fn key_source_name(tag: u8) -> &'static str {
    match tag {
        1 => "passphrase",
        2 => "keychain",
        3 => "yubikey",
        4 => "mock",
        _ => "unknown",
    }
}

fn keychain_supported() -> bool {
    if std::env::var("QSC_DISABLE_KEYCHAIN").ok().as_deref() == Some("1") {
        return false;
    }
    #[cfg(feature = "keychain")]
    {
        Entry::new(VAULT_KEYCHAIN_SERVICE, VAULT_KEYCHAIN_ACCOUNT).is_ok()
    }
    #[cfg(not(feature = "keychain"))]
    {
        false
    }
}

fn keychain_store_key(key: &[u8]) -> Result<(), ProviderError> {
    #[cfg(feature = "keychain")]
    {
        let entry = Entry::new(VAULT_KEYCHAIN_SERVICE, VAULT_KEYCHAIN_ACCOUNT)
            .map_err(|_| ProviderError::ProviderFailed)?;
        let mut enc = hex_encode(key);
        let res = entry
            .set_password(&enc)
            .map_err(|_| ProviderError::ProviderFailed);
        enc.zeroize();
        res?;
        Ok(())
    }
    #[cfg(not(feature = "keychain"))]
    {
        let _ = key;
        Err(ProviderError::TokenUnavailable)
    }
}

fn keychain_remove_key() -> Result<(), ProviderError> {
    #[cfg(feature = "keychain")]
    {
        let entry = Entry::new(VAULT_KEYCHAIN_SERVICE, VAULT_KEYCHAIN_ACCOUNT)
            .map_err(|_| ProviderError::ProviderFailed)?;
        entry
            .delete_password()
            .map_err(|_| ProviderError::ProviderFailed)?;
        Ok(())
    }
    #[cfg(not(feature = "keychain"))]
    {
        Err(ProviderError::TokenUnavailable)
    }
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

fn resolve_passphrase(args: &VaultInitArgs) -> Option<String> {
    if let Some(p) = args.passphrase.clone() {
        if !p.is_empty() {
            return Some(p);
        }
    }

    if let Some(env_name) = args.passphrase_env.clone() {
        if let Ok(v) = std::env::var(env_name) {
            if !v.is_empty() {
                return Some(v);
            }
        }
    }

    if let Some(path) = args.passphrase_file.clone() {
        if let Ok(b) = fs::read(&path) {
            let mut v = String::from_utf8_lossy(&b).to_string();
            while v.ends_with('\n') || v.ends_with('\r') {
                v.pop();
            }
            if !v.is_empty() {
                return Some(v);
            }
        }
    }

    if args.passphrase_stdin {
        let mut buf = String::new();
        if std::io::stdin().read_to_string(&mut buf).is_ok() {
            while buf.ends_with('\n') || buf.ends_with('\r') {
                buf.pop();
            }
            if !buf.is_empty() {
                return Some(buf);
            }
        }
    }

    if let Ok(v) = std::env::var("QSC_PASSPHRASE") {
        if !v.is_empty() {
            return Some(v);
        }
    }

    None
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
            if let Err(_) = argon2.hash_password_into(pass_bytes, salt, key_bytes) {
                return Err(ProviderError::ProviderFailed);
            }
        }
        KeySource::Keychain => {
            rand_core::OsRng.fill_bytes(key_bytes);
        }
        KeySource::YubiKeyStub => {
            return Err(ProviderError::YubiKeyNotImplemented);
        }
        KeySource::MockProvider => {
            // Deterministic mock key for CI tests only.
            *key_bytes = [0x42u8; 32];
        }
    }
    Ok(())
}

fn handle_provider_error(err: ProviderError) -> ! {
    match err {
        ProviderError::YubiKeyNotImplemented => {
            crate::print_error_marker("vault_yubikey_not_implemented");
        }
        ProviderError::TokenMissing => {
            crate::print_error_marker("vault_token_missing");
        }
        ProviderError::TokenUnavailable => {
            crate::print_error_marker("vault_token_unavailable");
        }
        ProviderError::ProviderFailed => {
            crate::print_error_marker("vault_provider_failed");
        }
    }
}

fn zeroize_passphrase(pass: &mut Option<String>) {
    if let Some(p) = pass.as_mut() {
        p.zeroize();
    }
}

fn fail_with_marker_pass(code: &str, pass: &mut Option<String>) -> ! {
    zeroize_passphrase(pass);
    crate::print_error_marker(code);
}

fn fail_with_marker_buffers(code: &str, pass_bytes: &mut Vec<u8>, key_bytes: &mut [u8; 32]) -> ! {
    pass_bytes.zeroize();
    key_bytes.zeroize();
    crate::print_error_marker(code);
}

fn handle_provider_error_with_pass(err: ProviderError, pass: &mut Option<String>) -> ! {
    zeroize_passphrase(pass);
    handle_provider_error(err);
}

fn vault_path_resolved() -> (PathBuf, PathBuf) {
    let cfg = std::env::var("QSC_CONFIG_DIR")
        .ok()
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."));
    (cfg.clone(), cfg.join("vault.qsv"))
}
