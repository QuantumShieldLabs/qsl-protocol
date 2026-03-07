# Test Plan — Audit Issue #13 (SCKA monotonicity)

Goals: G4, G5

## Invariant
Non-monotonic peer advance IDs must be deterministically rejected and must not mutate state.

## Scope
Suite-2 SCKA monotonicity enforcement in `tools/refimpl/quantumshield_refimpl/src/suite2/scka.rs`.

## Tests
- `scka_rejects_nonmonotonic_epoch_deterministically_and_no_mutation` (suite2/scka.rs)
- `scka_accepts_next_monotonic_epoch_and_updates_state` (suite2/scka.rs)

## Evidence
- CI: public required checks (suite2 vectors + unit tests)
- PR: NA-0034 Issue #13 closure
