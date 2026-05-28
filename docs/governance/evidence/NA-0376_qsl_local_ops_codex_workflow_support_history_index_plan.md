Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-05-28

# NA-0376 QSL Local Ops Codex Workflow Support and History Index Plan

Goals: G1, G2, G3, G4, G5

## Executive Summary

NA-0376 converts repeated Codex local-ops workflow-support recommendations into
a staged governance plan. It does not implement qstart/qresume changes, helper
scripts, response writers, polling helpers, validation profiles, allow-files,
directive manifests, claim scanners, history indexes, backup coverage changes,
CI workflow changes, runtime changes, or public-claim changes.

The immediate reason for this planning lane is operational: NA-0375 proved that
the off-host backup target and host-identity path is blocked pending deliberate
no-secret operator input. While that input remains external to Codex, local
workflow-support planning can reduce repeated command-shape risk and improve
handoff confidence without touching runtime, service, protocol, crypto,
backup-script, secret, remote, or website/public-document surfaces.

Selected successor:

`NA-0377 -- QSL Local Ops Codex Workflow Support Implementation Authorization Plan`

NA-0377 is not implemented by NA-0376. It should authorize the first bounded
implementation lane only after this plan, scope boundaries, backup implications,
and validation markers are accepted.

## Live NA-0376 Scope

Live `NEXT_ACTIONS.md` records `NA-0376 -- QSL Local Ops Codex Workflow Support
and History Index Plan` as the sole READY item.

Live objective:

- Produce an authorization/plan for Codex workflow support and
  directive/response/history indexing while the off-host backup target path
  awaits operator input.

Live protections:

- no runtime/service/protocol/crypto/dependency/workflow implementation unless a
  future directive explicitly authorizes exact scope;
- no secret handling;
- no backup script/timer/fstab mutation;
- no target setup;
- no public/readiness/privacy overclaim.

Live acceptance:

1. READY_COUNT 1.
2. READY NA-0376.
3. NA-0375 DONE.
4. D-0732 exists once.
5. D-0733 exists once.
6. D-0734 absent at start.
7. no NA-0376 implementation in closeout.

The optional `docs/ops/CODEX_WORKFLOW_SUPPORT_HISTORY_INDEX_PLAN.md` artifact
was not added in this lane. The live queue objective permits planning, but it
does not explicitly name that additional file path. To keep scope minimal, this
plan is captured in the required evidence and testplan only.

## Inherited NA-0375 Required-Stop Result

NA-0375 recorded `OPERATOR_RESPONSE_REQUIRED_STOP` because final authorized
discovery found no deliberate no-secret operator response candidate for the
off-host backup target-candidate and host-identity chain.

Inherited state:

- qsl-protocol PR #1013 merged as `aada0bc6e0c2`.
- qsl-protocol PR #1014 merged as `cb6d469d6bdb`.
- D-0732 records the NA-0375 required-stop evidence.
- D-0733 records NA-0375 closeout and restores NA-0376 as sole READY.
- The off-host target/host-identity chain remains blocked pending external
  operator input.
- Local same-host backup continuity exists, but it is not complete disaster
  recovery and does not prove off-host encrypted backup.
- qsl-server PR #56 remains bounded harness evidence only at `d40e6003fdf0`.
- qsl-attachments PR #37 remains service-local prerequisite evidence only at
  `96b9352bd63e`.
- qshield embedded relay/demo evidence remains reference/oracle evidence only.

NA-0376 does not alter that blocker. It only records a local workflow-support
plan that can proceed safely while operator input is absent.

## Workflow-Support Request Inventory

Read-only sources inspected:

- `/home/victor/work/qsl/codex/requests/codex_workflow_support_request_20260523.md`
- `/home/victor/work/qsl/codex/requests/directive_response_history_read_access_request_20260523.md`
- response files from D175 through D194 where present
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- local backup plan and status files where present

