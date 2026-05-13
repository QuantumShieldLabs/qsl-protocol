Goals: G1, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-12
Replaces: n/a
Superseded-By: n/a

# NA-0274 qsl-attachments Malformed JSON / Reject-Taxonomy Harness

Directive: QSL-DIR-2026-05-12-077 / NA-0274

## Executive Summary

NA-0274 merged the first executable qsl-attachments reject-taxonomy harness.
The harness covers malformed JSON / Axum extractor rejects, canonical
`reason_code` behavior, capability rejects, no persistence after rejected
requests, opaque ciphertext boundaries, log/audit no-secret behavior, expiry
cleanup, and no-panic reject paths.

The qsl-attachments dependency preflight passed, so no dependency remediation
PR was needed. The harness PR includes a minimal qsl-attachments implementation
fix: Axum JSON extractor rejections now map into a sanitized
`REJECT_QATTSVC_MALFORMED_JSON` service error body while preserving the Axum
extractor status. No qsl-attachments dependency files or workflows changed. No
qsl-protocol runtime, protocol, crypto, state-machine, qsl-server,
qsc-desktop, website, workflow, script, Cargo, dependency, branch-protection,
or public-safety configuration path changed in this evidence lane. This
evidence does not claim production readiness.

## qsl-attachments Dependency Remediation PR Evidence

Dependency remediation was not used.

Pre-harness qsl-attachments validation:

- Repository: `QuantumShieldLabs/qsl-attachments`
- Starting SHA: `1e1ae272a4cb`
- `cargo audit --deny warnings`: pass.
- `cargo test --locked`: pass, 29 tests total before the new harness.
- `Cargo.toml` changed: no.
- `Cargo.lock` changed: no.
- Dependency remediation PR: n/a.

## qsl-attachments Harness PR Evidence

- Repository: `QuantumShieldLabs/qsl-attachments`
- PR: #32, `NA-0274: add qsl-attachments reject taxonomy harness`
- PR URL: https://github.com/QuantumShieldLabs/qsl-attachments/pull/32
- Head SHA: `6e621a30a16b`
- Merge SHA: `99eae6facf11`
- Merged at: 2026-05-13T02:09:39Z
- Required check: `rust` success before merge.
- Required check URL:
  https://github.com/QuantumShieldLabs/qsl-attachments/actions/runs/25773843334/job/75702325655
- Changed paths:
  - `src/lib.rs`
  - `tests/reject_taxonomy_harness.rs`
- Implementation changed: yes.
- Implementation change scope: minimal Axum `Json<T>` rejection mapping for
  create-session and commit handlers.
- Dependency files changed in harness PR: no.
- Workflow changed: no.
- Production-readiness claim: no.
- Harness is loopback/local only.

Validation before merge:

- `cargo fmt --check`: pass after formatting recovery.
- `cargo test --locked --test reject_taxonomy_harness -- --test-threads=1`:
  pass, 6 tests.
- `cargo test --locked`: pass, 35 tests total after the new harness.
- `cargo clippy --locked --all-targets -- -D warnings`: pass after a test
  helper cleanup.
- `cargo audit --deny warnings`: pass.
- `git diff --check`: pass.
- Changed-file overclaim scan: pass.
- Changed-file leak-safe pattern scan: pass.
- No `Cargo.toml`, `Cargo.lock`, or `.github/**` diff.

Post-merge qsl-attachments main validation:

- qsl-attachments final SHA: `99eae6facf11`
- `cargo audit --deny warnings`: pass.
- `cargo test --locked`: pass, 35 tests total.

## Harness Coverage

### Malformed JSON / Extractor Reject

- Malformed create-session JSON returns `400` with
  `REJECT_QATTSVC_MALFORMED_JSON`.
- Malformed commit JSON returns `400` with
  `REJECT_QATTSVC_MALFORMED_JSON`.
- Extractor reject bodies are sanitized as `malformed JSON request body`.
- Malformed create rejects do not create sessions or objects.
- Malformed commit rejects do not promote a staged session into an object and
  leave the existing session state intact.

