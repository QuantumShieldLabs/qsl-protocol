Goals: G1, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-13
Replaces: n/a
Superseded-By: n/a

# NA-0279 qsl-server Rate-Limit and Global Route-Cap Design

Directive: QSL-DIR-2026-05-13-082 / NA-0279

## Executive Summary

NA-0279 is a planning-only qsl-server service-hardening lane. It records the
current qsl-server baseline from a read-only audit of the PR #52 main state and
defines the semantic policy plus executable harness shape needed for NA-0280.

No qsl-server implementation, qsl-server harness, qsl-server dependency,
qsl-protocol runtime, protocol, crypto, qsl-attachments, qsc-desktop, website,
workflow, script, Cargo, branch-protection, or public-safety configuration
change is made by NA-0279.

Recommended future policy:

- Preserve current per-route queue-cap semantics.
- Add bounded in-memory rate accounting after successful relay bearer auth and
  route-token syntax validation.
- Use deterministic token-bucket-style limits with a hard burst cap and an
  injectable monotonic clock for tests.
- Add a global cap on live route queues.
- Stop creating live route queues on unknown pulls.
- Delete drained empty queues or expire them deterministically under an idle
  route TTL.
- Keep all rejects deterministic, no-mutation for queues, and no-secret in
  logs.

Current gaps remain explicit: qsl-server does not currently implement in-app
rate limiting, a global route-count cap, or route TTL cleanup.

## Current Baseline from NA-0277

NA-0277 proved the current overload baseline in qsl-server PR #52:

- per-route in-memory FIFO queues exist;
- configured `MAX_QUEUE_DEPTH` is enforced per route;
- a full route returns `429 ERR_OVERLOADED`;
- overloaded pushes do not enqueue extra messages;
- a saturated route remains pullable and drains accepted FIFO items;
- one route token's queue pressure does not block another route token;
- oversized body rejects return `413 ERR_TOO_LARGE` without enqueue;
- missing or wrong bearer auth returns `401 ERR_UNAUTHORIZED` without enqueue;
- pressure logs avoid raw route tokens, bearer auth, and payload bytes;
- accepted `x-msg-id` remains documented non-secret operational metadata;
- no in-app rate limiting is implemented;
- no global route-count cap is implemented.

## qsl-server Read-Only Audit Sources

Read-only qsl-server repo:

- path: `/srv/qbuild/work/NA-0237D/qsl-server`
- repository: `QuantumShieldLabs/qsl-server`
- audited SHA: `75e16e35c399`
- PR baseline: qsl-server PR #52

Inspected sources:

- `src/lib.rs`
- `src/main.rs`
- `tests/relay_smoke.rs`
- `tests/hardening_auth_reject_logging.rs`
- `tests/idempotency_semantics.rs`
- `tests/idempotency_logging.rs`
- `tests/config_semantics.rs`
- `tests/abuse_rate_queue.rs`
- `tests/abuse_rate_queue_logging.rs`
- `README.md`
- `docs/server/DOC-SRV-001_Deployment_Hardening_Contract_v1.0.0_DRAFT.md`
- `docs/server/DOC-SRV-002_Systemd_Hardening_Plan_v1.0.0_DRAFT.md`
- `docs/server/DOC-SRV-003_Relay_Inbox_Contract_v1.0.0_DRAFT.md`
- `docs/server/DOC-SRV-004_Relay_Auth_And_Hardening_Contract_v1.0.0_DRAFT.md`
- `docs/server/DOC-SRV-005_Route_Token_API_Shape_Review_v1.0.0_DRAFT.md`

Read-only validation run on the audited qsl-server SHA:

- `cargo audit --deny warnings`: pass.
- `cargo test --locked`: pass.
- post-validation qsl-server worktree remained clean.

## Current Per-Route Queue Cap Semantics

Current code stores queues in:

- `HashMap<String, VecDeque<(String, Vec<u8>)>>`

The map key is the route token from `X-QSL-Route-Token`; each value is a FIFO
queue of `(msg_id, payload bytes)`.

Current push order:

1. Check optional relay bearer auth.
2. Resolve non-empty route token header.
3. Reject empty body.
4. Reject body larger than `MAX_BODY_BYTES`.
5. Generate or read `x-msg-id`.
6. Lock the queue map.
7. Create the route queue with `entry(...).or_default()`.
8. If queue length is at or above `MAX_QUEUE_DEPTH`, return
   `429 ERR_OVERLOADED`.
9. Otherwise enqueue the message and return 200 with the message id.

The existing tests prove:

- accepted messages are delivered FIFO;
- full route rejects do not enqueue;
- after drain, the route can accept again;
- route pressure is isolated to that route token;
- oversized and auth-rejected pushes do not enqueue.

## Current Body Cap Semantics

