//! NA-0681 (D616 §5) — the invite system against the REAL Slice-1 relay, in process.
//!
//! WHY THIS FILE EXISTS SEPARATELY FROM `NA_0681_invite_encodings.rs`.
//!
//! That file proves qsc agrees with itself. This one proves qsc agrees with the RELAY —
//! the property Slice 1's own opacity test structurally could not observe, because it
//! encodes and decodes with the relay's codec at both ends (D616 C8, logged as the epic's
//! first §3b hollow-proof entry). No mocks: `common::start_qsl_server` runs qsl-server's
//! actual router at the pinned Slice-1 commit.
//!
//! ⚠ WHAT A GREEN HERE DOES NOT ASSERT: nothing about messaging or delivery states
//! (Slice 3), nothing about the GUI (Slice 4), no timing or constant-time claim, and
//! nothing about a multi-host topology — both parties run on ONE host, which is the ruled
//! acceptance topology (epic §4 Q2) and is not two machines.

mod common;

use qsc::invite::*;
use quantumshield_refimpl::crypto::stdcrypto::{runtime_pq_sig_keypair, StdCrypto};
use quantumshield_refimpl::crypto::traits::PqSigMldsa65;
use std::sync::{Mutex, MutexGuard, OnceLock};

fn guard() -> MutexGuard<'static, ()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
        .lock()
        .unwrap_or_else(|e| e.into_inner())
}

fn id(seed: u8) -> [u8; ID_LEN] {
    let mut a = [0u8; ID_LEN];
    for (i, b) in a.iter_mut().enumerate() {
        *b = seed.wrapping_add(i as u8);
    }
    a
}

/// A realistic identity bundle: ML-KEM-768 public key sized, real ML-DSA-65 key.
fn bundle_and_key() -> (Vec<u8>, Vec<u8>, Vec<u8>) {
    let (sig_pk, sig_sk) = runtime_pq_sig_keypair();
    let kem_pk = vec![0xC3u8; 1184];
    let bundle = canonical_bundle_bytes(&kem_pk, &sig_pk).expect("bundle");
    (bundle, sig_pk, sig_sk)
}

fn post(base: &str, path: &str, body: serde_json::Value) -> (u16, String) {
    let c = reqwest::blocking::Client::new();
    let r = c
        .post(format!("{base}{path}"))
        .json(&body)
        .send()
        .expect("relay reachable");
    let s = r.status().as_u16();
    (s, r.text().unwrap_or_default())
}

fn create(
    base: &str,
    iid: &str,
    cap: &str,
    bundle: &[u8],
    sig: &[u8],
    expiry: u64,
) -> (u16, String) {
    post(
        base,
        "/v1/invite/create",
        serde_json::json!({
            "invite_id": iid,
            "cap_hash": cap_hash_hex(cap),
            "expiry": expiry,
            "bundle_b64": base64_url(bundle),
            "invite_sig_b64": base64_url(sig),
        }),
    )
}

fn base64_url(b: &[u8]) -> String {
    use base64::engine::general_purpose::URL_SAFE_NO_PAD;
    use base64::Engine;
    URL_SAFE_NO_PAD.encode(b)
}

fn base64_url_decode(s: &str) -> Vec<u8> {
    use base64::engine::general_purpose::URL_SAFE_NO_PAD;
    use base64::Engine;
    URL_SAFE_NO_PAD
        .decode(s)
        .expect("relay returned valid base64url")
}

fn future() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs()
        + 3600
}

// ---------------------------------------------------------------------------
// §5.7 — THE CROSS-IMPLEMENTATION CODEC GATE (OBS-BY / C2 / C8)
// ---------------------------------------------------------------------------

