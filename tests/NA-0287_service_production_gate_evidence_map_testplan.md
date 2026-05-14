Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-14

# NA-0287 Service Production-Gate Evidence Map Testplan

Goals: G1, G3, G4, G5

## Objective

Validate the governance/evidence-only work for `NA-0287 — Service
Production-Gate Evidence Map and Deployment Boundary Plan`.

The lane must map current qsl-server and qsl-attachments hardening evidence
into production-gate boundaries without implementing service behavior or
making stronger public claims.

## Protected Invariants

- qsl-server remains read-only in this directive.
- qsl-attachments remains read-only in this directive.
- qsl-protocol runtime, protocol, wire, crypto, auth, and state-machine
  behavior remain unchanged.
- qsc/qsl runtime, qsc-desktop, website, external website, workflows, scripts,
  Cargo manifests, Cargo lockfiles, branch protection, public-safety
  configuration, and dependencies remain unchanged.
- Known service deployment gaps remain explicit.
- No production-readiness claim is introduced.
- No public-internet readiness claim is introduced.
- No external-review-complete claim is introduced.
- Metadata phase-2 gaps remain explicit.
- Exactly one qsl-protocol READY item remains during the Packet E evidence PR:
  NA-0287.

## Allowed / Forbidden Scope

Allowed qsl-protocol paths:

- `docs/governance/evidence/NA-0287_service_production_gate_evidence_map.md`
- `tests/NA-0287_service_production_gate_evidence_map_testplan.md`
- `docs/public/QSL_SERVER_ATTACHMENTS_PRODUCTION_BOUNDARY_PLAN.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Forbidden paths:

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
- `website/**`
- external website repository paths
- runtime, protocol, crypto, demo, or service implementation code
- branch-protection or public-safety configuration
- branch deletion

## qsl-server Evidence-Map Requirements

The evidence map must classify:

- API contract / docs consistency.
- Auth and bearer/route-token handling.
- Reject/no-mutation behavior.
- Payload/body limits.
- Queue depth / overload.
- Rate limiting and global route caps.
- Route lifecycle / TTL / retention.
- `x-msg-id` semantics.
- Logging/no-secret behavior.
- Config fail-closed behavior.
- Local loopback harness coverage.
- Deployment assumptions.
- TLS/proxy/public exposure boundaries.
- Observability/health/metrics gaps.
- Operational runbook gaps.
- Dependency/advisory posture.
- External review readiness.

For each category, the map must separate executable proof from docs-only,
not-ready, out-of-scope, or future-gated work.

## qsl-attachments Evidence-Map Requirements

The evidence map must classify:

- Opaque-ciphertext boundary.
- Malformed JSON and reject taxonomy.
- Capability scope and abuse.
- Retention / cleanup / recovery.
- Disk pressure / quota / abuse.
- Backup / partial restore / transactional recovery.
- No-secret/no-plaintext logging.
- Restart and same-root recovery.
- Partial restore boundary.
- Unsupported hot/live backup boundary.
- Public exposure / deployment assumptions.
- Operational backup/restore runbook gaps.
- Observability/health/metrics gaps.
- Dependency/advisory posture.
- External review readiness.

For each category, the map must identify the evidence source, PR/NA reference,
classification, and remaining gate.

## Deployment Boundary Requirements

The evidence map must include boundaries for:

- local/test harness scope;
- private-network proof scope;
- public internet exposure;
- TLS/proxy operation;
- observability and metrics;
- logging and secret handling;
- backup and restore;
- rate and abuse;
- external review.

The map must distinguish qsl-protocol demo evidence from qsl-server and
qsl-attachments service operation evidence.

## No Implementation Proof

Validation must prove the Packet E diff is governance/evidence only and does
not touch service implementation or runtime paths.

Required checks:

- `git diff --name-only origin/main...HEAD`
- `python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main`
  with the exact allowed path list
- explicit forbidden-path scan

## No Production-Readiness Claim

The map must not make affirmative claims that any service is production ready,
deployment ready, production relay ready, production attachment ready,
production backup/restore ready, or public service ready.

Prohibited phrases may appear only when explicitly negated, listed as
prohibited wording, or described as future/unproven.

## No Public-Internet Readiness Claim

The map must state that public internet exposure is not authorized by the
current evidence and remains future-gated on exact deployment proof.

## Link / Leak / Goal-Lint Expectations

Expected qsl-protocol validation:

- queue parser reports `READY_COUNT 1` and `READY NA-0287`;
- decisions parser reports D-0544 exactly once and no duplicate decision IDs;
- D-0545 remains absent before closeout;
- scope guard accepts only the allowed paths;
- markdown link check passes;
- leak scan passes;
- direct overclaim scan has no affirmative prohibited claims;
- PR body has a standalone `Goals: G1, G3, G4, G5` line near the top.

## CI Expectations

- qsl-protocol public-safety is required and green before the PR.
- Required PR checks attach and pass normally.
- The PR is evidence/governance only and should classify as docs-only under
  NA-0262A cost-control behavior.
- No admin bypass, direct push, squash, rebase, branch-protection mutation, or
  branch deletion is allowed.

## Successor Handoff

If the evidence PR merges and post-merge public-safety is green, the optional
closeout may mark NA-0287 DONE and restore exactly one successor:

`NA-0288 — Metadata Phase-2 and External Review Readiness Gap Plan`

NA-0288 must remain planning-only unless a separate directive explicitly
authorizes implementation.
