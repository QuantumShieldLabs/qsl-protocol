Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-06

# NA-0430 qsc Adversarial Fuzz Validation Blocker Triage Plan

Goals: G1, G2, G3, G4, G5

## Executive summary

NA-0430 consumes the failed PR #1127 lockfile-only cleanup attempt and
classifies the qsc adversarial fuzz blocker as a lockfile resolution problem,
not a runtime, protocol, crypto, workflow, harness, test, vector, or public
surface problem.

PR #1127 made the nested qsc fuzz lock audit green and removed the pqcrypto
packages, but it used a broad lock refresh that also moved the direct
`ml-dsa = "=0.1.0-rc.7"` dependency from the previously building
`pkcs8 0.11.0-rc.11` / `spki 0.8.0-rc.4` / `signature 3.0.0-rc.10` chain to
`pkcs8 0.11.0` / `spki 0.8.0` / `signature 3.0.0`. The active
`qsc-adversarial-smoke` cargo-fuzz build then failed before fuzz execution with
Rust `E0277` conversion errors in `ml-dsa 0.1.0-rc.7` `pkcs8.rs`.

Proof-root simulations found a narrower lockfile-only path that removes the
pqcrypto residual, updates the nested advisory blockers, keeps the
`ml-dsa` release-candidate compatibility chain, passes nested lock audit, and
builds all qsc fuzz bins with `cargo +nightly build --locked --bins`.

Authorization classification:

`FUZZ_BLOCKER_LOCKFILE_PRECISE_VERSION_RETRY_AUTHORIZED`

Selected successor:

`NA-0431 -- QSL qsc Fuzz Lock Precise-Version pqcrypto Cleanup Retry Implementation Harness`

PR #1127 should remain closed and unmerged, with its branch retained until the
NA-0431 retry evidence no longer needs it.

## Live NA-0430 scope

Live `NEXT_ACTIONS.md` lists:

`NA-0430 -- QSL qsc Adversarial Fuzz Validation Blocker Triage Plan`

Status: READY.

Allowed NA-0430 mutation paths:

