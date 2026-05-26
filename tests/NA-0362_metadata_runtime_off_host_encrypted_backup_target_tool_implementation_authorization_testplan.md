Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-05-25

# NA-0362 Metadata Runtime Off-Host Encrypted Backup Target Tool Implementation Authorization Testplan

## Objective

Validate that NA-0362 records an off-host encrypted backup target/tool
implementation authorization decision without performing implementation. The
lane must refresh prior no-secret restore and key custody/recovery evidence,
refresh qsl-server/qsl-attachments source authority, refresh local
backup/tool/key/off-host/restore state, select a safe future no-secret
target/tool implementation harness or exact blocker, and preserve all
no-secret/no-operation/no-network boundaries.

## Protected invariants

- NA-0362 is qsl-protocol governance/authorization only.
- qsl-protocol is the only mutable repository.
- qsl-server and qsl-attachments are read-only.
- qshield runtime, qsc, qsp, protocol, crypto, key schedules, dependencies,
  workflows, README, START_HERE, docs/public, website, branch protection,
  public-safety configuration, backup scripts, timers, fstab, service units,
  restore targets, off-host targets, deployment scripts, rollback scripts,
  local backup configuration, and production service behavior are not changed.
- No real off-host setup, remote connection, repository initialization, tool
  installation, backup, restore, restore target creation/mount/copy, deploy,
  rollback, key generation, key upload, passphrase collection, private-key
  inspection, recovery-envelope content creation, or secret material handling
  occurs.
- Local continuity backup is not complete disaster recovery.
- Off-host encrypted backup is not complete.
- Dry-run restore evidence is not real restore execution.
- No-secret custody/recovery evidence is not real key custody or real key
  recovery implementation.
- Future no-secret target/tool evidence must not be presented as real off-host
  backup.
- qsl-server PR #56 remains bounded end-to-end harness evidence only.
- qsl-attachments PR #37 remains service-local prerequisite evidence only.
- qshield embedded relay/demo evidence remains reference/oracle evidence only.

## Allowed scope

