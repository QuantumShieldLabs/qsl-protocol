# NA-0435 qsc Provider Error Strategy Authorization Testplan

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-06

Goals: G1, G2, G3, G4, G5

## Purpose

Validate that NA-0435 consumes D278/D279 provider-error stop evidence and
selects an exact NA-0436 successor without implementing hooks, provider fakes,
runtime changes, crypto changes, dependency changes, tests, vectors, workflows,
public-surface changes, or backup/local-ops mutation.

## Scope

Allowed NA-0435 changed paths:

- `docs/governance/evidence/NA-0435_qsl_qsc_provider_error_path_test_hook_defensive_branch_authorization_plan.md`
- `tests/NA-0435_qsl_qsc_provider_error_path_test_hook_defensive_branch_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Forbidden changed paths include runtime, crypto, dependency, Cargo, lockfile,
workflow, executable test, fuzz target, vector, qsl-server, qsl-attachments,
qshield runtime, website, public docs, README, START_HERE, qwork/qstart/qresume
/qshell, qsl-backup, backup status, backup plan, rollback, and `/backup/qsl`
paths.

## qwork proof checks

- Confirm qwork proof files exist under `/srv/qbuild/work/NA-0435/.qwork/`.
- Confirm `.kv` reports `startup_result=OK`, `lane=NA-0435`,
  `repo=qsl-protocol`, clean worktree/index/untracked state, READY_COUNT 1,
  READY NA-0435, and requested lane status READY.
- Confirm JSON proof parses and mirrors the `.kv` proof for lane, repo, path,
  head, origin/main, clean-state fields, READY count, queue top, and requested
  lane status.
- Confirm Codex did not run qwork, qstart, or qresume.
- Confirm live `HEAD` and `origin/main` match the qwork proof after fetch.

## Inheritance checks

- Confirm D278 response exists at
  `/home/victor/work/qsl/codex/responses/NA0434_20260607T013227Z_D278.md`.
- Confirm D279 response exists at
  `/home/victor/work/qsl/codex/responses/NA0434_20260607T023903Z_D279.md`.
- Confirm D278 proof root exists.
- Confirm D279 recovery evidence exists.
- Confirm PR #1138 is merged at `a1c15d8ac377`.
- Confirm NA-0434 remains BLOCKED and NA-0435 remains READY until optional
  closeout.

## Strategy checks

- Confirm `pq_encap_failed` classification:
  `ENCAP_FAILED_DEFENSIVE_BRANCH_DOCUMENTATION_CANDIDATE`.
- Confirm secondary `pq_encap_failed` classification:
  `ENCAP_FAILED_TEST_ONLY_SEAM_REQUIRES_RUNTIME_CHANGE`.
- Confirm `pq_decap_failed` classification:
  `DECAP_ONLY_TEST_IMPLEMENTATION_READY`.
- Confirm primary authorization classification:
  `NARROW_DECAP_ONLY_TEST_AUTHORIZATION_READY`.
- Confirm selected successor:
  `NA-0436 -- QSL qsc pq_decap_failed No-Mutation Test Implementation Harness`.
- Confirm the future NA-0436 path bundle is exact and includes only:
  - `qsl/qsl-client/qsc/tests/handshake_provider_error_no_mutation.rs`
  - `docs/governance/evidence/NA-0436_qsl_qsc_pq_decap_failed_no_mutation_test_implementation_harness.md`
  - `tests/NA-0436_qsl_qsc_pq_decap_failed_no_mutation_test_implementation_testplan.md`
  - `DECISIONS.md`
  - `TRACEABILITY.md`
  - `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Public claim checks

- Confirm the evidence states this is internal governance evidence only.
- Confirm no public readiness, production readiness, public-internet readiness,
  external-review completion, crypto-complete, side-channel-free, bug-free,
  vulnerability-free, or perfect-crypto claim is made.
- Confirm cargo audit green is described only as dependency-health evidence.
- Confirm no README, START_HERE, public docs, website, or public technical
  paper path is changed.

## Validation commands

Required local validation:

- `git diff --check`
- exact allowed-path scope guard
- link check
- leak scan
- overclaim/claim-boundary scan
- classifier or queue/decision helper
- PR body preflight
- goal-lint
- `cargo audit --deny warnings`
- `cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock`
- `cargo tree -i rustls-webpki --locked`
- `cargo tree -i ml-kem --locked`
- pqcrypto inverse tree absence probes
- `cargo fmt --check`
- `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1`
- `cargo test -p quantumshield_refimpl --features pqcrypto --locked --test pqkem768`
- `python3 formal/model_qsc_handshake_suite_id_bounded.py`
- `python3 formal/run_model_checks.py`
- `scripts/ci/qsc_adversarial.sh` if locally feasible, with any missing
  cargo-fuzz tooling recorded and PR CI used as authoritative smoke proof

Required before PR merge:

- READY_COUNT 1.
- READY NA-0435.
- D-0857 exists once.
- D-0858 absent.
- no duplicate decision IDs.
- only the five allowed NA-0435 paths changed.
- no runtime/crypto/dependency/Cargo/lockfile/workflow/test/vector mutation.
- root and nested cargo audits are green.
- qsl-backup SHA is unchanged.
- no backup or restore was run.
- public-safety and required PR checks are green.

## Optional closeout readiness

Optional closeout to NA-0436 is allowed only after the NA-0435 PR merges and
post-merge public-safety is green. Closeout must mark NA-0435 DONE, keep
NA-0434 BLOCKED history unchanged, add D-0858, and restore exactly one READY
successor:

`NA-0436 -- QSL qsc pq_decap_failed No-Mutation Test Implementation Harness`
