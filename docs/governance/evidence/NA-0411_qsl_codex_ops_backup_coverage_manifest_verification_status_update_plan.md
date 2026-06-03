Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-03

# NA-0411 QSL Codex Ops Backup Coverage Manifest Verification Status Update Plan

Goals: G4

## Executive Summary

NA-0411 verified, read-only, that the operator-applied qsl-backup source-list
update from NA-0407 has appeared in scheduled same-host manifest evidence. The
latest post-source-list manifest records the Codex ops source root in the
installed daily source list.

Classification:

`CODEX_OPS_MANIFEST_PRESENT_SAME_HOST`

Selected successor:

`NA-0412 -- QSL Codex Ops Backup Status / Plan Update Authorization Plan`

The result is internal local-ops continuity evidence only. Codex did not run a
backup or restore, did not mutate qsl-backup, and did not update the local
backup status or backup plan files.

## Live NA-0411 Scope

Live `NEXT_ACTIONS.md` shows one READY item:

- `NA-0411 -- QSL Codex Ops Backup Coverage Manifest Verification / Status Update Plan`
- status: READY
- goals: G4

Allowed NA-0411 qsl-protocol mutation paths are this evidence file, the
matching NA-0411 testplan, `DECISIONS.md`, `TRACEABILITY.md`, and
`docs/ops/ROLLING_OPERATIONS_JOURNAL.md`.

The live scope forbids backup execution, restore execution, qsl-backup
mutation, backup status mutation, backup plan mutation, systemd/timer/fstab
mutation, durable Director State Index output, helper mutation, runtime or
workflow mutation, public docs or website updates, and public-readiness or
backup-complete claims.

## qwork Startup Proof

Startup command:

`qwork NA-0411 qsl-protocol`

Result:

- `startup_result=OK`
- `lane=NA-0411`
- `repo=qsl-protocol`
- `head_equals_origin_main=yes`
- `worktree_clean=yes`
- `index_clean=yes`
- `untracked_clean=yes`
- `ready_count=1`
- `queue_top_ready=NA-0411`
- `requested_lane_status=READY`
- JSON proof: `/srv/qbuild/logs/NA-0411/startup.qsl-protocol.json`

Host timestamps recorded during startup:

- America/Chicago: 2026-06-03T16:03:19-05:00
- UTC: 2026-06-03T21:03:19+00:00

Live GitHub/origin state resolves the D244 SHA inconsistency. PR #1088 is
MERGED with merge prefix `ae273816a59a`; `origin/main` is exactly that merge
commit. The earlier `9fef1a934d4e` prefix belongs to the prior NA-0409
closeout state, not current NA-0411 main.

## qwork Fixed-Surface Proof

The fixed qwork surface was rechecked before NA-0411 evidence edits:

- bare `qwork` resolves to `/home/victor/.local/bin/qwork`
- `qwork NA-0411 qsl-protocol` succeeds from the qsl-protocol checkout
- `qwork NA-0411 qsl-protocol` succeeds from `/tmp`
- the qshell bad-lane fail-closed smoke prints `shell-survived`
- no qwork, qstart, qresume, or qshell mutation was performed

Recovered failure recorded during this lane:

- failing command: the first `/tmp` qwork smoke was run in parallel with
  another qwork smoke and returned `reason=lane-lock-held`
- classification: recoverable command-shape/test-sequencing issue because
  qwork correctly serializes by lane lock
- corrective action: reran the `/tmp` qwork smoke serially
- final result: `/tmp` qwork smoke passed

## D242/D244 qwork Issue Context

D242 diagnosed a prior qwork lane-mismatch symptom. D244 then reproduced and
fixed the real qwork cwd-dependent queue verification bug by updating local
`/srv/qbuild/tools/qwork.sh` to read the target checkout queue file. D244 did
not modify qshell, did not run backup or restore, and restored NA-0411 as the
sole READY backup manifest/status lane through D-0807.

## NA-0407 Source-List Inheritance

NA-0407 validated the human-operator-applied qsl-backup source-list update.
The inherited state for NA-0411 is:

- installed qsl-backup checksum prefix: `e9ecff3d22ed`
- qsl-backup syntax check: passed
- Codex ops source inclusion count: `1`
- NA-0407 classification before NA-0411: `SOURCE_LIST_UPDATED_NOT_MANIFEST_PROVEN`
- backup status and backup plan update: future-gated

## qsl-backup Source-List Validation

NA-0411 inspected `/usr/local/sbin/qsl-backup` read-only:

- stat: `755 root root`, size `6800`, mtime
  `2026-06-02 23:50:10.822505978 -0500`
- checksum prefix: `e9ecff3d22ed`
- `bash -n /usr/local/sbin/qsl-backup`: passed
- exact Codex ops source count in the script: `1`
- `daily_sources` includes Codex logs, responses, backup plan, and Codex ops

Full checksum and source-list proof is stored under:

`/srv/qbuild/tmp/NA0411_codex_ops_manifest_status_20260603T160508-0500/backup`

## Manifest / Log Verification

Latest scheduled same-host evidence inspected:

- latest manifest: `/backup/qsl/manifests/daily-20260603T023518-0500.manifest.txt`
- latest log: `/backup/qsl/logs/daily-20260603T023518-0500.log`
- latest manifest mtime: `2026-06-03T02:35:23-05:00`
- latest log mtime: `2026-06-03T02:35:46-05:00`

