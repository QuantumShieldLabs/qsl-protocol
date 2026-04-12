mod common;

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::{Mutex, MutexGuard, OnceLock};

const ROUTE_TOKEN_BOB: &str = "route_token_bob_na0192b_abcdefghijkl";

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
    let mut cmd = common::qsc_std_command();
    cmd.env("QSC_CONFIG_DIR", cfg)
        .env("QSC_MARK_FORMAT", "plain")
        .env("QSC_QSP_SEED", "1")
        .env("QSC_ALLOW_SEED_FALLBACK", "1");
    cmd
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

fn send_file(
    cfg: &Path,
    relay: &str,
    to: &str,
    path: &Path,
    chunk_size: &str,
    max_file_size: &str,
    max_chunks: &str,
) -> String {
    let out = qsc_base(cfg)
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
            chunk_size,
            "--max-file-size",
            max_file_size,
            "--max-chunks",
            max_chunks,
            "--receipt",
            "delivered",
        ])
        .output()
        .expect("file send");
    let text = output_text(&out);
    assert!(out.status.success(), "{}", text);
    assert_no_leaks(&text);
    text
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
    for _ in 0..128 {
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
                "1",
                "--max-file-size",
                max_file_size,
                "--max-file-chunks",
                max_file_chunks,
                "--out",
                out_dir.to_str().unwrap(),
                "--emit-receipts",
                "delivered",
                "--file-confirm-mode",
                "complete-only",
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
    assert_no_leaks(&combined);
    combined
}

fn receive_confirmation(
    cfg: &Path,
    relay: &str,
    mailbox: &str,
    from: &str,
    out_dir: &Path,
) -> std::process::Output {
    qsc_base(cfg)
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
            "1",
            "--max-file-size",
            "2000000",
            "--max-file-chunks",
            "80",
            "--out",
            out_dir.to_str().unwrap(),
        ])
        .output()
        .expect("receive confirmation")
}

#[test]
fn small_then_medium_confirmations_do_not_replay_reject() {
    let _guard = test_guard();
    let server = common::start_inbox_server(4 * 1024 * 1024, 512);
    let base = safe_test_root().join(format!("na0192b_confirm_chain_{}", std::process::id()));
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
    contacts_add_with_route_token(&alice_cfg, "bob", ROUTE_TOKEN_BOB);
    contacts_add_with_route_token(&bob_cfg, "bob", ROUTE_TOKEN_BOB);
    relay_set_inbox_token(&alice_cfg, ROUTE_TOKEN_BOB);
    relay_set_inbox_token(&bob_cfg, ROUTE_TOKEN_BOB);

    let small = base.join("small.bin");
    let medium = base.join("medium.bin");
    fs::write(&small, vec![0x41; 12_288]).expect("small payload");
    fs::write(&medium, vec![0x42; 49_152]).expect("medium payload");

    let small_send = send_file(
        &bob_cfg,
        server.base_url(),
        "bob",
        &small,
        "4096",
        "2000000",
        "80",
    );
    assert!(
        small_send.contains("event=file_xfer_complete"),
        "{}",
        small_send
    );

    let alice_small = receive_until_file_complete(
        &alice_cfg,
        server.base_url(),
        ROUTE_TOKEN_BOB,
        "bob",
        &alice_out,
        "2000000",
        "80",
    );
    assert!(
        alice_small.contains("event=file_confirm_send"),
        "{}",
        alice_small
    );

    let bob_small = receive_confirmation(
        &bob_cfg,
        server.base_url(),
        ROUTE_TOKEN_BOB,
        "bob",
        &bob_out,
    );
    let bob_small_text = output_text(&bob_small);
    assert!(bob_small.status.success(), "{}", bob_small_text);
    assert!(
        bob_small_text.contains("event=file_confirm_recv"),
        "{}",
        bob_small_text
    );
    assert!(
        !bob_small_text.contains("qsp_replay_reject"),
        "{}",
        bob_small_text
    );
    assert_no_leaks(&bob_small_text);

    let medium_send = send_file(
        &bob_cfg,
        server.base_url(),
        "bob",
        &medium,
        "16384",
        "2000000",
        "80",
    );
    assert!(
        medium_send.contains("event=file_xfer_complete"),
        "{}",
        medium_send
    );

    let alice_medium = receive_until_file_complete(
        &alice_cfg,
        server.base_url(),
        ROUTE_TOKEN_BOB,
        "bob",
        &alice_out,
        "2000000",
        "80",
    );
    assert!(
        alice_medium.contains("event=file_confirm_send"),
        "{}",
        alice_medium
    );
    assert!(
        !alice_medium.contains("qsp_replay_reject"),
        "{}",
        alice_medium
    );

    let bob_medium = receive_confirmation(
        &bob_cfg,
        server.base_url(),
        ROUTE_TOKEN_BOB,
        "bob",
        &bob_out,
    );
    let bob_medium_text = output_text(&bob_medium);
    assert!(bob_medium.status.success(), "{}", bob_medium_text);
    assert!(
        bob_medium_text.contains("event=file_confirm_recv"),
        "{}",
        bob_medium_text
    );
    assert!(
        !bob_medium_text.contains("qsp_replay_reject"),
        "{}",
        bob_medium_text
    );
    assert!(
        bob_medium_text.contains("QSC_FILE_DELIVERY state=peer_confirmed"),
        "{}",
        bob_medium_text
    );
    assert_no_leaks(&bob_medium_text);
}

