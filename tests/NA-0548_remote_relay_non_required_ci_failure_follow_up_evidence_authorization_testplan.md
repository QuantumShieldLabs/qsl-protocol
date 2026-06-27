Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-27

# NA-0548 Remote/Relay Non-Required CI Failure Follow-Up Evidence Authorization Testplan

## Objective

Validate that NA-0548 consumes D-1084/D-1085, verifies fresh qwork proof and
current-main required-check health, reviews NA-0547 failure signatures,
classifies per-target evidence gaps, rejects exact implementation until more
evidence is captured, classifies the qsl-server boundary, designs exact
follow-up GitHub evidence permissions, selects a successor, and preserves
authorization-only scope.

## Required Markers

- `NA0548_D1084_REPRO_LOG_CAPTURE_CONSUMED_OK`
- `NA0548_D1085_CLOSEOUT_CONSUMED_OK`
- `NA0548_FRESH_QWORK_PROOF_OK`
- `NA0548_CURRENT_MAIN_CHECKS_CLASSIFIED_OK`
- `NA0548_PUBLIC_SAFETY_GREEN_OK`
- `NA0548_ADVISORIES_GREEN_OK`
- `NA0548_NO_FAILED_REQUIRED_CHECKS_OK`
- `NA0548_NA0547_REPRO_RESULTS_INHERITED_OK`
- `NA0548_REMOTE_HANDSHAKE_SIGNATURE_REVIEWED_OK`
- `NA0548_REMOTE_RELAY_SIGNATURE_REVIEWED_OK`
- `NA0548_RELAY_UI_SIGNATURE_REVIEWED_OK`
- `NA0548_WORKFLOW_SURFACE_REVIEWED_OK`
- `NA0548_REFERENCED_SURFACE_REVIEWED_OK`
- `NA0548_EVIDENCE_GAPS_CLASSIFIED_OK`
- `NA0548_IMPLEMENTATION_READINESS_REVIEWED_OK`
- `NA0548_QSL_SERVER_BOUNDARY_CLASSIFIED_OK`
- `NA0548_FOLLOW_UP_PERMISSION_DESIGN_OK`
- `NA0548_LOG_PRIVATE_MATERIAL_POLICY_OK`
- `NA0548_RESULT_CLASSIFICATION_SELECTED_OK`
- `NA0548_SUCCESSOR_MODEL_SELECTED_OR_STOP_RECORDED_OK`
- `NA0548_NO_RERUN_EXECUTED_OK`
- `NA0548_NO_WORKFLOW_DISPATCH_OK`
- `NA0548_NO_LOCAL_REPRODUCTION_EXECUTED_OK`
- `NA0548_NO_WORKFLOW_MUTATION_OK`
- `NA0548_NO_RUNTIME_MUTATION_OK`
- `NA0548_NO_QSC_SOURCE_MUTATION_OK`
- `NA0548_NO_DEPENDENCY_LOCKFILE_MUTATION_OK`
- `NA0548_NO_QSL_SERVER_ATTACHMENTS_MUTATION_OK`
- `NA0548_NO_PUBLIC_SITE_MUTATION_OK`
- `NA0548_NO_CLOUDFLARE_MUTATION_OK`
- `NA0548_NO_RAW_LOGS_COMMITTED_OK`
- `NA0548_NO_PRIVATE_MATERIAL_PUBLISHED_OK`
- `NA0548_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0548_NO_PRODUCTION_READINESS_CLAIM_OK`
- `NA0548_ONE_READY_INVARIANT_OK`

## Expected Result

`REMOTE_RELAY_FOLLOW_UP_EVIDENCE_TARGETED_GITHUB_CAPTURE_READY`

Selected successor:

`NA-0549 -- QSL Remote/Relay Non-Required CI Failure Targeted Follow-Up Evidence Harness`

## Evidence Gap Classifications

- remote-handshake:
  `REMOTE_HANDSHAKE_GAP_ADDITIONAL_GITHUB_LOGS_REQUIRED`
- remote-relay:
  `REMOTE_RELAY_GAP_ADDITIONAL_GITHUB_LOGS_REQUIRED`
- relay-ui-integration:
  `RELAY_UI_GAP_ADDITIONAL_GITHUB_LOGS_REQUIRED`

## Implementation Readiness

All targets are:

`IMPLEMENTATION_NOT_READY_MORE_EVIDENCE_REQUIRED`

No exact implementation mutation path is selected by NA-0548.

## Scope Guard

Allowed implementation paths:

- `docs/governance/evidence/NA-0548_remote_relay_non_required_ci_failure_follow_up_evidence_authorization_plan.md`
- `tests/NA-0548_remote_relay_non_required_ci_failure_follow_up_evidence_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Any workflow, runtime/source, qsc source/test/fuzz/Cargo, dependency/lockfile,
qsl-server, qsl-attachments, public-site, Cloudflare, backup, or operator-local
system mutation fails this testplan.

## Validation

Validation must prove:

- fresh qwork proof matches live pre-fetch repo state;
- public-safety and advisories completed success;
- no failed required checks;
- D-1086 exists once after patch;
- D-1087 is absent before closeout;
- duplicate decision count is zero;
- all required markers are present;
- changed files are limited to the allowed implementation set;
- changed Markdown links resolve;
- added lines and new files pass private-material and overclaim scans;
- raw logs are not committed;
- no rerun, workflow dispatch, or local reproduction occurred;
- focused qsc runtime tests are skipped only because this lane is
  authorization-only with no runtime/source/dependency/workflow mutation.
