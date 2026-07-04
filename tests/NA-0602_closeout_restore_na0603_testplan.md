# NA-0602 Closeout and NA-0603 Restoration Testplan

Goals: G1, G2, G3, G4, G5

This testplan records closeout-only validation for NA-0602. It does not
authorize NA-0603 implementation, LAN runtime tests, qsc LAN commands,
qsl-server startup, qsl-attachments runtime, remote/Tailnet action, workflow
mutation, source mutation, dependency mutation, lockfile mutation, or
private-material publication.

## Required Markers

- `NA0602_CLOSEOUT_IMPLEMENTATION_PR_MERGED_OK`
- `NA0602_CLOSEOUT_D1195_ACCEPTED_OK`
- `NA0602_CLOSEOUT_PR1478_POSTMERGE_GREEN_OK`
- `NA0602_CLOSEOUT_PUBLIC_SAFETY_GREEN_OK`
- `NA0602_CLOSEOUT_ADVISORIES_GREEN_OK`
- `NA0602_CLOSEOUT_NO_FAILED_REQUIRED_CHECKS_OK`
- `NA0602_CLOSEOUT_D1196_RESTORED_NA0603_OK`
- `NA0602_CLOSEOUT_SELECTED_SUCCESSOR_EXACT_OK`
- `NA0602_CLOSEOUT_NA0602_DONE_OK`
- `NA0602_CLOSEOUT_NA0603_READY_OK`
- `NA0602_CLOSEOUT_NO_NA0603_IMPLEMENTATION_OK`
- `NA0602_CLOSEOUT_NO_LAN_RUNTIME_TEST_OK`
- `NA0602_CLOSEOUT_NO_QSC_LAN_COMMAND_OK`
- `NA0602_CLOSEOUT_NO_QSL_SERVER_STARTUP_OK`
- `NA0602_CLOSEOUT_NO_QSL_ATTACHMENTS_RUNTIME_OK`
- `NA0602_CLOSEOUT_NO_CODEX_SSH_TO_LAPTOP_OK`
- `NA0602_CLOSEOUT_NO_LAPTOP_SSH_SERVER_SETUP_OK`
- `NA0602_CLOSEOUT_NO_SECOND_CODEX_ON_LAPTOP_OK`
- `NA0602_CLOSEOUT_NO_REMOTE_TAILSCALE_WORKFLOW_ACTION_OK`
- `NA0602_CLOSEOUT_NO_SOURCE_WORKFLOW_DEPENDENCY_LOCKFILE_MUTATION_OK`
- `NA0602_CLOSEOUT_NO_PUBLIC_SITE_CLOUDFLARE_MUTATION_OK`
- `NA0602_CLOSEOUT_NO_PRIVATE_MATERIAL_PUBLISHED_OK`
- `NA0602_CLOSEOUT_NO_SECRET_VALUE_ACCESS_OK`
- `NA0602_CLOSEOUT_NO_ENDPOINT_VALUE_PUBLISHED_OK`
- `NA0602_CLOSEOUT_NO_PRIVATE_PORT_VALUE_PUBLISHED_OK`
- `NA0602_CLOSEOUT_NO_TOKEN_OR_AUTHORIZATION_PUBLISHED_OK`
- `NA0602_CLOSEOUT_NO_CAPABILITY_VALUE_PUBLISHED_OK`
- `NA0602_CLOSEOUT_NO_PAYLOAD_BODY_PLAINTEXT_PUBLISHED_OK`
- `NA0602_CLOSEOUT_NO_SEED_OR_KEY_MATERIAL_PUBLISHED_OK`
- `NA0602_CLOSEOUT_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0602_CLOSEOUT_NO_PRODUCTION_READINESS_CLAIM_OK`
- `NA0602_CLOSEOUT_NO_REMOTE_READY_CLAIM_OK`
- `NA0602_CLOSEOUT_NO_TAILNET_READY_CLAIM_OK`
- `NA0602_CLOSEOUT_NO_LAN_READY_OVERCLAIM_OK`
- `NA0602_CLOSEOUT_NO_CRYPTO_COMPLETE_CLAIM_OK`
- `NA0602_CLOSEOUT_NO_ATTACHMENT_COMPLETE_CLAIM_OK`
- `NA0602_CLOSEOUT_ONE_READY_INVARIANT_OK`

## Validation Plan

- Verify implementation PR #1478 merged.
- Verify D-1195 exists once and is Accepted.
- Verify PR #1478 post-merge public-safety and advisories completed success
  with no failed required checks.
- Verify D-1196 exists once after closeout patch and D-1197 is absent.
- Verify NA-0602 is DONE.
- Verify READY_COUNT 1 with READY NA-0603.
- Verify selected successor text matches the D-1195 successor.
- Verify closeout changed only allowed closeout paths.
- Verify no NA-0603 implementation occurred.
- Verify no LAN runtime test, qsc LAN command, qsl-server startup,
  qsl-attachments runtime, Codex SSH to laptop, laptop SSH server setup, second
  Codex on laptop, remote action, Tailscale action, workflow dispatch/rerun,
  workflow mutation, source mutation, dependency mutation, lockfile mutation,
  deployment mutation, public-site mutation, DNS mutation, or Cloudflare
  mutation occurred.
- Verify no endpoint values, private port values, hostnames, topology details
  beyond classes, tokens, Authorization values, capabilities, payloads,
  plaintext, ciphertext bodies, seeds, key material, raw logs, raw artifacts, or
  private material are published.
- Verify no public-readiness claim is introduced.
- Verify no production-readiness claim is introduced.
- Verify no remote-ready claim is introduced.
- Verify no Tailnet-ready claim is introduced.
- Verify no LAN-ready overclaim is introduced.
- Verify no vulnerability-free claim is introduced.
- Verify no bug-free claim is introduced.
- Verify no crypto-complete claim is introduced.
- Verify no attachment-complete claim is introduced.
