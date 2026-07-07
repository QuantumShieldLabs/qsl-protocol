Goals: G5 (primary), supports G1, G2, G3, G4

Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-07-07

# NA-0612 — Error/Retry Normalization Review Test Plan

## Scope

Records class-safe audit markers for the NA-0612 read-only error/retry normalization
review under directive QSL-DIR-2026-07-07-549 (D549), LITE-CEREMONY class. Reads code
and in-repo contracts; changes no source, test, Cargo, or workflow; applies no fix.

## Required Markers

- NA0612_D1220_CONSUMED_OK
- NA0612_D1221_CONSUMED_OK
- NA0612_FRESH_QWORK_PROOF_OK
- NA0612_D1222_ABSENT_BEFORE_IMPLEMENTATION_OK
- NA0612_LITE_CEREMONY_CERTIFIED_OK
- NA0612_READ_ONLY_NO_SOURCE_MUTATION_OK
- NA0612_OBSERVATION_CHANNEL_MODEL_APPLIED_OK
- NA0612_REJECT_TAXONOMY_LOCAL_ONLY_OK
- NA0612_RETRY_CAUSE_AGNOSTIC_OK
- NA0612_NO_WIRE_NACK_OK
- NA0612_NO_REMOTE_FAILURE_CAUSE_ORACLE_OK
- NA0612_ENG0006_RESOLVED_OK
- NA0612_ENG0009_DETERMINISTIC_JITTER_FILED_OK
- NA0612_SERVICE_SIDE_SCOPE_NOTE_RECORDED_OK
- NA0612_SUCCESSOR_NA0613_ENG0007_SELECTED_OK
- NA0612_PRIVATE_MATERIAL_SCAN_OK
- NA0612_RESULT_CLASSIFICATION_SELECTED_OK

## Validation Plan (class-only)

1. Verify fresh operator qwork proof (lane NA-0612) and live main health before
   mutation; verify D-1220/D-1221 once and D-1222 absent; READY_COUNT 1 with READY
   NA-0612.
2. Verify the evidence doc applies the observation-channel model and records:
   reject taxonomy local-only (recv_reject/REJECT_ATT_*/REJECT_QSC_HS_*), retry
   cause-agnostic (bounded_retry unit error), no wire NACK, no remote failure-cause
   oracle; ENG-0009 deterministic-jitter note; service-side scope note.
3. Verify the ledger resolves ENG-0006 and files ENG-0009.
4. Confirm scope guard (only docs/governance paths; NO `.rs`/Cargo/workflow/`.claude`);
   confirm no fix applied; run the no-private-material scan.
5. Restore NA-0613 (ENG-0007 attachment-plane metadata feasibility) as the sole READY
   successor.

## Result

`ERROR_RETRY_DISTINGUISHABILITY_LOCAL_ONLY_NO_REMOTE_ORACLE`. Evidence:
`docs/governance/evidence/NA-0612_error_retry_normalization_review_harness.md`.
