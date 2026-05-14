Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-14

# NA-0286 qsl-attachments Backup / Partial Restore / Transactional Recovery Harness Testplan

Goals: G1, G3, G4, G5

## Objective

Record the executable proof required for `NA-0286 — qsl-attachments
Executable Backup / Partial Restore / Transactional Recovery Harness`.

The qsl-attachments sibling PR must prove cold full-root backup/restore
behavior, partial restore fail-closed boundaries, transactional recovery
invariants, no resurrection of rejected/expired/deleted/aborted state,
opaque-ciphertext preservation, no secret/plaintext logging, and no panic on
the exercised recovery paths.

## Protected Invariants

- qsl-attachments remains opaque-ciphertext only.
- qsl-server remains untouched.
- qsl-protocol remains implementation-clean.
- Cold full-root backup/restore behavior is deterministic under the local
  tempdir harness.
- Unsupported partial restore fails closed.
- Incoherent artifacts do not expose objects or plaintext.
- Rejected, expired, deleted, and aborted state does not resurrect.
- Capabilities, descriptors, ciphertext, plaintext, and sentinel secrets do
  not leak in logs, audit snapshots, recovery summaries, error bodies, docs, or
  evidence.
- Existing malformed JSON / reject-taxonomy, retention / cleanup / recovery,
  disk pressure / quota / abuse, and capability scope / abuse / logging
  harnesses remain green.
- Production readiness is not claimed.
- Production backup/restore readiness is not claimed.
- Hot/live backup support is not implied.

## qsl-attachments Executable Harness Proof

Required qsl-attachments proof:

- `backup_restore_recovery` integration test target.
- `backup_restore_logging` integration test target.
- Existing `service_contract` target remains green.
- Existing `reject_taxonomy_harness` target remains green.
- Existing `retention_cleanup_recovery` target remains green.
- Existing `retention_cleanup_logging` target remains green.
- Existing `disk_pressure_quota_abuse` target remains green.
- Existing `disk_pressure_quota_logging` target remains green.
- Existing `capability_scope_abuse` target remains green.
- Existing `capability_scope_logging` target remains green.
- Full `cargo test --locked` remains green.
- `cargo clippy --locked --all-targets -- -D warnings` remains green.
- `cargo audit --deny warnings` remains green.

## Chosen Semantics

- Cold full-root restore means stopped/quiesced copy of the whole storage root
  plus matching service config, then startup on the copied root.
- Coherent committed objects require paired `object.json` and `ciphertext.bin`
  with matching locator and ciphertext length.
- Coherent open sessions are best-effort resumable only when `session.json`
  and every journaled part file survive coherently.
- Partial restore remains unsupported and fail-closed.
- Object metadata-only, object bytes-only, session metadata-only, orphan part,
  missing journaled part, mismatched locator, and mismatched length fixtures do
  not expose object bytes or resumable state.
- Rejected, expired, deleted, and aborted state does not resurrect after
  restore/recovery.
- The service stores and returns opaque ciphertext bytes only and never
  decrypts client plaintext.

## Allowed / Forbidden qsl-attachments Scope

Allowed:

- `tests/**/*.rs`
- `tests/*.rs`
- `src/**` only if a bounded test-proven defect requires minimal
  implementation repair
- `README.md` or `docs/**` only if clarification is required

Forbidden in the harness PR:

- `Cargo.toml`
- `Cargo.lock`
- `.github/**`
- workflow changes
- dependency changes
- deployment files
- production deployment
- public internet exposure

Actual result: qsl-attachments PR #36 changed only:

- `tests/backup_restore_logging.rs`
- `tests/backup_restore_recovery.rs`

## qsl-protocol Evidence Scope

Allowed qsl-protocol scope:

- `docs/governance/evidence/NA-0286_qsl_attachments_backup_restore_recovery_harness.md`
- `tests/NA-0286_qsl_attachments_backup_restore_recovery_harness_testplan.md`
- `docs/public/QSL_SERVER_ATTACHMENTS_PRODUCTION_BOUNDARY_PLAN.md` only for
  handoff references
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Forbidden qsl-protocol scope:

- runtime/protocol/crypto code;
- qsl-server implementation;
- qsl-attachments implementation paths inside qsl-protocol;
- qsc/qsl apps;
- qsc-desktop;
- website/external website;
- `.github/**`;
- scripts;
- Cargo manifests/lockfiles;
- branch protection or public-safety configuration.

## Opaque-Ciphertext Boundary

The harness must show the service stores and returns opaque ciphertext bytes
only. It must not introduce service-side plaintext handling, decryption,
message-plane transcript parsing, plaintext filename handling, or descriptor
reconstruction.

Plaintext, descriptor, capability, and ciphertext sentinels may appear only as
test-side negative markers asserted absent from logs, audit events, recovery
summaries, error bodies, and captured output.

## No Production-Readiness Claim

The evidence must not claim production readiness, deployment readiness, public
internet readiness, external review completion, metadata elimination, strong
identity-hiding guarantees, production backup/restore readiness, hot/live
backup support, partial restore support, cross-node replication support, or a
completed release-security proof.

Unsupported boundaries and known gaps must remain visible.

## Link / Leak / Goal-Lint Expectations

Expected qsl-protocol validation:

- queue parser reports `READY_COUNT 1` and `READY NA-0286`;
- decisions parser reports D-0542 exactly once and no duplicate IDs;
- D-0543 remains absent before closeout;
- scope guard accepts only allowed paths;
- link check passes;
- leak scan passes;
- overclaim scan has no affirmative prohibited claims;
- PR body has a standalone `Goals: G1, G3, G4, G5` line near the top.

## CI Expectations

- qsl-attachments PR #36 required `rust` check is green before merge.
- qsl-protocol evidence PR required checks attach and pass normally.
- qsl-protocol public-safety remains required and green before and after merge.
- qsl-protocol docs/governance-only classification should avoid unnecessary
  full-suite cost under NA-0262A.

## Successor Handoff

Recommended successor:

`NA-0287 — Service Production-Gate Evidence Map and Deployment Boundary Plan`

The successor should map matured qsl-server and qsl-attachments service
hardening evidence into production-gate boundaries and deployment planning
without implementing NA-0287 inside NA-0286 and without making a production
readiness claim.
