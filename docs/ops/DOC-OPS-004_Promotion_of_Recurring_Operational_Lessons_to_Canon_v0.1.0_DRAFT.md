Goals: G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-04-04

# DOC-OPS-004 — Promotion of Recurring Operational Lessons to Canon v0.1.0 DRAFT

## 1. Purpose and boundary

This document defines when operational lessons captured in the rolling journal must be promoted into durable repo canon.

The rolling journal is the first stop for per-directive operational memory. It is not the permanent home for stable rules that future directives need in order to execute truthfully.

## 2. Promotion criteria

Promote a lesson out of the rolling journal when any of the following is true:

- the same lesson or workaround recurs across multiple directives;
- the lesson is required to prove authority, scope, or validation truthfully;
- the lesson defines a stable recoverable-vs-fatal operating rule;
- the lesson materially improves continuity, safety, or auditability for future operators; or
- leaving the lesson journal-only would force future work to depend on operator memory instead of checked-in guidance.

Do not leave stable recurring lessons in the journal indefinitely.

## 3. Destination selection

Promote the lesson to the smallest truthful canonical location:

- `START_HERE.md` for repo-wide workflow, authority order, or entrypoint rules;
- `AGENTS.md` for assistant operating behavior, recoverable-failure handling, scope guards, validation expectations, or documentation hygiene;
- `DECISIONS.md` when the lesson freezes a lasting policy or invariant that future work must preserve;
- `TRACEABILITY.md` when the lesson changes how goals, docs, tests, or evidence map together; and
- supporting `docs/**` when the lesson is operational procedure or strategic guidance that should stay subordinate to the governance spine.

If a lesson would affect queue order, protocol behavior, crypto semantics, or runtime behavior, it is not a journal-promotion-only change; govern it explicitly in the appropriate lane.

## 4. Promotion timing rule

Once a lesson is clearly recurring or load-bearing, promote it in the next truthful in-scope governance lane.

Do not wait for a perfect summary or a later memory dump if the rule is already needed now.

When promoted, reference the originating directive or journal context in the governing evidence so reviewers can see why the rule was added.

## 5. Examples

Examples that may remain journal-only:

- one transient GitHub API outage;
- one stale local cache incident that did not recur;
- one PR-specific check-run delay;
- one host-specific disk observation that did not affect later directives.

Examples that must become canon:

- a recurring requirement to record recoverable failures continuously instead of reconstructing them later;
- a repeated need to publish continuity-sensitive state off-host after the first green local validation bundle;
- a recurring secret-safe wording rule for governance evidence;
- a stable audit-planning rule that future directives will keep using; or
- a recurring authority-proof or scope-guard step that future operators need to execute correctly.

## 6. Non-authority reminder

Promoting a lesson into canon makes the rule explicit and durable. It does not let the rolling journal outrank the governance spine, and it does not let supporting docs override `NEXT_ACTIONS.md`.
