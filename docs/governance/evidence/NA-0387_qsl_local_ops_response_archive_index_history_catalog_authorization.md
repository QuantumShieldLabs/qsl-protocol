Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-30

# NA-0387 QSL Local Ops Response Archive Index and History Catalog Authorization

Goals: G1, G2, G3, G4, G5

## Executive summary

NA-0387 authorizes a future NA-0388 qsl-protocol implementation harness for a
metadata-only response/archive/history catalog that writes only temporary proof
output under `/srv/qbuild/tmp`.

Classification:

`RESPONSE_HISTORY_CATALOG_IMPLEMENTATION_AUTHORIZATION_READY_FOR_TEMP_OUTPUT_HARNESS`

NA-0387 does not implement a catalog, does not create an index, does not mutate
any archived response, request, directive, journal, or ops-history file, and does
not change runtime, workflow, dependency, backup, qsl-server, qsl-attachments,
qshield runtime, website, README, START_HERE, or public docs surfaces.

The first future lane should prove the cataloger with qsl-protocol fixtures and
temporary output only. Durable real local catalog output remains forbidden until
a later directive explicitly chooses the location and records backup impact.

Selected successor:

`NA-0388 -- QSL Local Ops Response Archive Index and History Catalog Implementation Harness`

## Live NA-0387 scope

Live `NEXT_ACTIONS.md` records:

- READY_COUNT `1`.
- READY `NA-0387 -- QSL Local Ops Response Archive Index and History Catalog Authorization Plan`.
- NA-0386 DONE.
- D-0754 exists once.
- D-0755 exists once.
- D-0756 absent at startup.
- public-safety remains required and green.

Live objective:

- authorize whether and how to create a durable response/archive/history catalog
  or index that improves handoff evidence without mutating archived responses,
  hiding secrets, weakening backup boundaries, or conflating same-host
  continuity with disaster recovery.

Live protections:

- no runtime, service, protocol, crypto, dependency, or workflow implementation
  unless future exact scope authorizes exact files;
- no secret handling;
- no backup script, timer, fstab, or source-list mutation unless future exact
  scope authorizes it;
- no target setup;
- no public, readiness, or privacy overclaim.

NA-0387 live scope is compatible with this authorization-only directive. It does
not authorize catalog implementation in NA-0387.

## Inherited NA-0386 result

NA-0386 added real response archive write support in
`scripts/ci/qsl_codex_response_writer.py`, added fixtures under
`inputs/local_ops/response_writer_real_archive_fixtures/`, and wrote exactly one
synthetic no-secret real archive smoke file:

`/home/victor/work/qsl/codex/responses/NA0386_20260530T080430-0500_D205.md`

Smoke SHA-256:

`2d06eb23330873576f813d875dadb08b5b26c019138f9cef77af27b8d20b5e40`

Inherited properties:

- helper real-archive writes require explicit dual authorization;
- wrong archive paths are rejected;
- high-confidence secret patterns are rejected before write;
- existing response archive files are not overwritten, deleted, renamed, or
  rewritten;
- no response index, directive index, request index, journal index, or local
  history catalog was created;
- backup classification remains same-host local continuity only;
- the smoke file is durable archive content but is not proven restored and was
  not proven snapshotted by the latest daily snapshot cited in NA-0386.

NA-0387 inherits this result as source evidence only. It does not reuse the
response writer and does not write another real archive smoke file.

## Response/request/directive/journal/ops history inventory

Read-only local history inventory found:

| Root | State | Count / recent shape | Catalog/index state | Backup classification |
|---|---|---|---|---|
| `/home/victor/work/qsl/codex/responses` | Present, `victor:victor`, directory mode `0775` | 168 files; recent files include final D205 and the NA-0386 synthetic smoke file; typical response files are mode `0600` | No catalog-like files found in the directory listing | Same-host local continuity source list includes responses; latest observed daily snapshot predates the NA-0386 smoke file |
| `/home/victor/work/qsl/codex/requests` | Present, `victor:victor`, directory mode `0775` | 2 request files | No catalog-like files found | Backup coverage not proven by the status file reviewed in this lane |
| `/home/victor/work/qsl/codex/directives` | Absent | n/a | n/a | No root to classify |
| `/home/victor/work/qsl/codex/journals` | Absent | n/a | n/a | No root to classify |
| `/home/victor/work/qsl/codex/ops` | Present, `victor:victor`, directory mode `0775` | Contains backup-status material under an ops backup subtree | No catalog-like files found at the inspected level | Backup coverage for this ops root is not proven by the source list reviewed in this lane |

