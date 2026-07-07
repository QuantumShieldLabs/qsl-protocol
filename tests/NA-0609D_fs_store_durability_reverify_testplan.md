Goals: G4 (primary), supports G1, G2, G3, G5

Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-07-06

# NA-0609D — fs_store Durability Re-Verification Test Plan

## Scope

Records validation for the NA-0609D read-only re-verification and ENG-0004
false-positive correction under directive QSL-DIR-2026-07-06-545 (D545),
LITE-CEREMONY class. Reads security-critical code; changes no source, test, Cargo,
or workflow.

## Required Markers

- NA0609D_D1214_CONSUMED_OK
- NA0609D_D1215_CONSUMED_OK
- NA0609D_FRESH_QWORK_PROOF_OK
- NA0609D_D1216_ABSENT_BEFORE_IMPLEMENTATION_OK
- NA0609D_LITE_CEREMONY_CERTIFIED_OK
- NA0609D_WRITE_ATOMIC_DURABLE_SEQUENCE_VERIFIED_OK
- NA0609D_UNIX_DIR_FSYNC_CONFIRMED_OK
- NA0609D_ENG0004_MARKED_WONTFIX_OK
- NA0609D_WF0005_FILED_OK
- NA0609D_NA0609B_EVIDENCE_ADDENDUM_OK
- NA0609D_READ_ONLY_NO_SOURCE_MUTATION_OK
- NA0609D_PRIVATE_MATERIAL_SCAN_OK
- NA0609D_RESULT_CLASSIFICATION_SELECTED_OK
- NA0609D_NA0609_SOLE_READY_OK

## Validation Plan (class-only)

1. Verify fresh operator qwork proof and live main health before mutation; verify
   D-1214/D-1215 once and D-1216 absent; READY_COUNT 1 with READY NA-0609.
2. Re-verify `fs_store/mod.rs`: `write_atomic` performs content sync_all -> atomic
   rename -> `fsync_dir_best_effort`; the `#[cfg(unix)]` variant does the real
   directory fsync; the no-op is only `#[cfg(not(unix))]`.
3. Set ledger ENG-0004 to wontfix with the correction; add WF-0005; add the
   corrective addendum to the NA-0609B evidence doc.
4. Confirm scope guard (only docs/governance paths; NO `.rs`/Cargo/workflow/
   `.claude`); run the no-private-material scan.

## Result

`FS_STORE_DURABILITY_SOUND_ON_UNIX_ENG0004_FALSE_POSITIVE`. Evidence:
`docs/governance/evidence/NA-0609D_fs_store_durability_reverify_harness.md`.
