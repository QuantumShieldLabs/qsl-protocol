use assert_cmd::Command;
use quantumshield_refimpl::crypto::stdcrypto::StdCrypto;
use quantumshield_refimpl::crypto::traits::{Kmac, PqKem768};
use quantumshield_refimpl::suite2::establish::init_from_base_handshake;
use quantumshield_refimpl::suite2::types::{SUITE2_PROTOCOL_VERSION, SUITE2_SUITE_ID};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

mod common;

const ROUTE_TOKEN_ALICE: &str = "route_token_alice_abcdefghijklmnop";
const ROUTE_TOKEN_BOB: &str = "route_token_bob_abcdefghijklmnopqr";

fn kem_pk_len() -> usize {
    pqcrypto_kyber::kyber768::public_key_bytes()
}

fn kem_ct_len() -> usize {
    pqcrypto_kyber::kyber768::ciphertext_bytes()
}

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

fn session_path(cfg: &Path, peer: &str) -> PathBuf {
    cfg.join("qsp_sessions").join(format!("{}.qsv", peer))
}

fn pending_path(cfg: &Path, self_label: &str, peer: &str) -> PathBuf {
    cfg.join(format!("handshake_pending_{}_{}.json", self_label, peer))
}

fn post_raw(relay: &str, channel: &str, body: Vec<u8>) {
    let url = format!("{}/v1/push/{}", relay.trim_end_matches('/'), channel);
    let client = reqwest::blocking::Client::new();
    let _ = client.post(url).body(body).send();
}

fn run_qsc(cfg: &Path, args: &[&str]) -> std::process::Output {
    Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", cfg)
        .args(args)
        .output()
        .expect("qsc command")
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
        "{}",
        String::from_utf8_lossy(&out.stdout)
    );
}

fn relay_inbox_set(cfg: &Path, token: &str) {
    let out = run_qsc(cfg, &["relay", "inbox-set", "--token", token]);
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stdout)
    );
}

fn build_fake_resp(session_id: [u8; 16]) -> Vec<u8> {
    let mut out = Vec::with_capacity(4 + 2 + 1 + 16 + kem_ct_len() + 32 + sig_pk_len() + 32 + 32);
    out.extend_from_slice(b"QHSM");
    out.extend_from_slice(&1u16.to_be_bytes());
    out.push(2);
    out.extend_from_slice(&session_id);
    out.extend_from_slice(&vec![1u8; kem_ct_len()]);
    out.extend_from_slice(&[2u8; 32]);
    out.extend_from_slice(&vec![3u8; sig_pk_len()]);
    out.extend_from_slice(&vec![4u8; sig_sig_len()]);
    out.extend_from_slice(&[5u8; 32]);
    out
}

fn build_fake_confirm(session_id: [u8; 16]) -> Vec<u8> {
    let mut out = Vec::with_capacity(4 + 2 + 1 + 16 + 32);
    out.extend_from_slice(b"QHSM");
    out.extend_from_slice(&1u16.to_be_bytes());
    out.push(3);
    out.extend_from_slice(&session_id);
    out.extend_from_slice(&[9u8; 32]);
    out
}

fn sig_pk_len() -> usize {
    pqcrypto_dilithium::dilithium3::public_key_bytes()
}

fn sig_sig_len() -> usize {
    pqcrypto_dilithium::dilithium3::signature_bytes()
}

fn kmac_out_32(key: &[u8], label: &str, data: &[u8]) -> [u8; 32] {
    let c = StdCrypto;
    let out = c.kmac256(key, label, data, 32);
    let mut arr = [0u8; 32];
    arr.copy_from_slice(&out[..32]);
    arr
}

fn parse_hs_init(bytes: &[u8]) -> ([u8; 16], [u8; 32]) {
    let pk_len = kem_pk_len();
    let spk_len = sig_pk_len();
    assert_eq!(bytes.len(), 4 + 2 + 1 + 16 + pk_len + spk_len + 32);
    assert_eq!(&bytes[0..4], b"QHSM");
    assert_eq!(u16::from_be_bytes([bytes[4], bytes[5]]), 1u16);
    assert_eq!(bytes[6], 1u8);
    let mut sid = [0u8; 16];
    sid.copy_from_slice(&bytes[7..23]);
    let mut dh_pub = [0u8; 32];
    dh_pub.copy_from_slice(&bytes[(23 + pk_len + spk_len)..(23 + pk_len + spk_len + 32)]);
    (sid, dh_pub)
}

