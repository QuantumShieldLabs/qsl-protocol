Goals: G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-04-07

# NA-0224 Closeout Evidence Test Plan

## Scope

- validate the docs/governance-only `NA-0224` closeout lane;
- confirm the merged modularization / file-size reduction plan refresh is referenced truthfully from refreshed `main`; and
- confirm only the approved governance companions changed while promoting exactly one direct successor.

## Docs-only validation checkpoints

- local goal-lint using a synthesized `GITHUB_EVENT_PATH`
- markdown inventory counts from the `AGENTS.md` manual runbook
- deterministic local-link existence check from `AGENTS.md`
- changed-path scope proof limited to:
  - `NEXT_ACTIONS.md`
  - `DECISIONS.md`
  - `TRACEABILITY.md`
  - `docs/archive/testplans/NA-0224_qsc_modularization_plan_refresh_evidence.md`
  - `tests/NA-0224_closeout_evidence_testplan.md`
- added-line leak-safe scan confirming no secret-like values were introduced

## Reference targets

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/design/DOC-QSC-011_qsc_Modularization_and_File_Size_Reduction_Plan_v0.1.0_DRAFT.md`
- `docs/archive/testplans/NA-0224_qsc_modularization_plan_refresh_evidence.md`
- `tests/NA-0224_closeout_evidence_testplan.md`

## Acceptance checkpoints

- the archive evidence doc records the merged PR #669 implementation state now present on refreshed `main`
- the queue transition marks `NA-0224` `DONE` and promotes exactly one successor `READY` item: `NA-0225`
- `DECISIONS.md` records `NA-0224` closeout/evidence and the rationale for promoting the TUI controller state / command-flow decomposition next
- `TRACEABILITY.md` records both `NA-0224 closeout/evidence` and `NA-0225 READY`
- the closeout lane remains governance-only and introduces no runtime changes
