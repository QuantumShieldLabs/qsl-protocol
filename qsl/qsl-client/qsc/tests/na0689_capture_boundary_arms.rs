//! NA-0689 (D-1328 Ruling 11.3) — THE CAPTURE BOUNDARY, MEASURED END-TO-END.
//!
//! ⚠ WHAT THIS FILE IS FOR. The quarantine store must capture a judged-unrecoverable item and must
//! capture **nothing else**. Those are two different failures and only one of them is loud: a store
//! that captures too little loses the bytes this lane exists to keep, and a store that captures too
//! much silently becomes a copy of ordinary traffic. So every arm here is a PAIR — a zero and a
//! positive from the same setup, against the same relay.
//!
//! ⚠ A ZERO ALONE WOULD BE THE VACUOUS HALF. "No items were captured" is exactly what a run that
//! never reached the site also reports. The positive in each pair is what proves the instrument
//! could have counted.
//!
//! ⚠ THE RELAY MUST BE THE REAL ONE FOR D1. The D1 backstop is gated on `AckMode::Lease`, so it
//! needs real lease expiry and real redelivery; the `common` mock parses only `max=` and always
//! pops, and would make this arm vacuous. (For D2–D5 capture is ack-mode-independent — `capture_at`
//! runs unconditionally, upstream of the ack — but D1 is the one site where the mode is the gate.)
//!
//! ⚠ WHAT A GREEN HERE DOES NOT ASSERT: nothing about the `IgnoredWrongDevice` or `Err` captures at
//! D2/D3/D4. Those are HOSTILE-PEER witnesses that a stock `qsc` peer cannot produce over the wire,
//! and they are pinned exhaustively at the decision layer instead — see
//! `transport::confirm_capture_reason_tests` and D-1328 Ruling 11.2.

mod common;

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::{Mutex, MutexGuard, OnceLock};
use std::thread;
use std::time::Duration;

/// A 1-second server-side pull lease, so an unacked item becomes visible again quickly.
/// The same values NA-0644 uses to prove lease redelivery.
const TEST_PULL_LEASE_SECS: usize = 1;
const LEASE_EXPIRY_WAIT: Duration = Duration::from_millis(2500);

const ALICE_INBOX: &str = "na0689cb_alice_inbox_token_abcdefg";
const BOB_INBOX: &str = "na0689cb_bob_inbox_token_hijklmno";

fn guard() -> MutexGuard<'static, ()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
        .lock()
        .unwrap_or_else(|e| e.into_inner())
}

fn output_text(out: &std::process::Output) -> String {
    format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    )
}

fn ensure_dir_700(path: &Path) {
    fs::create_dir_all(path).expect("create dir");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut p = fs::metadata(path).expect("meta").permissions();
        p.set_mode(0o700);
        fs::set_permissions(path, p).expect("chmod");
    }
}

fn test_root(tag: &str) -> PathBuf {
    let root = common::unique_test_root(tag);
    ensure_dir_700(&root);
    root
}

fn qsc(cfg: &Path) -> Command {
    // ⚠ `qsc_std_command()` ALREADY applies the mock-vault unlock args; re-adding them makes clap
    // reject `--unlock-passphrase-env` as repeated and fails setup before any measurement runs.
    let mut c = common::qsc_std_command();
    c.env("QSC_CONFIG_DIR", cfg)
        .env("QSC_QSP_SEED", "1")
        .env("QSC_ALLOW_SEED_FALLBACK", "1")
        .env("QSC_UNSAFE_TEST_SEED_FALLBACK", "1")
        .env("QSC_MARK_FORMAT", "plain");
    c
}

fn run_ok(cfg: &Path, args: &[&str]) -> String {
    let out = qsc(cfg).args(args).output().expect("run qsc");
    let text = output_text(&out);
    assert!(out.status.success(), "expected success: {args:?}\n{text}");
    text
}

fn run_fail(cfg: &Path, args: &[&str]) -> String {
    let out = qsc(cfg).args(args).output().expect("run qsc");
    let text = output_text(&out);
    assert!(
        !out.status.success(),
        "expected FAILURE but the command succeeded: {args:?}\n{text}"
    );
    text
}

