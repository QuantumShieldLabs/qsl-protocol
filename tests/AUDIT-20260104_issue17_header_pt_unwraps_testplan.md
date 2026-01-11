# AUDIT-20260104 Issue #17 — Multiple unwraps on header_pt (DRAFT)

Goals: G4, G5

## Invariant
Malformed header_pt MUST NOT panic, MUST deterministically reject, and MUST NOT mutate session state on reject.

## Tests (CI-exercised)
- header_pt_invalid_rejects_deterministically_and_no_state_mutation (tools/refimpl/quantumshield_refimpl/src/suite2/ratchet.rs:775)
- header_pt_invalid_does_not_panic (tools/refimpl/quantumshield_refimpl/src/suite2/ratchet.rs:794)

## Evidence
- PR #TBD
- CI checks green
