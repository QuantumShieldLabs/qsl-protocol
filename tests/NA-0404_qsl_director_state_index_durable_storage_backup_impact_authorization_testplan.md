Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-02

# NA-0404 Director State Index Durable Storage Backup Impact Authorization Testplan

Goals: G1, G2, G3, G4, G5

## Objective

Verify that NA-0404 completes only the durable Director State Index storage and
backup-impact authorization plan, selects the correct NA-0405 successor, and
does not create durable index output or mutate helper, fixture, runtime,
dependency, workflow, public, backup, response archive, or local history paths.

## Protected Invariants

- READY_COUNT remains exactly 1.
- READY remains NA-0404 until optional closeout.
- NA-0403 remains DONE.
- D-0788 exists once.
- D-0789 exists once.
- D-0790 exists once.
- D-0791 is added once by this authorization plan.
- D-0792 remains absent until optional closeout.
- Public-safety remains required and green.
- Generated index data remains advisory and temp-only.

## Allowed Scope

- `docs/governance/evidence/NA-0404_qsl_director_state_index_durable_storage_backup_impact_authorization_plan.md`
- `tests/NA-0404_qsl_director_state_index_durable_storage_backup_impact_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Forbidden Scope

- `scripts/ci/qsl_director_state_index.py`
- `inputs/local_ops/director_state_index_fixtures/**`
- `.github/**`
- `Cargo.toml`
- `Cargo.lock`
- runtime, service, protocol, crypto, qshield runtime, qsl-server,
  qsl-attachments, qsc-desktop, website, README, START_HERE, docs/public,
  backup scripts, backup timers, fstab, source lists, response archives,
  request archives, directive archives, journal archives, unrelated ops
  history, durable Director State Index output, local qstart/qresume tooling,
  branch protection configuration, public-safety configuration, secret
  handling, and public technical paper files.

## D225/D226/D227 Recovery Requirements

- Confirm D225 was a stale-checkout false stop and true `origin/main`
  dependency health is green.
- Confirm D226 found the real missing-fixture prerequisite blocker.
- Confirm D227 added only `secret_sentinel_reject.json`, merged PR #1071 at
  `835c1c5d75c8`, and left NA-0404 READY.
- Confirm D-0790 exists once before adding D-0791.
- Confirm D-0791 and D-0792 are absent at start.

## NA-0403 Inheritance Requirements

- Read NA-0403 evidence and testplan.
- Preserve temp-output-only behavior.
- Preserve stale-state rejection.
- Preserve live repo/GitHub/CI authority above generated summaries.
- Preserve the future durable-storage backup-impact gate.
- Preserve qsl-server PR #56 and qsl-attachments PR #37 as read-only bounded
  evidence only.

## Temp Harness Review Requirements

Run:

- `python3 scripts/ci/qsl_director_state_index.py --help`
- `python3 -m py_compile scripts/ci/qsl_director_state_index.py`
- `python3 scripts/ci/qsl_director_state_index.py fixture --fixtures-dir inputs/local_ops/director_state_index_fixtures --tmp-dir /srv/qbuild/tmp/NA0403_director_state_index_NA0404_storage_plan_fixture_check --json`

Acceptance:

- helper help succeeds;
- compile succeeds;
- fixture matrix reports `fixture_count=20`, `pass_count=20`, and
  `fail_count=0`;
- output remains under `/srv/qbuild/tmp`; and
- no durable Director State Index output is created.

## Storage Options Requirements

The evidence must compare:

1. temp-output only;
2. qsl-protocol tracked governance summary;
3. local Codex ops durable current index;
4. local Codex ops durable snapshot archive;
5. response archive embedding only; and
6. no durable Director State Index.

Each option must record path example, benefits, risks, stale-state risk,
backup-impact status, complexity, scope risk, recommendation, and future
validation needs.

## Backup Coverage Requirements

Perform read-only backup/history inspection only. Required evidence:

- `/backup/qsl` mount and capacity proof;
- latest manifests/logs listing;
- installed daily source list;
- response archive coverage;
- Codex requests/directives/journals/ops coverage or absence;
- `/srv/qbuild/tmp` temporary classification; and
- no backup or restore execution.

Classify coverage using:

- BACKUP_COVERED_SAME_HOST
- BACKUP_COVERAGE_PARTIAL
- BACKUP_COVERAGE_UNKNOWN
- BACKUP_COVERAGE_ABSENT
- NOT_DURABLE
- NOT_AUTHORIZED

If `/home/victor/work/qsl/codex/ops` coverage is not proven, select the
backup coverage / authority blocker successor.

## Durable Authority / Staleness Policy Requirements

The plan must require future durable indexes to remain advisory only and below
live repo/GitHub/CI evidence. Future use must revalidate:

- origin/main SHA;
- READY item;
- latest decision;
- duplicate decision count;
- public-safety status;
- branch protection;
- dependency/advisory health;
- backup coverage; and
- generator/schema freshness.

Missing live evidence or mismatch must reject the durable index for current
evidence.

## Write / Retention / Update Policy Requirements

The plan must define future policy choices for:

- exclusive create;
- safe atomic replace;
- current pointer plus history snapshots;
- retention;
- no delete by default;
- no cleanup by default;
- no overwrite without exact future authority;
- checksum proof;
- permissions/owner;
- no-secret scan;
- public-claim scan; and
- rollback/recovery handling.

Do not implement these policies in NA-0404.

## No Implementation Requirements

- Do not implement durable storage.
- Do not write a durable Director State Index.
- Do not create durable index directories.
- Do not mutate helper or fixtures.
- Do not update backup scripts, timers, fstab, source lists, or systemd units.
- Do not run backup or restore.

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
only Codex response archive write authorized by the directive is the final D228
response file after all work is complete.

## No Response Archive Mutation Requirements

Do not write, edit, move, delete, copy, or overwrite response archive files
during NA-0404. Final D228 response file creation is handled after PR work.

## Public Claim Boundary Requirements

The plan must remain internal governance/local-ops evidence only. It must not
update public docs, website, README, START_HERE, disclosure policy, security
policy, issue templates, or public technical paper files.

## Successor Selection Requirements

Select exactly one NA-0405 successor:

- implementation harness if backup coverage and authority are proven;
- backup coverage / authority blocker if coverage or authority is not proven;
- scope conflict resolution if live scope conflicts with the directive.

Do not implement NA-0405.

## Backup-Impact Requirements

If NA-0404 changes only tracked qsl-protocol governance/testplan/traceability
and rolling journal files and creates no durable local index, no backup-plan
update is required. If a durable local Codex ops path is authorized later,
future backup coverage proof or an explicit blocker is required.

## Required Local Checks

Run:

- startup queue and decision proof;
- branch protection and public-safety proof;
- PR preservation proof;
- dependency/advisory proof;
- helper help/compile/fixture matrix;
- read-only backup/history inspection;
- cargo metadata JSON parse;
- metadata runtime no-secret harnesses where directly runnable;
- `cargo fmt --check`;
- `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1`;
- `python3 formal/model_qsc_handshake_suite_id_bounded.py`;
- `python3 formal/run_model_checks.py`;
- qshield-cli test/build where feasible;
- qsc NA-0313 harness where directly runnable;
- queue/decisions after patch;
- scope guard;
- link-check;
- leak-scan;
- classifier;
- goal-lint or PR body preflight; and
- overclaim scan over changed lines.

## CI Expectations

Create a PR only after local validation passes. Merge only after required checks
complete normally without admin bypass, squash, rebase, force-push, direct push,
amend, or branch deletion flags.

## Successor Handoff

If post-merge public-safety is green, optional closeout may mark NA-0404 DONE
and restore the exact selected NA-0405 successor. Closeout must not implement
NA-0405.
