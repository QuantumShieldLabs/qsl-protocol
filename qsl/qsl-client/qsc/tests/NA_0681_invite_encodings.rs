//! NA-0681 (D616) — the four canonical encodings and the verify order.
//!
//! WHAT THIS FILE IS FOR, AND WHAT IT CANNOT SEE.
//!
//! The Slice-2 census found the cryptography settled and every real risk to be an ENCODING
//! risk. These are the properties that, if they drift, fail as an attack or as *success*
//! rather than as an honest error. Every assertion below is therefore a KNOWN-ANSWER test
//! or a negative control — never the implementation agreeing with itself.
//!
//! The hash and commitment vectors were computed INDEPENDENTLY (Python `hashlib`, before
//! this file was written) and are pinned as literals. If the implementation changes, these
//! fail; if the test and the implementation drift together, they still fail, because
//! neither produced the constant.
//!
//! ⚠ WHAT A GREEN HERE DOES NOT ASSERT: nothing about the relay. Cross-implementation
//! agreement — the OBS-BY property — is `NA_0681_invite_relay_contract.rs`, which runs the
//! real relay. A green here with a red there is exactly the situation Slice 1's opacity
//! test could not distinguish, which is why both exist.

use qsc::invite::*;
use quantumshield_refimpl::crypto::stdcrypto::{runtime_pq_sig_keypair, StdCrypto};
use quantumshield_refimpl::crypto::traits::PqSigMldsa65;

const RELAY: &str = "https://relay.example.org";

fn id(seed: u8) -> [u8; ID_LEN] {
    let mut a = [0u8; ID_LEN];
    for (i, b) in a.iter_mut().enumerate() {
        *b = seed.wrapping_add(i as u8);
    }
    a
}

// ---------------------------------------------------------------------------
// §2a(1) — wire_id. The renderer whose drift returns HTTP 200.
// ---------------------------------------------------------------------------

#[test]
fn wire_id_is_32_lowercase_hex_and_round_trips() {
    let x = id(0);
    let w = wire_id(&x);
    // KNOWN ANSWER, not a re-derivation.
    assert_eq!(w, "000102030405060708090a0b0c0d0e0f");
    assert_eq!(w.len(), 32);
    assert!(w
        .bytes()
        .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit()));
    assert_eq!(wire_id_parse(&w).expect("round trip"), x);
}

/// NEGATIVE CONTROL for the C4 class: the realistic drift is a case change, and it must be
/// REFUSED rather than quietly accepted. If this ever passes, a mis-rendered `invite_id`
/// can reach the wire — where the relay answers 200 and the handshake vanishes.
///
/// What would make this control vacuous: if `wire_id_parse` rejected *everything*. Guarded
/// by the round-trip assertion above, which requires it to accept the correct rendering.
#[test]
fn wire_id_parse_refuses_uppercase_and_wrong_length() {
    let good = wire_id(&id(0));
    assert!(
        wire_id_parse(&good.to_uppercase()).is_err(),
        "uppercase must be refused"
    );
    assert!(wire_id_parse(&good[..30]).is_err(), "short must be refused");
    assert!(
        wire_id_parse(&format!("{good}00")).is_err(),
        "long must be refused"
    );
    assert!(wire_id_parse("").is_err(), "empty must be refused");
    // and the positive case still works, so the refusals above mean something
    assert!(wire_id_parse(&good).is_ok());
}

/// The rendering must satisfy the client's OWN route-token validator, or the handshake push
/// cannot even be formed. Measured at census: 32 chars sits inside `22..=128`.
#[test]
fn wire_id_is_a_valid_route_token() {
    let w = wire_id(&id(7));
    assert!(
        qsc::adversarial::route::route_token_is_valid(&w),
        "wire_id must be usable as a route token"
    );
}

// ---------------------------------------------------------------------------
// §2a(2) — cap_hash. Hashed as a STRING by the relay; a mismatch reads as a wrong cap.
// ---------------------------------------------------------------------------

