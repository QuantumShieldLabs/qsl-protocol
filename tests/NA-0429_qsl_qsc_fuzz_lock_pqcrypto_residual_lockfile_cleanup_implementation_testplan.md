Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-05

# NA-0429 qsc Fuzz Lock pqcrypto Residual Lockfile Cleanup Implementation Testplan

Goals: G1, G2, G3, G4, G5

## Purpose

Validate that NA-0429 performs only the NA-0428-authorized nested qsc fuzz lock
cleanup, removes the pqcrypto residual from
`qsl/qsl-client/qsc/fuzz/Cargo.lock`, restores nested fuzz lock audit health,
preserves root dependency health, and avoids runtime, crypto, root Cargo,
workflow, script, fuzz target, test, vector, service, public, backup, and
qwork-tool mutation.

## Preconditions

- qwork proof files exist and are read without running qwork:
  - `/srv/qbuild/work/NA-0429/.qwork/startup.qsl-protocol.kv`
  - `/srv/qbuild/work/NA-0429/.qwork/startup.qsl-protocol.json`
- qwork proof fields report lane NA-0429, repo qsl-protocol, clean worktree,
  READY_COUNT 1, queue top READY NA-0429, requested lane status READY, and
  matching head/origin_main.
- Live `HEAD` and `origin/main` match qwork proof after fetch.
- PR #1126 is MERGED at `c621ff09df61`.
- Queue helper reports READY_COUNT 1 and READY NA-0429.
- NA-0428 is DONE.
- Decision helper reports latest D-0845 and duplicate count zero.
- D-0844 exists once, D-0845 exists once, and D-0846 is absent at start.
- public-safety is required and green on current `origin/main`.
- Root `cargo audit --deny warnings` is green.
- `cargo tree -i rustls-webpki --locked` reports `rustls-webpki v0.103.13` or
  newer safe version.
- Root pqcrypto package IDs are absent from the locked graph.
- The pre-mutation nested fuzz lock contains `pqcrypto-mlkem`,
  `pqcrypto-traits`, and `pqcrypto-internals`.
- A rollback copy of `qsl/qsl-client/qsc/fuzz/Cargo.lock` exists and matches
  the preimage SHA before mutation.

## Allowed scope

Allowed changed paths for the NA-0429 implementation PR:

- `qsl/qsl-client/qsc/fuzz/Cargo.lock`
- `docs/governance/evidence/NA-0429_qsl_qsc_fuzz_lock_pqcrypto_residual_lockfile_cleanup_implementation_harness.md`
- `tests/NA-0429_qsl_qsc_fuzz_lock_pqcrypto_residual_lockfile_cleanup_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

No other qsl-protocol paths may change.

## Forbidden scope

Forbidden changes include:

- `qsl/qsl-client/qsc/fuzz/Cargo.toml`;
- root `Cargo.toml` or root `Cargo.lock`;
- `qsl/qsl-client/qsc/Cargo.toml`;
- runtime code;
- crypto implementation code;
- workflows or scripts;
- fuzz target source;
- tests or vectors outside this governance testplan;
- qsl-server, qsl-attachments, qshield runtime, website, public docs, README,
  or START_HERE;
- qwork, qstart, qresume, or qshell;
- qsl-backup;
- backup status or backup plan files;
- `/backup/qsl`;
- rollback subtree paths;
- public technical paper content;
- secret material.

Forbidden public assurance claims include no production-readiness claim, no
public-internet-readiness claim, no external-review-complete claim, no
crypto-complete claim, no side-channel-free claim, no metadata-free claim, no
anonymity claim, no untraceability claim, no off-host-backup-complete claim, no
disaster-recovery completion claim, no restore-proven claim, no backup-complete
claim, no vulnerability-free claim, no bug-free claim, and no perfect-crypto
claim.

## Required evidence assertions

Verify the evidence doc includes:

- Executive summary.
- Live NA-0429 scope.
- qwork proof-file verification.
- NA-0428 authorization inheritance.
- Pre-mutation lockfile review.
- Rollback copy proof.
- Lockfile-only cleanup command.
- Lockfile diff summary.
- Nested fuzz lock audit proof.
- Root audit proof.
- pqcrypto residual removal proof.
- rustls-webpki / ml-kem state proof.
- qsc adversarial validation.
- No runtime/crypto/root dependency/workflow/test/vector mutation proof.
- Public claim/external review/website boundary.
- Rejected alternatives.
- Backup-impact statement.
- Selected successor.
- Next recommendation.

## Dependency assertions

Verify:

- `cargo update` was run from `qsl/qsl-client/qsc/fuzz`;
- the only changed implementation path is `qsl/qsl-client/qsc/fuzz/Cargo.lock`;
- nested `cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock`
  passes after refresh;
- root `cargo audit --deny warnings` passes after refresh;
- nested fuzz lock no longer contains `pqcrypto-mlkem`, `pqcrypto-traits`, or
  `pqcrypto-internals`;
- nested fuzz lock contains `rustls-webpki 0.103.13` or newer safe version;
- nested fuzz lock contains `ml-kem`;
- root `Cargo.toml`, root `Cargo.lock`, `qsl/qsl-client/qsc/Cargo.toml`, and
  `qsl/qsl-client/qsc/fuzz/Cargo.toml` are unchanged by SHA;
- root pqcrypto package IDs remain absent;
- root `rustls-webpki` remains `v0.103.13` or newer safe version.

## Required decision assertions

Verify D-0846 exists once after the patch and states:

- exact lockfile path updated:
  `qsl/qsl-client/qsc/fuzz/Cargo.lock`;
- nested fuzz lock audit green result;
- root cargo audit green result;
- pqcrypto residual removed or explained;
- `qsl/qsl-client/qsc/fuzz/Cargo.toml` unchanged;
- root `Cargo.toml` and root `Cargo.lock` unchanged;
- no runtime, crypto, workflow, test, or vector mutation;
- no backup or restore;
- no public crypto-complete claim;
- no vulnerability-free or perfect-crypto claim;
- selected NA-0430 successor;
- exactly one READY remains mandatory.

## Validation commands

Run before PR:

```bash
git diff --check
python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main --head HEAD \
  --allowed qsl/qsl-client/qsc/fuzz/Cargo.lock \
  --allowed docs/governance/evidence/NA-0429_qsl_qsc_fuzz_lock_pqcrypto_residual_lockfile_cleanup_implementation_harness.md \
  --allowed tests/NA-0429_qsl_qsc_fuzz_lock_pqcrypto_residual_lockfile_cleanup_implementation_testplan.md \
  --allowed DECISIONS.md \
  --allowed TRACEABILITY.md \
  --allowed docs/ops/ROLLING_OPERATIONS_JOURNAL.md \
  --forbidden .github/ \
  --forbidden Cargo.toml \
  --forbidden Cargo.lock \
  --forbidden qsl/qsl-client/qsc/fuzz/Cargo.toml \
  --forbidden qsl/qsl-client/qsc/fuzz/fuzz_targets/ \
  --forbidden qsl/qsl-client/qsc/Cargo.toml \
  --forbidden qsl-server/ \
  --forbidden qsl-attachments/ \
  --forbidden apps/ \
  --forbidden website/ \
  --forbidden README.md \
  --forbidden START_HERE.md
