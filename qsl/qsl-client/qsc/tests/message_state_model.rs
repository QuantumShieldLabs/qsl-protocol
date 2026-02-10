mod common;

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

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
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("qsc"));
    cmd.env("QSC_CONFIG_DIR", cfg)
        .env("QSC_MARK_FORMAT", "plain")
        .env("QSC_QSP_SEED", "1")
        .env("QSC_ALLOW_SEED_FALLBACK", "1");
    cmd
}

fn timeline_first_item_id_and_state(text: &str) -> Option<(String, String)> {
    for line in text.lines() {
        if !line.contains("event=timeline_item") {
            continue;
        }
        let mut id = None;
        let mut state = None;
        for part in line.split_whitespace() {
            if let Some(v) = part.strip_prefix("id=") {
                id = Some(v.to_string());
            }
            if let Some(v) = part.strip_prefix("state=") {
                state = Some(v.to_string());
            }
        }
        if let (Some(i), Some(s)) = (id, state) {
            return Some((i, s));
        }
    }
    None
}

fn assert_no_secrets(text: &str) {
    let upper = text.to_ascii_uppercase();
    for forbidden in ["TOKEN", "SECRET", "PASS", "PRIVATE", "BEARER", "CREDENTIAL"] {
        assert!(
            !upper.contains(forbidden),
            "found forbidden pattern {} in output: {}",
            forbidden,
            text
        );
    }
}

fn write_ack_payload(path: &Path, msg_id: &str) {
    let payload = format!(
        "{{\"v\":1,\"t\":\"ack\",\"kind\":\"delivered\",\"msg_id\":\"{}\"}}",
        msg_id
    );
    fs::write(path, payload.as_bytes()).unwrap();
}

#[test]
fn honest_delivery_requires_explicit_ack() {
    let server = common::start_inbox_server(1024 * 1024, 16);
    let base = safe_test_root().join(format!("na0118_honest_delivery_{}", std::process::id()));
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

    let payload = base.join("msg.bin");
    fs::write(&payload, b"na0118-honest-delivery").unwrap();

    let send = qsc_base(&alice_cfg)
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
        .expect("send");
    assert!(send.status.success(), "{}", output_text(&send));

    let alice_list_before = qsc_base(&alice_cfg)
        .args(["timeline", "list", "--peer", "bob", "--limit", "10"])
        .output()
        .expect("timeline list before");
    let before_text = output_text(&alice_list_before);
    assert!(alice_list_before.status.success(), "{}", before_text);
    let (_, state_before) = timeline_first_item_id_and_state(&before_text).expect("timeline item");
    assert_eq!(state_before, "SENT", "{}", before_text);

    let bob_recv = qsc_base(&bob_cfg)
        .args([
            "receive",
            "--transport",
            "relay",
            "--relay",
            server.base_url(),
            "--mailbox",
            "bob",
            "--from",
            "bob",
            "--max",
            "1",
            "--out",
            bob_out.to_str().unwrap(),
        ])
        .output()
        .expect("bob receive");
    assert!(bob_recv.status.success(), "{}", output_text(&bob_recv));

    let alice_recv = qsc_base(&alice_cfg)
        .args([
            "receive",
            "--transport",
            "relay",
            "--relay",
            server.base_url(),
            "--mailbox",
            "bob",
            "--from",
            "bob",
            "--max",
            "1",
            "--out",
            alice_out.to_str().unwrap(),
        ])
        .output()
        .expect("alice receive");
    assert!(alice_recv.status.success(), "{}", output_text(&alice_recv));
    let alice_recv_text = output_text(&alice_recv);
    assert!(
        !alice_recv_text.contains("event=receipt_recv"),
        "{}",
        alice_recv_text
    );
    assert!(
        !alice_recv_text.contains("to=DELIVERED"),
        "{}",
        alice_recv_text
    );

    let alice_list_after = qsc_base(&alice_cfg)
        .args(["timeline", "list", "--peer", "bob", "--limit", "10"])
        .output()
        .expect("timeline list after");
    let after_text = output_text(&alice_list_after);
    assert!(alice_list_after.status.success(), "{}", after_text);
    let (_, state_after) = timeline_first_item_id_and_state(&after_text).expect("timeline item");
    assert_eq!(state_after, "SENT", "{}", after_text);
}

