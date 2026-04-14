mod common;

use argon2::{Algorithm, Argon2, Params, Version};
use chacha20poly1305::aead::{Aead, KeyInit};
use chacha20poly1305::{ChaCha20Poly1305, Key, Nonce};
use predicates::prelude::*;
use std::fs;
use std::path::{Path, PathBuf};

/// Create a writable, safe test root without relying on $HOME.
fn test_root() -> PathBuf {
    if let Ok(v) = std::env::var("QSC_TEST_ROOT") {
        return PathBuf::from(v);
    }
    if let Ok(v) = std::env::var("CARGO_TARGET_DIR") {
        return PathBuf::from(v).join("qsc-test-tmp");
    }
    PathBuf::from("target").join("qsc-test-tmp")
}

fn qsc_cmd() -> assert_cmd::Command {
    assert_cmd::cargo::cargo_bin_cmd!("qsc")
}

fn rewrite_passphrase_vault_profile(
    cfg: &Path,
    passphrase: &str,
    kdf_m_kib: u32,
    kdf_t: u32,
    kdf_p: u32,
) {
    let vault_file = cfg.join("vault.qsv");
    let bytes = fs::read(&vault_file).unwrap();
    assert!(bytes.len() > 39, "vault envelope too short");
    assert_eq!(&bytes[0..6], b"QSCV01");
    assert_eq!(bytes[6], 1, "expected passphrase vault");

    let salt_len = bytes[7] as usize;
    let nonce_len = bytes[8] as usize;
    assert_eq!(salt_len, 16);
    assert_eq!(nonce_len, 12);

    let old_kdf_m_kib = u32::from_le_bytes([bytes[9], bytes[10], bytes[11], bytes[12]]);
    let old_kdf_t = u32::from_le_bytes([bytes[13], bytes[14], bytes[15], bytes[16]]);
    let old_kdf_p = u32::from_le_bytes([bytes[17], bytes[18], bytes[19], bytes[20]]);
    let ct_len = u32::from_le_bytes([bytes[21], bytes[22], bytes[23], bytes[24]]) as usize;

    let salt = &bytes[25..25 + salt_len];
    let mut off = 25 + salt_len;
    let nonce = &bytes[off..off + nonce_len];
    off += nonce_len;
    let ciphertext = &bytes[off..off + ct_len];

    let old_params = Params::new(old_kdf_m_kib, old_kdf_t, old_kdf_p, Some(32)).unwrap();
    let old_argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, old_params);
    let mut old_key = [0u8; 32];
    old_argon2
        .hash_password_into(passphrase.as_bytes(), salt, &mut old_key)
        .unwrap();
    let old_cipher = ChaCha20Poly1305::new(Key::from_slice(&old_key));
    let plaintext = old_cipher
        .decrypt(Nonce::from_slice(nonce), ciphertext)
        .unwrap();

    let new_params = Params::new(kdf_m_kib, kdf_t, kdf_p, Some(32)).unwrap();
    let new_argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, new_params);
    let mut new_key = [0u8; 32];
    new_argon2
        .hash_password_into(passphrase.as_bytes(), salt, &mut new_key)
        .unwrap();
    let new_cipher = ChaCha20Poly1305::new(Key::from_slice(&new_key));
    let new_ciphertext = new_cipher
        .encrypt(Nonce::from_slice(nonce), plaintext.as_ref())
        .unwrap();

    let mut out =
        Vec::with_capacity(6 + 1 + 1 + 1 + 4 * 4 + salt_len + nonce_len + new_ciphertext.len());
    out.extend_from_slice(b"QSCV01");
    out.push(1);
    out.push(16);
    out.push(12);
    out.extend_from_slice(&kdf_m_kib.to_le_bytes());
    out.extend_from_slice(&kdf_t.to_le_bytes());
    out.extend_from_slice(&kdf_p.to_le_bytes());
    out.extend_from_slice(&(new_ciphertext.len() as u32).to_le_bytes());
    out.extend_from_slice(salt);
    out.extend_from_slice(nonce);
    out.extend_from_slice(&new_ciphertext);

    fs::write(&vault_file, out).unwrap();
}

