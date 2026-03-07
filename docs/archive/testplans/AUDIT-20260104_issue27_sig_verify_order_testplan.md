# AUDIT-20260104 Issue #27 — Signature verification order in QSP handshake

Goals: G2, G3

## Invariant being protected
Handshake signature verification must only occur after cheap structural/semantic checks, so malformed inputs are rejected
before expensive verification is attempted.

## What must never happen
- Signature verification is attempted on malformed HS1/HS2 (bad protocol_version, suite_id, or signature length).
- Rejects become nondeterministic or mutate state on failure.

## Expected behavior
- Structural checks run before any ed25519/PQ signature verify call.
- Malformed HS1/HS2 rejects deterministically (fail-closed) before verify() is called.
- No state mutation on reject (inputs remain unchanged for reference-typed parameters).

## Tests
- `issue27_malformed_hs1_rejects_before_verify_and_is_deterministic` (tools/refimpl/quantumshield_refimpl/src/qsp/handshake.rs)
- `issue27_malformed_hs2_rejects_before_verify_and_is_deterministic` (tools/refimpl/quantumshield_refimpl/src/qsp/handshake.rs)
