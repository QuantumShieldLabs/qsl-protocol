Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-30

# NA-0575 Closeout and NA-0576 Restoration Testplan

## Purpose

Record deterministic acceptance markers for closing NA-0575 after D-1140 and
restoring the exact D-1140-selected NA-0576 successor. This closeout does not
implement NA-0576.

## Required Markers

- NA0575_CLOSEOUT_IMPLEMENTATION_PR_MERGED_OK
- NA0575_CLOSEOUT_D1140_ACCEPTED_OK
- NA0575_CLOSEOUT_PUBLIC_SAFETY_GREEN_OK
- NA0575_CLOSEOUT_ADVISORIES_GREEN_OK
- NA0575_CLOSEOUT_NO_FAILED_REQUIRED_CHECKS_OK
- NA0575_CLOSEOUT_D1141_RESTORED_NA0576_OK
- NA0575_CLOSEOUT_SELECTED_SUCCESSOR_EXACT_OK
- NA0575_CLOSEOUT_NO_NA0576_IMPLEMENTATION_OK
- NA0575_CLOSEOUT_NO_REMOTE_ACTION_OK
- NA0575_CLOSEOUT_NO_QSL_SERVER_START_OK
- NA0575_CLOSEOUT_NO_QSC_SEND_RECEIVE_OK
- NA0575_CLOSEOUT_NO_WORKFLOW_DISPATCH_OK
- NA0575_CLOSEOUT_NO_RERUN_EXECUTED_OK
- NA0575_CLOSEOUT_NO_QSL_ATTACHMENTS_OK
- NA0575_CLOSEOUT_NO_PRIVATE_MATERIAL_PUBLISHED_OK
- NA0575_CLOSEOUT_NO_PUBLIC_READINESS_CLAIM_OK
- NA0575_CLOSEOUT_NO_PRODUCTION_READINESS_CLAIM_OK
- NA0575_CLOSEOUT_NO_SECURITY_COMPLETION_CLAIM_OK
- NA0575_CLOSEOUT_ONE_READY_INVARIANT_OK

## Evidence Mapping

- Implementation merge proof maps to PR #1423 and D-1140.
- Required-check proof maps to post-merge public-safety, advisories, and
  no-failed-required-check classification artifacts.
- Successor proof maps to the D-1140-selected NA-0576 start/bind proof
  completion authorization block restored in `NEXT_ACTIONS.md`.
- Boundary markers map to the closeout scope guard, private-material scan,
  overclaim scan, and repository diff proof.

## Expected Result

NA-0575 is DONE, NA-0576 is the sole READY item, D-1141 exists once, duplicate
decision IDs remain zero, no NA-0576 implementation occurs, and no qsl-server
start, remote action, qsc send/receive, workflow dispatch/rerun,
qsl-attachments action, private-material publication, or public/production
readiness claim occurs during closeout.
