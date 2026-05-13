Goals: G1, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-13
Replaces: n/a
Superseded-By: n/a

# NA-0280 qsl-server Rate / Global-Cap Harness Evidence

Directive: QSL-DIR-2026-05-13-083 / NA-0280

## Executive Summary

NA-0280 implements and test-backs the qsl-server executable
rate-limit/global route-cap harness designed in NA-0279.

qsl-server PR #53 merged the local in-app rate/global-cap implementation and
tests. qsl-protocol records evidence only in this PR: no qsl-protocol runtime,
protocol, crypto, qsl-attachments, qsc-desktop, website, workflow, script,
Cargo, branch-protection, public-safety, or dependency behavior changes are
made here.

The qsl-server controls are local deterministic hardening primitives. They do
not approve public exposure, production relay operation, qsl-attachments
service operation, or production deployment. Reverse proxy / edge rate limiting
and deployment-layer controls remain separate requirements.

## NA-0279 Design Carried Forward

NA-0279 established the baseline and target semantics:

- preserve the existing per-route queue cap and body-size cap;
- add bounded in-memory push-rate accounting;
- add a global cap on live route queues;
- create live route slots only for accepted pushes;
- stop creating route slots on unknown pulls;
- release route capacity deterministically when routes drain empty;
- keep auth, body, rate, route-cap, and queue-cap rejects deterministic;
- keep rejected requests from unexpectedly mutating queues, routes, or
  accounting;
- keep route tokens, bearer auth, auth headers, and payload bytes out of logs.

## qsl-server PR Evidence

- Repository: `QuantumShieldLabs/qsl-server`
- PR: #53, `https://github.com/QuantumShieldLabs/qsl-server/pull/53`
- Branch: `na-0280-rate-global-cap-harness`
- Head SHA: `7812ca65fc65`
- Merge SHA: `92793d678538`
- Required check: `rust` completed success.
- qsl-server implementation changed: yes.
- qsl-server dependency changed: no.
- qsl-server workflow changed: no.

Changed paths:

- `README.md`
- `docs/server/DOC-SRV-001_Deployment_Hardening_Contract_v1.0.0_DRAFT.md`
- `docs/server/DOC-SRV-003_Relay_Inbox_Contract_v1.0.0_DRAFT.md`
- `src/lib.rs`
- `src/main.rs`
- `tests/abuse_rate_queue.rs`
- `tests/config_semantics.rs`
- `tests/rate_global_cap.rs`
- `tests/rate_global_cap_logging.rs`

## Chosen Rate / Global-Cap Semantics

Config knobs added in qsl-server:

- `MAX_ROUTE_COUNT`: caps live route slots. Default and ceiling are 256.
- `PUSH_RATE_BURST`: caps per-route push burst tokens. Default and ceiling
  are 256.
- `PUSH_RATE_REFILL_PER_SEC`: refills per-route push tokens. Default is 256,
  ceiling is 4096, and `0` is allowed for deterministic no-refill operation.

Behavior:

- Accepted push to a new route creates a live route slot only if
  `MAX_ROUTE_COUNT` allows it.
- Push to a new route beyond the live-route cap returns
  `429 ERR_ROUTE_CAP`.
- Push to an existing route does not consume another route slot.
- Unknown pull returns 204 and does not create a route slot.
- Pull delivery removes messages; when a route drains empty, qsl-server
  removes the live route slot and its per-route rate bucket.
- Per-route push rate limiting uses bounded in-memory token-bucket accounting.
- Exhausted per-route push tokens return `429 ERR_RATE_LIMITED`.
- Existing full-queue behavior remains `429 ERR_OVERLOADED`.
- Existing body-size behavior remains `413 ERR_TOO_LARGE`.
- Existing missing/wrong bearer behavior remains `401 ERR_UNAUTHORIZED`.

Accounting boundary:

- Route and rate accounting are process-local and memory-only.
- Rate accounting is tied to live route slots and is removed when the route
  drains empty.
- Source-IP, process-wide, auth-mode, pull-rate, and time-based route TTL
  controls remain future work unless separately specified and tested.

## Harness Coverage

Global route cap:

- `global_route_cap_rejects_new_routes_without_mutating_existing_routes`
  configures one live route, accepts route A, rejects route B with
  `ERR_ROUTE_CAP`, proves route B is not pullable, and proves route A drains
  normally.

