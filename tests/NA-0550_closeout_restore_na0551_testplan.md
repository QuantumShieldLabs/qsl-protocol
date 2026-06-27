Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-27

# NA-0550 Closeout and NA-0551 Restoration Testplan

## Purpose

Validate that NA-0550 implementation PR #1373 merged, D-1090 was accepted
once, post-merge public-safety/advisories were green, NA-0550 was marked DONE,
and the exact D-1090-selected NA-0551 successor was restored as the sole READY
item without implementing NA-0551.

## Required Markers

- NA0550_CLOSEOUT_BOUNDARY_AUTH_PR_MERGED_OK
- NA0550_CLOSEOUT_D1090_ACCEPTED_OK
- NA0550_CLOSEOUT_PUBLIC_SAFETY_GREEN_OK
- NA0550_CLOSEOUT_ADVISORIES_GREEN_OK
- NA0550_CLOSEOUT_NO_FAILED_REQUIRED_CHECKS_OK
- NA0550_CLOSEOUT_D1091_RESTORED_NA0551_OK
- NA0550_CLOSEOUT_SELECTED_SUCCESSOR_EXACT_OK
- NA0550_CLOSEOUT_NO_NA0551_IMPLEMENTATION_OK
- NA0550_CLOSEOUT_NO_QSL_SERVER_COMMAND_OK
- NA0550_CLOSEOUT_NO_QSL_SERVER_CLONE_BUILD_RUN_OK
- NA0550_CLOSEOUT_NO_QSL_ATTACHMENTS_COMMAND_OK
- NA0550_CLOSEOUT_NO_QSL_ATTACHMENTS_CLONE_BUILD_RUN_OK
- NA0550_CLOSEOUT_NO_RERUN_EXECUTED_OK
- NA0550_CLOSEOUT_NO_WORKFLOW_DISPATCH_OK
- NA0550_CLOSEOUT_NO_LOCAL_REPRODUCTION_OK
- NA0550_CLOSEOUT_NO_WORKFLOW_MUTATION_OK
- NA0550_CLOSEOUT_NO_RUNTIME_MUTATION_OK
- NA0550_CLOSEOUT_NO_QSC_SOURCE_MUTATION_OK
- NA0550_CLOSEOUT_NO_DEPENDENCY_LOCKFILE_MUTATION_OK
- NA0550_CLOSEOUT_NO_QSL_SERVER_ATTACHMENTS_MUTATION_OK
- NA0550_CLOSEOUT_NO_PUBLIC_SITE_MUTATION_OK
- NA0550_CLOSEOUT_NO_CLOUDFLARE_MUTATION_OK
- NA0550_CLOSEOUT_NO_PRIVATE_MATERIAL_PUBLISHED_OK
- NA0550_CLOSEOUT_NO_PUBLIC_READINESS_CLAIM_OK
- NA0550_CLOSEOUT_NO_PRODUCTION_READINESS_CLAIM_OK
- NA0550_CLOSEOUT_ONE_READY_INVARIANT_OK

## Acceptance Criteria

- NA-0550 implementation PR #1373 is merged.
- D-1090 exists once and is Accepted.
- D-1091 exists once after closeout.
- D-1092 is absent.
- NA-0550 is DONE.
- READY_COUNT is 1.
- READY item is NA-0551.
- The NA-0551 successor block exactly matches the D-1090-selected remote smoke
  exact remediation successor.
- No NA-0551 implementation occurred.
- No qsl-server/qsl-attachments command, clone, build, run, or mutation
  occurred.
- No rerun, workflow dispatch, local reproduction, workflow/runtime/qsc
  source/dependency mutation, public-site mutation, Cloudflare mutation, raw log
  commit, raw artifact commit, or private-material publication occurred.

## Validation Commands

- `git diff --check`
- exact five-path closeout scope guard
- queue/decision proof for READY_COUNT 1, READY NA-0551, NA-0550 DONE,
  D-1090 once, D-1091 once, D-1092 absent, and duplicate decision count zero
- marker proof
- link-check
- private-material scan
- overclaim scan
- docs/governance-only classifier
- PR body preflight
- goal-lint if available
- `cargo audit`
- nested qsc fuzz lock cargo audit
- `cargo fmt --check`
- `sh -n scripts/ci/qsc_adversarial.sh`
- `bash -n scripts/ci/qsc_adversarial.sh`
