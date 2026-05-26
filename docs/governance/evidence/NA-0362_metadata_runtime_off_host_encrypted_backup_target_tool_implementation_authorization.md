Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-25

# NA-0362 Metadata Runtime Off-Host Encrypted Backup Target Tool Implementation Authorization

Goals: G1, G2, G3, G4, G5

## Executive summary

NA-0362 is a qsl-protocol governance and authorization lane only. It decides
whether future off-host encrypted backup target/tool implementation can proceed
after NA-0355 selected the target/tool classes, NA-0359 proved a no-secret
restore-drill dry-run harness, and NA-0361 proved a no-secret key custody /
key recovery harness.

Authorization result:

- `NO_SECRET_TARGET_HARNESS_AUTHORIZATION_READY`
- `NO_SECRET_TOOL_HARNESS_AUTHORIZATION_READY`
- `REAL_OFF_HOST_TARGET_IMPLEMENTATION_BLOCKED_BACKUP_PLAN`
- `REAL_OFF_HOST_TARGET_IMPLEMENTATION_BLOCKED_LOCAL_OPS`
- `REAL_OFF_HOST_TARGET_IMPLEMENTATION_BLOCKED_SECRET_HANDLING`
- `REAL_OFF_HOST_TARGET_IMPLEMENTATION_BLOCKED_OPERATOR_RUNBOOK`
- `REAL_OFF_HOST_TARGET_IMPLEMENTATION_BLOCKED_RESTORE_DRILL`
- `REAL_OFF_HOST_TARGET_IMPLEMENTATION_BLOCKED_TARGET_ACCESS`
- `REAL_OFF_HOST_TARGET_IMPLEMENTATION_DEFERRED`
- `REAL_TOOL_IMPLEMENTATION_BLOCKED_INSTALLATION`
- `REAL_TOOL_IMPLEMENTATION_BLOCKED_BACKUP_PLAN`
- `REAL_TOOL_IMPLEMENTATION_BLOCKED_LOCAL_OPS`
- `REAL_TOOL_IMPLEMENTATION_BLOCKED_SECRET_HANDLING`
- `REAL_TOOL_IMPLEMENTATION_BLOCKED_RESTORE_DRILL`
- `REAL_TOOL_IMPLEMENTATION_DEFERRED`

Selected successor:

`NA-0363 -- Metadata Runtime Off-Host Encrypted Backup Target / Tool No-Secret Implementation Harness`

The future NA-0363 lane may implement only a qsl-protocol no-secret fixture and
harness. It may model an SSH/SFTP-compatible target class, a restic-style
encrypted snapshot repository class, simulated snapshot/check/prune/restore
metadata, simulated retention/purge and monitoring/alert matrices, fail-closed
negative cases, proof markers, and claim boundaries. It must not install tools,
initialize repositories, connect to remotes, run backup or restore operations,
create restore targets, generate or upload keys, collect passphrases, inspect
private keys, create recovery-envelope contents, handle secret material, mutate
local backup configuration, or claim real off-host backup or disaster recovery.

## Live NA-0362 scope

The live queue marks NA-0362 READY and requires an authorization result for
future off-host encrypted backup target/tool implementation after no-secret
restore and no-secret key custody/recovery harness evidence.

Allowed current mutation scope is limited to qsl-protocol governance evidence,
this testplan lane, D-0706, TRACEABILITY, and the rolling operations journal.
Optional non-runtime planning artifacts were not needed because the safe
successor can be specified exactly without adding fixture/script code in
NA-0362.

Forbidden current scope includes qsl-server implementation, qsl-attachments
implementation, qshield runtime implementation, qsc/qsp/protocol/crypto
implementation, dependency or workflow mutation, website/public-doc mutation,
README or START_HERE mutation, backup script/timer/fstab mutation, off-host
setup, remote connection, repository initialization, tool installation, backup,
restore, restore target creation/mount/copy, deploy, rollback, real key
generation, key upload, passphrase collection, private-key inspection,
recovery-envelope content creation, and secret material handling.

