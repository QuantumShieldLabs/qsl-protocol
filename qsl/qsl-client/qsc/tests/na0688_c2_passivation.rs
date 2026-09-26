// NA-0688 / D622 C2 (R1a as amended A6, RULING A, §2.1) — PASSIVATION.
//
// ⚠ WHY THIS FILE CARRIES ITS OWN HANDSHAKE FIXTURE, AT REAL COST.
//
// The obvious host for these guards is the receipts fixture, and it would make every one of
// them VACUOUS. Those tests run with `QSC_QSP_SEED` + the seed fallback, which produces a
// DEGENERATE SELF-DH session (`dhr == dhs_pub`) — and `qsp_should_ratchet` returns `false`
// immediately for exactly that shape. A "an ack originates no DH boundary" assertion written
// there passes with NO SUPPRESSION IMPLEMENTED AT ALL: it observes a branch that could never
// have been taken. The DH branch is reachable only over a real handshake, so the handshake
// dance is replicated here from `handshake_mvp.rs`. A dead guard is worse than an expensive
// fixture; the cost is recorded rather than absorbed silently.
//
// WHAT IS BEING GUARDED — ⚠ A6 HAS SINCE BEEN REVERSED, AND THIS HEADER IS SWEPT TO MATCH.
//   A control send originates NOTHING: no reply boundary, no N/T fallback, no PQ reseed, no
//   advertisement — AND no establishment either. A6 originally carved establishment out as a
//   necessity every send could perform; that exception was measured to mint a fresh DH keypair
//   and advance the shared root, wedging sessions permanently and bidirectionally, so it was
//   reversed by operator ruling.
//   ENG-0086 finding 1 still holds — "the recipient's automatic ack becomes their first send" —
//   which is precisely why the receipt cannot simply be dropped: it is written to the durable
//   owed-receipt hold and flushed on the peer's first real send. See `crate::owed_receipts`.
//
// NA-0785 PLAN F03 / S6b — FF6: THE LEGACY MARKERS, RE-EXPRESSED ON THE HEAD'S DIRECTIONAL WIRE.
//   The fixture is now `common::init_real_pair` (two successor vaults of `profile::ACTIVE`,
//   pinned identities, a REAL handshake) over the Mock relay, whose raw mailbox is the
//   observation instrument (an observer's view, as E3 already argued). The head's directional path
//   emits NONE of the legacy origination markers: `qsp_dh_ratchet`, `qsp_pq_reseed` and
//   `qsp_scka_adv` are registered at src/output/event_tables.rs:528-530 and emitted by no source
//   line, and `receipt_owed` (:614) likewise; `receipt_send`/`receipt_flush` are emitted only by
//   `flush_owed_receipts` (src/lib.rs:1087-1127), whose store nothing but its own put-back writes
//   (:1119). What the head DOES put on the wire is observable by frame class:
//     NDR1 ................ a delivery receipt, fixed RECEIPT_LEN (src/directional_delivery.rs:11,
//                           prefix :46-53), keyed to the RECEIVING epoch (:30-45) — no send chain.
//     NDE1, byte 4 = 1 .... a boundary: a fresh sender DH and root transition (the directional
//                           analog of a DH ratchet), taken only when owner == role and due
//                           (demand, >= 4 since the last boundary, 900 s, or no send epoch:
//                           :602-606) — there is no "reply" reason and no pending_send_ratchet.
//     NDE1, byte 4 = 0 .... an ordinary frame (an application message or a control request).
//   Each legacy assertion is KEPT verbatim where it can still fail or serves as a tripwire, and
//   paired with the directional witness that carries its property (REPLACED); where the property
//   itself does not exist on the head it is RETIRED, the source line named. Lane map MAP_S6b.tsv.
//   A synthetic local run; it says nothing about production or the real relay deployment.

mod common;

use common::{PairRelay, RealPair, VaultFixture};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, MutexGuard, OnceLock};

const ROUTE_TOKEN_ALICE: &str = "na0688_c2_alice_route_token_abcdef";
const ROUTE_TOKEN_BOB: &str = "na0688_c2_bob_route_token_ghijklm";

/// The head's fixed delivery-receipt length (src/directional_delivery.rs:11, RECEIPT_PREFIX + 48).
const NDR1_LEN: usize = 113;

