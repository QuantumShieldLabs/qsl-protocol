Status: Supporting
Owner: QSL Director
Last-Updated: 2026-06-18
Replaces: n/a
Superseded-By: n/a

# NA-0497 Closeout and NA-0498 Restoration Testplan

## Purpose

Validate that NA-0497 is closed only after PR #1265 merged and post-merge
public-safety completed success for `28afc4cc8085`, and that the selected
NA-0498 core assurance checkpoint successor is restored as the sole READY item
without implementing NA-0498.

## Allowed closeout paths

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0497_closeout_restore_na0498_testplan.md`

## Preconditions

- qwork proof files exist, report `startup_result=OK`, and match live
  HEAD/origin before fetch.
- `/` usage is below the 95% stop threshold.
- PR #1265 is merged at `28afc4cc8085`.
- D373 and D374 response files exist.
- D-0983 exists exactly once.
- D-0984 is absent before closeout mutation.
- READY count before closeout is exactly one.
- READY before closeout is NA-0497.
- Post-merge public-safety on `28afc4cc8085` is success.
- qsc-adversarial-smoke on `28afc4cc8085` is success.
- qsc-linux-full-suite on `28afc4cc8085` is success or accepted by repository
  policy.
- macos-qsc-full-serial on `28afc4cc8085` is success or accepted by repository
  policy.

## Required Inheritance Proof

- PR #1265 merged at `28afc4cc8085`.
- D373 response exists and is consumed.
- D374 response exists and is consumed.
- D373 stopped because post-merge public-safety had not attached or completed
  green during the short window; no red result was observed.
- D374 stopped because the bounded closeout poll reached cap while
  public-safety and qsc-linux-full-suite were still in progress; no completed
  failures were observed.
- D-0983 records the NA-0497 implementation evidence.
- `qsl/qsl-client/qsc/tests/binding_negative_vector_consumer.rs` is present.
- The qsc integration test parses the internal manifest and validates schema,
  counts, unique IDs, layer/group coverage, qsc-frame mappings, refimpl
  supporting-only boundaries, formal-token supporting-only boundaries,
  no-secret policy metadata, and public/conformance/completion/proof overclaim
  caveats.
- The implementation makes no dynamic crypto execution claim.
- NA-0497 remained READY until closeout.

## Required Closeout Changes

- Mark NA-0497 DONE.
- Add D-0984: `NA-0497 closeout and NA-0498 restoration`.
- Restore `NA-0498 -- QSL Core Assurance Checkpoint and Next Highest-Risk
  Security Lane Authorization Plan` as the sole READY successor.
- Update TRACEABILITY for PR #1265 implementation/evidence, D373/D374
  responses, D-0983, PR #1265 post-merge public-safety proof, D-0984, closeout
  PR, and NA-0498 successor.
- Update the rolling operations journal.
- Add this closeout testplan.

## Forbidden Closeout Changes

- Do not implement NA-0498.
- Do not mutate qsc integration tests.
- Do not mutate qsc source.
- Do not mutate qsc fuzz target code.
- Do not mutate qsc fuzz Cargo metadata or lockfiles.
- Do not mutate workflows.
- Do not mutate scripts or helpers.
- Do not mutate validator scripts.
- Do not mutate corpus, vector, input, internal manifest, dependency, lockfile,
  formal, refimpl, service, public, backup, qsl-backup, qwork, qstart, qresume,
  or qshell paths.
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
cargo test -p qsc --locked --test binding_negative_vector_consumer -- --test-threads=1 --nocapture
python3 -m json.tool inputs/suite2/internal_negative_binding_vectors/qsl_binding_negative_vector_manifest_v1.json >/dev/null
python3 scripts/audit/validate_binding_fuzz_corpus_no_secrets.py --format json --path qsl/qsl-client/qsc/fuzz/corpus/qsc_binding_semantics
python3 scripts/audit/validate_binding_fuzz_corpus_no_secrets.py --format json --path qsl/qsl-client/qsc/fuzz/corpus
python3 formal/run_model_checks.py
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

NA-0497 is accepted as DONE only when PR #1265 merge proof, D373/D374
inheritance, D-0983, PR #1265 post-merge public-safety success,
qsc-adversarial-smoke success, qsc-linux-full-suite success,
macos-qsc-full-serial success, validator proof, local validation, exact
five-path closeout scope, and one-READY queue proof all pass. NA-0498 is
accepted as READY only as the selected core assurance checkpoint; no NA-0498
execution is performed by this closeout.
