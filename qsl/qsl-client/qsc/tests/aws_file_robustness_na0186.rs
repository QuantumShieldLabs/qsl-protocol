mod common;

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::{Mutex, MutexGuard, OnceLock};

const ROUTE_TOKEN_BOB: &str = "route_token_bob_abcdefghijklmnopqr";

fn test_guard() -> MutexGuard<'static, ()> {
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
        if !bytes[i].is_ascii_hexdigit() {
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
        .expect("contacts add");
    assert!(out.status.success(), "{}", output_text(&out));
    let list = qsc_base(cfg)
        .args(["contacts", "device", "list", "--label", label])
        .output()
        .expect("contacts device list");
    assert!(list.status.success(), "{}", output_text(&list));
    let list_text = output_text(&list);
    let device_id = list_text
        .lines()
        .find(|line| line.starts_with("device="))
        .and_then(|line| {
            line.split_whitespace()
                .find_map(|tok| tok.strip_prefix("device="))
        })
        .unwrap_or_else(|| panic!("missing device id in output: {list_text}"));
    let trust = qsc_base(cfg)
        .args([
            "contacts",
            "device",
            "trust",
            "--label",
            label,
            "--device",
            device_id,
            "--confirm",
        ])
        .output()
        .expect("contacts trust");
    assert!(trust.status.success(), "{}", output_text(&trust));
}

fn relay_set_inbox_token(cfg: &Path, token: &str) {
    let out = qsc_base(cfg)
        .args(["relay", "inbox-set", "--token", token])
        .output()
        .expect("relay inbox set");
    assert!(out.status.success(), "{}", output_text(&out));
}

fn send_payload_file(
    cfg: &Path,
    relay: &str,
    to: &str,
    payload_path: &Path,
) -> std::process::Output {
    qsc_base(cfg)
        .args([
            "send",
            "--transport",
            "relay",
            "--relay",
            relay,
            "--to",
            to,
            "--file",
            payload_path.to_str().unwrap_or("<invalid>"),
        ])
        .output()
        .expect("send payload file")
}

#[test]
fn integrity_failure_manifest_mismatch_fail_clean_and_rerun_reset() {
    let _guard = test_guard();
    let server = common::start_inbox_server(1024 * 1024, 64);
    let base = safe_test_root().join(format!("na0186_integrity_{}", std::process::id()));
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

    let chunk_path = base.join("chunk.json");
    let chunk_json = r#"{"v":1,"t":"file_chunk","file_id":"filea001","filename":"x.bin","total_size":4,"chunk_index":0,"chunk_count":1,"chunk_hash":"a7c976db1723adb41274178dc82e9b77","manifest_hash":"m1","chunk":[1,2,3,4]}"#;
    fs::write(&chunk_path, chunk_json.as_bytes()).expect("write chunk");

    let bad_manifest_path = base.join("manifest_bad.json");
    let bad_manifest_json = r#"{"v":1,"t":"file_manifest","file_id":"filea001","filename":"x.bin","total_size":4,"chunk_count":1,"chunk_hashes":["a7c976db1723adb41274178dc82e9b77"],"manifest_hash":"bad","confirm_requested":false,"confirm_id":"c1"}"#;
    fs::write(&bad_manifest_path, bad_manifest_json.as_bytes()).expect("write manifest");

    let send_chunk = send_payload_file(&alice_cfg, server.base_url(), "bob", &chunk_path);
    assert!(send_chunk.status.success(), "{}", output_text(&send_chunk));
    let send_manifest = send_payload_file(&alice_cfg, server.base_url(), "bob", &bad_manifest_path);
    assert!(
        send_manifest.status.success(),
        "{}",
        output_text(&send_manifest)
    );

    let recv_bad = qsc_base(&bob_cfg)
        .args([
            "receive",
            "--transport",
            "relay",
            "--relay",
            server.base_url(),
            "--mailbox",
            ROUTE_TOKEN_BOB,
            "--from",
            "bob",
            "--max",
            "2",
            "--out",
            bob_out.to_str().unwrap_or("<invalid>"),
        ])
        .output()
        .expect("recv bad manifest");
    assert!(!recv_bad.status.success(), "manifest mismatch must fail");
    let recv_bad_text = output_text(&recv_bad);
    assert!(
        recv_bad_text.contains("reason=manifest_mismatch"),
        "{recv_bad_text}"
    );
    assert!(
        recv_bad_text
            .contains("QSC_FILE_INTEGRITY_FAIL reason=manifest_mismatch action=purge_partials"),
        "{recv_bad_text}"
    );

    // Re-send first chunk after failure; receiver should reset stale state and accept.
    let resend_chunk = send_payload_file(&alice_cfg, server.base_url(), "bob", &chunk_path);
    assert!(
        resend_chunk.status.success(),
        "{}",
        output_text(&resend_chunk)
    );
    let recv_reset = qsc_base(&bob_cfg)
        .args([
            "receive",
            "--transport",
            "relay",
            "--relay",
            server.base_url(),
            "--mailbox",
            ROUTE_TOKEN_BOB,
            "--from",
            "bob",
            "--max",
            "1",
            "--out",
            bob_out.to_str().unwrap_or("<invalid>"),
        ])
        .output()
        .expect("recv reset chunk");
    assert!(recv_reset.status.success(), "{}", output_text(&recv_reset));
    let recv_reset_text = output_text(&recv_reset);
    assert!(
        recv_reset_text.contains("event=file_xfer_reset")
            && recv_reset_text.contains("reason=rerun_detected"),
        "{recv_reset_text}"
    );
    let (v1_bad, hex_bad) = leak_counts(&recv_bad_text);
    assert_eq!(v1_bad, 0, "v1 path leak in bad recv output");
    assert_eq!(hex_bad, 0, "hex32plus leak in bad recv output");
    let (v1_reset, hex_reset) = leak_counts(&recv_reset_text);
    assert_eq!(v1_reset, 0, "v1 path leak in reset recv output");
    assert_eq!(hex_reset, 0, "hex32plus leak in reset recv output");
}

