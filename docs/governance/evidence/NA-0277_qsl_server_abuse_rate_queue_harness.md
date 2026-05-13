Goals: G1, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-13
Replaces: n/a
Superseded-By: n/a

# NA-0277 qsl-server Abuse Rate Queue Harness

Directive: QSL-DIR-2026-05-13-080 / NA-0277

## Executive Summary

NA-0277 records executable qsl-server abuse/rate/queue-cap harness evidence for
bounded overload behavior, per-route queue caps, rejected-request no-mutation,
route isolation, body-limit rejects, auth rejects under queue pressure, pull
drain behavior, no secret logging under pressure, and no-panic pressure paths.

qsl-server PR #52 merged a local/loopback harness and docs clarification only.
No qsl-server source, dependency, Cargo, or workflow files changed.

Selected current semantics:

- qsl-server uses per-route in-memory FIFO queues.
- `MAX_QUEUE_DEPTH` is explicit per route. Accepted pushes enqueue until the
  configured depth; the next push returns `429 ERR_OVERLOADED` and does not
  enqueue an extra item.
- Overloaded routes remain pullable and drain exactly the accepted FIFO items.
- Queue pressure on one route token does not affect a different route token.
- Oversized bodies return `413 ERR_TOO_LARGE` before enqueue.
- Missing or wrong bearer auth returns `401 ERR_UNAUTHORIZED` before enqueue.
- Logs may include accepted `x-msg-id` values as documented non-secret
  metadata; route tokens, auth headers, and payloads must not appear in logs.
- No in-app rate limiting is implemented.
- No global route-count cap is implemented.

No qsl-protocol runtime, protocol, crypto, state-machine, qsl-attachments,
qsc-desktop, website, workflow, script, Cargo, dependency, branch-protection,
or public-safety configuration path changed in this evidence lane. This
evidence does not claim production readiness.

## qsl-server PR Evidence

- Repository: `QuantumShieldLabs/qsl-server`
- PR: #52, `NA-0277: add qsl-server abuse rate queue harness`
- PR URL: https://github.com/QuantumShieldLabs/qsl-server/pull/52
- Head SHA: `979270e3d5e2`
- Merge SHA: `75e16e35c399`
- Merged at: 2026-05-13T16:48:54Z
- Required check: `rust` success before merge.
- Required check URL:
  https://github.com/QuantumShieldLabs/qsl-server/actions/runs/25813284893/job/75835034139
- Changed paths:
  - `README.md`
  - `docs/server/DOC-SRV-003_Relay_Inbox_Contract_v1.0.0_DRAFT.md`
  - `tests/abuse_rate_queue.rs`
  - `tests/abuse_rate_queue_logging.rs`
- Implementation changed: no.
- Dependency files changed: no.
- Workflow changed: no.
- Production-readiness claim: no.
- Harness is loopback/local only.

Validation before qsl-server PR merge:

- `cargo fmt --check`: pass.
- `cargo test --locked --test hardening_auth_reject_logging -- --test-threads=1`:
  pass, 7 tests.
- `cargo test --locked --test idempotency_semantics -- --test-threads=1`:
  pass, 4 tests.
- `cargo test --locked --test idempotency_logging -- --test-threads=1`:
  pass, 1 test.
- `cargo test --locked --test config_semantics -- --test-threads=1`:
  pass, 7 tests.
- `cargo test --locked --test abuse_rate_queue -- --test-threads=1`:
  pass, 3 tests.
- `cargo test --locked --test abuse_rate_queue_logging -- --test-threads=1`:
  pass, 1 test.
- `cargo test --locked`: pass, including the new abuse/rate/queue harness.
- `cargo clippy --locked --all-targets -- -D warnings`: pass.
- `cargo audit --deny warnings`: pass.
- `git diff --check`: pass.
- Changed-file overclaim scan: pass.
- Changed-file leak/secret shape scan: pass; the only intentional test
  sentinel hits are asserted absent from captured service logs.
- No `Cargo.toml`, `Cargo.lock`, `.github/**`, `scripts/**`, deployment, or
  packaging diff.

Post-merge qsl-server main validation:

- `cargo audit --deny warnings`: pass on `75e16e35c399`.
- `cargo test --locked`: pass on `75e16e35c399`.

Recovered qsl-server validation/CI failures:

