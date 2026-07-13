// NA-0640 (D576): the FIRST full-stack e2e proof — two isolated qsc clients, the REAL
// qsl-server relay (in-process at the pinned rev), and the REAL qsl-attachments service
// (in-process), together, with zero product-source change.
//
// Every prior "integration" test swaps in a mock for at least one leg: na0182 uses the
// embedded `qsc relay serve`, na0197c/na0617 use the test-local inbox in common. Here
// the relay leg is qsl-server's actual router (/v1/push, /v1/pull, x-qsl-route-token,
// optional bearer auth) and the attachment leg is qsl-attachments' actual upload-session
// state machine, at the same time.
//
// Coverage (stated limits — see docs/governance/evidence/NA-0640_as_built.md):
// - message round-trip A -> real relay -> B with plaintext byte-match (recv_1.bin) and
//   the receipt round-trip back to A (peer_confirmed);
// - >4 MiB attachment round-trip on the REAL attachment path (QSC_ATTACHMENT_SERVICE
//   set; upload sessions on the real service; descriptor through the real relay;
//   download + byte-verify);
// - auth modes: OPEN relay (message + attachment) and BEARER-TOKEN relay (message,
//   plus a wrong-token negative proving the server actually enforces).
//
// D576 boundary: if any of this only passes by editing product code, that is a REAL
// integration bug — file an ENG and STOP; never patch product code to green this test.

mod common;

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::{Mutex, MutexGuard, OnceLock};

const ALICE_MAILBOX: &str = "na0640_alice_mailbox_token_abcd12";
const BOB_MAILBOX: &str = "na0640_bob_mailbox_token_wxyz567";
const RELAY_BEARER: &str = "na0640_relay_bearer_token_0123456789abcdef";
const WRONG_BEARER: &str = "na0640_wrong_bearer_token_fedcba9876543210";

// Strictly above the 4 MiB legacy ceiling so the send takes the real attachment path.
const ATTACHMENT_BYTES: usize = 6 * 1024 * 1024 + 321;

fn e2e_guard() -> MutexGuard<'static, ()> {
    static TEST_GUARD: OnceLock<Mutex<()>> = OnceLock::new();
    TEST_GUARD
        .get_or_init(|| Mutex::new(()))
        .lock()
        .unwrap_or_else(|e| e.into_inner())
}

fn output_text(out: &std::process::Output) -> String {
    let mut s = String::from_utf8_lossy(&out.stdout).to_string();
    s.push_str(&String::from_utf8_lossy(&out.stderr));
    s
}

fn assert_no_leaks(text: &str) {
    assert_eq!(
        text.matches("/v1/").count(),
        0,
        "found v1 path marker in output: {text}"
    );
}

fn ensure_dir_700(path: &Path) {
    fs::create_dir_all(path).expect("create dir");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(path, fs::Permissions::from_mode(0o700)).expect("chmod 700");
    }
}

fn create_dir_700(path: &Path) {
    let _ = fs::remove_dir_all(path);
    ensure_dir_700(path);
}

fn safe_test_root(tag: &str) -> PathBuf {
    let root = if let Ok(v) = std::env::var("QSC_TEST_ROOT") {
        PathBuf::from(v)
    } else if let Ok(v) = std::env::var("CARGO_TARGET_DIR") {
        PathBuf::from(v)
    } else {
        PathBuf::from("target")
    };
    let root = root.join("qsc-test-tmp").join(tag);
    create_dir_700(&root);
    root
}

fn qsc_cmd(cfg: &Path, bearer: Option<&str>) -> Command {
    let mut cmd = common::qsc_std_command();
    cmd.env("QSC_CONFIG_DIR", cfg)
        .env("QSC_QSP_SEED", "1")
        .env("QSC_ALLOW_SEED_FALLBACK", "1")
        .env("QSC_UNSAFE_TEST_SEED_FALLBACK", "1")
        .env("QSC_MARK_FORMAT", "plain");
    if let Some(token) = bearer {
        cmd.env("QSC_RELAY_TOKEN", token);
    }
    cmd
}

fn run_ok(cfg: &Path, bearer: Option<&str>, args: &[&str]) -> String {
    let out = qsc_cmd(cfg, bearer).args(args).output().expect("run qsc");
    let text = output_text(&out);
    assert!(out.status.success(), "command failed: {:?}\n{}", args, text);
    text
}

fn init_identity(cfg: &Path, bearer: Option<&str>) -> String {
    common::init_mock_vault(cfg);
    run_ok(cfg, bearer, &["identity", "rotate", "--confirm"]);
    let show = run_ok(cfg, bearer, &["identity", "show"]);
    show.lines()
        .find_map(|line| line.strip_prefix("identity_fp="))
        .unwrap_or_else(|| panic!("missing identity_fp output: {show}"))
        .to_string()
}

