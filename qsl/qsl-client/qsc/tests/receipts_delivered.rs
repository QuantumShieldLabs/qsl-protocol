mod common;

use quantumshield_refimpl::qse::EnvelopeProfile;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

const ROUTE_TOKEN_BOB: &str = "route_token_bob_abcdefghijklmnopqr";

fn safe_test_root() -> PathBuf {
    let root = if let Ok(v) = env::var("QSC_TEST_ROOT") {
        PathBuf::from(v)
    } else if let Ok(v) = env::var("CARGO_TARGET_DIR") {
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

fn combined_output(output: &std::process::Output) -> String {
    let mut s = String::from_utf8_lossy(&output.stdout).to_string();
    s.push_str(&String::from_utf8_lossy(&output.stderr));
    s
}

fn assert_no_secrets(s: &str) {
    for needle in [
        "TOKEN",
        "SECRET",
        "KEY",
        "PASS",
        "PRIVATE",
        "BEARER",
        "CREDENTIAL",
        "RELAY_TOKEN",
    ] {
        assert!(!s.contains(needle), "secret token leaked: {needle}");
    }
}

fn contacts_route_set(cfg: &Path, label: &str, token: &str) {
    let out = common::qsc_std_command()
        .env("QSC_CONFIG_DIR", cfg)
        .args([
            "contacts",
            "add",
            "--label",
            label,
            "--fp",
            "fp-pinned-test",
            "--route-token",
            token,
        ])
        .output()
        .expect("contacts add pinned");
    assert!(out.status.success(), "{}", combined_output(&out));
}

fn send_msg(cfg: &Path, relay: &str, to: &str, file: &Path, with_receipt: bool) -> String {
    let mut cmd = common::qsc_std_command();
    cmd.env("QSC_CONFIG_DIR", cfg)
        .env("QSC_QSP_SEED", "1")
        .env("QSC_ALLOW_SEED_FALLBACK", "1")
        .env("QSC_UNSAFE_TEST_SEED_FALLBACK", "1")
        .env("QSC_MARK_FORMAT", "plain")
        .args([
            "send",
            "--transport",
            "relay",
            "--relay",
            relay,
            "--to",
            to,
            "--file",
            file.to_str().unwrap(),
        ]);
    if with_receipt {
        cmd.args(["--receipt", "delivered"]);
    }
    let out = cmd.output().expect("send output");
    assert!(
        out.status.success(),
        "send failed: {}",
        combined_output(&out)
    );
    combined_output(&out)
}

fn recv_msg(
    cfg: &Path,
    relay: &str,
    mailbox: &str,
    from: &str,
    out_dir: &Path,
    emit_receipts: bool,
) -> String {
    recv_msg_with_mode(cfg, relay, mailbox, from, out_dir, emit_receipts, None)
}

/// NA-0682: the same receive, with an explicit `--receipt-mode`, so a test can exercise
/// F6's opt-out knob rather than merely observing whatever the default happens to be.
fn recv_msg_with_mode(
    cfg: &Path,
    relay: &str,
    mailbox: &str,
    from: &str,
    out_dir: &Path,
    emit_receipts: bool,
    receipt_mode: Option<&str>,
) -> String {
    let mut cmd = common::qsc_std_command();
    cmd.env("QSC_CONFIG_DIR", cfg)
        .env("QSC_QSP_SEED", "1")
        .env("QSC_ALLOW_SEED_FALLBACK", "1")
        .env("QSC_UNSAFE_TEST_SEED_FALLBACK", "1")
        .env("QSC_MARK_FORMAT", "plain")
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
            "--out",
            out_dir.to_str().unwrap(),
        ]);
    if emit_receipts {
        cmd.args(["--emit-receipts", "delivered"]);
    }
    if let Some(m) = receipt_mode {
        cmd.args(["--receipt-mode", m]);
    }
    let out = cmd.output().expect("receive output");
    assert!(
        out.status.success(),
        "receive failed: {}",
        combined_output(&out)
    );
    combined_output(&out)
}

#[test]
fn receipt_mode_off_suppresses_an_ack_the_sender_actually_requested() {
    let server = common::start_inbox_server(1024 * 1024, 32);
    let base = safe_test_root().join(format!("na0113_off_{}", std::process::id()));
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
    contacts_route_set(&alice_cfg, "bob", ROUTE_TOKEN_BOB);
    contacts_route_set(&bob_cfg, "bob", ROUTE_TOKEN_BOB);

    // (1) The SENDER genuinely asks for a delivery ack, so an ack is possible here.
    let msg = base.join("msg.bin");
    fs::write(&msg, b"hello-bob-receipt-requested").unwrap();
    let send_out = send_msg(&alice_cfg, server.base_url(), "bob", &msg, true);
    assert!(send_out.contains("event=receipt_request kind=delivered msg_id=<redacted>"));

    // (2) THE PROPERTY: the RECIPIENT's knob says off, so no ack is emitted -- even though
    // the sender asked for one and the recipient was also handed `--emit-receipts`.
    // The recipient's own policy is authoritative; that is what F6 requirement (a) means.
    let bob_recv = recv_msg_with_mode(
        &bob_cfg,
        server.base_url(),
        ROUTE_TOKEN_BOB,
        "bob",
        &bob_out,
        true,
        Some("off"),
    );
    assert!(
        !bob_recv.contains("event=receipt_send"),
        "--receipt-mode off must suppress the ack entirely: {bob_recv}"
    );

    // (3) ⚠ NON-VACUITY, and it is the whole reason this test was rewritten.
    //
    // NA-0682 deferred F6, so delivery acks are OFF BY DEFAULT again. The previous form of
    // this test sent WITHOUT requesting a receipt and then observed that no ack appeared --
    // which, once the default went back to off, would have passed with the off knob
    // completely broken. A negative result is only evidence if the instrument COULD have
    // returned positive, so the same fixture must be shown to produce an ack when the knob
    // is not off.
    let msg2 = base.join("msg2.bin");
    fs::write(&msg2, b"hello-bob-again").unwrap();
    let send2 = send_msg(&alice_cfg, server.base_url(), "bob", &msg2, true);
    assert!(send2.contains("event=receipt_request kind=delivered msg_id=<redacted>"));
    let bob_recv2 = recv_msg(
        &bob_cfg,
        server.base_url(),
        ROUTE_TOKEN_BOB,
        "bob",
        &bob_out,
        true,
    );
    assert!(
        bob_recv2.contains("event=receipt_send"),
        "the fixture must be able to produce an ack, or step (2) proves nothing: {bob_recv2}"
    );
}

