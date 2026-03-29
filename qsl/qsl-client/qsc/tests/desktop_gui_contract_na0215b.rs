mod common;

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

const ROUTE_TOKEN_ALICE: &str = "route_token_alice_abcdefghijklmnop";
const ROUTE_TOKEN_BOB: &str = "route_token_bob_abcdefghijklmnopqr";

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

fn unique_test_dir(tag: &str) -> PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock")
        .as_nanos();
    safe_test_root().join(format!("{tag}_{}_{}", std::process::id(), nonce))
}

fn output_text(out: &std::process::Output) -> String {
    let mut s = String::from_utf8_lossy(&out.stdout).to_string();
    s.push_str(&String::from_utf8_lossy(&out.stderr));
    s
}

fn init_vault(cfg: &Path) {
    let out = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", cfg)
        .env("QSC_PASSPHRASE", "desktop-passphrase")
        .env("QSC_DISABLE_KEYCHAIN", "1")
        .args([
            "vault",
            "init",
            "--non-interactive",
            "--passphrase-env",
            "QSC_PASSPHRASE",
        ])
        .output()
        .expect("vault init");
    assert!(out.status.success(), "vault init: {}", output_text(&out));
}

fn qsc_plain(cfg: &Path) -> Command {
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("qsc"));
    cmd.env("QSC_CONFIG_DIR", cfg)
        .env("QSC_MARK_FORMAT", "plain");
    cmd
}

fn qsc_with_unlock(cfg: &Path) -> Command {
    let mut cmd = qsc_plain(cfg);
    cmd.env("QSC_PASSPHRASE", "desktop-passphrase")
        .env("QSC_DISABLE_KEYCHAIN", "1")
        .arg("--unlock-passphrase-env")
        .arg("QSC_PASSPHRASE");
    cmd
}

fn identity_fp(cfg: &Path) -> String {
    let out = qsc_with_unlock(cfg)
        .args(["identity", "show"])
        .output()
        .expect("identity show");
    assert!(out.status.success(), "{}", output_text(&out));
    output_text(&out)
        .lines()
        .find_map(|line| line.strip_prefix("identity_fp=").map(ToOwned::to_owned))
        .unwrap_or_else(|| panic!("missing identity_fp: {}", output_text(&out)))
}

fn device_id(cfg: &Path, label: &str) -> String {
    let out = qsc_with_unlock(cfg)
        .args(["contacts", "device", "list", "--label", label])
        .output()
        .expect("contacts device list");
    assert!(out.status.success(), "{}", output_text(&out));
    output_text(&out)
        .lines()
        .find(|line| line.starts_with("device="))
        .and_then(|line| {
            line.split_whitespace()
                .find_map(|part| part.strip_prefix("device="))
        })
        .unwrap_or_else(|| panic!("missing device output: {}", output_text(&out)))
        .to_string()
}

fn trust_device(cfg: &Path, label: &str) {
    let device = device_id(cfg, label);
    let out = qsc_with_unlock(cfg)
        .args([
            "contacts",
            "device",
            "trust",
            "--label",
            label,
            "--device",
            device.as_str(),
            "--confirm",
        ])
        .output()
        .expect("contacts device trust");
    assert!(out.status.success(), "{}", output_text(&out));
}

fn handshake_status(cfg: &Path, peer: &str) -> String {
    let out = qsc_with_unlock(cfg)
        .args(["handshake", "status", "--peer", peer])
        .output()
        .expect("handshake status");
    assert!(out.status.success(), "{}", output_text(&out));
    output_text(&out)
}