fn lane_lock() -> MutexGuard<'static, ()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
        .lock()
        .unwrap_or_else(|e| e.into_inner())
}

fn ensure_dir_700(p: &Path) {
    fs::create_dir_all(p).unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let _ = fs::set_permissions(p, fs::Permissions::from_mode(0o700));
    }
}

fn output_text(o: &std::process::Output) -> String {
    format!(
        "{}{}",
        String::from_utf8_lossy(&o.stdout),
        String::from_utf8_lossy(&o.stderr)
    )
}

fn run_ok(v: &VaultFixture, args: &[&str]) -> String {
    let out = v.run(args);
    let text = output_text(&out);
    assert!(out.status.success(), "command failed {args:?}\n{text}");
    text
}

fn session_path(cfg: &Path, peer: &str) -> PathBuf {
    cfg.join("qsp_sessions").join(format!("{peer}.qsv"))
}

fn send_msg(
    v: &VaultFixture,
    relay: &str,
    to: &str,
    body: &[u8],
    tag: &str,
    with_receipt: bool,
) -> String {
    let dir = v.iso.root.join("payloads");
    ensure_dir_700(&dir);
    let f = dir.join(format!("{tag}.bin"));
    fs::write(&f, body).unwrap();
    let mut args = vec![
        "send",
        "--transport",
        "relay",
        "--relay",
        relay,
        "--to",
        to,
        "--file",
        f.to_str().unwrap(),
    ];
    if with_receipt {
        args.extend_from_slice(&["--receipt", "delivered"]);
    }
    run_ok(v, &args)
}

fn recv_msg(
    v: &VaultFixture,
    relay: &str,
    mailbox: &str,
    from: &str,
    out: &Path,
    emit_receipts: bool,
) -> String {
    ensure_dir_700(out);
    let mut args = vec![
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
        "4",
        "--out",
        out.to_str().unwrap(),
    ];
    if emit_receipts {
        args.extend_from_slice(&["--emit-receipts", "delivered"]);
    }
    run_ok(v, &args)
}

/// What a send ORIGINATED, counted from its LEGACY markers. Kept as a tripwire: the head emits none
/// of these, so a non-zero count means the legacy origination path came back.
#[derive(Debug, Default, PartialEq, Eq)]
struct Origination {
    dh_boundaries: usize,
    dh_first_send: usize,
    dh_reply: usize,
    dh_fallback: usize,
    pq_reseeds: usize,
    advertisements: usize,
}

fn count_origination(output: &str) -> Origination {
    let mut o = Origination::default();
    for line in output.lines() {
        if line.contains("event=qsp_dh_ratchet") && line.contains("dir=send") {
            o.dh_boundaries += 1;
            if line.contains("reason=first_send") {
                o.dh_first_send += 1;
            } else if line.contains("reason=reply") {
                o.dh_reply += 1;
            } else if line.contains("reason=fallback") {
                o.dh_fallback += 1;
            }
        }
        if line.contains("event=qsp_pq_reseed") && line.contains("dir=send") {
            o.pq_reseeds += 1;
        }
        if line.contains("event=qsp_scka_adv") && line.contains("dir=send") {
            o.advertisements += 1;
        }
    }
    o
}

/// What a step PUT ON THE WIRE, by the head's frame classes (see the header). The measurement
/// instrument for E2/E3 and the assertion surface for the directional witnesses.
#[derive(Debug, Default, PartialEq, Eq)]
struct Frames {
    receipts: usize,
    receipts_malformed: usize,
    boundaries: usize,
    ordinary: usize,
    other: usize,
}

fn classify(frames: &[Vec<u8>]) -> Frames {
    let mut f = Frames::default();
    for w in frames {
        if w.starts_with(b"NDR1") {
            f.receipts += 1;
            if w.len() != NDR1_LEN {
                f.receipts_malformed += 1;
            }
        } else if w.starts_with(b"NDE1") && w.len() > 4 && w[4] == 1 {
            f.boundaries += 1;
        } else if w.starts_with(b"NDE1") && w.len() > 4 && w[4] == 0 {
            f.ordinary += 1;
        } else {
            f.other += 1;
        }
    }
    f
}

