Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-05-28

# NA-0376 QSL Local Ops Codex Workflow Support and History Index Plan Test Plan

Goals: G1, G2, G3, G4, G5

## Objective

Validate that NA-0376 records a local-ops workflow-support and
directive/response/history-index plan only, preserves all no-implementation and
public-claim boundaries, and selects the exact NA-0377 successor.

## Protected Invariants

- READY remains NA-0376 during the planning PR.
- READY_COUNT remains 1.
- D-0734 exists once after the planning PR.
- D-0735 remains absent until a separate closeout.
- No helper implementation is introduced.
- No qstart/qresume mutation is introduced.
- No polling helper, response writer, validation profile, manifest generator,
  allow-file generator, index, or claim scanner is created.
- No qsl-server, qsl-attachments, qshield runtime, qsc/qsp/protocol/crypto,
  dependency, workflow, website/public-doc, README, START_HERE, backup script,
  timer, fstab, service, local backup config, off-host target, restore target,
  key, credential, secret, deploy, rollback, backup, or restore change occurs.
- Local same-host backup continuity is not presented as complete disaster
  recovery.
- Off-host encrypted backup is not presented as complete.
- Local-ops planning is not presented as implemented tooling.

## Allowed Scope

- `docs/governance/evidence/NA-0376_qsl_local_ops_codex_workflow_support_history_index_plan.md`
- `tests/NA-0376_qsl_local_ops_codex_workflow_support_history_index_plan_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

The optional `docs/ops/CODEX_WORKFLOW_SUPPORT_HISTORY_INDEX_PLAN.md` artifact is
allowed only if live NA-0376 scope explicitly names it. This lane does not add
that optional artifact.

## Forbidden Scope

No changes are allowed in:

- `.github/**`
- `scripts/**`
- qstart/qresume tool files
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
- `/home/victor/work/qsl/codex/**` except the final required D195 response file
- backup scripts, timers, fstab, system services, source lists, keys, restore
  targets, remote destinations, or monitoring configs

## Workflow-Support Request Inventory Requirements

The evidence must review:

- qstart/qresume fast-forward request;
- directive manifest request;
- response-file writer request;
- validation profile request;
- bounded polling helper request;
- scope-guard allow-file request;
- source/authority helper request;
- directive/response/journal index request;
- claim-boundary scanner request;
- known-transient CI note request;
- packet evidence templates;
- exact successor block;
- backup coverage for history roots;
- no-history-rewrite/no-amend guard;
- public-safety API/file-list failure recovery;
- response archive hygiene;
- D132 bundle status and cleanup boundary.

## History Availability Requirements

The evidence must classify authorized read-only roots as PRESENT, ABSENT, or
PARTIAL:

- `/home/victor/work/qsl/codex/directives`
- `/home/victor/work/qsl/codex/responses`
- `/home/victor/work/qsl/codex/journals`
- `/home/victor/work/qsl/codex/requests`
- `/home/victor/work/qsl/codex/ops`

The evidence must also classify whether each root should be indexed, should not
be mutated in NA-0376, and requires future backup-plan review.

## Backup Coverage Requirements

The evidence must:

- state current local backup posture;
- preserve same-host continuity limitation;
- identify which local history roots are currently covered or unknown;
- record that NA-0376 itself requires no backup-plan update;
- record that future history index, manifest, request/archive, journal, ops,
  response-writer, allow-file, polling-log, or cleanup changes require backup
  coverage review.

## qstart/qresume Planning Requirements

The plan must cover:

- expected-main SHA input;
- clean worktree check;
- fetch and fast-forward behavior;
- stale checkout detection;
- no dirty overwrite;
- no force;
- helper availability detection;
- fail-closed conflict handling;
- evidence output;
- tests required.

## Directive Manifest Planning Requirements

The plan must cover:

- directive ID;
- target NA;
- expected SHA;
- prior response path;
- mutable repo;
- read-only repos;
- allowed and forbidden paths;
- required checks;
- PR metadata;
- optional closeout successor block;
- validation rules;
- conflict handling;
- storage and backup boundaries;
- tests required.

## Response Writer Planning Requirements

The plan must cover:

- standard response wrapper;
- response file path;
- timestamp format;
- collision handling;
- required section skeleton;
- no-secret output policy;
- final path print;
- response archive backup coverage;
- tests required.

## Polling Helper Planning Requirements

The plan must cover:

- PR check polling;
- public-safety polling;
- merge commit polling;
- PR versus push contexts;
- CodeQL neutral/skip handling;
- docs-only skip handling;
- public-safety API 404 recovery;
- required check red behavior;
- bounded retry limits;
- failure summaries;
- JSON parsing safety;
- tests required.

## Validation Profile Planning Requirements

The plan must cover governance-plan, governance-closeout, docs-only,
runtime-harness, cross-repo read-only, and local-ops profiles, including output
summary format, commands included, failure policy, cost-control behavior, and
tests required.

## Source/Authority Helper Planning Requirements

The plan must cover read-only external repo proof, no fetch/checkout/clone
mutation by default, default branch proof, PR merge proof, latest CI proof,
branch protection summary, viewer permission, open PR summary, local path/SHA if
present, classification values, JSON/human output, stop behavior, and tests.

## Claim Scanner Planning Requirements

The plan must cover changed-line scanning, high-risk terms, unsafe/negated/
prohibited/future-gated/evidence-caveat classifications, file/line references,
false positive handling, public-readiness/privacy overclaim policy, PR body
preflight integration, and tests required.

## History Index Planning Requirements

The plan must cover index location, schema fields, read-only usage, explicit
authorization before update, journal mirror strategy, history retention,
evidence templates, backup coverage, and tests required.

## Governance/Security Boundary Requirements

The evidence must include a staged matrix with item, risk, implementation repo
or path category, future allowed files, forbidden files/actions, backup impact,
CI impact, security impact, evidence needed, priority, and recommended
successor lane.

## Public-Claim Boundary Requirements

The evidence must state that NA-0376 planning is not production readiness,
public-internet readiness, external review completion, metadata runtime claim
expansion, off-host backup completion, disaster recovery completion, operator
response intake, target setup, verified host identity, qsl-server production
proof, qsl-attachments production proof, qshield production proof, website work,
or public technical position paper work.

## Successor Selection Requirements

Expected successor:

`NA-0377 -- QSL Local Ops Codex Workflow Support Implementation Authorization Plan`

Evidence must explain rationale and reject narrower alternatives unless live
evidence requires one.

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
  --allowed docs/governance/evidence/NA-0376_qsl_local_ops_codex_workflow_support_history_index_plan.md \
  --allowed tests/NA-0376_qsl_local_ops_codex_workflow_support_history_index_plan_testplan.md \
  --allowed DECISIONS.md \
  --allowed TRACEABILITY.md \
  --allowed docs/ops/ROLLING_OPERATIONS_JOURNAL.md
python3 scripts/ci/qsl_evidence_helper.py link-check
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
bash scripts/ci/classify_ci_scope.sh \
  docs/governance/evidence/NA-0376_qsl_local_ops_codex_workflow_support_history_index_plan.md \
  tests/NA-0376_qsl_local_ops_codex_workflow_support_history_index_plan_testplan.md \
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

After the planning PR merges and post-merge public-safety is green, a separate
closeout may mark NA-0376 DONE and restore:

`NA-0377 -- QSL Local Ops Codex Workflow Support Implementation Authorization Plan`

Closeout must not implement NA-0377.
