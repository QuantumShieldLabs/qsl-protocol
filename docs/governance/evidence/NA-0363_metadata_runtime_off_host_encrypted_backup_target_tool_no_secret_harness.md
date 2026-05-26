Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-25

# NA-0363 Metadata Runtime Off-Host Encrypted Backup Target Tool No-Secret Harness

Goals: G1, G2, G3, G4, G5

## Executive summary

NA-0363 implements the qsl-protocol-only no-secret off-host encrypted backup
target/tool fixture and harness authorized by NA-0362. The fixture models
simulated SSH/SFTP target metadata, simulated target identity metadata, a
simulated restic-style encrypted snapshot repository, simulated
snapshot/check/prune/restore relationships, simulated retention/purge metadata,
and simulated monitoring/alert metadata.

This lane does not connect to any remote host, create a remote directory,
initialize a repository, install a tool, run a backup, run a restore, create or
mount a restore target, copy restore payloads, generate or upload keys, collect
passphrases, inspect private keys, create recovery-envelope contents, handle
secret material, deploy, rollback, mutate local backup configuration, or mutate
qsl-server, qsl-attachments, or qshield runtime behavior.

Harness result:

- `NA0363_OPERATION_EXECUTED_COUNT 0`
- `OFF_HOST_TARGET_TOOL_SECRET_FINDING_COUNT 0`
- `NA0363_NEGATIVE_CASES_PASSED 10`
- proof artifact: `/srv/qbuild/tmp/NA-0363_off_host_backup_target_tool_no_secret.zpOUV6/na0363_off_host_backup_target_tool_no_secret_proof.txt`
- proof artifact size: `2398` bytes
- proof artifact sha256: `0796c5b81e74c59c213b9193d416a24fd71d3d3dc7836f38342c5634d9488de4`

Selected successor:

`NA-0364 -- Metadata Runtime Restore Drill Isolated Restore Authorization Plan`

## Live NA-0363 scope

The live queue marks NA-0363 READY and authorizes only a qsl-protocol no-secret
fixture and harness for off-host target/tool boundaries. Allowed implementation
scope is limited to:

- `inputs/metadata_runtime/off_host_backup_target_tool_no_secret_fixture_v1.json`
- `scripts/ci/metadata_runtime_off_host_backup_target_tool_no_secret_harness.sh`
- this evidence file
- `tests/NA-0363_metadata_runtime_off_host_encrypted_backup_target_tool_no_secret_harness_testplan.md`
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

## Inherited NA-0362 authorization

NA-0362 recorded `NO_SECRET_TARGET_HARNESS_AUTHORIZATION_READY` and
`NO_SECRET_TOOL_HARNESS_AUTHORIZATION_READY`. It authorized a future
qsl-protocol-only no-secret target/tool implementation harness and kept real
off-host target setup and real tool implementation blocked by backup-plan,
local-ops, secret-handling, operator-runbook, restore-drill, target-access, and
tool-installation prerequisites.

NA-0362 selected NA-0363 as the exact successor and did not authorize remote
connections, repository init, tool install, backup, restore, key handling,
recovery-envelope content, deploy, rollback, service mutation, runtime
mutation, dependency mutation, workflow mutation, website/public-doc mutation,
or public-claim expansion.

## Inherited NA-0361 no-secret key custody/recovery harness

NA-0361 added a qsl-protocol-only no-secret key custody / key recovery fixture
and harness. It validates simulated key IDs, custody records,
recovery-envelope metadata, rotation, old-archive compatibility, incident
response, emergency access, fail-closed negative cases, and temporary proof
under `/srv/qbuild/tmp`.

NA-0361 remains no-secret evidence only. It is not real key custody, not real
key recovery, not recovery-envelope content creation, and not a real-key
implementation.

## Inherited NA-0359 restore-drill dry-run harness

NA-0359 added a qsl-protocol-only no-secret restore-drill dry-run harness. It
validates fixture manifest/checksum relationships, fail-closed negative cases,
temporary proof under `/srv/qbuild/tmp`, and explicit no-restore markers.

NA-0359 remains dry-run evidence only. It is not real restore execution, not an
isolated restore drill, and not complete disaster recovery.

## Inherited NA-0355 target/tool selection

NA-0355 selected an SSH/SFTP-compatible target class and a restic-style
encrypted snapshot repository class at class level only. It did not select a
live host, account, credential, repository path, key, passphrase, schedule,
retention value, or alert channel.

