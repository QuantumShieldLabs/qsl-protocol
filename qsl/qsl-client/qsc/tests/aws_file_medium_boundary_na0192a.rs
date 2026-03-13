mod common;

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::{Mutex, MutexGuard, OnceLock};

const ROUTE_TOKEN_BOB: &str = "route_token_bob_na0192a_abcdefghijkl";
const ROUTE_TOKEN_ALICE: &str = "route_token_alice_na0192a_abcdefghijk";

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

fn leak_counts(text: &str) -> (usize, usize, usize) {
    let v1 = text.matches("/v1/").count();
    let auth = text.matches("Authorization").count() + text.matches("Bearer").count();
    let token = text.matches("RELAY_TOKEN").count()
        + text.matches("ALICE_RELAY_BEARER_TOKEN").count()
        + text.matches("BOB_RELAY_BEARER_TOKEN").count()
        + text.matches("route_token").count();
    (v1, auth, token)
}

fn assert_no_leaks(text: &str) {
    let (v1, auth, token) = leak_counts(text);
    assert_eq!(v1, 0, "v1 path leak in output: {}", text);
    assert_eq!(auth, 0, "Authorization/Bearer leak in output: {}", text);
    assert_eq!(token, 0, "token literal leak in output: {}", text);
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

fn receive_until_file_complete(
    cfg: &Path,
    relay: &str,
    mailbox: &str,
    from: &str,
    out_dir: &Path,
    max_file_size: &str,
    max_file_chunks: &str,
) -> String {
    let mut combined = String::new();
    for _ in 0..8 {
        let recv = qsc_base(cfg)
            .args([
                "receive",
                "--transport",
                "relay",
                "--relay",
                relay,
                "--mailbox",
                mailbox,
                "--from",
                from,
                "--max",
                "16",
                "--max-file-size",
                max_file_size,
                "--max-file-chunks",
                max_file_chunks,
                "--out",
                out_dir.to_str().unwrap(),
            ])
            .output()
            .expect("receive");
        let recv_text = output_text(&recv);
        assert!(recv.status.success(), "{}", recv_text);
        combined.push_str(&recv_text);
        if recv_text.contains("event=file_xfer_complete") {
            break;
        }
    }
    combined
}

#[test]
fn chunk_size_32768_rejected_fail_closed_no_mutation() {
    let _guard = test_guard();
    let server = common::start_inbox_server(1024 * 1024, 128);
    let base = safe_test_root().join(format!("na0192a_chunk_reject_{}", std::process::id()));
    create_dir_700(&base);
    let cfg = base.join("cfg");
    create_dir_700(&cfg);
    common::init_mock_vault(&cfg);
    contacts_add_with_route_token(&cfg, "bob", ROUTE_TOKEN_BOB);

    let payload = base.join("medium.bin");
    fs::write(&payload, vec![0x5a; 1_200_000]).expect("payload");
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
            "--chunk-size",
            "32768",
            "--max-file-size",
            "2000000",
            "--max-chunks",
            "80",
            "--receipt",
            "delivered",
        ])
        .output()
        .expect("file send");
    let text = output_text(&out);
    assert!(
        !out.status.success(),
        "oversize chunk size must fail closed"
    );
    assert!(text.contains("event=file_xfer_reject"), "{}", text);
    assert!(
        text.contains("reason=file_xfer_chunk_bound_invalid"),
        "{}",
        text
    );
    let after = collect_files(&cfg);
    assert_eq!(before, after, "cfg mutated on chunk bound reject");
    assert_no_leaks(&text);
}

#[test]
fn supported_16384_chunk_roundtrip_still_succeeds() {
    let _guard = test_guard();
    let server = common::start_inbox_server(2 * 1024 * 1024, 256);
    let base = safe_test_root().join(format!("na0192a_chunk_ok_{}", std::process::id()));
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
    fs::write(&payload, vec![0x41; 49_152]).expect("payload");

    let send = qsc_base(&alice_cfg)
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
            "--chunk-size",
            "16384",
            "--receipt",
            "delivered",
        ])
        .output()
        .expect("file send");
    let send_text = output_text(&send);
    assert!(send.status.success(), "{}", send_text);
    assert!(
        send_text.contains("event=file_xfer_manifest"),
        "{}",
        send_text
    );
    assert!(
        send_text.contains("event=file_xfer_complete"),
        "{}",
        send_text
    );
    assert_no_leaks(&send_text);

    let recv_text = receive_until_file_complete(
        &bob_cfg,
        server.base_url(),
        ROUTE_TOKEN_BOB,
        "bob",
        &bob_out,
        "2000000",
        "80",
    );
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
        !recv_text.contains("QSC_FILE_INTEGRITY_FAIL"),
        "{}",
        recv_text
    );
    assert_no_leaks(&recv_text);
}

#[test]
fn medium_16384_roundtrip_succeeds_with_explicit_receive_bounds() {
    let _guard = test_guard();
    let server = common::start_inbox_server(4 * 1024 * 1024, 512);
    let base = safe_test_root().join(format!("na0192a_medium_ok_{}", std::process::id()));
    create_dir_700(&base);
    let alice_cfg = base.join("alice_cfg");
    let bob_cfg = base.join("bob_cfg");
    let alice_out = base.join("alice_out");
    create_dir_700(&alice_cfg);
    create_dir_700(&bob_cfg);
    create_dir_700(&alice_out);
    common::init_mock_vault(&alice_cfg);
    common::init_mock_vault(&bob_cfg);
    contacts_add_with_route_token(&bob_cfg, "alice", ROUTE_TOKEN_ALICE);
    relay_set_inbox_token(&alice_cfg, ROUTE_TOKEN_ALICE);

    let payload = base.join("medium.bin");
    fs::write(&payload, vec![0x42; 1_200_000]).expect("payload");

    let send = qsc_base(&bob_cfg)
        .args([
            "file",
            "send",
            "--transport",
            "relay",
            "--relay",
            server.base_url(),
            "--to",
            "alice",
            "--path",
            payload.to_str().unwrap(),
            "--chunk-size",
            "16384",
            "--max-file-size",
            "2000000",
            "--max-chunks",
            "80",
            "--receipt",
            "delivered",
        ])
        .output()
        .expect("file send");
    let send_text = output_text(&send);
    assert!(send.status.success(), "{}", send_text);
    assert!(
        send_text.contains("event=file_xfer_complete"),
        "{}",
        send_text
    );
    assert_no_leaks(&send_text);

    let recv_text = receive_until_file_complete(
        &alice_cfg,
        server.base_url(),
        ROUTE_TOKEN_ALICE,
        "alice",
        &alice_out,
        "2000000",
        "80",
    );
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
        !recv_text.contains("QSC_FILE_INTEGRITY_FAIL"),
        "{}",
        recv_text
    );
    assert!(
        !recv_text.contains("reason=size_exceeds_max"),
        "{}",
        recv_text
    );
    assert!(
        !recv_text.contains("reason=chunk_count_exceeds_max"),
        "{}",
        recv_text
    );
    assert_no_leaks(&recv_text);
}
