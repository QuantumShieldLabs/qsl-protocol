Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-15
Replaces: n/a
Superseded-By: n/a

# NA-0294 Public Evidence Navigation Refresh Test Plan

## Objective

Refresh README, START_HERE, and docs/public evidence navigation so public
readers can find current proof and current gaps without unsupported claims.

## Protected Invariants

- README and START_HERE improve navigation without changing protocol,
  cryptographic, runtime, service, demo, or website behavior.
- Every public hook remains evidence-bound.
- Every "what is proven" statement points to evidence.
- "What is not proven" boundaries remain visible.
- Metadata phase-2 remains incomplete.
- External review remains incomplete.
- Service production readiness remains incomplete.
- Public internet readiness remains incomplete.

## Allowed Scope

- `README.md`
- `START_HERE.md`
- `docs/public/INDEX.md`
- `docs/public/RELEASE_READINESS_EVIDENCE_MAP.md`
- `docs/public/EXTERNAL_REVIEW_PACKAGE.md`
- `docs/governance/evidence/NA-0294_public_evidence_navigation_refresh_audit.md`
- `tests/NA-0294_public_evidence_navigation_refresh_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Forbidden Scope

- `website/**`
- external website repositories
- `.github/**`
- `scripts/**`
- `Cargo.toml`
- `Cargo.lock`
- `qsp/**`
- `qsc/**`
- `qsl/**`
- `qsl-client/**`
- `apps/**`
- `tools/**`
- `inputs/**`
- `formal/**`
- `qsc-desktop/**`
- `qsl-server/**`
- `qsl-attachments/**`
- runtime, protocol, crypto, demo, service, workflow, dependency,
  branch-protection, and public-safety configuration paths.

## README Checks

- The public hook is present near the top:
  "Post-quantum messaging needs evidence, not slogans. QSL is building that
  evidence in public."
- Research-stage and not-production-ready boundaries are prominent.
- "Start With The Evidence" links to the public evidence landing page, release
  readiness map, external review package, demo acceptance criteria, service
  production-gate map, and metadata phase-2 evidence.
- "What Is Proven Now" and "What Is Not Proven Yet" remain accurate and
  bounded.
- Reviewer/contributor actions point to evidence, demos, claim boundaries, and
  governance records.

## START_HERE Checks

- Audience entry points exist for public overview, evidence inspection, demos,
  security/claim review, and contribution.
- Governance spine, authoritative-source priority, and fail-closed operational
  discipline remain intact.
- No safety instruction is removed or weakened.

## docs/public Index Checks

- The index exists and remains the authoritative public/release posture front
  door.
- Sections cover what QSL is, what QSL is not, evidence map, demo evidence,
  service evidence, metadata phase-2 evidence, external review package, public
  visibility strategy, claim boundaries, and how to help.
- All links exist and point to current in-repo evidence or governance records.

## Evidence Link Checks

Run:

```bash
python3 scripts/ci/qsl_evidence_helper.py link-check
```

Expected result: `TOTAL_MISSING 0`.

## Claim-Boundary Checks

Scan changed lines for high-risk phrases:

- production-ready
- deployment-ready
- production relay ready
- production attachment ready
- public internet ready
- external review complete
- externally reviewed
- metadata-free
- anonymity
- anonymous messaging
- untraceable
- quantum-proof
- unbreakable
- military-grade
- guaranteed secure
- proven true Triple Ratchet
- ready for production
- release ready
- sanitized errors implemented
- retention implemented
- purge implemented
- anonymous
- private by default

Matches are acceptable only when they are negated, listed as prohibited,
marked `NOT_READY`, or explicitly future/unproven.

## Link / Leak / Goal-Lint Expectations

```bash
git diff --check origin/main...HEAD
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main
python3 scripts/ci/qsl_evidence_helper.py link-check
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
python3 tools/goal_lint.py
```

Goal-lint should be run with a synthetic pull-request event before PR creation
or through the repository PR wrapper after PR creation.

## CI Expectations

- Required PR checks complete successfully or with accepted neutral/skipped
  docs-only behavior.
- `public-safety` remains required and green.
- Docs-only classification should permit full-suite skips where current
  public-safety policy allows it.

## Successor Handoff

After merge and post-merge public-safety success, close out NA-0294 and restore
NA-0295 for website landing page handoff and evidence-visuals planning. Do not
implement NA-0295 inside this lane.
