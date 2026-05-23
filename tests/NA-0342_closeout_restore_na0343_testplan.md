Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-23

# NA-0342 Closeout and NA-0343 Restoration Test Plan

Goals: G1, G2, G3, G4, G5

## Objective

Verify that NA-0342 is marked DONE only after the qsl-attachments
source/authority blocker-resolution PR merged and post-merge `public-safety`
completed success, then restore the exact NA-0343 successor selected by D-0666:

`NA-0343 -- Metadata Runtime qsl-attachments Production Size-Class Implementation Authorization Plan`

## Protected Invariants

- Exactly one READY item exists after closeout: NA-0343.
- NA-0342 is DONE.
- D-0666 exists once and D-0667 exists once.
- D-0668 is absent.
- No NA-0343 implementation is included by this closeout.
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
- `tests/NA-0342_closeout_restore_na0343_testplan.md`

## Forbidden Scope

- README, START_HERE, docs/public, website, external website repo, qsc-desktop.
- qsl-attachments implementation changes.
- qsl-server implementation changes.
- qshield runtime or test implementation paths.
- qsc/qsp/protocol/crypto/key-schedule changes.
- `Cargo.toml`, `Cargo.lock`, dependency updates, and workflow changes.
- branch-protection or public-safety configuration changes.
- formal, input, tools/refimpl, app runtime, service implementation, or
  production-service changes.
- NA-0343 implementation.

## Required Local Checks

- `python3 scripts/ci/qsl_evidence_helper.py queue`
- `python3 scripts/ci/qsl_evidence_helper.py decisions`
- D-0666/D-0667/D-0668 count proof.
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

## CI Expectations

- `public-safety` remains required by branch protection.
- PR checks attach and complete successfully.
- Post-merge main `public-safety` completes success.
- No admin bypass, squash, rebase, direct push, branch deletion command, or
  delete-branch flag is used.

## Successor Handoff

NA-0343 must create an implementation authorization plan or stop with exact
changed-prerequisite evidence. It must not implement qsl-attachments production
size-class behavior unless a later exact directive separately authorizes
mutation and names the repository, base SHA, allowed files, CI, rollback,
deploy boundary, backup boundary, qsl-server integration boundary, and
public-claim boundary.