## Inherited NA-0361 no-secret key custody/recovery harness

NA-0361 added a qsl-protocol-only no-secret fixture and harness for simulated
key custody and simulated key recovery metadata. The harness validates
simulated key IDs, simulated custody records, simulated recovery-envelope
metadata, simulated rotation, simulated old-archive compatibility, simulated
incident response, simulated emergency access, operator-runbook markers,
fail-closed negative cases, no-secret artifact behavior, backup-plan impact,
service boundaries, and claim boundaries.

Inherited proof:

- fixture: `inputs/metadata_runtime/key_custody_recovery_no_secret_fixture_v1.json`
- harness: `scripts/ci/metadata_runtime_key_custody_recovery_no_secret_harness.sh`
- evidence: `docs/governance/evidence/NA-0361_metadata_runtime_key_custody_key_recovery_no_secret_harness.md`
- testplan: `tests/NA-0361_metadata_runtime_key_custody_key_recovery_no_secret_harness_testplan.md`
- operation count: `NA0361_OPERATION_EXECUTED_COUNT 0`
- secret scan marker: `KEY_CUSTODY_RECOVERY_SECRET_FINDING_COUNT 0`
- negative cases: `NA0361_NEGATIVE_CASES_PASSED 8`

NA-0361 remains no-secret harness evidence only. It is not real key custody,
not real key recovery, not a real recovery envelope, not a repository password,
not an off-host backup, and not a real restore.

## Inherited NA-0359 restore-drill dry-run harness

NA-0359 added a qsl-protocol-only no-secret restore-drill dry-run fixture and
harness. The harness validates deterministic manifest/checksum relationships,
writes redacted proof under `/srv/qbuild/tmp`, proves seven fail-closed
negative cases, and emits NA0359 markers.

Inherited proof:

- fixture: `inputs/metadata_runtime/restore_drill_dry_run_fixture_v1.json`
- harness: `scripts/ci/metadata_runtime_restore_drill_dry_run_harness.sh`
- evidence: `docs/governance/evidence/NA-0359_metadata_runtime_restore_drill_dry_run_harness.md`
- operation count: `NA0359_OPERATION_EXECUTED_COUNT 0`
- restore target count: `NA0359_RESTORE_TARGET_CREATED_COUNT 0`
- key operation count: `NA0359_KEY_OPERATION_COUNT 0`
- off-host operation count: `NA0359_OFF_HOST_OPERATION_COUNT 0`
- secret scan marker: `RESTORE_DRY_RUN_SECRET_FINDING_COUNT 0`

NA-0359 remains dry-run evidence only. It is not real restore execution and it
does not prove complete disaster recovery.

## Inherited NA-0355 target/tool selection

NA-0355 selected target/tool classes only:

- target class: SSH/SFTP-compatible off-host host controlled by, or explicitly
  delegated to, the operator;
- tool class: restic-style encrypted snapshot repository with client-side
  encryption, manifest/check verification, prune, and isolated restore support.

The selection remains class-level only. It did not choose a live host, remote
path, provider account, credential, key, passphrase, repository password,
schedule, retention value, alert channel, repository initialization, backup,
restore, deploy, rollback, purge, or secret-handling path.

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
| source classification | `FRESH_SOURCE` |
| authority classification | `COMPLETE_AUTHORITY` |
| CI classification | `COMPLETE_CI` |

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
| remote `HEAD` / `main` | `96b9352bd63e` |
| branch state | detached HEAD, clean |
| PR #37 | merged, merge `96b9352bd63e` |
| viewer permission | `ADMIN` |
| branch protection | strict `rust` required; force pushes disabled; deletions disabled |
| open PRs | none listed |
| latest listed main CI | `rust` success on `96b9352bd63e` |
| source classification | `FRESH_SOURCE` |
| authority classification | `COMPLETE_AUTHORITY` |
| CI classification | `COMPLETE_CI` |