#[test]
fn delivered_receipt_roundtrip() {
    let server = common::start_inbox_server(1024 * 1024, 32);
    let base = safe_test_root().join(format!("na0113_roundtrip_{}", std::process::id()));
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
    contacts_route_set(&alice_cfg, "bob", ROUTE_TOKEN_BOB);
    contacts_route_set(&bob_cfg, "bob", ROUTE_TOKEN_BOB);
    let msg = base.join("msg.bin");
    fs::write(&msg, b"hello-bob-with-receipt").unwrap();

    let send_out = send_msg(&alice_cfg, server.base_url(), "bob", &msg, true);
    assert!(send_out.contains("event=receipt_request kind=delivered msg_id=<redacted>"));

    let bob_recv = recv_msg(
        &bob_cfg,
        server.base_url(),
        ROUTE_TOKEN_BOB,
        "bob",
        &bob_out,
        true,
    );
    assert!(bob_recv.contains("event=receipt_send kind=delivered bucket=small msg_id=<redacted>"));

    let alice_recv = recv_msg(
        &alice_cfg,
        server.base_url(),
        ROUTE_TOKEN_BOB,
        "bob",
        &alice_out,
        false,
    );
    assert!(alice_recv.contains("event=receipt_recv kind=delivered msg_id=<redacted>"));
    assert!(alice_recv.contains("event=delivered_to_peer kind=delivered msg_id=<redacted>"));
    assert!(!alice_out.join("recv_1.bin").exists());
}

#[test]
fn ack_camouflage_small_bucket() {
    let server = common::start_inbox_server(1024 * 1024, 32);
    let base = safe_test_root().join(format!("na0113_camouflage_{}", std::process::id()));
    create_dir_700(&base);
    let alice_cfg = base.join("alice_cfg");
    let bob_cfg = base.join("bob_cfg");
    let bob_out = base.join("bob_out");
    create_dir_700(&alice_cfg);
    create_dir_700(&bob_cfg);
    create_dir_700(&bob_out);
    common::init_mock_vault(&alice_cfg);
    common::init_mock_vault(&bob_cfg);
    contacts_route_set(&alice_cfg, "bob", ROUTE_TOKEN_BOB);
    contacts_route_set(&bob_cfg, "bob", ROUTE_TOKEN_BOB);
    let msg = base.join("msg.bin");
    fs::write(&msg, b"hello-camo").unwrap();

    let _ = send_msg(&alice_cfg, server.base_url(), "bob", &msg, true);
    let bob_recv = recv_msg(
        &bob_cfg,
        server.base_url(),
        ROUTE_TOKEN_BOB,
        "bob",
        &bob_out,
        true,
    );
    assert!(bob_recv.contains("event=receipt_send kind=delivered bucket=small msg_id=<redacted>"));

    let pending = server.drain_channel(ROUTE_TOKEN_BOB);
    assert!(!pending.is_empty(), "expected queued ACK envelope");
    let ack_len = pending[0].len();
    assert!(
        ack_len >= EnvelopeProfile::Standard.min_size_bytes(),
        "ack must be at least standard profile size"
    );
    assert!(ack_len <= 4096, "ack must remain in bounded small class");
}

#[test]
fn no_secrets_in_receipt_outputs() {
    let server = common::start_inbox_server(1024 * 1024, 32);
    let base = safe_test_root().join(format!("na0113_nosecrets_{}", std::process::id()));
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
    contacts_route_set(&alice_cfg, "bob", ROUTE_TOKEN_BOB);
    contacts_route_set(&bob_cfg, "bob", ROUTE_TOKEN_BOB);
    let msg = base.join("msg.bin");
    fs::write(&msg, b"hello-nosecrets").unwrap();

    let mut all = String::new();
    all.push_str(&send_msg(&alice_cfg, server.base_url(), "bob", &msg, true));
    all.push_str(&recv_msg(
        &bob_cfg,
        server.base_url(),
        ROUTE_TOKEN_BOB,
        "bob",
        &bob_out,
        true,
    ));
    all.push_str(&recv_msg(
        &alice_cfg,
        server.base_url(),
        ROUTE_TOKEN_BOB,
        "bob",
        &alice_out,
        false,
    ));
    assert_no_secrets(&all);
}
