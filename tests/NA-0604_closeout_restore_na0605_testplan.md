# NA-0604 Closeout and NA-0605 Restoration Test Plan

Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-07-05

Goals: G1, G2, G3, G4, G5

## Scope

This test plan records the NA-0604 closeout after D-1199 and PR #1482, and the
restoration of NA-0605 as the sole READY successor. It is closeout-only and
does not authorize NA-0605 implementation, LAN runtime execution, qsc source
mutation, qsl-server mutation, qsl-attachments runtime, workflow mutation,
dependency/lockfile mutation, Tailnet action, public endpoint action, laptop
commands by Codex, or private-value publication.

## Required Markers

- NA0604_CLOSEOUT_D1199_CONSUMED_OK
- NA0604_CLOSEOUT_PR1482_MERGED_OK
- NA0604_CLOSEOUT_PR1482_POSTMERGE_GREEN_OK
- NA0604_CLOSEOUT_TLS_REQUIRED_POLICY_CONFIRMED_OK
- NA0604_CLOSEOUT_QSL_PROTOCOL_LAYER_CLARIFIED_OK
- NA0604_CLOSEOUT_TRANSPORT_LAYER_CLARIFIED_OK
- NA0604_CLOSEOUT_SSH_LOCAL_FORWARD_SUCCESSOR_SELECTED_OK
- NA0604_CLOSEOUT_D1200_RESTORED_NA0605_OK
- NA0604_CLOSEOUT_NA0604_DONE_OK
- NA0604_CLOSEOUT_NO_NA0605_IMPLEMENTATION_OK
- NA0604_CLOSEOUT_NO_QSC_SOURCE_MUTATION_OK
- NA0604_CLOSEOUT_NO_QSL_SERVER_MUTATION_OK
- NA0604_CLOSEOUT_NO_QSL_ATTACHMENTS_RUNTIME_OK
- NA0604_CLOSEOUT_NO_CODEX_SSH_TO_LAPTOP_OK
- NA0604_CLOSEOUT_NO_LAPTOP_SSH_SERVER_SETUP_OK
- NA0604_CLOSEOUT_NO_SECOND_CODEX_ON_LAPTOP_OK
- NA0604_CLOSEOUT_NO_REMOTE_TAILSCALE_WORKFLOW_ACTION_OK
- NA0604_CLOSEOUT_NO_PUBLIC_SITE_CLOUDFLARE_MUTATION_OK
- NA0604_CLOSEOUT_PRIVATE_MATERIAL_SCAN_OK
- NA0604_CLOSEOUT_NO_ENDPOINT_VALUE_PUBLISHED_OK
- NA0604_CLOSEOUT_NO_PRIVATE_PORT_VALUE_PUBLISHED_OK
- NA0604_CLOSEOUT_NO_TOKEN_OR_AUTHORIZATION_PUBLISHED_OK
- NA0604_CLOSEOUT_NO_CAPABILITY_VALUE_PUBLISHED_OK
- NA0604_CLOSEOUT_NO_PAYLOAD_BODY_PLAINTEXT_PUBLISHED_OK
- NA0604_CLOSEOUT_NO_SEED_OR_KEY_MATERIAL_PUBLISHED_OK
- NA0604_CLOSEOUT_NO_PUBLIC_READINESS_CLAIM_OK
- NA0604_CLOSEOUT_NO_PRODUCTION_READINESS_CLAIM_OK
- NA0604_CLOSEOUT_NO_REMOTE_READY_CLAIM_OK
- NA0604_CLOSEOUT_NO_TAILNET_READY_CLAIM_OK
- NA0604_CLOSEOUT_NO_LAN_READY_OVERCLAIM_OK
- NA0604_CLOSEOUT_NO_CRYPTO_COMPLETE_CLAIM_OK
- NA0604_CLOSEOUT_NO_ATTACHMENT_COMPLETE_CLAIM_OK
- NA0604_CLOSEOUT_ONE_READY_INVARIANT_OK

## Validation

- qwork proof-file verification.
- D-1199 / PR #1482 inheritance review.
- PR #1482 post-merge required-check classification.
- closeout scope guard for the five allowed paths only.
- queue and decision proof for D-1199 once, D-1200 once, D-1201 absent,
  NA-0604 DONE, READY_COUNT 1, and READY NA-0605.
- marker proof.
- link-check.
- added-line and new-file private-material scan.
- prohibited-material and overclaim scans.
- LAN/private-topology publication scan.
- protocol/transport claim-boundary scan.
- crypto/triple-ratchet/attachment claim-boundary scan.
- docs/governance-only classifier.
- PR body preflight and goal-lint if available.
- root cargo audit, nested qsc fuzz cargo audit, locked metadata, cargo fmt,
  and qsc-adversarial shell syntax checks.

Focused runtime tests are skipped because this is a closeout-only governance
patch with no source/runtime/workflow/dependency mutation, no LAN runtime
execution, and no remote/Tailnet execution.
