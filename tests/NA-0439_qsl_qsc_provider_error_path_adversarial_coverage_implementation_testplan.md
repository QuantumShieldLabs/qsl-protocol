# NA-0439 qsc Provider Error Path Adversarial Coverage Implementation Testplan

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-07

Goals: G1, G2, G3, G4, G5

## Objective

Validate that NA-0439 adds the existing
`handshake_provider_error_no_mutation` test to the qsc adversarial script,
preserves the `pq_encap_failed` caveat, and changes only the exact authorized
script plus NA-0439 governance evidence paths.

## Protected invariants

- READY_COUNT remains 1.
- READY remains NA-0439 until optional closeout.
- D-0865 exists once after this lane.
- D-0866 remains absent until optional closeout.
- The new script step runs before cargo-fuzz targets.
- The new provider-error test does not silently skip.
- `pq_decap_failed` evidence remains bounded to the NA-0436 no-mutation test.
- `pq_encap_failed` remains defensive-branch documentation only.
- No executable coverage claim is made for `pq_encap_failed`.
- No runtime, crypto, dependency, Cargo, lockfile, workflow, executable test,
  fuzz target, vector, service, public-surface, or backup/local-ops path is
  changed.
- No public claim expansion is introduced.

## Allowed scope

- `scripts/ci/qsc_adversarial.sh`
- `docs/governance/evidence/NA-0439_qsl_qsc_provider_error_path_adversarial_coverage_implementation_harness.md`
- `tests/NA-0439_qsl_qsc_provider_error_path_adversarial_coverage_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Forbidden scope

Forbidden changed paths include runtime, crypto, dependency, Cargo manifest,
lockfile, workflow, executable test, fuzz target, vector, qsl-server,
qsl-attachments, qshield runtime, website, public docs, README, START_HERE,
qwork/qstart/qresume/qshell, qsl-backup, backup status, backup plan, rollback
subtree, and backup tree paths.

Forbidden commands include qwork, qstart, qresume, sudo, backup, restore,
`cargo update`, `cargo generate-lockfile`, dependency remediation commands, and
workflow/branch-protection mutation.

## Queue and decision checks

Run:

```bash
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
```

Required:

- READY_COUNT 1.
- READY NA-0439.
- NA-0438 DONE.
- NA-0437 DONE.
- NA-0436 DONE.
- NA-0435 DONE.
- NA-0434 BLOCKED.
- NA-0433 DONE.
- NA-0432 DONE.
- NA-0431 DONE.
- NA-0430 DONE.
- NA-0429 BLOCKED.
- D-0863 exists once.
- D-0864 exists once.
- D-0865 exists once after patching.
- D-0866 absent.
- duplicate decision count zero.

## Script checks

Run:

```bash
git diff -- scripts/ci/qsc_adversarial.sh
sh -n scripts/ci/qsc_adversarial.sh
bash -n scripts/ci/qsc_adversarial.sh
```

Required:

- Diff adds `NA0439_QSC_PROVIDER_ERROR_NO_MUTATION_ADVERSARIAL_STEP`.
- Diff adds the provider-error no-mutation test command before the first
  `run_fuzz_target` call.
- Shell syntax checks pass.
- File mode is preserved.

## Provider-error test validation

Run:

```bash
cargo test -p qsc --locked --test handshake_provider_error_no_mutation -- --test-threads=1 --nocapture
```

Required:

- Test passes.
- Markers include:
  - `NA0436_PQ_DECAP_FAILED_MARKER_OK`
  - `NA0436_NO_SESSION_MUTATION_ON_DECAP_REJECT_OK`
  - `NA0436_PENDING_STORE_NO_MUTATION_ON_DECAP_REJECT_OK`
  - `NA0436_PQ_ENCAP_FAILED_CAVEAT_PRESERVED_OK`
  - `NA0436_NO_RUNTIME_HOOK_USED_OK`

## Local validation commands

Run:

```bash
sh -n scripts/ci/qsc_adversarial.sh
bash -n scripts/ci/qsc_adversarial.sh
cargo test -p qsc --locked --test handshake_provider_error_no_mutation -- --test-threads=1 --nocapture
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
cargo test -p quantumshield_refimpl --features pqcrypto --locked --test pqkem768
cargo audit --deny warnings
cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock
cargo tree -i rustls-webpki --locked
cargo tree -i ml-kem --locked || true
cargo tree -i pqcrypto-mlkem --locked || true
cargo tree -i pqcrypto-traits --locked || true
cargo tree -i pqcrypto-internals --locked || true
cargo fmt --check
python3 formal/model_qsc_handshake_suite_id_bounded.py
python3 formal/run_model_checks.py
```

Required:

- Syntax checks pass.
- Direct provider-error test passes.
- qsc `send_commit` passes.
- provider `pqkem768` passes.
- root audit is green.
- nested qsc fuzz lock audit is green.
- `rustls-webpki` is v0.103.13 or newer safe version.
- root pqcrypto inverse probes show package-ID absence or documented absence.
- nested qsc fuzz lock residual scan shows zero pqcrypto matches.
- fmt and formal checks pass.

## Local qsc adversarial script validation

Run:

```bash
if [ -x scripts/ci/qsc_adversarial.sh ]; then
  scripts/ci/qsc_adversarial.sh
