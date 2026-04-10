Goals: G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-04-10

# NA-0231 Closeout Evidence Test Plan

## Scope

- Validate the docs/governance-only `NA-0231` closeout lane.
- Confirm the merged ML-DSA timing-oracle stale-on-main resolution is referenced truthfully from refreshed `main`.
- Confirm the queue transition marks `NA-0231` `DONE` and promotes exactly one direct successor, `NA-0232`.
- Confirm only the approved governance closeout surfaces changed.

## Docs-only validation checkpoints

- local goal-lint using a synthesized `GITHUB_EVENT_PATH`
- markdown inventory counts from the `AGENTS.md` manual runbook
- deterministic local-link existence check from `AGENTS.md`
- changed-path scope proof limited to:
  - `NEXT_ACTIONS.md`
  - `DECISIONS.md`
  - `TRACEABILITY.md`
  - `docs/archive/testplans/NA-0231_mldsa_65_timing_oracle_resolution_evidence.md`
  - `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
  - `tests/NA-0231_closeout_evidence_testplan.md`
- added-line leak-safe scan confirming no secret-like values were introduced
- no runtime battery

## Reference targets

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/archive/testplans/NA-0231_mldsa_65_timing_oracle_resolution_evidence.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `docs/audit/DOC-AUD-003_Security_Audit_Packet_Intake_and_Remediation_Plan_v0.1.0_DRAFT.md`
- `docs/audit/incoming/2026-04-09_security_batch/`
- `tests/NA-0231_closeout_evidence_testplan.md`

## Acceptance checkpoints

- the archive evidence doc records the merged PR #683 implementation state now present on refreshed `main`
- the queue transition marks `NA-0231` `DONE` and promotes exactly one successor `READY` item: `NA-0232`
- `DECISIONS.md` records `NA-0231` closeout/evidence and the rationale for promoting `QSC_HANDSHAKE_SEED` next
- `TRACEABILITY.md` records both `NA-0231 closeout/evidence` and `NA-0232 READY`
- the required rolling-journal entry is present at `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- the staged 8-file audit packet remains present and unchanged on `main`
- the closeout lane remains governance-only and introduces no runtime changes
