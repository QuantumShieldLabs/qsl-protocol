//! NA-0623 (ENG-0012 Stage 2a): Suite-2 SCKA sender core — advertisement + PQ reseed.
//!
//! End-to-end refimpl proofs that the SCKA send side (`send_pq_advertise` / `send_pq_reseed`)
//! composes with the frozen CTXT receiver (`apply_pq_reseed` via `recv_wire`) and the classical DH
//! ratchet (`send_boundary` / `recv_dh_boundary`, NA-0621):
//!   (a) round-trip: advertise -> encapsulate -> the receiver's apply_pq_reseed decrypts and both
//!       parties converge on the directional PQ seeds and the advanced root;
//!   (b) THE HEADLINE (D560 AMENDMENT): PQ-PCS healing that SURVIVES a subsequent DH ratchet — a
//!       state snapshot taken before a PQ reseed cannot open the post-reseed DH boundary, because
//!       the reseed advanced the root the DH ratchet reads;
//!   (c) monotonicity / one-time / tombstone rejection with no state mutation (send + receive).

use quantumshield_refimpl::crypto::stdcrypto::{runtime_pq_kem_keypair, StdCrypto};
use quantumshield_refimpl::crypto::traits::{PqKem768, X25519Dh};
use quantumshield_refimpl::suite2::establish::init_from_base_handshake;
use quantumshield_refimpl::suite2::parse::decode_suite2_wire;
use quantumshield_refimpl::suite2::ratchet::{
    recv_dh_boundary, recv_pq_adv_session, recv_pq_reseed, recv_wire, send_boundary,
    send_pq_advertise, send_pq_reseed, track_peer_adv,
};
use quantumshield_refimpl::suite2::state::Suite2SessionState;

const PV: u16 = 0x0500;
const SID: u16 = 0x0002;

/// Establish a fresh authenticated Suite-2 session pair (A = initiator, B = responder). Both
/// parties share one root `RK` (the single session-level slot, equal across the pair). The
/// X25519 ratchet private keys are populated (as the real client does post-establishment) so
/// the DH ratchet can run. Returns `(A, B, R0)`.
fn establish_pair(c: &StdCrypto) -> (Suite2SessionState, Suite2SessionState, [u8; 32]) {
    let (a_priv, a_pub) = c.keypair();
    let (b_priv, b_pub) = c.keypair();
    let dh_init = c.dh(&a_priv, &b_pub); // == c.dh(&b_priv, &a_pub)
    let pq_init_ss = [0x42u8; 32];
    let session_id = [0x01u8; 16];

    let mut a = init_from_base_handshake(
        c,
        true,
        PV,
        SID,
        &session_id,
        &dh_init,
        &pq_init_ss,
        &a_pub.0,
        &b_pub.0,
        true,
    )
    .expect("establish A");
    let mut b = init_from_base_handshake(
        c,
        false,
        PV,
        SID,
        &session_id,
        &dh_init,
        &pq_init_ss,
        &b_pub.0,
        &a_pub.0,
        true,
    )
    .expect("establish B");
    a.set_dh_self_priv(a_priv.0);
    b.set_dh_self_priv(b_priv.0);

    let r0 = a.rk;
    assert_eq!(a.rk, b.rk, "both parties share the root");
    (a, b, r0)
}

fn kdf_rk_pq_expected(c: &StdCrypto, rk: &[u8; 32], pq_ss: &[u8]) -> [u8; 32] {
    use quantumshield_refimpl::crypto::traits::Kmac;
    let mut data = Vec::new();
    data.extend_from_slice(pq_ss);
    data.push(0x01);
    let out = c.kmac256(rk, "QSP5.0/RKPQ", &data, 32);
    let mut a = [0u8; 32];
    a.copy_from_slice(&out);
    a
}