Additional read-only observations:

- `/backup/qsl` was mounted as same-host ext4 storage.
- `/srv/qbuild` had 468 GiB total, 121 GiB used, and 323 GiB available at
  startup.
- `/backup/qsl` had 916 GiB total, 24 GiB used, and 884 GiB available.
- The current local continuity status says this is not disaster recovery because
  source and backup storage are on the same host.
- No response, request, directive, journal, or ops history content was copied or
  quoted into this evidence.

## Index/catalog requirements design

Future catalog output should use a versioned JSON schema plus a concise human
summary. Candidate fields:

- schema version;
- generated timestamp;
- source root;
- root type: `responses`, `requests`, `directives`, `journals`, or `ops`;
- relative path under the authorized root;
- file type;
- size;
- mtime;
- SHA-256;
- directive ID if inferable;
- target NA if inferable;
- response start timestamp if inferable;
- directive begin timestamp if inferable;
- D suffix if inferable;
- PR numbers if safely extractable;
- merge SHAs if safely extractable;
- decision IDs if safely extractable;
- closeout state if safely extractable;
- selected successor if safely extractable;
- backup coverage classification;
- no-secret scan status or explicit secret-scan-skipped status;
- extraction status;
- errors.

Design constraints:

- metadata-first;
- no full response body copying;
- no archived file mutation;
- no deletion;
- no overwrite;
- no hidden redaction;
- no index mutation in NA-0387;
- future temp-output first;
- future durable output only after exact authorization;
- deterministic ordering and stable summaries;
- fail closed on path traversal, symlink escape, unreadable files, malformed
  inputs, ambiguous roots, and high-confidence secret material encountered
  during bounded content extraction.

## No-secret/content-extraction/sensitive-material policy

Future cataloging must not turn local history into a secret-bearing data lake.

Policy:

- cataloger reads file metadata and SHA-256 without copying full bodies into the
  output;
- bounded content extraction is allowed only for explicit metadata markers such
  as directive IDs, target NAs, PR numbers, merge SHAs, decision IDs, wrapper
  timestamps, closeout state, and selected successor;
- if high-confidence secret material is detected while scanning content, the
  cataloger must stop for that file or for the run without quoting the matched
  material;
- output may record a non-content status such as `secret_detected` only if no
  secret text, credential value, key body, host fingerprint, auth header, route
  token, recovery-envelope content, or secret-bearing path token is copied;
- false positives require explicit evidence and fixture coverage; they must not
  be handled by silent content rewriting;
- raw credentials, private keys, tokens, recovery-envelope content, host
  fingerprints, connection output, and secret paths must never be parsed or
  stored;
- test fixtures may use harmless sentinel strings to prove rejection.

## Output location/backup/retention options

| Option | Backup impact | Mutation risk | Secret risk | CI suitability | Restore value | Authority | Recommendation |
|---|---|---|---|---|---|---|---|
| qsl-protocol tracked fixture/test outputs only | Covered by repo history once merged | Low; fixture-only | Low with synthetic fixtures | High | Rebuildable evidence only | Authorized for future fixture data | Use for deterministic test fixtures |
| `/srv/qbuild/tmp` temp catalog output | Same-host temp area; not durable authority | Low if unique temp dir and no overwrite | Low if metadata-only | High for local proof | Short-lived proof only | Recommended for first implementation | Select for NA-0388 harness proof |
| `/home/victor/work/qsl/codex/ops/catalog` | Backup coverage not proven in this lane | Medium; durable local ops mutation | Medium if policy fails | Poor for CI | Useful if later backed up | Not authorized now | Defer pending backup-impact review |
| `/home/victor/work/qsl/codex/responses` adjacent catalog | Response archive is same-host continuity covered, but indexes next to responses increase archive surface | High; risks confusing immutable response archive | Medium | Poor for CI | Durable local search value | Not authorized now | Reject for first lane |
| qsl-protocol docs/ops generated catalog | Tracked if committed, but can churn governance | Medium; repo file becomes generated evidence | Medium if extraction policy fails | Medium | Durable but not live-local | Not authorized now | Defer |
| No catalog/manual status quo | No new backup impact | No new mutation | No new secret risk | n/a | Low handoff value | Always available fallback | Reject as first successor because temp-output harness is safe |