#[test]
fn vault_init_noninteractive_requires_passphrase_no_mutation() {
    let base = test_root().join("na0061_noninteractive_no_pass");
    let _ = fs::remove_dir_all(&base);
    fs::create_dir_all(&base).unwrap();

    let cfg = base.join("cfg");
    fs::create_dir_all(&cfg).unwrap();

    let vault_file = cfg.join("vault.qsv");
    assert!(!vault_file.exists());

    let mut cmd = qsc_cmd();
    cmd.env("QSC_TEST_ROOT", &base)
        .env("QSC_CONFIG_DIR", &cfg)
        .env("QSC_DISABLE_KEYCHAIN", "1")
        .env("QSC_NONINTERACTIVE", "1")
        .args(["vault", "init"]);

    cmd.assert()
        .failure()
        .stdout(predicate::str::contains("QSC_MARK/1 event=error"))
        .stdout(predicate::str::contains(
            "code=vault_passphrase_required_noninteractive",
        ));

    // No mutation on reject: vault file must not appear.
    assert!(!vault_file.exists());
}

#[test]
fn vault_init_invalid_key_source_redacts_stderr() {
    let base = test_root().join("na0069_invalid_key_source");
    let _ = fs::remove_dir_all(&base);
    fs::create_dir_all(&base).unwrap();

    let cfg = base.join("cfg");
    fs::create_dir_all(&cfg).unwrap();

    let secret = "super-secret-passphrase";
    let passphrase_file = common::write_passphrase_file(&cfg, "invalid-key-source", secret);
    let mut cmd = qsc_cmd();
    cmd.env("QSC_TEST_ROOT", &base)
        .env("QSC_CONFIG_DIR", &cfg)
        .args([
            "vault",
            "init",
            "--key-source",
            "bogus",
            "--passphrase-file",
            passphrase_file.to_str().unwrap(),
        ]);

    cmd.assert()
        .failure()
        .stdout(predicate::str::contains("code=key_source_invalid"))
        .stdout(predicate::str::contains(secret).not())
        .stderr(predicate::str::contains(secret).not());

    let vault_file = cfg.join("vault.qsv");
    assert!(!vault_file.exists());
}

#[test]
fn vault_init_with_passphrase_creates_encrypted_file_and_redacts() {
    let base = test_root().join("na0061_passphrase_ok");
    let _ = fs::remove_dir_all(&base);
    fs::create_dir_all(&base).unwrap();

    let cfg = base.join("cfg");
    fs::create_dir_all(&cfg).unwrap();

    let pass = "correct horse battery staple";
    let mut cmd = qsc_cmd();
    cmd.env("QSC_TEST_ROOT", &base)
        .env("QSC_CONFIG_DIR", &cfg)
        .env("QSC_DISABLE_KEYCHAIN", "1")
        .env("QSC_NONINTERACTIVE", "1")
        .args(["vault", "init", "--passphrase-stdin"])
        .write_stdin(pass);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("QSC_MARK/1 event=vault_init"));

    let vault_file = cfg.join("vault.qsv");
    assert!(vault_file.exists());

    // Redaction guarantee: vault file must not contain the passphrase bytes.
    let bytes = fs::read(&vault_file).unwrap();
    assert!(!bytes.windows(pass.len()).any(|w| w == pass.as_bytes()));
    assert!(!bytes
        .windows(b"QSC_TEST_SECRET".len())
        .any(|w| w == b"QSC_TEST_SECRET"));

    // Status must not echo secrets.
    let mut st = qsc_cmd();
    st.env("QSC_TEST_ROOT", &base)
        .env("QSC_CONFIG_DIR", &cfg)
        .args(["vault", "status"]);
    st.assert()
        .success()
        .stdout(predicate::str::contains("QSC_MARK/1 event=vault_status"))
        .stdout(predicate::str::contains("present=true"))
        .stdout(predicate::str::contains(pass).not());
}

