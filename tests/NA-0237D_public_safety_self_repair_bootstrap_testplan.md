Status: Supporting
Owner: QSL governance
Last-Updated: 2026-04-23

Goals: G4

# NA-0237D public-safety Self-Repair Bootstrap Testplan

## Docs-only validation checkpoints

- Confirm `NEXT_ACTIONS.md` leaves exactly one `READY` item after the governance repair and that the sole READY item is `NA-0237D`.
- Confirm `NA-0237C` is marked `BLOCKED` with resume pointers to PR `#715`, the dirty local worktree, and the preservation bundle.
- Run local goal-lint with a synthetic pull-request event carrying `Goals: G4`.
- Run the `AGENTS.md` manual markdown link-integrity runbook and record summary counts only.
- Run the markdown inventory commands for `tests/*.md`, `tests/**/*.md`, `docs/*.md`, and `docs/**/*.md`.
- Run an added-line leak-safe scan and confirm `v1-path pattern count=0`, `hex32plus pattern count=0`, and no credential-like markers are introduced.

## References

- Queue repair: `NEXT_ACTIONS.md`
- Decision entry: `DECISIONS.md` D-0429
- Traceability entries: `TRACEABILITY.md` entries for `NA-0237C blocked-on-bootstrap` and `NA-0237D READY`
- Archive evidence: `docs/archive/testplans/NA-0237C_blocked_on_workflow_bootstrap_deadlock_evidence.md`
- Rolling journal entry: `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
