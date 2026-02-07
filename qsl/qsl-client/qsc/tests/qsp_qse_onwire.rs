mod common;

use quantumshield_refimpl::crypto::stdcrypto::StdCrypto;
use quantumshield_refimpl::crypto::traits::{Hash, Kmac};
use quantumshield_refimpl::qse::{Envelope, EnvelopeProfile};
use quantumshield_refimpl::suite2::ratchet::{Suite2RecvWireState, Suite2SendState};
use quantumshield_refimpl::suite2::state::Suite2SessionState;
use quantumshield_refimpl::suite2::types::{SUITE2_PROTOCOL_VERSION, SUITE2_SUITE_ID};
use quantumshield_refimpl::suite2::{recv_wire_canon, send_wire_canon};
use reqwest::blocking::Client as HttpClient;
use serde::Deserialize;
use std::collections::BTreeSet;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Deserialize)]
struct InboxPullItem {
    #[serde(rename = "id")]
    _id: String,
    data: Vec<u8>,
}

#[derive(Deserialize)]
struct InboxPullResp {
    items: Vec<InboxPullItem>,
}

const QSE_ENV_VERSION_V1: u16 = 0x0100;

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

fn qsp_session_for_channel(seed: u64, channel: &str) -> Suite2SessionState {
    let c = StdCrypto;
    let seed_bytes = seed.to_le_bytes();
    let seed_hash = c.sha512(&seed_bytes);
    let mut seed_key = [0u8; 32];
    seed_key.copy_from_slice(&seed_hash[..32]);

    let base = kmac_out::<32>(&c, &seed_key, "QSC.QSP.BASE", channel.as_bytes());
    let session_id = kmac_out::<16>(&c, &base, "QSC.QSP.SID", channel.as_bytes());
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

fn qsp_pack_for_channel(seed: u64, channel: &str, plaintext: &[u8]) -> Vec<u8> {
    let c = StdCrypto;
    let st = qsp_session_for_channel(seed, channel);
    let out = send_wire_canon(&c, &c, &c, st.send, 0, plaintext).expect("send_wire");
    let mut env = Envelope {
        env_version: QSE_ENV_VERSION_V1,
        flags: 0,
        route_token: Vec::new(),
        timestamp_bucket: 0,
        payload: out.wire,
        padding: Vec::new(),
    };
    let encoded_len = env.encode().len();
    let min_len = EnvelopeProfile::Standard.min_size_bytes();
    if encoded_len < min_len {
        let need = min_len - encoded_len;
        let pad = c.kmac256(&env.payload, "QSC.QSP.PAD", b"", need);
        env = env
            .pad_to_profile(EnvelopeProfile::Standard, &pad)
            .expect("pad");
    }
    env.encode()
}

#[test]
fn on_wire_is_envelope_not_raw() {
    let server = common::start_inbox_server(1024 * 1024, 32);
    let base = safe_test_root().join(format!("na0092_onwire_{}", std::process::id()));
    ensure_dir_700(&base);
    let cfg = base.join("cfg");
    ensure_dir_700(&cfg);

    let payload = base.join("msg.bin");
    let plaintext = b"hello-qsp".to_vec();
    fs::write(&payload, &plaintext).expect("write payload");

    let output = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", &cfg)
        .env("QSC_QSP_SEED", "1")
        .env("QSC_ALLOW_SEED_FALLBACK", "1")
        .env("QSC_MARK_FORMAT", "plain")
        .args([
            "send",
            "--transport",
            "relay",
            "--relay",
            server.base_url(),
            "--to",
            "peer",
            "--file",
            payload.to_str().unwrap(),
        ])
        .output()
        .expect("run send");
    assert!(output.status.success(), "send failed");

    let url = format!("{}/v1/pull/peer?max=1", server.base_url());
    let resp: InboxPullResp = HttpClient::new().get(url).send().unwrap().json().unwrap();
    assert_eq!(resp.items.len(), 1, "expected 1 inbox item");

    let env_bytes = &resp.items[0].data;
    let env = Envelope::decode(env_bytes).expect("decode envelope");
    assert_eq!(env.env_version, QSE_ENV_VERSION_V1);
    assert!(env.payload.len() > plaintext.len());

    let st = qsp_session_for_channel(1, "peer");
    let out = recv_wire_canon(
        &StdCrypto,
        &StdCrypto,
        &StdCrypto,
        st.recv,
        &env.payload,
        None,
        None,
    )
    .expect("recv_wire");
    assert_eq!(out.plaintext, plaintext);
}

#[test]
fn tamper_rejects_no_write() {
    let server = common::start_inbox_server(1024 * 1024, 32);
    let base = safe_test_root().join(format!("na0092_tamper_{}", std::process::id()));
    ensure_dir_700(&base);
    let cfg = base.join("cfg");
    ensure_dir_700(&cfg);
    let out_dir = base.join("out");
    ensure_dir_700(&out_dir);

    let plaintext = b"tamper-test".to_vec();
    let env_bytes = qsp_pack_for_channel(1, "peer", &plaintext);
    let mut env = Envelope::decode(&env_bytes).expect("decode env");
    if let Some(b) = env.payload.first_mut() {
        *b ^= 0x01;
    }
    let env_bytes = env.encode();

    let url = format!("{}/v1/push/peer", server.base_url());
    let resp = HttpClient::new().post(url).body(env_bytes).send().unwrap();
    assert!(resp.status().is_success());

    let output = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", &cfg)
        .env("QSC_QSP_SEED", "1")
        .env("QSC_ALLOW_SEED_FALLBACK", "1")
        .env("QSC_MARK_FORMAT", "plain")
        .args([
            "receive",
            "--transport",
            "relay",
            "--relay",
            server.base_url(),
            "--from",
            "peer",
            "--max",
            "1",
            "--out",
            out_dir.to_str().unwrap(),
        ])
        .output()
        .expect("run receive");

    assert!(!output.status.success(), "receive should fail on tamper");
    let combined = String::from_utf8_lossy(&output.stdout).to_string()
        + &String::from_utf8_lossy(&output.stderr);
    assert!(combined.contains("qsp_env_decode_failed") || combined.contains("qsp_verify_failed"));
    assert!(!out_dir.join("recv_1.bin").exists());
}

#[test]
fn status_truthy_active_inactive() {
    let base = safe_test_root().join(format!("na0092_status_{}", std::process::id()));
    let _ = fs::remove_dir_all(&base);
    ensure_dir_700(&base);
    let cfg = base.join("cfg");
    ensure_dir_700(&cfg);

    let output = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", &cfg)
        .env("QSC_MARK_FORMAT", "plain")
        .args(["status"])
        .output()
        .expect("status");
    let combined = String::from_utf8_lossy(&output.stdout).to_string()
        + &String::from_utf8_lossy(&output.stderr);
    assert!(combined.contains("event=qsp_status"));
    assert!(combined.contains("INACTIVE"));

    let server = common::start_inbox_server(1024 * 1024, 32);
    let payload = base.join("msg.bin");
    fs::write(&payload, b"hello").expect("write payload");

    let output_send = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", &cfg)
        .env("QSC_QSP_SEED", "1")
        .env("QSC_ALLOW_SEED_FALLBACK", "1")
        .env("QSC_MARK_FORMAT", "plain")
        .args([
            "send",
            "--transport",
            "relay",
            "--relay",
            server.base_url(),
            "--to",
            "peer",
            "--file",
            payload.to_str().unwrap(),
        ])
        .output()
        .expect("send");
    assert!(output_send.status.success());

    let output2 = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", &cfg)
        .env("QSC_MARK_FORMAT", "plain")
        .args(["status"])
        .output()
        .expect("status after");
    let combined2 = String::from_utf8_lossy(&output2.stdout).to_string()
        + &String::from_utf8_lossy(&output2.stderr);
    assert!(combined2.contains("event=qsp_status"));
    assert!(combined2.contains("ACTIVE"));
}
