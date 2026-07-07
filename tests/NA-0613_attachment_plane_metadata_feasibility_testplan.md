Goals: G5 (primary), supports G1, G2, G3, G4

Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-07-07

# NA-0613 — Attachment-Plane Metadata Mitigation Feasibility Test Plan

## Scope

Records class-safe markers for the NA-0613 read-only attachment-plane metadata
feasibility+design study under directive QSL-DIR-2026-07-07-550 (D550), LITE-CEREMONY
class. Reads code and the doc corpus; changes no source, test, Cargo, workflow, or
attachment contract; applies no fix.

## Required Markers

- NA0613_D1221_CONSUMED_OK
- NA0613_D1222_CONSUMED_OK
- NA0613_FRESH_QWORK_PROOF_OK
- NA0613_D1223_ABSENT_BEFORE_IMPLEMENTATION_OK
- NA0613_LITE_CEREMONY_CERTIFIED_OK
- NA0613_READ_ONLY_NO_SOURCE_MUTATION_OK
- NA0613_DOC_G5_006_AUTHORED_OK
- NA0613_CHANNEL_INVENTORY_C1_C4_OK
- NA0613_THREAT_OBSERVATION_SEPARATION_OK
- NA0613_M1_CLIENT_FEASIBLE_NO_CONTRACT_CHANGE_OK
- NA0613_COST_BENEFIT_MATRIX_OK
- NA0613_HONEST_RESIDUAL_RECORDED_OK
- NA0613_ENG0007_RESOLVED_OK
- NA0613_ENG0010_ENG0011_FILED_OK
- NA0613_SUCCESSOR_NA0614_SELECTED_OK
- NA0613_PRIVATE_MATERIAL_SCAN_OK
- NA0613_RESULT_CLASSIFICATION_SELECTED_OK

## Validation Plan (class-only)

1. Verify fresh operator qwork proof (lane NA-0613) and live main health; D-1221/D-1222
   once and D-1223 absent; READY_COUNT 1 with READY NA-0613; DOC-G5-006 free.
2. Verify DOC-G5-006 records the threat/observation separation (peer descriptor vs
   service opaque object), the C1–C4 channel inventory, the M1–M4 mitigations, the
   cost/benefit matrix, the ranked recommendations, and the honest residual.
3. Verify the ledger resolves ENG-0007 and files ENG-0010 (recommended M1/M2/M3) and
   ENG-0011 (deferred timing/cover).
4. Confirm scope guard (only docs/governance paths; NO `.rs`/Cargo/workflow/`.claude`/
   contract); confirm no fix; run the no-private-material scan.
5. Restore NA-0614 (object-size/part-count bucketing implementation) as the sole READY
   successor.

## Result

`ATTACHMENT_PLANE_METADATA_MITIGATION_DESIGN_ESTABLISHED`. Design:
`docs/design/DOC-G5-006_Attachment_Plane_Metadata_Mitigation_Feasibility_and_Design_v0.1.0_DRAFT.md`.
