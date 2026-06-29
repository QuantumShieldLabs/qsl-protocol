Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-29

# NA-0567 Remote Relay Codex Recovery Authority Pivot Testplan

## Purpose

Validate that NA-0567 only records an authorization pivot from the accepted
D-1122/D-1123 operator-proof-only lane to the exact NA-0568 Codex-executed
remote recovery successor. This testplan does not authorize NA-0568 execution.

## Required Markers

- NA0567_D1122_CONSUMED_OK
- NA0567_D1123_CONSUMED_OK
- NA0567_FRESH_QWORK_PROOF_OK
- NA0567_CURRENT_MAIN_CHECKS_CLASSIFIED_OK
- NA0567_OPERATOR_INTENT_RECORDED_OK
- NA0567_CODEX_REMOTE_RECOVERY_AUTHORITY_SELECTED_OK
- NA0567_NA0568_COMMAND_ALLOWLIST_SELECTED_OK
- NA0567_NA0568_PRIVATE_MATERIAL_POLICY_SELECTED_OK
- NA0567_NA0568_STOP_CONDITIONS_SELECTED_OK
- NA0567_NO_REMOTE_ACTION_OK
- NA0567_NO_SSH_TAILSCALE_REMOTE_COMMAND_OK
- NA0567_NO_QSC_SEND_RECEIVE_OK
- NA0567_NO_WORKFLOW_DISPATCH_OK
- NA0567_NO_SOURCE_MUTATION_OK
- NA0567_NO_ACCOUNT_SERVICE_MUTATION_OK
- NA0567_NO_PUBLIC_SITE_MUTATION_OK
- NA0567_NO_CLOUDFLARE_MUTATION_OK
- NA0567_NO_SECRET_VALUES_PUBLISHED_OK
- NA0567_NO_PRIVATE_MATERIAL_PUBLISHED_OK
- NA0567_ONE_READY_INVARIANT_OK

## Gate Checks

Fresh qwork proof must be parsed from copied lane proof files. Required values:

- startup result OK;
- lane NA-0567;
- repository qsl-protocol;
- lane path `/srv/qbuild/work/NA-0567/qsl-protocol`;
- HEAD equals origin/main;
- worktree, index, and untracked state clean;
- READY count one;
- queue top READY NA-0567;
- requested lane status READY.

Pre-fetch live state must show a clean worktree, HEAD equal to origin/main,
root disk below the 95 percent stop threshold, and `/backup/qsl` mounted.

After fetch, origin/main must equal or descend from `e26e57f58273`.

## Queue And Decision Checks

Before patch:

- READY count is 1.
- READY item is NA-0567.
- NA-0566 is DONE.
- NA-0565 is DONE.
- D-1122 exists once and is Accepted.
- D-1123 exists once and is Accepted.
- D-1124 is absent.
- D-1125 is absent.
- duplicate decision count is 0.

After patch:

- READY count remains 1.
- READY item remains NA-0567.
- D-1124 exists once.
- D-1125 is absent.
- duplicate decision count remains 0.

## Authority Checks

The evidence and decision must record:

- D-1122 consumed;
- D-1123 consumed;
- operator intent for a future Codex-executed recovery lane;
- `REMOTE_RELAY_CODEX_EXECUTED_RECOVERY_AUTH_READY`;
- exact NA-0568 recovery model;
- exact NA-0568 command allowlist;
- exact NA-0568 private-material policy;
- exact NA-0568 stop conditions;
- exact successor NA-0568 selected.

## Forbidden Action Checks

NA-0567 must not perform:

- SSH;
- Tailscale;
- remote command;
- remote probe;
- qsc send/receive;
- workflow dispatch;
- rerun;
- sudo;
- systemctl;
- service mutation;
- account, shell, or authorized_keys mutation;
- qsl-server or qsl-attachments command;
- source, script, workflow, dependency, or lockfile mutation;
- public-site mutation;
- Cloudflare mutation;
- secret value request or publication;
- private endpoint, private port, private topology, token, bearer value,
  Authorization header, payload, response body, process identity, or private
  material publication.

## Validation Commands

Required local validation before PR:

- `git diff --check`
- exact five-path implementation scope guard
- queue/decision proof
- marker proof
- markdown link check
- added-line/new-file private-material scan
- overclaim scan
- docs/governance-only classifier
- PR body preflight
- goal-lint if available
- root cargo audit
- nested qsc fuzz lock cargo audit
- `cargo fmt --check`
- `sh -n scripts/ci/qsc_adversarial.sh`
- `bash -n scripts/ci/qsc_adversarial.sh`

Focused qsc runtime tests may be skipped because NA-0567 is an
authorization-only governance/evidence/testplan lane and mutates no qsc source,
runtime, dependency, workflow, executable test, fuzz target, or vector.

## Acceptance

Pass only if all markers are present, D-1124 exists exactly once, D-1125 remains
absent, the one-READY invariant is preserved, and no forbidden action or private
material publication occurs.
