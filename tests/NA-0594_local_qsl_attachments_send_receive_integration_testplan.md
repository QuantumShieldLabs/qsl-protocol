Status: Supporting
Owner: QSL governance
Last-Updated: 2026-07-02

# NA-0594 local qsl-attachments send / receive integration testplan

## Required markers

- NA0594_D1177_SEED_FALLBACK_HARDENING_CONSUMED_OK
- NA0594_D1178_CLOSEOUT_CONSUMED_OK
- NA0594_FRESH_QWORK_PROOF_OK
- NA0594_CURRENT_MAIN_CHECKS_CLASSIFIED_OK
- NA0594_QSC_ATTACHMENT_SURFACE_REVALIDATED_OK
- NA0594_QSL_SERVER_BOUNDARY_REVALIDATED_OK
- NA0594_QSL_ATTACHMENTS_RUNTIME_REVALIDATED_OK
- NA0594_QSL_SERVER_VALIDATION_OK
- NA0594_QSL_ATTACHMENTS_VALIDATION_OK
- NA0594_QSC_FOCUSED_VALIDATION_OK
- NA0594_INTEGRATION_HARNESS_DESIGNED_OK
- NA0594_LOCAL_RUNTIME_STARTED_OK
- NA0594_ATTACHMENTS_SEND_RECEIVE_CLASSIFIED_OK
- NA0594_QSC_ENCRYPTION_OWNER_CONFIRMED_OK
- NA0594_QSL_SERVER_CONTROL_PLANE_ONLY_OK
- NA0594_QSL_ATTACHMENTS_OPAQUE_STORAGE_OK
- NA0594_SEED_FALLBACK_BLOCKED_IN_ATTACHMENT_PATH_OK
- NA0594_NEGATIVES_CLASSIFIED_OK
- NA0594_METADATA_REVIEW_OK
- NA0594_CLEANUP_DONE_OK
- NA0594_ISSUE_INVESTIGATION_EXECUTED_OR_SKIPPED_OK
- NA0594_SAFE_FIX_APPLIED_OR_SKIPPED_OK
- NA0594_PRIVATE_MATERIAL_SCAN_OK
- NA0594_NO_ENDPOINT_VALUE_PUBLISHED_OK
- NA0594_NO_PRIVATE_PORT_VALUE_PUBLISHED_OK
- NA0594_NO_TOKEN_OR_AUTHORIZATION_PUBLISHED_OK
- NA0594_NO_CAPABILITY_VALUE_PUBLISHED_OK
- NA0594_NO_PAYLOAD_BODY_PLAINTEXT_PUBLISHED_OK
- NA0594_NO_SEED_OR_KEY_MATERIAL_PUBLISHED_OK
- NA0594_NO_REMOTE_TAILSCALE_WORKFLOW_ACTION_OK
- NA0594_NO_PUBLIC_READINESS_CLAIM_OK
- NA0594_NO_PRODUCTION_READINESS_CLAIM_OK
- NA0594_NO_CRYPTO_COMPLETE_CLAIM_OK
- NA0594_NO_ATTACHMENT_COMPLETE_CLAIM_OK
- NA0594_RESULT_CLASSIFICATION_SELECTED_OK
- NA0594_SUCCESSOR_SELECTED_OK
- NA0594_ONE_READY_INVARIANT_OK

## Focused validation commands

- `cargo metadata --locked --format-version=1`
- `cargo audit --deny warnings`
- `cargo fmt --check`
- `cargo test -p qsc --test na_0591_true_triple_ratchet_path`
- `cargo test -p qsc --test na_0593_seed_fallback_hardening`
- `cargo test -p qsc --test attachments_contract_na0217h`
- `cargo test -p qsc --test attachment_streaming_na0197c`
- `cargo test -p qsc --test qsp_protocol_gate`
- `cargo test -p qsc --test relay_auth_header`
- `cargo test -p qsc --test receive_e2e_peer_separation`
- qsl-server: metadata, audit, fmt, test, and build.
- qsl-attachments: metadata, audit, fmt, test, and build.
- proof-root local qsl-attachments send/receive harness.

## Expected outcome

NA-0594 records D-1179, proves a local qsc/qsl-server/qsl-attachments
attachment-bearing send/receive exchange with proof-root-only state, classifies
selected fail-closed negatives and residual metadata, preserves qsc-owned
encryption/decryption, preserves qsl-server relay/control-plane-only behavior,
preserves qsl-attachments opaque storage behavior, and selects NA-0595 local
adversarial and metadata stress as the next READY successor without implementing
NA-0595 in this implementation PR.
