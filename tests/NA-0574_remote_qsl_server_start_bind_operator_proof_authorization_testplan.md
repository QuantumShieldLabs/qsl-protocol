Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-30

# NA-0574 Remote qsl-server Start / Bind Operator Proof Authorization Testplan

## Purpose

Record deterministic acceptance markers for NA-0574. This lane is
authorization-only and selects the non-secret NA-0575 operator proof capture
model without starting qsl-server or executing remote actions.

## Required Markers

- NA0574_D1136_QSL_SERVER_STAGE_CONSUMED_OK
- NA0574_D1137_CLOSEOUT_CONSUMED_OK
- NA0574_FRESH_QWORK_PROOF_OK
- NA0574_CURRENT_MAIN_CHECKS_CLASSIFIED_OK
- NA0574_D498_BIND_BLOCKER_CONSUMED_OK
- NA0574_OPERATOR_PROOF_SCHEMA_SELECTED_OK
- NA0574_FUTURE_PROOF_FILES_SELECTED_OK
- NA0574_FUTURE_PROOF_FIELDS_SELECTED_OK
- NA0574_SAFE_TO_PASTE_POLICY_SELECTED_OK
- NA0574_PRIVATE_MATERIAL_POLICY_SELECTED_OK
- NA0574_DECISION_TREE_SELECTED_OK
- NA0574_RESULT_CLASSIFICATION_SELECTED_OK
- NA0574_SUCCESSOR_SELECTED_OK
- NA0574_NO_REMOTE_ACTION_OK
- NA0574_NO_SSH_SCP_TAILSCALE_OK
- NA0574_NO_QSL_SERVER_START_OK
- NA0574_NO_QSC_SEND_RECEIVE_OK
- NA0574_NO_WORKFLOW_DISPATCH_OK
- NA0574_NO_QSL_ATTACHMENTS_OK
- NA0574_NO_QSL_PROTOCOL_SOURCE_MUTATION_OK
- NA0574_NO_ENDPOINT_VALUE_PUBLISHED_OK
- NA0574_NO_PRIVATE_PORT_VALUE_PUBLISHED_OK
- NA0574_NO_TOKEN_OR_AUTHORIZATION_PUBLISHED_OK
- NA0574_NO_PROCESS_IDENTITY_PUBLISHED_OK
- NA0574_NO_PAYLOAD_BODY_PUBLISHED_OK
- NA0574_NO_PRIVATE_MATERIAL_PUBLISHED_OK
- NA0574_NO_PUBLIC_READINESS_CLAIM_OK
- NA0574_NO_PRODUCTION_READINESS_CLAIM_OK
- NA0574_ONE_READY_INVARIANT_OK

## Evidence Mapping

- D-1136 and D-1137 markers map to DECISIONS.md inheritance proof.
- qwork and current-main markers map to proof-root startup and required-check
  classification artifacts.
- D498 blocker markers map to the NA-0573 evidence/testplan and D498 response.
- Operator schema, future proof files/fields, safe-to-paste policy,
  private-material policy, decision tree, result classification, and successor
  markers map to the NA-0574 evidence document.
- Boundary markers map to scope guard, private-material scan, overclaim scan,
  and no-runtime/no-remote/no-workflow evidence.

## Expected Result

NA-0574 is accepted when D-1138 exists once, D-1139 is absent, READY_COUNT
remains 1 with READY NA-0574 before closeout, the implementation scope guard
contains only the allowed five implementation files, all required markers above
are present, no private material is published, and no qsl-server start or remote
action occurs.
