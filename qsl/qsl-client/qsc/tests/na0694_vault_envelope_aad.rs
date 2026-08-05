// NA-0694 (D628, D-1334): ENG-0107 — the vault envelope's entire 53-byte header
// (magic ‖ key_source ‖ salt_len ‖ nonce_len ‖ KDF M/T/P ‖ ct_len ‖ salt ‖ nonce) is
// bound as ChaCha20Poly1305 associated data at every encrypt and the decrypt; the magic
// is QSCV02 (hard break, no migration); a recognized QSCV01 envelope refuses with its
// own name at unlock AND status; the KDF-profile check applies to BOTH key sources.
//
// The five instruments and their honest red-stories (D628 §4c, Ruling B):
// (i)   roundtrip — red if the encrypt-side AAD and the decrypt-side AAD rebuild EVER
//       diverge (the single-builder property this lane exists to hold).
// (ii)  key_source byte tamper — behavior pin: detected today by derivation failure
//       (keychain load fails headless) AND post-change by AAD; deliberately NOT
//       red-capable against Ruling 1 alone (D628 §0.4).
// (iii) KDF-param byte tamper — behavior pin: the canonical-profile check fires first;
//       red only if the profile check AND the AAD binding both vanish.
// (iv)  ⚠ THE load-bearing Ruling-1 instrument: a structurally canonical, correctly
//       keyed QSCV02 envelope encrypted with EMPTY AAD must be REFUSED — every
//       pre-AEAD check accepts it, so only the header binding can turn it away
//       (negative-control A reds exactly this test and nothing else).
// (v)   the Ruling-3 instrument: a QSCV01-magic envelope refuses with exactly
//       `vault_version_unsupported` — not wrong-passphrase, not corrupt — at unlock
//       AND at vault status (negative-control B reds exactly this test).
//
// Harness: the NA-0693 pattern — every test serializes on ENV_LOCK, fixtures live in
// fresh per-test dirs chmod 0700 after create_dir_all (the Slice-B W1/X1 lesson: N-04
// refuses loose fixture ancestry), and the product is driven through the CLI/pub
// surface only.

mod common;

use argon2::{Algorithm, Argon2, Params, Version};
use chacha20poly1305::aead::{Aead, KeyInit};
use chacha20poly1305::{ChaCha20Poly1305, Key, Nonce};
use predicates::prelude::*;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, MutexGuard, OnceLock};

const PASS: &str = "na0694-envelope-aad-pass";
const KDF_M_KIB: u32 = 19456;
const KDF_T: u32 = 2;
const KDF_P: u32 = 1;
const HEADER_LEN: usize = 53;

static ENV_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