#[test]
fn cap_hash_is_sha256_over_the_wire_string_lowercase() {
    let cap_wire = wire_id(&id(0));
    assert_eq!(cap_wire, "000102030405060708090a0b0c0d0e0f");
    // KNOWN ANSWER, computed independently.
    assert_eq!(
        cap_hash_hex(&cap_wire),
        "d65df89f702eec58ca3c7bf2001c9ffc7cd80553ae01d42a799ba26756142d96"
    );
}

/// NEGATIVE CONTROLS for the two ways this silently breaks. Both alternatives are pinned as
/// literals, so this test fails if the implementation ever switches to either.
#[test]
fn cap_hash_is_not_over_raw_bytes_and_not_uppercase() {
    let cap_wire = wire_id(&id(0));
    let actual = cap_hash_hex(&cap_wire);
    assert_ne!(
        actual, "be45cb2605bf36bebde684841a28f0fd43c69850a3dce5fedba69928ee3a8991",
        "hashing the RAW 16 bytes instead of the wire string would produce this"
    );
    assert_ne!(
        actual, "2d49968925b3248f1dab2fdd60b0225670182872240293083fd3c0424328d202",
        "hashing the UPPERCASE hex string would produce this"
    );
    assert!(actual.bytes().all(|b| !b.is_ascii_uppercase()));
}

// ---------------------------------------------------------------------------
// §2a(3) — the canonical bundle. Explicit layout, never a re-serialization.
// ---------------------------------------------------------------------------

#[test]
fn canonical_bundle_layout_is_exact_and_known() {
    let kem = vec![0xA1u8; 4];
    let sig = vec![0xB2u8; 3];
    let b = canonical_bundle_bytes(&kem, &sig).expect("bundle");
    // KNOWN ANSWER: ver ‖ len ‖ kem ‖ len ‖ sig
    assert_eq!(hex(&b), "010004a1a1a1a10003b2b2b2");
    let (k, s) = canonical_bundle_parse(&b).expect("parse");
    assert_eq!(k, kem);
    assert_eq!(s, sig);
}

/// The privacy line, enforced structurally: the bundle is uploaded to the relay, so it must
/// carry no timestamp. A `created_at` would be a SECOND relay-visible timestamp.
///
/// This asserts on LENGTH, which is the only way to observe the absence of a field.
#[test]
fn canonical_bundle_carries_no_timestamp() {
    let kem = vec![0xA1u8; 4];
    let sig = vec![0xB2u8; 3];
    let b = canonical_bundle_bytes(&kem, &sig).expect("bundle");
    // 1 (ver) + 2 + 4 + 2 + 3 = 12. An 8-byte created_at would make it 20.
    assert_eq!(
        b.len(),
        12,
        "an extra timestamp field would change this length"
    );
}

#[test]
fn canonical_bundle_rejects_trailing_bytes_and_empty_keys() {
    let b = canonical_bundle_bytes(&[0xA1; 4], &[0xB2; 3]).expect("bundle");
    let mut extended = b.clone();
    extended.push(0x00);
    assert_eq!(
        canonical_bundle_parse(&extended).unwrap_err(),
        INVITE_MALFORMED,
        "no silent extension riding v1"
    );
    assert!(canonical_bundle_bytes(&[], &[0xB2; 3]).is_err());
    assert!(canonical_bundle_bytes(&[0xA1; 4], &[]).is_err());
    assert!(
        canonical_bundle_parse(&b).is_ok(),
        "the positive case still parses"
    );
}

// ---------------------------------------------------------------------------
// §2b — the commitment
// ---------------------------------------------------------------------------

#[test]
fn commitment_is_domain_separated_sha256_known_answer() {
    let b = canonical_bundle_bytes(&[0xA1; 4], &[0xB2; 3]).expect("bundle");
    assert_eq!(
        hex(&commitment(&b)),
        "d4514d31a24413b3683b1c6d5a2ce34a2389d150ca0aee46c906901bc64b01e9"
    );
    assert_eq!(DS_COMMIT, "QSL.invite.identity-commitment.v1");
}

