Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-17

# NA-0491 Binding Fuzz Corpus Secret-Material Validator Implementation Harness

Goals: G1, G2, G3, G4, G5

## Executive summary

NA-0491 consumes NA-0490/D356 and implements the dependency-free repo-local
validator selected by D-0969:

`scripts/audit/validate_binding_fuzz_corpus_no_secrets.py`

The validator scans supplied corpus paths recursively, defaults to
`qsl/qsl-client/qsc/fuzz/corpus`, emits deterministic JSON when requested,
redacts findings, rejects disallowed secret-material classes, accepts the
current qsc fuzz corpus, and allows the absent binding corpus only under an
explicit `--allow-missing` policy.

No checked-in corpus is added. No vector or input file is mutated. No qsc
source, qsc fuzz target, qsc fuzz Cargo file, qsc fuzz lockfile,
qsc-adversarial script, workflow, dependency, lockfile, formal model, refimpl,
service, public-doc, backup, qsl-backup, qwork, qstart, qresume, or qshell path
is mutated.

Future checked-in binding corpus remains blocked until a later directive
authorizes exact corpus scope and requires the validator gate. Cargo audit
green remains dependency-health evidence only. This evidence is bounded
internal engineering evidence only.

## Live NA-0491 scope

Startup READY item:

`NA-0491 -- QSL Binding Fuzz Corpus Secret-Material Validator Implementation Harness`

Allowed mutation paths used by this implementation PR:

- `scripts/audit/validate_binding_fuzz_corpus_no_secrets.py`
- this evidence doc
- `tests/NA-0491_qsl_binding_fuzz_corpus_secret_material_validator_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Forbidden and not performed:

- checked-in corpus mutation
- vector/input mutation
- qsc source mutation
- qsc fuzz target mutation
- qsc fuzz Cargo or lockfile mutation
- qsc-adversarial script mutation
- workflow mutation
- dependency or lockfile mutation
- formal/refimpl/service/public/qshield/qsl-server/qsl-attachments mutation
- backup/restore/qsl-backup mutation
- qwork/qstart/qresume/qshell mutation
- file move/archive/delete
- no public-readiness claim, no crypto-complete claim, no fuzz-complete claim,
  no corpus-complete claim, no vector-complete claim, no replay-proof claim,
  no downgrade-proof claim, no side-channel-free claim, no vulnerability-free
  claim, no bug-free claim, and no perfect-crypto claim

## qwork proof-file verification

Codex read the operator-provided qwork proof files and did not run qwork,
qstart, or qresume:

- `/srv/qbuild/work/NA-0491/.qwork/startup.qsl-protocol.kv`
- `/srv/qbuild/work/NA-0491/.qwork/startup.qsl-protocol.json`

Required qwork fields matched:

- `startup_result=OK`
- `lane=NA-0491`
- `repo=qsl-protocol`
- `path=/srv/qbuild/work/NA-0491/qsl-protocol`
- `head_equals_origin_main=yes`
- `worktree_clean=yes`
- `index_clean=yes`
- `untracked_clean=yes`
- `ready_count=1`
- `queue_top_ready=NA-0491`
- `requested_lane_status=READY`

Freshness proof:

- proof HEAD and live pre-fetch HEAD both matched `08f54515306b`
- proof `origin/main` and live pre-fetch `origin/main` both matched
  `08f54515306b`
- fetch was performed only after proof/live refs matched
- `origin/main` equals or descends from `08f54515306b`
- clean `main` was checked against `origin/main`

Startup queue and decision proof:

- READY_COUNT 1
- READY NA-0491
- NA-0490 DONE
- NA-0489 DONE
- NA-0488 DONE
- NA-0487 DONE
- D-0969 exists once
- D-0970 exists once
- D-0971 absent before this patch
- duplicate decision count 0

Startup health proof:

- public-safety on `08f54515306b`: success
- qsc-adversarial-smoke on `08f54515306b`: success
- `/` usage: 89 percent, below the 95 percent hard stop threshold
- `/backup/qsl` usage: 11 percent
- installed `/usr/local/sbin/qsl-backup` SHA256 matched the expected digest
- installed qsl-backup source inclusion count for the Codex ops source was
  exactly 1

## NA-0490 / D356 inheritance

Consumed inheritance sources:

- D356 response:
  `/home/victor/work/qsl/codex/responses/NA0490_20260617T031514Z_D356.md`
- `docs/governance/evidence/NA-0490_qsl_binding_fuzz_corpus_secret_material_validator_authorization_plan.md`
- `tests/NA-0490_qsl_binding_fuzz_corpus_secret_material_validator_authorization_testplan.md`
- D-0969 and D-0970 in `DECISIONS.md`
- NA-0491 block in `NEXT_ACTIONS.md`

Inherited selections consumed:

- `BINDING_FUZZ_CORPUS_VALIDATOR_SCRIPT_READY`
- `SECRET_PATTERN_MATRIX_READY`
- `VALIDATOR_NO_DEPENDENCY_NO_WORKFLOW_READY`
- `VALIDATOR_WORKFLOW_INTEGRATION_LATER` preserved
- future checked-in binding corpus remains blocked until validator passes
- validator script path selected:
  `scripts/audit/validate_binding_fuzz_corpus_no_secrets.py`
- no dependency/workflow change selected
- no corpus/vector/input mutation selected
- current disk pressure status: `/` 89 percent, `/backup/qsl` 11 percent

## Pre-mutation review

Preimage state:

- `scripts/audit/validate_binding_fuzz_corpus_no_secrets.py` was absent.
- Existing `scripts/audit/` files were `README.md`, `audit_pr_delta.sh`, and
  `run_goal_lint_pr.sh`.
- Existing audit helper style is fail-closed, local-first, and writes audit
  artifacts outside the repo.
- Existing `scripts/ci/qsl_evidence_helper.py leak-scan` redacts findings and
  catches common private-key/token patterns, but it does not provide
  corpus-specific label rules, high-entropy policy, deterministic corpus JSON,
  or missing-corpus policy.
- Existing checked-in qsc fuzz corpus directories were
  `qsc_payload_boundaries`, `qsc_route_http`, and `qsc_vault_envelope`.
- Existing checked-in qsc fuzz corpus file count was 10.
- No `qsl/qsl-client/qsc/fuzz/corpus/qsc_binding_semantics/` directory
  existed.
- Internal negative vector manifest existed at
  `inputs/suite2/internal_negative_binding_vectors/qsl_binding_negative_vector_manifest_v1.json`.

## Validator script implementation

Implemented:

- Python 3 standard library only.
- CLI supports repeated `--path PATH`.
- CLI supports `--paths PATH ...`.
- CLI defaults to `qsl/qsl-client/qsc/fuzz/corpus` when no path is supplied.
- CLI supports `--format json` and `--format text`.
- CLI supports explicit `--allow-missing`.
- Recursive directory scan uses deterministic sorting.
- Symlinks are not followed.
- Missing paths are rejected unless `--allow-missing` is supplied.
- JSON output has no timestamps and uses sorted keys with deterministic
  separators.
- Findings include path, kind, severity, byte offset or line number, redacted
  context hash, and `redaction=[redacted]`.
- The validator emits no matched payload bytes.
- Exit code is 0 for no findings and nonzero for findings.

Required implementation markers are present in the script and emitted in JSON:

- `NA0491_VALIDATOR_SCOPE_CONSUMED_OK`
- `NA0491_SECRET_PATTERN_MATRIX_READY_OK`
- `NA0491_VALIDATOR_SCRIPT_IMPLEMENTED_OK`
- `NA0491_VALIDATOR_REJECTS_PRIVATE_KEY_MARKERS_OK`
- `NA0491_VALIDATOR_REJECTS_SECRET_LABELS_OK`
- `NA0491_VALIDATOR_REJECTS_HIGH_ENTROPY_UNALLOWLISTED_OK`
- `NA0491_VALIDATOR_ALLOWS_SYNTHETIC_PUBLIC_BYTES_OK`
- `NA0491_VALIDATOR_REPORTS_DETERMINISTIC_JSON_OK`
- `NA0491_NO_DEPENDENCY_CHANGE_OK`
- `NA0491_NO_WORKFLOW_CHANGE_OK`
- `NA0491_NO_CORPUS_MUTATION_OK`
- `NA0491_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0491_NO_CRYPTO_COMPLETE_CLAIM_OK`
- `NA0491_NO_FUZZ_COMPLETE_CLAIM_OK`
- `NA0491_NO_CORPUS_COMPLETE_CLAIM_OK`
- `NA0491_NO_VECTOR_COMPLETE_CLAIM_OK`

## Secret pattern matrix implementation

Reject classes implemented:

- PEM private-key marker fragments
- OpenSSH private-key marker fragments
- age/minisign private-key style markers
- obvious API token labels
- common token value shapes
- passphrase labels
- KEM secret-key names
- signature secret-key names
- identity secret-key names
- backup/recovery key labels
- runtime/service secret labels
- private endpoint / production-like identifier markers
- qsc vault secret filename/path labels, while preserving the existing
  `qsc_vault_envelope` fuzz corpus path
- user/operator data markers
- unallowlisted high-entropy encoded-looking printable spans

Allow/safe behavior proven:

- short synthetic byte arrays pass
- mutated public-message-like bytes pass
- vector IDs pass
- manifest category names pass
- non-secret labels pass
- expected reject strings pass
- small structured metadata passes
- arbitrary short binary fuzz bytes are not rejected merely because they are
  binary

High-entropy policy:

- v1 flags encoded-looking printable spans above configured length and entropy
  thresholds.
- v1 uses conservative thresholds to avoid false positives on the existing
  parser/envelope corpus.
- v1 does not treat short binary corpus bytes as secret merely because they are
  binary.

## Redaction proof

Proof artifacts:

- `/srv/qbuild/tmp/NA0491_corpus_secret_material_validator_impl_20260617T102138Z/validator/private_marker.json`
- `/srv/qbuild/tmp/NA0491_corpus_secret_material_validator_impl_20260617T102138Z/validator/secret_label.json`
- `/srv/qbuild/tmp/NA0491_corpus_secret_material_validator_impl_20260617T102138Z/validator/high_entropy.json`

Redaction result:

- Reject outputs include finding kind, severity, path, offset/line, short
  digest, and `[redacted]`.
- Reject outputs do not include the synthetic marker payload.
- Reject outputs do not include the secret-label fixture payload prose.
- Reject outputs do not include the high-entropy encoded fixture span.

## Safe synthetic corpus proof

Fixture root:

`/srv/qbuild/tmp/NA0491_corpus_secret_material_validator_impl_20260617T102138Z/fixtures/safe/`

Safe fixture contents:

- short synthetic byte file
- mutated public-message-like byte file
- JSON metadata with vector IDs, category names, non-secret labels, and expected
  reject strings

Validation result:

- command: validator `--format json --path <safe fixture>`
- exit: 0
- result: pass
- findings: 0
- proof:
  `/srv/qbuild/tmp/NA0491_corpus_secret_material_validator_impl_20260617T102138Z/validator/safe.json`

## Private-key marker reject proof

Fixture root:

`/srv/qbuild/tmp/NA0491_corpus_secret_material_validator_impl_20260617T102138Z/fixtures/private_marker/`

Validation result:

- command: validator `--format json --path <private-marker fixture>`
- exit: 2
- result: fail
- finding kinds: `private_key_marker`
- output redacted
- proof:
  `/srv/qbuild/tmp/NA0491_corpus_secret_material_validator_impl_20260617T102138Z/validator/private_marker.json`

## Secret-label reject proof

Fixture root:

`/srv/qbuild/tmp/NA0491_corpus_secret_material_validator_impl_20260617T102138Z/fixtures/secret_label/`

Validation result:

- command: validator `--format json --path <secret-label fixture>`
- exit: 2
- result: fail
- findings: 9
- finding kinds: `api/token-adjacent secret labels were not needed for this
  fixture`; `passphrase_label`, `kem_secret_key_label`,
  `signature_secret_key_label`, `identity_secret_key_label`,
  `backup_recovery_key_label`, `runtime_service_secret_label`,
  `private_endpoint_marker`, `operator_user_data_marker`, and
  `qsc_secret_filename_marker`
- output redacted
- proof:
  `/srv/qbuild/tmp/NA0491_corpus_secret_material_validator_impl_20260617T102138Z/validator/secret_label.json`

## High-entropy encoded text reject proof

Fixture root:

`/srv/qbuild/tmp/NA0491_corpus_secret_material_validator_impl_20260617T102138Z/fixtures/high_entropy/`

Validation result:

- command: validator `--format json --path <high-entropy fixture>`
- exit: 2
- result: fail
- finding kinds: `high_entropy_encoded_span`
- output redacted
- proof:
  `/srv/qbuild/tmp/NA0491_corpus_secret_material_validator_impl_20260617T102138Z/validator/high_entropy.json`

## Existing corpus pass proof

Command:

`python3 scripts/audit/validate_binding_fuzz_corpus_no_secrets.py --format json --path qsl/qsl-client/qsc/fuzz/corpus`

Result:

- exit: 0
- result: pass
- files scanned: 10
- bytes scanned: 1182
- findings: 0
- proof:
  `/srv/qbuild/tmp/NA0491_corpus_secret_material_validator_impl_20260617T102138Z/validator/current_corpus.json`

## Missing binding corpus allow-missing proof

Command:

`python3 scripts/audit/validate_binding_fuzz_corpus_no_secrets.py --format json --allow-missing --path qsl/qsl-client/qsc/fuzz/corpus/qsc_binding_semantics`

Result:

- exit: 0
- result: pass
- path status: `missing_allowed`
- findings: 0
- proof:
  `/srv/qbuild/tmp/NA0491_corpus_secret_material_validator_impl_20260617T102138Z/validator/missing_binding_allow.json`

Negative policy proof:

- the same missing path without `--allow-missing` exited 2
- proof:
  `/srv/qbuild/tmp/NA0491_corpus_secret_material_validator_impl_20260617T102138Z/validator/missing_binding_no_allow.json`

## Deterministic JSON proof

The validator was run twice on the same secret-label fixture. The JSON outputs
were byte-identical.

Proof:

- first output:
  `/srv/qbuild/tmp/NA0491_corpus_secret_material_validator_impl_20260617T102138Z/validator/deterministic_a.json`
- second output:
  `/srv/qbuild/tmp/NA0491_corpus_secret_material_validator_impl_20260617T102138Z/validator/deterministic_b.json`
- compare exit: 0

## No dependency / no workflow proof

Implementation proof:

- The validator imports only Python standard library modules.
- No `Cargo.toml`, lockfile, dependency manifest, workflow, or
  qsc-adversarial script is mutated.
- `cargo audit --deny warnings`: PASS.
- `cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock`:
  PASS.
- Cargo audit green is dependency-health evidence only.

## No corpus/vector/input mutation proof

No checked-in corpus/vector/input path is changed.

The existing checked-in corpus remains:

- `qsc_payload_boundaries`: 5 files
- `qsc_route_http`: 3 files
- `qsc_vault_envelope`: 2 files
- no checked-in `qsc_binding_semantics` corpus

The proof fixtures live only under:

`/srv/qbuild/tmp/NA0491_corpus_secret_material_validator_impl_20260617T102138Z/fixtures/`

## Validation

Local validation results:

- `python3 -m py_compile scripts/audit/validate_binding_fuzz_corpus_no_secrets.py`: PASS with pycache redirected outside the repo
- validator help: PASS
- validator safe fixture: PASS
- validator private-marker reject fixture: expected nonzero PASS
- validator secret-label reject fixture: expected nonzero PASS
- validator high-entropy reject fixture: expected nonzero PASS
- existing qsc fuzz corpus validator scan: PASS
- missing binding corpus with `--allow-missing`: PASS
- missing binding corpus without `--allow-missing`: expected nonzero PASS
- deterministic JSON compare: PASS
- internal negative vector manifest JSON validation: PASS
- `python3 formal/model_qsc_kem_signature_transcript_binding_bounded.py`: PASS
- `python3 formal/run_model_checks.py`: PASS
- qsc binding negative test without cfg: PASS
- qsc binding negative test with `qsc_binding_fuzz_helper`: PASS
- refimpl signature provider-boundary test: PASS
- refimpl `pqkem768` test: PASS
- qsc key lifecycle zeroization test: PASS
- qsc provider-error no-mutation test: PASS
- root cargo audit: PASS
- nested qsc fuzz lock audit: PASS
- `cargo fmt --check`: PASS
- `sh -n scripts/ci/qsc_adversarial.sh`: PASS
- `bash -n scripts/ci/qsc_adversarial.sh`: PASS
- local `sh scripts/ci/qsc_adversarial.sh`: reached expected local
  cargo-fuzz unavailable boundary after adversarial Rust prerequisite tests
  passed

Recovered-failure evidence:

- Failing command: direct `scripts/ci/qsc_adversarial.sh` invocation.
  Classification: recoverable command-shape issue because the script is not
  executable in the local checkout. Corrective action: reran the same local
  validation through `sh scripts/ci/qsc_adversarial.sh`. Final result:
  prerequisite Rust adversarial tests passed and the script reached the
  expected local `cargo fuzz` unavailable boundary.

## Scope guard

Expected implementation PR changed paths:

- `scripts/audit/validate_binding_fuzz_corpus_no_secrets.py`
- `docs/governance/evidence/NA-0491_qsl_binding_fuzz_corpus_secret_material_validator_implementation_harness.md`
- `tests/NA-0491_qsl_binding_fuzz_corpus_secret_material_validator_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