/// The property Slice 1 could not prove about itself: bytes written by QSC's codec come
/// back from the RELAY's codec byte-for-byte identical.
///
/// Asserted on DECODED BYTES, never on the base64 strings — the relay accepts padded input
/// and emits unpadded, so a string comparison would pass every same-implementation test and
/// break against a relay that re-encodes.
///
/// Two payloads, deliberately: 256 adversarial bytes that are not valid anything, AND a
/// REAL identity bundle, because the adversarial vector proves the codec is byte-clean while
/// the real bundle proves the thing we actually ship survives it.
#[test]
fn qsc_and_relay_codecs_agree_byte_for_byte() {
    let _g = guard();
    let relay = common::start_qsl_server(2 * 1024 * 1024, 512, None);
    let base = relay.base_url();

    let adversarial: Vec<u8> = (0..=255u8).rev().collect();
    let (real_bundle, _pk, _sk) = bundle_and_key();

    for (n, (name, payload)) in [
        ("256 adversarial bytes", adversarial),
        ("the real identity bundle", real_bundle),
    ]
    .into_iter()
    .enumerate()
    {
        // Distinct slots per payload: reusing one invite_id makes the second create a
        // legitimate ERR_INVITE_DUPLICATE, which would look like a codec failure.
        let iid = wire_id(&id(0x40 + n as u8 * 0x10));
        let cap = wire_id(&id(0x41 + n as u8 * 0x10));
        let sig = vec![0xFEu8, 0x00, 0x01, 0xFF];
        let (st, body) = create(&base, &iid, &cap, &payload, &sig, future());
        assert_eq!(st, 200, "create failed for {name}: {body}");

        let (st, body) = post(
            &base,
            "/v1/invite/redeem",
            serde_json::json!({"invite_id": iid, "cap": cap}),
        );
        assert_eq!(st, 200, "redeem failed for {name}: {body}");
        let v: serde_json::Value = serde_json::from_str(&body).expect("json");

        let returned = base64_url_decode(v["bundle_b64"].as_str().expect("bundle_b64"));
        assert_eq!(
            returned, payload,
            "{name}: relay round trip is NOT byte-identical"
        );
        let returned_sig = base64_url_decode(v["invite_sig_b64"].as_str().expect("sig"));
        assert_eq!(returned_sig, sig, "{name}: signature bytes drifted");
    }
}

/// The padding asymmetry, pinned as a RULE rather than left as folklore: the relay ACCEPTS
/// a padded upload (it strips `=`) but always EMITS unpadded. A client that compared the
/// returned string to what it sent would break here while every byte was actually fine.
#[test]
fn relay_accepts_padded_upload_but_emits_unpadded() {
    let _g = guard();
    let relay = common::start_qsl_server(2 * 1024 * 1024, 512, None);
    let base = relay.base_url();

    let payload = b"opaque bundle bytes, length 2 mod 3".to_vec();
    let padded = {
        use base64::engine::general_purpose::URL_SAFE;
        use base64::Engine;
        URL_SAFE.encode(&payload)
    };
    assert!(padded.contains('='), "test vector must actually be padded");

    let iid = wire_id(&id(0x50));
    let cap = wire_id(&id(0x51));
    let (st, body) = post(
        &base,
        "/v1/invite/create",
        serde_json::json!({
            "invite_id": iid,
            "cap_hash": cap_hash_hex(&cap),
            "expiry": future(),
            "bundle_b64": padded,
            "invite_sig_b64": base64_url(&[0xAAu8; 4]),
        }),
    );
    assert_eq!(st, 200, "relay must accept padded input: {body}");

    let (st, body) = post(
        &base,
        "/v1/invite/redeem",
        serde_json::json!({"invite_id": iid, "cap": cap}),
    );
    assert_eq!(st, 200, "{body}");
    let v: serde_json::Value = serde_json::from_str(&body).unwrap();
    let returned_str = v["bundle_b64"].as_str().unwrap();
    assert!(
        !returned_str.contains('='),
        "relay must emit UNPADDED base64url"
    );
    // The bytes are identical even though the STRINGS are not. This is the whole point.
    assert_eq!(base64_url_decode(returned_str), payload);
    assert_ne!(
        returned_str, padded,
        "the strings differ — only the bytes match"
    );
}

