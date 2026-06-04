Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-04

# NA-0416 QSL Backup Log Code 23 Temp Rollback Subtree Cleanup Permission Remediation Authorization Plan

Goals: G4

## Executive Summary

NA-0416 is an authorization lane for the scheduled same-host qsl-backup log
code 23 warning caused by the NA-0407 rollback subtree. It does not implement
cleanup or permission remediation.

Current classification:

`CODE23_STILL_ACTIVE_NA0407_ROLLBACK_SUBTREE`

The latest scheduled log and manifest remain the 2026-06-04 daily pair. The
log still has exactly one rsync code 23 result, and that result still names the
NA-0407 rollback directory as permission denied. The matching manifest still
includes Codex ops exactly once.

Selected successor:

`NA-0417 -- QSL Backup Log Code 23 Root Operator Cleanup / Permission Remediation Packet Plan`

Recommended option:

Option 2 is the preferred direction to plan next: preserve rollback evidence
first, then have a future root-operator packet adjust rollback evidence
permissions or ownership only within exact path scope. Deletion, relocation, or
qsl-backup exclusion are rejected for the next lane because they are more
destructive or would change backup source behavior before preservation is
complete.

NA-0416 did not run backup, did not run restore, did not run sudo, did not
mutate qsl-backup, did not mutate the temp rollback subtree, did not mutate
backup status or plan files, made no public readiness claim, made no off-host
backup claim, made no disaster recovery claim, made no restore-proof claim, made
no backup-complete claim, made no external-review claim, and made no public
technical-paper claim.

## Live NA-0416 Scope

Live `NEXT_ACTIONS.md` showed one READY item:

- `NA-0416 -- QSL Backup Log Code 23 Temp Rollback Subtree Cleanup / Permission Remediation Authorization Plan`
- status: READY
- goals: G1, G2, G3, G4, G5

Allowed qsl-protocol mutation paths for this evidence PR are exactly this
evidence file, the matching NA-0416 testplan, `DECISIONS.md`,
`TRACEABILITY.md`, and `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`.

Allowed local inspection is read-only and limited to the backup log/manifest
roots, the NA-0407 temp packet and rollback paths, qsl-backup, Codex ops backup
status and plan docs, Codex ops, and the qwork proof files. The only local
write performed was creation of proof files under the allowed NA-0416 proof
root:

`/srv/qbuild/tmp/NA0416_code23_cleanup_authorization_20260604T070944--0500/`

Forbidden scope includes backup execution, restore execution, sudo, qsl-backup
mutation, temp rollback subtree deletion/move/copy/chmod/chown or other
mutation, backup status/plan mutation, systemd/timer/fstab/source-list/retention
or backup script mutation, durable Director State Index output,
qwork/qstart/qresume/qshell mutation, runtime/protocol/crypto/dependency/workflow
mutation, qsl-server, qsl-attachments, qshield runtime, website, public docs,
README, START_HERE, public technical paper work, secret handling, and
public-claim expansion.

Acceptance requires exact code 23 status, rollback evidence preservation
requirements, selected future scope, no-backup/no-restore/no-qsl-backup/no-temp
mutation boundaries, one READY item, and public-safety remaining green.

## qwork Proof-File Verification

Codex did not run qwork, qstart, or qresume.

The operator-provided qwork proof files existed:

- `/srv/qbuild/work/NA-0416/.qwork/startup.qsl-protocol.kv`
- `/srv/qbuild/work/NA-0416/.qwork/startup.qsl-protocol.json`

They were copied into the NA-0416 proof root under `qwork/`.

The `.kv` proof contained the required values:

- `startup_result=OK`
- `lane=NA-0416`
- `repo=qsl-protocol`
- `path=/srv/qbuild/work/NA-0416/qsl-protocol`
- `head=ebf66f0f3d25`
- `origin_main=ebf66f0f3d25`
- `head_equals_origin_main=yes`
- `worktree_clean=yes`
- `index_clean=yes`
- `untracked_clean=yes`
- `ready_count=1`
- `queue_top_ready=NA-0416`
- `requested_lane_status=READY`

