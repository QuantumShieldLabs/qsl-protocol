Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-29

# NA-0382 QSL Local Ops Directive Manifest and Allow-File Harness

Goals: G1, G2, G3, G4, G5

## Executive Summary

NA-0382 adds a standalone qsl-protocol local-ops validator at
`scripts/ci/qsl_directive_manifest_validate.py` plus fixture-first coverage
under `inputs/local_ops/directive_manifest_fixtures/` and
`inputs/local_ops/scope_allow_file_fixtures/`.

The helper validates machine-readable directive manifests, scope allow-files,
changed paths, and helper-compatible emitted scope files without mutating
`qsl_evidence_helper.py`, `qsl_bounded_check_poll.py`,
`public_safety_gate.py`, workflows, dependencies, runtime code, qsl-server,
qsl-attachments, backup configuration, or local history archives.

## Live NA-0382 Scope

Live `NEXT_ACTIONS.md` records:

- READY_COUNT `1`.
- READY `NA-0382 -- QSL Local Ops Directive Manifest and Allow-File Implementation Harness`.
- NA-0381 DONE.
- D-0744 exists once.
- D-0745 exists once.
- D-0746 absent at startup.
- public-safety remains required and green.

Authorized durable paths are the validator, fixture directories, this evidence
file, the NA-0382 testplan, `DECISIONS.md`, `TRACEABILITY.md`, and
`docs/ops/ROLLING_OPERATIONS_JOURNAL.md`.

## Inherited NA-0381 Authorization

NA-0381 authorized a future standalone Python standard-library validator at
`scripts/ci/qsl_directive_manifest_validate.py` with tracked fixtures under:

- `inputs/local_ops/directive_manifest_fixtures/`
- `inputs/local_ops/scope_allow_file_fixtures/`

NA-0381 explicitly avoided direct `qsl_evidence_helper.py` mutation and
recommended temporary helper-compatible output for later scope-guard use.

## Helper Path

Helper:

`scripts/ci/qsl_directive_manifest_validate.py`

Supported commands:

- `validate-manifest`
- `validate-allow-file`
- `validate`
- `emit-scope-files`
- `fixture`

The helper uses Python standard library modules only and performs no network,
GitHub, branch, workflow, dependency, runtime, or backup operation.

## Manifest Schema

The accepted schema version is `qsl.directive_manifest.v1`, following the live
NA-0382 directive. This intentionally supersedes the NA-0381 planning note that
described an integer schema version.

Required top-level fields are:

- `schema_version`
- `directive_id`
- `target_na`
- `title`
- `expected_origin_main`
- `prior_response_path`
- `mutable_repos`
- `read_only_repos`
- `allowed_paths`
- `forbidden_paths`
- `allowed_local_paths`
- `forbidden_local_paths`
- `required_checks`
- `required_evidence_files`
- `forbidden_operations`
- `public_claim_boundaries`
- `backup_impact_expected`
- `operator_input_required`
- `packet_plan`
- `implementation_paths`
- `temporary_artifact_paths`
- `closeout_successor`
- `stop_conditions`
- `response_file_expected`
- `history_read_only_paths`

Unknown top-level keys fail closed. Missing required fields fail closed.
Malformed JSON fails closed. Secret-shaped values fail closed.

## Allow-File Schema

Allow-files are UTF-8 text:

- blank lines are ignored;
- comments beginning `#` are ignored;
- unprefixed lines are exact repo-relative paths;
- `glob:` lines are accepted only for scoped non-broad globs;
- `local:` lines require manifest authorization;
- parent traversal, ambiguous normalization, absolute repo paths, repo-wide
  globs, and hidden recursive repo-wide globs fail closed.

Forbidden overlay precedence is enforced during changed-path validation:
forbidden wins even when another rule allows the same path.

## Fixture Matrix and Markers

Fixture matrix proof log:

`/srv/qbuild/tmp/NA0382_manifest_allow_file_20260529T143345-0500/fixture_matrix.log`

The matrix has 34 cases, all passing locally. Markers include:

- `NA0382_DIRECTIVE_MANIFEST_AUTHORIZATION_OK`
- `NA0382_ALLOW_FILE_AUTHORIZATION_OK`
- `NA0382_MANIFEST_VALIDATION_OK`
- `NA0382_ALLOW_FILE_VALIDATION_OK`
- `NA0382_NEXT_ACTIONS_CONFLICT_FAILS_CLOSED_OK`
- `NA0382_EXPECTED_MAIN_MISMATCH_FAILS_CLOSED_OK`
- `NA0382_FORBIDDEN_PATH_REJECT_OK`
- `NA0382_UNLISTED_PATH_REJECT_OK`
- `NA0382_PARENT_TRAVERSAL_REJECT_OK`
- `NA0382_BROAD_GLOB_REJECT_OK`
- `NA0382_FORBIDDEN_OVERLAY_WINS_OK`
- `NA0382_SUCCESSOR_MISMATCH_REJECT_OK`
- `NA0382_PUBLIC_CLAIM_BOUNDARY_REQUIRED_OK`
- `NA0382_NO_WORKFLOW_CHANGE_OK`
- `NA0382_NO_DEPENDENCY_CHANGE_OK`
- `NA0382_NO_RUNTIME_CHANGE_OK`
- `NA0382_NO_SECRET_MATERIAL_OK`
- `NA0382_METADATA_RUNTIME_MANIFEST_ALLOW_FILE_HARNESS_OK`

## Positive Cases

Positive fixtures prove:

- valid manifest validates;
- valid allow-file validates;
- valid changed paths validate;
- exact path is accepted;
- comments and blanks are accepted;
- helper-compatible scope files are emitted only under `/srv/qbuild/tmp`.

## Negative / Fail-Closed Cases

Negative fixtures prove rejection of:

- malformed JSON;
- missing schema version;
- unknown top-level key;
- missing or wrong directive ID;
- missing or wrong target NA;
- missing or wrong expected origin/main;
- missing prior response path;
- missing required checks;
- missing public-claim boundary;
- missing stop conditions;
- successor mismatch;
- secret-shaped value;
- operator-input mismatch;
- missing forbidden operation;
- malformed allow-file line;
- parent traversal;
- absolute path without `local:`;
- local path without manifest authorization;
- broad glob;
- hidden repo-wide glob;
- path normalization ambiguity;
- forbidden changed path;
- unlisted changed path;
- forbidden overlay over an allowed path.

## Integration Proof with Existing Scope Guard

`emit-scope-files` writes:

- `allowed_paths.txt`
- `forbidden_paths.txt`
- `scope_summary.json`

The live smoke emitted to
`/srv/qbuild/tmp/NA0382_scope_emit_20260529T142826-0500/` and
`scripts/ci/qsl_evidence_helper.py scope-guard` accepted the emitted files.
This proves integration without changing `qsl_evidence_helper.py`.

## No-Mutation / No-Network / No-Secret Proof

The helper has no `gh api`, network, branch, push, merge, workflow, dependency,
runtime, qsl-server, qsl-attachments, backup, restore, or secret-handling
operation. The fixture matrix includes `NA0382_NO_SECRET_MATERIAL_OK`.

## Backup-Plan Impact

No backup-plan update is required for NA-0382 because durable artifacts are
tracked qsl-protocol files and proof logs / emitted scope files stay temporary
under `/srv/qbuild/tmp`. The same-host continuity limitation remains unchanged.

## Runtime / Service / Dependency / Workflow Boundary

NA-0382 changes no runtime/service/protocol/crypto code, no Cargo files, no
dependencies, no workflows, no public-safety gate, no qsl bounded polling
helper, and no qsl evidence helper.

## qsl-server / qsl-attachments Boundary

qsl-server PR #56 remains read-only bounded harness evidence at
`d40e6003fdf0`. qsl-attachments PR #37 remains read-only service-local
prerequisite evidence at `96b9352bd63e`. Neither repository is mutated.

## Public-Claim Boundary

This lane does not prove production readiness or public internet readiness, and
it makes no external review completion, anonymity, metadata-free messaging,
untraceability, off-host backup completion, disaster recovery completion,
target setup, host identity verification, real restore completion, or key
custody/recovery claim.

## Selected Successor

Selected successor:

`NA-0383 -- QSL Local Ops Response Writer Implementation Authorization Plan`

Rationale: the directive manifest / allow-file harness is now the first
workflow-support primitive; the response writer remains the next bounded
local-ops authorization plan and is not implemented here.

## Rejected Alternatives

- Modifying `qsl_evidence_helper.py` directly now.
- Shell-only validation.
- Manual-only validation.
- Workflow changes.
- New dependencies.
- Runtime/service changes.

## Next Recommendation

After NA-0382 merges and post-merge public-safety is green, optionally close
NA-0382 and restore NA-0383 as the sole READY successor.
