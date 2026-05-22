# NA-0333 Closeout / NA-0334 Restoration Test Plan

Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-05-22

Goals: G1, G2, G3, G4, G5

## Objective

Close NA-0333 after the prerequisite plan PR has merged and restore the exact
NA-0334 successor selected by D-0648:
`NA-0334 -- Metadata Runtime qshield Demo Cover Traffic Prototype Authorization Plan`.

## Protected Invariants

- Exactly one READY item exists after closeout: NA-0334.
- NA-0333 is DONE.
- D-0648 remains present exactly once.
- D-0649 is added exactly once.
- NA-0334 is not implemented by this closeout.
- Cover traffic is not implemented by this closeout.
- No cover-traffic prototype is implemented by this closeout.
- qshield embedded relay/demo evidence remains distinct from qsl-server and
  qsl-attachments production behavior.
- qsl-server and qsl-attachments production cover traffic remain cross-repo
  gated.
- No production-readiness, public-internet-readiness, external-review-complete,
  anonymity, metadata-free, untraceable, timing-hidden, or traffic-shape-hidden
  claim is introduced.

## Allowed Scope

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0333_closeout_restore_na0334_testplan.md`

## Forbidden Scope

- Runtime implementation paths.
- qshield implementation paths.
- qsl-server implementation paths.
- qsl-attachments implementation paths.
- qsc, qsp, protocol-core, crypto, state-machine, or key-schedule paths.
- Cargo manifests or lockfiles.
- Workflow or branch-protection configuration.
- Public docs, README, START_HERE, website, qsc-desktop, formal, input, tools,
  app runtime, or service implementation paths.

## Closeout Requirements

1. Record qsl-protocol PR #928 head and merge evidence.
2. Record post-merge public-safety success on the PR #928 merge commit.
3. Mark NA-0333 DONE in `NEXT_ACTIONS.md`.
4. Restore exactly one READY item: NA-0334.
5. Add D-0649 describing closeout and successor restoration.
6. Update `TRACEABILITY.md` with D-0649, the closeout testplan, and the selected
   successor.
7. Update the rolling operations journal.

## Queue Checks

- `python3 scripts/ci/qsl_evidence_helper.py queue`
- Expected: `READY_COUNT 1`, sole READY item NA-0334, NA-0333 DONE.

## Decision Checks

- `python3 scripts/ci/qsl_evidence_helper.py decisions`
- Expected: D-0648 once, D-0649 once, D-0650 absent, duplicate count zero.

## Scope Checks

- `python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main`
  with only the allowed closeout paths.
- `git diff --name-only origin/main...HEAD`
- Classifier proof for the changed closeout paths.

## Claim Checks

Scan the diff for prohibited claim families and confirm any matches are only
negated, forbidden, or future-gated:

- production-readiness
- public-internet-readiness
- external-review-complete
- anonymity
- metadata-free
- untraceable
- timing-hidden
- traffic-shape-hidden
- cover traffic implemented

## Link / Leak Checks

- `python3 scripts/ci/qsl_evidence_helper.py link-check`
- `python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main`
- Expected: no missing links and no secret findings.

## Required Local Checks

- `git diff --check`
- `cargo audit --deny warnings`
- `cargo tree -i rustls-webpki --locked`
- `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1`
- `python3 formal/model_qsc_handshake_suite_id_bounded.py`
- `python3 formal/run_model_checks.py`
- `python3 scripts/ci/qsl_evidence_helper.py queue`
- `python3 scripts/ci/qsl_evidence_helper.py decisions`
- `python3 scripts/ci/qsl_evidence_helper.py link-check`
- `python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main`

## CI Expectations

- The PR must keep required checks green.
- `public-safety` must remain required and green before merge.
- Merge must use normal merge with `--match-head-commit`.
- No admin bypass, squash, rebase, direct push, branch deletion, or delete-branch
  flag is allowed.

## Successor Handoff

The restored successor is an authorization-plan lane only. It may decide whether
a future qshield demo-only cover-traffic prototype is authorized, deferred, or
rejected. It must not implement NA-0334 or any production cover traffic.
