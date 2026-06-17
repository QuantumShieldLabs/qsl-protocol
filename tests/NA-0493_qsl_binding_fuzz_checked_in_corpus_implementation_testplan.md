Status: Supporting
Owner: QSL Director
Last-Updated: 2026-06-17

# NA-0493 Checked-In Corpus Implementation Testplan

## Objective

Validate that NA-0493 adds exactly the seven authorized raw binary
`qsc_binding_semantics` corpus seeds and only the allowed governance evidence,
while preserving no-secret-material, no-public-claim, no-source-mutation,
no-dependency, and one-READY boundaries.

## Protected invariants

- Exactly one READY item remains during implementation.
- The corpus is data-only and validator-gated.
- Every seed is public/synthetic and at most 64 bytes.
- No qsc source, qsc fuzz target, Cargo, lockfile, script, workflow, dependency,
  formal, refimpl, service, public-doc, backup, or qwork mutation occurs.
- No public-readiness claim is made. no crypto-complete claim is made. no
  fuzz-complete claim is made. no corpus-complete claim is made. no
  vector-complete claim is made. no replay-proof claim is made. no
  downgrade-proof claim is made. no side-channel-free claim is made. no
  vulnerability-free claim is made. no bug-free claim is made. no perfect-crypto
  claim is made.

## Allowed scope

- The seven exact seed files under
  `qsl/qsl-client/qsc/fuzz/corpus/qsc_binding_semantics/`.
- `docs/governance/evidence/NA-0493_qsl_binding_fuzz_checked_in_corpus_implementation_harness.md`
- `tests/NA-0493_qsl_binding_fuzz_checked_in_corpus_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Forbidden scope

- Any corpus file beyond the selected seven files.
- Corpus README or metadata file.
- qsc source mutation.
- qsc fuzz target mutation.
- qsc fuzz Cargo or lockfile mutation.
- root Cargo or lockfile mutation.
- qsc-adversarial script mutation.
- workflow or dependency mutation.
- vector/input mutation outside the exact corpus path.
- formal/refimpl/service/public/qshield/qsl-server/qsl-attachments mutation.
- backup/restore/qsl-backup mutation.
- qwork/qstart/qresume/qshell mutation.
- file move/archive/delete.

## Corpus file validation

Run:

```bash
find qsl/qsl-client/qsc/fuzz/corpus/qsc_binding_semantics -maxdepth 1 -type f -printf '%f %s\n' | sort
find qsl/qsl-client/qsc/fuzz/corpus/qsc_binding_semantics -maxdepth 1 -type f | wc -l
```

Expected:

- exactly seven files.
- every filename matches the selected list.
- every size is at most 64 bytes.
- no README or metadata file exists.

## Validator validation

Run:

```bash
python3 scripts/audit/validate_binding_fuzz_corpus_no_secrets.py --format json --path qsl/qsl-client/qsc/fuzz/corpus/qsc_binding_semantics
python3 scripts/audit/validate_binding_fuzz_corpus_no_secrets.py --format json --path qsl/qsl-client/qsc/fuzz/corpus
```

Expected:

- both commands exit 0.
- finding count is 0.
- no `--allow-missing` is needed after the corpus exists.

## Size/name validation

Run:

```bash
sha256sum qsl/qsl-client/qsc/fuzz/corpus/qsc_binding_semantics/*.bin | sort
file qsl/qsl-client/qsc/fuzz/corpus/qsc_binding_semantics/*.bin || true
```

Expected:

- all seven selected `.bin` files are present.
- no extra corpus file appears.
- all files remain small raw binary seeds.

## Category mapping validation

Verify read-only selector behavior:

- `0x00` selects A1 mutation.
- `0x01` selects B1 mutation.
- `0x02` selects A2 mutation.
- `0x03` selects suite confusion.
- `0x04` selects replay.
- `0x05` selects stale public-record / trusted-pin.
- `0xff` selects vector traceability through the fuzz target special case.

Expected: the seven seed first bytes match these categories.

## No secret validation

Expected:

- dedicated validator finding count is 0.
- corpus bytes contain no private-key marker, passphrase, token, runtime key,
  backup key, production endpoint, operator data, user data, qsc secret
  filename, or internal negative vector manifest JSON content.
- corpus bytes contain no public/conformance/interoperability claim text.

## Fuzz target/Cargo validation

Run:

```bash
RUSTFLAGS='--cfg qsc_binding_fuzz_helper' cargo check --manifest-path qsl/qsl-client/qsc/fuzz/Cargo.toml --locked --bin qsc_binding_semantics
sh -n scripts/ci/qsc_adversarial.sh
bash -n scripts/ci/qsc_adversarial.sh
```

If local cargo-fuzz is available, run:

```bash
cd qsl/qsl-client/qsc/fuzz
RUSTFLAGS='--cfg qsc_binding_fuzz_helper' cargo fuzz run qsc_binding_semantics -- -runs=1
```

If local cargo-fuzz is unavailable, record that fact and rely on PR
`qsc-adversarial-smoke` if green.

## Inherited qsc/refimpl/formal/vector tests

Run:

```bash
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

Expected: all commands exit 0.

## Audit/fmt checks

Run:

```bash
cargo audit --deny warnings
cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock
cargo fmt --check
git diff --check
```

Expected: all commands exit 0.

## Public claim boundary

PR and evidence text must state the bounded internal-engineering nature of the
corpus. They must not state public readiness, crypto completion, fuzz
completion, corpus completion, vector completion, replay proof, downgrade proof,
side-channel freedom, vulnerability freedom, bug freedom, or perfect crypto.

## Closeout prerequisites

Closeout to NA-0494 is allowed only after:

- implementation PR merged.
- post-merge public-safety is green.
- post-merge qsc-adversarial-smoke is green or accepted skipped by directive.
- queue has exactly one READY item.
- D-0975 exists once.
- D-0976 is absent before closeout.