qsl-attachments PR #37 remains service-local prerequisite evidence only. It is
not production/public-internet proof, not complete disaster recovery proof, and
not proof that hot/live backup or partial restore is supported. No
qsl-attachments mutation was performed.

## Local backup/tool/key/off-host/restore evidence refresh

Read-only local evidence:

- `/backup/qsl` is mounted from `/dev/sda1` as ext4.
- `/backup/qsl` reported about 21 GiB used of 916 GiB, about 3 percent.
- `/srv/qbuild` reported about 58 GiB used of 468 GiB, about 13 percent.
- `/usr/local/sbin/qsl-backup` syntax check passed.
- `/usr/local/sbin/qsl-backup preflight` reported the target mounted and daily
  sources present.
- `/usr/local/sbin/qsl-backup list` listed checkpoint snapshots from
  2026-05-17 and daily snapshots through 2026-05-25.
- Manifests and logs exist for current local snapshots through 2026-05-25.
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
- `REAL_KEY_CUSTODY_NOT_READY`
- `REAL_KEY_RECOVERY_NOT_READY`
- `OFF_HOST_TARGET_NOT_READY`
- `OFF_HOST_TOOL_NOT_READY`
- `NO_SECRET_TARGET_TOOL_HARNESS_READY_FOR_AUTHORIZATION`
- `REAL_RESTORE_NOT_AUTHORIZED`

No read-only evidence proves an off-host encrypted target, off-host repository,
repository initialization, repository password/key custody, real recovery
envelope, key rotation implementation, emergency access implementation, real
incident-response procedure, off-host restore drill, remote retention/purge,
remote monitoring, or production operator runbook completion.

## Target implementation authorization decision

| Category | Result |
| --- | --- |
| `NO_SECRET_TARGET_HARNESS_AUTHORIZATION_READY` | selected |
| `REAL_OFF_HOST_TARGET_IMPLEMENTATION_BLOCKED_BACKUP_PLAN` | applies to real target setup |
| `REAL_OFF_HOST_TARGET_IMPLEMENTATION_BLOCKED_LOCAL_OPS` | applies to real target setup |
| `REAL_OFF_HOST_TARGET_IMPLEMENTATION_BLOCKED_SECRET_HANDLING` | applies to real target setup |
| `REAL_OFF_HOST_TARGET_IMPLEMENTATION_BLOCKED_OPERATOR_RUNBOOK` | applies to real target setup |
| `REAL_OFF_HOST_TARGET_IMPLEMENTATION_BLOCKED_RESTORE_DRILL` | applies to reliance on real target setup |
| `REAL_OFF_HOST_TARGET_IMPLEMENTATION_BLOCKED_TARGET_ACCESS` | applies to real host/account/path/quota/identity |
| `REAL_OFF_HOST_TARGET_IMPLEMENTATION_DEFERRED` | applies to real target setup |

Future no-secret qsl-protocol target harness work can be authorized now. It
may model target-class metadata, host identity fields, quota, path, retention,
monitoring, and reject cases using benign simulated values only.

Real SSH/SFTP off-host target setup cannot be authorized now because there is
no approved live target, remote account, remote path, host identity policy,
quota, credential boundary, monitoring policy, backup-plan update, local-ops
authorization, operator runbook, or real restore-drill evidence.

## Tool implementation authorization decision

| Category | Result |
| --- | --- |
| `NO_SECRET_TOOL_HARNESS_AUTHORIZATION_READY` | selected |
| `REAL_TOOL_IMPLEMENTATION_BLOCKED_INSTALLATION` | applies to restic/borg/rclone/age installation or use |
| `REAL_TOOL_IMPLEMENTATION_BLOCKED_BACKUP_PLAN` | applies to real tool install/use and durable artifacts |
| `REAL_TOOL_IMPLEMENTATION_BLOCKED_LOCAL_OPS` | applies to packages, scripts, timers, paths, logs, and monitoring |
| `REAL_TOOL_IMPLEMENTATION_BLOCKED_SECRET_HANDLING` | applies to repository password/key material |
| `REAL_TOOL_IMPLEMENTATION_BLOCKED_RESTORE_DRILL` | applies to reliance on check/prune/restore behavior |
| `REAL_TOOL_IMPLEMENTATION_DEFERRED` | applies to real tool installation/use |

