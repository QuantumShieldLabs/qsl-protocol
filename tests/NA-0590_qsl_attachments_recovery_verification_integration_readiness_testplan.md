Status: Supporting
Owner: QSL governance
Last-Updated: 2026-07-02

# NA-0590 qsl-attachments Recovery Verification and Integration Readiness Testplan

## Scope

This testplan records the NA-0590 recovery-verification and integration-readiness harness. It does not implement NA-0591, does not run full qsl-attachments send/receive integration, does not claim the true triple-ratchet path is complete, and does not claim public, production, or security completion.

## Required Markers

- NA0590_D1169_ATTACHMENTS_RECOVERY_CONSUMED_OK
- NA0590_D1170_CLOSEOUT_CONSUMED_OK
- NA0590_FRESH_QWORK_PROOF_OK
- NA0590_CURRENT_MAIN_CHECKS_CLASSIFIED_OK
- NA0590_QSL_ATTACHMENTS_PR38_RECOVERY_VERIFIED_OK
- NA0590_QSL_ATTACHMENTS_POST_RECOVERY_VALIDATION_OK
- NA0590_QSL_PROTOCOL_ATTACHMENT_EXPECTATION_REVALIDATED_OK
- NA0590_QSC_ATTACHMENT_COMMAND_SURFACE_REVALIDATED_OK
- NA0590_TRIPLE_RATCHET_PRELIMINARY_BOUNDARY_REVIEW_OK
- NA0590_RUNTIME_MODEL_REVALIDATED_OK
- NA0590_CRYPTO_OPAQUE_BOUNDARY_REVALIDATED_OK
- NA0590_AUTH_ACCESS_BOUNDARY_REVALIDATED_OK
- NA0590_STORAGE_RETENTION_RESOURCE_REVALIDATED_OK
- NA0590_METADATA_EXPOSURE_REVALIDATED_OK
- NA0590_FAILURE_BEHAVIOR_REVALIDATED_OK
- NA0590_LOCAL_SMOKE_EXECUTED_OR_DEFERRED_OK
- NA0590_FIRST_LOCAL_INTEGRATION_PLAN_SELECTED_OK
- NA0590_ISSUE_INVESTIGATION_EXECUTED_OR_SKIPPED_OK
- NA0590_SAFE_FIX_APPLIED_OR_SKIPPED_OK
- NA0590_PRIVATE_MATERIAL_SCAN_OK
- NA0590_NO_ENDPOINT_VALUE_PUBLISHED_OK
- NA0590_NO_PRIVATE_PORT_VALUE_PUBLISHED_OK
- NA0590_NO_TOKEN_OR_AUTHORIZATION_PUBLISHED_OK
- NA0590_NO_CAPABILITY_VALUE_PUBLISHED_OK
- NA0590_NO_PAYLOAD_BODY_PLAINTEXT_PUBLISHED_OK
- NA0590_NO_KEY_MATERIAL_PUBLISHED_OK
- NA0590_NO_FULL_ATTACHMENT_INTEGRATION_OK
- NA0590_NO_REMOTE_TAILSCALE_WORKFLOW_ACTION_OK
- NA0590_NO_QSL_SERVER_MUTATION_OK
- NA0590_NO_PUBLIC_READINESS_CLAIM_OK
- NA0590_NO_PRODUCTION_READINESS_CLAIM_OK
- NA0590_NO_TRIPLE_RATCHET_COMPLETE_CLAIM_OK
- NA0590_RESULT_CLASSIFICATION_SELECTED_OK
- NA0590_SUCCESSOR_SELECTED_OK
- NA0590_ONE_READY_INVARIANT_OK

## Evidence Requirements

- Verify fresh qwork proof and clean qsl-protocol startup state before fetch, qsl-attachments work, GitHub metadata review, publication, or mutation.
- Verify D-1169 and D-1170 inheritance and D-1171 absence before patch.
- Verify qsl-attachments PR #38 recovery on main, including `Cargo.lock`-only scope and recovered dependency state.
- Rerun qsl-attachments metadata, audit, fmt, test, and build after recovery.
- Revalidate qsl-protocol attachment expectations and qsc attachment command surface.
- Complete the true triple-ratchet preliminary boundary packet and preserve the dedicated verification ordering.
- Revalidate runtime, crypto/opaque-data, auth/access, storage/retention/resource, metadata, and failure behavior boundaries.
- Execute or explicitly defer optional local no-secret smoke with safety rationale.
- Select the first local attachment integration plan without running integration.
- Publish only safe classes; keep raw logs, endpoint values, private ports, tokens, Authorization values, raw capabilities, object IDs if capability-like, payloads, response bodies, plaintext, key material, and raw storage paths proof-root-only.
- Select exactly one successor and preserve exactly one READY item until closeout.

## Validation Commands

- `git diff --check`
- scope guard over tracked, staged, and untracked files
- qsl-attachments scope guard if a qsl-attachments PR is created
- queue/decision proof
- marker proof
- markdown link check
- added-line/new-file private-material scan
- qsl-attachments artifact/log private-material scan
- secret-value/prohibited-material scan
- overclaim scan
- triple-ratchet claim-boundary scan
- docs/governance/source-diagnostic classifier
- PR body preflight
- goal-lint when available
- `cargo audit --deny warnings`
- nested qsc fuzz `cargo audit --deny warnings`
- `cargo metadata --locked --format-version=1`
- `cargo fmt --check`
- `sh -n scripts/ci/qsc_adversarial.sh`
- `bash -n scripts/ci/qsc_adversarial.sh`
- qsl-attachments metadata, audit, fmt, test, and build
- focused qsc tests only if qsl-protocol/qsc attachment paths change
- additional tests for any safe fix

## Expected Result

NA-0590 records D-1171, preserves qsl-protocol source/script behavior, verifies qsl-attachments PR #38 recovery and post-recovery validation, selects `QSL_ATTACHMENTS_RECOVERY_VERIFICATION_READINESS_PASS_TRIPLE_RATCHET_VERIFY_REQUIRED`, and selects `NA-0591 -- QSL Local qsc True Triple-Ratchet E2EE Path Verification Harness` as the successor before any full attachment integration.
