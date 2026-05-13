Goals: G1, G3, G4, G5

# NA-0274 qsl-attachments Reject-Taxonomy Harness Testplan

## Objective

Verify that NA-0274 records the merged qsl-attachments executable malformed
JSON / reject-taxonomy harness without changing qsl-protocol runtime behavior,
qsl-server implementation, protocol/wire/crypto behavior, workflows, scripts,
Cargo files, branch protection, or public-safety configuration.

## Protected Invariants

- Exactly one READY item remains during the evidence PR: NA-0274.
- D-0518 exists once after the evidence patch.
- D-0519 remains absent in the evidence PR.
- qsl-attachments harness PR #32 is merged before qsl-protocol evidence merge.
- qsl-attachments dependency remediation is not used unless audit fails.
- qsl-attachments implementation changes are limited to minimal test-backed
  reject-taxonomy/capability/logging fixes.
- qsl-attachments dependency files remain unchanged in the harness PR.
- qsl-attachments workflows remain unchanged.
- qsl-protocol implementation paths remain untouched.
- No production readiness or deployment readiness claim is introduced.
- Opaque ciphertext boundaries remain explicit.
- Proven bugs, current behavior, future semantic decisions, and
  recommendations remain separated.

## qsl-attachments Dependency Remediation Proof

The evidence must record:

- initial qsl-attachments `cargo audit --deny warnings` result;
- initial qsl-attachments `cargo test --locked` result;
- whether remediation was used;
- if remediation was not used, explicit statement that no dependency PR,
  `Cargo.toml`, or `Cargo.lock` change was needed.

## qsl-attachments Executable Harness Proof

The evidence must record:

- qsl-attachments harness PR number, head SHA, merge SHA, changed paths, and
  check result;
- whether implementation code changed;
- proof that no dependency or workflow files changed;
- focused harness test result;
- full qsl-attachments test result;
- qsl-attachments audit result;
- recovered local validation failures, if any.

## Required Harness Coverage

The qsl-attachments harness must cover:

- malformed JSON / Axum extractor rejects;
- canonical `reason_code` behavior;
- missing capability rejects;
- wrong capability rejects;
- wrong resource / capability mismatch rejects;
- rejected requests do not persist sessions, parts, or objects unexpectedly;
- opaque ciphertext bytes round trip without plaintext parsing;
- capability/token values absent from logs;
- descriptor sentinels absent from logs;
- ciphertext/plaintext sentinels absent from logs;
- cleanup/expiry baseline where helper behavior exists;
- malformed/capability reject paths do not panic.

## Allowed / Forbidden qsl-attachments Scope

Allowed qsl-attachments harness scope:

- `tests/**/*.rs`;
- `tests/*.rs`;
- implementation files only if a newly added test proves a bounded bug and the
  fix is minimal and test-backed.

Forbidden qsl-attachments harness scope:

- `Cargo.toml`;
- `Cargo.lock`;
- `.github/**`;
- `scripts/**`;
- deployment files;
- packaging files;
- new dependencies.

## qsl-protocol Evidence Scope

Allowed qsl-protocol paths:

- `docs/governance/evidence/NA-0274_qsl_attachments_reject_taxonomy_harness.md`;
- `tests/NA-0274_qsl_attachments_reject_taxonomy_harness_testplan.md`;
- `docs/public/QSL_SERVER_ATTACHMENTS_PRODUCTION_BOUNDARY_PLAN.md`;
- `DECISIONS.md`;
- `TRACEABILITY.md`;
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`.

Forbidden qsl-protocol paths include runtime, protocol, crypto, qsl-server,
qsl-attachments, qsc-desktop, website, workflow, script, Cargo, dependency,
branch-protection, and public-safety configuration paths.

## No Production-Readiness Claim

NA-0274 may claim only local executable reject-taxonomy evidence. It must not
claim qsl-attachments public operation, public internet exposure safety,
production attachment service approval, external review completion, metadata
elimination, anonymity, untraceability, or completed production deployment
review.

## Opaque-Ciphertext Boundary

The evidence must state that qsl-attachments stores/fetches opaque bytes only
and does not decrypt client plaintext. Any plaintext sentinel used in tests is
only a logging/no-secret marker and must not be interpreted as service
plaintext handling.

## Link / Leak / Goal-Lint Expectations

- Relative markdown links must resolve.
- Added-line leak scan must not report secrets, auth header values,
  capability values, payload sentinels, sensitive endpoints, or long
  secret-like hex dumps.
- Goal metadata must include G1, G3, G4, and G5 where required.
- PR body must include the exact standalone `Goals: G1, G3, G4, G5` line near
  the top.
- Known gaps must remain visible.

## CI Expectations

Local validation should include:

- `git status --porcelain=v1 --branch`
- `git diff --name-only origin/main...HEAD`
- `git diff --stat origin/main...HEAD`
- `git diff --check`
- direct overclaim phrase scan
- `python3 scripts/ci/qsl_evidence_helper.py queue`
- `python3 scripts/ci/qsl_evidence_helper.py decisions`
- `python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main`
- `python3 scripts/ci/qsl_evidence_helper.py link-check`
- `python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main`
- `cargo audit --deny warnings`
- `cargo tree -i rustls-webpki --locked`
- `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1`
- `python3 formal/run_model_checks.py`
- goal-lint or helper PR-body preflight with the exact Goals line.

Required CI must pass normally before merge. The expected qsl-protocol PR scope
is docs, governance, evidence, public-boundary handoff, and testplan only, so
docs-only cost control may skip full suites where policy allows.

## Successor Handoff

After qsl-attachments PR #32 and the qsl-protocol NA-0274 evidence PR merge,
NA-0274 may close out only under a separate closeout packet that promotes
exactly one READY successor. The recommended successor is NA-0275: qsl-server
`x-msg-id` / idempotency semantics decision and harness.
