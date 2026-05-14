Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-14

# NA-0284 qsl-attachments Capability Scope / Abuse / Logging Harness Testplan

Goals: G1, G3, G4, G5

## Objective

Record the executable proof required for `NA-0284 — qsl-attachments
Capability Scope / Abuse / Logging Harness`.

The qsl-attachments sibling PR must prove resource-scoped capabilities,
wrong-resource and malformed capability fail-closed behavior, deterministic
duplicate/replay behavior, delete/abort/expiry invalidation behavior,
unauthorized no-mutation/no-exposure behavior, bounded abuse loops,
opaque-ciphertext handling, no secret/plaintext leakage, and no panic under
bounded abuse.

## Protected Invariants

- qsl-attachments remains opaque-ciphertext only.
- qsl-server remains untouched.
- qsl-protocol remains implementation-clean.
- Capabilities remain resource-scoped.
- Wrong-resource and malformed capabilities fail closed.
- Unauthorized operations do not mutate sessions/objects or expose ciphertext
  or plaintext.
- Duplicate/replay capability behavior is deterministic and documented.
- Capabilities, descriptors, ciphertext, plaintext, and sentinel secrets do not
  leak in logs, audit snapshots, test output, docs, or evidence.
- Existing malformed JSON / reject-taxonomy, retention / cleanup / recovery,
  and disk pressure / quota / abuse harnesses remain green.
- Production readiness is not claimed.

## qsl-attachments Executable Harness Proof

Required qsl-attachments proof:

- `capability_scope_abuse` integration test target.
- `capability_scope_logging` integration test target.
- Existing `service_contract` target remains green.
- Existing `reject_taxonomy_harness` target remains green.
- Existing `retention_cleanup_recovery` target remains green.
- Existing `retention_cleanup_logging` target remains green.
- Existing `disk_pressure_quota_abuse` target remains green.
- Existing `disk_pressure_quota_logging` target remains green.
- Full `cargo test --locked` remains green.
- `cargo clippy --locked --all-targets -- -D warnings` remains green.
- `cargo audit --deny warnings` remains green.

## Chosen Semantics

- `resume_token` authorizes exactly one open session.
- `fetch_capability` authorizes exactly one committed object.
- Capabilities are reusable only inside that resource scope while the resource
  remains valid.
- Commit removes the open session and invalidates the resume token.
- Abort clears staged parts and invalidates the resume token.
- Session expiry clears staged parts and invalidates the resume token.
- Object expiry removes committed bytes and invalidates the fetch capability.
- Wrong-resource resume/fetch attempts reject first with resource capability
  reason codes, then with `REJECT_QATTSVC_ABUSE` after the configured limit.
- Missing/malformed capability headers reject with canonical reason codes.
- Broader production authorization service design, cross-node authorization
  state, backup/partial restore, and transactional recovery remain future work.

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

Actual result: qsl-attachments PR #35 changed only:

- `README.md`
- `tests/capability_scope_abuse.rs`
- `tests/capability_scope_logging.rs`
- `tests/support/mod.rs`

## qsl-protocol Evidence Scope

Allowed qsl-protocol scope:

- `docs/governance/evidence/NA-0284_qsl_attachments_capability_scope_abuse_logging_harness.md`
- `tests/NA-0284_qsl_attachments_capability_scope_abuse_logging_harness_testplan.md`
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
markers asserted absent from logs, audit events, error bodies, and captured
output.

## No Production-Readiness Claim

The evidence must not claim production readiness, deployment readiness, public
internet readiness, external review completion, metadata elimination, strong
identity-hiding guarantees, or a completed release-security proof.

Unsupported boundaries must remain visible.

## Link / Leak / Goal-Lint Expectations

Expected qsl-protocol validation:

- queue parser reports `READY_COUNT 1` and `READY NA-0284`;
- decisions parser reports D-0538 exactly once and no duplicate IDs;
- D-0539 remains absent before closeout;
- scope guard accepts only allowed paths;
- link check passes;
- leak scan passes;
- overclaim scan has no affirmative prohibited claims;
- PR body has a standalone `Goals: G1, G3, G4, G5` line near the top.

## CI Expectations

- qsl-attachments PR #35 required `rust` check is green before merge.
- qsl-protocol evidence PR required checks attach and pass normally.
- qsl-protocol public-safety remains required and green before and after merge.
- qsl-protocol docs/governance-only classification should avoid unnecessary
  full-suite cost under NA-0262A.

## Successor Handoff

Recommended successor:

`NA-0285 — qsl-attachments Backup / Partial Restore / Transactional Recovery Boundary Plan`

The successor should remain a qsl-attachments boundary planning lane and must
not implement qsl-attachments runtime behavior, production deployment,
branch-protection changes, qsl-server changes, qsl-protocol runtime changes,
website changes, or dependency changes unless explicitly authorized by a future
directive.
