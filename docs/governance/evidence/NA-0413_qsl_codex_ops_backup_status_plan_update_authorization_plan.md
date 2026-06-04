Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-04

# NA-0413 QSL Codex Ops Backup Status Plan Update Authorization Plan

Goals: G4

## Executive Summary

NA-0413 reviewed whether the local Codex backup status and backup plan
documents should be updated after NA-0411 proved that Codex ops appears in the
scheduled same-host qsl-backup manifest.

Classification:

`STATUS_PLAN_UPDATE_IMPLEMENTATION_AUTHORIZED_SAME_HOST_CAVEATED`

Selected successor:

`NA-0414 -- QSL Codex Ops Backup Status / Plan Update Implementation Harness`

Both local documents are candidate future mutable files, but NA-0413 did not
mutate either one. The future update must preserve the same-host continuity
caveat, the rsync code 23 caveat from the latest scheduled log, and the
no-backup/no-restore/no-public-overclaim boundary.

## Live NA-0413 Scope

Live `NEXT_ACTIONS.md` shows one READY item:

- `NA-0413 -- QSL Codex Ops Backup Status / Plan Update Authorization Plan`
- status: READY
- goals: G1, G2, G3, G4, G5

Allowed NA-0413 qsl-protocol mutation paths are this evidence file, the
matching NA-0413 testplan, `DECISIONS.md`, `TRACEABILITY.md`, and
`docs/ops/ROLLING_OPERATIONS_JOURNAL.md`.

Read-only local scope covers:

- `/home/victor/work/qsl/codex/ops/backup/QSL_BACKUP_STATUS.md`
- `/home/victor/work/qsl/codex/QSL_BACKUP_PLAN.md`
- `/backup/qsl/manifests`
- `/backup/qsl/logs`
- `/usr/local/sbin/qsl-backup`
- `/home/victor/work/qsl/codex/ops`

Forbidden scope includes backup execution, restore execution, qsl-backup
mutation, backup status or plan mutation, systemd/timer/fstab/source-list
mutation, durable Director State Index output, helper mutation, runtime or
workflow mutation, public docs or website updates, and readiness or backup
overclaims.

## qwork Proof-File Verification

Codex did not run qwork, qstart, or qresume.

The operator-provided qwork proof files existed and were copied into:

`/srv/qbuild/tmp/NA0413_backup_status_plan_authorization_20260603T205143-0500/qwork/`

The `.kv` proof contained the required values:

- `startup_result=OK`
- `lane=NA-0413`
- `repo=qsl-protocol`
- `path=/srv/qbuild/work/NA-0413/qsl-protocol`
- `head=82ebae4dc12ceda84f3ba459b877f10f0ca6ca51`
- `origin_main=82ebae4dc12ceda84f3ba459b877f10f0ca6ca51`
- `head_equals_origin_main=yes`
- `worktree_clean=yes`
- `index_clean=yes`
- `untracked_clean=yes`
- `ready_count=1`
- `queue_top_ready=NA-0413`
- `requested_lane_status=READY`

The workspace JSON proof parsed successfully and mirrored the required KV
values for lane, repo, path, head, origin_main, ready_count, queue_top_ready,
requested_lane_status, and clean-state fields.

After `git fetch --all --prune`, live `HEAD` and live `origin/main` still
matched the qwork proof. PR #1093 was MERGED with merge commit
`82ebae4dc12c`, matching the live main prefix. The queue helper reported
READY_COUNT `1` and READY `NA-0413`. The decisions helper reported latest
decision `D-0812` and duplicate count `0`.

## NA-0411 Inheritance

NA-0411 classification:

`CODEX_OPS_MANIFEST_PRESENT_SAME_HOST`

Inherited facts:

- qsl-backup checksum:
  `e9ecff3d22eda21ceb0e889e4dd5d6f4e270e09349c77a1f4872bfc0052f6232`
- exact Codex ops source inclusion count: `1`
- latest scheduled same-host manifest at NA-0411 included Codex ops exactly
  once
- latest scheduled log carried an rsync code 23 caveat from a
  permission-denied NA-0407 temp rollback subtree

NA-0411 was same-host continuity evidence only. It did not prove off-host
coverage, disaster recovery, restore success, or complete backup status.

## NA-0412 Proof-File Handoff Inheritance

NA-0412 changed the operator/Codex handoff pattern so future non-qwork lanes can
read stable qwork proof files under `/srv/qbuild/work/<NA>/.qwork/` and verify
live repo state directly. NA-0413 followed that pattern. Codex did not rerun
qwork.

## Status File Read-Only Review

File:

`/home/victor/work/qsl/codex/ops/backup/QSL_BACKUP_STATUS.md`

Observed metadata:

- exists: yes
- size: `8087` bytes
- mode/owner/group: `664 victor victor`
- mtime: `2026-05-17 15:39:12.683816083 -0500`

Review findings:

- The file predates the NA-0407 qsl-backup source-list update and NA-0411
  manifest proof.
- It does not mention the latest manifest
  `daily-20260603T023518-0500.manifest.txt`.
- It does not mention the latest log's rsync code 23 caveat.
- It has no explicit same-host manifest-evidence caveat for Codex ops.
- It still records an older qsl-backup installed checksum.
- Its current daily source-list block omits
  `/home/victor/work/qsl/codex/ops`.

Future target sections, if NA-0414 authorizes exact wording:

- summary and boundary section near lines 8-13
- installed script checksum section near lines 60-83
- current snapshot and manifest evidence section near lines 128-141
- current daily source-list section near lines 144-160
- validation or next-step notes near the end of the file

## Backup Plan Read-Only Review

File:

`/home/victor/work/qsl/codex/QSL_BACKUP_PLAN.md`