fn complete_handshake(relay: &str, alice_cfg: &Path, bob_cfg: &Path) {
    let alice_init = qsc_with_unlock(alice_cfg)
        .args([
            "handshake",
            "init",
            "--as",
            "self",
            "--peer",
            "bob",
            "--relay",
            relay,
        ])
        .output()
        .expect("alice handshake init");
    assert!(alice_init.status.success(), "{}", output_text(&alice_init));

    let bob_poll = qsc_with_unlock(bob_cfg)
        .args([
            "handshake",
            "poll",
            "--as",
            "self",
            "--peer",
            "alice",
            "--relay",
            relay,
            "--max",
            "4",
        ])
        .output()
        .expect("bob handshake poll");
    assert!(bob_poll.status.success(), "{}", output_text(&bob_poll));

    let alice_poll = qsc_with_unlock(alice_cfg)
        .args([
            "handshake",
            "poll",
            "--as",
            "self",
            "--peer",
            "bob",
            "--relay",
            relay,
            "--max",
            "4",
        ])
        .output()
        .expect("alice handshake poll");
    assert!(alice_poll.status.success(), "{}", output_text(&alice_poll));

    let bob_confirm = qsc_with_unlock(bob_cfg)
        .args([
            "handshake",
            "poll",
            "--as",
            "self",
            "--peer",
            "alice",
            "--relay",
            relay,
            "--max",
            "4",
        ])
        .output()
        .expect("bob handshake confirm");
    assert!(
        bob_confirm.status.success(),
        "{}",
        output_text(&bob_confirm)
    );
}

#[test]
fn desktop_gui_profile_surface_is_deterministic() {
    let base = unique_test_dir("na0215b_profile_surface");
    create_dir_700(&base);
    let cfg = base.join("cfg");
    create_dir_700(&cfg);
    init_vault(&cfg);

    let rotate = qsc_with_unlock(&cfg)
        .args(["identity", "rotate", "--confirm"])
        .output()
        .expect("identity rotate");
    assert!(rotate.status.success(), "{}", output_text(&rotate));

    let doctor = qsc_plain(&cfg)
        .args(["doctor", "--check-only"])
        .output()
        .expect("doctor");
    assert!(doctor.status.success(), "{}", output_text(&doctor));
    let doctor_text = output_text(&doctor);
    assert!(doctor_text.contains("event=doctor"), "{}", doctor_text);
    assert!(doctor_text.contains("dir_exists=true"), "{}", doctor_text);
    assert!(doctor_text.contains("symlink_safe=true"), "{}", doctor_text);
    assert!(doctor_text.contains("parent_safe=true"), "{}", doctor_text);

    let vault = qsc_plain(&cfg)
        .args(["vault", "status"])
        .output()
        .expect("vault status");
    assert!(vault.status.success(), "{}", output_text(&vault));
    let vault_text = output_text(&vault);
    assert!(vault_text.contains("event=vault_status"), "{}", vault_text);
    assert!(vault_text.contains("present=true"), "{}", vault_text);
    assert!(
        vault_text.contains("key_source=passphrase"),
        "{}",
        vault_text
    );

    let show = qsc_with_unlock(&cfg)
        .args(["identity", "show"])
        .output()
        .expect("identity show");
    assert!(show.status.success(), "{}", output_text(&show));
    let show_text = output_text(&show);
    assert!(show_text.contains("event=identity_show"), "{}", show_text);
    assert!(show_text.contains("identity_fp="), "{}", show_text);
}

#[test]
fn desktop_gui_contact_device_surface_is_deterministic() {
    let base = unique_test_dir("na0215b_contact_surface");
    create_dir_700(&base);
    let cfg = base.join("cfg");
    create_dir_700(&cfg);
    init_vault(&cfg);

    let rotate = qsc_with_unlock(&cfg)
        .args(["identity", "rotate", "--confirm"])
        .output()
        .expect("identity rotate");
    assert!(rotate.status.success(), "{}", output_text(&rotate));

    let add = qsc_with_unlock(&cfg)
        .args([
            "contacts",
            "add",
            "--label",
            "bob",
            "--fp",
            "QSCFP-test-bob",
            "--route-token",
            ROUTE_TOKEN_BOB,
        ])
        .output()
        .expect("contacts add");
    assert!(add.status.success(), "{}", output_text(&add));

    let list = qsc_with_unlock(&cfg)
        .args(["contacts", "list"])
        .output()
        .expect("contacts list");
    assert!(list.status.success(), "{}", output_text(&list));
    let list_text = output_text(&list);
    assert!(
        list_text.contains("event=contacts_list count=1"),
        "{}",
        list_text
    );
    assert!(list_text.contains("label=bob"), "{}", list_text);
    assert!(list_text.contains("blocked=false"), "{}", list_text);
    assert!(list_text.contains("device_count=1"), "{}", list_text);

    let devices = qsc_with_unlock(&cfg)
        .args(["contacts", "device", "list", "--label", "bob"])
        .output()
        .expect("contacts device list");
    assert!(devices.status.success(), "{}", output_text(&devices));
    let devices_text = output_text(&devices);
    assert!(
        devices_text.contains("event=contacts_device_list"),
        "{}",
        devices_text
    );
    assert!(devices_text.contains("device="), "{}", devices_text);
    assert!(devices_text.contains("state="), "{}", devices_text);
}