| Item | Evidence/source | Friction addressed | Risk if poor | Future stage | qsl-protocol change | local-ops change | backup-plan update |
|---|---|---|---|---|---|---|---|
| qstart/qresume fast-forward guard | workflow request item 1; repeated stale clean checkout recoveries | starts from expected main SHA | dirty overwrite or hidden state mutation | NA-0377 priority 1 | likely docs/tests plus helper authorization | yes, qstart/qresume tooling | no for plan; yes if tool outputs become durable history |
| directive manifest | workflow request item 2 | long prose parsing risk | stale manifest overriding live governance | NA-0377 priority 3 | schema/docs/test authorization | yes, directive sidecar storage | yes if manifests are archived/backed up |
| response-file writer | workflow request item 3; required response archives | filename/wrapper drift | secret copying or wrong directive identity | NA-0377 priority 4 | possible tests/docs only if repo helper is chosen | yes, response archive helper | yes for response archive coverage policy |
| validation profiles | workflow request item 4 | repeated validation command sets | false confidence if profiles skip required checks | later implementation lane | possible scripts/tests if authorized | optional | no unless outputs archived |
| bounded polling helpers | workflow request item 5; D175/D178/D179/D194 recoveries | watch-mode avoidance and JSON parsing errors | hiding red required checks or accepting missing checks | NA-0377 priority 2 | possible scripts/tests if authorized | optional | no unless logs archived |
| scope-guard allow-files | workflow request item 6 | long allowed-path command mistakes | overbroad allowlist | NA-0377 priority 3 | helper/test integration if authorized | yes for per-directive files | yes if allow-files are retained |
| source/authority helper | workflow request item 7; repeated qsl-server/qsl-attachments proof | cross-repo status refresh | accidental clone/fetch/mutation or stale authority | later implementation lane | possible helper/tests if authorized | optional | no unless outputs archived |
| directive/response/journal index | workflow request item 8; history read request | handoff lookup and successor trace | index becoming authority over live repo | later or split lane | docs/tests if repo-local spec only | yes, index under codex root | yes |
| claim-boundary scanner | repeated overclaim scans | public/readiness/privacy wording safety | false negatives on unsafe claims | later implementation lane | possible scripts/tests if authorized | optional | no unless reports archived |
| known-transient CI note | D194 GitHub PR-files API 404; prior polling notes | recovered-failure consistency | normalizing real failures as transient | helper/profile doc lane | docs/tests only | optional | no |
| packet evidence templates | repeated final-response/evidence shape | faster evidence assembly | template becomes check-box evidence | later docs/template lane | possible docs/tests | yes if stored under codex | yes if retained |
| successor block text | current directive | queue discipline | inventing successor without authorization | NA-0376 closeout | NEXT_ACTIONS only in closeout | no | no |
| backup coverage for history roots | backup plan/status; latest manifest | prevents response-only coverage gap | secret overcollection or unverified restore | backup prerequisite lane or NA-0377 boundary | docs/test evidence only | yes | yes |
| no-history-rewrite/no-amend guard | prior PR #1010 taint; journal lessons | protects PR evidence truth | blocking safe follow-up commits if too broad | NA-0377 priority 1/3 | possible helper/tests | optional | no |
| public-safety API/file-list recovery | D194 | bounded recovery for API 404 | rerunning true red checks | polling helper lane | possible helper/tests | optional | no |
| response archive hygiene | response duplicate D188 and growing archive | findability and collision clarity | deleting evidence or rewriting responses | index/response-writer lane | docs/tests | yes | yes |
| D132 bundle status/cleanup boundary | D132 bundle present | avoids forgotten local WIP bundle | unauthorized deletion | future explicit cleanup lane | docs/test evidence only | yes if cleanup later | yes if deletion/retention changes |

## Directive/Response/Journal/Request History Availability

Read-only local inspection found:

| Root | Availability | Backup classification | Index classification | NA-0376 action |
|---|---|---|---|---|
| `/home/victor/work/qsl/codex/directives` | ABSENT | BACKUP_COVERAGE_UNKNOWN | SHOULD_BE_INDEXED when created | SHOULD_NOT_BE_MUTATED_IN_NA0376 |
| `/home/victor/work/qsl/codex/responses` | PRESENT | BACKUP_COVERED by current source list and latest manifest match | SHOULD_BE_INDEXED | SHOULD_NOT_BE_MUTATED_IN_NA0376 except required D195 response |
| `/home/victor/work/qsl/codex/journals` | ABSENT | BACKUP_COVERAGE_UNKNOWN | SHOULD_BE_INDEXED when created | SHOULD_NOT_BE_MUTATED_IN_NA0376 |
| `/home/victor/work/qsl/codex/requests` | PRESENT | BACKUP_COVERAGE_UNKNOWN; latest daily manifest did not match this path | SHOULD_BE_INDEXED | SHOULD_NOT_BE_MUTATED_IN_NA0376 |
| `/home/victor/work/qsl/codex/ops` | PARTIAL | BACKUP_COVERAGE_UNKNOWN; latest daily manifest did not match this path | SHOULD_BE_INDEXED selectively | SHOULD_NOT_BE_MUTATED_IN_NA0376 |