/// The property the commitment exists for: a substituted bundle must not collide.
#[test]
fn commitment_changes_when_any_key_byte_changes() {
    let a = canonical_bundle_bytes(&[0xA1; 4], &[0xB2; 3]).expect("a");
    let b = canonical_bundle_bytes(&[0xA1, 0xA1, 0xA1, 0xA2], &[0xB2; 3]).expect("b");
    assert_ne!(commitment(&a), commitment(&b));
}

// ---------------------------------------------------------------------------
// §2a(4) — the payload and the QSLI-1- code
// ---------------------------------------------------------------------------

fn sample_payload() -> InvitePayload {
    InvitePayload {
        ver: INVITE_VER,
        typ: INVITE_TYPE_CONTACT,
        invite_id: id(0x10),
        expiry: 1_800_000_000,
        relay_ep: RELAY.to_string(),
        cap: id(0x20),
        commit: commitment(&canonical_bundle_bytes(&[0xA1; 4], &[0xB2; 3]).unwrap()),
    }
}

#[test]
fn invite_code_round_trips_and_is_sms_sized() {
    let p = sample_payload();
    let code = encode_invite_code(&p).expect("encode");
    assert!(code.starts_with("QSLI-1-"));
    // I3: the trusted side-channel carries <= ~250 characters.
    assert!(code.len() <= 250, "code was {} chars", code.len());
    assert_eq!(decode_invite_code(&code).expect("decode"), p);
}

#[test]
fn invite_code_is_unpadded_url_safe_base64() {
    let code = encode_invite_code(&sample_payload()).expect("encode");
    let b64 = code.strip_prefix("QSLI-1-").unwrap();
    assert!(!b64.contains('='), "must be unpadded");
    assert!(
        !b64.contains('+') && !b64.contains('/'),
        "must be URL-safe alphabet"
    );
}

/// Parse rules, each a distinct cause. A taxonomy that collapses these tells the user the
/// wrong thing to do about it.
#[test]
fn parse_rules_are_distinct_causes() {
    let p = sample_payload();
    let good = encode_payload(&p).expect("encode");

    // trailing bytes -> rejected, never silently extended
    let mut trailing = good.clone();
    trailing.push(0xFF);
    assert_eq!(decode_payload(&trailing).unwrap_err(), INVITE_MALFORMED);

    // unknown version -> "newer app needed", NOT malformed
    let mut newer = good.clone();
    newer[0] = 0x02;
    assert_eq!(decode_payload(&newer).unwrap_err(), INVITE_VERSION_NEWER);

    // unknown type -> its own cause
    let mut typ = good.clone();
    typ[1] = 0x09;
    assert_eq!(decode_payload(&typ).unwrap_err(), INVITE_TYPE_UNKNOWN);

    // truncated -> malformed
    assert_eq!(
        decode_payload(&good[..good.len() - 5]).unwrap_err(),
        INVITE_MALFORMED
    );
    assert_eq!(decode_payload(&[]).unwrap_err(), INVITE_MALFORMED);

    // the positive case still decodes, so every rejection above is meaningful
    assert!(decode_payload(&good).is_ok());
}

#[test]
fn a_qsli_prefix_with_another_version_reads_as_newer_not_garbage() {
    assert_eq!(
        decode_invite_code("QSLI-2-AAAA").unwrap_err(),
        INVITE_VERSION_NEWER
    );
    assert_eq!(decode_invite_code("hello").unwrap_err(), INVITE_MALFORMED);
}

/// The endpoint reuses the SHIPPED fail-closed policy. Loopback http must be admissible or
/// the slice's own two-party acceptance (real relay, in-process, 127.0.0.1, plain HTTP)
/// cannot run; everything else non-TLS must still be refused.
#[test]
fn relay_endpoint_uses_the_shipped_policy() {
    let mut p = sample_payload();

    p.relay_ep = "http://127.0.0.1:8443".to_string();
    let c = encode_invite_code(&p).expect("encode");
    assert!(
        decode_invite_code(&c).is_ok(),
        "loopback http must be admissible"
    );

    p.relay_ep = "http://relay.example.org".to_string();
    let c = encode_invite_code(&p).expect("encode");
    assert_eq!(
        decode_invite_code(&c).unwrap_err(),
        INVITE_MALFORMED,
        "non-loopback plain http must be refused"
    );
}

