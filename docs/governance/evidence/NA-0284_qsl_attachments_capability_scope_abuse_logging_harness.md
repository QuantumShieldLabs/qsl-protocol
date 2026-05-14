Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-14

# NA-0284 qsl-attachments Capability Scope / Abuse / Logging Harness

Goals: G1, G3, G4, G5

## Executive Summary

NA-0284 adds executable qsl-attachments capability scope / abuse / logging
proof in the sibling `QuantumShieldLabs/qsl-attachments` repository.

The qsl-attachments PR is harness/docs-only. It does not change service source,
dependencies, workflows, deployment posture, protocol behavior, qsl-server
behavior, qsl-protocol runtime behavior, qsc-desktop, or website surfaces.

The harness proves resource-scoped resume tokens and fetch capabilities,
wrong-resource and malformed capability fail-closed behavior, deterministic
duplicate/replay behavior under the current reusable-within-scope contract,
delete/abort/expiry invalidation behavior, bounded abuse loops, no
unauthorized mutation or exposure, opaque-ciphertext preservation, and
capability/descriptor/ciphertext/plaintext redaction in logs and error bodies.
It keeps the existing malformed JSON / reject-taxonomy, retention / cleanup /
recovery, and disk pressure / quota / abuse harnesses green.

## qsl-attachments PR Evidence

- Repository: `QuantumShieldLabs/qsl-attachments`
- PR: #35, <https://github.com/QuantumShieldLabs/qsl-attachments/pull/35>
- Head SHA: `d95e2ad6aef6`
- Merge SHA: `0b7b3fcf9afc`
- Changed paths:
  - `README.md`
  - `tests/capability_scope_abuse.rs`
  - `tests/capability_scope_logging.rs`
  - `tests/support/mod.rs`
- Implementation changed: no service source changed.
- Dependency changed: no.
- Workflow changed: no.
- Production readiness claim: no.
- Test posture: loopback/local/tempdir executable harness only.

## Chosen Capability Scope / Abuse / Logging Semantics

qsl-attachments keeps the current operator-scoped deployment policy plus
resource capability model:

- `resume_token` authorizes exactly one open session.
- `fetch_capability` authorizes exactly one committed object.
- Capabilities are reusable only inside that resource scope while the resource
  remains valid.
- A valid resume token can status/upload/commit/abort its one open session.
- Commit removes the open session and invalidates the resume token.
- Abort marks the session aborted, clears staged parts, and invalidates the
  resume token.
- Session expiry marks the session expired, clears staged parts, and
  invalidates the resume token.
- A valid fetch capability can repeatedly fetch its one committed object until
  object expiry.
- Object expiry marks the object expired, removes `ciphertext.bin`, and
  invalidates the fetch capability.
- Wrong-resource resume tokens fail with `REJECT_QATTSVC_RESUME_TOKEN` until
  the per-resource abuse threshold is exceeded, then fail with
  `REJECT_QATTSVC_ABUSE`.
- Wrong-resource fetch capabilities fail with
  `REJECT_QATTSVC_FETCH_CAPABILITY` until the per-resource abuse threshold is
  exceeded, then fail with `REJECT_QATTSVC_ABUSE`.
- Missing or malformed capability headers fail closed with canonical reason
  codes and do not create, mutate, or expose resource state.
- Logs and audit snapshots contain redacted handles and reason codes only, not
  raw capabilities, descriptors, ciphertext, plaintext, or wrong-capability
  sentinels.
- The service stores and returns opaque ciphertext bytes only and never
  decrypts client plaintext.

## Tests Run

qsl-attachments local validation completed green before PR creation:

- `cargo fmt --check`
- `cargo test --locked --test service_contract -- --test-threads=1`
- `cargo test --locked --test reject_taxonomy_harness -- --test-threads=1`
- `cargo test --locked --test retention_cleanup_recovery -- --test-threads=1`
- `cargo test --locked --test retention_cleanup_logging -- --test-threads=1`
- `cargo test --locked --test disk_pressure_quota_abuse -- --test-threads=1`
- `cargo test --locked --test disk_pressure_quota_logging -- --test-threads=1`
- `cargo test --locked --test capability_scope_abuse -- --test-threads=1`
- `cargo test --locked --test capability_scope_logging -- --test-threads=1`
- `cargo test --locked`
- `cargo clippy --locked --all-targets -- -D warnings`
- `cargo audit --deny warnings`
- `git diff --check`
- direct overclaim scan over changed files
- direct changed-file sentinel scan for the logging harness

The qsl-attachments PR required `rust` check completed success before merge.
After merge, qsl-attachments `origin/main` at `0b7b3fcf9afc` passed:

- `cargo audit --deny warnings`
- `cargo test --locked`
- `cargo clippy --locked --all-targets -- -D warnings`

