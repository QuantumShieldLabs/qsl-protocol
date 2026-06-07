# NA-0436 qsc pq_decap_failed No-Mutation Test Implementation Testplan

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-06

Goals: G1, G2, G3, G4, G5

## Purpose

Validate the narrowed NA-0435-authorized `pq_decap_failed` no-mutation qsc
integration test and prove the PR stays inside exact NA-0436 scope.

## Scope

Allowed implementation path:

- `qsl/qsl-client/qsc/tests/handshake_provider_error_no_mutation.rs`

Allowed governance paths:

- `docs/governance/evidence/NA-0436_qsl_qsc_pq_decap_failed_no_mutation_test_implementation_harness.md`
- `tests/NA-0436_qsl_qsc_pq_decap_failed_no_mutation_test_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Forbidden changed paths include runtime, crypto, dependency, Cargo, lockfile,
workflow, fuzz target, vector, qsl-server, qsl-attachments, qshield runtime,
website, public docs, README, START_HERE, qwork/qstart/qresume/qshell,
qsl-backup, backup status, backup plan, rollback, and `/backup/qsl` paths.

## qwork proof checks

- Confirm qwork proof files exist under `/srv/qbuild/work/NA-0436/.qwork/`.
- Confirm `.kv` reports `startup_result=OK`, `lane=NA-0436`,
  `repo=qsl-protocol`, clean worktree/index/untracked state, READY_COUNT 1,
  READY NA-0436, and requested lane status READY.
- Confirm JSON proof parses and mirrors the `.kv` proof for lane, repo, path,
  head, origin/main, clean-state fields, READY count, queue top, and requested
  lane status.
- Confirm Codex did not run qwork, qstart, or qresume.
- Confirm live `HEAD` and `origin/main` match the qwork proof after fetch.

## Implementation checks

- Confirm the test file was absent or copied to rollback before mutation.
- Confirm the test targets only `pq_decap_failed`.
- Confirm the test uses existing qsc CLI/test APIs and mock relay fixtures.
- Confirm the decap failure is triggered by malformed pending KEM secret state.
- Confirm the test asserts `event=handshake_reject` with `pq_decap_failed`.
- Confirm Alice session state remains unchanged.
- Confirm Alice pending/vault state remains unchanged after the reject.
- Confirm Bob responder pending/vault state remains unchanged or is not touched
  by Alice's reject path.
- Confirm no A2 frame is emitted after reject.
- Confirm the `pq_encap_failed` caveat is explicit and no executable coverage
  claim is made for that branch.
- Confirm no runtime hook, provider fake, provider seam, runtime code, crypto
  code, dependency, Cargo, lockfile, workflow, fuzz target, or vector mutation
  is introduced.

## Validation commands

Required local validation:

- `git diff --check`
- exact allowed-path scope guard
- link check
- leak scan
- overclaim scan
- classifier
- PR body preflight
- goal-lint
- `cargo test -p qsc --locked --test handshake_provider_error_no_mutation -- --test-threads=1 --nocapture`
- `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1`
- `cargo test -p quantumshield_refimpl --features pqcrypto --locked --test pqkem768`
- `cargo audit --deny warnings`
- `cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock`
- `cargo tree -i rustls-webpki --locked`
- `cargo tree -i ml-kem --locked`
- pqcrypto inverse tree absence probes
- nested qsc fuzz lock pqcrypto residual scan
- `cargo fmt --check`
- `python3 formal/model_qsc_handshake_suite_id_bounded.py`
- `python3 formal/run_model_checks.py`
- `scripts/ci/qsc_adversarial.sh` if locally feasible, with missing
  cargo-fuzz tooling recorded and PR CI `qsc-adversarial-smoke` required if
  local cargo-fuzz is unavailable.

Required before PR:

- READY_COUNT 1.
- READY NA-0436.
- D-0859 exists once.
- D-0860 absent.
- no duplicate decision IDs.
- changed paths limited to the exact authorized implementation path plus five
  allowed NA-0436 governance paths.
- exact new qsc test passes.
- qsc `send_commit` passes.
- provider `pqkem768` passes.
- formal checks pass.
- root cargo audit green.
- nested qsc fuzz lock audit green.
- no runtime/crypto/dependency/Cargo/lockfile/workflow/fuzz-target/vector
  mutation.
- no backup or restore by Codex.
- no qsl-backup, backup status, backup plan, or qwork mutation.
- no public-readiness claim.
- no production-readiness claim.
- no external-review claim.
- no crypto-complete claim.
- no side-channel-free claim.
- no bug-free claim.
- no vulnerability-free claim.
- no perfect-crypto claim.

## PR / merge checks

- PR body includes standalone `Goals: G1, G2, G3, G4, G5`.
- PR body includes Impact, No-regression, and Tests/Vectors.
- PR body mentions the exact test path, `pq_decap_failed` coverage,
  `pq_encap_failed` caveat preservation, no runtime/crypto/dependency/Cargo
  /lockfile/workflow/fuzz/vector mutation, root and nested audits, no public
  overclaim, and selected NA-0437 successor.
- Merge uses a merge commit after required checks pass.
- Post-merge public-safety must be green before optional closeout.
