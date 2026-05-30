Status: Supporting
Owner: QSL Local Ops
Last-Updated: 2026-05-30

# NA-0386 Closeout Restore NA-0387 Test Plan

Goals: G1, G2, G3, G4, G5

## Objective

Close NA-0386 after the response writer real-archive write harness merged and
restore `NA-0387 -- QSL Local Ops Response Archive Index and History Catalog
Authorization Plan` as the sole READY successor.

## Protected Invariants

- Exactly one READY item exists after closeout.
- NA-0386 is DONE and NA-0387 is READY.
- D-0754 and D-0755 each exist once; D-0756 is absent.
- NA-0387 is not implemented by this closeout.
- No response, request, directive, journal, or local-history index is created.
- No archived response file is overwritten, deleted, truncated, or edited.
- The NA-0386 synthetic smoke file remains the only helper-created real archive
  artifact for NA-0386.
- No workflow, dependency, runtime, qsl-server, qsl-attachments, qshield runtime,
  public-safety script, helper script, backup script, timer, fstab, website,
  README, START_HERE, or docs/public path is changed.
- No secret handling, remote/off-host setup, restore, deploy, rollback, target
  setup, or public claim expansion occurs.

## Allowed Scope

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0386_closeout_restore_na0387_testplan.md`

## Required Checks

1. `python3 scripts/ci/qsl_evidence_helper.py queue`
2. `python3 scripts/ci/qsl_evidence_helper.py decisions`
3. Smoke file checksum:
   `2d06eb23330873576f813d875dadb08b5b26c019138f9cef77af27b8d20b5e40`
4. Scope guard with only the allowed closeout paths.
5. Link check.
6. Added-line leak scan.
7. `cargo audit --deny warnings`
8. `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1`
9. `python3 formal/model_qsc_handshake_suite_id_bounded.py`
10. `python3 formal/run_model_checks.py`
11. PR body preflight and goal-lint.
12. Required CI, including public-safety, green before merge and after merge.

## Success Criteria

- `READY_COUNT 1`
- `READY NA-0387`
- NA-0386 is recorded DONE with PR #1035, merge `dab0bea38242`, smoke path, and
  smoke SHA-256.
- D-0755 records closeout and NA-0387 restoration.
- TRACEABILITY links D-0755, this testplan, PR #1035, the smoke path/checksum,
  and the selected successor.
- Public-safety remains required and green.
