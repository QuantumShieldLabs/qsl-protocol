Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-01

# NA-0399 QSL Backup / Restore / Key Custody External Guidance Mapping Plan

Goals: G1, G2, G3, G4, G5

Directive: QSL-DIR-2026-06-01-219

## Executive Summary

NA-0399 maps official backup, restore, key-management, and disaster-recovery
guidance into QSL's current evidence boundaries. This is a governance and
claim-boundary plan only.

This lane does not run a real backup, run a real restore, create a restore
target, initialize an off-host repository, connect to a remote host, scan a host
key, mutate known_hosts, install backup tools, generate or inspect keys, collect
passphrases, handle credentials, create a recovery envelope, mutate backup
scripts/timers/fstab/source lists, change runtime code, change cryptography,
change dependencies, mutate workflows, mutate sibling repositories, update
website/public docs, or expand public claims.

Conservative outcome:

- QSL has same-host local continuity evidence through `/backup/qsl`, nightly
  snapshots, manifests, logs, and response-archive coverage.
- Same-host local continuity is not disaster recovery.
- QSL has no completed off-host encrypted backup evidence.
- QSL has no real key custody or key recovery evidence.
- QSL has no real off-host restore proof and no complete restore-drill claim.
- No current QSL evidence supports disaster-recovery-complete,
  off-host-backup-complete, restore-proven, key-custody-implemented, or
  key-recovery-implemented claims.

Selected successor:

`NA-0400 -- QSL External Review / Disclosure / Public Claim Readiness Plan`

Rationale: after backup/restore/key custody guidance mapping, the next
external-watch group is external review, disclosure, and public-claim readiness.
Public claims and public technical-paper timing depend on the cumulative
evidence boundaries already mapped across PQC, RFC/drafts, advisories,
code/crypto, metadata/privacy, and backup/restore/key custody.

## Live NA-0399 Scope

Live `NEXT_ACTIONS.md` records:

- Status: READY.
- Goals: G1, G2, G3, G4, G5.
- Objective: create a qsl-protocol governance plan that maps external backup,
  restore, key custody, key recovery, off-host backup, and disaster-recovery
  guidance into QSL's current evidence boundaries.
- Must protect: no runtime/service/protocol/crypto/dependency/workflow
  implementation unless future exact scope authorizes exact files; no secret
  handling; no backup script/timer/fstab mutation; no target setup; no real
  backup, restore, key-custody, key-recovery, off-host, remote, deploy,
  rollback, repository-init, host-key-scan, or credential operation; no
  public/readiness/privacy overclaim.
- Acceptance: READY_COUNT 1, READY NA-0399, NA-0398 DONE, D-0778 once,
  D-0779 once after closeout, D-0780 absent before this lane, NA-0399 not
  implemented by NA-0398 closeout, no forbidden mutation, public-safety
  required and green.

Allowed qsl-protocol mutation for this lane is limited to:

