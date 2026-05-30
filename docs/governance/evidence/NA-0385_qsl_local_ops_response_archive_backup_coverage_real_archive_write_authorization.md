Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-30

# NA-0385 QSL Local Ops Response Archive Backup Coverage / Real-Archive Write Authorization

Goals: G1, G2, G3, G4, G5

## Executive summary

NA-0385 authorizes a future NA-0386 implementation harness to add a tightly
bounded real response archive output mode to `scripts/ci/qsl_codex_response_writer.py`.

Classification:

`REAL_ARCHIVE_WRITE_IMPLEMENTATION_AUTHORIZATION_READY_WITH_LOCAL_CONTINUITY_CAVEAT`

The authorization is narrow. NA-0385 performs no real response archive write,
does not mutate the response writer, does not create response indexes, does not
change backup configuration, and does not change runtime, workflow, dependency,
service, protocol, crypto, qshield runtime, qsl-server, or qsl-attachments
behavior.

The evidence supports a future one-file no-secret real archive smoke because
the real response archive is present in the current same-host local continuity
backup source list, the latest local continuity snapshot includes the response
archive, and the latest completed D203 response file is present in both live
archive and snapshot with matching checksum. This is not off-host recovery and
must not be described as disaster recovery.

Selected successor:

`NA-0386 -- QSL Local Ops Response Writer Real-Archive Write Implementation Harness`

## Live NA-0385 scope

Live `NEXT_ACTIONS.md` records:

- READY_COUNT `1`.
- READY `NA-0385 -- QSL Local Ops Response Archive Backup Coverage / Real-Archive Write Authorization Plan`.
- NA-0384 DONE.
- D-0750 exists once.
- D-0751 exists once.
- D-0752 absent at startup.
- public-safety remains required and green.

Live objective:

- authorize whether and how the response writer may write to the real Codex
  response archive, including backup coverage, no-secret, no-overwrite, and
  local-history boundaries.

Live protections:

- no runtime/service/protocol/crypto/dependency/workflow implementation unless
  future exact scope authorizes exact files;
- no secret handling;
- no backup script, timer, fstab, or source-list mutation unless future exact
  scope authorizes it;
- no target setup;
- no public, readiness, or privacy overclaim.

NA-0385 live scope is compatible with this authorization-only directive.

## Inherited NA-0384 response writer result

NA-0384 implemented the temp-output response writer harness in
`scripts/ci/qsl_codex_response_writer.py` and fixtures under
`inputs/local_ops/response_writer_fixtures/`.

Inherited properties:

- writes only to explicitly supplied `/srv/qbuild/tmp` output directories;
- validates metadata, timestamps, directive suffixes, required sections, and
  wrapper-relevant body structure;
- rejects high-confidence secret patterns before write;
- uses exclusive-create no-overwrite semantics;
- supports collision-safe `_r2` and `_r3` fixture proof;
- supports dry-run and validate-only no-write modes;
- emits human and JSON summaries;
- rejects real response archive output in NA-0384;
- creates no response, directive, journal, or local-history index.

NA-0384 did not use the helper to write to the real response archive. The final
D203 response file was written normally by Codex at
`/home/victor/work/qsl/codex/responses/NA0384_20260530T011754-0500_D203.md`.

## Real response archive read-only inventory

Read-only inventory of `/home/victor/work/qsl/codex/responses` found:

- directory present, owned by `victor:victor`, mode `0775`;
- 165 files at start;
- response files are regular Markdown files, commonly mode `0600`;
- D190 through D203 are present;
- D203 is present with mode `0600`;
- no response archive subdirectories;
- zero `_rN` collision-suffix files;
- zero response-writer-named files;
- no response index or directive/journal index found under the local Codex
  history root during this review.

Observed recent naming patterns:

- normal response files: `NA0384_20260530T011754-0500_D203.md`;
- recovery/closeout variants: `NA0375_CI_RECOVERY_..._D194.md`,
  `NA0369_CLOSEOUT_..._D186.md`;
- duplicate directive suffixes can exist when timestamps differ, as seen with
  two D188 files for NA-0371.

Implication: a future helper must never rely only on directive suffix for
uniqueness. It must use full filename collision checks, exclusive-create writes,
and collision-safe suffixes.

## Backup coverage / archive durability review

Read-only backup posture:

- `/backup/qsl` is mounted as same-host ext4 storage.
- `/srv/qbuild` had 468 GiB total, 117 GiB used, 328 GiB available at startup.
- `/backup/qsl` had 916 GiB total, 24 GiB used, 884 GiB available at startup.
- `QSL_BACKUP_STATUS.md` records `/home/victor/work/qsl/codex/responses` in the
  current daily source list.
- latest daily manifest/log at review time:
  `/backup/qsl/manifests/daily-20260530T023019-0500.manifest.txt`;
  `/backup/qsl/logs/daily-20260530T023019-0500.log`.
- latest daily log completed successfully with zero deleted files.
- latest daily snapshot includes the response archive directory.
- D203 exists in the latest daily snapshot and in the live archive with matching
  sha256 prefix `d11194f65d7e`.

Coverage classification:

- responses: same-host local continuity covered;
- requests: directory present, but not recorded in the current backup status
  source list inspected for this lane;
- directives: local history directory absent;
- journals: local history directory absent;
- ops: directory present, backup status files present;
- off-host encrypted backup: absent / blocked pending separate operator input;
- disaster recovery: not established.

NA-0385 itself requires no backup-plan update because it changes only
qsl-protocol governance, evidence, testplan, traceability, and journal files.

Future NA-0386 must record pre-write backup-impact evidence. It may prove that
the archive directory is in the same-host continuity source list and that the
latest successful snapshot contains the directory. It must not claim the newly
created future file is already in a snapshot unless a later authorized backup
or subsequent scheduled backup proves that fact.

## Real-archive write authorization options

| Option | Value | Risk | Backup impact | Recommendation |
|---|---|---|---|---|
| Future one-file no-secret generated smoke | Proves helper can safely cross the real archive boundary under explicit gate | Synthetic file becomes durable archive content | Existing same-host continuity covers directory; new file awaits next backup unless separately proven | Recommended |
| Future actual final Codex response only | Avoids synthetic archive artifact | Harder to test collision/negative behavior without risking final response path | Same as above | Defer until helper real-archive mode is proven |
| Block until backup coverage/local history improves | Avoids even a harmless archive artifact | Slows closure despite current response archive source-list coverage | Would require separate blocker lane | Rejected for responses; keep request/index gaps separate |
| Block until off-host backup exists | Strongest durability posture | Blocks local helper correctness on an external operator-input lane | Off-host remains absent | Rejected for one no-secret local smoke; caveat mandatory |
| Keep temp-output only/manual archive writes | Lowest immediate archive mutation | Leaves real-boundary helper untested | No new backup impact | Rejected as insufficient after NA-0384 |

## Future real-archive write boundary design

Future NA-0386 may implement a real archive mode only if all of these are true:

- output directory is exactly `/home/victor/work/qsl/codex/responses`;
- helper requires an explicit CLI flag such as `--allow-real-archive-output`;
- metadata requires `allow_real_archive_output=true`;
- helper fails closed unless both CLI and metadata authorize real archive mode;
- metadata output mode distinguishes real archive output from temp output;
- helper performs the same no-secret scan before opening the output file;
- helper rejects high-confidence secret patterns without printing secret content;
- helper rejects unauthorized output directories;
- helper uses exclusive create and never overwrites existing files;
- helper never deletes, renames, truncates, chmods, or edits existing archive
  files;
- helper writes exactly one authorized file per invocation;
- helper creates no response index, directive index, journal index, or local
  history index;
- helper prints and JSON-emits output path and sha256 checksum;
- helper verifies the file exists after write and checksum matches;
- helper records same-host continuity status only;
- helper does not claim off-host recovery, complete disaster recovery,
  production readiness, public-internet readiness, external review completion,
  anonymity, metadata-free behavior, or untraceability;
- helper real archive mode is never used for hidden background work.

## No-secret / overwrite / collision / retention policy

No-secret policy:

- high-confidence private key, token, credential-label, recovery-envelope,
  raw-credential, and project sentinel patterns reject before write;
- false positives are handled by changing the planned content or by a later
  explicit directive that updates the scanner policy; no silent redaction is
  allowed;
- rejected content must not be written partially and must not be echoed in full.

Overwrite and collision policy:

- existing files are immutable to the helper;
- base filename is attempted first;
- collision suffixes use `_r2`, `_r3`, and monotonic increments before `.md`;
- maximum collision attempts must be finite and documented;
- exhausting collision attempts fails closed without writing;
- exclusive create is required for the final write.

Retention policy:

- a future synthetic smoke file remains a durable audit artifact;
- cleanup/delete is forbidden without a separate explicit directive;
- synthetic file naming should make test status obvious, for example
  `NA0386_REAL_ARCHIVE_SMOKE_<timestamp>_D205.md`, if future helper changes
  support that exact name shape;
