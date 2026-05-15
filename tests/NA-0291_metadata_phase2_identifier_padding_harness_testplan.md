Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-15
Replaces: n/a
Superseded-By: n/a

# NA-0291 Metadata Phase-2 Identifier Padding Harness Test Plan

## Objective

Validate the first executable metadata phase-2 harness for identifier /
opaque-handle policy and padding-default policy without changing runtime
protocol, crypto, service, dependency, or public-copy behavior.

## Protected Invariants

- Metadata phase-2 remains evidence-bound and incomplete.
- Runtime identifier rotation is not claimed implemented.
- Runtime default padding is not claimed implemented.
- No qsp protocol-core change.
- No cryptographic state-machine change.
- No handshake, key schedule, QSP wire-format, or downgrade behavior change.
- No qsl-server, qsl-attachments, qsc-desktop, website, README, START_HERE,
  workflow, Cargo, dependency, branch-protection, or public-safety config
  change.
- No anonymity, metadata-free, untraceable, external-review-complete,
  production-readiness, or public-internet-readiness claim.

## Allowed / Forbidden Scope

Allowed paths:

- `inputs/metadata_phase2/identifier_padding_policy_vectors_v1.json`
- `scripts/ci/metadata_phase2_identifier_padding_harness.sh`
- `docs/governance/evidence/NA-0291_metadata_phase2_identifier_padding_harness.md`
- `tests/NA-0291_metadata_phase2_identifier_padding_harness_testplan.md`
- `docs/public/RELEASE_READINESS_EVIDENCE_MAP.md`
- `docs/public/EXTERNAL_REVIEW_PACKAGE.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Forbidden paths include qsp protocol-core, cryptographic state-machine code,
handshake/key schedule code, qsl-server implementation, qsl-attachments
implementation, qsc-desktop, website, README, START_HERE, workflows, Cargo
manifests/lockfiles, dependency files, branch-protection configuration, and
public-safety configuration.

## Executable Harness Requirements

Run:

```bash
scripts/ci/metadata_phase2_identifier_padding_harness.sh
```

The harness must emit these markers only after all checks pass:

- `NA0291_IDENTIFIER_POLICY_OK`
- `NA0291_ROTATION_POLICY_OK`
- `NA0291_PADDING_POLICY_OK`
- `NA0291_METADATA_PHASE2_HARNESS_OK`

It must also emit:

- `DESIGN_ONLY_ROTATION_POLICY_PROOF`
- `DESIGN_ONLY_PADDING_POLICY_PROOF`

## Identifier / Handle Requirements

The harness must prove:

- deterministic handle derivation for stable fixture input;
- no unexpected collision in the bounded fixture set;
- session, epoch, route, contact, and attachment-operation boundaries produce
  distinguishable handles;
- malformed, empty, ambiguous, and oversized identifier inputs reject;
- malformed handle text rejects;
- rejected identifier inputs do not mutate accepted-state snapshots.

## Rotation-Policy Requirements

The harness is allowed to prove policy fixtures only. It must not claim runtime
rotation. The design-only marker is required unless a later lane implements
runtime rotation under separately authorized scope.

## Padding-Policy Requirements

The harness must prove:

- deterministic bucket table for the named phase-2 demo profile;
- valid lengths map to expected buckets;
- invalid bucket configs reject;
- maximum overhead remains bounded;
- strip/verify restores valid padded payloads;
- invalid padding metadata rejects;
- padding metadata does not leak plaintext or sentinel material.

The design-only marker is required unless a later lane implements runtime
default padding under separately authorized scope.

## Metadata Conformance Requirements

The existing metadata baseline remains:

```bash
scripts/ci/metadata_conformance_smoke.sh
```

NA-0291 validation must run the new harness and the existing metadata
conformance smoke alongside each other.

## Negative Tests

Negative cases must cover:

- invalid identifier input;
- malformed handle input;
- invalid padding config;
- invalid padding metadata;
- zero-length and oversized payload input;
- malformed fixture rejection;
- no-mutation behavior for rejected identifier input.

## Leak / Panic Requirements

Validation must scan:

- added lines for high-confidence secrets;
- harness output for the fixture sentinel;
- harness output for panic/backtrace markers.

## Crypto Boundary Requirements

Validation must prove no changed path under qsp protocol-core, crypto
state-machine, handshake/key schedule, QSP wire-format, service implementation,
website, workflow, Cargo, dependency, branch-protection, or public-safety
configuration paths.

## Claim Boundary Requirements

Validation must scan for overclaims and preserve conservative wording for:

- production readiness;
- public internet readiness;
- external review completion;
- anonymity;
- metadata-free messaging;
- untraceability;
- quantum-proof status;
- proven true Triple Ratchet.

## CI Expectations

- `public-safety` remains required and green.
- New script/vector/test changes require full public-safety behavior, not a
  docs-only skip.
- `cargo audit --deny warnings` passes.
- `cargo tree -i rustls-webpki --locked` shows `rustls-webpki v0.103.13`.
- `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1`
  passes.
- `python3 formal/run_model_checks.py` passes.
- `scripts/ci/metadata_conformance_smoke.sh` passes.
- `scripts/ci/metadata_phase2_identifier_padding_harness.sh` passes.

## Successor Handoff

After the NA-0291 harness PR merges and post-merge public-safety is green,
closeout may mark NA-0291 DONE and restore NA-0292 as the sole READY successor
for metadata phase-2 sanitized errors and retention/purge design. NA-0292 must
not be implemented inside the NA-0291 harness PR.
