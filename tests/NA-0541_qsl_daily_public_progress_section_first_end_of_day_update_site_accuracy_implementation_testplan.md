Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-25
Replaces: n/a
Superseded-By: n/a

# NA-0541 Daily Public Progress Section, First End-of-Day Update, and Site Accuracy Implementation Testplan

## Scope

This testplan covers the D-1070-authorized NA-0541 public Progress
implementation and public accuracy sweep. It is public/docs/governance only.

## Required Markers

- NA0541_D1070_AUTHORIZATION_CONSUMED_OK
- NA0541_SELECTED_PATH_BUNDLE_ONLY_OK
- NA0541_PROGRESS_SECTION_ADDED_OK
- NA0541_DAILY_PROGRESS_TEMPLATE_APPLIED_OK
- NA0541_FIRST_ENTRY_20260625_PUBLISHED_OK
- NA0541_SITE_WIDE_PUBLIC_ACCURACY_SCAN_OK
- NA0541_STALE_STATUS_REFERENCE_SCAN_OK
- NA0541_BROKEN_PUBLIC_LINK_SCAN_OK
- NA0541_PUBLIC_CLAIM_CONSISTENCY_SCAN_OK
- NA0541_VERIFIED_INACCURACIES_CORRECTED_OK
- NA0541_SITE_CORRECTIONS_RECORDED_OK
- NA0541_OUT_OF_SCOPE_CORRECTIONS_DEFERRED_OK
- NA0541_NO_INTERNAL_PRIVATE_MATERIAL_PUBLISHED_OK
- NA0541_EVIDENCE_LINKS_RESOLVE_OK
- NA0541_NO_RAW_PROOF_LOGS_OK
- NA0541_NO_PRIVATE_MATERIAL_OK
- NA0541_NO_QSC_SOURCE_MUTATION_OK
- NA0541_NO_QSL_SERVER_ATTACHMENTS_OK
- NA0541_NO_PUBLIC_READINESS_CLAIM_OK
- NA0541_NO_PRODUCTION_READINESS_CLAIM_OK
- NA0541_NO_CRYPTO_COMPLETE_CLAIM_OK
- NA0541_ONE_READY_INVARIANT_OK

## Validation Commands

- `git diff --check`
- exact D-1070 allowlist scope guard
- queue/decision proof
- public scan record proof
- correction ledger proof
- repository relative-link check
- changed-public-file relative-link check
- Progress link proof
- added-line/new-file private-material scan
- changed-public-file private-material scan
- internal/private-promotion scan
- raw-proof-log/reference scan
- added-line/new-file overclaim scan
- changed-public-file overclaim scan
- Progress-entry claim-boundary proof
- claim matrix proof
- stale-status reference scan
- public-path contradiction scan
- qsl-server/qsl-attachments implication scan
- docs/public classifier
- marker proof
- PR body preflight
- goal-lint
- `cargo audit --deny warnings`
- `cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock`
- `cargo fmt --check`
- `sh -n scripts/ci/qsc_adversarial.sh`
- `bash -n scripts/ci/qsc_adversarial.sh`

## Runtime-Test Skip Rationale

Focused qsc runtime tests may be skipped because NA-0541 mutates only
public/docs/governance paths, creates no qsc runtime/source/dependency/workflow
change, and preserves qsl-server/qsl-attachments boundaries.

## Expected Result

The implementation passes with classification
`DAILY_PUBLIC_PROGRESS_SITE_ACCURACY_IMPLEMENTATION_PASS`, no forbidden path
mutation, no private material exposure, no public/website path creation, no
automation, no dependency/lockfile mutation, and no public/production/security
completion overclaim.
