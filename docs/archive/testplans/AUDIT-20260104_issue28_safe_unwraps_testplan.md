# AUDIT-20260104 Issue #28 — Redundant Safe Unwraps (Refimpl) Test Plan v1.0.0 DRAFT

Goals: G2, G3
Status: DRAFT
Date: 2026-01-17
Owner: QSL governance

## Scope
- Remove attacker-triggerable `unwrap()` / `expect()` from decode/parse surfaces in refimpl (QSP ProtocolMessage encode path).
- Preserve wire format and protocol semantics.

## Invariants
- Fail-closed: malformed/partial inputs return deterministic errors (no panics).
- Deterministic reject: repeated runs on identical inputs yield the same error identity.
- No mutation on reject: decode/encode paths must not mutate protocol state.

## Tests
- `protocol_message_encode_missing_pq_adv_rejects_deterministically_no_panic`
- `protocol_message_encode_missing_pq_ctxt_rejects_deterministically_no_panic`

## Evidence checklist
- Unit tests above pass in CI.
- Audit status table row #28 marked FIXED (guarded).
- DECISIONS and TRACEABILITY updated.
