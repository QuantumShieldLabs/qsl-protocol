# Documentation Index

This is the single docs front door for this repository.

## Docs hygiene guardrails
- Policy reference: `AGENTS.md` (Documentation hygiene guardrails section).
- Manual link-integrity runbook: `AGENTS.md` (Manual docs link-integrity check (runbook)).
- Docs PR checklist requirement: use `AGENTS.md` (search for heading `Docs PR checklist (copy/paste into PR body)`).
- Docs audit cadence requirement: use `AGENTS.md` (search for heading `Docs hygiene audit cadence`).
- Docs move protocol: use `AGENTS.md` (search for heading `Docs Move Protocol (Example: Move/Archive a Markdown Doc Safely)`).
- Quarterly spot-audit norm: reviewers request one docs-hygiene evidence snippet on PRs that move/rename docs.
- Front-door rule: use `START_HERE.md` for root onboarding and `docs/INDEX.md` for docs onboarding; avoid adding competing entry docs.
- Quick placement guide:
  - Defines active governance or normative protocol/public commitments -> root governance spine or `docs/canonical/**`.
  - Supports active implementation/operations -> `docs/**` in an existing active folder.
  - Historical/superseded plans or audits -> `docs/archive/**` (test plans under `docs/archive/testplans/**`).
  - Test planning markdown should not accumulate under `tests/`.
- New standalone docs should include the classification header:
  - Status: Authoritative | Supporting | Archive
  - Owner
  - Last-Updated
  - Replaces (optional)
  - Superseded-By (optional)

## Where to start
- Repository start path: `START_HERE.md`
- Documentation start path: `docs/INDEX.md`
- Public/release canonical path: `docs/public/INDEX.md`

## Taxonomy

### Authoritative
Canonical sources that define active governance or normative protocol/public commitments.
- Governance spine: `NEXT_ACTIONS.md`, `DECISIONS.md`, `TRACEABILITY.md`, `GOALS.md`, `PROJECT_CHARTER.md`
- Canonical protocol docs: `docs/canonical/`
- Canonical public/release posture: `docs/public/INDEX.md`
- Required public/legal docs: `SECURITY.md`, `SUPPORT.md`, `CODE_OF_CONDUCT.md`, `THIRD_PARTY_NOTICES.md`

### Supporting
Operational and developer guidance that supports implementation, review, and demos.
- Developer/operator docs: `docs/dev/`
- Supporting public docs and stubs: `docs/public/`
- Checklists and implementation aids: `docs/CHECKLIST_*`, `CHECKLIST_PROTOCOL_CHANGE.md`

### Archive
Historical and superseded materials retained for traceability.
- Archived docs: `docs/archive/`
- Deprecated entry docs (stubs only): `CHAT_STARTER.md`, `README_PHASE4.md`, `QSL_PUBLIC_RELEASE_PLAN.md`, `docs/archive/START_HERE_2.md`

## Key documentation clusters
- Canonical specs: `docs/canonical/`
- Privacy/security posture docs: `docs/privacy/`
- Spec-closure docs: `docs/spec-closure/`
- Schemas and registries: `docs/schemas/`
- Public docs canonical entry: `docs/public/INDEX.md`
- Test/reference docs: `docs/test/`

## Notes
- Keep delivery semantics language truthful: `accepted_by_relay != peer_confirmed`.
- Keep trust terminology consistent: `VERIFIED`, `TRUSTED`, `CHANGED`, `REVOKED`.

- Archived test plans index: `docs/archive/testplans/INDEX.md`
