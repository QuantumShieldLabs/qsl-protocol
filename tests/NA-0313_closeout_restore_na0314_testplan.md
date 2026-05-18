Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-18
Replaces: n/a
Superseded-By: n/a

Goals: G1, G2, G3, G4, G5

# NA-0313 Closeout and NA-0314 Restoration Testplan

## Objective

Close out NA-0313 after the bounded qsc suite-id parameter-block
implementation/harness PR merges and restore exactly one READY successor:

NA-0314 -- Metadata Runtime Identifier and Default Padding Transition Plan.

## Protected invariants

- NA-0313 is DONE only after PR #885 merged and post-merge `public-safety`
  completed successfully.
- Exactly one READY item exists after closeout: NA-0314.
- D-0606 exists exactly once and D-0607 remains absent.
- D-0605 implementation evidence remains linked.
- NA-0314 is not implemented by closeout.
- Metadata runtime reduction becomes the active next planning lane.
- No runtime/protocol/crypto/demo/service implementation path changes in
  closeout.
- No Cargo/dependency, workflow, qsc, qsp, qsl-server, qsl-attachments,
  qsc-desktop, website, README, START_HERE, docs/public, branch-protection, or
  public-safety configuration changes.
- No unsupported release, external-review, or privacy overclaim is introduced.

## Allowed scope

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0313_closeout_restore_na0314_testplan.md`

## Forbidden scope

- qsc/qsl/protocol runtime implementation paths.
- QHSM/QSP production wire-format implementation paths.
- production handshake, key schedule, or crypto state-machine paths.
- `Cargo.toml`, `Cargo.lock`, workflows, scripts, services, apps, formal,
  input, refimpl, qsc-desktop, website, README, START_HERE, and docs/public
  paths.
- Branch-protection or public-safety configuration.
- NA-0314 implementation.

## Required proof

- `python3 scripts/ci/qsl_evidence_helper.py queue`
  - `READY_COUNT 1`
  - `READY NA-0314 Metadata Runtime Identifier and Default Padding Transition Plan`
  - NA-0313 reports DONE.
- `python3 scripts/ci/qsl_evidence_helper.py decisions`
  - latest decision is D-0606.
  - duplicate decision count is zero.
- Direct scan confirms D-0606 exists once and D-0607 is absent.
- Scope guard reports only allowed paths and `FORBIDDEN_COUNT 0`.
- Link-check reports zero missing links.
- Added-line leak scan reports zero secret findings.
- Classifier reports the changed path set as docs/governance/testplan only.

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
- `public-safety` must remain required and complete green on the closeout PR
  head.
- Post-merge `public-safety` must complete green on final `origin/main`.
- Docs-only cost-control may skip full runtime suites when classification is
  truthful.

## Successor handoff

NA-0314 should start from NA-0288 through NA-0293 metadata evidence and create
a transition plan for metadata runtime identifier handling and default padding.
It must not claim runtime metadata reduction is implemented unless later live
scope explicitly authorizes executable work and proof.
