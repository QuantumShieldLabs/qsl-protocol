Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-07-01

# NA-0585 Remote Relay Diagnostic Surface and Codex Issue Authority Testplan

## Required Markers

- NA0585_D1159_QSC_TIMEOUT_PHASE_CONSUMED_OK
- NA0585_D1160_CLOSEOUT_CONSUMED_OK
- NA0585_FRESH_QWORK_PROOF_OK
- NA0585_CURRENT_MAIN_CHECKS_CLASSIFIED_OK
- NA0585_CODEX_ISSUE_AUTHORITY_HARDENED_OK
- NA0585_SOURCE_ANALYSIS_AUTHORITY_HARDENED_OK
- NA0585_SAFE_FIX_AUTHORITY_HARDENED_OK
- NA0585_QSC_RELAY_SOURCE_REVIEW_OK
- NA0585_QSC_TIMEOUT_PLUMBING_REVIEW_OK
- NA0585_WORKFLOW_HARNESS_REVIEW_OK
- NA0585_QSL_SERVER_ROUTE_REVIEW_OK
- NA0585_DIAGNOSTIC_MUTATION_PATHS_SELECTED_OK
- NA0585_DIAGNOSTIC_SURFACE_IMPLEMENTED_OR_DEFERRED_OK
- NA0585_REDATION_TESTS_PASS_OK
- NA0585_PRIVATE_MATERIAL_SCAN_OK
- NA0585_NO_CRYPTO_PROTOCOL_SEMANTIC_CHANGE_OK
- NA0585_NO_ENDPOINT_VALUE_PUBLISHED_OK
- NA0585_NO_PRIVATE_PORT_VALUE_PUBLISHED_OK
- NA0585_NO_TOKEN_OR_AUTHORIZATION_PUBLISHED_OK
- NA0585_NO_PAYLOAD_BODY_PUBLISHED_OK
- NA0585_NO_PROCESS_TOPOLOGY_KEY_MATERIAL_PUBLISHED_OK
- NA0585_NO_QSL_SERVER_SOURCE_MUTATION_OK
- NA0585_NO_QSL_ATTACHMENTS_OK
- NA0585_NO_DEPENDENCY_LOCKFILE_CHANGE_OK
- NA0585_NO_QWORK_EXECUTION_OK
- NA0585_NO_REMOTE_MUTATION_OK
- NA0585_NO_PUBLIC_READINESS_CLAIM_OK
- NA0585_NO_PRODUCTION_READINESS_CLAIM_OK
- NA0585_RESULT_CLASSIFICATION_SELECTED_OK
- NA0585_SUCCESSOR_SELECTED_OK
- NA0585_ONE_READY_INVARIANT_OK

## Classification Proof

- D-1159 consumed: yes.
- D-1160 consumed: yes.
- Fresh qwork proof from `2026-07-01T16:46:24Z` verified: yes.
- Current main checks classified: public-safety success, advisories success,
  suite2-vectors success, no failed or pending visible checks.
- Codex issue authority hardened in START_HERE, AGENTS, and bounded authority
  runbook: yes.
- qsc relay source review: completed.
- qsc timeout/error plumbing review: completed.
- workflow/harness review: completed.
- qsl-server route expectation review: completed read-only.
- Selected diagnostic mutation paths recorded before source/helper mutation.
- Diagnostic surface result:
  `REMOTE_RELAY_DIAGNOSTIC_SURFACE_SAFE_FIX_IMPLEMENTED`.
- Selected successor: NA-0586 diagnostic verification and timeout phase triage.

## Validation

- Focused qsc formatting passed.
- Touched remote helper bash syntax checks passed.
- Focused qsc relay diagnostic unit and integration tests passed.
- Private-material scan passed.
- No-semantics-change review passed.

## Boundary Proof

No endpoint values, private port values, token values, Authorization values,
route-token/capability values, payloads, response bodies, process identities,
private topology, authorized_keys content, public key material, private key
material, or secret values are published.

No qsl-server source mutation, qsl-attachments work, dependency/lockfile change,
workflow file mutation, qwork/qstart/qresume execution, remote mutation, public
readiness claim, production readiness claim, vulnerability-free claim, bug-free
claim, perfect-build claim, or perfect-crypto claim is introduced.
