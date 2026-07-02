Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-07-01

# NA-0587 Local qsc / qsl-server Relay Integration Pivot Testplan

## Required Markers

- NA0587_D1163_DNS_TIMEOUT_CONSUMED_OK
- NA0587_D1164_CLOSEOUT_CONSUMED_OK
- NA0587_LOCAL_INTEGRATION_PIVOT_RECORDED_OK
- NA0587_FRESH_QWORK_PROOF_OK
- NA0587_QSL_SERVER_SOURCE_REVIEW_OK
- NA0587_QSL_SERVER_AUDIT_BUILD_TEST_OK_OR_CLASSIFIED
- NA0587_QSL_SERVER_LOCAL_ROUTE_SHAPE_CLASSIFIED_OK
- NA0587_QSC_COMMAND_DISCOVERY_OK
- NA0587_QSC_LOCAL_RELAY_INTEGRATION_CLASSIFIED_OK
- NA0587_QSC_E2EE_EXECUTED_OR_DEFERRED_OK
- NA0587_SELECTED_NEGATIVES_EXECUTED_OR_DEFERRED_OK
- NA0587_METADATA_REVIEW_OK
- NA0587_ISSUE_INVESTIGATION_EXECUTED_OR_SKIPPED_OK
- NA0587_SAFE_FIX_APPLIED_OR_SKIPPED_OK
- NA0587_PRIVATE_MATERIAL_SCAN_OK
- NA0587_NO_REMOTE_ACTION_OK
- NA0587_NO_TAILSCALE_OK
- NA0587_NO_WORKFLOW_DISPATCH_OK
- NA0587_NO_QSL_ATTACHMENTS_OK
- NA0587_NO_PUBLIC_READINESS_CLAIM_OK
- NA0587_NO_PRODUCTION_READINESS_CLAIM_OK
- NA0587_RESULT_CLASSIFICATION_SELECTED_OK
- NA0587_SUCCESSOR_SELECTED_OK
- NA0587_ONE_READY_INVARIANT_OK

## Classification Proof

- D-1163 consumed: yes.
- D-1164 consumed: yes.
- Local integration pivot recorded:
  `LOCAL_CLIENT_RELAY_INTEGRATION_PIVOT_SELECTED`.
- qwork proof: fresh, lane NA-0587, clean worktree, READY_COUNT 1,
  READY NA-0587, D-1165 absent before patch.
- qsl-server source review: clean checkout at `6bf61d439fa2`.
- qsl-server validation: metadata, audit, fmt, test, and build passed.
- qsl-server local route shape: `LOCAL_QSL_SERVER_ROUTE_SHAPE_PASS`.
- qsl-server auth fail-closed: `LOCAL_QSL_SERVER_AUTH_FAIL_CLOSED_PASS`.
- qsc command discovery: `QSC_COMMAND_DISCOVERY_OK`.
- qsc local relay integration: `LOCAL_QSC_RELAY_PUSH_PULL_PASS`.
- qsc E2EE over local relay: `LOCAL_QSC_E2EE_OVER_RELAY_PASS`.
- qsc auth fail-closed: `LOCAL_QSC_RELAY_AUTH_FAIL_CLOSED_PASS`.
- Selected negatives: wrong bearer, empty pull after drain, and wrong-route
  pull all passed.
- Issue investigation: skipped because no failing integration issue remained.
- Safe fix: skipped because no source/test fix was needed.
- Result classification: `LOCAL_CLIENT_RELAY_E2EE_INTEGRATION_PASS`.
- Selected successor: Option B, local adversarial and metadata stress harness.

## Boundary Proof

No endpoint value beyond loopback class, private port value, token value,
Authorization value, route-token/capability value, payload, response body,
plaintext message content, process identity, private topology, key material,
secret environment value, raw qsl-server log, or raw qsc output is published.

No qsl-server source mutation, qsl-protocol qsc source/test mutation,
qsl-attachments work, dependency/lockfile change, workflow file mutation,
qwork/qstart/qresume execution, remote action, Tailscale action, workflow
dispatch/rerun, public-site mutation, or Cloudflare mutation is introduced.

No public readiness claim is introduced. No production readiness claim is
introduced. No vulnerability-free claim is introduced. No bug-free claim is
introduced. No perfect-build claim is introduced. No perfect-crypto claim is
introduced.

## Validation

Required validation covers diff/scope/queue/marker proof, link check,
private-material scan, overclaim scan, PR body preflight, goal-lint, root cargo
audit, nested qsc fuzz cargo audit, locked cargo metadata, cargo fmt, qsc
focused relay tests, qsl-server audit/build/test/fmt, and shell syntax checks
when scripts are touched.

