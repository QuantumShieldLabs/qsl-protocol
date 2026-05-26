Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-26

# NA-0364 Metadata Runtime Restore Drill Isolated Restore Authorization

Goals: G1, G2, G3, G4, G5

## Executive summary

NA-0364 is a qsl-protocol governance and authorization lane only. It decides
whether future isolated restore-drill implementation can proceed after:

- NA-0359 proved a qsl-protocol no-secret restore-drill dry-run harness;
- NA-0361 proved a qsl-protocol no-secret key custody / key recovery harness;
- NA-0363 proved a qsl-protocol no-secret off-host target/tool harness.

Authorization result:

- `NO_SECRET_ISOLATED_RESTORE_HARNESS_AUTHORIZATION_READY`
- `NO_SECRET_RESTORE_TARGET_HARNESS_AUTHORIZATION_READY`
- `REAL_ISOLATED_RESTORE_IMPLEMENTATION_BLOCKED_BACKUP_PLAN`
- `REAL_ISOLATED_RESTORE_IMPLEMENTATION_BLOCKED_LOCAL_OPS`
- `REAL_ISOLATED_RESTORE_IMPLEMENTATION_BLOCKED_SECRET_HANDLING`
- `REAL_ISOLATED_RESTORE_IMPLEMENTATION_BLOCKED_OPERATOR_RUNBOOK`
- `REAL_ISOLATED_RESTORE_IMPLEMENTATION_BLOCKED_KEY_CUSTODY`
- `REAL_ISOLATED_RESTORE_IMPLEMENTATION_BLOCKED_TARGET_ISOLATION`
- `REAL_ISOLATED_RESTORE_IMPLEMENTATION_DEFERRED`
- `REAL_RESTORE_TARGET_IMPLEMENTATION_BLOCKED_BACKUP_PLAN`
- `REAL_RESTORE_TARGET_IMPLEMENTATION_BLOCKED_LOCAL_OPS`
- `REAL_RESTORE_TARGET_IMPLEMENTATION_BLOCKED_SECRET_HANDLING`
- `REAL_RESTORE_TARGET_IMPLEMENTATION_BLOCKED_CLEANUP`
- `REAL_RESTORE_TARGET_IMPLEMENTATION_BLOCKED_MONITORING`
- `REAL_RESTORE_TARGET_IMPLEMENTATION_DEFERRED`

Selected successor:

`NA-0365 -- Metadata Runtime Restore Drill Isolated Restore No-Secret Implementation Harness`

The future NA-0365 lane may implement only a qsl-protocol no-secret fixture and
harness. It may model simulated isolated restore target metadata, simulated
manifest/checksum restore relationships, simulated old-archive compatibility,
simulated cleanup/monitoring/runbook metadata, fail-closed negative cases,
temporary proof under `/srv/qbuild/tmp`, and explicit claim boundaries. It must
not create a real restore target, mount anything, copy backup payloads, run a
backup, run a restore, connect to any off-host destination, initialize a
repository, install a tool, generate or upload keys, collect passphrases,
inspect private keys, create recovery-envelope contents, handle secret
material, mutate local backup configuration, deploy, rollback, mutate service
repos, mutate runtime code, or claim real restore completion.

## Live NA-0364 scope

The live queue marks NA-0364 READY and requires an authorization result for
future isolated restore-drill work after the no-secret dry-run, no-secret key
custody/recovery, and no-secret target/tool harnesses.

Allowed current mutation scope is limited to qsl-protocol governance evidence,
this testplan lane, D-0710, TRACEABILITY, and the rolling operations journal.
Optional qsl-protocol fixture or script artifacts are deferred to NA-0365
because the live NA-0364 queue entry authorizes planning and successor
selection, not implementation.

Forbidden current scope includes qsl-server implementation, qsl-attachments
implementation, qshield runtime implementation, qsc/qsp/protocol/crypto
implementation, dependency or workflow mutation, website/public-doc mutation,
README or START_HERE mutation, backup script/timer/fstab mutation, off-host
setup, remote connection, repository initialization, tool installation, backup,
restore, restore target creation/mount/copy, deploy, rollback, real key
generation, key upload, passphrase collection, private-key inspection,
recovery-envelope content creation, and secret material handling.

