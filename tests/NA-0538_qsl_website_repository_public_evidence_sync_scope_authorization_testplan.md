Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-25

# NA-0538 QSL Website / Repository Public Evidence Sync Scope Authorization Testplan

Goals: G1, G2, G3, G4, G5

## Objective

Validate that NA-0538 is authorization-only, consumes the required inheritance, inventories public-facing evidence and limits, selects one future implementation scope, and preserves all no-claim and no-mutation boundaries.

## Required markers

- `NA0538_D448_CLOSEOUT_CONSUMED_OK`
- `NA0538_D446_REPEATED_RUN_SUCCESS_CONSUMED_OK`
- `NA0538_PUBLIC_SURFACE_INVENTORY_OK`
- `NA0538_PUBLIC_EVIDENCE_INVENTORY_OK`
- `NA0538_PUBLIC_CLAIM_POLICY_SELECTED_OK`
- `NA0538_PROOF_REDACTION_RULES_SELECTED_OK`
- `NA0538_FUTURE_PATH_BUNDLE_SELECTED_OK`
- `NA0538_NO_PUBLIC_DOC_IMPLEMENTATION_OK`
- `NA0538_NO_QSC_SOURCE_MUTATION_OK`
- `NA0538_NO_QSL_SERVER_ATTACHMENTS_OK`
- `NA0538_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0538_NO_PRODUCTION_READINESS_CLAIM_OK`
- `NA0538_NO_CRYPTO_COMPLETE_CLAIM_OK`
- `NA0538_ONE_READY_INVARIANT_OK`

## Validation commands

- `git diff --check`
- Exact five-path scope guard including untracked files.
- Queue/decision proof: READY_COUNT 1, READY NA-0538, D-1066 once, D-1067 absent, duplicate decision count zero.
- Link-check.
- Added-line/new-file private-material scan.
- Added-line/new-file overclaim scan.
- Docs-only classifier.
- PR body preflight.
- Goal-lint.
- Marker proof.
- Root `cargo audit --deny warnings`.
- Nested qsc fuzz lock `cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock`.
- `cargo fmt --check`.
- `sh -n scripts/ci/qsc_adversarial.sh`.
- `bash -n scripts/ci/qsc_adversarial.sh`.

## Expected result

NA-0538 passes if the only repository mutations are:

- `docs/governance/evidence/NA-0538_qsl_website_repository_public_evidence_sync_scope_authorization_plan.md`
- `tests/NA-0538_qsl_website_repository_public_evidence_sync_scope_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

NA-0538 fails if it mutates any public implementation path, qsc source/test/fuzz/Cargo path, dependency or lockfile, workflow, script/helper, corpus/vector/input, formal/refimpl/service/public/backup path, qsl-server, qsl-attachments, qsl-backup, or backup path.

## Boundary assertions

- No website/README/public-doc implementation in NA-0538.
- No qsc source/test/fuzz/Cargo mutation.
- No workflow/script/helper mutation.
- No dependency/lockfile mutation.
- No qsl-server or qsl-attachments use or mutation.
- No remote action, SSH, qsc send/receive, qsc E2EE, or qsc protocol command.
- No qwork/qstart/qresume execution by Codex.
- No qsl-backup execution.
- No public-ready claim.
- No production-ready claim.
- No public-internet-ready claim.
- No external-review-complete claim.
- No crypto-complete claim.
- No identity-complete claim.
- No trust-complete claim.
- No replay-proof claim.
- No downgrade-proof claim.
- No vulnerability-free, bug-free, or perfect-crypto claim.