fn add_contact(cfg: &Path, bearer: Option<&str>, label: &str, fp: &str, route_token: &str) {
    run_ok(
        cfg,
        bearer,
        &[
            "contacts",
            "add",
            "--label",
            label,
            "--fp",
            fp,
            "--route-token",
            route_token,
        ],
    );
    let list = run_ok(cfg, bearer, &["contacts", "device", "list", "--label", label]);
    let device = list
        .lines()
        .find_map(|line| {
            line.split_whitespace()
                .find_map(|tok| tok.strip_prefix("device="))
        })
        .unwrap_or_else(|| panic!("missing device output: {list}"));
    run_ok(
        cfg,
        bearer,
        &[
            "contacts",
            "device",
            "trust",
            "--label",
            label,
            "--device",
            device,
            "--confirm",
        ],
    );
}

// Contact labels are aligned ("bob" on both sides) so receive --from matches the local
// peer label on both clients — the na0182 convention.
fn setup_pair(base: &Path, bearer: Option<&str>) -> (PathBuf, PathBuf, PathBuf, PathBuf) {
    let alice_cfg = base.join("alice_cfg");
    let bob_cfg = base.join("bob_cfg");
    let alice_out = base.join("alice_out");
    let bob_out = base.join("bob_out");
    create_dir_700(&alice_cfg);
    create_dir_700(&bob_cfg);
    create_dir_700(&alice_out);
    create_dir_700(&bob_out);

    let alice_fp = init_identity(&alice_cfg, bearer);
    let bob_fp = init_identity(&bob_cfg, bearer);

    run_ok(
        &alice_cfg,
        bearer,
        &["relay", "inbox-set", "--token", ALICE_MAILBOX],
    );
    run_ok(
        &bob_cfg,
        bearer,
        &["relay", "inbox-set", "--token", BOB_MAILBOX],
    );

    add_contact(&alice_cfg, bearer, "bob", bob_fp.as_str(), BOB_MAILBOX);
    add_contact(&bob_cfg, bearer, "bob", alice_fp.as_str(), ALICE_MAILBOX);

    (alice_cfg, bob_cfg, alice_out, bob_out)
}

fn message_round_trip(
    relay_url: &str,
    bearer: Option<&str>,
    base: &Path,
    alice_cfg: &Path,
    bob_cfg: &Path,
    alice_out: &Path,
    bob_out: &Path,
) {
    let msg_bytes: &[u8] = b"na0640 full-stack message through the real qsl-server";
    let msg = base.join("msg.txt");
    fs::write(&msg, msg_bytes).expect("write msg");

    let send_msg = run_ok(
        alice_cfg,
        bearer,
        &[
            "send",
            "--transport",
            "relay",
            "--relay",
            relay_url,
            "--to",
            "bob",
            "--file",
            msg.to_str().expect("msg path"),
            "--receipt",
            "delivered",
        ],
    );
    assert!(
        send_msg.contains("QSC_DELIVERY state=accepted_by_relay"),
        "{}",
        send_msg
    );

    let bob_recv = run_ok(
        bob_cfg,
        bearer,
        &[
            "receive",
            "--transport",
            "relay",
            "--relay",
            relay_url,
            "--mailbox",
            BOB_MAILBOX,
            "--from",
            "bob",
            "--max",
            "4",
            "--out",
            bob_out.to_str().expect("bob out"),
            "--emit-receipts",
            "delivered",
            "--receipt-mode",
            "immediate",
        ],
    );
    assert!(
        bob_recv.contains("QSC_RECEIPT mode=immediate status=sent kind=message peer=bob"),
        "{}",
        bob_recv
    );
    // The plaintext match: the received message body is byte-identical to what A sent.
    let received = fs::read(bob_out.join("recv_1.bin")).expect("read received message");
    assert_eq!(received, msg_bytes, "received plaintext differs from sent");

    let alice_recv = run_ok(
        alice_cfg,
        bearer,
        &[
            "receive",
            "--transport",
            "relay",
            "--relay",
            relay_url,
            "--mailbox",
            ALICE_MAILBOX,
            "--from",
            "bob",
            "--max",
            "4",
            "--out",
            alice_out.to_str().expect("alice out"),
        ],
    );
    assert!(
        alice_recv.contains("QSC_DELIVERY state=peer_confirmed"),
        "{}",
        alice_recv
    );

    assert_no_leaks(&send_msg);
    assert_no_leaks(&bob_recv);
    assert_no_leaks(&alice_recv);
}

