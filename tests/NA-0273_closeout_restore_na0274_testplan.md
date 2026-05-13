Goals: G1, G3, G4, G5

# NA-0273 Closeout and NA-0274 Restoration Testplan

## Objective

Verify that NA-0273 closes only after the qsl-server dependency remediation,
qsl-server executable harness, qsl-protocol evidence PR, and post-merge
public-safety proof are complete, then restore NA-0274 as the sole READY
successor without implementing NA-0274.

## Protected Invariants

- Exactly one READY item exists after closeout: NA-0274.
- NA-0273 is DONE after closeout.
- D-0516 exists once before closeout.
- D-0517 exists once after closeout.
- qsl-server PR #48, qsl-server PR #49, and qsl-protocol PR #799 are merged
  before closeout.
- qsl-protocol public-safety is required and green before closeout.
- qsl-protocol public-safety is required and green after closeout.
- No qsl-protocol runtime, protocol, wire, crypto, auth, state-machine, qsp
  protocol-core, qsc/qsl runtime, qsc-desktop, website, workflow, script,
  Cargo, dependency, branch-protection, or public-safety configuration change
  is introduced.
- No qsl-server or qsl-attachments implementation change is introduced by
  closeout.
- No production readiness or deployment readiness claim is introduced.

## Closeout Evidence Requirements

NEXT_ACTIONS must record:

- qsl-server PR #48 head and merge evidence;
- qsl-server PR #49 head and merge evidence;
- qsl-protocol PR #799 head and merge evidence;
- D-0516 and D-0517;
- NA-0273 DONE;
- READY NA-0274.

## NA-0274 Successor Scope

NA-0274 must be defined as qsl-attachments malformed JSON / reject-taxonomy
harness work:

- executable qsl-attachments harness required or prerequisite stop;
- malformed JSON/Axum extractor rejects;
- canonical `reason_code` behavior;
- capability reject behavior;
- rejected requests do not persist objects;
- capability, descriptor, ciphertext, and plaintext do not leak in logs;
- no production readiness claim;
- no crypto/state-machine change.

NA-0274 is not implemented by this closeout.

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
- goal-lint or helper PR-body preflight with the exact Goals line.

## CI Expectations

Required CI must pass normally before merge. This closeout is docs/governance
only, so docs-only cost control may skip full suites where policy allows.

## Stop Conditions

Stop if more than one READY item exists, NA-0274 cannot be promoted as the sole
READY successor, D-0517 would duplicate another decision, required CI is red,
public-safety is missing or not green, or any closeout correction would require
runtime, protocol, crypto, qsl-server, qsl-attachments, website, workflow,
script, Cargo, branch-protection, public-safety, dependency, or production
implementation changes.
