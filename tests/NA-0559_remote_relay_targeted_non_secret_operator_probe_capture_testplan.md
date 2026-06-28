Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-28

# NA-0559 Remote Relay Targeted Non-Secret Operator Probe Capture Testplan

Goals: G1, G2, G3, G4, G5

## Scope

This testplan records the authorization-only validation markers for NA-0559.
The lane pivots the next successor from operator-run probing to Codex-executed,
non-secret probing under strict command, redaction, and stop rules. NA-0559 does
not run probes and does not mutate source, scripts, workflows, dependencies,
public-site content, Cloudflare configuration, qsl-server, or qsl-attachments.

## Required Markers

- NA0559_D1106_OPERATOR_PROBE_AUTH_CONSUMED_OK
- NA0559_D1107_CLOSEOUT_CONSUMED_OK
- NA0559_FRESH_QWORK_PROOF_OK
- NA0559_CURRENT_MAIN_CHECKS_CLASSIFIED_OK
- NA0559_PUBLIC_SAFETY_GREEN_OK
- NA0559_ADVISORIES_GREEN_OK
- NA0559_NO_FAILED_REQUIRED_CHECKS_OK
- NA0559_OPERATOR_CODEX_PROBE_REQUEST_RECORDED_OK
- NA0559_CODEX_EXECUTED_PROBE_AUTHORITY_SELECTED_OK
- NA0559_INSPIRON_CONTEXT_RECORDED_OK
- NA0559_NA0560_EXACT_COMMAND_ALLOWLIST_SELECTED_OK
- NA0559_NA0560_PRIVATE_MATERIAL_POLICY_SELECTED_OK
- NA0559_RESULT_CLASSIFICATION_SELECTED_OK
- NA0559_SUCCESSOR_MODEL_SELECTED_OR_STOP_RECORDED_OK
- NA0559_NO_PROBES_EXECUTED_OK
- NA0559_NO_SSH_TAILSCALE_REMOTE_COMMAND_OK
- NA0559_NO_WORKFLOW_DISPATCH_OK
- NA0559_NO_RERUN_EXECUTED_OK
- NA0559_NO_LOCAL_REPRODUCTION_OK
- NA0559_NO_QSC_SEND_RECEIVE_OK
- NA0559_NO_SOURCE_MUTATION_OK
- NA0559_NO_SCRIPT_MUTATION_OK
- NA0559_NO_WORKFLOW_MUTATION_OK
- NA0559_NO_DEPENDENCY_LOCKFILE_MUTATION_OK
- NA0559_NO_QSL_SERVER_ATTACHMENTS_OK
- NA0559_NO_PUBLIC_SITE_MUTATION_OK
- NA0559_NO_CLOUDFLARE_MUTATION_OK
- NA0559_NO_SECRET_VALUES_REQUESTED_OK
- NA0559_NO_SECRET_VALUES_PUBLISHED_OK
- NA0559_NO_PRIVATE_ENDPOINT_TOPOLOGY_PUBLISHED_OK
- NA0559_NO_RAW_PAYLOAD_BODY_PUBLISHED_OK
- NA0559_NO_PUBLIC_READINESS_CLAIM_OK
- NA0559_NO_PRODUCTION_READINESS_CLAIM_OK
- NA0559_ONE_READY_INVARIANT_OK

## Evidence Expectations

- D-1106 and D-1107 exist once and are Accepted.
- Fresh qwork proof from `2026-06-28T19:38:46Z` or later is verified from
  copied proof files only.
- Current main required checks are classified from file-backed GitHub REST JSON.
- public-safety and advisories are completed success.
- No failed required checks remain.
- Operator request for Codex-executed probing is recorded.
- `inspiron` / `qslcodex` context is recorded without probing.
- Codex-executed probe authority is selected for NA-0560 only.
- The exact NA-0560 command allowlist is selected.
- The NA-0560 private-material policy is selected.
- Result classification is
  `REMOTE_RELAY_CODEX_EXECUTED_NON_SECRET_PROBE_AUTH_READY`.
- Selected successor is `NA-0560 -- QSL Remote Relay Codex-Executed
  Non-Secret Probe Harness`.

## Boundary Assertions

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
  private endpoint values, private topology, payloads, response bodies, secret
  environment values, raw logs, raw artifacts, or private material were
  published.
- No public, production, external-review, backup/restore, security-completion,
  build-perfection, or crypto-perfection overclaim is made.

## Validation Commands

- `git diff --check`
- exact five-path implementation scope guard
- queue/decision proof
- marker proof
- changed Markdown link-check
- added-line/new-file private-material scan
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

Focused qsc runtime tests may be skipped because this lane is authorization-only
and makes no qsc source/runtime/dependency/workflow mutation.
