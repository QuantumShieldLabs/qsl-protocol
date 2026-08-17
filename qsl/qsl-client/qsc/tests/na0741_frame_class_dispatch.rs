//! NA-0741 (D-1376) LANE 1 — **RECEIVE-SIDE FRAME-CLASS DISPATCH**, the ENG-0142 repair.
//!
//! A frame belonging to another protocol — a handshake A2, an invite request, an invite reply —
//! can land in a user's ORDINARY inbox as a normal consequence of the invite flow. Before this
//! lane every one of them reached `qsp_unpack_for_peer`, failed envelope decode, and **aborted the
//! whole receive batch** (`transport/mod.rs:1249`), wedging the mailbox: NA-0740 measured exactly
//! that, in **both** directions of a completed invite.
//!
//! The repair classifies a frame from its LEADING BYTES **before** unpack and, under
//! `AckMode::Lease` only, skips the three KNOWN-FOREIGN classes — leaving them **leased, unacked
//! and undestroyed** so their rightful consumer collects them one lease period later.
//!
//! ## ⚠ WHAT THESE ARMS DELIBERATELY DO **NOT** ASSERT
//!
//! **Unknown-class frames still abort the batch, and T3 pins that as CURRENT behaviour rather than
//! as a defect.** That is the ruled trade (N-PRIME): an attacker chooses their own leading bytes,
//! so skipping Unknown buys no adversarial ground, while it would cost six committed assertions
//! over Unknown-class junk fixtures and the NA-0187 contact-request onboarding surface. An arm that
//! asserted "no frame ever aborts" would be asserting a DIFFERENT option than the one ruled.
//!
//! **Legacy is byte-unchanged in lane 1.** T4 drives both modes and pins the difference. ⚠ ENG-0149
//! is satisfied by STATING and DRIVING both modes — driving a mode is not changing it.
//!
//! ## ⚠ ENG-0149: THE RELAY MUST BE THE REAL ONE, AND THE MOCK IS REFUSED BY NAME
//!
//! Every arm drives the REAL in-process `qsl-server` through
//! `common::start_qsl_server_with_store`. `common::start_inbox_server` — the test-local mock —
//! **does not appear in this file**, mechanically asserted in the lane's evidence
//! (`git grep -c 'start_inbox_server'` over this file == 0). The mock always pops on pull and
//! cannot express a lease, so "the frame is still resident afterwards" would be VACUOUS against
//! it: the very property E3 exists to prove.

mod common;

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::{Mutex, MutexGuard, OnceLock};
use std::thread;
use std::time::Duration;

/// A 1-second server-side pull lease, so an unacked item becomes visible again quickly. The same
/// value `NA_0644_ack_client.rs`, `na0708_ack_flush.rs` and `na0689_p3_a2_stranding.rs` use.
const TEST_PULL_LEASE_SECS: usize = 1;
const LEASE_EXPIRY_WAIT: Duration = Duration::from_millis(2500);

/// ⚠⚠ **T7 USES A PRODUCTION-LENGTH LEASE, AND THE REASON IS MEASURED RATHER THAN STYLISTIC.**
///
/// The 1-second lease every other arm uses is there so an unacked frame becomes visible again
/// quickly. In T7 it actively destroys the property under test: with four skipped frames at the
/// head and a re-pull per round, **the lease expires BETWEEN ROUNDS** (a round carries the 250 ms
/// receipt batching window), the same four frames are redelivered at the head, `want` is refilled
/// by them every round, and the batch burns all `RECV_CONTROL_ROUNDS_MAX` rounds without ever
/// reaching the tail. Measured directly: **16 skips for 4 planted frames** (4 × 4 rounds),
/// `recv_skip_summary count=16`, `recv_none`, zero delivered.
///
/// ⚠ **THAT IS A REAL RESIDUAL, NOT ONLY A TEST ARTIFACT, AND IT IS RECORDED AS ONE:** whenever
/// the pull lease is shorter than a pull round, §5.1's round condition does not prevent the
/// silent under-delivery it was written to prevent — it converts it into a bounded re-skip spin.
/// The production relay runs `PULL_LEASE_SECS=60`, where a round cannot outlive the lease, so the
/// shipped configuration is unaffected. This arm pins the round condition, so it runs at the
/// production figure and the lease-expiry interaction is recorded rather than folded in here.
const PRODUCTION_PULL_LEASE_SECS: usize = 60;

