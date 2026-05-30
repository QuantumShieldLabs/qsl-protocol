Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-29

# NA-0383 QSL Local Ops Response Writer Implementation Authorization

Goals: G1, G2, G3, G4, G5

## Executive Summary

NA-0383 authorizes a future bounded qsl-protocol response writer harness lane.
This is planning and authorization evidence only. NA-0383 does not implement a
response writer, add fixtures, generate response files, create response or
history indexes, mutate backup configuration, change workflows, change
dependencies, change runtime/service/protocol/crypto code, or expand public
claims.

Authorization classification:

`RESPONSE_WRITER_IMPLEMENTATION_AUTHORIZATION_READY`

Selected successor:

`NA-0384 -- QSL Local Ops Response Writer Implementation Harness`

The first implementation lane should use a standalone Python standard-library
helper in qsl-protocol with fixture tests that write only under
`/srv/qbuild/tmp`. Real writes to `/home/victor/work/qsl/codex/responses` must
remain future-gated unless NA-0384 live scope explicitly authorizes them and
records backup impact.

## Live NA-0383 Scope

Live `NEXT_ACTIONS.md` records:

- READY_COUNT `1`.
- READY `NA-0383 -- QSL Local Ops Response Writer Implementation Authorization Plan`.
- NA-0382 DONE.
- D-0746 exists once.
- D-0747 exists once.
- D-0748 absent at startup.
- public-safety remains required and green.

Allowed NA-0383 mutable paths:

- `docs/governance/evidence/NA-0383_qsl_local_ops_response_writer_implementation_authorization.md`
- `tests/NA-0383_qsl_local_ops_response_writer_implementation_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Forbidden NA-0383 scope includes response writer implementation, response writer
fixtures, generated response files, response/archive indexes, helper
implementation paths, `.github/**`, workflows, Cargo/dependency files,
runtime/service/protocol/crypto paths, qshield runtime, qsl-server,
qsl-attachments, qsc-desktop, website/public docs, README, START_HERE,
`qsl_evidence_helper.py`, `qsl_bounded_check_poll.py`,
`qsl_directive_manifest_validate.py`, `public_safety_gate.py`, qstart/qresume,
backup scripts/timers/fstab/source lists, remote/off-host setup, restore,
deploy, rollback, secret/key/credential handling, and public-claim expansion.

## Inherited NA-0382 Result

NA-0382 delivered the standalone directive manifest and allow-file validator in
qsl-protocol PR #1027 and closeout PR #1028.

- PR #1027 merge: `6efe0b6f8db5`.
- PR #1028 merge and startup `origin/main`: `859caa6a3e9`.
- Helper: `scripts/ci/qsl_directive_manifest_validate.py`.
- Fixtures: `inputs/local_ops/directive_manifest_fixtures/` and
  `inputs/local_ops/scope_allow_file_fixtures/`.
- Fixture proof log:
  `/srv/qbuild/tmp/NA0382_manifest_allow_file_20260529T143345-0500/fixture_matrix.log`.
- D-0746 records the implementation harness decision.
- D-0747 records closeout and restores NA-0383.

NA-0382 did not mutate `qsl_evidence_helper.py`, `qsl_bounded_check_poll.py`,
`public_safety_gate.py`, workflows, dependencies, runtime code, qsl-server,
qsl-attachments, qshield runtime, website/public docs, backup configuration, or
local history indexes.

## Response Archive / History / Backup Posture Review

Read-only inspection found:

- `/home/victor/work/qsl/codex/responses` exists and contains D175-D201 response
  files with `CODEX RESPONSE BEGIN/END` wrappers.
- `/home/victor/work/qsl/codex/requests` exists.
- `/home/victor/work/qsl/codex/ops` exists.
- `/home/victor/work/qsl/codex/directives` and
  `/home/victor/work/qsl/codex/journals` were absent during inspection.
- `/backup/qsl` is mounted as same-host local continuity storage, not complete
  disaster recovery.
- Current backup status lists `/home/victor/work/qsl/codex/responses` in the
  daily source list and records prior non-destructive restore drill evidence.
- Requests, directives, journals, and future response/history indexes are not
  proven covered as durable archive roots by this lane.

Response writer testing can be done safely under `/srv/qbuild/tmp` only. Real
archive writes by a helper should remain explicit future scope, and durable
reliance on helper-created archive files should trigger backup-plan review or
an explicit accepted residual risk statement.

## Existing Response-Writing Behavior Inventory

Prior responses D175-D201 show:

- filename pattern normally follows
  `NAxxxx_<YYYYMMDD>T<HHMMSS><timezone-offset>_Dnnn.md`;
- closeout/recovery qualifiers appear in some filenames;
- response files include actual response start timestamps and UTC timestamps;
- response files include directive timestamps and directive IDs;
- the standard `CODEX RESPONSE BEGIN/END` wrapper is consistently present;
- sections begin with `0. Directive / Response Identity Check`;
- stop-reason sections are consistently present even when no stop occurred;
- saved response file path is recorded in the response body;
- no observed `_r2` or `_r3` collision suffix file was present in D175-D201;
- D188 had two responses with different timestamps, proving same directive
  suffix reuse can happen without filename collision;
- manual response discipline remains fragile and is a direct local-ops friction
  point.

The future writer should mechanize only the wrapper, filename, collision, basic
section skeleton, validation, and no-secret stop behavior. It should not decide
queue state, merge PRs, mutate GitHub, change backup configuration, or create
indexes.

## Response Writer Requirements and Output Contract

Future helper contract:

- accept explicit output directory;
- accept target NA, directive suffix such as `D203`, directive ID, directive
  begin timestamps, optional directive end timestamps, and response start
  timestamp arguments or auto-capture mode;
- default timezone mode to America/Chicago and also record UTC;
- support a template mode for a standard skeleton;
- write a standard `CODEX RESPONSE BEGIN/END` wrapper;
- include required section headings;
- include saved response path;
- support dry-run mode with no file write;
- support validate-only mode with no file write;
- support no-secret scan mode;
- support JSON summary output;
- print final output path on successful write;
- never overwrite, delete, or edit an existing response file;
- never create response, directive, journal, or history indexes;
- never wait in background or imply future completion.

The helper should use Python standard library only if implemented inside
qsl-protocol.

## No-Secret / Sensitive-Material / Redaction / Stop Policy

Future behavior must scan response body and metadata for high-confidence private
key, token, credential, passphrase, recovery-envelope, and secret marker
patterns. If a high-confidence pattern is found, the default behavior must fail
closed before writing the response body.

Required no-secret behavior:

- do not quote or print secret content;
- report only generic category and path/field when safe;
- avoid writing secret-bearing response bodies to archive;
- never silently redact by mutating content unless a future directive explicitly
  authorizes a redaction mode;
- allow false-positive-safe marker text in fixtures;
- include positive no-secret and negative secret-shaped fixture coverage.

## Filename / Collision / Timestamp / Wrapper / Section Skeleton Design

Future filename format:

`NAxxxx_<YYYYMMDD>T<HHMMSS><timezone-offset>_Dnnn.md`

Future collision behavior:

- if the target filename exists, append `_r2`;
- if `_r2` exists, append `_r3`, and continue monotonically;
- preserve `.md` extension;
- never overwrite;
- never delete or edit existing response files.

Future validation behavior:

- normalize `NA-0384` and `NA0384` to canonical filename prefix `NA0384`;
- reject invalid target NA;
- reject invalid directive suffix;
- reject invalid timestamp or timezone offset;
- create the output directory when explicitly allowed;
- reject output directories outside explicitly allowed roots;
- include `CODEX RESPONSE BEGIN/END`;
- include required section headings, saved response path, and stop-reason
  section.

## Lifecycle / Storage / Backup-Impact / Local-History Boundary Analysis

Recommended first implementation storage:

- helper path: `scripts/ci/qsl_codex_response_writer.py`;
- fixtures: `inputs/local_ops/response_writer_fixtures/`;
- proof output: `/srv/qbuild/tmp/NA0384_response_writer_*`.

NA-0383 itself requires no backup-plan update because it changes only tracked
qsl-protocol governance/testplan/traceability/journal files. Future NA-0384 can
also avoid backup-plan update if tests write only under `/srv/qbuild/tmp`.

Real writes under `/home/victor/work/qsl/codex/responses` should be authorized
explicitly by live scope. Durable response archive use should be reviewed
against local backup coverage. Response/archive index creation is a separate
future lane and must not be bundled into the writer harness.

## Integration Plan with Manifest Validator, Polling Helper, Goal-Lint, and Public-Safety

Integration boundaries:

- the directive manifest validator can supply target NA, directive ID, prior
  response path, expected response path, required sections, and allowed output
  roots in a future lane;
- manifest data must never override live directive, queue, branch-protection, or
  public-safety truth;
- bounded polling helper remains separate and should not be imported or mutated
  by the response writer;
- goal-lint remains PR-body governance;
- public-safety remains an independent required check;
- response writer must not merge PRs, inspect GitHub, mutate GitHub, alter
  queue state, or change branch protection.

## Fixture and Negative-Case Test Strategy

Future NA-0384 fixtures should cover:

- valid minimal response body;
- valid full response body;
- missing required section;
- invalid target NA;
- invalid directive suffix;
- invalid timestamp;
- first collision creates `_r2`;
- collision chain creates `_r3`;
- missing output directory is created only when allowed;
- unauthorized output directory is rejected;
- existing file is never overwritten;
- high-confidence secret marker is rejected;
- false-positive-safe text is accepted;
- dry-run writes no file;
- validate-only writes no file;
- JSON summary is valid;
- wrapper is correct;
- stop-reason section is present.

## Candidate Implementation Path Risk Matrix

| Option | Value | Risk | Backup impact | CI/security/testability | Recommendation |
|---|---|---|---|---|---|
| Standalone qsl-protocol Python helper | High; narrow owner and fixtures | Low if temp-output only | None for temp tests; real archive mode future-gated | Standard-library, deterministic, fixture-friendly | Recommended |
| Extend `qsl_directive_manifest_validate.py` | Medium integration | Blurs manifest and writer responsibilities | Same as standalone | Testable but larger helper surface | Reject for first lane |
| Extend `qsl_evidence_helper.py` | Medium reuse | Shared helper blast radius | Same as standalone | Higher regression risk | Reject for first lane |
| Shell script | Medium speed | More brittle parsing/JSON/timestamp handling | Same as standalone | Harder portable validation | Reject |
| Local `/srv/qbuild/tools` writer | Useful locally | Outside qsl-protocol tracked review | Local-tool backup review required | Harder CI proof | Reject for NA-0384 |
| No writer / continue manual | No code risk | Keeps manual errors and archive friction | No new impact | No testable improvement | Reject |

## First-Lane Authorization Decision

`RESPONSE_WRITER_IMPLEMENTATION_AUTHORIZATION_READY`

NA-0384 may implement the standalone qsl-protocol response writer harness if it
keeps tests under `/srv/qbuild/tmp`, avoids real archive writes unless live
scope explicitly authorizes them, and preserves all no-runtime, no-workflow,
no-dependency, no-secret, no-index, no-backup-config, no-public-claim, and
fail-closed boundaries.

## Future Allowed / Forbidden Path Bundle

Allowed future NA-0384 paths:

- `scripts/ci/qsl_codex_response_writer.py`
- `inputs/local_ops/response_writer_fixtures/`
- `docs/governance/evidence/NA-0384_qsl_local_ops_response_writer_harness.md`
- `tests/NA-0384_qsl_local_ops_response_writer_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Future temporary output:

- `/srv/qbuild/tmp/NA0384_response_writer_*`

Future real response archive output:

- only if exact NA-0384 live scope explicitly authorizes it;
- otherwise test with temp output only.

Forbidden future paths unless separately authorized include `.github/**`,
scripts other than the exact helper, `qsl_evidence_helper.py`,
`qsl_bounded_check_poll.py`, `qsl_directive_manifest_validate.py`, workflows,
Cargo files, dependencies, runtime/service/protocol/crypto paths, qsl-server,
qsl-attachments, qshield runtime, qsc-desktop, backup scripts/timers/fstab,
website/public docs, response indexes, directive/journal indexes, and
`/home/victor/work/qsl/codex/**` except exact response output if a future
directive explicitly authorizes it.

## Audit-Finding Carry-Forward and Routine Audit Cadence Recommendation

NA-0380 audit findings are carried forward as follows:

- GOV-001 local history backup coverage remains open; response writer does not
  solve backup coverage.
- GOV-002 broad validation cost remains open; response writer should use bounded
  fixtures and not imply full validation.
- GOV-003 manual handoff friction is partly addressed by the future writer, but
  history indexes remain separate future work.
- GOV-004 distributed CI policy is not changed by response writer planning.
- GOV-005 service-local proof limits remain unchanged.
- GOV-006 public position paper remains future-gated.
- GOV-007 D132 cleanup remains a deliberate future directive only.

Recurring audit cadence should become a future governance/local-ops policy lane,
for example:

`QSL Governance Routine Audit Cadence and Finding Promotion Policy Plan`

That cadence lane should not displace NA-0384 unless response writer
implementation becomes blocked.

## Governance / Security / Fail-Closed Requirements

Future writer requirements:

- no overwrite;
- no deletion;
- no mutation of existing archives;
- no secret write;
- collision-safe filenames;
- deterministic timestamp validation;
- explicit output path;
- no index mutation;
- no backup config mutation;
- no queue mutation;
- no GitHub mutation;
- no branch mutation;
- human and JSON summaries;
- strict fixture tests.

## Public-Claim / External-Review / Website Boundary

NA-0383 is authorization only. Future response writer implementation would not
be production readiness, public-internet readiness, external review, metadata
runtime proof, off-host backup proof, operator-response resolution, code/crypto
audit remediation, local-ops completion, or public technical position paper
readiness.

No website, docs/public, README, START_HERE, or external website update is part
of NA-0383.

## Future Validation / Marker / Verification Plan

Future NA-0384 markers:

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
- `NA0384_BACKUP_IMPACT_OK`
- `NA0384_NO_INDEX_MUTATION_OK`
- `NA0384_NO_WORKFLOW_CHANGE_OK`
- `NA0384_NO_DEPENDENCY_CHANGE_OK`
- `NA0384_NO_RUNTIME_CHANGE_OK`
- `NA0384_NO_SECRET_MATERIAL_OK`
- `NA0384_NO_METADATA_FREE_CLAIM_OK`
- `NA0384_NO_ANONYMITY_CLAIM_OK`
- `NA0384_NO_UNTRACEABLE_CLAIM_OK`
- `NA0384_NO_PRODUCTION_READY_CLAIM_OK`
- `NA0384_NO_PUBLIC_INTERNET_READY_CLAIM_OK`

## Selected Successor

`NA-0384 -- QSL Local Ops Response Writer Implementation Harness`

Rationale: NA-0382 completed the manifest/allow-file harness, NA-0383 found no
backup/archive/no-secret blocker for a temp-output first implementation lane,
and a standalone response writer is the smallest testable next workflow-support
improvement.

## Rejected Alternatives

- Implement response writer immediately in NA-0383.
- Mutate real response archive now.
- Create response or history indexes now.
- Change backup plan now.
- Change workflows now.
- Extend existing shared helpers first.
- Add public technical paper now.
- Promote routine audit cadence ahead of NA-0384 while writer implementation is
  authorized.

## Backup-Plan Impact Statement

No backup-plan update is required for NA-0383 because changed durable files are
tracked qsl-protocol governance/testplan/traceability/journal files only. Future
NA-0384 temp-output tests under `/srv/qbuild/tmp` also require no backup-plan
update. Future durable helper writes to real response archives, indexes,
requests, directives, journals, ops roots, backup source lists, or local system
backup configuration require explicit future authorization and backup-impact
review.

## Next Recommendation

Proceed with NA-0384 as a qsl-protocol-only response writer implementation
harness using temp-output fixtures, no overwrite behavior, no-secret fail-closed
behavior, collision-safe filenames, standard wrappers, and no workflow/runtime/
dependency/public-claim drift.
