Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-04

# NA-0417 QSL Backup Log Code 23 Root Operator Cleanup Permission Remediation Packet Plan

Goals: G4

## Executive Summary

NA-0417 is a planning-only lane for the scheduled same-host qsl-backup code 23
warning caused by the NA-0407 root-owned rollback subtree. It does not generate
operator packet files, does not execute root action, and does not mutate the
rollback subtree.

Current classification:

`CODE23_STILL_ACTIVE_NA0407_ROLLBACK_SUBTREE`

The latest scheduled log and manifest remain the 2026-06-04 daily pair. The
log still has exactly one rsync code 23 result, and that result still names the
NA-0407 rollback directory as permission denied. The matching manifest still
includes Codex ops exactly once.

Selected successor:

`NA-0418 -- QSL Backup Log Code 23 Root Operator Evidence Preservation / Permission Remediation Packet Generation Harness`

Recommended next direction:

NA-0418 should generate a bounded root-operator packet harness that preserves
rollback evidence first and then prepares permission remediation under exact
path scope. It should not select deletion as the first implementation path.
Delete-after-preservation remains a later, further-gated option only if future
root evidence proves it safe and explicitly authorized.

NA-0417 did not run qwork, qstart, qresume, sudo, backup, or restore. It did
not mutate `/usr/local/sbin/qsl-backup`, `/backup/qsl`, backup status or plan
files, the NA-0407 temp rollback subtree, systemd units, timers, fstab, source
lists, retention, backup scripts, runtime, protocol, crypto, dependency,
workflow, qsl-server, qsl-attachments, qshield runtime, website, public docs,
README, START_HERE, or any public technical paper surface. It made no off-host
backup completion, disaster recovery, restore proof, backup completion,
production readiness, public-internet readiness, external-review, metadata-free,
anonymity, untraceability, bug-free, vulnerability-free, or perfect-crypto
claim.

## Live NA-0417 Scope

Live `NEXT_ACTIONS.md` showed one READY item:

- `NA-0417 -- QSL Backup Log Code 23 Root Operator Cleanup / Permission Remediation Packet Plan`
- status: READY
- goals: G1, G2, G3, G4, G5

Allowed qsl-protocol mutation paths for this evidence PR are exactly:

