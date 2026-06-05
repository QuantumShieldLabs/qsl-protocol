Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-05

# NA-0428 Closeout / Restore NA-0429 Testplan

Goals: G1, G2, G3, G4, G5

## Purpose

Verify that NA-0428 is closed only after the qsc fuzz-lock authorization PR
merged and post-merge public-safety passed, and that the exact lockfile-only
NA-0429 successor is restored as the sole READY item without implementing
NA-0429 or remediating the nested qsc fuzz lock.

## Preconditions

- PR #1125 is MERGED.
- PR #1125 merge commit is `0929deb8ddc7`.
- Post-merge public-safety on `0929deb8ddc7` completed success.
- Queue before closeout has READY_COUNT 1 and READY NA-0428.
- D-0844 exists once.
- D-0845 is absent before closeout.
- Duplicate decision count is zero.
- D-0844 selected
  `NA-0429 -- QSL qsc Fuzz Lock pqcrypto Residual Lockfile Cleanup Implementation Harness`.

## Allowed closeout paths

- `NEXT_ACTIONS.md`;
- `DECISIONS.md`;
- `TRACEABILITY.md`;
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`;
- `tests/NA-0428_closeout_restore_na0429_testplan.md`.

No other qsl-protocol paths may change.

## Forbidden scope

This closeout must not:

- implement NA-0429;
- remediate the nested qsc fuzz lock;
- mutate runtime, crypto, root dependency files, Cargo files, lockfiles,
  workflow, fuzz target, test, vector, qsl-server, qsl-attachments, qshield
  runtime, website, public docs, README, or START_HERE paths;
- run qwork, qstart, or qresume;
- run backup or restore;
- mutate qsl-backup;
- mutate backup status or backup plan files;
- mutate qwork/qstart/qresume/qshell;
- create public technical paper content;
- allow more than one READY item;
- handle secret material;
- create or imply unsupported public, security, readiness, privacy, backup, or
  restore claims;
- create or imply unsupported external-review or cryptographic-completeness
  claims;
- create or imply unsupported side-channel assurance, defect-absence,
  vulnerability-absence, or absolute-crypto-assurance claims.

## Required NEXT_ACTIONS changes

- Mark NA-0428 DONE.
- Restore exactly one READY item:
  `NA-0429 -- QSL qsc Fuzz Lock pqcrypto Residual Lockfile Cleanup Implementation Harness`.
- NA-0429 block must include:
  - lockfile-only cleanup objective;
  - allowed future mutation of `qsl/qsl-client/qsc/fuzz/Cargo.lock`;
  - no runtime mutation;
  - no crypto mutation;
  - no root dependency mutation;
  - no workflow mutation;
  - no fuzz target mutation;
  - no test or vector mutation;
  - no public overclaim;
  - no backup/restore/qsl-backup/status/plan mutation;
  - no secret material handling;
  - exactly-one-READY discipline.

## Required decision

Add D-0845:

`NA-0428 closeout and NA-0429 restoration`

D-0845 must state:

- NA-0428 is closed after PR #1125 merged and post-merge public-safety passed.
- D-0844 authorized lockfile-only remediation and selected NA-0429.
- NA-0429 is restored as the sole READY successor.
- The closeout does not implement NA-0429.
- The closeout does not remediate the nested qsc fuzz lock.
- The closeout preserves one-READY discipline, public-claim boundaries,
  no runtime/crypto/root-dependency/Cargo/lockfile/workflow/fuzz-target/test/vector mutation,
  no backup mutation, and no public overclaim.

## Validation commands

Run:

```bash
git diff --check
python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main --head HEAD \
  --allowed NEXT_ACTIONS.md \
  --allowed DECISIONS.md \
  --allowed TRACEABILITY.md \
  --allowed docs/ops/ROLLING_OPERATIONS_JOURNAL.md \
  --allowed tests/NA-0428_closeout_restore_na0429_testplan.md \
  --forbidden .github/ \
  --forbidden Cargo.toml \
  --forbidden Cargo.lock \
  --forbidden qsl/ \
  --forbidden qsl-server/ \
  --forbidden qsl-attachments/ \
  --forbidden apps/ \
  --forbidden website/ \
  --forbidden README.md \
  --forbidden START_HERE.md \
  --forbidden docs/governance/evidence/NA-0428_qsl_qsc_fuzz_lock_pqcrypto_residual_dependency_blocker_authorization_plan.md \
  --forbidden tests/NA-0428_qsl_qsc_fuzz_lock_pqcrypto_residual_dependency_blocker_authorization_testplan.md
python3 scripts/ci/qsl_evidence_helper.py link-check --root .
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
cargo audit --deny warnings
cargo tree -i rustls-webpki --locked
cargo tree -i pqcrypto-mlkem --locked || true
cargo tree -i pqcrypto-traits --locked || true
cargo tree -i pqcrypto-internals --locked || true
cargo fmt --check
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
python3 formal/model_qsc_handshake_suite_id_bounded.py
python3 formal/run_model_checks.py
```

Additional checks:

- exact changed-path guard for the five allowed closeout paths;
- added-line overclaim scan;
- classifier proof;
- PR body preflight;
- goal-lint with `Goals: G1, G2, G3, G4, G5`;
- required checks, including public-safety, green before merge;
- post-merge public-safety green after merge.

## Acceptance criteria

- READY_COUNT 1.
- READY NA-0429.
- NA-0428 DONE.
- D-0844 exists once.
- D-0845 exists once.
- D-0846 absent.
- Duplicate decision count 0.
- Only the five allowed closeout paths changed.
- No NA-0429 implementation by closeout.
- No runtime/crypto/root-dependency/Cargo/lockfile/workflow/fuzz-target/test/vector/public/service/backup mutation.
- No public overclaim.

## Post-fix hardening review checklist

- Correctness under stress: queue restoration is deterministic and preserves
  one READY item.
- Minimality: only closeout governance/testplan paths changed.
- Maintainability: NA-0429 carries explicit allowed/forbidden scope and
  acceptance criteria.
- Coverage quality: queue, decisions, scope, link, leak, overclaim, classifier,
  dependency, qsc, and formal/model checks verify the closeout.
- Cross-lane stability: public-safety remains green and no platform-specific
  runtime path changes are introduced.
