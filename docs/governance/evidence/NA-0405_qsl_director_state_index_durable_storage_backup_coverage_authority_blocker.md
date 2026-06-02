Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-02

# NA-0405 QSL Director State Index Durable Storage Backup Coverage Authority Blocker

Goals: G1, G2, G3, G4, G5

## Executive Summary

NA-0405 performed the read-only backup coverage and storage-authority review
needed before any future durable Director State Index file can be written under
Codex ops. The review confirms the NA-0404 blocker remains: the installed local
continuity backup source list and latest manifest cover Codex `responses`,
Codex `logs`, and the backup plan file, but do not cover
`/home/victor/work/qsl/codex/ops`.

The proposed future durable current-index path:

`/home/victor/work/qsl/codex/ops/director_state_index/current/director_state_index.json`

is therefore not inferably covered if created later. Exact directory authority
is also not sufficient for a future durable write until backup/source-list
authorization is resolved. The selected successor is:

`NA-0406 -- QSL Codex Ops Backup Coverage / Source-List Authorization Plan`

No durable Director State Index output was written. No backup script, timer,
fstab entry, source list, backup status file, backup plan, helper, fixture,
runtime, dependency, workflow, public doc, website, README, START_HERE,
response archive, or local history file was mutated by this blocker review.

## Live NA-0405 Scope

Live `NEXT_ACTIONS.md` shows:

- item: `NA-0405 -- QSL Director State Index Durable Storage Backup Coverage / Authority Blocker Resolution`
- status: READY
- objective: resolve backup coverage and storage authority for any future
  durable Director State Index under `/home/victor/work/qsl/codex/ops`
- required boundary: no durable local index unless future exact scope and
  backup review authorize it
- protected surfaces: runtime, service, protocol, crypto, dependency, workflow,
  backup script/timer/fstab/source-list, public docs, website, README,
  START_HERE, durable local index output, response archives, local history,
  qsl-server, qsl-attachments, qshield runtime, and secret-handling paths
- acceptance: READY_COUNT 1, READY NA-0405, NA-0404 DONE, D-0791 once,
  D-0792 once, no NA-0405 implementation by closeout, and public-safety
  required/green

The live scope matches this directive. It authorizes blocker-resolution
governance evidence and read-only backup/history inspection only.

## Inherited NA-0404 Blocker

NA-0404 completed durable Director State Index storage / backup-impact
authorization planning in PR #1072, then NA-0404 closeout restored NA-0405 in
PR #1073. NA-0404 found:

- qsl-protocol PR #1072 merged at `f4ebd89b69e0`
- qsl-protocol PR #1073 merged at `3ed19a72d124`
- current expected origin/main was
  `3ed19a72d124762450cef461596cad5b25350dc1`
- READY_COUNT was 1 and READY was NA-0405
- D-0791 and D-0792 existed once
- D-0793 was absent
- response archives were BACKUP_COVERED_SAME_HOST
- Codex ops was BACKUP_COVERAGE_ABSENT for the proposed durable current-index
  root
- `/srv/qbuild/tmp` remained NOT_DURABLE
- no durable Director State Index output was written

NA-0405 inherits the same storage gate and may resolve it only with read-only
proof or by selecting an authorization successor. It may not repair coverage by
mutating backup configuration in this lane.

## Backup Coverage Source Inventory

Read-only local evidence reviewed:

- `/backup/qsl` is mounted on `/dev/sda1` as ext4.
- current disk proof showed `/backup/qsl` available and mounted.
- latest manifest/log evidence at review time:
  - `/backup/qsl/manifests/daily-20260602T023434-0500.manifest.txt`
  - `/backup/qsl/logs/daily-20260602T023434-0500.log`
- backup plan file exists at
  `/home/victor/work/qsl/codex/QSL_BACKUP_PLAN.md`
- backup status file exists at
  `/home/victor/work/qsl/codex/ops/backup/QSL_BACKUP_STATUS.md`
- installed and candidate qsl-backup scripts were inspected read-only.

Installed daily source list:

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

