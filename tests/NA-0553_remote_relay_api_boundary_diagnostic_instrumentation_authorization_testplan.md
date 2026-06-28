Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-28

# NA-0553 Remote Relay API Boundary Diagnostic Instrumentation Authorization Testplan

This testplan records the governance and evidence markers for NA-0553. NA-0553 is authorization-only and does not implement diagnostic instrumentation.

Required markers:

- NA0553_D1094_BOUNDARY_DIAGNOSIS_CONSUMED_OK
- NA0553_D1095_CLOSEOUT_CONSUMED_OK
- NA0553_FRESH_QWORK_PROOF_OK
- NA0553_CURRENT_MAIN_CHECKS_CLASSIFIED_OK
- NA0553_PUBLIC_SAFETY_GREEN_OK
- NA0553_ADVISORIES_GREEN_OK
- NA0553_NO_FAILED_REQUIRED_CHECKS_OK
- NA0553_STATUS_BODY_GAP_INHERITED_OK
- NA0553_QSC_DIAGNOSTIC_PATH_REVIEWED_OK
- NA0553_SCRIPT_DIAGNOSTIC_PATH_REVIEWED_OK
- NA0553_WORKFLOW_DIAGNOSTIC_BOUNDARY_CLASSIFIED_OK
- NA0553_REDACTION_POLICY_SELECTED_OK
- NA0553_INSTRUMENTATION_MODEL_SELECTED_OK
- NA0553_EXACT_FUTURE_PATH_BUNDLE_SELECTED_OR_STOP_RECORDED_OK
- NA0553_FUTURE_VALIDATION_POLICY_SELECTED_OK
- NA0553_RESULT_CLASSIFICATION_SELECTED_OK
- NA0553_SUCCESSOR_MODEL_SELECTED_OR_STOP_RECORDED_OK
- NA0553_NO_RERUN_EXECUTED_OK
- NA0553_NO_WORKFLOW_DISPATCH_OK
- NA0553_NO_LOCAL_REPRODUCTION_EXECUTED_OK
- NA0553_NO_SCRIPT_REMEDIATION_OK
- NA0553_NO_WORKFLOW_MUTATION_OK
- NA0553_NO_RUNTIME_MUTATION_OK
- NA0553_NO_QSC_SOURCE_MUTATION_OK
- NA0553_NO_DEPENDENCY_LOCKFILE_MUTATION_OK
- NA0553_NO_QSL_SERVER_ATTACHMENTS_OK
- NA0553_NO_PUBLIC_SITE_MUTATION_OK
- NA0553_NO_CLOUDFLARE_MUTATION_OK
- NA0553_NO_RAW_LOGS_COMMITTED_OK
- NA0553_NO_PRIVATE_MATERIAL_PUBLISHED_OK
- NA0553_NO_PUBLIC_READINESS_CLAIM_OK
- NA0553_NO_PRODUCTION_READINESS_CLAIM_OK
- NA0553_ONE_READY_INVARIANT_OK

Validation gates:

- `git diff --check`
- exact five-path implementation scope guard
- queue/decision proof: READY_COUNT 1, READY NA-0553, D-1096 once, D-1097 absent, duplicate decision count zero
- marker proof for every marker above
- changed Markdown local-link check
- added-line/new-file private-material scan
- added-line/new-file overclaim scan
- docs/governance-only classifier
- PR body preflight
- goal-lint if available
- root cargo audit
- nested qsc fuzz lock cargo audit
- `cargo fmt --check`
- `sh -n scripts/ci/qsc_adversarial.sh`
- `bash -n scripts/ci/qsc_adversarial.sh`

Focused qsc runtime tests may be skipped for NA-0553 because this lane is authorization-only and changes no qsc runtime, source, tests, dependency, lockfile, workflow, script, qsl-server, qsl-attachments, public-site, or Cloudflare path.
