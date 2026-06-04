Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-04

# NA-0415 Backup Log Code 23 Temp Rollback Subtree Review Testplan

Goals: G4

## Purpose

Validate that NA-0415 identifies the scheduled same-host qsl-backup code 23
source read-only, preserves rollback evidence and same-host caveats, selects the
correct NA-0416 successor, and avoids backup, restore, qsl-backup, temp subtree,
status/plan, runtime, workflow, public docs, website, qsl-server, and
qsl-attachments mutation.

## Scope

Allowed qsl-protocol changes:

- `docs/governance/evidence/NA-0415_qsl_backup_log_code_23_permission_denied_temp_rollback_subtree_review_plan.md`
- `tests/NA-0415_qsl_backup_log_code_23_permission_denied_temp_rollback_subtree_review_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Allowed read-only local paths:

- qwork proof files under `/srv/qbuild/work/NA-0415/.qwork/`
- `/backup/qsl/logs`
- `/backup/qsl/manifests`
- `/srv/qbuild/tmp`
- the NA-0407 packet and rollback paths
- `/usr/local/sbin/qsl-backup`
- `/home/victor/work/qsl/codex/ops/backup/QSL_BACKUP_STATUS.md`
- `/home/victor/work/qsl/codex/QSL_BACKUP_PLAN.md`
- `/home/victor/work/qsl/codex/ops`

Allowed local temp output:

- `/srv/qbuild/tmp/NA0415_backup_log_code23_review_<timestamp>/`

Forbidden scope includes backup execution, restore execution, qsl-backup
mutation, temp rollback subtree mutation, backup status/plan mutation, systemd,
timer, fstab, source-list, retention, backup script, qwork/qstart/qresume/qshell,
runtime, protocol, crypto, dependency, workflow, qsl-server, qsl-attachments,
qshield runtime, website, public docs, README, START_HERE, public technical
paper, secret handling, and public-claim expansion.

## Required Assertions

- qwork proof files exist, parse, and match live repo state.
- Codex does not run qwork, qstart, or qresume.
- PR #1097 is merged at `68ab384961c1`.
- READY_COUNT remains `1` and READY remains NA-0415 before closeout.
- NA-0414 is DONE.
- D-0815 exists once.
- D-0816 exists once.
- D-0817 exists once after this patch.
- D-0818 is absent before optional closeout.
- Duplicate decision count remains `0`.
- qsl-backup checksum prefix remains `e9ecff3d22ed`.
- Codex ops source inclusion count in qsl-backup is exactly `1`.
- Latest scheduled manifest includes `/home/victor/work/qsl/codex/ops`
  exactly once.
- Latest scheduled log code 23 source is classified as
  `CODE23_SOURCE_CONFIRMED_NA0407_ROLLBACK_SUBTREE`.
- No other latest-log rsync failure source is found.
- Temp rollback subtree is inspected read-only.
- Permission-denied checksum/read attempts are recorded as evidence, not fixed.
- No backup or restore mode is run.
- qsl-backup is not mutated.
- Temp rollback subtree is not mutated.
- Local backup status and plan docs are not mutated.
- No qsl-server or qsl-attachments path is mutated.
- No README, START_HERE, public docs, website, workflow, runtime, crypto,
  dependency, qshield runtime, or qsl-backup path is mutated.
- Selected successor is
  `NA-0416 -- QSL Backup Log Code 23 Temp Rollback Subtree Cleanup / Permission Remediation Authorization Plan`.

## Validation Commands

```bash
git diff --check
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main \
  --allowed docs/governance/evidence/NA-0415_qsl_backup_log_code_23_permission_denied_temp_rollback_subtree_review_plan.md \
  --allowed tests/NA-0415_qsl_backup_log_code_23_permission_denied_temp_rollback_subtree_review_testplan.md \
  --allowed DECISIONS.md \
  --allowed TRACEABILITY.md \
  --allowed docs/ops/ROLLING_OPERATIONS_JOURNAL.md
python3 scripts/ci/qsl_evidence_helper.py link-check
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
python3 scripts/ci/qsl_evidence_helper.py pr-body-preflight --file /tmp/na0415_pr_body.md --scan-overclaims
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
- record packet and rollback path mode/owner/group
- record visible packet and rollback counts
- record permission-denied outcomes for rollback content/size/checksum attempts
- verify local status/plan docs still preserve code 23 and same-host caveats
- verify no backup, restore, qsl-backup, temp subtree, status, or plan mutation
  occurs

## Scope Guard

The qsl-protocol changed path set must be exactly the five allowed NA-0415
evidence paths.

## Public-Safety and CI

Before merge, required PR checks must pass, including `public-safety`. After
merge, public-safety must complete success on the merge commit before optional
closeout starts.

## Acceptance Criteria

- Code 23 source is classified read-only.
- Same-host continuity caveat is explicit.
- Latest manifest presence is not described as backup completion.
- No backup or restore mode is run.
- qsl-backup remains unchanged.
- Temp rollback subtree remains unchanged.
- Local backup status and plan docs remain unchanged.
- No public-readiness or backup-complete overclaim is introduced.
- Selected NA-0416 successor includes cleanup/remediation authorization,
  rollback evidence preservation, exact path scope, no-backup/no-restore,
  no-qsl-backup-mutation, no-temp-mutation, and no-public-overclaim caveats.
- Queue and decision helpers remain clean.