#[test]
fn vault_status_missing_fails() {
    let base = test_root().join("na0061_status_missing");
    let _ = fs::remove_dir_all(&base);
    fs::create_dir_all(&base).unwrap();

    let cfg = base.join("cfg");
    fs::create_dir_all(&cfg).unwrap();

    let vault_file = cfg.join("vault.qsv");
    assert!(!vault_file.exists());

    let mut cmd = qsc_cmd();
    cmd.env("QSC_TEST_ROOT", &base)
        .env("QSC_CONFIG_DIR", &cfg)
        .args(["vault", "status"]);

    cmd.assert()
        .failure()
        .stdout(predicate::str::contains("QSC_MARK/1 event=error"))
        .stdout(predicate::str::contains("code=vault_missing"));

    assert!(!vault_file.exists());
}

#[test]
fn vault_init_yubikey_stub_fails_no_mutation() {
    let base = test_root().join("na0062_yubikey_stub");
    let _ = fs::remove_dir_all(&base);
    fs::create_dir_all(&base).unwrap();

    let cfg = base.join("cfg");
    fs::create_dir_all(&cfg).unwrap();

    let vault_file = cfg.join("vault.qsv");
    assert!(!vault_file.exists());

    let mut cmd = qsc_cmd();
    cmd.env("QSC_TEST_ROOT", &base)
        .env("QSC_CONFIG_DIR", &cfg)
        .env("QSC_NONINTERACTIVE", "1")
        .args(["vault", "init", "--key-source", "yubikey"]);

    cmd.assert()
        .failure()
        .stdout(predicate::str::contains("QSC_MARK/1 event=error"))
        .stdout(predicate::str::contains(
            "code=vault_yubikey_not_implemented",
        ));

    assert!(!vault_file.exists());
}

#[test]
fn vault_init_mock_provider_rejected_without_mutation() {
    let base = test_root().join("na0062_mock_provider");
    let _ = fs::remove_dir_all(&base);
    fs::create_dir_all(&base).unwrap();

    let cfg = base.join("cfg");
    fs::create_dir_all(&cfg).unwrap();

    let vault_file = cfg.join("vault.qsv");
    assert!(!vault_file.exists());

    let mut cmd = qsc_cmd();
    cmd.env("QSC_TEST_ROOT", &base)
        .env("QSC_CONFIG_DIR", &cfg)
        .env("QSC_NONINTERACTIVE", "1")
        .args(["vault", "init", "--key-source", "mock"]);

    cmd.assert()
        .failure()
        .stdout(predicate::str::contains("QSC_MARK/1 event=error"))
        .stdout(predicate::str::contains("code=vault_mock_provider_retired"));

    assert!(!vault_file.exists());
}

#[test]
fn bootstrap_unlock_rejects_retired_mock_provider_vault() {
    let base = test_root().join("na0233_bootstrap_rejects_retired_mock_provider");
    let _ = fs::remove_dir_all(&base);
    fs::create_dir_all(&base).unwrap();

    let cfg = base.join("cfg");
    fs::create_dir_all(&cfg).unwrap();
    let passphrase = "test-passphrase";
    common::init_passphrase_vault(&cfg, passphrase);

    let vault_file = cfg.join("vault.qsv");
    let mut bytes = fs::read(&vault_file).unwrap();
    bytes[6] = 4;
    fs::write(&vault_file, bytes).unwrap();

    let mut status = qsc_cmd();
    status
        .env("QSC_TEST_ROOT", &base)
        .env("QSC_CONFIG_DIR", &cfg)
        .args(["vault", "status"]);
    status
        .assert()
        .success()
        .stdout(predicate::str::contains("key_source=mock_retired"));

    let mut rotate = qsc_cmd();
    rotate
        .env("QSC_TEST_ROOT", &base)
        .env("QSC_CONFIG_DIR", &cfg)
        .env("QSC_DESKTOP_SESSION_PASSPHRASE", passphrase)
        .args(["--unlock-passphrase-env", "QSC_DESKTOP_SESSION_PASSPHRASE"]);
    rotate.args(["identity", "rotate", "--confirm"]);
    rotate
        .assert()
        .failure()
        .stdout(predicate::str::contains("code=vault_mock_provider_retired"));

    assert!(!cfg.join("self.identity").exists());
}

