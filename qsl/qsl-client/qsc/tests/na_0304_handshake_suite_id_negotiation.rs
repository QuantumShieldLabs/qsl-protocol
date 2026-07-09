mod common;

use argon2::{Algorithm, Argon2, Params, Version};
use chacha20poly1305::aead::{Aead, Payload};
use chacha20poly1305::KeyInit;
use chacha20poly1305::{ChaCha20Poly1305, Key, Nonce};
use quantumshield_refimpl::suite2::state::Suite2SessionState;
use quantumshield_refimpl::suite2::types::{SUITE2_PROTOCOL_VERSION, SUITE2_SUITE_ID};
use std::fs;
use std::path::{Path, PathBuf};

const ROUTE_TOKEN_ALICE: &str = "route_token_alice_na0304_abcdefghijkl";
const ROUTE_TOKEN_BOB: &str = "route_token_bob_na0304_abcdefghijklmn";
const QSP_SESSION_STORE_KEY_SECRET: &str = "qsp_session_store_key_v1";
const QSP_SESSION_BLOB_MAGIC: &[u8; 6] = b"QSSV01";
const QSP_SESSION_BLOB_VERSION: u8 = 1;

fn safe_test_root() -> PathBuf {
    let root = if let Ok(v) = std::env::var("QSC_TEST_ROOT") {
        PathBuf::from(v)
    } else if let Ok(v) = std::env::var("CARGO_TARGET_DIR") {
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
    let mut cmd = common::qsc_std_command();
    cmd.env("QSC_CONFIG_DIR", cfg)
        .env("QSC_MARK_FORMAT", "plain")
        .args(args)
        .output()
        .expect("qsc command")
}

fn init_identity(cfg: &Path, label: &str) {
    let out = run_qsc(cfg, &["identity", "rotate", "--as", label, "--confirm"]);
    assert!(out.status.success(), "{}", output_text(&out));
}

fn identity_fp(cfg: &Path, label: &str) -> String {
    let out = run_qsc(cfg, &["identity", "show", "--as", label]);
    assert!(out.status.success(), "{}", output_text(&out));
    output_text(&out)
        .lines()
        .find_map(|line| line.strip_prefix("identity_fp="))
        .map(ToOwned::to_owned)
        .unwrap_or_else(|| panic!("missing identity_fp in output: {}", output_text(&out)))
}

fn contacts_add_authenticated_with_route(cfg: &Path, label: &str, fp: &str, token: &str) {
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
}

fn relay_inbox_set(cfg: &Path, token: &str) {
    let out = run_qsc(cfg, &["relay", "inbox-set", "--token", token]);
    assert!(out.status.success(), "{}", output_text(&out));
}

fn seed_authenticated_pair(alice_cfg: &Path, bob_cfg: &Path) {
    init_identity(alice_cfg, "alice");
    init_identity(bob_cfg, "bob");
    let alice_fp = identity_fp(alice_cfg, "alice");
    let bob_fp = identity_fp(bob_cfg, "bob");
    contacts_add_authenticated_with_route(alice_cfg, "bob", bob_fp.as_str(), ROUTE_TOKEN_BOB);
    contacts_add_authenticated_with_route(bob_cfg, "alice", alice_fp.as_str(), ROUTE_TOKEN_ALICE);
    relay_inbox_set(alice_cfg, ROUTE_TOKEN_ALICE);
    relay_inbox_set(bob_cfg, ROUTE_TOKEN_BOB);
}

fn session_path(cfg: &Path, peer: &str) -> PathBuf {
    cfg.join("qsp_sessions").join(format!("{peer}.qsv"))
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
    let blob = fs::read(session_path(cfg, peer)).expect("session blob read");
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
    // NA-0622: strip the qsc session-blob v2 DH-ratchet trigger prefix (b"QTRG" + 13 bytes).
    // NA-0624: a v3 plaintext additionally carries scka_len(u32 LE) + SCKA section between the
    // trigger and the QS2S snapshot (scka_len == 0 for a non-advertising session).
    let snapshot: &[u8] = if plaintext.len() >= 17 && &plaintext[..4] == b"QTRG" {
        let rest = &plaintext[17..];
        if rest.starts_with(b"QS2S") {
            rest
        } else {
            let scka_len = u32::from_le_bytes([rest[0], rest[1], rest[2], rest[3]]) as usize;
            &rest[4 + scka_len..]
        }
    } else {
        &plaintext
    };
    Suite2SessionState::restore_bytes(snapshot).expect("session restore")
}

fn marker_value<'a>(text: &'a str, event: &str, msg: &str, key: &str) -> &'a str {
    for line in text.lines() {
        if !line.contains(&format!("event={event}")) || !line.contains(&format!("msg={msg}")) {
            continue;
        }
        for part in line.split_whitespace() {
            if let Some(v) = part.strip_prefix(&format!("{key}=")) {
                return v;
            }
        }
    }
    panic!("missing marker value event={event} msg={msg} key={key}: {text}");
}

