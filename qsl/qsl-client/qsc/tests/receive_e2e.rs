mod common;

use predicates::str::contains;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const ROUTE_TOKEN_ALICE: &str = "route_token_alice_abcdefghijklmnop";
const ROUTE_TOKEN_BOB: &str = "route_token_bob_abcdefghijklmnopqr";
const ROUTE_TOKEN_PEER0: &str = "route_token_peer0_abcdefghijklmnop";

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

fn qsc_with_unlock(cfg: &Path, passphrase: &str) -> Command {
    let mut cmd = common::qsc_std_command();
    cmd.env("QSC_CONFIG_DIR", cfg);
    common::add_global_unlock_passphrase_file_arg(&mut cmd, cfg, "receive-e2e", passphrase);
    cmd
}

fn qsc_with_unlock_marked(cfg: &Path, passphrase: &str) -> Command {
    let mut cmd = qsc_with_unlock(cfg, passphrase);
    cmd.env("QSC_QSP_SEED", "1")
        .env("QSC_ALLOW_SEED_FALLBACK", "1")
        .env("QSC_MARK_FORMAT", "plain");
    cmd
}

fn identity_rotate(cfg: &Path, label: &str, passphrase: &str) {
    let out = qsc_with_unlock(cfg, passphrase)
        .args(["identity", "rotate", "--as", label, "--confirm"])
        .output()
        .expect("identity rotate");
    assert!(out.status.success(), "{}", combined_output(&out));
}

fn identity_fp(cfg: &Path, label: &str, passphrase: &str) -> String {
    let out = qsc_with_unlock(cfg, passphrase)
        .args(["identity", "show", "--as", label])
        .output()
        .expect("identity show");
    assert!(out.status.success(), "{}", combined_output(&out));
    combined_output(&out)
        .lines()
        .find_map(|line| line.strip_prefix("identity_fp="))
        .map(ToOwned::to_owned)
        .unwrap_or_else(|| panic!("missing identity_fp marker: {}", combined_output(&out)))
}

