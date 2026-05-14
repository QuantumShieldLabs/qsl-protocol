Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-14

# NA-0286 qsl-attachments Backup / Partial Restore / Transactional Recovery Harness

Goals: G1, G3, G4, G5

## Executive Summary

NA-0286 adds executable qsl-attachments backup / partial restore /
transactional recovery proof in the sibling `QuantumShieldLabs/qsl-attachments`
repository.

The qsl-attachments PR is harness-only. It does not change service source,
dependencies, workflows, deployment posture, protocol behavior, qsl-server
behavior, qsl-protocol runtime behavior, qsc-desktop, or website surfaces.

The harness proves stopped/quiesced full-root copy into a new temp storage
root, coherent committed-object recovery, best-effort coherent open-session
recovery, fail-closed partial restore fixtures, fail-closed mismatched object
metadata, no resurrection of rejected/expired/deleted/aborted state,
opaque-ciphertext preservation, no secret/plaintext leakage in logs/audit
snapshots/recovery summaries/error bodies, and no panic on the exercised
restore/recovery paths. It keeps the existing malformed JSON / reject-taxonomy,
retention / cleanup / recovery, disk pressure / quota / abuse, and capability
scope / abuse / logging harnesses green.

## NA-0285 Boundary Carried Forward

NA-0285 established the current boundary:

- qsl-attachments is a single-node local-disk runtime.
- The storage root is the durability boundary.
- Same-root startup reconciliation is supported and test-backed.
- Cold or quiesced full-root backup/restore plus matching service
  configuration is the only documented backup shape for executable proof.
- Hot/live backup while mutations continue is unsupported.
- Partial restore is unsupported and must fail closed unless a future lane
  proves a narrower supported case.
- Cross-file transactional durability is not claimed.
- Reconciliation must discard or reject incoherent artifacts rather than
  reconstruct missing journals, missing parts, missing object metadata, missing
  object bytes, plaintext, or capability material.

## qsl-attachments PR Evidence

- Repository: `QuantumShieldLabs/qsl-attachments`
- PR: #36, <https://github.com/QuantumShieldLabs/qsl-attachments/pull/36>
- Head SHA: `fafd4cecb614`
- Merge SHA: `320be68fe632`
- Changed paths:
  - `tests/backup_restore_logging.rs`
  - `tests/backup_restore_recovery.rs`
- Implementation changed: no service source changed.
- Dependency changed: no.
- Workflow changed: no.
- Production readiness claim: no.
- Production backup/restore readiness claim: no.
- Test posture: loopback/local/tempdir executable harness only.

## Chosen Backup / Partial Restore / Transactional Recovery Semantics

- Cold full-root restore means a stopped/quiesced copy of the entire storage
  root plus matching service configuration, then starting a new service state
  on the copied root.
- Coherent committed objects are recovered only when `object.json` and
  `ciphertext.bin` both survive with matching locator and ciphertext length.
- Coherent open sessions are best-effort resumable only when `session.json`
  survives and every journaled staged part exists with the expected length.
- Partial restore remains unsupported. Metadata-only objects, bytes-only
  objects, metadata-only sessions, missing journaled parts, and orphan parts
  fail closed.
- Mismatched object metadata, including a locator mismatch or ciphertext length
  mismatch, fails closed and is not fetchable.
- Rejected, expired, deleted, and aborted state does not resurrect after
  restore or reconciliation.
- The service stores and returns opaque ciphertext bytes only and never
  decrypts client plaintext.
- Logs, audit snapshots, recovery summaries, and error bodies must not include
  capabilities, descriptor sentinels, ciphertext sentinels, plaintext
  sentinels, or wrong-capability sentinels.

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
- `cargo test --locked --test backup_restore_recovery -- --test-threads=1`
- `cargo test --locked --test backup_restore_logging -- --test-threads=1`
- `cargo test --locked`
- `cargo clippy --locked --all-targets -- -D warnings`
- `cargo audit --deny warnings`
- `git diff --cached --check`
- direct overclaim scan over changed files
- direct changed-file secret-pattern scan

The qsl-attachments PR required `rust` check completed success before merge.
After merge, qsl-attachments `origin/main` at `320be68fe632` passed:

- `cargo audit --deny warnings`
- `cargo test --locked`
- `cargo clippy --locked --all-targets -- -D warnings`

## Harness Coverage

### Cold Full-Root Backup / Restore

`cold_full_root_restore_recovers_only_coherent_committed_state` creates a
coherent committed object and a coherent open session, copies the entire
storage root to a new temp root, starts a new `AppState` with matching config,
and proves:

- the committed object remains fetchable with its existing fetch capability;
- the service returns the same opaque ciphertext bytes;
- the coherent open session remains resumable under the best-effort boundary;
- the restored open session can receive the missing part and commit normally;
- recovery counts report one recovered committed object and one resumable
  session.

### Partial Restore Fail-Closed

`partial_restore_object_json_without_ciphertext_fails_closed` proves an object
directory containing only `object.json` is discarded and not fetchable.

`partial_restore_ciphertext_without_object_json_fails_closed` proves an object
directory containing only `ciphertext.bin` is discarded and not fetchable.

`partial_restore_orphan_parts_and_missing_parts_fail_closed` proves
`session.json` without the required journaled part is discarded, an orphan
parts-only session directory is discarded, and neither becomes resumable.

### Transactional Recovery Invariants

`mismatched_descriptor_or_object_metadata_fails_closed` corrupts restored
object metadata by changing the stored locator reference and by changing the
declared ciphertext length. Startup reconciliation discards both objects, and
neither locator is fetchable.

The same fixtures prove that recovery metadata does not override the
object-pair boundary: `object.json` must match its directory and
`ciphertext.bin` length before the object is exposed.

### No Resurrection

`rejected_expired_deleted_and_aborted_state_do_not_resurrect` proves:

- malformed rejected create residue does not leak the plaintext sentinel;
- an expired session remains expired after full-root restore;
- an expired object remains expired after full-root restore and does not
  regain ciphertext bytes or a fetchable capability;
- an aborted session remains unusable after full-root restore;
- a committed session removed during commit does not regain resumable session
  access after restore.

### Opaque Ciphertext

The cold full-root restore test fetches the exact sentinel ciphertext bytes
that were committed before the root copy. No test adds service-side plaintext
handling or decryption. The logging test uses a separate plaintext sentinel
that is never accepted as service payload and asserts it is absent from every
captured output surface.

### Logging / No-Secret

`backup_restore_logs_redact_capability_descriptor_ciphertext_plaintext`
captures malformed JSON, commit, full-root restore fetch, wrong fetch
capability, and metadata-only partial restore paths. It asserts logs, original
and restored audit snapshots, recovery summaries, and error bodies omit:

- resume tokens;
- fetch capabilities;
- wrong-capability sentinels;
- descriptor sentinels;
- ciphertext sentinels;
- plaintext sentinels.

### No-Panic

Focused serial harnesses, full qsl-attachments tests, and clippy completed
green. No panic path was observed in full-root, partial, mismatched, expired,
aborted, rejected, or logging fixtures.

## Results

Passed:

- qsl-attachments local validation bundle listed above.
- qsl-attachments PR #36 `rust` required check.
- qsl-attachments PR #36 merge to `main`.
- qsl-attachments merged-main audit, test, and clippy checks.

Failed tests or checks fixed locally:

- `cargo fmt --check` first reported rustfmt diffs in the two new test files.
  Classification: recoverable in-scope formatting failure. Corrective action:
  ran `cargo fmt`. Final result: `cargo fmt --check` passed.
- `cargo test --locked --test backup_restore_logging -- --test-threads=1`
  first failed because the restored logging fixture used `SystemClock`, so a
  fixed-test-clock object looked expired. Classification: recoverable
  test-shape defect. Corrective action: restored logging fixtures now reuse
  the `TestClock` and `TestDiskSpace` from the source fixture. Final result:
  the logging test passed and the full suite passed.

Remaining semantic decisions:

- Hot/live backup remains unsupported.
- Partial restore remains unsupported except for fail-closed handling.
- Cross-file transactional durability is not claimed.
- Open-session restore remains best-effort and requires coherent
  `session.json` plus matching journaled parts.
- Broader backup automation, object-store integration, multi-node replication,
  and production deployment remain future work.

## No-Production Boundary

This evidence does not claim production readiness, deployment readiness, public
internet readiness, external review completion, metadata elimination, strong
identity-hiding guarantees, production backup/restore readiness, hot/live
backup support, partial restore support, cross-node replication support, or a
completed release-security proof.

The proof is limited to qsl-attachments local tempdir harnesses over the
current single-node local storage root contract.

## Next Recommended Harness

The next recommended service lane is `NA-0287 — Service Production-Gate
Evidence Map and Deployment Boundary Plan`: map the matured qsl-server and
qsl-attachments service-hardening evidence into explicit production-gate
boundaries without making a production-readiness claim.