- `docs/governance/evidence/NA-0399_qsl_backup_restore_key_custody_external_guidance_mapping_plan.md`
- `tests/NA-0399_qsl_backup_restore_key_custody_external_guidance_mapping_plan_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Allowed source verification is targeted read-only verification of official
backup tool documentation, key-management guidance, contingency/recovery
guidance, and official backup/resilience guidance.

Allowed read-only local inspection is limited to qsl-protocol evidence, local
backup status, manifests/logs, response/history inventory, and read-only
`qsl-backup` preflight/list style commands.

Forbidden scope includes runtime/protocol/crypto changes, dependency/Cargo
changes, workflow changes, qsl-server/qsl-attachments changes, qshield runtime,
website/public docs, README/START_HERE, backup scripts/timers/fstab/source-list
mutation, real backup/restore/off-host/key operations, remote setup, host-key
scan, known_hosts mutation, repository initialization, secret handling, and any
public claim expansion.

Stop conditions include source verification unavailable for required categories,
scope conflict, multiple READY items, unsupported current claims, backup
guidance treated as implementation, code/crypto/dependency/workflow mutation,
secret handling, real backup/restore/off-host/key operations, or local
continuity being described as complete disaster recovery.

The future Project Goal / Operating Principles canon request remains a future
governance candidate only. It does not override NA-0399 or the selected NA-0400
successor.

## Inherited NA-0398 Rationale

NA-0398 selected NA-0399 after completing secure messaging and metadata privacy
claim-boundary planning. It established:

- qsl-protocol PR #1059 merged at `4859cdc524aa`.
- qsl-protocol PR #1060 merged at `eb9acaa1cb76`.
- READY_COUNT 1 and READY NA-0399.
- NA-0398 DONE.
- D-0778 and D-0779 present once.
- D-0780 absent before NA-0399.
- no metadata-free, anonymity, untraceable, hidden timing, hidden traffic-shape,
  hidden attachment-size, production-ready, public-internet-ready, or
  external-review-complete claim.
- qsl-server PR #56 remains bounded harness evidence only.
- qsl-attachments PR #37 remains service-local prerequisite evidence only.
- qshield demo evidence remains non-production evidence only.

Inherited successor rationale:

- NA-0392 identified backup/restore/key custody as a watch category.
- NA-0393 recorded backup/restore/key custody as a future queue candidate but
  not the immediate PQC/RFC/advisory/code/metadata priority.
- NA-0394 through NA-0398 narrowed adjacent public-claim boundaries.
- Backup/restore/key custody remains a major prerequisite before public paper
  or public readiness claims can expand.

## Authoritative Backup / Restore / Key Guidance Source Verification

Access date for all web sources in this section: 2026-06-01.

This is targeted source verification, not implementation authorization,
external review, tool installation, backup execution, restore execution, or
public-claim proof.

| Source | Authority / publisher | URL | Source tier | Classification | Relevance to QSL | Claim-boundary implication |
|---|---|---|---|---|---|---|
| Restic Documentation | restic authors / Read the Docs | https://restic.readthedocs.io/en/stable/ | Tier 1 official tool documentation | OFFICIAL_TOOL_DOCUMENTATION | Defines restic repository, backup, restore, check, prune, and key-management model. | Supports the class-level restic-style repository mapping only; QSL has no restic install or repository proof. |
| Restic: Preparing a new repository | restic authors | https://restic.readthedocs.io/en/stable/030_preparing_a_new_repo.html | Tier 1 official tool documentation | OFFICIAL_TOOL_DOCUMENTATION | Shows repository initialization, password/key requirement, local and SFTP repository modes. | Loss of repository password can make data irrecoverable; QSL cannot claim restore readiness without custody/recovery proof. |
| Restic: Working with repositories | restic authors | https://restic.readthedocs.io/en/stable/045_working_with_repos.html | Tier 1 official tool documentation | OFFICIAL_BACKUP_RESTORE_GUIDANCE | Covers repository consistency and data checks. | A future QSL backup lane needs check/read-data evidence before stronger repository-health claims. |
| Restic: Restoring from backup | restic authors | https://restic.readthedocs.io/en/stable/050_restore.html | Tier 1 official tool documentation | OFFICIAL_BACKUP_RESTORE_GUIDANCE | Covers snapshot restore, dry run, mount, and in-place restore caveats. | QSL dry-run/no-secret evidence is not a real restore; real restore needs target, key, repository, cleanup, and verification proof. |
| Restic: Removing backup snapshots | restic authors | https://restic.readthedocs.io/en/stable/060_forget.html | Tier 1 official tool documentation | OFFICIAL_BACKUP_RESTORE_GUIDANCE | Covers forget/prune and recommends repository checks after pruning. | QSL cannot claim retention/pruning/monitoring without real policy and check evidence. |
| Restic: Encryption / Manage repository keys | restic authors | https://restic.readthedocs.io/en/stable/070_encryption.html | Tier 1 official tool documentation | OFFICIAL_KEY_MANAGEMENT_GUIDANCE | Shows multiple repository keys/passwords. | QSL must distinguish simulated key metadata from real repository key custody. |
| Borg Documentation | BorgBackup project | https://borgbackup.readthedocs.io/en/stable/index.html | Tier 1 official tool documentation | OFFICIAL_TOOL_DOCUMENTATION | Defines Borg as deduplicating backup with authenticated encryption, SSH off-site repositories, checks, extract, prune, and mount. | Borg is relevant as an alternate tool class; QSL has no Borg install, repository, or restore evidence. |
| Borg extract | BorgBackup project | https://borgbackup.readthedocs.io/en/stable/usage/extract.html | Tier 1 official tool documentation | OFFICIAL_BACKUP_RESTORE_GUIDANCE | Documents restore/extract behavior and dry-run mode. | A future Borg lane would need dry-run and real extract proof; no current QSL claim. |
| Borg prune | BorgBackup project | https://borgbackup.readthedocs.io/en/master/usage/prune.html | Tier 1 official tool documentation | OFFICIAL_BACKUP_RESTORE_GUIDANCE | Documents retention/prune behavior and deletion risk. | Retention policy must be tested and monitored; QSL has no real off-host prune proof. |
| rclone Documentation | rclone project | https://rclone.org/docs/ | Tier 1 official tool documentation | OFFICIAL_TOOL_DOCUMENTATION | Defines remotes, copy/sync/check behavior, and interactive safety flags. | rclone is relevant to transport/sync and remote backends; QSL has no configured rclone remote. |
| rclone Crypt | rclone project | https://rclone.org/crypt/ | Tier 1 official tool documentation | OFFICIAL_TOOL_DOCUMENTATION | Defines client-side encryption wrapper and cryptcheck for encrypted remotes. | rclone crypt is an encryption layer, not a repository/restore proof by itself. |
| age README | age project / FiloSottile | https://github.com/FiloSottile/age | Tier 1 official project documentation | OFFICIAL_TOOL_DOCUMENTATION | Describes age as file encryption with explicit recipients and passphrase/SSH-key-oriented usage. | age is relevant for envelope/file encryption designs; QSL has no age install or real recipient/key custody proof. |
| Using the GNU Privacy Guard | GnuPG project | https://gnupg.org/documentation/manuals/gnupg/ | Tier 1 official tool documentation | OFFICIAL_TOOL_DOCUMENTATION | Official GnuPG manual and key-management sections. | GnuPG is installed locally, but installation is not key custody or encrypted backup proof. |
| OpenPGP Key Management | GnuPG project | https://gnupg.org/documentation/manuals/gnupg/OpenPGP-Key-Management.html | Tier 1 official key-management documentation | OFFICIAL_KEY_MANAGEMENT_GUIDANCE | Covers key generation, revocation certificates, passphrase changes, key editing, import/export concepts. | QSL has no real GnuPG key/revocation/recovery envelope evidence for backup reliance. |
| ssh_config(5) | OpenSSH / OpenBSD manual pages | https://man.openbsd.org/OpenBSD-current/man/ssh_config | Tier 1 official tool documentation | OFFICIAL_TOOL_DOCUMENTATION | Defines SSH client configuration, including host-key checking behavior. | QSL must not configure off-host SSH or weaken host-key verification without explicit future authorization. |
| ssh-keygen(1) | OpenSSH / OpenBSD manual pages | https://man.openbsd.org/cgi-bin/man.cgi/OpenBSD-current/man1/ssh-keygen.1 | Tier 1 official tool documentation | OFFICIAL_TOOL_DOCUMENTATION | Documents known_hosts lookup, hashing, key/certificate operations, and host/user certificate concepts. | QSL has no host identity proof; future host identity must be deliberate and no-secret. |
| NIST Key Management Guidelines / SP 800-57 Part 1 Rev. 5 | NIST CSRC | https://csrc.nist.gov/Projects/key-management/key-management-guidelines | Tier 1 official key-management guidance | OFFICIAL_KEY_MANAGEMENT_GUIDANCE | Provides general key-management guidance and best practices for cryptographic keying material. | QSL must not treat no-secret fixture metadata as real key lifecycle control. |
| NIST SP 800-34 Rev. 1 | NIST CSRC | https://csrc.nist.gov/pubs/sp/800/34/r1/upd1/final | Tier 1 official contingency guidance | OFFICIAL_DISASTER_RECOVERY_GUIDANCE | Guides contingency planning, recovery priorities, and information system contingency plans. | Same-host snapshots alone do not satisfy complete contingency/disaster recovery. |
| NIST SP 800-184 | NIST CSRC | https://csrc.nist.gov/pubs/sp/800/184/final | Tier 1 official recovery guidance | OFFICIAL_DISASTER_RECOVERY_GUIDANCE | Covers cybersecurity event recovery planning, playbooks, testing, metrics, and resilience. | QSL needs tested recovery scenarios and improvement loops before stronger DR claims. |
| NCSC: Offline backups in an online world | UK National Cyber Security Centre | https://www.ncsc.gov.uk/blog-post/offline-backups-in-an-online-world | Tier 1 official resilience guidance | OFFICIAL_BACKUP_RESTORE_GUIDANCE | Discusses offline/cold backup separation and backup plan protection. | QSL local same-host mounted backup is not offline/off-host separation. |

Citation gaps and uncertainty:

- Tool documentation describes possible secure backup models; it does not prove
  QSL has implemented any such model.
- CISA backup guidance was identified during source discovery, but the direct
  page/PDF open was not available through the web module during this run. NIST
  and NCSC official sources are sufficient for the resilience guidance mapping.
- Borg stable and master prune documentation versions differ; prune guidance is
  used only as category context, not as QSL implementation evidence.
- No source conflict changes the conservative QSL classification.

## Prior Backup / Restore / Key Evidence Intake

| Evidence source | Evidence present | QSL boundary |
|---|---|---|
| NA-0355 target/tool selection | Selected SSH/SFTP-compatible off-host host class and restic-style encrypted repository class; rejected direct setup. | Class-level planning only; no target, repository, key, or restore proof. |
| NA-0359 restore dry-run harness | qsl-protocol no-secret restore-drill dry-run fixture and fail-closed harness. | DRY_RUN_ONLY; not real restore execution. |
| NA-0361 key custody/recovery no-secret harness | Prior traceability and NA-0364/0366/0375 evidence record a no-secret key custody/recovery fixture and harness. | NO_SECRET_HARNESS_ONLY; not real key custody, recovery, passphrase handling, or recovery envelope. |
| NA-0363 off-host target/tool no-secret harness | Prior evidence records simulated SSH/SFTP target, restic-style repository metadata, retention/check/prune/restore relationships, and no operations. | NO_SECRET_HARNESS_ONLY; not real off-host setup, host identity, tool execution, or repository. |
| NA-0365 isolated restore no-secret harness | Prior evidence records simulated isolated restore target, manifest/checksum restore relationships, cleanup, monitoring/runbook metadata. | NO_SECRET_HARNESS_ONLY; no real restore target, mount, copy, or restored artifact. |
| NA-0366 through NA-0375 blocker chain | Real target/tool implementation blocked; operator response absent; required stop recorded. | BLOCKED_PENDING_OPERATOR_INPUT, BLOCKED_PENDING_OFF_HOST_TARGET, BLOCKED_PENDING_KEY_CUSTODY, BLOCKED_PENDING_KEY_RECOVERY, BLOCKED_PENDING_REAL_RESTORE. |
| NA-0384 through NA-0388 local history/response archive | Response writer and response history catalog helpers exist; real archive smoke and metadata catalog proof remain local and caveated. | Local history support evidence only; no off-host or DR proof. |
| Local backup status | `/backup/qsl` mounted; daily snapshots/manifests/logs present through 2026-06-01; qsl-backup preflight/list read-only passed. | LOCAL_CONTINUITY_ONLY; same-host continuity only. |
| qsl-server PR #56 | Merged at `d40e6003fdf0`; latest merge CI green. | Service-local harness evidence only, not public-internet or DR proof. |
| qsl-attachments PR #37 | Merged at `96b9352bd63`; latest merge CI green. | Service-local prerequisite evidence only, not production/off-host proof. |

Internal claim allowed:

- QSL has same-host local continuity evidence.
- QSL has no-secret backup/restore/key/off-host harnesses.
- QSL has a selected off-host target/tool class and operator-input blocker.
- QSL has response archive helper evidence and same-host response archive
  coverage.

Public or readiness claim forbidden:

- disaster recovery complete.
- off-host backup complete.
- restore proven or restore drill complete.
- key custody implemented.
- key recovery implemented.
- recovery envelope ready.
- target configured.
- host identity verified.
- production ready, public-internet ready, or externally reviewed.

## Read-Only Local Backup / Tool / Key / Restore Inventory

Read-only local checks on 2026-06-01 found:

- `/backup/qsl` mounted from a local ext4 disk.
- `/backup/qsl` capacity: 916G total, 25G used, 883G available, 3% used.
- `/srv/qbuild` capacity: 468G total, 151G used, 294G available, 34% used.
- Latest listed manifest: `/backup/qsl/manifests/daily-20260601T023339-0500.manifest.txt`.
- Latest listed log: `/backup/qsl/logs/daily-20260601T023339-0500.log`.
- `qsl-backup preflight` reported target mounted and daily sources present.
- `qsl-backup list` listed daily snapshots through `daily-20260601T023339-0500`
  and two checkpoint snapshots from 2026-05-17.
- Tool detection: `gpg`, `ssh`, `rsync`, and `qsl-backup` present; `restic`,
  `borg`, `rclone`, and `age` absent.
- Response archive present, including D205 through D218 final response files.
- NA-0386 helper-created smoke file hash matched
  `2d06eb23330873576f813d875dadb08b5b26c019138f9cef77af27b8d20b5e40`.
- `/home/victor/work/qsl/codex/requests` present with two request files.
- `/home/victor/work/qsl/codex/directives` absent or empty.
- `/home/victor/work/qsl/codex/journals` absent or empty.
- `/home/victor/work/qsl/codex/ops/backup` present with local backup status,
  installed-script copies, and one restore-drill record.

No real backup, real restore, off-host connection, host-key scan, repository
initialization, key operation, passphrase/credential handling, or backup
configuration mutation occurred in NA-0399.

## Same-Host Local Continuity Map

Current evidence:

- Mounted local target `/backup/qsl`.
- Nightly local snapshots and manifests/logs through 2026-06-01.
- `qsl-backup` read-only preflight/list passed.
- Backup status document from 2026-05-17 says local continuity backup is
  mounted, writable, scheduled, and same-host only.
- Prior non-destructive local restore drill restored selected files under the
  backup volume and compared hashes.

Classification:

- `LOCAL_CONTINUITY_ONLY`
- `IMPLEMENTED_BUT_EVIDENCE_INCOMPLETE` for local continuity coverage
- `CLAIM_BOUNDARY_REQUIRED`

Claim boundary:

- Allowed: same-host local continuity exists.
- Forbidden: same-host continuity is complete disaster recovery.

Missing evidence:

- off-host target.
- encrypted repository.
- key custody and key recovery.
- real isolated restore outside the source host.
- monitoring/alerting and runbook evidence.

## Off-Host Encrypted Backup Map

Current evidence:

- NA-0355 selected an SSH/SFTP-compatible off-host host class and restic-style
  encrypted repository class.
- NA-0363 simulated target/tool metadata with no secret and no real operation.
- NA-0366 found real target/tool implementation blocked.
- NA-0369 through NA-0375 established absent operator response and required stop.
- No target label, host identity, credential boundary, capacity/retention,
  monitoring/runbook, repository, or connection evidence exists.

Classification:

- `BLOCKED_PENDING_OPERATOR_INPUT`
- `BLOCKED_PENDING_OFF_HOST_TARGET`
- `NO_SECRET_HARNESS_ONLY`
- `NOT_IMPLEMENTED`

Claim boundary:

- Allowed: off-host target/tool class selected and blocked pending operator input.
- Forbidden: off-host encrypted backup is complete.

Future evidence needed:

- deliberate no-secret operator target response.
- host identity evidence source and verification boundary.
- credential placeholder and no-secret boundary.
- approved target capacity/retention/monitoring/runbook.
- explicit real-operation authorization.

## Backup Tool / Repository Model Map

Current evidence:

- NA-0355 recommended a restic-style repository class.
- `restic`, `borg`, `rclone`, and `age` are absent on this host.
- `gpg`, `ssh`, and `rsync` are present, but that does not establish backup
  repository or key custody.
- No restic/Borg/rclone repository is initialized.
- No repository check, prune, retention, or real restore proof exists for an
  off-host encrypted repository.

Classification:

- `GOVERNANCE_PLANNED`
- `NO_SECRET_HARNESS_ONLY`
- `NOT_IMPLEMENTED`
- `BLOCKED_PENDING_BACKUP_PLAN_REVIEW`

Claim boundary:

- Allowed: tool/repository class mapping exists.
- Forbidden: repository configured, backup ready, restore ready, or retention
  proven.

Future evidence needed:

- authorized tool install/version decision if needed.
- repository initialization boundary.
- key custody/recovery before reliance.
- check, prune, retention, monitoring, and restore proof.

## Key Custody Map

Current evidence:

- NA-0361 no-secret harness evidence exists through prior traceability and
  inherited NA-0364/0366/0375 summaries.
- No real repository password, private key, recipient key, hardware token,
  split-secret arrangement, accountable custody owner, or passphrase process is
  recorded.
- NIST SP 800-57 guidance requires real protection and lifecycle treatment for
  cryptographic keying material; no-secret fixtures are not that lifecycle.

Classification:

- `NO_SECRET_HARNESS_ONLY`
- `BLOCKED_PENDING_KEY_CUSTODY`
- `NOT_IMPLEMENTED`

Claim boundary:

- Allowed: simulated no-secret custody metadata exists.
- Forbidden: key custody implemented or key custody ready.

Future evidence needed:

- exact no-secret authorization for custody design.
- accountable role and custody medium.
- backup-key lifecycle, rotation, revocation, and access control.
- evidence wording that never copies secret material into governance docs.

## Key Recovery Map

Current evidence:

- NA-0361 simulated key recovery metadata.
- No recovery envelope content exists.
- No emergency access procedure, old archive compatibility proof with real keys,
  lost/exposed key procedure, or recovery exercise exists.
- No real key or passphrase was handled in NA-0399.

Classification:

- `NO_SECRET_HARNESS_ONLY`
- `BLOCKED_PENDING_KEY_RECOVERY`
- `NOT_IMPLEMENTED`

Claim boundary:

- Allowed: no-secret recovery harness evidence exists.
- Forbidden: key recovery implemented, key recovery ready, or recovery envelope
  ready.

Future evidence needed:

- separate recovery-envelope authorization.
- break-glass trigger and accountable operator.
- recovery test that preserves no-secret evidence boundaries.
- old archive recovery criteria before reliance.

## Restore Drill Map

Current evidence:

- NA-0359 dry-run restore harness exists.
- NA-0365 isolated restore no-secret harness exists.
- Local 2026-05-17 backup drill restored selected files under `/backup/qsl`
  with matching hashes.
- No real off-host restore was run.
- No new restore target was created in NA-0399.
- No mount/copy/restore operation occurred in NA-0399.

Classification:

- `DRY_RUN_ONLY`
- `NO_SECRET_HARNESS_ONLY`
- `LOCAL_CONTINUITY_ONLY` for the 2026-05-17 local drill
- `BLOCKED_PENDING_REAL_RESTORE`

Claim boundary:

- Allowed: dry-run/no-secret restore harness and local same-host selected-file
  drill evidence.
- Forbidden: restore proven, restore drill complete, real restore complete, or
  disaster recovery complete.

Future evidence needed:

- authorized isolated restore target.
- off-host repository and key custody/recovery.
- restored artifact verification.
- cleanup, monitoring, and runbook proof.

## Retention / Pruning / Monitoring / Runbook Map

Current evidence:

- Local `qsl-backup` records daily keep/checkpoint keep constants in the script,
  and the local backup status describes a systemd timer.
- NA-0363 simulated retention/purge metadata for target/tool harnessing.
- Restic and Borg documentation show prune/check concepts that would need real
  evidence in a future lane.
- No real off-host retention, prune, monitor, alert, emergency stop, or operator
  runbook exists.

Classification:

- `LOCAL_CONTINUITY_ONLY`
- `NO_SECRET_HARNESS_ONLY`
- `GOVERNANCE_PLANNED`
- `BLOCKED_PENDING_BACKUP_PLAN_REVIEW`

Claim boundary:

- Allowed: local same-host retention script intent and simulated no-secret
  retention metadata.
- Forbidden: off-host retention/pruning/monitoring proven.

Future evidence needed:

- policy for retention/prune schedule.
- check-after-prune and repository-health evidence.
- alert destination class with no-secret handling.
- operator runbook and incident stop procedure.

## Disaster Recovery Map

Disaster recovery requires more than local snapshots. Relevant axes:

- local continuity: present, same-host only.
- off-host backup: not implemented.
- key custody: not implemented.
- key recovery: not implemented.
- real restore proof: not present for off-host/DR.
- monitoring/runbook: incomplete.
- external review: not complete.

Classification:

- `LOCAL_CONTINUITY_ONLY`
- `CLAIM_BOUNDARY_REQUIRED`
- `BLOCKED_PENDING_OFF_HOST_TARGET`
- `BLOCKED_PENDING_KEY_CUSTODY`
- `BLOCKED_PENDING_KEY_RECOVERY`
- `BLOCKED_PENDING_REAL_RESTORE`
- `BLOCKED_PENDING_EXTERNAL_REVIEW`

Claim boundary:

- Allowed: QSL has local continuity evidence and future-gated DR prerequisites.
- Forbidden: disaster recovery complete.

Future evidence needed:

- off-host separation.
- key custody and recovery proof.
- restore exercise and verification.
- recovery playbook/testing/metrics aligned with NIST recovery guidance.

## Operator Response / Target Identity Blocker Map

Current evidence:

- NA-0369 created a no-secret operator action packet.
- NA-0371 created a no-secret operator response collection request.
- NA-0372 through NA-0375 found no deliberate no-secret operator response.
- NA-0375 recorded required stop.

Missing non-secret input:

- target label/class/contact.
- trust boundary.
- host identity evidence source and fingerprint format class.
- credential placeholder only.
- capacity/quota/retention.
- monitoring/alerting and runbook owner.
- emergency stop contact.
- public-claim acknowledgement.
- no-secret affirmation.

Classification:

- `BLOCKED_PENDING_OPERATOR_INPUT`
- `BLOCKED_PENDING_OFF_HOST_TARGET`
- `CLAIM_BOUNDARY_REQUIRED`

Claim boundary:

- Allowed: operator response required.
- Forbidden: target configured or host identity verified.

## Local History / Response Archive / Evidence Backup Map

Current evidence:

- Response archive present under `/home/victor/work/qsl/codex/responses`.
- D205 smoke file hash matched expected value.
- D205 final through D218 final response files are present.
- Requests directory present with two files.
- Directives and journals history directories absent or empty.
- Ops backup directory present with local backup status and restore-drill docs.
- NA-0386 response writer real-archive smoke and NA-0388 metadata-only catalog
  evidence remain bounded local-ops evidence.

Classification:

- `LOCAL_CONTINUITY_ONLY`
- `IMPLEMENTED_BUT_EVIDENCE_INCOMPLETE` for response archive helper behavior
- `GOVERNANCE_PLANNED` for durable catalog/index/report coverage

Claim boundary:

- Allowed: local response archive/history evidence exists and same-host backup
  coverage is visible.
- Forbidden: response archive disaster recovery or off-host recovery complete.

Future evidence needed:

- durable metadata-only catalog/index authorization if needed.
- directive/journal coverage decision.
- off-host backup and restore evidence for response/history roots.

## Backup / Restore / Key Custody Evidence Matrix

| Axis | External source basis | QSL current evidence | Evidence class | Confidence | Claim allowed? | Claim forbidden? | Missing evidence | Future lane | Priority |
|---|---|---|---|---|---|---|---|---|---|
| Same-host local continuity | NCSC/NIST backup and recovery guidance; local qsl-backup docs | `/backup/qsl` mounted; daily snapshots/logs/manifests; preflight/list pass | LOCAL_CONTINUITY_ONLY | High | Yes, same-host continuity | Complete disaster recovery | off-host separation, recovery playbook, real DR restore | Backup monitoring / retention / runbook | High |
| Off-host encrypted backup | restic/Borg/rclone/OpenSSH guidance | target/tool class selected; operator response absent | BLOCKED_PENDING_OFF_HOST_TARGET | High | Class selected, blocked | Off-host backup complete | target, host identity, credential boundary, repository | Off-host operator response refresh | Critical |
| Backup tool / repository model | restic/Borg/rclone/age/GnuPG docs | restic-style class selected; restic/borg/rclone/age absent | GOVERNANCE_PLANNED | High | Tool class mapping | Repository configured | install/version, init, check, prune, restore | Real off-host target/tool authorization | High |
| Key custody | NIST SP 800-57; restic/GnuPG/age docs | no-secret harness only | NO_SECRET_HARNESS_ONLY | High | simulated metadata only | key custody implemented | custody owner, medium, lifecycle, access controls | Real key custody authorization | Critical |
| Key recovery | NIST SP 800-57; GnuPG revocation/recovery concepts | no-secret harness only | NO_SECRET_HARNESS_ONLY | High | simulated recovery metadata | key recovery implemented | recovery envelope, break-glass, recovery exercise | Real key recovery envelope authorization | Critical |
| Restore dry-run | restic/Borg restore dry-run guidance | NA-0359 dry-run harness | DRY_RUN_ONLY | High | dry-run harness | restore proven | real repository, key, target, verification | Real isolated restore authorization | High |
| Isolated restore no-secret | NIST recovery testing; restic restore guidance | NA-0365 simulated isolated restore | NO_SECRET_HARNESS_ONLY | High | simulated isolated restore metadata | real restore complete | target creation/mount/copy, cleanup | Real isolated restore authorization | High |
| Real restore | NIST SP 800-34/184; restic/Borg restore docs | local selected-file drill only; no off-host real restore | BLOCKED_PENDING_REAL_RESTORE | High | local selected-file same-host drill with caveat | restore proven / DR complete | off-host repo, key, restore target, hash proof | Real isolated restore authorization | Critical |
| Retention/pruning | restic forget/prune/check; Borg prune; qsl-backup script | local retention constants; simulated no-secret retention | LOCAL_CONTINUITY_ONLY | Medium | local script intent with caveat | off-host retention proven | policy, prune/check run, alerting | Backup monitoring / retention / runbook | High |
| Monitoring/alerting | NIST recovery planning; tool check guidance | simulated monitoring metadata; no real off-host monitor | GOVERNANCE_PLANNED | Medium | future-gated | monitoring complete | alert class, runbook, failures | Backup monitoring / retention / runbook | Medium |
| Runbook/operator response | NIST SP 800-34/184 | NA-0369/0371 requests; NA-0375 stop | BLOCKED_PENDING_OPERATOR_INPUT | High | operator response required | target configured | non-secret target/host/runbook fields | Off-host operator response refresh | Critical |
| Disaster recovery | NIST SP 800-34/184; NCSC offline separation | local continuity only plus blocked prerequisites | CLAIM_BOUNDARY_REQUIRED | High | DR is future-gated | disaster recovery complete | off-host, key, restore, monitoring, playbook | External review / public claim readiness | Critical |
| Local history/evidence backup | local qsl-backup status; NA-0384-0388 | response archive present; same-host backup coverage | LOCAL_CONTINUITY_ONLY | High | local evidence archive exists | off-host evidence recovery complete | durable catalog, off-host backup, restore proof | Response/directive/journal coverage review | Medium |

## Claim Language Policy

Allowed only with caveats:

- same-host local continuity.
- no-secret harness.
- dry-run restore harness.
- simulated isolated restore harness.
- off-host target/tool class selected.
- operator response required.
- evidence incomplete.
- future gated.

Forbidden unless exact future evidence exists:

- disaster recovery complete.
- off-host backup complete.
- restore proven.
- restore drill complete.
- real restore complete.
- key custody implemented.
- key recovery implemented.
- recovery envelope ready.
- target configured.
- host identity verified.
- production ready.
- public internet ready.
- externally reviewed.
- backup ready, restore ready, key custody ready, or key recovery ready.

## Future Queue Candidates

| Candidate | Source/evidence basis | Why next / why not next | Likely allowed scope | Likely forbidden scope | Public-claim implication |
|---|---|---|---|---|---|
| External Review / Disclosure / Public Claim Readiness Plan | NA-0394-0399 cumulative claim-boundary maps | Selected next; public claim readiness now needs consolidated external-review/disclosure policy. | governance evidence/testplan/decisions/traceability/journal; targeted source verification if authorized | runtime, dependencies, workflows, public docs unless authorized | Keeps public claims bounded before any public paper. |
| Backup / Restore / Key Custody Critical Evidence Gap Resolution | NA-0399 blocker matrix | Not selected because no new immediate critical blocker was discovered beyond known operator/key/restore gaps. | exact blocker evidence only | real operations unless explicitly authorized | Would reduce backup claim risk before public work if Director selects it. |
| Off-Host Operator Response Refresh / Required Input Plan | NA-0375 required stop | Not selected because another blind intake may repeat absence. | no-secret request/response handling | target setup, credentials, host-key scan | Keeps off-host claims blocked. |
| Real Off-Host Target / Host Identity Authorization Plan | NA-0355/0366/0375 | Not next until operator supplies non-secret target fields. | authorization evidence | connection, scan, setup unless exact scope | Could unblock later off-host proof. |
| Real Key Custody / Recovery Envelope Authorization Plan | NIST SP 800-57; NA-0361 | Not next because it requires explicit secret/no-secret boundary authorization. | custody/recovery plan and no-secret evidence | real keys/passphrases unless exact scope | Required before key claims. |
| Real Isolated Restore Authorization Plan | restic/Borg/NIST recovery guidance; NA-0359/0365 | Not next because off-host target and keys are absent. | restore authorization/runbook | restore target creation/mount/copy unless exact scope | Required before restore claims. |
| Backup Monitoring / Retention / Runbook Plan | restic/Borg/NIST guidance; qsl-backup status | Not next because off-host/key blockers are more fundamental. | governance/runbook planning | backup config mutation unless exact scope | Improves local continuity evidence. |
| Response/Directive/Journal Backup Coverage Review Plan | NA-0384-0388; local archive inventory | Not next because public-claim readiness is broader. | local-history coverage planning | archive mutation or durable catalog unless exact scope | Keeps evidence archive caveated. |
| Project Goal / Operating Principles Canon Authorization Plan | Human carry-forward request | Not next because NA-0400 external review/readiness is the selected successor. | governance canon authorization | changing queue priorities without directive | Useful after public-claim readiness is bounded. |
| Director State Index Authorization Plan | Local-ops residual | Not next; useful but lower priority. | governance/local-ops planning | durable state index unless authorized | Helps continuity but not public claim readiness by itself. |
| Public Technical Position Paper Evidence Prerequisite Plan | Future public paper gate | Not next because external review/disclosure/readiness must be mapped first. | prerequisite map | writing public paper or website | Prevents premature public claims. |

## Selected Successor

Selected successor:

`NA-0400 -- QSL External Review / Disclosure / Public Claim Readiness Plan`

Rejected alternatives:

- `NA-0400 -- QSL Backup / Restore / Key Custody Critical Evidence Gap Resolution`:
  rejected because NA-0399 found known, already bounded gaps rather than a new
  immediate critical blocker requiring reordering.
- `NA-0400 -- QSL Backup / Restore / Key Custody Source Verification Blocker
  Resolution`: rejected because official-source verification succeeded with
  caveats.

## Future Path / Scope Bundle

Future NA-0400 allowed paths if the selected successor is implemented later:

- `docs/governance/evidence/NA-0400_qsl_external_review_disclosure_public_claim_readiness_plan.md`
- `tests/NA-0400_qsl_external_review_disclosure_public_claim_readiness_plan_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Future forbidden scope:

- dependency changes.
- Cargo.toml/Cargo.lock changes.
- runtime code.
- crypto implementation.
- qsc/qsp/qsl implementation.
- qshield runtime.
- qsl-server.
- qsl-attachments.
- workflows.
- public docs/website unless explicitly authorized.
- backup scripts/timers/fstab.
- response archives.
- real backup/restore/key/off-host operations.
- external claims.

Future NA-0400 may use targeted web only to cite/verify external review,
disclosure, and public-claim readiness sources if live scope authorizes.

## Public Claim / External Review / Website Boundary

- Backup/restore/key guidance mapping is not implementation.
- Same-host continuity is not disaster recovery.
- No off-host-backup-complete claim is supported.
- No restore-proven claim is supported.
- No key-custody-implemented claim is supported.
- No key-recovery-implemented claim is supported.
- No website/public docs update is authorized here.
- No production claim is supported.
- No public-internet readiness claim is supported.
- No bug-free or perfect-crypto claim is supported.
- Source discovery is not external review.

## Future Validation / Marker Plan

Future NA-0400 markers if the selected successor is implemented later:

- `NA0400_EXTERNAL_REVIEW_READINESS_PLAN_OK`
- `NA0400_DISCLOSURE_POLICY_REFERENCE_OK`
- `NA0400_PUBLIC_CLAIM_BOUNDARY_OK`
- `NA0400_NO_EXTERNAL_REVIEW_COMPLETE_CLAIM_OK`
- `NA0400_NO_PRODUCTION_READY_CLAIM_OK`
- `NA0400_NO_PUBLIC_INTERNET_READY_CLAIM_OK`
- `NA0400_NO_METADATA_FREE_CLAIM_OK`
- `NA0400_NO_ANONYMITY_CLAIM_OK`
- `NA0400_NO_UNTRACEABLE_CLAIM_OK`
- `NA0400_NO_OFF_HOST_BACKUP_COMPLETE_CLAIM_OK`
- `NA0400_NO_DISASTER_RECOVERY_COMPLETE_CLAIM_OK`
- `NA0400_NO_RUNTIME_CHANGE_OK`
- `NA0400_NO_CRYPTO_IMPLEMENTATION_CHANGE_OK`
- `NA0400_NO_DEPENDENCY_CHANGE_OK`
- `NA0400_NO_WORKFLOW_CHANGE_OK`
- `NA0400_NO_SECRET_MATERIAL_OK`

