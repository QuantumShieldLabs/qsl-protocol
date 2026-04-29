Goals: G4

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-04-27

# NA-0237D Closeout / Restore NA-0237C Testplan

## Purpose

This governance-only testplan records the validation checkpoints for closing `NA-0237D` from already-merged implementation state and restoring `NA-0237C` as the sole READY item.

## Validation Checkpoints

- Local goal-lint passes via the accepted synthetic-event path with the governance PR body metadata.
- Manual markdown link-integrity runbook from `AGENTS.md` reports `TOTAL_MISSING 0`.
- Docs inventory commands report both root and recursive markdown counts for `tests/` and `docs/`.
- Added-line leak-safe scan reports no sensitive values and zero `v1-path pattern` / `hex32plus pattern` hits.
- Refreshed merged-state proof shows PR `#717` merged normally and refreshed `main` carries the expected six-path bootstrap repair.
- Refreshed PR proof shows `#715` received a fresh PR-side suite on the same unchanged head, so the old workflow-self-repair bootstrap deadlock is gone.
- `gh pr diff <PR> --name-only` shows only the six directive-authorized governance paths.

## Evidence References

- Archive evidence doc: `docs/archive/testplans/NA-0237D_self_repair_bootstrap_evidence.md`
- Queue transition: `NEXT_ACTIONS.md` marks `NA-0237D` DONE and restores `NA-0237C` READY.
- Decision entry: `DECISIONS.md` `D-0431`.
- Traceability entries: `TRACEABILITY.md` `NA-0237D closeout/evidence` and `NA-0237C READY`.
- Rolling journal entry: `docs/ops/ROLLING_OPERATIONS_JOURNAL.md` entry for Directive 357.

## Acceptance Checkpoints

- The closeout is governance-only and does not modify `.github`, runtime/service code, website/public-runtime surfaces, `Cargo.toml`, `Cargo.lock`, qsl-server, or qsl-attachments.
- Refreshed merged-state proof shows PR `#717` merged unchanged and refreshed `main` carries the expected six-path bootstrap repair.
- Refreshed re-evaluation proof shows PR `#715` was re-run on the same unchanged head and the old bootstrap deadlock is gone, even though the branch remains red on its own merits.
- Refreshed `main` after merge shows `NA-0237D` DONE and `NA-0237C` as the sole READY item.
