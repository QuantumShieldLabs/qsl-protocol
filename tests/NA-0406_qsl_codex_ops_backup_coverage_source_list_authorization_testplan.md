Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-02

# NA-0406 Codex Ops Backup Coverage Source List Authorization Testplan

Goals: G1, G2, G3, G4, G5

## Objective

Verify that NA-0406 plans exact Codex ops backup coverage/source-list authority
for a future lane, selects the correct NA-0407 successor, and does not mutate
backup configuration, backup status, backup plan, helper, fixture, runtime,
dependency, workflow, public, response archive, or local history paths.

## Protected Invariants

- READY_COUNT remains exactly 1.
- READY remains NA-0406 until optional closeout.
- NA-0405 remains DONE.
- D-0793 exists once.
- D-0794 exists once.
- D-0795 is added once by this authorization plan.
- D-0796 remains absent until optional closeout.
- Public-safety remains required and green.
- Durable Director State Index output remains blocked.
- Same-host continuity is not described as disaster recovery.

## Allowed Scope

- `docs/governance/evidence/NA-0406_qsl_codex_ops_backup_coverage_source_list_authorization_plan.md`
- `tests/NA-0406_qsl_codex_ops_backup_coverage_source_list_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Forbidden Scope

- `scripts/**`
- `inputs/**`
- `.github/**`
- `Cargo.toml`
- `Cargo.lock`
- dependency changes
- runtime, service, protocol, crypto, qshield runtime, qsl-server,
  qsl-attachments, qsc-desktop, website, README, START_HERE, docs/public,
  backup scripts, backup timers, fstab, source lists, backup status files,
  backup plans, response archives, request archives, directive archives,
  journal archives, unrelated ops history, durable Director State Index output,
  local qstart/qresume tooling, branch protection configuration, public-safety
  configuration, secret handling, and public technical paper files

## NA-0405 Inheritance Requirements

- Confirm qsl-protocol PR #1074 merged at `6ccf51542dab`.
- Confirm qsl-protocol PR #1075 merged at `9dce76c68df8`.
- Confirm expected `origin/main` is
  `9dce76c68df8d8d37aa1f8b9816342ba8b294fea`.
- Confirm READY_COUNT 1 and READY NA-0406 at startup.
- Confirm NA-0405 DONE.
- Confirm D-0793 exists once.
- Confirm D-0794 exists once.
- Confirm D-0795 absent at startup.
- Preserve NA-0405's finding that Codex ops coverage was not proven.

## Backup Coverage Source Inventory Requirements

Perform read-only backup/history inspection only. Required evidence:

- `/backup/qsl` mount and capacity proof.
- latest manifest/log listing.
- backup plan and status file inspection.
- installed qsl-backup script source roots.
- systemd service/timer definitions.
- response archive evidence.
- Codex ops, requests, directives, and journals coverage or absence.
- no backup execution.
- no restore execution.

Classify coverage using:

- BACKUP_COVERED_SAME_HOST
- BACKUP_COVERAGE_PARTIAL
- BACKUP_COVERAGE_UNKNOWN
- BACKUP_COVERAGE_ABSENT
- NOT_DURABLE
- NOT_AUTHORIZED

## Source-List Authority Discovery Requirements

The evidence must answer:

1. Where current backup source roots are defined.
2. Whether the source list is embedded in a script or separate config.
3. Which exact file would need future mutation.
4. Whether qsl-protocol alone can change coverage.
5. Which target path should be added first.
6. What no-secret and size/volatility review is required.
7. What proof is possible without a real backup.
8. What proof requires scheduled or explicitly authorized backup evidence.
9. Whether the next successor is implementation harness, authority discovery,
   or scope conflict resolution.

Required classifications:

- SOURCE_LIST_AUTHORITY_CLEAR or a documented weaker classification
- SOURCE_LIST_IMPLEMENTATION_READY_FOR_FUTURE_SCOPE or a documented blocker
  classification

## No Backup Mutation Requirements

- Do not mutate backup scripts.
- Do not mutate timers.
- Do not mutate fstab.
- Do not mutate source lists.
- Do not mutate backup status files.
- Do not mutate backup plan files.
- Do not run a backup.
- Do not run a restore.
- Do not configure off-host targets.
- Do not handle keys, credentials, passphrases, private keys, or recovery
  envelopes.

## No Backup Execution Requirements

Allowed read-only commands include source inspection, manifest/log inspection,
`qsl-backup preflight`, and `qsl-backup list` when the subcommands are verified
as read-only. `daily`, `checkpoint`, and `weekly-cache` modes are forbidden in
NA-0406.

## No Restore Execution Requirements

Restore commands, restore target creation, restore copy operations, mount
setup, key/passphrase operations, and recovery-envelope handling are forbidden.

## No Durable Index Output Requirements

Verify no path is created under:

- `/home/victor/work/qsl/codex/ops/director_state_index`
- `/home/victor/work/qsl/codex/ops/director_state_index/current`
- `/home/victor/work/qsl/codex/ops/director_state_index/history`

Temp fixture proof under `/srv/qbuild/tmp/NA0403_director_state_index_*` is
allowed.

## No Helper Mutation Requirements

Verify `scripts/ci/qsl_director_state_index.py` has no diff and still passes
help, compile, and fixture matrix checks.

## No Local History Mutation Requirements

Do not mutate Codex request, directive, journal, response, or ops history. The
only Codex response archive write authorized by the directive is the final D230
response file after qsl-protocol work is complete or stopped.

## Public Claim Boundary Requirements

The authorization evidence must remain internal governance/local-ops only. It
must not update public docs, website, README, START_HERE, disclosure policy,
security policy, issue templates, or public technical paper files.

## Successor Selection Requirements

Select exactly one NA-0407 successor:

- `NA-0407 -- QSL Codex Ops Backup Coverage / Source-List Implementation Harness`
  if source-list authority is clear and exact mutation path is known.
- `NA-0407 -- QSL Codex Ops Backup Coverage / Source-List Authority Discovery`
  if source-list authority or exact mutation path remains ambiguous.
- `NA-0407 -- QSL Codex Ops Backup Coverage Scope Conflict Resolution` if live
  NA-0406 scope conflicts with the directive.

Do not implement NA-0407.

## Backup-Impact Requirements

If NA-0406 changes only tracked qsl-protocol governance/testplan/traceability
and rolling journal files and creates no durable local index, no backup-plan
update is required. If durable Codex ops output or backup configuration
mutation is needed, stop or select the future successor.

## Required Local Checks

Run:

- startup queue and decision proof.
- branch protection and public-safety proof.
- PR preservation proof.
- dependency/advisory proof.
- helper help/compile/fixture matrix.
- read-only backup/history/source-list inspection.
- cargo metadata JSON parse.
- metadata runtime no-secret harnesses where directly runnable.
- `cargo fmt --check`.
- `cargo audit --deny warnings`.
- `cargo tree -i rustls-webpki --locked`.
- `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1`.
- `python3 formal/model_qsc_handshake_suite_id_bounded.py`.
- `python3 formal/run_model_checks.py`.
- qshield-cli test/build where feasible.
- qsc NA-0313 harness where directly runnable.
- queue/decisions.
- exact allowed-path scope guard.
- `python3 scripts/ci/qsl_evidence_helper.py link-check`.
- `python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main`.
- changed-line overclaim scan.
- classifier or changed-path proof.
- PR body preflight / goal-lint.

## CI Expectations

Required qsl-protocol protected checks, including `public-safety`, must pass
before merge. Post-merge public-safety must also complete successfully before
optional closeout. Docs/governance-only cost-control skips are acceptable only
when public-safety reports them truthfully and remains green.

## Successor Handoff

Future NA-0407 must use the exact source-list authority identified by NA-0406,
preserve no-secret and no-public-claim boundaries, prove rollback and
preflight/list behavior, and avoid backup/restore execution unless future exact
scope authorizes it.