## Inherited NA-0363 no-secret target/tool harness

NA-0363 added a qsl-protocol-only no-secret off-host target/tool fixture and
harness. The fixture models simulated SSH/SFTP target metadata, simulated
target identity metadata, a simulated restic-style encrypted snapshot
repository, simulated snapshot/check/prune/restore relationships, simulated
retention/purge metadata, simulated monitoring/alert metadata, and ten
fail-closed negative cases.

Inherited proof:

- fixture: `inputs/metadata_runtime/off_host_backup_target_tool_no_secret_fixture_v1.json`
- harness: `scripts/ci/metadata_runtime_off_host_backup_target_tool_no_secret_harness.sh`
- evidence: `docs/governance/evidence/NA-0363_metadata_runtime_off_host_encrypted_backup_target_tool_no_secret_harness.md`
- testplan: `tests/NA-0363_metadata_runtime_off_host_encrypted_backup_target_tool_no_secret_harness_testplan.md`
- operation count: `NA0363_OPERATION_EXECUTED_COUNT 0`
- secret scan marker: `OFF_HOST_TARGET_TOOL_SECRET_FINDING_COUNT 0`
- negative cases: `NA0363_NEGATIVE_CASES_PASSED 10`

NA-0363 remains no-secret harness evidence only. It is not a real off-host
target, not a real repository, not tool installation, not off-host backup
completion, and not restore execution.

## Inherited NA-0361 no-secret key custody/recovery harness

NA-0361 added a qsl-protocol-only no-secret key custody / key recovery fixture
and harness. The fixture validates simulated key IDs, custody records,
recovery-envelope metadata, rotation, old-archive compatibility, incident
response, emergency access, operator-runbook markers, fixture hashes, forbidden
operations, claim boundaries, and eight fail-closed negative cases.

Inherited proof:

- fixture: `inputs/metadata_runtime/key_custody_recovery_no_secret_fixture_v1.json`
- harness: `scripts/ci/metadata_runtime_key_custody_recovery_no_secret_harness.sh`
- evidence: `docs/governance/evidence/NA-0361_metadata_runtime_key_custody_key_recovery_no_secret_harness.md`
- testplan: `tests/NA-0361_metadata_runtime_key_custody_key_recovery_no_secret_harness_testplan.md`
- operation count: `NA0361_OPERATION_EXECUTED_COUNT 0`
- secret scan marker: `KEY_CUSTODY_RECOVERY_SECRET_FINDING_COUNT 0`
- negative cases: `NA0361_NEGATIVE_CASES_PASSED 8`

NA-0361 remains no-secret harness evidence only. It is not real key custody,
not real key recovery, not a real recovery envelope, and not a secret-handling
implementation.

## Inherited NA-0359 restore dry-run harness

NA-0359 added a qsl-protocol-only no-secret restore-drill dry-run fixture and
harness. The harness validates deterministic manifest/checksum relationships,
writes redacted proof under `/srv/qbuild/tmp`, proves seven fail-closed
negative cases, and emits explicit no-operation markers.

Inherited proof:

- fixture: `inputs/metadata_runtime/restore_drill_dry_run_fixture_v1.json`
- harness: `scripts/ci/metadata_runtime_restore_drill_dry_run_harness.sh`
- evidence: `docs/governance/evidence/NA-0359_metadata_runtime_restore_drill_dry_run_harness.md`
- testplan: `tests/NA-0359_metadata_runtime_restore_drill_dry_run_harness_testplan.md`
- operation count: `NA0359_OPERATION_EXECUTED_COUNT 0`
- restore target count: `NA0359_RESTORE_TARGET_CREATED_COUNT 0`
- key operation count: `NA0359_KEY_OPERATION_COUNT 0`
- off-host operation count: `NA0359_OFF_HOST_OPERATION_COUNT 0`
- secret scan marker: `RESTORE_DRY_RUN_SECRET_FINDING_COUNT 0`

NA-0359 remains dry-run evidence only. It is not real restore execution, not an
isolated restore target, and not complete disaster recovery.

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
- `ISOLATED_RESTORE_AUTHORIZATION_READY`
- `REAL_ISOLATED_RESTORE_BLOCKED`
- `REAL_KEY_CUSTODY_NOT_READY`
- `REAL_KEY_RECOVERY_NOT_READY`
- `OFF_HOST_BACKUP_NOT_READY`

