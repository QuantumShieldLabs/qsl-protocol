Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-16

# NA-0489 Binding Fuzz Ephemeral Seed Strategy Implementation Testplan

Goals: G1, G2, G3, G4, G5

## Objective

Validate deterministic ephemeral seed recipe implementation inside
`qsc_binding_semantics` while preserving no checked-in corpus, no runtime
manifest consumption, no secret/private material, no qsc source/helper
mutation, no Cargo/script/workflow mutation, and no public-claim expansion.

## Protected invariants

- qwork proof files are read, not regenerated.
- READY_COUNT is exactly 1 at startup.
- READY item is NA-0489 at startup.
- NA-0488 is DONE.
- NA-0487 is DONE.
- D-0965 exists once.
- D-0966 exists once.
- D-0967 is absent before the patch and exists once after the patch.
- duplicate decision count is zero.
- PR #1248 is merged at `fdc67c8b8526`.
- `qsc_binding_semantics` keeps arbitrary-byte coverage.
- deterministic seed recipes are target-local only.
- no checked-in binding corpus is introduced.
- no vector/input mutation is introduced.
- no qsc source/helper, qsc fuzz Cargo, qsc-adversarial script, workflow,
  dependency, lockfile, formal, refimpl, service, public-doc, backup, or
  qsl-backup mutation is introduced.
- no public-readiness claim, no crypto-complete claim, no fuzz-complete claim,
  no corpus-complete claim, no vector-complete claim, no replay-proof claim, no
  downgrade-proof claim, no side-channel-free claim, no vulnerability-free
  claim, and no perfect-crypto claim is introduced.

## Allowed scope

- `qsl/qsl-client/qsc/fuzz/fuzz_targets/qsc_binding_semantics.rs`
- `docs/governance/evidence/NA-0489_qsl_binding_fuzz_ephemeral_seed_strategy_implementation_harness.md`
- this testplan
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Forbidden scope

- checked-in corpus mutation
- vector/input mutation
- qsc source/helper mutation outside the fuzz target
- qsc fuzz Cargo mutation
- qsc fuzz `Cargo.lock` mutation
- root `Cargo.lock` mutation
- qsc-adversarial script mutation
- workflow mutation
- dependency mutation
- formal mutation
- refimpl mutation
- qsl-server, qsl-attachments, qshield runtime, or qshield-cli mutation
- service, public-doc, README, START_HERE, website, or public technical paper
  mutation
- qwork, qstart, qresume, or qshell mutation
- backup, restore, qsl-backup, backup status, backup plan, rollback, or backup
  tree mutation

## Seed recipe validation

Inspect `qsc_binding_semantics.rs`.

Required:

- A1 mutation seed recipe exists.
- B1 mutation seed recipe exists.
- A2 mutation seed recipe exists.
- suite-confusion seed recipe exists.
- replay seed recipe exists.
- stale public-record / trusted-pin seed recipe exists.
- vector-manifest traceability seed recipe exists.
- raw arbitrary-byte exercise remains in the target.
- fuzzer input still drives category and mutation choices.
- generated data is bounded, synthetic, public, and ephemeral.
- target does not panic on empty or arbitrary input.

## No corpus validation

Run:

```bash
find qsl/qsl-client/qsc/fuzz/corpus/qsc_binding_semantics -maxdepth 2 -type f -o -type d 2>/dev/null | sort
git diff --name-only -- qsl/qsl-client/qsc/fuzz/corpus inputs inputs/suite2/internal_negative_binding_vectors
```

Required:

- no `qsc_binding_semantics` corpus directory or files are added.
- no corpus path changes.
- no `inputs/` path changes.
- no internal negative vector manifest changes.

## No secret validation

Run changed-line leak scans and targeted grep over the fuzz target.

Required:

- no private key, KEM secret key, signing key, passphrase, runtime key, backup
  key, operator data, user data, live service data, private endpoint, PEM
  private header, or production-like secret string in changed lines.
