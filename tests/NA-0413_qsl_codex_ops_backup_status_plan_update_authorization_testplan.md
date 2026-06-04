Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-04

# NA-0413 Backup Status Plan Update Authorization Testplan

Goals: G4

## Purpose

Validate that NA-0413 authorizes the future Codex ops backup status / plan
update lane without implementing it, running backup, running restore, mutating
qsl-backup, mutating local backup status/plan files, or expanding public
claims.

## Scope

Allowed qsl-protocol changes:

- `docs/governance/evidence/NA-0413_qsl_codex_ops_backup_status_plan_update_authorization_plan.md`
- `tests/NA-0413_qsl_codex_ops_backup_status_plan_update_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Read-only local evidence:

- `/home/victor/work/qsl/codex/ops/backup/QSL_BACKUP_STATUS.md`
- `/home/victor/work/qsl/codex/QSL_BACKUP_PLAN.md`
- `/backup/qsl/manifests`
- `/backup/qsl/logs`
- `/usr/local/sbin/qsl-backup`
- `/home/victor/work/qsl/codex/ops`
- qwork proof files under `/srv/qbuild/work/NA-0413/.qwork/`

## Required Assertions

- qwork proof files exist, parse, and match live repo state.
- Codex does not run qwork, qstart, or qresume.
- PR #1093 is merged at `82ebae4dc12c`.
- READY_COUNT remains `1` and READY remains NA-0413 before closeout.
- D-0812 exists once before the NA-0413 patch.
- D-0813 exists once after the NA-0413 patch.
- D-0814 is absent before optional closeout.
- Duplicate decision count remains `0`.
- qsl-backup checksum is
  `e9ecff3d22eda21ceb0e889e4dd5d6f4e270e09349c77a1f4872bfc0052f6232`.
- Codex ops source inclusion count in qsl-backup is exactly `1`.
- Latest manifest includes `/home/victor/work/qsl/codex/ops` exactly once.
- Latest log rsync code 23 caveat is preserved.
- Local status and plan files are inspected read-only and not mutated.
- No backup or restore operation is run.
- No qsl-server or qsl-attachments path is mutated.
- No README, START_HERE, public docs, website, workflow, runtime, crypto,
  dependency, or qsl-backup path is mutated.

## Validation Commands

```bash
git diff --check
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main \
  --allowed docs/governance/evidence/NA-0413_qsl_codex_ops_backup_status_plan_update_authorization_plan.md \
  --allowed tests/NA-0413_qsl_codex_ops_backup_status_plan_update_authorization_testplan.md \
  --allowed DECISIONS.md \
  --allowed TRACEABILITY.md \
  --allowed docs/ops/ROLLING_OPERATIONS_JOURNAL.md
python3 scripts/ci/qsl_evidence_helper.py link-check
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
python3 scripts/ci/qsl_evidence_helper.py pr-body-preflight --file /tmp/na0413_pr_body.md --scan-overclaims
cargo audit --deny warnings
cargo tree -i rustls-webpki --locked
cargo fmt --check
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
python3 formal/model_qsc_handshake_suite_id_bounded.py
python3 formal/run_model_checks.py
```

Also run goal-lint with a synthetic pull-request event containing a standalone
near-top `Goals: G4` line.

## Scope Guard

The changed path set must be exactly the five allowed NA-0413 qsl-protocol
paths. Any local status/plan file mutation is a stop condition for this lane.

## Public-Safety and CI

Before merge, required PR checks must pass, including `public-safety`. After
merge, public-safety must complete success on the merge commit before optional
closeout starts.

## Acceptance Criteria

- NA-0413 evidence selects exactly one NA-0414 successor.
- The selected successor is
  `NA-0414 -- QSL Codex Ops Backup Status / Plan Update Implementation Harness`.
- Both local backup status and plan docs are selected only as future mutable
  candidates.
- Same-host continuity and rsync code 23 caveats remain explicit.
- No backup or restore operation is run.
- No qsl-backup mutation occurs.
- No local status/plan mutation occurs during NA-0413.
- No public-readiness or backup-completion overclaim is introduced.
- Queue and decision helpers remain clean.
