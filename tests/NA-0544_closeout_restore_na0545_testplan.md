Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-26

# NA-0544 Closeout and NA-0545 Restoration Testplan

## Objective

Validate that NA-0544 closeout accepts D-1078 after the implementation PR
merged with public-safety and advisories green, marks NA-0544 DONE, records
D-1079, and restores NA-0545 as the sole READY successor without implementing
NA-0545.

## Required Markers

- `NA0544_CLOSEOUT_PROOF_REVIEW_PR_MERGED_OK`
- `NA0544_CLOSEOUT_D1078_ACCEPTED_OK`
- `NA0544_CLOSEOUT_PUBLIC_SAFETY_GREEN_OK`
- `NA0544_CLOSEOUT_ADVISORIES_GREEN_OK`
- `NA0544_CLOSEOUT_D1079_RESTORED_NA0545_OK`
- `NA0544_CLOSEOUT_NO_NA0545_IMPLEMENTATION_OK`
- `NA0544_CLOSEOUT_NO_CODEX_OPERATOR_ACTION_OK`
- `NA0544_CLOSEOUT_NO_CODEX_QWORK_EXECUTION_OK`
- `NA0544_CLOSEOUT_NO_CODEX_QSL_BACKUP_EXECUTION_OK`
- `NA0544_CLOSEOUT_NO_QSC_SOURCE_MUTATION_OK`
- `NA0544_CLOSEOUT_NO_REMOTE_ACTION_OK`
- `NA0544_CLOSEOUT_NO_QSL_SERVER_ATTACHMENTS_OK`
- `NA0544_CLOSEOUT_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0544_CLOSEOUT_NO_PRODUCTION_READINESS_CLAIM_OK`
- `NA0544_CLOSEOUT_ONE_READY_INVARIANT_OK`

## Validation Gates

- NA-0544 proof-review PR merged;
- post-merge public-safety completed success;
- post-merge advisories completed success;
- no observed failed required checks;
- exact five-path closeout scope guard;
- queue/decision proof: READY_COUNT 1, READY NA-0545, NA-0544 DONE,
  D-1078 once, D-1079 once, D-1080 absent, duplicate decision count zero;
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

Focused qsc runtime tests may be skipped because this closeout changes only
governance/testplan files and does not mutate qsc source/runtime/dependency,
wire, protocol, security implementation, or workflow paths.

## Expected Result

Success leaves exactly one READY item:

`NA-0545 -- QSL Remote/Relay Non-Required CI Failure Forward-Audit Authorization Plan`

This closeout fails if it implements NA-0545, executes operator actions, runs
qwork/qstart/qresume, runs qsl-backup, mutates qsc source/dependency/workflow
paths, performs remote action, uses qsl-server or qsl-attachments, introduces
public-readiness or production-readiness claims, or leaves more than one READY
item.
