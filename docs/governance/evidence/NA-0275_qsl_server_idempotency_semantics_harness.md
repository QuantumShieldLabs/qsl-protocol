Goals: G1, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-13
Replaces: n/a
Superseded-By: n/a

# NA-0275 qsl-server x-msg-id / Idempotency Semantics Harness

Directive: QSL-DIR-2026-05-13-078 / NA-0275

## Executive Summary

NA-0275 records and test-backs the current qsl-server `x-msg-id` semantics.
The selected semantics are:

`x-msg-id` is a client-supplied message identifier, not an idempotency key.
Each accepted duplicate `x-msg-id` push appends a separate FIFO queue item.

qsl-server PR #50 merged an executable local/loopback harness that proves
duplicate `x-msg-id` behavior, mixed supplied and server-generated IDs,
auth/oversize/queue-cap reject no-mutation behavior, pull/delete behavior,
logging/no-secret boundaries, and blank `x-msg-id` no-panic behavior. The PR
also clarifies qsl-server README and relay contract docs to state that accepted
message IDs are non-secret operational metadata and duplicate IDs are not
deduplicated.

No qsl-server service source, dependency, workflow, deployment, or packaging
file changed. No qsl-protocol runtime, protocol, crypto, state-machine,
qsl-attachments, qsc-desktop, website, workflow, script, Cargo, dependency,
branch-protection, or public-safety configuration path changed in this
evidence lane. This evidence does not claim production readiness.

## qsl-server PR Evidence

- Repository: `QuantumShieldLabs/qsl-server`
- PR: #50, `NA-0275: add qsl-server x-msg-id semantics harness`
- PR URL: https://github.com/QuantumShieldLabs/qsl-server/pull/50
- Head SHA: `9a8cb69af099`
- Merge SHA: `0429763ef125`
- Merged at: 2026-05-13T03:21:46Z
- Required check: `rust` success before merge.
- Required check URL:
  https://github.com/QuantumShieldLabs/qsl-server/actions/runs/25776195797/job/75709226121
- Changed paths:
  - `README.md`
  - `docs/server/DOC-SRV-003_Relay_Inbox_Contract_v1.0.0_DRAFT.md`
  - `tests/idempotency_logging.rs`
  - `tests/idempotency_semantics.rs`
- Implementation changed: no.
- Dependency files changed: no.
- Workflow changed: no.
- Production-readiness claim: no.
- Harness is loopback/local only.

Chosen `x-msg-id` semantics:

- Supplied `x-msg-id` values are opaque message identifiers.
- Supplied `x-msg-id` values are not idempotency keys.
- Duplicate supplied IDs are accepted as separate queue entries when each push
  otherwise passes auth, route-token, body-size, and queue-depth checks.
- FIFO ordering and payload association are preserved.
- Pull is delete-on-deliver; after duplicate-ID items are delivered, repeated
  pull returns empty deterministically.
- Future idempotency or deduplication remains a separate semantic hardening
  lane if product design later requires it.

Validation before qsl-server PR merge:

- `cargo fmt --check`: pass after formatting recovery.
- `cargo test --locked --test hardening_auth_reject_logging -- --test-threads=1`:
  pass, 7 tests.
- `cargo test --locked --test idempotency_semantics -- --test-threads=1`:
  pass, 4 tests.
- `cargo test --locked --test idempotency_logging -- --test-threads=1`:
  pass, 1 test.
- `cargo test --locked`: pass after one bounded retry of an existing
  default-parallel logging capture flake.
- `cargo audit --deny warnings`: pass.
- `git diff --check`: pass.
- Changed-file overclaim scan: pass.
- Changed-file leak/secret shape scan: pass; only test sentinel literals and
  README's placeholder `Bearer <token>` wording were observed.
- No `Cargo.toml`, `Cargo.lock`, `.github/**`, `scripts/**`, deployment, or
  packaging diff.

## Harness Coverage

### Duplicate x-msg-id

- Two accepted pushes with the same supplied `x-msg-id` both return that ID.
- A pull with `max=2` returns two items in FIFO order.
- Both returned items carry the duplicate ID.
- Payloads remain bound to the correct queue entries.
- A repeated pull after delivery returns `204 No Content`.

### Mixed Supplied / Auto IDs

- A supplied ID, an auto-generated ID, and a duplicate supplied ID coexist in
  the same queue.
