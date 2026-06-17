Status: Supporting
Owner: QSL Director
Last-Updated: 2026-06-17

# NA-0493 QSL Binding Fuzz Checked-In Corpus Implementation Harness

## Executive summary

NA-0493 consumes NA-0492/D360 authorization and adds the selected minimal
checked-in `qsc_binding_semantics` corpus. The implementation is data-only:
exactly seven raw binary seed files were added under
`qsl/qsl-client/qsc/fuzz/corpus/qsc_binding_semantics/`.

Every seed is eight bytes, public/synthetic, and selector-first. The validator
passed on the new corpus and on the full qsc fuzz corpus with zero findings, and
the new-corpus JSON output matched on a repeated run.

This lane does not mutate qsc source, qsc fuzz target code, qsc fuzz Cargo,
Cargo locks, qsc-adversarial scripts, workflows, dependencies, vectors/inputs,
formal models, refimpl, services, public docs, backup tooling, or qwork tooling.
The selected successor is `NA-0494 -- QSL Binding Fuzz Corpus Validator CI
Integration Authorization Plan`.

## Live NA-0493 scope

Allowed corpus paths:

- `qsl/qsl-client/qsc/fuzz/corpus/qsc_binding_semantics/seed_00_a1_mutation.bin`
- `qsl/qsl-client/qsc/fuzz/corpus/qsc_binding_semantics/seed_01_b1_mutation.bin`
- `qsl/qsl-client/qsc/fuzz/corpus/qsc_binding_semantics/seed_02_a2_mutation.bin`
- `qsl/qsl-client/qsc/fuzz/corpus/qsc_binding_semantics/seed_03_suite_confusion.bin`
- `qsl/qsl-client/qsc/fuzz/corpus/qsc_binding_semantics/seed_04_replay.bin`
- `qsl/qsl-client/qsc/fuzz/corpus/qsc_binding_semantics/seed_05_stale_public_record.bin`
- `qsl/qsl-client/qsc/fuzz/corpus/qsc_binding_semantics/seed_ff_vector_traceability.bin`

Allowed governance paths:

- `docs/governance/evidence/NA-0493_qsl_binding_fuzz_checked_in_corpus_implementation_harness.md`
- `tests/NA-0493_qsl_binding_fuzz_checked_in_corpus_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

All other qsl-protocol paths were read-only for implementation.

## qwork proof-file verification

Codex did not run qwork, qstart, or qresume. It read:

- `/srv/qbuild/work/NA-0493/.qwork/startup.qsl-protocol.kv`
- `/srv/qbuild/work/NA-0493/.qwork/startup.qsl-protocol.json`

The proofs recorded `startup_result=OK`, lane `NA-0493`, repo
`qsl-protocol`, path `/srv/qbuild/work/NA-0493/qsl-protocol`,
`head_equals_origin_main=yes`, clean worktree/index/untracked state,
`ready_count=1`, `queue_top_ready=NA-0493`, and
`requested_lane_status=READY`. JSON mirrored the required fields.

Proof HEAD and proof `origin/main` matched live local refs before fetch. Fetch
occurred only after that match. Startup `origin/main` was `8fb19091060e` and
descended from the required base.

## NA-0492 / D360 inheritance

NA-0492/D360 selected
`BINDING_FUZZ_MINIMAL_CORPUS_IMPLEMENTATION_READY`.

Inherited future corpus path:
`qsl/qsl-client/qsc/fuzz/corpus/qsc_binding_semantics/`.

Inherited exact seed list:

- `seed_00_a1_mutation.bin`
- `seed_01_b1_mutation.bin`
- `seed_02_a2_mutation.bin`
- `seed_03_suite_confusion.bin`
- `seed_04_replay.bin`
- `seed_05_stale_public_record.bin`
- `seed_ff_vector_traceability.bin`

D360 selected exactly seven raw binary files, max 64 bytes per file,
public/synthetic content only, and mandatory validator gating before commit. No
corpus README or metadata file was selected. Traceability must live in
governance evidence, not in corpus files.

D360 selected no qsc source mutation, no qsc fuzz target mutation, no Cargo or
lockfile mutation, no script/workflow/dependency mutation, no vector/input
mutation outside the exact corpus path, and no formal/refimpl/service/public or
backup mutation. D359's proof-output stop was recovered by D360. Startup disk
status for NA-0493 was below the 95% stop threshold.

No public-readiness claim is made. no crypto-complete claim is made. no
fuzz-complete claim is made. no corpus-complete claim is made. no
vector-complete claim is made. no replay-proof claim is made. no downgrade-proof
claim is made. no side-channel-free claim is made. no vulnerability-free claim
is made. no bug-free claim is made. no perfect-crypto claim is made.

## Pre-mutation corpus review

Before mutation,
`qsl/qsl-client/qsc/fuzz/corpus/qsc_binding_semantics/` was absent.

Existing qsc fuzz corpus directories were:

- `qsc_payload_boundaries`: 5 files.
- `qsc_route_http`: 3 files.
- `qsc_vault_envelope`: 2 files.

The current qsc fuzz corpus validator scan passed before mutation. All seven
selected future seed paths were absent. The tracked diff and untracked inventory
were empty before mutation.

Read-only selector review found:

- first byte `0x00` maps to A1 mutation.
- first byte `0x01` maps to B1 mutation.
- first byte `0x02` maps to A2 mutation.
- first byte `0x03` maps to suite confusion.
- first byte `0x04` maps to replay.
- first byte `0x05` maps to stale public-record / trusted-pin.
- first byte `0xff` is special-cased by the fuzz target as vector
  traceability.

## Corpus implementation

Created directory:
`qsl/qsl-client/qsc/fuzz/corpus/qsc_binding_semantics/`.

Created exactly seven raw binary seed files. Each file is eight bytes. The bytes
are selector-first and include only short synthetic byte values. No text label,
README, metadata file, manifest JSON, private key marker, passphrase, token,
runtime key, backup key, production endpoint, user/operator data, or qsc secret
filename was added to the corpus.

## Corpus path proof

The corpus directory contains only the seven selected `.bin` files. Proof root
records:

- `corpus/file_sizes_sorted.txt`
- `corpus/file_count.txt`
- `corpus/sha256_sorted.txt`
- `corpus/file_command.txt`
- `corpus/exact_set_guard.txt`

## Seven-file proof

Recorded file sizes:

- `seed_00_a1_mutation.bin`: 8 bytes.
- `seed_01_b1_mutation.bin`: 8 bytes.
- `seed_02_a2_mutation.bin`: 8 bytes.
- `seed_03_suite_confusion.bin`: 8 bytes.
- `seed_04_replay.bin`: 8 bytes.
- `seed_05_stale_public_record.bin`: 8 bytes.
- `seed_ff_vector_traceability.bin`: 8 bytes.

The exact-set guard passed with count 7 and max size 8.

## Category mapping proof

The corpus uses these selector mappings:

- `seed_00_a1_mutation.bin`: first byte `0x00`, A1 mutation.
- `seed_01_b1_mutation.bin`: first byte `0x01`, B1 mutation.
- `seed_02_a2_mutation.bin`: first byte `0x02`, A2 mutation.
- `seed_03_suite_confusion.bin`: first byte `0x03`, suite confusion.
- `seed_04_replay.bin`: first byte `0x04`, replay.
- `seed_05_stale_public_record.bin`: first byte `0x05`, stale
  public-record / trusted-pin.
- `seed_ff_vector_traceability.bin`: first byte `0xff`, direct
  vector-traceability selector in the fuzz target.

No qsc binding fuzz target code was modified.

## Size proof

Every seed is 8 bytes, below the 64-byte limit. No extra corpus file was added.
No corpus README or metadata file was added.

## Validator-gate proof

Commands run after adding the corpus:

```bash
python3 scripts/audit/validate_binding_fuzz_corpus_no_secrets.py --format json --path qsl/qsl-client/qsc/fuzz/corpus/qsc_binding_semantics
python3 scripts/audit/validate_binding_fuzz_corpus_no_secrets.py --format json --path qsl/qsl-client/qsc/fuzz/corpus
```

Both exited 0. New corpus summary: 7 files scanned, 56 bytes scanned, finding
count 0, result `pass`. Full qsc fuzz corpus summary: finding count 0, result
`pass`. No `--allow-missing` was needed after the corpus existed.

## Deterministic validator JSON proof

The validator was run twice on
`qsl/qsl-client/qsc/fuzz/corpus/qsc_binding_semantics`. The two JSON outputs
were byte-for-byte identical.

Proof root records:

- `validator/qsc_binding_semantics_corpus.json`
- `validator/qsc_binding_semantics_corpus_second.json`
- `validator/qsc_binding_semantics_cmp.out`

## No secret/private material proof

The dedicated validator found zero findings. The corpus content scan recorded no
claim text and no public/conformance/interoperability wording inside the raw
files. The files contain short selector/synthetic bytes only.

No private key, KEM secret key, signing secret key, identity secret key,
passphrase, token, runtime key, backup/recovery key, production endpoint,
operator data, user data, qsc secret filename, or internal negative vector
manifest content was added.

## No qsc source/fuzz target/Cargo/script/workflow mutation proof

The implementation diff is limited to the seven selected corpus files plus the
allowed governance paths. There is no mutation under qsc source, qsc fuzz target
code, qsc fuzz Cargo metadata, qsc fuzz `Cargo.lock`, root `Cargo.lock`,
qsc-adversarial scripts, workflows, dependencies, vectors/inputs outside the
exact corpus path, formal, refimpl, service, public-doc, website, backup, or
qwork tooling.

## Validation

Local validation completed before PR:

- validator py_compile.
- validator on new corpus and all qsc fuzz corpus.
- internal negative vector manifest JSON validation.
- formal binding model and formal runner.
- cfg-on fuzz target cargo check.
- qsc binding negative tests with and without `qsc_binding_fuzz_helper`.
- refimpl signature provider-boundary test.
- refimpl `pqkem768`.
- qsc key lifecycle zeroization.
- qsc provider-error no-mutation.
- root cargo audit.
- nested qsc fuzz lock audit.
- cargo fmt.
- qsc-adversarial shell syntax under `sh -n` and `bash -n`.

Local cargo-fuzz was unavailable on PATH, so local one-run fuzz execution was
not feasible. PR `qsc-adversarial-smoke` remains the required cargo-fuzz-backed
CI evidence.

## Scope guard

Allowed changed paths are the seven selected corpus files plus:

- `docs/governance/evidence/NA-0493_qsl_binding_fuzz_checked_in_corpus_implementation_harness.md`
- `tests/NA-0493_qsl_binding_fuzz_checked_in_corpus_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

No file move, archive, or deletion was performed.

## Backup-impact statement

No backup or restore command was run. `/usr/local/sbin/qsl-backup` was checked
read-only and matched the expected installed-helper digest prefix. The Codex ops
source appeared exactly once in that helper. No backup plan, backup status,
rollback, qsl-backup, `/backup/qsl`, or archived-path state was mutated.

## Successor selection

Selected successor:
`NA-0494 -- QSL Binding Fuzz Corpus Validator CI Integration Authorization Plan`.

Rationale: the validator exists and the minimal checked-in corpus now exists.
The next safety hardening question is whether and how the validator should be
integrated into CI/public-safety/qsc-adversarial gates. Workflow/script changes
require a separate authorization lane.

## Next recommendation

Merge NA-0493 only after PR public-safety and qsc-adversarial-smoke are green.
If post-merge public-safety and qsc-adversarial-smoke are green, close out
NA-0493 and restore NA-0494 as authorization-only work. Do not implement NA-0494
inside this lane.
