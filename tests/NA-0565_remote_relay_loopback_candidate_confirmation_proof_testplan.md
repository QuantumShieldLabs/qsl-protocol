Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-29

# NA-0565 Remote Relay Loopback Candidate Confirmation Proof Testplan

This governance testplan records NA-0565 qwork proof, D-1118/D-1119
inheritance, current-main checks, proof-root-only probe generation, exact SSH
readiness, exact remote candidate confirmation probe, private-material review,
result classification, selected successor, no-mutation boundaries, and the
one-READY invariant.

## Required Markers

NA0565_D1118_AUTHORITY_CONSUMED_OK
NA0565_D1119_CLOSEOUT_CONSUMED_OK
NA0565_FRESH_QWORK_PROOF_OK
NA0565_CURRENT_MAIN_CHECKS_CLASSIFIED_OK
NA0565_PUBLIC_SAFETY_GREEN_OK
NA0565_ADVISORIES_GREEN_OK
NA0565_NO_FAILED_REQUIRED_CHECKS_OK
NA0565_CANDIDATE_PROBE_SCRIPT_GENERATED_OK
NA0565_CANDIDATE_PROBE_SCRIPT_STATIC_REVIEW_OK
NA0565_SSH_CANDIDATE_READINESS_CLASSIFIED_OK
NA0565_REMOTE_CANDIDATE_PROBE_EXECUTED_OR_ACCESS_STOP_OK
NA0565_PRIVATE_MATERIAL_SCAN_OK
NA0565_CANDIDATE_LISTENER_COUNT_CLASSIFIED_OK
NA0565_CANDIDATE_TCP_CLASSIFIED_OK
NA0565_CANDIDATE_RELAY_SHAPE_CLASSIFIED_OK
NA0565_CANDIDATE_CONFIRMATION_CLASSIFIED_OK
NA0565_RESULT_CLASSIFICATION_SELECTED_OK
NA0565_SUCCESSOR_MODEL_SELECTED_OR_STOP_RECORDED_OK
NA0565_NO_ENDPOINT_VALUE_PUBLISHED_OK
NA0565_NO_PRIVATE_PORT_VALUE_PUBLISHED_OK
NA0565_NO_ROUTE_TOKEN_CAPABILITY_PUBLISHED_OK
NA0565_NO_BEARER_AUTH_PUBLISHED_OK
NA0565_NO_PRIVATE_TOPOLOGY_PUBLISHED_OK
NA0565_NO_PROCESS_IDENTITY_PUBLISHED_OK
NA0565_NO_AUTHORIZED_KEYS_CONTENT_PUBLISHED_OK
NA0565_NO_PAYLOAD_BODY_PUBLISHED_OK
NA0565_NO_ACCOUNT_SERVICE_MUTATION_OK
NA0565_NO_QSC_SEND_RECEIVE_OK
NA0565_NO_WORKFLOW_DISPATCH_OK
NA0565_NO_RERUN_EXECUTED_OK
NA0565_NO_SOURCE_MUTATION_OK
NA0565_NO_SCRIPT_MUTATION_OK
NA0565_NO_WORKFLOW_MUTATION_OK
NA0565_NO_DEPENDENCY_LOCKFILE_MUTATION_OK
NA0565_NO_QSL_SERVER_ATTACHMENTS_OK
NA0565_NO_PUBLIC_SITE_MUTATION_OK
NA0565_NO_CLOUDFLARE_MUTATION_OK
NA0565_NO_PRIVATE_MATERIAL_PUBLISHED_OK
NA0565_NO_PUBLIC_READINESS_CLAIM_OK
NA0565_NO_PRODUCTION_READINESS_CLAIM_OK
NA0565_ONE_READY_INVARIANT_OK

## Evidence Summary

- D-1118 and D-1119 were consumed.
- Fresh qwork proof from `2026-06-29T02:58:54Z` was verified before fetch,
  probes, or repository mutation.
- Current main required checks were classified: public-safety success,
  advisories success, suite2-vectors success, no failed required checks, and no
  pending required checks.
- The proof-root-only candidate confirmation probe script was generated and
  statically reviewed.
- SSH candidate readiness was classified
  `SSH_CANDIDATE_CONFIRMATION_READY`.
- The remote candidate confirmation probe executed once through SSH stdin.
- Remote proof classes: candidate listener count class `none`, candidate
  listener class `absent`, candidate TCP connect class `not_checked`,
  candidate v1 push HEAD class `not_checked`, candidate v1 pull HEAD class
  `not_checked`, candidate relay shape class `unknown`, candidate confirmation
  class `unknown`, expected target alignment class `unknown`, action owner
  `unknown`, and operator action required `unknown`.
- Private-material scans passed.
- Result classification was
  `LOOPBACK_CANDIDATE_MISSING_SERVICE_DEPLOYMENT_PROOF_REQUIRED`.
- Selected successor was
  `NA-0566 -- QSL Remote Relay Listener Deployment Proof Authorization Plan`.

## Boundary Assertions

NA-0565 executed only the two exact authorized SSH commands. It executed no
extra SSH, Tailscale, remote, service, sudo, qsc, workflow dispatch, rerun,
qsl-server, qsl-attachments, qwork, qstart, qresume, qsl-backup, or backup
command.

NA-0565 changed no source files, repository scripts, workflow files,
dependencies, lockfiles, qsc source/test/fuzz/Cargo files, qsl-server paths,
qsl-attachments paths, public-site content, docs/public content, public paths,
website paths, deployment settings, or Cloudflare configuration.

No endpoint values, private port values, private hosts/IPs/topology,
route-token/capability values, bearer values, Authorization headers, process
identities, payloads, response bodies, raw authorized key material, public SSH
key material, private keys, secret environment values, Cloudflare tokens, API
keys, raw logs, raw artifacts, or private material are published.

No public-readiness or production-readiness claim is made.
