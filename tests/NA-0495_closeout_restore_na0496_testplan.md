Status: Supporting
Owner: QSL Director
Last-Updated: 2026-06-18
Replaces: n/a
Superseded-By: n/a

# NA-0495 Closeout and NA-0496 Restoration Testplan

## Purpose

Validate that NA-0495 is closed only after PR #1261 merged and post-merge
public-safety completed success for `399fe6c2f61b`, and that the selected
NA-0496 successor is restored as the sole READY item without implementing
NA-0496.

## Allowed closeout paths

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0495_closeout_restore_na0496_testplan.md`

## Preconditions

- qwork proof files exist, report `startup_result=OK`, and match live
  HEAD/origin before fetch.
- PR #1261 is merged at `399fe6c2f61b`.
- D366 response exists at
  `/home/victor/work/qsl/codex/responses/NA0495_20260618T054854Z_D366.md`.
- D-0979 exists exactly once.
- D-0980 is absent before closeout mutation.
- READY count before closeout is exactly one.
- READY before closeout is NA-0495.
- Post-merge public-safety on `399fe6c2f61b` is green.
- qsc-adversarial-smoke on `399fe6c2f61b` is green.

## Required Inheritance Proof

- PR #1261 merged at `399fe6c2f61b`.
- qsc-adversarial integrates the existing validator.
- The binding corpus scan is present.
- The full qsc fuzz corpus scan is present.
- No `--allow-missing` is used for the binding corpus.
- Validator findings fail closed under `set -eu`.
- Existing stable adversarial tests, provider-error no-mutation, target order,
  and qsc binding helper cfg behavior are preserved.
- D366 stopped only because post-merge public-safety/full-suite checks were
  still in progress after bounded polling.
- qsc-adversarial-smoke was green at D366 stop.
- NA-0495 remained READY until closeout.

## Required Closeout Changes

- Mark NA-0495 DONE.
- Add D-0980: `NA-0495 closeout and NA-0496 restoration`.
- Restore `NA-0496 -- QSL Binding Negative Vector Consumer Test Scope
  Authorization Plan` as the sole READY successor.
- Update TRACEABILITY for PR #1261 implementation, D366 response, D-0979,
  PR #1261 post-merge public-safety proof, D-0980, closeout PR, and NA-0496
  successor.
- Update the rolling operations journal.
- Add this closeout testplan.

## Forbidden Closeout Changes

- Do not implement NA-0496.
- Do not mutate `scripts/ci/qsc_adversarial.sh`.
- Do not mutate workflows.
- Do not mutate helpers.
- Do not mutate validator scripts.
- Do not mutate corpus, vector, input, qsc source, qsc fuzz target, Cargo,
  lockfile, dependency, formal, refimpl, service, public, backup, qsl-backup,
  qwork, qstart, qresume, or qshell paths.
- Do not move, archive, or delete files.
- Do not implement CI watcher/tooling.
- Do not make a public-readiness claim.
- Do not make a production-readiness claim.
- Do not make a public-internet-readiness claim.
- Do not make an external-review-complete claim.
- Do not make a crypto-complete claim.
- Do not make a fuzz-complete claim.
- Do not make a corpus-complete claim.
- Do not make a validator-complete claim beyond bounded NA-0495 implementation
  evidence.
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
python3 formal/run_model_checks.py
cargo audit --deny warnings
cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock
cargo fmt --check
sh -n scripts/ci/qsc_adversarial.sh
bash -n scripts/ci/qsc_adversarial.sh
```

## Acceptance

NA-0495 is accepted as DONE only when PR #1261 merge proof, D366 inheritance,
D-0979, PR #1261 post-merge public-safety success, qsc-adversarial-smoke
success, validator proof, local validation, exact five-path closeout scope, and
one-READY queue proof all pass. NA-0496 is accepted as READY only as a scope
authorization plan; no NA-0496 implementation work is performed by this
closeout.
