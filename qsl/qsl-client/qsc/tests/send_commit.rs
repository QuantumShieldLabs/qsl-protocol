mod common;
use predicates::prelude::*;
use std::fs;
use std::path::{Path, PathBuf};

/// Create a writable, safe test root without relying on $HOME.
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

fn create_dir_700(path: &Path) {
    let _ = fs::remove_dir_all(path);
    fs::create_dir_all(path).unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(path, fs::Permissions::from_mode(0o700)).unwrap();
    }
}

fn ensure_dir_700(path: &Path) {
    fs::create_dir_all(path).unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(path, fs::Permissions::from_mode(0o700)).unwrap();
    }
}

const ROUTE_TOKEN_PEER: &str = "route_token_peer_abcdefghijklmnopq";

#[test]
fn mock_key_source_remains_retired() {
    let base = safe_test_root().join(format!("na0237a_mock_retired_{}", std::process::id()));
    create_dir_700(&base);

    let cfg = base.join("cfg");
    create_dir_700(&cfg);

    let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("qsc");
    cmd.env("QSC_CONFIG_DIR", &cfg).args([
        "vault",
        "init",
        "--protocol",
        "directional-v1",
        "--non-interactive",
        "--key-source",
        "mock",
    ]);
    cmd.assert()
        .failure()
        .stdout(predicate::str::contains("vault_mock_provider_retired"));
}

fn init_cfg_with_peer_route_token(cfg: &Path) {
    common::init_mock_vault(cfg);
    let mut add = common::qsc_assert_command();
    add.env("QSC_CONFIG_DIR", cfg)
        .env("QSC_QSP_SEED", "1")
        .env("QSC_ALLOW_SEED_FALLBACK", "1")
        .env("QSC_UNSAFE_TEST_SEED_FALLBACK", "1")
        .args([
            "contacts",
            "add",
            "--label",
            "peer",
            "--fp",
            "fp-test",
            "--route-token",
            ROUTE_TOKEN_PEER,
        ]);
    add.assert().success();
}

fn read_send_seq(path: &PathBuf) -> u64 {
    let content = fs::read_to_string(path).expect("read send.state");
    let line = content
        .lines()
        .find(|l| l.trim().starts_with("send_seq="))
        .expect("send_seq present");
    line.trim()
        .strip_prefix("send_seq=")
        .unwrap()
        .parse::<u64>()
        .expect("send_seq parse")
}

