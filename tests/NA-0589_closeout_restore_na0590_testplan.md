Status: Supporting
Owner: QSL governance
Last-Updated: 2026-07-02

# NA-0589 Closeout and NA-0590 Restoration Testplan

## Scope

This closeout marks NA-0589 DONE and restores the exact D-1169-selected NA-0590 recovery-verification successor. It does not implement NA-0590.

## Required Markers

- NA0589_CLOSEOUT_IMPLEMENTATION_PR_MERGED_OK
- NA0589_CLOSEOUT_D1169_ACCEPTED_OK
- NA0589_CLOSEOUT_PUBLIC_SAFETY_GREEN_OK
- NA0589_CLOSEOUT_ADVISORIES_GREEN_OK
- NA0589_CLOSEOUT_NO_FAILED_REQUIRED_CHECKS_OK
- NA0589_CLOSEOUT_D1170_RESTORED_NA0590_OK
- NA0589_CLOSEOUT_SELECTED_SUCCESSOR_EXACT_OK
- NA0589_CLOSEOUT_NO_NA0590_IMPLEMENTATION_OK
- NA0589_CLOSEOUT_NO_QSL_ATTACHMENTS_RUNTIME_OK
- NA0589_CLOSEOUT_NO_REMOTE_ACTION_OK
- NA0589_CLOSEOUT_NO_WORKFLOW_DISPATCH_OR_RERUN_OK
- NA0589_CLOSEOUT_NO_PRIVATE_MATERIAL_PUBLISHED_OK
- NA0589_CLOSEOUT_NO_PUBLIC_READINESS_CLAIM_OK
- NA0589_CLOSEOUT_NO_PRODUCTION_READINESS_CLAIM_OK
- NA0589_CLOSEOUT_NO_SECURITY_COMPLETION_CLAIM_OK
- NA0589_CLOSEOUT_ONE_READY_INVARIANT_OK

## Validation

- Verify implementation PR #1452 merged.
- Verify D-1169 exists once and is Accepted.
- Verify D-1170 is added once.
- Verify NA-0589 is DONE.
- Verify READY_COUNT 1 and READY NA-0590.
- Verify closeout scope is limited to allowed governance/testplan paths.
- Verify no qsl-attachments runtime, qsc send/receive, qsl-server mutation, remote action, Tailscale, workflow dispatch/rerun, or private-material publication occurred.
- Verify no public claim, no production claim, and no security-completion claim is introduced.
- Run scope guard, queue/decision proof, marker proof, link check, private-material scan, overclaim scan, PR body preflight, goal-lint, cargo audits, locked metadata, cargo fmt, and qsc adversarial shell syntax checks.
