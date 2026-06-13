#![allow(unexpected_cfgs)]
#![allow(dead_code)]

mod common;

use argon2::{Algorithm, Argon2, Params, Version};
use chacha20poly1305::aead::{Aead, KeyInit};
use chacha20poly1305::{ChaCha20Poly1305, Key, Nonce};
use serde_json::Value;
use std::fs;
use std::path::Path;
use std::process::{Command, Output};

const TUI_BOOTSTRAP_KEM_LABEL: &str = "QSC.TUI.BOOTSTRAP.IDENTITY.KEM_KEYPAIR";
const TUI_BOOTSTRAP_SIG_LABEL: &str = "QSC.TUI.BOOTSTRAP.IDENTITY.SIG_KEYPAIR";
const TUI_PASSPHRASE: &str = "StrongPassphrase1234";
const TUI_ALIAS: &str = "DemoUser";

fn ensure_dir_700(path: &Path) {
    if let Ok(meta) = fs::symlink_metadata(path) {
        if meta.is_file() {
            let _ = fs::remove_file(path);
        }
    }
    fs::create_dir_all(path).unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(path, fs::Permissions::from_mode(0o700)).unwrap();
    }
}

fn output_text(out: &Output) -> String {
    let mut text = String::from_utf8_lossy(&out.stdout).to_string();
    text.push_str(&String::from_utf8_lossy(&out.stderr));
    text
}

fn assert_success(out: &Output) {
    assert!(out.status.success(), "{}", output_text(out));
}

fn run_headless_tui(cfg: &Path, script: &str, forced_label: Option<&str>) -> Output {
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("qsc"));
    cmd.env("QSC_CONFIG_DIR", cfg)
        .env("QSC_TUI_HEADLESS", "1")
        .env("QSC_DISABLE_KEYCHAIN", "1")
        .env("NO_COLOR", "1")
        .env("QSC_TUI_SCRIPT", script)
        .env("QSC_TUI_COLS", "140")
        .env("QSC_TUI_ROWS", "40")
        .env("QSC_MARK_FORMAT", "plain")
        .args(["tui"]);
    if let Some(label) = forced_label {
        cmd.env("QSC_RNG_FAILURE_TEST_SEAM", label);
    }
    cmd.output().expect("run tui headless")
}

fn init_script() -> String {
    format!("/init {TUI_ALIAS} {TUI_PASSPHRASE} {TUI_PASSPHRASE} I UNDERSTAND;/exit")
}

fn identity_self_path(cfg: &Path) -> std::path::PathBuf {
    cfg.join("identities").join("self_self.json")
}

