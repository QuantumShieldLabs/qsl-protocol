mod common;

use quantumshield_refimpl::crypto::stdcrypto::StdCrypto;
use quantumshield_refimpl::crypto::traits::Hash;
use serde_json::json;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::{Mutex, MutexGuard, OnceLock};

const ROUTE_TOKEN_BOB: &str = "route_token_bob_na0189";

fn file_integrity_test_guard() -> MutexGuard<'static, ()> {
    static TEST_GUARD: OnceLock<Mutex<()>> = OnceLock::new();
    TEST_GUARD
        .get_or_init(|| Mutex::new(()))
        .lock()
        .unwrap_or_else(|e| e.into_inner())
}

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
    let _ = fs::create_dir_all(path);
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let _ = fs::set_permissions(path, fs::Permissions::from_mode(0o700));
    }
}

fn create_dir_700(path: &Path) {
    let _ = fs::remove_dir_all(path);
    ensure_dir_700(path);
}

fn output_text(out: &std::process::Output) -> String {
    let mut s = String::from_utf8_lossy(&out.stdout).to_string();
    s.push_str(&String::from_utf8_lossy(&out.stderr));
    s
}

fn qsc_base(cfg: &Path) -> Command {
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("qsc"));
    cmd.env("QSC_CONFIG_DIR", cfg)
        .env("QSC_MARK_FORMAT", "plain")
        .env("QSC_QSP_SEED", "1")
        .env("QSC_ALLOW_SEED_FALLBACK", "1");
    cmd
}

fn leak_counts(text: &str) -> (usize, usize) {
    let v1 = text.matches("/v1/").count();
    let bytes = text.as_bytes();
    let mut long_hex = 0usize;
    let mut i = 0usize;
    while i < bytes.len() {
        let b = bytes[i];
        if !b.is_ascii_hexdigit() {
            i += 1;
            continue;
        }
        let start = i;
        while i < bytes.len() && bytes[i].is_ascii_hexdigit() {
            i += 1;
        }
        if i.saturating_sub(start) >= 32 {
            long_hex += 1;
        }
    }
    (v1, long_hex)
}

fn assert_no_leaks(text: &str) {
    let (v1, hex32) = leak_counts(text);
    assert_eq!(v1, 0, "found v1 path marker in output: {text}");
    assert_eq!(hex32, 0, "found long hex marker in output: {text}");
    let upper = text.to_ascii_uppercase();
    assert!(
        !upper.contains("AUTHORIZATION") && !upper.contains("BEARER"),
        "found auth marker in output: {text}"
    );
}

fn hex_encode(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut out = String::with_capacity(bytes.len() * 2);
    for &b in bytes {
        out.push(HEX[(b >> 4) as usize] as char);
        out.push(HEX[(b & 0x0f) as usize] as char);
    }
    out
}

fn file_xfer_chunk_hash(chunk: &[u8]) -> String {
    let c = StdCrypto;
    let hash = c.sha512(chunk);
    hex_encode(&hash[..16])
}

fn file_xfer_id(peer: &str, filename: &str, payload: &[u8]) -> String {
    let c = StdCrypto;
    let mut data = Vec::new();
    data.extend_from_slice(peer.as_bytes());
    data.push(0);
    data.extend_from_slice(filename.as_bytes());
    data.push(0);
    data.extend_from_slice(payload);
    let hash = c.sha512(&data);
    hex_encode(&hash[..12])
}

fn file_xfer_manifest_hash(
    file_id: &str,
    total_size: usize,
    chunk_count: usize,
    chunk_hashes: &[String],
) -> String {
    let c = StdCrypto;
    let joined = chunk_hashes.join(",");
    let data = format!("{}|{}|{}|{}", file_id, total_size, chunk_count, joined);
    let hash = c.sha512(data.as_bytes());
    hex_encode(&hash[..16])
}

fn contacts_add_with_route_token(cfg: &Path, label: &str, token: &str) {
    let out = qsc_base(cfg)
        .args([
            "contacts",
            "add",
            "--label",
            label,
            "--fp",
            "fp-test",
            "--route-token",
            token,
        ])
        .output()
        .expect("contacts add route token");
    assert!(out.status.success(), "{}", output_text(&out));
}

fn relay_set_inbox_token(cfg: &Path, token: &str) {
    let out = qsc_base(cfg)
        .args(["relay", "inbox-set", "--token", token])
        .output()
        .expect("relay inbox set");
    assert!(out.status.success(), "{}", output_text(&out));
}

