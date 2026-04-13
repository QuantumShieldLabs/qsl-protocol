Goals: G4, G5

# NA-0233A Closeout Evidence Testplan

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-04-12

## Purpose

This docs-only testplan records the validation checkpoints for the `NA-0233A` closeout and the truthful restore of `NA-0233` as the sole READY item.

## Validation Checkpoints

- Local goal-lint passes for the governance-only closeout / queue-repair PR.
- Manual markdown link-integrity runbook from `AGENTS.md` reports `TOTAL_MISSING 0`.
- Docs inventory commands report both root and recursive markdown counts for `tests/` and `docs/`.
- Added-line leak-safe scan reports no sensitive values, no `v1-path pattern` matches, and no `hex32plus pattern` matches.
- `gh pr diff <PR> --name-only` shows only the six directive-authorized governance paths.

## Evidence References

- Archive evidence doc: `docs/archive/testplans/NA-0233A_qsc_pr_critical_path_ci_rebalance_evidence.md`
- Queue transition: `NEXT_ACTIONS.md` marks `NA-0233A` DONE and restores `NA-0233` READY.
- Decision entry: `DECISIONS.md` `D-0405`.
- Traceability entries: `TRACEABILITY.md` `NA-0233A closeout/evidence` and `NA-0233 restored READY`.
- Rolling journal entry: `docs/ops/ROLLING_OPERATIONS_JOURNAL.md` entry for Directive 287.

## Acceptance Checkpoints

- The closeout is governance-only and does not modify qsc runtime, runtime tests, qsc-desktop, qsl-server, qsl-attachments, `.github`, website/public-runtime, `Cargo.toml`, or `Cargo.lock`.
- Refreshed main proof for PR #690 remains the sole basis for closing `NA-0233A` and restoring `NA-0233` to READY.
- PR #688 remains OPEN and untouched in this directive.
- Refreshed main after merge shows `NA-0233A` DONE and `NA-0233` as the sole READY item.
