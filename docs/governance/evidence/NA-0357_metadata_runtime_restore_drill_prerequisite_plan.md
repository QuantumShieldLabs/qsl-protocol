Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-25

# NA-0357 Metadata Runtime Restore Drill Prerequisite Plan

Goals: G1, G2, G3, G4, G5

## Executive summary

NA-0357 records the restore-drill prerequisite plan after NA-0356 key
custody/key recovery prerequisite planning. It is governance and planning only.

Result:

- restore-drill classification: `RESTORE_DRILL_READY_FOR_AUTHORIZATION`;
- backup classification: `LOCAL_CONTINUITY_PROVEN` and
  `OFF_HOST_BACKUP_NOT_READY`;
- key classification: `KEY_CUSTODY_PARTIAL` and `KEY_RECOVERY_PARTIAL`;
- future dry-run direction: no-secret fixture and manifest/checksum validation
  inside qsl-protocol before any live backup or restore operation;
- future isolated real restore direction: non-live isolated target only after
  explicit key custody/recovery, backup-plan, local-ops, cleanup, monitoring,
  and evidence authorization;
- selected successor:
  `NA-0358 -- Metadata Runtime Restore Drill Implementation Authorization Plan`.

NA-0357 executed no restore, created no restore target, mounted no restore
target, copied no restored payload, generated no key, uploaded no key,
collected no passphrase, inspected no private key material, initialized no
repository, configured no remote target, ran no backup, ran no deploy, ran no
rollback, and changed no local backup script, timer, fstab, source list,
service repo, runtime, dependency, workflow, website, README, START_HERE, or
public docs.

## Live NA-0357 scope

The live queue marks NA-0357 READY and requires a restore-drill prerequisite
plan after NA-0356. The live objective is to define restore-drill
prerequisites, isolated verification boundary, retention/old-key
compatibility, monitoring/alerting evidence, backup-plan update trigger, and
operator runbook gates before any off-host encrypted backup target/tool
implementation or key-handling operation can safely proceed.

The live scope permits read-only refresh of NA-0356 key custody/recovery
evidence, NA-0355 target/tool selection evidence, qsl-server and
qsl-attachments authority/CI, and local backup/off-host posture. It forbids
off-host setup, backup, restore, deploy, rollback, key generation, key upload,
passphrase collection, secret material handling, private key inspection,
restore target creation/mount/copy, backup script/timer/fstab mutation,
qsl-server or qsl-attachments mutation, qshield/qsc/qsp/protocol/crypto
mutation, dependency/workflow mutation, website/public-doc mutation, README
mutation, and START_HERE mutation.

Acceptance requires exactly one READY item, NA-0356 DONE, D-0694 and D-0695
present once, and a restore-drill prerequisite decision or exact blocker before
future off-host implementation authorization.

## Inherited NA-0356 key custody/recovery prerequisite plan

NA-0356 selected a future custody direction and a future recovery direction
only:

- custody direction: operator-held repository secret with offline recovery
  envelope and strict no-secret evidence handling;
- recovery direction: sealed offline recovery envelope plus isolated restore
  verification before reliance;
- classification: `KEY_CUSTODY_PARTIAL`, `KEY_RECOVERY_PARTIAL`, and
  `TARGET_TOOL_IMPLEMENTATION_DEFERRED`;
- selected successor: `NA-0357 -- Metadata Runtime Restore Drill Prerequisite
  Plan`.

NA-0356 did not implement custody or recovery. It created no key, uploaded no
key, collected no passphrase, inspected no private key material, initialized no
repository, configured no remote target, ran no backup, ran no restore, and
changed no local backup script, timer, fstab, source list, service repo,
runtime, dependency, workflow, website, README, START_HERE, or public docs.

## Inherited NA-0355 target/tool selection

NA-0355 selected a class-level target and tool strategy only:

- target class: SSH/SFTP-compatible off-host host controlled by, or explicitly
  delegated to, the operator;
- tool class: restic-style encrypted snapshot repository with client-side
  encryption, check, prune, and isolated restore support;
