Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-24

# NA-0355 Metadata Runtime Off-Host Encrypted Backup Target Tool Selection Plan

Goals: G1, G2, G3, G4, G5

## Executive summary

NA-0355 records a target/tool selection result after NA-0354 authorization. It
is a qsl-protocol governance and planning lane only.

Selection result:

- preferred target class: SSH/SFTP-compatible off-host host controlled by the
  operator or explicitly delegated to the operator;
- preferred tool class: restic-style encrypted snapshot repository with
  client-side encryption, manifest/check verification, prune, and isolated
  restore support;
- implementation classification: `TARGET_TOOL_SELECTION_PARTIAL`;
- gating classification: `TARGET_TOOL_DEFERRED_KEY_CUSTODY`.

This selection is a class-level strategy, not a destination setup. NA-0355 did
not choose a live host, remote path, provider account, credential, key,
passphrase, repository password, schedule, retention value, or alert channel.
No off-host repository was initialized and no backup, restore, deploy,
rollback, purge, key generation, key upload, passphrase collection, or secret
handling occurred.

Selected successor:

`NA-0356 -- Metadata Runtime Key Custody / Key Recovery Prerequisite Plan`

## Live NA-0355 scope

The live queue marks NA-0355 READY and requires target/tool selection before
any implementation harness, local-ops mutation, secret handling, or live backup
operation. The live scope permits read-only refresh of NA-0354 evidence,
qsl-server and qsl-attachments authority, and local backup posture. It forbids
qsl-server, qsl-attachments, qshield runtime, qsc/qsp, protocol, crypto,
dependency, workflow, website, README, START_HERE, docs/public, backup script,
timer, fstab, local backup source list, off-host setup, key handling, backup,
restore, deploy, rollback, or public-claim mutation.

## Inherited NA-0354 authorization

NA-0354 recorded `OFF_HOST_IMPLEMENTATION_DEFERRED`. It authorized target/tool
selection as the next prerequisite and preserved these blockers:

- target selection;
- encryption tool selection;
- key custody and key recovery;
- secret handling;
- restore drill;
- retention and purge;
- monitoring and alerting;
- backup-plan update;
- local-ops authorization.

NA-0354 also preserved the boundaries that qsl-server PR #56 is bounded
end-to-end harness evidence only, qsl-attachments PR #37 is service-local
prerequisite evidence only, and qshield embedded relay/demo evidence is
reference/oracle evidence only.

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
| branch protection | present, strict `rust` required |
| force pushes | disabled |
| deletions | disabled |
| admins enforced | enabled |
| open PRs | none listed |
| latest listed main CI | `ci` success on `d40e6003fdf0` |
| source classification | `FRESH_SOURCE` |
| authority classification | `COMPLETE_AUTHORITY` |
| CI classification | `COMPLETE_CI` |

qsl-server remains transport-only. No qsl-server mutation was performed.

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
| branch protection | present, strict `rust` required |
| force pushes | disabled |
| deletions | disabled |
| admins enforced | not enabled in current protection |
| open PRs | none listed |
| latest listed main CI | `rust` success on `96b9352bd63e` |
| source classification | `FRESH_SOURCE` |
| authority classification | `COMPLETE_AUTHORITY` |
| CI classification | `COMPLETE_CI` |

qsl-attachments remains a single-node local-disk attachment service. Current
service-local evidence still supports cold full-root backup/restore plus
matching service configuration only; hot/live backup and partial restore remain
unsupported. No qsl-attachments mutation was performed.

## Local backup/installed tool/off-host evidence refresh

Read-only local evidence:

- `/backup/qsl` is mounted from `/dev/sda1` as ext4.
- `/backup/qsl` reported about 20 GiB used of 916 GiB, about 3 percent.
- `/srv/qbuild` reported about 57 GiB used of 468 GiB, about 13 percent.
- `/usr/local/sbin/qsl-backup` syntax check passed.
- `/usr/local/sbin/qsl-backup preflight` reported the target mounted and daily
  sources present.
