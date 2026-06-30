Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-30

# NA-0578 qsl-server Failed Start Cause Corrected Loopback Proof Testplan

This testplan records the marker set for the NA-0578 failed-start cause classification and corrected loopback proof harness.

## Required Markers

- NA0578_D1145_FAILED_START_CONSUMED_OK
- NA0578_D1146_CLOSEOUT_CONSUMED_OK
- NA0578_FRESH_QWORK_PROOF_OK
- NA0578_CURRENT_MAIN_CHECKS_CLASSIFIED_OK
- NA0578_AUTHORITY_MODEL_APPLIED_OK
- NA0578_FAILURE_CAUSE_INVESTIGATION_COMPLETED_OK
- NA0578_QSL_SERVER_CLI_SOURCE_REVIEW_OK
- NA0578_BIND_PORT_COMMAND_SHAPE_CLASSIFIED_OK
- NA0578_REMOTE_SCRIPTS_STATIC_REVIEW_OK
- NA0578_SSH_READINESS_CLASSIFIED_OK
- NA0578_REMOTE_INSPECTION_CLASSIFIED_OK
- NA0578_CORRECTED_BOUNDED_START_EXECUTED_OR_SKIPPED_OK
- NA0578_CORRECTED_POSTCHECK_EXECUTED_OR_SKIPPED_OK
- NA0578_CLEANUP_EXECUTED_OR_SKIPPED_OK
- NA0578_PRIVATE_MATERIAL_SCAN_OK
- NA0578_RESULT_CLASSIFICATION_SELECTED_OK
- NA0578_SUCCESSOR_SELECTED_OK
- NA0578_NO_ENDPOINT_VALUE_PUBLISHED_OK
- NA0578_NO_PRIVATE_PORT_VALUE_PUBLISHED_OK
- NA0578_NO_TOKEN_OR_AUTHORIZATION_PUBLISHED_OK
- NA0578_NO_PROCESS_IDENTITY_PUBLISHED_OK
- NA0578_NO_COMMAND_LINE_PUBLISHED_OK
- NA0578_NO_PAYLOAD_BODY_PUBLISHED_OK
- NA0578_NO_SUDO_SYSTEMCTL_TAILSCALE_OK
- NA0578_NO_ACCOUNT_AUTHORIZED_KEYS_MUTATION_OK
- NA0578_NO_WRITES_OUTSIDE_QSLCODEX_TEST_WORKSPACE_OK
- NA0578_NO_QSC_SEND_RECEIVE_OK
- NA0578_NO_WORKFLOW_DISPATCH_OK
- NA0578_NO_QSL_ATTACHMENTS_OK
- NA0578_NO_QSL_PROTOCOL_SOURCE_MUTATION_OK
- NA0578_NO_QSL_SERVER_SOURCE_MUTATION_OK
- NA0578_NO_PRIVATE_MATERIAL_PUBLISHED_OK
- NA0578_NO_PUBLIC_READINESS_CLAIM_OK
- NA0578_NO_PRODUCTION_READINESS_CLAIM_OK
- NA0578_ONE_READY_INVARIANT_OK

## Classification Proof

- Failed-start cause: `QSL_SERVER_FAILED_START_CAUSE_HARNESS_BIND_ARG_BUG_CONFIRMED`.
- SSH readiness: `SSH_QSL_SERVER_FAILED_START_FOLLOWUP_READY`.
- Remote inspection: `QSL_SERVER_FAILED_START_FOLLOWUP_CORRECTED_START_SAFE`.
- Corrected bounded start: `QSL_SERVER_CORRECTED_BOUNDED_START_AMBIGUOUS_STOP`.
- Corrected postcheck: `QSL_SERVER_CORRECTED_POSTCHECK_LISTENER_NOT_READY`.
- Cleanup: `QSL_SERVER_CORRECTED_CLEANUP_NOT_NEEDED`.
- Result: `QSL_SERVER_FAILED_START_INSUFFICIENT_PROOF`.
- Selected successor: `NA-0579 -- QSL Remote qsl-server Failed Start Proof Completion Follow-Up Harness`.

## Boundary Proof

The lane publishes only safe classes. It does not publish endpoint values, private port values, route-token/capability values, bearer values, Authorization values, private topology, process identity, command lines, payloads, response bodies, authorized_keys content, key material, raw private logs, or secret values.

No sudo, systemctl, service, firewall, Tailscale, account, shell, authorized_keys mutation, qsc send/receive, workflow dispatch/rerun, qsl-attachments work, qsl-server source mutation, qsl-protocol source/script/workflow/dependency mutation, public-site mutation, Cloudflare mutation, public-readiness claim, production-readiness claim, vulnerability-free claim, bug-free claim, perfect-build claim, or perfect-crypto claim is introduced.