fn guard() -> MutexGuard<'static, ()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
        .lock()
        .unwrap_or_else(|e| e.into_inner())
}

// ---------------------------------------------------------------------------
// Scaffolding — adopted wholesale from `na0708_ack_flush.rs` / `na0689_p3_a2_stranding.rs`
// rather than re-derived beside them, so a setup difference cannot masquerade as this
// lane's result.
// ---------------------------------------------------------------------------

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

fn run_any(cfg: &Path, args: &[&str]) -> (bool, String) {
    let out = qsc(cfg).args(args).output().expect("run qsc");
    (out.status.success(), output_text(&out))
}

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

/// ⚠ Adding the contact is NOT enough to send: its device must also be TRUSTED.
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
            "contacts",
            "device",
            "trust",
            "--label",
            label,
            "--device",
            device,
            "--confirm",
        ],
    );
}

/// Alice sends, Bob receives. Both sides label the peer `bob`, exactly as `na0708_ack_flush.rs`
/// does, so `--from bob` on Bob's side resolves to Alice's identity.
fn setup(root: &Path, alice_inbox: &str, bob_inbox: &str) -> (PathBuf, PathBuf) {
    let alice = party(root, "alice", alice_inbox);
    let bob = party(root, "bob", bob_inbox);
    let alice_fp = fingerprint(&alice);
    let bob_fp = fingerprint(&bob);
    add_contact(&alice, "bob", &bob_fp, bob_inbox);
    add_contact(&bob, "bob", &alice_fp, alice_inbox);
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
    assert!(
        text.contains("QSC_DELIVERY state=accepted_by_relay"),
        "{text}"
    );
}

/// ⚠ **THE MODE IS ALWAYS PASSED EXPLICITLY**, never inherited from the C4 default, because the
/// skip is gated on Lease and an arm that inherited its mode would stop measuring the moment a
/// default moved.
fn receive_args<'a>(
    relay: &'a str,
    mailbox: &'a str,
    from: &'a str,
    out: &'a str,
    max: &'a str,
    ack_mode: &'a str,
) -> Vec<&'a str> {
    vec![
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
        max,
        "--out",
        out,
        "--ack-mode",
        ack_mode,
    ]
}

fn recv_file_count(out: &Path) -> usize {
    fs::read_dir(out)
        .expect("read out dir")
        .filter_map(|e| e.ok())
        .filter(|e| {
            let name = e.file_name().to_string_lossy().to_string();
            name.starts_with("recv_") && name.ends_with(".bin")
        })
        .count()
}

/// ⚠ **LINE-SCOPED, ON PURPOSE.** A marker assertion written as two independent `contains` calls
/// over the whole capture is satisfied by two DIFFERENT lines — `event=qsp_unpack … ok=true` on one
/// and `ok=false` on another would pass a check meant to pin one line. Every marker assertion in
/// this file goes through here so the fields are read from the SAME emission.
fn has_marker_line(text: &str, event: &str, needles: &[&str]) -> bool {
    let ev = format!("event={event}");
    text.lines()
        .any(|l| l.contains(&ev) && needles.iter().all(|n| l.contains(n)))
}

/// Put an arbitrary frame into a mailbox through the relay's own public push route — the same
/// route the client uses, and the same route a peer's handshake or invite frame arrives by.
fn push_raw(base: &str, route_token: &str, bytes: &[u8]) {
    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .expect("push client");
    let resp = client
        .post(format!("{}/v1/push", base.trim_end_matches('/')))
        .header("X-QSL-Route-Token", route_token)
        .body(bytes.to_vec())
        .send()
        .expect("push frame");
    assert!(
        resp.status().is_success(),
        "the relay refused the frame, so the arm would measure nothing: {}",
        resp.status()
    );
}

/// Read a mailbox's CURRENTLY VISIBLE items straight off the wire.
///
/// ⚠ **`ack=lease` IS DELIBERATE AND IS THE NON-DESTRUCTIVE CHOICE.** A legacy pull DELETES what it
/// returns, so a residency probe written that way would consume the very evidence it exists to
/// report and could never be run twice. A lease pull re-leases and leaves the item in place.
fn raw_pull_lease(base: &str, route_token: &str, max: usize) -> Vec<Vec<u8>> {
    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .expect("pull client");
    let resp = client
        .get(format!(
            "{}/v1/pull?max={}&ack=lease",
            base.trim_end_matches('/'),
            max
        ))
        .header("X-QSL-Route-Token", route_token)
        .send()
        .expect("raw pull");
    if resp.status().as_u16() == 204 {
        return Vec::new();
    }
    assert!(
        resp.status().is_success(),
        "raw pull failed: {}",
        resp.status()
    );
    let body: serde_json::Value = resp.json().expect("pull json");
    body.get("items")
        .and_then(|i| i.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|it| it.get("data").and_then(|d| d.as_array()))
                .map(|d| {
                    d.iter()
                        .filter_map(|n| n.as_u64())
                        .map(|n| n as u8)
                        .collect::<Vec<u8>>()
                })
                .collect()
        })
        .unwrap_or_default()
}

