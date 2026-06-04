Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-04

# NA-0415 QSL Backup Log Code 23 Permission-Denied Temp Rollback Subtree Review Plan

Goals: G4

## Executive Summary

NA-0415 performed a read-only review of the scheduled same-host qsl-backup
rsync code 23 caveat inherited from NA-0414.

Classification:

`CODE23_SOURCE_CONFIRMED_NA0407_ROLLBACK_SUBTREE`

The latest scheduled log is the 2026-06-04 daily run. It still contains exactly
one rsync code 23 result, exactly one permission-denied line, and that line
points to the NA-0407 temp rollback subtree. The latest matching manifest still
includes Codex ops exactly once.

Selected successor:

`NA-0416 -- QSL Backup Log Code 23 Temp Rollback Subtree Cleanup / Permission Remediation Authorization Plan`

NA-0415 did not run backup, did not run restore, did not mutate qsl-backup, did
not mutate the temp rollback subtree, did not mutate backup status or plan
files, and did not make public readiness, off-host backup, disaster recovery,
restore-proof, or backup-complete claims.

## Live NA-0415 Scope

Live `NEXT_ACTIONS.md` showed one READY item:

- `NA-0415 -- QSL Backup Log Code 23 Permission-Denied Temp Rollback Subtree Review Plan`
- status: READY
- goals: G1, G2, G3, G4, G5

Allowed qsl-protocol mutation paths for this evidence PR were exactly this
evidence file, the matching NA-0415 testplan, `DECISIONS.md`,
`TRACEABILITY.md`, and `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`.

Allowed local inspection was read-only and limited to the backup log/manifest
roots, the NA-0407 temp packet and rollback paths, qsl-backup, the Codex ops
backup status and plan docs, Codex ops, and the qwork proof files. The only
local write performed was creation of NA-0415 proof files under the allowed temp
proof root:

`/srv/qbuild/tmp/NA0415_backup_log_code23_review_20260604T051642-0500/`

Forbidden scope included backup execution, restore execution, qsl-backup
mutation, temp rollback subtree deletion/move/chmod/chown or other mutation,
backup status/plan mutation, systemd/timer/fstab/source-list/retention/script
mutation, durable Director State Index output, qwork/qstart/qresume/qshell
mutation, runtime/protocol/crypto/dependency/workflow mutation, qsl-server,
qsl-attachments, qshield runtime, website, public docs, README, START_HERE,
public technical paper work, secret handling, and public-claim expansion.

Acceptance required identifying the code 23 source without mutating files,
preserving same-host caveats, preserving no-backup/no-restore and
no-qsl-backup-mutation boundaries, leaving exactly one READY item before
closeout, and keeping public-safety green.

## qwork Proof-File Verification

Codex did not run qwork, qstart, or qresume.

The operator-provided qwork proof files existed:

- `/srv/qbuild/work/NA-0415/.qwork/startup.qsl-protocol.kv`
- `/srv/qbuild/work/NA-0415/.qwork/startup.qsl-protocol.json`

They were copied into the NA-0415 proof root under `qwork/`.

The `.kv` proof contained the required values:

- `startup_result=OK`
- `lane=NA-0415`
- `repo=qsl-protocol`
- `path=/srv/qbuild/work/NA-0415/qsl-protocol`
- `head=68ab384961c1`
- `origin_main=68ab384961c1`
- `head_equals_origin_main=yes`
- `worktree_clean=yes`
- `index_clean=yes`
- `untracked_clean=yes`
- `ready_count=1`
- `queue_top_ready=NA-0415`
- `requested_lane_status=READY`

The workspace JSON proof parsed successfully and mirrored the required KV
values for lane, repo, path, head, origin_main, ready_count, queue_top_ready,
requested_lane_status, and clean-state fields.

After `git fetch --all --prune`, live `HEAD` and live `origin/main` still
matched the qwork proof. PR #1097 was MERGED with merge commit
`68ab384961c1`.

Startup queue and decision proof:

- queue helper: READY_COUNT `1`; READY `NA-0415`
- NA-0414: DONE
- decisions helper: latest decision `D-0816`; duplicate count `0`
- D-0815 once
- D-0816 once
- D-0817 absent at start

Current main protection proof:

- protected `public-safety` check on `68ab384961c1`: completed success
- qsl-backup checksum prefix: `e9ecff3d22ed`
- qsl-backup Codex ops source inclusion count: `1`
- `cargo audit --deny warnings`: passed
- locked rustls tree: `rustls-webpki v0.103.13`

