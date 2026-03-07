# AUDIT-20260104 Issue #16 — DoS via large collection deserialization (DRAFT)

Goals: G4, G5

## Invariant
restore_bytes MUST NOT allocate or loop based on unbounded attacker-controlled lengths. Oversized or truncated inputs MUST reject deterministically and MUST NOT mutate pre-existing state on reject.

## Tests (CI-exercised)
- restore_bytes_rejects_oversize_lengths_deterministically (tools/refimpl/quantumshield_refimpl/src/suite2/state.rs:333)
- restore_bytes_rejects_truncated_buffers_deterministically (tools/refimpl/quantumshield_refimpl/src/suite2/state.rs:355)

## Evidence
- PR #TBD
- CI checks green
