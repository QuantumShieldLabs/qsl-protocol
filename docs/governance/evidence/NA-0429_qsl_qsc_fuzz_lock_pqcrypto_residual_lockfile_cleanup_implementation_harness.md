Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-05

# NA-0429 QSL qsc Fuzz Lock pqcrypto Residual Lockfile Cleanup Implementation Harness

Goals: G1, G2, G3, G4, G5

## Executive summary

NA-0429 implements the NA-0428-authorized lockfile-only cleanup for the
separate qsc cargo-fuzz workspace lock:

`qsl/qsl-client/qsc/fuzz/Cargo.lock`

The refresh removed `pqcrypto-mlkem`, `pqcrypto-traits`, and
`pqcrypto-internals` from the nested fuzz lock, raised nested
`rustls-webpki` to `0.103.13`, introduced nested `ml-kem 0.2.3`, and made the
nested fuzz lock pass `cargo audit --deny warnings --file
qsl/qsl-client/qsc/fuzz/Cargo.lock`.

Root `cargo audit --deny warnings` remained green. Root `Cargo.toml`, root
`Cargo.lock`, `qsl/qsl-client/qsc/Cargo.toml`, and
`qsl/qsl-client/qsc/fuzz/Cargo.toml` were unchanged by SHA. No runtime, crypto,
workflow, script, fuzz target, test, vector, service, website, public-doc,
README, or START_HERE path was mutated.

The selected successor is:

`NA-0430 -- QSL qsc Provider Error Path / No-Mutation Read-Only Audit Plan`

## Live NA-0429 scope

Live `NEXT_ACTIONS.md` lists:

`NA-0429 -- QSL qsc Fuzz Lock pqcrypto Residual Lockfile Cleanup Implementation Harness`

Status: READY.

Allowed implementation mutation path:

- `qsl/qsl-client/qsc/fuzz/Cargo.lock`

Allowed governance/evidence mutation paths:

- `docs/governance/evidence/NA-0429_qsl_qsc_fuzz_lock_pqcrypto_residual_lockfile_cleanup_implementation_harness.md`
- `tests/NA-0429_qsl_qsc_fuzz_lock_pqcrypto_residual_lockfile_cleanup_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Forbidden mutation scope preserved:

- no `qsl/qsl-client/qsc/fuzz/Cargo.toml` mutation;
- no root `Cargo.toml` or root `Cargo.lock` mutation;
- no runtime or crypto implementation mutation;
- no workflow or script mutation;
- no fuzz target source mutation;
- no test or vector mutation outside the NA-0429 governance testplan;
- no qsl-server, qsl-attachments, qshield runtime, website, public-doc, README,
  or START_HERE mutation;
- no qwork, qstart, qresume, qshell, qsl-backup, backup status, backup plan,
  rollback subtree, or `/backup/qsl` mutation.

Acceptance criteria:

- root cargo audit green;
- nested qsc fuzz lock audit green;
- pqcrypto residual removed or explicitly explained;
- qsc adversarial intent preserved;
- no runtime, crypto, root dependency, workflow, fuzz target, test, vector,
  service, public, backup, or local-ops mutation;
- public-claim caveats explicit;
- public-safety green before merge and after merge;
- exactly one READY item remains.

Stop conditions preserved:

- qwork proof files missing, malformed, stale, or inconsistent;
- qwork, qstart, or qresume run by Codex;
- PR #1126 not merged at expected lineage;
- queue not READY NA-0429 at start;
- D-0845 absent or D-0846 already present at start;
- root cargo audit not green;
- nested fuzz lock audit cannot be made green by lockfile-only cleanup;
- lock refresh changes any implementation path besides
  `qsl/qsl-client/qsc/fuzz/Cargo.lock`;
- qsc adversarial failure appears related to dependency, lockfile, or fuzz
  tooling semantics rather than local environment availability;
- successor cannot be selected safely;
- forbidden mutation or unsupported public claim introduced.

## qwork proof-file verification

Codex read, but did not run, qwork proof files:

- `/srv/qbuild/work/NA-0429/.qwork/startup.qsl-protocol.kv`
- `/srv/qbuild/work/NA-0429/.qwork/startup.qsl-protocol.json`

The `.kv` proof reported the required values:

- `startup_result=OK`
- `lane=NA-0429`
- `repo=qsl-protocol`
- `path=/srv/qbuild/work/NA-0429/qsl-protocol`
- `head_equals_origin_main=yes`
- `worktree_clean=yes`
- `index_clean=yes`
- `untracked_clean=yes`
- `ready_count=1`
- `queue_top_ready=NA-0429`
- `requested_lane_status=READY`

The JSON proof parsed successfully and mirrored the required `.kv` fields for
lane, repo, path, head, origin/main, clean state, READY count, queue-top READY,
and requested-lane status.

After `git fetch --all --prune`, live `HEAD` and `origin/main` still matched
the qwork proof at `c621ff09df61`. PR #1126 was verified MERGED with merge
commit `c621ff09df61`. Public-safety on that commit completed success and is a
required branch-protection context.

Proof root:

`/srv/qbuild/tmp/NA0429_qsc_fuzz_lock_pqcrypto_cleanup_20260605T185313-0500`

The qwork proof files were copied into the proof root under `qwork/`.

## NA-0428 authorization inheritance

NA-0428 classified the nested qsc fuzz lock blocker as:

`FUZZ_LOCK_REMEDIATION_AUTHORIZED_LOCKFILE_ONLY`

Inherited facts:

- root `cargo audit --deny warnings` was green;
- root `rustls-webpki` was `v0.103.13`;
- root `ml-kem` was active;
- root `pqcrypto-mlkem`, `pqcrypto-traits`, and
  `pqcrypto-internals` package IDs were absent from the locked graph;
- the separate committed qsc fuzz lock was audit-red before NA-0429;
- the separate committed qsc fuzz lock contained `pqcrypto-mlkem 0.1.1`,
  `pqcrypto-traits 0.3.5`, `pqcrypto-internals 0.2.11`, and
  `rustls-webpki 0.103.10`;
- `qsl/qsl-client/qsc/fuzz/Cargo.toml` is a real cargo-fuzz workspace;
- `scripts/ci/qsc_adversarial.sh` and
  `.github/workflows/qsc-adversarial.yml` wire qsc adversarial/fuzz checks into
  active tooling;
- NA-0428 authorized future implementation mutation only for
  `qsl/qsl-client/qsc/fuzz/Cargo.lock`.

## Pre-mutation lockfile review

Preimage SHA proof:

- `qsl/qsl-client/qsc/fuzz/Cargo.toml`:
  `815110f12fa6443285bb2997b8c8c01ac221f487dfb8c0e0fd3b907f878b5db6`
- `qsl/qsl-client/qsc/fuzz/Cargo.lock`:
  `a4a3378781b7ce88a556fad897ec53c90c870e095b6353b57c0a2de990e6770a`
- root `Cargo.toml`:
  `72627c3442cadadf585e65fd83cc26e23db71fd256606a20410a6984318d4532`
- root `Cargo.lock`:
  `9348e3d309db252444afbf3155761f978fd28c4f0671556cd97d479add8202bd`
- `qsl/qsl-client/qsc/Cargo.toml`:
  `e7b74760ee685961ff16a4dd0c80a02d4b9025ad1046d66be528e8770c98c21f`

Preimage nested lock package entries:

- `pqcrypto-mlkem 0.1.1`
- `pqcrypto-traits 0.3.5`
- `pqcrypto-internals 0.2.11`
- `rustls-webpki 0.103.10`
- `ml-kem` absent

Pre-mutation nested audit:

`cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock`

Result: red before mutation with denied findings including
`RUSTSEC-2026-0098`, `RUSTSEC-2026-0099`, `RUSTSEC-2026-0104`,
`RUSTSEC-2026-0161`, `RUSTSEC-2026-0162`, `RUSTSEC-2026-0163`, and
`RUSTSEC-2026-0097`.

Pre-mutation root audit:

`cargo audit --deny warnings`

Result: passed.

## Rollback copy proof

Rollback copy:

`/srv/qbuild/tmp/NA0429_qsc_fuzz_lock_pqcrypto_cleanup_20260605T185313-0500/rollback/qsc-fuzz-Cargo.lock.preimage`

Rollback SHA:

`a4a3378781b7ce88a556fad897ec53c90c870e095b6353b57c0a2de990e6770a`

The rollback copy SHA matched the preimage SHA and the rollback copy was
readable before mutation.

## Lockfile-only cleanup command

Command:

```bash
cd qsl/qsl-client/qsc/fuzz
cargo update
```

No Cargo manifest, root lock, workflow, script, runtime, crypto, fuzz target,
test, vector, service, public, backup, or qwork path was changed by this
command.

Immediate changed-path proof after the refresh:

- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `qsl/qsl-client/qsc/fuzz/Cargo.lock`

The only changed implementation path was the authorized nested fuzz lock.

## Lockfile diff summary

Saved diff:

`/srv/qbuild/tmp/NA0429_qsc_fuzz_lock_pqcrypto_cleanup_20260605T185313-0500/diffs/qsc-fuzz-Cargo.lock.diff`

Diff size:

- 2,278 diff lines saved in proof root;
- `1 file changed, 309 insertions(+), 1009 deletions(-)`.

After SHA:

`fd7cfd20f9d912004f7ee90750abed785fc518df066adb17f85ea781c9a5a0d5`

## Nested fuzz lock audit proof

Command:

`cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock`

Result after lock refresh: passed.

The nested fuzz lock now has 293 crate dependencies under audit, down from 362
before refresh, and no denied audit warnings were reported.

## Root audit proof

Command:

`cargo audit --deny warnings`

Result after lock refresh: passed.

The root workspace lock remained at 375 crate dependencies under audit and was
not mutated.

## pqcrypto residual removal proof

Command:

`rg -n "pqcrypto-mlkem|pqcrypto-traits|pqcrypto-internals" qsl/qsl-client/qsc/fuzz/Cargo.lock`

Result after lock refresh: zero matches.

TOML-parsed nested lock after-state:

- `pqcrypto-mlkem`: absent
- `pqcrypto-traits`: absent
- `pqcrypto-internals`: absent

Root inverse-tree probes for `pqcrypto-mlkem`, `pqcrypto-traits`, and
`pqcrypto-internals` also reported package-ID absence through zero-failure-safe
command shape.

## rustls-webpki / ml-kem state proof

TOML-parsed nested lock after-state:

- `rustls-webpki 0.103.13`
- `ml-kem 0.2.3`
- `rand 0.9.4`

Root inverse tree after-state:

- root `cargo tree -i rustls-webpki --locked` reported
  `rustls-webpki v0.103.13`;
- root `cargo tree -i ml-kem --locked` reported `ml-kem v0.2.1` active through
  `quantumshield_refimpl`, `qsc`, `qsl-tui`, and `refimpl_actor`.

## qsc adversarial validation

Stable adversarial tests from `sh scripts/ci/qsc_adversarial.sh` passed:

- `adversarial_properties`: 8 passed;
- `adversarial_miri`: 6 passed.

The fuzz-target portion was attempted but was locally unavailable:

- direct `scripts/ci/qsc_adversarial.sh` exited 126 because the script is not
  executable in the worktree;
- rerun via `sh scripts/ci/qsc_adversarial.sh` reached the fuzz phase and then
  `cargo +nightly fuzz` exited 101 with `error: no such command: fuzz`;
- proof root records `CARGO_FUZZ_AVAILABILITY_RC=101`.

Classification: local tooling/environment availability issue. This is not a
dependency, lockfile, fuzz target, or workflow regression from the lock refresh.
Codex did not install cargo-fuzz and did not mutate script permissions.

Proceeding is supported because root audit, nested fuzz lock audit, qsc
`send_commit`, refimpl provider test, and formal checks passed.

## No runtime / crypto / root dependency / workflow / test / vector mutation proof

Changed paths before commit are limited to:

- `qsl/qsl-client/qsc/fuzz/Cargo.lock`
- `docs/governance/evidence/NA-0429_qsl_qsc_fuzz_lock_pqcrypto_residual_lockfile_cleanup_implementation_harness.md`
- `tests/NA-0429_qsl_qsc_fuzz_lock_pqcrypto_residual_lockfile_cleanup_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

