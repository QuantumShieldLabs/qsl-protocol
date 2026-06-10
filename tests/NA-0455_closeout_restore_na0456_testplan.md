Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-10

# NA-0455 Closeout and NA-0456 Restoration Testplan

## Scope

This closeout testplan records the governance-only validation for closing
NA-0455 after PR #1179 and restoring the selected qsc-first NA-0456 successor.
It does not implement NA-0456 and does not authorize runtime, crypto,
dependency, Cargo, lockfile, workflow, executable test, fuzz target, vector,
formal model, qsl-server, qsl-attachments, qshield runtime, qshield-cli,
website, public-doc, README, START_HERE, qwork/qstart/qresume/qshell,
qsl-backup, backup, restore, status/plan, rollback, backup tree, or
public-surface mutation.

## Required Markers

- NA0455_CLOSEOUT_PR1179_MERGED_OK
- NA0455_CLOSEOUT_POST_MERGE_PUBLIC_SAFETY_OK
- NA0455_DONE_OK
- NA0456_QSC_PROVIDER_RNG_SCOPE_RESTORED_READY_OK
- NA0456_NO_IMPLEMENTATION_BY_CLOSEOUT_OK
- NA0456_NO_DEPENDENCY_CHANGE_OK
- NA0456_NO_WORKFLOW_CHANGE_OK
- NA0456_NO_PUBLIC_READINESS_CLAIM_OK
- NA0456_NO_CRYPTO_COMPLETE_CLAIM_OK
- NA0456_NO_RNG_FAILURE_COMPLETE_CLAIM_OK
- NA0456_NO_PROVIDER_RNG_COMPLETE_CLAIM_OK
- NA0456_ONE_READY_INVARIANT_OK

## Validation Commands

- `git diff --check`
- exact closeout scope guard for:
  - `NEXT_ACTIONS.md`
  - `DECISIONS.md`
  - `TRACEABILITY.md`
  - `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
  - `tests/NA-0455_closeout_restore_na0456_testplan.md`
- local markdown link check
- added-line leak scan
- added-line overclaim scan
- PR body preflight
- local goal-lint preflight
- `python3 scripts/ci/qsl_evidence_helper.py queue`
- `python3 scripts/ci/qsl_evidence_helper.py decisions`
- `cargo audit --deny warnings`
- `cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock`
- `python3 scripts/ci/qsl_evidence_helper.py public-safety-status --sha <merge-sha> --repo QuantumShieldLabs/qsl-protocol --report-only`

## Acceptance Criteria

- NA-0455 is DONE.
- NA-0456 is the sole READY item.
- D-0898 exists once.
- D-0899 is absent before NA-0456 begins.
- Duplicate decision count is zero.
- Changed paths are limited to the five closeout paths.
- No implementation mutation occurs.
- No backup or restore is run.
- qsl-backup, backup status files, and backup plan files are not mutated.
- Root cargo audit is green.
- Nested qsc fuzz lock audit is green.
- Public-safety is green before closeout merge and after closeout merge.
- No public-readiness, production-readiness, external-review-complete,
  crypto-complete, side-channel-free, RNG-failure-complete,
  provider-RNG-complete, vulnerability-free, bug-free, or perfect-crypto claim
  is introduced.

## Residuals

- NA-0456 remains authorization-only until a later directive executes it.
- qsc provider-dependent RNG no-mutation evidence remains unimplemented.
- refimpl provider RNG boundary evidence remains unimplemented.
- Provider RNG evidence gaps remain gaps and must not be converted into
  guarantees.
- Cargo audit green remains dependency-health evidence only.
