Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-27

# NA-0545 Remote/Relay Non-Required CI Failure Forward-Audit Authorization Testplan

## Objective

Validate that NA-0545 performs read-only authorization and triage for the three
non-required remote/relay CI failures, records D-1080, selects a safe successor,
and preserves required-check, qsc, workflow, runtime, remote, qsl-server,
qsl-attachments, public-site, Cloudflare, private-material, and no-claim
boundaries.

## Required Markers

- `NA0545_D1078_PROOF_REVIEW_CONSUMED_OK`
- `NA0545_D1079_CLOSEOUT_CONSUMED_OK`
- `NA0545_FRESH_QWORK_PROOF_OK`
- `NA0545_CURRENT_MAIN_CHECKS_CLASSIFIED_OK`
- `NA0545_PUBLIC_SAFETY_GREEN_OK`
- `NA0545_ADVISORIES_GREEN_OK`
- `NA0545_NO_FAILED_REQUIRED_CHECKS_OK`
- `NA0545_REMOTE_HANDSHAKE_AUDITED_OK`
- `NA0545_REMOTE_RELAY_AUDITED_OK`
- `NA0545_RELAY_UI_INTEGRATION_AUDITED_OK`
- `NA0545_HISTORICAL_RUNS_CLASSIFIED_OK`
- `NA0545_WORKFLOW_INVENTORY_READ_ONLY_OK`
- `NA0545_REFERENCED_SCRIPT_INVENTORY_READ_ONLY_OK`
- `NA0545_LOG_VISIBILITY_CLASSIFIED_OK`
- `NA0545_TARGET_CHECK_CLASSIFICATIONS_SELECTED_OK`
- `NA0545_SUCCESSOR_MODEL_SELECTED_OR_STOP_RECORDED_OK`
- `NA0545_NO_WORKFLOW_MUTATION_OK`
- `NA0545_NO_RUNTIME_MUTATION_OK`
- `NA0545_NO_QSC_SOURCE_MUTATION_OK`
- `NA0545_NO_DEPENDENCY_LOCKFILE_MUTATION_OK`
- `NA0545_NO_REMOTE_ACTION_OK`
- `NA0545_NO_QWORK_EXECUTION_OK`
- `NA0545_NO_QSL_BACKUP_EXECUTION_OK`
- `NA0545_NO_QSL_SERVER_ATTACHMENTS_OK`
- `NA0545_NO_PUBLIC_SITE_MUTATION_OK`
- `NA0545_NO_CLOUDFLARE_MUTATION_OK`
- `NA0545_NO_PRIVATE_MATERIAL_PUBLISHED_OK`
- `NA0545_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0545_NO_PRODUCTION_READINESS_CLAIM_OK`
- `NA0545_ONE_READY_INVARIANT_OK`

## Validation Gates

- qwork proof-file verification without running qwork/qstart/qresume;
- pre-fetch clean worktree, index, untracked, disk, mount, HEAD, and origin/main
  proof;
- origin/main equals or descends from `594704571c36`;
- queue proof before patch: READY_COUNT 1, READY NA-0545, NA-0544 DONE,
  NA-0543 DONE;
- decision proof before patch: D-1078 once, D-1079 once, D-1080 absent,
  D-1081 absent, duplicate decision count zero;
- inheritance review for D-1078, D-1079, NA-0544 DONE, NA-0545 READY, and prior
  non-required forward-audit failures;
- fresh current-main GitHub check-run, workflow-run, branch-protection, and
  combined-status classification;
- public-safety completed success;
- advisories completed success;
- no failed required checks;
- historical run/job/log metadata classification for remote-handshake,
  remote-relay, and relay-ui-integration;
- current workflow inventory read-only;
- referenced script/test inventory read-only;
- saved-log-extract private-material scan;
- exact five-path implementation scope guard;
- marker proof;
- changed Markdown link check;
- added-line/new-file private-material scan;
- added-line/new-file overclaim scan;
- docs/governance-only classifier;
- PR body preflight;
- goal-lint when available;
- root cargo audit;
- nested qsc fuzz lock cargo audit;
- `cargo fmt --check`;
- `sh -n scripts/ci/qsc_adversarial.sh`;
- `bash -n scripts/ci/qsc_adversarial.sh`.

Focused qsc runtime tests may be skipped because this lane changes only
governance/testplan evidence and does not mutate qsc source/runtime/dependency,
workflow, protocol, wire, or security implementation paths.

## Expected Result

Success classification:

`REMOTE_RELAY_FORWARD_AUDIT_REPRODUCTION_AUTHORIZATION_READY`

Selected successor:

`NA-0546 -- QSL Remote/Relay Non-Required CI Failure Bounded Reproduction Authorization Plan`

This lane fails if qwork proof is invalid, current main required checks fail or
are ambiguous, any target check cannot be classified, workflow/script ownership
cannot be identified, private material would be published, scope exceeds the
allowed five paths, a forbidden mutation/action occurs, or any prohibited
public, production, completion, defect-absence, or perfection claim is
introduced.
