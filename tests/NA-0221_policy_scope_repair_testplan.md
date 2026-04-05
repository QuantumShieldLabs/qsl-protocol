Goals: G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-04-05

# NA-0221 Policy / Scope Repair Test Plan

## Scope

- validate the docs/governance-only `NA-0221` policy/scope repair lane;
- confirm the live `NA-0221` queue item stays `READY` and remains the sole active item; and
- confirm the repaired lane truthfully includes the conflicting protected responder-side identity-binding test surface without introducing runtime changes.

## Docs-only validation checkpoints

- local goal-lint using a synthesized `pull_request` event payload
- markdown inventory counts from the `AGENTS.md` manual runbook:
  - `git ls-files 'tests/*.md'`
  - `git ls-files 'tests/**/*.md'`
  - `git ls-files 'docs/*.md'`
  - `git ls-files 'docs/**/*.md'`
- deterministic local-link existence check from the `AGENTS.md` manual runbook
- changed-path scope proof limited to:
  - `NEXT_ACTIONS.md`
  - `DECISIONS.md`
  - `TRACEABILITY.md`
  - `docs/archive/testplans/NA-0221_authenticated_establishment_policy_scope_repair_evidence.md`
  - `tests/NA-0221_policy_scope_repair_testplan.md`
- added-line leak-safe scan confirming no secret-like values were introduced

## Reference targets

- [NEXT_ACTIONS.md](../NEXT_ACTIONS.md)
- [DECISIONS.md](../DECISIONS.md)
- [TRACEABILITY.md](../TRACEABILITY.md)
- [DOC-CAN-003](../docs/canonical/DOC-CAN-003_QSP_Suite-2_True_Triple_Ratchet_v5.0.0_DRAFT.md)
- [DOC-AUD-002](../docs/audit/DOC-AUD-002_qsc_Handshake_Execution_Security_Audit_v0.1.0_DRAFT.md)
- [identity_binding.rs](../qsl/qsl-client/qsc/tests/identity_binding.rs)
- [NA-0221 policy/scope repair evidence](../docs/archive/testplans/NA-0221_authenticated_establishment_policy_scope_repair_evidence.md)

## Acceptance checkpoints

- the repaired `NA-0221` block explicitly states that first-contact TOFU establishment is no longer allowed on this Suite-2 handshake path
- the repaired `NA-0221` block explicitly includes `qsl/qsl-client/qsc/tests/identity_binding.rs` in scope
- `DECISIONS.md` records that fail-closed authenticated establishment outranks responder-side first-contact TOFU for this lane and that the repair is governance-only
- `TRACEABILITY.md` points to the new archive evidence doc and the new test-plan stub without claiming closeout
- the archive evidence doc records the canonical requirement, the `P1` audit finding, and the conflicting protected test expectation from `identity_binding.rs`
- no qsc runtime paths, runtime tests, qsc-desktop paths, sibling repos, workflow files, or cargo manifests change in this lane
