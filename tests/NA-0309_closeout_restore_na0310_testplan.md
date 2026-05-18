Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-17
Replaces: n/a
Superseded-By: n/a

Goals: G1, G2, G3, G4, G5

# NA-0309 Closeout and NA-0310 Restoration Testplan

## Objective

Close out NA-0309 after the qsc handshake suite-id bounded formal model merges
and restore exactly one READY successor:

NA-0310 -- qsc Handshake Suite-ID Vector Schema and Refimpl Oracle.

## Protected invariants

- NA-0309 is DONE only after PR #877 merged and post-merge `public-safety`
  completed successfully.
- Exactly one READY item exists after closeout: NA-0310.
- D-0598 exists exactly once and D-0599 is absent.
- The NA-0309 selected successor remains visible.
- No NA-0310 implementation is authorized by closeout.
- Missing explicit qsc handshake suite-id runtime implementation evidence
  remains visible until a later authorized lane implements and tests an
  explicit schema surface.
- NA-0310 is a vector/refimpl successor, not a qsc runtime, QHSM wire-format,
  key schedule, or production handshake implementation lane by closeout.

## Allowed scope

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0309_closeout_restore_na0310_testplan.md`

## Forbidden scope

- qsc/qsl/protocol runtime implementation paths.
- QSP wire-format implementation paths.
- production handshake, key schedule, or crypto state-machine paths.
- `Cargo.toml`, `Cargo.lock`, workflows, scripts, services, apps, formal,
  input, refimpl, qsc-desktop, website, README, START_HERE, and docs/public
  paths.
- Branch-protection or public-safety configuration.

## Required proof

- `python3 scripts/ci/qsl_evidence_helper.py queue`
  - `READY_COUNT 1`
  - `READY NA-0310 qsc Handshake Suite-ID Vector Schema and Refimpl Oracle`
  - NA-0309 reports DONE.
- `python3 scripts/ci/qsl_evidence_helper.py decisions`
  - latest decision is D-0598.
  - duplicate decision count is zero.
- Direct scan confirms D-0598 exists once and D-0599 is absent.
- Scope guard reports only allowed paths and `FORBIDDEN_COUNT 0`.
- Link-check reports zero missing links.
- Added-line leak scan reports zero secret findings.
- Classifier reports the changed path set as docs-only.

## Required local checks

- `git diff --check`
- `cargo audit --deny warnings`
- `cargo tree -i rustls-webpki --locked`
- `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1`
- `python3 formal/run_model_checks.py`
- goal-lint on the closeout PR body.

## CI expectations

- Required checks must attach and complete green before merge.
- `public-safety` must remain a required status check and complete green on the
  closeout PR head.
- Post-merge `public-safety` must complete green on final `origin/main`.
- Docs-only cost-control may skip full runtime suites when the classifier
  truthfully selects the cheaper path.

## Successor handoff

NA-0310 should use the NA-0309 model properties to define qsc handshake
suite-id vector schema, vector categories, deterministic refimpl oracle
behavior, transcript/key-context fields, and reject/no-mutation expectations.
It must not implement qsc runtime or QHSM/QSP production wire changes unless a
future live directive explicitly authorizes that scope.
