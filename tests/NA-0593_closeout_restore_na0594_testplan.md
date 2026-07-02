Status: Supporting
Owner: QSL governance
Last-Updated: 2026-07-02

# NA-0593 closeout / NA-0594 restoration testplan

## Scope

This testplan records the NA-0593 closeout lane. It verifies D524, PR #1460,
and D-1177 inheritance, accepts result classification
`SEED_FALLBACK_HARDENING_IMPLEMENTATION_PASS_ATTACHMENT_DEFERRED`, marks
NA-0593 DONE, records D-1178, and restores exactly one READY successor:
NA-0594. It does not implement NA-0594, does not mutate qsc source or tests,
does not mutate qsl-server, and does not run or mutate qsl-attachments.

## Required Markers

- NA0593_CLOSEOUT_D524_CONSUMED_OK
- NA0593_CLOSEOUT_D1177_ACCEPTED_OK
- NA0593_CLOSEOUT_PR1460_POSTMERGE_GREEN_OK
- NA0593_CLOSEOUT_PUBLIC_SAFETY_GREEN_OK
- NA0593_CLOSEOUT_ADVISORIES_GREEN_OK
- NA0593_CLOSEOUT_NO_FAILED_REQUIRED_CHECKS_OK
- NA0593_CLOSEOUT_RESULT_CLASSIFICATION_ACCEPTED_OK
- NA0593_CLOSEOUT_D1178_RESTORED_NA0594_OK
- NA0593_CLOSEOUT_NA0593_DONE_OK
- NA0593_CLOSEOUT_NO_NA0594_IMPLEMENTATION_OK
- NA0593_CLOSEOUT_NO_QSC_SOURCE_MUTATION_OK
- NA0593_CLOSEOUT_NO_QSL_SERVER_MUTATION_OK
- NA0593_CLOSEOUT_NO_QSL_ATTACHMENTS_INTEGRATION_OK
- NA0593_CLOSEOUT_NO_REMOTE_TAILSCALE_WORKFLOW_ACTION_OK
- NA0593_CLOSEOUT_NO_DEPENDENCY_LOCKFILE_MUTATION_OK
- NA0593_CLOSEOUT_NO_PUBLIC_READINESS_CLAIM_OK
- NA0593_CLOSEOUT_NO_PRODUCTION_READINESS_CLAIM_OK
- NA0593_CLOSEOUT_NO_CRYPTO_COMPLETE_CLAIM_OK
- NA0593_CLOSEOUT_NO_TRIPLE_RATCHET_COMPLETE_OVERCLAIM_OK
- NA0593_CLOSEOUT_NO_ATTACHMENT_COMPLETE_CLAIM_OK
- NA0593_CLOSEOUT_ONE_READY_INVARIANT_OK

## Validation Commands

- `git diff --check`
- exact five-path closeout scope guard
- queue/decision proof
- marker proof
- markdown link check
- added-line/new-file private-material scan
- secret/prohibited-material scan
- overclaim scan
- crypto/triple-ratchet/attachment claim-boundary scan
- docs/governance-only classifier
- PR body preflight
- goal-lint
- root `cargo audit`
- nested qsc fuzz `cargo audit`
- `cargo metadata --locked --format-version=1`
- `cargo fmt --check`
- `sh -n scripts/ci/qsc_adversarial.sh`
- `bash -n scripts/ci/qsc_adversarial.sh`

Focused qsc runtime tests may be skipped because this closeout mutates only
governance and testplan files, does not mutate qsc source or tests, does not
mutate dependencies or workflows, and does not implement NA-0594.

## Expected Result

NA-0593 is DONE, D-1178 exists once, and NA-0594 is the only READY item. No
NA-0594 implementation occurs in this closeout. No qsc source/test mutation,
qsl-server mutation, qsl-attachments mutation/integration, remote action,
Tailscale action, workflow dispatch/rerun, dependency mutation, lockfile
mutation, private-material publication, no public-readiness claim, no
production-readiness claim, no crypto-complete claim, no triple-ratchet-complete
overclaim, and no attachment-complete claim occurs.
