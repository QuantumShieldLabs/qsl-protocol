# NA-0446 Closeout and NA-0447 Restoration Testplan

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-09

Goals: G1, G2, G3, G4, G5

## Objective

Validate that NA-0446 is closed only after PR #1161 merged, post-merge
public-safety passed under the D297 progress-aware wait policy, and the exact
qsc key lifecycle zeroization test evidence remains intact. Restore exactly
one successor, NA-0447, as READY without implementing NA-0447.

## Protected invariants

- READY_COUNT remains 1.
- NA-0446 is DONE after closeout.
- NA-0447 is READY after closeout.
- NA-0434 remains BLOCKED.
- D-0879 exists once.
- D-0880 exists once after closeout.
- D-0881 remains absent until future NA-0447 work.
- PR #1161 remains merged at `19761797425f`.
- Post-merge public-safety is green on `19761797425f`.
- qsc key lifecycle zeroization tests pass and remain bounded evidence only.
- Direct memory zeroization is not claimed.
- Secret-material-complete coverage is not claimed.
- qshield-cli demo boundary is preserved.
- refimpl cleanup/zeroization scope remains deferred.
- No runtime, crypto, dependency, Cargo, lockfile, workflow, executable test,
  fuzz target, vector, formal model, service, public-surface, or
  backup/local-ops path is changed.
- No public claim expansion is introduced.

