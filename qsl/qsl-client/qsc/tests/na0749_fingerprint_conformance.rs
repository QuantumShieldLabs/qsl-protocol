//! NA-0749 (D-1391) — the `qsl-fp-v1` conformance vector, asserted against values computed
//! OUTSIDE this codebase and sealed before any of it was written.
//!
//! Provenance: `/srv/qbuild/operator/NA-0749/vector_v2/`, sealed at STOP 004 and re-verified
//! unchanged at STOP 005 §7. The values were produced by an independent Python tool whose SHA-512
//! was itself validated against the published NIST vector for `sha512("abc")` BEFORE any lane value
//! was computed, and corroborated by coreutils `sha512sum`, OpenSSL, and two further engines the
//! SR-15 re-read wrote — one of them implemented from FIPS 180-4 in-session.
//!
//! ⚠ The RATIFIED mockup's fingerprint values are FABRICATED PLACEHOLDERS and appear in NO test.
//! Every value below is computed, not illustrative.
//!
//! If this test and the Rust disagree, one of them is wrong and this test says so. That is its
//! entire purpose: the implementation is checked against an independent artefact, never against
//! itself.

use qsc::identity::{identity_fingerprint_from_identity, identity_voice_form};

/// The synthetic key material the vector fixes. NOT real keys, and deliberately not random:
/// fully specified so any reader can regenerate the preimage byte for byte.
/// `kem_pk[i] = i mod 256` over 1184 bytes (ML-KEM-768 public key length).
/// `sig_pk[i] = 255 - (i mod 256)` over 1952 bytes (ML-DSA-65 public key length).
fn kem_pk() -> Vec<u8> {
    (0..1184u32).map(|i| (i % 256) as u8).collect()
}
fn sig_pk() -> Vec<u8> {
    (0..1952u32).map(|i| (255 - (i % 256)) as u8).collect()
}

/// The sealed FULL form: lowercase hex of the digest's first 32 bytes, 64 characters, no prefix.
const SEALED_IDENTITY_FULL: &str =
    "d67b4a10510394ca268c9e8cfde8980fd6280dc8c379d4ea8c8642ac9a750349";

/// The sealed VOICE form: exactly 30 decimal digits, leading zeros legal.
const SEALED_IDENTITY_VOICE: &str = "187363336018275058094178831816";

#[test]
fn na0749_identity_fingerprint_matches_the_sealed_independent_vector() {
    let fp = identity_fingerprint_from_identity(&kem_pk(), &sig_pk());
    assert_eq!(
        fp, SEALED_IDENTITY_FULL,
        "the combined identity fingerprint does not match the sealed independent vector"
    );
}

#[test]
fn na0749_identity_voice_form_matches_the_sealed_independent_vector() {
    let fp = identity_fingerprint_from_identity(&kem_pk(), &sig_pk());
    let voice = identity_voice_form(&fp);
    assert_eq!(
        voice, SEALED_IDENTITY_VOICE,
        "the voice form does not match the sealed independent vector"
    );
}

/// C4/W4: the voice form is EXACTLY 30 decimal digits, and it is derivable from the FULL form
/// alone — the property `identity_pin_matches_seen_identity`'s second tier depends on, since that
/// function receives the fingerprint only as a string.
#[test]
fn na0749_voice_form_is_thirty_digits_and_derivable_from_the_full_form_alone() {
    let fp = identity_fingerprint_from_identity(&kem_pk(), &sig_pk());
    let voice = identity_voice_form(&fp);
    assert_eq!(voice.len(), 30, "voice form must be exactly 30 characters");
    assert!(
        voice.chars().all(|c| c.is_ascii_digit()),
        "voice form must be digits only, got {voice}"
    );
    // Derivability across the API boundary: the only input is the rendered 64-hex string.
    assert_eq!(voice, identity_voice_form(SEALED_IDENTITY_FULL));
}

/// C3: the full form carries NO prefix and is lowercase hex. The retired format began `QSCFP-`.
#[test]
fn na0749_full_form_has_no_prefix_and_is_lowercase_hex() {
    let fp = identity_fingerprint_from_identity(&kem_pk(), &sig_pk());
    assert_eq!(fp.len(), 64, "full form must be 64 hex characters");
    assert!(!fp.contains('-'), "full form must carry no prefix or grouping");
    assert!(
        fp.chars().all(|c| c.is_ascii_digit() || ('a'..='f').contains(&c)),
        "full form must be LOWERCASE hex, got {fp}"
    );
}

/// C7: determinism — the construction contains no ambient input (no time, no rng, no config).
#[test]
fn na0749_construction_is_deterministic_across_calls() {
    let a = identity_fingerprint_from_identity(&kem_pk(), &sig_pk());
    let b = identity_fingerprint_from_identity(&kem_pk(), &sig_pk());
    assert_eq!(a, b);
    assert_eq!(identity_voice_form(&a), identity_voice_form(&b));
}

/// W7 — THE BLOCKER-1 REGRESSION, and the reason this whole construction was redesigned.
///
/// The rejected v1 construction hashed `DOMAIN || 0x00 || kem_pk || sig_pk`, which is injective
/// over the CONCATENATION and not over the PAIR: every re-split of the same 3136 bytes is a
/// different `(kem, sig)` yielding an identical fingerprint — 3136 forgeries, one of which carries
/// an attacker-chosen signing key behind a byte-identical read-aloud value.
///
/// The length-prefixed construction closes it. This test walks EVERY split position.
#[test]
fn na0749_no_resplit_of_the_key_material_collides_with_the_true_pair() {
    let kem = kem_pk();
    let sig = sig_pk();
    let truth = identity_fingerprint_from_identity(&kem, &sig);
    let mut material = kem.clone();
    material.extend_from_slice(&sig);

    let mut collisions = Vec::new();
    for k in 0..=material.len() {
        if k == kem.len() {
            continue; // the true split
        }
        let fp = identity_fingerprint_from_identity(&material[..k], &material[k..]);
        if fp == truth {
            collisions.push(k);
        }
    }
    assert!(
        collisions.is_empty(),
        "the construction is ambiguous: {} re-splits collide with the true pair (first few: {:?})",
        collisions.len(),
        &collisions[..collisions.len().min(5)]
    );
}