- `docs/governance/evidence/NA-0430_qsl_qsc_adversarial_fuzz_validation_blocker_triage_plan.md`
- `tests/NA-0430_qsl_qsc_adversarial_fuzz_validation_blocker_triage_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Read-only inspection covered PR #1127, PR #1128, D270/D271 response files,
NA-0428/NA-0429 evidence, `NEXT_ACTIONS.md`, `DECISIONS.md`,
`TRACEABILITY.md`, the rolling journal, the qsc fuzz manifest and lock, qsc
adversarial script/workflow wiring, root manifests and lock, qsc manifest,
formal roots, and inputs roots.

Proof-root simulation path:

`/srv/qbuild/tmp/NA0430_qsc_adversarial_fuzz_blocker_triage_20260606T020200Z/`

Forbidden NA-0430 mutation scope remained:

- no runtime code mutation;
- no crypto implementation mutation;
- no dependency manifest or lockfile mutation;
- no workflow or script mutation;
- no test, fuzz target, or vector mutation;
- no qsl-server, qsl-attachments, qshield runtime, website, public docs,
  README, or START_HERE mutation;
- no qwork, qstart, qresume, or qshell execution or mutation by Codex;
- no backup or restore execution;
- no qsl-backup, backup status, backup plan, rollback subtree, `/backup/qsl`,
  systemd, timer, or fstab mutation;
- no public technical paper content;
- no public readiness claim;
- no production readiness claim;
- no public-internet readiness claim;
- no external-review completion claim;
- no crypto-complete claim;
- no side-channel-free claim;
- no bug-free claim;
- no vulnerability-free claim;
- no perfect-crypto claim;
- no metadata-free, anonymity, untraceability, backup-complete,
  off-host-backup-complete, disaster-recovery-complete, or restore-proven
  claim.

Acceptance criteria for this triage lane:

- PR #1127 failure analyzed;
- failed `qsc-adversarial-smoke` job cited;
- lockfile/fuzz-tooling blocker classified;
- future remediation scope exact;
- no runtime/crypto/dependency/source mutation;
- root cargo audit remains green;
- public-safety green before merge and after merge;
- exactly one READY item remains.

Stop conditions preserved:

- qwork proof missing, malformed, stale, or inconsistent;
- qwork, qstart, or qresume run by Codex;
- PR #1128 not merged;
- PR #1127 merged;
- queue not READY NA-0430 at start;
- D-0846 absent or D-0847 already present at start;
- root cargo audit not green;
- PR #1127 failure evidence unavailable;
- successor cannot be selected safely;
- forbidden mutation or public overclaim introduced.

## qwork proof-file verification

Codex read, but did not run, qwork proof files:

- `/srv/qbuild/work/NA-0430/.qwork/startup.qsl-protocol.kv`
- `/srv/qbuild/work/NA-0430/.qwork/startup.qsl-protocol.json`

The `.kv` proof reported:

- `startup_result=OK`
- `lane=NA-0430`
- `repo=qsl-protocol`
- `path=/srv/qbuild/work/NA-0430/qsl-protocol`
- `head=3a529a9912cb1e5ad0fbbed3922511ebe0e94482`
- `origin_main=3a529a9912cb1e5ad0fbbed3922511ebe0e94482`
- `head_equals_origin_main=yes`
- `worktree_clean=yes`
- `index_clean=yes`
- `untracked_clean=yes`
- `ready_count=1`
- `queue_top_ready=NA-0430`
- `requested_lane_status=READY`

The JSON proof parsed successfully and mirrored the required `.kv` fields.
After `git fetch --all --prune`, live `HEAD` and `origin/main` still matched
the qwork proof exactly. `origin/main` equals and descends from
`3a529a9912cb`. The qwork proof files were copied under the proof root in
`qwork/`.

The local system clock reported `2026-06-05T21:00:51-05:00` /
`2026-06-06T02:00:51+00:00`; the directive identity timestamp remains
`2026-06-06T09:34:30-05:00` / `2026-06-06T14:34:30Z`.

## D270/D271 inheritance

D270 attempted the NA-0428-authorized nested qsc fuzz lockfile-only cleanup in
PR #1127 on branch `na-0429-qsc-fuzz-lock-pqcrypto-cleanup` at head
`967c95c37fea`. D270 recorded that the broad lock refresh:

- changed only the allowed implementation lockfile plus governance files;
- removed nested pqcrypto packages;
- updated nested `rustls-webpki` to `0.103.13`;
- introduced nested `ml-kem 0.2.3`;
- made nested lock audit green;
- failed GitHub `qsc-adversarial-smoke` while building a cargo-fuzz target.

D270 classified the failure as dependency/lockfile/fuzz-tooling related and
stopped rather than broadening scope.

D271 closed PR #1127 unmerged, retained its branch, restored main to clean
governance truth through PR #1128, marked NA-0429 BLOCKED rather than DONE, and
restored NA-0430 as the sole READY triage item. PR #1128 merged at
`3a529a9912cb`.

## Stewardship template application

### Crypto / Protocol Steward

Review question: Does the PR #1127 failure require protocol, wire, state
machine, provider, or crypto implementation mutation?

Evidence reviewed: qsc fuzz `Cargo.toml`, current and PR #1127 nested locks,
root `Cargo.lock`, root inverse trees, PR #1127 logs, qsc adversarial
script/workflow wiring, and proof-root build simulations.

Findings: The failing package is the direct fuzz-workspace dependency
`ml-dsa = "=0.1.0-rc.7"` after the lock moved its transitive compatibility
chain to `pkcs8 0.11.0`. The failure occurs in third-party crate compilation
before any qsc fuzz target executes. A lockfile-only proof-root recipe keeps
the previous `pkcs8 0.11.0-rc.11` chain and builds.

Risk classification: MEDIUM dependency/fuzz validation blocker; LOW evidence
for runtime or protocol semantic regression.

Public-claim impact: no crypto-complete claim, no side-channel-free claim, no
bug-free claim, no vulnerability-free claim, no perfect-crypto claim, and no
external-review-complete claim is supported.

Scope impact: no runtime, protocol, provider, or crypto source mutation is
authorized by NA-0430.

Recommended action: authorize an exact lockfile-only precise-version retry.

### CI / Dependency / Release Health Steward

Review question: Why did an audit-green lock fail active qsc adversarial CI,
and can the next retry stay lockfile-only?

Evidence reviewed: PR #1127 failed job `79833893300`, PR #1127 diff, lockfile
comparison, nested audit output, proof-root simulations, `scripts/ci/qsc_adversarial.sh`,
and `.github/workflows/qsc-adversarial.yml`.

Findings: The broad lock refresh solved the audit findings but upgraded the
`ml-dsa` transitive crypto-format chain to versions that do not compile with
`ml-dsa 0.1.0-rc.7`. A proof-root selective plus precise recipe:

```bash
cd qsl/qsl-client/qsc/fuzz
cargo update -p qsc -p quantumshield_refimpl
cargo update -p rustls-webpki --precise 0.103.13
cargo update -p rand@0.9.2 --precise 0.9.4
```

removed nested `pqcrypto-mlkem`, `pqcrypto-traits`, and
`pqcrypto-internals`, updated nested `rustls-webpki` to `0.103.13`, updated
nested `rand` to `0.9.4`, kept `ml-dsa 0.1.0-rc.7` on `pkcs8 0.11.0-rc.11`,
passed nested `cargo audit --deny warnings --file ...`, and built all fuzz
bins with `cargo +nightly build --locked --bins`.

Risk classification: ACTIVE CI/dependency blocker with a bounded lockfile-only
retry path.

Public-claim impact: audit green is dependency-health evidence only.

Scope impact: future mutation can remain limited to
`qsl/qsl-client/qsc/fuzz/Cargo.lock` plus governance evidence.

Recommended action: select precise-version lockfile retry successor and require
GitHub `qsc-adversarial-smoke` success before merge.

### Public Claims / External Review Steward

Review question: Does this triage or future retry support public security or
readiness claims?

Evidence reviewed: PR #1127/PR #1128 evidence, governance docs, root and nested
audit status, proof-root build results, and public-claim guardrails.

Findings: This lane is internal governance evidence. It does not create public
technical paper content and does not update public docs, README, START_HERE, or
website surfaces. Dependency-health and fuzz-build evidence are not broad
public assurance proofs.

Risk classification: CLAIM_BOUNDARY.

Public-claim impact: no public readiness claim, no production readiness claim,
no public-internet readiness claim, no external-review-complete claim, no
crypto-complete claim, no side-channel-free claim, no bug-free claim, no
vulnerability-free claim, and no perfect-crypto claim.

Scope impact: no public-surface mutation is authorized.

Recommended action: keep claim caveats explicit in NA-0431.

### Product / Demo / Service Boundary Steward

Review question: Does fuzz/adversarial tooling status alter product, demo, or
service readiness?

Evidence reviewed: qsc adversarial workflow, qsc fuzz workspace, PR #1127
check status, and service/public-surface scope.

Findings: qsc fuzz/adversarial validation is active CI tooling for internal
quality and dependency health. It is not a production deployment, public
service, external demo, qsl-server change, qsl-attachments change, or website
readiness path.

Risk classification: SERVICE_BOUNDARY.

Public-claim impact: no production/public-internet readiness claim.

Scope impact: no qsl-server, qsl-attachments, qshield runtime, public docs, or
website mutation is authorized.

Recommended action: keep NA-0431 scoped to the nested fuzz lock and
governance-only support files.

### Local Ops / Backup / Restore Steward

Review question: Did the triage require backup, restore, qsl-backup, status,
plan, rollback, or local-ops mutation?

Evidence reviewed: qsl-backup checksum and source-list count, directive
boundaries, worktree status, and proof-root paths.

Findings: No backup or restore was run. `/usr/local/sbin/qsl-backup` checksum
was `e9ecff3d22ed...` with exact Codex ops source-list inclusion count 1.
Proof-root simulations wrote only under `/srv/qbuild/tmp/...`.

Risk classification: LOW local-ops impact.

Public-claim impact: no backup-complete, off-host-backup-complete,
disaster-recovery-complete, or restore-proven claim.

Scope impact: no local-ops mutation is authorized.

Recommended action: no backup-plan or status change is needed.

## PR #1127 failure evidence review

PR #1127 metadata:

- title: `NA-0429: clean qsc fuzz lock pqcrypto residual`
- state: CLOSED
- mergedAt: null
- mergeCommit: null
- head branch: `na-0429-qsc-fuzz-lock-pqcrypto-cleanup`
- head SHA: `967c95c37fea`
- mergeStateStatus: BEHIND after closure

PR #1127 changed:

- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/governance/evidence/NA-0429_qsl_qsc_fuzz_lock_pqcrypto_residual_lockfile_cleanup_implementation_harness.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `qsl/qsl-client/qsc/fuzz/Cargo.lock`
- `tests/NA-0429_qsl_qsc_fuzz_lock_pqcrypto_residual_lockfile_cleanup_implementation_testplan.md`

Failed job:

- check: `qsc-adversarial-smoke`
- job id: `79833893300`
- URL: `https://github.com/QuantumShieldLabs/qsl-protocol/actions/runs/27046664839/job/79833893300`
- phase: dependency/build before fuzz execution
- first failing package: `ml-dsa 0.1.0-rc.7`
- visible dependency chain: `pkcs8 0.11.0`, `spki 0.8.0`,
  `signature 3.0.0`
