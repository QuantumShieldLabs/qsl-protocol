mod common;

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::{Mutex, MutexGuard, OnceLock};

const ROUTE_TOKEN_BOB: &str = "route_token_bob_abcdefghijklmnopqr";

fn file_transfer_test_guard() -> MutexGuard<'static, ()> {
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

fn collect_files(root: &Path) -> Vec<(String, Vec<u8>)> {
    fn walk(base: &Path, cur: &Path, out: &mut Vec<(String, Vec<u8>)>) {
        let Ok(rd) = fs::read_dir(cur) else {
            return;
        };
        let mut ents: Vec<_> = rd.filter_map(Result::ok).collect();
        ents.sort_by_key(|e| e.path());
        for e in ents {
            let p = e.path();
            if p.is_dir() {
                walk(base, &p, out);
                continue;
            }
            if p.is_file() {
                let rel = p
                    .strip_prefix(base)
                    .unwrap_or(&p)
                    .to_string_lossy()
                    .to_string();
                let bytes = fs::read(&p).unwrap_or_default();
                out.push((rel, bytes));
            }
        }
    }
    let mut out = Vec::new();
    walk(root, root, &mut out);
    out.sort_by(|a, b| a.0.cmp(&b.0));
    out
}

fn run_file_send(
    cfg: &Path,
    relay: &str,
    to: &str,
    path: &Path,
    chunk_size: usize,
) -> std::process::Output {
    qsc_base(cfg)
        .args([
            "file",
            "send",
            "--transport",
            "relay",
            "--relay",
            relay,
            "--to",
            to,
            "--path",
            path.to_str().unwrap(),
            "--chunk-size",
            &chunk_size.to_string(),
        ])
        .output()
        .expect("file send")
}

fn run_file_send_with_receipt(
    cfg: &Path,
    relay: &str,
    to: &str,
    path: &Path,
    chunk_size: usize,
) -> std::process::Output {
    qsc_base(cfg)
        .args([
            "file",
            "send",
            "--transport",
            "relay",
            "--relay",
            relay,
            "--to",
            to,
            "--path",
            path.to_str().unwrap(),
            "--chunk-size",
            &chunk_size.to_string(),
            "--receipt",
            "delivered",
        ])
        .output()
        .expect("file send")
}

fn assert_no_secrets(text: &str) {
    let upper = text.to_ascii_uppercase();
    for forbidden in [
        "TOKEN",
        "SECRET",
        "PASS",
        "PRIVATE",
        "BEARER",
        "CREDENTIAL",
        "RELAY_TOKEN",
    ] {
        assert!(
            !upper.contains(forbidden),
            "found forbidden pattern {} in output: {}",
            forbidden,
            text
        );
    }
}

fn leak_counts(text: &str) -> (usize, usize) {
    let v1 = text.matches("/v1/").count();
    let bytes = text.as_bytes();
    let mut long_hex = 0usize;
    let mut i = 0usize;
    while i < bytes.len() {
        let b = bytes[i];
        let is_hex = b.is_ascii_hexdigit();
        if !is_hex {
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
        .expect("contacts add route token");
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
        .expect("contacts device trust");
    assert!(trust.status.success(), "{}", output_text(&trust));
}

fn relay_set_inbox_token(cfg: &Path, token: &str) {
    let out = qsc_base(cfg)
        .args(["relay", "inbox-set", "--token", token])
        .output()
        .expect("relay inbox set");
    assert!(out.status.success(), "{}", output_text(&out));
}

#[test]
fn bounds_reject_file_too_large_no_mutation() {
    let _guard = file_transfer_test_guard();
    let server = common::start_inbox_server(1024 * 1024, 64);
    let base = safe_test_root().join(format!("na0119_bounds_{}", std::process::id()));
    create_dir_700(&base);
    let cfg = base.join("cfg");
    create_dir_700(&cfg);
    common::init_mock_vault(&cfg);

    let payload = base.join("big.bin");
    fs::write(&payload, vec![b'A'; 4096]).unwrap();
    let before = collect_files(&cfg);

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
            payload.to_str().unwrap(),
            "--max-file-size",
            "64",
        ])
        .output()
        .expect("file send oversize");
    assert!(!out.status.success(), "oversize send must fail");
    let text = output_text(&out);
    assert!(text.contains("event=file_xfer_reject"), "{}", text);
    assert!(text.contains("reason=size_exceeds_max"), "{}", text);
    let after = collect_files(&cfg);
    assert_eq!(before, after, "cfg mutated on bounds reject");
}