// ---------------------------------------------------------------------------
// FIXTURES. ⚠ Every foreign frame is built from the ENCODER that writes the real thing, never
// from a transcribed byte pair — a hand-typed discriminator is a copy that can drift out of
// agreement with its producer without any gate noticing.
// ---------------------------------------------------------------------------

fn invite_resp_frame(route_token: &str) -> Vec<u8> {
    qsc::invite::encode_envelope_resp(route_token, b"na0741-b1-frame-bytes")
        .expect("encode invite reply envelope")
}

fn invite_init_frame(route_token: &str) -> Vec<u8> {
    qsc::invite::encode_envelope(&qsc::invite::HandshakeEnvelope {
        bundle: b"na0741-bundle-bytes".to_vec(),
        route_token: route_token.to_string(),
        a1: b"na0741-a1-frame-bytes".to_vec(),
    })
    .expect("encode invite request envelope")
}

/// A frame carrying the handshake magic. ⚠ The magic is spelled here rather than referenced
/// because `HS_MAGIC` is `pub(crate)` — deliberately NOT part of the public surface — so an
/// integration test cannot see it. The bytes are pinned against the classifier by T2's own
/// negative control: one flipped byte must change the outcome, which no transcription error
/// could survive.
const HS_MAGIC_WIRE: &[u8; 4] = b"QHSM";

fn handshake_frame() -> Vec<u8> {
    let mut v = HS_MAGIC_WIRE.to_vec();
    v.extend_from_slice(b"\x00\x01\x03na0741-handshake-body");
    v
}

// ===========================================================================
// T1 — invite-class frames at the head do not abort the batch.
// ===========================================================================

/// One arm: a KNOWN-FOREIGN frame at the head, a real message behind it. Shared by T1 (both
/// invite classes) and T2 (handshake), because the three classes must behave identically and a
/// per-class copy of this body would let them drift apart silently.
fn foreign_frame_arm(
    base: &str,
    root: &Path,
    tag: &str,
    alice_inbox: &str,
    bob_inbox: &str,
    foreign: &[u8],
    expect_class: &str,
) {
    let arm = root.join(tag);
    ensure_dir_700(&arm);
    let (alice, bob) = setup(&arm, alice_inbox, bob_inbox);

    // ORDER IS THE POINT: the foreign frame is at the HEAD, the real message behind it.
    push_raw(base, bob_inbox, foreign);
    let payload = format!("na0741 {tag} payload").into_bytes();
    send_message(&alice, base, &arm, "m1.bin", &payload);

    let out = arm.join("out");
    ensure_dir_700(&out);
    let (ok, text) = run_any(
        &bob,
        &receive_args(
            base,
            bob_inbox,
            "bob",
            out.to_str().expect("out"),
            "8",
            "lease",
        ),
    );

    assert!(
        ok,
        "THE WEDGE: a frame of class `{expect_class}` at the head aborted the whole batch. This \
         is ENG-0142 — the frame must be skipped before unpack, not decoded.\n{text}"
    );
    let class_needle = format!("class={expect_class}");
    assert!(
        has_marker_line(
            &text,
            "recv_frame_skipped",
            &[class_needle.as_str(), "disposition=left_leased"],
        ),
        "no `recv_frame_skipped class={expect_class} disposition=left_leased` marker:\n{text}"
    );
    assert!(
        has_marker_line(&text, "recv_item", &[]),
        "the real message behind the foreign frame was never delivered:\n{text}"
    );

    // The payload is byte-correct, not merely present.
    let delivered = fs::read(out.join("recv_1.bin")).expect("recv_1.bin");
    assert_eq!(
        delivered, payload,
        "the delivered payload does not match what was sent:\n{text}"
    );

    // ---- E3: LEASED, UNACKED, UNDESTROYED. ----
    // ⚠ The wait is mandatory: inside the lease a pulled-but-unacked item is held INVISIBLE, not
    // deleted, so probing early would measure VISIBILITY where the claim is SURVIVAL.
    thread::sleep(LEASE_EXPIRY_WAIT);
    let resident = raw_pull_lease(base, bob_inbox, 8);
    assert!(
        resident.iter().any(|f| f.as_slice() == foreign),
        "E3 FAILED: the skipped {expect_class} frame is GONE from the relay store. It must be left \
         leased and unacked so its rightful consumer collects it — this lane adds no new frame \
         consumption anywhere. {} item(s) resident.\n{text}",
        resident.len()
    );
}

