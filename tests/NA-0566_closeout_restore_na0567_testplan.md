Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-29

# NA-0566 Closeout and NA-0567 Restoration Testplan

This governance closeout testplan records acceptance of D-1122, implementation
PR merge proof, post-merge check proof, exact NA-0567 successor restoration, no
NA-0567 implementation, no probe/remote/qsc action, redaction boundaries, and
the one-READY invariant.

## Required Markers

NA0566_CLOSEOUT_AUTHORIZATION_PR_MERGED_OK
NA0566_CLOSEOUT_D1122_ACCEPTED_OK
NA0566_CLOSEOUT_PUBLIC_SAFETY_GREEN_OK
NA0566_CLOSEOUT_ADVISORIES_GREEN_OK
NA0566_CLOSEOUT_NO_FAILED_REQUIRED_CHECKS_OK
NA0566_CLOSEOUT_D1123_RESTORED_NA0567_OK
NA0566_CLOSEOUT_SELECTED_SUCCESSOR_EXACT_OK
NA0566_CLOSEOUT_NO_NA0567_IMPLEMENTATION_OK
NA0566_CLOSEOUT_NO_PROBES_EXECUTED_OK
NA0566_CLOSEOUT_NO_SSH_TAILSCALE_REMOTE_COMMAND_OK
NA0566_CLOSEOUT_NO_WORKFLOW_DISPATCH_OK
NA0566_CLOSEOUT_NO_RERUN_EXECUTED_OK
NA0566_CLOSEOUT_NO_QSC_SEND_RECEIVE_OK
NA0566_CLOSEOUT_NO_SOURCE_MUTATION_OK
NA0566_CLOSEOUT_NO_SCRIPT_MUTATION_OK
NA0566_CLOSEOUT_NO_WORKFLOW_MUTATION_OK
NA0566_CLOSEOUT_NO_DEPENDENCY_LOCKFILE_MUTATION_OK
NA0566_CLOSEOUT_NO_QSL_SERVER_ATTACHMENTS_OK
NA0566_CLOSEOUT_NO_ACCOUNT_SERVICE_MUTATION_OK
NA0566_CLOSEOUT_NO_PUBLIC_SITE_MUTATION_OK
NA0566_CLOSEOUT_NO_CLOUDFLARE_MUTATION_OK
NA0566_CLOSEOUT_NO_SECRET_VALUES_PUBLISHED_OK
NA0566_CLOSEOUT_NO_PRIVATE_MATERIAL_PUBLISHED_OK
NA0566_CLOSEOUT_NO_PUBLIC_READINESS_CLAIM_OK
NA0566_CLOSEOUT_NO_PRODUCTION_READINESS_CLAIM_OK
NA0566_CLOSEOUT_ONE_READY_INVARIANT_OK

## Evidence Summary

- NA-0566 implementation PR #1405 merged at `64945f366851`.
- D-1122 exists once and is Accepted.
- Post-merge public-safety completed success.
- Post-merge advisories completed success.
- Post-merge suite2-vectors completed success.
- No failed required checks were classified.
- D-1123 restores NA-0567 as the exactly one READY successor using the
  D-1122-selected successor block.
- NA-0566 is marked DONE.

## Boundary Summary

This closeout does not implement NA-0567. It executes no probes, SSH, Tailscale,
remote commands, workflow dispatches, reruns, qsc send/receive, qsc E2EE,
qsl-server/qsl-attachments commands, qwork/qstart/qresume, qsl-backup, or
backup mutation.

This closeout changes no source files, repository scripts, workflow files,
dependencies, lockfiles, qsc source/test/fuzz/Cargo paths, qsl-server paths,
qsl-attachments paths, account/service state, public-site paths, Cloudflare
configuration, raw-log repository docs, raw-artifact repository docs, or private
material.
