Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-29

# NA-0571 Closeout and NA-0572 Restoration Testplan

## Objective

Verify that NA-0571 closes only after D-1132 merged, post-merge checks passed,
and the exact D-1132-selected NA-0572 qsl-server lockfile-only implementation
successor is restored as the sole READY item without implementing NA-0572.

## Required Markers

- NA0571_CLOSEOUT_IMPLEMENTATION_PR_MERGED_OK
- NA0571_CLOSEOUT_D1132_ACCEPTED_OK
- NA0571_CLOSEOUT_PUBLIC_SAFETY_GREEN_OK
- NA0571_CLOSEOUT_ADVISORIES_GREEN_OK
- NA0571_CLOSEOUT_D1133_RESTORED_NA0572_OK
- NA0571_CLOSEOUT_SELECTED_SUCCESSOR_EXACT_OK
- NA0571_CLOSEOUT_NO_NA0572_IMPLEMENTATION_OK
- NA0571_CLOSEOUT_NO_QSL_SERVER_SOURCE_MUTATION_OK
- NA0571_CLOSEOUT_NO_QSL_SERVER_PR_OPENED_OK
- NA0571_CLOSEOUT_NO_REMOTE_ACTION_OK
- NA0571_CLOSEOUT_NO_QSC_SEND_RECEIVE_OK
- NA0571_CLOSEOUT_NO_WORKFLOW_DISPATCH_OK
- NA0571_CLOSEOUT_NO_PRIVATE_MATERIAL_PUBLISHED_OK
- NA0571_CLOSEOUT_ONE_READY_INVARIANT_OK

## Verification Expectations

- NA-0571 implementation PR #1415 is merged.
- D-1132 exists once and is Accepted.
- D-1133 is added exactly once by closeout.
- public-safety completed success after the implementation merge.
- advisories completed success after the implementation merge.
- NA-0571 is marked DONE.
- NA-0572 is restored READY using the exact D-1132-selected successor block.
- NA-0572 implementation does not occur.
- qsl-server source-of-truth is not mutated in closeout.
- No qsl-server PR is opened in closeout.
- No remote action, qsc send/receive, workflow dispatch, private-material
  publication, public-site mutation, or Cloudflare mutation occurs.
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
- nested qsc fuzz `cargo audit --deny warnings`
- `cargo metadata --locked --format-version=1`
- `cargo fmt --check`
- `sh -n scripts/ci/qsc_adversarial.sh`
- `bash -n scripts/ci/qsc_adversarial.sh`

Focused qsc runtime tests may be skipped because this is closeout-only
governance/testplan work with no qsl-protocol source, runtime, dependency,
workflow, executable test, fuzz target, vector, or qsc send/receive mutation.