struct Fixture {
    pair: RealPair,
    alice_out: PathBuf,
    bob_out: PathBuf,
    relay: String,
    server: common::InboxTestServer,
}

impl Fixture {
    fn alice(&self) -> &VaultFixture {
        &self.pair.a
    }
    fn bob(&self) -> &VaultFixture {
        &self.pair.b
    }
    /// The raw frames waiting in a mailbox, observed WITHOUT consuming them.
    fn mailbox(&self, route: &str) -> Vec<Vec<u8>> {
        let items = self.server.drain_channel(route);
        self.server.replace_channel(route, items.clone());
        items
    }
    /// Run `step` and return what it added to `route`'s mailbox, by frame class.
    fn pushed_during<T>(&self, route: &str, step: impl FnOnce() -> T) -> (T, Frames) {
        let before = self.mailbox(route).len();
        let result = step();
        let after = self.mailbox(route);
        (result, classify(&after[before.min(after.len())..]))
    }
}

/// alice and bob hold a real session; alice has sent one message REQUESTING A RECEIPT and bob
/// has NOT yet received it. Bob's send chain is therefore still unseeded.
fn fixture(tag: &str) -> Fixture {
    let server = common::start_inbox_server(1024 * 1024, 64);
    let relay = server.base_url().to_string();
    let pair = common::init_real_pair(
        tag,
        PairRelay::Mock(&server),
        ("alice", ROUTE_TOKEN_ALICE),
        ("bob", ROUTE_TOKEN_BOB),
    );
    assert!(
        session_path(&pair.a.cfg, "bob").exists(),
        "alice session missing"
    );
    assert!(
        session_path(&pair.b.cfg, "alice").exists(),
        "bob session missing"
    );
    let alice_out = pair.a.iso.root.join("alice_out");
    let bob_out = pair.b.iso.root.join("bob_out");
    for d in [&alice_out, &bob_out] {
        ensure_dir_700(d);
    }
    send_msg(&pair.a, &relay, "bob", b"c2-first-from-alice", "m1", true);
    Fixture {
        pair,
        alice_out,
        bob_out,
        relay,
        server,
    }
}

// ---------------------------------------------------------------------------
// B1 BASELINE INSTRUMENT — the SAME instrument that becomes the guards below.
// Run it once before suppression and once after; the numbers are E2.
// ---------------------------------------------------------------------------

/// E2 — what a delivery receipt originates, MEASURED rather than argued.
///
/// ⚠ TOLERANT BY DESIGN. Before suppression, an ack takes the ratchet-on-reply boundary and
/// the session desynchronises — `REJECT_S2_HDR_AUTH_FAIL` — which is ENG-0086 finding 1
/// happening rather than being predicted. A measurement that asserted success would panic on
/// the very behaviour it exists to record, so the receive is run TOLERANTLY here. The guards
/// below are the ones that assert. (S6b: the head's frame classes are measured beside the legacy
/// marker counts.)
#[test]
fn e2_measure_what_an_ack_originates() {
    let _g = lane_lock();
    let f = fixture("na0688_c2_e2");

    // Bob's chain is UNSEEDED here: alice has sent, bob has not. ENG-0086 finding 1's case.
    let (out1, wire1) = f.pushed_during(ROUTE_TOKEN_ALICE, || {
        f.bob().run(&[
            "receive",
            "--transport",
            "relay",
            "--relay",
            &f.relay,
            "--mailbox",
            ROUTE_TOKEN_BOB,
            "--from",
            "alice",
            "--max",
            "4",
            "--out",
            f.bob_out.to_str().unwrap(),
            "--emit-receipts",
            "delivered",
        ])
    });
    let text1 = output_text(&out1);
    let first = count_origination(&text1);

    // A user reply from bob, for the like-with-like comparison E3 needs.
    let (reply, reply_wire) = f.pushed_during(ROUTE_TOKEN_BOB, || {
        send_msg(f.alice(), &f.relay, "bob", b"c2-reply-probe", "rp", false)
    });
    let reply_counts = count_origination(&reply);

    println!("=== E2 MEASUREMENT — BEFORE SUPPRESSION (NA-0688 C2) ===");
    println!("receive-with-ack succeeded : {}", out1.status.success());
    println!("ack origination            : {first:?}");
    println!("ack wire (head classes)    : {wire1:?}");
    println!("user send origination      : {reply_counts:?}");
    println!("user send wire             : {reply_wire:?}");
    println!(
        "session broke              : {}",
        text1.contains("REJECT_S2_HDR_AUTH_FAIL")
    );
    println!("=== END E2 ===");
}