- `docs/governance/evidence/NA-0417_qsl_backup_log_code_23_root_operator_cleanup_permission_remediation_packet_plan.md`
- `tests/NA-0417_qsl_backup_log_code_23_root_operator_cleanup_permission_remediation_packet_plan_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Allowed local inspection is read-only and limited to backup log and manifest
roots, the NA-0407 temp packet and rollback paths, qsl-backup, Codex ops backup
status and plan docs, Codex ops, the prior response file, and the qwork proof
files. The only local writes performed were proof files under:

`/srv/qbuild/tmp/NA0417_root_operator_packet_plan_20260604T080548-0500/`

Forbidden scope includes qwork/qstart/qresume execution by Codex, sudo, backup
execution, restore execution, qsl-backup mutation, `/backup/qsl` mutation, temp
rollback subtree deletion/move/copy/chmod/chown or other mutation, executable
operator packet generation, backup status/plan mutation, systemd/timer/fstab
source-list/retention or backup script mutation, durable Director State Index
output, qwork/qstart/qresume/qshell mutation, runtime/protocol/crypto/dependency
or workflow mutation, qsl-server, qsl-attachments, qshield runtime, website,
public docs, README, START_HERE, public technical paper work, secret handling,
and public-claim expansion.

Acceptance requires exact code 23 status, rollback evidence preservation
requirements, selected future scope, no-backup/no-restore/no-sudo/no-qsl-backup
and no-temp-mutation boundaries, one READY item, and public-safety remaining
green.

## qwork Proof-File Verification

Codex did not run qwork, qstart, or qresume.

The operator-provided qwork proof files existed:

- `/srv/qbuild/work/NA-0417/.qwork/startup.qsl-protocol.kv`
- `/srv/qbuild/work/NA-0417/.qwork/startup.qsl-protocol.json`

They were copied into the NA-0417 proof root under `qwork/`.

The `.kv` proof contained the required values:

- `startup_result=OK`
- `lane=NA-0417`
- `repo=qsl-protocol`
- `path=/srv/qbuild/work/NA-0417/qsl-protocol`
- `head=3bf432f123f1`
- `origin_main=3bf432f123f1`
- `head_equals_origin_main=yes`
- `worktree_clean=yes`
- `index_clean=yes`
- `untracked_clean=yes`
- `ready_count=1`
- `queue_top_ready=NA-0417`
- `requested_lane_status=READY`

The workspace JSON proof parsed successfully and mirrored the required KV
values for lane, repo, path, head, origin_main, ready_count, queue_top_ready,
requested_lane_status, and clean-state fields.

After `git fetch --all --prune`, live `HEAD` and live `origin/main` still
matched the qwork proof. PR #1101 was MERGED with merge commit
`3bf432f123f1`.

Startup proof:

- queue helper: READY_COUNT `1`; READY `NA-0417`
- NA-0416: DONE
- decisions helper: latest decision `D-0820`; duplicate count `0`
- D-0819 once
- D-0820 once
- D-0821 absent at start
- protected `public-safety` check on current main: completed success
- qsl-backup checksum prefix: `e9ecff3d22ed`
- qsl-backup Codex ops source inclusion count: `1`
- `cargo audit --deny warnings`: passed
- locked rustls tree: `rustls-webpki v0.103.13`

One public-safety helper invocation failed because the script requires a
`GH_TOKEN` or `GITHUB_TOKEN` environment variable. This was a recoverable
verification-tool environment issue. Codex corrected by using authenticated
read-only `gh` API calls without printing or exporting token material; branch
protection listed `public-safety` as required and the current main check-run
completed success.

## NA-0416 Inheritance

NA-0416 classification:

`CODE23_STILL_ACTIVE_NA0407_ROLLBACK_SUBTREE`

Inherited facts:

- latest scheduled log during NA-0416:
  `/backup/qsl/logs/daily-20260604T023542-0500.log`
- latest scheduled manifest during NA-0416:
  `/backup/qsl/manifests/daily-20260604T023542-0500.manifest.txt`
- latest log had exactly one rsync code 23 / permission-denied source
- the source named the NA-0407 rollback directory
- rollback path existed as `root:root` mode `2700`
- qsl-backup checksum matched the expected `e9ecff3d22ed` prefix
- qsl-backup Codex ops source inclusion count was exactly `1`
- Codex ops manifest presence remained same-host manifest-present evidence only
- NA-0416 did not run backup, restore, or sudo and did not mutate qsl-backup,
  the temp subtree, status docs, or plan docs

NA-0416 selected NA-0417 to plan the future root-operator packet shape, because
rollback file content inspection, checksum proof, permission changes, movement,
or deletion require future exact-scope operator/root action.

## Latest Log/Manifest Reconfirmation

Latest scheduled log identified during NA-0417:

`/backup/qsl/logs/daily-20260604T023542-0500.log`

Latest scheduled manifest identified during NA-0417:

`/backup/qsl/manifests/daily-20260604T023542-0500.manifest.txt`

Latest log summary:

- SHA-256 prefix: `9267d2a0543b`
- size: `1003` bytes
- `rsync error` count: `1`
- `code 23` count: `1`
- `Permission denied` count: `1`
- `NA0407` count: `1`
- `rollback` count: `1`
- `qsl-backup.preimage` count: `0`
- exact NA-0407 rollback directory count: `1`

Minimal identifying lines:

```text
rsync: [sender] opendir "/srv/qbuild/tmp/NA0407_qsl_backup_root_action_20260602T232945-0500/rollback" failed: Permission denied (13)
rsync error: some files/attrs were not transferred (see previous errors) (code 23) at main.c(1347) [sender=3.2.7]
```

Latest manifest summary:

- SHA-256 prefix: `1f209ce66e64`
- size: `20157` bytes
- Codex ops path count: `1`
- NA-0407 count: `0`
- rollback count: `0`

No additional rsync error, code 23, or permission-denied source was found in the
latest log. The code 23 warning remains active and is still tied exactly to the
NA-0407 rollback subtree.

## Root Operator Packet Plan Design

Future packet objective:

Preserve rollback evidence, inspect/read root-owned rollback metadata and file
checksum, decide whether permission remediation, move, deletion, or retain-as-is
is appropriate, avoid Codex sudo, avoid backup/restore execution, and avoid
qsl-backup mutation unless a later directive explicitly changes scope.

Future packet path pattern:

`/srv/qbuild/tmp/NA0418_code23_root_operator_packet_<timestamp>/`

Exact root-owned paths to inspect:

- `/srv/qbuild/tmp/NA0407_qsl_backup_root_action_20260602T232945-0500`
- `/srv/qbuild/tmp/NA0407_qsl_backup_root_action_20260602T232945-0500/rollback`
- the exact rollback file under the rollback directory, expected from readable
  packet scripts to follow the `qsl-backup.preimage.<sha256>` filename pattern

Exact expected qsl-backup SHA:

`e9ecff3d22eda21ceb0e889e4dd5d6f4e270e09349c77a1f4872bfc0052f6232`

Future packet stages:

1. Preflight stage:
   - require root/operator execution by the human operator, not Codex
   - verify exact rollback path
   - verify rollback directory owner/mode is still `root:root` and `2700`
     before action
   - verify qsl-backup checksum is unchanged
   - verify no backup or restore will run

2. Evidence preservation stage:
   - capture metadata of the packet directory and rollback directory
   - checksum readable packet metadata files
   - checksum the root-owned rollback file if root execution can read it
   - preserve or report the rollback file checksum without printing file content
   - record whether rollback file content is exactly qsl-backup preimage only
     if safe and without dumping secret or unrelated content

3. Remediation choice stage:
   - permission remediation option
   - move evidence option
   - retain-evidence option
   - deletion-after-preservation option
   - reject unsafe or ambiguous options

4. Validation stage:
   - confirm no backup or restore was run
   - confirm qsl-backup is unchanged unless exact future scope authorizes a
     different result
   - confirm rollback evidence is preserved
   - confirm no public claim expansion

5. Rollback stage:
   - define how to reverse permission or move changes if a future packet
     implements them
   - do not define rollback for deletion unless preservation is complete

Exact forbidden future-packet commands and behaviors unless a later directive
explicitly authorizes otherwise:

- Codex must not run `sudo`
- Codex must not run generated operator scripts
- no backup execution
- no restore execution
- no qsl-backup mutation
- no deletion, move, chmod, chown, copy, or other mutation of rollback subtree
  paths by Codex
- no systemd, timer, fstab, retention, backup target, source-list, or backup
  script mutation
- no secret material printing

Exact no-secret handling:

- write path and metadata evidence only
- write file checksums, sizes, owners, modes, and timestamps
- do not print rollback file contents
- do not include tokens, auth headers, route tokens, passphrases, private keys,
  or secret-bearing URLs
- use short SHA prefixes in checked-in prose unless tooling requires full hashes

Expected output artifacts:

- generated operator README
- packet manifest
- checksum manifest
- preflight report
- evidence preservation report
- apply script only if exact future scope authorizes executable script
  generation
- verify script only if exact future scope authorizes executable script
  generation
- rollback script only if exact future scope authorizes executable script
  generation

Exact operator output to paste back after future packet execution, if any:

- packet root path
- packet manifest SHA-256 prefix
- qsl-backup preflight SHA-256 prefix and unchanged/changed status
- rollback directory pre-action owner/mode
- rollback file name and checksum prefix
- selected remediation option
- post-action rollback directory owner/mode
- confirmation that no backup or restore was run
- confirmation that rollback file contents were not pasted
- confirmation that qsl-backup was not mutated unless explicitly authorized

## Remediation Selection Logic

NA-0418 should choose between packet types using the current code 23 status and
root-readable evidence.

Options:

1. Root evidence-preservation-only packet:
   - Use if content preservation is the only safe first step, or if permission
     remediation scope remains ambiguous.

2. Root permission-remediation packet:
   - Recommended default after evidence preservation.
   - Use if root evidence proves the rollback file is expected preimage
     evidence and making the subtree readable enough for scheduled backup is
     acceptable.

3. Root move-evidence packet:
   - Use only if preserving evidence outside the source-listed tree is more
     appropriate than permission changes and references can be updated safely.

4. Root delete-after-preservation packet:
   - Keep later-gated.
   - Use only if root evidence preservation is complete and a later directive
     explicitly authorizes destructive cleanup.

5. No-cleanup / retain-as-is decision:
   - Accept only if the project intentionally accepts persistent scheduled code
     23 warnings as an operational caveat.

6. Ambiguity blocker:
   - Use if root evidence or scope is unclear enough that continuing would risk
     untruthful evidence or behavior drift.

Recommended default:

The first future implementation should be an evidence-preservation plus
permission-remediation packet generation harness, not deletion.

## Status/Plan Impact Review

Read-only inspection covered:

- `/home/victor/work/qsl/codex/ops/backup/QSL_BACKUP_STATUS.md`
- `/home/victor/work/qsl/codex/QSL_BACKUP_PLAN.md`

Both docs still cite NA-0414 status alignment and do not cite NA-0415,
NA-0416, or NA-0417. They already preserve the same-host continuity caveat and
the scheduled rsync code 23 caveat. They should remain unchanged until actual
remediation, retention, or clean-log evidence exists.

Future status/plan update triggers:

- if NA-0418/NA-0419 clears the warning, update the local status and plan docs
  in an exact-scope lane to cite the cleared log evidence and preserve
  same-host caveats
- if the warning persists by decision, update the local status and plan docs in
  an exact-scope lane to cite the intentional persistent caveat
- if root evidence changes the classification, update both docs only after a
  future directive authorizes local status/plan mutation

## Selected Successor

Selected successor:

`NA-0418 -- QSL Backup Log Code 23 Root Operator Evidence Preservation / Permission Remediation Packet Generation Harness`

Rationale:

The latest log is not clean, the code 23 source remains exactly the root-owned
NA-0407 rollback subtree, and packet generation is no longer premature once
NA-0417 has frozen the future root-operator design. The least destructive next
step is generation of a no-secret packet harness for evidence preservation and
permission remediation, with no execution by Codex.

## Future Path/Scope Bundle

Future allowed scope for the normal NA-0418 packet generation harness may
include:

- qsl-protocol governance evidence/testplan paths for NA-0418
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- temp packet output under
  `/srv/qbuild/tmp/NA0418_code23_root_operator_packet_<timestamp>/`
- generated operator README/manifest/apply/verify/rollback scripts only if the
  exact future scope authorizes them
- read-only inspection of `/backup/qsl/logs`, `/backup/qsl/manifests`,
  `/srv/qbuild/tmp/NA0407_qsl_backup_root_action_20260602T232945-0500`, and
  `/usr/local/sbin/qsl-backup`

Future root operator packet may inspect, if future scope authorizes:

- `/srv/qbuild/tmp/NA0407_qsl_backup_root_action_20260602T232945-0500`
- `/srv/qbuild/tmp/NA0407_qsl_backup_root_action_20260602T232945-0500/rollback`
- exact rollback file under rollback directory

Future forbidden unless exact future scope authorizes:

- sudo by Codex
- root action execution by Codex
- backup execution
- restore execution
- qsl-backup mutation
- deletion/move/chmod/chown of rollback subtree
- backup status/plan mutation
- qwork/qstart/qresume/qshell mutation
- runtime/dependency/workflow changes
- public docs or website changes
- public claims

## Future Validation/Marker Plan

Common NA-0418 markers:

- `NA0418_ROOT_OPERATOR_PACKET_GENERATION_OK`
- `NA0418_ROLLBACK_EVIDENCE_PRESERVATION_PLAN_OK`
- `NA0418_CODEX_NO_SUDO_OK`
- `NA0418_NO_BACKUP_EXECUTION_OK`
- `NA0418_NO_RESTORE_EXECUTION_OK`
- `NA0418_NO_QSL_BACKUP_MUTATION_OK`
- `NA0418_NO_TEMP_SUBTREE_EXECUTION_BY_CODEX_OK`
- `NA0418_SAME_HOST_CONTINUITY_CAVEAT_OK`
- `NA0418_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0418_NO_SECRET_MATERIAL_OK`
- `NA0418_PERMISSION_REMEDIATION_PACKET_PLAN_OK`
- `NA0418_EXACT_ROOT_PATH_SCOPE_OK`

If future evidence proves packet generation is premature, use
`NA0418_PACKET_SCOPE_BLOCKER_OK`. If future direction becomes retain-as-is, use
`NA0418_RETAIN_ROLLBACK_EVIDENCE_PERSISTENT_CAVEAT_OK`.

## Public Claim/External Review/Website Boundary

Root operator packet planning is internal local-ops governance only.

This lane preserves these boundaries:

- same-host continuity is not disaster recovery
- manifest presence is not backup completion
- not off-host backup complete
- not restore proof
- not external review
- not public technical paper
- not production readiness
- not public-internet readiness
- no README, START_HERE, public docs, or website update
- no security policy update
- no backup-complete claim

## Rejected Alternatives

- Generate executable operator packet files in NA-0417: rejected because live
  scope authorizes planning only.
- Run sudo or ask Codex to inspect rollback content directly: rejected because
  Codex sudo is forbidden.
- Run backup or restore to prove the warning state: rejected because NA-0417 is
  read-only for backup state and must not execute backup or restore.
- Mutate qsl-backup exclusion/source behavior: rejected because it changes
  backup behavior before rollback evidence preservation.
- Delete the rollback subtree first: rejected because deletion is destructive
  and requires prior preservation plus later exact authorization.
- Update local backup status/plan docs now: rejected because actual remediation
  has not happened and the existing NA-0414 caveat remains truthful.

## Backup-Impact Statement

NA-0417 made no backup-impacting runtime or local backup-system change. It did
not run backup, run restore, mutate qsl-backup, mutate `/backup/qsl`, mutate
backup status or plan docs, mutate source lists, mutate the NA-0407 rollback
subtree, or generate operator packet scripts.

The latest scheduled log remains code-23 caveated. The latest manifest remains
same-host source-presence evidence only. It is not off-host backup, disaster
recovery, restore proof, backup completion, production readiness,
public-internet readiness, external review, or public technical paper evidence.

## Next Recommendation

Proceed to NA-0418 as a packet generation harness that creates a no-secret,
operator-run root packet under the future allowed temp root, with evidence
preservation and permission remediation as the default direction. Codex must not
execute that packet, run sudo, run backup, run restore, mutate qsl-backup, or
mutate the rollback subtree in NA-0418 unless a later directive explicitly
authorizes exact behavior.
