Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-30

# NA-0389 Closeout and NA-0390 Restoration Testplan

Goals: G1, G2, G3, G4, G5

## Objective

Close NA-0389 after the routine audit cadence authorization PR merged and
restore `NA-0390 -- QSL Local Ops Routine Audit Cadence Implementation Harness`
as the sole READY successor.

## Protected invariants

- Exactly one READY queue item exists after closeout.
- NA-0389 is DONE.
- NA-0390 is READY and is not executed by this closeout.
- D-0760 exists once, D-0761 exists once, and D-0762 is absent.
- public-safety remains required and green.
- No audit helper is implemented.
- No audit scheduler, cron, workflow, timer, or background automation is
  created.
- No durable audit report output is created.
- No response, request, directive, journal, or ops-history archive is mutated.
- No runtime, service, protocol, crypto, dependency, workflow, backup script,
  timer, fstab, public-safety script, qsl-server, qsl-attachments, qshield
  runtime, website, README, START_HERE, or docs/public path changes.
- No secret handling, target setup, off-host backup setup, restore, deploy,
  rollback, or public/readiness/privacy claim expansion.

## Allowed scope

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0389_closeout_restore_na0390_testplan.md`

## Forbidden scope

All implementation, runtime, helper, workflow, dependency, public-doc, backup,
history-root, qsl-server, qsl-attachments, qshield runtime, qsc, qsp, protocol,
crypto, service, durable audit report, scheduler, cron, and local tool paths
remain forbidden unless future live scope explicitly authorizes exact files.

## Queue and decision requirements

- `python3 scripts/ci/qsl_evidence_helper.py queue` reports READY_COUNT `1` and
  READY `NA-0390`.
- `python3 scripts/ci/qsl_evidence_helper.py decisions` reports latest D-0761
  with duplicate count `0`.
- Direct scan confirms D-0762 is absent.

## Closeout proof

- NEXT_ACTIONS records PR #1041 head `d21daab1df42` and merge
  `13acdd0e9268`.
- DECISIONS records D-0761 and selected successor NA-0390.
- TRACEABILITY links D-0761, this testplan, PR #1041, and post-merge
  public-safety proof.
- Rolling journal records Packet U closeout branch and scope.

## Validation requirements

- `git diff --check`
- `python3 scripts/ci/qsl_evidence_helper.py queue`
- `python3 scripts/ci/qsl_evidence_helper.py decisions`
- Scope guard over changed files.
- Markdown link integrity check.
- Leak and overclaim scan over changed files.
- `cargo audit --deny warnings`
- `cargo tree -i rustls-webpki --locked`
- `cargo fmt --check`
- qsc `send_commit` test.
- Formal model checks.
- Goal-lint on the closeout PR body.

## CI expectations

The closeout PR must merge only after required qsl-protocol checks complete
green or accepted under existing neutral-check policy. public-safety remains
required before and after merge.

## Successor handoff

NA-0390 may implement a bounded temp-output routine audit cadence harness if its
future live scope authorizes exact files. This closeout does not implement that
harness, change audit tooling, create a scheduler, create a durable audit report
store, change runtime behavior, or mutate local history archives.
