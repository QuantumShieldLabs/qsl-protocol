Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-06

# NA-0431 qsc Fuzz Lock Precise-Version pqcrypto Cleanup Retry Implementation Harness

Goals: G1, G2, G3, G4, G5

## Executive summary

NA-0431 implements the D-0847-authorized precise-version nested qsc fuzz lock
cleanup. The only implementation file changed is:

`qsl/qsl-client/qsc/fuzz/Cargo.lock`

The update used the exact authorized commands from the qsc fuzz workspace:

```bash
cargo update -p qsc -p quantumshield_refimpl
cargo update -p rustls-webpki --precise 0.103.13
cargo update -p rand@0.9.2 --precise 0.9.4
```

The refreshed nested lock removes `pqcrypto-mlkem`, `pqcrypto-traits`, and
`pqcrypto-internals`, updates nested `rustls-webpki` to `0.103.13`, updates the
nested `rand 0.9` line to `0.9.4`, introduces `ml-kem 0.2.3`, and preserves
`ml-dsa 0.1.0-rc.7` on the cargo-fuzz build-compatible
`pkcs8 0.11.0-rc.11` / `spki 0.8.0-rc.4` / `signature 3.0.0-rc.10` chain.

Validation passed for root cargo audit, nested fuzz lock audit, proof-root qsc
fuzz-bin build, qsc `send_commit`, provider `pqkem768`, formal model checks,
and formatting. The local `scripts/ci/qsc_adversarial.sh` Rust test phases
passed, then the script stopped because the local `cargo-fuzz` subcommand is
not installed. PR CI `qsc-adversarial-smoke` remains a required merge gate.

Selected successor after successful merge and qsc-adversarial-smoke success:

`NA-0432 -- QSL qsc Provider Error Path / No-Mutation Read-Only Audit Plan`

## Live NA-0431 scope

Live `NEXT_ACTIONS.md` lists:

`NA-0431 -- QSL qsc Fuzz Lock Precise-Version pqcrypto Cleanup Retry Implementation Harness`

Status: READY.

Allowed implementation mutation:

- `qsl/qsl-client/qsc/fuzz/Cargo.lock`

Allowed governance/evidence mutation:

