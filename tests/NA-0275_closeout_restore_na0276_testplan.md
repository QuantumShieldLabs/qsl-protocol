Goals: G1, G3, G4, G5

# NA-0275 Closeout and NA-0276 Restoration Testplan

## Objective

Verify that NA-0275 closes only after qsl-server PR #50, qsl-protocol PR #803,
D-0520 evidence, and post-merge public-safety are green, then restores exactly
one READY successor: NA-0276 qsl-server invalid config fail-closed semantics
harness.

## Protected Invariants

- READY_COUNT is 1 after closeout.
- NA-0275 is DONE after closeout.
- NA-0276 is READY after closeout.
- D-0520 exists once.
- D-0521 exists once after the closeout patch.
- D-0522 remains absent.
- qsl-server PR #50 remains merged.
- qsl-protocol PR #803 remains merged.
- public-safety remains required and green.
- No NA-0276 implementation occurs in this closeout.
- No qsl-protocol implementation paths change.
- No qsl-server or qsl-attachments implementation paths change.
- No protocol, wire, crypto, auth, state-machine, qsp protocol-core, qsc/qsl
  runtime, qsc-desktop, website, workflow, script, Cargo, dependency,
  branch-protection, or public-safety configuration change is introduced.
- No production readiness or deployment readiness claim is introduced.

## Closeout Proof

The closeout must record:

- qsl-server PR #50 head and merge SHA;
- qsl-protocol PR #803 head and merge SHA;
- chosen `x-msg-id` semantics;
- D-0520 evidence decision;
- D-0521 closeout decision;
- post-PR #803 public-safety success;
- NA-0276 successor block with explicit scope and must-protect invariants.

## NA-0276 Successor Scope

NA-0276 is only a future directive target. It must not be implemented by this
closeout.

The restored successor must cover:

- qsl-server invalid `MAX_BODY_BYTES` startup semantics;
- qsl-server invalid `MAX_QUEUE_DEPTH` startup semantics;
- executable qsl-server config/startup harness;
- semantic decision evidence;
- no production-readiness claim;
- route token, auth, and payload no-leak invariants.

Allowed future NA-0276 work is limited to qsl-server tests/harness, qsl-server
docs if needed, qsl-protocol governance/evidence/testplan, and minimal
qsl-server behavior repair only if test-backed and explicitly scoped.

## qsl-protocol Closeout Scope

Allowed qsl-protocol paths:

- `NEXT_ACTIONS.md`;
- `DECISIONS.md`;
- `TRACEABILITY.md`;
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`;
- `tests/NA-0275_closeout_restore_na0276_testplan.md`;
- `docs/governance/evidence/**` only if current repo convention strictly
  requires it.

Forbidden qsl-protocol paths include runtime, protocol, crypto, qsl-server,
qsl-attachments, qsc-desktop, website, workflow, script, Cargo, dependency,
branch-protection, and public-safety configuration paths.

## No Production-Readiness Claim

The closeout may claim only that NA-0275 evidence is merged and that NA-0276 is
restored as the next queue item. It must not claim qsl-server public operation,
public internet exposure safety, production relay approval, production
attachment service operation, external review completion, metadata elimination,
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

Required CI must pass normally before merge. The closeout scope is
docs/governance/testplan only, so docs-only cost control may skip full suites
where policy allows.

## Success Criteria

- qsl-protocol closeout PR merges normally with a merge commit.
- Post-merge main has READY_COUNT 1.
- Post-merge main has READY NA-0276.
- Post-merge main has NA-0275 DONE.
- Post-merge main has D-0521.
- Post-merge public-safety is green.
