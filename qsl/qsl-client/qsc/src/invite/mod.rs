#![allow(unexpected_cfgs)]

//! NA-0681 (D616) — MESSAGING EPIC SLICE 2: the invite system's client half.
//!
//! ## Why this module exists as its own module (D616 F4, from OBS-CG)
//!
//! `transport/mod.rs` owns every socket in qsc and keeps doing so — the three invite
//! HTTP calls live there beside the existing relay calls, reusing `relay_http_client()`,
//! the CA-file handling and the NA-0663 TLS taxonomy. What lives HERE is everything that
//! is not a socket: the canonical encodings, the commitment, the signature, the invite
//! state machine, and the handshake envelope.
//!
//! ## The four canonical encodings (D616 §2a) — the whole point of this file
//!
//! The Slice-2 census found the cryptography settled and **every real risk to be an
//! encoding risk**, each with a failure mode that presents as an attack or as *success*
//! rather than as an honest error:
//!
//! 1. `wire_id` — the string rendering of `invite_id` and `cap`. `invite_id` **is** the
//!    relay route token, and the relay finds an invite slot by hashing that header
//!    *string*. A rendering mismatch means the slot lookup MISSES, the push is accepted
//!    as an ordinary message to an unknown route, and the relay answers **200 OK** while
//!    the handshake lands somewhere nobody reads. That is the only failure in this slice
//!    that returns success, so there is exactly ONE renderer and everything uses it.
//! 2. `cap_hash_hex` — the relay hashes the capability **as a string** and stores whatever
//!    the client uploaded without validating its shape, so a case or rendering mismatch
//!    comes back as `ERR_INVITE_CAP_INVALID`: an encoding bug wearing the costume of a
//!    wrong capability. Lowercase hex, over the UTF-8 bytes of the wire string, always.
//! 3. `canonical_bundle_bytes` — the commitment pre-image. An EXPLICIT byte layout built
//!    from the identity keys, **never** a re-serialization of the on-disk
//!    `IdentityPublicRecord`: `serde_json` offers no byte-stability contract, and the
//!    Phase-0 codec instrument proved that encoding drift does not have to raise a decode
//!    error — a one-character alphabet transposition produced a *clean decode to the wrong
//!    bytes*, which then fails the commitment and is indistinguishable from a
//!    key-substitution attack.
//! 4. `encode_payload` / the `QSLI-1-` codec — the signature's pre-image. Base64 is
//!    `URL_SAFE_NO_PAD`, which is what the whole crate already uses and what the relay's
//!    hand-rolled codec emits. **The commitment is computed over DECODED BYTES; comparing
//!    the base64 string the relay returned against the one that was sent is a defect** —
//!    the relay accepts padded input and emits unpadded, so a string comparison passes
//!    every same-implementation test and breaks against a relay that re-encodes.
//!
//! ## Time (D616 §2k, operator ruling)
//!
//! Every time decision takes the clock as a PARAMETER, in the `_at` idiom that
//! `vault/protection.rs` already established and documents as "the test-visible clock
//! seam". Negative controls force an expired or clamped invite by passing a value, never
//! by sleeping.
//!
//! ## Privacy (D616 §2k, binding)
//!
//! The ONLY relay-visible timestamp in v1 is the invite expiry, which is unavoidable
//! because the relay stores it. That is why `canonical_bundle_bytes` carries no
//! `created_at`: those bytes are uploaded to the relay, so a creation timestamp would be
//! a second relay-visible timestamp.

use super::*;
use sha2::Sha256;

// ---------------------------------------------------------------------------
// Domain separation and format constants
// ---------------------------------------------------------------------------

/// Domain separation for the identity commitment (DESIGN §2). Fixed-length ASCII, so the
/// concatenation with the bundle is unambiguous.
pub const DS_COMMIT: &str = "QSL.invite.identity-commitment.v1";
/// Domain separation for the invite payload signature (DESIGN §4).
pub const DS_SIG: &str = "QSL.invite.payload.v1";

/// Human/app-recognizable prefix. The "1" duplicates the version inside the signed payload.
pub const INVITE_CODE_PREFIX: &str = "QSLI-1-";

pub const INVITE_VER: u8 = 0x01;
pub const INVITE_TYPE_CONTACT: u8 = 0x01;
/// Version of the CANONICAL BUNDLE ENCODING — not of any stored struct. Minted here.
pub const BUNDLE_VER: u8 = 0x01;

/// `QSLH-1` handshake envelope (D616 §2e / F1). Versioned TLV that WRAPS the existing
/// fixed-length A1 frame; the frame itself is untouched, which is safe because
/// `hs_transcript_mac` binds the A1 BYTES and is therefore blind to an outer envelope.
pub const ENVELOPE_VER: u8 = 0x01;
pub const ENVELOPE_TYPE_INIT: u8 = 0x01;
pub const TAG_BUNDLE: u8 = 0x01;
pub const TAG_ROUTE_TOKEN: u8 = 0x02;
pub const TAG_A1: u8 = 0x03;

/// 16 bytes each, per DESIGN §1.
pub const ID_LEN: usize = 16;
pub const COMMIT_LEN: usize = 32;

// ---------------------------------------------------------------------------
// Failure taxonomy (D616 §2g)
//
// `&'static str` codes, NOT new `ErrorCode` variants -- the D599 precedent, where the TLS
// trust work added `relay_tls_untrusted` and the ca-file trio as codes and deliberately
// not as variants. Slice 2 owns the CODES; the Appendix-F wording is Slice 4's.
// ---------------------------------------------------------------------------

// Local, decided before any socket is opened.
pub const INVITE_MALFORMED: &str = "invite_malformed";
pub const INVITE_VERSION_NEWER: &str = "invite_version_newer";
pub const INVITE_TYPE_UNKNOWN: &str = "invite_type_unknown";
/// Expired by the LOCAL clock. The invite dies here, before any network attempt.
pub const INVITE_EXPIRED: &str = "invite_expired";
/// Client-side single-use. The arm that survives a hostile relay (I2).
pub const INVITE_ALREADY_REDEEMED: &str = "invite_already_redeemed";
pub const INVITE_REVOKED_LOCALLY: &str = "invite_revoked_locally";
pub const INVITE_SOFT_CAP_REACHED: &str = "invite_soft_cap_reached";

// Relay-reported.
pub const INVITE_NOT_FOUND: &str = "invite_not_found";
pub const INVITE_REVOKED: &str = "invite_revoked";
/// DELIBERATELY distinct from `INVITE_EXPIRED`: the relay clamps expiry against ITS clock
/// and cannot report the clamp at create, so a local clock saying "alive" while the relay
/// says "expired" is a NORMAL outcome. Collapsing the two would blame the user's clock for
/// a relay ceiling.
pub const INVITE_EXPIRED_AT_RELAY: &str = "invite_expired_at_relay";
pub const INVITE_ALREADY_USED: &str = "invite_already_used";
pub const INVITE_CAP_INVALID: &str = "invite_cap_invalid";
pub const INVITE_TICKET_INVALID: &str = "invite_ticket_invalid";
pub const INVITE_RATE_LIMITED: &str = "invite_rate_limited";
pub const INVITE_SLOT_CAP_FULL: &str = "invite_slot_cap_full";
pub const INVITE_TOO_LARGE: &str = "invite_too_large";
pub const INVITE_CREATE_FAILED: &str = "invite_create_failed";
pub const INVITE_REVOKE_INVALID: &str = "invite_revoke_invalid";
/// NA-0755 v2 (`D-1397`) — `invite_clear` refused because the record is not `Creating`.
///
/// ⚠ TWO LAYERS, and they are deliberately different strings (SR-15 **M-8**). This const keeps
/// the module's prefixed VALUE form, which is what the facade's value-shape census scrapes; the
/// facade re-mints the short wire discriminant `"clear_refused"`, because **zero of the existing
/// 38 wire codes carry an `invite_` prefix** and the desktop documents that set as "the
/// protocol's to change, not the desktop's".
pub const INVITE_CLEAR_REFUSED: &str = "invite_clear_refused";

/// The two security-relevant failures. DISTINCT from each other and from everything else,
/// because their causes are different -- substituted KEYS versus tampered invite FIELDS --
/// and the user needs to be told that someone may be interfering either way (DESIGN §6).
/// Severity is accent, never red: red stays reserved for vault-loss.
pub const INVITE_COMMITMENT_MISMATCH: &str = "invite_commitment_mismatch";
pub const INVITE_SIGNATURE_INVALID: &str = "invite_signature_invalid";

// Handshake envelope.
pub const HANDSHAKE_ENVELOPE_MALFORMED: &str = "handshake_envelope_malformed";
pub const HANDSHAKE_ENVELOPE_VERSION_NEWER: &str = "handshake_envelope_version_newer";

// ---------------------------------------------------------------------------
// §2k — the clock seam
// ---------------------------------------------------------------------------

/// Wall-clock seconds. The ONE accessor this module uses; every decision that depends on
/// it takes the value as a parameter through an `_at` entry point, so tests force the
/// outcome deterministically instead of sleeping.
///
/// Deliberately NOT re-plumbed through `dedup`, `qsp`, `attachments` or `vault::protection`,
/// each of which has its own private copy of this function and its own frozen tests. That
/// four-way consolidation is a named follow-up micro-lane (D616 §11), not this slice.
pub fn now_unix_s() -> u64 {
    // NA-0688 C1 (R4a): delegates to the ONE clock. See `crate::clock`.
    crate::clock::now_unix_s()
}

// ---------------------------------------------------------------------------
// §2a(1) — wire_id: the ONE string rendering of invite_id and cap
// ---------------------------------------------------------------------------

