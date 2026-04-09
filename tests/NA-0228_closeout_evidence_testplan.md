Goals: G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-04-08

# NA-0228 Closeout Evidence Test Plan

## Scope

- validate the docs/governance-only `NA-0228` closeout lane;
- confirm the merged TUI command residual shell / dispatch decomposition is referenced truthfully from refreshed `main`; and
- confirm only the approved governance companions changed while promoting exactly one direct successor.

## Docs-only validation checkpoints

- local goal-lint using a synthesized `GITHUB_EVENT_PATH`
- markdown inventory counts from the `AGENTS.md` manual runbook
- deterministic local-link existence check from `AGENTS.md`
- changed-path scope proof limited to:
  - `NEXT_ACTIONS.md`
  - `DECISIONS.md`
  - `TRACEABILITY.md`
  - `docs/archive/testplans/NA-0228_qsc_tui_command_residual_shell_dispatch_decomposition_evidence.md`
  - `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
  - `tests/NA-0228_closeout_evidence_testplan.md`
- added-line leak-safe scan confirming no secret-like values were introduced

## Reference targets

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/archive/testplans/NA-0228_qsc_tui_command_residual_shell_dispatch_decomposition_evidence.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0228_closeout_evidence_testplan.md`

## Acceptance checkpoints

- the archive evidence doc records the merged PR #677 implementation state now present on refreshed `main`
- the queue transition marks `NA-0228` `DONE` and promotes exactly one successor `READY` item: `NA-0229`
- `DECISIONS.md` records `NA-0228` closeout/evidence and the rationale for promoting the residual state shell / ownership mediation decomposition next
- `TRACEABILITY.md` records both `NA-0228 closeout/evidence` and `NA-0229 READY`
- the required rolling-journal entry is present at `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- the closeout lane remains governance-only and introduces no runtime changes
