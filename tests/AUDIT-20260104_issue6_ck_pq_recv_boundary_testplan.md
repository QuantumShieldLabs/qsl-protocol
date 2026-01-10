# AUDIT-20260104 Issue #6 — ck_pq_recv Boundary Handling Test Plan (DRAFT)

Goals: G4, G5

## Invariant
- Boundary handling MUST update `ck_pq_recv` deterministically on success (per SCKA reseed) and MUST NOT mutate state on reject.

## Tests (CI-exercised)
- `boundary_reject_is_deterministic_and_no_state_mutation_on_bad_ct_len`
  - Location: `tools/refimpl/quantumshield_refimpl/src/suite2/ratchet.rs`
  - Asserts deterministic reject code and no mutation of `Suite2BoundaryState` on reject.
- `boundary_success_advances_ck_pq_recv_from_reseed`
  - Location: `tools/refimpl/quantumshield_refimpl/src/suite2/ratchet.rs`
  - Asserts `ck_pq_recv` is updated to the post-reseed chain step on success.

## Evidence
- CI: required checks green on the Issue #6 hardening PR.
- Audit status table updated: `docs/audit/AUDIT_CODE_ANALYSIS_STATUS_20260104.md`.
- Spec review reference: `tests/AUDIT-20260104_issue6_ck_pq_recv_boundary_spec_review.md`.
