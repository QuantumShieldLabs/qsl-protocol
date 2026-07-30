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
// WHAT IS BEING GUARDED (R1a as amended by ruling A6):
//   A control send never originates a ROTATION — no reply boundary, no N/T fallback, no PQ
//   reseed, no advertisement — BUT it establishes its own sending chain if it has none.
//   Establishment is a NECESSITY, not a rotation opportunity: a send with no chain has
//   nothing to send on. ENG-0086 finding 1 makes that the COMMON case, not an edge one —
//   "the recipient's automatic ack becomes their first send".

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

/// Drive bob to an ESTABLISHED chain, then have him ack again. Returns (ack output, fixture).
fn established_chain_ack(tag: &str) -> (String, Fixture) {
    let f = fixture(tag);
    // First receive: establishes bob's chain (and acks).
    recv_msg(&f.bob, &f.relay, ROUTE_TOKEN_BOB, "alice", &f.bob_out, true);
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

/// GUARD — the ESTABLISHMENT exception (ruling A6 item 3), asserted in BOTH directions:
/// an ack on an unseeded chain DOES establish, and establishes ONLY.
#[test]
fn an_ack_on_an_unseeded_chain_establishes_and_only_establishes() {
    let _g = lane_lock();
    let f = fixture("na0688_c2_g3");
    let out = recv_msg(&f.bob, &f.relay, ROUTE_TOKEN_BOB, "alice", &f.bob_out, true);
    assert!(
        out.contains("event=receipt_send"),
        "the fixture must actually ack:\n{out}"
    );
    let o = count_origination(&out);
    assert_eq!(
        o.dh_first_send, 1,
        "an ack on an UNSEEDED chain must establish it — suppressing this would leave the \
         sender with no chain to send on at all:\n{out}"
    );
    assert_eq!(
        o.dh_reply, 0,
        "establishment must be reported as `first_send`, NEVER as `reply`. Before C2 this \
         reported `reply`, because a pending reply flag won the label over the actual cause:\n{out}"
    );
    assert_eq!(
        o.dh_fallback, 0,
        "establishment is not a fallback rotation:\n{out}"
    );
    assert_eq!(
        o.pq_reseeds, 0,
        "establishment must not drag a PQ reseed with it:\n{out}"
    );
    assert_eq!(
        o.advertisements, 0,
        "establishment must not mint an advertisement:\n{out}"
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

    // Establish bob's chain and drain the first message.
    recv_msg(&f.bob, &f.relay, ROUTE_TOKEN_BOB, "alice", &f.bob_out, true);

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
#[test]
fn e3_measure_envelope_distinguishability() {
    let _g = lane_lock();
    let f = fixture("na0688_c2_e3");

    // Bob acks alice's message (establishing his chain), then sends a user reply.
    recv_msg(&f.bob, &f.relay, ROUTE_TOKEN_BOB, "alice", &f.bob_out, true);
    send_msg(
        &f.bob,
        &f.relay,
        "alice",
        b"bobs-user-reply-body",
        "e3r",
        false,
    );

    // Raw bytes as the relay holds them — an observer's view.
    let stored = f.server.drain_channel(ROUTE_TOKEN_ALICE);
    let lens: Vec<usize> = stored.iter().map(|e| e.len()).collect();

    println!("=== E3 MEASUREMENT — envelope lengths as they sat on the relay ===");
    println!("bob -> alice envelopes: {lens:?}");
    println!("=== END E3 ===");

    // ⚠ A single sample cannot answer R2b. Refuse a comparison that was never made.
    assert!(
        lens.len() >= 2,
        "E3 needs at least bob's ack AND his user reply to compare; got {lens:?}"
    );
}
