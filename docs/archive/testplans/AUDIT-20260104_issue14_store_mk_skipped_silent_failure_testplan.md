# AUDIT-20260104 Issue #14 — store_mk_skipped silent failure (DRAFT)

Goals: G4, G5

## Invariant
store_mk_skipped MUST NOT fail silently. Any failure to store must return a deterministic error and MUST NOT mutate state.

## Required behavior
- Deterministic reject on failure (same input -> same error).
- No state mutation on reject (mk_skipped + mk_order unchanged).
- Success path stores the key and indexes it in mk_order exactly once.

## Tests (CI-exercised)
- store_mk_skipped_rejects_deterministically_and_no_state_mutation_on_failure (tools/refimpl/quantumshield_refimpl/src/qsp/state.rs)
- store_mk_skipped_success_stores_and_indexes (tools/refimpl/quantumshield_refimpl/src/qsp/state.rs)

## Evidence
- PR #TBD
- CI checks green