- classification: `TARGET_TOOL_SELECTION_PARTIAL`;
- gating classification: `TARGET_TOOL_DEFERRED_KEY_CUSTODY`.

NA-0355 did not select a live host, remote path, provider account, credential,
repository password, key, passphrase, schedule, retention value, or alert
channel. It did not initialize a repository and did not run backup, restore,
deploy, rollback, purge, key generation, key upload, passphrase collection, or
secret handling.

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

qsl-server remains transport-only for this lane. PR #56 remains bounded
end-to-end harness evidence only, not production or public-internet proof. No
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
| branch protection | present, strict `rust` required |
| force pushes | disabled |
| deletions | disabled |
| admins enforced | not enabled in current protection |
| open PRs | none listed |
| latest listed main CI | `rust` success on `96b9352bd63e` |
| source classification | `FRESH_SOURCE` |
| authority classification | `COMPLETE_AUTHORITY` |
| CI classification | `COMPLETE_CI` |

qsl-attachments remains a single-node local-disk service. PR #37 remains
service-local prerequisite evidence only. Its cold/quiesced full-root restore
boundary does not prove hot/live backup, partial restore, public-internet
readiness, or complete disaster recovery. No qsl-attachments mutation was
performed.

## Local backup/key/restore evidence refresh

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

| Tool | Local result | Restore-drill impact |
| --- | --- | --- |
| `restic` | not found | future selected tool class is not installed or implemented |
| `borg` | not found | fallback not available locally |
| `rclone` | not found | cloud/object transport tooling not available locally |
| `age` | not found | envelope-wrapper candidate not available locally |
| `gpg` | found | available but not preferred as primary backup tool |
| `ssh` | found | supports the selected SSH/SFTP-compatible target class |
| `rsync` | found | supports current local continuity model |

Backup/history evidence:

- local backup status lists `/srv/qbuild/work`, `/srv/qbuild/tmp`, qbuild
  mirrors, qbuild evidence/logs/archive, Codex logs, Codex responses, and
  `QSL_BACKUP_PLAN.md` as daily sources;
- Codex responses and requests paths were present;
- Codex directives and journals paths were absent;
- Codex ops backup path was present, but the installed status did not list the
  whole ops tree as a daily source group;
- the local backup plan states that same-host platter backup is local
  continuity only and should not be the only disaster recovery copy.

Classification:

- `LOCAL_CONTINUITY_PROVEN`
- `RESTORE_DRILL_READY_FOR_AUTHORIZATION`
- `KEY_CUSTODY_PARTIAL`
- `KEY_RECOVERY_PARTIAL`
- `OFF_HOST_BACKUP_NOT_READY`

No current evidence proves an off-host encrypted backup target, repository
initialization, repository password/key custody, key recovery envelope, key
rotation procedure, emergency access procedure, off-host restore drill, remote
retention/purge, remote monitoring/alerting, or operator runbook.

## Restore-drill threat/value model

| Threat/value row | Risk | Required boundary |
| --- | --- | --- |
| false confidence from untested backups | a backup may exist but fail when needed | drill evidence must distinguish dry-run, fixture, isolated real restore, and live operation |
| restore failure | missing files, corrupt archive, missing key, or bad path can block recovery | fail closed and retain evidence without claiming recovery readiness |
| wrong restore path | restored data can overwrite live work or service state | isolated target must be non-live and explicitly named before use |
| live data overwrite | destructive data loss | production roots are forbidden for restore drills until a future explicit directive |
| stale manifest | validation can compare against outdated source state | manifest must bind source snapshot ID, timestamp, and path inventory |
| corrupted archive | restore can silently return incomplete data | checksum and repository check stages must precede trust |
| missing key or wrong passphrase | encrypted archive becomes unrecoverable | key custody/recovery proof is required before isolated real restore |
| old key archive compatibility | rotated keys can strand old snapshots | old-key inventory and retention windows must be defined |
| partial restore | only a subset is restored without being detected | required file inventory and checksum coverage must name expected omissions |
| failed cleanup | temp targets can retain sensitive or stale data | cleanup plan and post-cleanup proof are required |
| secret leakage in logs | response files, PRs, journals, and logs can become sensitive | no-secret evidence and added-line scans are mandatory |
| public claim risk | planning can be mistaken for executed recovery | all public-facing claims remain NOT_READY/PARTIAL until exact evidence exists |
| RTO/RPO realism | operators may assume timing/recovery freshness not proven | future drills must record bounded recovery time and recovery point facts |
| operator burden | complex procedures increase mistakes | runbook must be concise, repeatable, and stop-first |
| CI/dry-run feasibility | CI must not need real secrets | fixture-only dry-run evidence is the CI path |
| isolated target requirements | restore must not touch live state | future operation must prove target isolation before any restore command |