fn contacts_route_set(cfg: &Path, label: &str, token: &str, passphrase: Option<&str>) {
    let mut cmd = common::qsc_std_command();
    cmd.env("QSC_CONFIG_DIR", cfg);
    if let Some(pass) = passphrase {
        common::add_global_unlock_passphrase_file_arg(&mut cmd, cfg, "contacts-route", pass);
    }
    let out = cmd
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

fn contacts_add_authenticated_with_route(
    cfg: &Path,
    label: &str,
    fp: &str,
    token: &str,
    passphrase: &str,
) {
    let out = qsc_with_unlock(cfg, passphrase)
        .args([
            "contacts",
            "add",
            "--label",
            label,
            "--fp",
            fp,
            "--route-token",
            token,
        ])
        .output()
        .expect("contacts add authenticated");
    assert!(out.status.success(), "{}", combined_output(&out));
}

fn relay_inbox_set(cfg: &Path, token: &str, passphrase: Option<&str>) {
    let mut cmd = common::qsc_std_command();
    cmd.env("QSC_CONFIG_DIR", cfg);
    if let Some(pass) = passphrase {
        common::add_global_unlock_passphrase_file_arg(&mut cmd, cfg, "relay-inbox", pass);
    }
    let out = cmd
        .args(["relay", "inbox-set", "--token", token])
        .output()
        .expect("relay inbox set");
    assert!(out.status.success(), "{}", combined_output(&out));
}

#[test]
fn receive_two_way_e2e_local_inbox() {
    let server = common::start_inbox_server(1024 * 1024, 32);
    let base = safe_test_root().join(format!("na0091_recv_e2e_{}", std::process::id()));
    create_dir_700(&base);

    let alice_cfg = base.join("alice_cfg");
    let bob_cfg = base.join("bob_cfg");
    create_dir_700(&alice_cfg);
    create_dir_700(&bob_cfg);
    common::init_mock_vault(&alice_cfg);
    common::init_mock_vault(&bob_cfg);
    contacts_route_set(&alice_cfg, "bob", ROUTE_TOKEN_BOB, None);
    contacts_route_set(&bob_cfg, "alice", ROUTE_TOKEN_ALICE, None);

    let alice_out = base.join("alice_out");
    let bob_out = base.join("bob_out");
    create_dir_700(&alice_out);
    create_dir_700(&bob_out);

    let msg_a = base.join("msg_a.bin");
    let msg_b = base.join("msg_b.bin");
    fs::write(&msg_a, b"hello-bob").expect("write msg_a");
    fs::write(&msg_b, b"hello-alice").expect("write msg_b");

    let output_a = common::qsc_std_command()
        .env("QSC_CONFIG_DIR", &alice_cfg)
        .env("QSC_QSP_SEED", "1")
        .env("QSC_ALLOW_SEED_FALLBACK", "1")
        .env("QSC_MARK_FORMAT", "plain")
        .args([
            "send",
            "--transport",
            "relay",
            "--relay",
            server.base_url(),
            "--to",
            "bob",
            "--file",
            msg_a.to_str().unwrap(),
        ])
        .output()
        .expect("send a");
    assert!(output_a.status.success(), "send a failed");

    let output_b = common::qsc_std_command()
        .env("QSC_CONFIG_DIR", &bob_cfg)
        .env("QSC_QSP_SEED", "1")
        .env("QSC_ALLOW_SEED_FALLBACK", "1")
        .env("QSC_MARK_FORMAT", "plain")
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
        .expect("receive b");
    let out_b = combined_output(&output_b);
    assert!(output_b.status.success(), "receive b failed");
    assert!(out_b.contains("event=recv_start transport=relay"));
    assert!(out_b.contains("event=recv_item"));
    assert!(out_b.contains("event=recv_commit"));

    let bob_file = bob_out.join("recv_1.bin");
    let bob_bytes = fs::read(&bob_file).expect("bob recv file");
    assert_eq!(bob_bytes, b"hello-bob");

    let output_b_send = common::qsc_std_command()
        .env("QSC_CONFIG_DIR", &bob_cfg)
        .env("QSC_QSP_SEED", "1")
        .env("QSC_ALLOW_SEED_FALLBACK", "1")
        .env("QSC_MARK_FORMAT", "plain")
        .args([
            "send",
            "--transport",
            "relay",
            "--relay",
            server.base_url(),
            "--to",
            "alice",
            "--file",
            msg_b.to_str().unwrap(),
        ])
        .output()
        .expect("send b");
    assert!(output_b_send.status.success(), "send b failed");

    let output_a_recv = common::qsc_std_command()
        .env("QSC_CONFIG_DIR", &alice_cfg)
        .env("QSC_QSP_SEED", "1")
        .env("QSC_ALLOW_SEED_FALLBACK", "1")
        .env("QSC_MARK_FORMAT", "plain")
        .args([
            "receive",
            "--transport",
            "relay",
            "--relay",
            server.base_url(),
            "--mailbox",
            ROUTE_TOKEN_ALICE,
            "--from",
            "alice",
            "--max",
            "1",
            "--out",
            alice_out.to_str().unwrap(),
        ])
        .output()
        .expect("receive a");
    let out_a = combined_output(&output_a_recv);
    assert!(output_a_recv.status.success(), "receive a failed");
    assert!(out_a.contains("event=recv_start transport=relay"));
    assert!(out_a.contains("event=recv_item"));
    assert!(out_a.contains("event=recv_commit"));

    let alice_file = alice_out.join("recv_1.bin");
    let alice_bytes = fs::read(&alice_file).expect("alice recv file");
    assert_eq!(alice_bytes, b"hello-alice");

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
        assert!(!out_b.contains(pat), "secret pattern in receive output");
        assert!(!out_a.contains(pat), "secret pattern in receive output");
    }
}

