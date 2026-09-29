// NA-0785 PLAN F03 / S6a -- FF4: HANDSHAKE VECTORS OF THE IMPLEMENTED DEVELOPMENT PROFILE.
//
// ⚠ PROFILE LABEL. Every vector in this file pins the integration head's IMPLEMENTED development
// profile, `profile::ACTIVE` (today `&IMPLEMENTED`, id NA0780-DIR-INTEGRATION-03, carried in the
// QHSM version-2 parameter block as the critical 0x7f80 parameter). None of these is a SUCCESSOR
// vector. The successor frames (QHSM v3 A1/B1/A2, QSLH v2, QSLI-2) WILL CHANGE: the operator's
// RBANK_bootstrap_PROTECT decision (2026-09-26) requires identity material to be hidden from the
// relay in the successor bootstrap, which F04 implements; C01's identity landing then flips
// `ACTIVE` to `TARGET`. When that happens these vectors are re-pinned by that work, not carried.
//
// What is pinned here (INSTRUMENTS.md (a) FF4; e3/e4):
//   * the A1/B1/A2 header and length under ACTIVE (moved here from handshake_mvp.rs's frame
//     parsers, whose v1 shape the head refuses);
//   * W5 -- a parameter block one byte short is refused with the EXACT code
//     REJECT_QSC_HS_MALFORMED_LENGTH (the unit test at src/handshake/mod.rs:3218 asserts only
//     `is_err`);
//   * W6 -- a six-byte frame is refused `handshake_len`;
//   * W9 -- a B1 whose parameter block differs from A1's by one byte is refused
//     REJECT_QSC_HS_CONTEXT_MISMATCH, no session is created, and the pending initiator survives
//     to complete with the genuine B1 (no mutation);
//   * a live real-pair handshake over the leasing relay under ACTIVE.
// Frames are captured and injected through the Mock relay (drain/replace/enqueue is the
// instrument; the properties are the clients' own admission decisions, relay-independent). A
// synthetic local run; it says nothing about production or the real relay deployment.

mod common;

use common::profile;
use common::{PairRelay, VaultFixture};
use quantumshield_refimpl::crypto::stdcrypto::{
    runtime_pq_kem_ciphertext_bytes, runtime_pq_kem_public_key_bytes,
    runtime_pq_sig_public_key_bytes, runtime_pq_sig_signature_bytes,
};

const ROUTE_TOKEN_ALICE: &str = "f03_s6a_vec_route_alice_012345678";
const ROUTE_TOKEN_BOB: &str = "f03_s6a_vec_route_bob_0123456789ab";

const HS_TYPE_A1: u8 = 1;
const HS_TYPE_B1: u8 = 2;
const HS_TYPE_A2: u8 = 3;

/// The QHSM parameter block the head emits under ACTIVE: the critical suite-context parameter
/// (0x0001: Suite-2 protocol 0x0500, suite 0x0002) followed by the critical 0x7f80 profile
/// parameter carrying `profile::ACTIVE.id`.
fn active_parameter_block() -> Vec<u8> {
    let mut block = vec![0x00, 0x01, 0x01, 0x00, 0x04, 0x05, 0x00, 0x00, 0x02];
    block.extend([0x7f, 0x80, 0x01]);
    block.extend((profile::ACTIVE.id.len() as u16).to_be_bytes());
    block.extend(profile::ACTIVE.id.as_bytes());
    block
}

/// magic(4) || version(2) = 2 || type(1) || block_len(2) || block.
fn active_header(frame_type: u8) -> Vec<u8> {
    let block = active_parameter_block();
    let mut out = b"QHSM".to_vec();
    out.extend(2u16.to_be_bytes());
    out.push(frame_type);
    out.extend((block.len() as u16).to_be_bytes());
    out.extend(block);
    out
}

fn text(out: &std::process::Output) -> String {
    format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    )
}

fn session_exists(v: &VaultFixture, peer: &str) -> bool {
    v.cfg
        .join("qsp_sessions")
        .join(format!("{peer}.qsv"))
        .exists()
}

fn public_field(text: &str, field: &str) -> String {
    let value = text
        .lines()
        .find_map(|line| line.strip_prefix(field))
        .unwrap_or_else(|| panic!("missing {field}: {text}"));
    common::scraped_marker_value(field, value)
}

