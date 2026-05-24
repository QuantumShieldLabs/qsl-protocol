Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-24

# NA-0353 Metadata Runtime Off-Host Encrypted Backup Prerequisite Plan

Goals: G1, G2, G3, G4, G5

## Executive summary

NA-0353 records the off-host encrypted backup prerequisite plan after the
NA-0352 production backup/deploy/rollback boundary harness. It is a
qsl-protocol governance and planning lane only.

Current result:

- local `/backup/qsl` continuity backup is mounted, scheduled, and has current
  local snapshots;
- local continuity remains same-host continuity only, not complete disaster
  recovery;
- no off-host encrypted backup implementation, remote destination, key custody
  procedure, key recovery procedure, restore drill, off-host retention/purge
  procedure, monitoring/alerting procedure, or operator runbook is proven;
- qsl-server and qsl-attachments source, authority, and CI are fresh enough for
  the next authorization-planning lane;
- the selected successor is
  `NA-0354 -- Metadata Runtime Off-Host Encrypted Backup Implementation Authorization Plan`.

NA-0353 did not run a backup, restore, deploy, rollback, purge, service
mutation, off-host setup, key generation, passphrase collection, or
secret-dependent command.

## Live NA-0353 scope

The live queue restored NA-0353 as:

`Metadata Runtime Off-Host Encrypted Backup Prerequisite Plan`

The scope requires planning exact off-host encrypted backup prerequisites before
future production service roots, deploy configs, rollback artifacts, restore
fixtures, monitoring artifacts, or local ops history can be treated as durable
disaster-recovery evidence.

Allowed behavior in this lane:

- inspect local backup state read-only;
- inspect qsl-server and qsl-attachments source, authority, and CI read-only;
- inspect qsl-protocol/qbuild/Codex backup evidence read-only;
- identify target classes, encryption/key-handling boundaries, restore-drill
  prerequisites, retention/purge boundaries, monitoring/logging expectations,
  operator runbook needs, and public-claim boundaries;
- select the exact NA-0354 successor.

Forbidden behavior in this lane:

- qsl-server or qsl-attachments mutation;
- qshield runtime, qsc, qsp, protocol, crypto, key schedule, dependency,
  workflow, website, README, START_HERE, docs/public, branch-protection,
  public-safety, backup script, timer, fstab, system service, source-list, or
  local backup configuration mutation;
- live backup, restore, deploy, rollback, purge, public-ingress cutover,
  off-host setup, remote backup operation, key generation, passphrase
  collection, or secret material handling;
- production readiness, public-internet readiness, external-review-complete,
  anonymity, metadata-free, untraceable, hidden attachment size, hidden timing
  metadata, hidden traffic shape, hidden all metadata, or quantum-proof claims.

## Inherited NA-0352 boundary harness

NA-0352 added:

- `scripts/ci/metadata_runtime_production_backup_deploy_rollback_harness.sh`
- `inputs/metadata_runtime/production_backup_deploy_rollback_fixture_v1.json`

The harness emitted the required NA0352 markers, wrote only a temporary artifact
under `/srv/qbuild/tmp`, reported `SECRET_FINDING_COUNT 0`, and reported
`NA0352_OPERATION_EXECUTED_COUNT 0`.

The inherited proof is useful because it keeps the boundary executable:

- production backup/deploy/rollback/restore authorization is fixture-only;
- off-host backup remains future-gated;
- same-host local continuity is not complete disaster recovery;
- qsl-server PR #56 remains bounded end-to-end modeled harness evidence;
- qsl-attachments PR #37 remains service-local prerequisite evidence;
- qshield embedded relay/demo evidence remains reference/oracle evidence only;
- no live operation, public-ingress cutover, secret-dependent test, or public
  claim expansion occurred.

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
| branch protection | present |
| required check | strict `rust` |
| force pushes | disabled |
| deletions | disabled |
| admins enforced | enabled |
| open PRs | none listed |
| latest listed main CI | `ci` success on `d40e6003fdf0` |
| source classification | `FRESH_SOURCE` |
| authority classification | `COMPLETE_AUTHORITY` |
| CI classification | `COMPLETE_CI` |

