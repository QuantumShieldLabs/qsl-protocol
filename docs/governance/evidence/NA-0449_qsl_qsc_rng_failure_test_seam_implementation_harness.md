Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-09

# NA-0449 QSL qsc RNG Failure Test Seam Implementation Harness

Goals: G1, G2, G3, G4, G5

## Executive summary

NA-0449 consumes D-0883 / NA-0448 and implements the exact qsc RNG failure
test-only seam authorized for selected qsc surfaces. The implementation uses
the custom cfg `qsc_rng_failure_test_seam` and the seam selector
`QSC_RNG_FAILURE_TEST_SEAM` only inside cfg-gated code.

Normal builds without `--cfg qsc_rng_failure_test_seam` do not read the seam
selector and keep the existing production `OsRng` behavior. The normal-build
test proves the selector is inert without the custom cfg.

Implemented evidence:

- handshake session ID RNG failure aborts before pending/session state write;
- vault init RNG failure aborts before vault file or temp vault file write;
- session-store RNG failure aborts before session blob or session-store key
  secret write;
- no dependency, Cargo manifest, lockfile, or workflow mutation occurred;
- no broad RNG-failure-complete claim is made.

Selected successor for closeout:

`NA-0450 -- QSL qsc RNG Failure Residual Surface Triage Authorization Plan`

## Live NA-0449 scope

Live `NEXT_ACTIONS.md` lists:

`NA-0449 -- QSL qsc RNG Failure Test Seam Implementation Harness`

Status: READY.

Allowed implementation mutation paths:

- `qsl/qsl-client/qsc/tests/rng_failure_behavior.rs`
- `qsl/qsl-client/qsc/src/handshake/mod.rs`
- `qsl/qsl-client/qsc/src/protocol_state/mod.rs`
- `qsl/qsl-client/qsc/src/vault/mod.rs`

Allowed governance mutation paths:

- `docs/governance/evidence/NA-0449_qsl_qsc_rng_failure_test_seam_implementation_harness.md`
- `tests/NA-0449_qsl_qsc_rng_failure_test_seam_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Forbidden mutation scope includes dependency metadata, Cargo manifests,
lockfiles, workflows, executable tests outside the exact qsc test path, fuzz
targets, vectors, formal models, qsl-server, qsl-attachments, qshield runtime,
qshield-cli, website, public docs, README, START_HERE, qwork/qstart/qresume,
qsl-backup, backup status files, backup plan files, rollback subtree paths,
`/backup/qsl`, public technical paper content, branch protection, and public
claim surfaces.

## qwork proof-file verification

Codex read, but did not run, qwork proof files:

- `/srv/qbuild/work/NA-0449/.qwork/startup.qsl-protocol.kv`
- `/srv/qbuild/work/NA-0449/.qwork/startup.qsl-protocol.json`

Required `.kv` markers passed:

- `startup_result=OK`
- `lane=NA-0449`
- `repo=qsl-protocol`
- `path=/srv/qbuild/work/NA-0449/qsl-protocol`
- `head_equals_origin_main=yes`
- `worktree_clean=yes`
- `index_clean=yes`
- `untracked_clean=yes`
- `ready_count=1`
- `queue_top_ready=NA-0449`
- `requested_lane_status=READY`

The JSON proof parsed successfully and mirrored the `.kv` proof for lane, repo,
path, HEAD, origin/main, clean-state fields, READY count, top READY item, and
requested lane status.

After `git fetch --all --prune`, live `HEAD` and `origin/main` still matched
the qwork proof at `e4eb7f4fcf6`. PR #1166 was verified MERGED with merge
commit `e4eb7f4fcf6`.

Recorded timestamps:

- Local: `2026-06-08T23:05:58-05:00`
- UTC: `2026-06-09T04:05:58+00:00`

Proof root:

`/srv/qbuild/tmp/NA0449_qsc_rng_failure_test_seam_impl_20260609T040744Z`

## NA-0448 authorization inheritance

NA-0448 selected:

`QSC_RNG_TEST_SEAM_IMPLEMENTATION_READY`

D-0883 authorized only these qsc implementation paths:

- `qsl/qsl-client/qsc/tests/rng_failure_behavior.rs`
- `qsl/qsl-client/qsc/src/handshake/mod.rs`
- `qsl/qsl-client/qsc/src/protocol_state/mod.rs`
- `qsl/qsl-client/qsc/src/vault/mod.rs`

Required inherited strategy:

- use custom cfg `qsc_rng_failure_test_seam`;
- activate the seam only with the explicit future test command;
- use an environment selector only inside cfg-gated code;
- do not add fallback random bytes;
- do not add deterministic replacement bytes;
- do not add fallback-to-success behavior;
- fail closed before selected pending/session/vault mutation;
- keep ordinary qsc tests green without the custom cfg;
- do not mutate Cargo manifests, lockfiles, dependencies, or workflows.

Residuals inherited from NA-0448 remain out of scope here:

- route/contact/attachment RNG;
- provider-dependent key and crypto generation;
- refimpl/provider RNG failure;
- qshield-cli demo RNG boundaries;
- formal/model RNG behavior;
- fuzz and vector RNG failure behavior.

## Pre-mutation review and rollback setup

Preimage SHA256 values were recorded:

- `qsl/qsl-client/qsc/src/handshake/mod.rs`: `8517723afa16026a040d01e2346e5580095ec8b1aeb99bc941cc1144cdca3d41`
- `qsl/qsl-client/qsc/src/protocol_state/mod.rs`: `6f69b20722a1de2e3737c8a4caad3274359015d29e460f6fc96b728014b46c59`
- `qsl/qsl-client/qsc/src/vault/mod.rs`: `7c7aa20574c609f919ce928fa1836005107d742262de12e32fd43356f2c165d6`

The three source paths existed with mode 664 and owner `victor:victor`.
`qsl/qsl-client/qsc/tests/rng_failure_behavior.rs` was absent before patching,
so an absent marker was written under the proof root rollback directory.

Preimage copies were stored under:

`/srv/qbuild/tmp/NA0449_qsc_rng_failure_test_seam_impl_20260609T040744Z/rollback/`

Initial `git diff --name-only` before mutation was empty.

## Test-only RNG failure seam implementation

The implementation adds cfg-only helpers in the exact selected source files:

- `handshake/mod.rs`: `QSC.HS.SID` can force session ID generation failure
  before `hs_pending_store`.
- `protocol_state/mod.rs`: `QSC.QSP.SESSION_STORE_KEY` and
  `QSC.QSP.SESSION_BLOB_NONCE` can force session-store key/blob nonce failure
  before session blob write.
- `vault/mod.rs`: selected vault salt/nonce/default-token and existing vault
  write nonce labels can force failure before vault writes.

The seam is active only under:

```bash
RUSTFLAGS='--cfg qsc_rng_failure_test_seam' cargo test -p qsc --locked --test rng_failure_behavior -- --test-threads=1 --nocapture
```

Normal builds compile the non-cfg branches and keep direct `OsRng` use.

Required implementation markers emitted by the cfg test:

- `NA0449_RNG_FAILURE_AUTHORIZATION_CONSUMED_OK`
- `NA0449_RNG_FAILURE_TEST_SEAM_IMPLEMENTED_OK`
- `NA0449_RNG_FAILURE_FORCED_BY_TEST_ONLY_SEAM_OK`
- `NA0449_PRODUCTION_SEMANTICS_UNCHANGED_OK`
- `NA0449_QSC_HANDSHAKE_SESSION_ID_RNG_FAILURE_NO_PENDING_OK`
- `NA0449_QSC_VAULT_RNG_FAILURE_NO_VAULT_WRITE_OK`
- `NA0449_QSC_SESSION_STORE_RNG_FAILURE_NO_SESSION_WRITE_OK`
- `NA0449_NO_DEPENDENCY_CHANGE_OK`
- `NA0449_NO_WORKFLOW_CHANGE_OK`
- `NA0449_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0449_NO_CRYPTO_COMPLETE_CLAIM_OK`
- `NA0449_NO_RNG_FAILURE_COMPLETE_CLAIM_OK`
- `NA0449_STEWARD_REVIEW_TEMPLATE_USED_OK`
- `NA0449_ONE_READY_INVARIANT_OK`

## Handshake session ID RNG failure proof

Test:

`handshake_session_id_rng_failure_writes_no_pending_state`

Forced label:

`QSC.HS.SID`

Proof:

- command exits failure with `rng_failure_forced`;
- Alice vault bytes are unchanged after the forced failure;
- no `qsp_sessions/bob.qsv` session blob exists;
- relay channel for Bob remains empty;
- marker emitted:
  `NA0449_QSC_HANDSHAKE_SESSION_ID_RNG_FAILURE_NO_PENDING_OK`.

## Vault RNG failure proof

Test:

`vault_rng_failure_writes_no_vault_file`

Forced label:

`QSC.VAULT.INIT.SALT`

Proof:

- command exits failure with `rng_failure_forced`;
- `vault.qsv` is absent;
- `vault.qsv.tmp` is absent;
- marker emitted: `NA0449_QSC_VAULT_RNG_FAILURE_NO_VAULT_WRITE_OK`.

This is bounded vault RNG evidence for the selected salt/init path. It is not
all vault RNG failure coverage.

## Session/protocol-state RNG failure proof

Test:

`session_store_rng_failure_writes_no_session_blob`

Forced label:

`QSC.QSP.SESSION_STORE_KEY`

Proof:

- A1 and B1 are generated normally before the forced session-store step;
- Alice poll exits failure with `handshake_session_store_failed`;
- no `qsp_sessions/bob.qsv` session blob exists;
- `qsp_session_store_key_v1` remains absent from Alice's vault;
- Alice pending state remains unchanged;
- Alice vault bytes are unchanged;
- relay channel for Bob remains empty, so A2 was not emitted;
- marker emitted:
  `NA0449_QSC_SESSION_STORE_RNG_FAILURE_NO_SESSION_WRITE_OK`.

## Production semantics unchanged proof

Normal command:

```bash
cargo test -p qsc --locked --test rng_failure_behavior -- --test-threads=1 --nocapture
```

Result:

- PASS, 1 test.
- The test sets `QSC_RNG_FAILURE_TEST_SEAM=QSC.VAULT.INIT.SALT` without the
  custom cfg and vault init succeeds.
- Marker emitted: `NA0449_PRODUCTION_SEMANTICS_UNCHANGED_OK`.

Additional normal qsc validation:

- `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1`: PASS, 3 tests.
- `cargo test -p qsc --locked --test key_lifecycle_zeroization -- --test-threads=1 --nocapture`: PASS, 6 tests.
- `cargo test -p qsc --locked --test handshake_provider_error_no_mutation -- --test-threads=1 --nocapture`: PASS, 1 test.

## No dependency / no workflow mutation proof

No Cargo manifest was changed. No lockfile was changed. No dependency command
such as `cargo update` or `cargo generate-lockfile` was run. No workflow file
was changed.

Dependency validation:

- `cargo audit --deny warnings`: PASS.
- `cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock`: PASS.
- `cargo tree -i rustls-webpki --locked`: PASS, `rustls-webpki v0.103.13`.
- `cargo tree -i ml-kem --locked`: PASS, `ml-kem v0.2.1`.
- root pqcrypto inverse probes for `pqcrypto-mlkem`, `pqcrypto-traits`, and
  `pqcrypto-internals` returned expected package-ID absence.
- nested qsc fuzz lock pqcrypto residual scan returned zero matches.

## No RNG-failure-complete claim proof

No RNG-failure-complete claim is made. NA-0449 is bounded internal qsc evidence
only for selected handshake, vault, and session-store paths.

No crypto-complete claim is made.
No side-channel-free claim is made.
No vulnerability-free claim is made.
No bug-free claim is made.
No perfect-crypto claim is made.
No public-readiness claim is made.
No production-readiness claim is made.
No public-internet-readiness claim is made.
No external-review-complete claim is made.
No public technical paper content is created.

Cargo audit green remains dependency-health evidence only.

## qshield-cli demo-local boundary proof

qshield-cli remains demo-local and out of scope for NA-0449. No qshield-cli
path changed. No qshield runtime path changed. No qshield-cli RNG failure
claim is made.

## refimpl/provider RNG backlog proof

refimpl/provider RNG failure remains backlog. NA-0449 does not change
`tools/refimpl/**` and does not alter provider traits or key-generation
contracts.

Validation:

- `cargo test -p quantumshield_refimpl --features pqcrypto --locked --test pqkem768`: PASS, 3 tests.

## Validation

Focused implementation validation:

- cfg seam test command: PASS, 4 tests.
- normal seam test command: PASS, 1 test.
- `cargo fmt --check`: PASS after formatting correction.
- `git diff --cached --check`: PASS.
- exact staged path scope guard: PASS, 9 changed paths and 0 forbidden paths.
- `python3 scripts/ci/qsl_evidence_helper.py link-check`: PASS, 0 missing links.
- `python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main`: PASS, 0 findings.
- staged added-line overclaim scan: PASS, 0 affirmative findings after same-line caveat wording cleanup.
- `python3 scripts/ci/qsl_evidence_helper.py pr-body-preflight`: PASS for the local PR body draft.
- `bash scripts/ci/classify_ci_scope.sh ...`: PASS, `runtime_critical`.
- queue helper after D-0885 patch: READY_COUNT 1, READY NA-0449.
- decisions helper after D-0885 patch: latest D-0885, duplicate decision count 0.

Broader validation completed before final governance scan:

- `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1`: PASS.
- `cargo test -p qsc --locked --test key_lifecycle_zeroization -- --test-threads=1 --nocapture`: PASS.
- `cargo test -p qsc --locked --test handshake_provider_error_no_mutation -- --test-threads=1 --nocapture`: PASS.
- `cargo test -p quantumshield_refimpl --features pqcrypto --locked --test pqkem768`: PASS.
- `sh -n scripts/ci/qsc_adversarial.sh`: PASS.
- `bash -n scripts/ci/qsc_adversarial.sh`: PASS.
- `cargo audit --deny warnings`: PASS.
- nested qsc fuzz lock audit: PASS.
- dependency tree probes: PASS / expected absence as above.
- `python3 formal/model_qsc_handshake_suite_id_bounded.py`: PASS.
- `python3 formal/run_model_checks.py`: PASS.
- local qsc adversarial script: stable adversarial properties, miri-shaped
  tests, and provider-error no-mutation step passed; local script then stopped
  at unavailable `cargo fuzz`. PR CI qsc-adversarial-smoke remains required for
  cargo-fuzz-backed evidence.
- PR #1167 first CI run reported CodeQL hard-coded cryptographic value alerts
  in the allowed `vault/mod.rs` seam changes. The corrective commit restored
  `ChaCha20Poly1305::generate_nonce` for selected vault nonce paths and moved
  the cfg-only failure check into a fail-before-generate helper. Post-fix local
  `cargo fmt --check`, cfg seam test, normal seam test, key lifecycle test, and
  provider-error no-mutation test passed.

## Public claim/external review/website boundary

No README, START_HERE, public-doc, or website path changed.

No public-readiness claim is made.
No production-readiness claim is made.
No public-internet-readiness claim is made.
No external-review-complete claim is made.
No crypto-complete claim is made.
No side-channel-free claim is made.
No RNG-failure-complete claim is made.
No bug-free claim is made.
No vulnerability-free claim is made.
No perfect-crypto claim is made.

NA-0449 tests are internal bounded evidence only.

## Rejected alternatives

Cargo feature:

Rejected because D-0883 required a custom cfg seam and no Cargo feature,
manifest, lockfile, dependency, or workflow mutation.

Runtime RNG trait extraction:

Rejected because it would broaden provider/runtime contracts beyond the exact
D-0883 source paths and would risk production semantic drift.

Deterministic fallback bytes:

Rejected because the seam must fail closed and must not silently replace failed
randomness with deterministic or fallback random bytes.

Broad qsc RNG surface expansion:

Rejected for NA-0449 because route/contact/attachment/provider/refimpl/formal,
fuzz, and vector RNG surfaces remain future-gated residuals.

## Backup-impact statement

No backup was run. No restore was run. No sudo was run. qsl-backup was not
mutated. Backup status and backup plan files were not mutated. Rollback
subtree paths and `/backup/qsl` were not mutated.

Read-only boundary checks:

- qsl-backup SHA matched the required value.
- qsl-backup source-list inclusion count for Codex ops was exactly 1.

This remains same-host boundary evidence only. No off-host backup completion
claim is made. No disaster recovery claim is made. No restore proof claim is
made. No backup-complete claim is made.

## Successor selection

Default successor selected:

`NA-0450 -- QSL qsc RNG Failure Residual Surface Triage Authorization Plan`

Rationale:

NA-0449 implemented the first bounded qsc seam. The next safest step is to
triage residual qsc RNG surfaces that were explicitly not selected:

- route/contact/attachment RNG;
- provider-dependent qsc generation boundaries;
- formal/model/fuzz/vector RNG residuals;
- claim boundaries for qsc versus refimpl/qshield-cli evidence.

NA-0450 must not implement runtime, crypto, dependency, Cargo, lockfile,
workflow, public, service, backup, or qsl-backup changes unless a later exact
directive authorizes precise paths.

## Next recommendation

Open the NA-0449 evidence PR after final scope/link/leak/claim/goal
validation. Merge only after required PR checks pass, including public-safety.
After merge and post-merge public-safety success, optionally close out NA-0449
and restore the selected NA-0450 residual-surface triage authorization plan as
the sole READY successor.