The workspace JSON proof parsed successfully and mirrored the required KV
values for lane, repo, path, head, origin_main, ready_count, queue_top_ready,
requested_lane_status, and clean-state fields.

After `git fetch --all --prune`, live `HEAD` and live `origin/main` still
matched the qwork proof. PR #1099 was MERGED with merge commit
`ebf66f0f3d25`.

Startup proof:

- queue helper: READY_COUNT `1`; READY `NA-0416`
- NA-0415: DONE
- decisions helper: latest decision `D-0818`; duplicate count `0`
- D-0817 once
- D-0818 once
- D-0819 absent at start
- protected `public-safety` check on current main: completed success
- qsl-backup checksum prefix: `e9ecff3d22ed`
- qsl-backup Codex ops source inclusion count: `1`
- `cargo audit --deny warnings`: passed
- locked rustls tree: `rustls-webpki v0.103.13`

## NA-0415 Inheritance

NA-0415 classification:

`CODE23_SOURCE_CONFIRMED_NA0407_ROLLBACK_SUBTREE`

Inherited facts:

- latest scheduled log during NA-0415:
  `/backup/qsl/logs/daily-20260604T023542-0500.log`
- latest scheduled manifest during NA-0415:
  `/backup/qsl/manifests/daily-20260604T023542-0500.manifest.txt`
- latest log had exactly one rsync code 23 / permission-denied source
- the source named the NA-0407 rollback directory
- rollback path existed as `root:root` mode `2700`
- qsl-backup checksum matched the expected `e9ecff3d22ed` prefix
- qsl-backup Codex ops source inclusion count was exactly `1`
- Codex ops manifest presence remained same-host manifest-present evidence only
- NA-0415 did not run backup or restore and did not mutate qsl-backup, the temp
  subtree, status docs, or plan docs

NA-0415 selected NA-0416 specifically to decide whether and how a future lane
should preserve rollback evidence and remediate the scheduled warning.

## Latest Log/Manifest Reconfirmation

Latest scheduled log identified during NA-0416:

`/backup/qsl/logs/daily-20260604T023542-0500.log`

Latest scheduled manifest identified during NA-0416:

`/backup/qsl/manifests/daily-20260604T023542-0500.manifest.txt`

Latest log summary:

- SHA-256 prefix: `9267d2a0543b`
- size: `1003` bytes
- `rsync error` count: `1`
- `code 23` count: `1`
- `Permission denied` count: `1`
- `NA0407` count: `1`
- `rollback` count: `1`
- `qsl-backup.preimage` count: `0`
- exact NA-0407 rollback directory count: `1`

Minimal identifying lines:

```text
rsync: [sender] opendir "/srv/qbuild/tmp/NA0407_qsl_backup_root_action_20260602T232945-0500/rollback" failed: Permission denied (13)
rsync error: some files/attrs were not transferred (see previous errors) (code 23) at main.c(1347) [sender=3.2.7]
```

Latest manifest summary:

- SHA-256 prefix: `1f209ce66e64`
- size: `20157` bytes
- Codex ops path count: `1`
- NA-0407 count: `0`
- rollback count: `0`

No additional rsync error, code 23, or permission-denied source was found in the
latest log. The code 23 warning remains active and is still tied exactly to the
NA-0407 rollback subtree.

## Rollback Evidence Preservation Review

Reviewed packet path:

`/srv/qbuild/tmp/NA0407_qsl_backup_root_action_20260602T232945-0500`

Reviewed rollback path:

`/srv/qbuild/tmp/NA0407_qsl_backup_root_action_20260602T232945-0500/rollback`

Path metadata:

- parent temp root exists as directory `victor:victor` mode `2775`
- packet path exists as directory `victor:victor` mode `2755`
- rollback path exists as directory `root:root` mode `2700`
- rollback directory is not readable by `victor`
- rollback directory is not searchable by `victor`
- visible readable packet files include operator manifest, checksums, preflight
  report, expected patch, source excerpt, README, apply script, verify script,
  and rollback script
- visible rollback entries: none, because directory read is denied

Readable packet metadata:

- schema: `qsl.na0407.operator_root_action_packet.v1`
- original directive: `QSL-DIR-2026-06-03-233`
- target NA: `NA-0407`
- target file: `/usr/local/sbin/qsl-backup`
- target block: `daily_sources`
- added path: `/home/victor/work/qsl/codex/ops`
- expected preimage checksum prefix: `c82ee76fa357`
- generated packet files list excludes rollback contents
- no-secret statement says packet generation used path-only secret scans
- operator-run statement says root-owned mutation required human operator
  inspection and privileged execution if approved

The readable apply and rollback scripts identify an expected rollback filename
with the form `qsl-backup.preimage.<expected-preimage-sha256>`. The exact
filename and full checksum are recorded in the local NA-0416 proof root; this
checked-in evidence uses a prefix to avoid unnecessary long-hex prose.

Preservation conclusions:

- rollback metadata can be preserved without privileged access
- rollback file content cannot be inspected, checksummed, or archived by Codex
  without privileged access
- retaining the rollback subtree is safe for evidence preservation but leaves
  the scheduled same-host code 23 warning active
- cleanup or permission remediation requires future operator/root action
- any future action must first preserve rollback checksum and metadata of the
  rollback file under exact path scope
- after actual remediation, backup status and plan docs should be refreshed in a
  later exact-scope lane if their caveat status changes

## Remediation Options Matrix

| Option | Assessment |
|---|---|
| 1 -- Retain rollback subtree as root-owned evidence | Preserves evidence with no mutation, but the scheduled backup warning remains active. This is acceptable only if the caveat is intentionally carried forward. |
| 2 -- Adjust permissions to make rollback subtree readable | Preferred direction for the next planning lane. It may clear the warning while preserving evidence, but requires root/operator action and exact owner/mode/rollback requirements. |
| 3 -- Move rollback evidence outside source-listed backup tree | May clear the warning, but is a root relocation and creates reference/update risk. Defer unless Option 2 is rejected. |
| 4 -- Delete rollback subtree after preserving evidence | Likely clears the warning, but is destructive and must not be planned as the first remediation path. |
| 5 -- Exclude rollback subtree from backup | Avoids warning but mutates qsl-backup/source-list/exclude behavior and may hide evidence. Rejected for this lane and next lane. |
| 6 -- Run a future check only, no cleanup | No mutation, but it leaves the warning unresolved. Useful only as diagnostic fallback. |

## Authorization Decision

NA-0416 authorizes no implementation and no root action.

Recommended successor type:

`NA-0417 -- QSL Backup Log Code 23 Root Operator Cleanup / Permission Remediation Packet Plan`

Recommended remediation direction:

Plan an evidence-preserving root-operator packet centered first on Option 2:
preserve rollback evidence and then make the rollback subtree readable to the
scheduled same-host backup process if exact future evidence proves that is the
least disruptive fix. The future lane must retain alternatives and stop if
permission remediation would weaken evidence integrity or security posture.

NA-0416 specifically does not authorize:

- sudo
- chmod or chown
- deletion
- movement
- copying rollback file contents
- backup execution
- restore execution
- qsl-backup mutation
- backup status or plan mutation
- systemd/timer/fstab/source-list/retention/script mutation

## Selected Successor

Selected NA-0417:

`NA-0417 -- QSL Backup Log Code 23 Root Operator Cleanup / Permission Remediation Packet Plan`

Reason: the code 23 warning is still active, the source remains exactly the
root-owned NA-0407 rollback directory, rollback content preservation cannot be
completed without privileged inspection or archival, and any real remediation
must be framed as an operator/root packet rather than a Codex-executed action.

