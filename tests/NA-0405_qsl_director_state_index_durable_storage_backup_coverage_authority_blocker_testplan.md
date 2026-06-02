Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-02

# NA-0405 Director State Index Durable Storage Backup Coverage Authority Blocker Testplan

Goals: G1, G2, G3, G4, G5

## Objective

Verify that NA-0405 resolves the current backup coverage / authority blocker by
read-only inspection and governance evidence only, selects the correct NA-0406
successor, and does not write durable Director State Index output or mutate
backup configuration, helper, fixture, runtime, dependency, workflow, public,
response archive, or local history paths.

## Protected Invariants

- READY_COUNT remains exactly 1.
- READY remains NA-0405 until optional closeout.
- NA-0404 remains DONE.
- D-0791 exists once.
- D-0792 exists once.
- D-0793 is added once by this blocker evidence.
- D-0794 remains absent until optional closeout.
- Public-safety remains required and green.
- Generated Director State Index data remains advisory and temp-only.

## Allowed Scope

- `docs/governance/evidence/NA-0405_qsl_director_state_index_durable_storage_backup_coverage_authority_blocker.md`
- `tests/NA-0405_qsl_director_state_index_durable_storage_backup_coverage_authority_blocker_testplan.md`
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
  configuration, secret handling, and public technical paper files.

## NA-0404 Inheritance Requirements

- Confirm NA-0404 PR #1072 merged at `f4ebd89b69e0`.
- Confirm NA-0404 closeout PR #1073 merged at `3ed19a72d124`.
- Confirm expected `origin/main` is
  `3ed19a72d124762450cef461596cad5b25350dc1`.
- Confirm READY_COUNT 1 and READY NA-0405 at startup.
- Confirm NA-0404 DONE.
- Confirm D-0791 exists once.
- Confirm D-0792 exists once.
- Confirm D-0793 absent at startup.
- Preserve NA-0404's finding that Codex ops coverage was not proven.

## Backup Coverage Inventory Requirements

Perform read-only backup/history inspection only. Required evidence:

- `/backup/qsl` mount and capacity proof.
- latest manifests/logs listing.
- installed daily source list.
- candidate source list if readable.
- response archive coverage.
- Codex requests/directives/journals/ops coverage or absence.
- `/srv/qbuild/tmp` temporary classification.
- no backup or restore execution.

Classify coverage using:

- BACKUP_COVERED_SAME_HOST
- BACKUP_COVERAGE_PARTIAL
- BACKUP_COVERAGE_UNKNOWN
- BACKUP_COVERAGE_ABSENT
- NOT_DURABLE
- NOT_AUTHORIZED

## Ops Coverage Classification Requirements

The evidence must answer:

1. Is `/home/victor/work/qsl/codex/ops` backup-covered?
2. Is `/home/victor/work/qsl/codex/ops/director_state_index/current/director_state_index.json`
   inferably backup-covered if created later?
3. Is the directory authority clear enough to authorize a future durable index
   write?
4. Is backup coverage same-host only or off-host?
5. What exact evidence is missing?
6. Which successor is correct?

If Codex ops coverage is absent or unknown, select:

`NA-0406 -- QSL Codex Ops Backup Coverage / Source-List Authorization Plan`

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

## No Durable Output Requirements

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
only Codex response archive write authorized by the directive is the final D229
response file after qsl-protocol work is complete or stopped.

## Public Claim Boundary Requirements

The blocker evidence must remain internal governance/local-ops only. It must
not update public docs, website, README, START_HERE, disclosure policy, security
policy, issue templates, or public technical paper files.

## Successor Selection Requirements

Select exactly one NA-0406 successor:

- `NA-0406 -- QSL Director State Index Durable Storage Implementation Harness`
  if Codex ops coverage and authority are proven.
- `NA-0406 -- QSL Codex Ops Backup Coverage / Source-List Authorization Plan`
  if Codex ops coverage is absent or unknown.
- `NA-0406 -- QSL Director State Index Backup Coverage Scope Conflict Resolution`
  if live NA-0405 scope conflicts with the directive.

Do not implement NA-0406.

## Backup-Impact Requirements

If NA-0405 changes only tracked qsl-protocol governance/testplan/traceability
and rolling journal files and creates no durable local index, no backup-plan
update is required. If durable Codex ops output or backup configuration mutation
is needed, stop or select the authorization successor.

## Required Local Checks

Run:

- startup queue and decision proof.
- branch protection and public-safety proof.
- PR preservation proof.
- dependency/advisory proof.
- helper help/compile/fixture matrix.
- read-only backup/history inspection.
- cargo metadata JSON parse.
- metadata runtime no-secret harnesses where directly runnable.
- `cargo fmt --check`.
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

If Codex ops coverage remains absent, NA-0406 must resolve exact backup/source
authority before any durable Director State Index write is authorized. NA-0406
must preserve advisory-only index semantics, live repo/GitHub/CI authority,
stale-state rejection, same-host continuity caveat, no public claims, and no
secret handling.
