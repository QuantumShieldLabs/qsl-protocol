Status: Supporting
Owner: QSL governance
Last-Updated: 2026-04-21

# NA-0237 Scope Repair Testplan

Goals: G4

## Docs-only validation checkpoints

- `NEXT_ACTIONS.md` keeps `NA-0237` as the sole `READY` item and adds only the two repaired scope lines:
  - `tools/refimpl/quantumshield_refimpl/src/qsp/state.rs` only if directly touched by the bounded clippy-only fix required to pass the lane's required validation
  - `tools/refimpl/quantumshield_refimpl/tests/**` only if directly touched by bounded KT verifier vectors/regressions
- `DECISIONS.md` records `D-0423` for the scope repair and explicitly states this PR is governance-only.
- `TRACEABILITY.md` contains the `NA-0237 scope repair` entry pointing to `docs/archive/testplans/NA-0237_scope_repair_qsp_state_clippy_and_refimpl_tests_evidence.md`.
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md` contains the matching Directive 336 entry.
- Local goal-lint passes via the accepted synthetic-event path with the governance PR body metadata.
- The markdown inventory commands and the manual link-integrity runbook from `AGENTS.md` pass.
- The added-line leak-safe scan reports zero secret-like markers.

## References

- `docs/archive/testplans/NA-0237_scope_repair_qsp_state_clippy_and_refimpl_tests_evidence.md`
- `DECISIONS.md` (`D-0423`)
- `TRACEABILITY.md`
- `NEXT_ACTIONS.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
