//! NA-0626 (ENG-0026): combined DH+PQ boundary — sender + receiver.
//!
//! End-to-end refimpl proofs for the single-message hybrid ratchet boundary
//! (`FLAG_BOUNDARY | FLAG_PQ_CTXT` with a FRESH `DH_pub`):
//!   (a) round-trip: `send_combined_boundary` -> `recv_pq_reseed` decrypts, both parties converge
//!       on the design-locked DH-first-then-PQ root composition
//!       (`RK_final = KDF_RK_PQ(KDF_RK_DH(RK_pre, dh_out), ss)`) and a coherent full-directional
//!       key schedule, and the next message flows strictly in order;
//!   (b) healing: a pre-combined state snapshot cannot open a post-combined DH boundary (the
//!       combined boundary's PQ secret lands in the root lineage the DH ratchet reads);
//!   (c) fail-closed receiver rules: n != 0 => NOT_IN_ORDER; zero `DH_pub` => HDR_AUTH_FAIL;
//!       fresh `DH_pub` without local DH capability => LOCAL_UNSUPPORTED; replay (now-stale
//!       `DH_pub`) rejects; every reject returns the input state unmodified;
//!   (d) sender rejects are deterministic with no state mutation; ADV|CTXT (0x0007) stays
//!       unsupported at the session entry point.

use quantumshield_refimpl::crypto::stdcrypto::{runtime_pq_kem_keypair, StdCrypto};
use quantumshield_refimpl::crypto::traits::{Aead, Kmac, PqKem768, X25519Dh};
use quantumshield_refimpl::suite2::establish::init_from_base_handshake;
use quantumshield_refimpl::suite2::ratchet::{
    header_key, nonce_hdr, recv_dh_boundary, recv_pq_reseed, send_combined_boundary, send_wire,
};
use quantumshield_refimpl::suite2::state::Suite2SessionState;
use quantumshield_refimpl::suite2::{binding, types};

const PV: u16 = 0x0500;
const SID: u16 = 0x0002;