// ---------------------------------------------------------------------------
// §5.8 — THE SILENT-200 GUARD (C4): the only failure that returns success
// ---------------------------------------------------------------------------

/// (8a) A ticketless push to a live invite slot must be REFUSED with 403.
///
/// (8b) A MIS-RENDERED route token must NOT be accepted as success. This is the assertion
/// most likely to be written backwards, so it asserts the two outcomes are DIFFERENT rather
/// than asserting a status in isolation: correct rendering -> 403 (the slot was found and
/// it refused us), mis-rendered -> 200 (the slot was never found at all, and the push
/// vanished into an unrelated route).
///
/// A 200 on the correct rendering would mean the route token did not resolve to the slot —
/// which is exactly the silent failure C4 describes.
#[test]
fn a_ticketless_push_is_refused_and_a_misrendered_token_is_not_success() {
    let _g = guard();
    let relay = common::start_qsl_server(2 * 1024 * 1024, 512, None);
    let base = relay.base_url();

    let iid_bytes = id(0x60);
    let iid = wire_id(&iid_bytes);
    let cap = wire_id(&id(0x61));
    let (bundle, _pk, _sk) = bundle_and_key();
    let (st, body) = create(&base, &iid, &cap, &bundle, &[0xAAu8; 8], future());
    assert_eq!(st, 200, "{body}");
    // Consume it so the slot is live and ticketed.
    let (st, _b) = post(
        &base,
        "/v1/invite/redeem",
        serde_json::json!({"invite_id": iid, "cap": cap}),
    );
    assert_eq!(st, 200);

    let c = reqwest::blocking::Client::new();
    let push = |route: &str| -> u16 {
        c.post(format!("{base}/v1/push"))
            .header("X-QSL-Route-Token", route)
            .body(vec![0x01u8, 0x02, 0x03])
            .send()
            .expect("push")
            .status()
            .as_u16()
    };

    // (8a) correct rendering, no ticket -> the slot is FOUND and refuses.
    let correct = push(&iid);
    assert_eq!(
        correct, 403,
        "a ticketless push to a live invite slot must be 403; a 200 means the route token \
         did not resolve to the slot"
    );

    // (8b) the realistic drift: uppercase hex. The slot is NOT found, and the relay accepts
    // the push into an unrelated route — the silent success C4 warns about.
    let misrendered = push(&iid.to_uppercase());
    assert_eq!(
        misrendered, 200,
        "a mis-rendered token lands in a non-slot route (this is the hazard, demonstrated)"
    );
    assert_ne!(
        correct, misrendered,
        "correct and mis-rendered renderings MUST behave differently — if they ever agree, \
         the guard has stopped observing anything"
    );
}

// ---------------------------------------------------------------------------
// §5.3 / §5.5 — replay and the interception race, relay-side arm
// ---------------------------------------------------------------------------

/// A second redemption returns ALREADY_USED, **not** NOT_FOUND. The distinction is the
/// interception signal: "someone got here first" versus "this never existed". The relay
/// keeps a tombstone precisely so the two stay tellable apart.
#[test]
fn a_replayed_redemption_is_already_used_not_not_found() {
    let _g = guard();
    let relay = common::start_qsl_server(2 * 1024 * 1024, 512, None);
    let base = relay.base_url();

    let iid = wire_id(&id(0x70));
    let cap = wire_id(&id(0x71));
    let (bundle, _pk, _sk) = bundle_and_key();
    let (st, body) = create(&base, &iid, &cap, &bundle, &[0xBBu8; 8], future());
    assert_eq!(st, 200, "{body}");

    let (st, _b) = post(
        &base,
        "/v1/invite/redeem",
        serde_json::json!({"invite_id": iid, "cap": cap}),
    );
    assert_eq!(st, 200, "the first redemption must win");

    let (st, body) = post(
        &base,
        "/v1/invite/redeem",
        serde_json::json!({"invite_id": iid, "cap": cap}),
    );
    assert_eq!(st, 409, "second redemption: {body}");
    assert!(
        body.contains("ERR_INVITE_ALREADY_USED"),
        "must say ALREADY_USED, not NOT_FOUND: {body}"
    );

    // The negative half: an invite that genuinely never existed is a DIFFERENT answer.
    // Without this, the assertion above would pass against a relay that said ALREADY_USED
    // for everything.
    let (st, body) = post(
        &base,
        "/v1/invite/redeem",
        serde_json::json!({"invite_id": wire_id(&id(0x7F)), "cap": cap}),
    );
    assert_eq!(st, 404, "unknown invite: {body}");
    assert!(body.contains("ERR_INVITE_NOT_FOUND"), "{body}");
}

