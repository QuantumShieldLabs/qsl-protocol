Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-05-25

# NA-0361 Metadata Runtime Key Custody Key Recovery No-Secret Harness Testplan

## Objective

Validate that NA-0361 adds only qsl-protocol no-secret key custody / key
recovery fixture and harness evidence. The lane must prove simulated custody,
simulated recovery-envelope metadata, simulated rotation, simulated old-archive
compatibility, simulated incident response, simulated emergency access,
fail-closed negative cases, no-secret artifacts, backup-plan impact, and claim
boundaries without real key, passphrase, private-key, recovery-envelope,
backup, restore, deploy, rollback, off-host, service, dependency, workflow,
website, or public-claim mutation.

## Protected invariants

- NA-0361 is no-secret fixture/harness evidence only.
- qsl-protocol is the only mutable repository.
- qsl-server and qsl-attachments are read-only.
- qshield runtime, qsc, qsp, protocol, crypto, key-schedule, dependencies,
  workflows, README, START_HERE, docs/public, website, branch protection,
  public-safety configuration, backup scripts, timers, fstab, service units,
  restore targets, off-host targets, deployment scripts, rollback scripts,
  local backup configuration, and production service behavior are not changed.
- No real key generation, key upload, passphrase collection, private-key
  inspection, recovery-envelope content creation, or secret material handling
  occurs.
- No backup, restore, deploy, rollback, off-host operation, restore target
  creation/mount/copy, or local backup mutation occurs.
- Local continuity backup is not complete disaster recovery.
- Off-host encrypted backup is not complete.
- Dry-run restore evidence is not real restore execution.
- No-secret custody/recovery evidence is not real key custody or real key
  recovery implementation.
- qsl-server PR #56 remains bounded end-to-end harness evidence only.
- qsl-attachments PR #37 remains service-local prerequisite evidence only.
- qshield embedded relay/demo evidence remains reference/oracle evidence only.

## Allowed scope

- `inputs/metadata_runtime/key_custody_recovery_no_secret_fixture_v1.json`
- `scripts/ci/metadata_runtime_key_custody_recovery_no_secret_harness.sh`
- `docs/governance/evidence/NA-0361_metadata_runtime_key_custody_key_recovery_no_secret_harness.md`
- `tests/NA-0361_metadata_runtime_key_custody_key_recovery_no_secret_harness_testplan.md`
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
  collection, private key inspection, secret material handling, recovery
  envelope content creation, repository initialization, remote/off-host backup
  tooling, deploy, or rollback.

## Prior key-custody authorization review requirements

Evidence must review:

- live NA-0361 queue scope;
- NA-0360 key custody/key recovery authorization;
- NA-0359 restore-drill dry-run harness;
- NA-0356 key custody/recovery prerequisite plan;
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
- `NO_SECRET_KEY_CUSTODY_HARNESS_READY`
- `NO_SECRET_KEY_RECOVERY_HARNESS_READY`
- `OFF_HOST_BACKUP_NOT_READY`
- `REAL_RESTORE_NOT_AUTHORIZED`

## Fixture schema requirements

The fixture must include schema version, artifact class, source
classification, local backup classification, off-host classification, custody
mode, recovery mode, simulated key IDs, simulated custody records, simulated
recovery-envelope metadata, simulated rotation matrix, simulated old-archive
compatibility matrix, simulated incident-response cases, simulated emergency
access cases, operator runbook markers, no-secret sentinel list, expected
validation outcomes, tamper/negative cases, forbidden operations, claim
boundaries, required markers, backup-plan impact, qsl-server/qsl-attachments
boundary, and qshield demo boundary.

The fixture must contain no real keys, real passphrases, private key text,
tokens, credentials, real recovery-envelope contents, real off-host endpoints,
real backup repository names, real secret paths, private material paths, raw
secret material, or unredacted sensitive operational data.

## Harness behavior requirements

Run:

```bash
bash -n scripts/ci/metadata_runtime_key_custody_recovery_no_secret_harness.sh
python3 -m json.tool inputs/metadata_runtime/key_custody_recovery_no_secret_fixture_v1.json >/dev/null
bash scripts/ci/metadata_runtime_key_custody_recovery_no_secret_harness.sh inputs/metadata_runtime/key_custody_recovery_no_secret_fixture_v1.json
```

Expected:

- JSON validates.
- Fixture hash relationships validate.
- Simulated custody records validate.
- Simulated recovery-envelope metadata validates.
- Simulated rotation matrix validates.
- Simulated old-archive compatibility validates.
- Simulated incident-response entries validate.
- Simulated emergency-access/operator-runbook entries validate.
- Forbidden operations are listed and not executed.
- Proof artifact is written under `/srv/qbuild/tmp/NA-0361_*`.
- Operation counters remain zero.
- Harness exits nonzero on tampered or incomplete fixtures.

## Marker requirements

Required markers:

- `NA0361_KEY_CUSTODY_AUTHORIZATION_OK`
- `NA0361_KEY_RECOVERY_AUTHORIZATION_OK`
- `NA0361_NO_SECRET_KEY_CUSTODY_HARNESS_OK`
- `NA0361_NO_SECRET_KEY_RECOVERY_HARNESS_OK`
- `NA0361_SIMULATED_CUSTODY_FIXTURE_OK`
- `NA0361_SIMULATED_RECOVERY_ENVELOPE_OK`
- `NA0361_SIMULATED_ROTATION_MATRIX_OK`
- `NA0361_INCIDENT_RESPONSE_MARKER_OK`
- `NA0361_OPERATOR_RUNBOOK_MARKER_OK`
- `NA0361_BACKUP_PLAN_IMPACT_OK`
- `NA0361_NO_REAL_KEY_GENERATION_OK`
- `NA0361_NO_KEY_UPLOAD_OK`
- `NA0361_NO_PASSPHRASE_COLLECTION_OK`
- `NA0361_NO_PRIVATE_KEY_INSPECTION_OK`
- `NA0361_NO_SECRET_MATERIAL_OK`
- `NA0361_NO_SECRET_ARTIFACT_OK`
- `NA0361_NO_REAL_KEY_CUSTODY_CLAIM_OK`
- `NA0361_NO_REAL_KEY_RECOVERY_CLAIM_OK`
- `NA0361_NO_DISASTER_RECOVERY_COMPLETE_CLAIM_OK`
- `NA0361_NO_PRODUCTION_READY_CLAIM_OK`
- `NA0361_NO_PUBLIC_INTERNET_READY_CLAIM_OK`
- `NA0361_METADATA_RUNTIME_KEY_CUSTODY_RECOVERY_NO_SECRET_OK`

## Negative/fail-closed requirements

The harness must prove at least these fail-closed cases:

- missing custody record;
- missing recovery-envelope metadata;
- simulated rotation mismatch;
- missing old-archive compatibility;
- prohibited operation field;
- missing claim boundary;
- missing no-secret marker;
- sentinel leak detection.

## Artifact redaction/secret-scan requirements

The fixture, harness, and proof artifact must not contain real key material,
private key content, passphrases, tokens, credentials, real recovery-envelope
contents, real off-host endpoints, or secret material. The proof artifact must
not contain sentinel labels and must report:

- `KEY_CUSTODY_RECOVERY_SECRET_FINDING_COUNT 0`
- `NA0361_SENTINEL_LEAK_FINDING_COUNT 0`
- `NA0361_OPERATION_EXECUTED_COUNT 0`

## Backup-plan impact requirements

Record that no backup-plan update is required for current NA-0361 because
tracked paths stay under qsl-protocol and proof artifacts are temporary under
`/srv/qbuild/tmp`.

Record that future real key custody/recovery, recovery envelopes, durable
secret-related artifacts, off-host targets, source-list changes, scripts,
timers, fstab, system services, monitoring artifacts, real restore artifacts,
backup, restore, deploy, rollback, and public-claim mutation require explicit
backup-plan and local-ops authorization.

## Public-ingress/timing/traffic-shape boundary requirements

Evidence must state that NA-0361 changes no public ingress and does not prove
hidden attachment size, hidden timing metadata, hidden traffic shape, hidden all
metadata, or padding that hides all metadata.

## External-review boundary requirements

Evidence must state that external review remains incomplete and that no-secret
key custody/recovery harness evidence does not prove external-review
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
would reduce friction while not implementing them in NA-0361.

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
bash -n scripts/ci/metadata_runtime_key_custody_recovery_no_secret_harness.sh
python3 -m json.tool inputs/metadata_runtime/key_custody_recovery_no_secret_fixture_v1.json >/dev/null
bash scripts/ci/metadata_runtime_key_custody_recovery_no_secret_harness.sh inputs/metadata_runtime/key_custody_recovery_no_secret_fixture_v1.json
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
python3 formal/model_qsc_handshake_suite_id_bounded.py
python3 formal/run_model_checks.py
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main \
  --allowed inputs/metadata_runtime/key_custody_recovery_no_secret_fixture_v1.json \
  --allowed scripts/ci/metadata_runtime_key_custody_recovery_no_secret_harness.sh \
  --allowed docs/governance/evidence/NA-0361_metadata_runtime_key_custody_key_recovery_no_secret_harness.md \
  --allowed tests/NA-0361_metadata_runtime_key_custody_key_recovery_no_secret_harness_testplan.md \
  --allowed DECISIONS.md \
  --allowed TRACEABILITY.md \
  --allowed docs/ops/ROLLING_OPERATIONS_JOURNAL.md
python3 scripts/ci/qsl_evidence_helper.py link-check
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
bash scripts/ci/classify_ci_scope.sh <changed paths>
```

## CI expectations

- PR goal-lint passes with `Goals: G1, G2, G3, G4, G5`.
- Required checks complete normally.
- `public-safety` remains required and green.
- Merge uses a normal merge commit with `--match-head-commit`.
- No admin bypass, squash, rebase, direct push, or branch deletion command is
  used.

## Successor handoff

After merge and green required checks, closeout should restore exactly:

`NA-0362 -- Metadata Runtime Off-Host Encrypted Backup Target / Tool Implementation Authorization Plan`

No NA-0362 implementation is authorized by NA-0361.
