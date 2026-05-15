Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-15
Replaces: n/a
Superseded-By: n/a

# NA-0293 Metadata Phase-2 Sanitized Errors and Retention/Purge Harness

## Executive Summary

NA-0293 adds executable metadata phase-2 harness proof for sanitized-error and
retention/purge metadata policy fixtures. The selected path is a deterministic
qsl-protocol policy-fixture harness. It does not change runtime behavior,
protocol semantics, qsp protocol-core, cryptographic state-machine code,
qsl-server, qsl-attachments, qsc-desktop, website copy, README, START_HERE,
workflows, Cargo files, dependencies, branch protection, or public-safety
configuration.

The harness proves a narrow evidence lane only. It does not claim anonymity,
metadata-free messaging, untraceability, external review completion,
production readiness, public internet readiness, complete sanitized-error
coverage, or production retention/deletion guarantees. Metadata phase-2 remains
evidence-bound and incomplete beyond the fixture proof below.

## Selected Path

Path 1: executable harness using existing qsl-protocol surfaces only.

Rationale:

- NA-0292 supplied the sanitized-error and retention/purge policy design.
- Existing metadata smoke and NA-0291 harness surfaces already prove bounded
  metadata negative checks and identifier/padding policy fixtures.
- A dedicated deterministic harness can prove sanitized-error and
  retention/purge policy fixtures without touching forbidden protocol,
  crypto, service, dependency, website, or public-copy paths.

## Changed Files

- `inputs/metadata_phase2/sanitized_errors_retention_policy_vectors_v1.json`
- `scripts/ci/metadata_phase2_sanitized_errors_retention_harness.sh`
- `docs/governance/evidence/NA-0293_metadata_phase2_sanitized_errors_retention_harness.md`
- `tests/NA-0293_metadata_phase2_sanitized_errors_retention_harness_testplan.md`
- `docs/public/RELEASE_READINESS_EVIDENCE_MAP.md`
- `docs/public/EXTERNAL_REVIEW_PACKAGE.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Implementation changed: no runtime implementation changed. The executable
addition is a harness script plus deterministic policy-vector input.

## Sanitized-Error Proof

The vector file defines coarse safe status classes and reason codes for the
bounded fixture set:

- malformed metadata maps to `REJECT_METADATA_INVALID`;
- invalid identifier/handle input maps to `REJECT_METADATA_INVALID`;
- invalid padding config maps to `REJECT_METADATA_INVALID`;
- oversized metadata maps to `REJECT_METADATA_INVALID`;
- unauthorized route/capability-like input maps to
  `REJECT_METADATA_UNAUTHORIZED`;
- expired, deleted, and purged-state access map to
  `REJECT_METADATA_UNAVAILABLE`.

The harness builds sanitized error envelopes containing only `ok`,
`status_class`, and `reason_code`. It verifies allowed status/reason values,
rejects forbidden output keys, rejects panic/backtrace text, and checks that
raw route-token, capability, identifier, handle, descriptor, plaintext,
ciphertext, and internal-path sentinels do not appear in emitted artifacts.

Rejected sanitized-error fixtures do not mutate the accepted-state snapshot.

## Retention/Purge Metadata Proof

The retention/purge fixtures model active, deleted, expired, purged, and
tombstoned records. The harness proves:

- deleted state access stays deleted and returns a coarse unavailable reason;
- expired state access stays expired and returns a coarse unavailable reason;
- purged state access stays purged and returns a coarse unavailable reason;
- tombstoned access is coarse and deterministic;
- missing/rejected-state fixtures do not create new state;
- purge/deletion error fixtures do not reveal descriptor, path, capability,
  plaintext, or ciphertext sentinels.

The retention-window cases are deterministic and bounded by an explicit
`retention_window_seconds` fixture. They prove only fixture behavior, not a
production retention or deletion guarantee.

## Cross-Surface Integration Proof

The NA-0293 harness runs the policy-fixture model and, by default, executes:

- `scripts/ci/metadata_phase2_identifier_padding_harness.sh`;
- `scripts/ci/metadata_conformance_smoke.sh`;
- `scripts/ci/demo_cli_smoke.sh`.

The final NA-0293 markers are emitted only after those checks pass and the
harness artifact scan finds no sentinel or panic/backtrace leakage.

The baseline demo adversarial stress script remains a separate validation
command because it is heavier and already has its own artifact directory and
marker set.

## Negative Tests

Negative coverage includes:

- malformed metadata reject;
- invalid identifier/handle reject;
- invalid padding config reject;
- oversized metadata reject;
- unauthorized route/capability-like reject;
- expired/deleted/purged/tombstoned state reject;
- missing-record reject without state creation;
- scanner rejection for panic/backtrace text;
- scanner rejection for forbidden output fields and state-count hints.

## Leak/Panic Checks

The harness scans generated artifacts for:

- route-token sentinel;
- capability sentinel;
- identifier and handle sentinels;
- descriptor sentinel;
- plaintext and ciphertext sentinels;
- internal-path sentinel;
- panic/backtrace indicators.

Validation also runs the repository added-line leak scan and direct artifact
scans.

## Boundedness Proof

The harness is bounded by:

- `max_fixture_count` in the vector file;
- `max_metadata_len` of `4096`;
- `retention_window_seconds` of `3600`;
- finite fixture arrays;
- `NA0293_POLICY_TIMEOUT_SECONDS` defaulting to `120`;
- `NA0293_CROSS_SURFACE_TIMEOUT_SECONDS` defaulting to `900`;
- non-network deterministic policy logic, with optional local demo/smoke
  scripts remaining loopback/local only.

The harness exits nonzero on invariant failure.

## Crypto/Protocol Boundary Proof

NA-0293 does not change:

- qsp protocol-core paths;
- cryptographic state-machine paths;
- handshake or key schedule paths;
- QSP wire-format behavior;
- downgrade behavior;
- Cargo manifests or lockfiles;
- qsl-server, qsl-attachments, qsc-desktop, website, README, START_HERE,
  workflows, branch protection, or public-safety configuration.

## Claim Boundary Proof

This lane preserves:

- no anonymity claim;
- no metadata-free claim;
- no untraceable claim;
- no external-review-complete claim;
- no production-readiness claim;
- no public-internet-readiness claim;
- no claim that metadata phase-2 is complete;
- no claim that production sanitized-error behavior is complete;
- no claim that production retention or deletion guarantees are implemented.

## Remaining Gaps

- Runtime identifier rotation is not implemented.
- Runtime default padding is not implemented.
- Broader sanitized-error normalization remains future work.
- Production retention, purge, deletion, logs, metrics, proxy, backup, support
  bundle, and deployment behavior remain future-gated.
- Timing, batching, jitter, cover traffic, contact-graph hiding, IP/location
  hiding, and deployment metadata remain future-gated.
- External review completion remains not ready.

## Next Recommended Lane

After merge and public-safety success, close out NA-0293 and restore NA-0294
for claim-safe public evidence navigation and README/START_HERE attention
refresh. NA-0294 should remain docs-only and must preserve all claim
boundaries.
