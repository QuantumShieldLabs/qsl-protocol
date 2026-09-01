//! NA-0771 (`D-1412`) — THE ENG-0252 ARMS.
//!
//! `ENG-0252`: a frame carrying nothing but the public 16-byte `session_id` and arbitrary
//! bytes could destroy the addressed handshake pending record before any cryptographic
//! operation ran. The repair deletes every `hs_pending_clear` that is not class (i) (a
//! session was stored) or class (iv) (the local record will not parse), and makes the
//! initiator's context-mismatch exit a `continue` as the responder's already was.
//!
//! ## THE FOUR ARMS AND WHAT EACH PINS
//!
//!   * **A1** the INITIATOR site (`:1783`). A 6438-byte forged RESP carrying the victim's
//!     cleartext session id. Asserts the record is UNTOUCHED with `assert_eq!(before,
//!     after)` — the strong form. `assert!(!after.is_empty())` cannot tell "survived"
//!     from "replaced" and is not used here.
//!   * **A2** the RESPONDER decode-Err site (`:2247`), reached by NINE bytes that do not
//!     even carry a session id. Ships with BOTH controls: the positive (a `suite-required`
//!     initiator makes a `legacy-compat` responder's pending WIRE-EXPLICIT) and the
//!     negative (a `legacy-compat` initiator must leave it null). Without the negative the
//!     positive proves nothing.
//!   * **A3** the RESPONDER confirm-MAC site (`:2101`), reached by a confirm whose mac and
//!     sig are ZERO. It is class (ii) — "a MAC was checked" — and a zero mac reaches it,
//!     which is why class (ii) is not automatically safe.
//!   * **A4** the LEASE-SERVER completion arm, on a REAL in-process `qsl-server` with real
//!     lease expiry, at **N = 1 and N = 4** poison frames against `--max 4`.
//!
//! ## ⚠⚠ WHY A4 ASSERTS A FAILURE AT N = 4, AND WHY THAT IS NOT A BUG
//!
//! The relay delivers in strict insertion order (`store.rs:723`, `ORDER BY seq LIMIT ?3`),
//! a lease moves no message (`:752`), and rejects are never acked. So for **N < `--max`**
//! the poison and the honest B1 arrive together and the `continue` walks past the poison
//! to complete the handshake; for **N >= `--max`** the pull returns only poison and there
//! is nothing behind it to continue to.
//!
//! **This lane does not repair that.** It is `ENG-0198`'s recorded shape — *"the
//! budget-exhaustion mechanism ... on the poll side, with `--max` as the budget the
//! rejects exhaust"* — which is OPEN and pre-existing. `ENG-0252` is filed as a third
//! cause of the same outward shape.
//!
//! ⚠ A4's N = 4 case therefore asserts a state this lane calls a DEFECT. It is pinned as
//! the **`ENG-0198` BOUNDARY**, not as desired behaviour: it exists so that a later lane
//! which changes `--max`, the fetch shape, or the ack discipline sees this arm move
//! instead of finding a silent pass. **When `ENG-0198` is repaired, THIS ARM SHOULD GO
//! RED, and that is the arm working.**
//!
//! ## THE COUNT GUARD, AND WHAT IT DOES NOT CATCH
//! `g_clear_sites` pins that `hs_pending_clear` has exactly four call sites and names
//! them. Its limits are stated at the guard itself.

#![allow(dead_code, unused_imports, unused_variables)]
mod common;

use quantumshield_refimpl::crypto::stdcrypto::{
    runtime_pq_kem_ciphertext_bytes, runtime_pq_sig_public_key_bytes,
    runtime_pq_sig_signature_bytes,
};
use std::thread;
use std::time::Duration;

// ⚠ The harness helpers below are TRANSCRIBED from
// `na_0313_handshake_suite_id_parameter_block.rs` rather than shared: they are not in
// `tests/common/`, and `common/mod.rs` is consumed by ~108 targets, so widening it for one
// lane would put every one of them in this change's blast radius. `na_0313` itself
// transcribes from `handshake_mvp.rs` for the same reason. Transcribed, not re-derived.


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

fn identity_kem_pk(cfg: &Path, label: &str) -> String {
    let out = run_qsc(cfg, &["identity", "show", "--as", label]);
    assert!(out.status.success(), "{}", output_text(&out));
    output_text(&out)
        .lines()
        .find_map(|line| line.strip_prefix("identity_kem_pk="))
        .map(ToOwned::to_owned)
        .unwrap_or_else(|| panic!("missing identity_kem_pk in output: {}", output_text(&out)))
}

