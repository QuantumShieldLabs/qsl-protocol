mod common;

use assert_cmd::Command;
use chacha20poly1305::aead::Aead;
use chacha20poly1305::KeyInit;
use chacha20poly1305::{ChaCha20Poly1305, Key, Nonce};
use quantumshield_refimpl::crypto::stdcrypto::StdCrypto;
use quantumshield_refimpl::crypto::traits::Hash;
use std::fs;
use std::path::{Path, PathBuf};

const ROUTE_TOKEN_ALICE: &str = "route_token_alice_abcdefghijklmnop";
const ROUTE_TOKEN_BOB: &str = "route_token_bob_abcdefghijklmnopqr";

fn safe_test_root() -> PathBuf {
    let root = std::env::temp_dir().join("qsc-test-tmp");
    ensure_dir_700(&root);
    root
}

fn ensure_dir_700(path: &Path) {
    let _ = fs::create_dir_all(path);
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let _ = fs::set_permissions(path, fs::Permissions::from_mode(0o700));
    }
}

fn create_dir_700(path: &Path) {
    let _ = fs::remove_dir_all(path);
    ensure_dir_700(path);
}

fn output_text(out: &std::process::Output) -> String {
    let mut text = String::from_utf8_lossy(&out.stdout).to_string();
    text.push_str(&String::from_utf8_lossy(&out.stderr));
    text
}

fn run_qsc(cfg: &Path, args: &[&str]) -> std::process::Output {
    Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", cfg)
        .env("QSC_MARK_FORMAT", "plain")
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

fn relay_inbox_set(cfg: &Path, token: &str) {
    let out = run_qsc(cfg, &["relay", "inbox-set", "--token", token]);
    assert!(out.status.success(), "{}", output_text(&out));
}

#[test]
fn handshake_status_tracks_establishment_after_full_exchange() {
    let base = safe_test_root().join(format!("na0217i_handshake_status_{}", std::process::id()));
    create_dir_700(&base);
    let alice_cfg = base.join("alice");
    let bob_cfg = base.join("bob");
    create_dir_700(&alice_cfg);
    create_dir_700(&bob_cfg);
    common::init_mock_vault(&alice_cfg);
    common::init_mock_vault(&bob_cfg);
    seed_authenticated_pair(&alice_cfg, &bob_cfg);
    relay_inbox_set(&alice_cfg, ROUTE_TOKEN_ALICE);
    relay_inbox_set(&bob_cfg, ROUTE_TOKEN_BOB);

    let server = common::start_inbox_server(1024 * 1024, 16);
    let relay = server.base_url().to_string();

    let initial = run_qsc(&alice_cfg, &["handshake", "status", "--peer", "bob"]);
    assert!(initial.status.success(), "{}", output_text(&initial));
    let initial_text = output_text(&initial);
    assert!(
        initial_text.contains("event=handshake_status status=no_session peer=bob"),
        "{}",
        initial_text
    );

    let alice_init = run_qsc(
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
    assert!(alice_init.status.success(), "{}", output_text(&alice_init));

    let bob_poll = run_qsc(
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
    assert!(bob_poll.status.success(), "{}", output_text(&bob_poll));

    let alice_poll = run_qsc(
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
    assert!(alice_poll.status.success(), "{}", output_text(&alice_poll));

    let bob_confirm = run_qsc(
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
    assert!(
        bob_confirm.status.success(),
        "{}",
        output_text(&bob_confirm)
    );

    let alice_status = run_qsc(&alice_cfg, &["handshake", "status", "--peer", "bob"]);
    assert!(
        alice_status.status.success(),
        "{}",
        output_text(&alice_status)
    );
    let alice_status_text = output_text(&alice_status);
    assert!(
        alice_status_text.contains("event=handshake_status status=established peer=bob"),
        "{}",
        alice_status_text
    );
    assert!(
        alice_status_text.contains("send_ready=yes"),
        "{}",
        alice_status_text
    );

    let bob_status = run_qsc(&bob_cfg, &["handshake", "status", "--peer", "alice"]);
    assert!(bob_status.status.success(), "{}", output_text(&bob_status));
    let bob_status_text = output_text(&bob_status);
    assert!(
        bob_status_text.contains("event=handshake_status status=established_recv_only peer=alice"),
        "{}",
        bob_status_text
    );
    assert!(
        bob_status_text.contains("send_ready=no"),
        "{}",
        bob_status_text
    );
    assert!(
        bob_status_text.contains("send_ready_reason=chainkey_unset"),
        "{}",
        bob_status_text
    );
}