#[test]
fn wrong_peer_ack_rejected_no_mutation() {
    let server = common::start_inbox_server(1024 * 1024, 16);
    let base = safe_test_root().join(format!("na0118_wrong_peer_ack_{}", std::process::id()));
    create_dir_700(&base);
    let alice_cfg = base.join("alice_cfg");
    let bob_cfg = base.join("bob_cfg");
    let mallory_cfg = base.join("mallory_cfg");
    let alice_out = base.join("alice_out");
    create_dir_700(&alice_cfg);
    create_dir_700(&bob_cfg);
    create_dir_700(&mallory_cfg);
    create_dir_700(&alice_out);
    common::init_mock_vault(&alice_cfg);
    common::init_mock_vault(&bob_cfg);
    common::init_mock_vault(&mallory_cfg);

    let payload = base.join("msg.bin");
    fs::write(&payload, b"na0118-wrong-peer-ack").unwrap();

    let send = qsc_base(&alice_cfg)
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
        .expect("send");
    assert!(send.status.success(), "{}", output_text(&send));

    let list = qsc_base(&alice_cfg)
        .args(["timeline", "list", "--peer", "bob", "--limit", "10"])
        .output()
        .expect("timeline list");
    let list_text = output_text(&list);
    let (msg_id, state_before) = timeline_first_item_id_and_state(&list_text).expect("timeline");
    assert_eq!(state_before, "SENT", "{}", list_text);

    let forged = base.join("forged_ack.json");
    write_ack_payload(&forged, &msg_id);
    let forge_send = qsc_base(&mallory_cfg)
        .args([
            "send",
            "--transport",
            "relay",
            "--relay",
            server.base_url(),
            "--to",
            "alice",
            "--file",
            forged.to_str().unwrap(),
        ])
        .output()
        .expect("forged send");
    assert!(forge_send.status.success(), "{}", output_text(&forge_send));

    let recv = qsc_base(&alice_cfg)
        .args([
            "receive",
            "--transport",
            "relay",
            "--relay",
            server.base_url(),
            "--mailbox",
            "bob",
            "--from",
            "mallory",
            "--max",
            "1",
            "--out",
            alice_out.to_str().unwrap(),
        ])
        .output()
        .expect("alice receive forged ack");
    assert!(!recv.status.success(), "{}", output_text(&recv));
    let recv_text = output_text(&recv);
    assert!(
        recv_text.contains("event=qsp_unpack code=qsp_hdr_auth_failed ok=false"),
        "{}",
        recv_text
    );

    let list_after = qsc_base(&alice_cfg)
        .args(["timeline", "list", "--peer", "bob", "--limit", "10"])
        .output()
        .expect("timeline list after");
    let after_text = output_text(&list_after);
    let (_, state_after) = timeline_first_item_id_and_state(&after_text).expect("timeline after");
    assert_eq!(state_after, "SENT", "{}", after_text);
}

