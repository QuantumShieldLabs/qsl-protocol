Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-26

# NA-0542 QSL Local Ops SSD Hygiene Shared Cargo Target Authorization Testplan

Goals: G1, G2, G3, G4, G5

## Objective

Validate that NA-0542 is authorization-only, consumes the D454/D453 handoff and
operator cleanup context, inventories installed qbuild SSD maintenance
read-only, classifies the scheduled-run state, inventories qbuild storage and
cache state, reviews the current maintenance safety posture, discovers qwork
and qbuild architecture read-only, selects shared Cargo target policy, selects
exact future implementation paths, preserves isolated-target exceptions, and
introduces no local-ops implementation.

## Required Markers

- `NA0542_D454_HANDOFF_CONSUMED_OK`
- `NA0542_OPERATOR_CLEANUP_CONTEXT_CONSUMED_OK`
- `NA0542_MAINTENANCE_INSTALLATION_INVENTORIED_OK`
- `NA0542_NIGHTLY_RUN_STATE_CLASSIFIED_OK`
- `NA0542_STORAGE_INVENTORY_OK`
- `NA0542_CURRENT_SCRIPT_SAFETY_REVIEWED_OK`
- `NA0542_QWORK_QBUILD_ARCHITECTURE_DISCOVERED_OK`
- `NA0542_SHARED_TARGET_OPTIONS_REVIEWED_OK`
- `NA0542_SHARED_TARGET_DESIGN_SELECTED_OR_DEFERRED_OK`
- `NA0542_ISOLATED_TARGET_EXCEPTIONS_SELECTED_OK`
- `NA0542_ENV_PRECEDENCE_SELECTED_OK`
- `NA0542_CONCURRENCY_LOCKING_POLICY_SELECTED_OK`
- `NA0542_PRESSURE_THRESHOLDS_SELECTED_OK`
- `NA0542_RETENTION_POLICY_SELECTED_OK`
- `NA0542_ARCHIVE_TRANSACTION_POLICY_SELECTED_OK`
- `NA0542_LOGGING_MONITORING_POLICY_SELECTED_OK`
- `NA0542_OPERATOR_CODEX_BOUNDARY_SELECTED_OK`
- `NA0542_EXACT_FUTURE_PATHS_SELECTED_OK`
- `NA0542_ROLLBACK_PLAN_SELECTED_OK`
- `NA0542_NO_IMPLEMENTATION_OK`
- `NA0542_NO_QWORK_EXECUTION_OK`
- `NA0542_NO_QSL_BACKUP_EXECUTION_OK`
- `NA0542_NO_BACKUP_MUTATION_OK`
- `NA0542_NO_QSC_SOURCE_MUTATION_OK`
- `NA0542_NO_REMOTE_ACTION_OK`
- `NA0542_NO_QSL_SERVER_ATTACHMENTS_OK`
- `NA0542_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0542_NO_PRODUCTION_READINESS_CLAIM_OK`
- `NA0542_ONE_READY_INVARIANT_OK`

## Validation Commands

- `git diff --check`
- Exact five-path authorization scope guard including untracked files.
- Queue/decision proof:
  - READY_COUNT 1.
  - READY NA-0542.
  - D-1074 once.
  - D-1075 absent.
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

NA-0542 passes if the only repository mutations are:

- `docs/governance/evidence/NA-0542_qsl_local_ops_ssd_hygiene_shared_cargo_target_authorization_plan.md`
- `tests/NA-0542_qsl_local_ops_ssd_hygiene_shared_cargo_target_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

NA-0542 fails if it mutates:

- `NEXT_ACTIONS.md`;
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

- NA-0542 is authorization-only.
- D454 handoff consumed.
- Operator cleanup context consumed.
- Installed maintenance reviewed read-only.
- Scheduled run state classified.
- Storage inventory completed.
- qwork/qbuild architecture discovered read-only.
- Shared Cargo target design selected.
- Proof-root-isolated target exceptions preserved.
- Operator/Codex boundary selected.
- Exact future paths selected.
- Rollback plan selected.
- No local-ops implementation occurs.
- No maintenance apply occurs.
- No maintenance dry-run occurs.
- No systemd mutation occurs.
- No qwork, qstart, or qresume is executed by Codex.
- No qsl-backup execution occurs.
- No backup mutation occurs.
- No qsc source/test/fuzz/Cargo mutation occurs.
- No dependency or lockfile mutation occurs.
- No workflow mutation occurs.
- No qsl-server or qsl-attachments use or mutation occurs.
- No public-readiness claim is introduced.
- No production-readiness claim is introduced.
- No public-internet-readiness claim is introduced.
- No external-review-complete claim is introduced.
- No reproducibility-complete claim is introduced.
- No backup/restore-complete claim is introduced.
- No vulnerability-free, bug-free, or perfect-build claim is introduced.

## Focused Runtime Test Skip Rationale

Focused qsc runtime tests may be skipped because NA-0542 is authorization-only,
changes only docs/governance/testplan files, does not mutate qsc runtime,
source, dependency, Cargo, workflow, corpus, vector, formal, refimpl, service,
public, backup, qsl-server, or qsl-attachments paths, and current main
public-safety/advisories were green before the authorization patch.
