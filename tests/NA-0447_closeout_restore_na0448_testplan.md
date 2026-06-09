# NA-0447 Closeout and NA-0448 Restoration Testplan

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-09

Goals: G1, G2, G3, G4, G5

## Objective

Validate that NA-0447 is closed only after PR #1163 merged, post-merge
public-safety passed, and D-0881 selected the qsc RNG failure test-seam
authorization successor. Restore exactly one successor, NA-0448, as READY
without implementing NA-0448.

## Protected invariants

- READY_COUNT remains 1.
- NA-0447 is DONE after closeout.
- NA-0448 is READY after closeout.
- NA-0446 remains DONE.
- NA-0434 remains BLOCKED.
- NA-0429 remains BLOCKED.
- D-0881 exists once.
- D-0882 exists once after closeout.
- D-0883 remains absent until future NA-0448 work.
- PR #1163 remains merged at `db2aaa7a831`.
- Post-merge public-safety is green on `db2aaa7a831`.
- NA-0448 remains authorization-only for a qsc RNG failure test seam or
  equivalent bounded strategy.
- No NA-0448 implementation occurs in this closeout.
- No runtime, crypto, dependency, Cargo, lockfile, workflow, executable test,
  fuzz target, vector, formal model, service, public-surface, or
  backup/local-ops path is changed.
- No public claim expansion is introduced.

## Allowed scope

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0447_closeout_restore_na0448_testplan.md`

## Forbidden scope

Forbidden changed paths include runtime, crypto, dependency, Cargo manifest,
lockfile, workflow, executable test source, fuzz target, vector, formal model,
qsl-server, qsl-attachments, qshield runtime, qshield-cli, website, public
docs, README, START_HERE, qwork/qstart/qresume/qshell, qsl-backup, backup
status, backup plan, rollback subtree, and backup tree paths.

Forbidden commands include qwork, qstart, qresume, sudo, backup, restore,
generated operator scripts, `cargo update`, `cargo generate-lockfile`,
dependency remediation commands, workflow mutation, branch-protection mutation,
and NA-0448 implementation.

No public technical paper work is in scope.

## PR #1163 merge/public-safety checks

Run:

```bash
gh pr view 1163 --repo QuantumShieldLabs/qsl-protocol --json number,state,mergedAt,mergeCommit,headRefOid,title,url
python3 scripts/ci/qsl_evidence_helper.py public-safety-status --sha db2aaa7a83115f355eb4561b605f60fabd74773a
```

Required:

- PR #1163 state is MERGED.
- PR #1163 merge commit starts with `db2aaa7a831`.
- post-merge public-safety is completed success.
- public-safety is not red or ambiguous.

## Queue and decision checks

Run:

```bash
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
```

Required:

- READY_COUNT 1.
- READY NA-0448.
- NA-0447 DONE.
- NA-0446 DONE.
- NA-0445 DONE.
- NA-0444 DONE.
- NA-0443 DONE.
- NA-0442 DONE.
- NA-0441 DONE.
- NA-0440 DONE.
- NA-0439 DONE.
- NA-0438 DONE.
- NA-0437 DONE.
- NA-0436 DONE.
- NA-0435 DONE.
- NA-0434 BLOCKED.
- NA-0429 BLOCKED.
- D-0881 exists once.
- D-0882 exists once.
- D-0883 absent.
- duplicate decision count zero.

## Scope guard

Run after patching and before PR creation:

```bash
git diff --name-only origin/main
git ls-files --others --exclude-standard
python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main --allowed NEXT_ACTIONS.md DECISIONS.md TRACEABILITY.md docs/ops/ROLLING_OPERATIONS_JOURNAL.md tests/NA-0447_closeout_restore_na0448_testplan.md
```

The combined changed-path set must be exactly:

- `DECISIONS.md`
- `NEXT_ACTIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0447_closeout_restore_na0448_testplan.md`

## Link, leak, classifier, PR-body, and goal checks

Run:

```bash
git diff --check
python3 scripts/ci/qsl_evidence_helper.py link-check
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
python3 scripts/ci/qsl_evidence_helper.py pr-body-preflight --file "$PR_BODY_FILE"
bash scripts/ci/classify_ci_scope.sh $(git diff --name-only origin/main)
```

Required:

- no whitespace errors;
- link check passes;
- added-line leak scan has zero findings;
- added-line overclaim scan has zero affirmative findings;
- PR body includes `Goals:`, `Impact:`, `No-regression:`, and
  `Tests/Vectors:`;
- goal-lint passes;
- classifier reports governance/docs scope only.

## Dependency and inherited health checks

Run:

```bash
cargo audit --deny warnings
cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock
cargo tree -i rustls-webpki --locked
cargo tree -i ml-kem --locked || true
cargo tree -i pqcrypto-mlkem --locked || true
cargo tree -i pqcrypto-traits --locked || true
cargo tree -i pqcrypto-internals --locked || true
rg -n "pqcrypto-mlkem|pqcrypto-traits|pqcrypto-internals" qsl/qsl-client/qsc/fuzz/Cargo.lock || true
cargo test -p qsc --locked --test key_lifecycle_zeroization -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test handshake_provider_error_no_mutation -- --test-threads=1 --nocapture
```

Required:

- root cargo audit passes;
- nested qsc fuzz lock cargo audit passes;
- `rustls-webpki` remains on a safe version;
- root pqcrypto inverse probes remain expected package-ID absence evidence;
- nested qsc fuzz lock pqcrypto scan remains zero-match evidence;
- qsc key lifecycle and provider-error tests pass;
- no Cargo manifest or lockfile mutation occurs.

## NA-0448 authorization boundary check

Confirm that the restored NA-0448 block preserves:

- no runtime mutation;
- no crypto mutation;
- no dependency mutation;
- no Cargo or lockfile mutation;
- no workflow mutation;
- no executable test implementation;
- no fuzz target mutation;
- no vector mutation;
- no formal model mutation;
- no qsl-server or qsl-attachments mutation;
- no qshield runtime or qshield-cli mutation;
- no public docs, website, README, or START_HERE mutation;
- no backup, restore, qsl-backup, status, plan, rollback, or backup tree
  mutation;
- no public-readiness claim;
- no production-readiness claim;
- no public-internet-readiness claim;
- no external-review-complete claim;
- no crypto-complete claim;
- no RNG-failure-complete claim;
- no side-channel-free claim;
- no vulnerability-free claim;
- no bug-free claim;
- no perfect-crypto claim.

## Post-merge checks

After closeout merge:

```bash
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
python3 scripts/ci/qsl_evidence_helper.py public-safety-status --sha <merge-sha>
```

Required:

- READY_COUNT 1.
- READY NA-0448.
- NA-0447 DONE.
- D-0882 exists once.
- D-0883 absent.
- duplicate decision count zero.
- post-merge public-safety completed success.
- Codex does not run qwork post-merge.