- Rust error code: `E0277`

The first compiler error states that `?` could not convert an error to
`pkcs8::Error` in `ml-dsa-0.1.0-rc.7/src/pkcs8.rs:107`. The same error class
appears again at line 112, and a related conversion error appears for
`pkcs8::spki::Error` at line 210. This happened before the fuzz targets could
run.

Classification: dependency/lockfile/fuzz-tooling build blocker.

## Lockfile diff / dependency graph analysis

Compared locks:

- origin/main nested fuzz lock SHA:
  `a4a3378781b7ce88a556fad897ec53c90c870e095b6353b57c0a2de990e6770a`
- PR #1127 nested fuzz lock SHA:
  `fd7cfd20f9d912004f7ee90750abed785fc518df066adb17f85ea781c9a5a0d5`

Package counts:

- origin/main nested fuzz lock: 362 packages
- PR #1127 nested fuzz lock: 293 packages
- removed package identities: 157
- added package identities: 88

Key graph changes in PR #1127:

- `pqcrypto-mlkem 0.1.1`: removed
- `pqcrypto-traits 0.3.5`: removed
- `pqcrypto-internals 0.2.11`: removed
- `rustls-webpki 0.103.10`: updated to `0.103.13`
- `rustls 0.23.37`: updated to `0.23.40`
- `hyper-rustls 0.27.7`: updated to `0.27.9`
- `rand 0.8.5` and `rand 0.9.2`: collapsed to `rand 0.9.4`
- `ml-kem 0.2.3`: introduced
- `ml-dsa 0.1.0-rc.7`: unchanged as the direct fuzz dependency
- `pkcs8 0.11.0-rc.11`: updated to `pkcs8 0.11.0`
- `spki 0.8.0-rc.4`: updated to `spki 0.8.0`
- `signature 3.0.0-rc.10`: updated to `signature 3.0.0`

