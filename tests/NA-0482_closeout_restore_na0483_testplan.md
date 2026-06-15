Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-15

# NA-0482 Closeout and NA-0483 Restoration Testplan

Goals: G1, G2, G3, G4, G5

## Objective

Verify that NA-0482 closes only after the authorization evidence PR merged and
post-merge public-safety completed success, then restore exactly one READY
successor: NA-0483 Binding Negative Vector Suite Implementation Harness.

## Protected invariants

- NA-0482 is marked DONE only after PR #1234 merge evidence and post-merge
  public-safety success on `280179428e5a`.
- NA-0483 is restored as the sole READY item.
- NA-0483 implements only the selected internal binding negative vector
  README/manifest scope unless a later exact directive changes scope.
- This closeout does not implement NA-0483.
- This closeout does not mutate inputs or vectors.
- No runtime, crypto, dependency, Cargo, lockfile, workflow, executable test,
  fuzz target, formal model, qsc source, refimpl source/test, service,
  public-doc, website, qshield, qsl-server, qsl-attachments, backup, restore,
  qsl-backup, status, plan, rollback, durable Director State Index, public
  technical paper, or backup tree mutation is authorized by this closeout.
- No public-readiness, production-readiness, public-internet-readiness,
  external-review-complete, crypto-complete, vector-complete, KEM-complete,
  signature-complete, qsc/refimpl-equivalence-complete,
  provider-boundary-complete, formal-proof-complete, side-channel-free,
  vulnerability-free, bug-free, or perfect-crypto claim is introduced.
- Cargo audit green remains dependency-health evidence only.

## Allowed scope

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- this testplan

## Forbidden scope

- NA-0483 implementation mutation.
- Input/vector mutation.
- Runtime/crypto/dependency/Cargo/lockfile/workflow mutation.
- qsc source or executable-test mutation.
- refimpl source or executable-test mutation.
- executable test/fuzz/formal mutation.
- service/public/qshield/qsl-server/qsl-attachments mutation.
- backup/restore/qsl-backup mutation.
- public overclaim or completion claims.

## Closeout proof commands

- `python3 scripts/ci/qsl_evidence_helper.py queue`.
- `python3 scripts/ci/qsl_evidence_helper.py decisions --select D-0952 --select D-0953 --select D-0954`.
- `git diff --check`.
- Exact path scope guard against the five allowed closeout paths.
- Manual markdown link-integrity check.
- Added-line overclaim scan.
- Leak scan.
- PR body preflight and goal-lint.
- `PYTHONDONTWRITEBYTECODE=1 python3 formal/run_model_checks.py`.
- `cargo fmt --check`.
- `cargo audit --deny warnings`.
- `cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock`.

## Acceptance criteria

- READY_COUNT is 1.
- READY item is NA-0483.
- D-0953 exists once.
- D-0954 is absent before future NA-0483 work.
- Duplicate decision count is 0.
- Changed paths are limited to the five allowed closeout paths.
- PR #1234 post-merge public-safety on `280179428e5a` is success.
- No input/vector mutation occurs.
- No public overclaim is introduced.
- No backup or restore is run.