fn send_payload_file(cfg: &Path, relay: &str, to: &str, path: &Path) -> String {
    let out = qsc_base(cfg)
        .args([
            "send",
            "--transport",
            "relay",
            "--relay",
            relay,
            "--to",
            to,
            "--file",
            path.to_str().unwrap(),
        ])
        .output()
        .expect("send payload");
    let text = output_text(&out);
    assert!(out.status.success(), "{}", text);
    text
}

#[test]
fn manifest_verification_accepts_payloads_when_sender_and_receiver_peer_labels_differ() {
    let _guard = file_integrity_test_guard();
    let server = common::start_inbox_server(1024 * 1024, 128);
    let base = safe_test_root().join(format!(
        "na0189_manifest_peer_labels_{}",
        std::process::id()
    ));
    create_dir_700(&base);
    let alice_cfg = base.join("alice_cfg");
    let bob_cfg = base.join("bob_cfg");
    let bob_out = base.join("bob_out");
    create_dir_700(&alice_cfg);
    create_dir_700(&bob_cfg);
    create_dir_700(&bob_out);
    common::init_mock_vault(&alice_cfg);
    common::init_mock_vault(&bob_cfg);
    contacts_add_with_route_token(&alice_cfg, "bob", ROUTE_TOKEN_BOB);
    relay_set_inbox_token(&bob_cfg, ROUTE_TOKEN_BOB);

    let payload = b"na0189-small-file".to_vec();
    let filename = "small.bin";
    let sender_peer = "alice";
    let receiver_peer = "bob";
    assert_ne!(sender_peer, receiver_peer);
    let file_id = file_xfer_id(sender_peer, filename, &payload);
    let chunk_hash = file_xfer_chunk_hash(&payload);
    let manifest_hash = file_xfer_manifest_hash(
        file_id.as_str(),
        payload.len(),
        1,
        std::slice::from_ref(&chunk_hash),
    );

    let chunk_path = base.join("chunk.json");
    let chunk = json!({
        "v": 1,
        "t": "file_chunk",
        "file_id": file_id,
        "filename": filename,
        "total_size": payload.len(),
        "chunk_index": 0,
        "chunk_count": 1,
        "chunk_hash": chunk_hash,
        "manifest_hash": manifest_hash,
        "chunk": payload,
    });
    fs::write(
        &chunk_path,
        serde_json::to_vec(&chunk).expect("encode chunk"),
    )
    .expect("write chunk");

    let manifest_path = base.join("manifest.json");
    let manifest = json!({
        "v": 1,
        "t": "file_manifest",
        "file_id": chunk["file_id"],
        "filename": filename,
        "total_size": chunk["total_size"],
        "chunk_count": 1,
        "chunk_hashes": [chunk["chunk_hash"].as_str().unwrap()],
        "manifest_hash": chunk["manifest_hash"],
        "confirm_requested": false,
        "confirm_id": "",
    });
    fs::write(
        &manifest_path,
        serde_json::to_vec(&manifest).expect("encode manifest"),
    )
    .expect("write manifest");

    let chunk_send = send_payload_file(&alice_cfg, server.base_url(), receiver_peer, &chunk_path);
    let manifest_send =
        send_payload_file(&alice_cfg, server.base_url(), receiver_peer, &manifest_path);
    let recv = qsc_base(&bob_cfg)
        .args([
            "receive",
            "--transport",
            "relay",
            "--relay",
            server.base_url(),
            "--mailbox",
            ROUTE_TOKEN_BOB,
            "--from",
            receiver_peer,
            "--max",
            "8",
            "--out",
            bob_out.to_str().unwrap(),
        ])
        .output()
        .expect("receive payloads");
    assert!(recv.status.success(), "{}", output_text(&recv));
    let recv_text = output_text(&recv);
    assert!(
        recv_text.contains("event=file_xfer_manifest")
            && recv_text.contains("event=file_xfer_complete"),
        "{}",
        recv_text
    );
    assert!(
        !recv_text.contains("reason=manifest_mismatch")
            && !recv_text.contains("QSC_FILE_INTEGRITY_FAIL")
            && !recv_text.contains("state=peer_confirmed"),
        "{}",
        recv_text
    );

    assert_no_leaks(&chunk_send);
    assert_no_leaks(&manifest_send);
    assert_no_leaks(&recv_text);
}
