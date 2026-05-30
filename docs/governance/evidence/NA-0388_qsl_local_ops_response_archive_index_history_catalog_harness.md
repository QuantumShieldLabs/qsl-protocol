Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-30

# NA-0388 QSL Local Ops Response Archive Index and History Catalog Harness

Goals: G1, G2, G3, G4, G5

## Executive summary

NA-0388 delivers the qsl-protocol temp-output response/archive/history catalog
harness authorized by NA-0387.

Implemented helper:

`scripts/ci/qsl_response_history_catalog.py`

Fixture directory:

`inputs/local_ops/response_history_catalog_fixtures/`

Temporary proof bundle:

`/srv/qbuild/tmp/NA0388_response_history_catalog_20260530T141441-0500/`

The helper emits metadata-only catalog JSON and a human summary under
`/srv/qbuild/tmp`. It does not create a durable local catalog, does not write
under `/home/victor/work/qsl/codex`, does not mutate archived responses, and
does not copy full response bodies into catalog output.

## Live NA-0388 scope

Live `NEXT_ACTIONS.md` records READY_COUNT `1` and READY
`NA-0388 -- QSL Local Ops Response Archive Index and History Catalog Implementation Harness`.

The live objective is to implement a bounded temp-output catalog harness that:

- scans authorized history roots read-only;
- emits metadata-only output under `/srv/qbuild/tmp`;
- proves no archive mutation;
- proves no secret content copy;
- proves no durable index output;
- preserves runtime, workflow, dependency, backup, qsl-server, qsl-attachments,
  website, README, START_HERE, and docs/public boundaries.

The live scope matches this directive.

## Inherited NA-0387 authorization

NA-0387 selected:

`RESPONSE_HISTORY_CATALOG_IMPLEMENTATION_AUTHORIZATION_READY_FOR_TEMP_OUTPUT_HARNESS`

D-0756 authorized the future standalone helper and fixture directory. D-0757
closed NA-0387 and restored NA-0388 as the sole READY successor.

Inherited evidence:

- qsl-protocol PR #1037 merged as `f8165a6626fa`.
- qsl-protocol PR #1038 merged as `bf682a102ddc`.
- NA-0386 smoke file exists at
  `/home/victor/work/qsl/codex/responses/NA0386_20260530T080430-0500_D205.md`.
- Smoke SHA-256 matched
  `2d06eb23330873576f813d875dadb08b5b26c019138f9cef77af27b8d20b5e40`.

## Implemented helper path

`scripts/ci/qsl_response_history_catalog.py` is standalone Python standard
library code.

The helper has three modes:

- `fixture`: runs the no-network fixture matrix and writes temp proof output.
- `scan`: scans explicitly supplied allow-listed roots and writes temp catalog
  output.
- `validate`: validates a catalog JSON file and rejects body/content fields.

The helper does not import network, GitHub, subprocess, shell, deletion, or
remote-operation facilities.

## Fixture matrix and markers

Fixture proof log:

`/srv/qbuild/tmp/NA0388_response_history_catalog_20260530T141441-0500/fixture_matrix.log`

Fixture catalog:

`/srv/qbuild/tmp/NA0388_response_history_catalog_20260530T141441-0500/catalog.json`

Fixture catalog SHA-256:

`9422809fde326f66e0c8076660297cc198ac5e69c81c2cf1704236f638496f5e`

Fixture log SHA-256:

`53a51dc1c7c3ca7c074db32f01a82c27f26514cbdd7c8e00e940d69330e58e4b`

Markers emitted by fixture proof include:

- `NA0388_RESPONSE_HISTORY_CATALOG_AUTHORIZATION_OK`
- `NA0388_RESPONSE_HISTORY_CATALOG_HELPER_OK`
- `NA0388_RESPONSE_ARCHIVE_SCAN_OK`
- `NA0388_REQUESTS_SCAN_OK`
- `NA0388_DIRECTIVES_ROOT_ABSENT_OR_SCANNED_OK`
- `NA0388_JOURNALS_ROOT_ABSENT_OR_SCANNED_OK`
- `NA0388_OPS_SCAN_OK`
- `NA0388_METADATA_ONLY_OK`
- `NA0388_NO_FULL_BODY_COPY_OK`
- `NA0388_SECRET_SENTINEL_REJECT_OK`
- `NA0388_SYMLINK_OR_TRAVERSAL_REJECT_OK`
- `NA0388_TEMP_OUTPUT_BOUNDARY_OK`
- `NA0388_NO_REAL_CATALOG_WRITE_OK`
- `NA0388_NO_ARCHIVE_MUTATION_OK`
- `NA0388_NO_DELETE_OK`
- `NA0388_BACKUP_IMPACT_OK`
- `NA0388_NO_WORKFLOW_CHANGE_OK`
- `NA0388_NO_DEPENDENCY_CHANGE_OK`
- `NA0388_NO_RUNTIME_CHANGE_OK`
- `NA0388_NO_SECRET_MATERIAL_OK`
- `NA0388_METADATA_RUNTIME_HISTORY_CATALOG_OK`
- `NA0388_NO_METADATA_FREE_CLAIM_OK`
- `NA0388_NO_ANONYMITY_CLAIM_OK`
- `NA0388_NO_UNTRACEABLE_CLAIM_OK`
- `NA0388_NO_PRODUCTION_READY_CLAIM_OK`
- `NA0388_NO_PUBLIC_INTERNET_READY_CLAIM_OK`

## Positive cases

The fixture matrix proves:

- valid CODEX RESPONSE wrapper detection;
- target NA, D suffix, and timestamp inference from response filenames;
- closeout/recovery-style response filename handling;
- synthetic smoke marker detection;
- request, directive, journal, and ops root handling;
- empty journal root handling;
- valid catalog JSON validation.

## Negative and fail-closed cases

The fixture matrix proves:

- missing response wrapper is flagged without body copy;
- malformed response filename is flagged;
- duplicate inferred response directive is flagged;
- secret sentinel is rejected or marked without quoting secret material;
- generated non-UTF8/binary content is marked without body copy;
- generated symlink is rejected;
- parent traversal root is rejected;
- unknown root label is rejected;
- root outside the allowed path is rejected;
- output outside `/srv/qbuild/tmp` is rejected;
- durable catalog output under local history roots is rejected;
- modeled full-body catalog fields are rejected by validation.

## Live read-only smoke proof

Live smoke command wrote only under:

`/srv/qbuild/tmp/NA0388_response_history_catalog_20260530T141441-0500/live/`

Live catalog:

`/srv/qbuild/tmp/NA0388_response_history_catalog_20260530T141441-0500/live/catalog.json`

Live catalog SHA-256:

`3ab3fbec0309a1167a7a55ede4cd55b4836b3ed8b632b5eac3749b7f6b94418c`

Live roots:

- responses: present, 169 files scanned.
- requests: present, 2 files scanned.
- directives: absent, recorded as absent.
- journals: absent, recorded as absent.
- ops: present, 8 files scanned recursively.

Live smoke metadata confirmed:

- NA-0386 smoke file entry present with SHA-256
  `2d06eb23330873576f813d875dadb08b5b26c019138f9cef77af27b8d20b5e40`.
- D206 response entry present with SHA-256
  `38d691ac169562c9a2eb6d2839c14bc1af57b4fb8de70726bde24a659d4cb3ac`.
- sentinel rejected count: `0`.
- nonfatal metadata error summary: duplicate inferred directive `6`,
  response wrapper missing `1`.

The nonfatal metadata errors are catalog observations only; they did not cause
archive mutation or content copy.

## No-full-body-copy proof

The catalog schema stores source label, relative path, classification, size,
mtime, SHA-256, filename-inferred metadata, bounded header fields, wrapper flag,
smoke marker flag, extraction status, secret scan status, and error identifiers.

It does not store full response bodies.