fn env_lock() -> MutexGuard<'static, ()> {
    ENV_LOCK
        .get_or_init(|| Mutex::new(()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

fn ensure_dir_700(path: &Path) {
    fs::create_dir_all(path).unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(path, fs::Permissions::from_mode(0o700)).unwrap();
    }
}

fn test_root() -> PathBuf {
    let root = if let Ok(v) = std::env::var("QSC_TEST_ROOT") {
        PathBuf::from(v)
    } else if let Ok(v) = std::env::var("CARGO_TARGET_DIR") {
        PathBuf::from(v)
    } else {
        PathBuf::from("target")
    };
    let root = root.join("qsc-test-tmp").join("na0694-vault-envelope-aad");
    ensure_dir_700(&root);
    root
}

fn fresh_case(tag: &str) -> (PathBuf, PathBuf) {
    let base = test_root().join(format!("{}_{}", tag, std::process::id()));
    if base.exists() {
        fs::remove_dir_all(&base).unwrap();
    }
    ensure_dir_700(&base);
    let cfg = base.join("cfg");
    ensure_dir_700(&cfg);
    (base, cfg)
}

fn qsc_cmd() -> assert_cmd::Command {
    assert_cmd::cargo::cargo_bin_cmd!("qsc")
}

fn unlock_cmd(base: &Path, cfg: &Path, pass: &str) -> assert_cmd::Command {
    let passphrase_file = common::write_passphrase_file(cfg, "na0694-unlock", pass);
    let mut cmd = qsc_cmd();
    cmd.env("QSC_TEST_ROOT", base)
        .env("QSC_CONFIG_DIR", cfg)
        .env("QSC_DISABLE_KEYCHAIN", "1")
        .args([
            "vault",
            "unlock",
            "--non-interactive",
            "--passphrase-file",
            passphrase_file.to_str().unwrap(),
        ]);
    cmd
}

/// (i) The positive control, driven through the pub surface: init writes a QSCV02
/// envelope, a secret write re-encrypts it (both bound), and unlock + read-back
/// re-derives the AAD from parsed state. Green only while every serializer and both
/// AAD sides route through the ONE header builder.
#[test]
fn qscv02_roundtrip_unlocks_and_reads_back() {
    let _g = env_lock();
    let (_base, cfg) = fresh_case("roundtrip");
    std::env::set_var("QSC_CONFIG_DIR", &cfg);
    std::env::set_var("QSC_DISABLE_KEYCHAIN", "1");
    qsc::vault::set_process_passphrase(None);
    qsc::set_vault_unlocked(false);

    qsc::vault::vault_init_with_passphrase(PASS).expect("vault init");
    let bytes = fs::read(cfg.join("vault.qsv")).expect("vault read after init");
    assert_eq!(&bytes[..6], b"QSCV02", "product-written magic after init");

    qsc::vault::unlock_with_passphrase(PASS).expect("unlock after init");
    qsc::vault::secret_set("na0694.probe", "roundtrip-value").expect("secret set");
    let bytes = fs::read(cfg.join("vault.qsv")).expect("vault read after set");
    assert_eq!(
        &bytes[..6],
        b"QSCV02",
        "product-written magic after rewrite"
    );

    let got = qsc::vault::secret_get("na0694.probe").expect("secret get");
    assert_eq!(got.as_deref(), Some("roundtrip-value"));
}

/// (ii) Behavior pin: flipping the key_source byte — the byte that selects the ENTIRE
/// key-derivation path — fails closed and mutates nothing. Honest detection story:
/// keychain load fails headless BEFORE the AAD is consulted, so this pin is not
/// red-capable against Ruling 1 alone (§0.4); post-change the AAD also refuses it.
/// NA-0696 (D630 A2.2, D-1336): the refusal is TWO-LAYER — a key_source tamper is
/// intercepted at KEY LOAD, and on a headless build that reads the load-class marker
/// `vault_token_unavailable` (the D5 split retired the `vault_locked` collapse); every
/// tamper that reaches decrypt still fails the AAD as `vault_locked`. The pinned
/// marker is deliberately EXACT: the suite build being headless is itself a pinned
/// premise, and a future keychain-enabled suite build SHOULD turn this red and force
/// the re-census.
#[test]
fn qscv02_key_source_byte_tamper_fails_closed() {
    let _g = env_lock();
    let (base, cfg) = fresh_case("key_source_tamper");
    common::init_passphrase_vault(&cfg, PASS);

    let vault_file = cfg.join("vault.qsv");
    let mut bytes = fs::read(&vault_file).expect("vault read");
    assert_eq!(bytes[6], 1, "product wrote a passphrase envelope");
    bytes[6] = 2;
    fs::write(&vault_file, &bytes).expect("tamper write");

    unlock_cmd(&base, &cfg, PASS)
        .assert()
        .failure()
        .stdout(predicate::str::contains("code=vault_token_unavailable"));

    assert_eq!(
        bytes,
        fs::read(&vault_file).expect("vault re-read"),
        "fail-closed: the unlock attempt must not mutate the envelope"
    );
}

/// (iii) Behavior pin: flipping one KDF_M_KIB byte refuses without mutation. Honest
/// detection story: the canonical-profile check (its key_source gate removed by this
/// lane, N-06) fires before any derivation; red only if that check AND the AAD both
/// vanish.
#[test]
fn qscv02_kdf_param_byte_tamper_fails_closed() {
    let _g = env_lock();
    let (base, cfg) = fresh_case("kdf_param_tamper");
    common::init_passphrase_vault(&cfg, PASS);

    let vault_file = cfg.join("vault.qsv");
    let mut bytes = fs::read(&vault_file).expect("vault read");
    bytes[9] ^= 0x01;
    fs::write(&vault_file, &bytes).expect("tamper write");

    unlock_cmd(&base, &cfg, PASS)
        .assert()
        .failure()
        .stdout(predicate::str::contains("code=vault_parse_failed"));

    assert_eq!(
        bytes,
        fs::read(&vault_file).expect("vault re-read"),
        "fail-closed: the unlock attempt must not mutate the envelope"
    );
}

/// (iv) ⚠ THE load-bearing Ruling-1 instrument (D628 §0.4, Ruling B): a hand-built,
/// structurally canonical QSCV02 envelope — correct salt-derived key, canonical KDF
/// params, valid payload JSON — encrypted with the two-argument (EMPTY-AAD) form the
/// product no longer uses. Every pre-AEAD check accepts it; only the header binding
/// can refuse it. Revert Ruling 1 (negative-control A) and this test goes red alone.
#[test]
fn qscv02_unauthenticated_envelope_refused() {
    let _g = env_lock();
    let (base, cfg) = fresh_case("unauthenticated_envelope");

    let salt = [0x5a_u8; 16];
    let nonce = [0x0b_u8; 12];
    let params = Params::new(KDF_M_KIB, KDF_T, KDF_P, Some(32)).expect("argon2 params");
    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);
    let mut key = [0u8; 32];
    argon2
        .hash_password_into(PASS.as_bytes(), &salt, &mut key)
        .expect("vault key");
    let cipher = ChaCha20Poly1305::new(Key::from_slice(&key));
    let plaintext = br#"{"version":1,"secrets":{}}"#;
    let ciphertext = cipher
        .encrypt(Nonce::from_slice(&nonce), plaintext.as_slice())
        .expect("empty-aad encrypt");

    let mut out = Vec::with_capacity(HEADER_LEN + ciphertext.len());
    out.extend_from_slice(b"QSCV02");
    out.push(1);
    out.push(16);
    out.push(12);
    out.extend_from_slice(&KDF_M_KIB.to_le_bytes());
    out.extend_from_slice(&KDF_T.to_le_bytes());
    out.extend_from_slice(&KDF_P.to_le_bytes());
    out.extend_from_slice(&(ciphertext.len() as u32).to_le_bytes());
    out.extend_from_slice(&salt);
    out.extend_from_slice(&nonce);
    assert_eq!(out.len(), HEADER_LEN, "canonical header width");
    out.extend_from_slice(&ciphertext);

    let vault_file = cfg.join("vault.qsv");
    fs::write(&vault_file, &out).expect("write hand-built envelope");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&vault_file, fs::Permissions::from_mode(0o600)).unwrap();
    }

    // Refused as an AEAD failure (vault_locked) — not a parse failure: the envelope is
    // structurally perfect and its key derives; the missing binding is the only cause.
    unlock_cmd(&base, &cfg, PASS)
        .assert()
        .failure()
        .stdout(predicate::str::contains("code=vault_locked"));
}

