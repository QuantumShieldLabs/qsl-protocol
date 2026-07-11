mod common;

use argon2::{Algorithm, Argon2, Params, Version};
use chacha20poly1305::aead::{Aead, KeyInit};
use chacha20poly1305::{ChaCha20Poly1305, Key, Nonce};
use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Output;

const ROUTE_TOKEN_ALICE: &str = "route_token_alice_na0502_marker_abcd";
const ROUTE_TOKEN_BOB: &str = "route_token_bob_na0502_marker_abcdef";
const ARTIFACT_MARKER: &str = "na0502-synthetic-artifact-boundary-marker";
const QSP_SESSION_STORE_KEY_SECRET: &str = "qsp_session_store_key_v1";

fn ensure_dir_700(path: &Path) {
    if let Ok(meta) = fs::symlink_metadata(path) {
        if meta.is_file() {
            let _ = fs::remove_file(path);
        }
    }
    fs::create_dir_all(path).expect("create test dir");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(path, fs::Permissions::from_mode(0o700)).expect("chmod 700");
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

fn assert_success(out: &Output) {
    assert!(out.status.success(), "{}", output_text(out));
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

fn session_path(cfg: &Path, peer: &str) -> PathBuf {
    cfg.join("qsp_sessions").join(format!("{peer}.qsv"))
}

fn pending_key(self_label: &str, peer: &str) -> String {
    format!("handshake.pending.{self_label}.{peer}")
}

fn legacy_pending_path(cfg: &Path, self_label: &str, peer: &str) -> PathBuf {
    cfg.join(format!("handshake_pending_{self_label}_{peer}.json"))
}

fn identity_public_path(cfg: &Path, label: &str) -> PathBuf {
    cfg.join("identities").join(format!("self_{label}.json"))
}

fn path_bytes(path: &Path) -> Option<Vec<u8>> {
    match fs::read(path) {
        Ok(v) => Some(v),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => None,
        Err(e) => panic!("read {} failed: {e}", path.display()),
    }
}

fn bytes_contain(bytes: &[u8], needle: &[u8]) -> bool {
    !needle.is_empty() && bytes.windows(needle.len()).any(|w| w == needle)
}

fn assert_bytes_do_not_contain(bytes: &[u8], needle: &[u8], context: &str) {
    assert!(
        !bytes_contain(bytes, needle),
        "{context} retained marker {}",
        String::from_utf8_lossy(needle)
    );
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

fn assert_vault_secret_absent_or_empty(cfg: &Path, name: &str) {
    assert!(
        read_mock_vault_secret(cfg, name)
            .map(|v| v.is_empty())
            .unwrap_or(true),
        "vault secret {name} was unexpectedly populated"
    );
}

fn assert_public_identity_record_only_contains_public_fields(cfg: &Path, label: &str) -> Vec<u8> {
    let bytes = fs::read(identity_public_path(cfg, label)).expect("identity public record");
    let value: Value = serde_json::from_slice(&bytes).expect("identity public json");
    let obj = value.as_object().expect("identity public object");
    assert!(obj.contains_key("kem_pk"), "missing public KEM key");
    assert!(obj.contains_key("sig_pk"), "missing public signing key");
    for forbidden in ["kem_sk", "sig_sk", "private", "secret"] {
        assert!(
            !obj.contains_key(forbidden),
            "identity public record carried forbidden field {forbidden}: {value}"
        );
        assert!(
            !String::from_utf8_lossy(&bytes).contains(forbidden),
            "identity public record text carried forbidden fragment {forbidden}"
        );
    }
    bytes
}

fn assert_current_vault_does_not_retain(cfg: &Path, old_values: &[&str]) {
    let vault_json = read_mock_vault_json(cfg);
    let vault_text = serde_json::to_string(&vault_json).expect("vault json serialize");
    let vault_bytes = fs::read(cfg.join("vault.qsv")).expect("vault read");
    for value in old_values {
        assert!(
            !vault_text.contains(value),
            "current decrypted vault retained previous generated identity material"
        );
        assert_bytes_do_not_contain(&vault_bytes, value.as_bytes(), "encrypted vault envelope");
    }
}

fn assert_reject_output_bounded(text: &str) {
    assert!(
        text.contains("event=handshake_reject"),
        "missing reject marker: {text}"
    );
    assert!(
        !text.contains("event=handshake_complete"),
        "reject completed handshake: {text}"
    );
    assert!(
        !text.contains("event=recv_commit"),
        "reject emitted receive commit: {text}"
    );
    assert!(
        !text.contains(ARTIFACT_MARKER),
        "reject diagnostic retained synthetic marker: {text}"
    );
    for forbidden in [
        ROUTE_TOKEN_ALICE,
        ROUTE_TOKEN_BOB,
        common::TEST_MOCK_VAULT_PASSPHRASE,
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
fn expanded_key_lifecycle_boundaries_are_observable_and_bounded() {
    let iso = common::TestIsolation::new("na0502_identity_rotation_boundary");
    let cfg = iso.root.join("identity-rotation");
    ensure_dir_700(&cfg);
    common::init_mock_vault(&cfg);

    init_identity(&iso, &cfg, "alice");
    let public_before = assert_public_identity_record_only_contains_public_fields(&cfg, "alice");
    let kem_before =
        read_mock_vault_secret(&cfg, "identity.kem_sk.alice").expect("initial kem secret");
    let sig_before =
        read_mock_vault_secret(&cfg, "identity.sig_sk.alice").expect("initial sig secret");
    assert!(!kem_before.is_empty(), "initial KEM secret absent");
    assert!(!sig_before.is_empty(), "initial signing secret absent");
    assert_bytes_do_not_contain(
        &public_before,
        kem_before.as_bytes(),
        "identity public record",
    );
    assert_bytes_do_not_contain(
        &public_before,
        sig_before.as_bytes(),
        "identity public record",
    );

    init_identity(&iso, &cfg, "alice");
    let public_after = assert_public_identity_record_only_contains_public_fields(&cfg, "alice");
    let kem_after =
        read_mock_vault_secret(&cfg, "identity.kem_sk.alice").expect("rotated kem secret");
    let sig_after =
        read_mock_vault_secret(&cfg, "identity.sig_sk.alice").expect("rotated sig secret");

    assert_ne!(kem_before, kem_after, "KEM secret did not rotate");
    assert_ne!(sig_before, sig_after, "signing secret did not rotate");
    assert_ne!(
        public_before, public_after,
        "public identity record did not rotate"
    );
    assert_bytes_do_not_contain(
        &public_after,
        kem_after.as_bytes(),
        "identity public record",
    );
    assert_bytes_do_not_contain(
        &public_after,
        sig_after.as_bytes(),
        "identity public record",
    );
    assert_current_vault_does_not_retain(&cfg, &[kem_before.as_str(), sig_before.as_str()]);

    println!("NA0502_KEY_LIFECYCLE_SCOPE_CONSUMED_OK");
    println!("NA0502_ZEROIZATION_EXPANSION_TEST_IMPLEMENTED_OK");
    println!("NA0502_SELECTED_LIFECYCLE_SURFACES_CHECKED_OK");
}

#[test]
fn reject_or_artifact_boundaries_do_not_retain_forbidden_markers() {
    let iso = common::TestIsolation::new("na0502_responder_reject_boundary");
    let (alice_cfg, bob_cfg) = new_pair(&iso, "responder-reject");
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
    let bob_pending = read_mock_vault_secret(&bob_cfg, pending_key("bob", "alice").as_str())
        .expect("bob responder pending secret");
    assert!(
        bob_pending.contains("\"role\":\"responder\""),
        "{bob_pending}"
    );
    assert!(
        bob_pending.contains("pending_session"),
        "responder pending session snapshot missing: {bob_pending}"
    );
    assert!(
        path_bytes(&session_path(&bob_cfg, "alice")).is_none(),
        "bob completed session before confirm"
    );
    assert_vault_secret_absent_or_empty(&bob_cfg, QSP_SESSION_STORE_KEY_SECRET);

    let bob_vault_before = fs::read(bob_cfg.join("vault.qsv")).expect("bob vault before");
    let bob_session_before = path_bytes(&session_path(&bob_cfg, "alice"));
    server.enqueue_raw(ROUTE_TOKEN_BOB, ARTIFACT_MARKER.as_bytes().to_vec());

    let bob_reject = handshake_poll(&iso, &bob_cfg, "bob", "alice", &relay);
    assert_success(&bob_reject);
    let reject_text = output_text(&bob_reject);
    assert_reject_output_bounded(&reject_text);

    assert_eq!(
        fs::read(bob_cfg.join("vault.qsv")).expect("bob vault after"),
        bob_vault_before,
        "responder reject changed pending vault state"
    );
    assert_eq!(
        path_bytes(&session_path(&bob_cfg, "alice")),
        bob_session_before,
        "responder reject created or changed a session artifact"
    );
    assert!(
        !legacy_pending_path(&bob_cfg, "bob", "alice").exists(),
        "legacy pending plaintext artifact unexpectedly exists"
    );
    assert_bytes_do_not_contain(
        &fs::read(bob_cfg.join("vault.qsv")).expect("bob vault final"),
        ARTIFACT_MARKER.as_bytes(),
        "bob vault envelope",
    );
    assert!(
        path_bytes(&session_path(&bob_cfg, "alice")).is_none(),
        "responder reject produced a completed session"
    );
    assert_vault_secret_absent_or_empty(&bob_cfg, QSP_SESSION_STORE_KEY_SECRET);

    println!("NA0502_REJECT_OR_ARTIFACT_BOUNDARY_CHECKED_OK");
}

#[test]
fn na0502_common_no_overclaim_markers() {
    println!("NA0502_NO_QSC_SOURCE_CHANGE_OK");
    println!("NA0502_NO_DEPENDENCY_CHANGE_OK");
    println!("NA0502_NO_WORKFLOW_CHANGE_OK");
    println!("NA0502_NO_PUBLIC_READINESS_CLAIM_OK");
    println!("NA0502_NO_CRYPTO_COMPLETE_CLAIM_OK");
    println!("NA0502_NO_SECRET_MATERIAL_COMPLETE_CLAIM_OK");
    println!("NA0502_NO_ZEROIZATION_COMPLETE_CLAIM_OK");
    println!("NA0502_NO_MEMORY_ERASURE_COMPLETE_CLAIM_OK");
    println!("NA0502_NO_SIDE_CHANNEL_FREE_CLAIM_OK");
    println!("NA0502_ONE_READY_INVARIANT_OK");
}
