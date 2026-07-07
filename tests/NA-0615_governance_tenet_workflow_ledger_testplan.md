Goals: G4 (primary), supports G1, G2, G3, G5

Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-07-07

# NA-0615 — Governance Tenet + Workflow Ledger Test Plan

## Scope

Class-safe validation for the NA-0615 docs/governance LITE-CEREMONY lane under directive
QSL-DIR-2026-07-07-552 (D552). Records the design tenet, DOC-OPS-006 §9a/§9b, and
WF-0006..WF-0009. No source/workflow/dependency change; no operator-infra edit.

## Required Markers

- NA0615_D1224_CONSUMED_OK
- NA0615_D1225_CONSUMED_OK
- NA0615_FRESH_STARTUP_PROOF_OK
- NA0615_D1226_ABSENT_BEFORE_IMPLEMENTATION_OK
- NA0615_LITE_CEREMONY_CERTIFIED_OK
- NA0615_DESIGN_TENET_IN_CHARTER_OK
- NA0615_DOC_OPS_006_9A_9B_ADDED_OK
- NA0615_WF_0006_TO_0009_FILED_OK
- NA0615_DOCS_GOVERNANCE_ONLY_OK
- NA0615_NO_OPERATOR_INFRA_EDIT_OK
- NA0615_SUCCESSOR_NA0616_SELECTED_OK
- NA0615_PRIVATE_MATERIAL_SCAN_OK
- NA0615_RESULT_CLASSIFICATION_SELECTED_OK

## Validation Plan (class-only)

1. Verify fresh operator startup proof (lane NA-0615) and main health; D-1224/D-1225
   once, D-1226 absent; sole READY NA-0615.
2. Verify PROJECT_CHARTER gains the Design tenets subsection; DOC-OPS-006 gains §9a
   (LITE read-only-audit fast-path checklist) and §9b (batch-audit convention);
   IMPROVEMENT_LEDGER gains WF-0006..WF-0009.
3. Scope guard: only docs/governance paths changed; NO `.rs`/Cargo/workflow/`.claude`/
   hook and no operator-infra edit. Run the no-private-material and overclaim scans.
4. Restore NA-0616 (default: ENG-0001 self-label footgun remediation) as the sole READY
   successor.

## Result

`GOVERNANCE_TENET_AND_WORKFLOW_LEDGER_RECORDED`. Evidence:
`docs/governance/evidence/NA-0615_governance_tenet_workflow_ledger_harness.md`.