## Dry-run restore model analysis

| Option | Local evidence | Secret risk | Confidence gained | Confidence not gained | Result |
| --- | --- | --- | --- | --- | --- |
| manifest-only validation | local manifests exist for current continuity snapshots | low if manifests contain paths only | proves inventory shape, freshness, expected files | does not prove archive decryption or data restoration | recommended as a future stage |
| checksum-only validation | manifest process exists, but exact checksum coverage needs future definition | low if checksums are not secret material | proves changed/corrupt local artifacts can be detected | does not prove target isolation or key recovery | recommended with redaction rules |
| fixture-only restore simulation | qsl-protocol harness style exists from NA-0352 | low | proves command sequencing, stop conditions, artifact scan, marker contract | does not prove real repository readability | recommended for NA-0358 authorization |
| no-secret archive simulation | selected restic-style class exists only as a future tool class | low if fixture repository contains no real source data or secrets | can prove restore command boundaries before live data | does not prove production source coverage | recommended after exact fixture/tool authorization |
| qsl-protocol fixture harness extension | existing local fixture harness patterns are green | low | gives CI-repeatable evidence and markers | cannot initialize a real off-host repository | recommended |
| existing `qsl-backup list/preflight` | preflight/list passed read-only | low | proves current local continuity target is mounted and visible | does not prove off-host encrypted restore | supportive only |
| dry-run only | current evidence is dry-run/list/preflight only | low | safe prerequisite evidence | cannot justify disaster recovery or key recovery reliance | insufficient as final proof |

Future dry-run tests must emit no-secret markers, prove no restore execution,
prove no restore target creation/mount/copy unless explicitly authorized, and
fail closed on missing manifest, stale manifest, checksum mismatch, missing key
placeholder, missing cleanup plan, or public-claim overreach.

## Isolated real restore model analysis

| Future target category | Safety | Required authorization | Key/secret dependency | Backup-plan impact | Evidence value | Result |
| --- | --- | --- | --- | --- | --- | --- |
| isolated temp directory under `/srv/qbuild/tmp` | highest feasible for first real drill if non-live and auto-cleaned | future local-ops/restore directive with exact path and cleanup | yes for encrypted real archive | likely yes if durable artifacts or source lists change | proves real restore mechanics without live overwrite | recommended later |
| isolated disposable disk | strong isolation, more setup | future local system/device directive | yes | yes | proves disk-level recovery and cleanup | deferred |
| isolated non-live qbuild restore target | useful if path is clearly outside active worktrees | future qbuild/local-ops directive | yes | yes | proves qbuild tree restore shape | deferred |
| off-host restore to staging machine | strong disaster-recovery signal | future staging host/off-host directive | yes | yes | proves remote recovery and host rebuild assumptions | deferred |
| production root restore | unsafe for drill planning | production incident directive only | yes | yes | high but destructive risk | rejected for drill |
| no isolated restore | avoids immediate risk | none | none | none | no real restore confidence | rejected as final state |

No isolated real restore is authorized by NA-0357. Future isolated real restore
must prove target isolation, no live overwrite, no secret logs, cleanup, alert
on failure, and backup-plan coverage before any operation.

## Manifest/checksum/artifact integrity plan

Required future plan elements:

- manifest source: authoritative backup manifest generated with the snapshot or
  fixture archive, including source set, snapshot ID, timestamp, expected path
  inventory, and explicit exclusions;
