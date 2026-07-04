Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-07-04

Goals: G1, G2, G3, G4, G5

# NA-0598 qsl-server Exact 4 MiB Relay Boundary Fix Testplan

## Required Markers

- NA0598_D1185_QSL_SERVER_BUG_CONSUMED_OK
- NA0598_D1186_CLOSEOUT_CONSUMED_OK
- NA0598_FRESH_QWORK_PROOF_OK
- NA0598_CURRENT_MAIN_CHECKS_CLASSIFIED_OK
- NA0598_QSL_SERVER_SOURCE_REVIEW_OK
- NA0598_QSL_SERVER_REPRODUCTION_OK
- NA0598_FIX_STRATEGY_SELECTED_OK
- NA0598_QSL_SERVER_FIX_APPLIED_OR_NOT_NEEDED_OK
- NA0598_QSL_SERVER_PR_MERGED_OR_NOT_NEEDED_OK
- NA0598_QSL_SERVER_VALIDATION_OK
- NA0598_EXACT_4MIB_RETEST_OK
- NA0598_ABOVE_THRESHOLD_CONTROLS_OK
- NA0598_RESOURCE_AUTH_REGRESSION_TESTS_OK
- NA0598_METADATA_REVIEW_OK
- NA0598_CLEANUP_DONE_OK
- NA0598_PRIVATE_MATERIAL_SCAN_OK
- NA0598_NO_QSC_THRESHOLD_CHANGE_OK
- NA0598_NO_QSL_ATTACHMENTS_PATH_CHANGE_OK
- NA0598_NO_ENDPOINT_VALUE_PUBLISHED_OK
- NA0598_NO_PRIVATE_PORT_VALUE_PUBLISHED_OK
- NA0598_NO_TOKEN_OR_AUTHORIZATION_PUBLISHED_OK
- NA0598_NO_CAPABILITY_VALUE_PUBLISHED_OK
- NA0598_NO_PAYLOAD_BODY_PLAINTEXT_PUBLISHED_OK
- NA0598_NO_SEED_OR_KEY_MATERIAL_PUBLISHED_OK
- NA0598_NO_REMOTE_TAILSCALE_WORKFLOW_ACTION_OK
- NA0598_NO_PUBLIC_READINESS_CLAIM_OK
- NA0598_NO_PRODUCTION_READINESS_CLAIM_OK
- NA0598_NO_CRYPTO_COMPLETE_CLAIM_OK
- NA0598_NO_ATTACHMENT_COMPLETE_CLAIM_OK
- NA0598_RESULT_CLASSIFICATION_SELECTED_OK
- NA0598_SUCCESSOR_SELECTED_OK
- NA0598_ONE_READY_INVARIANT_OK

## Validation Commands

- qwork proof verification: PASS.
- startup queue/decision proof: PASS.
- current main required-check classification: PASS.
- qsl-server source review and reproduction: PASS.
- qsl-server validation before patch, on fix branch, and post-merge: PASS.
- qsl-server PR #59 merge proof: PASS.
- local exact 4 MiB qsc/qsl-server retest: PASS.
- just-over-4 MiB and known-good greater-than-4-MiB qsl-attachments controls: PASS.
- qsl-server resource/auth regression tests: PASS.
- metadata/private-material scans: PASS.
- cleanup proof: PASS.
- final result classification: `QSL_SERVER_EXACT_4MIB_RELAY_BOUNDARY_FIX_PASS`.
- selected successor: `NA-0599 -- QSL Remote / Tailnet Full-Stack Reintroduction Readiness Harness`.

## Boundary Assertions

- qsc threshold behavior is unchanged.
- exact 4 MiB remains legacy in-message.
- qsl-attachments remains not used for exact 4 MiB.
- greater-than-4-MiB controls continue to use qsl-attachments.
- qsl-server queues remain bounded.
- qsl-server auth and route isolation remain fail-closed.
- no dependency or lockfile mutation occurred.
- no remote, Tailscale, workflow, deployment, public-site, or Cloudflare action occurred.
- no private material is published.
