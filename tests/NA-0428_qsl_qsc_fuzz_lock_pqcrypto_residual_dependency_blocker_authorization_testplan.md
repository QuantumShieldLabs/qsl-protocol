Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-05

# NA-0428 qsc Fuzz Lock pqcrypto Residual Dependency Blocker Authorization Testplan

Goals: G1, G2, G3, G4, G5

## Purpose

Validate that NA-0428 classifies the nested qsc fuzz-lock pqcrypto residual,
selects the exact lockfile-only NA-0429 successor, and preserves all
no-runtime, no-crypto, no-dependency, no-Cargo-mutation, no-lockfile-mutation,
no-workflow, no-fuzz-target, no-test/vector, no-service, no-public-surface,
no-backup, and no-public-overclaim boundaries for this authorization PR.

## Preconditions

- qwork proof files exist and are read without running qwork:
  - `/srv/qbuild/work/NA-0428/.qwork/startup.qsl-protocol.kv`
  - `/srv/qbuild/work/NA-0428/.qwork/startup.qsl-protocol.json`
- qwork proof fields report lane NA-0428, repo qsl-protocol, clean worktree,
  READY_COUNT 1, queue top READY NA-0428, requested lane status READY, and
  matching head/origin_main.
- Live `HEAD` and `origin/main` match qwork proof after fetch.
- PR #1124 is MERGED at `311a93ea9a47`.
- Queue helper reports READY_COUNT 1 and READY NA-0428.
- NA-0427 is DONE.
- Decision helper reports latest D-0843 and duplicate count zero.
- D-0842 exists once, D-0843 exists once, and D-0844 is absent at start.
- public-safety is green on current `origin/main`.
- Root `cargo audit --deny warnings` is green.
- `cargo tree -i rustls-webpki --locked` reports `rustls-webpki v0.103.13` or
  newer safe version.
- Root pqcrypto package IDs are absent from the locked graph.
- qsl-backup checksum/source-count boundary matches the directive.

## Allowed scope

Allowed changed paths for the NA-0428 evidence PR:

