# NA-0483 Closeout Restore NA-0484 Testplan

Goals: G1, G2, G3, G4, G5

## Objective

Verify NA-0483 can be closed after the implementation PR merged and post-merge public-safety completed success, then restore exactly one READY successor: `NA-0484 -- QSL Fuzz Binding Coverage Scope Authorization Plan`.

## Protected invariants

- Exactly one READY item remains.
- NA-0483 is marked DONE.
- NA-0484 is READY and authorization scoped only.
- No NA-0484 implementation is performed.
- No public claim boundary is expanded.

## Allowed scope

- `NEXT_ACTIONS.md`.
- `DECISIONS.md`.
- `TRACEABILITY.md`.
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`.
- `tests/NA-0483_closeout_restore_na0484_testplan.md`.

## Forbidden scope

- Implementation mutation.
- Runtime, crypto, dependency, Cargo, lockfile, or workflow mutation.
- Executable test, fuzz target, vector, or formal model mutation.
- qsc source/test, refimpl source/test, qsl-server, qsl-attachments, qshield runtime, qshield-cli, website, public docs, README, START_HERE, qwork, qstart, qresume, qshell, qsl-backup, backup status, backup plan, rollback, durable Director State Index, public technical paper, or backup tree mutation.
- Public-readiness, production-readiness, public-internet-readiness, external-review-complete, crypto-complete, vector-complete, KEM-complete, signature-complete, identity-complete, transcript-complete, qsc/refimpl-equivalence-complete, provider-boundary-complete, provider-RNG-complete, formal-proof-complete, replay-proof, downgrade-proof, side-channel-free, vulnerability-free, bug-free, or perfect-crypto claims.

## Preconditions

- NA-0483 implementation PR #1236 is merged.
- Post-merge public-safety on the implementation merge commit is success.
- D-0954 exists once.
- D-0955 is absent before closeout.
- Queue has exactly one READY item: NA-0483.

## Queue validation

- Run `python3 scripts/ci/qsl_evidence_helper.py queue`.
- Require `READY_COUNT 1`.
- Require READY NA-0484 after the closeout patch.
- Require NA-0483 DONE after the closeout patch.

## Decision validation

- Run `python3 scripts/ci/qsl_evidence_helper.py decisions`.
- Require D-0955 exists once.
- Require no duplicate decision IDs.
- Require D-0956 absent.

## Scope guard

- Compare changed paths against the allowed closeout path set.
- Require zero changes outside the allowed closeout paths.
- Require no mutation under `inputs/suite2/vectors/`, qsc, refimpl, formal, fuzz, Cargo, lockfile, workflow, service, public-doc, backup, or qwork paths.

## Claim-boundary validation

- Run link-check.
- Run leak-scan over changed paths.
- Run an added-line overclaim scan over closeout changes.
- Require no public readiness, completion, proof, side-channel-free, vulnerability-free, bug-free, or perfect-crypto overclaim.

## Dependency and inherited health

- Root cargo audit and nested qsc fuzz lock audit remain dependency-health evidence only.
- No dependency files are changed by closeout.
- Formal/qsc/refimpl implementation tests are inherited from the implementation PR and post-merge public-safety; this closeout does not introduce executable test changes.

## PR validation

- PR body must include Goals, Impact, No-regression, and Tests/Vectors.
- Goal-lint must pass.
- Required PR checks must pass before merge.
- Post-merge public-safety must complete success after closeout merge.

## Acceptance criteria

- NA-0483 DONE.
- NA-0484 READY.
- D-0955 accepted.
- Exactly one READY item remains.
- No implementation mutation.
- No public overclaim.