/// Two successor vaults with pinned identities, trusted devices and inbox routes, and NO
/// handshake yet (the vectors intercept it). Same steps as `common::init_real_pair`'s setup.
fn pinned_pair(tag: &str) -> (VaultFixture, VaultFixture) {
    let alice =
        common::init_successor_vault(&format!("{tag}_a"), common::TEST_MOCK_VAULT_PASSPHRASE);
    let bob = common::init_successor_vault(&format!("{tag}_b"), common::TEST_MOCK_VAULT_PASSPHRASE);
    for (v, label, route) in [
        (&alice, "alice", ROUTE_TOKEN_ALICE),
        (&bob, "bob", ROUTE_TOKEN_BOB),
    ] {
        v.run_ok(&["identity", "rotate", "--as", label, "--confirm"]);
        v.run_ok(&["relay", "inbox-set", "--token", route]);
    }
    let alice_public = alice.run_ok(&["identity", "show", "--as", "alice"]);
    let bob_public = bob.run_ok(&["identity", "show", "--as", "bob"]);
    for (v, label, route, public) in [
        (&alice, "bob", ROUTE_TOKEN_BOB, &bob_public),
        (&bob, "alice", ROUTE_TOKEN_ALICE, &alice_public),
    ] {
        v.run_ok(&[
            "contacts",
            "add",
            "--label",
            label,
            "--fp",
            &public_field(public, "identity_fp="),
            "--kem-pk",
            &public_field(public, "identity_kem_pk="),
            "--sig-pk",
            &public_field(public, "identity_sig_pk="),
            "--route-token",
            route,
        ]);
        let devices = v.run_ok(&["contacts", "device", "list", "--label", label]);
        let device = devices
            .lines()
            .find_map(|line| line.strip_prefix("device="))
            .and_then(|line| line.split_whitespace().next())
            .unwrap_or_else(|| panic!("missing device: {devices}"));
        let device = common::scraped_marker_value("device", device);
        v.run_ok(&[
            "contacts",
            "device",
            "trust",
            "--label",
            label,
            "--device",
            &device,
            "--confirm",
        ]);
    }
    (alice, bob)
}

fn hs(v: &VaultFixture, verb: &str, me: &str, peer: &str, relay: &str) -> String {
    let mut args = vec![
        "handshake",
        verb,
        "--as",
        me,
        "--peer",
        peer,
        "--relay",
        relay,
    ];
    if verb == "poll" {
        args.extend(["--max", "4"]);
    }
    let out = v.run(&args);
    let t = text(&out);
    assert!(out.status.success(), "handshake {verb} failed: {t}");
    t
}

/// Take exactly one frame from a mailbox and put it back (capture without consuming).
fn capture_one(server: &common::InboxTestServer, route: &str) -> Vec<u8> {
    let items = server.drain_channel(route);
    assert_eq!(items.len(), 1, "exactly one handshake frame in the mailbox");
    server.replace_channel(route, items.clone());
    items.into_iter().next().unwrap()
}