## Source/authority/CI refresh for qsl-server and qsl-attachments

### qsl-server

| Field | Result |
| --- | --- |
| repository | `QuantumShieldLabs/qsl-server` |
| default branch | `main` |
| local path inspected | `/srv/qbuild/work/NA-0237D/qsl-server` |
| local HEAD | `d40e6003fdf0` |
| remote `HEAD` / `main` | `d40e6003fdf0` |
| branch state | detached HEAD, clean |
| PR #56 | merged, merge `d40e6003fdf0` |
| viewer permission | `ADMIN` |
| branch protection | strict `rust` required; force pushes disabled; deletions disabled; admins enforced |
| open PRs | none listed |
| latest listed main CI | `ci` success on `d40e6003fdf0` |
| classification | `FRESH_SOURCE`, `COMPLETE_AUTHORITY`, `COMPLETE_CI` |

qsl-server PR #56 remains bounded end-to-end harness evidence only. It is not
production proof, public-internet proof, or external-review proof. No
qsl-server mutation was performed.

### qsl-attachments

| Field | Result |
| --- | --- |
| repository | `QuantumShieldLabs/qsl-attachments` |
| default branch | `main` |
| local path inspected | `/srv/qbuild/work/NA-0237D/qsl-attachments` |
| local HEAD | `96b9352bd63e` |
| remote `HEAD` / `main` | `96b9352bd63e` |
| branch state | detached HEAD, clean |
| PR #37 | merged, merge `96b9352bd63e` |
| viewer permission | `ADMIN` |
| branch protection | strict `rust` required; force pushes disabled; deletions disabled |
| open PRs | none listed |
| latest listed main CI | `rust` success on `96b9352bd63e` |
| classification | `FRESH_SOURCE`, `COMPLETE_AUTHORITY`, `COMPLETE_CI` |

qsl-attachments PR #37 remains service-local prerequisite evidence only. It is
not production proof, public-internet proof, or external-review proof. No
qsl-attachments mutation was performed.

## Local backup/tool/key/off-host/restore evidence refresh

Read-only local evidence:

- `/backup/qsl` is mounted at `/dev/sda1`, 916G total, 21G used, 887G free.
- `/srv/qbuild` has 468G total, 58G used, 387G free.
- `qsl-backup preflight` reported the target mounted and daily sources present.
- `qsl-backup list` reported daily snapshots through `daily-20260525T023319-0500`.
- `qsl-backup-daily.timer` is enabled and active.
- `gpg`, `ssh`, and `rsync` are installed.
- `restic`, `borg`, `rclone`, and `age` are not installed.
- `/home/victor/work/qsl/codex/responses` and `/requests` are present.
- `/home/victor/work/qsl/codex/directives` and `/journals` are absent.
- Current backup source status covers `/home/victor/work/qsl/codex/responses`
  and `QSL_BACKUP_PLAN.md`, but not the absent directives/journals paths.
- D132 preservation bundle is present under `/srv/qbuild/tmp/NA-0322_D132_resume_bundle`.

Classification:

- `LOCAL_CONTINUITY_PROVEN`
- `NO_SECRET_DRY_RUN_RESTORE_PROVEN`
- `NO_SECRET_KEY_CUSTODY_RECOVERY_PROVEN`
- `OFF_HOST_TARGET_NOT_READY`
- `OFF_HOST_TOOL_NOT_READY`
- `NO_SECRET_TARGET_TOOL_HARNESS_READY`
- `REAL_KEY_CUSTODY_NOT_READY`
- `REAL_KEY_RECOVERY_NOT_READY`
- `REAL_RESTORE_NOT_AUTHORIZED`

## No-secret target/tool fixture design and schema

The fixture schema is self-contained and deterministic. Required fields cover:

- schema version and artifact class;
- source, local backup, key custody, and restore classifications;
- target mode and tool mode, both `simulated only`;
- simulated SSH/SFTP target metadata;
- simulated target identity metadata;
- simulated restic-style repository metadata;
- simulated snapshot, check, prune, and restore metadata;
- simulated snapshot/check/prune/restore matrix;
- simulated retention/purge matrix;
- simulated monitoring/alert matrix;
- operator runbook markers;
- integrity hashes;
- operation counters;
- no-secret sentinels;
- expected validation outcomes;
- tamper/negative cases;
- forbidden operations;
- claim boundaries;
- required markers;
- backup-plan impact;
- qsl-server, qsl-attachments, and qshield boundaries;
- selected successor.

