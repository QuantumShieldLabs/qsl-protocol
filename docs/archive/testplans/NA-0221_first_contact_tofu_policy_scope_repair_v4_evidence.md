Status: Archive
Owner: qsc
Last-Updated: 2026-04-05
Replaces: docs/archive/testplans/NA-0221_first_contact_tofu_policy_scope_repair_v3_evidence.md
Goals: G4, G5

# NA-0221 First-Contact TOFU Policy / Scope Repair v4 Evidence

## Purpose

Record the refreshed-main contradiction proof showing that `NA-0221` must cover the full currently-known protected first-contact TOFU test surface for the qsc Suite-2 handshake lane before the next implementation attempt begins.

## Canonical Authenticated-Establishment Requirement

- [DOC-CAN-003](../../canonical/DOC-CAN-003_QSP_Suite-2_True_Triple_Ratchet_v5.0.0_DRAFT.md) says Suite-2 state is not allowed to commit until authenticated peer identity exists at the system layer, and missing authenticated commitment must fail closed before any state commit.

## Audit P1 Requirement

- [DOC-AUD-002](../../audit/DOC-AUD-002_qsc_Handshake_Execution_Security_Audit_v0.1.0_DRAFT.md) records `NA0220-F001` / `P1`: current qsc accept paths still write durable pending/session state for `identity_unknown`, and the minimal fix direction is to reject before `hs_pending_store(...)` or `qsp_session_store(...)`.

## Conflicting Protected-Test Expectations On Refreshed Main

- [identity_binding.rs](../../../qsl/qsl-client/qsc/tests/identity_binding.rs) still sets only route tokens, drives a full first-contact handshake, asserts success, and expects `identity_unknown` without a silent pin.
- [identity_foundation_contract_na0217d.rs](../../../qsl/qsl-client/qsc/tests/identity_foundation_contract_na0217d.rs) still lets Alice start establishment with only Bob's route token while Bob pins Alice by verification code, then asserts a no-reject handshake contract.
- [identity_secret_at_rest.rs](../../../qsl/qsl-client/qsc/tests/identity_secret_at_rest.rs) still imports a legacy identity, sets only Bob's route token, and asserts `handshake init` success during first-contact legacy migration.
- [send_ready_markers_na0168.rs](../../../qsl/qsl-client/qsc/tests/send_ready_markers_na0168.rs) still performs a route-only bootstrap handshake, asserts success through confirmation, and only pins both peers after establishment to check send-ready markers.
- [receive_e2e.rs](../../../qsl/qsl-client/qsc/tests/receive_e2e.rs) still performs a route-only mailbox peer-separation handshake and asserts success before receive-path checks continue.
- The broader scan also confirmed already-scoped route-only handshake establishment assumptions in:
  - [handshake_contract_na0217i.rs](../../../qsl/qsl-client/qsc/tests/handshake_contract_na0217i.rs)
  - [handshake_mvp.rs](../../../qsl/qsl-client/qsc/tests/handshake_mvp.rs)
  - [handshake_security_closure.rs](../../../qsl/qsl-client/qsc/tests/handshake_security_closure.rs)
- No additional directly conflicting out-of-scope test was found beyond `send_ready_markers_na0168.rs` and `receive_e2e.rs`.

## Policy Decision

- For `NA-0221`, first-contact TOFU establishment in qsc Suite-2 is retired on initiator, responder, legacy identity-migration, send-ready bootstrap, receive/bootstrap, and the already-scoped route-only handshake-canary paths.
- Fail-closed authenticated establishment takes precedence over preserving first-contact TOFU convenience on any of those paths.
- The live [NEXT_ACTIONS.md](../../../NEXT_ACTIONS.md) block therefore needs explicit scope and acceptance text covering every conflicting protected test surface listed above.

## Scope-Repair-Only Note

- This directive repairs queue truth, decision history, and traceability only.
- It introduces no runtime or test-semantic changes in `qsl/qsl-client/qsc/src/**` or `qsl/qsl-client/qsc/tests/**`.