/// Lowercase hex, 32 characters, for a 16-byte value.
///
/// Chosen by measurement, not taste: `route_token_is_valid` requires 22–128 characters of
/// `[A-Za-z0-9_-]`, and 32 sits inside that with room (base64url would be 22, exactly at
/// the floor). Lowercase hex is also the shipped precedent -- `generate_route_token` is 32
/// random bytes through `hex_encode` -- and it matches the relay's own `{b:02x}` rendering,
/// which matters because the relay compares these values as strings.
///
/// USED FOR: the `X-QSL-Route-Token` header, `invite_id` in all three JSON bodies, and
/// `cap` in the redeem body. Same function every time. There is no second renderer.
pub fn wire_id(x: &[u8; ID_LEN]) -> String {
    hex_encode(x)
}

/// Parse a `wire_id` back to bytes. Strict: exactly 32 lowercase-hex characters.
pub fn wire_id_parse(s: &str) -> Result<[u8; ID_LEN], &'static str> {
    if s.len() != ID_LEN * 2
        || s.bytes()
            .any(|b| !b.is_ascii_lowercase() && !b.is_ascii_digit())
    {
        return Err(INVITE_MALFORMED);
    }
    let bytes = hex_decode(s).map_err(|_| INVITE_MALFORMED)?;
    let mut out = [0u8; ID_LEN];
    if bytes.len() != ID_LEN {
        return Err(INVITE_MALFORMED);
    }
    out.copy_from_slice(&bytes);
    Ok(out)
}

// ---------------------------------------------------------------------------
// §2a(2) — cap_hash: SHA-256 over the capability STRING's bytes
// ---------------------------------------------------------------------------

/// `SHA-256(wire_string.as_bytes())` rendered lowercase hex.
///
/// NOT over the raw 16 bytes, and NOT uppercase. This mirrors the relay's `sha256_hex`
/// exactly (`Sha256::digest(s.as_bytes())` then `{b:02x}`). The relay stores the uploaded
/// `cap_hash` verbatim without validating its shape, so any divergence here surfaces at
/// redemption as `ERR_INVITE_CAP_INVALID` -- indistinguishable, from the outside, from an
/// attacker presenting a wrong capability.
pub fn cap_hash_hex(cap_wire: &str) -> String {
    hex_encode(&Sha256::digest(cap_wire.as_bytes()))
}

// ---------------------------------------------------------------------------
// §2a(3) — canonical_bundle_bytes: the commitment pre-image
// ---------------------------------------------------------------------------

/// The byte-exact encoding of a published identity bundle:
///
/// ```text
/// bundle_ver   u8      = 0x01
/// kem_pk_len   u16 BE
/// kem_pk       bytes            (ML-KEM-768 public key)
/// sig_pk_len   u16 BE
/// sig_pk       bytes            (ML-DSA-65 public key)
/// ```
///
/// Built FROM the keys. Never by serializing `IdentityPublicRecord` (C11): that struct is
/// `deny_unknown_fields` and lives on disk, `serde_json` guarantees no byte-stable field
/// ordering across versions, and a re-serialization is precisely the drift that produces a
/// wrong-but-well-formed pre-image.
///
/// NO `created_at` and NO `key_ids`. `key_ids` is derivable from the two keys already in
/// the pre-image, so it adds nothing a commitment can use. `created_at` is forbidden: these
/// bytes are uploaded to the relay, and the only relay-visible timestamp in v1 is the
/// invite expiry.
pub fn canonical_bundle_bytes(kem_pk: &[u8], sig_pk: &[u8]) -> Result<Vec<u8>, &'static str> {
    if kem_pk.is_empty() || sig_pk.is_empty() {
        return Err(INVITE_MALFORMED);
    }
    if kem_pk.len() > u16::MAX as usize || sig_pk.len() > u16::MAX as usize {
        return Err(INVITE_MALFORMED);
    }
    let mut out = Vec::with_capacity(1 + 2 + kem_pk.len() + 2 + sig_pk.len());
    out.push(BUNDLE_VER);
    out.extend_from_slice(&(kem_pk.len() as u16).to_be_bytes());
    out.extend_from_slice(kem_pk);
    out.extend_from_slice(&(sig_pk.len() as u16).to_be_bytes());
    out.extend_from_slice(sig_pk);
    Ok(out)
}

/// The verifier's half of the same definition. Writer and verifier share this file so they
/// cannot drift; this function exists so the redeemer can recover the peer's keys from the
/// bytes it just authenticated, NOT so anyone can re-encode them.
pub fn canonical_bundle_parse(bytes: &[u8]) -> Result<(Vec<u8>, Vec<u8>), &'static str> {
    let mut off = 0usize;
    let ver = *bytes.first().ok_or(INVITE_MALFORMED)?;
    off += 1;
    if ver != BUNDLE_VER {
        return Err(INVITE_VERSION_NEWER);
    }
    let kem_pk = read_u16_prefixed(bytes, &mut off)?;
    let sig_pk = read_u16_prefixed(bytes, &mut off)?;
    if off != bytes.len() {
        // No silent extension riding v1.
        return Err(INVITE_MALFORMED);
    }
    if kem_pk.is_empty() || sig_pk.is_empty() {
        return Err(INVITE_MALFORMED);
    }
    Ok((kem_pk, sig_pk))
}

fn read_u16_prefixed(bytes: &[u8], off: &mut usize) -> Result<Vec<u8>, &'static str> {
    if *off + 2 > bytes.len() {
        return Err(INVITE_MALFORMED);
    }
    let len = u16::from_be_bytes([bytes[*off], bytes[*off + 1]]) as usize;
    *off += 2;
    if *off + len > bytes.len() {
        return Err(INVITE_MALFORMED);
    }
    let v = bytes[*off..*off + len].to_vec();
    *off += len;
    Ok(v)
}

// ---------------------------------------------------------------------------
// §2b — commitment and signature
// ---------------------------------------------------------------------------

/// `commit = SHA-256( DS_commit ‖ canonical_bundle_bytes )`.
///
/// SHA-256 rather than the crate's `Sha512`: `cap_hash` is relay-mandated SHA-256, so the
/// primitive arrives in this client regardless, and one hash for the whole invite system
/// beats two. It also yields exactly the 32 bytes DESIGN §1 reserves for `commit`.
pub fn commitment(canonical_bundle: &[u8]) -> [u8; COMMIT_LEN] {
    let mut h = Sha256::new();
    h.update(DS_COMMIT.as_bytes());
    h.update(canonical_bundle);
    let d = h.finalize();
    let mut out = [0u8; COMMIT_LEN];
    out.copy_from_slice(&d);
    out
}

/// The message the invite signature covers: `DS_sig ‖ payload_bytes`.
///
/// No circularity even though `payload_bytes` contains `commit`: the redeemer verifies the
/// commitment against the bundle FIRST, then verifies this signature over the payload it
/// already holds, using the key the commitment just authenticated.
pub fn sig_msg(payload_bytes: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(DS_SIG.len() + payload_bytes.len());
    out.extend_from_slice(DS_SIG.as_bytes());
    out.extend_from_slice(payload_bytes);
    out
}

// ---------------------------------------------------------------------------
// §2a(4) — the invite payload and the QSLI-1- code
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InvitePayload {
    pub ver: u8,
    pub typ: u8,
    pub invite_id: [u8; ID_LEN],
    pub expiry: u64,
    pub relay_ep: String,
    pub cap: [u8; ID_LEN],
    pub commit: [u8; COMMIT_LEN],
}

/// ```text
/// ver        u8      = 0x01
/// type       u8      = 0x01
/// invite_id  16 B
/// expiry     u64 BE
/// relay_ep   u16 BE len + UTF-8
/// cap        16 B
/// commit     32 B
/// ```
pub fn encode_payload(p: &InvitePayload) -> Result<Vec<u8>, &'static str> {
    let ep = p.relay_ep.as_bytes();
    if ep.is_empty() || ep.len() > u16::MAX as usize {
        return Err(INVITE_MALFORMED);
    }
    let mut out = Vec::with_capacity(1 + 1 + ID_LEN + 8 + 2 + ep.len() + ID_LEN + COMMIT_LEN);
    out.push(p.ver);
    out.push(p.typ);
    out.extend_from_slice(&p.invite_id);
    out.extend_from_slice(&p.expiry.to_be_bytes());
    out.extend_from_slice(&(ep.len() as u16).to_be_bytes());
    out.extend_from_slice(ep);
    out.extend_from_slice(&p.cap);
    out.extend_from_slice(&p.commit);
    Ok(out)
}

/// Strict, fail-closed, and ORDERED: the version is checked before anything else is
/// interpreted with v1 rules, then the type, then the fields, then trailing bytes.
pub fn decode_payload(bytes: &[u8]) -> Result<InvitePayload, &'static str> {
    let ver = *bytes.first().ok_or(INVITE_MALFORMED)?;
    if ver != INVITE_VER {
        // "A newer app is needed" -- distinct from malformed, because the user's action
        // differs and telling them "corrupt" when the truth is "old client" is a lie.
        return Err(INVITE_VERSION_NEWER);
    }
    let typ = *bytes.get(1).ok_or(INVITE_MALFORMED)?;
    if typ != INVITE_TYPE_CONTACT {
        return Err(INVITE_TYPE_UNKNOWN);
    }
    let mut off = 2usize;
    let invite_id = read_array::<ID_LEN>(bytes, &mut off)?;
    if off + 8 > bytes.len() {
        return Err(INVITE_MALFORMED);
    }
    let mut e = [0u8; 8];
    e.copy_from_slice(&bytes[off..off + 8]);
    let expiry = u64::from_be_bytes(e);
    off += 8;
    let ep_bytes = read_u16_prefixed(bytes, &mut off)?;
    let relay_ep = String::from_utf8(ep_bytes).map_err(|_| INVITE_MALFORMED)?;
    let cap = read_array::<ID_LEN>(bytes, &mut off)?;
    let commit = read_array::<COMMIT_LEN>(bytes, &mut off)?;
    if off != bytes.len() {
        // Trailing bytes are REJECTED: no silent extension riding v1 (DESIGN §1).
        return Err(INVITE_MALFORMED);
    }
    // The endpoint is validated by the SHIPPED policy -- https anywhere, http on loopback
    // only -- rather than DESIGN §1's flat "https:// enforced at parse". That is deliberate
    // and load-bearing: the two-party acceptance runs the real relay in-process on
    // 127.0.0.1 over plain HTTP, so an https-only parser would make the slice's own
    // end-to-end proof impossible to run without inventing a TLS rig. The fail-closed
    // property is preserved -- this is the same rule every other relay operation obeys.
    crate::adversarial::route::validate_relay_endpoint_url(&relay_ep)
        .map_err(|_| INVITE_MALFORMED)?;
    Ok(InvitePayload {
        ver,
        typ,
        invite_id,
        expiry,
        relay_ep,
        cap,
        commit,
    })
}