#[test]
fn invite_class_frames_at_head_do_not_abort_the_batch() {
    let _g = guard();
    let relay =
        common::start_qsl_server_with_store(2 * 1024 * 1024, 512, None, TEST_PULL_LEASE_SECS);
    let base = relay.base_url().to_string();
    let root = test_root("na0741_t1");

    foreign_frame_arm(
        &base,
        &root,
        "resp",
        "na0741-t1-resp-alice-tok-aaaaaaaa",
        "na0741-t1-resp-bob-tok-bbbbbbbbbb",
        &invite_resp_frame("na0741-t1-third-party-route-tok-cc"),
        "invite_resp",
    );

    foreign_frame_arm(
        &base,
        &root,
        "init",
        "na0741-t1-init-alice-tok-dddddddd",
        "na0741-t1-init-bob-tok-eeeeeeeeee",
        &invite_init_frame("na0741-t1-third-party-route-tok-ff"),
        "invite_init",
    );

    // ---- NEGATIVE CONTROL: the same arrangement with NO foreign frame. ----
    // Without this the arms above cannot tell "the skip worked" from "the fixture never landed".
    let ctrl = root.join("control");
    ensure_dir_700(&ctrl);
    let (c_alice, c_bob) = setup(
        &ctrl,
        "na0741-t1-ctrl-alice-tok-gggggggg",
        "na0741-t1-ctrl-bob-tok-hhhhhhhhhh",
    );
    send_message(&c_alice, &base, &ctrl, "m1.bin", b"na0741 t1 control payload");
    let c_out = ctrl.join("out");
    ensure_dir_700(&c_out);
    let (c_ok, c_text) = run_any(
        &c_bob,
        &receive_args(
            &base,
            "na0741-t1-ctrl-bob-tok-hhhhhhhhhh",
            "bob",
            c_out.to_str().expect("out"),
            "8",
            "lease",
        ),
    );
    assert!(c_ok, "the control receive must succeed:\n{c_text}");
    assert_eq!(recv_file_count(&c_out), 1, "control delivery:\n{c_text}");
    assert!(
        !c_text.contains("recv_frame_skipped"),
        "CONTROL FAILED: a skip marker fired with no foreign frame planted, so the marker is not \
         evidence of a skip:\n{c_text}"
    );
}

// ===========================================================================
// T2 — a handshake-class frame at the head does not abort the batch.
// ⚠ This arm pins NA-0740's OTHER wedge victim: the INVITER's mailbox, where a bare A2 lands.
// ===========================================================================

