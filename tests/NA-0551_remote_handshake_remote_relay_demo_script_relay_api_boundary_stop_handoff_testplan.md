Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-27

# NA-0551 Relay API Boundary Stop Handoff Testplan

This testplan records the governance-only terminal stop handoff for NA-0551.
It does not authorize script remediation, reruns, workflow dispatch, local
reproduction, qsc send/receive, qsl-server, qsl-attachments, dependency changes,
or workflow mutation.

## Required Markers

- NA0551_STOP_D1090_BOUNDARY_AUTH_CONSUMED_OK
- NA0551_STOP_D1091_CLOSEOUT_CONSUMED_OK
- NA0551_STOP_D465_ATTEMPT_CONSUMED_OK
- NA0551_STOP_D466_DIAGNOSIS_CONSUMED_OK
- NA0551_STOP_BRANCH_PRESERVED_OK
- NA0551_STOP_BRANCH_NOT_MERGED_OK
- NA0551_STOP_NO_PR_OPENED_OK
- NA0551_STOP_RELAY_INBOX_PUSH_FAILED_CLASSIFIED_OK
- NA0551_STOP_SCRIPT_ONLY_FIX_NOT_SUPPORTED_OK
- NA0551_STOP_D1092_ACCEPTED_OK
- NA0551_STOP_SELECTED_NA0552_OK
- NA0551_STOP_NO_SCRIPT_CHANGES_MERGED_OK
- NA0551_STOP_NO_WORKFLOW_MUTATION_OK
- NA0551_STOP_NO_QSC_SOURCE_MUTATION_OK
- NA0551_STOP_NO_DEPENDENCY_LOCKFILE_MUTATION_OK
- NA0551_STOP_NO_QSL_SERVER_ATTACHMENTS_OK
- NA0551_STOP_NO_RERUN_EXECUTED_OK
- NA0551_STOP_NO_WORKFLOW_DISPATCH_OK
- NA0551_STOP_NO_LOCAL_REPRODUCTION_OK
- NA0551_STOP_NO_QWORK_EXECUTION_OK
- NA0551_STOP_NO_QSL_BACKUP_OK
- NA0551_STOP_NO_PUBLIC_SITE_MUTATION_OK
- NA0551_STOP_NO_CLOUDFLARE_MUTATION_OK
- NA0551_STOP_NO_PRIVATE_MATERIAL_PUBLISHED_OK
- NA0551_STOP_NO_PUBLIC_READINESS_CLAIM_OK
- NA0551_STOP_NO_PRODUCTION_READINESS_CLAIM_OK
- NA0551_STOP_ONE_READY_INVARIANT_OK

## Validation Gates

- `git diff --check`
- exact five-path scope guard, including untracked-file check
- queue/decision proof: READY_COUNT 1, READY NA-0551, D-1092 once, D-1093
  absent, duplicate decision count zero
- marker proof for every marker listed above
- deterministic markdown link-check
- private-material scan over added lines and new files
- overclaim scan over added lines and PR body
- docs/governance-only classifier
- PR body preflight
- goal-lint when available
- root cargo audit
- nested qsc fuzz lock cargo audit
- `cargo fmt --check`
- `sh -n scripts/ci/qsc_adversarial.sh`
- `bash -n scripts/ci/qsc_adversarial.sh`

Focused qsc runtime tests are intentionally skipped because this is a
governance-only stop handoff, no qsc source/runtime/dependency/workflow path is
mutated, and local reproduction is forbidden.

## Expected Result

`REMOTE_SMOKE_DEMO_SCRIPT_REMEDIATION_RELAY_API_BOUNDARY_STOP_ACCEPTED`
