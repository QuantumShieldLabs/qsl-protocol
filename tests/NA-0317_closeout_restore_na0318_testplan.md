Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-18

# NA-0317 Closeout and NA-0318 Restoration Testplan

Goals: G1, G2, G3, G4, G5

## Objective

Close out NA-0317 after the qshield ack/commit authorization PR merged and
restore exactly one READY successor:

`NA-0318 -- Metadata Runtime qshield Ack/Commit Poll Implementation Harness`

This closeout must not implement NA-0318.

## Protected Invariants

- Preserve exactly one READY item after closeout: NA-0318.
- Mark NA-0317 DONE only after PR #893 merged and post-merge public-safety was
  green.
- Preserve D-0613 authorization evidence.
- Add D-0614 closeout evidence exactly once.
- Keep the current destructive qshield poll mutation boundary explicit:
  `PROVEN_REMOTE_MUTATION` and `NEEDS_RUNTIME_CHANGE`.
- Do not claim runtime metadata reduction.
- Do not claim anonymity, metadata-free behavior, untraceability,
  public-internet readiness, production readiness, or external review
  completion.
- Do not change runtime, protocol, crypto, qsc, qsp, key schedule, qsl-server,
  qsl-attachments, qsc-desktop, website, README, START_HERE, workflow, Cargo,
  dependency, branch-protection, or public-safety paths.

## Allowed Scope

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0317_closeout_restore_na0318_testplan.md`

## Forbidden Scope

- NA-0318 implementation.
- qshield runtime implementation files.
- qsc, qsp, protocol-core, crypto state-machine, key schedule, service,
  website, qsc-desktop, qsl-server, qsl-attachments, workflow, Cargo,
  dependency, README, START_HERE, branch-protection, or public-safety changes.

## Closeout Evidence Requirements

The closeout must record:

- qsl-protocol PR #893 URL and merge SHA;
- validated head SHA;
- post-merge public-safety result;
- D-0613 authorization boundary;
- selected successor rationale;
- no NA-0318 implementation;
- no runtime metadata reduction claim.

## Queue Requirements

After patch:

- READY_COUNT is 1;
- READY is NA-0318;
- NA-0317 is DONE;
- D-0614 exists once;
- D-0615 is absent.

## Successor Requirements

NA-0318 must be the implementation harness selected by NA-0317:

`NA-0318 -- Metadata Runtime qshield Ack/Commit Poll Implementation Harness`

The successor must protect:

- no unsupported production/public-internet/external-review/anonymity claims;
- no claim of metadata-free or untraceable behavior;
- executable proof or exact prerequisite stop;
- the inherited qshield mutation boundary classification;
- no broader metadata runtime implementation unless a future exact directive
  authorizes it.

## Required Local Checks

Run or record why not run:

- `git diff --check`
- `python3 scripts/ci/qsl_evidence_helper.py queue`
- `python3 scripts/ci/qsl_evidence_helper.py decisions`
- `python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main`
- `python3 scripts/ci/qsl_evidence_helper.py link-check`
- `python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main`
- `cargo audit --deny warnings`
- `cargo tree -i rustls-webpki --locked`
- `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1`
- `python3 formal/model_qsc_handshake_suite_id_bounded.py`
- `python3 formal/run_model_checks.py`
- local goal-lint with a PR-event payload
- classifier proof for the changed path set

## CI Expectations

The closeout PR must keep required checks green, including `public-safety`. If
CI is docs-only cost-controlled, record the classifier result and public-safety
result. Do not merge if required checks are red or missing.

## NA-0262A Cost-Control Timing Proof

This closeout is docs/governance/testplan only. Classifier evidence should show
`docs_only=true`, allowing the cost-control behavior from NA-0262A to skip
push-only full suites where branch protection permits. Required checks,
especially `public-safety`, must still attach and complete successfully.

## Handoff

After merge and post-merge public-safety success, the next directive should run
NA-0318. It must implement only the qshield ack/commit harness selected by
NA-0317 or stop with exact prerequisite evidence.
