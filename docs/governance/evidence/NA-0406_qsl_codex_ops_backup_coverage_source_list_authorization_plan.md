Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-02

# NA-0406 QSL Codex Ops Backup Coverage Source List Authorization Plan

Goals: G1, G2, G3, G4, G5

## Executive Summary

NA-0406 reviewed the live Codex ops backup coverage blocker inherited from
NA-0405 and inspected the current same-host local backup surfaces read-only.
The active daily backup source roots are defined in the embedded
`daily_sources` array inside `/usr/local/sbin/qsl-backup`. No separate source
list or config file was found.

Current coverage still includes `/home/victor/work/qsl/codex/logs`,
`/home/victor/work/qsl/codex/responses`, and
`/home/victor/work/qsl/codex/QSL_BACKUP_PLAN.md`. It does not include
`/home/victor/work/qsl/codex/ops`, requests, directives, journals, or the
future director state index subtree. Therefore a future durable Director State
Index write under Codex ops remains blocked until future exact scope proves
coverage and preserves stale-state, no-secret, and public-claim boundaries.

Source-list authority is clear enough to authorize a future implementation
harness: the exact local-system file is `/usr/local/sbin/qsl-backup`, and the
exact source definition is the embedded `daily_sources` array. That future lane
must explicitly authorize local-system mutation of that path. A qsl-protocol PR
alone cannot change the installed backup source list.

Selected successor:

`NA-0407 -- QSL Codex Ops Backup Coverage / Source-List Implementation Harness`

## Live NA-0406 Scope

Live `NEXT_ACTIONS.md` shows:

- item: `NA-0406 -- QSL Codex Ops Backup Coverage / Source-List Authorization Plan`
- status: READY
- objective: authorize and define the exact future backup coverage /
  source-list plan for Codex ops before any durable Director State Index file
  may be written under `/home/victor/work/qsl/codex/ops`
- protected surfaces: runtime, service, protocol, crypto, dependency,
  workflow, backup script, timer, fstab, source list, public docs, website,
  README, START_HERE, durable local index output, response archive, local
  history, qsl-server, qsl-attachments, qshield runtime, and secret-handling
  paths
- acceptance: READY_COUNT 1, READY NA-0406, NA-0405 DONE, D-0793 once,
  D-0794 once, no NA-0406 implementation by NA-0405 closeout, and
  public-safety required/green

The live scope matches this directive. It authorizes governance evidence,
testplan, decision, traceability, and rolling-journal updates only. It permits
read-only backup/source inspection and forbids backup/source-list mutation.

## Inherited NA-0405 Blocker

NA-0405 established:

- qsl-protocol PR #1074 merged at `6ccf51542dab`
- qsl-protocol PR #1075 merged at `9dce76c68df8`
- final origin/main was `9dce76c68df8`
- READY_COUNT was 1 and READY was NA-0406
- NA-0405 was DONE
- D-0793 and D-0794 existed once
- D-0795 was absent
- Codex responses/logs/backup plan were BACKUP_COVERED_SAME_HOST
- Codex ops was BACKUP_COVERAGE_ABSENT
- requests, directives, and journals were absent from coverage
- `/srv/qbuild/tmp` remained NOT_DURABLE
- no backup/source-list mutation, backup execution, restore execution, durable
  Director State Index output, helper mutation, fixture mutation, response
  archive mutation beyond D229, or local history mutation occurred

NA-0406 inherits the blocker that the proposed durable path:

`/home/victor/work/qsl/codex/ops/director_state_index/current/director_state_index.json`

is not inferably covered under the current daily source list.

## Backup Coverage Source Inventory

Read-only evidence reviewed:

- `/home/victor/work/qsl/codex/QSL_BACKUP_PLAN.md`
- `/home/victor/work/qsl/codex/ops/backup/QSL_BACKUP_STATUS.md`
- `/usr/local/sbin/qsl-backup`
- `/etc/systemd/system/qsl-backup-daily.service` through `systemctl cat`
- `/etc/systemd/system/qsl-backup-daily.timer` through `systemctl cat`
- latest manifest:
  `/backup/qsl/manifests/daily-20260602T023434-0500.manifest.txt`
- latest log:
  `/backup/qsl/logs/daily-20260602T023434-0500.log`
- Codex local history directories under `/home/victor/work/qsl/codex`

Current daily source roots embedded in `/usr/local/sbin/qsl-backup`:

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

Current weekly cache source roots:

- `/srv/qbuild/cache/cargo`
- `/srv/qbuild/cache/rustup`
- `/srv/qbuild/cache/sccache`

Current exclusions embedded in `/usr/local/sbin/qsl-backup`:

- `*/target/***`
- `srv/qbuild/cache/targets/***`
- `/srv/qbuild/cache/targets/***`
- `srv/qbuild/tmp/qsc-test-tmp/***`
- `/srv/qbuild/tmp/qsc-test-tmp/***`
- `srv/qbuild/tmp/qsl_dur_*/***`
- `/srv/qbuild/tmp/qsl_dur_*/***`
- `srv/qbuild/tmp/node-compile-cache/***`
- `/srv/qbuild/tmp/node-compile-cache/***`
- `*/__pycache__/***`
- `*/.pytest_cache/***`

Coverage classification:

| Path | Classification | Evidence |
|---|---|---|
| `/home/victor/work/qsl/codex/logs` | BACKUP_COVERED_SAME_HOST | present in installed daily source list |
| `/home/victor/work/qsl/codex/responses` | BACKUP_COVERED_SAME_HOST | present in installed daily source list and latest manifest source block |
| `/home/victor/work/qsl/codex/QSL_BACKUP_PLAN.md` | BACKUP_COVERED_SAME_HOST | present in installed daily source list |
| `/home/victor/work/qsl/codex/requests` | BACKUP_COVERAGE_ABSENT | not in source list or latest manifest |
| `/home/victor/work/qsl/codex/directives` | BACKUP_COVERAGE_ABSENT | directory absent and not in source list or latest manifest |
| `/home/victor/work/qsl/codex/journals` | BACKUP_COVERAGE_ABSENT | directory absent and not in source list or latest manifest |
| `/home/victor/work/qsl/codex/ops` | BACKUP_COVERAGE_ABSENT | not in source list or latest manifest |
| `/home/victor/work/qsl/codex/ops/director_state_index` | BACKUP_COVERAGE_ABSENT | directory absent and not in source list or latest manifest |
| `/srv/qbuild/tmp/NA0403_director_state_index_*` | NOT_DURABLE | temp-only fixture output |

The live `/usr/local/sbin/qsl-backup` checksum is
`c82ee76fa3573`. The older status file records an earlier preserved installed
checksum for `qsl-backup.installed`; that status file is historical evidence,
not current source-list authority.

## Source-List Authority Discovery

Answers:

1. Current backup source roots are defined in the `daily_sources` and
   `weekly_cache_sources` arrays embedded in `/usr/local/sbin/qsl-backup`.
2. The source list is embedded in the script, not in a separate config file.
3. Future Codex ops coverage would require mutating `/usr/local/sbin/qsl-backup`
   unless a future directive first installs a separate source-list/config
   mechanism.
4. The backup status file is generated evidence. It is not the source of the
   installed source list.
5. The backup plan file is policy documentation. It is not executable
   authority for the installed source roots.
6. The systemd service/timer schedule the script through
   `ExecStart=/usr/local/sbin/qsl-backup daily`; they do not define source
   roots.
7. Mutation is local-system/local-ops authority, not qsl-protocol PR-only
   authority. Future exact scope must authorize `/usr/local/sbin/qsl-backup`
   mutation and must stop if required privileges or operator authorization are
   unavailable.

Classifications:

- SOURCE_LIST_AUTHORITY_CLEAR
- SOURCE_LIST_IMPLEMENTATION_READY_FOR_FUTURE_SCOPE

Non-selected classifications:

- SOURCE_LIST_AUTHORITY_PARTIAL
- SOURCE_LIST_AUTHORITY_UNKNOWN
- SOURCE_LIST_MUTATION_REQUIRES_USER_ACTION
- SOURCE_LIST_IMPLEMENTATION_BLOCKED

## Authority / Risk / Backup-Plan Update Options

### 1. No Backup Change; Keep Durable Index Blocked

- value: safest no-mutation state
- risk: durable Director State Index remains unavailable
- path authority: no new authority required
- backup impact: none now
- stale-state risk: no durable index risk
- restore implications: no new restore surface
- test/verification needs: document blocker and selected successor
- recommendation: not enough to advance durable storage, but acceptable if
  future local-system mutation is not authorized

### 2. Future Source-List Update for `/home/victor/work/qsl/codex/ops`

- value: covers the proposed durable index subtree and existing ops backup
  evidence under one narrow root
- risk: ops may contain operational files requiring no-secret and volatility
  review
- path authority: future local-system scope for `/usr/local/sbin/qsl-backup`
- backup impact: source-list change plus status/manifest evidence required
- stale-state risk: durable index still must reject stale/conflicting state
- restore implications: restore must be separately authorized and must not
  overwrite live files without exact scope
- test/verification needs: syntax check, preflight/list, no-secret scan,
  source-list diff, rollback plan, scheduled or authorized manifest proof