#[test]
fn file_transfer_accepts_valid_chunks_and_manifest() {
    let _guard = file_transfer_test_guard();
    let server = common::start_inbox_server(1024 * 1024, 128);
    let base = safe_test_root().join(format!("na0119_valid_{}", std::process::id()));
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

    let payload = base.join("file.bin");
    // Keep this multi-chunk to exercise chunk+manifest flow while avoiding
    // oversized mock-relay JSON responses that have been flaky on macOS CI.
    fs::write(&payload, vec![0x5a; 24_576]).unwrap();

    let send = run_file_send(&alice_cfg, server.base_url(), "bob", &payload, 8192);
    assert!(send.status.success(), "{}", output_text(&send));
    let send_text = output_text(&send);
    assert!(
        send_text.contains("event=file_xfer_prepare"),
        "{}",
        send_text
    );
    assert!(
        send_text.contains("event=file_xfer_manifest id="),
        "{}",
        send_text
    );
    assert!(
        send_text.contains("event=file_xfer_complete id="),
        "{}",
        send_text
    );

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
            "bob",
            "--max",
            "64",
            "--out",
            bob_out.to_str().unwrap(),
        ])
        .output()
        .expect("receive");
    assert!(recv.status.success(), "{}", output_text(&recv));
    let recv_text = output_text(&recv);
    assert!(recv_text.contains("event=file_xfer_chunk"), "{}", recv_text);
    assert!(
        recv_text.contains("event=file_xfer_manifest"),
        "{}",
        recv_text
    );
    assert!(
        recv_text.contains("event=file_xfer_complete"),
        "{}",
        recv_text
    );
    assert!(
        !bob_out.join("recv_1.bin").exists(),
        "file transfer payload must not be written plaintext to out dir"
    );

    let list = qsc_base(&bob_cfg)
        .args(["timeline", "list", "--peer", "bob", "--limit", "10"])
        .output()
        .expect("timeline list");
    assert!(list.status.success(), "{}", output_text(&list));
    let list_text = output_text(&list);
    assert!(
        list_text.contains("event=timeline_list count=1 peer=bob"),
        "{}",
        list_text
    );
    assert!(list_text.contains("kind=file"), "{}", list_text);
    assert!(list_text.contains("state=RECEIVED"), "{}", list_text);
}

#[test]
fn tampered_chunk_reject_no_mutation() {
    let _guard = file_transfer_test_guard();
    let server = common::start_inbox_server(1024 * 1024, 64);
    let base = safe_test_root().join(format!("na0119_tampered_chunk_{}", std::process::id()));
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

    let forged = base.join("forged_chunk.json");
    let bad = r#"{"v":1,"t":"file_chunk","file_id":"deadbeef01","filename":"x.bin","total_size":4,"chunk_index":0,"chunk_count":1,"chunk_hash":"0011","manifest_hash":"aa22","chunk":[1,2,3,4]}"#;
    fs::write(&forged, bad.as_bytes()).unwrap();

    let send = qsc_base(&alice_cfg)
        .args([
            "send",
            "--transport",
            "relay",
            "--relay",
            server.base_url(),
            "--to",
            "bob",
            "--file",
            forged.to_str().unwrap(),
        ])
        .output()
        .expect("send forged");
    assert!(send.status.success(), "{}", output_text(&send));

    let before = qsc_base(&bob_cfg)
        .args(["timeline", "list", "--peer", "bob", "--limit", "10"])
        .output()
        .expect("timeline before");
    assert!(output_text(&before).contains("event=timeline_list count=0 peer=bob"));
    let before_fs = collect_files(&bob_cfg);

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
            "bob",
            "--max",
            "1",
            "--out",
            bob_out.to_str().unwrap(),
        ])
        .output()
        .expect("recv forged");
    assert!(!recv.status.success(), "tampered chunk must fail");
    let text = output_text(&recv);
    assert!(text.contains("event=file_xfer_reject"), "{}", text);
    assert!(text.contains("reason=chunk_hash_invalid"), "{}", text);

    let after = qsc_base(&bob_cfg)
        .args(["timeline", "list", "--peer", "bob", "--limit", "10"])
        .output()
        .expect("timeline after");
    assert!(output_text(&after).contains("event=timeline_list count=0 peer=bob"));
    assert_eq!(before_fs, collect_files(&bob_cfg), "cfg mutated on reject");
}

