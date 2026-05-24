Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-24

# NA-0354 Metadata Runtime Off-Host Encrypted Backup Implementation Authorization Testplan

Goals: G1, G2, G3, G4, G5

## Objective

Validate that NA-0354 records an off-host encrypted backup implementation
authorization decision after NA-0353 without running or configuring any
off-host backup, restore, deploy, rollback, purge, service mutation, key
generation, passphrase collection, secret-dependent test, or local backup
mutation.

## Protected invariants

- NA-0354 is authorization planning only.
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
- No claim says or implies production readiness, public-internet readiness,
  external-review completion, anonymity, metadata-free behavior,
  untraceability, hidden attachment size, hidden timing metadata, hidden
  traffic shape, hidden all metadata, or padding hiding all metadata.

## Allowed scope

- `docs/governance/evidence/NA-0354_metadata_runtime_off_host_encrypted_backup_implementation_authorization.md`
- `tests/NA-0354_metadata_runtime_off_host_encrypted_backup_implementation_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Forbidden scope

- qsl-server and qsl-attachments mutation.
- qshield runtime, qsc/qsp, protocol, crypto, key-schedule, dependency,
  workflow, website, README, START_HERE, docs/public, service, deployment,
  backup script, timer, fstab, source-list, off-host destination, restore,
  rollback, purge, branch-protection, or public-safety configuration mutation.

## Prior prerequisite plan review requirements

The evidence must review NA-0353 and confirm:

- local backup remains same-host continuity only;
- off-host encrypted backup remains unimplemented and unproven;
- target selection, encryption tool selection, key custody, key recovery,
  restore drill, retention/purge, monitoring/alerting, operator runbook, and
  backup-plan update remain prerequisites.

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

## Local backup/off-host evidence refresh requirements

Record:

- `/backup/qsl` mount status;
- local snapshot, manifest, and log availability;
- backup status and backup plan availability;
- `qsl-backup` syntax/preflight/list read-only results if safe;
- whether off-host encrypted backup is configured or proven;
- whether target/tool/key/restore/retention/monitoring/runbook evidence exists;
- whether Codex directives, requests, journals, ops, responses, and D132 are
  present and/or covered.

## Implementation authorization decision requirements

The evidence must choose one of the directive decision categories and explain
why. If prerequisite-gated, it must name the exact blocker categories and select
an exact successor.

## Future implementation bundle requirements

The evidence must define future repositories, allowed files, forbidden files,
commands, tests, artifacts, markers, PR/order, backup-plan update requirements,
key-handling requirements, restore-drill requirements, monitoring/logging
requirements, public-claim boundaries, and stop conditions.

## Target/tool/key matrix requirements

The evidence must evaluate external disk, NAS, object storage, SSH/SFTP host,
encrypted cloud bucket, removable offline media, age, gpg, restic, borg,
rclone crypt, existing qsl-backup extension, no encryption, operator-held
passphrase, age recipient file, hardware token, split secret, offline recovery
envelope, and service-managed key only.

## Key-handling/secrets requirements

The evidence must explicitly state:

- no key generation in NA-0354;
- no key upload in NA-0354;
- no secret material printed;
- no passphrase collection;
- no secret-dependent tests;
- future key generation, custody, recovery, rotation, emergency access,
  operator runbook, secret scanning, and no-secret-artifact requirements.

## Restore drill/retention/purge/monitoring/runbook requirements

The evidence must define future dry-run restore, isolated real restore,
manifest/checksum verification, retention, purge, failed-backup cleanup,
failed-restore cleanup, monitoring/alerting, runbook, emergency stop, operator
verification, artifact/evidence handling, backup-plan impact, and stop
conditions.

## Backup-plan impact requirements

The evidence must decide:

- whether NA-0354 itself requires a backup-plan update;
- whether future off-host implementation requires a backup-plan update;
- whether local workflow-support/history-index backup coverage should precede
  or follow off-host work;
- whether directive/response history indexing should become NA-0355 or later;
- whether D132 remains protected and untouched.

## Public-ingress/timing/traffic-shape boundary requirements

The evidence must state that public ingress remains future-gated and that
current evidence does not prove hidden attachment size, hidden timing metadata,
hidden traffic shape, hidden all metadata, or padding hiding all metadata.

## External-review boundary requirements

The evidence must state that external review remains incomplete and that
NA-0354 is internal authorization planning only.

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
closeout.

## Successor handoff

If the authorization plan merges and closeout is later executed, NA-0355 must
be restored as:

`Metadata Runtime Off-Host Encrypted Backup Target / Tool Selection Plan`

NA-0355 must not be implemented by NA-0354 closeout.
