Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-24

# NA-0354 Metadata Runtime Off-Host Encrypted Backup Implementation Authorization

Goals: G1, G2, G3, G4, G5

## Executive summary

NA-0354 converts the NA-0353 prerequisite plan into an implementation
authorization decision. The decision is prerequisite-gated:

`OFF_HOST_IMPLEMENTATION_DEFERRED`

The next lane must select a concrete backup target class and encryption tool
before any implementation harness, backup-plan mutation, local-ops script
change, off-host setup, key generation, passphrase collection, restore drill,
or monitoring integration is authorized.

Selected successor:

`NA-0355 -- Metadata Runtime Off-Host Encrypted Backup Target / Tool Selection Plan`

NA-0354 is qsl-protocol governance only. It changed no qsl-server,
qsl-attachments, qshield runtime, qsc/qsp, protocol, crypto, dependency,
workflow, website, README, START_HERE, docs/public, local backup script, timer,
fstab, system service, off-host destination, key file, passphrase, restore
state, deploy state, rollback state, or public service behavior.

## Live NA-0354 scope

The live queue marks NA-0354 READY and requires an authorization lane after
NA-0353:

- define exact implementation authorization, executable evidence, stop
  conditions, backup-plan updates, local-ops dependencies, encryption and
  key-handling boundaries, restore-drill requirements, retention/purge rules,
  monitoring/logging expectations, and operator runbook requirements before any
  off-host encrypted backup setup or live operation;
- preserve qsl-server and qsl-attachments production boundaries;
- preserve qshield embedded relay/demo proof as reference/oracle evidence only;
- preserve all public-claim limits around production readiness, public-internet
  readiness, external review, anonymity, metadata-free behavior,
  untraceability, attachment size, timing metadata, traffic shape, and disaster
  recovery.

## Inherited NA-0353 prerequisite plan

NA-0353 found local `/backup/qsl` continuity backup mounted, scheduled, and
current, but classified it as same-host continuity only. It did not prove
off-host encrypted backup.

Inherited missing prerequisites:

- backup target class;
- encryption tool;
- key custody, key recovery, key rotation, and emergency access;
- passphrase and secret-handling boundaries;
- isolated off-host restore drill;
- off-host retention, purge, failed-upload cleanup, and failed-restore cleanup;
- monitoring, alerting, redacted logs, and operator runbook;
- backup-plan update and local-ops authorization for any future mutation.

NA-0353 also preserved the boundaries that qsl-server PR #56 is bounded
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
| remote main SHA | `d40e6003fdf0` |
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

### qsl-attachments

| Field | Result |
| --- | --- |
| repository | `QuantumShieldLabs/qsl-attachments` |
| default branch | `main` |
| local path inspected | `/srv/qbuild/work/NA-0237D/qsl-attachments` |
| local HEAD | `96b9352bd63e` |
| remote main SHA | `96b9352bd63e` |
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

No source, authority, open-PR, or CI blocker was found in either service repo.
No service repo mutation was performed.

## Local backup/off-host evidence refresh

Read-only local evidence:

- `/backup/qsl` is mounted from `/dev/sda1` as ext4.
- `/backup/qsl` reported about 20 GiB used of 916 GiB, about 3 percent.
- `/srv/qbuild` reported about 57 GiB used of 468 GiB, about 13 percent.
- `qsl-backup preflight` reported the target mounted and daily sources present.
- `qsl-backup list` listed checkpoint snapshots from 2026-05-17 and daily
  snapshots through 2026-05-24.
- manifests and logs exist for the listed local snapshots.
- `qsl-backup-daily.timer` is enabled and waiting.
- D132 preservation bundle remains present under `/srv/qbuild/tmp`.

Current backup status/plan evidence:

- local daily sources include `/srv/qbuild/work`, `/srv/qbuild/tmp`, qbuild
  mirrors, qbuild evidence/logs/archive, Codex logs, Codex responses, and
  `QSL_BACKUP_PLAN.md`;
- Codex directives and journals paths were absent at inspection;
- Codex responses and requests paths were present;
- Codex ops backup path was present;
- the local plan explicitly says the platter backup is local continuity and
  should not be the only disaster recovery copy.