Installed weekly cache source list:

- `/srv/qbuild/cache/cargo`
- `/srv/qbuild/cache/rustup`
- `/srv/qbuild/cache/sccache`

Observed latest manifest/source evidence:

- included `/home/victor/work/qsl/codex/responses`
- included `/home/victor/work/qsl/codex/logs`
- included `/home/victor/work/qsl/codex/QSL_BACKUP_PLAN.md`
- did not include `/home/victor/work/qsl/codex/ops`
- did not include `/home/victor/work/qsl/codex/requests`
- did not include `/home/victor/work/qsl/codex/directives`
- did not include `/home/victor/work/qsl/codex/journals`

Local history directory evidence:

- `/home/victor/work/qsl/codex/responses` exists and contains D224 through
  D228 plus earlier responses.
- `/home/victor/work/qsl/codex/requests` exists but is not in the installed
  daily source list.
- `/home/victor/work/qsl/codex/directives` was absent.
- `/home/victor/work/qsl/codex/journals` was absent.
- `/home/victor/work/qsl/codex/ops` exists and contains backup status,
  installed backup evidence, logs, restore-drill evidence, and a candidate
  script copy.
- `/home/victor/work/qsl/codex/ops/director_state_index` does not exist.

## Codex Ops Backup Coverage Determination

Classifications:

- `/home/victor/work/qsl/codex/responses`: BACKUP_COVERED_SAME_HOST
- `/home/victor/work/qsl/codex/logs`: BACKUP_COVERED_SAME_HOST
- `/home/victor/work/qsl/codex/QSL_BACKUP_PLAN.md`: BACKUP_COVERED_SAME_HOST
- `/home/victor/work/qsl/codex/requests`: BACKUP_COVERAGE_ABSENT
- `/home/victor/work/qsl/codex/directives`: BACKUP_COVERAGE_ABSENT for the
  absent directory
- `/home/victor/work/qsl/codex/journals`: BACKUP_COVERAGE_ABSENT for the absent
  directory
- `/home/victor/work/qsl/codex/ops`: BACKUP_COVERAGE_ABSENT
- `/home/victor/work/qsl/codex/ops/director_state_index/current/director_state_index.json`:
  BACKUP_COVERAGE_ABSENT if created later under the current source list
- `/srv/qbuild/tmp/NA0403_director_state_index_*`: NOT_DURABLE

Conclusion:

1. `/home/victor/work/qsl/codex/ops` is not backup-covered by current evidence.
2. The future durable index path is not inferably backup-covered if created
   later.
3. Directory authority is not clear enough to authorize a future durable index
   write.
4. Proven current coverage is same-host local continuity only. It is not
   off-host evidence.
5. Missing evidence is an explicit source-list or status/manifest proof that
   covers Codex ops and the intended director state index subtree, plus exact
   scope authority for making any needed backup configuration change.

## Durable Director State Index Authority Determination

Durable index storage remains blocked until a future directive proves or
authorizes all of the following:

- exact durable path authority
- backup coverage for Codex ops or the precise director state index subtree
- same-host continuity caveat retained
- off-host backup treated as separate evidence
- helper support for the exact durable path if durable generation is later
  authorized
- stale-state rejection preserved
- live repo/GitHub/CI authority preserved
- no-secret scan passes
- public-claim scan passes
- checksum requirement is defined
- generated timestamp and generator version are included
- no overwrite, delete, cleanup, or retention policy is assumed without exact
  future scope
- public-safety and required CI are green

Required future index fields include:

- origin/main SHA
- READY item
- latest decision
- public-safety status
- generated timestamp
- generator version
- advisory disclaimer
- checksum

The durable index remains advisory only. It does not prove local-ops completion,
does not replace live repo/GitHub/CI evidence, and must be rejected when stale
or conflicting.

## Resolution Options

### 1. No Backup Change; Keep Durable Index Blocked

- value: safest current result and matches observed evidence
- risk: durable handoff remains unavailable
- path authority: no new authority required
- backup impact: none now
- stale-state risk: no durable index risk
- restore implications: no new restore surface
- verification needs: document blocker and successor
- recommendation: recommended for NA-0405