fn parse_hs_resp(bytes: &[u8]) -> ([u8; 16], Vec<u8>, [u8; 32]) {
    let ct_len = kem_ct_len();
    let spk_len = sig_pk_len();
    let ss_len = sig_sig_len();
    assert_eq!(
        bytes.len(),
        4 + 2 + 1 + 16 + ct_len + 32 + spk_len + ss_len + 32
    );
    assert_eq!(&bytes[0..4], b"QHSM");
    assert_eq!(u16::from_be_bytes([bytes[4], bytes[5]]), 1u16);
    assert_eq!(bytes[6], 2u8);
    let mut sid = [0u8; 16];
    sid.copy_from_slice(&bytes[7..23]);
    let kem_ct = bytes[23..(23 + ct_len)].to_vec();
    let sig_off = 23 + ct_len + 32 + spk_len;
    let mut dh_pub = [0u8; 32];
    dh_pub.copy_from_slice(&bytes[(sig_off + ss_len)..(sig_off + ss_len + 32)]);
    (sid, kem_ct, dh_pub)
}

#[derive(serde::Deserialize)]
struct PendingDump {
    kem_sk: Vec<u8>,
}

#[test]
fn handshake_two_party_establishes_session() {
    let base = safe_test_root().join(format!("na0095_handshake_ok_{}", std::process::id()));
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

    let out_init = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", &alice_cfg)
        .args([
            "handshake",
            "init",
            "--as",
            "alice",
            "--peer",
            "bob",
            "--relay",
            &relay,
        ])
        .output()
        .expect("handshake init");
    assert!(out_init.status.success());

    let out_bob = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", &bob_cfg)
        .args([
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
        ])
        .output()
        .expect("handshake poll bob");
    assert!(out_bob.status.success());

    assert!(!session_path(&bob_cfg, "alice").exists());
    assert!(
        pending_path(&bob_cfg, "bob", "alice").exists(),
        "{}{}",
        String::from_utf8_lossy(&out_bob.stdout),
        String::from_utf8_lossy(&out_bob.stderr)
    );

    let out_alice = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", &alice_cfg)
        .args([
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
        ])
        .output()
        .expect("handshake poll alice");
    assert!(out_alice.status.success());

    assert!(session_path(&alice_cfg, "bob").exists());
    assert!(!session_path(&bob_cfg, "alice").exists());

    let out_bob_confirm = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", &bob_cfg)
        .args([
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
        ])
        .output()
        .expect("handshake poll bob confirm");
    assert!(out_bob_confirm.status.success());

    assert!(session_path(&bob_cfg, "alice").exists());

    let out_status = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", &alice_cfg)
        .args(["handshake", "status", "--peer", "bob"])
        .output()
        .expect("handshake status");
    let mut combined = String::from_utf8_lossy(&out_status.stdout).to_string()
        + &String::from_utf8_lossy(&out_status.stderr);
    combined.push_str(&String::from_utf8_lossy(&out_init.stdout));
    combined.push_str(&String::from_utf8_lossy(&out_init.stderr));
    combined.push_str(&String::from_utf8_lossy(&out_bob.stdout));
    combined.push_str(&String::from_utf8_lossy(&out_bob.stderr));
    combined.push_str(&String::from_utf8_lossy(&out_alice.stdout));
    combined.push_str(&String::from_utf8_lossy(&out_alice.stderr));
    combined.push_str(&String::from_utf8_lossy(&out_bob_confirm.stdout));
    combined.push_str(&String::from_utf8_lossy(&out_bob_confirm.stderr));
    assert!(combined.contains("event=handshake_status"));
    assert!(combined.contains("status=established"));
    let pk_len_s = format!("kem_pk_len={}", kem_pk_len());
    let ct_len_s = format!("kem_ct_len={}", kem_ct_len());
    assert!(combined.contains(&pk_len_s));
    assert!(combined.contains(&ct_len_s));
    assert!(combined.contains("msg=A2"));
    for pat in [
        "TOKEN",
        "SECRET",
        "KEY",
        "PASS",
        "PRIVATE",
        "BEARER",
        "CREDENTIAL",
    ] {
        assert!(!combined.contains(pat));
    }
}

#[test]
fn handshake_tamper_rejects_no_mutation() {
    let base = safe_test_root().join(format!("na0095_handshake_tamper_{}", std::process::id()));
    let _ = fs::remove_dir_all(&base);
    ensure_dir_700(&base);
    let bob_cfg = base.join("bob");
    ensure_dir_700(&bob_cfg);
    common::init_mock_vault(&bob_cfg);
    contacts_add_with_route(&bob_cfg, "alice", ROUTE_TOKEN_ALICE);
    relay_inbox_set(&bob_cfg, ROUTE_TOKEN_BOB);

    let server = common::start_inbox_server(1024 * 1024, 16);
    let relay = server.base_url().to_string();

    post_raw(&relay, ROUTE_TOKEN_BOB, vec![0u8; 10]);

    let out_bob = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", &bob_cfg)
        .args([
            "handshake",
            "poll",
            "--as",
            "bob",
            "--peer",
            "alice",
            "--relay",
            &relay,
            "--max",
            "2",
        ])
        .output()
        .expect("handshake poll bob");
    assert!(out_bob.status.success());
    assert!(!session_path(&bob_cfg, "alice").exists());
}