else
  sh scripts/ci/qsc_adversarial.sh
fi
```

Required:

- Existing stable Rust adversarial phases pass.
- `NA0439_QSC_PROVIDER_ERROR_NO_MUTATION_ADVERSARIAL_STEP` appears.
- `handshake_provider_error_no_mutation` runs and passes before cargo-fuzz.
- If local cargo-fuzz is unavailable, record the exact output and proceed only
  if all pre-fuzz phases passed and PR CI `qsc-adversarial-smoke` is required
  before merge.
- If the script fails before the provider-error test runs, restore from
  rollback and stop.

## PR and CI validation

Before merge:

- `public-safety` must complete success.
- `qsc-adversarial-smoke` must complete success.
- `qsc-adversarial-miri` must complete success if attached/required by the
  check shape.
- No required context may fail.

Do not merge if PR `qsc-adversarial-smoke` fails.

## Scope guard

Run:

```bash
git diff --name-only origin/main
git ls-files --others --exclude-standard
```

Allowed changed paths:

- `scripts/ci/qsc_adversarial.sh`
- `docs/governance/evidence/NA-0439_qsl_qsc_provider_error_path_adversarial_coverage_implementation_harness.md`
- `tests/NA-0439_qsl_qsc_provider_error_path_adversarial_coverage_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Required: no other changed or untracked paths.

## Link, leak, and overclaim checks

Run the repository manual link-integrity check for relative markdown links.

Run added-line leak and overclaim scans. Required:

- `TOTAL_MISSING 0`
- no secret findings
- no affirmative public-readiness claim
- no affirmative production-readiness claim
- no affirmative external-review claim
- no affirmative crypto-complete claim
- no affirmative side-channel-free claim
- no affirmative bug-free claim
- no affirmative vulnerability-free claim
- no affirmative perfect-crypto claim
- no executable coverage claim for `pq_encap_failed`

## Backup boundary

Verify:

```bash
sha256sum /usr/local/sbin/qsl-backup
grep -F -c '/home/victor/work/qsl/codex/ops' /usr/local/sbin/qsl-backup
```

Required:

- qsl-backup SHA remains
  `e9ecff3d22eda21ceb0e889e4dd5d6f4e270e09349c77a1f4872bfc0052f6232`.
- Source inclusion count remains `1`.
- No backup or restore is run.
- No qsl-backup, backup status, backup plan, rollback subtree, or backup tree
  path is mutated.

## Acceptance criteria

NA-0439 is acceptable only if:

- the adversarial harness scope is implemented only in
  `scripts/ci/qsc_adversarial.sh`;
- the new provider-error test runs before cargo-fuzz;
- direct provider-error test passes;
- local script proves the new test runs before any local cargo-fuzz
  unavailability failure;
- PR `qsc-adversarial-smoke` passes before merge;
- `pq_encap_failed` caveat is preserved;
- `pq_decap_failed` test evidence is consumed without overclaim;
- root and nested audits are green;
- no forbidden paths are mutated;
- no public overclaim is introduced;
- exactly one READY item remains.