#[test]
fn file_chunk_push_retry_is_bounded_and_deterministic() {
    let _guard = test_guard();
    let server = common::start_inbox_server_with_fail_pushes(1024 * 1024, 128, 2);
    let base = safe_test_root().join(format!("na0186_retry_{}", std::process::id()));
    create_dir_700(&base);
    let cfg = base.join("cfg");
    create_dir_700(&cfg);
    common::init_mock_vault(&cfg);
    contacts_add_with_route_token(&cfg, "bob", ROUTE_TOKEN_BOB);
    let payload = base.join("data.bin");
    fs::write(&payload, vec![0x5a; 4096]).expect("payload");

    let out = qsc_base(&cfg)
        .args([
            "file",
            "send",
            "--transport",
            "relay",
            "--relay",
            server.base_url(),
            "--to",
            "bob",
            "--path",
            payload.to_str().unwrap_or("<invalid>"),
            "--chunk-size",
            "2048",
        ])
        .output()
        .expect("file send retry");
    let text = output_text(&out);
    assert!(out.status.success(), "{text}");
    assert!(text.contains("QSC_FILE_PUSH_RETRY attempt=1"), "{text}");
    assert!(text.contains("QSC_FILE_PUSH_RETRY attempt=2"), "{text}");
    assert!(text.contains("backoff_ms=50"), "{text}");
    assert!(text.contains("backoff_ms=100"), "{text}");
    let (v1, hex) = leak_counts(&text);
    assert_eq!(v1, 0, "v1 path leak in retry output");
    assert_eq!(hex, 0, "hex32plus leak in retry output");
}

#[test]
fn file_chunk_push_retry_exhaustion_fails_closed() {
    let _guard = test_guard();
    let server = common::start_inbox_server_with_fail_pushes(1024 * 1024, 128, 8);
    let base = safe_test_root().join(format!("na0186_retry_exhaust_{}", std::process::id()));
    create_dir_700(&base);
    let cfg = base.join("cfg");
    create_dir_700(&cfg);
    common::init_mock_vault(&cfg);
    contacts_add_with_route_token(&cfg, "bob", ROUTE_TOKEN_BOB);
    let payload = base.join("data.bin");
    fs::write(&payload, vec![0x5a; 4096]).expect("payload");

    let out = qsc_base(&cfg)
        .args([
            "file",
            "send",
            "--transport",
            "relay",
            "--relay",
            server.base_url(),
            "--to",
            "bob",
            "--path",
            payload.to_str().unwrap_or("<invalid>"),
            "--chunk-size",
            "2048",
        ])
        .output()
        .expect("file send retry exhausted");
    let text = output_text(&out);
    assert!(
        !out.status.success(),
        "send must fail after bounded retries"
    );
    assert!(text.contains("QSC_FILE_PUSH_RETRY attempt=1"), "{text}");
    assert!(text.contains("QSC_FILE_PUSH_RETRY attempt=2"), "{text}");
    assert!(text.contains("relay_inbox_push_failed"), "{text}");
    let (v1, hex) = leak_counts(&text);
    assert_eq!(v1, 0, "v1 path leak in retry exhaustion output");
    assert_eq!(hex, 0, "hex32plus leak in retry exhaustion output");
}
