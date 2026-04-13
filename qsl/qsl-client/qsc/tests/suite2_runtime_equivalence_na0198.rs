mod common;

use argon2::{Algorithm, Argon2, Params, Version};
use chacha20poly1305::aead::{Aead, Payload};
use chacha20poly1305::KeyInit;
use chacha20poly1305::{ChaCha20Poly1305, Key, Nonce};
use quantumshield_refimpl::crypto::stdcrypto::StdCrypto;
use quantumshield_refimpl::crypto::traits::{Hash, Kmac};
use quantumshield_refimpl::qse::Envelope;
use quantumshield_refimpl::suite2::ratchet::{Suite2RecvWireState, Suite2SendState};
use quantumshield_refimpl::suite2::state::Suite2SessionState;
use quantumshield_refimpl::suite2::types::{SUITE2_PROTOCOL_VERSION, SUITE2_SUITE_ID};
use quantumshield_refimpl::suite2::{recv_wire_canon, send_wire_canon};
use std::collections::BTreeSet;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const ROUTE_TOKEN_PEER: &str = "route_token_peer_na0198_abcdefghijkl";
const QSP_SESSION_STORE_KEY_SECRET: &str = "qsp_session_store_key_v1";
const QSP_SESSION_BLOB_MAGIC: &[u8; 6] = b"QSSV01";
const QSP_SESSION_BLOB_VERSION: u8 = 1;

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

fn create_dir_700(path: &Path) {
    let _ = fs::remove_dir_all(path);
    ensure_dir_700(path);
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

fn combined_output(output: &std::process::Output) -> String {
    let mut combined = String::from_utf8_lossy(&output.stdout).to_string();
    combined.push_str(&String::from_utf8_lossy(&output.stderr));
    combined
}

fn qsc_base(cfg: &Path) -> Command {
    let mut cmd = common::qsc_std_command();
    cmd.env("QSC_CONFIG_DIR", cfg)
        .env("QSC_QSP_SEED", "1")
        .env("QSC_ALLOW_SEED_FALLBACK", "1")
        .env("QSC_MARK_FORMAT", "plain");
    cmd
}

fn contacts_route_set(cfg: &Path, label: &str, token: &str) {
    let out = qsc_base(cfg)
        .args([
            "contacts",
            "add",
            "--label",
            label,
            "--fp",
            "fp-pinned-test",
            "--route-token",
            token,
        ])
        .output()
        .expect("contacts add");
    assert!(out.status.success(), "{}", combined_output(&out));
}

fn read_mock_vault_secret(cfg: &Path, name: &str) -> String {
    let vault_path = cfg.join("vault.qsv");
    let bytes = fs::read(&vault_path).expect("vault read");
    assert!(bytes.len() > 39, "vault envelope too short");
    assert_eq!(&bytes[0..6], b"QSCV01");
    assert_eq!(bytes[6], 1, "expected passphrase vault");
    let salt_len = bytes[7] as usize;
    let nonce_len = bytes[8] as usize;
    assert_eq!(salt_len, 16);
    assert_eq!(nonce_len, 12);
    let kdf_m_kib = u32::from_le_bytes([bytes[9], bytes[10], bytes[11], bytes[12]]);
    let kdf_t = u32::from_le_bytes([bytes[13], bytes[14], bytes[15], bytes[16]]);
    let kdf_p = u32::from_le_bytes([bytes[17], bytes[18], bytes[19], bytes[20]]);
    let ct_len = u32::from_le_bytes([bytes[21], bytes[22], bytes[23], bytes[24]]) as usize;
    let mut salt = [0u8; 16];
    salt.copy_from_slice(&bytes[25..25 + salt_len]);
    let mut off = 25 + salt_len;
    let nonce = &bytes[off..off + nonce_len];
    off += nonce_len;
    let ciphertext = &bytes[off..off + ct_len];
    let params = Params::new(kdf_m_kib, kdf_t, kdf_p, Some(32)).expect("argon2 params");
    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);
    let mut key = [0u8; 32];
    argon2
        .hash_password_into(common::TEST_MOCK_VAULT_PASSPHRASE.as_bytes(), &salt, &mut key)
        .expect("vault key");
    let cipher = ChaCha20Poly1305::new(Key::from_slice(&key));
    let plaintext = cipher
        .decrypt(Nonce::from_slice(nonce), ciphertext)
        .expect("vault decrypt");
    let root: serde_json::Value = serde_json::from_slice(&plaintext).expect("vault json");
    root.get("secrets")
        .and_then(|v| v.get(name))
        .and_then(|v| v.as_str())
        .expect("secret missing")
        .to_string()
}