qsl-server remains transport-only. Current qsl-server evidence includes
packaging/runbook/update/verify material and the NA-0349 end-to-end modeled
harness, but no production host operation was performed by NA-0353.

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
| branch protection | present |
| required check | strict `rust` |
| force pushes | disabled |
| deletions | disabled |
| admins enforced | not enabled in current protection |
| open PRs | none listed |
| latest listed main CI | `rust` success on `96b9352bd63e` |
| source classification | `FRESH_SOURCE` |
| authority classification | `COMPLETE_AUTHORITY` |
| CI classification | `COMPLETE_CI` |

qsl-attachments remains a single-node local-disk service. Its current contract
supports cold or quiesced whole-root backup/restore plus matching service
configuration only. Hot/live backup and partial restore remain unsupported.

## Local backup/off-host evidence refresh

Read-only local backup evidence:

- `/backup/qsl` is mounted from `/dev/sda1` as ext4.
- Disk use at inspection was about 20 GiB used of 916 GiB, about 3 percent.
- `/srv/qbuild` disk use was about 57 GiB used of 468 GiB, about 13 percent.
- `/usr/local/sbin/qsl-backup preflight` reported the target mounted and daily
  sources present.
- `/usr/local/sbin/qsl-backup list` listed checkpoint snapshots from
  2026-05-17 and daily snapshots through 2026-05-24.
- `qsl-backup-daily.timer` is enabled and waiting for the next scheduled run.
- manifests and logs exist for current daily snapshots.
- D132 preservation bundle
  `/srv/qbuild/tmp/NA-0322_D132_resume_bundle` is still present.

Current source-list evidence from
`/home/victor/work/qsl/codex/ops/backup/QSL_BACKUP_STATUS.md`:

- included: `/srv/qbuild/work`, `/srv/qbuild/tmp`, Codex logs, Codex
  responses, and `QSL_BACKUP_PLAN.md`;
- not fully proven as a covered source group: Codex directives, requests,
  journals, and ops history directories;
- absent during inspection: `/home/victor/work/qsl/codex/directives` and
  `/home/victor/work/qsl/codex/journals`;
- present during inspection: Codex responses, requests, and ops paths.

Classification:

- `LOCAL_CONTINUITY_PROVEN`
- `OFF_HOST_BACKUP_NOT_READY` for current disaster-recovery evidence
- `OFF_HOST_BACKUP_READY_FOR_AUTHORIZATION` for the next planning lane

No current evidence proves an off-host encrypted backup destination,
encryption/key custody, key recovery, off-host restore drill, off-host
retention/purge, off-host monitoring/alerting, or operator runbook.

## Off-host encrypted backup threat/value model

| Threat/value row | Current status | Required response |
| --- | --- | --- |
| Local disk loss | local platter snapshot helps only if source SSD fails and backup disk survives | off-host copy and restore proof |
| Machine loss | same host contains source and backup disk | off-host encrypted backup required |
| Ransomware/local compromise | attacker may reach mounted local backup | offline/immutable or access-separated off-host target |
| Accidental deletion | local snapshots provide bounded recovery | retention policy and restore drill |
| Corrupted local snapshot | manifests exist locally only | off-host manifest/checksum verification |
| Failed restore | local restore drill exists for limited files | full restore drill to isolated target |
| Key loss | no off-host key plan exists | key recovery and emergency access plan |
| Key exposure | no key custody plan exists | least-privilege storage, rotation, revocation, no-secret evidence |
| Off-host provider compromise | no provider chosen | client-side encryption before upload |
| Off-host provider outage | no target class chosen | target class and recovery fallback plan |
| Network outage | no off-host operation exists | retry/backoff and operator-visible failure policy |
| Cost/quota growth | no remote quota plan exists | budget/quota, pruning, and alert thresholds |
| Retention/purge mismatch | local retention exists only | off-host retention and purge consistency |
| Privacy of backup content | no encrypted remote copy exists | encrypt before upload; no plaintext remote artifacts |
| Log/artifact leakage | current evidence avoids secrets | future logs must redact paths, tokens, keys, and capabilities |
| Operator error | no off-host runbook exists | runbook with preflight, stop, rollback, and evidence rules |
| RTO/RPO | no off-host objective set | explicit recovery time and recovery point objectives |
| Public claims | current claims stay bounded | no disaster-recovery or readiness claim until evidence exists |