SHA comparison after refresh proves these files unchanged:

- `qsl/qsl-client/qsc/fuzz/Cargo.toml`
- root `Cargo.toml`
- root `Cargo.lock`
- `qsl/qsl-client/qsc/Cargo.toml`

Diff check over `scripts/ci/qsc_adversarial.sh` and
`.github/workflows/qsc-adversarial.yml` was empty. No fuzz target source,
runtime source, crypto source, qsl-server, qsl-attachments, qshield runtime,
website, public-doc, README, or START_HERE path changed.

## Public claim / external review / website boundary

This cleanup is internal engineering and dependency-health evidence only.

No production-readiness claim is made. No public-internet-readiness claim is
made. No external-review-complete claim is made. No crypto-complete claim is
made. No side-channel-free claim is made. No bug-free claim is made. No
vulnerability-free claim is made. No perfect-crypto claim is made. No
metadata-free claim is made. No anonymity claim is made. No untraceability claim
is made. No off-host-backup-complete claim is made. No disaster-recovery
completion claim is made. No restore-proven claim is made. No backup-complete
claim is made.

Cargo audit green is dependency-health evidence only and not a proof that all
defects or all vulnerabilities are absent.

No public technical paper content, README, START_HERE, website, or public-doc
path was changed.

## Rejected alternatives

