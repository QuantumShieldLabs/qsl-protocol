Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-27

# NA-0550 Relay UI qsl-server Boundary Authorization Testplan

## Purpose

Validate that NA-0550 is authorization-only, consumes D-1088/D-1089, classifies
current-main required checks, reviews relay UI qsl-server/qsl-attachments
boundaries, selects an exact successor, and preserves all no-mutation and
no-claim boundaries.

## Required Markers

- NA0550_D1088_TARGETED_EVIDENCE_CONSUMED_OK
- NA0550_D1089_CLOSEOUT_CONSUMED_OK
- NA0550_FRESH_QWORK_PROOF_OK
- NA0550_CURRENT_MAIN_CHECKS_CLASSIFIED_OK
- NA0550_PUBLIC_SAFETY_GREEN_OK
- NA0550_ADVISORIES_GREEN_OK
- NA0550_NO_FAILED_REQUIRED_CHECKS_OK
- NA0550_RELAY_UI_FAILURE_INHERITED_OK
- NA0550_RELAY_UI_WORKFLOW_BOUNDARY_REVIEWED_OK
- NA0550_RELAY_UI_TEST_BOUNDARY_REVIEWED_OK
- NA0550_QSL_SERVER_METADATA_REVIEWED_OR_LIMIT_RECORDED_OK
- NA0550_QSL_ATTACHMENTS_METADATA_REVIEWED_OR_NON_INVOLVEMENT_OK
- NA0550_QSL_SERVER_BOUNDARY_MODEL_SELECTED_OK
- NA0550_REMOTE_SMOKE_SEQUENCING_REVIEWED_OK
- NA0550_RESULT_CLASSIFICATION_SELECTED_OK
- NA0550_SUCCESSOR_MODEL_SELECTED_OR_STOP_RECORDED_OK
- NA0550_NO_QSL_SERVER_COMMAND_OK
- NA0550_NO_QSL_SERVER_CLONE_BUILD_RUN_OK
- NA0550_NO_QSL_SERVER_MUTATION_OK
- NA0550_NO_QSL_ATTACHMENTS_COMMAND_OK
- NA0550_NO_QSL_ATTACHMENTS_CLONE_BUILD_RUN_OK
- NA0550_NO_QSL_ATTACHMENTS_MUTATION_OK
- NA0550_NO_RERUN_EXECUTED_OK
- NA0550_NO_WORKFLOW_DISPATCH_OK
- NA0550_NO_LOCAL_REPRODUCTION_EXECUTED_OK
- NA0550_NO_WORKFLOW_MUTATION_OK
- NA0550_NO_RUNTIME_MUTATION_OK
- NA0550_NO_QSC_SOURCE_MUTATION_OK
- NA0550_NO_DEPENDENCY_LOCKFILE_MUTATION_OK
- NA0550_NO_PUBLIC_SITE_MUTATION_OK
- NA0550_NO_CLOUDFLARE_MUTATION_OK
- NA0550_NO_PRIVATE_MATERIAL_PUBLISHED_OK
- NA0550_NO_PUBLIC_READINESS_CLAIM_OK
- NA0550_NO_PRODUCTION_READINESS_CLAIM_OK
- NA0550_ONE_READY_INVARIANT_OK

## Evidence Sources

- qwork proof files under `/srv/qbuild/work/NA-0550/.qwork/`
- proof root:
  `/srv/qbuild/tmp/NA0550_relay_ui_qsl_server_boundary_authorization_20260627T215616Z`
- `docs/governance/evidence/NA-0550_relay_ui_qsl_server_boundary_authorization_plan.md`
- DECISIONS.md D-1090
- TRACEABILITY.md NA-0550 row
- Rolling operations journal NA-0550 entry

## Acceptance Criteria

- D-1088 and D-1089 exist once and are Accepted.
- qwork proof timestamp is verified at `2026-06-27T21:49:17Z`.
- Current main is `93971209e26c` and required checks are classified.
- public-safety and advisories are success.
- No failed required check is observed.
- Relay UI workflow/test boundary is reviewed without executing relay UI tests.
- qsl-server metadata is reviewed or visibility limits are recorded.
- qsl-attachments non-involvement is recorded.
- qsl-server boundary model is selected.
- Remote-handshake/remote-relay sequencing is reviewed.
- Result classification and selected successor are recorded.
- Exactly one READY remains before closeout.

## Validation Commands

- `git diff --check`
- `git diff --name-only`
- `git diff --cached --name-only`
- `git ls-files --others --exclude-standard`
- queue/decision proof for READY_COUNT 1, READY NA-0550, D-1090 once,
  D-1091 absent, and duplicate decision count zero
- marker proof for all required NA-0550 markers
- changed Markdown link check
- added-line/new-file private-material scan
- added-line/new-file overclaim scan
- docs/governance-only classifier
- PR body preflight
- goal-lint if available and safe
- `cargo audit`
- nested qsc fuzz lock cargo audit
- `cargo fmt --check`
- `sh -n scripts/ci/qsc_adversarial.sh`
- `bash -n scripts/ci/qsc_adversarial.sh`

Focused qsc runtime tests are skipped because NA-0550 is authorization-only, no
local qsc runtime reproduction is authorized, no qsc source/runtime/dependency
or workflow mutation occurred, and no qsl-server/qsl-attachments execution
occurred.