### 2. Authorize Future Source-List Update for Codex Ops

- value: smallest path to make Codex ops durable-index storage eligible
- risk: backup scope may include operational files that need exclusion review
- path authority: future exact scope required
- backup impact: source-list and status/manifest evidence required
- stale-state risk: durable index still requires stale rejection
- restore implications: future restore must preserve ops evidence without
  overwriting live files unless separately authorized
- verification needs: preflight/list proof, source-list diff, no-secret review,
  manifest proof, status update, and rollback plan
- recommendation: selected successor lane

### 3. Authorize Broader Codex Root Coverage After Exclusion Review

- value: covers responses, requests, ops, and possible future history roots
- risk: broader than the immediate durable-index need; may pull in unsuitable
  state without exclusions
- path authority: future exact scope required
- backup impact: larger source-list change
- stale-state risk: response/history artifacts remain historical evidence only
- restore implications: broader restore planning needed
- verification needs: exclusion review, no-secret scan, source-list proof,
  manifest proof, status update, rollback plan
- recommendation: possible future alternative, not recommended as the first
  smallest lane

### 4. Use Response Archive Only

- value: response archives are already same-host covered
- risk: response files are narrative history, not machine current-state
  artifacts
- path authority: final response only is authorized in this directive
- backup impact: no new backup impact
- stale-state risk: high if historical responses are mistaken for live state
- restore implications: responses remain historical evidence
- verification needs: final response path proof only
- recommendation: insufficient for durable index storage

### 5. Use qsl-protocol Tracked Governance Only

- value: git-backed, reviewable, CI-gated
- risk: high churn and easy to confuse a generated summary with live evidence
- path authority: future tracked path scope required
- backup impact: no local Codex ops backup impact, but still advisory
- stale-state risk: medium unless every use rechecks live evidence
- restore implications: governed by normal repository recovery
- verification needs: generated disclaimer and stale rejection
- recommendation: not the selected path for local handoff state

### 6. Use `/srv/qbuild/tmp` Only

- value: already supported by the helper and fixture matrix
- risk: not durable by design
- path authority: temp output is already bounded by NA-0403 helper behavior
- backup impact: no durable index backup impact
- stale-state risk: low if regenerated per directive
- restore implications: none for durable state
- verification needs: helper fixture matrix
- recommendation: keep as fallback only

## Future Backup Coverage Authorization Design

Because Codex ops coverage is absent, NA-0406 should design a future
authorization lane before durable index implementation. It should consider:

- exact source path:
  `/home/victor/work/qsl/codex/ops`
- possible narrower source path:
  `/home/victor/work/qsl/codex/ops/director_state_index`
- intended durable output path:
  `/home/victor/work/qsl/codex/ops/director_state_index/current/director_state_index.json`
- whether the source-list change should cover only the director state index
  subtree or all Codex ops after exclusion review
- explicit exclusions, if any
- qsl-backup preflight/list/dry-run safety classification before use
- backup status update requirements
- manifest/log proof that the selected path is included
- no secret material in generated artifacts
- no backup execution unless a future directive authorizes it explicitly
- no restore unless a future directive authorizes it explicitly
- rollback plan for any source-list change
- same-host continuity caveat
- public-claim caveat

NA-0406 should not write a durable Director State Index. It should authorize or
plan backup/source-list coverage only, unless its future exact scope says
otherwise.

## Durable Index Authority Preservation

Future durable index storage remains blocked until:

- path authority is explicit
- backup coverage is proven
- stale-state policy is preserved
- helper support for exact durable path is authorized
- no-secret and public-claim scans pass
- no overwrite/delete/cleanup policy is assumed
- public-safety and CI are green
- same-host continuity is not described as disaster recovery

The durable index must remain advisory only. Live repo/GitHub/CI evidence stays
authoritative. Same-host continuity must not be promoted to off-host evidence.
The durable index must not claim local-ops completion.

## Selected Successor

Selected successor:

`NA-0406 -- QSL Codex Ops Backup Coverage / Source-List Authorization Plan`

Rationale:

- Codex ops is not in the installed daily source list.
- The latest manifest does not include Codex ops.
- The future durable index path would sit under Codex ops.
- This directive forbids source-list mutation, backup status mutation, backup
  execution, and restore execution.
- A durable index implementation lane would be premature without coverage and
  authority proof.
- No live NA-0405 scope conflict was found.

Rejected successors:

- `NA-0406 -- QSL Director State Index Durable Storage Implementation Harness`
  because backup coverage and directory authority are not proven.
- `NA-0406 -- QSL Director State Index Backup Coverage Scope Conflict Resolution`
  because the live NA-0405 scope matched this directive.

## Future Path / Scope Bundle

Allowed future NA-0406 paths for the selected backup/source-list authorization
successor:

- `docs/governance/evidence/NA-0406_qsl_codex_ops_backup_coverage_source_list_authorization_plan.md`
- `tests/NA-0406_qsl_codex_ops_backup_coverage_source_list_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Forbidden unless future exact scope authorizes:

- backup script/timer/fstab/source-list mutation
- backup execution
- restore execution
- durable Director State Index output
- helper mutation
- runtime, crypto, dependency, or workflow changes
- qsl-server or qsl-attachments changes
- public docs or website changes
- public claims
- secret handling

## Future Validation / Marker Plan

Future NA-0406 markers should include:

- `NA0406_CODEX_OPS_BACKUP_COVERAGE_AUTHORIZATION_OK`
- `NA0406_SOURCE_LIST_CHANGE_PLAN_OK`
- `NA0406_BACKUP_IMPACT_REVIEW_OK`
- `NA0406_NO_BACKUP_MUTATION_OK`
- `NA0406_NO_RESTORE_MUTATION_OK`
- `NA0406_NO_DURABLE_INDEX_WRITE_OK`
- `NA0406_NO_HELPER_MUTATION_OK`
- `NA0406_SAME_HOST_CONTINUITY_CAVEAT_OK`
- `NA0406_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0406_NO_SECRET_MATERIAL_OK`

Future validation should include queue/decision proof, backup source-list proof,
manifest/status proof, no-secret scan, public-claim scan, scope guard, link
check, leak scan, cargo audit, dependency tree proof, qsc targeted checks,
formal model checks, qshield-cli build/test where feasible, and required CI.

## Public Claim / External Review / Website Boundary

This work is internal governance/local-ops only. It is:

- not disaster recovery
- not off-host backup evidence
- not restore proof
- not public docs
- not external review
- not a public technical paper
- not production readiness
- not public-internet readiness
- not a security policy update

No README, START_HERE, docs/public, website, disclosure policy, security policy,
issue template, or public technical paper file is changed by NA-0405.

## Rejected Alternatives

- Write the durable Director State Index now.
- Create `/home/victor/work/qsl/codex/ops/director_state_index`.
- Treat response archives as a current machine index.
- Treat `/srv/qbuild/tmp` as durable storage.
- Mutate backup source lists, backup status, backup plan, backup scripts,
  timers, fstab, or systemd units.
- Run a backup or restore.
- Broaden scope into helper, fixture, runtime, dependency, workflow, public doc,
  qsl-server, qsl-attachments, website, README, START_HERE, response archive,
  request archive, directive archive, journal archive, or secret-handling work.

## Backup-Plan Impact Statement

No backup-plan update is required for NA-0405 because this patch changes only
tracked qsl-protocol governance, traceability, rolling journal, and testplan
files. It writes no durable Director State Index file and performs no backup
configuration mutation, backup execution, or restore execution.

Future Codex ops coverage likely requires explicit NA-0406 authorization before
any source-list, status, or backup plan mutation occurs.

## Next Recommendation

Execute:

`NA-0406 -- QSL Codex Ops Backup Coverage / Source-List Authorization Plan`

That lane should produce exact source-list authority and backup-impact evidence
for Codex ops before any durable Director State Index implementation harness is
selected.