- `/usr/local/sbin/qsl-backup list` listed checkpoint snapshots from
  2026-05-17 and daily snapshots through 2026-05-24.
- manifests and logs exist for the listed local snapshots.
- `qsl-backup-daily.timer` is enabled and waiting.
- D132 preservation bundle remains present under `/srv/qbuild/tmp`.

Installed tool availability:

| Tool | Local result | Selection impact |
| --- | --- | --- |
| `restic` | not found | can be selected only as future authorized tool class |
| `borg` | not found | deferred/fallback only |
| `rclone` | not found | deferred |
| `age` | not found | deferred |
| `gpg` | found | available but not preferred as primary backup system |
| `ssh` | found | supports preferred SSH/SFTP target class |
| `rsync` | found | supports current local continuity and possible transport fallback |

Backup status/plan evidence:

- local daily sources include `/srv/qbuild/work`, `/srv/qbuild/tmp`, qbuild
  mirrors, qbuild evidence/logs/archive, Codex logs, Codex responses, and
  `QSL_BACKUP_PLAN.md`;
- current local plan states that the platter backup is local continuity and
  should not be the only disaster recovery copy;
- `/home/victor/work/qsl/codex/responses` and `/requests` were present;
- `/home/victor/work/qsl/codex/directives` and `/journals` were absent at
  inspection;
- `/home/victor/work/qsl/codex/ops/backup` was present but is not listed as a
  daily source group in the inspected installed status.

Classification:

- `LOCAL_CONTINUITY_PROVEN`
- `TARGET_SELECTION_READY`
- `TOOL_SELECTION_READY`
- `OFF_HOST_BACKUP_NOT_READY`

No read-only evidence proves an off-host encrypted backup destination,
configured remote repository, target credentials, encryption-key custody, key
recovery, key rotation, off-host restore drill, remote retention/purge,
monitoring/alerting, or operator runbook.

## Target class option analysis

| Target class | Local evidence | Security/operations assessment | Result |
| --- | --- | --- | --- |
| external disk | local platter backup already exists, same host | removable media can improve separation if physically removed, but monitoring and operator discipline are weak | deferred |
| NAS | none configured | may share site and ransomware exposure unless isolated; restore can be testable but target hardening is separate work | deferred |
| SSH/SFTP host | `ssh` installed; no remote configured | provider-independent, simple audit model, compatible with restic/borg classes, easier to isolate restore; requires host, account, quota, and credential plan | recommended target class |
| object storage / S3-compatible | no local config or tool present | strong durability, but credentials, cost/quota, provider policy, and rclone/restic support must be authorized first | deferred candidate |
| Backblaze B2 or similar object storage | no local config or tool present | similar to object storage with explicit account/provider dependency | deferred candidate |
| encrypted cloud bucket | no local config or tool present | acceptable only with client-side encryption not solely provider-managed encryption | deferred candidate |
| removable offline media | no current off-host media workflow | strong ransomware separation, poor automation and monitoring | deferred |
| no target / deferred | current off-host proof absent | would leave same-host continuity as the only current backup posture | rejected as final strategy |

The selected target class is an SSH/SFTP-compatible off-host host. The exact
host, account, path, quota, network policy, and credentials remain future
operator/local-ops prerequisites.

## Encryption/tool class option analysis

| Tool class | Local evidence | Security/operations assessment | Result |
| --- | --- | --- | --- |
| existing qsl-backup extension with encrypted archive stage | current qsl-backup is local rsync snapshot only | preserves current model but would need new archive/encryption/retention/restore logic and local-ops mutation | deferred |
| restic | not installed | integrated client-side encryption, snapshots, check, prune, and restore; repository password/key custody is the main blocker | recommended tool class |
| borg | not installed | strong encrypted repository model, usually SSH-oriented; remote borg availability and repo compatibility add operational coupling | secondary fallback |
| rclone crypt | not installed | useful transport/cloud layer but weaker as sole backup/restore policy without a backup engine | deferred |
| age-wrapped tar/rsync | `age` not installed | clear public-recipient model, but archive/incremental/retention/restore semantics must be built around it | deferred |
| gpg-wrapped archive | `gpg` installed | available but higher operator/key management complexity and poor incremental restore ergonomics | rejected as primary |
| ssh/rsync plus separate encryption layer | `ssh` and `rsync` installed | transport is available, but separate encryption plus retention/check/restore logic is too easy to under-specify | fallback only |
| no tool / deferred | current off-host proof absent | off-host copy without client-side encryption is not acceptable | rejected |

