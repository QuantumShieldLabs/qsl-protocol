Status: Supporting
Owner: QSL governance
Last-Updated: 2026-04-23

# NA-0237C public-safety Main-Red Recursion Repair Testplan

Goals: G4

## Docs-only Validation Checkpoints

- `NEXT_ACTIONS.md` marks `NA-0237B` as `BLOCKED` on `public-safety` main-red recursion and adds the resume note pointing to PR `#713`, the local implementation worktree, and the preservation bundle.
- `NEXT_ACTIONS.md` promotes `NA-0237C — public-safety Main-Red Recursion Repair` as the sole `READY` item using the exact approved successor block.
- `docs/archive/testplans/NA-0237B_blocked_on_public_safety_main_red_recursion_evidence.md` records the exact PR-head blocker truth, the main-side failing advisory/public-safety truth, the bounded PR changed-path proof, and the preservation/resume proof.
- `DECISIONS.md` records `D-0428` and states that `NA-0237B` is now blocked on `public-safety` recursion rather than on remaining dependency ambiguity.
- `TRACEABILITY.md` contains both the `NA-0237B blocked-on-recursion` entry and the `NA-0237C READY` entry.
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md` contains the matching Directive 349 entry.
- Local goal-lint passes via the accepted synthetic-event path with governance PR metadata.
- The markdown inventory commands and manual link-integrity runbook from `AGENTS.md` pass.
- The added-line leak-safe scan reports zero secret-like markers.

## References

- `NEXT_ACTIONS.md`
- `docs/archive/testplans/NA-0237B_blocked_on_public_safety_main_red_recursion_evidence.md`
- `DECISIONS.md` (`D-0428`)
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