## Harness Coverage

### Resume Capability Scope

`wrong_resume_capability_cannot_mutate_other_session` proves a resume token
from one session cannot upload to, abort, or otherwise mutate another session.
The target session remains resumable with the correct token, and the unrelated
session remains independently usable.

### Fetch Capability Scope

`wrong_fetch_capability_cannot_fetch_other_object` proves a fetch capability
for one object cannot fetch another object, does not expose either object's
ciphertext in the error body, and does not mutate either committed object.
Both objects remain fetchable with their correct capabilities.

### Missing / Malformed Capabilities

`missing_malformed_capabilities_fail_closed_with_reason_code` proves missing
and malformed resume/fetch capability headers fail with canonical reason codes,
do not stage parts, do not leak returned capabilities, and preserve later valid
upload/commit/fetch behavior.

### Delete / Abort / Expiry Interactions

`deleted_or_aborted_resource_capability_behavior_is_deterministic` proves abort
clears staged parts and invalidates the resume token. Repeated abort/upload
with the old token stays fail-closed. It also proves repeated fetch after object
expiry returns deterministic `REJECT_QATTSVC_EXPIRED`.

### Duplicate / Replay Capability Behavior

`duplicate_capability_use_matches_documented_semantics` proves current
capabilities are reusable within scope while valid:

- duplicate upload of identical staged bytes succeeds idempotently without
  duplicating staged parts;
- status with the same resume token remains valid before commit;
- duplicate commit after session removal fails with
  `REJECT_QATTSVC_SESSION_STATE`;
- repeated fetch with the same valid fetch capability returns the same opaque
  ciphertext bytes.

### Abuse Loops

`bounded_capability_abuse_has_no_panic_and_no_unbounded_growth` proves repeated
wrong resume/fetch capability attempts stay bounded, cross from resource-token
rejects to `REJECT_QATTSVC_ABUSE` after the configured limit, do not grow the
test storage root, and do not block later valid access with the correct
capability.

### Unauthorized No-Mutation / No-Exposure

The wrong-resource, missing/malformed, abort/expiry, and abuse tests assert
that rejected operations do not stage parts, abort another session, alter
committed object bytes, expose ciphertext/plaintext in error bodies, or make
valid resources unavailable to correct capabilities.

### Opaque Ciphertext

The harness stores and fetches sentinel ciphertext as opaque bytes only. The
service never decrypts client plaintext, and the logging harness asserts both
ciphertext and plaintext sentinels are absent from logs, audit snapshots, and
error bodies.

### Logging / No-Secret

`capability_abuse_logs_redact_capabilities_descriptor_ciphertext_plaintext`
captures malformed JSON, wrong resume capability, wrong fetch capability, valid
commit, and valid fetch paths. It asserts logs, audit snapshots, and error
bodies omit:

- resume tokens;
- fetch capabilities;
- wrong-capability sentinels;
- descriptor sentinels;
- ciphertext sentinels;
- plaintext sentinels.

### No-Panic

Full `cargo test --locked`, focused serial harness tests, and the bounded abuse
loops completed green. No panic path was observed.

## Results

Passed:

- qsl-attachments local validation bundle listed above.
- qsl-attachments PR #35 `rust` required check.
- qsl-attachments PR #35 merge to `main`.
- qsl-attachments merged-main audit, test, and clippy checks.

Recovered local warning issue:

- Initial focused test runs showed dead-code warnings from shared integration
  helpers when each test target compiled only the helpers it used.
- Classification: recoverable test-harness hygiene issue that would fail the
  required clippy gate.
- Corrective action: added a test-helper-local dead-code allowance in
  `tests/support/mod.rs`.
- Final result: targeted tests, full tests, and clippy passed.

Remaining semantic decisions:

- Capabilities remain reusable within resource scope while valid; one-time
  capability use is not introduced by this lane.
- Abuse tracking remains in-memory and per target resource.
- Broader authorization-service design, deployment policy expansion, cross-node
  authorization state, hot/live backup, partial restore, and transactional
  recovery remain future work.

## No-Production Boundary

This evidence does not claim production readiness, deployment readiness, public
internet readiness, external review completion, metadata elimination, strong
identity-hiding guarantees, or a completed release-security proof.

The harness is local/loopback executable evidence for the current single-node
local-disk attachment service boundary only.

## Next Recommended Harness

Promote the next qsl-attachments hardening lane as:

`NA-0285 — qsl-attachments Backup / Partial Restore / Transactional Recovery Boundary Plan`

Recommended scope:

- backup / partial restore / transactional recovery boundary design;
- executable harness plan;
- explicit unsupported hot/live backup and partial-restore boundaries;
- no qsl-attachments implementation changes in the planning lane.