fn read_array<const N: usize>(bytes: &[u8], off: &mut usize) -> Result<[u8; N], &'static str> {
    if *off + N > bytes.len() {
        return Err(INVITE_MALFORMED);
    }
    let mut out = [0u8; N];
    out.copy_from_slice(&bytes[*off..*off + N]);
    *off += N;
    Ok(out)
}

/// `QSLI-1-<base64url(payload)>`, unpadded.
pub fn encode_invite_code(p: &InvitePayload) -> Result<String, &'static str> {
    let bytes = encode_payload(p)?;
    Ok(format!(
        "{}{}",
        INVITE_CODE_PREFIX,
        URL_SAFE_NO_PAD.encode(&bytes)
    ))
}

/// Parse a pasted code. Whitespace-trimmed (users paste from anywhere); everything else is
/// strict.
pub fn decode_invite_code(code: &str) -> Result<InvitePayload, &'static str> {
    let code = code.trim();
    let Some(b64) = code.strip_prefix(INVITE_CODE_PREFIX) else {
        // A `QSLI-` prefix with a different version number is a newer format, not garbage.
        if code.starts_with("QSLI-") {
            return Err(INVITE_VERSION_NEWER);
        }
        return Err(INVITE_MALFORMED);
    };
    if b64.is_empty() {
        return Err(INVITE_MALFORMED);
    }
    let bytes = URL_SAFE_NO_PAD.decode(b64).map_err(|_| INVITE_MALFORMED)?;
    decode_payload(&bytes)
}

// ---------------------------------------------------------------------------
// §2e — the QSLH-1 handshake envelope
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HandshakeEnvelope {
    /// The initiator's canonical bundle bytes -- the same definition the commitment uses.
    pub bundle: Vec<u8>,
    /// The initiator's inbox route token, so the responder can answer at all. This is the
    /// field whose absence from the shipped A1 frame is the entire reason this envelope
    /// exists (C5/C6).
    pub route_token: String,
    /// The existing A1 frame, VERBATIM. Never re-encoded, never inspected here.
    pub a1: Vec<u8>,
}

/// ```text
/// ver   u8 = 0x01
/// type  u8 = 0x01
/// records: tag u8 · len u16 BE · value
/// ```
///
/// Records are emitted in ascending tag order so the encoding is canonical.
pub fn encode_envelope(env: &HandshakeEnvelope) -> Result<Vec<u8>, &'static str> {
    let rt = env.route_token.as_bytes();
    if env.bundle.is_empty() || rt.is_empty() || env.a1.is_empty() {
        return Err(HANDSHAKE_ENVELOPE_MALFORMED);
    }
    if env.bundle.len() > u16::MAX as usize
        || rt.len() > u16::MAX as usize
        || env.a1.len() > u16::MAX as usize
    {
        return Err(HANDSHAKE_ENVELOPE_MALFORMED);
    }
    let mut out = Vec::with_capacity(2 + 9 + env.bundle.len() + rt.len() + env.a1.len());
    out.push(ENVELOPE_VER);
    out.push(ENVELOPE_TYPE_INIT);
    for (tag, val) in [
        (TAG_BUNDLE, env.bundle.as_slice()),
        (TAG_ROUTE_TOKEN, rt),
        (TAG_A1, env.a1.as_slice()),
    ] {
        out.push(tag);
        out.extend_from_slice(&(val.len() as u16).to_be_bytes());
        out.extend_from_slice(val);
    }
    Ok(out)
}

/// Unknown tags are REJECTED, not skipped. This is a security context and v1 does not
/// loosen: a future device manifest (P2) arrives as a new tag under a bumped `ver`, which
/// is exactly P1's "new types get their own risk review".
pub fn decode_envelope(bytes: &[u8]) -> Result<HandshakeEnvelope, &'static str> {
    let ver = *bytes.first().ok_or(HANDSHAKE_ENVELOPE_MALFORMED)?;
    if ver != ENVELOPE_VER {
        return Err(HANDSHAKE_ENVELOPE_VERSION_NEWER);
    }
    let typ = *bytes.get(1).ok_or(HANDSHAKE_ENVELOPE_MALFORMED)?;
    if typ != ENVELOPE_TYPE_INIT {
        return Err(HANDSHAKE_ENVELOPE_MALFORMED);
    }
    let mut off = 2usize;
    let (mut bundle, mut route_token, mut a1) = (None, None, None);
    while off < bytes.len() {
        let tag = bytes[off];
        off += 1;
        let val = read_u16_prefixed(bytes, &mut off)?;
        let slot = match tag {
            TAG_BUNDLE => &mut bundle,
            TAG_ROUTE_TOKEN => &mut route_token,
            TAG_A1 => &mut a1,
            _ => return Err(HANDSHAKE_ENVELOPE_MALFORMED),
        };
        if slot.is_some() {
            // A duplicate tag is a malformed frame, not a last-wins update.
            return Err(HANDSHAKE_ENVELOPE_MALFORMED);
        }
        *slot = Some(val);
    }
    let (Some(bundle), Some(rt), Some(a1)) = (bundle, route_token, a1) else {
        return Err(HANDSHAKE_ENVELOPE_MALFORMED);
    };
    if bundle.is_empty() || rt.is_empty() || a1.is_empty() {
        return Err(HANDSHAKE_ENVELOPE_MALFORMED);
    }
    let route_token = String::from_utf8(rt).map_err(|_| HANDSHAKE_ENVELOPE_MALFORMED)?;
    // The peer's route token must satisfy the same validator our own does, or we would
    // store something we can never send to.
    crate::adversarial::route::normalize_route_token(&route_token)
        .map_err(|_| HANDSHAKE_ENVELOPE_MALFORMED)?;
    Ok(HandshakeEnvelope {
        bundle,
        route_token,
        a1,
    })
}

// ---------------------------------------------------------------------------
// Verification: commitment THEN signature, each gating the next (DESIGN §4)
// ---------------------------------------------------------------------------

/// Verify a redeemed bundle against the invite code the redeemer holds.
///
/// ORDER IS THE CONTRACT, and it is the same idiom the handshake already uses one layer
/// down (B1 checks the cheap symmetric transcript MAC before the expensive ML-DSA
/// signature, then pins the identity):
///
/// 1. commitment: `SHA-256(DS_commit ‖ bundle) == payload.commit` -- a hostile relay
///    serving a SUBSTITUTE bundle fails here;
/// 2. signature: `invite_sig` verifies over `DS_sig ‖ payload_bytes` **using the ML-DSA key
///    the commitment just authenticated** -- a tampered `relay_ep`, `expiry`, `cap` or any
///    other code byte fails here, because the attacker cannot re-sign.
///
/// The two failures are returned as DISTINCT codes. A caller that cannot tell them apart
/// cannot tell "someone swapped the keys" from "someone edited the invite".
pub fn verify_redeemed_bundle(
    payload: &InvitePayload,
    payload_bytes: &[u8],
    bundle: &[u8],
    invite_sig: &[u8],
) -> Result<(Vec<u8>, Vec<u8>), &'static str> {
    // (1) commitment
    if commitment(bundle) != payload.commit {
        return Err(INVITE_COMMITMENT_MISMATCH);
    }
    // The bundle is authenticated now, so its keys can be trusted as the signer's.
    let (kem_pk, sig_pk) = canonical_bundle_parse(bundle)?;
    // (2) signature, gated by (1)
    let msg = sig_msg(payload_bytes);
    let c = StdCrypto;
    match c.verify(&sig_pk, &msg, invite_sig) {
        Ok(true) => Ok((kem_pk, sig_pk)),
        _ => Err(INVITE_SIGNATURE_INVALID),
    }
}

// ---------------------------------------------------------------------------
// §2h — persistence and the invite state machine
//
// Vault-backed JSON blobs under two secret keys, the same shape contacts already use. Two
// keys because the two sides have different lifetimes: what Alice CREATED and what Bob
// REDEEMED.
// ---------------------------------------------------------------------------

