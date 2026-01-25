use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use std::path::PathBuf;

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

fn qsc_cmd() -> Command {
    // Use cargo_bin_cmd (non-deprecated; compatible with custom target dirs).
    Command::new(assert_cmd::cargo::cargo_bin("qsc"))
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
            "code=noninteractive_passphrase_required",
        ));

    // No mutation on reject: vault file must not appear.
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
        .env("QSC_PASSPHRASE", pass)
        .args(["vault", "init"]);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("QSC_MARK/1 event=vault_init"));

    let vault_file = cfg.join("vault.qsv");
    assert!(vault_file.exists());

    // Redaction guarantee: vault file must not contain the passphrase bytes.
    let bytes = fs::read(&vault_file).unwrap();
    assert!(!bytes
        .windows(pass.as_bytes().len())
        .any(|w| w == pass.as_bytes()));

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
