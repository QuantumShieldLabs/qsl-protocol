Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-07-01

# NA-0584 Remote Relay Runner / Service Reachability Remediation Testplan

## Required Markers

- NA0584_D1157_ARTIFACT_TRIAGE_CONSUMED_OK
- NA0584_D1158_CLOSEOUT_CONSUMED_OK
- NA0584_FRESH_QWORK_PROOF_OK
- NA0584_CURRENT_MAIN_CHECKS_CLASSIFIED_OK
- NA0584_AUTHORITY_MODEL_APPLIED_OK
- NA0584_QSC_TIMEOUT_SEMANTICS_REVIEW_OK
- NA0584_QSL_SERVER_ROUTE_REVIEW_OK
- NA0584_REMOTE_SCRIPTS_STATIC_REVIEW_OK
- NA0584_SSH_READINESS_CLASSIFIED_OK
- NA0584_LOCAL_QSL_SERVER_PRECHECK_CLASSIFIED_OK
- NA0584_WORKFLOW_DIAGNOSTIC_ACTION_CLASSIFIED_OK
- NA0584_WORKFLOW_RESULTS_CLASSIFIED_OK
- NA0584_WORKFLOW_ARTIFACTS_SCANNED_OK
- NA0584_WORKFLOW_LOGS_SCANNED_OK
- NA0584_REMOTE_POSTRUN_SNAPSHOT_CLASSIFIED_OK
- NA0584_FAILURE_CAUSE_CLASSIFIED_OK
- NA0584_PRIVATE_MATERIAL_SCAN_OK
- NA0584_RESULT_CLASSIFICATION_SELECTED_OK
- NA0584_SUCCESSOR_SELECTED_OK
- NA0584_NO_ENDPOINT_VALUE_PUBLISHED_OK
- NA0584_NO_PRIVATE_PORT_VALUE_PUBLISHED_OK
- NA0584_NO_TOKEN_OR_AUTHORIZATION_PUBLISHED_OK
- NA0584_NO_UNMASKED_AUTHORIZATION_PUBLISHED_OK
- NA0584_NO_PROCESS_IDENTITY_PUBLISHED_OK
- NA0584_NO_COMMAND_LINE_PUBLISHED_OK
- NA0584_NO_PAYLOAD_BODY_PUBLISHED_OK
- NA0584_NO_REMOTE_MUTATION_OK
- NA0584_NO_QSL_SERVER_START_STOP_CLEANUP_OK
- NA0584_NO_MANUAL_QSC_SEND_RECEIVE_OK
- NA0584_NO_WORKFLOW_FILE_MUTATION_OK
- NA0584_NO_QSL_ATTACHMENTS_OK
- NA0584_NO_QSL_PROTOCOL_SOURCE_MUTATION_OK
- NA0584_NO_QSL_SERVER_SOURCE_MUTATION_OK
- NA0584_NO_PRIVATE_MATERIAL_PUBLISHED_OK
- NA0584_NO_PUBLIC_READINESS_CLAIM_OK
- NA0584_NO_PRODUCTION_READINESS_CLAIM_OK
- NA0584_ONE_READY_INVARIANT_OK

## Classification Proof

- D-1157 consumed: yes.
- D-1158 consumed: yes.
- Fresh qwork proof verified before fetch/action/SSH/proof publication: yes.
- Current main checks classified: public-safety success, advisories success, no
  failed or pending required checks.
- qsc timeout semantics review: qsc generic timeout only; DNS/TCP/TLS/HTTP phase
  not separable by current diagnostics.
- qsl-server route review: `/v1/push`, `/v1/pull?max=N`,
  `X-QSL-Route-Token`, optional bearer, no payload/body publication.
- Local qsl-server precheck:
  `REMOTE_RELAY_REACHABILITY_PRECHECK_LOCAL_READY`.
- Diagnostic action:
  `DIAGNOSTIC_ACTION_PLAN_RERUN_FAILED_D512_EXACT`.
- remote-handshake result: failed rerun, reached relay push, qsc generic timeout.
- remote-relay result: failed rerun, reached relay push, qsc generic timeout.
- Postrun snapshot: local relay still ready; request-arrival delta unavailable.
- Failure cause:
  `REMOTE_RELAY_REACHABILITY_QSC_TIMEOUT_PHASE_CLASSIFIED`.
- Result:
  `REMOTE_RELAY_REACHABILITY_QSC_TIMEOUT_PHASE_ONLY`.
- Selected successor:
  `NA-0585 -- QSL Remote Relay Diagnostic Surface Improvement Harness`.

## Boundary Proof

The lane publishes only safe classes, run IDs, workflow names, job names, and
redacted summaries. Raw SSH output, workflow logs, and workflow artifacts remain
proof-root-only.

No endpoint values, private port values, route-token/capability values, bearer
values, unmasked Authorization values, private topology, process identity,
command lines, payloads, response bodies, authorized_keys content, public key
material, private key material, secret values, Cloudflare tokens, API tokens, or
private material are published.

No remote mutation, qsl-server start, qsl-server stop, qsl-server cleanup, qsc
manual send/receive, workflow file mutation, qsl-attachments work,
qsl-protocol source/script/workflow/dependency mutation, qsl-server source
mutation, public-readiness claim, production-readiness claim, vulnerability-free
claim, bug-free claim, perfect-build claim, or perfect-crypto claim is
introduced.

## Validation

- Exact five-path implementation scope guard:
  - `docs/governance/evidence/NA-0584_remote_relay_runner_service_reachability_remediation_harness.md`
  - `tests/NA-0584_remote_relay_runner_service_reachability_remediation_testplan.md`
  - `DECISIONS.md`
  - `TRACEABILITY.md`
  - `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- Queue/decision proof:
  - READY_COUNT 1.
  - READY NA-0584.
  - D-1159 exists once after patch.
  - D-1160 is absent.
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
