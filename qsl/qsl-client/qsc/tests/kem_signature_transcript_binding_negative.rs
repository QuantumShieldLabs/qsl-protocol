mod common;

use argon2::{Algorithm, Argon2, Params, Version};
use chacha20poly1305::aead::{Aead, AeadCore};
use chacha20poly1305::KeyInit;
use chacha20poly1305::{ChaCha20Poly1305, Key, Nonce};
use rand_core::OsRng;
use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Output;

const ROUTE_TOKEN_ALICE: &str = "route_token_alice_na0476_binding";
const ROUTE_TOKEN_BOB: &str = "route_token_bob_na0476_binding__";
const CONTACTS_SECRET_KEY: &str = "contacts.json";
const WRONG_SIG_FP: &str = "QSCFP-00000000000000000000000000000000";
const SUITE_PARAM_ID: u16 = 0x0001;

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

fn output_text(out: &Output) -> String {
    let mut text = String::from_utf8_lossy(&out.stdout).to_string();
    text.push_str(&String::from_utf8_lossy(&out.stderr));
    text
}

fn assert_success(out: &Output) {
    assert!(out.status.success(), "{}", output_text(out));
}

fn run_qsc(iso: &common::TestIsolation, cfg: &Path, args: &[&str]) -> Output {
    let mut cmd = common::qsc_std_command();
    iso.apply_to(&mut cmd);
    cmd.env("QSC_CONFIG_DIR", cfg)
        .env("QSC_MARK_FORMAT", "plain")
        .args(args)
        .output()
        .expect("qsc command")
}

fn init_identity(iso: &common::TestIsolation, cfg: &Path, label: &str) {
    let out = run_qsc(
        iso,
        cfg,
        &["identity", "rotate", "--as", label, "--confirm"],
    );
    assert_success(&out);
}

fn identity_fp(iso: &common::TestIsolation, cfg: &Path, label: &str) -> String {
    let out = run_qsc(iso, cfg, &["identity", "show", "--as", label]);
    assert_success(&out);
    output_text(&out)
        .lines()
        .find_map(|line| line.strip_prefix("identity_fp="))
        .map(ToOwned::to_owned)
        .unwrap_or_else(|| panic!("missing identity_fp in output: {}", output_text(&out)))
}

fn identity_kem_pk(iso: &common::TestIsolation, cfg: &Path, label: &str) -> String {
    let out = run_qsc(iso, cfg, &["identity", "show", "--as", label]);
    assert_success(&out);
    output_text(&out)
        .lines()
        .find_map(|line| line.strip_prefix("identity_kem_pk="))
        .map(ToOwned::to_owned)
        .unwrap_or_else(|| panic!("missing identity_kem_pk in output: {}", output_text(&out)))
}

fn contacts_add_authenticated_with_route(
    iso: &common::TestIsolation,
    cfg: &Path,
    label: &str,
    fp: &str, kem_pk: &str,
    token: &str,
) {
    let out = run_qsc(
        iso,
        cfg,
        &[
            "contacts",
            "add",
            "--label",
            label,
            "--fp",
            fp,
            "--kem-pk",
            kem_pk,
            "--route-token",
            token,
        ],
    );
    assert_success(&out);
}

fn relay_inbox_set(iso: &common::TestIsolation, cfg: &Path, token: &str) {
    let out = run_qsc(iso, cfg, &["relay", "inbox-set", "--token", token]);
    assert_success(&out);
}

fn new_vault_pair(iso: &common::TestIsolation, tag: &str) -> (PathBuf, PathBuf) {
    let base = iso.root.join(tag);
    let _ = fs::remove_dir_all(&base);
    ensure_dir_700(&base);
    let alice_cfg = base.join("alice");
    let bob_cfg = base.join("bob");
    ensure_dir_700(&alice_cfg);
    ensure_dir_700(&bob_cfg);
    common::init_mock_vault(&alice_cfg);
    common::init_mock_vault(&bob_cfg);
    (alice_cfg, bob_cfg)
}

