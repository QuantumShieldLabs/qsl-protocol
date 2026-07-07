Goals: G5 (primary), supports G1, G2, G3, G4

Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-07-07

# NA-0609 — Metadata and Implementation-Attack Hardening Plan Test Plan

## Scope

Records class-safe validation for the NA-0609 planning lane under directive
QSL-DIR-2026-07-06-546 (D546). Planning/docs only; changes no source, test, Cargo,
workflow, or spec; authorizes no LAN/runtime action; implements no hardening item.

## Required Markers

- NA0609_D1215_CONSUMED_OK
- NA0609_D1216_CONSUMED_OK
- NA0609_FRESH_QWORK_PROOF_OK
- NA0609_D1217_ABSENT_BEFORE_IMPLEMENTATION_OK
- NA0609_DOC_G5_005_AUTHORED_OK
- NA0609_SEVEN_AREAS_COVERED_OK
- NA0609_RANKED_BACKLOG_OK
- NA0609_LEDGER_FOLDED_OK
- NA0609_NO_SOURCE_MUTATION_OK
- NA0609_NO_LAN_RUNTIME_AUTHORIZED_OK
- NA0609_SUCCESSOR_NA0610_SELECTED_OK
- NA0609_NO_METADATA_FREE_CLAIM_OK
- NA0609_PRIVATE_MATERIAL_SCAN_OK
- NA0609_RESULT_CLASSIFICATION_SELECTED_OK

## Validation Plan (class-only)

1. Verify fresh operator qwork proof and live main health before mutation; verify
   D-1215/D-1216 once and D-1217 absent; READY_COUNT 1 with READY NA-0609.
2. Verify DOC-G5-005 exists with the DOC-G5-005 designation, covers the seven
   objective areas, and produces a prioritized backlog (§9) with severities and
   recommended lane shapes; verify it respects the DOC-G5-001 non-goals (no
   "metadata eliminated" claim).
3. Verify the ledger folds the backlog (ENG-0005/0006/0007 added; ENG-0001/0002
   ranked in the plan).
4. Confirm scope guard (only docs/governance paths; NO `.rs`/Cargo/workflow/
   `.claude`); confirm no LAN/runtime authorization and no hardening implementation;
   run the no-private-material and overclaim/claim-boundary scans.
5. Select the NA-0610 successor at closeout.

## Result

`HOSTILE_ANALYST_METADATA_IMPL_ATTACK_HARDENING_PLAN_ESTABLISHED`. Evidence:
`docs/governance/evidence/NA-0609_hostile_analyst_metadata_impl_attack_hardening_plan_harness.md`.
