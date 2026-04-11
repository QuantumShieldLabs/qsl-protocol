Goals: G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-04-10

# NA-0233 Scope Repair Testplan

## Purpose

This docs-only testplan records the validation checkpoints for the `NA-0233` governance scope repair.

## Validation Checkpoints

- Local goal-lint passes for the governance-only scope-repair PR.
- Manual markdown link-integrity runbook from `AGENTS.md` reports `TOTAL_MISSING 0`.
- Docs inventory commands report both root and recursive markdown counts for `tests/` and `docs/`.
- Added-line leak-safe scan reports no sensitive values, no `v1-path pattern` hits, and no `hex32plus pattern` hits in added lines.
- `gh pr diff <PR> --name-only` shows only the six directive-authorized changed paths.

## Evidence References

- Archive evidence doc: `docs/archive/testplans/NA-0233_mockprovider_fixed_key_scope_repair_evidence.md`
- Repaired queue block: `NEXT_ACTIONS.md` `NA-0233 — MockProvider Fixed Vault Key Resolution`
- Decision entry: `DECISIONS.md` `D-0402`
- Traceability entry: `TRACEABILITY.md` `NA-0233 scope repair`
- Rolling journal entry: `docs/ops/ROLLING_OPERATIONS_JOURNAL.md` entry for Directive 281

## Acceptance Checkpoints

- The scope repair is governance-only and does not modify qsc runtime, runtime tests, qsc-desktop, qsl-server, qsl-attachments, `.github`, website/public-runtime, `Cargo.toml`, or `Cargo.lock`.
- Refreshed current-main contradiction proof remains the basis for the repaired queue scope.
- `NA-0233` remains the sole READY item after merge.
- The staged 8-file audit packet remains present and unchanged on refreshed `main`.
