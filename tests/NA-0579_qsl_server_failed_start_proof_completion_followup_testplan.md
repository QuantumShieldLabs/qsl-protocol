Status: Supporting
Owner: QSL governance
Last-Updated: 2026-07-01

# NA-0579 qsl-server Failed Start Proof Completion Follow-Up Testplan

This testplan records the marker set for the NA-0579 failed-start proof completion follow-up harness.

## Required Markers

- NA0579_D1147_FAILED_START_FOLLOWUP_CONSUMED_OK
- NA0579_D1148_CLOSEOUT_CONSUMED_OK
- NA0579_FRESH_QWORK_PROOF_OK
- NA0579_CURRENT_MAIN_CHECKS_CLASSIFIED_OK
- NA0579_AUTHORITY_MODEL_APPLIED_OK
- NA0579_AUTOMATIC_FAILURE_CAUSE_POLICY_APPLIED_OK
- NA0579_GATING_DEFECT_INVESTIGATION_COMPLETED_OK
- NA0579_LOCAL_DRY_RUN_ASSERTION_OK
- NA0579_QSL_SERVER_CLI_SOURCE_REVIEW_OK
- NA0579_REMOTE_SCRIPTS_STATIC_REVIEW_OK
- NA0579_SSH_READINESS_CLASSIFIED_OK
- NA0579_REMOTE_INSPECTION_CLASSIFIED_OK
- NA0579_CORRECTED_BOUNDED_START_EXECUTED_OR_SKIPPED_OK
- NA0579_CORRECTED_POSTCHECK_EXECUTED_OR_SKIPPED_OK
- NA0579_CLEANUP_EXECUTED_OR_SKIPPED_OK
- NA0579_PRIVATE_MATERIAL_SCAN_OK
- NA0579_RESULT_CLASSIFICATION_SELECTED_OK
- NA0579_SUCCESSOR_SELECTED_OK
- NA0579_NO_ENDPOINT_VALUE_PUBLISHED_OK
- NA0579_NO_PRIVATE_PORT_VALUE_PUBLISHED_OK
- NA0579_NO_TOKEN_OR_AUTHORIZATION_PUBLISHED_OK
- NA0579_NO_PROCESS_IDENTITY_PUBLISHED_OK
- NA0579_NO_COMMAND_LINE_PUBLISHED_OK
- NA0579_NO_PAYLOAD_BODY_PUBLISHED_OK
- NA0579_NO_SUDO_SYSTEMCTL_TAILSCALE_OK
- NA0579_NO_ACCOUNT_AUTHORIZED_KEYS_MUTATION_OK
- NA0579_NO_WRITES_OUTSIDE_QSLCODEX_TEST_WORKSPACE_OK
- NA0579_NO_QSC_SEND_RECEIVE_OK
- NA0579_NO_WORKFLOW_DISPATCH_OK
- NA0579_NO_QSL_ATTACHMENTS_OK
- NA0579_NO_QSL_PROTOCOL_SOURCE_MUTATION_OK
- NA0579_NO_QSL_SERVER_SOURCE_MUTATION_OK
- NA0579_NO_PRIVATE_MATERIAL_PUBLISHED_OK
- NA0579_NO_PUBLIC_READINESS_CLAIM_OK
- NA0579_NO_PRODUCTION_READINESS_CLAIM_OK
- NA0579_ONE_READY_INVARIANT_OK

## Classification Proof

- NA-0578 gating defect: `QSL_SERVER_NA0578_GATING_DEFECT_INSPECTION_STATE_NOT_PERSISTED`.
- Local dry-run assertion: pass.
- SSH readiness: `SSH_QSL_SERVER_PROOF_COMPLETION_FOLLOWUP_READY`.
- Remote inspection: `QSL_SERVER_PROOF_COMPLETION_CORRECTED_START_SAFE`.
- Corrected bounded start: `QSL_SERVER_PROOF_COMPLETION_BOUNDED_START_TEMP_SMOKE_STARTED`.
- Corrected postcheck: `QSL_SERVER_PROOF_COMPLETION_POSTCHECK_TEMP_LOOPBACK_ROUTE_SHAPE_PASS`.
- Cleanup: `QSL_SERVER_PROOF_COMPLETION_CLEANUP_DONE`.
- Result: `QSL_SERVER_PROOF_COMPLETION_TEMP_LOOPBACK_ROUTE_SHAPE_PASS_EXPECTED_BIND_REQUIRED`.
- Selected successor: `NA-0580 -- QSL Remote qsl-server Expected Bind / Endpoint Alignment Harness`.

## Boundary Proof

The lane publishes only safe classes. It does not publish endpoint values, private port values, route-token/capability values, bearer values, Authorization values, private topology, process identity, command lines, payloads, response bodies, authorized_keys content, key material, raw private logs, or secret values.

No sudo, systemctl, service, firewall, Tailscale, account, shell, authorized_keys mutation, qsc send/receive, workflow dispatch/rerun, qsl-attachments work, qsl-server source mutation, qsl-protocol source/script/workflow/dependency mutation, public-site mutation, Cloudflare mutation, public-readiness claim, production-readiness claim, vulnerability-free claim, bug-free claim, perfect-build claim, or perfect-crypto claim is introduced.
