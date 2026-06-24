Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-06-24

# NA-0531 closeout and NA-0532 restoration testplan

Goals: G1, G2, G3, G4, G5

## Purpose

Record closeout validation for NA-0531 after implementation PR #1335 merged and post-merge public-safety/advisories completed success. This closeout marks NA-0531 DONE and restores the D-1052-selected NA-0532 retry successor without implementing NA-0532.

## Expected queue result

- NA-0531 is DONE.
- NA-0532 is READY.
- READY_COUNT remains 1.
- NA-0532 title is `QSL Remote qsc E2EE Wrong-Peer / Stale-Trust Retry After Trigger Remediation Implementation Harness`.

## Expected decision result

- D-1052 exists once.
- D-1053 exists once after patch.
- Duplicate decision count is 0.

## Scope guard

Allowed changed paths:

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0531_closeout_restore_na0532_testplan.md`

No implementation, qsc source/test/fuzz/Cargo, workflow/script/helper, dependency/lockfile, corpus/vector/input, formal/refimpl/service/public/backup, qsl-server, qsl-attachments, qwork/qstart/qresume, or qsl-backup path may change.

## Boundary assertions

This closeout performs no remote action, SSH execution, qsc E2EE, qsc send/receive, qsc protocol command, qsl-server use, qsl-attachments use, remote file write, package install, remote source checkout/build, qwork/qstart/qresume execution, or qsl-backup execution.

No public-readiness, production-readiness, public-internet-readiness, crypto-complete, identity-complete, trust-complete, replay-proof, downgrade-proof, secret-material-complete, side-channel-free, vulnerability-free, bug-free, or perfect-crypto claim is introduced.

## Validation

Expected validation:

- `git diff --check`
- exact five-path closeout scope guard
- queue/decision parser
- link-check
- leak-scan
- overclaim scan
- PR body preflight
- goal-lint after PR creation
- required PR checks green before merge
- post-merge public-safety green
