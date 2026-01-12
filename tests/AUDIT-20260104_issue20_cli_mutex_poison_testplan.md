# Audit Issue #20 — CLI relay poisoned mutex handling (Test Plan)

- Goals: G4, G5
- Status: DRAFT
- Date: 2026-01-11

## Invariant
- Poisoned relay state lock must not panic.
- Rejection must be deterministic (stable error string).
- Failed lock attempts must not mutate relay state.

## Tests
- `relay_state_lock_poisoned_is_deterministic_and_no_mutation` (apps/qshield-cli/src/commands/relay.rs)
- `relay_state_lock_poisoned_returns_err` (apps/qshield-cli/src/commands/relay.rs)