/// A party: its own config dir, vault, identity and inbox token.
/// Adopted from `na0688_c4_collateral_arms.rs` rather than re-derived beside it.
fn party(root: &Path, name: &str, inbox: &str) -> PathBuf {
    let cfg = root.join(name);
    ensure_dir_700(&cfg);
    common::init_mock_vault(&cfg);
    run_ok(&cfg, &["identity", "rotate", "--confirm"]);
    run_ok(&cfg, &["relay", "inbox-set", "--token", inbox]);
    cfg
}

fn fingerprint(cfg: &Path) -> String {
    run_ok(cfg, &["identity", "show"])
        .lines()
        .find_map(|l| l.strip_prefix("identity_fp="))
        .expect("identity_fp")
        .trim()
        .to_string()
}

/// ⚠ Adding the contact is NOT enough to send: its device must also be TRUSTED. This is the
/// NA-0644 `setup_pair` sequence adopted wholesale — the arms measure capture behaviour, and any
/// bespoke setup here is a way to measure my own scaffolding by mistake.
fn add_contact(cfg: &Path, label: &str, fp: &str, route_token: &str) {
    run_ok(
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
        ],
    );
    let list = run_ok(cfg, &["contacts", "device", "list", "--label", label]);
    let device = list
        .lines()
        .find_map(|line| {
            line.split_whitespace()
                .find_map(|tok| tok.strip_prefix("device="))
        })
        .unwrap_or_else(|| panic!("missing device output: {list}"));
    run_ok(
        cfg,
        &[
            "contacts", "device", "trust", "--label", label, "--device", device, "--confirm",
        ],
    );
}

fn setup(root: &Path) -> (PathBuf, PathBuf) {
    let alice = party(root, "alice", ALICE_INBOX);
    let bob = party(root, "bob", BOB_INBOX);
    let alice_fp = fingerprint(&alice);
    let bob_fp = fingerprint(&bob);
    add_contact(&alice, "bob", &bob_fp, BOB_INBOX);
    add_contact(&bob, "bob", &alice_fp, ALICE_INBOX);
    (alice, bob)
}

fn send_message(alice: &Path, relay: &str, base: &Path, name: &str, bytes: &[u8]) {
    let msg = base.join(name);
    fs::write(&msg, bytes).expect("write msg");
    let text = run_ok(
        alice,
        &[
            "send",
            "--transport",
            "relay",
            "--relay",
            relay,
            "--to",
            "bob",
            "--file",
            msg.to_str().expect("msg path"),
        ],
    );
    assert!(text.contains("QSC_DELIVERY state=accepted_by_relay"), "{text}");
}

fn receive_args<'a>(relay: &'a str, out: &'a str) -> Vec<&'a str> {
    vec![
        "receive",
        "--transport",
        "relay",
        "--relay",
        relay,
        "--mailbox",
        BOB_INBOX,
        "--from",
        "bob",
        "--max",
        "8",
        "--out",
        out,
        "--ack-mode",
        "lease",
    ]
}

/// The quarantine store as a USER meets it — through the shipped verb, never by reaching into the
/// store's files. ⚠ Marker lines are prefixed (`QSC_MARK/1 event=…`), so the event is matched by
/// CONTAINMENT and the count read as a token: a `strip_prefix("event=…")` here silently matches
/// nothing, and inside an `unwrap_or(0)` that would report "clean" for every possible tree.
fn quarantine_entries(cfg: &Path) -> (usize, String) {
    let out = qsc(cfg).args(["quarantine", "list"]).output().expect("run");
    let text = output_text(&out);
    let n = text
        .lines()
        .find(|l| l.contains("event=quarantine_list"))
        .and_then(|line| {
            line.split_whitespace()
                .find_map(|tok| tok.strip_prefix("count="))
        })
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or_else(|| panic!("no quarantine_list count in output:\n{text}"));
    (n, text)
}

