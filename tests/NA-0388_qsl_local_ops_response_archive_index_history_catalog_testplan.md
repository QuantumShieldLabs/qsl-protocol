Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-30

# NA-0388 QSL Local Ops Response Archive Index and History Catalog Testplan

Goals: G1, G2, G3, G4, G5

## Objective

Validate the NA-0388 temp-output response/archive/history catalog harness.

## Protected invariants

- READY_COUNT remains `1`.
- READY remains `NA-0388` until optional closeout.
- NA-0387 remains DONE.
- D-0756 and D-0757 exist once.
- D-0758 exists once after this implementation PR.
- D-0759 remains absent until optional closeout.
- Existing response, request, directive, journal, and ops-history files are not
  edited, deleted, moved, overwritten, truncated, or copied into a durable
  catalog.
- No durable catalog/index output is created.
- No secret-bearing content is copied into catalog output.

## Allowed scope

- `scripts/ci/qsl_response_history_catalog.py`
- `inputs/local_ops/response_history_catalog_fixtures/`
- `docs/governance/evidence/NA-0388_qsl_local_ops_response_archive_index_history_catalog_harness.md`
- `tests/NA-0388_qsl_local_ops_response_archive_index_history_catalog_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- temporary proof output under `/srv/qbuild/tmp/NA0388_response_history_catalog_*`

## Forbidden scope

Forbidden paths include workflows, `.github/**`, Cargo/dependency files,
runtime/service/protocol/crypto/auth/state-machine files, qshield runtime,
qsl-server, qsl-attachments, qsc-desktop, website, docs/public, README,
START_HERE, backup scripts/timers/fstab/source lists/system services,
`scripts/ci/qsl_codex_response_writer.py`,
`scripts/ci/qsl_evidence_helper.py`,
`scripts/ci/qsl_bounded_check_poll.py`,
`scripts/ci/qsl_directive_manifest_validate.py`,
`scripts/ci/public_safety_gate.py`, `/srv/qbuild/tools/**`, durable response,
request, directive, journal indexes, and `/home/victor/work/qsl/codex/**`
except the final D207 response file required by the directive.

## Helper CLI requirements

Run:

```bash
python3 scripts/ci/qsl_response_history_catalog.py --help
python3 scripts/ci/qsl_response_history_catalog.py fixture --fixture-dir inputs/local_ops/response_history_catalog_fixtures --tmp-dir /srv/qbuild/tmp/NA0388_response_history_catalog_<timestamp> --json
python3 scripts/ci/qsl_response_history_catalog.py scan --root responses=/home/victor/work/qsl/codex/responses --root requests=/home/victor/work/qsl/codex/requests --root directives=/home/victor/work/qsl/codex/directives --root journals=/home/victor/work/qsl/codex/journals --root ops=/home/victor/work/qsl/codex/ops --out-dir /srv/qbuild/tmp/NA0388_response_history_catalog_<timestamp>/live --json
python3 scripts/ci/qsl_response_history_catalog.py validate --catalog /srv/qbuild/tmp/NA0388_response_history_catalog_<timestamp>/live/catalog.json --json
```

## Fixture requirements

Fixtures must include:

- valid wrapped response;
- closeout/recovery response filename;
- synthetic smoke marker response;
- request file;
- directive file;
- empty journal root;
- ops file;
- malformed filename;
- missing wrapper;
- duplicate inferred directive;
- secret sentinel;
- generated binary/non-UTF8 case;
- generated symlink case;
- traversal, unknown-label, outside-root, outside-output, durable-output, and
  full-body-field rejection cases.

## Positive validation requirements

The fixture matrix must pass all positive cases and emit:

- `NA0388_RESPONSE_HISTORY_CATALOG_HELPER_OK`
- `NA0388_RESPONSE_ARCHIVE_SCAN_OK`
- `NA0388_REQUESTS_SCAN_OK`
- `NA0388_DIRECTIVES_ROOT_ABSENT_OR_SCANNED_OK`
- `NA0388_JOURNALS_ROOT_ABSENT_OR_SCANNED_OK`
- `NA0388_OPS_SCAN_OK`

## Negative and fail-closed requirements

The fixture matrix must prove:

- missing wrapper flagged;
- malformed filename handled;
- duplicate inferred directive flagged;
- secret sentinel rejected without content quote;
- binary/non-UTF8 handled without body copy;
- symlink rejected;
- parent traversal rejected;
- unknown root label rejected;
- root outside allow-list rejected;
- output outside `/srv/qbuild/tmp` rejected;
- durable catalog output rejected;
- full-body catalog field rejected.

## Live smoke requirements

Live smoke scans only explicitly supplied allow-listed roots. It writes only to
`/srv/qbuild/tmp/NA0388_response_history_catalog_<timestamp>/live`.

Required live evidence:

- root counts;
- catalog path;
- catalog SHA-256;
- NA-0386 smoke file metadata and checksum;
- D206 response metadata and checksum;
- secret-sentinel count;
- absent directive/journal root handling;
- no archived response mutation.

## Metadata-only and no-full-body-copy requirements

Catalog output must contain metadata fields only: source label, relative path,
classification, size, mtime, SHA-256, filename inference, bounded header
metadata, wrapper/smoke flags, extraction status, secret scan status, and error
identifiers.

Catalog output must not include full body, body text, raw text, content,
wrapper bodies, or secret-bearing material.

## No archive mutation requirements

Post-smoke checks must confirm local history counts and key response checksums
remain stable. Existing history files must not be overwritten, deleted, moved,
truncated, or rewritten.

## No durable catalog requirements

The helper must reject output outside `/srv/qbuild/tmp`. The proof bundle must
not be committed, and no durable catalog/index under `/home/victor/work/qsl/codex`
may exist.

## No-network and no-mutation requirements

The helper must use only Python standard library code. It must not call the
network, GitHub, subprocess, shell, branch mutation, deletion, or scanned-root
mutation facilities.

## No-secret requirements

Secret sentinel detection must record status only. It must not quote secret-like
content in stdout, catalog JSON, summary text, fixture log, evidence, or PR
body.

## Backup-impact requirements

Evidence must state that:

- tracked helper, fixtures, governance, traceability, and journal updates live
  in qsl-protocol;
- proof logs and catalogs stay under `/srv/qbuild/tmp`;
- no backup-plan update is required for this temp-output harness;
- future durable catalog/index output requires separate backup-impact review;
- same-host continuity is not disaster recovery.

## Public-claim boundary requirements

Do not introduce production, public internet, external-review, anonymity,
metadata-free, untraceability, off-host backup, restore, or disaster-recovery
claims.

## Successor selection requirements

Selected successor after NA-0388:

`NA-0389 -- QSL Local Ops Routine Audit Cadence Authorization Plan`

NA-0389 must not be implemented by this PR.

## Required local checks

Run and record:

```bash
python3 scripts/ci/qsl_response_history_catalog.py --help
python3 scripts/ci/qsl_response_history_catalog.py fixture --fixture-dir inputs/local_ops/response_history_catalog_fixtures --tmp-dir /srv/qbuild/tmp/NA0388_response_history_catalog_<timestamp> --json
python3 -m py_compile scripts/ci/qsl_response_history_catalog.py
python3 scripts/ci/qsl_response_history_catalog.py validate --catalog /srv/qbuild/tmp/NA0388_response_history_catalog_<timestamp>/live/catalog.json --json
python3 scripts/ci/qsl_codex_response_writer.py --help
python3 scripts/ci/qsl_codex_response_writer.py fixture --fixture-dir inputs/local_ops/response_writer_fixtures --tmp-dir /srv/qbuild/tmp/NA0388_response_writer_<timestamp>
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
python3 scripts/ci/qsl_evidence_helper.py pr-body-preflight --body-file <body>
```

## CI expectations

Required checks must attach and complete green before merge. public-safety must
remain required and green before merge and after merge. No admin bypass, direct
push, squash, rebase, force-push, amend, or branch deletion is authorized.

## Successor handoff

If optional closeout runs, NA-0388 must become DONE and NA-0389 must become the
sole READY item. Closeout must not implement NA-0389.
