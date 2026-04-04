Goals: G4, G5

# NA-0219 Rolling Operations Journal and Audit Program Test Plan

## Scope

- validate the docs/governance-only `NA-0219` implementation;
- confirm the new rolling-journal procedure, template, promotion rule, and director-ready audit program are checked in under the allowed paths only; and
- confirm queue closeout remains pending and `NEXT_ACTIONS.md` is untouched.

## Docs-only validation checkpoints

- local goal-lint using a synthesized `GITHUB_EVENT_PATH`
- markdown inventory counts from the `AGENTS.md` manual runbook
- deterministic local-link existence check from `AGENTS.md`
- changed-path scope proof limited to the approved files
- added-line leak-safe scan confirming no secret-like values were introduced

## Reference targets

- `START_HERE.md`
- `AGENTS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/DOC-OPS-003_Rolling_Operations_Journal_Procedure_v0.1.0_DRAFT.md`
- `docs/ops/TEMPLATE_Rolling_Operations_Journal_v0.1.0.md`
- `docs/ops/DOC-OPS-004_Promotion_of_Recurring_Operational_Lessons_to_Canon_v0.1.0_DRAFT.md`
- `docs/audit/DOC-AUD-001_qsc_Director_Ready_Crypto_and_Code_Audit_Program_v0.1.0_DRAFT.md`

## Acceptance checkpoints

- the rolling-journal procedure states its supporting posture, required per-directive fields, update cadence, off-host publication expectation, and secret prohibition
- the template matches the procedure fields and uses placeholders only in its worked example
- the promotion-to-canon rule distinguishes journal-only observations from lessons that must become canon
- the director-ready audit program is supporting strategic guidance, names candidate future qsc audit/remediation areas, and states explicitly that it does not outrank `NEXT_ACTIONS.md`
- `AGENTS.md` requires recovered failures and operational lessons to be recorded continuously without weakening fail-closed boundaries
- `DECISIONS.md` and `TRACEABILITY.md` record this lane as implementation/evidence only, with closeout and queue promotion still pending
