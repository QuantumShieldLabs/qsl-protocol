Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-29

# NA-0380 Closeout Restore NA-0381 Testplan

## Objective

Close NA-0380 after the bounded CI/public-safety polling helper harness merged
and restore the exact successor:
`NA-0381 -- QSL Local Ops Directive Manifest and Allow-File Implementation Authorization Plan`.

## Protected Invariants

- Exactly one READY queue item exists after closeout.
- NA-0380 is DONE.
- NA-0381 is READY.
- D-0742 exists once.
- D-0743 exists once.
- D-0744 is absent.
- NA-0381 is not implemented by closeout.
- Public-safety remains required and green.

## Allowed Scope

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0380_closeout_restore_na0381_testplan.md`

## Forbidden Scope

- Runtime, service, protocol, crypto, qsc, qsp, qshield runtime, qsl-server,
  qsl-attachments, qsc-desktop, website, public docs, README, START_HERE,
  workflow, Cargo/dependency, helper implementation, public-safety gate,
  qsl_evidence_helper, backup script/timer/fstab, local tool, branch-protection,
  public-safety configuration, secret, credential, key, remote/off-host target,
  backup, restore, deploy, rollback, and public-claim paths.

## Required Local Checks

1. `python3 scripts/ci/qsl_evidence_helper.py queue`
2. `python3 scripts/ci/qsl_evidence_helper.py decisions`
3. Scope guard proving only allowed closeout paths changed.
4. `python3 scripts/ci/qsl_evidence_helper.py link-check --root .`
5. `python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main`
6. `cargo audit --deny warnings`
7. `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1`
8. `python3 formal/model_qsc_handshake_suite_id_bounded.py`
9. `python3 formal/run_model_checks.py`
10. Goal-lint on the closeout PR body.

## CI Expectations

- Required checks must attach normally.
- Required checks must complete green.
- `public-safety` must remain required before merge.
- Post-merge `public-safety` must complete green.
- No admin bypass, squash, rebase, direct push, force-push, amend, or branch
  deletion is authorized.

## Success Criteria

- Queue helper reports READY_COUNT `1`, READY `NA-0381`, and NA-0380 DONE.
- Decision helper reports D-0742 once, D-0743 once, D-0744 absent, and duplicate
  count zero.
- Scope guard reports no forbidden path.
- Link, leak, dependency, qsc, formal/model, goal-lint, and public-safety checks
  pass.
