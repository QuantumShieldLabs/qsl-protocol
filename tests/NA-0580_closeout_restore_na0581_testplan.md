Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-07-01

# NA-0580 Closeout and NA-0581 Restoration Testplan

This testplan records the closeout proof markers for marking NA-0580 DONE and
restoring the D-1151-selected NA-0581 successor.

## Required Markers

- NA0580_CLOSEOUT_IMPLEMENTATION_PR_MERGED_OK
- NA0580_CLOSEOUT_D1151_ACCEPTED_OK
- NA0580_CLOSEOUT_PUBLIC_SAFETY_GREEN_OK
- NA0580_CLOSEOUT_ADVISORIES_GREEN_OK
- NA0580_CLOSEOUT_NO_FAILED_REQUIRED_CHECKS_OK
- NA0580_CLOSEOUT_D1152_RESTORED_NA0581_OK
- NA0580_CLOSEOUT_SELECTED_SUCCESSOR_EXACT_OK
- NA0580_CLOSEOUT_NO_NA0581_IMPLEMENTATION_OK
- NA0580_CLOSEOUT_NO_REMOTE_ACTION_OK
- NA0580_CLOSEOUT_NO_QSL_SERVER_START_OK
- NA0580_CLOSEOUT_NO_QSC_SEND_RECEIVE_OK
- NA0580_CLOSEOUT_NO_WORKFLOW_DISPATCH_OK
- NA0580_CLOSEOUT_NO_QSL_ATTACHMENTS_OK
- NA0580_CLOSEOUT_NO_PRIVATE_MATERIAL_PUBLISHED_OK
- NA0580_CLOSEOUT_NO_PUBLIC_READINESS_CLAIM_OK
- NA0580_CLOSEOUT_NO_PRODUCTION_READINESS_CLAIM_OK
- NA0580_CLOSEOUT_NO_SECURITY_COMPLETION_CLAIM_OK
- NA0580_CLOSEOUT_ONE_READY_INVARIANT_OK

## Closeout Proof

- Implementation PR #1434 merged at `c45bea4e686d`.
- D-1151 exists once and is Accepted.
- D-1151 result classification: `QSL_SERVER_EXPECTED_BIND_ENDPOINT_VALUE_UNAVAILABLE`.
- D-1151 selected successor: `NA-0581 -- QSL Remote qsl-server Expected Bind Failure Remediation Harness`.
- Post-merge public-safety completed success.
- Post-merge advisories completed success.
- No failed required checks were classified.
- D-1152 records closeout and restoration.
- NA-0580 is marked DONE.
- NA-0581 is restored READY as the exact D-1151-selected successor.
- READY_COUNT remains 1.

## Boundary Proof

Closeout does not implement NA-0581. It does not run remote action, SSH, scp,
Tailscale, remote command, qsl-server start, qsl-server deployment,
qsl-server mutation, qsl-attachments command or mutation, qsc send/receive,
workflow dispatch, workflow rerun, qsl-protocol source/script/workflow/dependency
mutation, public-site mutation, Cloudflare mutation, or private-material
publication.

No endpoint values, private port values, topology, token values, Authorization
values, command lines, process identities, payloads, response bodies,
authorized_keys content, public key material, private key material, secret
environment values, Cloudflare tokens, or API keys are published.

No public-readiness claim is made. No production-readiness claim is made. No
public-internet-readiness claim is made. No external-review-complete claim is
made. No vulnerability-free claim is made. No bug-free claim is made. No
perfect-build claim is made. No perfect-crypto claim is made.

## Validation

- Exact five-path closeout scope guard:
  - `NEXT_ACTIONS.md`
  - `DECISIONS.md`
  - `TRACEABILITY.md`
  - `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
  - `tests/NA-0580_closeout_restore_na0581_testplan.md`
- Queue/decision proof:
  - READY_COUNT 1.
  - READY NA-0581.
  - NA-0580 DONE.
  - D-1151 exists once.
  - D-1152 exists once.
  - D-1153 absent.
  - Duplicate decision count zero.
- Marker proof.
- Link-check.
- Added-line/private-material scan.
- Overclaim scan.
- Docs/governance-only classifier.
- PR body preflight and goal-lint.
- Root and nested qsc fuzz cargo audits.
- Cargo fmt check.
- qsc-adversarial shell syntax checks.