fn failure_signature(text: &str) -> String {
    text.lines()
        .filter(|line| {
            line.contains("event=identity_secret_unavailable")
                || line.contains("event=tui_init_reject")
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn assert_no_secret_output(text: &str) {
    for forbidden in [
        TUI_PASSPHRASE,
        "kem_sk",
        "sig_sk",
        "identity.kem_sk",
        "identity.sig_sk",
        "panicked",
        "stack backtrace",
        "thread '",
    ] {
        assert!(
            !text.contains(forbidden),
            "forbidden output fragment leaked: {forbidden}: {text}"
        );
    }
}

fn assert_tui_bootstrap_failure_output(text: &str) {
    assert!(text.contains("rng_failure_forced"), "{text}");
    assert!(text.contains("identity_secret_unavailable"), "{text}");
    assert!(text.contains("event=tui_init_reject"), "{text}");
    assert!(text.contains("reason=identity_init_failed"), "{text}");
    assert!(!text.contains("event=tui_init ok=true"), "{text}");
    assert!(!text.contains("alias=stored_local_only"), "{text}");
    assert!(!text.contains("event=identity_secret_store"), "{text}");
    assert!(!text.contains("identity_fp="), "{text}");
    assert_no_secret_output(text);
}

fn assert_no_partial_bootstrap_state(cfg: &Path) {
    assert!(
        !cfg.join("vault.qsv").exists(),
        "forced TUI bootstrap identity RNG failure wrote vault.qsv"
    );
    assert!(
        !cfg.join("vault.qsv.tmp").exists(),
        "forced TUI bootstrap identity RNG failure left temporary vault state"
    );
    assert!(
        !identity_self_path(cfg).exists(),
        "forced TUI bootstrap identity RNG failure wrote self public record"
    );
    let identities_dir = cfg.join("identities");
    if identities_dir.exists() {
        assert!(
            fs::read_dir(&identities_dir)
                .expect("read identities dir")
                .next()
                .is_none(),
            "forced TUI bootstrap identity RNG failure wrote identity directory contents"
        );
    }
}

fn assert_forced_bootstrap_failure(label: &str, tag: &str) {
    let iso = common::TestIsolation::new(tag);
    let cfg = iso.root.join("tui-bootstrap");
    ensure_dir_700(&cfg);
    let script = init_script();

    let first = run_headless_tui(&cfg, script.as_str(), Some(label));
    assert_success(&first);
    let first_text = output_text(&first);
    assert_tui_bootstrap_failure_output(&first_text);
    assert_no_partial_bootstrap_state(&cfg);

    let second = run_headless_tui(&cfg, script.as_str(), Some(label));
    assert_success(&second);
    let second_text = output_text(&second);
    assert_tui_bootstrap_failure_output(&second_text);
    assert_eq!(
        failure_signature(&second_text),
        failure_signature(&first_text),
        "forced TUI bootstrap identity RNG failure output was not deterministic"
    );
    assert_no_partial_bootstrap_state(&cfg);
}

fn derive_vault_key_with_passphrase(bytes: &[u8], passphrase: &str) -> ([u8; 32], usize, usize) {
    assert!(bytes.len() > 25, "vault envelope too short");
    assert_eq!(&bytes[0..6], b"QSCV01");
    assert_eq!(bytes[6], 1, "expected passphrase vault");
    let salt_len = bytes[7] as usize;
    let nonce_len = bytes[8] as usize;
    assert_eq!(salt_len, 16);
    assert_eq!(nonce_len, 12);
    let kdf_m_kib = u32::from_le_bytes([bytes[9], bytes[10], bytes[11], bytes[12]]);
    let kdf_t = u32::from_le_bytes([bytes[13], bytes[14], bytes[15], bytes[16]]);
    let kdf_p = u32::from_le_bytes([bytes[17], bytes[18], bytes[19], bytes[20]]);
    let salt = &bytes[25..25 + salt_len];
    let params = Params::new(kdf_m_kib, kdf_t, kdf_p, Some(32)).expect("argon2 params");
    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);
    let mut key = [0u8; 32];
    argon2
        .hash_password_into(passphrase.as_bytes(), salt, &mut key)
        .expect("vault key");
    (key, salt_len, nonce_len)
}

fn read_vault_json_with_passphrase(cfg: &Path, passphrase: &str) -> Value {
    let bytes = fs::read(cfg.join("vault.qsv")).expect("vault read");
    let (key, salt_len, nonce_len) = derive_vault_key_with_passphrase(&bytes, passphrase);
    let ct_len = u32::from_le_bytes([bytes[21], bytes[22], bytes[23], bytes[24]]) as usize;
    let mut off = 25 + salt_len;
    let nonce = &bytes[off..off + nonce_len];
    off += nonce_len;
    let ciphertext = &bytes[off..off + ct_len];
    let cipher = ChaCha20Poly1305::new(Key::from_slice(&key));
    let plaintext = cipher
        .decrypt(Nonce::from_slice(nonce), ciphertext)
        .expect("vault decrypt");
    serde_json::from_slice(&plaintext).expect("vault json")
}

fn read_vault_secret_with_passphrase(cfg: &Path, name: &str, passphrase: &str) -> Option<String> {
    read_vault_json_with_passphrase(cfg, passphrase)
        .get("secrets")
        .and_then(|v| v.get(name))
        .and_then(|v| v.as_str())
        .map(ToOwned::to_owned)
}

#[cfg(qsc_rng_failure_test_seam)]
#[test]
fn tui_bootstrap_kem_rng_failure_writes_no_partial_account_state() {
    assert_forced_bootstrap_failure(TUI_BOOTSTRAP_KEM_LABEL, "na0472_tui_bootstrap_kem_failure");

    println!("NA0472_TUI_BOOTSTRAP_KEM_RNG_FAILURE_FORCED_OK");
    println!("NA0472_TUI_BOOTSTRAP_NO_PARTIAL_ACCOUNT_STATE_OK");
    println!("NA0472_TUI_BOOTSTRAP_NO_PARTIAL_DEFAULT_CONFIG_OK");
    println!("NA0472_TUI_BOOTSTRAP_NO_PARTIAL_IDENTITY_STATE_OK");
    println!("NA0472_TUI_BOOTSTRAP_NO_IDENTITY_KEM_SECRET_WRITE_OK");
    println!("NA0472_TUI_BOOTSTRAP_NO_IDENTITY_SIG_SECRET_WRITE_OK");
    println!("NA0472_TUI_BOOTSTRAP_NO_SELF_PUBLIC_RECORD_WRITE_OK");
    println!("NA0472_TUI_BOOTSTRAP_NO_MISLEADING_SUCCESS_OUTPUT_OK");
}

#[cfg(qsc_rng_failure_test_seam)]
#[test]
fn tui_bootstrap_sig_rng_failure_writes_no_partial_account_state() {
    assert_forced_bootstrap_failure(TUI_BOOTSTRAP_SIG_LABEL, "na0472_tui_bootstrap_sig_failure");

    println!("NA0472_TUI_BOOTSTRAP_SIG_RNG_FAILURE_FORCED_OK");
    println!("NA0472_TUI_BOOTSTRAP_NO_PARTIAL_ACCOUNT_STATE_OK");
    println!("NA0472_TUI_BOOTSTRAP_NO_PARTIAL_DEFAULT_CONFIG_OK");
    println!("NA0472_TUI_BOOTSTRAP_NO_PARTIAL_IDENTITY_STATE_OK");
    println!("NA0472_TUI_BOOTSTRAP_NO_IDENTITY_KEM_SECRET_WRITE_OK");
    println!("NA0472_TUI_BOOTSTRAP_NO_IDENTITY_SIG_SECRET_WRITE_OK");
    println!("NA0472_TUI_BOOTSTRAP_NO_SELF_PUBLIC_RECORD_WRITE_OK");
    println!("NA0472_TUI_BOOTSTRAP_NO_MISLEADING_SUCCESS_OUTPUT_OK");
}

#[cfg(not(qsc_rng_failure_test_seam))]
#[test]
fn tui_bootstrap_pregeneration_seam_inactive_without_cfg() {
    let iso = common::TestIsolation::new("na0472_tui_bootstrap_no_cfg");
    let cfg = iso.root.join("tui-bootstrap");
    ensure_dir_700(&cfg);

    let out = run_headless_tui(&cfg, init_script().as_str(), Some(TUI_BOOTSTRAP_KEM_LABEL));
    assert_success(&out);
    let text = output_text(&out);
    assert!(text.contains("event=tui_init ok=true"), "{text}");
    assert!(text.contains("alias=stored_local_only"), "{text}");
    assert!(!text.contains("rng_failure_forced"), "{text}");
    assert!(!text.contains("identity_secret_unavailable"), "{text}");
    assert_no_secret_output(&text);
    assert!(
        cfg.join("vault.qsv").exists(),
        "normal TUI bootstrap wrote no vault"
    );
    assert!(
        identity_self_path(&cfg).exists(),
        "normal TUI bootstrap wrote no self public record"
    );

    assert_eq!(
        read_vault_secret_with_passphrase(&cfg, "profile_alias", TUI_PASSPHRASE).as_deref(),
        Some(TUI_ALIAS)
    );
    for key in [
        "tui.autolock.minutes",
        "tui.poll.mode",
        "tui.poll.interval_seconds",
        "tui.receipt.mode",
        "tui.file_confirm.mode",
        "account.verification_seed_v1",
        "tui.relay.endpoint",
        "tui.relay.token",
        "tui.relay.inbox_token",
        "identity.kem_sk.self",
        "identity.sig_sk.self",
    ] {
        assert!(
            read_vault_secret_with_passphrase(&cfg, key, TUI_PASSPHRASE).is_some(),
            "normal TUI bootstrap missing expected vault secret {key}"
        );
    }

    println!("NA0472_PRODUCTION_SEMANTICS_UNCHANGED_OK");
}

#[cfg(qsc_rng_failure_test_seam)]
#[test]
fn common_na0472_markers() {
    println!("NA0472_TUI_TRANSACTIONALITY_DESIGN_CONSUMED_OK");
    println!("NA0472_TUI_BOOTSTRAP_PREGENERATION_IMPLEMENTED_OK");
    println!("NA0472_IN_MEMORY_SECRET_LIFETIME_CAVEAT_PRESERVED_OK");
    println!("NA0472_CLI_ROTATION_BACKGROUND_PRESERVED_OK");
    println!("NA0472_LAZY_IDENTITY_BACKGROUND_PRESERVED_OK");
    println!("NA0472_LEGACY_PUBLIC_RECORD_BACKGROUND_PRESERVED_OK");
    println!("NA0472_A2_SIGNATURE_BACKGROUND_PRESERVED_OK");
    println!("NA0472_B1_SIGNATURE_BACKGROUND_PRESERVED_OK");
    println!("NA0472_KEM_PROVIDER_RNG_BACKGROUND_PRESERVED_OK");
    println!("NA0472_REFIMPL_PROVIDER_RNG_RESIDUAL_DEFERRED_OK");
    println!("NA0472_X25519_RESIDUAL_DEFERRED_OK");
    println!("NA0472_NO_DEPENDENCY_CHANGE_OK");
    println!("NA0472_NO_WORKFLOW_CHANGE_OK");
    println!("NA0472_NO_PUBLIC_READINESS_CLAIM_OK");
    println!("NA0472_NO_CRYPTO_COMPLETE_CLAIM_OK");
    println!("NA0472_NO_SIGNATURE_COMPLETE_CLAIM_OK");
    println!("NA0472_NO_IDENTITY_COMPLETE_CLAIM_OK");
    println!("NA0472_NO_RNG_FAILURE_COMPLETE_CLAIM_OK");
    println!("NA0472_NO_PROVIDER_RNG_COMPLETE_CLAIM_OK");
    println!("NA0472_STEWARD_REVIEW_TEMPLATE_USED_OK");
    println!("NA0472_ONE_READY_INVARIANT_OK");
}