// ---------------------------------------------------------------------------
// THE GUARDS. One per origination branch, plus RULING A's deferred rotation, plus §2.1.
// All of them run over the REAL handshake above, so every branch they assert about is
// actually reachable. On the seeded fixture they would pass without any suppression at all.
// ---------------------------------------------------------------------------

/// Give bob an ESTABLISHED sending chain, by the only route that now exists — and keep BOTH sides
/// in step while doing it.
///
/// ⚠ THE ROUND-TRIP IS NOT OPTIONAL, and a one-sided warm-up was measured to break these fixtures
/// outright (`qsp_scka_adv code=qsp_auth_failed dir=recv`). Bob's first send is a DH boundary that
/// moves the shared root; if alice never receives it, her already-sent advertisement was
/// authenticated under the OLD root and bob can no longer verify it. That is the same shape as the
/// wedge this lane exists to close, arriving from the other side — so the warm-up drains bob's
/// message on alice's side before any guard runs.
///
/// A6 was REVERSED: an ack no longer establishes, so bob's first receive DEFERS its receipt to the
/// owed-receipt hold. Every guard here is about what an ack does OVER AN ESTABLISHED CHAIN, so the
/// fixture must hand bob one. **This changes the fixture only — not one assertion below moves.**
/// Without it the guards would not weaken, they would go VACUOUS, and each says so itself ("the
/// fixture must actually ack, or this guard is vacuous").
fn warm_up_bobs_chain(f: &Fixture) {
    // Bob drains alice's opening message; his receipt is OWED, not sent (no chain yet).
    recv_msg(
        f.bob(),
        &f.relay,
        ROUTE_TOKEN_BOB,
        "alice",
        &f.bob_out,
        true,
    );
    // Bob's own send establishes the chain and flushes what he owed.
    send_msg(f.bob(), &f.relay, "alice", b"c2-warmup", "warm", false);
    // ⚠ Alice MUST take bob's boundary, or the two roots diverge and nothing below authenticates.
    recv_msg(
        f.alice(),
        &f.relay,
        ROUTE_TOKEN_ALICE,
        "bob",
        &f.alice_out,
        false,
    );
}

/// Drive bob to an ESTABLISHED chain, then have him ack again. Returns (ack output, what the ack
/// put on alice's wire, fixture).
fn established_chain_ack(tag: &str) -> (String, Frames, Fixture) {
    let f = fixture(tag);
    warm_up_bobs_chain(&f);
    // Alice sends again; bob acks over an ESTABLISHED chain.
    send_msg(f.alice(), &f.relay, "bob", b"c2-second", "m2", true);
    let (out, wire) = f.pushed_during(ROUTE_TOKEN_ALICE, || {
        recv_msg(
            f.bob(),
            &f.relay,
            ROUTE_TOKEN_BOB,
            "alice",
            &f.bob_out,
            true,
        )
    });
    (out, wire, f)
}

/// GUARD — R1a: over an ESTABLISHED chain a control send originates NOTHING.
/// One assertion per suppressed branch, so a regression names which branch came back.
#[test]
fn an_ack_over_an_established_chain_originates_nothing() {
    let _g = lane_lock();
    let (out, wire, _f) = established_chain_ack("na0688_c2_g1");
    // REPLACED (`event=receipt_send` is not emitted on the head): the ack is on the wire as NDR1
    // receipt frames, each of the fixed receipt length.
    assert!(
        wire.receipts >= 1 && wire.receipts_malformed == 0,
        "the fixture must actually ack, or this guard is vacuous: {wire:?}\n{out}"
    );
    let o = count_origination(&out);
    assert_eq!(
        o.dh_reply, 0,
        "an ack must not take the ratchet-on-REPLY boundary:\n{out}"
    );
    assert_eq!(
        o.dh_fallback, 0,
        "an ack must not take the N/T FALLBACK boundary:\n{out}"
    );
    assert_eq!(
        o.pq_reseeds, 0,
        "an ack must not originate a PQ RESEED:\n{out}"
    );
    assert_eq!(
        o.advertisements, 0,
        "an ack must not mint an SCKA ADVERTISEMENT:\n{out}"
    );
    assert_eq!(
        o.dh_boundaries, 0,
        "over an established chain there is nothing left to establish, so an ack must \
         originate no boundary at all:\n{out}"
    );
    // The directional witness for the reply / fallback / any-boundary guards above: over an
    // established chain the ack puts NO boundary frame on the wire.
    assert_eq!(
        wire.boundaries, 0,
        "over an established chain an ack must put no boundary (NDE1 byte 4 = 1) on the wire: \
         {wire:?}\n{out}"
    );
}

