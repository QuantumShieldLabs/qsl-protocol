Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-18

# NA-0316 Closeout and NA-0317 Restoration Testplan

Goals: G1, G2, G3, G4, G5

## Objective

Validate that NA-0316 is closed only after the qshield poll no-mutation blocker
evidence PR merged and post-merge public-safety was green, and that exactly one
successor is restored: `NA-0317 -- Metadata Runtime qshield Ack/Commit Poll
Semantics Authorization`.

## Protected Invariants

- NA-0316 must be DONE.
- NA-0317 must be the sole READY item.
- D-0611 must exist once.
- D-0612 must exist once.
- D-0613 must be absent.
- No NA-0317 implementation is authorized or made by this closeout.
- qshield remote queue mutation remains visible as `PROVEN_REMOTE_MUTATION` and
  `NEEDS_RUNTIME_CHANGE`.
- Runtime metadata reduction remains unimplemented until a later exact
  executable lane proves it.

## Allowed Scope

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0316_closeout_restore_na0317_testplan.md`

## Forbidden Scope

- README, START_HERE, docs/public, `.github`, scripts, Cargo, qsp, qsc, qsl,
  qsl-client, apps, tools, inputs, formal, qsc-desktop, qsl-server,
  qsl-attachments, website, protocol/crypto/runtime/demo/service implementation
  paths, branch-protection, public-safety configuration, branch deletion, and
  NA-0317 implementation.

## Queue Requirements

- `python3 scripts/ci/qsl_evidence_helper.py queue` reports `READY_COUNT 1` and
  `READY NA-0317`.
- `NEXT_ACTIONS.md` marks NA-0316 DONE.
- `NEXT_ACTIONS.md` restores NA-0317 with the exact selected title.
- NA-0317 objective is authorization for qshield ack/commit or equivalent
  queue-preserving poll semantics, not runtime implementation.

## Decision Requirements

- `python3 scripts/ci/qsl_evidence_helper.py decisions` reports D-0612 as
  latest, no duplicate decisions, D-0611 once, and D-0612 once.
- D-0613 remains absent.
- D-0612 states that NA-0316 classified the qshield poll blocker, NA-0317 is
  selected from NA-0316 evidence, no NA-0317 implementation is authorized, and
  runtime metadata reduction remains unimplemented.

## Scope Requirements

- Scope guard allows only the closeout files.
- No runtime/protocol/crypto/service/dependency/workflow/public-doc/README/
  START_HERE/website paths are changed.
- No branch deletion command is used.

## Mutation Boundary Requirements

- The closeout preserves `PROVEN_REMOTE_MUTATION` and `NEEDS_RUNTIME_CHANGE`.
- The closeout must not claim remote no-mutation.
- The closeout must not present local-only no-mutation as remote no-mutation.
- The next lane must require exact authorization before qshield runtime files
  are touched.

## Claim-Boundary Requirements

- No production readiness, public-internet readiness, external-review-complete,
  anonymity, metadata-free, untraceable, quantum-proof, unbreakable,
  guaranteed-secure, or broad-readiness claim is introduced.
- Any high-risk wording appears only as prohibited, negated, future-gated, or
  not-proven wording.
- Metadata runtime gaps remain visible.
- qshield poll/no-mutation risk remains visible.

## Backup-Impact Requirements

- Changed paths remain under qsl-protocol governance/testplan locations already
  covered by the `/srv/qbuild/work` backup scope.
- No new durable evidence location outside the existing backup scope is
  introduced.

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

NA-0317 must start from the NA-0316 classification that qshield `/poll`
destructively removes queued messages before local receive-side validation. It
must authorize exact ack/commit or equivalent queue-preserving semantics before
runtime implementation, or stop with exact prerequisite evidence. It must not
implement broader metadata runtime behavior unless a future directive explicitly
authorizes exact files, tests, markers, and stop conditions.
