Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-02

# NA-0404 Director State Index Fixture Matrix Prerequisite Recovery Testplan

Goals: G1, G2, G3, G4, G5

## Objective

Verify that the inherited Director State Index fixture matrix prerequisite is
repaired by adding only the missing `secret_sentinel_reject.json` fixture while
preserving helper logic, queue state, public-claim boundaries, and no durable
index output.

## Protected Invariants

- READY_COUNT remains exactly 1.
- READY remains NA-0404.
- NA-0403 remains DONE.
- D-0788 exists once.
- D-0789 exists once.
- D-0790 is added once by this recovery.
- D-0791 remains absent.
- Public-safety remains required.
- The helper remains unchanged.
- Existing fixture files remain unchanged.

## Allowed Scope

- `inputs/local_ops/director_state_index_fixtures/secret_sentinel_reject.json`
- `docs/governance/evidence/NA-0404_director_state_index_fixture_matrix_prerequisite_recovery.md`
- `tests/NA-0404_director_state_index_fixture_matrix_prerequisite_recovery_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Forbidden Scope

- `scripts/ci/qsl_director_state_index.py`
- any other helper or script file.
- any existing fixture file.
- `.github/**`
- `Cargo.toml`
- `Cargo.lock`
- runtime, protocol, crypto, qshield runtime, qsl-server, qsl-attachments,
  qsc-desktop, website, README, START_HERE, docs/public, backup scripts,
  backup timers, fstab, source lists, durable Director State Index output,
  response archives, local history, qstart/qresume tooling, and secret-handling
  paths.

## D226 Inheritance

D226 proved true `origin/main` was dependency-healthy and then stopped because
the helper required `secret_sentinel_reject.json` while tracked main contained
only 19 fixture JSON files. This recovery starts from that root cause.

## Pre-Patch Reproduction

Run the fixture matrix before adding the missing fixture:

`python3 scripts/ci/qsl_director_state_index.py fixture --fixtures-dir inputs/local_ops/director_state_index_fixtures --tmp-dir /srv/qbuild/tmp/NA0403_director_state_index_NA0404_prerecovery_fixture_check --json`

Acceptance: command fails only because the required
`secret_sentinel_reject.json` fixture is missing.

## Fixture Schema Discovery

Read `scripts/ci/qsl_director_state_index.py` and neighboring fixtures
read-only. Confirm:

- fixture schema is `qsl.director_state_index.fixture_case.v1`;
- fixture cases may use `set` and `remove`;
- `secret_sentinel_reject.json` is expected to fail;
- a harmless helper-recognized test sentinel can trigger the rejection path; and
- no helper mutation is needed.

## Missing Fixture Requirements

The new fixture must:

- match neighboring fixture formatting;
- use deterministic JSON;
- include no real secret, credential, token, private key, passphrase, password,
  or recovery-envelope material;
- use only a harmless helper-recognized test sentinel;
- produce the expected fail result; and
- require no changes to helper logic or existing fixtures.

## Fixture Matrix Acceptance

After adding the fixture, run:

`python3 scripts/ci/qsl_director_state_index.py fixture --fixtures-dir inputs/local_ops/director_state_index_fixtures --tmp-dir /srv/qbuild/tmp/NA0403_director_state_index_NA0404_recovered_fixture_check --json`

Acceptance:

- `fixture_count=20`;
- `pass_count=20`;
- `fail_count=0`;
- `secret_sentinel_reject.json` fails with the helper's secret-sentinel
  rejection; and
- output remains under the requested `/srv/qbuild/tmp` directory.

## No-Secret / Leak-Scan Requirements

Run:

- `python3 scripts/ci/qsl_evidence_helper.py leak-scan`
- targeted high-confidence credential-pattern scan over the new fixture.

Acceptance:

- added-line leak scan reports zero findings;
- targeted scan reports zero matches; and
- required PR checks, including CodeQL and public-safety, are not bypassed.

## No-Helper-Mutation Requirements

Verify `scripts/ci/qsl_director_state_index.py` has no diff and still passes:

- `python3 -m py_compile scripts/ci/qsl_director_state_index.py`
- `python3 scripts/ci/qsl_director_state_index.py --help`

## No Durable Index Requirements

Verify no durable Director State Index output is created. Fixture proof output
must stay under `/srv/qbuild/tmp/NA0403_director_state_index_*`.

## Queue Preservation Requirements

Run:

- `python3 scripts/ci/qsl_evidence_helper.py queue`
- `python3 scripts/ci/qsl_evidence_helper.py decisions`

Acceptance:

- READY_COUNT 1;
- READY NA-0404;
- D-0790 exists once after the patch;
- D-0791 absent; and
- duplicate decision count zero.

## CI Expectations

Run local validation: diff check, fixture matrix, helper compile/help, cargo
audit, rustls-webpki tree, cargo fmt, qsc send_commit, formal model checks,
qshield-cli test/build if feasible, scope guard, link-check, leak-scan,
classifier, goal-lint or PR body preflight, and overclaim scan.

The PR may merge only after required checks pass without admin bypass, squash,
rebase, direct push, branch deletion flags, force-push, or amend.

## Successor Handoff

After merge, leave NA-0404 READY and recommend retrying NA-0404 durable storage
and backup-impact authorization. Do not proceed to NA-0405 until NA-0404
completes.