Current body cap policy:

- `MAX_BODY_BYTES` defaults to 1 MiB.
- Values above the built-in ceiling are explicitly capped to 1 MiB.
- Non-numeric or zero config fails startup with deterministic config errors.
- A body longer than the effective limit returns `413 ERR_TOO_LARGE`.
- Body-size rejects happen before queue-map locking and before route queue
  creation.

Existing proof:

- `src/lib.rs` unit tests and `tests/relay_smoke.rs` prove the 413 reject.
- `tests/config_semantics.rs` proves default, invalid, capped, and effective
  body-limit behavior.
- `tests/abuse_rate_queue.rs` proves oversized pushes under pressure do not
  mutate queues.

## Current Rate-Limit / Global-Cap Gaps

Current implementation:

- has no per-route request-rate limit;
- has no per-relay-token request-rate limit;
- has no source-IP request-rate limit;
- has no process-wide request-rate limit;
- has no bounded accounting table for rate records;
- has no global route-count cap.

The README and relay inbox contract explicitly state that no in-app rate
limiting and no global route-count cap are implemented. NA-0277 added a test
that asserts those gap statements remain present.

## Current Route Lifecycle / TTL Baseline

Current route lifecycle is queue-map lifetime only:

- An accepted push creates a route queue if it does not already exist.
- A pull for an unknown authenticated route also creates an empty queue because
  `pull_message` uses `entry(...).or_default()`.
- Pull delivery removes messages from the route FIFO.
- Empty route queues are not removed after drain.
- There is no idle timeout.
- There is no TTL cleanup.
- There is no max route-count cleanup path.

This is bounded only by per-route queue depth and payload size, not by the
number of route entries or empty queues.

## Existing Proof Coverage

Current proof coverage includes:

- relay smoke round trip, empty pull, body cap, queue cap, and retired legacy
  routes;
- auth reject no-mutation for missing and wrong bearer auth;
- route-token reject no-mutation for missing or empty route-token headers;
- pull delete-on-deliver and bad `max=0` no-mutation;
- duplicate `x-msg-id` semantics as identifiers rather than idempotency keys;
- `x-msg-id` logging boundary;
- invalid config fail-closed startup behavior;
- per-route overload, drain, route isolation, body reject, auth reject, and
  pressure-log no-secret coverage from NA-0277.

## Missing Proof / Design Gaps

Missing before NA-0280:

- executable rate-limit reject proof;
- executable global route-count cap reject proof;
- deterministic proof that rejected rate/global-cap requests do not enqueue;
- deterministic proof that wrong bearer auth does not create route or rate
  accounting state;
- deterministic proof that unknown pulls no longer create route queues, if that
  lifecycle repair is implemented;
- deterministic route TTL or empty-route cleanup proof;
- bounded accounting proof for rate records and route records;
- no-secret log proof for rate-limit and route-cap rejects;
- explicit config parsing semantics for new limits.

## Threat / Abuse Model

The future implementation must cover these abuse classes:

- route spray: many syntactically valid route tokens try to create unbounded
  empty or non-empty queues;
- burst pushes: a small number of routes receive high-rate pushes that stay
  under body size but overwhelm CPU, memory, or logs;
- unauthorized traffic: missing or wrong bearer auth tries to create queues,
  rate records, or log pressure;
- oversized payloads: bodies larger than the cap must remain pre-queue rejects;
- token guessing: wrong route tokens must not reveal whether another route
  exists through logs or different secret-bearing output;
- queue memory pressure: worst-case memory remains bounded by route cap, queue
  depth, and body size;
- log pressure: rate/global-cap rejects remain bounded and secret-free;
- route lifecycle abuse: empty pulls and drained queues cannot permanently
  consume route slots.

## Recommended Rate-Limit Design

Dimension policy:

- Primary in-app dimension: per route token for canonical push.
- Process-wide dimension: global push bucket to cap aggregate accepted push
  pressure.
- Optional auth-mode dimension: when `RELAY_TOKEN` is set, a shared bearer-auth
  bucket may cap all authenticated relay traffic after auth succeeds.
- Pull rate limiting should be separately configurable from push rate limiting
  because pulls do not add payload bytes but can still create CPU/log pressure.
- Source-IP limits should remain an upstream proxy/load-balancer concern for
  the first in-app implementation because direct IP identity is proxy-sensitive
  and harder to test deterministically without defining trusted-forwarded-header
  policy.

Accounting order:

1. Reject missing or wrong bearer auth before route, queue, or rate accounting.
2. Reject missing or malformed route token before per-route accounting.
3. Reject empty or oversized bodies before queue mutation.
4. Apply route-count admission only when a request would create a new live
   route queue.
5. Apply rate buckets before enqueue.
6. Enqueue only after all checks pass.

Algorithm:

- Use token-bucket-style accounting with capacity as the hard burst cap.
- Refill by monotonic time.
- Keep buckets in bounded in-memory state.
- Require an injectable or controllable clock for deterministic tests before
  claiming time-based refill behavior.
- Use small config values in tests so rejects are reached without long loops.

Initial recommended config names for the later implementation lane:

- `MAX_ROUTE_COUNT`
- `ROUTE_IDLE_TTL_SECS`
- `PUSH_RATE_BURST`
- `PUSH_RATE_REFILL_PER_SEC`
- `PULL_RATE_BURST`
- `PULL_RATE_REFILL_PER_SEC`

The exact config names can change in NA-0280 if the implementation uses the
same documented semantics and executable tests.

## Recommended Global Route-Cap Design

Policy:

- Add a maximum number of live route queues.
- Count only live route queues, not unauthorized attempts.
- A new live route queue is created only by an accepted push to a route that
  does not already exist.
- Pulling an unknown route should return 204 without creating a route queue.
- A push to a new route when the live route cap is reached should return
  `429 ERR_ROUTE_CAP`.
- Existing routes continue to use their own queue-depth and rate checks.
- A full existing route still returns `429 ERR_OVERLOADED`, not
  `ERR_ROUTE_CAP`.

Route cap check order:

1. Auth success.
2. Route-token syntax success.
3. Body is non-empty and within body limit.
4. If route exists, proceed to rate and queue-depth checks.
5. If route does not exist and live route count is at cap, reject
   `429 ERR_ROUTE_CAP` without creating any route or rate record.
6. If route does not exist and the cap has room, create the route only when the
   request will be accepted.

## Recommended Route Lifecycle / TTL Design

Recommended lifecycle:

- Unknown pull: return 204 without route creation.
- Accepted push: creates route if absent and cap allows it.
- Pull delivery: delete delivered messages.
- Empty after pull: remove the route immediately unless a later directive
  explicitly chooses idle-empty retention for compatibility.
- Non-empty route: keep until drained or idle TTL expires.
- Idle TTL cleanup: run opportunistically on push/pull before admission and
  after drain, using a monotonic clock.
- Rate-accounting TTL: expire inactive buckets so rate records remain bounded
  even when route queues are removed.

This is a behavior change from current empty-queue retention and unknown-pull
route creation, so NA-0280 must include executable tests if it implements it.

## Reject / Error Semantics

Recommended future rejects:

- `429 ERR_RATE_LIMITED` for rate-bucket exhaustion.
- `429 ERR_ROUTE_CAP` for a new route rejected by the global live-route cap.
- Existing `429 ERR_OVERLOADED` remains the per-route queue-depth reject.
- Existing `413 ERR_TOO_LARGE` remains the body-size reject.
- Existing `401 ERR_UNAUTHORIZED` remains the missing/wrong bearer reject.
- Existing `400 ERR_MISSING_ROUTE_TOKEN`, `400 ERR_BAD_ROUTE_TOKEN`,
  `400 ERR_EMPTY_BODY`, and `400 ERR_BAD_MAX` remain unchanged unless a later
  directive explicitly changes them with tests.

No-mutation requirements:

- Rate-limit rejects do not enqueue.
- Route-cap rejects do not create a route queue.
- Wrong bearer auth creates no queue, no route slot, and no privileged
  per-route accounting state.
- Oversized bodies create no route queue and do not enqueue.
- Unknown pulls do not create route queues once the lifecycle repair lands.

## Logging / No-Secret Requirements

Rate/global-cap logging must never include:

- raw route tokens;
- raw bearer tokens;
- `Authorization` header values;
- payload bytes;
- secret-bearing URL material.

Allowed logging fields:

- bounded event names such as `event=rate_limited` and `event=route_cap`;
- status/error code;
- queue depth and configured max where not secret-bearing;
- redacted deterministic route id such as the existing short channel id;
- accepted `x-msg-id` only under the existing documented non-secret metadata
  boundary.

The NA-0280 harness must capture logs for rate/global-cap rejects and assert
that route, auth, and payload sentinels are absent.

## Memory-Bound Requirements

The future implementation must make these bounds explicit:

- maximum live route queues: `MAX_ROUTE_COUNT`;
- maximum messages per live route: `MAX_QUEUE_DEPTH`;
- maximum bytes per accepted message: `MAX_BODY_BYTES`;
- maximum route-rate accounting records: bounded by live routes plus a small
  configured retention/cleanup margin;
- maximum process/global buckets: constant-size;
- maximum auth-mode buckets: constant-size for current single-token auth mode.

Worst-case queued payload memory is bounded by:

```text
MAX_ROUTE_COUNT * MAX_QUEUE_DEPTH * MAX_BODY_BYTES
```

