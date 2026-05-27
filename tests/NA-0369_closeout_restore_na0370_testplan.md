Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-05-27

# NA-0369 Closeout and NA-0370 Restoration Testplan

## Objective

Validate the governance-only closeout for NA-0369 after PR #1000 merged the
no-secret operator action packet and restore exactly one READY successor:

`NA-0370 -- Metadata Runtime Off-Host Backup Target Candidate / Host Identity Operator Response Intake`

This closeout must not implement NA-0370.

## Protected Invariants

- Exactly one READY item exists after closeout: NA-0370.
- NA-0369 is DONE.
- D-0720 and D-0721 each exist once.
- D-0722 is absent.
- qsl-protocol is the only mutable repository.
- qsl-server and qsl-attachments are not mutated.
- qshield runtime is not mutated.
- qsc/qsp/protocol/crypto/key-schedule implementation is not mutated.
- Cargo dependencies and workflows are unchanged.
- README, START_HERE, website, and docs/public are unchanged.
- Backup scripts, timers, fstab, service units, source lists, restore paths,
  rollback paths, remote destinations, key material, passphrase material,
  credential material, recovery-envelope content, and local backup
  configuration are unchanged.
- No remote/off-host connection occurs.
- No host-key scan occurs.
- No `known_hosts` mutation occurs.
- No repository init occurs.
- No tool installation occurs.
- No backup, restore, deploy, or rollback occurs.
- No real restore target creation/mount/copy occurs.
- No real key generation or key upload occurs.
- No passphrase collection occurs.
- No credential handling occurs.
- No private-key inspection occurs.
- No recovery-envelope content creation occurs.
- No secret handling occurs.
- No claim states or implies production readiness.
- No claim states or implies public-internet readiness.
- No claim states or implies external-review completion.
- No claim states or implies anonymity.
- No claim states or implies metadata-free behavior.
- No claim states or implies untraceability.
- No claim states or implies hidden attachment size.
- No claim states or implies hidden timing metadata.
- No claim states or implies hidden traffic shape.
- No claim states or implies hidden all metadata.
- No claim states or implies configured target.
- No claim states or implies verified host identity.
- No claim states or implies complete off-host backup.
- No claim states or implies complete disaster recovery.
- No claim states or implies real restore completion.
- No claim states or implies real key custody implementation.
- No claim states or implies real key recovery implementation.

## Allowed Closeout Paths

Allowed files:

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0369_closeout_restore_na0370_testplan.md`

## Forbidden Paths

Forbidden changes include README, START_HERE, docs/public, `.github`, Cargo
manifests or lockfiles, qsp, qsc, qsl, qsl-client, apps, tools, inputs, formal,
scripts, qsc-desktop, qsl-server, qsl-attachments, website, runtime/protocol
implementation paths, branch-protection/public-safety configuration, backup
scripts/timers/fstab/local system paths, and branch deletion.

## Queue and Decision Checks

Validation must confirm:

- READY_COUNT is 1.
- READY item is NA-0370.
- NA-0369 is DONE.
- D-0720 exists once.
- D-0721 exists once.
- D-0722 is absent.
- Duplicate decision count is zero.

## PR #1000 Post-Merge Public-Safety Proof

Validation must confirm:

- qsl-protocol PR #1000 merged from validated head `cca65ba7ed6`.
- qsl-protocol PR #1000 merge commit `b9a57357e07d` exists on `origin/main`.
- post-merge qsl-protocol `public-safety` completed success on
  `b9a57357e07d`.
- direct check-runs for `b9a57357e07d` show no failures and no in-progress
  jobs.
- `qsc-linux-full-suite`, `macos-qsc-full-serial`, and
  `qsc-adversarial-smoke` completed success or are in an accepted configured
  state.

## No NA-0370 Implementation Proof

Validation must confirm:

- changed paths are exactly the allowed closeout paths.
- no `inputs/metadata_runtime/**` response file is added or edited.
- no target candidate value, host identity value, credential, or secret is
  introduced.
- no remote target setup, host-key scan, `known_hosts` mutation, repository
  init, tool installation, backup, restore, deploy, rollback, or
  runtime/service change is introduced.
- the operator packet remains no-secret template evidence until a future
  response is provided and validated.

## Scope Guard

Run:

- `git diff --name-only origin/main...HEAD`
- `git diff --stat origin/main...HEAD`
- `git diff --check`
- `python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main` with exact allowed paths
- `bash scripts/ci/classify_ci_scope.sh NEXT_ACTIONS.md DECISIONS.md TRACEABILITY.md docs/ops/ROLLING_OPERATIONS_JOURNAL.md tests/NA-0369_closeout_restore_na0370_testplan.md`

Expected:

- changed paths exactly match the allowed closeout paths.
- forbidden count is zero.
- classifier result is docs-only.

## Link, Leak, and Overclaim Checks

Run:

- `python3 scripts/ci/qsl_evidence_helper.py link-check`
- `python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main`
- changed-line overclaim scan over added governance/testplan text.

Expected:

- markdown link check passes.
- secret finding count is zero.
- no affirmative overclaim is introduced.

## Dependency and Advisory Checks

Run:

- `cargo audit --deny warnings`
- `cargo tree -i rustls-webpki --locked`

Expected:

- cargo audit passes.
- `rustls-webpki` is version `0.103.13` or newer safe version.

## Required Local Checks

Run:

- `python3 scripts/ci/qsl_evidence_helper.py queue`
- `python3 scripts/ci/qsl_evidence_helper.py decisions`
- direct D-count checks for D-0720, D-0721, and D-0722
- `cargo fmt --check`
- `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1`
- `python3 formal/model_qsc_handshake_suite_id_bounded.py`
- `python3 formal/run_model_checks.py`
- local goal-lint using the closeout PR body
- classifier proof for the changed path set

## CI Expectations

The closeout PR must merge only after required qsl-protocol checks complete
normally and `public-safety` is green. No admin bypass, direct push, squash,
rebase, or delete-branch flag is allowed.

After merge, final main must preserve:

- READY NA-0370.
- NA-0369 DONE.
- D-0721 once.
- D-0722 absent.
- post-merge `public-safety` success.
- cargo audit green.
- `rustls-webpki v0.103.13` or newer safe version.