## Off-host encrypted backup prerequisite matrix

| Row | Current evidence | Proof status | Risk | Future proof required | Secret/key impact | Backup-plan impact | Implementation need | Blocker or ready | Successor relation |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Backup target class | none selected | missing | wrong durability/cost/security model | choose object storage, SSH host, removable media, or other exact target class | may require credentials | update required before mutation | future plan | ready for authorization | NA-0354 |
| Encryption tool choice | none selected | missing | incompatible restore or weak operator practice | compare restic/borg/rclone+crypt/age/GPG or exact local choice | key/passphrase handling required | update required | future plan | ready for authorization | NA-0354 |
| Key custody | no plan | missing | key loss or exposure | custody owner, storage location, access controls | high | update required | future plan | ready for authorization | NA-0354 |
| Key recovery | no plan | missing | encrypted backups unrecoverable | escrow/recovery procedure and test | high | update required | future plan | ready for authorization | NA-0354 |
| Passphrase/secret storage | no secret handling | missing | secrets printed or stored in evidence | no-secret artifact rule; secret manager or local protected file boundary | high | update required | future plan | ready for authorization | NA-0354 |
| Restore drill | local limited restore drill only | partial | backup cannot be trusted | isolated off-host restore drill with manifest/checksum verification | may require decryption secret | update required | future drill | ready for authorization | NA-0354 |
| Retention/purge | local retention only | partial | stale or missing remote backups | off-host retention, prune, failed-upload cleanup, evidence | may reveal names/counts | update required | future plan | ready for authorization | NA-0354 |
| Manifest/checksum | local manifests exist | partial | corrupted remote copy | remote manifest/checksum verification and tamper detection | no secret values | update required | future plan | ready for authorization | NA-0354 |
| Monitoring/alerting | local timer status only | partial | silent backup failure | alert channel, redacted logs, missed-run detection | may touch account secrets | update required | future plan | ready for authorization | NA-0354 |
| Cost/quota | no off-host quota | missing | runaway storage cost or failed backups | quota threshold, budget, pruning proof | remote account metadata | update required | future plan | ready for authorization | NA-0354 |
| Local qbuild source coverage | `/srv/qbuild/work` covered locally | partial | host loss loses both source and local backup | off-host encrypted coverage for selected roots | no secret values | update required | future implementation | ready for authorization | NA-0354 |
| qsl-server/qsl-attachments source coverage | local checkouts and GitHub refs exist | partial | local WIP loss | off-host coverage for unpushed work or mirror policy | none expected | update required if relied on | future plan | ready | NA-0354 |
| Codex responses coverage | included locally | partial | response evidence lost with host | off-host encrypted coverage | no secret values | update required | future implementation | ready | NA-0354 |
| Codex directives/requests/journals/ops coverage | directives/journals absent; requests/ops present but not fully covered | partial/missing | directive/history continuity gaps | local-ops source-list update or explicit noncritical classification | possible sensitive history | update likely | future local-ops | deferred | later or NA-0354 dependency |
| D132 bundle | present under `/srv/qbuild/tmp` and local backup source includes tmp | partial | preserved WIP lost with host | off-host encrypted coverage or retirement decision | no secret material quoted | update likely | future decision | ready | NA-0354 boundary |
| qsl-server runtime data future | qsl-server queue is non-durable | partial | false durable queue expectations | explicit non-durable queue/runbook proof | secrets/env may exist | update if roots added | future service lane | deferred | after NA-0354 |
| qsl-attachments object storage future | single local storage root; cold full-root only | partial | hot/partial restore overclaim | off-host encrypted cold/quiesced root drill | capabilities must not print | update required | future service lane | deferred | after NA-0354 |
| Deployment config future | `/etc`, systemd, proxy/TLS not in qbuild backup | missing | config cannot be restored | redacted config inventory and backup policy | secrets likely | update required | future service lane | deferred | after NA-0354 |
| Logs/monitoring future | no off-host monitor proof | missing | no failure visibility | redacted logs and alert evidence | secret leakage risk | update required | future lane | deferred | after NA-0354 |
| Disaster recovery claim boundary | explicitly not met | not ready | overclaim | off-host encrypted backup plus restore drill | key proof required | update required | future claim gate | blocked until evidence | later |
| Public claim boundary | conservative internal docs only | ready | premature public copy | public-claim audit before public wording changes | no secrets | no current update | future audit | deferred | later |

