use assert_cmd::Command;
use chacha20poly1305::aead::Aead;
use chacha20poly1305::KeyInit;
use chacha20poly1305::{ChaCha20Poly1305, Key, Nonce};
use quantumshield_refimpl::crypto::stdcrypto::StdCrypto;
use quantumshield_refimpl::crypto::traits::Hash;
use quantumshield_refimpl::suite2::establish::init_from_base_handshake;
use quantumshield_refimpl::suite2::types::{SUITE2_PROTOCOL_VERSION, SUITE2_SUITE_ID};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

mod common;

const ROUTE_TOKEN_ALICE: &str = "route_token_alice_abcdefghijklmnop";
const ROUTE_TOKEN_BOB: &str = "route_token_bob_abcdefghijklmnopqr";

fn safe_test_root() -> PathBuf {
    let root = if let Ok(v) = env::var("QSC_TEST_ROOT") {
        PathBuf::from(v)
    } else if let Ok(v) = env::var("CARGO_TARGET_DIR") {
        PathBuf::from(v)
    } else {
        PathBuf::from("target")
    };
    let root = root.join("qsc-test-tmp");
    ensure_dir_700(&root);
    root
}

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

fn output_text(out: &std::process::Output) -> String {
    let mut text = String::from_utf8_lossy(&out.stdout).to_string();
    text.push_str(&String::from_utf8_lossy(&out.stderr));
    text
}

fn session_path(cfg: &Path, peer: &str) -> PathBuf {
    cfg.join("qsp_sessions").join(format!("{}.qsv", peer))
}

fn run_qsc(cfg: &Path, args: &[&str]) -> std::process::Output {
    Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", cfg)
        .args(args)
        .output()
        .expect("qsc command")
}

fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

#[derive(serde::Deserialize)]
struct TestIdentityPublicRecord {
    kem_pk: Vec<u8>,
    #[serde(default)]
    sig_pk: Vec<u8>,
}

fn identity_record_path(cfg: &Path, label: &str) -> PathBuf {
    cfg.join("identities").join(format!("self_{}.json", label))
}

fn init_identity(cfg: &Path, label: &str) {
    let out = run_qsc(cfg, &["identity", "rotate", "--as", label, "--confirm"]);
    assert!(out.status.success(), "{}", output_text(&out));
}

fn identity_fps(cfg: &Path, label: &str) -> (String, String) {
    let bytes = fs::read(identity_record_path(cfg, label)).expect("identity record");
    let rec: TestIdentityPublicRecord = serde_json::from_slice(&bytes).expect("identity json");
    let c = StdCrypto;
    let kem_hash = c.sha512(&rec.kem_pk);
    let sig_hash = c.sha512(&rec.sig_pk);
    (
        format!("QSCFP-{}", hex_encode(&kem_hash[..16])),
        format!("QSCFP-{}", hex_encode(&sig_hash[..16])),
    )
}

fn read_mock_vault_root(cfg: &Path) -> serde_json::Value {
    let vault_path = cfg.join("vault.qsv");
    let bytes = fs::read(&vault_path).expect("vault read");
    assert!(bytes.len() > 39, "vault envelope too short");
    assert_eq!(&bytes[0..6], b"QSCV01");
    let salt_len = bytes[7] as usize;
    let nonce_len = bytes[8] as usize;
    let ct_len = u32::from_le_bytes([bytes[21], bytes[22], bytes[23], bytes[24]]) as usize;
    let mut off = 25 + salt_len;
    let nonce = &bytes[off..off + nonce_len];
    off += nonce_len;
    let ciphertext = &bytes[off..off + ct_len];
    let key = [0x42u8; 32];
    let cipher = ChaCha20Poly1305::new(Key::from_slice(&key));
    let plaintext = cipher
        .decrypt(Nonce::from_slice(nonce), ciphertext)
        .expect("vault decrypt");
    serde_json::from_slice(&plaintext).expect("vault json")
}

