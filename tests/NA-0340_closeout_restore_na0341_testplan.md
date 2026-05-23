Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-23

# NA-0340 Closeout and NA-0341 Restoration Test Plan

Goals: G1, G2, G3, G4, G5

## Objective

Validate the governance-only closeout for NA-0340 after the qsl-protocol-only
qsl-attachments production size-class cross-repo authorization/source-authority
plan merged, and restore exactly one successor: NA-0341 -- Metadata Runtime
qsl-attachments Source / Authority Bundle.

## Protected Invariants

- NA-0340 is marked DONE only after PR #942 merged and post-merge
  `public-safety` completed success.
- Exactly one READY item exists after closeout: NA-0341.
- D-0662 remains present once.
- D-0663 is added once.
- D-0664 remains absent.
- NA-0341 is not implemented by this closeout.
- qshield embedded relay/demo evidence remains distinct from qsl-server and
  qsl-attachments production behavior.
- qsl-attachments production object-size padding remains unimplemented and
  unproven.
- qsl-server production timing/storage behavior remains unimplemented and
  unproven.
- No claim is added that attachment size, timing metadata, traffic shape, or
  all metadata is hidden.
- No anonymity, metadata-free, untraceable, production-readiness,
  public-internet-readiness, or external-review-complete claim is introduced.

## Allowed Scope

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- this testplan

## Forbidden Scope

- qshield runtime or test implementation paths.
- qsl-server implementation changes.
- qsl-attachments implementation changes.
- qsc/qsp/protocol/crypto/state-machine/key-schedule implementation changes.
- Dependency, Cargo manifest, or lockfile changes.
- Workflow, branch-protection, or public-safety configuration changes.
- Website, README, START_HERE, docs/public, qsc-desktop, formal, input,
  tool/refimpl, production-service, or public-copy changes.
- NA-0341 implementation.

## Required Local Checks

- `python3 scripts/ci/qsl_evidence_helper.py queue`
- `python3 scripts/ci/qsl_evidence_helper.py decisions`
- D-0662/D-0663/D-0664 count proof.
- `python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main`
  with the closeout allowed path set.
- `python3 scripts/ci/qsl_evidence_helper.py link-check`
- `python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main`
- changed-line overclaim scan with explicit allowed-match review.
- classifier proof for changed paths.
- `cargo audit --deny warnings`
- `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1`
- `python3 formal/model_qsc_handshake_suite_id_bounded.py`
- goal-lint/PR body metadata proof.

## CI Expectations

- Required PR checks complete green before merge.
- `public-safety` remains required by branch protection.
- Post-merge main `public-safety` completes success.

## Successor Handoff

NA-0341 must obtain or stop on exact qsl-attachments source/authority proof
before any qsl-attachments production size-class implementation authorization
or mutation.
