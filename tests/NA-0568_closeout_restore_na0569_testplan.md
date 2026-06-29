Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-29

# NA-0568 Closeout Restore NA-0569 Testplan

## Purpose

Validate that NA-0568 closes only after the D-1126 implementation PR merged,
post-merge required gates were healthy, and exactly one successor, NA-0569, is
restored as READY without implementing NA-0569.

## Required Markers

- NA0568_CLOSEOUT_IMPLEMENTATION_PR_MERGED_OK
- NA0568_CLOSEOUT_D1126_ACCEPTED_OK
- NA0568_CLOSEOUT_PUBLIC_SAFETY_GREEN_OK
- NA0568_CLOSEOUT_ADVISORIES_GREEN_OK
- NA0568_CLOSEOUT_D1127_RESTORED_NA0569_OK
- NA0568_CLOSEOUT_NO_NA0569_IMPLEMENTATION_OK
- NA0568_CLOSEOUT_NO_REMOTE_ACTION_OK
- NA0568_CLOSEOUT_NO_QSC_SEND_RECEIVE_OK
- NA0568_CLOSEOUT_NO_WORKFLOW_DISPATCH_OK
- NA0568_CLOSEOUT_NO_SOURCE_MUTATION_OK
- NA0568_CLOSEOUT_NO_PUBLIC_SITE_MUTATION_OK
- NA0568_CLOSEOUT_ONE_READY_INVARIANT_OK

## Preconditions

- NA-0568 implementation PR #1409 merged at `9be00b932806`.
- D-1126 exists exactly once and is Accepted.
- D-1127 is absent before the closeout patch.
- Post-merge public-safety completed success.
- Post-merge advisories completed success.
- No failed branch-protection required check was classified.
- D-1126 selected the exact NA-0569 qsc relay command discovery authorization
  successor.
- NA-0568 is the sole READY item before closeout.

## Closeout Scope

Allowed closeout paths:

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0568_closeout_restore_na0569_testplan.md`

Forbidden closeout actions:

- implementing NA-0569;
- remote action;
- SSH command;
- qsc send/receive;
- qsc E2EE;
- workflow dispatch;
- rerun;
- source, script, workflow, dependency, lockfile, runtime, qsc, qsl-server, or
  qsl-attachments mutation;
- account, shell, authorized_keys, service, sudo, systemctl, firewall, or
  Tailscale mutation;
- public-site or Cloudflare mutation;
- private material publication.

## Queue Checks

After closeout:

- NA-0568 is DONE.
- NA-0569 is READY.
- READY count is exactly 1.
- No other READY item exists.

## Decision Checks

After closeout:

- D-1126 exists once and is Accepted.
- D-1127 exists once and is Accepted.
- D-1128 is absent.
- Duplicate decision count is 0.

## Acceptance

Pass only if the closeout patch restores NA-0569 exactly, preserves the
one-READY invariant, and performs no NA-0569 implementation or forbidden remote,
runtime, source, workflow, public-site, Cloudflare, or private-material action.
