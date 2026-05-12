Goals: G1, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-12
Replaces: n/a
Superseded-By: n/a

# NA-0270 qsl-server Read-Only Audit and Test-Harness Design

Directive: QSL-DIR-2026-05-12-071 / NA-0270

## Executive Summary

NA-0270 performed a read-only audit of the local `qsl-server` sibling
worktree and designed the first executable hardening harness for a future
implementation lane. No `qsl-server` files were changed, no service behavior was
changed, and this document makes no production readiness claim.

The current server already has useful transport-only relay evidence: canonical
header-carried route tokens, optional bearer auth, queue and body limits,
delete-on-deliver pulls, route-token redaction, and no-payload logging tests.
The audit also found concrete documentation/API-contract mismatches and several
gaps that should block stronger service claims until executable harnesses cover
them.

## Repo Identity and Status

- Primary inspected path: `/srv/qbuild/work/NA-0237D/qsl-server`.
- Alternate `/home/victor/work/qsl/qsl-server`: absent.
- Other local sibling copies: `/srv/qbuild/work/NA-0237*/qsl-server`.
- Worktree status: clean.
- Branch/status: `main...mirror/main`.
- HEAD: `0826ffa4d6f3` (`Merge pull request #46 from QuantumShieldLabs/na-0012/governance-closeout`).
- Remotes: `origin=https://github.com/QuantumShieldLabs/qsl-server.git`, `mirror=/srv/qbuild/mirrors/qsl-server.git`.
- Build system/language: Rust 2021 / Cargo, Axum/Tokio service.
- Audit mode: read-only; no fetch, build, test, branch creation, staging, or file mutation in the sibling repo.

## Service Role

`qsl-server` is a transport-only relay. It stores and forwards opaque payload
bytes keyed by a route token. It must not parse QSP protocol messages, perform
cryptography, inspect payload contents, or become an attachment-service runtime.

The implementation stores queues in process memory:

- `HashMap<String, VecDeque<(String, Vec<u8>)>>` guarded by `Mutex`.
- Route token selects the queue.
- Push stores `(message_id, payload_bytes)`.
- Pull deletes messages on delivery.

## Current Evidence Baseline

Observed evidence in the sibling repo:

- `README.md` states transport-only, no protocol parsing, no crypto, fail-closed deterministic errors, and no secret/payload logging.
- `src/lib.rs` implements `POST /v1/push` and `GET /v1/pull`.
- `src/main.rs` defaults to loopback bind and caps body/queue limits.
- `tests/relay_smoke.rs` covers basic push/pull, empty pull, oversize, queue full, retired legacy paths, and missing/empty route-token header rejects.
- `src/lib.rs` unit tests cover queue deletion, payload log redaction, route-token log redaction, overload logging, and optional bearer auth.
- `tests/no_secrets_examples.rs` checks packaging/runbook examples for token-like bearer literals.
- Packaging exists for systemd and Caddy with loopback bind and `/v1/*` access-log suppression guidance.

This evidence is service-hardening groundwork, not production proof.

## API Surface Inventory

Implemented routes:

- `POST /v1/push`
  - Required header: `X-QSL-Route-Token`.
  - Optional header: `x-msg-id`.
  - Body: raw opaque bytes.
  - Success: `200` JSON `{ "id": "<msg_id>" }`.
  - Rejects: `401 ERR_UNAUTHORIZED`, `400 ERR_MISSING_ROUTE_TOKEN`, `400 ERR_BAD_ROUTE_TOKEN`, `400 ERR_EMPTY_BODY`, `413 ERR_TOO_LARGE`, `429 ERR_OVERLOADED`, `500 ERR_LOCK_POISON`.

- `GET /v1/pull?max=N`
  - Required header: `X-QSL-Route-Token`.
  - Success with items: `200` JSON `{ "items": [ { "id": "...", "data": [u8...] } ] }`.
  - Empty queue: `204`.
  - Rejects: `401 ERR_UNAUTHORIZED`, `400 ERR_MISSING_ROUTE_TOKEN`, `400 ERR_BAD_ROUTE_TOKEN`, `400 ERR_BAD_MAX`, `500 ERR_LOCK_POISON`.

Retired legacy routes:

- `POST /v1/push/:channel`: not routed; current tests expect `404`.
- `GET /v1/pull/:channel?max=N`: not routed; current tests expect `404`.

## Auth/Token Model

- Relay auth is optional and controlled by `RELAY_TOKEN`.
- If `RELAY_TOKEN` is unset or empty, auth is disabled.
- If set, `Authorization: Bearer <token>` is required for push and pull.
- Wrong or missing bearer token returns `401 ERR_UNAUTHORIZED`.
- Route selection is separate from bearer auth and uses `X-QSL-Route-Token`.
- Missing/empty route token returns `400 ERR_MISSING_ROUTE_TOKEN`.
- Non-UTF-8 route-token header maps to `400 ERR_BAD_ROUTE_TOKEN` in code, but the audit did not find an executable test for this case.

## Queue/Body/Overload Model

- Default and ceiling body limit: `1 MiB`.
- Default and ceiling queue depth: `256`.
- Empty push body returns `400 ERR_EMPTY_BODY`.
- Oversized push returns `413 ERR_TOO_LARGE` before queue mutation.
- Queue-full push returns `429 ERR_OVERLOADED` before queue mutation.
- Pull `max=0` returns `400 ERR_BAD_MAX`.
- Pull `max` is capped to queue depth.
- Pull deletes delivered messages from the in-memory queue.

## Logging/No-Secret Model

Application log points are limited:

- startup: `qsl-server listening on <addr>`;
- startup/config failures: deterministic error code strings;
- overload: `event=overloaded queue_depth=<n> max=<n>`;
- push/pull success: redacted `channel_id`, message id, and byte count.

Current tests prove payload bytes and raw route tokens are not emitted by
application logs in selected success/overload paths. They do not yet prove that
authorization headers, malformed route-token input, all reject paths, reverse
proxy logs, or retained operational evidence are secret-free.

## Storage/State Model

- Storage is in-memory only.
- Restart loses queued messages.
- No disk persistence is implemented.
- No TTL cleanup is implemented.
- Queue depth bounds memory per route token, but there is no global queue cap or
route count cap.
- Pull is delete-on-deliver.

## Deployment/Network Assumptions

- Default bind is `127.0.0.1`.
- `0.0.0.0` bind is allowed only by explicit CLI/env configuration.
- qsl-server is HTTP-only; TLS termination is expected upstream.
- Caddy example keeps `/v1/*` access logs skipped/sanitized.
- Systemd unit runs as `qslrelay` with hardening directives.
- No in-app health endpoint or metrics endpoint is implemented.
- Deployment verification uses systemd status and authenticated relay probes, not a dedicated health route.

## Existing Tests

The audit found these executable categories:

- smoke: default limits are nonzero;
- relay integration: push/pull, empty pull, oversize, queue full, legacy route retirement, missing/empty route-token rejects;
- library unit/integration: delete-on-deliver, payload-not-logged, route-token redaction, overload log shape, optional bearer auth enabled/disabled;
- CLI/config unit tests: CLI/env precedence, caps, loopback default, explicit public bind, invalid port env reject;
- example hygiene: no token-like `RELAY_TOKEN` in env example and no token-like bearer literals in packaging/runbook.

## Proven Bugs

1. Queue-full error-name mismatch.
   - `src/lib.rs` and current tests return/use `ERR_OVERLOADED` for queue full.
   - `README.md` also documents `ERR_OVERLOADED`.
   - `docs/server/DOC-SRV-003_Relay_Inbox_Contract_v1.0.0_DRAFT.md`, `docs/server/DOC-SRV-004_Relay_Auth_And_Hardening_Contract_v1.0.0_DRAFT.md`, and `tests/NA-0004_relay_auth_hardening_plan.md` still name `ERR_QUEUE_FULL`.
   - Severity: medium documentation/contract mismatch. A future lane must choose the canonical token and update docs/tests consistently.

2. Pull response shape mismatch in README.
   - `README.md` describes canonical pull as returning the oldest message bytes.
   - `src/lib.rs` returns JSON with an `items` array, and `tests/relay_smoke.rs` deserializes that JSON shape.
   - Severity: medium API documentation mismatch.

