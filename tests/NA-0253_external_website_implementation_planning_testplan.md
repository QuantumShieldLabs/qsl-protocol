Goals: G1, G3, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-07
Replaces: n/a
Superseded-By: n/a

# NA-0253 External Website Implementation Planning Testplan

## Objective

Validate that NA-0253 creates an operator-ready external website implementation
planning package in qsl-protocol without editing website implementation source
or the external website repository.

## Protected Invariant

Website implementation remains evidence-bound and future-scoped. This lane must
not claim production readiness, a proven true Triple Ratchet, quantum-proof
security, anonymity, metadata elimination, external review completion, or
external product proof of QSL protocol release readiness.

## Scope Guard

Allowed changed paths for the implementation-planning PR:

- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/public/EXTERNAL_WEBSITE_IMPLEMENTATION_DIRECTIVE.md`
- `docs/governance/evidence/NA-0253_external_website_implementation_planning_audit.md`
- `tests/NA-0253_external_website_implementation_planning_testplan.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Forbidden changed paths include `.github/**`, `scripts/**`, Cargo manifests or
lockfiles, qsc/qsl apps, tools, inputs, formal models, qsc-desktop, qsl-server,
qsl-attachments, website implementation files, public-safety configuration, and
any external website repository.

## Directive Package Validation

- Confirm the directive contains a Goals line.
- Confirm it states planning only and no external website repo edit.
- Confirm it records external website repo prerequisites, local clone/worktree
  requirement, branch naming, no-production-deployment rule, backup/snapshot,
  and rollback requirement.
- Confirm it has page-by-page tasks for homepage, protocol status, demo/GUI,
  metadata/privacy, Suite-2 / Triple-Ratchet wording, external product
  separation, healthcare/PQC consulting, evidence links, and roadmap/release
  readiness.
- Confirm it includes safe copy snippets, prohibited copy snippets, a static
  overclaim scan list, verification checklist, rollback checklist, future
  external website directive template, stop conditions, and known uncertainties.

## Evidence Map Validation

- Confirm directive and audit link to GOALS, ROADMAP, DECISIONS, TRACEABILITY,
  WEBSITE_IMPLEMENTATION_HANDOFF, WEBSITE_CLAIM_MATRIX, WEBSITE_UPDATE_PLAN,
  SUITE2_TRIPLE_RATCHET_CLAIM_BOUNDARY, EXTERNAL_REVIEW_PACKAGE,
  RELEASE_READINESS_EVIDENCE_MAP, DEMO_ACCEPTANCE_CRITERIA,
  CONFORMANCE_VECTOR_PRIORITIZATION, DOC-G5-001, DOC-G5-003, and NA-0251 audit
  evidence.
- Confirm safe interpretations do not convert supporting evidence into
  production release approval.

## No Website / External Repo Change Proof

Required proof commands:

- `git diff --name-only origin/main...HEAD`
- `python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main ...`
- `git status --porcelain=v1 --branch`

Expected result: only allowed qsl-protocol planning/governance paths changed,
and no website implementation or external website repo files changed.

## Queue Parser Expectation

Run:

```bash
python3 scripts/ci/qsl_evidence_helper.py queue
```

Expected result:

- `READY_COUNT 1`
- sole READY item: `NA-0253`
- `NEXT_ACTIONS.md` unchanged in this implementation-planning lane

## Decision Parser Expectation

Run:

```bash
python3 scripts/ci/qsl_evidence_helper.py decisions
```

Expected result:

- D-0473 exists once after the NA-0253 PR branch changes.
- D-0474 is absent.
- Duplicate decision count is zero.

## CI Expectations

Local validation should include:

- `git diff --check`
- queue parser
- decision parser
- scope guard
- `cargo audit --deny warnings`
- `cargo tree -i rustls-webpki --locked`
- `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1`
- `goal-lint`
- markdown link validation if established
- `python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main`

Required PR checks must pass normally. `public-safety` must remain required and
green before merge and after merge.
