Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-29

# NA-0383 Closeout and NA-0384 Restoration Testplan

Goals: G1, G2, G3, G4, G5

## Objective

Validate the governance-only closeout that marks NA-0383 DONE and restores
`NA-0384 -- QSL Local Ops Response Writer Implementation Harness` as the sole
READY successor without implementing NA-0384.

## Protected Invariants

- READY_COUNT is exactly `1`.
- READY item is `NA-0384` after closeout.
- NA-0383 is DONE.
- D-0748 exists once.
- D-0749 exists once.
- D-0750 is absent.
- public-safety remains required and green.
- No runtime, workflow, dependency, public-claim, backup, response archive, or
  response-writer implementation mutation is introduced by closeout.

## Allowed Scope

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0383_closeout_restore_na0384_testplan.md`

## Forbidden Scope

Forbidden scope includes README, START_HERE, docs/public, `.github/**`, Cargo
files, qsp/qsc/qsl runtime paths, qshield runtime, qsl-server,
qsl-attachments, qsc-desktop, website paths, formal/input/script/runtime
implementation paths, backup scripts/timers/fstab/local system paths, branch
deletion, `/srv/qbuild/tools/**`, and `/home/victor/work/qsl/codex/**`.

## Required Local Checks

- `python3 scripts/ci/qsl_evidence_helper.py queue`
- `python3 scripts/ci/qsl_evidence_helper.py decisions`
- `python3 scripts/ci/qsl_evidence_helper.py scope-guard`
- `python3 scripts/ci/qsl_evidence_helper.py link-check --root .`
- `python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main`
- `cargo audit --deny warnings`
- `cargo tree -i rustls-webpki --locked`
- `cargo fmt --check`
- `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1`
- `python3 formal/model_qsc_handshake_suite_id_bounded.py`
- `python3 formal/run_model_checks.py`
- goal-lint against the closeout PR body

## Success Criteria

- Scope guard reports only the allowed closeout paths.
- Queue helper reports READY_COUNT `1` and READY `NA-0384`.
- Decisions helper reports latest D-0749 and duplicate count `0`.
- D-0750 is absent.
- Link and leak checks pass.
- Required CI completes green before merge.
- Post-merge public-safety completes success.

## Successor Handoff

The selected successor is `NA-0384 -- QSL Local Ops Response Writer
Implementation Harness`. This closeout does not authorize or perform response
writer implementation.
