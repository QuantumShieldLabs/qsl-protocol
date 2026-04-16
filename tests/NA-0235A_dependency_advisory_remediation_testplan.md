Goals: G4

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-04-16

# NA-0235A Dependency Advisory Remediation Testplan

## Purpose

This companion stub records the docs-only validation and traceability hooks for the governance lane that marks `NA-0235` blocked and promotes `NA-0235A` as the sole READY successor.

## Validation checkpoints

- `docs/archive/testplans/NA-0235_blocked_on_dependency_advisories_evidence.md` exists and records truthful blocker proof for PR `#695`.
- `NEXT_ACTIONS.md` marks `NA-0235` `BLOCKED`, adds the resume note that points back to PR `#695`, and promotes exactly one successor READY block: `NA-0235A`.
- `DECISIONS.md` appends `D-0410` and records that the live blocker is dependency advisories rather than queue ambiguity or CI wiring.
- `TRACEABILITY.md` records one `NA-0235 blocked-on-dependencies` entry and one `NA-0235A READY` entry.
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md` contains the `DIRECTIVE 296` entry for this governance-only queue-repair lane.

## Acceptance checkpoints

- The queue repair is truthful: `NA-0235` is no longer `READY`, `NA-0235A` is the sole `READY` item, and PR `#695` remains open and untouched.
- The validation evidence is docs-only and does not introduce runtime, workflow, branch-protection, or dependency-file changes.