## Future Project Goal / Operating Principles Canon Carry-Forward Note

The human-requested future project-goal / operating-principles canon artifact
remains a future governance candidate:

`QSL Project Goal and Operating Principles Canon Authorization Plan`

It should record QSL's north star, security before speed, evidence over vibes,
code and crypto excellence, no public overclaiming, one-READY queue discipline,
routine audits as operating rhythm, external awareness without hype, public
technical paper timing, shorter/safer future directives, and
Director/Codex/human roles. It is not implemented by NA-0399 and is not
promoted over the selected NA-0400 successor.

## Rejected Alternatives

- Real backup/restore/key/off-host implementation now: rejected as out of
  scope and unsafe without operator/key/restore authorization.
- Changing backup scripts now: rejected as out of scope.
- Changing qsl-server or qsl-attachments now: rejected as out of scope.
- Writing public docs now: rejected as out of scope.
- Starting the public technical paper now: rejected because evidence and review
  boundaries remain incomplete.
- Claiming disaster recovery or off-host backup completion now: rejected as
  unsupported.

## Backup-Plan Impact Statement

No NA-0399 backup-plan update is required because durable changes are limited to
tracked qsl-protocol governance, evidence, testplan, traceability, and journal
files.

Future durable backup/restore/key reports, local history catalogs, response
archive mutations, real operator responses, real target connection, host
identity capture, credential handling, repository/tool setup, key
custody/recovery, monitoring artifacts, backup source-list changes, backup
script/timer/fstab changes, real backup, real restore, deploy, rollback, or
public-claim mutation require separate backup-impact review.