#[test]
fn replay_chunk_reject_deterministic_no_mutation() {
    let _guard = file_transfer_test_guard();
    let server = common::start_inbox_server(1024 * 1024, 128);
    let base = safe_test_root().join(format!("na0119_replay_{}", std::process::id()));
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

    let good_chunk = [1u8, 2u8, 3u8, 4u8];
    let chunk_hash = "a7c976db1723adb41274178dc82e9b77";
    let forged = base.join("replay_chunk.json");
    let json = format!(
        "{{\"v\":1,\"t\":\"file_chunk\",\"file_id\":\"abcd1111\",\"filename\":\"x.bin\",\"total_size\":4,\"chunk_index\":0,\"chunk_count\":1,\"chunk_hash\":\"{}\",\"manifest_hash\":\"m1\",\"chunk\":[1,2,3,4]}}",
        chunk_hash
    );
    fs::write(&forged, json.as_bytes()).unwrap();
    assert_eq!(
        chunk_hash, "a7c976db1723adb41274178dc82e9b77",
        "hash fixture must stay deterministic"
    );
    assert_eq!(good_chunk.len(), 4);

    for _ in 0..2 {
        let send = qsc_base(&alice_cfg)
            .args([
                "send",
                "--transport",
                "relay",
                "--relay",
                server.base_url(),
                "--to",
                "bob",
                "--file",
                forged.to_str().unwrap(),
            ])
            .output()
            .expect("send replay chunk");
        assert!(send.status.success(), "{}", output_text(&send));
    }

    let first_recv = qsc_base(&bob_cfg)
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
            bob_out.to_str().unwrap(),
        ])
        .output()
        .expect("first receive");
    assert!(first_recv.status.success(), "{}", output_text(&first_recv));
    let before = collect_files(&bob_cfg);

    let second_recv = qsc_base(&bob_cfg)
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
            bob_out.to_str().unwrap(),
        ])
        .output()
        .expect("second receive");
    assert!(!second_recv.status.success(), "replayed chunk must fail");
    let text = output_text(&second_recv);
    assert!(text.contains("event=file_xfer_reject"), "{}", text);
    assert!(text.contains("reason=chunk_order_invalid"), "{}", text);
    assert_eq!(
        before,
        collect_files(&bob_cfg),
        "cfg mutated on replay reject"
    );
}

#[test]
fn no_plaintext_and_marker_determinism() {
    let _guard = file_transfer_test_guard();
    let base = safe_test_root().join(format!("na0119_determinism_{}", std::process::id()));
    create_dir_700(&base);
    let payload = base.join("det.bin");
    fs::write(&payload, vec![0x7c; 8192]).unwrap();

    let run_once = |name: &str| -> String {
        let server = common::start_inbox_server(1024 * 1024, 64);
        let cfg = base.join(name);
        create_dir_700(&cfg);
        common::init_mock_vault(&cfg);
        contacts_add_with_route_token(&cfg, "bob", ROUTE_TOKEN_BOB);
        let out = run_file_send(&cfg, server.base_url(), "bob", &payload, 2048);
        assert!(out.status.success(), "{}", output_text(&out));
        let text = output_text(&out);
        assert_no_secrets(&text);
        let vault_blob = cfg.join("vault.qsv");
        assert!(vault_blob.exists(), "vault blob missing");
        let bytes = fs::read(vault_blob).unwrap();
        assert!(
            !String::from_utf8_lossy(&bytes).contains("det.bin"),
            "plaintext filename leaked in vault blob"
        );
        text.lines()
            .filter(|l| l.contains("event=file_xfer_"))
            .collect::<Vec<_>>()
            .join("\n")
    };

    let a = run_once("cfg_a");
    let b = run_once("cfg_b");
    assert_eq!(a, b, "file transfer markers must be deterministic");
}

