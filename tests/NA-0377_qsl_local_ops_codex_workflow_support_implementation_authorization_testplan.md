Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-05-28

# NA-0377 QSL Local Ops Codex Workflow Support Implementation Authorization Test Plan

Goals: G1, G2, G3, G4, G5

## Objective

Validate that NA-0377 selects the first bounded local-ops workflow-support
successor while preserving no-implementation, no-runtime, no-secret,
no-backup-mutation, no-workflow-mutation, queue, CI, and public-claim
boundaries.

## Protected Invariants

- READY remains NA-0377 during the authorization PR.
- READY_COUNT remains 1.
- D-0736 exists once after the authorization PR.
- D-0737 remains absent until optional closeout.
- NA-0377 does not implement qstart/qresume, helper scripts, response writers,
  polling helpers, validation profiles, directive manifests, allow-files,
  history indexes, claim scanners, backup coverage changes, runtime changes,
  workflows, dependencies, qsl-server, qsl-attachments, qshield runtime,
  website/public docs, README, START_HERE, backup scripts, timers, fstab,
  system services, remote/off-host setup, restore, deploy, rollback, target
  setup, host-key scans, key handling, credential handling, or secret handling.

## Allowed Scope

- `docs/governance/evidence/NA-0377_qsl_local_ops_codex_workflow_support_implementation_authorization.md`
- `tests/NA-0377_qsl_local_ops_codex_workflow_support_implementation_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

The optional `docs/ops/CODEX_WORKFLOW_SUPPORT_IMPLEMENTATION_AUTHORIZATION.md`
artifact is allowed only if live NA-0377 scope explicitly names it. This lane
does not add that optional artifact.

## Forbidden Scope

No changes are allowed in:

- `.github/**`
- `scripts/**`
- `/srv/qbuild/tools/**`
- qstart/qresume tool files
- `/home/victor/work/qsl/codex/**` except the required final D196 response file
- `/usr/local/sbin/qsl-backup`
- systemd timers/services
- fstab
- backup source lists
- `Cargo.toml`
- `Cargo.lock`
- qsc/qsp/qsl runtime paths
- qshield runtime paths
- qsl-server or qsl-attachments paths
- qsc-desktop
- website or external website repository
- `README.md`
- `START_HERE.md`
- `docs/public/**`

## NA-0376 Inheritance Requirements

The evidence must record:

- NA-0376 planned workflow-support areas;
- NA-0376 priority order;
- off-host backup target/host-identity operator input remains external;
- qsl-server PR #56 and qsl-attachments PR #37 remain bounded evidence only;
- NA-0376 did not implement local-ops tooling.

## qstart/qresume Discovery Requirements

The evidence must classify:

- qstart/qresume source found or absent;
- source path;
- authority boundary;
- backup boundary;
- future readiness or blocker status;
- exact future allowed and forbidden paths.

## Polling Helper Discovery Requirements

The evidence must inspect existing qsl-protocol polling/check helpers and state:

- whether implementation could stay in qsl-protocol;
- whether workflow changes are required;
- whether tests can cover helper behavior;
- backup and CI impact.

## Manifest/Allow-File Discovery Requirements

The evidence must inspect:

- live directive patterns;
- `qsl_evidence_helper.py scope-guard` support;
- manifest/schema conventions if any;
- local versus qsl-protocol storage boundaries;
- backup coverage.

## Response Writer Discovery Requirements

The evidence must inspect:

- response archive presence;
- naming and collision requirements;
- backup coverage;
- helper-location authority;
- no-secret output risks.

## History Index Discovery Requirements

The evidence must inspect directives, responses, journals, requests, and ops
roots, then classify source, backup coverage, authority, and readiness.

## Risk Matrix Requirements

The evidence must include the eleven candidate lanes with value, risk,
authority, path category, backup impact, CI impact, security impact,
testability, readiness, and order.

## First-Lane Authorization Requirements

The evidence must decide whether qstart/qresume can be selected, explain why,
and reject safer alternatives only with evidence.

## Path Bundle Requirements

The evidence must define exact future allowed paths for the selected successor
and must forbid unrelated local ops, runtime, workflow, service, dependency,
backup, secret, and public-claim changes.

## Backup Impact Requirements

The evidence must decide:

- whether NA-0377 itself requires a backup-plan update;
- whether selected NA-0378 needs backup-impact review;
- which local history roots remain not covered or partially covered;
- D132 preservation status.

## Fail-Closed Requirements

Future work must require clean worktree checks, expected-main SHA validation,
configured-remote fetches only, fast-forward-only behavior, finite retries,
bounded waits, exact scope guards, no force, no stash-as-mutation, no amend,
no branch deletion, no hidden cleanup, deterministic output, and testable red
check behavior.

## Public-Claim Boundary Requirements

The evidence must not claim production readiness, public-internet readiness,
external review completion, anonymity, metadata-free behavior, untraceability,
hidden attachment size, hidden timing, hidden traffic shape, off-host backup
completion, complete disaster recovery, target setup, host identity
verification, real restore completion, or local-ops tooling completion.

## Successor Selection Requirements

Expected successor if qstart/qresume source, authority, and backup boundaries
are clear:

`NA-0378 -- QSL Local Ops qstart/qresume Fast-Forward Guard Implementation Harness`

If those boundaries are not clear, evidence must instead select the exact
blocker successor.

## Required Local Checks

Run from the qsl-protocol repo root:

```bash
git diff --check
cargo audit --deny warnings
cargo tree -i rustls-webpki --locked
cargo fmt --check
python3 -m json.tool inputs/metadata_runtime/off_host_backup_target_candidate_host_identity_operator_action_packet_v1.json >/dev/null
python3 -m json.tool inputs/metadata_runtime/off_host_backup_target_candidate_host_identity_operator_response_collection_request_v1.json >/dev/null
bash scripts/ci/metadata_runtime_production_backup_deploy_rollback_harness.sh inputs/metadata_runtime/production_backup_deploy_rollback_fixture_v1.json
bash scripts/ci/metadata_runtime_restore_drill_dry_run_harness.sh inputs/metadata_runtime/restore_drill_dry_run_fixture_v1.json
bash scripts/ci/metadata_runtime_key_custody_recovery_no_secret_harness.sh inputs/metadata_runtime/key_custody_recovery_no_secret_fixture_v1.json
bash scripts/ci/metadata_runtime_off_host_backup_target_tool_no_secret_harness.sh inputs/metadata_runtime/off_host_backup_target_tool_no_secret_fixture_v1.json
bash scripts/ci/metadata_runtime_restore_drill_isolated_restore_no_secret_harness.sh inputs/metadata_runtime/restore_drill_isolated_restore_no_secret_fixture_v1.json
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
cargo +stable test -p qsc --locked --test na_0313_handshake_suite_id_parameter_block -- --test-threads=1
python3 formal/model_qsc_handshake_suite_id_bounded.py
python3 formal/run_model_checks.py
cargo +stable build -p qshield-cli --locked
cargo +stable test -p qshield-cli --locked --tests -- --test-threads=1
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main \
  --allowed docs/governance/evidence/NA-0377_qsl_local_ops_codex_workflow_support_implementation_authorization.md \
  --allowed tests/NA-0377_qsl_local_ops_codex_workflow_support_implementation_authorization_testplan.md \
  --allowed DECISIONS.md \
  --allowed TRACEABILITY.md \
  --allowed docs/ops/ROLLING_OPERATIONS_JOURNAL.md
python3 scripts/ci/qsl_evidence_helper.py link-check
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
bash scripts/ci/classify_ci_scope.sh \
  docs/governance/evidence/NA-0377_qsl_local_ops_codex_workflow_support_implementation_authorization.md \
  tests/NA-0377_qsl_local_ops_codex_workflow_support_implementation_authorization_testplan.md \
  DECISIONS.md \
  TRACEABILITY.md \
  docs/ops/ROLLING_OPERATIONS_JOURNAL.md
```

## CI Expectations

The PR must merge only after required qsl-protocol checks complete normally and
`public-safety` remains required and green. No admin bypass, direct push,
squash, rebase, amend-after-PR, force-push, or branch deletion command is
allowed.

## Successor Handoff

After the authorization PR merges and post-merge public-safety is green, a
separate closeout may mark NA-0377 DONE and restore the selected NA-0378
successor. Closeout must not implement NA-0378.