Future no-secret qsl-protocol tool harness work can be authorized now. It may
model restic-style repository metadata, check/snapshot/prune/restore states,
retention/purge behavior, monitoring markers, fail-closed negative cases, and
claim-boundary markers without calling restic or any backup tool.

Real restic-style tool implementation cannot be authorized now. `restic` is
not installed, repository initialization would create real operational state,
and any real tool use would require backup-plan, local-ops, secret-handling,
restore-drill, retention, monitoring, and operator-runbook approval first.

## Future no-secret target/tool implementation bundle

Future repo:

- qsl-protocol only.

Future allowed files:

- `inputs/metadata_runtime/off_host_backup_target_tool_no_secret_fixture_v1.json`
- `scripts/ci/metadata_runtime_off_host_backup_target_tool_no_secret_harness.sh`
- `docs/governance/evidence/NA-0363_metadata_runtime_off_host_backup_target_tool_no_secret_harness.md`
- `tests/NA-0363_metadata_runtime_off_host_backup_target_tool_no_secret_harness_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Future forbidden files and surfaces:

- qsl-server, qsl-attachments, qshield runtime, qsc/qsp/protocol/crypto,
  dependencies, workflows, README, START_HERE, docs/public, website,
  branch-protection/public-safety configuration, local backup scripts, timers,
  fstab, source lists, service units, restore paths, off-host destinations,
  real key custody files, passphrase paths, private keys, and recovery-envelope
  content.

Future commands:

- JSON validation for the fixture.
- Shell syntax validation for the harness.
- Harness execution against the fixture.
- qsl-protocol queue, decisions, scope guard, link-check, leak-scan, classifier,
  goal-lint, advisory health, formatting, and existing metadata/qshield/formal
  validation checks.

Future artifacts:

- tracked fixture/harness/governance/testplan files under qsl-protocol;
- temporary redacted proof under `/srv/qbuild/tmp/NA-0363_*` only.

Future markers:

- the full marker set listed in the future validation section below.

Future PR/order:

1. Implement only the no-secret qsl-protocol fixture and harness.
2. Prove all no-operation/no-secret markers.
3. Keep READY on NA-0363 until a separate closeout restores the next successor.

Future backup-plan update requirement:

- no backup-plan update is required for a pure qsl-protocol no-secret fixture
  and harness if proof remains temporary under `/srv/qbuild/tmp`;
- backup-plan update is required before any real target, repository, tool
  install/use, credentials, durable backup artifacts, restore targets,
  monitoring artifacts, source-list changes, scripts, timers, fstab, services,
  backup, restore, deploy, rollback, or public-claim mutation.

Future key-handling requirement:

- simulated key/recovery labels are allowed only as benign fixture values;
- real key generation, passphrase collection, private-key inspection, key
  upload, secret handling, and recovery-envelope content creation remain
  forbidden until an exact future directive authorizes them.

Future restore dependency requirement:

- simulated restore/check/prune matrices may be modeled;
- real restore execution and real restore target creation/mount/copy remain
  forbidden until a future isolated restore directive authorizes them.

Future monitoring/logging requirement:

- monitoring and alerting may be modeled as no-secret fixture metadata;
- real monitoring setup, alert channels, log destinations, retention jobs, and
  system services remain blocked.

Future public-claim boundary:

- no future no-secret harness may be described as real off-host backup,
  complete disaster recovery, production readiness, public-internet readiness,
  external-review completion, anonymity, metadata-free behavior, untraceable
  behavior, hidden attachment size, hidden timing, hidden traffic shape, or
  padding that hides all metadata.

Future stop conditions:

- stop on any need for a remote connection, repository init, tool install, real
  backup, real restore, key/passphrase/private-key/secret handling, local backup
  mutation, service repo mutation, runtime mutation, dependency/workflow change,
  website/public-doc change, README/START_HERE change, or stronger public
  claim.

## Real off-host target/tool boundary and blocker analysis

| Real action | NA-0362 result | Prerequisites before future authorization |
| --- | --- | --- |
| SSH/SFTP target setup | forbidden/blocked | approved host, account, path, quota, host identity, credentials, backup-plan update, local-ops authorization, operator runbook |
| external disk target | forbidden/blocked | media custody, rotation, mount policy, backup-plan update, restore plan, operator runbook |
| NAS target | forbidden/blocked | network isolation, credentials, retention, monitoring, backup-plan update, restore plan |
| object storage/cloud bucket | forbidden/blocked | provider/account policy, cost/quota, credentials, client-side encryption, backup-plan update, tool authorization |
| offline media target | forbidden/blocked | custody/rotation procedure, restore procedure, monitoring replacement, backup-plan update |
| restic repository init | forbidden/blocked | tool installation, repository password/key custody, backup-plan update, restore/check policy |
| repository check/snapshot/prune/restore | forbidden/blocked | real repo, key custody, retention policy, monitoring, isolated restore plan |
| retention/purge | forbidden/blocked | recovery point policy, failure cleanup, monitoring, operator runbook |
| host identity verification | forbidden/blocked | approved target identity and operator verification procedure |
| credentials/secrets | forbidden/blocked | secret-handling directive, no-secret evidence discipline, backup coverage, recovery procedure |

Each real action is forbidden in NA-0362 because this lane is authorization
planning only and because the necessary backup-plan, local-ops, key custody,
target access, restore-drill, retention, monitoring, and runbook prerequisites
are still unresolved.

## No-secret target/tool fixture and simulated repository authorization analysis

| Option | Safety | Confidence gained | Confidence not gained | Result |
| --- | --- | --- | --- | --- |
| qsl-protocol simulated SSH/SFTP target fixture | safe if values are benign and no network calls occur | validates target boundary fields and fail-closed host/access prerequisites | no real target access, host identity, quota, credentials, or connectivity | recommended |
| qsl-protocol simulated restic-style repository fixture | safe if no restic command or repository init occurs | validates repository state model, operation counters, and claim boundaries | no real repository, password, encryption, check, prune, or restore proof | recommended |
| simulated snapshot/check/prune/restore matrix | safe if fixture-only | validates expected sequencing, reject cases, and operation counters | no real backup or restore behavior | recommended |
| simulated retention/monitoring matrix | safe if fixture-only | validates missed-run, quota, stale snapshot, prune, and alert boundary markers | no real scheduler, alert channel, or log pipeline | recommended |
| qsl-server/qsl-attachments service-local harness | unsafe for NA-0363 scope unless separately authorized | could add service-local detail | would mutate read-only repos and exceed selected qsl-protocol-only lane | deferred/rejected |
| no no-secret implementation | safe but weak | avoids new artifact | leaves target/tool authorization without executable no-secret proof | rejected |

## Tool installation / repository init / remote connection blocker analysis

- Tool installation cannot proceed now.
- Repository initialization cannot proceed now.
- Remote connection cannot proceed now.
- Target identity verification cannot proceed now except as simulated fixture
  metadata.
- Repository check/prune/restore cannot proceed now.
- No-secret modeling is sufficient for the next lane because it can exercise
  boundary logic, operation counters, fail-closed reject cases, marker evidence,
  backup-plan impact, and public-claim discipline without touching real systems.
- Real tool/target work remains blocked by key custody, key recovery,
  backup-plan update, local-ops authorization, restore drill, monitoring,
  retention, and operator runbook prerequisites.

## Key custody/recovery dependency for target/tool implementation

The no-secret target/tool harness can proceed after NA-0361 because NA-0361
proved simulated custody/recovery metadata and no-secret artifact handling.

Real target/tool implementation requires real key custody and real key recovery
before repository initialization, because a restic-style encrypted repository
depends on a repository password/key lifecycle, loss/exposure response,
recovery-envelope handling, rotation, emergency access, and restore validation.

Simulated key/recovery evidence is sufficient only for the no-secret harness.
It is not sufficient to initialize a real repository, store a real secret,
configure a real remote, or rely on a real off-host copy.

## Restore drill / retention / monitoring dependency analysis

Real off-host target/tool work requires a restore drill plan and, before
reliance, isolated restore execution evidence. Retention, purge, prune, check,
and monitoring behavior can be modeled no-secret in NA-0363, but real
monitoring setup, alert channels, scheduler changes, purge jobs, and restore
targets remain blocked.

An operator runbook must precede real target/tool setup. It must define target
identity verification, credential handling, repository secret custody, recovery
envelope inventory, restore drill steps, cleanup, failure handling, missed-run
response, quota pressure response, prune safety, and public-claim boundaries.

## Backup-plan impact and local-ops dependency decision

NA-0362 itself does not require a backup-plan update because changed paths are
qsl-protocol governance/testplan/journal files under `/srv/qbuild/work`, and no
new durable evidence location outside the existing backup posture is required.

Future NA-0363 no-secret qsl-protocol fixture/harness work also does not
require a backup-plan update if proof artifacts remain temporary under
`/srv/qbuild/tmp`.

Backup-plan update and exact local-ops authorization are required before any
real target, real repository, real tool install/use, credentials, durable backup
artifacts, off-host repository, restore target, monitoring artifact,
source-list change, script, timer, fstab, service, backup, restore, deploy,
rollback, key material, recovery envelope, retention/purge job, or public-claim
mutation.

Local workflow-support/history-index work would materially reduce friction, but
it should follow the no-secret target/tool harness unless a later directive
explicitly reorders the lane. The current evidence shows responses and requests
paths are present, directives and journals paths are absent, and the installed
daily source list covers Codex responses but not the whole directives,
requests, journals, or ops history set.

D132 cleanup is not authorized and the preservation bundle remains present.

## Public-ingress/timing/traffic-shape boundary

NA-0362 changes no public ingress and no service runtime. It does not prove
hidden attachment size, hidden timing metadata, hidden traffic shape, hidden all
metadata, or padding that hides all metadata. qsl-server PR #56 remains bounded
end-to-end harness evidence only, qsl-attachments PR #37 remains service-local
prerequisite evidence only, and qshield embedded relay/demo evidence remains
reference/oracle evidence only.

## External-review sensitivity

External review remains incomplete. NA-0362 is internal implementation
authorization planning only. Any stronger public or review-facing statement
requires real key custody evidence, real key recovery evidence, off-host backup
evidence, real restore drill evidence, service evidence, deployment evidence,
monitoring/log evidence, rollback evidence, and review evidence.

## Public claim boundary

NA-0362 introduces no claim of production readiness, public-internet readiness,
external-review completion, anonymity, metadata-free behavior, untraceable
behavior, hidden attachment size, hidden timing metadata, hidden traffic shape,
hidden all metadata, local continuity as complete disaster recovery, off-host
encrypted backup completion, real restore completion, real key custody
implementation, or real key recovery implementation.

Future no-secret NA-0363 evidence must keep the same boundaries.

## Future validation/marker/verification plan

Future NA-0363 no-secret target/tool work should emit or prove:

- `NA0363_TARGET_TOOL_AUTHORIZATION_OK`
- `NA0363_NO_SECRET_TARGET_HARNESS_OK`
- `NA0363_NO_SECRET_TOOL_HARNESS_OK`
- `NA0363_SIMULATED_SSH_SFTP_TARGET_OK`
- `NA0363_SIMULATED_RESTIC_STYLE_REPOSITORY_OK`
- `NA0363_SIMULATED_SNAPSHOT_CHECK_PRUNE_RESTORE_MATRIX_OK`
- `NA0363_SIMULATED_RETENTION_PURGE_MATRIX_OK`
- `NA0363_SIMULATED_MONITORING_ALERT_MATRIX_OK`
- `NA0363_BACKUP_PLAN_IMPACT_OK`
- `NA0363_NO_REMOTE_CONNECTION_OK`
- `NA0363_NO_REPOSITORY_INIT_OK`
- `NA0363_NO_TOOL_INSTALLATION_OK`
- `NA0363_NO_REAL_BACKUP_OK`
- `NA0363_NO_REAL_RESTORE_OK`
- `NA0363_NO_KEY_GENERATION_OK`
- `NA0363_NO_PASSPHRASE_COLLECTION_OK`
- `NA0363_NO_SECRET_MATERIAL_OK`
- `NA0363_NO_OFF_HOST_BACKUP_COMPLETE_CLAIM_OK`
- `NA0363_NO_DISASTER_RECOVERY_COMPLETE_CLAIM_OK`
- `NA0363_NO_PRODUCTION_READY_CLAIM_OK`
- `NA0363_NO_PUBLIC_INTERNET_READY_CLAIM_OK`

If future evidence blocks NA-0363, blocker markers should name the exact
missing fixture field, missing no-secret proof, unsafe command, scope conflict,
backup-plan dependency, local-ops dependency, or claim-boundary failure.

## Workflow-support and history-index future work note

Read-only history paths improved confidence where present. Responses and
requests were available; directives and journals paths were absent at
inspection. A future local-ops workflow/history index remains useful,
especially for qstart/qresume fast-forwarding, response-file writing, bounded
polling, directive manifests, validation profiles, per-directive allow-files,
read-only source/authority refresh, claim-boundary scanning, history indexing,
and backup coverage for directives/requests/journals/ops history folders.

These local-ops improvements should not be implemented by NA-0362. They should
be considered after the no-secret target/tool harness unless a later directive
explicitly selects local-ops as the next successor.

## Selected successor

`NA-0363 -- Metadata Runtime Off-Host Encrypted Backup Target / Tool No-Secret Implementation Harness`

Rationale:

- NA-0355 already selected target/tool classes.
- NA-0359 already proved no-secret restore dry-run discipline.
- NA-0361 already proved no-secret key custody/recovery discipline.
- A qsl-protocol no-secret target/tool harness is the smallest executable next
  proof that increases confidence without touching real off-host systems.
- Real target/tool setup remains blocked by unresolved backup-plan, local-ops,
  secret-handling, target-access, restore-drill, retention, monitoring, and
  operator-runbook prerequisites.

## Rejected alternatives

- direct remote target setup;
- direct repository initialization;
- direct restic/borg/rclone/age installation or use;
- direct backup or restore;
- direct key generation, upload, passphrase collection, private-key inspection,
  recovery-envelope content creation, or secret handling;
- qsl-server or qsl-attachments implementation;
- restore-drill isolated restore authorization before no-secret target/tool
  harness evidence;
- local-ops workflow/history-index as NA-0363 before the no-secret target/tool
  harness;
- external-review, website/public-claim audit, or technical-position-paper work
  before off-host backup gaps are still explicitly modeled.

## Backup-plan impact statement

No NA-0362 backup-plan update is required. Future no-secret NA-0363
qsl-protocol fixture/harness work does not require a backup-plan update if it
stays under qsl-protocol and writes only temporary redacted proof under
`/srv/qbuild/tmp`.

Future real off-host target/tool implementation requires backup-plan update and
exact local-ops authorization before any local system, backup script, timer,
fstab, source-list, tool, repository, key, credential, restore, retention,
purge, monitoring, deploy, rollback, or public-claim mutation.

## Next recommendation

Run NA-0363 as a qsl-protocol-only no-secret target/tool implementation
harness. Do not install tools, initialize a repository, configure or contact a
remote, generate keys, collect passphrases, inspect private keys, create
recovery-envelope contents, or run backup/restore operations until a future
directive explicitly authorizes those steps.
