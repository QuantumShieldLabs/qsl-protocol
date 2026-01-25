// QSC vault: encrypted-at-rest secrets store (NA-0061).
//
// Invariants:
// - encrypted-at-rest default (no plaintext mode)
// - keychain preferred when available; deterministic passphrase fallback
// - noninteractive never prompts; fails closed with stable marker
// - no-mutation-on-reject for all storage boundaries touched
//
// This module intentionally prints only deterministic markers (no secrets).

use std::fs;
use std::io::Write;
use std::path::PathBuf;

use argon2::{Algorithm, Argon2, Params, Version};
use chacha20poly1305::aead::{Aead, KeyInit};
use chacha20poly1305::{ChaCha20Poly1305, Key, Nonce};
use clap::{Args, Subcommand};
use rand_core::RngCore;

#[derive(Debug, Args)]
pub struct VaultArgs {
    #[command(subcommand)]
    pub cmd: VaultCmd,
}

#[derive(Debug, Subcommand)]
pub enum VaultCmd {
    /// Initialize vault (creates encrypted envelope)
    Init,
    /// Report vault status (no secrets; deterministic markers)
    Status,
}

pub fn cmd_vault(a: VaultArgs) {
    match a.cmd {
        VaultCmd::Init => vault_init(),
        VaultCmd::Status => vault_status(),
    }
}

fn vault_init() {
    // Keychain preferred when available, but tests force passphrase path.
    if std::env::var("QSC_DISABLE_KEYCHAIN").ok().as_deref() != Some("1") {
        // Hook point only; fail closed for now (deterministic) rather than silently falling back.
        crate::print_error_marker("keychain_not_implemented");
    }

    // Noninteractive: never prompt; require QSC_PASSPHRASE.
    let noninteractive = std::env::var("QSC_NONINTERACTIVE").ok().as_deref() == Some("1");
    let pass = std::env::var("QSC_PASSPHRASE").ok();

    if noninteractive && pass.as_deref().unwrap_or("").is_empty() {
        crate::print_error_marker("noninteractive_passphrase_required");
    }
    let pass = match pass {
        Some(p) if !p.is_empty() => p,
        _ => {
            // Interactive prompting not implemented in this step; fail closed deterministically.
            crate::print_error_marker("passphrase_required");
        }
    };

    let (_cfg_dir, vault_path) = vault_path_resolved();

    // Enforce parent directory exists (fail closed; no mutation).
    if let Some(parent) = vault_path.parent() {
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
    } else {
        crate::print_error_marker("vault_path_invalid");
    }

    if vault_path.exists() {
        crate::print_error_marker("vault_exists");
    }

    let mut salt = [0u8; 16];
    rand_core::OsRng.fill_bytes(&mut salt);

    let params = Params::new(19456, 2, 1, Some(32)).unwrap();
    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);

    let mut key_bytes = [0u8; 32];
    if let Err(_) = argon2.hash_password_into(pass.as_bytes(), &salt, &mut key_bytes) {
        crate::print_error_marker("kdf_failed");
    }

    let cipher = ChaCha20Poly1305::new(Key::from_slice(&key_bytes));

    let mut nonce_bytes = [0u8; 12];
    rand_core::OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    let plaintext = b"QSC_VAULT/1";

    let ciphertext = match cipher.encrypt(nonce, plaintext.as_ref()) {
        Ok(ct) => ct,
        Err(_) => {
            crate::print_error_marker("encrypt_failed");
        }
    };

    let mut buf = Vec::with_capacity(6 + 16 + 12 + 4 * 4 + ciphertext.len());
    buf.extend_from_slice(b"QSCV01");
    buf.extend_from_slice(&salt);
    buf.extend_from_slice(&nonce_bytes);
    buf.extend_from_slice(&(19456u32).to_le_bytes());
    buf.extend_from_slice(&(2u32).to_le_bytes());
    buf.extend_from_slice(&(1u32).to_le_bytes());
    buf.extend_from_slice(&(ciphertext.len() as u32).to_le_bytes());
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

fn vault_path_resolved() -> (PathBuf, PathBuf) {
    let cfg = std::env::var("QSC_CONFIG_DIR")
        .ok()
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."));
    (cfg.clone(), cfg.join("vault.qsv"))
}
