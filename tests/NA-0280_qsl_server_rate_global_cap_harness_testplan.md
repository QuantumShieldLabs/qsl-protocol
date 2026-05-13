Goals: G1, G3, G4, G5

# NA-0280 qsl-server Rate / Global-Cap Harness Testplan

## Objective

Verify that NA-0280 records qsl-server executable rate-limit/global route-cap
harness evidence after the qsl-server implementation PR merges, without
changing qsl-protocol runtime, protocol, crypto, qsl-attachments, qsc-desktop,
website, workflows, scripts, Cargo files, branch protection, public-safety, or
dependencies.

## Protected Invariants

- qsl-server rate/global-cap behavior is deterministic.
- qsl-server route and rate accounting are bounded.
- Rejected qsl-server requests do not unexpectedly mutate queues, routes, or
  privileged accounting.
- Unknown pulls do not create route slots.
- Draining a route to empty releases route capacity under the chosen semantics.
- Route tokens, bearer tokens, auth headers, and payloads do not leak in logs.
- Existing auth/reject/logging, x-msg-id, invalid config, and abuse/queue
  harnesses remain green.
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
- chosen route-cap semantics;
- chosen rate-limit semantics;
- route slot lifecycle and release semantics;
- validation commands and CI result;
- no dependency or workflow changes.

Minimum executable qsl-server tests:

- global route cap rejects new routes without mutating existing routes;
- unknown pull does not create route slot;
- draining empty route releases global slot;
- rate-limit reject does not enqueue;
- wrong auth and oversize do not consume route or rate state;
- existing queue overload still returns `ERR_OVERLOADED`;
- rate and route-cap logs redact route, auth, and payload sentinels.

## Chosen Semantics

- `MAX_ROUTE_COUNT` bounds live route slots.
- Accepted push to a new route creates a slot only when the cap allows.
- Push to a new route beyond the cap returns `429 ERR_ROUTE_CAP`.
- Unknown pull returns 204 without creating a slot.
- Pull delivery removes messages.
- Draining a route to empty removes the live route slot and per-route rate
  bucket.
- `PUSH_RATE_BURST` and `PUSH_RATE_REFILL_PER_SEC` implement a bounded
  in-memory per-route push token bucket.
- Exhausted route push tokens return `429 ERR_RATE_LIMITED`.
- Existing per-route queue overload remains `429 ERR_OVERLOADED`.
- Existing oversized body reject remains `413 ERR_TOO_LARGE`.
- Existing missing/wrong bearer reject remains `401 ERR_UNAUTHORIZED`.
- `PUSH_RATE_REFILL_PER_SEC=0` is allowed for deterministic no-refill tests.

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

- `docs/governance/evidence/NA-0280_qsl_server_rate_global_cap_harness.md`
- `tests/NA-0280_qsl_server_rate_global_cap_harness_testplan.md`
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

NA-0280 may claim only local qsl-server executable rate/global-cap proof. It
must not claim production relay operation, public exposure approval,
qsl-attachments service readiness, external service review completion, edge
policy completion, or production deployment approval.

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

After the NA-0280 qsl-protocol evidence PR merges and post-merge
public-safety is green, a separate closeout may mark NA-0280 DONE and restore
exactly one READY successor: NA-0281, qsl-server Route Lifecycle / TTL /
Retention Harness.

The closeout must not implement NA-0281.
