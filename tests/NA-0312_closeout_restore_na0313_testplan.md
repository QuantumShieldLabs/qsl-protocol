Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-18
Replaces: n/a
Superseded-By: n/a

Goals: G1, G2, G3, G4, G5

# NA-0312 Closeout and NA-0313 Restoration Testplan

## Objective

Close out NA-0312 after the qsc handshake suite-id parameter-block
implementation authorization PR merges and restore exactly one READY successor:

NA-0313 -- qsc Handshake Suite-ID Parameter-Block Implementation Harness.

## Protected invariants

- NA-0312 is DONE only after PR #883 merged and post-merge `public-safety`
  completed successfully.
- Exactly one READY item exists after closeout: NA-0313.
- D-0604 exists exactly once and D-0605 remains absent.
- The NA-0312 selected successor remains visible.
- No NA-0313 implementation is authorized by closeout.
- Missing direct qsc handshake suite-id runtime evidence remains visible until
  the future authorized implementation/harness lane proves explicit admission.
- NA-0313 must follow the file, test, marker, vector, refimpl, model,
  compatibility, transcript/key-context, and stop-condition boundaries frozen
  by D-0603.
- Metadata Runtime Identifier and Default Padding Transition Plan remains the
  recommended immediate successor after NA-0313 unless NA-0313 stops on a qsc
  prerequisite blocker.

## Allowed scope

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0312_closeout_restore_na0313_testplan.md`

## Forbidden scope

- qsc/qsl/protocol runtime implementation paths.
- QHSM/QSP production wire-format implementation paths.
- production handshake, key schedule, or crypto state-machine paths.
- `Cargo.toml`, `Cargo.lock`, workflows, scripts, services, apps, formal,
  input, refimpl, qsc-desktop, website, README, START_HERE, and docs/public
  paths.
- Branch-protection or public-safety configuration.
- NA-0313 implementation.

## Required proof

- `python3 scripts/ci/qsl_evidence_helper.py queue`
  - `READY_COUNT 1`
  - `READY NA-0313 qsc Handshake Suite-ID Parameter-Block Implementation Harness`
  - NA-0312 reports DONE.
- `python3 scripts/ci/qsl_evidence_helper.py decisions`
  - latest decision is D-0604.
  - duplicate decision count is zero.
- Direct scan confirms D-0604 exists once and D-0605 is absent.
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

NA-0313 must implement or stop within the exact D-0603 boundary. It must prove
explicit qsc suite-id parameter-block admission with NA-0310 vectors/refimpl,
NA-0309 model checks, NA-0311 harness requirements, and the NA-0313 marker set.
It must not hide metadata runtime reduction; the metadata runtime identifier and
default-padding transition should be inserted immediately after NA-0313 unless
NA-0313 stops on a qsc prerequisite blocker.
