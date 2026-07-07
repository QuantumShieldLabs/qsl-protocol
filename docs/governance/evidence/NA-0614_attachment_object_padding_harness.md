Goals: G5 (primary), supports G1, G2, G3, G4

Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-07-07

# NA-0614 — Mandatory Baseline Attachment-Object Padding

## Summary

NA-0614 implements DOC-G5-005 §9 rank 4 / ledger ENG-0010 under directive
QSL-DIR-2026-07-07-551 (D551): mandatory baseline attachment-object size padding so the
stored ciphertext object reveals only a size-ladder bucket to the attachment
service/network observer. The design is locked in DOC-G5-007 (which supersedes
DOC-G5-006's incorrect "client-only, no format change" M1 conclusion, corrected inline).

Result classification: `ATTACHMENT_OBJECT_PADDING_BASELINE_SHIPPED`.

Pre-release, no negotiation/versioning machinery is used: padding is the one-and-only
format, eliminating the metadata-downgrade surface by construction. This is not a
metadata-free/anonymity/unlinkability/traffic-analysis claim; access existence/timing
(ENG-0011) remain out of scope.

## Design (DOC-G5-007)

- Additive authenticated `content_len` (true length) distinct from `plaintext_len`
  (padded/encrypted length = a ladder bucket). Invariant `0 < content_len <=
  plaintext_len`.
- The receiver's exact `sum(decrypted) == plaintext_len` integrity check is
  **preserved** (now over the padded length); it additionally truncates the written
  file to `content_len`. Strengthened posture, not weakened.
- `content_len` is bound in the per-part AAD (`QATT-PART-V1|...|content_len|...`) and the
  confirm MAC (`QATT-CONFIRM-V1|...|content_len|...`) — tampering fails AEAD/confirm.
- Ladder (sender policy; receiver ladder-agnostic): powers of two 4 KiB..64 MiB plus a
  cap bucket == `ATTACHMENT_DEFAULT_MAX_FILE_SIZE` (100 MiB). The "too-big" gate uses
  `content_len`; padding never exceeds the max.
- `deny_unknown_fields` retained; `content_len` is a required v1 field from first
  release. Versioned-evolution convention documented (future changes bump `v`), not
  permissive parsing.

## Changed surface

- `qsl/qsl-client/qsc/src/store/mod.rs`: `AttachmentTransferRecord.content_len`
  (`#[serde(default)]` for forgiving pre-release persistence).
- `qsl/qsl-client/qsc/src/adversarial/payload.rs`: `AttachmentDescriptorPayload.content_len`
  (required, strict wire field); NA-0610 descriptor test templates updated.
- `qsl/qsl-client/qsc/src/attachments/mod.rs`: `ATTACHMENT_PAD_LADDER` +
  `attachment_pad_to_ladder`; sender pads to the bucket (zero-fill inside AEAD),
  `content_len` in AAD/confirm; receiver validates `content_len`, preserves the exact
  check, truncates to `content_len`; descriptor validation gains the `content_len`
  bound; the record↔descriptor equality predicate includes `content_len`.
- Tests: `attachment_streaming_na0197c.rs` and `adversarial_properties.rs` descriptor
  samples gain `content_len`; new co-located `na0614_padding_tests` module.

No Cargo/handshake/SCKA/crypto-suite-negotiation/message-plane/workflow/`.claude`
change; no new dependency.

## Validation

- `cargo fmt --check` OK; `cargo build` OK; `cargo clippy` clean; `cargo metadata
  --locked` OK; Cargo.toml/Cargo.lock unchanged.
- New `na0614_padding_tests` (6): ladder rounds up to buckets / rejects 0 and >max /
  output >= input and is a bucket; `attachment_validate_descriptor` accepts a valid
  padded descriptor and rejects `content_len == 0` and `content_len > plaintext_len`
  (`REJECT_ATT_DESC_INCONSISTENT_SHAPE`).
- All qsc binary unit tests: 38 pass.
- Regression suites pass: `attachments_contract_na0217h` (real send -> pad -> upload ->
  fetch -> decrypt -> truncate round-trip is byte-exact), `adversarial_properties`,
  `receive_no_mutation`, and the `attachment_streaming_na0197c` forged-descriptor
  negative test (fail-closed reject preserved with a well-formed-except-forgery
  descriptor).
- Fail-closed: `content_len` is AEAD/confirm-bound (tamper -> `REJECT_ATT_DECRYPT_AUTH`);
  `content_len > plaintext_len` / `== 0` reject at both the descriptor gate and the
  decrypt guard.

## Claim boundary

This lane changes the attachment descriptor/AAD/confirm format and sender/receiver
semantics; it does not touch the handshake/crypto-suite negotiation or the message
plane. No endpoint, port, token, capability, key, seed, plaintext, ciphertext body, or
raw private material is published. No public-readiness, production-readiness,
security-completion, crypto-complete, metadata-free, anonymity, unlinkability, or
traffic-analysis-resistant claim is made; padding reduces object-size metadata to a
bucket and the honest residual (size range, access existence/timing) remains
documented in DOC-G5-006/007.
