Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-29

# NA-0384 QSL Local Ops Response Writer Harness

Goals: G1, G2, G3, G4, G5

## Executive Summary

NA-0384 adds a standalone qsl-protocol response writer harness for local Codex
operations. The helper writes only response-format files under explicitly
supplied `/srv/qbuild/tmp` directories, validates metadata and body headings
before write, rejects high-confidence secret patterns before write, preserves
no-overwrite collision behavior, and provides dry-run, validate-only, template,
fixture, and JSON summary modes.

Classification:

`NA0384_RESPONSE_WRITER_HELPER_OK`

This is a temp-output harness only. It does not authorize writes to
`/home/victor/work/qsl/codex/responses`, create response indexes, create
history indexes, change backup configuration, change workflows, change
dependencies, or change runtime/service/protocol/crypto code.

## Live NA-0384 Scope

Live `NEXT_ACTIONS.md` records:

- READY_COUNT `1`.
- READY `NA-0384 -- QSL Local Ops Response Writer Implementation Harness`.
- NA-0383 DONE.
- D-0748 exists once.
- D-0749 exists once.
- D-0750 absent at startup.
- public-safety remains required and green.

Allowed NA-0384 implementation/evidence paths:

- `scripts/ci/qsl_codex_response_writer.py`
- `inputs/local_ops/response_writer_fixtures/`
- `docs/governance/evidence/NA-0384_qsl_local_ops_response_writer_harness.md`
- `tests/NA-0384_qsl_local_ops_response_writer_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Inherited NA-0383 Authorization

NA-0383 authorized this lane in D-0748 and restored it as the sole READY
successor in D-0749. The inherited expected helper path was
`scripts/ci/qsl_codex_response_writer.py`, and the inherited fixture path was
`inputs/local_ops/response_writer_fixtures/`.

NA-0383 explicitly rejected direct real-archive writing, response index
creation, shell-only implementation, extending `qsl_evidence_helper.py`,
changing workflows, changing dependencies, and changing backup configuration.

## Helper Path

Helper:

- `scripts/ci/qsl_codex_response_writer.py`

The helper uses only Python standard-library modules. It contains no network
calls, no GitHub calls, no branch mutation, no deletion path, no overwrite path,
and no imports from the shared qsl local-ops helpers.

## Metadata Schema

Required metadata schema:

- `schema_version`: `qsl.codex_response_writer.metadata.v1`
- `target_na`
- `directive_suffix`
- `directive_id`
- `response_start_local`
- `response_start_utc`
- `directive_begin_local`
- `directive_begin_utc`
- `directive_end_local`
- `directive_end_utc`
- `timezone`
- `timezone_offset`
- `output_mode`
- `required_sections`
- `allow_real_archive_output`
- `no_secret_required`

The helper also accepts optional paired `response_end_local` and
`response_end_utc` fields for deterministic wrapped-output fixtures.

Validation is fail-closed for malformed JSON, unknown metadata fields, invalid
target NA, invalid directive suffix, invalid directive ID, invalid timestamp,
UTC/local timestamp mismatch, invalid timezone offset, non-`America/Chicago`
timezone, non-`temp-output` output mode, `allow_real_archive_output=true`,
`no_secret_required=false`, and missing baseline required headings.

## Output Contract

Filename format:

`NAxxxx_<YYYYMMDD>T<HHMMSS><timezone-offset>_Dnnn.md`

Collision behavior:

- base filename first;
- then `_r2`;
- then `_r3`;
- continuing monotonically;
- extension remains `.md`;
- exclusive create is used for the final write.

Wrapper:

- `CODEX RESPONSE BEGIN`
- response start timestamp lines
- directive begin timestamp lines
- directive ID line
- body sections
- response end timestamp lines
- directive end timestamp lines
- `CODEX RESPONSE END`

Write mode creates only the explicitly supplied authorized temp output
directory if needed. Dry-run and validate-only modes write no response file.

## Fixture Matrix and Markers

Fixture directory:

- `inputs/local_ops/response_writer_fixtures/`

Representative proof log:

- `/srv/qbuild/tmp/NA0384_response_writer_20260529T223436-0500_preflight/fixture_matrix.log`

Markers proven by fixture mode:

- `NA0384_RESPONSE_WRITER_AUTHORIZATION_OK`
- `NA0384_RESPONSE_WRITER_HELPER_OK`
- `NA0384_RESPONSE_FILENAME_OK`
- `NA0384_RESPONSE_COLLISION_R2_OK`
- `NA0384_RESPONSE_COLLISION_R3_OK`
- `NA0384_RESPONSE_WRAPPER_OK`
- `NA0384_REQUIRED_SECTIONS_OK`
- `NA0384_DRY_RUN_NO_WRITE_OK`
- `NA0384_VALIDATE_ONLY_NO_WRITE_OK`
- `NA0384_NO_OVERWRITE_OK`
- `NA0384_SECRET_PATTERN_REJECT_OK`
- `NA0384_JSON_SUMMARY_OK`
- `NA0384_TEMP_OUTPUT_BOUNDARY_OK`
- `NA0384_REAL_ARCHIVE_WRITE_BLOCKED_OK`
- `NA0384_BACKUP_IMPACT_OK`
- `NA0384_NO_INDEX_MUTATION_OK`
- `NA0384_NO_WORKFLOW_CHANGE_OK`
- `NA0384_NO_DEPENDENCY_CHANGE_OK`
- `NA0384_NO_RUNTIME_CHANGE_OK`
- `NA0384_NO_SECRET_MATERIAL_OK`
- `NA0384_METADATA_RUNTIME_RESPONSE_WRITER_HARNESS_OK`
- `NA0384_NO_METADATA_FREE_CLAIM_OK`
- `NA0384_NO_ANONYMITY_CLAIM_OK`
- `NA0384_NO_UNTRACEABLE_CLAIM_OK`
- `NA0384_NO_PRODUCTION_READY_CLAIM_OK`
- `NA0384_NO_PUBLIC_INTERNET_READY_CLAIM_OK`

## Positive Cases

Fixture mode proves:

- valid minimal response write under `/srv/qbuild/tmp`;
- valid full response write under `/srv/qbuild/tmp`;
- valid stop-reason body;
- false-positive-safe policy text accepted;
- wrapper generation;
- baseline and full required-section enforcement;
- deterministic filename;
- `_r2` collision selection;
- `_r3` collision selection;
- dry-run no-write;
- validate-only no-write;
- JSON summary serialization.

## Negative / Fail-Closed Cases

Fixture mode proves rejection of:

- malformed metadata JSON;
- missing target NA;
- invalid target NA;
- missing directive suffix;
- invalid directive suffix;
- invalid timestamp;
- invalid timezone offset;
- missing required section;
- missing wrapper-relevant timestamp metadata;
- unauthorized output directory;
- requested real response archive output;
- collision with `--no-collision`;
- high-confidence secret sentinel.

## Temp-Output Smoke Proof

Live smoke used:

- temp root: `/srv/qbuild/tmp/NA0384_response_writer_smoke_20260529T222715-0500`
- dry-run JSON summary: PASS
- validate-only JSON summary: PASS
- write JSON summary under temp output: PASS
- real archive output attempt to `/home/victor/work/qsl/codex/responses`: rejected with exit code `2`
- response archive `NA0384*_D203.md` count before smoke: `0`
- response archive `NA0384*_D203.md` count after smoke: `0`

## No-Mutation / No-Network / No-Secret Proof

The helper has no network or GitHub call sites and no branch mutation call
sites. It writes only the selected response file under an authorized temp
output directory in write mode. Fixture mode writes only under its supplied
`/srv/qbuild/tmp` directory, including `fixture_matrix.log`.

The helper scans metadata and body text before write for high-confidence private
key, token, credential label, recovery-envelope marker, raw-credential marker,
and `QSL_TEST_FORBIDDEN_SECRET_SENTINEL` patterns. It reports only pattern
categories and does not print matched secret content.

## Archive-Output Rejection Proof

The helper rejects:

- any output directory at or below `/home/victor/work/qsl/codex/responses`;
- any metadata with `output_mode` other than `temp-output`;
- any metadata with `allow_real_archive_output=true`.

NA-0384 did not use the helper to write to the real response archive.

## Backup-Plan Impact

No backup-plan update is required for NA-0384 because durable files are tracked
qsl-protocol helper, fixture, governance, testplan, traceability, and journal
paths, while proof logs and smoke outputs remain under `/srv/qbuild/tmp`.

Real response archive writes remain gated for NA-0385 or later because they
would affect `/home/victor/work/qsl/codex/responses` durability, backup
coverage, local-history boundaries, no-secret posture, and no-overwrite
operations.

Same-host `/backup/qsl` continuity remains not complete disaster recovery.

## Runtime / Service / Dependency / Workflow Boundary

NA-0384 changes no runtime/service/protocol/crypto files, no `.github/**`
workflows, no `Cargo.toml`, no `Cargo.lock`, and no dependency files. It does
not mutate `qsl_evidence_helper.py`, `qsl_bounded_check_poll.py`,
`qsl_directive_manifest_validate.py`, or `public_safety_gate.py`.

## qsl-server / qsl-attachments Boundary

Read-only startup proof confirmed:

- qsl-server PR #56 merged at `d40e6003fdf0`.
- qsl-attachments PR #37 merged at `96b9352bd63e`.

No qsl-server or qsl-attachments repository was cloned or mutated.

## Public-Claim Boundary

This lane preserves the following prohibited-claim boundaries:

- no production deployment readiness claim;
- no public-internet readiness claim;
- no external-review-complete claim;
- no metadata-free claim;
- no anonymity claim;
- no untraceable behavior claim;
- no off-host backup completion claim;
- no disaster recovery completion claim;
- no qsl-server production proof claim;
- no qsl-attachments production proof claim;
- no website readiness claim;
- no local-ops completion claim.

## Selected Successor

Selected successor:

`NA-0385 -- QSL Local Ops Response Archive Backup Coverage / Real-Archive Write Authorization Plan`

Rationale: NA-0384 proves only temp-output response writer mechanics. Real
response archive writes require an explicit backup coverage, no-secret,
no-overwrite, and local-history authorization plan before durable archive use.

## Rejected Alternatives

- Writing the real response archive now.
- Creating response indexes now.
- Creating directive or journal indexes now.
- Extending `qsl_evidence_helper.py`.
- Extending `qsl_bounded_check_poll.py`.
- Extending `qsl_directive_manifest_validate.py`.
- Changing workflows.
- Adding dependencies.
- Using a shell-only writer.

## Next Recommendation

Proceed to NA-0385 only after NA-0384 merge and closeout. NA-0385 should decide
whether and how real archive writes may be authorized, including backup
coverage, no-secret, no-overwrite, and local-history boundaries. It should not
retroactively treat temp-output proof as durable archive authorization.