/// GUARD — §2.1 as narrowed: no persistent write from `qsp_pack` on a control send over an
/// established chain.
///
/// ⚠ WHY THE MARKERS ARE A SUFFICIENT OBSERVABLE, and this is the load-bearing part.
/// `qsp_scka_store` is the only persistent write `qsp_pack` performs, and it is gated on
/// `scka_dirty`, which the D622 P1 side-effect inventory enumerated as being set in EXACTLY
/// FOUR places — all four inside the three origination branches (advertisement, DH boundary,
/// PQ reseed ok, PQ reseed encap-fail). Zero origination therefore implies `scka_dirty` stays
/// false and the store never runs. The store cannot be observed directly by file identity
/// because it writes into the session blob, which every send touches anyway.
#[test]
fn an_ack_over_an_established_chain_writes_nothing_persistent_from_pack() {
    let _g = lane_lock();
    let (out, wire, _f) = established_chain_ack("na0688_c2_g2");
    let o = count_origination(&out);
    assert_eq!(
        (o.dh_boundaries, o.pq_reseeds, o.advertisements),
        (0, 0, 0),
        "all four `scka_dirty` sites live inside these three branches; any one of them firing \
         means `qsp_scka_store` ran on a control send:\n{out}"
    );
    // The directional witness: the ack originates no boundary and nothing of an unknown class.
    assert_eq!(
        (wire.boundaries, wire.other),
        (0, 0),
        "an ack over an established chain must originate no boundary and no unknown frame: \
         {wire:?}\n{out}"
    );
}