## Future implementation authorization options

| Option | Feasibility | Risk | Secret/key impact | Backup/local-ops impact | CI cost | Claim boundary | Result |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Off-host encrypted backup implementation authorization plan | high | planning must not become live setup | high, but can remain no-secret | defines required backup-plan update | low/medium | no completion claim | recommended |
| Off-host encrypted backup blocker resolution | medium | premature unless an exact blocker is proven | high | may need operator target/key input | low | no completion claim | deferred |
| Local-ops workflow/history index and backup coverage first | high | delays off-host backup | low/medium | improves history coverage | low | no readiness claim | deferred |
| Backup target selection prerequisite | high | needs operator/provider choice | medium/high | required before implementation | low | no completion claim | included in NA-0354 |
| Key custody/key recovery prerequisite | high | most sensitive part | high | required before implementation | low | no completion claim | included in NA-0354 |
| Restore-drill prerequisite | high | may require isolated storage | high | required before trust | medium | no completion claim | included in NA-0354 |
| Monitoring/alert prerequisite | medium | needs account/channel choice | medium | required before unattended operation | low/medium | no completion claim | included in NA-0354 |
| External review readiness audit | medium | premature before backup evidence | low | no direct impact | low | review incomplete | deferred |
| Website/public claim boundary audit | medium | public-copy risk | low | no direct impact | low | conservative only | deferred |
| Public technical position paper | medium | narrative can overclaim | low | no direct impact | low | evidence-bounded only | deferred |

## Encryption/key-handling/secrets boundary plan

NA-0353 created no key, uploaded no key, collected no passphrase, printed no
secret material, and handled no secret-bearing file.

Future NA-0354 must define:

- encryption tool selection criteria and exact rejected alternatives;
- client-side encryption before any off-host upload;
- key/passphrase owner, storage location, permissions, rotation, revocation,
  and recovery procedure;
- emergency access procedure that does not normalize shared plaintext secrets;
- no-secret evidence rule for logs, manifests, CI output, response files, and
  PR bodies;
- secret scanning for added evidence and generated artifacts;
- failure behavior when the key is unavailable or the remote target rejects
  writes;
- restore validation that proves the key procedure without disclosing the key.

## Restore drill/retention/purge/monitoring/runbook plan

Future restore drill categories:

- metadata-only restore of manifests and indexes to prove inventory integrity;
- qbuild/Codex source restore to an isolated target;
- qsl-server configuration restore using redacted config manifests;
- qsl-attachments cold/quiesced full storage-root restore to an isolated target;
- failed restore cleanup that does not overwrite live files;
- key-loss and remote-target-unavailable tabletop drills.

Boundary rules:

- dry-run or fixture restores may run only when explicitly authorized;
- real restore to live roots requires a later operation directive;
- restore targets must be isolated from live `/srv/qbuild`, service roots, and
  production paths;
