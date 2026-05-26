Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-25

# NA-0363 Metadata Runtime Off-Host Encrypted Backup Target Tool No-Secret Harness Testplan

## Objective

Validate that NA-0363 implements only qsl-protocol no-secret fixture and
harness evidence for off-host encrypted backup target/tool boundaries. The
lane must prove simulated target/tool relationships and fail-closed negative
cases without any real remote connection, repository initialization, tool
installation, backup, restore, key handling, deploy, rollback, local backup
mutation, or public-claim expansion.

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

## Allowed scope

- `inputs/metadata_runtime/off_host_backup_target_tool_no_secret_fixture_v1.json`
- `scripts/ci/metadata_runtime_off_host_backup_target_tool_no_secret_harness.sh`
- `docs/governance/evidence/NA-0363_metadata_runtime_off_host_encrypted_backup_target_tool_no_secret_harness.md`
- `tests/NA-0363_metadata_runtime_off_host_encrypted_backup_target_tool_no_secret_harness_testplan.md`
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

## Prior off-host target/tool authorization review requirements

Evidence must review live NA-0363 scope, inherited NA-0362 authorization,
NA-0361 no-secret key custody / key recovery harness, NA-0359 no-secret
restore-drill dry-run harness, NA-0355 target/tool class selection,
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
custody/recovery harness status, real restore status, real key custody/recovery
status, off-host target/tool status, local history folder presence, backup
coverage, and D132 protection status.

## Fixture schema requirements

The fixture must include schema version, artifact class, source
classification, local backup classification, key custody classification,
restore classification, simulated-only target mode, simulated-only tool mode,
simulated SSH/SFTP target metadata, simulated target identity metadata,
simulated restic-style repository metadata, simulated snapshot/check/prune/
restore metadata, simulated retention/purge metadata, simulated monitoring/
alert metadata, operator runbook markers, no-secret sentinels, expected
outcomes, negative cases, forbidden operations, claim boundaries, required
markers, backup-plan impact, qsl-server/qsl-attachments boundaries, qshield
demo boundary, and selected successor.

## Harness behavior requirements

The harness must:

- pass `bash -n`;
- accept a fixture path argument;
- validate fixture existence;
- validate JSON with Python;
- validate required schema fields;
- validate simulated target/repository/snapshot/check/prune/restore/
  retention/monitoring relationships;
- validate forbidden operations and claim boundaries;
- validate negative cases fail closed;
- write text proof under `/srv/qbuild/tmp/NA-0363_*`;
- emit required markers;
- exit nonzero on tampered or incomplete fixture;
- avoid backup/restore/off-host/key commands, network, service mutation, and
  local backup mutation.

## Marker requirements

Harness output must include every `NA0363_*` marker required by the directive,
including target/tool authorization, no-secret harness, simulated target,
simulated repository, simulated matrix, retention/purge, monitoring/alert,
operator runbook, backup-plan impact, no remote connection, no repository init,
no tool installation, no real backup, no real restore, no key generation, no
passphrase collection, no secret material, no unsupported readiness/privacy
claim, and final metadata runtime no-secret success.

## Negative/fail-closed requirements

The harness must fail closed for:

- missing target metadata;
- missing repository metadata;
- snapshot/check mismatch;
- missing retention/purge entry;
- missing monitoring/alert entry;
- prohibited operation field;
- remote connection attempted flag;
- missing claim boundary;
- missing no-secret marker;
- sentinel leak detection.

## Artifact redaction/secret-scan requirements

Scan the fixture, harness, and proof artifact. Verify no key, passphrase,
token, private-key, credential, raw secret material, real remote hostname, real
repository path, or sentinel leak is present outside the sentinel list. Proof
artifacts must be under `/srv/qbuild/tmp/NA-0363_*`, text-only, rebuildable,
and not required as durable evidence beyond summarized path/checksum/scan
proof.

## Backup-plan impact requirements

Record whether a backup-plan update is required. Expected result:
`NO_BACKUP_PLAN_UPDATE_REQUIRED_NOW` if the only durable changes are tracked
qsl-protocol paths and temporary proof under `/srv/qbuild/tmp`.

Record that future real off-host target/repository/tool/key/recovery-envelope/
restore/monitoring/source-list/script/timer/fstab/system-service/backup/restore/
deploy/rollback/public-claim work remains backup-plan and local-ops gated.

## Public-ingress/timing/traffic-shape boundary requirements

Confirm no public ingress, timing, traffic-shape, padding, cover-traffic,
website, service, public-doc, or public-claim behavior is changed. Confirm no
claim states that attachment size, timing metadata, traffic shape, or all
metadata is hidden.

## External-review boundary requirements

Confirm external review remains incomplete and no service-local, no-secret, or
demo evidence is presented as external-review completion.

## Claim-boundary requirements

Confirm no production-readiness, public-internet-readiness, anonymity,
metadata-free, untraceable, hidden-size, hidden-timing, hidden-traffic-shape,
off-host-backup-complete, disaster-recovery-complete, real-restore-complete,
real-key-custody-implemented, or real-key-recovery-implemented claim is
introduced.

## Workflow-support deferral requirements

Record whether qstart/qresume fast-forwarding, response-file writing, bounded
polling helpers, machine-readable manifests, validation profiles, per-directive
allow-files, source/authority helpers, claim-boundary scanners, directive/
response/journal indexes, and local history backup coverage would reduce
friction. Do not implement workflow-support items in NA-0363.

## Required local checks

Run at minimum:

```text
bash -n scripts/ci/metadata_runtime_off_host_backup_target_tool_no_secret_harness.sh
python3 -m json.tool inputs/metadata_runtime/off_host_backup_target_tool_no_secret_fixture_v1.json >/dev/null
bash scripts/ci/metadata_runtime_off_host_backup_target_tool_no_secret_harness.sh inputs/metadata_runtime/off_host_backup_target_tool_no_secret_fixture_v1.json
cargo audit --deny warnings
cargo tree -i rustls-webpki --locked
cargo fmt --check
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main
python3 scripts/ci/qsl_evidence_helper.py link-check
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
```

Run the prior metadata runtime harnesses, feasible qshield/qsc/formal checks,
goal-lint, classifier proof, overclaim scan, and public-safety proof required
by the directive before PR merge.

## CI expectations

Required qsl-protocol checks must pass normally before merge. `public-safety`
must remain required and green before merge and after merge. No admin bypass,
direct push, squash, rebase, or branch deletion is allowed.

## Successor handoff

If NA-0363 merges cleanly, closeout should restore exactly:

`NA-0364 -- Metadata Runtime Restore Drill Isolated Restore Authorization Plan`

Closeout must not implement NA-0364.