#[test]
fn receive_mailbox_peer_separation_fail_closed() {
    let server = common::start_inbox_server(1024 * 1024, 32);
    let base = safe_test_root().join(format!("na0108_mailbox_peer_{}", std::process::id()));
    create_dir_700(&base);

    let alice_cfg = base.join("alice_cfg");
    let bob_cfg = base.join("bob_cfg");
    create_dir_700(&alice_cfg);
    create_dir_700(&bob_cfg);

    let bob_out = base.join("bob_out");
    create_dir_700(&bob_out);
    let msg = base.join("msg.bin");
    fs::write(&msg, b"hello-bob-mailbox-peer").expect("write msg");

    common::init_passphrase_vault(&alice_cfg, "test-pass-a");
    common::init_passphrase_vault(&bob_cfg, "test-pass-b");
    identity_rotate(&alice_cfg, "alice", "test-pass-a");
    identity_rotate(&bob_cfg, "bob", "test-pass-b");
    let alice_fp = identity_fp(&alice_cfg, "alice", "test-pass-a");
    let bob_fp = identity_fp(&bob_cfg, "bob", "test-pass-b");
    contacts_add_authenticated_with_route(
        &alice_cfg,
        "bob",
        bob_fp.as_str(),
        ROUTE_TOKEN_BOB,
        "test-pass-a",
    );
    contacts_add_authenticated_with_route(
        &bob_cfg,
        "alice",
        alice_fp.as_str(),
        ROUTE_TOKEN_ALICE,
        "test-pass-b",
    );
    relay_inbox_set(&bob_cfg, ROUTE_TOKEN_BOB, Some("test-pass-b"));

    let hs_init = qsc_with_unlock_marked(&alice_cfg, "test-pass-a")
        .args([
            "handshake",
            "init",
            "--as",
            "alice",
            "--peer",
            "bob",
            "--relay",
            server.base_url(),
        ])
        .output()
        .expect("hs_init");
    assert!(hs_init.status.success(), "hs_init failed");

    let hs_bob_poll_1 = qsc_with_unlock_marked(&bob_cfg, "test-pass-b")
        .args([
            "handshake",
            "poll",
            "--as",
            "bob",
            "--peer",
            "alice",
            "--relay",
            server.base_url(),
            "--max",
            "4",
        ])
        .output()
        .expect("hs_bob_poll_1");
    assert!(hs_bob_poll_1.status.success(), "hs_bob_poll_1 failed");

    let hs_alice_poll_2 = qsc_with_unlock_marked(&alice_cfg, "test-pass-a")
        .args([
            "handshake",
            "poll",
            "--as",
            "alice",
            "--peer",
            "bob",
            "--relay",
            server.base_url(),
            "--max",
            "4",
        ])
        .output()
        .expect("hs_alice_poll_2");
    assert!(hs_alice_poll_2.status.success(), "hs_alice_poll_2 failed");

    let hs_bob_poll_3 = qsc_with_unlock_marked(&bob_cfg, "test-pass-b")
        .args([
            "handshake",
            "poll",
            "--as",
            "bob",
            "--peer",
            "alice",
            "--relay",
            server.base_url(),
            "--max",
            "4",
        ])
        .output()
        .expect("hs_bob_poll_3");
    assert!(hs_bob_poll_3.status.success(), "hs_bob_poll_3 failed");

    let send1 = qsc_with_unlock_marked(&alice_cfg, "test-pass-a")
        .args([
            "send",
            "--transport",
            "relay",
            "--relay",
            server.base_url(),
            "--to",
            "bob",
            "--file",
            msg.to_str().unwrap(),
        ])
        .output()
        .expect("send1");
    assert!(
        send1.status.success(),
        "send1 failed: {}",
        combined_output(&send1)
    );

    let recv_seed = qsc_with_unlock_marked(&bob_cfg, "test-pass-b")
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
        .expect("recv_alice");
    let recv_seed_out = combined_output(&recv_seed);
    assert!(
        recv_seed.status.success(),
        "recv_alice failed: {recv_seed_out}"
    );
    assert!(recv_seed_out.contains("event=recv_start transport=relay"));
    assert!(recv_seed_out.contains("event=qsp_unpack ok=true"));
    assert!(recv_seed_out.contains("event=recv_commit"));

    let send2 = qsc_with_unlock_marked(&alice_cfg, "test-pass-a")
        .args([
            "send",
            "--transport",
            "relay",
            "--relay",
            server.base_url(),
            "--to",
            "bob",
            "--file",
            msg.to_str().unwrap(),
        ])
        .output()
        .expect("send2");
    assert!(send2.status.success(), "send2 failed");

    let recv_no_seed = qsc_with_unlock_marked(&bob_cfg, "test-pass-b")
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
        .expect("recv_no_seed");
    let recv_no_seed_out = combined_output(&recv_no_seed);
    assert!(
        recv_no_seed.status.success(),
        "recv_no_seed failed unexpectedly: {recv_no_seed_out}"
    );
    assert!(recv_no_seed_out.contains("event=qsp_unpack ok=true"));

    let before_entries = fs::read_dir(&bob_out).unwrap().count();
    let recv_bad_mailbox = qsc_with_unlock_marked(&bob_cfg, "test-pass-b")
        .args([
            "receive",
            "--transport",
            "relay",
            "--relay",
            server.base_url(),
            "--mailbox",
            "bad/mailbox",
            "--from",
            "alice",
            "--max",
            "1",
            "--out",
            bob_out.to_str().unwrap(),
        ])
        .output()
        .expect("recv_bad_mailbox");
    let recv_bad_mailbox_out = combined_output(&recv_bad_mailbox);
    assert!(
        !recv_bad_mailbox.status.success(),
        "recv_bad_mailbox should fail"
    );
    assert!(
        recv_bad_mailbox_out.contains("event=error code=recv_mailbox_invalid")
            || recv_bad_mailbox_out.contains("event=error code=QSC_ERR_ROUTE_TOKEN_INVALID")
    );
    let after_bad_mailbox_entries = fs::read_dir(&bob_out).unwrap().count();
    assert_eq!(before_entries, after_bad_mailbox_entries);

    let send3 = qsc_with_unlock_marked(&alice_cfg, "test-pass-a")
        .args([
            "send",
            "--transport",
            "relay",
            "--relay",
            server.base_url(),
            "--to",
            "bob",
            "--file",
            msg.to_str().unwrap(),
        ])
        .output()
        .expect("send3");
    assert!(send3.status.success(), "send3 failed");

    let before_wrong_peer_entries = fs::read_dir(&bob_out).unwrap().count();
    let recv_wrong_peer = qsc_with_unlock_marked(&bob_cfg, "test-pass-b")
        .args([
            "receive",
            "--transport",
            "relay",
            "--relay",
            server.base_url(),
            "--mailbox",
            ROUTE_TOKEN_BOB,
            "--from",
            "charlie",
            "--max",
            "1",
            "--out",
            bob_out.to_str().unwrap(),
        ])
        .output()
        .expect("recv_wrong_peer");
    let recv_wrong_peer_out = combined_output(&recv_wrong_peer);
    assert!(
        !recv_wrong_peer.status.success(),
        "recv_wrong_peer should fail"
    );
    assert!(
        recv_wrong_peer_out.contains("event=error code=protocol_inactive")
            || recv_wrong_peer_out.contains("event=error code=qsp_hdr_auth_failed")
    );
    let after_wrong_peer_entries = fs::read_dir(&bob_out).unwrap().count();
    assert_eq!(before_wrong_peer_entries, after_wrong_peer_entries);
}