- manifests and checksums must be verified before any claim of restorable data;
- retention must define keep counts/ages, prune order, failed-upload cleanup,
  and purge evidence;
- monitoring must detect missed runs, failed uploads, failed prunes, low quota,
  and restore-drill failures without printing secrets;
- operator runbooks must include preflight, stop, emergency hold, recovery,
  rollback, and evidence collection steps.

## Backup-plan impact and local-ops dependency decision

NA-0353 itself does not require a backup-plan update because changed files are
qsl-protocol governance/testplan/journal paths under `/srv/qbuild/work`, already
covered by the local continuity backup source list. The required Codex response
file is under the existing responses path, which is also covered locally.

Future off-host encrypted backup implementation will require a backup-plan
update before mutation because it will introduce or rely on new off-host target
configuration, encryption/key handling, restore-drill evidence, retention/purge
policy, monitoring/logging, and possibly local history source coverage.

Local workflow-support and history-index backup coverage should follow the
off-host authorization plan unless NA-0354 proves that Codex directive/request/
journal/ops history must be made durable first. Current evidence makes off-host
encrypted backup the higher production-hardening prerequisite.

D132 remains present and must not be deleted without explicit authorization.

## Public-ingress/timing/traffic-shape boundary

NA-0353 does not change public ingress. Public ingress remains future-gated.

Current evidence does not prove hidden attachment size, hidden timing metadata,
hidden traffic shape, hidden all metadata, or padding that hides all metadata.
Size-class, padding, batching, jitter, retry, and cover-traffic evidence remains
bounded and still leaves observable metadata surfaces.

## External-review sensitivity

External review remains not complete. NA-0353 is prerequisite planning only. It
does not make qsl-server, qsl-attachments, qshield, qsc, qsp, website, public
docs, or production deployment externally reviewed.

Any stronger statement requires off-host backup implementation evidence,
restore drill evidence, service evidence, deployment evidence, monitoring/log
evidence, rollback evidence, public-ingress evidence if relevant, and review
evidence.

## Public claim boundary

NA-0353 does not update website copy, public docs, README, or START_HERE.

Future public wording must not imply:

- production readiness;
- public-internet readiness;
- completed external review;
- anonymity;
- metadata-free behavior;
- untraceable behavior;
- hidden attachment size;
- hidden timing metadata;
- hidden traffic shape;
- padding hiding all metadata;
- local continuity backup as complete disaster recovery;
- off-host encrypted backup as complete before implementation and restore
  evidence exist.

## Future validation/marker/verification plan

Future NA-0354 marker candidates:

- `NA0354_OFF_HOST_BACKUP_PREREQUISITE_PLAN_OK`
- `NA0354_BACKUP_TARGET_BOUNDARY_OK`
- `NA0354_ENCRYPTION_TOOL_BOUNDARY_OK`
- `NA0354_KEY_CUSTODY_BOUNDARY_OK`
- `NA0354_KEY_RECOVERY_BOUNDARY_OK`
- `NA0354_SECRET_HANDLING_BOUNDARY_OK`
- `NA0354_RESTORE_DRILL_BOUNDARY_OK`
- `NA0354_RETENTION_PURGE_BOUNDARY_OK`
- `NA0354_MANIFEST_CHECKSUM_BOUNDARY_OK`
- `NA0354_MONITORING_ALERT_BOUNDARY_OK`
- `NA0354_LOCAL_HISTORY_BACKUP_BOUNDARY_OK`
- `NA0354_D132_BUNDLE_BOUNDARY_OK`
- `NA0354_NO_OFF_HOST_BACKUP_OPERATION_OK`
- `NA0354_NO_SECRET_MATERIAL_OK`
- `NA0354_NO_DISASTER_RECOVERY_COMPLETE_CLAIM_OK`
- `NA0354_NO_PRODUCTION_READY_CLAIM_OK`
- `NA0354_NO_PUBLIC_INTERNET_READY_CLAIM_OK`

If blocked, use exact blocker markers such as:

- `NA0354_BACKUP_TARGET_SELECTION_BLOCKED`
- `NA0354_KEY_CUSTODY_BLOCKED`
- `NA0354_KEY_RECOVERY_BLOCKED`
- `NA0354_RESTORE_DRILL_BLOCKED`
- `NA0354_BACKUP_PLAN_UPDATE_BLOCKED`
- `NA0354_LOCAL_HISTORY_BACKUP_SCOPE_BLOCKED`

Future verification bundle:

- qsl-protocol queue/decision proof;
- qsl-server/qsl-attachments source/authority/CI refresh;
- local backup/off-host status refresh;
- backup target, encryption tool, key custody, key recovery, restore-drill,
  retention/purge, monitoring/alerting, and runbook matrices;
- no-secret artifact scan;
- overclaim scan;
- scope guard, link-check, leak-scan, goal-lint, cargo audit, rustls-webpki
  proof, and required public-safety checks.

## Workflow-support and history-index future work note

Read-only history availability:

- `/home/victor/work/qsl/codex/directives`: absent.
- `/home/victor/work/qsl/codex/responses`: present; prior D166/NA0352 response
  was present and improved identity confidence.
- `/home/victor/work/qsl/codex/journals`: absent.
- `/home/victor/work/qsl/codex/requests`: present; workflow-support and
  history-access requests were inspected read-only.
- `/home/victor/work/qsl/codex/ops`: present but not fully proven as a covered
  source group in the installed backup source list.

Future local-ops improvements would materially reduce friction:

- qstart/qresume fast-forward to expected `origin/main`;
- response-file writer;
- bounded PR/public-safety polling helper;
- machine-readable directive manifest;
- validation profiles;
- per-directive allow-file;
- read-only source/authority helper;
- claim-boundary scanner;
- directive/response/journal index;
- backup coverage for directives, requests, journals, and ops history folders.

These items are not implemented by NA-0353.

## Selected successor

Selected:

`NA-0354 -- Metadata Runtime Off-Host Encrypted Backup Implementation Authorization Plan`

Rationale:

- current local continuity backup is healthy enough for source/governance
  continuity but not enough for disaster recovery;
- no qsl-server/qsl-attachments source, authority, CI, or open-PR blocker was
  found;
- the missing off-host target, encryption tool, key custody, key recovery,
  restore drill, retention/purge, monitoring, and runbook decisions are exactly
  what an implementation authorization plan must freeze before any mutation;
- local-ops workflow/history-index work is important, but current evidence
  makes off-host encrypted backup the higher production-hardening prerequisite.

## Rejected alternatives

- `Metadata Runtime Off-Host Encrypted Backup Blocker Resolution`: rejected for
  the immediate successor because no exact source/authority/CI blocker was
  found; target/key/restore unknowns belong in the authorization plan.
- `QSL Local Ops Codex Workflow Support and History Index Plan`: deferred
  because it reduces friction but does not outrank the off-host backup gap.
- `Metadata Runtime Restore Drill Prerequisite Plan`: included as a required
  NA-0354 authorization component rather than split out first.
- `Metadata Runtime External Review Readiness Gap Audit`: deferred because
  external review remains premature before off-host backup evidence.
- `Metadata Runtime Website / Public Claim Boundary Audit`: deferred because
  no public copy changes in NA-0353 and claims remain conservative.
- `Public Technical Position Paper Evidence-Bounded Draft Plan`: deferred until
  backup/deploy/rollback/off-host backup boundaries are clearer.

## Backup-plan impact statement

No backup-plan update is required for NA-0353. Future off-host encrypted backup
implementation will require backup-plan update and exact local-ops
authorization before any script, timer, fstab, system service, source list,
off-host destination, key custody, restore drill, retention/purge, or monitoring
mutation.

## Next recommendation

After NA-0353 merges and closeout completes, restore:

`NA-0354 -- Metadata Runtime Off-Host Encrypted Backup Implementation Authorization Plan`
