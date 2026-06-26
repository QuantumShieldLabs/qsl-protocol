Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-26

# NA-0542 Closeout Restore NA-0543 Testplan

Goals: G1, G2, G3, G4, G5

## Objective

Validate that NA-0542 authorization PR #1357 merged, D-1074 is accepted, main
post-merge public-safety and advisories are green, NA-0542 is marked DONE, and
the exact D-1074-selected NA-0543 implementation harness is restored as the sole
READY item without implementing NA-0543.

## Required Markers

- `NA0542_CLOSEOUT_AUTHORIZATION_PR_MERGED_OK`
- `NA0542_CLOSEOUT_D1074_ACCEPTED_OK`
- `NA0542_CLOSEOUT_PUBLIC_SAFETY_GREEN_OK`
- `NA0542_CLOSEOUT_ADVISORIES_GREEN_OK`
- `NA0542_CLOSEOUT_EXACT_PATH_BUNDLE_OK`
- `NA0542_CLOSEOUT_OPERATOR_BOUNDARY_OK`
- `NA0542_CLOSEOUT_D1075_RESTORED_NA0543_OK`
- `NA0542_CLOSEOUT_NO_NA0543_IMPLEMENTATION_OK`
- `NA0542_CLOSEOUT_NO_QWORK_EXECUTION_OK`
- `NA0542_CLOSEOUT_NO_QSL_BACKUP_EXECUTION_OK`
- `NA0542_CLOSEOUT_NO_LOCAL_OPS_MUTATION_OK`
- `NA0542_CLOSEOUT_NO_REMOTE_ACTION_OK`
- `NA0542_CLOSEOUT_NO_QSL_SERVER_ATTACHMENTS_OK`
- `NA0542_CLOSEOUT_ONE_READY_INVARIANT_OK`

## Validation Commands

- `git diff --check`
- Exact five-path closeout scope guard including untracked files.
- Queue/decision proof:
  - READY_COUNT 1.
  - READY NA-0543.
  - NA-0542 DONE.
  - D-1074 once.
  - D-1075 once.
  - D-1076 absent.
  - duplicate decision count zero.
- Link-check.
- Added-line/new-file private-material scan.
- Added-line/new-file overclaim scan.
- Docs-only classifier.
- Marker proof.
- PR body preflight.
- Goal-lint.
- Root cargo audit.
- Nested qsc fuzz lock cargo audit.
- `cargo fmt --check`.
- `sh -n scripts/ci/qsc_adversarial.sh`.
- `bash -n scripts/ci/qsc_adversarial.sh`.

## Expected Result

NA-0542 closeout passes if the only repository mutations are:

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0542_closeout_restore_na0543_testplan.md`

NA-0542 closeout fails if it mutates:

- NA-0543 tracked implementation paths;
- local maintenance scripts;
- systemd service or timer files;
- qwork/qbuild source or configuration;
- shell profiles;
- Cargo configuration;
- shared target directories;
- target symlinks;
- qbuild tmp, archive, backup, or cache contents;
- qsl-backup state;
- qsc source/test/fuzz/Cargo paths;
- dependency or lockfile paths;
- workflow paths;
- public Progress or website content;
- qsl-server or qsl-attachments paths.

## Boundary Assertions

- Authorization PR #1357 is merged.
- D-1074 is accepted.
- post-merge public-safety is green.
- post-merge advisories is green.
- NA-0542 is DONE.
- NA-0543 is READY as the sole successor.
- The exact D-1074 tracked path bundle is restored.
- The exact D-1074 operator-owned path bundle is restored.
- No NA-0543 implementation occurs.
- No qwork, qstart, or qresume is executed by Codex.
- No qsl-backup execution occurs.
- No local-ops mutation occurs.
- No remote action occurs.
- No qsl-server or qsl-attachments use or mutation occurs.
- Exactly one READY remains.

## Focused Runtime Test Skip Rationale

Focused qsc runtime tests may be skipped because closeout mutates only
governance/queue/testplan files, does not implement NA-0543, does not mutate qsc
runtime/source/dependency/workflow paths, and the authorization PR post-merge
public-safety/advisories were green before closeout.