Latest daily manifest checked:

`/backup/qsl/manifests/daily-20260528T023303-0500.manifest.txt`

Manifest match summary:

- responses: 1 match
- requests: 0 matches
- directives: 0 matches
- journals: 0 matches
- ops: 0 matches
- backup plan file: 1 match

Future backup-plan review is required before implementing durable directive
indexes, request archives, journal mirrors, ops history preservation, response
writer output conventions, allow-file storage, manifest storage, or cleanup of
local preservation bundles.

## Backup Coverage Review

Current local backup posture:

- `/backup/qsl` is mounted as same-host local continuity storage.
- Daily manifests and logs exist through `daily-20260528T023303-0500`.
- Current backup status records daily sources that include qbuild work, Codex
  responses, and the backup plan file.
- The current local backup is same-host continuity only. It is not complete
  disaster recovery and does not prove off-host encrypted backup.

NA-0376 itself does not require a backup-plan update because its durable changes
are limited to qsl-protocol governance, traceability, testplan, and journal
paths under the repository. Future implementation of local history indexes,
directive manifests, response writer outputs, request archive retention, journal
mirrors, ops-history preservation, or backup source-list changes requires a
separate backup coverage review.

## qstart/qresume Fast-Forward and Startup Hygiene Plan

Future qstart/qresume support should:

1. Require an expected main SHA from the directive manifest or operator handoff.
2. Confirm the worktree is clean before any checkout or fast-forward.
3. Fetch only configured remotes for the target repo.
4. Verify `origin/main` equals the expected SHA.
5. Fast-forward or switch a clean directive branch to the expected SHA only when
   it can do so without overwriting dirty or untracked user work.
6. Fail closed on dirty worktree, missing expected SHA, non-fast-forward need,
   wrong repository, missing helper dependencies, or conflicting live queue
   state.
7. Print evidence: worktree path, branch, HEAD, origin/main, expected SHA, dirty
   status, READY item, decision counts, and helper availability.
8. Avoid force, amend, branch deletion, stash-as-mutation, hidden cleanup, or
   cross-repo mutation.

Required tests:

- clean stale worktree fast-forwards to expected SHA;
- dirty worktree refuses to fast-forward;
- origin/main mismatch fails;
- missing helper is reported;
- mirror-tracking or stale local branch does not become live evidence;
- no forced checkout occurs.

## Directive Manifest and Allow-File Plan

Future manifest fields should include:

- directive ID and target NA;
- expected qsl-protocol main SHA;
- prior response path and expected response filename prefix/suffix;
- mutable repository and read-only repositories;
- allowed mutable paths and forbidden path patterns;
- required checks and public-safety policy;
- required startup identity checks;
- PR branch, commit title, PR title, and PR body fields;
- optional closeout successor block text;
- backup-impact classification;
- stop conditions;
- no-secret and no-public-claim markers;
- response archive destination;
- history/index update authorization flag.

Future allow-files should be generated from the manifest, reviewed as evidence,
and consumed by scope guard. Conflict handling must fail closed when live
`NEXT_ACTIONS.md`, branch protection, GitHub PR state, or repository state
contradicts the manifest.

Required tests:

- manifest validates exact directive/target/SHA;
- forbidden paths cannot be allowed by wildcard accident;
- allow-file rejects unstated paths;
- stale manifest loses to live governance;
- optional closeout block must be exact.

## Response-File Writer Plan

Future response writer support should:

- create the response output directory if needed;
- use America/Chicago response start timestamp in the filename;
- append `_r2`, `_r3`, and later suffixes on collision;
- write a complete `CODEX RESPONSE BEGIN/END` wrapper;
- include required timestamp, directive ID, identity check, summary, branch/PR,
  validation, recovery, scope, public-safety, and response-file proof sections;
- refuse to copy secrets, raw keys, passphrases, tokens, credentials, or
  unredacted sensitive operational output;
- print the final path;
- keep historical responses append-only unless a future directive explicitly
  authorizes archive maintenance.

Required tests:

- first-write path selection;
- collision suffix selection;
- directive ID and target validation;
- section skeleton completeness;
- secret-pattern refusal;
- no mutation outside the response path.