## Next Recommendation

After NA-0399 merges and closes out, run:

`NA-0400 -- QSL External Review / Disclosure / Public Claim Readiness Plan`

Keep backup/restore/key custody residuals as explicit public-claim blockers and
future queue candidates. Do not implement backup/restore/key/off-host operations
without an exact future directive and no-secret/secret-boundary authorization.

## Source List

| Source | Publisher | URL | Access date | Source tier | Classification | Relevance |
|---|---|---|---|---|---|---|
| Restic Documentation | restic authors | https://restic.readthedocs.io/en/stable/ | 2026-06-01 | Tier 1 | OFFICIAL_TOOL_DOCUMENTATION | restic repository/restore/key/check/prune model |
| Restic Preparing a New Repository | restic authors | https://restic.readthedocs.io/en/stable/030_preparing_a_new_repo.html | 2026-06-01 | Tier 1 | OFFICIAL_TOOL_DOCUMENTATION | repository initialization, SFTP, password/key caveat |
| Restic Working with Repositories | restic authors | https://restic.readthedocs.io/en/stable/045_working_with_repos.html | 2026-06-01 | Tier 1 | OFFICIAL_BACKUP_RESTORE_GUIDANCE | repository integrity/check requirements |
| Restic Restoring from Backup | restic authors | https://restic.readthedocs.io/en/stable/050_restore.html | 2026-06-01 | Tier 1 | OFFICIAL_BACKUP_RESTORE_GUIDANCE | restore and dry-run model |
| Restic Removing Backup Snapshots | restic authors | https://restic.readthedocs.io/en/stable/060_forget.html | 2026-06-01 | Tier 1 | OFFICIAL_BACKUP_RESTORE_GUIDANCE | retention/prune/check caveats |
| Restic Encryption | restic authors | https://restic.readthedocs.io/en/stable/070_encryption.html | 2026-06-01 | Tier 1 | OFFICIAL_KEY_MANAGEMENT_GUIDANCE | repository key/password management |
| Borg Documentation | BorgBackup project | https://borgbackup.readthedocs.io/en/stable/index.html | 2026-06-01 | Tier 1 | OFFICIAL_TOOL_DOCUMENTATION | alternate encrypted backup repository model |
| Borg extract | BorgBackup project | https://borgbackup.readthedocs.io/en/stable/usage/extract.html | 2026-06-01 | Tier 1 | OFFICIAL_BACKUP_RESTORE_GUIDANCE | extract/dry-run restore behavior |
| Borg prune | BorgBackup project | https://borgbackup.readthedocs.io/en/master/usage/prune.html | 2026-06-01 | Tier 1 | OFFICIAL_BACKUP_RESTORE_GUIDANCE | retention/prune model |
| rclone Documentation | rclone project | https://rclone.org/docs/ | 2026-06-01 | Tier 1 | OFFICIAL_TOOL_DOCUMENTATION | remote copy/sync/check behavior |
| rclone Crypt | rclone project | https://rclone.org/crypt/ | 2026-06-01 | Tier 1 | OFFICIAL_TOOL_DOCUMENTATION | encrypted remote wrapper and cryptcheck |
| age README | age project | https://github.com/FiloSottile/age | 2026-06-01 | Tier 1 | OFFICIAL_TOOL_DOCUMENTATION | file encryption / recipient model context |
| GnuPG Manual | GnuPG project | https://gnupg.org/documentation/manuals/gnupg/ | 2026-06-01 | Tier 1 | OFFICIAL_TOOL_DOCUMENTATION | installed tool and key-management context |
| OpenPGP Key Management | GnuPG project | https://gnupg.org/documentation/manuals/gnupg/OpenPGP-Key-Management.html | 2026-06-01 | Tier 1 | OFFICIAL_KEY_MANAGEMENT_GUIDANCE | generation/revocation/import/export context |
| ssh_config(5) | OpenSSH / OpenBSD | https://man.openbsd.org/OpenBSD-current/man/ssh_config | 2026-06-01 | Tier 1 | OFFICIAL_TOOL_DOCUMENTATION | SSH host-key checking context |
| ssh-keygen(1) | OpenSSH / OpenBSD | https://man.openbsd.org/cgi-bin/man.cgi/OpenBSD-current/man1/ssh-keygen.1 | 2026-06-01 | Tier 1 | OFFICIAL_TOOL_DOCUMENTATION | known_hosts and key/certificate context |
| NIST Key Management Guidelines | NIST CSRC | https://csrc.nist.gov/Projects/key-management/key-management-guidelines | 2026-06-01 | Tier 1 | OFFICIAL_KEY_MANAGEMENT_GUIDANCE | key lifecycle and protection guidance |
| NIST SP 800-34 Rev. 1 | NIST CSRC | https://csrc.nist.gov/pubs/sp/800/34/r1/upd1/final | 2026-06-01 | Tier 1 | OFFICIAL_DISASTER_RECOVERY_GUIDANCE | contingency planning and DR planning context |
| NIST SP 800-184 | NIST CSRC | https://csrc.nist.gov/pubs/sp/800/184/final | 2026-06-01 | Tier 1 | OFFICIAL_DISASTER_RECOVERY_GUIDANCE | recovery planning/playbook/testing context |
| NCSC Offline Backups in an Online World | UK NCSC | https://www.ncsc.gov.uk/blog-post/offline-backups-in-an-online-world | 2026-06-01 | Tier 1 | OFFICIAL_BACKUP_RESTORE_GUIDANCE | offline/off-host separation and backup-plan context |
| NA-0355 target/tool selection | QSL local evidence | `docs/governance/evidence/NA-0355_metadata_runtime_off_host_encrypted_backup_target_tool_selection_plan.md` | 2026-06-01 | Local | LOCAL_QSL_EVIDENCE | target/tool class selection |
| NA-0359 restore dry-run | QSL local evidence | `docs/governance/evidence/NA-0359_metadata_runtime_restore_drill_dry_run_harness.md` | 2026-06-01 | Local | LOCAL_QSL_EVIDENCE | dry-run restore harness |
| NA-0366 through NA-0375 blocker chain | QSL local evidence | `docs/governance/evidence/NA-0375_metadata_runtime_off_host_backup_operator_response_required_stop_await_input.md` | 2026-06-01 | Local | LOCAL_QSL_EVIDENCE | operator-input/off-host blocker |
| NA-0384 through NA-0388 local history evidence | QSL local evidence | `docs/governance/evidence/NA-0388_qsl_local_ops_response_archive_index_history_catalog_harness.md` | 2026-06-01 | Local | LOCAL_QSL_EVIDENCE | response archive/local history posture |
