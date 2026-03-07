mod common;

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::time::{SystemTime, UNIX_EPOCH};

const ROUTE_ALICE: &str = "route_token_alice_na0177_primary";
const ROUTE_BOB_A: &str = "route_token_boba_na0177_primary";
const ROUTE_BOB_B: &str = "route_token_bobb_na0177_primary";

fn ensure_dir_700(path: &Path) {
    fs::create_dir_all(path).expect("create dir");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(path, fs::Permissions::from_mode(0o700)).expect("chmod 700");
    }
}

fn unique_test_dir(tag: &str) -> PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock")
        .as_nanos();
    std::env::current_dir()
        .expect("cwd")
        .join("target")
        .join("qsc-tests")
        .join(format!("{tag}-{}-{nonce}", std::process::id()))
}

fn output_text(out: &Output) -> String {
    let mut s = String::from_utf8_lossy(&out.stdout).to_string();
    s.push_str(&String::from_utf8_lossy(&out.stderr));
    s
}

fn assert_no_leaks(s: &str) {
    assert_eq!(s.matches("/v1/").count(), 0, "unexpected /v1/ output: {s}");
    let mut run = 0usize;
    for ch in s.chars() {
        if ch.is_ascii_hexdigit() {
            run = run.saturating_add(1);
            assert!(run < 32, "unexpected long-hex output: {s}");
        } else {
            run = 0;
        }
    }
}

fn qsc(cfg: &Path) -> Command {
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("qsc"));
    cmd.env("QSC_CONFIG_DIR", cfg)
        .env("QSC_MARK_FORMAT", "plain")
        .env("QSC_ALLOW_SEED_FALLBACK", "1")
        .env("QSC_QSP_SEED", "17")
        .env("QSC_TEST_MODE", "1");
    cmd
}

fn run(cfg: &Path, args: &[&str]) -> Output {
    qsc(cfg).args(args).output().expect("run qsc")
}

fn init_cfg(cfg: &Path) {
    ensure_dir_700(cfg);
    common::init_mock_vault(cfg);
}

fn relay_inbox_set(cfg: &Path, token: &str) {
    let out = run(cfg, &["relay", "inbox-set", "--token", token]);
    assert!(out.status.success(), "{}", output_text(&out));
}

fn contacts_add_trusted(cfg: &Path, label: &str, fp: &str, route_token: &str) {
    let add = run(
        cfg,
        &[
            "contacts",
            "add",
            "--label",
            label,
            "--fp",
            fp,
            "--route-token",
            route_token,
            "--verify",
        ],
    );
    assert!(add.status.success(), "{}", output_text(&add));
    let list = run(cfg, &["contacts", "device", "list", "--label", label]);
    assert!(list.status.success(), "{}", output_text(&list));
    let list_text = output_text(&list);
    let device = list_text
        .lines()
        .find(|line| line.starts_with("device="))
        .and_then(|line| {
            line.split_whitespace()
                .find_map(|tok| tok.strip_prefix("device="))
        })
        .unwrap_or_else(|| panic!("missing device id in output: {list_text}"))
        .to_string();
    let trust = run(
        cfg,
        &[
            "contacts",
            "device",
            "trust",
            "--label",
            label,
            "--device",
            device.as_str(),
            "--confirm",
        ],
    );
    assert!(trust.status.success(), "{}", output_text(&trust));
}

fn add_two_bob_devices_on_alice(cfg: &Path) -> (String, String) {
    let add = run(
        cfg,
        &[
            "contacts",
            "add",
            "--label",
            "bob",
            "--fp",
            "ABCD-EFGH-JKMP-QRST-V",
            "--route-token",
            ROUTE_BOB_A,
            "--verify",
        ],
    );
    assert!(add.status.success(), "{}", output_text(&add));
    let add_second = run(
        cfg,
        &[
            "contacts",
            "device",
            "add",
            "--label",
            "bob",
            "--fp",
            "BBBB-CCCC-DDDD-EEEE-F",
            "--route-token",
            ROUTE_BOB_B,
        ],
    );
    assert!(add_second.status.success(), "{}", output_text(&add_second));
    let list = run(cfg, &["contacts", "device", "list", "--label", "bob"]);
    assert!(list.status.success(), "{}", output_text(&list));
    let mut devices: Vec<String> = output_text(&list)
        .lines()
        .filter_map(|line| {
            if !line.starts_with("device=") {
                return None;
            }
            line.split_whitespace()
                .find_map(|tok| tok.strip_prefix("device=").map(|v| v.to_string()))
        })
        .collect();
    devices.sort();
    assert_eq!(devices.len(), 2, "expected two bob devices: {devices:?}");
    for dev in devices.iter() {
        let trust = run(
            cfg,
            &[
                "contacts",
                "device",
                "trust",
                "--label",
                "bob",
                "--device",
                dev.as_str(),
                "--confirm",
            ],
        );
        assert!(trust.status.success(), "{}", output_text(&trust));
    }
    (devices[0].clone(), devices[1].clone())
}

