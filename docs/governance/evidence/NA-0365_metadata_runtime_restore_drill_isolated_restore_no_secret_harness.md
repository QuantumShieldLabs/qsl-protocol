Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-26

# NA-0365 Metadata Runtime Restore Drill Isolated Restore No-Secret Harness

Goals: G1, G2, G3, G4, G5

## Executive summary

NA-0365 implements the qsl-protocol-only no-secret isolated restore fixture and
harness authorized by NA-0364. The fixture models simulated isolated restore
target metadata, simulated manifest/checksum restore relationships, simulated
old-archive compatibility, simulated cleanup metadata, simulated
monitoring/alert metadata, and simulated operator-runbook metadata.

This lane does not create a real restore target, mount anything, copy backup or
restore payloads, run a backup, run a restore, connect to off-host storage,
initialize a repository, install a tool, generate or upload keys, collect
passphrases, inspect private keys, create recovery-envelope contents, handle
secret material, deploy, rollback, mutate local backup configuration, or mutate
qsl-server, qsl-attachments, or qshield runtime behavior.

Harness result:

- `NA0365_OPERATION_EXECUTED_COUNT 0`
- `NA0365_REAL_RESTORE_TARGET_CREATED_COUNT 0`
- `NA0365_MOUNT_ATTEMPT_COUNT 0`
- `NA0365_COPY_ATTEMPT_COUNT 0`
- `NA0365_REAL_BACKUP_COUNT 0`
- `NA0365_REAL_RESTORE_COUNT 0`
- `NA0365_KEY_OPERATION_COUNT 0`
- `NA0365_OFF_HOST_OPERATION_COUNT 0`
- `ISOLATED_RESTORE_SECRET_FINDING_COUNT 0`
- `NA0365_NEGATIVE_CASES_PASSED 12`
- proof artifact: `/srv/qbuild/tmp/NA-0365_restore_drill_isolated_restore_no_secret.DhfhzI/na0365_restore_drill_isolated_restore_no_secret_proof.txt`
- proof artifact size: `1988` bytes
- proof artifact sha256: `aa84f05743695537f41b8a11a3dc3996dfdd722c07cd23c955ce0fbcfa7f80a1`

Selected successor:

`NA-0366 -- Metadata Runtime Off-Host Encrypted Backup Real Target / Tool Blocker Resolution`

## Live NA-0365 scope

The live queue marks NA-0365 READY and authorizes only a qsl-protocol no-secret
isolated restore fixture and harness. Allowed implementation scope is limited
to:

- `inputs/metadata_runtime/restore_drill_isolated_restore_no_secret_fixture_v1.json`
- `scripts/ci/metadata_runtime_restore_drill_isolated_restore_no_secret_harness.sh`
- this evidence file
- `tests/NA-0365_metadata_runtime_restore_drill_isolated_restore_no_secret_harness_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Forbidden scope includes qsl-server mutation, qsl-attachments mutation, qshield
runtime mutation, qsc/qsp/protocol/crypto/key-schedule implementation change,
dependency or workflow mutation, website/public-doc mutation, README or
START_HERE mutation, backup script/timer/fstab mutation, local backup source
list mutation, real off-host setup, remote connection, repository
initialization, tool installation, backup, restore, restore target
creation/mount/copy, deploy, rollback, real key generation, key upload,
passphrase collection, private-key inspection, recovery-envelope content
creation, and secret material handling.

## Inherited NA-0364 authorization

NA-0364 recorded:

- `NO_SECRET_ISOLATED_RESTORE_HARNESS_AUTHORIZATION_READY`
- `NO_SECRET_RESTORE_TARGET_HARNESS_AUTHORIZATION_READY`
- real isolated restore blocked by backup-plan, local-ops, secret-handling,
  operator-runbook, real key custody/recovery, and target-isolation gaps;
- real restore target creation/mount/copy blocked by backup-plan, local-ops,
  secret-handling, cleanup, and monitoring gaps.

NA-0364 selected NA-0365 as the exact successor and did not authorize real
restore execution, real target creation, mount, copy, off-host setup, backup,
restore, deploy, rollback, key handling, recovery-envelope content, service
mutation, runtime mutation, dependency mutation, workflow mutation, website or
public-doc mutation, or public-claim expansion.

## Inherited NA-0363 no-secret target/tool harness

NA-0363 added a qsl-protocol-only no-secret off-host target/tool fixture and
harness. It validates simulated SSH/SFTP target metadata, simulated target
identity metadata, simulated restic-style repository metadata, simulated
snapshot/check/prune/restore relationships, simulated retention/purge metadata,
simulated monitoring/alert metadata, ten fail-closed negative cases, forbidden
operation counters, no-secret proof under `/srv/qbuild/tmp`, and claim
boundaries.

NA-0363 remains no-secret harness evidence only. It is not a real off-host
target, not a real repository, not tool installation, not off-host backup
completion, and not restore execution.

## Inherited NA-0361 no-secret key custody/recovery harness

NA-0361 added a qsl-protocol-only no-secret key custody / key recovery fixture
and harness. It validates simulated key IDs, custody records,
recovery-envelope metadata, rotation, old-archive compatibility, incident
response, emergency access, fail-closed negative cases, and temporary proof
under `/srv/qbuild/tmp`.

NA-0361 remains no-secret evidence only. It is not real key custody, not real
key recovery, not recovery-envelope content creation, and not a real-key
implementation.

## Inherited NA-0359 restore dry-run harness

NA-0359 added a qsl-protocol-only no-secret restore-drill dry-run fixture and
harness. It validates deterministic manifest/checksum relationships, writes
redacted proof under `/srv/qbuild/tmp`, proves seven fail-closed negative
cases, and emits explicit no-operation markers.

NA-0359 remains dry-run evidence only. It is not real restore execution, not an
isolated restore target, and not complete disaster recovery.

## Source/authority/CI refresh for qsl-server and qsl-attachments

### qsl-server

| Field | Result |
| --- | --- |
| repository | `QuantumShieldLabs/qsl-server` |
| default branch | `main` |
| local path inspected | `/srv/qbuild/work/NA-0237D/qsl-server` |
| local HEAD | `d40e6003fdf0` |
| PR #56 | merged, merge `d40e6003fdf0` |
| viewer permission | `ADMIN` |
| branch protection | strict `rust` required; force pushes disabled; deletions disabled; admins enforced |
| open PRs | none listed |
| latest listed main CI | `ci` success on `d40e6003fdf0` |
| classification | `FRESH_SOURCE`, `COMPLETE_AUTHORITY`, `COMPLETE_CI` |

qsl-server PR #56 remains bounded end-to-end harness evidence only. It is not
production proof, public-internet proof, off-host backup proof, or
external-review proof. No qsl-server mutation was performed.

### qsl-attachments

| Field | Result |
| --- | --- |
| repository | `QuantumShieldLabs/qsl-attachments` |
| default branch | `main` |
| local path inspected | `/srv/qbuild/work/NA-0237D/qsl-attachments` |
| local HEAD | `96b9352bd63e` |
| PR #37 | merged, merge `96b9352bd63e` |
| viewer permission | `ADMIN` |
| branch protection | strict `rust` required; force pushes disabled; deletions disabled |
| open PRs | none listed |
| latest listed main CI | `rust` success on `96b9352bd63e` |
| classification | `FRESH_SOURCE`, `COMPLETE_AUTHORITY`, `COMPLETE_CI` |

qsl-attachments PR #37 remains service-local prerequisite evidence only. It is
not production/public-internet proof, not complete disaster recovery proof, and
not proof that hot/live backup or partial restore is supported. No
qsl-attachments mutation was performed.

## Local backup/tool/key/off-host/restore evidence refresh

Read-only local evidence:

- `/backup/qsl` is mounted from `/dev/sda1` as ext4.
- `/backup/qsl` reported 916G total, 21G used, and 886G available.
- `/srv/qbuild` reported 468G total, 58G used, and 387G available.
- `/usr/local/sbin/qsl-backup` syntax check passed.
- `/usr/local/sbin/qsl-backup preflight` reported the target mounted and daily
  sources present.
- `/usr/local/sbin/qsl-backup list` listed checkpoint snapshots from
  2026-05-17 and daily snapshots through `daily-20260526T023618-0500`.
- Manifests and logs exist for current local snapshots through 2026-05-26.
- `qsl-backup-daily.timer` is enabled and waiting.
- D132 preservation bundle remains present under `/srv/qbuild/tmp`.
- Installed tool discovery found `gpg`, `ssh`, and `rsync`.
- Installed tool discovery did not find `restic`, `borg`, `rclone`, or `age`.

Backup/history evidence:

- `QSL_BACKUP_PLAN.md` states the platter backup is local continuity and should
  not be the only disaster recovery copy.
- Backup status lists daily sources for `/srv/qbuild/work`, `/srv/qbuild/tmp`,
  qbuild mirrors, qbuild evidence/logs/archive, Codex logs, Codex responses,
  and `QSL_BACKUP_PLAN.md`.
- `/home/victor/work/qsl/codex/responses` and `/requests` were present.
- `/home/victor/work/qsl/codex/directives` and `/journals` were absent at
  inspection.
- `/home/victor/work/qsl/codex/ops/backup` was present, but the installed daily
  source list does not cover the whole ops tree.

Classification:

- `LOCAL_CONTINUITY_PROVEN`
- `NO_SECRET_DRY_RUN_RESTORE_PROVEN`
- `NO_SECRET_KEY_CUSTODY_RECOVERY_PROVEN`
- `NO_SECRET_TARGET_TOOL_PROVEN`
- `NO_SECRET_ISOLATED_RESTORE_HARNESS_READY`
- `REAL_ISOLATED_RESTORE_BLOCKED`
- `REAL_KEY_CUSTODY_NOT_READY`
- `REAL_KEY_RECOVERY_NOT_READY`
- `OFF_HOST_BACKUP_NOT_READY`

No read-only evidence proves a real off-host encrypted repository, real
repository initialization, real repository password/key custody, real recovery
envelope, real key rotation implementation, emergency access implementation,
off-host restore drill, remote retention/purge, remote monitoring, or
production operator runbook completion.

## No-secret isolated restore fixture design and schema

The fixture schema lives at
`inputs/metadata_runtime/restore_drill_isolated_restore_no_secret_fixture_v1.json`
and includes schema version, artifact class, source classification, local
backup classification, key custody classification, off-host target/tool
classification, restore classification, simulated-only restore mode, simulated
restore target mode, simulated restore target metadata, simulated manifest
metadata, simulated checksum metadata, simulated old-archive compatibility
metadata, simulated cleanup metadata, simulated monitoring/alert metadata,
simulated operator-runbook metadata, integrity hashes, operation counters,
no-secret sentinels, expected validation outcomes, tamper cases, forbidden
operations, claim boundaries, required markers, backup-plan impact, and
qsl-server / qsl-attachments / qshield boundaries.

The fixture contains only benign fake values. It contains no real backup
payloads, real restore paths, real mount paths, real copied payload paths, real
remote hosts, real endpoints, real usernames, real credentials, real keys, real
passphrases, private key text, tokens, real repository names, real off-host
paths, private material paths, raw secret material, or unredacted sensitive
operational data.

## No-secret fixture implementation summary

The fixture includes:

- one simulated isolated restore target metadata entry;
- one simulated manifest entry and one checksum entry with deterministic
  SHA-256 validation;
- one simulated old-archive compatibility entry tied to NA-0361-style
  simulated key/recovery metadata;
- one simulated cleanup entry;
- one simulated monitoring/alert entry;
- one simulated operator-runbook entry;
- twelve negative/fail-closed cases;
- explicit no-secret sentinel labels that must never appear in proof output.

## Harness implementation summary

The harness lives at
`scripts/ci/metadata_runtime_restore_drill_isolated_restore_no_secret_harness.sh`.
It accepts a fixture path argument, validates that temporary proof output stays
under `/srv/qbuild/tmp`, validates JSON with Python, checks required schema
fields, validates simulated restore target, manifest/checksum, old-archive
compatibility, cleanup, monitoring, and runbook relationships, recomputes
fixture hashes, validates forbidden operations and public-claim boundaries, runs
all negative cases in memory, writes a redacted text proof artifact under
`/srv/qbuild/tmp/NA-0365_*`, emits all required NA0365 markers, and exits
nonzero on tampered or incomplete fixtures.

The harness does not call backup, restore, off-host, deploy, rollback, key,
passphrase, service, systemd, mount, network, tool-installation, repository, or
local backup mutation commands.

## Harness execution and marker evidence

Local execution:

```text
bash -n scripts/ci/metadata_runtime_restore_drill_isolated_restore_no_secret_harness.sh
python3 -m json.tool inputs/metadata_runtime/restore_drill_isolated_restore_no_secret_fixture_v1.json >/dev/null
bash scripts/ci/metadata_runtime_restore_drill_isolated_restore_no_secret_harness.sh inputs/metadata_runtime/restore_drill_isolated_restore_no_secret_fixture_v1.json
```

Result:

- proof artifact:
  `/srv/qbuild/tmp/NA-0365_restore_drill_isolated_restore_no_secret.DhfhzI/na0365_restore_drill_isolated_restore_no_secret_proof.txt`
- proof artifact size: `1988` bytes
- proof artifact SHA-256:
  `aa84f05743695537f41b8a11a3dc3996dfdd722c07cd23c955ce0fbcfa7f80a1`
- `ISOLATED_RESTORE_SECRET_FINDING_COUNT 0`
- `NA0365_OPERATION_EXECUTED_COUNT 0`
- `NA0365_REAL_RESTORE_TARGET_CREATED_COUNT 0`
- `NA0365_MOUNT_ATTEMPT_COUNT 0`
- `NA0365_COPY_ATTEMPT_COUNT 0`
- `NA0365_REAL_BACKUP_COUNT 0`
- `NA0365_REAL_RESTORE_COUNT 0`
- `NA0365_KEY_OPERATION_COUNT 0`
- `NA0365_OFF_HOST_OPERATION_COUNT 0`
- `NA0365_NEGATIVE_CASES_PASSED 12`

Required marker output:

- `NA0365_ISOLATED_RESTORE_AUTHORIZATION_OK`
- `NA0365_NO_SECRET_ISOLATED_RESTORE_HARNESS_OK`
- `NA0365_NO_SECRET_RESTORE_TARGET_HARNESS_OK`
- `NA0365_SIMULATED_RESTORE_TARGET_OK`
- `NA0365_SIMULATED_MANIFEST_CHECKSUM_RESTORE_OK`
- `NA0365_SIMULATED_OLD_ARCHIVE_COMPATIBILITY_OK`
- `NA0365_SIMULATED_CLEANUP_MONITORING_RUNBOOK_OK`
- `NA0365_BACKUP_PLAN_IMPACT_OK`
- `NA0365_NO_REAL_RESTORE_TARGET_CREATION_OK`
- `NA0365_NO_MOUNT_OK`
- `NA0365_NO_COPY_OK`
- `NA0365_NO_REAL_BACKUP_OK`
- `NA0365_NO_REAL_RESTORE_OK`
- `NA0365_NO_KEY_GENERATION_OK`
- `NA0365_NO_PASSPHRASE_COLLECTION_OK`
- `NA0365_NO_SECRET_MATERIAL_OK`
- `NA0365_NO_RESTORE_DRILL_COMPLETE_CLAIM_OK`
- `NA0365_NO_DISASTER_RECOVERY_COMPLETE_CLAIM_OK`
- `NA0365_NO_PRODUCTION_READY_CLAIM_OK`
- `NA0365_NO_PUBLIC_INTERNET_READY_CLAIM_OK`
- `NA0365_METADATA_RUNTIME_ISOLATED_RESTORE_NO_SECRET_OK`

## Negative/fail-closed fixture validation

The harness mutates fixture copies in memory and requires all negative cases to
fail closed:

- missing restore target metadata;
- missing manifest;
- checksum mismatch;
- missing old-archive compatibility;
- missing cleanup;
- missing monitoring/alert;
- prohibited operation field;
- mount attempted;
- copy attempted;
- missing claim boundary;
- missing no-secret marker;
- sentinel leak detection.

No negative fixture is committed. No negative case performs any restore,
target, mount, copy, backup, off-host, key, secret, deploy, rollback, service,
or local backup operation.

## Artifact redaction, secret-scan, cleanup, and `/srv/qbuild/tmp` proof

The harness writes only a small no-secret text proof artifact under
`/srv/qbuild/tmp/NA-0365_*`. The proof records checksums, size, marker output,
classification summaries, operation counts, negative-case count, and
secret-scan result.

The fixture contains sentinel labels only under its dedicated no-secret
sentinel list. The proof artifact contains no sentinel. The harness scans
fixture-visible text and proof text for common private-key, token, passphrase,
and credential-shaped patterns and records
`ISOLATED_RESTORE_SECRET_FINDING_COUNT 0`.

No cleanup of D132 or backup artifacts was performed. No durable artifact
outside tracked qsl-protocol files and `/srv/qbuild/tmp` proof was created.

## Backup-plan impact and local-ops dependency decision

Backup-plan impact classification:

`NO_BACKUP_PLAN_UPDATE_REQUIRED_NOW`

Reason: durable changes stay in tracked qsl-protocol paths under
`/srv/qbuild/work`, and proof artifacts are temporary under
`/srv/qbuild/tmp/NA-0365_*`.

Future backup-plan and local-ops authorization remains required for any real
restore target, durable restored artifact, real key material, recovery
envelope, off-host target, source-list change, script/timer/fstab/system
service, monitoring artifact, backup, restore, deploy, rollback, public-claim
mutation, or durable local history/index storage change.

Local workflow-support/history-index work would reduce friction, especially
qstart/qresume fast-forwarding, response-file writing, bounded polling helpers,
machine-readable manifests, validation profiles, per-directive allow-files,
source/authority helpers, claim-boundary scanners, directive/response/journal
indexes, and backup coverage for directives/requests/journals/ops history
folders. NA-0365 does not implement those items.

## Public-ingress/timing/traffic-shape boundary

NA-0365 changes no public ingress, timing behavior, traffic-shape behavior,
padding behavior, cover-traffic behavior, website content, service runtime,
public docs, or public-claim surface. It does not claim that attachment size,
timing metadata, traffic shape, or all metadata is hidden.

## External-review sensitivity

External review remains incomplete. NA-0365 is no-secret fixture/harness
evidence only and is not production proof, public-internet proof,
external-review completion, real restore execution, off-host backup completion,
real key custody, real key recovery, or complete disaster recovery.

## Public claim boundary

NA-0365 introduces no claim of:

- production readiness;
- public-internet readiness;
- external-review completion;
- anonymity;
- metadata-free behavior;
- untraceable behavior;
- hidden attachment size;
- hidden timing metadata;
- hidden traffic shape;
- padding hiding all metadata;
- complete disaster recovery;
- off-host encrypted backup completion;
- real restore drill execution;
- real restore completion;
- real key custody implementation;
- real key recovery implementation.

Any stronger claim requires real key custody evidence, real key recovery
evidence, off-host backup evidence, real restore drill evidence, service
evidence, deployment evidence, monitoring/log evidence, rollback evidence, and
review evidence.

## Successor selection

Selected successor:

`NA-0366 -- Metadata Runtime Off-Host Encrypted Backup Real Target / Tool Blocker Resolution`

Rationale: NA-0365 now provides the no-secret isolated restore harness that was
authorized by NA-0364. The next highest metadata-runtime recovery blocker is
the real off-host encrypted backup target/tool gap: `restic`, `borg`, `rclone`,
and `age` were not installed during read-only discovery, no real off-host target
or encrypted repository is configured, and real isolated restore remains
blocked until off-host target/tool, repository, key custody/recovery,
backup-plan, local-ops, cleanup, and monitoring prerequisites are addressed.

## Rejected alternatives

- Real key custody / key recovery implementation authorization plan: important
  future work, but the immediate missing proof after no-secret target/tool and
  no-secret isolated restore is the real target/tool blocker.
- QSL Local Ops Codex Workflow Support and History Index Plan: useful
  operational work, but it does not unblock real off-host repository or restore
  evidence by itself.
- External review readiness gap audit: future work only; external review needs
  stronger recovery/off-host/key evidence first.
- Website / public claim boundary audit: future work only; no public claim is
  changed by NA-0365.
- Public technical position paper evidence-bounded draft plan: future work
  after stronger recovery/off-host/key evidence exists.

## Backup-plan impact statement

No backup-plan update is required for NA-0365. The classification is valid only
for this qsl-protocol no-secret harness and temporary `/srv/qbuild/tmp` proof.
It does not authorize real restore targets, real restore payloads, off-host
targets, repositories, tools, credentials, key material, recovery envelopes,
durable monitoring artifacts, source-list changes, scripts, timers, fstab,
services, backup, restore, deploy, rollback, or public-claim mutation.

## Next recommendation

Close NA-0365 only after the implementation PR merges and post-merge
public-safety is green. Restore exactly one READY successor:

`NA-0366 -- Metadata Runtime Off-Host Encrypted Backup Real Target / Tool Blocker Resolution`
