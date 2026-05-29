Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-29

# NA-0381 QSL Local Ops Directive Manifest and Allow-File Implementation Authorization

Goals: G1, G2, G3, G4, G5

## Executive Summary

NA-0381 authorizes the next bounded local-ops lane to build a qsl-protocol directive manifest and scope allow-file validation harness. This is authorization and planning evidence only. NA-0381 does not add manifest parser code, allow-file parser code, generated manifests, generated allow-files, workflow changes, dependency changes, runtime changes, public-safety changes, or `qsl_evidence_helper.py` changes.

Selected successor:

`NA-0382 -- QSL Local Ops Directive Manifest and Allow-File Implementation Harness`

Recommended first implementation shape:

- standalone Python standard-library validator at `scripts/ci/qsl_directive_manifest_validate.py`;
- tracked fixture directories under `inputs/local_ops/directive_manifest_fixtures/` and `inputs/local_ops/scope_allow_file_fixtures/`;
- no workflow, dependency, runtime, service, public docs, backup-script, or local-history mutation;
- integration with existing `scripts/ci/qsl_evidence_helper.py scope-guard` by producing a helper-compatible temporary pattern file after stricter manifest/allow-file validation, not by mutating `qsl_evidence_helper.py` in NA-0381.

Authorization classification:

`DIRECTIVE_MANIFEST_ALLOW_FILE_IMPLEMENTATION_AUTHORIZATION_READY`

## Live NA-0381 Scope

Live `NEXT_ACTIONS.md` records:

- READY_COUNT `1`.
- READY `NA-0381 -- QSL Local Ops Directive Manifest and Allow-File Implementation Authorization Plan`.
- NA-0380 DONE.
- D-0742 once.
- D-0743 once.
- D-0744 absent at start.
- public-safety remains required and green.

Allowed NA-0381 mutable paths:

- `docs/governance/evidence/NA-0381_qsl_local_ops_directive_manifest_allow_file_implementation_authorization.md`
- `tests/NA-0381_qsl_local_ops_directive_manifest_allow_file_implementation_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Forbidden NA-0381 scope:

- manifest parser, allow-file parser, helper implementation, generated manifest, generated allow-file, workflow, dependency, Cargo, runtime, protocol, crypto, auth, state-machine, qshield runtime, qsl-server, qsl-attachments, qsc-desktop, website, public docs, README, START_HERE, qstart/qresume, backup script/timer/fstab, local backup source list, remote/off-host setup, restore, deploy, rollback, secret/key/credential handling, and public-claim expansion.

## Inherited NA-0380 Result

NA-0380 delivered the bounded CI/public-safety polling helper harness in qsl-protocol PR #1023 and closeout PR #1024. Inherited proof:

- PR #1023 merge: `678995bac98e`.
- PR #1024 merge and current `origin/main`: `2503a46a2be5`.
- Helper: `scripts/ci/qsl_bounded_check_poll.py`.
- Fixtures: `inputs/local_ops/qsl_bounded_check_poll_fixtures/`.
- Fixture proof log: `/srv/qbuild/tmp/NA0380_bounded_check_poll_20260528T225422-0500/fixture_matrix.log`.
- D-0742 records the polling helper implementation decision.
- D-0743 records closeout and restores NA-0381.

NA-0380 intentionally did not mutate `scripts/ci/public_safety_gate.py`, `scripts/ci/qsl_evidence_helper.py`, workflows, dependencies, runtime, qsl-server, qsl-attachments, qshield runtime, website/public docs, backup scripts/timers/fstab, or local history indexes.

## Audit Report Intake and Relevance Mapping

Both NA-0380 audit report checksums matched.

- Overall project audit: `/srv/qbuild/tmp/NA0380_post_completion_audit_20260529T005653-0500/NA0380_overall_project_readonly_audit.md`
- Overall SHA-256: `66dd26c0b35b97113f160e4dd67fdc9992bd3be91c72452359fbef74dcef0913`
- Code/crypto audit: `/srv/qbuild/tmp/NA0380_post_completion_audit_20260529T005653-0500/NA0380_code_crypto_readonly_audit.md`
- Code/crypto SHA-256: `70c21179e7a57dd168dff77e2d5bb18ac2ad1c7c285b216da7875ca712d1c099`

| Finding | Severity | NA-0381 direct handling | Later lane | Schema effect | Backup effect | Public-claim effect |
|---|---|---|---|---|---|---|
| GOV-001 local history backup coverage | MEDIUM | Records local/durable storage boundary | yes | manifest must classify local paths separately | durable local manifests outside repo need backup review | prevents same-host continuity overclaim |
| GOV-002 broad validation expense | MEDIUM | defines validation profile fields | yes | `required_checks` must support bounded profiles | none for tracked fixtures | avoids full-suite overclaim |
| GOV-003 response/journal indexing/manual handoff | MEDIUM | manifest includes response/history fields | yes | `response_file_expected` and `history_read_only_paths` required | local archive outputs need backup review | avoids local-ops-complete claim |
| GOV-004 distributed CodeQL/docs-only policy | LOW | codifies required check/policy fields | yes | explicit check acceptance policy required | none | avoids hidden CI downgrade |
| GOV-006 public position paper evidence map | OPPORTUNITY | keeps public claims gated | yes | `public_claim_boundaries` required | none | no public-claim expansion |
| CC-001 duplicate crypto dependency review | MEDIUM | carry-forward only | yes | no dependency schema change | none | no crypto-readiness overclaim |
| CC-002 validation timeout | MEDIUM | validation behavior requires bounded profiles | yes | timeout/profile fields required | none | evidence-incomplete retained |
| CC-003 test-only lint cleanup | LOW | carry-forward only | yes | no schema change | none | no broad-clippy overclaim |
| CC-004 panic-demo non-production boundary | MEDIUM | carry-forward only | yes | `public_claim_boundaries` must preserve non-production wording | none | no runtime-readiness overclaim |
| CC-005 refimpl actor deterministic RNG/nonce boundary | MEDIUM | carry-forward only | yes | manifest must not classify harness proof as production proof | none | no external-review overclaim |
| CC-007 fuzz/property/formal gap opportunity | OPPORTUNITY | marker plan includes future validation evidence | yes | validation fields support future fixtures | none | no complete-proof claim |

## Existing Scope-Guard / qsl_evidence_helper / Manifest Surface Discovery

Read-only discovery found:

- `scripts/ci/qsl_evidence_helper.py scope-guard` already supports `--allowed-file` and `--forbidden-file`.
- Current pattern-file semantics ignore blank/comment lines, then treat plain entries as exact-or-prefix patterns and glob metacharacters through `fnmatch`.
- Current scope guard does not enforce parent-traversal rejection, absolute-path rejection, exact-only default semantics, broad-glob rejection, schema versioning, manifest identity checks, public-claim boundaries, backup-impact declarations, or required-check policy.
- `scripts/ci/qsl_bounded_check_poll.py` is standalone and standard-library only; it should be consumed by future validation/PR flow, not changed by NA-0381.
- `scripts/ci/classify_ci_scope.sh` classifies docs-only/workflow/runtime scope, but it is not a directive manifest validator.
- `tools/goal_lint.py` validates PR body goals and core-change discipline, but not directive scope.
- Existing `inputs/local_ops/qsl_bounded_check_poll_fixtures/*.json` provide a good model for deterministic local fixture testing.

Classifications:

- `MANIFEST_IMPLEMENTATION_AUTHORITY_CLEAR`
- `MANIFEST_SCHEMA_READY`
- `ALLOW_FILE_SCOPE_GUARD_INTEGRATION_READY`
- `QSL_EVIDENCE_HELPER_CHANGE_REQUIRED`: no for NA-0382 standalone first lane
- `QSL_EVIDENCE_HELPER_CHANGE_AVOIDABLE`: yes
- `IMPLEMENTATION_READY`: yes, for the exact future NA-0382 harness only
- `IMPLEMENTATION_BLOCKED`: no

## Directive Manifest Schema Design

Format: JSON.

Schema version: integer `schema_version`, initial value `1`.

Top-level required keys:

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

Optional keys:

- `referenced_artifacts` with `path`, `sha256`, and `required` fields.
- `validation_profiles` for named bounded command groups.
- `allow_file_paths` for tracked fixture allow-files or operator-supplied allow-files.
- `notes` for non-authoritative human context.

Forbidden key classes:

- secret, token, password, passphrase, private key, auth header, credential, known_hosts content, branch-protection override, admin bypass, direct push, force push, delete branch, squash, rebase, deploy, restore, remote target setup, and any key that instructs a tool to weaken fail-closed behavior.

Path entry shape for `allowed_paths`, `forbidden_paths`, `allowed_local_paths`, and `forbidden_local_paths`:

```json
{
  "path": "docs/governance/evidence/example.md",
  "mode": "exact",
  "reason": "governance evidence"
}
```

Allowed `mode` values:

- `exact`: repo-relative exact path only.
- `glob`: explicit glob only, accepted only after broad-glob checks.
- `local_exact`: local absolute path, accepted only under `allowed_local_paths` and only when a future directive authorizes local mutation.

Required identity behavior:

- `directive_id` must match the active directive.
- `target_na` must match live `NEXT_ACTIONS.md`.
- `expected_origin_main` must match live `git rev-parse origin/main`.
- `mutable_repos` must be exactly the repos authorized by live directive scope.
- manifest conflicts with `NEXT_ACTIONS.md`, branch protection, public-safety, or live repo state fail closed.

Validation order:

1. Parse JSON and reject malformed input.
2. Validate schema version and reject unknown top-level keys.
3. Run no-secret scan over manifest text and referenced allow-files.
4. Validate directive identity, target NA, origin/main, and prior response path.
5. Validate mutable/read-only repo declarations.
6. Normalize and validate repo paths, local paths, forbidden operations, and public-claim boundaries.
7. Validate allow-file syntax if referenced.
8. Compute changed paths and apply forbidden overlay before allowed paths.
9. Validate required checks, evidence files, response path, backup impact, and closeout successor.
10. Emit human-readable and machine-readable summaries.

Failure behavior:

- exit nonzero;
- print exact failing field/path/check where safe;
- do not mutate files, branch protection, PRs, workflows, or repositories;
- do not continue to later validation stages when an earlier identity, path, secret, or authority check fails.

## Allow-File Schema Design

Format: UTF-8 line-based text.

Rules:

- one entry per line;
- blank lines ignored;
- comments begin with `#` after optional leading whitespace;
- default entries are exact repo-relative paths;
- explicit globs require `glob:` prefix;
- explicit local paths require `local:` prefix and future directive authorization;
- absolute paths are rejected unless `local:` is used and local scope is explicitly authorized;
- parent traversal is rejected;
- `.git` paths are rejected;
- hidden broad globs are rejected;
- forbidden overlay wins over allowed entries;
- no hidden default broad allow exists.

Examples:

```text
# exact repo paths
docs/governance/evidence/NA-0382_qsl_local_ops_directive_manifest_allow_file_harness.md
tests/NA-0382_qsl_local_ops_directive_manifest_allow_file_testplan.md

# explicit fixture glob, accepted only after broad-glob checks
glob:inputs/local_ops/directive_manifest_fixtures/*.json
```

Rejected examples:

```text
../DECISIONS.md
/etc/fstab
*
glob:**
glob:docs/**
local:/home/victor/work/qsl/codex/responses/example.md
```

Integration output:

- validator emits a JSON summary containing accepted entries, rejected entries, forbidden-overlay hits, normalized helper-compatible patterns, and changed-path classification;
- if the validator invokes `qsl_evidence_helper.py scope-guard`, it must pass a generated temporary compatibility file derived from the stricter allow-file, not the raw line-based allow-file when prefixes are present.

## Manifest / Allow-File Lifecycle, Storage, and Backup-Impact Analysis

NA-0381 creates no manifest, allow-file, parser, or generated compatibility file.

Future NA-0382 should use tracked qsl-protocol fixtures only:

- `inputs/local_ops/directive_manifest_fixtures/`
- `inputs/local_ops/scope_allow_file_fixtures/`

Recommended future live directive behavior:

- live directive manifests, if supplied by an operator, remain outside repo unless the directive explicitly permits tracked fixture storage;
- temporary compatibility files may be written under `/srv/qbuild/tmp` during validation and must not be treated as durable history;
- durable local operator manifests or allow-files under `/home/victor/work/qsl/codex/**` require a backup/history coverage review before adoption;
- local history index work should precede durable local manifest archives.

Backup-plan decision:

- NA-0381 requires no backup-plan update because changes are qsl-protocol governance/testplan/traceability/journal paths only.
- Future NA-0382 tracked fixtures require no backup-plan update if all durable artifacts stay tracked in qsl-protocol.
- Durable manifests, allow-files, indexes, response-writer output, or local directive archives outside repo tracking require separate backup-impact review.

## Integration Plan with Bounded Polling Helper, qsl_evidence_helper, Goal-Lint, and Public-Safety

Future NA-0382 flow:

1. Validate manifest before tracked edits or PR creation.
2. Validate allow-file syntax and path normalization before `scope-guard`.
3. Run existing `qsl_evidence_helper.py queue` and `decisions`.
4. Feed validated, helper-compatible allow/forbidden pattern files into `qsl_evidence_helper.py scope-guard`.
5. Run `tools/goal_lint.py` or PR body preflight to confirm `Goals:` metadata.
6. Use `scripts/ci/qsl_bounded_check_poll.py pr` after PR creation to poll required checks.
7. Use `scripts/ci/qsl_bounded_check_poll.py public-safety` after merge.

Non-negotiable integration boundaries:

- manifest cannot override `NEXT_ACTIONS.md`;
- live repo state beats manifest content;
- manifest mismatch stops;
- public-safety remains independent and required;
- no auto-merge, admin bypass, branch protection mutation, workflow mutation, or check weakening is allowed.

## Fixture and Negative-Case Test Strategy

Future NA-0382 fixtures must include:

- valid manifest;
- missing `directive_id`;
- wrong `target_na`;
- wrong `expected_origin_main`;
- forbidden path touched;
- allowed exact path accepted;
- unlisted path rejected;
- broad glob rejected;
- parent traversal rejected;
- local absolute path rejected unless explicitly local-scoped;
- missing required check rejected;
- successor mismatch rejected;
- public-claim boundary missing rejected;
- unknown key rejected;
- malformed JSON rejected;
- allow-file comments and blank lines handled;
- allow-file exact paths handled;
- forbidden overlay wins over allowed;
- no-secret scan rejects secret-shaped material.

## Candidate Implementation Path Risk Matrix

| Option | Value | Risk | Backup impact | CI/security impact | Testability | Status |
|---|---|---|---|---|---|---|
| Standalone `scripts/ci/qsl_directive_manifest_validate.py` | Exact validator without shared-helper churn | new helper surface | low if tracked | standard-library, no workflow change | high with fixtures | recommended |
| Extend `scripts/ci/qsl_evidence_helper.py` | single helper surface | larger shared-helper regression risk | low | may affect existing scope/CI diagnostics | medium | defer unless future scope requires |
| Shell validator | POSIX-friendly | JSON/path validation brittle | low | higher parsing risk | low | reject |
| Local `/srv/qbuild/tools` validator | close to qstart/qresume | local-tool backup and portability concerns | needs backup review | outside repo CI evidence | medium | reject for first lane |
| No validator/manual directives | no code | preserves manual drift and fragmented evidence | none | weaker fail-closed posture | poor | reject |

## First-Lane Authorization Decision

NA-0381 authorizes future standalone qsl-protocol directive manifest and allow-file validator harness work in NA-0382.

Authorized first lane:

`DIRECTIVE_MANIFEST_ALLOW_FILE_IMPLEMENTATION_AUTHORIZATION_READY`

Not authorized by NA-0381:

- implementation in NA-0381;
- `qsl_evidence_helper.py` mutation in NA-0381;
- workflow changes;
- dependencies;
- runtime/service/protocol/crypto changes;
- generated live manifests or allow-files;
- local history/archive mutation;
- backup configuration mutation;
- public-claim expansion.

## Future Allowed / Forbidden Path Bundle

Recommended future NA-0382 allowed paths:

- `scripts/ci/qsl_directive_manifest_validate.py`
- `inputs/local_ops/directive_manifest_fixtures/`
- `inputs/local_ops/scope_allow_file_fixtures/`
- `docs/governance/evidence/NA-0382_qsl_local_ops_directive_manifest_allow_file_harness.md`
- `tests/NA-0382_qsl_local_ops_directive_manifest_allow_file_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Alternative path only if future live NA-0382 scope explicitly authorizes it:

- `scripts/ci/qsl_evidence_helper.py`

Forbidden future paths unless separately authorized:

- `.github/**`
- `scripts/ci/public_safety_gate.py`
- `scripts/ci/qsl_bounded_check_poll.py`
- workflows
- Cargo files
- dependencies
- runtime/service/protocol/crypto paths
- qsl-server/qsl-attachments
- qshield runtime
- backup scripts/timers/fstab
- website/public docs
- `/home/victor/work/qsl/codex/**`
- `/srv/qbuild/tools/**`

## Audit-Finding Carry-Forward Matrix

| Finding | Summary | Severity | Addressed by NA-0381 | Proposed NA-0382 | Future lane needed | Timing | Scope | Public-claim implication |
|---|---|---|---|---|---|---|---|---|
| GOV-001 | local history backup coverage partial | MEDIUM | boundary recorded | no | yes | after NA-0382 or response writer | backup/local-ops | no disaster-recovery claim |
| GOV-002 | broad validation expensive | MEDIUM | profiles planned | partial | yes | after manifest harness | CI | evidence-incomplete retained |
| GOV-003 | response/journal indexing manual | MEDIUM | response fields planned | partial | yes | after or alongside validation profiles | local-ops | no local-ops-complete claim |
| GOV-004 | CodeQL/docs-only policy distributed | LOW | policy fields planned | yes | maybe | NA-0382 first | CI | no silent skip/neutral drift |
| GOV-006 | public paper opportunity | OPPORTUNITY | future-gated | no | yes | after external-review readiness | public-claim | no public-copy expansion |
| CC-001 | duplicate crypto dependencies | MEDIUM | carry-forward | no | yes | after local-ops stabilization | dependency/crypto | no crypto-readiness overclaim |
| CC-002 | long all-workspace test timeout | MEDIUM | bounded profiles planned | partial | yes | after manifest harness | validation | no full-suite overclaim |
| CC-003 | test-only clippy lints | LOW | carry-forward | no | yes | opportunistic cleanup | tests | no all-target clean claim |
| CC-004 | qsc panic-demo boundary | MEDIUM | carry-forward | no | yes | security hardening lane | runtime diagnostics | no production-readiness claim |
| CC-005 | refimpl actor deterministic nonce/RNG boundary | MEDIUM | carry-forward | no | yes | crypto review readiness | refimpl/test | no external-review overclaim |
| CC-007 | fuzz/property/formal gap | OPPORTUNITY | carry-forward | no | yes | external-review preparation | verification | no complete-proof claim |

## Governance / Security / Fail-Closed Requirements

Future tooling must enforce:

- manifest cannot override `NEXT_ACTIONS.md`;
- live repo state beats manifest;
- manifest mismatch stops;
- one READY item required;
- exact path matching by default;
- forbidden overlay wins;
- no broad globs by default;
- no parent traversal;
- no absolute local paths unless explicitly local-scoped;
- no secret material;
- no branch protection bypass;
- no admin bypass;
- no auto-merge;
- no hidden mutation;
- deterministic output;
- bounded checks;
- machine-readable and human-readable summaries.

## Public-Claim / External-Review / Website Boundary

NA-0381 authorization does not:

- implement manifest or allow-file tooling;
- prove production readiness;
- prove public-internet readiness;
- complete external review;
- change metadata runtime claims;
- prove off-host backup;
- solve operator response absence;
- justify a website/public docs update;
- justify a public technical position paper now.

The NA-0380 audit reports do not expand public claims. qsl-server PR #56 remains bounded harness evidence only. qsl-attachments PR #37 remains service-local prerequisite evidence only.

## Future Validation / Marker / Verification Plan

Future NA-0382 should emit or record:

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
- `NA0382_NO_METADATA_FREE_CLAIM_OK`
- `NA0382_NO_ANONYMITY_CLAIM_OK`
- `NA0382_NO_UNTRACEABLE_CLAIM_OK`
- `NA0382_NO_PRODUCTION_READY_CLAIM_OK`
- `NA0382_NO_PUBLIC_INTERNET_READY_CLAIM_OK`

## Selected Successor

Selected successor:

`NA-0382 -- QSL Local Ops Directive Manifest and Allow-File Implementation Harness`

Rationale:

- NA-0380 closed bounded polling-helper implementation.
- NA-0381 schema/path/backup/fail-closed review found no blocker to a standalone qsl-protocol harness.
- A standalone helper avoids shared `qsl_evidence_helper.py` churn while still integrating with `scope-guard`.

## Rejected Alternatives

- Implementing manifest/allow-file tooling in NA-0381: rejected as out of scope.
- Modifying `qsl_evidence_helper.py` now: rejected because standalone validation can provide stricter preflight and compatibility output.
- Changing workflows now: rejected because local harness and PR evidence are sufficient.
- Adding a public technical paper now: rejected because external-review readiness remains future-gated.
- Changing runtime/dependencies now: rejected as unrelated and out of scope.

## Backup-Plan Impact Statement

No backup-plan update is required for NA-0381 because all durable changes are qsl-protocol governance, testplan, traceability, and journal paths. Future durable local manifests, allow-files, response-writer outputs, history indexes, local directive archives, or backup source-list changes require separate backup-impact review.

## Next Recommendation

After NA-0381 merges and public-safety remains green, close out NA-0381 and restore:

`NA-0382 -- QSL Local Ops Directive Manifest and Allow-File Implementation Harness`