/// (a) Round-trip: B advertises a receive key, A encapsulates to it and reseeds, B decrypts via the
/// frozen apply_pq_reseed path, and both parties converge on the directional PQ seed and the
/// advanced root.
#[test]
fn reseed_round_trip_sender_to_apply_pq_reseed_decrypts_and_converges() {
    let c = StdCrypto;
    let (mut a, mut b, r0) = establish_pair(&c);

    // B advertises receive key adv_id = 1 (the caller owns the ML-KEM keypair + advertised-key
    // store; here we register the id in B's known_targets as send_pq_advertise would).
    let adv_id: u32 = 1;
    let (pk_b, sk_b) = runtime_pq_kem_keypair();
    b.recv.known_targets.insert(adv_id);

    // A (initiator, live send chain) encapsulates to B's advertised public key and reseeds A->B.
    let (pq_ct, ss_a) = c.encap(&pk_b).expect("encap");
    let out = send_pq_reseed(&c, &c, &c, a.clone(), adv_id, &pq_ct, &ss_a, b"hello-pq")
        .expect("send_pq_reseed");
    a = out.state;
    let wire = out.wire;

    // B decapsulates (caller-side) and applies the reseed via recv_wire -> apply_pq_reseed.
    let ss_b = c.decap(&sk_b, &pq_ct).expect("decap");
    assert_eq!(ss_a, ss_b, "ML-KEM correctness: encap ss == decap ss");
    let rout = recv_wire(
        &c,
        &c,
        &c,
        b.recv.clone(),
        &b.rk,
        &wire,
        Some(&ss_b),
        Some(adv_id),
    )
    .expect("recv_wire reseed");
    b.recv = rout.state;
    b.rk = rout.rk;

    assert_eq!(
        rout.plaintext, b"hello-pq",
        "receiver decrypts the reseed body"
    );

    // Convergence: A's send PQ chain == B's recv PQ chain; both roots advanced to KDF_RK_PQ(R0,ss).
    assert_eq!(
        a.send.ck_pq, b.recv.ck_pq_recv,
        "directional PQ seeds converge"
    );
    let expect_r1 = kdf_rk_pq_expected(&c, &r0, &ss_a);
    assert_eq!(a.rk, expect_r1, "sender root advanced via KDF_RK_PQ");
    assert_eq!(b.rk, expect_r1, "receiver root advanced identically");
    assert_ne!(
        a.rk, r0,
        "root actually changed (PQ hardening landed in RK)"
    );
    // Header keys recomputed from the new root and converge: A's send header key (A->B) equals B's
    // recv header key (A->B), so the next post-reseed message decrypts under the new key schedule.
    assert_eq!(
        a.send.hk_s, b.recv.hk_r,
        "post-reseed directional header keys converge on the new root"
    );
}