/// GUARD — **THE POST-REVERSAL LAW: an ack on an unseeded chain ORIGINATES NOTHING and OWES the
/// receipt.**
///
/// ⚠⚠ THIS GUARD WAS INVERTED, NOT WEAKENED, AND THE DISTINCTION IS THE WHOLE POINT.
///
/// It was `an_ack_on_an_unseeded_chain_establishes_and_only_establishes`, and it pinned **ruling
/// A6**: that an ack on an unseeded chain DOES establish, reporting `reason=first_send`. **A6 was
/// REVERSED by operator ruling**, so its subject law no longer exists — and a guard whose subject
/// has been overturned is not migrated by moving a value, it is turned to face the other way.
///
/// **Why A6 was reversed, in one line:** `send_boundary` is the only way the refimpl can seed a
/// send chain, and it MINTS A FRESH DH KEYPAIR AND ADVANCES THE SHARED ROOT — so an establishing
/// ack moved the recipient's key, and a sender who had not pulled that ack then computed a
/// boundary against a stale one. Measured: a **permanent, bidirectional wedge**, with the sender's
/// own pull failing too. `handshake_mvp::a_first_send_ack_never_wedges_the_session` is the
/// regression pin for that.
///
/// ⚠ RED-CAPABLE IN **BOTH** DIRECTIONS, which is what stops an inversion from becoming a hole:
///   * it fails if an unseeded-chain ack **establishes** again (a regression back to A6), and
///   * it fails if the receipt is **silently dropped** instead of owed — the failure mode that
///     made plain refusal unacceptable, since alice would sit on SENT forever.
///
/// Asserting only the first would let the receipt vanish; asserting only the second would let the
/// keypair mint return.
///
/// S6b: on the head the receipt needs no send chain at all — it is an NDR1 frame keyed to the
/// RECEIVING epoch — so it is sent at once, never owed. DIRECTION 2 therefore proves "not dropped"
/// end to end (alice's message reaches DELIVERED). DIRECTION 1's ack is the NDR1 frame; the boundary
/// the head's receive may push beside it is the directional core's receive maintenance (B's owner
/// transition), whose no-wedge property is pinned by
/// `handshake_mvp::a_first_send_ack_never_wedges_the_session` and `f03_crossed_send.rs`.
#[test]
fn an_ack_on_an_unseeded_chain_originates_nothing_and_owes_the_receipt() {
    let _g = lane_lock();
    let f = fixture("na0688_c2_g3");
    let (out, wire) = f.pushed_during(ROUTE_TOKEN_ALICE, || {
        recv_msg(
            f.bob(),
            &f.relay,
            ROUTE_TOKEN_BOB,
            "alice",
            &f.bob_out,
            true,
        )
    });
    println!("unseeded-chain ack wire (head classes): {wire:?}");

    // DIRECTION 1 — NOTHING is originated. Not a boundary, not a reseed, not an advertisement.
    let o = count_origination(&out);
    assert_eq!(
        o.dh_boundaries, 0,
        "an ack on an UNSEEDED chain must originate NO boundary — establishment mints a keypair          and advances the shared root, which is exactly what wedged the session:
{out}"
    );
    assert_eq!(
        o.dh_first_send, 0,
        "and specifically no `reason=first_send`, the marker A6 used to require here:
{out}"
    );
    assert_eq!(
        (o.dh_reply, o.dh_fallback, o.pq_reseeds, o.advertisements),
        (0, 0, 0, 0),
        "a control send originates nothing at all — no rotation, no reseed, no advertisement:
{out}"
    );
    // The directional witness: the ack itself is a bare NDR1 receipt of the fixed length — it can
    // carry no DH, no root transition and no advertisement.
    assert!(
        wire.receipts >= 1 && wire.receipts_malformed == 0 && wire.other == 0,
        "the ack must be a fixed-length NDR1 receipt and nothing of an unknown class: {wire:?}\n{out}"
    );

    // DIRECTION 2 — the receipt is not dropped. REPLACED (the head emits no `receipt_owed`; its
    // receipt needs no chain and leaves at once): alice's first message reaches DELIVERED.
    let alice_recv = recv_msg(
        f.alice(),
        &f.relay,
        ROUTE_TOKEN_ALICE,
        "bob",
        &f.alice_out,
        false,
    );
    assert!(
        alice_recv.contains("event=message_state_transition from=SENT to=DELIVERED"),
        "the receipt must reach alice — a client that dropped it would pass the origination \
         assertions above while losing the first receipt of every conversation:\n{out}\n{alice_recv}"
    );
    assert!(
        !out.contains("event=receipt_send"),
        "and it must NOT have been sent: there is no chain to send it on:
{out}"
    );
}

