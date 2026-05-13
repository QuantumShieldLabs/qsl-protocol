Goals: G1, G3, G4, G5

# NA-0273 qsl-server Auth/Reject/Logging Harness Testplan

## Objective

Verify that NA-0273 records the merged qsl-server dependency advisory
remediation and executable auth/reject/logging harness without changing
qsl-protocol runtime behavior, qsl-attachments implementation,
protocol/wire/crypto behavior, workflows, scripts, Cargo files, branch
protection, or public-safety configuration.

## Protected Invariants

- Exactly one READY item remains during the evidence PR: NA-0273.
- D-0516 exists once after the evidence patch.
- The next closeout decision ID remains absent in the evidence PR.
- qsl-server dependency remediation PR #48 is merged before qsl-protocol
  evidence merge.
- qsl-server harness PR #49 is merged before qsl-protocol evidence merge.
- qsl-server dependency remediation is limited to dependency advisory fixes.
- qsl-server harness is tests-only unless evidence explicitly records a
  minimal implementation fix.
- qsl-protocol implementation paths remain untouched.
- qsl-attachments implementation paths remain untouched.
- No protocol, wire, crypto, auth, state-machine, qsp protocol-core, qsc/qsl
  runtime, qsc-desktop, website, workflow, script, Cargo, dependency,
  branch-protection, or public-safety configuration change is introduced in
  qsl-protocol.
- No production readiness or deployment readiness claim is introduced.
- Proven bugs, current behavior, future semantic decisions, and
  recommendations remain separated.

## qsl-server Dependency Remediation Proof

The evidence must record:

- qsl-server PR number, head SHA, merge SHA, changed paths, and check result;
- initial `cargo audit --deny warnings` findings;
- patched versions for `bytes`, `quinn-proto`, `rustls-webpki`, and `rand`;
- whether `Cargo.toml` changed and why;
- whether `Cargo.lock` changed;
- proof that no qsl-server source, test, workflow, script, deployment, or
  packaging file changed in the dependency PR;
- `cargo audit --deny warnings` pass after remediation;
- `cargo test --locked` pass after remediation.

## qsl-server Executable Harness Proof

The evidence must record:

- qsl-server harness PR number, head SHA, merge SHA, changed paths, and check
  result;
- whether implementation code changed;
- proof that the harness PR did not change dependencies or workflows;
- focused harness test result;
- full qsl-server test result;
- qsl-server audit result after dependency remediation;
- recovered CI/test-shape failure evidence, if any.

## Required Harness Coverage

The qsl-server harness must cover:

- missing and wrong bearer auth rejects;
- rejected pushes do not mutate queues;
- full queue returns `429 ERR_OVERLOADED`;
- empty pull behavior;
- pull JSON `items` behavior;
- pull deletes delivered messages;
- retired legacy routes return 404 without mutating or consuming canonical
  queues;
- raw route tokens are absent from captured logs;
- Authorization/Bearer values are absent from captured logs;
- payload/sentinel values are absent from captured logs;
- invalid `PORT` fails startup;
- invalid size/depth environment behavior is captured as current behavior
  without overstating fail-closed semantics.

## Allowed / Forbidden qsl-server Scope

Allowed qsl-server dependency remediation scope:

- `Cargo.lock`;
- `Cargo.toml` only if strictly needed for patched versions.

Allowed qsl-server harness scope:

- `tests/**/*.rs`;
- `tests/*.rs`;
- implementation files only if a newly added test proves a bounded bug and the
  fix is minimal and test-backed.

Forbidden qsl-server harness scope:

- `Cargo.toml`;
- `Cargo.lock`;
- `.github/**`;
- `scripts/**`;
- deployment files;
- packaging files;
- new dependencies.

## qsl-protocol Evidence Scope

Allowed qsl-protocol paths:

- `docs/governance/evidence/NA-0273_qsl_server_auth_reject_logging_harness.md`;
- `tests/NA-0273_qsl_server_auth_reject_logging_harness_testplan.md`;
- `docs/public/QSL_SERVER_ATTACHMENTS_PRODUCTION_BOUNDARY_PLAN.md`;
- `DECISIONS.md`;
- `TRACEABILITY.md`;
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`.

Forbidden qsl-protocol paths include runtime, protocol, crypto, qsl-server,
qsl-attachments, qsc-desktop, website, workflow, script, Cargo, dependency,
branch-protection, and public-safety configuration paths.

## No Production-Readiness Claim

NA-0273 may claim only local executable hardening evidence and dependency
audit health. It must not claim qsl-server public operation, public internet
exposure safety, production relay approval, production attachment service
operation, external review completion, metadata elimination, anonymity,
untraceability, or completed production deployment review.

## Link / Leak / Goal-Lint Expectations

- Relative markdown links must resolve.
- Added-line leak scan must not report secrets, auth header values, route
  tokens, payload sentinels, sensitive endpoints, or long secret-like hex
  dumps.
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
is docs, governance, evidence, and testplan only, so docs-only cost control may
skip full suites where policy allows.

## Successor Handoff

After qsl-server PR #48, qsl-server PR #49, and the qsl-protocol NA-0273
evidence PR merge, NA-0273 may close out only under a separate closeout packet
that promotes exactly one READY successor. The recommended successor is
NA-0274: qsl-attachments malformed JSON / reject-taxonomy harness work.
