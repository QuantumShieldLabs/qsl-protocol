Status: Supporting
Owner: QSL Director
Last-Updated: 2026-06-17

# NA-0492 Checked-In Corpus Scope Authorization Testplan

## Scope

This testplan covers the NA-0492 authorization-only governance patch. It does
not authorize corpus, vector, input, qsc source, qsc fuzz target, Cargo,
lockfile, script, workflow, dependency, formal, refimpl, service, public-doc,
backup, restore, or qwork tooling mutation.

## Required local validation

- qwork proof files are present, parsed, and fresh against live pre-fetch refs.
- Queue proof shows `READY_COUNT=1` and READY `NA-0492`.
- D-0971 and D-0972 exist once; D-0973 is absent before patch and exists once
  after patch.
- D359 response exists and the exact stray file
  `/tmp/na0492_cargo_fuzz_version.out` is either absent or copied into the proof
  root, hashed, summarized, deleted, and verified absent.
- Disk proof shows `/` below 95%.
- qsl-backup is checked read-only and matches the expected digest/source-list
  boundary.
- Current qsc fuzz corpus inventory is recorded.
- `qsc_binding_semantics` corpus absence is recorded.
- Binding target categories and qsc-adversarial target inclusion are recorded.
- Local cargo-fuzz availability is recorded under the proof root.

## Validator gate

Run and record:

```bash
python3 scripts/audit/validate_binding_fuzz_corpus_no_secrets.py --format json --path qsl/qsl-client/qsc/fuzz/corpus
python3 scripts/audit/validate_binding_fuzz_corpus_no_secrets.py --format json --allow-missing --path qsl/qsl-client/qsc/fuzz/corpus/qsc_binding_semantics
```

Expected results:

- Current corpus scan passes with zero findings.
- Missing `qsc_binding_semantics` corpus passes only with explicit
  `--allow-missing`.
- Missing `qsc_binding_semantics` corpus without `--allow-missing` exits
  nonzero as fail-closed proof.

## Scope guard

Changed qsl-protocol paths must be limited to:

- `docs/governance/evidence/NA-0492_qsl_binding_fuzz_checked_in_corpus_scope_authorization_plan.md`
- `tests/NA-0492_qsl_binding_fuzz_checked_in_corpus_scope_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

No corpus/vector/input file may be added, deleted, moved, archived, or modified
by NA-0492.

## Broader validation

Required validation commands:

```bash
git diff --check
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

Run inherited qsc provider-RNG, key-lifecycle, and provider-error tests as
needed by the directive. If local full qsc-adversarial execution reaches missing
local cargo-fuzz, record the exact output under the proof root and rely on PR CI
`qsc-adversarial-smoke` if green.

## PR body and claim boundary

The PR body must include:

```md
Goals: G1, G2, G3, G4, G5
Impact:
No-regression:
Tests/Vectors:
```

The PR body must state authorization-only scope, selected successor, no
implementation mutation, no corpus/vector/input mutation, no qsc
source/fuzz/Cargo/script/workflow mutation, no dependency/lockfile mutation, and
no public overclaim.

No public-readiness claim is allowed. no production-readiness claim is allowed.
no public-internet-readiness claim is allowed. no external-review-complete claim
is allowed. no crypto-complete claim is allowed. no fuzz-complete claim is
allowed. no corpus-complete claim is allowed. no vector-complete claim is
allowed. no replay-proof claim is allowed. no downgrade-proof claim is allowed.
no side-channel-free claim is allowed. no vulnerability-free claim is allowed.
no bug-free claim is allowed. no perfect-crypto claim is allowed.

## Expected future NA-0493 authorization output

If NA-0492 merges and closeout restores NA-0493, the future implementation lane
is expected to add exactly seven small raw binary seed files under
`qsl/qsl-client/qsc/fuzz/corpus/qsc_binding_semantics/`:

- `seed_00_a1_mutation.bin`
- `seed_01_b1_mutation.bin`
- `seed_02_a2_mutation.bin`
- `seed_03_suite_confusion.bin`
- `seed_04_replay.bin`
- `seed_05_stale_public_record.bin`
- `seed_ff_vector_traceability.bin`

Each file must be at most 64 bytes and must pass the validator before commit.