Classification:

- `LOCAL_CONTINUITY_PROVEN`
- `OFF_HOST_BACKUP_NOT_READY`
- `OFF_HOST_BACKUP_BLOCKED` for live operation

No read-only evidence proves an off-host encrypted backup destination,
configured remote, encryption-key custody, key recovery, off-host restore drill,
remote retention/purge, remote monitoring/alerting, or operator runbook.

## Implementation authorization decision

Decision category:

`OFF_HOST_IMPLEMENTATION_DEFERRED`

The deferral is exact and prerequisite-based:

- `OFF_HOST_IMPLEMENTATION_BLOCKED_TARGET`
- `OFF_HOST_IMPLEMENTATION_BLOCKED_ENCRYPTION_TOOL`
- `OFF_HOST_IMPLEMENTATION_BLOCKED_KEY_CUSTODY`
- `OFF_HOST_IMPLEMENTATION_BLOCKED_KEY_RECOVERY`
- `OFF_HOST_IMPLEMENTATION_BLOCKED_SECRET_HANDLING`
- `OFF_HOST_IMPLEMENTATION_BLOCKED_RESTORE_DRILL`
- `OFF_HOST_IMPLEMENTATION_BLOCKED_MONITORING`
- `OFF_HOST_IMPLEMENTATION_BLOCKED_BACKUP_PLAN`
- `OFF_HOST_IMPLEMENTATION_BLOCKED_LOCAL_OPS`

Rationale:

- a future no-secret dry-run harness is feasible, but only after target/tool
  semantics are selected;
- off-host network operation must remain forbidden until a later local-ops lane
  authorizes exact destination, credentials, key custody, backup-plan update,
  and restore-drill procedure;
- backup-plan update must precede any target, key, source-list, script, timer,
  fstab, system service, restore drill, retention/purge, monitoring, or
  off-host operation mutation;
- local workflow/history indexing would reduce friction, but it is not the
  next prerequisite because the target/tool choice is the first missing
  implementation boundary.

## Future implementation authorization bundle

Future work must be staged:

1. Target/tool selection and option rejection.
2. Key custody/key recovery prerequisite plan with no secret material.
3. No-secret dry-run fixture harness in qsl-protocol only.
4. Local-ops backup-plan and candidate-script/source-list update.
5. Isolated restore drill.
6. Retention/purge and monitoring/alerting integration.
7. Production-use gate only after the above evidence exists.

Future allowed repositories by stage:

| Stage | Repository/path authority |
| --- | --- |
| selection/prerequisite plans | qsl-protocol governance/testplan only |
| no-secret dry-run harness | qsl-protocol governance, tests, and fixture paths only |
| script/source-list/config candidate | local-ops only, with exact directive and backup-plan update |
| service data roots | future qsl-server/qsl-attachments/local-ops directive only |
| live operation | future local-ops directive only after key/restore evidence |

Future forbidden scope until exact authorization:

- qsl-server or qsl-attachments runtime mutation;
- qshield/qsc/qsp/protocol/crypto/key-schedule mutation;
- dependency or workflow mutation;
- website, README, START_HERE, or docs/public mutation;
- local backup script, timer, fstab, source-list, or system-service mutation;
- off-host setup, network backup operation, key generation, key upload,
  passphrase collection, restore, deploy, rollback, or purge;
- secret-dependent tests or evidence.

Future stop conditions:

- target or tool remains ambiguous;
- key custody or recovery cannot be documented without handling secret material;
- restore drill cannot be isolated from live data;
- backup-plan update is required but not authorized;
- logs, manifests, PR body, response file, or journal would expose secret
  material or sensitive endpoint details;
- any public/privacy/readiness claim would exceed current evidence.

## Target/tool/key option matrix

