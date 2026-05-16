Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-15
Replaces: n/a
Superseded-By: n/a

# NA-0295 Website Landing Page Handoff and Evidence Visuals Testplan

## Objective

Validate that NA-0295 creates a claim-safe website landing page handoff and
evidence-visuals plan without mutating the website, external website
repositories, protocol/runtime code, service code, workflows, scripts, Cargo
metadata, dependencies, branch protection, or public-safety configuration.

## Protected Invariants

- The active queue remains exactly one READY item: NA-0295.
- D-0566 is added once and no duplicate decision IDs are introduced.
- Website handoff remains planning only.
- No website or external website repository is mutated.
- No protocol, wire, crypto, state-machine, runtime, demo, service,
  qsc-desktop, qsl-server, or qsl-attachments implementation changes occur.
- No workflow, script, Cargo, dependency, branch-protection, or public-safety
  configuration changes occur.
- No production-readiness, public-internet-readiness,
  external-review-complete, anonymity, metadata-free, untraceable,
  quantum-proof, unbreakable, or guaranteed-secure claim is introduced.
- Metadata phase-2, service production gates, public internet readiness, and
  external review completion remain explicit gaps.

## Allowed Scope

Allowed by the live NA-0295 queue entry:

- `docs/governance/evidence/NA-0295_website_landing_evidence_visuals_plan.md`
- `tests/NA-0295_website_landing_evidence_visuals_testplan.md`
- `docs/public/WEBSITE_IMPLEMENTATION_HANDOFF.md` if present
- `docs/public/PUBLIC_ATTENTION_AND_VISIBILITY_STRATEGY.md` only if a safe
  reference update is needed
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md` only if consistent

## Forbidden Scope

Forbidden for NA-0295:

- website source or external website repository files;
- website deployment, DNS, hosting, forms, social, or public posting work;
- README.md and START_HERE.md;
- `docs/public/INDEX.md`;
- `.github/**`, `scripts/**`, `Cargo.toml`, `Cargo.lock`;
- `qsp/**`, `qsc/**`, `qsl/**`, `qsl-client/**`, `apps/**`, `tools/**`,
  `inputs/**`, `formal/**`, `qsc-desktop/**`, `qsl-server/**`,
  `qsl-attachments/**`;
- runtime, protocol, crypto, demo, or service implementation paths;
- branch-protection or public-safety configuration;
- branch deletion.

## Website / Source Read-Only Audit Requirements

The NA-0295 plan must record:

- local docs baseline;
- public website status from safe unauthenticated inspection where available;
- website source status, including unknown or unverified source paths;
- no-mutation proof;
- risks from external product/service language near qsl-protocol claims;
- implementation prerequisites for a future website lane.

If live website source cannot be verified as public and safe to inspect, the
plan must not infer source contents and must recommend a separate source
verification lane.

## Landing Page Handoff Requirements

The plan must define:

- hero section headline, subheadline, primary CTA, secondary CTA, and boundary
  note;
- why-this-matters section;
- evidence-first section;
- demo section;
- service hardening section;
- metadata honesty section;
- external review section;
- how-to-help section;
- footer or claim-boundary section.

For each section, the plan must include proposed copy, evidence target,
claim-boundary note, visual suggestion, implementation notes, and risk level.

## Evidence Visuals Requirements

The plan must include future visual guidance for:

- evidence receipts map;
- proven / not proven matrix;
- fail-closed demo flow;
- metadata phase-2 fixture proof diagram;
- service hardening map;
- external review readiness ladder;
- demo timeline / build journal timeline;
- security claims with receipts infographic;
- reviewer path diagram;
- contributor path diagram.

For each visual, the plan must record objective, audience, source evidence,
required data, suggested format, safe caption, prohibited caption,
implementation difficulty, evidence gate before publishing, and whether it can
be implemented in repo docs before website work.

## Claim Matrix Requirements

The plan must include:

- safe claims;
- claims requiring evidence links;
- prohibited claims;
- replacement language;
- hero, service, metadata, external review, and demo copy boundaries;
- future website implementation stop conditions.

## Link, Leak, and Overclaim Expectations

Validation must include:

- `python3 scripts/ci/qsl_evidence_helper.py link-check`
- `python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main`
- a direct high-risk phrase scan over changed lines.

High-risk phrase matches are acceptable only when they are:

- explicitly prohibited wording;
- explicitly negated;
- explicitly `NOT_READY`;
- explicitly future-gated or unproven;
- examples of what not to say.

## CI Expectations

Before PR:

- queue helper reports READY_COUNT 1 and READY NA-0295;
- decisions helper reports no duplicates and D-0566 once;
- scope guard passes with only allowed paths changed;
- diff check passes;
- link-check passes;
- added-line leak scan passes;
- `cargo audit --deny warnings` passes;
- `cargo tree -i rustls-webpki --locked` reports the locked dependency path;
- `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1`
  passes;
- `python3 formal/run_model_checks.py` passes if present;
- goal-lint passes for the PR body;
- classifier proof reports the changed docs/governance path set as docs-only.

PR merge is allowed only when required checks complete normally and
public-safety remains required and green.

## Future Website Implementation Gate

A future implementation lane must not begin until:

- the exact website source repository, default branch, and deployment path are
  verified;
- live website copy has been re-audited;
- the claim matrix is current;
- evidence links are stable;
- future validation includes screenshots, link checks, overclaim scans,
  accessibility review, and rollback proof;
- all `NOT_READY` boundaries remain visible.
