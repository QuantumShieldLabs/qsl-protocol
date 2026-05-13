Goals: G1, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-12
Replaces: n/a
Superseded-By: n/a

# NA-0273 qsl-server Auth/Reject/Logging Harness

Directive: QSL-DIR-2026-05-12-076 / NA-0273

## Executive Summary

NA-0273 first remediated qsl-server dependency advisories that blocked the
required validation gate, then merged an executable qsl-server hardening
harness for auth rejects, no-mutation on rejects, queue overload, pull response
shape, retired legacy routes, logging/no-secret behavior, and bounded startup
configuration evidence.

The qsl-server dependency remediation was lockfile-only. The qsl-server harness
PR was tests-only. No qsl-server service implementation code changed, no
qsl-server workflow changed, and no qsl-protocol runtime, protocol, crypto,
state-machine, qsl-attachments, website, workflow, script, Cargo, dependency,
branch-protection, or public-safety configuration path changed in this evidence
lane. This evidence does not claim production readiness.

## qsl-server Dependency Remediation PR Evidence

- Repository: `QuantumShieldLabs/qsl-server`
- PR: #48, `NA-0273: remediate qsl-server dependency advisories`
- PR URL: https://github.com/QuantumShieldLabs/qsl-server/pull/48
- Head SHA: `5994d57dc0d9`
- Merge SHA: `f8d223523628`
- Merged at: 2026-05-13T00:26:16Z
- Required check: `rust` success before merge.
- Required check URL:
  https://github.com/QuantumShieldLabs/qsl-server/actions/runs/25770296908/job/75691753499
- Changed paths:
  - `Cargo.lock`
- `Cargo.toml` changed: no. Existing constraints permitted patched versions.
- qsl-server source implementation changed: no.
- qsl-server workflow changed: no.
- Production-readiness claim: no.

Initial `cargo audit --deny warnings` findings before remediation:

- `RUSTSEC-2026-0007` / `bytes 1.11.0`: integer overflow in
  `BytesMut::reserve`; patched by `bytes 1.11.1`.
- `RUSTSEC-2026-0037` / `quinn-proto 0.11.13`: Quinn endpoint denial of
  service; patched by `quinn-proto 0.11.14`.
- `RUSTSEC-2026-0099`, `RUSTSEC-2026-0104`, `RUSTSEC-2026-0049`, and
  `RUSTSEC-2026-0098` / `rustls-webpki 0.103.9`: patched by
  `rustls-webpki 0.103.13`.
- `RUSTSEC-2026-0097` / `rand 0.9.2`: denied warning; patched by
  `rand 0.9.3`.

Dependency path classification:

- `bytes`: normal dependency path through `axum`, `tokio`, `tower-http`, and
  dev/test `reqwest` paths.
- `rustls-webpki`: dev/test TLS path through `reqwest` and `rustls`.
- `quinn-proto` and `rand`: lockfile-resolved transitive path from `reqwest`'s
  transport dependency set; `cargo tree` did not print a selected default
  target path after remediation, but the lockfile entries were updated and
  `cargo audit --deny warnings` passed.

Remediation commands:

- `cargo update -p bytes --precise 1.11.1`
- `cargo update -p quinn-proto --precise 0.11.14`
- `cargo update -p rustls-webpki --precise 0.103.13`
- `cargo update -p rand --precise 0.9.3`

Post-remediation validation:

- `cargo audit --deny warnings`: pass.
- `cargo test --locked`: pass.
- `git diff --check`: pass.
- Changed files: `Cargo.lock` only.

## qsl-server Harness PR Evidence

- Repository: `QuantumShieldLabs/qsl-server`
- PR: #49, `NA-0273: add qsl-server auth reject logging harness`
- PR URL: https://github.com/QuantumShieldLabs/qsl-server/pull/49
- Head SHA: `0b4c335b9ef0`
- Merge SHA: `ab643f22bd42`
- Merged at: 2026-05-13T00:31:29Z
- Required check: `rust` success before merge on the corrected head.
- Required check URL:
  https://github.com/QuantumShieldLabs/qsl-server/actions/runs/25770470407/job/75692283690
- Changed paths:
  - `tests/hardening_auth_reject_logging.rs`
- Implementation changed: no.
- Dependency files changed in harness PR: no.
- Workflow changed: no.
- Production-readiness claim: no.
- Harness is loopback/local only.

The D075 local harness was preserved before dependency remediation at:

- `/srv/qbuild/tmp/NA-0273_d075_harness_preserve_20260512T192244-0500`
- SHA-256: `57fb09bf003dfe32ad586f09c5039d1893064cd7930cfe68e17c70d22da80be4`
- Line count before restore: 516.

The first PR #49 CI attempt failed because the log-capture test did not set an
explicit info-level subscriber. That was classified as a recoverable test-shape
issue, fixed by making the harness log subscriber capture info-level events
deterministically, and the corrected PR head passed CI.

Validation for the corrected harness:

- `cargo fmt --check`: pass.
- `cargo test --locked --test hardening_auth_reject_logging -- --test-threads=1`:
  pass, 7 tests.
- `cargo test --locked`: pass.
- `cargo audit --deny warnings`: pass.
- `git diff --check`: pass.

## Harness Coverage

### Auth / Reject

- Missing bearer auth rejects with `401 ERR_UNAUTHORIZED`.
- Wrong bearer auth rejects with `401 ERR_UNAUTHORIZED`.
- Correct bearer auth can still push and pull after rejects.
- Missing or empty route-token header rejects with deterministic
  route-token errors.

### No-Mutation

- Missing/wrong bearer rejects do not enqueue data.
- Oversize push rejects do not enqueue data.
- Full-queue rejects do not append a second item.
- Pulling from a different route token does not consume the original route's
  queued item.
- Retired legacy route requests do not mutate or consume canonical queues.

### Queue / Overload

- Queue full returns `429 ERR_OVERLOADED`.
- The accepted item remains retrievable after a rejected overload push.
- Overload logging is captured without route token, bearer token, or payload
  sentinel leakage.

### Pull Response

- Empty pull returns 204.
- Non-empty pull returns JSON with an `items` array.
- Pull respects bounded `max` values.
- Successful pull deletes delivered messages.
- Bad `max` rejects deterministically without consuming queue items.

### Legacy Route

- Retired legacy path-token push returns 404.
- Retired legacy path-token pull returns 404.
- Canonical queue state survives both retired-route probes.

### Logging / No-Secret

- Captured logs prove service events are emitted with redacted route metadata.
- Raw route-token sentinels are absent from captured logs.
- Correct and wrong bearer-token sentinels are absent from captured logs.
- Authorization/Bearer strings are absent from captured logs.
- Payload sentinels are absent from captured logs.

### Config / Startup

- Invalid `PORT` fails process startup.
- Invalid `MAX_BODY_BYTES` and `MAX_QUEUE_DEPTH` are captured as current
  fallback/capping behavior, not overstated as fail-closed behavior.
- The harness keeps fail-closed startup for invalid size/depth as future work.

### No-Panic

- Focused and full test runs completed without panics outside intentional test
  assertions.

## Results

Passed:

- Dependency advisory remediation passed audit and tests.
- Harness passed focused and full qsl-server test suites.
- qsl-server CI `rust` passed on both PR #48 and corrected PR #49.
- qsl-server main after PR #49 passed `cargo audit --deny warnings` and
  `cargo test --locked`.

Recovered failure:

- PR #49 first CI run failed at
  `logs_do_not_leak_route_auth_or_payload_on_success_or_rejects` because the
  harness did not explicitly capture info-level events under CI's default full
  test command.
- Corrective action: make the test subscriber use an explicit info max level.
- Final result: corrected PR head `0b4c335b9ef0` passed required CI.

Remaining semantic decisions:

- Whether duplicate `x-msg-id` pushes should remain independent queue entries
  or become idempotent.
- Whether invalid `MAX_BODY_BYTES` and `MAX_QUEUE_DEPTH` should fail startup
  instead of falling back to defaults.
- Whether qsl-server needs additional rate limits, route-count caps, TTLs,
  persistence, cleanup, global queue limits, and operational endpoints.
- Whether non-UTF-8 route-token headers need a more explicit canonical reject
  taxonomy.

## No-Production Boundary

NA-0273 provides dependency-health remediation and executable local qsl-server
hardening evidence. It does not deploy qsl-server, expose a public relay,
approve public internet operation, certify production service operation, or
change qsl-protocol runtime behavior. Known qsl-server and qsl-attachments
production gates remain future work.

## Next Recommended Harness

The next recommended executable service harness is NA-0274:
qsl-attachments malformed JSON / reject-taxonomy coverage. It should focus on
Axum extractor rejects, canonical `reason_code` behavior, capability rejects,
no persistence on rejected requests, and no capability/descriptor/ciphertext or
plaintext leakage in service logs.
