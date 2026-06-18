Status: Supporting
Owner: QSL Director
Last-Updated: 2026-06-18
Replaces: n/a
Superseded-By: n/a

# NA-0496 Closeout and NA-0497 Restoration Testplan

## Purpose

Validate that NA-0496 is closed only after PR #1263 merged and post-merge
public-safety completed success for `ba839e2d8f33`, and that the selected
NA-0497 successor is restored as the sole READY item without implementing
NA-0497.

## Allowed closeout paths

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0496_closeout_restore_na0497_testplan.md`

## Preconditions

- qwork proof files exist, report `startup_result=OK`, and match live
  HEAD/origin before fetch.
- `/` usage is below the 95% stop threshold.
- PR #1263 is merged at `ba839e2d8f33`.
- D368, D369, D370, and D371 response files exist.
- D-0981 exists exactly once.
- D-0982 is absent before closeout mutation.
- READY count before closeout is exactly one.
- READY before closeout is NA-0496.
- Post-merge public-safety on `ba839e2d8f33` is success.
- qsc-adversarial-smoke on `ba839e2d8f33` is success.
- qsc-linux-full-suite and macos-qsc-full-serial are success or accepted by
  repository policy for docs/governance-only scope.

## Required Inheritance Proof

- PR #1263 merged at `ba839e2d8f33`.
- D368 selected `BINDING_NEGATIVE_VECTOR_SCHEMA_MAPPING_TEST_READY`.
- D368 selected successor NA-0497.
- D368 selected future test path
  `qsl/qsl-client/qsc/tests/binding_negative_vector_consumer.rs`.
- Future semantics are manifest parsing plus schema/category/layer/mapping
  coverage.
- Future semantics make no dynamic crypto execution claim.
- qsc-frame vectors remain the qsc-facing subset.
- refimpl and formal-token sections remain supporting-only.
- D368 made no implementation mutation.
- D369 stopped at disk 95%.
- D370 reduced disk to 89%.
- D371 reduced disk to 74.92% exact and 74% by `df`.
- D371 archived old completed worktrees only and preserved active NA-0496,
  qwork proof files, and response files.
- NA-0496 remained READY until closeout.

## Required Closeout Changes

- Mark NA-0496 DONE.
- Add D-0982: `NA-0496 closeout and NA-0497 restoration`.
- Restore `NA-0497 -- QSL Binding Negative Vector Consumer Test
  Implementation Harness` as the sole READY successor.
- Update TRACEABILITY for PR #1263 evidence, D368 response, D-0981, PR #1263
  post-merge public-safety proof, D-0982, closeout PR, and NA-0497 successor.
- Update the rolling operations journal.
- Add this closeout testplan.

## Forbidden Closeout Changes

- Do not implement NA-0497.
- Do not mutate qsc integration tests.
- Do not mutate qsc source.
- Do not mutate qsc fuzz target code.
- Do not mutate qsc fuzz Cargo metadata or lockfiles.
- Do not mutate workflows.
- Do not mutate scripts or helpers.
- Do not mutate validator scripts.
- Do not mutate corpus, vector, input, dependency, lockfile, formal, refimpl,
  service, public, backup, qsl-backup, qwork, qstart, qresume, or qshell paths.
- Do not move, archive, or delete files.
- Do not make a public-readiness claim.
- Do not make a production-readiness claim.
- Do not make a public-internet-readiness claim.
- Do not make a public/conformance vector claim.
- Do not make an external-review-complete claim.
- Do not make a crypto-complete claim.
- Do not make a fuzz-complete claim.
- Do not make a corpus-complete claim.
- Do not make a vector-complete claim.
- Do not make a replay-proof claim.
- Do not make a downgrade-proof claim.
- Do not make a side-channel-free claim.
- Do not make a vulnerability-free claim.
- Do not make a bug-free claim.
- Do not make a perfect-crypto claim.

## Validation Commands

Run before closeout mutation:

```bash
git diff --check
python3 scripts/audit/validate_binding_fuzz_corpus_no_secrets.py --format json --path qsl/qsl-client/qsc/fuzz/corpus/qsc_binding_semantics
python3 scripts/audit/validate_binding_fuzz_corpus_no_secrets.py --format json --path qsl/qsl-client/qsc/fuzz/corpus
python3 -m json.tool inputs/suite2/internal_negative_binding_vectors/qsl_binding_negative_vector_manifest_v1.json >/dev/null
python3 formal/model_qsc_kem_signature_transcript_binding_bounded.py
python3 formal/run_model_checks.py
cargo test -p qsc --locked --test kem_signature_transcript_binding_negative -- --test-threads=1 --nocapture
RUSTFLAGS='--cfg qsc_binding_fuzz_helper' cargo test -p qsc --locked --test kem_signature_transcript_binding_negative -- --test-threads=1 --nocapture
cargo test -p quantumshield_refimpl --features pqcrypto --locked --test signature_provider_boundary -- --test-threads=1 --nocapture
cargo test -p quantumshield_refimpl --features pqcrypto --locked --test pqkem768
cargo audit --deny warnings
cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock
cargo fmt --check
sh -n scripts/ci/qsc_adversarial.sh
bash -n scripts/ci/qsc_adversarial.sh
```

Run before PR:

```bash
git diff --check
python3 scripts/ci/qsl_evidence_helper.py link-check --root .
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
python3 scripts/ci/qsl_evidence_helper.py pr-body-preflight --file "$PROOF_DIR/closeout/pr_body.md"
python3 scripts/audit/validate_binding_fuzz_corpus_no_secrets.py --format json --path qsl/qsl-client/qsc/fuzz/corpus/qsc_binding_semantics
python3 scripts/audit/validate_binding_fuzz_corpus_no_secrets.py --format json --path qsl/qsl-client/qsc/fuzz/corpus
python3 -m json.tool inputs/suite2/internal_negative_binding_vectors/qsl_binding_negative_vector_manifest_v1.json >/dev/null
python3 formal/run_model_checks.py
cargo audit --deny warnings
cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock
cargo fmt --check
sh -n scripts/ci/qsc_adversarial.sh
bash -n scripts/ci/qsc_adversarial.sh
```

## Acceptance

NA-0496 is accepted as DONE only when PR #1263 merge proof, D368/D369/D370/D371
inheritance, D-0981, PR #1263 post-merge public-safety success,
qsc-adversarial-smoke success, repository-policy acceptance for docs/governance
full-suite skips, validator proof, local validation, exact five-path closeout
scope, and one-READY queue proof all pass. NA-0497 is accepted as READY only as
the selected implementation harness; no NA-0497 implementation work is performed
by this closeout.
