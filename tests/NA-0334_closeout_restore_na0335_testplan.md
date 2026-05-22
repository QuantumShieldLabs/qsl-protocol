# NA-0334 Closeout / NA-0335 Restoration Test Plan

Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-05-22

Goals: G1, G2, G3, G4, G5

## Objective

Close NA-0334 after the authorization PR has merged and restore the exact
NA-0335 successor selected by D-0650:
`NA-0335 -- Metadata Runtime qshield Demo Cover Traffic Prototype Implementation Harness`.

## Protected Invariants

- Exactly one READY item exists after closeout: NA-0335.
- NA-0334 is DONE.
- D-0650 remains present exactly once.
- D-0651 is added exactly once.
- NA-0335 is not implemented by this closeout.
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
- `tests/NA-0334_closeout_restore_na0335_testplan.md`

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

1. Record qsl-protocol PR #930 head and merge evidence.
2. Record post-merge public-safety success on the PR #930 merge commit.
3. Mark NA-0334 DONE in `NEXT_ACTIONS.md`.
4. Restore exactly one READY item: NA-0335.
5. Add D-0651 describing closeout and successor restoration.
6. Update `TRACEABILITY.md` with D-0651, the closeout testplan, and the
   selected successor.
7. Update the rolling operations journal.

## Queue Checks

- `python3 scripts/ci/qsl_evidence_helper.py queue`
- Expected: `READY_COUNT 1`, sole READY item NA-0335, NA-0334 DONE.

## Decision Checks

- `python3 scripts/ci/qsl_evidence_helper.py decisions`
- Expected: D-0650 once, D-0651 once, D-0652 absent, duplicate count zero.

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
- goal-lint through PR CI/preflight.

## CI Expectations

- The PR must keep required checks green.
- `public-safety` must remain required and green before merge.
- Merge must use normal merge with `--match-head-commit`.
- No admin bypass, squash, rebase, direct push, branch deletion, or delete-branch
  flag is allowed.

## Successor Handoff

The restored successor is an implementation-harness lane only for the qshield
embedded relay/demo boundary authorized by NA-0334. It must not implement
production cover traffic, fixed-rate cover, qsl-server behavior,
qsl-attachments behavior, public-internet behavior, or unsupported
public/privacy claims.
