Goals: G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-04-09

# NA-0229 Closeout Evidence Test Plan

## Scope

- validate the docs/governance-only `NA-0229` closeout lane;
- confirm the merged TUI state residual shell / ownership mediation decomposition is referenced truthfully from refreshed `main`; and
- confirm only the approved governance companions plus the staged audit packet changed while promoting exactly one direct successor.

## Docs-only validation checkpoints

- local goal-lint using a synthesized `GITHUB_EVENT_PATH`
- markdown inventory counts from the `AGENTS.md` manual runbook
- deterministic local-link existence check from `AGENTS.md`
- changed-path scope proof limited to:
  - `NEXT_ACTIONS.md`
  - `DECISIONS.md`
  - `TRACEABILITY.md`
  - `docs/archive/testplans/NA-0229_qsc_tui_state_residual_shell_ownership_mediation_decomposition_evidence.md`
  - `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
  - `docs/audit/incoming/2026-04-09_security_batch/**`
  - `tests/NA-0229_closeout_evidence_testplan.md`
- added-line leak-safe scan confirming no secret-like values were introduced
- host-side and repo-copy SHA-256 comparison for the staged 8-file audit packet

## Reference targets

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/archive/testplans/NA-0229_qsc_tui_state_residual_shell_ownership_mediation_decomposition_evidence.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `docs/audit/incoming/2026-04-09_security_batch/`
- `tests/NA-0229_closeout_evidence_testplan.md`

## Acceptance checkpoints

- the archive evidence doc records the merged PR #679 implementation state now present on refreshed `main`
- the staged audit packet exists in repo truth and its repo-copy SHA-256s match the host-side originals exactly
- the queue transition marks `NA-0229` `DONE` and promotes exactly one successor `READY` item: `NA-0230`
- `DECISIONS.md` records `NA-0229` closeout/evidence and the rationale for promoting the audit-packet intake lane next
- `TRACEABILITY.md` records both `NA-0229 closeout/evidence` and `NA-0230 READY`
- the required rolling-journal entry is present at `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- the closeout lane remains governance-only and introduces no runtime changes