3. Legacy route-token migration document is stale.
   - `docs/server/DOC-SRV-005_Route_Token_API_Shape_Review_v1.0.0_DRAFT.md` still says legacy path-based routes remain accepted during a compatibility window.
   - Current `src/lib.rs` only routes token-free `/v1/push` and `/v1/pull`, and `tests/relay_smoke.rs` proves legacy path routes return `404` without queue mutation.
   - Severity: medium stale design/contract text that can mislead deployment and client validation.

4. Auth stance in deployment contract is stale.
   - `docs/server/DOC-SRV-001_Deployment_Hardening_Contract_v1.0.0_DRAFT.md` says the current relay-layer stance is no authentication.
   - Current implementation and `DOC-SRV-004` support optional `RELAY_TOKEN` bearer auth.
   - Severity: low-to-medium deployment-contract mismatch.

5. Invalid numeric limit env values silently fall back.
   - `PORT` env parse failure exits with `ERR_INVALID_ENV_PORT`.
   - `MAX_BODY_BYTES` and `MAX_QUEUE_DEPTH` env parse failures are ignored by `env_usize` and fall back through default resolution.
   - Severity: medium config fail-closed gap to resolve in a future implementation lane; this audit does not change semantics.

6. Message-id idempotency contract is unproven/inconsistent.
   - `DOC-SRV-003` labels `message_id` as a client-chosen idempotency key.
   - `src/lib.rs` accepts `x-msg-id` but appends every push; no deduplication or duplicate-replay behavior is implemented or tested.
   - Severity: medium contract ambiguity until a future lane either implements idempotency or documents duplicate acceptance explicitly.

## Evidence Gaps

- No executable test for malformed/non-UTF-8 route-token header.
- No executable test for `ERR_BAD_MAX`.
- No executable test proving wrong bearer auth leaves a pre-existing queue item untouched.
- No executable test proving Authorization header values are absent from logs.
- No executable test proving payloads are absent from reject-path logs.
- No global queue cap or route-count cap evidence.
- No rate limiting, retry/backoff, or abuse/spam harness.
- No TTL, retention, cleanup, persistence, restart/recovery, or backup/restore proof.
- No dedicated health/metrics/observability endpoint.
- No request timeout proof in-app; docs push this to upstream proxy/load balancer.
- No cross-host qsl-server service harness in this lane.
- No proof that deployment scripts never expose real token values under all failure modes.
- No proof that proxy/access logs are secret-free beyond static Caddy example checks.
- No long-running soak/stress proof for many route tokens, large queues, or concurrent push/pull bursts.

## Recommendations

- Build the first executable qsl-server hardening harness before any stronger service claim.
- Fix the proven docs/contract mismatches in a bounded future lane.
- Decide duplicate `x-msg-id` semantics before using idempotency or replay-resistance wording.
- Add fail-closed validation for invalid `MAX_BODY_BYTES` and `MAX_QUEUE_DEPTH` if that is the intended production posture.
- Add a safe health/ops story, either through a dedicated health endpoint or explicit deployment-level probe contract.
- Keep production service wording blocked until auth, rate limit, logging, queue, deployment, persistence, and observability gaps have executable proof.

## Recommended Test Harness

The first harness should be a qsl-server executable contract suite in the
sibling repo, with deterministic local loopback tests and log capture. It should
start the Axum app with explicit limits/auth, use HTTP clients against loopback,
capture application logs, and assert both HTTP behavior and no-mutation/no-leak
invariants.

### Auth Tests

- Missing `Authorization` with `RELAY_TOKEN` set returns `401 ERR_UNAUTHORIZED`.
- Wrong bearer token returns `401 ERR_UNAUTHORIZED`.
- Malformed bearer scheme returns `401 ERR_UNAUTHORIZED`.
- Correct bearer token allows push/pull.
- Wrong/missing bearer does not mutate a pre-existing queue.
- Wrong or missing `X-QSL-Route-Token` rejects deterministically.
- Auth header value is absent from captured logs and retained test artifacts.

### Route Tests

- Unknown `/v1/*` route returns `404` and does not mutate queues.
- Retired path-token push/pull remain rejected.
- Malformed/non-UTF-8 route-token header returns `ERR_BAD_ROUTE_TOKEN`.
- Empty/whitespace route-token header returns `ERR_MISSING_ROUTE_TOKEN`.
- Repeated route tokens isolate queues correctly from other route tokens.

