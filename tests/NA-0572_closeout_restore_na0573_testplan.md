Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-29

# NA-0572 Closeout and NA-0573 Restoration Testplan

## Objective

Verify that NA-0572 closes only after D-1134 merged, qsl-server recovery is
complete, post-merge checks are green, and the exact D-1134-selected NA-0573
successor is restored as the sole READY item without implementing NA-0573.

## Required Markers

- NA0572_CLOSEOUT_IMPLEMENTATION_PR_MERGED_OK
- NA0572_CLOSEOUT_D1134_ACCEPTED_OK
- NA0572_CLOSEOUT_QSL_SERVER_RECOVERY_PR_MERGED_OR_ALREADY_FIXED_OK
- NA0572_CLOSEOUT_PUBLIC_SAFETY_GREEN_OK
- NA0572_CLOSEOUT_ADVISORIES_GREEN_OK
- NA0572_CLOSEOUT_D1135_RESTORED_NA0573_OK
- NA0572_CLOSEOUT_SELECTED_SUCCESSOR_EXACT_OK
- NA0572_CLOSEOUT_NO_NA0573_IMPLEMENTATION_OK
- NA0572_CLOSEOUT_NO_QSL_SERVER_DEPLOYMENT_OK
- NA0572_CLOSEOUT_NO_QSL_ATTACHMENTS_OK
- NA0572_CLOSEOUT_NO_REMOTE_ACTION_OK
- NA0572_CLOSEOUT_NO_QSC_SEND_RECEIVE_OK
- NA0572_CLOSEOUT_NO_WORKFLOW_DISPATCH_OK
- NA0572_CLOSEOUT_NO_PRIVATE_MATERIAL_PUBLISHED_OK
- NA0572_CLOSEOUT_ONE_READY_INVARIANT_OK

## Verification Expectations

- NA-0572 implementation PR #1417 is merged.
- D-1134 exists once and is Accepted.
- qsl-server PR #57 is merged, or already-fixed proof is accepted.
- Post-merge public-safety completed success.
- Post-merge advisories completed success.
- No failed required checks are present.
- NA-0572 is marked DONE.
- NA-0573 is restored READY using the exact D-1134-selected successor block.
- NA-0573 implementation does not occur.
- No qsl-server deployment or service run occurs in closeout.
- No qsl-attachments work occurs.
- No remote action, qsc send/receive, workflow dispatch/rerun, public-site
  mutation, Cloudflare mutation, or private-material publication occurs.
- Exactly one READY remains mandatory.

## Validation Commands

- `git diff --check`
- exact five-path closeout scope guard
- queue/decision proof
- marker proof
- deterministic markdown link check
- added-line/new-file private-material scan
- overclaim scan
- docs/governance-only classifier
- PR body preflight
- goal-lint if available
- root `cargo audit --deny warnings`
- nested qsc fuzz `cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock`
- `cargo fmt --check`
- `sh -n scripts/ci/qsc_adversarial.sh`
- `bash -n scripts/ci/qsc_adversarial.sh`

Focused qsc runtime tests may be skipped because closeout does not mutate qsc
source/runtime/dependency/workflow paths and qsc send/receive is not
authorized.
