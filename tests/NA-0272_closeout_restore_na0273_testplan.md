Goals: G1, G3, G4, G5

# NA-0272 Closeout and NA-0273 Restoration Testplan

## Objective

Verify that NA-0272 is closed only after qsl-server docs/API repair, qsl-protocol
evidence/harness prep, and post-merge public-safety proof are complete, then
restore NA-0273 as the sole READY successor without implementing NA-0273.

## Protected Invariants

- Exactly one READY item exists after closeout: NA-0273.
- NA-0272 is DONE.
- D-0514 exists once.
- D-0515 exists once after the closeout patch.
- No duplicate decision IDs exist.
- qsl-server PR #47 evidence remains recorded.
- qsl-protocol PR #797 evidence remains recorded.
- public-safety remains required and green.
- No qsl-protocol runtime, qsl-server implementation, qsl-attachments
  implementation, protocol/wire/crypto, auth, state-machine, qsp protocol-core,
  qsc/qsl runtime, qsc-desktop, website, workflow, script, Cargo, dependency,
  branch-protection, or public-safety configuration change is introduced.
- No production readiness or deployment readiness claim is introduced.

## Required Queue State

Before closeout:

- READY_COUNT 1.
- READY NA-0272.
- D-0514 exists.
- D-0515 absent.
- qsl-server PR #47 merged.
- qsl-protocol PR #797 merged.
- post-merge public-safety success is recorded on the PR #797 merge.

After closeout:

- READY_COUNT 1.
- READY NA-0273.
- NA-0272 DONE.
- D-0515 exists once.

## NA-0273 Successor Scope

NA-0273 must be defined as executable qsl-server auth/reject/logging harness
work. It may include qsl-server tests/harness under a separate explicit
qsl-server packet if test-backed and scoped. It must not change qsl-protocol
runtime/crypto behavior, qsl-attachments implementation, website source,
workflow configuration, scripts, Cargo files, dependencies, branch protection,
or public-safety configuration.

NA-0273 is not implemented by this closeout.

## Evidence Requirements

The closeout must record:

- qsl-server PR #47, head SHA, and merge SHA;
- qsl-protocol PR #797, head SHA, and merge SHA;
- D-0514 and D-0515;
- NA-0272 DONE;
- NA-0273 READY;
- post-merge public-safety success;
- docs-only cost-control behavior for full-suite skips.

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
is governance/testplan only, so docs-only cost control may skip full suites
where policy allows.

## Stop Conditions

Stop if more than one READY item exists, NA-0273 cannot be promoted as the sole
READY successor, public-safety is not required/green, D-0515 would duplicate an
existing decision, or the closeout would need any forbidden path or production
readiness claim.