- Fuzz `Cargo.toml` mutation rejected: the lock refresh succeeded without
  changing the fuzz manifest.
- Root Cargo mutation rejected: the root workspace was already audit-green and
  the root lock was unchanged.
- Runtime or crypto implementation mutation rejected: the issue was confined to
  the separate nested fuzz lock.
- Workflow or script mutation rejected: qsc adversarial linkage remained intact
  and local fuzz unavailability is a tool-installation issue, not a workflow
  semantic issue.
- Audit waiver rejected: lockfile-only cleanup removed the denied nested
  findings instead.
- Installing cargo-fuzz locally rejected: outside the declared mutation scope.

## Backup-impact statement

Codex did not run backup or restore. Codex did not run sudo. Codex did not
mutate qsl-backup, `/backup/qsl`, backup logs, backup manifests, backup status
files, backup plan files, rollback subtree paths, systemd, timers, fstab,
source lists, retention settings, or backup scripts.

The qsl-backup SHA remained:

`e9ecff3d22eda21ceb0e889e4dd5d6f4e270e09349c77a1f4872bfc0052f6232`

The qsl-backup Codex ops source inclusion count remained exactly `1`.

## Selected successor

Normal successor selected:

`NA-0430 -- QSL qsc Provider Error Path / No-Mutation Read-Only Audit Plan`

Reason: lock cleanup succeeded, nested and root audits are green, pqcrypto
residuals were removed, and the only qsc adversarial limitation was local
cargo-fuzz subcommand availability. The remaining higher-value follow-up is the
provider-error/no-mutation read-only audit preserved by NA-0428/NA-0429
sequencing.

## Next recommendation

After the NA-0429 implementation PR merges and post-merge public-safety is
green, close out NA-0429 and restore the selected NA-0430 read-only audit lane.
Do not implement NA-0430 in the NA-0429 closeout.
