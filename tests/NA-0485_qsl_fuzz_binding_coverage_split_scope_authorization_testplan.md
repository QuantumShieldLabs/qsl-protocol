Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-15

# NA-0485 QSL Fuzz Binding Coverage Split-Scope Authorization Testplan

Goals: G1, G2, G3, G4, G5

## Objective

Validate the NA-0485 split-scope authorization lane. The lane must consume
NA-0484, inventory current qsc fuzz/Cargo/script/CI shape, classify helper/API
access, classify exact Cargo/script/CI and corpus/secret boundaries, select one
successor, and preserve no-implementation/no-public-overclaim boundaries.

## Protected invariants

- qwork proof files are read and copied, not regenerated.
- Exactly one READY item remains.
- NA-0484 inheritance is consumed.
- Existing qsc fuzz targets are inventoried.
- qsc fuzz Cargo metadata is inventoried.
- qsc adversarial script and workflow integration are inventoried.
- Helper/API access is classified.
- Cargo/script/CI exact scope is classified.
- Corpus/secret-material strategy is classified.
- No fuzz implementation is performed.
- No fuzz target or corpus is mutated.
- No qsc source or executable test is mutated.
- No input/vector path is mutated.
- No runtime, crypto, dependency, Cargo, lockfile, workflow, script, executable
  test, formal model, refimpl source/test, service, public, website, README,
  START_HERE, qwork/qstart/qresume/qshell, qsl-backup, backup status, backup
  plan, rollback, durable Director State Index, public technical paper, or
  backup tree mutation occurs.
- no public-readiness claim is introduced.
- no production-readiness claim is introduced.
- no public-internet-readiness claim is introduced.
- no external-review-complete claim is introduced.
- no crypto-complete claim is introduced.
- no fuzz-complete claim is introduced.
- no vector-complete claim is introduced.
- no KEM-complete claim is introduced.
- no signature-complete claim is introduced.
- no identity-complete claim is introduced.
- no transcript-complete claim is introduced.
- no qsc/refimpl-equivalence-complete claim is introduced.
- no provider-boundary-complete claim is introduced.
- no provider-RNG-complete claim is introduced.
- no formal-proof-complete claim is introduced.
- no replay-proof claim is introduced.
- no downgrade-proof claim is introduced.
- no side-channel-free claim is introduced.
- no vulnerability-free claim is introduced.
- no bug-free claim is introduced.
- no perfect-crypto claim is introduced.
- Cargo audit green remains dependency-health evidence only.

## Allowed scope

- `docs/governance/evidence/NA-0485_qsl_fuzz_binding_coverage_split_scope_authorization_plan.md`
- this testplan
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Forbidden scope

- fuzz implementation mutation;
- fuzz target or corpus mutation;
- qsc runtime/source or executable-test mutation;
- refimpl source or executable-test mutation;
- dependency, Cargo manifest, lockfile, workflow, or script mutation;
- vector/input mutation;
- formal model mutation;
- qsl-server, qsl-attachments, qshield runtime, qshield-cli, service, website,
  public-doc, README, START_HERE mutation;
- qwork, qstart, qresume, qshell mutation;
- backup, restore, qsl-backup, backup status, backup plan, rollback, systemd,
  timer, fstab, or backup tree mutation.

## Startup validation

Run:

```bash
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
python3 scripts/ci/qsl_evidence_helper.py public-safety-status --sha "$(git rev-parse origin/main)"
cargo audit --deny warnings
cargo tree -i rustls-webpki --locked
cargo tree -i ml-kem --locked || true
cargo tree -i pqcrypto-mlkem --locked || true
cargo tree -i pqcrypto-traits --locked || true
cargo tree -i pqcrypto-internals --locked || true
cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock
rg -n "pqcrypto-mlkem|pqcrypto-traits|pqcrypto-internals" qsl/qsl-client/qsc/fuzz/Cargo.lock || true
```

Required:

- READY_COUNT 1;
- READY NA-0485;
- D-0956 and D-0957 exist once;
- D-0958 absent at startup;
- duplicate decision count zero;
- public-safety success on current `origin/main`;
- root cargo audit green;
- nested qsc fuzz lock audit green.

## Fuzz / Cargo / script / CI inventory validation

Run:

```bash
rg --files qsl/qsl-client/qsc/fuzz qsl/qsl-client/qsc/fuzz/fuzz_targets
find qsl/qsl-client/qsc/fuzz -maxdepth 3 -type d -o -type f | sort
sed -n '1,220p' qsl/qsl-client/qsc/fuzz/Cargo.toml
sed -n '1,260p' scripts/ci/qsc_adversarial.sh
sed -n '1,220p' .github/workflows/qsc-adversarial.yml
sed -n '1,260p' qsl/qsl-client/qsc/fuzz/fuzz_targets/qsc_route_http.rs
sed -n '1,260p' qsl/qsl-client/qsc/fuzz/fuzz_targets/qsc_payload_boundaries.rs
sed -n '1,260p' qsl/qsl-client/qsc/fuzz/fuzz_targets/qsc_vault_envelope.rs
```

