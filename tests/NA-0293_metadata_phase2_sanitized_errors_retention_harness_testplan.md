Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-15
Replaces: n/a
Superseded-By: n/a

# NA-0293 Metadata Phase-2 Sanitized Errors and Retention/Purge Harness Test Plan

## Objective

Validate that NA-0293 adds executable metadata phase-2 harness proof for
sanitized-error and retention/purge policy fixtures without changing runtime,
protocol, crypto, service, dependency, public-copy, branch-protection, or
workflow behavior.

## Protected Invariants

- Metadata phase-2 remains evidence-bound and incomplete.
- Sanitized-error proof remains a bounded fixture harness.
- Retention/purge proof remains bounded fixture policy, not production deletion
  or retention behavior.
- No qsp protocol-core change.
- No cryptographic state-machine, handshake, key-schedule, QSP wire-format, or
  downgrade behavior change.
- No qsl-server, qsl-attachments, qsc-desktop, website, README, START_HERE,
  workflow, Cargo, lockfile, dependency, branch-protection, or public-safety
  configuration change.
- No anonymity, metadata-free, untraceable, external-review-complete,
  production-readiness, or public-internet-readiness claim.

## Allowed/Forbidden Scope

Allowed paths:

- `inputs/metadata_phase2/sanitized_errors_retention_policy_vectors_v1.json`
- `scripts/ci/metadata_phase2_sanitized_errors_retention_harness.sh`
- `docs/governance/evidence/NA-0293_metadata_phase2_sanitized_errors_retention_harness.md`
- `tests/NA-0293_metadata_phase2_sanitized_errors_retention_harness_testplan.md`
- `docs/public/RELEASE_READINESS_EVIDENCE_MAP.md`
- `docs/public/EXTERNAL_REVIEW_PACKAGE.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Forbidden paths include:

- `README.md`
- `START_HERE.md`
- `.github/**`
- `Cargo.toml`
- `Cargo.lock`
- `qsp/**`
- qsp protocol-core paths under `tools/refimpl/**`
- cryptographic state-machine, handshake, key-schedule, and QSP wire-format
  implementation paths
- `qsl-server/**`
- `qsl-attachments/**`
- `qsc-desktop/**`
- `website/**`
- dependency files
- branch-protection and public-safety configuration.

## Executable Harness Requirements

The harness must:

- parse `inputs/metadata_phase2/sanitized_errors_retention_policy_vectors_v1.json`;
- enforce a positive bounded `max_fixture_count`;
- exit nonzero on invariant failure;
- emit the stable markers only after all checks pass:
  - `NA0293_SANITIZED_ERROR_POLICY_OK`
  - `NA0293_RETENTION_PURGE_POLICY_OK`
  - `NA0293_METADATA_PHASE2_SANITIZED_RETENTION_OK`

## Sanitized-Error Requirements

The harness must prove coarse deterministic reject mapping for:

- malformed metadata;
- invalid identifier or handle;
- invalid padding config;
- oversized metadata;
- unauthorized route/capability-like input;
- expired/deleted/purged state access.

It must reject forbidden output fields and panic/backtrace text, and rejected
inputs must not mutate accepted fixture state.

## Retention/Purge Requirements

The harness must prove:

- deleted state does not resurrect;
- expired state does not resurrect;
- purged state does not resurrect;
- tombstoned/future-marker state is coarse and deterministic;
- rejected-state fixture input does not create state;
- retention-window fixtures are deterministic and bounded;
- purge/deletion error fixtures do not reveal sensitive details.

## Cross-Surface Integration Requirements

Run when present and feasible:

- `scripts/ci/metadata_phase2_identifier_padding_harness.sh`;
- `scripts/ci/metadata_conformance_smoke.sh`;
- `scripts/ci/demo_cli_smoke.sh`.

Run `DEMO_STRESS_PROFILE=baseline scripts/ci/demo_adversarial_stress.sh` as a
separate validation command if time and environment permit.

## Negative Tests

Negative coverage must include:

- invalid metadata fixture rejects;
- malformed fixture rejects;
- oversized fixture rejects or is bounded;
- secret-bearing fixture values do not leak;
- panic/backtrace strings cause harness failure;
- forbidden output fields cause harness failure.

## Leak/Panic Requirements

The harness and validation must scan for:

- route tokens;
- capabilities;
- identifiers and handles;
- descriptors;
- plaintext and ciphertext sentinels;
- internal-path sentinels;
- panic/backtrace indicators.

## Crypto Boundary Requirements

Validation must prove:

- no qsp protocol-core path changed;
- no crypto state-machine path changed;
- no handshake/key-schedule path changed;
- no QSP wire-format path changed;
- no downgrade behavior changed;
- no Cargo or dependency path changed.

## Claim Boundary Requirements

Validation must scan changed content for unsupported claims and preserve:

- no anonymity claim;
- no metadata-free claim;
- no untraceable claim;
- no external-review-complete claim;
- no production-readiness claim;
- no public-internet-readiness claim;
- no metadata phase-2 completion claim.

## CI Expectations

- Required checks attach and pass normally.
- `public-safety` remains required and green.
- Script/input changes may trigger full-suite behavior; do not bypass.
- `cargo audit --deny warnings` passes.
- `cargo tree -i rustls-webpki --locked` continues to show
  `rustls-webpki v0.103.13`.
- `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1`
  passes.
- `python3 formal/run_model_checks.py` passes.

## Successor Handoff

After the NA-0293 harness PR merges and post-merge public-safety is green, a
separate closeout may mark NA-0293 DONE and restore exactly one successor:

`NA-0294 - Public Evidence Navigation and README/START_HERE Attention Refresh`

NA-0294 must not be implemented by this test plan.