fn set_primary(cfg: &Path, label: &str, device: &str) {
    let out = run(
        cfg,
        &[
            "contacts",
            "device",
            "primary",
            "set",
            "--label",
            label,
            "--device",
            device,
            "--confirm",
        ],
    );
    assert!(out.status.success(), "{}", output_text(&out));
}

fn timeline_first_out_id(cfg: &Path, peer: &str) -> String {
    let out = run(cfg, &["timeline", "list", "--peer", peer, "--limit", "20"]);
    assert!(out.status.success(), "{}", output_text(&out));
    let text = output_text(&out);
    for line in text.lines() {
        if !line.contains("event=timeline_item") {
            continue;
        }
        if !line.contains(" dir=out ") {
            continue;
        }
        if let Some(id) = line
            .split_whitespace()
            .find_map(|tok| tok.strip_prefix("id=").map(|v| v.to_string()))
        {
            return id;
        }
    }
    panic!("missing outbound timeline item: {text}");
}

#[test]
fn message_wrong_device_receipt_ignored_then_correct_device_confirms() {
    let root = unique_test_dir("na0177_peer_confirm_msg_wrong_device");
    let alice_cfg = root.join("alice_cfg");
    let bob_a_cfg = root.join("bob_a_cfg");
    let bob_b_cfg = root.join("bob_b_cfg");
    init_cfg(&alice_cfg);
    init_cfg(&bob_a_cfg);
    init_cfg(&bob_b_cfg);

    relay_inbox_set(&alice_cfg, ROUTE_ALICE);
    relay_inbox_set(&bob_a_cfg, ROUTE_BOB_A);
    relay_inbox_set(&bob_b_cfg, ROUTE_BOB_B);

    let (d1, d2) = add_two_bob_devices_on_alice(&alice_cfg);
    set_primary(&alice_cfg, "bob", d1.as_str());

    contacts_add_trusted(&bob_a_cfg, "alice", "AAAA-BBBB-CCCC-DDDD-E", ROUTE_ALICE);
    contacts_add_trusted(&bob_b_cfg, "alice", "AAAA-BBBB-CCCC-DDDD-E", ROUTE_ALICE);

    let server = common::start_inbox_server(1024 * 1024, 64);
    let msg_path = root.join("msg.bin");
    fs::write(&msg_path, b"hello-peer-confirm").expect("write msg");

    let send = run(
        &alice_cfg,
        &[
            "send",
            "--transport",
            "relay",
            "--relay",
            server.base_url(),
            "--to",
            "bob",
            "--file",
            msg_path.to_str().expect("utf8"),
        ],
    );
    assert!(send.status.success(), "{}", output_text(&send));
    let send_text = output_text(&send);
    assert!(
        send_text.contains(
            format!(
                "QSC_DELIVERY state=accepted_by_relay policy=primary_only peer=bob device={}",
                d1
            )
            .as_str()
        ),
        "{}",
        send_text
    );
    let msg_id = timeline_first_out_id(&alice_cfg, "bob");

    let wrong_ack_path = root.join("wrong_ack.json");
    let wrong_ack = format!(
        "{{\"v\":1,\"t\":\"ack\",\"kind\":\"delivered\",\"msg_id\":\"{}\"}}",
        msg_id
    );
    fs::write(&wrong_ack_path, wrong_ack.as_bytes()).expect("write wrong ack");
    let wrong_inject = run(
        &alice_cfg,
        &[
            "util",
            "receipt-apply",
            "--peer",
            "bob",
            "--channel",
            format!("bob#{}", d2).as_str(),
            "--msg-id",
            msg_id.as_str(),
        ],
    );
    assert!(
        wrong_inject.status.success(),
        "{}",
        output_text(&wrong_inject)
    );
    let wrong_text = output_text(&wrong_inject);
    assert!(
        wrong_text.contains(
            format!(
                "QSC_RECEIPT_IGNORED reason=wrong_device peer=bob device={}",
                d2
            )
            .as_str()
        ),
        "{}",
        wrong_text
    );

    let correct_inject = run(
        &alice_cfg,
        &[
            "util",
            "receipt-apply",
            "--peer",
            "bob",
            "--channel",
            format!("bob#{}", d1).as_str(),
            "--msg-id",
            msg_id.as_str(),
        ],
    );
    assert!(
        correct_inject.status.success(),
        "{}",
        output_text(&correct_inject)
    );
    let correct_text = output_text(&correct_inject);
    assert!(
        correct_text.contains(
            format!(
                "QSC_DELIVERY state=peer_confirmed policy=primary_only peer=bob device={}",
                d1
            )
            .as_str()
        ),
        "{}",
        correct_text
    );
    assert_no_leaks(&send_text);
    assert_no_leaks(&wrong_text);
    assert_no_leaks(&correct_text);
}

