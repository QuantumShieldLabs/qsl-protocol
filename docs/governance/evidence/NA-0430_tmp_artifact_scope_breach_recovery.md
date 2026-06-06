Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-06

# NA-0430 Temporary Proof Artifact Scope-Breach Recovery

## Executive Summary

D272 completed NA-0430 evidence and closeout, merged PR #1129 and PR #1130, and restored NA-0431 as the sole READY item. After closeout, D272 reported a proof-output boundary breach: a dependency probe wrote temporary files under `/tmp/na0430_*` instead of the authorized proof root.

D273 recovered that operational breach. Six exact candidate files were found, safety-checked, scanned for high-confidence secrets, moved into `/srv/qbuild/tmp/NA0430_tmp_artifact_recovery_20260606T114326Z/stray_tmp/`, and verified by checksum. No current-user `/tmp/na0430_*` files remain after recovery.

Classification: `NA0430_TMP_PROOF_ARTIFACT_SCOPE_BREACH_RECOVERED`.

## D272 Inheritance

- D272 response exists at `/home/victor/work/qsl/codex/responses/NA0430_20260605T214500-0500_D272.md`.
- PR #1129 is MERGED at `19c4624dfe8b`.
- PR #1130 is MERGED at `9d22e3062c47`.
- D272 records NA-0430 DONE and NA-0431 READY.
- D272 reported the `/tmp/na0430_*` proof-output boundary issue and stopped further optional evidence collection.

## Current Repo/Queue State

- Refreshed `main` equals `origin/main` at `9d22e3062c47`.
- `origin/main` descends from `9d22e3062c47`.
- Queue helper output: READY_COUNT 1.
- Sole READY item: `NA-0431 -- QSL qsc Fuzz Lock Precise-Version pqcrypto Cleanup Retry Implementation Harness`.
- Direct queue proof shows NA-0430 DONE and NA-0429 BLOCKED.
- Decision helper output before this patch: latest D-0848, D-0847 once, D-0848 once, D-0849 absent, duplicate count zero.

## Stray `/tmp/na0430_*` Discovery

The exact discovery command was:

```bash
find /tmp -maxdepth 1 -user "$(id -un)" -type f -name 'na0430_*' -print | sort
```

It found six files:

- `/tmp/na0430_pqcrypto-internals.err`
- `/tmp/na0430_pqcrypto-internals.out`
- `/tmp/na0430_pqcrypto-mlkem.err`
- `/tmp/na0430_pqcrypto-mlkem.out`
- `/tmp/na0430_pqcrypto-traits.err`
- `/tmp/na0430_pqcrypto-traits.out`

## Candidate Safety Checks

Each candidate was checked for:

- exact `/tmp/na0430_` prefix;
- regular-file type;
- non-symlink status;
- current-user ownership;
- size at or below 10 MB;
- stat output;
- pre-move SHA256;
- `file` output;
- bounded first/last text excerpts where applicable.

The high-confidence secret scan over the six candidates reported `SECRET_FINDING_COUNT 0`.

## Move/No-Op Proof

All six safe candidates were moved with exact `mv --` source paths into:

`/srv/qbuild/tmp/NA0430_tmp_artifact_recovery_20260606T114326Z/stray_tmp/`

Each original path was absent after its move, each destination existed under the proof root, and each post-move checksum matched its pre-move checksum.

## Post-Move Verification

The post-move discovery command reported zero remaining current-user files matching `/tmp/na0430_*`.

The moved files remain preserved under the proof root for audit continuity.

## qsl-Protocol No-Mutation Proof

Before and after the temp-artifact recovery, qsl-protocol was clean on `main` / `origin/main` before the governance recovery branch was created. This recovery patch changes only:

- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/governance/evidence/NA-0430_tmp_artifact_scope_breach_recovery.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0430_tmp_artifact_scope_breach_recovery_testplan.md`

It changes no runtime code, crypto code, dependency manifests, lockfiles, workflows, scripts, executable tests, fuzz targets, vectors, public docs, README, START_HERE, service repo, qwork/qstart/qresume/qshell path, backup path, status file, plan file, or rollback subtree.

## Dependency/Main Health Proof

- Root `cargo audit --deny warnings` passed on current `main`.
- `cargo tree -i rustls-webpki --locked` reported `rustls-webpki v0.103.13`.
- `cargo tree -i ml-kem --locked` reported root `ml-kem v0.2.1`.
- Root inverse-tree probes for `pqcrypto-mlkem`, `pqcrypto-traits`, and `pqcrypto-internals` reported package-ID absence.

These checks are dependency-health evidence only.

## Public-Safety Proof

`python3 scripts/ci/qsl_evidence_helper.py public-safety-status --sha 9d22e3062c47 --repo QuantumShieldLabs/qsl-protocol` reported:

- `public-safety` completed success;
- `qsc-adversarial-smoke` completed success;
- `qsc-linux-full-suite` skipped;
- `macos-qsc-full-serial` skipped;
- no red or ambiguous public-safety state.

## Backup/qsl-Backup Boundary Proof

- `/usr/local/sbin/qsl-backup` SHA matched the directive-required value.
- Narrow source-list proof showed `/home/victor/work/qsl/codex/ops` appears exactly once in `/usr/local/sbin/qsl-backup`.
- Read-only hashes were captured for qsl-backup, the backup plan, and the backup status file.
- Codex did not run backup, restore, sudo, generated operator scripts, qwork, qstart, qresume, cargo update, lockfile remediation commands, or lockfile generation.
- Codex did not touch `/backup/qsl`.

## Public Claim/External Review/Website Boundary

This recovery records operational evidence only. It makes no claim expansion for public materials, website status, external review status, release status, broad security completeness, or absence of defects. Cargo audit green remains dependency-health evidence only.

## Rejected Alternatives

- Delete the stray files: rejected because the directive required evidence preservation.
- Leave safe files in `/tmp`: rejected because the authorized recovery scope provided a proof root.
- Move arbitrary `/tmp` content: rejected because only exact safe `/tmp/na0430_*` files were authorized.
- Start NA-0431: rejected because this directive is recovery-only.
- Edit NEXT_ACTIONS: rejected because NA-0431 was already the sole READY item and queue mutation was out of scope.

## Backup-Impact Statement

No backup configuration, backup executable, backup status, backup plan, backup source list, rollback subtree, or `/backup/qsl` path was mutated. This recovery has no backup-impact beyond read-only boundary proof.

## Next Recommendation

Proceed to NA-0431 only under a separate directive. NA-0431 should retain the D-0847/D-0848 precise-version lockfile-only scope and must not inherit D272's temp-proof output mistake.
