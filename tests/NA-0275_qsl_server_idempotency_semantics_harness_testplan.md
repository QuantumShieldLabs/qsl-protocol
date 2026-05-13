Goals: G1, G3, G4, G5

# NA-0275 qsl-server x-msg-id / Idempotency Semantics Harness Testplan

## Objective

Verify that NA-0275 records the merged qsl-server executable `x-msg-id` /
duplicate-message semantics harness without changing qsl-protocol runtime
behavior, qsl-attachments implementation, protocol/wire/crypto behavior,
workflows, scripts, Cargo files, branch protection, or public-safety
configuration.

## Protected Invariants

- Exactly one READY item remains during the evidence PR: NA-0275.
- D-0520 exists once after the evidence patch.
- D-0521 remains absent in the evidence PR.
- qsl-server harness PR #50 is merged before qsl-protocol evidence merge.
- qsl-server harness scope is tests plus docs clarification only unless
  evidence explicitly records a minimal implementation fix.
- qsl-server dependency files remain unchanged in the harness PR.
- qsl-server workflows remain unchanged.
- qsl-protocol implementation paths remain untouched.
- qsl-attachments implementation paths remain untouched.
- No protocol, wire, crypto, auth, state-machine, qsp protocol-core, qsc/qsl
  runtime, qsc-desktop, website, workflow, script, Cargo, dependency,
  branch-protection, or public-safety configuration change is introduced in
  qsl-protocol.
- No production readiness or deployment readiness claim is introduced.
- Proven bugs, current behavior, future semantic decisions, and
  recommendations remain separated.

## qsl-server Executable Harness Proof

The evidence must record:

- qsl-server harness PR number, head SHA, merge SHA, changed paths, and check
  result;
- chosen `x-msg-id` semantics;
- whether implementation code changed;
- proof that the harness PR did not change dependencies or workflows;
- focused harness test results;
- full qsl-server test result;
- qsl-server audit result;
- recovered local validation failures, if any.

## Chosen Semantics

NA-0275 records the current behavior:

- `x-msg-id` is a client-supplied message identifier.
- `x-msg-id` is not an idempotency key.
- Duplicate supplied IDs are accepted as separate queued messages when each
  push otherwise passes validation.
- Pull returns duplicate-ID items in FIFO order and deletes them on delivery.
- Future idempotency or deduplication remains a separate semantic hardening
  decision.

## Required Harness Coverage

The qsl-server harness must cover:

- duplicate `x-msg-id` pushes;
- supplied and auto-generated ID coexistence;
- duplicate supplied ID ordering and payload integrity;
- auth-rejected duplicate attempts do not mutate queues;
- oversize duplicate attempts do not mutate queues;
- queue-cap duplicate attempts do not mutate queues;
- pull/delete behavior for duplicate-ID messages;
- repeated pull after deletion;
- `x-msg-id` logging boundary as non-secret metadata;
- route-token, auth-header, and payload sentinels absent from captured logs;
- blank or malformed `x-msg-id` handling without panic;
- existing `tests/hardening_auth_reject_logging.rs` remains green.

## Allowed / Forbidden qsl-server Scope

Allowed qsl-server harness scope:

- `tests/**/*.rs`;
- `tests/*.rs`;
- `README.md` only for harness note or `x-msg-id` semantic clarification;
- `docs/**` only for harness note or `x-msg-id` semantic clarification;
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

- `docs/governance/evidence/NA-0275_qsl_server_idempotency_semantics_harness.md`;
- `tests/NA-0275_qsl_server_idempotency_semantics_harness_testplan.md`;
- `docs/public/QSL_SERVER_ATTACHMENTS_PRODUCTION_BOUNDARY_PLAN.md`;
- `DECISIONS.md`;
- `TRACEABILITY.md`;
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`.

Forbidden qsl-protocol paths include runtime, protocol, crypto, qsl-server,
qsl-attachments, qsc-desktop, website, workflow, script, Cargo, dependency,
branch-protection, and public-safety configuration paths.

## No Production-Readiness Claim

NA-0275 may claim only local executable qsl-server duplicate-message semantics
evidence. It must not claim qsl-server public operation, public internet
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
is docs, governance, evidence, public-boundary handoff, and testplan only, so
docs-only cost control may skip full suites where policy allows.

## Successor Handoff

After qsl-server PR #50 and the qsl-protocol NA-0275 evidence PR merge,
NA-0275 may close out only under a separate closeout packet that promotes
exactly one READY successor. The recommended successor is NA-0276: qsl-server
invalid config fail-closed semantics harness.
