Status: Supporting
Owner: QSL governance
Last-Updated: 2026-04-21

# NA-0237A Scope Repair Testplan

Goals: G4

## Docs-only validation checkpoints

- `NEXT_ACTIONS.md` keeps `NA-0237A` as the sole `READY` item and adds only the repaired clippy-only scope line:
  - `tools/refimpl/quantumshield_refimpl/src/qsp/state.rs` only if directly touched by the bounded clippy-only fix required to pass the lane’s required validation
- `NEXT_ACTIONS.md` problem text now records that the first local `NA-0237A` implementation attempt already proved the bounded send_commit seam is correct but still stopped on the required `qsp/state.rs` clippy gate.
- `DECISIONS.md` records `D-0425` for the `NA-0237A` scope repair and explicitly states this PR is governance-only.
- `TRACEABILITY.md` contains the `NA-0237A scope repair` entry pointing to `docs/archive/testplans/NA-0237A_scope_repair_qsp_state_clippy_evidence.md`.
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md` contains the matching Directive 342 entry.
- Local goal-lint passes via the accepted synthetic-event path with the governance PR body metadata.
- The markdown inventory commands and the manual link-integrity runbook from `AGENTS.md` pass.
- The added-line leak-safe scan reports zero secret-like markers.

## References

- `docs/archive/testplans/NA-0237A_scope_repair_qsp_state_clippy_evidence.md`
- `DECISIONS.md` (`D-0425`)
- `TRACEABILITY.md`
- `NEXT_ACTIONS.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
