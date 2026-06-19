Status: Supporting
Owner: QSL Director
Last-Updated: 2026-06-19
Replaces: n/a
Superseded-By: n/a

# NA-0498 Closeout and NA-0499 Restoration Testplan

## Purpose

Validate that NA-0498 is closed only after PR #1267 merged and post-merge
public-safety completed success for `5cc59ddf26ba`, and that the selected
NA-0499 side-channel / secret-material lifecycle assurance scope authorization
successor is restored as the sole READY item without implementing NA-0499.

## Allowed closeout paths

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0498_closeout_restore_na0499_testplan.md`

## Preconditions

- qwork proof files exist, report `startup_result=OK`, and match live
  HEAD/origin before fetch.
- `/` usage is below the 95% stop threshold.
- PR #1267 is merged at `5cc59ddf26ba`.
- D376 response exists.
- D-0985 exists exactly once.
- D-0986 is absent before closeout mutation.
- READY count before closeout is exactly one.
- READY before closeout is NA-0498.
- Post-merge public-safety on `5cc59ddf26ba` is success.
- qsc-adversarial-smoke on `5cc59ddf26ba` is success.
- qsc-linux-full-suite on `5cc59ddf26ba` is success or accepted by repository
  policy.
- macos-qsc-full-serial on `5cc59ddf26ba` is success or accepted by repository
  policy.

## Required inheritance proof

- PR #1267 merged at `5cc59ddf26ba`.
- D376 response exists and is consumed.
- D376 stopped because post-merge public-safety had not attached or completed
  green during the short attach/early-failure window; no completed failures
  were observed.
- D-0985 records the NA-0498 core assurance checkpoint evidence.
- NA-0498 consumed NA-0497/D375 inheritance.
- Core evidence inventory was completed.
- Highest-risk residual review was completed.
- Hostile cryptographer, red-team, SRE, side-channel caveat, formal mapping,
  external-review, release-claim, and prioritization reviews were completed.
- Primary classification is `CORE_ASSURANCE_SIDE_CHANNEL_SECRET_MATERIAL_NEXT`.
- Selected successor is `NA-0499 -- QSL Side-Channel / Secret-Material
  Lifecycle Assurance Scope Authorization Plan`.
- NA-0498 remained READY until closeout.

## Required closeout changes

- Mark NA-0498 DONE.
- Add D-0986: `NA-0498 closeout and NA-0499 restoration`.
- Restore `NA-0499 -- QSL Side-Channel / Secret-Material Lifecycle Assurance
  Scope Authorization Plan` as the sole READY successor.
- Update TRACEABILITY for PR #1267 evidence, D376 response, D-0985, PR #1267
  post-merge public-safety proof, D-0986, the closeout PR, and NA-0499
  successor.
- Update the rolling operations journal.
- Add this closeout testplan.

## Forbidden closeout changes

- Do not implement NA-0499.
- Do not mutate qsc source, tests, fuzz target code, fuzz Cargo metadata, or
  lockfiles.
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
- Do not make an external-review-complete claim.
- Do not make a crypto-complete claim.
- Do not make a KEM-complete claim.
- Do not make a signature-complete claim.
- Do not make an identity-complete claim.
- Do not make a provider-RNG-complete claim.
- Do not make a secret-material-complete claim.
- Do not make a side-channel-free claim.
- Do not make a replay-proof claim.
- Do not make a downgrade-proof claim.
- Do not make a vulnerability-free claim.
- Do not make a bug-free claim.
- Do not make a perfect-crypto claim.

## Validation commands

Run before closeout mutation:

```bash
git diff --check
python3 scripts/audit/validate_binding_fuzz_corpus_no_secrets.py --format json --path qsl/qsl-client/qsc/fuzz/corpus/qsc_binding_semantics
python3 scripts/audit/validate_binding_fuzz_corpus_no_secrets.py --format json --path qsl/qsl-client/qsc/fuzz/corpus
cargo test -p qsc --locked --test binding_negative_vector_consumer -- --test-threads=1 --nocapture
python3 -m json.tool inputs/suite2/internal_negative_binding_vectors/qsl_binding_negative_vector_manifest_v1.json >/dev/null
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
cargo test -p qsc --locked --test binding_negative_vector_consumer -- --test-threads=1 --nocapture
python3 -m json.tool inputs/suite2/internal_negative_binding_vectors/qsl_binding_negative_vector_manifest_v1.json >/dev/null
python3 formal/run_model_checks.py
cargo audit --deny warnings
cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock
cargo fmt --check
sh -n scripts/ci/qsc_adversarial.sh
bash -n scripts/ci/qsc_adversarial.sh
```

## Acceptance

NA-0498 is accepted as DONE only when PR #1267 merge proof, D376 inheritance,
D-0985, PR #1267 post-merge public-safety success, qsc-adversarial-smoke
success, qsc-linux-full-suite success or policy acceptance,
macos-qsc-full-serial success or policy acceptance, validator proof, local
validation, exact five-path closeout scope, and one-READY queue proof all pass.
NA-0499 is accepted as READY only as the selected side-channel /
secret-material lifecycle assurance scope authorization successor; no NA-0499
execution is performed by this closeout.
