Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-27

# NA-0547 Remote/Relay Non-Required CI Failure Bounded Reproduction Log Capture Testplan

## Objective

Validate that NA-0547 consumes D-1082/D-1083, verifies fresh qwork proof and
current-main required-check health, runs only the exact authorized GitHub
Actions rerun/dispatch commands, captures proof-root-only logs, publishes only
redacted summaries, classifies each target, selects a successor, and preserves
workflow, runtime, qsc, dependency, qsl-server, qsl-attachments, public-site,
Cloudflare, raw-log, private-material, and no-claim boundaries.

## Required Markers

- `NA0547_D1082_REPRO_AUTH_CONSUMED_OK`
- `NA0547_D1083_CLOSEOUT_CONSUMED_OK`
- `NA0547_FRESH_QWORK_PROOF_OK`
- `NA0547_CURRENT_MAIN_CHECKS_CLASSIFIED_OK`
- `NA0547_PUBLIC_SAFETY_GREEN_OK`
- `NA0547_ADVISORIES_GREEN_OK`
- `NA0547_NO_FAILED_REQUIRED_CHECKS_OK`
- `NA0547_REPRODUCTION_COMMAND_GATE_OK`
- `NA0547_HISTORICAL_RERUNS_EXECUTED_OR_STOP_RECORDED_OK`
- `NA0547_CURRENT_MAIN_DISPATCHES_EXECUTED_OR_STOP_RECORDED_OK`
- `NA0547_REMOTE_HANDSHAKE_CLASSIFIED_OK`
- `NA0547_REMOTE_RELAY_CLASSIFIED_OK`
- `NA0547_RELAY_UI_CLASSIFIED_OK`
- `NA0547_RAW_LOGS_PROOF_ROOT_ONLY_OK`
- `NA0547_PRIVATE_MATERIAL_SCAN_OK`
- `NA0547_REDACTED_SUMMARIES_OK`
- `NA0547_OVERALL_CLASSIFICATION_SELECTED_OK`
- `NA0547_SUCCESSOR_MODEL_SELECTED_OR_STOP_RECORDED_OK`
- `NA0547_NO_LOCAL_REPRODUCTION_EXECUTED_OK`
- `NA0547_NO_WORKFLOW_MUTATION_OK`
- `NA0547_NO_RUNTIME_MUTATION_OK`
- `NA0547_NO_QSC_SOURCE_MUTATION_OK`
- `NA0547_NO_DEPENDENCY_LOCKFILE_MUTATION_OK`
- `NA0547_NO_QSL_SERVER_ATTACHMENTS_MUTATION_OK`
- `NA0547_NO_PUBLIC_SITE_MUTATION_OK`
- `NA0547_NO_CLOUDFLARE_MUTATION_OK`
- `NA0547_NO_RAW_LOGS_COMMITTED_OK`
- `NA0547_NO_PRIVATE_MATERIAL_PUBLISHED_OK`
- `NA0547_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0547_NO_PRODUCTION_READINESS_CLAIM_OK`
- `NA0547_ONE_READY_INVARIANT_OK`

## Expected Reproduction Result

Per-target classifications:

- remote-handshake: `REMOTE_HANDSHAKE_REPRODUCED_CURRENT`
- remote-relay: `REMOTE_RELAY_REPRODUCED_CURRENT`
- relay-ui-integration: `RELAY_UI_REPRODUCED_CURRENT`

Overall classification:

`REMOTE_RELAY_REPRODUCTION_PARTIAL_MORE_EVIDENCE_REQUIRED`

Selected successor:

`NA-0548 -- QSL Remote/Relay Non-Required CI Failure Follow-Up Evidence Authorization Plan`

## Scope Guard

Allowed implementation paths:

- `docs/governance/evidence/NA-0547_remote_relay_non_required_ci_failure_bounded_reproduction_log_capture_harness.md`
- `tests/NA-0547_remote_relay_non_required_ci_failure_bounded_reproduction_log_capture_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Any workflow, runtime/source, qsc source/test/fuzz/Cargo, dependency/lockfile,
qsl-server, qsl-attachments, public-site, Cloudflare, backup, or operator-local
system mutation fails this testplan.

## Validation

Validation must prove:

- fresh qwork proof matches live pre-fetch repo state;
- public-safety and advisories completed success;
- no failed required checks;
- exact D-1082 commands were used;
- D-1084 exists once after patch;
- D-1085 is absent before closeout;
- duplicate decision count is zero;
- all required markers are present;
- changed files are limited to the allowed implementation set;
- raw logs remain proof-root-only;
- redacted extracts pass private-material scan;
- added lines and new files pass private-material and overclaim scans;
- no raw logs are committed;
- focused qsc runtime tests are skipped only because NA-0547 used GitHub
  Actions reproduction evidence only and did not mutate runtime/source,
  dependency, qsc, or workflow paths.