`qsl/qsl-client/qsc/fuzz/Cargo.toml` directly depends on:

```toml
libfuzzer-sys = "0.4"
ml-dsa = "=0.1.0-rc.7"
qsc = { path = ".." }
```

It does not directly depend on `pkcs8`, `spki`, `signature`, pqcrypto packages,
`rustls-webpki`, `rand`, or `ml-kem`.

Root cause: PR #1127 used broad lock resolution. It solved the pqcrypto,
rustls-webpki, and rand audit blockers, but it also advanced the
release-candidate `ml-dsa` supporting crates past the compatibility set that
compiled on main.

Proof-root selective plus precise simulation showed the next retry can avoid
that broad transitive movement while still clearing the nested audit blockers.

## Toolchain / reproduction feasibility review

Local tool availability:

- `cargo 1.95.0`
- `rustc 1.95.0`
- `cargo +nightly 1.98.0-nightly`
- `cargo-fuzz`: not installed locally
- Codex did not install cargo-fuzz or change toolchains.

`scripts/ci/qsc_adversarial.sh` invokes:

- stable qsc adversarial property tests;
- stable qsc adversarial miri test;
- `cargo +nightly fuzz run` for three qsc fuzz targets.

`.github/workflows/qsc-adversarial.yml` installs `cargo-fuzz 0.13.1` in CI and
runs `sh scripts/ci/qsc_adversarial.sh` for non-docs pull requests and pushes
to main.

