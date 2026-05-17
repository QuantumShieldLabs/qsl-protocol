Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-17
Replaces: n/a
Superseded-By: n/a

Goals: G1, G2, G3, G4, G5

# NA-0308 Closeout and NA-0309 Restoration Testplan

## Objective

Close out NA-0308 after the qsc handshake suite-id formal/vector design merges
and restore exactly one READY successor:

NA-0309 -- qsc Handshake Suite-ID Formal Model Properties.

## Protected invariants

- NA-0308 is DONE only after PR #875 merged and post-merge `public-safety`
  completed successfully.
- Exactly one READY item exists after closeout: NA-0309.
- D-0596 exists exactly once and D-0597 is absent.
- The NA-0308 selected successor remains visible.
- No NA-0309 implementation is authorized by closeout.
- Missing explicit qsc handshake suite-id implementation evidence remains
  visible until a later authorized lane implements and tests an explicit schema
  surface.
- NA-0309 is a formal/model successor, not a qsc runtime, QHSM wire-format,
  vector/refimpl, key schedule, or production handshake implementation lane by
  closeout.

## Allowed scope

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0308_closeout_restore_na0309_testplan.md`

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
  - `READY NA-0309 qsc Handshake Suite-ID Formal Model Properties`
  - NA-0308 reports DONE.
- `python3 scripts/ci/qsl_evidence_helper.py decisions`
  - latest decision is D-0596.
  - duplicate decision count is zero.
- Direct scan confirms D-0596 exists once and D-0597 is absent.
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

## Successor handoff

NA-0309 is a no-implementation formal/model lane unless its future directive
explicitly authorizes narrower model implementation scope. It must preserve
the NA-0308 conclusion that executable bounded model properties should be
added before vector/refimpl or qsc parameter-block implementation work.
