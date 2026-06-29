Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-29

# NA-0569 Closeout Restore NA-0570 Testplan

## Purpose

Validate that NA-0569 closes only after D491 implementation PR #1411 merged,
D-1128 was accepted, post-merge gates and dependency-health checks were green,
and exactly one successor, NA-0570, is restored READY without implementing
NA-0570.

## Required Markers

- NA0569_CLOSEOUT_IMPLEMENTATION_PR_MERGED_OK
- NA0569_CLOSEOUT_D1128_ACCEPTED_OK
- NA0569_CLOSEOUT_PUBLIC_SAFETY_GREEN_OK
- NA0569_CLOSEOUT_ADVISORIES_GREEN_OK
- NA0569_CLOSEOUT_ROOT_CARGO_AUDIT_GREEN_OK
- NA0569_CLOSEOUT_NESTED_QSC_FUZZ_AUDIT_GREEN_OK
- NA0569_CLOSEOUT_D1129_RESTORED_NA0570_OK
- NA0569_CLOSEOUT_NO_NA0570_IMPLEMENTATION_OK
- NA0569_CLOSEOUT_NO_REMOTE_ACTION_OK
- NA0569_CLOSEOUT_NO_QSC_COMMAND_EXECUTION_OK
- NA0569_CLOSEOUT_NO_QSL_SERVER_COMMAND_EXECUTION_OK
- NA0569_CLOSEOUT_NO_QSL_ATTACHMENTS_COMMAND_EXECUTION_OK
- NA0569_CLOSEOUT_NO_WORKFLOW_DISPATCH_OK
- NA0569_CLOSEOUT_NO_SOURCE_MUTATION_OK
- NA0569_CLOSEOUT_NO_DEPENDENCY_LOCKFILE_MUTATION_OK
- NA0569_CLOSEOUT_NO_ACCOUNT_SERVICE_MUTATION_OK
- NA0569_CLOSEOUT_NO_PUBLIC_SITE_MUTATION_OK
- NA0569_CLOSEOUT_NO_PRIVATE_MATERIAL_PUBLISHED_OK
- NA0569_CLOSEOUT_NO_PUBLIC_READINESS_CLAIM_OK
- NA0569_CLOSEOUT_NO_PRODUCTION_READINESS_CLAIM_OK
- NA0569_CLOSEOUT_ONE_READY_INVARIANT_OK

## Preconditions

- D491 implementation PR #1411 merged at `bcbb5f7a66d1`.
- D-1128 exists exactly once and is Accepted.
- D-1129 is absent before the closeout patch.
- Post-merge public-safety completed success.
- Post-merge advisories completed success.
- No failed branch-protection required check was classified.
- Root `cargo audit --deny warnings` completed success after D491.
- Nested qsc fuzz cargo audit completed success after D491.
- NA-0569 is the sole READY item before closeout.

## Closeout Scope

Allowed closeout paths:

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0569_closeout_restore_na0570_testplan.md`

Forbidden closeout actions:

- implementing NA-0570;
- remote action;
- SSH, Tailscale, or remote command;
- qsc command execution;
- qsl-server command execution;
- qsl-attachments command execution;
- workflow dispatch or rerun;
- source, script, workflow, dependency, lockfile, runtime, qsc, qsl-server, or
  qsl-attachments mutation;
- account, shell, authorized_keys, service, sudo, systemctl, firewall, or
  Tailscale mutation;
- public-site or Cloudflare mutation;
- private material publication;
- public-readiness, production-readiness, vulnerability-free, bug-free, or
  perfect-build claim.

## Queue Checks

After closeout:

- NA-0569 is DONE.
- NA-0570 is READY.
- READY count is exactly 1.
- No other READY item exists.

## Decision Checks

After closeout:

- D-1128 exists once and is Accepted.
- D-1129 exists once and is Accepted.
- D-1130 is absent.
- Duplicate decision count is 0.

## Acceptance

Pass only if the closeout patch restores NA-0570 exactly, preserves the
one-READY invariant, and performs no NA-0570 implementation or forbidden remote,
runtime, qsc, qsl-server, qsl-attachments, workflow, source, dependency,
public-site, Cloudflare, account/service, private-material, or overclaim action.
