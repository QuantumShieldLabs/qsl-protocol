Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-02

# NA-0403 Closeout / Restore NA-0404 Testplan

Goals: G1, G2, G3, G4, G5

## Objective

Verify that NA-0403 is closed after PR #1069 post-merge public-safety
completes successfully and that exactly one successor is restored:

`NA-0404 -- QSL Director State Index Durable Storage / Backup Impact Authorization Plan`

This closeout must not implement NA-0404, mutate the Director State Index helper
or fixtures, or authorize durable Director State Index output.

## Protected Invariants

- READY_COUNT is exactly 1.
- READY item is NA-0404 after closeout.
- NA-0403 is DONE.
- D-0788 exists once.
- D-0789 exists once.
- D-0790 is absent.
- public-safety remains required and green.
- Live repo/GitHub/CI remains authoritative over any index output.
- No branch-protection, workflow, runtime, protocol, crypto, dependency, Cargo,
  qshield runtime, qsl-server, qsl-attachments, qsc-desktop, website, README,
  START_HERE, docs/public, backup script/timer/fstab/source-list, response
  archive, local history, durable Director State Index, helper, fixture, local
  qstart/qresume, or secret-bearing path is changed.

## Allowed Scope

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0403_closeout_restore_na0404_testplan.md`

## Forbidden Scope

- Runtime, protocol, crypto, qsc/qsp/qsl, qshield runtime, service, workflow,
  dependency, Cargo, qsl-server, qsl-attachments, qsc-desktop, website,
  README, START_HERE, docs/public, public security policy, security.txt,
  SECURITY.md, issue template, public paper, backup script/timer/fstab,
  backup source-list, off-host target, restore target, key, credential,
  passphrase, response archive, request archive, local history, qstart/qresume,
  Director State Index helper, Director State Index fixtures, durable Director
  State Index output, and branch-protection mutation.

## PR #1069 Post-Merge Public-Safety Prerequisite

- qsl-protocol PR #1069 is merged.
- PR #1069 head is `9a7aa5229be870f5d474b725d0a60ff13e3e41be`.
- PR #1069 merge commit is `901f877367a8ff4292a2dd45d15713afa383866e`.
- post-merge public-safety on the merge commit is completed success.
- Required branch-protection contexts have no failed required check.
- `qsc-linux-full-suite` and `macos-qsc-full-serial` are completed success or
  accepted under current policy while public-safety is success.

## Queue / Decision Requirements

- Queue before closeout has READY_COUNT 1 and READY NA-0403.
- D-0788 exists once before closeout.
- D-0789 is absent before closeout.
- No duplicate decisions are present.
- Mark NA-0403 DONE.
- Restore NA-0404 with exact title:
  `QSL Director State Index Durable Storage / Backup Impact Authorization Plan`.
- Keep READY_COUNT 1.

## D-0789 Requirements

D-0789 must state:

- NA-0403 is complete after PR #1069 post-merge public-safety completed
  successfully.
- NA-0404 is restored as the next READY successor for durable Director State
  Index storage / backup-impact authorization.
- No NA-0404 implementation is authorized by closeout.
- No durable Director State Index output is authorized by closeout.
- Live repo/GitHub/CI remains authoritative over index output.
- Runtime, security, public-claim, backup, local-history, response-archive,
  helper, fixture, and secret-handling boundaries remain protected.

## NA-0404 Successor Requirements

- NA-0404 objective is authorization planning only.
- NA-0404 must preserve live repo/GitHub/CI authority.
- NA-0404 must preserve stale-state rejection and no public claims.
- NA-0404 must require exact future scope plus backup review before any durable
  local index mutation.

## No NA-0404 Implementation Requirements

- Closeout must not implement durable storage.
- Closeout must not create or mutate backup scripts, timers, fstab entries,
  source lists, off-host targets, restore targets, or key-custody mechanisms.
- Closeout must not change runtime, workflow, dependency, service, protocol,
  crypto, website, public docs, README, START_HERE, or docs/public paths.

## No Durable Director State Index Output Requirements

- Closeout must not create durable Director State Index output.
- Existing temporary PR #1069 proof output remains temporary evidence only.
- Any future durable local index storage requires explicit NA-0404 scope and
  backup-impact authorization.

## No Helper Mutation Requirements

- `scripts/ci/qsl_director_state_index.py` must not change.
- `inputs/local_ops/director_state_index_fixtures/**` must not change.
- The closeout must not modify any other helper script.

## No Response Archive / Local History Mutation Requirements

- Closeout must not mutate response archives, request archives, local history,
  local ops indexes, or durable Codex history roots.
- The final D224 response file is the only response archive write authorized by
  the directive and is outside this PR patch.

## Public Claim Boundary

- Closeout must not claim public readiness.
- Closeout must not claim production readiness.
- Closeout must not claim public internet readiness.
- Closeout must not claim external-review completion.
- Closeout must not claim privacy completion.
- Closeout must not claim metadata-free behavior.
- Closeout must not claim anonymity.
- Closeout must not claim untraceability.
- Closeout must not claim complete disaster recovery.
- Closeout must not claim off-host backup completion.
- Closeout must not claim restore completion.
- Closeout must not claim key custody.
- Closeout must not claim key recovery.
- Closeout must not claim vulnerability-free status.
- Closeout must not claim bug-free status.
- Closeout must not claim perfect crypto.

## Validation Commands

Run or record:

- `git status --porcelain=v1 --branch`
- `git diff --name-only origin/main...HEAD`
- `git diff --stat origin/main...HEAD`
- `git diff --check`
- `python3 scripts/ci/qsl_evidence_helper.py queue`
- `python3 scripts/ci/qsl_evidence_helper.py decisions`
- D-0789 count is one.
- D-0790 count is zero.
- Scope guard for the exact allowed path set.
- Link-check, leak-scan, classifier, overclaim scan, and PR body preflight /
  goal-lint.
- `cargo audit --deny warnings`
- `cargo tree -i rustls-webpki --locked`
- `cargo fmt --check`
- qsc send_commit.
- formal model checks.

## CI Expectations

- Branch-protection required contexts must complete normally.
- public-safety must remain required and complete success.
- No admin bypass, direct push, squash, rebase, force-push, branch deletion, or
  history rewrite is allowed.
- If public-safety fails or remains pending after bounded polling, closeout must
  stop for user action.