// ---------------------------------------------------------------------------
// D1 — the NA-0644 backstop: the known destruction site, and the one this lane was named for.
//
// PAIRED IN ONE TEST ON PURPOSE. The zero and the positive share a relay, a pair of identities and
// a session, so the only thing that differs between them is whether the item was judged
// unrecoverable. Two separate tests would let a setup difference masquerade as the result.
// ---------------------------------------------------------------------------
#[test]
fn a_clean_receive_captures_nothing_and_the_replay_backstop_captures_exactly_one() {
    let _g = guard();
    let relay =
        common::start_qsl_server_with_store(2 * 1024 * 1024, 512, None, TEST_PULL_LEASE_SECS);
    let base = relay.base_url().to_string();
    let root = test_root("na0689_capture_boundary_d1");
    let (alice, bob) = setup(&root);

    // ---- ZERO: an ordinary message, received cleanly. ----
    let out1 = root.join("out1");
    ensure_dir_700(&out1);
    send_message(&alice, &base, &root, "clean.txt", b"na0689 an ordinary message");
    let clean = run_ok(
        &bob,
        &receive_args(&base, out1.to_str().expect("out1")),
    );
    assert!(
        out1.join("recv_1.bin").is_file(),
        "the ordinary message must have been written: {clean}"
    );
    let (after_clean, listing_clean) = quarantine_entries(&bob);
    assert_eq!(
        after_clean, 0,
        "A SUCCESSFUL RECEIVE MUST CAPTURE NOTHING. A store that captures processed traffic is a \
         copy of the inbox, not a quarantine.\nreceive:\n{clean}\nlisting:\n{listing_clean}"
    );

    // ---- POSITIVE: the commit-before-write seam, which is what MAKES an item unrecoverable. ----
    //
    // The ratchet consumes the message key durably BEFORE the payload is written, so a failure in
    // that gap leaves an envelope nobody can ever decrypt again — no matter how often the relay
    // redelivers it. This is NA-0644's scenario (f), adopted wholesale: occupy the `write_atomic`
    // rename target with a DIRECTORY so the commit succeeds and the write then fails.
    //
    // ⚠ A FRESH OUT DIR, so the rename target is `recv_1.bin` again rather than a number this test
    // would have to predict from the previous phase.
    let out2 = root.join("out2");
    ensure_dir_700(&out2);
    send_message(&alice, &base, &root, "doomed.txt", b"na0689 the doomed message");
    fs::create_dir(out2.join("recv_1.bin")).expect("occupy rename target");
    let failed = run_fail(&bob, &receive_args(&base, out2.to_str().expect("out2")));
    assert!(
        failed.contains("recv_write_failed"),
        "the write must fail INSIDE the gap, or the item is not unrecoverable and this arm \
         measures nothing: {failed}"
    );
    fs::remove_dir(out2.join("recv_1.bin")).expect("clear rename target");

    // The item was never acked, so the lease expires and the relay redelivers an envelope whose
    // key is already consumed — the backstop's exact precondition.
    thread::sleep(LEASE_EXPIRY_WAIT);
    let redelivered = run_ok(&bob, &receive_args(&base, out2.to_str().expect("out2")));
    assert!(
        redelivered.contains("event=ack_replay_unrecoverable"),
        "the redelivery must reach the D1 backstop: {redelivered}"
    );

    let (after_seam, listing_seam) = quarantine_entries(&bob);
    assert_eq!(
        after_seam, 1,
        "THE UNRECOVERABLE ITEM MUST BE KEPT. Before this lane it was acked away with a log marker \
         as the only witness — that destruction is the defect NA-0689 exists to remove.\n\
         redelivery:\n{redelivered}\nlisting:\n{listing_seam}"
    );

    // ⚠ BOTH DISCRIMINATORS, because neither implies the other (Rulings 2 and 7): `subclass` says
    // WHY the item was kept, `content` says WHAT the bytes are. At D1 the wire envelope is all
    // there is — the key was consumed in an earlier run, so the ciphertext is permanently
    // undecryptable and is kept for correlation, never for recovery.
    assert!(
        listing_seam.contains("subclass=unrecoverable"),
        "D1's capture must be witnessed as unrecoverable: {listing_seam}"
    );
    assert!(
        listing_seam.contains("content=wire_envelope"),
        "D1 stores the WIRE ENVELOPE, not an inner payload: {listing_seam}"
    );

    // ⚠ NO CONTENT IS PRINTED AND NONE IS PRINTABLE — the summary type carries no accessor for the
    // stored bytes at all. Asserted here because a listing is exactly where a redaction slip would
    // surface first.
    assert!(
        !listing_seam.contains("na0689 the doomed message"),
        "the listing must never print captured content: {listing_seam}"
    );
    assert!(
        listing_seam.contains("content_readable=false"),
        "the limitation must be stated unconditionally, never inferable only from silence: \
         {listing_seam}"
    );

    // ---- The drop verb: a stored item must ALWAYS be deletable. ----
    let entry_id = listing_seam
        .lines()
        .find(|l| l.contains("event=quarantine_item"))
        .and_then(|l| {
            l.split_whitespace()
                .find_map(|tok| tok.strip_prefix("id="))
        })
        .unwrap_or_else(|| panic!("no quarantine_item id: {listing_seam}"))
        .to_string();
    run_ok(&bob, &["quarantine", "drop", "--id", &entry_id]);
    let (after_drop, listing_drop) = quarantine_entries(&bob);
    assert_eq!(
        after_drop, 0,
        "`quarantine drop` must remove the named item — otherwise this lane traded 'destroyed \
         without consent' for 'kept without consent'.\nlisting:\n{listing_drop}"
    );
}

