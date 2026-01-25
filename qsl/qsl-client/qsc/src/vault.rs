// QSC vault: encrypted-at-rest secrets store (NA-0061 Phase 2).
//
// Invariants:
// - encrypted-at-rest default (no plaintext mode)
// - keychain preferred when available; deterministic passphrase fallback
// - noninteractive never prompts; fails closed with stable marker
// - no-mutation-on-reject for all storage boundaries touched
//
// This module intentionally prints only deterministic markers (no secrets).

use std::fs;
use std::io::{IsTerminal, Write};
use std::path::PathBuf;

use argon2::{Algorithm, Argon2, Params, Version};
use chacha20poly1305::aead::{Aead, KeyInit};
use chacha20poly1305::{ChaCha20Poly1305, Key, Nonce};
use clap::{Args, Subcommand};
use rand_core::RngCore;
use zeroize::Zeroize;

const VAULT_MAGIC: &[u8; 6] = b"QSCV01";
const KDF_M_KIB: u32 = 19456;
const KDF_T: u32 = 2;
const KDF_P: u32 = 1;

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

    /// Explicit key source selection: passphrase | keychain | yubikey.
    #[arg(long, value_name = "SRC")]
    key_source: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum KeySource {
    Keychain,
    Passphrase,
    YubiKeyStub,
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

    let pass = resolve_passphrase(&args);
    let pass_present = pass.as_ref().map(|p| !p.is_empty()).unwrap_or(false);

    let mut key_source = resolve_key_source(&args, pass_present);

    match key_source {
        KeySource::YubiKeyStub => {
            crate::print_error_marker("yubikey_not_implemented");
        }
        KeySource::Keychain => {
            let keychain_available =
                std::env::var("QSC_KEYCHAIN_AVAILABLE").ok().as_deref() == Some("1");
            if keychain_available {
                crate::print_error_marker("keychain_not_implemented");
            } else if pass_present {
                // Deterministic passphrase fallback when keychain is unavailable.
                key_source = KeySource::Passphrase;
            } else if noninteractive {
                crate::print_error_marker("passphrase_required_noninteractive");
            } else {
                crate::print_error_marker("keychain_unavailable");
            }
        }
        KeySource::Passphrase => {
            if !pass_present {
                if noninteractive {
                    crate::print_error_marker("passphrase_required_noninteractive");
                } else {
                    crate::print_error_marker("passphrase_required");
                }
            }
        }
    }

    let pass_bytes = match pass {
        Some(p) => p.into_bytes(),
        None => Vec::new(),
    };

    let mut salt = [0u8; 16];
    rand_core::OsRng.fill_bytes(&mut salt);

    let params = Params::new(KDF_M_KIB, KDF_T, KDF_P, Some(32)).unwrap();
    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);

    let mut key_bytes = [0u8; 32];
    if let Err(_) = argon2.hash_password_into(&pass_bytes, &salt, &mut key_bytes) {
        let mut pass_bytes = pass_bytes;
        pass_bytes.zeroize();
        crate::print_error_marker("kdf_failed");
    }

    let cipher = ChaCha20Poly1305::new(Key::from_slice(&key_bytes));

    let mut nonce_bytes = [0u8; 12];
    rand_core::OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    let plaintext = b"QSC_VAULT/1\nsecret=QSC_TEST_SECRET\n";

    let ciphertext = match cipher.encrypt(nonce, plaintext.as_ref()) {
        Ok(ct) => ct,
        Err(_) => {
            let mut pass_bytes = pass_bytes;
            pass_bytes.zeroize();
            key_bytes.zeroize();
            crate::print_error_marker("encrypt_failed");
        }
    };

    key_bytes.zeroize();
    let mut pass_bytes = pass_bytes;
    pass_bytes.zeroize();

    let (_cfg_dir, vault_path) = vault_path_resolved();

    if vault_path.exists() {
        crate::print_error_marker("vault_exists");
    }

    let parent = match vault_path.parent() {
        Some(p) => p,
        None => crate::print_error_marker("vault_path_invalid"),
    };

    // Only create directory after all crypto work succeeded to minimize mutation on reject.
    if let Err(_) = fs::create_dir_all(parent) {
        crate::print_error_marker("vault_parent_create_failed");
    }

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        if let Err(_) = fs::set_permissions(parent, fs::Permissions::from_mode(0o700)) {
            crate::print_error_marker("vault_parent_perms_failed");
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
        crate::print_error_marker("vault_write_failed");
    }

    crate::print_marker("vault_init", &[("path", "redacted")]);
}

fn vault_status() {
    let (_cfg_dir, vault_path) = vault_path_resolved();
    let present = vault_path.exists();
    crate::print_marker(
        "vault_status",
        &[("present", if present { "true" } else { "false" })],
    );
}

fn resolve_key_source(args: &VaultInitArgs, pass_present: bool) -> KeySource {
    let env_src = std::env::var("QSC_KEY_SOURCE").ok();
    let src = args
        .key_source
        .as_ref()
        .or(env_src.as_ref())
        .map(|s| s.as_str());

    match src {
        Some("yubikey") => KeySource::YubiKeyStub,
        Some("keychain") => KeySource::Keychain,
        Some("passphrase") => KeySource::Passphrase,
        Some(_) => {
            crate::print_error_marker("key_source_invalid");
        }
        None => {
            if std::env::var("QSC_DISABLE_KEYCHAIN").ok().as_deref() == Some("1") {
                KeySource::Passphrase
            } else if pass_present {
                // Deterministic passphrase fallback when keychain is not available.
                KeySource::Passphrase
            } else {
                KeySource::Keychain
            }
        }
    }
}

fn key_source_tag(src: KeySource) -> u8 {
    match src {
        KeySource::Passphrase => 1,
        KeySource::Keychain => 2,
        KeySource::YubiKeyStub => 3,
    }
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

    if let Ok(v) = std::env::var("QSC_PASSPHRASE") {
        if !v.is_empty() {
            return Some(v);
        }
    }

    None
}

fn vault_path_resolved() -> (PathBuf, PathBuf) {
    let cfg = std::env::var("QSC_CONFIG_DIR")
        .ok()
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."));
    (cfg.clone(), cfg.join("vault.qsv"))
}
