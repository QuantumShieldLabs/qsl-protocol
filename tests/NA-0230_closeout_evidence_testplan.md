Goals: G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-04-09

# NA-0230 Closeout Evidence Test Plan

## Scope

- validate the docs/governance-only `NA-0230` closeout lane;
- confirm the merged security-audit intake/remediation-plan canon is referenced truthfully from refreshed `main`; and
- confirm only the approved governance companions changed while promoting exactly one direct successor.

## Docs-only validation checkpoints

- local goal-lint using a synthesized `GITHUB_EVENT_PATH`
- markdown inventory counts from the `AGENTS.md` manual runbook
- deterministic local-link existence check from `AGENTS.md`
- changed-path scope proof limited to:
  - `NEXT_ACTIONS.md`
  - `DECISIONS.md`
  - `TRACEABILITY.md`
  - `docs/archive/testplans/NA-0230_security_audit_packet_intake_and_remediation_plan_evidence.md`
  - `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
  - `tests/NA-0230_closeout_evidence_testplan.md`
- added-line leak-safe scan confirming no secret-like values were introduced

## Reference targets

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/archive/testplans/NA-0230_security_audit_packet_intake_and_remediation_plan_evidence.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `docs/audit/DOC-AUD-003_Security_Audit_Packet_Intake_and_Remediation_Plan_v0.1.0_DRAFT.md`
- `docs/audit/incoming/2026-04-09_security_batch/`
- `tests/NA-0230_closeout_evidence_testplan.md`

## Acceptance checkpoints

- the archive evidence doc records the merged PR #681 implementation state now present on refreshed `main`
- the queue transition marks `NA-0230` `DONE` and promotes exactly one successor `READY` item: `NA-0231`
- `DECISIONS.md` records `NA-0230` closeout/evidence and the rationale for promoting the ML-DSA timing lane next
- `TRACEABILITY.md` records both `NA-0230 closeout/evidence` and `NA-0231 READY`
- the staged packet remains present and unchanged on `main`
- the required rolling-journal entry is present at `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- the closeout lane remains governance-only and introduces no runtime changes
