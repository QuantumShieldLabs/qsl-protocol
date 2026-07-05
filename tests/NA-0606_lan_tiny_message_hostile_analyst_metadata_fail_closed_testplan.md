# NA-0606 LAN Tiny-Message Hostile Analyst Metadata and Fail-Closed Test Plan

Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-07-05

Goals: G1, G2, G3, G4, G5

## Scope

This test plan records class-safe proof review for the operator-assisted
private-LAN SSH local-forward tiny qsc E2EE hostile analyst and fail-closed
stress lane. It does not authorize qsc source mutation, qsl-server source
mutation, qsl-attachments runtime, dependency or lockfile mutation, workflow
dispatch, Tailnet, public endpoint exposure, laptop Codex control, or
private-value publication.

## Required Markers

- NA0606_D1201_LAN_TINY_PASS_CONSUMED_OK
- NA0606_D1202_CLOSEOUT_CONSUMED_OK
- NA0606_FRESH_QWORK_PROOF_OK
- NA0606_CURRENT_MAIN_CHECKS_CLASSIFIED_OK
- NA0606_QSL_SERVER_VALIDATION_OK
- NA0606_QSL_SERVER_LOOPBACK_STARTUP_OK
- NA0606_QSL_SERVER_NEGATIVES_CLASSIFIED_OK
- NA0606_OPERATOR_COMMAND_PACKET_CREATED_OK
- NA0606_LAPTOP_PROOF_CLASSIFIED_OK
- NA0606_BASELINE_TINY_E2EE_CLASSIFIED_OK
- NA0606_FAIL_CLOSED_NEGATIVE_MATRIX_OK
- NA0606_METADATA_MINIMIZATION_MATRIX_OK
- NA0606_QSL_SERVER_PLAINTEXT_LOGGING_REVIEW_OK
- NA0606_SEED_FALLBACK_QSC_STATE_REVIEW_OK
- NA0606_HOSTILE_ANALYST_ROADMAP_EXPANDED_OK
- NA0606_CLEANUP_DONE_OK
- NA0606_PRIVATE_MATERIAL_SCAN_OK
- NA0606_NO_ENDPOINT_VALUE_PUBLISHED_OK
- NA0606_NO_PRIVATE_PORT_VALUE_PUBLISHED_OK
- NA0606_NO_TOKEN_OR_AUTHORIZATION_PUBLISHED_OK
- NA0606_NO_CAPABILITY_VALUE_PUBLISHED_OK
- NA0606_NO_PAYLOAD_BODY_PLAINTEXT_PUBLISHED_OK
- NA0606_NO_SEED_OR_KEY_MATERIAL_PUBLISHED_OK
- NA0606_NO_QSL_ATTACHMENTS_RUNTIME_OK
- NA0606_NO_CODEX_SSH_TO_LAPTOP_OK
- NA0606_NO_LAPTOP_SSH_SERVER_SETUP_OK
- NA0606_NO_SECOND_CODEX_ON_LAPTOP_OK
- NA0606_NO_REMOTE_TAILSCALE_WORKFLOW_ACTION_OK
- NA0606_NO_PUBLIC_SITE_CLOUDFLARE_MUTATION_OK
- NA0606_NO_PUBLIC_READINESS_CLAIM_OK
- NA0606_NO_PRODUCTION_READINESS_CLAIM_OK
- NA0606_NO_REMOTE_READY_CLAIM_OK
- NA0606_NO_TAILNET_READY_CLAIM_OK
- NA0606_NO_LAN_READY_OVERCLAIM_OK
- NA0606_NO_CRYPTO_COMPLETE_CLAIM_OK
- NA0606_NO_ATTACHMENT_COMPLETE_CLAIM_OK
- NA0606_RESULT_CLASSIFICATION_SELECTED_OK
- NA0606_SUCCESSOR_SELECTED_OK
- NA0606_ONE_READY_INVARIANT_OK

## Validation

- qwork proof file verification
- live repo status and HEAD/origin-main proof
- current-main required check classification
- D-1201/D-1202 inheritance review
- qsl-server locked metadata, audit, format, tests, and build
- qsl-server loopback bind and startup classification
- build-server qsl-server route/auth fail-closed probes
- operator packet creation and private-material scan
- laptop result schema validation
- laptop result private-material value scan
- baseline tiny E2EE result classification
- fail-closed negative matrix
- metadata minimization matrix
- qsl-server plaintext/log review
- seed fallback and qsc state review
- hostile analyst hardening roadmap expansion
- cleanup proof
- scope guard
- marker proof
- link check
- added-line private-material scan
- runtime artifact/log private-material scan
- prohibited-material scan
- overclaim scan
- LAN/private-topology publication scan
- protocol/transport claim-boundary scan
- crypto/triple-ratchet/attachment/remote-readiness/LAN-readiness claim-boundary scan
- docs/governance/runtime-evidence classifier
- PR body preflight and goal-lint if available
- root cargo audit
- nested qsc fuzz cargo audit
- cargo metadata --locked
- cargo fmt --check
- qsc adversarial shell syntax checks
- qsl-server validation rerun