NA-0387 itself requires no backup-plan update because it changes only
qsl-protocol governance/evidence/testplan/traceability/journal paths. Future
durable output outside qsl-protocol tracked fixtures or `/srv/qbuild/tmp`
requires explicit backup-impact review.

## Catalog/index fixture strategy

Future NA-0388 fixture coverage should include:

- valid wrapped response file;
- response file missing wrapper;
- response file with D suffix;
- response file with closeout or recovery qualifier;
- response smoke file marker;
- request file;
- directive file if a fixture root is needed;
- journal file;
- ops file;
- malformed file name;
- duplicate inferred directive ID;
- missing metadata;
- secret sentinel file;
- binary or non-UTF8 file;
- unreadable-file simulation if feasible without unstable platform behavior;
- symlink handling;
- parent traversal path;
- root outside the allow-list;
- temp output only;
- JSON summary;
- deterministic human summary.

Fixture roots must be synthetic and must not copy real response bodies.

## Candidate implementation path risk matrix

| Candidate | Value | Risk | Backup impact | Secret risk | CI impact | Dependency/workflow impact | Testability | Authority | Recommendation |
|---|---|---|---|---|---|---|---|---|---|
| Standalone qsl-protocol helper `scripts/ci/qsl_response_history_catalog.py` | Clear single responsibility for metadata cataloging | New helper surface | Low if temp-output only | Manageable with fixtures | Good | No dependency or workflow change needed | High | Future scope can authorize exact file | Select |
| Extend `qsl_codex_response_writer.py` | Reuses response archive knowledge | Blurs writing and indexing responsibilities | Higher if archive paths are nearby | Higher coupling | Medium | No dependency expected | Medium | Not needed for cataloging | Reject |
| Extend `qsl_directive_manifest_validate.py` | Reuses local-ops validation style | Wrong domain; manifest validator should stay focused | Low | Medium | Medium | No dependency expected | Medium | Not appropriate | Reject |
| Extend `qsl_evidence_helper.py` | Centralizes evidence helpers | Large shared helper churn | Low | Medium | Medium | No dependency expected | Medium | Avoid widening shared tool | Reject |
| Shell script | Simple | Fragile parsing and portability | Low | Higher error risk | Medium | POSIX-only constraints apply | Low | Possible but weaker | Reject |
| Local `/srv/qbuild/tools` cataloger | Could work outside repo | Out-of-scope local tool mutation | Unknown | Medium | Poor | Not CI-friendly | Low | Not authorized | Reject |
| Manual-only status quo | No implementation risk | Does not solve handoff friction | None | None | n/a | None | n/a | Always fallback | Reject as successor |

## Routine audit cadence interaction

Catalog/index work is not routine audit cadence. A catalog can support future
audits by making response and local-history metadata easier to inspect, but it
does not establish audit policy, audit recurrence, finding ownership, or release
gate timing.

Routine audit cadence should remain a future governance/local-ops policy lane.
NA-0380 audit findings remain future-lane input. The current queue should not
jump directly to public paper or audit cadence while the bounded catalog harness
is ready.

## First-lane authorization decision

Decision:

`RESPONSE_HISTORY_CATALOG_IMPLEMENTATION_AUTHORIZATION_READY_FOR_TEMP_OUTPUT_HARNESS`

Rationale:

- history roots can be inventoried read-only without mutating archived files;
- no catalog-like local files were present in the inspected roots;
- a standalone helper can use synthetic fixtures and `/srv/qbuild/tmp` output;
- durable real catalog output is unnecessary for the first proof;
- backup coverage gaps for request/ops durable catalog locations can be deferred
  because the selected first lane has no durable local catalog output;
- the same-host backup caveat remains explicit.

## Future allowed/forbidden path bundle

Future NA-0388 allowed paths if live scope selects the implementation harness:

- `scripts/ci/qsl_response_history_catalog.py`
- `inputs/local_ops/response_history_catalog_fixtures/`
- `docs/governance/evidence/NA-0388_qsl_local_ops_response_archive_index_history_catalog_harness.md`
- `tests/NA-0388_qsl_local_ops_response_archive_index_history_catalog_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Future temporary output:

- `/srv/qbuild/tmp/NA0388_response_history_catalog_*`

Future real catalog output:

- forbidden unless future live scope explicitly authorizes the exact durable
  location and backup-impact handling.

Forbidden unless separately authorized:

- `.github/**`;
- `scripts/ci/qsl_codex_response_writer.py`;
- `scripts/ci/qsl_evidence_helper.py`;
- `scripts/ci/qsl_bounded_check_poll.py`;
- `scripts/ci/qsl_directive_manifest_validate.py`;
- workflows;
- Cargo files and dependencies;
- runtime, service, protocol, crypto, auth, state-machine, qsc, qsp, and qshield
  runtime paths;
- qsl-server, qsl-attachments, qsc-desktop, website, README, START_HERE, and
  docs/public paths;
- backup scripts, timers, fstab, services, source lists, targets, keys, restore,
  deploy, rollback, and off-host setup paths;
- durable local catalog paths under `/home/victor/work/qsl/codex/**`;
- response archive file mutation;
- directive, journal, request, or ops-history mutation.

## Audit-finding carry-forward

- GOV-001 backup caveat remains: same-host continuity is not disaster recovery,
  and off-host encrypted backup remains blocked pending deliberate no-secret
  operator input.
- GOV-003 archive/index friction is directly addressed by the selected next
  lane.
- Code and crypto audit findings remain separate future lanes.
- Routine audit cadence remains a recommended future policy lane.
- The public technical position paper remains future-gated on stronger evidence,
  claim discipline, and external-review readiness.

## Governance/security/fail-closed requirements

Future catalog implementation must enforce:

- no archived file mutation;
- no deletion;
- no overwrite;
- no full response body copy;
- no secret content storage;
- no durable catalog writes in the first harness;
- exact root allow-list;
- symlink and path traversal rejection;
- deterministic output ordering;
- JSON and human summaries;
- metadata-only checksums and file metadata;
- bounded content extraction only for approved metadata fields;
- high-confidence secret detection without quoting the matched content;
- explicit backup caveat;
- no public-claim expansion.

## Public-claim/external-review/website boundary

Catalog authorization is not catalog implementation. A future metadata catalog
harness would not prove production readiness, would not prove public-internet
readiness, would not prove external review completion, would not prove off-host
backup completion, would not prove operator response absence resolution, would
not prove code/crypto audit closure, would not prove anonymity, would not prove
metadata-free behavior, would not prove untraceability, and would not prove
disaster recovery completion.

No website, public docs, README, or START_HERE update is authorized by NA-0387.
The public technical position paper remains future-gated.

## Future validation/marker/verification plan

Future NA-0388 should emit or prove:

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
- `NA0388_NO_METADATA_FREE_CLAIM_OK`
- `NA0388_NO_ANONYMITY_CLAIM_OK`
- `NA0388_NO_UNTRACEABLE_CLAIM_OK`
- `NA0388_NO_PRODUCTION_READY_CLAIM_OK`
- `NA0388_NO_PUBLIC_INTERNET_READY_CLAIM_OK`

## Selected successor

Selected:

`NA-0388 -- QSL Local Ops Response Archive Index and History Catalog Implementation Harness`

Rationale:

- the future helper can be standalone and qsl-protocol tracked;
- fixture-only and `/srv/qbuild/tmp` output keeps backup impact low;
- no real durable local catalog is needed for the first proof;
- current history-root gaps can be represented as absent, scanned, or
  coverage-unknown metadata rather than blocking implementation.

Rejected alternatives:

- implementing the catalog in NA-0387;
- writing a durable local catalog now;
- placing an index adjacent to response files;
- mutating response archives;
- changing backup plan now;
- changing workflows now;
- extending response writer, manifest validator, or evidence helper now;
- implementing routine audit cadence now;
- implementing public paper now;
- blocking on off-host backup before a temp-output harness.

## Backup-plan impact statement

No backup-plan update is required for NA-0387 because durable changes are limited
to qsl-protocol governance evidence, testplan, D-0756, traceability, and the
rolling journal. The final D206 response file is the only authorized write under
the Codex response archive and is handled outside the qsl-protocol PR.

Future catalog/index implementation requires backup-plan review if it writes
durable output outside qsl-protocol tracked fixtures or `/srv/qbuild/tmp`,
especially under `/home/victor/work/qsl/codex/ops/catalog` or adjacent to
response archive files.

## Next recommendation

Proceed to NA-0388 as a temp-output implementation harness only. Keep durable
real catalog output, backup-source changes, off-host backup, routine audit
cadence, and public technical position paper work in later explicit lanes.
