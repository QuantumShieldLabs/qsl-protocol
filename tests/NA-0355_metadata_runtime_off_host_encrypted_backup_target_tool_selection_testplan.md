Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-24

# NA-0355 Metadata Runtime Off-Host Encrypted Backup Target Tool Selection Testplan

Goals: G1, G2, G3, G4, G5

## Objective

Validate that NA-0355 records an off-host encrypted backup target/tool selection
plan without performing off-host setup, key handling, backup, restore, deploy,
rollback, purge, local backup mutation, qsl-server mutation, qsl-attachments
mutation, or public-claim expansion.

## Protected invariants

- NA-0355 is target/tool selection planning only.
- qsl-protocol is the only mutable repository.
- qsl-server and qsl-attachments are read-only.
- Local backup scripts, timers, fstab, source lists, service units, targets,
  keys, passphrases, remote destinations, backup operations, restore
  operations, deploy operations, rollback operations, and purge operations are
  not changed or executed.
- Local continuity backup is not complete disaster recovery.
- Off-host encrypted backup is not complete.
- qsl-server PR #56 remains bounded end-to-end harness evidence.
- qsl-attachments PR #37 remains service-local prerequisite evidence.
- qshield embedded relay/demo proof remains reference/oracle evidence only.
- No claim states or implies production readiness, public-internet readiness,
  external-review completion, anonymity, metadata-free behavior, untraceable
  behavior, hidden attachment size, hidden timing metadata, hidden traffic
  shape, hidden all metadata, or padding hiding all metadata.

## Allowed scope