/// DESIGN §5.1: CREATED → ACTIVE → {REDEEMED, EXPIRED, REVOKED} → ARCHIVED.
///
/// `Creating` is not decoration. The record is written BEFORE the create call so a crash in
/// the network window leaves the slot KNOWN rather than orphaned at a relay with no local
/// trace -- the no-silent-loss discipline (epic §1) applied one slice early.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum InviteState {
    Creating,
    Active,
    Redeemed,
    Expired,
    Revoked,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct InviteRecord {
    /// `wire_id` form -- the same string used as the route token and in every JSON body.
    pub invite_id: String,
    pub cap: String,
    pub expiry: u64,
    pub relay_ep: String,
    pub state: InviteState,
    /// Returned exactly once by the relay at create.
    ///
    /// ⚠ STATED LIMITATION, not a bug to be papered over: a crash between the relay's 200
    /// and this field being persisted loses the token permanently, and the invite can then
    /// only expire, never be revoked. Writing the record BEFORE the call is what keeps the
    /// slot known in that window; recovering the token is not possible by construction,
    /// because the relay stores only its digest.
    #[serde(default)]
    pub revoke_token: Option<String>,
    #[serde(default)]
    pub created_unix: u64,
    /// NA-0755 v2 — the OPTIONAL local note: *who this invite is for*. Vault-stored beside the
    /// record and never sent anywhere; the relay sees the payload, and the payload has no room
    /// for it by construction (`encode_payload`).
    ///
    /// ⚠ **`None`, never `Some("")`.** The mint normalises at the boundary (`invite_create_at`)
    /// so no consumer has to special-case an empty string — one of them would forget.
    ///
    /// ⚠ **NOT `self_label`.** `self_label` selects WHICH IDENTITY mints; this names WHO IT IS
    /// FOR. They were `Option`s of the same type at four signatures, which is why the parameter
    /// is called `recipient_label` and sits LAST (SR-15 **B-2**).
    ///
    /// `#[serde(default)]` for the same reason `revoke_token` and `created_unix` carry it: a
    /// blob written before this lane loads with the field absent ⇒ `None`.
    #[serde(default)]
    pub label: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
struct InviteStore {
    #[serde(default)]
    invites: std::collections::BTreeMap<String, InviteRecord>,
}

/// Bob's side. Written BEFORE the redeem call and completed on receipt, because the
/// capability is burned the instant the relay answers and the relay clears the blobs in the
/// same transaction (DESIGN §5.2 "cached until the handshake completes"). Without this, a
/// dropped connection strands the redeemer on a consumed invite with no way back — the
/// bundle is gone server-side and a second redeem returns already-used.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RedemptionRecord {
    pub invite_id: String,
    /// True once the relay has answered. A record with this false and no bundle is a
    /// redemption that may or may not have consumed the slot — reported honestly rather
    /// than retried blindly.
    #[serde(default)]
    pub consumed: bool,
    #[serde(default)]
    pub bundle: Option<String>,
    #[serde(default)]
    pub invite_sig: Option<String>,
    #[serde(default)]
    pub ticket: Option<String>,
    /// Set once the handshake has been pushed. This is the CLIENT-SIDE single-use flag —
    /// the arm that survives a hostile relay (I2). A relay that serves the same invite
    /// twice still cannot make this client hand shake twice.
    #[serde(default)]
    pub handshake_done: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
struct RedemptionStore {
    #[serde(default)]
    redemptions: std::collections::BTreeMap<String, RedemptionRecord>,
}

fn invite_store_load() -> Result<InviteStore, &'static str> {
    match vault::secret_get(INVITES_SECRET_KEY) {
        Ok(None) => Ok(InviteStore::default()),
        Ok(Some(v)) => serde_json::from_str(&v).map_err(|_| INVITE_MALFORMED),
        Err(e) => Err(e),
    }
}

fn invite_store_save(st: &InviteStore) -> Result<(), &'static str> {
    let json = serde_json::to_string(st).map_err(|_| INVITE_MALFORMED)?;
    vault::secret_set(INVITES_SECRET_KEY, &json)
}

fn redemption_store_load() -> Result<RedemptionStore, &'static str> {
    match vault::secret_get(REDEMPTIONS_SECRET_KEY) {
        Ok(None) => Ok(RedemptionStore::default()),
        Ok(Some(v)) => serde_json::from_str(&v).map_err(|_| INVITE_MALFORMED),
        Err(e) => Err(e),
    }
}

fn redemption_store_save(st: &RedemptionStore) -> Result<(), &'static str> {
    let json = serde_json::to_string(st).map_err(|_| INVITE_MALFORMED)?;
    vault::secret_set(REDEMPTIONS_SECRET_KEY, &json)
}

/// DESIGN §8 Q2: a soft cap of 10 ACTIVE invites, enforced client-side. The relay's own cap
/// is 256 slots and bounds storage; this one bounds how many live doors a user can leave
/// open without noticing.
pub const ACTIVE_INVITE_SOFT_CAP: usize = 10;

/// DESIGN §8 Q1.
pub const DEFAULT_INVITE_TTL_SECS: u64 = 72 * 60 * 60;

/// Absorbs clock skew between this client and the relay so the SIGNED expiry equals the
/// STORED expiry (F3). At default config a client asking for its full 72 h is asking for
/// exactly the relay's ceiling, so any client-ahead skew clamps; five minutes out of
/// seventy-two hours buys the invariant.
pub const CLOCK_SKEW_MARGIN_SECS: u64 = 300;

/// Resolve the expiry to REQUEST, given the relay's advertised ceiling.
///
/// `advertised_max` of 0 means "not advertised" — an older relay, or one that does not offer
/// invites — and is treated as UNKNOWN, never as a ceiling of zero.
///
/// ⚠ The probe that supplies `advertised_max` is an OPTIMISATION. The CONTRACT is that a
/// clamp is a normal outcome and never an error, and that is tested with the probe disabled.
pub fn resolve_expiry(now: u64, desired_ttl: u64, advertised_max: u64) -> u64 {
    let ttl = if advertised_max == 0 {
        desired_ttl
    } else {
        desired_ttl.min(advertised_max)
    };
    now.saturating_add(ttl.saturating_sub(CLOCK_SKEW_MARGIN_SECS).max(1))
}

/// The `QSLH-1` RESPONSE envelope (type 0x02): the responder's route token plus the B1
/// frame, verbatim.
///
/// Why it must exist: the initiator reached the responder through a one-shot invite slot
/// whose ticket is burned by the very push that delivered A1. Without an address coming
/// back, the handshake dead-ends after B1 — the initiator has nowhere to send A2. This is
/// P3's "authenticated in-session announcement", which is how endpoints are meant to travel,
/// and it is why the initiator's token rides the request envelope and the responder's rides
/// this one. Neither is ever put in the invite code: the code is seen by whoever intercepts
/// it, and a permanent inbox token in it would be exactly the correlation surface I1 forbids.
pub const ENVELOPE_TYPE_RESP: u8 = 0x02;
pub const TAG_B1: u8 = 0x04;

pub fn encode_envelope_resp(self_route_token: &str, b1: &[u8]) -> Result<Vec<u8>, &'static str> {
    let rt = self_route_token.as_bytes();
    if rt.is_empty()
        || b1.is_empty()
        || rt.len() > u16::MAX as usize
        || b1.len() > u16::MAX as usize
    {
        return Err(HANDSHAKE_ENVELOPE_MALFORMED);
    }
    let mut out = Vec::with_capacity(2 + 6 + rt.len() + b1.len());
    out.push(ENVELOPE_VER);
    out.push(ENVELOPE_TYPE_RESP);
    for (tag, val) in [(TAG_ROUTE_TOKEN, rt), (TAG_B1, b1)] {
        out.push(tag);
        out.extend_from_slice(&(val.len() as u16).to_be_bytes());
        out.extend_from_slice(val);
    }
    Ok(out)
}

/// Returns `(responder_route_token, b1_bytes)`. Unknown and duplicate tags are REFUSED, the
/// same rule the request envelope uses.
pub fn decode_envelope_resp(bytes: &[u8]) -> Result<(String, Vec<u8>), &'static str> {
    let ver = *bytes.first().ok_or(HANDSHAKE_ENVELOPE_MALFORMED)?;
    if ver != ENVELOPE_VER {
        return Err(HANDSHAKE_ENVELOPE_VERSION_NEWER);
    }
    if *bytes.get(1).ok_or(HANDSHAKE_ENVELOPE_MALFORMED)? != ENVELOPE_TYPE_RESP {
        return Err(HANDSHAKE_ENVELOPE_MALFORMED);
    }
    let mut off = 2usize;
    let (mut rt, mut b1) = (None, None);
    while off < bytes.len() {
        let tag = bytes[off];
        off += 1;
        let val = read_u16_prefixed(bytes, &mut off)?;
        let slot = match tag {
            TAG_ROUTE_TOKEN => &mut rt,
            TAG_B1 => &mut b1,
            _ => return Err(HANDSHAKE_ENVELOPE_MALFORMED),
        };
        if slot.is_some() {
            return Err(HANDSHAKE_ENVELOPE_MALFORMED);
        }
        *slot = Some(val);
    }
    let (Some(rt), Some(b1)) = (rt, b1) else {
        return Err(HANDSHAKE_ENVELOPE_MALFORMED);
    };
    if rt.is_empty() || b1.is_empty() {
        return Err(HANDSHAKE_ENVELOPE_MALFORMED);
    }
    let rt = String::from_utf8(rt).map_err(|_| HANDSHAKE_ENVELOPE_MALFORMED)?;
    crate::adversarial::route::normalize_route_token(&rt)
        .map_err(|_| HANDSHAKE_ENVELOPE_MALFORMED)?;
    Ok((rt, b1))
}

// ---------------------------------------------------------------------------
// §2c / §2d — the entry points
//
// Every one takes the clock as a PARAMETER (§2k). The `_at` form is the real
// implementation; the wrapper supplies the real clock. Tests force expiry deterministically
// by passing a value, never by sleeping.
// ---------------------------------------------------------------------------

fn csprng_16() -> [u8; ID_LEN] {
    let mut b = [0u8; ID_LEN];
    OsRng.fill_bytes(&mut b);
    b
}

/// Alice: mint an invite.
///
/// Gates on CONFIGURATION, never on probe results (L3-1): an unlocked vault and a relay
/// endpoint. A failed reachability probe does not block creation.
/// ⚠ `recipient_label` is LAST and is NOT `self_label` (SR-15 **B-2**). Both are
/// `Option<&str>`, so the compiler cannot tell a transposition from the truth at any call
/// site; a transposed bare token (`"Mom"`, `"alice"`) passes `channel_label_ok` and, on a
/// profile with ZERO identities, is **silently adopted as the identity's own label**
/// (`identity/mod.rs:536-537`) — the ENG-0001 class. Position and name are the cheap half of
/// the cure; the transposition seal is the half that can fail.
pub fn invite_create(
    self_label: Option<&str>,
    relay: &str,
    ttl_secs: u64,
    recipient_label: Option<&str>,
) -> Result<String, &'static str> {
    invite_create_at(self_label, relay, ttl_secs, now_unix_s(), recipient_label)
}

