Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-05-28

# NA-0379 QSL Local Ops Bounded CI Polling Helper Implementation Authorization

Goals: G1, G2, G3, G4, G5

## Executive Summary

NA-0379 authorizes a future qsl-protocol bounded CI/public-safety polling helper
implementation lane. It does not implement the helper, change workflows, change
`public_safety_gate.py`, change `qsl_evidence_helper.py`, change dependencies,
change runtime code, mutate local qbuild tools, or alter public/readiness/privacy
claims.

Decision:

`POLLING_HELPER_IMPLEMENTATION_AUTHORIZATION_READY`

Selected successor:

`NA-0380 -- QSL Local Ops Bounded CI Polling Helper Implementation Harness`

The authorized first lane is a standalone repo-local Python helper:

`scripts/ci/qsl_bounded_check_poll.py`

The future helper must be fixture-tested, bounded, fail-closed for red required
checks, explicit about PR-vs-push contexts, and report exact remaining or failed
checks with URLs. It must not use watch mode, bypass branch protection, suppress
red checks, mutate workflows, or mutate public-safety policy.

## Live NA-0379 Scope

Live `NEXT_ACTIONS.md` records:

- READY_COUNT: `1`
- READY: `NA-0379 -- QSL Local Ops Bounded CI Polling Helper Implementation Authorization Plan`
- NA-0378: `DONE`
- D-0738: present once
- D-0739: present once
- D-0740: absent at start

Live objective:

- Authorize the next bounded local-ops workflow-support lane for
  CI/public-safety polling helper implementation without workflow, runtime, or
  dependency drift.

Live protections:

- no runtime, service, protocol, crypto, dependency, or workflow implementation
  unless future live scope explicitly authorizes exact files;
- no secret handling;
- no backup script, timer, or fstab mutation;
- no target setup;
- no public, readiness, or privacy overclaim.

NA-0379 is authorization-only. The optional
`docs/ops/CODEX_BOUNDED_CI_POLLING_HELPER_IMPLEMENTATION_AUTHORIZATION.md`
artifact was not added because the live queue entry does not explicitly name
that path. This evidence file is the planning artifact for NA-0379.

## Inherited NA-0378 Result

NA-0378 delivered the bounded qstart/qresume fast-forward guard outside
the repo in `/srv/qbuild/tools/qshell.sh`, then restored NA-0379 as the sole
READY successor.

Inherited proof:

- PR #1019 merged at `488e07defec4`.
- PR #1020 merged at `2268ed440c05`.
- qshell original SHA-256:
  `6b74ed7a7903ac1574ead2df80c285f06c8550f447c745407d12c741161d339a`
- qshell new SHA-256:
  `7200e968f1b1d70a106aba1043b48739eb44eeb592058e74930002f5fc915f3b`
- rollback backup:
  `/srv/qbuild/tools/backups/NA0378/qshell.sh.20260528T200427-0500.6b74ed7a7903.bak`
- proof log:
  `/srv/qbuild/tmp/NA0378_qstart_qresume_guard_20260528T200719-0500/harness.log`

NA-0378 proved clean stale fast-forward, already-current no-op, dirty tracked
reject, untracked reject, expected-SHA mismatch reject, diverged reject, no
dirty overwrite, no force, and no reset-hard behavior. NA-0379 does not mutate
qshell again.

## Polling-Friction Inventory

