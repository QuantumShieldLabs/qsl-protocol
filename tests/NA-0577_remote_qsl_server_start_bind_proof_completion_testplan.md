Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-30

# NA-0577 Remote qsl-server Start / Bind Proof Completion Testplan

This testplan records the marker set for the NA-0577 qsl-server start/bind proof completion harness.

## Required Markers

- NA0577_D1142_AUTHORITY_MODEL_CONSUMED_OK
- NA0577_D1143_WAIT_WORK_POLICY_CONSUMED_OK
- NA0577_D1144_CLOSEOUT_CONSUMED_OK
- NA0577_FRESH_QWORK_PROOF_OK
- NA0577_CURRENT_MAIN_CHECKS_CLASSIFIED_OK
- NA0577_REMOTE_SCRIPTS_STATIC_REVIEW_OK
- NA0577_SSH_READINESS_CLASSIFIED_OK
- NA0577_REMOTE_INSPECTION_CLASSIFIED_OK
- NA0577_BOUNDED_START_EXECUTED_OR_SKIPPED_OK
- NA0577_BOUNDED_POSTCHECK_EXECUTED_OR_SKIPPED_OK
- NA0577_CLEANUP_EXECUTED_OR_SKIPPED_OK
- NA0577_PRIVATE_MATERIAL_SCAN_OK
- NA0577_RESULT_CLASSIFICATION_SELECTED_OK
- NA0577_SUCCESSOR_SELECTED_OK
- NA0577_NO_ENDPOINT_VALUE_PUBLISHED_OK
- NA0577_NO_PRIVATE_PORT_VALUE_PUBLISHED_OK
- NA0577_NO_TOKEN_OR_AUTHORIZATION_PUBLISHED_OK
- NA0577_NO_PROCESS_IDENTITY_PUBLISHED_OK
- NA0577_NO_PAYLOAD_BODY_PUBLISHED_OK
- NA0577_NO_SUDO_SYSTEMCTL_TAILSCALE_OK
- NA0577_NO_ACCOUNT_AUTHORIZED_KEYS_MUTATION_OK
- NA0577_NO_WRITES_OUTSIDE_QSLCODEX_TEST_WORKSPACE_OK
- NA0577_NO_QSC_SEND_RECEIVE_OK
- NA0577_NO_WORKFLOW_DISPATCH_OK
- NA0577_NO_QSL_ATTACHMENTS_OK
- NA0577_NO_QSL_PROTOCOL_SOURCE_MUTATION_OK
- NA0577_NO_PRIVATE_MATERIAL_PUBLISHED_OK
- NA0577_NO_PUBLIC_READINESS_CLAIM_OK
- NA0577_NO_PRODUCTION_READINESS_CLAIM_OK
- NA0577_ONE_READY_INVARIANT_OK

## Classification Proof

- SSH readiness: `SSH_QSL_SERVER_START_BIND_COMPLETION_READY`.
- Remote inspection: `QSL_SERVER_START_BIND_COMPLETION_TEMP_LOOPBACK_SMOKE_SAFE`.
- Bounded start: `QSL_SERVER_BOUNDED_START_FAILED`.
- Bounded postcheck: `QSL_SERVER_BOUNDED_POSTCHECK_SKIPPED`.
- Cleanup: `QSL_SERVER_BOUNDED_CLEANUP_DONE`.
- Result: `QSL_SERVER_START_BIND_INSUFFICIENT_PROOF`.
- Selected successor: `NA-0578 -- QSL Remote qsl-server Start / Bind Proof Completion Follow-Up Harness`.

## Boundary Proof

The lane publishes only safe classes. It does not publish endpoint values, private port values, route-token/capability values, bearer values, Authorization headers, private topology, process identity, payloads, response bodies, authorized_keys content, key material, raw private logs, or secret values.

No sudo, systemctl, service, firewall, Tailscale, account, shell, authorized_keys mutation, qsc send/receive, workflow dispatch/rerun, qsl-attachments work, qsl-server source mutation, qsl-protocol source/script/workflow/dependency mutation, public-site mutation, Cloudflare mutation, public-readiness claim, production-readiness claim, vulnerability-free claim, bug-free claim, perfect-build claim, or perfect-crypto claim is introduced.