// ---------------------------------------------------------------------------
// D4 and D5 — THE ZEROS, through the real delivered-receipt round-trip.
//
// One flow exercises both sites' success paths: the ack Alice receives back is classified
// `DeliveredAck` -- a class this build UNDERSTANDS -- so it passes D5's forward-compat branch
// without capture and lands on D4's apply, which confirms. Neither may capture anything.
//
// ⚠ THE MOCK RELAY IS CORRECT HERE, AND THE GROUND IS MEASURED RATHER THAN CONVENIENT: capture at
// D2-D5 is ack-mode-independent, because `quarantine::capture_at` runs UNCONDITIONALLY and
// UPSTREAM of `record_seen_and_queue_ack` (which is itself a no-op under legacy). Only D1 is gated
// on `AckMode::Lease`, and D1 has its own real-relay arm above.
// ---------------------------------------------------------------------------
#[test]
fn a_delivered_receipt_that_applies_captures_nothing_at_d4_or_d5() {
    let _g = guard();
    let relay = common::start_inbox_server(1024 * 1024, 64);
    let base = relay.base_url().to_string();
    let root = test_root("na0689_capture_boundary_d4d5");
    let (alice, bob) = setup(&root);

    // Alice sends and REQUESTS a delivered receipt. Without the explicit request no ack is ever
    // produced, and the arm would measure an empty mailbox instead of an applied confirm.
    let msg = root.join("receipted.txt");
    fs::write(&msg, b"na0689 a message that asks for a receipt").expect("write msg");
    let sent = run_ok(
        &alice,
        &[
            "send",
            "--transport",
            "relay",
            "--relay",
            &base,
            "--to",
            "bob",
            "--file",
            msg.to_str().expect("msg path"),
            "--receipt",
            "delivered",
        ],
    );
    assert!(
        sent.contains("event=receipt_request kind=delivered"),
        "the receipt must actually be requested, or no ack is ever sent: {sent}"
    );

    // Bob receives it and, per the NA-0688 C3 default, sends the delivered ack back.
    let bob_out = root.join("bob_out");
    ensure_dir_700(&bob_out);
    let bob_recv = run_ok(
        &bob,
        &[
            "receive",
            "--transport",
            "relay",
            "--relay",
            &base,
            "--mailbox",
            BOB_INBOX,
            "--from",
            "bob",
            "--max",
            "8",
            "--out",
            bob_out.to_str().expect("out"),
        ],
    );
    assert!(
        bob_recv.contains("event=receipt_send kind=delivered"),
        "the recipient must actually emit the ack: {bob_recv}"
    );
    let (bob_captured, bob_listing) = quarantine_entries(&bob);
    assert_eq!(
        bob_captured, 0,
        "receiving an ordinary message must capture nothing: {bob_listing}"
    );

    // Alice collects the ack. It classifies as DeliveredAck -- known to this build -- so D5 does
    // not capture it, and D4's apply confirms, so D4 does not either.
    let alice_out = root.join("alice_out");
    ensure_dir_700(&alice_out);
    let alice_recv = run_ok(
        &alice,
        &[
            "receive",
            "--transport",
            "relay",
            "--relay",
            &base,
            "--mailbox",
            ALICE_INBOX,
            "--from",
            "bob",
            "--max",
            "8",
            "--out",
            alice_out.to_str().expect("out"),
        ],
    );
    assert!(
        alice_recv.contains("event=delivered_to_peer"),
        "the ack must APPLY -- this marker is emitted only inside D4's Confirmed arm, so without \
         it the zero below would be measuring an ack that never arrived: {alice_recv}"
    );

    let (alice_captured, alice_listing) = quarantine_entries(&alice);
    assert_eq!(
        alice_captured, 0,
        "AN APPLIED CONFIRM MUST CAPTURE NOTHING. D4 shares its ack with a success arm, so a \
         blanket capture here would store every successfully applied confirm and turn the store \
         into a copy of ordinary traffic.\nreceive:\n{alice_recv}\nlisting:\n{alice_listing}"
    );

    // ⚠ The zero above is only evidence because the SAME instrument counted 1 at D1 in this file's
    // other arm, against the same verb and the same parser. A count that can only ever read zero
    // would satisfy this assertion on every possible tree.
    assert!(
        alice_listing.contains("event=quarantine_list count=0"),
        "the listing must positively report an empty store rather than printing nothing: \
         {alice_listing}"
    );
}

