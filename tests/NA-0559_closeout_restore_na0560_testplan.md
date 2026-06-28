Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-28

# NA-0559 Closeout Restore NA-0560 Testplan

Goals: G1, G2, G3, G4, G5

## Scope

This testplan records the closeout-only validation markers for restoring the
D-1108-selected NA-0560 successor after NA-0559 authorization merged. It does
not implement NA-0560 and does not run probes.

## Required Markers

- NA0559_CLOSEOUT_AUTHORIZATION_PR_MERGED_OK
- NA0559_CLOSEOUT_D1108_ACCEPTED_OK
- NA0559_CLOSEOUT_PUBLIC_SAFETY_GREEN_OK
- NA0559_CLOSEOUT_ADVISORIES_GREEN_OK
- NA0559_CLOSEOUT_NO_FAILED_REQUIRED_CHECKS_OK
- NA0559_CLOSEOUT_D1109_RESTORED_NA0560_OK
- NA0559_CLOSEOUT_SELECTED_SUCCESSOR_EXACT_OK
- NA0559_CLOSEOUT_NO_NA0560_IMPLEMENTATION_OK
- NA0559_CLOSEOUT_NO_PROBES_EXECUTED_OK
- NA0559_CLOSEOUT_NO_SSH_TAILSCALE_REMOTE_COMMAND_OK
- NA0559_CLOSEOUT_NO_WORKFLOW_DISPATCH_OK
- NA0559_CLOSEOUT_NO_RERUN_EXECUTED_OK
- NA0559_CLOSEOUT_NO_SOURCE_MUTATION_OK
- NA0559_CLOSEOUT_NO_SCRIPT_MUTATION_OK
- NA0559_CLOSEOUT_NO_WORKFLOW_MUTATION_OK
- NA0559_CLOSEOUT_NO_DEPENDENCY_LOCKFILE_MUTATION_OK
- NA0559_CLOSEOUT_NO_QSL_SERVER_ATTACHMENTS_OK
- NA0559_CLOSEOUT_NO_PUBLIC_SITE_MUTATION_OK
- NA0559_CLOSEOUT_NO_CLOUDFLARE_MUTATION_OK
- NA0559_CLOSEOUT_NO_SECRET_VALUES_PUBLISHED_OK
- NA0559_CLOSEOUT_NO_PRIVATE_MATERIAL_PUBLISHED_OK
- NA0559_CLOSEOUT_NO_PUBLIC_READINESS_CLAIM_OK
- NA0559_CLOSEOUT_NO_PRODUCTION_READINESS_CLAIM_OK
- NA0559_CLOSEOUT_ONE_READY_INVARIANT_OK

## Evidence Expectations

- NA-0559 implementation PR #1391 is merged.
- D-1108 exists once and is Accepted.
- Post-merge public-safety and advisories completed success.
- No failed required check remains.
- D-1109 records NA-0559 closeout and restores NA-0560.
- NA-0559 is DONE.
- READY_COUNT is 1.
- READY is NA-0560.
- D-1110 is absent.
- Duplicate decision count is zero.

## Boundary Assertions

- No NA-0560 implementation occurred.
- No probe execution occurred.
- No SSH, Tailscale, remote command, workflow dispatch, rerun, local
  reproduction, qsc send/receive, qsc E2EE, qsl-server action,
  qsl-attachments action, qwork/qstart/qresume execution, qsl-backup execution,
  backup mutation, public-site mutation, or Cloudflare mutation occurred.
- No source, script, workflow, dependency, lockfile, qsc source/test/fuzz/Cargo,
  qsl-server, qsl-attachments, public, website, README public-progress, or
  docs/public path was mutated.
- No secret values were requested or published.
- No route-token/capability values, bearer values, Authorization headers,
  endpoint values, private topology, payloads, response bodies, secret
  environment values, raw logs, raw artifacts, or private material were
  published.
- No public, production, external-review, backup/restore, security-completion,
  build-perfection, or crypto-perfection overclaim is made.

## Validation Commands

- `git diff --check`
- exact five-path closeout scope guard
- queue/decision proof
- marker proof
- changed Markdown link-check
- private-material scan
- prohibited-material scan
- overclaim scan
- docs/governance-only classifier
- PR body preflight
- goal-lint if available
- root cargo audit
- nested qsc fuzz lock cargo audit
- `cargo fmt --check`
- `sh -n scripts/ci/qsc_adversarial.sh`
- `bash -n scripts/ci/qsc_adversarial.sh`

Focused qsc runtime tests may be skipped because this lane is closeout-only and
makes no qsc source/runtime/dependency/workflow mutation.
