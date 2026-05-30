Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-30

# NA-0385 QSL Local Ops Response Archive Backup Coverage / Real-Archive Write Authorization Testplan

Goals: G1, G2, G3, G4, G5

## Objective

Validate that NA-0385 makes an authorization-only decision about future real
response archive writes, based on read-only response archive inventory, read-only
backup coverage evidence, no-secret/no-overwrite boundaries, local-history
impact, and successor risk.

## Protected invariants

- READY_COUNT remains `1`.
- READY remains `NA-0385` until optional closeout.
- NA-0384 remains DONE.
- D-0750 exists once.
- D-0751 exists once.
- D-0752 exists once after Packet Q.
- D-0753 remains absent until optional closeout.
- NA-0385 performs no real response archive write except the final D204
  response file written by Codex outside qsl-protocol.
- No response writer mutation occurs.
- No response, directive, journal, or local-history index is created.
- No backup script, timer, fstab, service, source-list, target, key,
  credential, restore, deploy, rollback, qsl-server, qsl-attachments, runtime,
  workflow, dependency, website, docs/public, README, or START_HERE mutation
  occurs.

## Allowed scope

- `docs/governance/evidence/NA-0385_qsl_local_ops_response_archive_backup_coverage_real_archive_write_authorization.md`
- `tests/NA-0385_qsl_local_ops_response_archive_backup_coverage_real_archive_write_authorization_testplan.md`
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
directive/journal indexes, and `/home/victor/work/qsl/codex/**` except the
final D204 response file required by the directive.

## NA-0384 inheritance requirements

Evidence must summarize D-0750 and D-0751, confirm the temp-output helper result,
and preserve that NA-0384 did not use the helper to write to the real response
archive.

## Archive inventory requirements

The review must read-only inspect `/home/victor/work/qsl/codex/responses` and
record presence, file count, D190 through D203 presence, D203 presence,
permissions/ownership shape, subdirectory count, `_rN` collision count,
response-writer generated file absence, and response index absence.

## Backup coverage requirements

The review must read-only inspect backup plan/status files, current mount state,
latest manifests/logs, and snapshot contents. It must classify response archive
coverage as same-host local continuity only and must not claim disaster
recovery.

## Real-archive write authorization option requirements

Evidence must compare at least:

- future one-file no-secret generated smoke;
- future actual final response only;
- blocking pending backup/local-history improvement;
- blocking pending off-host backup;
- temp-output only/manual writes.

## No-secret / overwrite / collision requirements

Evidence must define high-confidence secret rejection, false-positive handling,
no silent redaction, collision suffix behavior, finite collision attempts,
exclusive-create writes, immutable existing files, no cleanup/delete without a
future directive, synthetic smoke retention, and path/checksum evidence.

## Local-history / index requirements

Evidence must state that real response writing does not solve response indexes,
directive indexes, journal indexes, request/ops coverage, off-host backup, or
routine audit cadence.

## Risk matrix requirements

Evidence must compare the candidate NA-0386 successors and state value, risk,
prerequisites, backup impact, implementation scope, and recommendation.

## Authorization decision requirements

Evidence must select exactly one classification:

- `REAL_ARCHIVE_WRITE_IMPLEMENTATION_AUTHORIZATION_READY_WITH_LOCAL_CONTINUITY_CAVEAT`
- `REAL_ARCHIVE_WRITE_BLOCKED_PENDING_BACKUP_COVERAGE`

## Path bundle requirements

Evidence must define exact future allowed paths for the selected successor and
forbidden future paths unless separately authorized.

## Audit carry-forward requirements

Evidence must carry forward GOV-003 response/local-history residuals, GOV-001
backup caveats, code/crypto residuals, routine audit cadence, and public paper
timing without promoting them ahead of the selected successor.

## Fail-closed requirements

Evidence must require no overwrite, no delete, no secret write, no index, no
backup config mutation, explicit future scope, pre-write scan, checksum/path
evidence, deterministic summaries, and same-host-only wording.

## Public-claim boundary requirements

Evidence must not introduce production-readiness, public-internet-readiness,
external-review-complete, anonymity, metadata-free, untraceable, complete
disaster recovery, off-host backup complete, qsl-server production, or
qsl-attachments production claims.

## Successor selection requirements

If coverage is sufficient, selected successor must be:

`NA-0386 -- QSL Local Ops Response Writer Real-Archive Write Implementation Harness`

If coverage is missing or ambiguous, selected successor must be:

`NA-0386 -- QSL Local Ops Response Archive Backup Coverage Blocker Resolution`

NA-0386 must not be implemented by NA-0385.

## Required local checks

Run and record:

```bash
python3 scripts/ci/qsl_codex_response_writer.py --help
python3 scripts/ci/qsl_codex_response_writer.py fixture --fixture-dir inputs/local_ops/response_writer_fixtures --tmp-dir /srv/qbuild/tmp/NA0385_response_writer_<timestamp>
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

If optional closeout runs, NA-0385 must be marked DONE and the selected exact
NA-0386 successor must become the sole READY item. Closeout must not implement
NA-0386.