| Option class | Recommendation | Reason |
| --- | --- | --- |
| external disk | deferred | improves separation if physically removed, but still operator-heavy and easy to leave same-site |
| NAS | deferred | can help local hardware loss but may still share site/ransomware risk unless isolated |
| object storage | candidate | strong off-site durability and CI-friendly dry-run modeling, but needs credentials, cost/quota, and client-side encryption |
| SSH/SFTP host | candidate | simple operational model and restic/borg fit, but needs host hardening and credential handling |
| encrypted cloud bucket | candidate | useful if encryption is client-side and provider keys are not the only control |
| removable offline media | deferred | strong ransomware separation, weak automation and monitoring |
| age | candidate for file/key wrapping | simple recipient model, but not a complete backup system by itself |
| gpg | deferred | widely available, but operator/key management complexity is higher |
| restic | candidate | integrated encryption, snapshots, prune, and restore verification; needs repository password/key custody |
| borg | candidate | integrated encrypted repositories and prune; SSH-oriented operations need host/access choices |
| rclone crypt | deferred | useful transport layer, but restore semantics and key handling need additional controls |
| existing qsl-backup extension | deferred | preserves current local model, but off-host encryption/retention/restore features would need careful local-ops work |
| none | rejected | off-host backup without client-side encryption is not acceptable |
| operator-held passphrase | candidate with recovery plan | simple, but loss/exposure risk is high |
| age recipient file | candidate with offline private key custody | clear no-secret public recipient boundary, but recovery still needs a protected private key |
| hardware token | deferred | stronger custody, higher operational complexity |
| split secret | deferred | useful for recovery, but too complex before target/tool selection |
| offline recovery envelope | candidate | needed for recoverability if any encrypted remote copy becomes authoritative |
| service-managed key only | rejected unless secondary | weakens client-side encryption and custody clarity |

## Key-handling/secrets/passphrase/recovery boundary plan

NA-0354 created no key, uploaded no key, collected no passphrase, printed no
secret material, and ran no secret-dependent test.

Future key prerequisites:

- choose the encryption tool first;
- define who owns the key/passphrase and where it is held;
- define offline recovery material and verification without publishing it;
- define rotation, revocation, loss, and exposure response;
- define emergency access that does not normalize shared plaintext secrets;
- define no-secret output rules for logs, manifests, CI, PR bodies, response
  files, and rolling journal entries;
- run secret/leak scans over added evidence and generated artifacts;
- fail closed if any key, passphrase, bearer value, remote credential, token,
  or secret-bearing endpoint would be printed.

## Restore drill/retention/purge/monitoring/runbook authorization plan

Future restore categories:

- dry-run restore model with fixture data and no secrets;
- isolated real restore to a non-live target after key custody is authorized;
- manifest/checksum verification against source and restored copy;
- explicit proof that no live files are overwritten.

Future retention/purge requirements:

- remote retention count/time policy;
- pruning command and dry-run evidence;
- failed-upload cleanup policy;
- failed-restore cleanup policy;
- quota/cost thresholds;
- evidence that purge does not delete the only usable recovery point.

Future monitoring/runbook requirements:

- missed-run detection;
- failed-backup alert;
- failed-prune alert;
- failed-restore alert;
- redacted log policy;
- operator preflight, emergency stop, and escalation steps;
- evidence bundle that records status without secrets.

## Backup-plan impact and local-ops dependency decision

NA-0354 itself requires no backup-plan update because it changes only
qsl-protocol governance/testplan/journal paths already under `/srv/qbuild/work`.

Future off-host implementation requires backup-plan update and exact local-ops
authorization before any target, key, source-list, script, timer, fstab,
system-service, restore-drill, retention/purge, monitoring, backup, restore,
deploy, rollback, or public-claim mutation.

Local workflow-support and directive/response/history indexing should follow
target/tool selection unless the selected target/tool work proves history
coverage is a direct prerequisite. The inspected history state was:

- `/home/victor/work/qsl/codex/directives`: absent;
- `/home/victor/work/qsl/codex/responses`: present;
- `/home/victor/work/qsl/codex/journals`: absent;
- `/home/victor/work/qsl/codex/requests`: present;
- `/home/victor/work/qsl/codex/ops/backup`: present.

## Public-ingress/timing/traffic-shape boundary

No public-ingress, website, public-doc, service deployment, or runtime behavior
changed in NA-0354. Current evidence does not prove hidden attachment size,
hidden timing metadata, hidden traffic shape, hidden all metadata, or padding
that hides all metadata.

## External-review sensitivity

