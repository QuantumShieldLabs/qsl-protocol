Goals: G4, G5

# NA-0234 Closeout Evidence Testplan

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-04-15

## Purpose

This docs-only testplan records the validation checkpoints for the `NA-0234` closeout and `NA-0235` successor promotion.

## Validation Checkpoints

- Local goal-lint passes for the governance-only closeout PR.
- Manual markdown link-integrity runbook from `AGENTS.md` reports `TOTAL_MISSING 0`.
- Docs inventory commands report both root and recursive markdown counts for `tests/` and `docs/`.
- Added-line leak-safe scan reports no sensitive values, no `v1-path pattern` matches, and no `hex32plus pattern` matches.
- `gh pr diff <PR> --name-only` shows only the six directive-authorized governance paths.

## Evidence References

- Archive evidence doc: `docs/archive/testplans/NA-0234_vault_read_path_kdf_floor_format_acceptance_resolution_evidence.md`
- Queue transition: `NEXT_ACTIONS.md` marks `NA-0234` DONE and promotes `NA-0235` READY.
- Decision entry: `DECISIONS.md` `D-0409`.
- Traceability entries: `TRACEABILITY.md` `NA-0234 closeout/evidence` and `NA-0235 READY`.
- Rolling journal entry: `docs/ops/ROLLING_OPERATIONS_JOURNAL.md` entry for Directive 293.

## Acceptance Checkpoints

- The closeout is governance-only and does not modify qsc runtime, qsc runtime tests, qsc-desktop, qsl-server, qsl-attachments, `.github`, website/public-runtime, `Cargo.toml`, or `Cargo.lock`.
- The staged 8-file audit packet remains present and unchanged.
- Refreshed `main` after merge shows `NA-0234` DONE and `NA-0235` as the sole READY item.
