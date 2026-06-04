Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-04

# NA-0414 Backup Status Plan Update Implementation Testplan

Goals: G4

## Purpose

Validate that NA-0414 updates exactly the authorized local backup status and
backup plan files while preserving same-host continuity, the latest log rsync
code 23 caveat, no-backup/no-restore, qsl-backup non-mutation, and no public
overclaim boundaries.

## Scope

Allowed local mutable files:

- `/home/victor/work/qsl/codex/ops/backup/QSL_BACKUP_STATUS.md`
- `/home/victor/work/qsl/codex/QSL_BACKUP_PLAN.md`

Allowed qsl-protocol changes:

- `docs/governance/evidence/NA-0414_qsl_codex_ops_backup_status_plan_update_implementation_harness.md`
- `tests/NA-0414_qsl_codex_ops_backup_status_plan_update_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Read-only evidence:

- qwork proof files under `/srv/qbuild/work/NA-0414/.qwork/`
- `/backup/qsl/manifests`
- `/backup/qsl/logs`
- `/usr/local/sbin/qsl-backup`

## Required Assertions

- qwork proof files exist, parse, and match live repo state.
- Codex does not run qwork, qstart, or qresume.
- PR #1095 is merged at `2c4c2f2ebd58`.
- READY_COUNT remains `1` and READY remains NA-0414 before closeout.
- D-0814 exists once before the NA-0414 patch.
- D-0815 exists once after the NA-0414 patch.
- D-0816 is absent before optional closeout.
- Duplicate decision count remains `0`.
- qsl-backup checksum is
  `e9ecff3d22eda21ceb0e889e4dd5d6f4e270e09349c77a1f4872bfc0052f6232`.
- Codex ops source inclusion count in qsl-backup is exactly `1`.
- Latest manifest includes `/home/victor/work/qsl/codex/ops` exactly once.
- Latest log rsync code 23 caveat is preserved.
- Both local docs include same-host caveats after mutation.
- No backup or restore mode is run.
- qsl-backup is not mutated.
- No qsl-server or qsl-attachments path is mutated.
- No README, START_HERE, public docs, website, workflow, runtime, crypto,
  dependency, qshield runtime, or qsl-backup path is mutated.

## Validation Commands

```bash
git diff --check
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main \
  --allowed docs/governance/evidence/NA-0414_qsl_codex_ops_backup_status_plan_update_implementation_harness.md \
  --allowed tests/NA-0414_qsl_codex_ops_backup_status_plan_update_implementation_testplan.md \
  --allowed DECISIONS.md \
  --allowed TRACEABILITY.md \
  --allowed docs/ops/ROLLING_OPERATIONS_JOURNAL.md
python3 scripts/ci/qsl_evidence_helper.py link-check
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
python3 scripts/ci/qsl_evidence_helper.py pr-body-preflight --file /tmp/na0414_pr_body.md --scan-overclaims
cargo audit --deny warnings
cargo tree -i rustls-webpki --locked
cargo fmt --check
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
python3 formal/model_qsc_handshake_suite_id_bounded.py
python3 formal/run_model_checks.py
```

Also run goal-lint with a synthetic pull-request event containing a standalone
near-top `Goals: G4` line.

## Local Doc Checks

Required local post-patch checks:

- before/after SHA-256 recorded for both local docs
- rollback copies exist under the proof root
- local docs mention same-host continuity
- local docs mention latest scheduled log rsync code 23 caveat
- local docs state NA-0414 did not run backup or restore
- local docs state NA-0414 did not mutate qsl-backup
- local secret scan count is `0`
- local prohibited positive overclaim phrase count is `0`

## Scope Guard

The qsl-protocol changed path set must be exactly the five allowed NA-0414
evidence paths. The local changed path set must be exactly the two authorized
local docs.

## Public-Safety and CI

Before merge, required PR checks must pass, including `public-safety`. After
merge, public-safety must complete success on the merge commit before optional
closeout starts.

## Acceptance Criteria

- Both authorized local docs are updated together.
- Same-host continuity caveat is explicit.
- Latest scheduled log rsync code 23 caveat is explicit.
- No backup or restore mode is run.
- qsl-backup remains unchanged.
- No public-readiness or comprehensive backup-coverage overclaim is introduced.
- Selected successor is
  `NA-0415 -- QSL Backup Log Code 23 Permission-Denied Temp Rollback Subtree Review Plan`.
- Queue and decision helpers remain clean.