pub fn invite_create_at(
    self_label: Option<&str>,
    relay: &str,
    ttl_secs: u64,
    now: u64,
    recipient_label: Option<&str>,
) -> Result<String, &'static str> {
    if !vault_unlocked() {
        return Err("vault_locked");
    }
    // NA-0755 v2: `absent = None, never ""` is enforced HERE, at the boundary, not at the
    // render. If the empty string reached the record every consumer would have to special-case
    // it and one of them would forget.
    let recipient_label = recipient_label
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .map(|l| l.to_string());
    // NA-0711: resolved BEFORE the network, and propagated (D647 A4 Δ36/Δ37).
    let self_label =
        crate::identity::identity_resolved_self_label(self_label).map_err(|e| e.as_str())?;
    let self_label = self_label.as_str();
    let relay_ep = crate::adversarial::route::normalize_relay_endpoint(relay)?;

    // DESIGN §8 Q2: a soft cap on LIVE doors, client-side. The relay's 256-slot cap bounds
    // storage; this one bounds how many active invites a user can forget about.
    let mut store = invite_store_load()?;
    let active = store
        .invites
        .values()
        .filter(|r| r.state == InviteState::Active && r.expiry > now)
        .count();
    if active >= ACTIVE_INVITE_SOFT_CAP {
        return Err(INVITE_SOFT_CAP_REACHED);
    }

    // F3: pre-clamp against the relay's advertised ceiling so the SIGNED expiry equals the
    // STORED expiry. The probe is an OPTIMISATION -- if it fails we proceed, because the
    // CONTRACT is that a clamp is normal and never an error.
    let advertised = match crate::transport::relay_server_info(&relay_ep) {
        Ok(crate::transport::RelayServerInfoOutcome::Reachable { doc, .. }) => {
            doc.invite_max_expiry_secs
        }
        _ => 0,
    };
    let expiry = resolve_expiry(now, ttl_secs, advertised);

    let kp = crate::identity::identity_self_kem_keypair(self_label)
        .map_err(|_| "identity_secret_unavailable")?;
    let bundle = canonical_bundle_bytes(&kp.kem_pk, &kp.sig_pk)?;
    let commit = commitment(&bundle);

    let invite_id = csprng_16();
    let cap = csprng_16();
    let invite_id_wire = wire_id(&invite_id);
    let cap_wire = wire_id(&cap);

    let payload = InvitePayload {
        ver: INVITE_VER,
        typ: INVITE_TYPE_CONTACT,
        invite_id,
        expiry,
        relay_ep: relay_ep.clone(),
        cap,
        commit,
    };
    let payload_bytes = encode_payload(&payload)?;
    let invite_sig = StdCrypto
        .sign(&kp.sig_sk, &sig_msg(&payload_bytes))
        .map_err(|_| INVITE_CREATE_FAILED)?;

    // COMMIT BEFORE THE NETWORK. A crash in the window below leaves the slot KNOWN rather
    // than orphaned at a relay with no local trace.
    store.invites.insert(
        invite_id_wire.clone(),
        InviteRecord {
            invite_id: invite_id_wire.clone(),
            cap: cap_wire.clone(),
            expiry,
            relay_ep: relay_ep.clone(),
            state: InviteState::Creating,
            revoke_token: None,
            created_unix: now,
            label: recipient_label,
        },
    );
    invite_store_save(&store)?;

    let revoke_token = crate::transport::invite_create_call(
        &relay_ep,
        &invite_id_wire,
        &cap_hash_hex(&cap_wire),
        expiry,
        &bundle,
        &invite_sig,
    )?;

    let mut store = invite_store_load()?;
    if let Some(rec) = store.invites.get_mut(&invite_id_wire) {
        rec.state = InviteState::Active;
        rec.revoke_token = Some(revoke_token);
    }
    invite_store_save(&store)?;

    encode_invite_code(&payload)
}

/// Alice: kill a slot. Recorded LOCALLY as well as at the relay, so a late redemption of a
/// revoked invite is refused client-side even if the relay lies (DESIGN §5.1).
pub fn invite_revoke(invite_id_wire: &str) -> Result<(), &'static str> {
    if !vault_unlocked() {
        return Err("vault_locked");
    }
    let mut store = invite_store_load()?;
    let rec = store
        .invites
        .get_mut(invite_id_wire)
        .ok_or(INVITE_NOT_FOUND)?;
    let relay_ep = rec.relay_ep.clone();
    let token = rec.revoke_token.clone().ok_or(INVITE_REVOKE_INVALID)?;
    // Local first: even if the relay call fails, this client will refuse the invite.
    rec.state = InviteState::Revoked;
    invite_store_save(&store)?;
    crate::transport::invite_revoke_call(&relay_ep, invite_id_wire, &token)
}

/// Alice: remove a local row that can never become actionable.
///
/// ⚠⚠ **WHAT `Creating` MEANS, AND WHAT IT DOES NOT** (SR-15 **B-1**, sustained). It does NOT
/// mean "the relay never confirmed". It means *the local transition to `Active` did not
/// complete*, and two measured paths leave it set with the relay half LANDED: the create call
/// erroring after the relay committed, and — the operator's own scenario — a vault failure in
/// the post-network window, where the `revoke_token` the relay just returned is a stack local
/// dropped unpersisted.
///
/// So the rationale for clearing is NOT "it is safe": **there is no token, so revoke is
/// impossible from this client at any time; Clear removes a local row that can never become
/// actionable.** If the relay did register the slot it stays open until it expires and cannot
/// be revoked from here — the record's own doc says recovering the token is not possible by
/// construction. The user-facing copy says exactly that, and never says "safe".
///
/// ⚠ The gate is the one EVERY sibling carries. The facade's pre-check is documented as no
/// substitute: it is point-in-time, and a caller outside that gate reopens the window.
///
/// ⚠ **A local tidy, not a repair — and the marker is the trace that survives it.** Deleting
/// the row removes the only local evidence the orphan existed, so the verb emits one marker
/// carrying the id, the prior state and the record's age. **The label NEVER rides it**: the
/// marker layer redacts by value SHAPE, and that redactor is measurably blind to a human name.
pub fn invite_clear(invite_id_wire: &str) -> Result<(), &'static str> {
    if !vault_unlocked() {
        return Err("vault_locked");
    }
    let mut store = invite_store_load()?;
    let rec = store.invites.get(invite_id_wire).ok_or(INVITE_NOT_FOUND)?;
    // Creating ONLY. Active has a token and Revoke is its control; Redeemed is a real contact's
    // provenance; Revoked and Expired are list hygiene, a separate decision this verb
    // deliberately does not take.
    //
    // ⚠ `InviteState::Expired` is NEVER STORED — zero constructors, and expiry is a read-time
    // overlay in the facade — so that arm is unreachable from a loaded record today.
    if rec.state != InviteState::Creating {
        return Err(INVITE_CLEAR_REFUSED);
    }
    let age_s = now_unix_s().saturating_sub(rec.created_unix).to_string();
    store.invites.remove(invite_id_wire);
    invite_store_save(&store)?;
    crate::output::emit_marker(
        "invite_cleared",
        None,
        &[
            ("invite_id", invite_id_wire),
            ("prior_state", "creating"),
            ("age_secs", age_s.as_str()),
        ],
    );
    Ok(())
}

pub fn invite_list() -> Result<Vec<InviteRecord>, &'static str> {
    if !vault_unlocked() {
        return Err("vault_locked");
    }
    Ok(invite_store_load()?.invites.values().cloned().collect())
}

/// Bob: redeem an invite and hand shake into the slot. DESIGN §5.2, in order.
pub fn invite_redeem(
    code: &str,
    alias: &str,
    self_label: Option<&str>,
) -> Result<String, &'static str> {
    invite_redeem_at(code, alias, self_label, now_unix_s())
}

