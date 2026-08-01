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
