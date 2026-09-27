// NA-0785 PLAN F03 / S9 -- C01 O9 DISTINCT CODES F03 OWNS, ON THE REAL RECEIVE PATH.
//
//   e1 an authenticated frame whose typed body carries the wrong NDI magic is refused
//      INTEGRATION_MAGIC (was INTEGRATION_PROFILE, the code of a profile mismatch).
//   e5 an authenticated frame whose typed body is a file kind (1..=4) is refused
//      INTEGRATION_FILE_GATED on its kind byte, before any file-shape validation (a malformed
//      kind-4 payload was INTEGRATION_FILE_SHAPE, a bad request byte INTEGRATION_FILE_REQUEST).
//
// Each refusal keeps its DISPOSITION: the receive loop skips the frame as an expected
// non-admission (protocol_state expected_non_admission), delivers the honest message beside
// it, and exits 0. e2 (QueuedIntent's foreign profile -> INTENT_PROFILE) has no test seam
// outside the crate; it is pinned in src/directional_delivery.rs's own test module.
//
// The authenticated hostile frames come from the existing acceptance seams
// (na0780_test_hostile_wire, na0780_test_receive_response); the file needs the crate's
// existing `na0780-test-hooks` feature and is empty without it.
// A synthetic local run; it says nothing about production or the real relay deployment.
#![cfg(feature = "na0780-test-hooks")]

mod common;

use common::{profile, PairRelay, TEST_MOCK_VAULT_PASSPHRASE, TEST_MOCK_VAULT_PASSPHRASE_ENV};
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{Duration, Instant};

const CASE_ENV: &str = "QSC_F03_S9_CASE";

/// Run `case` alone in a child of this test binary: the in-process seams select a vault and
/// unlock it, so the parent never does either.
fn isolated_child(case: &str) -> bool {
    if let Ok(selected) = std::env::var(CASE_ENV) {
        assert_eq!(selected, case, "exact child case guard");
        return true;
    }
    let mut child = Command::new(std::env::current_exe().unwrap())
        .args([case, "--exact", "--nocapture", "--test-threads=1"])
        .env(CASE_ENV, case)
        .env("QSC_DISABLE_KEYCHAIN", "1")
        .env_remove(profile::ACTIVE.location_env)
        .env_remove("QSC_PASSPHRASE")
        .env_remove(TEST_MOCK_VAULT_PASSPHRASE_ENV)
        .spawn()
        .expect("spawn isolated case");
    let deadline = Instant::now() + Duration::from_secs(590);
    let status = loop {
        if let Some(status) = child.try_wait().expect("observe isolated case") {
            break status;
        }
        if Instant::now() >= deadline {
            let _ = child.kill();
            let _ = child.wait();
            panic!("isolated case deadline");
        }
        std::thread::sleep(Duration::from_millis(25));
    };
    assert!(status.success(), "isolated case {case} failed");
    false
}

fn tree(root: &Path) -> BTreeMap<PathBuf, Vec<u8>> {
    fn walk(root: &Path, path: &Path, out: &mut BTreeMap<PathBuf, Vec<u8>>) {
        for entry in fs::read_dir(path).unwrap() {
            let entry = entry.unwrap();
            let kind = entry.file_type().unwrap();
            if kind.is_dir() {
                walk(root, &entry.path(), out);
            } else {
                let rel = entry.path().strip_prefix(root).unwrap().to_owned();
                out.insert(rel, fs::read(entry.path()).unwrap());
            }
        }
    }
    let mut out = BTreeMap::new();
    if root.exists() {
        walk(root, root, &mut out);
    }
    out
}

/// The sender's authenticated directional core, read through a read-only vault session.
fn sender_core(cfg: &Path, peer: &str) -> String {
    std::env::set_var(profile::ACTIVE.location_env, cfg);
    let session = qsc::vault::open_session_with_passphrase(TEST_MOCK_VAULT_PASSPHRASE)
        .expect("authenticated read-only session");
    let raw = qsc::vault::session_get(
        &session,
        &format!("na0780_directional_transaction_v2/{peer}"),
    )
    .unwrap()
    .expect("authenticated directional transaction");
    let state: serde_json::Value = serde_json::from_str(&raw).unwrap();
    assert_eq!(
        state["version"],
        profile::ACTIVE.id,
        "sender state carries the ACTIVE profile"
    );
    serde_json::to_string(&state["core"]).unwrap()
}

