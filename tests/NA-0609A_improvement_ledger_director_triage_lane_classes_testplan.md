Goals: G4 (primary), supports G1–G5

Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-07-06

# NA-0609A — Improvement Ledger, Director Triage, and Lane-Class Definitions Test Plan

## Scope

Records class-safe governance validation for NA-0609A, an inserted
governance/tooling lane executed under directive QSL-DIR-2026-07-06-542 (D542).
It does not implement NA-0609 and changes no qsc/qsl-server/qsl-attachments
source, test, dependency, lockfile, or workflow; no protocol behavior; no
`.claude/settings.json` or guardrail-hook edit; and no runtime/LAN action.

## Required Markers

- NA0609A_D1209_CONSUMED_OK
- NA0609A_D1210_CONSUMED_OK
- NA0609A_FRESH_QWORK_PROOF_OK
- NA0609A_CURRENT_MAIN_HEALTH_OK
- NA0609A_D1211_ABSENT_BEFORE_IMPLEMENTATION_OK
- NA0609A_LEDGER_CREATED_DOC_OPS_007_OK
- NA0609A_LEDGER_SEEDED_CLASS_ONLY_OK
- NA0609A_MANDATORY_READ_WIRING_OK
- NA0609A_DIRECTOR_TRIAGE_DISCIPLINE_OK
- NA0609A_LANE_CLASSES_DEFINED_OK
- NA0609A_FAIL_CLOSED_BOUNDARY_PRESERVED_OK
- NA0609A_NEXT_ACTIONS_INSERTED_IN_PROGRESS_OK
- NA0609A_ONE_READY_INVARIANT_OK
- NA0609A_NO_PROTOCOL_OR_SOURCE_MUTATION_OK
- NA0609A_NO_SETTINGS_OR_HOOK_MUTATION_OK
- NA0609A_PRIVATE_MATERIAL_SCAN_OK
- NA0609A_RESULT_CLASSIFICATION_SELECTED_OK
- NA0609A_SUCCESSOR_NA0609_SOLE_READY_OK

## Validation Plan (class-only)

1. Verify operator qwork proof for NA-0609 and live main health before mutation;
   verify D-1209/D-1210 once and D-1211 absent; READY_COUNT 1 with READY NA-0609.
2. Verify `docs/ops/IMPROVEMENT_LEDGER.md` exists with the DOC-OPS-007 header, the
   entry schema, the status lifecycle, and class-only seed entries.
3. Verify the ledger is referenced in `CLAUDE.md`, `START_HERE.md`, and `AGENTS.md`,
   and that `AGENTS.md` carries the read/file rule.
4. Verify `docs/ops/DIRECTOR_OPERATIONS.md` §8 (triage) and §9 (lane classes) exist
   with the fail-closed hard boundary intact.
5. Verify `NEXT_ACTIONS.md` inserts NA-0609A IN_PROGRESS and NA-0609 remains the
   sole READY item (READY_COUNT 1).
6. Confirm scope guard (only the allowed governance paths changed), no protocol/
   source mutation, no settings/hook edit; run the no-private-material scan and
   goal-lint; confirm no fail-closed rule was removed or narrowed.

## Result

Result classification: `IMPROVEMENT_LEDGER_AND_LANE_DISCIPLINE_ESTABLISHED`.
Evidence: `docs/governance/evidence/NA-0609A_improvement_ledger_director_triage_lane_classes_harness.md`.