The selected tool class is a restic-style encrypted snapshot repository. Restic
itself is not installed now, so future implementation requires explicit
local-ops/package authorization and key custody planning before install,
repository init, backup, prune, check, or restore.

## Target/tool compatibility and staging decision

Classification:

- `TARGET_TOOL_SELECTION_PARTIAL`
- `TARGET_TOOL_DEFERRED_KEY_CUSTODY`
- `TARGET_TOOL_DEFERRED_LOCAL_OPS`
- `TARGET_TOOL_DEFERRED_RESTORE_DRILL`

Target selection can be made at class level now: SSH/SFTP-compatible off-host
host. Tool selection can be made at class level now: restic-style encrypted
snapshot repository. Implementation must be split into later lanes because:

- key custody and key recovery define the repository password/key lifecycle;
- local-ops must authorize tool installation, remote account/host details,
  backup-plan update, source-list changes, retention, schedule, logs, and
  alerting;
- restore-drill planning must define isolated restore target, verification,
  cleanup, and no-secret evidence before real trust is assigned to a remote
  encrypted copy;
- a no-secret qsl-protocol fixture harness may precede real setup, but it must
  not initialize a repository or touch a remote target.

## Key custody/key recovery dependency analysis

NA-0355 created no key, uploaded no key, collected no passphrase, printed no
secret material, and ran no secret-dependent test.

Target/tool selection is meaningful only at class level before key custody.
The selected restic-style tool class requires a future plan for:

- custody owner and accountable operator role;
- repository password or key generation authority;
- storage location and access controls;
- offline recovery envelope;
- recovery test that does not expose secret material in evidence;
- rotation and revocation procedure;
- emergency access and lost/exposed key response;
- no-secret artifact rule for logs, manifests, PR bodies, response files, and
  journals;
- secret scanning of any generated evidence before merge or handoff.

Because those decisions precede repository initialization, the exact successor
must be the key custody/key recovery prerequisite plan.

## Restore drill/retention/purge/monitoring dependency analysis

Future target/tool work must support:

- no-secret dry-run restore modeling before any live operation;
- isolated real restore only after key custody is authorized;
- manifest/check verification and repository consistency checks;
- clear recovery point objective and recovery time objective for off-host
  evidence;
- retention and prune policy that does not delete the only usable recovery
  point;
- failed-backup and failed-prune cleanup;
- failed-restore cleanup of isolated restore targets;
- monitoring for missed runs, failed backup/check/prune/restore, quota
  pressure, and stale snapshots;
- operator-visible redacted evidence without endpoint secrets or credentials.

## Backup-plan impact and local-ops dependency decision

NA-0355 itself does not require a backup-plan update because changed paths are
qsl-protocol governance/testplan/journal files under `/srv/qbuild/work`, and
the required response file stays under the existing Codex responses path.

Future off-host target/tool implementation does require backup-plan update and
local-ops authorization before any target, key, source-list, tool install,
script, timer, fstab, system-service, repository, restore-drill, retention,
purge, monitoring, backup, restore, deploy, rollback, or public-claim mutation.

Workflow-support/history-index coverage should follow the key-custody plan
unless a later directive explicitly reorders it. The local workflow request
would reduce friction, especially for fast-forward handoff, response-file
writing, bounded polling, validation profiles, per-directive allow-files,
read-only source/authority refresh, claim-boundary scanning, history indexing,
and backup coverage for directives/requests/journals/ops history folders. It
does not outrank key custody for the selected restic-style target/tool path.

## Public-ingress/timing/traffic-shape boundary

