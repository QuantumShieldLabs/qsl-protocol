use assert_cmd::Command;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

mod common;

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
    cfg.join("qsp_sessions").join(format!("{}.bin", peer))
}

fn pending_path(cfg: &Path, self_label: &str, peer: &str) -> PathBuf {
    cfg.join(format!("handshake_pending_{}_{}.json", self_label, peer))
}

fn post_raw(relay: &str, channel: &str, body: Vec<u8>) {
    let url = format!("{}/v1/push/{}", relay.trim_end_matches('/'), channel);
    let client = reqwest::blocking::Client::new();
    let _ = client.post(url).body(body).send();
}

fn build_fake_resp(session_id: [u8; 16]) -> Vec<u8> {
    let mut out = Vec::with_capacity(4 + 2 + 1 + 16 + kem_ct_len() + 32);
    out.extend_from_slice(b"QHSM");
    out.extend_from_slice(&1u16.to_be_bytes());
    out.push(2);
    out.extend_from_slice(&session_id);
    out.extend_from_slice(&vec![1u8; kem_ct_len()]);
    out.extend_from_slice(&[2u8; 32]);
    out
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

    assert!(session_path(&alice_cfg, "bob").exists());
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
    assert!(combined.contains("event=handshake_status"));
    assert!(combined.contains("status=established"));
    let pk_len_s = format!("kem_pk_len={}", kem_pk_len());
    let ct_len_s = format!("kem_ct_len={}", kem_ct_len());
    assert!(combined.contains(&pk_len_s));
    assert!(combined.contains(&ct_len_s));
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

    let server = common::start_inbox_server(1024 * 1024, 16);
    let relay = server.base_url().to_string();

    post_raw(&relay, "hs-bob", vec![0u8; 10]);

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
    post_raw(&relay, "hs-alice", bad_resp);

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
