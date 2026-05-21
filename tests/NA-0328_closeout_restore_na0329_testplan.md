Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-21

# NA-0328 Closeout Restore NA-0329 Testplan

## Objective

Close NA-0328 after PR #918 merged the metadata runtime qshield embedded
relay/demo bounded jitter authorization plan, then restore exactly one READY
successor:

`NA-0329 -- Metadata Runtime qshield Demo Bounded Jitter Implementation Harness`

This closeout must not implement NA-0329.

## Protected Invariants

- Exactly one READY item exists after closeout: NA-0329.
- NA-0328 is DONE.
- D-0638 remains present once.
- D-0639 records the closeout.
- No later decision ID is introduced by this closeout.
- No NA-0329 implementation is included.
- No qshield runtime, qsl-server, qsl-attachments, qsc/qsp/protocol/crypto,
  key-schedule, Cargo/dependency, workflow, website, README, START_HERE,
  branch-protection, public-safety, qsc-desktop, docs/public, formal, input,
  tools/refimpl, or app runtime change is included.
- The NA-0328 authorization plan is not presented as implemented mitigation.
- qshield embedded relay/demo proof remains distinct from qsl-server and
  qsl-attachments production timing.
- qsl-server and qsl-attachments production timing remain unproven and
  cross-repo-gated.
- Timing metadata and traffic shape are not claimed hidden.
- No anonymity, metadata-free, untraceable, production-readiness,
  public-internet-readiness, or external-review-complete claim is introduced.

## Allowed Scope

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0328_closeout_restore_na0329_testplan.md`

## Forbidden Scope

- NA-0329 implementation.
- Bounded jitter implementation.
- Runtime timing mitigation.
- qshield runtime changes.
- qsl-server or qsl-attachments changes.
- qsc/qsp/protocol/crypto/key-schedule changes.
- Cargo or dependency changes.
- Workflow, website, README, START_HERE, branch-protection, or public-safety
  configuration changes.

## Merge Evidence Requirements

Record:

- PR #918 title;
- PR #918 head SHA;
- PR #918 merge SHA;
- post-merge `public-safety` status on the merge SHA;
- selected successor from D-0638 evidence.

## Queue Requirements

After patch:

- `python3 scripts/ci/qsl_evidence_helper.py queue` reports
  `READY_COUNT 1`.
- The sole READY item is NA-0329.
- NA-0328 status is DONE.
- NA-0329 body states that it is future implementation scope and has not been
  implemented by closeout.

## Decision Requirements

After patch:

- `python3 scripts/ci/qsl_evidence_helper.py decisions` reports latest
  decision D-0639.
- Duplicate decision count is zero.
- D-0638 remains present once.
- D-0640 remains absent.

## Required Local Checks

- `python3 scripts/ci/qsl_evidence_helper.py queue`
- `python3 scripts/ci/qsl_evidence_helper.py decisions`
- exact allowed-path scope guard
- `python3 scripts/ci/qsl_evidence_helper.py link-check`
- `python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main`
- changed-line overclaim scan
- classifier proof for changed paths
- local goal-lint via synthetic PR event
- `cargo audit --deny warnings`
- `cargo tree -i rustls-webpki --locked`
- `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1`
- `python3 formal/run_model_checks.py`

## CI Expectations

Required PR checks must pass normally before merge, including `public-safety`.
Cost-control skips are acceptable only when CI reports them as skipped and
public-safety remains green.

## Successor Boundary

NA-0329 must remain a queued successor only. It may not be implemented,
partially implemented, or presented as complete by this closeout.
