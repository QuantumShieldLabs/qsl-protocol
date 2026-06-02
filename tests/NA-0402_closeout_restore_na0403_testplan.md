Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-01

# NA-0402 Closeout / Restore NA-0403 Testplan

Goals: G1, G2, G3, G4, G5

## Objective

Verify that NA-0402 is closed after the Director State Index authorization plan
merges and that exactly one successor is restored:

`NA-0403 -- QSL Director State Index Implementation Harness`

This closeout must not implement NA-0403 or create a Director State Index.

## Protected Invariants

- READY_COUNT is exactly 1.
- READY item is NA-0403 after closeout.
- NA-0402 is DONE.
- D-0786 exists once.
- D-0787 exists once.
- D-0788 is absent.
- public-safety remains required and green.
- No branch-protection, workflow, runtime, protocol, crypto, dependency, Cargo,
  qshield runtime, qsl-server, qsl-attachments, qsc-desktop, website, README,
  START_HERE, docs/public, backup script/timer/fstab/source-list, response
  archive, local history, durable Director State Index, local qstart/qresume,
  or secret-bearing path is changed.

## Allowed Scope

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0402_closeout_restore_na0403_testplan.md`

## Forbidden Scope

- Runtime, protocol, crypto, qsc/qsp/qsl, qshield runtime, service, workflow,
  dependency, Cargo, qsl-server, qsl-attachments, qsc-desktop, website,
  README, START_HERE, docs/public, public security policy, security.txt,
  SECURITY.md, issue template, public paper, backup script/timer/fstab,
  backup source-list, off-host target, restore target, key, credential,
  passphrase, response archive, request archive, local history, qstart/qresume,
  Director State Index implementation, durable Director State Index output, and
  branch-protection mutation.

## Preconditions

- qsl-protocol PR #1067 is merged.
- PR #1067 merge commit is recorded.
- post-merge public-safety on the PR #1067 merge commit is green.
- Queue before closeout has READY_COUNT 1 and READY NA-0402.
- D-0786 exists once and D-0787 is absent before closeout.
- The selected successor is exact:
  `NA-0403 -- QSL Director State Index Implementation Harness`.

## Queue Requirements

- Mark NA-0402 DONE.
- Restore NA-0403 with exact title:
  `QSL Director State Index Implementation Harness`.
- Keep READY_COUNT 1.
- Do not implement NA-0403 or create a Director State Index.

## Decision Requirements

D-0787 must state:

- NA-0402 delivered Director State Index authorization planning.
- NA-0403 is selected based on NA-0402 evidence.
- No NA-0403 implementation is authorized by closeout.
- Runtime, security, backup, public-doc, public-claim, local-history,
  response-archive, durable-index, and secret-handling boundaries remain
  protected.

## Traceability Requirements

TRACEABILITY must link:

- D-0787.
- PR #1067.
- NA-0402 evidence and D-0786 dependency.
- NA-0403 selected successor.
- Backup-impact classification.
- qsl-server and qsl-attachments read-only boundaries.

## Validation Commands

Run or record:

- `git status --porcelain=v1 --branch`
- `git diff --name-only origin/main...HEAD`
- `git diff --stat origin/main...HEAD`
- `git diff --check`
- `python3 scripts/ci/qsl_evidence_helper.py queue`
- `python3 scripts/ci/qsl_evidence_helper.py decisions`
- D-0787 count is one.
- D-0788 count is zero.
- Scope guard for the exact allowed path set.
- Link-check, leak-scan, classifier, and PR-body preflight / goal-lint.
- `cargo audit --deny warnings`
- qsc send_commit.
- formal model checks.

## Backup Impact

No backup-plan update is required if the changed path set remains limited to the
allowed qsl-protocol governance/testplan/traceability/journal files.

Future durable Director State Index output remains future-lane scoped and
requires its own backup-impact assessment if durable nonstandard storage is
introduced.

## Successor Handoff

The next directive may authorize NA-0403 implementation-harness work only. It
must not infer runtime, crypto, dependency, public-doc, website, public
technical paper, security policy, disclosure program, durable local index,
response archive, local history, or public-claim authorization from this
closeout.
