Goals: G1, G3, G4, G5

# NA-0274 Closeout and NA-0275 Restoration Testplan

## Objective

Verify that NA-0274 closes only after the qsl-attachments executable
malformed JSON / reject-taxonomy harness and qsl-protocol evidence PR are
merged, and that NA-0275 is restored as the sole READY successor without
implementing NA-0275.

## Protected Invariants

- Exactly one READY item exists after closeout: NA-0275.
- NA-0274 is DONE after closeout.
- D-0518 exists once.
- D-0519 exists once.
- qsl-attachments PR #32 evidence remains recorded.
- qsl-protocol PR #801 evidence remains recorded.
- public-safety remains required and green.
- No qsl-protocol runtime, protocol, wire, crypto, auth, state-machine,
  qsl-server, qsl-attachments, qsc-desktop, website, workflow, script, Cargo,
  dependency, branch-protection, or public-safety configuration path changes.
- No production readiness or deployment readiness claim is introduced.

## Closeout Evidence Requirements

The closeout must record:

- qsl-attachments PR #32 head and merge SHA;
- qsl-protocol PR #801 head and merge SHA;
- D-0518 evidence;
- D-0519 closeout decision;
- post-merge public-safety success on PR #801 merge;
- successor NA-0275 READY block exactly as authorized by directive scope.

## NA-0275 Successor Requirements

NA-0275 must be:

- `qsl-server x-msg-id / Idempotency Semantics Decision and Harness`;
- READY;
- Goals: G1, G3, G4, G5;
- executable qsl-server harness work, not docs-only;
- allowed to change qsl-server wire/behavior only if test-backed and
  explicitly scoped;
- forbidden from crypto/state-machine change;
- forbidden from qsl-protocol runtime/crypto, qsl-attachments, and website
  changes.

## Validation Expectations

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
- PR-body preflight or goal-lint with the exact Goals line.

The expected qsl-protocol closeout PR scope is docs/governance/testplan only,
so docs-only cost control may skip full suites where policy allows.

## Success Criteria

- NA-0274 DONE.
- NA-0275 READY.
- D-0519 latest decision.
- Required CI passes normally.
- public-safety remains required and green.
- No implementation or production-readiness drift.
