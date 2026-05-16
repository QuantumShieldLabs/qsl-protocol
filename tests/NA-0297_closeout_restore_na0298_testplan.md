Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-16
Replaces: n/a
Superseded-By: n/a

# NA-0297 Closeout Restore NA-0298 Testplan

## Objective

Close NA-0297 after the website source blocker resolution audit merged and
restore exactly one READY successor: NA-0298 Website Source Operator Action and
Implementation Blocker Resolution.

## Protected Invariants

- NA-0297 closeout does not implement NA-0298.
- NA-0297 closeout does not mutate a website or external website repository.
- Operator authority is not assumed from public source clues.
- Source/deploy blocker documentation does not equal a live website update.
- Production readiness, public internet readiness, external review completion,
  anonymity, metadata-free messaging, and untraceability remain unclaimed.
- public-safety remains required and green.
- Exactly one READY queue item exists after closeout.

## Allowed Scope

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0297_closeout_restore_na0298_testplan.md`

## Forbidden Scope

- `README.md`
- `START_HERE.md`
- `docs/public/**`
- `.github/**`
- `scripts/**`
- `Cargo.toml`
- `Cargo.lock`
- `qsp/**`
- `qsc/**`
- `qsl/**`
- `qsl-client/**`
- `apps/**`
- `tools/**`
- `inputs/**`
- `formal/**`
- `qsc-desktop/**`
- `qsl-server/**`
- `qsl-attachments/**`
- `website/**`
- any external website repository
- runtime, protocol, crypto, demo, or service implementation paths
- branch-protection or public-safety configuration

## Queue Expectations

- Before closeout: READY_COUNT `1`, READY `NA-0297`.
- After closeout patch: READY_COUNT `1`, READY `NA-0298`.
- NA-0297 is DONE.
- NA-0298 remains blocker-resolution/operator-action work, not website
  implementation.

## Decision Expectations

- D-0570 exists exactly once before closeout.
- D-0571 exists exactly once after closeout.
- D-0572 remains absent.
- No duplicate decision IDs appear.

## Successor Selection Expectations

- If source/deploy/authority remains unverified, NA-0298 must be restored as
  Website Source Operator Action and Implementation Blocker Resolution.
- The successor must request exact operator-supplied source repository, branch,
  build command, preview/staging command, deployment target, rollback process,
  hosting/deployment path, and future PR/build authorization.
- The successor must forbid website implementation unless a later directive
  verifies source, authority, preview, deploy, rollback, claim scan, and link
  scan.

## Validation Expectations

Required local validation:

- `git diff --check origin/main...HEAD`
- changed-line overclaim scan
- `python3 scripts/ci/qsl_evidence_helper.py queue`
- `python3 scripts/ci/qsl_evidence_helper.py decisions`
- `python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main`
  with exactly the closeout allowed paths
- `python3 scripts/ci/qsl_evidence_helper.py link-check`
- `python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main`
- `cargo audit --deny warnings`
- `cargo tree -i rustls-webpki --locked`
- `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1`
- `python3 formal/run_model_checks.py` when present
- goal-lint with a PR body containing `Goals: G1, G2, G3, G4, G5`
- changed-path classifier proof

## CI Expectations

- PR checks attach without watch mode.
- Required checks pass normally before merge.
- `public-safety` remains required by branch protection.
- Post-merge main `public-safety` completes success.
- Docs/governance-only cost-control may skip heavy full-suite jobs only when
  classifiers prove the changed paths are docs-only.

## Rejection Conditions

Stop if any validation shows:

- more than one READY item;
- READY item other than NA-0298 after closeout;
- NA-0297 not DONE;
- D-0571 missing or duplicated;
- D-0572 present;
- unsupported production-readiness, public-internet-readiness,
  external-review-complete, anonymity, metadata-free, untraceable,
  website-updated, or implementation claim;
- website or external website mutation;
- forbidden implementation, workflow, script, Cargo, dependency, or
  branch-protection path changes;
- public-safety red or missing.
