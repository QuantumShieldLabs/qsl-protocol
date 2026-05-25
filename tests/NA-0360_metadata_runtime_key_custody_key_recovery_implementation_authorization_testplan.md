# NA-0360 Metadata Runtime Key Custody Key Recovery Implementation Authorization Testplan

Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-05-25

## Objective

Validate that NA-0360 records a qsl-protocol-only governance authorization
decision for metadata-runtime key custody and key recovery after NA-0359 dry-run
restore evidence. NA-0360 must select an exact successor or blocker without
implementing real keys, recovery envelopes, off-host backup, restore targets,
runtime behavior, service behavior, dependencies, workflows, public docs, or
public claims.

## Protected invariants

- NA-0360 is authorization planning only.
- qsl-protocol is the only mutable repository.
- qsl-server and qsl-attachments are read-only.
- qshield runtime, qsc, qsp, protocol, crypto, and key-schedule paths are not
  changed.
- No dependency, Cargo, workflow, website, README, START_HERE, docs/public,
  branch-protection, public-safety, backup script, timer, fstab, source-list,
  service configuration, deploy, rollback, restore, real restore target,
  off-host destination, key, passphrase, private-key, or secret-handling path is
  changed or executed.
- Local continuity backup is not complete disaster recovery.
- Off-host encrypted backup is not complete.
- Dry-run restore evidence is not real restore execution.
- Key custody and key recovery are not implemented by NA-0360.

## Allowed scope

- `docs/governance/evidence/NA-0360_metadata_runtime_key_custody_key_recovery_implementation_authorization.md`
- `tests/NA-0360_metadata_runtime_key_custody_key_recovery_implementation_authorization_testplan.md`
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
  collection, private key inspection, secret material handling, repository
  initialization, remote/off-host backup tooling, deploy, or rollback.

## Prior dry-run/key-readiness review requirements

Evidence must review:

- live NA-0360 queue scope;
- NA-0359 dry-run harness and fixture proof;
- NA-0358 restore-drill implementation authorization;
- NA-0356 key custody/recovery prerequisite plan;
- NA-0355 off-host target/tool selection;
- qsl-server PR #56 and qsl-attachments PR #37 boundaries;
- local backup/key/off-host/restore posture;
- public-claim boundaries.

## Source/authority refresh requirements

Refresh qsl-server and qsl-attachments read-only and record local path/SHA,
remote default branch SHA, PR #56 / PR #37 merge status, latest main CI status,
viewer permission, branch protection, open PR list, and classifications:

- `FRESH_SOURCE` or exact stale/unknown status;
- `COMPLETE_AUTHORITY` or exact authority blocker;
- `COMPLETE_CI` or exact CI blocker.

## Local backup/key/off-host/restore evidence refresh requirements

Record `/backup/qsl` mount status, snapshot list, manifests/logs availability,
backup status file, backup plan file, safe `qsl-backup` syntax/preflight/list
results, installed availability for `restic`, `borg`, `rclone`, `age`, `gpg`,
`ssh`, and `rsync`, restore dry-run harness status, real restore status, key
custody/recovery status, recovery envelope status, key rotation status,
emergency access status, incident response status, local history folder
presence, backup coverage, and D132 protection status.

Required classification:

- `LOCAL_CONTINUITY_PROVEN`
- `NO_SECRET_DRY_RUN_RESTORE_PROVEN`
- `REAL_KEY_CUSTODY_NOT_READY`
- `REAL_KEY_RECOVERY_NOT_READY`
- `NO_SECRET_KEY_CUSTODY_HARNESS_READY_FOR_AUTHORIZATION`
- `NO_SECRET_KEY_RECOVERY_HARNESS_READY_FOR_AUTHORIZATION`
- `OFF_HOST_BACKUP_NOT_READY`
- `REAL_RESTORE_NOT_AUTHORIZED`

## Key custody authorization requirements

Evidence must decide:

- whether a no-secret qsl-protocol key custody harness can be authorized now;
- whether real key custody can be authorized now;
- whether recovery envelope work remains blocked or no-secret only;
- whether backup-plan update must precede key material;
- whether local-ops workflow/history coverage must precede real key custody;
- whether future implementation can be staged.

Required categories:

- `NO_SECRET_KEY_CUSTODY_HARNESS_AUTHORIZATION_READY`
- `REAL_KEY_CUSTODY_IMPLEMENTATION_BLOCKED_BACKUP_PLAN`
- `REAL_KEY_CUSTODY_IMPLEMENTATION_BLOCKED_LOCAL_OPS`
- `REAL_KEY_CUSTODY_IMPLEMENTATION_BLOCKED_SECRET_HANDLING`
- `REAL_KEY_CUSTODY_IMPLEMENTATION_BLOCKED_OPERATOR_RUNBOOK`
- `REAL_KEY_CUSTODY_IMPLEMENTATION_BLOCKED_RESTORE_DRILL`
- `KEY_CUSTODY_IMPLEMENTATION_DEFERRED` for real custody only.

## Key recovery authorization requirements

Evidence must decide:

- whether a no-secret qsl-protocol key recovery harness can be authorized now;
- whether real recovery envelope implementation can be authorized now;
- whether old archive compatibility remains modeled only;
- whether restore drill evidence must precede real recovery;
- whether future implementation can be staged.

Required categories:

- `NO_SECRET_KEY_RECOVERY_HARNESS_AUTHORIZATION_READY`
- `REAL_KEY_RECOVERY_IMPLEMENTATION_BLOCKED_BACKUP_PLAN`
- `REAL_KEY_RECOVERY_IMPLEMENTATION_BLOCKED_LOCAL_OPS`
- `REAL_KEY_RECOVERY_IMPLEMENTATION_BLOCKED_SECRET_HANDLING`
- `REAL_KEY_RECOVERY_IMPLEMENTATION_BLOCKED_OPERATOR_RUNBOOK`
- `REAL_KEY_RECOVERY_IMPLEMENTATION_BLOCKED_RESTORE_DRILL`
- `KEY_RECOVERY_IMPLEMENTATION_DEFERRED` for real recovery only.

## Future no-secret bundle requirements

Evidence must define a future no-secret implementation bundle with exact repo,
allowed files, forbidden files, commands, tests, artifacts, markers, PR/order,
backup-plan update requirement, key-handling requirement, restore dependency,
monitoring/logging requirement, public-claim boundary, and stop conditions.

Expected result:

- qsl-protocol fixture and harness only;
- simulated key IDs and simulated recovery-envelope metadata only;
- proof under `/srv/qbuild/tmp/NA-0361_*`;
- no real key generation, key upload, passphrase collection, private-key
  inspection, secret material handling, or real recovery envelope.

## Real key blocker requirements

Evidence must analyze why real key generation, real key storage, real
passphrase collection, real recovery envelope, real operator-held secret, real
hardware token, real split secret, real key rotation, real lost/exposed-key
response, real emergency access, and real restore with secrets are blocked in
NA-0360.

## Simulated fixture requirements

Evidence must evaluate:

- qsl-protocol simulated custody/recovery fixture;
- qsl-protocol marker/check harness;
- incident-response marker harness;
- simulated old-key/rotation matrix;
- qsl-server/qsl-attachments service-local fixture harness;
- rejecting no no-secret implementation.

Each option must record safety, secret risk, backup-plan impact, confidence
gained, confidence not gained, CI feasibility, and recommended/deferred/rejected
status.

## Recovery envelope/rotation/emergency access requirements

Evidence must state that recovery-envelope implementation can proceed now only
as no-secret simulation, real recovery envelope is blocked, rotation can be
simulated no-secret, old archive compatibility can be modeled with fake key
IDs, emergency access can be modeled, incident response can be modeled, and no
private-key inspection, passphrase collection, or secret logs are allowed.

