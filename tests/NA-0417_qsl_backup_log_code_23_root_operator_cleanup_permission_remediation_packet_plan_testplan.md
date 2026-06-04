Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-04

# NA-0417 Backup Log Code 23 Root Operator Packet Plan Testplan

Goals: G4

## Purpose

Validate that NA-0417 performs only root-operator packet planning for the
scheduled same-host qsl-backup code 23 warning, preserves rollback evidence
requirements, selects the correct NA-0418 packet generation harness successor,
and avoids qwork, qstart, qresume, sudo, backup, restore, qsl-backup, temp
subtree, status/plan, runtime, workflow, public docs, website, qsl-server, and
qsl-attachments mutation.

## Scope

Allowed qsl-protocol changes:

- `docs/governance/evidence/NA-0417_qsl_backup_log_code_23_root_operator_cleanup_permission_remediation_packet_plan.md`
- `tests/NA-0417_qsl_backup_log_code_23_root_operator_cleanup_permission_remediation_packet_plan_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Allowed read-only local paths:

- qwork proof files under `/srv/qbuild/work/NA-0417/.qwork/`
- `/backup/qsl/logs`
- `/backup/qsl/manifests`
- `/srv/qbuild/tmp`
- the NA-0407 packet and rollback paths
- `/usr/local/sbin/qsl-backup`
- `/home/victor/work/qsl/codex/ops/backup/QSL_BACKUP_STATUS.md`
- `/home/victor/work/qsl/codex/QSL_BACKUP_PLAN.md`
- `/home/victor/work/qsl/codex/ops`
- `/home/victor/work/qsl/codex/responses/NA0416_20260604T074224-0500_D252.md`

Allowed local temp output:

- `/srv/qbuild/tmp/NA0417_root_operator_packet_plan_<timestamp>/`

Forbidden scope includes qwork/qstart/qresume execution by Codex, sudo, backup
execution, restore execution, qsl-backup mutation, `/backup/qsl` mutation, temp
rollback subtree mutation, executable operator packet generation, backup
status/plan mutation, systemd, timer, fstab, source-list, retention, backup
script, qwork/qstart/qresume/qshell mutation, runtime, protocol, crypto,
dependency, workflow, qsl-server, qsl-attachments, qshield runtime, website,
public docs, README, START_HERE, public technical paper, secret handling, and
public-claim expansion.

## Required Assertions

- qwork proof files exist, parse, and match live repo state.
- Codex does not run qwork, qstart, or qresume.
- PR #1101 is merged at `3bf432f123f1`.
- READY_COUNT remains `1` and READY remains NA-0417 before closeout.
- NA-0416 is DONE.
- D-0819 exists once.
- D-0820 exists once.
- D-0821 exists once after this patch.
- D-0822 is absent before optional closeout.
- Duplicate decision count remains `0`.
- qsl-backup checksum remains
  `e9ecff3d22eda21ceb0e889e4dd5d6f4e270e09349c77a1f4872bfc0052f6232`.
- Codex ops source inclusion count in qsl-backup is exactly `1`.
- Latest scheduled manifest includes `/home/victor/work/qsl/codex/ops`
  exactly once.
- Latest scheduled log status is classified as
  `CODE23_STILL_ACTIVE_NA0407_ROLLBACK_SUBTREE`.
- No other latest-log rsync failure source is found.
- Temp rollback subtree is inspected read-only.
- Permission-denied rollback inspection remains evidence, not a fix.
- No executable operator packet files are generated.
- Selected successor is
  `NA-0418 -- QSL Backup Log Code 23 Root Operator Evidence Preservation / Permission Remediation Packet Generation Harness`.
- The future NA-0418 scope requires rollback evidence preservation, exact path
  scope, no sudo by Codex, no backup, no restore, no qsl-backup mutation, no
  temp subtree mutation by Codex, no secret material, and no public overclaim.

## Validation Commands

```bash
git diff --check
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main \
  --allowed docs/governance/evidence/NA-0417_qsl_backup_log_code_23_root_operator_cleanup_permission_remediation_packet_plan.md \
  --allowed tests/NA-0417_qsl_backup_log_code_23_root_operator_cleanup_permission_remediation_packet_plan_testplan.md \
  --allowed DECISIONS.md \
  --allowed TRACEABILITY.md \
  --allowed docs/ops/ROLLING_OPERATIONS_JOURNAL.md
python3 scripts/ci/qsl_evidence_helper.py link-check
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
python3 scripts/ci/qsl_evidence_helper.py pr-body-preflight --file /tmp/na0417_pr_body.md --scan-overclaims
cargo audit --deny warnings
cargo tree -i rustls-webpki --locked
cargo fmt --check
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
python3 formal/model_qsc_handshake_suite_id_bounded.py
python3 formal/run_model_checks.py
```

Also run goal-lint with a synthetic pull-request event containing a standalone
near-top `Goals: G4` line.

## Read-Only Local Checks

Required local evidence checks:

- identify latest scheduled log and manifest
- count `rsync error`, `code 23`, `Permission denied`, `NA0407`, `rollback`,
  `qsl-backup.preimage`, and the exact NA-0407 rollback path in the latest log
- record minimal matching error lines only
- count Codex ops in the latest manifest
- record packet, rollback, and parent path mode/owner/group
- record readable packet files
- record rollback readability/searchability
- record permission-denied outcomes for rollback content inspection
- verify qsl-backup checksum and Codex ops source inclusion count
- verify local status/plan docs remain unchanged and still preserve code 23 and
  same-host caveats
- verify no backup, restore, sudo, qsl-backup, temp subtree, status, or plan
  mutation occurs

## Scope Guard

The qsl-protocol changed path set must be exactly the five allowed NA-0417
evidence paths.

## Public-Safety and CI

Before merge, required PR checks must pass, including `public-safety`. After
merge, public-safety must complete success on the merge commit before optional
closeout starts.

Use bounded REST polling only. Do not use watch modes.

## Acceptance Criteria

- Code 23 active status is classified read-only.
- Same-host continuity caveat is explicit.
- Latest manifest presence is not described as backup completion.
- Rollback evidence preservation requirements are explicit.
- Future packet generation direction is explicit and no packet files are
  generated in NA-0417.
- No backup or restore mode is run.
- No sudo is run.
- qsl-backup remains unchanged.
- Temp rollback subtree remains unchanged.
- Local backup status and plan docs remain unchanged.
- No public-readiness or backup-complete overclaim is introduced.
- Selected NA-0418 successor includes root-operator packet generation,
  rollback evidence preservation, exact path scope, no Codex sudo,
  no-backup/no-restore, no-qsl-backup-mutation, no-temp-mutation by Codex,
  no-secret, and no-public-overclaim caveats.
- Queue and decision helpers remain clean.
