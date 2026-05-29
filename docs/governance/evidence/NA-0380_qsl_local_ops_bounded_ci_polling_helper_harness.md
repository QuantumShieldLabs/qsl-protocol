Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-29

# NA-0380 QSL Local Ops Bounded CI Polling Helper Harness

Goals: G1, G2, G3, G4, G5

## Executive Summary

NA-0380 implements the standalone bounded CI/public-safety polling helper
authorized by NA-0379:

`scripts/ci/qsl_bounded_check_poll.py`

The helper uses only Python standard library modules, calls GitHub only through
`gh api` in live mode, provides deterministic no-network fixture mode, reports
red/pending/missing/API states with check names and URLs, and exits nonzero for
red required checks, missing public-safety, malformed fixtures, API failures
after the bounded retry window, and timeouts.

No workflow, dependency, runtime, `public_safety_gate.py`,
`qsl_evidence_helper.py`, qbuild tool, backup script/timer/fstab, qsl-server,
qsl-attachments, website/public docs, README, START_HERE, branch protection,
public-safety configuration, secret, target, restore, deploy, rollback, or
public-claim surface is changed.

## Live NA-0380 Scope

Live `NEXT_ACTIONS.md` records:

- READY_COUNT `1`.
- READY `NA-0380 -- QSL Local Ops Bounded CI Polling Helper Implementation Harness`.
- NA-0379 DONE.
- D-0740 once.
- D-0741 once.
- D-0742 absent at start.

Allowed implementation/evidence paths for this directive:

- `scripts/ci/qsl_bounded_check_poll.py`
- `inputs/local_ops/qsl_bounded_check_poll_fixtures/*.json`
- `docs/governance/evidence/NA-0380_qsl_local_ops_bounded_ci_polling_helper_harness.md`
- `tests/NA-0380_qsl_local_ops_bounded_ci_polling_helper_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Inherited NA-0379 Authorization

NA-0379 selected a standalone qsl-protocol Python helper under
`scripts/ci/qsl_bounded_check_poll.py`, with fixture-first tests and no workflow,
dependency, runtime, public-safety gate, qsl_evidence_helper, backup, secret,
sibling-repo, website, or public-claim mutation.

Inherited evidence:

- PR #1021 merged at `9d73b62f7d62`.
- PR #1022 merged at `3aa72a5eb69d`.
- D-0740 records the authorization decision.
- D-0741 records NA-0379 closeout and NA-0380 restoration.
- NA-0379 required explicit PR-vs-push, docs-only, CodeQL neutral/skipped,
  red-check, timeout, API-failure, and stale-rerun handling.

## Helper Design Summary

Implemented CLI:

- `pr --repo OWNER/REPO --pr N --required --interval S --max-iters N`
- `pr --repo OWNER/REPO --pr N --all --interval S --max-iters N`
- `public-safety --repo OWNER/REPO --sha SHA --interval S --max-iters N`
- `sha-summary --repo OWNER/REPO --sha SHA --report-only`
- `fixture --fixture PATH --policy POLICY_NAME`

Common behavior:

- bounded iteration count and bounded optional sleep;
- no watch mode;
- no PR, branch, run, workflow, or branch-protection mutation;
- latest check-run selected by timestamp/run/check-run id;
- stale red reruns preserved in summary;
- human-readable summary by default;
- machine-readable JSON summary via `--json`;
- explicit report-only SHA summaries for push/merge contexts.

## Implemented Paths

- Helper: `scripts/ci/qsl_bounded_check_poll.py`
- Fixtures: `inputs/local_ops/qsl_bounded_check_poll_fixtures/*.json`
- Evidence: this file.
- Testplan: `tests/NA-0380_qsl_local_ops_bounded_ci_polling_helper_testplan.md`
- Governance: `DECISIONS.md`, `TRACEABILITY.md`, and
  `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Fixture Schema and Cases

Fixture schema:

- `target`: mode, repo, PR, SHA, branch.
- `required_contexts`: required check names for the fixture.
- `policies`: named policy maps for required/all/public-safety/report-only and
  explicit docs-only or CodeQL acceptance.
- `markers`: expected proof markers printed by fixture mode.
- `iterations`: ordered snapshots containing `check_runs` and optional
  `api_errors`.

Fixture cases:

- all required green;
- public-safety in progress then success;
- red required check;
- red public-safety;
- pending timeout;
- docs-only skips accepted under explicit docs-only policy;
- skipped check rejected without policy;
- CodeQL neutral accepted under explicit policy;
- CodeQL red rejected;
- missing public-safety rejected;
- push SHA report-only summary with PR-only context absence tolerated;
- stale failed rerun with latest success and stale failure reported;
- transient API 404 then success;
- persistent API 404 failure;
- malformed JSON failure.

## Fixture Test Results and Markers

Fixture proof log:

`/srv/qbuild/tmp/NA0380_bounded_check_poll_20260528T225422-0500/fixture_matrix.log`

The fixture matrix passed and printed:

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
- `NA0380_METADATA_RUNTIME_BOUNDED_CI_POLLING_HELPER_OK`
- `NA0380_NO_METADATA_FREE_CLAIM_OK`
- `NA0380_NO_ANONYMITY_CLAIM_OK`
- `NA0380_NO_UNTRACEABLE_CLAIM_OK`
- `NA0380_NO_PRODUCTION_READY_CLAIM_OK`
- `NA0380_NO_PUBLIC_INTERNET_READY_CLAIM_OK`

## Live Read-Only Smoke Results

Read-only smoke checks were run with no mutation and no waiting:

- `public-safety` on `origin/main` SHA `3aa72a5eb69d`: success.
- `sha-summary --report-only` on `origin/main` SHA `3aa72a5eb69d`: summary
  produced; report-only skipped/non-required contexts did not gate.
- `sha-summary --report-only` on PR #1022 head `4df792b20e4`: summary
  produced; push/PR context distinctions remained report-only.

## No-Watch / No-Mutation Proof

The helper does not call:

- `gh pr checks --watch`;
- `gh pr merge`;
- `gh run rerun`;
- `git push`;
- branch deletion, force-push, amend, squash, rebase, admin-bypass, workflow, or
  branch-protection mutation commands.

The helper imports only Python standard library modules.

## Fail-Closed Red / Timeout / API Behavior

- Red required checks fail nonzero immediately.
- Red public-safety fails nonzero immediately.
- Missing public-safety fails nonzero at the bounded timeout.
- Pending required checks fail nonzero at the bounded timeout.
- Malformed fixture JSON fails nonzero.
- API errors are reported with endpoint/status/message; persistent API failure
  fails nonzero after the bounded window.

## PR-vs-Push Context Handling

`pr` mode gates PR-head required or all observed checks. `sha-summary` mode is
explicitly report-only and does not treat missing PR-only contexts on push or
merge SHAs as merge-gate failures.

## Docs-Only and CodeQL Policy Handling

Docs-only skipped full-suite checks are accepted only when policy explicitly
sets docs-only skip acceptance. Skipped checks are rejected without that policy.

CodeQL neutral is accepted only under explicit CodeQL neutral policy. CodeQL red
is rejected even under that policy.

## Backup-Plan Impact

No backup-plan update is required for NA-0380 implementation because durable
artifacts are tracked in qsl-protocol and proof logs remain temporary under
`/srv/qbuild/tmp`. Local continuity remains same-host continuity only and is not
complete disaster recovery.

## Runtime / Service / Dependency / Workflow Boundary

NA-0380 changes no runtime, protocol, crypto, qsc, qsp, qsl, qshield runtime,
qsc-desktop, qsl-server, qsl-attachments, workflow, dependency, Cargo,
branch-protection, public-safety configuration, backup, restore, deploy,
rollback, service, website, README, START_HERE, or public docs path.

## qsl-server / qsl-attachments Boundary

qsl-server PR #56 remains read-only merged bounded harness evidence at
`d40e6003fdf0`. qsl-attachments PR #37 remains read-only merged service-local
prerequisite evidence at `96b9352bd63e`. Neither repository is mutated.

## Public-Claim Boundary

This helper is local-ops CI evidence tooling only. It does not prove production
readiness, public-internet readiness, external review completion,
metadata-free behavior, anonymity, untraceable behavior, hidden size, hidden
timing, hidden traffic shape, off-host backup completion, complete disaster
recovery, real restore completion, target configuration, host identity
verification, key custody, or key recovery.

## Selected Successor

Selected successor:

`NA-0381 -- QSL Local Ops Directive Manifest and Allow-File Implementation Authorization Plan`

Rationale:

- NA-0380 closes the bounded polling-helper implementation gap.
- The next workflow-support risk is directive scope/allow-file precision before
  broader response writer or validation-profile automation.
- NA-0381 remains authorization/planning only and must not be implemented by
  NA-0380 closeout.

## Rejected Alternatives

- Modify `public_safety_gate.py`: rejected because standalone helper meets the
  bounded polling need without public-safety policy drift.
- Modify `qsl_evidence_helper.py`: rejected because standalone helper avoids
  coupling to the broad governance helper.
- Add workflow integration now: rejected because NA-0380 helper proof does not
  require workflow changes.
- Add dependencies: rejected because standard library is sufficient.
- Implement response writer or validation profiles now: rejected as successor
  work outside NA-0380.

## Next Recommendation

After NA-0380 merge and public-safety green, close out NA-0380 and restore
NA-0381 as the sole READY successor. Do not implement NA-0381 during closeout.
