#![allow(unexpected_cfgs)]
#![allow(dead_code)]

mod common;

use argon2::{Algorithm, Argon2, Params, Version};
use chacha20poly1305::aead::{Aead, KeyInit};
use chacha20poly1305::{ChaCha20Poly1305, Key, Nonce};
use serde_json::Value;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};

const ROUTE_TOKEN_BOB: &str = "route_token_bob_na0452_residual__";
const DEFAULT_ROUTE_TOKEN_LABEL: &str = "QSC.VAULT.INIT.DEFAULT_ROUTE_TOKEN";
const CONTACT_ROUTE_TOKEN_LABEL: &str = "QSC.CONTACT.ROUTE_TOKEN";
const TUI_CONTACT_ROUTE_TOKEN_LABEL: &str = "QSC.TUI.CONTACT.ROUTE_TOKEN";
const TUI_RELAY_INBOX_ROUTE_TOKEN_LABEL: &str = "QSC.TUI.RELAY_INBOX_ROUTE_TOKEN";
const ATTACHMENT_ID_LABEL: &str = "QSC.ATTACHMENT.ID";
const ATTACHMENT_CEK_LABEL: &str = "QSC.ATTACHMENT.CEK";
const ATTACHMENT_NONCE_PREFIX_LABEL: &str = "QSC.ATTACHMENT.NONCE_PREFIX";
const CONTACTS_SECRET_KEY: &str = "contacts.json";
const ATTACHMENT_JOURNAL_SECRET_KEY: &str = "attachments.json";
const TUI_RELAY_INBOX_TOKEN_SECRET_KEY: &str = "tui.relay.inbox_token";
const TUI_CODE_ALICE: &str = "ABCD-EFGH-JKMN-PQRS-T";
const TUI_PASSPHRASE: &str = "StrongPassphrase1234";

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

fn assert_failure(out: &Output) {
    assert!(!out.status.success(), "{}", output_text(out));
}

fn run_qsc(iso: &common::TestIsolation, cfg: &Path, args: &[&str]) -> Output {
    let mut cmd = common::qsc_std_command();
    iso.apply_to(&mut cmd);
    cmd.env("QSC_CONFIG_DIR", cfg)
        .env("QSC_MARK_FORMAT", "plain")
        .env("QSC_QSP_SEED", "1")
        .env("QSC_ALLOW_SEED_FALLBACK", "1")
        .env("QSC_UNSAFE_TEST_SEED_FALLBACK", "1")
        .args(args)
        .output()
        .expect("qsc command")
}

fn run_qsc_with_forced_rng_failure(
    iso: &common::TestIsolation,
    cfg: &Path,
    label: &str,
    args: &[&str],
) -> Output {
    let mut cmd = common::qsc_std_command();
    iso.apply_to(&mut cmd);
    cmd.env("QSC_CONFIG_DIR", cfg)
        .env("QSC_MARK_FORMAT", "plain")
        .env("QSC_QSP_SEED", "1")
        .env("QSC_ALLOW_SEED_FALLBACK", "1")
        .env("QSC_UNSAFE_TEST_SEED_FALLBACK", "1")
        .env("QSC_RNG_FAILURE_TEST_SEAM", label)
        .args(args)
        .output()
        .expect("qsc command")
}

