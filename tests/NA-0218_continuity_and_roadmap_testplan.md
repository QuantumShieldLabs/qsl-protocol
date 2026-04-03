Goals: G4, G5

# NA-0218 Continuity and Roadmap Test Plan

## Scope

- validate the docs/governance-only `NA-0218` implementation;
- confirm the new continuity/runbook/roadmap canon is checked in under the allowed paths only; and
- confirm queue closeout remains pending and `NEXT_ACTIONS.md` is untouched.

## Docs-only validation bundle

- local goal-lint using a synthesized `GITHUB_EVENT_PATH`
- markdown inventory counts from the `AGENTS.md` manual runbook
- deterministic local-link existence check from `AGENTS.md`
- changed-path scope proof limited to the approved files

## Reference targets

- `START_HERE.md`
- `AGENTS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/DOC-OPS-001_qbuild_Continuity_and_Disaster_Recovery_Runbook_v0.1.0_DRAFT.md`
- `docs/ops/DOC-OPS-002_Continuity_Snapshot_Manifest_and_Offhost_Procedure_v0.1.0_DRAFT.md`
- `docs/program/DOC-PROG-001_Goal_to_Release_Roadmap_v0.1.0_DRAFT.md`

## Acceptance checkpoints

- the continuity runbook documents qbuild control-plane posture, mirror/worktree topology, authority proof, merge refresh, GitHub-only recovery, host prep or `qstart` expectations, in-flight branch handling, overlay bundles, end-of-day state, and the no-secrets rule
- the snapshot procedure documents minimum contents, cadence, restore steps, and an explicit off-host storage requirement
- the roadmap maps `G1` through `G5` to release gates and current merged workstreams, explains the completed `NA-0217*` wave, and states that `NEXT_ACTIONS.md` remains the execution source of truth
- `AGENTS.md` codifies recoverable-vs-fatal workflow handling without weakening fail-closed scope, security, authority, or destructive-action boundaries
- `DECISIONS.md` and `TRACEABILITY.md` record this as implementation/evidence only, with closeout and queue promotion still pending