#[test]
fn tui_receive_headless_marks() {
    let server = common::start_inbox_server(1024 * 1024, 32);
    let base = safe_test_root().join(format!("na0091_tui_recv_{}", std::process::id()));
    create_dir_700(&base);
    let cfg = base.join("cfg");
    create_dir_700(&cfg);
    common::init_mock_vault(&cfg);
    contacts_route_set(&cfg, "peer-0", ROUTE_TOKEN_PEER0, None);
    relay_inbox_set(&cfg, ROUTE_TOKEN_PEER0, None);
    let msg = base.join("msg.bin");
    fs::write(&msg, b"hello").expect("write msg");

    let output_send = common::qsc_std_command()
        .env("QSC_CONFIG_DIR", &cfg)
        .env("QSC_QSP_SEED", "1")
        .env("QSC_ALLOW_SEED_FALLBACK", "1")
        .env("QSC_MARK_FORMAT", "plain")
        .args([
            "send",
            "--transport",
            "relay",
            "--relay",
            server.base_url(),
            "--to",
            "peer-0",
            "--file",
            msg.to_str().unwrap(),
        ])
        .output()
        .expect("send to peer-0");
    assert!(output_send.status.success(), "send failed");

    let mut cmd = common::qsc_assert_command();
    cmd.env("QSC_CONFIG_DIR", &cfg)
        .env("QSC_QSP_SEED", "1")
        .env("QSC_ALLOW_SEED_FALLBACK", "1")
        .env("QSC_MARK_FORMAT", "plain")
        .env("QSC_TUI_HEADLESS", "1")
        .env("QSC_TUI_SCRIPT", "/receive;/exit")
        .args([
            "tui",
            "--transport",
            "relay",
            "--relay",
            server.base_url(),
            "--seed",
            "7",
            "--scenario",
            "happy-path",
        ]);
    cmd.assert()
        .success()
        .stdout(contains("event=tui_receive"))
        .stdout(contains("event=recv_item"));
}
