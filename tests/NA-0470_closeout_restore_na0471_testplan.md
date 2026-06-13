Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-13

# NA-0470 Closeout Restore NA-0471 Testplan

## Objective

Close out NA-0470 after evidence PR #1210 merged and post-merge
public-safety completed success, then restore NA-0471 as the sole READY
successor without implementing NA-0471.

## Protected Invariants

- NA-0470 is DONE only after PR #1210 merged at `95feccd041a5` and
  post-merge public-safety completed success.
- NA-0471 is READY and governance-only transactionality design authorization.
- Exactly one READY item remains.
- Closeout changes only `NEXT_ACTIONS.md`, `DECISIONS.md`, `TRACEABILITY.md`,
  `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`, and this testplan.
- No implementation mutation occurs.
- No runtime mutation occurs.
- No crypto mutation occurs.
- No dependency mutation occurs.
- No Cargo manifest mutation occurs.
- No lockfile mutation occurs.
- No workflow mutation occurs.
- No qsc source mutation occurs.
- No executable test mutation occurs.
- No fuzz target mutation occurs.
- No vector mutation occurs.
- No formal model mutation occurs.
- No refimpl mutation occurs.
- No qsl-server mutation occurs.
- No qsl-attachments mutation occurs.
- No qshield runtime mutation occurs.
- No qshield-cli mutation occurs.
- No website mutation occurs.
- No public docs mutation occurs.
- No README mutation occurs.
- No START_HERE mutation occurs.
- No qwork/qstart/qresume/qshell mutation occurs.
- No backup is run.
- No restore is run.
- No qsl-backup, backup status, backup plan, rollback subtree, or backup tree
  path is mutated.
- No public-readiness claim is introduced.
- No production-readiness claim is introduced.
- No public-internet-readiness claim is introduced.
- No external-review-complete claim is introduced.
- No crypto-complete claim is introduced.
- No KEM-complete claim is introduced.
- No signature-complete claim is introduced.
- No identity-complete claim is introduced.
- No RNG-failure-complete claim is introduced.
- No provider-RNG-complete claim is introduced.
- No secret-material-complete claim is introduced.
- No side-channel-free claim is introduced.
- No vulnerability-free claim is introduced.
- No bug-free claim is introduced.
- No perfect-crypto claim is introduced.

## NA-0471 Restored Scope

Future NA-0471 may mutate only:

- `docs/governance/evidence/NA-0471_qsl_qsc_tui_account_bootstrap_transactionality_design_authorization_plan.md`
- `tests/NA-0471_qsl_qsc_tui_account_bootstrap_transactionality_design_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Future NA-0471 may inspect read-only:

- `qsl/qsl-client/qsc/src/tui/controller/commands/locked.rs`
- `qsl/qsl-client/qsc/src/identity/mod.rs`
- `qsl/qsl-client/qsc/src/vault/mod.rs`
- `qsl/qsl-client/qsc/tests/`
- `docs/governance/evidence/NA-0470_qsl_qsc_tui_account_bootstrap_identity_provider_rng_failure_scope_authorization_plan.md`

## Validation Commands

Run from the qsl-protocol repo root:

```bash
git diff --check
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
python3 scripts/ci/qsl_evidence_helper.py classifier --changed-files <changed-files>
cargo audit --deny warnings
cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock
```

Also run the directive closeout scope guard, link-check, leak-scan,
overclaim scan, PR body preflight, and goal-lint.

## Expected Results

- READY_COUNT is 1.
- The sole READY item is NA-0471.
- NA-0470 is DONE.
- D-0929 exists exactly once.
- D-0930 is absent.
- Duplicate decision count is zero.
- Changed paths are limited to the five closeout paths.
- Root cargo audit is green.
- Nested qsc fuzz lock audit is green.
- Public-safety is green before merge and after merge.
- qsl-backup source hash and source-list proof remain read-only boundary
  evidence only.
