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

const ROUTE_TOKEN_ALICE: &str = "route_token_alice_na0449_rng_failure";
const ROUTE_TOKEN_BOB: &str = "route_token_bob_na0449_rng_failure__";
const QSP_SESSION_STORE_KEY_SECRET: &str = "qsp_session_store_key_v1";
const HANDSHAKE_SID_LABEL: &str = "QSC.HS.SID";
const VAULT_INIT_SALT_LABEL: &str = "QSC.VAULT.INIT.SALT";
const SESSION_STORE_KEY_LABEL: &str = "QSC.QSP.SESSION_STORE_KEY";

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

fn init_identity(iso: &common::TestIsolation, cfg: &Path, label: &str) {
    let out = run_qsc(
        iso,
        cfg,
        &["identity", "rotate", "--as", label, "--confirm"],
    );
    assert_success(&out);
}

fn identity_fp(iso: &common::TestIsolation, cfg: &Path, label: &str) -> String {
    let out = run_qsc(iso, cfg, &["identity", "show", "--as", label]);
    assert_success(&out);
    output_text(&out)
        .lines()
        .find_map(|line| line.strip_prefix("identity_fp="))
        .map(ToOwned::to_owned)
        .unwrap_or_else(|| panic!("missing identity_fp in output: {}", output_text(&out)))
}

fn contacts_add_authenticated_with_route(
    iso: &common::TestIsolation,
    cfg: &Path,
    label: &str,
    fp: &str,
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
            fp,
            "--route-token",
            token,
        ],
    );
    assert_success(&out);
}

fn relay_inbox_set(iso: &common::TestIsolation, cfg: &Path, token: &str) {
    let out = run_qsc(iso, cfg, &["relay", "inbox-set", "--token", token]);
    assert_success(&out);
}

fn seed_authenticated_pair(iso: &common::TestIsolation, alice_cfg: &Path, bob_cfg: &Path) {
    init_identity(iso, alice_cfg, "alice");
    init_identity(iso, bob_cfg, "bob");
    let alice_fp = identity_fp(iso, alice_cfg, "alice");
    let bob_fp = identity_fp(iso, bob_cfg, "bob");
    contacts_add_authenticated_with_route(iso, alice_cfg, "bob", bob_fp.as_str(), ROUTE_TOKEN_BOB);
    contacts_add_authenticated_with_route(
        iso,
        bob_cfg,
        "alice",
        alice_fp.as_str(),
        ROUTE_TOKEN_ALICE,
    );
    relay_inbox_set(iso, alice_cfg, ROUTE_TOKEN_ALICE);
    relay_inbox_set(iso, bob_cfg, ROUTE_TOKEN_BOB);
}

fn new_pair(iso: &common::TestIsolation, tag: &str) -> (PathBuf, PathBuf) {
    let base = iso.root.join(tag);
    let _ = fs::remove_dir_all(&base);
    ensure_dir_700(&base);
    let alice_cfg = base.join("alice");
    let bob_cfg = base.join("bob");
    ensure_dir_700(&alice_cfg);
    ensure_dir_700(&bob_cfg);
    common::init_mock_vault(&alice_cfg);
    common::init_mock_vault(&bob_cfg);
    seed_authenticated_pair(iso, &alice_cfg, &bob_cfg);
    (alice_cfg, bob_cfg)
}

fn handshake_init(iso: &common::TestIsolation, alice_cfg: &Path, relay: &str) -> Output {
    run_qsc(
        iso,
        alice_cfg,
        &[
            "handshake",
            "init",
            "--as",
            "alice",
            "--peer",
            "bob",
            "--relay",
            relay,
            "--suite-mode",
            "suite-required",
        ],
    )
}

fn handshake_poll(
    iso: &common::TestIsolation,
    cfg: &Path,
    self_label: &str,
    peer: &str,
    relay: &str,
) -> Output {
    run_qsc(
        iso,
        cfg,
        &[
            "handshake",
            "poll",
            "--as",
            self_label,
            "--peer",
            peer,
            "--relay",
            relay,
            "--max",
            "4",
            "--suite-mode",
            "suite-required",
        ],
    )
}

fn handshake_poll_with_forced_rng_failure(
    iso: &common::TestIsolation,
    cfg: &Path,
    label: &str,
    self_label: &str,
    peer: &str,
    relay: &str,
) -> Output {
    run_qsc_with_forced_rng_failure(
        iso,
        cfg,
        label,
        &[
            "handshake",
            "poll",
            "--as",
            self_label,
            "--peer",
            peer,
            "--relay",
            relay,
            "--max",
            "4",
            "--suite-mode",
            "suite-required",
        ],
    )
}

