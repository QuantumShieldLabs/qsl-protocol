# AUDIT-20260104 Issue #15 — DH ratchet pn/ns overflow (DRAFT)

Goals: G4, G5

## Invariant
Counters MUST NOT wrap or truncate silently. On overflow, the DH ratchet MUST reject deterministically and MUST NOT mutate state.

## Required behavior
- Deterministic reject on overflow (same input -> same error).
- No state mutation on reject (pn/ns + ratchet state unchanged).
- Success path near boundary preserves pn semantics.

## Tests (CI-exercised)
- dh_ratchet_rejects_on_ns_overflow_deterministically_and_no_state_mutation (tools/refimpl/quantumshield_refimpl/src/qsp/ratchet.rs:522)
- dh_ratchet_success_near_boundary_does_not_corrupt_pn (tools/refimpl/quantumshield_refimpl/src/qsp/ratchet.rs:540)

## Evidence
- PR #TBD
- CI checks green
