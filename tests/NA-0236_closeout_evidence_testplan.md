Goals: G4

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-04-19

# NA-0236 Closeout Evidence Testplan

## Purpose

This docs-only testplan records the validation checkpoints for the `NA-0236` closeout, the durable archive evidence for the merged KT canon-closure lane, and the promotion of `NA-0237` as the sole READY item.

## Validation Checkpoints

- Local goal-lint passes for the governance-only closeout PR.
- Manual markdown link-integrity runbook from `AGENTS.md` reports `TOTAL_MISSING 0`.
- Docs inventory commands report both root and recursive markdown counts for `tests/` and `docs/`.
- Added-line leak-safe scan reports no sensitive values and zero sanitized path-marker or long-hex hits.
- One read-only refreshed-main dependency-audit proof on `qsl-protocol` `main` is green.
- `gh pr diff <PR> --name-only` shows only the six directive-authorized governance paths.

## Evidence References

- Archive evidence doc: `docs/archive/testplans/NA-0236_kt_serialization_profile_bundle_signature_closure_evidence.md`
- Queue transition: `NEXT_ACTIONS.md` marks `NA-0236` DONE and promotes `NA-0237` READY.
- Decision entry: `DECISIONS.md` `D-0422`.
- Traceability entries: `TRACEABILITY.md` `NA-0236 closeout/evidence` and `NA-0237 READY`.
- Rolling journal entry: `docs/ops/ROLLING_OPERATIONS_JOURNAL.md` entry for Directive 324.

## Acceptance Checkpoints

- The closeout is governance-only and does not modify runtime/source/test code, `.github`, website/public-runtime, `Cargo.toml`, `Cargo.lock`, qsl-server, or qsl-attachments.
- Refreshed merged-state proof shows PR `#705` merged normally and refreshed `main` carries the expected KT canon-closure doc plus the supporting schema/spec-closure updates.
- Refreshed `DOC-AUD-003` proof shows the KT prerequisite blocker is now closed on `main` and the next truthful lane is KT verifier implementation before `F06`.
- Refreshed `main` after merge shows `NA-0236` DONE and `NA-0237` as the sole READY item.
