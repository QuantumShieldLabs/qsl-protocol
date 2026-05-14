Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-14

# NA-0282 qsl-attachments Retention / Cleanup / Recovery Harness

Goals: G1, G3, G4, G5

## Executive Summary

NA-0282 adds executable qsl-attachments retention / cleanup / recovery proof in the sibling `QuantumShieldLabs/qsl-attachments` repository.

The qsl-attachments PR is tests-only. It does not change service source, dependencies, workflows, deployment posture, protocol behavior, qsl-server behavior, qsl-protocol runtime behavior, qsc-desktop, or website surfaces.

The harness proves deterministic request-path cleanup, explicit same-root restart/recovery boundaries, reject no-persistence, opaque ciphertext preservation, capability scope, session delete/abort determinism, and secret-safe cleanup/recovery output. It keeps the existing malformed JSON / reject-taxonomy harness green.

## qsl-attachments PR Evidence

- Repository: `QuantumShieldLabs/qsl-attachments`
- PR: #33, <https://github.com/QuantumShieldLabs/qsl-attachments/pull/33>
- Head SHA: `b68a61e7546c`
- Merge SHA: `248665c8b85a`
- Changed paths:
  - `tests/retention_cleanup_logging.rs`
  - `tests/retention_cleanup_recovery.rs`
- Implementation changed: no.
- Dependency changed: no.
- Workflow changed: no.
- Production-readiness claim: no.
- Test posture: loopback/local executable harness only.

## Chosen Retention / Cleanup / Recovery Semantics

qsl-attachments keeps the existing bounded single-node local-disk semantics:

- Expiry cleanup is deterministic and request-path triggered by canonical service operations.
- Expired open sessions are marked `expired_session`, their staged part bytes are removed, and their resume-token hash is cleared.
- Expired committed objects are marked `expired_object`, their `ciphertext.bin` bytes are removed, and their fetch-capability hash is cleared.
- Expired objects are not fetchable after cleanup; the service returns the existing expired reject.
- Unexpired committed objects survive cleanup and remain fetchable with the correct fetch capability.
- Same-root restart recovery re-exposes only coherent open sessions and committed objects with both `object.json` and `ciphertext.bin`.
- Incoherent sessions, orphan staged artifacts, object records missing paired ciphertext, and object ciphertext missing paired metadata are discarded fail-closed.
- Malformed JSON rejects do not create session/object state.
- Missing or wrong session capabilities do not create staged parts.
- Wrong-resource fetch capabilities cannot read another object and do not mutate committed object availability.
- Session `DELETE` remains the current abort behavior; repeated deletes fail closed and do not resurrect access.
- No object delete endpoint is introduced by this lane.
- The service remains opaque-ciphertext only and never decrypts client plaintext.
- Recovery remains limited to the current same-root local-disk boundary; hot/live backup, partial restore, cross-node replication, and stronger crash-transaction guarantees remain unsupported.

## Tests Run

qsl-attachments local validation completed green:

- `cargo fmt --check`
- `cargo test --locked --test service_contract -- --test-threads=1`
- `cargo test --locked --test reject_taxonomy_harness -- --test-threads=1`
- `cargo test --locked --test retention_cleanup_recovery -- --test-threads=1`
- `cargo test --locked --test retention_cleanup_logging -- --test-threads=1`
- `cargo test --locked`
- `cargo clippy --locked --all-targets -- -D warnings`
- `cargo audit --deny warnings`
- `git diff --check`
- direct overclaim scan over changed lines
- direct captured-output secret scan for the logging harness

The qsl-attachments PR required `rust` check completed success before merge.

## Harness Coverage

### Reject No-Persistence

`rejected_malformed_json_and_capability_requests_leave_no_recoverable_state` proves:

- malformed JSON create rejects with the bounded malformed JSON reason code;
- no sessions or objects are created by malformed create;
- same-root restart after the malformed reject has no recoverable session or object state;
- missing and wrong resume-token uploads do not create staged parts;
- the valid pre-existing session remains open with zero staged parts after restart.

