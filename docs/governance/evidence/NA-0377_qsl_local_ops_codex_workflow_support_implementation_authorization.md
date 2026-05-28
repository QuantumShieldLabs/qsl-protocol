Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-05-28

# NA-0377 QSL Local Ops Codex Workflow Support Implementation Authorization

Goals: G1, G2, G3, G4, G5

## Executive Summary

NA-0377 authorizes the first bounded local-ops workflow-support lane selected
from NA-0376 evidence. It is governance and authorization evidence only. It
does not change qstart, qresume, helper scripts, response writers, polling
helpers, directive manifests, allow-files, validation profiles, history
indexes, backup scripts, workflows, runtime code, dependencies, service code,
website/public docs, or local backup configuration.

Selected successor:

`NA-0378 -- QSL Local Ops qstart/qresume Fast-Forward Guard Implementation Harness`

Rationale: qstart and qresume source was found in `/srv/qbuild/tools/qshell.sh`;
the startup friction reproduced a stale clean qsl-protocol checkout that needed
a manual fast-forward to the expected `origin/main`; and the local continuity
backup source list includes `/srv/qbuild/tools`. The future lane must still
perform exact local-tool dirty-state, backup-impact, and no-secret checks before
any qshell mutation.

## Live NA-0377 Scope

Live `NEXT_ACTIONS.md` records:

- READY_COUNT: `1`
- READY: `NA-0377 -- QSL Local Ops Codex Workflow Support Implementation Authorization Plan`
- NA-0376: `DONE`
- D-0734: exact ID present once
- D-0735: exact ID present once
- D-0736: exact ID absent at start

Live objective:

- Authorize the first bounded implementation lane for Codex workflow support
  and history indexing without runtime, service, secret, or backup-script drift.

Live protections:

- no runtime, service, protocol, crypto, dependency, or workflow implementation
  unless future NA-0377 live scope explicitly authorizes exact files;
- no secret handling;
- no backup script, timer, or fstab mutation;
- no target setup;
- no public, readiness, or privacy overclaim.

The optional `docs/ops/CODEX_WORKFLOW_SUPPORT_IMPLEMENTATION_AUTHORIZATION.md`
artifact was not added. The live NA-0377 queue entry does not explicitly name
that optional path, so this lane keeps the plan in required evidence only.

## Inherited NA-0376 Plan

NA-0376 planned these local-ops workflow-support areas:

1. qstart/qresume fast-forward and startup hygiene.
2. Machine-readable directive manifest.
3. Response-file writer.
4. Standard validation profiles.
5. Bounded PR/public-safety polling helpers.
6. Per-directive scope-guard allow-files.
7. Read-only source/authority/CI refresh helper.
8. Directive/response/journal index.
9. Claim-boundary scanner.
10. Known-transient CI note.
11. Packet evidence templates.
12. Exact successor block text.
13. Backup coverage for directive/request/response/journal/ops history.
14. No-history-rewrite / no-amend-after-PR guard.
15. Public-safety API/file-list failure recovery procedure.
16. Codex response archive hygiene.
17. D132 bundle status and future cleanup boundary.

NA-0376 priority order placed qstart/qresume first, bounded polling second, and
manifest/allow-file support third. NA-0377 rechecked source, authority, backup
coverage, and risk before selecting the successor.

## Local Workflow-Support History

Read-only history inspected:

- `/home/victor/work/qsl/codex/responses`: present, including D175 through D195.
- `/home/victor/work/qsl/codex/requests`: present, including the workflow
  support request and read-only history request.
- `/home/victor/work/qsl/codex/directives`: absent.
- `/home/victor/work/qsl/codex/journals`: absent.
- `/home/victor/work/qsl/codex/ops`: directory present with backup status
  under `ops/backup` when available.

The latest local continuity manifest inspected was
`daily-20260528T023303-0500`. Its source list includes `/srv/qbuild/tools`,
`/srv/qbuild/work`, `/srv/qbuild/tmp`, Codex responses, and the backup plan file.
It does not include Codex requests, directives, journals, or ops-history roots
as separate sources.

## qstart/qresume Source/Authority/Backup Discovery

Discovery results:

- Plain shell: `command -v qstart` and `command -v qresume` did not find
  standalone commands.
- After sourcing `/srv/qbuild/tools/qshell.sh`: both names are shell functions.
- Source file: `/srv/qbuild/tools/qshell.sh`.
- Current behavior: `qstart` ensures a worktree exists and changes directory;
  `qresume` validates an existing worktree and changes directory. Neither
  proves expected `origin/main` nor fast-forwards a stale clean checkout.