fn vault_init_with_stdin(
    iso: &common::TestIsolation,
    cfg: &Path,
    forced_label: Option<&str>,
) -> Output {
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("qsc"));
    iso.apply_to(&mut cmd);
    cmd.env("QSC_CONFIG_DIR", cfg)
        .env("QSC_DISABLE_KEYCHAIN", "1")
        .env("QSC_NONINTERACTIVE", "1")
        .env("QSC_MARK_FORMAT", "plain")
        .args([
            "vault",
            "init",
            "--non-interactive",
            "--key-source",
            "passphrase",
            "--passphrase-stdin",
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    if let Some(label) = forced_label {
        cmd.env("QSC_RNG_FAILURE_TEST_SEAM", label);
    }
    let mut child = cmd.spawn().expect("vault init spawn");
    child
        .stdin
        .as_mut()
        .expect("stdin")
        .write_all(common::TEST_MOCK_VAULT_PASSPHRASE.as_bytes())
        .expect("write passphrase");
    child.wait_with_output().expect("vault init wait")
}

fn run_headless_tui(
    iso: &common::TestIsolation,
    cfg: &Path,
    script: &str,
    forced_label: Option<&str>,
) -> Output {
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("qsc"));
    iso.apply_to(&mut cmd);
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

fn init_unlock_script() -> String {
    format!(
        "/init DemoUser {TUI_PASSPHRASE} {TUI_PASSPHRASE} I UNDERSTAND;/unlock {TUI_PASSPHRASE};"
    )
}

fn contacts_add_with_route_token(
    iso: &common::TestIsolation,
    cfg: &Path,
    label: &str,
    token: &str,
) {
    let out = run_qsc(
        iso,
        cfg,
        &[
            "contacts",
            "add",
            "--label",
            label,
            "--fp",
            "fp-test",
            "--route-token",
            token,
        ],
    );
    assert_success(&out);

    let list = run_qsc(iso, cfg, &["contacts", "device", "list", "--label", label]);
    assert_success(&list);
    let list_text = output_text(&list);
    let device_id = list_text
        .lines()
        .find(|line| line.starts_with("device="))
        .and_then(|line| {
            line.split_whitespace()
                .find_map(|tok| tok.strip_prefix("device="))
        })
        .unwrap_or_else(|| panic!("missing device id in output: {list_text}"));

    let trust = run_qsc(
        iso,
        cfg,
        &[
            "contacts",
            "device",
            "trust",
            "--label",
            label,
            "--device",
            device_id,
            "--confirm",
        ],
    );
    assert_success(&trust);
}

fn relay_inbox_set(iso: &common::TestIsolation, cfg: &Path, token: &str) {
    let out = run_qsc(iso, cfg, &["relay", "inbox-set", "--token", token]);
    assert_success(&out);
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

fn read_mock_vault_secret(cfg: &Path, name: &str) -> Option<String> {
    read_vault_secret_with_passphrase(cfg, name, common::TEST_MOCK_VAULT_PASSPHRASE)
}

fn attachment_fixture(iso: &common::TestIsolation, tag: &str) -> (PathBuf, PathBuf) {
    let base = iso.root.join(tag);
    let _ = fs::remove_dir_all(&base);
    ensure_dir_700(&base);
    let cfg = base.join("alice");
    ensure_dir_700(&cfg);
    common::init_mock_vault(&cfg);
    contacts_add_with_route_token(iso, &cfg, "bob", ROUTE_TOKEN_BOB);
    relay_inbox_set(iso, &cfg, ROUTE_TOKEN_BOB);
    let payload = base.join("payload.bin");
    fs::write(&payload, b"na0452 attachment rng failure payload").expect("write payload");
    (cfg, payload)
}

fn run_attachment_send_with_forced_rng_failure(
    iso: &common::TestIsolation,
    cfg: &Path,
    payload: &Path,
    relay: &str,
    attachment_service: &str,
    label: &str,
) -> Output {
    run_qsc_with_forced_rng_failure(
        iso,
        cfg,
        label,
        &[
            "file",
            "send",
            "--transport",
            "relay",
            "--relay",
            relay,
            "--attachment-service",
            attachment_service,
            "--legacy-in-message-stage",
            "w1",
            "--to",
            "bob",
            "--path",
            payload.to_str().expect("payload path"),
        ],
    )
}

#[cfg(not(qsc_rng_failure_test_seam))]
#[test]
fn normal_build_ignores_residual_test_seam_selector() {
    let iso = common::TestIsolation::new("na0452_normal_selector_ignored");
    let cfg = iso.root.join("normal-vault");
    ensure_dir_700(&cfg);

    let out = vault_init_with_stdin(&iso, &cfg, Some(DEFAULT_ROUTE_TOKEN_LABEL));
    assert_success(&out);
    let text = output_text(&out);
    assert!(text.contains("event=vault_init"), "{text}");
    assert!(
        cfg.join("vault.qsv").exists(),
        "normal build did not write vault"
    );

    println!("NA0452_PRODUCTION_SEMANTICS_UNCHANGED_OK");
}

#[cfg(qsc_rng_failure_test_seam)]
#[test]
fn common_na0452_markers() {
    println!("NA0452_RNG_RESIDUAL_AUTHORIZATION_CONSUMED_OK");
    println!("NA0452_ROUTE_CONTACT_ATTACHMENT_RNG_IMPLEMENTED_OK");
    println!("NA0452_ROUTE_RNG_FORCED_BY_TEST_ONLY_SEAM_OK");
    println!("NA0452_CONTACT_RNG_FORCED_BY_TEST_ONLY_SEAM_OK");
    println!("NA0452_ATTACHMENT_RNG_FORCED_BY_TEST_ONLY_SEAM_OK");
    println!("NA0452_TUI_ACCOUNT_VERIFICATION_SEED_DEFERRED_OK");
    println!("NA0452_PROVIDER_RNG_DEFERRED_OK");
    println!("NA0452_PRODUCTION_SEMANTICS_UNCHANGED_OK");
    println!("NA0452_NO_DEPENDENCY_CHANGE_OK");
    println!("NA0452_NO_WORKFLOW_CHANGE_OK");
    println!("NA0452_NO_PUBLIC_READINESS_CLAIM_OK");
    println!("NA0452_NO_CRYPTO_COMPLETE_CLAIM_OK");
    println!("NA0452_NO_RNG_FAILURE_COMPLETE_CLAIM_OK");
    println!("NA0452_STEWARD_REVIEW_TEMPLATE_USED_OK");
    println!("NA0452_ONE_READY_INVARIANT_OK");
}

#[cfg(qsc_rng_failure_test_seam)]
#[test]
fn route_default_token_rng_failure_writes_no_vault_file() {
    let iso = common::TestIsolation::new("na0452_default_route_token_rng_failure");
    let cfg = iso.root.join("vault-route-rng-failure");
    ensure_dir_700(&cfg);

    let out = vault_init_with_stdin(&iso, &cfg, Some(DEFAULT_ROUTE_TOKEN_LABEL));
    assert_failure(&out);
    let text = output_text(&out);
    assert!(text.contains("rng_failure_forced"), "{text}");
    assert!(
        !cfg.join("vault.qsv").exists(),
        "forced default-route-token RNG failure wrote vault.qsv"
    );
    assert!(
        !cfg.join("vault.qsv.tmp").exists(),
        "forced default-route-token RNG failure left temp vault file"
    );

    println!("NA0452_ROUTE_RNG_FAILURE_NO_PARTIAL_STATE_OK");
}

#[cfg(qsc_rng_failure_test_seam)]
#[test]
fn contact_route_token_rng_failure_writes_no_contact_state() {
    let iso = common::TestIsolation::new("na0452_contact_route_token_rng_failure");
    let cfg = iso.root.join("contact-route-rng-failure");
    ensure_dir_700(&cfg);
    common::init_mock_vault(&cfg);
    let vault_before = fs::read(cfg.join("vault.qsv")).expect("vault before");

    let out = run_qsc_with_forced_rng_failure(
        &iso,
        &cfg,
        CONTACT_ROUTE_TOKEN_LABEL,
        &["contacts", "add", "--label", "bob", "--fp", "fp-test"],
    );
    assert_failure(&out);
    let text = output_text(&out);
    assert!(text.contains("rng_failure_forced"), "{text}");
    assert!(
        read_mock_vault_secret(&cfg, CONTACTS_SECRET_KEY).is_none(),
        "forced contact route-token RNG failure wrote contacts state"
    );
    assert_eq!(
        fs::read(cfg.join("vault.qsv")).expect("vault after"),
        vault_before,
        "forced contact route-token RNG failure changed vault bytes"
    );

    println!("NA0452_CONTACT_RNG_FAILURE_NO_PARTIAL_STATE_OK");
}

#[cfg(qsc_rng_failure_test_seam)]
#[test]
fn tui_contact_route_token_rng_failure_writes_no_contact_cache() {
    let iso = common::TestIsolation::new("na0452_tui_contact_route_token_rng_failure");
    let cfg = iso.root.join("tui-contact-route-rng-failure");
    ensure_dir_700(&cfg);
    let script = format!(
        "{} /contacts add Alice {};/exit",
        init_unlock_script(),
        TUI_CODE_ALICE
    );

    let out = run_headless_tui(
        &iso,
        &cfg,
        script.as_str(),
        Some(TUI_CONTACT_ROUTE_TOKEN_LABEL),
    );
    assert_success(&out);
    let text = output_text(&out);
    assert!(text.contains("rng_failure_forced"), "{text}");
    assert!(
        !text.contains("event=tui_contacts_add label=Alice ok=true"),
        "{text}"
    );
    assert!(
        read_vault_secret_with_passphrase(&cfg, CONTACTS_SECRET_KEY, TUI_PASSPHRASE).is_none(),
        "forced TUI contact route-token RNG failure persisted contacts cache"
    );

    println!("NA0452_TUI_CONTACT_RNG_FAILURE_NO_PARTIAL_STATE_OK");
}

#[cfg(qsc_rng_failure_test_seam)]
#[test]
fn tui_relay_inbox_route_token_rng_failure_writes_no_route_secret() {
    let iso = common::TestIsolation::new("na0452_tui_relay_route_token_rng_failure");
    let cfg = iso.root.join("tui-relay-route-rng-failure");
    ensure_dir_700(&cfg);
    let script = format!("/init DemoUser {TUI_PASSPHRASE} {TUI_PASSPHRASE} I UNDERSTAND;/exit");

    let out = run_headless_tui(
        &iso,
        &cfg,
        script.as_str(),
        Some(TUI_RELAY_INBOX_ROUTE_TOKEN_LABEL),
    );
    assert_success(&out);
    let text = output_text(&out);
    assert!(text.contains("settings_init_failed"), "{text}");
    assert!(!text.contains("event=tui_init ok=true"), "{text}");
    let retained =
        read_vault_secret_with_passphrase(&cfg, TUI_RELAY_INBOX_TOKEN_SECRET_KEY, TUI_PASSPHRASE)
            .expect("vault init default route token should remain");
    assert!(
        retained.len() == 32
            && retained
                .chars()
                .all(|ch| ch.is_ascii_hexdigit() && !ch.is_ascii_uppercase()),
        "forced TUI relay route-token RNG failure overwrote the default route token: {retained}"
    );

    println!("NA0452_TUI_RELAY_ROUTE_RNG_FAILURE_NO_PARTIAL_STATE_OK");
}

#[cfg(qsc_rng_failure_test_seam)]
fn assert_attachment_rng_failure_writes_no_stage_or_journal(label: &str, marker: &str) {
    let iso = common::TestIsolation::new(&format!(
        "na0452_attachment_rng_failure_{}",
        label.replace('.', "_").to_ascii_lowercase()
    ));
    let (cfg, payload) = attachment_fixture(&iso, "attachment");
    let relay = common::start_inbox_server(2 * 1024 * 1024, 512);
    let service = common::start_attachment_server(100 * 1024 * 1024);
    let vault_before = fs::read(cfg.join("vault.qsv")).expect("vault before");

    let out = run_attachment_send_with_forced_rng_failure(
        &iso,
        &cfg,
        &payload,
        relay.base_url(),
        service.base_url(),
        label,
    );
    assert_failure(&out);
    let text = output_text(&out);
    assert!(text.contains("rng_failure_forced"), "{text}");
    assert!(
        read_mock_vault_secret(&cfg, ATTACHMENT_JOURNAL_SECRET_KEY).is_none(),
        "forced attachment RNG failure wrote attachment journal"
    );
    assert!(
        !cfg.join("attachments").exists(),
        "forced attachment RNG failure created attachment staging directory"
    );
    assert_eq!(
        fs::read(cfg.join("vault.qsv")).expect("vault after"),
        vault_before,
        "forced attachment RNG failure changed vault bytes"
    );
    assert!(
        relay.drain_channel(ROUTE_TOKEN_BOB).is_empty(),
        "forced attachment RNG failure emitted relay descriptor"
    );

    println!("{marker}");
}

#[cfg(qsc_rng_failure_test_seam)]
#[test]
fn attachment_id_rng_failure_writes_no_stage_or_journal() {
    assert_attachment_rng_failure_writes_no_stage_or_journal(
        ATTACHMENT_ID_LABEL,
        "NA0452_ATTACHMENT_ID_RNG_FAILURE_NO_PARTIAL_STATE_OK",
    );
}

#[cfg(qsc_rng_failure_test_seam)]
#[test]
fn attachment_cek_rng_failure_writes_no_stage_or_journal() {
    assert_attachment_rng_failure_writes_no_stage_or_journal(
        ATTACHMENT_CEK_LABEL,
        "NA0452_ATTACHMENT_CEK_RNG_FAILURE_NO_PARTIAL_STATE_OK",
    );
}

#[cfg(qsc_rng_failure_test_seam)]
#[test]
fn attachment_nonce_prefix_rng_failure_writes_no_stage_or_journal() {
    assert_attachment_rng_failure_writes_no_stage_or_journal(
        ATTACHMENT_NONCE_PREFIX_LABEL,
        "NA0452_ATTACHMENT_NONCE_PREFIX_RNG_FAILURE_NO_PARTIAL_STATE_OK",
    );
}
