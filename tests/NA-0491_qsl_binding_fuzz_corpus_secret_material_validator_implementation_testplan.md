Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-17

# NA-0491 Binding Fuzz Corpus Secret-Material Validator Implementation Testplan

Goals: G1, G2, G3, G4, G5

## Objective

Validate the dependency-free binding fuzz corpus secret-material validator
implementation while preserving no checked-in corpus, no vector/input mutation,
no qsc source/fuzz target/Cargo/script/workflow mutation beyond the new
validator script, no dependency/lockfile mutation, no backup/restore mutation,
and no public-claim expansion.

## Protected invariants

- qwork proof files are read, not regenerated.
- Codex does not run qwork, qstart, or qresume.
- READY_COUNT is exactly 1 at startup.
- READY item is NA-0491 at startup.
- NA-0490, NA-0489, NA-0488, and NA-0487 are DONE.
- D-0969 exists once.
- D-0970 exists once.
- D-0971 is absent before the patch and exists once after the patch.
- duplicate decision count is zero.
- `/` usage remains below 95 percent.
- qsl-backup installed SHA and source inclusion boundary match directive
  expectations.
- no checked-in `qsc_binding_semantics` corpus exists.
- future checked-in binding corpus remains blocked until later exact
  authorization.
- validator output never prints matched secret payload bytes.
- no public-readiness claim, no production-readiness claim,
  no public-internet-readiness claim, no external-review-complete claim,
  no crypto-complete claim, no fuzz-complete claim, no corpus-complete claim,
  no vector-complete claim, no replay-proof claim, no downgrade-proof claim,
  no side-channel-free claim, no vulnerability-free claim, no bug-free claim,
  and no perfect-crypto claim is introduced.

## Allowed scope

- `scripts/audit/validate_binding_fuzz_corpus_no_secrets.py`
- `docs/governance/evidence/NA-0491_qsl_binding_fuzz_corpus_secret_material_validator_implementation_harness.md`
- this testplan
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Forbidden scope

- checked-in corpus mutation
- vector/input mutation
- qsc source mutation
- qsc fuzz target mutation
- qsc fuzz Cargo mutation
- qsc fuzz lockfile mutation
- qsc-adversarial script mutation
- workflow mutation
- dependency or lockfile mutation
- formal/refimpl/service/public/qshield/qsl-server/qsl-attachments mutation
- backup/restore/qsl-backup mutation
- qwork/qstart/qresume/qshell mutation
- file move/archive/delete
- no public-readiness claim and no crypto-complete claim

## Validator CLI validation

Run:

```bash
python3 scripts/audit/validate_binding_fuzz_corpus_no_secrets.py --help
python3 scripts/audit/validate_binding_fuzz_corpus_no_secrets.py --format json --path qsl/qsl-client/qsc/fuzz/corpus
python3 scripts/audit/validate_binding_fuzz_corpus_no_secrets.py --format text --path qsl/qsl-client/qsc/fuzz/corpus
python3 scripts/audit/validate_binding_fuzz_corpus_no_secrets.py --format json --allow-missing --path qsl/qsl-client/qsc/fuzz/corpus/qsc_binding_semantics
```

Required:

- help exits 0.
- JSON and text modes work.
- default and explicit path scanning work.
- current qsc fuzz corpus exits 0.
- missing binding corpus exits 0 only with explicit `--allow-missing`.
- reports are deterministic and redacted.

## Fixture validation

Create fixtures only under the proof root, not in the repo.

Required fixture groups:

- safe synthetic corpus with short byte file, mutated public-message-like bytes,
  and small JSON metadata
- synthetic private-key marker reject fixture
- secret-label reject fixture
- high-entropy encoded text reject fixture

Required:

- safe fixture exits 0.
- private-marker fixture exits nonzero.
- secret-label fixture exits nonzero.
- high-entropy fixture exits nonzero.
- reject outputs include redacted findings only.

## Existing corpus validation

Run:

```bash
python3 scripts/audit/validate_binding_fuzz_corpus_no_secrets.py --format json --path qsl/qsl-client/qsc/fuzz/corpus
```

Required:

- exit 0.
- findings 0.
- files scanned include current checked-in parser/envelope corpus files.
- no `qsc_binding_semantics` corpus is created.

## Redaction validation

Inspect reject JSON outputs.

Required:

- findings include path, kind, severity, offset or line, and context hash.
- findings include `[redacted]`.
- findings do not include matched payload bytes.
- findings do not include high-entropy fixture spans.
- findings do not include synthetic marker fixture payloads.

## Deterministic JSON validation

Run the same reject fixture twice with `--format json` and compare outputs.

Required:

- byte-identical JSON outputs.
- no timestamps or nondeterministic ordering.

## Inherited qsc/refimpl/formal/vector tests

Run:

```bash
PYTHONPYCACHEPREFIX="$PROOF_DIR/validation/pycache" python3 -m py_compile scripts/audit/validate_binding_fuzz_corpus_no_secrets.py
python3 -m json.tool inputs/suite2/internal_negative_binding_vectors/qsl_binding_negative_vector_manifest_v1.json >/dev/null
python3 formal/model_qsc_kem_signature_transcript_binding_bounded.py
python3 formal/run_model_checks.py
cargo test -p qsc --locked --test kem_signature_transcript_binding_negative -- --test-threads=1 --nocapture
RUSTFLAGS='--cfg qsc_binding_fuzz_helper' cargo test -p qsc --locked --test kem_signature_transcript_binding_negative -- --test-threads=1 --nocapture
cargo test -p quantumshield_refimpl --features pqcrypto --locked --test signature_provider_boundary -- --test-threads=1 --nocapture
cargo test -p quantumshield_refimpl --features pqcrypto --locked --test pqkem768
cargo test -p qsc --locked --test key_lifecycle_zeroization -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test handshake_provider_error_no_mutation -- --test-threads=1 --nocapture
```

Use a proof-root `CARGO_TARGET_DIR` if needed to keep generated build artifacts
outside the repo tree.

Required:

- all commands pass.
- cargo-fuzz local unavailability, if reached by the full qsc-adversarial
  script, is recorded exactly and PR CI qsc-adversarial-smoke remains required.

## Audit/fmt checks

Run:

```bash
cargo audit --deny warnings
cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock
cargo fmt --check
sh -n scripts/ci/qsc_adversarial.sh
bash -n scripts/ci/qsc_adversarial.sh
git diff --check
python3 scripts/ci/qsl_evidence_helper.py link-check --root .
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
python3 scripts/ci/qsl_evidence_helper.py pr-body-preflight --file "$PR_BODY_FILE"
```

Required:

- root audit PASS.
- nested qsc fuzz lock audit PASS.
- cargo audit green is dependency-health evidence only.
- formatting PASS.
- shell syntax PASS.
- diff check PASS.
- link-check PASS.
- leak-scan PASS.
- PR body preflight PASS.

## Public claim boundary

Required:

- no public-readiness claim is introduced.
- no production-readiness claim is introduced.
- no public-internet-readiness claim is introduced.
- no external-review-complete claim is introduced.
- no crypto-complete claim is introduced.
- no fuzz-complete claim is introduced.
- no corpus-complete claim is introduced.
- no vector-complete claim is introduced.
- no replay-proof claim is introduced.
- no downgrade-proof claim is introduced.
- no side-channel-free claim is introduced.
- no vulnerability-free claim is introduced.
- no bug-free claim is introduced.
- no perfect-crypto claim is introduced.

## Closeout prerequisites

Before optional closeout:

- implementation PR merged with required checks green.
- post-merge public-safety green.
- qsc-adversarial-smoke green or accepted skipped/neutral per directive.
- READY remains NA-0491.
- D-0971 exists once on main.
- D-0972 absent before closeout.
- exact successor selected.
- exactly one READY item remains.
