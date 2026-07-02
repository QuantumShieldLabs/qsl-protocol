Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-07-02

# NA-0588 Local qsc / qsl-server Adversarial and Metadata Stress Testplan

## Required Markers

- NA0588_D1165_LOCAL_E2EE_PASS_CONSUMED_OK
- NA0588_D1166_CLOSEOUT_CONSUMED_OK
- NA0588_FRESH_QWORK_PROOF_OK
- NA0588_CURRENT_MAIN_CHECKS_CLASSIFIED_OK
- NA0588_QSL_SERVER_SOURCE_REVIEW_OK
- NA0588_QSL_SERVER_VALIDATION_OK
- NA0588_QSC_COMMAND_DISCOVERY_OK
- NA0588_STRESS_HARNESS_DESIGNED_OK
- NA0588_BASELINE_REVALIDATION_OK
- NA0588_REPETITION_STRESS_CLASSIFIED_OK
- NA0588_MULTI_MESSAGE_EMPTY_QUEUE_CLASSIFIED_OK
- NA0588_ROUTE_ISOLATION_CLASSIFIED_OK
- NA0588_AUTH_NEGATIVES_CLASSIFIED_OK
- NA0588_MALFORMED_INPUTS_EXECUTED_OR_DEFERRED_OK
- NA0588_RESTART_BOUNDARY_EXECUTED_OR_DEFERRED_OK
- NA0588_CONCURRENCY_EXECUTED_OR_DEFERRED_OK
- NA0588_METADATA_REVIEW_OK
- NA0588_DIAGNOSTICS_REVIEW_OK
- NA0588_ISSUE_INVESTIGATION_EXECUTED_OR_SKIPPED_OK
- NA0588_SAFE_FIX_APPLIED_OR_SKIPPED_OK
- NA0588_PRIVATE_MATERIAL_SCAN_OK
- NA0588_NO_ENDPOINT_VALUE_PUBLISHED_OK
- NA0588_NO_PRIVATE_PORT_VALUE_PUBLISHED_OK
- NA0588_NO_TOKEN_OR_AUTHORIZATION_PUBLISHED_OK
- NA0588_NO_PAYLOAD_BODY_PLAINTEXT_PUBLISHED_OK
- NA0588_NO_KEY_MATERIAL_PUBLISHED_OK
- NA0588_NO_QSL_ATTACHMENTS_OK
- NA0588_NO_REMOTE_TAILSCALE_WORKFLOW_ACTION_OK
- NA0588_NO_PUBLIC_READINESS_CLAIM_OK
- NA0588_NO_PRODUCTION_READINESS_CLAIM_OK
- NA0588_RESULT_CLASSIFICATION_SELECTED_OK
- NA0588_SUCCESSOR_SELECTED_OK
- NA0588_ONE_READY_INVARIANT_OK

## Classification Proof

- D-1165 consumed: yes.
- D-1166 consumed: yes.
- qwork proof: fresh, lane NA-0588, clean worktree, READY_COUNT 1,
  READY NA-0588, D-1167 absent before patch.
- Current main checks: classified green or satisfied before implementation.
- qsl-server source review: clean checkout at `6bf61d439fa2`.
- qsl-server validation: metadata, audit, fmt, test, and build passed.
- qsc command discovery: `QSC_COMMAND_DISCOVERY_OK`.
- Stress harness design: loopback-only, proof-root-only, no public bind, no
  remote action, no workflow action, and explicit process cleanup.
- Baseline revalidation: `BASELINE_LOCAL_QSC_E2EE_PASS`.
- Repetition stress: `REPETITION_STRESS_PASS`.
- Multi-message/empty queue: `MULTI_MESSAGE_ORDERING_PASS` and
  `EMPTY_PULL_AFTER_DRAIN_PASS`.
- Route isolation: `ROUTE_ISOLATION_PASS`.
- Auth negatives: `AUTH_NEGATIVES_FAIL_CLOSED_PASS`.
- Malformed/bounded input tests: `MALFORMED_INPUTS_FAIL_CLOSED_PASS`.
- Restart boundary: `RESTART_BOUNDARY_VOLATILE_QUEUE_EXPECTED`.
- Concurrency/rapid operations: `CONCURRENCY_RAPID_OPS_PASS`.
- Metadata review: `METADATA_MINIMIZATION_REVIEW_PASS`.
- Diagnostics review: `DIAGNOSTICS_ACTIONABLE_PASS`.
- Issue investigation: skipped for product code because no failing or ambiguous
  stress case remained; proof-root-only harness repair evidence was recorded.
- Safe fix: skipped for project-owned source/test paths because no qsc or
  qsl-server source bug was selected.
- Result classification:
  `LOCAL_QSC_QSL_SERVER_E2EE_ADVERSARIAL_METADATA_STRESS_PASS`.
- Selected successor:
  `NA-0589 -- QSL Local qsl-attachments Integration Readiness Harness`.

## Boundary Proof

No endpoint value beyond loopback class, private port value, token value,
Authorization value, route-token/capability value, bearer value, payload,
response body, plaintext message content, envelope body, process identity,
private topology, key material, secret environment value, raw qsl-server log,
or raw qsc output is published.

No qsc source/test/helper mutation, qsl-server source/test mutation,
qsl-attachments command/runtime/integration/mutation, dependency/lockfile
change, workflow file mutation, qwork/qstart/qresume execution, remote action,
Tailscale action, workflow dispatch/rerun, public-site mutation, or Cloudflare
mutation is introduced.

No public readiness claim is introduced. No production readiness claim is
introduced. No vulnerability-free claim is introduced. No bug-free claim is
introduced. No perfect-build claim is introduced. No perfect-crypto claim is
introduced. No crypto-complete claim is introduced.

## Validation

Required validation covers diff/scope/queue/marker proof, link check,
private-material scan, stress-artifact scan proof, secret/prohibited-material
scan, overclaim scan, docs/governance/source classifier, PR body preflight,
goal-lint where available, root cargo audit, nested qsc fuzz cargo audit,
locked cargo metadata, cargo fmt, qsc adversarial shell syntax checks, focused
qsc relay tests, and qsl-server metadata/audit/fmt/test/build validation.
