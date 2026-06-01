Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-01

# NA-0400 Closeout / Restore NA-0401 Testplan

Goals: G1, G2, G3, G4, G5

## Objective

Verify that NA-0400 is closed after the external review / disclosure / public
claim readiness plan merges and that exactly one successor is restored:

`NA-0401 -- QSL Project Goal and Operating Principles Canon Authorization Plan`

This closeout must not implement NA-0401.

## Protected Invariants

- READY_COUNT is exactly 1.
- READY item is NA-0401 after closeout.
- NA-0400 is DONE.
- D-0782 exists once.
- D-0783 exists once.
- D-0784 is absent.
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
- `tests/NA-0400_closeout_restore_na0401_testplan.md`

## Forbidden Scope

- Runtime, protocol, crypto, qsc/qsp/qsl, qshield runtime, service, workflow,
  dependency, Cargo, qsl-server, qsl-attachments, qsc-desktop, website,
  README, START_HERE, docs/public, public security policy, security.txt,
  SECURITY.md, issue template, public paper, backup script/timer/fstab,
  backup source-list, off-host target, restore target, key, credential,
  passphrase, response archive, request archive, qstart/qresume, and branch
  protection mutation.

## Preconditions

- qsl-protocol PR #1063 is merged.
- PR #1063 merge commit is recorded.
- post-merge public-safety on the PR #1063 merge commit is green.
- Queue before closeout has READY_COUNT 1 and READY NA-0400.
- D-0782 exists once and D-0783 is absent before closeout.
- No source-verification or public-claim blocker successor was selected by
  NA-0400.

## Queue Requirements

- Mark NA-0400 DONE.
- Restore NA-0401 with exact title:
  `QSL Project Goal and Operating Principles Canon Authorization Plan`.
- Keep READY_COUNT 1.
- Do not implement NA-0401 or create the future canon artifact.

## Decision Requirements

D-0783 must state:

- NA-0400 delivered external review / disclosure / public claim readiness
  planning.
- NA-0401 is selected based on NA-0400 evidence.
- No NA-0401 implementation is authorized by closeout.
- Runtime, security, backup, public-doc, public-claim, and secret-handling
  boundaries remain protected.

## Traceability Requirements

TRACEABILITY must link:

- D-0783.
- PR #1063.
- NA-0400 evidence and D-0782 dependency.
- NA-0401 selected successor.
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
- D-0783 count is one.
- D-0784 count is zero.
- Scope guard for the exact allowed path set.
- Link-check, leak-scan, classifier, and goal-lint.
- `cargo audit --deny warnings`
- qsc send_commit.
- formal model checks.

## Backup Impact

No backup-plan update is required if the changed path set remains limited to the
allowed qsl-protocol governance/testplan/traceability/journal files.

Future project-goal canon artifacts remain future-lane scoped and require their
own backup-impact assessment if durable nonstandard storage is introduced.

## Successor Handoff

The next directive may authorize NA-0401 planning only. It must not infer
runtime, crypto, dependency, public-doc, website, public technical paper,
security policy, disclosure program, or public-claim authorization from this
closeout.
