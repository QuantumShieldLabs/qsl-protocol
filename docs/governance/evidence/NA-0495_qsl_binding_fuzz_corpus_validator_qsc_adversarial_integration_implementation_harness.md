Status: Supporting
Owner: QSL Director
Last-Updated: 2026-06-18

# NA-0495 QSL Binding Fuzz Corpus Validator qsc-Adversarial Integration Implementation Harness

## Executive summary

NA-0495 consumes NA-0494/D365 authorization and integrates the existing
binding fuzz corpus secret-material validator into
`scripts/ci/qsc_adversarial.sh`.

The script now runs the validator before cargo-fuzz target execution, scans the
checked-in `qsc_binding_semantics` corpus and the full qsc fuzz corpus, rejects
missing binding corpus by default, and fails closed on validator findings. The
validator script, workflow files, qsc source, qsc fuzz target code, corpus
files, Cargo metadata, lockfiles, dependencies, formal models, refimpl,
services, public docs, backup paths, and qwork tooling remain unchanged.

## Live NA-0495 scope

Allowed implementation path:

- `scripts/ci/qsc_adversarial.sh`

Allowed governance paths:

- `docs/governance/evidence/NA-0495_qsl_binding_fuzz_corpus_validator_qsc_adversarial_integration_implementation_harness.md`
- `tests/NA-0495_qsl_binding_fuzz_corpus_validator_qsc_adversarial_integration_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

No workflow, helper, validator, corpus/vector/input, qsc source, qsc fuzz
target, qsc fuzz Cargo, qsc fuzz lockfile, root lockfile, dependency, formal,
refimpl, service, public-doc, backup, qsl-backup, qwork, qstart, qresume,
archive, move, or delete mutation is part of NA-0495.

## qwork proof-file verification

Codex did not run qwork, qstart, or qresume. It read and copied:

- `/srv/qbuild/work/NA-0495/.qwork/startup.qsl-protocol.kv`
- `/srv/qbuild/work/NA-0495/.qwork/startup.qsl-protocol.json`

The proof files recorded `startup_result=OK`, lane `NA-0495`, repo
`qsl-protocol`, path `/srv/qbuild/work/NA-0495/qsl-protocol`,
`head_equals_origin_main=yes`, clean worktree/index/untracked state,
`ready_count=1`, `queue_top_ready=NA-0495`, and
`requested_lane_status=READY`. JSON mirrored the required fields.

Proof `HEAD` and proof `origin/main` matched live refs before fetch:
`ba7791206e9c`. Fetch occurred only after that match. `origin/main` equaled
`ba7791206e9c` and descended from `ba7791206e9c`.

Startup disk status: `/` was 94% used, below the 95% stop threshold;
`/backup/qsl` was 11% used.

## NA-0494 / D365 inheritance

NA-0494/D365 selected qsc-adversarial integration as the first validator CI
integration. Local/testplan-only gating was rejected as insufficient because it
does not automatically block PR regressions.

The public-safety helper and workflow integration were deferred because they
would broaden helper or workflow behavior. A standalone CI job was rejected for
the first integration because it would mutate workflows. JSON artifact upload
was deferred to a later workflow-authorized lane.

The inherited future NA-0495 path was `scripts/ci/qsc_adversarial.sh`. NA-0495
had to scan both `qsl/qsl-client/qsc/fuzz/corpus/qsc_binding_semantics` and
`qsl/qsl-client/qsc/fuzz/corpus`, fail closed on validator findings, reject a
missing binding corpus, and avoid workflow, dependency, helper, validator,
corpus, source, fuzz target, and Cargo mutation.

At startup, current main public-safety and qsc-adversarial-smoke were green.
The checked-in binding corpus existed with exactly seven seed files, each
8 bytes. The validator passed the binding corpus and all qsc fuzz corpus.

## Pre-mutation review

Preimage was recorded for `scripts/ci/qsc_adversarial.sh`. The script contained
no validator integration and no `--allow-missing` use.

Existing qsc-adversarial behavior before mutation:

- Stable Rust adversarial tests: `adversarial_properties` and
  `adversarial_miri`.
- Provider-error no-mutation step:
  `NA0439_QSC_PROVIDER_ERROR_NO_MUTATION_ADVERSARIAL_STEP` and
  `handshake_provider_error_no_mutation`.
- Cargo-fuzz targets: `qsc_route_http`, `qsc_payload_boundaries`,
  `qsc_vault_envelope`, and `qsc_binding_semantics`.
- Existing marker: `NA0487_FUZZ_CI_ADVERSARIAL_TARGET_INCLUDED_OK`.

Pre-mutation validator proof passed:

- Binding corpus: 7 files, 56 bytes, zero findings.
- Full qsc fuzz corpus: 17 files, 1238 bytes, zero findings.

## qsc-adversarial validator integration implementation

The script now defines repo-relative paths:

- `VALIDATOR="scripts/audit/validate_binding_fuzz_corpus_no_secrets.py"`
- `BINDING_CORPUS="${FUZZ_DIR}/corpus/qsc_binding_semantics"`
- `ALL_QSC_CORPUS="${FUZZ_DIR}/corpus"`

The new `run_binding_fuzz_corpus_validator` function runs:

- `python3 "${VALIDATOR}" --format text --path "${BINDING_CORPUS}"`
- `python3 "${VALIDATOR}" --format text --path "${ALL_QSC_CORPUS}"`

The function is called after the stable adversarial/provider-error tests and
before any `run_fuzz_target` call. With `set -eu`, any validator finding or
missing required path exits nonzero and stops the script.

Required markers are present:

- `NA0495_VALIDATOR_CI_SCOPE_CONSUMED_OK`
- `NA0495_VALIDATOR_QSC_ADVERSARIAL_STEP_INCLUDED_OK`
- `NA0495_VALIDATOR_FAILS_ON_FINDINGS_OK`
- `NA0495_VALIDATOR_SCANS_BINDING_CORPUS_OK`
- `NA0495_VALIDATOR_SCANS_ALL_QSC_CORPUS_OK`
- `NA0495_NO_WORKFLOW_CHANGE_OK`
- `NA0495_NO_DEPENDENCY_CHANGE_OK`
- `NA0495_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0495_NO_CRYPTO_COMPLETE_CLAIM_OK`
- `NA0495_NO_FUZZ_COMPLETE_CLAIM_OK`
- `NA0495_NO_CORPUS_COMPLETE_CLAIM_OK`
- `NA0495_NO_VECTOR_COMPLETE_CLAIM_OK`

## Validator scans binding corpus proof

Command:

```bash
python3 scripts/audit/validate_binding_fuzz_corpus_no_secrets.py --format text --path qsl/qsl-client/qsc/fuzz/corpus/qsc_binding_semantics
```

Result: pass, 7 files scanned, 56 bytes scanned, zero findings.

## Validator scans all corpus proof

Command:

```bash
python3 scripts/audit/validate_binding_fuzz_corpus_no_secrets.py --format text --path qsl/qsl-client/qsc/fuzz/corpus
```

Result: pass, 17 files scanned, 1238 bytes scanned, zero findings.

## Fail-closed proof

A proof-root-only fixture containing a configured disallowed label marker was
created under the NA-0495 proof root. Direct validator text and JSON scans both
exited 2 with one redacted finding.

The qsc-adversarial script uses the same validator command shape under
`set -eu`, so validator findings stop the script before cargo-fuzz targets run.

## Missing binding corpus reject proof

The repo corpus was not removed. Direct validator execution against a
proof-root missing path without `--allow-missing` exited 2 with a redacted
`missing_path` finding.

Static script proof shows `--allow-missing` is absent and the binding corpus is
validated through `BINDING_CORPUS`, so a missing checked-in binding corpus fails
the qsc-adversarial path.

## No workflow / dependency / helper / validator mutation proof

Changed-path scope is limited to:

- `scripts/ci/qsc_adversarial.sh`
- `docs/governance/evidence/NA-0495_qsl_binding_fuzz_corpus_validator_qsc_adversarial_integration_implementation_harness.md`
- `tests/NA-0495_qsl_binding_fuzz_corpus_validator_qsc_adversarial_integration_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