- `docs/governance/evidence/NA-0355_metadata_runtime_off_host_encrypted_backup_target_tool_selection_plan.md`
- `tests/NA-0355_metadata_runtime_off_host_encrypted_backup_target_tool_selection_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Forbidden scope

- qsl-server and qsl-attachments mutation.
- qshield runtime, qsc/qsp, protocol, crypto, key-schedule, dependency,
  workflow, website, README, START_HERE, docs/public, service, deployment,
  backup script, timer, fstab, source-list, off-host destination, restore,
  rollback, purge, branch-protection, or public-safety configuration mutation.

## Prior authorization review requirements

The evidence must review NA-0354 and confirm:

- NA-0354 selected NA-0355 as the target/tool selection plan;
- off-host encrypted backup remains unimplemented and unproven;
- target selection, tool selection, key custody, key recovery, restore drill,
  retention/purge, monitoring/alerting, operator runbook, backup-plan update,
  and local-ops authorization remain prerequisites.

## Source/authority refresh requirements

Refresh qsl-server and qsl-attachments read-only and record:

- local path and SHA if present;
- remote default branch SHA;
- PR #56 / PR #37 merge status;
- latest main CI status;
- viewer permission;
- branch protection;
- open PR list;
- classifications: `FRESH_SOURCE`, `COMPLETE_AUTHORITY`, and `COMPLETE_CI`, or
  exact blocker classifications if evidence differs.

## Local backup/installed tool/off-host evidence refresh requirements

Record:

- `/backup/qsl` mount status;
- local snapshot, manifest, and log availability;
- backup status and backup plan availability;
- `qsl-backup` syntax/preflight/list read-only results if safe;
- installed availability for `restic`, `borg`, `rclone`, `age`, `gpg`, `ssh`,
  and `rsync`;
- whether off-host encrypted backup is configured or proven;
- whether target/tool/key/restore/retention/monitoring/runbook evidence exists;
- whether Codex directives, requests, journals, ops, responses, and D132 are
  present and/or covered.

## Target option requirements

The evidence must evaluate:

- external disk;
- NAS;
- SSH/SFTP host;
- object storage / S3-compatible;
- Backblaze B2 or similar object storage;
- encrypted cloud bucket;
- removable offline media;
- no target / deferred.

Each row must record local evidence, operational burden, security benefit,
key/secret implications, restore testability, monitoring feasibility,
cost/quota risk, provider/outage risk, ransomware/local compromise resistance,
availability risk, compatibility with current qsl-backup posture,
implementation complexity, backup-plan impact, and result.

## Tool option requirements

The evidence must evaluate:

- existing qsl-backup extension with encrypted archive stage;
- restic;
- borg;
- rclone crypt;
- age-wrapped tar/rsync;
- gpg-wrapped archive;
- ssh/rsync transport plus separate encryption layer;
- no tool / deferred.

Each row must record local installed availability, secret/key handling
requirements, restore workflow, manifest/checksum support, incremental support,
retention/purge support, monitoring/exit-code behavior, dry-run/testability,
CI/local harness feasibility, complexity, compatibility with current
qsl-backup, backup-plan impact, and result.

## Target/tool compatibility requirements

The evidence must decide:

- whether target selection can be made now;
- whether tool selection can be made now;
- whether target/tool should be split into separate lanes;
- whether key custody must precede tool implementation;
- whether local-ops backup-plan update must precede tool implementation;
- whether a dry-run harness can precede real off-host setup;
- whether restore-drill planning and monitoring/logging must be planned first.

Expected classification:

- `TARGET_TOOL_SELECTION_PARTIAL`
- `TARGET_TOOL_DEFERRED_KEY_CUSTODY`
- `TARGET_TOOL_DEFERRED_LOCAL_OPS`
- `TARGET_TOOL_DEFERRED_RESTORE_DRILL`

## Key custody dependency requirements

The evidence must explicitly state:

- no key generation in NA-0355;
- no key upload in NA-0355;
- no secret material printed;
- no passphrase collection;
- no secret-dependent tests;
- future key generation, custody, recovery, rotation, emergency access,
  operator runbook, secret scanning, no-secret-artifact, and lost/exposed key
  response requirements.

## Restore drill dependency requirements

The evidence must define future dry-run restore, isolated real restore,
manifest/checksum verification, retention, purge, failed-backup cleanup,
failed-restore cleanup, monitoring/alerting, runbook, emergency stop, operator
verification, artifact/evidence handling, backup-plan impact, and stop
conditions.

## Backup-plan impact requirements

The evidence must decide:

- whether NA-0355 itself requires a backup-plan update;
- whether future off-host implementation requires a backup-plan update;
- whether local workflow-support/history-index backup coverage should precede
  or follow target/tool implementation;
- whether directive/response history indexing should become NA-0356 or later;
- whether D132 remains protected and untouched.

## Public-ingress/timing/traffic-shape boundary requirements

The evidence must state that public ingress remains future-gated and that
current evidence does not prove hidden attachment size, hidden timing metadata,
hidden traffic shape, hidden all metadata, or padding hiding all metadata.

## External-review boundary requirements

The evidence must state that external review remains incomplete and that
NA-0355 is internal target/tool selection planning only.

## Claim-boundary requirements

The evidence must not strengthen any public/privacy/readiness claim. It must
preserve explicit caveats for production readiness, public-internet readiness,
external-review completion, anonymity, metadata-free behavior, untraceability,
attachment size, timing, traffic shape, local continuity, and off-host backup
completion.

## Workflow-support deferral requirements

The evidence must record whether read-only history paths were present and
whether workflow-support improvements would reduce friction, while not
implementing those improvements.

## Required local checks

Run or record the reason if not feasible:

```bash
cargo audit --deny warnings
cargo tree -i rustls-webpki --locked
cargo fmt --check
bash -n scripts/ci/metadata_runtime_production_backup_deploy_rollback_harness.sh
bash scripts/ci/metadata_runtime_production_backup_deploy_rollback_harness.sh inputs/metadata_runtime/production_backup_deploy_rollback_fixture_v1.json
python3 -m json.tool inputs/metadata_runtime/production_backup_deploy_rollback_fixture_v1.json >/dev/null
cargo +stable test -p qshield-cli --locked -- --test-threads=1
cargo +stable build -p qshield-cli --locked
scripts/ci/demo_cli_smoke.sh
DEMO_STRESS_PROFILE=baseline scripts/ci/demo_adversarial_stress.sh
DEMO_SOAK_RUNS=3 scripts/ci/demo_soak_repeated_run.sh
bash scripts/ci/metadata_runtime_identifier_padding_harness_plan.sh
bash scripts/ci/metadata_phase2_identifier_padding_harness.sh
bash scripts/ci/metadata_phase2_sanitized_errors_retention_harness.sh
scripts/ci/metadata_conformance_smoke.sh
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
python3 formal/model_qsc_handshake_suite_id_bounded.py
python3 formal/run_model_checks.py
python3 -m json.tool inputs/suite2/qsc_handshake_suite_id_vectors_na0310.json >/dev/null
cargo +stable test --manifest-path tools/refimpl/quantumshield_refimpl/Cargo.toml --locked --test na_0310_qsc_suite_id_vector_oracle -- --test-threads=1
cargo +stable test --manifest-path tools/refimpl/quantumshield_refimpl/Cargo.toml --locked -- --test-threads=1
cargo +stable test -p qsc --locked --test na_0313_handshake_suite_id_parameter_block -- --test-threads=1
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main
python3 scripts/ci/qsl_evidence_helper.py link-check
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
bash scripts/ci/classify_ci_scope.sh <changed_paths>
```

Goal-lint must pass against a synthesized PR event or live PR event containing
a standalone `Goals: G1, G2, G3, G4, G5` line.

## CI expectations

Required qsl-protocol protected checks, including `public-safety`, must pass
before merge. Post-merge `public-safety` must be required and green before
optional closeout.

## Successor handoff

If the target/tool selection plan merges and closeout is later executed,
NA-0356 must be restored as:

`Metadata Runtime Key Custody / Key Recovery Prerequisite Plan`

NA-0356 must not be implemented by NA-0355 closeout.
