Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-25

# NA-0540 Closeout and NA-0541 Restoration Test Plan

## Purpose

Record the closeout proof for NA-0540 after authorization PR #1353 merged and
post-merge public-safety/advisories completed success. This closeout marks
NA-0540 DONE and restores NA-0541 as the sole READY successor. It does not
implement NA-0541, publish daily Progress content, correct public pages, or
mutate runtime/source/dependency/workflow/local-ops paths.

## Required Markers

- NA0540_CLOSEOUT_PR1353_MERGED_OK
- NA0540_CLOSEOUT_D1070_ACCEPTED_OK
- NA0540_CLOSEOUT_PUBLIC_SAFETY_GREEN_OK
- NA0540_CLOSEOUT_ADVISORIES_GREEN_OK
- NA0540_CLOSEOUT_D1071_RESTORED_NA0541_OK
- NA0540_CLOSEOUT_EXACT_D1070_PATH_BUNDLE_OK
- NA0540_CLOSEOUT_NO_NA0541_IMPLEMENTATION_OK
- NA0540_CLOSEOUT_NO_PUBLIC_DOC_MUTATION_OK
- NA0540_CLOSEOUT_NO_PUBLIC_CORRECTION_IMPLEMENTATION_OK
- NA0540_CLOSEOUT_NO_LOCAL_OPS_MUTATION_OK
- NA0540_CLOSEOUT_NO_QSC_SOURCE_MUTATION_OK
- NA0540_CLOSEOUT_NO_WORKFLOW_DEPENDENCY_MUTATION_OK
- NA0540_CLOSEOUT_NO_QSL_SERVER_ATTACHMENTS_OK
- NA0540_CLOSEOUT_NO_PUBLIC_READINESS_CLAIM_OK
- NA0540_CLOSEOUT_NO_PRODUCTION_READINESS_CLAIM_OK
- NA0540_CLOSEOUT_NO_CRYPTO_COMPLETE_CLAIM_OK
- NA0540_CLOSEOUT_ONE_READY_INVARIANT_OK

## Evidence Requirements

- PR #1353 merged at `ea38f150368e`.
- D-1070 exists once and records
  `DAILY_PUBLIC_PROGRESS_SITE_ACCURACY_IMPLEMENTATION_READY`.
- Post-merge `advisories` completed success for `ea38f150368e`.
- Post-merge `public-safety` completed success for `ea38f150368e`.
- D-1071 exists once.
- `NEXT_ACTIONS.md` marks NA-0540 DONE.
- `NEXT_ACTIONS.md` restores exactly one READY item, NA-0541.
- The NA-0541 block repeats the exact D-1070 path bundle and does not include
  wildcard mutation authority.
- No README, docs/public, public Progress entry, public correction, local-ops,
  qsc source/test/fuzz/Cargo, dependency/lockfile, workflow/script/helper,
  corpus/vector/input, formal/refimpl/service/public/backup, `public/`,
  `website/`, qsl-server, or qsl-attachments mutation occurs in this closeout.

## Scope Guard

The only closeout mutation paths are:

- NEXT_ACTIONS.md
- DECISIONS.md
- TRACEABILITY.md
- docs/ops/ROLLING_OPERATIONS_JOURNAL.md
- tests/NA-0540_closeout_restore_na0541_testplan.md

All other changed paths are a failure for this closeout.

## Queue and Decision Proof

Required queue state after the closeout patch:

- READY_COUNT 1.
- READY item: NA-0541.
- NA-0540 DONE.
- NA-0539 DONE.
- NA-0538 DONE.
- D-1070 count: 1.
- D-1071 count: 1.
- Duplicate decision count: 0.

## Validation Commands

- `git diff --check`
- exact five-path closeout scope guard, including untracked files
- queue/decision proof
- marker proof for all required markers
- link-check
- added-line/new-file private-material scan
- added-line/new-file overclaim scan
- docs-only classifier
- PR body preflight
- goal-lint
- root `cargo audit --deny warnings`
- nested qsc fuzz lock `cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock`
- `cargo fmt --check`
- `sh -n scripts/ci/qsc_adversarial.sh`
- `bash -n scripts/ci/qsc_adversarial.sh`

Focused qsc runtime tests may be skipped because this closeout is
governance-only and does not mutate runtime/source/dependency/workflow paths.

## Claim Boundary

This closeout does not claim public readiness, production readiness,
public-internet readiness, external-review-complete status, crypto completeness,
identity completeness, trust completeness, replay-proof status,
downgrade-proof status, secret-material completeness, side-channel freedom,
vulnerability freedom, bug freedom, or perfect crypto.
