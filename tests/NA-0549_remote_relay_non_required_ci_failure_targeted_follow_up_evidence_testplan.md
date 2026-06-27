Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-27

# NA-0549 Remote/Relay Non-Required CI Failure Targeted Follow-Up Evidence Testplan

## Objective

Validate that NA-0549 consumes D-1086/D-1087, verifies fresh qwork proof and
current-main required-check health, reviews the six authorized NA-0547 runs,
captures proof-root-only metadata/log/artifact evidence, scans and redacts
private material, updates per-target evidence gaps, reviews implementation
readiness, classifies the qsl-server boundary, selects a successor, and
preserves read-only/no-mutation scope.

## Required Markers

- `NA0549_D1086_FOLLOW_UP_AUTH_CONSUMED_OK`
- `NA0549_D1087_CLOSEOUT_CONSUMED_OK`
- `NA0549_FRESH_QWORK_PROOF_OK`
- `NA0549_CURRENT_MAIN_CHECKS_CLASSIFIED_OK`
- `NA0549_PUBLIC_SAFETY_GREEN_OK`
- `NA0549_ADVISORIES_GREEN_OK`
- `NA0549_NO_FAILED_REQUIRED_CHECKS_OK`
- `NA0549_TARGET_RUN_SET_VERIFIED_OK`
- `NA0549_RUN_JOB_CHECK_METADATA_CAPTURED_OK`
- `NA0549_ARTIFACT_METADATA_CAPTURED_OK`
- `NA0549_RAW_LOGS_PROOF_ROOT_ONLY_OK`
- `NA0549_PRIVATE_MATERIAL_SCAN_OK`
- `NA0549_REDACTED_SUMMARIES_OK`
- `NA0549_REMOTE_HANDSHAKE_CORRELATED_OK`
- `NA0549_REMOTE_RELAY_CORRELATED_OK`
- `NA0549_RELAY_UI_CORRELATED_OK`
- `NA0549_EVIDENCE_GAPS_UPDATED_OK`
- `NA0549_IMPLEMENTATION_READINESS_REVIEWED_OK`
- `NA0549_QSL_SERVER_BOUNDARY_CLASSIFIED_OK`
- `NA0549_RESULT_CLASSIFICATION_SELECTED_OK`
- `NA0549_SUCCESSOR_MODEL_SELECTED_OR_STOP_RECORDED_OK`
- `NA0549_NO_RERUN_EXECUTED_OK`
- `NA0549_NO_WORKFLOW_DISPATCH_OK`
- `NA0549_NO_LOCAL_REPRODUCTION_EXECUTED_OK`
- `NA0549_NO_WORKFLOW_MUTATION_OK`
- `NA0549_NO_RUNTIME_MUTATION_OK`
- `NA0549_NO_QSC_SOURCE_MUTATION_OK`
- `NA0549_NO_DEPENDENCY_LOCKFILE_MUTATION_OK`
- `NA0549_NO_QSL_SERVER_ATTACHMENTS_MUTATION_OK`
- `NA0549_NO_PUBLIC_SITE_MUTATION_OK`
- `NA0549_NO_CLOUDFLARE_MUTATION_OK`
- `NA0549_NO_RAW_LOGS_COMMITTED_OK`
- `NA0549_NO_PRIVATE_MATERIAL_PUBLISHED_OK`
- `NA0549_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0549_NO_PRODUCTION_READINESS_CLAIM_OK`
- `NA0549_ONE_READY_INVARIANT_OK`

## Expected Result

`REMOTE_RELAY_TARGETED_EVIDENCE_QSL_SERVER_BOUNDARY_READY`

Selected successor:

`NA-0550 -- QSL Relay UI qsl-server Boundary Authorization Plan`

## Evidence-Gap Classifications

- remote-handshake:
  `REMOTE_HANDSHAKE_GAP_EXACT_IMPLEMENTATION_READY`
- remote-relay:
  `REMOTE_RELAY_GAP_EXACT_IMPLEMENTATION_READY`
- relay-ui-integration:
  `RELAY_UI_GAP_QSL_SERVER_BOUNDARY_REQUIRED`

## Implementation Readiness

- remote-handshake: `IMPLEMENTATION_READY_EXACT_PATHS`
- remote-relay: `IMPLEMENTATION_READY_EXACT_PATHS`
- relay-ui-integration:
  `IMPLEMENTATION_NOT_READY_QSL_SERVER_BOUNDARY_REQUIRED`

## qsl-server Boundary

`QSL_SERVER_BOUNDARY_SEPARATE_AUTHORIZATION_REQUIRED`

## Validation Gates

Implementation validation must record:

- `git diff --check`
- exact five-path implementation scope guard
- queue/decision proof with READY_COUNT 1, READY NA-0549, D-1088 once, D-1089
  absent, and duplicate decision count zero
- marker proof for every marker in this testplan
- link-check for changed Markdown files
- added-line/new-file private-material scan
- raw-log proof-root private-material scan result
- redacted-extract private-material scan result
- artifact private-material scan result
- added-line/new-file overclaim scan
- docs/governance-only classifier
- PR body preflight
- goal-lint if available and safe
- root cargo audit
- nested qsc fuzz lock cargo audit
- `cargo fmt --check`
- `sh -n scripts/ci/qsc_adversarial.sh`
- `bash -n scripts/ci/qsc_adversarial.sh`

Focused qsc runtime tests may be skipped because NA-0549 uses read-only GitHub
metadata/log/artifact evidence only; no local qsc runtime reproduction was
authorized; and no qsc source/runtime/dependency/workflow mutation occurred.
