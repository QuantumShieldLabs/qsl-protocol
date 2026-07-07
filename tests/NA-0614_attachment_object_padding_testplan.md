Goals: G5 (primary), supports G1, G2, G3, G4

Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-07-07

# NA-0614 — Attachment Object Padding Test Plan

## Scope

Validation for the NA-0614 mandatory baseline attachment-object padding under directive
QSL-DIR-2026-07-07-551 (D551). Crypto/format-adjacent: descriptor schema + AEAD AAD +
confirm MAC + sender/receiver semantics. Design-locked in DOC-G5-007. No
dependency/handshake/message-plane/workflow change.

## Required Markers

- NA0614_D1222_CONSUMED_OK
- NA0614_D1223_CONSUMED_OK
- NA0614_FRESH_QWORK_PROOF_OK
- NA0614_D1224_ABSENT_BEFORE_IMPLEMENTATION_OK
- NA0614_DESIGN_LOCK_DOC_G5_007_OK
- NA0614_DOC_G5_006_CORRECTED_OK
- NA0614_CONTENT_LEN_ADDITIVE_EXACT_CHECK_PRESERVED_OK
- NA0614_CONTENT_LEN_AAD_CONFIRM_BOUND_OK
- NA0614_LADDER_DETERMINISTIC_VECTORS_OK
- NA0614_RECEIVER_LADDER_AGNOSTIC_OK
- NA0614_ROUND_TRIP_BYTE_EXACT_OK
- NA0614_FAILCLOSED_CONTENT_LEN_REJECTS_OK
- NA0614_DENY_UNKNOWN_FIELDS_RETAINED_OK
- NA0614_NO_CARGO_HANDSHAKE_MESSAGE_PLANE_CHANGE_OK
- NA0614_ALL_QSC_UNIT_TESTS_PASS_OK
- NA0614_REGRESSION_SUITES_PASS_OK
- NA0614_PRIVATE_MATERIAL_SCAN_OK
- NA0614_DESIGN_TENET_PERSISTED_OK

## Validation Plan (class-only)

1. Verify fresh operator qwork proof (lane NA-0614) and main health; D-1222/D-1223 once,
   D-1224 absent; sole READY NA-0614.
2. Design-lock: DOC-G5-007 authored (content_len semantics, ladder, AAD/confirm binding,
   fail-closed rules, versioned-evolution convention); DOC-G5-006 corrected inline.
3. Unit: `na0614_padding_tests` — ladder vectors (round-up, 0/over-max reject, output is
   a bucket >= input) and `attachment_validate_descriptor` accept/reject (content_len 0
   and > plaintext_len).
4. Integration: `attachments_contract_na0217h` round-trip byte-exact with padding;
   `attachment_streaming_na0197c` forged-descriptor negative fail-closed;
   `adversarial_properties` and `receive_no_mutation` unchanged.
5. cargo fmt/build/clippy/audit/metadata --locked green; Cargo unchanged; scope guard
   confirms no handshake/message-plane/Cargo/workflow/`.claude` change and that the exact
   check + deny_unknown_fields are preserved.
6. Persist the project design tenet to memory (closeout).

## Result

`ATTACHMENT_OBJECT_PADDING_BASELINE_SHIPPED`. Design: DOC-G5-007; feasibility
correction: DOC-G5-006. Evidence:
`docs/governance/evidence/NA-0614_attachment_object_padding_harness.md`.
