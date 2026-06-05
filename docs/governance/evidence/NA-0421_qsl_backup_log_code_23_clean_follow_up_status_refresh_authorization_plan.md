Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-05

# NA-0421 QSL Backup Log Code 23 Clean Follow-Up Status Refresh Authorization Plan

Goals: G4

## Executive summary

NA-0421 reviewed the current clean scheduled same-host backup evidence after
NA-0420 and authorized a later local status/plan refresh lane.

Classification:

`STATUS_REFRESH_AUTHORIZED_CLEAN_SAME_HOST_CODE23_CLEARED`

Selected successor:

`NA-0422 -- QSL Backup Log Code 23 Clean Status / Plan Refresh Implementation Harness`

Future mutable local files selected by this authorization:

- `/home/victor/work/qsl/codex/ops/backup/QSL_BACKUP_STATUS.md`
- `/home/victor/work/qsl/codex/QSL_BACKUP_PLAN.md`

This NA-0421 lane does not update those local files. It only records exact
future scope and required wording because both files still carry NA-0414
code-23 caveat wording and neither cites the clean NA-0420 scheduled proof.

The clean proof remains same-host scheduled-log evidence only. It makes no
off-host backup completion claim, no disaster recovery completion claim, no
restore proof claim, no backup completion claim, no production readiness claim,
no public-internet readiness claim, no external-review completion claim, no
public technical paper claim, no vulnerability-free claim, no bug-free claim,
and no perfect-crypto claim.

## Live NA-0421 scope

Live queue proof before patching reported exactly one READY item:

- `NA-0421 -- QSL Backup Log Code 23 Clean Follow-Up / Status Refresh Authorization Plan`
- status: READY
- goals: G1, G2, G3, G4, G5

Allowed qsl-protocol mutation paths for this evidence PR are exactly:

- `docs/governance/evidence/NA-0421_qsl_backup_log_code_23_clean_follow_up_status_refresh_authorization_plan.md`
- `tests/NA-0421_qsl_backup_log_code_23_clean_follow_up_status_refresh_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Allowed local writes were limited to proof files under:

- `/srv/qbuild/tmp/NA0421_clean_followup_status_refresh_authorization_20260605T092806-0500/`

Allowed read-only local paths included:

- qwork proof files under `/srv/qbuild/work/NA-0421/.qwork/`
- `/backup/qsl/logs`
- `/backup/qsl/manifests`
- `/srv/qbuild/tmp/NA0418_code23_root_operator_packet_20260604T092447-05-00/operator_result`
- `/srv/qbuild/tmp/NA0407_qsl_backup_root_action_20260602T232945-0500/rollback`
- `/usr/local/sbin/qsl-backup`
- `/home/victor/work/qsl/codex/ops/backup/QSL_BACKUP_STATUS.md`
- `/home/victor/work/qsl/codex/QSL_BACKUP_PLAN.md`
- `/home/victor/work/qsl/codex/ops`

Forbidden mutation scope was preserved:

- no qwork, qstart, or qresume execution by Codex;
- no sudo;
- no generated operator script execution;
- no backup;
- no restore;
- no qsl-backup mutation;
- no `/backup/qsl` mutation;
- no NA-0407 rollback subtree mutation;
- no backup status or backup plan mutation;
- no systemd, timer, fstab, source-list, retention, or backup-script mutation;
- no qwork/qstart/qresume/qshell mutation;
- no runtime, crypto, dependency, workflow, qsl-server, qsl-attachments,
  qshield runtime, website, public docs, README, or START_HERE mutation;
- no durable Director State Index output;
- no public technical paper work;
- no public overclaim.

Acceptance criteria for this lane:

- qwork proof is read and verified without rerunning qwork;
- clean scheduled log evidence is cited accurately;
- status/plan files are inspected read-only only;
- future status/plan mutable candidates and wording are exact;
- same-host continuity caveat is preserved;
- no backup or restore operation is run;
- no qsl-backup mutation occurs;
- no rollback subtree mutation occurs;
- no public-readiness or backup-complete overclaim is introduced;
- exactly one READY item remains until closeout.

Stop conditions included missing or stale qwork proof, PR #1110 not merged,
origin/main not equal to or descended from PR #1110, queue not READY NA-0421,
D-0829 absent, D-0830 present at start, clean scheduled log evidence not
verifiable, newer backup evidence regression, cargo audit not green,
qsl-backup source-list regression, any forbidden execution or mutation,
public-safety red or missing, more than one READY item, and any public
overclaim.

## qwork proof-file verification

Codex did not run qwork, qstart, or qresume.

The operator-provided qwork proof files existed:

- `/srv/qbuild/work/NA-0421/.qwork/startup.qsl-protocol.kv`
- `/srv/qbuild/work/NA-0421/.qwork/startup.qsl-protocol.json`

The `.kv` proof contained all required values:

- `startup_result=OK`
- `lane=NA-0421`
- `repo=qsl-protocol`
- `path=/srv/qbuild/work/NA-0421/qsl-protocol`
- `head_equals_origin_main=yes`
- `worktree_clean=yes`
- `index_clean=yes`
- `untracked_clean=yes`
- `ready_count=1`
- `queue_top_ready=NA-0421`
- `requested_lane_status=READY`

The JSON proof parsed successfully and mirrored the required `.kv` fields for
lane, repo, path, head, origin/main, ready count, queue top READY, requested
lane status, and clean-state values.

Recorded timestamps:

- local: `2026-06-05T09:26:40-05:00`
- UTC: `2026-06-05T14:26:40+00:00`

After `git fetch --all --prune`, live `HEAD` and live `origin/main` still
matched the qwork proof at `4b520529256c`. PR #1110 was MERGED with merge
commit `4b520529256c`.

Queue proof reported READY_COUNT `1`, READY `NA-0421`, and NA-0420 DONE.
Decision proof reported latest `D-0829`, D-0828 once, D-0829 once, D-0830
absent, and duplicate decision count zero.

The qwork proof files were copied into:

- `/srv/qbuild/tmp/NA0421_clean_followup_status_refresh_authorization_20260605T092806-0500/qwork/`

## NA-0420 inheritance

Inherited NA-0420 classification:

`CODE23_REMEDIATION_VERIFIED_CLEAN_SCHEDULED_LOG`

Inherited clean scheduled same-host pair:

- log: `/backup/qsl/logs/daily-20260605T023308-0500.log`
- manifest: `/backup/qsl/manifests/daily-20260605T023308-0500.manifest.txt`

Inherited checksums:

- log SHA256:
  `2e82582cc6a0215d07c074194cebd9e3ce18d9f7470b2dfbc6174f142bc8c0f4`
- manifest SHA256:
  `4331d00be68fe5d8ea4fba678db7a38d3c79b4cdd1318d7eb689741b070c9d23`

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

Inherited operator remediation state:

- operator action time: `2026-06-04T16:23:29Z`
- operator result path:
  `/srv/qbuild/tmp/NA0418_code23_root_operator_packet_20260604T092447-05-00/operator_result`
- rollback directory state after operator remediation: `root:root` mode `2755`
- qsl-backup SHA256:
  `e9ecff3d22eda21ceb0e889e4dd5d6f4e270e09349c77a1f4872bfc0052f6232`
- qsl-backup Codex ops source inclusion count: `1`

NA-0420 and D-0828 selected NA-0421 only for clean follow-up/status refresh
authorization. They did not authorize status/plan mutation in NA-0420.

## Clean log/manifest reconfirmation

NA-0421 reconfirmed read-only that the latest scheduled log and manifest under
`/backup/qsl/logs` and `/backup/qsl/manifests` were still:

- `/backup/qsl/logs/daily-20260605T023308-0500.log`
- `/backup/qsl/manifests/daily-20260605T023308-0500.manifest.txt`

No newer scheduled pair existed at NA-0421 review time.

Artifact metadata:

- log: `victor:victor` mode `644`, mtime
  `2026-06-05 02:33:44.166110544 -0500`
- manifest: `victor:victor` mode `644`, mtime
  `2026-06-05 02:33:12.036546521 -0500`

Reconfirmed checksums:

- log SHA256:
  `2e82582cc6a0215d07c074194cebd9e3ce18d9f7470b2dfbc6174f142bc8c0f4`
- manifest SHA256:
  `4331d00be68fe5d8ea4fba678db7a38d3c79b4cdd1318d7eb689741b070c9d23`

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

The reviewed log ended with the normal daily snapshot completion marker for
`daily-20260605T023308-0500`. That marker is same-host script output only. It
is not off-host backup proof, not disaster recovery proof, not restore proof,
and not comprehensive backup coverage proof.

Copies of the reviewed log and manifest were stored under:

- `/srv/qbuild/tmp/NA0421_clean_followup_status_refresh_authorization_20260605T092806-0500/logs/`
- `/srv/qbuild/tmp/NA0421_clean_followup_status_refresh_authorization_20260605T092806-0500/manifests/`

## Status file read-only review

Read-only file:

- `/home/victor/work/qsl/codex/ops/backup/QSL_BACKUP_STATUS.md`

Metadata:

- exists: yes
- owner/mode: `victor:victor` mode `664`
- mtime: `2026-06-03 21:44:18.951416552 -0500`
- SHA256:
  `036b608b75c6b6d0f7609b7120a0bc89ac74278ec28265cc6cfff0000afd9dea`

Review findings:

- mentions older NA-0414 code 23 caveat: yes, five NA-0414 references and
  explicit code 23 caveat sections;
- mentions NA-0420 clean scheduled proof: no;
- mentions clean log path
  `/backup/qsl/logs/daily-20260605T023308-0500.log`: no;
- mentions clean manifest path
  `/backup/qsl/manifests/daily-20260605T023308-0500.manifest.txt`: no;
- preserves same-host continuity caveat: yes;
- avoids positive off-host claims, positive disaster-recovery claims, positive
  restore-proof claims, positive backup-complete claims, positive production
  readiness claims, positive public-internet readiness claims, positive
  external-review claims, positive public technical paper claims, positive
  vulnerability-free claims, positive bug-free claims, and positive
  perfect-crypto claims: yes; references are caveated or negative.

Exact sections needing future update:

- `## Summary`, specifically the NA-0414 status update paragraph that still
  says the latest scheduled log carries the rsync code 23 caveat.