Targeted scans of fixture and live catalog outputs found no copied fixture body
text, no CODEX wrapper bodies, and no secret sentinel text in catalog output.

## No archive mutation proof

The helper only opens scanned roots for read operations. It writes catalog JSON,
summary text, and fixture proof logs under `/srv/qbuild/tmp`.

Post-smoke history counts remained:

- responses: 169 files.
- requests: 2 files.
- directives: absent.
- journals: absent.
- ops top-level files: 0.

The NA-0386 smoke checksum remained
`2d06eb23330873576f813d875dadb08b5b26c019138f9cef77af27b8d20b5e40`.

No D207 response file existed during this proof phase.

## No durable catalog proof

The helper rejects output directories outside `/srv/qbuild/tmp`.

The fixture matrix rejected `/home/victor/work/qsl/codex/ops/catalog` as a
durable output request. A post-smoke catalog/index filename search under local
history roots found no durable catalog/index file created by NA-0388.

## No-secret and sentinel policy proof

The helper uses high-confidence sentinel patterns and records only pattern IDs
and statuses. It does not quote matched secret-shaped material.

The fixture sentinel case was marked `secret_sentinel_rejected`. The live smoke
had zero secret-sentinel rejections.

## Backup and local continuity caveat

`/backup/qsl` was mounted as same-host storage during startup and post-smoke
checks. The latest observed manifest/log entries were daily local backup
artifacts.

NA-0388 does not run backup, restore, deploy, rollback, target setup, key
handling, or off-host operations.

Backup-impact classification:

- tracked helper, fixtures, evidence, testplan, D/traceability, and journal
  changes are qsl-protocol repository content;
- proof logs and generated catalogs stay under `/srv/qbuild/tmp`;
- no backup-plan update is required for this temp-output harness;
- any future durable local catalog/index output requires separate
  backup-impact review.

Same-host continuity remains not disaster recovery.

## Runtime, service, dependency, and workflow boundary

NA-0388 changes no workflow, dependency, Cargo, runtime, protocol, crypto,
state-machine, qshield runtime, qsc, qsp, qsl runtime, service, website,
README, START_HERE, docs/public, backup script, timer, fstab, source-list, or
system-service path.

The shared local-ops helpers remain untouched:

- `scripts/ci/qsl_codex_response_writer.py`
- `scripts/ci/qsl_evidence_helper.py`
- `scripts/ci/qsl_bounded_check_poll.py`
- `scripts/ci/qsl_directive_manifest_validate.py`
- `scripts/ci/public_safety_gate.py`

## qsl-server and qsl-attachments boundary

qsl-server PR #56 remains read-only bounded harness evidence at
`d40e6003fdf0`.

qsl-attachments PR #37 remains read-only service-local prerequisite evidence at
`96b9352bd63`.

NA-0388 does not clone or mutate either repository.

## Public-claim boundary

This harness is local-ops evidence only. It is not a production, public
internet, external-review, anonymity, metadata-free, untraceability, off-host
backup, restore, or disaster-recovery proof.

## Routine audit cadence interaction

The catalog helper can support later routine audits by giving operators a
metadata evidence primitive. It does not define audit cadence, finding
ownership, report locations, or public-claim timing.

## Selected successor

Selected successor:

`NA-0389 -- QSL Local Ops Routine Audit Cadence Authorization Plan`

Rationale: after the temp-output history catalog harness exists, the next local
ops lane should authorize when and how routine project and code/crypto audits
recur without disrupting the one-READY queue or overstating readiness.

## Rejected alternatives

- writing a durable catalog now;
- mutating archived response files;
- creating response/request/directive/journal indexes under real history roots;
- changing backup plans now;
- changing workflows;
- adding dependencies;
- jumping directly to a public technical paper.

## Next recommendation

Close NA-0388 only after the implementation PR merges and post-merge
public-safety is green. Then restore NA-0389 as the sole READY successor without
implementing NA-0389 in the closeout.
