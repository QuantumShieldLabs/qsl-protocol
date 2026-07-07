Goals: G4 (primary), supports G1, G2, G3, G5

Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-07-07

# NA-0617 — Attachment Single-Send-Per-Session Clarification + Resend Fix Test Plan

## Scope

Validation for the NA-0617 resolution of ENG-0002 (DOC-G5-005 §9 rank 6) under directive
QSL-DIR-2026-07-07-554 (D554). Client-side attachment journal reuse logic + tests + docs.
No protocol/wire/crypto/state-machine semantic change; no attachment
wire/descriptor/object/padding format change; no dependency/workflow change; no
qsl-attachments/qsl-server change.

## Required Markers

- NA0617_D1227_CONSUMED_OK
- NA0617_D1228_CONSUMED_OK
- NA0617_FRESH_STARTUP_PROOF_OK
- NA0617_D1229_ABSENT_BEFORE_IMPLEMENTATION_OK
- NA0617_DESIGN_LOCK_TWO_LAYER_SESSION_MODEL_OK
- NA0617_REJECT_TRIGGER_CONSUMED_SESSION_REUSE_OK
- NA0617_FIX_EXCLUDES_CONSUMED_SESSION_STATES_OK
- NA0617_DISTINCT_MULTI_SEND_SUCCEEDS_OK
- NA0617_SAME_FILE_RESEND_FRESH_SESSION_OK
- NA0617_RESUME_PRESERVED_OK
- NA0617_INFLIGHT_BLOCK_PRESERVED_OK
- NA0617_NEGATIVE_CONTROL_RESEND_FAILS_WITHOUT_FIX_OK
- NA0617_NO_NONCE_REUSE_FRESH_ENC_CTX_OK
- NA0617_NO_PROTOCOL_WIRE_CRYPTO_STATEMACHINE_CHANGE_OK
- NA0617_NO_ATTACHMENT_FORMAT_CHANGE_OK
- NA0617_REGRESSION_SUITES_PASS_OK
- NA0617_ENG0002_RESOLVED_OK
- NA0617_PRIVATE_MATERIAL_SCAN_OK

## Validation Plan (class-only)

1. Verify fresh operator startup proof (lane NA-0617) and main health; D-1227/D-1228
   consumed once each and Accepted; D-1229 absent before implementation.
2. Design-lock: confirm the two-layer session model (L1 service upload session, single
   object by design; L2 qsc client session, multi-send capable) and that
   `REJECT_QATTSVC_SESSION_STATE` originates in qsl-attachments on consumed-session reuse,
   not in qsc as a per-session cap.
3. Fix: `attachment_find_outbound_by_source` excludes `COMMITTED`/`ACCEPTED_BY_RELAY` (in
   addition to `PEER_CONFIRMED`) from reuse; single caller (the send path); resumable
   (`SESSION_CREATED`/`UPLOADING`) and in-flight (`AWAITING_CONFIRMATION`) states preserved.
4. Tests (`na_0617_attachment_single_send_per_session.rs`, real service in-process):
   distinct multi-send succeeds; same-file resend now succeeds without
   `REJECT_QATTSVC_SESSION_STATE`; interrupted upload resumes and commits; resend while
   awaiting confirmation is blocked as `attachment_send_inflight`.
5. Negative control: with the predicate reverted to `PEER_CONFIRMED`-only, the resend test
   fails with `code=REJECT_QATTSVC_SESSION_STATE` (class-only), proving the guard.
6. Regression: `attachment_streaming_na0197c` full suite passes (incl. resume and
   peer-confirm-after-persistence); qsc adversarial suite unaffected.
7. Build gates: `cargo fmt --check`, build, clippy, `cargo metadata --locked`, audit.
8. Private-material scan on all added/changed lines (no endpoints, tokens, capabilities,
   keys, plaintext, ciphertext bodies, seeds, raw logs, or private command lines).

## Result

`ATTACHMENT_RESEND_CONSUMED_SESSION_FAIL_CLOSED_TO_FRESH_SESSION`. ENG-0002 resolved
(fixed). No protocol/wire/crypto/state-machine or attachment-format change; no
dependency/workflow change.