pub fn invite_redeem_at(
    code: &str,
    alias: &str,
    self_label: Option<&str>,
    now: u64,
) -> Result<String, &'static str> {
    if !vault_unlocked() {
        return Err("vault_locked");
    }
    // NA-0711: resolved BEFORE the network, and propagated (D647 A4 Δ36/Δ37).
    let self_label =
        crate::identity::identity_resolved_self_label(self_label).map_err(|e| e.as_str())?;
    let self_label = self_label.as_str();
    // (1) PARSE -- entirely local.
    let payload = decode_invite_code(code)?;
    let payload_bytes = encode_payload(&payload)?;
    let invite_id_wire = wire_id(&payload.invite_id);
    let cap_wire = wire_id(&payload.cap);

    // (2) EXPIRY BY LOCAL CLOCK. The invite dies HERE, before any socket is opened.
    if payload.expiry <= now {
        return Err(INVITE_EXPIRED);
    }

    // (3) CLIENT-SIDE SINGLE USE, checked before the network. This is the arm that survives
    // a hostile relay (I2): a relay that serves the same invite twice still cannot make
    // this client hand shake twice.
    let mut rstore = redemption_store_load()?;
    if let Some(prev) = rstore.redemptions.get(&invite_id_wire) {
        if prev.handshake_done || prev.consumed {
            return Err(INVITE_ALREADY_REDEEMED);
        }
    }

    // (4) COMMIT BEFORE THE NETWORK.
    rstore.redemptions.insert(
        invite_id_wire.clone(),
        RedemptionRecord {
            invite_id: invite_id_wire.clone(),
            consumed: false,
            bundle: None,
            invite_sig: None,
            ticket: None,
            handshake_done: false,
        },
    );
    redemption_store_save(&rstore)?;

    // (5) REDEEM. The capability is burned the instant this returns.
    let (bundle, invite_sig, ticket) =
        crate::transport::invite_redeem_call(&payload.relay_ep, &invite_id_wire, &cap_wire)?;

    // (6) CACHE THE RESPONSE IMMEDIATELY. The relay cleared its copy in the same
    // transaction, so without this a drop here strands us on a consumed invite forever.
    let mut rstore = redemption_store_load()?;
    rstore.redemptions.insert(
        invite_id_wire.clone(),
        RedemptionRecord {
            invite_id: invite_id_wire.clone(),
            consumed: true,
            bundle: Some(URL_SAFE_NO_PAD.encode(&bundle)),
            invite_sig: Some(URL_SAFE_NO_PAD.encode(&invite_sig)),
            ticket: Some(ticket.clone()),
            handshake_done: false,
        },
    );
    redemption_store_save(&rstore)?;

    // (7) VERIFY: commitment THEN signature, each gating the next.
    let (peer_kem_pk, peer_sig_pk) =
        verify_redeemed_bundle(&payload, &payload_bytes, &bundle, &invite_sig)?;

    // (8) PENDING contact (I5). The route token is the invite SLOT -- the only address we
    // have -- and it is replaced by the peer's real token when the response envelope
    // arrives. Storing the slot is honest: it IS where A1 goes.
    let fp = crate::contacts::contacts_provision_from_invite(
        alias,
        &peer_kem_pk,
        &peer_sig_pk,
        &invite_id_wire,
        &payload.relay_ep,
        &invite_id_wire,
    )?;

    // (9) HANDSHAKE-INIT into the slot: our bundle + route token + the A1 frame VERBATIM,
    // presenting the one-shot ticket.
    let kp = crate::identity::identity_self_kem_keypair(self_label)
        .map_err(|_| "identity_secret_unavailable")?;
    let self_bundle = canonical_bundle_bytes(&kp.kem_pk, &kp.sig_pk)?;
    let self_inbox = crate::contacts::relay_self_inbox_route_token()?;
    crate::handshake::perform_handshake_init_with_route(
        self_label,
        alias,
        &payload.relay_ep,
        &invite_id_wire,
        HandshakeSuiteMode::LegacyCompat,
        crate::handshake::A1Delivery::InviteSlot {
            bundle: &self_bundle,
            self_route_token: &self_inbox,
            ticket: &ticket,
        },
    )?;

    let mut rstore = redemption_store_load()?;
    if let Some(r) = rstore.redemptions.get_mut(&invite_id_wire) {
        r.handshake_done = true;
    }
    redemption_store_save(&rstore)?;
    Ok(fp)
}

// ===========================================================================================
// NA-0742 (D-1378) LANE 2 — THE INVITE-FINISH SCAN + THE PRODUCER ACKS. ENG-0196's repair.
// ===========================================================================================

/// Frames requested per pull while scanning for the invite reply.
const FINISH_SCAN_BATCH: usize = 16;
/// ⚠ **THE ABSOLUTE BOUND ON PULLS PER RUN.** The scan cannot spin — leased rows are excluded
/// server-side, so each batch strictly shrinks the unleased set — but the shape is bounded by a
/// COUNTER rather than by that argument, because NA-0741 Finding 1 is exactly a case where a
/// re-poll's termination rested on a timing property nobody had named. This bound holds even
/// under pathological lease expiry mid-loop.
const FINISH_SCAN_PULLS_MAX: usize = 8;
/// `--max` becomes the TOTAL-SCAN clamp, not a per-pull size: `clamp(--max, 16, 128)`.
const FINISH_SCAN_TOTAL_MIN: usize = 16;
const FINISH_SCAN_TOTAL_MAX: usize = 128;

/// NA-0742: emit the producer-ack outcome, and NEVER fail the caller on a lost ack.
///
/// ⚠ `receive`'s posture, adopted deliberately: the state the frame caused is already durable, the
/// lease expires server-side, and the redelivery is handled. A repair that turned a lost ack into a
/// failed `invite finish` would be a worse defect than the one being fixed.
///
/// ⚠ `LegacyComplete` is marked with the existing `ack_legacy_complete` precedent rather than
/// reported as `acked=0` — a pre-durability relay has already delivered delete-on-pull, so nothing
/// was lost and nothing will redeliver. Collapsing it to a zero would report a healthy old server
/// as a failed ack.
fn emit_producer_ack(caller: &str, relay_ep: &str, route_token: &str, id: &str) {
    let ids = [id.to_string()];
    match crate::transport::producer_ack(relay_ep, route_token, &ids) {
        Ok(crate::transport::AckFlushOutcome::Acked(n)) => {
            let acked = n.to_string();
            crate::output::emit_marker(
                "producer_ack",
                None,
                &[("caller", caller), ("sent", "1"), ("acked", acked.as_str())],
            );
        }
        Ok(crate::transport::AckFlushOutcome::LegacyComplete) => {
            crate::output::emit_marker(
                "ack_legacy_complete",
                None,
                &[("caller", caller), ("count", "1")],
            );
        }
        Err(code) => {
            crate::output::emit_marker(
                "producer_ack",
                Some(code),
                &[("caller", caller), ("sent", "1"), ("acked", "0")],
            );
        }
    }
}

/// NA-0742: **THE DRAIN-FORWARD, CAPPED SCAN.** Pull batches of at most [`FINISH_SCAN_BATCH`],
/// classify every frame from its LEADING BYTES in wire order, and select the FIRST `InviteResp`.
///
/// ⚠⚠ **EVERYTHING NOT SELECTED IS LEFT COMPLETELY UNTOUCHED** — not acked, not consumed, not
/// decoded. It stays leased and its rightful consumer collects it one lease period later. This is
/// load-bearing rather than tidy: the relay's ack has **no lease-ownership check**, so acking a
/// frame this command did not consume would destroy another consumer's in-flight copy.
///
/// ⚠ `classify` is CONSUMED, never widened. Its own rule is *"THIS IS A MATCH, NOT AN
/// IDENTIFICATION"*: a frame that matches `01 02` and then fails to decode is a genuinely malformed
/// invite reply as far as any client can tell, and that is exactly the case where the existing
/// error is the CORRECT diagnosis. The false diagnosis this lane removes is the one produced by
/// frames that never matched at all.
///
/// STOPS on: the first `InviteResp`, a short batch (the unleased head is exhausted), or
/// [`FINISH_SCAN_PULLS_MAX`] pulls.
fn finish_scan_select_invite_resp(
    relay_ep: &str,
    self_inbox: &str,
    max: usize,
) -> Result<(Option<crate::InboxPullItem>, Vec<crate::InboxPullItem>), &'static str> {
    let total = max.clamp(FINISH_SCAN_TOTAL_MIN, FINISH_SCAN_TOTAL_MAX);
    let mut scanned = 0usize;
    let mut pulls = 0usize;
    let mut classes: Vec<&'static str> = Vec::new();
    let mut selected: Option<crate::InboxPullItem> = None;
    // NA-0768 E4: the Handshake-class frames THIS pull already leased, kept rather than dropped.
    let mut handshakes: Vec<crate::InboxPullItem> = Vec::new();
    let mut head_exhausted = false;

    'scan: while pulls < FINISH_SCAN_PULLS_MAX && scanned < total {
        let want = FINISH_SCAN_BATCH.min(total - scanned);
        let items = crate::transport::relay_inbox_pull(relay_ep, self_inbox, want)?;
        pulls += 1;
        let returned = items.len();
        for item in items {
            scanned += 1;
            let class = crate::frameclass::classify(&item.data);
            let name = class.name();
            if !classes.iter().any(|c| *c == name) {
                classes.push(name);
            }
            if class == crate::frameclass::FrameClass::Handshake {
                // ALREADY LEASED by the pull above (STOP 004 M4): keeping it costs no pull and
                // no lease. Dropping it is what strands the inviter (ENG-0250/ENG-0251).
                handshakes.push(item);
                continue;
            }
            if class == crate::frameclass::FrameClass::InviteResp && selected.is_none() {
                selected = Some(item);
                // NO `break 'scan` inside the batch -- see sec 2.1. The relay leases the whole
                // batch at pull time, so finishing the inspection costs nothing at the relay.
                continue;
            }
        }
        if selected.is_some() {
            break 'scan;
        }
        if returned < want {
            // A short batch means the relay had nothing more to give: the unleased head is
            // exhausted, and pulling again would only re-ask an empty mailbox.
            head_exhausted = true;
            break;
        }
    }

    // ⚠ `truncated` answers the operator's actual question: did I scan everything, or did I hit
    // the cap? It is true only when the scan ran out of BUDGET with the head still unexhausted.
    let truncated = selected.is_none() && !head_exhausted;
    let scanned_s = scanned.to_string();
    let pulls_s = pulls.to_string();
    // ⚠⚠ BARE CLASS NAMES ONLY, DEDUPED, FROM THE CLASSIFIER'S OWN FIXED FIVE-TOKEN VOCABULARY.
    // No digits, no counts, no ids inside this field. The redactor fires on `len() >= 24` PLUS a
    // digit, so a digit-free token list is redaction-safe AT ANY LENGTH by construction rather
    // than by staying short — lane 1 lost a whole marker diagnostic to exactly that rule. The
    // counts ride as their own short fields.
    let classes_s = classes.join(",");
    crate::output::emit_marker(
        "invite_scan_summary",
        None,
        &[
            ("scanned", scanned_s.as_str()),
            ("pulls", pulls_s.as_str()),
            ("truncated", if truncated { "true" } else { "false" }),
            (
                "selected",
                if selected.is_some() {
                    "invite_resp"
                } else {
                    "none"
                },
            ),
            ("classes", classes_s.as_str()),
        ],
    );
    Ok((selected, handshakes))
}

/// Alice: collect the handshake from her own invite slot and answer it.
///
/// The slot is deliberately ungated for pull -- its creator must be able to collect what
/// was left there. Client-side single use lives HERE: the first valid handshake per
/// `invite_id` is accepted and subsequent ones refused, independent of the relay (I2).
pub fn invite_accept(
    self_label: Option<&str>,
    invite_id_wire: &str,
    alias: &str,
    max: usize,
) -> Result<Option<String>, &'static str> {
    invite_accept_at(self_label, invite_id_wire, alias, max, now_unix_s())
}