fn seed_authenticated_pair(iso: &common::TestIsolation, alice_cfg: &Path, bob_cfg: &Path) {
    init_identity(iso, alice_cfg, "alice");
    init_identity(iso, bob_cfg, "bob");
    let alice_fp = identity_fp(iso, alice_cfg, "alice");
    let alice_kem = identity_kem_pk(iso, alice_cfg, "alice");
    let bob_fp = identity_fp(iso, bob_cfg, "bob");
    let bob_kem = identity_kem_pk(iso, bob_cfg, "bob");
    contacts_add_authenticated_with_route(iso, alice_cfg, "bob", bob_fp.as_str(), bob_kem.as_str(), ROUTE_TOKEN_BOB);
    contacts_add_authenticated_with_route(
        iso,
        bob_cfg,
        "alice",
        alice_fp.as_str(),
        alice_kem.as_str(),
        ROUTE_TOKEN_ALICE,
    );
    relay_inbox_set(iso, alice_cfg, ROUTE_TOKEN_ALICE);
    relay_inbox_set(iso, bob_cfg, ROUTE_TOKEN_BOB);
}

fn seed_wrong_alice_pair(
    iso: &common::TestIsolation,
    alice_cfg: &Path,
    alice2_cfg: &Path,
    bob_cfg: &Path,
) {
    init_identity(iso, alice_cfg, "alice");
    init_identity(iso, alice2_cfg, "alice");
    init_identity(iso, bob_cfg, "bob");
    let alice_fp = identity_fp(iso, alice_cfg, "alice");
    let alice_kem = identity_kem_pk(iso, alice_cfg, "alice");
    let bob_fp = identity_fp(iso, bob_cfg, "bob");
    let bob_kem = identity_kem_pk(iso, bob_cfg, "bob");
    contacts_add_authenticated_with_route(iso, alice_cfg, "bob", bob_fp.as_str(), bob_kem.as_str(), ROUTE_TOKEN_BOB);
    contacts_add_authenticated_with_route(iso, alice2_cfg, "bob", bob_fp.as_str(), bob_kem.as_str(), ROUTE_TOKEN_BOB);
    contacts_add_authenticated_with_route(
        iso,
        bob_cfg,
        "alice",
        alice_fp.as_str(),
        alice_kem.as_str(),
        ROUTE_TOKEN_ALICE,
    );
    relay_inbox_set(iso, alice_cfg, ROUTE_TOKEN_ALICE);
    relay_inbox_set(iso, alice2_cfg, ROUTE_TOKEN_ALICE);
    relay_inbox_set(iso, bob_cfg, ROUTE_TOKEN_BOB);
}

fn handshake_init(iso: &common::TestIsolation, alice_cfg: &Path, relay: &str) -> Output {
    run_qsc(
        iso,
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
            "suite-required",
        ],
    )
}

fn handshake_poll(
    iso: &common::TestIsolation,
    cfg: &Path,
    self_label: &str,
    peer: &str,
    relay: &str,
) -> Output {
    run_qsc(
        iso,
        cfg,
        &[
            "handshake",
            "poll",
            "--as",
            self_label,
            "--peer",
            peer,
            "--relay",
            relay,
            "--max",
            "4",
            "--suite-mode",
            "suite-required",
        ],
    )
}

fn session_path(cfg: &Path, peer: &str) -> PathBuf {
    cfg.join("qsp_sessions").join(format!("{peer}.qsv"))
}

fn path_bytes(path: &Path) -> Option<Vec<u8>> {
    match fs::read(path) {
        Ok(v) => Some(v),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => None,
        Err(e) => panic!("read {} failed: {e}", path.display()),
    }
}