- checksum source: deterministic checksum inventory for non-secret fixture
  files and any approved real-restore validation subset;
- validation stages: parse manifest, reject stale/missing manifest, verify
  source inventory, verify archive/repository check result, restore only into
  approved isolated target, verify restored checksums, verify no forbidden
  files, scan evidence artifacts, cleanup target, and verify cleanup;
- artifact redaction: no path should expose secrets, credentials, endpoint
  tokens, private keys, passphrases, auth headers, or route tokens;
- no-secret proof: marker output and leak-scan over added evidence;
- failed validation behavior: stop immediately, preserve only redacted logs,
  avoid purge/prune, and do not retry with weaker validation;
- output artifact location: qsl-protocol fixture evidence may stay under
  checked-in docs/tests; temporary generated proof belongs under
  `/srv/qbuild/tmp` unless future backup-plan authorization chooses a durable
  location;
- cleanup behavior: remove isolated restore target after validation unless a
  future directive explicitly preserves it as evidence;
- future evidence retention: keep summaries and marker logs, not restored
  payloads or secrets;
- backup-plan impact: any durable restore artifact, key artifact, recovery
  envelope inventory, source-list mutation, timer/script change, or local
  system change requires backup-plan and local-ops authorization first.

## Key custody/recovery dependency for restore drill

Dry-run restore can proceed before key custody implementation only when it uses
no-secret fixtures, placeholder key-status checks, and no live encrypted
repository. It must not imply that key custody works.

Isolated real restore requires key custody and key recovery implementation
evidence first or in the same future explicitly authorized operation bundle.
Required dependencies:

- repository secret custody owner and access path;
- sealed offline recovery envelope inventory;
- old-key archive compatibility plan;
- key rotation compatibility, including whether old keys remain sealed until
  old snapshots expire or are migrated;
- emergency access trigger, access log, and post-use rotation assessment;
- no private-key inspection in evidence;
- no passphrase collection in CI or PR text;
- no secret logs in restore, journal, response, or PR body;
- fail-closed stop if a key is missing, ambiguous, exposed, wrong, or cannot
  be recovered without printing secret material.

## Retention/purge/old-archive compatibility analysis

Future restore-drill implementation must define:

- retention policy dependency: how many snapshots and what time windows must
  remain before a drill result is meaningful;
- purge policy dependency: prune/purge must never delete the only usable
  recovery point or the only snapshot compatible with an old key;
- old backup availability: at least one current and one older eligible snapshot
  should be named for compatibility testing when authorized;
- old key compatibility: retained old archives must retain their required
  sealed key material until expiry or migration;
- archive retirement boundary: archived snapshots may be retired only after
  retention policy, key compatibility, and recovery evidence support it;
- failed backup cleanup: incomplete or failed snapshots must not become restore
  candidates;
- failed restore cleanup: failed isolated targets must be cleaned or explicitly
  preserved with no secrets and a stop reason;
- evidence retention: keep redacted summary, markers, manifest identifiers, and
  checksum summary; do not keep restored payloads in PR evidence;
- monitoring/alerting dependency: alert on missed backup, failed check, failed
  prune, failed restore, missing manifest, missing checksum, missing key proof,
  quota pressure, and stale recovery envelope inventory;
- public-claim boundary: retention/purge planning is not proof of disaster
  recovery or off-host backup completion.

## Failed restore cleanup/monitoring/alerting/runbook plan

Future runbook sections must include:

- failed dry-run cleanup: remove generated fixture output and preserve only
  redacted markers/log summaries;
- failed isolated restore cleanup: stop before retry, quarantine or remove the
  isolated target, record a redacted failure summary, and do not prune;
- stale temp path cleanup: detect pre-existing restore target paths and fail
  closed unless the future directive authorizes cleanup;
- alert on failed restore drill;
- alert on missing manifest/checksum;
- alert on missing key custody/recovery proof;
- emergency stop for wrong target path, live path match, secret in logs,
  checksum mismatch, stale manifest, wrong key, missing recovery envelope,
  unexpected network/off-host command, or public-claim overreach;