#[test]
fn handshake_class_frame_at_head_does_not_abort_the_batch() {
    let _g = guard();
    let relay =
        common::start_qsl_server_with_store(2 * 1024 * 1024, 512, None, TEST_PULL_LEASE_SECS);
    let base = relay.base_url().to_string();
    let root = test_root("na0741_t2");

    foreign_frame_arm(
        &base,
        &root,
        "magic",
        "na0741-t2-alice-tok-aaaaaaaaaaaa",
        "na0741-t2-bob-tok-bbbbbbbbbbbbbb",
        &handshake_frame(),
        "handshake",
    );

    // ---- THE NEGATIVE CONTROL, AND IT IS THE STRONGEST IN THE PLAN. ----
    // ONE byte of the magic is flipped. The frame then matches NO discriminator, classifies
    // Unknown, is NOT skipped, reaches unpack, and aborts the batch EXACTLY as today. One byte
    // separates skip from abort, proven in both directions rather than asserted.
    let mut bent = handshake_frame();
    bent[3] ^= 0x01;
    assert_ne!(
        bent,
        handshake_frame(),
        "the altered fixture is identical to the original, so neither polarity below means \
         anything"
    );
    assert_eq!(
        bent[0..3],
        handshake_frame()[0..3],
        "the control must differ in exactly ONE byte of the magic, or it is testing something else"
    );

    let ctrl = root.join("bent");
    ensure_dir_700(&ctrl);
    const CTRL_BOB: &str = "na0741-t2-bent-bob-tok-cccccccccc";
    let (c_alice, c_bob) = setup(&ctrl, "na0741-t2-bent-alice-tok-dddddddd", CTRL_BOB);
    push_raw(&base, CTRL_BOB, &bent);
    send_message(&c_alice, &base, &ctrl, "m1.bin", b"na0741 t2 bent payload");

    let c_out = ctrl.join("out");
    ensure_dir_700(&c_out);
    let (c_ok, c_text) = run_any(
        &c_bob,
        &receive_args(
            &base,
            CTRL_BOB,
            "bob",
            c_out.to_str().expect("out"),
            "8",
            "lease",
        ),
    );
    assert!(
        !c_ok,
        "CONTROL FAILED: a one-byte-bent magic must classify Unknown and still abort the batch. \
         If this passes, the classifier is matching something broader than the magic:\n{c_text}"
    );
    assert!(
        !c_text.contains("recv_frame_skipped"),
        "CONTROL FAILED: the bent frame was SKIPPED. The classifier is over-matching — it must \
         compare all four magic bytes:\n{c_text}"
    );
}

// ===========================================================================
// T3 — Unknown-class junk still reaches unpack and still rejects.
//
// ⚠ **RED-FIRST-EXEMPT BY CONSTRUCTION, AND ITS OBLIGATION IS STRONGER.** This arm asserts
// UNCHANGED behaviour, so it must pass on the unrepaired tree AND after the repair. It is the arm
// that proves the classifier does not OVER-SKIP, and the one that keeps the suite's six committed
// Unknown-class assertions honest.
// ===========================================================================

#[test]
fn unknown_class_junk_still_reaches_unpack_and_still_rejects() {
    let _g = guard();
    let relay =
        common::start_qsl_server_with_store(2 * 1024 * 1024, 512, None, TEST_PULL_LEASE_SECS);
    let base = relay.base_url().to_string();
    let root = test_root("na0741_t3");
    const BOB: &str = "na0741-t3-bob-tok-bbbbbbbbbbbbbb";
    let (_alice, bob) = setup(&root, "na0741-t3-alice-tok-aaaaaaaaaaaa", BOB);

    // `FF FF …` matches NO discriminator: not the handshake magic, and `FF != 0x01`, so it is
    // none of the three envelope classes either.
    let junk = vec![0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF];
    push_raw(&base, BOB, &junk);

    let out = root.join("out");
    ensure_dir_700(&out);
    let (ok, text) = run_any(
        &bob,
        &receive_args(&base, BOB, "bob", out.to_str().expect("out"), "8", "lease"),
    );

    assert!(
        !ok,
        "OVER-SKIP: Unknown-class junk must STILL abort the batch. `Unknown` is deliberately not \
         known-foreign — skipping it would delete six committed assertions and the NA-0187 \
         onboarding surface:\n{text}"
    );
    assert!(
        has_marker_line(&text, "qsp_unpack", &["ok=false"]),
        "the junk frame must still REACH unpack and be rejected there:\n{text}"
    );
    assert!(
        !text.contains("recv_frame_skipped"),
        "OVER-SKIP: an Unknown-class frame emitted a skip marker. `class=unknown` and \
         `class=message` cannot appear on `recv_frame_skipped` under N-PRIME — those classes never \
         reach the skip arm:\n{text}"
    );
}

// ===========================================================================
// T4 — the MODE DISCRIMINATION test (E5). One arrangement, both modes.
// ⚠ This is the proof that the Legacy bound is REAL rather than asserted.
// ===========================================================================

