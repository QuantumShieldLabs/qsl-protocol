Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-06-11

# NA-0463 Closeout and NA-0464 Restoration Testplan

## Objective

Close out NA-0463 after the qsc A2 signature provider RNG failure no-output implementation PR merges and post-merge public-safety is green, then restore `NA-0464 -- QSL qsc Identity Provider RNG Failure Split-Scope Authorization Plan` as the sole READY successor.

## Protected Invariants

- Exactly one READY item remains.
- NA-0463 is DONE.
- NA-0464 is READY.
- D-0914 exists once.
- D-0915 is absent during closeout.
- Duplicate decision IDs remain absent.
- NA-0463 A2 evidence remains bounded no-output-only evidence.
- A2 signing failure is not described as pre-mutation no-mutation.
- NA-0464 is authorization-only and does not implement identity provider RNG work.
- No public claim expansion occurs.

## Allowed Scope

- `NEXT_ACTIONS.md`.
- `DECISIONS.md`.
- `TRACEABILITY.md`.
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`.
- `tests/NA-0463_closeout_restore_na0464_testplan.md`.

## Forbidden Scope

- No qsc source mutation.
- No executable qsc test mutation.
- No runtime, crypto, dependency, Cargo, lockfile, workflow, fuzz target, vector, formal model, refimpl, qsl-server, qsl-attachments, qshield runtime, qshield-cli, website, public docs, README, START_HERE, qwork/qstart/qresume/qshell, backup, qsl-backup, backup status, backup plan, rollback, or backup tree mutation.
- No backup.
- No restore.
- No public-readiness, production-readiness, public-internet-readiness, external-review-complete, crypto-complete, KEM-complete, signature-complete, identity-complete, RNG-failure-complete, provider-RNG-complete, side-channel-free, vulnerability-free, bug-free, or perfect-crypto claim.

## Queue and Decision Checks

- Run `python3 scripts/ci/qsl_evidence_helper.py queue`.
- Run `python3 scripts/ci/qsl_evidence_helper.py decisions`.
- Verify READY_COUNT 1.
- Verify READY NA-0464.
- Verify NA-0463 DONE.
- Verify D-0914 exists once.
- Verify D-0915 absent.
- Verify duplicate decision count zero.

## Scope Guard

- Run `git diff --name-only origin/main...HEAD`.
- Verify the diff contains exactly:
  - `NEXT_ACTIONS.md`
  - `DECISIONS.md`
  - `TRACEABILITY.md`
  - `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
  - `tests/NA-0463_closeout_restore_na0464_testplan.md`

## Link Check

- Run the repository relative markdown link check.
- Required result: `TOTAL_MISSING 0`.

## Leak Scan

- Run added-line leak scan for sensitive endpoint, token, auth header, and long-hex output patterns.
- Required result: zero findings.

## Overclaim Scan

- Run added-line public-claim overclaim scan.
- Required result: zero affirmative overclaims.

## PR Body Preflight

- Verify the closeout PR body includes:
  - `Goals: G1, G2, G3, G4, G5`
  - Impact
  - No-regression
  - Tests/Vectors
- Verify the PR body states closeout-only scope, no NA-0464 implementation, and no public overclaim.

## Dependency Health

- Run `cargo audit --deny warnings`.
- Run `cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock`.
- Cargo audit remains dependency-health evidence only.

## Public-Safety

- Verify public-safety completed success on NA-0463 PR #1195 merge commit before closeout.
- Verify closeout PR checks pass before merge.
- Verify public-safety completed success on the closeout merge commit after merge.

## Acceptance Criteria

- NA-0463 is DONE.
- NA-0464 is READY and authorization-only.
- Exactly one READY item remains.
- D-0914 exists once and D-0915 remains absent.
- Changed paths are limited to the five closeout paths.
- Root cargo audit is green.
- Nested qsc fuzz lock audit is green.
- Link, leak, overclaim, PR body, scope guard, queue, and decision checks pass.
- No backup or restore is run.
- No qsl-backup, status, plan, rollback, or backup tree path is mutated.
