Goals: G4

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-04-19

# NA-0236 — KT Serialization/Profile + BundleTBS / Bundle-Signature Canon Closure Testplan

## Scope

This is the required docs-only/governance stub for `NA-0236`.

The lane is limited to canonical KT serialization/profile closure, supporting schema/spec-closure alignment, governance updates, and the rolling journal entry. No runtime, workflow, or queue-closeout edits are part of this item.

## Required artifacts

- `docs/canonical/DOC-CAN-008_QSP_Key_Transparency_Profile_and_Bundle_Signature_Closure_v0.1.0_DRAFT.md`
- `docs/schemas/DOC-SCL-002_Shared_Schemas_v1.0.json`
- `docs/spec-closure/DOC-SCL-001_Suite_Parameter_Registry_Deployment_Profiles_v1.0_DRAFT.md`
- `docs/spec-closure/DOC-SCL-002_Shared_Schemas_Error_Reason_Code_Registry_v1.0_DRAFT.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Local validation

- Run local goal-lint against the docs/governance PR body.
- Run the markdown inventory commands from `AGENTS.md`.
- Run the manual markdown link-integrity runbook from `AGENTS.md`.
- Run an added-line leak-safe scan proving no secret-like values were introduced.
- Run changed-path scope proof before push and again from the opened PR.

## Expected outcome

- `QSP-4.3.2-KT1` is frozen as the canonical KT-enabled bundle profile.
- `BundleLeafData` and `BundleTBS` are separated so KT proofs do not recurse into the leaf hash.
- `kt_log_id`, `kt_sth`, `kt_inclusion_proof`, and `kt_consistency_proof` have exact meanings for the later implementation lane.
- All-zero / empty KT remains forbidden outside explicit non-production disabled mode.
- Queue closeout remains deferred, so `NEXT_ACTIONS.md` is unchanged and `NA-0236` remains the sole READY item after merge.