fn identity_sig_pk(cfg: &Path, label: &str) -> String {
    let out = run_qsc(cfg, &["identity", "show", "--as", label]);
    assert!(out.status.success(), "{}", output_text(&out));
    output_text(&out)
        .lines()
        .find_map(|line| line.strip_prefix("identity_sig_pk="))
        .map(ToOwned::to_owned)
        .unwrap_or_else(|| panic!("missing identity_sig_pk in output: {}", output_text(&out)))
}

fn contacts_add_authenticated_with_route(cfg: &Path, label: &str, fp: &str, kem_pk: &str, sig_pk: &str, token: &str) {
    let out = run_qsc(
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
            "--sig-pk",
            sig_pk,
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
    let alice_kem = identity_kem_pk(alice_cfg, "alice");
    let alice_sig = identity_sig_pk(alice_cfg, "alice");
    let bob_fp = identity_fp(bob_cfg, "bob");
    let bob_kem = identity_kem_pk(bob_cfg, "bob");
    let bob_sig = identity_sig_pk(bob_cfg, "bob");
    contacts_add_authenticated_with_route(alice_cfg, "bob", bob_fp.as_str(), bob_kem.as_str(), bob_sig.as_str(), ROUTE_TOKEN_BOB);
    contacts_add_authenticated_with_route(bob_cfg, "alice", alice_fp.as_str(), alice_kem.as_str(), alice_sig.as_str(), ROUTE_TOKEN_ALICE);
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
    assert_eq!(&bytes[0..6], b"QSCV02");
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
        .decrypt(
            Nonce::from_slice(nonce),
            Payload {
                msg: ciphertext,
                // NA-0694 (D628 §2e F3): the product now binds the 53-byte header as
                // AEAD AAD; the header prefix of the file is that AAD verbatim.
                aad: &bytes[..53],
            },
        )
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
        // NA-0624: a v3 plaintext additionally carries scka_len(u32 LE) + SCKA section between
        // the trigger and the QS2S snapshot (scka_len == 0 for a non-advertising session).
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

const LEASE_SECS: usize = 3;
const LEASE_WAIT: Duration = Duration::from_millis(4500);

fn pending_raw(cfg: &Path, self_label: &str, peer: &str) -> String {
    read_mock_vault_secret(cfg, &format!("handshake.pending.{self_label}.{peer}"))
        .unwrap_or_default()
}

fn pending_suite_context(cfg: &Path, self_label: &str, peer: &str) -> Value {
    let raw = pending_raw(cfg, self_label, peer);
    assert!(!raw.is_empty(), "no pending record for {self_label}.{peer}");
    let v: Value = serde_json::from_str(&raw).expect("pending is JSON");
    v.get("suite_context").cloned().unwrap_or(Value::Null)
}

fn session_id_of(cfg: &Path, self_label: &str, peer: &str) -> [u8; 16] {
    let v: Value = serde_json::from_str(&pending_raw(cfg, self_label, peer)).unwrap();
    let arr = v.get("session_id").and_then(|x| x.as_array()).expect("session_id");
    let mut s = [0u8; 16];
    for (k, b) in arr.iter().enumerate() { s[k] = b.as_u64().unwrap() as u8; }
    s
}

fn push_raw(base: &str, token: &str, bytes: &[u8]) {
    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(5)).build().expect("client");
    let r = client.post(&format!("{base}/v1/push"))
        .header("X-QSL-Route-Token", token)
        .body(bytes.to_vec()).send().expect("push");
    assert_eq!(r.status().as_u16(), 200, "raw push rejected");
}

/// A wire-version-2 RESP carrying `session_id` and otherwise zeros. The whole of the
/// attacker's bill of materials is the published inbox route token and the cleartext
/// session id; no key, no MAC, no signature.
fn forged_v2_resp(session_id: &[u8; 16]) -> Vec<u8> {
    let payload_len = 16 + runtime_pq_kem_ciphertext_bytes() + 32
        + runtime_pq_sig_public_key_bytes() + runtime_pq_sig_signature_bytes() + 32;
    let mut f = Vec::with_capacity(9 + payload_len);
    f.extend_from_slice(b"QHSM");
    f.extend_from_slice(&2u16.to_be_bytes());
    f.push(2u8);                                 // HS_TYPE_RESP
    f.extend_from_slice(&0u16.to_be_bytes());    // parameter block length 0
    f.extend_from_slice(session_id);
    f.resize(9 + payload_len, 0u8);
    f
}

// ── A1 ─────────────────────────────────────────────────────────────────────
#[test]
fn na0771_a1_initiator_pending_survives_unauthenticated_frame() {
    let server = common::start_inbox_server(1024 * 1024, 64);
    let base = safe_test_root().join(format!("na0771_a1_{}", std::process::id()));
    create_dir_700(&base);
    let relay = server.base_url().to_string();
    let (alice, _bob) = new_pair(&base, "a1");

    let init = init_alice(&alice, &relay, "legacy-compat");
    assert!(init.status.success(), "{}", output_text(&init));
    let a1 = server.drain_channel(ROUTE_TOKEN_BOB).pop().expect("A1 queued");
    assert_eq!(u16::from_be_bytes([a1[4], a1[5]]), 1, "legacy-compat emits a v1 A1");

    let before = pending_raw(&alice, "alice", "bob");
    assert!(!before.is_empty(), "precondition: alice holds a pending record");

    // the session id, read off the wire exactly as a relay operator would
    let sid: [u8; 16] = a1[7..23].try_into().expect("session_id at the v1 payload offset");
    let evil = forged_v2_resp(&sid);
    server.replace_channel(ROUTE_TOKEN_ALICE, vec![evil]);

    let poll = poll_alice(&alice, &relay, "legacy-compat");
    let text = output_text(&poll);
    assert!(text.contains("REJECT_QSC_HS_CONTEXT_MISMATCH"), "the frame must still be REJECTED by name: {text}");

    let after = pending_raw(&alice, "alice", "bob");
    // ⚠ THE STRONG FORM. `!after.is_empty()` would pass on a record that survived but was
    // REPLACED; equality is the property.
    assert_eq!(before, after, "ENG-0252: an unauthenticated frame must leave the pending record UNTOUCHED");
}

// ── A2 ─────────────────────────────────────────────────────────────────────
#[test]
fn na0771_a2_responder_pending_survives_nine_byte_frame() {
    let server = common::start_inbox_server(1024 * 1024, 64);
    let base = safe_test_root().join(format!("na0771_a2_{}", std::process::id()));
    create_dir_700(&base);
    let relay = server.base_url().to_string();

    // POSITIVE CONTROL: a suite-required initiator makes a legacy-compat responder's
    // pending WIRE-EXPLICIT. `hs_decode_init` admits the parameter block where the two
    // poll decoders do not.
    let (a_pos, b_pos) = new_pair(&base, "a2-pos");
    let i = init_alice(&a_pos, &relay, "suite-required");
    assert!(i.status.success(), "{}", output_text(&i));
    let a1 = server.drain_channel(ROUTE_TOKEN_BOB).pop().expect("A1");
    assert_eq!(u16::from_be_bytes([a1[4], a1[5]]), 2, "suite-required emits a v2 A1");
    server.replace_channel(ROUTE_TOKEN_BOB, vec![a1]);
    assert!(poll_bob(&b_pos, &relay, "legacy-compat").status.success());
    let ctx_pos = pending_suite_context(&b_pos, "bob", "alice");
    assert!(!ctx_pos.is_null(), "positive control: the responder's pending must be wire-explicit");

    // NEGATIVE CONTROL: a legacy-compat initiator must leave it null. Without this arm the
    // positive proves nothing.
    let (a_neg, b_neg) = new_pair(&base, "a2-neg");
    assert!(init_alice(&a_neg, &relay, "legacy-compat").status.success());
    let a1n = server.drain_channel(ROUTE_TOKEN_BOB).pop().expect("A1 v1");
    server.replace_channel(ROUTE_TOKEN_BOB, vec![a1n]);
    assert!(poll_bob(&b_neg, &relay, "legacy-compat").status.success());
    assert!(pending_suite_context(&b_neg, "bob", "alice").is_null(),
        "negative control: a v1 A1 must leave suite_context null");

    // THE NINE BYTES. No session id at all.
    let before = pending_raw(&b_pos, "bob", "alice");
    assert!(!before.is_empty());
    let nine = vec![0x51u8, 0x48, 0x53, 0x4d, 0x00, 0x02, 0x03, 0x00, 0x00];
    assert_eq!(nine.len(), 9);
    server.replace_channel(ROUTE_TOKEN_BOB, vec![nine]);
    assert!(poll_bob(&b_pos, &relay, "legacy-compat").status.success());
    let after = pending_raw(&b_pos, "bob", "alice");
    assert_eq!(before, after, "ENG-0252: nine bytes must not destroy a wire-explicit responder pending");
}

// ── A3 ─────────────────────────────────────────────────────────────────────
#[test]
fn na0771_a3_responder_pending_survives_zero_mac_confirm() {
    let server = common::start_inbox_server(1024 * 1024, 64);
    let base = safe_test_root().join(format!("na0771_a3_{}", std::process::id()));
    create_dir_700(&base);
    let relay = server.base_url().to_string();
    let (alice, bob) = new_pair(&base, "a3");

    assert!(init_alice(&alice, &relay, "suite-required").status.success());
    let a1 = server.drain_channel(ROUTE_TOKEN_BOB).pop().expect("A1");
    server.replace_channel(ROUTE_TOKEN_BOB, vec![a1.clone()]);
    assert!(poll_bob(&bob, &relay, "legacy-compat").status.success());
    assert!(!pending_suite_context(&bob, "bob", "alice").is_null());
    let before = pending_raw(&bob, "bob", "alice");

    // A well-formed CONFIRM: the pending's own session id, the CANONICAL public block, and
    // ZERO for mac and sig. It passes the context test and fails the confirm MAC.
    let sid = session_id(&a1);
    let block = canonical_suite_block();
    let payload_len = 16 + 32 + runtime_pq_sig_signature_bytes();
    let mut f = Vec::with_capacity(9 + block.len() + payload_len);
    f.extend_from_slice(b"QHSM");
    f.extend_from_slice(&2u16.to_be_bytes());
    f.push(3u8);                                        // HS_TYPE_CONFIRM
    f.extend_from_slice(&(block.len() as u16).to_be_bytes());
    f.extend_from_slice(&block);
    f.extend_from_slice(&sid);
    f.resize(9 + block.len() + payload_len, 0u8);
    server.replace_channel(ROUTE_TOKEN_BOB, vec![f]);

    let text = output_text(&poll_bob(&bob, &relay, "legacy-compat"));
    assert!(text.contains("REJECT_QSC_HS_TRANSCRIPT_CONTEXT"),
        "the confirm must still be rejected by name at :2101's branch: {text}");
    let after = pending_raw(&bob, "bob", "alice");
    assert_eq!(before, after,
        "a MAC that FAILED proves the sender knew nothing; the record must survive");
}

// ── A4 ─────────────────────────────────────────────────────────────────────
fn a4_at(n: usize) -> (bool, String) {
    let server = common::start_qsl_server_with_store(2 * 1024 * 1024, 512, None, LEASE_SECS);
    let base = safe_test_root().join(format!("na0771_a4_{}_{}", n, std::process::id()));
    create_dir_700(&base);
    let relay = server.base_url().to_string();
    let (alice, bob) = new_pair(&base, &format!("a4n{n}"));

    assert!(init_alice(&alice, &relay, "legacy-compat").status.success());
    let sid = session_id_of(&alice, "alice", "bob");
    let evil = forged_v2_resp(&sid);
    for _ in 0..n { push_raw(&relay, ROUTE_TOKEN_ALICE, &evil); }

    // bob answers honestly; his B1 queues BEHIND the poison, in insertion order
    assert!(poll_bob(&bob, &relay, "legacy-compat").status.success());

    let p1 = output_text(&poll_alice(&alice, &relay, "legacy-compat"));
    thread::sleep(LEASE_WAIT);
    let p2 = output_text(&poll_alice(&alice, &relay, "legacy-compat"));
    (session_path(&alice, "bob").exists(), format!("{p1}\n{p2}"))
}

#[test]
fn na0771_a4_lease_relay_completes_below_max_and_not_at_it() {
    // N = 1 < --max 4 : the `continue` walks past the poison and the handshake COMPLETES.
    let (session_n1, text_n1) = a4_at(1);
    assert!(text_n1.contains("REJECT_QSC_HS_CONTEXT_MISMATCH"), "the poison must be rejected by name");
    assert!(session_n1, "N=1: the honest B1 behind the poison must complete the handshake");

    // N = 4 >= --max 4 : the pull returns ONLY poison and there is nothing to continue to.
    // ⚠⚠ THIS ASSERTS A DEFECT, DELIBERATELY. It is `ENG-0198`'s budget-exhaustion shape,
    // OPEN and pre-existing, which NA-0771 does NOT repair. Pinned as the BOUNDARY so a
    // later lane that changes `--max`, the fetch shape or the ack discipline sees this arm
    // move rather than finding a silent pass.
    // ⇒ WHEN `ENG-0198` IS REPAIRED THIS ASSERTION SHOULD GO RED. That is the arm working.
    let (session_n4, _text_n4) = a4_at(4);
    assert!(!session_n4,
        "N=4: ENG-0198 BOUNDARY — at or above `--max` the poll returns only poison and \
         cannot reach the honest B1. If this failed, ENG-0198's shape has changed and this \
         arm must be re-derived, not deleted.");
}

// ── THE COUNT GUARD ────────────────────────────────────────────────────────
#[test]
fn na0771_g_clear_sites_are_three_and_named() {
    let src = fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/handshake/mod.rs"))
        .expect("read handshake/mod.rs");
    let calls: Vec<usize> = src
        .lines()
        .enumerate()
        .filter(|(_, l)| l.trim() == "let _ = hs_pending_clear(self_label, peer);")
        .map(|(i, _)| i + 1)
        .collect();
    // ⚠⚠ FOUR -> THREE, UPDATED BY NA-0775 (`D-1418`) ON `RULING_NA0775_008` sec 2, WHICH
    // ORDERS THIS EDIT BY NAME. THE INVARIANT IS UNTOUCHED AND IS NOW STRICTLY STRONGER.
    //
    // NA-0775 deleted the TWO class-(iv) clears — the unparseable-suite-context exits in both
    // the initiator and responder pending branches — because a clear there destroyed the state
    // a later pass needed while the new ack contract refused to retire the frame, leaving it
    // permanently un-consumable (`ENG-0281`). **ZERO CLASS-(iv) SITES REMAIN.** All three
    // survivors are class (i): each sits immediately after a successful `qsp_session_store`,
    // one of them in NA-0775's late-landing guard's SKIP arm, where the session was stored by
    // ANOTHER pass of the same handshake.
    //
    // ⚠ THIS GUARD WENT RED BECAUSE IT DID ITS JOB. Its author wrote that a later lane should
    //   "see this arm move rather than find a silent pass" — that is quoted here rather than
    //   paraphrased, because it is the reason this test was worth keeping. It moved. It was not
    //   relaxed, not converted to a range, and not deleted; only the literal count and the site
    //   list changed, and the WHEN it guards is unchanged.
    assert_eq!(calls.len(), 3,
        "NA-0771 INVARIANT, AS TIGHTENED BY NA-0775 (D-1418): a pending record is destroyed \
         only when a session was stored (class i). The class-(iv) arm of this invariant — the \
         local record will not parse — HAS NO SITES: NA-0775 deleted both (ENG-0281). Found {} \
         call sites at {:?}; expected 3, all class (i). If this is FOUR again, a clear has been \
         put back at a deleted site or a new one added — re-derive the class of every site \
         before changing this number.", calls.len(), calls);

    // ⚠⚠ WHAT THIS GUARD DOES NOT CATCH (M-5). A count pins WHERE; the invariant is about
    // WHEN. Three mechanisms defeat it and are present in the file today:
    //   (i)   a clear INLINED through `vault::secret_set(&key, "")` + `fs::remove_file`,
    //         which is all `hs_pending_clear` is — the count stays at four;
    //   (ii)  a `hs_pending_store` of a record with emptied fields — `secret_set` on the
    //         pending key has THREE writers (`:1231` the loader's legacy migration,
    //         `:1244` store, `:1253` clear), and store is unconditional;
    //   (iii) a surviving site MOVED — wrapping the class-(i) clear in a condition, or
    //         hoisting it above `qsp_session_store`, keeps four and breaks class (i).
    // ⇒ THE PROPERTY'S REAL PIN IS A1–A4, which assert what survives what. This guard is a
    //   cheap tripwire for the likeliest regression: a clear put back at a deleted site.
}
