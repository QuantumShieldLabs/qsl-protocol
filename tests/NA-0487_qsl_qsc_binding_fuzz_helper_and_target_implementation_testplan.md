Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-16

# NA-0487 qsc Binding Fuzz Helper and Target Implementation Testplan

Goals: G1, G2, G3, G4, G5

## Objective

Validate the recovered NA-0487 implementation harness. The lane must add a
cfg-gated qsc binding fuzz helper, a semantic binding fuzz target, qsc fuzz
Cargo metadata, and qsc-adversarial script inclusion while preserving normal
production behavior and all forbidden mutation boundaries.

## Protected invariants

- exactly one READY item at startup
- READY remains NA-0487 until closeout
- qwork proof files are read, not regenerated
- helper is visible only under exact cfg `qsc_binding_fuzz_helper`
- normal no-cfg qsc builds do not expose or require the helper
- helper emits no secret/private material
- helper does not use a target-local fake oracle
- no dependency mutation
- no root `Cargo.lock` mutation
- no qsc fuzz `Cargo.lock` mutation
- no workflow mutation
- no checked-in corpus
- no vector/input mutation
- no formal mutation
- no refimpl mutation
- no service/public-doc/backup/qsl-backup mutation
- no directive-forbidden readiness, completion, proof, side-channel, or
  vulnerability claim

## Allowed scope

- `qsl/qsl-client/qsc/src/lib.rs`
- `qsl/qsl-client/qsc/src/adversarial/mod.rs`
- `qsl/qsl-client/qsc/src/adversarial/binding_fuzz.rs`
- `qsl/qsl-client/qsc/src/handshake/mod.rs`
- `qsl/qsl-client/qsc/src/identity/mod.rs`
- `qsl/qsl-client/qsc/fuzz/fuzz_targets/qsc_binding_semantics.rs`
- `qsl/qsl-client/qsc/fuzz/Cargo.toml`
- `scripts/ci/qsc_adversarial.sh`
- `docs/governance/evidence/NA-0487_qsl_qsc_binding_fuzz_helper_and_target_implementation_harness.md`
- this testplan
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Forbidden scope

- dependencies and lockfiles
- workflows
- vectors, inputs, and checked-in corpus
- formal models
- refimpl code
- qsc executable tests
- qsl-server, qsl-attachments, qshield runtime, qshield-cli
- website, public docs, README, START_HERE
- qwork, qstart, qresume, qshell mutation
- backup, restore, qsl-backup, rollback, backup status, backup plan

## cfg gating validation

Run:

```bash
rg -n "qsc_binding_fuzz_helper|binding_fuzz" qsl/qsl-client/qsc/src/lib.rs qsl/qsl-client/qsc/src/adversarial/mod.rs qsl/qsl-client/qsc/src/handshake/mod.rs qsl/qsl-client/qsc/src/identity/mod.rs
RUSTFLAGS='--cfg qsc_binding_fuzz_helper' cargo check -p qsc --locked
```

Required:

- `adversarial::binding_fuzz` export is cfg-gated
- helper compiles with cfg
- helper is not exported in normal no-cfg builds
- no production behavior drift claim is introduced

## Normal no-cfg validation

Run:

```bash
cargo test -p qsc --locked --test kem_signature_transcript_binding_negative -- --test-threads=1 --nocapture
```

Required:

- pass without `qsc_binding_fuzz_helper`
- existing qsc binding negative reject markers remain present
- no helper is required by normal build

## Fuzz target validation

Run:

```bash
RUSTFLAGS='--cfg qsc_binding_fuzz_helper' cargo check --manifest-path qsl/qsl-client/qsc/fuzz/Cargo.toml --locked --bin qsc_binding_semantics
```

If local cargo-fuzz exists, run:

```bash
cd qsl/qsl-client/qsc/fuzz
RUSTFLAGS='--cfg qsc_binding_fuzz_helper' cargo fuzz run qsc_binding_semantics -- -runs=1
```

If local cargo-fuzz is unavailable, record exact output and require PR CI
`qsc-adversarial-smoke` for cargo-fuzz-backed evidence.

## qsc-adversarial script validation

Run:

```bash
sh -n scripts/ci/qsc_adversarial.sh
bash -n scripts/ci/qsc_adversarial.sh
```

Required:

- script syntax passes
- existing targets are preserved
- `qsc_binding_semantics` is included with cfg `qsc_binding_fuzz_helper`
- provider-error no-mutation step and marker are preserved

## Inherited qsc/refimpl/formal/vector tests

Run:

```bash
python3 -m json.tool inputs/suite2/internal_negative_binding_vectors/qsl_binding_negative_vector_manifest_v1.json >/dev/null
python3 formal/model_qsc_kem_signature_transcript_binding_bounded.py
python3 formal/run_model_checks.py
cargo test -p qsc --locked --test kem_signature_transcript_binding_negative -- --test-threads=1 --nocapture
RUSTFLAGS='--cfg qsc_binding_fuzz_helper' cargo test -p qsc --locked --test kem_signature_transcript_binding_negative -- --test-threads=1 --nocapture
cargo test -p quantumshield_refimpl --features pqcrypto --locked --test signature_provider_boundary -- --test-threads=1 --nocapture
```

Also run inherited qsc provider-RNG, key-lifecycle, provider-error, stable
`send_commit`, and refimpl `pqkem768` tests listed in D349.

## Cargo/audit/fmt checks

Run:

```bash
cargo fmt --check
cargo audit --deny warnings
cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock
cargo tree -i rustls-webpki --locked
cargo tree -i ml-kem --locked || true
cargo tree -i pqcrypto-mlkem --locked || true
cargo tree -i pqcrypto-traits --locked || true
cargo tree -i pqcrypto-internals --locked || true
```

Required:

- fmt passes
- root audit passes
- nested qsc fuzz lock audit passes
- dependency probes do not require mutation
- no lockfile changes

## Public claim boundary

Evidence and PR text must state that this is bounded internal engineering
evidence only. It must not claim directive-forbidden readiness, completion,
proof, side-channel, or vulnerability properties.

## Closeout prerequisites

NA-0487 closeout to NA-0488 is allowed only after:

- implementation PR merges
- post-merge public-safety is green
- post-merge qsc-adversarial-smoke is green
- queue remains exactly one READY item
- D-0963 exists on main
- no forbidden mutation is present
- selected NA-0488 block preserves no-runtime, no-crypto, no-dependency,
  no-workflow, no-secret, and no-public-overclaim boundaries