#[test]
fn send_failure_no_commit() {
    if !common::directional_case_child("send_failure_no_commit") { return; }
    use qsc::msgqueue::MsgState;
    use sha2::{Digest, Sha512};
    fn hash(bytes: &[u8]) -> Vec<u8> { Sha512::digest(bytes)[..32].to_vec() }
    fn run(cfg: &Path, args: &[&str]) -> std::process::Output {
        common::qsc_std_command().env("QSC_CONFIG_DIR", cfg)
            .env("QSC_DISABLE_KEYCHAIN", "1").env("QSC_MARK_FORMAT", "plain")
            .args(args).output().expect("normal CLI operation")
    }
    fn poll(cfg: &Path, relay: &str, route: &str, peer: &str, out: &Path) {
        eprintln!("NA0780_DIAG phase=receipt_poll begin");
        let result = run(cfg, &["receive", "--transport", "relay", "--relay", relay,
            "--mailbox", route, "--from", peer, "--max", "8", "--out", out.to_str().unwrap()]);
        assert!(result.status.success(), "normal authenticated receive failed");
        eprintln!("NA0780_DIAG phase=receipt_poll end");
    }
    fn outputs(dir: &Path) -> std::collections::BTreeMap<PathBuf, Vec<u8>> {
        fs::read_dir(dir).unwrap().map(|entry| {
            let entry = entry.unwrap();
            assert!(entry.file_type().unwrap().is_file());
            let name = entry.file_name();
            let digest = name.to_str().unwrap().strip_prefix("recv_")
                .and_then(|v| v.strip_suffix(".bin")).expect("stable output name");
            assert!(digest.len() == 64 && digest.bytes().all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b)));
            (entry.path(), fs::read(entry.path()).unwrap())
        }).collect()
    }
    let base = safe_test_root().join(format!("na0070_send_fail_{}", std::process::id()));
    assert!(!base.exists(), "preserve any prior fixture");
    ensure_dir_700(&base);
    let cfg = base.join("alice"); let peer_cfg = base.join("bob");
    let sender_out = base.join("alice_out"); let peer_out = base.join("bob_out");
    for dir in [&cfg, &peer_cfg, &sender_out, &peer_out] { ensure_dir_700(dir); }
    const A: &str = "route_token_alice_abcdefghijklmnop";
    const B: &str = "route_token_bob_abcdefghijklmnopqr";
    eprintln!("NA0780_DIAG phase=authenticated_pair begin");
    common::init_directional_pair(&cfg, "alice", A, &peer_cfg, "bob", B);
    common::directional_pair_assert(&common::directional_state(&cfg, "bob"),
        &common::directional_state(&peer_cfg, "alice"));
    eprintln!("NA0780_DIAG phase=authenticated_pair end");
    assert!(run(&cfg, &["config", "set", "policy-profile", "strict"]).status.success());
    let relay = common::start_inbox_server(1024 * 1024, 16);
    relay.record_directional_pushes();
    relay.set_fail_pushes(1);
    let payload = base.join("msg.bin"); fs::write(&payload, b"hello").unwrap();
    assert!(!cfg.join("outbox.json").exists() && !cfg.join("send.state").exists());
    eprintln!("NA0780_DIAG phase=rejected_send begin");
    // Even a concrete override cannot conceal an invalid stored account policy.
    let config_path=cfg.join("config.txt");let config_before=fs::read(&config_path).unwrap();
    fs::write(&config_path,b"policy_profile=unknown\n").unwrap();
    let refused=run(&cfg,&["send","--transport","relay","--relay",relay.base_url(),
        "--to","bob","--file",payload.to_str().unwrap(),"--pad-bucket","enhanced"]);
    assert!(!refused.status.success());
    assert!(String::from_utf8_lossy(&refused.stdout).contains("directional_configuration_error"));
    assert!(common::directional_queue_records(&cfg,"bob").is_empty());
    assert!(relay.directional_pushes().is_empty(),"configuration failure cannot reach transport");
    fs::write(&config_path,config_before).unwrap();
    let result = run(&cfg, &["send", "--transport", "relay", "--relay", relay.base_url(),
        "--to", "bob", "--file", payload.to_str().unwrap()]);
    let text = String::from_utf8_lossy(&result.stdout);
    eprintln!("NA0780_DIAG phase=rejected_send end cli_success={}", result.status.success());
    assert!(text.contains("event=send_attempt ok=false"), "positive failed-attempt diagnostic");
    let records = common::directional_queue_records(&cfg, "bob");
    assert_eq!(records.len(), 1, "exact intended queue row");
    let saved = &records[0];
    let intent = saved.directional_intent.as_ref().expect("persisted enqueue intent").clone();
    let options: serde_json::Value = serde_json::from_slice(&intent).unwrap();
    assert!(options["padding"]["profile"] == 3 && options["padding"]["size"] == 4096
        && options["padding"]["maximum"] == 4096, "strict resolved once at enqueue");
    assert!(saved.peer == "bob" && saved.body == b"hello" && saved.state == MsgState::Queued);
    assert!(saved.last_error.is_some() && saved.attempts > 0, "failure persisted");
    let wire = saved.ciphertext.as_ref().expect("durable packed ciphertext").clone();
    assert!(saved.channel.is_some());
    let state = common::directional_state(&cfg, "bob");
    let flights: Vec<_> = state["flights"].as_object().unwrap().iter()
        .filter(|(_, f)| f["id"] == saved.msg_id).collect();
    assert_eq!(flights.len(), 1);
    let (slot, flight) = (flights[0].0.clone(), flights[0].1.clone());
    assert!(flight["wire"] == serde_json::to_value(&wire).unwrap() && flight["accepted"] == false);
    assert!(state["completed"].get(&saved.msg_id).is_none());
    let attempts = relay.directional_pushes();
    assert_eq!(attempts.len(), 1, "fault must hit intended application, not maintenance");
    assert!(attempts[0].status == 500 && attempts[0].response_written && attempts[0].body == wire);
    assert!(relay.drain_channel(B).is_empty(), "rejected push never enqueued");
    assert!(outputs(&peer_out).is_empty());
    assert!(!cfg.join("outbox.json").exists() && !cfg.join("send.state").exists());

    assert!(run(&cfg, &["config", "set", "policy-profile", "baseline"]).status.success());
    let unchanged = common::directional_queue_records(&cfg, "bob");
    assert!(unchanged[0].directional_intent.as_ref() == Some(&intent), "policy change cannot alter queued operation");
    relay.set_fail_pushes(0);
    eprintln!("NA0780_DIAG phase=same_operation_retry begin");
    let retry = run(&cfg, &["outbox", "retry", "--relay", relay.base_url()]);
    assert!(retry.status.success(), "normal immediate manual retry failed");
    let accepted = relay.directional_pushes();
    assert!(accepted.iter().any(|a| a.status == 200 && a.response_written && a.body == wire), "byte-exact accepted retry");
    let after_rows = common::directional_queue_records(&cfg, "bob");
    assert_eq!(after_rows.len(), 1);
    let after = &after_rows[0];
    assert!(after.directional_intent.as_ref() == Some(&intent), "restart/retry preserves profile, size and limits");
    assert!(after.msg_id == saved.msg_id && after.seq == saved.seq && after.peer == saved.peer && after.body == saved.body);
    assert!(after.state == MsgState::Sent, "relay acceptance is not delivery");
    let retried = common::directional_state(&cfg, "bob");
    assert!(retried["core"] == state["core"], "retry did not reseal or consume another slot");
    for field in ["id", "epoch", "slot", "wire", "body_hash"] {
        assert!(retried["flights"][&slot][field] == flight[field], "same durable operation field");
    }
    assert!(retried["flights"][&slot]["accepted"] == true);
    eprintln!("NA0780_DIAG phase=same_operation_retry end");
    poll(&peer_cfg, relay.base_url(), B, "alice", &peer_out);
    let delivered_outputs = outputs(&peer_out);
    assert!(delivered_outputs.len() == 1 && delivered_outputs.values().next().unwrap() == b"hello");
    let received = common::directional_state(&peer_cfg, "alice");
    let disposition = &received["dispositions"][&slot];
    assert!(disposition["hash"] == serde_json::to_value(hash(&wire)).unwrap(), "authenticated exact wire disposition");
    let receipt: Vec<u8> = serde_json::from_value(disposition["receipt"].clone()).unwrap();
    assert!(!receipt.is_empty());
    assert!(relay.directional_pushes().iter().any(|a| a.status == 200 && a.response_written && a.body == receipt), "actual exact committed receipt sent");

    // Deliberate duplicate while the exact receipt remains live, before closure
    // retirement. This is an in-case replay arm, never an automatic test retry.
    relay.enqueue_raw(B, wire.clone());
    poll(&peer_cfg, relay.base_url(), B, "alice", &peer_out);
    assert!(outputs(&peer_out) == delivered_outputs, "replay cannot replace/add an application");
    let replayed = common::directional_state(&peer_cfg, "alice");
    assert!(replayed["dispositions"][&slot] == received["dispositions"][&slot], "same authenticated disposition and receipt");
    let mut completed = false;
    for _ in 0..8 {
        poll(&cfg, relay.base_url(), A, "bob", &sender_out);
        let current = common::directional_queue_records(&cfg, "bob");
        assert_eq!(current.len(), 1);
        assert!(current[0].msg_id == saved.msg_id && current[0].seq == saved.seq
            && current[0].peer == saved.peer && current[0].body == saved.body);
        if current[0].state == MsgState::Delivered {
            assert!(current[0].directional_wire_hash.map(|h| h.to_vec()) == Some(hash(&wire)));
            let s = common::directional_state(&cfg, "bob");
            assert!(s["flights"].get(&slot).is_none(), "authenticated receipt completed original flight");
            completed = true; break;
        }
        poll(&peer_cfg, relay.base_url(), B, "alice", &peer_out);
    }
    assert!(completed, "authenticated receipt must project Delivered within bounded drains");
    assert!(outputs(&peer_out) == delivered_outputs && outputs(&sender_out).is_empty());
    assert!(!cfg.join("outbox.json").exists() && !cfg.join("send.state").exists());

}

