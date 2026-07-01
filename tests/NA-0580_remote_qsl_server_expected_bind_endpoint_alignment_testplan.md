Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-07-01

# NA-0580 Remote qsl-server Expected Bind / Endpoint Alignment Testplan

This testplan records the marker set for the NA-0580 expected-bind / endpoint alignment harness.

## Required Markers

- NA0580_D1149_TEMP_ROUTE_SHAPE_PROOF_CONSUMED_OK
- NA0580_D1150_CLOSEOUT_CONSUMED_OK
- NA0580_FRESH_QWORK_PROOF_OK
- NA0580_CURRENT_MAIN_CHECKS_CLASSIFIED_OK
- NA0580_AUTHORITY_MODEL_APPLIED_OK
- NA0580_AUTOMATIC_FAILURE_CAUSE_POLICY_APPLIED_OK
- NA0580_QSL_SERVER_ROUTE_REVIEW_OK
- NA0580_QSC_RELAY_EXPECTATION_REVIEW_OK
- NA0580_GITHUB_METADATA_NO_SECRET_VALUES_OK
- NA0580_REMOTE_SCRIPTS_STATIC_REVIEW_OK
- NA0580_SSH_READINESS_CLASSIFIED_OK
- NA0580_REMOTE_ALIGNMENT_INSPECTION_CLASSIFIED_OK
- NA0580_EXPECTED_BIND_START_EXECUTED_OR_SKIPPED_OK
- NA0580_EXPECTED_BIND_POSTCHECK_EXECUTED_OR_SKIPPED_OK
- NA0580_CLEANUP_EXECUTED_OR_SKIPPED_OK
- NA0580_FAILURE_CAUSE_INVESTIGATION_EXECUTED_OR_SKIPPED_OK
- NA0580_PRIVATE_MATERIAL_SCAN_OK
- NA0580_RESULT_CLASSIFICATION_SELECTED_OK
- NA0580_SUCCESSOR_SELECTED_OK
- NA0580_NO_ENDPOINT_VALUE_PUBLISHED_OK
- NA0580_NO_PRIVATE_PORT_VALUE_PUBLISHED_OK
- NA0580_NO_TOKEN_OR_AUTHORIZATION_PUBLISHED_OK
- NA0580_NO_PROCESS_IDENTITY_PUBLISHED_OK
- NA0580_NO_COMMAND_LINE_PUBLISHED_OK
- NA0580_NO_PAYLOAD_BODY_PUBLISHED_OK
- NA0580_NO_SUDO_SYSTEMCTL_TAILSCALE_OK
- NA0580_NO_ACCOUNT_AUTHORIZED_KEYS_MUTATION_OK
- NA0580_NO_WRITES_OUTSIDE_QSLCODEX_TEST_WORKSPACE_OK
- NA0580_NO_QSC_SEND_RECEIVE_OK
- NA0580_NO_WORKFLOW_DISPATCH_OK
- NA0580_NO_QSL_ATTACHMENTS_OK
- NA0580_NO_QSL_PROTOCOL_SOURCE_MUTATION_OK
- NA0580_NO_QSL_SERVER_SOURCE_MUTATION_OK
- NA0580_NO_PRIVATE_MATERIAL_PUBLISHED_OK
- NA0580_NO_PUBLIC_READINESS_CLAIM_OK
- NA0580_NO_PRODUCTION_READINESS_CLAIM_OK
- NA0580_ONE_READY_INVARIANT_OK

## Classification Proof

- SSH readiness: `SSH_QSL_SERVER_EXPECTED_BIND_ALIGNMENT_READY`.
- Remote inspection: `QSL_SERVER_EXPECTED_BIND_ALIGNMENT_ENDPOINT_VALUE_UNAVAILABLE`.
- Expected-bind bounded start: `QSL_SERVER_EXPECTED_BIND_BOUNDED_START_SKIPPED_NOT_SAFE`.
- Expected-bind postcheck: `QSL_SERVER_EXPECTED_BIND_POSTCHECK_ENDPOINT_ALIGNMENT_UNPROVEN`.
- Cleanup: `QSL_SERVER_EXPECTED_BIND_CLEANUP_NOT_NEEDED`.
- Failure-cause investigation: `QSL_SERVER_EXPECTED_BIND_FAILURE_ENDPOINT_VALUE_UNAVAILABLE`.
- Result: `QSL_SERVER_EXPECTED_BIND_ENDPOINT_VALUE_UNAVAILABLE`.
- Selected successor: `NA-0581 -- QSL Remote qsl-server Expected Bind Failure Remediation Harness`.

## Boundary Proof

The lane publishes only safe classes. It does not publish endpoint values, private port values, route-token/capability values, bearer values, Authorization values, private topology, process identity, command lines, payloads, response bodies, authorized_keys content, key material, raw private logs, or secret values.

No sudo, systemctl, service, firewall, Tailscale, account, shell, authorized_keys mutation, qsc send/receive, workflow dispatch/rerun, qsl-attachments work, qsl-server source mutation, qsl-protocol source/script/workflow/dependency mutation, public-site mutation, Cloudflare mutation, public-readiness claim, production-readiness claim, vulnerability-free claim, bug-free claim, perfect-build claim, or perfect-crypto claim is introduced.

## Validation

- Exact five-path implementation scope guard:
  - `docs/governance/evidence/NA-0580_remote_qsl_server_expected_bind_endpoint_alignment_harness.md`
  - `tests/NA-0580_remote_qsl_server_expected_bind_endpoint_alignment_testplan.md`
  - `DECISIONS.md`
  - `TRACEABILITY.md`
  - `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- Queue/decision proof:
  - READY_COUNT 1.
  - READY NA-0580.
  - D-1151 exists once after patch.
  - D-1152 is absent.
  - Duplicate decision count zero.
- Marker proof for every required marker above.
- Link-check.
- Added-line/new-file private-material scan.
- Remote-output private-material scan proof.
- Secret-value/prohibited-material scan.
- Overclaim scan.
- Docs/governance-only classifier.
- PR body preflight and goal-lint.
- Root and nested qsc fuzz cargo audits.
- Cargo metadata/fmt checks.
- qsc-adversarial shell syntax checks.
