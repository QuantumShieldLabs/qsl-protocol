mod common;

use argon2::{Algorithm, Argon2, Params, Version};
use chacha20poly1305::aead::{Aead, Payload};
use chacha20poly1305::KeyInit;
use chacha20poly1305::{ChaCha20Poly1305, Key, Nonce};
use quantumshield_refimpl::suite2::state::Suite2SessionState;
use quantumshield_refimpl::suite2::types::{SUITE2_PROTOCOL_VERSION, SUITE2_SUITE_ID};
use serde_json::Value;
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

const ROUTE_TOKEN_ALICE: &str = "route_token_alice_na0313_abcdefghijkl";
const ROUTE_TOKEN_BOB: &str = "route_token_bob_na0313_abcdefghijklmn";
const QSP_SESSION_STORE_KEY_SECRET: &str = "qsp_session_store_key_v1";
const QSP_SESSION_BLOB_MAGIC: &[u8; 6] = b"QSSV01";
const QSP_SESSION_BLOB_VERSION: u8 = 1;
const SUITE_PARAM_ID: u16 = 0x0001;
const UNKNOWN_PARAM_ID: u16 = 0x0002;
const SECRET_SENTINEL: &[u8] = b"NA0313_SENTINEL_NO_ECHO";
const REQUIRED_CATEGORIES: &[&str] = &[
    "valid_v2_suite2_parameter_block",
    "legacy_v1_compatibility_allowed",
    "legacy_v1_rejected_in_suite_required_mode",
    "unsupported_suite_id",
    "downgraded_suite_id",
    "stripped_suite_id_parameter",
    "mismatched_suite_id_A1_B1",
    "mismatched_suite_id_B1_A2",
    "duplicate_suite_id_parameter",
    "unknown_critical_parameter",
    "unknown_noncritical_parameter",
    "noncanonical_parameter_order",
    "malformed_parameter_length",
    "inconsistent_protocol_version_suite_id",
    "replayed_A1_with_suite_context",
    "replayed_A2_with_suite_context",
    "valid_suite2_with_transcript_binding",
    "transcript_binding_mismatch",
    "key_schedule_context_mismatch",
    "missing_key_context_in_required_mode",
];

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

fn new_pair(root: &Path, tag: &str) -> (PathBuf, PathBuf) {
    let alice_cfg = root.join(format!("{tag}-alice"));
    let bob_cfg = root.join(format!("{tag}-bob"));
    create_dir_700(&alice_cfg);
    create_dir_700(&bob_cfg);
    common::init_mock_vault(&alice_cfg);
    common::init_mock_vault(&bob_cfg);
    seed_authenticated_pair(&alice_cfg, &bob_cfg);
    (alice_cfg, bob_cfg)
}

fn session_path(cfg: &Path, peer: &str) -> PathBuf {
    cfg.join("qsp_sessions").join(format!("{peer}.qsv"))
}

fn read_mock_vault_json(cfg: &Path) -> Value {
    let bytes = fs::read(cfg.join("vault.qsv")).expect("vault read");
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
    serde_json::from_slice(&plaintext).expect("vault json")
}

fn read_mock_vault_secret(cfg: &Path, name: &str) -> Option<String> {
    read_mock_vault_json(cfg)
        .get("secrets")
        .and_then(|v| v.get(name))
        .and_then(|v| v.as_str())
        .map(ToOwned::to_owned)
}

fn assert_no_pending(cfg: &Path, self_label: &str, peer: &str) {
    let key = format!("handshake.pending.{self_label}.{peer}");
    let value = read_mock_vault_secret(cfg, &key);
    assert!(
        value.as_deref().unwrap_or("").is_empty(),
        "pending state survived reject for {key}"
    );
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
    let store_key_hex =
        read_mock_vault_secret(cfg, QSP_SESSION_STORE_KEY_SECRET).expect("store key");
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
    {
        // NA-0622: strip the qsc session-blob v2 DH-ratchet trigger prefix (b"QTRG" + 13 bytes).
        let snapshot: &[u8] = if plaintext.len() >= 17 && &plaintext[..4] == b"QTRG" {
            &plaintext[17..]
        } else {
            &plaintext
        };
        Suite2SessionState::restore_bytes(snapshot).expect("session restore")
    }
}

