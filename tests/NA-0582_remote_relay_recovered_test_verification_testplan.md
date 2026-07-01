Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-07-01

# NA-0582 Remote Relay Recovered Test Verification Testplan

This testplan records the marker set for the NA-0582 recovered remote relay
verification harness.

## Required Markers

- NA0582_D1153_RECOVERED_QSL_SERVER_CONSUMED_OK
- NA0582_D1154_CLOSEOUT_CONSUMED_OK
- NA0582_FRESH_QWORK_PROOF_OK
- NA0582_CURRENT_MAIN_CHECKS_CLASSIFIED_OK
- NA0582_AUTHORITY_MODEL_APPLIED_OK
- NA0582_QSC_RELAY_EXPECTATION_REVIEW_OK
- NA0582_REMOTE_POSTCHECK_CLASSIFIED_OK
- NA0582_WORKFLOW_METADATA_REVIEW_OK
- NA0582_WORKFLOW_ACTION_EXECUTED_OR_SKIPPED_OK
- NA0582_REMOTE_HANDSHAKE_RESULT_CLASSIFIED_OK
- NA0582_REMOTE_RELAY_RESULT_CLASSIFIED_OK
- NA0582_WORKFLOW_LOG_PRIVATE_SCAN_OK
- NA0582_FAILURE_CAUSE_INVESTIGATION_EXECUTED_OR_SKIPPED_OK
- NA0582_PRIVATE_MATERIAL_SCAN_OK
- NA0582_RESULT_CLASSIFICATION_SELECTED_OK
- NA0582_SUCCESSOR_SELECTED_OK
- NA0582_NO_ENDPOINT_VALUE_PUBLISHED_OK
- NA0582_NO_PRIVATE_PORT_VALUE_PUBLISHED_OK
- NA0582_NO_TOKEN_OR_AUTHORIZATION_PUBLISHED_OK
- NA0582_NO_PROCESS_IDENTITY_PUBLISHED_OK
- NA0582_NO_COMMAND_LINE_PUBLISHED_OK
- NA0582_NO_PAYLOAD_BODY_PUBLISHED_OK
- NA0582_NO_REMOTE_MUTATION_OK
- NA0582_NO_QSL_SERVER_START_STOP_CLEANUP_OK
- NA0582_NO_MANUAL_QSC_SEND_RECEIVE_OK
- NA0582_NO_WORKFLOW_FILE_MUTATION_OK
- NA0582_NO_QSL_ATTACHMENTS_OK
- NA0582_NO_QSL_PROTOCOL_SOURCE_MUTATION_OK
- NA0582_NO_QSL_SERVER_SOURCE_MUTATION_OK
- NA0582_NO_PRIVATE_MATERIAL_PUBLISHED_OK
- NA0582_NO_PUBLIC_READINESS_CLAIM_OK
- NA0582_NO_PRODUCTION_READINESS_CLAIM_OK
- NA0582_ONE_READY_INVARIANT_OK

## Classification Proof

- SSH readiness: `SSH_REMOTE_RELAY_RECOVERED_VERIFY_READY`.
- Recovered qsl-server postcheck:
  `REMOTE_RELAY_RECOVERED_QSL_SERVER_POSTCHECK_READY`.
- remote-handshake workflow action: dispatch on `main`.
- remote-relay workflow action: dispatch on `main`.
- remote-handshake run `28498817017`: failure.
- remote-relay run `28498817988`: failure.
- Workflow log scan: pass for publishable summaries; raw logs quarantined.
- Failure-cause investigation:
  `REMOTE_RELAY_VERIFICATION_FAILURE_SERVICE_UNREACHABLE_AFTER_LOCAL_READY`.
- Result:
  `REMOTE_RELAY_RECOVERED_VERIFICATION_FAIL_SERVICE_UNREACHABLE_AFTER_LOCAL_READY`.
- Selected successor:
  `NA-0583 -- QSL Remote Relay Service Reachability After Local Ready Triage Harness`.

## Boundary Proof

The lane publishes only safe classes, run IDs, workflow names, check names, and
redacted summaries. It does not publish endpoint values, private port values,
route-token/capability values, bearer values, Authorization values, private
topology, process identity, command lines, payloads, response bodies,
authorized_keys content, public key material, private key material, raw private
logs, or secret values.

No remote mutation, qsl-server start, qsl-server stop, qsl-server cleanup, qsc
send/receive, workflow file mutation, qsl-attachments work, qsl-protocol source
mutation, qsl-server source mutation, public-readiness claim, production-
readiness claim, vulnerability-free claim, bug-free claim, perfect-build claim,
or perfect-crypto claim is introduced.

## Validation

- Exact five-path implementation scope guard:
  - `docs/governance/evidence/NA-0582_remote_relay_recovered_test_verification_harness.md`
  - `tests/NA-0582_remote_relay_recovered_test_verification_testplan.md`
  - `DECISIONS.md`
  - `TRACEABILITY.md`
  - `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- Queue/decision proof:
  - READY_COUNT 1.
  - READY NA-0582.
  - D-1155 exists once after patch.
  - D-1156 is absent.
  - Duplicate decision count zero.
- Marker proof for every required marker above.
- Link-check.
- Added-line/new-file private-material scan.
- Remote-output and workflow-log private-material scan proof.
- Secret-value/prohibited-material scan.
- Overclaim scan.
- Docs/governance-only classifier.
- PR body preflight and goal-lint.
- Root and nested qsc fuzz cargo audits.
- Cargo metadata/fmt checks.
- qsc-adversarial shell syntax checks.