/// A same-length wrong capability is refused with no mutation — the D-0014 lesson, from the
/// client's side of the wire. A different-length value would prove nothing about the fold.
#[test]
fn a_same_length_wrong_capability_is_refused_and_the_slot_survives() {
    let _g = guard();
    let relay = common::start_qsl_server(2 * 1024 * 1024, 512, None);
    let base = relay.base_url();

    let iid = wire_id(&id(0x80));
    let cap = wire_id(&id(0x81));
    let wrong = wire_id(&id(0x82));
    assert_eq!(cap.len(), wrong.len(), "the control requires equal lengths");

    let (bundle, _pk, _sk) = bundle_and_key();
    let (st, _b) = create(&base, &iid, &cap, &bundle, &[0xCCu8; 8], future());
    assert_eq!(st, 200);

    let (st, body) = post(
        &base,
        "/v1/invite/redeem",
        serde_json::json!({"invite_id": iid, "cap": wrong}),
    );
    assert_eq!(st, 403, "{body}");
    assert!(body.contains("ERR_INVITE_CAP_INVALID"), "{body}");

    // NO MUTATION: the real capability still works afterwards. Without this the test would
    // pass against a relay that burned the slot on every attempt.
    let (st, body) = post(
        &base,
        "/v1/invite/redeem",
        serde_json::json!({"invite_id": iid, "cap": cap}),
    );
    assert_eq!(st, 200, "the slot must survive a wrong capability: {body}");
}

// ---------------------------------------------------------------------------
// §5.1 / §5.2 — the two security failures, over the real wire
// ---------------------------------------------------------------------------

/// End to end through the relay: a bundle the relay SUBSTITUTES fails the commitment, and a
/// tampered code field fails the signature — distinctly, on data that actually made the
/// round trip rather than on locally constructed bytes.
#[test]
fn substitution_and_tampering_fail_distinctly_through_the_real_relay() {
    let _g = guard();
    let relay = common::start_qsl_server(2 * 1024 * 1024, 512, None);
    let base = relay.base_url();

    let (bundle, _sig_pk, sig_sk) = bundle_and_key();
    let iid_bytes = id(0x90);
    let cap_bytes = id(0x91);
    let iid = wire_id(&iid_bytes);
    let cap = wire_id(&cap_bytes);
    let expiry = future();

    let payload = InvitePayload {
        ver: INVITE_VER,
        typ: INVITE_TYPE_CONTACT,
        invite_id: iid_bytes,
        expiry,
        relay_ep: base.to_string(),
        cap: cap_bytes,
        commit: commitment(&bundle),
    };
    let payload_bytes = encode_payload(&payload).expect("encode");
    let invite_sig = StdCrypto
        .sign(&sig_sk, &sig_msg(&payload_bytes))
        .expect("sign");

    // HONEST CASE first, so every rejection below means something.
    let (st, _b) = create(&base, &iid, &cap, &bundle, &invite_sig, expiry);
    assert_eq!(st, 200);
    let (st, body) = post(
        &base,
        "/v1/invite/redeem",
        serde_json::json!({"invite_id": iid, "cap": cap}),
    );
    assert_eq!(st, 200, "{body}");
    let v: serde_json::Value = serde_json::from_str(&body).unwrap();
    let got_bundle = base64_url_decode(v["bundle_b64"].as_str().unwrap());
    let got_sig = base64_url_decode(v["invite_sig_b64"].as_str().unwrap());
    verify_redeemed_bundle(&payload, &payload_bytes, &got_bundle, &got_sig)
        .expect("the honest round trip must verify");

    // SUBSTITUTED BUNDLE: a hostile relay serving different keys.
    let (attacker_pk, _sk) = runtime_pq_sig_keypair();
    let substituted = canonical_bundle_bytes(&vec![0xEEu8; 1184], &attacker_pk).unwrap();
    assert_eq!(
        verify_redeemed_bundle(&payload, &payload_bytes, &substituted, &got_sig).unwrap_err(),
        INVITE_COMMITMENT_MISMATCH
    );

    // TAMPERED FIELD: the attacker cannot re-sign.
    let mut tampered = payload.clone();
    tampered.relay_ep = "https://evil.example.org".to_string();
    let tampered_bytes = encode_payload(&tampered).unwrap();
    assert_eq!(
        verify_redeemed_bundle(&tampered, &tampered_bytes, &got_bundle, &got_sig).unwrap_err(),
        INVITE_SIGNATURE_INVALID
    );
}

