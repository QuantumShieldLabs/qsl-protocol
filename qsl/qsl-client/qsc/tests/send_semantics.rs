// NA-0785 PLAN F03 / S6a -- FF1 (seeded-session contamination), moved onto an HONEST fixture.
//
// Every test here used to run over a SEEDED contact (`QSC_QSP_SEED` + the seed fallback, a
// fabricated `fp-test` pin) that the integration head refuses at setup
// (`directional_profile_required`). Each now runs over a REAL pair: `common::init_real_pair`
// over the real in-process leasing qsl-server (the DEFAULT relay for any property about
// delivery, loss or redelivery) -- two successor vaults of `profile::ACTIVE`, pinned
// identities, trusted devices and a real handshake. No seeded session, no seed fallback, no
// fabricated key. The peer is the handshaken contact `bob`.
//
// Where the head no longer emits a legacy marker, the assertion is REPLACED by the marker the
// head does emit for the same property, and the old -> new mapping is named at the assertion
// (lane map MAP_S6a.tsv). A synthetic local run; it says nothing about production or the real
// relay deployment.

mod common;

use common::{PairRelay, QslRelayTestServer, RealPair, VaultFixture};
use predicates::prelude::*;
use std::fs;
use std::path::{Path, PathBuf};

/// A route that nothing listens on: the send must fail at the network.
const DEAD_RELAY: &str = "http://127.0.0.1:9";

fn private_dir(path: &Path) -> PathBuf {
    fs::create_dir_all(path).unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(path, fs::Permissions::from_mode(0o700)).unwrap();
    }
    path.to_path_buf()
}

fn combined_output(output: &std::process::Output) -> String {
    let mut combined = String::from_utf8_lossy(&output.stdout).to_string();
    combined.push_str(&String::from_utf8_lossy(&output.stderr));
    combined
}

/// A real pair over the leasing relay. The server is returned so it outlives the pair.
fn real_pair(tag: &str) -> (QslRelayTestServer, RealPair) {
    let server = common::start_qsl_server(1024 * 1024, 32, None);
    let pair = common::init_real_pair(
        tag,
        PairRelay::Leasing(&server),
        ("alice", "f03_s6a_ss_route_alice_0123456789"),
        ("bob", "f03_s6a_ss_route_bob_0123456789ab"),
    );
    (server, pair)
}

fn payload(v: &VaultFixture, name: &str, body: &[u8]) -> PathBuf {
    let dir = private_dir(&v.iso.root.join("payloads"));
    let path = dir.join(name);
    fs::write(&path, body).expect("write payload");
    path
}

fn send(v: &VaultFixture, relay: &str, to: &str, file: &Path) -> std::process::Output {
    v.command()
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
        ])
        .output()
        .expect("run send")
}

/// Every `recv_*.bin` the receiver has projected into `out`, by content.
fn received(out: &Path) -> Vec<Vec<u8>> {
    let mut all: Vec<Vec<u8>> = fs::read_dir(out)
        .unwrap()
        .map(|e| e.unwrap())
        .filter(|e| {
            let name = e.file_name().to_string_lossy().into_owned();
            name.starts_with("recv_") && name.ends_with(".bin")
        })
        .map(|e| fs::read(e.path()).unwrap())
        .collect();
    all.sort();
    all
}

fn receive(v: &VaultFixture, relay: &str, mailbox: &str, from: &str, out: &Path) -> String {
    let result = v
        .command()
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
            "8",
            "--out",
            out.to_str().unwrap(),
        ])
        .output()
        .expect("run receive");
    let text = combined_output(&result);
    assert!(result.status.success(), "receive failed: {text}");
    text
}

/// Relay-independent refusal (no relay is contacted), run over the real pair all the same.
#[test]
fn send_refuses_without_transport() {
    let (_server, pair) = real_pair("f03_s6a_send_no_transport");
    let alice = &pair.a;
    let file = payload(alice, "msg.bin", b"hello");

    let mut cmd = assert_cmd::Command::from_std(alice.command());
    cmd.env("QSC_MARK_FORMAT", "plain").args([
        "send",
        "--to",
        "bob",
        "--file",
        file.to_str().unwrap(),
    ]);
    cmd.assert().failure().stdout(predicate::eq(
        "QSC_MARK/1 event=error code=send_transport_required\n",
    ));
}

