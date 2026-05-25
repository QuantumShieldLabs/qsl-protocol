Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-25

# NA-0356 Metadata Runtime Key Custody Key Recovery Prerequisite Plan Testplan

Goals: G1, G2, G3, G4, G5

## Objective

Validate that NA-0356 records a no-secret key custody/key recovery prerequisite
plan for the selected SSH/SFTP-compatible off-host target class and
restic-style encrypted repository tool class without performing key handling,
off-host setup, backup, restore, deploy, rollback, local backup mutation,
service repo mutation, runtime mutation, dependency mutation, or public-claim
expansion.

## Protected invariants

- NA-0356 is prerequisite planning only.
- qsl-protocol is the only mutable repository.
- qsl-server and qsl-attachments are read-only.
- qshield runtime, qsc, qsp, protocol, crypto, and key-schedule paths are not
  changed.
- Local backup scripts, timers, fstab, service units, source lists, targets,
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

- `docs/governance/evidence/NA-0356_metadata_runtime_key_custody_key_recovery_prerequisite_plan.md`
- `tests/NA-0356_metadata_runtime_key_custody_key_recovery_prerequisite_plan_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Forbidden scope

- qsl-server and qsl-attachments mutation.
- qshield runtime, qsc/qsp, protocol, crypto, key-schedule, dependency,
  workflow, website, README, START_HERE, docs/public, service, deployment,
  backup script, timer, fstab, source-list, off-host destination, restore,
  rollback, purge, branch-protection, or public-safety configuration mutation.
- Key generation, key upload, passphrase collection, private key inspection,
  secret material handling, repository initialization, or remote/off-host
  backup tooling.

## Prior target/tool selection review requirements

The evidence must review NA-0355 and confirm:

- selected target class is SSH/SFTP-compatible off-host host;
- selected tool class is restic-style encrypted snapshot repository;
- selection remains class-level only;
- no destination, credential, key, passphrase, repository, schedule, retention,
  alerting, backup, restore, deploy, rollback, purge, or local backup mutation
  occurred;
- key custody/key recovery is the inherited blocker.

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

## Local backup/key/off-host evidence refresh requirements

Record:

- `/backup/qsl` mount status;
- local snapshot, manifest, and log availability;
- backup status and backup plan availability;
- `qsl-backup` syntax/preflight/list read-only results if safe;
- installed availability for `restic`, `borg`, `rclone`, `age`, `gpg`, `ssh`,
  and `rsync`;
- whether off-host encrypted backup is configured or proven;
- whether key custody, key recovery, key rotation, emergency access, operator
  runbook, no-secret artifact guidance, and secret scanning guidance exist;
- whether Codex directives, requests, journals, ops, responses, and D132 are
  present and/or covered.

## Key custody threat/value requirements

The evidence must analyze key loss, passphrase loss, single-operator
dependency, operator unavailability, accidental exposure, clipboard/shell
history exposure, file-permission exposure, accidental backup of key material,
private key storage in repo, secrets in logs/artifacts, provider compromise,
hardware failure, emergency access, legal/organizational handoff, rotation,
revocation, auditability, operator burden, and no-secret CI/artifacts.

## Key recovery threat/value requirements

The evidence must analyze recovery envelope absence, recovery envelope
compromise, recovery envelope loss, recovery verification without secret
disclosure, periodic recovery drill, multiple-person recovery, break-glass
procedure, lost primary device, compromised primary device, retired key,
rotation with existing archives, restoring old archives after rotation, test
recovery without production secrets, and public-claim risk if recovery is
unproven.

## Custody model option requirements

The evidence must evaluate:

- operator-held passphrase;
- age recipient private key offline;
- gpg recipient private key offline;
- hardware token;
- split secret;
- offline recovery envelope;
- service-managed key;
- no custody model / deferred.

Each row must record local evidence, secret exposure risk, recovery
reliability, operator burden, automation burden, auditability, compatibility
with SSH/SFTP plus restic-style repository class, compatibility with no-secret
CI, compatibility with future restore drill, backup-plan impact, and result.

## Recovery model option requirements

The evidence must evaluate:

- sealed offline recovery envelope;
- second offline encrypted copy;
- second operator escrow;
- split secret recovery;
- hardware token backup;
- paper recovery instructions without secret;
- no recovery / rejected.

Each row must record local evidence, risk, operational burden, restore
reliability, rotation burden, incident response suitability, no-secret evidence
feasibility, and result.

## Key rotation/emergency/incident response requirements

The evidence must define:

- rotation trigger categories;
- revocation boundary;
- archive compatibility after rotation;
- old backup retention after key rotation;
- emergency access boundaries;
- lost key response;
- exposed key response;
- compromised backup repository response;
- operator handoff response;
- no-public-claim boundary until drills prove behavior.

## No-secret artifact/runbook requirements

The evidence must state:

- no key generation in NA-0356;
- no key upload in NA-0356;
- no secret material printed;
- no passphrase collection;
- no private key content inspection;
- no secret-dependent tests;
- future no-secret fixture strategy;
- future redaction strategy;
- future shell-history/clipboard avoidance;
- future file-permission checks;
- future secret scanning requirements;
- future operator runbook requirements;
- future emergency-stop requirements.

## Backup-plan impact requirements

The evidence must decide:

- whether NA-0356 itself requires a backup-plan update;
- whether future key custody/recovery implementation needs backup-plan update
  before any key material exists;
- whether local workflow-support/history-index backup coverage should precede
  or follow key custody implementation;
- whether directive/response history index should become NA-0357 or later;
- whether D132 remains protected and untouched.

## Restore drill dependency requirements

The evidence must decide:

- whether target/tool implementation may proceed after NA-0356;
- whether another restore-drill prerequisite is required first;
- dry-run restore boundary;
- isolated real restore boundary;
- retention/old-key compatibility;
- purge/old-key compatibility;
- failed recovery cleanup;
- alerting if recovery verification fails;
- evidence artifacts and backup-plan impact.

## Public-ingress/timing/traffic-shape boundary requirements

The evidence must state that public ingress remains future-gated and that
current evidence does not prove hidden attachment size, hidden timing metadata,
hidden traffic shape, hidden all metadata, or padding hiding all metadata.

## External-review boundary requirements

The evidence must state that external review remains incomplete and that
NA-0356 is internal prerequisite planning only.

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
bash scripts/ci/metadata_conformance_smoke.sh
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
python3 formal/model_qsc_handshake_suite_id_bounded.py
python3 formal/run_model_checks.py
python3 -m json.tool inputs/suite2/qsc_handshake_suite_id_vectors_na0310.json >/dev/null
cargo +stable test -p quantumshield_refimpl --locked --test na_0310_qsc_suite_id_vector_oracle -- --test-threads=1
cargo +stable test -p quantumshield_refimpl --locked -- --test-threads=1
cargo +stable test -p qsc --locked --test na_0313_handshake_suite_id_parameter_block -- --test-threads=1
```

## CI expectations

The NA-0356 PR must merge normally only after required checks complete green,
including `public-safety`. No admin bypass, direct push, squash, rebase,
delete-branch flag, branch-protection mutation, or public-safety mutation is
allowed.

## Successor handoff

The evidence must select exactly one NA-0357 successor and must not implement
NA-0357. If restore-drill planning is selected, target/tool implementation,
key handling, local-ops backup-plan update, repository initialization, backup,
restore, retention/purge, monitoring, deploy, rollback, and public claims
remain future-gated.