#[test]
fn lease_skips_where_legacy_still_aborts() {
    let _g = guard();
    let relay =
        common::start_qsl_server_with_store(2 * 1024 * 1024, 512, None, TEST_PULL_LEASE_SECS);
    let base = relay.base_url().to_string();
    let root = test_root("na0741_t4");

    // ⚠ SEPARATE MAILBOXES PER LEG, AND THAT IS LOAD-BEARING: a legacy pull DELETES what it
    // returns, so a shared mailbox would let the legacy leg destroy the lease leg's fixture and
    // the second measurement would be of an empty inbox.
    let frame = invite_resp_frame("na0741-t4-third-party-route-tok-a");

    // ---- LEG 1: LEASE — skips, exits 0. ----
    let lease_dir = root.join("lease");
    ensure_dir_700(&lease_dir);
    const LEASE_BOB: &str = "na0741-t4-lease-bob-tok-aaaaaaaaa";
    let (l_alice, l_bob) = setup(&lease_dir, "na0741-t4-lease-alice-tok-bbbbbbb", LEASE_BOB);
    push_raw(&base, LEASE_BOB, &frame);
    send_message(&l_alice, &base, &lease_dir, "m1.bin", b"na0741 t4 lease payload");
    let l_out = lease_dir.join("out");
    ensure_dir_700(&l_out);
    let (l_ok, l_text) = run_any(
        &l_bob,
        &receive_args(
            &base,
            LEASE_BOB,
            "bob",
            l_out.to_str().expect("out"),
            "8",
            "lease",
        ),
    );
    assert!(l_ok, "under LEASE the foreign frame must be skipped:\n{l_text}");
    assert!(
        has_marker_line(
            &l_text,
            "recv_frame_skipped",
            &["class=invite_resp", "disposition=left_leased"]
        ),
        "under LEASE the skip marker must fire:\n{l_text}"
    );

    // ---- LEG 2: LEGACY — aborts exactly as today. ----
    let legacy_dir = root.join("legacy");
    ensure_dir_700(&legacy_dir);
    const LEGACY_BOB: &str = "na0741-t4-legacy-bob-tok-ccccccccc";
    let (g_alice, g_bob) = setup(&legacy_dir, "na0741-t4-legacy-alice-tok-ddddddd", LEGACY_BOB);
    push_raw(&base, LEGACY_BOB, &frame);
    send_message(
        &g_alice,
        &base,
        &legacy_dir,
        "m1.bin",
        b"na0741 t4 legacy payload",
    );
    let g_out = legacy_dir.join("out");
    ensure_dir_700(&g_out);
    let (g_ok, g_text) = run_any(
        &g_bob,
        &receive_args(
            &base,
            LEGACY_BOB,
            "bob",
            g_out.to_str().expect("out"),
            "8",
            "legacy",
        ),
    );
    assert!(
        !g_ok,
        "E5: under LEGACY the same arrangement must abort EXACTLY as today. Legacy is \
         byte-unchanged in lane 1; if this leg passes, the skip is not gated on the ack mode:\n\
         {g_text}"
    );
    assert!(
        !g_text.contains("recv_frame_skipped"),
        "E5: the skip arm fired under LEGACY. It is gated on `AckMode::Lease` and must be \
         unreachable here:\n{g_text}"
    );
}

// ===========================================================================
// T5 — the skip marker leaks nothing.
// ===========================================================================

#[test]
fn the_skip_marker_leaks_nothing() {
    let _g = guard();
    let relay =
        common::start_qsl_server_with_store(2 * 1024 * 1024, 512, None, TEST_PULL_LEASE_SECS);
    let base = relay.base_url().to_string();
    let root = test_root("na0741_t5");

    // ⚠ A THIRD PARTY'S TOKEN. NA-0740 measured the responder's route token riding an invite reply
    // IN THE CLEAR AT BYTE 5. This value is never passed to any command in this arm, so any
    // appearance in the output came from the frame's CONTENT and nowhere else.
    const LEAK_CANARY: &str = "na0741-t5-leak-canary-route-token";
    const BOB: &str = "na0741-t5-bob-tok-bbbbbbbbbbbbbb";

    let frame = invite_resp_frame(LEAK_CANARY);

    // ---- CONTROL (i): FIXTURE NON-VACUITY — the token really is in the planted bytes. ----
    assert!(
        frame
            .windows(LEAK_CANARY.len())
            .any(|w| w == LEAK_CANARY.as_bytes()),
        "FIXTURE VACUOUS: the canary is not in the planted frame, so 'it did not leak' would be \
         true of a frame that never carried it"
    );

    let (_alice, bob) = setup(&root, "na0741-t5-alice-tok-aaaaaaaaaaaa", BOB);
    push_raw(&base, BOB, &frame);

    let out = root.join("out");
    ensure_dir_700(&out);
    let (ok, text) = run_any(
        &bob,
        &receive_args(&base, BOB, "bob", out.to_str().expect("out"), "8", "lease"),
    );
    assert!(ok, "the foreign frame must be skipped:\n{text}");
    assert!(
        has_marker_line(&text, "recv_frame_skipped", &["class=invite_resp"]),
        "the marker under test never fired, so this arm would prove nothing:\n{text}"
    );

    // ---- CONTROL (ii): DETECTOR NON-VACUITY. ----
    // ⚠ THE CONTROL THE FIRST DRAFT WAS MISSING. Without it, a detector that read only stdout
    // while the leak went to stderr — or searched for the raw token while the marker rendered it
    // encoded — would PASS and still be blind. `text` is stdout+stderr, both captured explicitly.
    let mut salted = text.clone();
    salted.push_str(LEAK_CANARY);
    assert!(
        salted.contains(LEAK_CANARY),
        "DETECTOR VACUOUS: the search cannot find the canary even when it is definitely present, \
         so its absence below is worthless"
    );

    // ---- THE ASSERTION. ----
    assert!(
        !text.contains(LEAK_CANARY),
        "LEAK: a third party's route token reached the operator's console through the skip \
         marker. The marker may carry NO field derived from the CONTENT of `item.data`; a length \
         is permitted (precedent: `meta_bucket`'s `orig=`).\n{text}"
    );
}