/// GUARD — RULING A, deferred rotation, BOTH halves.
///
/// An ack must not rotate when rotation is due, AND must not consume the due-state. The
/// second half is the one that matters most: an ack that cleared `pending_send_ratchet`
/// without rotating would be strictly worse than an ack that rotated — the human's reply
/// boundary would simply vanish, silently, with no marker anywhere.
///
/// S6b: HALF 2's subject — the ratchet-on-reply due-state `pending_send_ratchet`, taken by the next
/// user send with `reason=reply` — does not exist on the head's directional path (the field survives
/// only in the legacy session record, src/protocol_state/mod.rs:270; the directional boundary is
/// decided at src/directional_delivery.rs:602-606 with no reply reason). HALF 2 is RETIRED; the
/// head's "each side's rotation is taken" property is pinned by
/// `handshake_mvp::dh_ratchet_e2e_roundtrip_over_real_handshake` (both fresh DH/root transitions
/// authenticated). The user reply itself is still sent and must still reach alice.
#[test]
fn a_due_rotation_survives_an_ack_and_is_taken_by_the_next_user_send() {
    let _g = lane_lock();
    let f = fixture("na0688_c2_g4");

    warm_up_bobs_chain(&f);

    // Alice sends again. Bob receiving this sets `pending_send_ratchet`: a rotation is DUE.
    send_msg(
        f.alice(),
        &f.relay,
        "bob",
        b"c2-makes-rotation-due",
        "m3",
        true,
    );
    let (ack, wire) = f.pushed_during(ROUTE_TOKEN_ALICE, || {
        recv_msg(
            f.bob(),
            &f.relay,
            ROUTE_TOKEN_BOB,
            "alice",
            &f.bob_out,
            true,
        )
    });

    // HALF 1 — the ack did not rotate.
    let o = count_origination(&ack);
    assert_eq!(
        o.dh_boundaries, 0,
        "rotation was DUE and a control send must not take it:\n{ack}"
    );
    assert_eq!(
        wire.boundaries, 0,
        "rotation was DUE and the ack must put no boundary on the wire: {wire:?}\n{ack}"
    );

    // HALF 2 — bob's next USER send still goes out and reaches alice.
    send_msg(
        f.bob(),
        &f.relay,
        "alice",
        b"c2-bobs-real-reply",
        "br",
        false,
    );
    let reply_out = f.alice_out.join("reply");
    let got = recv_msg(
        f.alice(),
        &f.relay,
        ROUTE_TOKEN_ALICE,
        "bob",
        &reply_out,
        false,
    );
    let bodies: Vec<Vec<u8>> = fs::read_dir(&reply_out)
        .unwrap()
        .map(|e| fs::read(e.unwrap().path()).unwrap())
        .collect();
    assert!(
        bodies.iter().any(|b| b == b"c2-bobs-real-reply"),
        "bob's user reply after the ack must reach alice:\n{got}"
    );
}

/// GUARD — the user path is UNTOUCHED. A user send still rotates exactly as before; the
/// witness D622 names for this is `handshake_mvp::dh_ratchet_e2e_roundtrip_over_real_handshake`,
/// which runs unmodified in the suite. This is the same property asserted locally, so a
/// regression is attributable to C2 rather than surfacing in a distant file.
///
/// S6b: the head takes B's fresh boundary as RECEIVE MAINTENANCE (see handshake_mvp's
/// "Receive maintenance can have emitted B's boundary"), and the head sends every receipt, so
/// "bob's reply path" is his receive plus his user send: across them bob must put a boundary on the
/// wire.
#[test]
fn a_user_reply_still_rotates_the_ratchet() {
    let _g = lane_lock();
    let f = fixture("na0688_c2_g5");
    let ((_, user_send), wire) = f.pushed_during(ROUTE_TOKEN_ALICE, || {
        // Bob receives WITHOUT requesting receipts, so nothing but his own reply path can rotate.
        let r = recv_msg(
            f.bob(),
            &f.relay,
            ROUTE_TOKEN_BOB,
            "alice",
            &f.bob_out,
            false,
        );
        let s = send_msg(f.bob(), &f.relay, "alice", b"c2-user-reply", "ur", false);
        (r, s)
    });
    // REPLACED: the legacy count (`qsp_dh_ratchet dir=send`, no emit site on the head) cannot
    // witness this; bob's side must put a boundary (NDE1 byte 4 = 1) on the wire.
    println!(
        "user send legacy origination: {:?}",
        count_origination(&user_send)
    );
    assert!(
        wire.boundaries >= 1,
        "a USER reply must still originate a boundary — passivation must not have suppressed \
         the human path:\n{user_send}\n{wire:?}"
    );
}

