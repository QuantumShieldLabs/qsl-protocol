Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-18

# NA-0315 Closeout and NA-0316 Restoration Testplan

Goals: G1, G2, G3, G4, G5

## Objective

Validate that NA-0315 is closed only after its executable harness-plan PR merged
and post-merge public-safety was green, and that exactly one successor is
restored: `NA-0316 -- Metadata Runtime qshield Poll No-Mutation Blocker
Resolution`.

## Protected Invariants

- NA-0315 must be DONE.
- NA-0316 must be the sole READY item.
- D-0609 must exist once.
- D-0610 must exist once.
- D-0611 must be absent.
- No NA-0316 implementation is authorized or made by this closeout.
- Runtime metadata reduction remains unimplemented until a later exact
  executable lane proves it.
- The qshield poll/no-mutation blocker must remain visible.

## Allowed Scope

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0315_closeout_restore_na0316_testplan.md`

## Forbidden Scope

- README, START_HERE, docs/public, `.github`, scripts, Cargo, qsp, qsc, qsl,
  qsl-client, apps, tools, inputs, formal, qsc-desktop, qsl-server,
  qsl-attachments, website, protocol/crypto/runtime/demo/service implementation
  paths, branch-protection, public-safety configuration, branch deletion, and
  NA-0316 implementation.

## Queue Requirements

- `python3 scripts/ci/qsl_evidence_helper.py queue` reports `READY_COUNT 1` and
  `READY NA-0316`.
- `NEXT_ACTIONS.md` marks NA-0315 DONE.
- `NEXT_ACTIONS.md` restores NA-0316 with the exact selected title.

## Decision Requirements

- `python3 scripts/ci/qsl_evidence_helper.py decisions` reports D-0610 as
  latest, no duplicate decisions, D-0609 once, and D-0610 once.
- D-0611 remains absent.
- D-0610 states that NA-0315 delivered the executable harness plan or exact
  blocker, NA-0316 is selected from NA-0315 evidence, no NA-0316 implementation
  is authorized, and runtime metadata reduction remains unimplemented.

## Scope Requirements

- Scope guard allows only the closeout files.
- No runtime/protocol/crypto/service/dependency/workflow/public-doc/README/
  START_HERE/website paths are changed.
- No branch deletion command is used.

## Claim-Boundary Requirements

- No production readiness, public-internet readiness, external-review-complete,
  anonymity, metadata-free, untraceable, quantum-proof, unbreakable,
  guaranteed-secure, or broad-readiness claim is introduced.
- Any high-risk wording appears only as prohibited, negated, future-gated, or
  not-proven wording.
- Metadata runtime gaps remain visible.
- qshield poll/no-mutation risk remains visible.

## Required Local Checks

- `git diff --check`
- `python3 scripts/ci/qsl_evidence_helper.py queue`
- `python3 scripts/ci/qsl_evidence_helper.py decisions`
- `python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main`
  with exact allowed closeout paths
- `python3 scripts/ci/qsl_evidence_helper.py link-check`
- `python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main`
- `bash scripts/ci/classify_ci_scope.sh <changed_paths>`
- goal-lint with a PR body containing `Goals: G1, G2, G3, G4, G5`
- `cargo audit --deny warnings`
- `cargo tree -i rustls-webpki --locked`
- `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1`
- `python3 formal/model_qsc_handshake_suite_id_bounded.py`
- `python3 formal/run_model_checks.py`

## CI Expectations

- PR checks must attach and complete successfully before merge.
- `public-safety` must remain required and green before merge and after merge.
- Docs/governance-only cost control may skip full suites only if public-safety
  classifies the closeout patch accordingly.

## Successor Handoff

NA-0316 must start from the qshield poll/no-mutation blocker selected by NA-0315.
It must either resolve the queue-removal boundary with executable proof or stop
with an exact prerequisite. It must not implement broader metadata runtime
behavior unless a future directive explicitly authorizes exact files, tests,
markers, and stop conditions.