#[test]
fn active_profile_frames_pin_the_implemented_header() {
    let server = common::start_inbox_server(1024 * 1024, 16);
    let relay = server.base_url().to_string();
    let (alice, bob) = pinned_pair("f03_s6a_vec_shape");

    hs(&alice, "init", "alice", "bob", &relay);
    let a1 = capture_one(&server, ROUTE_TOKEN_BOB);
    hs(&bob, "poll", "bob", "alice", &relay);
    let b1 = capture_one(&server, ROUTE_TOKEN_ALICE);
    let a_done = hs(&alice, "poll", "alice", "bob", &relay);
    let a2 = capture_one(&server, ROUTE_TOKEN_BOB);
    let b_done = hs(&bob, "poll", "bob", "alice", &relay);

    let kem_pk = runtime_pq_kem_public_key_bytes();
    let kem_ct = runtime_pq_kem_ciphertext_bytes();
    let sig_pk = runtime_pq_sig_public_key_bytes();
    let sig = runtime_pq_sig_signature_bytes();
    for (name, frame, frame_type, payload_len) in [
        // A1: sid || kem_pk || sig_pk || dh_pub || resp_kem_ct
        ("A1", &a1, HS_TYPE_A1, 16 + kem_pk + sig_pk + 32 + kem_ct),
        // B1: sid || kem_ct || mac || sig_pk || sig || dh_pub
        ("B1", &b1, HS_TYPE_B1, 16 + kem_ct + 32 + sig_pk + sig + 32),
        // A2: sid || mac || sig
        ("A2", &a2, HS_TYPE_A2, 16 + 32 + sig),
    ] {
        let header = active_header(frame_type);
        assert_eq!(&frame[0..4], b"QHSM", "{name} magic");
        assert_eq!(
            u16::from_be_bytes([frame[4], frame[5]]),
            2,
            "{name} version (explicit suite)"
        );
        assert_eq!(frame[6], frame_type, "{name} type");
        assert_eq!(
            &frame[..header.len()],
            header.as_slice(),
            "{name} header and parameter block under profile::ACTIVE ({})",
            profile::ACTIVE.id
        );
        assert_eq!(frame.len(), header.len() + payload_len, "{name} length");
    }
    // The three frames of one handshake carry one session id.
    let off = active_header(HS_TYPE_A1).len();
    assert_eq!(&a1[off..off + 16], &b1[off..off + 16], "A1/B1 session id");
    assert_eq!(&a1[off..off + 16], &a2[off..off + 16], "A1/A2 session id");
    assert!(a_done.contains("event=handshake_complete"), "{a_done}");
    assert!(b_done.contains("event=handshake_complete"), "{b_done}");
}

/// W5 (INSTRUMENTS.md e3): a parameter block ONE BYTE SHORT, with every declared length made
/// consistent so the parser -- not the frame-length check -- meets it, is refused with the EXACT
/// code REJECT_QSC_HS_MALFORMED_LENGTH (src/handshake/mod.rs:317-318), and the responder writes
/// nothing.
#[test]
fn w5_truncated_parameter_block_is_refused_with_the_exact_code() {
    let server = common::start_inbox_server(1024 * 1024, 16);
    let relay = server.base_url().to_string();
    let (alice, bob) = pinned_pair("f03_s6a_vec_w5");

    hs(&alice, "init", "alice", "bob", &relay);
    let mut a1 = server.drain_channel(ROUTE_TOKEN_BOB).pop().expect("A1");
    let block_len = u16::from_be_bytes([a1[7], a1[8]]) as usize;
    assert_eq!(
        block_len,
        active_parameter_block().len(),
        "genuine A1 block"
    );
    a1.remove(9 + block_len - 1);
    a1[7..9].copy_from_slice(&((block_len - 1) as u16).to_be_bytes());
    server.enqueue_raw(ROUTE_TOKEN_BOB, a1);

    let out = hs(&bob, "poll", "bob", "alice", &relay);
    assert!(
        out.contains("event=handshake_reject reason=REJECT_QSC_HS_MALFORMED_LENGTH"),
        "W5: the exact code must be MALFORMED_LENGTH: {out}"
    );
    assert_eq!(
        out.matches("event=handshake_reject").count(),
        1,
        "W5: one refusal, of that frame: {out}"
    );
    assert!(
        !out.contains("event=handshake_send"),
        "W5: no B1 may be produced: {out}"
    );
    assert!(
        server.drain_channel(ROUTE_TOKEN_ALICE).is_empty(),
        "W5: nothing sent to the initiator"
    );
    assert!(!session_exists(&bob, "alice"), "W5: no session");
    let status = hs_status(&bob, "alice");
    assert!(
        status.contains("status=no_session"),
        "W5: responder unchanged: {status}"
    );
}

fn hs_status(v: &VaultFixture, peer: &str) -> String {
    let out = v.run(&["handshake", "status", "--peer", peer]);
    let t = text(&out);
    assert!(out.status.success(), "{t}");
    t
}