Unknown pull no slot:

- `unknown_pull_does_not_create_route_slot` pulls an unknown route under
  cap 1, then successfully pushes a new route. This proves the unknown pull did
  not consume the cap.

Route slot release:

- `draining_empty_route_releases_global_slot` accepts route A under cap 1,
  drains it, and then accepts route B under the same cap.

Rate-limit reject:

- `rate_limit_rejects_without_enqueueing` configures burst 1 with no refill,
  accepts the first push, rejects the second with `ERR_RATE_LIMITED`, and
  proves only the accepted message drains.

No-mutation:

- Route-cap rejects do not create pullable state for the rejected route.
- Rate-limit rejects do not enqueue the rejected message.
- Existing queue overload still drains only the accepted message.

Auth/oversize no state:

- `wrong_auth_and_oversize_do_not_consume_route_or_rate_state` proves wrong
  bearer auth and oversized body rejects do not consume route slots or
  privileged rate state before later accepted requests reuse those route names.

Existing queue overload unchanged:

- `existing_queue_overload_still_returns_err_overloaded` keeps the existing
  `ERR_OVERLOADED` taxonomy for full per-route queues.

Logging / no-secret:

- `rate_and_route_cap_logs_redact_route_auth_payload` captures logs across
  accepted push, rate-limit reject, route-cap reject, and auth reject, then
  proves raw route tokens, bearer values, the `Authorization` header word, and
  payload sentinels are absent. Accepted `x-msg-id` remains documented
  non-secret operational metadata.

No-panic:

- The new burst, route-cap, rate-limit, auth reject, oversize reject, and
  drain paths are exercised by local harnesses without panic.

## Results

qsl-server local validation passed:

- `cargo fmt --check`
- `cargo test --locked --test hardening_auth_reject_logging -- --test-threads=1`
- `cargo test --locked --test idempotency_semantics -- --test-threads=1`
- `cargo test --locked --test idempotency_logging -- --test-threads=1`
- `cargo test --locked --test config_semantics -- --test-threads=1`
- `cargo test --locked --test abuse_rate_queue -- --test-threads=1`
- `cargo test --locked --test abuse_rate_queue_logging -- --test-threads=1`
- `cargo test --locked --test rate_global_cap -- --test-threads=1`
- `cargo test --locked --test rate_global_cap_logging -- --test-threads=1`
- `cargo test --locked`
- `cargo clippy --locked --all-targets -- -D warnings`
- `cargo audit --deny warnings`
- `git diff --check`

qsl-server CI:

- PR #53 required `rust` check completed success.
- PR #53 merged normally with a merge commit and matched head SHA.

Recovered local failures:

- Pre-edit `cargo test --locked` failed once in an existing logging test.
  Classification: recoverable local test-order/log-capture flake. Corrective
  action: isolated diagnostic test passed and immediate full rerun passed.
- During implementation, `cargo fmt --check` reported mechanical formatting
  differences. Corrective action: `cargo fmt`; final result passed.
- During implementation, adding public fields to the existing `Limits` struct
  broke existing struct literals. Classification: recoverable in-scope
  implementation shape issue. Corrective action: preserve `Limits` and add a
  separate `ResourceControls` struct. Final result: focused tests and full
  suite passed.

## No-Production Boundary

This evidence proves local qsl-server executable harness behavior only. It
does not approve public exposure, production relay operation, qsl-attachments
service operation, production deployment, edge policy, incident response,
external service review, or operational memory sizing.

Remaining deployment and service-hardening gaps remain visible:

- route lifecycle / idle TTL / retention beyond delete-on-empty;
- process-wide, auth-mode, source-IP, and pull rate limits;
- deployment-layer reverse proxy / edge rate limiting;
- public exposure readiness;
- external review and operational runbook evidence.

## Next Recommended Harness

The next recommended qsl-server lane is NA-0281:

`qsl-server Route Lifecycle / TTL / Retention Harness`

Recommended first proofs:

- deterministic idle route cleanup or TTL semantics;
- capacity release after cleanup;
- no unexpected mutation on cleanup-triggering requests;
- bounded cleanup work per request;
- secret-safe logs for cleanup/retention events;
- preservation of NA-0280 rate/global-cap behavior.