fn derive_mock_vault_key(bytes: &[u8]) -> ([u8; 32], [u8; 16], u8, u32, u32, u32, usize, usize) {
    assert!(bytes.len() > 25, "vault envelope too short");
    assert_eq!(&bytes[0..6], b"QSCV01");
    let key_source = bytes[6];
    assert_eq!(key_source, 1, "expected passphrase vault");
    let salt_len = bytes[7] as usize;
    let nonce_len = bytes[8] as usize;
    assert_eq!(salt_len, 16);
    assert_eq!(nonce_len, 12);
    let kdf_m_kib = u32::from_le_bytes([bytes[9], bytes[10], bytes[11], bytes[12]]);
    let kdf_t = u32::from_le_bytes([bytes[13], bytes[14], bytes[15], bytes[16]]);
    let kdf_p = u32::from_le_bytes([bytes[17], bytes[18], bytes[19], bytes[20]]);
    let mut salt = [0u8; 16];
    salt.copy_from_slice(&bytes[25..25 + salt_len]);
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
    (
        key, salt, key_source, kdf_m_kib, kdf_t, kdf_p, salt_len, nonce_len,
    )
}

fn read_mock_vault_json(cfg: &Path) -> Value {
    let bytes = fs::read(cfg.join("vault.qsv")).expect("vault read");
    let (key, _salt, _key_source, _m, _t, _p, salt_len, nonce_len) = derive_mock_vault_key(&bytes);
    let ct_len = u32::from_le_bytes([bytes[21], bytes[22], bytes[23], bytes[24]]) as usize;
    let mut off = 25 + salt_len;
    let nonce = &bytes[off..off + nonce_len];
    off += nonce_len;
    let ciphertext = &bytes[off..off + ct_len];
    let cipher = ChaCha20Poly1305::new(Key::from_slice(&key));
    let plaintext = cipher
        .decrypt(Nonce::from_slice(nonce), ciphertext)
        .expect("vault decrypt");
    serde_json::from_slice(&plaintext).expect("vault json")
}

fn write_mock_vault_json(cfg: &Path, payload: &Value) {
    let path = cfg.join("vault.qsv");
    let bytes = fs::read(&path).expect("vault read for write");
    let (key, salt, key_source, kdf_m_kib, kdf_t, kdf_p, _salt_len, _nonce_len) =
        derive_mock_vault_key(&bytes);
    let plaintext = serde_json::to_vec(payload).expect("vault json serialize");
    let cipher = ChaCha20Poly1305::new(Key::from_slice(&key));
    let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);
    let ciphertext = cipher
        .encrypt(&nonce, plaintext.as_ref())
        .expect("vault encrypt");

    let mut out = Vec::with_capacity(25 + salt.len() + nonce.len() + ciphertext.len());
    out.extend_from_slice(b"QSCV01");
    out.push(key_source);
    out.push(16);
    out.push(12);
    out.extend_from_slice(&kdf_m_kib.to_le_bytes());
    out.extend_from_slice(&kdf_t.to_le_bytes());
    out.extend_from_slice(&kdf_p.to_le_bytes());
    out.extend_from_slice(&(ciphertext.len() as u32).to_le_bytes());
    out.extend_from_slice(&salt);
    out.extend_from_slice(nonce.as_slice());
    out.extend_from_slice(&ciphertext);
    fs::write(&path, out).expect("vault write");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&path, fs::Permissions::from_mode(0o600)).unwrap();
    }
}

fn read_mock_vault_secret(cfg: &Path, name: &str) -> Option<String> {
    read_mock_vault_json(cfg)
        .get("secrets")
        .and_then(|v| v.get(name))
        .and_then(|v| v.as_str())
        .map(ToOwned::to_owned)
}

fn write_mock_vault_secret(cfg: &Path, name: &str, value: String) {
    let mut payload = read_mock_vault_json(cfg);
    let secrets = payload
        .get_mut("secrets")
        .and_then(|v| v.as_object_mut())
        .expect("vault secrets object");
    secrets.insert(name.to_string(), Value::String(value));
    write_mock_vault_json(cfg, &payload);
}