#[test]
fn vault_unlock_rejects_below_floor_passphrase_profile_without_mutation() {
    let base = test_root().join("na0234_rejects_below_floor_profile");
    let _ = fs::remove_dir_all(&base);
    fs::create_dir_all(&base).unwrap();

    let cfg = base.join("cfg");
    fs::create_dir_all(&cfg).unwrap();
    let passphrase = "test-passphrase";
    common::init_passphrase_vault(&cfg, passphrase);
    rewrite_passphrase_vault_profile(&cfg, passphrase, 4096, 1, 1);

    let vault_file = cfg.join("vault.qsv");
    let before = fs::read(&vault_file).unwrap();
    let passphrase_file = common::write_passphrase_file(&cfg, "na0234-below-floor", passphrase);

    let mut unlock = qsc_cmd();
    unlock
        .env("QSC_TEST_ROOT", &base)
        .env("QSC_CONFIG_DIR", &cfg)
        .args([
            "vault",
            "unlock",
            "--non-interactive",
            "--passphrase-file",
            passphrase_file.to_str().unwrap(),
        ]);

    unlock
        .assert()
        .failure()
        .stdout(predicate::str::contains("code=vault_parse_failed"));

    assert_eq!(before, fs::read(&vault_file).unwrap());
}

#[test]
fn vault_unlock_rejects_noncanonical_passphrase_profile_without_mutation() {
    let base = test_root().join("na0234_rejects_noncanonical_profile");
    let _ = fs::remove_dir_all(&base);
    fs::create_dir_all(&base).unwrap();

    let cfg = base.join("cfg");
    fs::create_dir_all(&cfg).unwrap();
    let passphrase = "test-passphrase";
    common::init_passphrase_vault(&cfg, passphrase);
    rewrite_passphrase_vault_profile(&cfg, passphrase, 19456, 3, 1);

    let vault_file = cfg.join("vault.qsv");
    let before = fs::read(&vault_file).unwrap();
    let passphrase_file = common::write_passphrase_file(&cfg, "na0234-noncanonical", passphrase);

    let mut unlock = qsc_cmd();
    unlock
        .env("QSC_TEST_ROOT", &base)
        .env("QSC_CONFIG_DIR", &cfg)
        .args([
            "vault",
            "unlock",
            "--non-interactive",
            "--passphrase-file",
            passphrase_file.to_str().unwrap(),
        ]);

    unlock
        .assert()
        .failure()
        .stdout(predicate::str::contains("code=vault_parse_failed"));

    assert_eq!(before, fs::read(&vault_file).unwrap());
}

#[test]
fn vault_init_without_qsc_config_dir_uses_xdg_or_home_not_cwd() {
    let mut base = test_root().join("na0109_vault_default_path");
    if base.is_relative() {
        base = std::env::current_dir().unwrap().join(base);
    }
    let _ = fs::remove_dir_all(&base);
    fs::create_dir_all(&base).unwrap();

    let cwd = base.join("repo_like_cwd");
    let home = base.join("home");
    fs::create_dir_all(&cwd).unwrap();
    fs::create_dir_all(&home).unwrap();

    let expected = home.join(".config").join("qsc").join("vault.qsv");
    let unexpected = cwd.join("vault.qsv");
    assert!(!expected.exists());
    assert!(!unexpected.exists());

    let mut cmd = qsc_cmd();
    let pass = "default-path-passphrase";
    cmd.current_dir(&cwd)
        .env("QSC_TEST_ROOT", &base)
        .env("HOME", &home)
        .env("QSC_DISABLE_KEYCHAIN", "1")
        .env("QSC_NONINTERACTIVE", "1")
        .env_remove("QSC_CONFIG_DIR")
        .env_remove("XDG_CONFIG_HOME")
        .args(["vault", "init", "--passphrase-stdin"])
        .write_stdin(pass);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("QSC_MARK/1 event=vault_init"));

    assert!(expected.exists());
    assert!(!unexpected.exists());
}