// ===========================================================================
// T6 — E2, THE WEDGE ITSELF. Both mailboxes of a completed invite receive cleanly.
// ===========================================================================

/// Drive a REAL invite handshake over the REAL relay to completion. The sequence is
/// `NA_0681_two_party_handshake.rs`'s, adopted wholesale via `na0689_p3_a2_stranding.rs`:
/// create → redeem → accept → finish.
///
/// It leaves the arrangement NA-0740 measured: an **A2 handshake frame** in the INVITER's ordinary
/// inbox (from `invite finish`) and an **invite reply** in the REDEEMER's (from `invite accept`).
/// Both wedged their mailbox before this lane, in both directions.
fn drive_invite_to_completion(inviter: &Path, redeemer: &Path, base: &str) {
    let code = run_ok(inviter, &["invite", "create", "--relay", base, "--ttl-secs", "3600"]);
    let code = code
        .lines()
        .find(|l| l.starts_with("QSLI-1-"))
        .expect("invite code on stdout")
        .trim()
        .to_string();

    let listing = run_ok(inviter, &["invite", "list"]);
    let invite_id = listing
        .lines()
        .find_map(|l| l.strip_prefix("invite="))
        .and_then(|l| l.split_whitespace().next())
        .expect("invite id")
        .to_string();

    run_ok(
        redeemer,
        &["invite", "redeem", "--code", &code, "--alias", "inviter"],
    );
    run_ok(
        inviter,
        &[
            "invite",
            "accept",
            "--invite-id",
            &invite_id,
            "--alias",
            "redeemer",
        ],
    );
    let finish = run_ok(
        redeemer,
        &["invite", "finish", "--alias", "inviter", "--relay", base],
    );
    assert!(
        finish.contains("invite_finish=ok"),
        "A2 was never produced, so this arm would measure nothing: {finish}"
    );
}

#[test]
fn both_mailboxes_of_a_completed_invite_receive_cleanly() {
    let _g = guard();
    let relay =
        common::start_qsl_server_with_store(2 * 1024 * 1024, 512, None, TEST_PULL_LEASE_SECS);
    let base = relay.base_url().to_string();
    let root = test_root("na0741_t6");

    const INVITER_INBOX: &str = "na0741-t6-inviter-tok-aaaaaaaaaa";
    const REDEEMER_INBOX: &str = "na0741-t6-redeemer-tok-bbbbbbbbb";
    let inviter = party(&root, "inviter", INVITER_INBOX);
    let redeemer = party(&root, "redeemer", REDEEMER_INBOX);
    drive_invite_to_completion(&inviter, &redeemer, &base);

    // ---- THE INVITER'S ORDINARY INBOX — residue: the A2 handshake frame. ----
    let i_out = root.join("inviter_out");
    ensure_dir_700(&i_out);
    let (i_ok, i_text) = run_any(
        &inviter,
        &receive_args(
            &base,
            INVITER_INBOX,
            "redeemer",
            i_out.to_str().expect("out"),
            "8",
            "lease",
        ),
    );
    assert!(
        i_ok,
        "ENG-0142, INVITER SIDE: an ordinary receive on the inviter's own inbox is wedged by the \
         A2 handshake frame the invite flow itself put there.\n{i_text}"
    );
    assert!(
        has_marker_line(&i_text, "recv_frame_skipped", &["class=handshake"]),
        "the A2 frame must be SKIPPED by class, not decoded:\n{i_text}"
    );

    // ---- THE REDEEMER'S ORDINARY INBOX — residue: the invite reply. ----
    let r_out = root.join("redeemer_out");
    ensure_dir_700(&r_out);
    let (r_ok, r_text) = run_any(
        &redeemer,
        &receive_args(
            &base,
            REDEEMER_INBOX,
            "inviter",
            r_out.to_str().expect("out"),
            "8",
            "lease",
        ),
    );
    assert!(
        r_ok,
        "ENG-0142, REDEEMER SIDE: an ordinary receive on the redeemer's own inbox is wedged by the \
         invite reply the invite flow itself put there.\n{r_text}"
    );
    assert!(
        has_marker_line(&r_text, "recv_frame_skipped", &["class=invite_resp"]),
        "the invite reply must be SKIPPED by class, not decoded:\n{r_text}"
    );
}