pub fn invite_accept_at(
    self_label: Option<&str>,
    invite_id_wire: &str,
    alias: &str,
    max: usize,
    now: u64,
) -> Result<Option<String>, &'static str> {
    if !vault_unlocked() {
        return Err("vault_locked");
    }
    // NA-0711 (D647 A4 Δ36/Δ37, R238 §5.3): THIS PATH NEEDS ITS OWN PRE-PULL SITE. Its pull is
    // below, in THIS function, not in `perform_handshake_poll_with_tokens` -- a check placed only
    // there would leave the accept path (the path the defect was found on) uncovered. And it
    // RETURNS Err rather than emitting a marker and carrying on: a refusal that is downgraded to a
    // marker is not a refusal. Because it lands BEFORE the pull, it also lands before the slot is
    // marked Redeemed, so a mislabelled accept stays RETRYABLE -- a PARTIAL on ENG-0175 (R238 §2),
    // never a closure: every other failure mode still burns the slot.
    let self_label =
        crate::identity::identity_resolved_self_label(self_label).map_err(|e| e.as_str())?;
    let self_label = self_label.as_str();
    let store = invite_store_load()?;
    let rec = store.invites.get(invite_id_wire).ok_or(INVITE_NOT_FOUND)?;
    match rec.state {
        // Client-side single use, and it survives a relay that replays the slot.
        InviteState::Redeemed => return Err(INVITE_ALREADY_REDEEMED),
        InviteState::Revoked => return Err(INVITE_REVOKED_LOCALLY),
        _ => {}
    }
    if rec.expiry <= now {
        return Err(INVITE_EXPIRED);
    }
    let relay_ep = rec.relay_ep.clone();

    let items = crate::transport::relay_inbox_pull(&relay_ep, invite_id_wire, max)?;
    let Some(item) = items.into_iter().next() else {
        return Ok(None);
    };
    // NA-0742: needed after `item` is consumed below, to retire exactly this frame.
    // ⚠ **ACCEPT GETS AN ACK, NOT A SCAN**, and the asymmetry is deliberate rather than an
    // oversight. A foreign frame at the head of the invite SLOT still blocks `invite accept` with
    // the same shape ENG-0196 describes for finish — but the slot's route token IS the invite id,
    // so putting a frame there is capability-gated, which places it inside ENG-0142's remaining
    // ADVERSARIAL clause rather than ENG-0196's non-adversarial one. Named, not fixed.
    let consumed_id = item.id.clone();
    let env = decode_envelope(&item.data)?;
    let (peer_kem_pk, peer_sig_pk) = canonical_bundle_parse(&env.bundle)?;

    // Provision BEFORE processing A1. The responder pins the initiator's full identity
    // against this record, so if the envelope's bundle and A1's keys disagree the handshake
    // fails closed -- which is the integrity link between the envelope and the frame.
    let fp = crate::contacts::contacts_provision_from_invite(
        alias,
        &peer_kem_pk,
        &peer_sig_pk,
        &env.route_token,
        &relay_ep,
        invite_id_wire,
    )?;

    let self_inbox = crate::contacts::relay_self_inbox_route_token()?;
    let a1_item = crate::InboxPullItem {
        id: item.id,
        data: env.a1,
    };
    crate::handshake::perform_handshake_poll_with_tokens(
        // Already resolved above, pre-pull; the poll re-checks it idempotently.
        Some(self_label),
        alias,
        &relay_ep,
        "",
        &env.route_token,
        max,
        HandshakeSuiteMode::LegacyCompat,
        crate::handshake::HsPollSource::Provided(std::slice::from_ref(&a1_item)),
        // B1 rides back wrapped, carrying OUR route token, because the slot's ticket is
        // burned and the initiator otherwise has no address for us.
        Some(crate::handshake::HsReplyWrap {
            self_route_token: &self_inbox,
        }),
    )?;

    let mut store = invite_store_load()?;
    if let Some(r) = store.invites.get_mut(invite_id_wire) {
        r.state = InviteState::Redeemed;
    }
    invite_store_save(&store)?;
    // NA-0742: the A1 envelope this command consumed is retired from the INVITE SLOT — the mailbox
    // it was pulled from, never the ordinary inbox. The last effect of this frame is the slot
    // becoming `Redeemed` above; no push follows on this path.
    // NA-0770 (D-1411): unconditional — the ack mode this gated on no longer exists.
    {
        // GUARD 1: re-read the durable state this frame caused. Same stated limits as finish's.
        debug_assert!(
            invite_store_load()
                .ok()
                .and_then(|s| s.invites.get(invite_id_wire).map(|r| r.state.clone()))
                == Some(InviteState::Redeemed),
            "NA-0742 guard 1 (accept): the slot must read Redeemed before the A1 envelope that \
             redeemed it may be acked"
        );
        emit_producer_ack("accept", &relay_ep, invite_id_wire, &consumed_id);
    }
    Ok(Some(fp))
}

/// Bob: collect the wrapped B1 from his own inbox, learn the peer's real route token, and
/// let the existing initiator logic finish the handshake.
/// NA-0768 (`D-1409`): the poll's TWO durable-commit shapes, read together.
/// `.0` = a session exists for `peer`; `.1` = a pending record exists for `peer`.
/// ⚠ Errors PROPAGATE (B3): "unreadable" is not "absent", and the caller must not ack on a
/// witness it could not read.
fn witness_shapes(self_label: &str, peer: &str) -> Result<(bool, bool), &'static str> {
    let sess = match crate::protocol_state::qsp_session_load(peer) {
        Ok(v) => v.is_some(),
        Err(e) => return Err(e.as_str()),
    };
    let pend = match crate::handshake::hs_pending_present(self_label, peer) {
        Ok(v) => v,
        Err(e) => return Err(e.as_str()),
    };
    Ok((sess, pend))
}

