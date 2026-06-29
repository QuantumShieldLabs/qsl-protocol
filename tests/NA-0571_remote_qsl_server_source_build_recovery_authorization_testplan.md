Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-29

# NA-0571 qsl-server Source / Build Recovery Authorization Testplan

## Objective

Verify that NA-0571 consumes the NA-0570 source-audit stop, classifies current
qsl-server source/build recovery safely, selects an exact successor or records a
stop reason, and preserves all no-mutation/no-remote boundaries.

## Required Markers

- NA0571_D1130_SOURCE_AUDIT_STOP_CONSUMED_OK
- NA0571_D1131_CLOSEOUT_CONSUMED_OK
- NA0571_FRESH_QWORK_PROOF_OK
- NA0571_CURRENT_MAIN_CHECKS_CLASSIFIED_OK
- NA0571_QSL_SERVER_SOURCE_ACQUIRED_OK
- NA0571_BASELINE_QSL_SERVER_AUDIT_CLASSIFIED_OK
- NA0571_QSL_SERVER_DEPENDENCY_PATH_CLASSIFIED_OK
- NA0571_SCRATCH_RECOVERY_ATTEMPTED_OR_NOT_NEEDED_OK
- NA0571_RECOVERY_FEASIBILITY_CLASSIFIED_OK
- NA0571_SUCCESSOR_SELECTED_OR_STOP_RECORDED_OK
- NA0571_NO_QSL_SERVER_SOURCE_OF_TRUTH_MUTATION_OK
- NA0571_NO_QSL_SERVER_PR_OPENED_OK
- NA0571_NO_QSL_SERVER_DEPLOYMENT_OK
- NA0571_NO_QSL_ATTACHMENTS_OK
- NA0571_NO_REMOTE_ACTION_OK
- NA0571_NO_QSC_SEND_RECEIVE_OK
- NA0571_NO_WORKFLOW_DISPATCH_OK
- NA0571_NO_QSL_PROTOCOL_SOURCE_MUTATION_OK
- NA0571_NO_PUBLIC_SITE_MUTATION_OK
- NA0571_NO_PRIVATE_MATERIAL_PUBLISHED_OK
- NA0571_NO_PUBLIC_READINESS_CLAIM_OK
- NA0571_NO_PRODUCTION_READINESS_CLAIM_OK
- NA0571_ONE_READY_INVARIANT_OK

## Verification Expectations

- D-1130 exists once and is Accepted.
- D-1131 exists once and is Accepted.
- D-1132 is added exactly once by this implementation patch.
- D-1133 remains absent before optional closeout.
- qwork proof is fresh and matches live pre-fetch HEAD and origin/main.
- Current main checks are classified before qsl-server source acquisition or
  repository mutation.
- qsl-server source is acquired only under the proof root.
- Baseline qsl-server audit is classified.
- Baseline dependency path is classified.
- Scratch recovery is attempted only under the proof root or is explicitly not
  needed.
- Recovery feasibility classification is recorded.
- Successor is selected exactly when the result classification is not a stop.
- qsl-server source-of-truth is not mutated.
- No qsl-server PR is opened.
- No qsl-server deployment or run occurs.
- No qsl-attachments work occurs.
- No remote action, qsc send/receive, workflow dispatch, qsl-protocol source
  mutation, public-site mutation, or private-material publication occurs.
- No public-readiness or production-readiness claim is made.
- Exactly one READY remains mandatory.

## Validation Commands

- `git diff --check`
- exact five-path implementation scope guard
- no qsl-protocol source/script/workflow/dependency path change
- no qsl-server source-of-truth path change
- queue/decision proof
- marker proof
- deterministic markdown link check
- added-line/new-file private-material scan
- qsl-server scratch-output private-material scan proof
- overclaim scan
- docs/governance-only classifier
- PR body preflight
- goal-lint if available
- root `cargo audit --deny warnings`
- nested qsc fuzz `cargo audit --deny warnings`
- `cargo metadata --locked --format-version=1`
- `cargo fmt --check`
- `sh -n scripts/ci/qsc_adversarial.sh`
- `bash -n scripts/ci/qsc_adversarial.sh`

Focused qsc runtime tests may be skipped because this lane is authorization-only
governance/testplan work with no qsl-protocol source, runtime, dependency,
workflow, executable test, fuzz target, vector, or qsc send/receive mutation.
