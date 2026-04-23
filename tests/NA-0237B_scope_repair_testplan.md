Status: Supporting
Owner: QSL governance
Last-Updated: 2026-04-23

# NA-0237B Scope Repair Testplan

Goals: G4

## Docs-only Validation Checkpoints

- `NEXT_ACTIONS.md` keeps `NA-0237B` as the sole `READY` item and adds the exact `qsp/state.rs` clippy-only scope line.
- `NEXT_ACTIONS.md` problem text records that the first local `NA-0237B` implementation attempt already proved the dependency remediation is bounded and locally valid, but required validation also depends on `tools/refimpl/quantumshield_refimpl/src/qsp/state.rs`.
- `docs/archive/testplans/NA-0237B_scope_repair_qsp_state_clippy_evidence.md` records the stopped clippy proof, the bounded dependency-remediation changed-path proof, and the governance-only no-runtime-semantics statement.
- `DECISIONS.md` records `D-0427` for the `NA-0237B` scope repair.
- `TRACEABILITY.md` records one `NA-0237B scope repair` entry pointing at the archive evidence doc.
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md` contains the matching Directive 347 entry.
- Local goal-lint passes via the accepted synthetic-event path with governance PR metadata.
- The markdown inventory commands and manual link-integrity runbook from `AGENTS.md` pass.
- The added-line leak-safe scan reports zero secret-like markers.

## References

- `NEXT_ACTIONS.md`
- `docs/archive/testplans/NA-0237B_scope_repair_qsp_state_clippy_evidence.md`
- `DECISIONS.md` (`D-0427`)
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