No read-only evidence proves a real off-host encrypted repository, real
repository initialization, real repository password/key custody, real recovery
envelope, real key rotation implementation, emergency access implementation,
off-host restore drill, remote retention/purge, remote monitoring, or
production operator runbook completion.

## Isolated restore implementation authorization decision

| Category | Result |
| --- | --- |
| `NO_SECRET_ISOLATED_RESTORE_HARNESS_AUTHORIZATION_READY` | selected for future qsl-protocol-only fixture/harness work |
| `REAL_ISOLATED_RESTORE_IMPLEMENTATION_BLOCKED_BACKUP_PLAN` | applies to any real target, durable restored artifact, source-list, script, timer, fstab, service, monitoring, or public-claim mutation |
| `REAL_ISOLATED_RESTORE_IMPLEMENTATION_BLOCKED_LOCAL_OPS` | applies to real target lifecycle, cleanup, operator workflow, and history index coverage |
| `REAL_ISOLATED_RESTORE_IMPLEMENTATION_BLOCKED_SECRET_HANDLING` | applies to any real repository password, key, private key, passphrase, or recovery-envelope content |
| `REAL_ISOLATED_RESTORE_IMPLEMENTATION_BLOCKED_OPERATOR_RUNBOOK` | applies until real restore target creation, restore validation, cleanup, monitoring, and stop conditions are operator-approved |
| `REAL_ISOLATED_RESTORE_IMPLEMENTATION_BLOCKED_KEY_CUSTODY` | applies because real key custody and recovery are not implemented |
| `REAL_ISOLATED_RESTORE_IMPLEMENTATION_BLOCKED_TARGET_ISOLATION` | applies because no real isolated target path, mount policy, cleanup policy, quota, or rollback boundary is approved |
| `REAL_ISOLATED_RESTORE_IMPLEMENTATION_DEFERRED` | applies to all real restore operations |

Future no-secret qsl-protocol isolated restore harness work can be authorized
now because NA-0359, NA-0361, and NA-0363 have established safe simulated
evidence boundaries. Real isolated restore cannot be authorized now.

## Restore target implementation authorization decision

| Category | Result |
| --- | --- |
| `NO_SECRET_RESTORE_TARGET_HARNESS_AUTHORIZATION_READY` | selected for simulated restore-target metadata only |
| `REAL_RESTORE_TARGET_IMPLEMENTATION_BLOCKED_BACKUP_PLAN` | applies to any durable real target or restored artifact |
| `REAL_RESTORE_TARGET_IMPLEMENTATION_BLOCKED_LOCAL_OPS` | applies to target creation, cleanup, disk pressure, history, and operational ownership |
| `REAL_RESTORE_TARGET_IMPLEMENTATION_BLOCKED_SECRET_HANDLING` | applies to any secret-dependent target access or restore repository access |
| `REAL_RESTORE_TARGET_IMPLEMENTATION_BLOCKED_CLEANUP` | applies until deletion, verification, quarantine, and failure cleanup are operator-approved |
| `REAL_RESTORE_TARGET_IMPLEMENTATION_BLOCKED_MONITORING` | applies until monitoring/log/alert behavior is approved and backup-covered |
| `REAL_RESTORE_TARGET_IMPLEMENTATION_DEFERRED` | applies to real target creation, mount, and copy |

Future qsl-protocol no-secret restore-target harness work can model target
metadata, quota, cleanup markers, monitoring markers, and fail-closed negative
cases. It must not create, mount, or copy into a real restore target.

## Future no-secret isolated restore implementation bundle

The exact future NA-0365 bundle should be qsl-protocol-only and no-secret.

Allowed future repo:

- `QuantumShieldLabs/qsl-protocol`

Allowed future files:

- `inputs/metadata_runtime/isolated_restore_no_secret_fixture_v1.json`
- `scripts/ci/metadata_runtime_isolated_restore_no_secret_harness.sh`
- `docs/governance/evidence/NA-0365_metadata_runtime_restore_drill_isolated_restore_no_secret_harness.md`
- `tests/NA-0365_metadata_runtime_restore_drill_isolated_restore_no_secret_harness_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Allowed future behavior:

- simulated isolated restore target metadata;
- simulated restore manifest/checksum relationships;
- simulated old-archive compatibility matrix;
- simulated cleanup, monitoring, alert, and operator-runbook matrix;
- deterministic no-secret fixture data;
- in-memory fail-closed negative cases;
- temporary proof under `/srv/qbuild/tmp/NA-0365_*`;
- marker output proving no real target creation, mount, copy, backup, restore,
  off-host operation, key generation, passphrase collection, or secret
  material handling.

Forbidden future behavior:

- real restore target creation;
- mount;
- copy of backup payloads;
- backup execution;
- restore execution;
- deploy or rollback;
- remote/off-host connection;
- repository initialization;
- tool installation;
- key generation or upload;
- passphrase collection;
- private-key inspection;
- recovery-envelope content creation;
- secret material handling;
- qsl-server, qsl-attachments, qshield runtime, qsc/qsp/protocol/crypto,
  dependency, workflow, website, README, START_HERE, docs/public, backup
  script/timer/fstab/source-list, system service, branch-protection, or
  public-safety mutation.

Required future commands:

- `bash -n scripts/ci/metadata_runtime_isolated_restore_no_secret_harness.sh`
- `python3 -m json.tool inputs/metadata_runtime/isolated_restore_no_secret_fixture_v1.json >/dev/null`
- `bash scripts/ci/metadata_runtime_isolated_restore_no_secret_harness.sh inputs/metadata_runtime/isolated_restore_no_secret_fixture_v1.json`
- prior NA-0359, NA-0361, and NA-0363 harnesses;
- qsl-protocol queue, decision, scope, link, leak, classifier, goal-lint, cargo
  audit, rustls-webpki, formatting, formal/model, qsc, and feasible qshield
  checks required by the future directive.

Backup-plan result:

- no backup-plan update is required for a pure qsl-protocol tracked
  fixture/harness plus temporary `/srv/qbuild/tmp` proof;
- backup-plan and local-ops authorization are required before any real target,
  durable restored artifact, key material, recovery envelope, off-host target,
  source-list change, script/timer/fstab/service change, monitoring artifact,
  backup, restore, deploy, rollback, or public-claim mutation.

## Real isolated restore boundary and blocker analysis

| Candidate | NA-0364 result |
| --- | --- |
| real `/srv/qbuild/tmp` restore target | blocked; even under tmp it would create real restored state and needs target lifecycle, cleanup, backup-plan, and local-ops approval |
| real disposable disk restore target | blocked; it requires mount/storage ownership, cleanup, monitoring, and operator runbook approval |
| real staging machine restore target | blocked; it requires off-host target/tool/key/recovery evidence, target isolation, and local-ops authorization |
| real non-live qbuild restore target | blocked; it still creates real restored artifacts and requires backup-plan and cleanup approval |
| real production root restore | rejected for this phase; it is not an isolated no-secret harness and would be a live restore operation |
| real mount | blocked; no mount operation is authorized |
| real copy | blocked; no backup payload or restored payload copy is authorized |
| real backup payload retrieval | blocked; no backup or off-host repository access is authorized |
| real repository access | blocked; no repository exists and no secret handling is approved |
| real key/passphrase use | blocked; real key custody and recovery are not implemented |
| real cleanup | blocked; can be modeled only until target lifecycle is approved |
| real monitoring/alerting | blocked; can be modeled only until local-ops authorization and backup coverage exist |
| real retention/purge behavior | blocked; can be modeled only |

Prerequisites before any real isolated restore operation:

- exact backup-plan update;
- local-ops workflow and history coverage decision;
- approved restore target path/device/isolation policy;
- cleanup/quarantine/failure runbook;
- monitoring/logging/alert policy;
- real key custody and recovery implementation;
- real off-host target/tool/repository implementation;
- secret-handling procedure;
- operator-approved command set and stop conditions;
- no-overclaim public boundary;
- required CI and governance approval.

## No-secret isolated restore fixture and simulated restore-target authorization analysis

| Option | Result | Rationale |
| --- | --- | --- |
| qsl-protocol simulated isolated restore target fixture | recommended | Safe if metadata-only and no target directory is created; gives next-lane structure for target isolation checks |
| qsl-protocol simulated manifest/checksum restore fixture | recommended | Extends NA-0359 manifest proof into isolated restore relationships without reading backup payloads |
| qsl-protocol simulated old-archive compatibility matrix | recommended | Uses NA-0361 simulated recovery and NA-0363 simulated repository metadata without real secrets |
| qsl-protocol simulated cleanup/monitoring/runbook matrix | recommended | Keeps cleanup/alert/runbook prerequisites visible before real target work |
| qsl-server/qsl-attachments service-local fixture harness | deferred | These repos are read-only in this lane; future service-local work needs a separate directive |
| no no-secret implementation | rejected | Safe no-secret implementation boundaries are exact enough to proceed to NA-0365 |

The no-secret options are CI-feasible in qsl-protocol and should produce only
tracked fixture/harness files plus temporary proof under `/srv/qbuild/tmp`.
They do not prove real restore, real off-host backup, real key custody, real
key recovery, or production/public-internet readiness.

## Restore target creation / mount / copy blocker analysis

Real restore target creation cannot proceed now. Mount cannot proceed now. Copy
cannot proceed now. Live root restore is outside this phase and must not be
allowed by NA-0364 or NA-0365.

No-secret modeling is sufficient for the next lane because it can prove the
exact harness contract, markers, negative cases, and claim boundaries before
any real target work. Real restore remains blocked by key custody, backup plan,
local-ops, off-host target/tool, monitoring, retention, cleanup, and operator
runbook prerequisites.

## Key custody/recovery dependency for isolated restore

The no-secret isolated restore harness can proceed after NA-0361 and NA-0363
because it uses simulated key/recovery metadata only. Simulated key/recovery
evidence is sufficient for a no-secret fixture/harness lane.

Real isolated restore requires real key custody, real key recovery, repository
secret handling, old-archive compatibility procedure, emergency access
procedure, and recovery-envelope handling. Those are not implemented, so real
restore remains blocked.

## Off-host target/tool dependency for isolated restore

The no-secret isolated restore harness can proceed after NA-0363 because it
uses simulated target/tool/repository metadata only. No-secret target/tool
evidence is sufficient for a no-secret fixture/harness lane.

Real isolated restore requires real off-host target access, repository
initialization, tool installation or approved tool use, check/prune/restore
proof, retention policy, target identity policy, monitoring, and operator
runbook evidence. Those are not implemented, so real restore remains blocked.

## Retention/purge/monitoring/runbook dependency analysis

Real restore work requires retention, purge, cleanup, monitoring, alerting, and
operator runbook evidence before any target creation or reliance. NA-0365
should model these as no-secret markers and fail-closed fixture fields only.

Real monitoring setup remains blocked because it would create operational
artifacts and possibly service or backup-plan changes. The operator runbook
must precede any real restore target creation.

## Backup-plan impact and local-ops dependency decision

No NA-0364 backup-plan update is required because changed paths stay under the
qsl-protocol worktree already covered by `/srv/qbuild/work`.

Future NA-0365 no-secret fixture/harness work does not require a backup-plan
update if durable changes remain tracked qsl-protocol files and temporary
proof remains under `/srv/qbuild/tmp`.

Backup-plan and local-ops authorization are required before any:

- real restore target;
- durable restored artifact;
- key material;
- real recovery envelope;
- off-host target or repository;
- source-list change;
- backup script, timer, fstab, or system service change;
- monitoring artifact;
- backup, restore, deploy, or rollback operation;
- public-claim mutation.

The workflow-support/history-index request would reduce friction but should
follow the no-secret implementation harness unless future evidence selects it
as a blocker. At inspection, Codex responses and requests were present;
directives and journals were absent; backup status covers responses and the
backup plan, but not a full directives/journals/ops history tree. D132 cleanup
remains not authorized.

## Public-ingress/timing/traffic-shape boundary

NA-0364 changes no public ingress, timing, traffic-shape, padding, cover
traffic, website, service, public-doc, or public-claim behavior. The lane does
not claim that attachment size, timing metadata, traffic shape, all metadata,
or padding behavior is hidden.

## External-review sensitivity

External review remains incomplete. NA-0364 is an authorization plan, not real
restore execution, not public-internet proof, not production proof, and not
external-review completion. Stronger claims require real key custody evidence,
real key recovery evidence, off-host backup evidence, real restore-drill
evidence, service evidence, deployment evidence, monitoring/log evidence,
rollback evidence, and review evidence.

## Public claim boundary

No NA-0364 wording may state or imply production readiness, public-internet
readiness, external-review completion, anonymity, metadata-free behavior,
untraceable behavior, hidden attachment size, hidden timing metadata, hidden
traffic shape, hidden all metadata, local continuity as complete disaster
recovery, off-host encrypted backup completion, real restore-drill completion,
real key custody implementation, or real key recovery implementation.

## Future validation/marker/verification plan

Future NA-0365 markers:

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

Verification requirements:

- JSON parse for the future fixture;
- shell syntax check for the future harness;
- future harness execution and fail-closed negative cases;
- prior NA-0359, NA-0361, and NA-0363 harnesses;
- queue and decision checks;
- scope guard for exact allowed paths;
- link check and added-line leak scan;
- overclaim scan;
- classifier proof;
- dependency/advisory health;
- required public-safety proof before and after merge.

## Workflow-support and history-index future work note

The local workflow-support request remains useful. The highest-friction items
observed in NA-0364 were stale local checkout handoff, response-file writing,
bounded PR/public-safety polling, machine-readable directive scope, validation
profiles, per-directive allow-files, read-only source/authority helper output,
claim-boundary scanning, directive/response/journal index, and backup coverage
for directives/requests/journals/ops history folders.

These items should not be implemented in NA-0364. They can become a later
local-ops successor if no-secret restore implementation is blocked or after
the no-secret implementation harness lands.

## Selected successor

Selected successor:

`NA-0365 -- Metadata Runtime Restore Drill Isolated Restore No-Secret Implementation Harness`

Rationale:

- NA-0359 proves no-secret manifest/checksum dry-run restore evidence.
- NA-0361 proves no-secret simulated key custody/recovery evidence.
- NA-0363 proves no-secret simulated target/tool/repository evidence.
- NA-0364 can name exact qsl-protocol-only allowed files, commands, markers,
  artifacts, forbidden operations, backup-plan boundaries, and stop conditions.
- Real restore remains blocked, but no-secret implementation is safe and useful
  as the next evidence step.

## Rejected alternatives

- `Metadata Runtime Restore Drill Isolated Restore Blocker Resolution`:
  rejected because no blocker prevents the no-secret implementation harness.
- `QSL Local Ops Codex Workflow Support and History Index Plan`: deferred
  because it is helpful but not required before the no-secret harness.
- `Metadata Runtime External Review Readiness Gap Audit`: deferred because
  real off-host backup, real key custody/recovery, and real restore evidence
  remain absent.
- `Metadata Runtime Website / Public Claim Boundary Audit`: deferred because
  this lane already preserves public-claim boundaries and no website/public-doc
  mutation is needed now.
- `Public Technical Position Paper Evidence-Bounded Draft Plan`: deferred until
  off-host backup, real restore, review, service, monitoring, and rollback
  evidence are stronger.
- direct real restore: rejected.
- direct restore target creation/mount/copy: rejected.
- direct backup or restore: rejected.
- direct key/passphrase handling: rejected.

## Backup-plan impact statement

`NO_BACKUP_PLAN_UPDATE_REQUIRED_NOW`.

NA-0364 changes only qsl-protocol governance/testplan/journal paths in the
existing `/srv/qbuild/work` scope. Future real restore, off-host target/tool,
key custody/recovery, recovery envelope, durable restored artifact,
monitoring, source-list, script/timer/fstab/system-service, backup, restore,
deploy, rollback, or public-claim work requires backup-plan and local-ops
authorization first.

## Next recommendation

Merge NA-0364 authorization if local validation and required CI are green, then
close out NA-0364 and restore exactly:

`NA-0365 -- Metadata Runtime Restore Drill Isolated Restore No-Secret Implementation Harness`