#[test]
fn duplicate_confirmation_still_rejects_without_state_mutation() {
    let _guard = test_guard();
    let server = common::start_inbox_server(4 * 1024 * 1024, 512);
    let base = safe_test_root().join(format!("na0192b_confirm_replay_{}", std::process::id()));
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
    contacts_add_with_route_token(&alice_cfg, "bob", ROUTE_TOKEN_BOB);
    contacts_add_with_route_token(&bob_cfg, "bob", ROUTE_TOKEN_BOB);
    relay_set_inbox_token(&alice_cfg, ROUTE_TOKEN_BOB);
    relay_set_inbox_token(&bob_cfg, ROUTE_TOKEN_BOB);

    let payload = base.join("medium.bin");
    fs::write(&payload, vec![0x43; 49_152]).expect("payload");

    let send = send_file(
        &bob_cfg,
        server.base_url(),
        "bob",
        &payload,
        "16384",
        "2000000",
        "80",
    );
    assert!(send.contains("event=file_xfer_complete"), "{}", send);

    let alice_recv = receive_until_file_complete(
        &alice_cfg,
        server.base_url(),
        ROUTE_TOKEN_BOB,
        "bob",
        &alice_out,
        "2000000",
        "80",
    );
    assert!(
        alice_recv.contains("event=file_confirm_send"),
        "{}",
        alice_recv
    );

    let ack_frames = server.drain_channel(ROUTE_TOKEN_BOB);
    assert_eq!(
        ack_frames.len(),
        1,
        "expected exactly one confirmation frame"
    );
    server.enqueue_raw(ROUTE_TOKEN_BOB, ack_frames[0].clone());
    server.enqueue_raw(ROUTE_TOKEN_BOB, ack_frames[0].clone());

    let first = receive_confirmation(
        &bob_cfg,
        server.base_url(),
        ROUTE_TOKEN_BOB,
        "bob",
        &bob_out,
    );
    let first_text = output_text(&first);
    assert!(first.status.success(), "{}", first_text);
    assert!(
        first_text.contains("event=file_confirm_recv"),
        "{}",
        first_text
    );
    assert_no_leaks(&first_text);

    let before = qsc_base(&bob_cfg)
        .args(["timeline", "list", "--peer", "bob", "--limit", "10"])
        .output()
        .expect("timeline before replay");
    let before_text = output_text(&before);
    assert!(before.status.success(), "{}", before_text);

    let second = receive_confirmation(
        &bob_cfg,
        server.base_url(),
        ROUTE_TOKEN_BOB,
        "bob",
        &bob_out,
    );
    let second_text = output_text(&second);
    assert!(!second.status.success(), "{}", second_text);
    assert!(second_text.contains("qsp_replay_reject"), "{}", second_text);
    assert_no_leaks(&second_text);

    let after = qsc_base(&bob_cfg)
        .args(["timeline", "list", "--peer", "bob", "--limit", "10"])
        .output()
        .expect("timeline after replay");
    let after_text = output_text(&after);
    assert!(after.status.success(), "{}", after_text);
    assert_eq!(before_text, after_text, "timeline mutated on replay reject");
}
