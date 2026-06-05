Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-05

# NA-0420 QSL Backup Log Code 23 Post-Remediation Scheduled Backup Verification Plan

Goals: G4

## Executive summary

NA-0420 inspected the first scheduled same-host backup log and manifest pair
after the NA-0418 operator permission remediation. The reviewed scheduled pair
was:

- log: `/backup/qsl/logs/daily-20260605T023308-0500.log`
- manifest: `/backup/qsl/manifests/daily-20260605T023308-0500.manifest.txt`

Result:

- the scheduled log has zero `code 23`, zero `Permission denied`, zero
  `rsync error`, and zero nonzero-exit markers;
- the scheduled log contains no NA-0407 rollback-subtree reference;
- the scheduled manifest still contains `/home/victor/work/qsl/codex/ops`
  exactly once;
- the NA-0407 rollback subtree does not appear in the scheduled manifest;
- Codex did not run backup, restore, sudo, generated packet scripts, qwork,
  qstart, or qresume;
- Codex did not mutate qsl-backup, `/backup/qsl`, the rollback subtree, backup
  status files, or backup plan files.

Classification:

`CODE23_REMEDIATION_VERIFIED_CLEAN_SCHEDULED_LOG`

Selected successor:

`NA-0421 -- QSL Backup Log Code 23 Clean Follow-Up / Status Refresh Authorization Plan`

This is same-host scheduled-log evidence only:

- not off-host backup;
- not disaster recovery;
- not restore proof;
- not backup completion;
- not production readiness;
- not public-internet readiness;
- not external-review completion;
- not public technical paper evidence;
- not defect-free or perfect-crypto proof.

## Live NA-0420 scope

Live queue proof before patching reported exactly one READY item:

- `NA-0420 -- QSL Backup Log Code 23 Post-Remediation Scheduled Backup Verification Plan`
- status: READY
- goals: G1, G2, G3, G4, G5

Allowed qsl-protocol mutation paths for this evidence PR are exactly:

- `docs/governance/evidence/NA-0420_qsl_backup_log_code_23_post_remediation_scheduled_backup_verification_plan.md`
- `tests/NA-0420_qsl_backup_log_code_23_post_remediation_scheduled_backup_verification_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Allowed local writes were limited to proof files under:

- `/srv/qbuild/tmp/NA0420_post_remediation_scheduled_backup_verification_retry_20260605T083900-0500/`

Forbidden mutation scope was preserved: no qwork/qstart/qresume execution by
Codex, no sudo, no generated packet script execution, no backup, no restore, no
qsl-backup mutation, no `/backup/qsl` mutation, no rollback subtree mutation,
no backup status/plan mutation, no systemd/timer/fstab/source-list or retention
mutation, no qwork/qstart/qresume/qshell mutation, no runtime, crypto,
dependency, workflow, qsl-server, qsl-attachments, qshield runtime, website,
public docs, README, or START_HERE mutation, no durable Director State Index
output, and no public claim expansion.

Acceptance criteria:

- a scheduled log/manifest after operator remediation is inspected;
- same-host continuity caveat is preserved;
- no backup or restore is run by Codex;
- no qsl-backup or rollback subtree mutation occurs by Codex;
- no public-readiness or backup-complete overclaim is introduced;
- exactly one READY item remains until closeout.

Stop conditions included missing or stale qwork proof, PR #1108 not merged,
`origin/main` not equal to or descended from PR #1108, queue not READY NA-0420,
D-0827 absent, D-0828 present at start, no scheduled pair after operator
remediation, cargo audit not green, qsl-backup source-list regression, any
forbidden execution or mutation, public-safety red or missing, more than one
READY item, and any public overclaim.

## qwork proof-file verification

Codex did not run qwork, qstart, or qresume.

The operator-provided qwork proof files existed:

- `/srv/qbuild/work/NA-0420/.qwork/startup.qsl-protocol.kv`
- `/srv/qbuild/work/NA-0420/.qwork/startup.qsl-protocol.json`

The `.kv` proof contained all required values: `startup_result=OK`, lane
`NA-0420`, repo `qsl-protocol`, path
`/srv/qbuild/work/NA-0420/qsl-protocol`, clean worktree/index/untracked state,
READY_COUNT `1`, queue top READY `NA-0420`, requested lane status `READY`, and
`head_equals_origin_main=yes`. The JSON proof parsed successfully and mirrored
the required `.kv` fields.

Recorded timestamps:

- local: `2026-06-05T08:33:17-05:00`
- UTC: `2026-06-05T13:33:17+00:00`

After `git fetch --all --prune`, live `HEAD` and live `origin/main` still
matched the qwork proof at `d56f2643e87d`. PR #1108 was MERGED with merge
commit `d56f2643e87d`.

Queue proof reported READY_COUNT `1`, READY `NA-0420`, and NA-0419 DONE.
Decision proof reported latest `D-0827`, D-0826 once, D-0827 once, D-0828
absent, and duplicate decision count zero.

## D259/D260 inheritance

D259 completed NA-0419 and merged:

- evidence PR #1107 at `9f376ef20fc9`;
- closeout PR #1108 at `d56f2643e87d`.

D259/D-0826 accepted the operator packet execution verification only because
packet-local result files and live read-only state supported the operator
markers. D259/D-0827 restored NA-0420 as the exact READY successor because no
post-operator scheduled pair existed yet.

D260 retried NA-0420 and correctly stopped before governance patching because
no scheduled backup log/manifest pair newer than the operator action existed.
D260 preserved the classification:

`CODE23_REMEDIATION_APPLIED_PENDING_SCHEDULED_BACKUP_PROOF`

Inherited operator action timestamp:

- local: `2026-06-04T11:23:29-05:00`
- UTC: `2026-06-04T16:23:29Z`

Inherited qsl-backup proof:

- expected qsl-backup SHA matched live `/usr/local/sbin/qsl-backup`;
- Codex ops source inclusion count in qsl-backup remained exactly `1`.

## Operator remediation state reconfirmation

Operator result path:

`/srv/qbuild/tmp/NA0418_code23_root_operator_packet_20260604T092447-05-00/operator_result/`

The directory exists as `root:victor` mode `2755` with mtime
`2026-06-04 11:23:29 -0500`.

Files present:

- `pre_action_metadata.txt`
- `post_action_metadata.txt`
- `rollback_tree_metadata_before.txt`
- `rollback_file_checksums_before.txt`

All four files are `root:victor` mode `644` and have mtimes at
`2026-06-04 11:23:29 -0500`.

Read-only live state after the operator action:

- rollback directory exists as `root:root` mode `2755`;
- expected rollback preimage file is readable without privilege;
- live rollback preimage checksum matches the inherited checksum;
- `/usr/local/sbin/qsl-backup` checksum still matches the inherited expected
  value;
- exact Codex ops source inclusion count in qsl-backup is `1`.

Codex did not run generated packet scripts, backup, restore, or sudo, and did
not mutate the rollback subtree.

## Scheduled log/manifest discovery

Operator action timestamp used for discovery:

- `2026-06-04T16:23:29Z`

Backup artifact inventory at discovery time:

- log count: `24`
- manifest count: `24`
- newer logs after operator action: `1`
- newer manifests after operator action: `1`
- newer matched scheduled pairs after operator action: `1`

Post-operator scheduled pair:

- stamp: `20260605T023308-0500`
- log mtime UTC: `2026-06-05T07:33:44.166111+00:00`
- manifest mtime UTC: `2026-06-05T07:33:12.036546+00:00`

Because this is the only matched pair after operator action, it is both the
first and latest scheduled pair for NA-0420.

## Post-remediation log/manifest verification

Reviewed scheduled log:

`/backup/qsl/logs/daily-20260605T023308-0500.log`

Log counts:

- `rsync error`: `0`
- `code 23`: `0`
- `Permission denied`: `0`
- `NA0407`: `0`
- `rollback`: `0`
- exact NA-0407 rollback directory reference: `0`
- `qsl-backup.preimage`: `0`
- `rsync:` error lines: `0`
- nonzero-exit markers: `0`

The scheduled log ends with:

`qsl-backup complete: /backup/qsl/snapshots/daily/daily-20260605T023308-0500`

Reviewed scheduled manifest:

`/backup/qsl/manifests/daily-20260605T023308-0500.manifest.txt`

Manifest counts:

- `/home/victor/work/qsl/codex/ops`: `1`
- `NA0407`: `0`
- `rollback`: `0`
- exact NA-0407 rollback directory reference: `0`
- `qsl-backup.preimage`: `0`

Determination:

- the code 23 warning cleared in the first scheduled same-host log after the
  operator permission remediation;
- Codex ops remains manifest-present exactly once;
- the reviewed log contains no new warning or failure marker;
- the NA-0407 rollback subtree does not appear in the reviewed manifest;
- the reviewed pair is same-host scheduled backup evidence only.

## Status/plan impact review

Read-only files inspected:

- `/home/victor/work/qsl/codex/ops/backup/QSL_BACKUP_STATUS.md`
- `/home/victor/work/qsl/codex/QSL_BACKUP_PLAN.md`

Both files still carry the older NA-0414 code 23 caveat. Because NA-0420 now
has a clean post-remediation scheduled log, a future status/plan refresh is
warranted. This lane did not mutate either file. The future refresh should
preserve the same-host caveat and state that manifest presence is not backup
completion, is not off-host backup, and is not restore proof.

## Classification

Classification:

`CODE23_REMEDIATION_VERIFIED_CLEAN_SCHEDULED_LOG`

Reason: the first scheduled same-host log and manifest pair after the operator
action exists and the reviewed log contains zero code 23, permission-denied,
rsync-error, or nonzero-exit markers.

## Selected successor

Selected successor:

`NA-0421 -- QSL Backup Log Code 23 Clean Follow-Up / Status Refresh Authorization Plan`

Reason: code 23 cleared and no new backup failure was found.

NA-0421 must not implement status/plan mutation unless its live scope
explicitly authorizes exact files and exact wording.

## Future path/scope bundle

Future possible local mutable paths only if exact future scope authorizes:

- `/home/victor/work/qsl/codex/ops/backup/QSL_BACKUP_STATUS.md`
- `/home/victor/work/qsl/codex/QSL_BACKUP_PLAN.md`

Future read-only evidence paths:

- `/backup/qsl/logs`
- `/backup/qsl/manifests`
- `/srv/qbuild/tmp/NA0418_code23_root_operator_packet_20260604T092447-05-00/operator_result`
- `/usr/local/sbin/qsl-backup`

Future forbidden unless exact scope authorizes:

- backup execution;
- restore execution;
- qsl-backup mutation;
- rollback subtree mutation;
- qwork/qstart/qresume/qshell mutation;
- runtime/dependency/workflow changes;
- public docs or website changes;
- public claim expansion.

## Public claim/external review/website boundary

This scheduled backup verification is internal local-ops evidence only.

Even with a clean scheduled same-host log, this evidence is not disaster
recovery, is not off-host backup completion, is not restore proof, is not
external review, is not public technical paper work, is not production
readiness, is not public-internet readiness, is not backup completion, and is
not a vulnerability-free/perfect-crypto claim.

No README, START_HERE, public docs, website, or security policy update was made.

## No backup/no restore/no mutation proof

Codex did not run backup, restore, sudo, generated packet scripts, qwork,
qstart, or qresume.

Mutation boundaries preserved:

- no `/backup/qsl` mutation by Codex;
- no `/usr/local/sbin/qsl-backup` mutation;
- no rollback subtree mutation by Codex;
- no backup status file mutation;
- no backup plan file mutation;
- no systemd/timer/fstab/source-list/retention mutation;
- no qwork/qstart/qresume/qshell mutation;
- no qsl-server/qsl-attachments/qshield runtime/website/public-doc/README/
  START_HERE mutation.

Allowed proof-root writes:

- copied qwork proof files into the proof root;
- copied the reviewed scheduled log and manifest into the proof root.

## Dependency-health proof

Dependency health remained green on current `origin/main` before evidence
patching:

- `cargo audit --deny warnings` exited `0`;
- `cargo tree -i rustls-webpki --locked` reported `rustls-webpki v0.103.13`;
- `cargo tree -i pqcrypto-mlkem --locked || true` reported package ID not
  found;
- `cargo tree -i pqcrypto-traits --locked || true` reported package ID not
  found;
- `cargo tree -i pqcrypto-internals --locked || true` reported package ID not
  found.

The pqcrypto absence probes were first run without the directive's `|| true`
wrapper and returned nonzero with the expected package-not-found message. That
was classified as a valid zero-match proof plus command-shape recovery. The
commands were rerun in the required zero-failure-safe form and exited `0`.

## Rejected alternatives

- Select persistent-warning triage: rejected because the scheduled log contains
  zero code 23 and zero permission-denied markers.
- Select new-failure triage: rejected because the scheduled log contains no
  rsync-error or nonzero-exit marker.
- Select ambiguity resolution: rejected because exactly one post-operator
  scheduled pair exists, the log/manifest names match, and the log result is
  clear.
- Update backup status/plan directly in NA-0420: rejected because current
  scope permits read-only status/plan inspection only.
- Treat the clean same-host log as broader backup proof: rejected because this
  lane does not run restore and does not inspect off-host storage.

## Backup-impact statement

NA-0420 changes only qsl-protocol governance evidence, a testplan, decision
metadata, traceability, and rolling-journal text. It does not change backup
configuration or backup execution behavior.

Because the clean scheduled log changes the local-ops evidence state, a future
NA-0421 status/plan authorization lane is warranted. That future lane must keep
the same-host continuity caveat and must not turn manifest presence into a
backup-completion or restore-proof claim.

## Next recommendation

After this PR merges and post-merge public-safety is green, run the optional
closeout to mark NA-0420 DONE and restore the selected NA-0421 clean follow-up
status/plan authorization lane as the sole READY item. Do not implement NA-0421
inside the NA-0420 evidence PR.
