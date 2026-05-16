Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-16
Replaces: n/a
Superseded-By: n/a

# NA-0296 Closeout Restore NA-0297 Testplan

## Objective

Close NA-0296 after the website source verification readiness audit merged and
restore exactly one READY successor: NA-0297 Website Source Verification
Follow-Up and Implementation Blocker Resolution.

## Protected Invariants

- NA-0296 closeout does not implement NA-0297.
- NA-0296 closeout does not mutate a website or external website repository.
- Source-unverified readiness does not authorize website implementation.
- Website handoff, audit, and planning do not equal a live website update.
- Production readiness, public internet readiness, external review completion,
  anonymity, metadata-free messaging, and untraceability remain unclaimed.
- public-safety remains required and green.
- Exactly one READY queue item exists after closeout.

## Allowed Scope

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0296_closeout_restore_na0297_testplan.md`

## Forbidden Scope

- `README.md`
- `START_HERE.md`
- `docs/public/**`
- `.github/**`
- `scripts/**`
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
- `qsc-desktop/**`
- `qsl-server/**`
- `qsl-attachments/**`
- `website/**`
- any external website repository
- runtime, protocol, crypto, demo, or service implementation paths
- branch-protection or public-safety configuration

## Queue Expectations

- Before closeout: READY_COUNT `1`, READY `NA-0296`.
- After closeout patch: READY_COUNT `1`, READY `NA-0297`.
- NA-0296 is DONE.
- NA-0297 is a blocker-resolution/source-verification lane because NA-0296
  classified readiness as `PARTIAL_READY_SOURCE_UNVERIFIED`.

## Decision Expectations

- D-0568 exists exactly once before closeout.
- D-0569 exists exactly once after closeout.
- D-0570 remains absent.
- No duplicate decision IDs appear.

## Validation Expectations

Required local validation:

- `git diff --check origin/main...HEAD`
- queue proof
- decisions proof
- exact allowed-path scope guard
- link check
- added-content leak scan
- direct overclaim scan
- classifier proof for the changed path set
- `cargo audit --deny warnings`
- `cargo tree -i rustls-webpki --locked`
- `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1`
- `python3 formal/run_model_checks.py` when present
- goal-lint for the PR body

Required PR validation:

- Required checks complete normally before merge.
- No admin bypass, direct push, squash, rebase, branch deletion, or
  delete-branch flag is used.
- Post-merge public-safety completes successfully on final main.

## Future Lane Gate

NA-0297 may only resolve source/deploy and claim-scan blockers unless a future
directive verifies the exact website source repository, branch, build, preview,
deployment, rollback, link-scan, and claim-scan path and explicitly authorizes
website implementation.