## Allowed scope

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0446_closeout_restore_na0447_testplan.md`

## Forbidden scope

Forbidden changed paths include runtime, crypto, dependency, Cargo manifest,
lockfile, workflow, executable test source, fuzz target, vector, formal model,
qsl-server, qsl-attachments, qshield runtime, qshield-cli, website, public
docs, README, START_HERE, qwork/qstart/qresume/qshell, qsl-backup, backup
status, backup plan, rollback subtree, and backup tree paths.

Forbidden commands include qwork, qstart, qresume, sudo, backup, restore,
generated operator scripts, `cargo update`, `cargo generate-lockfile`,
dependency remediation commands, workflow mutation, branch-protection mutation,
public technical paper work, and NA-0447 implementation.

## PR #1161 merge/public-safety checks

Run:

```bash
gh pr view 1161 --repo QuantumShieldLabs/qsl-protocol --json number,state,mergedAt,mergeCommit,headRefOid,title,url
python3 scripts/ci/qsl_evidence_helper.py public-safety-status --sha 19761797425f049a77f812247a3ad5f874821372
```

Required:

- PR #1161 state is MERGED.
- PR #1161 merge commit starts with `19761797425f`.
- post-merge public-safety is completed success.
- public-safety is not red or ambiguous.

## Progress-aware wait evidence

Use REST polling, not watch mode, and save check-run snapshots under the proof
directory.

Required:

- public-safety completed success.
- no required check completed failure, cancelled, timed_out, or action_required.
- `macos-qsc-full-serial` completed success or accepted skip according to repo
  policy.
- `qsc-linux-full-suite` completed success or accepted skip according to repo
  policy.
- `qsc-adversarial-smoke` completed success or accepted check shape.
- attached `qsc-adversarial-miri` completed success or accepted check shape.
- no stale or ambiguous public-safety state was used to close out.

## qsc key lifecycle zeroization test check

Run:

```bash
test -f qsl/qsl-client/qsc/tests/key_lifecycle_zeroization.rs
rg -n "NA0446_KEY_LIFECYCLE_TEST_IMPLEMENTATION_OK|NA0446_PENDING_SECRET_CLEANUP_SUCCESS_BOUNDARY_OK|NA0446_REJECT_NO_MUTATION_BOUNDARY_OK|NA0446_SESSION_SECRET_STORE_BOUNDARY_OK|NA0446_ENCRYPTED_AT_REST_BOUNDARY_OK|NA0446_REDACTION_SENTINEL_BOUNDARY_OK|NA0446_NO_RUNTIME_HOOK_USED_OK|NA0446_DIRECT_MEMORY_ZEROIZATION_NOT_CLAIMED_OK|NA0446_NO_SECRET_MATERIAL_COMPLETE_CLAIM_OK|NA0446_QSHIELD_CLI_DEMO_BOUNDARY_PRESERVED_OK|NA0446_REFIMPL_SCOPE_DEFERRED_OK" qsl/qsl-client/qsc/tests/key_lifecycle_zeroization.rs
cargo test -p qsc --locked --test key_lifecycle_zeroization -- --test-threads=1 --nocapture
```

Required:

- exact test file exists;
- all required markers are present in source or output;
- all NA-0446 tests pass.

## Direct memory zeroization caveat check

Confirm:

- `NA0446_DIRECT_MEMORY_ZEROIZATION_NOT_CLAIMED_OK` is present;
- no direct runtime memory overwrite proof is claimed;
- no allocator overwrite proof is claimed;
- no `Drop` proof is claimed;
- no side-channel proof is claimed.

## Secret-material-complete caveat check

Confirm:

- `NA0446_NO_SECRET_MATERIAL_COMPLETE_CLAIM_OK` is present;
- no all-key-material coverage claim is introduced;
- no secret-material-complete claim is introduced;
- cargo audit green remains dependency-health evidence only.

## qshield-cli demo boundary check

Confirm:

- `NA0446_QSHIELD_CLI_DEMO_BOUNDARY_PRESERVED_OK` is present;
- qshield-cli remains demo-local boundary evidence only;
- no qshield-cli runtime, service-readiness, public-readiness, or production
  claim is introduced.

## refimpl deferred-scope check

Confirm:

- `NA0446_REFIMPL_SCOPE_DEFERRED_OK` is present;
- refimpl cleanup/zeroization scope remains deferred;
- refimpl evidence is not represented as qsc runtime cleanup proof.

## Root cargo audit green check

Run:

```bash
cargo audit --deny warnings
```

Required:

- root cargo audit passes;
- audit output is not used as public-readiness, production-readiness,
  external-review-complete, crypto-complete, secret-material-complete,
  vulnerability-free, bug-free, perfect-crypto, or side-channel-free proof.

## Nested fuzz lock audit green check

Run:

```bash
cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock
```

Required:

- nested qsc fuzz lock audit passes;
- no lockfile mutation occurs.

## NA-0446 DONE / NA-0447 READY check

Run:

```bash
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
```

Required:

- READY_COUNT 1.
- READY NA-0447.
- NA-0446 DONE.
- NA-0434 BLOCKED.
- NA-0429 BLOCKED.
- D-0879 exists once.
- D-0880 exists once.
- D-0881 absent.
- duplicate decision count zero.

## qsl-protocol closeout scope guard

Run after patching and before PR creation:

```bash
git diff --name-only origin/main
git ls-files --others --exclude-standard
python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main --allowed NEXT_ACTIONS.md DECISIONS.md TRACEABILITY.md docs/ops/ROLLING_OPERATIONS_JOURNAL.md tests/NA-0446_closeout_restore_na0447_testplan.md
```

The combined changed-path set must be exactly:

- `DECISIONS.md`
- `NEXT_ACTIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0446_closeout_restore_na0447_testplan.md`

## No runtime/dependency/workflow/test/vector/formal mutation

Confirm:

- no source path is changed;
- no workflow path is changed;
- no executable test source is changed by closeout;
- no vector path is changed;
- no formal model path is changed;
- no dependency manifest or lockfile is changed;
- no qsl-server, qsl-attachments, qshield runtime, qshield-cli, website,
  public docs, README, or START_HERE path is changed.

## No public overclaim

Confirm:

- no production-readiness claim is introduced;
- no public-readiness claim is introduced;
- no public-internet-readiness claim is introduced;
- no external-review-complete claim is introduced;
- no crypto-complete claim is introduced;
- no secret-material-complete claim is introduced;
- no side-channel-free claim is introduced;
- no bug-free claim is introduced;
- no vulnerability-free claim is introduced;
- no perfect-crypto claim is introduced;
- no metadata-free behavior claim is introduced;
- no anonymity claim is introduced;
- no untraceability claim is introduced;
- no public technical paper content is introduced;
- no README, START_HERE, public docs, or website path is changed.
