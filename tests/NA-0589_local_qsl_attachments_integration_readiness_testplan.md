Status: Supporting
Owner: QSL governance
Last-Updated: 2026-07-02

# NA-0589 Local qsl-attachments Integration Readiness Testplan

## Scope

This testplan records the NA-0589 readiness and boundary-classification harness. It does not implement NA-0590 and does not claim public, production, or security completion.

## Required Markers

- NA0589_D1167_LOCAL_STRESS_PASS_CONSUMED_OK
- NA0589_D1168_CLOSEOUT_CONSUMED_OK
- NA0589_FRESH_QWORK_PROOF_OK
- NA0589_CURRENT_MAIN_CHECKS_CLASSIFIED_OK
- NA0589_QSL_ATTACHMENTS_SOURCE_ACQUIRED_OK
- NA0589_QSL_ATTACHMENTS_BUILD_AUDIT_TEST_CLASSIFIED_OK
- NA0589_QSL_PROTOCOL_ATTACHMENT_EXPECTATION_REVIEW_OK
- NA0589_QSC_ATTACHMENT_COMMAND_SURFACE_REVIEW_OK
- NA0589_RUNTIME_MODEL_CLASSIFIED_OK
- NA0589_CRYPTO_OPAQUE_BOUNDARY_REVIEW_OK
- NA0589_AUTH_ACCESS_BOUNDARY_REVIEW_OK
- NA0589_STORAGE_RETENTION_RESOURCE_REVIEW_OK
- NA0589_METADATA_EXPOSURE_REVIEW_OK
- NA0589_FAILURE_BEHAVIOR_REVIEW_OK
- NA0589_LOCAL_SMOKE_EXECUTED_OR_DEFERRED_OK
- NA0589_FIRST_LOCAL_INTEGRATION_PLAN_SELECTED_OK
- NA0589_ISSUE_INVESTIGATION_EXECUTED_OR_SKIPPED_OK
- NA0589_SAFE_FIX_APPLIED_OR_SKIPPED_OK
- NA0589_PRIVATE_MATERIAL_SCAN_OK
- NA0589_NO_ENDPOINT_VALUE_PUBLISHED_OK
- NA0589_NO_PRIVATE_PORT_VALUE_PUBLISHED_OK
- NA0589_NO_TOKEN_OR_AUTHORIZATION_PUBLISHED_OK
- NA0589_NO_PAYLOAD_BODY_PLAINTEXT_PUBLISHED_OK
- NA0589_NO_KEY_MATERIAL_PUBLISHED_OK
- NA0589_NO_REMOTE_TAILSCALE_WORKFLOW_ACTION_OK
- NA0589_NO_QSL_SERVER_MUTATION_OK
- NA0589_NO_PUBLIC_READINESS_CLAIM_OK
- NA0589_NO_PRODUCTION_READINESS_CLAIM_OK
- NA0589_RESULT_CLASSIFICATION_SELECTED_OK
- NA0589_SUCCESSOR_SELECTED_OK
- NA0589_ONE_READY_INVARIANT_OK

## Evidence Requirements

- Verify fresh qwork proof and clean qsl-protocol startup state before fetch or mutation.
- Verify D-1167 and D-1168 inheritance and D-1169 absence before patch.
- Classify qsl-attachments source inventory, runtime model, crypto/opaque boundary, auth/access boundary, storage/retention/resource boundary, metadata exposure, and failure behavior.
- Record qsl-attachments validation before and after any authorized recovery.
- Record qsl-attachments PR evidence if a qsl-attachments recovery is performed.
- Publish only safe classes; keep raw logs, service details, tokens, capabilities, payloads, plaintext, key material, and raw storage paths proof-root-only.
- Select exactly one successor and preserve exactly one READY item until closeout.

## Validation Commands

- `git diff --check`
- scope guard over tracked, staged, and untracked files
- queue/decision proof
- marker proof
- markdown link check
- added-line/new-file private-material scan
- qsl-attachments artifact/log private-material scan
- overclaim scan
- PR body preflight
- goal-lint when available
- `cargo audit --deny warnings`
- nested qsc fuzz `cargo audit --deny warnings`
- `cargo metadata --locked --format-version=1`
- `cargo fmt --check`
- `sh -n scripts/ci/qsc_adversarial.sh`
- `bash -n scripts/ci/qsc_adversarial.sh`
- qsl-attachments metadata, audit, fmt, test, and build after recovery

## Expected Result

NA-0589 records D-1169, preserves qsl-protocol source/script behavior, records qsl-attachments PR #38 lockfile-only recovery evidence, selects `QSL_ATTACHMENTS_LOCKFILE_ONLY_RECOVERY_IMPLEMENTED`, and restores only a recovery-verification successor for NA-0590 during optional closeout.
