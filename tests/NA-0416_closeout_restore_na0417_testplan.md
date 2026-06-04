Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-04

# NA-0416 Closeout Restore NA-0417 Testplan

Goals: G4

## Purpose

Validate that NA-0416 is closed only after its authorization evidence PR merges
and post-merge public-safety is green, and that the selected NA-0417 root
operator cleanup / permission remediation packet plan is restored as the sole
READY successor without implementing NA-0417.

## Scope

Allowed closeout paths:

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0416_closeout_restore_na0417_testplan.md`

Forbidden closeout paths include local backup status/plan files, qsl-backup,
backup source lists, temp rollback subtrees, systemd/timer/fstab files,
runtime, protocol, crypto, dependency, workflow, qsl-server, qsl-attachments,
qshield runtime, website, public docs, README, and START_HERE.

## Preconditions

- PR #1100 merged as `015631bf54f1`.
- Post-merge public-safety completed success on `015631bf54f1`.
- Queue helper reports READY_COUNT `1` and READY NA-0416 before closeout.
- Decision helper reports latest D-0819 and duplicate count zero before
  closeout.
- D-0820 is absent before closeout.

## Required Assertions

- NA-0416 is marked DONE.
- NA-0417 is restored as the sole READY item.
- D-0820 exists once.
- D-0821 is absent.
- Duplicate decision count remains `0`.
- The NA-0417 block preserves rollback evidence preservation, same-host
  continuity, active code 23 source, no Codex sudo, no backup, no restore,
  no-qsl-backup-mutation, no-temp-subtree-mutation, no-secret, and
  no-public-overclaim caveats.
- NA-0417 root-operator packet planning is not implemented by closeout.
- No backup or restore operation is run.
- No sudo is run.
- No qsl-backup mutation occurs.
- No local status/plan mutation occurs during closeout.
- No temp rollback subtree deletion, move, chmod, chown, copy, or other
  mutation occurs during closeout.

## Validation Commands

```bash
git diff --check
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main \
  --allowed NEXT_ACTIONS.md \
  --allowed DECISIONS.md \
  --allowed TRACEABILITY.md \
  --allowed docs/ops/ROLLING_OPERATIONS_JOURNAL.md \
  --allowed tests/NA-0416_closeout_restore_na0417_testplan.md
python3 scripts/ci/qsl_evidence_helper.py link-check
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
python3 scripts/ci/qsl_evidence_helper.py pr-body-preflight --file /tmp/na0416_closeout_pr_body.md --scan-overclaims
cargo audit --deny warnings
cargo tree -i rustls-webpki --locked
cargo fmt --check
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
python3 formal/model_qsc_handshake_suite_id_bounded.py
python3 formal/run_model_checks.py
```

Also run goal-lint with a synthetic pull-request event containing a standalone
near-top `Goals: G4` line.

## Acceptance Criteria

- Changed path set is exactly the five allowed closeout paths.
- NA-0417 is READY and NA-0416 is DONE.
- No NA-0417 implementation is hidden in closeout.
- Same-host continuity and active code 23 caveats remain explicit.
- Rollback evidence preservation is required before any future temp subtree
  mutation.
- No sudo, backup, restore, qsl-backup mutation, or temp rollback subtree
  mutation occurs.
- public-safety is green before merge and after merge.
