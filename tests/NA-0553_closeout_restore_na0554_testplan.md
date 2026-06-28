Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-28

# NA-0553 Closeout / NA-0554 Restoration Testplan

## Scope

This testplan records the governance-only closeout of NA-0553 and restoration
of the exact D-1096-selected NA-0554 successor.

## Required Markers

- NA0553_CLOSEOUT_AUTHORIZATION_PR_MERGED_OK
- NA0553_CLOSEOUT_D1096_ACCEPTED_OK
- NA0553_CLOSEOUT_PUBLIC_SAFETY_GREEN_OK
- NA0553_CLOSEOUT_ADVISORIES_GREEN_OK
- NA0553_CLOSEOUT_NO_FAILED_REQUIRED_CHECKS_OK
- NA0553_CLOSEOUT_D1097_RESTORED_NA0554_OK
- NA0553_CLOSEOUT_SELECTED_SUCCESSOR_EXACT_OK
- NA0553_CLOSEOUT_NO_NA0554_IMPLEMENTATION_OK
- NA0553_CLOSEOUT_NO_RERUN_EXECUTED_OK
- NA0553_CLOSEOUT_NO_WORKFLOW_DISPATCH_OK
- NA0553_CLOSEOUT_NO_LOCAL_REPRODUCTION_OK
- NA0553_CLOSEOUT_NO_SCRIPT_REMEDIATION_OK
- NA0553_CLOSEOUT_NO_WORKFLOW_MUTATION_OK
- NA0553_CLOSEOUT_NO_QSC_SOURCE_MUTATION_OK
- NA0553_CLOSEOUT_NO_DEPENDENCY_LOCKFILE_MUTATION_OK
- NA0553_CLOSEOUT_NO_QSL_SERVER_ATTACHMENTS_OK
- NA0553_CLOSEOUT_NO_PUBLIC_SITE_MUTATION_OK
- NA0553_CLOSEOUT_NO_CLOUDFLARE_MUTATION_OK
- NA0553_CLOSEOUT_NO_RAW_LOGS_COMMITTED_OK
- NA0553_CLOSEOUT_NO_PRIVATE_MATERIAL_PUBLISHED_OK
- NA0553_CLOSEOUT_NO_PUBLIC_READINESS_CLAIM_OK
- NA0553_CLOSEOUT_NO_PRODUCTION_READINESS_CLAIM_OK
- NA0553_CLOSEOUT_ONE_READY_INVARIANT_OK

## Validation Gates

- `git diff --check`
- exact five-path closeout scope guard
- queue/decision proof: READY_COUNT 1, READY NA-0554, NA-0553 DONE, D-1096
  once, D-1097 once, D-1098 absent, duplicate decision count zero
- marker proof
- changed Markdown link-check
- added-line/new-file private-material scan
- added-line/new-file overclaim scan
- docs/governance-only classifier
- PR body preflight
- goal-lint when available
- root `cargo audit --deny warnings`
- nested qsc fuzz lock `cargo audit --deny warnings --file
  qsl/qsl-client/qsc/fuzz/Cargo.lock`
- `cargo fmt --check`
- `sh -n scripts/ci/qsc_adversarial.sh`
- `bash -n scripts/ci/qsc_adversarial.sh`

## Boundary

The closeout does not implement NA-0554 and does not mutate qsc source/test,
demo scripts, workflows, dependencies, lockfiles, qsl-server, qsl-attachments,
public-site content, Cloudflare configuration, backup state, operator-local
state, raw logs, or raw artifacts.
