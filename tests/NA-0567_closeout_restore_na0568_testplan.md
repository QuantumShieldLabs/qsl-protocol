Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-29

# NA-0567 Closeout Restore NA-0568 Testplan

## Purpose

Validate that NA-0567 closes only after the D-1124 authorization PR merged,
post-merge public-safety and advisories completed success, and exactly one
successor, NA-0568, is restored as READY without implementing NA-0568.

## Required Markers

- NA0567_CLOSEOUT_IMPLEMENTATION_PR_MERGED_OK
- NA0567_CLOSEOUT_D1124_ACCEPTED_OK
- NA0567_CLOSEOUT_PUBLIC_SAFETY_GREEN_OK
- NA0567_CLOSEOUT_ADVISORIES_GREEN_OK
- NA0567_CLOSEOUT_D1125_RESTORED_NA0568_OK
- NA0567_CLOSEOUT_NO_NA0568_IMPLEMENTATION_OK
- NA0567_CLOSEOUT_NO_REMOTE_ACTION_OK
- NA0567_CLOSEOUT_NO_QSC_SEND_RECEIVE_OK
- NA0567_CLOSEOUT_NO_SOURCE_MUTATION_OK
- NA0567_CLOSEOUT_NO_ACCOUNT_SERVICE_MUTATION_OK
- NA0567_CLOSEOUT_NO_PUBLIC_SITE_MUTATION_OK
- NA0567_CLOSEOUT_ONE_READY_INVARIANT_OK

## Preconditions

- NA-0567 implementation PR #1407 merged at `b91688db6a14`.
- D-1124 exists exactly once and is Accepted.
- D-1125 is absent before the closeout patch.
- Post-merge public-safety completed success.
- Post-merge advisories completed success.
- No failed required check was classified.
- NA-0567 is the sole READY item before closeout.

## Closeout Scope

Allowed closeout paths:

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0567_closeout_restore_na0568_testplan.md`

Forbidden closeout actions:

- implementing NA-0568;
- SSH, Tailscale, or remote command;
- remote probe or repair;
- qsc send/receive;
- workflow dispatch or rerun;
- source, script, workflow, dependency, lockfile, runtime, qsc, qsl-server, or
  qsl-attachments mutation;
- account, shell, authorized_keys, service, sudo, systemctl, firewall, or
  Tailscale mutation;
- public-site or Cloudflare mutation;
- private material publication.

## Queue Checks

After closeout:

- NA-0567 is DONE.
- NA-0568 is READY.
- READY count is exactly 1.
- No other READY item exists.

## Decision Checks

After closeout:

- D-1124 exists once and is Accepted.
- D-1125 exists once and is Accepted.
- Duplicate decision count is 0.

## Acceptance

Pass only if the closeout patch restores NA-0568 exactly, preserves the
one-READY invariant, and performs no NA-0568 implementation or forbidden remote,
runtime, source, account/service, public-site, Cloudflare, or private-material
action.
