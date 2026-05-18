Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-18
Replaces: n/a
Superseded-By: n/a

Goals: G1, G2, G3, G4, G5

# NA-0311 Closeout and NA-0312 Restoration Testplan

## Objective

Close out NA-0311 after the qsc harness requirements/test-seam plan PR merges
and restore exactly one READY successor:

NA-0312 -- qsc Handshake Suite-ID Parameter-Block Implementation Authorization.

## Protected invariants

- NA-0311 is DONE only after PR #881 merged and post-merge `public-safety`
  completed successfully.
- Exactly one READY item exists after closeout: NA-0312.
- D-0602 exists exactly once and D-0603 remains absent.
- The NA-0311 selected successor remains visible.
- No NA-0312 implementation is authorized by closeout.
- Missing direct qsc handshake suite-id runtime evidence remains visible until
  a later authorized lane implements and tests an explicit schema surface.
- NA-0312 is an implementation-authorization successor, not runtime
  implementation by closeout.
- Metadata runtime identifier/default-padding work remains visible as a
  near-term agenda item.

## Allowed scope

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0311_closeout_restore_na0312_testplan.md`

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
  - `READY NA-0312 qsc Handshake Suite-ID Parameter-Block Implementation Authorization`
  - NA-0311 reports DONE.
- `python3 scripts/ci/qsl_evidence_helper.py decisions`
  - latest decision is D-0602.
  - duplicate decision count is zero.
- Direct scan confirms D-0602 exists once and D-0603 is absent.
- Scope guard reports only allowed paths and `FORBIDDEN_COUNT 0`.
- Link-check reports zero missing links.
- Added-line leak scan reports zero secret findings.
- Classifier reports the changed path set as docs-only.

## Required local checks

- `git diff --check`
- `cargo audit --deny warnings`
- `cargo tree -i rustls-webpki --locked`
- `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1`
- `python3 formal/model_qsc_handshake_suite_id_bounded.py`
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

NA-0312 must authorize exact parameter-block implementation boundaries before
any qsc runtime or QHSM/QSP production wire changes. It must preserve the
NA-0311 evidence that no sufficient test-only seam exists in the current qsc
code.
