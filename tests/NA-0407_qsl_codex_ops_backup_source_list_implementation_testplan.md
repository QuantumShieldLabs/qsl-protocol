Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-03

# NA-0407 QSL Codex Ops Backup Source List Implementation Testplan

## Objective

Verify that NA-0407 records and validates the human-operator-applied Codex ops
same-host qsl-backup source-list change without Codex running sudo, backup,
restore, apply, rollback, helper mutation, fixture mutation, durable index
output, backup status mutation, backup plan mutation, runtime changes,
dependency changes, workflow changes, public-doc changes, sibling-repo changes,
or secret handling.

## Protected Invariants

- READY_COUNT remains `1` at start.
- READY item at start is
  `NA-0407 -- QSL Codex Ops Backup Coverage / Source-List Implementation Harness`.
- D-0795 exists once.
- D-0796 exists once.
- D-0797 is absent before patch and exists once after patch.
- No backup execution occurs.
- No restore execution occurs.
- No durable Director State Index file is created.
- No helper or fixture path is mutated.
- No runtime, protocol, crypto, dependency, workflow, public-doc, website,
  README, START_HERE, qsl-server, qsl-attachments, qshield runtime, backup
  status, or backup plan path is mutated.
- Same-host continuity remains caveated.
- Source-list evidence is not treated as manifest-backed coverage.

## Allowed Scope

Packet D may modify only:

- `docs/governance/evidence/NA-0407_qsl_codex_ops_backup_source_list_implementation_harness.md`
- `tests/NA-0407_qsl_codex_ops_backup_source_list_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Read-only local validation may inspect:

- `/usr/local/sbin/qsl-backup`
- `/srv/qbuild/tmp/NA0407_qsl_backup_root_action_20260602T232945-0500`
- `/backup/qsl/manifests`
- `/backup/qsl/logs`
- `/home/victor/work/qsl/codex/ops`

Temporary proof output must remain under the authorized D237 proof root, except
for the explicitly authorized Director State Index helper fixture path:

`/srv/qbuild/tmp/NA0403_director_state_index_NA0407_d237_fixture_check`

## Forbidden Scope

Do not:

- run sudo;
- run the apply script;
- run rollback;
- run a real backup;
- run a restore;
- mutate `/usr/local/sbin/qsl-backup`;
- mutate backup status files;
- mutate backup plan files;
- mutate systemd units, timers, or fstab;
- mutate helper scripts or fixtures;
- mutate runtime, protocol, crypto, dependencies, workflows, public docs,
  website, README, START_HERE, qsl-server, qsl-attachments, qshield runtime, or
  qsc-desktop paths;
- create durable Director State Index output;
- do not create a public technical paper or public-readiness material;
- copy secret-bearing content into evidence.

## D234 and D236 Recovery Requirements

Validate D236/D237 recovery before completing the clean patch:

- preserve the dirty qsl-protocol status, tracked diff names, tracked diff, and
  untracked inventory under the approved proof root;
- archive only expected D234 untracked draft evidence/testplan files;
- move `/tmp/NA0407_cargo_metadata_runtime.json`, if present, into the
  approved proof root;
- verify the original `/tmp` artifact path is gone after move;
- reset qsl-protocol to clean `origin/main` only when required by the directive;
- verify `origin/main` equals the expected NA-0406 closeout SHA;
- verify the D236 dirty draft paths are exactly allowed if the worktree is not
  reset;
- run the Director State Index fixture matrix only under the D237-authorized
  helper-required temp path.

## D231 and D233 Inheritance Requirements

Validate:

- D231 response exists and records the root-owned stop and temp-path scope
  issue.
- D233 response exists and records the operator packet path.
- The operator packet exists with apply, verify, rollback, manifest, expected
  patch, preimage excerpt, and preflight evidence files.
- Codex does not mutate archived D230, D231, D233, or D234 response files.

## Operator-Output Validation Requirements

Validate live state against operator-reported facts:

- post-action target checksum prefix is `e9ecff3d22ed`;
- source inclusion count is exactly `1`;
- verify marker is `NA0407_OPERATOR_SOURCE_LIST_VERIFY_OK`;
- verify no-operation marker is
  `NO_BACKUP_OR_RESTORE_COMMAND_EXECUTED_BY_VERIFY_SCRIPT`;
- old checksum prefix was `c82ee76fa357`;
- syntax mode was not advertised by the verify script and was skipped there.

## qsl-backup Post-State Validation Requirements

Required commands:

- `stat -c '%a %U %G %s %y %n' /usr/local/sbin/qsl-backup`
- `sha256sum /usr/local/sbin/qsl-backup`
- `bash -n /usr/local/sbin/qsl-backup`
- fixed-string count for `/home/victor/work/qsl/codex/ops`
- `daily_sources` block extraction
- corrected delta against the D233 preimage excerpt

Pass conditions:

- mode/owner/group remain acceptable;
- syntax passes;
- checksum matches expected post-action state;
- Codex ops appears exactly once;
- no source-list removal is detected.

## Verify-Script Requirements

Run only:

`bash /srv/qbuild/tmp/NA0407_qsl_backup_root_action_20260602T232945-0500/verify_after_operator_action.sh`

Pass conditions:

- marker `NA0407_OPERATOR_SOURCE_LIST_VERIFY_OK` appears;
- marker `NO_BACKUP_OR_RESTORE_COMMAND_EXECUTED_BY_VERIFY_SCRIPT` appears;
- source inclusion count is `1`;
- current target checksum prefix is `e9ecff3d22ed`;
- no apply, rollback, sudo, backup, or restore command is run.

## No Backup Execution Requirements

Snapshot `/backup/qsl/manifests` and `/backup/qsl/logs` before and after
verify-script validation. Pass if no new manifest/log file appears during the
validation window. If a new scheduled artifact appears independently, do not
claim coverage without content-level manifest proof and Director authorization.

## No Restore Execution Requirements

No restore command, restore target creation, rollback copy, rsync restore, or
restore-like local copy operation may run. The verify script must remain
read-only.

## No Manifest-Backed Claim Requirements

The evidence must classify the result as source-list evidence only unless a
future authorized manifest/status review proves otherwise. The default
classification is:

`SOURCE_LIST_UPDATED_NOT_MANIFEST_PROVEN`

## No Durable Index Output Requirements

Validate that `/home/victor/work/qsl/codex/ops/director_state_index` and
`/home/victor/work/qsl/codex/ops/director_state_index/current/director_state_index.json`
are absent. Director State Index fixture runs must remain temp-only.

## No Helper Mutation Requirements

`scripts/ci/qsl_director_state_index.py` may be inspected and compiled
read-only. It must not be edited.

## No Fixture Mutation Requirements

`inputs/local_ops/director_state_index_fixtures/**` may be read and used by the
fixture matrix. It must not be edited.

## Public Claim Boundary Requirements

Evidence must state that NA-0407 is internal local-ops evidence only and is not
disaster recovery, off-host backup evidence, restore proof, public docs,
external review, a public technical paper, production readiness, public-internet
readiness, or a security policy update.

## Successor Selection Requirements

Select exactly:

`NA-0408 -- QSL Codex Ops Backup Coverage Manifest Verification / Status Update Plan`

only if source-list validation succeeds. If validation fails, select a conflict
or rollback successor and stop without implementing NA-0408.

## Backup-Impact Requirements

Record that:

- the local source-list now includes Codex ops;
- backup status docs were not mutated;
- backup plan was not mutated;
- manifest/status verification remains future work;
- same-host continuity remains caveated;
- no off-host or restore claim is made.

## Required Local Checks

Run or record:

- startup qstart/qresume or manual-entry recovery proof;
- D234 dirty-worktree preservation proof;
- D236 dirty draft validation proof;
- clean worktree proof after reset, or exact allowed dirty-path proof when no
  reset is required;
- branch protection and public-safety proof;
- `cargo audit --deny warnings`;
- `cargo tree -i rustls-webpki --locked`;
- queue and decision helper proof;
- Director State Index helper help/compile/fixture matrix;
- qsl-backup read-only validation;
- operator verify script;
- no-backup/no-restore manifest/log comparison;
- Codex ops path-only safety scan;
- `cargo fmt --check`;
- metadata runtime JSON parse checks where available;
- directly runnable metadata runtime no-secret harnesses where available;
- `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1`;
- `python3 formal/model_qsc_handshake_suite_id_bounded.py`;
- `python3 formal/run_model_checks.py`;
- qshield-cli test/build where feasible;
- qsc NA-0313 harness if directly runnable;
- queue/decisions;
- scope guard;
- link check;
- leak scan;
- overclaim scan;
- classifier;
- PR body preflight / goal-lint.

## CI Expectations

Open the NA-0407 PR only after local scope and validation proof is clean. Merge
only after required checks pass normally and public-safety remains required.
Do not bypass required checks, squash, rebase, amend, force-push, direct-push,
or delete the branch manually.

## Successor Handoff

After the NA-0407 PR merge, NA-0407 remains READY until optional closeout.
Optional closeout may restore NA-0408 only if the NA-0407 PR merged cleanly,
public-safety is green, D-0797 exists once, D-0798 is absent, and the selected
successor is exact.
