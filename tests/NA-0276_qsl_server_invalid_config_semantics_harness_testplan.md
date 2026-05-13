Goals: G1, G3, G4, G5

# NA-0276 qsl-server Invalid Config Semantics Harness Testplan

## Objective

Verify that NA-0276 records the merged qsl-server executable invalid
configuration semantics harness without changing qsl-protocol runtime behavior,
qsl-attachments implementation, protocol/wire/crypto behavior, workflows,
scripts, Cargo files, branch protection, or public-safety configuration.

## Protected Invariants

- Exactly one READY item remains during the evidence PR: NA-0276.
- D-0522 exists once after the evidence patch.
- D-0523 remains absent in the evidence PR.
- qsl-server harness PR #51 is merged before qsl-protocol evidence merge.
- qsl-server invalid config behavior is deterministic.
- Malformed or zero `MAX_BODY_BYTES` / `MAX_QUEUE_DEPTH` does not silently
  create unsafe runtime settings.
- Above-ceiling capping, where retained, is explicit and test-backed.
- Existing qsl-server auth/reject/logging and x-msg-id harnesses remain green.
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
- chosen `MAX_BODY_BYTES` / `MAX_QUEUE_DEPTH` semantics;
- whether implementation code changed;
- proof that the harness PR did not change dependencies or workflows;
- focused config harness test result;
- existing auth/reject/logging and x-msg-id harness results;
- full qsl-server test result;
- qsl-server audit result;
- recovered local validation failures, if any.

## Chosen Semantics

NA-0276 records and test-backs:

- missing `MAX_BODY_BYTES` uses the default 1 MiB limit;
- non-numeric or zero `MAX_BODY_BYTES` fails startup;
- above-ceiling `MAX_BODY_BYTES` is capped to the built-in ceiling;
- missing `MAX_QUEUE_DEPTH` uses the default 256 limit;
- non-numeric or zero `MAX_QUEUE_DEPTH` fails startup;
- above-ceiling `MAX_QUEUE_DEPTH` is capped to the built-in ceiling;
- invalid `PORT` fails startup;
- invalid `BIND_ADDR` fails closed during bind parsing;
- missing `RELAY_TOKEN` disables bearer auth only when that mode is explicit;
- present `RELAY_TOKEN` requires bearer auth.

## Allowed / Forbidden qsl-server Scope

Allowed qsl-server harness scope:

- `tests/**/*.rs`;
- `tests/*.rs`;
- `README.md` only for config semantic clarification;
- `docs/**` only for config semantic clarification;
- `src/**` only for a minimal test-backed invalid-config behavior fix.

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

- `docs/governance/evidence/NA-0276_qsl_server_invalid_config_semantics_harness.md`;
- `tests/NA-0276_qsl_server_invalid_config_semantics_harness_testplan.md`;
- `docs/public/QSL_SERVER_ATTACHMENTS_PRODUCTION_BOUNDARY_PLAN.md`;
- `DECISIONS.md`;
- `TRACEABILITY.md`;
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`.

Forbidden qsl-protocol paths include runtime, protocol, crypto, qsl-server,
qsl-attachments, qsc-desktop, website, workflow, script, Cargo, dependency,
branch-protection, and public-safety configuration paths.

## No Production-Readiness Claim

NA-0276 may claim only local executable qsl-server invalid-configuration
semantics evidence. It must not claim qsl-server public operation, public
internet exposure safety, production relay approval, production attachment
service operation, external review completion, metadata elimination,
anonymity, untraceability, or completed production deployment review.

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

After qsl-server PR #51 and the qsl-protocol NA-0276 evidence PR merge,
NA-0276 may close out only under a separate closeout packet that promotes
exactly one READY successor. The recommended successor is NA-0277:
qsl-server abuse / rate-limit / queue-cap harness.