#[test]
fn s9_receive_refusals_carry_distinct_codes_and_keep_their_disposition() {
    if !isolated_child("s9_receive_refusals_carry_distinct_codes_and_keep_their_disposition") {
        return;
    }
    let relay = common::start_inbox_server(1024 * 1024, 32);
    let (a_route, b_route) = ("f03_s9_route_alice_01234567", "f03_s9_route_bob_0123456789");
    let pair = common::init_real_pair(
        "f03_s9",
        PairRelay::Mock(&relay),
        ("alice", a_route),
        ("bob", b_route),
    );
    let (alice, bob) = (&pair.a, &pair.b);
    let honest = alice.iso.root.join("s9-honest.bin");
    fs::write(&honest, b"f03 s9 honest message").unwrap();
    alice.run_ok(&[
        "send",
        "--transport",
        "relay",
        "--relay",
        relay.base_url(),
        "--to",
        "bob",
        "--file",
        honest.to_str().unwrap(),
    ]);
    let core = sender_core(&alice.cfg, "bob");

    // The exact code each authenticated hostile frame meets on the receiver's real receive
    // path (Transaction::receive -> body_decode), with the receiver's durable state unchanged.
    std::env::set_var(profile::ACTIVE.location_env, &bob.cfg);
    qsc::vault::protection::unlock_guarded(TEST_MOCK_VAULT_PASSPHRASE).unwrap();
    let durable = tree(&bob.cfg);
    let modes = ["body_profile", "body_file_shape", "body_request"];
    let mut frames = Vec::new();
    let mut codes = Vec::new();
    for mode in modes {
        let raw = qsc::na0780_test_hostile_wire(&core, mode, 0).unwrap();
        codes.push((mode, qsc::na0780_test_receive_response("alice", &raw).err()));
        assert!(
            tree(&bob.cfg) == durable,
            "{mode}: the refusal changed durable state"
        );
        frames.push(raw);
    }
    assert_eq!(
        codes,
        vec![
            ("body_profile", Some("INTEGRATION_MAGIC")),
            ("body_file_shape", Some("INTEGRATION_FILE_GATED")),
            ("body_request", Some("INTEGRATION_FILE_GATED")),
        ],
        "exact authenticated receive codes (C01 O9: e1, e5)"
    );

    // DISPOSITION: the same frames through the real CLI receive, beside the honest message.
    // Each is an expected non-admission: skipped, never output, and the receive exits 0.
    for raw in frames {
        relay.enqueue_raw(b_route, raw);
    }
    let out_dir = bob.iso.root.join("s9-in");
    fs::create_dir_all(&out_dir).unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&out_dir, fs::Permissions::from_mode(0o700)).unwrap();
    }
    let received = bob.run(&[
        "receive",
        "--transport",
        "relay",
        "--relay",
        relay.base_url(),
        "--mailbox",
        b_route,
        "--from",
        "alice",
        "--max",
        "8",
        "--out",
        out_dir.to_str().unwrap(),
    ]);
    let text = format!(
        "{}{}",
        String::from_utf8_lossy(&received.stdout),
        String::from_utf8_lossy(&received.stderr)
    );
    assert!(
        received.status.success(),
        "the refused frames must be skipped, not abort the receive: {text}"
    );
    assert!(
        text.contains("event=recv_skip_summary count=3"),
        "all three refused frames skipped: {text}"
    );
    let delivered: Vec<Vec<u8>> = tree(&out_dir).into_values().collect();
    assert_eq!(
        delivered,
        vec![b"f03 s9 honest message".to_vec()],
        "only the honest message is output"
    );
}
