# AUDIT-20260104 Issue #12 — mk_order stale entry guard (DRAFT)

Goals: G4, G5

## Invariant
- mk_order must not contain key identifiers that are absent from mk_skipped.
- After take_mk_skipped consumes an entry, mk_order no longer references it.

## Tests (CI-exercised)
- `take_mk_skipped_removes_from_mk_order` (tools/refimpl/quantumshield_refimpl/src/qsp/state.rs)
- `take_mk_skipped_on_missing_does_not_corrupt_order` (tools/refimpl/quantumshield_refimpl/src/qsp/state.rs)

## Evidence
- `cargo test -q`
- Audit table row #12 marked FIXED (guarded)
