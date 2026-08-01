// NA-0688 / D622 C3 (R1b, operator ruling on STOP #016 option (a)) — THE SENDER HALF.
//
// ⚠ WHY THIS FILE EXISTS. C3's first form flipped the sender half by giving
// `RelayMessageSender::new` a new default, and MEASUREMENT showed that value never reached the
// wire: `qsc send` builds its sender with `.with_meta(…, receipt)` and `with_meta` assigns the
// caller's choice UNCONDITIONALLY, so an absent `--receipt` overwrote the new default. Meanwhile
// `qsc outbox retry` and `qsc outbox discard`, which do NOT call `with_meta`, DID inherit it —
// so the SAME queued row could go out with or without a receipt request depending on which
// command drained it.
//
// The defect was invisible to every existing test, because the two halves of the feature were
// each pinned in isolation: a unit test asserted the constructor's field, and integration tests
// asserted the recipient's behaviour, and nothing asserted that the value on the CONSTRUCTOR
// survives to the WIRE. That is the gap these guards close.
//
// WHAT IS PINNED (operator ruling, verbatim in substance):
//   1. An absent `--receipt` means THE POLICY DEFAULT, so a default send actually requests a
//      receipt on the wire — the sender half is only ON if that is true.
//   2. A default `qsc send` and a `qsc outbox retry` of a default-queued row produce the SAME
//      receipt behaviour on the wire. One rule, all construction sites.
//   3. An EXPLICIT `--receipt off` still means off, verbatim, end to end.
//
// ⚠ THE OBSERVABLE IS THE PEER'S ACK, NOT THE SENDER'S OWN MARKER, and that is deliberate.
// A sender-side marker would only prove what the sender believes. Bob acks if and only if the
// message he received carried a `msg_id` in its data control envelope — i.e. if and only if the
// sender actually requested a receipt ON THE WIRE. Bob's receive uses no flags at all, so it is
// also an end-to-end check that the recipient half honours by default.

mod common;

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const ROUTE_TOKEN_BOB: &str = "route_token_bob_na0688_c3_sender";
/// A port nothing listens on: a send aimed here QUEUES the row and fails to push, which is the
/// only way to reach `outbox retry` with a row that a default `qsc send` created.
const DEAD_RELAY: &str = "http://127.0.0.1:9";

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
    let mut cmd = common::qsc_std_command();
    cmd.env("QSC_CONFIG_DIR", cfg)
        .env("QSC_MARK_FORMAT", "plain")
        .env("QSC_QSP_SEED", "1")
        .env("QSC_ALLOW_SEED_FALLBACK", "1")
        .env("QSC_UNSAFE_TEST_SEED_FALLBACK", "1");
    cmd
}