- operator verification: target path, source snapshot, manifest ID, checksum
  summary, key proof presence, no-secret scan, cleanup proof, and stop reason;
- no-secret logs: never print passphrases, private keys, tokens, raw secrets,
  auth headers, or sensitive endpoint fragments;
- audit artifact summary: command list, marker set, classifications, redacted
  manifest/checksum summary, cleanup proof, and backup-plan impact.

## Backup-plan impact and local-ops dependency decision

NA-0357 itself requires no backup-plan update because changed paths are
qsl-protocol governance/testplan/journal paths under the already-covered
`/srv/qbuild/work` tree.

Future restore-drill implementation requires backup-plan update and exact
local-ops authorization before any of the following exist or change:

- real restore target;
- durable restore artifact outside checked-in qsl-protocol docs/tests;
- key material or recovery envelope inventory;
- repository, remote target, source list, script, timer, fstab, system service,
  retention/purge policy, monitoring artifact, backup operation, restore
  operation, deploy operation, rollback operation, or public claim.

Local workflow-support/history-index work would materially reduce friction, but
it does not outrank restore-drill authorization now. The recommended order is:

1. NA-0358 restore-drill implementation authorization plan.
2. Local-ops workflow/history index and backup coverage lane if NA-0358 proves
   it is a direct prerequisite for executable restore evidence, otherwise keep
   it as a near-term follow-on.
3. Key custody/recovery or off-host implementation lane only after NA-0358
   defines exact executable restore evidence and stop conditions.

Read-only history/backup coverage at inspection:

- `/home/victor/work/qsl/codex/responses`: present and listed in backup status;
- `/home/victor/work/qsl/codex/requests`: present but not listed as a daily
  source in the inspected status;
- `/home/victor/work/qsl/codex/directives`: absent;
- `/home/victor/work/qsl/codex/journals`: absent;
- `/home/victor/work/qsl/codex/ops`: present indirectly under ops/backup, but
  the whole ops tree is not listed as a daily source group;
- D132 bundle: present under `/srv/qbuild/tmp` and not deleted.

## Public-ingress/timing/traffic-shape boundary

NA-0357 does not change public ingress. It does not prove hidden attachment
size, hidden timing metadata, hidden traffic shape, hidden all metadata, or
padding hiding all metadata. qshield embedded relay/demo evidence remains
reference/oracle evidence only. qsl-server PR #56 remains bounded end-to-end
harness evidence only. qsl-attachments PR #37 remains service-local
prerequisite evidence only.

## External-review sensitivity

External review remains incomplete. Restore-drill prerequisite planning is not
external review. Any stronger external-review statement requires key custody
evidence, key recovery evidence, off-host backup evidence, restore drill
evidence, service evidence, deployment evidence, monitoring/log evidence,
rollback evidence, and external reviewer evidence.

## Public claim boundary

NA-0357 must not be described as restore execution, disaster recovery
completion, off-host backup completion, key custody implementation, key
recovery implementation, production readiness, public-internet readiness,
external-review completion, anonymity, metadata-free behavior, untraceable
behavior, hidden attachment size, hidden timing metadata, hidden traffic shape,
or hidden all metadata.

Website/public docs remain unchanged. Any future public-claim update requires a
separate explicit directive and evidence that preserves all current NOT_READY
and PARTIAL boundaries.

## Future validation/marker/verification plan

Future NA-0358 validation should prove or explicitly block these markers:

