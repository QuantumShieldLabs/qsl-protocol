Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-30

# NA-0574 Closeout and NA-0575 Restoration Testplan

## Purpose

Record deterministic acceptance markers for closing NA-0574 after D-1138 and
restoring the exact D-1138-selected NA-0575 successor. This closeout does not
implement NA-0575.

## Required Markers

- NA0574_CLOSEOUT_IMPLEMENTATION_PR_MERGED_OK
- NA0574_CLOSEOUT_D1138_ACCEPTED_OK
- NA0574_CLOSEOUT_PUBLIC_SAFETY_GREEN_OK
- NA0574_CLOSEOUT_ADVISORIES_GREEN_OK
- NA0574_CLOSEOUT_D1139_RESTORED_NA0575_OK
- NA0574_CLOSEOUT_NO_NA0575_IMPLEMENTATION_OK
- NA0574_CLOSEOUT_NO_REMOTE_ACTION_OK
- NA0574_CLOSEOUT_NO_QSL_SERVER_START_OK
- NA0574_CLOSEOUT_NO_QSC_SEND_RECEIVE_OK
- NA0574_CLOSEOUT_NO_WORKFLOW_DISPATCH_OK
- NA0574_CLOSEOUT_NO_PRIVATE_MATERIAL_PUBLISHED_OK
- NA0574_CLOSEOUT_ONE_READY_INVARIANT_OK

## Evidence Mapping

- Implementation merge proof maps to PR #1421 and D-1138.
- Required-check proof maps to post-merge public-safety, advisories,
  suite2-vectors, and no-failed-required-check classification artifacts.
- Successor proof maps to the D-1138-selected NA-0575 start/bind operator proof
  capture block restored in `NEXT_ACTIONS.md`.
- Boundary markers map to the closeout scope guard, private-material scan,
  overclaim scan, and repository diff proof.

## Expected Result

NA-0574 is DONE, NA-0575 is the sole READY item, D-1139 exists once, D-1140 is
absent, duplicate decision IDs remain zero, no NA-0575 implementation occurs,
and no qsl-server start, remote action, qsc send/receive, workflow dispatch, or
private-material publication occurs during closeout.
