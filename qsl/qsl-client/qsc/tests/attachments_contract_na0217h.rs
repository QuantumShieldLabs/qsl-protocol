mod common;

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const ROUTE_TOKEN_BOB: &str = "route_token_bob_abcdefghijklmnopqr";

fn absolute_test_root(tag: &str) -> PathBuf {
    let root = common::unique_test_root(tag);
    if root.is_absolute() {
        root
    } else {
        env::current_dir().expect("cwd").join(root)
    }
}

fn ensure_dir_700(path: &Path) {
    fs::create_dir_all(path).expect("create dir");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(path, fs::Permissions::from_mode(0o700)).expect("chmod 700");
    }
}

fn output_text(out: &std::process::Output) -> String {
    let mut text = String::from_utf8_lossy(&out.stdout).to_string();
    text.push_str(&String::from_utf8_lossy(&out.stderr));
    text
}

fn qsc_base(cfg: &Path) -> Command {
    let mut cmd = common::qsc_std_command();
    cmd.env("QSC_CONFIG_DIR", cfg)
        .env("QSC_MARK_FORMAT", "plain")
        .env("QSC_QSP_SEED", "1")
        .env("QSC_ALLOW_SEED_FALLBACK", "1");
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
        .expect("contacts add");
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

fn run_attachment_send(
    cfg: &Path,
    relay: &str,
    attachment_service: &str,
    to: &str,
    path: &Path,
    with_receipt: bool,
) -> std::process::Output {
    let mut cmd = qsc_base(cfg);
    cmd.args([
        "file",
        "send",
        "--transport",
        "relay",
        "--relay",
        relay,
        "--attachment-service",
        attachment_service,
        "--to",
        to,
        "--path",
        path.to_str().expect("payload path"),
    ]);
    if with_receipt {
        cmd.args(["--receipt", "delivered"]);
    }
    cmd.output().expect("attachment send")
}

fn run_receive(
    cfg: &Path,
    relay: &str,
    mailbox: &str,
    out_dir: &Path,
    attachment_service: Option<&str>,
    emit_receipts: bool,
) -> std::process::Output {
    let mut cmd = qsc_base(cfg);
    cmd.args([
        "receive",
        "--transport",
        "relay",
        "--relay",
        relay,
        "--mailbox",
        mailbox,
        "--from",
        "bob",
        "--max",
        "64",
        "--max-file-size",
        "16777216",
        "--max-file-chunks",
        "512",
        "--out",
        out_dir.to_str().expect("out dir"),
    ]);
    if let Some(service) = attachment_service {
        cmd.args(["--attachment-service", service]);
    }
    if emit_receipts {
        cmd.args(["--emit-receipts", "delivered"]);
    }
    cmd.output().expect("receive")
}

fn setup_pair(base: &Path) -> (PathBuf, PathBuf, PathBuf, PathBuf) {
    let alice_cfg = base.join("alice_cfg");
    let bob_cfg = base.join("bob_cfg");
    let alice_out = base.join("alice_out");
    let bob_out = base.join("bob_out");
    ensure_dir_700(&alice_cfg);
    ensure_dir_700(&bob_cfg);
    ensure_dir_700(&alice_out);
    ensure_dir_700(&bob_out);
    common::init_mock_vault(&alice_cfg);
    common::init_mock_vault(&bob_cfg);
    contacts_add_with_route_token(&alice_cfg, "bob", ROUTE_TOKEN_BOB);
    contacts_add_with_route_token(&bob_cfg, "bob", ROUTE_TOKEN_BOB);
    relay_set_inbox_token(&alice_cfg, ROUTE_TOKEN_BOB);
    relay_set_inbox_token(&bob_cfg, ROUTE_TOKEN_BOB);
    (alice_cfg, bob_cfg, alice_out, bob_out)
}

#[test]
fn attachment_service_roundtrip_confirms_only_after_receiver_ack() {
    let relay = common::start_inbox_server(2 * 1024 * 1024, 512);
    let service = common::start_attachment_server(100 * 1024 * 1024);
    let base = absolute_test_root(&format!("na0217h_roundtrip_{}", std::process::id()));
    ensure_dir_700(&base);
    let (alice_cfg, bob_cfg, alice_out, bob_out) = setup_pair(&base);

    let payload = base.join("stream.bin");
    let payload_bytes = vec![0x5a; 6 * 1024 * 1024 + 321];
    fs::write(&payload, &payload_bytes).expect("write payload");

    let send = run_attachment_send(
        &alice_cfg,
        relay.base_url(),
        service.base_url(),
        "bob",
        &payload,
        true,
    );
    assert!(send.status.success(), "{}", output_text(&send));
    let send_text = output_text(&send);
    assert!(
        send_text.contains("event=attachment_service_commit"),
        "{}",
        send_text
    );
    assert!(
        send_text.contains("QSC_FILE_DELIVERY state=accepted_by_relay"),
        "{}",
        send_text
    );
    assert!(!send_text.contains("state=peer_confirmed"), "{}", send_text);

    let bob_receive = run_receive(
        &bob_cfg,
        relay.base_url(),
        ROUTE_TOKEN_BOB,
        &bob_out,
        Some(service.base_url()),
        true,
    );
    assert!(
        bob_receive.status.success(),
        "{}",
        output_text(&bob_receive)
    );
    let bob_text = output_text(&bob_receive);
    assert!(bob_text.contains("attachment_confirm_send"), "{}", bob_text);
    assert_eq!(
        fs::read(bob_out.join("stream.bin")).expect("read received payload"),
        payload_bytes
    );

    let alice_after_confirm = run_receive(
        &alice_cfg,
        relay.base_url(),
        ROUTE_TOKEN_BOB,
        &alice_out,
        None,
        false,
    );
    assert!(
        alice_after_confirm.status.success(),
        "{}",
        output_text(&alice_after_confirm)
    );
    let alice_text = output_text(&alice_after_confirm);
    assert!(
        alice_text.contains("QSC_FILE_DELIVERY state=peer_confirmed"),
        "{}",
        alice_text
    );
}
