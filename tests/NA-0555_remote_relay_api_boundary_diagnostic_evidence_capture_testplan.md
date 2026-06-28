Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-28

# NA-0555 Remote Relay API Boundary Diagnostic Evidence Capture Testplan

Goals: G1, G2, G3, G4, G5

## Required Markers

- NA0555_D1098_DIAGNOSTIC_IMPLEMENTATION_CONSUMED_OK
- NA0555_D1099_CLOSEOUT_CONSUMED_OK
- NA0555_FRESH_QWORK_PROOF_OK
- NA0555_CURRENT_MAIN_CHECKS_CLASSIFIED_OK
- NA0555_PUBLIC_SAFETY_GREEN_OK
- NA0555_ADVISORIES_GREEN_OK
- NA0555_NO_FAILED_REQUIRED_CHECKS_OK
- NA0555_DISPATCH_GATE_OK
- NA0555_REMOTE_HANDSHAKE_DISPATCH_EXECUTED_OK
- NA0555_REMOTE_RELAY_DISPATCH_EXECUTED_OK
- NA0555_RUN_JOB_METADATA_CAPTURED_OK
- NA0555_RAW_LOGS_PROOF_ROOT_ONLY_OK
- NA0555_DIAGNOSTIC_EXTRACTS_CAPTURED_OK
- NA0555_PRIVATE_MATERIAL_SCAN_OK
- NA0555_REMOTE_HANDSHAKE_CLASSIFIED_OK
- NA0555_REMOTE_RELAY_CLASSIFIED_OK
- NA0555_RESULT_CLASSIFICATION_SELECTED_OK
- NA0555_SUCCESSOR_MODEL_SELECTED_OR_STOP_RECORDED_OK
- NA0555_NO_RERUN_EXECUTED_OK
- NA0555_NO_SOURCE_MUTATION_OK
- NA0555_NO_SCRIPT_MUTATION_OK
- NA0555_NO_WORKFLOW_MUTATION_OK
- NA0555_NO_DEPENDENCY_LOCKFILE_MUTATION_OK
- NA0555_NO_QSL_SERVER_ATTACHMENTS_OK
- NA0555_NO_PUBLIC_SITE_MUTATION_OK
- NA0555_NO_CLOUDFLARE_MUTATION_OK
- NA0555_NO_RAW_LOGS_COMMITTED_OK
- NA0555_NO_PRIVATE_MATERIAL_PUBLISHED_OK
- NA0555_NO_PUBLIC_READINESS_CLAIM_OK
- NA0555_NO_PRODUCTION_READINESS_CLAIM_OK
- NA0555_ONE_READY_INVARIANT_OK

## Evidence Summary

- D-1098 and D-1099 were consumed and each exists once as Accepted.
- Fresh NA-0555 qwork proof from `2026-06-28T14:03:22Z` was verified.
- Current-main public-safety, advisories, and required checks were classified.
- The two exact authorized workflow dispatches executed once.
- remote-handshake run `28325075419` / job `83913585385` was captured.
- remote-relay run `28325168201` / job `83913828473` was captured.
- Raw logs and artifacts remain proof-root-only.
- Both targets classified as network/TLS timeout at the remote endpoint boundary.
- Overall result selected the remote relay environment and secret-boundary
  review authorization successor.

## Boundaries

- No rerun was executed.
- No source, script, workflow, dependency, or lockfile mutation occurred.
- No qsl-server or qsl-attachments command or mutation occurred.
- No public-site or Cloudflare mutation occurred.
- No raw logs or artifacts were committed.
- No private material was published.
- No public-readiness or production-readiness claim is made.
