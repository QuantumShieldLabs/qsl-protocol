Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-30

# NA-0388 Closeout and NA-0389 Restoration Testplan

## Objective

Close NA-0388 after the response archive/history catalog temp-output harness
merged and restore `NA-0389 -- QSL Local Ops Routine Audit Cadence Authorization
Plan` as the sole READY successor.

## Protected Invariants

- Exactly one READY queue item exists after closeout.
- NA-0388 is DONE.
- NA-0389 is READY and is not executed by this closeout.
- D-0758 exists once, D-0759 exists once, and D-0760 is absent.
- public-safety remains required and green.
- No durable catalog/index output is created.
- No response, request, directive, journal, or ops-history archive is mutated.
- No runtime, service, protocol, crypto, dependency, workflow, backup script,
  timer, fstab, public-safety script, qsl-server, qsl-attachments, qshield
  runtime, website, README, START_HERE, or docs/public path changes.
- No secret handling, target setup, off-host backup setup, restore, deploy,
  rollback, or public/readiness/privacy claim expansion.

## Allowed Scope

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0388_closeout_restore_na0389_testplan.md`

## Forbidden Scope

All implementation, runtime, helper, workflow, dependency, public-doc, backup,
history-root, qsl-server, qsl-attachments, qshield runtime, qsc, qsp, protocol,
crypto, service, and local tool paths remain forbidden unless future live scope
explicitly authorizes exact files.

## Queue and Decision Requirements

- `python3 scripts/ci/qsl_evidence_helper.py queue` reports READY_COUNT `1` and
  READY `NA-0389`.
- `python3 scripts/ci/qsl_evidence_helper.py decisions` reports latest D-0759
  with duplicate count `0`.
- Direct scan confirms D-0760 is absent.

## Closeout Proof

- NEXT_ACTIONS records PR #1039 head `48b233de9ac6` and merge `c928998a298f`.
- DECISIONS records D-0759 and selected successor NA-0389.
- TRACEABILITY links D-0759, this testplan, PR #1039, and the post-merge
  public-safety proof.
- Rolling journal records the Packet K closeout branch and scope.

## Validation Requirements

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

## CI Expectations

The closeout PR must merge only after required qsl-protocol checks complete
green or accepted under existing neutral-check policy. public-safety remains
required before and after merge.

## Successor Handoff

NA-0389 may authorize a future recurring audit cadence plan. This closeout does
not implement that plan, change audit tooling, change runtime behavior, or create
durable catalog/index output.