#[test]
fn handshake_out_of_order_rejects_no_mutation() {
    let base = safe_test_root().join(format!("na0095_handshake_ooo_{}", std::process::id()));
    let _ = fs::remove_dir_all(&base);
    ensure_dir_700(&base);
    let alice_cfg = base.join("alice");
    ensure_dir_700(&alice_cfg);
    common::init_mock_vault(&alice_cfg);
    contacts_add_with_route(&alice_cfg, "bob", ROUTE_TOKEN_BOB);
    relay_inbox_set(&alice_cfg, ROUTE_TOKEN_ALICE);

    let server = common::start_inbox_server(1024 * 1024, 16);
    let relay = server.base_url().to_string();

    let out_init = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", &alice_cfg)
        .args([
            "handshake",
            "init",
            "--as",
            "alice",
            "--peer",
            "bob",
            "--relay",
            &relay,
        ])
        .output()
        .expect("handshake init");
    assert!(out_init.status.success());

    let mut sid = [9u8; 16];
    sid[0] = 8;
    let bad_resp = build_fake_resp(sid);
    post_raw(&relay, ROUTE_TOKEN_ALICE, bad_resp);

    let out_alice = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", &alice_cfg)
        .args([
            "handshake",
            "poll",
            "--as",
            "alice",
            "--peer",
            "bob",
            "--relay",
            &relay,
            "--max",
            "2",
        ])
        .output()
        .expect("handshake poll alice");
    assert!(out_alice.status.success());
    assert!(!session_path(&alice_cfg, "bob").exists());
    assert!(pending_path(&alice_cfg, "alice", "bob").exists());
}

#[test]
fn handshake_a2_tamper_rejects_no_mutation() {
    let base = safe_test_root().join(format!("na0099_handshake_a2_tamper_{}", std::process::id()));
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

    let out_init = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", &alice_cfg)
        .args([
            "handshake",
            "init",
            "--as",
            "alice",
            "--peer",
            "bob",
            "--relay",
            &relay,
        ])
        .output()
        .expect("handshake init");
    assert!(out_init.status.success());

    let out_bob = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", &bob_cfg)
        .args([
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
        ])
        .output()
        .expect("handshake poll bob");
    assert!(out_bob.status.success());
    assert!(
        pending_path(&bob_cfg, "bob", "alice").exists(),
        "{}{}",
        String::from_utf8_lossy(&out_bob.stdout),
        String::from_utf8_lossy(&out_bob.stderr)
    );

    let out_alice = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", &alice_cfg)
        .args([
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
        ])
        .output()
        .expect("handshake poll alice");
    assert!(out_alice.status.success());

    let mut items = server.drain_channel(ROUTE_TOKEN_BOB);
    assert!(
        !items.is_empty(),
        "queue empty after alice poll: {}{}",
        String::from_utf8_lossy(&out_alice.stdout),
        String::from_utf8_lossy(&out_alice.stderr)
    );
    let mut tampered = items.remove(0);
    if let Some(b) = tampered.get_mut(10) {
        *b = b.wrapping_add(1);
    }
    items.insert(0, tampered);
    server.replace_channel(ROUTE_TOKEN_BOB, items);

    let out_bob_confirm = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", &bob_cfg)
        .args([
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
        ])
        .output()
        .expect("handshake poll bob confirm");
    assert!(out_bob_confirm.status.success());
    assert!(!session_path(&bob_cfg, "alice").exists());
    assert!(pending_path(&bob_cfg, "bob", "alice").exists());
}

