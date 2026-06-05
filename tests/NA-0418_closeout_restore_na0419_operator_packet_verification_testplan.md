Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-04

# NA-0418 Closeout Restore NA-0419 Operator Packet Verification Testplan

Goals: G4

## Objective

Validate the closeout-only lane that marks NA-0418 DONE after PR #1105
dependency-health remediation is merged and proven green, then restores
`NA-0419 -- QSL Backup Log Code 23 Operator Packet Execution Verification
Resume` as the sole READY successor.

This testplan does not validate operator-packet execution. NA-0419 owns that
verification.

## Protected Invariants

- qwork proof-file handoff is verified without running qwork, qstart, or
  qresume.
- PR #1105 is merged before closeout.
- `public-safety` is green on the PR #1105 remediation merge commit or current
  `origin/main` descendant.
- `cargo audit --deny warnings` is green on merged main.
- Former pqcrypto unmaintained package IDs are absent from the root workspace
  tree, or cargo audit is green without the pqcrypto RustSec blockers.
- NA-0418 is DONE only after dependency-health and public-safety proof.
- NA-0419 is the exact sole READY successor.
- Operator-packet verification remains pending.
- qsl-backup remains unchanged.
- Rollback evidence is preserved for NA-0419.
- No public overclaim is introduced.

## Allowed Scope

Allowed qsl-protocol mutation paths:

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0418_closeout_restore_na0419_operator_packet_verification_testplan.md`

Allowed read-only local paths:

- `/srv/qbuild/work/NA-0418/.qwork/startup.qsl-protocol.kv`
- `/srv/qbuild/work/NA-0418/.qwork/startup.qsl-protocol.json`
- `/srv/qbuild/tmp/NA0418_code23_root_operator_packet_20260604T092447-05-00`
- `/srv/qbuild/tmp/NA0418_code23_root_operator_packet_20260604T092447-05-00/operator_result`
- `/srv/qbuild/tmp/NA0407_qsl_backup_root_action_20260602T232945-0500/rollback`
- `/backup/qsl/manifests`
- `/backup/qsl/logs`
- `/usr/local/sbin/qsl-backup`
- `/home/victor/work/qsl/codex/ops/backup/QSL_BACKUP_STATUS.md`
- `/home/victor/work/qsl/codex/QSL_BACKUP_PLAN.md`

Allowed proof output root:

- `/srv/qbuild/tmp/NA0418_rustsec_remediation_closeout_<timestamp>/`

## Forbidden Scope

- Running qwork, qstart, or qresume by Codex.
- Running sudo by Codex.
- Running generated operator packet scripts by Codex.
- Running backup.
- Running restore.
- Mutating `/usr/local/sbin/qsl-backup`.
- Mutating `/backup/qsl`.
- Mutating the NA-0407 rollback subtree.
- Mutating backup status or backup plan files.
- Mutating qwork/qstart/qresume/qshell.
- Mutating runtime, crypto, dependency, workflow, qsl-server, qsl-attachments,
  qshield runtime, website, public docs, README, or START_HERE paths.
- Creating durable Director State Index output.
- Creating public technical paper content.
- Expanding public claims beyond the evidence recorded by this closeout.

## D257 Dependency Remediation Dependency

Required inherited facts from D257 / PR #1105:

- PR #1105 merged.
- D-0824 exists once.
- The cargo audit blocker was real.
- `pqcrypto-mlkem`, `pqcrypto-traits`, and `pqcrypto-internals` were
  runtime/security-critical reachable through the owned provider boundary.
- Audit waiver was rejected.
- ML-KEM provider replacement merged.
- qsc render compile ambiguity fix merged.
- Cargo audit was green after remediation.
- Operator-packet verification remained pending.

## Public-Safety Green Gate

Required command:

```bash
gh api "/repos/QuantumShieldLabs/qsl-protocol/commits/<merge-or-main-sha>/check-runs?per_page=100" -H "Accept: application/vnd.github+json"
```

Acceptance:

- `public-safety` exists.
- `public-safety` status is `completed`.
- `public-safety` conclusion is `success`.
- If `public-safety` is still in progress, bounded REST polling is allowed.
- If `public-safety` fails or remains incomplete after the bounded wait, stop
  without patching or merging.

## Cargo Audit Green Gate

Required commands:

```bash
cargo audit --deny warnings
cargo tree -i rustls-webpki --locked
cargo tree -i pqcrypto-mlkem --locked || true
cargo tree -i pqcrypto-traits --locked || true
cargo tree -i pqcrypto-internals --locked || true
```

Acceptance:

- `cargo audit --deny warnings` exits 0.
- `rustls-webpki` is `v0.103.13` or newer safe version.
- The pqcrypto inverse-tree commands prove package absence, or cargo audit is
  green without the pqcrypto RustSec blockers.

## NA-0419 Block Requirements

The restored NA-0419 block must include:

- Status: READY.
- Goals: G1, G2, G3, G4, G5.
- Operator result verification objective.
- qsl-backup unchanged proof requirement.
- Rollback post-action state requirement.
- Same-host continuity caveat.
- No backup, restore, or sudo by Codex.
- No generated packet script execution by Codex.
- No qsl-backup mutation.
- No rollback subtree mutation by Codex.
- No public overclaim.
- Exactly one READY item.

## No Backup / No Restore / No Generated Packet Execution Requirements

Validation must record that Codex did not:

- run backup;
- run restore;
- run sudo;
- run generated operator packet scripts;
- mutate qsl-backup;
- mutate `/backup/qsl`;
- mutate the rollback subtree.

Read-only inspection may record whether `operator_result` exists and the current
rollback directory owner/mode.

## No Public Overclaim Requirements

Closeout evidence must stay limited to:

- dependency-health restored;
- PR #1105 merged;
- public-safety green;
- cargo audit green;
- NA-0419 restored as pending operator-packet verification.

Closeout evidence must not expand readiness, external-review, backup,
recovery, restore, or universal-defect-absence claims.

## Queue / Decision Validation

Before patch:

- READY_COUNT 1.
- READY NA-0418.
- D-0824 exists once.
- D-0825 absent.
- Duplicate decision count 0.

After patch and before PR:

- READY_COUNT 1.
- READY NA-0419.
- NA-0418 DONE.
- D-0825 exists once.
- D-0826 absent.
- Duplicate decision count 0.
- Changed paths are exactly the five allowed closeout paths.

## CI / Public-Safety Expectations

Local validation:

```bash
git diff --check
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
python3 scripts/ci/qsl_evidence_helper.py link-check
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
cargo audit --deny warnings
cargo fmt --check
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
python3 formal/model_qsc_handshake_suite_id_bounded.py
python3 formal/run_model_checks.py
```

PR expectations:

- PR body includes `Goals: G4`.
- Goal lint passes.
- Required checks pass before merge.
- Merge uses a merge commit.
- Post-merge queue proof shows READY NA-0419.
- Post-merge decision proof shows D-0825 on main.
- Post-merge cargo audit remains green.
- Post-merge public-safety is green on the closeout merge commit.
