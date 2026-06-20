Status: Supporting
Owner: QSL Governance / Core Assurance
Last-Updated: 2026-06-20

# NA-0506 Closeout and NA-0507 Restoration Testplan

## Objective

Verify that NA-0506 is closed after the runbook implementation PR merged and
post-merge public-safety completed success, and that NA-0507 is restored as
the sole READY successor without implementing NA-0507.

## Protected invariants

- Exactly one READY item remains mandatory.
- NA-0506 is DONE.
- NA-0507 is READY.
- D-1002 exists once.
- No duplicate decision IDs.
- No remote action.
- No SSH execution.
- No remote account creation.
- No SSH key generation or installation.
- No local SSH config mutation.
- No system SSH config mutation.
- No known_hosts mutation.
- No authorized_keys mutation.
- No remote host mutation.
- No sudo/admin action.
- No qwork/qstart/qresume mutation.
- No qsl-backup execution or mutation.
- No qsc source/test/fuzz/Cargo mutation.
- No workflow/script/helper/dependency mutation.
- No corpus/vector/input mutation.
- No formal/refimpl/service/public/backup mutation.
- No public-readiness claim.
- No production-readiness claim.
- No public-internet-readiness claim.
- No crypto-complete claim.
- No replay-proof claim.
- No downgrade-proof claim.
- No side-channel-free claim.
- No vulnerability-free, bug-free, or perfect-crypto claim.

## Allowed scope

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0506_closeout_restore_na0507_testplan.md`

## Required evidence

- PR #1284 merged.
- D-1001 exists once on main.
- NA-0506 runbook exists on main.
- NA-0506 testplan exists on main.
- post-merge public-safety completed success on the PR #1284 merge commit
  inside the short attach/early-failure window.
- no completed red check was observed during the short post-merge poll.
- selected successor is the exact NA-0507 manual-action-readiness block.

## Validation commands

Required:

```bash
git diff --check
```

Required proof checks:

- exact five-path closeout scope guard.
- link-check.
- added-line leak scan.
- added-line overclaim scan.
- docs/governance classifier.
- PR body preflight.
- goal-lint.
- queue/decision proof.

## Acceptance criteria

- NA-0506 is marked DONE.
- NA-0507 is the only READY item.
- D-1002 records NA-0506 closeout and NA-0507 restoration.
- NA-0507 remains manual-action-readiness only.
- closeout does not implement NA-0507.
- closeout does not authorize Codex remote action.
