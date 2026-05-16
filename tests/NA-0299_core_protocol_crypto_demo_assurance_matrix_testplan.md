Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-16
Replaces: n/a
Superseded-By: n/a

# NA-0299 Core Protocol Crypto Demo Assurance Matrix Testplan

## Objective

Validate that NA-0299 produces a truthful core protocol, crypto, formal, vector, demo, metadata, service-boundary, dependency, and external-review assurance matrix and selects one exact executable successor without changing implementation behavior.

## Protected Invariants

- Exactly one queue item remains READY during NA-0299: NA-0299.
- No protocol-core implementation change.
- No crypto state-machine change.
- No QSP wire-format, handshake, key schedule, ratchet, replay, downgrade, or reject-semantics change.
- No service, demo runtime, qsc-desktop, website, or external website mutation.
- No workflow, script, Cargo, dependency, branch-protection, or public-safety configuration change.
- No production-readiness, public-internet-readiness, external-review-complete, anonymity, metadata-free, untraceable, quantum-proof, unbreakable, guaranteed-secure, or equivalent overclaim.

## Allowed Scope

- `docs/governance/evidence/NA-0299_core_protocol_crypto_demo_assurance_matrix.md`
- `tests/NA-0299_core_protocol_crypto_demo_assurance_matrix_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `docs/public/RELEASE_READINESS_EVIDENCE_MAP.md` only if safe evidence reference update is required
- `docs/public/EXTERNAL_REVIEW_PACKAGE.md` only if safe evidence reference update is required

## Forbidden Scope

- `README.md`
- `START_HERE.md`
- `docs/public/INDEX.md`
- `website/**`
- `.github/**`
- `scripts/**`
- `Cargo.toml`
- `Cargo.lock`
- protocol, crypto, runtime, demo, formal, input, refimpl, app, service, desktop, qsl-server, qsl-attachments, and external website implementation paths
- branch deletion or branch-protection/public-safety mutation

## Required Checks

```bash
git status --porcelain=v1 --branch
git diff --name-only origin/main...HEAD
git diff --stat origin/main...HEAD
git diff --check
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main --allow docs/governance/evidence/NA-0299_core_protocol_crypto_demo_assurance_matrix.md --allow tests/NA-0299_core_protocol_crypto_demo_assurance_matrix_testplan.md --allow DECISIONS.md --allow TRACEABILITY.md --allow docs/ops/ROLLING_OPERATIONS_JOURNAL.md
python3 scripts/ci/qsl_evidence_helper.py link-check
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
cargo audit --deny warnings
cargo tree -i rustls-webpki --locked
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
python3 formal/run_model_checks.py
bash scripts/ci/classify_ci_scope.sh docs/governance/evidence/NA-0299_core_protocol_crypto_demo_assurance_matrix.md tests/NA-0299_core_protocol_crypto_demo_assurance_matrix_testplan.md DECISIONS.md TRACEABILITY.md docs/ops/ROLLING_OPERATIONS_JOURNAL.md
```

Run a direct overclaim phrase scan over changed lines. Any match must be explicitly negated, marked prohibited, marked `NOT_READY`, marked `PARTIAL`, or used only as an example of what must not be claimed.

## Optional Checks

Run if bounded and available:

```bash
scripts/ci/metadata_conformance_smoke.sh
scripts/ci/metadata_phase2_identifier_padding_harness.sh
scripts/ci/metadata_phase2_sanitized_errors_retention_harness.sh
scripts/ci/demo_cli_smoke.sh
DEMO_STRESS_PROFILE=baseline scripts/ci/demo_adversarial_stress.sh
DEMO_SOAK_RUNS=3 scripts/ci/demo_soak_repeated_run.sh
cargo +stable build -p qshield-cli --locked
cargo +stable test -p qshield-cli --locked -- --test-threads=1
cargo +stable test -p quantumshield_refimpl --locked -- --test-threads=1
```

## Evidence Classification

The matrix must classify each domain as one or more of:

- `PROVEN_EXECUTABLE`
- `PARTIAL_EXECUTABLE`
- `DOCS_ONLY`
- `NOT_READY`
- `FUTURE_GATE`
- `OUT_OF_SCOPE`

## Next-Lane Selection Criteria

The successor must be exact, executable, and gap-driven. Prefer a lane that adds or consolidates proof rather than more planning. The selected lane must state whether protocol/crypto implementation change is allowed, and the default is NO unless the future directive explicitly authorizes it.

## Claim-Boundary Requirements

The evidence must keep visible:

- production readiness is not established;
- public-internet readiness is not established;
- external review is not complete;
- metadata fixture proof does not establish anonymity, metadata-free messaging, or untraceability;
- local demo evidence is not deployment proof;
- formal/model checks are bounded and do not constitute a complete proof.

## Link / Leak / Overclaim Expectations

`link-check` must report zero missing links. `leak-scan --mode added --base origin/main` must report zero secret findings. Overclaim scan must report no unsafe affirmative matches.

## CI Expectations

Required PR checks must complete normally before merge. Docs-only cost control may skip heavy suites when the classifier marks the changed path set as docs-only, but public-safety must remain required and green.

## Successor Handoff

NA-0299 should recommend:

**NA-0300 - Core Protocol Replay / Reject / No-Mutation Adversarial Harness**

NA-0299 closeout, if separately executed, must restore exactly one READY successor and must not implement NA-0300.
