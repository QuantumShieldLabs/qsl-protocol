Status: Supporting
Owner: qsc
Last-Updated: 2026-04-05
Replaces: tests/NA-0221_policy_scope_repair_v3_testplan.md
Goals: G4, G5

# NA-0221 Policy / Scope Repair v4 Testplan

## Goal

Prove the refreshed-main `NA-0221` queue item truthfully covers the full currently-known protected first-contact TOFU test surface for the qsc Suite-2 fail-closed authenticated-establishment remediation.

## References

- [NEXT_ACTIONS.md](../NEXT_ACTIONS.md)
- [DOC-CAN-003](../docs/canonical/DOC-CAN-003_QSP_Suite-2_True_Triple_Ratchet_v5.0.0_DRAFT.md)
- [DOC-AUD-002](../docs/audit/DOC-AUD-002_qsc_Handshake_Execution_Security_Audit_v0.1.0_DRAFT.md)
- [NA-0221_first_contact_tofu_policy_scope_repair_v4_evidence.md](../docs/archive/testplans/NA-0221_first_contact_tofu_policy_scope_repair_v4_evidence.md)
- [handshake_contract_na0217i.rs](../qsl/qsl-client/qsc/tests/handshake_contract_na0217i.rs)
- [handshake_mvp.rs](../qsl/qsl-client/qsc/tests/handshake_mvp.rs)
- [handshake_security_closure.rs](../qsl/qsl-client/qsc/tests/handshake_security_closure.rs)
- [identity_binding.rs](../qsl/qsl-client/qsc/tests/identity_binding.rs)
- [identity_foundation_contract_na0217d.rs](../qsl/qsl-client/qsc/tests/identity_foundation_contract_na0217d.rs)
- [identity_secret_at_rest.rs](../qsl/qsl-client/qsc/tests/identity_secret_at_rest.rs)
- [send_ready_markers_na0168.rs](../qsl/qsl-client/qsc/tests/send_ready_markers_na0168.rs)
- [receive_e2e.rs](../qsl/qsl-client/qsc/tests/receive_e2e.rs)

## Docs-Only Validation Checkpoints

1. Run local goal-lint with a synthetic PR event whose base is refreshed `origin/main` and whose body includes `Goals: G4, G5`.
2. Run the AGENTS.md markdown inventory commands for `tests/*.md`, `tests/**/*.md`, `docs/*.md`, and `docs/**/*.md`.
3. Run the AGENTS.md manual markdown link-integrity check and record only the summary count.
4. Run an added-line leak-safe scan on the branch diff and confirm `route-path pattern count = 0` and `hex32plus pattern count = 0`.
5. Prove the changed-path set is limited to the five allowed governance/doc paths.

## Acceptance Checkpoints

1. The refreshed `NA-0221` block explicitly lists every currently-known conflicting protected test surface that the next implementation lane must repair, including `send_ready_markers_na0168.rs` and `receive_e2e.rs`.
2. The queue text explicitly states that first-contact TOFU establishment is retired on initiator, responder, legacy identity-migration, send-ready bootstrap, receive/bootstrap, and the already-scoped route-only handshake-canary paths.
3. `DECISIONS.md` records the v4 policy/scope repair and notes that no runtime change occurs in this lane.
4. `TRACEABILITY.md` points to the v4 evidence doc and records the widened protected-test surface truthfully.
5. The broader contradiction scan records no additional directly conflicting out-of-scope tests beyond `send_ready_markers_na0168.rs` and `receive_e2e.rs`.
