Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-24

# NA-0536 Closeout and NA-0537 Restoration Testplan

## Objective

Record the closeout validation for NA-0536 after authorization PR #1345 merged and post-merge public-safety/advisories were green inside the short attach/early-failure window.

## Required gates

- PR #1345 merged at `749008231762`.
- Post-merge public-safety completed success.
- Post-merge advisories completed success.
- D-1062 exists once.
- D-1063 is absent before closeout patch and exists once after patch.
- NA-0536 is marked DONE.
- NA-0537 is restored READY using the D-1062 selected successor block.
- READY_COUNT is exactly 1 after patch.
- No NA-0537 implementation is started.

## Required local validation

- `git diff --check`
- exact five-path closeout scope guard
- queue/decision proof
- link-check
- leak-scan
- overclaim scan
- classifier
- PR body preflight
- root `cargo audit --deny warnings`
- nested qsc fuzz lock `cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock`
- `cargo fmt --check`
- `sh -n scripts/ci/qsc_adversarial.sh`
- `bash -n scripts/ci/qsc_adversarial.sh`

## Scope boundary

Allowed closeout paths:

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0536_closeout_restore_na0537_testplan.md`

Forbidden in closeout:

- remote action
- SSH execution
- qsc E2EE
- qsc send/receive
- qsc protocol commands
- qsl-server or qsl-attachments use
- qsc source/test/fuzz/Cargo mutation
- workflow/script/helper mutation
- dependency/lockfile mutation
- corpus/vector/input mutation
- formal/refimpl/service/public/backup mutation
- qwork/qstart/qresume
- qsl-backup
- no public/production/security-completion claims

## Claim boundary

This closeout restores the next READY item only. It does not implement NA-0537 and does not make a public-readiness, production-readiness, public-internet-readiness, external-review-complete, crypto-complete, identity-complete, trust-complete, replay-proof, downgrade-proof, secret-material-complete, side-channel-free, vulnerability-free, bug-free, or perfect-crypto claim.