- `docs/governance/evidence/NA-0431_qsl_qsc_fuzz_lock_precise_version_pqcrypto_cleanup_retry_implementation_harness.md`
- `tests/NA-0431_qsl_qsc_fuzz_lock_precise_version_pqcrypto_cleanup_retry_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Forbidden mutation scope included runtime, crypto, root Cargo, root dependency,
qsc manifest, fuzz manifest, workflow, script, fuzz target, executable test,
vector, qsl-server, qsl-attachments, qshield runtime, website, public docs,
README, START_HERE, qwork/qstart/qresume/qshell, qsl-backup, backup status,
backup plan, rollback subtree, and backup/restore paths.

Acceptance criteria:

- root cargo audit remains green;
- nested qsc fuzz lock audit becomes green;
- qsc fuzz bins build with the refreshed lock;
- qsc adversarial intent is preserved and PR `qsc-adversarial-smoke` passes
  before merge;
- pqcrypto residual is removed or explicitly classified;
- the `ml-dsa` release-candidate compatibility chain is preserved;
- no runtime, crypto, workflow, script, test, vector, manifest, or root Cargo
  mutation occurs;
- public-claim caveats remain explicit;
- exactly one READY item remains.

Stop conditions included any changed path outside the authorized set, any
lockfile-related validation failure, inability to make nested audit green, any
qsc adversarial failure related to the lockfile change, any backup/restore or
qsl-backup mutation, any qwork rerun, or any public overclaim.

## qwork proof-file verification

Codex read, but did not run, qwork proof files:

- `/srv/qbuild/work/NA-0431/.qwork/startup.qsl-protocol.kv`
- `/srv/qbuild/work/NA-0431/.qwork/startup.qsl-protocol.json`

Required `.kv` markers were present:

- `startup_result=OK`
- `lane=NA-0431`
- `repo=qsl-protocol`
- `path=/srv/qbuild/work/NA-0431/qsl-protocol`
- `head_equals_origin_main=yes`
- `worktree_clean=yes`
- `index_clean=yes`
- `untracked_clean=yes`
- `ready_count=1`
- `queue_top_ready=NA-0431`
- `requested_lane_status=READY`

The JSON proof parsed successfully and mirrored the required `.kv` values.
After `git fetch --all --prune`, live `HEAD` and `origin/main` still matched
the qwork proof at `f63626eed77a`, and `origin/main` descended from the
directive-required PR #1131 merge commit. Codex did not run qwork, qstart, or
qresume.

Proof root:

`/srv/qbuild/tmp/NA0431_qsc_fuzz_lock_precise_version_cleanup_20260606T140549Z`

The qwork proof files were copied into the proof root under `qwork/`.

## D272 / D273 inheritance

D272 completed NA-0430 and authorized the precise-version retry through
D-0847. D-0847 classified PR #1127's failed `qsc-adversarial-smoke` as a
dependency/lockfile/fuzz-tooling blocker: the broad lock refresh made nested
audit green but moved `ml-dsa 0.1.0-rc.7` onto final `pkcs8 0.11.0`,
`spki 0.8.0`, and `signature 3.0.0`, producing Rust `E0277` build errors
before fuzz execution.

D272 proof-root simulation showed that the precise-version recipe removes the
pqcrypto packages, updates nested advisory blockers, preserves the
release-candidate `ml-dsa` compatibility chain, passes nested audit, and builds
qsc fuzz bins.

D273 recovered the temporary proof-artifact boundary issue from D272, recorded
D-0849, and preserved NA-0431 as the sole READY item. D273 did not implement
NA-0431.

PR #1127 remains CLOSED and unmerged, with the branch retained at
`967c95c37fea`.

## Pre-mutation lockfile review

Preimage file SHA-256 prefixes:

- fuzz `Cargo.toml`: `815110f12fa6`
- fuzz `Cargo.lock`: `a4a3378781b7`
- root `Cargo.toml`: `72627c3442ca`
- root `Cargo.lock`: `9348e3d309db`
- qsc `Cargo.toml`: `e7b74760ee68`
- `scripts/ci/qsc_adversarial.sh`: `562933d06325`
- `.github/workflows/qsc-adversarial.yml`: `cf44378ae8d5`

Selected preimage nested lock entries:

- `pqcrypto-mlkem 0.1.1`
- `pqcrypto-traits 0.3.5`
- `pqcrypto-internals 0.2.11`
- `rustls-webpki 0.103.10`
- `rand 0.8.5`
- `rand 0.9.2`
- `ml-dsa 0.1.0-rc.7`
- `pkcs8 0.10.2`
- `pkcs8 0.11.0-rc.11`
- `spki 0.7.3`
- `spki 0.8.0-rc.4`
- `signature 2.2.0`
- `signature 3.0.0-rc.10`

Pre-mutation nested fuzz lock audit exited nonzero as expected for the inherited
red nested lock. The failure was classified as the known inherited dependency
state that NA-0431 is scoped to repair, not as a source-checkout regression.

Root `cargo audit --deny warnings` passed before mutation. Root
`rustls-webpki` was `v0.103.13`; root `ml-kem` was present; root pqcrypto
package-ID inverse-tree probes were absent.

The qsc adversarial workflow installs cargo-fuzz in CI and runs
`scripts/ci/qsc_adversarial.sh`. The script runs qsc adversarial Rust tests and
then the qsc fuzz targets from the nested fuzz workspace.

## Rollback copy proof

Rollback copy:

`/srv/qbuild/tmp/NA0431_qsc_fuzz_lock_precise_version_cleanup_20260606T140549Z/rollback/qsc-fuzz-Cargo.lock.preimage`

The rollback copy SHA-256 matched the fuzz lock preimage SHA-256 prefix
`a4a3378781b7`. The copy was readable before mutation.

No rollback was needed because the implementation and required validations
passed within the authorized scope.

## Precise-version cleanup commands

Codex switched to branch:

`na-0431-qsc-fuzz-lock-precise-version-cleanup`

From `qsl/qsl-client/qsc/fuzz`, Codex ran exactly:

```bash
cargo update -p qsc -p quantumshield_refimpl
```

Changed paths after command 1:

- `qsl/qsl-client/qsc/fuzz/Cargo.lock`

From `qsl/qsl-client/qsc/fuzz`, Codex ran exactly:

```bash
cargo update -p rustls-webpki --precise 0.103.13
```

Changed paths after command 2:

- `qsl/qsl-client/qsc/fuzz/Cargo.lock`

From `qsl/qsl-client/qsc/fuzz`, Codex ran exactly:

```bash
cargo update -p rand@0.9.2 --precise 0.9.4
```

Changed paths after command 3:

- `qsl/qsl-client/qsc/fuzz/Cargo.lock`

No broad source-checkout `cargo update`, root workspace update,
`cargo generate-lockfile`, manifest edit, workflow edit, script edit, source
edit, fuzz target edit, test edit, or vector edit was performed.

## Lockfile diff summary

Diffstat after the lock update:

```text
qsl/qsl-client/qsc/fuzz/Cargo.lock | 953 ++++---------------------------------
1 file changed, 99 insertions(+), 854 deletions(-)
```

After-update nested lock entries:

- `ml-dsa 0.1.0-rc.7`
- `ml-kem 0.2.3`
- `pkcs8 0.10.2`
- `pkcs8 0.11.0-rc.11`
- `rand 0.9.4`
- `rustls-webpki 0.103.13`
- `signature 2.2.0`
- `signature 3.0.0-rc.10`
- `spki 0.7.3`
- `spki 0.8.0-rc.4`

The nested lock no longer contains package entries for `pqcrypto-mlkem`,
`pqcrypto-traits`, or `pqcrypto-internals`.

## Nested fuzz lock audit proof

Command:

```bash
cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock
```

Result: passed after the precise-version cleanup.

This is dependency-health evidence for the nested fuzz lock only. It is not
public readiness proof. It is not external-review proof. It is not
crypto-complete proof. It is not side-channel-free proof. It is not bug-free
proof. It is not vulnerability-free proof. It is not perfect-crypto proof.

## Root audit proof

Command:

```bash
cargo audit --deny warnings
```

Result: passed before and after the lockfile change.

Root `cargo tree -i rustls-webpki --locked` reported `rustls-webpki v0.103.13`.
Root `cargo tree -i ml-kem --locked` reported `ml-kem v0.2.1`. Root
`pqcrypto-mlkem`, `pqcrypto-traits`, and `pqcrypto-internals` package-ID probes
reported absence.

## pqcrypto residual removal proof

Command:

```bash
rg -n "pqcrypto-mlkem|pqcrypto-traits|pqcrypto-internals" qsl/qsl-client/qsc/fuzz/Cargo.lock
```

Result: zero matches after the cleanup.

## ml-dsa / pkcs8 compatibility proof

After the precise-version cleanup, the nested lock keeps:

- `ml-dsa 0.1.0-rc.7`
- `pkcs8 0.11.0-rc.11`
- `spki 0.8.0-rc.4`
- `signature 3.0.0-rc.10`

The proof-root qsc fuzz-bin build passed with the refreshed lock:

```bash
CARGO_TARGET_DIR="$PROOF_DIR/cargo_target/qsc_fuzz_bins" \
  cargo +nightly build --locked \
  --manifest-path qsl/qsl-client/qsc/fuzz/Cargo.toml --bins