pub fn invite_finish(
    self_label: Option<&str>,
    alias: &str,
    relay: &str,
    max: usize,
) -> Result<bool, &'static str> {
    if !vault_unlocked() {
        return Err("vault_locked");
    }
    // NA-0711: resolved BEFORE this path's own pull, and propagated (D647 A4 Δ36/Δ37).
    let self_label =
        crate::identity::identity_resolved_self_label(self_label).map_err(|e| e.as_str())?;
    let self_label = self_label.as_str();
    let relay_ep = crate::adversarial::route::normalize_relay_endpoint(relay)?;
    let self_inbox = crate::contacts::relay_self_inbox_route_token()?;
    // NA-0770 (D-1411): THE HEAD-ONLY LEGACY ARM IS RETIRED WITH THE MODE, and what it guarded
    // is worth recording rather than deleting silently.
    //
    // NA-0742 (D-1378) put a head-only pull here for `AckMode::Legacy` because a delete-on-pull
    // relay destroys everything a scan returns: scanning under Legacy would have converted a
    // one-frame loss into a 16x–128x amplification (`clamp(--max, 16, 128)`). That arm's own
    // comment called the gate "mechanical, not decorative", and it was right.
    //
    // ⚠ **WHAT IS LOST, NAMED (L5).** The retired arm's counterpart test held the ONLY EMPIRICAL
    // PIN on the `1` in the 1-to-16 floor — the measured claim that Legacy destroyed exactly one
    // frame here. After this lane that lower bound is ASSERTED FROM SOURCE, not measured.
    //
    // ⚠ **AND WHAT IS NOT FIXED.** The amplification hazard belonged to *the relay deleting on
    // pull*, not to this client's enum. A PRE-DURABILITY RELAY IGNORES `ack=lease` AND DELETES
    // ANYWAY (`transport::relay_inbox_pull_mode_inner`), and this scan then pulls up to 128
    // frames against it. That exposure is PRE-EXISTING — the Lease arm already ran at this
    // base's default — and is UNCHANGED by this lane. It is named here because retiring the
    // other arm removes the only configuration in which a user could ask for the head-only
    // pull instead: the best achievable floor for an informed user on such a relay rises from
    // 1 to 16, permanently. `ENG-0043`'s open "restate the old-server fallback story" is the
    // home for that story and this lane makes it MORE owed, not less.
    let (resp_item, hs_items) = finish_scan_select_invite_resp(&relay_ep, &self_inbox, max)?;
    for hs in hs_items {
        let hs_id = hs.id.clone();
        // ── THE CANDIDATE SET ──────────────────────────────────────────────────────────
        // ⚠ `Ok(None)` ONLY (RULING_NA0768_005 sec 3, from cold read 3's B3). An `Err` from
        // `qsp_session_load` is NEITHER "no session" NOR a candidate -- a corrupt or legacy
        // session blob would otherwise make that contact a PERMANENT fan-out candidate. It is
        // MARKED and skipped.
        let mut cands: Vec<String> = Vec::new();
        if matches!(crate::protocol_state::qsp_session_load(alias), Ok(None)) {
            cands.push(alias.to_string());
        }
        if let Ok(entries) = crate::contacts::contacts_list_entries() {
            for (label, _rec) in entries {
                if label == alias {
                    continue;
                }
                match crate::protocol_state::qsp_session_load(&label) {
                    Ok(None) => cands.push(label),
                    Ok(Some(_)) => {}
                    Err(_) => crate::output::emit_marker(
                        "invite_finish_hs_skip",
                        Some("session_unreadable"),
                        &[("peer", label.as_str())],
                    ),
                }
            }
        }
        let mut consumed_by: Option<String> = None;
        let mut offered_n: usize = 0;
        for cand in &cands {
            // ⚠ M8: a candidate whose route token will not resolve is SKIPPED, never offered
            // under an empty token -- `relay_inbox_push` would fail AFTER `hs_pending_store`
            // has already committed, wedging that contact with a B1 that never left.
            let peer_tok = match crate::contacts::relay_peer_route_token(cand) {
                Ok(t) => t,
                Err(code) => {
                    crate::output::emit_marker(
                        "invite_finish_hs_skip",
                        Some(code),
                        &[("peer", cand.as_str())],
                    );
                    continue;
                }
            };
            // ── THE WITNESS, BOTH COMMIT SHAPES (B1) ──────────────────────────────────
            // ⚠ An unreadable witness is NEVER "not present": it is marked and the frame is
            // NOT acked (the fail-safe direction -- an unacked frame is redelivered; a
            // wrongly-acked frame is destroyed).
            let before = match witness_shapes(self_label, cand) {
                Ok(v) => v,
                Err(code) => {
                    crate::output::emit_marker(
                        "invite_finish_hs_skip",
                        Some(code),
                        &[("peer", cand.as_str())],
                    );
                    continue;
                }
            };
            offered_n += 1;
            // ⚠ N6: `LegacyCompat` matches all three shipped invite-path call sites
            // (invite/mod.rs:1126, :1367, :1465). It is the caller's mode, not a downgrade.
            let offer = crate::handshake::perform_handshake_poll_with_tokens(
                Some(self_label),
                cand.as_str(),
                &relay_ep,
                &self_inbox,
                peer_tok.as_str(),
                max,
                HandshakeSuiteMode::LegacyCompat,
                // ⚠⚠ EVERY FAN-OUT OFFER IS SPECULATIVE, INCLUDING THE CALLER'S OWN ALIAS.
                // A first version carved the caller's alias out as "not speculative", on the
                // reading that the desktop invoked the verb FOR that contact. **MEASURED, THAT
                // RE-CREATES THE EXACT NOISE `RULING_006` sec 2(a) FORBIDS** (arm S5): the
                // desktop LOOPS `contact_list`, so every contact takes its turn as the caller,
                // and an ordinary foreign A1 then fires `identity_mismatch` with two
                // fingerprints on every beat. The alias came from an ITERATION, not from a
                // user asserting whose frame this is.
                // ⇒ THE LINE THAT HOLDS: a peer ASSERTED by a caller (`handshake poll --peer`,
                //   `invite_accept`, this verb's own RESP path) keeps the security marker,
                //   unchanged. A peer reached by ITERATION does not.
                crate::handshake::HsPollSource::ProvidedSpeculative(std::slice::from_ref(&hs)),
                None,
            );
            // ⚠ B2: the offer's error is MARKED and swallowed. An offer must never fail this
            // verb -- the redeemer's shipped path rides it -- but a swallow without a mark is
            // the ENG-0198 shape (rc 0, nothing said).
            if let Err(code) = offer {
                crate::output::emit_marker(
                    "invite_finish_hs_offer_error",
                    Some(code),
                    &[("peer", cand.as_str())],
                );
            }
            let after = match witness_shapes(self_label, cand) {
                Ok(v) => v,
                Err(code) => {
                    crate::output::emit_marker(
                        "invite_finish_hs_skip",
                        Some(code),
                        &[("peer", cand.as_str())],
                    );
                    continue;
                }
            };
            // SESSION appeared (responder branch) OR PENDING appeared (no-pending branch,
            // which also pushed a B1). Either is a durable commit caused by THIS frame.
            if (!before.0 && after.0) || (!before.1 && after.1) {
                consumed_by = Some(cand.clone());
                break;
            }
        }
        let offered_s = offered_n.to_string();
        // ⚠ NA-0768 (`RULING_006` sec 3): `caller=` is an ALIAS and is redacted by SHAPE when
        // it is >= 24 chars and contains a digit -- and it gets NO exemption, because an alias
        // is user-typed and semantically sensitive however it is shaped.
        // ⚠⚠ **D2 (`RULING_007` sec 2): THE COMPANION DERIVES FROM THE CONTACT'S PINNED
        // FINGERPRINT, NEVER FROM THE ALIAS.** An 8-hex digest of the alias
        // (`contacts::route_token_hash8`) was built first and REFUSED: it is unsalted, unkeyed
        // and takes no install input, so cold read 4 inverted this lane's own two published
        // values (`56c5f992` -> `bravo`, `b6b1266e` -> `charlie`) from a 50-word dictionary,
        // with no access to any install and in milliseconds. `route_token_hash8` earns its name
        // on a HIGH-ENTROPY input -- **the property belongs to the INPUT, not to the helper** --
        // and an alias is short, user-typed and guessable. It re-admitted the alias in exactly
        // the shape `should_redact_value` cannot see.
        // The pin is the value the tree ALREADY prints IN FULL under the `fp` key, which
        // `should_redact_value` exempts BY NAME, so a PREFIX of it discloses strictly less than
        // the marker stream already carries on every honest handshake.
        // ⚠ **ABSENT, NOT SUBSTITUTED**, when the caller has no pin: an unpinned contact has no
        // stable non-alias identity, and inventing one is the thing D2 refused. A pin READ ERROR
        // is treated as "no pin" for this FIELD ONLY -- it is diagnostic bookkeeping and never a
        // gate; the gate's own `Err(_)` arm is untouched (D4).
        let caller_id: Option<String> = crate::identity::identity_read_pin(alias)
            .ok()
            .flatten()
            .map(|pin| pin.chars().take(8).collect());
        let mut fields: Vec<(&str, &str)> = vec![("caller", alias)];
        if let Some(cid) = caller_id.as_deref() {
            fields.push(("caller_id", cid));
        }
        // ⚠ M7: the ACTUAL fan-out width, not a constant.
        fields.push(("offered", offered_s.as_str()));
        fields.push((
            "consumed",
            if consumed_by.is_some() { "true" } else { "false" },
        ));
        crate::output::emit_marker("invite_finish_hs_offer", None, &fields);
        if consumed_by.is_some() {
            emit_producer_ack("finish_hs", &relay_ep, &self_inbox, &hs_id);
        } else {
            // ⚠⚠ **`RULING_008` sec 3 -- THE MITIGATION FOR THE COST THAT RULING ACCEPTS.**
            // The fan-out cannot say WHY a frame went unclaimed: at the identity gate
            // (`handshake/mod.rs:2272`) there is no second fact separating "not the addressee"
            // from "the addressee whose identity changed" -- which is exactly why a speculative
            // offer is silent at all. So this marker does not claim to know. It makes the
            // CONDITION visible: a frame nobody took, which the relay redelivers every lease
            // until the 7-day TTL.
            // ⚠ **NO FINGERPRINT, NO IDENTITY, NO ALIAS, AND NO SESSION ID.** The ruling permits
            //   "at most a hash8 of the session id"; it is DELIBERATELY OMITTED, because getting
            //   one means reading the frame's session id in `invite/mod.rs`, and a session-id
            //   correlator outside the crypto module is REFUSED by `RULING_005` sec 2 (design 2's
            //   G4). **A PERMISSION IS NOT AN INSTRUCTION**, and taking it would re-open a closed
            //   decision to buy a field nothing yet consumes.
            // ⚠ ONE PER FRAME PER BEAT: it sits inside `for hs in hs_items`, on the arm where no
            //   candidate consumed, so its cardinality is frames-unclaimed, never candidates.
            //   `candidates=` reports the fan-out WIDTH that failed to place the frame.
            // ⚠ The redelivery-until-TTL it surfaces is `ENG-0198`'s family: cross-referenced,
            //   NOT repaired here.
            crate::output::emit_marker(
                "invite_finish_hs_unconsumed",
                None,
                &[("candidates", offered_s.as_str())],
            );
        }
    }
    let item = match resp_item {
        Some(item) => item,
        // ⚠ TODAY'S PATH, UNCHANGED: `Ok(false)` -> `invite_finish=none` -> rc 0. Not an
        // error, because it is not one — the reply has not arrived yet. What is new is
        // that the scan marker now names the classes actually seen, so "none" stops being
        // indistinguishable from "sixteen frames I could not use".
        None => return Ok(false),
    };
    // The id is needed AFTER `item` is consumed below, to retire exactly the frame this command
    // consumed and nothing else.
    let consumed_id = item.id.clone();
    let (peer_route_token, b1) = decode_envelope_resp(&item.data)?;
    // The peer's real address, announced in-session and authenticated by the handshake that
    // follows -- never by re-invite (P3).
    crate::contacts::contacts_set_route_token(alias, &peer_route_token)?;
    let b1_item = crate::InboxPullItem {
        id: item.id,
        data: b1,
    };
    crate::handshake::perform_handshake_poll_with_tokens(
        // Already resolved above, pre-pull; the poll re-checks it idempotently.
        Some(self_label),
        alias,
        &relay_ep,
        &self_inbox,
        &peer_route_token,
        max,
        HandshakeSuiteMode::LegacyCompat,
        crate::handshake::HsPollSource::Provided(std::slice::from_ref(&b1_item)),
        // A2 goes bare to the peer's now-known token: ordinary routing from here on.
        None,
    )?;
    // NA-0742: ⚠⚠ **THE ACK GOES AFTER THE CONSUMED FRAME'S LAST EFFECT — WHICH IS NOT THE
    // DURABLE COMMIT.** For finish the last effect is the poll returning `Ok`, because the poll
    // performs the outbound A2 push internally: a frame whose reply never left is not consumed.
    // NA-0770 (D-1411): unconditional — the ack mode this gated on no longer exists.
    {
        // GUARD 1: re-read the durable state THIS FRAME caused. ⚠ Its limits, stated in the same
        // breath: it cannot witness the outbound push, and `debug_assert!` is absent from release
        // builds. It is a development tripwire, not the guarantee. The guarantee is the ordering
        // above, which is a call-site property because `producer_ack` bypasses the receive loop's
        // mechanical ack-eligibility check.
        debug_assert!(
            crate::contacts::relay_peer_route_token(alias).as_deref()
                == Ok(peer_route_token.as_str()),
            "NA-0742 guard 1 (finish): the contact's stored route token must be the one the \
             consumed RESP carried before that frame may be acked"
        );
        emit_producer_ack("finish", &relay_ep, &self_inbox, &consumed_id);
    }
    Ok(true)
}
