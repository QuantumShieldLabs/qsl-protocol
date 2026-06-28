Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-28

# NA-0562 Remote Relay Service Listener Non-Secret Proof Capture Testplan

This governance testplan records the NA-0562 service listener non-secret proof
capture evidence, D482 recovery, redaction boundary, result classification, and
selected successor.

## Required Markers

NA0562_D1112_AUTHORITY_CONSUMED_OK
NA0562_D1113_CLOSEOUT_CONSUMED_OK
NA0562_D482_MARKER_MISMATCH_RECOVERED_OK
NA0562_FRESH_QWORK_PROOF_OK
NA0562_CURRENT_MAIN_CHECKS_CLASSIFIED_OK
NA0562_PUBLIC_SAFETY_GREEN_OK
NA0562_ADVISORIES_GREEN_OK
NA0562_NO_FAILED_REQUIRED_CHECKS_OK
NA0562_REMOTE_LISTENER_PROBE_SCRIPT_GENERATED_OK
NA0562_REMOTE_LISTENER_PROBE_SCRIPT_STATIC_REVIEW_OK
NA0562_SSH_LISTENER_READINESS_CLASSIFIED_OK
NA0562_REMOTE_LISTENER_PROBE_EXECUTED_OR_ACCESS_STOP_OK
NA0562_PRIVATE_MATERIAL_SCAN_OK
NA0562_LISTENER_PRESENCE_CLASSIFIED_OK
NA0562_TCP_CONNECT_CLASSIFIED_OK
NA0562_V1_PUSH_HEAD_CLASSIFIED_OR_NOT_CHECKED_OK
NA0562_RESULT_CLASSIFICATION_SELECTED_OK
NA0562_SUCCESSOR_MODEL_SELECTED_OR_STOP_RECORDED_OK
NA0562_NO_ENDPOINT_VALUE_PUBLISHED_OK
NA0562_NO_ROUTE_TOKEN_CAPABILITY_PUBLISHED_OK
NA0562_NO_BEARER_AUTH_PUBLISHED_OK
NA0562_NO_PRIVATE_TOPOLOGY_PUBLISHED_OK
NA0562_NO_PROCESS_IDENTITY_PUBLISHED_OK
NA0562_NO_AUTHORIZED_KEYS_CONTENT_PUBLISHED_OK
NA0562_NO_PAYLOAD_BODY_PUBLISHED_OK
NA0562_NO_ACCOUNT_SERVICE_MUTATION_OK
NA0562_NO_QSC_SEND_RECEIVE_OK
NA0562_NO_WORKFLOW_DISPATCH_OK
NA0562_NO_RERUN_EXECUTED_OK
NA0562_NO_SOURCE_MUTATION_OK
NA0562_NO_SCRIPT_MUTATION_OK
NA0562_NO_WORKFLOW_MUTATION_OK
NA0562_NO_DEPENDENCY_LOCKFILE_MUTATION_OK
NA0562_NO_QSL_SERVER_ATTACHMENTS_OK
NA0562_NO_PUBLIC_SITE_MUTATION_OK
NA0562_NO_CLOUDFLARE_MUTATION_OK
NA0562_NO_PRIVATE_MATERIAL_PUBLISHED_OK
NA0562_NO_PUBLIC_READINESS_CLAIM_OK
NA0562_NO_PRODUCTION_READINESS_CLAIM_OK
NA0562_ONE_READY_INVARIANT_OK

## Evidence Summary

- D-1112 and D-1113 were consumed.
- D482 marker mismatch was recovered by using the D-1112 exact readiness marker.
- Fresh qwork proof from `2026-06-28T23:03:37Z` was verified.
- Current main required checks were classified: public-safety success,
  advisories success, suite2-vectors success, and no failed required checks.
- The proof-root-only remote listener script was generated and statically
  reviewed.
- SSH listener readiness was classified `SSH_LISTENER_PROBE_READY`.
- The remote listener probe executed once through SSH stdin after readiness
  succeeded.
- Listener presence class was `no`.
- TCP connect class was `refused`.
- v1_push HEAD class was `not_checked`.
- Result classification was `SERVICE_OWNER_PORT_MISMATCH_REMEDIATION_READY`.
- Selected successor was
  `NA-0563 -- QSL Remote Relay Loopback Port Alignment Authorization Plan`.

## Boundary Assertions

No endpoint values, route-token/capability values, bearer values, Authorization
headers, private topology, process identity, authorized_keys content, payloads,
response bodies, secret values, or private material were published.

No account/service mutation, qsc send/receive, workflow dispatch, rerun, source
mutation, repository script mutation, workflow mutation, dependency or lockfile
mutation, qsl-server/qsl-attachments action, public-site mutation, or Cloudflare
mutation occurred.

No public-readiness or production-readiness claim is made.
