Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-14

# NA-0476 Closeout and NA-0477 Restoration Testplan

## Objective

Close NA-0476 after implementation PR #1222 merged and post-merge public-safety completed success, then restore exactly one READY successor: NA-0477, the KEM / Signature / Transcript Formal Model Mapping Authorization Plan.

## Protected Invariants

- NA-0476 remains bounded internal qsc negative-test implementation evidence only.
- NA-0477 is authorization scope only and does not implement formal, runtime, crypto, dependency, workflow, vector, fuzz, service, public-doc, backup, or qsl-backup changes.
- Exactly one READY item remains.
- No public-readiness, production-readiness, public-internet-readiness, external-review-complete, crypto-complete, KEM-complete, signature-complete, identity-complete, transcript-complete, downgrade-proof, replay-proof, side-channel-free, vulnerability-free, bug-free, or perfect-crypto claim is introduced.

## Allowed Scope

- `NEXT_ACTIONS.md`.
- `DECISIONS.md`.
- `TRACEABILITY.md`.
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`.
- `tests/NA-0476_closeout_restore_na0477_testplan.md`.

## Forbidden Scope

- qsc runtime/source mutation.
- crypto implementation mutation.
- dependency, Cargo, lockfile, or workflow mutation.
- executable test, fuzz target, vector, or formal model mutation.
- refimpl mutation.
- qsl-server, qsl-attachments, qshield runtime, qshield-cli, website, public-doc, README, or START_HERE mutation.
- qwork, qstart, qresume, qshell, backup, restore, qsl-backup, backup status, backup plan, rollback, backup tree, durable Director State Index, or public technical paper mutation.

## Closeout Prerequisites

- PR #1222 merged at `e86797ed939d`.
- Post-merge public-safety on `e86797ed939d` completed success.
- D-0940 exists once.
- D-0941 is absent before the closeout patch.
- Queue before closeout patch is READY_COUNT 1 and READY NA-0476.

## Queue Validation

Run `python3 scripts/ci/qsl_evidence_helper.py queue`.

Expected result after patch:
- READY_COUNT 1.
- READY NA-0477.
- NA-0476 DONE.

## Decision Validation

Run `python3 scripts/ci/qsl_evidence_helper.py decisions` and direct decision-ID checks.

Expected result after patch:
- D-0940 exists once.
- D-0941 exists once.
- D-0942 is absent.
- duplicate decision count is zero.

## Scope Guard

Confirm changed paths are limited to the five allowed closeout paths listed above.

## Link And Scan Validation

- Run the deterministic local markdown link-integrity check and require zero missing links.
- Run leak-safe evidence wording checks.
- Run added-line overclaim scan and require zero affirmative overclaims.
- Run PR body preflight and goal-lint before merge.

## Dependency And Public-Safety Checks

- Root `cargo audit --deny warnings` remains dependency-health evidence only.
- Nested qsc fuzz lock audit remains dependency-health evidence only.
- PR checks must be green or accepted skipped/neutral by repo policy.
- Post-merge public-safety must complete success on the closeout merge commit.

## Public Claim Boundary

This closeout does not create a public-readiness, production-readiness, public-internet-readiness, external-review-complete, crypto-complete, KEM-complete, signature-complete, identity-complete, transcript-complete, downgrade-proof, replay-proof, side-channel-free, vulnerability-free, bug-free, or perfect-crypto claim.