/// (v) The Ruling-3 instrument: a recognized-but-old QSCV01 envelope refuses with
/// exactly its own name — quiet, distinct, fail-closed — at unlock AND at status
/// (Ruling A: one cause, one name, both sites). Revert the three-way version arm
/// (negative-control B) and this test goes red alone.
#[test]
fn qscv01_vault_refused_with_distinct_error() {
    let _g = env_lock();
    let (base, cfg) = fresh_case("qscv01_distinct");
    common::init_passphrase_vault(&cfg, PASS);

    let vault_file = cfg.join("vault.qsv");
    let mut bytes = fs::read(&vault_file).expect("vault read");
    assert_eq!(&bytes[..6], b"QSCV02", "product wrote the current magic");
    bytes[..6].copy_from_slice(b"QSCV01");
    fs::write(&vault_file, &bytes).expect("downgrade write");

    unlock_cmd(&base, &cfg, PASS)
        .assert()
        .failure()
        .stdout(predicate::str::contains("code=vault_version_unsupported"))
        .stdout(predicate::str::contains("code=vault_locked").not())
        .stdout(predicate::str::contains("code=vault_parse_failed").not());

    let mut st = qsc_cmd();
    st.env("QSC_TEST_ROOT", &base)
        .env("QSC_CONFIG_DIR", &cfg)
        .args(["vault", "status"]);
    st.assert()
        .failure()
        .stdout(predicate::str::contains("code=vault_version_unsupported"))
        .stdout(predicate::str::contains("code=vault_parse_failed").not());
}
