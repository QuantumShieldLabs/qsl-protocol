mod common;

use argon2::{Algorithm, Argon2, Params, Version};
use chacha20poly1305::aead::{Aead, Payload};
use chacha20poly1305::KeyInit;
use chacha20poly1305::{ChaCha20Poly1305, Key, Nonce};
use quantumshield_refimpl::qse::Envelope;
use quantumshield_refimpl::suite2::state::Suite2SessionState;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const ROUTE_TOKEN_PEER: &str = "route_token_peer_na0302_abcdefghijkl";
const QSP_SESSION_STORE_KEY_SECRET: &str = "qsp_session_store_key_v1";
const QSP_SESSION_BLOB_MAGIC: &[u8; 6] = b"QSSV01";
const QSP_SESSION_BLOB_VERSION: u8 = 1;
const PLAINTEXT_SENTINEL: &str = "NA0302_QSC_PLAINTEXT_SENTINEL_DO_NOT_ECHO";
const MALFORMED_SENTINEL: &[u8] = b"NA0302_QSC_NEGOTIATION_MALFORMED_SENTINEL_DO_NOT_ECHO";

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
        .env("QSC_UNSAFE_TEST_SEED_FALLBACK", "1")
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
        .hash_password_into(
            common::TEST_MOCK_VAULT_PASSPHRASE.as_bytes(),
            &salt,
            &mut key,
        )
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

fn assert_no_leak_or_panic(output: &str) {
    assert!(
        !output.contains(PLAINTEXT_SENTINEL),
        "plaintext sentinel leaked"
    );
    assert!(
        !output.contains("NA0302_QSC_NEGOTIATION_MALFORMED"),
        "malformed sentinel leaked"
    );
    assert!(!output.contains(ROUTE_TOKEN_PEER), "route token leaked");
    assert!(!output.contains("panicked"), "panic text leaked");
    assert!(!output.contains("stack backtrace"), "backtrace text leaked");
}

fn mutate_payload(mut envelope: Envelope, label: &str) -> Vec<u8> {
    match label {
        "unsupported-suite" => {
            envelope.payload[2..4].copy_from_slice(&0x9999u16.to_be_bytes());
        }
        "downgrade-version" => {
            envelope.payload[0..2].copy_from_slice(&0x0403u16.to_be_bytes());
            envelope.payload[2..4].copy_from_slice(&0x0001u16.to_be_bytes());
        }
        "unsupported-flags" => {
            let flags_offset = 10 + 32;
            envelope.payload[flags_offset..flags_offset + 2]
                .copy_from_slice(&0x8000u16.to_be_bytes());
        }
        "malformed" => {
            envelope.flags = 0;
            envelope.padding.clear();
            envelope.payload = MALFORMED_SENTINEL.to_vec();
        }
        other => panic!("unknown mutation label {other}"),
    }
    envelope.encode()
}

#[test]
fn qsc_cross_surface_rejects_suite2_negotiation_mutations_without_session_mutation() {
    let server = common::start_inbox_server(1024 * 1024, 32);
    let base = safe_test_root().join(format!("na0302_qsc_cross_{}", std::process::id()));
    create_dir_700(&base);
    let cfg = base.join("cfg");
    create_dir_700(&cfg);
    common::init_mock_vault(&cfg);
    contacts_route_set(&cfg, "peer", ROUTE_TOKEN_PEER);

    let payload = base.join("msg.bin");
    fs::write(&payload, PLAINTEXT_SENTINEL.as_bytes()).expect("write payload");

    let send = qsc_base(&cfg)
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
        .expect("qsc send");
    assert!(send.status.success(), "{}", combined_output(&send));
    let send_text = combined_output(&send);
    assert!(send_text.contains("event=qsp_pack ok=true"), "{send_text}");
    assert!(send_text.contains("event=send_commit"), "{send_text}");

    let accepted_state = load_session_state(&cfg, "peer");
    let accepted_snapshot = accepted_state.snapshot_bytes();
    let queued = server.drain_channel(ROUTE_TOKEN_PEER);
    assert_eq!(queued.len(), 1, "expected one queued Suite-2 envelope");
    let valid_envelope = Envelope::decode(&queued[0]).expect("decode qse envelope");

    for label in [
        "unsupported-suite",
        "downgrade-version",
        "unsupported-flags",
        "malformed",
    ] {
        let out_dir = base.join(format!("out-{label}"));
        create_dir_700(&out_dir);
        server.replace_channel(
            ROUTE_TOKEN_PEER,
            vec![mutate_payload(valid_envelope.clone(), label)],
        );

        let recv = qsc_base(&cfg)
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
                out_dir.to_str().unwrap(),
            ])
            .output()
            .expect("qsc receive");
        let recv_text = combined_output(&recv);
        assert!(
            !recv.status.success(),
            "{label} unexpectedly accepted: {recv_text}"
        );
        assert!(
            recv_text.contains("event=qsp_unpack code=qsp_verify_failed ok=false"),
            "{label} missing qsp fail-closed marker: {recv_text}"
        );
        assert!(
            !recv_text.contains("event=qsp_unpack ok=true"),
            "{recv_text}"
        );
        assert!(!recv_text.contains("event=recv_commit"), "{recv_text}");
        assert!(
            !out_dir.join("recv_1.bin").exists(),
            "{label} wrote plaintext output on reject"
        );
        assert_no_leak_or_panic(&recv_text);
        assert_eq!(
            accepted_snapshot,
            load_session_state(&cfg, "peer").snapshot_bytes(),
            "{label} mutated persisted qsc Suite-2 session state"
        );
    }

    println!("NA0302_QSC_CROSS_SURFACE_OK");
}
