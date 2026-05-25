Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-25

# NA-0356 Metadata Runtime Key Custody Key Recovery Prerequisite Plan

Goals: G1, G2, G3, G4, G5

## Executive summary

NA-0356 records the key custody and key recovery prerequisite plan after
NA-0355 selected the off-host backup target/tool classes. It is governance and
planning only.

Result:

- key custody/recovery classification: `KEY_CUSTODY_PARTIAL` and
  `KEY_RECOVERY_PARTIAL`;
- implementation classification: `TARGET_TOOL_IMPLEMENTATION_DEFERRED`;
- backup classification: `LOCAL_CONTINUITY_PROVEN` and
  `OFF_HOST_BACKUP_NOT_READY`;
- selected custody direction for future authorization: operator-held
  repository secret with offline recovery envelope and strict no-secret
  evidence handling;
- selected recovery direction for future authorization: sealed offline recovery
  envelope plus isolated restore verification before reliance;
- selected successor:
  `NA-0357 -- Metadata Runtime Restore Drill Prerequisite Plan`.

NA-0356 created no key, uploaded no key, collected no passphrase, inspected no
private key material, initialized no repository, configured no remote target,
ran no backup, ran no restore, and changed no local backup script, timer, fstab,
source list, service repo, runtime, dependency, workflow, website, README,
START_HERE, or public docs.

## Live NA-0356 scope

The live queue marks NA-0356 READY and requires a no-secret key custody/key
recovery prerequisite plan for the selected SSH/SFTP-compatible target class
and restic-style encrypted repository tool class. It allows read-only refresh
of NA-0355 evidence, qsl-server and qsl-attachments authority/CI, and local
backup/off-host posture. It forbids off-host setup, backup, restore, deploy,
rollback, key generation, key upload, passphrase collection, secret material
handling, private key inspection, backup script/timer/fstab mutation, qsl-server
or qsl-attachments mutation, qshield/qsc/qsp/protocol/crypto mutation,
dependency/workflow mutation, website/public-doc mutation, README mutation, and
START_HERE mutation.

Acceptance requires exactly one READY item, NA-0355 DONE, D-0692 and D-0693
present once, and a key custody/key recovery decision or exact blocker before
future implementation authorization.

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

## Local backup/key/off-host evidence refresh

Read-only local evidence:

- `/backup/qsl` is mounted as ext4 and has current local continuity snapshots.
- `/srv/qbuild` had ample free space at inspection.
- `qsl-backup` syntax check passed.
- `qsl-backup preflight` reported the target mounted and daily sources present.
- `qsl-backup list` listed checkpoint snapshots from 2026-05-17 and daily
  snapshots through 2026-05-24.
- local manifests and logs exist for the listed local snapshots.
- `qsl-backup-daily.timer` is enabled and waiting.
- D132 preservation bundle remains present under `/srv/qbuild/tmp`.
- installed tool discovery found `gpg`, `ssh`, and `rsync`; `restic`, `borg`,
  `rclone`, and `age` were not found.

Backup/history evidence:

- local backup status lists `/srv/qbuild/work`, `/srv/qbuild/tmp`, qbuild
  mirrors, qbuild evidence/logs/archive, Codex logs, Codex responses, and
  `QSL_BACKUP_PLAN.md` as daily sources;
- Codex responses and requests paths were present;
- Codex directives and journals paths were absent;
- Codex ops was present, but the installed status did not list the whole ops
  tree as a daily source group;
- the local backup plan states that same-host platter backup is local
  continuity only and should not be the only disaster recovery copy.

Classification:

- `LOCAL_CONTINUITY_PROVEN`
- `KEY_CUSTODY_PARTIAL`
- `KEY_RECOVERY_PARTIAL`
- `OFF_HOST_BACKUP_NOT_READY`

No current evidence proves an off-host encrypted backup target, repository
initialization, repository password/key custody, key recovery envelope, key
rotation procedure, emergency access procedure, off-host restore drill, remote
retention/purge, remote monitoring/alerting, or operator runbook.

## Key custody threat/value model

