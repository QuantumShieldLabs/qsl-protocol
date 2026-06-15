Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-15

# NA-0481 Closeout and NA-0482 Restoration Testplan

Goals: G1, G2, G3, G4, G5

## Objective

Verify that NA-0481 closes only after the refimpl signature provider-boundary implementation PR merged and post-merge public-safety completed success, then restore exactly one READY successor: NA-0482 Binding Negative Vector Suite Authorization Plan.

## Protected invariants

- NA-0481 is marked DONE only after PR #1232 merge evidence and post-merge public-safety success on `9e216cabb1e2`.
- NA-0482 is restored as the sole READY item.
- NA-0482 remains an authorization plan for exact binding negative vector suite scope.
- No NA-0482 implementation is performed by this closeout.
- No runtime, crypto, dependency, Cargo, lockfile, workflow, executable test, fuzz target, vector, formal model, refimpl source/test, qsc source/test, service, public-doc, website, qshield, qsl-server, qsl-attachments, backup, restore, qsl-backup, status, plan, rollback, durable Director State Index, public technical paper, or backup tree mutation is authorized by this closeout.
- No public-readiness, production-readiness, public-internet-readiness, external-review-complete, crypto-complete, KEM-complete, signature-complete, qsc/refimpl-equivalence-complete, provider-boundary-complete, provider-RNG-complete, formal-proof-complete, side-channel-free, vulnerability-free, bug-free, or perfect-crypto claim is introduced.
- Cargo audit green remains dependency-health evidence only.

## Allowed scope

- `NEXT_ACTIONS.md`.
- `DECISIONS.md`.
- `TRACEABILITY.md`.
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`.
- this testplan.

## Forbidden scope

- NA-0482 implementation mutation.
- Runtime/crypto/dependency/Cargo/lockfile/workflow mutation.
- qsc source or executable-test mutation.
- refimpl source or executable-test mutation.
- executable test/fuzz/vector/formal mutation.
- service/public/qshield/qsl-server/qsl-attachments mutation.
- backup/restore/qsl-backup mutation.
- public overclaim or completion claims.

## Closeout proof commands

- `python3 scripts/ci/qsl_evidence_helper.py queue`.
- `python3 scripts/ci/qsl_evidence_helper.py decisions`.
- `git diff --check`.
- Exact path scope guard against the five allowed closeout paths.
- Manual markdown link-integrity check.
- Added-line overclaim scan.
- Leak scan.
- PR body preflight and goal-lint.
- `python3 formal/run_model_checks.py`.
- `cargo fmt --check`.
- `cargo audit --deny warnings`.
- `cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock`.

## Acceptance criteria

- READY_COUNT is 1.
- READY item is NA-0482.
- D-0951 exists once.
- D-0952 is absent before future NA-0482 work.
- Duplicate decision count is 0.
- Changed paths are limited to the five allowed closeout paths.
- PR #1232 post-merge public-safety on `9e216cabb1e2` is success.
- No public overclaim is introduced.
- No backup or restore is run.