External review remains incomplete. NA-0354 is internal authorization planning,
not external-review evidence. Any stronger external claim requires later
off-host backup evidence, restore-drill evidence, service evidence, deployment
evidence, monitoring evidence, rollback evidence, and review evidence.

## Public claim boundary

The following claims remain forbidden:

- production readiness;
- public-internet readiness;
- external review completion;
- anonymity;
- metadata-free behavior;
- untraceable behavior;
- hidden attachment size;
- hidden timing metadata;
- hidden traffic shape;
- padding hiding all metadata;
- local backup as complete disaster recovery;
- off-host encrypted backup as complete.

## Future validation/marker/verification plan

Future NA-0355 target/tool selection should emit or record:

- `NA0355_BACKUP_TARGET_SELECTION_OK`
- `NA0355_ENCRYPTION_TOOL_SELECTION_OK`
- `NA0355_KEY_CUSTODY_PREREQUISITE_IDENTIFIED_OK`
- `NA0355_KEY_RECOVERY_PREREQUISITE_IDENTIFIED_OK`
- `NA0355_SECRET_HANDLING_PREREQUISITE_IDENTIFIED_OK`
- `NA0355_RESTORE_DRILL_PREREQUISITE_IDENTIFIED_OK`
- `NA0355_BACKUP_PLAN_UPDATE_REQUIRED_OK`
- `NA0355_NO_OFF_HOST_OPERATION_OK`
- `NA0355_NO_SECRET_MATERIAL_OK`
- `NA0355_NO_DISASTER_RECOVERY_COMPLETE_CLAIM_OK`
- `NA0355_NO_PRODUCTION_READY_CLAIM_OK`
- `NA0355_NO_PUBLIC_INTERNET_READY_CLAIM_OK`

Later implementation harnesses may add the remaining markers only after target
and tool selection are complete:

- `NA0355_OFF_HOST_BACKUP_AUTHORIZATION_OK`
- `NA0355_DRY_RUN_HARNESS_OK`
- `NA0355_RESTORE_DRILL_PLAN_OK`
- `NA0355_RETENTION_PURGE_PLAN_OK`
- `NA0355_MONITORING_ALERT_PLAN_OK`

## Workflow-support and history-index future work note

The local workflow-support request remains valid future local-ops work. The
following would materially reduce friction:

- qstart/qresume fast-forward to expected origin/main before handoff;
- response-file writer;
- bounded PR/public-safety polling helper;
- machine-readable directive manifest;
- validation profiles;
- per-directive allow-file;
- read-only source/authority helper;
- claim-boundary scanner;
- directive/response/journal index;
- backup coverage for directives/requests/journals/ops history folders.

NA-0354 does not implement those items.

## Selected successor

`NA-0355 -- Metadata Runtime Off-Host Encrypted Backup Target / Tool Selection Plan`

Rationale: the first missing implementation prerequisite is target/tool
selection. Key custody, recovery, restore drill, monitoring, and backup-plan
updates depend on the selected target and encryption tool.

## Rejected alternatives

- `Metadata Runtime Off-Host Encrypted Backup Implementation Harness`: rejected
  because target/tool/key/restore prerequisites are not selected.
- `Metadata Runtime Off-Host Encrypted Backup Blocker Resolution`: rejected
  because the blocker is specific target/tool selection, not an ambiguous
  generalized blocker.
- `Metadata Runtime Key Custody / Key Recovery Prerequisite Plan`: deferred
  until the tool/key model is selected.
- `Metadata Runtime Restore Drill Prerequisite Plan`: deferred until target and
  encryption semantics are selected.
- `QSL Local Ops Codex Workflow Support and History Index Plan`: useful but not
  the first off-host implementation prerequisite.
- external review, website/public-claim audit, or position paper: deferred
  until off-host backup and restore evidence exists.

## Backup-plan impact statement

No current backup-plan update is required for NA-0354. Future off-host work
requires backup-plan update and local-ops authorization before mutation or
operation.

## Next recommendation

Merge this authorization plan, close out NA-0354 only after required checks and
public-safety are green, and restore NA-0355 as the exact target/tool selection
lane without implementing NA-0355.
