Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-07-01

# NA-0587 Closeout and NA-0588 Restoration Testplan

## Required Markers

- NA0587_CLOSEOUT_IMPLEMENTATION_PR_MERGED_OK
- NA0587_CLOSEOUT_D1165_ACCEPTED_OK
- NA0587_CLOSEOUT_PUBLIC_SAFETY_GREEN_OK
- NA0587_CLOSEOUT_ADVISORIES_GREEN_OK
- NA0587_CLOSEOUT_NO_FAILED_REQUIRED_CHECKS_OK
- NA0587_CLOSEOUT_D1166_RESTORED_NA0588_OK
- NA0587_CLOSEOUT_SELECTED_SUCCESSOR_EXACT_OK
- NA0587_CLOSEOUT_NO_NA0588_IMPLEMENTATION_OK
- NA0587_CLOSEOUT_NO_REMOTE_ACTION_OK
- NA0587_CLOSEOUT_NO_WORKFLOW_DISPATCH_OR_RERUN_OK
- NA0587_CLOSEOUT_NO_QSL_ATTACHMENTS_OK
- NA0587_CLOSEOUT_NO_PRIVATE_MATERIAL_PUBLISHED_OK
- NA0587_CLOSEOUT_NO_PUBLIC_READINESS_CLAIM_OK
- NA0587_CLOSEOUT_NO_PRODUCTION_READINESS_CLAIM_OK
- NA0587_CLOSEOUT_NO_SECURITY_COMPLETION_CLAIM_OK
- NA0587_CLOSEOUT_ONE_READY_INVARIANT_OK

## Closeout Proof

- Implementation PR: #1448.
- Implementation merge commit: `f19619c43093`.
- Implementation head commit: `79139d7b74e2`.
- D-1165 exists once and is Accepted.
- D-1165 result classification:
  `LOCAL_CLIENT_RELAY_E2EE_INTEGRATION_PASS`.
- Post-merge public-safety: success.
- Post-merge advisories: success.
- Failed required checks: none.
- Pending required checks before closeout: none.
- D-1166 records NA-0587 closeout and NA-0588 restoration.
- NA-0587 status: DONE.
- Restored successor:
  `NA-0588 — QSL Local qsc / qsl-server Adversarial and Metadata Stress Harness`.
- READY_COUNT: 1.
- READY item: NA-0588.

## Boundary Proof

Closeout does not implement NA-0588 and does not execute remote action, SSH, scp,
Tailscale, remote command, qsl-server start/stop/cleanup/deployment/mutation,
qsl-attachments command/mutation, qsc send/receive, workflow dispatch/rerun,
qsl-protocol source/script/workflow/dependency mutation, qsl-server source
mutation, public-site mutation, Cloudflare mutation, qwork, qstart, or qresume.

No endpoint value, private port value, topology, token value, Authorization
value, command line, process identity, payload, response body, authorized_keys
content, public key material, private key material, secret environment value,
Cloudflare token, API key, raw private log, or private material is published.

No public-readiness claim is introduced. No production-readiness claim is
introduced. No public-internet-readiness claim is introduced. No
external-review-complete claim is introduced. No vulnerability-free claim is
introduced. No bug-free claim is introduced. No perfect-build claim is
introduced. No perfect-crypto claim is introduced. No crypto-complete claim is
introduced.

## Validation

Required validation covers exact closeout scope guard, queue/decision proof,
marker proof, link check, private-material scan, overclaim scan, PR body
preflight, goal-lint, root cargo audit, nested qsc fuzz cargo audit, locked cargo
metadata, cargo fmt, and shell syntax checks when scripts are touched.
