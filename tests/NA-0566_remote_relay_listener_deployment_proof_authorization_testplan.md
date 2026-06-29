Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-29

# NA-0566 Remote Relay Listener Deployment Proof Authorization Testplan

This governance testplan records NA-0566 qwork proof, D-1120/D-1121
inheritance, current-main checks, NA-0565 missing-candidate evidence review,
deployment/listener proof-boundary selection, NA-0567 operator proof model,
private-material policy, decision tree, result classification, selected
successor, no-mutation boundaries, and the one-READY invariant.

## Required Markers

NA0566_D1120_CANDIDATE_PROOF_CONSUMED_OK
NA0566_D1121_CLOSEOUT_CONSUMED_OK
NA0566_FRESH_QWORK_PROOF_OK
NA0566_CURRENT_MAIN_CHECKS_CLASSIFIED_OK
NA0566_PUBLIC_SAFETY_GREEN_OK
NA0566_ADVISORIES_GREEN_OK
NA0566_NO_FAILED_REQUIRED_CHECKS_OK
NA0566_CANDIDATE_MISSING_CONSUMED_OK
NA0566_DEPLOYMENT_LISTENER_BOUNDARY_SELECTED_OK
NA0566_NA0567_PROOF_MODEL_SELECTED_OK
NA0566_NA0567_PROOF_REQUIREMENTS_SELECTED_OK
NA0566_NA0567_PRIVATE_MATERIAL_POLICY_SELECTED_OK
NA0566_NA0567_DECISION_TREE_SELECTED_OK
NA0566_RESULT_CLASSIFICATION_SELECTED_OK
NA0566_SUCCESSOR_MODEL_SELECTED_OR_STOP_RECORDED_OK
NA0566_NO_PROBES_EXECUTED_OK
NA0566_NO_SSH_TAILSCALE_REMOTE_COMMAND_OK
NA0566_NO_WORKFLOW_DISPATCH_OK
NA0566_NO_RERUN_EXECUTED_OK
NA0566_NO_QSC_SEND_RECEIVE_OK
NA0566_NO_SOURCE_MUTATION_OK
NA0566_NO_SCRIPT_MUTATION_OK
NA0566_NO_WORKFLOW_MUTATION_OK
NA0566_NO_DEPENDENCY_LOCKFILE_MUTATION_OK
NA0566_NO_QSL_SERVER_ATTACHMENTS_OK
NA0566_NO_ACCOUNT_SERVICE_MUTATION_OK
NA0566_NO_PUBLIC_SITE_MUTATION_OK
NA0566_NO_CLOUDFLARE_MUTATION_OK
NA0566_NO_SECRET_VALUES_REQUESTED_OK
NA0566_NO_SECRET_VALUES_PUBLISHED_OK
NA0566_NO_PRIVATE_ENDPOINT_TOPOLOGY_PUBLISHED_OK
NA0566_NO_PRIVATE_PORT_VALUES_PUBLISHED_OK
NA0566_NO_PROCESS_IDENTITY_PUBLISHED_OK
NA0566_NO_SERVICE_NAME_PUBLISHED_UNLESS_PUBLIC_SAFE_OK
NA0566_NO_RAW_PAYLOAD_BODY_PUBLISHED_OK
NA0566_NO_PUBLIC_READINESS_CLAIM_OK
NA0566_NO_PRODUCTION_READINESS_CLAIM_OK
NA0566_ONE_READY_INVARIANT_OK

## Evidence Summary

- D-1120 was consumed and accepted once.
- D-1121 was consumed and accepted once.
- Fresh NA-0566 qwork proof was verified from copied proof files before fetch or
  repository mutation.
- Current main at `0de8707462b4` had public-safety success, advisories success,
  suite2-vectors success, no failed required checks, no pending required
  checks, and branch-protection required contexts green or conclusively
  satisfied.
- NA-0565 missing-candidate classification was consumed:
  `LOOPBACK_CANDIDATE_MISSING_SERVICE_DEPLOYMENT_PROOF_REQUIRED`.
- NA-0567 proof model selected: operator/service-owner proof capture only.
- NA-0567 proof requirements, private-material policy, and decision tree were
  selected.
- NA-0566 result classification:
  `REMOTE_RELAY_LISTENER_DEPLOYMENT_OPERATOR_PROOF_READY`.
- Selected successor:
  `NA-0567 -- QSL Remote Relay Listener Deployment Non-Secret Operator Proof Capture Harness`.

## Boundary Summary

NA-0566 executes no probes, SSH, Tailscale, remote commands, workflow dispatches,
reruns, qsc send/receive, qsc E2EE, qsl-server/qsl-attachments commands, or
operator/service mutations. It changes no source files, scripts, workflow files,
dependencies, lockfiles, qsc source/test/fuzz/Cargo paths, qsl-server paths,
qsl-attachments paths, public-site paths, or Cloudflare configuration.

NA-0566 requests and publishes no secret values, endpoint values, private port
values, route-token/capability values, bearer values, Authorization headers,
private topology, process identities, service names, payloads, response bodies,
secret environment values, authorized_keys material, public SSH key material,
private keys, raw logs, raw artifacts, or private material.