#[test]
fn file_relay_accepted_not_peer_confirmed_when_receipts_disabled() {
    let _guard = file_transfer_test_guard();
    let server = common::start_inbox_server(1024 * 1024, 256);
    let base = safe_test_root().join(format!("na0177_file_accept_only_{}", std::process::id()));
    create_dir_700(&base);
    let alice_cfg = base.join("alice_cfg");
    let bob_cfg = base.join("bob_cfg");
    let bob_out = base.join("bob_out");
    let alice_out = base.join("alice_out");
    create_dir_700(&alice_cfg);
    create_dir_700(&bob_cfg);
    create_dir_700(&bob_out);
    create_dir_700(&alice_out);
    common::init_mock_vault(&alice_cfg);
    common::init_mock_vault(&bob_cfg);
    contacts_add_with_route_token(&alice_cfg, "bob", ROUTE_TOKEN_BOB);
    relay_set_inbox_token(&bob_cfg, ROUTE_TOKEN_BOB);
    relay_set_inbox_token(&alice_cfg, ROUTE_TOKEN_BOB);
    let payload = base.join("file.bin");
    fs::write(&payload, vec![0x4f; 12_288]).unwrap();

    let send = run_file_send_with_receipt(&alice_cfg, server.base_url(), "bob", &payload, 4096);
    assert!(send.status.success(), "{}", output_text(&send));
    let send_text = output_text(&send);
    assert!(
        send_text.contains("QSC_FILE_DELIVERY state=accepted_by_relay"),
        "{}",
        send_text
    );
    assert!(
        send_text.contains("QSC_FILE_DELIVERY state=awaiting_confirmation"),
        "{}",
        send_text
    );
    assert!(send_text.contains(" peer=bob "), "{}", send_text);
    assert!(!send_text.contains("state=peer_confirmed"), "{}", send_text);

    let bob_recv = qsc_base(&bob_cfg)
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
            "64",
            "--out",
            bob_out.to_str().unwrap(),
        ])
        .output()
        .expect("bob receive");
    assert!(bob_recv.status.success(), "{}", output_text(&bob_recv));
    let bob_text = output_text(&bob_recv);
    assert!(bob_text.contains("event=receipt_disabled"), "{}", bob_text);
    assert!(
        !bob_text.contains("event=file_confirm_send "),
        "{}",
        bob_text
    );

    let alice_recv = qsc_base(&alice_cfg)
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
            "64",
            "--out",
            alice_out.to_str().unwrap(),
        ])
        .output()
        .expect("alice receive");
    assert!(alice_recv.status.success(), "{}", output_text(&alice_recv));
    let alice_text = output_text(&alice_recv);
    assert!(alice_text.contains("event=recv_none"), "{}", alice_text);
    assert!(
        !alice_text.contains("state=peer_confirmed"),
        "{}",
        alice_text
    );
    let mut all = String::new();
    all.push_str(&send_text);
    all.push_str(&bob_text);
    all.push_str(&alice_text);
    let (v1, long_hex) = leak_counts(&all);
    assert_eq!(v1, 0, "{}", all);
    assert_eq!(long_hex, 0, "{}", all);
}