fn write_mock_vault_root(cfg: &Path, root: &serde_json::Value) {
    let vault_path = cfg.join("vault.qsv");
    let bytes = fs::read(&vault_path).expect("vault read");
    assert!(bytes.len() > 39, "vault envelope too short");
    assert_eq!(&bytes[0..6], b"QSCV01");
    let salt_len = bytes[7] as usize;
    let nonce_len = bytes[8] as usize;
    let off = 25 + salt_len;
    let nonce = bytes[off..off + nonce_len].to_vec();
    let mut header = bytes[..off].to_vec();
    let plaintext = serde_json::to_vec(root).expect("vault json encode");
    let key = [0x42u8; 32];
    let cipher = ChaCha20Poly1305::new(Key::from_slice(&key));
    let ciphertext = cipher
        .encrypt(Nonce::from_slice(&nonce), plaintext.as_ref())
        .expect("vault encrypt");
    header[21..25].copy_from_slice(&(ciphertext.len() as u32).to_le_bytes());
    header.extend_from_slice(&nonce);
    header.extend_from_slice(&ciphertext);
    fs::write(&vault_path, header).expect("vault write");
}

fn mock_vault_secret(cfg: &Path, name: &str) -> Option<String> {
    read_mock_vault_root(cfg)
        .get("secrets")
        .and_then(|v| v.as_object())
        .and_then(|v| v.get(name))
        .and_then(|v| v.as_str())
        .map(|v| v.to_string())
}

fn set_mock_contact_sig_fp(cfg: &Path, label: &str, sig_fp: &str) {
    let mut root = read_mock_vault_root(cfg);
    let secrets = root
        .get_mut("secrets")
        .and_then(|v| v.as_object_mut())
        .expect("vault secrets object");
    let contacts_raw = secrets
        .get("contacts.json")
        .and_then(|v| v.as_str())
        .expect("contacts secret");
    let mut contacts: serde_json::Value =
        serde_json::from_str(contacts_raw).expect("contacts secret json");
    let rec = contacts
        .get_mut("peers")
        .and_then(|v| v.get_mut(label))
        .expect("contact record");
    rec["sig_fp"] = serde_json::Value::String(sig_fp.to_string());
    if let Some(device) = rec
        .get_mut("devices")
        .and_then(|v| v.as_array_mut())
        .and_then(|v| v.first_mut())
    {
        device["sig_fp"] = serde_json::Value::String(sig_fp.to_string());
    }
    secrets.insert(
        "contacts.json".to_string(),
        serde_json::Value::String(serde_json::to_string(&contacts).expect("contacts encode")),
    );
    write_mock_vault_root(cfg, &root);
}

