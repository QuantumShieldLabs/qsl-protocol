Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-14

# NA-0283 qsl-attachments Disk Pressure / Quota / Abuse Harness

Goals: G1, G3, G4, G5

## Executive Summary

NA-0283 adds executable qsl-attachments disk pressure / quota / abuse proof in
the sibling `QuantumShieldLabs/qsl-attachments` repository.

The qsl-attachments PR is tests-only. It does not change service source,
dependencies, workflows, deployment posture, protocol behavior, qsl-server
behavior, qsl-protocol runtime behavior, qsc-desktop, or website surfaces.

The harness proves deterministic quota and disk-headroom rejects, open-session
quota release, no unexpected persistence on rejected writes, cleanup behavior
under simulated pressure, same-root restart / recovery boundaries after
quota/disk rejects, partial artifact recovery boundaries, capability isolation
under pressure and abuse, opaque-ciphertext preservation, and secret-safe
quota/disk/abuse logging. It keeps the existing malformed JSON /
reject-taxonomy and retention / cleanup / recovery harnesses green.

## qsl-attachments PR Evidence

- Repository: `QuantumShieldLabs/qsl-attachments`
- PR: #34, <https://github.com/QuantumShieldLabs/qsl-attachments/pull/34>
- Head SHA: `baf5a9c9d3b7`
- Merge SHA: `4ae5ceab6f1a`
- Changed paths:
  - `tests/disk_pressure_quota_abuse.rs`
  - `tests/disk_pressure_quota_logging.rs`
- Implementation changed: no.
- Dependency changed: no.
- Workflow changed: no.
- Production readiness claim: no.
- Test posture: loopback/local/tempdir executable harness only.

## Chosen Disk Pressure / Quota / Abuse Semantics

qsl-attachments keeps the existing bounded single-node local-disk semantics:

- `max_ciphertext_bytes` rejects oversize creates with
  `REJECT_QATTSVC_QUOTA`.
- `max_open_sessions` is deployment-global and rejects excess open sessions
  with `REJECT_QATTSVC_QUOTA`.
- Aborting or completing an open session releases open-session quota under the
  current contract.
- Disk pressure is simulated through the existing `TestDiskSpace` provider,
  never by filling the host disk.
- Create checks require headroom for staged plus committed bytes and reserve.
- Upload checks require part-size plus reserve headroom.
- Commit checks require ciphertext-length plus reserve headroom.
- Create and upload disk rejects do not create new persisted session/object/part
  state.
- Commit disk rejects do not create committed objects; they preserve only the
  pre-existing contract-allowed committable session and staged part.
- Rejected quota/disk writes do not resurrect after same-root restart.
- Request-path cleanup still runs before mutation and can remove expired
  contract-eligible state while preserving unexpired committed objects.
- Partial staged/object artifacts remain fail-closed under startup
  reconciliation: orphan staged parts, object directories without metadata, and
  committed objects with mismatched `ciphertext.bin` length are discarded.
- Wrong-resource resume tokens and fetch capabilities cannot write, fetch, or
  mutate another resource under pressure.
- Repeated quota/disk/fetch-capability abuse loops remain bounded, do not panic,
  and do not grow the test tempdir.
- The service remains opaque-ciphertext only and never decrypts client
  plaintext.
- Quota/disk/abuse logs and audit events use redacted handles and reason codes
  only.

## Tests Run

qsl-attachments local validation completed green:

- `cargo fmt --check`
- `cargo test --locked --test service_contract -- --test-threads=1`
- `cargo test --locked --test reject_taxonomy_harness -- --test-threads=1`
- `cargo test --locked --test retention_cleanup_recovery -- --test-threads=1`
- `cargo test --locked --test retention_cleanup_logging -- --test-threads=1`
- `cargo test --locked --test disk_pressure_quota_abuse -- --test-threads=1`
- `cargo test --locked --test disk_pressure_quota_logging -- --test-threads=1`
- `cargo test --locked`
- `cargo clippy --locked --all-targets -- -D warnings`
- `cargo audit --deny warnings`
- `git diff --check`
- direct overclaim scan over changed files
- direct captured-output sentinel scan for the logging harness

The qsl-attachments PR required `rust` check completed success before merge.
After merge, qsl-attachments `origin/main` at `4ae5ceab6f1a` passed:

- `cargo audit --deny warnings`
- `cargo test --locked`
- `cargo clippy --locked --all-targets -- -D warnings`

## Harness Coverage

### Quota Rejects

`quota_rejects_do_not_persist_objects_sessions_or_parts` proves oversize create
rejects return `REJECT_QATTSVC_QUOTA`, leave no session/object entries, and do
not recover any rejected state after same-root restart.

### Open-Session Quota

`open_session_quota_rejects_and_release_is_deterministic` proves the
deployment-global open-session ceiling rejects an extra session without creating
extra state, and that aborting the first open session releases capacity for a
new session under current semantics.

### Disk-Headroom / Pressure