No public ingress, public website, README, START_HERE, or docs/public mutation
is part of NA-0355. Current evidence does not prove hidden attachment size,
hidden timing metadata, hidden traffic shape, hidden all metadata, or padding
that hides all metadata. qsl-server PR #56 and qsl-attachments PR #37 remain
bounded harness/service-local evidence only.

## External-review sensitivity

External review remains incomplete. NA-0355 is internal target/tool planning
only. Any stronger public or review-facing statement requires off-host backup
implementation evidence, restore-drill evidence, service evidence, deployment
evidence, monitoring evidence, rollback evidence, and review evidence.

## Public claim boundary

NA-0355 introduces no claim of production readiness, public-internet readiness,
external-review completion, anonymity, metadata-free behavior, untraceable
behavior, hidden attachment size, hidden timing metadata, hidden traffic shape,
hidden all metadata, local continuity as complete disaster recovery, or
off-host encrypted backup completion.

## Future validation/marker/verification plan

Future NA-0356 key custody/recovery work should emit or prove:

- `NA0356_TARGET_TOOL_SELECTION_PLAN_OK`
- `NA0356_TARGET_CLASS_DECISION_OK`
- `NA0356_TOOL_CLASS_DECISION_OK`
- `NA0356_KEY_CUSTODY_DEPENDENCY_OK`
- `NA0356_RESTORE_DRILL_DEPENDENCY_OK`
- `NA0356_BACKUP_PLAN_UPDATE_REQUIRED_OK`
- `NA0356_LOCAL_OPS_BOUNDARY_OK`
- `NA0356_NO_OFF_HOST_OPERATION_OK`
- `NA0356_NO_KEY_GENERATION_OK`
- `NA0356_NO_SECRET_MATERIAL_OK`
- `NA0356_NO_DISASTER_RECOVERY_COMPLETE_CLAIM_OK`
- `NA0356_NO_PRODUCTION_READY_CLAIM_OK`
- `NA0356_NO_PUBLIC_INTERNET_READY_CLAIM_OK`

If future evidence blocks key custody/recovery, the blocker markers should name
the exact missing custody owner, recovery procedure, rotation procedure,
emergency access procedure, secret-scanning rule, or no-secret evidence rule.

## Workflow-support and history-index future work note

Read-only history paths improved confidence where present. Responses and
requests were available; directives and journals paths were absent at
inspection. A future local-ops workflow/history index remains useful but should
not be implemented by NA-0355.

## Selected successor

`NA-0356 -- Metadata Runtime Key Custody / Key Recovery Prerequisite Plan`

Rationale:

- the target/tool class decision is now sufficient for planning;
- repository initialization cannot be safely specified before key custody and
  recovery semantics are explicit;
- implementation authorization would be premature without no-secret custody,
  recovery, rotation, emergency access, and exposed/lost-key response rules.

## Rejected alternatives

- direct off-host backup setup in NA-0355;
- choosing object storage first without local/provider evidence;
- using gpg-wrapped archives as the primary backup tool because incremental,
  check, prune, and restore ergonomics are weaker;
- using rsync transport without a complete client-side encryption and restore
  verification layer;
- selecting implementation authorization before key custody;
- selecting restore drill before key custody;
- selecting workflow-support/history-index work before key custody;
- public-claim, website, external-review, or position-paper work before the
  off-host encrypted backup prerequisites are complete.

## Backup-plan impact statement

No NA-0355 backup-plan update is required. Future off-host target/tool
implementation requires backup-plan update and exact local-ops authorization
before any local system, backup script, timer, fstab, source-list, tool,
repository, key, restore, retention, purge, monitoring, deploy, rollback, or
public-claim mutation.

## Next recommendation

Run NA-0356 as a no-secret key custody/key recovery prerequisite plan for the
selected SSH/SFTP-compatible target class and restic-style encrypted repository
tool class. Do not install restic, initialize a repository, configure a remote,
generate a key, collect a passphrase, or run backup/restore operations until a
future directive explicitly authorizes those steps.