- `NA0358_RESTORE_DRILL_PREREQUISITE_PLAN_OK`
- `NA0358_DRY_RUN_RESTORE_BOUNDARY_OK`
- `NA0358_ISOLATED_RESTORE_BOUNDARY_OK`
- `NA0358_MANIFEST_CHECKSUM_PLAN_OK`
- `NA0358_KEY_CUSTODY_DEPENDENCY_OK`
- `NA0358_KEY_RECOVERY_DEPENDENCY_OK`
- `NA0358_RETENTION_PURGE_DEPENDENCY_OK`
- `NA0358_MONITORING_ALERT_DEPENDENCY_OK`
- `NA0358_FAILED_RESTORE_CLEANUP_PLAN_OK`
- `NA0358_OPERATOR_RUNBOOK_PLAN_OK`
- `NA0358_BACKUP_PLAN_UPDATE_REQUIRED_OK`
- `NA0358_NO_RESTORE_EXECUTION_OK`
- `NA0358_NO_KEY_GENERATION_OK`
- `NA0358_NO_PASSPHRASE_COLLECTION_OK`
- `NA0358_NO_SECRET_MATERIAL_OK`
- `NA0358_NO_DISASTER_RECOVERY_COMPLETE_CLAIM_OK`
- `NA0358_NO_PRODUCTION_READY_CLAIM_OK`
- `NA0358_NO_PUBLIC_INTERNET_READY_CLAIM_OK`

If NA-0358 is blocked, blocker markers should name the exact missing isolated
target, manifest/checksum rule, key custody proof, key recovery proof,
retention/purge rule, monitoring rule, cleanup rule, runbook section, or
backup-plan/local-ops authorization.

## Workflow-support and history-index future work note

The accepted local workflow-support request remains valid future local-ops
work. The following items would reduce repeated directive friction:

- qstart/qresume fast-forward to expected `origin/main` before handoff;
- response-file writer;
- bounded PR/public-safety polling helper;
- machine-readable directive manifest;
- validation profiles;
- per-directive allow-file;
- read-only source/authority helper;
- claim-boundary scanner;
- directive/response/journal index;
- backup coverage for directives/requests/journals/ops history folders.

Those items are not implemented in NA-0357. They may become the exact
successor only if a future restore-drill authorization lane proves that local
history/ops coverage is a blocker for truthful executable evidence.

## Selected successor

Selected successor:

`NA-0358 -- Metadata Runtime Restore Drill Implementation Authorization Plan`

Rationale:

- target/tool classes have already been selected by NA-0355;
- key custody/recovery prerequisite planning exists from NA-0356;
- current source/authority/CI evidence for qsl-server and qsl-attachments is
  fresh enough for planning;
- local continuity backup is mounted and current, while off-host encrypted
  backup remains unimplemented;
- the next missing prerequisite is exact restore-drill authorization, including
  no-secret dry-run fixture boundaries, isolated real restore prerequisites,
  manifest/checksum rules, key dependencies, cleanup, monitoring, runbook,
  backup-plan triggers, and public-claim stops.

## Rejected alternatives

- Direct restore execution in NA-0357: rejected because NA-0357 is prerequisite
  planning only.
- Direct key handling: rejected because no key generation, passphrase
  collection, private-key inspection, or secret handling is authorized.
- Off-host target/tool implementation authorization: rejected for now because
  restore-drill authorization is the next missing prerequisite after NA-0356.
- Key custody/recovery implementation authorization: deferred until restore
  drill authorization defines exact executable proof and stop conditions.
- QSL Local Ops Codex Workflow Support and History Index Plan: useful but not
  the next blocking prerequisite.
- External review readiness gap audit, website/public-claim audit, or public
  technical position paper plan: deferred until backup, key, restore, service,
  deployment, monitoring, rollback, and review evidence are stronger.
- Claiming disaster recovery completion: rejected because current evidence is
  local continuity only and off-host encrypted backup remains unimplemented.

## Backup-plan impact statement

No NA-0357 backup-plan update is required. Future restore-drill implementation
requires backup-plan update and exact local-ops authorization before any local
system, backup script, timer, fstab, source-list, tool, repository, key,
restore target, durable restore artifact, retention, purge, monitoring,
deploy, rollback, backup, restore, or public-claim mutation.

## Next recommendation

After NA-0357 merges and public-safety is green, close out NA-0357 and restore
`NA-0358 -- Metadata Runtime Restore Drill Implementation Authorization Plan`
as the sole READY successor. NA-0358 must not execute a restore unless a later
explicit directive authorizes exact operation, target isolation, key handling,
backup-plan update, local-ops mutation, cleanup, monitoring, and no-secret
evidence.