// ===========================================================================
// T7 — foreign litter at the head still delivers up to --max (§5.1).
//
// ⚠ **THE ONLY ARM WHOSE RED-FIRST CONTROL IS A PARTIAL BUILD.** Its red is produced by a tree
// carrying the classify-and-skip insert WITHOUT the round-condition change — where the batch exits
// 0 having delivered NOTHING. That trade — a loud `rc 1` for a SILENT under-delivery — is the
// regression this arm exists to forbid.
// ===========================================================================

#[test]
fn foreign_litter_at_the_head_still_delivers_up_to_max() {
    let _g = guard();
    // ⚠ PRODUCTION-LENGTH LEASE — see `PRODUCTION_PULL_LEASE_SECS`. A 1-second lease expires
    // between rounds and re-delivers the same head, which measures lease expiry rather than the
    // round condition this arm exists to pin.
    let relay =
        common::start_qsl_server_with_store(2 * 1024 * 1024, 512, None, PRODUCTION_PULL_LEASE_SECS);
    let base = relay.base_url().to_string();
    let root = test_root("na0741_t7");

    const BOB: &str = "na0741-t7-bob-tok-bbbbbbbbbbbbbb";
    let (alice, bob) = setup(&root, "na0741-t7-alice-tok-aaaaaaaaaaaa", BOB);

    // FOUR foreign frames at the head — exactly `--max`, so the first round is entirely litter.
    for i in 0..4u8 {
        let token = format!("na0741-t7-third-party-route-tok-{i}0");
        push_raw(&base, BOB, &invite_resp_frame(&token));
    }
    // Two real messages BEHIND the litter.
    let p1 = b"na0741 t7 payload one".to_vec();
    let p2 = b"na0741 t7 payload two".to_vec();
    send_message(&alice, &base, &root, "m1.bin", &p1);
    send_message(&alice, &base, &root, "m2.bin", &p2);

    let out = root.join("out");
    ensure_dir_700(&out);
    let (ok, text) = run_any(
        &bob,
        &receive_args(&base, BOB, "bob", out.to_str().expect("out"), "4", "lease"),
    );

    assert!(ok, "the litter must not abort the batch:\n{text}");
    assert!(
        recv_file_count(&out) >= 1,
        "SILENT UNDER-DELIVERY: `--max 4` against four foreign frames delivered NOTHING and still \
         exited 0. A skipped frame occupies a slot in `want` and contributes nothing to \
         `stats.count`, so the round condition must treat it exactly as it treats a control \
         envelope — otherwise the repair trades a loud `rc 1` for a silent zero.\n{text}"
    );
    assert!(
        has_marker_line(&text, "recv_frame_skipped", &["class=invite_resp"]),
        "the litter must be skipped by class:\n{text}"
    );
    // ⚠ EXACT, AND THAT IS THE POINT: four frames planted, four skips. A count ABOVE four means
    // the same frames were re-pulled and re-skipped on later rounds — the lease expired between
    // rounds — which is a different mechanism from the one this arm pins and would let the arm
    // pass while the tail went undelivered. `id=` cannot distinguish them (it renders
    // `<redacted>`), so the COUNT is the only available discriminator.
    assert!(
        has_marker_line(&text, "recv_skip_summary", &["count=4"]),
        "expected exactly 4 skips for 4 planted frames. A higher count means the head was \
         re-delivered and re-skipped across rounds instead of staying leased:\n{text}"
    );
}
