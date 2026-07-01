Status: Supporting
Owner: QSL governance
Last-Updated: 2026-07-01

# NA-0579 Closeout Restore NA-0580 Testplan

Goals: G1, G2, G3, G4, G5

## Scope

This closeout accepts D-1149 after the NA-0579 implementation PR merged, marks
NA-0579 DONE, records D-1150, and restores exactly one READY successor:
`NA-0580 -- QSL Remote qsl-server Expected Bind / Endpoint Alignment Harness`.

This closeout does not implement NA-0580 and does not run remote, runtime, qsc,
workflow-dispatch, or qsl-attachments actions.

## Required Markers

- NA0579_CLOSEOUT_IMPLEMENTATION_PR_MERGED_OK
- NA0579_CLOSEOUT_D1149_ACCEPTED_OK
- NA0579_CLOSEOUT_PUBLIC_SAFETY_GREEN_OK
- NA0579_CLOSEOUT_ADVISORIES_GREEN_OK
- NA0579_CLOSEOUT_NO_FAILED_REQUIRED_CHECKS_OK
- NA0579_CLOSEOUT_D1150_RESTORED_NA0580_OK
- NA0579_CLOSEOUT_SELECTED_SUCCESSOR_EXACT_OK
- NA0579_CLOSEOUT_NO_NA0580_IMPLEMENTATION_OK
- NA0579_CLOSEOUT_NO_REMOTE_ACTION_OK
- NA0579_CLOSEOUT_NO_QSL_SERVER_START_OK
- NA0579_CLOSEOUT_NO_QSC_SEND_RECEIVE_OK
- NA0579_CLOSEOUT_NO_WORKFLOW_DISPATCH_OK
- NA0579_CLOSEOUT_NO_QSL_ATTACHMENTS_OK
- NA0579_CLOSEOUT_NO_PRIVATE_MATERIAL_PUBLISHED_OK
- NA0579_CLOSEOUT_NO_PUBLIC_READINESS_CLAIM_OK
- NA0579_CLOSEOUT_NO_PRODUCTION_READINESS_CLAIM_OK
- NA0579_CLOSEOUT_NO_SECURITY_COMPLETION_CLAIM_OK
- NA0579_CLOSEOUT_ONE_READY_INVARIANT_OK

## Validation

- Exact five-path closeout scope guard:
  - `NEXT_ACTIONS.md`
  - `DECISIONS.md`
  - `TRACEABILITY.md`
  - `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
  - `tests/NA-0579_closeout_restore_na0580_testplan.md`
- Queue/decision proof:
  - D-1149 exists once and is Accepted.
  - D-1150 exists once and is Accepted.
  - D-1151 is absent.
  - NA-0579 is DONE.
  - READY_COUNT is 1.
  - READY is NA-0580.
- Marker proof for every required marker above.
- Link-check.
- Added-line/private-material scan.
- Overclaim scan.
- Docs/governance-only classifier.
- PR body preflight and goal-lint.
- Root and nested qsc fuzz cargo audits.
- Cargo metadata/fmt checks.
- qsc-adversarial shell syntax checks.
