Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-15
Replaces: n/a
Superseded-By: n/a

# NA-0290 Metadata Phase-2 Identifier Rotation and Padding Defaults Design Test Plan

## Objective

Validate that NA-0290 produces design and governance evidence for metadata
phase-2 identifier rotation / opaque handle policy and padding-default policy
without implementing behavior or upgrading public claims.

## Protected Invariants

- Metadata phase-2 remains incomplete.
- Identifier rotation is not claimed implemented.
- Padding defaults are not claimed implemented.
- No protocol, wire, crypto, auth, negotiation, or state-machine behavior
  changes.
- No qsp protocol-core, qsc/qsl runtime, qshield demo implementation,
  qsl-server implementation, qsl-attachments implementation, qsc-desktop,
  website, README, START_HERE, workflow, script, Cargo, lockfile, dependency,
  branch-protection, or public-safety changes.
- No anonymity claim.
- No metadata-free claim.
- No untraceable claim.
- No production-readiness or public-internet-readiness claim.
- No external-review-complete claim.

## Allowed / Forbidden Scope

Allowed paths:

- `docs/governance/evidence/NA-0290_metadata_phase2_identifier_padding_design.md`
- `tests/NA-0290_metadata_phase2_identifier_padding_design_testplan.md`
- `docs/public/RELEASE_READINESS_EVIDENCE_MAP.md`
- `docs/public/EXTERNAL_REVIEW_PACKAGE.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Forbidden paths include:

- `README.md`
- `START_HERE.md`
- `.github/**`
- `scripts/**`
- `Cargo.toml`
- `Cargo.lock`
- `qsp/**`
- `qsc/**`
- `qsl/**`
- `qsl-client/**`
- `apps/**`
- `tools/**`
- `inputs/**`
- `formal/**`
- `qsc-desktop/**`
- `qsl-server/**`
- `qsl-attachments/**`
- `website/**`
- any runtime, protocol, crypto, demo, service, branch-protection,
  public-safety, dependency, or external website path.

## Baseline Inventory Requirements

The design evidence must classify current surfaces using:

- `PROVEN_EXECUTABLE`
- `DOCS_ONLY`
- `NOT_READY`
- `FUTURE_GATE`
- `OUT_OF_SCOPE`

The inventory must cover:

- existing executable identifier/handle evidence;
- existing executable padding evidence;
- current demo/metadata smoke evidence;
- current metadata conformance evidence;
- current qsl-server route-token and qsl-attachments capability evidence that
  is related but not sufficient for metadata phase-2 completion;
- current threat-model and design language;
- prohibited claims;
- what must be designed before implementation;
- what future tests should prove.

## Identifier Rotation Design Requirements

The design must answer:

- what identifiers and handles exist today;
- which identifiers are stable today;
- which identifiers should rotate in a future phase-2 lane;
- which rotation triggers should be considered;
- what should never rotate silently;
- what must remain auditable for deterministic tests;
- how rotation interacts with replay protection, no-mutation rejects, demo
  reproducibility, service route tokens, attachment capabilities, and external
  review evidence;
- what executable tests should prove rotation semantics later.

Required claim boundary:

- rotation must not be described as implemented by NA-0290;
- opaque handles must not be confused with auth tokens or attachment
  capabilities;
- long-term peer identity/trust material must not rotate silently.

## Padding Defaults Design Requirements

The design must answer:

- what padding behavior exists now;
- what padding is optional versus default;
- what default padding policy is safe to propose for a later harness without
  overclaiming;
- what lengths and buckets should be considered;
- what must be deterministic for tests;
- what remains future-gated;
- how padding interacts with metadata conformance smoke, demo smoke, external
  review package wording, performance/size overhead, and public claims;
- what executable tests should prove padding defaults later.

Required claim boundary:

- padding defaults must not be described as implemented by NA-0290;
- padding must not be presented as anonymity, metadata-free messaging,
  untraceability, or traffic-analysis resistance.

## Future Executable Harness Requirements

The design must recommend a successor executable lane that proves:

- rotating opaque delivery handles or a prerequisite stop;
- stale/wrong/malformed handle rejects;
- replay protection across handle rotation;
- no mutation on handle rejects;
- default padding inside a named phase-2 profile;
- malformed padding metadata rejects;
- receive-side padding mismatch rejects;
- no secret leakage in reject bodies or evidence output;
- existing metadata/demo/QSE baseline tests remain green.

## Claim-Boundary Requirements

The design and public references must preserve:

- `NOT_READY` status for metadata phase-2 completion;
- `NOT_READY` status for anonymity, metadata-free messaging, and
  untraceability;
- `NOT_READY` status for production readiness and public internet service
  readiness;
- `NOT_READY` status for external review completion.

## No Implementation Proof

Validation must prove no implementation paths changed.

Required checks:

```bash
git diff --name-only origin/main...HEAD
python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main \
  --allow docs/governance/evidence/NA-0290_metadata_phase2_identifier_padding_design.md \
  --allow tests/NA-0290_metadata_phase2_identifier_padding_design_testplan.md \
  --allow docs/public/RELEASE_READINESS_EVIDENCE_MAP.md \
  --allow docs/public/EXTERNAL_REVIEW_PACKAGE.md \
  --allow DECISIONS.md \
  --allow TRACEABILITY.md \
  --allow docs/ops/ROLLING_OPERATIONS_JOURNAL.md
```

The diff must contain only allowed docs/governance paths.

## Link / Leak / Goal-Lint Expectations

Required checks:

```bash
python3 scripts/ci/qsl_evidence_helper.py link-check
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
tools/goal_lint.py
```

Goal-lint may be run with a synthetic pull-request event payload if local
execution requires `GITHUB_EVENT_PATH`.

## CI Expectations

- Required checks attach and pass normally.
- `public-safety` remains required and green.
- Docs/governance-only cost control may skip heavy qsc suites according to
  NA-0262A, but the protected `public-safety` context must complete
  successfully.
- `cargo audit --deny warnings` passes.
- `cargo tree -i rustls-webpki --locked` continues to show
  `rustls-webpki v0.103.13`.
- `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1`
  passes.
- `python3 formal/run_model_checks.py` passes if present.

## Successor Handoff

After the NA-0290 design PR merges and post-merge public-safety is green, a
separate closeout may mark NA-0290 DONE and restore exactly one successor:

`NA-0291 - Metadata Phase-2 Identifier Rotation and Padding Defaults Executable Harness`

NA-0291 must not be implemented by this test plan.