The installed qsl-backup script changed before the latest manifest/log. The
latest manifest source list includes the Codex ops source root exactly once,
so a post-source-list scheduled manifest exists and records Codex ops in scope.

The latest log does not list every source path and has zero literal Codex ops
path matches. It also records an rsync code 23 caused by a permission-denied
rollback subdirectory under the NA-0407 temp proof area. That log caveat is why
this evidence does not claim backup completion, restore proof, off-host
coverage, or disaster recovery.

Zero-match recovery recorded during this lane:

- failing command: literal `rg` over the latest log for the Codex ops source
  returned exit code `1`
- classification: valid zero-match discovery because the log summary does not
  enumerate every source path
- corrective action: reran a zero-safe path-count helper over the latest log
  and manifest
- final result: latest log count `0`; latest manifest count `1`

## Backup Status / Plan Read-Only Review

NA-0411 inspected the local backup status and plan files read-only:

- `/home/victor/work/qsl/codex/ops/backup/QSL_BACKUP_STATUS.md`
- `/home/victor/work/qsl/codex/QSL_BACKUP_PLAN.md`

Both files are stale relative to the live qsl-backup source-list state and the
latest manifest evidence:

- the status file still records the old installed checksum and old daily source
  list
- the status daily source-list block omits Codex ops
- the plan daily scope still lists Codex logs but not Codex ops
- neither file records the 2026-06-03 post-source-list manifest result

A future local status/plan update is warranted, but it remains blocked unless
an exact future directive authorizes those local paths.

## Codex Ops Safety Recheck

NA-0411 inspected `/home/victor/work/qsl/codex/ops` read-only:

- file count: `8`
- total size: `32723` bytes
- symlink count: `0`
- symlink escape count: `0`
- binary candidate count: `0`
- high-confidence secret path-name finding count: `0`
- high-confidence secret content path finding count: `0`
- durable Director State Index path exists: `false`

No secret content was copied into tracked governance evidence.

## Coverage Classification

Result:

`CODEX_OPS_MANIFEST_PRESENT_SAME_HOST`

Reason:

- the qsl-backup source-list state is intact
- a scheduled manifest exists after the qsl-backup source-list change
- that manifest includes the Codex ops source root exactly once

Boundary:

- same-host continuity evidence only
- no backup-complete claim
- no restore-proof claim
- no off-host-backup claim
- no disaster-recovery claim

## Selected Successor

Because the post-source-list manifest includes Codex ops, the exact successor
is:

`NA-0412 -- QSL Codex Ops Backup Status / Plan Update Authorization Plan`

NA-0411 does not implement NA-0412.

## Future Path / Scope Bundle

If NA-0412 authorizes local status/plan update work, possible future local
paths are:

- `/home/victor/work/qsl/codex/ops/backup/QSL_BACKUP_STATUS.md`
- `/home/victor/work/qsl/codex/QSL_BACKUP_PLAN.md`

Future qsl-protocol governance paths are:

- `docs/governance/evidence/NA-0412_*.md`
- `tests/NA-0412_*_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Future forbidden surfaces remain backup execution, restore execution,
qsl-backup mutation, durable Director State Index output, helper mutation,
runtime/dependency/workflow changes, public docs or website changes, and public
claim expansion unless exact future scope authorizes otherwise.

## Future Validation / Marker Plan

Common NA-0412 markers:

- `NA0412_CODEX_OPS_SOURCE_LIST_PRESENT_OK`
- `NA0412_NO_BACKUP_EXECUTION_OK`
- `NA0412_NO_RESTORE_EXECUTION_OK`
- `NA0412_NO_DURABLE_INDEX_WRITE_OK`
- `NA0412_SAME_HOST_CONTINUITY_CAVEAT_OK`
- `NA0412_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0412_NO_SECRET_MATERIAL_OK`

Manifest-present successor markers:

- `NA0412_CODEX_OPS_MANIFEST_PRESENT_SAME_HOST_OK`
- `NA0412_STATUS_UPDATE_AUTHORIZATION_PLAN_OK`

## Public Claim / External Review / Website Boundary

NA-0411 is internal local-ops evidence only. It is not:

- disaster recovery
- off-host backup completion
- restore proof
- external review
- public technical paper work
- production readiness
- public-internet readiness
- a public website or README update
- a security policy update

No README, START_HERE, public docs, website, or security policy file changed.

## Rejected Alternatives

- Running a backup: rejected as out of scope.
- Running a restore: rejected as out of scope.
- Updating local backup status or plan files in NA-0411: rejected because the
  directive explicitly makes those files read-only.
- Treating source-list inclusion alone as backup-complete proof: rejected.
- Treating the rsync code 23 log caveat as a source-list conflict: rejected
  because the post-source-list manifest explicitly includes Codex ops.
- Creating durable Director State Index output: rejected as out of scope.

## Backup-Impact Statement

This PR changes only tracked qsl-protocol governance, traceability, journal,
and testplan files. It does not mutate local backup scripts, source lists,
status files, plan files, timers, fstab, snapshots, or restore outputs.

Future NA-0412 should update local status/plan files only if its live scope
authorizes the exact local paths and preserves the same-host continuity caveat
plus the latest log caveat.

## Next Recommendation

Merge NA-0411 evidence after validation and required checks. Then, if
post-merge public-safety is green, close NA-0411 and restore:

`NA-0412 -- QSL Codex Ops Backup Status / Plan Update Authorization Plan`
