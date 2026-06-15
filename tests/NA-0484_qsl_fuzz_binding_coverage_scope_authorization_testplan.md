Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-15

# NA-0484 QSL Fuzz Binding Coverage Scope Authorization Testplan

Goals: G1, G2, G3, G4, G5

## Objective

Validate the NA-0484 authorization lane. The lane must inventory current qsc
fuzz coverage, decide whether current fuzz reaches binding semantics, classify
future fuzz implementation impact, select one successor, and preserve all
no-implementation/no-public-overclaim boundaries.

## Protected invariants

- qwork proof files are read and copied, not regenerated.
- Exactly one READY item remains.
- NA-0483 internal negative vector evidence is consumed.
- Current qsc fuzz targets are inventoried.
- Binding fuzz candidate surfaces are inventoried.
- Cargo/workflow/CI impact is classified.
- Secret-material/corpus risk is classified.
- No fuzz implementation is performed.
- No fuzz target or corpus is mutated.
- No input/vector path is mutated.
- No runtime, crypto, dependency, Cargo, lockfile, workflow, executable test,
  formal model, qsc source/test, refimpl source/test, service, public,
  website, README, START_HERE, qwork/qstart/qresume/qshell, qsl-backup,
  backup status, backup plan, rollback, durable Director State Index, public
  technical paper, or backup tree mutation occurs.
- No public-readiness claim is introduced.
- No production-readiness claim is introduced.
- No public-internet-readiness claim is introduced.
- No external-review-complete claim is introduced.
- No crypto-complete claim is introduced.
- No fuzz-complete claim is introduced.
- No vector-complete claim is introduced.
- No KEM-complete claim is introduced.
- No signature-complete claim is introduced.
- No identity-complete claim is introduced.
- No transcript-complete claim is introduced.
- No qsc/refimpl-equivalence-complete claim is introduced.
- No provider-boundary-complete claim is introduced.
- No provider-RNG-complete claim is introduced.
- No formal-proof-complete claim is introduced.
- No replay-proof claim is introduced.
- No downgrade-proof claim is introduced.
- No side-channel-free claim is introduced.
- No vulnerability-free claim is introduced.
- No bug-free claim is introduced.
- No perfect-crypto claim is introduced.
- No off-host-backup-complete claim is introduced.
- No disaster-recovery-complete claim is introduced.
- No restore-proof claim is introduced.
- No backup-complete claim is introduced.
- No metadata-free claim is introduced.
- No anonymity claim is introduced.
- No untraceable claim is introduced.
- Cargo audit green remains dependency-health evidence only.

## Allowed scope

- `docs/governance/evidence/NA-0484_qsl_fuzz_binding_coverage_scope_authorization_plan.md`
- this testplan
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Forbidden scope

- implementation mutation;
- runtime/source or crypto mutation;
- qsc source or executable-test mutation;
- refimpl source or executable-test mutation;
- dependency, Cargo manifest, lockfile, or workflow mutation;
- fuzz target or fuzz corpus mutation;
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
- READY NA-0484;
- D-0954 and D-0955 exist once;
- D-0956 absent at startup;
- duplicate decision count zero;
- public-safety success on current `origin/main`;
- root cargo audit green;
- nested qsc fuzz lock audit green.

## Fuzz inventory validation

Run:

```bash
rg --files qsl/qsl-client/qsc/fuzz qsl/qsl-client/qsc/fuzz/fuzz_targets
sed -n '1,220p' qsl/qsl-client/qsc/fuzz/Cargo.toml
sed -n '1,260p' scripts/ci/qsc_adversarial.sh
sed -n '1,220p' .github/workflows/qsc-adversarial.yml
sed -n '1,260p' qsl/qsl-client/qsc/fuzz/fuzz_targets/qsc_route_http.rs
sed -n '1,260p' qsl/qsl-client/qsc/fuzz/fuzz_targets/qsc_payload_boundaries.rs
sed -n '1,260p' qsl/qsl-client/qsc/fuzz/fuzz_targets/qsc_vault_envelope.rs
```

Expected current inventory:

- target `qsc_route_http`;
- target `qsc_payload_boundaries`;
- target `qsc_vault_envelope`;
- corpora only for those three target names;
- `scripts/ci/qsc_adversarial.sh` explicitly runs those three targets;
- no current fuzz target directly reaches A1/B1/A2 binding semantics.

Expected classification:

- `FUZZ_BINDING_CURRENTLY_PARSER_ONLY`

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
rg -n "hs_decode_init|hs_decode_resp|hs_decode_confirm|hs_parse_parameter_block|hs_transcript_hash|hs_transcript_mac" qsl/qsl-client/qsc/src/handshake/mod.rs
rg -n "pub mod adversarial|pub mod envelope" qsl/qsl-client/qsc/src/lib.rs
```

Required:

- manifest valid JSON;
- manifest has 34 vector metadata entries;
- handshake binding helpers are identified as current qsc source internals;
- current public qsc library exports do not expose a binding/handshake
  adversarial helper.

## Inherited qsc/refimpl/formal tests

Run:

```bash
python3 formal/model_qsc_kem_signature_transcript_binding_bounded.py
python3 formal/run_model_checks.py
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
- formal binding model green;
- formal runner green;
- qsc binding negative tests green;
- refimpl signature provider-boundary test green;
- inherited qsc provider-RNG/key-lifecycle/provider-error tests green;
- refimpl `pqkem768` green.

## Scope guard

Changed paths must be exactly limited to:

- `docs/governance/evidence/NA-0484_qsl_fuzz_binding_coverage_scope_authorization_plan.md`
- `tests/NA-0484_qsl_fuzz_binding_coverage_scope_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Run a changed-path guard and require zero changes under:

- `qsl/qsl-client/qsc/fuzz/fuzz_targets/`;
- `qsl/qsl-client/qsc/fuzz/corpus/`;
- `inputs/`;
- `qsl/qsl-client/qsc/src/`;
- `qsl/qsl-client/qsc/tests/`;
- `tools/refimpl/`;
- `formal/`;
- `.github/workflows/`;
- Cargo manifests or lockfiles;
- service/public/backup/qwork paths.

## Link, leak, and overclaim validation

Run:

```bash
git diff --check
python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main --allowed docs/governance/evidence/NA-0484_qsl_fuzz_binding_coverage_scope_authorization_plan.md --allowed tests/NA-0484_qsl_fuzz_binding_coverage_scope_authorization_testplan.md --allowed DECISIONS.md --allowed TRACEABILITY.md --allowed docs/ops/ROLLING_OPERATIONS_JOURNAL.md
python3 scripts/ci/qsl_evidence_helper.py leak-scan --base origin/main
python3 scripts/ci/qsl_evidence_helper.py pr-body-preflight --body-file <pr-body-file>
python3 tools/goal_lint.py
cargo fmt --check
```

Also run the manual relative markdown link-integrity check from AGENTS.md.

Expected:

- diff check passes;
- scope guard passes;
- link check passes;
- leak scan passes;
- PR body preflight passes;
- goal-lint passes;
- overclaim scan finds no added public readiness/completion/proof claim.

## Adversarial script syntax and optional local smoke

Run:

```bash
rg -n "NA0439_QSC_PROVIDER_ERROR_NO_MUTATION_ADVERSARIAL_STEP|handshake_provider_error_no_mutation" scripts/ci/qsc_adversarial.sh
sh -n scripts/ci/qsc_adversarial.sh
bash -n scripts/ci/qsc_adversarial.sh
```

If feasible, run:

```bash
sh scripts/ci/qsc_adversarial.sh
```

If local cargo-fuzz is unavailable, record the exact output and rely on PR CI
`qsc-adversarial-smoke`.

## PR and merge validation

Before PR:

- READY_COUNT 1;
- READY NA-0484;
- D-0956 exists once;
- D-0957 absent;
- no duplicate decision IDs;
- changed paths limited to allowed NA-0484 paths;
- no fuzz target/corpus/vector/input/runtime/source/dependency/Cargo/lockfile/workflow/test/formal/refimpl mutation;
- no backup/restore/qsl-backup/status/plan/rollback mutation;
- root audit green;
- nested qsc fuzz lock audit green;
- inherited formal/qsc/refimpl tests green;
- no public overclaim.

After merge:

- public-safety completes success on the merge commit;
- required checks are success, skipped, or neutral per repo policy;
- main still has exactly one READY item until closeout;
- D-0956 is present on main;
- no qwork post-merge run.

## Acceptance criteria

- Current fuzz classification recorded as `FUZZ_BINDING_CURRENTLY_PARSER_ONLY`.
- Cargo/workflow/CI impact recorded as `FUZZ_IMPLEMENTATION_SPLIT_NEEDED`.
- Secret/corpus risk recorded as `FUZZ_CORPUS_GENERATED_EPHEMERAL_ONLY`.
- Primary authorization classification recorded as `FUZZ_BINDING_SPLIT_SCOPE_NEEDED`.
- Selected successor recorded as
  `NA-0485 -- QSL Fuzz Binding Coverage Split-Scope Authorization Plan`.
- No implementation mutation.
- No public overclaim.
- Exactly one READY item remains.