fn assert_session_suite2(cfg: &Path, peer: &str) {
    let st = load_session_state(cfg, peer);
    assert_eq!(st.send.protocol_version, SUITE2_PROTOCOL_VERSION);
    assert_eq!(st.recv.protocol_version, SUITE2_PROTOCOL_VERSION);
    assert_eq!(st.send.suite_id, SUITE2_SUITE_ID);
    assert_eq!(st.recv.suite_id, SUITE2_SUITE_ID);
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

fn param(id: u16, critical: bool, value: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(5 + value.len());
    out.extend_from_slice(&id.to_be_bytes());
    out.push(if critical { 0x01 } else { 0x00 });
    out.extend_from_slice(&(value.len() as u16).to_be_bytes());
    out.extend_from_slice(value);
    out
}

fn suite_block(protocol_version: u16, suite_id: u16) -> Vec<u8> {
    let mut value = Vec::with_capacity(4);
    value.extend_from_slice(&protocol_version.to_be_bytes());
    value.extend_from_slice(&suite_id.to_be_bytes());
    param(SUITE_PARAM_ID, true, &value)
}

fn canonical_suite_block() -> Vec<u8> {
    suite_block(0x0500, 0x0002)
}

fn concat_params(params: &[Vec<u8>]) -> Vec<u8> {
    let mut out = Vec::new();
    for p in params {
        out.extend_from_slice(p);
    }
    out
}

fn payload_offset(frame: &[u8]) -> usize {
    assert!(frame.len() >= 7, "QHSM frame too short");
    assert_eq!(&frame[0..4], b"QHSM");
    match u16::from_be_bytes([frame[4], frame[5]]) {
        1 => 7,
        2 => {
            assert!(frame.len() >= 9, "QHSM v2 frame too short");
            let block_len = u16::from_be_bytes([frame[7], frame[8]]) as usize;
            9 + block_len
        }
        other => panic!("unexpected QHSM version {other}"),
    }
}

fn param_block(frame: &[u8]) -> &[u8] {
    assert_eq!(u16::from_be_bytes([frame[4], frame[5]]), 2);
    let len = u16::from_be_bytes([frame[7], frame[8]]) as usize;
    &frame[9..9 + len]
}

fn replace_param_block(frame: &[u8], block: &[u8]) -> Vec<u8> {
    assert_eq!(u16::from_be_bytes([frame[4], frame[5]]), 2);
    assert!(block.len() <= 64);
    let old_payload = payload_offset(frame);
    let mut out = Vec::with_capacity(9 + block.len() + frame.len() - old_payload);
    out.extend_from_slice(&frame[0..7]);
    out.extend_from_slice(&(block.len() as u16).to_be_bytes());
    out.extend_from_slice(block);
    out.extend_from_slice(&frame[old_payload..]);
    out
}

fn set_session_id(frame: &mut [u8], session_id: &[u8; 16]) {
    let off = payload_offset(frame);
    frame[off..off + 16].copy_from_slice(session_id);
}

fn session_id(frame: &[u8]) -> [u8; 16] {
    let off = payload_offset(frame);
    let mut out = [0u8; 16];
    out.copy_from_slice(&frame[off..off + 16]);
    out
}

fn mutate_b1_transcript_field(frame: &[u8], text: &str) -> Vec<u8> {
    let mut out = frame.to_vec();
    let off = payload_offset(frame);
    let kem_ct_len = marker_usize(text, "handshake_send", "B1", "kem_ct_len");
    let sig_pk_len = marker_usize(text, "handshake_send", "B1", "sig_pk_len");
    let sig_len = frame.len() - off - 16 - kem_ct_len - 32 - sig_pk_len - 32;
    let dh_pub_off = off + 16 + kem_ct_len + 32 + sig_pk_len + sig_len;
    out[dh_pub_off] ^= 0x01;
    out
}

fn assert_v2_suite2_frame(frame: &[u8], frame_type: u8) {
    assert_eq!(&frame[0..4], b"QHSM");
    assert_eq!(u16::from_be_bytes([frame[4], frame[5]]), 2);
    assert_eq!(frame[6], frame_type);
    assert_eq!(param_block(frame), canonical_suite_block().as_slice());
}

fn assert_v1_frame(frame: &[u8], frame_type: u8) {
    assert_eq!(&frame[0..4], b"QHSM");
    assert_eq!(u16::from_be_bytes([frame[4], frame[5]]), 1);
    assert_eq!(frame[6], frame_type);
}

fn assert_no_leak_or_panic(text: &str) {
    for forbidden in [
        ROUTE_TOKEN_ALICE,
        ROUTE_TOKEN_BOB,
        common::TEST_MOCK_VAULT_PASSPHRASE,
        "QSC_DESKTOP_SESSION_PASSPHRASE",
        "NA0313_SENTINEL_NO_ECHO",
        "panicked",
        "stack backtrace",
        "thread '",
    ] {
        assert!(
            !text.contains(forbidden),
            "forbidden output fragment leaked: {forbidden}: {text}"
        );
    }
}

fn assert_reject_output(text: &str, reason: &str) {
    assert!(
        text.contains("event=handshake_reject"),
        "missing reject marker: {text}"
    );
    assert!(text.contains(reason), "missing reason {reason}: {text}");
    assert!(
        !text.contains("event=handshake_complete"),
        "reject completed handshake: {text}"
    );
    assert!(
        !text.contains("event=recv_commit"),
        "reject emitted recv_commit: {text}"
    );
    assert!(
        !text.contains("event=qsp_unpack ok=true"),
        "reject emitted qsp output: {text}"
    );
    assert_no_leak_or_panic(text);
}

fn assert_na0310_categories_present() {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .join("inputs/suite2/qsc_handshake_suite_id_vectors_na0310.json");
    let raw = fs::read_to_string(path).expect("read NA-0310 vectors");
    let doc: Value = serde_json::from_str(&raw).expect("parse NA-0310 vectors");
    let categories: BTreeSet<String> = doc
        .get("vectors")
        .and_then(|v| v.as_array())
        .expect("vectors array")
        .iter()
        .map(|v| {
            v.get("category")
                .and_then(|c| c.as_str())
                .expect("category")
                .to_string()
        })
        .collect();
    for required in REQUIRED_CATEGORIES {
        assert!(
            categories.contains(*required),
            "missing NA-0310 vector category {required}"
        );
    }
}

fn poll_bob(bob_cfg: &Path, relay: &str, suite_mode: &str) -> std::process::Output {
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
            "--suite-mode",
            suite_mode,
        ],
    )
}

