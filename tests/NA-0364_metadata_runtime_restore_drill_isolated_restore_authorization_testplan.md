Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-26

# NA-0364 Metadata Runtime Restore Drill Isolated Restore Authorization Testplan

## Objective

Validate that NA-0364 is a qsl-protocol governance/authorization lane that
authorizes, blocks, or prerequisites future isolated restore-drill work after
NA-0359 no-secret restore dry-run evidence, NA-0361 no-secret key
custody/recovery evidence, and NA-0363 no-secret off-host target/tool evidence.

## Protected invariants

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
- The no-secret target/tool harness is not real off-host backup.
- The NA-0364 authorization plan is not a real restore execution.

## Allowed scope

- `docs/governance/evidence/NA-0364_metadata_runtime_restore_drill_isolated_restore_authorization.md`
- `tests/NA-0364_metadata_runtime_restore_drill_isolated_restore_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Forbidden scope

- qsl-server, qsl-attachments, qshield runtime, qsc/qsp, protocol, crypto,
  key-schedule, dependency, workflow, website, README, START_HERE, docs/public,
  service, deployment, backup script, timer, fstab, source-list, off-host
  destination, restore, rollback, purge, branch-protection, or public-safety
  configuration mutation.
- Backup execution, restore execution, restore target creation, restore target
  mount, restore payload copy, key generation, key upload, passphrase
  collection, private-key inspection, secret material handling,
  recovery-envelope content creation, repository initialization, remote/off-host
  backup tooling, deploy, rollback, or tool installation.

## Prior isolated restore readiness review requirements

Evidence must review live NA-0364 scope, NA-0363 no-secret off-host target/tool
harness, NA-0361 no-secret key custody / key recovery harness, NA-0359
no-secret restore-drill dry-run harness, NA-0355 target/tool class selection,
qsl-server PR #56, qsl-attachments PR #37, local backup/tool/key/off-host/
restore posture, public-claim boundaries, and stop conditions.

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
`ssh`, and `rsync`, no-secret restore dry-run harness status, no-secret key
custody/recovery harness status, no-secret target/tool harness status, real
restore status, real key custody/recovery status, off-host target/tool status,
local history folder presence, backup coverage, and D132 protection status.

## Isolated restore authorization requirements

Evidence must state whether future isolated restore implementation is:

- no-secret harness authorization-ready;
- blocked by backup plan;
- blocked by local ops;
- blocked by secret handling;
- blocked by operator runbook;
- blocked by key custody;
- blocked by target isolation;
- deferred for any real operation.

Real restore operations must remain blocked unless every prerequisite is named
and future exact authorization exists.

## Restore target authorization requirements

Evidence must state whether future restore target implementation is:

- no-secret harness authorization-ready;
- blocked by backup plan;
- blocked by local ops;
- blocked by secret handling;
- blocked by cleanup;
- blocked by monitoring;
- deferred for any real target creation, mount, or copy.

## Future no-secret bundle requirements

If no-secret implementation is authorization-ready, evidence must define:

- future repo;
- future allowed files;
- future forbidden files and operations;
- future commands;
- future tests;
- future temporary artifacts;
- future markers;
- future PR/order;
- future backup-plan update requirement;
- future key-handling requirement;
- future off-host target/tool requirement;
- future monitoring/logging requirement;
- future public-claim boundary;
- future stop conditions.

## Real restore blocker requirements

Evidence must analyze real `/srv/qbuild/tmp` restore target, disposable disk
target, staging machine target, non-live qbuild target, production root
restore, mount, copy, backup payload retrieval, repository access, key or
passphrase requirement, cleanup, monitoring, retention, and purge. Each real
operation must be blocked or rejected in NA-0364.

## Simulated fixture requirements

Evidence must evaluate:

- qsl-protocol simulated isolated restore target fixture;
- qsl-protocol simulated manifest/checksum restore fixture;
- qsl-protocol simulated old-archive compatibility matrix;
- qsl-protocol simulated cleanup/monitoring/runbook matrix;
- qsl-server/qsl-attachments service-local fixture harness;
- rejected no-implementation option.

Each option must state safety, allowed files, secret risk, restore/off-host
risk, backup-plan impact, confidence gained, confidence not gained, CI
feasibility, and recommendation.

## Restore target creation/mount/copy blocker requirements

Confirm real restore target creation cannot proceed now, mount cannot proceed
now, copy cannot proceed now, live root restore is outside this phase, no-secret
modeling is sufficient for the next lane, and real restore remains blocked by
key custody, backup plan, local ops, off-host target/tool, monitoring,
retention, cleanup, and operator runbook.

## Key dependency requirements

Confirm no-secret isolated restore can proceed after NA-0361/NA-0363 using
simulated key/recovery evidence only. Confirm real isolated restore requires
real key custody/recovery and remains blocked without it.

## Off-host target/tool dependency requirements

Confirm no-secret isolated restore can proceed after NA-0363 using simulated
target/tool evidence only. Confirm real isolated restore requires real off-host
target/tool/repository proof and remains blocked without it.

## Retention/purge/monitoring/runbook dependency requirements

Confirm real restore work requires retention/purge/cleanup evidence,
monitoring/alert evidence, and operator runbook approval. Confirm future
no-secret harness should model cleanup and alert markers only.

## Backup-plan impact requirements

Record whether a backup-plan update is required. Expected result:
`NO_BACKUP_PLAN_UPDATE_REQUIRED_NOW` if the only durable changes are tracked
qsl-protocol governance/testplan/journal paths.

Record that future pure qsl-protocol no-secret harness proof may remain
temporary under `/srv/qbuild/tmp`, while real targets, durable restored
artifacts, key material, recovery envelopes, off-host targets, source-list
changes, scripts, timers, fstab, services, monitoring artifacts, backup,
restore, deploy, rollback, and public-claim mutation remain backup-plan and
local-ops gated.

## Public-ingress/timing/traffic-shape boundary requirements

Confirm no public ingress, timing, traffic-shape, padding, cover-traffic,
website, service, public-doc, or public-claim behavior is changed. Confirm no
claim states that attachment size, timing metadata, traffic shape, or all
metadata is hidden.

## External-review boundary requirements

Confirm external review remains incomplete and no service-local, no-secret,
demo, dry-run, key, target/tool, or authorization evidence is presented as
external-review completion.

## Claim-boundary requirements

Confirm no production-readiness, public-internet-readiness, anonymity,
metadata-free, untraceable, hidden-size, hidden-timing, hidden-traffic-shape,
off-host-backup-complete, disaster-recovery-complete, restore-drill-complete,
real-key-custody-implemented, or real-key-recovery-implemented claim is
introduced.

## Workflow-support deferral requirements

Record whether qstart/qresume fast-forwarding, response-file writing, bounded
polling helpers, machine-readable manifests, validation profiles, per-directive
allow-files, source/authority helpers, claim-boundary scanners, directive/
response/journal indexes, and local history backup coverage would reduce
friction. Do not implement workflow-support items in NA-0364.

## Required local checks

Run at minimum:

```text
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
bash -n scripts/ci/metadata_runtime_off_host_backup_target_tool_no_secret_harness.sh
python3 -m json.tool inputs/metadata_runtime/off_host_backup_target_tool_no_secret_fixture_v1.json >/dev/null
bash scripts/ci/metadata_runtime_off_host_backup_target_tool_no_secret_harness.sh inputs/metadata_runtime/off_host_backup_target_tool_no_secret_fixture_v1.json
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main --allowed docs/governance/evidence/NA-0364_metadata_runtime_restore_drill_isolated_restore_authorization.md --allowed tests/NA-0364_metadata_runtime_restore_drill_isolated_restore_authorization_testplan.md --allowed DECISIONS.md --allowed TRACEABILITY.md --allowed docs/ops/ROLLING_OPERATIONS_JOURNAL.md
python3 scripts/ci/qsl_evidence_helper.py link-check
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
```

Run feasible qshield/qsc/formal checks, goal-lint, classifier proof,
overclaim scan, and public-safety proof required by the directive before PR
merge.

## CI expectations

Required qsl-protocol checks must pass normally before merge. `public-safety`
must remain required and green before merge and after merge. No admin bypass,
direct push, squash, rebase, or branch deletion is allowed.

## Successor handoff

If NA-0364 merges cleanly, closeout should restore exactly:

`NA-0365 -- Metadata Runtime Restore Drill Isolated Restore No-Secret Implementation Harness`

Closeout must not implement NA-0365.
