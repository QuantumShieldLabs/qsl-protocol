Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-28

# NA-0557 Remote Relay Environment Non-Secret Operator Proof Capture Testplan

Goals: G1, G2, G3, G4, G5

## Required Markers

- NA0557_D1102_AUTHORIZATION_CONSUMED_OK
- NA0557_D1103_CLOSEOUT_CONSUMED_OK
- NA0557_D476_PARSER_STOP_RECOVERED_OK
- NA0557_FRESH_QWORK_PROOF_OK
- NA0557_CURRENT_MAIN_CHECKS_CLASSIFIED_OK
- NA0557_PUBLIC_SAFETY_GREEN_OK
- NA0557_ADVISORIES_GREEN_OK
- NA0557_NO_FAILED_REQUIRED_CHECKS_OK
- NA0557_OPERATOR_PROOF_PACKAGE_REVIEWED_OK
- NA0557_OPERATOR_PROOF_PRIVATE_MATERIAL_SCAN_OK
- NA0557_OPERATOR_PROOF_FIELDS_CLASSIFIED_OK
- NA0557_UNKNOWN_FIELDS_PRESERVED_OK
- NA0557_ENDPOINT_BOUNDARY_CLASSIFIED_OK
- NA0557_DNS_BOUNDARY_CLASSIFIED_OK
- NA0557_TCP_BOUNDARY_CLASSIFIED_OK
- NA0557_TLS_BOUNDARY_CLASSIFIED_OK
- NA0557_SERVICE_HEALTH_BOUNDARY_CLASSIFIED_OK
- NA0557_AUTH_ROUTE_BOUNDARY_CLASSIFIED_OK
- NA0557_RUNNER_PROOF_BOUNDARY_CLASSIFIED_OK
- NA0557_RESULT_CLASSIFICATION_SELECTED_OK
- NA0557_SUCCESSOR_MODEL_SELECTED_OR_STOP_RECORDED_OK
- NA0557_NO_SECRET_VALUES_ACCESSED_OK
- NA0557_NO_SECRET_VALUES_PUBLISHED_OK
- NA0557_NO_ROUTE_TOKEN_CAPABILITY_PUBLISHED_OK
- NA0557_NO_BEARER_AUTH_PUBLISHED_OK
- NA0557_NO_PRIVATE_ENDPOINT_TOPOLOGY_PUBLISHED_OK
- NA0557_NO_SSH_TAILSCALE_REMOTE_COMMAND_OK
- NA0557_NO_PROBES_EXECUTED_BY_CODEX_OK
- NA0557_NO_RERUN_EXECUTED_OK
- NA0557_NO_WORKFLOW_DISPATCH_OK
- NA0557_NO_LOCAL_REPRODUCTION_OK
- NA0557_NO_SOURCE_MUTATION_OK
- NA0557_NO_SCRIPT_MUTATION_OK
- NA0557_NO_WORKFLOW_MUTATION_OK
- NA0557_NO_DEPENDENCY_LOCKFILE_MUTATION_OK
- NA0557_NO_QSL_SERVER_ATTACHMENTS_OK
- NA0557_NO_PUBLIC_SITE_MUTATION_OK
- NA0557_NO_CLOUDFLARE_MUTATION_OK
- NA0557_NO_RAW_LOGS_COMMITTED_OK
- NA0557_NO_PRIVATE_MATERIAL_PUBLISHED_OK
- NA0557_NO_PUBLIC_READINESS_CLAIM_OK
- NA0557_NO_PRODUCTION_READINESS_CLAIM_OK
- NA0557_ONE_READY_INVARIANT_OK

## Evidence Summary

- D-1102 and D-1103 were consumed and each exists once as Accepted.
- D476 parser stop was consumed and recovered with a proof-root file-backed
  parser.
- Fresh NA-0557 qwork proof from `2026-06-28T16:33:05Z` was verified.
- Current-main public-safety, advisories, suite2-vectors, and required checks
  were classified.
- Operator proof package was reviewed proof-root-only.
- Operator and Codex private-material scans passed.
- Unknown fields and not_checked fields were preserved as known unknowns.
- Endpoint, DNS, TCP, TLS, service-health, auth/route, and runner-specific
  proof boundaries were classified.
- Result classification selected:
  `REMOTE_RELAY_OPERATOR_PROOF_TARGETED_NON_SECRET_PROBE_AUTH_READY`.
- Successor selected:
  `NA-0558 -- QSL Remote Relay Targeted Non-Secret Operator Probe Authorization Plan`.

## Boundaries

- No secret values were accessed or published.
- No route-token/capability values, bearer values, Authorization headers,
  private endpoint hosts, private topology, payloads, response bodies, or secret
  environment values were published.
- No SSH, Tailscale, remote command, probe, rerun, workflow dispatch, local
  reproduction, qsc send/receive, qsl-server/qsl-attachments command, qsl-backup
  execution, backup mutation, public-site mutation, or Cloudflare mutation
  occurred.
- No source, script, workflow, dependency, or lockfile mutation occurred.
- No raw logs or raw artifacts were committed.
- No public-readiness or production-readiness claim is made.