- recommendation: recommended successor path

### 3. Future Narrow Source-List Update for `/home/victor/work/qsl/codex/ops/director_state_index`

- value: smallest future durable-index subtree
- risk: does not cover existing ops backup evidence or future ops support files
- path authority: future local-system scope for `/usr/local/sbin/qsl-backup`
- backup impact: smaller source-list change
- stale-state risk: still requires stale-state rejection
- restore implications: restore scope is narrow but still must be authorized
- test/verification needs: same as option 2
- recommendation: acceptable alternative if future scope wants only the index
  subtree

### 4. Future Broader `/home/victor/work/qsl/codex` Coverage

- value: covers responses, logs, requests, ops, and possible future history
  roots under one parent
- risk: broader than the current blocker and may include unsuitable local
  history or secret-bearing material unless exclusions are reviewed first
- path authority: future local-system scope for `/usr/local/sbin/qsl-backup`
- backup impact: larger source-list change and exclusion review
- stale-state risk: historical artifacts must not be mistaken for current
  evidence
- restore implications: broader restore planning needed
- test/verification needs: full tree inventory, no-secret review, size review,
  source-list proof, manifest proof, rollback plan
- recommendation: not recommended as the first implementation harness

### 5. Use Response Archive Only

- value: already same-host covered
- risk: response files are narrative history, not durable current-state output
- path authority: only the final directive response is authorized here
- backup impact: none now
- stale-state risk: high if historical responses are treated as current
  evidence
- restore implications: response restore remains historical only
- test/verification needs: response file proof
- recommendation: insufficient for durable index storage

### 6. Use qsl-protocol Tracked Governance Only

- value: git-backed, reviewable, CI-gated
- risk: a generated state index in repo could be stale and must remain below
  live repo/GitHub/CI evidence
- path authority: future tracked path scope required
- backup impact: no Codex ops coverage impact
- stale-state risk: medium unless every use rechecks live evidence
- restore implications: normal repository recovery only
- test/verification needs: stale-state rejection, public-claim scan, CI proof
- recommendation: not sufficient for local Codex ops durable storage

### 7. Use `/srv/qbuild/tmp` Only

- value: useful for temp fixture proof
- risk: explicitly not durable
- path authority: temp-output-only helper scope
- backup impact: current backup includes `/srv/qbuild/tmp` generally but
  temp outputs remain operationally temporary
- stale-state risk: high
- restore implications: not a durable handoff path
- test/verification needs: temp-only proof
- recommendation: rejected for durable index storage

## Future Backup Coverage Authorization Design

Recommended future NA-0407 design:

- exact script path: `/usr/local/sbin/qsl-backup`
- exact source definition: embedded `daily_sources` array
- target path to add: `/home/victor/work/qsl/codex/ops`
- acceptable narrower alternative:
  `/home/victor/work/qsl/codex/ops/director_state_index`
- no broader `/home/victor/work/qsl/codex` root unless future scope includes a
  full exclusion review
- no source-list update unless future directive explicitly authorizes
  local-system mutation
- no real backup unless future directive explicitly authorizes it
- no restore unless future directive explicitly authorizes it
- no durable Director State Index write until coverage proof exists

Future proof sequence:

1. Verify clean qsl-protocol worktree and current queue.
2. Read the current script and record checksum/ownership/mode.
3. Review target path inventory for secret-bearing and high-churn material.
4. Add the target path to `daily_sources` only if exact future scope permits.
5. Run shell syntax validation on the script.
6. Run `qsl-backup preflight` and `qsl-backup list`.
7. Prove no backup or restore operation ran unless future scope authorizes it.
8. Record rollback plan: restore the prior script content/checksum/mode/owner
   or revert the exact source-list hunk.
9. If future scope permits dry-run, scheduled backup wait, or real backup,
   capture manifest proof that the new source appears.
10. Update backup status/plan only if future scope authorizes those local
    history files.
11. Preserve same-host-only caveat and avoid off-host claims.

Can proof be obtained without a real backup?

- Syntax and preflight/list proof can be obtained without a real backup.
- Source-list coverage intent can be proved by script diff plus preflight.
- Manifest proof requires either an explicitly authorized dry-run that writes
  backup artifacts, an explicitly authorized real backup, or waiting for the
  scheduled daily job after future scope permits that evidence capture.

## Durable Index Authority Preservation

Durable index storage remains blocked until:

- exact durable path authority is explicit
- backup coverage is proven for the selected path
- same-host continuity caveat is retained
- helper support for the exact durable path is authorized
- stale-state rejection is preserved
- live repo/GitHub/CI authority is preserved
- no-secret scan passes
- public-claim scan passes
- checksum requirement is defined
- generated timestamp and generator version are included
- no overwrite/delete/cleanup/retention policy is assumed without exact future
  scope
