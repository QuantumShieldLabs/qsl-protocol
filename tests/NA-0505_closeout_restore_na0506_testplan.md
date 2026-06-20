Status: Supporting
Owner: QSL Governance / Core Assurance
Last-Updated: 2026-06-20

# NA-0505 Closeout and NA-0506 Restoration Testplan

## Objective

Verify that NA-0505 closes only after the authorization PR merges and
post-merge public-safety completes success, then restore NA-0506 as the sole
READY successor without implementing NA-0506.

## Protected invariants

- Exactly one READY item remains.
- NA-0505 is marked DONE.
- NA-0506 is restored READY.
- D-0999 exists once.
- D-1000 exists once after closeout.
- No NA-0506 implementation is performed.
- No remote action is performed.
- No SSH execution is performed.
- No remote account creation is performed by Codex.
- No SSH key generation or installation is performed by Codex.
- No local SSH config mutation is performed.
- No remote host mutation is performed.
- No qsc source/test/fuzz/Cargo mutation is performed.
- No workflow/script/helper/dependency mutation is performed.
- No corpus/vector/input mutation is performed.
- No formal/refimpl/service/public/backup mutation is performed.
- no public-readiness claim is made.
- no production-readiness claim is made.
- no public-internet-readiness claim is made.
- no crypto-complete claim is made.
- no replay-proof claim is made.
- no downgrade-proof claim is made.
- no side-channel-free claim is made.

## Allowed scope

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0505_closeout_restore_na0506_testplan.md`

## Forbidden scope

- implementing NA-0506.
- creating remote users.
- generating or installing SSH keys.
- running SSH, scp, sftp, or rsync to remote.
- mutating local SSH config.
- mutating system SSH config.
- mutating known_hosts.
- mutating authorized_keys.
- mutating remote hosts.
- sudo/admin action.
- qwork/qstart/qresume mutation.
- qsc source/test/fuzz/Cargo mutation.
- workflow/script/helper/dependency mutation.
- corpus/vector/input mutation.
- formal/refimpl/service/public/backup mutation.
- qsl-backup execution or mutation.
- no public-readiness claim and no production-readiness claim.

## Required proof

Expected:

- PR #1282 merged with merge commit `ebe1c14286f0`.
- Post-merge public-safety completed success in the short attach/early-failure
  window.
- D-0999 exists once on main before closeout.
- NA-0505 remains READY before closeout.
- D-1000 is absent before closeout.
- Closeout patch touches exactly the five allowed closeout paths.

## Queue proof

Expected after patch:

- READY_COUNT 1.
- READY NA-0506.
- NA-0505 DONE.
- D-0999 once.
- D-1000 once.
- duplicate decision count zero.

## Validation

Run and require pass:

```bash
git diff --check
python3 scripts/ci/qsl_evidence_helper.py link-check --root .
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
python3 scripts/ci/qsl_evidence_helper.py pr-body-preflight --file <closeout-pr-body> --scan-overclaims
```

Also run:

- exact five-path closeout scope guard.
- added-line overclaim scan.
- docs-only classifier.
- local goal-lint with a synthesized PR event.
- queue/decision proof.

## Public claim boundary

The closeout PR must preserve:

- no public-readiness claim.
- no production-readiness claim.
- no public-internet-readiness claim.
- no external-review-complete claim.
- no crypto-complete claim.
- no replay-proof claim.
- no downgrade-proof claim.
- no side-channel-free claim.
- no vulnerability-free, bug-free, or perfect-crypto claim.