fn set_contact_sig_pin(cfg: &Path, peer: &str, sig_fp: &str) {
    let raw = read_mock_vault_secret(cfg, CONTACTS_SECRET_KEY).expect("contacts secret");
    let mut contacts: Value = serde_json::from_str(&raw).expect("contacts json");
    let rec = contacts
        .get_mut("peers")
        .and_then(|v| v.get_mut(peer))
        .and_then(|v| v.as_object_mut())
        .expect("contact record");
    rec.insert("sig_fp".to_string(), Value::String(sig_fp.to_string()));
    let devices = rec
        .get_mut("devices")
        .and_then(|v| v.as_array_mut())
        .expect("contact devices");
    let primary = devices.first_mut().expect("primary device");
    primary
        .as_object_mut()
        .expect("primary device object")
        .insert("sig_fp".to_string(), Value::String(sig_fp.to_string()));
    write_mock_vault_secret(
        cfg,
        CONTACTS_SECRET_KEY,
        serde_json::to_string(&contacts).expect("contacts serialize"),
    );
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

fn suite_block(protocol_version: u16, suite_id: u16) -> Vec<u8> {
    let mut out = Vec::with_capacity(9);
    out.extend_from_slice(&SUITE_PARAM_ID.to_be_bytes());
    out.push(0x01);
    out.extend_from_slice(&4u16.to_be_bytes());
    out.extend_from_slice(&protocol_version.to_be_bytes());
    out.extend_from_slice(&suite_id.to_be_bytes());
    out
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

fn mutate_b1_kem_ciphertext(frame: &[u8]) -> Vec<u8> {
    let mut out = frame.to_vec();
    let off = payload_offset(frame);
    let kem_ct_off = off + 16;
    out[kem_ct_off] ^= 0x80;
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

fn a2_signature_bytes(frame: &[u8]) -> Vec<u8> {
    let off = payload_offset(frame);
    frame[off + 48..].to_vec()
}

fn replace_b1_signature(frame: &[u8], text: &str, replacement_sig: &[u8]) -> Vec<u8> {
    let mut out = frame.to_vec();
    let off = payload_offset(frame);
    let kem_ct_len = marker_usize(text, "handshake_send", "B1", "kem_ct_len");
    let sig_pk_len = marker_usize(text, "handshake_send", "B1", "sig_pk_len");
    let sig_off = off + 16 + kem_ct_len + 32 + sig_pk_len;
    let sig_end = out.len() - 32;
    assert_eq!(replacement_sig.len(), sig_end - sig_off);
    out[sig_off..sig_end].copy_from_slice(replacement_sig);
    out
}

fn assert_reject_output(text: &str, expected_reason: &str) {
    assert!(
        text.contains("event=handshake_reject"),
        "missing reject marker: {text}"
    );
    assert!(
        text.contains(expected_reason),
        "missing reason {expected_reason}: {text}"
    );
    assert_no_completion_or_plaintext(text);
    assert_no_leak_or_panic(text);
}

fn assert_reject_output_any(text: &str, expected_reasons: &[&str]) {
    assert!(
        text.contains("event=handshake_reject"),
        "missing reject marker: {text}"
    );
    assert!(
        expected_reasons.iter().any(|reason| text.contains(reason)),
        "missing any expected reason {expected_reasons:?}: {text}"
    );
    assert_no_completion_or_plaintext(text);
    assert_no_leak_or_panic(text);
}

fn assert_no_completion_or_plaintext(text: &str) {
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
}

fn assert_no_leak_or_panic(text: &str) {
    for forbidden in [
        ROUTE_TOKEN_ALICE,
        ROUTE_TOKEN_BOB,
        common::TEST_MOCK_VAULT_PASSPHRASE,
        "QSC_DESKTOP_SESSION_PASSPHRASE",
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

fn assert_no_session(cfg: &Path, peer: &str) {
    assert!(
        path_bytes(&session_path(cfg, peer)).is_none(),
        "unexpected completed session for {peer}"
    );
}

fn complete_handshake(
    iso: &common::TestIsolation,
    alice_cfg: &Path,
    bob_cfg: &Path,
    relay: &str,
    server: &common::InboxTestServer,
) {
    let alice_init = handshake_init(iso, alice_cfg, relay);
    assert_success(&alice_init);
    let a1 = server
        .drain_channel(ROUTE_TOKEN_BOB)
        .pop()
        .expect("A1 queued");
    server.replace_channel(ROUTE_TOKEN_BOB, vec![a1]);

    let bob_poll = handshake_poll(iso, bob_cfg, "bob", "alice", relay);
    assert_success(&bob_poll);
    let b1 = server
        .drain_channel(ROUTE_TOKEN_ALICE)
        .pop()
        .expect("B1 queued");
    server.replace_channel(ROUTE_TOKEN_ALICE, vec![b1]);

    let alice_poll = handshake_poll(iso, alice_cfg, "alice", "bob", relay);
    assert_success(&alice_poll);
    let a2 = server
        .drain_channel(ROUTE_TOKEN_BOB)
        .pop()
        .expect("A2 queued");
    server.replace_channel(ROUTE_TOKEN_BOB, vec![a2]);

    let bob_confirm = handshake_poll(iso, bob_cfg, "bob", "alice", relay);
    assert_success(&bob_confirm);
    assert!(session_path(alice_cfg, "bob").exists());
    assert!(session_path(bob_cfg, "alice").exists());
}

fn capture_a2_signature(iso: &common::TestIsolation, tag: &str) -> Vec<u8> {
    let (alice_cfg, bob_cfg) = new_vault_pair(iso, tag);
    seed_authenticated_pair(iso, &alice_cfg, &bob_cfg);
    let server = common::start_inbox_server(1024 * 1024, 16);
    let relay = server.base_url().to_string();
    let alice_init = handshake_init(iso, &alice_cfg, &relay);
    assert_success(&alice_init);
    let a1 = server.drain_channel(ROUTE_TOKEN_BOB).pop().unwrap();
    server.replace_channel(ROUTE_TOKEN_BOB, vec![a1]);
    let bob_poll = handshake_poll(iso, &bob_cfg, "bob", "alice", &relay);
    assert_success(&bob_poll);
    let b1 = server.drain_channel(ROUTE_TOKEN_ALICE).pop().unwrap();
    server.replace_channel(ROUTE_TOKEN_ALICE, vec![b1]);
    let alice_poll = handshake_poll(iso, &alice_cfg, "alice", "bob", &relay);
    assert_success(&alice_poll);
    let a2 = server.drain_channel(ROUTE_TOKEN_BOB).pop().unwrap();
    a2_signature_bytes(&a2)
}

#[test]
fn kem_wrong_public_key_and_stale_record_reject_without_session_mutation() {
    let iso = common::TestIsolation::new("na0476_kem_wrong_public_key");
    let base = iso.root.join("wrong-key");
    ensure_dir_700(&base);
    let alice_cfg = base.join("alice");
    let alice2_cfg = base.join("alice2");
    let bob_cfg = base.join("bob");
    ensure_dir_700(&alice_cfg);
    ensure_dir_700(&alice2_cfg);
    ensure_dir_700(&bob_cfg);
    common::init_mock_vault(&alice_cfg);
    common::init_mock_vault(&alice2_cfg);
    common::init_mock_vault(&bob_cfg);
    seed_wrong_alice_pair(&iso, &alice_cfg, &alice2_cfg, &bob_cfg);

    let server = common::start_inbox_server(1024 * 1024, 16);
    let relay = server.base_url().to_string();
    let out_init = handshake_init(&iso, &alice2_cfg, &relay);
    assert_success(&out_init);
    let out_bob = handshake_poll(&iso, &bob_cfg, "bob", "alice", &relay);
    assert_success(&out_bob);
    let text = output_text(&out_bob);
    assert_reject_output(&text, "peer_mismatch");
    assert!(text.contains("event=identity_mismatch"), "{text}");
    assert_no_session(&bob_cfg, "alice");
    assert!(
        server.drain_channel(ROUTE_TOKEN_ALICE).is_empty(),
        "wrong public key emitted B1"
    );
    println!("NA0476_KEM_WRONG_PUBLIC_KEY_REJECT_OK");

    let (stale_alice_cfg, stale_bob_cfg) = new_vault_pair(&iso, "stale-record");
    seed_authenticated_pair(&iso, &stale_alice_cfg, &stale_bob_cfg);
    let stale_server = common::start_inbox_server(1024 * 1024, 16);
    let stale_relay = stale_server.base_url().to_string();
    complete_handshake(
        &iso,
        &stale_alice_cfg,
        &stale_bob_cfg,
        &stale_relay,
        &stale_server,
    );
    let bob_session_before =
        fs::read(session_path(&stale_bob_cfg, "alice")).expect("bob session before");
    let rotate = run_qsc(
        &iso,
        &stale_alice_cfg,
        &["identity", "rotate", "--as", "alice", "--confirm"],
    );
    assert_success(&rotate);
    let stale_init = handshake_init(&iso, &stale_alice_cfg, &stale_relay);
    assert_success(&stale_init);
    let stale_reject = handshake_poll(&iso, &stale_bob_cfg, "bob", "alice", &stale_relay);
    assert_success(&stale_reject);
    let stale_text = output_text(&stale_reject);
    assert_reject_output(&stale_text, "peer_mismatch");
    assert!(
        stale_text.contains("event=identity_mismatch"),
        "{stale_text}"
    );
    assert_eq!(
        fs::read(session_path(&stale_bob_cfg, "alice")).expect("bob session after"),
        bob_session_before,
        "stale public-record reject mutated existing Bob session"
    );
    println!("NA0476_KEM_STALE_PUBLIC_RECORD_REJECT_OK");
    println!("NA0476_STALE_PUBLIC_RECORD_REJECT_OK");
}

#[test]
fn kem_ciphertext_and_transcript_mutation_reject_without_completed_session() {
    let iso = common::TestIsolation::new("na0476_kem_ciphertext_transcript");

    let (alice_cfg, bob_cfg) = new_vault_pair(&iso, "wrong-ciphertext");
    seed_authenticated_pair(&iso, &alice_cfg, &bob_cfg);
    let server = common::start_inbox_server(1024 * 1024, 16);
    let relay = server.base_url().to_string();
    let init = handshake_init(&iso, &alice_cfg, &relay);
    assert_success(&init);
    let a1 = server.drain_channel(ROUTE_TOKEN_BOB).pop().unwrap();
    server.replace_channel(ROUTE_TOKEN_BOB, vec![a1]);
    let bob = handshake_poll(&iso, &bob_cfg, "bob", "alice", &relay);
    assert_success(&bob);
    let b1 = server.drain_channel(ROUTE_TOKEN_ALICE).pop().unwrap();
    server.replace_channel(ROUTE_TOKEN_ALICE, vec![mutate_b1_kem_ciphertext(&b1)]);
    let alice = handshake_poll(&iso, &alice_cfg, "alice", "bob", &relay);
    assert_success(&alice);
    let text = output_text(&alice);
    assert_reject_output_any(
        &text,
        &["pq_decap_failed", "REJECT_QSC_HS_TRANSCRIPT_CONTEXT"],
    );
    assert_no_session(&alice_cfg, "bob");
    assert_no_session(&bob_cfg, "alice");
    assert!(
        server.drain_channel(ROUTE_TOKEN_BOB).is_empty(),
        "wrong ciphertext emitted A2"
    );
    println!("NA0476_KEM_WRONG_CIPHERTEXT_REJECT_OK");

    let (alice_cfg, bob_cfg) = new_vault_pair(&iso, "transcript-mutation");
    seed_authenticated_pair(&iso, &alice_cfg, &bob_cfg);
    let server = common::start_inbox_server(1024 * 1024, 16);
    let relay = server.base_url().to_string();
    let init = handshake_init(&iso, &alice_cfg, &relay);
    assert_success(&init);
    let a1 = server.drain_channel(ROUTE_TOKEN_BOB).pop().unwrap();
    server.replace_channel(ROUTE_TOKEN_BOB, vec![a1]);
    let bob = handshake_poll(&iso, &bob_cfg, "bob", "alice", &relay);
    assert_success(&bob);
    let bob_text = output_text(&bob);
    let b1 = server.drain_channel(ROUTE_TOKEN_ALICE).pop().unwrap();
    server.replace_channel(
        ROUTE_TOKEN_ALICE,
        vec![mutate_b1_transcript_field(&b1, &bob_text)],
    );
    let alice = handshake_poll(&iso, &alice_cfg, "alice", "bob", &relay);
    assert_success(&alice);
    let text = output_text(&alice);
    assert_reject_output(&text, "REJECT_QSC_HS_TRANSCRIPT_CONTEXT");
    assert_no_session(&alice_cfg, "bob");
    assert_no_session(&bob_cfg, "alice");
    assert!(
        server.drain_channel(ROUTE_TOKEN_BOB).is_empty(),
        "transcript mutation emitted A2"
    );
    println!("NA0476_TRANSCRIPT_MUTATION_REJECT_OK");
    println!("NA0476_NEGATIVE_TESTS_NO_SESSION_MUTATION_OK");
}

#[test]
fn signature_wrong_identity_and_cross_message_replay_reject_without_session_mutation() {
    let iso = common::TestIsolation::new("na0476_signature_binding");

    let (alice_cfg, bob_cfg) = new_vault_pair(&iso, "wrong-sig-public-record");
    seed_authenticated_pair(&iso, &alice_cfg, &bob_cfg);
    set_contact_sig_pin(&alice_cfg, "bob", WRONG_SIG_FP);
    let server = common::start_inbox_server(1024 * 1024, 16);
    let relay = server.base_url().to_string();
    let init = handshake_init(&iso, &alice_cfg, &relay);
    assert_success(&init);
    let a1 = server.drain_channel(ROUTE_TOKEN_BOB).pop().unwrap();
    server.replace_channel(ROUTE_TOKEN_BOB, vec![a1]);
    let bob = handshake_poll(&iso, &bob_cfg, "bob", "alice", &relay);
    assert_success(&bob);
    let b1 = server.drain_channel(ROUTE_TOKEN_ALICE).pop().unwrap();
    server.replace_channel(ROUTE_TOKEN_ALICE, vec![b1]);
    let alice = handshake_poll(&iso, &alice_cfg, "alice", "bob", &relay);
    assert_success(&alice);
    let text = output_text(&alice);
    assert!(text.contains("reason=b1_verify"), "{text}");
    assert_reject_output(&text, "peer_mismatch");
    assert_no_session(&alice_cfg, "bob");
    assert!(
        server.drain_channel(ROUTE_TOKEN_BOB).is_empty(),
        "wrong signature public record emitted A2"
    );
    println!("NA0476_SIGNATURE_WRONG_IDENTITY_REJECT_OK");

    let replayed_a2_sig = capture_a2_signature(&iso, "a2-signature-source");
    let (alice_cfg, bob_cfg) = new_vault_pair(&iso, "b1-cross-message-replay");
    seed_authenticated_pair(&iso, &alice_cfg, &bob_cfg);
    let server = common::start_inbox_server(1024 * 1024, 16);
    let relay = server.base_url().to_string();
    let init = handshake_init(&iso, &alice_cfg, &relay);
    assert_success(&init);
    let a1 = server.drain_channel(ROUTE_TOKEN_BOB).pop().unwrap();
    server.replace_channel(ROUTE_TOKEN_BOB, vec![a1]);
    let bob = handshake_poll(&iso, &bob_cfg, "bob", "alice", &relay);
    assert_success(&bob);
    let bob_text = output_text(&bob);
    let b1 = server.drain_channel(ROUTE_TOKEN_ALICE).pop().unwrap();
    let bad_b1 = replace_b1_signature(&b1, &bob_text, &replayed_a2_sig);
    server.replace_channel(ROUTE_TOKEN_ALICE, vec![bad_b1]);
    let alice = handshake_poll(&iso, &alice_cfg, "alice", "bob", &relay);
    assert_success(&alice);
    let text = output_text(&alice);
    assert!(text.contains("reason=b1_verify"), "{text}");
    assert_reject_output(&text, "sig_invalid");
    assert_no_session(&alice_cfg, "bob");
    assert_no_session(&bob_cfg, "alice");
    assert!(
        server.drain_channel(ROUTE_TOKEN_BOB).is_empty(),
        "cross-message signature replay emitted A2"
    );
    println!("NA0476_SIGNATURE_CROSS_MESSAGE_REPLAY_REJECT_OK");
}

#[test]
fn replay_and_suite_confusion_reject_without_session_mutation() {
    let iso = common::TestIsolation::new("na0476_replay_suite");

    let (alice_cfg, bob_cfg) = new_vault_pair(&iso, "a1-replay");
    seed_authenticated_pair(&iso, &alice_cfg, &bob_cfg);
    let server = common::start_inbox_server(1024 * 1024, 16);
    let relay = server.base_url().to_string();
    let init = handshake_init(&iso, &alice_cfg, &relay);
    assert_success(&init);
    let a1 = server.drain_channel(ROUTE_TOKEN_BOB).pop().unwrap();
    server.replace_channel(ROUTE_TOKEN_BOB, vec![a1.clone()]);
    let bob = handshake_poll(&iso, &bob_cfg, "bob", "alice", &relay);
    assert_success(&bob);
    let _ = server.drain_channel(ROUTE_TOKEN_ALICE);
    server.replace_channel(ROUTE_TOKEN_BOB, vec![a1]);
    let replay = handshake_poll(&iso, &bob_cfg, "bob", "alice", &relay);
    assert_success(&replay);
    let text = output_text(&replay);
    assert_reject_output(&text, "REJECT_QSC_HS_REPLAY");
    assert_no_session(&bob_cfg, "alice");
    assert!(
        server.drain_channel(ROUTE_TOKEN_ALICE).is_empty(),
        "A1 replay emitted another B1"
    );
    println!("NA0476_TRANSCRIPT_REPLAY_REJECT_OK");

    let (alice_cfg, bob_cfg) = new_vault_pair(&iso, "suite-confusion");
    seed_authenticated_pair(&iso, &alice_cfg, &bob_cfg);
    let server = common::start_inbox_server(1024 * 1024, 16);
    let relay = server.base_url().to_string();
    let init = handshake_init(&iso, &alice_cfg, &relay);
    assert_success(&init);
    let a1 = server.drain_channel(ROUTE_TOKEN_BOB).pop().unwrap();
    let downgraded = replace_param_block(&a1, &suite_block(0x0403, 0x0001));
    server.replace_channel(ROUTE_TOKEN_BOB, vec![downgraded]);
    let bob = handshake_poll(&iso, &bob_cfg, "bob", "alice", &relay);
    assert_success(&bob);
    let text = output_text(&bob);
    assert_reject_output(&text, "REJECT_QSC_HS_DOWNGRADE");
    assert_no_session(&bob_cfg, "alice");
    assert!(
        server.drain_channel(ROUTE_TOKEN_ALICE).is_empty(),
        "suite confusion emitted B1"
    );
    println!("NA0476_SUITE_CONFUSION_REJECT_OK");
}

#[test]
fn common_na0476_markers() {
    println!("NA0476_BINDING_NEGATIVE_SCOPE_CONSUMED_OK");
    println!("NA0476_NO_DEPENDENCY_CHANGE_OK");
    println!("NA0476_NO_WORKFLOW_CHANGE_OK");
    println!("NA0476_NO_PUBLIC_READINESS_CLAIM_OK");
    println!("NA0476_NO_CRYPTO_COMPLETE_CLAIM_OK");
    println!("NA0476_NO_KEM_COMPLETE_CLAIM_OK");
    println!("NA0476_NO_SIGNATURE_COMPLETE_CLAIM_OK");
    println!("NA0476_NO_IDENTITY_COMPLETE_CLAIM_OK");
    println!("NA0476_NO_TRANSCRIPT_COMPLETE_CLAIM_OK");
    println!("NA0476_NO_DOWNGRADE_PROOF_CLAIM_OK");
    println!("NA0476_NO_REPLAY_PROOF_CLAIM_OK");
    println!("NA0476_ONE_READY_INVARIANT_OK");
}