/// W6 (INSTRUMENTS.md e4): a six-byte frame -- shorter than the fixed 7-byte QHSM prefix -- is
/// refused `handshake_len` (src/handshake/mod.rs:465-466) and writes nothing.
#[test]
fn w6_six_byte_frame_is_refused_handshake_len() {
    let server = common::start_inbox_server(1024 * 1024, 16);
    let relay = server.base_url().to_string();
    let (_alice, bob) = pinned_pair("f03_s6a_vec_w6");

    let six = active_header(HS_TYPE_A1)[..6].to_vec();
    assert_eq!(six.len(), 6);
    server.enqueue_raw(ROUTE_TOKEN_BOB, six);

    let out = hs(&bob, "poll", "bob", "alice", &relay);
    assert!(
        out.contains("event=handshake_reject reason=handshake_len"),
        "W6: a six-byte frame must be refused handshake_len: {out}"
    );
    assert!(!out.contains("event=handshake_send"), "W6: no reply: {out}");
    assert!(!session_exists(&bob, "alice"), "W6: no session");
    let status = hs_status(&bob, "alice");
    assert!(
        status.contains("status=no_session"),
        "W6: responder unchanged: {status}"
    );
}

/// W9: a B1 whose parameter block differs from A1's by ONE byte (the last byte of the profile
/// value) is refused REJECT_QSC_HS_CONTEXT_MISMATCH (hs_contexts_match, src/handshake/mod.rs:1308,
/// checked at :2014), no session is created, and the initiator's pending handshake is NOT
/// mutated: the genuine B1, delivered afterwards, still completes it.
#[test]
fn w9_b1_block_differing_by_one_byte_is_refused_no_session() {
    let server = common::start_inbox_server(1024 * 1024, 16);
    let relay = server.base_url().to_string();
    let (alice, bob) = pinned_pair("f03_s6a_vec_w9");

    hs(&alice, "init", "alice", "bob", &relay);
    hs(&bob, "poll", "bob", "alice", &relay);
    let genuine = server.drain_channel(ROUTE_TOKEN_ALICE).pop().expect("B1");
    let block_len = u16::from_be_bytes([genuine[7], genuine[8]]) as usize;
    let mut forged = genuine.clone();
    forged[9 + block_len - 1] ^= 0x01;
    assert_ne!(forged, genuine);
    server.enqueue_raw(ROUTE_TOKEN_ALICE, forged);

    let out = hs(&alice, "poll", "alice", "bob", &relay);
    assert!(
        out.contains("event=handshake_reject reason=REJECT_QSC_HS_CONTEXT_MISMATCH"),
        "W9: a one-byte block difference must be refused CONTEXT_MISMATCH: {out}"
    );
    assert!(!out.contains("event=handshake_complete"), "W9: {out}");
    assert!(!session_exists(&alice, "bob"), "W9: no session");
    assert!(
        server.drain_channel(ROUTE_TOKEN_BOB).is_empty(),
        "W9: no A2 sent"
    );

    // No mutation: the pending initiator is intact and the genuine B1 still completes it.
    server.enqueue_raw(ROUTE_TOKEN_ALICE, genuine);
    let done = hs(&alice, "poll", "alice", "bob", &relay);
    assert!(
        done.contains("event=handshake_complete peer=bob role=initiator"),
        "W9: the refused forgery must not have consumed the pending handshake: {done}"
    );
    assert!(session_exists(&alice, "bob"));
}

/// The live case: a real pair over the real in-process LEASING relay under ACTIVE.
#[test]
fn live_real_pair_handshake_over_leasing_completes_under_the_active_profile() {
    let server = common::start_qsl_server(1024 * 1024, 16, None);
    let pair = common::init_real_pair(
        "f03_s6a_vec_live",
        PairRelay::Leasing(&server),
        ("alice", ROUTE_TOKEN_ALICE),
        ("bob", ROUTE_TOKEN_BOB),
    );
    assert!(session_exists(&pair.a, "bob") && session_exists(&pair.b, "alice"));
    let a = hs_status(&pair.a, "bob");
    let b = hs_status(&pair.b, "alice");
    assert!(
        a.contains("status=awaiting_peer_confirm") && a.contains("send_ready=yes"),
        "{a}"
    );
    assert!(
        b.contains("status=established_recv_only") && b.contains("peer_confirmed=yes"),
        "{b}"
    );
    let (_, profile_id) = common::vault_payload_identity(&pair.a.cfg, &pair.a.passphrase);
    assert_eq!(
        profile_id,
        profile::ACTIVE.id,
        "the pair's vaults are of profile::ACTIVE"
    );
}
