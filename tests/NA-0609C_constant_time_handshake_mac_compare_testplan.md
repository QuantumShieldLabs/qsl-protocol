Goals: G4 (primary), supports G1, G2, G3, G5

Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-07-06

# NA-0609C — Constant-Time Handshake MAC Comparison Test Plan

## Scope

Records validation for the NA-0609C ENG-0003 remediation under directive
QSL-DIR-2026-07-06-544 (D544): a timing-only constant-time hardening of the two
handshake keyed-MAC comparisons in `qsl/qsl-client/qsc/src/handshake/mod.rs`, with
a co-located unit test. No new dependency; no other source; accept/reject
semantics and wire format unchanged.

## Required Markers

- NA0609C_D1212_CONSUMED_OK
- NA0609C_D1213_CONSUMED_OK
- NA0609C_FRESH_QWORK_PROOF_OK
- NA0609C_D1214_ABSENT_BEFORE_IMPLEMENTATION_OK
- NA0609C_CT_HELPER_ADDED_OK
- NA0609C_TWO_MAC_SITES_HARDENED_OK
- NA0609C_UNIT_TEST_EQUIVALENCE_PASS_OK
- NA0609C_HANDSHAKE_SUITES_PASS_OK
- NA0609C_NO_NEW_DEPENDENCY_OK
- NA0609C_SINGLE_SOURCE_FILE_SCOPE_OK
- NA0609C_ENG0003_CLOSED_OK
- NA0609C_WF0004_FILED_OK
- NA0609C_PRIVATE_MATERIAL_SCAN_OK
- NA0609C_RESULT_CLASSIFICATION_SELECTED_OK
- NA0609C_NA0609_SOLE_READY_OK

## Validation Plan (class-only)

1. Verify fresh operator qwork proof and live main health before mutation; verify
   D-1212/D-1213 once and D-1214 absent; READY_COUNT 1 with READY NA-0609.
2. Add `hs_ct_eq_32` and use it at both MAC-comparison sites; add the co-located
   `#[cfg(test)] mod ct_eq_tests`.
3. Run `cargo fmt --check`, `cargo build`, and the new unit tests
   (`handshake::ct_eq_tests`, 4) proving equivalence to `==`.
4. Run the existing handshake suites (`handshake_mvp`,
   `handshake_contract_na0217i`, `handshake_security_closure`,
   `kem_signature_transcript_binding_negative`,
   `handshake_provider_error_no_mutation`) and confirm all pass — accept/reject
   semantics preserved.
5. Run `cargo metadata --locked` and `cargo audit`; confirm Cargo.toml/Cargo.lock
   unchanged (no new dependency) and only `handshake/mod.rs` source changed.
6. Confirm scope guard (only the allowed paths), close ENG-0003 and file WF-0004
   in the ledger, and run the no-private-material scan.

## Result

`CONSTANT_TIME_HANDSHAKE_MAC_COMPARE_HARDENED`. Evidence:
`docs/governance/evidence/NA-0609C_constant_time_handshake_mac_compare_harness.md`.
