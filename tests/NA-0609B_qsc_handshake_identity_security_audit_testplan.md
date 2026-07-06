Goals: G4 (primary), supports G1, G2, G3, G5

Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-07-06

# NA-0609B — qsc Handshake and Identity Read-Only Security Audit Test Plan

## Scope

Records class-safe audit markers for the NA-0609B read-only handshake/identity
security audit under directive QSL-DIR-2026-07-06-543 (D543), LITE-CEREMONY class.
This lane reads security-critical code and changes no source, test, crypto,
dependency, lockfile, or workflow, and applies no fix; every concrete finding is
routed to a separate full-ritual remediation lane via the improvement ledger.

## Required Markers

- NA0609B_D1211_CONSUMED_OK
- NA0609B_D1212_CONSUMED_OK
- NA0609B_FRESH_QWORK_PROOF_OK
- NA0609B_CURRENT_MAIN_HEALTH_OK
- NA0609B_D1213_ABSENT_BEFORE_IMPLEMENTATION_OK
- NA0609B_LITE_CEREMONY_CERTIFIED_OK
- NA0609B_READ_ONLY_NO_SOURCE_MUTATION_OK
- NA0609B_TRANSCRIPT_BINDING_VERIFIED_SOUND_OK
- NA0609B_NO_MUTATION_ON_REJECT_VERIFIED_SOUND_OK
- NA0609B_IDENTITY_DUAL_PIN_MODEL_VERIFIED_SOUND_OK
- NA0609B_ENG0001_RESOLVED_OK
- NA0609B_ENG0003_NONCT_MAC_RECORDED_OK
- NA0609B_ENG0004_DIR_FSYNC_NOOP_RECORDED_OK
- NA0609B_NO_P0_P1_ESCALATION_OK
- NA0609B_LEDGER_UPDATED_OK
- NA0609B_PRIVATE_MATERIAL_SCAN_OK
- NA0609B_RESULT_CLASSIFICATION_SELECTED_OK
- NA0609B_NA0609_SOLE_READY_OK

## Validation Plan (class-only)

1. Verify fresh operator qwork proof and live main health before mutation; verify
   D-1211/D-1212 once and D-1213 absent; READY_COUNT 1 with READY NA-0609.
2. Verify the audit evidence doc records: verified-sound items (transcript binding,
   hybrid handshake + all-zero DH guard, ML-DSA fail-closed verify, downgrade/
   suite-context binding, dual-pin identity model, no-mutation-on-reject, atomic
   file write, replay guard) and the ranked findings (ENG-0003, ENG-0004, ENG-0001
   resolved) in the DOC-AUD-001 §6 schema.
3. Verify the ledger reflects ENG-0001 resolution and the new ENG-0003/ENG-0004.
4. Confirm scope guard (only docs/governance paths changed; NO `.rs`/Cargo/workflow/
   `.claude`); confirm no fix was applied; run the no-private-material scan.
5. Confirm no P0/P1 escalation was required.

## Result

`QSC_HANDSHAKE_IDENTITY_AUDIT_COMPLETE_NO_P0_P1_THREE_P3_HARDENING`.
Evidence: `docs/governance/evidence/NA-0609B_qsc_handshake_identity_security_audit_harness.md`.
