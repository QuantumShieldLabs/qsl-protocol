Goals: G4, G5

# NA-0232 Closeout Evidence Testplan

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-04-10

## Purpose

This docs-only testplan records the validation checkpoints for the `NA-0232` closeout and `NA-0233` successor promotion.

## Validation Checkpoints

- Local goal-lint passes for the governance-only closeout PR.
- Manual markdown link-integrity runbook from `AGENTS.md` reports `TOTAL_MISSING 0`.
- Docs inventory commands report both root and recursive markdown counts for `tests/` and `docs/`.
- Added-line leak-safe scan reports no sensitive values, no v1-path evidence, and no hex32plus evidence.
- `gh pr diff <PR> --name-only` shows only the six directive-authorized paths.

## Evidence References

- Archive evidence doc: `docs/archive/testplans/NA-0232_qsc_handshake_seed_deterministic_rng_path_resolution_evidence.md`
- Queue transition: `NEXT_ACTIONS.md` marks `NA-0232` DONE and promotes `NA-0233` READY.
- Decision entry: `DECISIONS.md` `D-0401`.
- Traceability entries: `TRACEABILITY.md` `NA-0232 closeout/evidence` and `NA-0233 READY`.
- Rolling journal entry: `docs/ops/ROLLING_OPERATIONS_JOURNAL.md` entry for Directive 279.

## Acceptance Checkpoints

- The closeout is governance-only and does not modify qsc runtime, runtime tests, qsc-desktop, qsl-server, qsl-attachments, `.github`, website/public-runtime, `Cargo.toml`, or `Cargo.lock`.
- The staged 8-file audit packet remains present and unchanged.
- Refreshed main after merge shows `NA-0232` DONE and `NA-0233` as the sole READY item.