#[test]
fn handshake_a2_replay_rejects_no_mutation() {
    let base = safe_test_root().join(format!("na0099_handshake_a2_replay_{}", std::process::id()));
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

    let out_init = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", &alice_cfg)
        .args([
            "handshake",
            "init",
            "--as",
            "alice",
            "--peer",
            "bob",
            "--relay",
            &relay,
        ])
        .output()
        .expect("handshake init");
    assert!(out_init.status.success());

    let out_bob = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", &bob_cfg)
        .args([
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
        ])
        .output()
        .expect("handshake poll bob");
    assert!(out_bob.status.success());

    let out_alice = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", &alice_cfg)
        .args([
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
        ])
        .output()
        .expect("handshake poll alice");
    assert!(out_alice.status.success());

    let mut items = server.drain_channel(ROUTE_TOKEN_BOB);
    assert!(!items.is_empty());
    let first = items.remove(0);
    let replay = first.clone();
    server.replace_channel(ROUTE_TOKEN_BOB, vec![first]);

    let out_bob_confirm = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", &bob_cfg)
        .args([
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
        ])
        .output()
        .expect("handshake poll bob confirm");
    assert!(out_bob_confirm.status.success());
    let out_bob_confirm_str = String::from_utf8_lossy(&out_bob_confirm.stdout);
    assert!(out_bob_confirm_str.contains("handshake_recv msg=A2 ok=true"));
    assert!(session_path(&bob_cfg, "alice").exists());
    let sess_before = fs::read(session_path(&bob_cfg, "alice")).expect("read bob session");

    server.enqueue_raw(ROUTE_TOKEN_BOB, replay);
    let out_bob_replay = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", &bob_cfg)
        .args([
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
        ])
        .output()
        .expect("handshake poll bob replay");
    assert!(out_bob_replay.status.success());
    let out_bob_replay_str = String::from_utf8_lossy(&out_bob_replay.stdout);
    assert!(out_bob_replay_str.contains("handshake_reject"));
    assert!(out_bob_replay_str.contains("reason=decode_failed"));
    assert!(session_path(&bob_cfg, "alice").exists());
    let sess_after = fs::read(session_path(&bob_cfg, "alice")).expect("read bob session after");
    assert_eq!(sess_before, sess_after);
}

#[test]
fn handshake_a2_out_of_order_rejects_no_mutation() {
    let base = safe_test_root().join(format!("na0099_handshake_a2_ooo_{}", std::process::id()));
    let _ = fs::remove_dir_all(&base);
    ensure_dir_700(&base);
    let bob_cfg = base.join("bob");
    ensure_dir_700(&bob_cfg);
    common::init_mock_vault(&bob_cfg);
    contacts_add_with_route(&bob_cfg, "alice", ROUTE_TOKEN_ALICE);
    relay_inbox_set(&bob_cfg, ROUTE_TOKEN_BOB);

    let server = common::start_inbox_server(1024 * 1024, 16);
    let relay = server.base_url().to_string();

    let mut sid = [3u8; 16];
    sid[0] = 7;
    let a2 = build_fake_confirm(sid);
    server.enqueue_raw(ROUTE_TOKEN_BOB, a2);

    let out_bob = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", &bob_cfg)
        .args([
            "handshake",
            "poll",
            "--as",
            "bob",
            "--peer",
            "alice",
            "--relay",
            &relay,
            "--max",
            "2",
        ])
        .output()
        .expect("handshake poll bob ooo");
    assert!(out_bob.status.success());
    assert!(!session_path(&bob_cfg, "alice").exists());
}