// ---------------------------------------------------------------------------
// §2b — VERIFY ORDER: commitment THEN signature, each gating the next
// ---------------------------------------------------------------------------

struct Signed {
    payload: InvitePayload,
    payload_bytes: Vec<u8>,
    bundle: Vec<u8>,
    sig: Vec<u8>,
}

fn make_signed() -> Signed {
    let (sig_pk, sig_sk) = runtime_pq_sig_keypair();
    let kem_pk = vec![0xC3u8; 1184]; // ML-KEM-768 public key size; opaque to this layer
    let bundle = canonical_bundle_bytes(&kem_pk, &sig_pk).expect("bundle");
    let mut payload = sample_payload();
    payload.commit = commitment(&bundle);
    let payload_bytes = encode_payload(&payload).expect("encode");
    let sig = StdCrypto
        .sign(&sig_sk, &sig_msg(&payload_bytes))
        .expect("sign");
    Signed {
        payload,
        payload_bytes,
        bundle,
        sig,
    }
}

#[test]
fn an_honest_invite_verifies() {
    let s = make_signed();
    let (kem, _sig_pk) =
        verify_redeemed_bundle(&s.payload, &s.payload_bytes, &s.bundle, &s.sig).expect("verify");
    assert_eq!(kem.len(), 1184);
}

/// A hostile relay serving a SUBSTITUTE bundle fails at step (1).
///
/// What would make this control vacuous: if `verify_redeemed_bundle` rejected everything.
/// Guarded by `an_honest_invite_verifies` above.
#[test]
fn a_substituted_bundle_fails_the_commitment() {
    let s = make_signed();
    let (attacker_pk, _sk) = runtime_pq_sig_keypair();
    let substituted = canonical_bundle_bytes(&vec![0xEEu8; 1184], &attacker_pk).expect("bundle");
    assert_eq!(
        verify_redeemed_bundle(&s.payload, &s.payload_bytes, &substituted, &s.sig).unwrap_err(),
        INVITE_COMMITMENT_MISMATCH
    );
}

/// A tampered CODE FIELD fails at step (2) — the attacker cannot re-sign. Each field is
/// exercised separately: they are separate opportunities to get the pre-image wrong.
#[test]
fn a_tampered_code_field_fails_the_signature() {
    for (name, mutate) in [
        (
            "relay_ep",
            (|p: &mut InvitePayload| p.relay_ep = "https://evil.example.org".to_string())
                as fn(&mut InvitePayload),
        ),
        ("expiry", |p: &mut InvitePayload| p.expiry += 1),
        ("cap", |p: &mut InvitePayload| p.cap[0] ^= 0xFF),
        ("invite_id", |p: &mut InvitePayload| p.invite_id[0] ^= 0xFF),
    ] {
        let s = make_signed();
        let mut tampered = s.payload.clone();
        mutate(&mut tampered);
        let tampered_bytes = encode_payload(&tampered).expect("encode");
        assert_eq!(
            verify_redeemed_bundle(&tampered, &tampered_bytes, &s.bundle, &s.sig).unwrap_err(),
            INVITE_SIGNATURE_INVALID,
            "tampering with {name} must fail the SIGNATURE, not the commitment"
        );
    }
}

/// The taxonomy's whole point: the two security failures must be TELLABLE APART. A test
/// suite that only asserts "it failed" cannot see the difference between "someone swapped
/// the keys" and "someone edited the invite", which are different things to tell a user.
#[test]
fn the_two_security_failures_are_distinct_codes() {
    assert_ne!(INVITE_COMMITMENT_MISMATCH, INVITE_SIGNATURE_INVALID);
    let s = make_signed();

    let (attacker_pk, _) = runtime_pq_sig_keypair();
    let substituted = canonical_bundle_bytes(&vec![0xEEu8; 1184], &attacker_pk).unwrap();
    let commit_err =
        verify_redeemed_bundle(&s.payload, &s.payload_bytes, &substituted, &s.sig).unwrap_err();

    let mut tampered = s.payload.clone();
    tampered.expiry += 1;
    let tampered_bytes = encode_payload(&tampered).unwrap();
    let sig_err =
        verify_redeemed_bundle(&tampered, &tampered_bytes, &s.bundle, &s.sig).unwrap_err();

    assert_ne!(commit_err, sig_err, "the two causes must not collapse");
}

