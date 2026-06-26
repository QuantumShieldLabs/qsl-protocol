Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-26

# NA-0543 Closeout and NA-0544 Restoration Testplan

## Objective

Validate that NA-0543 closeout accepts the merged D-1076 implementation,
records D-1077, marks NA-0543 DONE, restores NA-0544 as the sole READY
successor, and performs no operator action or NA-0544 implementation.

## Required Markers

- `NA0543_CLOSEOUT_IMPLEMENTATION_PR_MERGED_OK`
- `NA0543_CLOSEOUT_D1076_ACCEPTED_OK`
- `NA0543_CLOSEOUT_FRESH_QWORK_PROOF_OK`
- `NA0543_CLOSEOUT_REQUIRED_CHECKS_CLASSIFIED_OK`
- `NA0543_CLOSEOUT_PUBLIC_SAFETY_GREEN_OK`
- `NA0543_CLOSEOUT_ADVISORIES_GREEN_OK`
- `NA0543_CLOSEOUT_NO_FAILED_REQUIRED_CHECKS_OK`
- `NA0543_CLOSEOUT_NON_REQUIRED_FAILURES_FORWARDED_OK`
- `NA0543_CLOSEOUT_OPERATOR_BUNDLE_READY_OK`
- `NA0543_CLOSEOUT_ROLLBACK_BUNDLE_READY_OK`
- `NA0543_CLOSEOUT_D1077_RESTORED_NA0544_OK`
- `NA0543_CLOSEOUT_NO_OPERATOR_ACTION_EXECUTED_OK`
- `NA0543_CLOSEOUT_NO_NA0544_IMPLEMENTATION_OK`
- `NA0543_CLOSEOUT_NO_QWORK_EXECUTION_OK`
- `NA0543_CLOSEOUT_NO_QSL_BACKUP_EXECUTION_OK`
- `NA0543_CLOSEOUT_NO_LOCAL_OPS_INSTALL_MUTATION_OK`
- `NA0543_CLOSEOUT_NO_SHARED_TARGET_MUTATION_OK`
- `NA0543_CLOSEOUT_NO_REMOTE_ACTION_OK`
- `NA0543_CLOSEOUT_NO_QSL_SERVER_ATTACHMENTS_OK`
- `NA0543_CLOSEOUT_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0543_CLOSEOUT_NO_PRODUCTION_READINESS_CLAIM_OK`
- `NA0543_CLOSEOUT_NO_REPRODUCIBILITY_COMPLETE_CLAIM_OK`
- `NA0543_CLOSEOUT_NO_BACKUP_RESTORE_COMPLETE_CLAIM_OK`
- `NA0543_CLOSEOUT_ONE_READY_INVARIANT_OK`

## Validation Gates

- fresh qwork proof-file verification without running qwork/qstart/qresume;
- pre-fetch clean worktree, index, and untracked proof;
- root disk below the 95% hard-stop threshold;
- origin/main equals or descends from PR #1359 merge `717b38ac7d3d`;
- pre-closeout queue proof: READY_COUNT 1, READY NA-0543, NA-0542 DONE;
- pre-closeout decision proof: D-1076 once, D-1077 absent, D-1078 absent,
  duplicate decision count zero;
- current-main required-check classification with public-safety and advisories
  completed success;
- failed non-required remote/relay check-runs recorded as forward-audit
  evidence, not ignored;
- exact five-path closeout scope guard;
- queue proof after patch: READY_COUNT 1, READY NA-0544, NA-0543 DONE;
- decision proof after patch: D-1076 once, D-1077 once, D-1078 absent,
  duplicate decision count zero;
- link-check for changed Markdown files;
- added-line/new-file private-material scan;
- added-line/new-file overclaim scan;
- docs/governance-only classifier;
- PR body preflight and goal-lint when locally available;
- root cargo audit;
- nested qsc fuzz lock cargo audit;
- `cargo fmt --check`;
- `sh -n scripts/ci/qsc_adversarial.sh`;
- `bash -n scripts/ci/qsc_adversarial.sh`.

Focused qsc runtime tests may be skipped because this closeout changes only
governance/testplan files and does not mutate qsc source/runtime/dependency,
wire, protocol, security implementation, or workflow paths.

## Expected Result

Success leaves exactly one READY item:

`NA-0544 -- QSL Local Ops SSD Hygiene / Shared Cargo Target Operator Action Proof Review Harness`

This closeout fails if it performs NA-0544 implementation, executes the
operator action bundle, mutates installed/local paths, mutates systemd state,
runs qwork/qstart/qresume, runs qsl-backup, mutates backup state, creates or
mutates a shared target, touches qsc source/test/fuzz/Cargo paths, changes
dependencies or workflows, performs remote action, uses qsl-server or
qsl-attachments, introduces public-readiness or production-readiness claims, or
leaves more than one READY item.