Expected:

- target `qsc_route_http`;
- target `qsc_payload_boundaries`;
- target `qsc_vault_envelope`;
- corpora only for those three target names;
- `qsl/qsl-client/qsc/fuzz/Cargo.toml` explicitly enumerates all targets as
  `[[bin]]`;
- `scripts/ci/qsc_adversarial.sh` explicitly invokes the three target names;
- `.github/workflows/qsc-adversarial.yml` invokes the script and does not
  enumerate fuzz target names.

Expected classifications:

- `EXISTING_FUZZ_TARGETS_ENUMERATED_OK`;
- `FUZZ_TARGET_ADD_REQUIRES_CARGO_TOML`;
- `FUZZ_TARGET_ADD_REQUIRES_SCRIPT_CHANGE`;
- no workflow mutation expected;
- no dependency or lockfile mutation expected if existing dependencies suffice.

## Helper / API validation

Run:

```bash
rg -n "pub mod adversarial|pub mod envelope" qsl/qsl-client/qsc/src/lib.rs
find qsl/qsl-client/qsc/src -maxdepth 2 -type f | sort | rg 'adversarial|handshake|identity'
rg -n "fn hs_decode_init|fn hs_decode_resp|fn hs_decode_confirm|fn hs_transcript_hash|fn hs_transcript_mac|pub\\(crate\\) fn handshake_init|pub\\(crate\\) fn handshake_poll" qsl/qsl-client/qsc/src/handshake/mod.rs
```

Required:

- current public qsc library exports `adversarial` and `envelope`;
- current public adversarial helpers are parser helpers;
- A1/B1/A2 and semantic binding helpers are internal/private or `pub(crate)`;
- selected helper/API classification is
  `FUZZ_HELPER_ACCESS_REQUIRES_RUNTIME_SOURCE_DESIGN`;
- selected readiness classification is
  `FUZZ_BINDING_HELPER_API_DESIGN_NEEDED`.
- evidence records that qsc binding fuzz helper/API design outranks immediate
  side-channel review, vector-consumer test expansion, refimpl boundary fuzz,
  and external-review packaging because helper/API access is the direct blocker
  for semantic qsc binding fuzz.

## Binding and vector inheritance validation

Run:

```bash
python3 -m json.tool inputs/suite2/internal_negative_binding_vectors/qsl_binding_negative_vector_manifest_v1.json >/dev/null
python3 - <<'PY'
import json
p = "inputs/suite2/internal_negative_binding_vectors/qsl_binding_negative_vector_manifest_v1.json"
d = json.load(open(p))
print(len(d.get("vectors", [])))
PY
python3 formal/model_qsc_kem_signature_transcript_binding_bounded.py
python3 formal/run_model_checks.py
```

Required:

- manifest valid JSON;
- manifest has 34 vector metadata entries;
- formal binding model green;
- formal runner green;
- manifest remains internal metadata guidance only and no vector/input mutation
  occurs.

## Inherited qsc/refimpl tests

Run:

```bash
cargo test -p qsc --locked --test kem_signature_transcript_binding_negative -- --test-threads=1 --nocapture
cargo test -p quantumshield_refimpl --features pqcrypto --locked --test signature_provider_boundary -- --test-threads=1 --nocapture
RUSTFLAGS='--cfg qsc_rng_failure_test_seam' cargo test -p qsc --locked --test tui_account_bootstrap_transactionality -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test tui_account_bootstrap_transactionality -- --test-threads=1 --nocapture
RUSTFLAGS='--cfg qsc_rng_failure_test_seam' cargo test -p qsc --locked --test cli_identity_rotation_provider_rng_failure -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test cli_identity_rotation_provider_rng_failure -- --test-threads=1 --nocapture
RUSTFLAGS='--cfg qsc_rng_failure_test_seam' cargo test -p qsc --locked --test legacy_identity_public_record_provider_rng_failure -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test legacy_identity_public_record_provider_rng_failure -- --test-threads=1 --nocapture
RUSTFLAGS='--cfg qsc_rng_failure_test_seam' cargo test -p qsc --locked --test lazy_identity_provider_rng_failure -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test lazy_identity_provider_rng_failure -- --test-threads=1 --nocapture
RUSTFLAGS='--cfg qsc_rng_failure_test_seam' cargo test -p qsc --locked --test a2_signature_provider_rng_failure -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test a2_signature_provider_rng_failure -- --test-threads=1 --nocapture
RUSTFLAGS='--cfg qsc_rng_failure_test_seam' cargo test -p qsc --locked --test b1_signature_provider_rng_failure -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test b1_signature_provider_rng_failure -- --test-threads=1 --nocapture
RUSTFLAGS='--cfg qsc_rng_failure_test_seam' cargo test -p qsc --locked --test kem_provider_rng_failure -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test kem_provider_rng_failure -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test key_lifecycle_zeroization -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test handshake_provider_error_no_mutation -- --test-threads=1 --nocapture
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
cargo test -p quantumshield_refimpl --features pqcrypto --locked --test pqkem768
```