// ---------------------------------------------------------------------------
// §5 — clamp tolerance, with the probe DISABLED
// ---------------------------------------------------------------------------

/// C7/F3: a clamped expiry is a NORMAL outcome and never an error.
///
/// Asked for far beyond the relay's ceiling, the create still SUCCEEDS — the relay clamps
/// silently and the client must not treat that as failure. The probe plays no part here,
/// which is the point: the tolerance is the contract, the probe is only an optimisation.
#[test]
fn an_over_long_expiry_is_clamped_not_rejected() {
    let _g = guard();
    let relay = common::start_qsl_server(2 * 1024 * 1024, 512, None);
    let base = relay.base_url();

    let iid = wire_id(&id(0xA0));
    let cap = wire_id(&id(0xA1));
    let (bundle, _pk, _sk) = bundle_and_key();
    // Ten years, far past MAX_INVITE_EXPIRY_SECS.
    let absurd = future() + 10 * 365 * 24 * 3600;
    let (st, body) = create(&base, &iid, &cap, &bundle, &[0xDDu8; 8], absurd);
    assert_eq!(
        st, 200,
        "an over-long expiry must be CLAMPED, not rejected: {body}"
    );

    // And the negative half: an already-past expiry IS rejected, so the acceptance above is
    // not simply "the relay accepts anything".
    let (st, body) = create(
        &base,
        &wire_id(&id(0xA5)),
        &cap,
        &bundle,
        &[0xDDu8; 8],
        1_000_000,
    );
    assert_eq!(
        st, 400,
        "an expiry already in the past must be refused: {body}"
    );
}

/// `resolve_expiry` never asks for more than the ceiling, and treats "not advertised" as
/// UNKNOWN rather than as a ceiling of zero — an older relay must not yield invites that
/// are dead on arrival.
#[test]
fn expiry_resolution_respects_the_ceiling_and_treats_zero_as_unknown() {
    let now = 1_800_000_000u64;
    let day = 86_400u64;

    // Advertised ceiling below the request: clamp to the ceiling, minus the skew margin.
    let e = resolve_expiry(now, 30 * day, day);
    assert!(e <= now + day, "must not exceed the advertised ceiling");
    assert_eq!(e, now + day - CLOCK_SKEW_MARGIN_SECS);

    // Not advertised: honour the request rather than collapsing to zero.
    let e = resolve_expiry(now, 3 * day, 0);
    assert_eq!(e, now + 3 * day - CLOCK_SKEW_MARGIN_SECS);
    assert!(
        e > now,
        "a zero ceiling must never mean an instantly-dead invite"
    );

    // The margin is what buys signed_expiry == stored_expiry when the clocks disagree.
    assert!(resolve_expiry(now, day, day) < now + day);
}
