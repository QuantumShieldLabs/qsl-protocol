Status: Archive
Owner: QSL governance
Last-Updated: 2026-04-07

# NA-0224 qsc Modularization / File-Size Reduction Plan Refresh Evidence

Goals: G4, G5

## Scope

- Repo: `qsl-protocol`
- Lane: `NA-0224`
- Posture: implementation already merged; this closeout is governance-only and introduces no runtime changes
- Implementation PR: #669
- Implementation branch head before merge: `91833425716b`
- Implementation merge SHA: `59b3fba32794`
- Implementation mergedAt: `2026-04-07T00:49:12Z`

## Authority Proof

- refreshed `qsl-protocol` checkout `HEAD`, `main`, `mirror/main`, and `origin/main` all resolved to `59b3fba32794`
- refreshed merged main contains `DECISIONS.md` `D-0384`, the `TRACEABILITY.md` `NA-0224 implementation/evidence` entry, the refreshed `docs/design/DOC-QSC-011_qsc_Modularization_and_File_Size_Reduction_Plan_v0.1.0_DRAFT.md` plan artifact, and the merged `tests/NA-0224_qsc_modularization_plan_refresh_testplan.md` surface from PR #669
- refreshed live queue still showed `READY_COUNT=1` with `NA-0224` as the sole live `READY` item before this closeout
- refreshed sibling-repo proof stayed read-only and truthful: `qsl-server READY=0`, `qsl-attachments READY=0`
- PR `#669` is already `MERGED`; refreshed merged `main` is authoritative for queue truth, and this closeout lane does not mutate that PR

## Implementation Evidence Source On Main

- `DECISIONS.md` `D-0384`
- `TRACEABILITY.md` `NA-0224 implementation/evidence`
- `docs/design/DOC-QSC-011_qsc_Modularization_and_File_Size_Reduction_Plan_v0.1.0_DRAFT.md`
- `tests/NA-0224_qsc_modularization_plan_refresh_testplan.md`

## Exact Implementation Summary

- refreshed merged main proves `qsl/qsl-client/qsc/src/main.rs` is no longer the dominant concentration
- refreshed plan proves `qsl/qsl-client/qsc/src/tui/controller.rs` is now the dominant audit-radius / maintainability concentration
- refreshed plan names the next bounded extraction lane against current merged truth as `NA-0225 — qsc TUI Controller State / Command-Flow Decomposition`

## Acceptance-Proof Surface

- the refreshed plan is based on current merged main rather than stale pre-adversarial assumptions
- the plan preserves the current CLI/TUI, sidecar, marker, and honest-delivery contracts
- no runtime, workflow, server, or attachment-service semantics changed

## Implementation / CI Nuance Summary

- the implementation landed on PR #669 from refreshed `main`
- no runtime surfaces were changed
- protected CI completed green before merge
- this closeout PR is governance-only and introduces no runtime changes

## Closeout Note

- `NA-0224` is now closed truthfully because its merged implementation/evidence state is already durable on refreshed `main`
- the next truthful successor is `NA-0225 — qsc TUI Controller State / Command-Flow Decomposition`