/// D3 — the FILE-confirm site's success arm.
///
/// Alice sends a file requesting a delivered receipt; Bob receives it and emits the file confirm;
/// Alice collects that confirm and it APPLIES. `file_confirm_recv` is emitted only inside D3's
/// `Confirmed` arm, so it is what proves the zero below is measuring an applied confirm rather
/// than an empty mailbox.
#[test]
fn a_file_confirm_that_applies_captures_nothing_at_d3() {
    let _g = guard();
    let relay = common::start_inbox_server(1024 * 1024, 256);
    let base = relay.base_url().to_string();
    let root = test_root("na0689_capture_boundary_d3");
    let (alice, bob) = setup(&root);

    let payload = root.join("file.bin");
    fs::write(&payload, vec![0x51; 12_288]).expect("write payload");
    run_ok(
        &alice,
        &[
            "file",
            "send",
            "--transport",
            "relay",
            "--relay",
            &base,
            "--to",
            "bob",
            "--path",
            payload.to_str().expect("path"),
            "--chunk-size",
            "4096",
            "--receipt",
            "delivered",
        ],
    );

    let bob_out = root.join("bob_out");
    ensure_dir_700(&bob_out);
    let bob_text = run_ok(
        &bob,
        &[
            "receive",
            "--transport",
            "relay",
            "--relay",
            &base,
            "--mailbox",
            BOB_INBOX,
            "--from",
            "bob",
            "--max",
            "64",
            "--out",
            bob_out.to_str().expect("out"),
            "--emit-receipts",
            "delivered",
        ],
    );
    assert!(
        bob_text.contains("event=file_confirm_send"),
        "the recipient must emit the file confirm: {bob_text}"
    );
    let (bob_n, bob_listing) = quarantine_entries(&bob);
    assert_eq!(bob_n, 0, "receiving a file must capture nothing: {bob_listing}");

    let alice_out = root.join("alice_out");
    ensure_dir_700(&alice_out);
    let alice_text = run_ok(
        &alice,
        &[
            "receive",
            "--transport",
            "relay",
            "--relay",
            &base,
            "--mailbox",
            ALICE_INBOX,
            "--from",
            "bob",
            "--max",
            "64",
            "--out",
            alice_out.to_str().expect("out"),
        ],
    );
    assert!(
        alice_text.contains("event=file_confirm_recv"),
        "the confirm must APPLY, or the zero below measures an ack that never arrived: {alice_text}"
    );
    let (alice_n, alice_listing) = quarantine_entries(&alice);
    assert_eq!(
        alice_n, 0,
        "AN APPLIED FILE CONFIRM MUST CAPTURE NOTHING -- D3 shares its ack with this success arm.\n\
         receive:\n{alice_text}\nlisting:\n{alice_listing}"
    );
}

