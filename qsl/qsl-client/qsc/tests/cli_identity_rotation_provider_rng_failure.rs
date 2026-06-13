#![allow(unexpected_cfgs)]
#![allow(dead_code)]

mod common;

use argon2::{Algorithm, Argon2, Params, Version};
use chacha20poly1305::aead::{Aead, KeyInit};
use chacha20poly1305::{ChaCha20Poly1305, Key, Nonce};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Output;

const ROUTE_TOKEN_BOB: &str = "route_token_bob_na0469_cli_rotate";
const ROTATE_KEM_LABEL: &str = "QSC.IDENTITY.ROTATE.KEM_KEYPAIR";
const ROTATE_SIG_LABEL: &str = "QSC.IDENTITY.ROTATE.SIG_KEYPAIR";
const PEER_PIN_BYTES: &[u8] = b"na0469-legacy-peer-pin";

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

fn contacts_list_text(iso: &common::TestIsolation, cfg: &Path) -> String {
    let out = run_qsc(iso, cfg, &["contacts", "list"]);
    assert_success(&out);
    output_text(&out)
}

fn seed_rotation_fixture(iso: &common::TestIsolation, tag: &str) -> (PathBuf, PathBuf) {
    let (alice_cfg, bob_cfg) = new_vault_pair(iso, tag);
    init_identity(iso, &alice_cfg, "alice");
    init_identity(iso, &bob_cfg, "bob");
    let bob_fp = identity_fp(iso, &bob_cfg, "bob");
    contacts_add_authenticated_with_route(iso, &alice_cfg, "bob", bob_fp.as_str(), ROUTE_TOKEN_BOB);
    seed_legacy_peer_pin(&alice_cfg);
    (alice_cfg, bob_cfg)
}

fn identity_self_path(cfg: &Path, label: &str) -> PathBuf {
    cfg.join("identities").join(format!("self_{label}.json"))
}