```

Result: passed.

## qsc adversarial / fuzz validation

Required local validations passed:

- `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1`
- `cargo test -p quantumshield_refimpl --features pqcrypto --locked --test pqkem768`
- `python3 formal/model_qsc_handshake_suite_id_bounded.py`
- `python3 formal/run_model_checks.py`
- `cargo fmt --check`

Local `scripts/ci/qsc_adversarial.sh` status:

- qsc adversarial properties Rust test: passed, 8 tests;
- qsc adversarial miri Rust test: passed, 6 tests;
- script then exited 101 because local cargo does not have the `fuzz`
  subcommand installed.

Classification: local tooling availability caveat, not a lockfile/dependency
or fuzz-bin build failure. This classification is accepted only because nested
audit, root audit, qsc fuzz-bin build, qsc `send_commit`, provider `pqkem768`,
and formal checks passed locally.

PR CI `qsc-adversarial-smoke` must pass before merge.

## No runtime / crypto / workflow / test / vector mutation proof

Implementation changed paths before governance evidence:

- `qsl/qsl-client/qsc/fuzz/Cargo.lock`

After the lockfile update, SHA-256 prefixes for protected files remained:

- fuzz `Cargo.toml`: `815110f12fa6`
- root `Cargo.toml`: `72627c3442ca`
- root `Cargo.lock`: `9348e3d309db`
- qsc `Cargo.toml`: `e7b74760ee68`
- `scripts/ci/qsc_adversarial.sh`: `562933d06325`
- `.github/workflows/qsc-adversarial.yml`: `cf44378ae8d5`

No runtime code, crypto implementation, workflow, script, fuzz target,
executable test, vector, root Cargo, qsc Cargo, fuzz manifest, qsl-server,
qsl-attachments, qshield runtime, public docs, README, START_HERE, or website
path was changed.

## Public claim / external review / website boundary

This cleanup is internal engineering and dependency-hygiene evidence for the
nested qsc fuzz lock. It is not production readiness proof. It is not
public-internet readiness proof. It is not external-review completion proof. It
is not crypto-complete proof. It is not side-channel-free proof. It is not
bug-free proof. It is not vulnerability-free proof. It is not perfect-crypto
proof. It is not metadata-free behavior proof. It is not anonymity proof. It is
not untraceability proof. It is not off-host backup completion proof. It is not
disaster-recovery completion proof. It is not restore proof. It is not backup
completion proof.

No README, START_HERE, public doc, website, or public technical paper content
was changed.

## Rejected alternatives

- Broad source-checkout `cargo update`: rejected because PR #1127 showed a broad
  lock refresh could make nested audit green while breaking qsc fuzz-bin build
  compatibility.
- Manifest constraints in fuzz `Cargo.toml`: rejected because the precise
  lockfile-only recipe passed without changing the manifest.
- Fuzz target, runtime, crypto, workflow, or script mutation: rejected because
  the blocker was remediated by the authorized nested lock change.
- Waiver or advisory ignore: rejected because nested audit is green after the
  precise lockfile cleanup.

## Backup-impact statement

Codex did not run backup or restore. Codex did not run sudo. Codex did not
mutate qsl-backup, `/backup/qsl`, backup status files, backup plan files,
rollback subtree paths, timers, fstab, source lists, retention, or backup
scripts.

The qsl-backup source checksum matched the directive-required value, and the
Codex ops source-list inclusion count remained exactly 1.

## Selected successor

Normal successor after successful merge and qsc-adversarial-smoke success:

`NA-0432 -- QSL qsc Provider Error Path / No-Mutation Read-Only Audit Plan`

NA-0432 is not implemented by this PR. Queue transition to NA-0432 is reserved
for the optional closeout after this implementation PR merges and required
post-merge checks pass.

## Next recommendation

After this PR merges with public-safety and qsc-adversarial-smoke green, run the
optional closeout that marks NA-0431 DONE and restores the selected NA-0432
read-only provider-error/no-mutation audit lane as the sole READY item.
