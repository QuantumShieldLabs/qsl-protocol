Status: Archive
Owner: QSL governance
Last-Updated: 2026-04-05

# NA-0221 Authenticated-Establishment Policy / Scope Repair Evidence

Goals: G4, G5

## Scope

- Repo: `qsl-protocol`
- Lane: `NA-0221`
- Posture: governance-only queue/policy/scope repair
- Runtime changes: none
- Stale/open implementation work left untouched: PR `#657`

## Canonical authenticated-establishment requirement

- Current `main` canonical text in [DOC-CAN-003](../../canonical/DOC-CAN-003_QSP_Suite-2_True_Triple_Ratchet_v5.0.0_DRAFT.md) states that authenticated peer identity must exist before Suite-2 state is committed.
- The same canonical section requires the base handshake transcript to commit to the Suite-2 negotiation tuple and `session_id`, and to reject establishment if that authenticated commitment cannot be provided.

## Audit `P1` contradiction proof

- Current `main` audit text in [DOC-AUD-002](../../audit/DOC-AUD-002_qsc_Handshake_Execution_Security_Audit_v0.1.0_DRAFT.md) records `NA0220-F001` / `P1`: qsc accept paths still proceed on `identity_unknown` and can write durable pending/session state before the authenticated-establishment boundary is satisfied.
- The same audit fixes the minimal remediation direction: reject before `hs_pending_store(...)` or `qsp_session_store(...)` when authenticated peer identity is absent, stop overstating the authenticated base-handshake claim, and add direct negative regressions.

## Conflicting protected test expectation

- The protected test surface in [identity_binding.rs](../../../qsl/qsl-client/qsc/tests/identity_binding.rs) still carries responder-side first-contact TOFU expectations that conflict with the merged canonical/audit posture.
- `tofu_pins_on_first_handshake()` drives a route-only first-contact handshake and currently expects success while the combined output still contains `identity_unknown`.
- `handshake_accepts_verification_code_pin_without_peer_mismatch()` also remains part of the protected responder-side identity-binding surface that the next implementation lane must align to the fail-closed policy without widening beyond the approved handshake seam.

## Policy choice frozen for `NA-0221`

- For `NA-0221`, first-contact TOFU establishment in qsc Suite-2 accept paths is retired.
- Fail-closed authenticated establishment outranks preserving responder-side first-contact TOFU convenience on this lane.
- `qsl/qsl-client/qsc/tests/identity_binding.rs` is now explicitly in scope so the next implementation directive can update the directly conflicting protected tests truthfully.

## Governance-only note

- This lane repairs queue truth, scope truth, and supporting evidence only.
- It introduces no runtime changes and does not merge, close, or edit PR `#657`.
- The next implementation attempt must still prove the runtime remediation and protected test updates on a separate lane.
