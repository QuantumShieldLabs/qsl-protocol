Status: Supporting
Owner: qsc
Last-Updated: 2026-04-05
Replaces: tests/NA-0221_policy_scope_repair_v2_testplan.md

# NA-0221 Policy Scope Repair v3 Test Plan

Goals: G4, G5

## Goal

Validate that the `NA-0221` queue item truthfully matches merged canonical/audit posture and the full protected first-contact TOFU test surface on refreshed `main`, without introducing runtime changes.

## References

- Queue item: [NEXT_ACTIONS.md](../NEXT_ACTIONS.md)
- Canonical requirement: [DOC-CAN-003](../docs/canonical/DOC-CAN-003_QSP_Suite-2_True_Triple_Ratchet_v5.0.0_DRAFT.md)
- Audit requirement: [DOC-AUD-002](../docs/audit/DOC-AUD-002_qsc_Handshake_Execution_Security_Audit_v0.1.0_DRAFT.md)
- Conflict surface 1: [identity_binding.rs](../qsl/qsl-client/qsc/tests/identity_binding.rs)
- Conflict surface 2: [identity_foundation_contract_na0217d.rs](../qsl/qsl-client/qsc/tests/identity_foundation_contract_na0217d.rs)
- Conflict surface 3: [identity_secret_at_rest.rs](../qsl/qsl-client/qsc/tests/identity_secret_at_rest.rs)
- Evidence: [NA-0221_first_contact_tofu_policy_scope_repair_v3_evidence.md](../docs/archive/testplans/NA-0221_first_contact_tofu_policy_scope_repair_v3_evidence.md)

## Docs-Only Validation Checkpoints

- Run local goal-lint against a synthetic `pull_request` event with truthful base/head SHAs and a PR body that includes `Goals: G4, G5`.
- Run the manual markdown inventory commands from `AGENTS.md`.
- Run the manual markdown link-integrity runbook from `AGENTS.md`.
- Run an added-line leak-safe scan and report only summary counts for the route-path pattern and `hex32plus pattern`.

## Acceptance Checkpoints

- `NA-0221` remains the sole `READY` item and its live block explicitly includes `identity_secret_at_rest.rs` in scope.
- The repaired queue text explicitly states that first-contact TOFU establishment is no longer allowed on initiator, responder, or legacy identity-migration paths.
- `DECISIONS.md` records the v3 policy/scope repair as governance-only and not a runtime fix.
- `TRACEABILITY.md` records the v3 policy/scope repair and points to the new archive evidence doc.
- The PR diff is limited to the five governance-only paths allowed by Directive 250.
