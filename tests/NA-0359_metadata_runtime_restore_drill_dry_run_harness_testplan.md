# NA-0359 Metadata Runtime Restore Drill Dry-Run Harness Testplan

Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-05-25

## Objective

Validate that NA-0359 adds a qsl-protocol-only no-secret restore-drill dry-run
fixture and harness that prove manifest/checksum validation, redacted temporary
proof artifacts, fail-closed negative cases, cleanup/monitoring/runbook
markers, backup-plan impact, and public-claim boundaries without executing a
real restore or touching keys, off-host destinations, service repos, runtime
surfaces, or local backup configuration.

## Protected invariants

- NA-0359 is a dry-run fixture harness only.
- qsl-protocol is the only mutable repository.
- qsl-server and qsl-attachments are read-only.
- qshield runtime, qsc, qsp, protocol, crypto, and key-schedule paths are not
  changed.
- No dependency, Cargo, workflow, website, README, START_HERE, docs/public,
  branch-protection, public-safety, backup script, timer, fstab, local backup
  source-list, service configuration, deploy, rollback, restore, real restore
  target, off-host destination, key, passphrase, private-key, or secret-handling
  path is changed or executed.
- Local continuity backup is not complete disaster recovery.
- Off-host encrypted backup is not complete.
- Dry-run harness evidence is not real restore execution.

## Allowed scope

