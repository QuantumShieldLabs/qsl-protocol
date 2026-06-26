Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-26

# NA-0543 QSL Local Ops SSD Hygiene Shared Cargo Target Implementation Testplan

## Objective

Validate that NA-0543 implements only the D-1074-authorized tracked artifacts,
generates a dry-run-first operator action bundle and rollback bundle, proves the
qwork shared-target propagation mechanism on copies, and performs no
operator-owned local action.

## Required Markers

- `NA0543_D1074_AUTHORIZATION_CONSUMED_OK`
- `NA0543_LATEST_MAINTENANCE_STATE_RECHECKED_OK`
- `NA0543_CANONICAL_MAINTENANCE_SCRIPT_IMPLEMENTED_OK`
- `NA0543_NEWEST_DESCENDANT_MTIME_POLICY_OK`
- `NA0543_ACTIVE_PROCESS_SAFE_SKIP_OK`
- `NA0543_ARCHIVE_TRANSACTION_VERIFIED_OK`
- `NA0543_JSON_HUMAN_LOGGING_OK`
- `NA0543_SHARED_CACHE_EXCLUDED_OK`
- `NA0543_SHARED_TARGET_HELPER_IMPLEMENTED_OK`
- `NA0543_EXPLICIT_TARGET_PRESERVED_OK`
- `NA0543_ISOLATED_TARGET_EXCEPTIONS_OK`
- `NA0543_QWORK_PROPAGATION_MECHANISM_PROVED_OK`
- `NA0543_PROPOSED_SYSTEMD_UNITS_VALIDATED_OK`
- `NA0543_OPERATOR_ACTION_BUNDLE_COMPLETE_OK`
- `NA0543_ROLLBACK_BUNDLE_COMPLETE_OK`
- `NA0543_FIXTURE_TESTS_PASS_OK`
- `NA0543_EXACT_TRACKED_PATHS_ONLY_OK`
- `NA0543_NO_OPERATOR_ACTION_EXECUTED_OK`
- `NA0543_NO_LOCAL_OPS_INSTALL_MUTATION_OK`
- `NA0543_NO_QWORK_EXECUTION_OK`
- `NA0543_NO_QSL_BACKUP_EXECUTION_OK`
- `NA0543_NO_BACKUP_MUTATION_OK`
- `NA0543_NO_QSC_SOURCE_MUTATION_OK`
- `NA0543_NO_REMOTE_ACTION_OK`
- `NA0543_NO_QSL_SERVER_ATTACHMENTS_OK`
- `NA0543_NO_PRODUCTION_READINESS_CLAIM_OK`
- `NA0543_ONE_READY_INVARIANT_OK`

## Fixture Coverage

Maintenance fixture tests must run only under the proof root and cover:

- default dry-run leaves fixture work/proof roots unchanged;
- apply requires explicit `--apply`;
- live-path protections reject fixture roots under `/srv/qbuild` or `/backup`;
- newest-descendant mtime controls candidate age;
- active-process safe skip avoids self-match;
- mount guard simulation uses a fixture marker;
- target candidate selection finds only per-lane `qsl-protocol/target`;
- proof-root candidate selection ignores archived symlinks;
- shared cache is excluded;
- archive copy/verify/cutover succeeds;
- destination collision fails closed;
- path-preserving symlink verification succeeds;
- interrupted transaction shape leaves recoverable artifacts;
- broken symlink reporting is emitted;
- JSON and human logs are written;
- exit-code classification is distinct;
- log retention prunes old fixture logs.

Shared-target helper tests must run only under the proof root and cover:

- expected qsl-protocol path;
- rustc/toolchain key;
- explicit `CARGO_TARGET_DIR` preservation;
- qwork default only when no explicit value exists;
- unrelated repo safe rejection;
- invalid repo/toolchain/build-class rejection;
- no target creation in print-only mode;
- fixture preparation only under proof root;
- machine-readable proof output;
- isolated-target override behavior.

Proposed qwork/systemd tests must cover:

- proof-root copy patch applies cleanly;
- shell syntax passes;
- deterministic target propagation mechanism is present;
- qwork proof fields are emitted in fixture/simulated flow;
- actual qwork command is never run;
- `systemd-analyze verify` passes if available;
- safe-skip exit is accepted by proposed unit;
- rollback patch applies to proposed copy.

## Static Checks

- `git diff --check`
- exact nine-path allowlist guard including tracked, staged, and untracked files
- queue/decision proof
- fixture/static suite
- `bash -n scripts/local_ops/qbuild-ssd-maintenance.sh`
- `bash -n scripts/local_ops/qbuild-shared-target-env.sh`
- proposed qwork-copy syntax tests
- proposed systemd verification
- operator-bundle safety scan
- rollback-bundle completeness proof
- link-check
- added-line/new-file private-material scan
- added-line/new-file overclaim scan
- docs/tooling classifier
- marker proof
- PR body preflight
- goal-lint
- root cargo audit
- nested qsc fuzz lock cargo audit
- `cargo fmt --check`
- `sh -n scripts/ci/qsc_adversarial.sh`
- `bash -n scripts/ci/qsc_adversarial.sh`

Focused qsc runtime tests may be skipped because this lane makes no qsc
source/runtime/dependency/workflow mutation and local-ops behavior is covered by
proof-root fixture tests.

## Expected Result

Success classification:

`LOCAL_OPS_SSD_HYGIENE_SHARED_TARGET_OPERATOR_ACTION_BUNDLE_READY`

The lane fails if any mutation touches outside the exact D-1074 tracked paths,
if operator-owned paths are mutated, if qwork/qstart/qresume or qsl-backup is
run by Codex, if live maintenance dry-run/apply is run, if the shared target is
created by Codex, if qsc source/Cargo/workflow paths are changed, or if any
public/production/reproducibility/backup-completion claim is introduced.