- `docs/governance/evidence/NA-0428_qsl_qsc_fuzz_lock_pqcrypto_residual_dependency_blocker_authorization_plan.md`
- `tests/NA-0428_qsl_qsc_fuzz_lock_pqcrypto_residual_dependency_blocker_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

No other qsl-protocol paths may change.

## Forbidden scope

Forbidden changes include:

- runtime code;
- crypto implementation code;
- dependencies, `Cargo.toml`, or `Cargo.lock`;
- qsc fuzz lock or fuzz manifest mutation;
- fuzz target source;
- tests or vectors outside this governance testplan;
- workflows or branch protection;
- qsl-server, qsl-attachments, qshield runtime, website, public docs, README,
  or START_HERE;
- qwork, qstart, qresume, or qshell;
- qsl-backup;
- backup status or backup plan files;
- `/backup/qsl`;
- rollback subtree paths;
- public technical paper content;
- secret material.

Forbidden public assurance claims include no public-readiness claim, no
production-readiness claim, no public-internet-readiness claim, no
external-review completion claim, no crypto completion claim, no
side-channel-free status claim, no metadata-free behavior claim, no anonymity
claim, no untraceability claim, no off-host-backup completion claim, no
disaster-recovery completion claim, no restore proof claim, no backup
completion claim, no vulnerability-free status claim, no bug-free status claim,
and no perfect-crypto status claim.

## Required evidence assertions

Verify the evidence doc includes:

- Executive summary.
- Live NA-0428 scope.
- qwork proof-file verification.
- NA-0427 inheritance.
- Stewardship template application.
- Nested fuzz lock presence and content review.
- Fuzz / adversarial tooling linkage review.
- Remediation option matrix.
- Authorization decision.
- Successor selection.
- Future path/scope bundle.
- Future validation/marker plan.
- Public claim/external review/website boundary.
- Rejected alternatives.
- Backup-impact statement.
- Next recommendation.

## Stewardship assertions

Verify advisory summaries are present for:

- Crypto / Protocol Steward.
- CI / Dependency / Release Health Steward.
- Public Claims / External Review Steward.
- Product / Demo / Service Boundary Steward.
- Local Ops / Backup / Restore Steward.

Each summary must record:

- review question;
- evidence reviewed;
- findings;
- risk classification;
- public-claim impact;
- scope impact;
- recommended action.

## Dependency and fuzz-lock assertions

Required read-only commands:

```bash
rg -n "pqcrypto|pqcrypto-mlkem|pqcrypto-traits|pqcrypto-internals|rustls-webpki|ml-kem" qsl/qsl-client/qsc/fuzz/Cargo.toml qsl/qsl-client/qsc/fuzz/Cargo.lock Cargo.toml Cargo.lock qsl/qsl-client/qsc/Cargo.toml || true
cargo audit --deny warnings
cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock || true
cargo metadata --locked --format-version=1
cargo tree -i rustls-webpki --locked
cargo tree -i ml-kem --locked || true
cargo tree -i pqcrypto-mlkem --locked || true
cargo tree -i pqcrypto-traits --locked || true
cargo tree -i pqcrypto-internals --locked || true
```

Verify:

- root cargo audit is green;
- root `rustls-webpki` is `v0.103.13` or newer safe version;
- root `ml-kem` inverse tree reaches active root packages;
- root pqcrypto inverse trees report package absence;
- qsc fuzz lock exists and records pqcrypto package entries before remediation;
- qsc fuzz lock audit red state is recorded as blocker evidence;
- exact nested fuzz lock advisory IDs are recorded when available;
- root `cargo metadata` does not include `qsc-fuzz`;
- proof-root lock refresh simulation removes pqcrypto packages without real
  repo mutation;
- proof-root simulated nested lock audit passes;
- selected classification is
  `FUZZ_LOCK_REMEDIATION_AUTHORIZED_LOCKFILE_ONLY`;
- no Cargo, dependency, lockfile, workflow, runtime, crypto, fuzz target, test,
  or vector file is mutated by NA-0428.

## Fuzz / adversarial linkage assertions

Verify the evidence records:

- `qsl/qsl-client/qsc/fuzz/Cargo.toml` is a real cargo-fuzz manifest.
- `qsl/qsl-client/qsc/fuzz/Cargo.lock` is a separate committed lock.
- `scripts/ci/qsc_adversarial.sh` changes into the fuzz workspace and runs the
  three committed fuzz targets.
- `.github/workflows/qsc-adversarial.yml` runs the qsc adversarial smoke job
  for non-docs scopes.
- The fuzz/adversarial path is dev/test/CI tooling and not production runtime.
- The stale nested lock can affect adversarial confidence.
- Workflow/script mutation is rejected unless future proof contradicts NA-0428.

## Remediation option assertions

Verify the option matrix records:

- Option 1 lockfile-only selected.
- Option 2 Cargo.toml plus lock rejected as unnecessary.
- Option 3 harness alignment rejected as unsupported by current evidence.
- Option 4 workflow/script scope rejected as unnecessary.
- Option 5 waiver/ignore rejected.
- Option 6 ambiguity not selected.

## Required decision assertions

Verify D-0844 exists once after the patch and states:

- nested qsc fuzz lock blocker confirmed;
- exact classification
  `FUZZ_LOCK_REMEDIATION_AUTHORIZED_LOCKFILE_ONLY`;
- selected NA-0429 successor is
  `NA-0429 -- QSL qsc Fuzz Lock pqcrypto Residual Lockfile Cleanup Implementation Harness`;
- no runtime/crypto/dependency/workflow mutation occurred;
- no Cargo or lockfile mutation occurred in NA-0428;
- no backup/restore occurred;
- no public crypto-complete claim is made;
- no vulnerability-free or perfect-crypto claim is made;
- stewardship template was used;
- exactly one READY remains mandatory.

## Validation commands

Run before PR:

```bash
git diff --check
python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main --head HEAD \
  --allowed docs/governance/evidence/NA-0428_qsl_qsc_fuzz_lock_pqcrypto_residual_dependency_blocker_authorization_plan.md \
  --allowed tests/NA-0428_qsl_qsc_fuzz_lock_pqcrypto_residual_dependency_blocker_authorization_testplan.md \
  --allowed DECISIONS.md \
  --allowed TRACEABILITY.md \
  --allowed docs/ops/ROLLING_OPERATIONS_JOURNAL.md \
  --forbidden .github/ \
  --forbidden Cargo.toml \
  --forbidden Cargo.lock \
  --forbidden qsl/ \
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
cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock || true
cargo tree -i rustls-webpki --locked
cargo tree -i ml-kem --locked || true
cargo tree -i pqcrypto-mlkem --locked || true
cargo tree -i pqcrypto-traits --locked || true
cargo tree -i pqcrypto-internals --locked || true
cargo fmt --check
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
cargo test -p quantumshield_refimpl --features pqcrypto --locked --test pqkem768
python3 formal/model_qsc_handshake_suite_id_bounded.py
python3 formal/run_model_checks.py
```

Additional checks:

- exact changed-path guard for the five allowed NA-0428 paths;
- added-line overclaim scan;
- classifier proof;
- PR body preflight;
- goal-lint with `Goals: G1, G2, G3, G4, G5`;
- required checks, including public-safety, green before merge;
- post-merge public-safety green after merge.

## Acceptance criteria

- READY_COUNT 1.
- READY NA-0428 before optional closeout.
- NA-0427 DONE.
- D-0842 exists once.
- D-0843 exists once.
- D-0844 exists once.
- D-0845 absent before optional closeout.
- Duplicate decision count 0.
- Only the five allowed evidence paths changed.
- Classification is `FUZZ_LOCK_REMEDIATION_AUTHORIZED_LOCKFILE_ONLY`.
- Selected successor is exact NA-0429 lockfile cleanup implementation harness.
- No runtime/crypto/dependency/Cargo/lockfile/workflow/fuzz-target/test/vector/public/service/backup mutation.
- No public overclaim.

## Post-fix hardening review checklist

- Correctness under stress: the selected future scope follows live root and
  nested dependency evidence, active adversarial linkage, and proof-root lock
  refresh simulation.
- Minimality: NA-0428 changes only governance evidence/testplan, decision,
  traceability, and journal paths.
- Maintainability: NA-0429 gets a narrow allowed/forbidden scope and explicit
  validation markers.
- Coverage quality: validations cover queue, decisions, scope, links, leaks,
  overclaims, classifier, root and nested audit status, qsc, provider tests, and
  formal models.
- Cross-lane stability: Linux/macOS CI authority and public-safety remain
  required; no platform-specific runtime path changes are introduced.