Existing reject-taxonomy coverage also remains green for malformed commit, capability reject, wrong fetch capability, and opaque ciphertext round trip paths.

### Expiration / Cleanup

`expired_committed_object_is_removed_after_cleanup` proves an expired committed object is not fetchable after cleanup, loses `ciphertext.bin`, records `expired_object`, and does not retain the raw fetch capability.

`unexpired_committed_object_survives_cleanup` proves cleanup of an expired object does not remove an unexpired committed object. The unexpired object remains fetchable before and after restart.

### Restart / Recovery

`restart_recovery_preserves_only_contract_allowed_state` proves:

- a coherent partial session remains resumable after same-root restart;
- a session whose journal names a missing part is discarded fail-closed;
- a committed object with paired `object.json` and `ciphertext.bin` remains fetchable after restart;
- a committed object missing its paired ciphertext is discarded fail-closed;
- recovery summaries count only bounded recovery outcomes.

### Opaque Ciphertext

The new harness fetches committed bytes exactly as stored and keeps the existing `opaque_ciphertext_round_trip_preserves_bytes_and_hides_material_from_logs` reject-taxonomy test green.

The service never decrypts client plaintext. Plaintext sentinels used by tests are never submitted as service plaintext and are asserted absent from logs/audit/recovery output.

### Delete Behavior

`delete_and_repeated_fetch_are_deterministic` proves session delete/abort removes staged-part access, repeated delete fails closed, expired-object repeated fetch stays gone, and those fail-closed outcomes do not resurrect after same-root restart.

The current qsl-attachments service has no object delete route; this lane does not add one.

### Capability Scope

`wrong_resource_capability_cannot_access_other_resource` proves a fetch capability for one object cannot fetch another object, does not reduce availability of either committed object, and both valid objects remain recoverable after restart.

### Logging / No-Secret

`cleanup_recovery_logs_redact_capability_descriptor_ciphertext_plaintext` proves cleanup/audit/recovery output omits:

- session IDs;
- resume tokens;
- locator refs;
- fetch capabilities;
- attachment IDs;
- descriptor sentinels;
- ciphertext sentinels;
- plaintext sentinels.

The captured-output scan of the logging harness also passed.

### No-Panic

The targeted cleanup/recovery/reject tests and full `cargo test --locked` completed without panic. Fail-closed reject outcomes are asserted by status and reason code where relevant.

## Results

Passed:

- qsl-attachments local validation bundle listed above.
- qsl-attachments PR #33 `rust` required check.
- qsl-attachments PR #33 merge to `main`.

Recovered local test-shape issue:

- Initial `cargo test --locked --test retention_cleanup_recovery -- --test-threads=1` failed because the new repeated-delete assertion expected the wrong post-restart fail-closed status for the current abort contract.
- Classification: recoverable test-shape mismatch; service behavior was deterministic and fail-closed.
- Corrective action: align the test to the existing post-restart `REJECT_QATTSVC_SESSION_STATE` outcome.
- Final result: targeted and full validation passed.

Remaining semantic decisions:

- qsl-attachments still has no object delete route.
- Hot/live backup and partial restore remain unsupported.
- Abrupt-crash/open-session survival and cross-file transactional durability remain unsupported.
- Disk pressure / quota / abuse hardening remains the recommended successor lane.

## No-Production Boundary

This evidence does not claim production readiness, deployment readiness, public internet readiness, external review completion, metadata elimination, anonymity, untraceability, quantum-proof status, or a proven true Triple Ratchet.

The harness is local/loopback executable evidence for the current single-node local-disk attachment service boundary only.

## Next Recommended Harness

Promote the next executable qsl-attachments hardening lane as:

`NA-0283 — qsl-attachments Disk Pressure / Quota / Abuse Harness`

Recommended scope:

- quota reject determinism;
- no persistence on rejected writes;
- cleanup under pressure;
- abuse counter behavior;
- secret-safe quota/abuse logs;
- no qsl-protocol runtime, qsl-server, website, workflow, branch-protection, public-safety, or dependency drift.