/// ORDER, asserted directly: when BOTH are wrong, the commitment failure is what surfaces.
/// If the implementation ever checked the signature first, this flips — and the user would
/// be told "the invite was edited" when the truth is "the relay swapped the keys".
#[test]
fn commitment_is_checked_before_signature() {
    let s = make_signed();
    let (attacker_pk, _) = runtime_pq_sig_keypair();
    let substituted = canonical_bundle_bytes(&vec![0xEEu8; 1184], &attacker_pk).unwrap();
    let mut tampered = s.payload.clone();
    tampered.expiry += 1;
    let tampered_bytes = encode_payload(&tampered).unwrap();
    assert_eq!(
        verify_redeemed_bundle(&tampered, &tampered_bytes, &substituted, &s.sig).unwrap_err(),
        INVITE_COMMITMENT_MISMATCH,
        "with both broken, the FIRST gate must be the one that reports"
    );
}

// ---------------------------------------------------------------------------
// §2e — the QSLH-1 envelope (F1: wrapping, with the A1 frame carried VERBATIM)
// ---------------------------------------------------------------------------

fn sample_envelope() -> HandshakeEnvelope {
    HandshakeEnvelope {
        bundle: canonical_bundle_bytes(&[0xA1; 4], &[0xB2; 3]).unwrap(),
        route_token: "abcdefghijklmnopqrstuvwxyz012345".to_string(),
        a1: vec![0x51, 0x52, 0x53, 0x54],
    }
}

#[test]
fn envelope_round_trips_and_carries_a1_verbatim() {
    let e = sample_envelope();
    let bytes = encode_envelope(&e).expect("encode");
    let back = decode_envelope(&bytes).expect("decode");
    assert_eq!(back, e);
    // The A1 bytes must survive untouched: the transcript MAC binds them, so any
    // re-encoding here would break the handshake for an encoding reason.
    assert_eq!(back.a1, e.a1);
}

/// v1 does not loosen: an unknown tag is REFUSED, not skipped. A future device manifest
/// arrives under a bumped version, which gets its own risk review.
#[test]
fn envelope_refuses_unknown_and_duplicate_tags() {
    let e = sample_envelope();
    let good = encode_envelope(&e).expect("encode");

    let mut unknown = good.clone();
    unknown.extend_from_slice(&[0x7F, 0x00, 0x01, 0xAA]);
    assert_eq!(
        decode_envelope(&unknown).unwrap_err(),
        HANDSHAKE_ENVELOPE_MALFORMED,
        "unknown tag must be refused"
    );

    let mut dup = good.clone();
    dup.extend_from_slice(&[TAG_A1, 0x00, 0x01, 0xAA]);
    assert_eq!(
        decode_envelope(&dup).unwrap_err(),
        HANDSHAKE_ENVELOPE_MALFORMED,
        "a duplicate tag must be refused, not last-wins"
    );

    let mut newer = good.clone();
    newer[0] = 0x02;
    assert_eq!(
        decode_envelope(&newer).unwrap_err(),
        HANDSHAKE_ENVELOPE_VERSION_NEWER
    );

    assert!(
        decode_envelope(&good).is_ok(),
        "the positive case still decodes"
    );
}

/// A route token we could never send to is refused at decode, not stored and discovered
/// later.
#[test]
fn envelope_refuses_an_unusable_route_token() {
    let mut e = sample_envelope();
    e.route_token = "short".to_string();
    let bytes = encode_envelope(&e).expect("encode");
    assert_eq!(
        decode_envelope(&bytes).unwrap_err(),
        HANDSHAKE_ENVELOPE_MALFORMED
    );
}

fn hex(b: &[u8]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect()
}