#[test]
fn message_primary_switch_does_not_rebind_outstanding_item() {
    let root = unique_test_dir("na0177_peer_confirm_primary_switch");
    let alice_cfg = root.join("alice_cfg");
    init_cfg(&alice_cfg);
    relay_inbox_set(&alice_cfg, ROUTE_ALICE);
    let (d1, d2) = add_two_bob_devices_on_alice(&alice_cfg);
    set_primary(&alice_cfg, "bob", d1.as_str());

    let server = common::start_inbox_server(1024 * 1024, 64);
    let payload = root.join("msg.bin");
    fs::write(&payload, b"hello-switch").expect("write payload");

    let send1 = run(
        &alice_cfg,
        &[
            "send",
            "--transport",
            "relay",
            "--relay",
            server.base_url(),
            "--to",
            "bob",
            "--file",
            payload.to_str().expect("utf8"),
        ],
    );
    assert!(send1.status.success(), "{}", output_text(&send1));
    let msg_id = timeline_first_out_id(&alice_cfg, "bob");

    set_primary(&alice_cfg, "bob", d2.as_str());

    let confirm_old = run(
        &alice_cfg,
        &[
            "util",
            "receipt-apply",
            "--peer",
            "bob",
            "--channel",
            format!("bob#{}", d1).as_str(),
            "--msg-id",
            msg_id.as_str(),
        ],
    );
    assert!(
        confirm_old.status.success(),
        "{}",
        output_text(&confirm_old)
    );
    let old_text = output_text(&confirm_old);
    assert!(
        old_text.contains(
            format!(
                "QSC_DELIVERY state=peer_confirmed policy=primary_only peer=bob device={}",
                d1
            )
            .as_str()
        ),
        "{}",
        old_text
    );

    let send2 = run(
        &alice_cfg,
        &[
            "send",
            "--transport",
            "relay",
            "--relay",
            server.base_url(),
            "--to",
            "bob",
            "--file",
            payload.to_str().expect("utf8"),
        ],
    );
    assert!(send2.status.success(), "{}", output_text(&send2));
    let send2_text = output_text(&send2);
    assert!(
        send2_text.contains(
            format!(
                "QSC_DELIVERY state=accepted_by_relay policy=primary_only peer=bob device={}",
                d2
            )
            .as_str()
        ),
        "{}",
        send2_text
    );
    assert_no_leaks(&old_text);
    assert_no_leaks(&send2_text);
}

#[test]
fn file_wrong_device_confirmation_ignored_then_correct_device_confirms() {
    let root = unique_test_dir("na0177_peer_confirm_file_wrong_device");
    let alice_cfg = root.join("alice_cfg");
    let bob_a_cfg = root.join("bob_a_cfg");
    init_cfg(&alice_cfg);
    init_cfg(&bob_a_cfg);
    relay_inbox_set(&alice_cfg, ROUTE_ALICE);
    relay_inbox_set(&bob_a_cfg, ROUTE_BOB_A);
    let (d1, d2) = add_two_bob_devices_on_alice(&alice_cfg);
    set_primary(&alice_cfg, "bob", d1.as_str());
    contacts_add_trusted(&bob_a_cfg, "alice", "AAAA-BBBB-CCCC-DDDD-E", ROUTE_ALICE);

    let server = common::start_inbox_server(1024 * 1024, 64);
    let file_path = root.join("doc.bin");
    let payload = b"phasec-file-confirm";
    fs::write(&file_path, payload.as_slice()).expect("write file");

    let send = run(
        &alice_cfg,
        &[
            "file",
            "send",
            "--transport",
            "relay",
            "--relay",
            server.base_url(),
            "--to",
            "bob",
            "--path",
            file_path.to_str().expect("utf8"),
            "--chunk-size",
            "4096",
            "--receipt",
            "delivered",
        ],
    );
    assert!(send.status.success(), "{}", output_text(&send));
    let send_text = output_text(&send);

    let wrong = run(
        &alice_cfg,
        &[
            "util",
            "receipt-apply",
            "--peer",
            "bob",
            "--channel",
            format!("bob#{}", d2).as_str(),
            "--file-id",
            "latest",
            "--confirm-id",
            "dummy-confirm",
        ],
    );
    assert!(wrong.status.success(), "{}", output_text(&wrong));
    let wrong_text = output_text(&wrong);
    assert!(
        wrong_text.contains(
            format!(
                "QSC_RECEIPT_IGNORED reason=wrong_device peer=bob device={}",
                d2
            )
            .as_str()
        ),
        "{}",
        wrong_text
    );

    let correct = run(
        &alice_cfg,
        &[
            "util",
            "receipt-apply",
            "--peer",
            "bob",
            "--channel",
            format!("bob#{}", d1).as_str(),
            "--file-id",
            "latest",
            "--confirm-id",
            "auto",
        ],
    );
    assert!(correct.status.success(), "{}", output_text(&correct));
    let correct_text = output_text(&correct);
    assert!(
        correct_text.contains(
            format!(
                "QSC_FILE_DELIVERY state=peer_confirmed policy=primary_only peer=bob device={}",
                d1
            )
            .as_str()
        ),
        "{}",
        correct_text
    );
    assert_no_leaks(&send_text);
    assert_no_leaks(&wrong_text);
    assert_no_leaks(&correct_text);
}