No corpus/vector/input mutation, no qsc source/fuzz target/Cargo/script
mutation beyond the new validator script, no workflow/dependency/lockfile
mutation, no formal/refimpl/service/public/backup mutation, no qsl-backup
mutation, and no qwork/qstart/qresume/qshell mutation is introduced.

## Backup-impact statement

No backup or restore command was run. No qsl-backup, backup plan, backup
status, rollback, archive, `/backup/qsl`, or local-ops/nightly script path is
mutated. The installed qsl-backup helper hash and source inclusion boundary
were verified read-only at startup.

## Applicable stewardship and assurance review

Level-1 stewardship conclusion:

- The implementation stays inside the exact NA-0491 validator-script scope.
- Lead Director authority and exactly-one-READY queue discipline remain
  unchanged.
- Stewards remain advisory only.

Best-Known-Method Review:

- A dedicated standard-library validator is the narrowest durable gate before
  future checked-in corpus work.
- Deterministic redacted JSON makes future corpus evidence reproducible without
  exposing matched payloads.

Hostile Cryptographer Review:

- The validator fails closed on marker, label, filename/path, endpoint, user or
  operator data, and high-entropy classes that could smuggle private material
  into corpus files.
- High-entropy policy remains conservative and corpus-specific; if future
  corpus files need encoded public material, a later allowlist design must be
  explicitly authorized.