fn contacts_route_set(cfg: &Path, label: &str, token: &str) {
    let out = qsc_base(cfg)
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

fn relay_inbox_set(cfg: &Path, token: &str) {
    let out = qsc_base(cfg)
        .args(["relay", "inbox-set", "--token", token])
        .output()
        .expect("relay inbox set");
    assert!(out.status.success(), "{}", output_text(&out));
}

struct Pair {
    root: PathBuf,
    alice: PathBuf,
    bob: PathBuf,
    bob_out: PathBuf,
}

fn pair(tag: &str) -> Pair {
    let root = safe_test_root().join(format!("na0688_c3_{tag}_{}", std::process::id()));
    let alice = root.join("alice");
    let bob = root.join("bob");
    let bob_out = root.join("bob_out");
    for d in [&alice, &bob, &bob_out] {
        create_dir_700(d);
    }
    common::init_mock_vault(&alice);
    common::init_mock_vault(&bob);
    contacts_route_set(&alice, "bob", ROUTE_TOKEN_BOB);
    contacts_route_set(&bob, "bob", ROUTE_TOKEN_BOB);
    relay_inbox_set(&alice, ROUTE_TOKEN_BOB);
    relay_inbox_set(&bob, ROUTE_TOKEN_BOB);
    Pair {
        root,
        alice,
        bob,
        bob_out,
    }
}

/// Send from alice. `receipt` is the literal `--receipt` value, or `None` for no flag at all.
fn alice_send(p: &Pair, relay: &str, body: &[u8], tag: &str, receipt: Option<&str>) -> String {
    let f = p.root.join(format!("{tag}.bin"));
    fs::write(&f, body).unwrap();
    let mut args: Vec<String> = vec![
        "send".into(),
        "--transport".into(),
        "relay".into(),
        "--relay".into(),
        relay.into(),
        "--to".into(),
        "bob".into(),
        "--file".into(),
        f.to_str().unwrap().into(),
    ];
    if let Some(v) = receipt {
        args.push("--receipt".into());
        args.push(v.into());
    }
    let out = qsc_base(&p.alice).args(&args).output().expect("alice send");
    output_text(&out)
}

/// Bob receives with NO receipt flags — so this also proves the recipient half honours by default.
fn bob_receive(p: &Pair, relay: &str) -> String {
    let out = qsc_base(&p.bob)
        .args([
            "receive",
            "--transport",
            "relay",
            "--relay",
            relay,
            "--mailbox",
            ROUTE_TOKEN_BOB,
            "--from",
            "bob",
            "--max",
            "4",
            "--out",
            p.bob_out.to_str().unwrap(),
        ])
        .output()
        .expect("bob receive");
    output_text(&out)
}

/// GUARD 1 — a DEFAULT `qsc send` requests a receipt ON THE WIRE.
///
/// This is the assertion the first form of the flip would have failed. It does not look at any
/// sender-side field or marker; it asks whether the peer could act on what arrived.
#[test]
fn a_default_send_requests_a_receipt_on_the_wire() {
    let p = pair("default_send");
    let server = common::start_inbox_server(1024 * 1024, 64);

    let send = alice_send(&p, server.base_url(), b"c3-default-send", "m1", None);
    assert!(
        send.contains("QSC_DELIVERY state=accepted_by_relay"),
        "the fixture must actually deliver, or this guard is vacuous:\n{send}"
    );
    assert!(
        !send.contains("event=receipt_disabled"),
        "a default send must not report itself as receipt-disabled — that marker is keyed on \
         the RESOLVED request, so this firing means the policy default was not consulted:\n{send}"
    );

    let recv = bob_receive(&p, server.base_url());
    assert!(
        recv.contains("event=receipt_send"),
        "bob acks only if the message carried a msg_id, i.e. only if the sender actually \
         requested a receipt on the wire. No ack means the sender half is not really on:\n{recv}"
    );
}

/// GUARD 2 — ⚠ THE RULED PIN: a default send and an `outbox retry` of a default-queued row
/// produce the SAME receipt behaviour on the wire.
///
/// The two paths build their sender differently — `qsc send` goes through `with_meta`, `outbox
/// retry` does not. Both arms here start from the same default `qsc send`; only the drain differs.
///
/// ⚠ WHAT THIS GUARD ACTUALLY COVERS, stated precisely because measuring it corrected the claim
/// that motivated it. `msgqueue::attempt_one` packs a record **at most once in its life** and
/// replays the same bytes on every later attempt — a crypto-safety invariant, since re-packing
/// would burn a second message key — and `receipt_kind` is consumed at PACK time. So for an
/// already-packed row the retry's sender value is never read, and the two paths cannot diverge
/// no matter what they were constructed with. That is why this guard's red (below) fires on the
/// SEND path rather than the retry path.
///
/// It is still the pin the ruling asked for: it asserts end-to-end that a default-queued row
/// carries the same receipt semantics whether the send delivered it or a retry replayed it, and
/// it fails loudly if either changes. **What it does NOT reach is the narrow case of a record
/// whose FIRST PACK FAILED and is therefore still unpacked when the retry runs** — the only
/// situation in which the retry's own `receipt_kind` is consulted. That case is closed by
/// construction (all three sites resolve through one function) rather than by this test, and is
/// recorded in ENG-0096 rather than claimed as guarded.
#[test]
fn a_default_send_and_an_outbox_retry_agree_on_the_wire() {
    // ARM A — delivered by the send itself.
    let a = pair("agree_send");
    let server_a = common::start_inbox_server(1024 * 1024, 64);
    alice_send(&a, server_a.base_url(), b"c3-arm-a", "a1", None);
    let recv_a = bob_receive(&a, server_a.base_url());

    // ARM B — the SAME default send, but its push fails, so the row is drained later by
    // `outbox retry` against a live relay.
    let b = pair("agree_retry");
    let queued = alice_send(&b, DEAD_RELAY, b"c3-arm-b", "b1", None);
    assert!(
        !queued.contains("QSC_DELIVERY state=accepted_by_relay"),
        "arm B must NOT have been delivered by the send, or it is not testing the retry \
         path at all:\n{queued}"
    );
    let server_b = common::start_inbox_server(1024 * 1024, 64);
    let retry = qsc_base(&b.alice)
        .args(["outbox", "retry", "--relay", server_b.base_url()])
        .output()
        .expect("outbox retry");
    let retry_text = output_text(&retry);
    assert!(
        retry_text.contains("event=outbox_drain") && retry_text.contains("sent=1"),
        "the queued row must actually drain on retry, or arm B proves nothing:\n{retry_text}"
    );
    let recv_b = bob_receive(&b, server_b.base_url());

    // THE PIN: same behaviour, and it is the ON behaviour rather than both being silently off.
    let acked_a = recv_a.contains("event=receipt_send");
    let acked_b = recv_b.contains("event=receipt_send");
    assert_eq!(
        acked_a, acked_b,
        "a row queued by a DEFAULT send must go out with the same receipt semantics whether it \
         is drained by the send or by `outbox retry`. They disagree.\n\
         --- ARM A (send) ---\n{recv_a}\n--- ARM B (retry) ---\n{recv_b}"
    );
    assert!(
        acked_a,
        "both arms agreed, but agreed on NO receipt — the default is off somewhere and this \
         guard would pass vacuously:\n--- ARM A ---\n{recv_a}"
    );
}

/// GUARD 3 — an EXPLICIT `--receipt off` still means off, verbatim, end to end.
///
/// ⚠ This is what makes guards 1 and 2 non-vacuous: it proves the observable can come back
/// NEGATIVE in the same fixture, so "bob acked" is a measurement and not a constant. It is also
/// the capability the ruling required be preserved — before this commit, "no receipt for this
/// one message" was spelled by omitting the flag, and that spelling now means the opposite.
#[test]
fn an_explicit_receipt_off_still_means_off_end_to_end() {
    let p = pair("explicit_off");
    let server = common::start_inbox_server(1024 * 1024, 64);

    let send = alice_send(&p, server.base_url(), b"c3-explicit-off", "m1", Some("off"));
    assert!(
        send.contains("QSC_DELIVERY state=accepted_by_relay"),
        "the message must still be delivered — `off` suppresses the RECEIPT REQUEST, not the \
         send:\n{send}"
    );
    assert!(
        send.contains("event=receipt_disabled"),
        "an explicit `--receipt off` must report the request as disabled:\n{send}"
    );

    let recv = bob_receive(&p, server.base_url());
    assert!(
        !recv.contains("event=receipt_send"),
        "an explicit `off` must put the body on the wire RAW, with no msg_id for bob to ack. \
         An ack here means the explicit choice was overridden by the policy:\n{recv}"
    );
}
