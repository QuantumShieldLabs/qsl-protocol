Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-27

# NA-0546 Remote/Relay Non-Required CI Failure Bounded Reproduction Authorization Testplan

## Objective

Validate that NA-0546 consumes D-1080/D-1081, verifies fresh qwork proof and
current-main required-check health, authorizes exact future GitHub rerun and
dispatch paths, rejects unsafe local reproduction, records log/redaction/private
material policy, selects a successor, and preserves authorization-only scope.

## Required Markers

- `NA0546_D1080_FORWARD_AUDIT_CONSUMED_OK`
- `NA0546_D1081_CLOSEOUT_CONSUMED_OK`
- `NA0546_FRESH_QWORK_PROOF_OK`
- `NA0546_CURRENT_MAIN_CHECKS_CLASSIFIED_OK`
- `NA0546_PUBLIC_SAFETY_GREEN_OK`
- `NA0546_ADVISORIES_GREEN_OK`
- `NA0546_NO_FAILED_REQUIRED_CHECKS_OK`
- `NA0546_TARGET_FAILURES_INHERITED_OK`
- `NA0546_WORKFLOW_TRIGGER_INVENTORY_OK`
- `NA0546_REFERENCED_COMMAND_INVENTORY_OK`
- `NA0546_HISTORICAL_RERUN_DESIGN_OK`
- `NA0546_CURRENT_MAIN_DISPATCH_DESIGN_OK`
- `NA0546_LOCAL_REPRODUCTION_DESIGN_OK`
- `NA0546_QSL_SERVER_BOUNDARY_CLASSIFIED_OK`
- `NA0546_LOG_CAPTURE_POLICY_SELECTED_OK`
- `NA0546_REDACTION_POLICY_SELECTED_OK`
- `NA0546_PRIVATE_MATERIAL_SCAN_POLICY_SELECTED_OK`
- `NA0546_FUTURE_REPRODUCTION_CLASSIFICATIONS_SELECTED_OK`
- `NA0546_SUCCESSOR_MODEL_SELECTED_OR_STOP_RECORDED_OK`
- `NA0546_NO_RERUN_EXECUTED_OK`
- `NA0546_NO_WORKFLOW_DISPATCH_OK`
- `NA0546_NO_LOCAL_REPRODUCTION_EXECUTED_OK`
- `NA0546_NO_WORKFLOW_MUTATION_OK`
- `NA0546_NO_RUNTIME_MUTATION_OK`
- `NA0546_NO_QSC_SOURCE_MUTATION_OK`
- `NA0546_NO_DEPENDENCY_LOCKFILE_MUTATION_OK`
- `NA0546_NO_REMOTE_ACTION_OK`
- `NA0546_NO_QWORK_EXECUTION_OK`
- `NA0546_NO_QSL_BACKUP_EXECUTION_OK`
- `NA0546_NO_QSL_SERVER_ATTACHMENTS_MUTATION_OK`
- `NA0546_NO_PUBLIC_SITE_MUTATION_OK`
- `NA0546_NO_CLOUDFLARE_MUTATION_OK`
- `NA0546_NO_PRIVATE_MATERIAL_PUBLISHED_OK`
- `NA0546_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0546_NO_PRODUCTION_READINESS_CLAIM_OK`
- `NA0546_ONE_READY_INVARIANT_OK`

## Expected Authorization Result

`REMOTE_RELAY_REPRODUCTION_AUTHORIZATION_PARTIAL_GITHUB_RERUN_ONLY`

Historical rerun and current-main workflow dispatch are authorized for the
future lane. Local reproduction is not authorized.

## Exact Future Commands

Historical reruns:

- `gh run rerun 28222737830 --failed`
- `gh run rerun 28221877145 --failed`
- `gh run rerun 28221488004 --failed`

Current-main dispatch:

- `gh workflow run remote-handshake-tests.yml --ref main`
- `gh workflow run remote-relay-tests.yml --ref main -f scenario=happy-path -f seed=1`
- `gh workflow run relay-ui-integration.yml --ref main`

NA-0546 must not execute those commands.

## Scope Guard

Allowed implementation paths:

- `docs/governance/evidence/NA-0546_remote_relay_non_required_ci_failure_bounded_reproduction_authorization_plan.md`
- `tests/NA-0546_remote_relay_non_required_ci_failure_bounded_reproduction_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Any workflow, runtime/source, qsc source/test/fuzz/Cargo, dependency/lockfile,
qsl-server, qsl-attachments, public-site, or Cloudflare mutation fails this
testplan.

## Validation

Validation must prove:

- fresh qwork proof matches live pre-fetch repo state;
- public-safety and advisories completed success;
- no failed required checks;
- D-1082 exists once after patch;
- D-1083 is absent before closeout;
- duplicate decision count is zero;
- all required markers are present;
- changed files are limited to the allowed implementation set;
- added lines and new files pass private-material and overclaim scans;
- raw logs are not committed;
- focused qsc runtime tests are skipped only because this lane is
  authorization-only with no runtime/source/dependency/workflow mutation.