This formula is a ceiling model, not a production sizing claim. Any public
deployment profile still needs separate operational memory limits and upstream
controls.

## Auth Interaction

Preserve the current fail-closed auth ordering:

- When `RELAY_TOKEN` is set, missing or wrong bearer auth rejects before route
  lookup, route creation, queue mutation, or privileged per-route accounting.
- When `RELAY_TOKEN` is unset, route-token checks still apply and rate/global
  caps provide in-app resource bounds, but this does not replace upstream
  network access control.
- `Authorization: Bearer ...` remains relay auth only and must not become a
  route token.

## Implementation Prerequisites

NA-0280 should not implement production service posture. It should first add a
deterministic local harness and the minimal qsl-server implementation needed
to make that harness pass.

Prerequisites before implementation claims:

- configuration semantics for new limits;
- deterministic clock or controllable refill mechanism for time-based tests;
- route cleanup helper with no unknown-pull route creation;
- bounded accounting structure;
- log capture tests for new reject paths;
- explicit docs that rate/global-cap behavior is local in-app hardening and
  not a deployment approval.

## NA-0280 Executable Harness Plan

The first executable harness should use loopback-only qsl-server tests with
small limits:

1. `global_route_cap_rejects_new_routes_without_mutation`
   - configure route cap 1;
   - accept one route;
   - reject a second route with `429 ERR_ROUTE_CAP`;
   - prove the second route is not pullable;
   - prove the first route still drains normally.
2. `unknown_pull_does_not_create_route_slot`
   - configure route cap 1;
   - pull an unknown route and get 204;
   - push a new route and confirm it is accepted;
   - this proves the unknown pull did not consume the cap.
3. `rate_limited_push_rejects_without_enqueue`
   - configure a burst of 1 with no immediate refill or a fake clock;
   - accept first push;
   - reject second push with `429 ERR_RATE_LIMITED`;
   - drain and prove only the accepted message exists.
4. `wrong_auth_does_not_create_route_or_rate_state`
   - enable `RELAY_TOKEN`;
   - send wrong bearer auth with a new route;
   - then fill route cap with a legitimate route;
   - prove the wrong-auth route did not consume a route slot.
5. `oversize_does_not_create_route_or_rate_state`
   - configure tiny body cap and route cap;
   - reject oversized new route;
   - accept another route afterward.
6. `drain_or_ttl_releases_route_slot`
   - accept and drain route A;
   - accept route B under cap 1 after route A is empty, or advance fake clock
     and run cleanup if idle retention is chosen.
7. `rate_and_route_cap_logs_are_secret_free`
   - capture logs across rate-limit, route-cap, auth-reject, and oversize
     paths;
   - assert no raw route token, bearer token, auth header, or payload sentinel
     appears.
8. `config_semantics_for_new_limits_are_fail_closed`
   - missing values use documented defaults;
   - non-numeric or zero critical values fail startup;
   - above-ceiling values are capped or rejected according to the chosen
     documented policy.

All tests must be bounded, local, deterministic, and independent of public
network access.

## Alternatives Rejected

- Jump directly from NA-0277 to implementation without semantic design:
  rejected because rate/global-cap behavior changes lifecycle, memory, and
  reject semantics.
- Rely only on reverse-proxy policy: rejected because in-app queue and route
  resource bounds are still needed for local deterministic proof.
- Source-IP-only in-app rate limiting: rejected for the first lane because
  trusted client IP identity depends on proxy/header policy and is not yet
  specified.
- Fixed-window-only rate limiting: rejected as the primary design because
  window edges permit avoidable bursts; a token bucket gives a simple hard
  burst cap with smoother refill.
- Unbounded per-route accounting: rejected because it would replace one memory
  pressure path with another.
- Treat empty pulls as route creation: rejected for the future cap model
  because route spray could consume slots without storing payloads.
- Claim abuse resistance from per-route queue caps alone: rejected because
  queue depth does not bound route count or request rate.

## Non-Production / No-Production-Readiness Boundary

NA-0279 is planning evidence only. It does not implement rate limiting, global
route caps, route TTL cleanup, public exposure, production deployment,
operational proxy policy, incident response, or external service review. Any
future public-service statement must wait for separate executable qsl-server
proof plus deployment-boundary evidence.

## Explicitly Not Implemented in NA-0279

NA-0279 does not implement:

- qsl-server rate limiting;
- qsl-server global route-count caps;
- qsl-server route TTL or cleanup;
- qsl-server tests or harnesses;
- qsl-server Cargo/dependency changes;
- qsl-server workflow changes;
- qsl-attachments changes;
- qsl-protocol runtime/protocol/crypto changes;
- qsc/qsl runtime changes;
- website or external website changes;
- public-safety or branch-protection changes;
- NA-0280 executable harness work.
