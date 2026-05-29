Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-05-28

# NA-0379 QSL Local Ops Bounded CI Polling Helper Implementation Authorization Test Plan

Goals: G1, G2, G3, G4, G5

## Objective

Validate that NA-0379 authorizes the exact next bounded CI/public-safety polling
helper implementation lane without implementing the helper or changing runtime,
workflow, dependency, public-safety, qbuild tool, backup, secret, target, or
public-claim surfaces.

## Protected Invariants

- READY_COUNT remains 1.
- READY remains NA-0379 during the authorization PR.
- NA-0378 is DONE.
- D-0738 exists once.
- D-0739 exists once.
- D-0740 exists once after authorization.
- D-0741 remains absent until optional closeout.
- No polling helper implementation is added in NA-0379.
- No workflow, dependency, runtime, qsl-server, qsl-attachments, qshell,
  public-safety gate, or backup configuration mutation occurs.
- public-safety remains required and green before merge.

## Allowed Scope

- `docs/governance/evidence/NA-0379_qsl_local_ops_bounded_ci_polling_helper_implementation_authorization.md`
- `tests/NA-0379_qsl_local_ops_bounded_ci_polling_helper_implementation_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Forbidden Scope

No changes are allowed in:

- `.github/**`
- `scripts/**`
- `scripts/ci/public_safety_gate.py`
- `scripts/ci/qsl_evidence_helper.py`
- `Cargo.toml`
- `Cargo.lock`
- runtime, service, protocol, crypto, auth, state-machine, key schedule, qsc,
  qsp, qsl, qshield runtime, qsc-desktop, apps runtime, tools/refimpl paths
- qsl-server
- qsl-attachments
- website, external website, README, START_HERE, docs/public
- `/srv/qbuild/tools/**`
- `/usr/local/sbin/qsl-backup`
- backup scripts, timers, fstab, system services, source lists, keys,
  credentials, restore, deploy, rollback, or off-host target paths
- `/home/victor/work/qsl/codex/**` except the required D198 response file

## NA-0378 Inheritance Requirements

Evidence must record:

- PR #1019 and PR #1020 merge SHAs;
- qshell checksum, rollback, patch, and harness-log references;
- that NA-0378 selected NA-0379;
- that NA-0379 does not mutate qshell again.

## Polling-Friction Inventory Requirements

Evidence must inventory:

- custom polling Python heredoc/here-string mistakes;
- `gh pr checks` nonzero while checks are pending;
- late public-safety attach;
- PR contexts vs push contexts;
- missing PR-only contexts on merge commits;
- CodeQL neutral/skipped behavior;
- docs-only skip acceptance;
- public-safety PR-files API 404 recovery;
- long qsc-linux/macos full-suite checks;
- stale failed check-runs after rerun;
- job-level vs workflow-level rerun ambiguity;
- shell JSON handoff and argument-size issues;
- mergeability timing;
- public-safety SHA/ref command-shape issues;
- goal-lint synthetic-event usage.

## Existing Helper Discovery Requirements

Evidence must inspect read-only:

- `scripts/ci/qsl_evidence_helper.py`;
- `scripts/ci/public_safety_gate.py`;
- `scripts/ci/classify_ci_scope.sh`;
- `scripts/audit/run_goal_lint_pr.sh`;
- `tools/goal_lint.py`;
- relevant workflow context behavior.

Required classifications:

- `POLLING_HELPER_QSL_PROTOCOL_READY`
- `POLLING_HELPER_AUTHORITY_CLEAR`
- `POLLING_HELPER_BACKUP_IMPACT_LOW`
- `POLLING_HELPER_IMPLEMENTATION_READY` for future exact scope only
- `POLLING_HELPER_WORKFLOW_CHANGE_NOT_REQUIRED`
- `POLLING_HELPER_SHOULD_NOT_MUTATE_PUBLIC_SAFETY_GATE`

## Context-Model Requirements

Evidence must define accepted, retryable, rejected, and stop states for:

- PR head required checks;
- PR head all-check summaries;
- public-safety by SHA;
- post-merge main/push checks;
- CodeQL;
- docs-only skipped full suites;
- qsc-linux full suite;
- macOS qsc full serial;
- qsc adversarial smoke;
- PR-files API failures;
- branch protection required contexts;
- mergeability status.

## Helper Semantic Requirements

Future helper semantics must include:

- bounded REST polling;
- no watch mode;
- no infinite wait;
- deterministic exit codes;
- explicit PR-vs-push target mode;
- latest-run selection;
- JSON parsing through structured subprocess/file/stdin handling;
- failure summaries with URLs;
- pending summaries at timeout;
- report-only mode where appropriate;
- no automatic unlimited reruns;
- no mutation.

## Red-Check / Failure-Policy Requirements

Evidence must state:

- red required checks fail closed;
- timeouts fail closed;
- public-safety red fails closed;
- docs-only skip accepted only with classifier proof;
- CodeQL neutral accepted only by explicit policy;
- API 404 retryable only within bounded policy and with independent proof;
- stale failures remain visible even when latest run passes.

## Path / Test Strategy Requirements

Evidence must choose a future path and test strategy.

Expected future first path:

- `scripts/ci/qsl_bounded_check_poll.py`

Expected tests:

- fixture-based;
- no GitHub network for unit tests;
- all-green, pending, timeout, red, missing, skipped/neutral, docs-only,
  CodeQL neutral, API 404 metadata, stale failed rerun, PR-vs-push, and
  malformed JSON cases.

## Risk Matrix Requirements

Evidence must compare:

- standalone Python helper;
- `qsl_evidence_helper.py` subcommands;
- shell helper;
- local `/srv/qbuild/tools` helper;
- no helper/manual polling.

## Implementation Authorization Requirements

Authorization is valid only if the evidence selects a future lane and states
that NA-0379 implements no tooling.

Expected authorization:

`POLLING_HELPER_IMPLEMENTATION_AUTHORIZATION_READY`

## Path Bundle Requirements

Evidence must include exact future allowed and forbidden paths, including a
ban on `.github/**`, `public_safety_gate.py`, dependencies, runtime paths,
sibling repositories, backup/system paths, website/public docs, qbuild tools,
and Codex history roots unless separately authorized.

## Backup Impact Requirements

Evidence must decide:

- NA-0379 backup-plan update required: expected no.
- Future NA-0380 backup-plan update required: no if repo-local and temporary
  logs stay temporary; yes if durable outputs move outside authorized repo paths
  or `/srv/qbuild/tmp`.

## Fail-Closed Requirements

Evidence must require:

- no red-check suppression;
- no branch-protection bypass;
- no admin bypass;
- no merge if required checks are red/missing after timeout;
- no hidden mutation;
- no secrets in logs;
- no workflow or dependency changes.

## Public-Claim Boundary Requirements

Evidence must not claim:

- must not claim production readiness;
- must not claim public-internet readiness;
- must not claim external-review completion;
- must not claim metadata-free behavior;
- must not claim anonymity;
- must not claim untraceable behavior;
- must not claim hidden size;
- must not claim hidden timing;
- must not claim hidden traffic shape;
- must not claim off-host backup completion;
- must not claim disaster recovery completion;
- must not claim operator response availability.

## Successor Selection Requirements

Expected successor:

`NA-0380 -- QSL Local Ops Bounded CI Polling Helper Implementation Harness`

NA-0379 must not deliver NA-0380 tooling.

## Required Local Checks

Run and record:

- queue and decisions helpers;
- scope guard;
- link-check;
- leak-scan;
- overclaim scan;
- classifier proof;
- PR body preflight and goal-lint;
- `cargo audit --deny warnings`;
- `cargo tree -i rustls-webpki --locked`;
- `cargo fmt --check`;
- qsc send_commit;
- formal/model checks;
- metadata JSON/no-secret harnesses when directly runnable;
- qshield-cli test/build when feasible.

## CI Expectations

The PR body must include Goals, Impact, No-regression, and Tests/Vectors.
Required checks, including `public-safety`, must pass before merge. Post-merge
public-safety must be green before optional closeout.

## Successor Handoff

After the authorization PR merges and post-merge public-safety is green, a
separate closeout PR may mark NA-0379 DONE and restore NA-0380 as READY. That
closeout must not implement NA-0380.
