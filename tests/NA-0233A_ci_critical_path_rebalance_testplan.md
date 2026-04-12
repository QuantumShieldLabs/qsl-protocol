Goals: G4

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-04-12

# NA-0233A CI Critical-Path Rebalance Testplan

## Purpose

This docs-only testplan records the validation checkpoints for the queue-truth repair that marks `NA-0233` `BLOCKED` and promotes `NA-0233A` as the sole `READY` item.

## Validation Checkpoints

- Local goal-lint passes for the governance-only queue-repair PR.
- Manual markdown link-integrity runbook from `AGENTS.md` reports `TOTAL_MISSING 0`.
- Docs inventory commands report both root and recursive markdown counts for `tests/` and `docs/`.
- Added-line leak-safe scan reports no sensitive values, no `v1-path pattern` hits, and no `hex32plus pattern` hits in added lines.
- `gh pr diff <PR> --name-only` shows only the six directive-authorized changed paths.

## Evidence References

- Archive evidence doc: `docs/archive/testplans/NA-0233_blocked_on_pr_critical_path_ci_evidence.md`
- Repaired queue block: `NEXT_ACTIONS.md` `NA-0233 — MockProvider Fixed Vault Key Resolution`
- Successor queue block: `NEXT_ACTIONS.md` `NA-0233A — qsc PR Critical-Path CI Rebalance`
- Decision entry: `DECISIONS.md` `D-0403`
- Traceability entries: `NA-0233 blocked-on-ci`; `NA-0233A READY`
- Rolling journal entry: `docs/ops/ROLLING_OPERATIONS_JOURNAL.md` entry for Directive 285

## Acceptance Checkpoints

- `NA-0233` is `BLOCKED` with a resume note pointing to PR #688.
- `NA-0233A` is the sole `READY` item after merge.
- PR #688 remains open and untouched.
- No runtime paths, runtime tests, workflows, branch protection, qsc-desktop, qsl-server, qsl-attachments, `.github`, `Cargo.toml`, or `Cargo.lock` change.