/// D2 — the ATTACHMENT-confirm site's success arm. Identical in shape to D3 but routed through a
/// real attachment service, which is what makes it the attachment path rather than the inline one.
#[test]
fn an_attachment_confirm_that_applies_captures_nothing_at_d2() {
    let _g = guard();
    let relay = common::start_inbox_server(2 * 1024 * 1024, 512);
    let service = common::start_attachment_server(100 * 1024 * 1024);
    let base = relay.base_url().to_string();
    let svc = service.base_url().to_string();
    let root = test_root("na0689_capture_boundary_d2");
    let (alice, bob) = setup(&root);

    let payload = root.join("stream.bin");
    // ⚠ SIZE IS WHAT SELECTS THE PATH, not the --attachment-service flag alone. Below the
    // streaming threshold the send takes the inline file_chunk route, which post-W0 receive
    // rejects outright -- so a small payload makes this arm fail in SETUP rather than measure D2.
    // 6 MiB + 321 is the reference size `attachments_contract_na0217h` uses.
    fs::write(&payload, vec![0x62; 6 * 1024 * 1024 + 321]).expect("write payload");
    run_ok(
        &alice,
        &[
            "file",
            "send",
            "--transport",
            "relay",
            "--relay",
            &base,
            "--attachment-service",
            &svc,
            "--to",
            "bob",
            "--path",
            payload.to_str().expect("path"),
            // ⚠ NO --chunk-size HERE, and that is the difference between this arm and D3's: a
            // chunk size forces the legacy inline file_chunk path, which post-W0 receive REJECTS
            // outright (legacy_receive_retired_post_w0) -- so the attachment confirm would never
            // be produced and this arm would fail in setup rather than measure D2.
            "--receipt",
            "delivered",
        ],
    );

    let bob_out = root.join("bob_out");
    ensure_dir_700(&bob_out);
    let bob_text = run_ok(
        &bob,
        &[
            "receive",
            "--transport",
            "relay",
            "--relay",
            &base,
            "--attachment-service",
            &svc,
            "--mailbox",
            BOB_INBOX,
            "--from",
            "bob",
            "--max",
            "64",
            "--out",
            bob_out.to_str().expect("out"),
            "--emit-receipts",
            "delivered",
        ],
    );
    assert!(
        bob_text.contains("attachment_confirm_send"),
        "the recipient must emit the attachment confirm: {bob_text}"
    );

    let alice_out = root.join("alice_out");
    ensure_dir_700(&alice_out);
    let alice_text = run_ok(
        &alice,
        &[
            "receive",
            "--transport",
            "relay",
            "--relay",
            &base,
            "--attachment-service",
            &svc,
            "--mailbox",
            ALICE_INBOX,
            "--from",
            "bob",
            "--max",
            "64",
            "--out",
            alice_out.to_str().expect("out"),
        ],
    );
    assert!(
        alice_text.contains("event=attachment_confirm_recv"),
        "the confirm must APPLY, or the zero below measures nothing: {alice_text}"
    );
    let (alice_n, alice_listing) = quarantine_entries(&alice);
    assert_eq!(
        alice_n, 0,
        "AN APPLIED ATTACHMENT CONFIRM MUST CAPTURE NOTHING -- D2 shares its ack with this success \
         arm.\nreceive:\n{alice_text}\nlisting:\n{alice_listing}"
    );
}
