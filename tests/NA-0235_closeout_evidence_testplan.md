Goals: G4

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-04-19

# NA-0235 Closeout Evidence Testplan

## Purpose

This docs-only testplan records the validation checkpoints for the `NA-0235` closeout, the post-incident branch-protection verification, and the promotion of `NA-0236` as the sole READY item.

## Validation Checkpoints

- Local goal-lint passes for the governance-only closeout PR.
- Manual markdown link-integrity runbook from `AGENTS.md` reports `TOTAL_MISSING 0`.
- Docs inventory commands report both root and recursive markdown counts for `tests/` and `docs/`.
- Added-line leak-safe scan reports no sensitive values and zero sanitized path-marker or long-hex hits.
- Read-only refreshed-main `cargo audit --deny warnings` proof is green.
- `gh pr diff <PR> --name-only` shows only the six directive-authorized governance paths.

## Evidence References

- Archive evidence doc: `docs/archive/testplans/NA-0235_pr_dependency_audit_gate_fullsuite_governance_evidence.md`
- Queue transition: `NEXT_ACTIONS.md` marks `NA-0235` DONE and promotes `NA-0236` READY.
- Decision entry: `DECISIONS.md` `D-0420`.
- Traceability entries: `TRACEABILITY.md` `NA-0235 closeout/evidence` and `NA-0236 READY`.
- Rolling journal entry: `docs/ops/ROLLING_OPERATIONS_JOURNAL.md` entry for Directive 321.

## Acceptance Checkpoints

- The closeout is governance-only and does not modify qsc runtime, qsc runtime tests, qsl-server, qsl-attachments, `.github`, website/public-runtime, `Cargo.toml`, or `Cargo.lock`.
- Refreshed merged-state proof shows PR `#695` merged normally and refreshed `main` carries exactly the expected six-path `NA-0235` result.
- Refreshed branch-protection proof shows `public-safety` still remains a required GitHub Actions protected check after the manual UI refresh.
- Refreshed `main` after merge shows `NA-0235` DONE and `NA-0236` as the sole READY item.
