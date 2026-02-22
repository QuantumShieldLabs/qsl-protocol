mod common;

use assert_cmd::Command;
use quantumshield_refimpl::crypto::stdcrypto::StdCrypto;
use quantumshield_refimpl::crypto::traits::{Hash, Kmac};
use quantumshield_refimpl::suite2::ratchet::{Suite2RecvWireState, Suite2SendState};
use quantumshield_refimpl::suite2::state::Suite2SessionState;
use quantumshield_refimpl::suite2::types::{SUITE2_PROTOCOL_VERSION, SUITE2_SUITE_ID};
use std::collections::BTreeSet;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

const ROUTE_TOKEN_PEER0: &str = "route_token_peer0_abcdefghijklmnop";

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

fn kmac_out<const N: usize>(kmac: &StdCrypto, key: &[u8], label: &str, data: &[u8]) -> [u8; N] {
    let out = kmac.kmac256(key, label, data, N);
    out[..N].try_into().expect("kmac output")
}

fn seeded_session_state(seed: u64, peer: &str) -> Suite2SessionState {
    let c = StdCrypto;
    let seed_bytes = seed.to_le_bytes();
    let seed_hash = c.sha512(&seed_bytes);
    let mut seed_key = [0u8; 32];
    seed_key.copy_from_slice(&seed_hash[..32]);
    let base = kmac_out::<32>(&c, &seed_key, "QSC.QSP.BASE", peer.as_bytes());
    let session_id = kmac_out::<16>(&c, &base, "QSC.QSP.SID", peer.as_bytes());
    let hk = kmac_out::<32>(&c, &base, "QSC.QSP.HK", b"");
    let ck_ec = kmac_out::<32>(&c, &base, "QSC.QSP.CK.EC", b"");
    let ck_pq = kmac_out::<32>(&c, &base, "QSC.QSP.CK.PQ", b"");
    let rk = kmac_out::<32>(&c, &base, "QSC.QSP.RK", b"");
    let dh_pub = kmac_out::<32>(&c, &base, "QSC.QSP.DH", b"");
    let send = Suite2SendState {
        session_id,
        protocol_version: SUITE2_PROTOCOL_VERSION,
        suite_id: SUITE2_SUITE_ID,
        dh_pub,
        hk_s: hk,
        ck_ec,
        ck_pq,
        ns: 0,
        pn: 0,
    };
    let recv = Suite2RecvWireState {
        session_id,
        protocol_version: SUITE2_PROTOCOL_VERSION,
        suite_id: SUITE2_SUITE_ID,
        dh_pub,
        hk_r: hk,
        rk,
        ck_ec,
        ck_pq_send: ck_pq,
        ck_pq_recv: ck_pq,
        nr: 0,
        role_is_a: true,
        peer_max_adv_id_seen: 0,
        known_targets: BTreeSet::new(),
        consumed_targets: BTreeSet::new(),
        tombstoned_targets: BTreeSet::new(),
        mkskipped: Vec::new(),
    };
    Suite2SessionState { send, recv }
}

fn run_status(cfg: &Path) -> String {
    let out = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", cfg)
        .env("QSC_MARK_FORMAT", "plain")
        .args(["status"])
        .output()
        .expect("status");
    String::from_utf8_lossy(&out.stdout).to_string() + &String::from_utf8_lossy(&out.stderr)
}

fn write_legacy_session(cfg: &Path, peer: &str, seed: u64) {
    let sessions = cfg.join("qsp_sessions");
    ensure_dir_700(&sessions);
    let st = seeded_session_state(seed, peer);
    fs::write(sessions.join(format!("{}.bin", peer)), st.snapshot_bytes()).expect("legacy write");
}

fn ensure_peer0_route_token(cfg: &Path) {
    let out = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", cfg)
        .env("QSC_QSP_SEED", "1")
        .env("QSC_ALLOW_SEED_FALLBACK", "1")
        .args([
            "contacts",
            "add",
            "--label",
            "peer-0",
            "--fp",
            "fp-test",
            "--route-token",
            ROUTE_TOKEN_PEER0,
        ])
        .output()
        .expect("contacts add");
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stdout)
    );
}

#[test]
fn session_not_plaintext_on_disk() {
    let base = safe_test_root().join(format!("na0109_not_plaintext_{}", std::process::id()));
    let _ = fs::remove_dir_all(&base);
    ensure_dir_700(&base);
    let cfg = base.join("cfg");
    ensure_dir_700(&cfg);
    common::init_mock_vault(&cfg);
    ensure_peer0_route_token(&cfg);
    let relay = common::start_inbox_server(1024 * 1024, 32);
    let payload = base.join("payload.txt");
    fs::write(&payload, b"fixture-material-42").unwrap();

    let out = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", &cfg)
        .env("QSC_QSP_SEED", "1")
        .env("QSC_ALLOW_SEED_FALLBACK", "1")
        .env("QSC_MARK_FORMAT", "plain")
        .args([
            "send",
            "--transport",
            "relay",
            "--relay",
            relay.base_url(),
            "--to",
            "peer-0",
            "--file",
            payload.to_str().unwrap(),
        ])
        .output()
        .expect("send");
    assert!(out.status.success());

    let blob = cfg.join("qsp_sessions").join("peer-0.qsv");
    assert!(blob.exists(), "encrypted session blob missing");
    let legacy = cfg.join("qsp_sessions").join("peer-0.bin");
    if legacy.exists() {
        let legacy_bytes = fs::read(&legacy).unwrap();
        assert_eq!(legacy_bytes, b"QSC_SESSION_MIGRATED_V1\n");
    }

    let scan_needles = [
        "pq_init_ss",
        "session_id",
        "\"send\"",
        "\"recv\"",
        "ck_ec",
        "ck_pq",
        "hk_s",
        "hk_r",
        "rk",
    ];
    for entry in fs::read_dir(cfg.join("qsp_sessions")).unwrap() {
        let path = entry.unwrap().path();
        let bytes = fs::read(&path).unwrap();
        let text = String::from_utf8_lossy(&bytes).to_string();
        for needle in scan_needles {
            assert!(
                !text.contains(needle),
                "plaintext marker {needle} found in {}",
                path.display()
            );
        }
    }
}

