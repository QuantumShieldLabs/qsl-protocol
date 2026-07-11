mod common;

use argon2::{Algorithm, Argon2, Params, Version};
use chacha20poly1305::aead::{Aead, KeyInit};
use chacha20poly1305::{ChaCha20Poly1305, Key, Nonce};
use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

const ROUTE_TOKEN_ALICE: &str = "route_token_alice_na0446_sentinel_abcd";
const ROUTE_TOKEN_BOB: &str = "route_token_bob_na0446_sentinel_abcdef";
const QSP_SESSION_STORE_KEY_SECRET: &str = "qsp_session_store_key_v1";
const VAULT_PASSPHRASE_SENTINEL: &str = "na0446-controlled-passphrase-redaction-sentinel";
const OUTPUT_SENTINEL: &str = "na0446-controlled-output-redaction-sentinel";

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

fn run_qsc(iso: &common::TestIsolation, cfg: &Path, args: &[&str]) -> Output {
    let mut cmd = common::qsc_std_command();
    iso.apply_to(&mut cmd);
    cmd.env("QSC_CONFIG_DIR", cfg)
        .env("QSC_MARK_FORMAT", "plain")
        .args(args)
        .output()
        .expect("qsc command")
}

fn run_plain_qsc(iso: &common::TestIsolation, cfg: &Path, args: &[&str]) -> Output {
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("qsc"));
    iso.apply_to(&mut cmd);
    cmd.env("QSC_CONFIG_DIR", cfg)
        .env("QSC_MARK_FORMAT", "plain")
        .args(args)
        .output()
        .expect("qsc command")
}

fn assert_success(out: &Output) {
    assert!(out.status.success(), "{}", output_text(out));
}

fn assert_failure(out: &Output) {
    assert!(!out.status.success(), "{}", output_text(out));
}