No workflow, helper, validator script, corpus/vector/input, qsc source, qsc fuzz
target, qsc fuzz Cargo, qsc fuzz lockfile, root lockfile, dependency, formal,
refimpl, service, public-doc, backup, qsl-backup, qwork, qstart, qresume,
archive, move, or delete path is changed.

## Existing qsc-adversarial behavior preservation proof

The existing stable Rust tests remain before cargo-fuzz execution. The
provider-error no-mutation step and marker remain unchanged. The existing
cargo-fuzz target order and the `qsc_binding_semantics` target-specific
`RUSTFLAGS` behavior remain unchanged.

Local script execution reached and passed the new validator step, printed all
NA-0495 markers, and then reached the expected local `cargo fuzz` unavailable
boundary on this host. PR qsc-adversarial-smoke remains the required
cargo-fuzz-backed evidence before merge.

## Validation

Required local validation includes:

- `sh -n scripts/ci/qsc_adversarial.sh`
- `bash -n scripts/ci/qsc_adversarial.sh`
- validator binding corpus text and JSON scans
- validator all qsc fuzz corpus text and JSON scans
- proof-root fail-closed fixture rejection
- proof-root missing binding corpus rejection
- qsc-adversarial local prerequisite execution through the validator step
- manifest JSON validation
- formal model checks
- qsc binding negative tests with and without `qsc_binding_fuzz_helper`
- refimpl signature provider-boundary and `pqkem768`
- root and nested cargo audit
- cargo fmt
- link-check, leak-scan, classifier, PR body preflight, goal-lint, and scope
  guard