- Startup proof: this NA-0377 worktree was clean but stale on local `main`;
  `origin/main` matched the required `7a419cdf6ddd`, and a manual
  `git merge --ff-only origin/main` was required before live queue files matched
  NA-0377.
- Backup boundary: `/srv/qbuild/tools` appears in the local continuity backup
  source list. This is same-host continuity only, not complete disaster
  recovery.

Classification:

- `QSTART_QRESUME_SOURCE_FOUND`
- `QSTART_QRESUME_AUTHORITY_COMPLETE_FOR_FUTURE_EXACT_DIRECTIVE`
- `QSTART_QRESUME_BACKUP_BOUNDARY_CLEAR_WITH_LOCAL_CONTINUITY_LIMIT`
- `QSTART_QRESUME_IMPLEMENTATION_READY_FOR_FUTURE_HARNESS`

Future NA-0378 must stop if `/srv/qbuild/tools/qshell.sh` is dirty before work,
if the backup source list no longer includes `/srv/qbuild/tools`, or if the
needed change expands beyond qshell without explicit successor scope.

## Bounded Polling Helper Discovery

Existing qsl-protocol surfaces include:

- `scripts/ci/qsl_evidence_helper.py` with check summaries, public-safety
  status, scope guard, link check, leak scan, and PR body preflight helpers.
- `scripts/ci/public_safety_gate.py` with bounded wait helpers and deterministic
  fixture self-tests.
- `scripts/ci/require_pr_checks_green.sh` and `scripts/ci/post_merge_verify.sh`.

Classification:

- `POLLING_HELPER_QSL_PROTOCOL_READY`
- `POLLING_HELPER_AUTHORITY_CLEAR`
- `POLLING_HELPER_BACKUP_IMPACT_LOW`
- `POLLING_HELPER_IMPLEMENTATION_READY`
- `POLLING_HELPER_WORKFLOW_CHANGE_NOT_REQUIRED_FOR_A_HELPER_ONLY_LANE`

Polling helper work is a safe alternate first lane, but it is second priority
because this directive reproduced the qstart/qresume stale-start issue directly.

## Directive Manifest / Allow-File Discovery

Existing evidence:

- `NEXT_ACTIONS.md` contains human-readable directive blocks.
- `scripts/ci/qsl_evidence_helper.py scope-guard` already supports
  `--allowed-file`.
- No canonical directive manifest schema or storage convention exists yet.

Classification:

- `ALLOW_FILE_SCOPE_GUARD_READY`
- `MANIFEST_AUTHORITY_PARTIAL`
- `MANIFEST_BACKUP_BOUNDARY_PARTIAL`
- `MANIFEST_IMPLEMENTATION_BLOCKED_PENDING_STORAGE_AND_AUTHORITY_CHOICE`

Manifest work should not outrank qstart/qresume until the storage location is
selected and backup coverage is explicit. A qsl-protocol-only allow-file helper
lane remains viable later.

## Response Writer Discovery

Response archive evidence:

- `/home/victor/work/qsl/codex/responses` is present.
- D175 through D195 response files are present.
- The local continuity backup source list includes Codex responses.
- The expected collision behavior is `_r2`, `_r3`, and later suffixes.

Classification:

- `RESPONSE_WRITER_AUTHORITY_PARTIAL`
- `RESPONSE_WRITER_BACKUP_BOUNDARY_CLEAR_FOR_RESPONSES`
- `RESPONSE_WRITER_IMPLEMENTATION_BLOCKED_PENDING_EXACT_HELPER_LOCATION`

The response writer is useful but should wait until its helper location is
chosen and secret-output rules are testable.

## History Index / Backup Coverage Discovery

History roots:

- Directives: absent.
- Responses: present and backup-covered by source list.
- Journals: absent.
- Requests: present but not backup-covered by the inspected source list.
- Ops: present/partial but not backup-covered as a separate source.

Classification:

- `HISTORY_INDEX_SOURCE_PARTIAL`
- `HISTORY_INDEX_BACKUP_COVERAGE_PARTIAL`
- `HISTORY_INDEX_AUTHORITY_PARTIAL`
- `HISTORY_INDEX_IMPLEMENTATION_BLOCKED`

History indexing must wait for backup coverage and source/authority decisions.
It must not create an index under `/home/victor/work/qsl/codex` without a later
directive that explicitly authorizes that local history mutation.

## Candidate Implementation-Lane Risk Matrix

