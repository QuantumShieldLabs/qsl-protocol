Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-02

# NA-0404 QSL Director State Index Durable Storage Backup Impact Authorization Plan

Goals: G1, G2, G3, G4, G5

## Executive Summary

NA-0404 reviewed whether the Director State Index should move from temp-only
proof into durable storage. The review preserves the NA-0403 helper contract:
generated index data is advisory local-ops/governance evidence only, live
repo/GitHub/CI evidence remains authoritative, and stale or conflicting state
must be rejected before any future write.

The review does not authorize a durable local Codex ops current index now. The
local backup target is mounted and current response archives are covered by
same-host continuity snapshots, but `/home/victor/work/qsl/codex/ops` is not in
the installed daily source list and current manifests do not prove coverage for
that durable index root. The selected successor is therefore:

`NA-0405 -- QSL Director State Index Durable Storage Backup Coverage / Authority Blocker Resolution`

## Live NA-0404 Scope

Live `NEXT_ACTIONS.md` shows:

- item: `NA-0404 -- QSL Director State Index Durable Storage / Backup Impact Authorization Plan`
- status: READY
- objective: create a governance authorization plan for durable Director State
  Index storage and backup-impact handling
- required boundary: no durable local index until exact scope and backup review
  authorize it
- protected surfaces: runtime, service, protocol, crypto, dependency, workflow,
  backup script/timer/fstab, public docs, website, README, START_HERE,
  response archives, local history, helper, fixtures, qsl-server,
  qsl-attachments, qsc-desktop, and secret-handling paths
- acceptance: READY_COUNT 1, READY NA-0404, NA-0403 DONE, D-0788 once, D-0789
  once, and public-safety required/green

The live scope matches this directive. It authorizes planning and governance
evidence only.

## D225/D226/D227 Recovery Context

D225 stopped on a stale checkout. D226 later proved true `origin/main` was
dependency-healthy with `cargo audit --deny warnings` passing and
`rustls-webpki v0.103.13` present.

D226 then found a real prerequisite blocker: the Director State Index helper
required `secret_sentinel_reject.json`, but tracked main had only 19 fixtures.
That stop was correct because the D226 directive did not authorize helper or
fixture mutation.

D227 repaired that prerequisite in PR #1071 by adding only
`inputs/local_ops/director_state_index_fixtures/secret_sentinel_reject.json`.
PR #1071 merged at `835c1c5d75c8`, public-safety completed successfully, and
the fixture matrix now passes 20/20 on current `origin/main`.

## Inherited NA-0403 Evidence

NA-0403 added `scripts/ci/qsl_director_state_index.py` and the fixture matrix
under `inputs/local_ops/director_state_index_fixtures/`. The helper exposes
`fixture`, `generate`, and `validate` commands, uses Python standard library
only, and writes generated index proof only under
`/srv/qbuild/tmp/NA0403_director_state_index_*`.

NA-0403 explicitly deferred durable local index storage until backup-impact
authorization. It also required stale-state rejection, public-safety status
checks, duplicate decision rejection, no public-claim expansion, and no output
under Codex response or public docs paths.

## Director State Index Temp Harness Status Review

Current helper path:

`scripts/ci/qsl_director_state_index.py`

Fixture directory:

`inputs/local_ops/director_state_index_fixtures/`

Current validation:

- `python3 scripts/ci/qsl_director_state_index.py --help`: passed
- `python3 -m py_compile scripts/ci/qsl_director_state_index.py`: passed
- fixture matrix under
  `/srv/qbuild/tmp/NA0403_director_state_index_NA0404_retry_fixture_check`:
  `fixture_count=20`, `pass_count=20`, `fail_count=0`
- fixture matrix under
  `/srv/qbuild/tmp/NA0403_director_state_index_NA0404_storage_plan_fixture_check`:
  `fixture_count=20`, `pass_count=20`, `fail_count=0`

The temp harness remains advisory only and produces no durable Director State
Index output.

## Durable Storage Options Review

### 1. Continue Temp-Output Only

- example path: `/srv/qbuild/tmp/NA0403_director_state_index_*`
- benefits: no durable local-state burden; already validated by NA-0403 and
  D227