- public-safety and required CI are green

The durable index remains advisory only. It must never override live
repo/GitHub/CI evidence.

## Selected Successor

Selected successor:

`NA-0407 -- QSL Codex Ops Backup Coverage / Source-List Implementation Harness`

Rationale:

- source-list authority is clear
- exact mutation path is known
- current coverage is absent for Codex ops
- future implementation can be narrowly scoped to local-system source-list
  mutation and qsl-protocol governance evidence
- no live scope conflict was found

Rejected successors:

- `NA-0407 -- QSL Codex Ops Backup Coverage / Source-List Authority Discovery`
  because the source-list authority and exact mutation path are discoverable
  from live evidence.
- `NA-0407 -- QSL Codex Ops Backup Coverage Scope Conflict Resolution` because
  live NA-0406 scope matches the directive.

## Future Path / Scope Bundle

Future NA-0407 implementation harness allowed paths should include only paths
explicitly authorized by that future directive. Evidence supports considering:

- `/usr/local/sbin/qsl-backup`
- `/home/victor/work/qsl/codex/QSL_BACKUP_PLAN.md`
- `/home/victor/work/qsl/codex/ops/backup/QSL_BACKUP_STATUS.md`
- qsl-protocol evidence/testplan/decision/traceability/journal paths selected
  by future scope

Future forbidden unless explicitly authorized:

- backup timer, fstab, or systemd mutation
- backup execution
- restore execution
- durable Director State Index output
- helper mutation
- fixture mutation
- runtime/protocol/crypto/dependency/workflow changes
- qsl-server or qsl-attachments mutation
- public docs, README, START_HERE, or website changes
- public technical paper work
- secret handling
- response archive mutation

## Future Validation / Marker Plan

Future NA-0407 implementation harness markers:

- `NA0407_CODEX_OPS_BACKUP_SOURCE_LIST_IMPLEMENTATION_OK`
- `NA0407_SOURCE_LIST_PATH_AUTHORITY_OK`
- `NA0407_CODEX_OPS_PATH_ADDED_OK`
- `NA0407_BACKUP_SYNTAX_OK`
- `NA0407_BACKUP_PREFLIGHT_OK`
- `NA0407_NO_BACKUP_EXECUTION_OK`
- `NA0407_NO_RESTORE_EXECUTION_OK`
- `NA0407_ROLLBACK_PLAN_OK`
- `NA0407_SAME_HOST_CONTINUITY_CAVEAT_OK`
- `NA0407_NO_DURABLE_INDEX_WRITE_OK`
- `NA0407_NO_SECRET_MATERIAL_OK`
- `NA0407_NO_PUBLIC_READINESS_CLAIM_OK`

## Public Claim / External Review / Website Boundary

This authorization plan is internal governance/local-ops only. It is not:

- disaster-recovery evidence
- off-host backup evidence
- restore proof
- external review
- public technical paper work
- production-readiness evidence
- public-internet-readiness evidence
- public docs
- website work
- README or START_HERE work
- security policy or disclosure-policy work

No public claim expansion is authorized.

## Rejected Alternatives

- Mutating `/usr/local/sbin/qsl-backup` during NA-0406: rejected because this
  directive is authorization and planning only.
- Creating `/home/victor/work/qsl/codex/ops/director_state_index`: rejected
  because durable output remains blocked.
- Running `qsl-backup daily`, `checkpoint`, or `weekly-cache`: rejected because
  backup execution is not authorized.
- Running a restore: rejected because restore execution is not authorized.
- Broader Codex root coverage now: rejected because this directive forbids
  source-list mutation and broader scope requires a separate exclusion review.
- Treating response archives as a durable current-state index: rejected because
  responses are historical evidence only.

## Backup-Plan Impact Statement

No backup-plan update is required for NA-0406 because this PR changes only
tracked qsl-protocol governance, testplan, traceability, and journal files. It
does not create durable local output and does not mutate backup scripts,
source lists, status files, backup plan files, timers, fstab, or systemd units.

Future NA-0407 source-list work will likely require exact authorization for
`/usr/local/sbin/qsl-backup` and may require separately authorized updates to
Codex backup status or plan files after proof is captured.

## Next Recommendation

Execute:

`NA-0407 -- QSL Codex Ops Backup Coverage / Source-List Implementation Harness`

The future lane should add `/home/victor/work/qsl/codex/ops` to the daily
source list only after no-secret and size/volatility review, then prove syntax,
preflight/list, rollback, same-host caveat, and no backup/restore execution
unless future scope explicitly authorizes those operations.
