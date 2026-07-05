# NA-0607 Closeout Restore NA-0608 Test Plan

Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-07-05

Goals: G1, G2, G3, G4, G5

## Scope

This closeout records PR #1488 merge evidence, marks NA-0607 DONE, and restores
the exact D-1205-selected NA-0608 successor. It does not implement NA-0608 and
does not authorize source, workflow, dependency, lockfile, Tailnet, public-site,
deployment, qscwork runtime, or private-material publication changes.

## Required Markers

- NA0607_CLOSEOUT_IMPLEMENTATION_PR_MERGED_OK
- NA0607_CLOSEOUT_D1205_ACCEPTED_OK
- NA0607_CLOSEOUT_PR1488_POSTMERGE_GREEN_OK
- NA0607_CLOSEOUT_PUBLIC_SAFETY_GREEN_OK
- NA0607_CLOSEOUT_ADVISORIES_GREEN_OK
- NA0607_CLOSEOUT_SUITE2_VECTORS_GREEN_OK
- NA0607_CLOSEOUT_QSC_ADVERSARIAL_SMOKE_GREEN_OK
- NA0607_CLOSEOUT_CODEQL_GREEN_OK
- NA0607_CLOSEOUT_NO_FAILED_REQUIRED_CHECKS_OK
- NA0607_CLOSEOUT_D1206_RESTORED_NA0608_OK
- NA0607_CLOSEOUT_SELECTED_SUCCESSOR_EXACT_OK
- NA0607_CLOSEOUT_NA0607_DONE_OK
- NA0607_CLOSEOUT_NA0608_READY_OK
- NA0607_CLOSEOUT_NO_NA0608_IMPLEMENTATION_OK
- NA0607_CLOSEOUT_NO_QSC_SOURCE_MUTATION_OK
- NA0607_CLOSEOUT_NO_QSL_SERVER_SOURCE_MUTATION_OK
- NA0607_CLOSEOUT_NO_QSL_ATTACHMENTS_SOURCE_MUTATION_OK
- NA0607_CLOSEOUT_NO_DEPENDENCY_LOCKFILE_MUTATION_OK
- NA0607_CLOSEOUT_NO_QSCWORK_RUNTIME_OK
- NA0607_CLOSEOUT_NO_REMOTE_TAILSCALE_WORKFLOW_ACTION_OK
- NA0607_CLOSEOUT_NO_PUBLIC_SITE_CLOUDFLARE_MUTATION_OK
- NA0607_CLOSEOUT_NO_PRIVATE_MATERIAL_PUBLISHED_OK
- NA0607_CLOSEOUT_NO_PUBLIC_READINESS_CLAIM_OK
- NA0607_CLOSEOUT_NO_PRODUCTION_READINESS_CLAIM_OK
- NA0607_CLOSEOUT_NO_REMOTE_READY_CLAIM_OK
- NA0607_CLOSEOUT_NO_TAILNET_READY_CLAIM_OK
- NA0607_CLOSEOUT_NO_LAN_READY_OVERCLAIM_OK
- NA0607_CLOSEOUT_NO_CRYPTO_COMPLETE_CLAIM_OK
- NA0607_CLOSEOUT_NO_ATTACHMENT_COMPLETE_CLAIM_OK
- NA0607_CLOSEOUT_ONE_READY_INVARIANT_OK

## Validation

- implementation PR merge proof
- local main equals origin/main proof
- clean worktree proof
- D-1205 once and Accepted
- D-1206 once after patch
- D-1207 absent
- READY_COUNT 1
- NA-0607 DONE
- READY NA-0608
- public-safety success
- advisories success
- suite2-vectors success
- qsc-adversarial-smoke success
- CodeQL success
- no failed required checks
- exact closeout scope guard
- marker proof
- link check
- private-material scan
- overclaim scan
- PR body preflight and goal-lint if available
- root cargo audit
- nested qsc fuzz cargo audit
- cargo metadata --locked
- cargo fmt --check
- qsc adversarial shell syntax checks