Red-Team Review:

- Proof-root fixtures verify safe, private-marker, secret-label, and
  high-entropy cases without adding repo corpus files or printing payloads.

Production SRE Review:

- No workflow/dependency integration is introduced in NA-0491.
- `VALIDATOR_WORKFLOW_INTEGRATION_LATER` remains preserved for a later exact
  directive.

Side-Channel Caveat:

- no side-channel-free claim is made.

Formal-Model Mapping Residual:

- The validator governs corpus hygiene only and does not replace formal model
  checks.

External-Review Readiness:

- no external-review-complete claim is made.

Release-Claim Boundary:

- no public-readiness claim is made and no crypto-complete claim is made.

Assurance Gap Review Trigger:

- Future checked-in corpus work remains separately authorized and validator
  gated.

## Successor selection

Selected successor after successful NA-0491:

`NA-0492 -- QSL Binding Fuzz Checked-In Corpus Scope Authorization Plan`

Rationale:

- NA-0488/NA-0490 required a validator before any checked-in binding corpus.
- NA-0491 implements and proves that validator locally.
- The next safe lane is exact corpus scope authorization, not direct corpus
  implementation.
- `VALIDATOR_WORKFLOW_INTEGRATION_LATER` remains a separate possible future
  lane, but it does not block the next authorization-only corpus scope review.

## Next recommendation

After implementation PR merge and post-merge public-safety success, close out
NA-0491 and restore the selected NA-0492 authorization lane without
implementing NA-0492.