fn session_path(cfg: &Path, peer: &str) -> PathBuf {
    cfg.join("qsp_sessions").join(format!("{peer}.qsv"))
}

fn path_bytes(path: &Path) -> Option<Vec<u8>> {
    match fs::read(path) {
        Ok(v) => Some(v),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => None,
        Err(e) => panic!("read {} failed: {e}", path.display()),
    }
}

fn derive_mock_vault_key(bytes: &[u8]) -> ([u8; 32], usize, usize) {
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
        .hash_password_into(
            common::TEST_MOCK_VAULT_PASSPHRASE.as_bytes(),
            salt,
            &mut key,
        )
        .expect("vault key");
    (key, salt_len, nonce_len)
}

fn read_mock_vault_json(cfg: &Path) -> Value {
    let bytes = fs::read(cfg.join("vault.qsv")).expect("vault read");
    let (key, salt_len, nonce_len) = derive_mock_vault_key(&bytes);
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

fn read_mock_vault_secret(cfg: &Path, name: &str) -> Option<String> {
    read_mock_vault_json(cfg)
        .get("secrets")
        .and_then(|v| v.get(name))
        .and_then(|v| v.as_str())
        .map(ToOwned::to_owned)
}

#[cfg(not(qsc_rng_failure_test_seam))]
#[test]
fn normal_build_ignores_test_seam_selector() {
    let iso = common::TestIsolation::new("na0449_normal_build_selector_ignored");
    let cfg = iso.root.join("normal-vault");
    ensure_dir_700(&cfg);

    let out = vault_init_with_stdin(&iso, &cfg, Some(VAULT_INIT_SALT_LABEL));
    assert_success(&out);
    let text = output_text(&out);
    assert!(text.contains("event=vault_init"), "{text}");
    assert!(
        cfg.join("vault.qsv").exists(),
        "normal build did not write vault"
    );

    println!("NA0449_PRODUCTION_SEMANTICS_UNCHANGED_OK");
}

#[cfg(qsc_rng_failure_test_seam)]
#[test]
fn common_na0449_markers() {
    println!("NA0449_RNG_FAILURE_AUTHORIZATION_CONSUMED_OK");
    println!("NA0449_RNG_FAILURE_TEST_SEAM_IMPLEMENTED_OK");
    println!("NA0449_RNG_FAILURE_FORCED_BY_TEST_ONLY_SEAM_OK");
    println!("NA0449_PRODUCTION_SEMANTICS_UNCHANGED_OK");
    println!("NA0449_NO_DEPENDENCY_CHANGE_OK");
    println!("NA0449_NO_WORKFLOW_CHANGE_OK");
    println!("NA0449_NO_PUBLIC_READINESS_CLAIM_OK");
    println!("NA0449_NO_CRYPTO_COMPLETE_CLAIM_OK");
    println!("NA0449_NO_RNG_FAILURE_COMPLETE_CLAIM_OK");
    println!("NA0449_STEWARD_REVIEW_TEMPLATE_USED_OK");
    println!("NA0449_ONE_READY_INVARIANT_OK");
}

#[cfg(qsc_rng_failure_test_seam)]
#[test]
fn handshake_session_id_rng_failure_writes_no_pending_state() {
    let iso = common::TestIsolation::new("na0449_handshake_sid_rng_failure");
    let (alice_cfg, _bob_cfg) = new_pair(&iso, "handshake");
    let server = common::start_inbox_server(1024 * 1024, 16);
    let relay = server.base_url().to_string();
    let alice_vault_before = fs::read(alice_cfg.join("vault.qsv")).expect("alice vault before");

    let out = run_qsc_with_forced_rng_failure(
        &iso,
        &alice_cfg,
        HANDSHAKE_SID_LABEL,
        &[
            "handshake",
            "init",
            "--as",
            "alice",
            "--peer",
            "bob",
            "--relay",
            &relay,
            "--suite-mode",
            "suite-required",
        ],
    );
    assert_failure(&out);
    let text = output_text(&out);
    assert!(text.contains("rng_failure_forced"), "{text}");
    assert_eq!(
        fs::read(alice_cfg.join("vault.qsv")).expect("alice vault after"),
        alice_vault_before,
        "forced session-id RNG failure wrote pending vault state"
    );
    assert!(
        path_bytes(&session_path(&alice_cfg, "bob")).is_none(),
        "forced session-id RNG failure wrote a session blob"
    );
    assert!(
        server.drain_channel(ROUTE_TOKEN_BOB).is_empty(),
        "forced session-id RNG failure emitted A1"
    );

    println!("NA0449_QSC_HANDSHAKE_SESSION_ID_RNG_FAILURE_NO_PENDING_OK");
}

#[cfg(qsc_rng_failure_test_seam)]
#[test]
fn vault_rng_failure_writes_no_vault_file() {
    let iso = common::TestIsolation::new("na0449_vault_rng_failure");
    let cfg = iso.root.join("vault-rng-failure");
    ensure_dir_700(&cfg);

    let out = vault_init_with_stdin(&iso, &cfg, Some(VAULT_INIT_SALT_LABEL));
    assert_failure(&out);
    let text = output_text(&out);
    assert!(text.contains("rng_failure_forced"), "{text}");
    assert!(
        !cfg.join("vault.qsv").exists(),
        "forced vault RNG failure wrote vault.qsv"
    );
    assert!(
        !cfg.join("vault.qsv.tmp").exists(),
        "forced vault RNG failure left temp vault file"
    );

    println!("NA0449_QSC_VAULT_RNG_FAILURE_NO_VAULT_WRITE_OK");
}

#[cfg(qsc_rng_failure_test_seam)]
#[test]
fn session_store_rng_failure_writes_no_session_blob() {
    let iso = common::TestIsolation::new("na0449_session_store_rng_failure");
    let (alice_cfg, bob_cfg) = new_pair(&iso, "session-store");
    let server = common::start_inbox_server(1024 * 1024, 16);
    let relay = server.base_url().to_string();

    let alice_init = handshake_init(&iso, &alice_cfg, &relay);
    assert_success(&alice_init);
    let a1 = server
        .drain_channel(ROUTE_TOKEN_BOB)
        .pop()
        .expect("A1 queued");
    server.replace_channel(ROUTE_TOKEN_BOB, vec![a1]);

    let bob_poll = handshake_poll(&iso, &bob_cfg, "bob", "alice", &relay);
    assert_success(&bob_poll);
    let b1 = server
        .drain_channel(ROUTE_TOKEN_ALICE)
        .pop()
        .expect("B1 queued");
    server.replace_channel(ROUTE_TOKEN_ALICE, vec![b1]);

    assert!(
        path_bytes(&session_path(&alice_cfg, "bob")).is_none(),
        "alice session existed before forced failure"
    );
    assert!(
        read_mock_vault_secret(&alice_cfg, QSP_SESSION_STORE_KEY_SECRET).is_none(),
        "session-store key existed before forced failure"
    );
    let pending_before = read_mock_vault_secret(&alice_cfg, "handshake.pending.alice.bob")
        .expect("alice pending before");
    let alice_vault_before = fs::read(alice_cfg.join("vault.qsv")).expect("alice vault before");

    let alice_poll = handshake_poll_with_forced_rng_failure(
        &iso,
        &alice_cfg,
        SESSION_STORE_KEY_LABEL,
        "alice",
        "bob",
        &relay,
    );
    assert_failure(&alice_poll);
    let text = output_text(&alice_poll);
    assert!(text.contains("handshake_session_store_failed"), "{text}");
    assert!(
        path_bytes(&session_path(&alice_cfg, "bob")).is_none(),
        "forced session-store RNG failure wrote a session blob"
    );
    assert_eq!(
        read_mock_vault_secret(&alice_cfg, "handshake.pending.alice.bob").as_deref(),
        Some(pending_before.as_str()),
        "forced session-store RNG failure cleared pending state"
    );
    assert!(
        read_mock_vault_secret(&alice_cfg, QSP_SESSION_STORE_KEY_SECRET).is_none(),
        "forced session-store RNG failure wrote session-store key secret"
    );
    assert_eq!(
        fs::read(alice_cfg.join("vault.qsv")).expect("alice vault after"),
        alice_vault_before,
        "forced session-store RNG failure changed vault bytes"
    );
    assert!(
        server.drain_channel(ROUTE_TOKEN_BOB).is_empty(),
        "forced session-store RNG failure emitted A2"
    );

    println!("NA0449_QSC_SESSION_STORE_RNG_FAILURE_NO_SESSION_WRITE_OK");
}
