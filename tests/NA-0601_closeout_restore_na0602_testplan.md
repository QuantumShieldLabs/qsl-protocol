# NA-0601 Closeout and NA-0602 Restoration Testplan

Goals: G1, G2, G3, G4, G5

This testplan records closeout-only validation for NA-0601. It does not
authorize NA-0602 implementation, LAN runtime tests, remote/Tailnet action,
workflow mutation, source mutation, dependency mutation, lockfile mutation, or
private-material publication.

## Required Markers

- `NA0601_CLOSEOUT_IMPLEMENTATION_PR_MERGED_OK`
- `NA0601_CLOSEOUT_D1193_ACCEPTED_OK`
- `NA0601_CLOSEOUT_PR1476_POSTMERGE_GREEN_OK`
- `NA0601_CLOSEOUT_PUBLIC_SAFETY_GREEN_OK`
- `NA0601_CLOSEOUT_ADVISORIES_GREEN_OK`
- `NA0601_CLOSEOUT_NO_FAILED_REQUIRED_CHECKS_OK`
- `NA0601_CLOSEOUT_D1194_RESTORED_NA0602_OK`
- `NA0601_CLOSEOUT_SELECTED_SUCCESSOR_EXACT_OK`
- `NA0601_CLOSEOUT_NA0601_DONE_OK`
- `NA0601_CLOSEOUT_NA0602_READY_OK`
- `NA0601_CLOSEOUT_NO_NA0602_IMPLEMENTATION_OK`
- `NA0601_CLOSEOUT_NO_LAN_RUNTIME_TEST_OK`
- `NA0601_CLOSEOUT_NO_REMOTE_TAILSCALE_WORKFLOW_ACTION_OK`
- `NA0601_CLOSEOUT_NO_SOURCE_WORKFLOW_DEPENDENCY_LOCKFILE_MUTATION_OK`
- `NA0601_CLOSEOUT_NO_PRIVATE_MATERIAL_PUBLISHED_OK`
- `NA0601_CLOSEOUT_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0601_CLOSEOUT_NO_PRODUCTION_READINESS_CLAIM_OK`
- `NA0601_CLOSEOUT_NO_REMOTE_READY_CLAIM_OK`
- `NA0601_CLOSEOUT_NO_TAILNET_READY_CLAIM_OK`
- `NA0601_CLOSEOUT_NO_LAN_READY_OVERCLAIM_OK`
- `NA0601_CLOSEOUT_ONE_READY_INVARIANT_OK`

## Validation Plan

- Verify implementation PR #1476 merged.
- Verify D-1193 exists once and is Accepted.
- Verify PR #1476 post-merge public-safety and advisories completed success
  with no failed required checks.
- Verify D-1194 exists once after closeout patch and D-1195 is absent.
- Verify NA-0601 is DONE.
- Verify READY_COUNT 1 with READY NA-0602.
- Verify selected successor text matches the D-1193 successor.
- Verify closeout changed only allowed closeout paths.
- Verify no NA-0602 implementation occurred.
- Verify no LAN runtime test, remote action, Tailscale action, workflow
  dispatch/rerun, workflow mutation, source mutation, dependency mutation,
  lockfile mutation, deployment mutation, public-site mutation, DNS mutation, or
  Cloudflare mutation occurred.
- Verify no endpoint values, private port values, hostnames, topology details
  beyond classes, tokens, Authorization values, capabilities, payloads,
  plaintext, ciphertext bodies, seeds, key material, raw logs, raw artifacts, or
  private material are published.
- Verify no public-readiness, production-readiness, remote-ready, Tailnet-ready,
  LAN-ready overclaim, vulnerability-free, bug-free, crypto-complete, or
  attachment-complete claim is introduced.
