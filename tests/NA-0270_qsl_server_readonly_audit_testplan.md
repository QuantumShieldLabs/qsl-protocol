# NA-0270 qsl-server Read-Only Audit Testplan

Goals: G1, G3, G4, G5

## Objective

Verify that NA-0270 performs a read-only qsl-server code/security/ops audit and
designs the first executable qsl-server hardening test harness without changing
qsl-server implementation, qsl-attachments implementation, protocol/crypto
semantics, website source, workflows, scripts, Cargo files, branch protection,
or public-safety configuration.

## Protected Invariants

- Exactly one READY item remains during the audit/design PR: NA-0270.
- D-0510 exists once after the audit/design patch.
- D-0511 remains absent in the audit/design PR.
- No qsl-server implementation changes.
- No qsl-attachments implementation changes.
- No qsp protocol-core, qsc/qsl runtime, qsc-desktop, website, workflow,
  branch-protection, public-safety, Cargo, dependency, protocol, wire, crypto,
  auth, or state-machine changes.
- No production readiness claim.
- Known qsl-server gaps remain explicit.
- Proven bugs, evidence gaps, recommendations, and non-issues remain separated.

## Allowed Scope

- `docs/governance/evidence/NA-0270_qsl_server_readonly_audit_test_harness_design.md`
- `tests/NA-0270_qsl_server_readonly_audit_testplan.md`
- `docs/public/QSL_SERVER_ATTACHMENTS_PRODUCTION_BOUNDARY_PLAN.md` only for
  handoff/reference updates that do not change claims
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md` only if consistent

## Forbidden Scope

- `qsl-server/**`
- `qsl-attachments/**`
- `qsp/**`
- `qsc/**`
- `qsl/**`
- `qsl-client/**`
- `apps/**`
- `scripts/**`
- `.github/**`
- `Cargo.toml`
- `Cargo.lock`
- `tools/**`
- `inputs/**`
- `formal/**`
- `qsc-desktop/**`
- `website/**`
- runtime, protocol, crypto, service, desktop, public-safety, or branch
  protection configuration paths

## Read-Only qsl-server Audit Proof

The audit must record:

- sibling repo path, status, HEAD, branch, and remotes;
- build system/language;
- service role and transport-only boundary;
- API routes, request/response shapes, status codes, and error names;
- auth/token model;
- queue/body/overload model;
- logging/no-secret model;
- storage/state model;
- deployment/network assumptions;
- existing tests and docs;
- production-boundary gaps.

## Test-Harness Design Requirements

The design must include future executable coverage for:

- auth tests: missing auth, wrong auth, wrong bearer, wrong route token, and no secret logging;
- route tests: unknown route, malformed route, repeated route, and retired path-token routes;
- payload tests: empty body, oversized body, malformed/opaque payload, and payload no-log;
- queue tests: queue cap, overload error consistency, empty pull, and no mutation on rejected push;
- replay/idempotency tests: duplicate push, repeated pull, and message ID uniqueness if applicable;
- logging tests: auth header absent, route token absent, payload absent, and sanitized error output;
- config/startup tests: missing env, invalid max body, invalid queue cap, invalid port, and safe bind;
- health/ops tests: health endpoint if present, or explicit observability gap if absent;
- soak/stress tests: bounded bursts, overload behavior, no panic, and no unbounded growth.

## Proven Bug Recording

The audit must explicitly investigate and record the known
`ERR_QUEUE_FULL` / `ERR_OVERLOADED` mismatch, including exact evidence and
severity. It must not fix the mismatch in this lane.

Additional proven bugs must be recorded only when there is concrete file/line
or behavior evidence. Evidence gaps and recommendations must not be overstated
as proven bugs.

## Overclaim Scan

Scan added/changed docs for:

- `production-ready`
- `deployment-ready`
- `production relay ready`
- `qsl-server production ready`
- `production attachment ready`
- `qsl-attachments production ready`
- `external review complete`
- `metadata-free`
- `anonymity`
- `untraceable`
- `quantum-proof`
- `proven true Triple Ratchet`

Allowed uses must be explicitly negated, listed as future/unproven, or placed
inside prohibited wording sections. No heading may turn these phrases into an
affirmative claim.

## Link/Leak/Goal-Lint Expectations

- Relative markdown links must resolve.
- Added-line leak scan must not report secrets, auth headers, route tokens,
  secret-bearing URLs, or long secret-like hex dumps.
- Goal metadata must include G1, G3, G4, and G5 where required.
- PR body must include the exact standalone `Goals: G1, G3, G4, G5` line near
  the top.

## CI Expectations

Local validation should include:

- `git status --porcelain=v1 --branch`
- `git diff --name-only origin/main...HEAD`
- `git diff --stat origin/main...HEAD`
- `git diff --check`
- direct overclaim phrase scan
- `python3 scripts/ci/qsl_evidence_helper.py queue`
- `python3 scripts/ci/qsl_evidence_helper.py decisions`
- `python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main`
- `python3 scripts/ci/qsl_evidence_helper.py link-check`
- `python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main`
- `cargo audit --deny warnings`
- `cargo tree -i rustls-webpki --locked`
- `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1`
- `python3 formal/run_model_checks.py`
- goal-lint

Required CI must pass normally before merge. The expected PR scope is docs,
governance, public handoff, and testplan only, so docs-only cost control may
skip full suites where policy allows.

## Successor Handoff

After NA-0270 merges and closeout is authorized separately, the expected
successor is a read-only qsl-attachments code audit and test-harness design.
This testplan does not authorize NA-0271 implementation.