python3 scripts/ci/qsl_evidence_helper.py link-check --root .
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
cargo audit --deny warnings
cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock
cargo tree -i rustls-webpki --locked
cargo tree -i ml-kem --locked || true
cargo tree -i pqcrypto-mlkem --locked || true
cargo tree -i pqcrypto-traits --locked || true
cargo tree -i pqcrypto-internals --locked || true
rg -n "pqcrypto-mlkem|pqcrypto-traits|pqcrypto-internals" qsl/qsl-client/qsc/fuzz/Cargo.lock || true
cargo fmt --check
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
cargo test -p quantumshield_refimpl --features pqcrypto --locked --test pqkem768
python3 formal/model_qsc_handshake_suite_id_bounded.py
python3 formal/run_model_checks.py
scripts/ci/qsc_adversarial.sh
```

If `scripts/ci/qsc_adversarial.sh` is locally unavailable because the script is
not executable, rerun with `sh scripts/ci/qsc_adversarial.sh` and record the
exact outcome. If the stable adversarial tests pass but `cargo +nightly fuzz`
is unavailable because the `cargo-fuzz` subcommand is not installed, record the
exact reason and proceed only if root audit, nested audit, qsc send_commit,
refimpl provider test, and formal checks pass.

Additional checks:

- exact changed-path guard for the six allowed NA-0429 paths;
- added-line overclaim scan;
- classifier proof;
- PR body preflight;
- goal-lint with `Goals: G1, G2, G3, G4, G5`;
- required checks, including public-safety, green before merge;
- post-merge public-safety green after merge.

## Acceptance criteria

- READY_COUNT 1.
- READY NA-0429 before implementation PR merge.
- D-0844 exists once.
- D-0845 exists once.
- D-0846 exists once after the patch.
- D-0847 absent before optional closeout.
- Duplicate decision count 0.
- Only the six allowed implementation/governance paths changed.
- Nested fuzz lock audit green.
- Root cargo audit green.
- pqcrypto residual removed from nested fuzz lock.
- No runtime, crypto, root Cargo, workflow, script, fuzz target, test, vector,
  public, service, backup, or qwork-tool mutation.
- No public overclaim.
- Selected successor is safe and exact.

## Post-fix hardening review checklist

- Correctness under stress: nested audit, root audit, qsc send_commit, refimpl
  provider test, and formal checks all pass after the lock refresh.
- Minimality: only the nested fuzz lock and NA-0429 governance/evidence paths
  change.
- Maintainability: the fix is a standard lockfile refresh and does not add a
  new abstraction or special-case waiver.
- Coverage quality: validations would fail if pqcrypto residuals remained,
  nested audit stayed red, root audit regressed, or forbidden paths changed.
- Cross-lane stability: Linux local validation passes; macOS/workflow behavior
  is not changed because scripts and workflows are unchanged.
