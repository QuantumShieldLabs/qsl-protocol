Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-24

# NA-0353 Metadata Runtime Off-Host Encrypted Backup Prerequisite Plan Testplan

Goals: G1, G2, G3, G4, G5

## Objective

Validate that NA-0353 records an off-host encrypted backup prerequisite plan
after NA-0352 without running any live backup, restore, deploy, rollback,
purge, off-host setup, service mutation, key generation, passphrase collection,
or secret-dependent operation.

## Protected invariants

- NA-0353 is prerequisite planning only.
- qsl-protocol is the only mutable repository.
- qsl-server and qsl-attachments are read-only.
- No qshield runtime, qsc/qsp, protocol, crypto, key-schedule, dependency,
  workflow, website, README, START_HERE, docs/public, backup-script, timer,
  fstab, service-unit, local backup source-list, off-host target, deployment,
  restore, rollback, purge, or public-ingress mutation occurs.
- Local continuity backup is not complete disaster recovery.
- Off-host encrypted backup is not complete without exact implementation and
  restore evidence.
- qsl-server PR #56 remains bounded end-to-end modeled harness evidence.
- qsl-attachments PR #37 remains service-local prerequisite evidence.
- qshield embedded relay/demo proof remains reference/oracle evidence only.
- No claim says or implies production readiness, public-internet readiness,
  external-review completion, anonymity, metadata-free behavior,
  untraceability, hidden attachment size, hidden timing metadata, hidden traffic
  shape, or padding hiding all metadata.

## Allowed scope

- `docs/governance/evidence/NA-0353_metadata_runtime_off_host_encrypted_backup_prerequisite_plan.md`
- `tests/NA-0353_metadata_runtime_off_host_encrypted_backup_prerequisite_plan_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Forbidden scope

- qsl-server and qsl-attachments mutation.
- qshield runtime, qsc/qsp, protocol, crypto, key-schedule, dependency,
  workflow, website, README, START_HERE, docs/public, service, deployment,
  backup script, timer, fstab, source-list, off-host destination, restore,
  rollback, purge, or public-safety configuration mutation.

## Prior boundary harness review requirements

The evidence must review NA-0352 and confirm:

- the NA-0352 harness is local fixture evidence only;
- `NA0352_OPERATION_EXECUTED_COUNT 0` and `SECRET_FINDING_COUNT 0` were emitted;
- the harness wrote only `/srv/qbuild/tmp` artifacts;
- off-host backup remained future-gated;
- no live operation or claim expansion occurred.

## Source/authority refresh requirements

Refresh qsl-server and qsl-attachments read-only and record:

- local path and SHA if present;
- remote default branch SHA if available;
- PR #56 / PR #37 merge status;
- latest main CI status;
- viewer permission;
- branch protection;
- open PR list;
- classifications:
  `FRESH_SOURCE`, `COMPLETE_AUTHORITY`, and `COMPLETE_CI`, or exact blocker
  classifications if evidence differs.

## Local backup/off-host evidence refresh requirements

Record:

- `/backup/qsl` mount status;
- local snapshot, manifest, and log availability;
- backup status and backup plan availability;
- `qsl-backup` syntax/preflight/list read-only results if safe;
- whether off-host encrypted backup is configured or proven;
- whether restore drill, key handling, key recovery, off-host retention/purge,
  monitoring/alerting, and operator runbooks exist;
- whether Codex directives, requests, journals, ops, responses, and D132 are
  covered or future-gated.

## Threat/value model requirements

The evidence must cover local disk loss, machine loss, ransomware/local
compromise, accidental deletion, corrupted snapshots, failed restore, key loss,
key exposure, provider compromise, provider outage, network outage, cost/quota
growth, retention/purge mismatch, backup privacy, log/artifact leakage,
operator error, RTO/RPO, restore validation, and public claims.

## Prerequisite matrix requirements

The matrix must include:

- backup target class;
- encryption tool choice;
- key custody and recovery;
- passphrase/secret storage;
- restore drill;
- retention/purge;
- manifest/checksum;
- monitoring/alerting;
- cost/quota;
- qbuild, qsl-server/qsl-attachments, Codex responses, Codex directives/
  requests/journals/ops, D132, future qsl-server runtime data,
  qsl-attachments object storage, deployment configs, logs/monitoring,
  disaster-recovery claim boundary, and public-claim boundary.

Each row must classify current evidence, proof status, risk, future proof,
secret/key impact, backup-plan impact, implementation need, blocker/readiness,
and successor relation.

## Implementation option requirements

The evidence must evaluate at least:

- off-host encrypted backup implementation authorization plan;
- off-host encrypted backup blocker resolution;
- local-ops workflow/history index and backup coverage;
- backup target selection;
- key custody/key recovery;
- restore drill;
- monitoring/alerting;
- external review readiness;
- website/public claim boundary;
- public technical position paper.

## Encryption/key-handling/secrets requirements

The evidence must explicitly state:

- no key generation in NA-0353;
- no key upload in NA-0353;
- no secret material printed;
- no passphrase collection;
- future encryption tool, key custody, key recovery, rotation, emergency
  access, operator runbook, secret scanning, and no-secret-artifact
  requirements.

## Restore drill/retention/purge/monitoring/runbook requirements

The evidence must define:

- future restore drill categories;
- dry-run versus real restore boundary;
- isolated restore target requirement;
- manifest/checksum requirements;
- retention and purge policy requirements;
- failed backup and failed restore cleanup;
- monitoring/alerting;
- runbook and emergency stop;
- operator verification;
- evidence artifact and backup-plan impact.

## Backup-plan impact requirements

The evidence must decide:

- whether NA-0353 itself requires a backup-plan update;
- whether future off-host implementation requires a backup-plan update;
- whether local workflow-support/history-index backup coverage should precede
  or follow off-host work;
- whether directive/response history index should become NA-0354 or later;
- whether D132 remains protected and untouched.

## Public-ingress/timing/traffic-shape boundary requirements

The evidence must state that public ingress remains future-gated and that
current evidence does not prove hidden attachment size, hidden timing metadata,
hidden traffic shape, hidden all metadata, or padding that hides all metadata.

## External-review boundary requirements

The evidence must state that external review remains incomplete and that
NA-0353 is internal prerequisite planning only.

## Claim-boundary requirements

The evidence must not strengthen any public/privacy/readiness claim. It must
preserve explicit caveats for production readiness, public-internet readiness,
external-review completion, anonymity, metadata-free behavior, untraceability,
size, timing, traffic shape, local continuity, and off-host backup completion.

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
cargo +stable test -p quantumshield_refimpl --locked --test na_0310_qsc_suite_id_vector_oracle -- --test-threads=1
cargo +stable test -p quantumshield_refimpl --locked -- --test-threads=1
cargo +stable test -p qsc --locked --test na_0313_handshake_suite_id_parameter_block -- --test-threads=1
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main
python3 scripts/ci/qsl_evidence_helper.py link-check
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
bash scripts/ci/classify_ci_scope.sh <changed paths>
```

## CI expectations

Required qsl-protocol branch protection must include `public-safety`. Merge only
after required checks complete green without admin bypass, direct push, squash,
rebase, or branch deletion.

## Successor handoff

Expected successor after successful NA-0353 evidence and closeout:

`NA-0354 -- Metadata Runtime Off-Host Encrypted Backup Implementation Authorization Plan`

NA-0354 must not be implemented by NA-0353.
