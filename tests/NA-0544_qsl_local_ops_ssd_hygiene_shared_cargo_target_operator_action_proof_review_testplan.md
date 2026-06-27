Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-26

# NA-0544 QSL Local Ops SSD Hygiene Shared Cargo Target Operator Action Proof Review Testplan

## Objective

Validate that NA-0544 performs a read-only proof review of the human operator
action bundle execution, records D-1078, and preserves the operator/Codex,
qsc, dependency, workflow, remote, qsl-server, qsl-attachments, backup, and
claim boundaries.

## Required Markers

- `NA0544_D1076_OPERATOR_BUNDLE_CONSUMED_OK`
- `NA0544_D1077_CLOSEOUT_CONSUMED_OK`
- `NA0544_FRESH_QWORK_PROOF_OK`
- `NA0544_OPERATOR_ACTION_TRANSCRIPT_REVIEWED_OK`
- `NA0544_ROLLBACK_INVENTORY_VERIFIED_OK`
- `NA0544_INSTALLED_MAINTENANCE_SCRIPT_VERIFIED_OK`
- `NA0544_INSTALLED_SYSTEMD_UNITS_VERIFIED_OK`
- `NA0544_INSTALLED_QWORK_ENV_WRAPPER_VERIFIED_OK`
- `NA0544_SHARED_TARGET_DIRECTORY_VERIFIED_OK`
- `NA0544_QWORK_SHARED_TARGET_PROOF_OK`
- `NA0544_EXPLICIT_TARGET_PRESERVED_OK`
- `NA0544_UNRELATED_REPO_REJECTED_OK`
- `NA0544_POST_INSTALL_DRY_RUN_OK`
- `NA0544_MAINTENANCE_JSON_SUMMARY_OK`
- `NA0544_TIMER_ENABLED_ACTIVE_OK`
- `NA0544_LATEST_MAINTENANCE_STATE_CLASSIFIED_OK`
- `NA0544_NO_CODEX_OPERATOR_ACTION_EXECUTED_OK`
- `NA0544_NO_CODEX_QWORK_EXECUTION_OK`
- `NA0544_NO_CODEX_QSL_BACKUP_EXECUTION_OK`
- `NA0544_NO_CODEX_BACKUP_MUTATION_OK`
- `NA0544_NO_QSC_SOURCE_MUTATION_OK`
- `NA0544_NO_REMOTE_ACTION_OK`
- `NA0544_NO_QSL_SERVER_ATTACHMENTS_OK`
- `NA0544_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0544_NO_PRODUCTION_READINESS_CLAIM_OK`
- `NA0544_NO_REPRODUCIBILITY_COMPLETE_CLAIM_OK`
- `NA0544_NO_BACKUP_RESTORE_COMPLETE_CLAIM_OK`
- `NA0544_ONE_READY_INVARIANT_OK`

## Validation Gates

- fresh qwork proof-file verification without running qwork/qstart/qresume;
- pre-fetch clean worktree, index, and untracked proof;
- root disk below 95% and `/backup/qsl` mounted;
- current main equals or descends from `b4a64f78efe7`;
- queue proof before patch: READY_COUNT 1, READY NA-0544, NA-0543 DONE,
  NA-0542 DONE;
- decision proof before patch: D-1076 once, D-1077 once, D-1078 absent,
  D-1079 absent, duplicate decision count zero;
- current-main check proof: public-safety success, advisories success, and no
  observed failed required checks;
- D-1076/D-1077 inheritance review;
- rollback inventory hash proof;
- installed maintenance script, unit, qwork/env/wrapper hash and mode proof;
- shared-target directory owner/mode proof;
- qwork target-selection proof;
- explicit-target preservation proof;
- unrelated-repository rejection proof;
- post-install dry-run JSON proof;
- latest maintenance state classification;
- timer enabled/active and service result proof;
- exact five-path implementation scope guard;
- marker proof;
- changed Markdown link check;
- added-line/new-file private-material scan;
- added-line/new-file overclaim scan;
- docs/governance-only classifier;
- PR body preflight;
- goal-lint when locally available;
- root cargo audit;
- nested qsc fuzz lock cargo audit;
- `cargo fmt --check`;
- `sh -n scripts/ci/qsc_adversarial.sh`;
- `bash -n scripts/ci/qsc_adversarial.sh`.

Focused qsc runtime tests may be skipped because this lane changes only
governance/testplan files and does not mutate qsc source/runtime/dependency,
wire, protocol, security implementation, or workflow paths.

## Expected Result

Success classification:

`LOCAL_OPS_SSD_HYGIENE_SHARED_TARGET_OPERATOR_ACTION_PROOF_REVIEW_PASS`

The lane fails if qwork proof is missing or stale, rollback or installed hashes
do not match, shared target state is wrong, explicit-target preservation fails,
unrelated-repo rejection fails, dry-run JSON fails, timer state fails, latest
maintenance state cannot be classified, Codex performs a forbidden operator
action, scope exceeds the exact five implementation paths, or any public,
production, reproducibility, backup/restore, security-freedom, defect-freedom,
or build-perfection claim is introduced.
