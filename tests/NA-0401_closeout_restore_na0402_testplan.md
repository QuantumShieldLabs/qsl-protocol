Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-01

# NA-0401 Closeout / Restore NA-0402 Testplan

Goals: G1, G2, G3, G4, G5

## Objective

Verify that NA-0401 is closed after the internal project-goal and
operating-principles canon merges and that exactly one successor is restored:

`NA-0402 -- QSL Director State Index Authorization Plan`

This closeout must not implement NA-0402 or create a Director State Index.

## Protected Invariants

- READY_COUNT is exactly 1.
- READY item is NA-0402 after closeout.
- NA-0401 is DONE.
- D-0784 exists once.
- D-0785 exists once.
- D-0786 is absent.
- public-safety remains required and green.
- No branch-protection, workflow, runtime, protocol, crypto, dependency, Cargo,
  qshield runtime, qsl-server, qsl-attachments, qsc-desktop, website, README,
  START_HERE, docs/public, backup script/timer/fstab/source-list, response
  archive, local qstart/qresume, or secret-bearing path is changed.

## Allowed Scope

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0401_closeout_restore_na0402_testplan.md`

## Forbidden Scope

- Runtime, protocol, crypto, qsc/qsp/qsl, qshield runtime, service, workflow,
  dependency, Cargo, qsl-server, qsl-attachments, qsc-desktop, website,
  README, START_HERE, docs/public, public security policy, security.txt,
  SECURITY.md, issue template, public paper, backup script/timer/fstab,
  backup source-list, off-host target, restore target, key, credential,
  passphrase, response archive, request archive, qstart/qresume, Director State
  Index implementation, and branch-protection mutation.

## Preconditions

- qsl-protocol PR #1065 is merged.
- PR #1065 merge commit is recorded.
- post-merge public-safety on the PR #1065 merge commit is green.
- Queue before closeout has READY_COUNT 1 and READY NA-0401.
- D-0784 exists once and D-0785 is absent before closeout.
- The selected successor is exact:
  `NA-0402 -- QSL Director State Index Authorization Plan`.

## Queue Requirements

- Mark NA-0401 DONE.
- Restore NA-0402 with exact title:
  `QSL Director State Index Authorization Plan`.
- Keep READY_COUNT 1.
- Do not implement NA-0402 or create a Director State Index.

## Decision Requirements

D-0785 must state:

- NA-0401 delivered internal project-goal and operating-principles canon.
- NA-0402 is selected based on NA-0401 evidence.
- No NA-0402 implementation is authorized by closeout.
- Runtime, security, backup, public-doc, public-claim, and secret-handling
  boundaries remain protected.

## Traceability Requirements

TRACEABILITY must link:

- D-0785.
- PR #1065.
- NA-0401 canon artifact and D-0784 dependency.
- NA-0402 selected successor.
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
- D-0785 count is one.
- D-0786 count is zero.
- Scope guard for the exact allowed path set.
- Link-check, leak-scan, classifier, and PR-body preflight / goal-lint.
- `cargo audit --deny warnings`
- qsc send_commit.
- formal model checks.

## Backup Impact

No backup-plan update is required if the changed path set remains limited to the
allowed qsl-protocol governance/testplan/traceability/journal files.

Future Director State Index durable artifacts remain future-lane scoped and
require their own backup-impact assessment if durable nonstandard storage is
introduced.

## Successor Handoff

The next directive may authorize NA-0402 planning only. It must not infer
runtime, crypto, dependency, public-doc, website, public technical paper,
security policy, disclosure program, Director State Index implementation, or
public-claim authorization from this closeout.