`low_headroom_create_upload_commit_rejects_are_fail_closed` proves simulated
low headroom rejects create, upload, and commit deterministically with
`REJECT_QATTSVC_QUOTA`.

The test proves:

- create reject leaves zero sessions and zero objects;
- upload reject leaves zero staged parts and a session status with
  `stored_part_count = 0`;
- commit reject leaves zero objects and preserves only the already staged
  committable session/part.

### No-Persistence After Reject

`quota_disk_rejected_writes_do_not_resurrect_after_restart` proves quota create
rejects, disk upload rejects, and disk commit rejects do not resurrect new
objects or rejected parts after same-root restart.

The chosen contract remains explicit: a commit disk reject may preserve the
pre-existing committable session and staged part, but it must not create a
committed object.

### Cleanup Under Pressure

`cleanup_under_pressure_preserves_unexpired_valid_committed_object` proves
request-path cleanup can remove an expired committed object while a later create
is rejected for simulated low headroom. The unexpired committed object remains
fetchable and its `ciphertext.bin` stays present.

### Restart / Recovery

The new no-resurrection test and existing retention/recovery tests prove:

- rejected create/upload/commit state does not become recoverable after restart;
- coherent open sessions remain resumable;
- incoherent sessions are discarded fail-closed;
- paired committed objects remain fetchable;
- mismatched or incomplete committed object artifacts are discarded.

### Partial Write Boundary

`partial_write_recovery_boundary_is_explicit` proves startup reconciliation
removes:

- an unjournaled staged part under an otherwise coherent session;
- an object directory containing `ciphertext.bin` without `object.json`;
- a committed object whose `ciphertext.bin` length no longer matches
  `object.json`.

It also proves the coherent session remains open with zero staged parts and the
incoherent object is not fetchable.

### Capability Scope

`wrong_resource_capability_cannot_bypass_quota_or_fetch_other_object` proves a
resume token for one session cannot write another session even under simulated
low disk headroom, and a fetch capability for one object cannot fetch another
object. Valid object fetch remains available with the correct capability.

### Opaque Ciphertext

The harness stores and fetches opaque byte strings only. The service never
decrypts client plaintext, and the logging harness asserts ciphertext and
plaintext sentinels are absent from captured logs and audit events.

### Logging / No-Secret

`pressure_logs_redact_capability_descriptor_ciphertext_plaintext` captures the
quota/disk/cleanup/abuse paths and asserts logs/audit events omit:

- session IDs;
- resume tokens;
- locator refs;
- fetch capabilities;
- attachment IDs;
- descriptor sentinels;
- ciphertext sentinels;
- plaintext sentinels;
- wrong-capability sentinels.

The captured-output sentinel scan for the logging test also passed.

### No-Panic

`bounded_abuse_loop_has_no_panic_and_no_unbounded_growth` proves repeated
quota rejects and repeated wrong fetch-capability attempts do not panic and do
not grow the test tempdir beyond the pre-existing committed object. Full
`cargo test --locked` and clippy also completed green.

## Results

Passed:

- qsl-attachments local validation bundle listed above.
- qsl-attachments PR #34 `rust` required check.
- qsl-attachments PR #34 merge to `main`.
- qsl-attachments merged-main audit, test, and clippy checks.

Recovered local formatting issue:

- Initial `cargo fmt --check` failed on the newly added test files only.
- Classification: recoverable local formatting failure.
- Corrective action: ran `cargo fmt`, then reran `cargo fmt --check`.
- Final result: formatting check passed.

Recovered local warning issue:

- Initial dedicated test run showed a dead-code warning for an unused helper in
  the new disk pressure test file.
- Classification: recoverable test-harness hygiene issue that would fail the
  required clippy gate.
- Corrective action: removed the unused helper and reran the dedicated tests.
- Final result: targeted tests, full tests, and clippy passed.

Remaining semantic decisions:

- Disk pressure remains simulated by configured test disk-space providers, not
  by host disk exhaustion.
- Commit disk reject preserves the pre-existing committable session/part and
  rejects before creating an object.
- Cleanup under pressure remains request-path cleanup, not a background worker.
- Hot/live backup, partial restore, cross-node replication, and stronger
  crash-transaction guarantees remain unsupported.
- Capability replay / duplicate-use semantics beyond the pressure paths remain
  the recommended successor lane.

## No-Production Boundary

This evidence does not claim production readiness, deployment readiness, public
internet readiness, external review completion, metadata elimination, strong
identity-hiding guarantees, or a completed release-security proof.

The harness is local/loopback executable evidence for the current single-node
local-disk attachment service boundary only.

## Next Recommended Harness

Promote the next executable qsl-attachments hardening lane as:

`NA-0284 — qsl-attachments Capability Scope / Abuse / Logging Harness`

Recommended scope:

- wrong-resource capability rejects;
- replay or duplicate capability use where applicable;
- unauthorized fetch/delete/update rejection;
- bounded abuse loops;
- no secret/plaintext/ciphertext logging;
- no qsl-protocol runtime, qsl-server, website, workflow, branch-protection,
  public-safety, or dependency drift.
