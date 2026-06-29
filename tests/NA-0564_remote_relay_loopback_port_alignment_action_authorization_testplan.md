Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-29

# NA-0564 Remote Relay Loopback Port Alignment Action Authorization Testplan

This governance testplan records NA-0564 qwork proof, D-1116/D-1117
inheritance, current-main checks, proof-root-only probe generation, exact SSH
readiness, exact remote loopback probe, private-material review, result
classification, selected successor, no-mutation boundaries, and the one-READY
invariant.

## Required Markers

NA0564_D1116_AUTHORITY_CONSUMED_OK
NA0564_D1117_CLOSEOUT_CONSUMED_OK
NA0564_FRESH_QWORK_PROOF_OK
NA0564_CURRENT_MAIN_CHECKS_CLASSIFIED_OK
NA0564_PUBLIC_SAFETY_GREEN_OK
NA0564_ADVISORIES_GREEN_OK
NA0564_NO_FAILED_REQUIRED_CHECKS_OK
NA0564_ALIGNMENT_PROBE_SCRIPT_GENERATED_OK
NA0564_ALIGNMENT_PROBE_SCRIPT_STATIC_REVIEW_OK
NA0564_SSH_ALIGNMENT_READINESS_CLASSIFIED_OK
NA0564_REMOTE_ALIGNMENT_PROBE_EXECUTED_OR_ACCESS_STOP_OK
NA0564_PRIVATE_MATERIAL_SCAN_OK
NA0564_EXPECTED_TARGET_ALIGNMENT_CLASSIFIED_OK
NA0564_ACTION_OWNER_CLASSIFIED_OK
NA0564_OPERATOR_ACTION_REQUIRED_CLASSIFIED_OK
NA0564_OPERATOR_ACTION_BUNDLE_READY_OR_NOT_REQUIRED_OK
NA0564_RESULT_CLASSIFICATION_SELECTED_OK
NA0564_SUCCESSOR_MODEL_SELECTED_OR_STOP_RECORDED_OK
NA0564_NO_ENDPOINT_VALUE_PUBLISHED_OK
NA0564_NO_PRIVATE_PORT_VALUE_PUBLISHED_OK
NA0564_NO_ROUTE_TOKEN_CAPABILITY_PUBLISHED_OK
NA0564_NO_BEARER_AUTH_PUBLISHED_OK
NA0564_NO_PRIVATE_TOPOLOGY_PUBLISHED_OK
NA0564_NO_PROCESS_IDENTITY_PUBLISHED_OK
NA0564_NO_AUTHORIZED_KEYS_CONTENT_PUBLISHED_OK
NA0564_NO_PAYLOAD_BODY_PUBLISHED_OK
NA0564_NO_ACCOUNT_SERVICE_MUTATION_OK
NA0564_NO_QSC_SEND_RECEIVE_OK
NA0564_NO_WORKFLOW_DISPATCH_OK
NA0564_NO_RERUN_EXECUTED_OK
NA0564_NO_SOURCE_MUTATION_OK
NA0564_NO_SCRIPT_MUTATION_OK
NA0564_NO_WORKFLOW_MUTATION_OK
NA0564_NO_DEPENDENCY_LOCKFILE_MUTATION_OK
NA0564_NO_QSL_SERVER_ATTACHMENTS_OK
NA0564_NO_PUBLIC_SITE_MUTATION_OK
NA0564_NO_CLOUDFLARE_MUTATION_OK
NA0564_NO_PRIVATE_MATERIAL_PUBLISHED_OK
NA0564_NO_PUBLIC_READINESS_CLAIM_OK
NA0564_NO_PRODUCTION_READINESS_CLAIM_OK
NA0564_ONE_READY_INVARIANT_OK

## Evidence Summary

- D-1116 and D-1117 were consumed.
- Fresh qwork proof from `2026-06-29T01:58:42Z` was verified before fetch,
  probes, or repository mutation.
- Current main required checks were classified: public-safety success,
  advisories success, suite2-vectors success, no failed required checks, and no
  pending required checks.
- The proof-root-only alignment probe script was generated and statically
  reviewed.
- SSH alignment readiness was classified `SSH_ALIGNMENT_PROBE_READY`.
- The remote alignment probe executed once through SSH stdin.
- Remote proof classes: expected target listener `unknown`, other loopback
  listener presence `yes`, loopback listener count class `one`, candidate
  listener class `present`, expected target alignment `unknown`, action owner
  `unknown`, operator action required `unknown`.
- Private-material scans passed.
- Result classification was `LOOPBACK_ALIGNMENT_CANDIDATE_PROOF_READY`.
- Selected successor was
  `NA-0565 -- QSL Remote Relay Loopback Candidate Confirmation Proof Harness`.

## Boundary Assertions

NA-0564 executed only the two exact D-1116-authorized SSH commands. It executed
no extra SSH, Tailscale, remote, service, sudo, qsc, workflow dispatch, rerun,
qsl-server, qsl-attachments, qwork, qstart, qresume, qsl-backup, or backup
command.

NA-0564 changed no source files, repository scripts, workflow files,
dependencies, lockfiles, qsc source/test/fuzz/Cargo files, qsl-server paths,
qsl-attachments paths, public-site content, docs/public content, public paths,
website paths, deployment settings, or Cloudflare configuration.

No endpoint values, private port values, private hosts/IPs/topology,
route-token/capability values, bearer values, Authorization headers, process
identities, payloads, response bodies, raw authorized key material, public SSH
key material, private keys, secret environment values, Cloudflare tokens, API
keys, raw logs, raw artifacts, or private material are published.

No public-readiness or production-readiness claim is made.
