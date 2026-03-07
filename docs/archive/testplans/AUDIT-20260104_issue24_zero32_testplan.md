# AUDIT-20260104 Issue #24 — ZERO32 chain key sentinel guard

Goals: G2, G3

## Invariant being protected
All-zero 32-byte chain keys are an “unset/sentinel” representation (Suite-2 only) and MUST NOT be consumed as cryptographic keying
material.

## What must never happen
- Any code path uses an all-zero chain key (ck_*) as input to a KDF/AEAD/MAC to derive or use message keys.

## Expected behavior
- If a chain key would be consumed and is all-zero, the implementation MUST reject deterministically with a stable reason code that
  includes `reason_code=...` and MUST NOT mutate state.

## Tests
- `send_wire_rejects_unset_chainkey_deterministically_and_no_mutation` (tools/refimpl/quantumshield_refimpl/src/suite2/ratchet.rs)
  - Uses a zeroed chain key in send state.
  - Asserts deterministic reject with `reason_code=` and no mutation of state snapshot.
