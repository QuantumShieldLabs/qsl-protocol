Goals: G1, G3, G4, G5

# NA-0277 Closeout and NA-0278 Restoration Testplan

## Objective

Verify that NA-0277 closes only after the qsl-server abuse/rate/queue harness
and qsl-protocol evidence PRs merge with public-safety green, and that exactly
one successor, NA-0278, becomes READY without implementing NA-0278.

## Protected Invariants

- READY_COUNT is exactly 1 after closeout.
- NA-0277 is DONE after closeout.
- NA-0278 is READY after closeout.
- D-0524 exists once before closeout.
- D-0525 exists once after closeout.
- No duplicate decision IDs exist.
- qsl-server PR #52 evidence remains linked.
- qsl-protocol PR #807 evidence remains linked.
- No branch deletion is authorized or performed.
- No production implementation is authorized.
- No production-readiness claim is introduced.
- No qsl-protocol runtime, protocol, wire, crypto, auth, state-machine, qsp
  protocol-core, qsc/qsl runtime, qsl-attachments, qsl-server implementation,
  qsc-desktop, website, workflow, script, Cargo, dependency, branch-protection,
  or public-safety configuration path changes.

## Closeout Proof

The closeout must record:

- qsl-server PR #52 head and merge SHAs;
- qsl-protocol PR #807 head and merge SHAs;
- chosen/current qsl-server overload/rate semantics;
- D-0524 evidence decision;
- D-0525 closeout decision;
- post-merge public-safety success for PR #807 merge;
- NA-0278 successor text and constraints.

## NA-0278 Successor Boundary

NA-0278 is docs/governance/read-only audit work only:

- README public-attention refresh;
- docs/public evidence links only if needed;
- governance evidence and testplan;
- read-only stale branch audit;
- branch cleanup recommendation list;
- no branch deletion without later explicit approval;
- no implementation changes;
- no website/external repo changes.

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
- local goal-lint or helper PR-body preflight with exact Goals line.

Required CI must pass normally before merge. This closeout is
docs/governance-only, so NA-0262A cost control may skip full suites where
policy allows.