### Payload Tests

- Empty body returns `400 ERR_EMPTY_BODY` and no mutation.
- Oversized body returns `413 ERR_TOO_LARGE` and no mutation.
- Opaque binary payload round-trips through JSON `data` without interpretation.
- Payload bytes are absent from success and reject logs.
- Malformed content type or absent content type does not cause protocol parsing.

### Queue Tests

- Queue cap returns the canonical overload code consistently.
- Queue-full reject does not displace or mutate queued items.
- Pull from empty queue returns `204`.
- `max=0` returns `400 ERR_BAD_MAX`.
- `max` above queue cap is bounded.
- Queue isolation holds across multiple route tokens.

### Replay/Idempotency Tests

- Duplicate push with the same `x-msg-id` has explicitly chosen semantics.
- Repeated pull after delete-on-deliver returns `204`.
- Server-generated message IDs are non-empty and unique across a bounded sample.
- Unknown route-token pull does not create visible state beyond an empty queue.

### Logging Tests

- Authorization header absent from logs.
- Raw route token absent from logs.
- Payload absent from logs.
- Error responses do not echo bearer tokens, route tokens, or payload bytes.
- Overload and reject logs stay structured and avoid long secret-like values.

### Config/Startup Tests

- Missing optional env uses safe defaults.
- Invalid `PORT` exits fail-closed with deterministic error.
- Invalid `MAX_BODY_BYTES` has an explicit tested behavior.
- Invalid `MAX_QUEUE_DEPTH` has an explicit tested behavior.
- Values above ceilings are capped or rejected according to the future decision.
- Default bind is loopback.
- Public bind is explicit opt-in only.

### Health/Ops Tests

- If a health endpoint is added, prove it does not expose tokens, queue names, payload bytes, or sensitive config.
- If no health endpoint is added, prove the deployment probe contract remains explicit.
- Record the metrics/observability gap if no endpoint exists.

### Soak/Stress Tests

- Bounded push/pull bursts across several route tokens.
- Bounded overload behavior without panic.
- Bounded concurrent push/pull with queue depth invariants.
- No unbounded route growth in the declared stress profile, or explicit stop if the design still lacks a global cap.
- Log/artifact leak scan after stress.

## Proposed Future Implementation Lanes

1. qsl-server docs/API contract repair: align queue-full error name, pull response shape, route-token migration status, auth stance, and idempotency wording.
2. qsl-server hardening harness v1: add executable auth/route/payload/queue/log/config tests without changing service semantics except where a proven mismatch requires an explicit decision.
3. qsl-server config fail-closed and replay/idempotency decision: implement any chosen config/idempotency behavior with tests.
4. qsl-server abuse and operations harness: rate limit, global resource cap, soak/stress, health/metrics, and deployment log proof.
5. qsl-server deployment profile review: TLS termination, proxy headers, access logs, firewall/ACL, systemd hardening, retention/restart behavior, incident response, and external review prerequisites.

## Non-Production / No-Production-Readiness Boundary

NA-0270 is read-only audit and harness design only. It does not authorize
deployment, public internet exposure, production relay operation, service
hardening implementation, dependency updates, workflow changes, branch
protection changes, qsl-attachments changes, protocol changes, crypto changes,
website changes, or any production readiness claim.

Known qsl-server gaps remain visible and must not be hidden by future public
wording.

## Stop Conditions for Future Implementation

Future qsl-server implementation must stop if:

- a change would weaken auth, logging, fail-closed, no-mutation, or route-token hygiene;
- a reject path mutates a queue unless explicitly designed and tested;
- a test emits bearer tokens, route tokens, payload bytes, secret-bearing URLs, or long secret-like values;
- protocol parsing, cryptography, qsl-attachments semantics, or qsc behavior would move into qsl-server;
- public exposure, TLS, DNS, Cloudflare, firewall, or deployment mutation is needed without explicit authority;
- Cargo/dependency or workflow changes are needed outside the active lane;
- production readiness wording would be required before executable proof exists;
- required CI fails without an understood in-scope recovery.
