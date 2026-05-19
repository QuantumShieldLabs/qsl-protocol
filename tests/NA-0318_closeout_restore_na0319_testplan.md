Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-19

# NA-0318 Closeout and NA-0319 Restoration Testplan

Goals: G1, G2, G3, G4, G5

## Objective

Close out NA-0318 after the qshield ack/commit implementation PR merged and
restore exactly one READY successor:

`NA-0319 -- Metadata Runtime Identifier and Default Padding Executable Harness`

This closeout must not implement NA-0319.

## Protected Invariants

- Preserve exactly one READY item after closeout: NA-0319.
- Mark NA-0318 DONE only after PR #895 merged and post-merge `public-safety`
  was green.
- Preserve D-0615 implementation/harness evidence.
- Add D-0616 closeout evidence exactly once.
- Preserve the qshield embedded relay versus qsl-server production boundary.
- Do not claim runtime metadata reduction is complete.
- Do not claim anonymity, metadata-free behavior, untraceability,
  public-internet readiness, production readiness, or external review
  completion.
- Do not change runtime, protocol, crypto, qsc, qsp, key schedule, qshield
  implementation, qsl-server, qsl-attachments, qsc-desktop, website, README,
  START_HERE, workflow, Cargo, dependency, branch-protection, or public-safety
  paths.

## Allowed Scope

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0318_closeout_restore_na0319_testplan.md`

## Forbidden Scope

- NA-0319 implementation.
- qshield runtime implementation files.
- qsc, qsp, protocol-core, crypto state-machine, key schedule, service,
  website, qsc-desktop, qsl-server, qsl-attachments, workflow, Cargo,
  dependency, README, START_HERE, branch-protection, or public-safety changes.

## Closeout Evidence Requirements

The closeout must record:

- qsl-protocol PR #895 URL and merge SHA;
- validated head SHA;
- post-merge `public-safety` result;
- D-0615 implementation/harness boundary;
- selected successor rationale;
- no NA-0319 implementation;
- no qsl-server production relay claim.

## Queue Requirements

After patch:

- READY_COUNT is 1;
- READY is NA-0319;
- NA-0318 is DONE;
- D-0616 exists once;
- D-0617 is absent.

## Successor Requirements

NA-0319 must be the implementation lane selected by NA-0318:

`NA-0319 -- Metadata Runtime Identifier and Default Padding Executable Harness`

The successor must protect:

- no unsupported production/public-internet/external-review/anonymity claims;
- no claim of metadata-free or untraceable behavior;
- executable proof or exact prerequisite stop;
- qsl-server production boundary remains explicit;
- qshield embedded relay proof is not presented as qsl-server production proof.

## Required Local Checks

Run or record exact blocker:

- `git diff --check`
- `python3 scripts/ci/qsl_evidence_helper.py queue`
- `python3 scripts/ci/qsl_evidence_helper.py decisions`
- `python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main`
- `python3 scripts/ci/qsl_evidence_helper.py link-check`
- `python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main`
- classifier proof for the changed path set
- local goal-lint with a PR-event payload
- `cargo audit --deny warnings`
- `cargo tree -i rustls-webpki --locked`
- `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1`
- `python3 formal/model_qsc_handshake_suite_id_bounded.py`
- `python3 formal/run_model_checks.py`

## CI Expectations

The closeout PR must keep required checks green, including `public-safety`. If
CI is docs/governance-only cost-controlled, record the classifier result and
public-safety result. Do not merge if required checks are red or missing.

## NA-0262A Cost-Control Timing Proof

This closeout is governance/testplan only. Classifier evidence should show
`docs_only=true`, allowing cost-controlled full-suite skips where branch
protection permits. Required checks, especially `public-safety`, must still
attach and complete successfully.

## Handoff

After closeout merge and post-merge `public-safety` success, the next directive
should run NA-0319. It must implement only the exact metadata runtime
identifier/default-padding executable harness authorized by that future
directive or stop with exact prerequisite evidence.
