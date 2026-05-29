Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-28

# NA-0379 Closeout Restore NA-0380 Testplan

Goals: G1, G2, G3, G4, G5

## Objective

Close NA-0379 after the bounded CI polling helper implementation authorization
PR merges, and restore the exact successor:
`NA-0380 -- QSL Local Ops Bounded CI Polling Helper Implementation Harness`.

## Protected Invariants

- READY_COUNT remains exactly `1`.
- NA-0379 is `DONE`.
- NA-0380 is the sole `READY` item.
- D-0740 exists once.
- D-0741 exists once.
- D-0742 is absent.
- public-safety remains required and green.
- The closeout does not implement NA-0380.

## Allowed Scope

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0379_closeout_restore_na0380_testplan.md`

## Forbidden Scope

- `.github/**`
- `scripts/**`
- `Cargo.toml`
- `Cargo.lock`
- `qsp/**`
- `qsc/**`
- `qsl/**`
- `qsl-client/**`
- `apps/**`
- `tools/**`
- `inputs/**`
- `formal/**`
- `qsc-desktop/**`
- `qsl-server/**`
- `qsl-attachments/**`
- `website/**`
- `README.md`
- `START_HERE.md`
- `docs/public/**`
- runtime/service/protocol/crypto/demo/service implementation paths
- branch-protection or public-safety configuration
- backup scripts, timers, fstab, local system paths, secrets, remote/off-host
  targets, restore paths, deploy paths, rollback paths, and branch deletion

## NA-0379 Inheritance Requirements

- PR #1021 merged the NA-0379 authorization evidence.
- PR #1021 head is `52fcdab16132`.
- PR #1021 merge is `9d73b62f7d62`.
- D-0740 records the authorization decision.
- The NA-0379 evidence selects NA-0380 as the exact successor.
- Post-merge public-safety on `9d73b62f7d62` completed success.

## Queue Requirements

Run:

```bash
python3 scripts/ci/qsl_evidence_helper.py queue --select NA-0379 --select NA-0380
python3 scripts/ci/qsl_evidence_helper.py decisions --select D-0740 --select D-0741 --select D-0742
```

Expected:

- READY_COUNT `1`.
- READY `NA-0380`.
- NA-0379 `DONE`.
- D-0740 once.
- D-0741 once.
- D-0742 absent.
- duplicate decision count zero.

## Scope Guard

Run scope guard from `origin/main` to `HEAD` with only the allowed paths above.
Expected changed paths are limited to closeout governance and this testplan.

## Link And Leak Checks

Run:

```bash
python3 scripts/ci/qsl_evidence_helper.py link-check --root .
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
```

Expected:

- missing-link count zero.
- secret finding count zero.

## Dependency And Main Health

Run:

```bash
cargo audit --deny warnings
cargo tree -i rustls-webpki --locked
cargo fmt --check
```

Expected:

- audit passes.
- `rustls-webpki` is v0.103.13 or newer safe version.
- formatting passes.

## Runtime Boundary Checks

Run the established qsl-protocol health checks required by the directive,
including:

```bash
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
python3 formal/model_qsc_handshake_suite_id_bounded.py
python3 formal/run_model_checks.py
```

Expected:

- all commands pass.
- no runtime, protocol, crypto, dependency, workflow, or helper implementation
  file is changed by closeout.

## PR Metadata

The closeout PR body must include:

- `Goals: G1, G2, G3, G4, G5`
- Impact
- No-regression
- Tests/Vectors

Run:

```bash
python3 scripts/ci/qsl_evidence_helper.py pr-body-preflight --file <body-file>
scripts/audit/run_goal_lint_pr.sh <PR_NUMBER>
```

Expected:

- required PR body fields are present.
- goal-lint passes.

## CI Expectations

- Required PR checks pass normally.
- public-safety completes success on the PR head.
- The PR is merged with a normal merge commit and `--match-head-commit`.
- No admin bypass, squash, rebase, force-push, amend, direct push, or branch
  deletion command is used.
- Post-merge public-safety completes success on the merge SHA.

## Successor Handoff

NA-0380 is implementation scope only for a future exact directive. This closeout
does not authorize broader workflow, runtime, dependency, public-safety gate,
qsl_evidence_helper, backup, secret, qstart/qresume, website, public-claim, or
sibling-repo mutation.