fn marker_usize(text: &str, event: &str, msg: &str, key: &str) -> usize {
    marker_value(text, event, msg, key)
        .parse::<usize>()
        .unwrap_or_else(|_| panic!("non-numeric marker value key={key}: {text}"))
}

fn assert_qhsm_header(frame: &[u8], frame_type: u8) {
    assert!(frame.len() >= 7, "QHSM frame too short");
    assert_eq!(&frame[0..4], b"QHSM");
    assert_eq!(u16::from_be_bytes([frame[4], frame[5]]), 1);
    assert_eq!(frame[6], frame_type);
}

fn assert_init_frame_has_no_explicit_suite_slot(frame: &[u8], text: &str) {
    let size = marker_usize(text, "handshake_send", "A1", "size");
    let kem_pk_len = marker_usize(text, "handshake_send", "A1", "kem_pk_len");
    let sig_pk_len = marker_usize(text, "handshake_send", "A1", "sig_pk_len");
    assert_eq!(frame.len(), size);
    assert_qhsm_header(frame, 1);
    assert_eq!(frame.len(), 4 + 2 + 1 + 16 + kem_pk_len + sig_pk_len + 32);
}

fn assert_resp_frame_has_no_explicit_suite_slot(frame: &[u8], text: &str) {
    let size = marker_usize(text, "handshake_send", "B1", "size");
    let kem_ct_len = marker_usize(text, "handshake_send", "B1", "kem_ct_len");
    let sig_pk_len = marker_usize(text, "handshake_send", "B1", "sig_pk_len");
    let fixed_without_sig = 4 + 2 + 1 + 16 + kem_ct_len + 32 + sig_pk_len + 32;
    assert_eq!(frame.len(), size);
    assert_qhsm_header(frame, 2);
    assert!(
        frame.len() > fixed_without_sig,
        "B1 frame lacks signature bytes"
    );
}

fn assert_confirm_frame_has_no_explicit_suite_slot(frame: &[u8], text: &str) {
    let size = marker_usize(text, "handshake_send", "A2", "size");
    let fixed_without_sig = 4 + 2 + 1 + 16 + 32;
    assert_eq!(frame.len(), size);
    assert_qhsm_header(frame, 3);
    assert!(
        frame.len() > fixed_without_sig,
        "A2 frame lacks signature bytes"
    );
}

fn assert_session_is_suite2(st: &Suite2SessionState, label: &str) {
    assert_eq!(
        st.send.protocol_version, SUITE2_PROTOCOL_VERSION,
        "{label} send protocol version drifted"
    );
    assert_eq!(
        st.recv.protocol_version, SUITE2_PROTOCOL_VERSION,
        "{label} recv protocol version drifted"
    );
    assert_eq!(st.send.suite_id, SUITE2_SUITE_ID, "{label} send suite id");
    assert_eq!(st.recv.suite_id, SUITE2_SUITE_ID, "{label} recv suite id");
}

fn assert_no_leak_or_panic(text: &str) {
    for forbidden in [
        ROUTE_TOKEN_ALICE,
        ROUTE_TOKEN_BOB,
        "panicked",
        "stack backtrace",
        "thread '",
        "QSC_DESKTOP_SESSION_PASSPHRASE",
    ] {
        assert!(
            !text.contains(forbidden),
            "forbidden output fragment leaked: {forbidden}: {text}"
        );
    }
}

fn poll_bob(bob_cfg: &Path, relay: &str) -> std::process::Output {
    run_qsc(
        bob_cfg,
        &[
            "handshake",
            "poll",
            "--as",
            "bob",
            "--peer",
            "alice",
            "--relay",
            relay,
            "--max",
            "4",
        ],
    )
}

