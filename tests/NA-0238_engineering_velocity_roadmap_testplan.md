Status: Supporting
Owner: QSL governance
Last-Updated: 2026-04-30

# NA-0238 Engineering Velocity Roadmap Testplan

Goals: G1, G2, G3, G4, G5

## Objective

Prove `NA-0238` creates the Director-approved roadmap, engineering-velocity policy, workday/overnight autopilot policy, demo acceptance criteria, conformance-vector prioritization, and read-only audit evidence, then closes with `NA-0239` as the sole READY successor.

## Protected invariant

- Governance supports engineering and does not replace implementation, tests, vectors, demo behavior, or release-hardening automation.
- `public-safety` remains required and green.
- No code, workflow, script, Cargo, protocol, crypto, demo implementation, service, qsc-desktop, qsl-server, qsl-attachments, or website paths change.
- Exactly one READY item exists at every queue transition.
- Roadmap/demo language does not overclaim production readiness.

## Scope guard

Allowed changed paths are limited to the NA-0238 directive paths:

- `ROADMAP.md`
- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/governance/ENGINEERING_VELOCITY_POLICY.md`
- `docs/governance/WORKDAY_AUTOPILOT_POLICY.md`
- `docs/governance/evidence/NA-0238_overnight_security_audit_report.md`
- `docs/demo/DEMO_ACCEPTANCE_CRITERIA.md`
- `docs/conformance/CONFORMANCE_VECTOR_PRIORITIZATION.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0238_engineering_velocity_roadmap_testplan.md`

Forbidden path proof must confirm no `.github/**`, `scripts/**`, `Cargo.toml`, `Cargo.lock`, `qsp/**`, `qsc/**`, `qsl/**`, `qsl-client/**`, `apps/**`, `tools/refimpl/**`, `tools/actors/**`, `inputs/**`, `qsc-desktop/**`, `qsl-server/**`, `qsl-attachments/**`, `website/**`, runtime/protocol/crypto/demo/service code, branch-protection settings, or public-safety/check configuration changes.

## Roadmap artifact validation

`ROADMAP.md` must state:

- research-stage, not production-ready posture
- immediate recovery summary
- 30-day, 60-day, and 90-day priorities
- the principle that normal future work should produce executable behavior, invariant tests, conformance vectors, demo acceptance behavior, or release-hardening automation

## Autopilot policy validation

`WORKDAY_AUTOPILOT_POLICY.md` must define:

- Level 1 unattended actions inside active scope
- Level 2 actions requiring directive pre-authorization
- Level 3 hard stops
- overnight-mode prohibitions
- 120-minute public-safety/full-suite wait budget and 5-minute polling interval
- morning report requirements

## Demo acceptance validation

`DEMO_ACCEPTANCE_CRITERIA.md` must define valid send/decrypt acceptance, invalid/downgrade/malformed reject acceptance, KT malformed evidence rejection, attachment-path acceptance if applicable, metadata expectations, non-goals, and CI/demo-smoke linkage.

## Conformance prioritization validation

`CONFORMANCE_VECTOR_PRIORITIZATION.md` must prioritize KT, SCKA, downgrade, no-state-mutation, metadata, demo, and release-readiness vectors, plus the first 10 recommended next vectors/tests.

## Overnight audit/report validation

The audit report must include exact commands run, pass/fail summary, static audit findings, crypto review observations, suspected bugs or gaps, recommendations, explicit no-code-changed statement, public-safety red-main prevention recommendation, and uncertainty statements where findings are not proven bugs.

## Queue parser expectations

Before NA-0238:

- `READY_COUNT 1`
- `READY NA-0238`
- `NA-0237 DONE`
- `NA-0237A DONE`
- `NA-0237B DONE`
- `NA-0237C DONE`
- `NA-0237D DONE`
- D-0442 absent

After NA-0238:

- `READY_COUNT 1`
- `READY NA-0239`
- `NA-0238 DONE`
- `NA-0237 DONE`
- D-0442 exists once
- no duplicate decision IDs

## CI expectations

The PR must satisfy the protected contexts:

- `ci-4a`
- `ci-4b`
- `ci-4c`
- `ci-4d`
- `ci-4d-dur`
- `demo-cli-build`
- `demo-cli-smoke`
- `formal-scka-model`
- `goal-lint`
- `metadata-conformance-smoke`
- `suite2-vectors`
- `CodeQL`
- `macos-qsc-qshield-build`
- `public-safety`

`CodeQL` may be neutral only if GitHub accepts it. `public-safety` must be success. No branch-protection exception, admin bypass, direct push, check spoofing, squash merge, or rebase merge is allowed.
