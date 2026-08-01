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

mod common;

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, MutexGuard, OnceLock};

const ROUTE_TOKEN_ALICE: &str = "na0688_c2_alice_route_token_abcdef";
const ROUTE_TOKEN_BOB: &str = "na0688_c2_bob_route_token_ghijklm";

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

fn test_root(tag: &str) -> PathBuf {
    let root = std::env::var("CARGO_TARGET_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("target"))
        .join("qsc-test-tmp")
        .join(format!("{tag}_{}", std::process::id()));
    let _ = fs::remove_dir_all(&root);
    ensure_dir_700(&root);
    root
}

fn output_text(o: &std::process::Output) -> String {
    format!(
        "{}{}",
        String::from_utf8_lossy(&o.stdout),
        String::from_utf8_lossy(&o.stderr)
    )
}

fn qsc_cfg_cmd(cfg: &Path) -> std::process::Command {
    let mut cmd = common::qsc_std_command();
    cmd.env("QSC_CONFIG_DIR", cfg);
    cmd
}

fn run_qsc(cfg: &Path, args: &[&str]) -> std::process::Output {
    qsc_cfg_cmd(cfg).args(args).output().expect("qsc command")
}

fn run_ok(cfg: &Path, args: &[&str]) -> String {
    let out = run_qsc(cfg, args);
    let text = output_text(&out);
    assert!(out.status.success(), "command failed {args:?}\n{text}");
    text
}

fn init_identity(cfg: &Path, label: &str) {
    run_ok(cfg, &["identity", "rotate", "--as", label, "--confirm"]);
}

fn identity_field(cfg: &Path, label: &str, field: &str) -> String {
    let text = run_ok(cfg, &["identity", "show", "--as", label]);
    let prefix = format!("{field}=");
    for line in text.lines() {
        if let Some(v) = line.strip_prefix(prefix.as_str()) {
            return v.to_string();
        }
    }
    panic!("missing {field} in output: {text}");
}

#[allow(clippy::too_many_arguments)]
fn contacts_add_pinned_with_route(
    cfg: &Path,
    label: &str,
    fp: &str,
    kem_pk: &str,
    sig_pk: &str,
    token: &str,
) {
    run_ok(
        cfg,
        &[
            "contacts",
            "add",
            "--label",
            label,
            "--fp",
            fp,
            "--kem-pk",
            kem_pk,
            "--sig-pk",
            sig_pk,
            "--route-token",
            token,
        ],
    );
}

fn session_path(cfg: &Path, peer: &str) -> PathBuf {
    cfg.join("qsp_sessions").join(format!("{peer}.qsv"))
}

/// A REAL two-party handshake — the whole reason this file exists. Replicated from
/// `handshake_mvp::hs_dance`; the session it produces has `dhr != dhs_pub`, so
/// `qsp_should_ratchet` is actually reachable.
fn hs_dance(alice_cfg: &Path, bob_cfg: &Path, relay: &str) {
    init_identity(alice_cfg, "alice");
    init_identity(bob_cfg, "bob");
    let a_fp = identity_field(alice_cfg, "alice", "identity_fp");
    let a_kem = identity_field(alice_cfg, "alice", "identity_kem_pk");
    let a_sig = identity_field(alice_cfg, "alice", "identity_sig_pk");
    let b_fp = identity_field(bob_cfg, "bob", "identity_fp");
    let b_kem = identity_field(bob_cfg, "bob", "identity_kem_pk");
    let b_sig = identity_field(bob_cfg, "bob", "identity_sig_pk");
    contacts_add_pinned_with_route(alice_cfg, "bob", &b_fp, &b_kem, &b_sig, ROUTE_TOKEN_BOB);
    contacts_add_pinned_with_route(bob_cfg, "alice", &a_fp, &a_kem, &a_sig, ROUTE_TOKEN_ALICE);
    run_ok(
        alice_cfg,
        &["relay", "inbox-set", "--token", ROUTE_TOKEN_ALICE],
    );
    run_ok(bob_cfg, &["relay", "inbox-set", "--token", ROUTE_TOKEN_BOB]);

    run_ok(
        alice_cfg,
        &[
            "handshake",
            "init",
            "--as",
            "alice",
            "--peer",
            "bob",
            "--relay",
            relay,
        ],
    );
    for (cfg, me, peer) in [
        (bob_cfg, "bob", "alice"),
        (alice_cfg, "alice", "bob"),
        (bob_cfg, "bob", "alice"),
    ] {
        run_ok(
            cfg,
            &[
                "handshake",
                "poll",
                "--as",
                me,
                "--peer",
                peer,
                "--relay",
                relay,
                "--max",
                "4",
            ],
        );
    }
    assert!(
        session_path(alice_cfg, "bob").exists(),
        "alice session missing"
    );
    assert!(
        session_path(bob_cfg, "alice").exists(),
        "bob session missing"
    );
}

fn send_msg(
    cfg: &Path,
    relay: &str,
    to: &str,
    body: &[u8],
    tag: &str,
    with_receipt: bool,
) -> String {
    let f = cfg.join(format!("{tag}.bin"));
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
    run_ok(cfg, &args)
}

fn recv_msg(
    cfg: &Path,
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
    run_ok(cfg, &args)
}

/// What a send ORIGINATED, counted from its markers. This is the measurement instrument for
/// E2 and the assertion surface for the guards — one instrument for both, as B1 requires.
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

struct Fixture {
    _root: PathBuf,
    alice: PathBuf,
    bob: PathBuf,
    alice_out: PathBuf,
    bob_out: PathBuf,
    relay: String,
    server: common::InboxTestServer,
}

/// alice and bob hold a real session; alice has sent one message REQUESTING A RECEIPT and bob
/// has NOT yet received it. Bob's send chain is therefore still unseeded.
fn fixture(tag: &str) -> Fixture {
    let root = test_root(tag);
    let alice = root.join("alice");
    let bob = root.join("bob");
    let alice_out = root.join("alice_out");
    let bob_out = root.join("bob_out");
    for d in [&alice, &bob, &alice_out, &bob_out] {
        ensure_dir_700(d);
    }
    common::init_mock_vault(&alice);
    common::init_mock_vault(&bob);
    let server = common::start_inbox_server(1024 * 1024, 64);
    let relay = server.base_url().to_string();
    hs_dance(&alice, &bob, &relay);
    send_msg(&alice, &relay, "bob", b"c2-first-from-alice", "m1", true);
    Fixture {
        _root: root,
        alice,
        bob,
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
/// below are the ones that assert.
#[test]
fn e2_measure_what_an_ack_originates() {
    let _g = lane_lock();
    let f = fixture("na0688_c2_e2");

    // Bob's chain is UNSEEDED here: alice has sent, bob has not. ENG-0086 finding 1's case.
    let out1 = run_qsc(
        &f.bob,
        &[
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
        ],
    );
    let text1 = output_text(&out1);
    let first = count_origination(&text1);

    // A user reply from bob, for the like-with-like comparison E3 needs.
    let reply = send_msg(&f.alice, &f.relay, "bob", b"c2-reply-probe", "rp", false);
    let reply_counts = count_origination(&reply);

    println!("=== E2 MEASUREMENT — BEFORE SUPPRESSION (NA-0688 C2) ===");
    println!("receive-with-ack succeeded : {}", out1.status.success());
    println!("ack origination            : {first:?}");
    println!("user send origination      : {reply_counts:?}");
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
    recv_msg(&f.bob, &f.relay, ROUTE_TOKEN_BOB, "alice", &f.bob_out, true);
    // Bob's own send establishes the chain and flushes what he owed.
    send_msg(&f.bob, &f.relay, "alice", b"c2-warmup", "warm", false);
    // ⚠ Alice MUST take bob's boundary, or the two roots diverge and nothing below authenticates.
    recv_msg(&f.alice, &f.relay, ROUTE_TOKEN_ALICE, "bob", &f.alice_out, false);
}

/// Drive bob to an ESTABLISHED chain, then have him ack again. Returns (ack output, fixture).
fn established_chain_ack(tag: &str) -> (String, Fixture) {
    let f = fixture(tag);
    warm_up_bobs_chain(&f);
    // Alice sends again; bob acks over an ESTABLISHED chain.
    send_msg(&f.alice, &f.relay, "bob", b"c2-second", "m2", true);
    let out = recv_msg(&f.bob, &f.relay, ROUTE_TOKEN_BOB, "alice", &f.bob_out, true);
    (out, f)
}

/// GUARD — R1a: over an ESTABLISHED chain a control send originates NOTHING.
/// One assertion per suppressed branch, so a regression names which branch came back.
#[test]
fn an_ack_over_an_established_chain_originates_nothing() {
    let _g = lane_lock();
    let (out, _f) = established_chain_ack("na0688_c2_g1");
    assert!(
        out.contains("event=receipt_send"),
        "the fixture must actually ack, or this guard is vacuous:\n{out}"
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
    let (out, _f) = established_chain_ack("na0688_c2_g2");
    let o = count_origination(&out);
    assert_eq!(
        (o.dh_boundaries, o.pq_reseeds, o.advertisements),
        (0, 0, 0),
        "all four `scka_dirty` sites live inside these three branches; any one of them firing \
         means `qsp_scka_store` ran on a control send:\n{out}"
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
/// Asserting only the first would let the receipt vanish; asserting only the second would let the
/// keypair mint return.
#[test]
fn an_ack_on_an_unseeded_chain_originates_nothing_and_owes_the_receipt() {
    let _g = lane_lock();
    let f = fixture("na0688_c2_g3");
    let out = recv_msg(&f.bob, &f.relay, ROUTE_TOKEN_BOB, "alice", &f.bob_out, true);

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

    // DIRECTION 2 — the receipt is OWED, not dropped. Without this the guard above would be
    // satisfied by a client that simply threw the ack away.
    assert!(
        out.contains("event=receipt_owed"),
        "the receipt must be recorded to the durable hold — a client that dropped it would pass          the origination assertions above while losing the first receipt of every          conversation:
{out}"
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
#[test]
fn a_due_rotation_survives_an_ack_and_is_taken_by_the_next_user_send() {
    let _g = lane_lock();
    let f = fixture("na0688_c2_g4");

    warm_up_bobs_chain(&f);

    // Alice sends again. Bob receiving this sets `pending_send_ratchet`: a rotation is DUE.
    send_msg(
        &f.alice,
        &f.relay,
        "bob",
        b"c2-makes-rotation-due",
        "m3",
        true,
    );
    let ack = recv_msg(&f.bob, &f.relay, ROUTE_TOKEN_BOB, "alice", &f.bob_out, true);

    // HALF 1 — the ack did not rotate.
    let o = count_origination(&ack);
    assert_eq!(
        o.dh_boundaries, 0,
        "rotation was DUE and a control send must not take it:\n{ack}"
    );

    // HALF 2 — the due-state SURVIVED, and bob's next USER send takes it.
    let user_send = send_msg(
        &f.bob,
        &f.relay,
        "alice",
        b"c2-bobs-real-reply",
        "br",
        false,
    );
    let u = count_origination(&user_send);
    assert_eq!(
        u.dh_reply, 1,
        "the ack must LEAVE the due-state intact so the next USER send rotates with \
         reason=reply. Zero here means the ack silently consumed the human's reply \
         boundary — worse than rotating on the ack:\n{user_send}"
    );
}

/// GUARD — the user path is UNTOUCHED. A user send still rotates exactly as before; the
/// witness D622 names for this is `handshake_mvp::dh_ratchet_e2e_roundtrip_over_real_handshake`,
/// which runs unmodified in the suite. This is the same property asserted locally, so a
/// regression is attributable to C2 rather than surfacing in a distant file.
#[test]
fn a_user_reply_still_rotates_the_ratchet() {
    let _g = lane_lock();
    let f = fixture("na0688_c2_g5");
    // Bob receives WITHOUT acking, so nothing but his own send can rotate.
    recv_msg(
        &f.bob,
        &f.relay,
        ROUTE_TOKEN_BOB,
        "alice",
        &f.bob_out,
        false,
    );
    let user_send = send_msg(&f.bob, &f.relay, "alice", b"c2-user-reply", "ur", false);
    let u = count_origination(&user_send);
    assert!(
        u.dh_boundaries >= 1,
        "a USER reply must still originate a boundary — passivation must not have suppressed \
         the human path:\n{user_send}"
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
#[test]
fn e3_measure_envelope_distinguishability() {
    let _g = lane_lock();
    let f = fixture("na0688_c2_e3");

    // ⚠ NA-0688 WARM-UP: after the A6 reversal an ack cannot establish, so bob needs a chain of
    // his own before his ack can exist at all — the earlier form measured `ack=[]`.
    warm_up_bobs_chain(&f);
    send_msg(&f.alice, &f.relay, "bob", b"e3-trigger", "e3t", true);
    let _ = drained_lens(&f, ROUTE_TOKEN_ALICE); // discard everything the warm-up put on the wire

    // STEP 1 — bob acks alice's message over his established chain. Drained immediately, so what
    // comes back is unambiguously the ack.
    recv_msg(&f.bob, &f.relay, ROUTE_TOKEN_BOB, "alice", &f.bob_out, true);
    let ack_lens = drained_lens(&f, ROUTE_TOKEN_ALICE);

    // STEP 2 — a SHORT user reply: 20 bytes, well under the Standard 1024 floor.
    send_msg(
        &f.bob,
        &f.relay,
        "alice",
        b"bobs-user-reply-body",
        "e3r",
        false,
    );
    let short_lens = drained_lens(&f, ROUTE_TOKEN_ALICE);

    // STEP 3 — a LONG user reply: 4096 bytes, unambiguously over the floor.
    let long_body = vec![b'x'; 4096];
    send_msg(&f.bob, &f.relay, "alice", &long_body, "e3l", false);
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
