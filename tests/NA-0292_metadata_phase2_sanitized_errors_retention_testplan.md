Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-15
Replaces: n/a
Superseded-By: n/a

# NA-0292 Metadata Phase-2 Sanitized Errors and Retention/Purge Design Test Plan

## Objective

Validate that NA-0292 produces design and governance evidence for metadata
phase-2 sanitized-error expansion and retention/purge metadata policy without
implementing runtime behavior or upgrading public claims.

## Protected Invariants

- Metadata phase-2 remains evidence-bound and incomplete.
- Sanitized-error expansion is not claimed implemented.
- Retention/purge metadata policy is not claimed implemented.
- No protocol, wire, crypto, auth, negotiation, or state-machine behavior
  changes.
- No qsp protocol-core, qsc/qsl runtime, qshield demo implementation,
  qsl-server implementation, qsl-attachments implementation, qsc-desktop,
  website, README, START_HERE, workflow, script, Cargo, lockfile, dependency,
  branch-protection, or public-safety configuration changes.
- No anonymity claim.
- No metadata-free claim.
- No untraceable claim.
- No production-readiness or public-internet-readiness claim.
- No external-review-complete claim.

## Allowed / Forbidden Scope

Allowed paths:

- `docs/governance/evidence/NA-0292_metadata_phase2_sanitized_errors_retention_design.md`
- `tests/NA-0292_metadata_phase2_sanitized_errors_retention_testplan.md`
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

The sanitized-error inventory must cover:

- existing executable sanitized-error evidence;
- existing executable reject/no-mutation evidence;
- existing service reason-code evidence that is related but service-scoped;
- existing demo/metadata sanitized-error evidence;
- current error bodies, logs, and artifacts that are safe;
- current error bodies, logs, and artifacts that are unknown or risky;
- current threat-model and design language;
- prohibited claims;
- what future tests should prove.

The retention/purge inventory must cover:

- existing qsl-protocol retention/purge metadata evidence;
- qsl-server route lifecycle, TTL, and retention evidence and limits;
- qsl-attachments retention, cleanup, recovery, and backup/restore evidence and
  limits;
- demo retention/purge evidence;
- what retention/purge metadata is not addressed;
- what remains future-gated;
- what future tests should prove.

## Sanitized-Error Design Requirements

The design must answer:

- what sanitized errors should protect;
- which values are allowed in error fields;
- which values are forbidden in error fields;
- how error categories map across demo CLI, qshield public demo, metadata
  conformance smoke, qsl-server, qsl-attachments, and future metadata phase-2
  harnesses;
- what negative tests should prove;
- what future executable harness should prove;
- what remains out of scope;
- what public wording is safe and prohibited.

Required protected values include:

- route tokens;
- capabilities;
- identifiers and handles;
- contact and peer metadata;
- descriptors;
- plaintext and ciphertext sentinels;
- internal state;
- queue/session/object existence;
- timing and retry hints.

## Retention/Purge Design Requirements

The design must answer:

- what retention/purge metadata surfaces exist today;
- what belongs to qsl-protocol metadata phase-2 versus service production
  gates;
- what future qsl-protocol tests should prove;
- what remains qsl-server or qsl-attachments service-scoped;
- what policy public docs should use;
- what future executable harness requirements apply;
- what remains out of scope.

Required policy separation:

- protocol metadata phase-2 fixtures;
- non-production demo behavior;
- qsl-server service-local route/queue TTL behavior;
- qsl-attachments service-local session/object/backup behavior;
- retained governance/evidence artifacts.

## Cross-Surface Risk Matrix Requirements

The design must include a risk matrix for:

- error body leakage;
- log leakage;
- reason-code over-specificity;
- route/contact/identifier leakage;
- capability/token leakage;
- timing/order leakage;
- retention duration leakage;
- purge/deletion state leakage;
- backup/restore metadata leakage;
- rejected-state leakage;
- panic/backtrace leakage;
- public claim overreach.

For each category the design must record current evidence, remaining gap,
recommended design control, future executable test, and claim-boundary note.

## Future Executable Harness Requirements

The design must recommend NA-0293 as an executable successor that proves, or
stops with exact prerequisites:

- sanitized error fixtures for malformed metadata, invalid identifier, invalid
  padding, unauthorized route/capability, expired/deleted/purged state, and
  panic/backtrace absence;
- no secret/token/capability/identifier/descriptor/plaintext/ciphertext leakage;
- retention/purge fixtures for deleted/expired/purged state, no resurrection,
  no over-specific metadata in errors, no secret logs, and deterministic
  retention-window behavior;
- cross-surface preservation of metadata conformance smoke and NA-0291
  identifier/padding harness proof;
- markers:
  - `NA0293_SANITIZED_ERROR_POLICY_OK`
  - `NA0293_RETENTION_PURGE_POLICY_OK`
  - `NA0293_METADATA_PHASE2_SANITIZED_RETENTION_OK`

## Claim-Boundary Requirements

The design and public references must preserve:

- `NOT_READY` status for metadata phase-2 completion;
- `NOT_READY` status for anonymity, metadata-free messaging, and
  untraceability;
- `NOT_READY` status for production readiness and public internet service
  readiness;
- `NOT_READY` status for external review completion;
- no claim that sanitized-error policy is implemented;
- no claim that retention/purge policy is implemented.

## No Implementation Proof

Validation must prove no implementation paths changed.

Required checks:

```bash
git diff --name-only origin/main...HEAD
python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main \
  --allow docs/governance/evidence/NA-0292_metadata_phase2_sanitized_errors_retention_design.md \
  --allow tests/NA-0292_metadata_phase2_sanitized_errors_retention_testplan.md \
  --allow docs/public/RELEASE_READINESS_EVIDENCE_MAP.md \
  --allow docs/public/EXTERNAL_REVIEW_PACKAGE.md \
  --allow DECISIONS.md \
  --allow TRACEABILITY.md \
  --allow docs/ops/ROLLING_OPERATIONS_JOURNAL.md
```

The diff must contain only allowed docs/governance/public/testplan paths.

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

After the NA-0292 design PR merges and post-merge public-safety is green, a
separate closeout may mark NA-0292 DONE and restore exactly one successor:

`NA-0293 - Metadata Phase-2 Sanitized Errors and Retention/Purge Executable Harness`

NA-0293 must not be implemented by this test plan.
