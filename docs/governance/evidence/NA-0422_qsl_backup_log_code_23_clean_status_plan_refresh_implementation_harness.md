Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-05

# NA-0422 QSL Backup Log Code 23 Clean Status Plan Refresh Implementation Harness

Goals: G4

## Executive summary

NA-0422 completed the exact local status/plan refresh authorized by NA-0421.
The reviewed scheduled same-host backup log remained clean, the matching
manifest kept Codex ops present exactly once, and both authorized local files
now cite the clean scheduled evidence instead of the stale NA-0414 code 23
caveat.

Updated local files:

- `/home/victor/work/qsl/codex/ops/backup/QSL_BACKUP_STATUS.md`
- `/home/victor/work/qsl/codex/QSL_BACKUP_PLAN.md`

Clean scheduled same-host evidence:

- log: `/backup/qsl/logs/daily-20260605T023308-0500.log`
- manifest: `/backup/qsl/manifests/daily-20260605T023308-0500.manifest.txt`

The clean scheduled log is local same-host evidence only. It is not off-host
backup evidence, disaster recovery evidence, restore proof, backup completion
evidence, production readiness, public internet readiness, external review, or
public technical paper evidence.

Selected successor:

`NA-0423 -- QSL Domain Stewardship / Director Workflow Governance Authorization Plan`

## Live NA-0422 scope

Start proof reported exactly one READY item:

- `NA-0422 -- QSL Backup Log Code 23 Clean Status / Plan Refresh Implementation Harness`

Allowed local mutable files were exactly:

- `/home/victor/work/qsl/codex/ops/backup/QSL_BACKUP_STATUS.md`
- `/home/victor/work/qsl/codex/QSL_BACKUP_PLAN.md`

Allowed qsl-protocol mutation paths for this evidence PR are exactly:

- `docs/governance/evidence/NA-0422_qsl_backup_log_code_23_clean_status_plan_refresh_implementation_harness.md`
- `tests/NA-0422_qsl_backup_log_code_23_clean_status_plan_refresh_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Forbidden mutation scope was preserved:

- no qwork, qstart, or qresume execution by Codex;
- no sudo;
- no generated operator script execution;
- no backup;
- no restore;
- no `/usr/local/sbin/qsl-backup` mutation;
- no `/backup/qsl` mutation;
- no rollback subtree mutation;
- no systemd, timer, fstab, source-list, retention, or backup-script mutation;
- no qwork/qstart/qresume/qshell mutation;
- no runtime, crypto, dependency, workflow, qsl-server, qsl-attachments,
  qshield runtime, website, public docs, README, or START_HERE mutation;
- no durable Director State Index output;
- no public technical paper work;
- no public overclaim.

## qwork proof-file verification

Codex read, but did not run or regenerate, the expected qwork proof files:

- `/srv/qbuild/work/NA-0422/.qwork/startup.qsl-protocol.kv`
- `/srv/qbuild/work/NA-0422/.qwork/startup.qsl-protocol.json`

The `.kv` proof contained all required markers, including
`startup_result=OK`, lane `NA-0422`, repo `qsl-protocol`, path
`/srv/qbuild/work/NA-0422/qsl-protocol`, clean worktree/index/untracked
fields, READY_COUNT `1`, queue top READY `NA-0422`, and requested lane status
`READY`.

The JSON proof parsed successfully and mirrored the `.kv` proof for lane, repo,
path, head, origin/main, READY count, queue top READY, requested lane status,
and clean-state fields.

Recorded timestamps:

- local: `2026-06-05T11:07:54-05:00`
- UTC: `2026-06-05T16:07:54+00:00`

After `git fetch --all --prune`, live `HEAD` and live `origin/main` still
matched the qwork proof at `e7328d8fb3cf`. PR #1112 was MERGED with merge
commit `e7328d8fb3cf`.

Queue proof reported READY_COUNT `1`, READY `NA-0422`, and NA-0421 DONE.
Decision proof reported latest `D-0831`, duplicate decision count zero, D-0830
once, D-0831 once, and D-0832 absent at start.

The qwork proof files were copied into:

- `/srv/qbuild/tmp/NA0422_clean_status_plan_refresh_20260605T110903-0500/qwork/`

## NA-0421 inheritance

Inherited NA-0421 classification:

`STATUS_REFRESH_AUTHORIZED_CLEAN_SAME_HOST_CODE23_CLEARED`

Inherited clean scheduled same-host pair:

- log: `/backup/qsl/logs/daily-20260605T023308-0500.log`
- manifest: `/backup/qsl/manifests/daily-20260605T023308-0500.manifest.txt`

Inherited clean log result:

- rsync error count: `0`
- code 23 count: `0`
- Permission denied count: `0`
- NA0407 count: `0`
- rollback count: `0`
- exact rollback directory reference count: `0`
- qsl-backup.preimage count: `0`
- nonzero exit-code marker count: `0`

Inherited manifest result:

- `/home/victor/work/qsl/codex/ops` count: `1`
- NA0407 count: `0`
- rollback count: `0`
- exact rollback directory reference count: `0`
- qsl-backup.preimage count: `0`

D-0830 selected both local files as future mutable candidates for NA-0422.
D-0831 restored NA-0422 as the sole READY implementation successor.

## Clean log/manifest reconfirmation

NA-0422 reconfirmed read-only that the latest scheduled log and manifest under
`/backup/qsl/logs` and `/backup/qsl/manifests` were still:

- `/backup/qsl/logs/daily-20260605T023308-0500.log`
- `/backup/qsl/manifests/daily-20260605T023308-0500.manifest.txt`

No newer scheduled pair existed during review.

Artifact metadata:

- log: `victor:victor` mode `644`, mtime
  `2026-06-05 02:33:44.166110544 -0500`
- manifest: `victor:victor` mode `644`, mtime
  `2026-06-05 02:33:12.036546521 -0500`

Checksum prefixes:

- log SHA256: `2e82582cc6a0`
- manifest SHA256: `4331d00be68f`

Reconfirmed log counts:

- `rsync error`: `0`
- `code 23`: `0`
- `Permission denied`: `0`
- `NA0407`: `0`
- `rollback`: `0`
- exact NA-0407 rollback directory reference: `0`
- `qsl-backup.preimage`: `0`
- nonzero exit-code marker: `0`

Reconfirmed manifest counts:

- `/home/victor/work/qsl/codex/ops`: `1`
- `NA0407`: `0`
- `rollback`: `0`
- exact NA-0407 rollback directory reference: `0`
- `qsl-backup.preimage`: `0`

Copies of the reviewed log and manifest were stored under:

- `/srv/qbuild/tmp/NA0422_clean_status_plan_refresh_20260605T110903-0500/logs/`
- `/srv/qbuild/tmp/NA0422_clean_status_plan_refresh_20260605T110903-0500/manifests/`

## Pre-mutation local doc review

Status file:

- path: `/home/victor/work/qsl/codex/ops/backup/QSL_BACKUP_STATUS.md`
- file type: regular file, not symlink, readable
- metadata before patch: `victor:victor` mode `664`, mtime
  `2026-06-03 21:44:18.951416552 -0500`
- SHA256 prefix before patch: `036b608b75c6`
- stale NA-0414 code 23 caveat present before patch: yes
- NA-0420 clean proof present before patch: no
- clean scheduled log path present before patch: no
- clean scheduled manifest path present before patch: no
- same-host caveat present before patch: yes
- high-confidence secret markers before patch: `0`
- positive overclaim markers before patch: `0`

Backup plan file:

- path: `/home/victor/work/qsl/codex/QSL_BACKUP_PLAN.md`
- file type: regular file, not symlink, readable
- metadata before patch: `victor:victor` mode `664`, mtime
  `2026-06-03 21:44:18.952416565 -0500`
- SHA256 prefix before patch: `bba5e4ebad6`
- stale NA-0414 code 23 caveat present before patch: yes
- NA-0420 clean proof present before patch: no
- clean scheduled log path present before patch: no
- clean scheduled manifest path present before patch: no
- same-host caveat present before patch: yes
- high-confidence secret markers before patch: `0`
- positive overclaim markers before patch: `0`

## Rollback copy proof

Rollback copies were created before local mutation:

- `/srv/qbuild/tmp/NA0422_clean_status_plan_refresh_20260605T110903-0500/rollback/QSL_BACKUP_STATUS.md.rollback`
- `/srv/qbuild/tmp/NA0422_clean_status_plan_refresh_20260605T110903-0500/rollback/QSL_BACKUP_PLAN.md.rollback`

Rollback SHA256 prefixes:

- status rollback: `036b608b75c6`
- plan rollback: `bba5e4ebad6`

Rollback metadata preserved the original owner, mode, mtime, and file size.
Rollback copies still existed after the patch and their SHA256 prefixes still
matched the pre-mutation values.

## Local doc patch summary

The proposed patch was saved to:

- `/srv/qbuild/tmp/NA0422_clean_status_plan_refresh_20260605T110903-0500/local_patch/proposed_local_docs.patch`

The applied diff was saved to:

- `/srv/qbuild/tmp/NA0422_clean_status_plan_refresh_20260605T110903-0500/status_plan/applied_local_docs.diff`

Pre-apply patch scan:

- patch paths were exactly the two authorized local docs;
- required clean log and manifest references were present;
- same-host caveat and no-backup/no-restore/qsl-backup non-mutation markers
  were present;
- high-confidence secret marker count was `0`;
- positive overclaim marker count was `0`.

Post-apply scan:

- required clean log and manifest references were present in both files;
- same-host caveats remained present in both files;
- high-confidence secret marker count was `0`;
- positive overclaim marker count was `0`.

The only feasible manual local content edits were the two authorized local
docs. An mtime scan also observed the active Codex session log updating under
the Codex log directory, separate from the manual local doc patch.

## Backup status update proof

Status file after patch:

- metadata after patch: `victor:victor` mode `664`, mtime
  `2026-06-05 11:13:35.634703002 -0500`
- SHA256 prefix after patch: `ed0f7d1be99b`

The status file now cites:

- `/backup/qsl/logs/daily-20260605T023308-0500.log`
- `/backup/qsl/manifests/daily-20260605T023308-0500.manifest.txt`

The status file now states that:

- the prior NA-0414/NA-0407 rsync code 23 warning was cleared in the reviewed
  scheduled same-host log;
- Codex ops remained manifest-present exactly once;
- NA-0422 did not run backup or restore;
- NA-0422 did not mutate `/usr/local/sbin/qsl-backup`;
- future off-host backup, restore/key-custody proof, disaster recovery
  evidence, and durable Director State Index storage remain separate evidence
  lanes.

## Backup plan update proof

Backup plan file after patch:

- metadata after patch: `victor:victor` mode `664`, mtime
  `2026-06-05 11:13:35.635703038 -0500`
- SHA256 prefix after patch: `fc9ff1d85544`

The backup plan now cites:

- `/backup/qsl/logs/daily-20260605T023308-0500.log`
- `/backup/qsl/manifests/daily-20260605T023308-0500.manifest.txt`

The backup plan now states that:

- the prior NA-0414/NA-0407 rsync code 23 warning was cleared in the reviewed
  scheduled same-host log;
- Codex ops remained manifest-present exactly once;
- NA-0422 did not run backup or restore;
- NA-0422 did not mutate `/usr/local/sbin/qsl-backup`;
- NA-0422 did not change backup architecture or source-list behavior;
- future off-host backup, restore/key custody, disaster recovery evidence, and
  durable Director State Index storage remain separate evidence lanes.

## Same-host/no-public-overclaim caveat proof

Both updated local files preserve that the clean scheduled log is same-host
continuity evidence only.

The update does not claim:

- off-host backup completion;
- disaster recovery completion;
- restore proof;
- complete backup coverage;
- production readiness;
- public internet readiness;
- external review completion;
- public technical paper completion;
- bug-free status;
- vulnerability-free status;
- perfect crypto.

## No backup/no restore/no mutation proof

Codex did not run backup, did not run restore, did not run sudo, and did not
run generated operator packet scripts.

Read-only qsl-backup proof after local patch:

- `/usr/local/sbin/qsl-backup` SHA256 prefix: `e9ecff3d22ed`
- `/home/victor/work/qsl/codex/ops` source inclusion count: `1`

Codex did not mutate:

- `/usr/local/sbin/qsl-backup`;
- `/backup/qsl`;
- the NA-0407 rollback subtree;
- qwork/qstart/qresume/qshell;
- systemd units, timers, fstab, source lists, retention, or backup scripts;
- qsl-server;
- qsl-attachments;
- qshield runtime;
- website, public docs, README, or START_HERE.

## Dependency-health proof

Dependency health passed before mutation:

- `cargo audit --deny warnings`: passed
- `cargo tree -i rustls-webpki --locked`: `rustls-webpki v0.103.13`
- `cargo tree -i pqcrypto-mlkem --locked || true`: package absent
- `cargo tree -i pqcrypto-traits --locked || true`: package absent
- `cargo tree -i pqcrypto-internals --locked || true`: package absent

The cargo commands emitted non-fatal package-cache/advisory lock waiting
messages before completing. Final results were green.

## Selected successor

Because the local status/plan refresh succeeded and rollback was not needed,
NA-0422 selects:

`NA-0423 -- QSL Domain Stewardship / Director Workflow Governance Authorization Plan`

Rationale:

- The backup/log-code cleanup chain is complete enough to return to project
  process design.
- The user requested stewardship review after finishing the current
  backup/log-code chain.
- The next lane should define advisory domain stewards without changing the
  exactly-one-READY queue invariant or Lead Director final authority.

## Rejected alternatives

- `NA-0423 -- QSL Backup Status / Plan Clean Refresh Retry Plan`: rejected
  because the local status/plan refresh succeeded and rollback was not needed.
- `NA-0423 -- QSL Backup Log Code 23 Clean Proof Regression Triage Plan`:
  rejected because the latest scheduled same-host log/manifest pair remained
  clean and no newer regression evidence existed.
- Public technical paper or website work: rejected as out of scope and not
  supported by internal same-host local-ops evidence.
- Deep code/crypto audit: rejected for immediate successor timing because the
  current directive selected stewardship/workflow governance after this
  cleanup chain.

## Backup-impact statement

NA-0422 changed backup status and plan wording only. It did not change backup
architecture, source lists, exclusions, retention, timers, mounts, scripts, or
restore procedures.

The updated status/plan files remain same-host continuity documents. They do
not close the residual off-host backup, disaster recovery, restore/key custody,
or durable Director State Index evidence lanes.

## Next recommendation

Proceed to NA-0422 closeout only after this evidence PR merges and post-merge
public-safety is green. The closeout should mark NA-0422 DONE and restore the
selected stewardship governance NA-0423 block as the sole READY item without
implementing NA-0423.