/// INSTRUMENTS.md (a) FF1: QUEUED -> relay accepted exactly once -> the peer decrypts the exact
/// bytes exactly once, over the real pair on the leasing relay.
#[test]
fn send_happy_path_local_relay() {
    let (server, pair) = real_pair("f03_s6a_send_happy");
    let (alice, bob) = (&pair.a, &pair.b);
    let relay_addr = server.base_url().to_string();
    let file = payload(alice, "msg.bin", b"hello");

    let output = send(alice, &relay_addr, "bob", &file);
    if !output.status.success() {
        panic!("send failed: {}", combined_output(&output));
    }
    let combined = combined_output(&output);
    // REPLACED (the head emits no `send_prepare`): the message is durably QUEUED before any push.
    assert!(
        combined.contains("event=msgqueue_enqueued state=QUEUED"),
        "send_prepare -> the message must be committed to the durable queue first: {combined}"
    );
    // REPLACED (the head emits no `send_attempt ok=true` on success): the push was delivered and
    // the relay accepted it -- exactly once.
    assert!(
        combined.contains("event=relay_event action=deliver"),
        "send_attempt ok=true -> the push must be delivered: {combined}"
    );
    assert_eq!(
        combined
            .matches("QSC_DELIVERY state=accepted_by_relay")
            .count(),
        1,
        "the relay must accept the message exactly once: {combined}"
    );
    // REPLACED (the head emits no `send_commit`): the committed transition CREATED -> SENT.
    assert!(
        combined.contains("event=message_state_transition from=CREATED to=SENT"),
        "send_commit -> the message must commit to SENT: {combined}"
    );

    // FF1's property end to end: the PEER decrypts the exact bytes, exactly once.
    let out = private_dir(&bob.iso.root.join("recv_out"));
    let first = receive(bob, &relay_addr, &pair.b_route, "alice", &out);
    assert_eq!(
        received(&out),
        vec![b"hello".to_vec()],
        "the peer must decrypt the exact bytes (no established session on the peer is the FF1 \
         red reason): {first}"
    );
    let again = receive(bob, &relay_addr, &pair.b_route, "alice", &out);
    assert_eq!(
        received(&out),
        vec![b"hello".to_vec()],
        "and exactly once -- a second pull delivers nothing new: {again}"
    );
}

#[test]
fn send_failure_no_commit() {
    let (_server, pair) = real_pair("f03_s6a_send_fail");
    let alice = &pair.a;
    let file = payload(alice, "msg.bin", b"hello");

    let output = send(alice, DEAD_RELAY, "bob", &file);

    assert!(!output.status.success(), "send should fail");
    let combined = combined_output(&output);
    assert!(combined.contains("event=relay_event action=push_fail"));
    assert!(combined.contains("event=send_attempt ok=false"));
    assert!(!combined.contains("event=send_commit"));
    // The head's commit witness, beside the legacy one (which the head no longer emits anywhere,
    // so on its own it could no longer fail): a failed push never commits to SENT.
    assert!(
        !combined.contains("to=SENT"),
        "a failed push must not commit the message to SENT: {combined}"
    );
}

#[test]
fn a_second_message_while_one_is_stuck_is_queued_not_dropped() {
    // ⚠ NA-0682 (D617 census C4) — THIS TEST REPLACES `outbox_recovery_via_send_abort`,
    // WHICH ASSERTED THE DEFECT AS CORRECT BEHAVIOUR.
    //
    // What the old test pinned: with a message stuck in the single global in-flight slot,
    // a second `qsc send` REPLAYED the first and asserted `!contains("event=qsp_pack")` --
    // i.e. it asserted that the caller's new message was NEVER EVEN PACKED. It was silently
    // dropped, and the only sanctioned recovery (`send abort`) DESTROYED the stuck one.
    // That is a silent loss, and the test made it look intentional.
    //
    // What this test pins instead (F2 + §2b/§2c, operator-ruled): both messages are durably
    // QUEUED before anything is packed or pushed, so neither can be lost; and recovery means
    // DRAIN, not destroy. The old behaviour is now impossible: there is no path that pushes
    // a message the store has not already committed.
    let (server, pair) = real_pair("f03_s6a_second_msg");
    let (alice, bob) = (&pair.a, &pair.b);

    let first = payload(alice, "first.bin", b"first");
    let second = payload(alice, "second.bin", b"second-must-survive");

    // Both sends fail at the network (dead port) -- and both must be queued, not lost.
    for f in [&first, &second] {
        let out = send(alice, DEAD_RELAY, "bob", f);
        // Honest reporting: SAFE is not SENT, so this still exits non-zero.
        assert!(
            !out.status.success(),
            "a queued-not-sent message must not report success"
        );
    }

    // ⚠ THE POINT: TWO records on disk. Under the old behaviour the second was never packed
    // and never stored -- this count would have been 1.
    assert_eq!(
        common::queued_record_count(&alice.cfg),
        2,
        "the second message was dropped -- the C4 silent loss has returned"
    );

    // Recovery is DRAIN, not destroy: bring the relay up and both go out.
    let out = alice
        .command()
        .env("QSC_MARK_FORMAT", "plain")
        .args(["outbox", "retry", "--relay", server.base_url()])
        .output()
        .expect("outbox retry");
    assert!(out.status.success(), "{}", combined_output(&out));
    let text = combined_output(&out);
    assert!(
        text.contains("event=outbox_drain") && text.contains("sent=2"),
        "both queued messages must drain: {text}"
    );
    // REPLACED (the leasing relay has no test-side drain): both messages reached the relay, and
    // the peer decrypts both, exact bytes.
    let bob_out = private_dir(&bob.iso.root.join("recv_out"));
    let recv = receive(bob, server.base_url(), &pair.b_route, "alice", &bob_out);
    assert_eq!(
        received(&bob_out),
        vec![b"first".to_vec(), b"second-must-survive".to_vec()],
        "both messages must reach the relay: {recv}"
    );
}

#[test]
fn send_outputs_have_no_secrets() {
    let (_server, pair) = real_pair("f03_s6a_send_no_secrets");
    let alice = &pair.a;
    let file = payload(alice, "msg.bin", b"hello");

    let output = send(alice, DEAD_RELAY, "bob", &file);

    let combined = combined_output(&output);
    for needle in [
        "TOKEN",
        "SECRET",
        "KEY",
        "PASS",
        "PRIVATE",
        "BEARER",
        "CREDENTIAL",
    ] {
        assert!(
            !combined.contains(needle),
            "unexpected secret token in output"
        );
    }
}
