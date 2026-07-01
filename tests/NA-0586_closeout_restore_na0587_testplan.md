Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-07-01

# NA-0586 Closeout / Restore NA-0587 Testplan

Goals: G1, G2, G3, G4, G5

## Scope

This closeout marks NA-0586 DONE after implementation PR #1446 and restores the
exact D-1163-selected successor:

`NA-0587 -- QSL Remote Relay Network Path Remediation Harness`

No NA-0587 implementation, remote action, workflow dispatch/rerun, qsc
send/receive, qsl-server action, qsl-attachments action, source/script/workflow
mutation, dependency/lockfile change, public-site change, Cloudflare change, or
private-material publication is in scope.

## Required Markers

- `NA0586_CLOSEOUT_IMPLEMENTATION_PR_MERGED_OK`
- `NA0586_CLOSEOUT_D1163_ACCEPTED_OK`
- `NA0586_CLOSEOUT_PUBLIC_SAFETY_GREEN_OK`
- `NA0586_CLOSEOUT_ADVISORIES_GREEN_OK`
- `NA0586_CLOSEOUT_NO_FAILED_REQUIRED_CHECKS_OK`
- `NA0586_CLOSEOUT_D1164_RESTORED_NA0587_OK`
- `NA0586_CLOSEOUT_SELECTED_SUCCESSOR_EXACT_OK`
- `NA0586_CLOSEOUT_NO_NA0587_IMPLEMENTATION_OK`
- `NA0586_CLOSEOUT_NO_REMOTE_ACTION_OK`
- `NA0586_CLOSEOUT_NO_QSL_SERVER_START_STOP_CLEANUP_OK`
- `NA0586_CLOSEOUT_NO_QSC_SEND_RECEIVE_OK`
- `NA0586_CLOSEOUT_NO_WORKFLOW_DISPATCH_OR_RERUN_OK`
- `NA0586_CLOSEOUT_NO_QSL_ATTACHMENTS_OK`
- `NA0586_CLOSEOUT_NO_PRIVATE_MATERIAL_PUBLISHED_OK`
- `NA0586_CLOSEOUT_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0586_CLOSEOUT_NO_PRODUCTION_READINESS_CLAIM_OK`
- `NA0586_CLOSEOUT_NO_SECURITY_COMPLETION_CLAIM_OK`
- `NA0586_CLOSEOUT_ONE_READY_INVARIANT_OK`

## Validation Evidence

- Implementation PR #1446 merged at `7767fb130840` from head `13140a698afc`.
- D-1163 exists once, is Accepted, and selected
  `REMOTE_RELAY_DIAGNOSTIC_VERIFICATION_DNS_TIMEOUT`.
- D-1163 selected the exact NA-0587 successor restored in `NEXT_ACTIONS.md`.
- Post-merge public-safety and advisories completed success.
- Completed post-merge checks contained no failed or pending checks.
- Scope guard permits only `NEXT_ACTIONS.md`, `DECISIONS.md`,
  `TRACEABILITY.md`, `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`, and this file.
- Queue proof after patch must show NA-0586 DONE, READY_COUNT 1, READY NA-0587,
  D-1164 once, and no D-1165.

## Boundary Evidence

No endpoint value, private port value, token value, Authorization value,
route-token/capability value, payload, response body, process identity,
topology, authorized_keys content, public key material, private key material,
secret environment value, Cloudflare token, API key, raw log, or private
material is published.

No protocol, wire, crypto, auth, state-machine, qsc runtime/source,
qsl-server runtime/source, qsl-attachments runtime/source, dependency,
lockfile, workflow, public-site, or Cloudflare semantic change is introduced.

No public-readiness claim is introduced. No production-readiness claim is
introduced. No public-internet-readiness claim is introduced. No
external-review-complete claim is introduced. No vulnerability-free claim is
introduced. No bug-free claim is introduced. No perfect-build claim is
introduced. No perfect-crypto claim is introduced.