#[test]
fn full_stack_message_and_attachment_round_trip_open_relay() {
    let _guard = e2e_guard();
    let relay = common::start_qsl_server(2 * 1024 * 1024, 512, None);
    let service = common::start_attachment_server(100 * 1024 * 1024);
    let base = safe_test_root("na0640_open_relay");
    let (alice_cfg, bob_cfg, alice_out, bob_out) = setup_pair(&base, None);

    message_round_trip(
        relay.base_url(),
        None,
        &base,
        &alice_cfg,
        &bob_cfg,
        &alice_out,
        &bob_out,
    );

    // Attachment round-trip on the REAL attachment path: >4 MiB payload, real upload
    // sessions, descriptor through the real relay, download + byte-verify on B.
    let payload_bytes = vec![0x5a_u8; ATTACHMENT_BYTES];
    let payload = base.join("na0640_attachment.bin");
    fs::write(&payload, &payload_bytes).expect("write attachment payload");

    let send_file = {
        let out = qsc_cmd(&alice_cfg, None)
            .env("QSC_ATTACHMENT_SERVICE", service.base_url())
            .args([
                "file",
                "send",
                "--transport",
                "relay",
                "--relay",
                relay.base_url(),
                "--attachment-service",
                service.base_url(),
                "--to",
                "bob",
                "--path",
                payload.to_str().expect("payload path"),
                "--receipt",
                "delivered",
            ])
            .output()
            .expect("attachment send");
        let text = output_text(&out);
        assert!(out.status.success(), "attachment send failed: {text}");
        text
    };
    assert!(
        send_file.contains("QSC_FILE_DELIVERY state=accepted_by_relay"),
        "{}",
        send_file
    );
    assert!(
        send_file.contains("QSC_FILE_DELIVERY state=awaiting_confirmation"),
        "{}",
        send_file
    );

    let bob_recv_file = {
        let out = qsc_cmd(&bob_cfg, None)
            .env("QSC_ATTACHMENT_SERVICE", service.base_url())
            .args([
                "receive",
                "--transport",
                "relay",
                "--relay",
                relay.base_url(),
                "--mailbox",
                BOB_MAILBOX,
                "--from",
                "bob",
                "--max",
                "64",
                "--max-file-size",
                "16777216",
                "--max-file-chunks",
                "512",
                "--out",
                bob_out.to_str().expect("bob out"),
                "--attachment-service",
                service.base_url(),
                "--emit-receipts",
                "delivered",
            ])
            .output()
            .expect("attachment receive");
        let text = output_text(&out);
        assert!(out.status.success(), "attachment receive failed: {text}");
        text
    };
    // The byte-verification: B's downloaded file is identical to A's payload.
    let downloaded =
        fs::read(bob_out.join("na0640_attachment.bin")).expect("read downloaded attachment");
    assert_eq!(
        downloaded, payload_bytes,
        "downloaded attachment differs from uploaded payload"
    );

    let alice_confirm = run_ok(
        &alice_cfg,
        None,
        &[
            "receive",
            "--transport",
            "relay",
            "--relay",
            relay.base_url(),
            "--mailbox",
            ALICE_MAILBOX,
            "--from",
            "bob",
            "--max",
            "8",
            "--out",
            alice_out.to_str().expect("alice out"),
        ],
    );
    assert!(
        alice_confirm.contains("QSC_FILE_DELIVERY state=peer_confirmed"),
        "{}",
        alice_confirm
    );

    assert_no_leaks(&send_file);
    assert_no_leaks(&bob_recv_file);
    assert_no_leaks(&alice_confirm);
}

#[test]
fn full_stack_message_round_trip_token_auth_relay() {
    let _guard = e2e_guard();
    let relay = common::start_qsl_server(2 * 1024 * 1024, 512, Some(RELAY_BEARER));
    let base = safe_test_root("na0640_token_relay");
    let (alice_cfg, bob_cfg, alice_out, bob_out) = setup_pair(&base, Some(RELAY_BEARER));

    message_round_trip(
        relay.base_url(),
        Some(RELAY_BEARER),
        &base,
        &alice_cfg,
        &bob_cfg,
        &alice_out,
        &bob_out,
    );

    // Negative: a client with the WRONG bearer is rejected by the real server —
    // isolated third config dir so the failed attempt cannot poison pair state.
    let mallory_cfg = base.join("mallory_cfg");
    create_dir_700(&mallory_cfg);
    let mallory_fp = init_identity(&mallory_cfg, Some(WRONG_BEARER));
    let _ = mallory_fp;
    let bob_fp_for_mallory = {
        let show = run_ok(&bob_cfg, Some(RELAY_BEARER), &["identity", "show"]);
        show.lines()
            .find_map(|line| line.strip_prefix("identity_fp="))
            .expect("bob fp")
            .to_string()
    };
    add_contact(
        &mallory_cfg,
        Some(WRONG_BEARER),
        "bob",
        bob_fp_for_mallory.as_str(),
        BOB_MAILBOX,
    );
    let msg = base.join("mallory_msg.txt");
    fs::write(&msg, b"should never be accepted").expect("write mallory msg");
    let denied = qsc_cmd(&mallory_cfg, Some(WRONG_BEARER))
        .args([
            "send",
            "--transport",
            "relay",
            "--relay",
            relay.base_url(),
            "--to",
            "bob",
            "--file",
            msg.to_str().expect("mallory msg path"),
        ])
        .output()
        .expect("mallory send");
    let denied_text = output_text(&denied);
    assert!(
        !denied.status.success(),
        "wrong bearer token must be rejected by the real relay: {denied_text}"
    );
    assert!(
        !denied_text.contains("state=accepted_by_relay"),
        "wrong-bearer send must not reach accepted_by_relay: {denied_text}"
    );
}