/// (b) THE HEADLINE — PQ-PCS healing that survives a subsequent DH ratchet. A reseeds (root R0 ->
/// R1); then B performs a classical DH ratchet (root R1 -> R2) and sends under it. A state snapshot
/// captured BEFORE the reseed (root R0) CANNOT open that DH boundary, while the live receiver can —
/// proving the PQ epoch secret advanced the root the DH ratchet reads and is carried forward.
#[test]
fn pq_pcs_healing_survives_dh_ratchet() {
    let c = StdCrypto;
    let (mut a, mut b, r0) = establish_pair(&c);

    // Snapshot A BEFORE any reseed (an attacker who compromised the pre-reseed state, root R0).
    let a_pre_reseed = a.clone();

    // --- PQ reseed A -> B (before any DH ratchet, root == R0 on both) ---
    let adv_id: u32 = 1;
    let (pk_b, sk_b) = runtime_pq_kem_keypair();
    b.recv.known_targets.insert(adv_id);
    let (pq_ct, ss_a) = c.encap(&pk_b).expect("encap");
    let out = send_pq_reseed(&c, &c, &c, a.clone(), adv_id, &pq_ct, &ss_a, b"m1").expect("reseed");
    a = out.state;
    let ss_b = c.decap(&sk_b, &pq_ct).expect("decap");
    let rout = recv_wire(
        &c,
        &c,
        &c,
        b.recv.clone(),
        &b.rk,
        &out.wire,
        Some(&ss_b),
        Some(adv_id),
    )
    .expect("recv");
    b.recv = rout.state;
    // NA-0626 (ENG-0024): the advanced root returns in the outcome; there is no second slot to
    // adopt into — the classical ratchet reads the same single root the reseed advanced.
    b.rk = rout.rk;

    let r1 = kdf_rk_pq_expected(&c, &r0, &ss_a);
    assert_eq!(a.rk, r1, "A root advanced with the PQ secret");
    assert_eq!(b.rk, r1, "B root advanced with the PQ secret");
    assert_ne!(r1, r0, "reseed advanced the root (the AMENDMENT fix)");

    // --- Subsequent classical DH ratchet: B -> A boundary, sealed under NHK derived from R1 ---
    let sb = send_boundary(&c, &c, &c, &c, b.clone(), b"healed-secret").expect("send_boundary");
    let boundary_wire = sb.wire; // B's post-boundary state is not needed further in this test

    // The LIVE receiver A (root R1) opens the DH boundary and decrypts.
    let live = recv_dh_boundary(&c, &c, &c, &c, a.clone(), &boundary_wire);
    assert!(live.ok, "live A opens the post-reseed DH boundary");
    assert_eq!(live.plaintext, b"healed-secret");
    let a_after = live.state;
    assert_ne!(
        a_after.rk, r1,
        "DH ratchet advanced the root again (R1 -> R2)"
    );

    // THE HEADLINE ASSERTION: the pre-reseed snapshot (root R0) CANNOT open the boundary — the PQ
    // hardening is carried into the DH ratchet. (Without the reseed's root advance, B would have
    // sealed under NHK from R0 and this snapshot would succeed.)
    let attacker = recv_dh_boundary(&c, &c, &c, &c, a_pre_reseed, &boundary_wire);
    assert!(
        !attacker.ok,
        "a pre-reseed snapshot must NOT open the post-reseed DH boundary (PQ-PCS survived the DH ratchet)"
    );
}

/// (a') Advertisement send frames a parseable FLAG_PQ_ADV|FLAG_BOUNDARY boundary, records the id in
/// known_targets, and the peer's track step accepts it; re-advertising a non-increasing id and a
/// bad public-key length are rejected fail-closed.
#[test]
fn advertise_frames_parseable_boundary_and_track_enforces_monotonicity() {
    let c = StdCrypto;
    let (a, _b, _r0) = establish_pair(&c);

    let (pk_a, _sk_a) = runtime_pq_kem_keypair();
    let adv_id: u32 = 1;
    let out = send_pq_advertise(&c, &c, &c, a.clone(), adv_id, &pk_a, b"adv").expect("advertise");
    assert!(
        out.state.recv.known_targets.contains(&adv_id),
        "advertised id recorded in known_targets"
    );

    // Wire parses as a PQ_ADV boundary carrying the advertised id + public key.
    let (_pv, _sid, _mt, parsed) = decode_suite2_wire(&out.wire).expect("parse adv wire");
    assert_eq!(parsed.pq_adv_id, Some(adv_id));
    assert_eq!(parsed.pq_adv_pub.as_deref(), Some(pk_a.as_slice()));

    // Peer track (DOC-CAN-004 §3.2): first accept advances peer_max; non-monotonic rejects.
    let peer_max = track_peer_adv(0, adv_id, &pk_a).expect("track accepts monotonic");
    assert_eq!(peer_max, adv_id);
    assert_eq!(
        track_peer_adv(peer_max, adv_id, &pk_a),
        Err("REJECT_SCKA_ADV_NONMONOTONIC")
    );
    assert_eq!(
        track_peer_adv(0, adv_id, &pk_a[..1183]),
        Err("REJECT_SCKA_ADV_BAD_PUB_LEN")
    );

    // Re-advertising a non-increasing id fails closed with no state mutation.
    let before_known = out.state.recv.known_targets.clone();
    let readv = send_pq_advertise(&c, &c, &c, out.state.clone(), adv_id, &pk_a, b"adv");
    assert!(readv.is_err(), "re-advertising id 1 (== max known) rejects");
    assert_eq!(
        before_known, out.state.recv.known_targets,
        "no mutation on the retained state"
    );
}