Observed metadata:

- exists: yes
- size: `17510` bytes
- mode/owner/group: `664 victor victor`
- mtime: `2026-05-17 15:39:26.476121981 -0500`

Review findings:

- The file predates NA-0411 manifest proof and does not mention the latest
  manifest.
- Its daily live continuity backup scope lists Codex logs but omits Codex ops.
- Its verification checklist still names Codex logs but not Codex ops.
- It does not mention the latest log's rsync code 23 caveat.
- It states the same-host continuity boundary generally, but does not apply
  that boundary to the Codex ops manifest evidence.

Future target sections, if NA-0414 authorizes exact wording:

- executive recommendation near lines 6-16
- daily live continuity backup scope near lines 160-176
- restore plan near lines 474-496
- verification plan near lines 498-521
- open Director decisions or next-step notes near the end of the file

## Manifest/Log Reconfirmation

Latest scheduled same-host evidence at NA-0413:

- latest manifest:
  `/backup/qsl/manifests/daily-20260603T023518-0500.manifest.txt`
- latest log:
  `/backup/qsl/logs/daily-20260603T023518-0500.log`

The latest manifest still includes `/home/victor/work/qsl/codex/ops` exactly
once. It also records the expected qsl-backup checksum.

The latest log has zero literal Codex ops path matches, which is expected
because the log does not enumerate every source path. It still records the
permission-denied rollback subtree and rsync code 23 caveat. That caveat remains
active evidence and must be preserved by any future status or plan update.

No newer manifest or log appeared after the NA-0411 evidence.

## Codex Ops Safety Recheck

Read-only scan of `/home/victor/work/qsl/codex/ops`:

- file count: `8`
- total size: `32723` bytes
- symlink count: `0`
- symlink escape count: `0`
- binary candidate count: `0`
- high-confidence secret path finding count: `0`
- high-confidence secret content path finding count: `0`
- durable Director State Index path exists: `false`

No secret material was copied into tracked evidence.

## Status/Plan Update Authorization Decision

Decision:

Both local documents should be updated together in a later exact-scope
implementation lane.

Reason:

- The status file is stale relative to installed qsl-backup checksum,
  source-list, and same-host manifest evidence.
- The plan file is stale relative to daily live continuity scope and
  verification targets.
- Updating only one file would leave inconsistent local operational guidance.
- The current evidence is stable enough to plan a narrow update because the
  latest manifest still includes Codex ops exactly once and the latest log
  caveat is understood.

NA-0413 does not authorize Codex to mutate either local file during this lane.

## Selected Successor

Selected successor:

`NA-0414 -- QSL Codex Ops Backup Status / Plan Update Implementation Harness`

This is the normal implementation-harness successor because the status/plan
docs are stale and the current manifest/log evidence is not contradictory.

## Future Path/Scope Bundle

Future local mutable paths, if explicitly authorized by NA-0414:

- `/home/victor/work/qsl/codex/ops/backup/QSL_BACKUP_STATUS.md`
- `/home/victor/work/qsl/codex/QSL_BACKUP_PLAN.md`

Future qsl-protocol governance paths:

- `docs/governance/evidence/NA-0414_*.md`
- `tests/NA-0414_*_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Future forbidden scope unless an exact future directive changes it:

- backup execution
- restore execution
- qsl-backup mutation
- qwork/qstart/qresume/qshell mutation
- durable Director State Index output
- runtime, dependency, or workflow changes
- public docs or website changes
- public readiness or backup overclaims

## Future Validation/Marker Plan

Common NA-0414 markers:

- `NA0414_CODEX_OPS_MANIFEST_PRESENT_SAME_HOST_OK`
- `NA0414_LOG_CODE_23_CAVEAT_PRESERVED_OK`
- `NA0414_SAME_HOST_CONTINUITY_CAVEAT_OK`
- `NA0414_NO_BACKUP_EXECUTION_OK`
- `NA0414_NO_RESTORE_EXECUTION_OK`
- `NA0414_NO_QSL_BACKUP_MUTATION_OK`
- `NA0414_NO_DURABLE_INDEX_WRITE_OK`
- `NA0414_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0414_NO_SECRET_MATERIAL_OK`
- `NA0414_STATUS_PLAN_UPDATE_IMPLEMENTATION_AUTHORIZED_OK`
- `NA0414_EXACT_LOCAL_PATHS_SELECTED_OK`

## Public Claim/External Review/Website Boundary

NA-0413 is internal local-ops governance only. It does not update README,
START_HERE, public docs, website content, security policy, or public technical
paper material.

The same-host manifest evidence remains local continuity evidence only. It is
not off-host coverage, disaster recovery, restore proof, production readiness,
public-internet readiness, external review, or backup completion.

## Rejected Alternatives

- Blocker successor: rejected because the current wording targets and evidence
  caveats are clear enough for a future exact-scope implementation lane.
- Conflict successor: rejected because the latest manifest still includes Codex
  ops exactly once and the latest log caveat is consistent with NA-0411.
- No-update successor: rejected because both local documents are stale relative
  to NA-0411 and NA-0413 evidence.

## Backup-Impact Statement

NA-0413 has no backup runtime impact. It did not run a backup, run a restore,
mutate qsl-backup, mutate source lists, mutate systemd or fstab, or mutate the
local backup status/plan files.

The backup-impact classification remains:

`CODEX_OPS_MANIFEST_PRESENT_SAME_HOST`

with explicit caveats for same-host continuity and the latest log rsync code 23
result.

## Next Recommendation

Proceed to NA-0414 only after NA-0413 merges and closeout restores the selected
successor as READY. NA-0414 should update both local documents together, using
only exact authorized local paths and preserving all caveats.
