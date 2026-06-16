Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-16
Replaces:
Superseded-By:

# NA-0487 Closeout Restore NA-0488 Testplan

Goals: G1, G2, G3, G4, G5

## Objective

Validate the NA-0487 closeout and restoration of the selected NA-0488
successor. The closeout must verify PR #1245 post-merge safety, mark NA-0487
DONE, restore exactly one READY item, and not implement NA-0488.

## Protected invariants

- PR #1245 is merged before closeout.
- Post-merge public-safety for `ea1af60e84f0` is completed success before
  closeout mutation.
- qsc-adversarial-smoke for `ea1af60e84f0` is completed success.
- qsc-linux-full-suite and macos-qsc-full-serial are completed success or
  accepted by repository policy before closeout.
- D-0963 exists exactly once.
- D-0964 exists exactly once after closeout.
- D-0965 is absent after closeout.
- NA-0487 is marked DONE.
- NA-0488 is restored as the sole READY item.
- no implementation mutation occurs in closeout.
- no qsc source mutation occurs in closeout.
- no qsc fuzz target, qsc fuzz Cargo, corpus, or qsc-adversarial script mutation
  occurs in closeout.
- no runtime, crypto, dependency, lockfile, workflow, refimpl, vector, formal,
  service, public-doc, backup, qsl-backup, qwork/qstart/qresume/qshell, status,
  plan, rollback, or backup tree mutation occurs in closeout.
- no public-readiness claim is introduced.
- no crypto-complete claim is introduced.
- no fuzz-complete claim is introduced.
- no vector-complete claim is introduced.
- no replay-proof claim is introduced.
- no downgrade-proof claim is introduced.
- no side-channel-free claim is introduced.
- no vulnerability-free claim is introduced.
- no bug-free claim is introduced.
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
- Classifier reports docs-only/governance-only scope for the closeout diff.
- PR body preflight passes.
- Goal-lint passes.
- Queue proof reports READY_COUNT 1 and READY NA-0488.
- Queue proof reports NA-0487 DONE.
- Decision proof reports D-0963 once, D-0964 once, D-0965 absent, and duplicate
  decision count zero.
- `cargo fmt --check` passes.
- Root cargo audit remains green.
- Nested qsc fuzz lock audit remains green.
- `python3 formal/run_model_checks.py` passes.
- Internal negative binding vector manifest JSON validates.
- Post-closeout public-safety is green before merge.

## Acceptance

Closeout is acceptable only if NA-0488 is the sole READY item, no implementation
path is changed, D-0964 records the restoration, PR #1245 public-safety proof is
green, and no public overclaim is introduced.