/// (c) Sender-side reseed rejects (bad ciphertext length, bad shared-secret length, unset send
/// chain) are deterministic and leave the caller's retained state unmodified.
#[test]
fn reseed_sender_rejects_are_deterministic_and_no_mutation() {
    let c = StdCrypto;
    let (a, mut b, _r0) = establish_pair(&c);
    let adv_id: u32 = 1;
    let (pk_b, _sk_b) = runtime_pq_kem_keypair();
    b.recv.known_targets.insert(adv_id);
    let (pq_ct, ss_a) = c.encap(&pk_b).expect("encap");

    let retained = a.clone();

    // Bad ciphertext length.
    let e1 = send_pq_reseed(&c, &c, &c, a.clone(), adv_id, &pq_ct[..1087], &ss_a, b"x")
        .err()
        .expect("expected ct-len reject");
    assert_eq!(e1, "REJECT_SCKA_CTXT_BAD_CT_LEN");
    // Bad shared-secret length.
    let e2 = send_pq_reseed(&c, &c, &c, a.clone(), adv_id, &pq_ct, &ss_a[..31], b"x")
        .err()
        .expect("expected ss-len reject");
    assert_eq!(e2, "REJECT_SCKA_CTXT_BAD_SS_LEN");
    // Deterministic repeat.
    let e1b = send_pq_reseed(&c, &c, &c, a.clone(), adv_id, &pq_ct[..1087], &ss_a, b"x")
        .err()
        .expect("expected ct-len reject");
    assert_eq!(e1, e1b);

    // Responder B has an unset send chain until it ratchets: reseeding rejects (chain-key unset).
    let e3 = send_pq_reseed(&c, &c, &c, b.clone(), adv_id, &pq_ct, &ss_a, b"x")
        .err()
        .expect("expected chainkey-unset reject");
    assert!(e3.starts_with("REJECT_S2_CHAINKEY_UNSET"));

    // The retained sender state is byte-identical (rejects consumed a clone, never the original).
    assert_eq!(retained.send.ns, a.send.ns);
    assert_eq!(retained.rk, a.rk);
}

/// (c') Receiver one-time enforcement: a replayed reseed targeting an already-consumed advertised
/// key is rejected by the frozen apply_pq_reseed path with no state mutation.
#[test]
fn reseed_replay_is_rejected_one_time() {
    let c = StdCrypto;
    let (a, mut b, _r0) = establish_pair(&c);
    let adv_id: u32 = 1;
    let (pk_b, sk_b) = runtime_pq_kem_keypair();
    b.recv.known_targets.insert(adv_id);
    let (pq_ct, ss_a) = c.encap(&pk_b).expect("encap");
    let out = send_pq_reseed(&c, &c, &c, a, adv_id, &pq_ct, &ss_a, b"m").expect("reseed");
    let ss_b = c.decap(&sk_b, &pq_ct).expect("decap");

    let r1 = recv_wire(
        &c,
        &c,
        &c,
        b.recv.clone(),
        &b.rk,
        &out.wire,
        Some(&ss_b),
        Some(adv_id),
    )
    .expect("first recv accepts");
    b.recv = r1.state;
    b.rk = r1.rk;
    assert!(b.recv.consumed_targets.contains(&adv_id));
    assert!(b.recv.tombstoned_targets.contains(&adv_id));

    // Replay the same reseed: the target is now tombstoned/consumed -> reject, no mutation.
    let before_rk = b.rk;
    let replay = recv_wire(
        &c,
        &c,
        &c,
        b.recv.clone(),
        &b.rk,
        &out.wire,
        Some(&ss_b),
        Some(adv_id),
    );
    assert!(
        replay.is_err(),
        "replayed reseed to a consumed target rejects"
    );
    assert_eq!(
        before_rk, b.rk,
        "no state mutation on the retained receiver state"
    );
}