#[test]
fn handshake_fs_identity_compromise_cannot_decrypt_recorded_message() {
    let base = safe_test_root().join(format!("na0154_handshake_fs_{}", std::process::id()));
    let _ = fs::remove_dir_all(&base);
    ensure_dir_700(&base);
    let alice_cfg = base.join("alice");
    let bob_cfg = base.join("bob");
    let attacker_cfg = base.join("attacker");
    ensure_dir_700(&alice_cfg);
    ensure_dir_700(&bob_cfg);
    ensure_dir_700(&attacker_cfg);
    common::init_mock_vault(&alice_cfg);
    common::init_mock_vault(&bob_cfg);
    common::init_mock_vault(&attacker_cfg);
    contacts_add_with_route(&alice_cfg, "bob", ROUTE_TOKEN_BOB);
    contacts_add_with_route(&bob_cfg, "alice", ROUTE_TOKEN_ALICE);
    relay_inbox_set(&alice_cfg, ROUTE_TOKEN_ALICE);
    relay_inbox_set(&bob_cfg, ROUTE_TOKEN_BOB);

    let server = common::start_inbox_server(1024 * 1024, 16);
    let relay = server.base_url().to_string();

    let out_init = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", &alice_cfg)
        .args([
            "handshake",
            "init",
            "--as",
            "alice",
            "--peer",
            "bob",
            "--relay",
            &relay,
        ])
        .output()
        .expect("handshake init");
    assert!(out_init.status.success());

    let pending_bytes = fs::read(pending_path(&alice_cfg, "alice", "bob")).expect("pending read");
    let pending: PendingDump = serde_json::from_slice(&pending_bytes).expect("pending parse");
    assert_eq!(
        pending.kem_sk.len(),
        pqcrypto_kyber::kyber768::secret_key_bytes()
    );

    // Record A1 and preserve for normal flow.
    let a1_items = server.drain_channel(ROUTE_TOKEN_BOB);
    assert_eq!(a1_items.len(), 1);
    let a1 = a1_items[0].clone();
    server.replace_channel(ROUTE_TOKEN_BOB, a1_items);

    let out_bob = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", &bob_cfg)
        .args([
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
        ])
        .output()
        .expect("handshake poll bob");
    assert!(out_bob.status.success());

    // Record B1 and preserve for normal flow.
    let b1_items = server.drain_channel(ROUTE_TOKEN_ALICE);
    assert_eq!(b1_items.len(), 1);
    let b1 = b1_items[0].clone();
    server.replace_channel(ROUTE_TOKEN_ALICE, b1_items);

    let out_alice = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", &alice_cfg)
        .args([
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
        ])
        .output()
        .expect("handshake poll alice");
    assert!(out_alice.status.success());

    let out_bob_confirm = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", &bob_cfg)
        .args([
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
        ])
        .output()
        .expect("handshake poll bob confirm");
    assert!(out_bob_confirm.status.success());
    assert!(session_path(&bob_cfg, "alice").exists());

    let payload_path = base.join("payload.bin");
    fs::write(&payload_path, b"fs-proof-message").expect("write payload");
    let out_send = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", &alice_cfg)
        .args([
            "send",
            "--transport",
            "relay",
            "--relay",
            &relay,
            "--to",
            "bob",
            "--file",
            payload_path.to_str().unwrap(),
        ])
        .output()
        .expect("send");
    assert!(
        out_send.status.success(),
        "{}{}",
        String::from_utf8_lossy(&out_send.stdout),
        String::from_utf8_lossy(&out_send.stderr)
    );

    let wire_items = server.drain_channel(ROUTE_TOKEN_BOB);
    assert_eq!(wire_items.len(), 1);
    let wire = wire_items[0].clone();
    let wire_path = attacker_cfg.join("captured.qse");
    fs::write(&wire_path, &wire).expect("write wire");

    // Reconstruct attacker's old-model state from transcript + compromised long-term identity key.
    let (sid, _a_dh_pub) = parse_hs_init(&a1);
    let (sid_b1, kem_ct, _b_dh_pub) = parse_hs_resp(&b1);
    assert_eq!(sid, sid_b1);
    let c = StdCrypto;
    let ss_pq = c
        .decap(&pending.kem_sk, &kem_ct)
        .expect("attacker decap via compromised long-term key");
    let mut pq_mix = Vec::with_capacity(17);
    pq_mix.extend_from_slice(&sid);
    pq_mix.push(0x01);
    let pq_init_ss = kmac_out_32(&ss_pq, "QSC.HS.PQ", &pq_mix);
    let mut dh_mix = Vec::with_capacity(17);
    dh_mix.extend_from_slice(&sid);
    dh_mix.push(0x02);
    let dh_init_old = kmac_out_32(&pq_init_ss, "QSC.HS.DHINIT", &dh_mix);
    let dh_self_pub = kmac_out_32(&pq_init_ss, "QSC.HS.DHSELF.B", &sid);
    let dh_peer_pub = kmac_out_32(&pq_init_ss, "QSC.HS.DHSELF.A", &sid);
    let guessed = init_from_base_handshake(
        &c,
        false,
        SUITE2_PROTOCOL_VERSION,
        SUITE2_SUITE_ID,
        &sid,
        &dh_init_old,
        &pq_init_ss,
        &dh_self_pub,
        &dh_peer_pub,
        true,
    )
    .expect("attacker state derive");
    let sess_dir = attacker_cfg.join("qsp_sessions");
    ensure_dir_700(&sess_dir);
    fs::write(sess_dir.join("alice.qsv"), guessed.snapshot_bytes()).expect("write guessed session");

    let out_attack = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", &attacker_cfg)
        .args(["receive", "--file", wire_path.to_str().unwrap()])
        .output()
        .expect("attacker receive");
    assert!(!out_attack.status.success());
    let combined = String::from_utf8_lossy(&out_attack.stdout).to_string()
        + &String::from_utf8_lossy(&out_attack.stderr);
    assert!(
        combined.contains("qsp_env_decode_failed")
            || combined.contains("recv_qsp_open_failed")
            || combined.contains("recv_qsp_decode_failed")
            || combined.contains("recv_reject_parse"),
        "{}",
        combined
    );
    assert!(!combined.contains("send_ok=true"), "{}", combined);
}