- generated seed data uses only public constants, public labels, and arbitrary
  fuzzer input bytes.

## Manifest traceability validation

Run:

```bash
rg -n "qsl_binding_negative_vector_manifest|inputs/suite2|std::fs|File::|include_bytes!|include_str!" qsl/qsl-client/qsc/fuzz/fuzz_targets/qsc_binding_semantics.rs
python3 -m json.tool inputs/suite2/internal_negative_binding_vectors/qsl_binding_negative_vector_manifest_v1.json >/dev/null
```

Required:

- no runtime manifest path reference in target code.
- no `std::fs`, `File::`, `include_bytes!`, or `include_str!` in target code.
- manifest JSON validates as inherited traceability evidence only.

## Fuzz target build/run validation

Run:

```bash
cargo fmt --check
cargo test -p qsc --locked --test kem_signature_transcript_binding_negative -- --test-threads=1 --nocapture
RUSTFLAGS='--cfg qsc_binding_fuzz_helper' cargo test -p qsc --locked --test kem_signature_transcript_binding_negative -- --test-threads=1 --nocapture
RUSTFLAGS='--cfg qsc_binding_fuzz_helper' cargo check --manifest-path qsl/qsl-client/qsc/fuzz/Cargo.toml --locked --bin qsc_binding_semantics
```

If local cargo-fuzz is available, run:

```bash
cd qsl/qsl-client/qsc/fuzz
RUSTFLAGS='--cfg qsc_binding_fuzz_helper' cargo fuzz run qsc_binding_semantics -- -runs=1
```

If local cargo-fuzz is unavailable, record exact output and require PR CI
qsc-adversarial-smoke for cargo-fuzz-backed proof.

## Inherited qsc/refimpl/formal/vector tests

Run the directive-required inherited validation set:

```bash
python3 -m json.tool inputs/suite2/internal_negative_binding_vectors/qsl_binding_negative_vector_manifest_v1.json >/dev/null
python3 formal/model_qsc_kem_signature_transcript_binding_bounded.py
python3 formal/run_model_checks.py
cargo test -p qsc --locked --test kem_signature_transcript_binding_negative -- --test-threads=1 --nocapture
RUSTFLAGS='--cfg qsc_binding_fuzz_helper' cargo test -p qsc --locked --test kem_signature_transcript_binding_negative -- --test-threads=1 --nocapture
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

Required: PASS unless an explicit directive-approved local tool availability
caveat is recorded and PR CI supplies the missing evidence.

## Cargo/audit/fmt checks

Run:

```bash
cargo audit --deny warnings
cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock
cargo tree -i rustls-webpki --locked
cargo tree -i ml-kem --locked || true
cargo fmt --check
```

Required:

- root audit PASS.
- nested qsc fuzz lock audit PASS.
- dependency probes recorded.
- cargo audit green is dependency-health evidence only.
- cargo fmt PASS.

## Public claim boundary

Evidence and PR body must state:

- no public-readiness claim.
- no production-readiness claim.
- no crypto-complete claim.
- no fuzz-complete claim.
- no corpus-complete claim.
- no vector-complete claim.
- no replay-proof claim.
- no downgrade-proof claim.
- no side-channel-free claim.
- no vulnerability-free claim.
- no perfect-crypto claim.

## Closeout prerequisites

Before optional closeout:

- implementation PR merged.
- post-merge public-safety success on implementation merge commit.
- post-merge qsc-adversarial-smoke success on implementation merge commit.
- D-0967 exists once on main.
- queue still has exactly one READY item, NA-0489.
- selected NA-0490 successor is recorded.
- no red required check.
- worktree clean.

Do not implement NA-0490 in this lane.

## Post-fix hardening review

Report:

1. Correctness under stress.
2. Minimality.
3. Maintainability.
4. Coverage quality.
5. Cross-lane stability.
