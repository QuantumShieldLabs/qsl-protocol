Goals: G4 (primary), supports G1, G2, G3, G5

Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-07-07

# NA-0611 — Constant-Time Comparison Sweep Test Plan

## Scope

Records class-safe audit markers for the NA-0611 read-only constant-time comparison
sweep under directive QSL-DIR-2026-07-07-548 (D548), LITE-CEREMONY class. Reads
security-critical code; changes no source, test, Cargo, or workflow; applies no fix.

## Required Markers

- NA0611_D1219_CONSUMED_OK
- NA0611_D1220_CONSUMED_OK
- NA0611_FRESH_QWORK_PROOF_OK
- NA0611_D1221_ABSENT_BEFORE_IMPLEMENTATION_OK
- NA0611_LITE_CEREMONY_CERTIFIED_OK
- NA0611_READ_ONLY_NO_SOURCE_MUTATION_OK
- NA0611_SWEEP_COMPLETE_OK
- NA0611_NO_KEYED_SECRET_COMPARE_OUTSIDE_HANDSHAKE_OK
- NA0611_ENG0005_RESOLVED_OK
- NA0611_ENG0008_OPTIONAL_DEFENSE_IN_DEPTH_FILED_OK
- NA0611_SUCCESSOR_NA0612_ENG0006_SELECTED_OK
- NA0611_PRIVATE_MATERIAL_SCAN_OK
- NA0611_RESULT_CLASSIFICATION_SELECTED_OK

## Validation Plan (class-only)

1. Verify fresh operator qwork proof (lane NA-0611) and live main health before
   mutation; verify D-1219/D-1220 once and D-1221 absent; READY_COUNT 1 with READY
   NA-0611.
2. Verify the evidence doc records the sweep classification: keyed-MAC compares only
   in the already-fixed handshake seam; kmac_out are derivations; AEAD tag verify is
   in-primitive constant-time; integrity-hash and route-token compares
   verified-acceptable; and the optional P3 ENG-0008 verify-code item.
3. Verify the ledger resolves ENG-0005 and files ENG-0008.
4. Confirm scope guard (only docs/governance paths; NO `.rs`/Cargo/workflow/`.claude`);
   confirm no fix applied; run the no-private-material scan.
5. Restore NA-0612 (ENG-0006 error/retry review) as the sole READY successor.

## Result

`CONSTANT_TIME_POSTURE_SOUND_NO_KEYED_SECRET_COMPARE_OUTSIDE_HANDSHAKE`. Evidence:
`docs/governance/evidence/NA-0611_constant_time_comparison_sweep_harness.md`.