fn hex_nibble(c: u8) -> Option<u8> {
    match c {
        b'0'..=b'9' => Some(c - b'0'),
        b'a'..=b'f' => Some(c - b'a' + 10),
        b'A'..=b'F' => Some(c - b'A' + 10),
        _ => None,
    }
}

fn decode_hex(s: &str) -> Vec<u8> {
    assert!(s.len().is_multiple_of(2), "hex length");
    let bytes = s.as_bytes();
    let mut out = Vec::with_capacity(s.len() / 2);
    let mut i = 0usize;
    while i < bytes.len() {
        let hi = hex_nibble(bytes[i]).expect("hex hi");
        let lo = hex_nibble(bytes[i + 1]).expect("hex lo");
        out.push((hi << 4) | lo);
        i += 2;
    }
    out
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

fn load_session_state(cfg: &Path, peer: &str) -> Suite2SessionState {
    let blob_path = cfg.join("qsp_sessions").join(format!("{peer}.qsv"));
    let blob = fs::read(&blob_path).expect("session blob read");
    assert!(blob.len() >= 24, "session blob too short");
    assert_eq!(&blob[..6], QSP_SESSION_BLOB_MAGIC);
    assert_eq!(blob[6], QSP_SESSION_BLOB_VERSION);
    assert_eq!(blob[7], 12);
    let ct_len = u32::from_le_bytes([blob[8], blob[9], blob[10], blob[11]]) as usize;
    let nonce = &blob[12..24];
    let ciphertext = &blob[24..24 + ct_len];
    let store_key_hex = read_mock_vault_secret(cfg, QSP_SESSION_STORE_KEY_SECRET);
    let store_key = decode_hex(&store_key_hex);
    let cipher = ChaCha20Poly1305::new(Key::from_slice(&store_key));
    let aad = format!("QSC.QSP.SESSION.V{}:{}", QSP_SESSION_BLOB_VERSION, peer).into_bytes();
    let plaintext = cipher
        .decrypt(
            Nonce::from_slice(nonce),
            Payload {
                msg: ciphertext,
                aad: &aad,
            },
        )
        .expect("session decrypt");
    Suite2SessionState::restore_bytes(&plaintext).expect("session restore")
}

#[test]
fn seeded_runtime_state_matches_refimpl_send_receive_roundtrip() {
    let server = common::start_inbox_server(1024 * 1024, 32);
    let base = safe_test_root().join(format!("na0198_suite2_eq_{}", std::process::id()));
    create_dir_700(&base);

    let alice_cfg = base.join("alice_cfg");
    let bob_cfg = base.join("bob_cfg");
    let alice_out = base.join("alice_out");
    let bob_out = base.join("bob_out");
    create_dir_700(&alice_cfg);
    create_dir_700(&bob_cfg);
    create_dir_700(&alice_out);
    create_dir_700(&bob_out);
    common::init_mock_vault(&alice_cfg);
    common::init_mock_vault(&bob_cfg);
    contacts_route_set(&alice_cfg, "peer", ROUTE_TOKEN_PEER);
    contacts_route_set(&bob_cfg, "peer", ROUTE_TOKEN_PEER);

    let msg_a = b"hello-suite2-runtime-a".to_vec();
    let msg_b = b"hello-suite2-runtime-b".to_vec();
    let msg_a_path = base.join("msg_a.bin");
    let msg_b_path = base.join("msg_b.bin");
    fs::write(&msg_a_path, &msg_a).expect("write msg a");
    fs::write(&msg_b_path, &msg_b).expect("write msg b");

    let send_a = qsc_base(&alice_cfg)
        .args([
            "send",
            "--transport",
            "relay",
            "--relay",
            server.base_url(),
            "--to",
            "peer",
            "--file",
            msg_a_path.to_str().unwrap(),
        ])
        .output()
        .expect("send a");
    assert!(send_a.status.success(), "{}", combined_output(&send_a));

    let initial = seeded_session_state(1, "peer");
    let send_a_expected =
        send_wire_canon(&StdCrypto, &StdCrypto, &StdCrypto, initial.send, 0, &msg_a)
            .expect("refimpl send a");
    let expected_alice_after_send = Suite2SessionState {
        send: send_a_expected.state,
        recv: seeded_session_state(1, "peer").recv,
    };
    let alice_after_send = load_session_state(&alice_cfg, "peer");
    assert_eq!(
        alice_after_send.snapshot_bytes(),
        expected_alice_after_send.snapshot_bytes(),
        "alice persisted send state drifted from refimpl"
    );

    let queued_a = server.drain_channel(ROUTE_TOKEN_PEER);
    assert_eq!(
        queued_a.len(),
        1,
        "expected exactly one queued message from alice"
    );
    let env_a = Envelope::decode(&queued_a[0]).expect("decode alice envelope");
    assert_eq!(
        env_a.payload, send_a_expected.wire,
        "alice emitted non-canonical Suite-2 wire"
    );
    server.replace_channel(ROUTE_TOKEN_PEER, queued_a);

    let recv_b = qsc_base(&bob_cfg)
        .args([
            "receive",
            "--transport",
            "relay",
            "--relay",
            server.base_url(),
            "--mailbox",
            ROUTE_TOKEN_PEER,
            "--from",
            "peer",
            "--max",
            "1",
            "--out",
            bob_out.to_str().unwrap(),
        ])
        .output()
        .expect("receive b");
    assert!(recv_b.status.success(), "{}", combined_output(&recv_b));
    assert_eq!(
        fs::read(bob_out.join("recv_1.bin")).expect("bob file"),
        msg_a
    );

    let recv_b_expected = recv_wire_canon(
        &StdCrypto,
        &StdCrypto,
        &StdCrypto,
        seeded_session_state(1, "peer").recv,
        &env_a.payload,
        None,
        None,
    )
    .expect("refimpl recv b");
    assert_eq!(recv_b_expected.plaintext, msg_a);
    let expected_bob_after_recv = Suite2SessionState {
        send: seeded_session_state(1, "peer").send,
        recv: recv_b_expected.state,
    };
    let bob_after_recv = load_session_state(&bob_cfg, "peer");
    assert_eq!(
        bob_after_recv.snapshot_bytes(),
        expected_bob_after_recv.snapshot_bytes(),
        "bob persisted recv state drifted from refimpl"
    );

    let send_b = qsc_base(&bob_cfg)
        .args([
            "send",
            "--transport",
            "relay",
            "--relay",
            server.base_url(),
            "--to",
            "peer",
            "--file",
            msg_b_path.to_str().unwrap(),
        ])
        .output()
        .expect("send b");
    assert!(send_b.status.success(), "{}", combined_output(&send_b));

    let send_b_expected = send_wire_canon(
        &StdCrypto,
        &StdCrypto,
        &StdCrypto,
        expected_bob_after_recv.send,
        0,
        &msg_b,
    )
    .expect("refimpl send b");
    let expected_bob_after_send = Suite2SessionState {
        send: send_b_expected.state,
        recv: expected_bob_after_recv.recv,
    };
    let bob_after_send = load_session_state(&bob_cfg, "peer");
    assert_eq!(
        bob_after_send.snapshot_bytes(),
        expected_bob_after_send.snapshot_bytes(),
        "bob persisted send state drifted from refimpl"
    );

    let queued_b = server.drain_channel(ROUTE_TOKEN_PEER);
    assert_eq!(
        queued_b.len(),
        1,
        "expected exactly one queued message from bob"
    );
    let env_b = Envelope::decode(&queued_b[0]).expect("decode bob envelope");
    assert_eq!(
        env_b.payload, send_b_expected.wire,
        "bob emitted non-canonical Suite-2 wire"
    );
    server.replace_channel(ROUTE_TOKEN_PEER, queued_b);

    let recv_a = qsc_base(&alice_cfg)
        .args([
            "receive",
            "--transport",
            "relay",
            "--relay",
            server.base_url(),
            "--mailbox",
            ROUTE_TOKEN_PEER,
            "--from",
            "peer",
            "--max",
            "1",
            "--out",
            alice_out.to_str().unwrap(),
        ])
        .output()
        .expect("receive a");
    assert!(recv_a.status.success(), "{}", combined_output(&recv_a));
    assert_eq!(
        fs::read(alice_out.join("recv_1.bin")).expect("alice file"),
        msg_b
    );

    let recv_a_expected = recv_wire_canon(
        &StdCrypto,
        &StdCrypto,
        &StdCrypto,
        expected_alice_after_send.recv,
        &env_b.payload,
        None,
        None,
    )
    .expect("refimpl recv a");
    assert_eq!(recv_a_expected.plaintext, msg_b);
    let expected_alice_after_recv = Suite2SessionState {
        send: expected_alice_after_send.send,
        recv: recv_a_expected.state,
    };
    let alice_after_recv = load_session_state(&alice_cfg, "peer");
    assert_eq!(
        alice_after_recv.snapshot_bytes(),
        expected_alice_after_recv.snapshot_bytes(),
        "alice persisted recv state drifted from refimpl"
    );
}
