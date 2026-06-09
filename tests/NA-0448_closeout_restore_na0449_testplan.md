# NA-0448 Closeout and NA-0449 Restoration Testplan

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-09

Goals: G1, G2, G3, G4, G5

## Objective

Validate that NA-0448 is closed only after PR #1165 merged, post-merge
public-safety passed, and D-0883 selected the qsc RNG failure test seam
implementation successor. Restore exactly one successor, NA-0449, as READY
without implementing NA-0449.

## Protected invariants

- READY_COUNT remains 1.
- NA-0448 is DONE after closeout.
- NA-0449 is READY after closeout.
- NA-0447 remains DONE.
- NA-0434 remains BLOCKED.
- NA-0429 remains BLOCKED.
- D-0883 exists once.
- D-0884 exists once after closeout.
- D-0885 remains absent until future NA-0449 work.
- PR #1165 remains merged at `17018e34b001`.
- Post-merge public-safety is green on `17018e34b001`.
- NA-0449 is implementation scope only for exact D-0883 paths.
- No NA-0449 implementation occurs in this closeout.
- No runtime, crypto, dependency, Cargo, lockfile, workflow, executable test,
  fuzz target, vector, formal model, service, public-surface, or
  backup/local-ops path is changed outside this closeout's allowed governance
  paths.
- No public claim expansion is introduced.

## Allowed scope

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0448_closeout_restore_na0449_testplan.md`

## Forbidden scope

Forbidden changed paths include runtime, crypto, dependency, Cargo manifest,
lockfile, workflow, executable test source, fuzz target, vector, formal model,
qsl-server, qsl-attachments, qshield runtime, qshield-cli, website, public
docs, README, START_HERE, qwork/qstart/qresume/qshell, qsl-backup, backup
status, backup plan, rollback subtree, and backup tree paths.

Forbidden commands include qwork, qstart, qresume, sudo, backup, restore,
generated operator scripts, `cargo update`, `cargo generate-lockfile`,
dependency remediation commands, workflow mutation, branch-protection mutation,
and NA-0449 implementation.

No public technical paper work is in scope.

## PR #1165 merge/public-safety checks

Run:

```bash
gh pr view 1165 --repo QuantumShieldLabs/qsl-protocol --json number,state,mergedAt,mergeCommit,headRefOid,title,url
python3 scripts/ci/qsl_evidence_helper.py public-safety-status --sha 17018e34b0010f6ad1ac8bf15be3d0e509105eb9
```

Required:

- PR #1165 state is MERGED.
- PR #1165 merge commit starts with `17018e34b001`.
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
- READY NA-0449.
- NA-0448 DONE.
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
- D-0883 exists once.
- D-0884 exists once.
- D-0885 absent.
- duplicate decision count zero.

## Scope guard

Run after patching and before PR creation:

```bash
git diff --name-only origin/main
git ls-files --others --exclude-standard
python3 scripts/ci/qsl_evidence_helper.py scope-guard \
  --base origin/main \
  --allowed NEXT_ACTIONS.md \
  --allowed DECISIONS.md \
  --allowed TRACEABILITY.md \
  --allowed docs/ops/ROLLING_OPERATIONS_JOURNAL.md \
  --allowed tests/NA-0448_closeout_restore_na0449_testplan.md
```

The combined changed-path set must be exactly:

- `DECISIONS.md`
- `NEXT_ACTIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0448_closeout_restore_na0449_testplan.md`

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

## NA-0449 restoration boundary check

Confirm that the restored NA-0449 block preserves:

- exact D-0883 future mutable implementation paths;
- no implementation by closeout;
- no mutation outside closeout governance paths;
- no runtime/crypto/dependency/Cargo/lockfile/workflow/public/service/backup
  expansion outside the exact future D-0883 implementation scope;
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

## PR and post-merge checks

Before merge:

- local validation passes.
- PR required checks pass.
- public-safety is green.

After merge:

- verify READY NA-0449.
- verify NA-0448 DONE.
- verify D-0884 on main.
- verify public-safety is green on the closeout merge commit.
- do not run qwork post-merge.
