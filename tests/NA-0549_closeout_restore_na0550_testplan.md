Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-27

# NA-0549 Closeout and NA-0550 Restoration Test Plan

Goals: G1, G2, G3, G4, G5

## Purpose

Record the closeout validation markers for accepting NA-0549 targeted follow-up
evidence, marking NA-0549 DONE, and restoring NA-0550 as the sole READY
successor without implementing NA-0550.

## Required Markers

- NA0549_CLOSEOUT_TARGETED_EVIDENCE_PR_MERGED_OK
- NA0549_CLOSEOUT_D1088_ACCEPTED_OK
- NA0549_CLOSEOUT_PUBLIC_SAFETY_GREEN_OK
- NA0549_CLOSEOUT_ADVISORIES_GREEN_OK
- NA0549_CLOSEOUT_NO_FAILED_REQUIRED_CHECKS_OK
- NA0549_CLOSEOUT_D1089_RESTORED_NA0550_OK
- NA0549_CLOSEOUT_SELECTED_SUCCESSOR_EXACT_OK
- NA0549_CLOSEOUT_NO_NA0550_IMPLEMENTATION_OK
- NA0549_CLOSEOUT_NO_RERUN_EXECUTED_OK
- NA0549_CLOSEOUT_NO_WORKFLOW_DISPATCH_OK
- NA0549_CLOSEOUT_NO_LOCAL_REPRODUCTION_OK
- NA0549_CLOSEOUT_NO_WORKFLOW_MUTATION_OK
- NA0549_CLOSEOUT_NO_RUNTIME_MUTATION_OK
- NA0549_CLOSEOUT_NO_QSC_SOURCE_MUTATION_OK
- NA0549_CLOSEOUT_NO_DEPENDENCY_LOCKFILE_MUTATION_OK
- NA0549_CLOSEOUT_NO_QSL_SERVER_ATTACHMENTS_MUTATION_OK
- NA0549_CLOSEOUT_NO_PUBLIC_SITE_MUTATION_OK
- NA0549_CLOSEOUT_NO_CLOUDFLARE_MUTATION_OK
- NA0549_CLOSEOUT_NO_RAW_LOGS_COMMITTED_OK
- NA0549_CLOSEOUT_NO_RAW_ARTIFACTS_COMMITTED_OK
- NA0549_CLOSEOUT_NO_PRIVATE_MATERIAL_PUBLISHED_OK
- NA0549_CLOSEOUT_NO_PUBLIC_READINESS_CLAIM_OK
- NA0549_CLOSEOUT_NO_PRODUCTION_READINESS_CLAIM_OK
- NA0549_CLOSEOUT_ONE_READY_INVARIANT_OK

## Validation Gates

- `git diff --check` passes.
- Exact five-path closeout scope guard passes.
- Queue and decision proof shows READY_COUNT 1, READY NA-0550, NA-0549 DONE,
  D-1088 once, D-1089 once, D-1090 absent, and duplicate decision count zero.
- Marker proof finds every required marker in this test plan.
- Link-check for changed Markdown files passes.
- Added-line and new-file private-material scan passes.
- Added-line and new-file overclaim scan passes.
- Docs/governance-only classifier confirms no forbidden mutation paths changed.
- PR body preflight passes.
- goal-lint passes when available.
- Root cargo audit passes.
- Nested qsc fuzz lock cargo audit passes.
- `cargo fmt --check` passes.
- `sh -n scripts/ci/qsc_adversarial.sh` passes.
- `bash -n scripts/ci/qsc_adversarial.sh` passes.

## Boundary

This closeout does not authorize or perform NA-0550 implementation, rerun,
workflow dispatch, local reproduction, workflow mutation, runtime mutation, qsc
source/test/fuzz/Cargo mutation, dependency/lockfile mutation,
qsl-server/qsl-attachments mutation, public-site mutation, Cloudflare mutation,
raw-log commit, raw-artifact commit, or private-material publication.
