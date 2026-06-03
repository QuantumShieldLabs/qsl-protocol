Goals: G4

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-03

# NA-0409 Closeout / NA-0410 Restoration Testplan

## Purpose

Validate that NA-0409 closes only after qwork Director-facing startup hardening
evidence merged and post-merge public-safety completed success, and that the
preserved backup manifest/status lane is restored as NA-0410 without
implementing it.

## Required Queue Properties

- READY_COUNT is exactly `1`.
- The sole READY item is
  `NA-0410 -- QSL Codex Ops Backup Coverage Manifest Verification / Status Update Plan`.
- NA-0409 is DONE.
- D-0802 exists exactly once.
- D-0803 exists exactly once.
- D-0804 exists exactly once.

## Scope

Allowed changed paths:

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0409_closeout_restore_na0410_backup_manifest_status_testplan.md`

Forbidden changes include qwork implementation files, backup source lists,
backup status or plan files, qsl-backup, runtime, protocol, crypto,
dependencies, workflows, public docs, website, README, START_HERE, qsl-server,
qsl-attachments, response archives, and durable Director State Index output.

## Validation Commands

```bash
python3 scripts/ci/qsl_evidence_helper.py queue --select NA-0409 --select NA-0410
python3 scripts/ci/qsl_evidence_helper.py decisions --select D-0802 --select D-0803 --select D-0804
git diff --name-only origin/main...HEAD
git diff --check
python3 scripts/ci/qsl_evidence_helper.py link-check --root .
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
cargo audit --deny warnings
cargo tree -i rustls-webpki --locked
cargo fmt --check
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
python3 formal/model_qsc_handshake_suite_id_bounded.py
python3 formal/run_model_checks.py
```

Also run PR-body preflight with a standalone `Goals: G4` line.

## Pass Conditions

- Queue and decision helpers report the required properties above.
- Scope guard reports only allowed Packet D paths.
- Link/leak/PR-body checks pass.
- Dependency, qsc, and formal checks pass.
- Required PR checks attach and pass.
- Post-merge public-safety completes success.
- NA-0410 is restored only as the next READY lane; no NA-0410 implementation is
  performed.
- No backup or restore operation is run.
- No qsl-backup, backup source-list, backup status, or backup plan mutation
  occurs.