/// Establish a fresh authenticated Suite-2 session pair (A = initiator, B = responder), X25519
/// ratchet private keys populated (as the real client does post-establishment). Returns
/// `(A, B, R0)`.
fn establish_pair(c: &StdCrypto) -> (Suite2SessionState, Suite2SessionState, [u8; 32]) {
    let (a_priv, a_pub) = c.keypair();
    let (b_priv, b_pub) = c.keypair();
    let dh_init = c.dh(&a_priv, &b_pub);
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

fn kmac32_expected(c: &StdCrypto, key: &[u8; 32], label: &str, data: &[u8]) -> [u8; 32] {
    let out = c.kmac256(key, label, data, 32);
    let mut a = [0u8; 32];
    a.copy_from_slice(&out);
    a
}

/// The design-locked combined composition, computed independently: DH first, then PQ.
fn expected_combined_roots(
    c: &StdCrypto,
    rk_pre: &[u8; 32],
    dh_out: &[u8; 32],
    pq_ss: &[u8],
) -> ([u8; 32], [u8; 32]) {
    let out = c.kmac256(rk_pre, "QSP5.0/RKDH", dh_out, 64);
    let mut rk_dh = [0u8; 32];
    rk_dh.copy_from_slice(&out[0..32]);
    let mut data = Vec::new();
    data.extend_from_slice(pq_ss);
    data.push(0x01);
    let rk_final = kmac32_expected(c, &rk_dh, "QSP5.0/RKPQ", &data);
    (rk_dh, rk_final)
}

/// Frame a Suite-2 wire envelope around a hand-built ratchet header (test-only mirror of the
/// sender's framing, for constructing tamper/ordering shapes the pure sender cannot emit).
fn frame_wire(
    dh_pub: &[u8; 32],
    flags: u16,
    pq_prefix: &[u8],
    hdr_ct: &[u8],
    body_ct: &[u8],
) -> Vec<u8> {
    let mut header = Vec::new();
    header.extend_from_slice(dh_pub);
    header.extend_from_slice(&flags.to_be_bytes());
    header.extend_from_slice(pq_prefix);
    header.extend_from_slice(hdr_ct);
    let mut wire = Vec::new();
    wire.extend_from_slice(&PV.to_be_bytes());
    wire.extend_from_slice(&SID.to_be_bytes());
    wire.push(0x02);
    wire.push(0x00);
    wire.extend_from_slice(&(header.len() as u16).to_be_bytes());
    wire.extend_from_slice(&(body_ct.len() as u16).to_be_bytes());
    wire.extend_from_slice(&header);
    wire.extend_from_slice(body_ct);
    wire
}

/// Build a combined-shaped frame whose NHK-sealed header claims message index `n_claim` (sealed
/// at the receiver's expected n=0 nonce so the header OPENS, exposing the ordering violation).
fn build_combined_frame_with_n(
    c: &StdCrypto,
    session_id: &[u8; 16],
    rk_pre: &[u8; 32],
    sender_a2b: bool,
    fresh_dh_pub: &[u8; 32],
    n_claim: u32,
) -> Vec<u8> {
    let flags = types::FLAG_BOUNDARY | types::FLAG_PQ_CTXT;
    let mut pq_prefix = Vec::with_capacity(4 + 1088);
    pq_prefix.extend_from_slice(&1u32.to_be_bytes());
    pq_prefix.extend_from_slice(&[0x5au8; 1088]);
    let pq_bind = binding::pq_bind_sha512_32(c, flags, &pq_prefix);
    let ad_hdr = binding::ad_hdr(session_id, PV, SID, fresh_dh_pub, flags, &pq_bind);
    let nhk_s = header_key(c, rk_pre, sender_a2b, true).expect("nhk");
    let mut hdr_pt = Vec::with_capacity(8);
    hdr_pt.extend_from_slice(&0u32.to_be_bytes());
    hdr_pt.extend_from_slice(&n_claim.to_be_bytes());
    let hdr_ct = c.seal(
        &nhk_s,
        &nonce_hdr(c, session_id, fresh_dh_pub, 0),
        &ad_hdr,
        &hdr_pt,
    );
    frame_wire(fresh_dh_pub, flags, &pq_prefix, &hdr_ct, &[0xaau8; 16])
}

/// (a) Round-trip + convergence on the DH-first-then-PQ composition + in-order continuation.
#[test]
fn combined_boundary_round_trip_converges_on_dh_then_pq_composition() {
    let c = StdCrypto;
    let (mut a, mut b, r0) = establish_pair(&c);

    // B advertises receive key adv_id = 1 (caller-owned ML-KEM store, as the SCKA tests do).
    let adv_id: u32 = 1;
    let (pk_b, sk_b) = runtime_pq_kem_keypair();
    b.recv.known_targets.insert(adv_id);

    // A sends ONE message that both ratchets DH and reseeds PQ (caller-supplied fresh keypair).
    let (new_priv, new_pub) = c.keypair();
    let (pq_ct, ss_a) = c.encap(&pk_b).expect("encap");
    let out = send_combined_boundary(
        &c,
        &c,
        &c,
        &c,
        a.clone(),
        &new_priv.0,
        &new_pub.0,
        adv_id,
        &pq_ct,
        &ss_a,
        b"hybrid-boundary",
    )
    .expect("send_combined_boundary");
    let dh_out = c.dh(
        &new_priv,
        &quantumshield_refimpl::crypto::traits::X25519Pub(a.dh.dhr),
    );
    a = out.state;

    // B receives it through the session-level entry point.
    let ss_b = c.decap(&sk_b, &pq_ct).expect("decap");
    assert_eq!(ss_a, ss_b, "ML-KEM correctness");
    let rout = recv_pq_reseed(&c, &c, &c, &c, b.clone(), &out.wire, &ss_b, adv_id);
    assert!(rout.ok, "combined boundary accepted: {:?}", rout.reason);
    assert_eq!(rout.plaintext, b"hybrid-boundary");
    assert_eq!(
        rout.n,
        Some(0),
        "the combined frame is n=0 of the new epoch"
    );
    b = rout.state;

    // Root convergence on the independently computed DH-first-then-PQ composition.
    let (rk_dh, rk_final) = expected_combined_roots(&c, &r0, &dh_out, &ss_a);
    assert_ne!(rk_dh, r0);
    assert_ne!(
        rk_final, rk_dh,
        "PQ secret advanced the root after the DH step"
    );
    assert_eq!(
        a.rk, rk_final,
        "sender root == KDF_RK_PQ(KDF_RK_DH(R0, dh_out), ss)"
    );
    assert_eq!(b.rk, rk_final, "receiver root converges");

    // Full-directional schedule coherence (the ENG-0030 guarantee extends to the combined path).
    assert_eq!(a.send.hk_s, b.recv.hk_r, "A->B header keys converge");
    assert_eq!(b.send.hk_s, a.recv.hk_r, "B->A header keys converge");
    assert_eq!(a.send.ck_pq, b.recv.ck_pq_recv, "A->B PQ chains converge");
    assert_eq!(b.send.ck_pq, a.recv.ck_pq_recv, "B->A PQ chains converge");
    assert_eq!(b.dh.dhr, a.dh.dhs_pub, "receiver adopted the fresh DH key");
    assert_eq!(
        b.recv.nr, 1,
        "combined boundary consumed n=0 of the new epoch"
    );
    assert_eq!(a.send.ns, 1, "sender counters restarted for the new epoch");
    assert!(b.recv.mkskipped.is_empty(), "no receive-chain gap");
    assert!(
        b.recv.consumed_targets.contains(&adv_id) && b.recv.tombstoned_targets.contains(&adv_id),
        "the targeted advertised key is consumed one-time"
    );

    // A's next NORMAL message (n=1 of the new epoch) arrives strictly in order.
    let m = send_wire(&c, &c, &c, a.send.clone(), 0, b"after-combined").expect("send_wire");
    a.send = m.state;
    let r2 = quantumshield_refimpl::suite2::ratchet::recv_wire(
        &c,
        &c,
        &c,
        b.recv.clone(),
        &b.rk,
        &m.wire,
        None,
        None,
    )
    .expect("in-order recv after combined");
    assert_eq!(r2.plaintext, b"after-combined");
    assert!(
        r2.state.mkskipped.is_empty(),
        "mkskipped stays empty in-order"
    );
}

/// (b) Healing: a pre-combined snapshot (root R0) cannot open a post-combined DH boundary —
/// the combined boundary's PQ secret is carried in the root lineage the DH ratchet reads.
#[test]
fn combined_boundary_healing_survives_subsequent_dh_ratchet() {
    let c = StdCrypto;
    let (a, mut b, _r0) = establish_pair(&c);

    // Snapshot A before the combined boundary (an attacker who compromised pre-combined state).
    let a_pre_combined = a.clone();

    let adv_id: u32 = 1;
    let (pk_b, sk_b) = runtime_pq_kem_keypair();
    b.recv.known_targets.insert(adv_id);
    let (new_priv, new_pub) = c.keypair();
    let (pq_ct, ss_a) = c.encap(&pk_b).expect("encap");
    let out = send_combined_boundary(
        &c,
        &c,
        &c,
        &c,
        a.clone(),
        &new_priv.0,
        &new_pub.0,
        adv_id,
        &pq_ct,
        &ss_a,
        b"m",
    )
    .expect("combined send");
    let a_live = out.state;
    let ss_b = c.decap(&sk_b, &pq_ct).expect("decap");
    let rout = recv_pq_reseed(&c, &c, &c, &c, b.clone(), &out.wire, &ss_b, adv_id);
    assert!(rout.ok);
    b = rout.state;

    // B performs a subsequent classical DH ratchet, sealed under NHK derived from the combined
    // root lineage.
    let sb = quantumshield_refimpl::suite2::ratchet::send_boundary(&c, &c, &c, &c, b, b"healed")
        .expect("B ratchets after the combined boundary");

    // The live A opens it; the pre-combined snapshot cannot.
    let live = recv_dh_boundary(&c, &c, &c, &c, a_live, &sb.wire);
    assert!(live.ok, "live A opens the post-combined DH boundary");
    assert_eq!(live.plaintext, b"healed");
    let attacker = recv_dh_boundary(&c, &c, &c, &c, a_pre_combined, &sb.wire);
    assert!(
        !attacker.ok,
        "a pre-combined snapshot must NOT open the post-combined DH boundary (hybrid healing)"
    );
}

/// (c) n != 0 => NOT_IN_ORDER, no mutation (the combined frame is n=0 of the new epoch ONLY).
#[test]
fn combined_boundary_rejects_out_of_order_n_without_mutation() {
    let c = StdCrypto;
    let (a, b, _r0) = establish_pair(&c);

    let (_fresh_priv, fresh_pub) = c.keypair();
    let wire = build_combined_frame_with_n(
        &c,
        &b.recv.session_id,
        &a.rk, // == B's root; the sender's NHK_s derives from the shared pre-boundary root
        true,  // A->B
        &fresh_pub.0,
        1,
    );
    let before = b.snapshot_bytes();
    let rout = recv_pq_reseed(&c, &c, &c, &c, b, &wire, &[0x33u8; 32], 1);
    assert!(!rout.ok);
    assert!(
        rout.reason
            .unwrap_or("")
            .starts_with("REJECT_S2_BOUNDARY_NOT_IN_ORDER"),
        "unexpected reason: {:?}",
        rout.reason
    );
    assert_eq!(rout.state.snapshot_bytes(), before, "no mutation on reject");
}

/// (c) Zero `DH_pub` => HDR_AUTH_FAIL; fresh `DH_pub` without local DH capability =>
/// LOCAL_UNSUPPORTED; ADV|CTXT (0x0007) => LOCAL_UNSUPPORTED — all with no mutation.
#[test]
fn combined_boundary_anti_spoof_rejects_without_mutation() {
    let c = StdCrypto;
    let (a, b, _r0) = establish_pair(&c);

    // Zero DH_pub (a boundary must carry a real key). The zero key differs from dhr, so it
    // routes to the combined arm and dies at the zero check.
    let wire_zero = build_combined_frame_with_n(&c, &b.recv.session_id, &a.rk, true, &[0u8; 32], 0);
    let before = b.snapshot_bytes();
    let r1 = recv_pq_reseed(&c, &c, &c, &c, b.clone(), &wire_zero, &[0x33u8; 32], 1);
    assert!(!r1.ok);
    assert_eq!(r1.reason, Some("REJECT_S2_HDR_AUTH_FAIL"));
    assert_eq!(r1.state.snapshot_bytes(), before);

    // Fresh DH_pub against a session with no local DH capability (dhs_priv zero — the actor
    // plumbing-session shape): LOCAL_UNSUPPORTED before any crypto.
    let (_p, fresh_pub) = c.keypair();
    let wire_fresh =
        build_combined_frame_with_n(&c, &b.recv.session_id, &a.rk, true, &fresh_pub.0, 0);
    let mut b_nodh = b.clone();
    b_nodh.dh.dhs_priv = [0u8; 32];
    let before_nodh = b_nodh.snapshot_bytes();
    let r2 = recv_pq_reseed(&c, &c, &c, &c, b_nodh, &wire_fresh, &[0x33u8; 32], 1);
    assert!(!r2.ok);
    assert_eq!(r2.reason, Some("REJECT_S2_LOCAL_UNSUPPORTED"));
    assert_eq!(r2.state.snapshot_bytes(), before_nodh);

    // ADV|CTXT|BOUNDARY (0x0007) stays unsupported at the session entry point (parse accepts
    // the both-prefix shape; the receiver refuses it).
    let flags7 = types::FLAG_PQ_ADV | types::FLAG_PQ_CTXT | types::FLAG_BOUNDARY;
    let mut pq_prefix = Vec::new();
    pq_prefix.extend_from_slice(&1u32.to_be_bytes());
    pq_prefix.extend_from_slice(&[0x11u8; 1184]); // ADV prefix half
    pq_prefix.extend_from_slice(&2u32.to_be_bytes());
    pq_prefix.extend_from_slice(&[0x22u8; 1088]); // CTXT prefix half
    let wire7 = frame_wire(
        &fresh_pub.0,
        flags7,
        &pq_prefix,
        &[0x44u8; 24],
        &[0x55u8; 16],
    );
    let r3 = recv_pq_reseed(&c, &c, &c, &c, b.clone(), &wire7, &[0x33u8; 32], 1);
    assert!(!r3.ok);
    assert_eq!(r3.reason, Some("REJECT_S2_LOCAL_UNSUPPORTED"));
    assert_eq!(r3.state.snapshot_bytes(), before);
}

/// (c) Replay: after the combined boundary is accepted, the same wire re-presented carries a
/// now-CURRENT `DH_pub` (PQ-only discrimination) sealed under the OLD NHK at a stale counter —
/// it rejects, with no mutation.
#[test]
fn combined_boundary_replay_rejects_without_mutation() {
    let c = StdCrypto;
    let (a, mut b, _r0) = establish_pair(&c);

    let adv_id: u32 = 1;
    let (pk_b, sk_b) = runtime_pq_kem_keypair();
    b.recv.known_targets.insert(adv_id);
    let (new_priv, new_pub) = c.keypair();
    let (pq_ct, ss_a) = c.encap(&pk_b).expect("encap");
    let out = send_combined_boundary(
        &c,
        &c,
        &c,
        &c,
        a,
        &new_priv.0,
        &new_pub.0,
        adv_id,
        &pq_ct,
        &ss_a,
        b"m",
    )
    .expect("combined send");
    let ss_b = c.decap(&sk_b, &pq_ct).expect("decap");
    let first = recv_pq_reseed(&c, &c, &c, &c, b.clone(), &out.wire, &ss_b, adv_id);
    assert!(first.ok, "first delivery accepted");
    b = first.state;

    let before = b.snapshot_bytes();
    let replay = recv_pq_reseed(&c, &c, &c, &c, b.clone(), &out.wire, &ss_b, adv_id);
    assert!(!replay.ok, "replayed combined boundary rejects");
    assert_eq!(
        replay.state.snapshot_bytes(),
        before,
        "no mutation on replay"
    );
}

/// (d) Sender rejects: bad ciphertext/shared-secret lengths, zero caller keypair, and a missing
/// peer DH key are deterministic fail-closed rejects; the retained state is unmodified.
#[test]
fn combined_sender_rejects_are_deterministic_and_no_mutation() {
    let c = StdCrypto;
    let (a, mut b, _r0) = establish_pair(&c);
    let adv_id: u32 = 1;
    let (pk_b, _sk_b) = runtime_pq_kem_keypair();
    b.recv.known_targets.insert(adv_id);
    let (pq_ct, ss_a) = c.encap(&pk_b).expect("encap");
    let (new_priv, new_pub) = c.keypair();
    let retained = a.snapshot_bytes();

    let e1 = send_combined_boundary(
        &c,
        &c,
        &c,
        &c,
        a.clone(),
        &new_priv.0,
        &new_pub.0,
        adv_id,
        &pq_ct[..1087],
        &ss_a,
        b"x",
    )
    .err()
    .expect("ct-len reject");
    assert_eq!(e1, "REJECT_SCKA_CTXT_BAD_CT_LEN");

    let e2 = send_combined_boundary(
        &c,
        &c,
        &c,
        &c,
        a.clone(),
        &new_priv.0,
        &new_pub.0,
        adv_id,
        &pq_ct,
        &ss_a[..31],
        b"x",
    )
    .err()
    .expect("ss-len reject");
    assert_eq!(e2, "REJECT_SCKA_CTXT_BAD_SS_LEN");

    let e3 = send_combined_boundary(
        &c,
        &c,
        &c,
        &c,
        a.clone(),
        &[0u8; 32],
        &new_pub.0,
        adv_id,
        &pq_ct,
        &ss_a,
        b"x",
    )
    .err()
    .expect("zero-keypair reject");
    assert_eq!(e3, "REJECT_S2_LOCAL_UNSUPPORTED");

    let mut a_nodhr = a.clone();
    a_nodhr.dh.dhr = [0u8; 32];
    let e4 = send_combined_boundary(
        &c,
        &c,
        &c,
        &c,
        a_nodhr,
        &new_priv.0,
        &new_pub.0,
        adv_id,
        &pq_ct,
        &ss_a,
        b"x",
    )
    .err()
    .expect("no-peer-key reject");
    assert_eq!(e4, "REJECT_S2_LOCAL_UNSUPPORTED");

    // Deterministic repeat + the retained sender state is byte-identical.
    let e1b = send_combined_boundary(
        &c,
        &c,
        &c,
        &c,
        a.clone(),
        &new_priv.0,
        &new_pub.0,
        adv_id,
        &pq_ct[..1087],
        &ss_a,
        b"x",
    )
    .err()
    .expect("ct-len reject repeat");
    assert_eq!(e1, e1b);
    assert_eq!(
        retained,
        a.snapshot_bytes(),
        "rejects consumed clones, never the original"
    );
}