fn poll_alice(alice_cfg: &Path, relay: &str, suite_mode: &str) -> std::process::Output {
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
            "--suite-mode",
            suite_mode,
        ],
    )
}

fn init_alice(alice_cfg: &Path, relay: &str, suite_mode: &str) -> std::process::Output {
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
            "--suite-mode",
            suite_mode,
        ],
    )
}

#[test]
fn qsc_handshake_suite_id_parameter_block_harness() {
    assert_na0310_categories_present();
    let server = common::start_inbox_server(1024 * 1024, 64);
    let base = safe_test_root().join(format!("na0313_qsc_suite_id_{}", std::process::id()));
    create_dir_700(&base);
    let relay = server.base_url().to_string();

    let (alice_valid, bob_valid) = new_pair(&base, "valid-v2");
    let alice_init = init_alice(&alice_valid, &relay, "suite-required");
    assert!(alice_init.status.success(), "{}", output_text(&alice_init));
    let a1 = server
        .drain_channel(ROUTE_TOKEN_BOB)
        .pop()
        .expect("A1 queued");
    assert_v2_suite2_frame(&a1, 1);
    server.replace_channel(ROUTE_TOKEN_BOB, vec![a1.clone()]);
    let bob_poll = poll_bob(&bob_valid, &relay, "suite-required");
    assert!(bob_poll.status.success(), "{}", output_text(&bob_poll));
    let b1 = server
        .drain_channel(ROUTE_TOKEN_ALICE)
        .pop()
        .expect("B1 queued");
    assert_v2_suite2_frame(&b1, 2);
    server.replace_channel(ROUTE_TOKEN_ALICE, vec![b1.clone()]);
    let alice_poll = poll_alice(&alice_valid, &relay, "suite-required");
    assert!(alice_poll.status.success(), "{}", output_text(&alice_poll));
    let a2 = server
        .drain_channel(ROUTE_TOKEN_BOB)
        .pop()
        .expect("A2 queued");
    assert_v2_suite2_frame(&a2, 3);
    server.replace_channel(ROUTE_TOKEN_BOB, vec![a2.clone()]);
    let bob_confirm = poll_bob(&bob_valid, &relay, "suite-required");
    assert!(
        bob_confirm.status.success(),
        "{}",
        output_text(&bob_confirm)
    );
    assert!(session_path(&alice_valid, "bob").exists());
    assert!(session_path(&bob_valid, "alice").exists());
    assert_session_suite2(&alice_valid, "bob");
    assert_session_suite2(&bob_valid, "alice");
    let valid_text = [
        output_text(&alice_init),
        output_text(&bob_poll),
        output_text(&alice_poll),
        output_text(&bob_confirm),
    ]
    .join("\n");
    assert!(valid_text.contains("reason=ACCEPT_QSC_HS_SUITE2"));
    assert_no_leak_or_panic(&valid_text);
    println!("NA0313_QHSM_V2_PARAMETER_BLOCK_PARSE_OK");
    println!("NA0313_VALID_SUITE2_ACCEPT_OK");
    println!("NA0313_TRANSCRIPT_BINDING_OK");
    println!("NA0313_KEY_CONTEXT_BINDING_OK");

    let (alice_legacy, bob_legacy) = new_pair(&base, "legacy-compat");
    let legacy_init = init_alice(&alice_legacy, &relay, "legacy-compat");
    let legacy_a1 = server
        .drain_channel(ROUTE_TOKEN_BOB)
        .pop()
        .expect("legacy A1 queued");
    assert_v1_frame(&legacy_a1, 1);
    server.replace_channel(ROUTE_TOKEN_BOB, vec![legacy_a1.clone()]);
    let legacy_bob = poll_bob(&bob_legacy, &relay, "legacy-compat");
    let legacy_b1 = server
        .drain_channel(ROUTE_TOKEN_ALICE)
        .pop()
        .expect("legacy B1 queued");
    assert_v1_frame(&legacy_b1, 2);
    server.replace_channel(ROUTE_TOKEN_ALICE, vec![legacy_b1]);
    let legacy_alice = poll_alice(&alice_legacy, &relay, "legacy-compat");
    let legacy_a2 = server
        .drain_channel(ROUTE_TOKEN_BOB)
        .pop()
        .expect("legacy A2 queued");
    assert_v1_frame(&legacy_a2, 3);
    server.replace_channel(ROUTE_TOKEN_BOB, vec![legacy_a2]);
    let legacy_confirm = poll_bob(&bob_legacy, &relay, "legacy-compat");
    let legacy_text = [
        output_text(&legacy_init),
        output_text(&legacy_bob),
        output_text(&legacy_alice),
        output_text(&legacy_confirm),
    ]
    .join("\n");
    assert!(legacy_text.contains("reason=ACCEPT_QSC_HS_LEGACY_COMPATIBILITY"));
    assert!(session_path(&alice_legacy, "bob").exists());
    assert!(session_path(&bob_legacy, "alice").exists());
    assert_no_leak_or_panic(&legacy_text);
    println!("NA0313_LEGACY_COMPAT_ACCEPT_OK");

    let (alice_required_reject, bob_required_reject) = new_pair(&base, "legacy-required-reject");
    let _legacy_init = init_alice(&alice_required_reject, &relay, "legacy-compat");
    let legacy_a1 = server
        .drain_channel(ROUTE_TOKEN_BOB)
        .pop()
        .expect("legacy required-reject A1");
    server.replace_channel(ROUTE_TOKEN_BOB, vec![legacy_a1]);
    let reject = poll_bob(&bob_required_reject, &relay, "suite-required");
    let reject_text = output_text(&reject);
    assert_reject_output(&reject_text, "REJECT_QSC_HS_LEGACY_REQUIRED");
    assert!(!session_path(&bob_required_reject, "alice").exists());
    assert_no_pending(&bob_required_reject, "bob", "alice");
    assert!(server.drain_channel(ROUTE_TOKEN_ALICE).is_empty());
    println!("NA0313_REQUIRED_MODE_LEGACY_REJECT_OK");

    let (alice_fixture, bob_fixture) = new_pair(&base, "parser-fixture");
    let fixture_init = init_alice(&alice_fixture, &relay, "suite-required");
    assert!(
        fixture_init.status.success(),
        "{}",
        output_text(&fixture_init)
    );
    let fixture_a1 = server
        .drain_channel(ROUTE_TOKEN_BOB)
        .pop()
        .expect("fixture A1");
    let invalid_cases = [
        (
            "unsupported",
            replace_param_block(&fixture_a1, &suite_block(0x0500, 0x9999)),
            "REJECT_QSC_HS_SUITE_UNSUPPORTED",
        ),
        (
            "downgrade",
            replace_param_block(&fixture_a1, &suite_block(0x0403, 0x0001)),
            "REJECT_QSC_HS_DOWNGRADE",
        ),
        (
            "stripped",
            replace_param_block(&fixture_a1, &param(UNKNOWN_PARAM_ID, false, &[0x00])),
            "REJECT_QSC_HS_SUITE_MISSING",
        ),
        (
            "duplicate",
            replace_param_block(
                &fixture_a1,
                &concat_params(&[canonical_suite_block(), canonical_suite_block()]),
            ),
            "REJECT_QSC_HS_DUPLICATE_PARAMETER",
        ),
        (
            "unknown-critical",
            replace_param_block(
                &fixture_a1,
                &concat_params(&[
                    canonical_suite_block(),
                    param(UNKNOWN_PARAM_ID, true, SECRET_SENTINEL),
                ]),
            ),
            "REJECT_QSC_HS_UNKNOWN_CRITICAL",
        ),
        (
            "unknown-noncritical",
            replace_param_block(
                &fixture_a1,
                &concat_params(&[
                    canonical_suite_block(),
                    param(UNKNOWN_PARAM_ID, false, &[0x00]),
                ]),
            ),
            "REJECT_QSC_HS_UNKNOWN_PARAMETER",
        ),
        (
            "noncanonical",
            replace_param_block(
                &fixture_a1,
                &concat_params(&[
                    param(UNKNOWN_PARAM_ID, false, &[0x00]),
                    canonical_suite_block(),
                ]),
            ),
            "REJECT_QSC_HS_NONCANONICAL_ORDER",
        ),
        (
            "malformed",
            replace_param_block(
                &fixture_a1,
                &[0x00, 0x01, 0x01, 0x00, 0x03, 0x05, 0x00, 0x00, 0x02],
            ),
            "REJECT_QSC_HS_MALFORMED_LENGTH",
        ),
        (
            "inconsistent",
            replace_param_block(&fixture_a1, &suite_block(0x0403, 0x0002)),
            "REJECT_QSC_HS_INCONSISTENT_TUPLE",
        ),
    ];
    for (label, frame, reason) in invalid_cases {
        server.replace_channel(ROUTE_TOKEN_BOB, vec![frame]);
        let out = poll_bob(&bob_fixture, &relay, "suite-required");
        let text = output_text(&out);
        assert_reject_output(&text, reason);
        assert!(!session_path(&bob_fixture, "alice").exists(), "{label}");
        assert_no_pending(&bob_fixture, "bob", "alice");
        assert!(
            server.drain_channel(ROUTE_TOKEN_ALICE).is_empty(),
            "{label} emitted B1"
        );
    }
    println!("NA0313_UNSUPPORTED_SUITE_REJECT_OK");
    println!("NA0313_DOWNGRADE_SUITE_REJECT_OK");
    println!("NA0313_STRIPPED_SUITE_REJECT_OK");
    println!("NA0313_DUPLICATE_SUITE_REJECT_OK");
    println!("NA0313_UNKNOWN_CRITICAL_REJECT_OK");
    println!("NA0313_NONCANONICAL_REJECT_OK");
    println!("NA0313_MALFORMED_REJECT_OK");

    let (alice_mismatch, bob_mismatch) = new_pair(&base, "a1-b1-mismatch");
    let init = init_alice(&alice_mismatch, &relay, "suite-required");
    assert!(init.status.success(), "{}", output_text(&init));
    let a1 = server.drain_channel(ROUTE_TOKEN_BOB).pop().unwrap();
    server.replace_channel(ROUTE_TOKEN_BOB, vec![a1]);
    let bob = poll_bob(&bob_mismatch, &relay, "suite-required");
    let b1 = server.drain_channel(ROUTE_TOKEN_ALICE).pop().unwrap();
    let bad_b1 = replace_param_block(&b1, &suite_block(0x0500, 0x9999));
    server.replace_channel(ROUTE_TOKEN_ALICE, vec![bad_b1]);
    let alice = poll_alice(&alice_mismatch, &relay, "suite-required");
    let text = [output_text(&bob), output_text(&alice)].join("\n");
    assert_reject_output(&text, "REJECT_QSC_HS_CONTEXT_MISMATCH");
    assert!(!session_path(&alice_mismatch, "bob").exists());
    assert_no_pending(&alice_mismatch, "alice", "bob");
    assert!(server.drain_channel(ROUTE_TOKEN_BOB).is_empty());
    println!("NA0313_MISMATCH_SUITE_REJECT_OK");

    let (alice_transcript, bob_transcript) = new_pair(&base, "transcript-mismatch");
    let init = init_alice(&alice_transcript, &relay, "suite-required");
    assert!(init.status.success(), "{}", output_text(&init));
    let a1 = server.drain_channel(ROUTE_TOKEN_BOB).pop().unwrap();
    server.replace_channel(ROUTE_TOKEN_BOB, vec![a1]);
    let bob = poll_bob(&bob_transcript, &relay, "suite-required");
    let bob_text = output_text(&bob);
    let b1 = server.drain_channel(ROUTE_TOKEN_ALICE).pop().unwrap();
    let bad_b1 = mutate_b1_transcript_field(&b1, &bob_text);
    server.replace_channel(ROUTE_TOKEN_ALICE, vec![bad_b1]);
    let alice = poll_alice(&alice_transcript, &relay, "suite-required");
    let text = output_text(&alice);
    assert_reject_output(&text, "REJECT_QSC_HS_TRANSCRIPT_CONTEXT");
    assert!(!session_path(&alice_transcript, "bob").exists());
    assert_no_pending(&alice_transcript, "alice", "bob");
    assert!(server.drain_channel(ROUTE_TOKEN_BOB).is_empty());

    let (alice_key_context, _bob_key_context) = new_pair(&base, "key-context-missing");
    let legacy_init = init_alice(&alice_key_context, &relay, "legacy-compat");
    assert!(
        legacy_init.status.success(),
        "{}",
        output_text(&legacy_init)
    );
    let legacy_a1 = server.drain_channel(ROUTE_TOKEN_BOB).pop().unwrap();
    let mut fake_b1 = b1.clone();
    set_session_id(&mut fake_b1, &session_id(&legacy_a1));
    server.replace_channel(ROUTE_TOKEN_ALICE, vec![fake_b1]);
    let alice = poll_alice(&alice_key_context, &relay, "suite-required");
    let text = output_text(&alice);
    assert_reject_output(&text, "REJECT_QSC_HS_KEY_CONTEXT");
    assert!(!session_path(&alice_key_context, "bob").exists());
    assert_no_pending(&alice_key_context, "alice", "bob");

    let (alice_a2_mismatch, bob_a2_mismatch) = new_pair(&base, "b1-a2-mismatch");
    let init = init_alice(&alice_a2_mismatch, &relay, "suite-required");
    assert!(init.status.success(), "{}", output_text(&init));
    let a1 = server.drain_channel(ROUTE_TOKEN_BOB).pop().unwrap();
    server.replace_channel(ROUTE_TOKEN_BOB, vec![a1]);
    let bob = poll_bob(&bob_a2_mismatch, &relay, "suite-required");
    assert!(bob.status.success(), "{}", output_text(&bob));
    let b1 = server.drain_channel(ROUTE_TOKEN_ALICE).pop().unwrap();
    server.replace_channel(ROUTE_TOKEN_ALICE, vec![b1]);
    let alice = poll_alice(&alice_a2_mismatch, &relay, "suite-required");
    assert!(alice.status.success(), "{}", output_text(&alice));
    let a2 = server.drain_channel(ROUTE_TOKEN_BOB).pop().unwrap();
    let bad_a2 = replace_param_block(&a2, &suite_block(0x0500, 0x9999));
    server.replace_channel(ROUTE_TOKEN_BOB, vec![bad_a2]);
    let bob = poll_bob(&bob_a2_mismatch, &relay, "suite-required");
    let text = output_text(&bob);
    assert_reject_output(&text, "REJECT_QSC_HS_CONTEXT_MISMATCH");
    assert!(!session_path(&bob_a2_mismatch, "alice").exists());
    assert_no_pending(&bob_a2_mismatch, "bob", "alice");

    let (alice_replay_a1, bob_replay_a1) = new_pair(&base, "replay-a1");
    let init = init_alice(&alice_replay_a1, &relay, "suite-required");
    assert!(init.status.success(), "{}", output_text(&init));
    let a1 = server.drain_channel(ROUTE_TOKEN_BOB).pop().unwrap();
    server.replace_channel(ROUTE_TOKEN_BOB, vec![a1.clone()]);
    let bob = poll_bob(&bob_replay_a1, &relay, "suite-required");
    assert!(bob.status.success(), "{}", output_text(&bob));
    let _ = server.drain_channel(ROUTE_TOKEN_ALICE);
    server.replace_channel(ROUTE_TOKEN_BOB, vec![a1]);
    let replay = poll_bob(&bob_replay_a1, &relay, "suite-required");
    let text = output_text(&replay);
    assert_reject_output(&text, "REJECT_QSC_HS_REPLAY");
    assert!(!session_path(&bob_replay_a1, "alice").exists());
    assert_no_pending(&bob_replay_a1, "bob", "alice");

    let before_replay_a2 = fs::read(session_path(&bob_valid, "alice")).expect("bob session");
    server.replace_channel(ROUTE_TOKEN_BOB, vec![a2]);
    let replay = poll_bob(&bob_valid, &relay, "suite-required");
    let text = output_text(&replay);
    assert_reject_output(&text, "REJECT_QSC_HS_REPLAY");
    let after_replay_a2 = fs::read(session_path(&bob_valid, "alice")).expect("bob session after");
    assert_eq!(before_replay_a2, after_replay_a2);

    println!("NA0313_NO_MUTATION_ON_REJECT_OK");
    println!("NA0313_NO_OUTPUT_ON_REJECT_OK");
    println!("NA0313_NO_SECRET_LEAK_OK");
    println!("NA0313_QSC_SUITE_ID_PARAMETER_BLOCK_OK");
}
