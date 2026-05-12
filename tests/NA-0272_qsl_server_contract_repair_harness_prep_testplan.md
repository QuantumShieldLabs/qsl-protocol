Goals: G1, G3, G4, G5

# NA-0272 qsl-server Contract Repair and Harness Prep Testplan

## Objective

Verify that NA-0272 records the merged qsl-server docs/API contract repair and
prepares the first executable qsl-server hardening harness without changing
qsl-protocol runtime behavior, qsl-server implementation semantics,
qsl-attachments implementation, protocol/wire/crypto behavior, workflows,
scripts, Cargo files, branch protection, or public-safety configuration.

## Protected Invariants

- Exactly one READY item remains during the evidence PR: NA-0272.
- D-0514 exists once after the evidence patch.
- D-0515 remains absent in the evidence PR.
- qsl-server PR #47 is merged before qsl-protocol evidence merge.
- qsl-server implementation paths remain untouched by NA-0272.
- qsl-protocol implementation paths remain untouched.
- qsl-attachments implementation paths remain untouched.
- No protocol, wire, crypto, auth, state-machine, qsp protocol-core, qsc/qsl
  runtime, qsc-desktop, website, workflow, script, Cargo, dependency,
  branch-protection, or public-safety configuration change is introduced.
- No production readiness or deployment readiness claim is introduced.
- Proven bugs, current behavior, future semantic decisions, and
  recommendations remain separated.

## qsl-server Docs/API Repair Proof

The evidence must record:

- qsl-server PR number, head SHA, merge SHA, and changed docs;
- proof that only qsl-server docs/testplan-style files changed;
- queue-full docs aligned to `ERR_OVERLOADED`;
- pull response docs aligned to JSON `items`;
- legacy path-token route docs aligned to retired 404/no-mutation behavior;
- deployment/auth docs aligned to optional `RELAY_TOKEN` relay auth plus
  route-token header behavior;
- `x-msg-id` wording corrected so current behavior is not called idempotency;
- invalid `MAX_BODY_BYTES` / `MAX_QUEUE_DEPTH` fallback/capping recorded as
  current behavior with fail-closed startup marked future.

## qsl-server PR Proof

The qsl-server repair PR must prove:

- branch: `na-0272-docs-api-contract-repair`;
- commit: `NA-0272 repair qsl-server docs API contract`;
- PR title: `NA-0272: repair qsl-server docs API contract`;
- PR body contains `Goals: G1, G3, G4, G5`;
- required qsl-server checks pass normally;
- merge uses a merge commit and exact head-SHA match;
- no admin bypass, squash, rebase, direct push, workflow change, Cargo change,
  or implementation path change occurs.

## No Service Implementation Changes

The qsl-server diff must not include:

- `src/**`;
- Rust tests;
- `Cargo.toml`;
- `Cargo.lock`;
- `.github/**`;
- `scripts/**`;
- packaging/runtime code.

The qsl-protocol diff must not include:

- qsl-server implementation paths;
- qsl-attachments implementation paths;
- qsp, qsc, qsl, qsl-client, apps, tools, inputs, formal, qsc-desktop,
  website, scripts, workflows, Cargo files, branch-protection configuration, or
  public-safety configuration.

## Harness Prep Requirements

The NA-0272 evidence must prepare future executable qsl-server coverage for:

- auth/reject/no-mutation tests;
- route-token tests;
- queue/overload tests;
- pull JSON response tests;
- legacy route 404/no-mutation tests;
- duplicate `x-msg-id` semantics tests;
- logging/no-secret tests;
- config/startup tests.

Future harness scope must be clearly marked as not implemented by NA-0272.

## Overclaim Scan

Scan added/changed docs for:

- `production-ready`
- `deployment-ready`
- `production relay ready`
- `qsl-server production ready`
- `production attachment ready`
- `external review complete`
- `metadata-free`
- `anonymity`
- `untraceable`
- `quantum-proof`
- `proven true Triple Ratchet`

Allowed uses must be explicitly negated, listed as future/unproven, or placed
inside prohibited wording sections. No heading may turn these phrases into an
affirmative claim.

## Link/Leak/Goal-Lint Expectations

- Relative markdown links must resolve.
- Added-line leak scan must not report secrets, auth header values, route
  tokens, secret-bearing URLs, payloads, or long secret-like hex dumps.
- Goal metadata must include G1, G3, G4, and G5 where required.
- PR body must include the exact standalone `Goals: G1, G3, G4, G5` line near
  the top.

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

After qsl-server PR #47 and the qsl-protocol NA-0272 evidence PR merge, the
expected successor is NA-0273: qsl-server executable auth/reject/logging
harness work. NA-0272 does not implement NA-0273.