| Lane | Value | Risk | Authority | Path category | Backup impact | CI impact | Security impact | Testability | Readiness | Order |
|---|---|---|---|---|---|---|---|---|---|---|
| qstart/qresume fast-forward guard | Prevents stale clean handoff | Local tool mutation can hide state if broad | Clear for exact future directive | `/srv/qbuild/tools/qshell.sh` plus qsl-protocol evidence | Review required; tools source listed | No workflow change | Improves fail-closed startup | High with fixture worktrees | Ready for future harness | 1 |
| bounded polling helper | Reduces watch/API parsing friction | Accepting missing/red checks if wrong | Clear in qsl-protocol | `scripts/ci/**` future exact paths | Low | No workflow change required | Improves CI fail-closed behavior | High with fixtures | Ready alternate | 2 |
| directive manifest + allow-file | Reduces prose parsing and scope typos | Manifest could outrank live governance | Partial | qsl-protocol or local sidecars TBD | Partial | Low | Good if fail-closed | Medium | Blocked for manifest; allow-file ready | 3 |
| response writer | Prevents response filename/wrapper drift | Secret copying or wrong identity | Partial | local helper or qsl-protocol helper TBD | Responses covered | None | Good if no-secret | Medium | Blocked | 4 |
| directive/response/journal index | Improves handoff lookup | Index becoming authority | Partial | local history roots | Partial/high | None | Neutral | Medium | Blocked | 5 |
| validation profiles | Reduces repeated command sets | Superficial pass if too broad | Clear in qsl-protocol | future `scripts/ci/**` | Low | No workflow change | Good if strict | High | Ready later | 6 |
| source/authority helper | Improves cross-repo proof | Accidental mutation if poorly scoped | Partial | qsl-protocol helper likely | Low | None | Good if read-only | High | Later | 7 |
| claim-boundary scanner | Reduces overclaim risk | False negatives/positives | Clear in qsl-protocol | future `scripts/ci/**` | Low | None | Good | High | Later | 8 |
| packet templates | Reduces evidence drift | Template-only evidence | Partial | docs/local template TBD | Partial | None | Neutral | Medium | Later | 9 |
| known-transient CI note | Improves recovery reporting | Normalizing real failures | Clear docs-only | docs/evidence/runbook TBD | Low | None | Good if narrow | High | Later | 10 |
| backup coverage plan | Prevents local history gaps | Could drift into backup config mutation | Partial | governance docs only first | High for future | None | Good | Medium | Prereq for index | 11 |

## First-Lane Authorization Decision

NA-0377 selects qstart/qresume fast-forward guard as the first future lane.

This lane is selected because:

- the source file is exact;
- the stale clean checkout issue occurred during this directive;
- the backup source list includes `/srv/qbuild/tools`;
- the change can be constrained to a local qbuild shell helper plus
  qsl-protocol governance/testplan evidence;
- no secrets, remotes beyond configured Git remotes, workflows, runtime code, or
  backup scripts are needed.

Rejected as first lane:

- bounded polling helper: ready, but it did not address the live startup
  failure reproduced here.
- manifest/allow-file: useful, but manifest storage and backup boundaries are
  not yet complete.
- response writer: response archive backup is clear, but helper authority and
  no-secret output rules need exact location.
- history index: blocked by partial history roots and partial backup coverage.
- backup coverage prerequisite: needed for history/index work, but not required
  before a qshell-only qstart/qresume guard if NA-0378 records local continuity
  limits and backup impact.

## Future Allowed/Forbidden Path Bundle

Selected NA-0378 future allowed paths:

- `/srv/qbuild/tools/qshell.sh` for qstart/qresume expected-main SHA,
  clean-worktree, fetch, and fast-forward fail-closed guard only.
- qsl-protocol governance/evidence/testplan paths for NA-0378 evidence,
  decision, traceability, and rolling journal entries.

Selected NA-0378 future read-only paths:

- `/srv/qbuild/tools/env_qbuild.sh`
- `/srv/qbuild/tools/new_checkout.sh`
- `/srv/qbuild/tools/refresh_mirrors.sh`
- `/home/victor/work/qsl/codex/QSL_BACKUP_PLAN.md`
- `/home/victor/work/qsl/codex/ops/backup/QSL_BACKUP_STATUS.md`
- latest `/backup/qsl/manifests/*.manifest.txt` and `/backup/qsl/logs/*.log`

Selected NA-0378 future forbidden paths/actions unless explicitly widened:

- `/usr/local/sbin/qsl-backup`, systemd timers/services, fstab, backup source
  lists, remote/off-host setup, restore tooling, deploy/rollback tooling, key or
  credential handling, qsl-server, qsl-attachments, qshield runtime, qsc/qsp,
  protocol, crypto, Cargo/dependency, `.github/**`, website/public docs,
  README, START_HERE, and branch-protection/public-safety configuration.

## Backup-Plan and Local-Ops Storage Impact Analysis

NA-0377 itself requires no backup-plan update because this lane changes only
qsl-protocol governance, testplan, traceability, and journal files.

Selected NA-0378 requires a backup-impact review before any qshell mutation:

- `/srv/qbuild/tools` is in the current local continuity backup source list.
- The current backup is same-host continuity only.
- Future qstart/qresume output logs or durable manifests must not be introduced
  without explicit storage and backup coverage rules.
- Requests, directives, journals, and ops-history roots remain partially or not
  covered and must not be indexed before backup coverage is resolved.
- The D132 preservation bundle remains present and must not be deleted without
  explicit authorization.

## Governance/Security/Fail-Closed Requirements

Future qstart/qresume work must:

- require an expected main SHA;
- refuse dirty tracked or untracked worktrees before switching or fast-forward;
- fetch configured remotes only;
- prove `origin/main` equals the expected SHA;
- use only fast-forward behavior;
- avoid force, stash-as-mutation, amend, branch deletion, or hidden cleanup;
- print deterministic evidence;
- reject missing helper source, missing backup source-list coverage, or wrong
  repository;
- use bounded waits only;
- keep all failures explicit and recoverable-failure records complete.

## Public-Claim / External-Review / Website Boundary

NA-0377 local-ops authorization is not runtime implementation, not production
readiness, not public-internet readiness, not external-review completion, not
metadata-runtime claim expansion, not off-host backup completion, not complete
disaster recovery, not operator-response resolution, and not qsl-server or
qsl-attachments production proof.

No website or public docs are changed. The public technical position paper
remains future-gated.

## Future Validation / Marker / Verification Plan

Selected NA-0378 should require:

- `NA0378_QSTART_QRESUME_SOURCE_AUTHORITY_OK`
- `NA0378_QSTART_QRESUME_FAST_FORWARD_GUARD_OK`
- `NA0378_CLEAN_WORKTREE_FAIL_CLOSED_OK`
- `NA0378_EXPECTED_MAIN_SHA_MISMATCH_REJECT_OK`
- `NA0378_NO_DIRTY_OVERWRITE_OK`
- `NA0378_NO_FORCE_OK`
- `NA0378_BACKUP_IMPACT_OK`
- `NA0378_NO_RUNTIME_CHANGE_OK`
- `NA0378_NO_SECRET_MATERIAL_OK`
- `NA0378_NO_METADATA_FREE_CLAIM_OK`
- `NA0378_NO_ANONYMITY_CLAIM_OK`
- `NA0378_NO_UNTRACEABLE_CLAIM_OK`
- `NA0378_NO_PRODUCTION_READY_CLAIM_OK`
- `NA0378_NO_PUBLIC_INTERNET_READY_CLAIM_OK`

Expected validation bundle:

- local qshell fixture tests or equivalent shell-harness proof using disposable
  test repositories under `/srv/qbuild/tmp`;
- queue and decision proof;
- scope guard for qsl-protocol evidence paths;
- local-tool dirty-state proof before mutation;
- backup source-list proof;
- link check, leak scan, classifier proof, goal-lint, cargo audit,
  rustls-webpki proof, qsc send_commit, formal model checks, and qshield-cli
  build/test as feasible.

## Selected Successor

`NA-0378 -- QSL Local Ops qstart/qresume Fast-Forward Guard Implementation Harness`

The successor must implement no NA-0378 work during NA-0377 closeout. Closeout
may only restore the exact successor after Packet S merges and public-safety is
green.

## Backup-Plan Impact Statement

No backup-plan update is required for NA-0377. A future qstart/qresume guard
lane must record backup impact because it may mutate local qbuild tooling. Local
history index, request archive, ops-history, directive archive, journal archive,
manifest, allow-file, and response-writer output work remains backup-coverage
gated.

## Next Recommendation

Proceed to NA-0378 as a qstart/qresume fast-forward guard implementation
harness with exact local-tool scope and qsl-protocol governance evidence. Keep
bounded polling helper work as the next alternate if qshell dirty-state,
authority, or backup coverage fails at NA-0378 startup.