- The server-generated ID is non-empty and distinct from the supplied sentinel.
- Pull returns the three items in send order.
- Payloads remain uncorrupted and associated with the correct IDs.

### Auth / Oversize Reject No-Mutation

- A duplicate-ID push rejected for wrong bearer auth returns
  `401 ERR_UNAUTHORIZED` and does not enqueue.
- A duplicate-ID push rejected for oversize returns `413 ERR_TOO_LARGE` and
  does not enqueue.
- A duplicate-ID push rejected by queue cap returns `429 ERR_OVERLOADED` and
  does not enqueue.
- In each reject case, the already accepted item remains the only retrievable
  queue item.

### Pull / Delete Semantics

- Duplicate-ID queue entries are delivered in FIFO order.
- Pull deletes delivered entries.
- Repeated pull after delivery is deterministic and returns empty.
- Bad or rejected push attempts do not consume existing queue entries.

### Logging / No-Secret

- The qsl-server docs now state the current boundary: accepted message IDs may
  appear in service logs as non-secret operational metadata.
- The logging harness captures an accepted push/pull path and proves the
  message ID sentinel can appear as metadata.
- Captured logs exclude raw route-token sentinels.
- Captured logs exclude auth-token sentinels.
- Captured logs exclude `Authorization` / `Bearer` strings.
- Captured logs exclude payload sentinels.

### No-Panic

- Blank `x-msg-id` is treated as absent and receives a server-generated ID.
- Duplicate and blank-ID focused tests completed without panic.
- Full qsl-server tests completed without panic after the bounded retry noted
  in Results.

## Results

Passed:

- qsl-server preflight `cargo audit --deny warnings` passed on starting main.
- qsl-server preflight `cargo test --locked` passed on starting main.
- qsl-server PR #50 merged after required `rust` CI success.
- qsl-server harness tests and full locked test suite passed locally.
- qsl-server audit passed locally.

Recovered local validation failures:

- `cargo audit --deny warnings` initially failed in qsl-protocol because the
  clean local qsl-protocol worktree was still on stale `mirror/main` rather
  than directive-required `origin/main`. Classification: recoverable local
  checkout state. Corrective action: switched the clean qsl-protocol worktree
  to fetched `origin/main`. Final result: advisories self-test, audit, and
  `rustls-webpki v0.103.13` proof passed.
- qsl-server `cargo fmt --check` first reported formatting diffs in the new
  harness. Classification: recoverable local formatting issue. Corrective
  action: ran `cargo fmt`. Final result: `cargo fmt --check` passed.
- An early combined qsl-server `cargo test --locked` run failed because the new
  logging assertion lived in the same default-parallel test binary as other
  harness tests. Classification: recoverable test-shape issue. Corrective
  action: split the logging proof into `tests/idempotency_logging.rs`. Final
  result: focused and full tests passed.
- One later qsl-server `cargo test --locked` run failed in pre-existing
  `src/lib.rs::logs_do_not_contain_raw_channel` due default-parallel log
  capture timing. Classification: recoverable flaky existing test observation;
  no service or source change was made. Corrective action: one bounded retry.
  Final result: full `cargo test --locked` passed.

Remaining semantic decisions:

- Whether qsl-server should ever implement idempotent duplicate suppression for
  `x-msg-id`.
- Whether idempotency, if later required, should reject, ignore, or return a
  prior result for duplicate IDs.
- Whether accepted message IDs should continue to be logged raw as non-secret
  metadata or be hashed/redacted in a future metadata-minimization lane.
- Whether invalid `MAX_BODY_BYTES` and `MAX_QUEUE_DEPTH` should fail startup
  instead of falling back to defaults.
- Whether qsl-server needs TTL, persistence, global route caps, rate limits,
  and broader abuse hardening.

## No-Production Boundary

NA-0275 provides local executable qsl-server semantics evidence for current
transport-only queue behavior. It does not deploy qsl-server, expose a public
relay, approve public internet operation, certify production service
operation, or change qsl-protocol runtime behavior. qsl-server and
qsl-attachments production gates remain future work.

## Next Recommended Harness

The next recommended executable qsl-server harness is NA-0276: invalid
`MAX_BODY_BYTES` / `MAX_QUEUE_DEPTH` config fail-closed semantics. Current
qsl-server docs explicitly record fallback/capping as current behavior, and a
future lane should decide whether invalid size/depth startup must fail closed
with executable proof.
