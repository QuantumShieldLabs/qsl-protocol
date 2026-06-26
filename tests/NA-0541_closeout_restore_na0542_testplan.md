Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-25
Replaces: n/a
Superseded-By: n/a

# NA-0541 Closeout and NA-0542 Restoration Testplan

## Scope

This closeout accepts the merged NA-0541 implementation, verifies post-merge
public-safety and advisories, marks NA-0541 DONE, and restores NA-0542 as the
sole READY successor. It does not implement NA-0542.

## Required Markers

- NA0541_CLOSEOUT_IMPLEMENTATION_PR_MERGED_OK
- NA0541_CLOSEOUT_D1072_ACCEPTED_OK
- NA0541_CLOSEOUT_PUBLIC_SAFETY_GREEN_OK
- NA0541_CLOSEOUT_ADVISORIES_GREEN_OK
- NA0541_CLOSEOUT_FIRST_PROGRESS_ENTRY_PUBLISHED_OK
- NA0541_CLOSEOUT_SITE_ACCURACY_SWEEP_ACCEPTED_OK
- NA0541_CLOSEOUT_NO_MATERIAL_OUT_OF_SCOPE_CORRECTION_OK
- NA0541_CLOSEOUT_D1073_RESTORED_NA0542_OK
- NA0541_CLOSEOUT_NO_NA0542_IMPLEMENTATION_OK
- NA0541_CLOSEOUT_NO_LOCAL_OPS_MUTATION_OK
- NA0541_CLOSEOUT_NO_REMOTE_ACTION_OK
- NA0541_CLOSEOUT_NO_QSL_SERVER_ATTACHMENTS_OK
- NA0541_CLOSEOUT_NO_PUBLIC_READINESS_CLAIM_OK
- NA0541_CLOSEOUT_NO_PRODUCTION_READINESS_CLAIM_OK
- NA0541_CLOSEOUT_ONE_READY_INVARIANT_OK

## Validation Commands

- exact closeout path scope guard
- queue/decision proof
- D-1072 accepted proof
- D-1073 once proof
- duplicate decision count proof
- no NA-0542 implementation proof
- no local-ops mutation proof
- no public-content mutation proof
- link-check
- private-material scan
- added-line overclaim scan
- PR body preflight
- goal-lint
- `cargo audit --deny warnings`
- `cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock`
- `cargo fmt --check`

## Expected Result

NA-0541 is DONE, NA-0542 is the sole READY item, D-1072 and D-1073 each exist
once, D-1074 is absent, duplicate decision count is zero, and no NA-0542
implementation or forbidden mutation occurs.
