Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-29

# NA-0381 Closeout Restore NA-0382 Testplan

Goals: G1, G2, G3, G4, G5

## Objective

Close NA-0381 after directive manifest and allow-file implementation authorization merged, then restore the exact successor:

`NA-0382 -- QSL Local Ops Directive Manifest and Allow-File Implementation Harness`

## Protected Invariants

- Exactly one READY queue item exists after closeout.
- NA-0381 is DONE.
- NA-0382 is READY.
- D-0744 exists once.
- D-0745 exists once.
- D-0746 is absent.
- NA-0382 is not implemented by closeout.
- Public-safety remains required and green.

## Allowed Scope

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0381_closeout_restore_na0382_testplan.md`

## Forbidden Scope

- `README.md`
- `START_HERE.md`
- `docs/public/**`
- `.github/**`
- `Cargo.toml`
- `Cargo.lock`
- `qsp/**`
- `qsc/**`
- `qsl/**`
- `qsl-client/**`
- `apps/**`
- `tools/**`
- `inputs/**`
- `formal/**`
- `scripts/**`
- `qsc-desktop/**`
- `qsl-server/**`
- `qsl-attachments/**`
- `website/**`
- runtime, protocol, crypto, demo, service, branch-protection, public-safety configuration, backup scripts/timers/fstab/local system paths, branch deletion, `/home/victor/work/qsl/codex/**`, and `/srv/qbuild/tools/**`

## Required Local Checks

Run and record:

```bash
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main --allowed NEXT_ACTIONS.md --allowed DECISIONS.md --allowed TRACEABILITY.md --allowed docs/ops/ROLLING_OPERATIONS_JOURNAL.md --allowed tests/NA-0381_closeout_restore_na0382_testplan.md
python3 scripts/ci/qsl_evidence_helper.py link-check --root .
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
cargo audit --deny warnings
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
python3 formal/model_qsc_handshake_suite_id_bounded.py
python3 formal/run_model_checks.py
```

## CI Expectations

- Required checks must attach normally.
- Required checks must complete green.
- `public-safety` must remain required before merge.
- Post-merge `public-safety` must complete green.
- No admin bypass, squash, rebase, direct push, force-push, amend, or branch deletion is authorized.

## Success Criteria

- Queue helper reports READY_COUNT `1`, READY `NA-0382`, and NA-0381 DONE.
- Decision helper reports D-0744 once, D-0745 once, D-0746 absent, and duplicate count zero.
- Scope guard reports no forbidden path.
- Link, leak, dependency, qsc, formal/model, goal-lint, and public-safety checks pass.
- Changed paths remain closeout-only governance/testplan paths.