Proof-root local reproduction:

- PR #1127 lock copied into an archived proof-root repo failed
  `cargo +nightly build --manifest-path qsl/qsl-client/qsc/fuzz/Cargo.toml --locked --bins`
  with the same `ml-dsa 0.1.0-rc.7` / `pkcs8 0.11.0` `E0277` compiler errors.
- The precise-version proof-root lock passed nested audit and built all fuzz
  bins with the same `cargo +nightly build --locked --bins` command.

Future lane can use this build as a local preflight before opening a PR. Final
acceptance still requires GitHub `qsc-adversarial-smoke` success because the
actual workflow uses `cargo-fuzz`.

Recovered proof outcome recorded:

- failing command:
  `cargo +nightly build --manifest-path <proof-copy>/qsl/qsl-client/qsc/fuzz/Cargo.toml --locked --bins`
  using the PR #1127 lock;
- classification: expected proof-root reproduction of the known CI failure,
  not source checkout failure;
- corrective action: none needed for NA-0430; use the failure as evidence and
  compare against the successful precise-version proof build;
- final result: PR #1127 lock reproduced `E0277`, precise-version lock built
  successfully.

## Remediation strategy matrix

Option 1: Selective lockfile update preserving cargo-fuzz build compatibility.

- Status: accepted as part of the precise-version strategy.
- Evidence: updating only path packages removed pqcrypto and preserved the
  `ml-dsa` rc chain, but nested audit still failed on `rustls-webpki 0.103.10`
  and `rand 0.9.2`.
- Future paths: `qsl/qsl-client/qsc/fuzz/Cargo.lock` plus governance support.
- Risk: low if paired with precise updates and qsc-adversarial-smoke.
- Validation: nested audit, fuzz-bin build, qsc-adversarial-smoke, root audit,
  qsc send_commit, pqkem768, formal checks.
- Public caveat: dependency-health evidence only.

Option 2: Lockfile-only with explicit transitive version constraint via
`cargo update -p` / `--precise`.

- Status: recommended and authorized.
- Evidence: proof-root recipe removes pqcrypto, updates advisory blockers,
  preserves `pkcs8 0.11.0-rc.11`, passes nested audit, and builds fuzz bins.
- Future paths: `qsl/qsl-client/qsc/fuzz/Cargo.lock` plus NA-0431 evidence,
  testplan, DECISIONS, TRACEABILITY, and rolling journal.
- Risk: low to medium; final GitHub cargo-fuzz smoke remains required.
- Validation: root cargo audit green, nested fuzz lock audit green,
  qsc-adversarial-smoke success, qsc send_commit, provider pqkem768, formal
  checks, no runtime/source/workflow/test/vector/public mutation.
- Public caveat: no public-readiness or public-security claim expansion.

Option 3: Fuzz `Cargo.toml` dependency constraint / compatibility fix.

- Status: rejected for NA-0431 unless the precise lockfile retry unexpectedly
  fails.
- Evidence: no manifest change was needed in proof-root to create a compatible
  audit-green lock.
- Future paths if needed later: `qsl/qsl-client/qsc/fuzz/Cargo.toml` and
  `qsl/qsl-client/qsc/fuzz/Cargo.lock` only with new authorization.
- Risk: higher than lockfile-only because it changes declared dependency
  constraints.
- Validation: manifest-scope validation and qsc-adversarial-smoke.
- Public caveat: no claim expansion.

Option 4: Fuzz harness/provider alignment.

- Status: rejected for NA-0431.
- Evidence: failure occurs in third-party dependency compilation before fuzz
  target execution; proof-root lock builds without harness edits.
- Future paths if needed later: exact fuzz source paths only after new proof.
- Risk: higher; would touch executable fuzz source.
- Validation: fuzz target and qsc adversarial evidence.
- Public caveat: no claim expansion.

