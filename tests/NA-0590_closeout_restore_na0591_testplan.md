Status: Supporting
Owner: QSL governance
Last-Updated: 2026-07-02

# NA-0590 Closeout and NA-0591 Restoration Testplan

## Scope

This closeout records implementation PR #1454 merge proof, marks NA-0590 DONE, and restores exactly one D-1171-selected NA-0591 successor. It does not implement NA-0591 and does not run qsl-attachments send/receive integration.

## Required Markers

- NA0590_CLOSEOUT_IMPLEMENTATION_PR_MERGED_OK
- NA0590_CLOSEOUT_D1171_ACCEPTED_OK
- NA0590_CLOSEOUT_PUBLIC_SAFETY_GREEN_OK
- NA0590_CLOSEOUT_ADVISORIES_GREEN_OK
- NA0590_CLOSEOUT_NO_FAILED_REQUIRED_CHECKS_OK
- NA0590_CLOSEOUT_D1172_RESTORED_NA0591_OK
- NA0590_CLOSEOUT_SELECTED_SUCCESSOR_EXACT_OK
- NA0590_CLOSEOUT_NO_NA0591_IMPLEMENTATION_OK
- NA0590_CLOSEOUT_NO_QSL_ATTACHMENTS_INTEGRATION_OK
- NA0590_CLOSEOUT_NO_QSL_ATTACHMENTS_MUTATION_OK
- NA0590_CLOSEOUT_NO_QSL_SERVER_MUTATION_OK
- NA0590_CLOSEOUT_NO_REMOTE_ACTION_OK
- NA0590_CLOSEOUT_NO_WORKFLOW_DISPATCH_OR_RERUN_OK
- NA0590_CLOSEOUT_NO_PRIVATE_MATERIAL_PUBLISHED_OK
- NA0590_CLOSEOUT_NO_PUBLIC_READINESS_CLAIM_OK
- NA0590_CLOSEOUT_NO_PRODUCTION_READINESS_CLAIM_OK
- NA0590_CLOSEOUT_NO_TRIPLE_RATCHET_COMPLETE_CLAIM_OK
- NA0590_CLOSEOUT_NO_SECURITY_COMPLETION_CLAIM_OK
- NA0590_CLOSEOUT_ONE_READY_INVARIANT_OK

## Evidence Requirements

- Verify implementation PR #1454 merged with merge commit `b20d31ec2b59`.
- Verify D-1171 exists once and is Accepted before closeout.
- Verify D-1172 is absent before the closeout patch.
- Verify post-merge public-safety and advisories completed success with no failed required checks.
- Mark NA-0590 DONE.
- Restore exactly one READY successor: `NA-0591 -- QSL Local qsc True Triple-Ratchet E2EE Path Verification Harness`.
- Preserve no NA-0591 implementation, no qsl-attachments send/receive integration, no qsl-attachments mutation, no qsl-server mutation, no remote/Tailscale/workflow action, and no private-material publication.

## Validation Commands

- `git diff --check`
- closeout scope guard
- queue/decision proof
- marker proof
- markdown link check
- added-line/private-material scan
- overclaim and triple-ratchet claim-boundary scan
- PR body preflight
- goal-lint when available
- `cargo audit --deny warnings`
- nested qsc fuzz `cargo audit --deny warnings`
- `cargo metadata --locked --format-version=1`
- `cargo fmt --check`

## Expected Result

NA-0590 is DONE, D-1172 is recorded, and NA-0591 is restored as the sole READY item without implementing NA-0591.