## No-secret fixture implementation summary

The fixture is
`inputs/metadata_runtime/off_host_backup_target_tool_no_secret_fixture_v1.json`.
It contains only benign fake labels and metadata-only records. It includes one
valid simulated SSH/SFTP target, one valid simulated target identity, one valid
simulated restic-style repository, one valid simulated snapshot/check/prune/
restore matrix, one retention/purge entry, one monitoring/alert entry, and ten
negative cases.

The fixture does not contain real remote hosts, real endpoints, real usernames,
real credentials, real keys, real passphrases, private-key text, tokens, real
repository names, real off-host paths, private material paths, raw secret
material, or unredacted sensitive operational data.

## Harness implementation summary

The harness is
`scripts/ci/metadata_runtime_off_host_backup_target_tool_no_secret_harness.sh`.
It accepts a fixture path argument, validates JSON with Python, validates all
required schema fields, validates simulated metadata relationships, validates
forbidden operation and claim-boundary sets, runs fail-closed negative cases in
memory, writes a no-secret proof artifact under `/srv/qbuild/tmp/NA-0363_*`,
and emits all required `NA0363_*` markers.

The harness does not run backup/restore/off-host/key commands, does not use the
network, does not mutate services, and does not mutate local backup
configuration.

## Harness execution and marker evidence

Commands:

```text
bash -n scripts/ci/metadata_runtime_off_host_backup_target_tool_no_secret_harness.sh
python3 -m json.tool inputs/metadata_runtime/off_host_backup_target_tool_no_secret_fixture_v1.json >/dev/null
bash scripts/ci/metadata_runtime_off_host_backup_target_tool_no_secret_harness.sh inputs/metadata_runtime/off_host_backup_target_tool_no_secret_fixture_v1.json
```

Marker result:

```text
NA0363_TARGET_TOOL_AUTHORIZATION_OK
NA0363_NO_SECRET_TARGET_HARNESS_OK
NA0363_NO_SECRET_TOOL_HARNESS_OK
NA0363_SIMULATED_SSH_SFTP_TARGET_OK
NA0363_SIMULATED_RESTIC_STYLE_REPOSITORY_OK
NA0363_SIMULATED_SNAPSHOT_CHECK_PRUNE_RESTORE_MATRIX_OK
NA0363_SIMULATED_RETENTION_PURGE_MATRIX_OK
NA0363_SIMULATED_MONITORING_ALERT_MATRIX_OK
NA0363_OPERATOR_RUNBOOK_MARKER_OK
NA0363_BACKUP_PLAN_IMPACT_OK
NA0363_NO_REMOTE_CONNECTION_OK
NA0363_NO_REPOSITORY_INIT_OK
NA0363_NO_TOOL_INSTALLATION_OK
NA0363_NO_REAL_BACKUP_OK
NA0363_NO_REAL_RESTORE_OK
NA0363_NO_KEY_GENERATION_OK
NA0363_NO_PASSPHRASE_COLLECTION_OK
NA0363_NO_SECRET_MATERIAL_OK
NA0363_NO_OFF_HOST_BACKUP_COMPLETE_CLAIM_OK
NA0363_NO_DISASTER_RECOVERY_COMPLETE_CLAIM_OK
NA0363_NO_PRODUCTION_READY_CLAIM_OK
NA0363_NO_PUBLIC_INTERNET_READY_CLAIM_OK
NA0363_METADATA_RUNTIME_OFF_HOST_TARGET_TOOL_NO_SECRET_OK
```

The harness also emitted:

```text
NA0363_OPERATION_EXECUTED_COUNT 0
OFF_HOST_TARGET_TOOL_SECRET_FINDING_COUNT 0
NA0363_SENTINEL_LEAK_FINDING_COUNT 0
```

## Negative/fail-closed fixture validation

The harness validates these ten negative cases:

- missing target metadata;
- missing repository metadata;
- snapshot/check mismatch;
- missing retention/purge entry;
- missing monitoring/alert entry;
- prohibited operation counter;
- remote connection attempted flag;
- missing claim boundary;
- missing no-secret marker;
- sentinel leak detection.

The valid fixture passed and every negative case failed closed in memory before
proof artifact creation completed.

## Artifact redaction, secret-scan, cleanup, and `/srv/qbuild/tmp` proof

