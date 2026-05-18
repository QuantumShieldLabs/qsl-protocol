Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-18

# NA-0314 Closeout and NA-0315 Restoration Testplan

Goals: G1, G2, G3, G4, G5

## Objective

Validate that NA-0314 is closed only after its transition-plan PR merged and post-merge public-safety was green, and that exactly one successor is restored: `NA-0315 -- Metadata Runtime Identifier and Default Padding Executable Harness Plan`.

## Protected Invariants

- NA-0314 must be DONE.
- NA-0315 must be the sole READY item.
- D-0607 must exist once.
- D-0608 must exist once.
- D-0609 must be absent.
- No NA-0315 implementation is authorized or made by this closeout.
- Runtime metadata reduction remains unimplemented until a later exact executable lane proves it.

## Allowed Scope

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0314_closeout_restore_na0315_testplan.md`

## Forbidden Scope

- README, START_HERE, docs/public, `.github`, scripts, Cargo, qsp, qsc, qsl, qsl-client, apps, tools, inputs, formal, qsc-desktop, qsl-server, qsl-attachments, website, protocol/crypto/runtime/demo/service implementation paths, branch-protection, public-safety configuration, branch deletion, and NA-0315 implementation.

## Queue Requirements

- `python3 scripts/ci/qsl_evidence_helper.py queue` reports `READY_COUNT 1` and `READY NA-0315`.
- `NEXT_ACTIONS.md` marks NA-0314 DONE.
- `NEXT_ACTIONS.md` restores NA-0315 with the exact selected title.

## Decision Requirements

- `python3 scripts/ci/qsl_evidence_helper.py decisions` reports D-0608 as latest, no duplicate decisions, D-0607 once, and D-0608 once.
- D-0609 remains absent.
- D-0608 states that NA-0314 delivered the transition plan, NA-0315 is selected from NA-0314 evidence, no NA-0315 implementation is authorized, and runtime metadata reduction remains unimplemented.

## Scope Requirements

- Scope guard allows only the closeout files.
- No runtime/protocol/crypto/service/dependency/workflow/public-doc/README/START_HERE/website paths are changed.
- No branch deletion command is used.

## Claim-Boundary Requirements

- No production readiness, public-internet readiness, external-review-complete, anonymity, metadata-free, untraceable, quantum-proof, unbreakable, guaranteed-secure, or broad-readiness claim is introduced.
- Any high-risk wording appears only as prohibited, negated, future-gated, or not-proven wording.
- Metadata runtime gaps remain visible.

## Required Local Checks

- `git diff --check`
- `python3 scripts/ci/qsl_evidence_helper.py queue`
- `python3 scripts/ci/qsl_evidence_helper.py decisions`
- `python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main` with exact allowed closeout paths
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
- Docs/governance-only cost control may skip full suites only if public-safety classifies the closeout patch accordingly.

## Successor Handoff

After merge and post-merge public-safety success, NA-0315 is the active READY item. The next directive should target `NA-0315 -- Metadata Runtime Identifier and Default Padding Executable Harness Plan` and must not claim anonymity, metadata-free behavior, untraceability, production readiness, public-internet readiness, or external review completion.
