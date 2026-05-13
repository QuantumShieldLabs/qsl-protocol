Goals: G1, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-13
Replaces: n/a
Superseded-By: n/a

# NA-0276 qsl-server Invalid Config Semantics Harness

Directive: QSL-DIR-2026-05-13-079 / NA-0276

## Executive Summary

NA-0276 records and test-backs qsl-server startup/configuration semantics for
`MAX_BODY_BYTES`, `MAX_QUEUE_DEPTH`, `PORT`, `BIND_ADDR`, and `RELAY_TOKEN`.

qsl-server PR #51 merged an executable local/loopback harness and a minimal
test-backed implementation repair. The selected semantics are:

- Missing `MAX_BODY_BYTES` and `MAX_QUEUE_DEPTH` use documented defaults.
- Non-numeric or zero `MAX_BODY_BYTES` fails startup with
  `ERR_INVALID_CONFIG_MAX_BODY_BYTES`.
- Non-numeric or zero `MAX_QUEUE_DEPTH` fails startup with
  `ERR_INVALID_CONFIG_MAX_QUEUE_DEPTH`.
- Values above built-in ceilings are capped.
- Invalid `PORT` remains fail-closed.
- Invalid `BIND_ADDR` fails closed during bind-address parsing.
- Missing or empty `RELAY_TOKEN` disables bearer auth only; route-token header
  validation still applies. A present relay token requires bearer auth.

No qsl-protocol runtime, protocol, crypto, state-machine, qsl-attachments,
qsc-desktop, website, workflow, script, Cargo, dependency, branch-protection,
or public-safety configuration path changed in this evidence lane. This
evidence does not claim production readiness.

## qsl-server PR Evidence

- Repository: `QuantumShieldLabs/qsl-server`
- PR: #51, `NA-0276: add qsl-server invalid config semantics harness`
- PR URL: https://github.com/QuantumShieldLabs/qsl-server/pull/51
- Head SHA: `89a6b025bad7`
- Merge SHA: `6fa59d2f9a69`
- Merged at: 2026-05-13T15:11:40Z
- Required check: `rust` success before merge.
- Required check URL:
  https://github.com/QuantumShieldLabs/qsl-server/actions/runs/25807956811/job/75815973057
- Changed paths:
  - `README.md`
  - `docs/server/DOC-SRV-001_Deployment_Hardening_Contract_v1.0.0_DRAFT.md`
  - `docs/server/DOC-SRV-003_Relay_Inbox_Contract_v1.0.0_DRAFT.md`
  - `src/lib.rs`
  - `src/main.rs`
  - `tests/config_semantics.rs`
  - `tests/hardening_auth_reject_logging.rs`
  - `tests/smoke.rs`
- Implementation changed: yes, limited to qsl-server limit validation and
  startup config parsing for `MAX_BODY_BYTES` / `MAX_QUEUE_DEPTH`.
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
- `cargo test --locked`: pass, including the new config harness.
- `cargo clippy --locked --all-targets -- -D warnings`: pass.
- `cargo audit --deny warnings`: pass.
- `git diff --check`: pass.
- Changed-file overclaim scan: pass.
- Changed-file leak/secret shape scan: pass; the only flagged string was a
  test sentinel that the harness asserts is absent from invalid-config output.
- No `Cargo.toml`, `Cargo.lock`, `.github/**`, `scripts/**`, deployment, or
  packaging diff.

Post-merge qsl-server main validation:

- `cargo audit --deny warnings`: pass on `6fa59d2f9a69`.
- `cargo test --locked`: pass on `6fa59d2f9a69`.

## Harness Coverage

### MAX_BODY_BYTES

- Missing/default behavior: startup stays running with defaults.
- Invalid non-numeric behavior: startup fails with
  `ERR_INVALID_CONFIG_MAX_BODY_BYTES`.
- Zero behavior: startup fails with `ERR_INVALID_CONFIG_MAX_BODY_BYTES`.
- Above-ceiling behavior: startup stays running and validation caps the value
  to the built-in ceiling.
- Effective body-limit behavior: a 4-byte selected limit accepts a 4-byte body,
  rejects a 5-byte body with `413 ERR_TOO_LARGE`, and does not enqueue the
  rejected body.

### MAX_QUEUE_DEPTH

- Missing/default behavior: startup stays running with defaults.
- Invalid non-numeric behavior: startup fails with
  `ERR_INVALID_CONFIG_MAX_QUEUE_DEPTH`.
- Zero behavior: startup fails with `ERR_INVALID_CONFIG_MAX_QUEUE_DEPTH`.
- Above-ceiling behavior: startup stays running and validation caps the value
  to the built-in ceiling.
- Effective queue-cap behavior: a selected depth of 1 accepts the first push,
  rejects the second push with `429 ERR_OVERLOADED`, and preserves only the
  accepted item for pull.

### PORT

- Invalid `PORT` fails startup with `ERR_INVALID_ENV_PORT`.
- Existing invalid-port behavior is preserved.

### BIND_ADDR

- Invalid bind address fails closed with `ERR_BIND_PARSE`.
- The test uses local subprocess startup only and does not expose a public
  listener.

### RELAY_TOKEN

- Missing relay token allows canonical push without bearer auth while still
  requiring route-token header selection.
- Present relay token rejects missing bearer auth with
  `401 ERR_UNAUTHORIZED`.
- Present relay token accepts the correct bearer token.

### Regression Harnesses

- Existing qsl-server auth/reject/logging harness remains green.
- Existing x-msg-id semantics harness remains green.
- Existing x-msg-id logging harness remains green.
- Full qsl-server locked test suite remains green.

### Logging / No-Secret

- Invalid-config subprocess tests set a relay-token sentinel and assert it is
  absent from failure output.
- Invalid-config output is checked for deterministic error codes only.
- Captured failure output is checked for absence of panic/backtrace text.

### No-Panic

- Non-numeric/zero size/depth failures exit deterministically without panic.
- Invalid port and invalid bind address failures exit deterministically without
  panic.
- Full qsl-server tests complete without panic.

## Results

Passed:

- qsl-server PR #51 merged after required `rust` CI success.
- qsl-server focused harness tests passed.
- qsl-server full locked test suite passed before and after merge.
- qsl-server audit passed before and after merge.

Recovered local validation failure:

- Initial qsl-server `cargo test --locked` on starting main failed once in the
  pre-existing `hardening_auth_reject_logging` log-capture test because the
  captured logs did not include the expected push marker. Classification:
  recoverable local default-parallel logging-capture flake; the focused test
  passed immediately and the full suite passed serially. Corrective action:
  one bounded rerun of the exact full command after classification. Final
  result: `cargo test --locked` passed before edits, and full validation
  passed after the NA-0276 patch.

Remaining semantic decisions:

- Whether future qsl-server config should reject above-ceiling values instead
  of capping them. Current NA-0276 semantics retain explicit capping.
- Whether qsl-server should add global route/account caps, TTL, persistence,
  rate limits, or abuse controls.
- Whether accepted message IDs should later be hashed/redacted in logs.

## No-Production Boundary

NA-0276 provides local executable qsl-server startup/configuration semantics
evidence. It does not deploy qsl-server, expose a public relay, approve public
internet operation, certify production service operation, or change
qsl-protocol runtime behavior. qsl-server and qsl-attachments production gates
remain future work.

## Next Recommended Harness

The next recommended executable qsl-server harness is NA-0277:
abuse/rate-limit/queue-cap behavior under bounded overload pressure, including
deterministic rejects, queue/resource cap proof, and no secret logging under
pressure.