Option 5: Workflow/tooling preflight lane.

- Status: not selected.
- Evidence: local `cargo +nightly build --locked --bins` is sufficient to catch
  the compiler failure class, and CI remains the final cargo-fuzz authority.
- Future paths if needed later: exact tooling/governance paths only after new
  authorization.
- Risk: workflow/tooling churn if selected unnecessarily.
- Validation: tooling-specific proof.
- Public caveat: no public-readiness claim.

Option 6: Waiver/ignore.

- Status: rejected.
- Evidence: the nested fuzz lock is active and qsc adversarial smoke must pass.
- Future paths: none.
- Risk: would dilute fail-closed dependency and fuzz validation posture.
- Validation: not applicable.
- Public caveat: would be inappropriate.

Option 7: Stop/ambiguity.

- Status: not selected.
- Evidence: exact future lockfile-only retry scope is supported by proof-root
  simulation.
- Future paths: not applicable.
- Risk: unnecessary queue stall.
- Validation: not applicable.
- Public caveat: no claim expansion.

## Authorization decision

Classification:

`FUZZ_BLOCKER_LOCKFILE_PRECISE_VERSION_RETRY_AUTHORIZED`

Authorized future NA-0431 mutable paths:

- `qsl/qsl-client/qsc/fuzz/Cargo.lock`
- `docs/governance/evidence/NA-0431_qsl_qsc_fuzz_lock_precise_version_pqcrypto_cleanup_retry_implementation_harness.md`
- `tests/NA-0431_qsl_qsc_fuzz_lock_precise_version_pqcrypto_cleanup_retry_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Future NA-0431 command recipe to start from the nested fuzz workspace:

```bash
cd qsl/qsl-client/qsc/fuzz
cargo update -p qsc -p quantumshield_refimpl
cargo update -p rustls-webpki --precise 0.103.13
cargo update -p rand@0.9.2 --precise 0.9.4
```

Expected future lock evidence:

- pqcrypto packages absent from nested fuzz lock;
- nested `rustls-webpki 0.103.13`;
- nested `rand 0.9.4`;
- nested `ml-kem 0.2.3`;
- nested `ml-dsa 0.1.0-rc.7`;
- nested `pkcs8 0.11.0-rc.11`;
- nested `spki 0.8.0-rc.4`;
- nested `signature 3.0.0-rc.10`.

Future required validation:

- root `cargo audit --deny warnings`;
- nested fuzz lock `cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock`;
- local proof build of qsc fuzz bins if cargo-fuzz remains unavailable:
  `cargo +nightly build --manifest-path qsl/qsl-client/qsc/fuzz/Cargo.toml --locked --bins`;
- GitHub `qsc-adversarial-smoke` success before merge;
- qsc send_commit test;
- provider pqkem768 test;
- formal model checks;
- public-safety green before merge and after merge;
- no public overclaim.

No runtime, production crypto, workflow, script, test, vector, qsl-server,
qsl-attachments, qshield runtime, public docs, README, START_HERE, website,
backup, restore, qwork, qstart, qresume, qshell, qsl-backup, backup status, or
backup plan mutation is authorized.

PR #1127 branch retention recommendation: retain until NA-0431 has captured
and merged enough retry evidence that the failed-attempt branch is no longer
needed for comparison.

## Successor selection

Selected NA-0431 successor:

`NA-0431 -- QSL qsc Fuzz Lock Precise-Version pqcrypto Cleanup Retry Implementation Harness`

Do not implement NA-0431 in this lane.

## Future path/scope bundle

Allowed future NA-0431 mutation paths:

- `qsl/qsl-client/qsc/fuzz/Cargo.lock`
- `docs/governance/evidence/NA-0431_qsl_qsc_fuzz_lock_precise_version_pqcrypto_cleanup_retry_implementation_harness.md`
- `tests/NA-0431_qsl_qsc_fuzz_lock_precise_version_pqcrypto_cleanup_retry_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Allowed read-only future inspection:

- `qsl/qsl-client/qsc/fuzz/Cargo.toml`
- `scripts/ci/qsc_adversarial.sh`
- `.github/workflows/qsc-adversarial.yml`
- root `Cargo.toml`
- root `Cargo.lock`
- `qsl/qsl-client/qsc/Cargo.toml`
- PR #1127 evidence
- NA-0430 evidence

Future forbidden unless a later exact scope authorizes otherwise:

- runtime/crypto source changes;
- root Cargo changes;
- fuzz `Cargo.toml`;
- workflows/scripts;
- executable tests, fuzz target source, or vectors;
- qsl-server/qsl-attachments;
- public docs/README/START_HERE/website;
- backup/restore/qsl-backup;
- public-readiness or public-security claim expansion.

## Future validation/marker plan

Common NA-0431 markers:

- `NA0431_QSC_FUZZ_BLOCKER_RETRY_SCOPE_OK`
- `NA0431_LOCKFILE_ONLY_SCOPE_OK`
- `NA0431_ROOT_CARGO_AUDIT_GREEN_OK`
- `NA0431_FUZZ_LOCK_AUDIT_GREEN_OK`
- `NA0431_QSC_ADVERSARIAL_SMOKE_OK`
- `NA0431_NO_RUNTIME_CHANGE_OK`
- `NA0431_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0431_NO_CRYPTO_COMPLETE_CLAIM_OK`
- `NA0431_NO_SECRET_MATERIAL_OK`
- `NA0431_ONE_READY_INVARIANT_OK`
- `NA0431_STEWARD_REVIEW_TEMPLATE_USED_OK`

Additional expected evidence:

- precise-version command transcript;
- before/after nested lock package summary;
- proof that `pkcs8 0.11.0-rc.11` is retained for `ml-dsa 0.1.0-rc.7`;
- proof that PR #1127's `pkcs8 0.11.0` failure class is avoided;
- qsc fuzz-bin build preflight if local cargo-fuzz remains unavailable.

## Public claim/external review/website boundary

NA-0430 is internal governance triage evidence only.

Fuzz lock remediation is not production readiness. It is not public-internet
readiness, not external-review completion, not crypto completeness, not
side-channel assurance, not bug absence, not vulnerability absence, not perfect
crypto, not anonymity, not untraceability, not metadata-free proof, not backup
completion, not off-host backup completion, not disaster recovery completion,
not restore proof, not public technical paper content, and not website
readiness.

No README, START_HERE, public docs, website, or public paper path is updated.
Root cargo audit green and nested cargo audit green remain dependency-health
signals only.

## Rejected alternatives

- Reopening or merging PR #1127 as-is is rejected because it fails the active
  qsc adversarial smoke path.
- Waiving nested fuzz audit or qsc-adversarial-smoke is rejected because it
  would weaken fail-closed dependency and fuzz validation posture.
- Mutating fuzz `Cargo.toml` is rejected for the immediate successor because a
  lockfile-only precise retry has proof.
- Mutating fuzz targets or qsc runtime source is rejected because the failure
  occurs before target execution and proof-root lock resolution builds.
- Mutating workflows/scripts is rejected because CI wiring is functioning and
  correctly caught the PR #1127 build blocker.

## Backup-impact statement

No backup or restore was run. Codex did not run sudo. Codex did not mutate
`/usr/local/sbin/qsl-backup`, `/backup/qsl`, backup logs, backup manifests,
backup status, backup plan files, rollback subtree paths, systemd, timers,
fstab, source lists, retention policy, or backup scripts. qsl-backup checksum
remained `e9ecff3d22ed...` and the exact Codex ops source-list inclusion count
remained 1.

Proof-root simulations wrote only under:

`/srv/qbuild/tmp/NA0430_qsc_adversarial_fuzz_blocker_triage_20260606T020200Z/`

## Next recommendation

Proceed to NA-0431 as a precise-version lockfile-only retry implementation
harness. The next lane should apply the exact proof-root recipe, verify nested
audit and fuzz-bin build compatibility locally, then require GitHub
`qsc-adversarial-smoke` success before merge. If that precise retry fails, stop
and request a new authorization lane before considering fuzz `Cargo.toml`,
harness, workflow, or runtime/source changes.