### Canonical reason_code

- JSON extractor rejects now return service-shaped JSON with canonical
  `reason_code`.
- Existing service rejects remain service-shaped JSON with canonical
  `reason_code`, including resume-token, fetch-capability, expired, and
  locator/session-state rejects.
- Unknown, malformed, or invalid requests do not silently succeed.

### Capability Rejects

- Missing resume-token header rejects fail closed.
- Wrong resume-token header rejects fail closed.
- Missing fetch-capability header rejects fail closed.
- Wrong resource / capability mismatch rejects fail closed with
  `REJECT_QATTSVC_FETCH_CAPABILITY`.
- Correct capabilities remain usable after rejected probes.

### No Persistence on Reject

- Malformed create JSON leaves `sessions/` and `objects/` empty.
- Malformed commit JSON leaves `objects/` empty and the session
  `committable`.
- Missing/wrong resume-token upload rejects do not create a staged part.
- Fetch-capability rejects do not create, expose, or delete unrelated objects.

### Opaque Ciphertext Boundary

- The harness uploads and fetches an opaque ciphertext sentinel byte-for-byte.
- The service stores/fetches opaque bytes only and does not decrypt or inspect
  client plaintext.
- Plaintext sentinels are not introduced into logs or audit snapshots.

### Logging / No-Secret

- Captured tracing logs and audit snapshots exclude raw resume tokens.
- Captured tracing logs and audit snapshots exclude raw fetch capabilities.
- Captured tracing logs and audit snapshots exclude descriptor sentinels.
- Captured tracing logs and audit snapshots exclude ciphertext sentinels.
- Captured tracing logs and audit snapshots exclude plaintext sentinels.
- Error output is sanitized and does not echo malformed JSON body contents.

### Cleanup / Retention Baseline

- Existing expiry helpers are exercised by the harness.
- Expired sessions reject with `REJECT_QATTSVC_EXPIRED` and remove staged part
  bytes.
- Expired objects reject with `REJECT_QATTSVC_EXPIRED` and remove
  `ciphertext.bin`.

### No-Panic

- Focused and full test runs completed without panics outside intentional test
  assertions.

## Results

Passed:

- qsl-attachments dependency preflight passed without remediation.
- qsl-attachments harness PR #32 merged after required `rust` CI success.
- qsl-attachments merged main passed `cargo audit --deny warnings`.
- qsl-attachments merged main passed `cargo test --locked`.

Recovered local validation failures:

- `cargo fmt --check` first reported formatting diffs in the new harness.
  Corrective action: ran `cargo fmt`; final result: `cargo fmt --check`
  passed.
- A changed-file leak-safe scan first reported a false positive from a
  deliberately invalid token sentinel literal in the test. Corrective action:
  generated the invalid test token at runtime; final result: leak-safe pattern
  scan passed.
- `cargo clippy --locked --all-targets -- -D warnings` first reported a
  test-helper `&PathBuf` argument. Corrective action: changed the helper to
  take `&Path`; final result: clippy passed.

Remaining semantic decisions:

- Whether future qsl-attachments API shape errors should use a more granular
  reason-code taxonomy than the current extractor-level malformed JSON bucket.
- Whether additional range/header malformed cases need a dedicated
  reject-taxonomy harness.
- Whether qsl-attachments needs broader operational log capture beyond current
  tracing/audit snapshots.

## No-Production Boundary

NA-0274 provides local executable reject-taxonomy evidence for the current
single-node qsl-attachments runtime. It does not deploy qsl-attachments, expose
the service publicly, approve public internet operation, certify production
service operation, or change qsl-protocol runtime behavior. qsl-server and
qsl-attachments production gates remain future work.

## Next Recommended Harness

The next recommended executable service harness is NA-0275:
qsl-server `x-msg-id` / idempotency semantics. It should resolve whether
duplicate message IDs remain independent queue entries or become idempotent,
prove deterministic duplicate behavior, and preserve route-token/auth/payload
no-leak invariants.
