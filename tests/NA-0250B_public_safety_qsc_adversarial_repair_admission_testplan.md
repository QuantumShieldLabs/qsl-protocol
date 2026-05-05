Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-05

# NA-0250B Public-Safety qsc-adversarial Repair Admission Test Plan

## Objective

Add one bounded public-safety red-main admission profile for PR #749, the qsc-adversarial cargo-fuzz install repair, so the repair can be validated by normal required checks while `public-safety` and `qsc-adversarial-smoke` remain enforced.

## Protected Invariant

- `public-safety` remains a required protected context.
- `qsc-adversarial-smoke` is not skipped, disabled, weakened, or made continue-on-error.
- The qsc-adversarial workflow still invokes `scripts/ci/qsc_adversarial.sh`.
- Ordinary red-main PRs remain blocked.
- No branch-protection, protocol, runtime, crypto, demo, service, Cargo, qsc/qsl app, qsl-server, qsl-attachments, qsc-desktop, or website implementation change is part of this admission.

## Scope Guard

Allowed Packet A paths:

- `.github/workflows/public-ci.yml`
- `scripts/ci/public_safety_gate.py`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0250B_public_safety_qsc_adversarial_repair_admission_testplan.md`

PR #749 admission target paths are limited to:

- `.github/workflows/qsc-adversarial.yml`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0250A_qsc_adversarial_cargo_fuzz_install_repair_testplan.md`

## Failure Evidence

The profile requires latest `origin/main` evidence that:

- `public-safety` is completed/failure.
- `qsc-adversarial-smoke` is completed/failure.
- The qsc-adversarial failure log includes `Install cargo-fuzz`, `cargo-fuzz`, and `rustix` markers.
- `advisories` is completed success or otherwise accepted as non-failing.

## Positive Admission Proof

The positive proof is PR #749 with:

- exact PR number `749`;
- exact PR head SHA supplied by the workflow;
- sole READY item `NA-0250`;
- PR changed paths limited to the qsc-adversarial repair allowlist;
- PR-head `qsc-adversarial-smoke` completed success;
- PR #722 closed and unmerged;
- protected required-context set including `public-safety`;
- qsc-adversarial workflow text still running `scripts/ci/qsc_adversarial.sh`;
- no `continue-on-error` in the qsc-adversarial workflow.

## Negative Admission Proofs

The local fixture command must reject:

- wrong PR number;
- wrong PR head SHA;
- unrelated changed path;
- missing PR-head qsc-adversarial success;
- missing or multiple READY queue proof;
- wrong latest-main qsc-adversarial failure markers;
- missing required protected context;
- missing `public-safety` status evidence when the branch-protection API fallback is used;
- qsc-adversarial workflow `continue-on-error`.

## No-Weakening Proof

The helper inspects the PR-head qsc-adversarial workflow and fails closed unless:

- `qsc-adversarial-smoke` remains present;
- cargo-fuzz install remains version-pinned;
- `scripts/ci/qsc_adversarial.sh` remains invoked;
- `continue-on-error: true` is absent.

## Branch-Protection API Fallback Proof

The normal branch-protection API proof remains preferred. If the CI token receives GitHub API 403 for required-status-checks, fallback is allowed only for profile `qsc_adversarial_cargo_fuzz_install_repair` and requires:

- the fixed protected context set from `AGENTS.md`;
- `public-safety` present in that fixed set;
- PR check-rollup/status evidence for required contexts;
- PR-head public-safety status evidence;
- missing contexts rejected;
- CodeQL neutral accepted only under the existing GitHub acceptance rule;
- qsc-adversarial-smoke success on the PR head;
- exact profile name, PR number, and PR head SHA match.

## Queue Parser Expectation

Before Packet A merge and after Packet A merge:

- `READY_COUNT 1`
- `READY NA-0250`

## Decision Parser Expectation

After Packet A:

- D-0467 exists once.
- D-0468 is absent until PR #749 is refreshed.
- No duplicate decision IDs exist.

## CI Expectations

- Packet A public-safety must attach as a real check to the Packet A PR head.
- Workflow-dispatch bootstrap is acceptable only if GitHub counts that real public-safety check toward PR branch protection.
- PR #749 must pass `qsc-adversarial-smoke`, `public-safety`, and all protected required checks normally after Packet A.