| Friction | Source evidence | Cause | Failure mode | Desired helper behavior | Fail-closed behavior |
|---|---|---|---|---|---|
| Python heredoc / here-string mistakes | D179, D185, D197, rolling journal | Shell stdin was split between Python code and JSON payload | JSON was parsed as Python or stdin was empty | Parse JSON via `subprocess.run(..., capture_output=True)` or files/stdin with one clear data channel | Exit nonzero with parser metadata, no repository mutation |
| `gh pr checks` nonzero while checks pending | D186, D197, rolling journal | CLI treats queued/missing checks as failure | Nonzero looked like a hard red state | REST polling should distinguish missing, queued, in_progress, and completed | Timeout nonzero only after bounded wait, red nonzero immediately |
| Public-safety attaches late | D185, D188, D197 | Push and public-safety aggregate can attach after other jobs | Closeout blocked or helper ambiguous while suites were still running | Poll by SHA until public-safety is attached and completed, within budget | Timeout with exact remaining checks |
| PR contexts vs push contexts | D174, D197, rolling journal | PR required contexts and push check-run names differ | `checks-summary --sha` reported PR-only contexts missing on merge commits | Separate PR-head gate from push/main report-only summaries | Missing PR-only context on push SHA is report-only, not a merge gate |
| CodeQL neutral/skipped behavior | D186, rolling journal | CodeQL may report neutral for accepted conditions | False red if neutral is not policy-aware | Accept CodeQL neutral only under explicit policy; report skipped/neutral distinctly | CodeQL failure/cancelled/timed_out/action_required fail closed |
| Docs-only skip acceptance | D179, D186, D197 | Full suites are intentionally skipped for docs/governance-only PRs | False red when qsc full suites skip | Require classifier evidence before accepting docs-only full-suite skip | If classifier is missing/ambiguous, do not accept skip |
| Public-safety PR-files API 404 | D193/D194 and NA-0376/NA-0377 evidence | GitHub endpoint transiently failed while other data later succeeded | Public-safety failed before path scan proof | Treat as retryable only when independent endpoint later proves PR metadata/files reachable | If still 404 after retry budget, fail with exact endpoint metadata |
| Long qsc-linux and macOS full serial checks | D185, D188, D179 | Long-running push checks can outlive normal directive window | Main post-merge proof timed out | Bounded polling with remaining check names and URLs | Stop after cap; do not claim green |
| Stale failed check-runs after rerun | public_safety_gate fixture and journal | Older failed run remains on same SHA after rerun | Naive first-by-name sees stale failure | Select latest by run id/timestamps per check name and retain stale-run summary | Latest red fails; conflicting stale evidence is reported |
| Job-level vs workflow-level rerun ambiguity | rolling journal D174/D194 | Different tools rerun failed jobs vs workflows | Overbroad rerun or unsupported rerun path | Provide rerun guidance only; do not auto-rerun unless future directive allows | No unlimited reruns; red remains red |
| Shell JSON handoff / argument-size issues | D179 | Large JSON passed through environment/argv | Argument-size limit or truncation | Use files/stdin/subprocess JSON, never large shell-expanded literals | Exit with parser/source error |
| `mergeStateStatus` timing | prior CI handoffs | GitHub mergeability lags required checks | Premature merge attempt or dirty/unknown confusion | Report merge state as advisory beside check results | Never merge if required checks are red/missing |
| `public-safety-status --sha` vs `--ref` usage | D197 | Helper expects concrete SHA | Nonzero command-shape failure | Future helper should accept PR, branch, or SHA and print resolved target | Ambiguous target exits nonzero before polling |
| Goal-lint synthetic event usage | D190, D197, rolling journal | Local goal-lint requires event JSON | Malformed synthetic JSON | Prefer PR body preflight plus generated event through JSON encoder | Malformed event exits nonzero with path and field error |

## Existing Helper Surface Discovery

Read-only qsl-protocol helper surfaces:

- `scripts/ci/qsl_evidence_helper.py`
  - queue and decision parsers;
  - scope guard;
  - check-run summary for PR or SHA;
  - public-safety status by SHA;
  - link check, leak scan, PR body preflight, CI admission preflight.
- `scripts/ci/public_safety_gate.py`
  - PR file discovery;
  - branch required-check discovery;
  - public-safety/advisory gates;
  - bounded commit-check wait helper;
  - deterministic timeout-resilience fixtures.
- `scripts/ci/classify_ci_scope.sh`
  - docs-only, workflow-security, runtime-critical classifier.
- `scripts/audit/run_goal_lint_pr.sh` and `tools/goal_lint.py`
  - PR body and changed-file goal-lint support.
- `.github/workflows/**`
  - PR vs push conditions;
  - docs-only full-suite skips;
  - `public-safety` required aggregation.

Classifications:

- `POLLING_HELPER_QSL_PROTOCOL_READY`
- `POLLING_HELPER_AUTHORITY_CLEAR`
- `POLLING_HELPER_BACKUP_IMPACT_LOW`
- `POLLING_HELPER_IMPLEMENTATION_READY` for a future exact helper lane only
- `POLLING_HELPER_WORKFLOW_CHANGE_NOT_REQUIRED`
- `POLLING_HELPER_SHOULD_NOT_MUTATE_PUBLIC_SAFETY_GATE`

Changing `public_safety_gate.py` or workflows is not required for the first
polling-helper lane. The future helper should consume GitHub REST data and local
fixtures without changing the public-safety gate.

## Public-Safety / Check-Run Context Model

