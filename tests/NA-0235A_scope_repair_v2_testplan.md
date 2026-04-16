Goals: G4

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-04-16

# NA-0235A Scope Repair V2 Testplan

## Purpose

This docs-only testplan records the validation checkpoints for the `NA-0235A` governance scope repair v2.

## Validation Checkpoints

- Local goal-lint passes for the governance-only scope-repair v2 PR.
- Manual markdown link-integrity runbook from `AGENTS.md` reports `TOTAL_MISSING 0`.
- Docs inventory commands report both root and recursive markdown counts for `tests/` and `docs/`.
- Added-line leak-safe scan reports no sensitive values, no `v1-path pattern` hits, and no `hex32plus pattern` hits in added lines.
- `gh pr diff <PR> --name-only` shows only the six directive-authorized changed paths.

## Evidence References

- Archive evidence doc: `docs/archive/testplans/NA-0235A_scope_repair_tui_dependency_stack_evidence.md`
- Repaired queue block: `NEXT_ACTIONS.md` `NA-0235A — Runtime Dependency Advisory Remediation for Public-Safety Unblock`
- Decision entry: `DECISIONS.md` `D-0412`
- Traceability entry: `TRACEABILITY.md` `NA-0235A scope repair v2`
- Rolling journal entry: `docs/ops/ROLLING_OPERATIONS_JOURNAL.md` entry for Directive 302

## Acceptance Checkpoints

- The scope repair is governance-only and does not modify runtime code, runtime tests, manifests, lockfiles, `apps/qsl-tui/**`, `.github`, website/public-runtime, qsc-desktop, qsl-server, or qsl-attachments.
- Refreshed contradiction proof remains the basis for the repaired queue scope.
- `NA-0235A` remains the sole READY item after merge.
- PR `#695` remains open and untouched while the dependency remediation itself is deferred to the next implementation attempt.