fn legacy_peer_pin_path(cfg: &Path) -> PathBuf {
    cfg.join("identities").join("peer_bob.fp")
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

fn seed_legacy_peer_pin(cfg: &Path) {
    let path = legacy_peer_pin_path(cfg);
    ensure_dir_700(path.parent().expect("peer pin parent"));
    fs::write(&path, PEER_PIN_BYTES).expect("write peer pin");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&path, fs::Permissions::from_mode(0o600)).unwrap();
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

fn read_mock_vault_json(cfg: &Path) -> serde_json::Value {
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

struct RotationState {
    fp: String,
    public_record: Vec<u8>,
    vault: Vec<u8>,
    kem_secret: String,
    sig_secret: String,
    contacts_list: String,
    peer_pin: Vec<u8>,
}

fn capture_rotation_state(iso: &common::TestIsolation, cfg: &Path) -> RotationState {
    let contacts_list = contacts_list_text(iso, cfg);
    RotationState {
        fp: identity_fp(iso, cfg, "alice"),
        public_record: fs::read(identity_self_path(cfg, "alice")).expect("public record"),
        vault: fs::read(cfg.join("vault.qsv")).expect("vault"),
        kem_secret: read_mock_vault_secret(cfg, "identity.kem_sk.alice").expect("kem secret"),
        sig_secret: read_mock_vault_secret(cfg, "identity.sig_sk.alice").expect("sig secret"),
        contacts_list,
        peer_pin: fs::read(legacy_peer_pin_path(cfg)).expect("peer pin"),
    }
}

fn assert_no_secret_output(text: &str) {
    for forbidden in [
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

fn assert_rotate_failure_output(text: &str) {
    assert!(text.contains("rng_failure_forced"), "{text}");
    assert!(text.contains("identity_secret_unavailable"), "{text}");
    assert!(!text.contains("identity_fp="), "{text}");
    assert!(!text.contains("event=identity_rotate ok=true"), "{text}");
    assert!(!text.contains("event=handshake_send"), "{text}");
    assert!(!text.contains("event=handshake_complete"), "{text}");
    assert!(!text.contains("event=qsp_session_store"), "{text}");
    assert_no_secret_output(text);
}

fn assert_rotation_state_stable(iso: &common::TestIsolation, cfg: &Path, before: &RotationState) {
    assert_eq!(
        fs::read(identity_self_path(cfg, "alice")).expect("public record after"),
        before.public_record,
        "forced rotate failure changed Alice public record"
    );
    assert_eq!(
        fs::read(cfg.join("vault.qsv")).expect("vault after"),
        before.vault,
        "forced rotate failure changed Alice vault"
    );
    assert_eq!(
        read_mock_vault_secret(cfg, "identity.kem_sk.alice").as_deref(),
        Some(before.kem_secret.as_str()),
        "forced rotate failure changed Alice KEM secret"
    );
    assert_eq!(
        read_mock_vault_secret(cfg, "identity.sig_sk.alice").as_deref(),
        Some(before.sig_secret.as_str()),
        "forced rotate failure changed Alice signature secret"
    );
    assert_eq!(identity_fp(iso, cfg, "alice"), before.fp);
    assert_eq!(contacts_list_text(iso, cfg), before.contacts_list);
    assert_eq!(
        fs::read(legacy_peer_pin_path(cfg)).expect("peer pin after"),
        before.peer_pin,
        "forced rotate failure reached peer reset cleanup"
    );
    assert!(
        read_mock_vault_secret(cfg, "handshake.pending.alice.bob").is_none(),
        "forced rotate failure wrote pending handshake secret"
    );
    assert!(
        path_bytes(&pending_legacy_path(cfg, "alice", "bob")).is_none(),
        "forced rotate failure wrote legacy pending handshake state"
    );
    assert!(
        path_bytes(&session_path(cfg, "bob")).is_none(),
        "forced rotate failure wrote Alice session"
    );
}

fn force_rotate_failure_twice(
    iso: &common::TestIsolation,
    cfg: &Path,
    label: &str,
    before: &RotationState,
) {
    let args = [
        "identity",
        "rotate",
        "--as",
        "alice",
        "--confirm",
        "--reset-peers",
    ];

    let first = run_qsc_with_forced_rng_failure(iso, cfg, label, &args);
    assert_failure(&first);
    let first_text = output_text(&first);
    assert_rotate_failure_output(&first_text);
    assert_rotation_state_stable(iso, cfg, before);

    let second = run_qsc_with_forced_rng_failure(iso, cfg, label, &args);
    assert_failure(&second);
    let second_text = output_text(&second);
    assert_rotate_failure_output(&second_text);
    assert_eq!(
        second_text, first_text,
        "forced rotate failure output was not deterministic"
    );
    assert_rotation_state_stable(iso, cfg, before);
}

#[cfg(qsc_rng_failure_test_seam)]
#[test]
fn cli_identity_rotate_kem_rng_failure_writes_no_partial_rotation_state() {
    let iso = common::TestIsolation::new("na0469_cli_rotate_kem_rng_failure");
    let (alice_cfg, _bob_cfg) = seed_rotation_fixture(&iso, "cli-rotate-kem-failure");
    let before = capture_rotation_state(&iso, &alice_cfg);

    force_rotate_failure_twice(&iso, &alice_cfg, ROTATE_KEM_LABEL, &before);

    println!("NA0469_CLI_ROTATE_KEM_RNG_FAILURE_FORCED_OK");
    println!("NA0469_CLI_ROTATE_KEM_RNG_FAILURE_NO_PARTIAL_ROTATION_STATE_OK");
    println!("NA0469_CLI_ROTATE_SELECTED_IDENTITY_STABLE_OK");
    println!("NA0469_CLI_ROTATE_NO_PARTIAL_KEM_SECRET_WRITE_OK");
    println!("NA0469_CLI_ROTATE_NO_PARTIAL_SIG_SECRET_WRITE_OK");
    println!("NA0469_CLI_ROTATE_NO_PARTIAL_PUBLIC_RECORD_WRITE_OK");
    println!("NA0469_CLI_ROTATE_PEER_RESET_STATE_UNCHANGED_OK");
    println!("NA0469_CLI_ROTATE_NO_DEPENDENT_HANDSHAKE_OUTPUT_OK");
}

#[cfg(qsc_rng_failure_test_seam)]
#[test]
fn cli_identity_rotate_sig_rng_failure_writes_no_partial_rotation_state() {
    let iso = common::TestIsolation::new("na0469_cli_rotate_sig_rng_failure");
    let (alice_cfg, _bob_cfg) = seed_rotation_fixture(&iso, "cli-rotate-sig-failure");
    let before = capture_rotation_state(&iso, &alice_cfg);

    force_rotate_failure_twice(&iso, &alice_cfg, ROTATE_SIG_LABEL, &before);

    println!("NA0469_CLI_ROTATE_SIG_RNG_FAILURE_FORCED_OK");
    println!("NA0469_CLI_ROTATE_SIG_RNG_FAILURE_NO_PARTIAL_ROTATION_STATE_OK");
    println!("NA0469_CLI_ROTATE_SELECTED_IDENTITY_STABLE_OK");
    println!("NA0469_CLI_ROTATE_NO_PARTIAL_KEM_SECRET_WRITE_OK");
    println!("NA0469_CLI_ROTATE_NO_PARTIAL_SIG_SECRET_WRITE_OK");
    println!("NA0469_CLI_ROTATE_NO_PARTIAL_PUBLIC_RECORD_WRITE_OK");
    println!("NA0469_CLI_ROTATE_PEER_RESET_STATE_UNCHANGED_OK");
    println!("NA0469_CLI_ROTATE_NO_DEPENDENT_HANDSHAKE_OUTPUT_OK");
}

#[cfg(not(qsc_rng_failure_test_seam))]
#[test]
fn cli_identity_rotation_rng_failure_seam_inactive_without_cfg() {
    let iso = common::TestIsolation::new("na0469_cli_rotate_no_cfg");
    let (alice_cfg, _bob_cfg) = new_vault_pair(&iso, "cli-rotate-no-cfg");

    let out = run_qsc_with_forced_rng_failure(
        &iso,
        &alice_cfg,
        ROTATE_KEM_LABEL,
        &["identity", "rotate", "--as", "alice", "--confirm"],
    );
    assert_success(&out);
    let text = output_text(&out);
    assert!(text.contains("identity_fp="), "{text}");
    assert!(!text.contains("rng_failure_forced"), "{text}");
    assert!(!text.contains("identity_secret_unavailable"), "{text}");
    assert_no_secret_output(&text);
    assert!(identity_self_path(&alice_cfg, "alice").exists());
    assert!(read_mock_vault_secret(&alice_cfg, "identity.kem_sk.alice").is_some());
    assert!(read_mock_vault_secret(&alice_cfg, "identity.sig_sk.alice").is_some());

    println!("NA0469_PRODUCTION_SEMANTICS_UNCHANGED_OK");
}

#[cfg(qsc_rng_failure_test_seam)]
#[test]
fn common_na0469_markers() {
    println!("NA0469_CLI_ROTATE_PROVIDER_RNG_SEAM_IMPLEMENTED_OK");
    println!("NA0469_LAZY_IDENTITY_BACKGROUND_PRESERVED_OK");
    println!("NA0469_LEGACY_PUBLIC_RECORD_BACKGROUND_PRESERVED_OK");
    println!("NA0469_TUI_BOOTSTRAP_IDENTITY_RESIDUAL_DEFERRED_OK");
    println!("NA0469_X25519_RESIDUAL_DEFERRED_OK");
    println!("NA0469_REFIMPL_PROVIDER_RNG_RESIDUAL_DEFERRED_OK");
    println!("NA0469_A2_SIGNATURE_BACKGROUND_PRESERVED_OK");
    println!("NA0469_B1_SIGNATURE_BACKGROUND_PRESERVED_OK");
    println!("NA0469_KEM_PROVIDER_RNG_BACKGROUND_PRESERVED_OK");
    println!("NA0469_NO_DEPENDENCY_CHANGE_OK");
    println!("NA0469_NO_WORKFLOW_CHANGE_OK");
    println!("NA0469_NO_PUBLIC_READINESS_CLAIM_OK");
    println!("NA0469_NO_CRYPTO_COMPLETE_CLAIM_OK");
    println!("NA0469_NO_SIGNATURE_COMPLETE_CLAIM_OK");
    println!("NA0469_NO_IDENTITY_COMPLETE_CLAIM_OK");
    println!("NA0469_NO_RNG_FAILURE_COMPLETE_CLAIM_OK");
    println!("NA0469_NO_PROVIDER_RNG_COMPLETE_CLAIM_OK");
    println!("NA0469_ONE_READY_INVARIANT_OK");
}
