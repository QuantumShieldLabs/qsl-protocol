Goals: G1, G3, G4, G5

# NA-0277 qsl-server Abuse Rate Queue Harness Testplan

## Objective

Verify that NA-0277 records the merged qsl-server executable abuse/rate/queue
harness without changing qsl-protocol runtime behavior, qsl-attachments
implementation, protocol/wire/crypto behavior, workflows, scripts, Cargo files,
branch protection, or public-safety configuration.

## Protected Invariants

- Exactly one READY item remains during the evidence PR: NA-0277.
- D-0524 exists once after the evidence patch.
- D-0525 remains absent in the evidence PR.
- qsl-server harness PR #52 is merged before qsl-protocol evidence merge.
- Overload behavior is deterministic.
- Queue/resource caps are explicit.
- Rejected requests do not mutate queues unexpectedly.
- Route tokens, auth headers, and payloads do not leak under pressure.
- Rate limiting and global route caps are not claimed unless executable proof
  exists.
- qsl-protocol implementation paths remain untouched.
- qsl-attachments implementation paths remain untouched.
- No protocol, wire, crypto, auth, state-machine, qsp protocol-core, qsc/qsl
  runtime, qsc-desktop, website, workflow, script, Cargo, dependency,
  branch-protection, or public-safety configuration change is introduced in
  qsl-protocol.
- No production readiness or deployment readiness claim is introduced.
- Proven bugs, current behavior, future semantic decisions, and
  recommendations remain separated.

## qsl-server Executable Harness Proof

The evidence must record:

- qsl-server harness PR number, head SHA, merge SHA, changed paths, and check
  result;
- chosen/current overload and rate/global-cap semantics;
- whether qsl-server implementation code changed;
- proof that the harness PR did not change dependencies or workflows;
- focused abuse/rate/queue harness test results;
- existing auth/reject/logging, x-msg-id, and invalid config harness results;
- full qsl-server test result;
- qsl-server audit result;
- recovered local or CI failures, if any.

## Chosen / Current Semantics

NA-0277 records and test-backs:

- per-route in-memory FIFO queues;
- configured per-route queue depth as the active queue cap;
- `429 ERR_OVERLOADED` for pushes after the configured queue depth is full;
- no enqueue for overloaded pushes;
- route isolation under burst pressure;
- `413 ERR_TOO_LARGE` for oversized body rejects with no enqueue;
- `401 ERR_UNAUTHORIZED` for missing or wrong bearer auth with no enqueue;
- pull/drain returns exactly accepted messages;
- repeated pull after drain is deterministic;
- route tokens, auth headers, and payloads do not leak under pressure;
- accepted `x-msg-id` remains documented non-secret metadata;
- no in-app rate limiting is currently implemented;
- no global route-count cap is currently implemented.

## Allowed / Forbidden qsl-server Scope

Allowed qsl-server harness scope:

- `tests/**/*.rs`;
- `tests/*.rs`;
- `README.md` only for abuse/rate/queue semantic clarification;
- `docs/**` only for abuse/rate/queue semantic clarification;
- `src/**` only if a new test proves a current overload/queue/reject behavior
  violates the repaired contract or protected invariant.

Forbidden qsl-server harness scope:

- `Cargo.toml`;
- `Cargo.lock`;
- `.github/**`;
- `scripts/**`;
- deployment files;
- packaging files;
- dependency files;
- broad production rate-limiting implementation.

## qsl-protocol Evidence Scope

Allowed qsl-protocol paths:

- `docs/governance/evidence/NA-0277_qsl_server_abuse_rate_queue_harness.md`;
- `tests/NA-0277_qsl_server_abuse_rate_queue_harness_testplan.md`;
- `DECISIONS.md`;
- `TRACEABILITY.md`;
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`.

Forbidden qsl-protocol paths include runtime, protocol, crypto, qsl-server,
qsl-attachments, qsc-desktop, website, workflow, script, Cargo, dependency,
branch-protection, and public-safety configuration paths.

## No Production-Readiness Claim

NA-0277 may claim only local executable qsl-server overload/queue pressure
evidence. It must not claim qsl-server public operation, public internet
exposure safety, production relay approval, production attachment service
operation, external review completion, metadata elimination, anonymity,
untraceability, completed production deployment review, implemented in-app rate
limiting, or implemented global route caps.

## Link / Leak / Goal-Lint Expectations

- Relative markdown links must resolve.
- Added-line leak scan must not report secrets, auth header values, route
  tokens, payload sentinels, sensitive endpoints, or long secret-like hex
  dumps.
- Goal metadata must include G1, G3, G4, and G5 where required.
- PR body must include the exact standalone `Goals: G1, G3, G4, G5` line near
  the top.
- Known gaps must remain visible.

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
- goal-lint or helper PR-body preflight with the exact Goals line.

Required CI must pass normally before merge. The expected qsl-protocol PR scope
is docs, governance, evidence, and testplan only, so docs-only cost control may
skip full suites where policy allows.

## Successor Handoff

After qsl-server PR #52 and the qsl-protocol NA-0277 evidence PR merge, NA-0277
may close out only under a separate closeout packet that promotes exactly one
READY successor. The recommended successor is NA-0278: public README attention
refresh and stale branch cleanup audit.