// ============ NA-0625 (ENG-0023): authenticated ADV receive + [ADV, reseed] one-pack ============

/// Authenticated ADV round-trip over recv_wire: A advertises, B's recv_wire routes to
/// recv_pq_adv, the MAC verifies under the shared root, BOTH receive chains consume the slot
/// (no mkskipped growth), the watermark tracks, and A's NEXT normal message arrives IN ORDER.
#[test]
fn adv_recv_round_trip_consumes_chain_and_next_message_in_order() {
    let c = StdCrypto;
    let (mut a, mut b, _r0) = establish_pair(&c);

    let adv_id: u32 = 1;
    let (pk_a, _sk_a) = runtime_pq_kem_keypair();
    let out =
        send_pq_advertise(&c, &c, &c, a.clone(), adv_id, &pk_a, b"adv-payload").expect("advertise");
    a = out.state;

    // The `peer_adv_id` parameter carries the caller-owned peer-ADV watermark on the ADV path
    // (0 here: no peer advertisement tracked yet).
    let rout = recv_wire(&c, &c, &c, b.recv.clone(), &b.rk, &out.wire, None, Some(0))
        .expect("authenticated ADV receive");
    assert_eq!(rout.plaintext, b"adv-payload", "MAC stripped, payload back");
    let nr_before = b.recv.nr;
    let peer_max_before = b.recv.peer_max_adv_id_seen;
    b.recv = rout.state;
    assert_eq!(b.recv.nr, nr_before + 1, "ADV consumed its chain slot");
    assert_eq!(
        b.recv.peer_max_adv_id_seen, peer_max_before,
        "the frozen CTXT-path watermark field is untouched by an ADV"
    );
    assert!(
        b.recv.mkskipped.is_empty(),
        "no receive-chain gap from the ADV"
    );

    // A's next NORMAL message decrypts strictly in order (n == nr) — the Decision-2 retirement:
    // no skipped control slot, no mkskipped entry.
    let m = quantumshield_refimpl::suite2::ratchet::send_wire(
        &c,
        &c,
        &c,
        a.send.clone(),
        0,
        b"after-adv",
    )
    .expect("send_wire");
    a.send = m.state;
    let r2 =
        recv_wire(&c, &c, &c, b.recv.clone(), &b.rk, &m.wire, None, None).expect("in-order recv");
    assert_eq!(r2.plaintext, b"after-adv");
    b.recv = r2.state;
    assert!(
        b.recv.mkskipped.is_empty(),
        "mkskipped stays empty in-order"
    );
}

