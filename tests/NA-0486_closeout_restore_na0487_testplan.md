Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-15
Replaces:
Superseded-By:

# NA-0486 Closeout Restore NA-0487 Testplan

Goals: G1, G2, G3, G4, G5

## Objective

Validate the NA-0486 closeout and restoration of the selected NA-0487
successor. The closeout must mark NA-0486 DONE, restore exactly one READY item,
and not implement NA-0487.

## Protected invariants

- Evidence PR #1242 is merged before closeout.
- Post-evidence public-safety is completed success before closeout.
- NA-0486 is marked DONE.
- NA-0487 is restored as the sole READY item.
- D-0961 exists exactly once.
- D-0962 is absent.
- no qsc source mutation occurs in closeout.
- no qsc fuzz target, qsc fuzz Cargo, corpus, or qsc-adversarial script mutation
  occurs in closeout.
- no runtime, crypto, dependency, lockfile, workflow, refimpl, vector, formal,
  service, public-doc, backup, qsl-backup, qwork/qstart/qresume/qshell, or
  rollback mutation occurs in closeout.
- no public-readiness claim is introduced.
- no crypto-complete claim is introduced.
- no fuzz-complete claim is introduced.
- no vector-complete claim is introduced.
- no KEM-complete claim is introduced.
- no signature-complete claim is introduced.
- no replay-proof claim is introduced.
- no downgrade-proof claim is introduced.
- no side-channel-free claim is introduced.
- no vulnerability-free claim is introduced.
- no perfect-crypto claim is introduced.

## Allowed scope

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- this testplan

## Forbidden scope

- qsc source, runtime, or executable-test mutation.
- qsc fuzz target, qsc fuzz Cargo, corpus, or lockfile mutation.
- qsc-adversarial script or workflow mutation.
- dependency mutation.
- refimpl source or test mutation.
- vector/input mutation.
- formal model mutation.
- service, qshield, qsl-server, qsl-attachments, website, public-doc, README,
  or START_HERE mutation.
- qwork, qstart, qresume, or qshell mutation.
- backup, restore, qsl-backup, backup status, backup plan, rollback, or backup
  tree mutation.

## Required validation

- Scope guard reports exactly the five allowed closeout paths.
- `git diff --check` passes.
- Link-check passes.
- Leak-scan passes.
- Added-line overclaim scan passes.
- PR body preflight passes.
- Goal-lint passes.
- Queue proof reports READY_COUNT 1 and READY NA-0487.
- Decision proof reports latest D-0961, D-0960 once, D-0961 once, D-0962 absent,
  and duplicate decision count zero.
- Root cargo audit remains green.
- Nested qsc fuzz lock audit remains green.
- public-safety is green before merge.

## Acceptance

Closeout is acceptable only if NA-0487 is the sole READY item, no implementation
path is changed, D-0961 records the restoration, and public-safety is green.
