Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-29

# NA-0568 Remote Relay Inspiron Codex-Executed Deployment Recovery Testplan

## Purpose

Validate that NA-0568 executes only the D-1124/D-1125-authorized bounded remote
recovery lane, publishes only coarse classifications, preserves private-material
boundaries, and selects the exact NA-0569 successor.

## Required Markers

- NA0568_D1124_AUTHORITY_CONSUMED_OK
- NA0568_D1125_CLOSEOUT_CONSUMED_OK
- NA0568_FRESH_QWORK_PROOF_OK
- NA0568_CURRENT_MAIN_CHECKS_CLASSIFIED_OK
- NA0568_REMOTE_SCRIPTS_GENERATED_OK
- NA0568_REMOTE_SCRIPTS_STATIC_REVIEW_OK
- NA0568_SSH_READINESS_CLASSIFIED_OK
- NA0568_REMOTE_INVENTORY_CLASSIFIED_OK
- NA0568_REMOTE_REPAIR_EXECUTED_OR_SKIPPED_OK
- NA0568_REMOTE_POSTCHECK_CLASSIFIED_OK
- NA0568_ROLLBACK_MANIFEST_PRESENT_IF_WRITE_OK
- NA0568_PRIVATE_MATERIAL_SCAN_OK
- NA0568_RESULT_CLASSIFICATION_SELECTED_OK
- NA0568_SUCCESSOR_MODEL_SELECTED_OR_STOP_RECORDED_OK
- NA0568_NO_ENDPOINT_VALUE_PUBLISHED_OK
- NA0568_NO_PRIVATE_PORT_VALUE_PUBLISHED_OK
- NA0568_NO_ROUTE_TOKEN_CAPABILITY_PUBLISHED_OK
- NA0568_NO_BEARER_AUTH_PUBLISHED_OK
- NA0568_NO_PRIVATE_TOPOLOGY_PUBLISHED_OK
- NA0568_NO_PROCESS_IDENTITY_PUBLISHED_OK
- NA0568_NO_AUTHORIZED_KEYS_CONTENT_PUBLISHED_OK
- NA0568_NO_PAYLOAD_BODY_PUBLISHED_OK
- NA0568_NO_SUDO_SYSTEMCTL_TAILSCALE_OK
- NA0568_NO_ACCOUNT_AUTHORIZED_KEYS_MUTATION_OK
- NA0568_NO_WRITES_OUTSIDE_QSLCODEX_TEST_WORKSPACE_OK
- NA0568_NO_QSC_SEND_RECEIVE_OK
- NA0568_NO_WORKFLOW_DISPATCH_OK
- NA0568_NO_RERUN_EXECUTED_OK
- NA0568_NO_SOURCE_MUTATION_OK
- NA0568_NO_WORKFLOW_MUTATION_OK
- NA0568_NO_DEPENDENCY_LOCKFILE_MUTATION_OK
- NA0568_NO_QSL_SERVER_ATTACHMENTS_OK
- NA0568_NO_PUBLIC_SITE_MUTATION_OK
- NA0568_NO_CLOUDFLARE_MUTATION_OK
- NA0568_NO_PRIVATE_MATERIAL_PUBLISHED_OK
- NA0568_NO_PUBLIC_READINESS_CLAIM_OK
- NA0568_NO_PRODUCTION_READINESS_CLAIM_OK
- NA0568_ONE_READY_INVARIANT_OK

## Gate Checks

Fresh qwork proof must be copied and parsed before SSH or repository mutation.
Required proof values include startup OK, lane NA-0568, repo qsl-protocol,
clean worktree/index/untracked state, HEAD equal to origin/main, READY count
one, queue top READY NA-0568, and shared cargo target ready.

Pre-fetch live state must show a clean worktree, HEAD and origin/main matching
the qwork proof, root disk below the 95 percent stop threshold, and
`/backup/qsl` mounted.

After fetch, origin/main must equal or descend from `b4e0b5a52ca4`.

## Queue And Decision Checks

Before patch:

- READY count is 1.
- READY item is NA-0568.
- NA-0567 is DONE.
- NA-0566 is DONE.
- D-1124 exists once and is Accepted.
- D-1125 exists once and is Accepted.
- D-1126 is absent.
- D-1127 is absent.
- duplicate decision count is 0.

After patch:

- READY count remains 1.
- READY item remains NA-0568.
- D-1126 exists once.
- D-1127 is absent.
- duplicate decision count remains 0.

## Remote Command Checks

Only the exact NA-0568 SSH allowlist may run:

- SSH readiness once;
- remote inventory through SSH stdin once after readiness;
- remote repair through SSH stdin once only after repairable inventory
  classification;
- remote postcheck through SSH stdin once after inventory and repair if repair
  ran.

No scp, sftp, rsync, sudo, systemctl, service, Tailscale, firewall, qsc
send/receive, qsc E2EE, workflow dispatch, rerun, qsl-server, qsl-attachments,
qsl-backup, or backup mutation is allowed.

## Result Checks

Expected NA-0568 result:
`REMOTE_RECOVERY_QSC_RELAY_COMMAND_AUTH_REQUIRED`.

Required proof:

- SSH readiness classified ready.
- Inventory classified workspace repairable.
- Repair classified workspace directories repaired.
- Postcheck classified workspace ready but listener not ready.
- qsc binary ready.
- qsc relay start command not discovered.
- rollback manifest present if repair wrote files.
- repair manifest present if repair wrote files.
- no private material published.

## Validation Commands

Required local validation before PR:

- `git diff --check`
- exact five-path implementation scope guard
- queue/decision proof
- marker proof
- link-check
- added-line/new-file private-material scan
- remote-output private-material scan proof
- prohibited-material scan
- added-line/new-file overclaim scan
- docs/governance-only classifier
- PR body preflight
- goal-lint if available
- root cargo audit
- nested qsc fuzz lock cargo audit
- `cargo fmt --check`
- `sh -n scripts/ci/qsc_adversarial.sh`
- `bash -n scripts/ci/qsc_adversarial.sh`

Focused qsc runtime tests may be skipped because NA-0568 mutates no qsc source,
runtime, dependency, workflow, executable test, fuzz target, or vector and qsc
send/receive is not authorized.

## Acceptance

Pass only if all markers are present, D-1126 exists exactly once, D-1127 remains
absent, one READY remains, scope is limited to the five allowed implementation
paths, and no forbidden remote action, repository mutation, private-material
publication, or claim expansion occurs.