- First qsl-server CI run on PR #52 head `a8cf3955cf07` failed in
  `cargo test -q` because a new log-capture assertion lived in the same
  default-parallel integration test binary as other harness tests.
  Classification: recoverable in-scope test-shape issue. Corrective action:
  split the pressure logging proof into `tests/abuse_rate_queue_logging.rs`
  and push a forward fix commit without rewriting PR history. Final result:
  local `cargo test -q` passed, required validation passed, and PR #52 CI
  passed on head `979270e3d5e2`.
- Initial local `cargo fmt --check` reported formatting diffs in the new
  harness. Classification: recoverable local formatting issue. Corrective
  action: ran `cargo fmt`. Final result: `cargo fmt --check` passed.

## Harness Coverage

### Queue Cap / Overload

- `tests/abuse_rate_queue.rs` fills a route queue exactly to a configured
  depth of 3.
- The next two pushes to the same route return `429 ERR_OVERLOADED`.
- The overload response body is deterministic: `ERR_OVERLOADED`.
- The overloaded route remains pullable after rejection.

### No-Mutation

- Overloaded pushes do not enqueue extra messages.
- Oversized body rejects do not enqueue.
- Missing and wrong bearer auth rejects do not enqueue.
- Pull after pressure returns exactly the accepted items and no rejected
  payloads.

### Route Isolation

- A full route queue does not block a different route token.
- The isolated route accepts, delivers, and drains its own message while the
  overloaded route remains full.

### Body Limit

- A configured 4-byte body limit rejects a larger body with
  `413 ERR_TOO_LARGE`.
- Pull after the oversized reject returns `204 NO_CONTENT` for that route.

### Auth Under Pressure

- With `RELAY_TOKEN` set, missing bearer auth returns `401 ERR_UNAUTHORIZED`.
- Wrong bearer auth returns `401 ERR_UNAUTHORIZED`.
- Both auth rejects are exercised while the target route is already at queue
  capacity, and neither reject mutates the queue.

### Pull / Drain

- Pulling an overloaded route with a large `max` drains exactly the configured
  cap of accepted messages.
- A repeated pull after drain returns `204 NO_CONTENT`.
- After drain, the route accepts a new message and delivers it normally.

### Logging / No-Secret

- `tests/abuse_rate_queue_logging.rs` captures pressure-path logs and asserts
  that route token, bearer auth, wrong bearer auth, payload, overload payload,
  and reject payload sentinels are absent.
- The same test asserts accepted `x-msg-id` appears only as documented
  non-secret operational metadata.
- Overload logs are asserted to include the structured overload marker without
  secret sentinels.

### Rate-Limit / Global-Cap Evidence

- qsl-server README and relay inbox docs now state explicitly that no in-app
  rate limiting is implemented.
- qsl-server README and relay inbox docs now state explicitly that no global
  route-count cap is implemented.
- The harness checks those statements so the current PR cannot be interpreted
  as a rate-limit/global-cap implementation claim.

### No-Panic

- The new pressure tests complete without panic.
- Existing startup, auth/reject/logging, x-msg-id, config, and relay smoke
  harnesses remain green.

## Results

Passed:

- qsl-server PR #52 merged after required `rust` CI success.
- qsl-server focused harness tests passed.
- qsl-server full locked test suite passed before and after merge.
- qsl-server audit passed before and after merge.

Remaining semantic decisions:

- Whether qsl-server should implement per-client/per-route rate limiting.
- Whether qsl-server should implement a global route-count cap.
- Whether qsl-server should add TTL/retention or persistent queue storage.
- Whether accepted `x-msg-id` should be hashed or otherwise redacted in
  future operational logs.

## No-Production Boundary

NA-0277 provides local executable qsl-server abuse/rate/queue pressure
evidence. It does not deploy qsl-server, expose a public relay, approve public
internet operation, certify production service operation, add production
rate-limiting policy, or change qsl-protocol runtime behavior. qsl-server and
qsl-attachments production gates remain future work.

## Next Recommended Harness

The next recommended qsl-server/attachments-adjacent work is not another
qsl-server source change. Recommended follow-up is queue closeout to the public
README attention refresh and stale branch cleanup audit lane, while preserving
the explicit future gaps for qsl-server rate limiting, global route caps, TTL,
and persistence.
