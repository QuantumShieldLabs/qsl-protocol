Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-28

# NA-0552 Closeout and NA-0553 Restoration Testplan

This closeout testplan records that NA-0552 authorization was merged and that
the D-1094-selected NA-0553 diagnostic instrumentation authorization successor
is restored as the exactly one READY item. It does not authorize NA-0553
implementation.

## Required Markers

- NA0552_CLOSEOUT_AUTHORIZATION_PR_MERGED_OK
- NA0552_CLOSEOUT_D1094_ACCEPTED_OK
- NA0552_CLOSEOUT_PUBLIC_SAFETY_GREEN_OK
- NA0552_CLOSEOUT_ADVISORIES_GREEN_OK
- NA0552_CLOSEOUT_NO_FAILED_REQUIRED_CHECKS_OK
- NA0552_CLOSEOUT_D1095_RESTORED_NA0553_OK
- NA0552_CLOSEOUT_SELECTED_SUCCESSOR_EXACT_OK
- NA0552_CLOSEOUT_NO_NA0553_IMPLEMENTATION_OK
- NA0552_CLOSEOUT_NO_RERUN_EXECUTED_OK
- NA0552_CLOSEOUT_NO_WORKFLOW_DISPATCH_OK
- NA0552_CLOSEOUT_NO_LOCAL_REPRODUCTION_OK
- NA0552_CLOSEOUT_NO_SCRIPT_REMEDIATION_OK
- NA0552_CLOSEOUT_NO_WORKFLOW_MUTATION_OK
- NA0552_CLOSEOUT_NO_QSC_SOURCE_MUTATION_OK
- NA0552_CLOSEOUT_NO_DEPENDENCY_LOCKFILE_MUTATION_OK
- NA0552_CLOSEOUT_NO_QSL_SERVER_ATTACHMENTS_OK
- NA0552_CLOSEOUT_NO_PUBLIC_SITE_MUTATION_OK
- NA0552_CLOSEOUT_NO_CLOUDFLARE_MUTATION_OK
- NA0552_CLOSEOUT_NO_RAW_LOGS_COMMITTED_OK
- NA0552_CLOSEOUT_NO_PRIVATE_MATERIAL_PUBLISHED_OK
- NA0552_CLOSEOUT_NO_PUBLIC_READINESS_CLAIM_OK
- NA0552_CLOSEOUT_NO_PRODUCTION_READINESS_CLAIM_OK
- NA0552_CLOSEOUT_ONE_READY_INVARIANT_OK

## Validation Gates

- `git diff --check`
- exact five-path closeout scope guard, including untracked-file check
- queue/decision proof: READY_COUNT 1, READY NA-0553, NA-0552 DONE, D-1094
  once, D-1095 once, D-1096 absent, duplicate decision count zero
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

Focused qsc runtime tests are intentionally skipped because this closeout is
governance-only, no qsc source/runtime/dependency/workflow path is mutated, and
local reproduction or qsc send/receive is forbidden.

## Expected Result

`NA-0552 DONE; NA-0553 READY; D-1095 accepted once; one READY invariant held.`
