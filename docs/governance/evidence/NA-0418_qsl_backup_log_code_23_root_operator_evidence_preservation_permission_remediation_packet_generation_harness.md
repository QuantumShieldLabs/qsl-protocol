Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-04

# NA-0418 QSL Backup Log Code 23 Root Operator Evidence Preservation Permission Remediation Packet Generation Harness

Goals: G4

## Executive Summary

NA-0418 generated a bounded, no-secret root-operator packet for the active
scheduled same-host backup log code 23 warning caused by the NA-0407 rollback
directory. Codex generated and statically validated the packet only. Codex did
not run sudo, did not execute generated scripts, did not run backup modes, did
not run restore modes, did not mutate `/usr/local/sbin/qsl-backup`, did not
mutate `/backup/qsl`, and did not mutate the rollback subtree.

Generated packet:

`/srv/qbuild/tmp/NA0418_code23_root_operator_packet_20260604T092447-05-00/`

Classification remains:

`CODE23_STILL_ACTIVE_NA0407_ROLLBACK_SUBTREE`

The packet is now waiting for human root-operator execution. NA-0418 remains
READY until the operator apply/verify output is reviewed under a later
directive.

## Live NA-0418 Scope

Live `NEXT_ACTIONS.md` showed one READY item:

- `NA-0418 -- QSL Backup Log Code 23 Root Operator Evidence Preservation / Permission Remediation Packet Generation Harness`
- status: READY
- goals: G1, G2, G3, G4, G5

Allowed qsl-protocol mutation paths for this evidence PR are exactly:

- `docs/governance/evidence/NA-0418_qsl_backup_log_code_23_root_operator_evidence_preservation_permission_remediation_packet_generation_harness.md`
- `tests/NA-0418_qsl_backup_log_code_23_root_operator_evidence_preservation_permission_remediation_packet_generation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Allowed local writes were limited to:

- proof root: `/srv/qbuild/tmp/NA0418_packet_generation_proof_20260604T092447-05-00/`
- packet root: `/srv/qbuild/tmp/NA0418_code23_root_operator_packet_20260604T092447-05-00/`

Forbidden actions were preserved: no qwork/qstart/qresume execution by Codex,
no sudo, no generated script execution, no backup or restore execution, no
qsl-backup mutation, no `/backup/qsl` mutation, no rollback subtree mutation by
Codex, no backup status or plan mutation, no backup script/systemd/timer/fstab
mutation, no durable Director State Index output, no runtime/protocol/crypto
dependency/workflow mutation, no qsl-server or qsl-attachments mutation, no
public docs/website/README/START_HERE mutation, no public technical paper work,
and no public-claim expansion.

## qwork Proof-File Verification

Codex did not run qwork, qstart, or qresume.

The operator-provided qwork proof files existed:

- `/srv/qbuild/work/NA-0418/.qwork/startup.qsl-protocol.kv`
- `/srv/qbuild/work/NA-0418/.qwork/startup.qsl-protocol.json`

The `.kv` proof contained the required values: `startup_result=OK`, lane
`NA-0418`, repo `qsl-protocol`, path
`/srv/qbuild/work/NA-0418/qsl-protocol`, clean worktree/index/untracked state,
READY_COUNT `1`, queue top READY `NA-0418`, and requested lane status `READY`.
The JSON proof parsed successfully and mirrored the required `.kv` fields.

After `git fetch --all --prune`, live `HEAD` and live `origin/main` still
matched the qwork proof at `72ccd9a7cd68`. PR #1103 was MERGED with merge
commit `72ccd9a7cd68`. Queue proof reported READY_COUNT `1`, READY `NA-0418`,
and NA-0417 DONE. Decision proof reported latest `D-0822`, D-0821 once,
D-0822 once, D-0823 absent, and duplicate decision count zero.

Public-safety was required by branch protection and completed success on
current `origin/main`. Hard-start dependency checks passed:

- `cargo audit --deny warnings`
- `cargo tree -i rustls-webpki --locked` reported `rustls-webpki v0.103.13`

## NA-0417 Inheritance

NA-0417 selected NA-0418 after classifying the latest scheduled same-host
warning as `CODE23_STILL_ACTIVE_NA0407_ROLLBACK_SUBTREE`.

Inherited facts:

- latest reviewed log: `/backup/qsl/logs/daily-20260604T023542-0500.log`
- latest reviewed manifest: `/backup/qsl/manifests/daily-20260604T023542-0500.manifest.txt`
- exactly one latest-log code 23 result
- exactly one permission-denied source naming the NA-0407 rollback directory
- rollback directory existed as `root:root` mode `2700`
- qsl-backup checksum prefix matched `e9ecff3d22ed`
- qsl-backup Codex ops source inclusion count was exactly `1`
- Codex ops manifest presence remained same-host manifest evidence only
- no off-host, disaster-recovery, restore, backup-complete, or public-readiness
  claim was inherited

## Latest Log / Manifest Reconfirmation

Latest scheduled pair during NA-0418:

- log: `/backup/qsl/logs/daily-20260604T023542-0500.log`
- manifest: `/backup/qsl/manifests/daily-20260604T023542-0500.manifest.txt`

Latest log counts:

- `rsync error`: `1`
- `code 23`: `1`
- `Permission denied`: `1`
- `NA0407`: `1`
- `rollback`: `1`
- `qsl-backup.preimage`: `0`
- exact rollback directory reference: `1`

Minimal identifying lines:

```text
rsync: [sender] opendir "/srv/qbuild/tmp/NA0407_qsl_backup_root_action_20260602T232945-0500/rollback" failed: Permission denied (13)
rsync error: some files/attrs were not transferred (see previous errors) (code 23) at main.c(1347) [sender=3.2.7]
```

Latest manifest counts:

- Codex ops path count: `1`
- NA-0407 count: `0`
- rollback count: `0`

No additional latest-log backup failure source was found. Classification:

`CODE23_STILL_ACTIVE_NA0407_ROLLBACK_SUBTREE`

## Packet Preimage / Static Source Review

Read-only preimage review found:

- packet path exists:
  `/srv/qbuild/tmp/NA0407_qsl_backup_root_action_20260602T232945-0500`
- rollback path exists:
  `/srv/qbuild/tmp/NA0407_qsl_backup_root_action_20260602T232945-0500/rollback`
- packet path owner/mode: `victor:victor` `2755`
- rollback path owner/mode: `root:root` `2700`
- rollback path read/search access by Codex: false
- qsl-backup current checksum prefix: `e9ecff3d22ed`
- qsl-backup expected checksum prefix: `e9ecff3d22ed`
- qsl-backup Codex ops source inclusion count: `1`

Readable NA-0407 packet metadata identified the rollback file path pattern as
`rollback/qsl-backup.preimage.<expected-preimage-sha>`. The file itself remains
unreadable to Codex because the rollback directory is root-owned and not
searchable by the current user.

## Generated Packet Inventory

Packet path:

`/srv/qbuild/tmp/NA0418_code23_root_operator_packet_20260604T092447-05-00/`

Generated files:

- `README_OPERATOR_ACTION.md`
- `operator_packet_manifest.json`
- `checksums_before.txt`
- `preflight_report.txt`
- `expected_actions.txt`
- `apply_code23_permission_remediation.sh`
- `verify_after_operator_action.sh`
- `rollback_after_operator_action.sh`
- `expected_patch_or_mode_change.txt`

Script modes are executable. The optional `expected_patch_or_mode_change.txt`
is included to make the single expected mode change explicit.

## Packet Validation

Static validation passed without executing any generated script:

- `bash -n` passed for all three scripts
- manifest JSON parsed with `python3 -m json.tool`
- generated file set matched the required/justified packet set
- checksum inventory was recorded for all generated files
- forbidden-command scan passed for backup, restore, cleanup, push, merge,
  dependency update, and direct sudo-in-script strings
- apply and rollback scripts require root
- verify script does not require root
- all scripts contain the exact qsl-backup SHA and exact rollback path
- secret scan reported `SECRET_FINDING_COUNT 0`
- scripts do not print rollback file content
- scripts do not mutate qsl-backup
- scripts do not run backup or restore modes

## No-Execution Proof

Codex generated packet files and ran only static validation. Codex did not run:

- `sudo`
- any generated packet script
- backup modes
- restore modes
- qwork
- qstart
- qresume

Codex did not mutate:

- `/usr/local/sbin/qsl-backup`
- `/backup/qsl`
- the NA-0407 rollback subtree
- backup status files
- backup plan files
- backup scripts or service/timer configuration

## Root Operator Instructions

Operator must inspect the README:

```bash
less /srv/qbuild/tmp/NA0418_code23_root_operator_packet_20260604T092447-05-00/README_OPERATOR_ACTION.md
```

Operator apply command:

```bash
sudo bash /srv/qbuild/tmp/NA0418_code23_root_operator_packet_20260604T092447-05-00/apply_code23_permission_remediation.sh
```

Operator verify command:

```bash
bash /srv/qbuild/tmp/NA0418_code23_root_operator_packet_20260604T092447-05-00/verify_after_operator_action.sh
```

Optional rollback command:

```bash
sudo bash /srv/qbuild/tmp/NA0418_code23_root_operator_packet_20260604T092447-05-00/rollback_after_operator_action.sh
```

The operator must paste apply and verify output back to the Director before
Codex resumes.

## No Backup / No Restore / qsl-backup Non-Mutation Proof

The packet scripts verify `/usr/local/sbin/qsl-backup` checksum and do not
modify that file. The apply script changes only the rollback directory mode
from `2700` to `2755` after exact preflight matches. The optional rollback
script changes only that directory mode back to `2700`.

The packet scripts do not run backup modes, do not run restore modes, do not
delete evidence, and do not move rollback files.

## Same-Host Continuity Caveat

This work addresses a same-host scheduled log warning only. It is not off-host
backup evidence, not disaster recovery evidence, not restore proof, and not
backup-complete proof.

## Public Claim / External Review / Website Boundary

NA-0418 does not mutate public docs, website, README, START_HERE, qsl-server,
qsl-attachments, qshield runtime, runtime code, protocol code, crypto code,
dependencies, or workflows. It creates no public technical paper content and
does not claim production readiness, public-internet readiness, external review
completion, metadata-free behavior, anonymity, untraceability, bug-free status,
vulnerability-free status, or perfect crypto.

## Selected Successor or USER ACTION REQUIRED State

No successor is restored in this directive. NA-0418 remains READY until the
operator packet has been executed by the human operator and the apply/verify
output is reviewed in a later directive.

State:

`USER ACTION REQUIRED`

## Rejected Alternatives

- Run the packet now: rejected because Codex must not run generated scripts or
  root actions.
- Run backup now: rejected because this lane is packet generation only.
- Run restore now: rejected because this lane is not restore validation.
- Delete the rollback directory: rejected because rollback evidence must be
  preserved first and destructive cleanup is not authorized.
- Mutate qsl-backup: rejected because the current warning is caused by rollback
  directory permissions, and qsl-backup mutation is out of scope.
- Mark NA-0418 DONE: rejected because operator output has not been reviewed.

## Backup-Impact Statement

The packet is intended to clear the scheduled same-host code 23 warning after
operator execution by making the existing rollback directory readable and
searchable to the backup process. The packet does not prove the next scheduled
log is clean, does not run backup modes, and does not update backup status or
plan files. Any status/plan update requires later exact scope after operator
output and follow-up log evidence are reviewed.

## Next Recommendation

Human operator should inspect the packet, run apply as root, run verify, and
paste apply and verify output back to the Director. The later review directive
should keep NA-0418 READY until it verifies the operator output, confirms
qsl-backup remained unchanged, checks rollback evidence preservation markers,
and decides whether a follow-up clean-log/status-plan lane is justified.
