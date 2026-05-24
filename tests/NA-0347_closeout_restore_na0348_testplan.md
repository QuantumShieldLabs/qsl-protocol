Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-05-24
Replaces:
Superseded-By:

# NA-0347 Closeout and NA-0348 Restoration Test Plan

## Objective

Close NA-0347 after the qsl-server integration harness and qsl-protocol
governance companion have merged, then restore exactly one READY successor:
NA-0348 -- Metadata Runtime End-to-End qsl-server / qsl-attachments
Integration Evidence Plan.

## Protected Invariants

- NA-0347 is marked DONE only after qsl-server PR #55 and qsl-protocol PR #956
  have merged with required checks green.
- NA-0348 is restored as the sole READY item.
- D-0676 remains present exactly once and D-0677 is added exactly once.
- NA-0348 implementation is not included in the closeout.
- qsl-server/qsl-attachments production-service boundaries remain explicit.
- qsl-attachments PR #37 remains service-local prerequisite evidence only.
- qshield embedded relay/demo evidence remains reference/oracle evidence only.
- No claim is introduced that attachment size, timing metadata, traffic shape,
  all metadata, anonymity, metadata-free behavior, untraceability, production
  readiness, public-internet readiness, or external review completion is
  achieved.

## Allowed Scope

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0347_closeout_restore_na0348_testplan.md`

## Forbidden Scope

- qsl-protocol runtime, protocol, crypto, qsc/qsp, qsl-client, apps, tools,
  formal, scripts, inputs, workflow, dependency, README, START_HERE, docs/public,
  website, qsc-desktop, qsl-server, and qsl-attachments implementation paths.
- qsl-server, qsl-attachments, qshield runtime, branch-protection, public-safety,
  production deployment, secret, or public website changes.
- Branch deletion, direct push, squash, rebase, admin bypass, or history rewrite.

## Required Checks

1. Queue helper reports READY_COUNT 1 and READY NA-0348.
2. `NEXT_ACTIONS.md` records NA-0347 DONE and NA-0348 READY.
3. Decisions helper reports latest decision D-0677, duplicate count zero, and no
   D-0678.
4. Scope guard accepts only the allowed closeout paths.
5. Link-check and leak-scan pass.
6. Changed-line overclaim scan finds no unsupported production, public-internet,
   external-review, anonymity, metadata-free, untraceable, size-hidden,
   timing-hidden, or traffic-shape-hidden claim.
7. Dependency and main-health checks pass: `cargo audit --deny warnings` and
   `cargo tree -i rustls-webpki --locked`.
8. qsc `send_commit`, formal/model checks, classifier proof, PR-body preflight,
   and goal-lint pass.
9. qsl-protocol PR checks complete green and post-merge `public-safety` remains
   required and green.

## Success Criteria

- qsl-protocol closeout PR merges normally with `--match-head-commit`.
- qsl-protocol `origin/main` contains D-0677 exactly once.
- Queue state after merge is READY_COUNT 1, READY NA-0348, NA-0347 DONE.
- D-0678 remains absent.
- NA-0348 remains a future evidence-planning lane and is not implemented by this
  closeout.
