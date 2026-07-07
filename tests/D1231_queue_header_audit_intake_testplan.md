Goals: G4 (primary), supports G1, G2, G3, G5

Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-07-07

# D-1231 — Queue-Header Restructure + Comprehensive-Audit Intake Test Plan (governance)

## Scope

No-source governance/housekeeping change (D-1231; not an NA lane). Adds the `## LIVE QUEUE`
header to `NEXT_ACTIONS.md`, records the number≠run-order convention and ledger-as-backlog in
DOC-OPS-006, marks the DOC-G5-005 §9 table superseded, files ENG-0019/0020/0021 + the
ENG-0012 addendum + WF-0011, and updates a START_HERE pointer. No source/test/Cargo/CI-script
change; NA-0618/ENG-0013 remains the sole READY item.

## Required Markers

- D1231_LIVE_QUEUE_HEADER_ADDED_OK
- D1231_QUEUE_PARSER_STILL_RESOLVES_READY_COUNT_1_NA0618_OK
- D1231_ON_DECK_NOT_MISCOUNTED_AS_READY_OK
- D1231_NUMBER_NOT_RUNORDER_CONVENTION_RECORDED_OK
- D1231_LEDGER_IS_SINGLE_BACKLOG_G5005_SECTION9_SUPERSEDED_OK
- D1231_ENG0019_ENG0020_ENG0021_FILED_ONCE_OK
- D1231_ENG0012_ADDENDUM_APPENDED_OK
- D1231_WF0011_FILED_OK
- D1231_ALREADY_COVERED_MAPPING_RECORDED_OK
- D1231_NO_SOURCE_NO_CI_SCRIPT_CHANGE_OK
- D1231_SOLE_READY_UNCHANGED_NA0618_OK
- D1231_PRIVATE_MATERIAL_SCAN_OK

## Validation Plan (class-only)

1. `scripts/ci/qsl_evidence_helper.py queue --file NEXT_ACTIONS.md` reports `READY_COUNT 1`
   and `READY NA-0618` after the header insert (the ON DECK list is plain text and is not
   parsed as an NA block).
2. Scope guard: the changed set is docs/governance only — `NEXT_ACTIONS.md`,
   `docs/ops/IMPROVEMENT_LEDGER.md`, `docs/ops/DIRECTOR_OPERATIONS.md`,
   `docs/design/DOC-G5-005_*`, `START_HERE.md`, `DECISIONS.md`, `TRACEABILITY.md`,
   `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`, and this testplan. No `*.rs`, no `Cargo.*`, no
   `scripts/ci/*`, no `.github/*`, no `.claude/*`.
3. Ledger: ENG-0019/0020/0021 and WF-0011 present exactly once; the ENG-0012 addendum is
   appended; the already-covered mapping is recorded in D-1231.
4. Convention: DOC-OPS-006 records number≠run-order, ledger-as-backlog, and the WF-0011
   deferral of the physical archive split (naming the three CI scripts).
5. §9: the DOC-G5-005 §9 table carries the SUPERSEDED banner.
6. goal-lint: the PR body carries a `Goals:` line. Private-material scan on added lines.

## Result

`D1231_QUEUE_HEADER_AND_AUDIT_INTAKE_OK`. Queue legible at the top; ledger is the single
backlog; three new findings + WF-0011 filed; ENG-0012 sharpened; NA-0618/ENG-0013 unchanged
as sole READY (begins D-1232). Physical archive split deferred to WF-0011.
