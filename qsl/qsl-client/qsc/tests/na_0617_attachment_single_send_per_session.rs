// NA-0617 (ENG-0002): attachment single-send-per-session clarification + resend fix.
//
// Two distinct meanings of "session" are involved and MUST NOT be conflated:
//   L1 — the qsl-attachments SERVICE upload session: single-object by design
//        (create -> upload -> commit -> the session is consumed and destroyed, the
//        committed object persists). Reusing a consumed session fails closed with
//        REJECT_QATTSVC_SESSION_STATE. This is correct and unchanged.
//   L2 — the qsc CLIENT session (a config dir / identity used across `file send`
//        invocations): NOT limited to one attachment. Distinct sends each mint their
//        own L1 service session.
//
// Before NA-0617, a second `file send` of the SAME file (after it had committed and
// been accepted by the relay) reused the now-destroyed L1 session and surfaced a raw
// REJECT_QATTSVC_SESSION_STATE — a client footgun, not an L2 cap. NA-0617 excludes
// consumed-session states (COMMITTED / ACCEPTED_BY_RELAY / PEER_CONFIRMED) from record
// reuse so a re-send mints a fresh session, while preserving resume (SESSION_CREATED /
// UPLOADING) and in-flight blocking (AWAITING_CONFIRMATION).
//
// These tests drive the real qsc binary against the real qsl-attachments service run
// in-process by `common::start_attachment_server`, so they exercise the true L1 state
// machine end to end.

mod common;

use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::Command;
use std::sync::{Mutex, MutexGuard, OnceLock};

const ROUTE_TOKEN_BOB: &str = "route_token_bob_abcdefghijklmnopqr";

// Files must exceed the 4 MiB legacy ceiling to take the attachment/session path.
const LARGE_FILE_BYTES: usize = 5 * 1024 * 1024 + 8192;

fn attachment_test_guard() -> MutexGuard<'static, ()> {
    static TEST_GUARD: OnceLock<Mutex<()>> = OnceLock::new();
    TEST_GUARD
        .get_or_init(|| Mutex::new(()))
        .lock()
        .unwrap_or_else(|e| e.into_inner())
}