#[test]
fn replay_ack_does_not_advance_state() {
    let server = common::start_inbox_server(1024 * 1024, 16);
    let base = safe_test_root().join(format!("na0118_replay_ack_{}", std::process::id()));
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

    let payload = base.join("msg.bin");
    fs::write(&payload, b"na0118-replay-ack").unwrap();

    let send = qsc_base(&alice_cfg)
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
        .expect("send");
    assert!(send.status.success(), "{}", output_text(&send));

    let list1 = qsc_base(&alice_cfg)
        .args(["timeline", "list", "--peer", "bob", "--limit", "10"])
        .output()
        .expect("timeline list sent");
    let list1_text = output_text(&list1);
    let (msg_id, _) = timeline_first_item_id_and_state(&list1_text).expect("timeline item");

    let bob_recv = qsc_base(&bob_cfg)
        .args([
            "receive",
            "--transport",
            "relay",
            "--relay",
            server.base_url(),
            "--mailbox",
            "bob",
            "--from",
            "bob",
            "--max",
            "1",
            "--out",
            bob_out.to_str().unwrap(),
            "--emit-receipts",
            "delivered",
        ])
        .output()
        .expect("bob recv ack");
    assert!(bob_recv.status.success(), "{}", output_text(&bob_recv));

    let alice_recv = qsc_base(&alice_cfg)
        .args([
            "receive",
            "--transport",
            "relay",
            "--relay",
            server.base_url(),
            "--mailbox",
            "bob",
            "--from",
            "bob",
            "--max",
            "1",
            "--out",
            alice_out.to_str().unwrap(),
        ])
        .output()
        .expect("alice recv ack");
    let alice_recv_text = output_text(&alice_recv);
    assert!(alice_recv.status.success(), "{}", alice_recv_text);
    assert!(alice_recv_text.contains("event=receipt_recv kind=delivered msg_id=<redacted>"));

    let list2 = qsc_base(&alice_cfg)
        .args(["timeline", "list", "--peer", "bob", "--limit", "10"])
        .output()
        .expect("timeline list delivered");
    let list2_text = output_text(&list2);
    let (_, delivered_state) =
        timeline_first_item_id_and_state(&list2_text).expect("timeline item delivered");
    assert_eq!(delivered_state, "DELIVERED", "{}", list2_text);

    let replay = base.join("replay_ack.json");
    write_ack_payload(&replay, &msg_id);
    let replay_send = qsc_base(&bob_cfg)
        .args([
            "send",
            "--transport",
            "relay",
            "--relay",
            server.base_url(),
            "--to",
            "bob",
            "--file",
            replay.to_str().unwrap(),
        ])
        .output()
        .expect("replay send");
    assert!(
        replay_send.status.success(),
        "{}",
        output_text(&replay_send)
    );

    let replay_recv = qsc_base(&alice_cfg)
        .args([
            "receive",
            "--transport",
            "relay",
            "--relay",
            server.base_url(),
            "--mailbox",
            "bob",
            "--from",
            "bob",
            "--max",
            "1",
            "--out",
            alice_out.to_str().unwrap(),
        ])
        .output()
        .expect("replay recv");
    let replay_recv_text = output_text(&replay_recv);
    assert!(replay_recv.status.success(), "{}", replay_recv_text);
    assert!(
        replay_recv_text.contains(&format!(
            "event=message_state_reject code=state_duplicate reason=state_duplicate id={}",
            msg_id
        )),
        "{}",
        replay_recv_text
    );

    let list3 = qsc_base(&alice_cfg)
        .args(["timeline", "list", "--peer", "bob", "--limit", "10"])
        .output()
        .expect("timeline list after replay");
    let list3_text = output_text(&list3);
    let (_, final_state) =
        timeline_first_item_id_and_state(&list3_text).expect("timeline item final");
    assert_eq!(final_state, "DELIVERED", "{}", list3_text);
}

#[test]
fn state_markers_are_deterministic_and_secret_safe() {
    fn run_once(tag: &str) -> String {
        let server = common::start_inbox_server(1024 * 1024, 16);
        let base =
            safe_test_root().join(format!("na0118_determinism_{}_{}", tag, std::process::id()));
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

        let payload = base.join("msg.bin");
        fs::write(&payload, b"na0118-determinism").unwrap();

        let send = qsc_base(&alice_cfg)
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
            .expect("send");
        assert!(send.status.success(), "{}", output_text(&send));

        let bob_recv = qsc_base(&bob_cfg)
            .args([
                "receive",
                "--transport",
                "relay",
                "--relay",
                server.base_url(),
                "--mailbox",
                "bob",
                "--from",
                "bob",
                "--max",
                "1",
                "--out",
                bob_out.to_str().unwrap(),
                "--emit-receipts",
                "delivered",
            ])
            .output()
            .expect("bob receive");
        assert!(bob_recv.status.success(), "{}", output_text(&bob_recv));

        let alice_recv = qsc_base(&alice_cfg)
            .args([
                "receive",
                "--transport",
                "relay",
                "--relay",
                server.base_url(),
                "--mailbox",
                "bob",
                "--from",
                "bob",
                "--max",
                "1",
                "--out",
                alice_out.to_str().unwrap(),
            ])
            .output()
            .expect("alice receive");
        assert!(alice_recv.status.success(), "{}", output_text(&alice_recv));

        let mut all = String::new();
        all.push_str(&output_text(&send));
        all.push_str(&output_text(&bob_recv));
        all.push_str(&output_text(&alice_recv));
        assert_no_secrets(&all);

        all.lines()
            .filter(|line| line.contains("event=message_state_"))
            .collect::<Vec<_>>()
            .join("\n")
    }

    let a = run_once("a");
    let b = run_once("b");
    assert_eq!(
        a, b,
        "state markers not deterministic\nA:\n{}\nB:\n{}",
        a, b
    );
}