#[test]
fn outbox_commit_advances_once() {
    let base = safe_test_root().join(format!("na0070_send_commit_{}", std::process::id()));
    create_dir_700(&base);

    let cfg = base.join("cfg");
    create_dir_700(&cfg);
    init_cfg_with_peer_route_token(&cfg);

    let payload = cfg.join("msg.bin");
    fs::write(&payload, b"hello").expect("write payload");

    let outbox = cfg.join("outbox.json");
    let send_state = cfg.join("send.state");

    let relay = common::start_inbox_server(1024 * 1024, 8);
    let relay_addr = relay.base_url().to_string();

    let mut cmd = common::qsc_assert_command();
    cmd.env("QSC_CONFIG_DIR", &cfg)
        .env("QSC_QSP_SEED", "1")
        .env("QSC_ALLOW_SEED_FALLBACK", "1")
        .env("QSC_UNSAFE_TEST_SEED_FALLBACK", "1")
        .env("QSC_MARK_FORMAT", "plain")
        .args([
            "send",
            "--transport",
            "relay",
            "--relay",
            relay_addr.as_str(),
            "--to",
            "peer",
            "--file",
            payload.to_str().unwrap(),
        ]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("event=send_commit"));

    assert!(!outbox.exists());
    assert!(send_state.exists());
    assert_eq!(read_send_seq(&send_state), 1);

    let mut cmd = common::qsc_assert_command();
    cmd.env("QSC_CONFIG_DIR", &cfg)
        .env("QSC_QSP_SEED", "1")
        .env("QSC_ALLOW_SEED_FALLBACK", "1")
        .env("QSC_UNSAFE_TEST_SEED_FALLBACK", "1")
        .env("QSC_MARK_FORMAT", "plain")
        .args([
            "send",
            "--transport",
            "relay",
            "--relay",
            relay_addr.as_str(),
            "--to",
            "peer",
            "--file",
            payload.to_str().unwrap(),
        ]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("event=send_commit"));

    assert!(!outbox.exists());
    assert_eq!(read_send_seq(&send_state), 2);
}