| Threat/value row | Risk | Required boundary |
| --- | --- | --- |
| key loss | encrypted archives become unrecoverable | recovery envelope and periodic restore verification |
| passphrase loss | same as key loss for restic-style repository | documented emergency recovery path before reliance |
| single-operator dependency | operator absence blocks recovery | emergency access procedure with accountability |
| operator unavailable | recovery may miss RTO/RPO | role handoff and sealed recovery inventory |
| accidental exposure | shell history, clipboard, logs, screenshots, or PR text may leak secrets | no-secret evidence rule and operator handling runbook |
| file permission exposure | local secret files can be world-readable or backup-copied | future permission checks and explicit source/exclusion policy |
| private key in repo | permanent public or team exposure | fail closed; repository scans before merge |
| secret in logs/artifacts | evidence bundle becomes sensitive | redaction and added-line leak scan |
| cloud/provider compromise | provider-held data may be read or deleted | client-side encryption and access-separated target |
| hardware failure | local source and backup may fail together | off-host copy plus tested recovery |
| emergency access | recovery path can normalize shared plaintext | sealed envelope, access log, and break-glass criteria |
| legal/organizational handoff | unclear control changes can lose accountability | named role, handoff checklist, and revocation step |
| rotation/revocation | old archives may still need old secrets | archive/key compatibility and retention plan |
| auditability | cannot prove safety if evidence contains secrets | no-secret markers, command logs, and scans |
| operator burden | complex process increases mistakes | simplest sufficient model first |
| no-secret CI | CI must not need real secrets | fixture-only validation and sentinel-only scans |

## Key recovery threat/value model

| Threat/value row | Risk | Required boundary |
| --- | --- | --- |
| recovery envelope absence | backup is encrypted but not recoverable after primary loss | envelope required before trust |
| recovery envelope compromise | attacker can decrypt off-host repository | physical protection, tamper evidence, rotation response |
| recovery envelope loss | same failure mode as primary loss | duplicate offline copy or second operator escrow if authorized |
| recovery verification | proof may expose the secret | verify by restore result and redacted evidence, not secret values |
| periodic recovery drill | stale process may fail during incident | schedule and success markers before reliance |
| multiple-person recovery | reduces single-operator risk but increases complexity | defer until simple model proves insufficient |
| break-glass procedure | ad hoc access can leak secrets | explicit trigger, operator log, and post-use rotation |
| lost primary device | normal operations cannot access repository | recovery envelope and rekey/rotation procedure |
| compromised primary device | attacker may have repository secret | freeze, rotate, and treat old archives as exposed until assessed |
| retired key | old archives may require old key | retention and key archive compatibility plan |
| rotation with existing archives | new backups may not cover old restore points | preserve old key until old snapshots expire or are migrated |
| test recovery without production secrets | cannot run secret-dependent CI | no-secret fixture strategy and isolated real drill later |
| public-claim risk | untested recovery can be overstated | keep all disaster-recovery claims NOT_READY until drill evidence |

## Custody model option analysis

| Custody model | Local evidence | Risk/burden | Compatibility | Result |
| --- | --- | --- | --- | --- |
| operator-held passphrase | no live passphrase exists; restic-style class selected | simple but loss/exposure risk is high | compatible with SSH/SFTP + restic-style repository; compatible with no-secret CI if never printed | recommended with recovery envelope |
| age recipient private key offline | `age` not installed; no recipient exists | clean public-recipient model but separate wrapper complexity | useful for envelope wrapping, not primary restic repository control | deferred |
| gpg recipient private key offline | `gpg` installed | higher operator/keyring complexity | possible envelope wrapper, weaker ergonomics for this lane | deferred |
| hardware token | no token evidence | strong custody but introduces hardware, PIN, backup-token, and availability dependencies | potentially compatible later | deferred |
| split secret | no local procedure | strong for shared recovery, high ceremony and error risk | compatible only after runbook maturity | deferred |
| offline recovery envelope | no envelope exists | must be physically protected and tracked | compatible with selected tool class and future restore drill | recommended as recovery control |
| service-managed key | no service selected | provider/key-service dependency can undercut client-side control | not acceptable as sole control for this target/tool path | rejected as primary |
| no custody model/deferred | current state | leaves backup unrecoverable or unsafe | incompatible with implementation authorization | rejected |

Future authorization should use the simplest sufficient model first: one
operator-held repository secret for normal operation, plus sealed offline
recovery envelope. Any hardware-token, split-secret, age, or gpg wrapper can be
added later only if it reduces a demonstrated risk without weakening no-secret
evidence discipline.

## Recovery model option analysis

| Recovery model | Local evidence | Risk/burden | Restore reliability | Result |
| --- | --- | --- | --- | --- |
| sealed offline recovery envelope | absent | compromise/loss risk must be managed | high if tested periodically | recommended |
| second offline encrypted copy | absent | needs inventory and physical separation | useful backup to envelope | recommended for later local-ops consideration |
| second operator escrow | no second-operator procedure | improves availability but increases access surface | viable only with accountability | deferred |
| split secret recovery | absent | high complexity and ceremony | strong if practiced, brittle if not | deferred |
| hardware token backup | absent | token loss/PIN management | strong if duplicate token works | deferred |
| paper recovery instructions without secret | absent | cannot recover alone | good no-secret runbook companion | recommended as non-secret artifact |
| no recovery | current state | encrypted backups can be permanently lost | unacceptable for implementation | rejected |