fn contacts_add_authenticated_with_route(
    cfg: &Path,
    label: &str,
    fp: &str,
    sig_fp: &str,
    token: &str,
) {
    let out = run_qsc(
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
    assert!(out.status.success(), "{}", output_text(&out));
    set_mock_contact_sig_fp(cfg, label, sig_fp);
}

fn seed_authenticated_pair(alice_cfg: &Path, bob_cfg: &Path) {
    init_identity(alice_cfg, "alice");
    init_identity(bob_cfg, "bob");
    let (alice_fp, alice_sig_fp) = identity_fps(alice_cfg, "alice");
    let (bob_fp, bob_sig_fp) = identity_fps(bob_cfg, "bob");
    contacts_add_authenticated_with_route(
        alice_cfg,
        "bob",
        bob_fp.as_str(),
        bob_sig_fp.as_str(),
        ROUTE_TOKEN_BOB,
    );
    contacts_add_authenticated_with_route(
        bob_cfg,
        "alice",
        alice_fp.as_str(),
        alice_sig_fp.as_str(),
        ROUTE_TOKEN_ALICE,
    );
}

fn contacts_add_with_route(cfg: &Path, label: &str, token: &str) {
    let out = run_qsc(
        cfg,
        &[
            "contacts",
            "route-set",
            "--label",
            label,
            "--route-token",
            token,
        ],
    );
    assert!(
        out.status.success(),
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
}

fn relay_inbox_set(cfg: &Path, token: &str) {
    let out = run_qsc(cfg, &["relay", "inbox-set", "--token", token]);
    assert!(
        out.status.success(),
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
}

#[test]
fn handshake_rejects_tampered_transcript_no_mutation() {
    let base = safe_test_root().join(format!("na0154_hs_tamper_{}", std::process::id()));
    let _ = fs::remove_dir_all(&base);
    ensure_dir_700(&base);
    let alice_cfg = base.join("alice");
    let bob_cfg = base.join("bob");
    ensure_dir_700(&alice_cfg);
    ensure_dir_700(&bob_cfg);
    common::init_mock_vault(&alice_cfg);
    common::init_mock_vault(&bob_cfg);
    seed_authenticated_pair(&alice_cfg, &bob_cfg);
    relay_inbox_set(&alice_cfg, ROUTE_TOKEN_ALICE);
    relay_inbox_set(&bob_cfg, ROUTE_TOKEN_BOB);

    let server = common::start_inbox_server(1024 * 1024, 16);
    let relay = server.base_url().to_string();

    let out_init = run_qsc(
        &alice_cfg,
        &[
            "handshake",
            "init",
            "--as",
            "alice",
            "--peer",
            "bob",
            "--relay",
            &relay,
        ],
    );
    assert!(out_init.status.success());

    let out_bob = run_qsc(
        &bob_cfg,
        &[
            "handshake",
            "poll",
            "--as",
            "bob",
            "--peer",
            "alice",
            "--relay",
            &relay,
            "--max",
            "4",
        ],
    );
    assert!(out_bob.status.success());

    // Tamper B1 transcript/MAC bytes before Alice consumes it.
    let mut items = server.drain_channel(ROUTE_TOKEN_ALICE);
    assert_eq!(items.len(), 1);
    if let Some(b) = items[0].get_mut(8) {
        *b = b.wrapping_add(1);
    }
    server.replace_channel(ROUTE_TOKEN_ALICE, items);

    let out_alice = run_qsc(
        &alice_cfg,
        &[
            "handshake",
            "poll",
            "--as",
            "alice",
            "--peer",
            "bob",
            "--relay",
            &relay,
            "--max",
            "4",
        ],
    );
    assert!(out_alice.status.success());
    assert!(!session_path(&alice_cfg, "bob").exists());

    let combined = String::from_utf8_lossy(&out_alice.stdout).to_string()
        + &String::from_utf8_lossy(&out_alice.stderr);
    assert!(combined.contains("handshake_reject"), "{}", combined);
}

#[test]
fn handshake_unknown_peer_rejects_without_pending_or_session_state() {
    let base = safe_test_root().join(format!("na0221_hs_unknown_peer_{}", std::process::id()));
    let _ = fs::remove_dir_all(&base);
    ensure_dir_700(&base);
    let alice_cfg = base.join("alice");
    let bob_cfg = base.join("bob");
    ensure_dir_700(&alice_cfg);
    ensure_dir_700(&bob_cfg);
    common::init_mock_vault(&alice_cfg);
    common::init_mock_vault(&bob_cfg);
    contacts_add_with_route(&alice_cfg, "bob", ROUTE_TOKEN_BOB);
    contacts_add_with_route(&bob_cfg, "alice", ROUTE_TOKEN_ALICE);
    relay_inbox_set(&alice_cfg, ROUTE_TOKEN_ALICE);
    relay_inbox_set(&bob_cfg, ROUTE_TOKEN_BOB);

    let server = common::start_inbox_server(1024 * 1024, 16);
    let relay = server.base_url().to_string();

    let out_init = run_qsc(
        &alice_cfg,
        &[
            "handshake",
            "init",
            "--as",
            "alice",
            "--peer",
            "bob",
            "--relay",
            &relay,
        ],
    );
    assert!(out_init.status.success(), "{}", output_text(&out_init));

    let out_bob = run_qsc(
        &bob_cfg,
        &[
            "handshake",
            "poll",
            "--as",
            "bob",
            "--peer",
            "alice",
            "--relay",
            &relay,
            "--max",
            "4",
        ],
    );
    assert!(out_bob.status.success(), "{}", output_text(&out_bob));
    assert!(!session_path(&bob_cfg, "alice").exists());
    assert!(
        mock_vault_secret(&bob_cfg, "handshake.pending.bob.alice")
            .map(|v| v.is_empty())
            .unwrap_or(true),
        "pending secret should be absent for unknown peer"
    );
    assert!(
        server.drain_channel(ROUTE_TOKEN_ALICE).is_empty(),
        "unknown peer reject must not emit B1"
    );
    let combined = output_text(&out_bob);
    assert!(combined.contains("identity_unknown"), "{}", combined);
    assert!(combined.contains("handshake_reject"), "{}", combined);
}

#[test]
fn handshake_initializer_rejects_missing_authenticated_establishment_commitment() {
    let c = StdCrypto;
    let err = match init_from_base_handshake(
        &c,
        true,
        SUITE2_PROTOCOL_VERSION,
        SUITE2_SUITE_ID,
        &[0x11; 16],
        &[0x22; 32],
        &[0x33; 32],
        &[0x44; 32],
        &[0x55; 32],
        false,
    ) {
        Ok(_) => panic!("unauthenticated establish must reject"),
        Err(err) => err,
    };
    assert_eq!(err, "REJECT_S2_ESTABLISH_UNAUTHENTICATED");
}

#[test]
fn handshake_pinned_identity_mismatch_fails() {
    let base = safe_test_root().join(format!("na0154_hs_pin_mismatch_{}", std::process::id()));
    let _ = fs::remove_dir_all(&base);
    ensure_dir_700(&base);
    let alice_cfg = base.join("alice");
    let alice2_cfg = base.join("alice2");
    let bob_cfg = base.join("bob");
    ensure_dir_700(&alice_cfg);
    ensure_dir_700(&alice2_cfg);
    ensure_dir_700(&bob_cfg);
    common::init_mock_vault(&alice_cfg);
    common::init_mock_vault(&alice2_cfg);
    common::init_mock_vault(&bob_cfg);
    seed_authenticated_pair(&alice_cfg, &bob_cfg);
    contacts_add_with_route(&alice2_cfg, "bob", ROUTE_TOKEN_BOB);
    relay_inbox_set(&alice_cfg, ROUTE_TOKEN_ALICE);
    relay_inbox_set(&alice2_cfg, ROUTE_TOKEN_ALICE);
    relay_inbox_set(&bob_cfg, ROUTE_TOKEN_BOB);

    let server = common::start_inbox_server(1024 * 1024, 16);
    let relay = server.base_url().to_string();

    // Establish first session and pin Alice at Bob.
    assert!(run_qsc(
        &alice_cfg,
        &[
            "handshake",
            "init",
            "--as",
            "alice",
            "--peer",
            "bob",
            "--relay",
            &relay,
        ],
    )
    .status
    .success());
    assert!(run_qsc(
        &bob_cfg,
        &[
            "handshake",
            "poll",
            "--as",
            "bob",
            "--peer",
            "alice",
            "--relay",
            &relay,
            "--max",
            "4",
        ],
    )
    .status
    .success());
    assert!(run_qsc(
        &alice_cfg,
        &[
            "handshake",
            "poll",
            "--as",
            "alice",
            "--peer",
            "bob",
            "--relay",
            &relay,
            "--max",
            "4",
        ],
    )
    .status
    .success());
    assert!(run_qsc(
        &bob_cfg,
        &[
            "handshake",
            "poll",
            "--as",
            "bob",
            "--peer",
            "alice",
            "--relay",
            &relay,
            "--max",
            "4",
        ],
    )
    .status
    .success());
    let session_before = fs::read(session_path(&bob_cfg, "alice")).expect("session before");

    // New Alice identity must be rejected against Bob's pin.
    let out_init2 = run_qsc(
        &alice2_cfg,
        &[
            "handshake",
            "init",
            "--as",
            "alice",
            "--peer",
            "bob",
            "--relay",
            &relay,
        ],
    );
    assert!(out_init2.status.success());
    let out_bob2 = run_qsc(
        &bob_cfg,
        &[
            "handshake",
            "poll",
            "--as",
            "bob",
            "--peer",
            "alice",
            "--relay",
            &relay,
            "--max",
            "4",
        ],
    );
    assert!(out_bob2.status.success());

    let session_after = fs::read(session_path(&bob_cfg, "alice")).expect("session after");
    assert_eq!(session_before, session_after);

    let combined = String::from_utf8_lossy(&out_bob2.stdout).to_string()
        + &String::from_utf8_lossy(&out_bob2.stderr);
    assert!(combined.contains("peer_mismatch"), "{}", combined);
}