fn poll_alice(alice_cfg: &Path, relay: &str) -> std::process::Output {
    run_qsc(
        alice_cfg,
        &[
            "handshake",
            "poll",
            "--as",
            "alice",
            "--peer",
            "bob",
            "--relay",
            relay,
            "--max",
            "4",
        ],
    )
}

fn init_alice(alice_cfg: &Path, relay: &str) -> std::process::Output {
    run_qsc(
        alice_cfg,
        &[
            "handshake",
            "init",
            "--as",
            "alice",
            "--peer",
            "bob",
            "--relay",
            relay,
        ],
    )
}

#[test]
fn qsc_handshake_suite_id_seam_is_blocked_without_runtime_change() {
    let server = common::start_inbox_server(1024 * 1024, 32);
    let base = safe_test_root().join(format!("na0304_hs_suite_id_{}", std::process::id()));
    create_dir_700(&base);
    let alice_cfg = base.join("alice");
    let bob_cfg = base.join("bob");
    create_dir_700(&alice_cfg);
    create_dir_700(&bob_cfg);
    common::init_mock_vault(&alice_cfg);
    common::init_mock_vault(&bob_cfg);
    seed_authenticated_pair(&alice_cfg, &bob_cfg);

    let relay = server.base_url().to_string();

    let alice_init = init_alice(&alice_cfg, &relay);
    let alice_init_text = output_text(&alice_init);
    assert!(alice_init.status.success(), "{alice_init_text}");
    let a1_items = server.drain_channel(ROUTE_TOKEN_BOB);
    assert_eq!(a1_items.len(), 1);
    assert_init_frame_has_no_explicit_suite_slot(&a1_items[0], &alice_init_text);

    server.replace_channel(ROUTE_TOKEN_BOB, vec![a1_items[0].clone()]);
    let bob_poll = poll_bob(&bob_cfg, &relay);
    let bob_poll_text = output_text(&bob_poll);
    assert!(bob_poll.status.success(), "{bob_poll_text}");
    assert!(!session_path(&bob_cfg, "alice").exists());
    let b1_items = server.drain_channel(ROUTE_TOKEN_ALICE);
    assert_eq!(b1_items.len(), 1);
    assert_resp_frame_has_no_explicit_suite_slot(&b1_items[0], &bob_poll_text);

    server.replace_channel(ROUTE_TOKEN_ALICE, vec![b1_items[0].clone()]);
    let alice_poll = poll_alice(&alice_cfg, &relay);
    let alice_poll_text = output_text(&alice_poll);
    assert!(alice_poll.status.success(), "{alice_poll_text}");
    assert!(session_path(&alice_cfg, "bob").exists());
    let a2_items = server.drain_channel(ROUTE_TOKEN_BOB);
    assert_eq!(a2_items.len(), 1);
    assert_confirm_frame_has_no_explicit_suite_slot(&a2_items[0], &alice_poll_text);

    server.replace_channel(ROUTE_TOKEN_BOB, vec![a2_items[0].clone()]);
    let bob_confirm = poll_bob(&bob_cfg, &relay);
    let bob_confirm_text = output_text(&bob_confirm);
    assert!(bob_confirm.status.success(), "{bob_confirm_text}");
    assert!(session_path(&bob_cfg, "alice").exists());

    let alice_session = load_session_state(&alice_cfg, "bob");
    let bob_session = load_session_state(&bob_cfg, "alice");
    assert_session_is_suite2(&alice_session, "alice");
    assert_session_is_suite2(&bob_session, "bob");

    let combined = [
        alice_init_text,
        bob_poll_text,
        alice_poll_text,
        bob_confirm_text,
    ]
    .join("\n");
    assert!(combined.contains("event=handshake_complete peer=bob role=initiator"));
    assert!(combined.contains("event=handshake_complete peer=alice role=responder"));
    assert!(!combined.contains("event=recv_commit"));
    assert_no_leak_or_panic(&combined);

    println!("NA0304_QSC_HANDSHAKE_SESSION_SUITE2_STATE_OK");
    println!("NA0304_QSC_QHSM_NO_EXPLICIT_SUITE_ID_FIELD_OK");
    println!("NA0304_QSC_SUITE_ID_SEAM_BLOCKED");
    println!("NA0304_NO_IMPLEMENTATION_CHANGE_OK");
    println!("NA0304_BLOCKER_EVIDENCE_OK");
}
