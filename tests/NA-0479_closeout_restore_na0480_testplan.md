Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-06-14

# NA-0479 Closeout and NA-0480 Restoration Testplan

## Objective

Verify that NA-0479 closes only after the qsc/refimpl KEM and signature
binding mapping authorization PR merged and post-merge public-safety completed
success, then restore exactly one READY successor: NA-0480 refimpl KEM /
Signature Provider Boundary Test Scope Authorization Plan.

## Protected invariants

- NA-0479 is marked DONE only after PR #1228 merge evidence and post-merge
  public-safety success on `27e649c5d5cf`.
- NA-0480 is restored as the sole READY item.
- NA-0480 remains governance authorization scope only.
- No implementation, runtime, crypto, dependency, Cargo, lockfile, workflow,
  executable test, fuzz target, vector, formal model, refimpl, service,
  public-doc, website, qshield, qsl-server, qsl-attachments, backup, restore,
  qsl-backup, status, plan, rollback, durable Director State Index, public
  technical paper, or backup tree mutation is authorized by this closeout.
- No public-readiness, production-readiness, public-internet-readiness,
  external-review-complete, crypto-complete, KEM-complete, signature-complete,
  qsc/refimpl-equivalence-complete, formal-proof-complete,
  provider-RNG-complete, side-channel-free, vulnerability-free, bug-free, or
  perfect-crypto claim is introduced.

## Allowed scope

- `NEXT_ACTIONS.md`.
- `DECISIONS.md`.
- `TRACEABILITY.md`.
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`.
- `tests/NA-0479_closeout_restore_na0480_testplan.md`.

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
- READY item is NA-0480.
- D-0947 exists once.
- D-0948 is absent.
- Duplicate decision count is 0.
- Changed paths are limited to the five allowed closeout paths.
- PR #1228 post-merge public-safety on `27e649c5d5cf` is success.
- No public overclaim is introduced.
- No backup or restore is run.
