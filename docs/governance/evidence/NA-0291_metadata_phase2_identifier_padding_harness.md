Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-15
Replaces: n/a
Superseded-By: n/a

# NA-0291 Metadata Phase-2 Identifier Rotation and Padding Defaults Executable Harness

## Executive Summary

NA-0291 adds the first executable metadata phase-2 harness for identifier /
opaque-handle policy and padding-default policy. The selected path is harness
proof only: it uses deterministic fixtures and a bounded local harness without
changing qsp protocol-core, cryptographic state-machine code, qsl-server,
qsl-attachments, qsc-desktop, website, README, START_HERE, Cargo files,
dependencies, branch protection, or public-safety configuration.

This evidence does not claim runtime identifier rotation, runtime default
padding, anonymity, metadata-free messaging, untraceability, external review
completion, production readiness, or public internet readiness. Metadata
phase-2 remains evidence-bound and incomplete.

## Selected Path

Path 1: executable harness using existing qsl-protocol surfaces only.

Rationale:

- NA-0290 supplied the identifier/padding policy design and harness
  requirements.
- Existing qshield and qsc surfaces already prove optional padding and
  metadata-smoke behavior, but not runtime identifier rotation or default
  padding.
- A dedicated deterministic harness can prove policy fixtures, negative
  behavior, boundedness, and claim boundaries without touching forbidden
  protocol, crypto, service, dependency, or public-copy paths.

## Changed Files

- `inputs/metadata_phase2/identifier_padding_policy_vectors_v1.json`
- `scripts/ci/metadata_phase2_identifier_padding_harness.sh`
- `docs/governance/evidence/NA-0291_metadata_phase2_identifier_padding_harness.md`
- `tests/NA-0291_metadata_phase2_identifier_padding_harness_testplan.md`
- `docs/public/RELEASE_READINESS_EVIDENCE_MAP.md`
- `docs/public/EXTERNAL_REVIEW_PACKAGE.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Implementation changed: no runtime implementation changed. The only executable
addition is a harness script plus deterministic policy-vector input.

## Identifier / Opaque Handle Proof

The vector file defines a deterministic opaque-handle policy with a test-only
derivation label, bounded identifier syntax, explicit maximum identifier
length, and fixture outputs.

The harness proves:

- the same sender/contact/route/session/epoch/operation tuple derives the same
  handle across repeated runs;
- the bounded fixture set has no unexpected handle collision;
- malformed, empty, ambiguous, and oversized identifier inputs reject
  deterministically;
- malformed handle text rejects deterministically;
- rejected identifier inputs do not mutate the accepted-state snapshot.

The handle values are harness vectors, not production tokens or authorization
secrets.

## Rotation-Policy Proof

The harness proves that the policy can distinguish these rotation boundaries:

- session boundary;
- epoch boundary;
- route/relay boundary;
- contact boundary;
- attachment-operation boundary.

The harness emits `DESIGN_ONLY_ROTATION_POLICY_PROOF` and does not claim runtime
rotation is implemented.

## Padding Policy Proof

The vector file defines the first named metadata phase-2 demo padding profile:
`metadata-phase2-demo-default-v1`, with buckets `512`, `1024`, `2048`, and
`4096`.

The harness proves:

- the bucket table is deterministic and strictly valid;
- representative valid lengths map to expected buckets and `pad_len` values;
- invalid bucket configs reject deterministically;
- payload length `0` and payload length above the largest bucket reject;
- maximum overhead across the bounded profile is exactly `2047` bytes;
- strip/verify logic restores the original payload for valid padded fixtures;
- invalid padding metadata rejects deterministically;
- padding metadata output does not include plaintext or the secret sentinel.

The harness emits `DESIGN_ONLY_PADDING_POLICY_PROOF` and does not claim runtime
default padding is implemented.

## Metadata Conformance Proof

NA-0291 keeps the existing `scripts/ci/metadata_conformance_smoke.sh` as the
baseline metadata conformance smoke and runs the new harness alongside it during
validation. The new stable markers are emitted only after all harness checks
pass:

- `NA0291_IDENTIFIER_POLICY_OK`
- `NA0291_ROTATION_POLICY_OK`
- `NA0291_PADDING_POLICY_OK`
- `NA0291_METADATA_PHASE2_HARNESS_OK`

## Negative Tests

Negative coverage includes:

- invalid identifier inputs;
- malformed handle input;
- invalid padding bucket configurations;
- invalid padding metadata;
- zero-length and oversized payload lengths;
- no mutation of accepted identifier state after rejected metadata input.

## Leak / Panic Checks

The harness carries a secret sentinel in the vector file and checks that emitted
markers do not include it. It also checks that public padding metadata omits
plaintext fixture text. Validation scans cover added lines and harness output
for secret/panic/backtrace indicators.

## Boundedness Proof

The harness is bounded by:

- `max_fixture_count` in the vector file;
- maximum identifier length `64`;
- maximum payload length `4096`;
- timeout wrapper defaulting to `120` seconds;
- finite fixture lists and finite `1..=4096` overhead sweep.

No unbounded loop or network dependency is introduced.

## Crypto / Protocol Boundary Proof

NA-0291 does not change:

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
- no claim that metadata phase-2 is complete.

## Remaining Gaps

- Runtime identifier rotation is not implemented.
- Runtime default padding is not implemented.
- Retention/purge metadata policy remains future work.
- Broader sanitized-error coverage remains future work.
- Timing, batching, jitter, cover traffic, contact-graph hiding, IP/location
  hiding, and deployment metadata remain future-gated.
- External review completion remains not ready.

## Next Recommended Lane

After closeout, restore NA-0292 for metadata phase-2 sanitized errors and
retention/purge design. That lane should remain docs/design and claim-boundary
work unless separately authorized to add executable proof.