## NA-0414 Inheritance

NA-0414 classification:

`STATUS_PLAN_UPDATED_SAME_HOST_CAVEATED_CODE23_ACTIVE`

Inherited facts:

- NA-0411 classification: `CODEX_OPS_MANIFEST_PRESENT_SAME_HOST`
- qsl-backup includes the Codex ops source exactly once
- NA-0414 updated both local backup docs together
- same-host continuity remains a mandatory caveat
- the latest scheduled log code 23 caveat remained active after NA-0414
- no backup or restore operation was run by NA-0414
- qsl-backup was not mutated by NA-0414

NA-0414 selected NA-0415 specifically to review the code 23
permission-denied temp rollback subtree caveat.

## Log/Manifest Code 23 Review

Latest scheduled log identified during NA-0415:

`/backup/qsl/logs/daily-20260604T023542-0500.log`

Latest scheduled manifest identified during NA-0415:

`/backup/qsl/manifests/daily-20260604T023542-0500.manifest.txt`

The latest pair advanced from the 2026-06-03 pair inherited from NA-0414, but
the caveat source did not change.

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
- NA-0407 rollback path count: `0`

No other rsync error, code 23, or permission-denied source was present in the
latest log. The code 23 caveat is tied exactly to the NA-0407 rollback subtree.

## Temp Rollback Subtree Read-Only Review

Reviewed packet path:

`/srv/qbuild/tmp/NA0407_qsl_backup_root_action_20260602T232945-0500`

Reviewed rollback path:

`/srv/qbuild/tmp/NA0407_qsl_backup_root_action_20260602T232945-0500/rollback`

Path metadata:

- packet path exists: yes
- packet path type/mode/owner/group: directory, `2755`, `victor`, `victor`
- packet path stat size: `4096` bytes
- rollback path exists: yes
- rollback path type/mode/owner/group: directory, `2700`, `root`, `root`
- rollback path stat size: `4096` bytes

Read-only count and size results:

- visible entries under packet path: `10`
- visible entries under rollback path: `0`
- packet accessible `du -sb` result: `19017` bytes, with rollback
  permission denied
- rollback `du -sb` result: not reliable because directory read is denied
- rollback checksum attempt: permission denied without privilege

The permission-denied path inferred from the latest log is the rollback
directory itself. The readable packet metadata identifies `/usr/local/sbin/qsl-backup`
as the target file and records an expected preimage checksum prefix
`c82ee76fa357`. The readable scripts also refer to a rollback preimage file
under the root-only rollback directory. This supports that the unreadable
rollback subtree contains rollback evidence for qsl-backup, but NA-0415 cannot
prove the rollback copy checksum without privilege and must not attempt to
bypass permissions.

Visible high-confidence secret review:

- visible secret-looking top-level packet filename count: `0`
- readable packet metadata says path-only secret scans were used and no secret
  content was included by design
- no secret material was copied into NA-0415 evidence
- the root-only rollback subtree could not be content-scanned without privilege

The rollback path is still useful as rollback/evidence material until an
authorized future directive decides how to preserve or retire it. Cleanup or
permission remediation appears warranted because the same root-only subtree is
causing repeated scheduled same-host backup code 23 caveats. Exact future path
authorization is required before any chmod, chown, move, delete, or exclusion
change.

## Status/Plan Consistency Review

Read-only files reviewed:

- `/home/victor/work/qsl/codex/ops/backup/QSL_BACKUP_STATUS.md`
- `/home/victor/work/qsl/codex/QSL_BACKUP_PLAN.md`

The NA-0414 local docs still mention:

- Codex ops same-host source-list inclusion
- the NA-0414-reviewed 2026-06-03 manifest/log pair
- Codex ops manifest count exactly once
- the rsync code 23 caveat from the NA-0407 rollback subtree
- same-host continuity only
- no off-host backup claim
- no disaster recovery claim
- no restore validation claim
- no comprehensive backup coverage claim
- no public readiness claim
- no external review claim
- no public technical paper claim
- future code 23 remediation as a separate lane

The files do not explicitly name NA-0415 by ID, but their future-action wording
points to review/remediation of the same code 23 caveat that NA-0415 now
reviewed. Their wording remains accurate because they describe the 2026-06-03
pair as reviewed by NA-0414. After NA-0416 decides the cleanup/remediation
authorization path, a later status/plan refresh may be needed to record the
latest reviewed log/manifest pair and the selected remediation decision.

NA-0415 did not mutate either local status/plan file.

## Remediation Options

Considered classifications and successors:

- Confirmed NA-0407 rollback subtree with cleanup/remediation warranted:
  selected.
- Confirmed source with no cleanup needed: rejected because the warning is
  repeated on the latest scheduled log and distorts backup log accuracy.
- Ambiguous source: rejected because the latest log has exactly one code 23
  source and it names the NA-0407 rollback directory.
- Latest log clean: rejected because code 23 remains present.
- Other failure found: rejected because no other error source was present in
  the latest scheduled log.

## Selected Successor

Selected NA-0416 successor:

`NA-0416 -- QSL Backup Log Code 23 Temp Rollback Subtree Cleanup / Permission Remediation Authorization Plan`

Rationale:

- the code 23 source is confirmed
- the same path continues to affect scheduled same-host logs
- rollback/evidence preservation must be addressed before cleanup
- NA-0415 is read-only and cannot safely decide or perform file mutation

NA-0415 does not implement NA-0416.

## Future Path/Scope Bundle

Future possible mutable paths may include only if exact future scope authorizes:

- `/srv/qbuild/tmp/NA0407_qsl_backup_root_action_20260602T232945-0500/rollback`
- specific child files under that rollback directory, if identified and
  explicitly authorized
- qsl-protocol governance paths needed for NA-0416 evidence/testplan/decision

Future forbidden unless exact future scope authorizes:

- sudo
- chmod/chown
- deletion
- move
- backup execution
- restore execution
- qsl-backup mutation
- backup status/plan mutation
- qwork/qstart/qresume/qshell mutation
- runtime/dependency/workflow changes
- public docs/website
- public claims

NA-0416 should first decide how rollback evidence must be preserved before any
implementation directive is allowed to mutate the path.

## Future Validation/Marker Plan

Common NA-0416 markers:

- `NA0416_CODE23_REVIEW_COMPLETE_OK`
- `NA0416_SAME_HOST_CONTINUITY_CAVEAT_OK`
- `NA0416_NO_BACKUP_EXECUTION_OK`
- `NA0416_NO_RESTORE_EXECUTION_OK`
- `NA0416_NO_QSL_BACKUP_MUTATION_OK`
- `NA0416_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0416_NO_SECRET_MATERIAL_OK`

Cleanup/remediation authorization markers:

- `NA0416_CODE23_ROLLBACK_SUBTREE_CLEANUP_AUTHORIZATION_PLAN_OK`
- `NA0416_EXACT_PATH_SCOPE_REQUIRED_OK`

Suggested validation:

- verify current READY queue and decision sequence
- re-identify the latest scheduled log and manifest read-only
- confirm whether the same rollback path is still the only code 23 source
- verify qsl-backup checksum/source-list read-only
- verify rollback evidence preservation requirements
- run scope guard, link-check, leak-scan, overclaim scan, classifier,
  PR-body preflight, goal-lint, dependency health, qsc send_commit, and formal
  model checks
- require public-safety before merge and after merge

## Public Claim/External Review/Website Boundary

This review is internal local-ops evidence only.

It does not establish:

- off-host backup completion
- disaster recovery completion
- restore proof
- backup completion
- production readiness
- public-internet readiness
- external review
- public technical paper evidence
- metadata-free behavior
- anonymity
- untraceability
- bug-free status
- vulnerability-free status
- perfect crypto

No README, START_HERE, public docs, website, security policy, qsl-server, or
qsl-attachments update is introduced.

## Rejected Alternatives

- Run backup or restore to test the warning: rejected as forbidden and
  unnecessary for source classification.
- chmod/chown/delete/move the rollback path now: rejected as forbidden and
  unsafe without exact rollback-evidence authorization.
- Mutate qsl-backup, source lists, timers, fstab, or backup scripts: rejected
  as out of scope.
- Treat manifest presence as backup completion: rejected because the latest log
  remains code-23 caveated and the evidence is same-host only.
- Ignore the warning permanently: rejected because it is repeated and has an
  exact local source.

## Backup-Impact Statement

NA-0415 made no backup system mutation and ran no backup or restore operation.

The backup-impact classification is same-host caveat review only. The latest
manifest proves source presence for Codex ops in the same-host manifest exactly
once, but the matching latest log still has an rsync code 23 caveat. This is
not off-host backup, disaster recovery, restore proof, backup completion, or
public readiness evidence.

## Next Recommendation

Execute the selected NA-0416 authorization lane. It should preserve the exact
code 23 source, define rollback evidence preservation requirements, and decide
whether a later implementation directive may safely clean up or remediate the
root-only rollback subtree.