| Target | Gate role | Accepted states | Retryable states | Rejected states | Notes |
|---|---|---|---|---|---|
| PR head required checks | Merge gate | required contexts completed success; CodeQL neutral only under explicit policy | missing, queued, in_progress until cap | failure, cancelled, timed_out, action_required, red CodeQL, missing after cap | Exclude `public-safety` only if public-safety is evaluated through its own gate on same PR head |
| PR head all checks | Diagnostic | success, neutral, skipped reported by name | missing/queued/in_progress until cap in all-check mode | any red completed check in strict mode | Report-only mode may summarize without gating |
| Public-safety on PR head | Required gate | completed success | missing, queued, in_progress until cap | completed red, timeout, API failure after retry budget | Public-safety red is never treated as green |
| Merge commit / main push | Post-merge proof | public-safety completed success; full suites success or docs-only skipped when classifier supports | delayed attach, queued, in_progress | public-safety red, required push checks red, timeout | PR-only goal-lint/CodeQL aggregate names may be missing on push SHA and should be report-only |
| qsc-linux-full-suite | Push/full-suite proof | success; skipped only for docs-only scope | queued/in_progress | red, missing after cap when required by scope | Not a PR critical path for docs-only PRs |
| macos-qsc-full-serial | Push/full-suite proof | success; skipped only for docs-only scope | queued/in_progress | red, missing after cap when required by scope | Long-running; report URL |
| qsc-adversarial-smoke | Push/full-suite proof | success; skipped only if workflow policy permits | queued/in_progress | red, missing after cap when required by scope | Should remain visible in summaries |
| PR files API | Scope evidence | successful file list | transient 404/5xx within retry budget | persistent 404/5xx or malformed body after retry budget | 404 recoverable only if independent endpoint later succeeds |
| Branch protection required contexts | Authority | fetched required-context list includes `public-safety` | transient API errors | missing `public-safety`, force/deletion protection weakened | Helper reports, does not mutate |
| mergeStateStatus | Advisory | CLEAN/HAS_HOOKS as informational when checks green | UNKNOWN/BLOCKED/PENDING during attach | DIRTY with failing checks | It never overrides required checks |

## Bounded Polling Helper Semantics

Future helper semantics:

- support PR required-check polling;
- support all-check PR summaries;
- support public-safety polling by SHA;
- support post-merge main public-safety polling;
- resolve target mode explicitly: PR head, SHA, branch, or report-only push;
- print resolved SHA, PR number when present, branch when present, interval,
  max iterations, and elapsed iteration count;
- use REST polling, never `gh pr checks --watch` or any watch mode;
- no infinite loops, no background processes, no promises to keep watching;
- accept neutral/skipped only by explicit policy and context;
- accept docs-only full-suite skips only with classifier evidence;
- accept CodeQL neutral only under explicit policy;
- fail nonzero for red required checks;
- fail nonzero for timeout;
- exit zero only when the required state is met;
- print job/check URLs for failures and pending checks;
- print remaining queued/in-progress/missing checks at timeout;
- select latest check-run by id/timestamps per check name and report stale
  conflicting runs when present;
- use JSON via `subprocess` capture, stdin, or files, not shell-expanded JSON;
- provide rerun guidance but perform no automatic unlimited reruns;
- never mutate workflows, branch protection, public-safety policy, or repo files.

## Red-Check / Timeout / Skip / API-Failure Policy

- Red required check: fail closed immediately with name, conclusion, URL, and
  resolved target SHA.
- Pending or missing required check: wait bounded iterations, then fail with
  remaining names and URLs if any exist.
- Timeout: nonzero exit and exact `ITER=max/max` evidence.
- Docs-only skipped full suites: accepted only when classifier reports
  `docs_only=true` for the exact changed path set.
- CodeQL neutral: accepted only under the existing explicit neutral policy.
- CodeQL skipped: report; accept only if the target mode/policy allows skipped
  optional CodeQL output.
- CodeQL red: fail closed.
- Public-safety red: fail closed unless a later directive separately authorizes
  a bounded rerun for a proven transient.
- Public-safety in_progress: bounded wait.
- API 404 on PR files: retryable only within budget and only if independent PR
  metadata/check endpoints later succeed; otherwise fail with endpoint metadata.
- Missing PR-only context on push SHA: report-only if PR-head required checks and
  public-safety were green before merge.
- Stale failed check-run after rerun: latest run determines gate; stale failure
  remains visible in audit output.

## Future Helper Path and Test Strategy Authorization

Authorized first implementation path for NA-0380:

- `scripts/ci/qsl_bounded_check_poll.py`

Required supporting paths:

- `tests/NA-0380_qsl_bounded_ci_polling_helper_testplan.md`
- `docs/governance/evidence/NA-0380_qsl_local_ops_bounded_ci_polling_helper_harness.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Fixture paths may be used only if the future directive explicitly authorizes
them:

- `inputs/local_ops/qsl_bounded_check_poll_fixtures/*.json`

Test strategy:

- unit tests use mock JSON fixtures and no GitHub network;
- fixture coverage must include all-green, red required, pending timeout,
  missing context, docs-only skip, CodeQL neutral, public-safety success/red,
  API 404 metadata, PR-vs-push context distinction, stale failed after rerun,
  and malformed JSON;
- optional read-only integration against current PR/merge SHAs may be added only
  if future live scope permits;
- tests must prove deterministic exit codes and failure summaries.

## Candidate Implementation Risk Matrix

| Candidate | Value | Risk | Path category | Backup impact | CI impact | Security impact | Testability | Dependency/workflow impact | Status |
|---|---|---|---|---|---|---|---|---|---|
| Standalone `scripts/ci/qsl_bounded_check_poll.py` | Focused helper, low coupling, fixture-friendly | New helper must duplicate some REST/check logic | qsl-protocol script | Normal repo tracking; temp logs only | No workflow change required | Improves fail-closed CI evidence if strict | High | No new dependency, no workflow edit | Recommended |
| Add subcommands to `qsl_evidence_helper.py` | Reuses existing helper surface | Grows already broad governance helper and risks accidental behavior drift | existing qsl-protocol script | Normal repo tracking | No workflow change required | Good if scoped | Medium/high | No dependency/workflow edit | Alternative only if future directive chooses it |
| Shell helper | Simple CLI | JSON parsing and quoting friction repeats known failures | qsl-protocol script | Normal repo tracking | No workflow change required | Risk of brittle parsing | Medium/low | No dependency/workflow edit | Rejected for first lane |
| Local `/srv/qbuild/tools` helper | Close to operator workflow | Local tool mutation and backup/rollback review needed | local qbuild tool | Higher local backup-impact review | No repo CI by default | Good only with local harness | Medium | Outside qsl-protocol CI | Rejected for first lane |
| No helper / continue manual polling | No code change | Repeats known command-shape failures and inconsistent evidence | none | none | none | Leaves risk unchanged | none | none | Rejected |

## First-Lane Authorization Decision

NA-0379 authorizes the standalone qsl-protocol helper lane for NA-0380.

Required classification:

`POLLING_HELPER_IMPLEMENTATION_AUTHORIZATION_READY`

Rationale:

- prior evidence shows repeated bounded-polling command-shape friction;
- qsl-protocol already has REST/check-run helper patterns;
- a standalone Python helper can be fixture-tested without network;
- no workflow mutation is required;
- no public-safety gate mutation is required;
- backup impact is normal qsl-protocol source tracking if durable artifacts stay
  in repo-approved paths and temporary logs remain under `/srv/qbuild/tmp`.

## Future Allowed / Forbidden Path Bundle

Future NA-0380 allowed paths:

- `scripts/ci/qsl_bounded_check_poll.py`
- `tests/NA-0380_qsl_bounded_ci_polling_helper_testplan.md`
- `docs/governance/evidence/NA-0380_qsl_local_ops_bounded_ci_polling_helper_harness.md`
- `inputs/local_ops/qsl_bounded_check_poll_fixtures/*.json` only if exact future
  live scope authorizes fixtures
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Possible alternative allowed path only if future live scope chooses the
subcommand approach:

- `scripts/ci/qsl_evidence_helper.py`

Forbidden unless separately authorized:

- `.github/**`
- `scripts/ci/public_safety_gate.py`
- Cargo files and dependencies
- runtime, service, protocol, crypto, auth, state-machine, key schedule, qsc,
  qsp, qsl, qshield runtime, qsc-desktop, apps runtime, tools/refimpl
- qsl-server and qsl-attachments
- backup scripts, timers, fstab, system services, source lists
- website, external website, README, START_HERE, docs/public
- `/srv/qbuild/tools/**`
- `/home/victor/work/qsl/codex/**`

## Backup-Plan and Local-Ops Storage Impact Analysis

NA-0379 itself requires no backup-plan update because durable changes are
qsl-protocol governance, traceability, testplan, and rolling-journal files under
normal repo tracking.

Future NA-0380 requires no backup-plan update if:

- the helper lives in qsl-protocol;
- fixtures are tracked only under an explicitly authorized repo path;
- proof logs remain temporary under `/srv/qbuild/tmp`;
- no local history, response archive, backup source list, qbuild tool, service,
  timer, fstab, off-host target, key, credential, backup, restore, deploy, or
  rollback path is changed.

Future backup-plan review is required if helper outputs become durable outside
repo-authorized paths or `/srv/qbuild/tmp`.

The current local backup remains same-host continuity only, not complete
disaster recovery.

## Governance / Security / Fail-Closed Requirements

NA-0380 must require:

- bounded iterations and bounded sleep;
- no watch mode;
- no infinite loop;
- deterministic exit codes;
- no red-check suppression;
- no branch-protection bypass;
- no admin bypass;
- no merge if required checks are red/missing after timeout;
- exact failure summaries and job URLs;
- JSON parsing through structured subprocess/file/stdin handling;
- no shell argument-size failure pattern;
- no hidden mutation;
- no secrets in logs;
- no workflow changes;
- no new dependencies;
- fixtures proving each accepted, retryable, rejected, and timeout state.

## Public Claim / External Review / Website Boundary

NA-0379 is authorization planning only. Future NA-0380 helper implementation
would be local-ops CI evidence tooling only.

This lane does not prove:

- production readiness;
- public-internet readiness;
- external review completion;
- no metadata-free behavior claim;
- no anonymity claim;
- no untraceable-behavior claim;
- hidden attachment size;
- hidden timing;
- hidden traffic shape;
- off-host backup completion;
- disaster recovery completion;
- operator response availability;
- qsl-server or qsl-attachments production readiness.

No website, public docs, README, or START_HERE update is made. The public
technical position paper remains future-gated.

## Future Validation / Marker / Verification Plan

Future NA-0380 should prove:

- `NA0380_POLLING_HELPER_AUTHORIZATION_OK`
- `NA0380_PR_CHECK_POLLING_OK`
- `NA0380_PUBLIC_SAFETY_SHA_POLLING_OK`
- `NA0380_POST_MERGE_PUBLIC_SAFETY_POLLING_OK`
- `NA0380_PR_VS_PUSH_CONTEXT_OK`
- `NA0380_DOCS_ONLY_SKIP_POLICY_OK`
- `NA0380_CODEQL_NEUTRAL_POLICY_OK`
- `NA0380_RED_CHECK_FAILS_CLOSED_OK`
- `NA0380_TIMEOUT_FAILS_CLOSED_OK`
- `NA0380_API_FAILURE_REPORTING_OK`
- `NA0380_STALE_FAILED_RERUN_REPORTING_OK`
- `NA0380_NO_WATCH_MODE_OK`
- `NA0380_NO_WORKFLOW_CHANGE_OK`
- `NA0380_NO_DEPENDENCY_CHANGE_OK`
- `NA0380_NO_RUNTIME_CHANGE_OK`
- `NA0380_NO_SECRET_MATERIAL_OK`
- `NA0380_NO_METADATA_FREE_CLAIM_OK`
- `NA0380_NO_ANONYMITY_CLAIM_OK`
- `NA0380_NO_UNTRACEABLE_CLAIM_OK`
- `NA0380_NO_PRODUCTION_READY_CLAIM_OK`
- `NA0380_NO_PUBLIC_INTERNET_READY_CLAIM_OK`

## Selected Successor

Selected successor:

`NA-0380 -- QSL Local Ops Bounded CI Polling Helper Implementation Harness`

## Rejected Alternatives

- Implementing the helper in NA-0379: rejected as out of scope.
- Modifying `public_safety_gate.py` now: rejected; first lane can be standalone.
- Modifying workflows now: rejected; helper-only lane needs no workflow change.
- Using shell as the first helper: rejected because prior failures were largely
  shell/JSON handoff failures.
- Adding local `/srv/qbuild/tools` polling helper first: rejected because
  qsl-protocol fixture testing and repo tracking are safer for this lane.
- Continuing manual polling: rejected because it preserves known friction.
- Website/public-claim work now: rejected as out of scope.
- Public technical paper now: rejected as future-gated.

## Backup-Plan Impact Statement

No backup-plan update is required for NA-0379. Future helper logs or artifacts
must remain temporary unless a later directive authorizes durable storage and
backup coverage review.

## Next Recommendation

Proceed to NA-0380 with the standalone qsl-protocol helper path, fixture-first
tests, no workflow edits, no public-safety gate mutation, no dependency change,
and fail-closed required-check semantics.