## Future Path/Scope Bundle

Future allowed scope should include:

- qsl-protocol governance evidence/testplan paths for NA-0417
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- temp proof packet under `/srv/qbuild/tmp/NA0417_*`
- read-only inspection of backup logs, backup manifests, the NA-0407 packet,
  the NA-0407 rollback subtree, and `/usr/local/sbin/qsl-backup`
- generating operator packet scripts only if the live NA-0417 directive
  explicitly authorizes generated files and no execution

Future forbidden scope unless a later exact directive authorizes otherwise:

- sudo
- chmod/chown
- deletion
- move
- copy of rollback file contents
- backup execution
- restore execution
- qsl-backup mutation
- backup status or plan mutation
- qwork/qstart/qresume/qshell mutation
- runtime/dependency/workflow changes
- qsl-server or qsl-attachments mutation
- public docs, website, README, or START_HERE mutation
- no public readiness claims
- no disaster recovery claims
- no off-host backup claims
- no restore-proven claims
- no backup-complete claims
- no external-review claims
- no metadata-free claims
- no anonymity claims
- no untraceable claims
- no bug-free claims
- no vulnerability-free claims
- no perfect-crypto claims

## Future Validation/Marker Plan

Common NA-0417 markers:

- `NA0417_CODE23_AUTHORIZATION_PLAN_OK`
- `NA0417_ROLLBACK_EVIDENCE_PRESERVATION_OK`
- `NA0417_SAME_HOST_CONTINUITY_CAVEAT_OK`
- `NA0417_NO_BACKUP_EXECUTION_OK`
- `NA0417_NO_RESTORE_EXECUTION_OK`
- `NA0417_NO_QSL_BACKUP_MUTATION_OK`
- `NA0417_NO_TEMP_SUBTREE_MUTATION_OK`
- `NA0417_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0417_NO_SECRET_MATERIAL_OK`

Root-operator packet markers:

- `NA0417_ROOT_OPERATOR_PACKET_PLAN_OK`
- `NA0417_CODEX_NO_SUDO_OK`
- `NA0417_EXACT_PATH_SCOPE_REQUIRED_OK`

## Public Claim/External Review/Website Boundary

This authorization is internal local-ops governance only.

- same-host continuity is not disaster recovery
- manifest presence is not backup completion
- this is not off-host backup complete
- this is not restore proof
- this is not external review
- this is not public technical paper content
- this is not production readiness
- this is not public-internet readiness
- this is not a security policy update
- this is not backup-complete proof
- no README, START_HERE, public docs, or website update is authorized

## Rejected Alternatives

- Deleting rollback evidence now is rejected because content checksum and
  archival cannot be proven by Codex without privilege.
- Moving rollback evidence now is rejected because it is a privileged relocation
  and would require reference updates before preservation is complete.
- Excluding the rollback subtree from qsl-backup is rejected because it changes
  backup behavior and may hide an evidence caveat.
- Running backup or restore is rejected because this lane is authorization-only
  and the directive forbids backup and restore execution.
- Treating the latest manifest as backup completion is rejected because the
  latest log remains code-23 caveated.

## Backup-Impact Statement

NA-0416 does not change backup coverage. It does not run backup, does not run
restore, does not mutate qsl-backup, does not mutate backup status or plan
files, and does not mutate the temp rollback subtree.

Current backup-impact classification:

`SAME_HOST_CODE23_ACTIVE_ROOT_OPERATOR_PACKET_PLAN_SELECTED`

The local status and plan docs still accurately carry the NA-0414 same-host and
code 23 caveats. A future exact-scope status/plan refresh should occur only
after actual remediation changes the caveat state.

## Next Recommendation

Close NA-0416 after this evidence PR merges with public-safety green, then
restore NA-0417 as the sole READY successor using the root-operator packet plan
title and exact no-backup/no-restore/no-sudo/no-temp-mutation boundaries.
