# AUDIT-20260104 Issue #25 — Inconsistent error types

Goals: G2, G3

## Invariant being protected
A single canonical error surface exists at the refimpl boundary so that Suite-2 and QSP failures are composable and deterministically testable.

## What must never happen
- Suite-2 and QSP produce different boundary error types requiring ad-hoc mapping per caller.
- User-visible reject strings omit a stable `reason_code=<CODE>` token.

## Expected behavior
- Boundary layer returns `RefimplError`.
- All rejects format as: `invalid request: reject: <CODE>; reason_code=<CODE>`.
- At least one stateful reject path proves no state mutation on reject.

## Tests
- `formats_suite2_reject_with_reason_code_token` (tools/refimpl/quantumshield_refimpl/src/refimpl_error.rs)
- `formats_qsp_codec_with_reason_code_token` (tools/refimpl/quantumshield_refimpl/src/refimpl_error.rs)
- `formats_qsp_ratchet_with_reason_code_token` (tools/refimpl/quantumshield_refimpl/src/refimpl_error.rs)
- `dh_ratchet_send_canon_rejects_deterministically_and_no_mutation` (tools/refimpl/quantumshield_refimpl/src/qsp/mod.rs)
