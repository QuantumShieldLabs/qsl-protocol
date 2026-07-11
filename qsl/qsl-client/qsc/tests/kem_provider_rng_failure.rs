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

const ROUTE_TOKEN_ALICE: &str = "route_token_alice_na0458_kem_rng";
const ROUTE_TOKEN_BOB: &str = "route_token_bob_na0458_kem_rng__";
const KEM_KEYPAIR_LABEL: &str = "QSC.KEM.KEYPAIR";
const KEM_ENCAP_LABEL: &str = "QSC.KEM.ENCAP";

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

fn identity_sig_pk(iso: &common::TestIsolation, cfg: &Path, label: &str) -> String {
    let out = run_qsc(iso, cfg, &["identity", "show", "--as", label]);
    assert_success(&out);
    output_text(&out)
        .lines()
        .find_map(|line| line.strip_prefix("identity_sig_pk="))
        .map(ToOwned::to_owned)
        .unwrap_or_else(|| panic!("missing identity_sig_pk in output: {}", output_text(&out)))
}

fn contacts_add_authenticated_with_route(
    iso: &common::TestIsolation,
    cfg: &Path,
    label: &str,
    fp: &str, kem_pk: &str, sig_pk: &str,
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
            "--sig-pk",
            sig_pk,
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
    let alice_sig = identity_sig_pk(iso, alice_cfg, "alice");
    let bob_fp = identity_fp(iso, bob_cfg, "bob");
    let bob_kem = identity_kem_pk(iso, bob_cfg, "bob");
    let bob_sig = identity_sig_pk(iso, bob_cfg, "bob");
    contacts_add_authenticated_with_route(iso, alice_cfg, "bob", bob_fp.as_str(), bob_kem.as_str(), bob_sig.as_str(), ROUTE_TOKEN_BOB);
    contacts_add_authenticated_with_route(
        iso,
        bob_cfg,
        "alice",
        alice_fp.as_str(),
        alice_kem.as_str(), alice_sig.as_str(),
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

fn handshake_init_with_forced_rng_failure(
    iso: &common::TestIsolation,
    alice_cfg: &Path,
    relay: &str,
    label: &str,
) -> Output {
    run_qsc_with_forced_rng_failure(
        iso,
        alice_cfg,
        label,
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

fn identity_self_path(cfg: &Path, label: &str) -> PathBuf {
    cfg.join("identities").join(format!("self_{label}.json"))
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

#[cfg(not(qsc_rng_failure_test_seam))]
#[test]
fn kem_provider_rng_failure_seam_inactive_without_cfg() {
    let iso = common::TestIsolation::new("na0458_kem_provider_rng_normal_build");
    let (alice_cfg, bob_cfg) = new_vault_pair(&iso, "normal-build");
    init_identity(&iso, &bob_cfg, "bob");
    let bob_fp = identity_fp(&iso, &bob_cfg, "bob");
    let bob_kem = identity_kem_pk(&iso, &bob_cfg, "bob");
    let bob_sig = identity_sig_pk(&iso, &bob_cfg, "bob");
    contacts_add_authenticated_with_route(
        &iso,
        &alice_cfg,
        "bob",
        bob_fp.as_str(),
        bob_kem.as_str(), bob_sig.as_str(),
        ROUTE_TOKEN_BOB,
    );

    let server = common::start_inbox_server(1024 * 1024, 16);
    let relay = server.base_url().to_string();
    let alice_init =
        handshake_init_with_forced_rng_failure(&iso, &alice_cfg, &relay, KEM_KEYPAIR_LABEL);
    assert_success(&alice_init);
    let init_text = output_text(&alice_init);
    assert!(init_text.contains("event=handshake_send"), "{init_text}");
    assert!(init_text.contains("msg=A1"), "{init_text}");
    assert!(!init_text.contains("rng_failure_forced"), "{init_text}");
    assert!(
        identity_self_path(&alice_cfg, "alice").exists(),
        "normal build did not create Alice identity"
    );
    let a1 = server
        .drain_channel(ROUTE_TOKEN_BOB)
        .pop()
        .expect("normal A1 queued");
    server.replace_channel(ROUTE_TOKEN_BOB, vec![a1]);

    let alice_fp = identity_fp(&iso, &alice_cfg, "alice");
    let alice_kem = identity_kem_pk(&iso, &alice_cfg, "alice");
    let alice_sig = identity_sig_pk(&iso, &alice_cfg, "alice");
    contacts_add_authenticated_with_route(
        &iso,
        &bob_cfg,
        "alice",
        alice_fp.as_str(),
        alice_kem.as_str(), alice_sig.as_str(),
        ROUTE_TOKEN_ALICE,
    );
    relay_inbox_set(&iso, &bob_cfg, ROUTE_TOKEN_BOB);

    let bob_poll = handshake_poll_with_forced_rng_failure(
        &iso,
        &bob_cfg,
        KEM_ENCAP_LABEL,
        "bob",
        "alice",
        &relay,
    );
    assert_success(&bob_poll);
    let poll_text = output_text(&bob_poll);
    assert!(poll_text.contains("event=handshake_send"), "{poll_text}");
    assert!(poll_text.contains("msg=B1"), "{poll_text}");
    assert!(!poll_text.contains("pq_encap_failed"), "{poll_text}");
    assert!(!poll_text.contains("rng_failure_forced"), "{poll_text}");
    assert!(
        !server.drain_channel(ROUTE_TOKEN_ALICE).is_empty(),
        "normal build did not emit B1"
    );
    assert_no_secret_output(&(init_text + &poll_text));

    println!("NA0458_PRODUCTION_SEMANTICS_UNCHANGED_OK");
}

#[cfg(qsc_rng_failure_test_seam)]
#[test]
fn kem_keypair_rng_failure_writes_no_identity_or_session_state() {
    let iso = common::TestIsolation::new("na0458_kem_keypair_rng_failure");
    let (alice_cfg, bob_cfg) = new_vault_pair(&iso, "keypair-failure");
    init_identity(&iso, &bob_cfg, "bob");
    let bob_fp = identity_fp(&iso, &bob_cfg, "bob");
    let bob_kem = identity_kem_pk(&iso, &bob_cfg, "bob");
    let bob_sig = identity_sig_pk(&iso, &bob_cfg, "bob");
    contacts_add_authenticated_with_route(
        &iso,
        &alice_cfg,
        "bob",
        bob_fp.as_str(),
        bob_kem.as_str(), bob_sig.as_str(),
        ROUTE_TOKEN_BOB,
    );

    let server = common::start_inbox_server(1024 * 1024, 16);
    let relay = server.base_url().to_string();
    let alice_vault_before = fs::read(alice_cfg.join("vault.qsv")).expect("alice vault before");
    assert!(
        !identity_self_path(&alice_cfg, "alice").exists(),
        "Alice identity existed before forced keypair failure"
    );
    assert!(
        read_mock_vault_secret(&alice_cfg, "identity.kem_sk.alice").is_none(),
        "Alice KEM secret existed before forced keypair failure"
    );
    assert!(
        read_mock_vault_secret(&alice_cfg, "handshake.pending.alice.bob").is_none(),
        "Alice pending state existed before forced keypair failure"
    );

    let out = handshake_init_with_forced_rng_failure(&iso, &alice_cfg, &relay, KEM_KEYPAIR_LABEL);
    assert_failure(&out);
    let text = output_text(&out);
    assert!(text.contains("rng_failure_forced"), "{text}");
    assert!(text.contains("identity_secret_unavailable"), "{text}");
    assert!(!text.contains("event=handshake_send"), "{text}");
    assert_no_secret_output(&text);

    assert!(
        !identity_self_path(&alice_cfg, "alice").exists(),
        "forced KEM keypair failure wrote Alice public identity"
    );
    assert_eq!(
        fs::read(alice_cfg.join("vault.qsv")).expect("alice vault after"),
        alice_vault_before,
        "forced KEM keypair failure changed Alice vault"
    );
    assert!(
        read_mock_vault_secret(&alice_cfg, "identity.kem_sk.alice").is_none(),
        "forced KEM keypair failure wrote Alice KEM secret"
    );
    assert!(
        read_mock_vault_secret(&alice_cfg, "identity.sig_sk.alice").is_none(),
        "forced KEM keypair failure wrote Alice signature secret"
    );
    assert!(
        read_mock_vault_secret(&alice_cfg, "handshake.pending.alice.bob").is_none(),
        "forced KEM keypair failure wrote Alice pending state"
    );
    assert!(
        path_bytes(&pending_legacy_path(&alice_cfg, "alice", "bob")).is_none(),
        "forced KEM keypair failure wrote legacy pending state"
    );
    assert!(
        path_bytes(&session_path(&alice_cfg, "bob")).is_none(),
        "forced KEM keypair failure wrote Alice session"
    );
    assert!(
        server.drain_channel(ROUTE_TOKEN_BOB).is_empty(),
        "forced KEM keypair failure emitted A1"
    );

    println!("NA0458_KEM_KEYPAIR_RNG_FAILURE_FORCED_OK");
    println!("NA0458_KEM_KEYPAIR_RNG_FAILURE_NO_PARTIAL_STATE_OK");
}

#[cfg(qsc_rng_failure_test_seam)]
#[test]
fn kem_encap_rng_failure_writes_no_responder_state_or_b1() {
    let iso = common::TestIsolation::new("na0458_kem_encap_rng_failure");
    let (alice_cfg, bob_cfg) = new_vault_pair(&iso, "encap-failure");
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

    let bob_vault_before = fs::read(bob_cfg.join("vault.qsv")).expect("bob vault before");
    assert!(
        read_mock_vault_secret(&bob_cfg, "handshake.pending.bob.alice").is_none(),
        "Bob pending state existed before forced encap failure"
    );
    assert!(
        path_bytes(&session_path(&bob_cfg, "alice")).is_none(),
        "Bob session existed before forced encap failure"
    );

    let bob_poll = handshake_poll_with_forced_rng_failure(
        &iso,
        &bob_cfg,
        KEM_ENCAP_LABEL,
        "bob",
        "alice",
        &relay,
    );
    assert_success(&bob_poll);
    let text = output_text(&bob_poll);
    assert!(text.contains("event=handshake_reject"), "{text}");
    assert!(text.contains("pq_encap_failed"), "{text}");
    assert!(!text.contains("event=handshake_send"), "{text}");
    assert!(!text.contains("msg=B1"), "{text}");
    assert_no_secret_output(&text);

    assert_eq!(
        fs::read(bob_cfg.join("vault.qsv")).expect("bob vault after"),
        bob_vault_before,
        "forced KEM encap failure changed Bob vault"
    );
    assert!(
        read_mock_vault_secret(&bob_cfg, "handshake.pending.bob.alice").is_none(),
        "forced KEM encap failure wrote Bob responder pending state"
    );
    assert!(
        path_bytes(&pending_legacy_path(&bob_cfg, "bob", "alice")).is_none(),
        "forced KEM encap failure wrote legacy pending state"
    );
    assert!(
        path_bytes(&session_path(&bob_cfg, "alice")).is_none(),
        "forced KEM encap failure wrote Bob session"
    );
    assert!(
        server.drain_channel(ROUTE_TOKEN_ALICE).is_empty(),
        "forced KEM encap failure emitted B1"
    );

    println!("NA0458_KEM_ENCAP_RNG_FAILURE_FORCED_OK");
    println!("NA0458_KEM_ENCAP_RNG_FAILURE_NO_RESPONDER_STATE_OK");
    println!("NA0458_KEM_ENCAP_RNG_FAILURE_NO_B1_OUTPUT_OK");
}

#[cfg(qsc_rng_failure_test_seam)]
#[test]
fn common_na0458_markers() {
    println!("NA0458_KEM_PROVIDER_RNG_SEAM_IMPLEMENTED_OK");
    println!("NA0458_PQ_DECAP_FAILED_BACKGROUND_PRESERVED_OK");
    println!("NA0458_PQ_ENCAP_FAILED_CAVEAT_PRESERVED_OK");
    println!("NA0458_NO_REFIMPL_CHANGE_OK");
    println!("NA0458_NO_DEPENDENCY_CHANGE_OK");
    println!("NA0458_NO_WORKFLOW_CHANGE_OK");
    println!("NA0458_NO_PUBLIC_READINESS_CLAIM_OK");
    println!("NA0458_NO_CRYPTO_COMPLETE_CLAIM_OK");
    println!("NA0458_NO_RNG_FAILURE_COMPLETE_CLAIM_OK");
    println!("NA0458_NO_PROVIDER_RNG_COMPLETE_CLAIM_OK");
    println!("NA0458_ONE_READY_INVARIANT_OK");
}
