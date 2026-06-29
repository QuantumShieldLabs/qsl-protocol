Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-29

# NA-0564 Closeout and NA-0565 Restoration Testplan

This governance closeout testplan records acceptance of D-1118, implementation
PR merge proof, post-merge check proof, exact NA-0565 successor restoration, no
NA-0565 implementation, no operator action execution, redaction boundaries,
no-mutation boundaries, and the one-READY invariant.

## Required Markers

NA0564_CLOSEOUT_IMPLEMENTATION_PR_MERGED_OK
NA0564_CLOSEOUT_D1118_ACCEPTED_OK
NA0564_CLOSEOUT_PUBLIC_SAFETY_GREEN_OK
NA0564_CLOSEOUT_ADVISORIES_GREEN_OK
NA0564_CLOSEOUT_NO_FAILED_REQUIRED_CHECKS_OK
NA0564_CLOSEOUT_D1119_RESTORED_NA0565_OK
NA0564_CLOSEOUT_SELECTED_SUCCESSOR_EXACT_OK
NA0564_CLOSEOUT_NO_NA0565_IMPLEMENTATION_OK
NA0564_CLOSEOUT_NO_OPERATOR_ACTION_EXECUTED_OK
NA0564_CLOSEOUT_NO_SECRET_VALUES_PUBLISHED_OK
NA0564_CLOSEOUT_NO_PRIVATE_MATERIAL_PUBLISHED_OK
NA0564_CLOSEOUT_NO_WORKFLOW_DISPATCH_OK
NA0564_CLOSEOUT_NO_RERUN_EXECUTED_OK
NA0564_CLOSEOUT_NO_SOURCE_MUTATION_OK
NA0564_CLOSEOUT_NO_SCRIPT_MUTATION_OK
NA0564_CLOSEOUT_NO_WORKFLOW_MUTATION_OK
NA0564_CLOSEOUT_NO_DEPENDENCY_LOCKFILE_MUTATION_OK
NA0564_CLOSEOUT_NO_QSL_SERVER_ATTACHMENTS_OK
NA0564_CLOSEOUT_NO_ACCOUNT_SERVICE_MUTATION_OK
NA0564_CLOSEOUT_NO_PUBLIC_SITE_MUTATION_OK
NA0564_CLOSEOUT_NO_CLOUDFLARE_MUTATION_OK
NA0564_CLOSEOUT_NO_PUBLIC_READINESS_CLAIM_OK
NA0564_CLOSEOUT_NO_PRODUCTION_READINESS_CLAIM_OK
NA0564_CLOSEOUT_ONE_READY_INVARIANT_OK

## Evidence Summary

- NA-0564 implementation PR #1401 merged at `91443c1d43f5`.
- D-1118 exists once and is Accepted.
- Post-merge public-safety completed success.
- Post-merge advisories completed success.
- No failed required checks were classified.
- D-1119 restores NA-0565 as the exactly one READY successor using the
  D-1118-selected successor block.

## Boundary Assertions

This closeout does not implement NA-0565. It executes no operator action,
candidate confirmation proof, SSH, Tailscale, remote commands, workflow
dispatches, reruns, qsc send/receive, qsc E2EE, qsl-server commands,
qsl-attachments commands, or service/account actions.

This closeout changes no source files, repository scripts, workflow files,
dependencies, lockfiles, qsc source/test/fuzz/Cargo files, qsl-server paths,
qsl-attachments paths, public-site content, docs/public content, public paths,
website paths, or Cloudflare configuration.

No secret values, endpoint values, private endpoint hosts, private topology,
route-token/capability values, bearer values, Authorization headers, process
identities, raw payloads, response bodies, raw logs, raw artifacts, raw
authorized key material, public key material, or private material are
published.

No public-readiness or production-readiness claim is made.
