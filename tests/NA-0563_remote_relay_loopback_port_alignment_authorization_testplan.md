Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-29

# NA-0563 Remote Relay Loopback Port Alignment Authorization Testplan

This governance testplan records NA-0563 authorization-only evidence,
D-1114/D-1115 inheritance, NA-0562 listener proof consumption, loopback
alignment boundary selection, exact NA-0564 successor design, redaction
boundary, result classification, and the one-READY invariant.

## Required Markers

NA0563_D1114_LISTENER_PROOF_CONSUMED_OK
NA0563_D1115_CLOSEOUT_CONSUMED_OK
NA0563_FRESH_QWORK_PROOF_OK
NA0563_CURRENT_MAIN_CHECKS_CLASSIFIED_OK
NA0563_PUBLIC_SAFETY_GREEN_OK
NA0563_ADVISORIES_GREEN_OK
NA0563_NO_FAILED_REQUIRED_CHECKS_OK
NA0563_EXPECTED_LISTENER_ABSENT_CONSUMED_OK
NA0563_OTHER_LOOPBACK_LISTENER_PRESENT_CONSUMED_OK
NA0563_TCP_REFUSED_CONSUMED_OK
NA0563_LOOPBACK_ALIGNMENT_BOUNDARY_SELECTED_OK
NA0563_NA0564_MODEL_SELECTED_OK
NA0563_NA0564_COMMAND_ACTION_ALLOWLIST_SELECTED_OK
NA0563_NA0564_PROOF_SCHEMA_SELECTED_OK
NA0563_NA0564_PRIVATE_MATERIAL_POLICY_SELECTED_OK
NA0563_NA0564_DECISION_TREE_SELECTED_OK
NA0563_RESULT_CLASSIFICATION_SELECTED_OK
NA0563_SUCCESSOR_MODEL_SELECTED_OR_STOP_RECORDED_OK
NA0563_NO_PROBES_EXECUTED_OK
NA0563_NO_SSH_TAILSCALE_REMOTE_COMMAND_OK
NA0563_NO_WORKFLOW_DISPATCH_OK
NA0563_NO_RERUN_EXECUTED_OK
NA0563_NO_QSC_SEND_RECEIVE_OK
NA0563_NO_SOURCE_MUTATION_OK
NA0563_NO_SCRIPT_MUTATION_OK
NA0563_NO_WORKFLOW_MUTATION_OK
NA0563_NO_DEPENDENCY_LOCKFILE_MUTATION_OK
NA0563_NO_QSL_SERVER_ATTACHMENTS_OK
NA0563_NO_ACCOUNT_SERVICE_MUTATION_OK
NA0563_NO_PUBLIC_SITE_MUTATION_OK
NA0563_NO_CLOUDFLARE_MUTATION_OK
NA0563_NO_SECRET_VALUES_REQUESTED_OK
NA0563_NO_SECRET_VALUES_PUBLISHED_OK
NA0563_NO_PRIVATE_ENDPOINT_TOPOLOGY_PUBLISHED_OK
NA0563_NO_PROCESS_IDENTITY_PUBLISHED_OK
NA0563_NO_RAW_PAYLOAD_BODY_PUBLISHED_OK
NA0563_NO_PUBLIC_READINESS_CLAIM_OK
NA0563_NO_PRODUCTION_READINESS_CLAIM_OK
NA0563_ONE_READY_INVARIANT_OK

## Evidence Summary

- D-1114 and D-1115 were consumed.
- Fresh qwork proof from `2026-06-29T00:18:11Z` or later was verified before
  repository mutation.
- Current main required checks were classified: public-safety success,
  advisories success, suite2-vectors success or conclusively satisfied, and no
  failed required checks.
- NA-0562 listener proof was consumed: expected listener absent, other loopback
  listener present, TCP refused, and v1 push HEAD not checked.
- Loopback alignment was selected as the primary next boundary.
- NA-0564 Model C was selected: Codex-executed non-mutating proof plus
  operator-owned action bundle.
- NA-0564 command/action allowlist, proof schema, private-material policy, and
  decision tree were selected.
- Result classification was
  `REMOTE_RELAY_LOOPBACK_ALIGNMENT_ACTION_AUTH_READY`.
- Selected successor was
  `NA-0564 -- QSL Remote Relay Loopback Port Alignment Action Authorization Harness`.

## Boundary Assertions

NA-0563 executed no probes, SSH, Tailscale, remote commands, workflow dispatches,
reruns, qsc send/receive, qsc E2EE, service commands, qsl-server commands, or
qsl-attachments commands.

NA-0563 changed no source files, repository scripts, workflow files,
dependencies, lockfiles, qsc source/test/fuzz/Cargo files, qsl-server paths,
qsl-attachments paths, public-site content, docs/public content, public paths,
website paths, or Cloudflare configuration.

NA-0563 requested and published no secret values, endpoint values, private
endpoint hosts, private topology, route-token/capability values, bearer values,
Authorization headers, process identities, raw payloads, response bodies,
authorized_keys material, public SSH key material, raw logs, raw artifacts, or
private material.

No public-readiness or production-readiness claim is made.
