Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-28

# NA-0562 Closeout and NA-0563 Restoration Testplan

This governance testplan records NA-0562 implementation merge acceptance and
restores NA-0563 as the exact D-1114-selected READY successor without
implementing NA-0563.

## Required Markers

NA0562_CLOSEOUT_IMPLEMENTATION_PR_MERGED_OK
NA0562_CLOSEOUT_D1114_ACCEPTED_OK
NA0562_CLOSEOUT_PUBLIC_SAFETY_GREEN_OK
NA0562_CLOSEOUT_ADVISORIES_GREEN_OK
NA0562_CLOSEOUT_NO_FAILED_REQUIRED_CHECKS_OK
NA0562_CLOSEOUT_D1115_RESTORED_NA0563_OK
NA0562_CLOSEOUT_SELECTED_SUCCESSOR_EXACT_OK
NA0562_CLOSEOUT_NO_NA0563_IMPLEMENTATION_OK
NA0562_CLOSEOUT_NO_SECRET_VALUES_PUBLISHED_OK
NA0562_CLOSEOUT_NO_PRIVATE_MATERIAL_PUBLISHED_OK
NA0562_CLOSEOUT_NO_WORKFLOW_DISPATCH_OK
NA0562_CLOSEOUT_NO_RERUN_EXECUTED_OK
NA0562_CLOSEOUT_NO_SOURCE_MUTATION_OK
NA0562_CLOSEOUT_NO_SCRIPT_MUTATION_OK
NA0562_CLOSEOUT_NO_WORKFLOW_MUTATION_OK
NA0562_CLOSEOUT_NO_DEPENDENCY_LOCKFILE_MUTATION_OK
NA0562_CLOSEOUT_NO_QSL_SERVER_ATTACHMENTS_OK
NA0562_CLOSEOUT_NO_ACCOUNT_SERVICE_MUTATION_OK
NA0562_CLOSEOUT_NO_PUBLIC_SITE_MUTATION_OK
NA0562_CLOSEOUT_NO_CLOUDFLARE_MUTATION_OK
NA0562_CLOSEOUT_NO_PUBLIC_READINESS_CLAIM_OK
NA0562_CLOSEOUT_NO_PRODUCTION_READINESS_CLAIM_OK
NA0562_CLOSEOUT_ONE_READY_INVARIANT_OK

## Evidence Summary

- NA-0562 implementation PR #1397 merged at `39248060df9c`.
- D-1114 exists once and is Accepted.
- Post-merge public-safety and advisories completed success.
- No failed required checks were classified.
- NA-0562 is marked DONE.
- NA-0563 is restored READY using the exact D-1114-selected loopback port
  alignment authorization successor block.
- No NA-0563 implementation occurred.

## Boundary Assertions

No secret values, endpoint values, private topology, token values, bearer values,
Authorization headers, payloads, response bodies, process identity, raw
authorized_keys content, public key material, or private material were
published.

No workflow dispatch, rerun, source mutation, repository script mutation,
workflow mutation, dependency or lockfile mutation, qsl-server/qsl-attachments
mutation, account/service mutation, public-site mutation, or Cloudflare mutation
occurred.

No public-readiness or production-readiness claim is made.