## Key rotation/emergency access/incident response analysis

Rotation triggers:

- scheduled rotation after an approved interval;
- suspected exposure through logs, shell history, clipboard, screenshots, PR
  text, response files, journals, or artifacts;
- lost primary device or lost recovery envelope;
- operator handoff or role change;
- off-host provider/account compromise;
- repository corruption or unauthorized access evidence;
- restore drill failure caused by key/custody ambiguity.

Boundaries:

- revocation means no new backups with an exposed key; it does not by itself
  re-encrypt old archives;
- old backup retention must define whether old repository keys remain sealed
  until old snapshots expire or whether migration/re-encryption is authorized;
- emergency access requires a named trigger, operator log, isolated work area,
  no public copy/paste of secrets, and post-use rotation assessment;
- lost-key response is fail-closed: do not claim recoverability, do not prune
  the only remaining usable recovery point, and escalate to blocker direction;
- exposed-key response is fail-closed: freeze writes, preserve evidence without
  printing secrets, rotate before further operation, and assess old archive
  exposure;
- compromised repository response must assume attacker access to ciphertext and
  metadata, rotate credentials, check integrity, and avoid any claim that all
  metadata is hidden;
- operator handoff requires non-secret inventory, custody transfer proof,
  recovery drill, and revocation of retired access.

No public claim may be strengthened until restore drill, monitoring, backup
operation, deployment, and review evidence exist.

## No-secret artifact and operator-runbook boundary plan

NA-0356 no-secret boundaries:

- no key generation;
- no key upload;
- no passphrase collection;
- no secret material printed;
- no private key content inspection;
- no secret-dependent tests;
- no repository initialization;
- no remote target configuration;
- no backup/restore/deploy/rollback operation.

Future no-secret fixture strategy:

- use sentinel strings and fake fixtures only;
- assert that fixture secrets do not appear in generated logs, manifests, PR
  bodies, response files, or journals;
- keep CI independent of real repository passwords, private keys, provider
  credentials, auth headers, and remote endpoints;
- redact endpoint details in evidence prose and use short SHAs in narrative.

Future operator runbook requirements:

- shell-history and clipboard avoidance;
- file-permission checks for any local secret file or recovery envelope staging
  file;
- explicit source/exclusion handling so key material is not silently copied into
  normal backup snapshots;
- secret scanning of generated evidence before merge or handoff;
- emergency stop if any secret reaches logs/artifacts;
- documented break-glass trigger, accountable operator action, isolated restore
  target, cleanup, and post-use rotation assessment.

## Backup-plan impact and local-ops dependency decision

NA-0356 itself requires no backup-plan update because changed paths are
qsl-protocol governance/testplan/journal files under `/srv/qbuild/work`, and
the required response file stays under the existing Codex responses path.

Future key custody/recovery implementation requires backup-plan update and
exact local-ops authorization before any key material, recovery envelope,
secret file, remote credential, repository password path, source-list change,
script change, timer/fstab/system-service change, restore-drill artifact,
retention/purge policy, monitoring/logging artifact, backup operation, restore
operation, deploy operation, rollback operation, or public-claim mutation.

Local workflow-support and history-index coverage would materially reduce
friction but should follow the restore-drill prerequisite unless a later
directive reorders it. Current local evidence shows responses and requests were
present, directives and journals were absent, and the ops tree was not fully
proven as a covered source group. D132 remains present and must not be deleted
without explicit authorization.

## Restore drill/retention/monitoring dependency analysis

Key custody and recovery planning can proceed without handling secrets, but
target/tool implementation should not proceed yet. A restore-drill prerequisite
must define:

- dry-run restore boundary using fixture data and no real secrets;
- isolated real restore boundary for later local-ops execution;
- verification that proves recovery success without printing the secret;
- retention and old-key compatibility after rotation;
- purge rules that do not delete the only usable recovery point;
- failed recovery cleanup that never overwrites live data;
- alerting for missed backup/check/prune/restore and recovery verification
  failure;
- evidence artifacts, scan rules, and backup-plan impact.

This is why NA-0357 is selected as the restore-drill prerequisite plan rather
than direct target/tool implementation authorization.

## Public-ingress/timing/traffic-shape boundary

