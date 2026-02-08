# NA-0110 — Provenance Light Touch Plan

Status: READY

Scope:
- Governance updates: `NEXT_ACTIONS.md`, `TRACEABILITY.md`, `DECISIONS.md`.
- New root documentation: `NOTICE`, `PROVENANCE.md`, `SIGNED_RELEASES_RUNBOOK.md`.
- No changes under `qsl/**`, `.github/**`, `tools/**`, `apps/**`, `scripts/**`, `formal/**`, `inputs/**`, or `schemas/**`.

Objectives:
1. Add a minimal provenance baseline for users evaluating official project outputs.
2. Point users to authoritative proof lanes and explain lane purpose differences.
3. Provide signed-tag and checksum verification instructions without creating keys in-repo.

Implementation steps:
1. Create NA-0110 governance READY entry and decision.
2. Add root docs:
   - `NOTICE`
   - `PROVENANCE.md`
   - `SIGNED_RELEASES_RUNBOOK.md`
3. Add TRACEABILITY implementation and DONE references after PR creation/merge.
4. Close out NA-0110 and return queue to READY=0.

Verification checklist:
- `rg -n "Status:\\s*READY" NEXT_ACTIONS.md`:
  - Governance phase: exactly one READY (`NA-0110`)
  - Final close-out: no READY entries
- PR scope guard (`gh pr diff <PR> --name-only`) includes only allowed files.
- CI checks for each PR are green before merge.

Executed evidence:
- To be populated in implementation/close-out phases with PR links, merge SHAs, and scope-guard outputs.
