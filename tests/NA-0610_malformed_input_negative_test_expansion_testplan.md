Goals: G4 (primary), supports G1, G2, G3, G5

Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-07-07

# NA-0610 — Malformed Input Negative-Test Expansion Test Plan

## Scope

Records validation for the NA-0610 malformed attachment-descriptor/confirm
negative-test expansion under directive QSL-DIR-2026-07-07-547 (D547). Test-only: a
co-located `#[cfg(test)]` module in `qsl/qsl-client/qsc/src/adversarial/payload.rs`;
no production semantic change; no new dependency.

## Required Markers

- NA0610_D1217_CONSUMED_OK
- NA0610_D1218_CONSUMED_OK
- NA0610_FRESH_QWORK_PROOF_OK
- NA0610_D1219_ABSENT_BEFORE_IMPLEMENTATION_OK
- NA0610_DESCRIPTOR_NEGATIVES_ADDED_OK
- NA0610_SANITY_TEMPLATE_PARSES_OK
- NA0610_ALL_MALFORMED_REJECT_FAIL_CLOSED_OK
- NA0610_NO_PRODUCTION_SEMANTIC_CHANGE_OK
- NA0610_NO_NEW_DEPENDENCY_OK
- NA0610_TEST_ONLY_SINGLE_SOURCE_FILE_OK
- NA0610_REGRESSION_SUITES_PASS_OK
- NA0610_NA0608_DESCRIPTOR_HEDGE_CLOSED_OK
- NA0610_PRIVATE_MATERIAL_SCAN_OK
- NA0610_RESULT_CLASSIFICATION_SELECTED_OK

## Validation Plan (class-only)

1. Verify fresh operator qwork proof (lane NA-0610) and live main health before
   mutation; verify D-1217/D-1218 once and D-1219 absent; READY_COUNT 1 with READY
   NA-0610.
2. Add descriptor/confirm malformed negatives to the payload test module; assert the
   well-formed template parses (sanity) and every malformed input rejects (None).
3. Run `cargo fmt --check`, `cargo build`, and the payload negative tests
   (`payload::tests`, 10 pass).
4. Run the regression suites (`adversarial_properties`, `attachments_contract_na0217h`,
   `receive_no_mutation`) and all qsc unit tests; confirm no regression.
5. Run `cargo metadata --locked`; confirm Cargo.toml/Cargo.lock unchanged (no new
   dependency) and only `payload.rs` source changed.
6. Confirm scope guard and run the no-private-material scan; confirm no production
   semantic change and no malformed input accepted.

## Result

`MALFORMED_ATTACHMENT_DESCRIPTOR_NEGATIVES_PINNED`. Evidence:
`docs/governance/evidence/NA-0610_malformed_input_negative_test_expansion_harness.md`.