NA-0356 changes no public ingress, service deployment, website, README,
START_HERE, docs/public, qsl-server runtime, qsl-attachments runtime, or qshield
runtime behavior. Current evidence does not prove hidden attachment size,
hidden timing metadata, hidden traffic shape, hidden all metadata, or padding
that hides all metadata.

## External-review sensitivity

External review remains incomplete. Key custody/recovery planning is not
implementation evidence, not disaster-recovery completion, and not production
or public-internet proof. Stronger external claims require key custody evidence,
key recovery evidence, off-host backup evidence, restore-drill evidence,
service evidence, deployment evidence, monitoring/log evidence, rollback
evidence, and review evidence.

## Public claim boundary

NA-0356 introduces no claim of production readiness, public-internet readiness,
external-review completion, anonymity, metadata-free behavior, untraceable
behavior, hidden attachment size, hidden timing metadata, hidden traffic shape,
hidden all metadata, local continuity as complete disaster recovery, or
off-host encrypted backup completion. Website and public docs remain unchanged.

## Future validation/marker/verification plan

Future NA-0357 work should record or emit:

- `NA0357_KEY_CUSTODY_PREREQUISITE_PLAN_OK`
- `NA0357_KEY_RECOVERY_PREREQUISITE_PLAN_OK`
- `NA0357_CUSTODY_MODEL_DECISION_OK`
- `NA0357_RECOVERY_MODEL_DECISION_OK`
- `NA0357_KEY_ROTATION_BOUNDARY_OK`
- `NA0357_EMERGENCY_ACCESS_BOUNDARY_OK`
- `NA0357_INCIDENT_RESPONSE_BOUNDARY_OK`
- `NA0357_NO_KEY_GENERATION_OK`
- `NA0357_NO_PASSPHRASE_COLLECTION_OK`
- `NA0357_NO_SECRET_MATERIAL_OK`
- `NA0357_NO_PRIVATE_KEY_INSPECTION_OK`
- `NA0357_NO_SECRET_ARTIFACT_OK`
- `NA0357_BACKUP_PLAN_UPDATE_REQUIRED_OK`
- `NA0357_RESTORE_DRILL_DEPENDENCY_OK`
- `NA0357_NO_DISASTER_RECOVERY_COMPLETE_CLAIM_OK`
- `NA0357_NO_PRODUCTION_READY_CLAIM_OK`
- `NA0357_NO_PUBLIC_INTERNET_READY_CLAIM_OK`

If blocked, markers must name the exact missing restore-drill, custody,
recovery, local-ops, backup-plan, retention, monitoring, or evidence-hygiene
blocker.

## Workflow-support and history-index future work note

Read-only history availability improved confidence where present. Responses and
requests were available; directives and journals paths were absent. The
workflow-support request remains valid future local-ops work, especially:
qstart/qresume fast-forward, response-file writer, bounded polling helper,
machine-readable directive manifest, validation profiles, per-directive
allow-file, read-only source/authority helper, claim-boundary scanner,
directive/response/journal index, and backup coverage for directives, requests,
journals, and ops history folders.

NA-0356 does not implement those items.

## Selected successor

`NA-0357 -- Metadata Runtime Restore Drill Prerequisite Plan`

Rationale:

- custody/recovery can be planned without secrets, but recovery cannot be
  trusted until restore verification is specified;
- target/tool implementation would still require repository setup, key
  handling, local-ops mutation, backup-plan update, and future secret handling;
- the next smallest risk-reducing prerequisite is a no-secret restore-drill,
  retention, purge, monitoring, and verification-bundle plan.

## Rejected alternatives

- direct key generation or passphrase handling in NA-0356;
- direct off-host backup setup;
- direct target/tool implementation authorization before restore-drill planning;
- key custody/recovery implementation authorization before restore evidence
  boundaries are frozen;
- local-ops workflow/history index before restore-drill prerequisite planning;
- external review, website/public-claim audit, or position-paper work before
  off-host backup and restore evidence exists;
- claiming disaster recovery completion.

## Backup-plan impact statement

No NA-0356 backup-plan update is required. Future key custody/recovery,
restore-drill, or off-host implementation work requires backup-plan update and
exact local-ops authorization before any durable key, secret, source-list,
script, timer, fstab, system-service, target, repository, restore, retention,
purge, monitoring, backup, deploy, rollback, or public-claim mutation.

## Next recommendation

Close NA-0356 after required checks and public-safety pass, then restore
NA-0357 as the restore-drill prerequisite plan. Do not implement NA-0357 during
NA-0356 closeout.
