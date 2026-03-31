mod common;

use assert_cmd::Command;
use quantumshield_refimpl::crypto::stdcrypto::StdCrypto;
use quantumshield_refimpl::crypto::traits::{Hash, Kmac};
use quantumshield_refimpl::suite2::ratchet::{Suite2RecvWireState, Suite2SendState};
use quantumshield_refimpl::suite2::state::Suite2SessionState;
use quantumshield_refimpl::suite2::types::{SUITE2_PROTOCOL_VERSION, SUITE2_SUITE_ID};
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

fn test_root() -> PathBuf {
    let base = if let Ok(v) = std::env::var("QSC_TEST_ROOT") {
        PathBuf::from(v).join("qsc-test-tmp")
    } else if let Ok(v) = std::env::var("CARGO_TARGET_DIR") {
        PathBuf::from(v).join("qsc-test-tmp")
    } else {
        PathBuf::from("target").join("qsc-test-tmp")
    };
    if base.is_absolute() {
        base
    } else {
        std::env::current_dir().expect("cwd").join(base)
    }
}

fn unique_dir(tag: &str) -> PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock")
        .as_nanos();
    test_root().join(format!("{tag}_{}_{}", std::process::id(), nonce))
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

fn qsc_status(cfg: &Path) -> String {
    let out = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", cfg)
        .env("QSC_MARK_FORMAT", "plain")
        .arg("status")
        .output()
        .expect("status");
    let mut text = String::from_utf8_lossy(&out.stdout).to_string();
    text.push_str(&String::from_utf8_lossy(&out.stderr));
    assert!(out.status.success(), "{text}");
    text
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

fn write_legacy_session(cfg: &Path, peer: &str, seed: u64) {
    let sessions = cfg.join("qsp_sessions");
    ensure_dir_700(&sessions);
    let st = seeded_session_state(seed, peer);
    fs::write(sessions.join(format!("{peer}.bin")), st.snapshot_bytes()).expect("legacy write");
}

#[test]
fn status_reports_missing_seed_without_session_state() {
    let base = unique_dir("na0217c_status_missing_seed");
    let cfg = base.join("cfg");
    ensure_dir_700(&base);
    ensure_dir_700(&cfg);
    common::init_mock_vault(&cfg);

    let out = qsc_status(&cfg);
    assert!(
        out.contains("event=qsp_status status=INACTIVE reason=missing_seed"),
        "{out}"
    );
}

#[test]
fn status_preserves_legacy_migration_to_active_session() {
    let base = unique_dir("na0217c_status_legacy_migration");
    let cfg = base.join("cfg");
    ensure_dir_700(&base);
    ensure_dir_700(&cfg);
    common::init_mock_vault(&cfg);
    write_legacy_session(&cfg, "peer-0", 21);

    let first = qsc_status(&cfg);
    assert!(
        first.contains("event=session_migrate ok=true action=imported reason=legacy_plaintext"),
        "{first}"
    );
    assert!(
        first.contains("event=qsp_status status=ACTIVE reason=handshake"),
        "{first}"
    );
    assert!(
        cfg.join("qsp_sessions").join("peer-0.qsv").exists(),
        "encrypted session blob missing"
    );

    let second = qsc_status(&cfg);
    assert!(
        second.contains("event=session_load ok=true format=v1"),
        "{second}"
    );
    assert!(
        second.contains("event=qsp_status status=ACTIVE reason=handshake"),
        "{second}"
    );
}
