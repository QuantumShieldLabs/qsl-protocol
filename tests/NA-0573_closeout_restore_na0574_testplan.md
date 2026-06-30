Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-30

# NA-0573 Closeout and NA-0574 Restoration Testplan

## Purpose

Record deterministic acceptance markers for closing NA-0573 after D-1136 and
restoring the exact D-1136-selected NA-0574 successor. This closeout does not
implement NA-0574.

## Required Markers

- NA0573_CLOSEOUT_IMPLEMENTATION_PR_MERGED_OK
- NA0573_CLOSEOUT_D1136_ACCEPTED_OK
- NA0573_CLOSEOUT_PUBLIC_SAFETY_GREEN_OK
- NA0573_CLOSEOUT_ADVISORIES_GREEN_OK
- NA0573_CLOSEOUT_NO_FAILED_REQUIRED_CHECKS_OK
- NA0573_CLOSEOUT_D1137_RESTORED_NA0574_OK
- NA0573_CLOSEOUT_SELECTED_SUCCESSOR_EXACT_OK
- NA0573_CLOSEOUT_NO_NA0574_IMPLEMENTATION_OK
- NA0573_CLOSEOUT_NO_REMOTE_ACTION_OK
- NA0573_CLOSEOUT_NO_SSH_SCP_SUDO_SYSTEMCTL_TAILSCALE_OK
- NA0573_CLOSEOUT_NO_ACCOUNT_AUTHORIZED_KEYS_MUTATION_OK
- NA0573_CLOSEOUT_NO_QSC_SEND_RECEIVE_OK
- NA0573_CLOSEOUT_NO_WORKFLOW_DISPATCH_OK
- NA0573_CLOSEOUT_NO_RERUN_EXECUTED_OK
- NA0573_CLOSEOUT_NO_QSL_SERVER_SOURCE_MUTATION_OK
- NA0573_CLOSEOUT_NO_QSL_ATTACHMENTS_OK
- NA0573_CLOSEOUT_NO_SOURCE_SCRIPT_WORKFLOW_DEPENDENCY_MUTATION_OK
- NA0573_CLOSEOUT_NO_PUBLIC_SITE_MUTATION_OK
- NA0573_CLOSEOUT_NO_CLOUDFLARE_MUTATION_OK
- NA0573_CLOSEOUT_NO_PRIVATE_MATERIAL_PUBLISHED_OK
- NA0573_CLOSEOUT_NO_PUBLIC_READINESS_CLAIM_OK
- NA0573_CLOSEOUT_NO_PRODUCTION_READINESS_CLAIM_OK
- NA0573_CLOSEOUT_ONE_READY_INVARIANT_OK

## Evidence Mapping

- Implementation merge proof maps to PR #1419 and D-1136.
- Required-check proof maps to post-merge public-safety, advisories,
  suite2-vectors, and required-check classification artifacts.
- Successor proof maps to the D-1136-selected NA-0574 start/bind operator proof
  authorization block restored in `NEXT_ACTIONS.md`.
- Boundary markers map to the closeout scope guard, private-material scan,
  overclaim scan, and repository diff proof.

## Expected Result

NA-0573 is DONE, NA-0574 is the sole READY item, D-1137 exists once, D-1138 is
absent, duplicate decision IDs remain zero, and no NA-0574 implementation occurs
during closeout.
