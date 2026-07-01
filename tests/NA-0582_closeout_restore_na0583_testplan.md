# NA-0582 Closeout Restore NA-0583 Testplan

Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-07-01

## Purpose

Record the deterministic closeout checks for marking NA-0582 DONE and restoring
the exact D-1155-selected NA-0583 service-reachability triage successor as the
sole READY item.

## Markers

- NA0582_CLOSEOUT_IMPLEMENTATION_PR_MERGED_OK
- NA0582_CLOSEOUT_D1155_ACCEPTED_OK
- NA0582_CLOSEOUT_PUBLIC_SAFETY_GREEN_OK
- NA0582_CLOSEOUT_ADVISORIES_GREEN_OK
- NA0582_CLOSEOUT_NO_FAILED_REQUIRED_CHECKS_OK
- NA0582_CLOSEOUT_D1156_RESTORED_NA0583_OK
- NA0582_CLOSEOUT_SELECTED_SUCCESSOR_EXACT_OK
- NA0582_CLOSEOUT_NO_NA0583_IMPLEMENTATION_OK
- NA0582_CLOSEOUT_NO_REMOTE_ACTION_OK
- NA0582_CLOSEOUT_NO_QSL_SERVER_START_STOP_CLEANUP_OK
- NA0582_CLOSEOUT_NO_QSC_SEND_RECEIVE_OK
- NA0582_CLOSEOUT_NO_WORKFLOW_DISPATCH_OR_RERUN_OK
- NA0582_CLOSEOUT_NO_QSL_ATTACHMENTS_OK
- NA0582_CLOSEOUT_NO_PRIVATE_MATERIAL_PUBLISHED_OK
- NA0582_CLOSEOUT_NO_PUBLIC_READINESS_CLAIM_OK
- NA0582_CLOSEOUT_NO_PRODUCTION_READINESS_CLAIM_OK
- NA0582_CLOSEOUT_NO_SECURITY_COMPLETION_CLAIM_OK
- NA0582_CLOSEOUT_ONE_READY_INVARIANT_OK

## Evidence Requirements

- qsl-protocol implementation PR #1438 merged at `1bdf546107db` from head
  `d3b2296ee42f`.
- D-1155 exists once and is Accepted.
- D-1156 exists once after closeout.
- NA-0582 is DONE.
- NA-0583 is the sole READY item.
- public-safety completed success.
- advisories completed success.
- no failed required checks were classified.
- Closeout scope is limited to `NEXT_ACTIONS.md`, `DECISIONS.md`,
  `TRACEABILITY.md`, `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`, and this file.

## Boundaries

Closeout does not implement NA-0583. Closeout performs no remote action, SSH,
scp, Tailscale, qsl-server start/stop/cleanup, qsl-server deployment,
qsl-server mutation, qsl-attachments command/mutation, qsc send/receive,
workflow dispatch/rerun, qsl-protocol source/script/workflow/dependency
mutation, public-site mutation, Cloudflare mutation, or private-material
publication.

No endpoint values, private port values, topology, token values, Authorization
values, command lines, process identities, payloads, response bodies,
authorized_keys content, public key material, private key material, secret
environment values, Cloudflare tokens, or API keys are published.

No public-readiness, production-readiness, public-internet-readiness,
external-review-complete, vulnerability-free, bug-free, perfect-build, or
perfect-crypto claim is made.
