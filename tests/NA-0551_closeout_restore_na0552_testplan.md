Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-27

# NA-0551 Closeout and NA-0552 Restoration Testplan

This closeout testplan records that NA-0551 closed as a terminal relay API
boundary stop with no script remediation merge, and that NA-0552 is restored as
the exactly one READY successor. It does not authorize NA-0552 implementation,
reruns, workflow dispatch, local reproduction, qsc send/receive,
qsl-server/qsl-attachments, workflow mutation, qsc source mutation, dependency
changes, public-site mutation, Cloudflare mutation, or private-material
publication.

## Required Markers

- NA0551_CLOSEOUT_STOP_HANDOFF_PR_MERGED_OK
- NA0551_CLOSEOUT_D1092_ACCEPTED_OK
- NA0551_CLOSEOUT_PUBLIC_SAFETY_GREEN_OK
- NA0551_CLOSEOUT_ADVISORIES_GREEN_OK
- NA0551_CLOSEOUT_D1093_RESTORED_NA0552_OK
- NA0551_CLOSEOUT_NO_NA0552_IMPLEMENTATION_OK
- NA0551_CLOSEOUT_NO_SCRIPT_CHANGES_MERGED_OK
- NA0551_CLOSEOUT_RELAY_API_BOUNDARY_RECORDED_OK
- NA0551_CLOSEOUT_NO_RERUN_EXECUTED_OK
- NA0551_CLOSEOUT_NO_WORKFLOW_DISPATCH_OK
- NA0551_CLOSEOUT_NO_LOCAL_REPRODUCTION_OK
- NA0551_CLOSEOUT_NO_WORKFLOW_MUTATION_OK
- NA0551_CLOSEOUT_NO_QSC_SOURCE_MUTATION_OK
- NA0551_CLOSEOUT_NO_DEPENDENCY_LOCKFILE_MUTATION_OK
- NA0551_CLOSEOUT_NO_QSL_SERVER_ATTACHMENTS_OK
- NA0551_CLOSEOUT_NO_PUBLIC_SITE_MUTATION_OK
- NA0551_CLOSEOUT_NO_CLOUDFLARE_MUTATION_OK
- NA0551_CLOSEOUT_NO_PRIVATE_MATERIAL_PUBLISHED_OK
- NA0551_CLOSEOUT_NO_PUBLIC_READINESS_CLAIM_OK
- NA0551_CLOSEOUT_NO_PRODUCTION_READINESS_CLAIM_OK
- NA0551_CLOSEOUT_ONE_READY_INVARIANT_OK

## Validation Gates

- `git diff --check`
- exact five-path closeout scope guard, including untracked-file check
- queue/decision proof: READY_COUNT 1, READY NA-0552, NA-0551 DONE,
  D-1092 once, D-1093 once, D-1094 absent, duplicate decision count zero
- marker proof for every marker listed above
- deterministic markdown link-check
- private-material scan over added lines and new files
- overclaim scan over added lines and PR body
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
local reproduction is forbidden.

## Expected Result

`NA-0551 DONE; NA-0552 READY; D-1093 accepted once; one READY invariant held.`