## Restore/off-host dependency requirements

Evidence must state whether no-secret key custody/recovery harness should
precede off-host target/tool implementation, whether real key custody/recovery
must precede real restore reliance, whether NA-0359 dry-run restore evidence is
sufficient for the no-secret harness, whether off-host implementation remains
blocked, and which monitoring/restore/runbook prerequisites remain open.

## Backup-plan impact requirements

Evidence must decide:

- no backup-plan update is required for current NA-0360 governance/testplan
  paths;
- no backup-plan update is required for future pure qsl-protocol no-secret
  fixture/harness work with temporary `/srv/qbuild/tmp/NA-0361_*` proof;
- backup-plan update is required for real key material, recovery envelopes,
  durable secret-related artifacts, off-host repositories, restore targets,
  monitoring artifacts, source-list changes, scripts, timers, fstab, services,
  backup, restore, deploy, rollback, and public-claim mutation.

## Public-ingress/timing/traffic-shape boundary requirements

Evidence must state that NA-0360 changes no public ingress and does not prove
hidden attachment size, hidden timing metadata, hidden traffic shape, hidden all
metadata, or padding that hides all metadata.

## External-review boundary requirements

Evidence must state that external review remains incomplete and that
authorization or no-secret harness evidence does not prove external-review
completion.

## Claim-boundary requirements

Added evidence must not claim or imply production readiness, public-internet
readiness, external-review completion, anonymity, metadata-free behavior,
untraceable behavior, hidden attachment size, hidden timing metadata, hidden
traffic shape, hidden all metadata, real restore completion, complete disaster
recovery, off-host backup completion, real key custody implementation, or real
key recovery implementation.

## Workflow-support deferral requirements

Evidence must record whether local workflow-support/history-index improvements
would reduce friction while not implementing them in NA-0360.

## Required local checks

```bash
git status --porcelain=v1 --branch
git diff --name-only origin/main...HEAD
git diff --stat origin/main...HEAD
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
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main --allowed docs/governance/evidence/NA-0360_metadata_runtime_key_custody_key_recovery_implementation_authorization.md tests/NA-0360_metadata_runtime_key_custody_key_recovery_implementation_authorization_testplan.md DECISIONS.md TRACEABILITY.md docs/ops/ROLLING_OPERATIONS_JOURNAL.md
python3 scripts/ci/qsl_evidence_helper.py link-check
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
bash scripts/ci/classify_ci_scope.sh docs/governance/evidence/NA-0360_metadata_runtime_key_custody_key_recovery_implementation_authorization.md tests/NA-0360_metadata_runtime_key_custody_key_recovery_implementation_authorization_testplan.md DECISIONS.md TRACEABILITY.md docs/ops/ROLLING_OPERATIONS_JOURNAL.md
```

Heavy checks should include qshield metadata-runtime harnesses, qshield-cli
build/test, demo smoke/stress/soak, metadata phase-2 harnesses, metadata
conformance smoke, qsc `send_commit`, formal model checks, NA-0310 JSON/refimpl
oracle, full refimpl tests, and qsc NA-0313 harness as feasible.

## CI expectations

- PR body includes a standalone `Goals: G1, G2, G3, G4, G5` line.
- Goal-lint passes.
- Required GitHub checks attach and complete green before merge.
- Merge uses a normal merge with `--match-head-commit`.
- No admin bypass, direct push, squash, rebase, delete-branch flag, branch
  deletion command, branch-protection mutation, or public-safety mutation.
- Post-merge qsl-protocol main keeps public-safety required and green.

## Successor handoff

Selected successor must be exact:

`NA-0361 -- Metadata Runtime Key Custody / Key Recovery No-Secret Implementation Harness`

NA-0360 must not implement NA-0361. Real key custody, real key recovery,
off-host backup, restore targets, backup-plan updates, local-ops mutations,
secret handling, and public-claim changes remain future-authorized only.
