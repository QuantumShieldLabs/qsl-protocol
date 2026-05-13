Goals: G1, G3, G4, G5

# NA-0281 qsl-server Route Lifecycle / TTL / Retention Harness Testplan

## Objective

Verify that NA-0281 records qsl-server executable route lifecycle / TTL /
retention harness evidence after the qsl-server implementation PR merges,
without changing qsl-protocol runtime, protocol, crypto, qsl-attachments,
qsc-desktop, website, workflows, scripts, Cargo files, branch protection,
public-safety, or dependencies.

## Protected Invariants

- qsl-server route lifecycle behavior is deterministic.
- Unknown pulls do not create route slots.
- Accepted pushes create route slots only when caps allow.
- Draining a route to empty releases route capacity and per-route rate
  accounting.
- Idle route TTL cleanup releases route capacity and per-route rate accounting.
- Expired non-empty routes do not return stale queued messages.
- Rejected qsl-server requests do not unexpectedly mutate queues, routes, or
  accounting.
- Route tokens, bearer values, auth headers, and payloads do not leak in logs.
- Existing auth/reject/logging, x-msg-id, invalid config, abuse/queue, and
  rate/global-cap harnesses remain green.
- qsl-protocol remains implementation-clean.
- qsl-attachments remains untouched.
- qsc-desktop remains untouched.
- Website/external website remain untouched.
- No production deployment or public exposure claim is introduced.

## qsl-server Executable Harness Proof

The qsl-server proof must record:

- qsl-server PR number, head SHA, and merge SHA;
- qsl-server changed paths;
- whether qsl-server implementation changed;
- added config knobs;
- chosen route lifecycle / TTL / retention semantics;
- validation commands and CI result;
- no dependency or workflow changes.

Minimum executable qsl-server tests:

- unknown pull does not create route slot;
- drain-to-empty releases route slot and per-route rate bucket;
- idle route TTL releases capacity;
- expired route does not return stale message;
- expired route allows new route after cap;
- expired route releases rate bucket/accounting;
- push after expiry does not resurrect old messages;
- TTL cleanup logs redact route, auth, and payload sentinels;
- invalid TTL config is deterministic;
- existing rate/global-cap harness remains green.

## Chosen Semantics

- `ROUTE_IDLE_TTL_MS` is the qsl-server route idle TTL control.
- Missing TTL config uses the documented default.
- Non-numeric or zero TTL config fails startup deterministically.
- Above-ceiling TTL config is capped.
- TTL applies to live route state, including non-empty routes.
- Cleanup runs on canonical push/pull after auth, route-token, body-size, and
  pull-`max` validation.
- Expired queued messages are discarded and are not delivered.
- Cleanup releases route slots and per-route rate accounting.
- Cleanup logs contain redacted route identifiers and bounded counts only.

## Allowed / Forbidden qsl-server Scope

Allowed qsl-server scope for the already-merged implementation PR:

- `src/**`
- `tests/**/*.rs`
- `tests/*.rs`
- `README.md`
- `docs/**`

Forbidden qsl-server scope:

- `Cargo.toml`
- `Cargo.lock`
- `.github/**`
- `scripts/**`
- deployment files
- packaging files
- dependency files

## qsl-protocol Evidence Scope

Allowed qsl-protocol paths for this evidence PR:

- `docs/governance/evidence/NA-0281_qsl_server_route_lifecycle_ttl_retention_harness.md`
- `tests/NA-0281_qsl_server_route_lifecycle_ttl_retention_harness_testplan.md`
- `docs/public/QSL_SERVER_ATTACHMENTS_PRODUCTION_BOUNDARY_PLAN.md` only for
  handoff references
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Forbidden qsl-protocol scope:

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
- runtime/protocol/crypto/demo/service code
- branch-protection settings
- public-safety/check configuration
- branch deletion

## No Production-Readiness Claim

NA-0281 may claim only local qsl-server executable route lifecycle / TTL /
retention proof. It must not claim production relay operation, public exposure
approval, qsl-attachments service readiness, external service review
completion, edge policy completion, or production deployment approval.

## Link / Leak / Goal-Lint Expectations

- Relative markdown links must resolve.
- Added-line leak scan must not report secrets, auth header values, raw route
  tokens, payload sentinels, sensitive endpoints, or long secret-like hex
  dumps.
- Goal metadata must include G1, G3, G4, and G5 where required.
- PR body must include the exact standalone `Goals: G1, G3, G4, G5` line near
  the top.
- Known gaps must remain visible.

## CI Expectations

Local qsl-protocol validation should include:

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
- repo-local goal-lint or helper PR-body preflight with the exact Goals line.

Required CI must pass normally before merge. The expected qsl-protocol PR scope
is docs, governance, evidence, and testplan only, so docs-only cost control may
skip full suites where policy allows.

## Successor Handoff

After the NA-0281 qsl-protocol evidence PR merges and post-merge
public-safety is green, a separate closeout may mark NA-0281 DONE and restore
exactly one READY successor: NA-0282, qsl-attachments Retention / Cleanup /
Recovery Harness.

The closeout must not implement NA-0282.
