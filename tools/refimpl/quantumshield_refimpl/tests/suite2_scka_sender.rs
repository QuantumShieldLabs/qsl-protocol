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
    recv_dh_boundary, recv_wire, send_boundary, send_pq_advertise, send_pq_reseed, track_peer_adv,
};
use quantumshield_refimpl::suite2::state::Suite2SessionState;

const PV: u16 = 0x0500;
const SID: u16 = 0x0002;

/// Establish a fresh authenticated Suite-2 session pair (A = initiator, B = responder). Both
/// parties share one root `RK` (`recv.rk == dh.rk` on each, equal across the pair). The X25519
/// ratchet private keys are populated (as the real client does post-establishment) so the DH
/// ratchet can run. Returns `(A, B, R0)`.
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

    let r0 = a.recv.rk;
    assert_eq!(a.recv.rk, b.recv.rk, "both parties share the root");
    assert_eq!(a.recv.rk, a.dh.rk, "A: recv.rk == dh.rk at establishment");
    assert_eq!(b.recv.rk, b.dh.rk, "B: recv.rk == dh.rk at establishment");
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
    let rout = recv_wire(&c, &c, &c, b.recv.clone(), &wire, Some(&ss_b), Some(adv_id))
        .expect("recv_wire reseed");
    b.recv = rout.state;

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
    assert_eq!(a.recv.rk, expect_r1, "sender root advanced via KDF_RK_PQ");
    assert_eq!(a.dh.rk, expect_r1, "sender DH-root slot advanced too");
    assert_eq!(b.recv.rk, expect_r1, "receiver root advanced identically");
    assert_ne!(
        a.recv.rk, r0,
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

    // --- PQ reseed A -> B (before any DH ratchet, so recv.rk == dh.rk == R0 on both) ---
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
        &out.wire,
        Some(&ss_b),
        Some(adv_id),
    )
    .expect("recv");
    b.recv = rout.state;
    // Caller-side composition (Stage 2a): adopt the advanced root into the DH-ratchet slot so the
    // classical ratchet carries the PQ hardening forward (Stage 2b performs this in qsc session
    // state). A already synced its own dh.rk inside send_pq_reseed.
    b.dh.rk = b.recv.rk;

    let r1 = kdf_rk_pq_expected(&c, &r0, &ss_a);
    assert_eq!(a.dh.rk, r1, "A DH-root advanced with the PQ secret");
    assert_eq!(b.dh.rk, r1, "B DH-root advanced with the PQ secret");
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
        a_after.dh.rk, r1,
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
    assert_eq!(retained.recv.rk, a.recv.rk);
    assert_eq!(retained.dh.rk, a.dh.rk);
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
        &out.wire,
        Some(&ss_b),
        Some(adv_id),
    )
    .expect("first recv accepts");
    b.recv = r1.state;
    assert!(b.recv.consumed_targets.contains(&adv_id));
    assert!(b.recv.tombstoned_targets.contains(&adv_id));

    // Replay the same reseed: the target is now tombstoned/consumed -> reject, no mutation.
    let before = b.recv.clone();
    let replay = recv_wire(
        &c,
        &c,
        &c,
        b.recv.clone(),
        &out.wire,
        Some(&ss_b),
        Some(adv_id),
    );
    assert!(
        replay.is_err(),
        "replayed reseed to a consumed target rejects"
    );
    assert_eq!(
        before.rk, b.recv.rk,
        "no state mutation on the retained receiver state"
    );
}
