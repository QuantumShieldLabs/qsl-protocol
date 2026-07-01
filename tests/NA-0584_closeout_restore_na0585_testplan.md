Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-07-01

# NA-0584 Closeout and NA-0585 Restoration Testplan

## Required Markers

- NA0584_CLOSEOUT_IMPLEMENTATION_PR_MERGED_OK
- NA0584_CLOSEOUT_D1159_ACCEPTED_OK
- NA0584_CLOSEOUT_PUBLIC_SAFETY_GREEN_OK
- NA0584_CLOSEOUT_ADVISORIES_GREEN_OK
- NA0584_CLOSEOUT_NO_FAILED_REQUIRED_CHECKS_OK
- NA0584_CLOSEOUT_D1160_RESTORED_NA0585_OK
- NA0584_CLOSEOUT_SELECTED_SUCCESSOR_EXACT_OK
- NA0584_CLOSEOUT_NO_NA0585_IMPLEMENTATION_OK
- NA0584_CLOSEOUT_NO_REMOTE_ACTION_OK
- NA0584_CLOSEOUT_NO_QSL_SERVER_START_STOP_CLEANUP_OK
- NA0584_CLOSEOUT_NO_QSC_SEND_RECEIVE_OK
- NA0584_CLOSEOUT_NO_WORKFLOW_DISPATCH_OR_RERUN_OK
- NA0584_CLOSEOUT_NO_QSL_ATTACHMENTS_OK
- NA0584_CLOSEOUT_NO_PRIVATE_MATERIAL_PUBLISHED_OK
- NA0584_CLOSEOUT_NO_PUBLIC_READINESS_CLAIM_OK
- NA0584_CLOSEOUT_NO_PRODUCTION_READINESS_CLAIM_OK
- NA0584_CLOSEOUT_NO_SECURITY_COMPLETION_CLAIM_OK
- NA0584_CLOSEOUT_ONE_READY_INVARIANT_OK

## Closeout Proof

- Implementation PR #1442 merged at `409db738cb32` from head `ea62fdb06b87`.
- D-1159 exists once and is Accepted.
- D-1159 selected `REMOTE_RELAY_REACHABILITY_QSC_TIMEOUT_PHASE_ONLY`.
- D-1159 selected exact successor:
  `NA-0585 -- QSL Remote Relay Diagnostic Surface Improvement Harness`.
- Post-merge public-safety completed success.
- Post-merge advisories completed success.
- No failed required checks were classified.
- D-1160 restores NA-0585 as the sole READY successor.
- NA-0584 is marked DONE.
- NA-0585 implementation is not performed in this closeout.

## Boundary Proof

Closeout changes are limited to:

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0584_closeout_restore_na0585_testplan.md`

No remote action, SSH, scp, Tailscale, remote command, qsl-server start,
qsl-server stop, qsl-server cleanup, qsl-server deployment, qsl-server mutation,
qsl-attachments command or mutation, qsc send/receive, workflow dispatch,
workflow rerun, qsl-protocol source/script/workflow/dependency mutation,
qsl-server source mutation, public-site mutation, Cloudflare mutation, or
private-material publication occurs.

No endpoint values, private port values, topology, token values, Authorization
values, command lines, process identities, payloads, response bodies,
authorized_keys content, public key material, private key material, secret
environment values, Cloudflare tokens, or API keys are published.

No public-readiness claim, production-readiness claim, public-internet-readiness
claim, external-review-complete claim, vulnerability-free claim, bug-free claim,
perfect-build claim, or perfect-crypto claim is introduced.

## Validation

- Exact five-path closeout scope guard.
- Queue/decision proof:
  - D-1159 once.
  - D-1160 once.
  - D-1161 absent.
  - NA-0584 DONE.
  - READY_COUNT 1.
  - READY NA-0585.
- Marker proof for every required marker above.
- Link-check.
- Added-line/private-material scan.
- Overclaim scan.
- Docs/governance-only classifier.
- PR body preflight and goal-lint.
- Root cargo audit.
- Nested qsc fuzz cargo audit.
- `cargo fmt --check`.
- `sh -n scripts/ci/qsc_adversarial.sh`.
- `bash -n scripts/ci/qsc_adversarial.sh`.