/// NA-0626 (ENG-0030 STRUCTURAL — the NA-0625 regression INVERTED): after a party RECEIVES a
/// reseed through the session-level entry point (`recv_pq_reseed`), its SEND-side key schedule
/// is refreshed from the advanced root BY CONSTRUCTION. The NA-0625 shape of this test asserted
/// the staleness (`assert_ne!`) and then performed the caller-side composition qsc's CTXT arm
/// carried (root INJECT/ADOPT + `send.hk_s`/`send.ck_pq` refresh); the entry point performs all
/// of it atomically with the receive, so no caller can hold half a key schedule.
#[test]
fn reseed_receiver_send_schedule_must_be_refreshed_from_advanced_root() {
    let c = StdCrypto;
    let (mut a, mut b, _r0) = establish_pair(&c);

    // B (the responder) has no send chain until it ratchets: give it one with a DH boundary,
    // exactly as the reply-driven trigger does on the real client.
    let sb = send_boundary(&c, &c, &c, &c, b.clone(), b"reply").expect("B ratchets");
    b = sb.state;
    let ra = recv_dh_boundary(&c, &c, &c, &c, a.clone(), &sb.wire);
    assert!(ra.ok, "A opens B's boundary");
    a = ra.state;

    // A reseeds to B's advertised receive key (the caller owns the ML-KEM store).
    let adv_id: u32 = 1;
    let (pk_b, sk_b) = runtime_pq_kem_keypair();
    b.recv.known_targets.insert(adv_id);
    let (pq_ct, ss_a) = c.encap(&pk_b).expect("encap");
    let out = send_pq_reseed(&c, &c, &c, a.clone(), adv_id, &pq_ct, &ss_a, b"m").expect("reseed");
    a = out.state;

    // B receives it through the SESSION-LEVEL entry point: no INJECT, no ADOPT, no caller-side
    // send-half refresh — the entry point owns the whole schedule.
    let ss_b = c.decap(&sk_b, &pq_ct).expect("decap");
    let rout = recv_pq_reseed(&c, &c, &c, &c, b.clone(), &out.wire, &ss_b, adv_id);
    assert!(rout.ok, "session-level reseed receive accepts");
    assert_eq!(rout.plaintext, b"m");
    b = rout.state;

    // INVERTED ASSERTIONS: the returned state is coherent on the advanced root — both header-key
    // directions and both directional PQ chains, with no caller composition performed.
    assert_eq!(a.send.hk_s, b.recv.hk_r, "A->B header keys converge");
    assert_eq!(
        b.send.hk_s, a.recv.hk_r,
        "the receiver's send header key is refreshed from the advanced root (the ENG-0030 fix)"
    );
    assert_eq!(
        b.send.ck_pq, a.recv.ck_pq_recv,
        "the receiver's send PQ chain is refreshed from the advanced root (the ENG-0030 fix)"
    );
    assert_eq!(
        b.rk, a.rk,
        "the single session root converges (no ADOPT left to the caller — the slot is gone)"
    );

    // Concretely: B's advertisement (a control pre-envelope on the current send chain)
    // authenticates at A's authenticated ADV receiver — header AND body — built on the schedule
    // the entry point returned, exactly as the peer expects.
    let (pk_b2, _sk_b2) = runtime_pq_kem_keypair();
    let adv = send_pq_advertise(&c, &c, &c, b.clone(), 2, &pk_b2, b"post-reseed-adv")
        .expect("B advertises");
    let r = recv_wire(&c, &c, &c, a.recv.clone(), &a.rk, &adv.wire, None, Some(0))
        .expect("A authenticates B's post-reseed advertisement");
    assert_eq!(r.plaintext, b"post-reseed-adv");
}

/// NA-0626 (ENG-0024 companion): the session-level ADV receive mirrors the wire-level one —
/// authenticated round trip, chain slot consumed, root untouched, next message in order — with
/// the root injection internal and the FULL session state returned. Rejects leave the input
/// state unmodified (planted ADV with a wrong-root MAC).
#[test]
fn recv_pq_adv_session_round_trip_and_reject_no_mutation() {
    let c = StdCrypto;
    let (mut a, mut b, _r0) = establish_pair(&c);

    let adv_id: u32 = 1;
    let (pk_a, _sk_a) = runtime_pq_kem_keypair();
    let out =
        send_pq_advertise(&c, &c, &c, a.clone(), adv_id, &pk_a, b"adv-payload").expect("advertise");
    a = out.state;

    let rout = recv_pq_adv_session(&c, &c, &c, b.clone(), &out.wire, 0);
    assert!(rout.ok, "session-level ADV receive accepts");
    assert_eq!(rout.plaintext, b"adv-payload", "MAC stripped, payload back");
    let nr_before = b.recv.nr;
    let root_before = b.rk;
    b = rout.state;
    assert_eq!(b.recv.nr, nr_before + 1, "ADV consumed its chain slot");
    assert_eq!(b.rk, root_before, "an ADV advances no root");
    assert!(b.recv.mkskipped.is_empty(), "no receive-chain gap");

    // A's next normal message arrives strictly in order.
    let m = quantumshield_refimpl::suite2::ratchet::send_wire(&c, &c, &c, a.send.clone(), 0, b"x")
        .expect("send_wire");
    a.send = m.state;
    let r2 =
        recv_wire(&c, &c, &c, b.recv.clone(), &b.rk, &m.wire, None, None).expect("in-order recv");
    assert_eq!(r2.plaintext, b"x");

    // A replayed ADV rejects (stale counter under the session keys) with no state mutation.
    let before_bytes = b.snapshot_bytes();
    let replay = recv_pq_adv_session(&c, &c, &c, b.clone(), &out.wire, adv_id);
    assert!(!replay.ok, "replayed ADV rejects");
    assert_eq!(
        replay.state.snapshot_bytes(),
        before_bytes,
        "reject returns the input state unmodified"
    );
}

