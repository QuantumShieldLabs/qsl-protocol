Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-21

# NA-0329 Closeout Restore NA-0330 Testplan

## Objective

Close NA-0329 after PR #920 merged the metadata runtime qshield embedded
relay/demo bounded jitter implementation harness, then restore exactly one READY
successor:

`NA-0330 -- Metadata Runtime qshield Demo Batching Authorization Plan`

This closeout must not implement NA-0330.

## Protected Invariants

- Exactly one READY item exists after closeout: NA-0330.
- NA-0329 is DONE.
- D-0640 remains present once.
- D-0641 records the closeout.
- No later decision ID is introduced by this closeout.
- No NA-0330 implementation is included.
- No batching, cover traffic, broad queue scheduling, production-service timing
  behavior, qshield runtime, qsl-server, qsl-attachments, qsc/qsp/protocol/
  crypto, key-schedule, Cargo/dependency, workflow, website, README,
  START_HERE, branch-protection, public-safety, qsc-desktop, docs/public,
  formal, input, tools/refimpl, or app runtime change is included.
- The NA-0329 bounded-jitter proof is not presented as production proof.
- qshield embedded relay/demo proof remains distinct from qsl-server and
  qsl-attachments production timing.
- qsl-server and qsl-attachments production timing remain unproven and
  cross-repo-gated.
- Timing metadata and traffic shape are not claimed hidden.
- No anonymity, metadata-free, untraceable, production-readiness, public-internet-readiness, or external-review-complete claim is introduced.

## Allowed Scope

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0329_closeout_restore_na0330_testplan.md`

## Forbidden Scope

- NA-0330 implementation.
- Batching implementation.
- Cover traffic implementation.
- Broad queue scheduling implementation.
- Production-service timing behavior.
- qshield runtime changes.
- qsl-server or qsl-attachments changes.
- qsc/qsp/protocol/crypto/key-schedule changes.
- Cargo or dependency changes.
- Workflow, website, README, START_HERE, branch-protection, or public-safety
  configuration changes.

## Merge Evidence Requirements

Record:

- PR #920 title;
- PR #920 head SHA;
- PR #920 merge SHA;
- post-merge `public-safety` status on the merge SHA;
- selected successor from D-0640 evidence.

## Queue Requirements

After patch:

- `python3 scripts/ci/qsl_evidence_helper.py queue` reports
  `READY_COUNT 1`.
- The sole READY item is NA-0330.
- NA-0329 status is DONE.
- NA-0330 body states that it is an authorization/design lane and has not been
  implemented by closeout.

## Decision Requirements

After patch:

- `python3 scripts/ci/qsl_evidence_helper.py decisions` reports latest
  decision D-0641.
- Duplicate decision count is zero.
- D-0640 remains present once.
- D-0642 remains absent.

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

NA-0330 must remain a queued authorization/design successor only. It may not be
implemented, partially implemented, or presented as complete by this closeout.
