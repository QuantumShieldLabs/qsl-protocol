Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-07-01

# NA-0583 Remote Relay Service Reachability After Local Ready Triage Testplan

This testplan records the marker set for the NA-0583 artifact-backed remote
relay triage harness.

## Required Markers

- NA0583_D1155_RECOVERED_VERIFICATION_CONSUMED_OK
- NA0583_D1156_CLOSEOUT_CONSUMED_OK
- NA0583_D511_CLASSIFICATION_CORRECTED_OK
- NA0583_FRESH_QWORK_PROOF_OK
- NA0583_CURRENT_MAIN_CHECKS_CLASSIFIED_OK
- NA0583_WORKFLOW_RUN_METADATA_REVIEW_OK
- NA0583_WORKFLOW_ARTIFACT_INSPECTION_OK
- NA0583_WORKFLOW_LOG_REREVIEW_OK
- NA0583_MASKED_AUTH_HEADER_CLASSIFIED_OK
- NA0583_OPTIONAL_REMOTE_POSTCHECK_EXECUTED_OR_SKIPPED_OK
- NA0583_OPTIONAL_RERUN_EXECUTED_OR_SKIPPED_OK
- NA0583_FAILURE_CAUSE_CLASSIFIED_OK
- NA0583_PRIVATE_MATERIAL_SCAN_OK
- NA0583_RESULT_CLASSIFICATION_SELECTED_OK
- NA0583_SUCCESSOR_SELECTED_OK
- NA0583_NO_ENDPOINT_VALUE_PUBLISHED_OK
- NA0583_NO_PRIVATE_PORT_VALUE_PUBLISHED_OK
- NA0583_NO_TOKEN_OR_AUTHORIZATION_PUBLISHED_OK
- NA0583_NO_UNMASKED_AUTHORIZATION_PUBLISHED_OK
- NA0583_NO_PROCESS_IDENTITY_PUBLISHED_OK
- NA0583_NO_COMMAND_LINE_PUBLISHED_OK
- NA0583_NO_PAYLOAD_BODY_PUBLISHED_OK
- NA0583_NO_REMOTE_MUTATION_OK
- NA0583_NO_QSL_SERVER_START_STOP_CLEANUP_OK
- NA0583_NO_MANUAL_QSC_SEND_RECEIVE_OK
- NA0583_NO_WORKFLOW_FILE_MUTATION_OK
- NA0583_NO_QSL_ATTACHMENTS_OK
- NA0583_NO_QSL_PROTOCOL_SOURCE_MUTATION_OK
- NA0583_NO_QSL_SERVER_SOURCE_MUTATION_OK
- NA0583_NO_PRIVATE_MATERIAL_PUBLISHED_OK
- NA0583_NO_PUBLIC_READINESS_CLAIM_OK
- NA0583_NO_PRODUCTION_READINESS_CLAIM_OK
- NA0583_ONE_READY_INVARIANT_OK

## Classification Proof

- D-1155 consumed: yes.
- D-1156 consumed: yes.
- D511 classification correction:
  `LOCAL_READY_REMOTE_WORKFLOW_FAILED_ARTIFACTS_REQUIRED`.
- qwork proof: fresh and verified before fetch/artifact/log retrieval.
- Current main checks: public-safety success, advisories success,
  suite2-vectors success, and no failed required checks.
- remote-handshake run `28498817017`: identity confirmed, failure,
  artifacts present and inspected.
- remote-relay run `28498817988`: identity confirmed, failure, artifacts
  present and inspected.
- Artifact failure cause:
  `REMOTE_RELAY_ARTIFACT_BACKED_FAILURE_SERVICE_UNREACHABLE_AFTER_LOCAL_READY`.
- Result:
  `REMOTE_RELAY_TRIAGE_ARTIFACT_BACKED_SERVICE_UNREACHABLE_AFTER_LOCAL_READY`.
- Selected successor:
  `NA-0584 -- QSL Remote Relay Runner / Service Reachability Remediation Harness`.

## Boundary Proof

The lane publishes only safe classes, run IDs, workflow names, check/job names,
and redacted summaries. Raw workflow artifacts and logs remain proof-root-only.
It does not publish endpoint values, private port values, route-token/capability
values, bearer values, unmasked Authorization values, private topology, process
identity, command lines, payloads, response bodies, authorized_keys content,
public key material, private key material, raw private logs, or secret values.

No remote mutation, SSH command, qsl-server start, qsl-server stop,
qsl-server cleanup, qsc send/receive, workflow file mutation, workflow rerun,
qsl-attachments work, qsl-protocol source mutation, qsl-server source mutation,
public-readiness claim, production-readiness claim, vulnerability-free claim,
bug-free claim, perfect-build claim, or perfect-crypto claim is introduced.

## Validation

- Exact five-path implementation scope guard:
  - `docs/governance/evidence/NA-0583_remote_relay_service_reachability_after_local_ready_triage_harness.md`
  - `tests/NA-0583_remote_relay_service_reachability_after_local_ready_triage_testplan.md`
  - `DECISIONS.md`
  - `TRACEABILITY.md`
  - `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- Queue/decision proof:
  - READY_COUNT 1.
  - READY NA-0583.
  - D-1157 exists once after patch.
  - D-1158 is absent.
  - Duplicate decision count zero.
- Marker proof for every required marker above.
- Link-check.
- Added-line/new-file private-material scan.
- Workflow artifact/log private-material scan proof.
- Secret-value/prohibited-material scan.
- Overclaim scan.
- Docs/governance-only classifier.
- PR body preflight and goal-lint.
- Root cargo audit.
- Nested qsc fuzz cargo audit.
- `cargo metadata --locked --format-version=1`.
- `cargo fmt --check`.
- `sh -n scripts/ci/qsc_adversarial.sh`.
- `bash -n scripts/ci/qsc_adversarial.sh`.
