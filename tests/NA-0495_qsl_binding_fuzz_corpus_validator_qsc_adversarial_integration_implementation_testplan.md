Status: Supporting
Owner: QSL Director
Last-Updated: 2026-06-18

# NA-0495 QSL Binding Fuzz Corpus Validator qsc-Adversarial Integration Implementation Testplan

## Objective

Validate that NA-0495 integrates the existing binding fuzz corpus
secret-material validator into `scripts/ci/qsc_adversarial.sh` while preserving
fail-closed behavior and the directive's no-workflow/no-dependency boundary.

## Protected invariants

- The validator script is used unchanged.
- The binding corpus is scanned and is not allow-missing.
- The full qsc fuzz corpus is scanned.
- Validator findings fail the qsc-adversarial script before cargo-fuzz targets.
- Existing qsc-adversarial stable tests, provider-error no-mutation test, and
  cargo-fuzz target list are preserved.
- No public/completion/proof/freedom claim is expanded.
- Exactly one READY item remains until optional closeout.

## Allowed scope

- `scripts/ci/qsc_adversarial.sh`
- `docs/governance/evidence/NA-0495_qsl_binding_fuzz_corpus_validator_qsc_adversarial_integration_implementation_harness.md`
- `tests/NA-0495_qsl_binding_fuzz_corpus_validator_qsc_adversarial_integration_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Forbidden scope

No workflow, helper, validator script, corpus/vector/input, qsc source, qsc fuzz
target, qsc fuzz Cargo, qsc fuzz lockfile, root lockfile, dependency, formal,
refimpl, service, public-doc, website, README, START_HERE, backup,
qsl-backup, qwork, qstart, qresume, qshell, archive, move, or delete mutation is
allowed.

## Script integration validation

Run:

```bash
sh -n scripts/ci/qsc_adversarial.sh
bash -n scripts/ci/qsc_adversarial.sh
```

Static proof must show:

- `scripts/audit/validate_binding_fuzz_corpus_no_secrets.py` is invoked.
- `qsl/qsl-client/qsc/fuzz/corpus/qsc_binding_semantics` is scanned.
- `qsl/qsl-client/qsc/fuzz/corpus` is scanned.
- `--allow-missing` is absent.
- The validator step runs before the first `run_fuzz_target`.
- All required NA-0495 markers are present.

## Binding corpus validator scan

Run:

```bash
python3 scripts/audit/validate_binding_fuzz_corpus_no_secrets.py --format text --path qsl/qsl-client/qsc/fuzz/corpus/qsc_binding_semantics
python3 scripts/audit/validate_binding_fuzz_corpus_no_secrets.py --format json --path qsl/qsl-client/qsc/fuzz/corpus/qsc_binding_semantics
```

Expected: exit 0, zero findings, 7 files scanned, 56 bytes scanned.

## All corpus validator scan

Run:

```bash
python3 scripts/audit/validate_binding_fuzz_corpus_no_secrets.py --format text --path qsl/qsl-client/qsc/fuzz/corpus
python3 scripts/audit/validate_binding_fuzz_corpus_no_secrets.py --format json --path qsl/qsl-client/qsc/fuzz/corpus
```

Expected: exit 0, zero findings, 17 files scanned, 1238 bytes scanned.

## Fail-closed validator proof

Create a proof-root-only fixture with a configured disallowed marker. Run the
validator directly against that fixture.

Expected: nonzero exit and a redacted finding. Do not mutate repo corpus files.
Record that qsc-adversarial uses the same validator command under `set -eu`.

## Missing corpus no-allow proof

Do not remove the repo corpus. Prove statically that qsc-adversarial does not
use `--allow-missing`. Optionally run the validator directly against a
proof-root missing path without `--allow-missing`.

Expected: nonzero exit with a redacted `missing_path` finding.

## qsc-adversarial local/CI proof

Run if feasible:

```bash
sh scripts/ci/qsc_adversarial.sh
```

Expected local result: stable Rust adversarial checks pass, provider-error
no-mutation check passes, validator step passes and prints markers before
cargo-fuzz. If local `cargo fuzz` is unavailable, record the exact output and
require PR qsc-adversarial-smoke to be green before merge.

## Inherited qsc/refimpl/formal tests

Run:

```bash
python3 -m json.tool inputs/suite2/internal_negative_binding_vectors/qsl_binding_negative_vector_manifest_v1.json >/dev/null
python3 formal/model_qsc_kem_signature_transcript_binding_bounded.py
python3 formal/run_model_checks.py
cargo test -p qsc --locked --test kem_signature_transcript_binding_negative -- --test-threads=1 --nocapture
RUSTFLAGS='--cfg qsc_binding_fuzz_helper' cargo test -p qsc --locked --test kem_signature_transcript_binding_negative -- --test-threads=1 --nocapture
cargo test -p quantumshield_refimpl --features pqcrypto --locked --test signature_provider_boundary -- --test-threads=1 --nocapture
cargo test -p quantumshield_refimpl --features pqcrypto --locked --test pqkem768
```

## Audit/fmt checks

Run:

```bash
git diff --check
cargo audit --deny warnings
cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock
cargo fmt --check
python3 scripts/ci/qsl_evidence_helper.py link-check --root .
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
python3 scripts/ci/qsl_evidence_helper.py pr-body-preflight --file "$PROOF_DIR/pr/pr_body.md"
bash scripts/ci/classify_ci_scope.sh
```

Also run exact changed-path scope guard, added-line overclaim scan, and
goal-lint after PR creation.

## Public claim boundary

No public-readiness claim is allowed. no production-readiness claim is allowed.
no public-internet-readiness claim is allowed. no external-review-complete
claim is allowed. no crypto-complete claim is allowed. no fuzz-complete claim
is allowed. no corpus-complete claim is allowed. no vector-complete claim is
allowed. no replay-proof claim is allowed. no downgrade-proof claim is allowed.
no side-channel-free claim is allowed. no vulnerability-free claim is allowed.
no bug-free claim is allowed. no perfect-crypto claim is allowed.

## Closeout prerequisites

Before optional closeout:

- NA-0495 implementation PR is merged with merge commit.
- public-safety is green on the merge commit.
- qsc-adversarial-smoke is green on the merge commit.
- D-0979 exists once on main.
- D-0980 is absent before closeout.
- Exactly one READY remains before closeout mutation.
- Closeout restores a single approved NA-0496 READY successor and does not
  implement NA-0496.
