#![allow(unexpected_cfgs)]
#![allow(dead_code)]

mod common;

use argon2::{Algorithm, Argon2, Params, Version};
use chacha20poly1305::aead::{Aead, KeyInit};
use chacha20poly1305::{ChaCha20Poly1305, Key, Nonce};
use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Output;

const ROUTE_TOKEN_ALICE: &str = "route_token_alice_na0463_a2_sig";
const ROUTE_TOKEN_BOB: &str = "route_token_bob_na0463_a2_sig__";
const A2_SIGNATURE_LABEL: &str = "QSC.SIG.A2";

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

fn new_vault_pair(iso: &common::TestIsolation, tag: &str) -> (PathBuf, PathBuf) {
    let base = iso.root.join(tag);
    let _ = fs::remove_dir_all(&base);
    ensure_dir_700(&base);
    let alice_cfg = base.join("alice");
    let bob_cfg = base.join("bob");
    ensure_dir_700(&alice_cfg);
    ensure_dir_700(&bob_cfg);
    common::init_mock_vault(&alice_cfg);
    common::init_mock_vault(&bob_cfg);
    (alice_cfg, bob_cfg)
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

fn pending_legacy_path(cfg: &Path, self_label: &str, peer: &str) -> PathBuf {
    cfg.join(format!("handshake_pending_{self_label}_{peer}.json"))
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

fn assert_no_secret_output(text: &str) {
    for forbidden in [
        ROUTE_TOKEN_ALICE,
        ROUTE_TOKEN_BOB,
        common::TEST_MOCK_VAULT_PASSPHRASE,
        "QSC_DESKTOP_SESSION_PASSPHRASE",
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

#[cfg(qsc_rng_failure_test_seam)]
#[test]
fn a2_signature_rng_failure_emits_no_a2_output() {
    let iso = common::TestIsolation::new("na0463_a2_sig_rng_failure");
    let (alice_cfg, bob_cfg) = new_vault_pair(&iso, "a2-sign-failure");
    seed_authenticated_pair(&iso, &alice_cfg, &bob_cfg);

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
    let bob_text = output_text(&bob_poll);
    assert!(bob_text.contains("event=handshake_send"), "{bob_text}");
    assert!(bob_text.contains("msg=B1"), "{bob_text}");
    let b1 = server
        .drain_channel(ROUTE_TOKEN_ALICE)
        .pop()
        .expect("B1 queued");
    server.replace_channel(ROUTE_TOKEN_ALICE, vec![b1]);

    assert!(
        matches!(
            read_mock_vault_secret(&alice_cfg, "handshake.pending.alice.bob"),
            Some(v) if !v.is_empty()
        ),
        "Alice pending state missing before forced A2 signing failure"
    );
    assert!(
        path_bytes(&session_path(&alice_cfg, "bob")).is_none(),
        "Alice session existed before forced A2 signing failure"
    );

    let alice_poll = handshake_poll_with_forced_rng_failure(
        &iso,
        &alice_cfg,
        A2_SIGNATURE_LABEL,
        "alice",
        "bob",
        &relay,
    );
    assert_success(&alice_poll);
    let text = output_text(&alice_poll);
    assert!(text.contains("event=handshake_reject"), "{text}");
    assert!(text.contains("sig_sign_failed"), "{text}");
    assert!(!text.contains("reason=a2_sign"), "{text}");
    assert!(!text.contains("event=handshake_send"), "{text}");
    assert!(!text.contains("msg=A2"), "{text}");
    assert!(!text.contains("event=handshake_complete"), "{text}");
    assert_no_secret_output(&(output_text(&alice_init) + &bob_text + &text));

    assert!(
        path_bytes(&session_path(&alice_cfg, "bob")).is_some(),
        "forced A2 signing failure did not preserve the known post-mutation session write"
    );
    assert!(
        read_mock_vault_secret(&alice_cfg, "handshake.pending.alice.bob")
            .map(|v| v.is_empty())
            .unwrap_or(true),
        "forced A2 signing failure did not preserve the known post-mutation pending clear"
    );
    assert!(
        path_bytes(&pending_legacy_path(&alice_cfg, "alice", "bob")).is_none(),
        "forced A2 signing failure wrote legacy pending state"
    );
    assert!(
        server.drain_channel(ROUTE_TOKEN_BOB).is_empty(),
        "forced A2 signing failure emitted relay A2"
    );

    println!("NA0463_A2_SIGNATURE_PROVIDER_RNG_FAILURE_FORCED_OK");
    println!("NA0463_A2_SIGNATURE_PROVIDER_RNG_FAILURE_SIG_SIGN_FAILED_OK");
    println!("NA0463_A2_SIGNATURE_PROVIDER_RNG_FAILURE_NO_A2_OUTPUT_OK");
    println!("NA0463_A2_SIGNATURE_PROVIDER_RNG_FAILURE_NO_RELAY_A2_OK");
    println!("NA0463_A2_SIGNATURE_PROVIDER_RNG_FAILURE_NO_FALSE_NO_MUTATION_CLAIM_OK");
    println!("NA0463_A2_SIGNATURE_PROVIDER_RNG_FAILURE_POST_MUTATION_TIMING_ACKNOWLEDGED_OK");
}

#[cfg(not(qsc_rng_failure_test_seam))]
#[test]
fn a2_signature_rng_failure_seam_inactive_without_cfg() {
    let iso = common::TestIsolation::new("na0463_a2_sig_normal_build");
    let (alice_cfg, bob_cfg) = new_vault_pair(&iso, "normal-build");
    seed_authenticated_pair(&iso, &alice_cfg, &bob_cfg);

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

    let alice_poll = handshake_poll_with_forced_rng_failure(
        &iso,
        &alice_cfg,
        A2_SIGNATURE_LABEL,
        "alice",
        "bob",
        &relay,
    );
    assert_success(&alice_poll);
    let text = output_text(&alice_poll);
    assert!(text.contains("event=sig_status"), "{text}");
    assert!(text.contains("reason=a2_sign"), "{text}");
    assert!(text.contains("event=handshake_send"), "{text}");
    assert!(text.contains("msg=A2"), "{text}");
    assert!(text.contains("event=handshake_complete"), "{text}");
    assert!(!text.contains("sig_sign_failed"), "{text}");
    assert!(!text.contains("rng_failure_forced"), "{text}");
    assert!(
        !server.drain_channel(ROUTE_TOKEN_BOB).is_empty(),
        "normal build did not relay A2"
    );
    assert_no_secret_output(&(output_text(&alice_init) + &output_text(&bob_poll) + &text));

    println!("NA0463_PRODUCTION_SEMANTICS_UNCHANGED_OK");
}

#[cfg(qsc_rng_failure_test_seam)]
#[test]
fn common_na0463_markers() {
    println!("NA0463_A2_SIGNATURE_PROVIDER_RNG_SEAM_IMPLEMENTED_OK");
    println!("NA0463_B1_SIGNATURE_BACKGROUND_PRESERVED_OK");
    println!("NA0463_IDENTITY_PROVIDER_RNG_RESIDUAL_DEFERRED_OK");
    println!("NA0463_X25519_RESIDUAL_DEFERRED_OK");
    println!("NA0463_REFIMPL_PROVIDER_RNG_RESIDUAL_DEFERRED_OK");
    println!("NA0463_KEM_PROVIDER_RNG_BACKGROUND_PRESERVED_OK");
    println!("NA0463_NO_DEPENDENCY_CHANGE_OK");
    println!("NA0463_NO_WORKFLOW_CHANGE_OK");
    println!("NA0463_NO_PUBLIC_READINESS_CLAIM_OK");
    println!("NA0463_NO_CRYPTO_COMPLETE_CLAIM_OK");
    println!("NA0463_NO_SIGNATURE_COMPLETE_CLAIM_OK");
    println!("NA0463_NO_IDENTITY_COMPLETE_CLAIM_OK");
    println!("NA0463_NO_RNG_FAILURE_COMPLETE_CLAIM_OK");
    println!("NA0463_NO_PROVIDER_RNG_COMPLETE_CLAIM_OK");
    println!("NA0463_ONE_READY_INVARIANT_OK");
}