#[test]
fn file_peer_confirmed_after_valid_completion_ack() {
    let _guard = file_transfer_test_guard();
    let server = common::start_inbox_server(1024 * 1024, 256);
    let base = safe_test_root().join(format!("na0177_file_confirmed_{}", std::process::id()));
    create_dir_700(&base);
    let alice_cfg = base.join("alice_cfg");
    let bob_cfg = base.join("bob_cfg");
    let bob_out = base.join("bob_out");
    let alice_out = base.join("alice_out");
    create_dir_700(&alice_cfg);
    create_dir_700(&bob_cfg);
    create_dir_700(&bob_out);
    create_dir_700(&alice_out);
    common::init_mock_vault(&alice_cfg);
    common::init_mock_vault(&bob_cfg);
    contacts_add_with_route_token(&alice_cfg, "bob", ROUTE_TOKEN_BOB);
    contacts_add_with_route_token(&bob_cfg, "bob", ROUTE_TOKEN_BOB);
    relay_set_inbox_token(&bob_cfg, ROUTE_TOKEN_BOB);
    relay_set_inbox_token(&alice_cfg, ROUTE_TOKEN_BOB);
    let payload = base.join("file.bin");
    fs::write(&payload, vec![0x51; 12_288]).unwrap();

    let send = run_file_send_with_receipt(&alice_cfg, server.base_url(), "bob", &payload, 4096);
    assert!(send.status.success(), "{}", output_text(&send));

    let bob_recv = qsc_base(&bob_cfg)
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
            "64",
            "--out",
            bob_out.to_str().unwrap(),
            "--emit-receipts",
            "delivered",
        ])
        .output()
        .expect("bob receive");
    assert!(bob_recv.status.success(), "{}", output_text(&bob_recv));
    let bob_text = output_text(&bob_recv);
    assert!(bob_text.contains("event=file_confirm_send"), "{}", bob_text);

    let alice_recv = qsc_base(&alice_cfg)
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
            "64",
            "--out",
            alice_out.to_str().unwrap(),
        ])
        .output()
        .expect("alice receive");
    assert!(alice_recv.status.success(), "{}", output_text(&alice_recv));
    let alice_text = output_text(&alice_recv);
    assert!(
        alice_text.contains("QSC_FILE_DELIVERY state=peer_confirmed"),
        "{}",
        alice_text
    );
    assert!(alice_text.contains(" peer=bob "), "{}", alice_text);
    assert!(
        alice_text.contains("event=file_confirm_recv"),
        "{}",
        alice_text
    );
}

