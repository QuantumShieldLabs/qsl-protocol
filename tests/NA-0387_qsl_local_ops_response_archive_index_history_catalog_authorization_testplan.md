Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-30

# NA-0387 QSL Local Ops Response Archive Index and History Catalog Authorization Testplan

Goals: G1, G2, G3, G4, G5

## Objective

Validate that NA-0387 makes an authorization-only decision for future
response/archive/history catalog work based on read-only local history
inventory, inherited NA-0386 response-writer evidence, no-secret policy,
output-location risk, backup impact, fixture strategy, and successor selection.

## Protected invariants

- READY_COUNT remains `1`.
- READY remains `NA-0387` until optional closeout.
- NA-0386 remains DONE.
- D-0754 exists once.
- D-0755 exists once.
- D-0756 exists once after the authorization PR.
- D-0757 remains absent until optional closeout.
- NA-0387 does not implement a catalog or index.
- Existing response, request, directive, journal, and ops-history files are not
  edited, deleted, overwritten, truncated, moved, or copied into a durable
  catalog.
- No durable catalog output is created.
- No runtime, workflow, dependency, qsl-server, qsl-attachments, qshield runtime,
  website, public docs, README, START_HERE, backup script, timer, fstab, target,
  key, restore, deploy, rollback, or off-host setup mutation occurs.

## Allowed scope

