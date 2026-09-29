mod common;

use std::env;
use std::fs;
use std::path::{Path, PathBuf};

const ROUTE_TOKEN_ALICE: &str = "route_token_alice_abcdefghijklmnop";
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

fn create_dir_700(path: &Path) {
    let _ = fs::remove_dir_all(path);
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

fn combined_output(output: &std::process::Output) -> String {
    let mut combined = String::from_utf8_lossy(&output.stdout).to_string();
    combined.push_str(&String::from_utf8_lossy(&output.stderr));
    combined
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
    assert!(out.status.success(), "{}", combined_output(&out));
}

fn identity_fp(cfg: &Path, label: &str) -> String {
    let out = run_qsc(cfg, &["identity", "show", "--as", label]);
    assert!(out.status.success(), "{}", combined_output(&out));
    combined_output(&out)
        .lines()
        .find_map(|line| line.strip_prefix("identity_fp="))
        .map(ToOwned::to_owned)
        .unwrap_or_else(|| panic!("missing identity_fp in output: {}", combined_output(&out)))
}

fn identity_kem_pk(cfg: &Path, label: &str) -> String {
    let out = run_qsc(cfg, &["identity", "show", "--as", label]);
    assert!(out.status.success(), "{}", combined_output(&out));
    combined_output(&out)
        .lines()
        .find_map(|line| line.strip_prefix("identity_kem_pk="))
        .map(ToOwned::to_owned)
        .unwrap_or_else(|| panic!("missing identity_kem_pk in output: {}", combined_output(&out)))
}

fn identity_sig_pk(cfg: &Path, label: &str) -> String {
    let out = run_qsc(cfg, &["identity", "show", "--as", label]);
    assert!(out.status.success(), "{}", combined_output(&out));
    combined_output(&out)
        .lines()
        .find_map(|line| line.strip_prefix("identity_sig_pk="))
        .map(ToOwned::to_owned)
        .unwrap_or_else(|| panic!("missing identity_sig_pk in output: {}", combined_output(&out)))
}

fn contacts_route_set(cfg: &Path, label: &str, token: &str) {
    let out = run_qsc(
        cfg,
        &[
            "contacts",
            "add",
            "--label",
            label,
            "--fp",
            "fp-pinned-test",
            "--route-token",
            token,
        ],
    );
    assert!(out.status.success(), "{}", combined_output(&out));
}

fn contacts_add_authenticated_with_route(
    cfg: &Path,
    label: &str,
    fp: &str,
    kem_pk: &str, sig_pk: &str,
    token: &str,
) {
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
    assert!(out.status.success(), "{}", combined_output(&out));
}

fn relay_inbox_set(cfg: &Path, token: &str) {
    let out = run_qsc(cfg, &["relay", "inbox-set", "--token", token]);
    assert!(out.status.success(), "{}", combined_output(&out));
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
    contacts_add_authenticated_with_route(
        alice_cfg,
        "bob",
        bob_fp.as_str(),
        bob_kem.as_str(), bob_sig.as_str(),
        ROUTE_TOKEN_BOB,
    );
    contacts_add_authenticated_with_route(
        bob_cfg,
        "alice",
        alice_fp.as_str(),
        alice_kem.as_str(), alice_sig.as_str(),
        ROUTE_TOKEN_ALICE,
    );
}

// Establish the supported profile through the normal authenticated CLI callers.
// Seed fallback cannot stand in for a directional vault transaction.
fn establish_directional_pair(base: &Path, relay: &str) -> (PathBuf, PathBuf) {
    let alice = base.join("alice");
    let bob = base.join("bob");
    create_dir_700(&alice);
    create_dir_700(&bob);
    common::init_mock_vault(&alice);
    common::init_mock_vault(&bob);
    seed_authenticated_pair(&alice, &bob);
    relay_inbox_set(&alice, ROUTE_TOKEN_ALICE);
    relay_inbox_set(&bob, ROUTE_TOKEN_BOB);
    for (cfg, action, label, peer) in [
        (&alice, "init", "alice", "bob"),
        (&bob, "poll", "bob", "alice"),
        (&alice, "poll", "alice", "bob"),
        (&bob, "poll", "bob", "alice"),
    ] {
        let out = run_qsc(cfg, &["handshake", action, "--as", label, "--peer", peer, "--relay", relay]);
        assert!(out.status.success(), "{}", combined_output(&out));
    }
    (alice, bob)
}

#[test]
fn send_refuses_when_protocol_inactive() {
    let base = safe_test_root().join(format!("na0094_send_inactive_{}", std::process::id()));
    create_dir_700(&base);
    let cfg = base.join("cfg");
    create_dir_700(&cfg);
    common::init_mock_vault(&cfg);
    contacts_route_set(&cfg, "bob", ROUTE_TOKEN_BOB);
    let msg = base.join("msg.bin");
    fs::write(&msg, b"hello").expect("write msg");

    let output = common::qsc_std_command()
        .env("QSC_CONFIG_DIR", &cfg)
        .env(
            common::TEST_MOCK_VAULT_PASSPHRASE_ENV,
            common::TEST_MOCK_VAULT_PASSPHRASE,
        )
        .env("QSC_ALLOW_SEED_FALLBACK", "1")
        .env("QSC_UNSAFE_TEST_SEED_FALLBACK", "1")
        .env("QSC_MARK_FORMAT", "plain")
        .args([
            "send",
            "--transport",
            "relay",
            "--relay",
            "http://127.0.0.1:1",
            "--to",
            "bob",
            "--file",
            msg.to_str().unwrap(),
        ])
        .output()
        .expect("send inactive");
    let out = combined_output(&output);
    assert!(!output.status.success(), "send should fail");
    assert!(
        out.contains("event=error code=protocol_inactive reason=missing_seed"),
        "missing protocol_inactive marker: {}",
        out
    );
    let outbox = cfg.join("outbox.json");
    assert!(!outbox.exists(), "outbox should not be created");
}

#[test]
fn receive_refuses_when_protocol_inactive() {
    let base = safe_test_root().join(format!("na0094_recv_inactive_{}", std::process::id()));
    create_dir_700(&base);
    let cfg = base.join("cfg");
    create_dir_700(&cfg);
    common::init_mock_vault(&cfg);
    let out_dir = base.join("out");
    create_dir_700(&out_dir);

    let output = common::qsc_std_command()
        .env("QSC_CONFIG_DIR", &cfg)
        .env(
            common::TEST_MOCK_VAULT_PASSPHRASE_ENV,
            common::TEST_MOCK_VAULT_PASSPHRASE,
        )
        .env("QSC_ALLOW_SEED_FALLBACK", "1")
        .env("QSC_UNSAFE_TEST_SEED_FALLBACK", "1")
        .env("QSC_MARK_FORMAT", "plain")
        .args([
            "receive",
            "--transport",
            "relay",
            "--relay",
            "http://127.0.0.1:1",
            "--from",
            "bob",
            "--max",
            "1",
            "--out",
            out_dir.to_str().unwrap(),
        ])
        .output()
        .expect("receive inactive");
    let out = combined_output(&output);
    assert!(!output.status.success(), "receive should fail");
    assert!(
        out.contains("event=error code=protocol_inactive reason=missing_seed"),
        "missing protocol_inactive marker: {}",
        out
    );
    let entries = fs::read_dir(&out_dir).unwrap().count();
    assert_eq!(entries, 0, "out dir should remain empty");
}

#[test]
fn handshake_midpoint_protocol_gate_stays_honest() {
    let server = common::start_inbox_server(1024 * 1024, 32);
    let base = safe_test_root().join(format!(
        "na0223_protocol_gate_midpoint_{}",
        std::process::id()
    ));
    create_dir_700(&base);
    let alice_cfg = base.join("alice");
    let bob_cfg = base.join("bob");
    create_dir_700(&alice_cfg);
    create_dir_700(&bob_cfg);
    common::init_mock_vault(&alice_cfg);
    common::init_mock_vault(&bob_cfg);
    seed_authenticated_pair(&alice_cfg, &bob_cfg);
    relay_inbox_set(&alice_cfg, ROUTE_TOKEN_ALICE);
    relay_inbox_set(&bob_cfg, ROUTE_TOKEN_BOB);

    let relay = server.base_url().to_string();
    let alice_msg = base.join("alice-midpoint-msg.bin");
    let bob_msg = base.join("bob-midpoint-msg.bin");
    fs::write(&alice_msg, b"alice-midpoint").expect("write alice msg");
    fs::write(&bob_msg, b"bob-midpoint").expect("write bob msg");

    let alice_init = run_qsc(
        &alice_cfg,
        &[
            "handshake",
            "init",
            "--as",
            "alice",
            "--peer",
            "bob",
            "--relay",
            &relay,
        ],
    );
    assert!(
        alice_init.status.success(),
        "{}",
        combined_output(&alice_init)
    );

    let bob_poll = run_qsc(
        &bob_cfg,
        &[
            "handshake",
            "poll",
            "--as",
            "bob",
            "--peer",
            "alice",
            "--relay",
            &relay,
            "--max",
            "4",
        ],
    );
    assert!(bob_poll.status.success(), "{}", combined_output(&bob_poll));

    let alice_poll = run_qsc(
        &alice_cfg,
        &[
            "handshake",
            "poll",
            "--as",
            "alice",
            "--peer",
            "bob",
            "--relay",
            &relay,
            "--max",
            "4",
        ],
    );
    assert!(
        alice_poll.status.success(),
        "{}",
        combined_output(&alice_poll)
    );

    let alice_status = run_qsc(&alice_cfg, &["handshake", "status", "--peer", "bob"]);
    let alice_status_text = combined_output(&alice_status);
    assert!(alice_status.status.success(), "{alice_status_text}");
    assert!(
        alice_status_text.contains("event=handshake_status status=awaiting_peer_confirm peer=bob"),
        "{}",
        alice_status_text
    );
    assert!(
        alice_status_text.contains("peer_confirmed=no"),
        "{}",
        alice_status_text
    );
    assert!(
        alice_status_text.contains("send_ready=yes"),
        "{}",
        alice_status_text
    );

    let alice_send = run_qsc(
        &alice_cfg,
        &[
            "send",
            "--transport",
            "relay",
            "--relay",
            &relay,
            "--to",
            "bob",
            "--file",
            alice_msg.to_str().unwrap(),
        ],
    );
    let alice_send_text = combined_output(&alice_send);
    assert!(alice_send.status.success(), "{alice_send_text}");
    assert!(
        alice_send_text.contains("event=msgqueue_enqueued state=QUEUED"),
        "{alice_send_text}"
    );
    assert!(
        alice_send_text.contains("QSC_DELIVERY state=accepted_by_relay"),
        "{alice_send_text}"
    );

    let bob_send = run_qsc(
        &bob_cfg,
        &[
            "send",
            "--transport",
            "relay",
            "--relay",
            &relay,
            "--to",
            "alice",
            "--file",
            bob_msg.to_str().unwrap(),
        ],
    );
    let bob_send_text = combined_output(&bob_send);
    assert!(!bob_send.status.success(), "{bob_send_text}");
    assert!(
        bob_send_text.contains("event=error code=protocol_inactive reason=missing_seed"),
        "{}",
        bob_send_text
    );
    assert!(
        !bob_cfg.join("outbox.json").exists(),
        "outbox should not be created"
    );
}

#[test]
fn send_allows_when_protocol_active() {
    let server = common::start_inbox_server(1024 * 1024, 32);
    let base = safe_test_root().join(format!("directional_send_active_{}", std::process::id()));
    create_dir_700(&base);
    let (alice, _bob) = establish_directional_pair(&base, server.base_url());
    let msg = base.join("msg.bin");
    fs::write(&msg, b"hello").unwrap();
    let output = run_qsc(&alice, &["send", "--transport", "relay", "--relay", server.base_url(), "--to", "bob", "--file", msg.to_str().unwrap()]);
    let out = combined_output(&output);
    assert!(output.status.success(), "{out}");
    assert!(out.contains("event=msgqueue_enqueued state=QUEUED"), "{out}");
    assert!(out.contains("QSC_DELIVERY state=accepted_by_relay"), "{out}");
    assert_eq!(common::queued_record_count(&alice), 1, "authoritative ordinary operation retains its queue row");
    let flights = server.drain_channel(ROUTE_TOKEN_BOB);
    assert_eq!(flights.len(), 1);
    assert!(flights[0].starts_with(b"NDE1"));
    // Normal discard cannot orphan the retained exact directional flight.
    let queue_dir = fs::read_dir(alice.join("msgqueue_v1")).unwrap()
        .map(Result::unwrap).find(|entry| entry.path().is_dir()).unwrap().path();
    let row = fs::read_dir(queue_dir).unwrap().map(Result::unwrap)
        .find(|entry| entry.path().extension().is_some_and(|ext| ext == "rec")).unwrap().path();
    let row_before = fs::read(&row).unwrap();
    let stem = row.file_stem().unwrap().to_str().unwrap();
    let (_, msg_id) = stem.split_once('_').expect("sequence and message ID");
    let discard = run_qsc(&alice, &["outbox", "discard", "--to", "bob", "--msg-id", msg_id,
        "--relay", server.base_url(), "--confirm"]);
    let discarded = combined_output(&discard);
    assert!(!discard.status.success(), "{discarded}");
    assert!(discarded.contains("code=directional_discard_refused"), "{discarded}");
    assert_eq!(fs::read(&row).unwrap(), row_before, "refusal preserves the exact queue record");
    assert_eq!(common::queued_record_count(&alice), 1);
}

#[test]
fn receive_allows_when_protocol_active() {
    let server = common::start_inbox_server(1024 * 1024, 32);
    let base = safe_test_root().join(format!("directional_recv_active_{}", std::process::id()));
    create_dir_700(&base);
    let (alice, bob) = establish_directional_pair(&base, server.base_url());
    let out_dir = base.join("out");
    create_dir_700(&out_dir);
    let msg = base.join("msg.bin");
    fs::write(&msg, b"hello").unwrap();
    let sent = run_qsc(&alice, &["send", "--transport", "relay", "--relay", server.base_url(), "--to", "bob", "--file", msg.to_str().unwrap()]);
    assert!(sent.status.success(), "{}", combined_output(&sent));
    let receive = run_qsc(&bob, &["receive", "--transport", "relay", "--relay", server.base_url(), "--mailbox", ROUTE_TOKEN_BOB, "--from", "alice", "--max", "1", "--out", out_dir.to_str().unwrap()]);
    let out = combined_output(&receive);
    assert!(receive.status.success(), "{out}");
    let payloads: Vec<_> = fs::read_dir(&out_dir).unwrap().map(Result::unwrap).filter(|entry| entry.path().extension().is_some_and(|extension| extension == "bin")).collect();
    assert_eq!(payloads.len(), 1, "exactly one committed projection");
    assert_eq!(fs::read(payloads[0].path()).unwrap(), b"hello");
    assert!(server.drain_channel(ROUTE_TOKEN_ALICE).iter().any(|wire| wire.starts_with(b"NDR1")), "authenticated delivery releases a protocol receipt");
    for pat in ["TOKEN", "SECRET", "KEY", "PASS", "PRIVATE", "BEARER", "CREDENTIAL"] {
        assert!(!out.contains(pat), "secret pattern in receive output");
    }
}

#[test]
fn status_output_no_secrets() {
    let base = safe_test_root().join(format!("na0094_status_secrets_{}", std::process::id()));
    create_dir_700(&base);
    let cfg = base.join("cfg");
    create_dir_700(&cfg);

    let mut cmd = common::qsc_assert_command();
    cmd.env("QSC_CONFIG_DIR", &cfg)
        .env("QSC_MARK_FORMAT", "plain")
        .args(["status"]);
    let output = cmd.output().expect("status output");
    let out = combined_output(&output);
    let secret_patterns = [
        "TOKEN",
        "SECRET",
        "KEY",
        "PASS",
        "PRIVATE",
        "BEARER",
        "CREDENTIAL",
    ];
    for pat in secret_patterns {
        assert!(!out.contains(pat), "secret pattern in status output");
    }
}