fn assert_output_redacted(text: &str, extra_forbidden: &[&str]) {
    for forbidden in [
        ROUTE_TOKEN_ALICE,
        ROUTE_TOKEN_BOB,
        common::TEST_MOCK_VAULT_PASSPHRASE,
        VAULT_PASSPHRASE_SENTINEL,
        OUTPUT_SENTINEL,
        "QSC_DESKTOP_SESSION_PASSPHRASE",
        "panicked",
        "stack backtrace",
        "thread '",
    ]
    .into_iter()
    .chain(extra_forbidden.iter().copied())
    {
        assert!(
            !text.contains(forbidden),
            "forbidden output fragment leaked: {forbidden}: {text}"
        );
    }
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

fn identity_kem_pk(iso: &common::TestIsolation, cfg: &Path, label: &str) -> String {
    let out = run_qsc(iso, cfg, &["identity", "show", "--as", label]);
    assert_success(&out);
    output_text(&out)
        .lines()
        .find_map(|line| line.strip_prefix("identity_kem_pk="))
        .map(ToOwned::to_owned)
        .unwrap_or_else(|| panic!("missing identity_kem_pk in output: {}", output_text(&out)))
}

fn contacts_add_authenticated_with_route(
    iso: &common::TestIsolation,
    cfg: &Path,
    label: &str,
    fp: &str, kem_pk: &str,
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
            "--kem-pk",
            kem_pk,
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
    let alice_kem = identity_kem_pk(iso, alice_cfg, "alice");
    let bob_fp = identity_fp(iso, bob_cfg, "bob");
    let bob_kem = identity_kem_pk(iso, bob_cfg, "bob");
    contacts_add_authenticated_with_route(iso, alice_cfg, "bob", bob_fp.as_str(), bob_kem.as_str(), ROUTE_TOKEN_BOB);
    contacts_add_authenticated_with_route(
        iso,
        bob_cfg,
        "alice",
        alice_fp.as_str(),
        alice_kem.as_str(),
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

fn pending_key(self_label: &str, peer: &str) -> String {
    format!("handshake.pending.{self_label}.{peer}")
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

fn vault_secret_present(cfg: &Path, name: &str) -> bool {
    read_mock_vault_secret(cfg, name)
        .map(|value| !value.is_empty())
        .unwrap_or(false)
}

fn assert_pending_nonempty(cfg: &Path, self_label: &str, peer: &str) {
    assert!(
        vault_secret_present(cfg, pending_key(self_label, peer).as_str()),
        "pending secret not present for {self_label}->{peer}"
    );
}

fn assert_pending_cleared(cfg: &Path, self_label: &str, peer: &str) {
    let key = pending_key(self_label, peer);
    assert_eq!(
        read_mock_vault_secret(cfg, key.as_str()).as_deref(),
        Some(""),
        "pending secret was not cleared for {self_label}->{peer}"
    );
}

fn assert_session_absent(cfg: &Path, peer: &str) {
    assert!(
        path_bytes(&session_path(cfg, peer)).is_none(),
        "session unexpectedly present for {peer}"
    );
}

fn assert_session_present(cfg: &Path, peer: &str) -> Vec<u8> {
    let path = session_path(cfg, peer);
    let bytes =
        fs::read(&path).unwrap_or_else(|e| panic!("missing session {}: {e}", path.display()));
    assert!(
        bytes.starts_with(b"QSSV01"),
        "session blob missing encrypted envelope magic"
    );
    bytes
}

fn assert_bytes_do_not_contain(bytes: &[u8], needle: &[u8], context: &str) {
    assert!(
        !bytes.windows(needle.len()).any(|w| w == needle),
        "{context} contained controlled plaintext marker {}",
        String::from_utf8_lossy(needle)
    );
}

fn assert_session_blob_encrypted_boundary(bytes: &[u8], context: &str) {
    for needle in [
        b"session_id".as_slice(),
        b"ck_ec".as_slice(),
        b"ck_pq".as_slice(),
        b"hk_s".as_slice(),
        b"hk_r".as_slice(),
        b"\"rk\"".as_slice(),
        b"\"send\"".as_slice(),
        b"\"recv\"".as_slice(),
        b"pq_init_ss".as_slice(),
    ] {
        assert_bytes_do_not_contain(bytes, needle, context);
    }
}

fn assert_vault_encrypted_boundary(cfg: &Path, context: &str) -> Vec<u8> {
    let bytes = fs::read(cfg.join("vault.qsv")).expect("vault read");
    assert!(bytes.starts_with(b"QSCV01"), "vault envelope magic missing");
    for needle in [
        ROUTE_TOKEN_ALICE.as_bytes(),
        ROUTE_TOKEN_BOB.as_bytes(),
        common::TEST_MOCK_VAULT_PASSPHRASE.as_bytes(),
        VAULT_PASSPHRASE_SENTINEL.as_bytes(),
        OUTPUT_SENTINEL.as_bytes(),
        b"handshake.pending.",
        QSP_SESSION_STORE_KEY_SECRET.as_bytes(),
    ] {
        assert_bytes_do_not_contain(&bytes, needle, context);
    }
    bytes
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

#[test]
fn pending_handshake_secret_cleanup_success_and_reject_boundaries() {
    let iso = common::TestIsolation::new("na0446_pending_cleanup_success");
    let (alice_cfg, bob_cfg) = new_pair(&iso, "success");
    let server = common::start_inbox_server(1024 * 1024, 16);
    let relay = server.base_url().to_string();

    let alice_init = handshake_init(&iso, &alice_cfg, &relay);
    assert_success(&alice_init);
    assert_pending_nonempty(&alice_cfg, "alice", "bob");
    assert_session_absent(&alice_cfg, "bob");
    assert!(!vault_secret_present(
        &alice_cfg,
        QSP_SESSION_STORE_KEY_SECRET
    ));

    let bob_poll_b1 = handshake_poll(&iso, &bob_cfg, "bob", "alice", &relay);
    assert_success(&bob_poll_b1);
    assert_pending_nonempty(&bob_cfg, "bob", "alice");
    assert_session_absent(&bob_cfg, "alice");

    let alice_poll_a2 = handshake_poll(&iso, &alice_cfg, "alice", "bob", &relay);
    assert_success(&alice_poll_a2);
    assert!(output_text(&alice_poll_a2).contains("event=handshake_complete"));
    assert_pending_cleared(&alice_cfg, "alice", "bob");
    assert_session_present(&alice_cfg, "bob");
    assert!(vault_secret_present(
        &alice_cfg,
        QSP_SESSION_STORE_KEY_SECRET
    ));

    let bob_poll_complete = handshake_poll(&iso, &bob_cfg, "bob", "alice", &relay);
    assert_success(&bob_poll_complete);
    assert!(output_text(&bob_poll_complete).contains("event=handshake_complete"));
    assert_pending_cleared(&bob_cfg, "bob", "alice");
    assert_session_present(&bob_cfg, "alice");
    assert!(vault_secret_present(&bob_cfg, QSP_SESSION_STORE_KEY_SECRET));

    let (reject_alice_cfg, reject_bob_cfg) = new_pair(&iso, "reject");
    let reject_server = common::start_inbox_server(1024 * 1024, 16);
    let reject_relay = reject_server.base_url().to_string();
    let reject_init = handshake_init(&iso, &reject_alice_cfg, &reject_relay);
    assert_success(&reject_init);
    assert_pending_nonempty(&reject_alice_cfg, "alice", "bob");
    let pending_vault_before = fs::read(reject_alice_cfg.join("vault.qsv")).expect("vault before");
    reject_server.enqueue_raw(
        ROUTE_TOKEN_ALICE,
        b"not-a-valid-qsc-handshake-frame".to_vec(),
    );
    let reject_poll = handshake_poll(&iso, &reject_alice_cfg, "alice", "bob", &reject_relay);
    assert_success(&reject_poll);
    let reject_text = output_text(&reject_poll);
    assert!(
        reject_text.contains("event=handshake_reject"),
        "{reject_text}"
    );
    assert!(
        !reject_text.contains("event=handshake_complete"),
        "{reject_text}"
    );
    assert_eq!(
        fs::read(reject_alice_cfg.join("vault.qsv")).expect("vault after"),
        pending_vault_before,
        "malformed-frame reject changed pending vault state"
    );
    assert_session_absent(&reject_alice_cfg, "bob");
    assert_session_absent(&reject_bob_cfg, "alice");

    println!("NA0446_KEY_LIFECYCLE_TEST_IMPLEMENTATION_OK");
    println!("NA0446_PENDING_SECRET_CLEANUP_SUCCESS_BOUNDARY_OK");
    println!("NA0446_REJECT_NO_MUTATION_BOUNDARY_OK");
    println!("NA0446_NO_RUNTIME_HOOK_USED_OK");
    println!("NA0446_DIRECT_MEMORY_ZEROIZATION_NOT_CLAIMED_OK");
}

#[test]
fn session_secret_store_inserted_only_after_success_and_encrypted_at_rest() {
    let iso = common::TestIsolation::new("na0446_session_secret_store");
    let (alice_cfg, bob_cfg) = new_pair(&iso, "session-store");
    let server = common::start_inbox_server(1024 * 1024, 16);
    let relay = server.base_url().to_string();

    assert!(!vault_secret_present(
        &alice_cfg,
        QSP_SESSION_STORE_KEY_SECRET
    ));
    assert!(!vault_secret_present(
        &bob_cfg,
        QSP_SESSION_STORE_KEY_SECRET
    ));
    assert_success(&handshake_init(&iso, &alice_cfg, &relay));
    assert!(!vault_secret_present(
        &alice_cfg,
        QSP_SESSION_STORE_KEY_SECRET
    ));
    assert_success(&handshake_poll(&iso, &bob_cfg, "bob", "alice", &relay));
    assert!(!vault_secret_present(
        &bob_cfg,
        QSP_SESSION_STORE_KEY_SECRET
    ));

    let alice_complete = handshake_poll(&iso, &alice_cfg, "alice", "bob", &relay);
    assert_success(&alice_complete);
    assert!(vault_secret_present(
        &alice_cfg,
        QSP_SESSION_STORE_KEY_SECRET
    ));
    let alice_blob = assert_session_present(&alice_cfg, "bob");
    assert_session_blob_encrypted_boundary(&alice_blob, "alice session");

    assert!(!vault_secret_present(
        &bob_cfg,
        QSP_SESSION_STORE_KEY_SECRET
    ));
    let bob_complete = handshake_poll(&iso, &bob_cfg, "bob", "alice", &relay);
    assert_success(&bob_complete);
    assert!(vault_secret_present(&bob_cfg, QSP_SESSION_STORE_KEY_SECRET));
    let bob_blob = assert_session_present(&bob_cfg, "alice");
    assert_session_blob_encrypted_boundary(&bob_blob, "bob session");

    println!("NA0446_SESSION_SECRET_STORE_BOUNDARY_OK");
    println!("NA0446_ENCRYPTED_AT_REST_BOUNDARY_OK");
}

#[test]
fn key_lifecycle_output_redaction_sentinel_scan() {
    let iso = common::TestIsolation::new("na0446_redaction_sentinel");
    let cfg = iso.root.join("redaction-cfg");
    ensure_dir_700(&cfg);
    let passphrase_file = common::write_passphrase_file(&cfg, "redaction", OUTPUT_SENTINEL);
    let out = run_plain_qsc(
        &iso,
        &cfg,
        &[
            "vault",
            "init",
            "--key-source",
            "bogus",
            "--passphrase-file",
            passphrase_file.to_str().expect("passphrase path"),
        ],
    );
    assert_failure(&out);
    let text = output_text(&out);
    assert!(text.contains("code=key_source_invalid"), "{text}");
    assert_output_redacted(&text, &[OUTPUT_SENTINEL]);
    assert!(!cfg.join("vault.qsv").exists());

    println!("NA0446_REDACTION_SENTINEL_BOUNDARY_OK");
    println!("NA0446_NO_SECRET_MATERIAL_COMPLETE_CLAIM_OK");
    println!("NA0446_QSHIELD_CLI_DEMO_BOUNDARY_PRESERVED_OK");
    println!("NA0446_REFIMPL_SCOPE_DEFERRED_OK");
}

#[test]
fn reject_paths_preserve_pending_session_vault_state() {
    let iso = common::TestIsolation::new("na0446_reject_preserve");
    let (alice_cfg, bob_cfg) = new_pair(&iso, "reject-preserve");
    let server = common::start_inbox_server(1024 * 1024, 16);
    let relay = server.base_url().to_string();

    let init = handshake_init(&iso, &alice_cfg, &relay);
    assert_success(&init);
    assert_pending_nonempty(&alice_cfg, "alice", "bob");
    let alice_vault_before = fs::read(alice_cfg.join("vault.qsv")).expect("alice vault before");
    let bob_vault_before = fs::read(bob_cfg.join("vault.qsv")).expect("bob vault before");
    let alice_session_before = path_bytes(&session_path(&alice_cfg, "bob"));
    let bob_session_before = path_bytes(&session_path(&bob_cfg, "alice"));

    server.enqueue_raw(
        ROUTE_TOKEN_ALICE,
        b"na0446-malformed-reject-boundary".to_vec(),
    );
    let reject = handshake_poll(&iso, &alice_cfg, "alice", "bob", &relay);
    assert_success(&reject);
    let text = output_text(&reject);
    assert!(text.contains("event=handshake_reject"), "{text}");
    assert_output_redacted(&text, &[]);

    assert_eq!(
        fs::read(alice_cfg.join("vault.qsv")).expect("alice vault after"),
        alice_vault_before,
        "alice pending vault changed on malformed reject"
    );
    assert_eq!(
        fs::read(bob_cfg.join("vault.qsv")).expect("bob vault after"),
        bob_vault_before,
        "bob vault changed during alice malformed reject"
    );
    assert_eq!(
        path_bytes(&session_path(&alice_cfg, "bob")),
        alice_session_before,
        "alice session changed on malformed reject"
    );
    assert_eq!(
        path_bytes(&session_path(&bob_cfg, "alice")),
        bob_session_before,
        "bob session changed during alice malformed reject"
    );
    assert!(!vault_secret_present(
        &alice_cfg,
        QSP_SESSION_STORE_KEY_SECRET
    ));
    assert!(!vault_secret_present(
        &bob_cfg,
        QSP_SESSION_STORE_KEY_SECRET
    ));

    println!("NA0446_REJECT_NO_MUTATION_BOUNDARY_OK");
}

#[test]
fn session_and_vault_encrypted_at_rest_boundaries() {
    let iso = common::TestIsolation::new("na0446_encrypted_at_rest");
    let (alice_cfg, bob_cfg) = new_pair(&iso, "encrypted-at-rest");
    let server = common::start_inbox_server(1024 * 1024, 16);
    let relay = server.base_url().to_string();

    assert_success(&handshake_init(&iso, &alice_cfg, &relay));
    assert_success(&handshake_poll(&iso, &bob_cfg, "bob", "alice", &relay));
    assert_success(&handshake_poll(&iso, &alice_cfg, "alice", "bob", &relay));
    assert_success(&handshake_poll(&iso, &bob_cfg, "bob", "alice", &relay));

    let alice_vault = assert_vault_encrypted_boundary(&alice_cfg, "alice vault");
    let bob_vault = assert_vault_encrypted_boundary(&bob_cfg, "bob vault");
    let alice_session = assert_session_present(&alice_cfg, "bob");
    let bob_session = assert_session_present(&bob_cfg, "alice");
    assert_session_blob_encrypted_boundary(&alice_session, "alice session");
    assert_session_blob_encrypted_boundary(&bob_session, "bob session");
    assert_ne!(
        alice_vault, bob_vault,
        "peer vault envelopes unexpectedly matched"
    );
    assert_ne!(
        alice_session, bob_session,
        "peer session envelopes unexpectedly matched"
    );

    println!("NA0446_ENCRYPTED_AT_REST_BOUNDARY_OK");
}

#[test]
fn vault_passphrase_redaction_and_no_plaintext_boundary() {
    let iso = common::TestIsolation::new("na0446_vault_passphrase_boundary");
    let cfg = iso.root.join("vault-passphrase-cfg");
    ensure_dir_700(&cfg);

    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("qsc"));
    iso.apply_to(&mut cmd);
    let out = cmd
        .env("QSC_CONFIG_DIR", &cfg)
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
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .and_then(|mut child| {
            use std::io::Write;
            child
                .stdin
                .as_mut()
                .expect("stdin")
                .write_all(VAULT_PASSPHRASE_SENTINEL.as_bytes())?;
            child.wait_with_output()
        })
        .expect("vault init with stdin");
    assert_success(&out);
    let text = output_text(&out);
    assert!(text.contains("event=vault_init"), "{text}");
    assert_output_redacted(&text, &[VAULT_PASSPHRASE_SENTINEL]);
    let vault_bytes = fs::read(cfg.join("vault.qsv")).expect("vault read");
    assert!(vault_bytes.starts_with(b"QSCV01"));
    assert_bytes_do_not_contain(
        &vault_bytes,
        VAULT_PASSPHRASE_SENTINEL.as_bytes(),
        "vault file",
    );

    println!("NA0446_REDACTION_SENTINEL_BOUNDARY_OK");
    println!("NA0446_ENCRYPTED_AT_REST_BOUNDARY_OK");
}
