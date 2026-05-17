Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-05-17
Replaces:
Superseded-By:

Goals: G1, G2, G3, G4, G5

# NA-0306 Closeout and NA-0307 Restoration Testplan

## Objective

Close out NA-0306 after the qsc handshake suite-id wire-format authorization
plan merges and restore exactly one READY successor:
NA-0307 -- qsc Handshake Suite-ID Compatibility and Transcript Binding Design.

## Protected Invariants

- NA-0306 is DONE only after PR #871 merged and post-merge `public-safety`
  completed successfully.
- Exactly one READY item exists after closeout: NA-0307.
- D-0592 exists exactly once and D-0593 is absent.
- The NA-0306 selected successor remains visible.
- No NA-0307 implementation is authorized by closeout.
- Missing explicit qsc handshake suite-id admission evidence remains visible
  until a later authorized lane implements and tests an explicit schema surface.

## Allowed Scope

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0306_closeout_restore_na0307_testplan.md`

## Forbidden Scope

- qsc/qsl/protocol runtime implementation paths.
- QSP wire-format implementation paths.
- production handshake, key schedule, or crypto state-machine paths.
- `Cargo.toml`, `Cargo.lock`, workflows, scripts, services, apps, formal,
  input, refimpl, qsc-desktop, website, README, START_HERE, and docs/public
  paths.
- Branch-protection or public-safety configuration.

## Required Proof

- `python3 scripts/ci/qsl_evidence_helper.py queue`
  - `READY_COUNT 1`
  - `READY NA-0307 qsc Handshake Suite-ID Compatibility and Transcript Binding Design`
  - NA-0306 reports DONE.
- `python3 scripts/ci/qsl_evidence_helper.py decisions`
  - latest decision is D-0592.
  - duplicate decision count is zero.
- Direct scan confirms D-0592 exists once and D-0593 is absent.
- Scope guard reports only allowed paths and `FORBIDDEN_COUNT 0`.
- Link-check reports zero missing links.
- Added-line leak scan reports zero secret findings.
- Classifier reports the changed path set as docs-only.

## Required Local Checks

- `git diff --check`
- `cargo audit --deny warnings`
- `cargo tree -i rustls-webpki --locked`
- `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1`
- `python3 formal/run_model_checks.py`
- goal-lint on the closeout PR body.

## CI Expectations

- Required checks must attach and complete green before merge.
- `public-safety` must remain a required status check and complete green on the
  closeout PR head.
- Post-merge `public-safety` must complete green on final `origin/main`.

## Successor Handoff

NA-0307 is a no-implementation design lane unless its future directive
explicitly authorizes narrower implementation scope. It must preserve the
NA-0306 conclusion that the next safe step is compatibility and transcript
binding design before any `QHSM` suite-id schema implementation.