fn safe_test_root() -> std::path::PathBuf {
    let root = if let Ok(v) = std::env::var("QSC_TEST_ROOT") {
        std::path::PathBuf::from(v)
    } else if let Ok(v) = std::env::var("CARGO_TARGET_DIR") {
        std::path::PathBuf::from(v)
    } else {
        std::path::PathBuf::from("target")
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

fn write_repeated_file(path: &Path, size: usize, byte: u8) {
    let mut file = fs::File::create(path).unwrap();
    let chunk = vec![byte; 1024 * 1024];
    let mut remaining = size;
    while remaining > 0 {
        let step = remaining.min(chunk.len());
        file.write_all(&chunk[..step]).unwrap();
        remaining -= step;
    }
    file.sync_all().unwrap();
}

fn qsc_base(cfg: &Path) -> Command {
    let mut cmd = common::qsc_std_command();
    cmd.env("QSC_CONFIG_DIR", cfg)
        .env("QSC_MARK_FORMAT", "plain")
        .env("QSC_QSP_SEED", "1")
        .env("QSC_ALLOW_SEED_FALLBACK", "1")
        .env("QSC_UNSAFE_TEST_SEED_FALLBACK", "1");
    cmd
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

fn setup_alice(base: &Path) -> std::path::PathBuf {
    let alice_cfg = base.join("alice_cfg");
    create_dir_700(&alice_cfg);
    common::init_mock_vault(&alice_cfg);
    contacts_add_with_route_token(&alice_cfg, "bob", ROUTE_TOKEN_BOB);
    relay_set_inbox_token(&alice_cfg, ROUTE_TOKEN_BOB);
    alice_cfg
}

/// One `file send` invocation. `receipt` requests a delivery receipt (which leaves the
/// record AWAITING_CONFIRMATION); `abort_after` interrupts the upload after N parts
/// (leaving the record UPLOADING) to exercise the resume path.
fn run_file_send(
    cfg: &Path,
    relay: &str,
    service: &str,
    to: &str,
    path: &Path,
    receipt: bool,
    abort_after: Option<u32>,
) -> std::process::Output {
    let mut cmd = qsc_base(cfg);
    if let Some(n) = abort_after {
        cmd.env(
            "QSC_ATTACHMENT_TEST_ABORT_AFTER_UPLOAD_PARTS",
            n.to_string(),
        );
    }
    cmd.args([
        "file",
        "send",
        "--transport",
        "relay",
        "--relay",
        relay,
        "--attachment-service",
        service,
        "--to",
        to,
        "--path",
        path.to_str().unwrap(),
    ]);
    if receipt {
        cmd.args(["--receipt", "delivered"]);
    }
    cmd.output().expect("file send")
}

fn assert_send_committed(out: &std::process::Output) {
    let text = output_text(out);
    assert!(out.status.success(), "expected send success:\n{text}");
    assert!(
        text.contains("event=attachment_service_commit"),
        "expected service commit marker:\n{text}"
    );
    assert!(
        text.contains("QSC_FILE_DELIVERY state=accepted_by_relay"),
        "expected accepted_by_relay marker:\n{text}"
    );
    assert!(
        !text.contains("REJECT_QATTSVC_SESSION_STATE"),
        "unexpected session-state reject:\n{text}"
    );
}

// L2: multiple DISTINCT files sent within one qsc session (one config dir) each
// succeed, each via its own L1 service session. This is the headline property —
// "single-send-per-session" is NOT a client cap.
#[test]
fn distinct_files_multi_send_in_one_session_each_succeed() {
    let _guard = attachment_test_guard();
    let relay = common::start_inbox_server(2 * 1024 * 1024, 512);
    let service = common::start_attachment_server(100 * 1024 * 1024);
    let base = safe_test_root().join(format!("na0617_distinct_multi_{}", std::process::id()));
    create_dir_700(&base);
    let alice_cfg = setup_alice(&base);

    let file_a = base.join("alpha.bin");
    let file_b = base.join("bravo.bin");
    write_repeated_file(&file_a, LARGE_FILE_BYTES, 0x41);
    write_repeated_file(&file_b, LARGE_FILE_BYTES, 0x42);

    let send_a = run_file_send(
        &alice_cfg,
        relay.base_url(),
        service.base_url(),
        "bob",
        &file_a,
        false,
        None,
    );
    assert_send_committed(&send_a);

    let send_b = run_file_send(
        &alice_cfg,
        relay.base_url(),
        service.base_url(),
        "bob",
        &file_b,
        false,
        None,
    );
    assert_send_committed(&send_b);
}

// Footgun fixed: re-sending the SAME file after it has been delivered (ACCEPTED_BY_RELAY)
// now mints a fresh L1 session and succeeds, instead of reusing the consumed session and
// failing closed with REJECT_QATTSVC_SESSION_STATE.
#[test]
fn same_file_resend_after_delivery_starts_fresh_session() {
    let _guard = attachment_test_guard();
    let relay = common::start_inbox_server(2 * 1024 * 1024, 512);
    let service = common::start_attachment_server(100 * 1024 * 1024);
    let base = safe_test_root().join(format!("na0617_resend_{}", std::process::id()));
    create_dir_700(&base);
    let alice_cfg = setup_alice(&base);

    let file_a = base.join("photo.bin");
    write_repeated_file(&file_a, LARGE_FILE_BYTES, 0x41);

    let first = run_file_send(
        &alice_cfg,
        relay.base_url(),
        service.base_url(),
        "bob",
        &file_a,
        false,
        None,
    );
    assert_send_committed(&first);

    // Second send of the identical file in the same qsc session.
    let second = run_file_send(
        &alice_cfg,
        relay.base_url(),
        service.base_url(),
        "bob",
        &file_a,
        false,
        None,
    );
    assert_send_committed(&second);
}

// Resume preserved: an interrupted upload (record left UPLOADING, whose L1 session is
// still alive) is still reused on the next `file send`, which resumes and commits.
// Deep resume semantics (bad-resume-token rejection) are additionally guarded by
// attachment_streaming_na0197c::attachment_upload_resume_and_invalid_resume_token_fail_closed.
#[test]
fn interrupted_upload_resumes_and_commits() {
    let _guard = attachment_test_guard();
    let relay = common::start_inbox_server(2 * 1024 * 1024, 512);
    let service = common::start_attachment_server(100 * 1024 * 1024);
    let base = safe_test_root().join(format!("na0617_resume_{}", std::process::id()));
    create_dir_700(&base);
    let alice_cfg = setup_alice(&base);

    let file_a = base.join("doc.bin");
    write_repeated_file(&file_a, LARGE_FILE_BYTES, 0x43);

    let interrupted = run_file_send(
        &alice_cfg,
        relay.base_url(),
        service.base_url(),
        "bob",
        &file_a,
        false,
        Some(1),
    );
    assert!(
        !interrupted.status.success(),
        "expected interrupted upload to fail:\n{}",
        output_text(&interrupted)
    );

    let resumed = run_file_send(
        &alice_cfg,
        relay.base_url(),
        service.base_url(),
        "bob",
        &file_a,
        false,
        None,
    );
    assert_send_committed(&resumed);
}

// In-flight blocking preserved: while a receipt-tracked send is AWAITING_CONFIRMATION,
// re-sending the same file is refused with attachment_send_inflight (the record is still
// found and caught by the send-time guard, NOT excluded as a consumed session).
#[test]
fn same_file_resend_while_awaiting_confirmation_is_blocked_inflight() {
    let _guard = attachment_test_guard();
    let relay = common::start_inbox_server(2 * 1024 * 1024, 512);
    let service = common::start_attachment_server(100 * 1024 * 1024);
    let base = safe_test_root().join(format!("na0617_inflight_{}", std::process::id()));
    create_dir_700(&base);
    let alice_cfg = setup_alice(&base);

    let file_a = base.join("receipted.bin");
    write_repeated_file(&file_a, LARGE_FILE_BYTES, 0x44);

    let first = run_file_send(
        &alice_cfg,
        relay.base_url(),
        service.base_url(),
        "bob",
        &file_a,
        true,
        None,
    );
    assert!(
        first.status.success(),
        "expected receipt-tracked send to succeed:\n{}",
        output_text(&first)
    );

    let second = run_file_send(
        &alice_cfg,
        relay.base_url(),
        service.base_url(),
        "bob",
        &file_a,
        true,
        None,
    );
    let second_text = output_text(&second);
    assert!(
        second_text.contains("attachment_send_inflight"),
        "expected in-flight block on resend while awaiting confirmation:\n{second_text}"
    );
}
