Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-07-01

# NA-0586 Remote Relay Diagnostic Verification and Timeout Phase Triage Testplan

## Required Markers

- NA0586_D1161_DIAGNOSTIC_SURFACE_CONSUMED_OK
- NA0586_D1162_CLOSEOUT_CONSUMED_OK
- NA0586_FRESH_QWORK_PROOF_OK
- NA0586_CURRENT_MAIN_CHECKS_CLASSIFIED_OK
- NA0586_AUTHORITY_MODEL_APPLIED_OK
- NA0586_SOURCE_DIAGNOSTIC_REVIEW_OK
- NA0586_WORKFLOW_DIAGNOSTIC_ACTION_EXECUTED_OR_SKIPPED_OK
- NA0586_WORKFLOW_RESULTS_CLASSIFIED_OK
- NA0586_WORKFLOW_ARTIFACTS_SCANNED_OK
- NA0586_WORKFLOW_LOGS_SCANNED_OK
- NA0586_DIAGNOSTIC_SURFACE_VERIFIED_OR_CLASSIFIED_OK
- NA0586_TIMEOUT_PHASE_CLASSIFIED_OK
- NA0586_FAILURE_CAUSE_INVESTIGATION_EXECUTED_OR_SKIPPED_OK
- NA0586_DIAGNOSTIC_FIX_APPLIED_OR_SKIPPED_OK
- NA0586_OPTIONAL_REMOTE_POSTCHECK_EXECUTED_OR_SKIPPED_OK
- NA0586_PRIVATE_MATERIAL_SCAN_OK
- NA0586_NO_CRYPTO_PROTOCOL_SEMANTIC_CHANGE_OK
- NA0586_NO_ENDPOINT_VALUE_PUBLISHED_OK
- NA0586_NO_PRIVATE_PORT_VALUE_PUBLISHED_OK
- NA0586_NO_TOKEN_OR_AUTHORIZATION_PUBLISHED_OK
- NA0586_NO_PAYLOAD_BODY_PUBLISHED_OK
- NA0586_NO_PROCESS_TOPOLOGY_KEY_MATERIAL_PUBLISHED_OK
- NA0586_NO_QSL_SERVER_SOURCE_MUTATION_OK
- NA0586_NO_QSL_ATTACHMENTS_OK
- NA0586_NO_DEPENDENCY_LOCKFILE_CHANGE_OK
- NA0586_NO_QWORK_EXECUTION_OK
- NA0586_NO_REMOTE_MUTATION_OK
- NA0586_NO_PUBLIC_READINESS_CLAIM_OK
- NA0586_NO_PRODUCTION_READINESS_CLAIM_OK
- NA0586_RESULT_CLASSIFICATION_SELECTED_OK
- NA0586_SUCCESSOR_SELECTED_OK
- NA0586_ONE_READY_INVARIANT_OK

## Classification Proof

- D-1161 consumed: yes.
- D-1162 consumed: yes.
- Fresh qwork proof from `2026-07-01T20:57:45Z` verified: yes.
- Current main checks classified: public-safety success, advisories success,
  suite2-vectors success, no failed required checks, no required pending checks,
  cargo audits success, locked metadata success, and no Cargo drift.
- Authority model applied: yes.
- Current diagnostic surface review: qsc/helper fields present.
- Workflow diagnostic action: exact authorized workflow dispatches on `main`.
- Workflow results: both completed failure, used only for diagnostic evidence.
- Diagnostic surface: present and parseable for remote-handshake and
  remote-relay.
- Timeout phase: `dns_timeout`.
- Failure-cause investigation: skipped because diagnostics were present,
  parseable, and actionable.
- Diagnostic fix: skipped because no fix was needed.
- Optional remote postcheck: skipped as not needed.
- Result classification:
  `REMOTE_RELAY_DIAGNOSTIC_VERIFICATION_DNS_TIMEOUT`.
- Selected successor: NA-0587 network-path remediation harness.

## Boundary Proof

No endpoint values, private port values, token values, Authorization values,
route-token/capability values, payloads, response bodies, process identities,
private topology, authorized_keys content, public key material, private key
material, or secret values are published.

No qsl-server source mutation, qsl-attachments work, dependency/lockfile change,
workflow file mutation, qwork/qstart/qresume execution, or remote mutation is
introduced. No public readiness claim is introduced. No production readiness
claim is introduced. No vulnerability-free claim is introduced. No bug-free
claim is introduced. No perfect-build claim is introduced. No perfect-crypto
claim is introduced.

## Validation

NA-0586 validation covers diff/scope/queue/marker proof, link check,
private-material scans, overclaim scan, docs/governance/source classifier, PR
body preflight, goal-lint, root and nested cargo audits, locked metadata, cargo
fmt, and qsc adversarial shell syntax. Focused qsc runtime tests are skipped
because no qsc source/runtime path changed.
