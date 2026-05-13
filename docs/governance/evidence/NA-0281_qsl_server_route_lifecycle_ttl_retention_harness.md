Goals: G1, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-13
Replaces: n/a
Superseded-By: n/a

# NA-0281 qsl-server Route Lifecycle / TTL / Retention Harness Evidence

Directive: QSL-DIR-2026-05-13-084 / NA-0281

## Executive Summary

NA-0281 implements and test-backs deterministic qsl-server route lifecycle,
idle TTL, and retention behavior.

qsl-server PR #54 merged the executable lifecycle / TTL / retention harness
and a minimal qsl-server implementation. qsl-protocol records evidence only in
this PR: no qsl-protocol runtime, protocol, crypto, qsl-attachments,
qsc-desktop, website, workflow, script, Cargo, branch-protection,
public-safety, or dependency behavior changes are made here.

The qsl-server controls remain local deterministic hardening primitives. They
do not approve public exposure, production relay operation, qsl-attachments
service operation, or production deployment. Reverse proxy / edge controls,
long-running operations review, and external review remain separate gates.

## NA-0280 Semantics Carried Forward

NA-0281 preserves the NA-0280 route-cap and rate semantics:

- `MAX_ROUTE_COUNT` bounds live route slots.
- Accepted pushes to new routes create live slots only when the cap allows.
- Pushes to new routes beyond the live-route cap return `429 ERR_ROUTE_CAP`.
- Unknown pulls return 204 without creating route slots.
- Pull delivery removes messages; draining a route to empty removes the live
  route slot and its per-route rate bucket.
- `PUSH_RATE_BURST` and `PUSH_RATE_REFILL_PER_SEC` implement bounded
  in-memory per-route push token-bucket accounting.
- Exhausted per-route push tokens return `429 ERR_RATE_LIMITED`.
- Existing per-route queue overload remains `429 ERR_OVERLOADED`.
- Existing body-size rejects remain `413 ERR_TOO_LARGE`.
- Existing missing/wrong bearer rejects remain `401 ERR_UNAUTHORIZED`.

## qsl-server PR Evidence

- Repository: `QuantumShieldLabs/qsl-server`
- PR: #54, `https://github.com/QuantumShieldLabs/qsl-server/pull/54`
- Branch: `na-0281-route-lifecycle-ttl-retention`
- Head SHA: `d5e6e5213a52`
- Merge SHA: `3f28d7433e74`
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
- `tests/config_semantics.rs`
- `tests/route_lifecycle_ttl.rs`
- `tests/route_lifecycle_ttl_logging.rs`

## Chosen Lifecycle / TTL / Retention Semantics

Config knob added in qsl-server:

- `ROUTE_IDLE_TTL_MS`: default 300000 ms; values above 86400000 ms are capped;
  non-numeric and zero values fail startup deterministically with
  `ERR_INVALID_CONFIG_ROUTE_IDLE_TTL_MS`.

Behavior:

- Time-based idle TTL applies to live route state, including non-empty routes.
- Cleanup is deterministic and access-triggered: canonical push/pull runs TTL
  cleanup after auth, route-token, body-size, and pull-`max` validation.
- Expired routes are removed before the current accepted push/pull is
  evaluated.
- Expired queued messages are discarded and are not returned by later pulls.
- Expiry releases the global route slot and the per-route rate bucket.
- Push after expiry creates a fresh route state if the request otherwise
  passes existing caps and validation.
- Pull after expiry returns 204 for the expired route.
- Cleanup logs use the redacted route identifier plus bounded counts only.

No background scheduler, persistent store, external service, dependency update,
workflow update, protocol change, or qsl-protocol runtime change was added.

## Harness Coverage

Unknown pull no slot:

- `unknown_pull_does_not_create_route_slot` proves unknown pull returns 204 and
  does not consume the only live route slot.

Accepted push creates slot:

- The route-cap and TTL tests prove accepted push creates the route state that
  later pulls, route-cap checks, and TTL cleanup observe.

Drain-to-empty releases slot/accounting:

- `drain_to_empty_releases_route_slot_and_rate_bucket` accepts a route under
  route cap 1 and burst 1, proves the second push is rate limited, drains the
  route, then proves the same route can be pushed again with a fresh rate
  bucket and another route can later use the released slot.

TTL / idle cleanup:

- `idle_route_ttl_releases_capacity` accepts route A under route cap 1, waits
  beyond the short test TTL, then accepts route B. Route A returns 204 after
  cleanup, proving expired route removal and capacity release.
- `expired_route_releases_rate_bucket` proves the per-route rate bucket is
  removed with the expired route and a later push to the same route starts with
  fresh accounting.

Stale message retention:

- `expired_route_does_not_return_stale_message` proves an expired non-empty
  route returns 204 rather than stale data.
- `push_after_expiry_does_not_resurrect_old_messages` proves a fresh push after
  expiry returns only the fresh message and does not resurrect expired queued
  bytes.

Route-cap / rate interaction:

- Existing NA-0280 `rate_global_cap` tests remain green.
- New TTL tests prove expiry releases route capacity before cap evaluation and
  releases per-route push-rate state.

Logging / no-secret:

- `ttl_cleanup_logs_redact_route_auth_payload` captures expiry logs and proves
  raw route tokens, bearer values, the `Authorization` header word, and payload
  sentinels are absent. The log includes only `event=route_expired`,
  `channel_id=`, `queued_messages=`, and `ttl_ms=`.

No-panic:

- Cleanup, expiry, rate, route-cap, auth reject, body reject, pull, and drain
  paths are exercised by local harnesses without panic.

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
- `cargo test --locked --test route_lifecycle_ttl -- --test-threads=1`
- `cargo test --locked --test route_lifecycle_ttl_logging -- --test-threads=1`
- `cargo test --locked`
- `cargo clippy --locked --all-targets -- -D warnings`
- `cargo audit --deny warnings`
- `git diff --check`

qsl-server CI:

- PR #54 required `rust` check completed success.
- PR #54 merged normally with a merge commit and matched head SHA.

Recovered local failures:

- A clean qsl-protocol worktree initially parsed stale local `main` content
  before being fast-forwarded to required `origin/main`. Classification:
  recoverable local checkout state. Corrective action: fast-forwarded the
  clean worktree. Final result: READY NA-0281 and D-0531/D-0532 handoff proof
  matched.
- `cargo fmt --check` reported mechanical formatting differences during
  qsl-server implementation. Classification: recoverable local formatting
  issue. Corrective action: ran `cargo fmt`. Final result: formatting check
  passed.
- `gh pr create` on this host did not support `--json`. Classification:
  recoverable CLI-shape issue. Corrective action: created the PR without that
  flag and fetched PR metadata separately. Final result: qsl-server PR #54 was
  opened, checked, and merged.
- The first qsl-server check polling snippet printed a non-fatal Python
  status-line syntax error while the counter-based poll still completed
  successfully. Corrective action: fetched check-run details separately. Final
  result: required `rust` check was completed success.

## No-Production Boundary

This evidence proves local qsl-server executable lifecycle / TTL / retention
behavior only. It does not approve public exposure, production relay
operation, qsl-attachments service operation, production deployment, edge
policy, incident response, external service review, or operational memory
sizing.

Remaining deployment and service-hardening gaps remain visible:

- process-wide, auth-mode, source-IP, and pull rate limits;
- deployment-layer reverse proxy / edge rate limiting;
- long-running retention/cleanup soak beyond the local deterministic harness;
- qsl-attachments retention / cleanup / recovery proof;
- public exposure readiness;
- external review and operational runbook evidence.

## Next Recommended Harness

The next recommended service-hardening lane is NA-0282:

`qsl-attachments Retention / Cleanup / Recovery Harness`

Recommended first proofs:

- deterministic qsl-attachments retention/cleanup behavior;
- restart/recovery boundaries;
- no persistence on reject;
- no capability, descriptor, ciphertext, plaintext, or auth leakage in logs;
- dependency, test, and audit health in the qsl-attachments repo.
