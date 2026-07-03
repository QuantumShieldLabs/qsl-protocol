Status: Supporting
Owner: QSL governance
Last-Updated: 2026-07-03

# NA-0595 local qsl-attachments adversarial metadata stress testplan

## Required markers

- NA0595_D1179_ATTACHMENT_INTEGRATION_CONSUMED_OK
- NA0595_D1180_CLOSEOUT_CONSUMED_OK
- NA0595_FRESH_QWORK_PROOF_OK
- NA0595_CURRENT_MAIN_CHECKS_CLASSIFIED_OK
- NA0595_QSC_ATTACHMENT_SURFACE_REVALIDATED_OK
- NA0595_QSL_SERVER_BOUNDARY_REVALIDATED_OK
- NA0595_QSL_ATTACHMENTS_RUNTIME_REVALIDATED_OK
- NA0595_QSL_SERVER_VALIDATION_OK
- NA0595_QSL_ATTACHMENTS_VALIDATION_OK
- NA0595_QSC_FOCUSED_VALIDATION_OK
- NA0595_STRESS_HARNESS_DESIGNED_OK
- NA0595_BASELINE_REVALIDATED_OK
- NA0595_REPETITION_STRESS_CLASSIFIED_OK
- NA0595_MULTI_ATTACHMENT_STRESS_CLASSIFIED_OK
- NA0595_THRESHOLD_BOUNDARY_CLASSIFIED_OK
- NA0595_CAPABILITY_NEGATIVES_CLASSIFIED_OK
- NA0595_DESCRIPTOR_OBJECT_NEGATIVES_CLASSIFIED_OK
- NA0595_ROUTE_PEER_NEGATIVES_CLASSIFIED_OK
- NA0595_RESTART_RETENTION_CLASSIFIED_OK
- NA0595_CONCURRENCY_CLASSIFIED_OK
- NA0595_METADATA_MATRIX_OK
- NA0595_CLEANUP_DONE_OK
- NA0595_ISSUE_INVESTIGATION_EXECUTED_OR_SKIPPED_OK
- NA0595_SAFE_FIX_APPLIED_OR_SKIPPED_OK
- NA0595_PRIVATE_MATERIAL_SCAN_OK
- NA0595_NO_ENDPOINT_VALUE_PUBLISHED_OK
- NA0595_NO_PRIVATE_PORT_VALUE_PUBLISHED_OK
- NA0595_NO_TOKEN_OR_AUTHORIZATION_PUBLISHED_OK
- NA0595_NO_CAPABILITY_VALUE_PUBLISHED_OK
- NA0595_NO_PAYLOAD_BODY_PLAINTEXT_PUBLISHED_OK
- NA0595_NO_SEED_OR_KEY_MATERIAL_PUBLISHED_OK
- NA0595_NO_REMOTE_TAILSCALE_WORKFLOW_ACTION_OK
- NA0595_NO_PUBLIC_READINESS_CLAIM_OK
- NA0595_NO_PRODUCTION_READINESS_CLAIM_OK
- NA0595_NO_CRYPTO_COMPLETE_CLAIM_OK
- NA0595_NO_ATTACHMENT_COMPLETE_CLAIM_OK
- NA0595_RESULT_CLASSIFICATION_SELECTED_OK
- NA0595_SUCCESSOR_SELECTED_OK
- NA0595_ONE_READY_INVARIANT_OK

## Focused validation commands

- `cargo metadata --locked --format-version=1`
- `cargo audit --deny warnings`
- nested qsc fuzz `cargo audit --deny warnings`
- `cargo fmt --check`
- `sh -n scripts/ci/qsc_adversarial.sh`
- `bash -n scripts/ci/qsc_adversarial.sh`
- `cargo test -p qsc --test na_0591_true_triple_ratchet_path`
- `cargo test -p qsc --test na_0593_seed_fallback_hardening`
- `cargo test -p qsc --test attachments_contract_na0217h`
- `cargo test -p qsc --test attachment_streaming_na0197c`
- `cargo test -p qsc --test qsp_protocol_gate`
- `cargo test -p qsc --test relay_auth_header`
- `cargo test -p qsc --test receive_e2e receive_mailbox_peer_separation_fail_closed`
- qsl-server: metadata, audit, fmt, test, and build.
- qsl-attachments: metadata, audit, fmt, test, and build.
- proof-root local qsl-attachments adversarial and metadata stress harness.

## Expected outcome

NA-0595 records D-1181, preserves the local-only qsc/qsl-server/qsl-attachments
attachment boundary, classifies baseline/repetition/multi-size/negative/
restart/concurrency/metadata/cleanup results, records the exact 4 MiB
resource-boundary gap, and selects `NA-0596 -- QSL Local Attachment Stress
Diagnostic Follow-Up Harness` without implementing NA-0596.
