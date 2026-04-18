Goals: G4

# NA-0235A Closeout Evidence Testplan

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-04-17

## Purpose

This docs-only testplan records the validation checkpoints for the `NA-0235A` closeout and the restoration of `NA-0235` as the sole READY item.

## Validation Checkpoints

- Local goal-lint passes for the governance-only closeout PR.
- Manual markdown link-integrity runbook from `AGENTS.md` reports `TOTAL_MISSING 0`.
- Docs inventory commands report both root and recursive markdown counts for `tests/` and `docs/`.
- Added-line leak-safe scan reports no sensitive values and zero sanitized path-marker or long-hex hits.
- Read-only refreshed-main `cargo audit --deny warnings` proof is green.
- `gh pr diff <PR> --name-only` shows only the six directive-authorized governance paths.

## Evidence References

- Archive evidence doc: `docs/archive/testplans/NA-0235A_runtime_dependency_advisory_remediation_evidence.md`
- Queue transition: `NEXT_ACTIONS.md` marks `NA-0235A` DONE and restores `NA-0235` READY.
- Decision entry: `DECISIONS.md` `D-0417`.
- Traceability entries: `TRACEABILITY.md` `NA-0235A closeout/evidence` and `NA-0235 restored READY`.
- Rolling journal entry: `docs/ops/ROLLING_OPERATIONS_JOURNAL.md` entry for Directive 311.

## Acceptance Checkpoints

- The closeout is governance-only and does not modify qsc runtime, qsc runtime tests, qsl-attachments, `.github`, website/public-runtime, `Cargo.toml`, or `Cargo.lock`.
- Refreshed merged-state proof shows qsl-attachments PR #31 and qsl-protocol PR #702 are durable on `main`.
- Refreshed qsl-protocol `main` is audit-green.
- Refreshed `main` after merge shows `NA-0235A` DONE and `NA-0235` as the sole READY item while PR `#695` remains OPEN and untouched.