## Bounded Polling Helper and CI Transient Classification Plan

Future polling helpers should use bounded REST polling and must not use watch
mode. They should distinguish PR-head check contexts from push/main contexts.

Required states:

- missing/not attached;
- queued/in progress;
- success;
- neutral/skipped;
- failure/cancelled/timed-out/action-required;
- CodeQL aggregate versus individual analysis jobs;
- docs-only skip behavior;
- public-safety API PR-files 404 or transient endpoint failure.

Transient recovery policy:

- Retry only after an independent read-only endpoint proves the expected data is
  available.
- Use bounded retries: transient GitHub/API/tool invocation up to two retries;
  stale/flaky CI reruns up to two total.
- Never rerun through a real red suite/advisory failure.
- Never bypass public-safety, branch protection, or required checks.

Required tests:

- all-green accepted;
- neutral/skipped accepted only for allowed contexts;
- missing required check times out;
- red required check fails;
- PR-files API 404 is recoverable only after later endpoint success;
- JSON parsing uses stdin or files safely rather than shell-expanded large
  strings.

## Validation Profile Plan

Future validation profiles should be evidence helpers, not authority sources.
Profiles should print commands run, result, elapsed time, and compact evidence
fields.

Profiles:

- `governance-plan`: queue, decisions, scope guard, link-check, leak-scan,
  classifier, goal-lint, cargo audit, rustls-webpki proof, formal smoke as
  needed.
- `governance-closeout`: queue transition, decision uniqueness, scope guard,
  link/leak, dependency health, public-safety proof, goal-lint.
- `docs-only`: docs placement, link integrity, leak-safe wording, goal-lint,
  classifier.
- `runtime-harness`: cargo fmt/test, relevant harnesses, formal/model checks,
  dependency health, no-overclaim scan.
- `cross-repo-read-only`: PR/source/authority/CI proof without mutation.
- `local-ops`: qstart/qresume, response archive, manifest/index, backup impact,
  and no-secret/no-mutation checks.

Failure policy:

- fail closed on red security/dependency/public-safety checks;
- report optional infeasible checks separately;
- do not silently downgrade a profile.

## Source/Authority Helper Plan

Future source/authority helper output should include JSON and human-readable
summaries for read-only repositories:

- repo name;
- default branch;
- remote default branch SHA;
- target PR state, mergedAt, head SHA, merge SHA, title, URL;
- latest main CI status;
- branch protection summary;
- viewer permission;
- open PR summary;
- local path and local SHA if present;
- classifications: `FRESH_SOURCE`, `STALE_SOURCE`, `UNKNOWN_SOURCE`,
  `COMPLETE_AUTHORITY`, `PARTIAL_AUTHORITY`, `BLOCKED_AUTHORITY`,
  `COMPLETE_CI`, `PARTIAL_CI`, `BLOCKED_CI`.

The helper must not fetch, clone, checkout, push, or mutate by default. If
additional freshness is required, a future directive must authorize it exactly.

Required tests:

- merged PR proof;
- absent repo or permission failure;
- stale local path classification;
- branch protection parsing;
- no mutation attempted.

## Claim-Boundary Scanner and Public-Claim Guard Plan

Future scanner behavior:

- scan changed lines and PR body text for high-risk terms;
- classify matches as `unsafe`, `negated`, `prohibited-wording`,
  `future-gated`, or `evidence-caveat`;
- report file and line references;
- require same-line or nearby context for negation;
- fail closed on affirmative unsupported public/readiness/privacy claims;
- integrate with PR body preflight.

High-risk categories:

- unsupported production and public-internet readiness claims;
- unsupported external review completion claims;
- no claims of anonymity, metadata-free behavior, or untraceability;
- no claims of hidden attachment size, hidden timing, hidden traffic shape, or
  all metadata hidden;
- no claims that off-host backup, disaster recovery, or real restore work is
  complete;
- no claims that key custody/recovery is implemented;
- no claims of verified host identity or configured target.

False positives should be resolved by clearer wording, not by weakening the
scanner.

## Directive/Response/Journal Index and Evidence Template Plan

Future index location should be under `/home/victor/work/qsl/codex/` only after
explicit local-ops authorization.

Index schema:

- directive ID;
- NA ID and title;
- directive path;
- response path;
- qsl-protocol PR numbers;
- cross-repo PR numbers;
- branch names;
- head and merge SHAs;
- selected successor;
- closeout state;
- decisions added;
- blockers;
- backup-impact classification;
- response-file status;
- journal mirror status.

Usage rule:

- The index is evidence only.
- It must not override the active directive, live repository state,
  `NEXT_ACTIONS.md`, `DECISIONS.md`, `TRACEABILITY.md`, GitHub PR state, branch
  protection, CI, dependency health, or backup status.

Evidence templates should cover:

- source/authority refresh;
- inherited proof inventory;
- evidence gap matrix;
- strategy option matrix;
- backup/deploy/rollback plan;
- public-claim boundary;
- closeout evidence.

Required tests:

- append-only update;
- duplicate directive detection;
- stale index conflict detection;
- no-secret scanning;
- backup coverage classification.

## Backup Coverage and Local History Preservation Plan

Roots requiring future coverage review:

- directives;
- responses;
- journals;
- requests;
- ops;
- directive manifests;
- allow-files;
- indexes;
- response writer outputs;
- polling/helper logs if retained.

Future backup plan should define:

- inclusion and exclusion lists;
- secret exclusion/redaction policy;
- restore verification for each history root;
- snapshot classification for live versus checkpoint backups;
- retention and cleanup policy;
- off-host encrypted backup dependency;
- D132 preservation bundle retention and cleanup authorization.

No backup script, timer, fstab, service, source list, key, passphrase, restore
target, or off-host backup change is made in NA-0376.

## Governance/Security Boundary and Implementation-Staging Matrix

| Priority | Item | Risk | Future allowed category | Forbidden files/actions | Backup impact | CI/security impact | Evidence needed |
|---|---|---|---|---|---|---|---|
| 1 | qstart/qresume fast-forward guard | stale clean checkout | qbuild/local-ops tooling with tests | dirty overwrite, force, branch deletion | maybe if logs retained | improves startup truth | stale/dirty/mismatch tests |
| 2 | bounded polling helpers | CI false accept | repo or local helper with tests | watch mode, bypass, red-check retry | no unless logs retained | improves branch-protection evidence | missing/red/transient tests |
| 3 | directive manifest and allow-file | stale scope authority | manifest schema, allow-file generation | overriding live governance | yes if retained | improves scope guard | conflict and wildcard tests |
| 4 | response writer | response identity drift | local response helper | secret copying, response rewrite | yes | improves evidence completeness | collision/secret tests |
| 5 | directive/response/journal index | stale index authority | local index plus tests | treating index as authority | yes | improves handoff | append/conflict tests |
| 6 | validation profiles | superficial pass | helper/profile scripts with tests | silent check downgrade | no unless logs retained | improves repeatability | profile failure tests |
| 7 | source/authority helper | cross-repo mutation | read-only helper | clone/fetch/checkout by default | no unless outputs retained | improves source proof | stale/permission tests |
| 8 | claim-boundary scanner | false negatives | helper and PR preflight | weakening public-safety wording | no | improves public-claim guard | unsafe/negated tests |
| 9 | packet evidence templates | boilerplate without proof | docs/templates | replacing real validation | yes if archived | improves handoff | completeness tests |
| 10 | known-transient CI note | overuse of transient label | docs/profile note | rerun true red checks | no | improves recovery discipline | D194-style scenario test |
| 11 | backup coverage plan | history loss | backup prerequisite planning | source-list mutation without auth | yes | protects local ops history | manifest/restore proof |

Recommended successor lane: NA-0377 should authorize implementation planning for
the first bounded group rather than implement all helpers at once.

## Public-Claim / External-Review / Website Boundary

NA-0376 local-ops planning is not:

- production readiness;
- public-internet readiness;
- external review completion;
- metadata runtime claim expansion;
- off-host encrypted backup completion;
- complete disaster recovery;
- operator response intake;
- target setup;
- host identity verification;
- qsl-server production proof;
- qsl-attachments production proof;
- qshield production proof;
- website/public-document work.

No website, public docs, README, or START_HERE update is made.

Public technical position paper work remains future-gated until off-host backup,
real restore/key custody, service production boundaries, monitoring, rollback,
public-claim, and external-review evidence are stronger.

## Future Validation / Marker / Verification Plan

If NA-0377 selects the expected implementation authorization successor, it
should require these markers:

- `NA0377_LOCAL_OPS_WORKFLOW_SUPPORT_PLAN_OK`
- `NA0377_IMPLEMENTATION_AUTHORIZATION_PLAN_OK`
- `NA0377_QSTART_QRESUME_FAST_FORWARD_AUTHORIZATION_OK`
- `NA0377_BOUNDED_POLLING_HELPER_AUTHORIZATION_OK`
- `NA0377_DIRECTIVE_MANIFEST_AUTHORIZATION_OK`
- `NA0377_RESPONSE_WRITER_AUTHORIZATION_OK`
- `NA0377_HISTORY_INDEX_AUTHORIZATION_OK`
- `NA0377_ALLOW_FILE_AUTHORIZATION_OK`
- `NA0377_SOURCE_AUTHORITY_HELPER_AUTHORIZATION_OK`
- `NA0377_CLAIM_SCANNER_AUTHORIZATION_OK`
- `NA0377_BACKUP_COVERAGE_BOUNDARY_OK`
- `NA0377_NO_RUNTIME_CHANGE_OK`
- `NA0377_NO_SECRET_MATERIAL_OK`
- `NA0377_NO_PUBLIC_CLAIM_CHANGE_OK`
- `NA0377_NO_METADATA_FREE_CLAIM_OK`
- `NA0377_NO_ANONYMITY_CLAIM_OK`
- `NA0377_NO_UNTRACEABLE_CLAIM_OK`
- `NA0377_NO_PRODUCTION_READY_CLAIM_OK`
- `NA0377_NO_PUBLIC_INTERNET_READY_CLAIM_OK`

Future validation bundle should include queue/decisions, scope guard,
link-check, leak-scan, claim scan, classifier proof, cargo audit, rustls-webpki
proof, targeted helper tests, and public-safety before and after merge.

## Selected Successor

Selected successor:

`NA-0377 -- QSL Local Ops Codex Workflow Support Implementation Authorization Plan`

Rationale:

- The inventory spans multiple related helpers; a single implementation
  authorization plan can choose the first safe subset without overfitting to one
  helper.
- qstart/qresume startup hygiene and bounded polling have the highest immediate
  risk-reduction value.
- Backup coverage and history index work need explicit boundaries before
  mutation under `/home/victor/work/qsl/codex`.
- The off-host operator-input blocker remains external and should not be
  reselected until deliberate no-secret operator input exists or the Director
  chooses to wait.

## Rejected Alternatives

- `NA-0377 -- QSL Local Ops qstart/qresume Fast-Forward Guard Authorization
  Plan`: useful but too narrow for the broader gathered request inventory.
- `NA-0377 -- QSL Local Ops Bounded CI Polling Helper Authorization Plan`:
  useful but should be coordinated with startup and manifest scope.
- `NA-0377 -- QSL Local Ops Directive Manifest and Allow-File Authorization
  Plan`: useful but depends on startup and response/archive boundaries.
- `NA-0377 -- QSL Local Ops Response Writer Authorization Plan`: useful but
  should include archive and backup coverage boundaries.
- `NA-0377 -- QSL Local Ops Directive / Response / Journal Index Authorization
  Plan`: important but should not proceed before backup coverage is settled.
- `NA-0377 -- QSL Local Ops Backup Coverage Prerequisite Plan`: important but
  can be a required boundary inside the implementation authorization plan unless
  evidence changes.
- `NA-0377 -- Metadata Runtime Off-Host Backup Operator Response Awaiting
  External Input`: rejected because no deliberate no-secret operator input is
  present, so it would loop.

## Backup-Plan Impact Statement

No backup-plan update is required for NA-0376 itself. The durable changes are
qsl-protocol governance, traceability, testplan, and journal files. They remain
within repository history and the qbuild worktree backup scope.

Future work that creates or mutates local directive manifests, request archives,
response-writer outputs, journals, indexes, ops-history mirrors, allow-files,
polling logs, backup source lists, retention policy, restore verification, or
cleanup of preservation bundles requires a backup-plan review first.

## Next Recommendation

Proceed with NA-0377 as an implementation authorization plan. It should choose
the first bounded implementation lane, expected to start with qstart/qresume
fast-forward/startup hygiene and bounded polling support, while explicitly
preserving no runtime/service/protocol/crypto/dependency/workflow drift, no
secret handling, no backup script/timer/fstab mutation, no target setup, no
public-claim expansion, and no history/index mutation without backup coverage.
