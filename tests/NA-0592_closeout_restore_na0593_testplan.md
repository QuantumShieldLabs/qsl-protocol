Status: Supporting
Owner: QSL governance
Last-Updated: 2026-07-02

# NA-0592 closeout / NA-0593 restoration testplan

## Scope

This testplan records the NA-0592 closeout lane. It verifies D-1175 and PR
#1458, marks NA-0592 DONE, records D-1176, and restores exactly one READY
successor: NA-0593. It does not implement NA-0593, does not mutate qsc source or
tests, and does not start full qsl-attachments send/receive integration.

## Required Markers

- NA0592_CLOSEOUT_IMPLEMENTATION_PR_MERGED_OK
- NA0592_CLOSEOUT_D1175_ACCEPTED_OK
- NA0592_CLOSEOUT_PR1458_POSTMERGE_GREEN_OK
- NA0592_CLOSEOUT_PUBLIC_SAFETY_GREEN_OK
- NA0592_CLOSEOUT_ADVISORIES_GREEN_OK
- NA0592_CLOSEOUT_NO_FAILED_REQUIRED_CHECKS_OK
- NA0592_CLOSEOUT_D1176_RESTORED_NA0593_OK
- NA0592_CLOSEOUT_SELECTED_SUCCESSOR_EXACT_OK
- NA0592_CLOSEOUT_NA0592_DONE_OK
- NA0592_CLOSEOUT_NA0593_READY_OK
- NA0592_CLOSEOUT_NO_NA0593_IMPLEMENTATION_OK
- NA0592_CLOSEOUT_NO_QSC_SOURCE_MUTATION_OK
- NA0592_CLOSEOUT_NO_QSL_SERVER_MUTATION_OK
- NA0592_CLOSEOUT_NO_QSL_ATTACHMENTS_INTEGRATION_OK
- NA0592_CLOSEOUT_NO_QSL_ATTACHMENTS_MUTATION_OK
- NA0592_CLOSEOUT_NO_REMOTE_TAILSCALE_WORKFLOW_ACTION_OK
- NA0592_CLOSEOUT_NO_DEPENDENCY_LOCKFILE_MUTATION_OK
- NA0592_CLOSEOUT_NO_PRIVATE_MATERIAL_PUBLISHED_OK
- NA0592_CLOSEOUT_NO_PUBLIC_READINESS_CLAIM_OK
- NA0592_CLOSEOUT_NO_PRODUCTION_READINESS_CLAIM_OK
- NA0592_CLOSEOUT_NO_CRYPTO_COMPLETE_CLAIM_OK
- NA0592_CLOSEOUT_NO_TRIPLE_RATCHET_COMPLETE_CLAIM_OK
- NA0592_CLOSEOUT_NO_SECURITY_COMPLETION_CLAIM_OK
- NA0592_CLOSEOUT_ONE_READY_INVARIANT_OK

## Validation Commands

- `git diff --check`
- closeout scope guard
- queue/decision proof
- marker proof
- markdown link check
- added-line/new-file private-material scan
- overclaim scan
- PR body preflight
- goal-lint
- root `cargo audit`
- nested qsc fuzz `cargo audit`
- `cargo metadata --locked --format-version=1`
- `cargo fmt --check`
- `sh -n scripts/ci/qsc_adversarial.sh`
- `bash -n scripts/ci/qsc_adversarial.sh`

## Expected Result

NA-0592 is DONE, D-1176 exists once, and NA-0593 is the only READY item. No
NA-0593 implementation occurs in this closeout.