- The "Latest scheduled same-host manifest/log evidence reviewed by NA-0414"
  subsection.
- The "Latest scheduled log caveat" subsection.
- The sentence stating that the latest scheduled log remains code-23 caveated.
- `## Recommended Next Actions`, item 1, which still recommends reviewing the
  code 23 permission-denied caveat.

Future mutable candidate decision for this file: selected.

## Backup plan read-only review

Read-only file:

- `/home/victor/work/qsl/codex/QSL_BACKUP_PLAN.md`

Metadata:

- exists: yes
- owner/mode: `victor:victor` mode `664`
- mtime: `2026-06-03 21:44:18.952416565 -0500`
- SHA256:
  `bba5e4ebad6a7a673cccb18d33f132b1cea9ab5d014ebc9f8e7a0fd43d84e220`

Review findings:

- mentions older NA-0414 code 23 caveat: yes, seven NA-0414 references and
  explicit code 23 caveat sections;
- mentions NA-0420 clean scheduled proof: no;
- mentions clean log path
  `/backup/qsl/logs/daily-20260605T023308-0500.log`: no;
- mentions clean manifest path
  `/backup/qsl/manifests/daily-20260605T023308-0500.manifest.txt`: no;
- preserves same-host continuity caveat: yes;
- avoids positive off-host claims, positive disaster-recovery claims, positive
  restore-proof claims, positive backup-complete claims, positive production
  readiness claims, positive public-internet readiness claims, positive
  external-review claims, positive public technical paper claims, positive
  vulnerability-free claims, positive bug-free claims, and positive
  perfect-crypto claims: yes; references are caveated or negative.

Exact sections needing future update:

- `## 1. Executive Recommendation`, specifically the current NA-0414 status
  alignment paragraph that still says the latest scheduled log has a code 23
  caveat.
- The "Current caveated evidence" subsection.
- The sentence stating that a code-23-caveated log must not be treated as
  off-host coverage, disaster recovery, restore validation, or comprehensive
  backup coverage proof.
- `## 18. Installed Status And Backup-Impact Rule`, specifically the "Current
  Codex ops status as of NA-0414" subsection.

Future mutable candidate decision for this file: selected.

## Status refresh authorization decision

Selected classification:

`STATUS_REFRESH_AUTHORIZED_CLEAN_SAME_HOST_CODE23_CLEARED`

Rationale:

- the clean scheduled same-host log remains current;
- no newer scheduled pair exists that would reintroduce code 23 or another
  backup failure;
- the status file and plan file still describe the older NA-0414 code 23
  caveat;
- neither file cites the NA-0420 clean log or manifest;
- both files already preserve same-host and public-claim caveats, so a later
  scoped refresh can update the code 23 status without weakening the
  boundaries.

Files selected for future mutable scope:

- status file: yes;
- plan file: yes.

NA-0421 did not mutate either file.

## Selected successor

Selected successor:

`NA-0422 -- QSL Backup Log Code 23 Clean Status / Plan Refresh Implementation Harness`

Reason:

- status refresh is authorized;
- both local files need bounded wording updates;
- implementation must be isolated to the exact local status/plan files plus
  qsl-protocol governance evidence;