#[test]
fn desktop_gui_message_surface_reports_delivery_and_timeline_truth() {
    let server = common::start_inbox_server(1024 * 1024, 16);
    let base = unique_test_dir("na0215b_message_surface");
    create_dir_700(&base);
    let alice_cfg = base.join("alice_cfg");
    let bob_cfg = base.join("bob_cfg");
    let alice_out = base.join("alice_out");
    let bob_out = base.join("bob_out");
    create_dir_700(&alice_cfg);
    create_dir_700(&bob_cfg);
    create_dir_700(&alice_out);
    create_dir_700(&bob_out);
    init_vault(&alice_cfg);
    init_vault(&bob_cfg);

    let alice_rotate = qsc_with_unlock(&alice_cfg)
        .args(["identity", "rotate", "--confirm"])
        .output()
        .expect("alice identity rotate");
    assert!(
        alice_rotate.status.success(),
        "{}",
        output_text(&alice_rotate)
    );
    let bob_rotate = qsc_with_unlock(&bob_cfg)
        .args(["identity", "rotate", "--confirm"])
        .output()
        .expect("bob identity rotate");
    assert!(bob_rotate.status.success(), "{}", output_text(&bob_rotate));

    let alice_fp = identity_fp(&alice_cfg);
    let bob_fp = identity_fp(&bob_cfg);

    let alice_inbox = qsc_with_unlock(&alice_cfg)
        .args(["relay", "inbox-set", "--token", ROUTE_TOKEN_ALICE])
        .output()
        .expect("alice inbox set");
    assert!(
        alice_inbox.status.success(),
        "{}",
        output_text(&alice_inbox)
    );
    let bob_inbox = qsc_with_unlock(&bob_cfg)
        .args(["relay", "inbox-set", "--token", ROUTE_TOKEN_BOB])
        .output()
        .expect("bob inbox set");
    assert!(bob_inbox.status.success(), "{}", output_text(&bob_inbox));

    let add_bob = qsc_with_unlock(&alice_cfg)
        .args([
            "contacts",
            "add",
            "--label",
            "bob",
            "--fp",
            bob_fp.as_str(),
            "--route-token",
            ROUTE_TOKEN_BOB,
        ])
        .output()
        .expect("alice adds bob");
    assert!(add_bob.status.success(), "{}", output_text(&add_bob));
    let add_alice = qsc_with_unlock(&bob_cfg)
        .args([
            "contacts",
            "add",
            "--label",
            "alice",
            "--fp",
            alice_fp.as_str(),
            "--route-token",
            ROUTE_TOKEN_ALICE,
        ])
        .output()
        .expect("bob adds alice");
    assert!(add_alice.status.success(), "{}", output_text(&add_alice));

    trust_device(&alice_cfg, "bob");
    trust_device(&bob_cfg, "alice");

    let payload = base.join("msg.txt");
    fs::write(&payload, "desktop gui contract").expect("payload write");

    let handshake_before = handshake_status(&alice_cfg, "bob");
    assert!(
        handshake_before.contains("event=handshake_status"),
        "{}",
        handshake_before
    );
    assert!(
        handshake_before.contains("status=no_session"),
        "{}",
        handshake_before
    );
    assert!(
        handshake_before.contains("send_ready=no"),
        "{}",
        handshake_before
    );
    assert!(
        handshake_before.contains("send_ready_reason=no_session"),
        "{}",
        handshake_before
    );

    let send_blocked = qsc_with_unlock(&alice_cfg)
        .args([
            "send",
            "--transport",
            "relay",
            "--relay",
            server.base_url(),
            "--to",
            "bob",
            "--file",
            payload.to_str().unwrap(),
            "--receipt",
            "delivered",
        ])
        .output()
        .expect("send blocked");
    assert!(
        !send_blocked.status.success(),
        "{}",
        output_text(&send_blocked)
    );
    let send_blocked_text = output_text(&send_blocked);
    assert!(
        send_blocked_text.contains("event=error code=protocol_inactive reason=missing_seed"),
        "{}",
        send_blocked_text
    );

    let bob_recv_blocked = qsc_with_unlock(&bob_cfg)
        .args([
            "receive",
            "--transport",
            "relay",
            "--relay",
            server.base_url(),
            "--mailbox",
            ROUTE_TOKEN_BOB,
            "--from",
            "alice",
            "--max",
            "4",
            "--out",
            bob_out.to_str().unwrap(),
            "--emit-receipts",
            "delivered",
            "--receipt-mode",
            "immediate",
        ])
        .output()
        .expect("bob receive blocked");
    assert!(
        !bob_recv_blocked.status.success(),
        "{}",
        output_text(&bob_recv_blocked)
    );
    let bob_recv_blocked_text = output_text(&bob_recv_blocked);
    assert!(
        bob_recv_blocked_text.contains("event=error code=protocol_inactive reason=missing_seed"),
        "{}",
        bob_recv_blocked_text
    );

    complete_handshake(server.base_url(), &alice_cfg, &bob_cfg);

    let alice_ready = handshake_status(&alice_cfg, "bob");
    assert!(
        alice_ready.contains("status=established"),
        "{}",
        alice_ready
    );
    assert!(alice_ready.contains("send_ready=yes"), "{}", alice_ready);

    let bob_ready = handshake_status(&bob_cfg, "alice");
    assert!(
        bob_ready.contains("status=established_recv_only"),
        "{}",
        bob_ready
    );
    assert!(bob_ready.contains("send_ready=no"), "{}", bob_ready);
    assert!(
        bob_ready.contains("send_ready_reason=chainkey_unset"),
        "{}",
        bob_ready
    );

    let send = qsc_with_unlock(&alice_cfg)
        .args([
            "send",
            "--transport",
            "relay",
            "--relay",
            server.base_url(),
            "--to",
            "bob",
            "--file",
            payload.to_str().unwrap(),
            "--receipt",
            "delivered",
        ])
        .output()
        .expect("send message");
    assert!(send.status.success(), "{}", output_text(&send));
    let send_text = output_text(&send);
    assert!(
        send_text.contains("QSC_DELIVERY state=accepted_by_relay"),
        "{}",
        send_text
    );

    let bob_recv = qsc_with_unlock(&bob_cfg)
        .args([
            "receive",
            "--transport",
            "relay",
            "--relay",
            server.base_url(),
            "--mailbox",
            ROUTE_TOKEN_BOB,
            "--from",
            "alice",
            "--max",
            "4",
            "--out",
            bob_out.to_str().unwrap(),
            "--emit-receipts",
            "delivered",
            "--receipt-mode",
            "immediate",
        ])
        .output()
        .expect("bob receive");
    assert!(bob_recv.status.success(), "{}", output_text(&bob_recv));
    let bob_recv_text = output_text(&bob_recv);
    assert!(
        bob_recv_text.contains("event=recv_commit"),
        "{}",
        bob_recv_text
    );

    let alice_recv = qsc_with_unlock(&alice_cfg)
        .args([
            "receive",
            "--transport",
            "relay",
            "--relay",
            server.base_url(),
            "--mailbox",
            ROUTE_TOKEN_ALICE,
            "--from",
            "bob",
            "--max",
            "4",
            "--out",
            alice_out.to_str().unwrap(),
        ])
        .output()
        .expect("alice receive");
    assert!(alice_recv.status.success(), "{}", output_text(&alice_recv));
    let alice_recv_text = output_text(&alice_recv);
    assert!(
        alice_recv_text.contains("QSC_DELIVERY state=peer_confirmed"),
        "{}",
        alice_recv_text
    );

    let timeline = qsc_with_unlock(&alice_cfg)
        .args(["timeline", "list", "--peer", "bob", "--limit", "8"])
        .output()
        .expect("timeline list");
    assert!(timeline.status.success(), "{}", output_text(&timeline));
    let timeline_text = output_text(&timeline);
    assert!(
        timeline_text.contains("event=timeline_list"),
        "{}",
        timeline_text
    );
    assert!(
        timeline_text.contains("event=timeline_item"),
        "{}",
        timeline_text
    );
    assert!(
        timeline_text.contains("state=peer_confirmed"),
        "{}",
        timeline_text
    );
}
