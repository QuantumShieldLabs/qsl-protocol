Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-07-01

# NA-0583 Closeout Restore NA-0584 Testplan

## Purpose

Record deterministic closeout checks for marking NA-0583 DONE and restoring the
exact D-1157-selected NA-0584 runner/service reachability remediation successor
as the sole READY item.

## Markers

- NA0583_CLOSEOUT_IMPLEMENTATION_PR_MERGED_OK
- NA0583_CLOSEOUT_D1157_ACCEPTED_OK
- NA0583_CLOSEOUT_PUBLIC_SAFETY_GREEN_OK
- NA0583_CLOSEOUT_ADVISORIES_GREEN_OK
- NA0583_CLOSEOUT_NO_FAILED_REQUIRED_CHECKS_OK
- NA0583_CLOSEOUT_D1158_RESTORED_NA0584_OK
- NA0583_CLOSEOUT_SELECTED_SUCCESSOR_EXACT_OK
- NA0583_CLOSEOUT_NO_NA0584_IMPLEMENTATION_OK
- NA0583_CLOSEOUT_NO_REMOTE_ACTION_OK
- NA0583_CLOSEOUT_NO_QSL_SERVER_START_STOP_CLEANUP_OK
- NA0583_CLOSEOUT_NO_QSC_SEND_RECEIVE_OK
- NA0583_CLOSEOUT_NO_WORKFLOW_DISPATCH_OR_RERUN_OK
- NA0583_CLOSEOUT_NO_QSL_ATTACHMENTS_OK
- NA0583_CLOSEOUT_NO_PRIVATE_MATERIAL_PUBLISHED_OK
- NA0583_CLOSEOUT_NO_PUBLIC_READINESS_CLAIM_OK
- NA0583_CLOSEOUT_NO_PRODUCTION_READINESS_CLAIM_OK
- NA0583_CLOSEOUT_NO_SECURITY_COMPLETION_CLAIM_OK
- NA0583_CLOSEOUT_ONE_READY_INVARIANT_OK

## Evidence Requirements

- qsl-protocol implementation PR #1440 merged at `01b38e45fba8`.
- D-1157 exists once and is Accepted.
- D-1158 exists once after closeout.
- NA-0583 is DONE.
- NA-0584 is the sole READY item.
- public-safety completed success.
- advisories completed success.
- no failed required checks were classified.
- Closeout scope is limited to `NEXT_ACTIONS.md`, `DECISIONS.md`,
  `TRACEABILITY.md`, `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`, and this file.

## Boundaries

Closeout does not implement NA-0584. Closeout performs no remote action, SSH,
scp, Tailscale, qsl-server start/stop/cleanup, qsl-server deployment,
qsl-server mutation, qsl-attachments command/mutation, qsc send/receive,
workflow dispatch/rerun, qsl-protocol source/script/workflow/dependency
mutation, qsl-server source mutation, public-site mutation, Cloudflare mutation,
or private-material publication.

No endpoint values, private port values, topology, token values, Authorization
values, command lines, process identities, payloads, response bodies,
authorized_keys content, public key material, private key material, secret
environment values, Cloudflare tokens, or API keys are published.

No public-readiness, production-readiness, public-internet-readiness,
external-review-complete, vulnerability-free, bug-free, perfect-build, or
perfect-crypto claim is made.