The proof artifact was written only under `/srv/qbuild/tmp/NA-0363_*`. It is
text, rebuildable, and not durable evidence beyond this summarized path,
checksum, size, markers, and scan result.

The harness scans the fixture and proof for private-key, token, passphrase-like,
and sentinel leakage patterns. The fixture stores sentinel labels only in the
dedicated no-secret sentinel list, and the proof artifact contains no sentinel.

No cleanup of D132 or backup artifacts was performed. No durable artifact
location outside the qsl-protocol repo and `/srv/qbuild/tmp` was required.

## Backup-plan impact and local-ops dependency decision

No backup-plan update is required for NA-0363 itself because the only durable
changes are tracked qsl-protocol fixture/harness/governance/testplan/journal
paths under `/srv/qbuild/work`, and proof artifacts are temporary under
`/srv/qbuild/tmp`.

Future backup-plan and local-ops authorization is required before any real
off-host target, real repository, tool installation/use, credentials, real key
material, real recovery envelope, durable backup artifact, restore target,
monitoring artifact, source-list change, script/timer/fstab/system-service
change, backup, restore, deploy, rollback, or public-claim mutation.

The local workflow-support/history-index request remains relevant. It would
reduce friction for fast-forwarding qstart/qresume worktrees, response writing,
bounded polling, validation profiles, allow-file scope guards, source/authority
helpers, claim-boundary scans, directive/response/journal indexes, and backup
coverage for directives/requests/journals/ops history folders. NA-0363 does not
implement those local-ops items.

## Public-ingress/timing/traffic-shape boundary

NA-0363 adds no public ingress, no service behavior, no website/public-doc
claim, no cover traffic, no padding change, no timing behavior, and no traffic
shape behavior. It does not show that attachment size, timing metadata, traffic
shape, or all metadata is hidden.

## External-review sensitivity

External review remains incomplete. NA-0363 is no-secret fixture/harness
evidence only and must not be presented as external-review completion,
production readiness, public-internet readiness, anonymity, metadata-free
behavior, or untraceable behavior.

## Public claim boundary

The following remain explicitly not ready or prohibited:

- production readiness;
- public-internet readiness;
- external-review completion;
- anonymity;
- metadata-free behavior;
- untraceable behavior;
- hidden attachment size;
- hidden timing metadata;
- hidden traffic shape;
- padding hides all metadata;
- off-host encrypted backup completion;
- complete disaster recovery;
- real restore drill execution;
- real key custody implementation;
- real key recovery implementation.

## Successor selection

Selected:

`NA-0364 -- Metadata Runtime Restore Drill Isolated Restore Authorization Plan`

Rationale: NA-0363 now provides the no-secret target/tool implementation
harness. The next highest-evidence lane is authorization for an isolated
restore drill because real restore execution remains unproven and is a
prerequisite to stronger recovery or public-claim posture. This successor is an
authorization plan only and does not implement NA-0364.

## Rejected alternatives

- Real off-host encrypted backup target/tool blocker resolution: rejected for
  now because real target/tool work still depends on restore, key custody,
  local-ops, backup-plan, target-access, operator-runbook, and secret-handling
  gates.
- QSL local ops workflow support and history index plan: useful but does not
  outrank the real restore authorization gap after target/tool no-secret proof.
- External review readiness gap audit: still future work, but external review
  needs stronger restore/off-host/key evidence first.
- Website/public claim boundary audit: future work only; no public-claim
  strengthening is authorized now.
- Public technical position paper evidence-bounded draft plan: future work
  after more recovery/off-host/key evidence exists.
- Direct remote target setup, repository init, tool install, backup, restore,
  key handling, deploy, rollback, or disaster-recovery claim: rejected as
  forbidden by NA-0363 scope.

## Backup-plan impact statement

Backup-plan impact classification:
`NO_BACKUP_PLAN_UPDATE_REQUIRED_NOW`.

The classification is valid only for this qsl-protocol no-secret harness and
temporary `/srv/qbuild/tmp` proof. It does not authorize real off-host backup,
real restore, real key custody/recovery, durable monitoring artifacts, local
backup source-list changes, scripts, timers, fstab, system services, or public
claim mutation.

## Next recommendation

Close NA-0363 after PR merge and restore the exact successor
`NA-0364 -- Metadata Runtime Restore Drill Isolated Restore Authorization Plan`
without implementing NA-0364.
