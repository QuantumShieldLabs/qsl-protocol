Goals: G5
Status: DRAFT

Scope:
- Governance-only ordering for public-release prep.

Objective:
- Ensure the runbook is the canonical ordering for scrub/cutover.
- Enforce one active directive at a time.

CI-gated assertions:
- goal-lint passes for runbook and plan updates.
- Freeze mode remains in effect (no READY items promoted).

Manual verification:
- Runbook marks Step 1 as CURRENT.
- QSL_PUBLIC_RELEASE_PLAN.md links to docs/public/PUBLIC_RELEASE_RUNBOOK.md.