Required:

- all listed tests pass;
- qsc binding negative tests green;
- refimpl signature provider-boundary test green;
- inherited qsc provider-RNG/key-lifecycle/provider-error tests green;
- refimpl `pqkem768` green.

## Scope guard

Changed paths must be exactly limited to:

- `docs/governance/evidence/NA-0485_qsl_fuzz_binding_coverage_split_scope_authorization_plan.md`
- `tests/NA-0485_qsl_fuzz_binding_coverage_split_scope_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Require zero changes under:

- `qsl/qsl-client/qsc/fuzz/fuzz_targets/`;
- `qsl/qsl-client/qsc/fuzz/corpus/`;
- `qsl/qsl-client/qsc/fuzz/Cargo.toml`;
- `qsl/qsl-client/qsc/fuzz/Cargo.lock`;
- `scripts/ci/qsc_adversarial.sh`;
- `.github/workflows/`;
- `inputs/`;
- `qsl/qsl-client/qsc/src/`;
- `qsl/qsl-client/qsc/tests/`;
- `tools/refimpl/`;
- `formal/`;
- Cargo manifests or lockfiles;
- service/public/backup/qwork paths.

## Link, leak, and overclaim validation

Run:

```bash
git diff --check
python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main --allowed docs/governance/evidence/NA-0485_qsl_fuzz_binding_coverage_split_scope_authorization_plan.md --allowed tests/NA-0485_qsl_fuzz_binding_coverage_split_scope_authorization_testplan.md --allowed DECISIONS.md --allowed TRACEABILITY.md --allowed docs/ops/ROLLING_OPERATIONS_JOURNAL.md
python3 scripts/ci/qsl_evidence_helper.py leak-scan --base origin/main
cargo fmt --check
```

Also run:

- manual relative markdown link-integrity check from AGENTS.md;
- PR body preflight;
- goal-lint after PR body exists;
- added-line overclaim scan.

Expected:

- diff check passes;
- scope guard passes;
- link check passes;
- leak scan passes;
- PR body preflight passes;
- goal-lint passes;
- overclaim scan finds no added unsupported public readiness/completion/proof
  claim.

## Adversarial script syntax and optional local smoke

Run:

```bash
rg -n "NA0439_QSC_PROVIDER_ERROR_NO_MUTATION_ADVERSARIAL_STEP|handshake_provider_error_no_mutation" scripts/ci/qsc_adversarial.sh
sh -n scripts/ci/qsc_adversarial.sh
bash -n scripts/ci/qsc_adversarial.sh
```

If feasible:

```bash
sh scripts/ci/qsc_adversarial.sh
```

If local cargo-fuzz is unavailable, record exact output and require PR CI
`qsc-adversarial-smoke` evidence.

## PR validation

- PR body must include Goals, Impact, No-regression, and Tests/Vectors.
- PR body must mention fuzz binding split-scope authorization.
- PR body must mention selected successor.
- PR body must mention no implementation mutation.
- PR body must mention no fuzz/vector/input mutation.
- PR body must mention no runtime/crypto/dependency/Cargo/lockfile/workflow/
  test/formal/refimpl mutation.
- PR body must include no public overclaim and no external-review-complete
  claim.
- Merge only after required checks pass.
- Post-merge public-safety must complete success.

## Acceptance criteria

- NA-0484 consumed.
- D-0958 accepted.
- Selected successor is
  `NA-0486 -- QSL qsc Binding Fuzz Helper / API Design Authorization Plan`.
- Implementation readiness classification is
  `FUZZ_BINDING_HELPER_API_DESIGN_NEEDED`.
- Helper/API access classification is
  `FUZZ_HELPER_ACCESS_REQUIRES_RUNTIME_SOURCE_DESIGN`.
- Cargo/script/CI exact scope classification is
  `FUZZ_EXACT_SCOPE_SPLIT_REQUIRED`.
- Corpus strategy is ephemeral generation plus metadata-only guidance.
- No implementation mutation.
- No public overclaim.
- Exactly one READY item remains.