- `docs/governance/evidence/NA-0387_qsl_local_ops_response_archive_index_history_catalog_authorization.md`
- `tests/NA-0387_qsl_local_ops_response_archive_index_history_catalog_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Forbidden scope

Forbidden paths include `.github/**`, workflows, Cargo/dependency files,
runtime/service/protocol/crypto/auth/state-machine files, qshield runtime,
qsl-server, qsl-attachments, qsc-desktop, website, docs/public, README,
START_HERE, backup scripts/timers/fstab/source lists/system services,
`scripts/ci/qsl_codex_response_writer.py`,
`scripts/ci/qsl_evidence_helper.py`,
`scripts/ci/qsl_bounded_check_poll.py`,
`scripts/ci/qsl_directive_manifest_validate.py`,
`scripts/ci/public_safety_gate.py`, `/srv/qbuild/tools/**`, response indexes,
directive/journal/request indexes, and `/home/victor/work/qsl/codex/**` except
the final D206 response file required by the directive.

## NA-0386 inheritance requirements

Evidence must summarize D-0754 and D-0755, confirm PR #1035 and #1036 handoff,
verify the NA-0386 synthetic smoke file path/checksum, and preserve that NA-0386
created no response index or broader history catalog.

## History-root inventory requirements

The review must read-only inspect:

- `/home/victor/work/qsl/codex/responses`;
- `/home/victor/work/qsl/codex/requests`;
- `/home/victor/work/qsl/codex/directives` if present;
- `/home/victor/work/qsl/codex/journals` if present;
- `/home/victor/work/qsl/codex/ops` if present.

For each root, record presence, directory ownership/mode, approximate file count,
recent filenames/metadata only, obvious missing roots, catalog/index presence or
absence, and backup coverage classification if known. Do not quote full file
contents.

## No-secret/catalog policy requirements

Evidence must require metadata-only output, no full response body copying,
bounded extraction only for approved metadata fields, high-confidence secret
detection without quoting content, no silent redaction, false-positive evidence,
and no parsing or storage of credentials, private keys, tokens, recovery
material, host fingerprints, connection output, or secret-bearing path tokens.

## Output-location requirements

Evidence must compare:

- qsl-protocol tracked fixture/test outputs only;
- `/srv/qbuild/tmp` temp catalog output;
- `/home/victor/work/qsl/codex/ops/catalog`;
- `/home/victor/work/qsl/codex/responses` adjacent catalog;
- qsl-protocol docs/ops generated catalog;
- no catalog/manual status quo.

For each, record backup impact, mutation risk, secret risk, maintainability, CI
suitability, restore value, authority, and recommendation.

## Fixture strategy requirements

Evidence must define future fixtures for valid/missing wrappers, D suffixes,
closeout/recovery qualifiers, smoke markers, request/directive/journal/ops
files, malformed names, duplicate inferred IDs, missing metadata, secret
sentinels, binary or non-UTF8 files, unreadable simulation if feasible,
symlinks, parent traversal, roots outside allow-list, temp output only, and JSON
summary output.

## Risk matrix requirements

Evidence must compare standalone helper, response-writer extension, directive
manifest validator extension, evidence-helper extension, shell script, local
`/srv/qbuild/tools` cataloger, and manual-only status quo. The matrix must state
value, risk, backup impact, secret risk, CI impact, dependency/workflow impact,
testability, authority, and recommendation.

## Routine audit cadence recommendation requirements

Evidence must state that a catalog/index is not routine audit cadence, can
support later audits, should not replace audit policy, and should not jump the
queue ahead of a ready bounded implementation harness.

## Authorization decision requirements

Evidence must select exactly one classification:

- `RESPONSE_HISTORY_CATALOG_IMPLEMENTATION_AUTHORIZATION_READY_FOR_TEMP_OUTPUT_HARNESS`
- `RESPONSE_HISTORY_CATALOG_BLOCKED_PENDING_BACKUP_OR_HISTORY_ROOT_DECISION`

## Path bundle requirements

If implementation is authorized, evidence must define future allowed NA-0388
paths for the standalone helper, fixtures, evidence, testplan, D/traceability,
and journal. It must forbid real durable catalog output unless future live scope
explicitly authorizes it.

## Fail-closed requirements

Evidence must require no archived file mutation, no deletion, no overwrite, no
secret content storage, no durable catalog writes in NA-0387, temp-output first,
exact root allow-list, symlink/path traversal rejection, deterministic output,
JSON/human summaries, metadata-only checksums, and same-host backup caveat.

## Public-claim boundary requirements

Evidence must not introduce:

- no production-readiness claims;
- no public-internet-readiness claims;
- no external-review-complete claims;
- no anonymity claims;
- no metadata-free claims;
- no untraceable claims;
- no complete disaster recovery claims;
- no off-host-backup-complete claims;
- no qsl-server production claims;
- no qsl-attachments production claims.

## Successor selection requirements

If ready, selected successor must be:

`NA-0388 -- QSL Local Ops Response Archive Index and History Catalog Implementation Harness`

If blocked, selected successor must be:

`NA-0388 -- QSL Local Ops History Catalog Backup Coverage Blocker Resolution`

NA-0388 must not be implemented by NA-0387.

## Required local checks

Run and record:

```bash
python3 scripts/ci/qsl_codex_response_writer.py --help
python3 scripts/ci/qsl_codex_response_writer.py fixture --fixture-dir inputs/local_ops/response_writer_fixtures --tmp-dir /srv/qbuild/tmp/NA0387_response_writer_<timestamp>
python3 scripts/ci/qsl_codex_response_writer.py fixture --fixture-dir inputs/local_ops/response_writer_real_archive_fixtures --tmp-dir /srv/qbuild/tmp/NA0387_response_writer_real_archive_<timestamp>
python3 scripts/ci/qsl_bounded_check_poll.py --help
python3 scripts/ci/qsl_bounded_check_poll.py fixture --fixture inputs/local_ops/qsl_bounded_check_poll_fixtures/pr_required_success.json --policy pr-required
python3 scripts/ci/qsl_directive_manifest_validate.py --help
python3 scripts/ci/qsl_directive_manifest_validate.py fixture --fixture-dir inputs/local_ops/directive_manifest_fixtures --allow-fixture-dir inputs/local_ops/scope_allow_file_fixtures
cargo audit --deny warnings
cargo tree -i rustls-webpki --locked
cargo fmt --check
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
python3 formal/model_qsc_handshake_suite_id_bounded.py
python3 formal/run_model_checks.py
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main --allowed-file <allowed> --forbidden-file <forbidden>
python3 scripts/ci/qsl_evidence_helper.py link-check --root .
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
```

## CI expectations

Required checks must attach and complete green before merge. public-safety must
remain required and green before merge and after merge. No admin bypass, direct
push, squash, rebase, force-push, amend, or branch deletion is authorized.

## Successor handoff

If optional closeout runs, NA-0387 must be marked DONE and the selected exact
NA-0388 successor must become the sole READY item. Closeout must not implement
NA-0388.
