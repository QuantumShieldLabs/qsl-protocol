Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-05

# NA-0419 QSL Backup Log Code 23 Operator Packet Execution Verification Resume

Goals: G4

## Executive Summary

NA-0419 resumed the operator-packet execution verification that D256 stopped
before governance acceptance. The human operator markers from the NA-0418
packet are accepted only because the packet-local `operator_result` files and
live read-only state support them.

Result:

- operator result files exist and support the apply/verify markers;
- rollback directory changed from `root:root` mode `2700` before action to
  `root:root` mode `2755` after action;
- the rollback file checksum can now be read without privilege and matches the
  operator-result checksum;
- `/usr/local/sbin/qsl-backup` remains unchanged and its Codex ops source
  inclusion count remains exactly `1`;
- Codex did not run sudo, generated packet scripts, backup, or restore;
- no scheduled log or manifest newer than the operator action exists yet.

Classification:

`CODE23_REMEDIATION_APPLIED_PENDING_SCHEDULED_BACKUP_PROOF`

Selected successor:

`NA-0420 -- QSL Backup Log Code 23 Post-Remediation Scheduled Backup Verification Plan`

This is same-host log remediation evidence only. It is not off-host backup,
disaster recovery, restore proof, backup completion, production readiness,
public-internet readiness, external review, public technical paper evidence, or
a claim of defect-free or perfect cryptography.

## Live NA-0419 Scope

Live queue proof before patching reported exactly one READY item:

- `NA-0419 -- QSL Backup Log Code 23 Operator Packet Execution Verification Resume`
- status: READY
- goals: G1, G2, G3, G4, G5

Allowed qsl-protocol mutation paths for this evidence PR are exactly:

- `docs/governance/evidence/NA-0419_qsl_backup_log_code_23_operator_packet_execution_verification_resume.md`
- `tests/NA-0419_qsl_backup_log_code_23_operator_packet_execution_verification_resume_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Allowed local writes were limited to proof files under:

- `/srv/qbuild/tmp/NA0419_operator_packet_verification_20260604T202016-05-00/`

Forbidden mutation scope was preserved: no qwork/qstart/qresume execution by
Codex, no sudo, no generated packet script execution, no backup, no restore, no
qsl-backup mutation, no `/backup/qsl` mutation, no NA-0407 rollback subtree
mutation, no backup status/plan mutation, no systemd/timer/fstab/source-list or
retention mutation, no qwork/qstart/qresume/qshell mutation, no runtime,
crypto, dependency, workflow, qsl-server, qsl-attachments, qshield runtime,
website, public docs, README, or START_HERE mutation, no durable Director State
Index output, and no public claim expansion.

Acceptance criteria:

- operator apply/verify markers are accepted only with live-state support;
- qsl-backup unchanged proof is recorded;
- rollback directory post-action state is recorded;
- same-host continuity caveat is preserved;
- no backup or restore is run by Codex;
- no qsl-backup or rollback subtree mutation occurs by Codex;
- no public-readiness or backup-complete overclaim is introduced;
- exactly one READY item remains until closeout.

Stop conditions included missing or stale qwork proof, PR #1106 not merged,
queue not READY NA-0419, D-0826 already present at start, missing
operator_result, unsupported operator markers, cargo audit not green,
qsl-backup source-list regression, any forbidden execution or mutation, public
safety red or missing, more than one READY item, and any public overclaim.

## qwork Proof-File Verification

Codex did not run qwork, qstart, or qresume.

The operator-provided qwork proof files existed:

- `/srv/qbuild/work/NA-0419/.qwork/startup.qsl-protocol.kv`
- `/srv/qbuild/work/NA-0419/.qwork/startup.qsl-protocol.json`

The `.kv` proof contained all required values: `startup_result=OK`, lane
`NA-0419`, repo `qsl-protocol`, path
`/srv/qbuild/work/NA-0419/qsl-protocol`, clean worktree/index/untracked state,
READY_COUNT `1`, queue top READY `NA-0419`, requested lane status `READY`, and
`head_equals_origin_main=yes`. The JSON proof parsed successfully and mirrored
the required `.kv` fields.

Recorded timestamps:

- local: `2026-06-04T20:18:38-05:00`
- UTC: `2026-06-05T01:18:38+00:00`

After `git fetch --all --prune`, live `HEAD` and live `origin/main` still
matched the qwork proof at `5cc99eaa4a0b`. PR #1106 was MERGED with merge
commit `5cc99eaa4a0b`.

Queue proof reported READY_COUNT `1`, READY `NA-0419`, and NA-0418 DONE.
Decision proof reported latest `D-0825`, D-0824 once, D-0825 once, D-0826
absent, and duplicate decision count zero.

## D255 Packet Generation Inheritance

D255 generated the NA-0418 root-operator packet under:

`/srv/qbuild/tmp/NA0418_code23_root_operator_packet_20260604T092447-05-00/`

Inherited facts:

- the packet was generated and statically validated only;
- Codex did not execute generated scripts;
- Codex did not run sudo, backup, or restore;
- qsl-backup was not mutated;
- `/backup/qsl` and the NA-0407 rollback subtree were not mutated by Codex;
- the packet was designed to preserve rollback metadata and file checksums
  before changing only the rollback directory mode from `2700` to `2755`;
- the packet did not print rollback file content.

## D256 Dependency-Health Stop Inheritance

D256 observed the operator markers in directive history but stopped before
governance acceptance because `cargo audit --deny warnings` failed on the
pqcrypto RustSec dependency-health blocker. D256 explicitly did not complete
the live operator-result support review and did not add D-0824.

Inherited operator-output markers from D256:

- `NA0418_OPERATOR_PACKET_APPLY_OK`
- `NA0418_OPERATOR_PACKET_VERIFY_OK`
- `NA0418_OPERATOR_PACKET_NO_BACKUP_EXECUTED`
- `NA0418_OPERATOR_PACKET_NO_RESTORE_EXECUTED`
- `NA0418_OPERATOR_PACKET_QSL_BACKUP_UNCHANGED`
- `NA0418_OPERATOR_PACKET_ROLLBACK_EVIDENCE_PRESERVED`

## D257 Remediation Inheritance

D257 remediated the cargo-audit blocker in PR #1105, merged as
`ee0fd66447a8`, by replacing the runtime/security-critical pqcrypto ML-KEM
provider with the maintained `ml-kem` provider while preserving the owned
provider API boundary. It rejected an audit waiver because the affected crates
were runtime/security-critical reachable.

Inherited dependency-health facts:

- `cargo audit --deny warnings` was green after remediation;
- `rustls-webpki` remained at `v0.103.13`;
- `pqcrypto-mlkem`, `pqcrypto-traits`, and `pqcrypto-internals` were absent
  from the root workspace tree;
- operator-packet verification remained pending and was not implemented by the
  dependency remediation PR.

## D258 Closeout Inheritance

D258 closed NA-0418 after PR #1105 and public-safety were green, then restored
NA-0419 as the sole READY successor in PR #1106, merged at `5cc99eaa4a0b`.

Inherited queue facts:

- NA-0418 is DONE;
- NA-0419 is READY;
- D-0825 exists once;
- D-0826 was absent at NA-0419 start;
- dependency health is restored;
- operator-packet verification remained residual work for this lane.

## Operator Output Marker Verification

The D256 response history includes all six required operator-output markers.
NA-0419 does not treat those markers as sufficient by themselves. They are
accepted only because packet-local files and live read-only state support them:

- `pre_action_metadata.txt` records the pre-action qsl-backup checksum and
  rollback directory as `root:root` mode `2700`;
- `post_action_metadata.txt` records the post-action qsl-backup checksum as
  unchanged and rollback directory as `root:root` mode `2755`;
- `rollback_tree_metadata_before.txt` records the pre-action rollback directory
  and file metadata;
- `rollback_file_checksums_before.txt` records the rollback file checksum;
- live rollback state now matches the expected post-action owner/mode.

## Operator Result File Review

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

Recorded content review:

- pre-action qsl-backup checksum matched the expected qsl-backup checksum;
- post-action qsl-backup checksum matched the pre-action checksum;
- pre-action rollback directory state was `root:root` mode `2700`;
- post-action rollback directory state was `root:root` mode `2755`;
- rollback file checksum evidence is present;
- rollback file content was not printed;
- no secret-looking content was found in the reviewed result files;
- metadata and checksum files support the rollback-evidence-preserved marker.

## Live Post-Operator State Verification

Read-only live state after the operator action:

- rollback parent exists as `victor:victor` mode `2755`;
- rollback directory exists as `root:root` mode `2755`;
- the rollback file is now readable without privilege through the remediated
  directory mode;
- the live rollback file checksum matches the operator-result checksum;
- qsl-backup checksum still matches expected value
  `e9ecff3d22ed...f6232`;
- exact Codex ops source inclusion count in qsl-backup is `1`;
- packet script / backup / restore process scan found no matching process;
- no newer backup log or manifest was generated during NA-0419 evidence
  collection by Codex.

The process scan returned no matches. That zero-match result is valid proof,
not a failure.

## Latest Log/Manifest Status After Operator Action

Operator action time, derived from operator-result mtimes and metadata:

- local: `2026-06-04 11:23:29 -0500`
- UTC: `2026-06-04T16:23:29+00:00`

Latest scheduled log:

- `/backup/qsl/logs/daily-20260604T023542-0500.log`
- mtime: `2026-06-04T02:36:09 -0500`

Latest scheduled manifest:

- `/backup/qsl/manifests/daily-20260604T023542-0500.manifest.txt`
- mtime: `2026-06-04T02:35:44 -0500`

No log or manifest under `/backup/qsl/logs` or `/backup/qsl/manifests` is newer
than the operator action time.

Latest pre-operator log counts:

- `rsync error`: `1`
- `code 23`: `1`
- `Permission denied`: `1`
- `NA0407`: `1`
- `rollback`: `1`
- exact rollback directory reference: `1`

Latest pre-operator manifest counts:

- Codex ops source path count: `1`
- `NA0407`: `0`
- `rollback`: `0`

Because the latest scheduled log predates the operator action, it cannot prove
whether code 23 cleared after remediation.

## Status/Plan Impact Review

Read-only status/plan review:

- `/home/victor/work/qsl/codex/ops/backup/QSL_BACKUP_STATUS.md`
- `/home/victor/work/qsl/codex/QSL_BACKUP_PLAN.md`

Both files still truthfully carry the NA-0414 caveat: Codex ops is included in
the same-host source list, but the latest reviewed scheduled log is code-23
caveated due to the NA-0407 rollback subtree. Since no newer scheduled log
exists after the operator action, status/plan updates should wait for NA-0420
scheduled-backup proof.

No backup status or backup plan file was mutated by NA-0419.

## Classification

`CODE23_REMEDIATION_APPLIED_PENDING_SCHEDULED_BACKUP_PROOF`

Reason:

- the operator action is supported by live state;
- qsl-backup is unchanged;
- rollback evidence is preserved and now readable without privilege;
- no scheduled backup log/manifest exists after the operator action.

The remediation is applied, but code 23 is not cleared until a later scheduled
backup log proves it.

## Selected Successor

Selected exact successor:

`NA-0420 -- QSL Backup Log Code 23 Post-Remediation Scheduled Backup Verification Plan`

This successor is selected because no newer scheduled backup exists after the
operator action. NA-0420 must inspect the next scheduled log and manifest
without running backup or restore.

## Future Path/Scope Bundle

Future allowed read-only paths for the selected NA-0420 lane:

- `/backup/qsl/logs`
- `/backup/qsl/manifests`
- `/srv/qbuild/tmp/NA0418_code23_root_operator_packet_20260604T092447-05-00/operator_result`
- `/usr/local/sbin/qsl-backup`
- qsl-protocol governance/testplan paths for NA-0420

Future forbidden unless exact future scope authorizes otherwise:

- backup execution;
- restore execution;
- qsl-backup mutation;
- rollback subtree mutation;
- backup status or backup plan mutation;
- qwork/qstart/qresume/qshell mutation;
- runtime, dependency, workflow, public docs, website, README, or START_HERE
  mutation;
- public overclaim.

## Public Claim/External Review/Website Boundary

NA-0419 is internal local-ops evidence only.

Boundaries preserved:

- permission remediation is not scheduled-backup clean proof until a newer
  scheduled log proves it;
- same-host continuity is not disaster recovery;
- manifest presence is not backup completion;
- this is not off-host backup completion;
- this is not restore proof;
- this is not external review;
- this is not public technical paper work;
- this is not production readiness or public-internet readiness;
- no README, START_HERE, public docs, website, or security policy update is
  authorized;
- cargo audit being green is dependency-health evidence only, not a universal
  security or defect-absence proof.

## No Backup/No Restore/No Sudo-by-Codex Proof

Codex did not run:

- qwork, qstart, or qresume;
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
- systemd units, timers, fstab, source lists, retention, or backup scripts.

The only local writes were proof files under the allowed NA-0419 proof root.

## Dependency-Health Proof

Dependency-health checks on current main passed:

- `cargo audit --deny warnings` exited `0`;
- `cargo tree -i rustls-webpki --locked` reported `rustls-webpki v0.103.13`;
- inverse-tree checks for `pqcrypto-mlkem`, `pqcrypto-traits`, and
  `pqcrypto-internals` returned package-ID-not-found output, which is valid
  absence proof after the D257 remediation.

Non-fatal warnings:

- parallel cargo commands printed package-cache/advisory-db lock waiting
  messages before completing successfully.

## Rejected Alternatives

- Treating the operator markers alone as proof: rejected because NA-0419
  requires live-state support.
- Calling code 23 cleared now: rejected because no scheduled log after the
  operator action exists yet.
- Updating backup status/plan files now: rejected because the current status
  should wait for scheduled backup proof.
- Running a backup manually to create proof: rejected because backup execution
  is forbidden in this lane.
- Running generated packet scripts again: rejected because Codex execution of
  generated operator scripts is forbidden.
- Mutating rollback permissions or qsl-backup: rejected as outside scope and
  unnecessary for verification.

## Backup-Impact Statement

NA-0419 does not change backup configuration, source lists, retention,
systemd/timer/fstab state, qsl-backup, `/backup/qsl`, local status/plan files,
or rollback subtree contents. It records that the operator permission
remediation was applied and that scheduled proof remains pending.

## Next Recommendation

Merge this evidence PR after validation and public-safety pass. If post-merge
public-safety is green, close out NA-0419 and restore NA-0420 as the sole READY
item with the scheduled-backup verification scope selected above.
