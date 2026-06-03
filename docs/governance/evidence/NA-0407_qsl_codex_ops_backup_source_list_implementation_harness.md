Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-03

# NA-0407 QSL Codex Ops Backup Source List Implementation Harness

Goals: G1, G2, G3, G4, G5

## Executive Summary

NA-0407 validates the human-operator-applied same-host qsl-backup source-list
change that adds Codex ops to the installed daily source roots. Codex did not
run sudo, the apply script, rollback, a backup, or a restore. The validation
result is intentionally narrow: Codex ops is source-listed, but future
manifest/status verification is still required before any manifest-backed
coverage statement or backup status update.

Selected successor:

`NA-0408 -- QSL Codex Ops Backup Coverage Manifest Verification / Status Update Plan`

## Live Scope

Live `NEXT_ACTIONS.md` shows:

- item: `NA-0407 -- QSL Codex Ops Backup Coverage / Source-List Implementation Harness`
- status: READY
- goals: G1, G2, G3, G4, G5
- objective: implement the future Codex ops backup coverage / source-list
  implementation harness using the exact source-list authority identified by
  NA-0406
- protected surfaces: runtime, service, protocol, crypto, dependency, workflow,
  public docs, website, README, START_HERE, durable local index output, helper,
  fixture, qsl-server, qsl-attachments, qshield runtime, qsc-desktop, secret
  handling, real backup, restore, off-host setup, key handling, credential
  handling, and passphrase handling paths

The D236 directive narrowed qsl-protocol mutation to this evidence file, the
NA-0407 testplan, `DECISIONS.md`, `TRACEABILITY.md`, and the rolling operations
journal. D237 preserves the same qsl-protocol mutation scope and adds a second
proof root for the resumed validation:

`/srv/qbuild/tmp/NA0407_recover_d234_and_complete_20260603T093523-0500`

`/srv/qbuild/tmp/NA0407_complete_after_d236_20260603T100508-0500`

D237 also explicitly authorizes the helper-required Director State Index
fixture output path for validation only:

`/srv/qbuild/tmp/NA0403_director_state_index_NA0407_d237_fixture_check`

## D231, D233, D234, D236, and D237 Continuity

D231 stopped because `/usr/local/sbin/qsl-backup` was root-owned and Codex could
not safely edit it without sudo. The pre-operator state was `755 root root`,
size `6766`, checksum prefix `c82ee76fa357`, and Codex ops source count `0`.
D231 also recorded a temp-path proof-scope issue.

D233 created the no-secret root-operator packet at:

`/srv/qbuild/tmp/NA0407_qsl_backup_root_action_20260602T232945-0500`

The D233 packet identifies `/usr/local/sbin/qsl-backup` and the embedded
`daily_sources` array as the exact source-list authority. Its expected patch
adds only:

`/home/victor/work/qsl/codex/ops`

D234 validated the operator-applied post-state but stopped before PR because
one cargo metadata proof artifact was written at the unapproved `/tmp` path and
the qsl-protocol worktree remained dirty with draft evidence. D236 recovered
that state before any reset:

- captured dirty status, tracked diff names, full tracked diff, and untracked
  inventory under the approved proof root
- archived the expected untracked NA-0407 testplan draft under the proof root
- moved the stray cargo metadata artifact into the proof root
- verified the original `/tmp` path was gone
- reset qsl-protocol to clean `origin/main` at `f31227092836`

Recovered D234 dirty state:

- tracked dirty paths: `DECISIONS.md`, `TRACEABILITY.md`,
  `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- untracked expected draft:
  `tests/NA-0407_qsl_codex_ops_backup_source_list_implementation_testplan.md`
- moved temp artifact size: `56965` bytes
- moved temp artifact checksum prefix: `05caba37ae93`

D236 then stopped before commit/PR because the directive-approved proof root
conflicted with the Director State Index helper output policy. The helper
requires a first-level `/srv/qbuild/tmp/NA0403_director_state_index_*` output
directory, while D236 authorized only the NA-0407 proof root and forbade helper
mutation.

D237 recovered that conflict by preserving the D236 dirty draft patch, proving
the dirty paths are exactly the allowed NA-0407 evidence paths, and authorizing
the helper-required fixture output path:

`/srv/qbuild/tmp/NA0403_director_state_index_NA0407_d237_fixture_check`

qstart/qresume hardening remains a carry-forward local-ops item; Codex did not
mutate qstart, qresume, or qshell in NA-0407.

## Operator-Applied Source-List Review

Codex inspected the installed qsl-backup script read-only after the operator
action:

- stat: `755 root root`, size `6800`
- checksum prefix: `e9ecff3d22ed`
- `bash -n /usr/local/sbin/qsl-backup`: passed
- fixed-string Codex ops source count: `1`
- `daily_sources` includes `/home/victor/work/qsl/codex/ops` exactly once

The qsl-backup checksum transition is:

- old prefix: `c82ee76fa357`
- new prefix: `e9ecff3d22ed`

The full checksum proof is stored in the D237 proof root.

## Verify Script Proof

Codex ran only the D233 verify script:

`bash /srv/qbuild/tmp/NA0407_qsl_backup_root_action_20260602T232945-0500/verify_after_operator_action.sh`

The verify output included:

- `current_sha=e9ecff3d22ed...`
- `source_inclusion_count=1`
- `NO_BACKUP_OR_RESTORE_COMMAND_EXECUTED_BY_VERIFY_SCRIPT`
- `NA0407_OPERATOR_SOURCE_LIST_VERIFY_OK`

Codex did not run the apply script, rollback script, sudo, a backup mode, or a
restore mode.

## Manifest / Log Status

D237 snapped `/backup/qsl/manifests` and `/backup/qsl/logs` before and after the
read-only validation window. Entry-only comparison found:

`NO_NEW_MANIFEST_OR_LOG_DURING_READ_ONLY_VALIDATION`

Classification:

`SOURCE_LIST_UPDATED_NOT_MANIFEST_PROVEN`

This is same-host continuity evidence only. It is not disaster recovery, not
off-host backup evidence, not restore proof, not backup completion evidence,
and not public readiness.

## Codex Ops Read-Only Review

Codex inspected `/home/victor/work/qsl/codex/ops` read-only:

- file count: `8`
- total size: `32723` bytes
- symlink count: `0`
- symlink escape count: `0`
- corrected binary candidate count: `0`
- high-confidence secret path-name finding count: `0`
- high-confidence secret content path finding count: `0`
- `/home/victor/work/qsl/codex/ops/director_state_index`: absent

No secret content was copied into this evidence. The scan output is path-only
and stored under the D237 proof root.

## Evidence Boundary

NA-0407 changes only qsl-protocol governance/testplan/traceability/journal
files. It does not mutate qsl-backup, backup status files, backup plan files,
backup scripts, timers, fstab, helper code, fixtures, runtime code, dependency
files, workflow files, public docs, website, README, START_HERE, qsl-server,
qsl-attachments, qshield runtime, response archives, local history, or durable
Director State Index output.

Future NA-0408 must verify manifest/status evidence before any backup status
update, backup plan update, or durable Director State Index storage work.

## Validation Summary

Local D237 validation includes:

- `python3 scripts/ci/qsl_evidence_helper.py queue`
- `python3 scripts/ci/qsl_evidence_helper.py decisions`
- startup `public-safety` required/green proof on `f31227092836`
- `cargo audit --deny warnings`
- `cargo tree -i rustls-webpki --locked`
- qsl-backup stat/checksum/syntax/source-count validation
- D233 verify script
- manifest/log before-after entry comparison
- Codex ops read-only inventory and path-only safety scan
- Director State Index fixture matrix using the D237-authorized helper prefix

Additional PR validation is recorded in the rolling operations journal and PR
evidence once the clean patch is opened.