- `inputs/metadata_runtime/restore_drill_dry_run_fixture_v1.json`
- `scripts/ci/metadata_runtime_restore_drill_dry_run_harness.sh`
- `docs/governance/evidence/NA-0359_metadata_runtime_restore_drill_dry_run_harness.md`
- `tests/NA-0359_metadata_runtime_restore_drill_dry_run_harness_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Forbidden scope

- qsl-server and qsl-attachments mutation.
- qshield runtime, qsc/qsp, protocol, crypto, key-schedule, dependency,
  workflow, website, README, START_HERE, docs/public, service, deployment,
  backup script, timer, fstab, source-list, off-host destination, restore,
  rollback, purge, branch-protection, or public-safety configuration mutation.
- Restore execution, restore target creation, restore target mount, restore
  payload copy, key generation, key upload, passphrase collection, private key
  inspection, secret material handling, repository initialization, or
  remote/off-host backup tooling.

## Prior restore authorization review requirements

Evidence must review NA-0358 and confirm:

- NA-0358 authorized a qsl-protocol no-secret dry-run harness.
- NA-0358 did not authorize isolated real restore, key handling, off-host
  setup, backup, deploy, rollback, or service/runtime mutation.
- NA-0357 and NA-0356 preserve the key custody/recovery, off-host backup,
  restore target, cleanup, monitoring, runbook, and backup-plan gates.

## Source/authority refresh requirements

Refresh qsl-server and qsl-attachments read-only and record local path/SHA,
remote default branch SHA, PR #56 / PR #37 merge status, latest main CI status,
viewer permission, branch protection, open PR list, and classifications.

## Local backup/key/restore evidence refresh requirements

Record `/backup/qsl` mount status, local snapshots, manifests, logs, backup
status, backup plan availability, safe `qsl-backup` syntax/preflight/list
results, installed availability for `restic`, `borg`, `rclone`, `age`, `gpg`,
`ssh`, and `rsync`, dry-run/real-restore/key/off-host evidence state, Codex
history path presence, backup coverage, and D132 protection status.

## Fixture schema requirements

The fixture must include schema version, artifact class, source
classification, backup state classification, restore state classification,
target type, restore mode, no-secret sentinels, manifest entries, checksums,
expected validation outcomes, tamper/negative cases, cleanup plan,
monitoring/alert plan, operator runbook summary, forbidden operations, claim
boundaries, required markers, backup-plan impact, qsl-server/qsl-attachments
boundary, and qshield demo boundary.

The fixture must not include real backup payloads, real private-material paths,
keys, passphrases, tokens, endpoint credentials, private key text, raw secret
material, or unredacted sensitive operational data.

## Harness behavior requirements

The harness must pass `bash -n`, accept a fixture path argument, validate
fixture existence, validate JSON with Python, validate required schema fields,
recompute manifest/checksum relationships, validate forbidden operations,
validate public-claim boundaries, run negative cases fail closed, write proof
under `/srv/qbuild/tmp/NA-0359_*`, emit required markers, exit nonzero on
tampered or incomplete fixture, and avoid backup/restore/off-host/key/service
operations.

## Marker requirements

The harness output must include all required NA0359 markers, ending with
`NA0359_METADATA_RUNTIME_RESTORE_DRY_RUN_OK`.

## Negative/fail-closed requirements

Required negative cases:

- missing manifest fails;
- checksum mismatch fails;
- missing cleanup plan fails;
- prohibited operation field fails;
- missing claim boundary fails;
- missing no-secret fixture marker fails;
- sentinel leak detection fails.

## Artifact redaction/secret-scan requirements

Scan fixture, harness, and proof artifact. Proof artifact must not contain
no-secret sentinel labels, private-key, token, credential, passphrase,
auth-header, or long-hex dumps. Proof artifact must stay under
`/srv/qbuild/tmp/NA-0359_*`. No durable new location outside qsl-protocol and
`/srv/qbuild/tmp` may be required.

## Backup-plan impact requirements

Evidence must state whether NA-0359 requires a backup-plan update. Expected
result: no update required if changes stay in qsl-protocol and proof artifacts
stay under `/srv/qbuild/tmp`.

Future backup-plan updates remain required for real restore targets, durable
restore artifacts, key material, recovery envelopes, off-host targets,
source-list changes, scripts, timers, fstab, system services, monitoring
artifacts, deploy, rollback, backup, restore, or public-claim mutation.

## Public-ingress/timing/traffic-shape boundary requirements

Evidence must state that NA-0359 changes no public ingress and does not prove
hidden attachment size, hidden timing metadata, hidden traffic shape, hidden all
metadata, or padding that hides all metadata.

## External-review boundary requirements

Evidence must state that external review remains incomplete and that dry-run
harness evidence is not external-review completion.

## Claim-boundary requirements

Added evidence must not claim or imply production readiness, public-internet
readiness, external-review completion, anonymity, metadata-free behavior,
untraceable behavior, hidden attachment size, hidden timing metadata, hidden
traffic shape, hidden all metadata, restore-drill completion, complete disaster
recovery, off-host backup completion, key custody implementation, or key
recovery implementation.

## Workflow-support deferral requirements

Evidence must record whether local workflow-support/history-index improvements
would reduce friction, while not implementing them in NA-0359.

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
python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main --allowed inputs/metadata_runtime/restore_drill_dry_run_fixture_v1.json scripts/ci/metadata_runtime_restore_drill_dry_run_harness.sh docs/governance/evidence/NA-0359_metadata_runtime_restore_drill_dry_run_harness.md tests/NA-0359_metadata_runtime_restore_drill_dry_run_harness_testplan.md DECISIONS.md TRACEABILITY.md docs/ops/ROLLING_OPERATIONS_JOURNAL.md
python3 scripts/ci/qsl_evidence_helper.py link-check
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
bash scripts/ci/classify_ci_scope.sh inputs/metadata_runtime/restore_drill_dry_run_fixture_v1.json scripts/ci/metadata_runtime_restore_drill_dry_run_harness.sh docs/governance/evidence/NA-0359_metadata_runtime_restore_drill_dry_run_harness.md tests/NA-0359_metadata_runtime_restore_drill_dry_run_harness_testplan.md DECISIONS.md TRACEABILITY.md docs/ops/ROLLING_OPERATIONS_JOURNAL.md
```

Additional heavy checks should run as feasible per directive, including qshield
metadata-runtime harnesses, qsc `send_commit`, formal model checks, metadata
conformance, demo smoke/stress/soak, and refimpl tests.

## CI expectations

- PR body includes a standalone `Goals: G1, G2, G3, G4, G5` line.
- Goal-lint passes.
- Required GitHub checks attach and complete green before merge.
- Merge uses a normal merge with `--match-head-commit`.
- No admin bypass, direct push, squash, rebase, delete-branch flag, or branch
  deletion command is used.
- Post-merge qsl-protocol main keeps public-safety required and green.

## Successor handoff

If NA-0359 merges and public-safety is green, close out NA-0359 separately and
restore exactly one READY successor:

`NA-0360 -- Metadata Runtime Key Custody / Key Recovery Implementation Authorization Plan`

The closeout must not implement NA-0360.
