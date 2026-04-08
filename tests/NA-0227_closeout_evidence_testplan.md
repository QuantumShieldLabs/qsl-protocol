Goals: G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-04-07

# NA-0227 Closeout Evidence Test Plan

## Scope

- validate the docs/governance-only `NA-0227` closeout lane;
- confirm the merged TUI state / poll-loop mediation decomposition is referenced truthfully from refreshed `main`; and
- confirm only the approved governance companions changed while promoting exactly one direct successor.

## Docs-only validation checkpoints

- local goal-lint using a synthesized `GITHUB_EVENT_PATH`
- markdown inventory counts from the `AGENTS.md` manual runbook
- deterministic local-link existence check from `AGENTS.md`
- changed-path scope proof limited to:
  - `NEXT_ACTIONS.md`
  - `DECISIONS.md`
  - `TRACEABILITY.md`
  - `docs/archive/testplans/NA-0227_qsc_tui_state_poll_loop_mediation_decomposition_evidence.md`
  - `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
  - `tests/NA-0227_closeout_evidence_testplan.md`
- added-line leak-safe scan confirming no secret-like values were introduced

## Reference targets

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/archive/testplans/NA-0227_qsc_tui_state_poll_loop_mediation_decomposition_evidence.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0227_closeout_evidence_testplan.md`

## Acceptance checkpoints

- the archive evidence doc records the merged PR #675 implementation state now present on refreshed `main`
- the queue transition marks `NA-0227` `DONE` and promotes exactly one successor `READY` item: `NA-0228`
- `DECISIONS.md` records `NA-0227` closeout/evidence and the rationale for promoting the residual command shell / dispatch decomposition next
- `TRACEABILITY.md` records both `NA-0227 closeout/evidence` and `NA-0228 READY`
- the required rolling-journal entry is present at `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- the closeout lane remains governance-only and introduces no runtime changes