- if the future directive chooses actual final-response output instead, it must
  say so explicitly and must not also write a synthetic smoke file.

## Local history / index / routine audit interaction

Real response writer authorization does not solve:

- response archive indexing;
- directive archive absence;
- journal archive absence;
- request/ops backup coverage gaps;
- off-host encrypted backup;
- routine audit cadence;
- public technical position paper readiness;
- qsl-server or qsl-attachments production proof;
- code/crypto audit residuals.

Future real archive mode must not silently become an indexing policy. Response,
directive, and journal index work remains a separate local-ops governance lane.

## Candidate successor risk matrix

| Candidate | Value | Risk | Prerequisites | Recommendation |
|---|---|---|---|---|
| NA-0386 real-archive write implementation harness | Closes the next direct boundary after temp-output helper | One durable no-secret archive artifact | NA-0385 authorization and same-host caveat | Select |
| NA-0386 backup coverage blocker resolution | Useful if response archive coverage were absent | Would defer helper boundary despite observed coverage | Missing/ambiguous backup coverage | Reject for current evidence |
| NA-0386 response archive index plan | Improves local history navigation | Larger governance surface, not required for one-file write proof | Separate index policy | Defer |
| NA-0386 validation profile authorization plan | Useful future local-ops support | Not the direct blocker after NA-0384 | Response writer boundary still pending | Defer |
| NA-0386 routine audit cadence plan | Improves recurring hygiene | Strategic, not immediate | Queue after real archive boundary | Defer |
| Return to off-host operator-response lane | Important for disaster recovery | Still blocked on deliberate no-secret operator input | Operator response | Defer |
| Public technical position paper plan | Useful later public framing | Public-claim risk before local evidence lanes close | More evidence and review | Defer |

## First-lane authorization decision

Decision:

`REAL_ARCHIVE_WRITE_IMPLEMENTATION_AUTHORIZATION_READY_WITH_LOCAL_CONTINUITY_CAVEAT`

Rationale:

- current response archive is locally same-host continuity covered;
- latest successful daily snapshot includes the response archive directory;
- D203 is present in both live archive and latest snapshot with matching
  checksum;
- the future write can be constrained to one no-secret file, no overwrite, no
  delete, no index, exact output directory, explicit dual authorization, and
  checksum/path evidence;
- off-host absence remains true but does not block one local no-secret smoke
  when the caveat is explicit.

## Future allowed / forbidden path bundle

Future NA-0386 allowed paths if live scope selects the implementation harness:

- `scripts/ci/qsl_codex_response_writer.py`
- `inputs/local_ops/response_writer_real_archive_fixtures/`
- `docs/governance/evidence/NA-0386_qsl_local_ops_response_writer_real_archive_write_harness.md`
- `tests/NA-0386_qsl_local_ops_response_writer_real_archive_write_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- exactly one no-secret real archive output file under
  `/home/victor/work/qsl/codex/responses/`, only if the live NA-0386 directive
  explicitly authorizes it.

Forbidden unless separately authorized:

- `.github/**`
- scripts other than the exact response writer path;
- `scripts/ci/qsl_evidence_helper.py`;
- `scripts/ci/qsl_bounded_check_poll.py`;
- `scripts/ci/qsl_directive_manifest_validate.py`;
- workflows;
- Cargo files and dependencies;
- runtime, service, protocol, crypto, auth, state-machine, qsc, qsp, qshield
  runtime, qsl-server, qsl-attachments, qsc-desktop, website, docs/public,
  README, START_HERE;
- backup scripts, timers, fstab, services, source lists, remote targets, keys,
  credentials, restore paths, or monitoring configuration;
- response, directive, journal, or local-history indexes;
- more than one real archive output file.

## Audit-finding carry-forward and routine audit cadence recommendation

Carry-forward:

- real archive authorization addresses part of GOV-003 response-writer/local
  history evidence, but not full indexing;
- response archive coverage depends on GOV-001 same-host backup evidence and
  does not establish off-host recovery;
- code/crypto audit findings are unaffected;
- public technical position paper remains future-gated;
- routine audit cadence should become a future governance/local-ops lane after
  the real archive boundary is proven or explicitly blocked.

## Governance / security / fail-closed requirements

Required future behavior:

- no real archive write in NA-0385;
- no overwrite, deletion, cleanup, truncation, or existing-file edit;
- no secret-bearing write;
- no silent redaction;
- no index mutation;
- no backup configuration mutation;
- no queue mutation outside explicitly authorized closeout;
- no GitHub mutation except normal qsl-protocol PR and CI operations;
- explicit future directive for any real archive write;
- pre-write no-secret scan;
- path and checksum evidence;
- deterministic human and JSON summaries;
- same-host continuity wording only.

## Public-claim / external-review / website boundary

This authorization is not:

- real archive write implementation;
- production readiness;
- public-internet readiness;
- external review completion;
- off-host backup completion;
- disaster recovery completion;
- anonymity, metadata-free, or untraceable behavior;
- qsl-server or qsl-attachments production proof;
- code/crypto audit closure;
- operator-response completion.

No website or docs/public update is authorized by NA-0385.

## Future validation / marker / verification plan

Future NA-0386 markers:

- `NA0386_REAL_ARCHIVE_WRITE_AUTHORIZATION_OK`
- `NA0386_RESPONSE_ARCHIVE_BACKUP_COVERAGE_OK`
- `NA0386_LOCAL_CONTINUITY_CAVEAT_OK`
- `NA0386_REAL_ARCHIVE_WRITE_HELPER_OK`
- `NA0386_ALLOW_REAL_ARCHIVE_EXPLICIT_OK`
- `NA0386_REAL_ARCHIVE_NO_SECRET_SCAN_OK`
- `NA0386_REAL_ARCHIVE_NO_OVERWRITE_OK`
- `NA0386_REAL_ARCHIVE_COLLISION_OK`
- `NA0386_REAL_ARCHIVE_PATH_CHECKSUM_OK`
- `NA0386_NO_INDEX_MUTATION_OK`
- `NA0386_NO_DELETE_OK`
- `NA0386_BACKUP_IMPACT_OK`
- `NA0386_NO_WORKFLOW_CHANGE_OK`
- `NA0386_NO_DEPENDENCY_CHANGE_OK`
- `NA0386_NO_RUNTIME_CHANGE_OK`
- `NA0386_NO_SECRET_MATERIAL_OK`
- `NA0386_NO_METADATA_FREE_CLAIM_OK`
- `NA0386_NO_ANONYMITY_CLAIM_OK`
- `NA0386_NO_UNTRACEABLE_CLAIM_OK`
- `NA0386_NO_PRODUCTION_READY_CLAIM_OK`
- `NA0386_NO_PUBLIC_INTERNET_READY_CLAIM_OK`
- `NA0386_NO_DISASTER_RECOVERY_COMPLETE_CLAIM_OK`

Validation bundle should include helper fixture proof, one authorized real
archive smoke or actual response proof, path/checksum proof, archive inventory
before/after, backup coverage read-only proof, scope guard, leak scan,
overclaim scan, queue/decisions proof, and required CI proof.

## Selected successor

Selected successor:

`NA-0386 -- QSL Local Ops Response Writer Real-Archive Write Implementation Harness`

Rationale:

- it is the direct next boundary after NA-0384 temp-output implementation;
- response archive same-host continuity coverage is sufficient for one harmless
  no-secret smoke when caveated;
- the boundary can be implemented and tested without runtime, workflow,
  dependency, backup-script, off-host, qsl-server, or qsl-attachments mutation.

## Rejected alternatives

- Writing a real archive file in NA-0385: rejected because NA-0385 is an
  authorization/planning lane.
- Creating a response index now: rejected because indexing policy is separate.
- Changing backup plan/source lists now: rejected because current evidence is
  enough for one future no-secret smoke and backup mutation is out of scope.
- Blocking until off-host backup exists: rejected for one local no-secret smoke,
  but off-host absence remains a hard caveat.
- Implementing public paper now: rejected as future-gated public framing.

## Backup-plan impact statement

NA-0385 requires no backup-plan update. Durable changes are limited to tracked
qsl-protocol governance, testplan, traceability, and journal files.

Future NA-0386 must not change backup scripts, timers, fstab, source lists,
services, targets, keys, credentials, restore paths, or monitoring. It may rely
only on read-only backup coverage evidence unless a later exact directive
authorizes backup mutation.

## Next recommendation

Proceed to NA-0386 as a one-file real response archive implementation harness
with explicit real-archive authorization, same-host continuity caveat, no-secret
pre-write rejection, exclusive-create no-overwrite behavior, collision-safe
naming, path/checksum proof, no index mutation, and no runtime/workflow/
dependency drift.