- `docs/governance/evidence/NA-0362_metadata_runtime_off_host_encrypted_backup_target_tool_implementation_authorization.md`
- `tests/NA-0362_metadata_runtime_off_host_encrypted_backup_target_tool_implementation_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Forbidden scope

- qsl-server and qsl-attachments mutation.
- qshield runtime, qsc/qsp, protocol, crypto, key-schedule, dependency,
  workflow, website, README, START_HERE, docs/public, service, deployment,
  backup script, timer, fstab, source-list, off-host destination, restore,
  rollback, purge, branch-protection, or public-safety configuration mutation.
- Backup execution, restore execution, restore target creation, restore target
  mount, restore payload copy, key generation, key upload, passphrase
  collection, private key inspection, secret material handling,
  recovery-envelope content creation, repository initialization, remote/off-host
  backup tooling, deploy, rollback, or tool installation.

## Prior off-host target/tool readiness review requirements

Evidence must review:

- live NA-0362 queue scope;
- NA-0361 no-secret key custody / key recovery harness;
- NA-0359 no-secret restore-drill dry-run harness;
- NA-0355 target/tool class selection;
- qsl-server PR #56 and qsl-attachments PR #37 boundaries;
- local backup/tool/key/off-host/restore posture;
- public-claim boundaries.

## Source/authority refresh requirements

Refresh qsl-server and qsl-attachments read-only and record local path/SHA,
remote default branch SHA, PR #56 / PR #37 merge status, latest main CI status,
viewer permission, branch protection, open PR list, and classifications:

- `FRESH_SOURCE` or exact stale/unknown status;
- `COMPLETE_AUTHORITY` or exact authority blocker;
- `COMPLETE_CI` or exact CI blocker.

## Local backup/tool/key/off-host/restore evidence refresh requirements

Record `/backup/qsl` mount status, snapshot list, manifests/logs availability,
backup status file, backup plan file, safe `qsl-backup` syntax/preflight/list
results, installed availability for `restic`, `borg`, `rclone`, `age`, `gpg`,
`ssh`, and `rsync`, restore dry-run harness status, real restore status, key
custody/recovery status, recovery envelope status, local history folder
presence, backup coverage, and D132 protection status.

Required classification:

- `LOCAL_CONTINUITY_PROVEN`
- `NO_SECRET_DRY_RUN_RESTORE_PROVEN`
- `NO_SECRET_KEY_CUSTODY_RECOVERY_PROVEN`
- `REAL_KEY_CUSTODY_NOT_READY`
- `REAL_KEY_RECOVERY_NOT_READY`
- `OFF_HOST_TARGET_NOT_READY`
- `OFF_HOST_TOOL_NOT_READY`
- `NO_SECRET_TARGET_TOOL_HARNESS_READY_FOR_AUTHORIZATION`
- `REAL_RESTORE_NOT_AUTHORIZED`

## Target authorization requirements

Evidence must record all target decision categories:

- `NO_SECRET_TARGET_HARNESS_AUTHORIZATION_READY`
- `REAL_OFF_HOST_TARGET_IMPLEMENTATION_BLOCKED_BACKUP_PLAN`
- `REAL_OFF_HOST_TARGET_IMPLEMENTATION_BLOCKED_LOCAL_OPS`
- `REAL_OFF_HOST_TARGET_IMPLEMENTATION_BLOCKED_SECRET_HANDLING`
- `REAL_OFF_HOST_TARGET_IMPLEMENTATION_BLOCKED_OPERATOR_RUNBOOK`
- `REAL_OFF_HOST_TARGET_IMPLEMENTATION_BLOCKED_RESTORE_DRILL`
- `REAL_OFF_HOST_TARGET_IMPLEMENTATION_BLOCKED_TARGET_ACCESS`
- `REAL_OFF_HOST_TARGET_IMPLEMENTATION_DEFERRED`

The selected no-secret target harness must be fixture-only and must not connect
to a remote or configure a real target.

## Tool authorization requirements

Evidence must record all tool decision categories:

- `NO_SECRET_TOOL_HARNESS_AUTHORIZATION_READY`
- `REAL_TOOL_IMPLEMENTATION_BLOCKED_INSTALLATION`
- `REAL_TOOL_IMPLEMENTATION_BLOCKED_BACKUP_PLAN`
- `REAL_TOOL_IMPLEMENTATION_BLOCKED_LOCAL_OPS`
- `REAL_TOOL_IMPLEMENTATION_BLOCKED_SECRET_HANDLING`
- `REAL_TOOL_IMPLEMENTATION_BLOCKED_RESTORE_DRILL`
- `REAL_TOOL_IMPLEMENTATION_DEFERRED`

The selected no-secret tool harness must be fixture-only and must not install
or run a real backup tool.

## Future no-secret bundle requirements

Evidence must define future repo, allowed files, forbidden files, commands,
tests, artifacts, markers, PR/order, backup-plan update requirement,
key-handling requirement, restore dependency requirement, monitoring/logging
requirement, public-claim boundary, and stop conditions.

## Real target/tool blocker requirements

Evidence must analyze why real SSH/SFTP target setup, external disk target, NAS
target, object storage target, cloud bucket target, offline media target,
repository init, repository check/snapshot/prune/restore, retention/purge,
monitoring/alerting, target identity verification, and credentials/secrets are
forbidden or blocked in NA-0362.

## Simulated fixture requirements

Evidence must compare:

- qsl-protocol simulated SSH/SFTP target fixture;
- qsl-protocol simulated restic-style repository fixture;
- qsl-protocol simulated snapshot/check/prune/restore matrix;
- qsl-protocol simulated retention/monitoring matrix;
- qsl-server/qsl-attachments service-local fixture harness;
- no no-secret implementation.

The selected option must be qsl-protocol-only and no-secret.

## Repository init / remote connection blocker requirements

Evidence must state whether tool installation, repository initialization,
remote connection, target identity verification, repository check/prune/restore,
and no-secret modeling can proceed now. Real operations must remain blocked.

## Key dependency requirements

Evidence must state that the no-secret target/tool harness can proceed after
NA-0361, while real target/tool implementation requires real key custody and
real key recovery before reliance.

## Restore/retention/monitoring dependency requirements

Evidence must state that real off-host target/tool work requires restore-drill
planning and later isolated restore evidence; retention/prune/check/restore and
monitoring/alerting may be modeled no-secret but real setup remains blocked.

## Backup-plan impact requirements

Evidence must state that NA-0362 and future pure qsl-protocol no-secret harness
work do not require a backup-plan update if proof remains temporary under
`/srv/qbuild/tmp`, and that real targets, repositories, tools, credentials,
durable artifacts, restore targets, monitoring artifacts, scripts, timers,
fstab, services, backup, restore, deploy, rollback, key material, recovery
envelopes, or public-claim mutation do require backup-plan and local-ops
authorization first.

## Public-ingress/timing/traffic-shape boundary requirements

Evidence must state that NA-0362 changes no public ingress and does not prove
hidden attachment size, hidden timing metadata, hidden traffic shape, hidden
all metadata, or padding that hides all metadata.

## External-review boundary requirements

Evidence must state that external review remains incomplete and that any
stronger claim requires real key custody, real key recovery, off-host backup,
real restore drill, service, deployment, monitoring/log, rollback, and review
evidence.

## Claim-boundary requirements

The authorization must not claim:

- production readiness;
- public-internet readiness;
- completed external review;
- anonymous operation;
- metadata-free behavior;
- untraceable behavior;
- hidden attachment size;
- hidden timing metadata;
- hidden traffic shape;
- complete disaster recovery;
- complete off-host backup;
- real restore completion;
- real key custody implementation;
- real key recovery implementation.

## Workflow-support deferral requirements

Evidence must record whether workflow support and history indexing would reduce
friction, but must not implement local workflow support in NA-0362.

## Required local checks

Run:

```bash
git status --porcelain=v1 --branch
git diff --name-only origin/main...HEAD
git diff --check
cargo audit --deny warnings
cargo tree -i rustls-webpki --locked
cargo fmt --check
bash -n scripts/ci/metadata_runtime_production_backup_deploy_rollback_harness.sh
bash scripts/ci/metadata_runtime_production_backup_deploy_rollback_harness.sh inputs/metadata_runtime/production_backup_deploy_rollback_fixture_v1.json
python3 -m json.tool inputs/metadata_runtime/production_backup_deploy_rollback_fixture_v1.json >/dev/null
bash -n scripts/ci/metadata_runtime_restore_drill_dry_run_harness.sh
python3 -m json.tool inputs/metadata_runtime/restore_drill_dry_run_fixture_v1.json >/dev/null
bash scripts/ci/metadata_runtime_restore_drill_dry_run_harness.sh inputs/metadata_runtime/restore_drill_dry_run_fixture_v1.json
bash -n scripts/ci/metadata_runtime_key_custody_recovery_no_secret_harness.sh
python3 -m json.tool inputs/metadata_runtime/key_custody_recovery_no_secret_fixture_v1.json >/dev/null
bash scripts/ci/metadata_runtime_key_custody_recovery_no_secret_harness.sh inputs/metadata_runtime/key_custody_recovery_no_secret_fixture_v1.json
bash scripts/ci/metadata_runtime_identifier_padding_harness_plan.sh inputs/metadata_runtime/identifier_padding_runtime_fixture_v1.json
bash scripts/ci/metadata_phase2_identifier_padding_harness.sh inputs/metadata_phase2/identifier_padding_policy_vectors_v1.json
bash scripts/ci/metadata_phase2_sanitized_errors_retention_harness.sh inputs/metadata_phase2/sanitized_errors_retention_policy_vectors_v1.json
bash scripts/ci/metadata_conformance_smoke.sh
cargo +stable test -p qshield-cli --locked -- --test-threads=1
cargo +stable build -p qshield-cli --locked
scripts/ci/demo_cli_smoke.sh
DEMO_STRESS_PROFILE=baseline scripts/ci/demo_adversarial_stress.sh
DEMO_SOAK_RUNS=3 scripts/ci/demo_soak_repeated_run.sh
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
python3 formal/model_qsc_handshake_suite_id_bounded.py
python3 formal/run_model_checks.py
python3 -m json.tool inputs/suite2/qsc_handshake_suite_id_vectors_na0310.json >/dev/null
cargo +stable test -p quantumshield_refimpl --test na_0310_qsc_suite_id_vector_oracle --locked -- --test-threads=1
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main \
  --allowed docs/governance/evidence/NA-0362_metadata_runtime_off_host_encrypted_backup_target_tool_implementation_authorization.md \
  --allowed tests/NA-0362_metadata_runtime_off_host_encrypted_backup_target_tool_implementation_authorization_testplan.md \
  --allowed DECISIONS.md \
  --allowed TRACEABILITY.md \
  --allowed docs/ops/ROLLING_OPERATIONS_JOURNAL.md
python3 scripts/ci/qsl_evidence_helper.py link-check
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
bash scripts/ci/classify_ci_scope.sh docs/governance/evidence/NA-0362_metadata_runtime_off_host_encrypted_backup_target_tool_implementation_authorization.md tests/NA-0362_metadata_runtime_off_host_encrypted_backup_target_tool_implementation_authorization_testplan.md DECISIONS.md TRACEABILITY.md docs/ops/ROLLING_OPERATIONS_JOURNAL.md
```

Optional harnesses may be reported as not feasible if they do not complete
under an explicit local bound and the core required gate set remains green.

## CI expectations

- PR goal-lint passes with `Goals: G1, G2, G3, G4, G5`.
- Required checks complete normally.
- `public-safety` remains required and green.
- Merge uses a normal merge commit with `--match-head-commit`.
- No admin bypass, squash, rebase, direct push, delete-branch flag, branch
  deletion command, branch-protection mutation, or public-safety mutation is
  used.

## Successor handoff

If Packet S merges and public-safety is green, a separate closeout directive may
mark NA-0362 DONE and restore exactly one successor:

`NA-0363 -- Metadata Runtime Off-Host Encrypted Backup Target / Tool No-Secret Implementation Harness`

The closeout must not implement NA-0363.
