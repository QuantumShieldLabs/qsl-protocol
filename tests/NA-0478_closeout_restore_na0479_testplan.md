Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-06-14

# NA-0478 Closeout and NA-0479 Restoration Testplan

## Objective

Verify that NA-0478 closes only after the qsc KEM/signature/transcript binding formal model implementation PR merged and post-merge public-safety completed success, then restore exactly one READY successor: NA-0479 qsc/refimpl KEM / Signature Binding Mapping Authorization Plan.

## Protected invariants

- NA-0478 is marked DONE only after PR #1226 merge evidence and post-merge public-safety success on `f3d0797de8c1`.
- NA-0479 is restored as the sole READY item.
- NA-0479 remains governance authorization scope only.
- No qsc runtime/source, qsc executable test, crypto, dependency, Cargo, lockfile, workflow, fuzz target, vector, refimpl, service, public-doc, website, qshield, qsl-server, qsl-attachments, backup, restore, qsl-backup, status, plan, rollback, durable Director State Index, or public technical paper mutation is authorized by this closeout.
- No public-readiness, production-readiness, public-internet-readiness, external-review-complete, crypto-complete, KEM-complete, signature-complete, identity-complete, transcript-complete, downgrade-proof, replay-proof, formal-proof-complete, side-channel-free, vulnerability-free, bug-free, or perfect-crypto claim is introduced.

## Allowed scope

- `NEXT_ACTIONS.md`.
- `DECISIONS.md`.
- `TRACEABILITY.md`.
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`.
- `tests/NA-0478_closeout_restore_na0479_testplan.md`.

## Forbidden scope

- Implementation mutation.
- Runtime/crypto/dependency/Cargo/lockfile/workflow mutation.
- qsc source or executable-test mutation.
- executable test/fuzz/vector/formal mutation.
- refimpl mutation.
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
- READY item is NA-0479.
- D-0945 exists once.
- D-0946 is absent.
- Duplicate decision count is 0.
- Changed paths are limited to the five allowed closeout paths.
- PR #1226 post-merge public-safety on `f3d0797de8c1` is success.
- No public overclaim is introduced.
- No backup or restore is run.