#[test]
fn file_confirm_replay_rejected_no_mutation() {
    let _guard = file_transfer_test_guard();
    let server = common::start_inbox_server(1024 * 1024, 512);
    let base = safe_test_root().join(format!("na0177_file_confirm_replay_{}", std::process::id()));
    create_dir_700(&base);
    let alice_cfg = base.join("alice_cfg");
    let bob_cfg = base.join("bob_cfg");
    let bob_out = base.join("bob_out");
    let alice_out = base.join("alice_out");
    create_dir_700(&alice_cfg);
    create_dir_700(&bob_cfg);
    create_dir_700(&bob_out);
    create_dir_700(&alice_out);
    common::init_mock_vault(&alice_cfg);
    common::init_mock_vault(&bob_cfg);
    contacts_add_with_route_token(&alice_cfg, "bob", ROUTE_TOKEN_BOB);
    contacts_add_with_route_token(&bob_cfg, "bob", ROUTE_TOKEN_BOB);
    relay_set_inbox_token(&bob_cfg, ROUTE_TOKEN_BOB);
    relay_set_inbox_token(&alice_cfg, ROUTE_TOKEN_BOB);
    let payload = base.join("file.bin");
    fs::write(&payload, vec![0x63; 12_288]).unwrap();

    let send = run_file_send_with_receipt(&alice_cfg, server.base_url(), "bob", &payload, 4096);
    assert!(send.status.success(), "{}", output_text(&send));

    let bob_recv = qsc_base(&bob_cfg)
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
            "64",
            "--out",
            bob_out.to_str().unwrap(),
            "--emit-receipts",
            "delivered",
        ])
        .output()
        .expect("bob receive");
    assert!(bob_recv.status.success(), "{}", output_text(&bob_recv));
    let ack_frames = server.drain_channel(ROUTE_TOKEN_BOB);
    assert_eq!(
        ack_frames.len(),
        1,
        "expected exactly one file confirmation frame"
    );
    server.enqueue_raw(ROUTE_TOKEN_BOB, ack_frames[0].clone());
    server.enqueue_raw(ROUTE_TOKEN_BOB, ack_frames[0].clone());

    let first = qsc_base(&alice_cfg)
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
            alice_out.to_str().unwrap(),
        ])
        .output()
        .expect("alice receive first");
    assert!(first.status.success(), "{}", output_text(&first));
    let first_text = output_text(&first);
    assert!(
        first_text.contains("state=peer_confirmed"),
        "{}",
        first_text
    );

    let before = qsc_base(&alice_cfg)
        .args(["timeline", "list", "--peer", "bob", "--limit", "10"])
        .output()
        .expect("timeline list before replay");
    assert!(before.status.success(), "{}", output_text(&before));
    let before_text = output_text(&before);

    let second = qsc_base(&alice_cfg)
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
            alice_out.to_str().unwrap(),
        ])
        .output()
        .expect("alice receive replay");
    assert!(!second.status.success(), "{}", output_text(&second));
    let second_text = output_text(&second);
    assert!(
        second_text.contains("code=qsp_replay_reject"),
        "{}",
        second_text
    );

    let after = qsc_base(&alice_cfg)
        .args(["timeline", "list", "--peer", "bob", "--limit", "10"])
        .output()
        .expect("timeline list after replay");
    assert!(after.status.success(), "{}", output_text(&after));
    let after_text = output_text(&after);
    assert_eq!(
        before_text, after_text,
        "timeline mutated on replay confirmation"
    );
}

#[test]
fn file_confirm_unknown_id_rejected_no_mutation() {
    let _guard = file_transfer_test_guard();
    let server = common::start_inbox_server(1024 * 1024, 256);
    let base = safe_test_root().join(format!(
        "na0177_file_confirm_unknown_{}",
        std::process::id()
    ));
    create_dir_700(&base);
    let alice_cfg = base.join("alice_cfg");
    let bob_cfg = base.join("bob_cfg");
    let alice_out = base.join("alice_out");
    create_dir_700(&alice_cfg);
    create_dir_700(&bob_cfg);
    create_dir_700(&alice_out);
    common::init_mock_vault(&alice_cfg);
    common::init_mock_vault(&bob_cfg);
    contacts_add_with_route_token(&alice_cfg, "bob", ROUTE_TOKEN_BOB);
    contacts_add_with_route_token(&bob_cfg, "bob", ROUTE_TOKEN_BOB);
    relay_set_inbox_token(&alice_cfg, ROUTE_TOKEN_BOB);

    let forged = base.join("forged_file_confirm.json");
    fs::write(
        &forged,
        br#"{"v":1,"t":"ack","kind":"file_confirmed","file_id":"unknown01","confirm_id":"confirm0001"}"#,
    )
    .unwrap();
    let send = qsc_base(&bob_cfg)
        .args([
            "send",
            "--transport",
            "relay",
            "--relay",
            server.base_url(),
            "--to",
            "bob",
            "--file",
            forged.to_str().unwrap(),
        ])
        .output()
        .expect("send forged file confirm");
    assert!(send.status.success(), "{}", output_text(&send));

    let recv = qsc_base(&alice_cfg)
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
            "4",
            "--out",
            alice_out.to_str().unwrap(),
        ])
        .output()
        .expect("recv forged file confirm");
    assert!(recv.status.success(), "{}", output_text(&recv));
    let text = output_text(&recv);
    assert!(text.contains("event=file_confirm_reject"), "{}", text);
    assert!(text.contains("reason=state_unknown"), "{}", text);
    assert!(!text.contains("state=peer_confirmed"), "{}", text);
}
