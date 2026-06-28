Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-28

# NA-0552 Remote Relay API Boundary Diagnosis Authorization Testplan

This testplan records governance-only validation for NA-0552. It authorizes no
script remediation, diagnostic implementation, workflow dispatch, rerun, local
reproduction, qsc send/receive, qsl-server/qsl-attachments use, public-site
mutation, Cloudflare mutation, or private-material publication.

## Required Markers

- NA0552_D1092_STOP_HANDOFF_CONSUMED_OK
- NA0552_D1093_CLOSEOUT_CONSUMED_OK
- NA0552_FRESH_QWORK_PROOF_OK
- NA0552_CURRENT_MAIN_CHECKS_CLASSIFIED_OK
- NA0552_PUBLIC_SAFETY_GREEN_OK
- NA0552_ADVISORIES_GREEN_OK
- NA0552_NO_FAILED_REQUIRED_CHECKS_OK
- NA0552_NA0551_STOP_INHERITED_OK
- NA0552_STOPPED_BRANCH_REVIEWED_OK
- NA0552_D465_D466_RUNS_REVIEWED_OK
- NA0552_RELAY_INBOX_PUSH_FAILED_REVIEWED_OK
- NA0552_QSC_RELAY_PUSH_SEMANTICS_REVIEWED_OK
- NA0552_STATUS_BODY_VISIBILITY_CLASSIFIED_OK
- NA0552_SECRET_ENV_BOUNDARY_CLASSIFIED_OK
- NA0552_PER_TARGET_BOUNDARY_CLASSIFIED_OK
- NA0552_RESULT_CLASSIFICATION_SELECTED_OK
- NA0552_SUCCESSOR_MODEL_SELECTED_OR_STOP_RECORDED_OK
- NA0552_NO_RERUN_EXECUTED_OK
- NA0552_NO_WORKFLOW_DISPATCH_OK
- NA0552_NO_LOCAL_REPRODUCTION_EXECUTED_OK
- NA0552_NO_SCRIPT_REMEDIATION_OK
- NA0552_NO_WORKFLOW_MUTATION_OK
- NA0552_NO_RUNTIME_MUTATION_OK
- NA0552_NO_QSC_SOURCE_MUTATION_OK
- NA0552_NO_DEPENDENCY_LOCKFILE_MUTATION_OK
- NA0552_NO_QSL_SERVER_ATTACHMENTS_MUTATION_OK
- NA0552_NO_PUBLIC_SITE_MUTATION_OK
- NA0552_NO_CLOUDFLARE_MUTATION_OK
- NA0552_NO_RAW_LOGS_COMMITTED_OK
- NA0552_NO_PRIVATE_MATERIAL_PUBLISHED_OK
- NA0552_NO_PUBLIC_READINESS_CLAIM_OK
- NA0552_NO_PRODUCTION_READINESS_CLAIM_OK
- NA0552_ONE_READY_INVARIANT_OK

## Validation Gates

- `git diff --check`
- exact five-path implementation scope guard, including untracked-file check
- queue/decision proof: READY_COUNT 1, READY NA-0552, D-1094 once, D-1095
  absent, duplicate decision count zero
- marker proof for every marker listed above
- deterministic markdown link-check for changed files
- private-material scan over added lines and new files
- overclaim scan over added lines, new files, and PR body
- docs/governance-only classifier
- PR body preflight
- goal-lint when available
- root cargo audit
- nested qsc fuzz lock cargo audit
- `cargo fmt --check`
- `sh -n scripts/ci/qsc_adversarial.sh`
- `bash -n scripts/ci/qsc_adversarial.sh`

Focused qsc runtime tests are intentionally skipped because this lane is
authorization-only, no qsc source/runtime/dependency/workflow path is mutated,
and local reproduction or qsc send/receive is forbidden.

## Expected Result

`REMOTE_RELAY_API_BOUNDARY_DIAGNOSTIC_INSTRUMENTATION_READY`
