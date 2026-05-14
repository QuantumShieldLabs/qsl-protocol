Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-14

# NA-0283 qsl-attachments Disk Pressure / Quota / Abuse Harness Testplan

Goals: G1, G3, G4, G5

## Objective

Record the executable proof required for `NA-0283 — qsl-attachments Disk
Pressure / Quota / Abuse Harness`.

The qsl-attachments sibling PR must prove deterministic quota rejects,
open-session quota behavior, simulated disk-headroom rejects, no unexpected
persistence on rejected writes, cleanup under pressure, same-root restart /
recovery after rejects, partial write boundaries, capability scope under
pressure, opaque-ciphertext handling, no secret/plaintext leakage, and no panic
under bounded abuse.

## Protected Invariants

- qsl-attachments remains opaque-ciphertext only.
- qsl-server remains untouched.
- qsl-protocol remains implementation-clean.
- Quota and disk-pressure rejects are deterministic and reason-coded.
- Rejected writes do not persist recoverable objects, sessions, or parts beyond
  the current contract-allowed pre-existing session state.
- Cleanup under pressure is explicit and preserves unexpired valid committed
  objects.
- Same-root restart does not resurrect rejected quota/disk writes.
- Partial staged/object artifacts fail closed under startup reconciliation.
- Capabilities remain resource-scoped.
- Capabilities, descriptors, ciphertext, plaintext, and sentinel secrets do not
  leak in logs, audit snapshots, test output, docs, or evidence.
- Production readiness is not claimed.

## qsl-attachments Executable Harness Proof

Required qsl-attachments proof:

- `disk_pressure_quota_abuse` integration test target.
- `disk_pressure_quota_logging` integration test target.
- Existing `service_contract` target remains green.
- Existing `reject_taxonomy_harness` target remains green.
- Existing `retention_cleanup_recovery` target remains green.
- Existing `retention_cleanup_logging` target remains green.
- Full `cargo test --locked` remains green.
- `cargo clippy --locked --all-targets -- -D warnings` remains green.
- `cargo audit --deny warnings` remains green.

## Chosen Semantics

- Oversize create rejects use `REJECT_QATTSVC_QUOTA` and create no state.
- Open-session quota is deployment-global and extra sessions reject without
  creating extra state.
- Aborting or completing a session releases open-session capacity under the
  current contract.
- Disk pressure is simulated by `TestDiskSpace` and tempdirs; tests do not fill
  the host disk.
- Create/upload/commit disk-headroom checks reject before creating new state
  beyond the chosen contract.
- Commit disk reject may preserve the already staged committable session/part,
  but must not create a committed object.
- Request-path cleanup can remove expired contract-eligible state under
  pressure and must not remove unexpired valid committed objects.
- Startup reconciliation discards orphan or incoherent staged/object artifacts
  fail-closed.
- Wrong-resource resume tokens/fetch capabilities cannot write or fetch another
  resource.
- Abuse loops remain bounded and do not grow the test tempdir.
- Hot/live backup, partial restore, cross-node replication, public exposure,
  deployment automation, and stronger crash-transaction guarantees remain
  unsupported.

## Allowed / Forbidden qsl-attachments Scope

Allowed:

- `tests/**/*.rs`
- `tests/*.rs`
- `src/**` only if a bounded test-proven defect requires minimal implementation
  repair
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

Actual result: qsl-attachments PR #34 changed only:

- `tests/disk_pressure_quota_abuse.rs`
- `tests/disk_pressure_quota_logging.rs`

## qsl-protocol Evidence Scope

Allowed qsl-protocol scope:

- `docs/governance/evidence/NA-0283_qsl_attachments_disk_pressure_quota_abuse_harness.md`
- `tests/NA-0283_qsl_attachments_disk_pressure_quota_abuse_harness_testplan.md`
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
only. It must not introduce service-side plaintext handling or decryption.

Plaintext and ciphertext sentinels may appear only as test-side negative
markers asserted absent from logs, audit events, and captured output.

## No Production-Readiness Claim

The evidence must not claim production readiness, deployment readiness, public
internet readiness, external review completion, metadata elimination, strong
identity-hiding guarantees, or a completed release-security proof.

Unsupported boundaries must remain visible.

## Link / Leak / Goal-Lint Expectations

Expected qsl-protocol validation:

- queue parser reports `READY_COUNT 1` and `READY NA-0283`;
- decisions parser reports D-0536 exactly once and no duplicate IDs;
- D-0537 remains absent before closeout;
- scope guard accepts only allowed paths;
- link check passes;
- leak scan passes;
- overclaim scan has no affirmative prohibited claims;
- PR body has a standalone `Goals: G1, G3, G4, G5` line near the top.

## CI Expectations

- qsl-attachments PR #34 required `rust` check is green before merge.
- qsl-protocol evidence PR required checks attach and pass normally.
- qsl-protocol public-safety remains required and green before and after merge.
- qsl-protocol docs/governance-only classification should avoid unnecessary
  full-suite cost under NA-0262A.

## Successor Handoff

Recommended successor:

`NA-0284 — qsl-attachments Capability Scope / Abuse / Logging Harness`

The successor should remain executable qsl-attachments harness work and must not
implement production deployment, branch-protection changes, qsl-server changes,
qsl-protocol runtime changes, website changes, or dependency changes unless
explicitly authorized by a future directive.
