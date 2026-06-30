Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-30

# NA-0576 Closeout and NA-0577 Restoration Testplan

## Purpose

Record deterministic acceptance markers for closing NA-0576 after D-1142 and
D-1143, verifying the D504 amendment post-merge gate, and restoring the exact
NA-0577 successor. This closeout does not implement NA-0577.

## Required Markers

- NA0576_CLOSEOUT_D1142_ACCEPTED_OK
- NA0576_CLOSEOUT_D1143_ACCEPTED_OK
- NA0576_CLOSEOUT_AMENDMENT_PR_MERGED_OK
- NA0576_CLOSEOUT_PUBLIC_SAFETY_GREEN_OK
- NA0576_CLOSEOUT_ADVISORIES_GREEN_OK
- NA0576_CLOSEOUT_D1144_RESTORED_NA0577_OK
- NA0576_CLOSEOUT_NO_NA0577_IMPLEMENTATION_OK
- NA0576_CLOSEOUT_WAIT_WORK_POLICY_DURABLE_OK
- NA0576_CLOSEOUT_NO_REMOTE_ACTION_OK
- NA0576_CLOSEOUT_NO_QSL_SERVER_START_OK
- NA0576_CLOSEOUT_NO_QSC_SEND_RECEIVE_OK
- NA0576_CLOSEOUT_NO_WORKFLOW_DISPATCH_OK
- NA0576_CLOSEOUT_NO_PRIVATE_MATERIAL_PUBLISHED_OK
- NA0576_CLOSEOUT_ONE_READY_INVARIANT_OK

## Evidence Mapping

- D-1142 and D-1143 acceptance markers map to `DECISIONS.md`.
- Amendment merge proof maps to PR #1426 and merge commit `070791c43b36`.
- Required-check proof maps to post-merge public-safety, advisories,
  suite2-vectors, no-failed-required-check, and no-pending-visible-check-run
  classification artifacts.
- Successor proof maps to the NA-0577 start/bind proof completion harness block
  restored in `NEXT_ACTIONS.md` and D-1144.
- Wait-work durability proof maps to `START_HERE.md`, `AGENTS.md`, and
  `docs/ops/CODEX_BOUNDED_OPERATIONAL_AUTHORITY.md`.
- Boundary markers map to the closeout scope guard, private-material scan,
  overclaim scan, and repository diff proof.

## Expected Result

NA-0576 is DONE, NA-0577 is the sole READY item, D-1144 exists once, duplicate
decision IDs remain zero, no NA-0577 implementation occurs, and no qsl-server
start, remote action, qsc send/receive, workflow dispatch/rerun, source
mutation, dependency mutation, private-material publication, or public or
production readiness claim occurs during closeout.
