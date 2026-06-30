Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-30

# NA-0576 Project-Wide Bounded Codex Operational Authority Hardening Testplan

## Purpose

Record deterministic acceptance markers for making bounded Codex operational
authority durable project-wide while preserving the qsl-server technical thread
as NA-0577.

## Required Markers

- NA0576_D1140_AUTHORITY_MODEL_CONSUMED_OK
- NA0576_D1141_CLOSEOUT_CONSUMED_OK
- NA0576_FRESH_QWORK_PROOF_OK
- NA0576_OPERATOR_REQUEST_CONSUMED_OK
- NA0576_AUTHORITY_TIERS_DEFINED_OK
- NA0576_DIRECTIVE_OPT_IN_REQUIRED_OK
- NA0576_APPROVED_TEST_HOST_REGISTRY_DEFINED_OK
- NA0576_REDACTED_EVIDENCE_POLICY_DEFINED_OK
- NA0576_RAW_OUTPUT_QUARANTINE_POLICY_DEFINED_OK
- NA0576_TIER1_DIAGNOSTICS_DEFINED_OK
- NA0576_TIER2_BOUNDED_TEST_ACTION_DEFINED_OK
- NA0576_TIER3_OPERATOR_ADMIN_BOUNDARY_DEFINED_OK
- NA0576_TIER4_FORBIDDEN_BOUNDARY_DEFINED_OK
- NA0576_CONTINUOUS_CI_WAIT_WORK_POLICY_DEFINED_OK
- NA0576_READ_ONLY_FORWARD_AUDIT_DURING_WAITS_DEFINED_OK
- NA0576_NO_IDLE_POLLING_POLICY_DEFINED_OK
- NA0576_WAIT_WORK_REPORTING_REQUIRED_OK
- NA0576_NEXT_LANE_WORK_DURING_WAITS_FORBIDDEN_OK
- NA0576_START_HERE_UPDATED_OK
- NA0576_AGENTS_UPDATED_OK
- NA0576_QSL_SERVER_THREAD_PRESERVED_OK
- NA0576_SUCCESSOR_NA0577_SELECTED_OK
- NA0576_NO_REMOTE_ACTION_OK
- NA0576_NO_QSL_SERVER_START_OK
- NA0576_NO_QSC_SEND_RECEIVE_OK
- NA0576_NO_WORKFLOW_DISPATCH_OK
- NA0576_NO_QSL_ATTACHMENTS_OK
- NA0576_NO_QSL_PROTOCOL_SOURCE_MUTATION_OK
- NA0576_NO_PRIVATE_MATERIAL_PUBLISHED_OK
- NA0576_NO_PUBLIC_READINESS_CLAIM_OK
- NA0576_NO_PRODUCTION_READINESS_CLAIM_OK
- NA0576_NO_VULNERABILITY_FREE_CLAIM_OK
- NA0576_NO_BUG_FREE_CLAIM_OK
- NA0576_ONE_READY_INVARIANT_OK

## Evidence Mapping

- D-1140/D-1141 consumption maps to `DECISIONS.md`, `TRACEABILITY.md`, and
  the NA-0575 evidence/testplan files.
- Authority model markers map to
  `docs/ops/CODEX_BOUNDED_OPERATIONAL_AUTHORITY.md`.
- Continuous CI wait-work markers map to the productive CI wait-work and
  read-only forward-audit policy in
  `docs/ops/CODEX_BOUNDED_OPERATIONAL_AUTHORITY.md`, plus the START_HERE and
  AGENTS pointers.
- START_HERE and AGENTS markers map to the added bounded authority sections in
  those files.
- qsl-server thread preservation maps to the NA-0577 successor selected in
  D-1142 and the NA-0576 evidence document.
- Boundary markers map to scope guard, private-material scan, overclaim scan,
  validation output, and no-runtime/no-remote-action proof.

## Expected Result

NA-0576 records D-1142, creates the durable bounded authority runbook, updates
START_HERE and AGENTS, preserves the qsl-server technical thread, selects
NA-0577 as successor, and keeps exactly one READY item as NA-0576 until
optional closeout. No remote action, qsl-server start, qsc send/receive,
workflow dispatch/rerun, qsl-attachments work, qsl-protocol runtime/source
mutation, private-material publication, or public/production/security overclaim
occurs. The amendment additionally records that long CI waits require
productive current-lane wait-work or read-only forward audits when such work is
available, require wait-work reporting, forbid idle polling, and forbid
next-lane implementation during waits.