Local cargo-fuzz is unavailable on this host, so PR qsc-adversarial-smoke must
be green before merge.

## Scope guard

The implementation PR must contain exactly the six allowed paths listed in the
scope section. It must contain no workflow, validator, corpus, vector/input,
qsc source/fuzz target/Cargo/lockfile, dependency, formal, refimpl, service,
public, backup, qsl-backup, qwork/qstart/qresume, move, archive, or deletion
mutation.

## Public claim boundary

No public-readiness claim is made. no production-readiness claim is made.
no public-internet-readiness claim is made. no external-review-complete claim
is made. no crypto-complete claim is made. no fuzz-complete claim is made.
no corpus-complete claim is made. no validator-complete claim beyond bounded
internal evidence is made. no vector-complete claim is made. no replay-proof
claim is made. no downgrade-proof claim is made. no side-channel-free claim is
made. no vulnerability-free claim is made. no bug-free claim is made. no perfect-crypto claim is made. Cargo audit green is dependency-health evidence only.

## Backup-impact statement

No backup or restore was run. `qsl-backup` was read only. The installed helper
matched expected digest prefix `e9ecff3d22ed`, and the Codex ops source
appeared exactly once in the installed helper source list. NA-0495 does not
mutate backup status, backup plan, rollback state, nightly/local-ops scripts,
or `/backup/qsl`.

## Successor selection

Selected successor after successful NA-0495:

`NA-0496 -- QSL Binding Negative Vector Consumer Test Scope Authorization Plan`

Rationale: after validator implementation, checked-in corpus, and
qsc-adversarial validator integration, the next strongest deterministic
assurance residual is authorizing tests that consume or map the internal
negative binding vector manifest. NA-0495 does not implement NA-0496.

## Next recommendation

Merge NA-0495 only after required checks pass, especially public-safety and
qsc-adversarial-smoke. If post-merge public-safety is green, close out NA-0495
and restore the selected NA-0496 authorization lane without implementing it.