- no backup, restore, qsl-backup mutation, rollback mutation, or public-claim
  expansion is needed.

NA-0421 does not implement NA-0422.

## Future path/scope bundle

Future local mutable paths for NA-0422:

- `/home/victor/work/qsl/codex/ops/backup/QSL_BACKUP_STATUS.md`
- `/home/victor/work/qsl/codex/QSL_BACKUP_PLAN.md`

Future qsl-protocol scope for NA-0422:

- qsl-protocol governance evidence/testplan path for NA-0422;
- `DECISIONS.md`;
- `TRACEABILITY.md`;
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`;
- optional closeout path only if separately directed.

Future required wording:

- cite `/backup/qsl/logs/daily-20260605T023308-0500.log`;
- cite `/backup/qsl/manifests/daily-20260605T023308-0500.manifest.txt`;
- state that code 23 was cleared in the reviewed scheduled same-host log;
- state that Codex ops remained manifest-present exactly once;
- preserve that this is same-host continuity evidence only;
- preserve no off-host backup claim;
- preserve no disaster recovery claim;
- preserve no restore proof claim;
- preserve no backup-complete claim;
- preserve no public readiness claim;
- record that Codex ran no backup or restore in NA-0420 or NA-0421.

Suggested exact replacement facts for both local files:

```text
NA-0420/NA-0421 status refresh: the reviewed scheduled same-host log
`/backup/qsl/logs/daily-20260605T023308-0500.log` has zero code 23,
zero Permission denied, zero rsync error, and zero NA-0407 rollback references.
The matching manifest
`/backup/qsl/manifests/daily-20260605T023308-0500.manifest.txt` includes
`/home/victor/work/qsl/codex/ops` exactly once. This clears the previously
recorded NA-0414 code 23 caveat for the reviewed scheduled log only.
This is same-host continuity evidence only. It is not off-host backup. It is
not disaster recovery. It is not restore proof. It is not backup completion. It
is not production readiness. It is not public-internet readiness. It is not
external review. It is not public technical paper evidence.
Codex did not run backup or restore in NA-0420 or NA-0421 and did not mutate
`/usr/local/sbin/qsl-backup`.
```

Future forbidden scope unless a later exact directive authorizes it:

- backup execution;
- restore execution;
- qsl-backup mutation;
- rollback subtree mutation;
- qwork/qstart/qresume/qshell mutation;
- runtime, dependency, or workflow changes;
- qsl-server, qsl-attachments, or qshield runtime changes;
- public docs, website, README, or START_HERE changes;
- public claims.

## Future validation/marker plan

Common NA-0422 markers:

- `NA0422_CLEAN_SCHEDULED_LOG_REFERENCE_OK`
- `NA0422_CLEAN_MANIFEST_REFERENCE_OK`
- `NA0422_CODE23_CLEARED_CAVEATED_OK`
- `NA0422_CODEX_OPS_MANIFEST_PRESENT_OK`
- `NA0422_SAME_HOST_CONTINUITY_CAVEAT_OK`
- `NA0422_NO_OFF_HOST_BACKUP_CLAIM_OK`
- `NA0422_NO_DISASTER_RECOVERY_CLAIM_OK`
- `NA0422_NO_RESTORE_PROOF_CLAIM_OK`
- `NA0422_NO_BACKUP_COMPLETE_CLAIM_OK`
- `NA0422_NO_BACKUP_EXECUTION_OK`
- `NA0422_NO_RESTORE_EXECUTION_OK`
- `NA0422_NO_QSL_BACKUP_MUTATION_OK`
- `NA0422_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0422_NO_SECRET_MATERIAL_OK`
- `NA0422_STATUS_PLAN_REFRESH_IMPLEMENTATION_AUTHORIZED_OK`
- `NA0422_EXACT_LOCAL_PATHS_SELECTED_OK`

Expected validation:

- exact-path scope guard for local status/plan files and qsl-protocol paths;
- read-only checksum proof before and after local file edits;
- grep proof for clean log and manifest paths;
- grep proof for same-host and public-claim caveats;
- grep proof that no backup/restore execution claim was introduced;
- qsl-backup checksum proof;
- rollback subtree no-mutation proof;
- qwork proof-file read without rerunning qwork;
- qsl-protocol link-check, leak-scan, overclaim scan, classifier, PR body
  preflight, goal-lint, dependency health, qsc send_commit, and formal model
  checks as directed by the future lane.

## Public claim/external review/website boundary

This authorization is internal local-ops evidence only.

The clean scheduled same-host log makes no disaster recovery claim, no
off-host backup completion claim, no backup completion claim, no restore proof
claim, no external review claim, no public technical paper claim, no production
readiness claim, no public-internet readiness claim, no metadata-free behavior
claim, no anonymity or untraceability claim, no bug-free claim, no
vulnerability-free claim, and no perfect-crypto claim.

Manifest presence is source-presence evidence only and is not backup
completion. No README, START_HERE, public docs, website, security policy, or
public technical paper update is authorized by this lane.

## No backup/no restore/no mutation proof

Codex did not run:

- qwork;
- qstart;
- qresume;
- sudo;
- generated packet scripts;
- backup;
- restore.

Codex did not mutate:

- `/usr/local/sbin/qsl-backup`;
- `/backup/qsl`;
- the NA-0407 rollback subtree;
- backup status files;
- backup plan files;
- systemd units, timers, fstab, source lists, retention, or backup scripts;
- qwork/qstart/qresume/qshell;
- runtime, crypto, dependency, workflow, qsl-server, qsl-attachments, qshield
  runtime, website, public docs, README, or START_HERE paths.

Boundary proofs:

- `/usr/local/sbin/qsl-backup` SHA256:
  `e9ecff3d22eda21ceb0e889e4dd5d6f4e270e09349c77a1f4872bfc0052f6232`
- qsl-backup Codex ops source inclusion count: `1`
- operator result directory: `root:victor` mode `2755`
- live rollback directory: `root:root` mode `2755`
- final status/plan files were copied into the NA-0421 proof root for
  read-only evidence only.

## Dependency-health proof

Dependency-health checks before patching:

- `cargo audit --deny warnings`: passed;
- `cargo tree -i rustls-webpki --locked`: reported
  `rustls-webpki v0.103.13`;
- `cargo tree -i pqcrypto-mlkem --locked || true`: package ID absent;
- `cargo tree -i pqcrypto-traits --locked || true`: package ID absent;
- `cargo tree -i pqcrypto-internals --locked || true`: package ID absent.

Recovered-failure evidence:

- Failing commands: the first three pqcrypto inverse-tree commands were run
  without the directive's `|| true` suffix and exited 101 with package-ID
  absence.
- Classification: recoverable zero-match proof/command-shape issue. The
  absent package IDs are the desired proof outcome, and the directive explicitly
  wraps those commands in `|| true`.
- Corrective action: reran the three commands with `|| true`.
- Final result: all three reruns exited 0 while preserving the package-ID
  absence output.

Non-fatal warning:

- Some cargo tree commands printed package-cache lock waiting messages before
  completing. This did not change the dependency result.

Cargo audit being green is dependency-health evidence only. It is not
vulnerability-free proof. It is not bug-free proof. It is not perfect-crypto
proof.

## Rejected alternatives

`STATUS_REFRESH_NOT_REQUIRED` was rejected because both local files still cite
NA-0414 code-23 caveat wording and neither cites the NA-0420 clean scheduled
proof.

`STATUS_REFRESH_BLOCKED_CODE23_REGRESSION` was rejected because no newer
scheduled pair exists and the latest reviewed log has zero code 23 markers.

`STATUS_REFRESH_BLOCKED_NEW_BACKUP_FAILURE` was rejected because the latest
reviewed log has zero rsync error, permission-denied, nonzero-exit, rollback,
and exact rollback-path markers.

`STATUS_REFRESH_BLOCKED_EVIDENCE_AMBIGUITY` was rejected because the qwork
proof, queue proof, inherited NA-0420 evidence, latest log/manifest discovery,
hashes, and marker counts are consistent.

Updating status/plan files in NA-0421 was rejected because the directive only
allows read-only inspection and future authorization, not direct status/plan
mutation.

## Backup-impact statement

This lane improves backup-status accuracy by authorizing a later update to
remove stale code-23 caveat wording after clean scheduled same-host evidence
was verified.

It does not change the backup system, run backup, run restore, alter backup
retention, modify qsl-backup, alter source lists, alter systemd/timer/fstab
state, or change rollback evidence. It does not add off-host backup or disaster
recovery proof.

## Next recommendation

After this evidence PR is merged and public-safety is green, close NA-0421 and
restore the selected NA-0422 successor:

`NA-0422 -- QSL Backup Log Code 23 Clean Status / Plan Refresh Implementation Harness`

NA-0422 should update only the two authorized local files and its own
qsl-protocol governance evidence, preserving same-host, no-backup/no-restore,
no-qsl-backup-mutation, no-rollback-mutation, and public-claim boundaries.
