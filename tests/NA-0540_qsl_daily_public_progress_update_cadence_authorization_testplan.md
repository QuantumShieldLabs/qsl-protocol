Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-25

# NA-0540 QSL Daily Public Progress Update Cadence Authorization Testplan

Goals: G1, G2, G3, G4, G5

## Objective

Validate that NA-0540 is authorization-only, consumes D452/D451 inheritance,
discovers and classifies existing public-facing repository surfaces, selects a
manual daily end-of-day Progress cadence, authorizes a site-wide public
accuracy sweep, selects exact future paths for NA-0541, and preserves no public
implementation, no public correction, no local-ops mutation, and no overclaim.

## Required Markers

- `NA0540_D452_CLOSEOUT_CONSUMED_OK`
- `NA0540_D451_PUBLIC_SYNC_PASS_CONSUMED_OK`
- `NA0540_SITE_WIDE_PUBLIC_SURFACE_INVENTORY_OK`
- `NA0540_PUBLIC_SURFACE_CLASSIFICATION_OK`
- `NA0540_DAILY_CADENCE_SELECTED_OK`
- `NA0540_END_OF_DAY_TARGET_SELECTED_OK`
- `NA0540_STABLE_HANDOFF_POLICY_SELECTED_OK`
- `NA0540_DAILY_ACCURACY_SWEEP_POLICY_SELECTED_OK`
- `NA0540_FACTUAL_CORRECTION_POLICY_SELECTED_OK`
- `NA0540_CLAIM_SAFETY_CORRECTION_POLICY_SELECTED_OK`
- `NA0540_STRUCTURAL_CHANGE_REQUIRES_AUTHORIZATION_OK`
- `NA0540_DAILY_ENTRY_TEMPLATE_SELECTED_OK`
- `NA0540_FIRST_ENTRY_SCOPE_20260625_SELECTED_OK`
- `NA0540_PUBLIC_FRONT_DOOR_PATHS_SELECTED_OK`
- `NA0540_PROGRESS_ARCHIVE_POLICY_SELECTED_OK`
- `NA0540_EVIDENCE_ELIGIBILITY_POLICY_SELECTED_OK`
- `NA0540_NO_UPDATE_POLICY_SELECTED_OK`
- `NA0540_CORRECTION_POLICY_SELECTED_OK`
- `NA0540_PROGRESS_CLAIM_POLICY_SELECTED_OK`
- `NA0540_ALL_FUTURE_PUBLIC_PATHS_EXPLICITLY_NAMED_OK`
- `NA0540_NO_INTERNAL_PRIVATE_PROMOTION_OK`
- `NA0540_MANUAL_QUEUE_DRIVEN_PROCESS_OK`
- `NA0540_AUTOMATION_DEFERRED_OK`
- `NA0540_SSD_GOVERNANCE_DEFERRED_NOT_REJECTED_OK`
- `NA0540_NO_PUBLIC_CONTENT_IMPLEMENTATION_OK`
- `NA0540_NO_PUBLIC_CORRECTION_IMPLEMENTATION_OK`
- `NA0540_NO_LOCAL_OPS_MUTATION_OK`
- `NA0540_NO_QSL_SERVER_ATTACHMENTS_OK`
- `NA0540_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0540_NO_PRODUCTION_READINESS_CLAIM_OK`
- `NA0540_ONE_READY_INVARIANT_OK`

## Validation Commands

- `git diff --check`
- Exact five-path authorization scope guard including untracked files.
- Queue/decision proof:
  - READY_COUNT 1.
  - READY NA-0540.
  - D-1070 once.
  - D-1071 absent.
  - duplicate decision count zero.
- Link-check.
- Added-line/new-file private-material scan.
- Added-line/new-file overclaim scan.
- Docs-only classifier.
- Marker proof.
- PR body preflight.
- Goal-lint.
- Root `cargo audit --deny warnings`.
- Nested qsc fuzz lock `cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock`.
- `cargo fmt --check`.
- `sh -n scripts/ci/qsc_adversarial.sh`.
- `bash -n scripts/ci/qsc_adversarial.sh`.

## Expected Result

NA-0540 passes if the only repository mutations are:

- `docs/governance/evidence/NA-0540_qsl_daily_public_progress_update_cadence_authorization_plan.md`
- `tests/NA-0540_qsl_daily_public_progress_update_cadence_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

NA-0540 fails if it mutates:

- `README.md`
- `docs/README.md`
- `docs/public/**`
- `public/**`
- `website/**`
- local maintenance scripts or systemd units
- qwork/qbuild/shared-target tooling
- qsc source/test/fuzz/Cargo paths
- dependency/lockfile paths
- workflows, scripts, or helpers
- corpus/vector/input paths
- formal/refimpl/service/public/backup paths outside the authorization files
- qsl-server or qsl-attachments paths

## Boundary Assertions

- NA-0540 is authorization-only.
- No daily public Progress content is implemented.
- No public page correction is implemented.
- No README/docs/public mutation occurs.
- No `docs/public/PROGRESS.md` or `docs/public/progress/**` path is created.
- No `public/` or `website/` path is created or mutated.
- No local-ops script, timer, service, cron, or automation mutation occurs.
- No qwork, qstart, or qresume is executed by Codex.
- No qsl-backup execution occurs.
- No qsc source/test/fuzz/Cargo mutation occurs.
- No workflow/script/helper mutation occurs.
- No dependency/lockfile mutation occurs.
- No qsl-server or qsl-attachments use or mutation occurs.
- No public-readiness claim is introduced.
- No production-readiness claim is introduced.
- No public-internet-readiness claim is introduced.
- No external-review-complete claim is introduced.
- No crypto-complete claim is introduced.
- No identity-complete claim is introduced.
- No trust-complete claim is introduced.
- No replay-proof claim is introduced.
- No downgrade-proof claim is introduced.
- No vulnerability-free, bug-free, or perfect-crypto claim is introduced.

## Focused Runtime Test Skip Rationale

Focused qsc runtime tests may be skipped because NA-0540 is
authorization-only, changes only docs/governance/testplan files, does not
mutate qsc runtime/source/dependency/workflow paths, and current main
public-safety and advisories were green before the authorization patch.
