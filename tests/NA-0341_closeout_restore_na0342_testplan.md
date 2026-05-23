Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-23

# NA-0341 Closeout and NA-0342 Restoration Test Plan

## Objective

Verify that NA-0341 is marked DONE only after its source/authority bundle PR
merged and post-merge `public-safety` completed success, then restore the exact
NA-0342 successor selected by D-0664:

`NA-0342 -- Metadata Runtime qsl-attachments Source / Authority Blocker Resolution`

## Protected invariants

- Exactly one READY item exists after closeout: NA-0342.
- NA-0341 is DONE.
- D-0664 exists once and D-0665 exists once.
- D-0666 is absent.
- No NA-0342 implementation is included by this closeout.
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

## Allowed scope

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0341_closeout_restore_na0342_testplan.md`

## Forbidden scope

- README, START_HERE, docs/public, website, external website repo, qsc-desktop.
- qsl-attachments implementation changes.
- qsl-server implementation changes.
- qshield runtime or test implementation paths.
- qsc/qsp/protocol/crypto/key-schedule changes.
- `Cargo.toml`, `Cargo.lock`, dependency updates, and workflow changes.
- branch-protection or public-safety configuration changes.
- formal, input, tools/refimpl, app runtime, service implementation, or
  production-service changes.

## Required local checks

- `python3 scripts/ci/qsl_evidence_helper.py queue`
- `python3 scripts/ci/qsl_evidence_helper.py decisions`
- `python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main`
  with the exact allowed closeout paths.
- `python3 scripts/ci/qsl_evidence_helper.py link-check`
- `python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main`
- changed-line overclaim scan with explicit allowed-match review.
- docs-only classifier proof for changed paths.
- `cargo audit --deny warnings`
- `cargo tree -i rustls-webpki --locked`
- `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1`
- `python3 formal/model_qsc_handshake_suite_id_bounded.py`
- `python3 formal/run_model_checks.py`
- goal-lint with a PR body containing a standalone `Goals: G1, G2, G3, G4, G5`
  line.

## CI expectations

- `public-safety` remains required by branch protection.
- PR checks attach and complete successfully.
- Post-merge main `public-safety` completes success.
- No admin bypass, squash, rebase, direct push, branch deletion command, or
  delete-branch flag is used.

## Successor handoff

NA-0342 must resolve the exact source/authority blocker recorded by NA-0341:
latest qsl-attachments source freshness plus mutation/PR/merge authority, or
stop with exact blocker evidence. NA-0342 must not implement qsl-attachments
production size-class behavior unless a later exact directive separately
authorizes implementation.