- risks: not available as a durable handoff artifact
- stale-state risk: low if generated and validated per directive, but output is
  intentionally temporary
- backup-impact status: NOT_DURABLE
- implementation complexity: already present
- scope risk: low
- recommendation: keep as the baseline and fallback
- future validation needs: preserve fixture matrix, stale rejects, and temp
  path boundary

### 2. qsl-protocol Tracked Governance Summary

- example future path: `docs/ops/DIRECTOR_STATE_INDEX.md`
- benefits: versioned by git and protected by PR review/checks
- risks: churn; merge noise; stale summary could be mistaken for repo truth
- stale-state risk: medium to high unless every use revalidates live evidence
- backup-impact status: repo-tracked
- implementation complexity: medium
- scope risk: medium because it can blur governance summary and live evidence
- recommendation: possible later, but not the first local durable index choice
- future validation needs: explicit disclaimer, generated timestamp, live SHA,
  READY item, latest decision, public-safety status, and stale rejection

### 3. Local Codex Ops Durable Current Index

- example future path:
  `/home/victor/work/qsl/codex/ops/director_state_index/current/director_state_index.json`
- benefits: handoff-friendly local current-state artifact
- risks: Codex ops backup coverage and authority must be proven first; stale
  summary could mislead future directives
- stale-state risk: high unless every read revalidates live repo/GitHub/CI
- backup-impact status: BACKUP_COVERAGE_ABSENT for the current ops root based
  on installed source list and manifest evidence
- implementation complexity: medium
- scope risk: high until backup authority is resolved
- recommendation: not authorized now; resolve backup coverage / authority first
- future validation needs: exact path authority, explicit CLI gate, metadata
  gate, stale rejection, checksum proof, backup coverage proof or explicit
  caveat, and no response archive/local history mutation

### 4. Local Codex Ops Durable Snapshot Archive

- example future path:
  `/home/victor/work/qsl/codex/ops/director_state_index/history/NA0405_<timestamp>_director_state_index.json`
- benefits: preserves historical snapshots
- risks: same backup authority issue as the current index, plus accumulation
  and retention risk
- stale-state risk: high; history must never be treated as current live proof
- backup-impact status: BACKUP_COVERAGE_ABSENT for the current ops root
- implementation complexity: high because retention and cleanup policy are
  required
- scope risk: high
- recommendation: future-gated even after current-index coverage is resolved
- future validation needs: retention policy, no-delete default, bounded snapshot
  count or explicit retention review, checksum proof, and clear stale labels

### 5. Response Archive Embedding Only

- example path: `/home/victor/work/qsl/codex/responses/*`
- benefits: final responses already capture narrative evidence and are covered
  by same-host continuity snapshots
- risks: not machine-friendly; increases response size; keeps state load high
- stale-state risk: medium because responses are historical by design
- backup-impact status: BACKUP_COVERED_SAME_HOST
- implementation complexity: low
- scope risk: high for this directive because response archive mutation is
  forbidden except the final D228 response file
- recommendation: insufficient for the index purpose
- future validation needs: keep responses as narrative evidence only

### 6. No Durable Director State Index

- example path: none
- benefits: simplest and safest if authority remains unresolved
- risks: state load remains high
- stale-state risk: none from durable index artifacts
- backup-impact status: none
- implementation complexity: none
- scope risk: low
- recommendation: valid fallback until Codex ops coverage is proven
- future validation needs: continue temp-only generation when needed

## Backup Coverage / Authority Review

Read-only local evidence:

- `/backup/qsl` is mounted on `/dev/sda1` as ext4.
- latest manifest/log evidence includes `daily-20260602T023434-0500`.
- installed daily source list includes:
  - `/srv/qbuild/tools`
  - `/srv/qbuild/docs`
  - `/srv/qbuild/logs`
  - `/srv/qbuild/evidence`
  - `/srv/qbuild/archive`
  - `/srv/qbuild/mirrors`
  - `/srv/qbuild/work`
  - `/srv/qbuild/tmp`
  - `/home/victor/work/qsl/codex/logs`
  - `/home/victor/work/qsl/codex/responses`
  - `/home/victor/work/qsl/codex/QSL_BACKUP_PLAN.md`
