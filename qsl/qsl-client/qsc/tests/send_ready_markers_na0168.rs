mod common;

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const ROUTE_TOKEN_ALICE: &str = "route_token_alice_abcdefghijklmnopq";
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

fn combined_output(output: &std::process::Output) -> String {
    let mut combined = String::from_utf8_lossy(&output.stdout).to_string();
    combined.push_str(&String::from_utf8_lossy(&output.stderr));
    combined
}

fn contacts_route_set(cfg: &Path, label: &str, token: &str) {
    let out = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", cfg)
        .args([
            "contacts",
            "route-set",
            "--label",
            label,
            "--route-token",
            token,
        ])
        .output()
        .expect("contacts route set");
    assert!(out.status.success(), "{}", combined_output(&out));
}

fn relay_inbox_set(cfg: &Path, token: &str) {
    let out = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", cfg)
        .args(["relay", "inbox-set", "--token", token])
        .output()
        .expect("relay inbox set");
    assert!(out.status.success(), "{}", combined_output(&out));
}

#[test]
fn responder_first_reply_is_send_ready_and_succeeds_after_bootstrap() {
    let base = safe_test_root().join(format!("na0168_send_ready_markers_{}", std::process::id()));
    let _ = fs::remove_dir_all(&base);
    ensure_dir_700(&base);
    let alice_cfg = base.join("alice");
    let bob_cfg = base.join("bob");
    ensure_dir_700(&alice_cfg);
    ensure_dir_700(&bob_cfg);

    common::init_mock_vault(&alice_cfg);
    common::init_mock_vault(&bob_cfg);
    contacts_route_set(&alice_cfg, "bob", ROUTE_TOKEN_BOB);
    contacts_route_set(&bob_cfg, "alice", ROUTE_TOKEN_ALICE);
    relay_inbox_set(&alice_cfg, ROUTE_TOKEN_ALICE);
    relay_inbox_set(&bob_cfg, ROUTE_TOKEN_BOB);

    let server = common::start_inbox_server(1024 * 1024, 32);
    let relay = server.base_url().to_string();

    let hs_init = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", &alice_cfg)
        .args([
            "handshake",
            "init",
            "--as",
            "alice",
            "--peer",
            "bob",
            "--relay",
            &relay,
        ])
        .output()
        .expect("handshake init alice");
    assert!(hs_init.status.success(), "{}", combined_output(&hs_init));

    let hs_b = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", &bob_cfg)
        .args([
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
        ])
        .output()
        .expect("handshake poll bob");
    assert!(hs_b.status.success(), "{}", combined_output(&hs_b));

    let hs_a = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", &alice_cfg)
        .args([
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
        ])
        .output()
        .expect("handshake poll alice");
    assert!(hs_a.status.success(), "{}", combined_output(&hs_a));

    let hs_b_confirm = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", &bob_cfg)
        .args([
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
        ])
        .output()
        .expect("handshake poll bob confirm");
    assert!(
        hs_b_confirm.status.success(),
        "{}",
        combined_output(&hs_b_confirm)
    );

    let bob_status = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", &bob_cfg)
        .args(["handshake", "status", "--peer", "alice"])
        .output()
        .expect("bob handshake status");
    let bob_status_out = combined_output(&bob_status);
    assert!(
        bob_status_out.contains("event=handshake_status"),
        "missing handshake status marker: {}",
        bob_status_out
    );
    assert!(
        bob_status_out.contains("send_ready=yes"),
        "missing send_ready=yes marker: {}",
        bob_status_out
    );
    assert!(
        !bob_status_out.contains("send_ready_reason="),
        "unexpected send_ready_reason marker: {}",
        bob_status_out
    );

    let msg = base.join("msg.bin");
    fs::write(&msg, b"na0168 pack-reason probe\n").expect("write msg");
    let bob_send = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", &bob_cfg)
        .args([
            "send",
            "--transport",
            "relay",
            "--relay",
            &relay,
            "--to",
            "alice",
            "--file",
            msg.to_str().expect("utf8 path"),
        ])
        .output()
        .expect("bob send after handshake");
    assert!(bob_send.status.success(), "{}", combined_output(&bob_send));
    let bob_send_out = combined_output(&bob_send);
    assert!(
        bob_send_out.contains("event=qsp_pack ok=true"),
        "missing qsp_pack success marker: {}",
        bob_send_out
    );
    assert!(
        !bob_send_out.contains("/v1/"),
        "output leaked URI path: {}",
        bob_send_out
    );

    let recv_dir = base.join("alice_recv");
    ensure_dir_700(&recv_dir);
    let alice_recv = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", &alice_cfg)
        .args([
            "receive",
            "--transport",
            "relay",
            "--relay",
            &relay,
            "--from",
            "bob",
            "--max",
            "4",
            "--out",
            recv_dir.to_str().expect("utf8 path"),
        ])
        .output()
        .expect("alice receive bob reply");
    assert!(
        alice_recv.status.success(),
        "{}",
        combined_output(&alice_recv)
    );
    let alice_recv_out = combined_output(&alice_recv);
    assert!(
        alice_recv_out.contains("event=qsp_unpack ok=true"),
        "missing qsp_unpack success marker: {}",
        alice_recv_out
    );
    assert!(
        !alice_recv_out.contains("qsp_hdr_auth_failed"),
        "unexpected auth failure marker: {}",
        alice_recv_out
    );
}