#[test]
fn tamper_session_blob_rejects_no_mutation() {
    let base = safe_test_root().join(format!("na0109_tamper_{}", std::process::id()));
    let _ = fs::remove_dir_all(&base);
    ensure_dir_700(&base);
    let cfg = base.join("cfg");
    ensure_dir_700(&cfg);
    common::init_mock_vault(&cfg);
    ensure_peer0_route_token(&cfg);
    write_legacy_session(&cfg, "peer-0", 9);

    let first = run_status(&cfg);
    assert!(first.contains("event=session_migrate ok=true action=imported"));
    assert!(first.contains("event=qsp_status status=ACTIVE reason=handshake"));

    let blob = cfg.join("qsp_sessions").join("peer-0.qsv");
    assert!(blob.exists());
    let mut tampered = fs::read(&blob).unwrap();
    let idx = tampered.len().saturating_sub(1);
    tampered[idx] ^= 0x01;
    fs::write(&blob, &tampered).unwrap();

    let second = run_status(&cfg);
    assert!(second.contains("event=error code=session_integrity_failed"));
    assert!(second.contains("event=qsp_status status=INACTIVE reason=session_invalid"));
    let after = fs::read(&blob).unwrap();
    assert_eq!(tampered, after, "tampered blob should remain unchanged");
}

#[test]
fn migration_idempotent() {
    let base = safe_test_root().join(format!("na0109_migrate_idem_{}", std::process::id()));
    let _ = fs::remove_dir_all(&base);
    ensure_dir_700(&base);
    let cfg = base.join("cfg");
    ensure_dir_700(&cfg);
    common::init_mock_vault(&cfg);
    write_legacy_session(&cfg, "peer-0", 11);

    let out1 = run_status(&cfg);
    assert!(out1.contains("event=session_migrate ok=true action=imported"));
    let blob = cfg.join("qsp_sessions").join("peer-0.qsv");
    let legacy = cfg.join("qsp_sessions").join("peer-0.bin");
    let blob_after_first = fs::read(&blob).unwrap();
    let legacy_after_first = fs::read(&legacy).unwrap();
    assert_eq!(legacy_after_first, b"QSC_SESSION_MIGRATED_V1\n");

    let out2 = run_status(&cfg);
    assert!(out2.contains("event=session_load ok=true format=v1"));
    let blob_after_second = fs::read(&blob).unwrap();
    let legacy_after_second = fs::read(&legacy).unwrap();
    assert_eq!(blob_after_first, blob_after_second);
    assert_eq!(legacy_after_first, legacy_after_second);
}

#[test]
fn migration_blocked_without_vault_no_mutation() {
    let base = safe_test_root().join(format!("na0109_migrate_blocked_{}", std::process::id()));
    let _ = fs::remove_dir_all(&base);
    ensure_dir_700(&base);
    let cfg = base.join("cfg");
    ensure_dir_700(&cfg);
    write_legacy_session(&cfg, "peer-0", 13);

    let legacy = cfg.join("qsp_sessions").join("peer-0.bin");
    let before = fs::read(&legacy).unwrap();
    let out = run_status(&cfg);
    assert!(out.contains("event=session_migrate code=migration_blocked ok=false action=skipped reason=vault_unavailable"));
    assert!(out.contains("event=qsp_status status=INACTIVE reason=session_invalid"));
    let after = fs::read(&legacy).unwrap();
    assert_eq!(before, after, "legacy session should remain unchanged");
    assert!(!cfg.join("qsp_sessions").join("peer-0.qsv").exists());
}

#[test]
fn no_secrets_in_output() {
    let base = safe_test_root().join(format!("na0109_no_secrets_{}", std::process::id()));
    let _ = fs::remove_dir_all(&base);
    ensure_dir_700(&base);
    let cfg = base.join("cfg");
    ensure_dir_700(&cfg);
    common::init_mock_vault(&cfg);
    ensure_peer0_route_token(&cfg);
    let relay = common::start_inbox_server(1024 * 1024, 32);
    let payload = base.join("payload.txt");
    fs::write(&payload, b"fixture-material-42").unwrap();

    let send = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", &cfg)
        .env("QSC_QSP_SEED", "1")
        .env("QSC_ALLOW_SEED_FALLBACK", "1")
        .env("QSC_MARK_FORMAT", "plain")
        .args([
            "send",
            "--transport",
            "relay",
            "--relay",
            relay.base_url(),
            "--to",
            "peer-0",
            "--file",
            payload.to_str().unwrap(),
        ])
        .output()
        .expect("send");
    assert!(send.status.success());
    let status = run_status(&cfg);
    let combined = String::from_utf8_lossy(&send.stdout).to_string()
        + &String::from_utf8_lossy(&send.stderr)
        + &status;
    for pat in [
        "TOKEN",
        "SECRET",
        "KEY",
        "PASS",
        "PRIVATE",
        "BEARER",
        "CREDENTIAL",
        "fixture-material-42",
    ] {
        assert!(!combined.contains(pat), "secret-like token leaked: {pat}");
    }
}