- latest manifest source list includes `/home/victor/work/qsl/codex/responses`
  but not `/home/victor/work/qsl/codex/ops`.
- `/home/victor/work/qsl/codex/requests` exists but is not in the installed
  source list.
- `/home/victor/work/qsl/codex/directives` and
  `/home/victor/work/qsl/codex/journals` were absent in read-only listing.
- `/srv/qbuild/tmp` is covered by local continuity snapshots but remains
  semantically temporary and is not a durable index root.

Classification:

- `/home/victor/work/qsl/codex/responses`: BACKUP_COVERED_SAME_HOST
- `/home/victor/work/qsl/codex/requests`: BACKUP_COVERAGE_ABSENT
- `/home/victor/work/qsl/codex/directives`: BACKUP_COVERAGE_ABSENT for the
  absent directory
- `/home/victor/work/qsl/codex/journals`: BACKUP_COVERAGE_ABSENT for the absent
  directory
- `/home/victor/work/qsl/codex/ops`: BACKUP_COVERAGE_ABSENT for the durable
  current-index root
- `/srv/qbuild/tmp/NA0403_director_state_index_*`: NOT_DURABLE

Backup-plan impact statement: no backup-plan update is required for this
NA-0404 patch because it changes only tracked qsl-protocol governance,
testplan, traceability, and journal files and writes no durable local index.
Future durable local Codex ops storage requires backup coverage / authority
resolution before implementation.

## Durable Index Authority / Staleness Policy

Any future durable Director State Index must be advisory only. It must not
override live qsl-protocol source, GitHub PR state, required CI, branch
protection, public-safety, cargo audit, dependency tree, or read-only local
backup evidence.

Required stale conditions:

- stored `origin_main_sha` differs from live `origin/main`
- stored READY item differs from live queue helper output
- stored latest decision differs from live decision helper output
- duplicate decisions are detected live
- required public-safety status is missing, red, ambiguous, or stale
- branch protection required contexts differ from live protection evidence
- backup coverage statement differs from live source list or manifests
- generated timestamp is outside the future directive's accepted freshness
  window
- generator version or schema is unknown

Before use, a future directive may cite the durable index only as a pointer to
what should be rechecked. It must re-run live queue, decisions, origin/main,
public-safety, dependency/advisory, branch-protection, and backup coverage
checks. Missing live sources must fail closed or be recorded as explicit
blockers; mismatches must reject the durable index for current evidence.

The durable file must display a warning that it is an internal local-ops
snapshot, may be stale, and is below live repo/GitHub/CI evidence.

## Durable Index Write / Retention / Update Policy

Future policy options:

- exclusive create: safest first write policy for new paths
- safe atomic replace: possible for a current index only if a future directive
  explicitly authorizes replacement semantics
- current pointer plus history snapshots: not authorized now; requires
  retention and accumulation review
- retention: snapshot history remains future-gated; no cleanup without exact
  future authorization
- deletion: not authorized by default
- overwrite: not authorized unless a future directive states the exact current
  file replacement policy
- checksum: required for every durable write
- permissions/owner: must be recorded and must not broaden access
- no-secret scan: required before durable write
- public-claim scan: required before durable write
- rollback/recovery: future directives must reject stale current files and
  preserve failed-write evidence without deleting artifacts unless authorized

NA-0404 performs no write, directory creation, retention policy, cleanup, or
helper mutation.

## Future Implementation Strategy

Because Codex ops backup coverage is not proven, the next step should not add a
durable-output mode. NA-0405 should first resolve backup coverage / authority.

After coverage is resolved by a future directive, a later implementation lane
may consider extending `scripts/ci/qsl_director_state_index.py` with an
explicit durable-output mode, fixtures for path validation, checksum proof, and
stale rejection. That later lane must preserve the temp-output mode and must
not mutate response archives, request archives, directive archives, journal
archives, or unrelated Codex history.

## Selected Successor

Selected successor:

`NA-0405 -- QSL Director State Index Durable Storage Backup Coverage / Authority Blocker Resolution`

Rationale: the durable current-index option depends on
`/home/victor/work/qsl/codex/ops` backup coverage and exact local authority.
Current installed backup source lists and manifests prove response archive
coverage but do not prove Codex ops coverage.

Rejected successors:

- `NA-0405 -- QSL Director State Index Durable Storage Implementation Harness`:
  rejected because backup coverage / authority for the Codex ops durable root
  is not proven.
- `NA-0405 -- QSL Director State Index Durable Storage Scope Conflict Resolution`:
  rejected because live NA-0404 scope matches this directive.

## Future Path / Scope Bundle

Selected blocker successor allowed paths:

- `docs/governance/evidence/NA-0405_qsl_director_state_index_durable_storage_backup_coverage_authority_blocker.md`
- `tests/NA-0405_qsl_director_state_index_durable_storage_backup_coverage_authority_blocker_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Future forbidden paths unless exact live scope authorizes them:

- response archives
- request, directive, journal, and unrelated ops history
- backup scripts, timers, fstab, source lists, systemd units, and real backup
  or restore operations
- runtime, protocol, crypto, qshield runtime, dependency, Cargo, and workflow
  paths
- qsl-server, qsl-attachments, qsc-desktop, website, public docs, README, and
  START_HERE
- secret material and public-claim surfaces

## Future Validation / Marker Plan

Future NA-0405 blocker markers:

- `NA0405_BACKUP_COVERAGE_BLOCKER_OK`
- `NA0405_DURABLE_STORAGE_AUTHORITY_BLOCKED_OK`
- `NA0405_NO_DURABLE_INDEX_WRITE_OK`
- `NA0405_NO_HELPER_MUTATION_OK`
- `NA0405_NO_RUNTIME_CHANGE_OK`
- `NA0405_NO_CRYPTO_IMPLEMENTATION_CHANGE_OK`
- `NA0405_NO_DEPENDENCY_CHANGE_OK`
- `NA0405_NO_WORKFLOW_CHANGE_OK`
- `NA0405_NO_SECRET_MATERIAL_OK`
- `NA0405_NO_PUBLIC_READINESS_CLAIM_OK`

If a later directive proves coverage and selects an implementation lane, that
later lane should use separate implementation markers for the explicit durable
output gate, exact path boundary, checksum proof, stale policy preservation,
live authority preservation, no response archive mutation, and no local history
mutation outside the exact authorized index path.

## Public Claim / External Review / Website Boundary

This authorization plan is internal qsl-protocol governance/local-ops evidence
only. It is not public docs, not external review, not a public technical paper,
not public readiness evidence, not service deployment evidence, not
public-internet evidence, not local-ops completion evidence, and not disaster
recovery proof.

NA-0404 does not update README, START_HERE, docs/public, website, security
policy, disclosure policy, issue templates, or public claim surfaces.

## Rejected Alternatives

- Writing a durable Director State Index in NA-0404.
- Creating `/home/victor/work/qsl/codex/ops/director_state_index`.
- Treating response archive embedding as sufficient durable machine-readable
  index storage.
- Treating same-host response archive coverage as proof that Codex ops durable
  index storage is covered.
- Updating backup scripts, timers, fstab, source lists, or systemd units in
  this directive.
- Running real backup or restore operations.
- Mutating `scripts/ci/qsl_director_state_index.py` or fixtures now.
- Creating public docs, website content, public technical paper material, or
  public security policy files.

## Backup-Plan Impact Statement

No backup-plan update is required for the NA-0404 patch because the only
durable changes are tracked qsl-protocol governance, testplan, traceability,
and rolling journal files. No durable Director State Index file is created and
no Codex history archive is modified.

A future durable local Codex ops index must not proceed until backup coverage
and storage authority are proven or explicitly recorded as a blocker.

## Next Recommendation

Merge the NA-0404 authorization plan after validation and required PR checks.
If post-merge public-safety is green, close out NA-0404 and restore the exact
selected blocker successor:

`NA-0405 -- QSL Director State Index Durable Storage Backup Coverage / Authority Blocker Resolution`