/// THE DECISION-2 PROOF at refimpl level: an [ADV, reseed] pair from the same sender round-trips
/// in one delivery sequence — the ADV consumes its slot so the strict in-order reseed receiver
/// sees n == nr and applies the reseed; both parties converge on the advanced root.
#[test]
fn adv_then_reseed_same_pack_round_trips() {
    let c = StdCrypto;
    let (mut a, mut b, r0) = establish_pair(&c);

    // B advertises a receive key out-of-band-style (registers its own target id, as
    // send_pq_advertise would); A holds B's advertised public key.
    let b_adv_id: u32 = 1;
    let (pk_b, sk_b) = runtime_pq_kem_keypair();
    b.recv.known_targets.insert(b_adv_id);

    // A packs [its own ADV, then a reseed to B's key] — the NA-0624 exclusion rule retired.
    let a_adv_id: u32 = 1;
    let (pk_a, _sk_a) = runtime_pq_kem_keypair();
    let adv_out =
        send_pq_advertise(&c, &c, &c, a.clone(), a_adv_id, &pk_a, b"").expect("A advertises");
    a = adv_out.state;
    let (pq_ct, ss_a) = c.encap(&pk_b).expect("encap to B");
    let reseed_out = send_pq_reseed(&c, &c, &c, a.clone(), b_adv_id, &pq_ct, &ss_a, b"reseed-pt")
        .expect("A reseeds in the same pack");
    a = reseed_out.state;

    // B receives IN PACK ORDER: ADV first (consumes slot n), reseed second (n+1 == nr).
    let r_adv = recv_wire(
        &c,
        &c,
        &c,
        b.recv.clone(),
        &b.rk,
        &adv_out.wire,
        None,
        Some(0),
    )
    .expect("ADV accepted");
    b.recv = r_adv.state;
    assert!(b.recv.mkskipped.is_empty(), "ADV left no gap");

    let ss_b = c.decap(&sk_b, &pq_ct).expect("decap");
    let r_reseed = recv_wire(
        &c,
        &c,
        &c,
        b.recv.clone(),
        &b.rk,
        &reseed_out.wire,
        Some(&ss_b),
        Some(b_adv_id),
    )
    .expect("reseed accepted immediately after the ADV in the same pack");
    assert_eq!(r_reseed.plaintext, b"reseed-pt");
    b.recv = r_reseed.state;
    b.rk = r_reseed.rk;

    // Convergence on the advanced root. (The peer-ADV watermark is caller-owned; here the ADV
    // authenticated against watermark 0 — the caller would persist a_adv_id as the new mark.)
    let r1 = kdf_rk_pq_expected(&c, &r0, &ss_a);
    assert_eq!(a.rk, r1, "sender root advanced");
    assert_eq!(b.rk, r1, "receiver root advanced identically");
    assert!(
        b.recv.mkskipped.is_empty(),
        "no mkskipped growth across [ADV, reseed]"
    );
}
