Status: Supporting
Owner: qsl-protocol maintainers
Last-Updated: 2026-04-05

# NA-0221 Policy / Scope Repair v2 Test Plan

Goals: G4, G5

## Goal

- Repair `NA-0221` queue truth so the next implementation attempt can update both conflicting protected first-contact TOFU expectations without policy ambiguity.

## References

- `NEXT_ACTIONS.md` `NA-0221 — Handshake Authenticated-Establishment Fail-Closed Remediation`
- `docs/canonical/DOC-CAN-003_QSP_Suite-2_True_Triple_Ratchet_v5.0.0_DRAFT.md`
- `docs/audit/DOC-AUD-002_qsc_Handshake_Execution_Security_Audit_v0.1.0_DRAFT.md`
- `qsl/qsl-client/qsc/tests/identity_binding.rs`
- `qsl/qsl-client/qsc/tests/identity_foundation_contract_na0217d.rs`

## Docs-Only Validation Checkpoints

- Run local goal-lint using a synthesized pull-request event payload with actual base/head SHAs.
- Run the markdown inventory commands from `AGENTS.md` for `tests/*.md`, `tests/**/*.md`, `docs/*.md`, and `docs/**/*.md`.
- Run the manual markdown link-integrity runbook from `AGENTS.md`.
- Run an added-line leak-safe scan on the allowed changed paths.

## Acceptance Checkpoints

- `NEXT_ACTIONS.md` keeps `NA-0221` as the sole `READY` item and explicitly brings `qsl/qsl-client/qsc/tests/identity_foundation_contract_na0217d.rs` into scope.
- The `NA-0221` Problem / Deliverables / Acceptance text states that first-contact TOFU establishment is retired on both initiator and responder sides for this Suite-2 path.
- `DECISIONS.md` appends `D-0375` recording this governance-only v2 policy/scope repair.
- `TRACEABILITY.md` adds a `NA-0221 policy/scope repair v2` changelog entry that points to the archive evidence doc.
- No runtime files change.
