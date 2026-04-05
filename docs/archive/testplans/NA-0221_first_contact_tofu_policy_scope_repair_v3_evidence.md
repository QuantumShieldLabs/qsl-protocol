Status: Archive
Owner: qsc
Last-Updated: 2026-04-05
Replaces: docs/archive/testplans/NA-0221_first_contact_tofu_policy_scope_repair_v2_evidence.md

# NA-0221 First-Contact TOFU Policy / Scope Repair v3 Evidence

## Purpose

This evidence records the remaining policy contradiction on refreshed `main` for `NA-0221`. It is queue/scope repair only and introduces no runtime changes.

## Canonical Authenticated-Establishment Requirement

- [DOC-CAN-003](../../canonical/DOC-CAN-003_QSP_Suite-2_True_Triple_Ratchet_v5.0.0_DRAFT.md) states that Suite-2 assumes authenticated peer identity exists before Suite-2 state is committed.
- The same canonical document requires reject-on-missing-authenticated-commitment via `REJECT_S2_ESTABLISH_UNAUTHENTICATED`.

## Audit P1 Requirement

- [DOC-AUD-002](../../audit/DOC-AUD-002_qsc_Handshake_Execution_Security_Audit_v0.1.0_DRAFT.md) records `NA0220-F001` / `P1`: qsc accept paths can still proceed on `identity_unknown` and write durable pending/session state.
- The audit remediation direction is explicit: reject before any `hs_pending_store(...)` or `qsp_session_store(...)` when authenticated peer identity is absent.

## Conflicting Protected-Test Expectations On Refreshed Main

- [identity_binding.rs](../../../qsl/qsl-client/qsc/tests/identity_binding.rs) still contains `tofu_pins_on_first_handshake`, which seeds only route tokens and still expects first-contact establishment success while surfacing `identity_unknown`.
- [identity_foundation_contract_na0217d.rs](../../../qsl/qsl-client/qsc/tests/identity_foundation_contract_na0217d.rs) still contains `verification_code_pin_preserves_handshake_contract`, where Alice only has route-level knowledge for Bob before `handshake init` and the full handshake is still expected to succeed without `handshake_reject`.
- [identity_secret_at_rest.rs](../../../qsl/qsl-client/qsc/tests/identity_secret_at_rest.rs) still contains `migrate_legacy_identity_into_vault`, which seeds only a route token for Bob and still asserts that `handshake init` succeeds while the legacy identity secret is migrated.

## Policy Decision

- For `NA-0221`, first-contact TOFU establishment in qsc Suite-2 is retired on initiator, responder, and legacy identity-migration paths.
- Fail-closed authenticated establishment takes precedence over preserving legacy TOFU convenience on those paths.
- The live [NEXT_ACTIONS.md](../../../NEXT_ACTIONS.md) block must therefore include all three conflicting protected-test surfaces in scope so the next implementation lane can update them truthfully.

## Scope-Repair-Only Note

- This governance repair does not alter qsc runtime behavior, protocol semantics, transport, service boundaries, qsc-desktop behavior, qsl-server, or qsl-attachments.
- PR #660 remains read-only context for contradiction discovery only; it is not mutated by this governance lane.
