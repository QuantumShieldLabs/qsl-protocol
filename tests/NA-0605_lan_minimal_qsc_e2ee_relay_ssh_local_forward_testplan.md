# NA-0605 LAN Minimal qsc E2EE Relay via SSH Local-Forward Test Plan

Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-07-05

Goals: G1, G2, G3, G4, G5

## Scope

This test plan records class-safe proof review for the operator-assisted
private-LAN SSH local-forward tiny qsc E2EE relay verification. It does not
authorize qsc source mutation, qsl-server source mutation, qsl-attachments
runtime, dependency or lockfile mutation, workflow dispatch, Tailnet, public
endpoint exposure, laptop Codex control, or private-value publication.

## Required Markers

- NA0605_D1199_TLS_POLICY_CONSUMED_OK
- NA0605_D1200_CLOSEOUT_CONSUMED_OK
- NA0605_FRESH_QWORK_PROOF_OK
- NA0605_CURRENT_MAIN_CHECKS_CLASSIFIED_OK
- NA0605_QSL_PROTOCOL_TRANSPORT_DISTINCTION_OK
- NA0605_QSL_SERVER_VALIDATION_OK
- NA0605_QSL_SERVER_LOOPBACK_BIND_OK
- NA0605_QSL_SERVER_STARTUP_CLASSIFIED_OK
- NA0605_OPERATOR_COMMAND_PACKET_CREATED_OK
- NA0605_LAPTOP_PROOF_CLASSIFIED_OK
- NA0605_SSH_LOCAL_FORWARD_CLASSIFIED_OK
- NA0605_TINY_SEND_CLASSIFIED_OK
- NA0605_TINY_RECEIVE_DECRYPT_VALIDATE_CLASSIFIED_OK
- NA0605_QSL_SERVER_PLAINTEXT_EXPOSURE_CLASSIFIED_OK
- NA0605_SEED_FALLBACK_CLASSIFIED_OK
- NA0605_METADATA_REVIEW_OK
- NA0605_CLEANUP_DONE_OK
- NA0605_HOSTILE_ANALYST_METADATA_ROADMAP_RECORDED_OK
- NA0605_PRIVATE_MATERIAL_SCAN_OK
- NA0605_NO_ENDPOINT_VALUE_PUBLISHED_OK
- NA0605_NO_PRIVATE_PORT_VALUE_PUBLISHED_OK
- NA0605_NO_TOKEN_OR_AUTHORIZATION_PUBLISHED_OK
- NA0605_NO_CAPABILITY_VALUE_PUBLISHED_OK
- NA0605_NO_PAYLOAD_BODY_PLAINTEXT_PUBLISHED_OK
- NA0605_NO_SEED_OR_KEY_MATERIAL_PUBLISHED_OK
- NA0605_NO_QSL_ATTACHMENTS_RUNTIME_OK
- NA0605_NO_CODEX_SSH_TO_LAPTOP_OK
- NA0605_NO_LAPTOP_SSH_SERVER_SETUP_OK
- NA0605_NO_SECOND_CODEX_ON_LAPTOP_OK
- NA0605_NO_REMOTE_TAILSCALE_WORKFLOW_ACTION_OK
- NA0605_NO_PUBLIC_SITE_CLOUDFLARE_MUTATION_OK
- NA0605_NO_PUBLIC_READINESS_CLAIM_OK
- NA0605_NO_PRODUCTION_READINESS_CLAIM_OK
- NA0605_NO_REMOTE_READY_CLAIM_OK
- NA0605_NO_TAILNET_READY_CLAIM_OK
- NA0605_NO_LAN_READY_OVERCLAIM_OK
- NA0605_NO_CRYPTO_COMPLETE_CLAIM_OK
- NA0605_NO_ATTACHMENT_COMPLETE_CLAIM_OK
- NA0605_RESULT_CLASSIFICATION_SELECTED_OK
- NA0605_SUCCESSOR_SELECTED_OK
- NA0605_ONE_READY_INVARIANT_OK

## Validation

- qwork proof file verification
- live repo status and HEAD/origin-main proof
- current-main required check classification
- D-1199/D-1200 inheritance review
- qsl-server locked metadata, audit, format, tests, and build
- qsl-server loopback bind and startup classification
- operator packet creation and private-material scan
- laptop result schema validation
- laptop result private-material value scan
- LAN tiny-message class result classification
- qsl-server plaintext/log review
- seed fallback review
- metadata review
- cleanup proof
- hostile analyst roadmap proof
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
