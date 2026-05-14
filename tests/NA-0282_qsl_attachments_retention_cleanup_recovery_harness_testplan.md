Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-14

# NA-0282 qsl-attachments Retention / Cleanup / Recovery Harness Testplan

Goals: G1, G3, G4, G5

## Objective

Record the executable proof required for `NA-0282 — qsl-attachments Retention / Cleanup / Recovery Harness`.

The qsl-attachments sibling PR must prove expiration/cleanup behavior, restart/recovery boundaries, reject no-persistence, opaque ciphertext preservation, capability scope, delete/abort determinism, and no secret/plaintext leakage in cleanup/recovery evidence.

## Protected Invariants

- qsl-attachments remains opaque-ciphertext only.
- qsl-server remains transport-only and untouched.
- qsl-protocol remains implementation-clean.
- Rejected requests do not create recoverable objects, sessions, or parts beyond the current contract-allowed pre-existing session state.
- Expired or deleted resources are not fetchable under the chosen semantics.
- Cleanup does not remove unexpired committed objects.
- Restart recovery re-exposes only contract-allowed coherent state.
- Capabilities, descriptors, ciphertext, plaintext, and sentinel secrets do not leak in logs, audit snapshots, recovery summaries, docs, or evidence.
- Production readiness is not claimed.

## qsl-attachments Executable Harness Proof

Required qsl-attachments proof:

- `retention_cleanup_recovery` integration test target.
- `retention_cleanup_logging` integration test target.
- Existing `service_contract` target remains green.
- Existing `reject_taxonomy_harness` target remains green.
- Full `cargo test --locked` remains green.
- `cargo clippy --locked --all-targets -- -D warnings` remains green.
- `cargo audit --deny warnings` remains green.

## Chosen Semantics

- Expiry cleanup remains request-path triggered by service operations.
- Expired sessions are marked expired, staged parts are removed, and resume-token hashes are cleared.
- Expired committed objects are marked expired, `ciphertext.bin` is removed, and fetch-capability hashes are cleared.
- Unexpired committed objects survive cleanup.
- Same-root restart recovery preserves only coherent open sessions and paired committed objects.
- Incoherent or orphaned local state is discarded fail-closed.
- Session `DELETE` remains abort; object delete is not introduced.
- Hot/live backup, partial restore, cross-node replication, and stronger crash-transaction guarantees remain unsupported.

## Allowed / Forbidden qsl-attachments Scope

Allowed:

- `tests/**/*.rs`
- `tests/*.rs`
- `src/**` only if a bounded test-proven defect requires minimal implementation repair
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

Actual result: qsl-attachments PR #33 changed only:

- `tests/retention_cleanup_logging.rs`
- `tests/retention_cleanup_recovery.rs`

## qsl-protocol Evidence Scope

Allowed qsl-protocol scope:

- `docs/governance/evidence/NA-0282_qsl_attachments_retention_cleanup_recovery_harness.md`
- `tests/NA-0282_qsl_attachments_retention_cleanup_recovery_harness_testplan.md`
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

The harness must show the service stores and returns opaque ciphertext bytes only. It must not introduce any service-side plaintext handling or decryption.

Plaintext sentinels may appear only as test-side negative markers asserted absent from logs/audit/recovery output.

## No Production-Readiness Claim

The evidence must not claim production readiness, deployment readiness, production relay readiness, production attachment readiness, external review completion, metadata-free behavior, anonymity, untraceability, quantum-proof status, or a proven true Triple Ratchet.

Unsupported boundaries must remain visible.

## Link / Leak / Goal-Lint Expectations

Expected qsl-protocol validation:

- queue parser reports `READY_COUNT 1` and `READY NA-0282`;
- decisions parser reports D-0534 exactly once and no duplicate IDs;
- scope guard accepts only allowed paths;
- link check passes;
- leak scan passes;
- overclaim scan has no affirmative prohibited claims;
- PR body has a standalone `Goals: G1, G3, G4, G5` line near the top.

## CI Expectations

- qsl-attachments PR #33 required `rust` check is green before merge.
- qsl-protocol evidence PR required checks attach and pass normally.
- qsl-protocol public-safety remains required and green before and after merge.
- qsl-protocol docs/governance-only classification should avoid unnecessary full-suite cost under NA-0262A.

## Successor Handoff

Recommended successor:

`NA-0283 — qsl-attachments Disk Pressure / Quota / Abuse Harness`

The successor should remain executable qsl-attachments harness work and must not implement production deployment, branch-protection changes, qsl-server changes, qsl-protocol runtime changes, website changes, or dependency changes unless explicitly authorized by a future directive.
