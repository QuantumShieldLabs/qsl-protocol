# NA-0607 LAN Minimal qsl-attachments Send Receive SSH Loopback-Forward Test Plan

Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-07-05

Goals: G1, G2, G3, G4, G5

## Scope

This test plan records class-safe proof review for the first bounded
private-LAN qsl-attachments send/receive verification through an SSH
loopback-forward harness. It does not authorize qsc source mutation,
qsl-server source mutation, qsl-attachments source mutation, dependency or
lockfile mutation, workflow dispatch, Tailnet, public endpoint exposure, sudo,
system install, personal laptop file access, or private-value publication.

## Required Markers

- NA0607_D1203_LAN_TINY_HOSTILE_PASS_CONSUMED_OK
- NA0607_D1204_CLOSEOUT_CONSUMED_OK
- NA0607_FRESH_QWORK_PROOF_OK
- NA0607_CURRENT_MAIN_CHECKS_CLASSIFIED_OK
- NA0607_QSCWORK_ACCESS_SAFETY_OK
- NA0607_QSCWORK_QSC_AVAILABLE_OK
- NA0607_QSL_SERVER_VALIDATION_OK
- NA0607_QSL_ATTACHMENTS_VALIDATION_OK
- NA0607_QSL_SERVER_STARTUP_OK
- NA0607_QSL_ATTACHMENTS_STARTUP_OK
- NA0607_SSH_LOOPBACK_FORWARDS_CLASSIFIED_OK
- NA0607_QSC_ATTACHMENT_SEND_CLASSIFIED_OK
- NA0607_QSC_ATTACHMENT_FETCH_DECRYPT_VALIDATE_CLASSIFIED_OK
- NA0607_QSC_ENCRYPTION_OWNER_CONFIRMED_OK
- NA0607_QSL_SERVER_PLAINTEXT_EXPOSURE_CLASSIFIED_OK
- NA0607_QSL_ATTACHMENTS_PLAINTEXT_KEY_EXPOSURE_CLASSIFIED_OK
- NA0607_SEED_FALLBACK_CLASSIFIED_OK
- NA0607_CAPABILITY_EXPOSURE_CLASSIFIED_OK
- NA0607_METADATA_MATRIX_OK
- NA0607_CLEANUP_DONE_OK
- NA0607_PRIVATE_MATERIAL_SCAN_OK
- NA0607_NO_ENDPOINT_VALUE_PUBLISHED_OK
- NA0607_NO_PRIVATE_PORT_VALUE_PUBLISHED_OK
- NA0607_NO_TOKEN_OR_AUTHORIZATION_PUBLISHED_OK
- NA0607_NO_CAPABILITY_VALUE_PUBLISHED_OK
- NA0607_NO_PAYLOAD_BODY_PLAINTEXT_PUBLISHED_OK
- NA0607_NO_SEED_OR_KEY_MATERIAL_PUBLISHED_OK
- NA0607_NO_QSC_SOURCE_MUTATION_OK
- NA0607_NO_QSL_SERVER_SOURCE_MUTATION_OK
- NA0607_NO_QSL_ATTACHMENTS_SOURCE_MUTATION_OK
- NA0607_NO_REMOTE_TAILSCALE_WORKFLOW_ACTION_OK
- NA0607_NO_PUBLIC_SITE_CLOUDFLARE_MUTATION_OK
- NA0607_NO_PUBLIC_READINESS_CLAIM_OK
- NA0607_NO_PRODUCTION_READINESS_CLAIM_OK
- NA0607_NO_REMOTE_READY_CLAIM_OK
- NA0607_NO_TAILNET_READY_CLAIM_OK
- NA0607_NO_LAN_READY_OVERCLAIM_OK
- NA0607_NO_CRYPTO_COMPLETE_CLAIM_OK
- NA0607_NO_ATTACHMENT_COMPLETE_CLAIM_OK
- NA0607_RESULT_CLASSIFICATION_SELECTED_OK
- NA0607_SUCCESSOR_SELECTED_OK
- NA0607_ONE_READY_INVARIANT_OK

## Validation

- qwork proof file verification
- live repo status and HEAD/origin-main proof
- disk and `/backup/qsl` mount proof
- current-main required check classification
- D-1203/D-1204 inheritance review
- qscwork access safety review
- qsc availability and seed-fallback environment review on qscwork
- qsl-server locked metadata, audit, format, tests, and build
- qsl-attachments locked metadata, audit, format, tests, and build
- qsl-server loopback bind and startup classification
- qsl-attachments loopback/proof-root storage startup classification
- SSH loopback-forward setup and endpoint policy classification
- remote qsc attachment send classification
- remote qsc attachment fetch/decrypt/validate classification
- qsc encryption owner review
- qsl-server plaintext/log review
- qsl-attachments plaintext/key exposure review
- seed fallback review
- capability exposure review
- metadata minimization matrix
- cleanup proof
- private-material review
- result classification and successor selection
- scope guard
- marker proof
- link check
- added-line/new-file private-material scan
- runtime artifact/log private-material scan
- qscwork artifact private-material scan
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
- qsl-attachments validation rerun
