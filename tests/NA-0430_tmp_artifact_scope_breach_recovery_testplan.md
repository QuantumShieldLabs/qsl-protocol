Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-06

# NA-0430 Temporary Proof Artifact Scope-Breach Recovery Testplan

## Objective

Validate that D273 recovers D272's `/tmp/na0430_*` proof-output boundary breach without changing queue order, source/runtime behavior, dependency state, workflows, executable tests, vectors, backup state, qwork tools, sibling repos, or public claims.

## Protected Invariants

- D272 completion evidence remains preserved.
- PR #1129 and PR #1130 remain merged.
- NA-0430 remains DONE.
- NA-0429 remains BLOCKED.
- NA-0431 remains the sole READY item.
- D-0847 and D-0848 remain exactly once.
- D-0849 is added exactly once by the recovery.
- D-0850 remains absent.
- Duplicate decision count remains zero.

## Allowed Scope

Allowed qsl-protocol changed paths are exactly:

- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/governance/evidence/NA-0430_tmp_artifact_scope_breach_recovery.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0430_tmp_artifact_scope_breach_recovery_testplan.md`

Allowed local cleanup is limited to moving exact safe `/tmp/na0430_*` files into the authorized proof root.

## Forbidden Scope

The recovery must not mutate:

- `NEXT_ACTIONS.md`;
- runtime code;
- crypto code;
- dependency manifests;
- lockfiles;
- workflows;
- scripts;
- executable tests;
- fuzz targets;
- vectors;
- qsl-server;
- qsl-attachments;
- qshield runtime;
- website;
- public docs;
- README;
- START_HERE;
- qwork/qstart/qresume/qshell;
- qsl-backup;
- backup status files;
- backup plan files;
- rollback subtree paths;
- `/backup/qsl`.

## D272 Inheritance Checks

Required checks:

- D272 response file exists.
- PR #1129 state is MERGED.
- PR #1130 state is MERGED.
- `main` equals `origin/main`.
- `origin/main` descends from `9d22e3062c47`.
- Queue helper reports READY_COUNT 1 and READY NA-0431.
- Direct queue excerpt shows NA-0430 DONE and NA-0429 BLOCKED.
- Decision helper reports latest D-0848 before recovery patch.

## Candidate Safety Checks

For each `/tmp/na0430_*` candidate:

- require exact `/tmp/na0430_` prefix;
- require regular file;
- reject symlink;
- require current-user owner;
- require size at or below 10 MB;
- record stat, pre-move SHA256, `file` output, and bounded text excerpts when safe;
- run high-confidence secret scan before moving.

If any candidate fails, stop before PR.

## Move/No-Op Validation

If candidates exist:

- move each exact candidate path with `mv --`;
- verify the original path is absent;
- verify the moved file exists under the proof root;
- verify post-move SHA256 equals pre-move SHA256;
- rerun the `/tmp/na0430_*` discovery and require zero remaining candidates.

If no candidates exist:

- record `NO_STRAY_NA0430_TMP_FILES_FOUND`;
- continue with governance evidence.

## qsl-Protocol Scope Guard

Required checks:

- `git diff --name-only` matches exactly the five allowed paths.
- `git diff --check` passes.
- `python3 scripts/ci/qsl_evidence_helper.py scope-guard` reports no forbidden path.
- No Cargo, workflow, source, executable-test, fuzz-target, vector, public, service, backup, qwork, status, plan, rollback, README, or START_HERE path appears in the changed path set.

## NA-0431 Remains READY Check

Required checks:

- `python3 scripts/ci/qsl_evidence_helper.py queue` reports READY_COUNT 1.
- The READY item is NA-0431.
- NA-0430 remains DONE.
- NA-0429 remains BLOCKED.
- `NEXT_ACTIONS.md` is not modified by this recovery.

## Dependency Health Checks

Required checks:

- `cargo audit --deny warnings`
- `cargo tree -i rustls-webpki --locked`
- `cargo tree -i ml-kem --locked || true`
- `cargo tree -i pqcrypto-mlkem --locked || true`
- `cargo tree -i pqcrypto-traits --locked || true`
- `cargo tree -i pqcrypto-internals --locked || true`

Root cargo audit must pass. Root pqcrypto package-ID probes must not show an active root blocker.

## Public-Safety Requirements

Required checks:

- public-safety helper reports green on current `origin/main` before patch;
- PR checks attach and pass before merge;
- public-safety is green after merge.

The recovery must not change public-safety workflows or helper scripts.

## No Public Overclaim Requirements

Changed content and the PR body must not introduce public claim expansion. Evidence may state dependency-health, queue, recovery, and no-mutation facts, but must not present internal recovery as broader security assurance, public release evidence, external review completion, website readiness, broad defect absence, or full cryptographic completeness.
