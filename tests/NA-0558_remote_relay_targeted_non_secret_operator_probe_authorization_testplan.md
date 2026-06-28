Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-28

# NA-0558 Remote Relay Targeted Non-Secret Operator Probe Authorization Testplan

Goals: G1, G2, G3, G4, G5

## Scope

This testplan records the authorization-only validation markers for NA-0558.
The lane designs future operator-run non-secret probe proof but does not run
probes or mutate source, scripts, workflows, dependencies, public-site content,
Cloudflare configuration, qsl-server, or qsl-attachments.

## Required Markers

- NA0558_D1104_OPERATOR_PROOF_CONSUMED_OK
- NA0558_D1105_CLOSEOUT_CONSUMED_OK
- NA0558_FRESH_QWORK_PROOF_OK
- NA0558_CURRENT_MAIN_CHECKS_CLASSIFIED_OK
- NA0558_PUBLIC_SAFETY_GREEN_OK
- NA0558_ADVISORIES_GREEN_OK
- NA0558_NO_FAILED_REQUIRED_CHECKS_OK
- NA0558_PRIOR_EVIDENCE_GAP_REVIEWED_OK
- NA0558_TARGETED_PROBE_DESIGN_SELECTED_OK
- NA0558_EXACT_OPERATOR_COMMAND_DESIGN_SELECTED_OK
- NA0558_FUTURE_PROOF_SCHEMA_SELECTED_OK
- NA0558_PRIVATE_MATERIAL_POLICY_SELECTED_OK
- NA0558_RESULT_CLASSIFICATION_SELECTED_OK
- NA0558_SUCCESSOR_MODEL_SELECTED_OR_STOP_RECORDED_OK
- NA0558_NO_PROBES_EXECUTED_OK
- NA0558_NO_SSH_TAILSCALE_REMOTE_COMMAND_OK
- NA0558_NO_WORKFLOW_DISPATCH_OK
- NA0558_NO_RERUN_EXECUTED_OK
- NA0558_NO_LOCAL_REPRODUCTION_OK
- NA0558_NO_QSC_SEND_RECEIVE_OK
- NA0558_NO_SOURCE_MUTATION_OK
- NA0558_NO_SCRIPT_MUTATION_OK
- NA0558_NO_WORKFLOW_MUTATION_OK
- NA0558_NO_DEPENDENCY_LOCKFILE_MUTATION_OK
- NA0558_NO_QSL_SERVER_ATTACHMENTS_OK
- NA0558_NO_PUBLIC_SITE_MUTATION_OK
- NA0558_NO_CLOUDFLARE_MUTATION_OK
- NA0558_NO_SECRET_VALUES_REQUESTED_OK
- NA0558_NO_SECRET_VALUES_PUBLISHED_OK
- NA0558_NO_PRIVATE_ENDPOINT_TOPOLOGY_PUBLISHED_OK
- NA0558_NO_RAW_PAYLOAD_BODY_PUBLISHED_OK
- NA0558_NO_PUBLIC_READINESS_CLAIM_OK
- NA0558_NO_PRODUCTION_READINESS_CLAIM_OK
- NA0558_ONE_READY_INVARIANT_OK

## Evidence Expectations

- D-1104 and D-1105 exist once and are Accepted.
- Fresh qwork proof from `2026-06-28T17:33:39Z` or later is verified from
  copied proof files only.
- Current main required checks are classified from file-backed GitHub REST JSON.
- public-safety and advisories are completed success.
- No failed required checks remain.
- Prior NA-0555/NA-0556/NA-0557 evidence gaps are reviewed.
- Targeted non-secret probe design, exact future operator command design,
  future proof schema, and private-material policy are selected.
- Result classification is
  `REMOTE_RELAY_TARGETED_NON_SECRET_OPERATOR_PROBE_CAPTURE_READY`.
- Selected successor is `NA-0559 -- QSL Remote Relay Targeted Non-Secret
  Operator Probe Capture Harness`.

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
  private endpoints, private topology, payloads, response bodies, secret
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