/// E3 — on-wire envelope distinguishability, receipt vs reply, LIKE WITH LIKE.
///
/// ⚠ THE INSTRUMENT IS THE RELAY, NOT THE RECEIVER'S MARKERS. The first attempt read
/// `meta_bucket ... metric=envelope_len` from the receiving client and could only ever see
/// ONE of the two envelopes — because an ack is consumed as `receipt_recv` and never becomes
/// a `recv_item` at all. That invisibility is the feature working (design §5), and it makes
/// the receiver blind to exactly the thing E3 must measure. Reading the raw bytes the mock
/// relay stored measures what an observer of the relay would actually see.
///
/// The comparison is like-with-like: both envelopes are bob's, both leave the same session,
/// and they are compared as they sat in the same mailbox.
///
/// ⚠ TWO CORRECTIONS MADE AT C3, BOTH BECAUSE THE FIRST FORM OF THIS INSTRUMENT MISLED.
///
///  1. **It read positionally and reported a subset.** The C2 run measured `[1024, 1320, 1212]`
///     and was recorded as "ack 1024 vs user reply 1212" — the 1320 (an SCKA advertisement
///     PRE-envelope) was dropped without comment, and nothing in the instrument said which
///     index was which. It now DRAINS BETWEEN STEPS, so every number is labelled by
///     construction rather than by the reader's assumption.
///  2. **It took one user-message sample, and that sample was not representative.** At C2 bob's
///     reply happened to carry a PQ RESEED (1212 bytes), because the establishing ack had eaten
///     his due rotation. With that defect fixed his reply takes a plain DH boundary and measures
///     **1024 — identical to the ack** — purely because a 20-byte body pads up to the same
///     Standard floor. A one-sample instrument would have flipped the R2b conclusion on what is
///     an artefact of the body size chosen. It now takes a SHORT sample (under the floor) and a
///     LONG one (over it), so the answer does not depend on which body the fixture picked.
///
/// S6b: the LONG sample is 2048 bytes, not 4096. It must still exceed the 1024 floor, and the head
/// caps a padded directional message at 4096 bytes (src/directional_delivery.rs:289-296, refused
/// INTEGRATION_PADDING_SIZE), which a 4096-byte body exceeds once its framing is added.
#[test]
fn e3_measure_envelope_distinguishability() {
    let _g = lane_lock();
    let f = fixture("na0688_c2_e3");

    // ⚠ NA-0688 WARM-UP: after the A6 reversal an ack cannot establish, so bob needs a chain of
    // his own before his ack can exist at all — the earlier form measured `ack=[]`.
    warm_up_bobs_chain(&f);
    send_msg(f.alice(), &f.relay, "bob", b"e3-trigger", "e3t", true);
    let _ = drained_lens(&f, ROUTE_TOKEN_ALICE); // discard everything the warm-up put on the wire

    // STEP 1 — bob acks alice's message over his established chain. Drained immediately, so what
    // comes back is unambiguously the ack.
    recv_msg(
        f.bob(),
        &f.relay,
        ROUTE_TOKEN_BOB,
        "alice",
        &f.bob_out,
        true,
    );
    let ack_lens = drained_lens(&f, ROUTE_TOKEN_ALICE);

    // STEP 2 — a SHORT user reply: 20 bytes, well under the Standard 1024 floor.
    send_msg(
        f.bob(),
        &f.relay,
        "alice",
        b"bobs-user-reply-body",
        "e3r",
        false,
    );
    let short_lens = drained_lens(&f, ROUTE_TOKEN_ALICE);

    // STEP 3 — a LONG user reply: 2048 bytes, unambiguously over the floor.
    let long_body = vec![b'x'; 2048];
    send_msg(f.bob(), &f.relay, "alice", &long_body, "e3l", false);
    let long_lens = drained_lens(&f, ROUTE_TOKEN_ALICE);

    println!("=== E3 MEASUREMENT — envelope lengths as they sat on the relay ===");
    println!("bob -> alice  ack only         : {ack_lens:?}");
    println!("bob -> alice  SHORT user reply : {short_lens:?}");
    println!("bob -> alice  LONG user reply  : {long_lens:?}");
    println!("=== END E3 ===");

    // ⚠ Refuse a comparison that was never made. Every arm must have produced something, or the
    // numbers above are a conclusion drawn from an empty mailbox.
    assert!(
        !ack_lens.is_empty() && !short_lens.is_empty() && !long_lens.is_empty(),
        "E3 needs all three arms to compare; got ack={ack_lens:?} short={short_lens:?} \
         long={long_lens:?}"
    );
}

/// The raw bytes the mock relay holds for a channel, drained — an observer's view.
///
/// ⚠ A send may push PRE-ENVELOPES (an SCKA advertisement) ahead of its main envelope, so an arm
/// can legitimately return more than one length. The MAIN envelope is the LAST one pushed
/// (`qsp_pack` pushes `pre_envelopes` first, then `pack.envelope`), and the whole vector is
/// printed so that reading is checkable instead of asserted.
fn drained_lens(f: &Fixture, channel: &str) -> Vec<usize> {
    f.server
        .drain_channel(channel)
        .iter()
        .map(|e| e.len())
        .collect()
}
